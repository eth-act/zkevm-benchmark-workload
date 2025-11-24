# Benchmark Results Directory

This directory contains the benchmark results displayed in the documentation. Results are automatically updated when profiling is run.

## Directory Structure

```
benchmark-results/
├── README.md                    # This file
├── gas-categorized/             # Results by gas categories
├── zkvm-comparisons/            # Results by zkVM implementations
├── comparisons/                 # Comparison reports
└── statistics/                  # Statistical analysis reports
```

## How Results Are Updated

Results are automatically updated when you run:

```bash
# Generate results and update documentation
./scripts/generate_results.sh --compare --statistics --output benchmark-results/markdown-reports/latest/profiling-results.md
./scripts/update-docs-with-results.sh
```

## Viewing Results

Results are displayed in the main [Benchmark Results](/benchmark-results) documentation page.
