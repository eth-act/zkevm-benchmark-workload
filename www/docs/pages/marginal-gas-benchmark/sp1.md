# ZK Gas Benchmark Report 2025-12-29 (sp1, both)

## Context

- **Generated**: 2025-12-29 21:07:06
- **Prover**: sp1-cluster-v5.2.3
- **Mode**: both
- **CPU**: AMD EPYC 7B13 64-Core Processor
- **RAM**: 371 GiB
- **GPUs**: 4x NVIDIA GeForce RTX 4090

## Regression Results

### Time/Gas Bar Chart (R² ≥ 0.7)

*Only opcodes/precompiles with R² ≥ 0.7 are shown.*

![Time/Gas Bar Chart](/marginal-gas-benchmark/sp1/plots/bar_time_per_gas.png)

### Cycles/Gas Bar Chart (R² ≥ 0.9)

*Only opcodes/precompiles with R² ≥ 0.9 (green) are shown.*

![Cycles/Gas Bar Chart](/marginal-gas-benchmark/sp1/plots/bar_cycles_per_gas.png)

### Regression Results

| Opcode | Max Ops | Max Gas | Max ZK Cycles | Time/Gas (R²) | Cycles/Gas (R²) | Time/Cycle (R²) |
|--------|---------|---------|---------------|---------------|-----------------|-----------------|
| modexp | 36.00 | 116.12K | 187.44M | 554.91µs (<span style="color: #28a745; font-weight: bold;">0.9945</span>) | 3.79K (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.15µs (<span style="color: #28a745; font-weight: bold;">0.9945</span>) |
| point_evaluation | 9.00 | 499.92K | 1.33B | 398.23µs (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 2.96K (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.13µs (<span style="color: #28a745; font-weight: bold;">1.0000</span>) |
| bls12_map_fp_to_g1 | 75.00 | 468.01K | 481.23M | 159.35µs (<span style="color: #28a745; font-weight: bold;">0.9993</span>) | 1.16K (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.14µs (<span style="color: #28a745; font-weight: bold;">0.9993</span>) |
| bls12_pairing | 9.00 | 983.01K | 922.93M | 133.93µs (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 995.58 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.13µs (<span style="color: #28a745; font-weight: bold;">1.0000</span>) |
| bls12_g1add | 771.00 | 429.98K | 241.74M | 112.19µs (<span style="color: #28a745; font-weight: bold;">0.9995</span>) | 816.57 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.14µs (<span style="color: #28a745; font-weight: bold;">0.9995</span>) |
| bls12_map_fp2_to_g2 | 27.00 | 693.24K | 534.42M | 112.10µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) | 830.06 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.14µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) |
| bls12_g2add | 450.00 | 376.04K | 222.44M | 110.98µs (<span style="color: #28a745; font-weight: bold;">0.9986</span>) | 810.27 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.14µs (<span style="color: #28a745; font-weight: bold;">0.9986</span>) |
| blake2f | 10.00 | 704.62K | 392.21M | 81.60µs (<span style="color: #28a745; font-weight: bold;">0.9995</span>) | 597.02 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.14µs (<span style="color: #28a745; font-weight: bold;">0.9995</span>) |
| mulmod | 210.00K | 4.60M | 815.00M | 52.68µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) | 422.25 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.12µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) |
| bls12_g1msm | 39.00 | 942.30K | 331.18M | 51.92µs (<span style="color: #28a745; font-weight: bold;">0.9974</span>) | 371.60 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.14µs (<span style="color: #28a745; font-weight: bold;">0.9974</span>) |
| bn128_mul | 72.00 | 487.85K | 34.78M | 40.27µs (<span style="color: #28a745; font-weight: bold;">0.9977</span>) | 77.49 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.52µs (<span style="color: #28a745; font-weight: bold;">0.9977</span>) |
| bls12_g2msm | 24.00 | 1.14M | 304.01M | 39.40µs (<span style="color: #28a745; font-weight: bold;">0.9966</span>) | 280.55 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.14µs (<span style="color: #28a745; font-weight: bold;">0.9966</span>) |
| div | 210.00K | 3.30M | 398.40M | 38.98µs (<span style="color: #28a745; font-weight: bold;">0.9973</span>) | 306.02 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.13µs (<span style="color: #28a745; font-weight: bold;">0.9973</span>) |
| bn128_add | 801.00 | 263.28K | 10.69M | 37.83µs (<span style="color: #28a745; font-weight: bold;">0.9196</span>) | 40.55 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.93µs (<span style="color: #28a745; font-weight: bold;">0.9193</span>) |
| sdiv | 210.00K | 3.30M | 350.52M | 32.61µs (<span style="color: #28a745; font-weight: bold;">0.9951</span>) | 260.42 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.13µs (<span style="color: #28a745; font-weight: bold;">0.9951</span>) |
| mod | 210.00K | 3.30M | 345.27M | 31.87µs (<span style="color: #28a745; font-weight: bold;">0.9997</span>) | 255.42 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.12µs (<span style="color: #28a745; font-weight: bold;">0.9997</span>) |
| selfbalance | 600.00K | 5.59M | 762.70M | 31.23µs (<span style="color: #28a745; font-weight: bold;">0.9980</span>) | 234.00 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.13µs (<span style="color: #28a745; font-weight: bold;">0.9980</span>) |
| addmod | 201.00K | 4.41M | 411.79M | 24.18µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) | 193.00 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.13µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) |
| bn128_pairing | 8.00 | 956.88K | 49.89M | 22.11µs (<span style="color: #28a745; font-weight: bold;">0.9909</span>) | 54.18 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.41µs (<span style="color: #28a745; font-weight: bold;">0.9909</span>) |
| eq | 450.00K | 6.11M | 297.74M | 14.68µs (<span style="color: #28a745; font-weight: bold;">0.9872</span>) | 99.03 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.15µs (<span style="color: #28a745; font-weight: bold;">0.9872</span>) |
| exp | 2.25K | 3.69M | 319.05M | 12.03µs (<span style="color: #28a745; font-weight: bold;">0.9968</span>) | 87.58 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.14µs (<span style="color: #28a745; font-weight: bold;">0.9968</span>) |
| swap16 | 300.00K | 1.18M | 57.77M | 10.79µs (<span style="color: #28a745; font-weight: bold;">0.9845</span>) | 53.34 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.20µs (<span style="color: #28a745; font-weight: bold;">0.9845</span>) |
| prevrandao | 900.00K | 5.66M | 231.90M | 10.61µs (<span style="color: #28a745; font-weight: bold;">0.9904</span>) | 78.50 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.14µs (<span style="color: #28a745; font-weight: bold;">0.9903</span>) |
| swap8 | 300.00K | 1.14M | 57.19M | 9.93µs (<span style="color: #28a745; font-weight: bold;">0.9847</span>) | 53.37 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.19µs (<span style="color: #28a745; font-weight: bold;">0.9846</span>) |
| sar | 450.00K | 6.11M | 234.20M | 9.80µs (<span style="color: #28a745; font-weight: bold;">0.9839</span>) | 78.34 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.13µs (<span style="color: #28a745; font-weight: bold;">0.9839</span>) |
| mstore | 501.00K | 4.21M | 194.41M | 9.77µs (<span style="color: #28a745; font-weight: bold;">0.9946</span>) | 62.70 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.16µs (<span style="color: #28a745; font-weight: bold;">0.9946</span>) |
| smod | 600.00K | 9.34M | 457.26M | 9.71µs (<span style="color: #28a745; font-weight: bold;">0.9969</span>) | 79.62 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.12µs (<span style="color: #28a745; font-weight: bold;">0.9969</span>) |
| sha256 | 1.50K | 2.68M | 39.93M | 9.61µs (<span style="color: #28a745; font-weight: bold;">0.9985</span>) | 12.40 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.77µs (<span style="color: #28a745; font-weight: bold;">0.9985</span>) |
| ecrecover | 201.00 | 674.36K | 8.54M | 9.55µs (<span style="color: #28a745; font-weight: bold;">0.9380</span>) | 10.67 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.89µs (<span style="color: #28a745; font-weight: bold;">0.9380</span>) |
| swap1 | 300.00K | 1.10M | 56.68M | 9.49µs (<span style="color: #28a745; font-weight: bold;">0.9781</span>) | 53.37 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.18µs (<span style="color: #28a745; font-weight: bold;">0.9784</span>) |
| call | 49.50K | 6.16M | 366.39M | 9.27µs (<span style="color: #28a745; font-weight: bold;">0.9932</span>) | 68.84 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.13µs (<span style="color: #28a745; font-weight: bold;">0.9931</span>) |
| callcode | 49.50K | 6.16M | 356.72M | 9.26µs (<span style="color: #28a745; font-weight: bold;">0.9991</span>) | 66.91 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.14µs (<span style="color: #28a745; font-weight: bold;">0.9990</span>) |
| staticcall | 49.50K | 6.01M | 353.79M | 8.97µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) | 66.80 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.13µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) |
| coinbase | 1.20M | 7.54M | 259.71M | 8.25µs (<span style="color: #28a745; font-weight: bold;">0.9857</span>) | 58.00 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.14µs (<span style="color: #28a745; font-weight: bold;">0.9856</span>) |
| caller | 1.20M | 7.54M | 259.71M | 8.17µs (<span style="color: #28a745; font-weight: bold;">0.9845</span>) | 58.00 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.14µs (<span style="color: #28a745; font-weight: bold;">0.9845</span>) |
| origin | 1.20M | 7.54M | 259.71M | 8.10µs (<span style="color: #28a745; font-weight: bold;">0.9678</span>) | 58.00 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.14µs (<span style="color: #28a745; font-weight: bold;">0.9678</span>) |
| delegatecall | 49.50K | 6.01M | 302.12M | 7.55µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) | 56.48 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.13µs (<span style="color: #28a745; font-weight: bold;">0.9999</span>) |
| push32 | 525.00K | 3.94M | 123.65M | 7.52µs (<span style="color: #28a745; font-weight: bold;">0.9244</span>) | 41.71 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.18µs (<span style="color: #28a745; font-weight: bold;">0.9244</span>) |
| address | 900.00K | 5.66M | 195.00M | 7.42µs (<span style="color: #28a745; font-weight: bold;">0.9953</span>) | 58.00 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.13µs (<span style="color: #28a745; font-weight: bold;">0.9954</span>) |
| mul | 600.00K | 9.34M | 400.26M | 7.33µs (<span style="color: #28a745; font-weight: bold;">0.9892</span>) | 60.62 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.12µs (<span style="color: #28a745; font-weight: bold;">0.9892</span>) |
| codecopy | 900.00K | 12.22M | 417.43M | 7.13µs (<span style="color: #28a745; font-weight: bold;">0.9950</span>) | 47.07 (<span style="color: #28a745; font-weight: bold;">0.9929</span>) | 0.15µs (<span style="color: #28a745; font-weight: bold;">0.9996</span>) |
| shr | 600.00K | 8.14M | 263.36M | 7.09µs (<span style="color: #28a745; font-weight: bold;">0.9971</span>) | 51.34 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.14µs (<span style="color: #28a745; font-weight: bold;">0.9971</span>) |
| keccak256 | 24.00K | 38.51M | 701.87M | 6.68µs (<span style="color: #28a745; font-weight: bold;">0.9992</span>) | 17.78 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.38µs (<span style="color: #28a745; font-weight: bold;">0.9992</span>) |
| blobbasefee | 1.20M | 7.54M | 222.51M | 6.62µs (<span style="color: #28a745; font-weight: bold;">0.9940</span>) | 42.50 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.16µs (<span style="color: #28a745; font-weight: bold;">0.9941</span>) |
| mload | 1.00M | 9.38M | 323.76M | 6.55µs (<span style="color: #28a745; font-weight: bold;">0.9755</span>) | 54.01 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.12µs (<span style="color: #28a745; font-weight: bold;">0.9755</span>) |
| gasprice | 1.20M | 7.54M | 217.71M | 6.50µs (<span style="color: #28a745; font-weight: bold;">0.9995</span>) | 40.50 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.16µs (<span style="color: #28a745; font-weight: bold;">0.9995</span>) |
| calldataload | 501.00K | 4.70M | 162.31M | 6.35µs (<span style="color: #28a745; font-weight: bold;">0.9983</span>) | 54.34 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.12µs (<span style="color: #28a745; font-weight: bold;">0.9983</span>) |
| callvalue | 1.20M | 7.54M | 214.11M | 6.31µs (<span style="color: #28a745; font-weight: bold;">0.9789</span>) | 39.00 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.16µs (<span style="color: #28a745; font-weight: bold;">0.9789</span>) |
| shl | 450.00K | 6.11M | 196.40M | 6.26µs (<span style="color: #28a745; font-weight: bold;">0.9913</span>) | 50.34 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.12µs (<span style="color: #28a745; font-weight: bold;">0.9913</span>) |
| signextend | 1.50M | 23.27M | 721.57M | 5.91µs (<span style="color: #28a745; font-weight: bold;">0.9918</span>) | 53.42 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.11µs (<span style="color: #28a745; font-weight: bold;">0.9919</span>) |
| codesize | 1.20M | 7.54M | 193.71M | 5.63µs (<span style="color: #28a745; font-weight: bold;">0.9864</span>) | 30.50 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.18µs (<span style="color: #28a745; font-weight: bold;">0.9863</span>) |
| gas | 1.20M | 7.54M | 192.51M | 5.51µs (<span style="color: #28a745; font-weight: bold;">0.9836</span>) | 30.00 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.18µs (<span style="color: #28a745; font-weight: bold;">0.9837</span>) |
| calldatasize | 1.20M | 7.54M | 197.31M | 5.48µs (<span style="color: #28a745; font-weight: bold;">0.9780</span>) | 32.00 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.17µs (<span style="color: #28a745; font-weight: bold;">0.9780</span>) |
| pc | 1.20M | 7.54M | 197.31M | 5.43µs (<span style="color: #28a745; font-weight: bold;">0.9565</span>) | 32.00 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.17µs (<span style="color: #28a745; font-weight: bold;">0.9563</span>) |
| and | 600.00K | 8.14M | 261.66M | 5.24µs (<span style="color: #28a745; font-weight: bold;">0.9897</span>) | 24.03 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.22µs (<span style="color: #28a745; font-weight: bold;">0.9896</span>) |
| chainid | 1.20M | 7.54M | 196.11M | 5.04µs (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 31.50 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.16µs (<span style="color: #28a745; font-weight: bold;">1.0000</span>) |
| push0 | 1.20M | 7.54M | 188.91M | 4.92µs (<span style="color: #28a745; font-weight: bold;">0.9425</span>) | 28.50 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.17µs (<span style="color: #28a745; font-weight: bold;">0.9421</span>) |
| xor | 1.50M | 20.27M | 652.66M | 4.91µs (<span style="color: #28a745; font-weight: bold;">0.9720</span>) | 24.03 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.20µs (<span style="color: #28a745; font-weight: bold;">0.9720</span>) |
| gaslimit | 1.20M | 7.54M | 192.51M | 4.79µs (<span style="color: #28a745; font-weight: bold;">0.9959</span>) | 30.00 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.16µs (<span style="color: #28a745; font-weight: bold;">0.9960</span>) |
| returndatasize | 1.20M | 7.54M | 192.51M | 4.70µs (<span style="color: #28a745; font-weight: bold;">0.9944</span>) | 30.00 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.16µs (<span style="color: #28a745; font-weight: bold;">0.9945</span>) |
| basefee | 1.20M | 7.54M | 196.11M | 4.68µs (<span style="color: #28a745; font-weight: bold;">0.9866</span>) | 31.50 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.15µs (<span style="color: #28a745; font-weight: bold;">0.9867</span>) |
| number | 1.20M | 7.54M | 198.51M | 4.66µs (<span style="color: #28a745; font-weight: bold;">0.9843</span>) | 32.50 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.14µs (<span style="color: #28a745; font-weight: bold;">0.9845</span>) |
| sstore | 61.20K | 6.86M | 236.88M | 4.61µs (<span style="color: #28a745; font-weight: bold;">0.9998</span>) | 34.90 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.13µs (<span style="color: #28a745; font-weight: bold;">0.9998</span>) |
| byte | 600.00K | 8.14M | 220.15M | 4.54µs (<span style="color: #28a745; font-weight: bold;">0.9691</span>) | 27.34 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.17µs (<span style="color: #28a745; font-weight: bold;">0.9691</span>) |
| timestamp | 1.20M | 7.54M | 198.51M | 4.33µs (<span style="color: #28a745; font-weight: bold;">0.9887</span>) | 32.50 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.13µs (<span style="color: #28a745; font-weight: bold;">0.9886</span>) |
| returndatacopy | 180.00K | 3.30M | 79.64M | 4.20µs (<span style="color: #28a745; font-weight: bold;">0.9857</span>) | 26.45 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.16µs (<span style="color: #28a745; font-weight: bold;">0.9857</span>) |
| calldatacopy | 600.00K | 8.16M | 240.18M | 4.14µs (<span style="color: #28a745; font-weight: bold;">0.9724</span>) | 35.33 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.12µs (<span style="color: #28a745; font-weight: bold;">0.9724</span>) |
| jumpdest | 2.40M | 3.03M | 93.71M | 4.10µs (<span style="color: #28a745; font-weight: bold;">0.9930</span>) | 26.00 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.16µs (<span style="color: #28a745; font-weight: bold;">0.9930</span>) |
| msize | 1.20M | 7.56M | 193.38M | 3.96µs (<span style="color: #28a745; font-weight: bold;">0.9917</span>) | 30.00 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.13µs (<span style="color: #28a745; font-weight: bold;">0.9917</span>) |
| lt | 600.00K | 8.14M | 267.66M | 3.95µs (<span style="color: #28a745; font-weight: bold;">0.9971</span>) | 27.36 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.14µs (<span style="color: #28a745; font-weight: bold;">0.9970</span>) |
| gt | 600.00K | 8.14M | 267.66M | 3.84µs (<span style="color: #28a745; font-weight: bold;">0.9896</span>) | 27.36 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.14µs (<span style="color: #28a745; font-weight: bold;">0.9895</span>) |
| slt | 2.10M | 28.36M | 1.00B | 3.79µs (<span style="color: #ffc107; font-weight: bold;">0.8388</span>) | 38.02 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.10µs (<span style="color: #ffc107; font-weight: bold;">0.8383</span>) |
| add | 900.00K | 12.18M | 414.49M | 3.78µs (<span style="color: #28a745; font-weight: bold;">0.9987</span>) | 32.36 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.12µs (<span style="color: #28a745; font-weight: bold;">0.9987</span>) |
| push16 | 1.00M | 7.35M | 188.49M | 3.73µs (<span style="color: #28a745; font-weight: bold;">0.9960</span>) | 28.36 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.13µs (<span style="color: #28a745; font-weight: bold;">0.9961</span>) |
| sgt | 900.00K | 12.18M | 429.79M | 3.55µs (<span style="color: #ffc107; font-weight: bold;">0.8265</span>) | 38.02 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.09µs (<span style="color: #ffc107; font-weight: bold;">0.8261</span>) |
| dup16 | 600.00K | 4.70M | 101.33M | 3.54µs (<span style="color: #28a745; font-weight: bold;">0.9449</span>) | 16.99 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.21µs (<span style="color: #28a745; font-weight: bold;">0.9461</span>) |
| mstore8 | 1.00M | 8.38M | 198.35M | 3.36µs (<span style="color: #28a745; font-weight: bold;">0.9735</span>) | 26.01 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.13µs (<span style="color: #28a745; font-weight: bold;">0.9736</span>) |
| extcodehash | 200.00K | 21.95M | 490.88M | 3.24µs (<span style="color: #28a745; font-weight: bold;">0.9982</span>) | 22.15 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.15µs (<span style="color: #28a745; font-weight: bold;">0.9982</span>) |
| dup8 | 900.00K | 6.90M | 149.81M | 3.21µs (<span style="color: #28a745; font-weight: bold;">0.9217</span>) | 17.02 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.19µs (<span style="color: #28a745; font-weight: bold;">0.9216</span>) |
| dup1 | 900.00K | 6.80M | 148.13M | 3.05µs (<span style="color: #ffc107; font-weight: bold;">0.8879</span>) | 17.00 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.18µs (<span style="color: #ffc107; font-weight: bold;">0.8879</span>) |
| or | 900.00K | 12.18M | 391.99M | 3.02µs (<span style="color: #ffc107; font-weight: bold;">0.8897</span>) | 24.02 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.13µs (<span style="color: #ffc107; font-weight: bold;">0.8900</span>) |
| push1 | 1.20M | 8.74M | 175.71M | 2.75µs (<span style="color: #28a745; font-weight: bold;">0.9551</span>) | 15.34 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.18µs (<span style="color: #28a745; font-weight: bold;">0.9551</span>) |
| sub | 1.50M | 20.27M | 712.67M | 2.59µs (<span style="color: #ffc107; font-weight: bold;">0.7971</span>) | 37.36 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.07µs (<span style="color: #ffc107; font-weight: bold;">0.7967</span>) |
| balance | 300.00K | 32.40M | 549.04M | 2.43µs (<span style="color: #28a745; font-weight: bold;">0.9993</span>) | 16.60 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.15µs (<span style="color: #28a745; font-weight: bold;">0.9993</span>) |
| extcodecopy | 102.00K | 14.28M | 241.92M | 2.30µs (<span style="color: #28a745; font-weight: bold;">0.9967</span>) | 15.75 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.15µs (<span style="color: #28a745; font-weight: bold;">0.9967</span>) |
| iszero | 1.35M | 13.98M | 388.29M | 2.30µs (<span style="color: #ffc107; font-weight: bold;">0.8634</span>) | 19.33 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.12µs (<span style="color: #ffc107; font-weight: bold;">0.8634</span>) |
| extcodesize | 198.00K | 21.73M | 376.38M | 2.25µs (<span style="color: #28a745; font-weight: bold;">0.9978</span>) | 16.66 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.13µs (<span style="color: #28a745; font-weight: bold;">0.9978</span>) |
| sload | 198.00K | 21.73M | 342.58M | 2.10µs (<span style="color: #28a745; font-weight: bold;">0.9979</span>) | 15.00 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.14µs (<span style="color: #28a745; font-weight: bold;">0.9979</span>) |
| jumpi | 1.00M | 17.86M | 282.14M | 2.09µs (<span style="color: #28a745; font-weight: bold;">0.9972</span>) | 11.60 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.18µs (<span style="color: #28a745; font-weight: bold;">0.9972</span>) |
| mcopy | 60.00K | 6.60M | 100.43M | 2.09µs (<span style="color: #28a745; font-weight: bold;">0.9951</span>) | 14.80 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.14µs (<span style="color: #28a745; font-weight: bold;">0.9948</span>) |
| tstore | 198.00K | 21.13M | 367.66M | 2.08µs (<span style="color: #28a745; font-weight: bold;">0.9995</span>) | 15.90 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.13µs (<span style="color: #28a745; font-weight: bold;">0.9995</span>) |
| identity | 1.50M | 259.80M | 10.20B | 2.06µs (<span style="color: #ffc107; font-weight: bold;">0.7010</span>) | 4.66 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.44µs (<span style="color: #dc3545; font-weight: bold;">0.6975</span>) |
| blobhash | 1.50M | 14.01M | 340.85M | 1.92µs (<span style="color: #28a745; font-weight: bold;">0.9644</span>) | 22.34 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.09µs (<span style="color: #28a745; font-weight: bold;">0.9642</span>) |
| not | 900.00K | 9.34M | 256.48M | 1.89µs (<span style="color: #28a745; font-weight: bold;">0.9312</span>) | 18.34 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.10µs (<span style="color: #28a745; font-weight: bold;">0.9312</span>) |
| ripemd160 | 1.50K | 6.90M | 70.23M | 1.55µs (<span style="color: #28a745; font-weight: bold;">0.9559</span>) | 9.01 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.17µs (<span style="color: #28a745; font-weight: bold;">0.9550</span>) |
| pop | 1.80M | 7.68M | 180.31M | 0.96µs (<span style="color: #ffc107; font-weight: bold;">0.7252</span>) | 15.00 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.06µs (<span style="color: #ffc107; font-weight: bold;">0.7252</span>) |
| jump | 600.00K | 7.68M | 109.50M | 0.88µs (<span style="color: #ffc107; font-weight: bold;">0.8975</span>) | 7.88 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.11µs (<span style="color: #ffc107; font-weight: bold;">0.8975</span>) |
| tload | 1.50M | 159.77M | 619.43M | 0.35µs (<span style="color: #28a745; font-weight: bold;">0.9982</span>) | 2.43 (<span style="color: #28a745; font-weight: bold;">1.0000</span>) | 0.14µs (<span style="color: #28a745; font-weight: bold;">0.9983</span>) |
| create | 4.20K | 134.56M | 111.70M | 0.15µs (<span style="color: #28a745; font-weight: bold;">0.9997</span>) | 0.8048 (<span style="color: #28a745; font-weight: bold;">0.9996</span>) | 0.19µs (<span style="color: #28a745; font-weight: bold;">0.9990</span>) |
| create2 | 4.20K | 134.68M | 117.66M | 0.15µs (<span style="color: #28a745; font-weight: bold;">0.9997</span>) | 0.8391 (<span style="color: #28a745; font-weight: bold;">0.9997</span>) | 0.18µs (<span style="color: #28a745; font-weight: bold;">0.9989</span>) |
| log0 | 120.00K | 45.96M | 46.49M | 0.15µs (<span style="color: #28a745; font-weight: bold;">0.9268</span>) | 0.5558 (<span style="color: #28a745; font-weight: bold;">0.9944</span>) | 0.27µs (<span style="color: #28a745; font-weight: bold;">0.9422</span>) |
| log1 | 81.00K | 82.44M | 66.44M | 0.14µs (<span style="color: #28a745; font-weight: bold;">0.9995</span>) | 0.5628 (<span style="color: #28a745; font-weight: bold;">0.9997</span>) | 0.24µs (<span style="color: #28a745; font-weight: bold;">0.9988</span>) |
| log2 | 42.00K | 133.62M | 55.47M | 0.08µs (<span style="color: #28a745; font-weight: bold;">0.9756</span>) | 0.3078 (<span style="color: #28a745; font-weight: bold;">0.9999</span>) | 0.26µs (<span style="color: #28a745; font-weight: bold;">0.9747</span>) |
| log4 | 27.30K | 107.56M | 47.82M | 0.08µs (<span style="color: #28a745; font-weight: bold;">0.9897</span>) | 0.3136 (<span style="color: #28a745; font-weight: bold;">0.9998</span>) | 0.25µs (<span style="color: #28a745; font-weight: bold;">0.9886</span>) |
| log3 | 35.70K | 127.10M | 55.01M | 0.07µs (<span style="color: #28a745; font-weight: bold;">0.9979</span>) | 0.316 (<span style="color: #28a745; font-weight: bold;">0.9999</span>) | 0.22µs (<span style="color: #28a745; font-weight: bold;">0.9981</span>) |

## Proving time vs ZK Cycles

This section examines whether ZK cycles are a good proxy for proving time.

### Time/Cycles Bar Chart (R² ≥ 0.7)

*Only opcodes/precompiles with R² ≥ 0.7 are shown.*

![Time/Cycles Bar Chart](/marginal-gas-benchmark/sp1/plots/bar_time_per_cycle.png)

### Combined ZK Cycles ↔ Proving Time (All Opcodes)

![Combined ZK Cycles vs Proving Time](/marginal-gas-benchmark/sp1/plots/combined_zkcycles_proving.png)

### Excluding: none

![Regular Opcodes](/marginal-gas-benchmark/sp1/plots/combined_zkcycles_regular.png)

### Excluding: blake2f, modexp

![Regular Opcodes No Outliers](/marginal-gas-benchmark/sp1/plots/combined_zkcycles_regular_no_outliers.png)

### Excluding: blake2f, keccak256, log0, log1, log2, log3, log4, modexp

![Regular Opcodes Minimal](/marginal-gas-benchmark/sp1/plots/combined_zkcycles_regular_minimal.png)


## Regression Charts

### sp1 Gas vs Proving Time

**modexp**: Slope = 554.91µs/gas, R² = 0.9945

![modexp](/marginal-gas-benchmark/sp1/plots/gas_proving_modexp.png)

**point_evaluation**: Slope = 398.23µs/gas, R² = 1.0000

![point_evaluation](/marginal-gas-benchmark/sp1/plots/gas_proving_point_evaluation.png)

**bls12_map_fp_to_g1**: Slope = 159.35µs/gas, R² = 0.9993

![bls12_map_fp_to_g1](/marginal-gas-benchmark/sp1/plots/gas_proving_bls12_map_fp_to_g1.png)

**bls12_pairing**: Slope = 133.93µs/gas, R² = 1.0000

![bls12_pairing](/marginal-gas-benchmark/sp1/plots/gas_proving_bls12_pairing.png)

**bls12_g1add**: Slope = 112.19µs/gas, R² = 0.9995

![bls12_g1add](/marginal-gas-benchmark/sp1/plots/gas_proving_bls12_g1add.png)

**bls12_map_fp2_to_g2**: Slope = 112.10µs/gas, R² = 0.9999

![bls12_map_fp2_to_g2](/marginal-gas-benchmark/sp1/plots/gas_proving_bls12_map_fp2_to_g2.png)

**bls12_g2add**: Slope = 110.98µs/gas, R² = 0.9986

![bls12_g2add](/marginal-gas-benchmark/sp1/plots/gas_proving_bls12_g2add.png)

**blake2f**: Slope = 81.60µs/gas, R² = 0.9995

![blake2f](/marginal-gas-benchmark/sp1/plots/gas_proving_blake2f.png)

**mulmod**: Slope = 52.68µs/gas, R² = 0.9999

![mulmod](/marginal-gas-benchmark/sp1/plots/gas_proving_mulmod.png)

**bls12_g1msm**: Slope = 51.92µs/gas, R² = 0.9974

![bls12_g1msm](/marginal-gas-benchmark/sp1/plots/gas_proving_bls12_g1msm.png)

**bn128_mul**: Slope = 40.27µs/gas, R² = 0.9977

![bn128_mul](/marginal-gas-benchmark/sp1/plots/gas_proving_bn128_mul.png)

**bls12_g2msm**: Slope = 39.40µs/gas, R² = 0.9966

![bls12_g2msm](/marginal-gas-benchmark/sp1/plots/gas_proving_bls12_g2msm.png)

**div**: Slope = 38.98µs/gas, R² = 0.9973

![div](/marginal-gas-benchmark/sp1/plots/gas_proving_div.png)

**bn128_add**: Slope = 37.83µs/gas, R² = 0.9196

![bn128_add](/marginal-gas-benchmark/sp1/plots/gas_proving_bn128_add.png)

**sdiv**: Slope = 32.61µs/gas, R² = 0.9951

![sdiv](/marginal-gas-benchmark/sp1/plots/gas_proving_sdiv.png)

**mod**: Slope = 31.87µs/gas, R² = 0.9997

![mod](/marginal-gas-benchmark/sp1/plots/gas_proving_mod.png)

**selfbalance**: Slope = 31.23µs/gas, R² = 0.9980

![selfbalance](/marginal-gas-benchmark/sp1/plots/gas_proving_selfbalance.png)

**addmod**: Slope = 24.18µs/gas, R² = 0.9999

![addmod](/marginal-gas-benchmark/sp1/plots/gas_proving_addmod.png)

**bn128_pairing**: Slope = 22.11µs/gas, R² = 0.9909

![bn128_pairing](/marginal-gas-benchmark/sp1/plots/gas_proving_bn128_pairing.png)

**eq**: Slope = 14.68µs/gas, R² = 0.9872

![eq](/marginal-gas-benchmark/sp1/plots/gas_proving_eq.png)

**exp**: Slope = 12.03µs/gas, R² = 0.9968

![exp](/marginal-gas-benchmark/sp1/plots/gas_proving_exp.png)

**swap16**: Slope = 10.79µs/gas, R² = 0.9845

![swap16](/marginal-gas-benchmark/sp1/plots/gas_proving_swap16.png)

**prevrandao**: Slope = 10.61µs/gas, R² = 0.9904

![prevrandao](/marginal-gas-benchmark/sp1/plots/gas_proving_prevrandao.png)

**swap8**: Slope = 9.93µs/gas, R² = 0.9847

![swap8](/marginal-gas-benchmark/sp1/plots/gas_proving_swap8.png)

**sar**: Slope = 9.80µs/gas, R² = 0.9839

![sar](/marginal-gas-benchmark/sp1/plots/gas_proving_sar.png)

**mstore**: Slope = 9.77µs/gas, R² = 0.9946

![mstore](/marginal-gas-benchmark/sp1/plots/gas_proving_mstore.png)

**smod**: Slope = 9.71µs/gas, R² = 0.9969

![smod](/marginal-gas-benchmark/sp1/plots/gas_proving_smod.png)

**sha256**: Slope = 9.61µs/gas, R² = 0.9985

![sha256](/marginal-gas-benchmark/sp1/plots/gas_proving_sha256.png)

**ecrecover**: Slope = 9.55µs/gas, R² = 0.9380

![ecrecover](/marginal-gas-benchmark/sp1/plots/gas_proving_ecrecover.png)

**swap1**: Slope = 9.49µs/gas, R² = 0.9781

![swap1](/marginal-gas-benchmark/sp1/plots/gas_proving_swap1.png)

**call**: Slope = 9.27µs/gas, R² = 0.9932

![call](/marginal-gas-benchmark/sp1/plots/gas_proving_call.png)

**callcode**: Slope = 9.26µs/gas, R² = 0.9991

![callcode](/marginal-gas-benchmark/sp1/plots/gas_proving_callcode.png)

**staticcall**: Slope = 8.97µs/gas, R² = 0.9999

![staticcall](/marginal-gas-benchmark/sp1/plots/gas_proving_staticcall.png)

**coinbase**: Slope = 8.25µs/gas, R² = 0.9857

![coinbase](/marginal-gas-benchmark/sp1/plots/gas_proving_coinbase.png)

**caller**: Slope = 8.17µs/gas, R² = 0.9845

![caller](/marginal-gas-benchmark/sp1/plots/gas_proving_caller.png)

**origin**: Slope = 8.10µs/gas, R² = 0.9678

![origin](/marginal-gas-benchmark/sp1/plots/gas_proving_origin.png)

**delegatecall**: Slope = 7.55µs/gas, R² = 0.9999

![delegatecall](/marginal-gas-benchmark/sp1/plots/gas_proving_delegatecall.png)

**push32**: Slope = 7.52µs/gas, R² = 0.9244

![push32](/marginal-gas-benchmark/sp1/plots/gas_proving_push32.png)

**address**: Slope = 7.42µs/gas, R² = 0.9953

![address](/marginal-gas-benchmark/sp1/plots/gas_proving_address.png)

**mul**: Slope = 7.33µs/gas, R² = 0.9892

![mul](/marginal-gas-benchmark/sp1/plots/gas_proving_mul.png)

**codecopy**: Slope = 7.13µs/gas, R² = 0.9950

![codecopy](/marginal-gas-benchmark/sp1/plots/gas_proving_codecopy.png)

**shr**: Slope = 7.09µs/gas, R² = 0.9971

![shr](/marginal-gas-benchmark/sp1/plots/gas_proving_shr.png)

**keccak256**: Slope = 6.68µs/gas, R² = 0.9992

![keccak256](/marginal-gas-benchmark/sp1/plots/gas_proving_keccak256.png)

**blobbasefee**: Slope = 6.62µs/gas, R² = 0.9940

![blobbasefee](/marginal-gas-benchmark/sp1/plots/gas_proving_blobbasefee.png)

**mload**: Slope = 6.55µs/gas, R² = 0.9755

![mload](/marginal-gas-benchmark/sp1/plots/gas_proving_mload.png)

**gasprice**: Slope = 6.50µs/gas, R² = 0.9995

![gasprice](/marginal-gas-benchmark/sp1/plots/gas_proving_gasprice.png)

**calldataload**: Slope = 6.35µs/gas, R² = 0.9983

![calldataload](/marginal-gas-benchmark/sp1/plots/gas_proving_calldataload.png)

**callvalue**: Slope = 6.31µs/gas, R² = 0.9789

![callvalue](/marginal-gas-benchmark/sp1/plots/gas_proving_callvalue.png)

**shl**: Slope = 6.26µs/gas, R² = 0.9913

![shl](/marginal-gas-benchmark/sp1/plots/gas_proving_shl.png)

**signextend**: Slope = 5.91µs/gas, R² = 0.9918

![signextend](/marginal-gas-benchmark/sp1/plots/gas_proving_signextend.png)

**codesize**: Slope = 5.63µs/gas, R² = 0.9864

![codesize](/marginal-gas-benchmark/sp1/plots/gas_proving_codesize.png)

**gas**: Slope = 5.51µs/gas, R² = 0.9836

![gas](/marginal-gas-benchmark/sp1/plots/gas_proving_gas.png)

**calldatasize**: Slope = 5.48µs/gas, R² = 0.9780

![calldatasize](/marginal-gas-benchmark/sp1/plots/gas_proving_calldatasize.png)

**pc**: Slope = 5.43µs/gas, R² = 0.9565

![pc](/marginal-gas-benchmark/sp1/plots/gas_proving_pc.png)

**and**: Slope = 5.24µs/gas, R² = 0.9897

![and](/marginal-gas-benchmark/sp1/plots/gas_proving_and.png)

**chainid**: Slope = 5.04µs/gas, R² = 1.0000

![chainid](/marginal-gas-benchmark/sp1/plots/gas_proving_chainid.png)

**push0**: Slope = 4.92µs/gas, R² = 0.9425

![push0](/marginal-gas-benchmark/sp1/plots/gas_proving_push0.png)

**xor**: Slope = 4.91µs/gas, R² = 0.9720

![xor](/marginal-gas-benchmark/sp1/plots/gas_proving_xor.png)

**gaslimit**: Slope = 4.79µs/gas, R² = 0.9959

![gaslimit](/marginal-gas-benchmark/sp1/plots/gas_proving_gaslimit.png)

**returndatasize**: Slope = 4.70µs/gas, R² = 0.9944

![returndatasize](/marginal-gas-benchmark/sp1/plots/gas_proving_returndatasize.png)

**basefee**: Slope = 4.68µs/gas, R² = 0.9866

![basefee](/marginal-gas-benchmark/sp1/plots/gas_proving_basefee.png)

**number**: Slope = 4.66µs/gas, R² = 0.9843

![number](/marginal-gas-benchmark/sp1/plots/gas_proving_number.png)

**sstore**: Slope = 4.61µs/gas, R² = 0.9998

![sstore](/marginal-gas-benchmark/sp1/plots/gas_proving_sstore.png)

**byte**: Slope = 4.54µs/gas, R² = 0.9691

![byte](/marginal-gas-benchmark/sp1/plots/gas_proving_byte.png)

**timestamp**: Slope = 4.33µs/gas, R² = 0.9887

![timestamp](/marginal-gas-benchmark/sp1/plots/gas_proving_timestamp.png)

**returndatacopy**: Slope = 4.20µs/gas, R² = 0.9857

![returndatacopy](/marginal-gas-benchmark/sp1/plots/gas_proving_returndatacopy.png)

**calldatacopy**: Slope = 4.14µs/gas, R² = 0.9724

![calldatacopy](/marginal-gas-benchmark/sp1/plots/gas_proving_calldatacopy.png)

**jumpdest**: Slope = 4.10µs/gas, R² = 0.9930

![jumpdest](/marginal-gas-benchmark/sp1/plots/gas_proving_jumpdest.png)

**msize**: Slope = 3.96µs/gas, R² = 0.9917

![msize](/marginal-gas-benchmark/sp1/plots/gas_proving_msize.png)

**lt**: Slope = 3.95µs/gas, R² = 0.9971

![lt](/marginal-gas-benchmark/sp1/plots/gas_proving_lt.png)

**gt**: Slope = 3.84µs/gas, R² = 0.9896

![gt](/marginal-gas-benchmark/sp1/plots/gas_proving_gt.png)

**slt**: Slope = 3.79µs/gas, R² = 0.8388

![slt](/marginal-gas-benchmark/sp1/plots/gas_proving_slt.png)

**add**: Slope = 3.78µs/gas, R² = 0.9987

![add](/marginal-gas-benchmark/sp1/plots/gas_proving_add.png)

**push16**: Slope = 3.73µs/gas, R² = 0.9960

![push16](/marginal-gas-benchmark/sp1/plots/gas_proving_push16.png)

**sgt**: Slope = 3.55µs/gas, R² = 0.8265

![sgt](/marginal-gas-benchmark/sp1/plots/gas_proving_sgt.png)

**dup16**: Slope = 3.54µs/gas, R² = 0.9449

![dup16](/marginal-gas-benchmark/sp1/plots/gas_proving_dup16.png)

**mstore8**: Slope = 3.36µs/gas, R² = 0.9735

![mstore8](/marginal-gas-benchmark/sp1/plots/gas_proving_mstore8.png)

**extcodehash**: Slope = 3.24µs/gas, R² = 0.9982

![extcodehash](/marginal-gas-benchmark/sp1/plots/gas_proving_extcodehash.png)

**dup8**: Slope = 3.21µs/gas, R² = 0.9217

![dup8](/marginal-gas-benchmark/sp1/plots/gas_proving_dup8.png)

**dup1**: Slope = 3.05µs/gas, R² = 0.8879

![dup1](/marginal-gas-benchmark/sp1/plots/gas_proving_dup1.png)

**or**: Slope = 3.02µs/gas, R² = 0.8897

![or](/marginal-gas-benchmark/sp1/plots/gas_proving_or.png)

**push1**: Slope = 2.75µs/gas, R² = 0.9551

![push1](/marginal-gas-benchmark/sp1/plots/gas_proving_push1.png)

**sub**: Slope = 2.59µs/gas, R² = 0.7971

![sub](/marginal-gas-benchmark/sp1/plots/gas_proving_sub.png)

**balance**: Slope = 2.43µs/gas, R² = 0.9993

![balance](/marginal-gas-benchmark/sp1/plots/gas_proving_balance.png)

**extcodecopy**: Slope = 2.30µs/gas, R² = 0.9967

![extcodecopy](/marginal-gas-benchmark/sp1/plots/gas_proving_extcodecopy.png)

**iszero**: Slope = 2.30µs/gas, R² = 0.8634

![iszero](/marginal-gas-benchmark/sp1/plots/gas_proving_iszero.png)

**extcodesize**: Slope = 2.25µs/gas, R² = 0.9978

![extcodesize](/marginal-gas-benchmark/sp1/plots/gas_proving_extcodesize.png)

**sload**: Slope = 2.10µs/gas, R² = 0.9979

![sload](/marginal-gas-benchmark/sp1/plots/gas_proving_sload.png)

**jumpi**: Slope = 2.09µs/gas, R² = 0.9972

![jumpi](/marginal-gas-benchmark/sp1/plots/gas_proving_jumpi.png)

**mcopy**: Slope = 2.09µs/gas, R² = 0.9951

![mcopy](/marginal-gas-benchmark/sp1/plots/gas_proving_mcopy.png)

**tstore**: Slope = 2.08µs/gas, R² = 0.9995

![tstore](/marginal-gas-benchmark/sp1/plots/gas_proving_tstore.png)

**identity**: Slope = 2.06µs/gas, R² = 0.7010

![identity](/marginal-gas-benchmark/sp1/plots/gas_proving_identity.png)

**blobhash**: Slope = 1.92µs/gas, R² = 0.9644

![blobhash](/marginal-gas-benchmark/sp1/plots/gas_proving_blobhash.png)

**not**: Slope = 1.89µs/gas, R² = 0.9312

![not](/marginal-gas-benchmark/sp1/plots/gas_proving_not.png)

**ripemd160**: Slope = 1.55µs/gas, R² = 0.9559

![ripemd160](/marginal-gas-benchmark/sp1/plots/gas_proving_ripemd160.png)

**pop**: Slope = 0.96µs/gas, R² = 0.7252

![pop](/marginal-gas-benchmark/sp1/plots/gas_proving_pop.png)

**jump**: Slope = 0.88µs/gas, R² = 0.8975

![jump](/marginal-gas-benchmark/sp1/plots/gas_proving_jump.png)

**tload**: Slope = 0.35µs/gas, R² = 0.9982

![tload](/marginal-gas-benchmark/sp1/plots/gas_proving_tload.png)

**create**: Slope = 0.15µs/gas, R² = 0.9997

![create](/marginal-gas-benchmark/sp1/plots/gas_proving_create.png)

**create2**: Slope = 0.15µs/gas, R² = 0.9997

![create2](/marginal-gas-benchmark/sp1/plots/gas_proving_create2.png)

**log0**: Slope = 0.15µs/gas, R² = 0.9268

![log0](/marginal-gas-benchmark/sp1/plots/gas_proving_log0.png)

**log1**: Slope = 0.14µs/gas, R² = 0.9995

![log1](/marginal-gas-benchmark/sp1/plots/gas_proving_log1.png)

**log2**: Slope = 0.08µs/gas, R² = 0.9756

![log2](/marginal-gas-benchmark/sp1/plots/gas_proving_log2.png)

**log4**: Slope = 0.08µs/gas, R² = 0.9897

![log4](/marginal-gas-benchmark/sp1/plots/gas_proving_log4.png)

**log3**: Slope = 0.07µs/gas, R² = 0.9979

![log3](/marginal-gas-benchmark/sp1/plots/gas_proving_log3.png)

### sp1 Gas vs ZK Cycles

**modexp**: Slope = 3.79K cycles/gas, R² = 1.0000

![modexp](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_modexp.png)

**point_evaluation**: Slope = 2.96K cycles/gas, R² = 1.0000

![point_evaluation](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_point_evaluation.png)

**bls12_map_fp_to_g1**: Slope = 1.16K cycles/gas, R² = 1.0000

![bls12_map_fp_to_g1](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_bls12_map_fp_to_g1.png)

**bls12_pairing**: Slope = 995.58 cycles/gas, R² = 1.0000

![bls12_pairing](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_bls12_pairing.png)

**bls12_map_fp2_to_g2**: Slope = 830.06 cycles/gas, R² = 1.0000

![bls12_map_fp2_to_g2](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_bls12_map_fp2_to_g2.png)

**bls12_g1add**: Slope = 816.57 cycles/gas, R² = 1.0000

![bls12_g1add](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_bls12_g1add.png)

**bls12_g2add**: Slope = 810.27 cycles/gas, R² = 1.0000

![bls12_g2add](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_bls12_g2add.png)

**blake2f**: Slope = 597.02 cycles/gas, R² = 1.0000

![blake2f](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_blake2f.png)

**mulmod**: Slope = 422.25 cycles/gas, R² = 1.0000

![mulmod](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_mulmod.png)

**bls12_g1msm**: Slope = 371.60 cycles/gas, R² = 1.0000

![bls12_g1msm](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_bls12_g1msm.png)

**div**: Slope = 306.02 cycles/gas, R² = 1.0000

![div](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_div.png)

**bls12_g2msm**: Slope = 280.55 cycles/gas, R² = 1.0000

![bls12_g2msm](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_bls12_g2msm.png)

**sdiv**: Slope = 260.42 cycles/gas, R² = 1.0000

![sdiv](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_sdiv.png)

**mod**: Slope = 255.42 cycles/gas, R² = 1.0000

![mod](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_mod.png)

**selfbalance**: Slope = 234.00 cycles/gas, R² = 1.0000

![selfbalance](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_selfbalance.png)

**addmod**: Slope = 193.00 cycles/gas, R² = 1.0000

![addmod](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_addmod.png)

**eq**: Slope = 99.03 cycles/gas, R² = 1.0000

![eq](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_eq.png)

**exp**: Slope = 87.58 cycles/gas, R² = 1.0000

![exp](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_exp.png)

**smod**: Slope = 79.62 cycles/gas, R² = 1.0000

![smod](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_smod.png)

**prevrandao**: Slope = 78.50 cycles/gas, R² = 1.0000

![prevrandao](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_prevrandao.png)

**sar**: Slope = 78.34 cycles/gas, R² = 1.0000

![sar](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_sar.png)

**bn128_mul**: Slope = 77.49 cycles/gas, R² = 1.0000

![bn128_mul](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_bn128_mul.png)

**call**: Slope = 68.84 cycles/gas, R² = 1.0000

![call](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_call.png)

**callcode**: Slope = 66.91 cycles/gas, R² = 1.0000

![callcode](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_callcode.png)

**staticcall**: Slope = 66.80 cycles/gas, R² = 1.0000

![staticcall](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_staticcall.png)

**mstore**: Slope = 62.70 cycles/gas, R² = 1.0000

![mstore](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_mstore.png)

**mul**: Slope = 60.62 cycles/gas, R² = 1.0000

![mul](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_mul.png)

**address**: Slope = 58.00 cycles/gas, R² = 1.0000

![address](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_address.png)

**caller**: Slope = 58.00 cycles/gas, R² = 1.0000

![caller](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_caller.png)

**coinbase**: Slope = 58.00 cycles/gas, R² = 1.0000

![coinbase](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_coinbase.png)

**origin**: Slope = 58.00 cycles/gas, R² = 1.0000

![origin](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_origin.png)

**delegatecall**: Slope = 56.48 cycles/gas, R² = 1.0000

![delegatecall](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_delegatecall.png)

**calldataload**: Slope = 54.34 cycles/gas, R² = 1.0000

![calldataload](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_calldataload.png)

**bn128_pairing**: Slope = 54.18 cycles/gas, R² = 1.0000

![bn128_pairing](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_bn128_pairing.png)

**mload**: Slope = 54.01 cycles/gas, R² = 1.0000

![mload](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_mload.png)

**signextend**: Slope = 53.42 cycles/gas, R² = 1.0000

![signextend](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_signextend.png)

**swap8**: Slope = 53.37 cycles/gas, R² = 1.0000

![swap8](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_swap8.png)

**swap1**: Slope = 53.37 cycles/gas, R² = 1.0000

![swap1](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_swap1.png)

**swap16**: Slope = 53.34 cycles/gas, R² = 1.0000

![swap16](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_swap16.png)

**shr**: Slope = 51.34 cycles/gas, R² = 1.0000

![shr](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_shr.png)

**shl**: Slope = 50.34 cycles/gas, R² = 1.0000

![shl](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_shl.png)

**codecopy**: Slope = 47.07 cycles/gas, R² = 0.9929

![codecopy](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_codecopy.png)

**blobbasefee**: Slope = 42.50 cycles/gas, R² = 1.0000

![blobbasefee](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_blobbasefee.png)

**push32**: Slope = 41.71 cycles/gas, R² = 1.0000

![push32](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_push32.png)

**bn128_add**: Slope = 40.55 cycles/gas, R² = 1.0000

![bn128_add](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_bn128_add.png)

**gasprice**: Slope = 40.50 cycles/gas, R² = 1.0000

![gasprice](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_gasprice.png)

**callvalue**: Slope = 39.00 cycles/gas, R² = 1.0000

![callvalue](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_callvalue.png)

**sgt**: Slope = 38.02 cycles/gas, R² = 1.0000

![sgt](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_sgt.png)

**slt**: Slope = 38.02 cycles/gas, R² = 1.0000

![slt](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_slt.png)

**sub**: Slope = 37.36 cycles/gas, R² = 1.0000

![sub](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_sub.png)

**calldatacopy**: Slope = 35.33 cycles/gas, R² = 1.0000

![calldatacopy](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_calldatacopy.png)

**sstore**: Slope = 34.90 cycles/gas, R² = 1.0000

![sstore](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_sstore.png)

**number**: Slope = 32.50 cycles/gas, R² = 1.0000

![number](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_number.png)

**timestamp**: Slope = 32.50 cycles/gas, R² = 1.0000

![timestamp](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_timestamp.png)

**add**: Slope = 32.36 cycles/gas, R² = 1.0000

![add](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_add.png)

**calldatasize**: Slope = 32.00 cycles/gas, R² = 1.0000

![calldatasize](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_calldatasize.png)

**pc**: Slope = 32.00 cycles/gas, R² = 1.0000

![pc](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_pc.png)

**chainid**: Slope = 31.50 cycles/gas, R² = 1.0000

![chainid](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_chainid.png)

**basefee**: Slope = 31.50 cycles/gas, R² = 1.0000

![basefee](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_basefee.png)

**codesize**: Slope = 30.50 cycles/gas, R² = 1.0000

![codesize](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_codesize.png)

**msize**: Slope = 30.00 cycles/gas, R² = 1.0000

![msize](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_msize.png)

**gaslimit**: Slope = 30.00 cycles/gas, R² = 1.0000

![gaslimit](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_gaslimit.png)

**gas**: Slope = 30.00 cycles/gas, R² = 1.0000

![gas](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_gas.png)

**returndatasize**: Slope = 30.00 cycles/gas, R² = 1.0000

![returndatasize](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_returndatasize.png)

**push0**: Slope = 28.50 cycles/gas, R² = 1.0000

![push0](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_push0.png)

**push16**: Slope = 28.36 cycles/gas, R² = 1.0000

![push16](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_push16.png)

**gt**: Slope = 27.36 cycles/gas, R² = 1.0000

![gt](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_gt.png)

**lt**: Slope = 27.36 cycles/gas, R² = 1.0000

![lt](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_lt.png)

**byte**: Slope = 27.34 cycles/gas, R² = 1.0000

![byte](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_byte.png)

**returndatacopy**: Slope = 26.45 cycles/gas, R² = 1.0000

![returndatacopy](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_returndatacopy.png)

**mstore8**: Slope = 26.01 cycles/gas, R² = 1.0000

![mstore8](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_mstore8.png)

**jumpdest**: Slope = 26.00 cycles/gas, R² = 1.0000

![jumpdest](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_jumpdest.png)

**and**: Slope = 24.03 cycles/gas, R² = 1.0000

![and](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_and.png)

**xor**: Slope = 24.03 cycles/gas, R² = 1.0000

![xor](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_xor.png)

**or**: Slope = 24.02 cycles/gas, R² = 1.0000

![or](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_or.png)

**blobhash**: Slope = 22.34 cycles/gas, R² = 1.0000

![blobhash](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_blobhash.png)

**extcodehash**: Slope = 22.15 cycles/gas, R² = 1.0000

![extcodehash](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_extcodehash.png)

**iszero**: Slope = 19.33 cycles/gas, R² = 1.0000

![iszero](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_iszero.png)

**not**: Slope = 18.34 cycles/gas, R² = 1.0000

![not](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_not.png)

**keccak256**: Slope = 17.78 cycles/gas, R² = 1.0000

![keccak256](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_keccak256.png)

**dup8**: Slope = 17.02 cycles/gas, R² = 1.0000

![dup8](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_dup8.png)

**dup1**: Slope = 17.00 cycles/gas, R² = 1.0000

![dup1](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_dup1.png)

**dup16**: Slope = 16.99 cycles/gas, R² = 1.0000

![dup16](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_dup16.png)

**extcodesize**: Slope = 16.66 cycles/gas, R² = 1.0000

![extcodesize](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_extcodesize.png)

**balance**: Slope = 16.60 cycles/gas, R² = 1.0000

![balance](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_balance.png)

**tstore**: Slope = 15.90 cycles/gas, R² = 1.0000

![tstore](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_tstore.png)

**extcodecopy**: Slope = 15.75 cycles/gas, R² = 1.0000

![extcodecopy](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_extcodecopy.png)

**push1**: Slope = 15.34 cycles/gas, R² = 1.0000

![push1](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_push1.png)

**pop**: Slope = 15.00 cycles/gas, R² = 1.0000

![pop](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_pop.png)

**sload**: Slope = 15.00 cycles/gas, R² = 1.0000

![sload](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_sload.png)

**mcopy**: Slope = 14.80 cycles/gas, R² = 1.0000

![mcopy](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_mcopy.png)

**sha256**: Slope = 12.40 cycles/gas, R² = 1.0000

![sha256](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_sha256.png)

**jumpi**: Slope = 11.60 cycles/gas, R² = 1.0000

![jumpi](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_jumpi.png)

**ecrecover**: Slope = 10.67 cycles/gas, R² = 1.0000

![ecrecover](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_ecrecover.png)

**ripemd160**: Slope = 9.01 cycles/gas, R² = 1.0000

![ripemd160](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_ripemd160.png)

**jump**: Slope = 7.88 cycles/gas, R² = 1.0000

![jump](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_jump.png)

**identity**: Slope = 4.66 cycles/gas, R² = 1.0000

![identity](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_identity.png)

**tload**: Slope = 2.43 cycles/gas, R² = 1.0000

![tload](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_tload.png)

**create2**: Slope = 0.8391 cycles/gas, R² = 0.9997

![create2](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_create2.png)

**create**: Slope = 0.8048 cycles/gas, R² = 0.9996

![create](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_create.png)

**log1**: Slope = 0.5628 cycles/gas, R² = 0.9997

![log1](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_log1.png)

**log0**: Slope = 0.5558 cycles/gas, R² = 0.9944

![log0](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_log0.png)

**log3**: Slope = 0.316 cycles/gas, R² = 0.9999

![log3](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_log3.png)

**log4**: Slope = 0.3136 cycles/gas, R² = 0.9998

![log4](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_log4.png)

**log2**: Slope = 0.3078 cycles/gas, R² = 0.9999

![log2](/marginal-gas-benchmark/sp1/plots/gas_zkcycles_log2.png)

### sp1 ZK Cycles vs Proving Time

**bn128_add**: Slope = 0.93µs/cycle, R² = 0.9193

![bn128_add](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_bn128_add.png)

**ecrecover**: Slope = 0.89µs/cycle, R² = 0.9380

![ecrecover](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_ecrecover.png)

**sha256**: Slope = 0.77µs/cycle, R² = 0.9985

![sha256](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_sha256.png)

**bn128_mul**: Slope = 0.52µs/cycle, R² = 0.9977

![bn128_mul](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_bn128_mul.png)

**bn128_pairing**: Slope = 0.41µs/cycle, R² = 0.9909

![bn128_pairing](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_bn128_pairing.png)

**keccak256**: Slope = 0.38µs/cycle, R² = 0.9992

![keccak256](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_keccak256.png)

**log0**: Slope = 0.27µs/cycle, R² = 0.9422

![log0](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_log0.png)

**log2**: Slope = 0.26µs/cycle, R² = 0.9747

![log2](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_log2.png)

**log4**: Slope = 0.25µs/cycle, R² = 0.9886

![log4](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_log4.png)

**log1**: Slope = 0.24µs/cycle, R² = 0.9988

![log1](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_log1.png)

**and**: Slope = 0.22µs/cycle, R² = 0.9896

![and](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_and.png)

**log3**: Slope = 0.22µs/cycle, R² = 0.9981

![log3](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_log3.png)

**dup16**: Slope = 0.21µs/cycle, R² = 0.9461

![dup16](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_dup16.png)

**xor**: Slope = 0.20µs/cycle, R² = 0.9720

![xor](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_xor.png)

**swap16**: Slope = 0.20µs/cycle, R² = 0.9845

![swap16](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_swap16.png)

**dup8**: Slope = 0.19µs/cycle, R² = 0.9216

![dup8](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_dup8.png)

**create**: Slope = 0.19µs/cycle, R² = 0.9990

![create](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_create.png)

**swap8**: Slope = 0.19µs/cycle, R² = 0.9846

![swap8](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_swap8.png)

**codesize**: Slope = 0.18µs/cycle, R² = 0.9863

![codesize](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_codesize.png)

**gas**: Slope = 0.18µs/cycle, R² = 0.9837

![gas](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_gas.png)

**push32**: Slope = 0.18µs/cycle, R² = 0.9244

![push32](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_push32.png)

**jumpi**: Slope = 0.18µs/cycle, R² = 0.9972

![jumpi](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_jumpi.png)

**dup1**: Slope = 0.18µs/cycle, R² = 0.8879

![dup1](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_dup1.png)

**push1**: Slope = 0.18µs/cycle, R² = 0.9551

![push1](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_push1.png)

**create2**: Slope = 0.18µs/cycle, R² = 0.9989

![create2](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_create2.png)

**swap1**: Slope = 0.18µs/cycle, R² = 0.9784

![swap1](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_swap1.png)

**push0**: Slope = 0.17µs/cycle, R² = 0.9421

![push0](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_push0.png)

**ripemd160**: Slope = 0.17µs/cycle, R² = 0.9550

![ripemd160](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_ripemd160.png)

**calldatasize**: Slope = 0.17µs/cycle, R² = 0.9780

![calldatasize](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_calldatasize.png)

**pc**: Slope = 0.17µs/cycle, R² = 0.9563

![pc](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_pc.png)

**byte**: Slope = 0.17µs/cycle, R² = 0.9691

![byte](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_byte.png)

**callvalue**: Slope = 0.16µs/cycle, R² = 0.9789

![callvalue](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_callvalue.png)

**gasprice**: Slope = 0.16µs/cycle, R² = 0.9995

![gasprice](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_gasprice.png)

**chainid**: Slope = 0.16µs/cycle, R² = 1.0000

![chainid](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_chainid.png)

**gaslimit**: Slope = 0.16µs/cycle, R² = 0.9960

![gaslimit](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_gaslimit.png)

**returndatacopy**: Slope = 0.16µs/cycle, R² = 0.9857

![returndatacopy](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_returndatacopy.png)

**jumpdest**: Slope = 0.16µs/cycle, R² = 0.9930

![jumpdest](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_jumpdest.png)

**returndatasize**: Slope = 0.16µs/cycle, R² = 0.9945

![returndatasize](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_returndatasize.png)

**blobbasefee**: Slope = 0.16µs/cycle, R² = 0.9941

![blobbasefee](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_blobbasefee.png)

**mstore**: Slope = 0.16µs/cycle, R² = 0.9946

![mstore](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_mstore.png)

**codecopy**: Slope = 0.15µs/cycle, R² = 0.9996

![codecopy](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_codecopy.png)

**basefee**: Slope = 0.15µs/cycle, R² = 0.9867

![basefee](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_basefee.png)

**eq**: Slope = 0.15µs/cycle, R² = 0.9872

![eq](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_eq.png)

**extcodehash**: Slope = 0.15µs/cycle, R² = 0.9982

![extcodehash](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_extcodehash.png)

**modexp**: Slope = 0.15µs/cycle, R² = 0.9945

![modexp](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_modexp.png)

**extcodecopy**: Slope = 0.15µs/cycle, R² = 0.9967

![extcodecopy](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_extcodecopy.png)

**balance**: Slope = 0.15µs/cycle, R² = 0.9993

![balance](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_balance.png)

**lt**: Slope = 0.14µs/cycle, R² = 0.9970

![lt](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_lt.png)

**number**: Slope = 0.14µs/cycle, R² = 0.9845

![number](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_number.png)

**tload**: Slope = 0.14µs/cycle, R² = 0.9983

![tload](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_tload.png)

**coinbase**: Slope = 0.14µs/cycle, R² = 0.9856

![coinbase](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_coinbase.png)

**mcopy**: Slope = 0.14µs/cycle, R² = 0.9948

![mcopy](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_mcopy.png)

**caller**: Slope = 0.14µs/cycle, R² = 0.9845

![caller](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_caller.png)

**bls12_g2msm**: Slope = 0.14µs/cycle, R² = 0.9966

![bls12_g2msm](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_bls12_g2msm.png)

**gt**: Slope = 0.14µs/cycle, R² = 0.9895

![gt](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_gt.png)

**sload**: Slope = 0.14µs/cycle, R² = 0.9979

![sload](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_sload.png)

**bls12_g1msm**: Slope = 0.14µs/cycle, R² = 0.9974

![bls12_g1msm](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_bls12_g1msm.png)

**origin**: Slope = 0.14µs/cycle, R² = 0.9678

![origin](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_origin.png)

**callcode**: Slope = 0.14µs/cycle, R² = 0.9990

![callcode](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_callcode.png)

**shr**: Slope = 0.14µs/cycle, R² = 0.9971

![shr](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_shr.png)

**bls12_g1add**: Slope = 0.14µs/cycle, R² = 0.9995

![bls12_g1add](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_bls12_g1add.png)

**exp**: Slope = 0.14µs/cycle, R² = 0.9968

![exp](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_exp.png)

**bls12_map_fp_to_g1**: Slope = 0.14µs/cycle, R² = 0.9993

![bls12_map_fp_to_g1](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_bls12_map_fp_to_g1.png)

**bls12_g2add**: Slope = 0.14µs/cycle, R² = 0.9986

![bls12_g2add](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_bls12_g2add.png)

**blake2f**: Slope = 0.14µs/cycle, R² = 0.9995

![blake2f](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_blake2f.png)

**prevrandao**: Slope = 0.14µs/cycle, R² = 0.9903

![prevrandao](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_prevrandao.png)

**bls12_map_fp2_to_g2**: Slope = 0.14µs/cycle, R² = 0.9999

![bls12_map_fp2_to_g2](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_bls12_map_fp2_to_g2.png)

**extcodesize**: Slope = 0.13µs/cycle, R² = 0.9978

![extcodesize](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_extcodesize.png)

**point_evaluation**: Slope = 0.13µs/cycle, R² = 1.0000

![point_evaluation](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_point_evaluation.png)

**call**: Slope = 0.13µs/cycle, R² = 0.9931

![call](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_call.png)

**bls12_pairing**: Slope = 0.13µs/cycle, R² = 1.0000

![bls12_pairing](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_bls12_pairing.png)

**staticcall**: Slope = 0.13µs/cycle, R² = 0.9999

![staticcall](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_staticcall.png)

**delegatecall**: Slope = 0.13µs/cycle, R² = 0.9999

![delegatecall](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_delegatecall.png)

**selfbalance**: Slope = 0.13µs/cycle, R² = 0.9980

![selfbalance](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_selfbalance.png)

**timestamp**: Slope = 0.13µs/cycle, R² = 0.9886

![timestamp](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_timestamp.png)

**sstore**: Slope = 0.13µs/cycle, R² = 0.9998

![sstore](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_sstore.png)

**msize**: Slope = 0.13µs/cycle, R² = 0.9917

![msize](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_msize.png)

**push16**: Slope = 0.13µs/cycle, R² = 0.9961

![push16](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_push16.png)

**tstore**: Slope = 0.13µs/cycle, R² = 0.9995

![tstore](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_tstore.png)

**mstore8**: Slope = 0.13µs/cycle, R² = 0.9736

![mstore8](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_mstore8.png)

**address**: Slope = 0.13µs/cycle, R² = 0.9954

![address](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_address.png)

**div**: Slope = 0.13µs/cycle, R² = 0.9973

![div](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_div.png)

**or**: Slope = 0.13µs/cycle, R² = 0.8900

![or](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_or.png)

**addmod**: Slope = 0.13µs/cycle, R² = 0.9999

![addmod](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_addmod.png)

**sdiv**: Slope = 0.13µs/cycle, R² = 0.9951

![sdiv](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_sdiv.png)

**sar**: Slope = 0.13µs/cycle, R² = 0.9839

![sar](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_sar.png)

**mod**: Slope = 0.12µs/cycle, R² = 0.9997

![mod](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_mod.png)

**mulmod**: Slope = 0.12µs/cycle, R² = 0.9999

![mulmod](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_mulmod.png)

**shl**: Slope = 0.12µs/cycle, R² = 0.9913

![shl](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_shl.png)

**smod**: Slope = 0.12µs/cycle, R² = 0.9969

![smod](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_smod.png)

**mload**: Slope = 0.12µs/cycle, R² = 0.9755

![mload](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_mload.png)

**mul**: Slope = 0.12µs/cycle, R² = 0.9892

![mul](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_mul.png)

**iszero**: Slope = 0.12µs/cycle, R² = 0.8634

![iszero](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_iszero.png)

**calldatacopy**: Slope = 0.12µs/cycle, R² = 0.9724

![calldatacopy](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_calldatacopy.png)

**calldataload**: Slope = 0.12µs/cycle, R² = 0.9983

![calldataload](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_calldataload.png)

**add**: Slope = 0.12µs/cycle, R² = 0.9987

![add](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_add.png)

**jump**: Slope = 0.11µs/cycle, R² = 0.8975

![jump](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_jump.png)

**signextend**: Slope = 0.11µs/cycle, R² = 0.9919

![signextend](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_signextend.png)

**not**: Slope = 0.10µs/cycle, R² = 0.9312

![not](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_not.png)

**slt**: Slope = 0.10µs/cycle, R² = 0.8383

![slt](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_slt.png)

**sgt**: Slope = 0.09µs/cycle, R² = 0.8261

![sgt](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_sgt.png)

**blobhash**: Slope = 0.09µs/cycle, R² = 0.9642

![blobhash](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_blobhash.png)

**sub**: Slope = 0.07µs/cycle, R² = 0.7967

![sub](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_sub.png)

**pop**: Slope = 0.06µs/cycle, R² = 0.7252

![pop](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_pop.png)

**identity**: Slope = 0.44µs/cycle, R² = 0.6975

![identity](/marginal-gas-benchmark/sp1/plots/zkcycles_proving_identity.png)

## Appendix: Per-Op-Count Regression

### Gas ↔ Proving Time

| Opcode | Time/Gas | R² | Std Error |
|--------|----------|-----|-----------|
| modexp | 554.91µs/gas | <span style="color: #28a745; font-weight: bold;">0.9945</span> | 29.11µs |
| point_evaluation | 398.23µs/gas | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 1.59µs |
| bls12_map_fp_to_g1 | 159.35µs/gas | <span style="color: #28a745; font-weight: bold;">0.9993</span> | 3.07µs |
| bls12_pairing | 133.93µs/gas | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.46µs |
| bls12_g1add | 112.19µs/gas | <span style="color: #28a745; font-weight: bold;">0.9995</span> | 1.84µs |
| bls12_map_fp2_to_g2 | 112.10µs/gas | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.84µs |
| bls12_g2add | 110.98µs/gas | <span style="color: #28a745; font-weight: bold;">0.9986</span> | 2.97µs |
| blake2f | 81.60µs/gas | <span style="color: #28a745; font-weight: bold;">0.9995</span> | 1.78µs |
| mulmod | 52.68µs/gas | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.33µs |
| bls12_g1msm | 51.92µs/gas | <span style="color: #28a745; font-weight: bold;">0.9974</span> | 1.87µs |
| bn128_mul | 40.27µs/gas | <span style="color: #28a745; font-weight: bold;">0.9977</span> | 1.36µs |
| bls12_g2msm | 39.40µs/gas | <span style="color: #28a745; font-weight: bold;">0.9966</span> | 1.62µs |
| div | 38.98µs/gas | <span style="color: #28a745; font-weight: bold;">0.9973</span> | 1.43µs |
| bn128_add | 37.83µs/gas | <span style="color: #28a745; font-weight: bold;">0.9196</span> | 7.91µs |
| sdiv | 32.61µs/gas | <span style="color: #28a745; font-weight: bold;">0.9951</span> | 1.62µs |
| mod | 31.87µs/gas | <span style="color: #28a745; font-weight: bold;">0.9997</span> | 0.39µs |
| selfbalance | 31.23µs/gas | <span style="color: #28a745; font-weight: bold;">0.9980</span> | 0.99µs |
| addmod | 24.18µs/gas | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.14µs |
| bn128_pairing | 22.11µs/gas | <span style="color: #28a745; font-weight: bold;">0.9909</span> | 1.23µs |
| eq | 14.68µs/gas | <span style="color: #28a745; font-weight: bold;">0.9872</span> | 1.18µs |
| exp | 12.03µs/gas | <span style="color: #28a745; font-weight: bold;">0.9968</span> | 0.48µs |
| swap16 | 10.79µs/gas | <span style="color: #28a745; font-weight: bold;">0.9845</span> | 0.96µs |
| prevrandao | 10.61µs/gas | <span style="color: #28a745; font-weight: bold;">0.9904</span> | 0.74µs |
| swap8 | 9.93µs/gas | <span style="color: #28a745; font-weight: bold;">0.9847</span> | 0.88µs |
| sar | 9.80µs/gas | <span style="color: #28a745; font-weight: bold;">0.9839</span> | 0.89µs |
| mstore | 9.77µs/gas | <span style="color: #28a745; font-weight: bold;">0.9946</span> | 0.51µs |
| smod | 9.71µs/gas | <span style="color: #28a745; font-weight: bold;">0.9969</span> | 0.38µs |
| sha256 | 9.61µs/gas | <span style="color: #28a745; font-weight: bold;">0.9985</span> | 0.26µs |
| ecrecover | 9.55µs/gas | <span style="color: #28a745; font-weight: bold;">0.9380</span> | 1.74µs |
| swap1 | 9.49µs/gas | <span style="color: #28a745; font-weight: bold;">0.9781</span> | 0.71µs |
| call | 9.27µs/gas | <span style="color: #28a745; font-weight: bold;">0.9932</span> | 0.54µs |
| callcode | 9.26µs/gas | <span style="color: #28a745; font-weight: bold;">0.9991</span> | 0.20µs |
| staticcall | 8.97µs/gas | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.05µs |
| coinbase | 8.25µs/gas | <span style="color: #28a745; font-weight: bold;">0.9857</span> | 0.70µs |
| caller | 8.17µs/gas | <span style="color: #28a745; font-weight: bold;">0.9845</span> | 0.72µs |
| origin | 8.10µs/gas | <span style="color: #28a745; font-weight: bold;">0.9678</span> | 1.04µs |
| delegatecall | 7.55µs/gas | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.05µs |
| push32 | 7.52µs/gas | <span style="color: #28a745; font-weight: bold;">0.9244</span> | 1.08µs |
| address | 7.42µs/gas | <span style="color: #28a745; font-weight: bold;">0.9953</span> | 0.36µs |
| mul | 7.33µs/gas | <span style="color: #28a745; font-weight: bold;">0.9892</span> | 0.54µs |
| codecopy | 7.13µs/gas | <span style="color: #28a745; font-weight: bold;">0.9950</span> | 0.36µs |
| shr | 7.09µs/gas | <span style="color: #28a745; font-weight: bold;">0.9971</span> | 0.27µs |
| keccak256 | 6.68µs/gas | <span style="color: #28a745; font-weight: bold;">0.9992</span> | 0.14µs |
| blobbasefee | 6.62µs/gas | <span style="color: #28a745; font-weight: bold;">0.9940</span> | 0.36µs |
| mload | 6.55µs/gas | <span style="color: #28a745; font-weight: bold;">0.9755</span> | 0.73µs |
| gasprice | 6.50µs/gas | <span style="color: #28a745; font-weight: bold;">0.9995</span> | 0.10µs |
| calldataload | 6.35µs/gas | <span style="color: #28a745; font-weight: bold;">0.9983</span> | 0.18µs |
| callvalue | 6.31µs/gas | <span style="color: #28a745; font-weight: bold;">0.9789</span> | 0.53µs |
| shl | 6.26µs/gas | <span style="color: #28a745; font-weight: bold;">0.9913</span> | 0.42µs |
| signextend | 5.91µs/gas | <span style="color: #28a745; font-weight: bold;">0.9918</span> | 0.38µs |
| codesize | 5.63µs/gas | <span style="color: #28a745; font-weight: bold;">0.9864</span> | 0.47µs |
| gas | 5.51µs/gas | <span style="color: #28a745; font-weight: bold;">0.9836</span> | 0.50µs |
| calldatasize | 5.48µs/gas | <span style="color: #28a745; font-weight: bold;">0.9780</span> | 0.41µs |
| pc | 5.43µs/gas | <span style="color: #28a745; font-weight: bold;">0.9565</span> | 0.82µs |
| and | 5.24µs/gas | <span style="color: #28a745; font-weight: bold;">0.9897</span> | 0.38µs |
| chainid | 5.04µs/gas | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.00µs |
| push0 | 4.92µs/gas | <span style="color: #28a745; font-weight: bold;">0.9425</span> | 0.86µs |
| xor | 4.91µs/gas | <span style="color: #28a745; font-weight: bold;">0.9720</span> | 0.59µs |
| gaslimit | 4.79µs/gas | <span style="color: #28a745; font-weight: bold;">0.9959</span> | 0.22µs |
| returndatasize | 4.70µs/gas | <span style="color: #28a745; font-weight: bold;">0.9944</span> | 0.25µs |
| basefee | 4.68µs/gas | <span style="color: #28a745; font-weight: bold;">0.9866</span> | 0.39µs |
| number | 4.66µs/gas | <span style="color: #28a745; font-weight: bold;">0.9843</span> | 0.42µs |
| sstore | 4.61µs/gas | <span style="color: #28a745; font-weight: bold;">0.9998</span> | 0.05µs |
| byte | 4.54µs/gas | <span style="color: #28a745; font-weight: bold;">0.9691</span> | 0.57µs |
| timestamp | 4.33µs/gas | <span style="color: #28a745; font-weight: bold;">0.9887</span> | 0.33µs |
| returndatacopy | 4.20µs/gas | <span style="color: #28a745; font-weight: bold;">0.9857</span> | 0.36µs |
| calldatacopy | 4.14µs/gas | <span style="color: #28a745; font-weight: bold;">0.9724</span> | 0.49µs |
| jumpdest | 4.10µs/gas | <span style="color: #28a745; font-weight: bold;">0.9930</span> | 0.24µs |
| msize | 3.96µs/gas | <span style="color: #28a745; font-weight: bold;">0.9917</span> | 0.21µs |
| lt | 3.95µs/gas | <span style="color: #28a745; font-weight: bold;">0.9971</span> | 0.15µs |
| gt | 3.84µs/gas | <span style="color: #28a745; font-weight: bold;">0.9896</span> | 0.28µs |
| slt | 3.79µs/gas | <span style="color: #ffc107; font-weight: bold;">0.8388</span> | 0.83µs |
| add | 3.78µs/gas | <span style="color: #28a745; font-weight: bold;">0.9987</span> | 0.09µs |
| push16 | 3.73µs/gas | <span style="color: #28a745; font-weight: bold;">0.9960</span> | 0.17µs |
| sgt | 3.55µs/gas | <span style="color: #ffc107; font-weight: bold;">0.8265</span> | 0.81µs |
| dup16 | 3.54µs/gas | <span style="color: #28a745; font-weight: bold;">0.9449</span> | 0.43µs |
| mstore8 | 3.36µs/gas | <span style="color: #28a745; font-weight: bold;">0.9735</span> | 0.39µs |
| extcodehash | 3.24µs/gas | <span style="color: #28a745; font-weight: bold;">0.9982</span> | 0.08µs |
| dup8 | 3.21µs/gas | <span style="color: #28a745; font-weight: bold;">0.9217</span> | 0.47µs |
| dup1 | 3.05µs/gas | <span style="color: #ffc107; font-weight: bold;">0.8879</span> | 0.54µs |
| or | 3.02µs/gas | <span style="color: #ffc107; font-weight: bold;">0.8897</span> | 0.53µs |
| push1 | 2.75µs/gas | <span style="color: #28a745; font-weight: bold;">0.9551</span> | 0.30µs |
| sub | 2.59µs/gas | <span style="color: #ffc107; font-weight: bold;">0.7971</span> | 0.65µs |
| balance | 2.43µs/gas | <span style="color: #28a745; font-weight: bold;">0.9993</span> | 0.05µs |
| extcodecopy | 2.30µs/gas | <span style="color: #28a745; font-weight: bold;">0.9967</span> | 0.09µs |
| iszero | 2.30µs/gas | <span style="color: #ffc107; font-weight: bold;">0.8634</span> | 0.46µs |
| extcodesize | 2.25µs/gas | <span style="color: #28a745; font-weight: bold;">0.9978</span> | 0.07µs |
| sload | 2.10µs/gas | <span style="color: #28a745; font-weight: bold;">0.9979</span> | 0.07µs |
| jumpi | 2.09µs/gas | <span style="color: #28a745; font-weight: bold;">0.9972</span> | 0.08µs |
| mcopy | 2.09µs/gas | <span style="color: #28a745; font-weight: bold;">0.9951</span> | 0.10µs |
| tstore | 2.08µs/gas | <span style="color: #28a745; font-weight: bold;">0.9995</span> | 0.03µs |
| identity | 2.06µs/gas | <span style="color: #ffc107; font-weight: bold;">0.7010</span> | 0.95µs |
| blobhash | 1.92µs/gas | <span style="color: #28a745; font-weight: bold;">0.9644</span> | 0.26µs |
| not | 1.89µs/gas | <span style="color: #28a745; font-weight: bold;">0.9312</span> | 0.26µs |
| ripemd160 | 1.55µs/gas | <span style="color: #28a745; font-weight: bold;">0.9559</span> | 0.23µs |
| pop | 0.96µs/gas | <span style="color: #ffc107; font-weight: bold;">0.7252</span> | 0.29µs |
| jump | 0.88µs/gas | <span style="color: #ffc107; font-weight: bold;">0.8975</span> | 0.15µs |
| tload | 0.35µs/gas | <span style="color: #28a745; font-weight: bold;">0.9982</span> | 0.01µs |
| create | 0.15µs/gas | <span style="color: #28a745; font-weight: bold;">0.9997</span> | 0.00µs |
| create2 | 0.15µs/gas | <span style="color: #28a745; font-weight: bold;">0.9997</span> | 0.00µs |
| log0 | 0.15µs/gas | <span style="color: #28a745; font-weight: bold;">0.9268</span> | 0.02µs |
| log1 | 0.14µs/gas | <span style="color: #28a745; font-weight: bold;">0.9995</span> | 0.00µs |
| log2 | 0.08µs/gas | <span style="color: #28a745; font-weight: bold;">0.9756</span> | 0.01µs |
| log4 | 0.08µs/gas | <span style="color: #28a745; font-weight: bold;">0.9897</span> | 0.01µs |
| log3 | 0.07µs/gas | <span style="color: #28a745; font-weight: bold;">0.9979</span> | 0.00µs |

### Gas ↔ ZK Cycles

| Opcode | Cycles/Gas | R² | Std Error |
|--------|------------|-----|-----------|
| modexp | 3.79K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.145 |
| point_evaluation | 2.96K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.005583 |
| bls12_map_fp_to_g1 | 1.16K | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.00698 |
| bls12_pairing | 995.58 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.002471 |
| bls12_map_fp2_to_g2 | 830.06 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.003916 |
| bls12_g1add | 816.57 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0481 |
| bls12_g2add | 810.27 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.006868 |
| blake2f | 597.02 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.002866 |
| mulmod | 422.25 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0009255 |
| bls12_g1msm | 371.60 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.002802 |
| div | 306.02 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.009481 |
| bls12_g2msm | 280.55 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.001033 |
| sdiv | 260.42 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.009457 |
| mod | 255.42 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.009546 |
| selfbalance | 234.00 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.007161 |
| addmod | 193.00 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.001093 |
| eq | 99.03 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01568 |
| exp | 87.58 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 4.622e-05 |
| smod | 79.62 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.009412 |
| prevrandao | 78.50 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01787 |
| sar | 78.34 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0004705 |
| bn128_mul | 77.49 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.004442 |
| call | 68.84 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.05806 |
| callcode | 66.91 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.05779 |
| staticcall | 66.80 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01348 |
| mstore | 62.70 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.03255 |
| mul | 60.62 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.00943 |
| address | 58.00 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01798 |
| caller | 58.00 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01802 |
| coinbase | 58.00 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01797 |
| origin | 58.00 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01789 |
| delegatecall | 56.48 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01147 |
| calldataload | 54.34 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01456 |
| bn128_pairing | 54.18 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.001476 |
| mload | 54.01 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01423 |
| signextend | 53.42 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.009294 |
| swap8 | 53.37 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01232 |
| swap1 | 53.37 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.03019 |
| swap16 | 53.34 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0003765 |
| shr | 51.34 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0002656 |
| shl | 50.34 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0003668 |
| codecopy | 47.07 | <span style="color: #28a745; font-weight: bold;">0.9929</span> | 2.82 |
| blobbasefee | 42.50 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01809 |
| push32 | 41.71 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0006016 |
| bn128_add | 40.55 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01928 |
| gasprice | 40.50 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01796 |
| callvalue | 39.00 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0005647 |
| sgt | 38.02 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01239 |
| slt | 38.02 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01239 |
| sub | 37.36 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01241 |
| calldatacopy | 35.33 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0001926 |
| sstore | 34.90 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0583 |
| number | 32.50 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0179 |
| timestamp | 32.50 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01808 |
| add | 32.36 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01566 |
| calldatasize | 32.00 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.003185 |
| pc | 32.00 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01791 |
| chainid | 31.50 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01801 |
| basefee | 31.50 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01813 |
| codesize | 30.50 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01798 |
| msize | 30.00 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0001897 |
| gaslimit | 30.00 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01803 |
| gas | 30.00 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01783 |
| returndatasize | 30.00 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01809 |
| push0 | 28.50 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01804 |
| push16 | 28.36 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01441 |
| gt | 27.36 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01564 |
| lt | 27.36 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01563 |
| byte | 27.34 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0004367 |
| returndatacopy | 26.45 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0003697 |
| mstore8 | 26.01 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01318 |
| jumpdest | 26.00 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0001226 |
| and | 24.03 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01578 |
| xor | 24.03 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01552 |
| or | 24.02 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0123 |
| blobhash | 22.34 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01436 |
| extcodehash | 22.15 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.002532 |
| iszero | 19.33 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0002898 |
| not | 18.34 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0006821 |
| keccak256 | 17.78 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 1.358e-05 |
| dup8 | 17.02 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01236 |
| dup1 | 17.00 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.000136 |
| dup16 | 16.99 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.03248 |
| extcodesize | 16.66 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 6.751e-05 |
| balance | 16.60 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 2.933e-05 |
| tstore | 15.90 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.006938 |
| extcodecopy | 15.75 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01176 |
| push1 | 15.34 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0001363 |
| pop | 15.00 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0001595 |
| sload | 15.00 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.002069 |
| mcopy | 14.80 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.02032 |
| sha256 | 12.40 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.001021 |
| jumpi | 11.60 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.0001886 |
| ecrecover | 10.67 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.005079 |
| ripemd160 | 9.01 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.02436 |
| jump | 7.88 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 6.326e-05 |
| identity | 4.66 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.01649 |
| tload | 2.43 | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.009344 |
| create2 | 0.8391 | <span style="color: #28a745; font-weight: bold;">0.9997</span> | 0.01048 |
| create | 0.8048 | <span style="color: #28a745; font-weight: bold;">0.9996</span> | 0.01118 |
| log1 | 0.5628 | <span style="color: #28a745; font-weight: bold;">0.9997</span> | 0.0067 |
| log0 | 0.5558 | <span style="color: #28a745; font-weight: bold;">0.9944</span> | 0.02083 |
| log3 | 0.316 | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.002322 |
| log4 | 0.3136 | <span style="color: #28a745; font-weight: bold;">0.9998</span> | 0.003386 |
| log2 | 0.3078 | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.001867 |

### ZK Cycles ↔ Proving Time

| Opcode | Time/Cycle | R² | Std Error |
|--------|------------|-----|-----------|
| bn128_add | 0.93µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9193</span> | 0.20µs |
| ecrecover | 0.89µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9380</span> | 0.16µs |
| sha256 | 0.77µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9985</span> | 0.02µs |
| bn128_mul | 0.52µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9977</span> | 0.02µs |
| bn128_pairing | 0.41µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9909</span> | 0.02µs |
| keccak256 | 0.38µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9992</span> | 0.01µs |
| log0 | 0.27µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9422</span> | 0.03µs |
| log2 | 0.26µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9747</span> | 0.03µs |
| log4 | 0.25µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9886</span> | 0.02µs |
| log1 | 0.24µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9988</span> | 0.01µs |
| and | 0.22µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9896</span> | 0.02µs |
| log3 | 0.22µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9981</span> | 0.01µs |
| dup16 | 0.21µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9461</span> | 0.02µs |
| xor | 0.20µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9720</span> | 0.02µs |
| swap16 | 0.20µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9845</span> | 0.02µs |
| dup8 | 0.19µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9216</span> | 0.03µs |
| create | 0.19µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9990</span> | 0.00µs |
| swap8 | 0.19µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9846</span> | 0.02µs |
| codesize | 0.18µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9863</span> | 0.02µs |
| gas | 0.18µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9837</span> | 0.02µs |
| push32 | 0.18µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9244</span> | 0.03µs |
| jumpi | 0.18µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9972</span> | 0.01µs |
| dup1 | 0.18µs/cycle | <span style="color: #ffc107; font-weight: bold;">0.8879</span> | 0.03µs |
| push1 | 0.18µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9551</span> | 0.02µs |
| create2 | 0.18µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9989</span> | 0.00µs |
| swap1 | 0.18µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9784</span> | 0.01µs |
| push0 | 0.17µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9421</span> | 0.03µs |
| ripemd160 | 0.17µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9550</span> | 0.03µs |
| calldatasize | 0.17µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9780</span> | 0.01µs |
| pc | 0.17µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9563</span> | 0.03µs |
| byte | 0.17µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9691</span> | 0.02µs |
| callvalue | 0.16µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9789</span> | 0.01µs |
| gasprice | 0.16µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9995</span> | 0.00µs |
| chainid | 0.16µs/cycle | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.00µs |
| gaslimit | 0.16µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9960</span> | 0.01µs |
| returndatacopy | 0.16µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9857</span> | 0.01µs |
| jumpdest | 0.16µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9930</span> | 0.01µs |
| returndatasize | 0.16µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9945</span> | 0.01µs |
| blobbasefee | 0.16µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9941</span> | 0.01µs |
| mstore | 0.16µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9946</span> | 0.01µs |
| codecopy | 0.15µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9996</span> | 0.00µs |
| basefee | 0.15µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9867</span> | 0.01µs |
| eq | 0.15µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9872</span> | 0.01µs |
| extcodehash | 0.15µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9982</span> | 0.00µs |
| modexp | 0.15µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9945</span> | 0.01µs |
| extcodecopy | 0.15µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9967</span> | 0.01µs |
| balance | 0.15µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9993</span> | 0.00µs |
| lt | 0.14µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9970</span> | 0.01µs |
| number | 0.14µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9845</span> | 0.01µs |
| tload | 0.14µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9983</span> | 0.00µs |
| coinbase | 0.14µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9856</span> | 0.01µs |
| mcopy | 0.14µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9948</span> | 0.01µs |
| caller | 0.14µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9845</span> | 0.01µs |
| bls12_g2msm | 0.14µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9966</span> | 0.01µs |
| gt | 0.14µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9895</span> | 0.01µs |
| sload | 0.14µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9979</span> | 0.00µs |
| bls12_g1msm | 0.14µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9974</span> | 0.01µs |
| origin | 0.14µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9678</span> | 0.02µs |
| callcode | 0.14µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9990</span> | 0.00µs |
| shr | 0.14µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9971</span> | 0.01µs |
| bls12_g1add | 0.14µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9995</span> | 0.00µs |
| exp | 0.14µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9968</span> | 0.01µs |
| bls12_map_fp_to_g1 | 0.14µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9993</span> | 0.00µs |
| bls12_g2add | 0.14µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9986</span> | 0.00µs |
| blake2f | 0.14µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9995</span> | 0.00µs |
| prevrandao | 0.14µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9903</span> | 0.01µs |
| bls12_map_fp2_to_g2 | 0.14µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.00µs |
| extcodesize | 0.13µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9978</span> | 0.00µs |
| point_evaluation | 0.13µs/cycle | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.00µs |
| call | 0.13µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9931</span> | 0.01µs |
| bls12_pairing | 0.13µs/cycle | <span style="color: #28a745; font-weight: bold;">1.0000</span> | 0.00µs |
| staticcall | 0.13µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.00µs |
| delegatecall | 0.13µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.00µs |
| selfbalance | 0.13µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9980</span> | 0.00µs |
| timestamp | 0.13µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9886</span> | 0.01µs |
| sstore | 0.13µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9998</span> | 0.00µs |
| msize | 0.13µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9917</span> | 0.01µs |
| push16 | 0.13µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9961</span> | 0.01µs |
| tstore | 0.13µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9995</span> | 0.00µs |
| mstore8 | 0.13µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9736</span> | 0.02µs |
| address | 0.13µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9954</span> | 0.01µs |
| div | 0.13µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9973</span> | 0.00µs |
| or | 0.13µs/cycle | <span style="color: #ffc107; font-weight: bold;">0.8900</span> | 0.02µs |
| addmod | 0.13µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.00µs |
| sdiv | 0.13µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9951</span> | 0.01µs |
| sar | 0.13µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9839</span> | 0.01µs |
| mod | 0.12µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9997</span> | 0.00µs |
| mulmod | 0.12µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9999</span> | 0.00µs |
| shl | 0.12µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9913</span> | 0.01µs |
| smod | 0.12µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9969</span> | 0.00µs |
| mload | 0.12µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9755</span> | 0.01µs |
| mul | 0.12µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9892</span> | 0.01µs |
| iszero | 0.12µs/cycle | <span style="color: #ffc107; font-weight: bold;">0.8634</span> | 0.02µs |
| calldatacopy | 0.12µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9724</span> | 0.01µs |
| calldataload | 0.12µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9983</span> | 0.00µs |
| add | 0.12µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9987</span> | 0.00µs |
| jump | 0.11µs/cycle | <span style="color: #ffc107; font-weight: bold;">0.8975</span> | 0.02µs |
| signextend | 0.11µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9919</span> | 0.01µs |
| not | 0.10µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9312</span> | 0.01µs |
| slt | 0.10µs/cycle | <span style="color: #ffc107; font-weight: bold;">0.8383</span> | 0.02µs |
| sgt | 0.09µs/cycle | <span style="color: #ffc107; font-weight: bold;">0.8261</span> | 0.02µs |
| blobhash | 0.09µs/cycle | <span style="color: #28a745; font-weight: bold;">0.9642</span> | 0.01µs |
| sub | 0.07µs/cycle | <span style="color: #ffc107; font-weight: bold;">0.7967</span> | 0.02µs |
| pop | 0.06µs/cycle | <span style="color: #ffc107; font-weight: bold;">0.7252</span> | 0.02µs |
| identity | 0.44µs/cycle | <span style="color: #dc3545; font-weight: bold;">0.6975</span> | 0.21µs |

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
| add | 97.09 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| addmod | 1.54K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| address | 116.00 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| and | 72.09 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| balance | 1.66K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| basefee | 62.99 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| blake2f | 39.13M | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| blobbasefee | 84.99 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| blobhash | 67.02 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| bls12_g1add | 306.21K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| bls12_g1msm | 8.46M | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| bls12_g2add | 486.16K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| bls12_g2msm | 12.62M | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| bls12_map_fp2_to_g2 | 19.76M | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| bls12_map_fp_to_g1 | 6.40M | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| bls12_pairing | 102.45M | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| bn128_add | 6.08K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| bn128_mul | 464.94K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| bn128_pairing | 6.12M | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| byte | 82.01 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| call | 6.89K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| callcode | 6.69K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| calldatacopy | 212.01 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| calldataload | 163.03 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| calldatasize | 64.01 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| caller | 115.99 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| callvalue | 78.01 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| chainid | 62.99 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| codecopy | 282.40 | <span style="color: #28a745; font-weight: bold;">0.9929</span> |
| codesize | 60.99 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| coinbase | 115.99 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| create | 25.76K | <span style="color: #28a745; font-weight: bold;">0.9996</span> |
| create2 | 26.87K | <span style="color: #28a745; font-weight: bold;">0.9997</span> |
| delegatecall | 5.65K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| div | 1.53K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| dup1 | 51.00 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| dup16 | 50.98 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| dup8 | 51.07 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| ecrecover | 32.02K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| eq | 297.09 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| exp | 141.00K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| extcodecopy | 1.96K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| extcodehash | 2.21K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| extcodesize | 1.67K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| gas | 59.99 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| gaslimit | 59.99 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| gasprice | 80.99 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| gt | 82.09 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| identity | 125.89 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| iszero | 58.00 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| jump | 63.00 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| jumpdest | 26.00 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| jumpi | 116.00 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| keccak256 | 27.84K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| log0 | 208.41 | <span style="color: #28a745; font-weight: bold;">0.9944</span> |
| log1 | 566.34 | <span style="color: #28a745; font-weight: bold;">0.9997</span> |
| log2 | 974.39 | <span style="color: #28a745; font-weight: bold;">0.9999</span> |
| log3 | 1.12K | <span style="color: #28a745; font-weight: bold;">0.9999</span> |
| log4 | 1.23K | <span style="color: #28a745; font-weight: bold;">0.9998</span> |
| lt | 82.09 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| mcopy | 1.47K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| mload | 162.02 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| mod | 1.28K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| modexp | 5.18M | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| msize | 60.01 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| mstore | 188.09 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| mstore8 | 78.02 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| mul | 303.09 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| mulmod | 3.38K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| not | 55.01 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| number | 64.99 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| or | 72.07 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| origin | 115.99 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| pc | 63.99 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| point_evaluation | 147.76M | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| pop | 30.00 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| prevrandao | 157.00 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| push0 | 56.99 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| push1 | 46.01 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| push16 | 85.07 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| push32 | 125.12 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| returndatacopy | 238.50 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| returndatasize | 59.99 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| ripemd160 | 39.98K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| sar | 235.01 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| sdiv | 1.30K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| selfbalance | 1.17K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| sgt | 114.07 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| sha256 | 19.79K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| shl | 151.01 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| shr | 154.01 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| signextend | 267.08 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| sload | 1.50K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| slt | 114.07 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| smod | 398.09 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| sstore | 3.50K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| staticcall | 6.68K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| sub | 112.07 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| swap1 | 160.10 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| swap16 | 160.01 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| swap8 | 160.12 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| timestamp | 64.99 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| tload | 243.36 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| tstore | 1.59K | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| xor | 72.08 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |

### Op Count ↔ Proving Time

| Opcode | Time/Op (s) | R² |
|--------|-------------|-----|
| add | 1.13e-05 | <span style="color: #28a745; font-weight: bold;">0.9987</span> |
| addmod | 1.93e-04 | <span style="color: #28a745; font-weight: bold;">0.9999</span> |
| address | 1.48e-05 | <span style="color: #28a745; font-weight: bold;">0.9953</span> |
| and | 1.57e-05 | <span style="color: #28a745; font-weight: bold;">0.9897</span> |
| balance | 2.43e-04 | <span style="color: #28a745; font-weight: bold;">0.9993</span> |
| basefee | 9.37e-06 | <span style="color: #28a745; font-weight: bold;">0.9866</span> |
| blake2f | 5.35e+00 | <span style="color: #28a745; font-weight: bold;">0.9995</span> |
| blobbasefee | 1.32e-05 | <span style="color: #28a745; font-weight: bold;">0.9940</span> |
| blobhash | 5.77e-06 | <span style="color: #28a745; font-weight: bold;">0.9644</span> |
| bls12_g1add | 4.21e-02 | <span style="color: #28a745; font-weight: bold;">0.9995</span> |
| bls12_g1msm | 1.18e+00 | <span style="color: #28a745; font-weight: bold;">0.9974</span> |
| bls12_g2add | 6.66e-02 | <span style="color: #28a745; font-weight: bold;">0.9986</span> |
| bls12_g2msm | 1.77e+00 | <span style="color: #28a745; font-weight: bold;">0.9966</span> |
| bls12_map_fp2_to_g2 | 2.67e+00 | <span style="color: #28a745; font-weight: bold;">0.9999</span> |
| bls12_map_fp_to_g1 | 8.76e-01 | <span style="color: #28a745; font-weight: bold;">0.9993</span> |
| bls12_pairing | 1.38e+01 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| bn128_add | 5.67e-03 | <span style="color: #28a745; font-weight: bold;">0.9196</span> |
| bn128_mul | 2.42e-01 | <span style="color: #28a745; font-weight: bold;">0.9977</span> |
| bn128_pairing | 2.50e+00 | <span style="color: #28a745; font-weight: bold;">0.9909</span> |
| byte | 1.36e-05 | <span style="color: #28a745; font-weight: bold;">0.9691</span> |
| call | 9.27e-04 | <span style="color: #28a745; font-weight: bold;">0.9931</span> |
| callcode | 9.27e-04 | <span style="color: #28a745; font-weight: bold;">0.9991</span> |
| calldatacopy | 2.48e-05 | <span style="color: #28a745; font-weight: bold;">0.9724</span> |
| calldataload | 1.91e-05 | <span style="color: #28a745; font-weight: bold;">0.9983</span> |
| calldatasize | 1.10e-05 | <span style="color: #28a745; font-weight: bold;">0.9780</span> |
| caller | 1.63e-05 | <span style="color: #28a745; font-weight: bold;">0.9845</span> |
| callvalue | 1.26e-05 | <span style="color: #28a745; font-weight: bold;">0.9789</span> |
| chainid | 1.01e-05 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| codecopy | 4.28e-05 | <span style="color: #28a745; font-weight: bold;">0.9950</span> |
| codesize | 1.13e-05 | <span style="color: #28a745; font-weight: bold;">0.9864</span> |
| coinbase | 1.65e-05 | <span style="color: #28a745; font-weight: bold;">0.9857</span> |
| create | 4.83e-03 | <span style="color: #28a745; font-weight: bold;">0.9997</span> |
| create2 | 4.80e-03 | <span style="color: #28a745; font-weight: bold;">0.9997</span> |
| delegatecall | 7.55e-04 | <span style="color: #28a745; font-weight: bold;">0.9999</span> |
| div | 1.95e-04 | <span style="color: #28a745; font-weight: bold;">0.9973</span> |
| dup1 | 9.14e-06 | <span style="color: #ffc107; font-weight: bold;">0.8879</span> |
| dup16 | 1.06e-05 | <span style="color: #28a745; font-weight: bold;">0.9449</span> |
| dup8 | 9.63e-06 | <span style="color: #28a745; font-weight: bold;">0.9217</span> |
| ecrecover | 2.87e-02 | <span style="color: #28a745; font-weight: bold;">0.9380</span> |
| eq | 4.40e-05 | <span style="color: #28a745; font-weight: bold;">0.9872</span> |
| exp | 1.94e-02 | <span style="color: #28a745; font-weight: bold;">0.9968</span> |
| extcodecopy | 2.86e-04 | <span style="color: #28a745; font-weight: bold;">0.9966</span> |
| extcodehash | 3.24e-04 | <span style="color: #28a745; font-weight: bold;">0.9982</span> |
| extcodesize | 2.25e-04 | <span style="color: #28a745; font-weight: bold;">0.9978</span> |
| gas | 1.10e-05 | <span style="color: #28a745; font-weight: bold;">0.9836</span> |
| gaslimit | 9.57e-06 | <span style="color: #28a745; font-weight: bold;">0.9959</span> |
| gasprice | 1.30e-05 | <span style="color: #28a745; font-weight: bold;">0.9995</span> |
| gt | 1.15e-05 | <span style="color: #28a745; font-weight: bold;">0.9896</span> |
| identity | 5.57e-05 | <span style="color: #ffc107; font-weight: bold;">0.7013</span> |
| iszero | 6.89e-06 | <span style="color: #ffc107; font-weight: bold;">0.8634</span> |
| jump | 7.04e-06 | <span style="color: #ffc107; font-weight: bold;">0.8975</span> |
| jumpdest | 4.10e-06 | <span style="color: #28a745; font-weight: bold;">0.9930</span> |
| jumpi | 2.09e-05 | <span style="color: #28a745; font-weight: bold;">0.9972</span> |
| keccak256 | 1.05e-02 | <span style="color: #28a745; font-weight: bold;">0.9992</span> |
| log0 | 5.59e-05 | <span style="color: #28a745; font-weight: bold;">0.9268</span> |
| log1 | 1.38e-04 | <span style="color: #28a745; font-weight: bold;">0.9995</span> |
| log2 | 2.52e-04 | <span style="color: #28a745; font-weight: bold;">0.9757</span> |
| log3 | 2.42e-04 | <span style="color: #28a745; font-weight: bold;">0.9979</span> |
| log4 | 3.06e-04 | <span style="color: #28a745; font-weight: bold;">0.9898</span> |
| lt | 1.18e-05 | <span style="color: #28a745; font-weight: bold;">0.9971</span> |
| mcopy | 2.07e-04 | <span style="color: #28a745; font-weight: bold;">0.9947</span> |
| mload | 1.96e-05 | <span style="color: #28a745; font-weight: bold;">0.9755</span> |
| mod | 1.59e-04 | <span style="color: #28a745; font-weight: bold;">0.9997</span> |
| modexp | 7.57e-01 | <span style="color: #28a745; font-weight: bold;">0.9945</span> |
| msize | 7.93e-06 | <span style="color: #28a745; font-weight: bold;">0.9917</span> |
| mstore | 2.93e-05 | <span style="color: #28a745; font-weight: bold;">0.9946</span> |
| mstore8 | 1.01e-05 | <span style="color: #28a745; font-weight: bold;">0.9735</span> |
| mul | 3.67e-05 | <span style="color: #28a745; font-weight: bold;">0.9892</span> |
| mulmod | 4.21e-04 | <span style="color: #28a745; font-weight: bold;">0.9999</span> |
| not | 5.66e-06 | <span style="color: #28a745; font-weight: bold;">0.9312</span> |
| number | 9.31e-06 | <span style="color: #28a745; font-weight: bold;">0.9843</span> |
| or | 9.07e-06 | <span style="color: #ffc107; font-weight: bold;">0.8897</span> |
| origin | 1.62e-05 | <span style="color: #28a745; font-weight: bold;">0.9678</span> |
| pc | 1.09e-05 | <span style="color: #28a745; font-weight: bold;">0.9565</span> |
| point_evaluation | 1.99e+01 | <span style="color: #28a745; font-weight: bold;">1.0000</span> |
| pop | 1.91e-06 | <span style="color: #ffc107; font-weight: bold;">0.7252</span> |
| prevrandao | 2.12e-05 | <span style="color: #28a745; font-weight: bold;">0.9904</span> |
| push0 | 9.84e-06 | <span style="color: #28a745; font-weight: bold;">0.9425</span> |
| push1 | 8.24e-06 | <span style="color: #28a745; font-weight: bold;">0.9551</span> |
| push16 | 1.12e-05 | <span style="color: #28a745; font-weight: bold;">0.9960</span> |
| push32 | 2.26e-05 | <span style="color: #28a745; font-weight: bold;">0.9244</span> |
| returndatacopy | 3.79e-05 | <span style="color: #28a745; font-weight: bold;">0.9854</span> |
| returndatasize | 9.41e-06 | <span style="color: #28a745; font-weight: bold;">0.9944</span> |
| ripemd160 | 6.87e-03 | <span style="color: #28a745; font-weight: bold;">0.9559</span> |
| sar | 2.94e-05 | <span style="color: #28a745; font-weight: bold;">0.9839</span> |
| sdiv | 1.63e-04 | <span style="color: #28a745; font-weight: bold;">0.9951</span> |
| selfbalance | 1.56e-04 | <span style="color: #28a745; font-weight: bold;">0.9980</span> |
| sgt | 1.06e-05 | <span style="color: #ffc107; font-weight: bold;">0.8265</span> |
| sha256 | 1.53e-02 | <span style="color: #28a745; font-weight: bold;">0.9985</span> |
| shl | 1.88e-05 | <span style="color: #28a745; font-weight: bold;">0.9913</span> |
| shr | 2.13e-05 | <span style="color: #28a745; font-weight: bold;">0.9971</span> |
| signextend | 2.96e-05 | <span style="color: #28a745; font-weight: bold;">0.9918</span> |
| sload | 2.10e-04 | <span style="color: #28a745; font-weight: bold;">0.9979</span> |
| slt | 1.14e-05 | <span style="color: #ffc107; font-weight: bold;">0.8388</span> |
| smod | 4.86e-05 | <span style="color: #28a745; font-weight: bold;">0.9969</span> |
| sstore | 4.63e-04 | <span style="color: #28a745; font-weight: bold;">0.9998</span> |
| staticcall | 8.98e-04 | <span style="color: #28a745; font-weight: bold;">0.9999</span> |
| sub | 7.77e-06 | <span style="color: #ffc107; font-weight: bold;">0.7971</span> |
| swap1 | 2.85e-05 | <span style="color: #28a745; font-weight: bold;">0.9781</span> |
| swap16 | 3.24e-05 | <span style="color: #28a745; font-weight: bold;">0.9845</span> |
| swap8 | 2.98e-05 | <span style="color: #28a745; font-weight: bold;">0.9847</span> |
| timestamp | 8.66e-06 | <span style="color: #28a745; font-weight: bold;">0.9887</span> |
| tload | 3.47e-05 | <span style="color: #28a745; font-weight: bold;">0.9982</span> |
| tstore | 2.08e-04 | <span style="color: #28a745; font-weight: bold;">0.9995</span> |
| xor | 1.47e-05 | <span style="color: #28a745; font-weight: bold;">0.9720</span> |

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

### sp1 Max ZK Cycles by Opcode (colored by R²)

Bar length = Max ZK Cycles, color = Time/Gas R² (green=high, red=low)

<div style="overflow-x: auto;"><svg width="980" height="2560" xmlns="http://www.w3.org/2000/svg"><text x="490.0" y="20" text-anchor="middle" style="font-family: sans-serif; font-size: 14px; font-weight: bold; fill: #333;">sp1 Max ZK Cycles by Opcode (colored by Time/Gas R²)</text><text x="175" y="50" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">identity</text><rect x="180" y="35" width="600.0" height="20" fill="#ef4444" rx="2"/><text x="785" y="50" style="font-family: monospace; font-size: 11px; fill: #666;">10,200,500,155 (R²=0.7010)</text><text x="175" y="74" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">point_evaluation</text><rect x="180" y="59" width="78.27350999143238" height="20" fill="#22c55e" rx="2"/><text x="785" y="74" style="font-family: monospace; font-size: 11px; fill: #666;">1,330,714,918 (R²=1.0000)</text><text x="175" y="98" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">slt</text><rect x="180" y="83" width="58.91105201399803" height="20" fill="#f97316" rx="2"/><text x="785" y="98" style="font-family: monospace; font-size: 11px; fill: #666;">1,001,536,992 (R²=0.8388)</text><text x="175" y="122" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">bls12_pairing</text><rect x="180" y="107" width="54.28760701783451" height="20" fill="#22c55e" rx="2"/><text x="785" y="122" style="font-family: monospace; font-size: 11px; fill: #666;">922,934,573 (R²=1.0000)</text><text x="175" y="146" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">mulmod</text><rect x="180" y="131" width="47.938568635804316" height="20" fill="#22c55e" rx="2"/><text x="785" y="146" style="font-family: monospace; font-size: 11px; fill: #666;">814,995,628 (R²=0.9999)</text><text x="175" y="170" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">selfbalance</text><rect x="180" y="155" width="44.86236418276884" height="20" fill="#22c55e" rx="2"/><text x="785" y="170" style="font-family: monospace; font-size: 11px; fill: #666;">762,697,588 (R²=0.9980)</text><text x="175" y="194" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">signextend</text><rect x="180" y="179" width="42.44342944181838" height="20" fill="#22c55e" rx="2"/><text x="785" y="194" style="font-family: monospace; font-size: 11px; fill: #666;">721,573,681 (R²=0.9918)</text><text x="175" y="218" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">sub</text><rect x="180" y="203" width="41.919427959657725" height="20" fill="#ef4444" rx="2"/><text x="785" y="218" style="font-family: monospace; font-size: 11px; fill: #666;">712,665,219 (R²=0.7971)</text><text x="175" y="242" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">keccak256</text><rect x="180" y="227" width="41.28440450967279" height="20" fill="#22c55e" rx="2"/><text x="785" y="242" style="font-family: monospace; font-size: 11px; fill: #666;">701,869,291 (R²=0.9992)</text><text x="175" y="266" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">xor</text><rect x="180" y="251" width="38.39016878089543" height="20" fill="#84cc16" rx="2"/><text x="785" y="266" style="font-family: monospace; font-size: 11px; fill: #666;">652,664,871 (R²=0.9720)</text><text x="175" y="290" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">tload</text><rect x="180" y="275" width="36.43503794447028" height="20" fill="#22c55e" rx="2"/><text x="785" y="290" style="font-family: monospace; font-size: 11px; fill: #666;">619,426,017 (R²=0.9982)</text><text x="175" y="314" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">balance</text><rect x="180" y="299" width="32.29466537859192" height="20" fill="#22c55e" rx="2"/><text x="785" y="314" style="font-family: monospace; font-size: 11px; fill: #666;">549,036,232 (R²=0.9993)</text><text x="175" y="338" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">bls12_map_fp2_to_g2</text><rect x="180" y="323" width="31.435044353469745" height="20" fill="#22c55e" rx="2"/><text x="785" y="338" style="font-family: monospace; font-size: 11px; fill: #666;">534,421,958 (R²=0.9999)</text><text x="175" y="362" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">extcodehash</text><rect x="180" y="347" width="28.87401840346328" height="20" fill="#22c55e" rx="2"/><text x="785" y="362" style="font-family: monospace; font-size: 11px; fill: #666;">490,882,382 (R²=0.9982)</text><text x="175" y="386" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">bls12_map_fp_to_g1</text><rect x="180" y="371" width="28.3064135299746" height="20" fill="#22c55e" rx="2"/><text x="785" y="386" style="font-family: monospace; font-size: 11px; fill: #666;">481,232,626 (R²=0.9993)</text><text x="175" y="410" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">smod</text><rect x="180" y="395" width="26.89613185933038" height="20" fill="#22c55e" rx="2"/><text x="785" y="410" style="font-family: monospace; font-size: 11px; fill: #666;">457,256,662 (R²=0.9969)</text><text x="175" y="434" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">sgt</text><rect x="180" y="419" width="25.28068730763134" height="20" fill="#f97316" rx="2"/><text x="785" y="434" style="font-family: monospace; font-size: 11px; fill: #666;">429,792,758 (R²=0.8265)</text><text x="175" y="458" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">codecopy</text><rect x="180" y="443" width="24.55334644323624" height="20" fill="#22c55e" rx="2"/><text x="785" y="458" style="font-family: monospace; font-size: 11px; fill: #666;">417,427,357 (R²=0.9950)</text><text x="175" y="482" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">add</text><rect x="180" y="467" width="24.380740142246488" height="20" fill="#22c55e" rx="2"/><text x="785" y="482" style="font-family: monospace; font-size: 11px; fill: #666;">414,492,906 (R²=0.9987)</text><text x="175" y="506" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">addmod</text><rect x="180" y="491" width="24.22201357243154" height="20" fill="#22c55e" rx="2"/><text x="785" y="506" style="font-family: monospace; font-size: 11px; fill: #666;">411,794,422 (R²=0.9999)</text><text x="175" y="530" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">mul</text><rect x="180" y="515" width="23.543354732687614" height="20" fill="#84cc16" rx="2"/><text x="785" y="530" style="font-family: monospace; font-size: 11px; fill: #666;">400,256,656 (R²=0.9892)</text><text x="175" y="554" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">div</text><rect x="180" y="539" width="23.43414279375598" height="20" fill="#22c55e" rx="2"/><text x="785" y="554" style="font-family: monospace; font-size: 11px; fill: #666;">398,399,962 (R²=0.9973)</text><text x="175" y="578" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">blake2f</text><rect x="180" y="563" width="23.070322868888777" height="20" fill="#22c55e" rx="2"/><text x="785" y="578" style="font-family: monospace; font-size: 11px; fill: #666;">392,214,720 (R²=0.9995)</text><text x="175" y="602" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">or</text><rect x="180" y="587" width="23.057242863205463" height="20" fill="#f97316" rx="2"/><text x="785" y="602" style="font-family: monospace; font-size: 11px; fill: #666;">391,992,349 (R²=0.8897)</text><text x="175" y="626" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">iszero</text><rect x="180" y="611" width="22.839494069886616" height="20" fill="#f97316" rx="2"/><text x="785" y="626" style="font-family: monospace; font-size: 11px; fill: #666;">388,290,438 (R²=0.8634)</text><text x="175" y="650" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">extcodesize</text><rect x="180" y="635" width="22.138947244592245" height="20" fill="#22c55e" rx="2"/><text x="785" y="650" style="font-family: monospace; font-size: 11px; fill: #666;">376,380,558 (R²=0.9978)</text><text x="175" y="674" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">tstore</text><rect x="180" y="659" width="21.626211621777088" height="20" fill="#22c55e" rx="2"/><text x="785" y="674" style="font-family: monospace; font-size: 11px; fill: #666;">367,663,625 (R²=0.9995)</text><text x="175" y="698" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">call</text><rect x="180" y="683" width="21.55142017151413" height="20" fill="#22c55e" rx="2"/><text x="785" y="698" style="font-family: monospace; font-size: 11px; fill: #666;">366,392,108 (R²=0.9932)</text><text x="175" y="722" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">callcode</text><rect x="180" y="707" width="20.982645120110778" height="20" fill="#22c55e" rx="2"/><text x="785" y="722" style="font-family: monospace; font-size: 11px; fill: #666;">356,722,458 (R²=0.9991)</text><text x="175" y="746" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">staticcall</text><rect x="180" y="731" width="20.810206105035835" height="20" fill="#22c55e" rx="2"/><text x="785" y="746" style="font-family: monospace; font-size: 11px; fill: #666;">353,790,851 (R²=0.9999)</text><text x="175" y="770" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">sdiv</text><rect x="180" y="755" width="20.61782277380888" height="20" fill="#22c55e" rx="2"/><text x="785" y="770" style="font-family: monospace; font-size: 11px; fill: #666;">350,520,174 (R²=0.9951)</text><text x="175" y="794" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">mod</text><rect x="180" y="779" width="20.30899938749131" height="20" fill="#22c55e" rx="2"/><text x="785" y="794" style="font-family: monospace; font-size: 11px; fill: #666;">345,269,919 (R²=0.9997)</text><text x="175" y="818" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">sload</text><rect x="180" y="803" width="20.150910198187237" height="20" fill="#22c55e" rx="2"/><text x="785" y="818" style="font-family: monospace; font-size: 11px; fill: #666;">342,582,271 (R²=0.9979)</text><text x="175" y="842" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">blobhash</text><rect x="180" y="827" width="20.04906025120295" height="20" fill="#84cc16" rx="2"/><text x="785" y="842" style="font-family: monospace; font-size: 11px; fill: #666;">340,850,737 (R²=0.9644)</text><text x="175" y="866" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">bls12_g1msm</text><rect x="180" y="851" width="19.48041784035442" height="20" fill="#22c55e" rx="2"/><text x="785" y="866" style="font-family: monospace; font-size: 11px; fill: #666;">331,183,342 (R²=0.9974)</text><text x="175" y="890" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">mload</text><rect x="180" y="875" width="19.043612788416254" height="20" fill="#84cc16" rx="2"/><text x="785" y="890" style="font-family: monospace; font-size: 11px; fill: #666;">323,757,292 (R²=0.9755)</text><text x="175" y="914" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">exp</text><rect x="180" y="899" width="18.766469554550977" height="20" fill="#22c55e" rx="2"/><text x="785" y="914" style="font-family: monospace; font-size: 11px; fill: #666;">319,045,626 (R²=0.9968)</text><text x="175" y="938" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">bls12_g2msm</text><rect x="180" y="923" width="17.88194986797684" height="20" fill="#22c55e" rx="2"/><text x="785" y="938" style="font-family: monospace; font-size: 11px; fill: #666;">304,008,054 (R²=0.9966)</text><text x="175" y="962" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">delegatecall</text><rect x="180" y="947" width="17.770970898044165" height="20" fill="#22c55e" rx="2"/><text x="785" y="962" style="font-family: monospace; font-size: 11px; fill: #666;">302,121,319 (R²=0.9999)</text><text x="175" y="986" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">eq</text><rect x="180" y="971" width="17.51319942017098" height="20" fill="#84cc16" rx="2"/><text x="785" y="986" style="font-family: monospace; font-size: 11px; fill: #666;">297,738,989 (R²=0.9872)</text><text x="175" y="1010" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">jumpi</text><rect x="180" y="995" width="16.59549477257961" height="20" fill="#22c55e" rx="2"/><text x="785" y="1010" style="font-family: monospace; font-size: 11px; fill: #666;">282,137,245 (R²=0.9972)</text><text x="175" y="1034" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">gt</text><rect x="180" y="1019" width="15.743752184669223" height="20" fill="#84cc16" rx="2"/><text x="785" y="1034" style="font-family: monospace; font-size: 11px; fill: #666;">267,656,911 (R²=0.9896)</text><text x="175" y="1058" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">lt</text><rect x="180" y="1043" width="15.74372248024342" height="20" fill="#22c55e" rx="2"/><text x="785" y="1058" style="font-family: monospace; font-size: 11px; fill: #666;">267,656,406 (R²=0.9971)</text><text x="175" y="1082" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">shr</text><rect x="180" y="1067" width="15.490712474774723" height="20" fill="#22c55e" rx="2"/><text x="785" y="1082" style="font-family: monospace; font-size: 11px; fill: #666;">263,355,025 (R²=0.9971)</text><text x="175" y="1106" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">and</text><rect x="180" y="1091" width="15.390820196502412" height="20" fill="#84cc16" rx="2"/><text x="785" y="1106" style="font-family: monospace; font-size: 11px; fill: #666;">261,656,773 (R²=0.9897)</text><text x="175" y="1130" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">coinbase</text><rect x="180" y="1115" width="15.27612752631736" height="20" fill="#84cc16" rx="2"/><text x="785" y="1130" style="font-family: monospace; font-size: 11px; fill: #666;">259,706,902 (R²=0.9857)</text><text x="175" y="1154" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">caller</text><rect x="180" y="1139" width="15.276120232557362" height="20" fill="#84cc16" rx="2"/><text x="785" y="1154" style="font-family: monospace; font-size: 11px; fill: #666;">259,706,778 (R²=0.9845)</text><text x="175" y="1178" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">origin</text><rect x="180" y="1163" width="15.276110468330266" height="20" fill="#84cc16" rx="2"/><text x="785" y="1178" style="font-family: monospace; font-size: 11px; fill: #666;">259,706,612 (R²=0.9678)</text><text x="175" y="1202" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">not</text><rect x="180" y="1187" width="15.08642688707454" height="20" fill="#eab308" rx="2"/><text x="785" y="1202" style="font-family: monospace; font-size: 11px; fill: #666;">256,481,833 (R²=0.9312)</text><text x="175" y="1226" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">extcodecopy</text><rect x="180" y="1211" width="14.229658016215186" height="20" fill="#22c55e" rx="2"/><text x="785" y="1226" style="font-family: monospace; font-size: 11px; fill: #666;">241,916,048 (R²=0.9967)</text><text x="175" y="1250" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">bls12_g1add</text><rect x="180" y="1235" width="14.219471809811466" height="20" fill="#22c55e" rx="2"/><text x="785" y="1250" style="font-family: monospace; font-size: 11px; fill: #666;">241,742,874 (R²=0.9995)</text><text x="175" y="1274" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">calldatacopy</text><rect x="180" y="1259" width="14.127503437109649" height="20" fill="#84cc16" rx="2"/><text x="785" y="1274" style="font-family: monospace; font-size: 11px; fill: #666;">240,179,335 (R²=0.9724)</text><text x="175" y="1298" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">sstore</text><rect x="180" y="1283" width="13.93316702518103" height="20" fill="#22c55e" rx="2"/><text x="785" y="1298" style="font-family: monospace; font-size: 11px; fill: #666;">236,875,454 (R²=0.9998)</text><text x="175" y="1322" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">sar</text><rect x="180" y="1307" width="13.775823858119436" height="20" fill="#84cc16" rx="2"/><text x="785" y="1322" style="font-family: monospace; font-size: 11px; fill: #666;">234,200,489 (R²=0.9839)</text><text x="175" y="1346" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">prevrandao</text><rect x="180" y="1331" width="13.6406343106418" height="20" fill="#22c55e" rx="2"/><text x="785" y="1346" style="font-family: monospace; font-size: 11px; fill: #666;">231,902,154 (R²=0.9904)</text><text x="175" y="1370" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">blobbasefee</text><rect x="180" y="1355" width="13.087979939352296" height="20" fill="#22c55e" rx="2"/><text x="785" y="1370" style="font-family: monospace; font-size: 11px; fill: #666;">222,506,569 (R²=0.9940)</text><text x="175" y="1394" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">bls12_g2add</text><rect x="180" y="1379" width="13.083937431693515" height="20" fill="#22c55e" rx="2"/><text x="785" y="1394" style="font-family: monospace; font-size: 11px; fill: #666;">222,437,843 (R²=0.9986)</text><text x="175" y="1418" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">byte</text><rect x="180" y="1403" width="12.949658721906074" height="20" fill="#84cc16" rx="2"/><text x="785" y="1418" style="font-family: monospace; font-size: 11px; fill: #666;">220,154,993 (R²=0.9691)</text><text x="175" y="1442" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">gasprice</text><rect x="180" y="1427" width="12.805650665665814" height="20" fill="#22c55e" rx="2"/><text x="785" y="1442" style="font-family: monospace; font-size: 11px; fill: #666;">217,706,736 (R²=0.9995)</text><text x="175" y="1466" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">callvalue</text><rect x="180" y="1451" width="12.593892225669986" height="20" fill="#84cc16" rx="2"/><text x="785" y="1466" style="font-family: monospace; font-size: 11px; fill: #666;">214,106,666 (R²=0.9789)</text><text x="175" y="1490" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">number</text><rect x="180" y="1475" width="11.676303572390857" height="20" fill="#84cc16" rx="2"/><text x="785" y="1490" style="font-family: monospace; font-size: 11px; fill: #666;">198,506,894 (R²=0.9843)</text><text x="175" y="1514" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">timestamp</text><rect x="180" y="1499" width="11.676257809928929" height="20" fill="#84cc16" rx="2"/><text x="785" y="1514" style="font-family: monospace; font-size: 11px; fill: #666;">198,506,116 (R²=0.9887)</text><text x="175" y="1538" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">mstore8</text><rect x="180" y="1523" width="11.666854035747035" height="20" fill="#84cc16" rx="2"/><text x="785" y="1538" style="font-family: monospace; font-size: 11px; fill: #666;">198,346,244 (R²=0.9735)</text><text x="175" y="1562" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">pc</text><rect x="180" y="1547" width="11.60571009275182" height="20" fill="#84cc16" rx="2"/><text x="785" y="1562" style="font-family: monospace; font-size: 11px; fill: #666;">197,306,746 (R²=0.9565)</text><text x="175" y="1586" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">calldatasize</text><rect x="180" y="1571" width="11.60570244606795" height="20" fill="#84cc16" rx="2"/><text x="785" y="1586" style="font-family: monospace; font-size: 11px; fill: #666;">197,306,616 (R²=0.9780)</text><text x="175" y="1610" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">shl</text><rect x="180" y="1595" width="11.552381119492273" height="20" fill="#22c55e" rx="2"/><text x="785" y="1610" style="font-family: monospace; font-size: 11px; fill: #666;">196,400,109 (R²=0.9913)</text><text x="175" y="1634" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">chainid</text><rect x="180" y="1619" width="11.535116613112782" height="20" fill="#22c55e" rx="2"/><text x="785" y="1634" style="font-family: monospace; font-size: 11px; fill: #666;">196,106,598 (R²=1.0000)</text><text x="175" y="1658" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">basefee</text><rect x="180" y="1643" width="11.535088496844399" height="20" fill="#84cc16" rx="2"/><text x="785" y="1658" style="font-family: monospace; font-size: 11px; fill: #666;">196,106,120 (R²=0.9866)</text><text x="175" y="1682" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">address</text><rect x="180" y="1667" width="11.470159386512945" height="20" fill="#22c55e" rx="2"/><text x="785" y="1682" style="font-family: monospace; font-size: 11px; fill: #666;">195,002,271 (R²=0.9953)</text><text x="175" y="1706" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">mstore</text><rect x="180" y="1691" width="11.435484498553983" height="20" fill="#22c55e" rx="2"/><text x="785" y="1706" style="font-family: monospace; font-size: 11px; fill: #666;">194,412,769 (R²=0.9946)</text><text x="175" y="1730" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">codesize</text><rect x="180" y="1715" width="11.393959887646313" height="20" fill="#84cc16" rx="2"/><text x="785" y="1730" style="font-family: monospace; font-size: 11px; fill: #666;">193,706,816 (R²=0.9864)</text><text x="175" y="1754" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">msize</text><rect x="180" y="1739" width="11.374979406585775" height="20" fill="#22c55e" rx="2"/><text x="785" y="1754" style="font-family: monospace; font-size: 11px; fill: #666;">193,384,132 (R²=0.9917)</text><text x="175" y="1778" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">gas</text><rect x="180" y="1763" width="11.323371407762114" height="20" fill="#84cc16" rx="2"/><text x="785" y="1778" style="font-family: monospace; font-size: 11px; fill: #666;">192,506,753 (R²=0.9836)</text><text x="175" y="1802" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">gaslimit</text><rect x="180" y="1787" width="11.323353585106632" height="20" fill="#22c55e" rx="2"/><text x="785" y="1802" style="font-family: monospace; font-size: 11px; fill: #666;">192,506,450 (R²=0.9959)</text><text x="175" y="1826" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">returndatasize</text><rect x="180" y="1811" width="11.32333770353244" height="20" fill="#22c55e" rx="2"/><text x="785" y="1826" style="font-family: monospace; font-size: 11px; fill: #666;">192,506,180 (R²=0.9944)</text><text x="175" y="1850" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">push0</text><rect x="180" y="1835" width="11.11157991056371" height="20" fill="#eab308" rx="2"/><text x="785" y="1850" style="font-family: monospace; font-size: 11px; fill: #666;">188,906,121 (R²=0.9425)</text><text x="175" y="1874" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">push16</text><rect x="180" y="1859" width="11.087031643694926" height="20" fill="#22c55e" rx="2"/><text x="785" y="1874" style="font-family: monospace; font-size: 11px; fill: #666;">188,488,780 (R²=0.9960)</text><text x="175" y="1898" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">modexp</text><rect x="180" y="1883" width="11.025263143089479" height="20" fill="#22c55e" rx="2"/><text x="785" y="1898" style="font-family: monospace; font-size: 11px; fill: #666;">187,438,664 (R²=0.9945)</text><text x="175" y="1922" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">pop</text><rect x="180" y="1907" width="10.605832592137242" height="20" fill="#ef4444" rx="2"/><text x="785" y="1922" style="font-family: monospace; font-size: 11px; fill: #666;">180,307,995 (R²=0.7252)</text><text x="175" y="1946" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">push1</text><rect x="180" y="1931" width="10.335409146415527" height="20" fill="#84cc16" rx="2"/><text x="785" y="1946" style="font-family: monospace; font-size: 11px; fill: #666;">175,710,571 (R²=0.9551)</text><text x="175" y="1970" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">calldataload</text><rect x="180" y="1955" width="9.546996570777466" height="20" fill="#22c55e" rx="2"/><text x="785" y="1970" style="font-family: monospace; font-size: 11px; fill: #666;">162,306,900 (R²=0.9983)</text><text x="175" y="1994" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">dup8</text><rect x="180" y="1979" width="8.811910262649272" height="20" fill="#eab308" rx="2"/><text x="785" y="1994" style="font-family: monospace; font-size: 11px; fill: #666;">149,809,820 (R²=0.9217)</text><text x="175" y="2018" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">dup1</text><rect x="180" y="2003" width="8.713236395220662" height="20" fill="#f97316" rx="2"/><text x="785" y="2018" style="font-family: monospace; font-size: 11px; fill: #666;">148,132,282 (R²=0.8879)</text><text x="175" y="2042" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">push32</text><rect x="180" y="2027" width="7.273157303334211" height="20" fill="#eab308" rx="2"/><text x="785" y="2042" style="font-family: monospace; font-size: 11px; fill: #666;">123,649,737 (R²=0.9244)</text><text x="175" y="2066" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">create2</text><rect x="180" y="2051" width="6.920687821900239" height="20" fill="#22c55e" rx="2"/><text x="785" y="2066" style="font-family: monospace; font-size: 11px; fill: #666;">117,657,462 (R²=0.9997)</text><text x="175" y="2090" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">create</text><rect x="180" y="2075" width="6.570067955653102" height="20" fill="#22c55e" rx="2"/><text x="785" y="2090" style="font-family: monospace; font-size: 11px; fill: #666;">111,696,632 (R²=0.9997)</text><text x="175" y="2114" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">jump</text><rect x="180" y="2099" width="6.441150982953934" height="20" fill="#f97316" rx="2"/><text x="785" y="2114" style="font-family: monospace; font-size: 11px; fill: #666;">109,504,936 (R²=0.8975)</text><text x="175" y="2138" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">dup16</text><rect x="180" y="2123" width="5.960171685326541" height="20" fill="#eab308" rx="2"/><text x="785" y="2138" style="font-family: monospace; font-size: 11px; fill: #666;">101,327,887 (R²=0.9449)</text><text x="175" y="2162" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">mcopy</text><rect x="180" y="2147" width="5.9071584024695305" height="20" fill="#22c55e" rx="2"/><text x="785" y="2162" style="font-family: monospace; font-size: 11px; fill: #666;">100,426,617 (R²=0.9951)</text><text x="175" y="2186" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">jumpdest</text><rect x="180" y="2171" width="5.512250060840277" height="20" fill="#22c55e" rx="2"/><text x="785" y="2186" style="font-family: monospace; font-size: 11px; fill: #666;">93,712,846 (R²=0.9930)</text><text x="175" y="2210" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">returndatacopy</text><rect x="180" y="2195" width="4.684317423060713" height="20" fill="#84cc16" rx="2"/><text x="785" y="2210" style="font-family: monospace; font-size: 11px; fill: #666;">79,637,301 (R²=0.9857)</text><text x="175" y="2234" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">ripemd160</text><rect x="180" y="2219" width="4.130805858509396" height="20" fill="#84cc16" rx="2"/><text x="785" y="2234" style="font-family: monospace; font-size: 11px; fill: #666;">70,227,143 (R²=0.9559)</text><text x="175" y="2258" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">log1</text><rect x="180" y="2243" width="3.9082116557254243" height="20" fill="#22c55e" rx="2"/><text x="785" y="2258" style="font-family: monospace; font-size: 11px; fill: #666;">66,442,856 (R²=0.9995)</text><text x="175" y="2282" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">swap16</text><rect x="180" y="2267" width="3.398208722442787" height="20" fill="#84cc16" rx="2"/><text x="785" y="2282" style="font-family: monospace; font-size: 11px; fill: #666;">57,772,381 (R²=0.9845)</text><text x="175" y="2306" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">swap8</text><rect x="180" y="2291" width="3.3640359275108502" height="20" fill="#84cc16" rx="2"/><text x="785" y="2306" style="font-family: monospace; font-size: 11px; fill: #666;">57,191,415 (R²=0.9847)</text><text x="175" y="2330" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">swap1</text><rect x="180" y="2315" width="3.3341561573654035" height="20" fill="#84cc16" rx="2"/><text x="785" y="2330" style="font-family: monospace; font-size: 11px; fill: #666;">56,683,434 (R²=0.9781)</text><text x="175" y="2354" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">log2</text><rect x="180" y="2339" width="3.262587843174245" height="20" fill="#84cc16" rx="2"/><text x="785" y="2354" style="font-family: monospace; font-size: 11px; fill: #666;">55,466,713 (R²=0.9756)</text><text x="175" y="2378" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">log3</text><rect x="180" y="2363" width="3.2355636388890385" height="20" fill="#22c55e" rx="2"/><text x="785" y="2378" style="font-family: monospace; font-size: 11px; fill: #666;">55,007,279 (R²=0.9979)</text><text x="175" y="2402" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">bn128_pairing</text><rect x="180" y="2387" width="2.9342869413445936" height="20" fill="#22c55e" rx="2"/><text x="785" y="2402" style="font-family: monospace; font-size: 11px; fill: #666;">49,885,324 (R²=0.9909)</text><text x="175" y="2426" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">log4</text><rect x="180" y="2411" width="2.8125452050444086" height="20" fill="#84cc16" rx="2"/><text x="785" y="2426" style="font-family: monospace; font-size: 11px; fill: #666;">47,815,613 (R²=0.9897)</text><text x="175" y="2450" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">log0</text><rect x="180" y="2435" width="2.734298747726454" height="20" fill="#eab308" rx="2"/><text x="785" y="2450" style="font-family: monospace; font-size: 11px; fill: #666;">46,485,358 (R²=0.9268)</text><text x="175" y="2474" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">sha256</text><rect x="180" y="2459" width="2.348949702064879" height="20" fill="#22c55e" rx="2"/><text x="785" y="2474" style="font-family: monospace; font-size: 11px; fill: #666;">39,934,103 (R²=0.9985)</text><text x="175" y="2498" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">bn128_mul</text><rect x="180" y="2483" width="2.045882033516816" height="20" fill="#22c55e" rx="2"/><text x="785" y="2498" style="font-family: monospace; font-size: 11px; fill: #666;">34,781,700 (R²=0.9977)</text><text x="175" y="2522" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">bn128_add</text><rect x="180" y="2507" width="1" height="20" fill="#eab308" rx="2"/><text x="785" y="2522" style="font-family: monospace; font-size: 11px; fill: #666;">10,688,648 (R²=0.9196)</text><text x="175" y="2546" text-anchor="end" style="font-family: monospace; font-size: 12px; fill: #333;">ecrecover</text><rect x="180" y="2531" width="1" height="20" fill="#eab308" rx="2"/><text x="785" y="2546" style="font-family: monospace; font-size: 11px; fill: #666;">8,538,758 (R²=0.9380)</text></svg></div>
