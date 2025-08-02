#!/usr/bin/env python3
"""
Example Usage of zkEVM Analytics

This script demonstrates how to load and use the generated analytics JSON files
for creating website dashboards, reports, or other applications.
"""

import json
from pathlib import Path

def load_analytics(zkvm_name: str) -> dict:
    """Load analytics for a specific zkVM."""
    analytics_file = Path(__file__).parent / "analytics_output" / f"{zkvm_name}_analytics.json"
    
    if not analytics_file.exists():
        raise FileNotFoundError(f"Analytics file not found: {analytics_file}")
    
    with open(analytics_file, 'r') as f:
        return json.load(f)

def load_summary() -> dict:
    """Load the summary analytics comparing all zkVMs."""
    summary_file = Path(__file__).parent / "analytics_output" / "summary_analytics.json"
    
    with open(summary_file, 'r') as f:
        return json.load(f)

def print_zkvm_summary(zkvm_name: str):
    """Print a summary for a specific zkVM."""
    analytics = load_analytics(zkvm_name)
    
    print(f"\n🔍 {zkvm_name.upper()} Analytics Summary")
    print("=" * 40)
    
    summary = analytics["summary"]
    print(f"📊 Total Tests: {summary['total_tests']}")
    print(f"✅ Successful: {summary['successful_tests']} ({summary['success_rate_percent']}%)")
    print(f"❌ Failed: {summary['failed_tests']}")
    
    if analytics["performance"]["has_timing_data"]:
        perf = analytics["performance"]["proving_time_seconds"]
        print(f"⏱️  Average Proving Time: {perf['mean']:.1f}s")
        print(f"📈 Median Proving Time: {perf['median']:.1f}s")
        print(f"🚀 Fastest Test: {perf['min']:.1f}s")
        print(f"🐌 Slowest Test: {perf['max']:.1f}s")
    
    # Category breakdown
    print(f"\n📋 Test Categories:")
    for category, stats in analytics["categories"].items():
        success_rate = stats["success_rate_percent"]
        print(f"  {category}: {stats['successful']}/{stats['total']} ({success_rate}%)")
    
    # Top error patterns
    if analytics["errors"]["total_failures"] > 0:
        print(f"\n🐛 Top Error Patterns:")
        for error in analytics["errors"]["error_distribution"][:3]:
            print(f"  {error['pattern']}: {error['count']} ({error['percentage']:.1f}%)")

def print_comparison():
    """Print a comparison between all zkVMs."""
    summary = load_summary()
    
    print(f"\n🏆 zkVM Comparison")
    print("=" * 50)
    
    for zkvm, stats in summary["comparison"].items():
        print(f"\n{zkvm.upper()}:")
        print(f"  Success Rate: {stats['success_rate_percent']}%")
        print(f"  Total Tests: {stats['total_tests']}")
        
        if "performance" in stats:
            print(f"  Avg Proving Time: {stats['performance']['mean_proving_time_ms']/1000:.1f}s")

def get_fastest_tests(zkvm_name: str, limit: int = 5):
    """Get the fastest tests for a zkVM."""
    analytics = load_analytics(zkvm_name)
    return analytics["rankings"]["fastest"][:limit]

def get_slowest_tests(zkvm_name: str, limit: int = 5):
    """Get the slowest tests for a zkVM."""
    analytics = load_analytics(zkvm_name)
    return analytics["rankings"]["slowest"][:limit]

def get_tests_by_category(zkvm_name: str, category: str):
    """Get all tests for a specific category."""
    analytics = load_analytics(zkvm_name)
    return [test for test in analytics["individual_tests"] 
            if test["test_category"] == category]

def get_failed_tests_with_pattern(zkvm_name: str, error_pattern: str):
    """Get failed tests matching a specific error pattern."""
    analytics = load_analytics(zkvm_name)
    
    for error_dist in analytics["errors"]["error_distribution"]:
        if error_dist["pattern"] == error_pattern:
            return error_dist["examples"]
    
    return []

# Example usage for website dashboard
def generate_dashboard_data():
    """Generate data structure suitable for a web dashboard."""
    summary = load_summary()
    
    dashboard_data = {
        "overview": {
            "total_zkvms": len(summary["zkvms"]),
            "generated_at": summary["generated_at"]
        },
        "zkvm_cards": []
    }
    
    for zkvm in summary["zkvms"]:
        analytics = load_analytics(zkvm)
        
        card_data = {
            "name": zkvm,
            "success_rate": analytics["summary"]["success_rate_percent"],
            "total_tests": analytics["summary"]["total_tests"],
            "successful_tests": analytics["summary"]["successful_tests"],
            "failed_tests": analytics["summary"]["failed_tests"],
            "categories": analytics["categories"],
            "top_errors": analytics["errors"]["error_distribution"][:3] if analytics["errors"]["total_failures"] > 0 else []
        }
        
        if analytics["performance"]["has_timing_data"]:
            card_data["performance"] = {
                "mean_seconds": analytics["performance"]["proving_time_seconds"]["mean"],
                "median_seconds": analytics["performance"]["proving_time_seconds"]["median"]
            }
        
        dashboard_data["zkvm_cards"].append(card_data)
    
    return dashboard_data

def main():
    """Example usage of the analytics functions."""
    print("🚀 zkEVM Analytics Example Usage")
    
    # Load summary to see available zkVMs
    summary = load_summary()
    available_zkvms = summary["zkvms"]
    
    print(f"📊 Available zkVMs: {', '.join(available_zkvms)}")
    
    # Print summary for each zkVM
    for zkvm in available_zkvms:
        print_zkvm_summary(zkvm)
    
    # Print comparison
    print_comparison()
    
    # Example: Get fastest tests for RISC0
    if "risc0" in available_zkvms:
        print(f"\n🚀 Fastest RISC0 Tests:")
        fastest = get_fastest_tests("risc0", 3)
        for i, test in enumerate(fastest, 1):
            print(f"  {i}. {test['name']} ({test['proving_time_seconds']}s)")
    
    # Example: Generate dashboard data
    print(f"\n📊 Dashboard Data Structure:")
    dashboard = generate_dashboard_data()
    print(f"  Total zkVMs: {dashboard['overview']['total_zkvms']}")
    for card in dashboard["zkvm_cards"]:
        print(f"  {card['name']}: {card['success_rate']}% success rate")

if __name__ == "__main__":
    main() 