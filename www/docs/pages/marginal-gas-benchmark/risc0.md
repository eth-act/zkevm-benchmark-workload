# ZK Gas Benchmark Report 2025-12-29 (risc0, both)

## Context

- **Generated**: 2025-12-29 21:10:10
- **Prover**: risc0-v3.0.4
- **Mode**: both
- **CPU**: AMD EPYC 7B13 64-Core Processor
- **RAM**: 371 GiB
- **GPUs**: 4x NVIDIA GeForce RTX 4090

## Regression Results

### Time/Gas Bar Chart (R² ≥ 0.7)

*Only opcodes/precompiles with R² ≥ 0.7 are shown.*

![Time/Gas Bar Chart](/marginal-gas-benchmark/risc0/plots/bar_time_per_gas.png)

### Cycles/Gas Bar Chart (R² ≥ 0.9)

*Only opcodes/precompiles with R² ≥ 0.9 (green) are shown.*

![Cycles/Gas Bar Chart](/marginal-gas-benchmark/risc0/plots/bar_cycles_per_gas.png)

### Regression Results

| Opcode | Max Ops | Max Gas | Max ZK Cycles | Time/Gas (R²) | Cycles/Gas (R²) | Time/Cycle (R²) |
|--------|---------|---------|---------------|---------------|-----------------|-----------------|
| modexp | 36.00 | 116.12K | 193.68M | 1.36ms (<span style="color: #28a745; font-weight: bold;">0.9998</span>) | 3.91K (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9998</span>) |
| blake2f | 10.00 | 704.62K | 461.37M | 242.84µs (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 702.03 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">1.0000</span>) |
| mulmod | 210.00K | 4.60M | 824.50M | 151.85µs (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 420.63 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.36µs (<span style="color: #28a745; font-weight: bold;">1.0000</span>) |
| div | 210.00K | 3.30M | 407.58M | 110.29µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) | 306.78 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.36µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) |
| mod | 210.00K | 3.30M | 356.34M | 94.55µs (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 257.97 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.37µs (<span style="color: #28a745; font-weight: bold;">1.0000</span>) |
| sdiv | 210.00K | 3.30M | 355.50M | 92.84µs (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 257.18 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.36µs (<span style="color: #28a745; font-weight: bold;">1.0000</span>) |
| bn128_mul | 72.00 | 487.85K | 100.68M | 86.68µs (<span style="color: #28a745; font-weight: bold;">0.9997</span>) | 229.21 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.38µs (<span style="color: #28a745; font-weight: bold;">0.9997</span>) |
| point_evaluation | 9.00 | 499.92K | 101.65M | 85.54µs (<span style="color: #28a745; font-weight: bold;">0.9945</span>) | 223.09 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.38µs (<span style="color: #28a745; font-weight: bold;">0.9945</span>) |
| selfbalance | 600.00K | 5.59M | 786.76M | 84.67µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) | 241.80 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) |
| keccak256 | 24.00K | 38.51M | 3.07B | 84.62µs (<span style="color: #28a745; font-weight: bold;">0.9980</span>) | 80.73 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 1.05µs (<span style="color: #28a745; font-weight: bold;">0.9980</span>) |
| bn128_pairing | 8.00 | 956.88K | 205.53M | 81.72µs (<span style="color: #28a745; font-weight: bold;">0.9968</span>) | 225.96 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.36µs (<span style="color: #28a745; font-weight: bold;">0.9968</span>) |
| ecrecover | 201.00 | 674.36K | 119.41M | 81.32µs (<span style="color: #28a745; font-weight: bold;">0.9921</span>) | 193.83 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.42µs (<span style="color: #28a745; font-weight: bold;">0.9921</span>) |
| addmod | 201.00K | 4.41M | 428.76M | 71.00µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) | 196.25 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.36µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) |
| bls12_g1msm | 39.00 | 942.30K | 81.33M | 37.74µs (<span style="color: #28a745; font-weight: bold;">0.9893</span>) | 89.91 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.42µs (<span style="color: #28a745; font-weight: bold;">0.9893</span>) |
| bls12_pairing | 9.00 | 983.01K | 82.96M | 36.15µs (<span style="color: #28a745; font-weight: bold;">0.9979</span>) | 88.19 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.41µs (<span style="color: #28a745; font-weight: bold;">0.9979</span>) |
| eq | 450.00K | 6.11M | 318.09M | 34.89µs (<span style="color: #28a745; font-weight: bold;">0.9993</span>) | 101.29 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.34µs (<span style="color: #28a745; font-weight: bold;">0.9993</span>) |
| exp | 2.25K | 3.69M | 339.20M | 32.61µs (<span style="color: #28a745; font-weight: bold;">0.9997</span>) | 92.99 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9997</span>) |
| bls12_map_fp_to_g1 | 75.00 | 468.01K | 24.69M | 31.51µs (<span style="color: #28a745; font-weight: bold;">0.9772</span>) | 55.78 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.56µs (<span style="color: #28a745; font-weight: bold;">0.9773</span>) |
| smod | 600.00K | 9.34M | 484.20M | 28.99µs (<span style="color: #28a745; font-weight: bold;">0.9996</span>) | 80.97 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.36µs (<span style="color: #28a745; font-weight: bold;">0.9995</span>) |
| bls12_g2msm | 24.00 | 1.14M | 80.25M | 28.85µs (<span style="color: #28a745; font-weight: bold;">0.9960</span>) | 73.03 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.40µs (<span style="color: #28a745; font-weight: bold;">0.9960</span>) |
| sar | 450.00K | 6.11M | 243.24M | 28.77µs (<span style="color: #28a745; font-weight: bold;">0.9998</span>) | 78.36 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.37µs (<span style="color: #28a745; font-weight: bold;">0.9998</span>) |
| bn128_add | 801.00 | 263.28K | 14.61M | 28.52µs (<span style="color: #ffc107; font-weight: bold;">0.7509</span>) | 67.29 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.42µs (<span style="color: #ffc107; font-weight: bold;">0.7509</span>) |
| prevrandao | 900.00K | 5.66M | 232.72M | 27.62µs (<span style="color: #28a745; font-weight: bold;">0.9998</span>) | 78.49 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9998</span>) |
| bls12_g1add | 771.00 | 429.98K | 18.05M | 25.16µs (<span style="color: #28a745; font-weight: bold;">0.9518</span>) | 40.44 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.62µs (<span style="color: #28a745; font-weight: bold;">0.9522</span>) |
| call | 49.50K | 6.16M | 375.52M | 24.72µs (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 70.64 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">1.0000</span>) |
| callcode | 49.50K | 6.16M | 372.60M | 24.46µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) | 70.06 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9998</span>) |
| staticcall | 49.50K | 6.01M | 369.46M | 23.98µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) | 69.81 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.34µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) |
| mstore | 501.00K | 4.21M | 204.25M | 22.14µs (<span style="color: #28a745; font-weight: bold;">0.9961</span>) | 62.67 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9961</span>) |
| address | 900.00K | 5.66M | 195.82M | 21.50µs (<span style="color: #28a745; font-weight: bold;">0.9969</span>) | 57.99 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.37µs (<span style="color: #28a745; font-weight: bold;">0.9969</span>) |
| origin | 1.20M | 7.54M | 260.68M | 21.41µs (<span style="color: #28a745; font-weight: bold;">0.9995</span>) | 57.98 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.37µs (<span style="color: #28a745; font-weight: bold;">0.9996</span>) |
| caller | 1.20M | 7.54M | 260.68M | 21.35µs (<span style="color: #28a745; font-weight: bold;">0.9998</span>) | 57.98 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.37µs (<span style="color: #28a745; font-weight: bold;">0.9997</span>) |
| mul | 600.00K | 9.34M | 423.00M | 21.21µs (<span style="color: #28a745; font-weight: bold;">0.9970</span>) | 60.57 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9970</span>) |
| delegatecall | 49.50K | 6.01M | 315.92M | 20.95µs (<span style="color: #28a745; font-weight: bold;">0.9998</span>) | 59.11 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9998</span>) |
| coinbase | 1.20M | 7.54M | 260.68M | 20.80µs (<span style="color: #28a745; font-weight: bold;">0.9983</span>) | 57.98 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.36µs (<span style="color: #28a745; font-weight: bold;">0.9984</span>) |
| signextend | 1.50M | 23.27M | 738.37M | 20.53µs (<span style="color: #28a745; font-weight: bold;">0.9997</span>) | 55.40 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.37µs (<span style="color: #28a745; font-weight: bold;">0.9997</span>) |
| bls12_g2add | 450.00 | 376.04K | 18.62M | 20.28µs (<span style="color: #28a745; font-weight: bold;">0.9636</span>) | 52.96 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.38µs (<span style="color: #28a745; font-weight: bold;">0.9636</span>) |
| shl | 450.00K | 6.11M | 206.33M | 20.17µs (<span style="color: #28a745; font-weight: bold;">0.9991</span>) | 51.02 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.40µs (<span style="color: #28a745; font-weight: bold;">0.9992</span>) |
| calldataload | 501.00K | 4.70M | 165.98M | 20.03µs (<span style="color: #28a745; font-weight: bold;">0.9970</span>) | 56.34 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.36µs (<span style="color: #28a745; font-weight: bold;">0.9970</span>) |
| mload | 1.00M | 9.38M | 332.79M | 19.82µs (<span style="color: #28a745; font-weight: bold;">0.9983</span>) | 56.68 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9983</span>) |
| bls12_map_fp2_to_g2 | 27.00 | 693.24K | 27.17M | 19.25µs (<span style="color: #28a745; font-weight: bold;">0.9404</span>) | 40.15 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.48µs (<span style="color: #28a745; font-weight: bold;">0.9404</span>) |
| shr | 600.00K | 8.14M | 275.24M | 18.85µs (<span style="color: #28a745; font-weight: bold;">0.9984</span>) | 51.35 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.37µs (<span style="color: #28a745; font-weight: bold;">0.9983</span>) |
| swap16 | 300.00K | 1.18M | 58.45M | 17.95µs (<span style="color: #28a745; font-weight: bold;">0.9957</span>) | 53.37 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.34µs (<span style="color: #28a745; font-weight: bold;">0.9956</span>) |
| push32 | 525.00K | 3.94M | 134.05M | 16.51µs (<span style="color: #28a745; font-weight: bold;">0.9954</span>) | 47.79 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9954</span>) |
| swap1 | 300.00K | 1.10M | 57.28M | 16.31µs (<span style="color: #28a745; font-weight: bold;">0.9913</span>) | 53.35 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.31µs (<span style="color: #28a745; font-weight: bold;">0.9913</span>) |
| swap8 | 300.00K | 1.14M | 57.81M | 15.32µs (<span style="color: #28a745; font-weight: bold;">0.9917</span>) | 53.34 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.29µs (<span style="color: #28a745; font-weight: bold;">0.9917</span>) |
| blobbasefee | 1.20M | 7.54M | 223.48M | 14.61µs (<span style="color: #28a745; font-weight: bold;">0.9997</span>) | 42.48 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.34µs (<span style="color: #28a745; font-weight: bold;">0.9997</span>) |
| gasprice | 1.20M | 7.54M | 218.68M | 14.14µs (<span style="color: #28a745; font-weight: bold;">0.9986</span>) | 40.48 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9986</span>) |
| slt | 2.10M | 28.36M | 1.08B | 13.84µs (<span style="color: #28a745; font-weight: bold;">0.9963</span>) | 37.96 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.36µs (<span style="color: #28a745; font-weight: bold;">0.9963</span>) |
| sgt | 900.00K | 12.18M | 463.62M | 13.68µs (<span style="color: #28a745; font-weight: bold;">0.9896</span>) | 37.97 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.36µs (<span style="color: #28a745; font-weight: bold;">0.9896</span>) |
| sub | 1.50M | 20.27M | 768.67M | 13.48µs (<span style="color: #28a745; font-weight: bold;">0.9927</span>) | 37.30 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.36µs (<span style="color: #28a745; font-weight: bold;">0.9926</span>) |
| callvalue | 1.20M | 7.54M | 215.08M | 13.39µs (<span style="color: #28a745; font-weight: bold;">0.9978</span>) | 38.99 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.34µs (<span style="color: #28a745; font-weight: bold;">0.9978</span>) |
| codecopy | 900.00K | 12.22M | 362.91M | 12.70µs (<span style="color: #28a745; font-weight: bold;">0.9998</span>) | 35.67 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.36µs (<span style="color: #28a745; font-weight: bold;">0.9998</span>) |
| sstore | 61.20K | 6.86M | 245.91M | 12.64µs (<span style="color: #28a745; font-weight: bold;">0.9994</span>) | 36.08 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9995</span>) |
| calldatacopy | 600.00K | 8.16M | 242.35M | 12.43µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) | 35.67 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) |
| pc | 1.20M | 7.54M | 198.28M | 11.71µs (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 31.98 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.37µs (<span style="color: #28a745; font-weight: bold;">1.0000</span>) |
| add | 900.00K | 12.18M | 451.92M | 11.65µs (<span style="color: #28a745; font-weight: bold;">0.9991</span>) | 33.62 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9990</span>) |
| gas | 1.20M | 7.54M | 193.48M | 11.37µs (<span style="color: #28a745; font-weight: bold;">0.9961</span>) | 29.98 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.38µs (<span style="color: #28a745; font-weight: bold;">0.9961</span>) |
| timestamp | 1.20M | 7.54M | 199.48M | 11.32µs (<span style="color: #28a745; font-weight: bold;">0.9987</span>) | 32.48 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9988</span>) |
| basefee | 1.20M | 7.54M | 197.08M | 11.25µs (<span style="color: #28a745; font-weight: bold;">0.9987</span>) | 31.48 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.36µs (<span style="color: #28a745; font-weight: bold;">0.9988</span>) |
| number | 1.20M | 7.54M | 199.48M | 11.22µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) | 32.48 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) |
| calldatasize | 1.20M | 7.54M | 198.28M | 11.12µs (<span style="color: #28a745; font-weight: bold;">0.9972</span>) | 31.99 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9972</span>) |
| push16 | 1.00M | 7.35M | 189.56M | 11.11µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) | 28.37 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.39µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) |
| codesize | 1.20M | 7.54M | 194.68M | 10.76µs (<span style="color: #28a745; font-weight: bold;">0.9992</span>) | 30.48 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9992</span>) |
| chainid | 1.20M | 7.54M | 197.08M | 10.65µs (<span style="color: #28a745; font-weight: bold;">0.9979</span>) | 31.48 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.34µs (<span style="color: #28a745; font-weight: bold;">0.9980</span>) |
| lt | 600.00K | 8.14M | 290.39M | 10.60µs (<span style="color: #28a745; font-weight: bold;">0.9483</span>) | 27.28 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.39µs (<span style="color: #28a745; font-weight: bold;">0.9484</span>) |
| gaslimit | 1.20M | 7.54M | 193.48M | 10.58µs (<span style="color: #28a745; font-weight: bold;">0.9953</span>) | 29.98 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9954</span>) |
| msize | 1.20M | 7.56M | 194.36M | 10.57µs (<span style="color: #28a745; font-weight: bold;">0.9998</span>) | 30.01 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9998</span>) |
| returndatasize | 1.20M | 7.54M | 193.48M | 10.45µs (<span style="color: #28a745; font-weight: bold;">0.9996</span>) | 29.98 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9997</span>) |
| push0 | 1.20M | 7.54M | 189.88M | 10.31µs (<span style="color: #28a745; font-weight: bold;">0.9990</span>) | 28.48 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.36µs (<span style="color: #28a745; font-weight: bold;">0.9990</span>) |
| gt | 600.00K | 8.14M | 290.40M | 9.70µs (<span style="color: #28a745; font-weight: bold;">0.9983</span>) | 27.30 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.36µs (<span style="color: #28a745; font-weight: bold;">0.9984</span>) |
| blobhash | 1.50M | 14.01M | 363.18M | 9.55µs (<span style="color: #28a745; font-weight: bold;">0.9981</span>) | 27.00 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9980</span>) |
| or | 900.00K | 12.18M | 425.82M | 9.07µs (<span style="color: #28a745; font-weight: bold;">0.9728</span>) | 23.96 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.38µs (<span style="color: #28a745; font-weight: bold;">0.9726</span>) |
| mstore8 | 1.00M | 8.38M | 199.33M | 9.02µs (<span style="color: #28a745; font-weight: bold;">0.9980</span>) | 26.00 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9980</span>) |
| byte | 600.00K | 8.14M | 232.05M | 8.86µs (<span style="color: #28a745; font-weight: bold;">0.9964</span>) | 27.36 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.32µs (<span style="color: #28a745; font-weight: bold;">0.9964</span>) |
| xor | 1.50M | 20.27M | 708.67M | 8.63µs (<span style="color: #28a745; font-weight: bold;">0.9994</span>) | 23.95 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.36µs (<span style="color: #28a745; font-weight: bold;">0.9995</span>) |
| jumpdest | 2.40M | 3.03M | 95.30M | 8.55µs (<span style="color: #28a745; font-weight: bold;">0.9944</span>) | 25.84 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.33µs (<span style="color: #28a745; font-weight: bold;">0.9951</span>) |
| returndatacopy | 180.00K | 3.30M | 80.34M | 8.51µs (<span style="color: #28a745; font-weight: bold;">0.9920</span>) | 26.45 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.32µs (<span style="color: #28a745; font-weight: bold;">0.9920</span>) |
| and | 600.00K | 8.14M | 284.40M | 8.11µs (<span style="color: #28a745; font-weight: bold;">0.9966</span>) | 23.96 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.34µs (<span style="color: #28a745; font-weight: bold;">0.9964</span>) |
| iszero | 1.35M | 13.98M | 423.50M | 7.74µs (<span style="color: #28a745; font-weight: bold;">0.9938</span>) | 21.65 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.36µs (<span style="color: #28a745; font-weight: bold;">0.9940</span>) |
| extcodehash | 200.00K | 21.95M | 502.96M | 7.64µs (<span style="color: #28a745; font-weight: bold;">0.9998</span>) | 22.70 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.34µs (<span style="color: #28a745; font-weight: bold;">0.9998</span>) |
| not | 900.00K | 9.34M | 273.83M | 6.40µs (<span style="color: #28a745; font-weight: bold;">0.9948</span>) | 18.32 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9945</span>) |
| dup16 | 600.00K | 4.70M | 102.29M | 6.27µs (<span style="color: #28a745; font-weight: bold;">0.9768</span>) | 17.00 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.37µs (<span style="color: #28a745; font-weight: bold;">0.9768</span>) |
| dup8 | 900.00K | 6.90M | 151.01M | 5.95µs (<span style="color: #28a745; font-weight: bold;">0.9816</span>) | 16.98 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9814</span>) |
| extcodesize | 198.00K | 21.73M | 385.55M | 5.95µs (<span style="color: #28a745; font-weight: bold;">0.9996</span>) | 17.07 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9996</span>) |
| balance | 300.00K | 32.40M | 562.04M | 5.91µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) | 17.01 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) |
| dup1 | 900.00K | 6.80M | 149.41M | 5.89µs (<span style="color: #28a745; font-weight: bold;">0.9909</span>) | 16.98 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9904</span>) |
| tstore | 198.00K | 21.13M | 392.16M | 5.82µs (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 16.93 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.34µs (<span style="color: #28a745; font-weight: bold;">1.0000</span>) |
| extcodecopy | 102.00K | 14.28M | 246.94M | 5.54µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) | 16.06 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.34µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) |
| pop | 1.80M | 7.68M | 181.68M | 5.47µs (<span style="color: #28a745; font-weight: bold;">0.9867</span>) | 15.02 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.36µs (<span style="color: #28a745; font-weight: bold;">0.9865</span>) |
| sload | 198.00K | 21.73M | 354.21M | 5.39µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) | 15.53 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) |
| mcopy | 60.00K | 6.60M | 101.57M | 5.20µs (<span style="color: #28a745; font-weight: bold;">0.9974</span>) | 14.92 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9973</span>) |
| push1 | 1.20M | 8.74M | 176.69M | 5.19µs (<span style="color: #28a745; font-weight: bold;">0.9935</span>) | 15.34 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.34µs (<span style="color: #28a745; font-weight: bold;">0.9935</span>) |
| jumpi | 1.00M | 17.86M | 291.08M | 4.52µs (<span style="color: #28a745; font-weight: bold;">0.9978</span>) | 12.30 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.37µs (<span style="color: #28a745; font-weight: bold;">0.9978</span>) |
| ripemd160 | 1.50K | 6.90M | 73.80M | 3.15µs (<span style="color: #28a745; font-weight: bold;">0.9982</span>) | 9.38 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.34µs (<span style="color: #28a745; font-weight: bold;">0.9982</span>) |
| jump | 600.00K | 7.68M | 110.88M | 2.74µs (<span style="color: #28a745; font-weight: bold;">0.9976</span>) | 7.88 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9973</span>) |
| identity | 1.50M | 259.80M | 10.56B | 1.87µs (<span style="color: #28a745; font-weight: bold;">0.9274</span>) | 1.70 (<span style="color: #28a745; font-weight: bold;">0.9998</span>) | 1.10µs (<span style="color: #28a745; font-weight: bold;">0.9206</span>) |
| sha256 | 1.50K | 2.68M | 19.67M | 0.94µs (<span style="color: #ffc107; font-weight: bold;">0.8017</span>) | 3.41 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.28µs (<span style="color: #ffc107; font-weight: bold;">0.8015</span>) |
| tload | 1.50M | 159.77M | 621.48M | 0.86µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) | 2.43 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.36µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) |
| create2 | 4.20K | 134.68M | 189.22M | 0.56µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) | 1.37 (<span style="color: #28a745; font-weight: bold;">0.9997</span>) | 0.41µs (<span style="color: #28a745; font-weight: bold;">0.9995</span>) |
| create | 4.20K | 134.56M | 177.51M | 0.49µs (<span style="color: #28a745; font-weight: bold;">0.9998</span>) | 1.29 (<span style="color: #28a745; font-weight: bold;">0.9997</span>) | 0.38µs (<span style="color: #28a745; font-weight: bold;">0.9994</span>) |
| log0 | 120.00K | 45.96M | 371.63M | 0.30µs (<span style="color: #28a745; font-weight: bold;">0.9807</span>) | 7.77 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.04µs (<span style="color: #28a745; font-weight: bold;">0.9815</span>) |
| log1 | 81.00K | 82.44M | 571.66M | 0.27µs (<span style="color: #28a745; font-weight: bold;">0.9959</span>) | 6.76 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.04µs (<span style="color: #28a745; font-weight: bold;">0.9956</span>) |
| log3 | 35.70K | 127.10M | 644.16M | 0.16µs (<span style="color: #28a745; font-weight: bold;">0.9874</span>) | 4.98 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.03µs (<span style="color: #28a745; font-weight: bold;">0.9882</span>) |
| log2 | 42.00K | 133.62M | 635.44M | 0.15µs (<span style="color: #28a745; font-weight: bold;">0.9985</span>) | 4.67 (<span style="color: #28a745; font-weight: bold;">0.9999</span>) | 0.03µs (<span style="color: #28a745; font-weight: bold;">0.9988</span>) |
| log4 | 27.30K | 107.56M | 576.67M | 0.15µs (<span style="color: #28a745; font-weight: bold;">0.9983</span>) | 5.25 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.03µs (<span style="color: #28a745; font-weight: bold;">0.9982</span>) |

## Proving time vs ZK Cycles

This section examines whether ZK cycles are a good proxy for proving time.

### Time/Cycles Bar Chart (R² ≥ 0.7)

*Only opcodes/precompiles with R² ≥ 0.7 are shown.*

![Time/Cycles Bar Chart](/marginal-gas-benchmark/risc0/plots/bar_time_per_cycle.png)

### Combined ZK Cycles ↔ Proving Time (All Opcodes)

![Combined ZK Cycles vs Proving Time](/marginal-gas-benchmark/risc0/plots/combined_zkcycles_proving.png)

### Excluding: log0, log1, log2, log3, log4

![Regular Opcodes](/marginal-gas-benchmark/risc0/plots/combined_zkcycles_regular.png)

### Excluding: blake2f, log0, log1, log2, log3, log4, modexp

![Regular Opcodes No Outliers](/marginal-gas-benchmark/risc0/plots/combined_zkcycles_regular_no_outliers.png)

### Excluding: blake2f, keccak256, log0, log0, log1, log1, log2, log2, log3, log3, log4, log4, modexp

![Regular Opcodes Minimal](/marginal-gas-benchmark/risc0/plots/combined_zkcycles_regular_minimal.png)

### Only: log0, log1, log2, log3, log4

![BLS12 Precompiles](/marginal-gas-benchmark/risc0/plots/combined_zkcycles_bls12.png)


## Regression Charts

### risc0 Gas vs Proving Time

**modexp**: Slope = 1.36ms/gas, R² = 0.9998

![modexp](/marginal-gas-benchmark/risc0/plots/gas_proving_modexp.png)

**blake2f**: Slope = 242.84µs/gas, R² = 1.0000

![blake2f](/marginal-gas-benchmark/risc0/plots/gas_proving_blake2f.png)

**mulmod**: Slope = 151.85µs/gas, R² = 1.0000

![mulmod](/marginal-gas-benchmark/risc0/plots/gas_proving_mulmod.png)

**div**: Slope = 110.29µs/gas, R² = 0.9999

![div](/marginal-gas-benchmark/risc0/plots/gas_proving_div.png)

**mod**: Slope = 94.55µs/gas, R² = 1.0000

![mod](/marginal-gas-benchmark/risc0/plots/gas_proving_mod.png)

**sdiv**: Slope = 92.84µs/gas, R² = 1.0000

![sdiv](/marginal-gas-benchmark/risc0/plots/gas_proving_sdiv.png)

**bn128_mul**: Slope = 86.68µs/gas, R² = 0.9997

![bn128_mul](/marginal-gas-benchmark/risc0/plots/gas_proving_bn128_mul.png)

**point_evaluation**: Slope = 85.54µs/gas, R² = 0.9945

![point_evaluation](/marginal-gas-benchmark/risc0/plots/gas_proving_point_evaluation.png)

**selfbalance**: Slope = 84.67µs/gas, R² = 0.9999

![selfbalance](/marginal-gas-benchmark/risc0/plots/gas_proving_selfbalance.png)

**keccak256**: Slope = 84.62µs/gas, R² = 0.9980

![keccak256](/marginal-gas-benchmark/risc0/plots/gas_proving_keccak256.png)

**bn128_pairing**: Slope = 81.72µs/gas, R² = 0.9968

![bn128_pairing](/marginal-gas-benchmark/risc0/plots/gas_proving_bn128_pairing.png)

**ecrecover**: Slope = 81.32µs/gas, R² = 0.9921

![ecrecover](/marginal-gas-benchmark/risc0/plots/gas_proving_ecrecover.png)

**addmod**: Slope = 71.00µs/gas, R² = 0.9999

![addmod](/marginal-gas-benchmark/risc0/plots/gas_proving_addmod.png)

**bls12_g1msm**: Slope = 37.74µs/gas, R² = 0.9893

![bls12_g1msm](/marginal-gas-benchmark/risc0/plots/gas_proving_bls12_g1msm.png)

**bls12_pairing**: Slope = 36.15µs/gas, R² = 0.9979

![bls12_pairing](/marginal-gas-benchmark/risc0/plots/gas_proving_bls12_pairing.png)

**eq**: Slope = 34.89µs/gas, R² = 0.9993

![eq](/marginal-gas-benchmark/risc0/plots/gas_proving_eq.png)

**exp**: Slope = 32.61µs/gas, R² = 0.9997

![exp](/marginal-gas-benchmark/risc0/plots/gas_proving_exp.png)

**bls12_map_fp_to_g1**: Slope = 31.51µs/gas, R² = 0.9772

![bls12_map_fp_to_g1](/marginal-gas-benchmark/risc0/plots/gas_proving_bls12_map_fp_to_g1.png)

**smod**: Slope = 28.99µs/gas, R² = 0.9996

![smod](/marginal-gas-benchmark/risc0/plots/gas_proving_smod.png)

**bls12_g2msm**: Slope = 28.85µs/gas, R² = 0.9960

![bls12_g2msm](/marginal-gas-benchmark/risc0/plots/gas_proving_bls12_g2msm.png)

**sar**: Slope = 28.77µs/gas, R² = 0.9998

![sar](/marginal-gas-benchmark/risc0/plots/gas_proving_sar.png)

**bn128_add**: Slope = 28.52µs/gas, R² = 0.7509

![bn128_add](/marginal-gas-benchmark/risc0/plots/gas_proving_bn128_add.png)

**prevrandao**: Slope = 27.62µs/gas, R² = 0.9998

![prevrandao](/marginal-gas-benchmark/risc0/plots/gas_proving_prevrandao.png)

**bls12_g1add**: Slope = 25.16µs/gas, R² = 0.9518

![bls12_g1add](/marginal-gas-benchmark/risc0/plots/gas_proving_bls12_g1add.png)

**call**: Slope = 24.72µs/gas, R² = 1.0000

![call](/marginal-gas-benchmark/risc0/plots/gas_proving_call.png)

**callcode**: Slope = 24.46µs/gas, R² = 0.9999

![callcode](/marginal-gas-benchmark/risc0/plots/gas_proving_callcode.png)

**staticcall**: Slope = 23.98µs/gas, R² = 0.9999

![staticcall](/marginal-gas-benchmark/risc0/plots/gas_proving_staticcall.png)

**mstore**: Slope = 22.14µs/gas, R² = 0.9961

![mstore](/marginal-gas-benchmark/risc0/plots/gas_proving_mstore.png)

**address**: Slope = 21.50µs/gas, R² = 0.9969

![address](/marginal-gas-benchmark/risc0/plots/gas_proving_address.png)

**origin**: Slope = 21.41µs/gas, R² = 0.9995

![origin](/marginal-gas-benchmark/risc0/plots/gas_proving_origin.png)

**caller**: Slope = 21.35µs/gas, R² = 0.9998

![caller](/marginal-gas-benchmark/risc0/plots/gas_proving_caller.png)

**mul**: Slope = 21.21µs/gas, R² = 0.9970

![mul](/marginal-gas-benchmark/risc0/plots/gas_proving_mul.png)

**delegatecall**: Slope = 20.95µs/gas, R² = 0.9998

![delegatecall](/marginal-gas-benchmark/risc0/plots/gas_proving_delegatecall.png)

**coinbase**: Slope = 20.80µs/gas, R² = 0.9983

![coinbase](/marginal-gas-benchmark/risc0/plots/gas_proving_coinbase.png)

**signextend**: Slope = 20.53µs/gas, R² = 0.9997

![signextend](/marginal-gas-benchmark/risc0/plots/gas_proving_signextend.png)

**bls12_g2add**: Slope = 20.28µs/gas, R² = 0.9636

![bls12_g2add](/marginal-gas-benchmark/risc0/plots/gas_proving_bls12_g2add.png)

**shl**: Slope = 20.17µs/gas, R² = 0.9991

![shl](/marginal-gas-benchmark/risc0/plots/gas_proving_shl.png)

**calldataload**: Slope = 20.03µs/gas, R² = 0.9970

![calldataload](/marginal-gas-benchmark/risc0/plots/gas_proving_calldataload.png)

**mload**: Slope = 19.82µs/gas, R² = 0.9983

![mload](/marginal-gas-benchmark/risc0/plots/gas_proving_mload.png)

**bls12_map_fp2_to_g2**: Slope = 19.25µs/gas, R² = 0.9404

![bls12_map_fp2_to_g2](/marginal-gas-benchmark/risc0/plots/gas_proving_bls12_map_fp2_to_g2.png)

**shr**: Slope = 18.85µs/gas, R² = 0.9984

![shr](/marginal-gas-benchmark/risc0/plots/gas_proving_shr.png)

**swap16**: Slope = 17.95µs/gas, R² = 0.9957

![swap16](/marginal-gas-benchmark/risc0/plots/gas_proving_swap16.png)

**push32**: Slope = 16.51µs/gas, R² = 0.9954

![push32](/marginal-gas-benchmark/risc0/plots/gas_proving_push32.png)

**swap1**: Slope = 16.31µs/gas, R² = 0.9913

![swap1](/marginal-gas-benchmark/risc0/plots/gas_proving_swap1.png)

**swap8**: Slope = 15.32µs/gas, R² = 0.9917

![swap8](/marginal-gas-benchmark/risc0/plots/gas_proving_swap8.png)

**blobbasefee**: Slope = 14.61µs/gas, R² = 0.9997

![blobbasefee](/marginal-gas-benchmark/risc0/plots/gas_proving_blobbasefee.png)

**gasprice**: Slope = 14.14µs/gas, R² = 0.9986

![gasprice](/marginal-gas-benchmark/risc0/plots/gas_proving_gasprice.png)

**slt**: Slope = 13.84µs/gas, R² = 0.9963

![slt](/marginal-gas-benchmark/risc0/plots/gas_proving_slt.png)

**sgt**: Slope = 13.68µs/gas, R² = 0.9896

![sgt](/marginal-gas-benchmark/risc0/plots/gas_proving_sgt.png)

**sub**: Slope = 13.48µs/gas, R² = 0.9927

![sub](/marginal-gas-benchmark/risc0/plots/gas_proving_sub.png)

**callvalue**: Slope = 13.39µs/gas, R² = 0.9978

![callvalue](/marginal-gas-benchmark/risc0/plots/gas_proving_callvalue.png)

**codecopy**: Slope = 12.70µs/gas, R² = 0.9998

![codecopy](/marginal-gas-benchmark/risc0/plots/gas_proving_codecopy.png)

**sstore**: Slope = 12.64µs/gas, R² = 0.9994

![sstore](/marginal-gas-benchmark/risc0/plots/gas_proving_sstore.png)

**calldatacopy**: Slope = 12.43µs/gas, R² = 0.9999

![calldatacopy](/marginal-gas-benchmark/risc0/plots/gas_proving_calldatacopy.png)

**pc**: Slope = 11.71µs/gas, R² = 1.0000

![pc](/marginal-gas-benchmark/risc0/plots/gas_proving_pc.png)

**add**: Slope = 11.65µs/gas, R² = 0.9991

![add](/marginal-gas-benchmark/risc0/plots/gas_proving_add.png)

**gas**: Slope = 11.37µs/gas, R² = 0.9961

![gas](/marginal-gas-benchmark/risc0/plots/gas_proving_gas.png)

**timestamp**: Slope = 11.32µs/gas, R² = 0.9987

![timestamp](/marginal-gas-benchmark/risc0/plots/gas_proving_timestamp.png)

**basefee**: Slope = 11.25µs/gas, R² = 0.9987

![basefee](/marginal-gas-benchmark/risc0/plots/gas_proving_basefee.png)

**number**: Slope = 11.22µs/gas, R² = 0.9999

![number](/marginal-gas-benchmark/risc0/plots/gas_proving_number.png)

**calldatasize**: Slope = 11.12µs/gas, R² = 0.9972

![calldatasize](/marginal-gas-benchmark/risc0/plots/gas_proving_calldatasize.png)

**push16**: Slope = 11.11µs/gas, R² = 0.9999

![push16](/marginal-gas-benchmark/risc0/plots/gas_proving_push16.png)

**codesize**: Slope = 10.76µs/gas, R² = 0.9992

![codesize](/marginal-gas-benchmark/risc0/plots/gas_proving_codesize.png)

**chainid**: Slope = 10.65µs/gas, R² = 0.9979

![chainid](/marginal-gas-benchmark/risc0/plots/gas_proving_chainid.png)

**lt**: Slope = 10.60µs/gas, R² = 0.9483

![lt](/marginal-gas-benchmark/risc0/plots/gas_proving_lt.png)

**gaslimit**: Slope = 10.58µs/gas, R² = 0.9953

![gaslimit](/marginal-gas-benchmark/risc0/plots/gas_proving_gaslimit.png)

**msize**: Slope = 10.57µs/gas, R² = 0.9998

![msize](/marginal-gas-benchmark/risc0/plots/gas_proving_msize.png)

**returndatasize**: Slope = 10.45µs/gas, R² = 0.9996

![returndatasize](/marginal-gas-benchmark/risc0/plots/gas_proving_returndatasize.png)

**push0**: Slope = 10.31µs/gas, R² = 0.9990

![push0](/marginal-gas-benchmark/risc0/plots/gas_proving_push0.png)

**gt**: Slope = 9.70µs/gas, R² = 0.9983

![gt](/marginal-gas-benchmark/risc0/plots/gas_proving_gt.png)

**blobhash**: Slope = 9.55µs/gas, R² = 0.9981

![blobhash](/marginal-gas-benchmark/risc0/plots/gas_proving_blobhash.png)

**or**: Slope = 9.07µs/gas, R² = 0.9728

![or](/marginal-gas-benchmark/risc0/plots/gas_proving_or.png)

**mstore8**: Slope = 9.02µs/gas, R² = 0.9980

![mstore8](/marginal-gas-benchmark/risc0/plots/gas_proving_mstore8.png)

**byte**: Slope = 8.86µs/gas, R² = 0.9964

![byte](/marginal-gas-benchmark/risc0/plots/gas_proving_byte.png)

**xor**: Slope = 8.63µs/gas, R² = 0.9994

![xor](/marginal-gas-benchmark/risc0/plots/gas_proving_xor.png)

**jumpdest**: Slope = 8.55µs/gas, R² = 0.9944

![jumpdest](/marginal-gas-benchmark/risc0/plots/gas_proving_jumpdest.png)

**returndatacopy**: Slope = 8.51µs/gas, R² = 0.9920

![returndatacopy](/marginal-gas-benchmark/risc0/plots/gas_proving_returndatacopy.png)

**and**: Slope = 8.11µs/gas, R² = 0.9966

![and](/marginal-gas-benchmark/risc0/plots/gas_proving_and.png)

**iszero**: Slope = 7.74µs/gas, R² = 0.9938

![iszero](/marginal-gas-benchmark/risc0/plots/gas_proving_iszero.png)

**extcodehash**: Slope = 7.64µs/gas, R² = 0.9998

![extcodehash](/marginal-gas-benchmark/risc0/plots/gas_proving_extcodehash.png)

**not**: Slope = 6.40µs/gas, R² = 0.9948

![not](/marginal-gas-benchmark/risc0/plots/gas_proving_not.png)

**dup16**: Slope = 6.27µs/gas, R² = 0.9768

![dup16](/marginal-gas-benchmark/risc0/plots/gas_proving_dup16.png)

**dup8**: Slope = 5.95µs/gas, R² = 0.9816

![dup8](/marginal-gas-benchmark/risc0/plots/gas_proving_dup8.png)

**extcodesize**: Slope = 5.95µs/gas, R² = 0.9996

![extcodesize](/marginal-gas-benchmark/risc0/plots/gas_proving_extcodesize.png)

**balance**: Slope = 5.91µs/gas, R² = 0.9999

![balance](/marginal-gas-benchmark/risc0/plots/gas_proving_balance.png)

**dup1**: Slope = 5.89µs/gas, R² = 0.9909

![dup1](/marginal-gas-benchmark/risc0/plots/gas_proving_dup1.png)

**tstore**: Slope = 5.82µs/gas, R² = 1.0000

![tstore](/marginal-gas-benchmark/risc0/plots/gas_proving_tstore.png)

**extcodecopy**: Slope = 5.54µs/gas, R² = 0.9999

![extcodecopy](/marginal-gas-benchmark/risc0/plots/gas_proving_extcodecopy.png)

**pop**: Slope = 5.47µs/gas, R² = 0.9867

![pop](/marginal-gas-benchmark/risc0/plots/gas_proving_pop.png)

**sload**: Slope = 5.39µs/gas, R² = 0.9999

![sload](/marginal-gas-benchmark/risc0/plots/gas_proving_sload.png)

**mcopy**: Slope = 5.20µs/gas, R² = 0.9974

![mcopy](/marginal-gas-benchmark/risc0/plots/gas_proving_mcopy.png)

**push1**: Slope = 5.19µs/gas, R² = 0.9935

![push1](/marginal-gas-benchmark/risc0/plots/gas_proving_push1.png)

**jumpi**: Slope = 4.52µs/gas, R² = 0.9978

![jumpi](/marginal-gas-benchmark/risc0/plots/gas_proving_jumpi.png)

**ripemd160**: Slope = 3.15µs/gas, R² = 0.9982

![ripemd160](/marginal-gas-benchmark/risc0/plots/gas_proving_ripemd160.png)

**jump**: Slope = 2.74µs/gas, R² = 0.9976

![jump](/marginal-gas-benchmark/risc0/plots/gas_proving_jump.png)

**identity**: Slope = 1.87µs/gas, R² = 0.9274

![identity](/marginal-gas-benchmark/risc0/plots/gas_proving_identity.png)

**sha256**: Slope = 0.94µs/gas, R² = 0.8017

![sha256](/marginal-gas-benchmark/risc0/plots/gas_proving_sha256.png)

**tload**: Slope = 0.86µs/gas, R² = 0.9999

![tload](/marginal-gas-benchmark/risc0/plots/gas_proving_tload.png)

**create2**: Slope = 0.56µs/gas, R² = 0.9999

![create2](/marginal-gas-benchmark/risc0/plots/gas_proving_create2.png)

**create**: Slope = 0.49µs/gas, R² = 0.9998

![create](/marginal-gas-benchmark/risc0/plots/gas_proving_create.png)

**log0**: Slope = 0.30µs/gas, R² = 0.9807

![log0](/marginal-gas-benchmark/risc0/plots/gas_proving_log0.png)

**log1**: Slope = 0.27µs/gas, R² = 0.9959

![log1](/marginal-gas-benchmark/risc0/plots/gas_proving_log1.png)

**log3**: Slope = 0.16µs/gas, R² = 0.9874

![log3](/marginal-gas-benchmark/risc0/plots/gas_proving_log3.png)

**log2**: Slope = 0.15µs/gas, R² = 0.9985

![log2](/marginal-gas-benchmark/risc0/plots/gas_proving_log2.png)

**log4**: Slope = 0.15µs/gas, R² = 0.9983

![log4](/marginal-gas-benchmark/risc0/plots/gas_proving_log4.png)

### risc0 Gas vs ZK Cycles

**modexp**: Slope = 3.91K cycles/gas, R² = 1.0000

![modexp](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_modexp.png)

**blake2f**: Slope = 702.03 cycles/gas, R² = 1.0000

![blake2f](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_blake2f.png)

**mulmod**: Slope = 420.63 cycles/gas, R² = 1.0000

![mulmod](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_mulmod.png)

**div**: Slope = 306.78 cycles/gas, R² = 1.0000

![div](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_div.png)

**mod**: Slope = 257.97 cycles/gas, R² = 1.0000

![mod](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_mod.png)

**sdiv**: Slope = 257.18 cycles/gas, R² = 1.0000

![sdiv](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_sdiv.png)

**selfbalance**: Slope = 241.80 cycles/gas, R² = 1.0000

![selfbalance](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_selfbalance.png)

**bn128_mul**: Slope = 229.21 cycles/gas, R² = 1.0000

![bn128_mul](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_bn128_mul.png)

**bn128_pairing**: Slope = 225.96 cycles/gas, R² = 1.0000

![bn128_pairing](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_bn128_pairing.png)

**point_evaluation**: Slope = 223.09 cycles/gas, R² = 1.0000

![point_evaluation](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_point_evaluation.png)

**addmod**: Slope = 196.25 cycles/gas, R² = 1.0000

![addmod](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_addmod.png)

**ecrecover**: Slope = 193.83 cycles/gas, R² = 1.0000

![ecrecover](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_ecrecover.png)

**eq**: Slope = 101.29 cycles/gas, R² = 1.0000

![eq](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_eq.png)

**exp**: Slope = 92.99 cycles/gas, R² = 1.0000

![exp](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_exp.png)

**bls12_g1msm**: Slope = 89.91 cycles/gas, R² = 1.0000

![bls12_g1msm](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_bls12_g1msm.png)

**bls12_pairing**: Slope = 88.19 cycles/gas, R² = 1.0000

![bls12_pairing](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_bls12_pairing.png)

**smod**: Slope = 80.97 cycles/gas, R² = 1.0000

![smod](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_smod.png)

**keccak256**: Slope = 80.73 cycles/gas, R² = 1.0000

![keccak256](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_keccak256.png)

**prevrandao**: Slope = 78.49 cycles/gas, R² = 1.0000

![prevrandao](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_prevrandao.png)

**sar**: Slope = 78.36 cycles/gas, R² = 1.0000

![sar](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_sar.png)

**bls12_g2msm**: Slope = 73.03 cycles/gas, R² = 1.0000

![bls12_g2msm](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_bls12_g2msm.png)

**call**: Slope = 70.64 cycles/gas, R² = 1.0000

![call](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_call.png)

**callcode**: Slope = 70.06 cycles/gas, R² = 1.0000

![callcode](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_callcode.png)

**staticcall**: Slope = 69.81 cycles/gas, R² = 1.0000

![staticcall](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_staticcall.png)

**bn128_add**: Slope = 67.29 cycles/gas, R² = 1.0000

![bn128_add](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_bn128_add.png)

**mstore**: Slope = 62.67 cycles/gas, R² = 1.0000

![mstore](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_mstore.png)

**mul**: Slope = 60.57 cycles/gas, R² = 1.0000

![mul](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_mul.png)

**delegatecall**: Slope = 59.11 cycles/gas, R² = 1.0000

![delegatecall](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_delegatecall.png)

**address**: Slope = 57.99 cycles/gas, R² = 1.0000

![address](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_address.png)

**origin**: Slope = 57.98 cycles/gas, R² = 1.0000

![origin](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_origin.png)

**coinbase**: Slope = 57.98 cycles/gas, R² = 1.0000

![coinbase](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_coinbase.png)

**caller**: Slope = 57.98 cycles/gas, R² = 1.0000

![caller](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_caller.png)

**mload**: Slope = 56.68 cycles/gas, R² = 1.0000

![mload](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_mload.png)

**calldataload**: Slope = 56.34 cycles/gas, R² = 1.0000

![calldataload](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_calldataload.png)

**bls12_map_fp_to_g1**: Slope = 55.78 cycles/gas, R² = 1.0000

![bls12_map_fp_to_g1](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_bls12_map_fp_to_g1.png)

**signextend**: Slope = 55.40 cycles/gas, R² = 1.0000

![signextend](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_signextend.png)

**swap16**: Slope = 53.37 cycles/gas, R² = 1.0000

![swap16](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_swap16.png)

**swap1**: Slope = 53.35 cycles/gas, R² = 1.0000

![swap1](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_swap1.png)

**swap8**: Slope = 53.34 cycles/gas, R² = 1.0000

![swap8](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_swap8.png)

**bls12_g2add**: Slope = 52.96 cycles/gas, R² = 1.0000

![bls12_g2add](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_bls12_g2add.png)

**shr**: Slope = 51.35 cycles/gas, R² = 1.0000

![shr](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_shr.png)

**shl**: Slope = 51.02 cycles/gas, R² = 1.0000

![shl](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_shl.png)

**push32**: Slope = 47.79 cycles/gas, R² = 1.0000

![push32](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_push32.png)

**blobbasefee**: Slope = 42.48 cycles/gas, R² = 1.0000

![blobbasefee](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_blobbasefee.png)

**gasprice**: Slope = 40.48 cycles/gas, R² = 1.0000

![gasprice](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_gasprice.png)

**bls12_g1add**: Slope = 40.44 cycles/gas, R² = 1.0000

![bls12_g1add](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_bls12_g1add.png)

**bls12_map_fp2_to_g2**: Slope = 40.15 cycles/gas, R² = 1.0000

![bls12_map_fp2_to_g2](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_bls12_map_fp2_to_g2.png)

**callvalue**: Slope = 38.99 cycles/gas, R² = 1.0000

![callvalue](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_callvalue.png)

**sgt**: Slope = 37.97 cycles/gas, R² = 1.0000

![sgt](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_sgt.png)

**slt**: Slope = 37.96 cycles/gas, R² = 1.0000

![slt](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_slt.png)

**sub**: Slope = 37.30 cycles/gas, R² = 1.0000

![sub](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_sub.png)

**sstore**: Slope = 36.08 cycles/gas, R² = 1.0000

![sstore](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_sstore.png)

**calldatacopy**: Slope = 35.67 cycles/gas, R² = 1.0000

![calldatacopy](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_calldatacopy.png)

**codecopy**: Slope = 35.67 cycles/gas, R² = 1.0000

![codecopy](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_codecopy.png)

**add**: Slope = 33.62 cycles/gas, R² = 1.0000

![add](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_add.png)

**timestamp**: Slope = 32.48 cycles/gas, R² = 1.0000

![timestamp](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_timestamp.png)

**number**: Slope = 32.48 cycles/gas, R² = 1.0000

![number](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_number.png)

**calldatasize**: Slope = 31.99 cycles/gas, R² = 1.0000

![calldatasize](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_calldatasize.png)

**pc**: Slope = 31.98 cycles/gas, R² = 1.0000

![pc](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_pc.png)

**basefee**: Slope = 31.48 cycles/gas, R² = 1.0000

![basefee](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_basefee.png)

**chainid**: Slope = 31.48 cycles/gas, R² = 1.0000

![chainid](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_chainid.png)

**codesize**: Slope = 30.48 cycles/gas, R² = 1.0000

![codesize](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_codesize.png)

**msize**: Slope = 30.01 cycles/gas, R² = 1.0000

![msize](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_msize.png)

**gas**: Slope = 29.98 cycles/gas, R² = 1.0000

![gas](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_gas.png)

**gaslimit**: Slope = 29.98 cycles/gas, R² = 1.0000

![gaslimit](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_gaslimit.png)

**returndatasize**: Slope = 29.98 cycles/gas, R² = 1.0000

![returndatasize](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_returndatasize.png)

**push0**: Slope = 28.48 cycles/gas, R² = 1.0000

![push0](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_push0.png)

**push16**: Slope = 28.37 cycles/gas, R² = 1.0000

![push16](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_push16.png)

**byte**: Slope = 27.36 cycles/gas, R² = 1.0000

![byte](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_byte.png)

**gt**: Slope = 27.30 cycles/gas, R² = 1.0000

![gt](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_gt.png)

**lt**: Slope = 27.28 cycles/gas, R² = 1.0000

![lt](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_lt.png)

**blobhash**: Slope = 27.00 cycles/gas, R² = 1.0000

![blobhash](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_blobhash.png)

**returndatacopy**: Slope = 26.45 cycles/gas, R² = 1.0000

![returndatacopy](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_returndatacopy.png)

**mstore8**: Slope = 26.00 cycles/gas, R² = 1.0000

![mstore8](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_mstore8.png)

**jumpdest**: Slope = 25.84 cycles/gas, R² = 1.0000

![jumpdest](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_jumpdest.png)

**or**: Slope = 23.96 cycles/gas, R² = 1.0000

![or](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_or.png)

**and**: Slope = 23.96 cycles/gas, R² = 1.0000

![and](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_and.png)

**xor**: Slope = 23.95 cycles/gas, R² = 1.0000

![xor](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_xor.png)

**extcodehash**: Slope = 22.70 cycles/gas, R² = 1.0000

![extcodehash](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_extcodehash.png)

**iszero**: Slope = 21.65 cycles/gas, R² = 1.0000

![iszero](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_iszero.png)

**not**: Slope = 18.32 cycles/gas, R² = 1.0000

![not](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_not.png)

**extcodesize**: Slope = 17.07 cycles/gas, R² = 1.0000

![extcodesize](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_extcodesize.png)

**balance**: Slope = 17.01 cycles/gas, R² = 1.0000

![balance](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_balance.png)

**dup16**: Slope = 17.00 cycles/gas, R² = 1.0000

![dup16](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_dup16.png)

**dup1**: Slope = 16.98 cycles/gas, R² = 1.0000

![dup1](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_dup1.png)

**dup8**: Slope = 16.98 cycles/gas, R² = 1.0000

![dup8](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_dup8.png)

**tstore**: Slope = 16.93 cycles/gas, R² = 1.0000

![tstore](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_tstore.png)

**extcodecopy**: Slope = 16.06 cycles/gas, R² = 1.0000

![extcodecopy](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_extcodecopy.png)

**sload**: Slope = 15.53 cycles/gas, R² = 1.0000

![sload](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_sload.png)

**push1**: Slope = 15.34 cycles/gas, R² = 1.0000

![push1](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_push1.png)

**pop**: Slope = 15.02 cycles/gas, R² = 1.0000

![pop](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_pop.png)

**mcopy**: Slope = 14.92 cycles/gas, R² = 1.0000

![mcopy](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_mcopy.png)

**jumpi**: Slope = 12.30 cycles/gas, R² = 1.0000

![jumpi](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_jumpi.png)

**ripemd160**: Slope = 9.38 cycles/gas, R² = 1.0000

![ripemd160](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_ripemd160.png)

**jump**: Slope = 7.88 cycles/gas, R² = 1.0000

![jump](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_jump.png)

**log0**: Slope = 7.77 cycles/gas, R² = 1.0000

![log0](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_log0.png)

**log1**: Slope = 6.76 cycles/gas, R² = 1.0000

![log1](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_log1.png)

**log4**: Slope = 5.25 cycles/gas, R² = 1.0000

![log4](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_log4.png)

**log3**: Slope = 4.98 cycles/gas, R² = 1.0000

![log3](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_log3.png)

**log2**: Slope = 4.67 cycles/gas, R² = 0.9999

![log2](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_log2.png)

**sha256**: Slope = 3.41 cycles/gas, R² = 1.0000

![sha256](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_sha256.png)

**tload**: Slope = 2.43 cycles/gas, R² = 1.0000

![tload](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_tload.png)

**identity**: Slope = 1.70 cycles/gas, R² = 0.9998

![identity](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_identity.png)

**create2**: Slope = 1.37 cycles/gas, R² = 0.9997

![create2](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_create2.png)

**create**: Slope = 1.29 cycles/gas, R² = 0.9997

![create](/marginal-gas-benchmark/risc0/plots/gas_zkcycles_create.png)

### risc0 ZK Cycles vs Proving Time

**identity**: Slope = 1.10µs/cycle, R² = 0.9206

![identity](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_identity.png)

**keccak256**: Slope = 1.05µs/cycle, R² = 0.9980

![keccak256](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_keccak256.png)

**bls12_g1add**: Slope = 0.62µs/cycle, R² = 0.9522

![bls12_g1add](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_bls12_g1add.png)

**bls12_map_fp_to_g1**: Slope = 0.56µs/cycle, R² = 0.9773

![bls12_map_fp_to_g1](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_bls12_map_fp_to_g1.png)

**bls12_map_fp2_to_g2**: Slope = 0.48µs/cycle, R² = 0.9404

![bls12_map_fp2_to_g2](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_bls12_map_fp2_to_g2.png)

**bn128_add**: Slope = 0.42µs/cycle, R² = 0.7509

![bn128_add](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_bn128_add.png)

**bls12_g1msm**: Slope = 0.42µs/cycle, R² = 0.9893

![bls12_g1msm](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_bls12_g1msm.png)

**ecrecover**: Slope = 0.42µs/cycle, R² = 0.9921

![ecrecover](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_ecrecover.png)

**create2**: Slope = 0.41µs/cycle, R² = 0.9995

![create2](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_create2.png)

**bls12_pairing**: Slope = 0.41µs/cycle, R² = 0.9979

![bls12_pairing](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_bls12_pairing.png)

**shl**: Slope = 0.40µs/cycle, R² = 0.9992

![shl](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_shl.png)

**bls12_g2msm**: Slope = 0.40µs/cycle, R² = 0.9960

![bls12_g2msm](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_bls12_g2msm.png)

**push16**: Slope = 0.39µs/cycle, R² = 0.9999

![push16](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_push16.png)

**lt**: Slope = 0.39µs/cycle, R² = 0.9484

![lt](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_lt.png)

**point_evaluation**: Slope = 0.38µs/cycle, R² = 0.9945

![point_evaluation](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_point_evaluation.png)

**bls12_g2add**: Slope = 0.38µs/cycle, R² = 0.9636

![bls12_g2add](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_bls12_g2add.png)

**create**: Slope = 0.38µs/cycle, R² = 0.9994

![create](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_create.png)

**gas**: Slope = 0.38µs/cycle, R² = 0.9961

![gas](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_gas.png)

**or**: Slope = 0.38µs/cycle, R² = 0.9726

![or](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_or.png)

**bn128_mul**: Slope = 0.38µs/cycle, R² = 0.9997

![bn128_mul](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_bn128_mul.png)

**address**: Slope = 0.37µs/cycle, R² = 0.9969

![address](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_address.png)

**signextend**: Slope = 0.37µs/cycle, R² = 0.9997

![signextend](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_signextend.png)

**origin**: Slope = 0.37µs/cycle, R² = 0.9996

![origin](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_origin.png)

**dup16**: Slope = 0.37µs/cycle, R² = 0.9768

![dup16](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_dup16.png)

**caller**: Slope = 0.37µs/cycle, R² = 0.9997

![caller](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_caller.png)

**jumpi**: Slope = 0.37µs/cycle, R² = 0.9978

![jumpi](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_jumpi.png)

**sar**: Slope = 0.37µs/cycle, R² = 0.9998

![sar](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_sar.png)

**shr**: Slope = 0.37µs/cycle, R² = 0.9983

![shr](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_shr.png)

**mod**: Slope = 0.37µs/cycle, R² = 1.0000

![mod](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_mod.png)

**pc**: Slope = 0.37µs/cycle, R² = 1.0000

![pc](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_pc.png)

**slt**: Slope = 0.36µs/cycle, R² = 0.9963

![slt](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_slt.png)

**pop**: Slope = 0.36µs/cycle, R² = 0.9865

![pop](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_pop.png)

**push0**: Slope = 0.36µs/cycle, R² = 0.9990

![push0](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_push0.png)

**addmod**: Slope = 0.36µs/cycle, R² = 0.9999

![addmod](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_addmod.png)

**bn128_pairing**: Slope = 0.36µs/cycle, R² = 0.9968

![bn128_pairing](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_bn128_pairing.png)

**sub**: Slope = 0.36µs/cycle, R² = 0.9926

![sub](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_sub.png)

**sdiv**: Slope = 0.36µs/cycle, R² = 1.0000

![sdiv](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_sdiv.png)

**mulmod**: Slope = 0.36µs/cycle, R² = 1.0000

![mulmod](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_mulmod.png)

**xor**: Slope = 0.36µs/cycle, R² = 0.9995

![xor](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_xor.png)

**sgt**: Slope = 0.36µs/cycle, R² = 0.9896

![sgt](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_sgt.png)

**div**: Slope = 0.36µs/cycle, R² = 0.9999

![div](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_div.png)

**coinbase**: Slope = 0.36µs/cycle, R² = 0.9984

![coinbase](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_coinbase.png)

**smod**: Slope = 0.36µs/cycle, R² = 0.9995

![smod](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_smod.png)

**iszero**: Slope = 0.36µs/cycle, R² = 0.9940

![iszero](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_iszero.png)

**basefee**: Slope = 0.36µs/cycle, R² = 0.9988

![basefee](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_basefee.png)

**codecopy**: Slope = 0.36µs/cycle, R² = 0.9998

![codecopy](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_codecopy.png)

**tload**: Slope = 0.36µs/cycle, R² = 0.9999

![tload](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_tload.png)

**calldataload**: Slope = 0.36µs/cycle, R² = 0.9970

![calldataload](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_calldataload.png)

**gt**: Slope = 0.36µs/cycle, R² = 0.9984

![gt](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_gt.png)

**delegatecall**: Slope = 0.35µs/cycle, R² = 0.9998

![delegatecall](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_delegatecall.png)

**blobhash**: Slope = 0.35µs/cycle, R² = 0.9980

![blobhash](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_blobhash.png)

**mstore**: Slope = 0.35µs/cycle, R² = 0.9961

![mstore](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_mstore.png)

**codesize**: Slope = 0.35µs/cycle, R² = 0.9992

![codesize](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_codesize.png)

**gaslimit**: Slope = 0.35µs/cycle, R² = 0.9954

![gaslimit](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_gaslimit.png)

**msize**: Slope = 0.35µs/cycle, R² = 0.9998

![msize](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_msize.png)

**prevrandao**: Slope = 0.35µs/cycle, R² = 0.9998

![prevrandao](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_prevrandao.png)

**exp**: Slope = 0.35µs/cycle, R² = 0.9997

![exp](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_exp.png)

**dup8**: Slope = 0.35µs/cycle, R² = 0.9814

![dup8](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_dup8.png)

**sstore**: Slope = 0.35µs/cycle, R² = 0.9995

![sstore](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_sstore.png)

**selfbalance**: Slope = 0.35µs/cycle, R² = 0.9999

![selfbalance](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_selfbalance.png)

**mul**: Slope = 0.35µs/cycle, R² = 0.9970

![mul](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_mul.png)

**call**: Slope = 0.35µs/cycle, R² = 1.0000

![call](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_call.png)

**mload**: Slope = 0.35µs/cycle, R² = 0.9983

![mload](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_mload.png)

**not**: Slope = 0.35µs/cycle, R² = 0.9945

![not](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_not.png)

**gasprice**: Slope = 0.35µs/cycle, R² = 0.9986

![gasprice](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_gasprice.png)

**callcode**: Slope = 0.35µs/cycle, R² = 0.9998

![callcode](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_callcode.png)

**returndatasize**: Slope = 0.35µs/cycle, R² = 0.9997

![returndatasize](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_returndatasize.png)

**extcodesize**: Slope = 0.35µs/cycle, R² = 0.9996

![extcodesize](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_extcodesize.png)

**timestamp**: Slope = 0.35µs/cycle, R² = 0.9988

![timestamp](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_timestamp.png)

**modexp**: Slope = 0.35µs/cycle, R² = 0.9998

![modexp](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_modexp.png)

**calldatacopy**: Slope = 0.35µs/cycle, R² = 0.9999

![calldatacopy](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_calldatacopy.png)

**mcopy**: Slope = 0.35µs/cycle, R² = 0.9973

![mcopy](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_mcopy.png)

**jump**: Slope = 0.35µs/cycle, R² = 0.9973

![jump](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_jump.png)

**calldatasize**: Slope = 0.35µs/cycle, R² = 0.9972

![calldatasize](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_calldatasize.png)

**balance**: Slope = 0.35µs/cycle, R² = 0.9999

![balance](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_balance.png)

**sload**: Slope = 0.35µs/cycle, R² = 0.9999

![sload](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_sload.png)

**mstore8**: Slope = 0.35µs/cycle, R² = 0.9980

![mstore8](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_mstore8.png)

**dup1**: Slope = 0.35µs/cycle, R² = 0.9904

![dup1](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_dup1.png)

**add**: Slope = 0.35µs/cycle, R² = 0.9990

![add](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_add.png)

**blake2f**: Slope = 0.35µs/cycle, R² = 1.0000

![blake2f](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_blake2f.png)

**push32**: Slope = 0.35µs/cycle, R² = 0.9954

![push32](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_push32.png)

**number**: Slope = 0.35µs/cycle, R² = 0.9999

![number](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_number.png)

**extcodecopy**: Slope = 0.34µs/cycle, R² = 0.9999

![extcodecopy](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_extcodecopy.png)

**eq**: Slope = 0.34µs/cycle, R² = 0.9993

![eq](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_eq.png)

**blobbasefee**: Slope = 0.34µs/cycle, R² = 0.9997

![blobbasefee](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_blobbasefee.png)

**tstore**: Slope = 0.34µs/cycle, R² = 1.0000

![tstore](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_tstore.png)

**staticcall**: Slope = 0.34µs/cycle, R² = 0.9999

![staticcall](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_staticcall.png)

**callvalue**: Slope = 0.34µs/cycle, R² = 0.9978

![callvalue](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_callvalue.png)

**push1**: Slope = 0.34µs/cycle, R² = 0.9935

![push1](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_push1.png)

**and**: Slope = 0.34µs/cycle, R² = 0.9964

![and](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_and.png)

**chainid**: Slope = 0.34µs/cycle, R² = 0.9980

![chainid](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_chainid.png)

**extcodehash**: Slope = 0.34µs/cycle, R² = 0.9998

![extcodehash](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_extcodehash.png)

**swap16**: Slope = 0.34µs/cycle, R² = 0.9956

![swap16](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_swap16.png)

**ripemd160**: Slope = 0.34µs/cycle, R² = 0.9982

![ripemd160](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_ripemd160.png)

**jumpdest**: Slope = 0.33µs/cycle, R² = 0.9951

![jumpdest](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_jumpdest.png)

**byte**: Slope = 0.32µs/cycle, R² = 0.9964

![byte](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_byte.png)

**returndatacopy**: Slope = 0.32µs/cycle, R² = 0.9920

![returndatacopy](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_returndatacopy.png)

**swap1**: Slope = 0.31µs/cycle, R² = 0.9913

![swap1](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_swap1.png)

**swap8**: Slope = 0.29µs/cycle, R² = 0.9917

![swap8](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_swap8.png)

**sha256**: Slope = 0.28µs/cycle, R² = 0.8015

![sha256](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_sha256.png)

**log1**: Slope = 0.04µs/cycle, R² = 0.9956

![log1](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_log1.png)

**log0**: Slope = 0.04µs/cycle, R² = 0.9815

![log0](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_log0.png)

**log2**: Slope = 0.03µs/cycle, R² = 0.9988

![log2](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_log2.png)

**log3**: Slope = 0.03µs/cycle, R² = 0.9882

![log3](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_log3.png)

**log4**: Slope = 0.03µs/cycle, R² = 0.9982

![log4](/marginal-gas-benchmark/risc0/plots/zkcycles_proving_log4.png)

## Appendix: Per-Op-Count Regression

### Gas ↔ Proving Time

| Opcode | Time/Gas | R² | Std Error |
|--------|----------|-----|-----------|
| modexp | 1.36ms/gas | <span style="color: #28a745; font-weight: bold;">0.9998</span> | 13.02µs |
| blake2f | 242.84µs/gas | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 1.58µs |
| mulmod | 151.85µs/gas | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.54µs |
| div | 110.29µs/gas | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.60µs |
| mod | 94.55µs/gas | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.31µs |
| sdiv | 92.84µs/gas | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.40µs |
| bn128_mul | 86.68µs/gas | <span style="color: #28a745; font-weight: bold;">0.9997</span> | 1.04µs |
| point_evaluation | 85.54µs/gas | <span style="color: #28a745; font-weight: bold;">0.9945</span> | 4.50µs |
| selfbalance | 84.67µs/gas | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.57µs |
| keccak256 | 84.62µs/gas | <span style="color: #28a745; font-weight: bold;">0.9980</span> | 2.71µs |
| bn128_pairing | 81.72µs/gas | <span style="color: #28a745; font-weight: bold;">0.9968</span> | 2.68µs |
| ecrecover | 81.32µs/gas | <span style="color: #28a745; font-weight: bold;">0.9921</span> | 5.13µs |
| addmod | 71.00µs/gas | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.49µs |
| bls12_g1msm | 37.74µs/gas | <span style="color: #28a745; font-weight: bold;">0.9893</span> | 2.78µs |
| bls12_pairing | 36.15µs/gas | <span style="color: #28a745; font-weight: bold;">0.9979</span> | 1.17µs |
| eq | 34.89µs/gas | <span style="color: #28a745; font-weight: bold;">0.9993</span> | 0.65µs |
| exp | 32.61µs/gas | <span style="color: #28a745; font-weight: bold;">0.9997</span> | 0.40µs |
| bls12_map_fp_to_g1 | 31.51µs/gas | <span style="color: #28a745; font-weight: bold;">0.9772</span> | 3.40µs |
| smod | 28.99µs/gas | <span style="color: #28a745; font-weight: bold;">0.9996</span> | 0.43µs |
| bls12_g2msm | 28.85µs/gas | <span style="color: #28a745; font-weight: bold;">0.9960</span> | 1.30µs |
| sar | 28.77µs/gas | <span style="color: #28a745; font-weight: bold;">0.9998</span> | 0.32µs |
| bn128_add | 28.52µs/gas | <span style="color: #ffc107; font-weight: bold;">0.7509</span> | 11.61µs |
| prevrandao | 27.62µs/gas | <span style="color: #28a745; font-weight: bold;">0.9998</span> | 0.27µs |
| bls12_g1add | 25.16µs/gas | <span style="color: #28a745; font-weight: bold;">0.9518</span> | 4.00µs |
| call | 24.72µs/gas | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.11µs |
| callcode | 24.46µs/gas | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.17µs |
| staticcall | 23.98µs/gas | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.16µs |
| mstore | 22.14µs/gas | <span style="color: #28a745; font-weight: bold;">0.9961</span> | 0.98µs |
| address | 21.50µs/gas | <span style="color: #28a745; font-weight: bold;">0.9969</span> | 0.85µs |
| origin | 21.41µs/gas | <span style="color: #28a745; font-weight: bold;">0.9995</span> | 0.32µs |
| caller | 21.35µs/gas | <span style="color: #28a745; font-weight: bold;">0.9998</span> | 0.24µs |
| mul | 21.21µs/gas | <span style="color: #28a745; font-weight: bold;">0.9970</span> | 0.83µs |
| delegatecall | 20.95µs/gas | <span style="color: #28a745; font-weight: bold;">0.9998</span> | 0.23µs |
| coinbase | 20.80µs/gas | <span style="color: #28a745; font-weight: bold;">0.9983</span> | 0.60µs |
| signextend | 20.53µs/gas | <span style="color: #28a745; font-weight: bold;">0.9997</span> | 0.23µs |
| bls12_g2add | 20.28µs/gas | <span style="color: #28a745; font-weight: bold;">0.9636</span> | 2.79µs |
| shl | 20.17µs/gas | <span style="color: #28a745; font-weight: bold;">0.9991</span> | 0.43µs |
| calldataload | 20.03µs/gas | <span style="color: #28a745; font-weight: bold;">0.9970</span> | 0.78µs |
| mload | 19.82µs/gas | <span style="color: #28a745; font-weight: bold;">0.9983</span> | 0.58µs |
| bls12_map_fp2_to_g2 | 19.25µs/gas | <span style="color: #28a745; font-weight: bold;">0.9404</span> | 3.43µs |
| shr | 18.85µs/gas | <span style="color: #28a745; font-weight: bold;">0.9984</span> | 0.53µs |
| swap16 | 17.95µs/gas | <span style="color: #28a745; font-weight: bold;">0.9957</span> | 0.84µs |
| push32 | 16.51µs/gas | <span style="color: #28a745; font-weight: bold;">0.9954</span> | 0.56µs |
| swap1 | 16.31µs/gas | <span style="color: #28a745; font-weight: bold;">0.9913</span> | 0.76µs |
| swap8 | 15.32µs/gas | <span style="color: #28a745; font-weight: bold;">0.9917</span> | 0.99µs |
| blobbasefee | 14.61µs/gas | <span style="color: #28a745; font-weight: bold;">0.9997</span> | 0.16µs |
| gasprice | 14.14µs/gas | <span style="color: #28a745; font-weight: bold;">0.9986</span> | 0.37µs |
| slt | 13.84µs/gas | <span style="color: #28a745; font-weight: bold;">0.9963</span> | 0.42µs |
| sgt | 13.68µs/gas | <span style="color: #28a745; font-weight: bold;">0.9896</span> | 0.70µs |
| sub | 13.48µs/gas | <span style="color: #28a745; font-weight: bold;">0.9927</span> | 0.58µs |
| callvalue | 13.39µs/gas | <span style="color: #28a745; font-weight: bold;">0.9978</span> | 0.36µs |
| codecopy | 12.70µs/gas | <span style="color: #28a745; font-weight: bold;">0.9998</span> | 0.13µs |
| sstore | 12.64µs/gas | <span style="color: #28a745; font-weight: bold;">0.9994</span> | 0.22µs |
| calldatacopy | 12.43µs/gas | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.08µs |
| pc | 11.71µs/gas | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.04µs |
| add | 11.65µs/gas | <span style="color: #28a745; font-weight: bold;">0.9991</span> | 0.25µs |
| gas | 11.37µs/gas | <span style="color: #28a745; font-weight: bold;">0.9961</span> | 0.50µs |
| timestamp | 11.32µs/gas | <span style="color: #28a745; font-weight: bold;">0.9987</span> | 0.29µs |
| basefee | 11.25µs/gas | <span style="color: #28a745; font-weight: bold;">0.9987</span> | 0.28µs |
| number | 11.22µs/gas | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.08µs |
| calldatasize | 11.12µs/gas | <span style="color: #28a745; font-weight: bold;">0.9972</span> | 0.29µs |
| push16 | 11.11µs/gas | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.09µs |
| codesize | 10.76µs/gas | <span style="color: #28a745; font-weight: bold;">0.9992</span> | 0.22µs |
| chainid | 10.65µs/gas | <span style="color: #28a745; font-weight: bold;">0.9979</span> | 0.35µs |
| lt | 10.60µs/gas | <span style="color: #28a745; font-weight: bold;">0.9483</span> | 1.75µs |
| gaslimit | 10.58µs/gas | <span style="color: #28a745; font-weight: bold;">0.9953</span> | 0.51µs |
| msize | 10.57µs/gas | <span style="color: #28a745; font-weight: bold;">0.9998</span> | 0.08µs |
| returndatasize | 10.45µs/gas | <span style="color: #28a745; font-weight: bold;">0.9996</span> | 0.14µs |
| push0 | 10.31µs/gas | <span style="color: #28a745; font-weight: bold;">0.9990</span> | 0.23µs |
| gt | 9.70µs/gas | <span style="color: #28a745; font-weight: bold;">0.9983</span> | 0.28µs |
| blobhash | 9.55µs/gas | <span style="color: #28a745; font-weight: bold;">0.9981</span> | 0.30µs |
| or | 9.07µs/gas | <span style="color: #28a745; font-weight: bold;">0.9728</span> | 0.76µs |
| mstore8 | 9.02µs/gas | <span style="color: #28a745; font-weight: bold;">0.9980</span> | 0.29µs |
| byte | 8.86µs/gas | <span style="color: #28a745; font-weight: bold;">0.9964</span> | 0.37µs |
| xor | 8.63µs/gas | <span style="color: #28a745; font-weight: bold;">0.9994</span> | 0.15µs |
| jumpdest | 8.55µs/gas | <span style="color: #28a745; font-weight: bold;">0.9944</span> | 0.45µs |
| returndatacopy | 8.51µs/gas | <span style="color: #28a745; font-weight: bold;">0.9920</span> | 0.54µs |
| and | 8.11µs/gas | <span style="color: #28a745; font-weight: bold;">0.9966</span> | 0.34µs |
| iszero | 7.74µs/gas | <span style="color: #28a745; font-weight: bold;">0.9938</span> | 0.31µs |
| extcodehash | 7.64µs/gas | <span style="color: #28a745; font-weight: bold;">0.9998</span> | 0.07µs |
| not | 6.40µs/gas | <span style="color: #28a745; font-weight: bold;">0.9948</span> | 0.23µs |
| dup16 | 6.27µs/gas | <span style="color: #28a745; font-weight: bold;">0.9768</span> | 0.48µs |
| dup8 | 5.95µs/gas | <span style="color: #28a745; font-weight: bold;">0.9816</span> | 0.41µs |
| extcodesize | 5.95µs/gas | <span style="color: #28a745; font-weight: bold;">0.9996</span> | 0.09µs |
| balance | 5.91µs/gas | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.04µs |
| dup1 | 5.89µs/gas | <span style="color: #28a745; font-weight: bold;">0.9909</span> | 0.28µs |
| tstore | 5.82µs/gas | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01µs |
| extcodecopy | 5.54µs/gas | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.04µs |
| pop | 5.47µs/gas | <span style="color: #28a745; font-weight: bold;">0.9867</span> | 0.32µs |
| sload | 5.39µs/gas | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.04µs |
| mcopy | 5.20µs/gas | <span style="color: #28a745; font-weight: bold;">0.9974</span> | 0.19µs |
| push1 | 5.19µs/gas | <span style="color: #28a745; font-weight: bold;">0.9935</span> | 0.21µs |
| jumpi | 4.52µs/gas | <span style="color: #28a745; font-weight: bold;">0.9978</span> | 0.15µs |
| ripemd160 | 3.15µs/gas | <span style="color: #28a745; font-weight: bold;">0.9982</span> | 0.09µs |
| jump | 2.74µs/gas | <span style="color: #28a745; font-weight: bold;">0.9976</span> | 0.07µs |
| identity | 1.87µs/gas | <span style="color: #28a745; font-weight: bold;">0.9274</span> | 0.37µs |
| sha256 | 0.94µs/gas | <span style="color: #ffc107; font-weight: bold;">0.8017</span> | 0.33µs |
| tload | 0.86µs/gas | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.01µs |
| create2 | 0.56µs/gas | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.00µs |
| create | 0.49µs/gas | <span style="color: #28a745; font-weight: bold;">0.9998</span> | 0.01µs |
| log0 | 0.30µs/gas | <span style="color: #28a745; font-weight: bold;">0.9807</span> | 0.02µs |
| log1 | 0.27µs/gas | <span style="color: #28a745; font-weight: bold;">0.9959</span> | 0.01µs |
| log3 | 0.16µs/gas | <span style="color: #28a745; font-weight: bold;">0.9874</span> | 0.01µs |
| log2 | 0.15µs/gas | <span style="color: #28a745; font-weight: bold;">0.9985</span> | 0.00µs |
| log4 | 0.15µs/gas | <span style="color: #28a745; font-weight: bold;">0.9983</span> | 0.00µs |

### Gas ↔ ZK Cycles

| Opcode | Cycles/Gas | R² | Std Error |
|--------|------------|-----|-----------|
| modexp | 3.91K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.05548 |
| blake2f | 702.03 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.003845 |
| mulmod | 420.63 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01475 |
| div | 306.78 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0175 |
| mod | 257.97 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.02337 |
| sdiv | 257.18 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01745 |
| selfbalance | 241.80 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.005111 |
| bn128_mul | 229.21 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.008463 |
| bn128_pairing | 225.96 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.002614 |
| point_evaluation | 223.09 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.008398 |
| addmod | 196.25 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0007769 |
| ecrecover | 193.83 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.004032 |
| eq | 101.29 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.02858 |
| exp | 92.99 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.001999 |
| bls12_g1msm | 89.91 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.00232 |
| bls12_pairing | 88.19 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.002931 |
| smod | 80.97 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.02077 |
| keccak256 | 80.73 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0001587 |
| prevrandao | 78.49 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01303 |
| sar | 78.36 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0485 |
| bls12_g2msm | 73.03 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.002221 |
| call | 70.64 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.146 |
| callcode | 70.06 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.1453 |
| staticcall | 69.81 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.1154 |
| bn128_add | 67.29 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.06984 |
| mstore | 62.67 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.003245 |
| mul | 60.57 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01852 |
| delegatecall | 59.11 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.1172 |
| address | 57.99 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01283 |
| origin | 57.98 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01297 |
| coinbase | 57.98 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01346 |
| caller | 57.98 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01282 |
| mload | 56.68 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0158 |
| calldataload | 56.34 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0009552 |
| bls12_map_fp_to_g1 | 55.78 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.00736 |
| signextend | 55.40 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 6.051e-05 |
| swap16 | 53.37 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01695 |
| swap1 | 53.35 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01812 |
| swap8 | 53.34 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0003698 |
| bls12_g2add | 52.96 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.1624 |
| shr | 51.35 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.05555 |
| shl | 51.02 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.05619 |
| push32 | 47.79 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.001532 |
| blobbasefee | 42.48 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01323 |
| gasprice | 40.48 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01313 |
| bls12_g1add | 40.44 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.02711 |
| bls12_map_fp2_to_g2 | 40.15 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.003976 |
| callvalue | 38.99 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01211 |
| sgt | 37.97 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.02033 |
| slt | 37.96 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.02345 |
| sub | 37.30 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.02252 |
| sstore | 36.08 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.06192 |
| calldatacopy | 35.67 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 8.247e-05 |
| codecopy | 35.67 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 5.494e-05 |
| add | 33.62 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.02848 |
| timestamp | 32.48 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01292 |
| number | 32.48 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01328 |
| calldatasize | 31.99 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01043 |
| pc | 31.98 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01301 |
| basefee | 31.48 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01303 |
| chainid | 31.48 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01305 |
| codesize | 30.48 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01265 |
| msize | 30.01 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.000413 |
| gas | 29.98 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01324 |
| gaslimit | 29.98 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01335 |
| returndatasize | 29.98 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01273 |
| push0 | 28.48 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01289 |
| push16 | 28.37 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01699 |
| byte | 27.36 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.05331 |
| gt | 27.30 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.02397 |
| lt | 27.28 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.02589 |
| blobhash | 27.00 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0002251 |
| returndatacopy | 26.45 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0003056 |
| mstore8 | 26.00 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.004088 |
| jumpdest | 25.84 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.09462 |
| or | 23.96 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.02271 |
| and | 23.96 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.02841 |
| xor | 23.95 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.02856 |
| extcodehash | 22.70 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 5.128e-05 |
| iszero | 21.65 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.02629 |
| not | 18.32 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.02607 |
| extcodesize | 17.07 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 5.257e-05 |
| balance | 17.01 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 4.584e-06 |
| dup16 | 17.00 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0004251 |
| dup1 | 16.98 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.02709 |
| dup8 | 16.98 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0136 |
| tstore | 16.93 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.003899 |
| extcodecopy | 16.06 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01009 |
| sload | 15.53 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.002623 |
| push1 | 15.34 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01127 |
| pop | 15.02 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01038 |
| mcopy | 14.92 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.02028 |
| jumpi | 12.30 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0002197 |
| ripemd160 | 9.38 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.002035 |
| jump | 7.88 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01299 |
| log0 | 7.77 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.02176 |
| log1 | 6.76 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01361 |
| log4 | 5.25 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01303 |
| log3 | 4.98 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.02489 |
| log2 | 4.67 | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.02364 |
| sha256 | 3.41 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.001651 |
| tload | 2.43 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0008525 |
| identity | 1.70 | <span style="color: #28a745; font-weight: bold;">0.9998</span> | 0.01748 |
| create2 | 1.37 | <span style="color: #28a745; font-weight: bold;">0.9997</span> | 0.0153 |
| create | 1.29 | <span style="color: #28a745; font-weight: bold;">0.9997</span> | 0.01596 |

### ZK Cycles ↔ Proving Time

| Opcode | Time/Cycle | R² | Std Error |
|--------|------------|-----|-----------|
| identity | 1.10µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9206</span> | 0.23µs |
| keccak256 | 1.05µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9980</span> | 0.03µs |
| bls12_g1add | 0.62µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9522</span> | 0.10µs |
| bls12_map_fp_to_g1 | 0.56µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9773</span> | 0.06µs |
| bls12_map_fp2_to_g2 | 0.48µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9404</span> | 0.09µs |
| bn128_add | 0.42µs/cycle | <span style="color: #ffc107; font-weight: bold;">0.7509</span> | 0.17µs |
| bls12_g1msm | 0.42µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9893</span> | 0.03µs |
| ecrecover | 0.42µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9921</span> | 0.03µs |
| create2 | 0.41µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9995</span> | 0.01µs |
| bls12_pairing | 0.41µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9979</span> | 0.01µs |
| shl | 0.40µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9992</span> | 0.01µs |
| bls12_g2msm | 0.40µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9960</span> | 0.02µs |
| push16 | 0.39µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.00µs |
| lt | 0.39µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9484</span> | 0.06µs |
| point_evaluation | 0.38µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9945</span> | 0.02µs |
| bls12_g2add | 0.38µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9636</span> | 0.05µs |
| create | 0.38µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9994</span> | 0.01µs |
| gas | 0.38µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9961</span> | 0.02µs |
| or | 0.38µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9726</span> | 0.03µs |
| bn128_mul | 0.38µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9997</span> | 0.00µs |
| address | 0.37µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9969</span> | 0.01µs |
| signextend | 0.37µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9997</span> | 0.00µs |
| origin | 0.37µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9996</span> | 0.01µs |
| dup16 | 0.37µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9768</span> | 0.03µs |
| caller | 0.37µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9997</span> | 0.00µs |
| jumpi | 0.37µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9978</span> | 0.01µs |
| sar | 0.37µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9998</span> | 0.00µs |
| shr | 0.37µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9983</span> | 0.01µs |
| mod | 0.37µs/cycle | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.00µs |
| pc | 0.37µs/cycle | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.00µs |
| slt | 0.36µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9963</span> | 0.01µs |
| pop | 0.36µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9865</span> | 0.02µs |
| push0 | 0.36µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9990</span> | 0.01µs |
| addmod | 0.36µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.00µs |
| bn128_pairing | 0.36µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9968</span> | 0.01µs |
| sub | 0.36µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9926</span> | 0.02µs |
| sdiv | 0.36µs/cycle | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.00µs |
| mulmod | 0.36µs/cycle | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.00µs |
| xor | 0.36µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9995</span> | 0.01µs |
| sgt | 0.36µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9896</span> | 0.02µs |
| div | 0.36µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.00µs |
| coinbase | 0.36µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9984</span> | 0.01µs |
| smod | 0.36µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9995</span> | 0.01µs |
| iszero | 0.36µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9940</span> | 0.01µs |
| basefee | 0.36µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9988</span> | 0.01µs |
| codecopy | 0.36µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9998</span> | 0.00µs |
| tload | 0.36µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.00µs |
| calldataload | 0.36µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9970</span> | 0.01µs |
| gt | 0.36µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9984</span> | 0.01µs |
| delegatecall | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9998</span> | 0.00µs |
| blobhash | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9980</span> | 0.01µs |
| mstore | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9961</span> | 0.02µs |
| codesize | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9992</span> | 0.01µs |
| gaslimit | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9954</span> | 0.02µs |
| msize | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9998</span> | 0.00µs |
| prevrandao | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9998</span> | 0.00µs |
| exp | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9997</span> | 0.00µs |
| dup8 | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9814</span> | 0.02µs |
| sstore | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9995</span> | 0.01µs |
| selfbalance | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.00µs |
| mul | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9970</span> | 0.01µs |
| call | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.00µs |
| mload | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9983</span> | 0.01µs |
| not | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9945</span> | 0.01µs |
| gasprice | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9986</span> | 0.01µs |
| callcode | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9998</span> | 0.00µs |
| returndatasize | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9997</span> | 0.00µs |
| extcodesize | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9996</span> | 0.01µs |
| timestamp | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9988</span> | 0.01µs |
| modexp | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9998</span> | 0.00µs |
| calldatacopy | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.00µs |
| mcopy | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9973</span> | 0.01µs |
| jump | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9973</span> | 0.01µs |
| calldatasize | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9972</span> | 0.01µs |
| balance | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.00µs |
| sload | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.00µs |
| mstore8 | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9980</span> | 0.01µs |
| dup1 | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9904</span> | 0.02µs |
| add | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9990</span> | 0.01µs |
| blake2f | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.00µs |
| push32 | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9954</span> | 0.01µs |
| number | 0.35µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.00µs |
| extcodecopy | 0.34µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.00µs |
| eq | 0.34µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9993</span> | 0.01µs |
| blobbasefee | 0.34µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9997</span> | 0.00µs |
| tstore | 0.34µs/cycle | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.00µs |
| staticcall | 0.34µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.00µs |
| callvalue | 0.34µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9978</span> | 0.01µs |
| push1 | 0.34µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9935</span> | 0.01µs |
| and | 0.34µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9964</span> | 0.01µs |
| chainid | 0.34µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9980</span> | 0.01µs |
| extcodehash | 0.34µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9998</span> | 0.00µs |
| swap16 | 0.34µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9956</span> | 0.02µs |
| ripemd160 | 0.34µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9982</span> | 0.01µs |
| jumpdest | 0.33µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9951</span> | 0.02µs |
| byte | 0.32µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9964</span> | 0.01µs |
| returndatacopy | 0.32µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9920</span> | 0.02µs |
| swap1 | 0.31µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9913</span> | 0.01µs |
| swap8 | 0.29µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9917</span> | 0.02µs |
| sha256 | 0.28µs/cycle | <span style="color: #ffc107; font-weight: bold;">0.8015</span> | 0.10µs |
| log1 | 0.04µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9956</span> | 0.00µs |
| log0 | 0.04µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9815</span> | 0.00µs |
| log2 | 0.03µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9988</span> | 0.00µs |
| log3 | 0.03µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9882</span> | 0.00µs |
| log4 | 0.03µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9982</span> | 0.00µs |

### Op Count ↔ Gas Used (Marginal Property Check)

*High R² (≥ 0.99) indicates gas scales linearly with op count, confirming the marginal property.*

| Opcode | Gas/Op | Intercept | R² | Status |
|--------|--------|-----------|-----|--------|
| add | 3.00 | 9.48M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| addmod | 8.00 | 2.80M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| address | 2.00 | 3.86M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| and | 3.00 | 6.34M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| balance | 100.00 | 2.40M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| basefee | 2.00 | 5.14M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| blake2f | 65.54K | 49.27K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| blobbasefee | 2.00 | 5.14M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| blobhash | 3.00 | 9.50M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| bls12_g1add | 375.00 | 140.85K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| bls12_g1msm | 22.78K | 54.04K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| bls12_g2add | 600.00 | 106.04K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| bls12_g2msm | 45.00K | 55.64K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| bls12_map_fp2_to_g2 | 23.80K | 50.64K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| bls12_map_fp_to_g1 | 5.50K | 55.51K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| bls12_pairing | 102.90K | 56.91K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| bn128_add | 150.00 | 143.13K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| bn128_mul | 6.00K | 55.85K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| bn128_pairing | 113.00K | 52.88K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| byte | 3.00 | 6.34M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| calldatacopy | 6.00 | 4.56M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| calldataload | 3.00 | 3.20M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| calldatasize | 2.00 | 5.14M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| caller | 2.00 | 5.14M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| callvalue | 2.00 | 5.14M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| chainid | 2.00 | 5.14M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| codecopy | 6.00 | 6.82M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| codesize | 2.00 | 5.14M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| coinbase | 2.00 | 5.14M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| create | 32.01K | 123.81K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| create2 | 32.02K | 200.71K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| div | 5.00 | 2.25M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| dup1 | 3.00 | 4.10M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| dup16 | 3.00 | 2.90M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| dup8 | 3.00 | 4.20M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| ecrecover | 3.00K | 71.36K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| eq | 3.00 | 4.76M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| exp | 1.61K | 70.29K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| extcodehash | 100.00 | 1.95M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| extcodesize | 100.00 | 1.93M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| gas | 2.00 | 5.14M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| gaslimit | 2.00 | 5.14M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| gasprice | 2.00 | 5.14M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| gt | 3.00 | 6.34M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| iszero | 3.00 | 9.93M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| jump | 8.00 | 2.88M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| jumpdest | 1.00 | 625.61K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| jumpi | 10.00 | 7.81M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| keccak256 | 1.57K | 930.61K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| log0 | 375.00 | 957.61K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| lt | 3.00 | 6.34M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| mload | 3.00 | 6.37M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| mod | 5.00 | 2.25M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| modexp | 1.36K | 66.98K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| msize | 2.00 | 5.16M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| mstore | 3.00 | 2.71M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| mstore8 | 3.00 | 5.37M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| mul | 5.00 | 6.34M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| mulmod | 8.00 | 2.92M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| not | 3.00 | 6.64M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| number | 2.00 | 5.14M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| or | 3.00 | 9.48M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| origin | 2.00 | 5.14M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| pc | 2.00 | 5.14M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| point_evaluation | 50.00K | 49.92K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| pop | 2.00 | 4.08M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| prevrandao | 2.00 | 3.86M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| push0 | 2.00 | 5.14M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| push1 | 3.00 | 5.14M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| push16 | 3.00 | 4.34M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| push32 | 3.00 | 2.36M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| returndatasize | 2.00 | 5.14M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| ripemd160 | 4.44K | 240.82K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| sar | 3.00 | 4.76M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| sdiv | 5.00 | 2.25M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| selfbalance | 5.00 | 2.59M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| sgt | 3.00 | 9.48M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| sha256 | 1.60K | 290.57K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| shl | 3.00 | 4.76M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| shr | 3.00 | 6.34M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| signextend | 5.00 | 15.77M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| sload | 100.00 | 1.93M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| slt | 3.00 | 22.06M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| smod | 5.00 | 6.34M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| sub | 3.00 | 15.77M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| swap1 | 3.00 | 200.61K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| swap16 | 3.00 | 275.61K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| swap8 | 3.00 | 235.61K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| timestamp | 2.00 | 5.14M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| tload | 100.00 | 9.77M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| tstore | 100.00 | 1.33M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| xor | 3.00 | 15.77M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| log2 | 3.17K | 671.06K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| log3 | 3.54K | 702.56K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| log1 | 1.01K | 941.81K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| log4 | 3.92K | 658.46K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| call | 100.05 | 1.21M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| callcode | 100.05 | 1.21M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| delegatecall | 100.05 | 1.06M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| staticcall | 100.05 | 1.06M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| identity | 27.02 | 219.29M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| returndatacopy | 9.02 | 1.68M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| sstore | 100.29 | 724.38K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| extcodecopy | 124.37 | 1.60M | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |
| mcopy | 99.33 | 648.67K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | ✅ Good |

### Op Count ↔ ZK Cycles

| Opcode | Cycles/Op | R² |
|--------|-----------|-----|
| add | 100.86 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| addmod | 1.57K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| address | 115.97 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| and | 71.87 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| balance | 1.70K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| basefee | 62.97 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| blake2f | 46.01M | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| blobbasefee | 84.97 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| blobhash | 81.01 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| bls12_g1add | 15.16K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| bls12_g1msm | 2.05M | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| bls12_g2add | 31.77K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| bls12_g2msm | 3.29M | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| bls12_map_fp2_to_g2 | 955.45K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| bls12_map_fp_to_g1 | 306.79K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| bls12_pairing | 9.08M | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| bn128_add | 10.09K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| bn128_mul | 1.38M | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| bn128_pairing | 25.53M | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| byte | 82.07 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| call | 7.07K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| callcode | 7.01K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| calldatacopy | 214.01 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| calldataload | 169.02 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| calldatasize | 63.98 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| caller | 115.97 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| callvalue | 77.97 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| chainid | 62.97 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| codecopy | 214.01 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| codesize | 60.97 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| coinbase | 115.97 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| create | 41.31K | <span style="color: #28a745; font-weight: bold;">0.9997</span> |
| create2 | 43.77K | <span style="color: #28a745; font-weight: bold;">0.9997</span> |
| delegatecall | 5.91K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| div | 1.53K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| dup1 | 50.94 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| dup16 | 51.01 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| dup8 | 50.94 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| ecrecover | 581.49K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| eq | 303.87 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| exp | 149.72K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| extcodecopy | 2.00K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| extcodehash | 2.27K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| extcodesize | 1.71K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| gas | 59.97 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| gaslimit | 59.97 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| gasprice | 80.97 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| gt | 81.89 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| identity | 45.89 | <span style="color: #28a745; font-weight: bold;">0.9998</span> |
| iszero | 64.94 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| jump | 63.01 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| jumpdest | 25.84 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| jumpi | 123.00 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| keccak256 | 126.43K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| log0 | 2.91K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| log1 | 6.80K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| log2 | 14.79K | <span style="color: #28a745; font-weight: bold;">0.9999</span> |
| log3 | 17.63K | <span style="color: #28a745; font-weight: bold;">0.9999</span> |
| log4 | 20.57K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| lt | 81.84 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| mcopy | 1.48K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| mload | 170.03 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| mod | 1.29K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| modexp | 5.34M | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| msize | 60.01 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| mstore | 188.01 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| mstore8 | 78.01 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| mul | 302.86 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| mulmod | 3.37K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| not | 54.95 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| number | 64.97 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| or | 71.89 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| origin | 115.97 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| pc | 63.97 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| point_evaluation | 11.15M | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| pop | 30.04 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| prevrandao | 156.97 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| push0 | 56.97 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| push1 | 46.01 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| push16 | 85.12 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| push32 | 143.38 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| returndatacopy | 238.52 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| returndatasize | 59.97 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| ripemd160 | 41.67K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| sar | 235.07 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| sdiv | 1.29K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| selfbalance | 1.21K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| sgt | 113.90 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| sha256 | 5.45K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| shl | 153.06 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| shr | 154.06 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| signextend | 277.01 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| sload | 1.55K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| slt | 113.89 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| smod | 404.87 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| sstore | 3.62K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| staticcall | 6.98K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| sub | 111.89 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| swap1 | 160.04 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| swap16 | 160.11 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| swap8 | 160.03 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| timestamp | 64.97 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| tload | 243.07 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| tstore | 1.69K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| xor | 71.86 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |

### Op Count ↔ Proving Time

| Opcode | Time/Op (s) | R² |
|--------|-------------|-----|
| add | 3.49e-05 | <span style="color: #28a745; font-weight: bold;">0.9991</span> |
| addmod | 5.68e-04 | <span style="color: #28a745; font-weight: bold;">0.9999</span> |
| address | 4.30e-05 | <span style="color: #28a745; font-weight: bold;">0.9969</span> |
| and | 2.43e-05 | <span style="color: #28a745; font-weight: bold;">0.9966</span> |
| balance | 5.91e-04 | <span style="color: #28a745; font-weight: bold;">0.9999</span> |
| basefee | 2.25e-05 | <span style="color: #28a745; font-weight: bold;">0.9987</span> |
| blake2f | 1.59e+01 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| blobbasefee | 2.92e-05 | <span style="color: #28a745; font-weight: bold;">0.9997</span> |
| blobhash | 2.86e-05 | <span style="color: #28a745; font-weight: bold;">0.9981</span> |
| bls12_g1add | 9.44e-03 | <span style="color: #28a745; font-weight: bold;">0.9518</span> |
| bls12_g1msm | 8.60e-01 | <span style="color: #28a745; font-weight: bold;">0.9893</span> |
| bls12_g2add | 1.22e-02 | <span style="color: #28a745; font-weight: bold;">0.9636</span> |
| bls12_g2msm | 1.30e+00 | <span style="color: #28a745; font-weight: bold;">0.9960</span> |
| bls12_map_fp2_to_g2 | 4.58e-01 | <span style="color: #28a745; font-weight: bold;">0.9404</span> |
| bls12_map_fp_to_g1 | 1.73e-01 | <span style="color: #28a745; font-weight: bold;">0.9772</span> |
| bls12_pairing | 3.72e+00 | <span style="color: #28a745; font-weight: bold;">0.9979</span> |
| bn128_add | 4.28e-03 | <span style="color: #ffc107; font-weight: bold;">0.7509</span> |
| bn128_mul | 5.20e-01 | <span style="color: #28a745; font-weight: bold;">0.9997</span> |
| bn128_pairing | 9.23e+00 | <span style="color: #28a745; font-weight: bold;">0.9968</span> |
| byte | 2.66e-05 | <span style="color: #28a745; font-weight: bold;">0.9964</span> |
| call | 2.47e-03 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| callcode | 2.45e-03 | <span style="color: #28a745; font-weight: bold;">0.9999</span> |
| calldatacopy | 7.46e-05 | <span style="color: #28a745; font-weight: bold;">0.9999</span> |
| calldataload | 6.01e-05 | <span style="color: #28a745; font-weight: bold;">0.9970</span> |
| calldatasize | 2.22e-05 | <span style="color: #28a745; font-weight: bold;">0.9972</span> |
| caller | 4.27e-05 | <span style="color: #28a745; font-weight: bold;">0.9998</span> |
| callvalue | 2.68e-05 | <span style="color: #28a745; font-weight: bold;">0.9978</span> |
| chainid | 2.13e-05 | <span style="color: #28a745; font-weight: bold;">0.9979</span> |
| codecopy | 7.62e-05 | <span style="color: #28a745; font-weight: bold;">0.9998</span> |
| codesize | 2.15e-05 | <span style="color: #28a745; font-weight: bold;">0.9992</span> |
| coinbase | 4.16e-05 | <span style="color: #28a745; font-weight: bold;">0.9983</span> |
| create | 1.57e-02 | <span style="color: #28a745; font-weight: bold;">0.9998</span> |
| create2 | 1.80e-02 | <span style="color: #28a745; font-weight: bold;">0.9999</span> |
| delegatecall | 2.10e-03 | <span style="color: #28a745; font-weight: bold;">0.9998</span> |
| div | 5.51e-04 | <span style="color: #28a745; font-weight: bold;">0.9999</span> |
| dup1 | 1.77e-05 | <span style="color: #28a745; font-weight: bold;">0.9909</span> |
| dup16 | 1.88e-05 | <span style="color: #28a745; font-weight: bold;">0.9768</span> |
| dup8 | 1.79e-05 | <span style="color: #28a745; font-weight: bold;">0.9816</span> |
| ecrecover | 2.44e-01 | <span style="color: #28a745; font-weight: bold;">0.9921</span> |
| eq | 1.05e-04 | <span style="color: #28a745; font-weight: bold;">0.9993</span> |
| exp | 5.25e-02 | <span style="color: #28a745; font-weight: bold;">0.9997</span> |
| extcodecopy | 6.89e-04 | <span style="color: #28a745; font-weight: bold;">0.9999</span> |
| extcodehash | 7.64e-04 | <span style="color: #28a745; font-weight: bold;">0.9998</span> |
| extcodesize | 5.95e-04 | <span style="color: #28a745; font-weight: bold;">0.9996</span> |
| gas | 2.27e-05 | <span style="color: #28a745; font-weight: bold;">0.9961</span> |
| gaslimit | 2.12e-05 | <span style="color: #28a745; font-weight: bold;">0.9953</span> |
| gasprice | 2.83e-05 | <span style="color: #28a745; font-weight: bold;">0.9986</span> |
| gt | 2.91e-05 | <span style="color: #28a745; font-weight: bold;">0.9983</span> |
| identity | 5.06e-05 | <span style="color: #28a745; font-weight: bold;">0.9276</span> |
| iszero | 2.32e-05 | <span style="color: #28a745; font-weight: bold;">0.9938</span> |
| jump | 2.19e-05 | <span style="color: #28a745; font-weight: bold;">0.9976</span> |
| jumpdest | 8.55e-06 | <span style="color: #28a745; font-weight: bold;">0.9944</span> |
| jumpi | 4.52e-05 | <span style="color: #28a745; font-weight: bold;">0.9978</span> |
| keccak256 | 1.33e-01 | <span style="color: #28a745; font-weight: bold;">0.9980</span> |
| log0 | 1.12e-04 | <span style="color: #28a745; font-weight: bold;">0.9807</span> |
| log1 | 2.73e-04 | <span style="color: #28a745; font-weight: bold;">0.9960</span> |
| log2 | 4.84e-04 | <span style="color: #28a745; font-weight: bold;">0.9985</span> |
| log3 | 5.72e-04 | <span style="color: #28a745; font-weight: bold;">0.9873</span> |
| log4 | 5.68e-04 | <span style="color: #28a745; font-weight: bold;">0.9983</span> |
| lt | 3.18e-05 | <span style="color: #28a745; font-weight: bold;">0.9483</span> |
| mcopy | 5.16e-04 | <span style="color: #28a745; font-weight: bold;">0.9972</span> |
| mload | 5.95e-05 | <span style="color: #28a745; font-weight: bold;">0.9983</span> |
| mod | 4.73e-04 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| modexp | 1.86e+00 | <span style="color: #28a745; font-weight: bold;">0.9998</span> |
| msize | 2.11e-05 | <span style="color: #28a745; font-weight: bold;">0.9998</span> |
| mstore | 6.64e-05 | <span style="color: #28a745; font-weight: bold;">0.9961</span> |
| mstore8 | 2.71e-05 | <span style="color: #28a745; font-weight: bold;">0.9980</span> |
| mul | 1.06e-04 | <span style="color: #28a745; font-weight: bold;">0.9970</span> |
| mulmod | 1.21e-03 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| not | 1.92e-05 | <span style="color: #28a745; font-weight: bold;">0.9948</span> |
| number | 2.24e-05 | <span style="color: #28a745; font-weight: bold;">0.9999</span> |
| or | 2.72e-05 | <span style="color: #28a745; font-weight: bold;">0.9728</span> |
| origin | 4.28e-05 | <span style="color: #28a745; font-weight: bold;">0.9995</span> |
| pc | 2.34e-05 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| point_evaluation | 4.28e+00 | <span style="color: #28a745; font-weight: bold;">0.9945</span> |
| pop | 1.09e-05 | <span style="color: #28a745; font-weight: bold;">0.9867</span> |
| prevrandao | 5.52e-05 | <span style="color: #28a745; font-weight: bold;">0.9998</span> |
| push0 | 2.06e-05 | <span style="color: #28a745; font-weight: bold;">0.9990</span> |
| push1 | 1.56e-05 | <span style="color: #28a745; font-weight: bold;">0.9935</span> |
| push16 | 3.33e-05 | <span style="color: #28a745; font-weight: bold;">0.9999</span> |
| push32 | 4.95e-05 | <span style="color: #28a745; font-weight: bold;">0.9954</span> |
| returndatacopy | 7.67e-05 | <span style="color: #28a745; font-weight: bold;">0.9922</span> |
| returndatasize | 2.09e-05 | <span style="color: #28a745; font-weight: bold;">0.9996</span> |
| ripemd160 | 1.40e-02 | <span style="color: #28a745; font-weight: bold;">0.9982</span> |
| sar | 8.63e-05 | <span style="color: #28a745; font-weight: bold;">0.9998</span> |
| sdiv | 4.64e-04 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| selfbalance | 4.23e-04 | <span style="color: #28a745; font-weight: bold;">0.9999</span> |
| sgt | 4.10e-05 | <span style="color: #28a745; font-weight: bold;">0.9896</span> |
| sha256 | 1.51e-03 | <span style="color: #ffc107; font-weight: bold;">0.8017</span> |
| shl | 6.05e-05 | <span style="color: #28a745; font-weight: bold;">0.9991</span> |
| shr | 5.65e-05 | <span style="color: #28a745; font-weight: bold;">0.9984</span> |
| signextend | 1.03e-04 | <span style="color: #28a745; font-weight: bold;">0.9997</span> |
| sload | 5.39e-04 | <span style="color: #28a745; font-weight: bold;">0.9999</span> |
| slt | 4.15e-05 | <span style="color: #28a745; font-weight: bold;">0.9963</span> |
| smod | 1.45e-04 | <span style="color: #28a745; font-weight: bold;">0.9996</span> |
| sstore | 1.27e-03 | <span style="color: #28a745; font-weight: bold;">0.9995</span> |
| staticcall | 2.40e-03 | <span style="color: #28a745; font-weight: bold;">0.9999</span> |
| sub | 4.04e-05 | <span style="color: #28a745; font-weight: bold;">0.9927</span> |
| swap1 | 4.89e-05 | <span style="color: #28a745; font-weight: bold;">0.9913</span> |
| swap16 | 5.39e-05 | <span style="color: #28a745; font-weight: bold;">0.9957</span> |
| swap8 | 4.60e-05 | <span style="color: #28a745; font-weight: bold;">0.9917</span> |
| timestamp | 2.26e-05 | <span style="color: #28a745; font-weight: bold;">0.9987</span> |
| tload | 8.65e-05 | <span style="color: #28a745; font-weight: bold;">0.9999</span> |
| tstore | 5.82e-04 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| xor | 2.59e-05 | <span style="color: #28a745; font-weight: bold;">0.9994</span> |

### Opcodes/Precompiles Summary

| Opcode | N | Min Op Count | Max Op Count | Min Gas | Max Gas | Gas/Op |
|--------|---|--------------|--------------|---------|---------|--------|
| add | 4 | 0 | 900000 | 9.48M | 12.18M | 3.00 |
| addmod | 4 | 0 | 201000 | 2.80M | 4.41M | 8.00 |
| address | 4 | 0 | 900000 | 3.86M | 5.66M | 2.00 |
| and | 4 | 0 | 600000 | 6.34M | 8.14M | 3.00 |
| balance | 4 | 0 | 300000 | 2.40M | 32.40M | 100.00 |
| basefee | 4 | 0 | 1200000 | 5.14M | 7.54M | 2.00 |
| blake2f | 3 | 0 | 10 | 49.27K | 704.62K | 65.54K |
| blobbasefee | 4 | 0 | 1200000 | 5.14M | 7.54M | 2.00 |
| blobhash | 4 | 0 | 1503000 | 9.50M | 14.01M | 3.00 |
| bls12_g1add | 4 | 0 | 771 | 140.85K | 429.98K | 375.00 |
| bls12_g1msm | 4 | 0 | 39 | 54.04K | 942.30K | 22.78K |
| bls12_g2add | 4 | 0 | 450 | 106.04K | 376.04K | 600.00 |
| bls12_g2msm | 4 | 0 | 24 | 55.64K | 1.14M | 45.00K |
| bls12_map_fp2_to_g2 | 4 | 0 | 27 | 50.64K | 693.24K | 23.80K |
| bls12_map_fp_to_g1 | 4 | 0 | 75 | 55.51K | 468.01K | 5.50K |
| bls12_pairing | 4 | 0 | 9 | 56.91K | 983.01K | 102.90K |
| bn128_add | 4 | 0 | 801 | 143.13K | 263.28K | 150.00 |
| bn128_mul | 4 | 0 | 72 | 55.85K | 487.85K | 6.00K |
| bn128_pairing | 5 | 0 | 8 | 52.88K | 956.88K | 113.00K |
| byte | 4 | 0 | 600000 | 6.34M | 8.14M | 3.00 |
| call | 4 | 0 | 49500 | 1.21M | 6.16M | 100.05 |
| callcode | 4 | 0 | 49500 | 1.21M | 6.16M | 100.05 |
| calldatacopy | 4 | 0 | 600000 | 4.56M | 8.16M | 6.00 |
| calldataload | 4 | 0 | 501000 | 3.20M | 4.70M | 3.00 |
| calldatasize | 6 | 0 | 1200000 | 5.14M | 7.54M | 2.00 |
| caller | 4 | 0 | 1200000 | 5.14M | 7.54M | 2.00 |
| callvalue | 5 | 0 | 1200000 | 5.14M | 7.54M | 2.00 |
| chainid | 4 | 0 | 1200000 | 5.14M | 7.54M | 2.00 |
| codecopy | 4 | 0 | 900000 | 6.82M | 12.22M | 6.00 |
| codesize | 4 | 0 | 1200000 | 5.14M | 7.54M | 2.00 |
| coinbase | 4 | 0 | 1200000 | 5.14M | 7.54M | 2.00 |
| create | 4 | 0 | 4200 | 123.81K | 134.56M | 32.01K |
| create2 | 4 | 0 | 4200 | 200.71K | 134.68M | 32.02K |
| delegatecall | 4 | 0 | 49500 | 1.06M | 6.01M | 100.05 |
| div | 4 | 0 | 210000 | 2.25M | 3.30M | 5.00 |
| dup1 | 6 | 0 | 900000 | 4.10M | 6.80M | 3.00 |
| dup16 | 6 | 0 | 600000 | 2.90M | 4.70M | 3.00 |
| dup8 | 6 | 0 | 900000 | 4.20M | 6.90M | 3.00 |
| ecrecover | 4 | 0 | 201 | 71.36K | 674.36K | 3.00K |
| eq | 4 | 0 | 450000 | 4.76M | 6.11M | 3.00 |
| exp | 4 | 0 | 2250 | 70.29K | 3.69M | 1.61K |
| extcodecopy | 4 | 0 | 102000 | 1.59M | 14.28M | 124.37 |
| extcodehash | 5 | 0 | 200000 | 1.95M | 21.95M | 100.00 |
| extcodesize | 4 | 0 | 198000 | 1.93M | 21.73M | 100.00 |
| gas | 4 | 0 | 1200000 | 5.14M | 7.54M | 2.00 |
| gaslimit | 4 | 0 | 1200000 | 5.14M | 7.54M | 2.00 |
| gasprice | 4 | 0 | 1200000 | 5.14M | 7.54M | 2.00 |
| gt | 4 | 0 | 600000 | 6.34M | 8.14M | 3.00 |
| identity | 4 | 0 | 1500000 | 219.28M | 259.80M | 27.02 |
| iszero | 6 | 0 | 1350000 | 9.93M | 13.98M | 3.00 |
| jump | 6 | 0 | 600000 | 2.88M | 7.68M | 8.00 |
| jumpdest | 4 | 0 | 2400000 | 625.61K | 3.03M | 1.00 |
| jumpi | 4 | 0 | 1005000 | 7.81M | 17.86M | 10.00 |
| keccak256 | 4 | 0 | 24000 | 930.61K | 38.51M | 1.57K |
| log0 | 6 | 0 | 120000 | 957.61K | 45.96M | 375.00 |
| log1 | 4 | 0 | 81000 | 934.61K | 82.44M | 1.01K |
| log2 | 4 | 0 | 42000 | 661.61K | 133.62M | 3.17K |
| log3 | 4 | 0 | 35700 | 693.11K | 127.10M | 3.54K |
| log4 | 4 | 0 | 27300 | 649.01K | 107.56M | 3.92K |
| lt | 4 | 0 | 600000 | 6.34M | 8.14M | 3.00 |
| mcopy | 4 | 0 | 60000 | 642.01K | 6.60M | 99.33 |
| mload | 4 | 0 | 1002000 | 6.37M | 9.38M | 3.00 |
| mod | 4 | 0 | 210000 | 2.25M | 3.30M | 5.00 |
| modexp | 4 | 0 | 36 | 66.98K | 116.12K | 1.36K |
| msize | 5 | 0 | 1200000 | 5.16M | 7.56M | 2.00 |
| mstore | 4 | 0 | 501000 | 2.71M | 4.21M | 3.00 |
| mstore8 | 4 | 0 | 1002000 | 5.37M | 8.38M | 3.00 |
| mul | 4 | 0 | 600000 | 6.34M | 9.34M | 5.00 |
| mulmod | 4 | 0 | 210000 | 2.92M | 4.60M | 8.00 |
| not | 6 | 0 | 900000 | 6.64M | 9.34M | 3.00 |
| number | 4 | 0 | 1200000 | 5.14M | 7.54M | 2.00 |
| or | 6 | 0 | 900000 | 9.48M | 12.18M | 3.00 |
| origin | 4 | 0 | 1200000 | 5.14M | 7.54M | 2.00 |
| pc | 4 | 0 | 1200000 | 5.14M | 7.54M | 2.00 |
| point_evaluation | 4 | 0 | 9 | 49.92K | 499.92K | 50.00K |
| pop | 6 | 0 | 1800000 | 4.08M | 7.68M | 2.00 |
| prevrandao | 4 | 0 | 900000 | 3.86M | 5.66M | 2.00 |
| push0 | 4 | 0 | 1200000 | 5.14M | 7.54M | 2.00 |
| push1 | 6 | 0 | 1200000 | 5.14M | 8.74M | 3.00 |
| push16 | 4 | 0 | 1002000 | 4.34M | 7.35M | 3.00 |
| push32 | 6 | 0 | 525000 | 2.36M | 3.94M | 3.00 |
| returndatacopy | 4 | 0 | 180000 | 1.67M | 3.30M | 9.02 |
| returndatasize | 4 | 0 | 1200000 | 5.14M | 7.54M | 2.00 |
| ripemd160 | 4 | 0 | 1500 | 240.82K | 6.90M | 4.44K |
| sar | 4 | 0 | 450000 | 4.76M | 6.11M | 3.00 |
| sdiv | 4 | 0 | 210000 | 2.25M | 3.30M | 5.00 |
| selfbalance | 4 | 0 | 600000 | 2.59M | 5.59M | 5.00 |
| sgt | 6 | 0 | 900000 | 9.48M | 12.18M | 3.00 |
| sha256 | 4 | 0 | 1500 | 290.57K | 2.68M | 1.60K |
| shl | 4 | 0 | 450000 | 4.76M | 6.11M | 3.00 |
| shr | 4 | 0 | 600000 | 6.34M | 8.14M | 3.00 |
| signextend | 4 | 0 | 1500000 | 15.77M | 23.27M | 5.00 |
| sload | 4 | 0 | 198000 | 1.93M | 21.73M | 100.00 |
| slt | 6 | 0 | 2100000 | 22.06M | 28.36M | 3.00 |
| smod | 4 | 0 | 600000 | 6.34M | 9.34M | 5.00 |
| sstore | 4 | 0 | 61200 | 718.41K | 6.86M | 100.29 |
| staticcall | 4 | 0 | 49500 | 1.06M | 6.01M | 100.05 |
| sub | 6 | 0 | 1500000 | 15.77M | 20.27M | 3.00 |
| swap1 | 6 | 0 | 300000 | 200.61K | 1.10M | 3.00 |
| swap16 | 4 | 0 | 300000 | 275.61K | 1.18M | 3.00 |
| swap8 | 4 | 0 | 300000 | 235.61K | 1.14M | 3.00 |
| timestamp | 4 | 0 | 1200000 | 5.14M | 7.54M | 2.00 |
| tload | 4 | 0 | 1500000 | 9.77M | 159.77M | 100.00 |
| tstore | 4 | 0 | 198000 | 1.33M | 21.13M | 100.00 |
| xor | 4 | 0 | 1500000 | 15.77M | 20.27M | 3.00 |

### risc0 Max ZK Cycles by Opcode (colored by R²)

Bar length = Max ZK Cycles, color = Time/Gas R² (green=high, red=low)

<div style="overflow-x: auto;"><svg width="980" height="2560" xmlns="http://www.w3.org/2000/svg"><text x="490.0" y="20" text-anchor="middle" style="font-family: sans-serif; font-size: 14px; font-weight: bold; fill: #333;">risc0 Max ZK Cycles by Opcode (colored by Time/Gas R²)</text><text x="175" y="50" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">identity</text><rect x="180" y="35" width="600.0" height="20" fill="#eab308" rx="2"/><text x="785" y="50" style="font-family: monospace; font-size: 11px; fill: #666;">10,556,833,087 (R²=0.9274)</text><text x="175" y="74" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">keccak256</text><rect x="180" y="59" width="174.45206169527077" height="20" fill="#22c55e" rx="2"/><text x="785" y="74" style="font-family: monospace; font-size: 11px; fill: #666;">3,069,435,495 (R²=0.9980)</text><text x="175" y="98" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">slt</text><rect x="180" y="83" width="61.365454683351096" height="20" fill="#22c55e" rx="2"/><text x="785" y="98" style="font-family: monospace; font-size: 11px; fill: #666;">1,079,708,104 (R²=0.9963)</text><text x="175" y="122" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">mulmod</text><rect x="180" y="107" width="46.86069128147806" height="20" fill="#22c55e" rx="2"/><text x="785" y="122" style="font-family: monospace; font-size: 11px; fill: #666;">824,500,827 (R²=1.0000)</text><text x="175" y="146" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">selfbalance</text><rect x="180" y="131" width="44.71586833946501" height="20" fill="#22c55e" rx="2"/><text x="785" y="146" style="font-family: monospace; font-size: 11px; fill: #666;">786,763,264 (R²=0.9999)</text><text x="175" y="170" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">sub</text><rect x="180" y="155" width="43.68727662919428" height="20" fill="#22c55e" rx="2"/><text x="785" y="170" style="font-family: monospace; font-size: 11px; fill: #666;">768,665,479 (R²=0.9927)</text><text x="175" y="194" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">signextend</text><rect x="180" y="179" width="41.96519921732471" height="20" fill="#22c55e" rx="2"/><text x="785" y="194" style="font-family: monospace; font-size: 11px; fill: #666;">738,366,006 (R²=0.9997)</text><text x="175" y="218" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">xor</text><rect x="180" y="203" width="40.27719276187131" height="20" fill="#22c55e" rx="2"/><text x="785" y="218" style="font-family: monospace; font-size: 11px; fill: #666;">708,666,002 (R²=0.9994)</text><text x="175" y="242" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">log3</text><rect x="180" y="227" width="36.6107132522479" height="20" fill="#84cc16" rx="2"/><text x="785" y="242" style="font-family: monospace; font-size: 11px; fill: #666;">644,155,315 (R²=0.9874)</text><text x="175" y="266" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">log2</text><rect x="180" y="251" width="36.115142018253266" height="20" fill="#22c55e" rx="2"/><text x="785" y="266" style="font-family: monospace; font-size: 11px; fill: #666;">635,435,877 (R²=0.9985)</text><text x="175" y="290" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">tload</text><rect x="180" y="275" width="35.32214363218339" height="20" fill="#22c55e" rx="2"/><text x="785" y="290" style="font-family: monospace; font-size: 11px; fill: #666;">621,483,291 (R²=0.9999)</text><text x="175" y="314" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">log4</text><rect x="180" y="299" width="32.775176224589295" height="20" fill="#22c55e" rx="2"/><text x="785" y="314" style="font-family: monospace; font-size: 11px; fill: #666;">576,670,108 (R²=0.9983)</text><text x="175" y="338" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">log1</text><rect x="180" y="323" width="32.49032557144189" height="20" fill="#22c55e" rx="2"/><text x="785" y="338" style="font-family: monospace; font-size: 11px; fill: #666;">571,658,240 (R²=0.9959)</text><text x="175" y="362" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">balance</text><rect x="180" y="347" width="31.943860381317403" height="20" fill="#22c55e" rx="2"/><text x="785" y="362" style="font-family: monospace; font-size: 11px; fill: #666;">562,043,337 (R²=0.9999)</text><text x="175" y="386" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">extcodehash</text><rect x="180" y="371" width="28.585686172457716" height="20" fill="#22c55e" rx="2"/><text x="785" y="386" style="font-family: monospace; font-size: 11px; fill: #666;">502,957,196 (R²=0.9998)</text><text x="175" y="410" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">smod</text><rect x="180" y="395" width="27.519745079398543" height="20" fill="#22c55e" rx="2"/><text x="785" y="410" style="font-family: monospace; font-size: 11px; fill: #666;">484,202,259 (R²=0.9996)</text><text x="175" y="434" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">sgt</text><rect x="180" y="419" width="26.350168588205886" height="20" fill="#84cc16" rx="2"/><text x="785" y="434" style="font-family: monospace; font-size: 11px; fill: #666;">463,623,886 (R²=0.9896)</text><text x="175" y="458" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">blake2f</text><rect x="180" y="443" width="26.222003901992736" height="20" fill="#22c55e" rx="2"/><text x="785" y="458" style="font-family: monospace; font-size: 11px; fill: #666;">461,368,864 (R²=1.0000)</text><text x="175" y="482" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">add</text><rect x="180" y="467" width="25.685176318067136" height="20" fill="#22c55e" rx="2"/><text x="785" y="482" style="font-family: monospace; font-size: 11px; fill: #666;">451,923,532 (R²=0.9991)</text><text x="175" y="506" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">addmod</text><rect x="180" y="491" width="24.368632683677856" height="20" fill="#22c55e" rx="2"/><text x="785" y="506" style="font-family: monospace; font-size: 11px; fill: #666;">428,759,313 (R²=0.9999)</text><text x="175" y="530" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">or</text><rect x="180" y="515" width="24.201806743977365" height="20" fill="#84cc16" rx="2"/><text x="785" y="530" style="font-family: monospace; font-size: 11px; fill: #666;">425,824,057 (R²=0.9728)</text><text x="175" y="554" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">iszero</text><rect x="180" y="539" width="24.06972088181506" height="20" fill="#22c55e" rx="2"/><text x="785" y="554" style="font-family: monospace; font-size: 11px; fill: #666;">423,500,043 (R²=0.9938)</text><text x="175" y="578" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">mul</text><rect x="180" y="563" width="24.041469341079107" height="20" fill="#22c55e" rx="2"/><text x="785" y="578" style="font-family: monospace; font-size: 11px; fill: #666;">423,002,965 (R²=0.9970)</text><text x="175" y="602" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">div</text><rect x="180" y="587" width="23.164655591755118" height="20" fill="#22c55e" rx="2"/><text x="785" y="602" style="font-family: monospace; font-size: 11px; fill: #666;">407,575,671 (R²=0.9999)</text><text x="175" y="626" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">tstore</text><rect x="180" y="611" width="22.28844429583241" height="20" fill="#22c55e" rx="2"/><text x="785" y="626" style="font-family: monospace; font-size: 11px; fill: #666;">392,158,977 (R²=1.0000)</text><text x="175" y="650" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">extcodesize</text><rect x="180" y="635" width="21.912549804814393" height="20" fill="#22c55e" rx="2"/><text x="785" y="650" style="font-family: monospace; font-size: 11px; fill: #666;">385,545,218 (R²=0.9996)</text><text x="175" y="674" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">call</text><rect x="180" y="659" width="21.34266168112998" height="20" fill="#22c55e" rx="2"/><text x="785" y="674" style="font-family: monospace; font-size: 11px; fill: #666;">375,518,195 (R²=1.0000)</text><text x="175" y="698" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">callcode</text><rect x="180" y="683" width="21.17686372017184" height="20" fill="#22c55e" rx="2"/><text x="785" y="698" style="font-family: monospace; font-size: 11px; fill: #666;">372,601,026 (R²=0.9999)</text><text x="175" y="722" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">log0</text><rect x="180" y="707" width="21.121418228595317" height="20" fill="#84cc16" rx="2"/><text x="785" y="722" style="font-family: monospace; font-size: 11px; fill: #666;">371,625,478 (R²=0.9807)</text><text x="175" y="746" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">staticcall</text><rect x="180" y="731" width="20.998384058281548" height="20" fill="#22c55e" rx="2"/><text x="785" y="746" style="font-family: monospace; font-size: 11px; fill: #666;">369,460,726 (R²=0.9999)</text><text x="175" y="770" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">blobhash</text><rect x="180" y="755" width="20.64159779776577" height="20" fill="#22c55e" rx="2"/><text x="785" y="770" style="font-family: monospace; font-size: 11px; fill: #666;">363,183,171 (R²=0.9981)</text><text x="175" y="794" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">codecopy</text><rect x="180" y="779" width="20.626289854685847" height="20" fill="#22c55e" rx="2"/><text x="785" y="794" style="font-family: monospace; font-size: 11px; fill: #666;">362,913,832 (R²=0.9998)</text><text x="175" y="818" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">mod</text><rect x="180" y="803" width="20.252407614863337" height="20" fill="#22c55e" rx="2"/><text x="785" y="818" style="font-family: monospace; font-size: 11px; fill: #666;">356,335,478 (R²=1.0000)</text><text x="175" y="842" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">sdiv</text><rect x="180" y="827" width="20.204649144511002" height="20" fill="#22c55e" rx="2"/><text x="785" y="842" style="font-family: monospace; font-size: 11px; fill: #666;">355,495,181 (R²=1.0000)</text><text x="175" y="866" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">sload</text><rect x="180" y="851" width="20.131392629642704" height="20" fill="#22c55e" rx="2"/><text x="785" y="866" style="font-family: monospace; font-size: 11px; fill: #666;">354,206,253 (R²=0.9999)</text><text x="175" y="890" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">exp</text><rect x="180" y="875" width="19.27875698353362" height="20" fill="#22c55e" rx="2"/><text x="785" y="890" style="font-family: monospace; font-size: 11px; fill: #666;">339,204,366 (R²=0.9997)</text><text x="175" y="914" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">mload</text><rect x="180" y="899" width="18.914215670027485" height="20" fill="#22c55e" rx="2"/><text x="785" y="914" style="font-family: monospace; font-size: 11px; fill: #666;">332,790,363 (R²=0.9983)</text><text x="175" y="938" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">eq</text><rect x="180" y="923" width="18.07880155224055" height="20" fill="#22c55e" rx="2"/><text x="785" y="938" style="font-family: monospace; font-size: 11px; fill: #666;">318,091,484 (R²=0.9993)</text><text x="175" y="962" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">delegatecall</text><rect x="180" y="947" width="17.955334070159672" height="20" fill="#22c55e" rx="2"/><text x="785" y="962" style="font-family: monospace; font-size: 11px; fill: #666;">315,919,108 (R²=0.9998)</text><text x="175" y="986" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">jumpi</text><rect x="180" y="971" width="16.54377813504214" height="20" fill="#22c55e" rx="2"/><text x="785" y="986" style="font-family: monospace; font-size: 11px; fill: #666;">291,083,174 (R²=0.9978)</text><text x="175" y="1010" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">gt</text><rect x="180" y="995" width="16.50507067451563" height="20" fill="#22c55e" rx="2"/><text x="785" y="1010" style="font-family: monospace; font-size: 11px; fill: #666;">290,402,127 (R²=0.9983)</text><text x="175" y="1034" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">lt</text><rect x="180" y="1019" width="16.504229797339033" height="20" fill="#eab308" rx="2"/><text x="785" y="1034" style="font-family: monospace; font-size: 11px; fill: #666;">290,387,332 (R²=0.9483)</text><text x="175" y="1058" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">and</text><rect x="180" y="1043" width="16.16410091868681" height="20" fill="#22c55e" rx="2"/><text x="785" y="1058" style="font-family: monospace; font-size: 11px; fill: #666;">284,402,859 (R²=0.9966)</text><text x="175" y="1082" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">shr</text><rect x="180" y="1067" width="15.643205840145532" height="20" fill="#22c55e" rx="2"/><text x="785" y="1082" style="font-family: monospace; font-size: 11px; fill: #666;">275,237,855 (R²=0.9984)</text><text x="175" y="1106" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">not</text><rect x="180" y="1091" width="15.563109887786368" height="20" fill="#22c55e" rx="2"/><text x="785" y="1106" style="font-family: monospace; font-size: 11px; fill: #666;">273,828,589 (R²=0.9948)</text><text x="175" y="1130" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">coinbase</text><rect x="180" y="1115" width="14.816065510461545" height="20" fill="#22c55e" rx="2"/><text x="785" y="1130" style="font-family: monospace; font-size: 11px; fill: #666;">260,684,551 (R²=0.9983)</text><text x="175" y="1154" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">origin</text><rect x="180" y="1139" width="14.816033114382423" height="20" fill="#22c55e" rx="2"/><text x="785" y="1154" style="font-family: monospace; font-size: 11px; fill: #666;">260,683,981 (R²=0.9995)</text><text x="175" y="1178" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">caller</text><rect x="180" y="1163" width="14.816019587598506" height="20" fill="#22c55e" rx="2"/><text x="785" y="1178" style="font-family: monospace; font-size: 11px; fill: #666;">260,683,743 (R²=0.9998)</text><text x="175" y="1202" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">extcodecopy</text><rect x="180" y="1187" width="14.034879928380551" height="20" fill="#22c55e" rx="2"/><text x="785" y="1202" style="font-family: monospace; font-size: 11px; fill: #666;">246,939,808 (R²=0.9999)</text><text x="175" y="1226" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">sstore</text><rect x="180" y="1211" width="13.976611033255413" height="20" fill="#22c55e" rx="2"/><text x="785" y="1226" style="font-family: monospace; font-size: 11px; fill: #666;">245,914,583 (R²=0.9994)</text><text x="175" y="1250" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">sar</text><rect x="180" y="1235" width="13.824364352195426" height="20" fill="#22c55e" rx="2"/><text x="785" y="1250" style="font-family: monospace; font-size: 11px; fill: #666;">243,235,845 (R²=0.9998)</text><text x="175" y="1274" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">calldatacopy</text><rect x="180" y="1259" width="13.774167309613352" height="20" fill="#22c55e" rx="2"/><text x="785" y="1274" style="font-family: monospace; font-size: 11px; fill: #666;">242,352,642 (R²=0.9999)</text><text x="175" y="1298" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">prevrandao</text><rect x="180" y="1283" width="13.226927910032986" height="20" fill="#22c55e" rx="2"/><text x="785" y="1298" style="font-family: monospace; font-size: 11px; fill: #666;">232,724,117 (R²=0.9998)</text><text x="175" y="1322" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">byte</text><rect x="180" y="1307" width="13.188411018021053" height="20" fill="#22c55e" rx="2"/><text x="785" y="1322" style="font-family: monospace; font-size: 11px; fill: #666;">232,046,423 (R²=0.9964)</text><text x="175" y="1346" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">blobbasefee</text><rect x="180" y="1331" width="12.70177178088787" height="20" fill="#22c55e" rx="2"/><text x="785" y="1346" style="font-family: monospace; font-size: 11px; fill: #666;">223,484,141 (R²=0.9997)</text><text x="175" y="1370" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">gasprice</text><rect x="180" y="1355" width="12.428991111129424" height="20" fill="#22c55e" rx="2"/><text x="785" y="1370" style="font-family: monospace; font-size: 11px; fill: #666;">218,684,641 (R²=0.9986)</text><text x="175" y="1394" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">callvalue</text><rect x="180" y="1379" width="12.224358719748697" height="20" fill="#22c55e" rx="2"/><text x="785" y="1394" style="font-family: monospace; font-size: 11px; fill: #666;">215,084,191 (R²=0.9978)</text><text x="175" y="1418" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">shl</text><rect x="180" y="1403" width="11.726640478236444" height="20" fill="#22c55e" rx="2"/><text x="785" y="1418" style="font-family: monospace; font-size: 11px; fill: #666;">206,326,977 (R²=0.9991)</text><text x="175" y="1442" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">bn128_pairing</text><rect x="180" y="1427" width="11.68126215350098" height="20" fill="#22c55e" rx="2"/><text x="785" y="1442" style="font-family: monospace; font-size: 11px; fill: #666;">205,528,558 (R²=0.9968)</text><text x="175" y="1466" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">mstore</text><rect x="180" y="1451" width="11.60886277068406" height="20" fill="#22c55e" rx="2"/><text x="785" y="1466" style="font-family: monospace; font-size: 11px; fill: #666;">204,254,711 (R²=0.9961)</text><text x="175" y="1490" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">timestamp</text><rect x="180" y="1475" width="11.337755102653922" height="20" fill="#22c55e" rx="2"/><text x="785" y="1490" style="font-family: monospace; font-size: 11px; fill: #666;">199,484,647 (R²=0.9987)</text><text x="175" y="1514" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">number</text><rect x="180" y="1499" width="11.33774742989834" height="20" fill="#22c55e" rx="2"/><text x="785" y="1514" style="font-family: monospace; font-size: 11px; fill: #666;">199,484,512 (R²=0.9999)</text><text x="175" y="1538" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">mstore8</text><rect x="180" y="1523" width="11.32871688075407" height="20" fill="#22c55e" rx="2"/><text x="785" y="1538" style="font-family: monospace; font-size: 11px; fill: #666;">199,325,622 (R²=0.9980)</text><text x="175" y="1562" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">calldatasize</text><rect x="180" y="1547" width="11.269555558901867" height="20" fill="#22c55e" rx="2"/><text x="785" y="1562" style="font-family: monospace; font-size: 11px; fill: #666;">198,284,695 (R²=0.9972)</text><text x="175" y="1586" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">pc</text><rect x="180" y="1571" width="11.26952566357271" height="20" fill="#22c55e" rx="2"/><text x="785" y="1586" style="font-family: monospace; font-size: 11px; fill: #666;">198,284,169 (R²=1.0000)</text><text x="175" y="1610" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">basefee</text><rect x="180" y="1595" width="11.201348683405588" height="20" fill="#22c55e" rx="2"/><text x="785" y="1610" style="font-family: monospace; font-size: 11px; fill: #666;">197,084,614 (R²=0.9987)</text><text x="175" y="1634" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">chainid</text><rect x="180" y="1619" width="11.20130827355952" height="20" fill="#22c55e" rx="2"/><text x="785" y="1634" style="font-family: monospace; font-size: 11px; fill: #666;">197,083,903 (R²=0.9979)</text><text x="175" y="1658" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">address</text><rect x="180" y="1643" width="11.129701060130062" height="20" fill="#22c55e" rx="2"/><text x="785" y="1658" style="font-family: monospace; font-size: 11px; fill: #666;">195,823,994 (R²=0.9969)</text><text x="175" y="1682" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">codesize</text><rect x="180" y="1667" width="11.064862979016238" height="20" fill="#22c55e" rx="2"/><text x="785" y="1682" style="font-family: monospace; font-size: 11px; fill: #666;">194,683,186 (R²=0.9992)</text><text x="175" y="1706" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">msize</text><rect x="180" y="1691" width="11.046552487753662" height="20" fill="#22c55e" rx="2"/><text x="785" y="1706" style="font-family: monospace; font-size: 11px; fill: #666;">194,361,018 (R²=0.9998)</text><text x="175" y="1730" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">modexp</text><rect x="180" y="1715" width="11.007730504245682" height="20" fill="#22c55e" rx="2"/><text x="785" y="1730" style="font-family: monospace; font-size: 11px; fill: #666;">193,677,956 (R²=0.9998)</text><text x="175" y="1754" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">gaslimit</text><rect x="180" y="1739" width="10.996753632768694" height="20" fill="#22c55e" rx="2"/><text x="785" y="1754" style="font-family: monospace; font-size: 11px; fill: #666;">193,484,821 (R²=0.9953)</text><text x="175" y="1778" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">gas</text><rect x="180" y="1763" width="10.996735274990524" height="20" fill="#22c55e" rx="2"/><text x="785" y="1778" style="font-family: monospace; font-size: 11px; fill: #666;">193,484,498 (R²=0.9961)</text><text x="175" y="1802" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">returndatasize</text><rect x="180" y="1787" width="10.996663321593728" height="20" fill="#22c55e" rx="2"/><text x="785" y="1802" style="font-family: monospace; font-size: 11px; fill: #666;">193,483,232 (R²=0.9996)</text><text x="175" y="1826" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">push0</text><rect x="180" y="1811" width="10.792049685837757" height="20" fill="#22c55e" rx="2"/><text x="785" y="1826" style="font-family: monospace; font-size: 11px; fill: #666;">189,883,112 (R²=0.9990)</text><text x="175" y="1850" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">push16</text><rect x="180" y="1835" width="10.77371935908112" height="20" fill="#22c55e" rx="2"/><text x="785" y="1850" style="font-family: monospace; font-size: 11px; fill: #666;">189,560,595 (R²=0.9999)</text><text x="175" y="1874" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">create2</text><rect x="180" y="1859" width="10.754100691409368" height="20" fill="#22c55e" rx="2"/><text x="785" y="1874" style="font-family: monospace; font-size: 11px; fill: #666;">189,215,410 (R²=0.9999)</text><text x="175" y="1898" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">pop</text><rect x="180" y="1883" width="10.32587959870621" height="20" fill="#84cc16" rx="2"/><text x="785" y="1898" style="font-family: monospace; font-size: 11px; fill: #666;">181,680,979 (R²=0.9867)</text><text x="175" y="1922" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">create</text><rect x="180" y="1907" width="10.088936494710348" height="20" fill="#22c55e" rx="2"/><text x="785" y="1922" style="font-family: monospace; font-size: 11px; fill: #666;">177,512,031 (R²=0.9998)</text><text x="175" y="1946" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">push1</text><rect x="180" y="1931" width="10.042443972193874" height="20" fill="#22c55e" rx="2"/><text x="785" y="1946" style="font-family: monospace; font-size: 11px; fill: #666;">176,694,008 (R²=0.9935)</text><text x="175" y="1970" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">calldataload</text><rect x="180" y="1955" width="9.433545285719832" height="20" fill="#22c55e" rx="2"/><text x="785" y="1970" style="font-family: monospace; font-size: 11px; fill: #666;">165,980,605 (R²=0.9970)</text><text x="175" y="1994" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">dup8</text><rect x="180" y="1979" width="8.582549051720173" height="20" fill="#84cc16" rx="2"/><text x="785" y="1994" style="font-family: monospace; font-size: 11px; fill: #666;">151,007,563 (R²=0.9816)</text><text x="175" y="2018" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">dup1</text><rect x="180" y="2003" width="8.49178126254484" height="20" fill="#22c55e" rx="2"/><text x="785" y="2018" style="font-family: monospace; font-size: 11px; fill: #666;">149,410,529 (R²=0.9909)</text><text x="175" y="2042" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">push32</text><rect x="180" y="2027" width="7.618557623975438" height="20" fill="#22c55e" rx="2"/><text x="785" y="2042" style="font-family: monospace; font-size: 11px; fill: #666;">134,046,402 (R²=0.9954)</text><text x="175" y="2066" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">ecrecover</text><rect x="180" y="2051" width="6.786896620372795" height="20" fill="#22c55e" rx="2"/><text x="785" y="2066" style="font-family: monospace; font-size: 11px; fill: #666;">119,413,558 (R²=0.9921)</text><text x="175" y="2090" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">jump</text><rect x="180" y="2075" width="6.301804438105918" height="20" fill="#22c55e" rx="2"/><text x="785" y="2090" style="font-family: monospace; font-size: 11px; fill: #666;">110,878,496 (R²=0.9976)</text><text x="175" y="2114" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">dup16</text><rect x="180" y="2099" width="5.813952223568011" height="20" fill="#84cc16" rx="2"/><text x="785" y="2114" style="font-family: monospace; font-size: 11px; fill: #666;">102,294,872 (R²=0.9768)</text><text x="175" y="2138" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">point_evaluation</text><rect x="180" y="2123" width="5.777342778593843" height="20" fill="#22c55e" rx="2"/><text x="785" y="2138" style="font-family: monospace; font-size: 11px; fill: #666;">101,650,739 (R²=0.9945)</text><text x="175" y="2162" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">mcopy</text><rect x="180" y="2147" width="5.772509397258788" height="20" fill="#22c55e" rx="2"/><text x="785" y="2162" style="font-family: monospace; font-size: 11px; fill: #666;">101,565,697 (R²=0.9974)</text><text x="175" y="2186" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">bn128_mul</text><rect x="180" y="2171" width="5.722266090802313" height="20" fill="#22c55e" rx="2"/><text x="785" y="2186" style="font-family: monospace; font-size: 11px; fill: #666;">100,681,680 (R²=0.9997)</text><text x="175" y="2210" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">jumpdest</text><rect x="180" y="2195" width="5.416457277364169" height="20" fill="#22c55e" rx="2"/><text x="785" y="2210" style="font-family: monospace; font-size: 11px; fill: #666;">95,301,059 (R²=0.9944)</text><text x="175" y="2234" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">bls12_pairing</text><rect x="180" y="2219" width="4.715297399302341" height="20" fill="#22c55e" rx="2"/><text x="785" y="2234" style="font-family: monospace; font-size: 11px; fill: #666;">82,964,346 (R²=0.9979)</text><text x="175" y="2258" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">bls12_g1msm</text><rect x="180" y="2243" width="4.622236536077195" height="20" fill="#84cc16" rx="2"/><text x="785" y="2258" style="font-family: monospace; font-size: 11px; fill: #666;">81,326,966 (R²=0.9893)</text><text x="175" y="2282" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">returndatacopy</text><rect x="180" y="2267" width="4.566062814743071" height="20" fill="#22c55e" rx="2"/><text x="785" y="2282" style="font-family: monospace; font-size: 11px; fill: #666;">80,338,605 (R²=0.9920)</text><text x="175" y="2306" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">bls12_g2msm</text><rect x="180" y="2291" width="4.560940539951534" height="20" fill="#22c55e" rx="2"/><text x="785" y="2306" style="font-family: monospace; font-size: 11px; fill: #666;">80,248,480 (R²=0.9960)</text><text x="175" y="2330" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">ripemd160</text><rect x="180" y="2315" width="4.19448279944184" height="20" fill="#22c55e" rx="2"/><text x="785" y="2330" style="font-family: monospace; font-size: 11px; fill: #666;">73,800,758 (R²=0.9982)</text><text x="175" y="2354" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">swap16</text><rect x="180" y="2339" width="3.322033313493081" height="20" fill="#22c55e" rx="2"/><text x="785" y="2354" style="font-family: monospace; font-size: 11px; fill: #666;">58,450,252 (R²=0.9957)</text><text x="175" y="2378" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">swap8</text><rect x="180" y="2363" width="3.2858100638832233" height="20" fill="#22c55e" rx="2"/><text x="785" y="2378" style="font-family: monospace; font-size: 11px; fill: #666;">57,812,914 (R²=0.9917)</text><text x="175" y="2402" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">swap1</text><rect x="180" y="2387" width="3.2554122544876036" height="20" fill="#22c55e" rx="2"/><text x="785" y="2402" style="font-family: monospace; font-size: 11px; fill: #666;">57,278,073 (R²=0.9913)</text><text x="175" y="2426" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">bls12_map_fp2_to_g2</text><rect x="180" y="2411" width="1.5441117109328413" height="20" fill="#eab308" rx="2"/><text x="785" y="2426" style="font-family: monospace; font-size: 11px; fill: #666;">27,168,216 (R²=0.9404)</text><text x="175" y="2450" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">bls12_map_fp_to_g1</text><rect x="180" y="2435" width="1.4034310553081117" height="20" fill="#84cc16" rx="2"/><text x="785" y="2450" style="font-family: monospace; font-size: 11px; fill: #666;">24,692,979 (R²=0.9772)</text><text x="175" y="2474" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">sha256</text><rect x="180" y="2459" width="1.1178644488120435" height="20" fill="#f97316" rx="2"/><text x="785" y="2474" style="font-family: monospace; font-size: 11px; fill: #666;">19,668,514 (R²=0.8017)</text><text x="175" y="2498" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">bls12_g2add</text><rect x="180" y="2483" width="1.0580162164119284" height="20" fill="#84cc16" rx="2"/><text x="785" y="2498" style="font-family: monospace; font-size: 11px; fill: #666;">18,615,501 (R²=0.9636)</text><text x="175" y="2522" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">bls12_g1add</text><rect x="180" y="2507" width="1.0261277326947282" height="20" fill="#84cc16" rx="2"/><text x="785" y="2522" style="font-family: monospace; font-size: 11px; fill: #666;">18,054,432 (R²=0.9518)</text><text x="175" y="2546" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">bn128_add</text><rect x="180" y="2531" width="1" height="20" fill="#ef4444" rx="2"/><text x="785" y="2546" style="font-family: monospace; font-size: 11px; fill: #666;">14,612,379 (R²=0.7509)</text></svg></div>
