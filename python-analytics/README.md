# zkEVM Analytics Generator

This directory contains Python tools for analyzing zkEVM benchmark data and generating comprehensive analytics JSON files suitable for website dashboards.

## Quick Start

```bash
cd python-analytics
python3 generate_analytics.py
```

This will:
- Load all benchmark data from `../zkevm-metrics/`
- Generate individual analytics files for each zkVM
- Create a summary comparison file
- Save everything to `analytics_output/`

## Example Usage

After generating the analytics, you can use the example script to see how to work with the data:

```bash
python3 example_usage.py
```

This demonstrates:
- Loading analytics for specific zkVMs
- Comparing performance across zkVMs
- Extracting fastest/slowest tests
- Filtering tests by category or error pattern
- Generating dashboard-ready data structures

## Generated Files

### Individual zkVM Analytics
Each zkVM gets its own analytics file (e.g., `risc0_analytics.json`, `sp1_analytics.json`) containing:

```json
{
  "zkvm_name": "risc0",
  "generated_at": "2024-01-15T10:30:00",
  
  "summary": {
    "total_tests": 120,
    "successful_tests": 85,
    "failed_tests": 35,
    "success_rate_percent": 70.83
  },
  
  "performance": {
    "has_timing_data": true,
    "test_count": 85,
    "proving_time_ms": {
      "mean": 1250000.5,
      "median": 980000.0,
      "min": 456000,
      "max": 2450000,
      "std_dev": 654321.2
    },
    "proving_time_seconds": {
      "mean": 1250.0,
      "median": 980.0,
      "min": 456.0,
      "max": 2450.0
    }
  },
  
  "categories": {
    "bytecode": {
      "total": 40,
      "successful": 35,
      "failed": 5,
      "success_rate_percent": 87.5,
      "performance": {
        "test_count": 35,
        "mean_ms": 890000.0,
        "median_ms": 780000.0,
        "min_ms": 456000,
        "max_ms": 2450000
      }
    },
    "stateful_opcodes": { },
    "compute": { }
  },
  
  "errors": {
    "total_failures": 35,
    "error_patterns": {
      "Circuit Size Exceeded": 20,
      "Memory Allocation Failed": 10,
      "Timeout": 3,
      "Other": 2
    },
    "error_distribution": [
      {
        "pattern": "Circuit Size Exceeded",
        "count": 20,
        "percentage": 57.14,
        "examples": [
          {
            "test_name": "test_worst_compute::bls12_g2msm",
            "error_reason": "Error: RISC0 circuit size exceeded - instruction count limit reached"
          }
        ]
      }
    ]
  },
  
  "rankings": {
    "fastest": [
      {
        "name": "test_worst_bytecode_simple[opcodes_PUSH_POP]",
        "category": "bytecode",
        "proving_time_ms": 456000,
        "proving_time_seconds": 456.0
      }
    ],
    "slowest": [
      {
        "name": "test_worst_bytecode_single_opcode[CALLCODE]",
        "category": "bytecode", 
        "proving_time_ms": 2450000,
        "proving_time_seconds": 2450.0
      }
    ]
  },
  
  "individual_tests": [
    {
      "name": "test_worst_bytecode_simple[opcodes_PUSH_POP]",
      "zkvm": "risc0",
      "test_category": "bytecode",
      "status": "success",
      "proving_time_ms": 456000,
      "error_reason": null
    }
  ]
}
```

### Summary Analytics
The `summary_analytics.json` file provides a high-level comparison across all zkVMs:

```json
{
  "generated_at": "2024-01-15T10:30:00",
  "zkvms": ["risc0", "sp1"],
  "comparison": {
    "risc0": {
      "total_tests": 120,
      "successful_tests": 85,
      "failed_tests": 35,
      "success_rate_percent": 70.83,
      "performance": {
        "mean_proving_time_ms": 1250000.5,
        "median_proving_time_ms": 980000.0
      }
    },
    "sp1": { }
  }
}
```

## Data Structure

The analytics are generated from the `zkevm-metrics/` directory structure:
```
zkevm-metrics/
├── risc0/
│   ├── tests/           # Successful test results
│   │   ├── bytecode_results.json
│   │   └── stateful_opcodes_results.json
│   └── crash/           # Failed test results
│       ├── bytecode_failures.json
│       ├── stateful_opcodes_failures.json
│       └── compute_failures.json
└── sp1/
    ├── tests/
    └── crash/
```

## Analytics Features

### 📊 Performance Metrics
- Mean, median, min, max proving times
- Performance breakdown by test category
- Fastest and slowest test rankings
- Statistical analysis (standard deviation)

### 🎯 Success Rate Analysis
- Overall success rates per zkVM
- Success rates broken down by test category
- Detailed success/failure counts

### 🐛 Error Analysis
- Automatic error pattern recognition
- Error frequency and percentage breakdown
- Example error messages for each pattern
- Common failure types: Circuit Size Exceeded, Memory Allocation Failed, Timeouts, etc.

### 📋 Individual Test Details
- Complete information for every test
- Test categorization (bytecode, stateful_opcodes, compute, other)
- Easy filtering and searching capabilities for web interfaces

### 🔍 Test Categories
Tests are automatically categorized based on their filenames:
- **bytecode**: Bytecode execution tests
- **stateful_opcodes**: Stateful operation tests
- **compute**: Computational workload tests
- **other**: Uncategorized tests

## Use Cases

### Benchmark Website
The JSON files are perfect for:
- Dashboard widgets showing success rates
- Performance comparison charts
- Error analysis reports
- Individual test result tables
- Historical tracking of improvements

### API Integration
Easy to serve via REST API:
```javascript
// Get RISC0 analytics
fetch('/api/analytics/risc0')
  .then(response => response.json())
  .then(data => {
    console.log(`RISC0 success rate: ${data.summary.success_rate_percent}%`);
    console.log(`Average proving time: ${data.performance.proving_time_seconds.mean}s`);
  });
```

### Monitoring & Alerts
Use the JSON data for:
- Performance regression detection
- Success rate monitoring
- Error pattern alerts
- Automated reporting

## Requirements

- Python 3.7+
- No external dependencies (uses only standard library)

## File Structure

```
python-analytics/
├── zkvm_analyzer.py       # Core analytics engine
├── generate_analytics.py  # Simple script to run analytics
├── example_usage.py       # Example of how to use the analytics data
├── README.md             # This file
└── analytics_output/     # Generated JSON files (created automatically)
    ├── risc0_analytics.json
    ├── sp1_analytics.json
    └── summary_analytics.json
```

## Extending the Analytics

To add new analytics or modify the output format, edit `zkvm_analyzer.py`:

- `_calculate_performance_stats()`: Add new performance metrics
- `_analyze_errors()`: Add new error pattern recognition
- `generate_zkvm_analytics()`: Add new sections to the output JSON
- `_get_performance_rankings()`: Modify ranking criteria

The modular design makes it easy to add new features while maintaining backward compatibility with existing web interfaces. 