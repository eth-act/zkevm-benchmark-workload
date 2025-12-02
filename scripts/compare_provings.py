#!/usr/bin/env python3
"""
Script to compare proving time metrics between baseline and optimized proving runs.
Compares proving_time_ms data and calculates speedups.

Usage:
    python3 compare_provings.py <baseline_folder> <optimized_folder>

Example:
    python3 compare_provings.py foo-baseline foo-optimized

The script will look for all subfolders with *.json files in both folders and compare:
- proving_time_ms (the primary metric for proving performance, displayed in seconds)

Output includes:
- Detailed speedup table for all files
- Statistical analysis with best/worst performers
- Key findings summary with proving time improvements highlighted
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

def extract_proving_time(metrics_data: Dict) -> float:
    """Extract proving_time_ms from metrics data and convert to seconds."""
    try:
        return float(metrics_data["proving"]["success"]["proving_time_ms"]) / 1000.0
    except KeyError:
        return 0.0

def calculate_speedups(baseline_metrics: Dict[str, Dict], 
                      optimized_metrics: Dict[str, Dict]) -> Tuple[Dict[str, float], List[str]]:
    """Calculate speedups for all common files."""
    speedups = {}
    
    # Find common files
    common_files = set(baseline_metrics.keys()) & set(optimized_metrics.keys())
    
    for filename in common_files:
        baseline_time = extract_proving_time(baseline_metrics[filename])
        optimized_time = extract_proving_time(optimized_metrics[filename])
        
        if baseline_time > 0 and optimized_time > 0:
            speedup = baseline_time / optimized_time
            speedups[filename] = speedup
    
    # Sort files by name
    sorted_files = sorted(speedups.keys())
    
    return speedups, sorted_files

def print_speedup_table(speedups: Dict[str, float], files: List[str]):
    """Print a formatted table of speedups."""
    if not speedups:
        print("No data to display")
        return
    
    # Print header - use wider column for file names to accommodate subfolder/filename
    header = "File".ljust(30) + "Proving Time Speedup".ljust(20) + "Baseline (ms)".ljust(15) + "Optimized (ms)".ljust(15) + "Time Saved (ms)".ljust(15)
    print(header)
    print("-" * len(header))
    
    # Get baseline and optimized metrics for displaying actual times
    # We'll need to pass these in or recalculate, for now just show speedup
    for filename in files:
        if filename in speedups:
            speedup = speedups[filename]
            speedup_str = f"{speedup:.2f}x"
            row = filename.ljust(30) + speedup_str.ljust(20)
            print(row)

def print_detailed_speedup_table(speedups: Dict[str, float], baseline_metrics: Dict[str, Dict], 
                                optimized_metrics: Dict[str, Dict], files: List[str]):
    """Print a detailed formatted table of speedups with actual times."""
    if not speedups:
        print("No data to display")
        return
    
    # Print header - increase column width for human-readable names
    header = "Test".ljust(60) + "Speedup".ljust(12) + "Baseline (s)".ljust(15) + "Optimized (s)".ljust(15) + "Time Saved (s)".ljust(15)
    print(header)
    print("-" * len(header))
    
    # Print data rows
    for filename in files:
        if filename in speedups:
            speedup = speedups[filename]
            baseline_time = extract_proving_time(baseline_metrics[filename])
            optimized_time = extract_proving_time(optimized_metrics[filename])
            time_saved = baseline_time - optimized_time
            
            # Format the test name for display
            display_name = format_test_name(filename)
            
            speedup_str = f"{speedup:.2f}x"
            baseline_str = f"{baseline_time:,.0f}"
            optimized_str = f"{optimized_time:,.0f}"
            saved_str = f"{time_saved:,.0f}"
            
            row = (display_name[:60].ljust(60) + speedup_str.ljust(12) + 
                   baseline_str.ljust(15) + optimized_str.ljust(15) + saved_str.ljust(15))
            print(row)

def analyze_speedups(speedups: Dict[str, float], baseline_metrics: Dict[str, Dict], 
                    optimized_metrics: Dict[str, Dict]):
    """Analyze and print top/bottom performers and averages."""
    if not speedups:
        print("No data to analyze")
        return
    
    print("\n" + "="*80)
    print("ANALYSIS")
    print("="*80)
    
    # Overall summary
    print("\nOVERALL SUMMARY:")
    
    speedup_values = list(speedups.values())
    file_speedups = [(filename, speedup) for filename, speedup in speedups.items()]
    file_speedups.sort(key=lambda x: x[1], reverse=True)
    
    avg_speedup = statistics.mean(speedup_values)
    improvement_pct = (avg_speedup - 1.0) * 100
    
    # Calculate total time savings
    total_baseline_time = sum(extract_proving_time(baseline_metrics[f]) for f in speedups.keys())
    total_optimized_time = sum(extract_proving_time(optimized_metrics[f]) for f in speedups.keys())
    total_time_saved = total_baseline_time - total_optimized_time
    
    print(f"\nPROVING TIME PERFORMANCE:")
    # Format percentage to avoid negative zeros
    if abs(improvement_pct) < 0.05:  # Less than 0.05% is essentially zero
        pct_str = "(+0.0%)"
    else:
        pct_str = f"({improvement_pct:+.1f}%)"
    print(f"  Average speedup: {avg_speedup:.2f}x {pct_str}")
    print(f"  Min speedup: {min(speedup_values):.2f}x")
    print(f"  Max speedup: {max(speedup_values):.2f}x")
    print(f"  Total baseline time: {total_baseline_time:,.0f} seconds")
    print(f"  Total optimized time: {total_optimized_time:,.0f} seconds")
    print(f"  Total time saved: {total_time_saved:,.0f} seconds")
    
    # Top 3 best
    print("  Top 3 best speedups:")
    for i, (filename, speedup) in enumerate(file_speedups[:3]):
        baseline_time = extract_proving_time(baseline_metrics[filename])
        optimized_time = extract_proving_time(optimized_metrics[filename])
        time_saved = baseline_time - optimized_time
        display_name = format_test_name(filename)
        print(f"    {i+1}. {display_name}: {speedup:.2f}x (saved {time_saved:,.0f} s)")
    
    # Top 3 worst (if we have at least 3 entries)
    if len(file_speedups) >= 3:
        print("  Top 3 worst speedups:")
        for i, (filename, speedup) in enumerate(file_speedups[-3:]):
            baseline_time = extract_proving_time(baseline_metrics[filename])
            optimized_time = extract_proving_time(optimized_metrics[filename])
            time_diff = baseline_time - optimized_time
            display_name = format_test_name(filename)
            if time_diff >= 0:
                time_str = f"saved {time_diff:,.0f} s"
            else:
                time_str = f"lost {abs(time_diff):,.0f} s"
            print(f"    {i+1}. {display_name}: {speedup:.2f}x ({time_str})")

def main():
    """Main function."""
    if len(sys.argv) != 3:
        print("Usage: python3 compare_provings.py <baseline_folder> <optimized_folder>")
        print("\nExample:")
        print("  python3 compare_provings.py foo-baseline foo-optimized")
        print("  python3 compare_provings.py /path/to/baseline /path/to/optimized")
        sys.exit(1)
    
    baseline_folder = sys.argv[1]
    optimized_folder = sys.argv[2]
    
    # Convert to absolute paths if relative paths are provided
    if not os.path.isabs(baseline_folder):
        baseline_folder = os.path.abspath(baseline_folder)
    if not os.path.isabs(optimized_folder):
        optimized_folder = os.path.abspath(optimized_folder)
    
    print(f"Loading baseline metrics from: {baseline_folder}")
    baseline_metrics = load_metrics(baseline_folder)
    print(f"Loaded {len(baseline_metrics)} baseline files")
    
    print(f"\nLoading optimized metrics from: {optimized_folder}")
    optimized_metrics = load_metrics(optimized_folder)
    print(f"Loaded {len(optimized_metrics)} optimized files")
    
    print("\nCalculating speedups...")
    speedups, files = calculate_speedups(baseline_metrics, optimized_metrics)
    print(f"Found {len(speedups)} common files")
    
    if not speedups:
        print("No common files found or no valid data")
        return
    
    print("\n" + "="*80)
    print("PROVING TIME SPEEDUP COMPARISON TABLE")
    print("="*80)
    print_detailed_speedup_table(speedups, baseline_metrics, optimized_metrics, files)
    
    analyze_speedups(speedups, baseline_metrics, optimized_metrics)
    
    # Summary of key findings
    print("\n" + "="*80)
    print("KEY FINDINGS")
    print("="*80)
    
    # Calculate overall metrics
    speedup_values = list(speedups.values())
    avg_speedup = statistics.mean(speedup_values)
    improvement_pct = (avg_speedup - 1.0) * 100
    
    # Calculate total time savings
    total_baseline_time = sum(extract_proving_time(baseline_metrics[f]) for f in speedups.keys())
    total_optimized_time = sum(extract_proving_time(optimized_metrics[f]) for f in speedups.keys())
    total_time_saved = total_baseline_time - total_optimized_time
    
    print(f"\nTotal files analyzed: {len(speedups)}")
    
    # Only show IMPROVEMENT/REGRESSION if change is > 0.5%
    if abs(improvement_pct) > 0.5:
        status = "IMPROVEMENT" if avg_speedup > 1.0 else "REGRESSION"
    else:
        status = "NO CHANGE"
    
    # Format percentage to avoid negative zeros
    if abs(improvement_pct) < 0.05:  # Less than 0.05% is essentially zero
        pct_str = "(+0.0%)"
    else:
        pct_str = f"({improvement_pct:+.1f}%)"
        
    print(f"\n🎯 OVERALL PROVING PERFORMANCE: {avg_speedup:.2f}x {pct_str} - {status}")
    print(f"   Total time saved: {total_time_saved:,.0f} seconds")
    
    if avg_speedup > 1.0:
        efficiency_gain = (1 - 1/avg_speedup) * 100
        print(f"   Efficiency gain: {efficiency_gain:.1f}% reduction in proving time")

if __name__ == "__main__":
    main()
