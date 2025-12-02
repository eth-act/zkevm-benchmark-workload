#!/usr/bin/env python3
"""
Script to compare optimization metrics between unoptimized and optimized zkevm runs.
Compares region_cycles data and calculates speedups.

Usage:
    python3 compare_executions.py <baseline_folder> <optimized_folder>

Example:
    python3 compare_executions.py baseline-zkevm-metrics optimized-zkevm-metrics

The script will look for all subfolders with *.json files in both folders and compare:
- region_cycles data (verify_witness, post_state_compute, validation, etc.)
- total_num_cycles (added as the most general metric)

Output includes:
- Detailed speedup table for all files
- Statistical analysis with best/worst performers
- Key findings summary with total_num_cycles highlighted
"""

import json
import os
import sys
from pathlib import Path
from typing import Dict, List, Tuple
import statistics

# Import the test name parser for human-readable display names
try:
    from test_name_parser import get_display_name
    PARSER_AVAILABLE = True
except ImportError:
    PARSER_AVAILABLE = False
    def get_display_name(name: str) -> str:
        return name

def load_metrics(folder_path: str) -> Dict[str, Dict]:
    """Load all metric files from a folder, searching all subfolders."""
    metrics = {}
    folder = Path(folder_path)
    
    if not folder.exists():
        print(f"Warning: {folder} does not exist")
        return metrics
    
    # Find all subfolders that contain JSON files
    subfolders_with_json = []
    for subfolder in folder.iterdir():
        if subfolder.is_dir():
            json_files = list(subfolder.glob("*.json"))
            if json_files:
                subfolders_with_json.append(subfolder)
    
    if not subfolders_with_json:
        print(f"Warning: No subfolders with JSON files found in {folder}")
        return metrics
    
    print(f"Found subfolders with metrics: {[sf.name for sf in subfolders_with_json]}")
    
    # Load files from all subfolders
    for subfolder in subfolders_with_json:
        for file_path in subfolder.glob("*.json"):
            try:
                with open(file_path, 'r') as f:
                    data = json.load(f)
                    # Create a unique key combining subfolder and filename
                    filename = file_path.stem
                    subfolder_name = subfolder.name
                    unique_key = f"{subfolder_name}/{filename}"
                    metrics[unique_key] = data
            except (json.JSONDecodeError, FileNotFoundError) as e:
                print(f"Error loading {file_path}: {e}")
    
    return metrics

def format_test_name(filename: str) -> str:
    """Format test name for display using human-readable format.
    
    Args:
        filename: Can be either 'test_name' or 'subfolder/test_name'
    
    Returns:
        Formatted name, preserving subfolder prefix if present
    """
    if '/' in filename:
        subfolder, test_name = filename.rsplit('/', 1)
        try:
            formatted_name = get_display_name(test_name)
            return f"{subfolder}/{formatted_name}"
        except Exception:
            return filename
    else:
        try:
            return get_display_name(filename)
        except Exception:
            return filename

def extract_region_cycles(metrics_data: Dict) -> Dict[str, int]:
    """Extract region_cycles from metrics data and add total_num_cycles."""
    try:
        cycles = metrics_data["execution"]["success"]["region_cycles"].copy()
        # Add total_num_cycles as the last entry (most general metric)
        cycles["total_num_cycles"] = metrics_data["execution"]["success"]["total_num_cycles"]
        return cycles
    except KeyError:
        return {}

def calculate_speedups(unoptimized_metrics: Dict[str, Dict], 
                      optimized_metrics: Dict[str, Dict]) -> Tuple[Dict[str, Dict[str, float]], List[str]]:
    """Calculate speedups for all common files and region types."""
    speedups = {}
    all_regions = set()
    
    # Find common files
    common_files = set(unoptimized_metrics.keys()) & set(optimized_metrics.keys())
    
    # First pass: collect all possible regions from both datasets
    for filename in common_files:
        unopt_cycles = extract_region_cycles(unoptimized_metrics[filename])
        opt_cycles = extract_region_cycles(optimized_metrics[filename])
        
        if unopt_cycles:
            all_regions.update(unopt_cycles.keys())
        if opt_cycles:
            all_regions.update(opt_cycles.keys())
    
    # Second pass: calculate speedups for all regions
    for filename in common_files:
        unopt_cycles = extract_region_cycles(unoptimized_metrics[filename])
        opt_cycles = extract_region_cycles(optimized_metrics[filename])
        
        if not unopt_cycles and not opt_cycles:
            continue
            
        file_speedups = {}
        for region in all_regions:
            # Get cycles for this region, defaulting to 0 if missing
            unopt_value = unopt_cycles.get(region, 0)
            opt_value = opt_cycles.get(region, 0)
            
            # Calculate speedup
            if opt_value > 0:
                speedup = unopt_value / opt_value
                file_speedups[region] = speedup
            elif unopt_value > 0 and opt_value == 0:
                # Region exists in baseline but not optimized (infinite speedup)
                # This could indicate the region was eliminated/optimized away
                file_speedups[region] = float('inf')
            elif unopt_value == 0 and opt_value > 0:
                # Region doesn't exist in baseline but exists in optimized (0x speedup)
                # This indicates a new region was added
                file_speedups[region] = 0.0
            # If both are 0, skip this region for this file
        
        if file_speedups:
            speedups[filename] = file_speedups
    
    # Sort regions to put total_num_cycles last
    sorted_regions = sorted(all_regions)
    if "total_num_cycles" in sorted_regions:
        sorted_regions.remove("total_num_cycles")
        sorted_regions.append("total_num_cycles")
    
    return speedups, sorted_regions

def abbreviate_region_name(region: str) -> str:
    """Abbreviate long region names for better table formatting."""
    abbreviations = {
        "commit_public_inputs": "commit_pub",
        "public_inputs_preparation": "pub_prep",
        "public_keys_validation": "pubkey_val",
        "block_execution": "block_exec",
        "verify_witness": "verify_wit",
        "post_state_compute": "post_state",
        "total_num_cycles": "total_cycles",
        "validation": "validation",
        "read_input": "read_input"
    }
    return abbreviations.get(region, region[:12])  # Fallback to first 12 chars

def print_speedup_table(speedups: Dict[str, Dict[str, float]], regions: List[str]):
    """Print a formatted table of speedups."""
    if not speedups:
        print("No data to display")
        return
    
    # Sort files by name
    sorted_files = sorted(speedups.keys())
    
    # Format all test names and calculate optimal column width
    formatted_names = {filename: format_test_name(filename) for filename in sorted_files}
    max_display_length = max(len(name) for name in formatted_names.values())
    file_column_width = max(max_display_length + 2, 40)  # At least 40 for human-readable names
    
    # Print header - use dynamic column width for file names and abbreviated region names
    header = "Test".ljust(file_column_width)
    for region in regions:
        abbreviated = abbreviate_region_name(region)
        header += abbreviated.ljust(14)  # Reduced from 18 to 14 for better fit
    print(header)
    print("-" * len(header))
    
    # Print data rows
    for filename in sorted_files:
        display_name = formatted_names[filename]
        row = display_name[:file_column_width].ljust(file_column_width)
        for region in regions:
            if region in speedups[filename]:
                speedup = speedups[filename][region]
                if speedup == float('inf'):
                    speedup_str = "∞x"  # Infinite speedup (region eliminated)
                elif speedup == 0.0:
                    speedup_str = "0.00x"  # Zero speedup (new region added)
                else:
                    speedup_str = f"{speedup:.2f}x"
            else:
                speedup_str = "N/A"
            row += speedup_str.ljust(14)  # Reduced from 18 to 14 for better fit
        print(row)

def analyze_speedups(speedups: Dict[str, Dict[str, float]], regions: List[str]):
    """Analyze and print top/bottom performers and averages."""
    if not speedups:
        print("No data to analyze")
        return
    
    print("\n" + "="*80)
    print("ANALYSIS")
    print("="*80)
    
    # Overall summary
    print("\nOVERALL SUMMARY:")
    print("Note: Speedup < 1.0x means the optimized version is slower (regression)")
    print("      Speedup > 1.0x means the optimized version is faster (improvement)")
    
    for region in regions:
        region_speedups = []
        file_speedups = []
        
        for filename, file_data in speedups.items():
            if region in file_data:
                speedup = file_data[region]
                # Filter out infinite speedups for statistical analysis, but include zero speedups
                if speedup != float('inf'):
                    region_speedups.append(speedup)
                    file_speedups.append((filename, speedup))
        
        if not region_speedups:
            continue
        
        # Sort by speedup
        file_speedups.sort(key=lambda x: x[1], reverse=True)
        
        avg_speedup = statistics.mean(region_speedups)
        improvement_pct = (avg_speedup - 1.0) * 100
        
        print(f"\n{region.upper().replace('_', ' ')}:")
        # Format percentage to avoid negative zeros
        if abs(improvement_pct) < 0.05:  # Less than 0.05% is essentially zero
            pct_str = "(+0.0%)"
        else:
            pct_str = f"({improvement_pct:+.1f}%)"
        print(f"  Average speedup: {avg_speedup:.2f}x {pct_str}")
        print(f"  Min speedup: {min(region_speedups):.2f}x")
        print(f"  Max speedup: {max(region_speedups):.2f}x")
        
        # Top 3 best
        print("  Top 3 best speedups:")
        for i, (filename, speedup) in enumerate(file_speedups[:3]):
            display_name = format_test_name(filename)
            print(f"    {i+1}. {display_name}: {speedup:.2f}x")
        
        # Top 3 worst (if we have at least 3 entries)
        if len(file_speedups) >= 3:
            print("  Top 3 worst speedups:")
            for i, (filename, speedup) in enumerate(file_speedups[-3:]):
                display_name = format_test_name(filename)
                print(f"    {i+1}. {display_name}: {speedup:.2f}x")

def main():
    """Main function."""
    if len(sys.argv) != 3:
        print("Usage: python3 compare_executions.py <baseline_folder> <optimized_folder>")
        print("\nExample:")
        print("  python3 compare_executions.py zkevm-metrics local-optimized-zkevm-metrics")
        print("  python3 compare_executions.py /path/to/baseline /path/to/optimized")
        sys.exit(1)
    
    baseline_folder = sys.argv[1]
    optimized_folder = sys.argv[2]
    
    # Convert to absolute paths if relative paths are provided
    if not os.path.isabs(baseline_folder):
        baseline_folder = os.path.abspath(baseline_folder)
    if not os.path.isabs(optimized_folder):
        optimized_folder = os.path.abspath(optimized_folder)
    
    print(f"Loading baseline metrics from: {baseline_folder}")
    unoptimized_metrics = load_metrics(baseline_folder)
    print(f"Loaded {len(unoptimized_metrics)} baseline files")
    
    print(f"\nLoading optimized metrics from: {optimized_folder}")
    optimized_metrics = load_metrics(optimized_folder)
    print(f"Loaded {len(optimized_metrics)} optimized files")
    
    print("\nCalculating speedups...")
    speedups, regions = calculate_speedups(unoptimized_metrics, optimized_metrics)
    print(f"Found {len(speedups)} common files with {len(regions)} regions")
    
    if not speedups:
        print("No common files found or no valid data")
        return
    
    print(f"\nRegions found: {', '.join(regions)}")
    print("\n" + "="*80)
    print("SPEEDUP COMPARISON TABLE")
    print("="*80)
    print_speedup_table(speedups, regions)
    
    analyze_speedups(speedups, regions)
    
    # Summary of key findings
    print("\n" + "="*80)
    print("KEY FINDINGS")
    print("="*80)
    
    # Calculate overall metrics
    region_improvements = {}
    for region in regions:
        region_speedups = []
        for filename, file_data in speedups.items():
            if region in file_data:
                speedup = file_data[region]
                # Filter out infinite speedups for overall analysis, but include zero speedups
                if speedup != float('inf'):
                    region_speedups.append(speedup)
        if region_speedups:
            avg_speedup = statistics.mean(region_speedups)
            region_improvements[region] = avg_speedup
    
    # Sort regions by improvement, but keep total_num_cycles for special handling
    total_cycles_speedup = region_improvements.pop("total_num_cycles", None)
    sorted_regions = sorted(region_improvements.items(), key=lambda x: x[1], reverse=True)
    
    print("\nRegions ranked by average speedup:")
    for i, (region, speedup) in enumerate(sorted_regions):
        improvement_pct = (speedup - 1.0) * 100
        
        # Only show IMPROVEMENT/REGRESSION if change is > 0.5%
        if abs(improvement_pct) > 0.5:
            status = "IMPROVEMENT" if speedup > 1.0 else "REGRESSION"
        else:
            status = "NO CHANGE"
        
        # Format percentage to avoid negative zeros
        if abs(improvement_pct) < 0.05:  # Less than 0.05% is essentially zero
            pct_str = "(+0.0%)"
        else:
            pct_str = f"({improvement_pct:+.1f}%)"
            
        print(f"  {i+1}. {region.replace('_', ' ').title()}: {speedup:.2f}x {pct_str} - {status}")
    
    print(f"\nTotal files analyzed: {len(speedups)}")
    
    # Highlight total_num_cycles at the bottom as most important
    if total_cycles_speedup:
        improvement_pct = (total_cycles_speedup - 1.0) * 100
        
        # Only show IMPROVEMENT/REGRESSION if change is > 0.5%
        if abs(improvement_pct) > 0.5:
            status = "IMPROVEMENT" if total_cycles_speedup > 1.0 else "REGRESSION"
        else:
            status = "NO CHANGE"
        
        # Format percentage to avoid negative zeros
        if abs(improvement_pct) < 0.05:  # Less than 0.05% is essentially zero
            pct_str = "(+0.0%)"
        else:
            pct_str = f"({improvement_pct:+.1f}%)"
            
        print(f"\n🎯 OVERALL PERFORMANCE (Total Num Cycles): {total_cycles_speedup:.2f}x {pct_str} - {status}")

if __name__ == "__main__":
    main()
