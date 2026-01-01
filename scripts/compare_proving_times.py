#!/usr/bin/env python3
"""
Script to compare proving times across different zkVMs from benchmark JSON files.
Scans zkevm-metrics-{zkvm}-{gas}-{gpus} directories for benchmark data.
Gas can be a rational number (e.g., 0.1M, 0.2M, 1M).
"""

import argparse
import json
import os
import re
import sys
from pathlib import Path
from typing import Dict, List, Tuple, Optional
from collections import defaultdict

# Import test name parser for display names
try:
    from test_name_parser import get_display_name
    PARSER_AVAILABLE = True
except ImportError:
    PARSER_AVAILABLE = False
    print("Warning: test_name_parser not available. Using raw benchmark names.", file=sys.stderr)


def parse_directory_name(dirname: str) -> Tuple[Optional[str], Optional[str], Optional[str]]:
    """
    Parse the directory name to extract zkvm name, gas category, and GPU count.
    
    Args:
        dirname: Directory name in format zkevm-metrics-{zkvm}-{gas}-{gpus}
                 Gas can be a rational number (e.g., 0.1M, 0.2M, 1M)
        
    Returns:
        Tuple of (zkvm_name, gas_category, gpu_count) or (None, None, None) if parsing fails
    """
    # Pattern: zkevm-metrics-{zkvm}-{gas}-{gpus}
    # Example: zkevm-metrics-sp1-1M-1, zkevm-metrics-risc0-1M-4
    # Gas can be a rational number like 0.1M, 0.2M, etc.
    pattern = r'^zkevm-metrics-([a-z0-9]+)-([0-9.]+[A-Za-z]+)-(\d+)$'
    match = re.match(pattern, dirname)
    
    if match:
        zkvm = match.group(1)
        gas_category = match.group(2)
        gpu_count = match.group(3)
        return zkvm, gas_category, gpu_count
    
    return None, None, None


def extract_benchmark_name(test_name: str, use_display_name: bool = True) -> str:
    """
    Extract a human-readable benchmark name from the test name.
    
    Args:
        test_name: Full test name from JSON
        use_display_name: Whether to use display name formatting
        
    Returns:
        Formatted benchmark name
    """
    # Use display name parser if available
    if use_display_name and PARSER_AVAILABLE:
        try:
            return get_display_name(test_name)
        except Exception as e:
            print(f"Warning: Failed to get display name for '{test_name}': {e}", file=sys.stderr)
            # Fall through to basic formatting
    
    # Fallback: Basic formatting
    # Remove test file prefix
    if '::' in test_name:
        test_name = test_name.split('::', 1)[1]
    
    # Remove test_ prefix
    test_name = re.sub(r'^test_', '', test_name)
    
    return test_name


def parse_json_file(file_path: Path, use_display_name: bool = True) -> Optional[Tuple[str, float]]:
    """
    Parse a benchmark JSON file to extract benchmark name and proving time.
    
    Args:
        file_path: Path to the JSON file
        use_display_name: Whether to use display name formatting
        
    Returns:
        Tuple of (benchmark_name, proving_time_seconds) or None if parsing fails
    """
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        # Extract benchmark name
        benchmark_name = data.get('name', file_path.stem)
        benchmark_name = extract_benchmark_name(benchmark_name, use_display_name)
        
        # Extract proving time (convert from ms to seconds)
        proving_data = data.get('proving', {})
        success_data = proving_data.get('success', {})
        proving_time_ms = success_data.get('proving_time_ms')
        
        if proving_time_ms is not None:
            proving_time_s = proving_time_ms / 1000.0
            return benchmark_name, proving_time_s
        
        return None
    
    except Exception as e:
        print(f"Error parsing {file_path}: {e}", file=sys.stderr)
        return None


def find_benchmark_directories(root_dir: Path) -> List[Tuple[Path, str, str, str]]:
    """
    Find all zkevm-metrics-* directories (excluding zkevm-metrics-results*).
    
    Args:
        root_dir: Root directory to search
        
    Returns:
        List of tuples (dir_path, zkvm, gas_category, gpu_count)
    """
    results = []
    
    if not root_dir.exists():
        return results
    
    for dir_path in root_dir.iterdir():
        if dir_path.is_dir() and dir_path.name.startswith('zkevm-metrics-'):
            # Skip zkevm-metrics-results* directories
            if dir_path.name.startswith('zkevm-metrics-results'):
                continue
            
            zkvm, gas_category, gpu_count = parse_directory_name(dir_path.name)
            if zkvm:
                results.append((dir_path, zkvm, gas_category, gpu_count))
    
    return results


def collect_benchmark_data(benchmark_dirs: List[Tuple[Path, str, str, str]], use_display_name: bool = True) -> Dict:
    """
    Collect benchmark data from all directories.
    
    Args:
        benchmark_dirs: List of (dir_path, zkvm, gas_category, gpu_count) tuples
        use_display_name: Whether to use display name formatting for readable names
        
    Returns:
        Nested dictionary: {(gas_category, gpu_count): {zkvm: {benchmark: time}}}
    """
    organized = defaultdict(lambda: defaultdict(dict))
    
    for dir_path, zkvm, gas_category, gpu_count in benchmark_dirs:
        # Find all JSON files in the directory
        json_files = list(dir_path.rglob('*.json'))
        
        for json_file in json_files:
            # Skip hardware.json
            if json_file.name == 'hardware.json':
                continue
            
            result = parse_json_file(json_file, use_display_name)
            if result:
                benchmark_name, proving_time = result
                key = (gas_category, gpu_count)
                organized[key][zkvm][benchmark_name] = proving_time
    
    return organized


def get_all_benchmarks(zkvm_results: Dict[str, Dict[str, float]]) -> List[str]:
    """
    Get all unique benchmark names from all zkVMs.
    
    Args:
        zkvm_results: Dictionary mapping zkvm to benchmark results
        
    Returns:
        Sorted list of unique benchmark names
    """
    all_benchmarks = set()
    for zkvm, results in zkvm_results.items():
        all_benchmarks.update(results.keys())
    
    return sorted(all_benchmarks)


def format_time(time_seconds: float) -> str:
    """Format time in seconds with appropriate precision."""
    if time_seconds < 1:
        return f"{time_seconds:.3f}"
    elif time_seconds < 100:
        return f"{time_seconds:.2f}"
    else:
        return f"{time_seconds:.1f}"


def write_comparison_table_to_file(
    gas_category: str,
    gpu_count: str,
    zkvm_results: Dict,
    filter_zkvm: Optional[str] = None,
    filter_benchmark: Optional[str] = None
) -> str:
    """
    Write a comparison table for a specific gas category and GPU count to a string.
    
    Args:
        gas_category: Gas category (e.g., '1M')
        gpu_count: Number of GPUs
        zkvm_results: Results for all zkVMs
        filter_zkvm: Optional zkVM name to filter by
        filter_benchmark: Optional benchmark name pattern to filter by
        
    Returns:
        Formatted markdown table as string
    """
    lines = []
    
    # Filter zkVMs if requested
    if filter_zkvm:
        zkvm_results = {k: v for k, v in zkvm_results.items() if k == filter_zkvm}
    
    # Get all unique zkVMs
    zkvms = sorted(zkvm_results.keys())
    
    if not zkvms:
        return ""
    
    # Get all benchmarks
    all_benchmarks = get_all_benchmarks(zkvm_results)
    
    # Filter benchmarks if requested
    if filter_benchmark:
        all_benchmarks = [b for b in all_benchmarks if filter_benchmark.lower() in b.lower()]
    
    if not all_benchmarks:
        return ""
    
    # Add header
    lines.append(f"# zkVM Proving Time (in secs) Comparison\n")
    lines.append(f"**Gas Category:** {gas_category}\n")
    lines.append(f"**Number of GPUs:** {gpu_count}\n")
    lines.append("")
    
    # Create table header
    header = "| Benchmark | " + " | ".join(zkvms) + " |"
    separator = "|" + "|".join(["---"] * (len(zkvms) + 1)) + "|"
    
    lines.append(header)
    lines.append(separator)
    
    # Add each benchmark row
    for benchmark in all_benchmarks:
        row_parts = [benchmark]
        
        for zkvm in zkvms:
            if benchmark in zkvm_results[zkvm]:
                time_val = zkvm_results[zkvm][benchmark]
                row_parts.append(format_time(time_val))
            else:
                row_parts.append("N/A")
        
        row = "| " + " | ".join(row_parts) + " |"
        lines.append(row)
    
    return "\n".join(lines)


def print_comparison_table(
    organized_results: Dict,
    filter_zkvm: Optional[str] = None,
    filter_benchmark: Optional[str] = None,
    output_dir: Optional[str] = None,
    single_file: Optional[str] = None
):
    """
    Print comparison tables for each gas category and GPU count.
    
    Args:
        organized_results: Organized benchmark results
        filter_zkvm: Optional zkVM name to filter by
        filter_benchmark: Optional benchmark name pattern to filter by
        output_dir: Optional directory to write individual markdown files to
        single_file: Optional single file path to write all results to
    """
    # Sort by gas category and GPU count
    sorted_keys = sorted(organized_results.keys(), key=lambda x: (x[0], int(x[1])))
    
    # If output_dir is specified, create individual files
    if output_dir:
        output_path = Path(output_dir)
        output_path.mkdir(parents=True, exist_ok=True)
        
        for gas_category, gpu_count in sorted_keys:
            zkvm_results = organized_results[(gas_category, gpu_count)]
            
            content = write_comparison_table_to_file(
                gas_category, gpu_count, zkvm_results,
                filter_zkvm, filter_benchmark
            )
            
            if content:
                # Create filename: {gas-category}-{gpu-count}.md
                filename = f"{gas_category}-{gpu_count}.md"
                file_path = output_path / filename
                
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                
                print(f"Written: {file_path}")
        
        print(f"\nAll results written to directory: {output_dir}")
        return
    
    # If single_file is specified, write all to one file
    if single_file:
        f = open(single_file, 'w', encoding='utf-8')
        original_stdout = sys.stdout
        sys.stdout = f
    
    try:
        for gas_category, gpu_count in sorted_keys:
            zkvm_results = organized_results[(gas_category, gpu_count)]
            
            # Filter zkVMs if requested
            if filter_zkvm:
                zkvm_results = {k: v for k, v in zkvm_results.items() if k == filter_zkvm}
            
            # Get all unique zkVMs
            zkvms = sorted(zkvm_results.keys())
            
            if not zkvms:
                continue
            
            # Get all benchmarks
            all_benchmarks = get_all_benchmarks(zkvm_results)
            
            # Filter benchmarks if requested
            if filter_benchmark:
                all_benchmarks = [b for b in all_benchmarks if filter_benchmark.lower() in b.lower()]
            
            if not all_benchmarks:
                continue
            
            # Print header
            print(f"\n{'='*80}")
            print(f"Gas Category: {gas_category}")
            print(f"Number of GPUs: {gpu_count}")
            print(f"{'='*80}\n")
            
            # Create table header
            header = "| Benchmark | " + " | ".join(zkvms) + " |"
            separator = "|" + "|".join(["---"] * (len(zkvms) + 1)) + "|"
            
            print(header)
            print(separator)
            
            # Print each benchmark row
            for benchmark in all_benchmarks:
                row_parts = [benchmark]
                
                for zkvm in zkvms:
                    if benchmark in zkvm_results[zkvm]:
                        time_val = zkvm_results[zkvm][benchmark]
                        row_parts.append(format_time(time_val))
                    else:
                        row_parts.append("N/A")
                
                row = "| " + " | ".join(row_parts) + " |"
                print(row)
            
            print()
    
    finally:
        # Restore stdout if we redirected it
        if single_file:
            sys.stdout = original_stdout
            f.close()
            print(f"Results written to: {single_file}")


def parse_args():
    """Parse command line arguments."""
    parser = argparse.ArgumentParser(
        description='Compare proving times across different zkVMs from benchmark JSON files.',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog='''
Examples:
  # Show all benchmark results
  %(prog)s
  
  # Filter by zkVM (show only sp1 results)
  %(prog)s --zkvm sp1
  
  # Filter by benchmark name (show only CREATE benchmarks)
  %(prog)s --benchmark create
  
  # Export results to directory (creates separate files: 1M-1.md, 1M-4.md, etc.)
  %(prog)s --output results/
  
  # Export to a single markdown file
  %(prog)s --single-file all-results.md
  
  # Combine filters and export
  %(prog)s --zkvm risc0 --benchmark sstore --output results/
  
  # Specify custom directory
  %(prog)s --dir /path/to/benchmark/data
  
  # Disable test name formatting (show raw names)
  %(prog)s --no-format
        '''
    )
    
    parser.add_argument(
        '--zkvm',
        type=str,
        help='Filter results by zkVM name (e.g., sp1, risc0, zisk, airbender)'
    )
    
    parser.add_argument(
        '--benchmark',
        type=str,
        help='Filter results by benchmark name (case-insensitive substring match)'
    )
    
    parser.add_argument(
        '-o', '--output',
        type=str,
        help='Output directory for markdown files (creates {gas}-{gpu}.md files). If not specified, prints to stdout.'
    )
    
    parser.add_argument(
        '--single-file',
        type=str,
        help='Output to a single markdown file instead of multiple files per configuration.'
    )
    
    parser.add_argument(
        '--dir',
        type=str,
        help='Root directory to scan for zkevm-metrics-* directories (defaults to workspace root)'
    )
    
    parser.add_argument(
        '--quiet',
        action='store_true',
        help='Suppress informational messages (only show results)'
    )
    
    parser.add_argument(
        '--no-format',
        action='store_true',
        help='Disable display name formatting (show raw benchmark names)'
    )
    
    return parser.parse_args()


def main():
    """Main function to run the comparison."""
    try:
        args = parse_args()
        
        # Get the workspace root directory
        script_dir = Path(__file__).parent
        root_dir = script_dir.parent
        
        # Determine scan directory
        if args.dir:
            if Path(args.dir).is_absolute():
                scan_dir = Path(args.dir)
            else:
                scan_dir = root_dir / args.dir
        else:
            scan_dir = root_dir
        
        if not args.quiet:
            print("Scanning for zkevm-metrics-* directories...")
            print(f"Scan directory: {scan_dir}")
            if not args.no_format and PARSER_AVAILABLE:
                print("Using display names for readable benchmark names")
            elif args.no_format:
                print("Display name formatting disabled")
            elif not PARSER_AVAILABLE:
                print("Test name parser not available (using raw names)")
            print()
        
        # Find all benchmark directories
        benchmark_dirs = find_benchmark_directories(scan_dir)
        
        if not benchmark_dirs:
            print("No benchmark directories found!", file=sys.stderr)
            print(f"Searched in: {scan_dir}", file=sys.stderr)
            print("Looking for directories matching pattern: zkevm-metrics-{{zkvm}}-{{gas}}-{{gpus}} (gas can be 0.1M, 0.2M, 1M, etc.)", file=sys.stderr)
            print("(excluding zkevm-metrics-results*)", file=sys.stderr)
            return 1
        
        if not args.quiet:
            print(f"Found {len(benchmark_dirs)} benchmark directories:")
            for dir_path, zkvm, gas_category, gpu_count in benchmark_dirs:
                rel_path = dir_path.relative_to(root_dir) if dir_path.is_relative_to(root_dir) else dir_path
                print(f"  - {rel_path} (zkVM: {zkvm}, Gas: {gas_category}, GPUs: {gpu_count})")
            
            print("\nCollecting benchmark data from JSON files...\n")
        
        # Collect all benchmark data
        use_display_name = not args.no_format
        organized_results = collect_benchmark_data(benchmark_dirs, use_display_name)
        
        if not organized_results:
            print("No benchmark data found in JSON files!", file=sys.stderr)
            return 1
        
        # Print comparison tables
        print_comparison_table(
            organized_results,
            filter_zkvm=args.zkvm,
            filter_benchmark=args.benchmark,
            output_dir=args.output,
            single_file=args.single_file
        )
        
        if not args.quiet and not args.output and not args.single_file:
            print("\nComparison complete!")
        
        return 0
    
    except BrokenPipeError:
        # Handle broken pipe error (e.g., when piping to head)
        devnull = os.open(os.devnull, os.O_WRONLY)
        os.dup2(devnull, sys.stdout.fileno())
        return 0
    except KeyboardInterrupt:
        print("\n\nInterrupted by user", file=sys.stderr)
        return 130


if __name__ == "__main__":
    sys.exit(main())
