To run the profiling benchmarks, you need to have the following:

1. Run the script file `run_profiling_benchmarks.sh`
```bash
./scripts/download-and-extract-fixtures.sh
```

2. Then run the command

```bash
cargo run --bin profile-runner -- ./zkevm-fixtures/fixtures/state_tests/
```

3. The results will be saved in the `results/` directory
