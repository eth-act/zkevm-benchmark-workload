#!/usr/bin/env python3
"""
Quick CLI viewer for zkVM benchmark metrics.

Provides a command-line interface to view benchmark results without generating HTML.
"""

import json
import sys
from pathlib import Path
from typing import Dict, List, Any
import argparse

def format_cycles(cycles: int) -> str:
    """Format cycle count for human readability."""
    if cycles >= 1_000_000_000:
        return f"{cycles / 1_000_000_000:.2f}B"
    elif cycles >= 1_000_000:
        return f"{cycles / 1_000_000:.2f}M"
    elif cycles >= 1_000:
        return f"{cycles / 1_000:.2f}K"
    else:
        return str(cycles)

def load_metrics_data(metrics_dir: Path) -> Dict[str, List[Dict[str, Any]]]:
    """Load all metric data from JSON files organized by zkVM."""
    metrics_data = {}
    
    for zkvm_dir in metrics_dir.iterdir():
        if zkvm_dir.is_dir():
            zkvm_name = zkvm_dir.name
            metrics_data[zkvm_name] = []
            
            # Find all JSON files recursively
            for json_file in zkvm_dir.rglob("*.json"):
                try:
                    with open(json_file, 'r') as f:
                        data = json.load(f)
                        if isinstance(data, list):
                            for item in data:
                                item['file_path'] = str(json_file.relative_to(zkvm_dir))
                                metrics_data[zkvm_name].append(item)
                        elif isinstance(data, dict):
                            data['file_path'] = str(json_file.relative_to(zkvm_dir))
                            metrics_data[zkvm_name].append(data)
                except Exception as e:
                    print(f"Error reading {json_file}: {e}", file=sys.stderr)
    
    return metrics_data

def print_summary(metrics_data: Dict[str, List[Dict[str, Any]]]):
    """Print a summary table to the console."""
    print("\n" + "="*80)
    print("zkEVM BENCHMARK SUMMARY")
    print("="*80)
    
    header = f"{'zkVM':<12} {'Tests':<8} {'Avg Cycles':<12} {'Min Cycles':<12} {'Max Cycles':<12} {'Val Cycles':<12}"
    print(header)
    print("-" * len(header))
    
    for zkvm_name, tests in metrics_data.items():
        if not tests:
            continue
            
        total_cycles = [test.get('total_num_cycles', 0) for test in tests]
        validation_cycles = [
            test.get('region_cycles', {}).get('validation', 0) 
            for test in tests
        ]
        
        total_tests = len(tests)
        avg_cycles = sum(total_cycles) / total_tests if total_tests > 0 else 0
        min_cycles = min(total_cycles) if total_cycles else 0
        max_cycles = max(total_cycles) if total_cycles else 0
        total_validation = sum(validation_cycles)
        
        print(f"{zkvm_name:<12} {total_tests:<8} {format_cycles(int(avg_cycles)):<12} "
              f"{format_cycles(min_cycles):<12} {format_cycles(max_cycles):<12} "
              f"{format_cycles(total_validation):<12}")

def print_detailed(metrics_data: Dict[str, List[Dict[str, Any]]], zkvm: str = None, top: int = 10):
    """Print detailed results for specific zkVM or all."""
    
    if zkvm:
        if zkvm not in metrics_data:
            print(f"Error: zkVM '{zkvm}' not found. Available: {', '.join(metrics_data.keys())}")
            return
        selected_data = {zkvm: metrics_data[zkvm]}
    else:
        selected_data = metrics_data
    
    for zkvm_name, tests in selected_data.items():
        if not tests:
            continue
            
        print(f"\n" + "="*80)
        print(f"{zkvm_name.upper()} - TOP {top} MOST EXPENSIVE TESTS")
        print("="*80)
        
        # Sort tests by total cycles (descending)
        sorted_tests = sorted(tests, key=lambda x: x.get('total_num_cycles', 0), reverse=True)
        
        header = f"{'Test Name':<50} {'Total':<10} {'Valid':<10} {'Input':<10} {'Witness':<10}"
        print(header)
        print("-" * len(header))
        
        for i, test in enumerate(sorted_tests[:top]):
            name = test.get('name', 'Unknown')
            # Truncate long test names
            if len(name) > 47:
                name = name[:44] + "..."
                
            total_cycles = test.get('total_num_cycles', 0)
            region_cycles = test.get('region_cycles', {})
            validation = region_cycles.get('validation', 0)
            read_input = region_cycles.get('read_input', 0)
            verify_witness = region_cycles.get('verify-witness', 0)
            
            print(f"{name:<50} {format_cycles(total_cycles):<10} "
                  f"{format_cycles(validation):<10} {format_cycles(read_input):<10} "
                  f"{format_cycles(verify_witness):<10}")

def main():
    parser = argparse.ArgumentParser(description="View zkVM benchmark metrics in the terminal")
    parser.add_argument(
        "--metrics-dir", 
        type=Path, 
        default="zkevm-metrics",
        help="Directory containing metrics data (default: zkevm-metrics)"
    )
    parser.add_argument(
        "--zkvm",
        type=str,
        help="Show detailed results for specific zkVM only"
    )
    parser.add_argument(
        "--top",
        type=int,
        default=10,
        help="Number of top results to show in detailed view (default: 10)"
    )
    parser.add_argument(
        "--detailed",
        action="store_true",
        help="Show detailed results instead of just summary"
    )
    
    args = parser.parse_args()
    
    if not args.metrics_dir.exists():
        print(f"Error: Metrics directory {args.metrics_dir} does not exist", file=sys.stderr)
        sys.exit(1)
    
    metrics_data = load_metrics_data(args.metrics_dir)
    
    total_tests = sum(len(tests) for tests in metrics_data.values())
    if total_tests == 0:
        print("No metrics data found!", file=sys.stderr)
        sys.exit(1)
    
    if args.detailed:
        print_detailed(metrics_data, args.zkvm, args.top)
    else:
        print_summary(metrics_data)
        if not args.zkvm:
            print(f"\nTip: Use --detailed to see individual test results")
            print(f"     Use --zkvm <name> to filter by specific zkVM")

if __name__ == "__main__":
    main() 