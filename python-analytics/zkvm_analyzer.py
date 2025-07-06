#!/usr/bin/env python3
"""
zkEVM Benchmark Data Analyzer

Loads and processes zkEVM benchmark data from the zkevm-metrics directory
and generates comprehensive analytics for each zkVM.
"""

import json
import os
from pathlib import Path
from typing import Dict, List, Optional, Any
from dataclasses import dataclass
from collections import defaultdict
import statistics

@dataclass
class TestResult:
    """Represents a single test result."""
    name: str
    zkvm: str
    test_category: str
    status: str  # "success" or "crashed"
    proving_time_ms: Optional[int] = None
    error_reason: Optional[str] = None
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary for JSON serialization."""
        return {
            "name": self.name,
            "zkvm": self.zkvm,
            "test_category": self.test_category,
            "status": self.status,
            "proving_time_ms": self.proving_time_ms,
            "error_reason": self.error_reason
        }

class ZKVMAnalyzer:
    """Analyzes zkEVM benchmark data and generates comprehensive analytics."""
    
    def __init__(self, metrics_dir: str = "zkevm-metrics"):
        """Initialize with path to metrics directory."""
        self.metrics_dir = Path(metrics_dir)
        self.test_results: List[TestResult] = []
        
    def load_all_data(self) -> None:
        """Load all benchmark data from the metrics directory."""
        print("🔍 Loading zkEVM benchmark data...")
        
        if not self.metrics_dir.exists():
            raise FileNotFoundError(f"Metrics directory not found: {self.metrics_dir}")
        
        # Process each zkVM directory
        for zkvm_dir in self.metrics_dir.iterdir():
            if zkvm_dir.is_dir():
                zkvm_name = zkvm_dir.name
                print(f"  📊 Processing {zkvm_name}...")
                self._load_zkvm_data(zkvm_name, zkvm_dir)
        
        print(f"✅ Loaded {len(self.test_results)} total test results")
    
    def _load_zkvm_data(self, zkvm_name: str, zkvm_dir: Path) -> None:
        """Load data for a specific zkVM."""
        
        # Load successful tests
        tests_dir = zkvm_dir / "tests"
        if tests_dir.exists():
            # Check if we have aggregated JSON files (RISC0 style)
            json_files = list(tests_dir.glob("*.json"))
            if json_files:
                # Aggregated format
                for result_file in json_files:
                    category = self._extract_category_from_filename(result_file.name)
                    self._load_test_file(result_file, zkvm_name, category, "success")
            else:
                # Check for individual files in subdirectories (SP1 style)
                for subdir in tests_dir.iterdir():
                    if subdir.is_dir():
                        for result_file in subdir.glob("*.json"):
                            category = self._extract_category_from_filename(result_file.name)
                            self._load_test_file(result_file, zkvm_name, category, "success")
        
        # Load crashed tests
        crash_dir = zkvm_dir / "crash"
        if crash_dir.exists():
            # Check if we have aggregated JSON files (RISC0 style)
            json_files = list(crash_dir.glob("*.json"))
            if json_files:
                # Aggregated format
                for result_file in json_files:
                    category = self._extract_category_from_filename(result_file.name)
                    self._load_test_file(result_file, zkvm_name, category, "crashed")
            else:
                # Check for individual files in subdirectories (SP1 style)
                # SP1 has crash/tests/zkevm/ structure, so we need to go deeper
                for subdir in crash_dir.rglob("*.json"):
                    if subdir.is_file():
                        category = self._extract_category_from_filename(subdir.name)
                        self._load_test_file(subdir, zkvm_name, category, "crashed")
    
    def _extract_category_from_filename(self, filename: str) -> str:
        """Extract test category from filename."""
        if "bytecode" in filename:
            return "bytecode"
        elif "stateful" in filename or "opcodes" in filename:
            return "stateful_opcodes"
        elif "compute" in filename:
            return "compute"
        else:
            return "other"
    
    def _load_test_file(self, file_path: Path, zkvm_name: str, category: str, status: str) -> None:
        """Load results from a single JSON file."""
        try:
            with open(file_path, 'r') as f:
                data = json.load(f)
            
            for item in data:
                if status == "success" and "Proving" in item:
                    test_data = item["Proving"]
                    result = TestResult(
                        name=test_data["name"],
                        zkvm=zkvm_name,
                        test_category=category,
                        status=status,
                        proving_time_ms=test_data["proving_time_ms"]
                    )
                elif status == "crashed" and "Crashed" in item:
                    test_data = item["Crashed"]
                    result = TestResult(
                        name=test_data["name"],
                        zkvm=zkvm_name,
                        test_category=category,
                        status=status,
                        error_reason=test_data["reason"]
                    )
                else:
                    continue
                
                self.test_results.append(result)
                
        except Exception as e:
            print(f"    ⚠️  Warning: Could not load {file_path}: {e}")
    
    def generate_zkvm_analytics(self, zkvm_name: str) -> Dict[str, Any]:
        """Generate comprehensive analytics for a specific zkVM."""
        
        # Filter results for this zkVM
        zkvm_results = [r for r in self.test_results if r.zkvm == zkvm_name]
        
        if not zkvm_results:
            return {"error": f"No data found for zkVM: {zkvm_name}"}
        
        # Basic stats
        total_tests = len(zkvm_results)
        successful_tests = [r for r in zkvm_results if r.status == "success"]
        failed_tests = [r for r in zkvm_results if r.status == "crashed"]
        
        success_count = len(successful_tests)
        failure_count = len(failed_tests)
        success_rate = (success_count / total_tests * 100) if total_tests > 0 else 0
        
        # Performance analytics
        performance_stats = self._calculate_performance_stats(successful_tests)
        
        # Category breakdown
        category_stats = self._calculate_category_stats(zkvm_results)
        
        # Error analysis
        error_analysis = self._analyze_errors(failed_tests)
        
        # Individual test details
        individual_tests = [test.to_dict() for test in zkvm_results]
        
        # Performance rankings
        performance_rankings = self._get_performance_rankings(successful_tests)
        
        return {
            "zkvm_name": zkvm_name,
            "generated_at": self._get_timestamp(),
            
            # Summary statistics
            "summary": {
                "total_tests": total_tests,
                "successful_tests": success_count,
                "failed_tests": failure_count,
                "success_rate_percent": round(success_rate, 2)
            },
            
            # Performance metrics
            "performance": performance_stats,
            
            # Test category breakdown
            "categories": category_stats,
            
            # Error analysis
            "errors": error_analysis,
            
            # Performance rankings
            "rankings": performance_rankings,
            
            # All individual test results
            "individual_tests": individual_tests
        }
    
    def _calculate_performance_stats(self, successful_tests: List[TestResult]) -> Dict[str, Any]:
        """Calculate performance statistics from successful tests."""
        tests_with_times = [t for t in successful_tests if t.proving_time_ms is not None]
        
        if not tests_with_times:
            return {
                "has_timing_data": False,
                "message": "No timing data available"
            }
        
        times = [t.proving_time_ms for t in tests_with_times]
        times_seconds = [t / 1000 for t in times]
        
        return {
            "has_timing_data": True,
            "test_count": len(tests_with_times),
            "proving_time_ms": {
                "mean": round(statistics.mean(times), 2),
                "median": round(statistics.median(times), 2),
                "min": min(times),
                "max": max(times),
                "std_dev": round(statistics.stdev(times) if len(times) > 1 else 0, 2)
            },
            "proving_time_seconds": {
                "mean": round(statistics.mean(times_seconds), 2),
                "median": round(statistics.median(times_seconds), 2),
                "min": round(min(times_seconds), 2),
                "max": round(max(times_seconds), 2)
            }
        }
    
    def _calculate_category_stats(self, zkvm_results: List[TestResult]) -> Dict[str, Any]:
        """Calculate statistics broken down by test category."""
        categories = {}
        
        for result in zkvm_results:
            category = result.test_category
            if category not in categories:
                categories[category] = {
                    "total": 0,
                    "successful": 0,
                    "failed": 0,
                    "success_rate_percent": 0,
                    "performance": {}
                }
            
            categories[category]["total"] += 1
            if result.status == "success":
                categories[category]["successful"] += 1
            else:
                categories[category]["failed"] += 1
        
        # Calculate success rates and performance stats per category
        for category, stats in categories.items():
            if stats["total"] > 0:
                stats["success_rate_percent"] = round(
                    stats["successful"] / stats["total"] * 100, 2
                )
            
            # Performance stats for this category
            category_successful = [r for r in zkvm_results 
                                 if r.test_category == category and r.status == "success" and r.proving_time_ms]
            
            if category_successful:
                times = [r.proving_time_ms for r in category_successful]
                stats["performance"] = {
                    "test_count": len(category_successful),
                    "mean_ms": round(statistics.mean(times), 2),
                    "median_ms": round(statistics.median(times), 2),
                    "min_ms": min(times),
                    "max_ms": max(times)
                }
            else:
                stats["performance"] = {"test_count": 0}
        
        return categories
    
    def _analyze_errors(self, failed_tests: List[TestResult]) -> Dict[str, Any]:
        """Analyze error patterns in failed tests."""
        if not failed_tests:
            return {
                "total_failures": 0,
                "error_patterns": {},
                "common_errors": []
            }
        
        error_patterns = defaultdict(int)
        error_examples = defaultdict(list)
        
        for test in failed_tests:
            error_reason = test.error_reason or "Unknown error"
            
            # Categorize errors
            if "circuit size exceeded" in error_reason:
                pattern = "Circuit Size Exceeded"
            elif "Memory allocation failed" in error_reason:
                pattern = "Memory Allocation Failed"
            elif "timeout" in error_reason.lower():
                pattern = "Timeout"
            elif "unwrap" in error_reason and "Err" in error_reason:
                pattern = "Unwrap Error"
            elif "Panic" in error_reason:
                pattern = "Panic"
            elif "Invalid witness" in error_reason:
                pattern = "Invalid Witness"
            else:
                pattern = "Other"
            
            error_patterns[pattern] += 1
            if len(error_examples[pattern]) < 3:  # Keep first 3 examples
                error_examples[pattern].append({
                    "test_name": test.name,
                    "error_reason": error_reason
                })
        
        # Sort by frequency
        sorted_patterns = sorted(error_patterns.items(), key=lambda x: x[1], reverse=True)
        
        return {
            "total_failures": len(failed_tests),
            "error_patterns": dict(error_patterns),
            "error_distribution": [
                {
                    "pattern": pattern,
                    "count": count,
                    "percentage": round(count / len(failed_tests) * 100, 2),
                    "examples": error_examples[pattern]
                }
                for pattern, count in sorted_patterns
            ]
        }
    
    def _get_performance_rankings(self, successful_tests: List[TestResult]) -> Dict[str, Any]:
        """Get performance rankings - fastest and slowest tests."""
        tests_with_times = [t for t in successful_tests if t.proving_time_ms is not None]
        
        if not tests_with_times:
            return {"fastest": [], "slowest": []}
        
        # Sort by proving time
        sorted_by_time = sorted(tests_with_times, key=lambda x: x.proving_time_ms)
        
        fastest_5 = sorted_by_time[:5]
        slowest_5 = sorted_by_time[-5:][::-1]  # Reverse to get slowest first
        
        return {
            "fastest": [
                {
                    "name": test.name,
                    "category": test.test_category,
                    "proving_time_ms": test.proving_time_ms,
                    "proving_time_seconds": round(test.proving_time_ms / 1000, 2)
                }
                for test in fastest_5
            ],
            "slowest": [
                {
                    "name": test.name,
                    "category": test.test_category,
                    "proving_time_ms": test.proving_time_ms,
                    "proving_time_seconds": round(test.proving_time_ms / 1000, 2)
                }
                for test in slowest_5
            ]
        }
    
    def _get_timestamp(self) -> str:
        """Get current timestamp in ISO format."""
        from datetime import datetime
        return datetime.now().isoformat()
    
    def get_available_zkvms(self) -> List[str]:
        """Get list of available zkVMs from the loaded data."""
        return sorted(list(set(result.zkvm for result in self.test_results)))
    
    def generate_all_analytics(self, output_dir: str = "analytics_output") -> None:
        """Generate analytics JSON files for all zkVMs."""
        output_path = Path(output_dir)
        output_path.mkdir(exist_ok=True)
        
        available_zkvms = self.get_available_zkvms()
        
        print(f"📈 Generating analytics for {len(available_zkvms)} zkVMs...")
        
        for zkvm in available_zkvms:
            print(f"  📊 Generating analytics for {zkvm}...")
            
            analytics = self.generate_zkvm_analytics(zkvm)
            
            output_file = output_path / f"{zkvm}_analytics.json"
            with open(output_file, 'w') as f:
                json.dump(analytics, f, indent=2)
            
            print(f"    💾 Saved: {output_file}")
        
        # Generate combined summary
        self._generate_summary_analytics(output_path, available_zkvms)
        
        print(f"✅ Analytics generation complete! Files saved in {output_path}/")
    
    def _generate_summary_analytics(self, output_path: Path, zkvms: List[str]) -> None:
        """Generate a summary analytics file comparing all zkVMs."""
        print("  📋 Generating combined summary...")
        
        summary = {
            "generated_at": self._get_timestamp(),
            "zkvms": zkvms,
            "comparison": {}
        }
        
        for zkvm in zkvms:
            zkvm_results = [r for r in self.test_results if r.zkvm == zkvm]
            successful = [r for r in zkvm_results if r.status == "success"]
            failed = [r for r in zkvm_results if r.status == "crashed"]
            
            summary["comparison"][zkvm] = {
                "total_tests": len(zkvm_results),
                "successful_tests": len(successful),
                "failed_tests": len(failed),
                "success_rate_percent": round(len(successful) / len(zkvm_results) * 100, 2) if zkvm_results else 0
            }
            
            # Add performance summary if available
            tests_with_times = [r for r in successful if r.proving_time_ms]
            if tests_with_times:
                times = [r.proving_time_ms for r in tests_with_times]
                summary["comparison"][zkvm]["performance"] = {
                    "mean_proving_time_ms": round(statistics.mean(times), 2),
                    "median_proving_time_ms": round(statistics.median(times), 2)
                }
        
        summary_file = output_path / "summary_analytics.json"
        with open(summary_file, 'w') as f:
            json.dump(summary, f, indent=2)
        
        print(f"    💾 Saved: {summary_file}") 