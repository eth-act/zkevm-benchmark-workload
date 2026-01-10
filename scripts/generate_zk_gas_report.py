#!/usr/bin/env python3
"""
Generate ZK Gas Benchmark Report.

Analyzes benchmark results from execution mode (zk cycles) and/or proving mode (proving time),
performs regression analysis against gas usage, and generates a comprehensive markdown report.

Expected Directory Structure (NEW - 2026+):
  <base-dir>/
    execute/         # Execution results (run once, deterministic, no samples)
    prove/           # Proving results (sampled for statistics)
      sample-1/
      sample-2/
      sample-N/

Usage:
  python3 generate_zk_gas_report.py \
    --execution-input <base-dir>/execute \
    --proving-input <base-dir>/prove \
    --output reports/

Note: Also supports legacy structure with sample-*/execute/ and sample-*/prove/
"""

import argparse
import base64
import io
import json
import os
import re
import sys
from datetime import datetime
from pathlib import Path
from typing import Optional

import markdown
import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
from scipy import stats


# =============================================================================
# Multi-Sample and Outlier Detection
# =============================================================================

def detect_sample_dirs(input_dir: Path) -> list[Path]:
    """
    Detect sample-N subdirectories for multi-sample support.
    
    Returns list of sample directories if found, otherwise returns [input_dir]
    for backward compatibility with single-sample runs.
    """
    sample_dirs = sorted(input_dir.glob("sample-*/"))
    if sample_dirs:
        return sample_dirs
    # Backward compatible: single sample (no sample-N dirs)
    return [input_dir]


def detect_outliers_mad(values: np.ndarray, threshold: float = 3.5) -> np.ndarray:
    """
    Detect outliers using Modified Z-Score with MAD (Median Absolute Deviation).
    
    This method is more robust than standard Z-score or IQR when there are 
    multiple outliers (up to ~30% of data).
    
    Formula:
        MAD = median(|x_i - median(x)|)
        Modified_Z = 0.6745 * (x - median) / MAD
    
    Args:
        values: Array of values to check for outliers
        threshold: Modified Z-score threshold (default 3.5, standard for MAD)
    
    Returns:
        Boolean array where True = outlier
    
    Reference:
        Iglewicz, B. and Hoaglin, D.C. (1993). "How to Detect and Handle Outliers"
    """
    values = np.asarray(values, dtype=float)
    
    # Handle NaN values - they are not outliers, just missing
    nan_mask = np.isnan(values)
    if nan_mask.all():
        return np.zeros(len(values), dtype=bool)
    
    valid_values = values[~nan_mask]
    
    if len(valid_values) < 3:
        # Too few points to detect outliers
        return np.zeros(len(values), dtype=bool)
    
    median = np.median(valid_values)
    mad = np.median(np.abs(valid_values - median))
    
    if mad == 0:
        # All values identical or nearly so - no outliers
        return np.zeros(len(values), dtype=bool)
    
    # 0.6745 is the scaling factor to make MAD comparable to standard deviation
    # for normally distributed data
    modified_z = np.zeros(len(values))
    modified_z[~nan_mask] = 0.6745 * (valid_values - median) / mad
    
    return np.abs(modified_z) > threshold


def remove_outliers_per_opcode(
    df: pd.DataFrame, 
    y_col: str, 
    threshold: float = 3.5
) -> tuple[pd.DataFrame, pd.DataFrame, dict]:
    """
    Remove outliers from DataFrame, grouped by opcode.
    
    Args:
        df: DataFrame with opcode column and values
        y_col: Column name to check for outliers (e.g., 'proving_time_s')
        threshold: Modified Z-score threshold
    
    Returns:
        Tuple of (clean_df, outliers_df, stats_dict)
        - clean_df: DataFrame with outliers removed
        - outliers_df: DataFrame containing only the outliers
        - stats_dict: Dict mapping opcode -> number of outliers removed
    """
    clean_rows = []
    outlier_rows = []
    outlier_stats = {}
    
    for opcode, group in df.groupby("opcode"):
        if y_col not in group.columns or group[y_col].isna().all():
            clean_rows.append(group)
            outlier_stats[opcode] = 0
            continue
        
        values = group[y_col].values
        is_outlier = detect_outliers_mad(values, threshold)
        
        clean_rows.append(group[~is_outlier])
        outliers = group[is_outlier]
        if len(outliers) > 0:
            outlier_rows.append(outliers)
        
        outlier_stats[opcode] = int(is_outlier.sum())
    
    clean_df = pd.concat(clean_rows, ignore_index=True) if clean_rows else pd.DataFrame()
    outliers_df = pd.concat(outlier_rows, ignore_index=True) if outlier_rows else pd.DataFrame()
    
    return clean_df, outliers_df, outlier_stats


# =============================================================================
# Utility Functions
# =============================================================================

def format_number(n: float, unit: str = "") -> str:
    """Format large numbers with M/B suffixes."""
    if n is None or pd.isna(n):
        return "N/A"
    abs_n = abs(n)
    sign = "-" if n < 0 else ""
    if abs_n >= 1e9:
        return f"{sign}{abs_n/1e9:.2f}B{unit}"
    elif abs_n >= 1e6:
        return f"{sign}{abs_n/1e6:.2f}M{unit}"
    elif abs_n >= 1e3:
        return f"{sign}{abs_n/1e3:.2f}K{unit}"
    elif abs_n >= 1:
        return f"{sign}{abs_n:.2f}{unit}"
    else:
        return f"{sign}{abs_n:.4g}{unit}"


def format_r2(r2: float, high_threshold: float = 0.9, mid_threshold: float = 0.7) -> str:
    """Format R² value with color coding.
    
    - High (>= 0.9): Green
    - Mid (0.7 - 0.9): Yellow/Orange
    - Low (< 0.7): Red
    """
    if r2 is None or pd.isna(r2):
        return "N/A"
    
    r2_str = f"{r2:.4f}"
    
    if r2 >= high_threshold:
        return f'<span style="color: #28a745; font-weight: bold;">{r2_str}</span>'
    elif r2 >= mid_threshold:
        return f'<span style="color: #ffc107; font-weight: bold;">{r2_str}</span>'
    else:
        return f'<span style="color: #dc3545; font-weight: bold;">{r2_str}</span>'


def format_time(seconds: float) -> str:
    """Format time in appropriate units."""
    if seconds is None or pd.isna(seconds):
        return "N/A"
    if seconds >= 3600:
        return f"{seconds/3600:.2f}h"
    elif seconds >= 60:
        return f"{seconds/60:.2f}m"
    elif seconds >= 1:
        return f"{seconds:.2f}s"
    elif seconds >= 0.001:
        return f"{seconds*1000:.2f}ms"
    else:
        return f"{seconds*1e6:.2f}µs"


def format_time_hms(seconds: float) -> str:
    """Format time in h/m/s format for longer durations."""
    if seconds is None or pd.isna(seconds):
        return "N/A"
    if seconds >= 3600:
        h = int(seconds // 3600)
        m = int((seconds % 3600) // 60)
        s = int(seconds % 60)
        return f"{h}h {m}m {s}s"
    elif seconds >= 60:
        m = int(seconds // 60)
        s = int(seconds % 60)
        return f"{m}m {s}s"
    else:
        return f"{seconds:.1f}s"


def parse_duration(duration_obj: dict | None) -> float | None:
    """Convert duration object {secs, nanos} to seconds."""
    if duration_obj is None:
        return None
    secs = duration_obj.get("secs", 0)
    nanos = duration_obj.get("nanos", 0)
    return secs + nanos / 1e9


def extract_opcode_and_count(name: str) -> tuple[str, int]:
    """Extract opcode/precompile name and op_count from test name."""
    count_match = re.search(r"op_count_(\d+)", name)

    # Try marginal test format: test_marginal_OPCODE[...-op_count_N]
    opcode_match = re.search(r"test_marginal_(\w+)\[", name)
    if opcode_match and count_match:
        return opcode_match.group(1), int(count_match.group(1))

    # Try caller-contract test format: test_caller_OPCODE[...-op_count_N]
    # The fixture name already includes (num_calls * op_count) as the total ops
    # so we use the op_count directly without additional multiplication
    caller_match = re.search(r"test_caller_(\w+)\[", name)
    if caller_match and count_match:
        # op_count in fixture name is already the total (num_calls * per_call_ops)
        return caller_match.group(1).lower(), int(count_match.group(1))

    # Try fixed test format: test_fixed_OPCODE[...-op_count_N]
    # Used by test_marginal_fixed_precompiles.py
    fixed_match = re.search(r"test_fixed_(\w+)\[", name)
    if fixed_match and count_match:
        return "fixed_" + fixed_match.group(1).lower(), int(count_match.group(1))

    # Try direct opcode test format: test_OPCODE[...-op_count_N]
    # Used by simple tests like test_add, test_eq, etc.
    direct_match = re.search(r"test_(\w+)\[", name)
    if direct_match and count_match:
        opcode_name = direct_match.group(1).lower()
        # Skip known prefixes that are handled above
        # BUT: "caller" is also a valid opcode name (CALLER opcode) - only skip if
        # it matched as a prefix pattern (test_caller_OPCODE), not as test_caller[
        if opcode_name == "caller" and caller_match is None:
            # This is test_caller[...] for CALLER opcode, not test_caller_OPCODE[...]
            return opcode_name, int(count_match.group(1))
        if opcode_name not in ["marginal", "caller", "fixed"]:
            return opcode_name, int(count_match.group(1))

    opcode = opcode_match.group(1) if opcode_match else "unknown"
    op_count = int(count_match.group(1)) if count_match else 0

    return opcode, op_count


def detect_prover_from_path(input_dirs: list[Path]) -> str:
    """Detect prover name from input directory structure."""
    for input_dir in input_dirs:
        reth_dir = input_dir / "reth"
        if reth_dir.exists():
            for subdir in reth_dir.iterdir():
                if subdir.is_dir():
                    # Extract prover name (e.g., "risc0" from "risc0-v3.0.4")
                    name = subdir.name
                    if "-" in name:
                        return name.split("-")[0]
                    return name
    return "unknown"


def get_prover_version(input_dirs: list[Path]) -> str:
    """Get full prover version string from input directory structure."""
    for input_dir in input_dirs:
        reth_dir = input_dir / "reth"
        if reth_dir.exists():
            for subdir in reth_dir.iterdir():
                if subdir.is_dir():
                    return subdir.name
    return "unknown"


# =============================================================================
# Data Loading
# =============================================================================

def load_hardware_info(input_dirs: list[Path]) -> dict:
    """Load hardware info from hardware.json in input directories."""
    for input_dir in input_dirs:
        hw_file = input_dir / "hardware.json"
        if hw_file.exists():
            try:
                with open(hw_file) as f:
                    return json.load(f)
            except (json.JSONDecodeError, IOError):
                pass
    return {}


def load_json_results(input_dirs: list[Path], mode: str, sample_id: int = 1) -> pd.DataFrame:
    """
    Load JSON result files from input directories.
    
    Args:
        input_dirs: List of input directories
        mode: "execution" or "proving"
        sample_id: Sample number (for multi-sample tracking)
    
    Returns:
        DataFrame with columns: name, opcode, op_count, gas_used, zk_cycles, proving_time_s, sample
    """
    records = []
    
    for input_dir in input_dirs:
        reth_dir = input_dir / "reth"
        if not reth_dir.exists():
            print(f"Warning: No 'reth' directory in {input_dir}", file=sys.stderr)
            continue
            
        # Find the prover version directory
        for prover_dir in reth_dir.iterdir():
            if not prover_dir.is_dir():
                continue
                
            for json_file in prover_dir.glob("*.json"):
                try:
                    with open(json_file) as f:
                        data = json.load(f)
                    
                    name = data.get("name", json_file.stem)
                    opcode, op_count = extract_opcode_and_count(name)
                    
                    # Extract gas used
                    gas_used = None
                    if "metadata" in data:
                        gas_used = data["metadata"].get("block_used_gas")
                    
                    # Extract zk_cycles (execution mode)
                    zk_cycles = None
                    if "execution" in data and isinstance(data["execution"], dict):
                        exec_data = data["execution"]
                        # Handle nested success object
                        if "success" in exec_data and isinstance(exec_data["success"], dict):
                            zk_cycles = exec_data["success"].get("total_num_cycles")
                        else:
                            zk_cycles = exec_data.get("total_num_cycles")
                    
                    # Extract proving_time (proving mode)
                    proving_time_s = None
                    if "proving" in data and isinstance(data["proving"], dict):
                        prove_data = data["proving"]
                        # Skip crashed proofs
                        if "crashed" in prove_data:
                            continue
                        # Handle nested success object
                        if "success" in prove_data and isinstance(prove_data["success"], dict):
                            success_data = prove_data["success"]
                            # Try proving_time_ms first (milliseconds)
                            if "proving_time_ms" in success_data:
                                proving_time_s = success_data["proving_time_ms"] / 1000.0
                            # Try proving_duration (duration object)
                            elif "proving_duration" in success_data:
                                proving_time_s = parse_duration(success_data["proving_duration"])
                        else:
                            # Direct access (old format)
                            if "proving_time_ms" in prove_data:
                                proving_time_s = prove_data["proving_time_ms"] / 1000.0
                            elif "proving_duration" in prove_data:
                                proving_time_s = parse_duration(prove_data["proving_duration"])
                    
                    record = {
                        "name": name,
                        "opcode": opcode,
                        "op_count": op_count,
                        "gas_used": gas_used,
                        "zk_cycles": zk_cycles,
                        "proving_time_s": proving_time_s,
                        "sample": sample_id,
                    }
                    records.append(record)
                    
                except (json.JSONDecodeError, KeyError, IOError) as e:
                    print(f"Warning: Failed to parse {json_file}: {e}", file=sys.stderr)
    
    return pd.DataFrame(records)


def load_multi_sample_results(base_dir: Path, mode: str) -> tuple[pd.DataFrame, int]:
    """
    Load results from multiple sample directories.

    Supports two directory structures:
    1. NEW (2026+): base_dir/execute/ (no samples) and base_dir/prove/sample-*/
    2. OLD: base_dir/sample-*/execute/ and base_dir/sample-*/prove/

    Args:
        base_dir: Base directory that may contain sample-N subdirectories
        mode: "execution" or "proving"

    Returns:
        Tuple of (pooled DataFrame, number of samples)
    """
    all_dfs = []

    if mode == "execution":
        # NEW structure: execution runs once, no samples
        # Look directly in base_dir (which should be the execute/ folder)
        if base_dir.exists() and (base_dir / "reth").exists():
            df = load_json_results([base_dir], mode, sample_id=1)
            if not df.empty:
                all_dfs.append(df)
                return pd.concat(all_dfs, ignore_index=True), 1

        # OLD structure: sample-*/execute/
        sample_dirs = detect_sample_dirs(base_dir)
        for i, sample_dir in enumerate(sample_dirs, 1):
            possible_dirs = [sample_dir / "execute", sample_dir]
            for check_dir in possible_dirs:
                if check_dir.exists() and (check_dir / "reth").exists():
                    df = load_json_results([check_dir], mode, sample_id=i)
                    if not df.empty:
                        all_dfs.append(df)
                    break

    else:  # proving
        # NEW structure: prove/sample-*/ (samples inside prove dir)
        sample_dirs = sorted(base_dir.glob("sample-*/"))
        if sample_dirs:
            # NEW structure detected
            for i, sample_dir in enumerate(sample_dirs, 1):
                if sample_dir.exists() and (sample_dir / "reth").exists():
                    df = load_json_results([sample_dir], mode, sample_id=i)
                    if not df.empty:
                        all_dfs.append(df)
        else:
            # OLD structure: sample-*/prove/ or base_dir directly
            sample_dirs = detect_sample_dirs(base_dir)
            for i, sample_dir in enumerate(sample_dirs, 1):
                possible_dirs = [sample_dir / "prove", sample_dir]
                for check_dir in possible_dirs:
                    if check_dir.exists() and (check_dir / "reth").exists():
                        df = load_json_results([check_dir], mode, sample_id=i)
                        if not df.empty:
                            all_dfs.append(df)
                        break

    if not all_dfs:
        return pd.DataFrame(), 0

    pooled_df = pd.concat(all_dfs, ignore_index=True)
    num_samples = len(all_dfs)

    return pooled_df, num_samples


def filter_duplicates_prefer_fixed(df: pd.DataFrame) -> pd.DataFrame:
    """Filter out original opcodes when a fixed_ version exists.
    
    For example, if both 'balance' and 'fixed_balance' exist, only keep 'fixed_balance',
    then rename 'fixed_balance' to 'balance'.
    """
    # Filter out NaN opcodes first
    df = df[df["opcode"].notna()]
    opcodes = df["opcode"].unique()
    fixed_opcodes = {op for op in opcodes if isinstance(op, str) and op.startswith("fixed_")}
    
    # Find original opcodes that have a fixed version
    originals_with_fixed = set()
    for fixed_op in fixed_opcodes:
        # Extract the original name (remove 'fixed_' prefix)
        original_name = fixed_op[6:]  # len("fixed_") == 6
        if original_name in opcodes:
            originals_with_fixed.add(original_name)
    
    if originals_with_fixed:
        print(f"  Filtering out {len(originals_with_fixed)} original opcodes that have fixed versions", file=sys.stderr)
    
    # Filter out the originals that have fixed versions
    filtered_df = df[~df["opcode"].isin(originals_with_fixed)].copy()
    
    # Remove "fixed_" prefix from opcode names
    filtered_df["opcode"] = filtered_df["opcode"].apply(
        lambda x: x[6:] if x.startswith("fixed_") else x
    )
    
    return filtered_df


def merge_execution_and_proving(exec_df: pd.DataFrame, prove_df: pd.DataFrame) -> pd.DataFrame:
    """Merge execution and proving results by test name and sample."""
    if exec_df.empty:
        return prove_df
    if prove_df.empty:
        return exec_df
    
    # Determine columns to merge on
    exec_cols = ["name", "opcode", "op_count", "gas_used", "zk_cycles"]
    prove_cols = ["name", "proving_time_s"]
    
    # Include sample column if present
    if "sample" in exec_df.columns:
        exec_cols.append("sample")
    if "sample" in prove_df.columns and "sample" not in exec_cols:
        prove_cols.append("sample")
    
    # Merge on name (and sample if both have it)
    merge_on = ["name"]
    if "sample" in exec_df.columns and "sample" in prove_df.columns:
        merge_on.append("sample")
        prove_cols = ["name", "sample", "proving_time_s"]
    
    merged = pd.merge(
        exec_df[[c for c in exec_cols if c in exec_df.columns]],
        prove_df[[c for c in prove_cols if c in prove_df.columns]],
        on=merge_on,
        how="outer"
    )
    
    # Fill missing opcode/op_count from either side
    if "opcode" not in merged.columns or merged["opcode"].isna().all():
        for _, row in prove_df.iterrows():
            mask = merged["name"] == row["name"]
            if mask.any() and pd.isna(merged.loc[mask, "opcode"].iloc[0]):
                opcode, op_count = extract_opcode_and_count(row["name"])
                merged.loc[mask, "opcode"] = opcode
                merged.loc[mask, "op_count"] = op_count
    
    return merged


# =============================================================================
# Regression Analysis
# =============================================================================

def perform_regression(x: np.ndarray, y: np.ndarray) -> dict:
    """Perform linear regression and return results."""
    if len(x) < 2 or len(y) < 2:
        return None
    
    # Remove NaN values
    mask = ~(np.isnan(x) | np.isnan(y))
    x_clean = x[mask]
    y_clean = y[mask]
    
    if len(x_clean) < 2:
        return None
    
    try:
        slope, intercept, r_value, p_value, std_err = stats.linregress(x_clean, y_clean)
        return {
            "slope": slope,
            "intercept": intercept,
            "r_squared": r_value ** 2,
            "p_value": p_value,
            "std_err": std_err,
            "n_points": len(x_clean),
        }
    except Exception:
        return None


def compute_regressions(df: pd.DataFrame, remove_outliers: bool = True, outlier_threshold: float = 3.5) -> pd.DataFrame:
    """
    Compute all regression types for each opcode.
    
    Args:
        df: DataFrame with opcode, gas_used, zk_cycles, proving_time_s columns
        remove_outliers: If True, remove outliers before regression
        outlier_threshold: Modified Z-score threshold for outlier detection
    
    Returns:
        DataFrame with regression results per opcode
    """
    results = []
    
    for opcode in df["opcode"].unique():
        opcode_df = df[df["opcode"] == opcode].copy()
        
        gas = opcode_df["gas_used"].values.astype(float)
        zk_cycles = opcode_df["zk_cycles"].values.astype(float) if "zk_cycles" in opcode_df else None
        proving_time = opcode_df["proving_time_s"].values.astype(float) if "proving_time_s" in opcode_df else None
        op_count = opcode_df["op_count"].values.astype(float)
        
        # Count samples if available
        n_samples = opcode_df["sample"].nunique() if "sample" in opcode_df.columns else 1
        
        result = {
            "opcode": opcode,
            "n_points": len(opcode_df),
            "n_samples": n_samples,
            "min_op_count": int(opcode_df["op_count"].min()),
            "max_op_count": int(opcode_df["op_count"].max()),
            "min_gas": int(opcode_df["gas_used"].min()) if opcode_df["gas_used"].notna().any() else None,
            "max_gas": int(opcode_df["gas_used"].max()) if opcode_df["gas_used"].notna().any() else None,
            "max_zkcycles": int(opcode_df["zk_cycles"].max()) if "zk_cycles" in opcode_df and opcode_df["zk_cycles"].notna().any() else None,
        }
        
        # Calculate per-opcode gas (slope of gas vs op_count)
        gas_reg = perform_regression(op_count, gas)
        if gas_reg:
            result["gas_per_op"] = gas_reg["slope"]
        
        # Outlier detection for proving time
        n_outliers_proving = 0
        proving_time_clean = proving_time
        gas_clean_proving = gas
        if remove_outliers and proving_time is not None and not np.all(np.isnan(proving_time)):
            outlier_mask = detect_outliers_mad(proving_time, outlier_threshold)
            n_outliers_proving = int(outlier_mask.sum())
            if n_outliers_proving > 0:
                # Keep only non-outliers for regression
                proving_time_clean = proving_time[~outlier_mask]
                gas_clean_proving = gas[~outlier_mask]
        result["n_outliers_proving"] = n_outliers_proving
        
        # Outlier detection for zk_cycles
        n_outliers_cycles = 0
        zk_cycles_clean = zk_cycles
        gas_clean_cycles = gas
        if remove_outliers and zk_cycles is not None and not np.all(np.isnan(zk_cycles)):
            outlier_mask = detect_outliers_mad(zk_cycles, outlier_threshold)
            n_outliers_cycles = int(outlier_mask.sum())
            if n_outliers_cycles > 0:
                zk_cycles_clean = zk_cycles[~outlier_mask]
                gas_clean_cycles = gas[~outlier_mask]
        result["n_outliers_cycles"] = n_outliers_cycles
        
        # Gas <> ZK Cycles regression (with outliers removed)
        if zk_cycles_clean is not None and not np.all(np.isnan(zk_cycles_clean)):
            reg = perform_regression(gas_clean_cycles, zk_cycles_clean)
            if reg:
                result["gas_zkcycles_slope"] = reg["slope"]
                result["gas_zkcycles_r2"] = reg["r_squared"]
                result["gas_zkcycles_std_err"] = reg["std_err"]
        
        # Gas <> Proving Time regression (with outliers removed)
        if proving_time_clean is not None and not np.all(np.isnan(proving_time_clean)):
            reg = perform_regression(gas_clean_proving, proving_time_clean)
            if reg:
                result["gas_proving_slope"] = reg["slope"]
                result["gas_proving_r2"] = reg["r_squared"]
                result["gas_proving_std_err"] = reg["std_err"]
        
        # ZK Cycles <> Proving Time regression (with outliers removed from both)
        if (zk_cycles is not None and proving_time is not None and 
            not np.all(np.isnan(zk_cycles)) and not np.all(np.isnan(proving_time))):
            # For this regression, remove outliers in proving_time
            if remove_outliers:
                outlier_mask = detect_outliers_mad(proving_time, outlier_threshold)
                zk_clean = zk_cycles[~outlier_mask]
                prove_clean = proving_time[~outlier_mask]
            else:
                zk_clean = zk_cycles
                prove_clean = proving_time
            reg = perform_regression(zk_clean, prove_clean)
            if reg:
                result["zkcycles_proving_slope"] = reg["slope"]
                result["zkcycles_proving_r2"] = reg["r_squared"]
                result["zkcycles_proving_std_err"] = reg["std_err"]
        
        results.append(result)
    
    return pd.DataFrame(results)


# =============================================================================
# Chart Generation
# =============================================================================

def create_regression_plot(
    df: pd.DataFrame,
    opcode: str,
    x_col: str,
    y_col: str,
    x_label: str,
    y_label: str,
    title: str,
    remove_outliers: bool = True,
    outlier_threshold: float = 3.5,
) -> Optional[plt.Figure]:
    """
    Create a regression plot for a single opcode.
    
    If remove_outliers is True, outliers are detected and shown as red X markers
    but excluded from the regression line calculation.
    """
    opcode_df = df[df["opcode"] == opcode].copy()
    
    x = opcode_df[x_col].values.astype(float)
    y = opcode_df[y_col].values.astype(float)
    
    # Remove NaN
    mask = ~(np.isnan(x) | np.isnan(y))
    x = x[mask]
    y = y[mask]
    
    if len(x) < 2:
        return None
    
    fig, ax = plt.subplots(figsize=(8, 5))
    
    # Detect outliers in y values
    if remove_outliers:
        outlier_mask = detect_outliers_mad(y, outlier_threshold)
        x_clean = x[~outlier_mask]
        y_clean = y[~outlier_mask]
        x_outlier = x[outlier_mask]
        y_outlier = y[outlier_mask]
    else:
        x_clean, y_clean = x, y
        x_outlier, y_outlier = np.array([]), np.array([])
    
    # Scatter plot - clean points
    ax.scatter(x_clean, y_clean, alpha=0.7, edgecolors='black', linewidth=0.5,
               label=f'Data ({len(x_clean)} pts)', zorder=2)
    
    # Scatter plot - outliers as red X
    if len(x_outlier) > 0:
        ax.scatter(x_outlier, y_outlier, c='red', marker='x', s=100, linewidths=2,
                   label=f'Outliers ({len(x_outlier)} removed)', zorder=3)
    
    # Regression line (on clean data only)
    reg = perform_regression(x_clean, y_clean)
    if reg and len(x_clean) >= 2:
        x_line = np.linspace(x.min(), x.max(), 100)
        y_line = reg["slope"] * x_line + reg["intercept"]
        ax.plot(x_line, y_line, 'g-', linewidth=2, zorder=1,
                label=f'Fit: y = {reg["slope"]:.4g}x + {reg["intercept"]:.4g}\nR² = {reg["r_squared"]:.4f}')
    
    ax.legend(loc='best')
    ax.set_xlabel(x_label)
    ax.set_ylabel(y_label)
    ax.set_title(f"{title}\n{opcode}")
    ax.grid(True, alpha=0.3)
    
    plt.tight_layout()
    return fig


def fig_to_base64(fig: plt.Figure) -> str:
    """Convert matplotlib figure to base64 string."""
    buf = io.BytesIO()
    fig.savefig(buf, format='png', dpi=100, bbox_inches='tight')
    buf.seek(0)
    img_base64 = base64.b64encode(buf.read()).decode('utf-8')
    plt.close(fig)
    return img_base64


def save_fig(fig: plt.Figure, path: Path) -> None:
    """Save matplotlib figure to file."""
    fig.savefig(path, format='png', dpi=100, bbox_inches='tight')
    plt.close(fig)


# =============================================================================
# HTML Generation
# =============================================================================

HTML_TEMPLATE = """<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <style>
        :root {{
            --bg-color: #ffffff;
            --text-color: #333333;
            --heading-color: #1a1a1a;
            --border-color: #e0e0e0;
            --table-header-bg: #f5f5f5;
            --table-row-hover: #f9f9f9;
            --code-bg: #f4f4f4;
            --link-color: #0066cc;
        }}
        
        * {{
            box-sizing: border-box;
        }}
        
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
            line-height: 1.6;
            color: var(--text-color);
            max-width: 1200px;
            margin: 0 auto;
            padding: 2rem;
            background-color: var(--bg-color);
        }}
        
        h1, h2, h3, h4 {{
            color: var(--heading-color);
            margin-top: 2rem;
            margin-bottom: 1rem;
        }}
        
        h1 {{
            font-size: 2rem;
            border-bottom: 2px solid var(--border-color);
            padding-bottom: 0.5rem;
        }}
        
        h2 {{
            font-size: 1.5rem;
            border-bottom: 1px solid var(--border-color);
            padding-bottom: 0.3rem;
        }}
        
        h3 {{
            font-size: 1.25rem;
        }}
        
        table {{
            border-collapse: collapse;
            width: 100%;
            margin: 1rem 0;
            font-size: 0.9rem;
        }}
        
        th, td {{
            border: 1px solid var(--border-color);
            padding: 0.5rem 0.75rem;
            text-align: left;
        }}
        
        th {{
            background-color: var(--table-header-bg);
            font-weight: 600;
        }}
        
        tr:hover {{
            background-color: var(--table-row-hover);
        }}
        
        code {{
            background-color: var(--code-bg);
            padding: 0.2rem 0.4rem;
            border-radius: 3px;
            font-family: 'Consolas', 'Monaco', monospace;
            font-size: 0.9em;
        }}
        
        pre {{
            background-color: var(--code-bg);
            padding: 1rem;
            border-radius: 5px;
            overflow-x: auto;
        }}
        
        pre code {{
            padding: 0;
            background: none;
        }}
        
        img {{
            max-width: 100%;
            height: auto;
            display: block;
            margin: 1rem auto;
            border: 1px solid var(--border-color);
            border-radius: 5px;
        }}
        
        a {{
            color: var(--link-color);
            text-decoration: none;
        }}
        
        a:hover {{
            text-decoration: underline;
        }}
        
        ul, ol {{
            margin: 1rem 0;
            padding-left: 2rem;
        }}
        
        li {{
            margin: 0.25rem 0;
        }}
        
        hr {{
            border: none;
            border-top: 1px solid var(--border-color);
            margin: 2rem 0;
        }}
        
        .chart-container {{
            text-align: center;
            margin: 1.5rem 0;
        }}
        
        @media (max-width: 768px) {{
            body {{
                padding: 1rem;
            }}
            
            table {{
                font-size: 0.8rem;
            }}
            
            th, td {{
                padding: 0.3rem 0.5rem;
            }}
        }}
    </style>
</head>
<body>
{content}
</body>
</html>
"""


def markdown_to_html(md_content: str, title: str) -> str:
    """Convert markdown content to styled HTML."""
    # Convert markdown to HTML
    html_content = markdown.markdown(
        md_content,
        extensions=['tables', 'fenced_code', 'toc']
    )
    
    # Wrap in template
    return HTML_TEMPLATE.format(title=title, content=html_content)


# =============================================================================
# Report Generation
# =============================================================================

def generate_report(
    df: pd.DataFrame,
    regression_df: pd.DataFrame,
    hardware_info: dict,
    prover_name: str,
    prover_version: str,
    mode: str,
    output_dir: Path,
    inline_images: bool,
) -> str:
    """Generate the markdown report."""
    now = datetime.now()
    timestamp = now.strftime("%Y-%m-%d %H:%M:%S")
    date_str = now.strftime("%Y%m%d")
    
    lines = []
    
    # Title
    lines.append(f"# ZK Gas Benchmark Report {now.strftime('%Y-%m-%d')} ({prover_name}, {mode})")
    lines.append("")
    
    # Context
    lines.append("## Context")
    lines.append("")
    lines.append(f"- **Generated**: {timestamp}")
    lines.append(f"- **Prover**: {prover_version}")
    lines.append(f"- **Mode**: {mode}")
    
    # Calculate total proving time
    total_proving_time = 0.0
    if "proving_time_s" in df.columns:
        total_proving_time = df["proving_time_s"].sum()
        lines.append(f"- **Total Proving Time**: {format_time_hms(total_proving_time)}")
    
    if hardware_info:
        lines.append(f"- **CPU**: {hardware_info.get('cpu_model', 'N/A')}")
        lines.append(f"- **RAM**: {hardware_info.get('total_ram_gib', 'N/A')} GiB")
        gpus = hardware_info.get('gpus', [])
        if gpus:
            # GPUs can be list of strings or list of dicts
            gpu_names = []
            for gpu in gpus:
                if isinstance(gpu, dict):
                    gpu_names.append(gpu.get('model', gpu.get('name', str(gpu))))
                else:
                    gpu_names.append(str(gpu))
            # Count duplicates
            from collections import Counter
            gpu_counts = Counter(gpu_names)
            gpu_str = ", ".join(f"{count}x {name}" if count > 1 else name for name, count in gpu_counts.items())
            lines.append(f"- **GPUs**: {gpu_str}")
        else:
            lines.append("- **GPUs**: None")
    lines.append("")
    
    # Results Tables
    lines.append("## Regression Results")
    lines.append("")
    
    has_zkcycles = "gas_zkcycles_slope" in regression_df.columns and regression_df["gas_zkcycles_slope"].notna().any()
    has_proving = "gas_proving_slope" in regression_df.columns and regression_df["gas_proving_slope"].notna().any()
    has_both = "zkcycles_proving_slope" in regression_df.columns and regression_df["zkcycles_proving_slope"].notna().any()
    
    # Calculate total proving time per opcode for the table
    opcode_proving_times = {}
    if "proving_time_s" in df.columns:
        for opcode in df["opcode"].unique():
            opcode_df = df[df["opcode"] == opcode]
            opcode_proving_times[opcode] = opcode_df["proving_time_s"].sum()
    
    # Combined Summary Table
    def generate_combined_summary_table(regression_df: pd.DataFrame, opcode_times: dict, grand_total: float) -> list:
        """Generate combined summary table rows."""
        table_lines = []
        table_lines.append("### Regression Results")
        table_lines.append("")
        table_lines.append("| Opcode | Samples | Pts | Outliers | Max Ops | Max Gas | Total Time | % Total | Time/Gas (R²) | Cycles/Gas (R²) |")
        table_lines.append("|--------|---------|-----|----------|---------|---------|------------|---------|---------------|-----------------|")
        
        # Prepare data with R² quality flag
        valid_reg = regression_df.dropna(subset=["gas_proving_slope"]).copy()
        valid_reg["time_gas_r2"] = valid_reg["gas_proving_r2"].fillna(0)
        valid_reg["is_good_r2"] = valid_reg["time_gas_r2"] >= 0.7  # Green or Yellow
        
        # Add total proving time to the dataframe for sorting
        valid_reg["total_proving_time"] = valid_reg["opcode"].map(opcode_times).fillna(0)
        
        # Calculate threshold for "relatively long" (top 10% or > 5% of total)
        if grand_total > 0:
            pct_threshold = 5.0  # Highlight if > 5% of total
        else:
            pct_threshold = float('inf')
        
        # Sort: good R² first (descending by Time/Gas), then bad R² (descending by Time/Gas)
        good_r2 = valid_reg[valid_reg["is_good_r2"]].sort_values("gas_proving_slope", ascending=False)
        bad_r2 = valid_reg[~valid_reg["is_good_r2"]].sort_values("gas_proving_slope", ascending=False)
        
        def format_summary_row(row):
            opcode = row["opcode"]
            
            # Sample count
            n_samples = row.get("n_samples", 1)
            samples_str = str(int(n_samples)) if pd.notna(n_samples) else "1"
            
            # Data points
            n_points = row.get("n_points", 0)
            pts_str = str(int(n_points)) if pd.notna(n_points) else "N/A"
            
            # Outliers removed (sum of proving + cycles outliers)
            n_outliers_proving = row.get("n_outliers_proving", 0)
            n_outliers_cycles = row.get("n_outliers_cycles", 0)
            n_outliers = max(n_outliers_proving, n_outliers_cycles) if pd.notna(n_outliers_proving) and pd.notna(n_outliers_cycles) else 0
            if n_outliers > 0:
                outliers_str = f'<span style="color: #dc3545;">{int(n_outliers)}</span>'
            else:
                outliers_str = "0"
            
            # Max Op Count
            max_op_count = row.get("max_op_count")
            if pd.notna(max_op_count):
                max_ops_str = format_number(int(max_op_count))
            else:
                max_ops_str = "N/A"
            
            # Max Gas
            max_gas = row.get("max_gas")
            if pd.notna(max_gas):
                max_gas_str = format_number(max_gas)
            else:
                max_gas_str = "N/A"
            
            # Total Proving Time for this opcode
            total_time = opcode_times.get(opcode, 0)
            total_time_str = format_time_hms(total_time)
            
            # Percentage of total
            if grand_total > 0:
                pct = (total_time / grand_total) * 100
                # Highlight in red if relatively long (> 5% of total)
                if pct >= pct_threshold:
                    pct_str = f'<span style="color: #dc3545; font-weight: bold;">{pct:.1f}%</span>'
                    total_time_str = f'<span style="color: #dc3545; font-weight: bold;">{total_time_str}</span>'
                else:
                    pct_str = f"{pct:.1f}%"
            else:
                pct_str = "N/A"
            
            # Time/Gas
            time_gas = row.get("gas_proving_slope")
            time_gas_r2 = row.get("gas_proving_r2")
            if pd.notna(time_gas) and pd.notna(time_gas_r2):
                time_gas_str = f"{format_time(time_gas)} ({format_r2(time_gas_r2)})"
            else:
                time_gas_str = "N/A"
            
            # Cycles/Gas
            cycles_gas = row.get("gas_zkcycles_slope")
            cycles_gas_r2 = row.get("gas_zkcycles_r2")
            if pd.notna(cycles_gas) and pd.notna(cycles_gas_r2):
                cycles_gas_str = f"{format_number(cycles_gas)} ({format_r2(cycles_gas_r2)})"
            else:
                cycles_gas_str = "N/A"
            
            return f"| {opcode} | {samples_str} | {pts_str} | {outliers_str} | {max_ops_str} | {max_gas_str} | {total_time_str} | {pct_str} | {time_gas_str} | {cycles_gas_str} |"
        
        # Add good R² rows first
        for _, row in good_r2.iterrows():
            table_lines.append(format_summary_row(row))
        
        # Separator between good and bad R²
        if len(good_r2) > 0 and len(bad_r2) > 0:
            table_lines.append("|--------|---------|-----|----------|---------|---------|------------|---------|---------------|-----------------|")
        
        # Add bad R² rows
        for _, row in bad_r2.iterrows():
            table_lines.append(format_summary_row(row))
        
        table_lines.append("")
        return table_lines
    
    if has_proving:
        # Time/Gas Bar Chart FIRST (R² >= 0.7)
        lines.append("### Time/Gas Bar Chart (R² ≥ 0.7)")
        lines.append("")
        lines.append("*Only opcodes/precompiles with R² ≥ 0.7 are shown.*")
        lines.append("")
        
        # Filter to R² >= 0.7 and sort by slope descending
        time_gas_df = regression_df.dropna(subset=["gas_proving_slope", "gas_proving_r2"]).copy()
        time_gas_df = time_gas_df[time_gas_df["gas_proving_r2"] >= 0.7]
        time_gas_df = time_gas_df.sort_values("gas_proving_slope", ascending=True)  # ascending for horizontal bar
        
        if len(time_gas_df) > 0:
            fig, ax = plt.subplots(figsize=(12, max(8, len(time_gas_df) * 0.3)))
            
            y_pos = np.arange(len(time_gas_df))
            ax.barh(y_pos, time_gas_df["gas_proving_slope"] * 1e6, color="#28a745")  # All green
            ax.set_yticks(y_pos)
            ax.set_yticklabels(time_gas_df["opcode"])
            ax.set_xlabel("Time/Gas (µs)")
            ax.set_title(f"{prover_name} Proving Time per Gas Unit (R² ≥ 0.7)")
            ax.grid(True, alpha=0.3, axis='x')
            
            # Set minimum x-axis length to 1200µs for consistent comparison across zkVMs
            current_xlim = ax.get_xlim()
            ax.set_xlim(left=0, right=max(current_xlim[1], 1500))
            
            # Add value labels
            for i, (_, row) in enumerate(time_gas_df.iterrows()):
                val = row["gas_proving_slope"] * 1e6
                ax.text(val + 0.5, i, f"{val:.1f}µs", va='center', fontsize=8)
            
            plt.tight_layout()
            
            # Use temp directory for plots
            plots_dir_temp = output_dir / "plots"
            plots_dir_temp.mkdir(parents=True, exist_ok=True)
            
            if inline_images:
                img_base64 = fig_to_base64(fig)
                lines.append(f"![Time/Gas Bar Chart](data:image/png;base64,{img_base64})")
            else:
                plot_path = plots_dir_temp / "bar_time_per_gas.png"
                fig.savefig(plot_path, dpi=150, bbox_inches='tight')
                lines.append(f"![Time/Gas Bar Chart](plots/bar_time_per_gas.png)")
            
            plt.close(fig)
            lines.append("")
    
    # Cycles/Gas Bar Chart SECOND (R² >= 0.9)
    if has_zkcycles:
        lines.append("### Cycles/Gas Bar Chart (R² ≥ 0.9)")
        lines.append("")
        lines.append("*Only opcodes/precompiles with R² ≥ 0.9 (green) are shown.*")
        lines.append("")
        
        # Filter to R² >= 0.9 and sort by slope descending
        cycles_gas_df = regression_df.dropna(subset=["gas_zkcycles_slope", "gas_zkcycles_r2"]).copy()
        cycles_gas_df = cycles_gas_df[cycles_gas_df["gas_zkcycles_r2"] >= 0.9]
        cycles_gas_df = cycles_gas_df.sort_values("gas_zkcycles_slope", ascending=True)  # ascending for horizontal bar
        
        if len(cycles_gas_df) > 0:
            # Use temp directory for plots
            plots_dir_temp = output_dir / "plots"
            plots_dir_temp.mkdir(parents=True, exist_ok=True)
            
            fig, ax = plt.subplots(figsize=(12, max(8, len(cycles_gas_df) * 0.3)))
            
            y_pos = np.arange(len(cycles_gas_df))
            ax.barh(y_pos, cycles_gas_df["gas_zkcycles_slope"], color="#28a745")  # All green
            ax.set_yticks(y_pos)
            ax.set_yticklabels(cycles_gas_df["opcode"])
            ax.set_xlabel("Cycles/Gas")
            ax.set_title(f"{prover_name} ZK Cycles per Gas Unit (R² ≥ 0.9)")
            ax.grid(True, alpha=0.3, axis='x')
            
            # Add value labels
            for i, (_, row) in enumerate(cycles_gas_df.iterrows()):
                val = row["gas_zkcycles_slope"]
                ax.text(val + 1, i, f"{format_number(val)}", va='center', fontsize=8)
            
            plt.tight_layout()
            
            if inline_images:
                img_base64 = fig_to_base64(fig)
                lines.append(f"![Cycles/Gas Bar Chart](data:image/png;base64,{img_base64})")
            else:
                plot_path = plots_dir_temp / "bar_cycles_per_gas.png"
                fig.savefig(plot_path, dpi=150, bbox_inches='tight')
                lines.append(f"![Cycles/Gas Bar Chart](plots/bar_cycles_per_gas.png)")
            
            plt.close(fig)
            lines.append("")
    
    if has_proving:
        # Then the Regression Results table
        lines.extend(generate_combined_summary_table(regression_df, opcode_proving_times, total_proving_time))
    elif has_zkcycles:
        # Execution-only mode: show a simplified table with Max Gas, Max ZK Cycles, Cycles/Gas
        lines.append("### Regression Results (Execution Only)")
        lines.append("")
        lines.append("| Opcode | Max Ops | Max Gas | Max ZK Cycles | Cycles/Gas (R²) |")
        lines.append("|--------|---------|---------|---------------|-----------------|")
        
        # Sort by Cycles/Gas R² descending, then by max_zkcycles descending
        exec_df = regression_df.dropna(subset=["gas_zkcycles_slope"]).copy()
        exec_df = exec_df.sort_values(["gas_zkcycles_r2", "max_zkcycles"], ascending=[False, False])
        
        for _, row in exec_df.iterrows():
            opcode = row["opcode"]
            max_ops = format_number(int(row.get("max_op_count", 0))) if pd.notna(row.get("max_op_count")) else "N/A"
            max_gas = format_number(row.get("max_gas", 0)) if pd.notna(row.get("max_gas")) else "N/A"
            max_zkcycles = format_number(row.get("max_zkcycles", 0)) if pd.notna(row.get("max_zkcycles")) else "N/A"
            
            cycles_gas = row.get("gas_zkcycles_slope")
            cycles_gas_r2 = row.get("gas_zkcycles_r2")
            if pd.notna(cycles_gas) and pd.notna(cycles_gas_r2):
                cycles_gas_str = f"{format_number(cycles_gas)} ({format_r2(cycles_gas_r2)})"
            else:
                cycles_gas_str = "N/A"

            lines.append(f"| {opcode} | {max_ops} | {max_gas} | {max_zkcycles} | {cycles_gas_str} |")
        
        lines.append("")
    
    def sort_by_slope_with_r2_threshold(df: pd.DataFrame, slope_col: str, r2_col: str, threshold: float = 0.7) -> pd.DataFrame:
        """Sort by slope descending, but put low R² values at the bottom."""
        df = df.dropna(subset=[slope_col]).copy()
        df["_high_r2"] = df[r2_col] >= threshold
        df = df.sort_values(["_high_r2", slope_col], ascending=[False, False])
        return df.drop(columns=["_high_r2"])
    
    # Collect detailed tables and plots for appendix
    appendix_detailed_lines = []
    
    # Gas ↔ Proving Time (moved to appendix)
    if has_proving:
        appendix_detailed_lines.append("### Gas ↔ Proving Time")
        appendix_detailed_lines.append("")
        appendix_detailed_lines.append("| Opcode | Time/Gas | R² | Std Error |")
        appendix_detailed_lines.append("|--------|----------|-----|-----------|")
        sorted_df = sort_by_slope_with_r2_threshold(regression_df, "gas_proving_slope", "gas_proving_r2")
        for _, row in sorted_df.iterrows():
            time_per_gas = format_time(row['gas_proving_slope'])
            appendix_detailed_lines.append(
                f"| {row['opcode']} | {time_per_gas}/gas | "
                f"{format_r2(row['gas_proving_r2'])} | {format_time(row['gas_proving_std_err'])} |"
            )
        appendix_detailed_lines.append("")
    
    # Gas ↔ ZK Cycles (moved to appendix)
    if has_zkcycles:
        appendix_detailed_lines.append("### Gas ↔ ZK Cycles")
        appendix_detailed_lines.append("")
        appendix_detailed_lines.append("| Opcode | Cycles/Gas | R² | Std Error |")
        appendix_detailed_lines.append("|--------|------------|-----|-----------|")
        sorted_df = sort_by_slope_with_r2_threshold(regression_df, "gas_zkcycles_slope", "gas_zkcycles_r2")
        for _, row in sorted_df.iterrows():
            appendix_detailed_lines.append(
                f"| {row['opcode']} | {format_number(row['gas_zkcycles_slope'])} | "
                f"{format_r2(row['gas_zkcycles_r2'])} | {format_number(row['gas_zkcycles_std_err'])} |"
            )
        appendix_detailed_lines.append("")
    
    if has_both:
        appendix_detailed_lines.append("### ZK Cycles ↔ Proving Time")
        appendix_detailed_lines.append("")
        appendix_detailed_lines.append("| Opcode | Time/Cycle | R² | Std Error |")
        appendix_detailed_lines.append("|--------|------------|-----|-----------|")
        sorted_df = sort_by_slope_with_r2_threshold(regression_df, "zkcycles_proving_slope", "zkcycles_proving_r2")
        for _, row in sorted_df.iterrows():
            time_per_cycle = format_time(row['zkcycles_proving_slope'])
            appendix_detailed_lines.append(
                f"| {row['opcode']} | {time_per_cycle}/cycle | "
                f"{format_r2(row['zkcycles_proving_r2'])} | {format_time(row['zkcycles_proving_std_err'])} |"
            )
        appendix_detailed_lines.append("")
        
        # Create a separate section for Proving time vs ZK Cycles plots
        # These will be inserted before Regression Charts
        proving_vs_cycles_lines = []
        proving_vs_cycles_lines.append("## Proving time vs ZK Cycles")
        proving_vs_cycles_lines.append("")
        proving_vs_cycles_lines.append("This section examines whether ZK cycles are a good proxy for proving time.")
        proving_vs_cycles_lines.append("")
        
        # Time/Cycles Bar Chart (R² >= 0.7)
        proving_vs_cycles_lines.append("### Time/Cycles Bar Chart (R² ≥ 0.7)")
        proving_vs_cycles_lines.append("")
        proving_vs_cycles_lines.append("*Only opcodes/precompiles with R² ≥ 0.7 are shown.*")
        proving_vs_cycles_lines.append("")
        
        # Filter to R² >= 0.7 and sort by slope descending
        time_cycles_df = regression_df.dropna(subset=["zkcycles_proving_slope", "zkcycles_proving_r2"]).copy()
        time_cycles_df = time_cycles_df[time_cycles_df["zkcycles_proving_r2"] >= 0.7]
        time_cycles_df = time_cycles_df.sort_values("zkcycles_proving_slope", ascending=True)  # ascending for horizontal bar
        
        if len(time_cycles_df) > 0:
            plots_dir_temp = output_dir / "plots"
            plots_dir_temp.mkdir(parents=True, exist_ok=True)
            
            fig, ax = plt.subplots(figsize=(12, max(8, len(time_cycles_df) * 0.3)))
            
            y_pos = np.arange(len(time_cycles_df))
            # Convert slope to nanoseconds for better readability (slope is seconds/cycle)
            ax.barh(y_pos, time_cycles_df["zkcycles_proving_slope"] * 1e9, color="#28a745")  # All green
            ax.set_yticks(y_pos)
            ax.set_yticklabels(time_cycles_df["opcode"])
            ax.set_xlabel("Time/Cycle (ns)")
            ax.set_title(f"{prover_name} Proving Time per ZK Cycle (R² ≥ 0.7)")
            ax.grid(True, alpha=0.3, axis='x')
            
            # Add value labels
            for i, (_, row) in enumerate(time_cycles_df.iterrows()):
                val = row["zkcycles_proving_slope"] * 1e9  # nanoseconds
                ax.text(val + 0.5, i, f"{val:.1f}ns", va='center', fontsize=8)
            
            plt.tight_layout()
            
            if inline_images:
                img_base64 = fig_to_base64(fig)
                proving_vs_cycles_lines.append(f"![Time/Cycles Bar Chart](data:image/png;base64,{img_base64})")
            else:
                plot_path = plots_dir_temp / "bar_time_per_cycle.png"
                fig.savefig(plot_path, dpi=150, bbox_inches='tight')
                proving_vs_cycles_lines.append(f"![Time/Cycles Bar Chart](plots/bar_time_per_cycle.png)")
            
            plt.close(fig)
            proving_vs_cycles_lines.append("")
        
        # Combined scatter plot for ZK Cycles ↔ Proving Time (all opcodes)
        proving_vs_cycles_lines.append("### Combined ZK Cycles ↔ Proving Time (All Opcodes)")
        proving_vs_cycles_lines.append("")
        
        # Create combined scatter plot
        fig, ax = plt.subplots(figsize=(12, 8))
        
        # Get valid data
        valid_df = df.dropna(subset=["zk_cycles", "proving_time_s"])
        x_all = valid_df["zk_cycles"].values.astype(float)
        y_all = valid_df["proving_time_s"].values.astype(float)
        
        # Order opcodes by gas ↔ zk cycles slope (descending) for legend order
        slope_order = regression_df.dropna(subset=["gas_zkcycles_slope"]).sort_values(
            "gas_zkcycles_slope", ascending=False
        )["opcode"].tolist()
        # Add any opcodes not in regression_df at the end
        all_opcodes = valid_df["opcode"].unique()
        ordered_opcodes = [op for op in slope_order if op in all_opcodes]
        ordered_opcodes += [op for op in all_opcodes if op not in ordered_opcodes]
        
        # Color by opcode category (alphabetical for consistent colors)
        sorted_opcodes = sorted(valid_df["opcode"].unique())
        colors = plt.cm.tab20(np.linspace(0, 1, len(sorted_opcodes)))
        color_map = {op: colors[i] for i, op in enumerate(sorted_opcodes)}
        
        for opcode in ordered_opcodes:
            opcode_data = valid_df[valid_df["opcode"] == opcode]
            ax.scatter(
                opcode_data["zk_cycles"], 
                opcode_data["proving_time_s"],
                c=[color_map[opcode]],
                label=opcode,
                alpha=0.7,
                s=30
            )
        
        # Overall regression line
        if len(x_all) > 1:
            reg = perform_regression(x_all, y_all)
            if reg:
                x_line = np.linspace(x_all.min(), x_all.max(), 100)
                y_line = reg["slope"] * x_line + reg["intercept"]
                ax.plot(x_line, y_line, 'k--', linewidth=2, 
                        label=f'Overall: {format_time(reg["slope"])}/cycle, R²={reg["r_squared"]:.4f}')
        
        ax.set_xlabel("ZK Cycles")
        ax.set_ylabel("Proving Time (s)")
        ax.set_title(f"{prover_name} ZK Cycles vs Proving Time (All Opcodes Combined)")
        ax.legend(loc='upper left', fontsize=6, ncol=3, bbox_to_anchor=(1.02, 1))
        ax.grid(True, alpha=0.3)
        plt.tight_layout()
        
        plots_dir_temp = output_dir / "plots" if not inline_images else None
        if plots_dir_temp:
            plots_dir_temp.mkdir(parents=True, exist_ok=True)
        
        if inline_images:
            img_base64 = fig_to_base64(fig)
            proving_vs_cycles_lines.append(f"![Combined ZK Cycles vs Proving Time](data:image/png;base64,{img_base64})")
        else:
            plot_path = plots_dir_temp / "combined_zkcycles_proving.png"
            fig.savefig(plot_path, dpi=150, bbox_inches='tight')
            proving_vs_cycles_lines.append(f"![Combined ZK Cycles vs Proving Time](plots/combined_zkcycles_proving.png)")
        
        plt.close(fig)
        proving_vs_cycles_lines.append("")
        
        # Classify opcodes by their proving efficiency ratio
        # BLS12 precompiles have much faster proving per cycle
        if reg:
            bls12_opcodes = []
            regular_opcodes = []
            
            for opcode in ordered_opcodes:
                opcode_data = valid_df[valid_df["opcode"] == opcode]
                x_op = opcode_data["zk_cycles"].values.astype(float)
                y_op = opcode_data["proving_time_s"].values.astype(float)
                
                # Calculate expected y values on regression line
                y_expected = reg["slope"] * np.mean(x_op) + reg["intercept"]
                
                # Calculate ratio of actual to expected proving time
                ratio = np.mean(y_op) / y_expected if y_expected > 0 else 1.0
                
                # BLS12 precompiles have ratio < 0.55 (much faster proving per cycle)
                if ratio < 0.55:
                    bls12_opcodes.append(opcode)
                else:
                    regular_opcodes.append(opcode)
            
            # Plot for REGULAR opcodes (most opcodes/precompiles)
            # Note: BLS12 precompiles and point_evaluation are already excluded (classified separately)
            if regular_opcodes:
                excluded_list = ", ".join(sorted(bls12_opcodes)) if bls12_opcodes else "none"
                proving_vs_cycles_lines.append(f"### Excluding: {excluded_list}")
                proving_vs_cycles_lines.append("")
                
                fig, ax = plt.subplots(figsize=(12, 8))
                regular_df = valid_df[valid_df["opcode"].isin(regular_opcodes)]
                
                for opcode in [op for op in ordered_opcodes if op in regular_opcodes]:
                    opcode_data = regular_df[regular_df["opcode"] == opcode]
                    ax.scatter(
                        opcode_data["zk_cycles"], 
                        opcode_data["proving_time_s"],
                        c=[color_map[opcode]],
                        label=opcode,
                        alpha=0.7,
                        s=30
                    )
                
                # New regression line based on this subset
                x_regular = regular_df["zk_cycles"].values.astype(float)
                y_regular = regular_df["proving_time_s"].values.astype(float)
                reg_regular = perform_regression(x_regular, y_regular)
                if reg_regular:
                    x_line = np.linspace(x_regular.min(), x_regular.max(), 100)
                    y_line = reg_regular["slope"] * x_line + reg_regular["intercept"]
                    ax.plot(x_line, y_line, 'k--', linewidth=2, 
                            label=f'Regression: {format_time(reg_regular["slope"])}/cycle, R²={reg_regular["r_squared"]:.4f}')
                
                ax.set_xlabel("ZK Cycles")
                ax.set_ylabel("Proving Time (s)")
                ax.set_title(f"{prover_name} Regular Opcodes ({len(regular_opcodes)} opcodes)")
                ax.legend(loc='upper left', fontsize=6, ncol=3, bbox_to_anchor=(1.02, 1))
                ax.grid(True, alpha=0.3)
                plt.tight_layout()
                
                if inline_images:
                    img_base64 = fig_to_base64(fig)
                    proving_vs_cycles_lines.append(f"![Regular Opcodes](data:image/png;base64,{img_base64})")
                else:
                    plot_path = plots_dir_temp / "combined_zkcycles_regular.png"
                    fig.savefig(plot_path, dpi=150, bbox_inches='tight')
                    proving_vs_cycles_lines.append(f"![Regular Opcodes](plots/combined_zkcycles_regular.png)")
                
                plt.close(fig)
                proving_vs_cycles_lines.append("")
                
                # Additional plot excluding blake2f and modexp (outliers)
                outlier_opcodes = ["blake2f", "modexp"]
                regular_no_outliers = [op for op in regular_opcodes if op not in outlier_opcodes]
                
                if regular_no_outliers and len(regular_no_outliers) < len(regular_opcodes):
                    excluded_here = sorted(bls12_opcodes + outlier_opcodes)
                    proving_vs_cycles_lines.append(f"### Excluding: {', '.join(excluded_here)}")
                    proving_vs_cycles_lines.append("")
                    
                    fig, ax = plt.subplots(figsize=(12, 8))
                    regular_no_outliers_df = valid_df[valid_df["opcode"].isin(regular_no_outliers)]
                    
                    for opcode in [op for op in ordered_opcodes if op in regular_no_outliers]:
                        opcode_data = regular_no_outliers_df[regular_no_outliers_df["opcode"] == opcode]
                        ax.scatter(
                            opcode_data["zk_cycles"], 
                            opcode_data["proving_time_s"],
                            c=[color_map[opcode]],
                            label=opcode,
                            alpha=0.7,
                            s=30
                        )
                    
                    # New regression line based on this subset
                    x_no_outliers = regular_no_outliers_df["zk_cycles"].values.astype(float)
                    y_no_outliers = regular_no_outliers_df["proving_time_s"].values.astype(float)
                    reg_no_outliers = perform_regression(x_no_outliers, y_no_outliers)
                    if reg_no_outliers:
                        x_line = np.linspace(x_no_outliers.min(), x_no_outliers.max(), 100)
                        y_line = reg_no_outliers["slope"] * x_line + reg_no_outliers["intercept"]
                        ax.plot(x_line, y_line, 'k--', linewidth=2, 
                                label=f'Regression: {format_time(reg_no_outliers["slope"])}/cycle, R²={reg_no_outliers["r_squared"]:.4f}')
                    
                    ax.set_xlabel("ZK Cycles")
                    ax.set_ylabel("Proving Time (s)")
                    ax.set_title(f"{prover_name} Regular Opcodes excl. blake2f/modexp ({len(regular_no_outliers)} opcodes)")
                    ax.legend(loc='upper left', fontsize=6, ncol=3, bbox_to_anchor=(1.02, 1))
                    ax.grid(True, alpha=0.3)
                    plt.tight_layout()
                    
                    if inline_images:
                        img_base64 = fig_to_base64(fig)
                        proving_vs_cycles_lines.append(f"![Regular Opcodes No Outliers](data:image/png;base64,{img_base64})")
                    else:
                        plot_path = plots_dir_temp / "combined_zkcycles_regular_no_outliers.png"
                        fig.savefig(plot_path, dpi=150, bbox_inches='tight')
                        proving_vs_cycles_lines.append(f"![Regular Opcodes No Outliers](plots/combined_zkcycles_regular_no_outliers.png)")
                    
                    plt.close(fig)
                    proving_vs_cycles_lines.append("")
                
                # Additional plot excluding blake2f, modexp, keccak256, and log opcodes
                extra_outliers = ["blake2f", "modexp", "keccak256", "log0", "log1", "log2", "log3", "log4"]
                regular_minimal = [op for op in regular_opcodes if op not in extra_outliers]
                
                if regular_minimal and len(regular_minimal) < len(regular_no_outliers):
                    excluded_here = sorted(bls12_opcodes + extra_outliers)
                    proving_vs_cycles_lines.append(f"### Excluding: {', '.join(excluded_here)}")
                    proving_vs_cycles_lines.append("")
                    
                    fig, ax = plt.subplots(figsize=(12, 8))
                    regular_minimal_df = valid_df[valid_df["opcode"].isin(regular_minimal)]
                    
                    for opcode in [op for op in ordered_opcodes if op in regular_minimal]:
                        opcode_data = regular_minimal_df[regular_minimal_df["opcode"] == opcode]
                        ax.scatter(
                            opcode_data["zk_cycles"], 
                            opcode_data["proving_time_s"],
                            c=[color_map[opcode]],
                            label=opcode,
                            alpha=0.7,
                            s=30
                        )
                    
                    # New regression line based on this subset
                    x_minimal = regular_minimal_df["zk_cycles"].values.astype(float)
                    y_minimal = regular_minimal_df["proving_time_s"].values.astype(float)
                    reg_minimal = perform_regression(x_minimal, y_minimal)
                    if reg_minimal:
                        x_line = np.linspace(x_minimal.min(), x_minimal.max(), 100)
                        y_line = reg_minimal["slope"] * x_line + reg_minimal["intercept"]
                        ax.plot(x_line, y_line, 'k--', linewidth=2, 
                                label=f'Regression: {format_time(reg_minimal["slope"])}/cycle, R²={reg_minimal["r_squared"]:.4f}')
                    
                    ax.set_xlabel("ZK Cycles")
                    ax.set_ylabel("Proving Time (s)")
                    ax.set_title(f"{prover_name} Regular Opcodes excl. blake2f/modexp/keccak256/log ({len(regular_minimal)} opcodes)")
                    ax.legend(loc='upper left', fontsize=6, ncol=3, bbox_to_anchor=(1.02, 1))
                    ax.grid(True, alpha=0.3)
                    plt.tight_layout()
                    
                    if inline_images:
                        img_base64 = fig_to_base64(fig)
                        proving_vs_cycles_lines.append(f"![Regular Opcodes Minimal](data:image/png;base64,{img_base64})")
                    else:
                        plot_path = plots_dir_temp / "combined_zkcycles_regular_minimal.png"
                        fig.savefig(plot_path, dpi=150, bbox_inches='tight')
                        proving_vs_cycles_lines.append(f"![Regular Opcodes Minimal](plots/combined_zkcycles_regular_minimal.png)")
                    
                    plt.close(fig)
                    proving_vs_cycles_lines.append("")
            
            # Plot for BLS12 precompiles (much faster proving per cycle)
            if bls12_opcodes:
                included_list = ", ".join(sorted(bls12_opcodes))
                proving_vs_cycles_lines.append(f"### Only: {included_list}")
                proving_vs_cycles_lines.append("")
                
                fig, ax = plt.subplots(figsize=(12, 8))
                bls12_df = valid_df[valid_df["opcode"].isin(bls12_opcodes)]
                
                for opcode in [op for op in ordered_opcodes if op in bls12_opcodes]:
                    opcode_data = bls12_df[bls12_df["opcode"] == opcode]
                    ax.scatter(
                        opcode_data["zk_cycles"], 
                        opcode_data["proving_time_s"],
                        c=[color_map[opcode]],
                        label=opcode,
                        alpha=0.7,
                        s=50
                    )
                
                # New regression line based on this subset
                x_bls12 = bls12_df["zk_cycles"].values.astype(float)
                y_bls12 = bls12_df["proving_time_s"].values.astype(float)
                reg_bls12 = perform_regression(x_bls12, y_bls12)
                if reg_bls12:
                    x_line = np.linspace(x_bls12.min(), x_bls12.max(), 100)
                    y_line = reg_bls12["slope"] * x_line + reg_bls12["intercept"]
                    ax.plot(x_line, y_line, 'k--', linewidth=2, 
                            label=f'Regression: {format_time(reg_bls12["slope"])}/cycle, R²={reg_bls12["r_squared"]:.4f}')
                
                ax.set_xlabel("ZK Cycles")
                ax.set_ylabel("Proving Time (s)")
                ax.set_title(f"{prover_name} BLS12-381 Precompiles ({len(bls12_opcodes)} precompiles)")
                ax.legend(loc='upper left', fontsize=8, bbox_to_anchor=(1.02, 1))
                ax.grid(True, alpha=0.3)
                plt.tight_layout()
                
                if inline_images:
                    img_base64 = fig_to_base64(fig)
                    proving_vs_cycles_lines.append(f"![BLS12 Precompiles](data:image/png;base64,{img_base64})")
                else:
                    plot_path = plots_dir_temp / "combined_zkcycles_bls12.png"
                    fig.savefig(plot_path, dpi=150, bbox_inches='tight')
                    proving_vs_cycles_lines.append(f"![BLS12 Precompiles](plots/combined_zkcycles_bls12.png)")
                
                plt.close(fig)
                proving_vs_cycles_lines.append("")
        
        # Add the proving_vs_cycles section to lines
        lines.extend(proving_vs_cycles_lines)
        lines.append("")
    
    # Benchmark Time Analysis section (before Regression Charts)
    if has_proving and opcode_proving_times:
        lines.append("## Benchmark Time Analysis")
        lines.append("")
        lines.append("This section shows opcodes/precompiles ranked by total proving time across all test runs.")
        lines.append("")
        
        # Bar chart of proving time by opcode
        lines.append("### Total Proving Time by Opcode/Precompile")
        lines.append("")
        
        # Prepare data for chart - sort by proving time descending
        time_data = [(op, time) for op, time in opcode_proving_times.items() if time > 0]
        time_data.sort(key=lambda x: x[1], reverse=True)
        
        if time_data:
            # Create horizontal bar chart
            fig, ax = plt.subplots(figsize=(12, max(8, len(time_data) * 0.35)))
            
            opcodes = [d[0] for d in reversed(time_data)]  # reverse for bottom-to-top
            times = [d[1] for d in reversed(time_data)]
            
            # Color bars based on percentage of total
            colors = []
            for t in times:
                pct = (t / total_proving_time) * 100 if total_proving_time > 0 else 0
                if pct >= 5:
                    colors.append("#dc3545")  # red for > 5%
                elif pct >= 2:
                    colors.append("#ffc107")  # yellow for 2-5%
                else:
                    colors.append("#28a745")  # green for < 2%
            
            y_pos = np.arange(len(opcodes))
            bars = ax.barh(y_pos, times, color=colors)
            ax.set_yticks(y_pos)
            ax.set_yticklabels(opcodes)
            ax.set_xlabel("Total Proving Time (seconds)")
            ax.set_title(f"{prover_name} Total Proving Time by Opcode/Precompile")
            ax.grid(True, alpha=0.3, axis='x')
            
            # Add value labels
            for i, (bar, t) in enumerate(zip(bars, times)):
                pct = (t / total_proving_time) * 100 if total_proving_time > 0 else 0
                label = f"{format_time_hms(t)} ({pct:.1f}%)"
                ax.text(bar.get_width() + max(times) * 0.01, bar.get_y() + bar.get_height()/2, 
                       label, va='center', fontsize=8)
            
            # Extend x-axis to fit labels
            ax.set_xlim(0, max(times) * 1.3)
            
            plt.tight_layout()
            
            plots_dir_temp = output_dir / "plots"
            plots_dir_temp.mkdir(parents=True, exist_ok=True)
            
            if inline_images:
                img_base64 = fig_to_base64(fig)
                lines.append(f"![Total Proving Time Bar Chart](data:image/png;base64,{img_base64})")
            else:
                plot_path = plots_dir_temp / "bar_total_proving_time.png"
                fig.savefig(plot_path, dpi=150, bbox_inches='tight')
                lines.append(f"![Total Proving Time Bar Chart](plots/bar_total_proving_time.png)")
            
            plt.close(fig)
            lines.append("")
        
        # Table ordered by proving time
        lines.append("### Proving Time Rankings")
        lines.append("")
        lines.append("*Ordered by total proving time (descending). Red = >5% of total, Yellow = 2-5%, Green = <2%.*")
        lines.append("")
        lines.append("| Rank | Opcode | Total Time | % Total | Max Ops | Max Gas | Max ZK Cycles | Time/Gas (R²) |")
        lines.append("|------|--------|------------|---------|---------|---------|---------------|---------------|")
        
        for rank, (opcode, total_time) in enumerate(time_data, 1):
            # Get regression data for this opcode
            opcode_row = regression_df[regression_df["opcode"] == opcode]
            
            # Format total time
            total_time_str = format_time_hms(total_time)
            
            # Percentage of total
            pct = (total_time / total_proving_time) * 100 if total_proving_time > 0 else 0
            if pct >= 5:
                pct_str = f'<span style="color: #dc3545; font-weight: bold;">{pct:.1f}%</span>'
                total_time_str = f'<span style="color: #dc3545; font-weight: bold;">{total_time_str}</span>'
            elif pct >= 2:
                pct_str = f'<span style="color: #ffc107; font-weight: bold;">{pct:.1f}%</span>'
                total_time_str = f'<span style="color: #ffc107; font-weight: bold;">{total_time_str}</span>'
            else:
                pct_str = f"{pct:.1f}%"
            
            if not opcode_row.empty:
                max_ops = opcode_row.get("max_op_count").iloc[0]
                max_ops_str = format_number(int(max_ops)) if pd.notna(max_ops) else "N/A"
                
                max_gas = opcode_row.get("max_gas").iloc[0]
                max_gas_str = format_number(max_gas) if pd.notna(max_gas) else "N/A"
                
                max_zkcycles = opcode_row.get("max_zkcycles").iloc[0]
                max_zkcycles_str = format_number(max_zkcycles) if pd.notna(max_zkcycles) else "N/A"
                
                time_gas = opcode_row.get("gas_proving_slope").iloc[0]
                time_gas_r2 = opcode_row.get("gas_proving_r2").iloc[0]
                if pd.notna(time_gas) and pd.notna(time_gas_r2):
                    time_gas_str = f"{format_time(time_gas)} ({format_r2(time_gas_r2)})"
                else:
                    time_gas_str = "N/A"
            else:
                max_ops_str = "N/A"
                max_gas_str = "N/A"
                max_zkcycles_str = "N/A"
                time_gas_str = "N/A"
            
            lines.append(f"| {rank} | {opcode} | {total_time_str} | {pct_str} | {max_ops_str} | {max_gas_str} | {max_zkcycles_str} | {time_gas_str} |")
        
        lines.append("")
        lines.append(f"**Total Proving Time**: {format_time_hms(total_proving_time)}")
        lines.append("")
    
    # Charts
    lines.append("## Regression Charts")
    lines.append("")
    
    plots_dir = output_dir / "plots" if not inline_images else None
    if plots_dir:
        plots_dir.mkdir(parents=True, exist_ok=True)
    
    chart_configs = []
    # Gas ↔ Proving Time FIRST (swapped order)
    if has_proving:
        chart_configs.append({
            "x_col": "gas_used",
            "y_col": "proving_time_s",
            "x_label": "Gas Used",
            "y_label": "Proving Time (s)",
            "title": f"{prover_name} Gas vs Proving Time",
            "section": "gas_proving",
            "slope_col": "gas_proving_slope",
            "r2_col": "gas_proving_r2",
        })
    # Gas ↔ ZK Cycles SECOND (swapped order)
    if has_zkcycles:
        chart_configs.append({
            "x_col": "gas_used",
            "y_col": "zk_cycles",
            "x_label": "Gas Used",
            "y_label": "ZK Cycles",
            "title": f"{prover_name} Gas vs ZK Cycles",
            "section": "gas_zkcycles",
            "slope_col": "gas_zkcycles_slope",
            "r2_col": "gas_zkcycles_r2",
        })
    if has_both:
        chart_configs.append({
            "x_col": "zk_cycles",
            "y_col": "proving_time_s",
            "x_label": "ZK Cycles",
            "y_label": "Proving Time (s)",
            "title": f"{prover_name} ZK Cycles vs Proving Time",
            "section": "zkcycles_proving",
            "slope_col": "zkcycles_proving_slope",
            "r2_col": "zkcycles_proving_r2",
        })
    
    for config in chart_configs:
        lines.append(f"### {config['title']}")
        lines.append("")
        
        slope_col = config.get("slope_col", f"{config['section']}_slope")
        r2_col = config.get("r2_col", f"{config['section']}_r2")
        
        # Sort opcodes by slope (high R² first, then by slope descending)
        if slope_col in regression_df.columns and r2_col in regression_df.columns:
            sorted_reg_df = sort_by_slope_with_r2_threshold(regression_df, slope_col, r2_col)
            sorted_opcodes = sorted_reg_df["opcode"].tolist()
        else:
            sorted_opcodes = sorted(df["opcode"].unique())
        
        for opcode in sorted_opcodes:
            fig = create_regression_plot(
                df, opcode,
                config["x_col"], config["y_col"],
                config["x_label"], config["y_label"],
                config["title"],
            )
            if fig is None:
                continue
            
            # Get regression summary
            opcode_row = regression_df[regression_df["opcode"] == opcode]
            
            if not opcode_row.empty and slope_col in opcode_row.columns:
                slope = opcode_row[slope_col].iloc[0]
                r2 = opcode_row[r2_col].iloc[0] if r2_col in opcode_row.columns else None
                if pd.notna(slope):
                    r2_str = f"{r2:.4f}" if pd.notna(r2) else "N/A"
                    # Format slope with appropriate unit
                    if "proving" in config["section"]:
                        slope_str = f"{format_time(slope)}/{'gas' if 'gas' in config['section'] else 'cycle'}"
                    else:
                        slope_str = f"{format_number(slope)} cycles/gas"
                    lines.append(f"**{opcode}**: Slope = {slope_str}, R² = {r2_str}")
                    lines.append("")
            
            if inline_images:
                img_base64 = fig_to_base64(fig)
                lines.append(f"![{opcode}](data:image/png;base64,{img_base64})")
            else:
                img_path = plots_dir / f"{config['section']}_{opcode}.png"
                save_fig(fig, img_path)
                lines.append(f"![{opcode}](plots/{config['section']}_{opcode}.png)")
            lines.append("")
    
    # Appendix
    lines.append("## Appendix: Per-Op-Count Regression")
    lines.append("")
    
    # Add detailed tables and plots (moved from main section)
    lines.extend(appendix_detailed_lines)
    
    # Op Count ↔ Gas Used regression (verify marginal property)
    lines.append("### Op Count ↔ Gas Used (Marginal Property Check)")
    lines.append("")
    lines.append("*High R² (≥ 0.99) indicates gas scales linearly with op count, confirming the marginal property.*")
    lines.append("")
    lines.append("| Opcode | Gas/Op | Intercept | R² | Status |")
    lines.append("|--------|--------|-----------|-----|--------|")
    
    marginal_results = []
    for opcode in sorted(df["opcode"].unique()):
        opcode_df = df[df["opcode"] == opcode]
        reg = perform_regression(
            opcode_df["op_count"].values.astype(float),
            opcode_df["gas_used"].values.astype(float)
        )
        if reg:
            marginal_results.append({
                "opcode": opcode,
                "slope": reg["slope"],
                "intercept": reg["intercept"],
                "r_squared": reg["r_squared"],
                "n_points": reg["n_points"],
            })
    
    # Sort by R² descending
    marginal_results.sort(key=lambda x: x["r_squared"], reverse=True)
    
    for result in marginal_results:
        r2 = result["r_squared"]
        if r2 >= 0.99:
            status = "✅ Good"
        elif r2 >= 0.95:
            status = "⚠️ OK"
        else:
            status = "❌ Check"
        
        lines.append(
            f"| {result['opcode']} | {format_number(result['slope'])} | "
            f"{format_number(result['intercept'])} | {format_r2(r2)} | {status} |"
        )
    lines.append("")
    
    if has_zkcycles:
        lines.append("### Op Count ↔ ZK Cycles")
        lines.append("")
        lines.append("| Opcode | Cycles/Op | R² |")
        lines.append("|--------|-----------|-----|")
        for opcode in sorted(df["opcode"].unique()):
            opcode_df = df[df["opcode"] == opcode]
            reg = perform_regression(
                opcode_df["op_count"].values.astype(float),
                opcode_df["zk_cycles"].values.astype(float)
            )
            if reg:
                lines.append(f"| {opcode} | {format_number(reg['slope'])} | {format_r2(reg['r_squared'])} |")
        lines.append("")
    
    if has_proving:
        lines.append("### Op Count ↔ Proving Time")
        lines.append("")
        lines.append("| Opcode | Time/Op (s) | R² |")
        lines.append("|--------|-------------|-----|")
        for opcode in sorted(df["opcode"].unique()):
            opcode_df = df[df["opcode"] == opcode]
            if "proving_time_s" not in opcode_df.columns:
                continue
            reg = perform_regression(
                opcode_df["op_count"].values.astype(float),
                opcode_df["proving_time_s"].values.astype(float)
            )
            if reg:
                lines.append(f"| {opcode} | {reg['slope']:.2e} | {format_r2(reg['r_squared'])} |")
        lines.append("")
    
    # Opcodes/Precompiles Summary (in appendix)
    lines.append("### Opcodes/Precompiles Summary")
    lines.append("")
    lines.append("| Opcode | N | Min Op Count | Max Op Count | Min Gas | Max Gas | Gas/Op |")
    lines.append("|--------|---|--------------|--------------|---------|---------|--------|")
    
    for _, row in regression_df.sort_values("opcode").iterrows():
        gas_per_op = format_number(row.get("gas_per_op", 0))
        lines.append(
            f"| {row['opcode']} | {row['n_points']} | {row['min_op_count']} | "
            f"{row['max_op_count']} | {format_number(row.get('min_gas', 0))} | "
            f"{format_number(row.get('max_gas', 0))} | {gas_per_op} |"
        )
    lines.append("")
    
    # Max ZK Cycles Bar Chart (colored by R²)
    if has_zkcycles:
        lines.append(f"### {prover_name} Max ZK Cycles by Opcode (colored by R²)")
        lines.append("")
        lines.append("Bar length = Max ZK Cycles, color = Time/Gas R² (green=high, red=low)")
        lines.append("")
        
        # Get max_zkcycles and R² for each opcode
        chart_data = []
        for _, row in regression_df.iterrows():
            max_cycles = row.get("max_zkcycles", 0)
            r2 = row.get("gas_proving_r2", 0)
            if pd.notna(max_cycles) and max_cycles > 0:
                chart_data.append({
                    "opcode": row["opcode"],
                    "max_cycles": max_cycles,
                    "r2": r2 if pd.notna(r2) else 0
                })
        
        # Sort by max_cycles descending
        chart_data.sort(key=lambda x: x["max_cycles"], reverse=True)
        
        if chart_data:
            max_value = max(d["max_cycles"] for d in chart_data)
            bar_height = 20
            chart_height = len(chart_data) * (bar_height + 4) + 40
            label_width = 180
            bar_area_width = 600
            value_width = 200
            total_width = label_width + bar_area_width + value_width
            
            # Generate SVG as a single line to avoid markdown parsing issues
            svg_parts = []
            svg_parts.append(f'<div style="overflow-x: auto;"><svg width="{total_width}" height="{chart_height}" xmlns="http://www.w3.org/2000/svg">')
            svg_parts.append(f'<style>.bar-label {{ font-family: monospace; font-size: 12px; fill: #333; }} .bar-value {{ font-family: monospace; font-size: 11px; fill: #666; }} .chart-title {{ font-family: sans-serif; font-size: 14px; font-weight: bold; fill: #333; }}</style>')
            svg_parts.append(f'<text x="{total_width/2}" y="20" text-anchor="middle" class="chart-title">{prover_name} Max ZK Cycles by Opcode (colored by Time/Gas R²)</text>')
            
            for i, data in enumerate(chart_data):
                y = 35 + i * (bar_height + 4)
                bar_width = (data["max_cycles"] / max_value) * bar_area_width if max_value > 0 else 0
                r2 = data["r2"]
                
                # Color based on R² (green for high, yellow for medium, red for low)
                if r2 >= 0.99:
                    color = "#22c55e"  # green
                elif r2 >= 0.95:
                    color = "#84cc16"  # lime
                elif r2 >= 0.90:
                    color = "#eab308"  # yellow
                elif r2 >= 0.80:
                    color = "#f97316"  # orange
                else:
                    color = "#ef4444"  # red
                
                # Label (opcode name)
                svg_parts.append(f'<text x="{label_width - 5}" y="{y + bar_height - 5}" text-anchor="end" class="bar-label">{data["opcode"]}</text>')
                
                # Bar
                svg_parts.append(f'<rect x="{label_width}" y="{y}" width="{max(bar_width, 1)}" height="{bar_height}" fill="{color}" rx="2"/>')
                
                # Value label (max cycles and R²)
                cycles_str = f'{data["max_cycles"]:,.0f}'
                r2_str = f'R²={r2:.4f}' if r2 > 0 else 'R²=N/A'
                svg_parts.append(f'<text x="{label_width + bar_area_width + 5}" y="{y + bar_height - 5}" class="bar-value">{cycles_str} ({r2_str})</text>')
            
            svg_parts.append('</svg></div>')
            
            # Join without newlines to prevent markdown parsing issues
            lines.append("".join(svg_parts))
            lines.append("")
    
    return "\n".join(lines)


# =============================================================================
# Main
# =============================================================================

def main():
    parser = argparse.ArgumentParser(
        description="Generate ZK Gas Benchmark Report",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Directory Structure (NEW - 2026+):
  <base-dir>/
    execute/         # Execution results (run once, no samples)
    prove/           # Proving results (sampled)
      sample-1/
      sample-2/
      sample-N/

Usage:
  python3 generate_zk_gas_report.py \\
    --execution-input <base-dir>/execute \\
    --proving-input <base-dir>/prove \\
    --output reports/
        """,
    )
    parser.add_argument(
        "--proving-input",
        nargs="+",
        type=Path,
        default=[],
        help="Path to prove/ directory containing sample-*/ subdirs",
    )
    parser.add_argument(
        "--execution-input",
        nargs="+",
        type=Path,
        default=[],
        help="Path to execute/ directory (no samples)",
    )
    parser.add_argument(
        "--output",
        type=Path,
        required=True,
        help="Output directory for the report",
    )
    parser.add_argument(
        "--output-csv",
        type=Path,
        default=None,
        help="Output directory for CSV files (defaults to --output)",
    )
    parser.add_argument(
        "--inline-images",
        action="store_true",
        help="Inline images in markdown (base64). If not set, creates plots/ directory.",
    )
    
    args = parser.parse_args()
    
    if not args.proving_input and not args.execution_input:
        parser.error("At least one of --proving-input or --execution-input is required")
    
    # Determine mode
    if args.proving_input and args.execution_input:
        mode = "both"
    elif args.proving_input:
        mode = "proving"
    else:
        mode = "execution"
    
    # Collect all input directories
    all_inputs = list(args.proving_input) + list(args.execution_input)
    
    # Detect prover
    prover_name = detect_prover_from_path(all_inputs)
    prover_version = get_prover_version(all_inputs)
    
    # Load hardware info
    hardware_info = load_hardware_info(all_inputs)
    
    # Load data with multi-sample support
    print("Loading execution results...", file=sys.stderr)
    exec_df = pd.DataFrame()
    total_exec_samples = 0
    if args.execution_input:
        for input_dir in args.execution_input:
            df_part, n_samples = load_multi_sample_results(input_dir, "execution")
            if not df_part.empty:
                exec_df = pd.concat([exec_df, df_part], ignore_index=True)
                total_exec_samples = max(total_exec_samples, n_samples)
    print(f"  Loaded {len(exec_df)} execution records from {total_exec_samples} sample(s)", file=sys.stderr)
    
    print("Loading proving results...", file=sys.stderr)
    prove_df = pd.DataFrame()
    total_prove_samples = 0
    if args.proving_input:
        for input_dir in args.proving_input:
            df_part, n_samples = load_multi_sample_results(input_dir, "proving")
            if not df_part.empty:
                prove_df = pd.concat([prove_df, df_part], ignore_index=True)
                total_prove_samples = max(total_prove_samples, n_samples)
    print(f"  Loaded {len(prove_df)} proving records from {total_prove_samples} sample(s)", file=sys.stderr)
    
    # Merge
    print("Merging results...", file=sys.stderr)
    df = merge_execution_and_proving(exec_df, prove_df)
    print(f"  Merged: {len(df)} total records", file=sys.stderr)
    
    # Filter out original opcodes when fixed versions exist
    df = filter_duplicates_prefer_fixed(df)
    print(f"  After filtering fixed: {len(df)} records", file=sys.stderr)
    
    if df.empty:
        print("Error: No valid data loaded", file=sys.stderr)
        sys.exit(1)
    
    # Compute regressions with outlier removal enabled
    print("Computing regressions (with outlier detection)...", file=sys.stderr)
    regression_df = compute_regressions(df, remove_outliers=True, outlier_threshold=3.5)
    
    # Create output directory
    args.output.mkdir(parents=True, exist_ok=True)
    csv_dir = args.output_csv or args.output
    csv_dir.mkdir(parents=True, exist_ok=True)
    
    # Generate report
    print("Generating report...", file=sys.stderr)
    report = generate_report(
        df, regression_df, hardware_info,
        prover_name, prover_version, mode,
        args.output, args.inline_images,
    )
    
    # Generate filename
    now = datetime.now()
    timestamp_str = now.strftime('%Y%m%d-%H%M%S')
    base_filename = f"zk-gas-report-{prover_name}-{mode}-{timestamp_str}"
    
    # Save markdown report
    md_path = args.output / f"{base_filename}.md"
    with open(md_path, "w") as f:
        f.write(report)
    print(f"Report saved to {md_path}", file=sys.stderr)
    
    # Save HTML report
    report_title = f"ZK Gas Benchmark Report - {prover_name} ({mode})"
    html_content = markdown_to_html(report, report_title)
    html_path = args.output / f"{base_filename}.html"
    with open(html_path, "w") as f:
        f.write(html_content)
    print(f"HTML report saved to {html_path}", file=sys.stderr)
    
    # Save CSV
    csv_path = csv_dir / f"raw-data-{prover_name}-{mode}-{timestamp_str}.csv"
    df.to_csv(csv_path, index=False)
    print(f"Raw data saved to {csv_path}", file=sys.stderr)
    
    regression_csv_path = csv_dir / f"regression-{prover_name}-{mode}-{timestamp_str}.csv"
    regression_df.to_csv(regression_csv_path, index=False)
    print(f"Regression results saved to {regression_csv_path}", file=sys.stderr)


if __name__ == "__main__":
    main()

