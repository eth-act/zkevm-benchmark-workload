#!/usr/bin/env python3
"""
Generate a presentable metrics summary from the zkVM benchmark results.

This script processes all JSON metric files in the zkevm-metrics directory
and creates an HTML summary with tables and visualizations.
"""

import json
import os
import sys
from pathlib import Path
from typing import Dict, List, Any
import argparse

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

def generate_summary_table(metrics_data: Dict[str, List[Dict[str, Any]]]) -> str:
    """Generate an HTML summary table comparing zkVMs."""
    html = """
    <h2>zkVM Performance Summary</h2>
    <table class="summary-table">
        <thead>
            <tr>
                <th>zkVM</th>
                <th>Total Tests</th>
                <th>Avg Cycles</th>
                <th>Min Cycles</th>
                <th>Max Cycles</th>
                <th>Total Validation Cycles</th>
            </tr>
        </thead>
        <tbody>
    """
    
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
        
        html += f"""
            <tr>
                <td class="zkvm-name">{zkvm_name}</td>
                <td>{total_tests}</td>
                <td>{format_cycles(int(avg_cycles))}</td>
                <td>{format_cycles(min_cycles)}</td>
                <td>{format_cycles(max_cycles)}</td>
                <td>{format_cycles(total_validation)}</td>
            </tr>
        """
    
    html += """
        </tbody>
    </table>
    """
    return html

def generate_detailed_tables(metrics_data: Dict[str, List[Dict[str, Any]]]) -> str:
    """Generate detailed tables for each zkVM."""
    html = "<h2>Detailed Results by zkVM</h2>"
    
    for zkvm_name, tests in metrics_data.items():
        if not tests:
            continue
            
        html += f"""
        <h3>{zkvm_name.upper()} Results</h3>
        <table class="detailed-table">
            <thead>
                <tr>
                    <th>Test Name</th>
                    <th>Total Cycles</th>
                    <th>Validation</th>
                    <th>Read Input</th>
                    <th>Verify Witness</th>
                </tr>
            </thead>
            <tbody>
        """
        
        # Sort tests by total cycles (descending)
        sorted_tests = sorted(tests, key=lambda x: x.get('total_num_cycles', 0), reverse=True)
        
        for test in sorted_tests[:20]:  # Show top 20 most expensive tests
            name = test.get('name', 'Unknown')
            # Truncate long test names
            if len(name) > 80:
                name = name[:77] + "..."
                
            total_cycles = test.get('total_num_cycles', 0)
            region_cycles = test.get('region_cycles', {})
            validation = region_cycles.get('validation', 0)
            read_input = region_cycles.get('read_input', 0)
            verify_witness = region_cycles.get('verify-witness', 0)
            
            html += f"""
                <tr>
                    <td title="{test.get('name', 'Unknown')}">{name}</td>
                    <td>{format_cycles(total_cycles)}</td>
                    <td>{format_cycles(validation)}</td>
                    <td>{format_cycles(read_input)}</td>
                    <td>{format_cycles(verify_witness)}</td>
                </tr>
            """
        
        if len(sorted_tests) > 20:
            html += f"""
                <tr class="more-results">
                    <td colspan="5">... and {len(sorted_tests) - 20} more tests</td>
                </tr>
            """
        
        html += """
            </tbody>
        </table>
        """
    
    return html

def generate_html_report(metrics_data: Dict[str, List[Dict[str, Any]]]) -> str:
    """Generate a complete HTML report."""
    
    css = """
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            margin: 20px;
            line-height: 1.6;
            color: #333;
        }
        h1, h2, h3 {
            color: #2c3e50;
        }
        .summary-table, .detailed-table {
            border-collapse: collapse;
            width: 100%;
            margin: 20px 0;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }
        .summary-table th, .detailed-table th,
        .summary-table td, .detailed-table td {
            border: 1px solid #ddd;
            padding: 12px;
            text-align: left;
        }
        .summary-table th, .detailed-table th {
            background-color: #f8f9fa;
            font-weight: 600;
        }
        .zkvm-name {
            font-weight: 600;
            color: #2980b9;
        }
        .summary-table tr:nth-child(even) {
            background-color: #f8f9fa;
        }
        .detailed-table tr:nth-child(even) {
            background-color: #f8f9fa;
        }
        .more-results {
            font-style: italic;
            color: #666;
        }
        .header {
            text-align: center;
            margin-bottom: 40px;
        }
        .timestamp {
            color: #666;
            font-size: 0.9em;
        }
    </style>
    """
    
    from datetime import datetime
    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    
    html = f"""
    <!DOCTYPE html>
    <html>
    <head>
        <title>zkEVM Benchmark Results</title>
        <meta charset="utf-8">
        {css}
    </head>
    <body>
        <div class="header">
            <h1>zkEVM Benchmarking Results</h1>
            <p class="timestamp">Generated on {timestamp}</p>
        </div>
        
        {generate_summary_table(metrics_data)}
        {generate_detailed_tables(metrics_data)}
    </body>
    </html>
    """
    
    return html

def main():
    parser = argparse.ArgumentParser(description="Generate metrics summary from zkVM benchmark results")
    parser.add_argument(
        "--metrics-dir", 
        type=Path, 
        default="zkevm-metrics",
        help="Directory containing metrics data (default: zkevm-metrics)"
    )
    parser.add_argument(
        "--output", 
        type=Path, 
        default="metrics-summary.html",
        help="Output HTML file (default: metrics-summary.html)"
    )
    
    args = parser.parse_args()
    
    if not args.metrics_dir.exists():
        print(f"Error: Metrics directory {args.metrics_dir} does not exist", file=sys.stderr)
        sys.exit(1)
    
    print(f"Loading metrics data from {args.metrics_dir}...")
    metrics_data = load_metrics_data(args.metrics_dir)
    
    total_tests = sum(len(tests) for tests in metrics_data.values())
    print(f"Found {total_tests} test results across {len(metrics_data)} zkVMs")
    
    if total_tests == 0:
        print("No metrics data found!", file=sys.stderr)
        sys.exit(1)
    
    print(f"Generating HTML report...")
    html_report = generate_html_report(metrics_data)
    
    with open(args.output, 'w') as f:
        f.write(html_report)
    
    print(f"✅ Metrics summary generated: {args.output}")
    print(f"Open the file in your browser to view the results.")

if __name__ == "__main__":
    main() 