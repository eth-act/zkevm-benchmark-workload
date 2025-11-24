# RISC0 vs SP1 Performance Comparison

**Comparing:** `zkevm-metrics-risc0-1M-1` vs `zkevm-metrics-sp1-1M`

**Total Tests Compared:** 503

---

## Summary Statistics

### ⏱️ Proving Time Performance

| Metric | Value |
|--------|-------|
| **Average Speedup** (RISC0/SP1) | **0.208x** |
| **Median Speedup** | **0.204x** |
| **Min Speedup** | 0.014x |
| **Max Speedup** | 0.610x |
| **Total RISC0 Time** | 96,067s (26.7 hours) |
| **Total SP1 Time** | 498,745s (138.5 hours) |
| **Time Saved** | 402,678s (111.9 hours) |

✅ **RISC0 is 4.80x faster** on average

### 💾 Memory Usage

| Metric | Value |
|--------|-------|
| **Average Memory Ratio** (RISC0/SP1) | **1.459x** |
| **Median Memory Ratio** | **1.416x** |
| **Min Memory Ratio** | 0.284x |
| **Max Memory Ratio** | 3.359x |

✅ **SP1 uses 1.46x less memory** on average

---

## Detailed Per-Test Comparison

| Test Name | RISC0 Time (s) | SP1 Time (s) | Proving Time Winner | RISC0 Memory (GB) | SP1 Memory (GB) | Memory Winner |
|-----------|----------------|--------------|---------------------|-------------------|-----------------|---------------|
| PRECOMPILE_BLS12_MAP_FP_TO_G1, CALL, STATICCALL (Prague, 1M gas) | 78.7 | 5,652.6 | **RISC0 is 71.8x faster** | 672.38 | 477.65 | SP1 uses 1.4x less |
| PRECOMPILE_BLS12_MAP_FP2_TO_G2, CALL, STATICCALL (Prague, 1M gas) | 61.8 | 4,032.5 | **RISC0 is 65.3x faster** | 594.57 | 522.65 | SP1 uses 1.1x less |
| PRECOMPILE_BLS12_G1ADD, CALL, STATICCALL (Prague, 1M gas) | 60.9 | 3,211.8 | **RISC0 is 52.8x faster** | 647.07 | 373.25 | SP1 uses 1.7x less |
| PRECOMPILE_BLS12_G2ADD, CALL, STATICCALL (Prague, 1M gas) | 68.8 | 3,455.7 | **RISC0 is 50.2x faster** | 643.32 | 437.34 | SP1 uses 1.5x less |
| PRECOMPILE_BLS12_PAIRING, CALL, STATICCALL (Prague, 1M gas) | 113.5 | 5,157.3 | **RISC0 is 45.4x faster** | 670.04 | 474.84 | SP1 uses 1.4x less |
| PRECOMPILE_BLS12_G1MSM, CALL, STATICCALL (Prague, 1M gas) | 86.9 | 2,389.8 | **RISC0 is 27.5x faster** | 669.57 | 522.18 | SP1 uses 1.3x less |
| PRECOMPILE_BLS12_G2MSM, CALL, STATICCALL (Prague, 1M gas) | 63.8 | 1,738.4 | **RISC0 is 27.3x faster** | 713.16 | 476.71 | SP1 uses 1.5x less |
| PRECOMPILE_SHA2-256, CALL (Prague, 1M gas, SHA2, 256) | 20.0 | 415.5 | **RISC0 is 20.8x faster** | 666.76 | 450.93 | SP1 uses 1.5x less |
| PRECOMPILE_POINT_EVALUATION, CALL, STATICCALL (Prague, 1M gas, point evaluation) | 257.2 | 2,726.4 | **RISC0 is 10.6x faster** | 672.38 | 435.93 | SP1 uses 1.5x less |
| CALL, STATICCALL (Prague, 1M gas) | 237.0 | 1,472.0 | **RISC0 is 6.2x faster** | 645.20 | 449.05 | SP1 uses 1.4x less |
| PRECOMPILE_EC_MUL, CALL, STATICCALL (Prague, 1M gas) | 242.3 | 1,504.2 | **RISC0 is 6.2x faster** | 243.95 | 473.43 | **RISC0 uses 1.9x less** |
| CALL, STATICCALL (Prague, 1M gas) | 239.8 | 1,458.6 | **RISC0 is 6.1x faster** | 713.16 | 477.65 | SP1 uses 1.5x less |
| CALL, STATICCALL (Prague, 1M gas) | 19.4 | 114.5 | **RISC0 is 5.9x faster** | 672.38 | 477.18 | SP1 uses 1.4x less |
| SGT (Prague, 1M gas, ) | 73.4 | 426.3 | **RISC0 is 5.8x faster** | 668.63 | 450.93 | SP1 uses 1.5x less |
| ISZERO (Prague, 1M gas) | 69.0 | 396.7 | **RISC0 is 5.8x faster** | 670.04 | 341.85 | SP1 uses 2.0x less |
| PREVRANDAO (Prague, 1M gas) | 120.7 | 692.5 | **RISC0 is 5.7x faster** | 678.01 | 525.93 | SP1 uses 1.3x less |
| PRECOMPILE_EC_ADD, CALL, STATICCALL (Prague, 1M gas) | 79.8 | 455.1 | **RISC0 is 5.7x faster** | 711.76 | 474.84 | SP1 uses 1.5x less |
| CALL, STATICCALL (Prague, 1M gas) | 19.5 | 110.8 | **RISC0 is 5.7x faster** | 669.57 | 449.05 | SP1 uses 1.5x less |
| CALL, STATICCALL (Prague, 1M gas) | 156.4 | 885.3 | **RISC0 is 5.7x faster** | 642.85 | 477.65 | SP1 uses 1.3x less |
| MOD (Prague, 1M gas, ) | 57.4 | 321.3 | **RISC0 is 5.6x faster** | 713.16 | 437.34 | SP1 uses 1.6x less |
| EQ (Prague, 1M gas, ) | 71.7 | 401.1 | **RISC0 is 5.6x faster** | 201.76 | 449.05 | **RISC0 uses 2.2x less** |
| CALL, STATICCALL (Prague, 1M gas) | 18.5 | 101.4 | **RISC0 is 5.5x faster** | 698.63 | 376.07 | SP1 uses 1.9x less |
| JUMPI (Prague, 1M gas) | 42.6 | 233.3 | **RISC0 is 5.5x faster** | 632.54 | 477.65 | SP1 uses 1.3x less |
| SWAP6 (Prague, 1M gas) | 65.5 | 352.8 | **RISC0 is 5.4x faster** | 613.32 | 449.05 | SP1 uses 1.4x less |
| CALLDATACOPY (Prague, 1M gas, non zero data: False, fixed src dst: False, 1MiB, call) | 14.2 | 76.1 | **RISC0 is 5.4x faster** | 645.66 | 373.25 | SP1 uses 1.7x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 5 mod vul nagydani) | 454.3 | 2,441.4 | **RISC0 is 5.4x faster** | 712.23 | 341.38 | SP1 uses 2.1x less |
| JUMPDEST (Prague, 1M gas) | 33.6 | 180.2 | **RISC0 is 5.4x faster** | 633.95 | 477.65 | SP1 uses 1.3x less |
| SWAP12 (Prague, 1M gas) | 64.4 | 345.6 | **RISC0 is 5.4x faster** | 713.16 | 335.75 | SP1 uses 2.1x less |
| CALL, STATICCALL (Prague, 1M gas) | 84.9 | 454.6 | **RISC0 is 5.4x faster** | 613.32 | 477.65 | SP1 uses 1.3x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 128 mod even) | 604.7 | 3,212.2 | **RISC0 is 5.3x faster** | 677.54 | 450.93 | SP1 uses 1.5x less |
| SWAP13 (Prague, 1M gas) | 65.1 | 345.3 | **RISC0 is 5.3x faster** | 713.16 | 522.18 | SP1 uses 1.4x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 1045 mod) | 2,913.4 | 15,431.0 | **RISC0 is 5.3x faster** | 711.76 | 335.75 | SP1 uses 2.1x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 256 mod even) | 430.2 | 2,270.6 | **RISC0 is 5.3x faster** | 596.45 | 340.91 | SP1 uses 1.7x less |
| SLOAD, SSTORE (Prague, 1M gas, SSTORE new value) | 51.5 | 271.5 | **RISC0 is 5.3x faster** | 669.57 | 522.18 | SP1 uses 1.3x less |
| SWAP7 (Prague, 1M gas) | 65.9 | 346.4 | **RISC0 is 5.3x faster** | 620.35 | 477.65 | SP1 uses 1.3x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 3 mod vul guido) | 1,474.4 | 7,754.8 | **RISC0 is 5.3x faster** | 670.04 | 376.07 | SP1 uses 1.8x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 2 mod vul guido) | 896.1 | 4,703.1 | **RISC0 is 5.2x faster** | 633.95 | 449.05 | SP1 uses 1.4x less |
| DUP8 (Prague, 1M gas) | 31.5 | 164.7 | **RISC0 is 5.2x faster** | 633.95 | 475.30 | SP1 uses 1.3x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 1 mod vul guido) | 533.3 | 2,787.5 | **RISC0 is 5.2x faster** | 668.63 | 449.05 | SP1 uses 1.5x less |
| CALLDATACOPY (Prague, 1M gas, non zero data: False, fixed src dst: True, 10KiB, transaction) | 18.2 | 95.4 | **RISC0 is 5.2x faster** | 711.76 | 522.65 | SP1 uses 1.4x less |
| SWAP5 (Prague, 1M gas) | 65.3 | 341.4 | **RISC0 is 5.2x faster** | 620.82 | 479.52 | SP1 uses 1.3x less |
| MCOPY (Prague, 1M gas, fixed src dst: False, 0 bytes) | 49.9 | 260.7 | **RISC0 is 5.2x faster** | 613.32 | 341.38 | SP1 uses 1.8x less |
| RETURNDATACOPY (Prague, 1M gas, fixed dst: True, 10KiB) | 22.2 | 115.9 | **RISC0 is 5.2x faster** | 713.16 | 376.07 | SP1 uses 1.9x less |
| SWAP4 (Prague, 1M gas) | 64.6 | 336.5 | **RISC0 is 5.2x faster** | 698.63 | 352.16 | SP1 uses 2.0x less |
| SWAP9 (Prague, 1M gas) | 65.1 | 338.7 | **RISC0 is 5.2x faster** | 669.57 | 522.65 | SP1 uses 1.3x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 800 mod) | 2,921.8 | 15,203.5 | **RISC0 is 5.2x faster** | 633.95 | 449.05 | SP1 uses 1.4x less |
| SMOD (Prague, 1M gas, ) | 67.3 | 350.3 | **RISC0 is 5.2x faster** | 646.13 | 376.07 | SP1 uses 1.7x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 256 mod odd) | 431.6 | 2,244.1 | **RISC0 is 5.2x faster** | 509.26 | 436.87 | SP1 uses 1.2x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 867 mod) | 2,945.1 | 15,307.2 | **RISC0 is 5.2x faster** | 672.38 | 476.24 | SP1 uses 1.4x less |
| SLOAD, SSTORE (Prague, 1M gas, SLOAD) | 26.5 | 137.7 | **RISC0 is 5.2x faster** | 633.95 | 373.25 | SP1 uses 1.7x less |
| MSTORE (Prague, 1M gas, big memory expansion: True, offset initialized: False, 0 offset) | 51.5 | 266.7 | **RISC0 is 5.2x faster** | 616.60 | 341.85 | SP1 uses 1.8x less |
| SWAP10 (Prague, 1M gas) | 65.9 | 340.9 | **RISC0 is 5.2x faster** | 640.04 | 449.05 | SP1 uses 1.4x less |
| CODECOPY (Prague, 1M gas, fixed src dst: True, 0.50x max code size) | 22.4 | 115.8 | **RISC0 is 5.2x faster** | 698.63 | 477.65 | SP1 uses 1.5x less |
| MSTORE8 (Prague, 1M gas, big memory expansion: True, offset initialized: False, 31 offset) | 38.7 | 200.0 | **RISC0 is 5.2x faster** | 668.63 | 526.40 | SP1 uses 1.3x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 616 mod) | 2,875.6 | 14,848.6 | **RISC0 is 5.2x faster** | 670.04 | 435.93 | SP1 uses 1.5x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 5 mod vul nagydani) | 3,141.2 | 16,216.7 | **RISC0 is 5.2x faster** | 345.20 | 341.85 | SP1 uses 1.0x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 4 mod vul nagydani) | 509.1 | 2,628.1 | **RISC0 is 5.2x faster** | 649.41 | 376.07 | SP1 uses 1.7x less |
| EXTCODECOPY (Prague, 1M gas, 1KiB) | 24.5 | 126.3 | **RISC0 is 5.2x faster** | 601.13 | 449.05 | SP1 uses 1.3x less |
| SWAP2 (Prague, 1M gas) | 65.0 | 335.1 | **RISC0 is 5.2x faster** | 668.63 | 449.05 | SP1 uses 1.5x less |
| GASPRICE (Prague, 1M gas) | 46.8 | 240.7 | **RISC0 is 5.1x faster** | 633.95 | 451.40 | SP1 uses 1.4x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 64 mod odd) | 671.6 | 3,454.8 | **RISC0 is 5.1x faster** | 633.95 | 435.93 | SP1 uses 1.5x less |
| PUSH8 (Prague, 1M gas) | 37.6 | 193.5 | **RISC0 is 5.1x faster** | 712.70 | 435.93 | SP1 uses 1.6x less |
| SWAP15 (Prague, 1M gas) | 65.0 | 334.3 | **RISC0 is 5.1x faster** | 633.95 | 476.24 | SP1 uses 1.3x less |
| MSTORE (Prague, 1M gas, big memory expansion: True, offset initialized: False, 31 offset) | 57.5 | 295.7 | **RISC0 is 5.1x faster** | 648.95 | 476.71 | SP1 uses 1.4x less |
| SWAP1 (Prague, 1M gas) | 65.9 | 338.7 | **RISC0 is 5.1x faster** | 593.16 | 390.60 | SP1 uses 1.5x less |
| SWAP14 (Prague, 1M gas) | 65.0 | 333.6 | **RISC0 is 5.1x faster** | 708.48 | 522.18 | SP1 uses 1.4x less |
| XOR (Prague, 1M gas, ) | 33.8 | 173.5 | **RISC0 is 5.1x faster** | 598.32 | 449.05 | SP1 uses 1.3x less |
| TSTORE (Prague, 1M gas, dense val mut: False, key mut: False) | 28.9 | 148.5 | **RISC0 is 5.1x faster** | 632.07 | 352.16 | SP1 uses 1.8x less |
| CALLDATACOPY (Prague, 1M gas, non zero data: True, fixed src dst: False, 100 bytes, transaction) | 45.8 | 234.6 | **RISC0 is 5.1x faster** | 713.16 | 449.05 | SP1 uses 1.6x less |
| MSTORE (Prague, 1M gas, big memory expansion: True, offset initialized: False, 1 offset) | 57.7 | 295.5 | **RISC0 is 5.1x faster** | 632.07 | 473.90 | SP1 uses 1.3x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 3 mod vul nagydani) | 2,699.4 | 13,815.8 | **RISC0 is 5.1x faster** | 713.16 | 524.99 | SP1 uses 1.4x less |
| SWAP3 (Prague, 1M gas) | 66.2 | 338.6 | **RISC0 is 5.1x faster** | 643.32 | 351.69 | SP1 uses 1.8x less |
| DUP14 (Prague, 1M gas) | 32.8 | 167.8 | **RISC0 is 5.1x faster** | 670.04 | 524.99 | SP1 uses 1.3x less |
| RETURNDATACOPY (Prague, 1M gas, fixed dst: True, 0 bytes) | 39.8 | 203.3 | **RISC0 is 5.1x faster** | 632.54 | 376.07 | SP1 uses 1.7x less |
| CODECOPY (Prague, 1M gas, fixed src dst: True, max code size) | 22.3 | 113.7 | **RISC0 is 5.1x faster** | 712.23 | 525.93 | SP1 uses 1.4x less |
| MSTORE8 (Prague, 1M gas, big memory expansion: False, offset initialized: True, 1 offset) | 39.8 | 203.1 | **RISC0 is 5.1x faster** | 672.38 | 522.65 | SP1 uses 1.3x less |
| EXTCODECOPY (Prague, 1M gas, 5KiB) | 23.0 | 117.3 | **RISC0 is 5.1x faster** | 373.32 | 476.24 | **RISC0 uses 1.3x less** |
| CALLDATACOPY (Prague, 1M gas, non zero data: False, fixed src dst: False, 0 bytes, call) | 52.0 | 265.2 | **RISC0 is 5.1x faster** | 593.16 | 371.85 | SP1 uses 1.6x less |
| LT (Prague, 1M gas, ) | 35.8 | 182.4 | **RISC0 is 5.1x faster** | 628.79 | 522.65 | SP1 uses 1.2x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 128 mod odd) | 628.4 | 3,201.3 | **RISC0 is 5.1x faster** | 291.29 | 450.93 | **RISC0 uses 1.5x less** |
| CODECOPY (Prague, 1M gas, fixed src dst: True, 0.75x max code size) | 22.7 | 115.5 | **RISC0 is 5.1x faster** | 645.66 | 293.57 | SP1 uses 2.2x less |
| CALLDATACOPY (Prague, 1M gas, non zero data: False, fixed src dst: False, 10KiB, transaction) | 19.5 | 99.4 | **RISC0 is 5.1x faster** | 613.32 | 341.38 | SP1 uses 1.8x less |
| JUMPI (Prague, 1M gas) | 28.8 | 146.6 | **RISC0 is 5.1x faster** | 645.66 | 450.93 | SP1 uses 1.4x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 800 mod) | 1,058.1 | 5,381.1 | **RISC0 is 5.1x faster** | 640.04 | 450.93 | SP1 uses 1.4x less |
| MSTORE (Prague, 1M gas, big memory expansion: True, offset initialized: True, 0 offset) | 51.9 | 263.5 | **RISC0 is 5.1x faster** | 651.29 | 450.93 | SP1 uses 1.4x less |
| OR (Prague, 1M gas, ) | 33.8 | 171.5 | **RISC0 is 5.1x faster** | 713.16 | 450.93 | SP1 uses 1.6x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 64 mod even) | 670.8 | 3,405.9 | **RISC0 is 5.1x faster** | 698.63 | 476.24 | SP1 uses 1.5x less |
| CALLVALUE (Prague, 1M gas, from origin: True, non zero value: True) | 32.7 | 166.0 | **RISC0 is 5.1x faster** | 698.63 | 472.96 | SP1 uses 1.5x less |
| BASEFEE (Prague, 1M gas) | 39.2 | 199.0 | **RISC0 is 5.1x faster** | 712.23 | 473.90 | SP1 uses 1.5x less |
| SWAP8 (Prague, 1M gas) | 66.3 | 336.7 | **RISC0 is 5.1x faster** | 552.85 | 522.65 | SP1 uses 1.1x less |
| MSTORE (Prague, 1M gas, big memory expansion: False, offset initialized: False, 31 offset) | 58.1 | 294.9 | **RISC0 is 5.1x faster** | 698.63 | 383.10 | SP1 uses 1.8x less |
| MSTORE (Prague, 1M gas, big memory expansion: False, offset initialized: True, 1 offset) | 59.3 | 300.8 | **RISC0 is 5.1x faster** | 698.63 | 526.40 | SP1 uses 1.3x less |
| SWAP16 (Prague, 1M gas) | 66.1 | 334.9 | **RISC0 is 5.1x faster** | 642.85 | 475.30 | SP1 uses 1.4x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 2 mod vul pawel) | 923.0 | 4,677.4 | **RISC0 is 5.1x faster** | 610.04 | 522.18 | SP1 uses 1.2x less |
| CALLVALUE (Prague, 1M gas, from origin: True, non zero value: False) | 33.1 | 167.7 | **RISC0 is 5.1x faster** | 530.82 | 450.93 | SP1 uses 1.2x less |
| MSTORE (Prague, 1M gas, big memory expansion: False, offset initialized: False, 0 offset) | 52.5 | 266.0 | **RISC0 is 5.1x faster** | 698.63 | 449.05 | SP1 uses 1.6x less |
| ADDMOD, MULMOD (Prague, 1M gas, op ADDMOD, 191 mod bits) | 259.8 | 1,315.6 | **RISC0 is 5.1x faster** | 643.32 | 242.94 | SP1 uses 2.6x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 2 mod vul nagydani) | 1,233.2 | 6,243.6 | **RISC0 is 5.1x faster** | 627.38 | 477.18 | SP1 uses 1.3x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 4 mod vul pawel) | 662.6 | 3,353.0 | **RISC0 is 5.1x faster** | 668.63 | 473.90 | SP1 uses 1.4x less |
| CREATE2 (Prague, 1M gas) | 14.9 | 75.6 | **RISC0 is 5.1x faster** | 668.63 | 340.91 | SP1 uses 2.0x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 1360 mod vul common) | 607.1 | 3,069.7 | **RISC0 is 5.1x faster** | 638.16 | 450.46 | SP1 uses 1.4x less |
| SLOAD, SSTORE (Prague, 1M gas, absent slots: True) | 15.1 | 76.1 | **RISC0 is 5.1x faster** | 645.66 | 477.65 | SP1 uses 1.4x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 677 mod) | 725.1 | 3,663.0 | **RISC0 is 5.1x faster** | 639.57 | 449.05 | SP1 uses 1.4x less |
| CALLVALUE (Prague, 1M gas, from origin: False, non zero value: False) | 32.9 | 166.4 | **RISC0 is 5.0x faster** | 698.63 | 522.18 | SP1 uses 1.3x less |
| JUMPDEST, JUMP (Prague, 1M gas, 615b5b) | 23.6 | 118.9 | **RISC0 is 5.0x faster** | 678.01 | 450.93 | SP1 uses 1.5x less |
| MLOAD (Prague, 1M gas, big memory expansion: False, offset initialized: False, 0 offset) | 46.5 | 234.7 | **RISC0 is 5.0x faster** | 593.63 | 419.52 | SP1 uses 1.4x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 400 mod) | 959.7 | 4,839.7 | **RISC0 is 5.0x faster** | 713.16 | 435.93 | SP1 uses 1.6x less |
| MSTORE (Prague, 1M gas, big memory expansion: False, offset initialized: True, 31 offset) | 57.6 | 290.7 | **RISC0 is 5.0x faster** | 678.01 | 449.05 | SP1 uses 1.5x less |
| ADDMOD, MULMOD (Prague, 1M gas, op MULMOD, 191 mod bits) | 359.1 | 1,810.7 | **RISC0 is 5.0x faster** | 712.70 | 476.71 | SP1 uses 1.5x less |
| LOG3 (Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB non zero data) | 15.0 | 75.7 | **RISC0 is 5.0x faster** | 573.95 | 449.05 | SP1 uses 1.3x less |
| CALLVALUE (Prague, 1M gas, from origin: False, non zero value: True) | 32.5 | 163.6 | **RISC0 is 5.0x faster** | 672.38 | 450.46 | SP1 uses 1.5x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 600 mod) | 676.4 | 3,406.4 | **RISC0 is 5.0x faster** | 633.01 | 449.05 | SP1 uses 1.4x less |
| CODECOPY (Prague, 1M gas, fixed src dst: False, 0.25x max code size) | 31.8 | 159.9 | **RISC0 is 5.0x faster** | 641.45 | 340.44 | SP1 uses 1.9x less |
| MSIZE (Prague, 1M gas, 1 mem size) | 44.4 | 223.4 | **RISC0 is 5.0x faster** | 668.63 | 514.68 | SP1 uses 1.3x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 32 mod even) | 633.9 | 3,190.3 | **RISC0 is 5.0x faster** | 628.79 | 435.93 | SP1 uses 1.4x less |
| ADDMOD, MULMOD (Prague, 1M gas, op ADDMOD, 63 mod bits) | 143.6 | 722.5 | **RISC0 is 5.0x faster** | 643.32 | 476.71 | SP1 uses 1.3x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 4 mod vul nagydani) | 3,185.8 | 16,025.5 | **RISC0 is 5.0x faster** | 668.63 | 475.30 | SP1 uses 1.4x less |
| CALLDATACOPY (Prague, 1M gas, non zero data: False, fixed src dst: True, 1MiB, call) | 14.1 | 71.1 | **RISC0 is 5.0x faster** | 690.66 | 449.99 | SP1 uses 1.5x less |
| MSTORE8 (Prague, 1M gas, big memory expansion: False, offset initialized: False, 31 offset) | 39.3 | 197.4 | **RISC0 is 5.0x faster** | 698.63 | 473.90 | SP1 uses 1.5x less |
| PUSH20 (Prague, 1M gas) | 47.2 | 237.2 | **RISC0 is 5.0x faster** | 633.95 | 435.93 | SP1 uses 1.5x less |
| MSTORE (Prague, 1M gas, big memory expansion: False, offset initialized: False, 1 offset) | 58.8 | 295.2 | **RISC0 is 5.0x faster** | 639.57 | 448.59 | SP1 uses 1.4x less |
| CODECOPY (Prague, 1M gas, fixed src dst: True, 0 bytes) | 34.0 | 170.9 | **RISC0 is 5.0x faster** | 700.04 | 449.05 | SP1 uses 1.6x less |
| CALLDATACOPY (Prague, 1M gas, non zero data: True, fixed src dst: True, 100 bytes, call) | 33.1 | 166.2 | **RISC0 is 5.0x faster** | 641.91 | 450.46 | SP1 uses 1.4x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 1024 mod odd) | 14.7 | 73.6 | **RISC0 is 5.0x faster** | 713.16 | 515.15 | SP1 uses 1.4x less |
| MSTORE (Prague, 1M gas, big memory expansion: False, offset initialized: True, 0 offset) | 53.0 | 265.9 | **RISC0 is 5.0x faster** | 165.20 | 526.40 | **RISC0 uses 3.2x less** |
| MSTORE8 (Prague, 1M gas, big memory expansion: False, offset initialized: True, 31 offset) | 38.9 | 195.0 | **RISC0 is 5.0x faster** | 698.63 | 474.84 | SP1 uses 1.5x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 852 mod) | 1,058.8 | 5,310.8 | **RISC0 is 5.0x faster** | 670.04 | 340.44 | SP1 uses 2.0x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 3 mod vul nagydani) | 538.4 | 2,698.4 | **RISC0 is 5.0x faster** | 643.32 | 473.43 | SP1 uses 1.4x less |
| MOD, SMOD (Prague, 1M gas, op MOD, 191 mod bits) | 224.6 | 1,125.3 | **RISC0 is 5.0x faster** | 633.01 | 525.93 | SP1 uses 1.2x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 208 mod exp) | 595.1 | 2,982.2 | **RISC0 is 5.0x faster** | 670.04 | 450.93 | SP1 uses 1.5x less |
| MCOPY (Prague, 1M gas, fixed src dst: False, 10KiB) | 39.4 | 197.5 | **RISC0 is 5.0x faster** | 645.66 | 479.99 | SP1 uses 1.3x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 767 mod) | 650.7 | 3,259.7 | **RISC0 is 5.0x faster** | 711.76 | 392.94 | SP1 uses 1.8x less |
| CALLDATACOPY (Prague, 1M gas, non zero data: False, fixed src dst: True, 10KiB, call) | 23.0 | 115.0 | **RISC0 is 5.0x faster** | 713.16 | 474.84 | SP1 uses 1.5x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, mod min as exp heavy) | 1,540.1 | 7,712.4 | **RISC0 is 5.0x faster** | 713.16 | 522.65 | SP1 uses 1.4x less |
| CREATE (Prague, 1M gas) | 15.1 | 75.7 | **RISC0 is 5.0x faster** | 362.07 | 164.19 | SP1 uses 2.2x less |
| PUSH13 (Prague, 1M gas) | 42.0 | 210.3 | **RISC0 is 5.0x faster** | 713.16 | 376.07 | SP1 uses 1.9x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 200 mod vul common) | 438.2 | 2,193.2 | **RISC0 is 5.0x faster** | 643.32 | 340.44 | SP1 uses 1.9x less |
| MSTORE8 (Prague, 1M gas, big memory expansion: True, offset initialized: True, 1 offset) | 39.3 | 196.6 | **RISC0 is 5.0x faster** | 713.16 | 449.05 | SP1 uses 1.6x less |
| CALL (Prague, 1M gas, absent target: True) | 76.1 | 380.7 | **RISC0 is 5.0x faster** | 713.16 | 526.40 | SP1 uses 1.4x less |
| SHL (Prague, 1M gas, ) | 59.2 | 296.0 | **RISC0 is 5.0x faster** | 575.82 | 522.65 | SP1 uses 1.1x less |
| MSIZE (Prague, 1M gas, 1000 mem size) | 45.0 | 224.9 | **RISC0 is 5.0x faster** | 670.04 | 477.65 | SP1 uses 1.4x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 1360 mod) | 770.4 | 3,851.3 | **RISC0 is 5.0x faster** | 698.63 | 437.34 | SP1 uses 1.6x less |
| RETURN (Prague, 1M gas, 1MiB of zero data) | 15.0 | 74.8 | **RISC0 is 5.0x faster** | 365.35 | 272.00 | SP1 uses 1.3x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 512 mod odd) | 14.9 | 74.6 | **RISC0 is 5.0x faster** | 670.04 | 449.05 | SP1 uses 1.5x less |
| MLOAD (Prague, 1M gas, big memory expansion: True, offset initialized: True, 0 offset) | 46.3 | 231.1 | **RISC0 is 5.0x faster** | 698.63 | 522.18 | SP1 uses 1.3x less |
| SIGNEXTEND (Prague, 1M gas, ) | 54.6 | 272.7 | **RISC0 is 5.0x faster** | 638.16 | 449.05 | SP1 uses 1.4x less |
| RETURNDATACOPY (Prague, 1M gas, fixed dst: True, 100 bytes) | 33.2 | 165.8 | **RISC0 is 5.0x faster** | 670.04 | 477.18 | SP1 uses 1.4x less |
| CALLDATASIZE (Prague, 1M gas, 0 calldata length) | 34.9 | 174.1 | **RISC0 is 5.0x faster** | 713.16 | 477.18 | SP1 uses 1.5x less |
| MLOAD (Prague, 1M gas, big memory expansion: True, offset initialized: True, 1 offset) | 46.2 | 230.5 | **RISC0 is 5.0x faster** | 678.01 | 449.05 | SP1 uses 1.5x less |
| MSTORE (Prague, 1M gas, big memory expansion: True, offset initialized: True, 1 offset) | 58.1 | 290.1 | **RISC0 is 5.0x faster** | 646.13 | 449.05 | SP1 uses 1.4x less |
| MOD, SMOD (Prague, 1M gas, op MOD, 63 mod bits) | 119.0 | 593.3 | **RISC0 is 5.0x faster** | 620.82 | 524.99 | SP1 uses 1.2x less |
| GT (Prague, 1M gas, ) | 36.1 | 179.9 | **RISC0 is 5.0x faster** | 223.32 | 450.93 | **RISC0 uses 2.0x less** |
| CALLDATACOPY (Prague, 1M gas, non zero data: False, fixed src dst: True, 0 bytes, transaction) | 36.4 | 181.7 | **RISC0 is 5.0x faster** | 345.20 | 477.65 | **RISC0 uses 1.4x less** |
| DIV (Prague, 1M gas, 1) | 178.5 | 890.3 | **RISC0 is 5.0x faster** | 713.16 | 449.05 | SP1 uses 1.6x less |
| CALL (Prague, 1M gas, absent target: False) | 79.9 | 398.2 | **RISC0 is 5.0x faster** | 638.16 | 449.99 | SP1 uses 1.4x less |
| MSTORE8 (Prague, 1M gas, big memory expansion: True, offset initialized: True, 31 offset) | 39.4 | 196.2 | **RISC0 is 5.0x faster** | 713.16 | 435.93 | SP1 uses 1.6x less |
| MLOAD (Prague, 1M gas, big memory expansion: True, offset initialized: True, 31 offset) | 47.2 | 235.3 | **RISC0 is 5.0x faster** | 616.60 | 525.93 | SP1 uses 1.2x less |
| PRECOMPILE_IDENTITY, CALL (Prague, 1M gas) | 31.0 | 154.6 | **RISC0 is 5.0x faster** | 643.32 | 341.38 | SP1 uses 1.9x less |
| BYTE (Prague, 1M gas, ) | 35.5 | 176.7 | **RISC0 is 5.0x faster** | 662.54 | 335.75 | SP1 uses 2.0x less |
| ADDMOD, MULMOD (Prague, 1M gas, op MULMOD, 63 mod bits) | 190.6 | 949.3 | **RISC0 is 5.0x faster** | 690.66 | 435.93 | SP1 uses 1.6x less |
| PRECOMPILE_IDENTITY (Prague, 1M gas, 0 returned size) | 31.9 | 158.9 | **RISC0 is 5.0x faster** | 668.63 | 335.75 | SP1 uses 2.0x less |
| CALLDATACOPY (Prague, 1M gas, non zero data: False, fixed src dst: False, 0 bytes, transaction) | 52.3 | 260.5 | **RISC0 is 5.0x faster** | 632.54 | 522.18 | SP1 uses 1.2x less |
| MCOPY (Prague, 1M gas, fixed src dst: True, 0 bytes) | 33.5 | 166.9 | **RISC0 is 5.0x faster** | 345.20 | 450.46 | **RISC0 uses 1.3x less** |
| CALL, STATICCALL (Prague, 1M gas) | 52.8 | 262.9 | **RISC0 is 5.0x faster** | 641.45 | 449.05 | SP1 uses 1.4x less |
| CODECOPY (Prague, 1M gas, fixed src dst: True, 0.25x max code size) | 23.1 | 114.9 | **RISC0 is 5.0x faster** | 613.32 | 340.44 | SP1 uses 1.8x less |
| MLOAD (Prague, 1M gas, big memory expansion: False, offset initialized: False, 1 offset) | 47.3 | 235.3 | **RISC0 is 5.0x faster** | 644.26 | 522.18 | SP1 uses 1.2x less |
| CALLDATALOAD (Prague, 1M gas, empty) | 67.2 | 334.0 | **RISC0 is 5.0x faster** | 669.10 | 522.18 | SP1 uses 1.3x less |
| MLOAD (Prague, 1M gas, big memory expansion: False, offset initialized: True, 0 offset) | 47.3 | 234.9 | **RISC0 is 5.0x faster** | 678.01 | 479.52 | SP1 uses 1.4x less |
| SELFDESTRUCT (Prague, 1M gas, value bearing: False) | 15.0 | 74.5 | **RISC0 is 5.0x faster** | 420.66 | 473.90 | **RISC0 uses 1.1x less** |
| TLOAD (Prague, 1M gas, val mut: True, key mut: True) | 19.4 | 96.3 | **RISC0 is 5.0x faster** | 628.79 | 406.87 | SP1 uses 1.5x less |
| LOG0 (Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB non zero data) | 14.6 | 72.7 | **RISC0 is 5.0x faster** | 711.76 | 474.84 | SP1 uses 1.5x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 32 mod odd) | 716.3 | 3,557.2 | **RISC0 is 5.0x faster** | 570.66 | 473.90 | SP1 uses 1.2x less |
| RETURNDATASIZE (Prague, 1M gas, 1 returned size) | 32.1 | 159.2 | **RISC0 is 5.0x faster** | 644.26 | 479.99 | SP1 uses 1.3x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 3 mod pawel) | 789.0 | 3,917.8 | **RISC0 is 5.0x faster** | 668.63 | 449.05 | SP1 uses 1.5x less |
| MUL (Prague, 1M gas, ) | 58.8 | 291.7 | **RISC0 is 5.0x faster** | 700.04 | 477.65 | SP1 uses 1.5x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 2 mod vul nagydani) | 1,159.3 | 5,753.0 | **RISC0 is 5.0x faster** | 668.63 | 524.99 | SP1 uses 1.3x less |
| CALLCODE (Prague, 1M gas, absent target: False) | 81.8 | 406.1 | **RISC0 is 5.0x faster** | 612.38 | 449.05 | SP1 uses 1.4x less |
| PRECOMPILE_IDENTITY (Prague, 1M gas, 1 returned size) | 31.9 | 158.1 | **RISC0 is 5.0x faster** | 713.16 | 449.05 | SP1 uses 1.6x less |
| MOD, SMOD (Prague, 1M gas, op MOD, 127 mod bits) | 175.8 | 872.2 | **RISC0 is 5.0x faster** | 645.20 | 477.18 | SP1 uses 1.4x less |
| PRECOMPILE_RIPEMD-160, CALL (Prague, 1M gas, 160) | 24.2 | 120.1 | **RISC0 is 5.0x faster** | 594.57 | 477.18 | SP1 uses 1.2x less |
| CALLDATACOPY (Prague, 1M gas, non zero data: True, fixed src dst: False, 10KiB, transaction) | 25.5 | 126.2 | **RISC0 is 5.0x faster** | 613.32 | 522.65 | SP1 uses 1.2x less |
| PUSH7 (Prague, 1M gas) | 39.1 | 194.0 | **RISC0 is 5.0x faster** | 670.04 | 435.46 | SP1 uses 1.5x less |
| SLT (Prague, 1M gas, ) | 40.6 | 201.4 | **RISC0 is 5.0x faster** | 713.16 | 476.24 | SP1 uses 1.5x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 1 mod vul nagydani) | 407.3 | 2,018.4 | **RISC0 is 5.0x faster** | 638.16 | 189.97 | SP1 uses 3.4x less |
| DIV (Prague, 1M gas, 0) | 192.7 | 954.9 | **RISC0 is 5.0x faster** | 670.04 | 451.40 | SP1 uses 1.5x less |
| MSIZE (Prague, 1M gas, 1000000 mem size) | 44.3 | 219.5 | **RISC0 is 5.0x faster** | 713.16 | 449.05 | SP1 uses 1.6x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 1024 mod even) | 15.0 | 74.4 | **RISC0 is 5.0x faster** | 645.66 | 522.18 | SP1 uses 1.2x less |
| LOG3 (Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB zeros data) | 15.0 | 74.5 | **RISC0 is 5.0x faster** | 670.04 | 474.84 | SP1 uses 1.4x less |
| CREATE2 (Prague, 1M gas) | 15.3 | 75.5 | **RISC0 is 5.0x faster** | 644.73 | 474.84 | SP1 uses 1.4x less |
| CALLDATACOPY (Prague, 1M gas, non zero data: False, fixed src dst: False, 10KiB, call) | 27.3 | 135.3 | **RISC0 is 4.9x faster** | 171.76 | 450.93 | **RISC0 uses 2.6x less** |
| ADDMOD, MULMOD (Prague, 1M gas, op MULMOD, 255 mod bits) | 345.8 | 1,711.6 | **RISC0 is 4.9x faster** | 643.32 | 265.91 | SP1 uses 2.4x less |
| MCOPY (Prague, 1M gas, fixed src dst: True, 10KiB) | 29.4 | 145.4 | **RISC0 is 4.9x faster** | 250.98 | 310.44 | **RISC0 uses 1.2x less** |
| MOD, SMOD (Prague, 1M gas, op SMOD, 255 mod bits) | 175.1 | 866.6 | **RISC0 is 4.9x faster** | 632.07 | 216.69 | SP1 uses 2.9x less |
| JUMP (Prague, 1M gas) | 24.7 | 122.2 | **RISC0 is 4.9x faster** | 662.54 | 450.93 | SP1 uses 1.5x less |
| EXTCODESIZE (Prague, 1M gas, absent target: False) | 25.8 | 127.6 | **RISC0 is 4.9x faster** | 668.63 | 515.62 | SP1 uses 1.3x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 3 mod vul nagydani) | 3,038.9 | 15,030.5 | **RISC0 is 4.9x faster** | 713.16 | 525.93 | SP1 uses 1.4x less |
| EXP (Prague, 1M gas, ) | 112.8 | 557.8 | **RISC0 is 4.9x faster** | 678.01 | 474.84 | SP1 uses 1.4x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 200 mod vul common) | 346.3 | 1,712.2 | **RISC0 is 4.9x faster** | 611.91 | 526.40 | SP1 uses 1.2x less |
| SHL, SHR, SAR (Prague, 1M gas, shift right SAR) | 63.8 | 315.4 | **RISC0 is 4.9x faster** | 646.13 | 522.18 | SP1 uses 1.2x less |
| PUSH1 (Prague, 1M gas) | 31.5 | 155.6 | **RISC0 is 4.9x faster** | 678.01 | 449.05 | SP1 uses 1.5x less |
| MLOAD (Prague, 1M gas, big memory expansion: True, offset initialized: False, 0 offset) | 46.8 | 231.3 | **RISC0 is 4.9x faster** | 672.38 | 376.07 | SP1 uses 1.8x less |
| LOG1 (Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB non zero data) | 14.8 | 72.9 | **RISC0 is 4.9x faster** | 361.60 | 435.93 | **RISC0 uses 1.2x less** |
| CALLDATALOAD (Prague, 1M gas, one, loop) | 78.7 | 389.1 | **RISC0 is 4.9x faster** | 713.16 | 474.37 | SP1 uses 1.5x less |
| CALLDATACOPY (Prague, 1M gas, non zero data: True, fixed src dst: False, 100 bytes, call) | 46.8 | 231.1 | **RISC0 is 4.9x faster** | 698.63 | 283.72 | SP1 uses 2.5x less |
| MSTORE8 (Prague, 1M gas, big memory expansion: False, offset initialized: True, 0 offset) | 40.3 | 199.1 | **RISC0 is 4.9x faster** | 698.63 | 473.43 | SP1 uses 1.5x less |
| DUP7 (Prague, 1M gas) | 32.8 | 162.2 | **RISC0 is 4.9x faster** | 669.57 | 525.93 | SP1 uses 1.3x less |
| LOG2 (Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB non zero data) | 14.9 | 73.7 | **RISC0 is 4.9x faster** | 670.04 | 479.99 | SP1 uses 1.4x less |
| MSTORE8 (Prague, 1M gas, big memory expansion: True, offset initialized: True, 0 offset) | 40.2 | 198.8 | **RISC0 is 4.9x faster** | 641.91 | 372.32 | SP1 uses 1.7x less |
| LOG0 (Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB non zero data) | 15.0 | 74.0 | **RISC0 is 4.9x faster** | 678.01 | 251.85 | SP1 uses 2.7x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 2 mod pawel) | 985.1 | 4,863.4 | **RISC0 is 4.9x faster** | 668.63 | 449.05 | SP1 uses 1.5x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 2 mod vul example) | 736.0 | 3,633.4 | **RISC0 is 4.9x faster** | 643.32 | 473.90 | SP1 uses 1.4x less |
| SDIV (Prague, 1M gas, 0) | 214.6 | 1,059.5 | **RISC0 is 4.9x faster** | 613.32 | 449.99 | SP1 uses 1.4x less |
| CREATE2 (Prague, 1M gas) | 15.1 | 74.5 | **RISC0 is 4.9x faster** | 712.23 | 473.43 | SP1 uses 1.5x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 1 mod vul pawel) | 1,691.4 | 8,346.5 | **RISC0 is 4.9x faster** | 668.63 | 522.65 | SP1 uses 1.3x less |
| MLOAD (Prague, 1M gas, big memory expansion: False, offset initialized: True, 31 offset) | 47.0 | 231.9 | **RISC0 is 4.9x faster** | 641.91 | 407.34 | SP1 uses 1.6x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 2 mod vul nagydani) | 563.0 | 2,777.3 | **RISC0 is 4.9x faster** | 613.32 | 373.25 | SP1 uses 1.6x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 1152 mod vul common) | 431.0 | 2,125.8 | **RISC0 is 4.9x faster** | 643.32 | 450.93 | SP1 uses 1.4x less |
| CODECOPY (Prague, 1M gas, fixed src dst: False, max code size) | 28.2 | 138.9 | **RISC0 is 4.9x faster** | 698.63 | 515.62 | SP1 uses 1.4x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 408 mod) | 676.7 | 3,337.1 | **RISC0 is 4.9x faster** | 612.38 | 522.18 | SP1 uses 1.2x less |
| CALLCODE (Prague, 1M gas, absent target: True) | 77.6 | 382.6 | **RISC0 is 4.9x faster** | 646.13 | 449.99 | SP1 uses 1.4x less |
| LOG2 (Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB zeros data) | 15.2 | 75.0 | **RISC0 is 4.9x faster** | 361.60 | 437.34 | **RISC0 uses 1.2x less** |
| MSIZE (Prague, 1M gas, 100000 mem size) | 44.3 | 218.2 | **RISC0 is 4.9x faster** | 698.63 | 375.60 | SP1 uses 1.9x less |
| MLOAD (Prague, 1M gas, big memory expansion: False, offset initialized: True, 1 offset) | 46.7 | 230.1 | **RISC0 is 4.9x faster** | 361.60 | 522.65 | **RISC0 uses 1.4x less** |
| MLOAD (Prague, 1M gas, big memory expansion: False, offset initialized: False, 31 offset) | 47.1 | 231.9 | **RISC0 is 4.9x faster** | 678.01 | 522.65 | SP1 uses 1.3x less |
| SAR (Prague, 1M gas, ) | 67.3 | 331.0 | **RISC0 is 4.9x faster** | 643.32 | 522.18 | SP1 uses 1.2x less |
| LOG3 (Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB zeros data) | 14.5 | 71.4 | **RISC0 is 4.9x faster** | 668.63 | 373.25 | SP1 uses 1.8x less |
| RETURNDATACOPY (Prague, 1M gas, fixed dst: False, 100 bytes) | 41.9 | 206.3 | **RISC0 is 4.9x faster** | 670.04 | 351.69 | SP1 uses 1.9x less |
| LOG1 (Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB zeros data) | 14.7 | 72.3 | **RISC0 is 4.9x faster** | 180.66 | 351.69 | **RISC0 uses 1.9x less** |
| LOG0 (Prague, 1M gas, fixed offset: False, non zero topic, 0 bytes data) | 17.0 | 83.4 | **RISC0 is 4.9x faster** | 668.63 | 450.46 | SP1 uses 1.5x less |
| TSTORE (Prague, 1M gas, dense val mut: False, key mut: True) | 29.0 | 142.7 | **RISC0 is 4.9x faster** | 640.98 | 522.65 | SP1 uses 1.2x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 1 mod vul nagydani) | 557.7 | 2,740.8 | **RISC0 is 4.9x faster** | 668.63 | 375.60 | SP1 uses 1.8x less |
| DUP6 (Prague, 1M gas) | 33.0 | 162.4 | **RISC0 is 4.9x faster** | 672.38 | 377.00 | SP1 uses 1.8x less |
| ADDMOD, MULMOD (Prague, 1M gas, op MULMOD, 127 mod bits) | 247.7 | 1,217.1 | **RISC0 is 4.9x faster** | 490.04 | 477.65 | SP1 uses 1.0x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 1 mod vul marius) | 855.9 | 4,204.7 | **RISC0 is 4.9x faster** | 646.13 | 474.84 | SP1 uses 1.4x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 32 mod odd) | 786.3 | 3,862.2 | **RISC0 is 4.9x faster** | 616.60 | 525.93 | SP1 uses 1.2x less |
| CALLER (Prague, 1M gas) | 62.8 | 308.4 | **RISC0 is 4.9x faster** | 711.76 | 525.93 | SP1 uses 1.4x less |
| CHAINID (Prague, 1M gas) | 39.9 | 196.1 | **RISC0 is 4.9x faster** | 713.16 | 525.93 | SP1 uses 1.4x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, mod min as balanced) | 645.1 | 3,166.2 | **RISC0 is 4.9x faster** | 361.60 | 373.25 | **RISC0 uses 1.0x less** |
| CALLDATASIZE (Prague, 1M gas, 10000 calldata length) | 34.2 | 167.8 | **RISC0 is 4.9x faster** | 698.63 | 341.85 | SP1 uses 2.0x less |
| LOG4 (Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB non zero data) | 14.9 | 73.0 | **RISC0 is 4.9x faster** | 677.54 | 477.65 | SP1 uses 1.4x less |
| ADDMOD, MULMOD (Prague, 1M gas, op ADDMOD, 255 mod bits) | 202.3 | 991.8 | **RISC0 is 4.9x faster** | 670.04 | 335.75 | SP1 uses 2.0x less |
| MSTORE8 (Prague, 1M gas, big memory expansion: True, offset initialized: False, 0 offset) | 40.0 | 196.0 | **RISC0 is 4.9x faster** | 713.16 | 412.02 | SP1 uses 1.7x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 32 mod odd) | 570.8 | 2,797.9 | **RISC0 is 4.9x faster** | 642.85 | 476.71 | SP1 uses 1.3x less |
| MOD, SMOD (Prague, 1M gas, op SMOD, 191 mod bits) | 233.9 | 1,145.9 | **RISC0 is 4.9x faster** | 619.88 | 522.65 | SP1 uses 1.2x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 200 mod vul common) | 445.5 | 2,182.2 | **RISC0 is 4.9x faster** | 713.16 | 476.71 | SP1 uses 1.5x less |
| STATICCALL (Prague, 1M gas, absent target: False) | 82.2 | 402.6 | **RISC0 is 4.9x faster** | 628.79 | 479.52 | SP1 uses 1.3x less |
| CODECOPY (Prague, 1M gas, fixed src dst: False, 0.50x max code size) | 27.9 | 136.7 | **RISC0 is 4.9x faster** | 698.63 | 376.07 | SP1 uses 1.9x less |
| MOD, SMOD (Prague, 1M gas, op MOD, 255 mod bits) | 170.2 | 832.9 | **RISC0 is 4.9x faster** | 641.45 | 474.84 | SP1 uses 1.4x less |
| CALLDATACOPY (Prague, 1M gas, non zero data: False, fixed src dst: False, 1MiB, transaction) | 14.7 | 72.0 | **RISC0 is 4.9x faster** | 670.04 | 449.05 | SP1 uses 1.5x less |
| ADDMOD, MULMOD (Prague, 1M gas, op ADDMOD, 127 mod bits) | 207.1 | 1,013.7 | **RISC0 is 4.9x faster** | 613.32 | 474.84 | SP1 uses 1.3x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 4 mod pawel) | 715.4 | 3,499.8 | **RISC0 is 4.9x faster** | 668.63 | 522.18 | SP1 uses 1.3x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 1349 mod vul common) | 623.8 | 3,050.5 | **RISC0 is 4.9x faster** | 412.70 | 522.65 | **RISC0 uses 1.3x less** |
| CODECOPY (Prague, 1M gas, fixed src dst: False, 0 bytes) | 50.6 | 247.4 | **RISC0 is 4.9x faster** | 672.38 | 450.93 | SP1 uses 1.5x less |
| SWAP11 (Prague, 1M gas) | 66.7 | 326.2 | **RISC0 is 4.9x faster** | 698.63 | 476.24 | SP1 uses 1.5x less |
| RETURN (Prague, 1M gas, empty) | 80.9 | 395.3 | **RISC0 is 4.9x faster** | 618.95 | 449.99 | SP1 uses 1.4x less |
| LOG0 (Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB zeros data) | 14.9 | 72.7 | **RISC0 is 4.9x faster** | 669.57 | 450.46 | SP1 uses 1.5x less |
| CALLDATALOAD (Prague, 1M gas, zero, loop) | 79.3 | 387.6 | **RISC0 is 4.9x faster** | 670.04 | 474.84 | SP1 uses 1.4x less |
| STATICCALL (Prague, 1M gas, absent target: True) | 78.1 | 381.4 | **RISC0 is 4.9x faster** | 690.66 | 449.05 | SP1 uses 1.5x less |
| LOG1 (Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB zeros data) | 15.1 | 73.8 | **RISC0 is 4.9x faster** | 594.57 | 373.25 | SP1 uses 1.6x less |
| PUSH5 (Prague, 1M gas) | 36.2 | 176.8 | **RISC0 is 4.9x faster** | 713.16 | 448.59 | SP1 uses 1.6x less |
| AND (Prague, 1M gas, ) | 34.4 | 168.2 | **RISC0 is 4.9x faster** | 669.57 | 340.91 | SP1 uses 2.0x less |
| RETURNDATASIZE (Prague, 1M gas, 0 returned size) | 32.4 | 158.1 | **RISC0 is 4.9x faster** | 638.63 | 450.93 | SP1 uses 1.4x less |
| CALLDATACOPY (Prague, 1M gas, non zero data: False, fixed src dst: False, 100 bytes, transaction) | 43.6 | 213.0 | **RISC0 is 4.9x faster** | 698.63 | 449.99 | SP1 uses 1.6x less |
| MSIZE (Prague, 1M gas, 0 mem size) | 45.1 | 220.2 | **RISC0 is 4.9x faster** | 613.32 | 392.94 | SP1 uses 1.6x less |
| TSTORE (Prague, 1M gas, dense val mut: True, key mut: False) | 45.6 | 222.6 | **RISC0 is 4.9x faster** | 638.16 | 525.93 | SP1 uses 1.2x less |
| SLOAD, SSTORE (Prague, 1M gas, absent slots: True, SSTORE same value) | 15.0 | 73.4 | **RISC0 is 4.9x faster** | 713.16 | 522.18 | SP1 uses 1.4x less |
| LOG0 (Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB non zero data) | 14.7 | 71.8 | **RISC0 is 4.9x faster** | 713.16 | 341.85 | SP1 uses 2.1x less |
| CALLDATACOPY (Prague, 1M gas, non zero data: False, fixed src dst: True, 100 bytes, call) | 31.3 | 152.9 | **RISC0 is 4.9x faster** | 257.54 | 449.05 | **RISC0 uses 1.7x less** |
| LOG4 (Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB zeros data) | 15.0 | 73.3 | **RISC0 is 4.9x faster** | 599.26 | 341.85 | SP1 uses 1.8x less |
| MLOAD (Prague, 1M gas, big memory expansion: True, offset initialized: False, 1 offset) | 47.2 | 229.9 | **RISC0 is 4.9x faster** | 718.32 | 473.90 | SP1 uses 1.5x less |
| PUSH0 (Prague, 1M gas) | 38.3 | 186.4 | **RISC0 is 4.9x faster** | 713.16 | 522.65 | SP1 uses 1.4x less |
| CODESIZE (Prague, 1M gas) | 49.6 | 241.5 | **RISC0 is 4.9x faster** | 459.57 | 525.93 | **RISC0 uses 1.1x less** |
| TLOAD (Prague, 1M gas, val mut: True, key mut: False) | 18.9 | 91.8 | **RISC0 is 4.9x faster** | 670.04 | 522.18 | SP1 uses 1.3x less |
| MSTORE (Prague, 1M gas, big memory expansion: True, offset initialized: True, 31 offset) | 58.3 | 284.0 | **RISC0 is 4.9x faster** | 672.38 | 392.47 | SP1 uses 1.7x less |
| RETURNDATASIZE (Prague, 1M gas, 1 returned size) | 32.5 | 158.1 | **RISC0 is 4.9x faster** | 678.01 | 477.65 | SP1 uses 1.4x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 1 mod vul nagydani) | 394.8 | 1,921.7 | **RISC0 is 4.9x faster** | 643.32 | 474.84 | SP1 uses 1.4x less |
| PUSH11 (Prague, 1M gas) | 41.9 | 203.7 | **RISC0 is 4.9x faster** | 597.38 | 376.07 | SP1 uses 1.6x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 600 mod) | 1,056.4 | 5,140.2 | **RISC0 is 4.9x faster** | 648.95 | 476.71 | SP1 uses 1.4x less |
| MSTORE8 (Prague, 1M gas, big memory expansion: False, offset initialized: False, 0 offset) | 40.7 | 197.8 | **RISC0 is 4.9x faster** | 698.63 | 473.90 | SP1 uses 1.5x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 298 mod exp) | 1,767.4 | 8,592.2 | **RISC0 is 4.9x faster** | 678.01 | 522.18 | SP1 uses 1.3x less |
| MOD, SMOD (Prague, 1M gas, op SMOD, 63 mod bits) | 128.4 | 624.1 | **RISC0 is 4.9x faster** | 628.79 | 526.40 | SP1 uses 1.2x less |
| DUP5 (Prague, 1M gas) | 33.2 | 161.3 | **RISC0 is 4.9x faster** | 614.26 | 526.40 | SP1 uses 1.2x less |
| NUMBER (Prague, 1M gas) | 41.7 | 202.4 | **RISC0 is 4.9x faster** | 670.04 | 222.32 | SP1 uses 3.0x less |
| LOG1 (Prague, 1M gas, fixed offset: False, non zero topic, 0 bytes data) | 16.7 | 81.4 | **RISC0 is 4.9x faster** | 628.32 | 449.05 | SP1 uses 1.4x less |
| LOG2 (Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB non zero data) | 15.0 | 73.0 | **RISC0 is 4.9x faster** | 632.54 | 474.84 | SP1 uses 1.3x less |
| SELFBALANCE (Prague, 1M gas) | 150.2 | 729.5 | **RISC0 is 4.9x faster** | 640.04 | 524.99 | SP1 uses 1.2x less |
| CREATE (Prague, 1M gas, zero data) | 15.3 | 74.2 | **RISC0 is 4.9x faster** | 574.41 | 449.99 | SP1 uses 1.3x less |
| CREATE2 (Prague, 1M gas) | 15.0 | 72.9 | **RISC0 is 4.9x faster** | 345.20 | 449.05 | **RISC0 uses 1.3x less** |
| EXTCODECOPY (Prague, 1M gas, 512) | 24.8 | 120.3 | **RISC0 is 4.9x faster** | 640.51 | 450.93 | SP1 uses 1.4x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 32 mod even) | 798.9 | 3,876.3 | **RISC0 is 4.9x faster** | 698.63 | 392.00 | SP1 uses 1.8x less |
| CALLDATACOPY (Prague, 1M gas, non zero data: False, fixed src dst: False, 100 bytes, call) | 46.8 | 226.9 | **RISC0 is 4.9x faster** | 643.32 | 392.47 | SP1 uses 1.6x less |
| BALANCE (Prague, 1M gas, absent target: True) | 23.2 | 112.6 | **RISC0 is 4.9x faster** | 641.91 | 522.18 | SP1 uses 1.2x less |
| LOG0 (Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB non zero data) | 15.1 | 73.1 | **RISC0 is 4.9x faster** | 698.63 | 513.27 | SP1 uses 1.4x less |
| DELEGATECALL (Prague, 1M gas, absent target: False) | 70.1 | 339.8 | **RISC0 is 4.8x faster** | 698.63 | 450.93 | SP1 uses 1.5x less |
| CODECOPY (Prague, 1M gas, fixed src dst: False, 0.75x max code size) | 28.1 | 136.1 | **RISC0 is 4.8x faster** | 672.38 | 474.84 | SP1 uses 1.4x less |
| DELEGATECALL (Prague, 1M gas, absent target: True) | 67.4 | 326.6 | **RISC0 is 4.8x faster** | 613.32 | 449.99 | SP1 uses 1.4x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, mod min as base heavy) | 2,288.5 | 11,093.4 | **RISC0 is 4.8x faster** | 672.38 | 477.18 | SP1 uses 1.4x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 16 mod even) | 1,040.4 | 5,043.0 | **RISC0 is 4.8x faster** | 666.29 | 477.18 | SP1 uses 1.4x less |
| LOG2 (Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB zeros data) | 15.1 | 73.0 | **RISC0 is 4.8x faster** | 666.76 | 474.84 | SP1 uses 1.4x less |
| MSTORE8 (Prague, 1M gas, big memory expansion: False, offset initialized: False, 1 offset) | 40.9 | 198.3 | **RISC0 is 4.8x faster** | 670.04 | 450.46 | SP1 uses 1.5x less |
| LOG0 (Prague, 1M gas, fixed offset: True, non zero topic, 0 bytes data) | 15.3 | 74.4 | **RISC0 is 4.8x faster** | 148.32 | 522.18 | **RISC0 uses 3.5x less** |
| RETURNDATASIZE (Prague, 1M gas, 0 returned size) | 32.5 | 157.7 | **RISC0 is 4.8x faster** | 698.63 | 407.80 | SP1 uses 1.7x less |
| MCOPY (Prague, 1M gas, fixed src dst: True, 100 bytes) | 34.5 | 167.3 | **RISC0 is 4.8x faster** | 709.88 | 476.71 | SP1 uses 1.5x less |
| SUB (Prague, 1M gas, ) | 40.9 | 198.1 | **RISC0 is 4.8x faster** | 718.32 | 450.46 | SP1 uses 1.6x less |
| BALANCE (Prague, 1M gas, absent accounts: True) | 17.8 | 86.3 | **RISC0 is 4.8x faster** | 713.16 | 526.40 | SP1 uses 1.4x less |
| LOG1 (Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB non zero data) | 15.0 | 72.5 | **RISC0 is 4.8x faster** | 666.76 | 376.07 | SP1 uses 1.8x less |
| TSTORE (Prague, 1M gas, dense val mut: True, key mut: True) | 45.2 | 218.9 | **RISC0 is 4.8x faster** | 672.38 | 522.18 | SP1 uses 1.3x less |
| RETURNDATACOPY (Prague, 1M gas, fixed dst: False, 10KiB) | 29.2 | 141.3 | **RISC0 is 4.8x faster** | 643.32 | 474.84 | SP1 uses 1.4x less |
| EXTCODEHASH (Prague, 1M gas, absent target: False) | 28.3 | 136.8 | **RISC0 is 4.8x faster** | 643.32 | 450.93 | SP1 uses 1.4x less |
| LOG4 (Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB non zero data) | 15.0 | 72.6 | **RISC0 is 4.8x faster** | 645.66 | 522.18 | SP1 uses 1.2x less |
| TIMESTAMP (Prague, 1M gas) | 42.2 | 204.4 | **RISC0 is 4.8x faster** | 613.32 | 450.46 | SP1 uses 1.4x less |
| PUSH16 (Prague, 1M gas) | 45.9 | 222.2 | **RISC0 is 4.8x faster** | 593.63 | 477.65 | SP1 uses 1.2x less |
| DUP10 (Prague, 1M gas) | 32.5 | 157.1 | **RISC0 is 4.8x faster** | 646.13 | 351.69 | SP1 uses 1.8x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 3 mod vul pawel) | 768.4 | 3,716.3 | **RISC0 is 4.8x faster** | 666.29 | 449.05 | SP1 uses 1.5x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 32 mod even) | 759.3 | 3,672.5 | **RISC0 is 4.8x faster** | 632.54 | 450.93 | SP1 uses 1.4x less |
| CREATE (Prague, 1M gas) | 15.1 | 73.2 | **RISC0 is 4.8x faster** | 670.04 | 450.46 | SP1 uses 1.5x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 24 mod even) | 845.6 | 4,086.7 | **RISC0 is 4.8x faster** | 592.70 | 392.47 | SP1 uses 1.5x less |
| JUMPDEST, JUMP (Prague, 1M gas, 5b) | 37.4 | 180.6 | **RISC0 is 4.8x faster** | 640.04 | 522.18 | SP1 uses 1.2x less |
| REVERT (Prague, 1M gas, 1KiB of non, zero data) | 52.3 | 252.9 | **RISC0 is 4.8x faster** | 672.38 | 473.90 | SP1 uses 1.4x less |
| RETURN (Prague, 1M gas, 1KiB of zero data) | 61.0 | 294.4 | **RISC0 is 4.8x faster** | 620.82 | 522.65 | SP1 uses 1.2x less |
| CALLDATACOPY (Prague, 1M gas, non zero data: True, fixed src dst: False, 10KiB, call) | 26.6 | 128.5 | **RISC0 is 4.8x faster** | 638.16 | 522.65 | SP1 uses 1.2x less |
| PUSH2 (Prague, 1M gas) | 33.1 | 159.6 | **RISC0 is 4.8x faster** | 670.04 | 376.07 | SP1 uses 1.8x less |
| CALLDATASIZE (Prague, 1M gas, 1000 calldata length) | 35.0 | 169.2 | **RISC0 is 4.8x faster** | 629.26 | 474.84 | SP1 uses 1.3x less |
| REVERT (Prague, 1M gas, 1KiB of zero data) | 63.8 | 308.0 | **RISC0 is 4.8x faster** | 678.01 | 351.69 | SP1 uses 1.9x less |
| MSTORE8 (Prague, 1M gas, big memory expansion: True, offset initialized: False, 1 offset) | 40.0 | 193.1 | **RISC0 is 4.8x faster** | 668.63 | 341.38 | SP1 uses 2.0x less |
| BLOBHASH (Prague, 1M gas) | 51.1 | 246.2 | **RISC0 is 4.8x faster** | 623.63 | 477.65 | SP1 uses 1.3x less |
| DUP11 (Prague, 1M gas) | 33.4 | 161.0 | **RISC0 is 4.8x faster** | 632.07 | 474.84 | SP1 uses 1.3x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 1360 mod vul common) | 693.9 | 3,339.7 | **RISC0 is 4.8x faster** | 640.04 | 351.69 | SP1 uses 1.8x less |
| PUSH14 (Prague, 1M gas) | 45.3 | 218.0 | **RISC0 is 4.8x faster** | 668.63 | 479.52 | SP1 uses 1.4x less |
| REVERT (Prague, 1M gas, 1MiB of zero data) | 15.0 | 72.1 | **RISC0 is 4.8x faster** | 645.66 | 392.94 | SP1 uses 1.6x less |
| SHL, SHR, SAR (Prague, 1M gas, shift right SHR) | 60.0 | 287.9 | **RISC0 is 4.8x faster** | 613.32 | 340.91 | SP1 uses 1.8x less |
| MCOPY (Prague, 1M gas, fixed src dst: False, 100 bytes) | 49.0 | 235.4 | **RISC0 is 4.8x faster** | 645.66 | 522.18 | SP1 uses 1.2x less |
| ORIGIN (Prague, 1M gas) | 64.4 | 309.2 | **RISC0 is 4.8x faster** | 670.04 | 476.71 | SP1 uses 1.4x less |
| LOG0 (Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB zeros data) | 15.1 | 72.5 | **RISC0 is 4.8x faster** | 376.13 | 522.18 | **RISC0 uses 1.4x less** |
| SELFDESTRUCT (Prague, 1M gas, value bearing: True) | 15.2 | 72.8 | **RISC0 is 4.8x faster** | 599.26 | 522.18 | SP1 uses 1.1x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 215 mod exp) | 1,640.5 | 7,873.2 | **RISC0 is 4.8x faster** | 638.16 | 477.18 | SP1 uses 1.3x less |
| LOG2 (Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB non zero data) | 14.9 | 71.7 | **RISC0 is 4.8x faster** | 670.04 | 373.25 | SP1 uses 1.8x less |
| LOG1 (Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB non zero data) | 15.1 | 72.2 | **RISC0 is 4.8x faster** | 645.66 | 474.84 | SP1 uses 1.4x less |
| PUSH21 (Prague, 1M gas) | 50.0 | 239.5 | **RISC0 is 4.8x faster** | 612.85 | 474.84 | SP1 uses 1.3x less |
| CALLDATACOPY (Prague, 1M gas, non zero data: False, fixed src dst: True, 1MiB, transaction) | 15.0 | 71.7 | **RISC0 is 4.8x faster** | 678.01 | 500.62 | SP1 uses 1.4x less |
| LOG4 (Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB non zero data) | 14.9 | 71.6 | **RISC0 is 4.8x faster** | 613.32 | 351.69 | SP1 uses 1.7x less |
| DUP12 (Prague, 1M gas) | 33.7 | 161.7 | **RISC0 is 4.8x faster** | 670.04 | 449.05 | SP1 uses 1.5x less |
| CALLDATACOPY (Prague, 1M gas, non zero data: True, fixed src dst: True, 10KiB, transaction) | 21.7 | 103.7 | **RISC0 is 4.8x faster** | 362.07 | 341.85 | SP1 uses 1.1x less |
| EXTCODEHASH (Prague, 1M gas, absent target: True) | 30.7 | 147.1 | **RISC0 is 4.8x faster** | 646.13 | 373.25 | SP1 uses 1.7x less |
| RETURN (Prague, 1M gas, 1KiB of non, zero data) | 50.2 | 240.2 | **RISC0 is 4.8x faster** | 678.01 | 376.07 | SP1 uses 1.8x less |
| CALL, STATICCALL (Prague, 1M gas) | 15.0 | 71.9 | **RISC0 is 4.8x faster** | 613.32 | 450.93 | SP1 uses 1.4x less |
| EXTCODESIZE (Prague, 1M gas, absent target: True) | 25.0 | 119.6 | **RISC0 is 4.8x faster** | 613.32 | 435.93 | SP1 uses 1.4x less |
| PUSH9 (Prague, 1M gas) | 39.6 | 189.0 | **RISC0 is 4.8x faster** | 639.57 | 526.40 | SP1 uses 1.2x less |
| BLOBBASEFEE (Prague, 1M gas) | 50.5 | 241.1 | **RISC0 is 4.8x faster** | 672.38 | 376.07 | SP1 uses 1.8x less |
| JUMPDEST, JUMP (Prague, 1M gas, 00) | 30.2 | 144.1 | **RISC0 is 4.8x faster** | 698.63 | 450.93 | SP1 uses 1.5x less |
| MLOAD (Prague, 1M gas, big memory expansion: True, offset initialized: False, 31 offset) | 46.5 | 222.0 | **RISC0 is 4.8x faster** | 645.66 | 477.65 | SP1 uses 1.4x less |
| SELFDESTRUCT (Prague, 1M gas, value bearing: True) | 15.0 | 71.5 | **RISC0 is 4.8x faster** | 670.04 | 265.91 | SP1 uses 2.5x less |
| DUP3 (Prague, 1M gas) | 33.5 | 160.0 | **RISC0 is 4.8x faster** | 469.88 | 473.43 | **RISC0 uses 1.0x less** |
| LOG0 (Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB zeros data) | 15.6 | 74.3 | **RISC0 is 4.8x faster** | 594.57 | 435.93 | SP1 uses 1.4x less |
| PUSH4 (Prague, 1M gas) | 34.2 | 163.2 | **RISC0 is 4.8x faster** | 678.01 | 449.05 | SP1 uses 1.5x less |
| JUMPDEST, JUMP (Prague, 1M gas, 605b) | 25.7 | 122.5 | **RISC0 is 4.8x faster** | 672.38 | 522.18 | SP1 uses 1.3x less |
| LOG0 (Prague, 1M gas, fixed offset: False, zeros topic, 0 bytes data) | 17.6 | 83.9 | **RISC0 is 4.8x faster** | 698.63 | 524.99 | SP1 uses 1.3x less |
| PUSH24 (Prague, 1M gas) | 54.1 | 258.1 | **RISC0 is 4.8x faster** | 698.63 | 474.84 | SP1 uses 1.5x less |
| LOG4 (Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB zeros data) | 14.9 | 71.2 | **RISC0 is 4.8x faster** | 645.66 | 539.66 | SP1 uses 1.2x less |
| CALLDATACOPY (Prague, 1M gas, non zero data: False, fixed src dst: True, 100 bytes, transaction) | 30.1 | 143.4 | **RISC0 is 4.8x faster** | 613.32 | 473.90 | SP1 uses 1.3x less |
| PUSH10 (Prague, 1M gas) | 40.8 | 194.4 | **RISC0 is 4.8x faster** | 698.63 | 477.18 | SP1 uses 1.5x less |
| LOG2 (Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB non zero data) | 14.9 | 70.9 | **RISC0 is 4.8x faster** | 678.01 | 392.94 | SP1 uses 1.7x less |
| MCOPY (Prague, 1M gas, fixed src dst: False, 1MiB) | 15.4 | 73.3 | **RISC0 is 4.8x faster** | 645.66 | 500.15 | SP1 uses 1.3x less |
| PUSH6 (Prague, 1M gas) | 37.5 | 178.8 | **RISC0 is 4.8x faster** | 670.04 | 437.34 | SP1 uses 1.5x less |
| DUP4 (Prague, 1M gas) | 33.1 | 157.9 | **RISC0 is 4.8x faster** | 646.13 | 477.65 | SP1 uses 1.4x less |
| CREATE (Prague, 1M gas) | 15.3 | 72.6 | **RISC0 is 4.8x faster** | 594.57 | 449.05 | SP1 uses 1.3x less |
| MOD, SMOD (Prague, 1M gas, op SMOD, 127 mod bits) | 184.4 | 877.6 | **RISC0 is 4.8x faster** | 698.63 | 335.75 | SP1 uses 2.1x less |
| COINBASE (Prague, 1M gas) | 63.5 | 302.3 | **RISC0 is 4.8x faster** | 700.04 | 392.94 | SP1 uses 1.8x less |
| SDIV (Prague, 1M gas, 1) | 222.0 | 1,056.2 | **RISC0 is 4.8x faster** | 645.66 | 449.05 | SP1 uses 1.4x less |
| RETURNDATASIZE (Prague, 1M gas) | 32.3 | 153.4 | **RISC0 is 4.8x faster** | 518.16 | 451.40 | SP1 uses 1.1x less |
| PUSH22 (Prague, 1M gas) | 51.7 | 245.9 | **RISC0 is 4.8x faster** | 669.10 | 476.71 | SP1 uses 1.4x less |
| ADD (Prague, 1M gas, ) | 38.3 | 182.2 | **RISC0 is 4.8x faster** | 613.32 | 213.41 | SP1 uses 2.9x less |
| REVERT (Prague, 1M gas, empty) | 88.4 | 419.9 | **RISC0 is 4.8x faster** | 645.66 | 477.65 | SP1 uses 1.4x less |
| PUSH19 (Prague, 1M gas) | 49.1 | 233.3 | **RISC0 is 4.7x faster** | 643.32 | 450.93 | SP1 uses 1.4x less |
| LOG1 (Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB non zero data) | 15.0 | 71.3 | **RISC0 is 4.7x faster** | 621.76 | 435.93 | SP1 uses 1.4x less |
| LOG2 (Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB zeros data) | 15.1 | 71.5 | **RISC0 is 4.7x faster** | 632.07 | 522.18 | SP1 uses 1.2x less |
| REVERT (Prague, 1M gas, 1MiB of non, zero data) | 15.3 | 72.4 | **RISC0 is 4.7x faster** | 670.04 | 474.84 | SP1 uses 1.4x less |
| LOG4 (Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB zeros data) | 15.0 | 71.2 | **RISC0 is 4.7x faster** | 698.63 | 392.94 | SP1 uses 1.8x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 765 mod) | 863.2 | 4,096.1 | **RISC0 is 4.7x faster** | 711.76 | 351.69 | SP1 uses 2.0x less |
| ADDRESS (Prague, 1M gas) | 65.0 | 308.5 | **RISC0 is 4.7x faster** | 363.48 | 474.84 | **RISC0 uses 1.3x less** |
| PUSH17 (Prague, 1M gas) | 46.1 | 218.6 | **RISC0 is 4.7x faster** | 613.32 | 392.00 | SP1 uses 1.6x less |
| SLOAD, SSTORE (Prague, 1M gas, SSTORE same value) | 43.7 | 207.3 | **RISC0 is 4.7x faster** | 416.45 | 473.90 | **RISC0 uses 1.1x less** |
| DUP13 (Prague, 1M gas) | 33.8 | 160.1 | **RISC0 is 4.7x faster** | 713.16 | 376.07 | SP1 uses 1.9x less |
| LOG1 (Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB zeros data) | 15.3 | 72.6 | **RISC0 is 4.7x faster** | 208.32 | 435.93 | **RISC0 uses 2.1x less** |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 8 mod even) | 1,862.8 | 8,825.8 | **RISC0 is 4.7x faster** | 698.63 | 212.00 | SP1 uses 3.3x less |
| RETURNDATACOPY (Prague, 1M gas, fixed dst: True, 1MiB) | 15.9 | 75.2 | **RISC0 is 4.7x faster** | 362.07 | 449.05 | **RISC0 uses 1.2x less** |
| DUP2 (Prague, 1M gas) | 33.6 | 158.9 | **RISC0 is 4.7x faster** | 368.16 | 476.71 | **RISC0 uses 1.3x less** |
| SHR (Prague, 1M gas, ) | 60.9 | 288.1 | **RISC0 is 4.7x faster** | 642.85 | 522.18 | SP1 uses 1.2x less |
| NOT (Prague, 1M gas) | 32.6 | 154.2 | **RISC0 is 4.7x faster** | 632.54 | 450.93 | SP1 uses 1.4x less |
| CALLDATACOPY (Prague, 1M gas, non zero data: True, fixed src dst: True, 100 bytes, transaction) | 32.8 | 154.9 | **RISC0 is 4.7x faster** | 613.32 | 449.05 | SP1 uses 1.4x less |
| SLOAD, SSTORE (Prague, 1M gas, absent slots: True) | 15.0 | 70.9 | **RISC0 is 4.7x faster** | 594.57 | 392.47 | SP1 uses 1.5x less |
| CREATE2 (Prague, 1M gas, 0 bytes) | 15.9 | 75.0 | **RISC0 is 4.7x faster** | 638.16 | 476.71 | SP1 uses 1.3x less |
| PUSH23 (Prague, 1M gas) | 53.4 | 251.9 | **RISC0 is 4.7x faster** | 645.66 | 473.90 | SP1 uses 1.4x less |
| LOG3 (Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB zeros data) | 15.1 | 71.0 | **RISC0 is 4.7x faster** | 642.85 | 449.05 | SP1 uses 1.4x less |
| DUP9 (Prague, 1M gas) | 33.8 | 159.3 | **RISC0 is 4.7x faster** | 632.54 | 449.99 | SP1 uses 1.4x less |
| BLOBHASH (Prague, 1M gas) | 51.0 | 239.9 | **RISC0 is 4.7x faster** | 643.32 | 476.24 | SP1 uses 1.4x less |
| RETURNDATACOPY (Prague, 1M gas, fixed dst: False, 0 bytes) | 50.3 | 236.2 | **RISC0 is 4.7x faster** | 624.57 | 449.05 | SP1 uses 1.4x less |
| JUMPDEST, JUMP (Prague, 1M gas, 605b5b) | 28.8 | 135.4 | **RISC0 is 4.7x faster** | 712.70 | 476.71 | SP1 uses 1.5x less |
| LOG3 (Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB non zero data) | 14.9 | 70.0 | **RISC0 is 4.7x faster** | 713.16 | 449.05 | SP1 uses 1.6x less |
| DUP15 (Prague, 1M gas) | 33.2 | 155.4 | **RISC0 is 4.7x faster** | 645.66 | 479.52 | SP1 uses 1.3x less |
| LOG2 (Prague, 1M gas, fixed offset: True, zeros topic, 0 bytes data) | 16.0 | 75.0 | **RISC0 is 4.7x faster** | 698.63 | 522.18 | SP1 uses 1.3x less |
| BALANCE (Prague, 1M gas, absent target: False) | 24.2 | 113.3 | **RISC0 is 4.7x faster** | 620.82 | 522.18 | SP1 uses 1.2x less |
| CALLDATACOPY (Prague, 1M gas, non zero data: True, fixed src dst: True, 10KiB, call) | 22.4 | 105.1 | **RISC0 is 4.7x faster** | 670.04 | 392.94 | SP1 uses 1.7x less |
| CALLDATACOPY (Prague, 1M gas, non zero data: False, fixed src dst: True, 0 bytes, call) | 37.6 | 175.9 | **RISC0 is 4.7x faster** | 672.38 | 474.84 | SP1 uses 1.4x less |
| SLOAD, SSTORE (Prague, 1M gas, absent slots: True, SSTORE new value) | 15.2 | 71.1 | **RISC0 is 4.7x faster** | 698.63 | 450.46 | SP1 uses 1.6x less |
| LOG1 (Prague, 1M gas, fixed offset: False, zeros topic, 0 bytes data) | 16.3 | 76.4 | **RISC0 is 4.7x faster** | 638.16 | 340.44 | SP1 uses 1.9x less |
| LOG0 (Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB zeros data) | 15.5 | 72.4 | **RISC0 is 4.7x faster** | 648.95 | 450.93 | SP1 uses 1.4x less |
| PUSH3 (Prague, 1M gas) | 34.9 | 163.4 | **RISC0 is 4.7x faster** | 670.04 | 473.90 | SP1 uses 1.4x less |
| SLOAD, SSTORE (Prague, 1M gas, absent slots: True) | 15.1 | 70.5 | **RISC0 is 4.7x faster** | 534.57 | 450.93 | SP1 uses 1.2x less |
| LOG1 (Prague, 1M gas, fixed offset: True, non zero topic, 0 bytes data) | 16.1 | 75.0 | **RISC0 is 4.7x faster** | 613.32 | 522.18 | SP1 uses 1.2x less |
| LOG2 (Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB zeros data) | 15.1 | 70.2 | **RISC0 is 4.7x faster** | 643.32 | 269.66 | SP1 uses 2.4x less |
| CREATE (Prague, 1M gas) | 15.4 | 71.5 | **RISC0 is 4.7x faster** | 462.38 | 449.05 | SP1 uses 1.0x less |
| PUSH27 (Prague, 1M gas) | 57.9 | 269.7 | **RISC0 is 4.7x faster** | 698.63 | 450.93 | SP1 uses 1.5x less |
| LOG2 (Prague, 1M gas, fixed offset: False, non zero topic, 0 bytes data) | 15.8 | 73.8 | **RISC0 is 4.7x faster** | 669.57 | 476.24 | SP1 uses 1.4x less |
| PUSH15 (Prague, 1M gas) | 46.6 | 216.8 | **RISC0 is 4.7x faster** | 623.63 | 451.40 | SP1 uses 1.4x less |
| DUP1 (Prague, 1M gas) | 33.8 | 157.3 | **RISC0 is 4.6x faster** | 665.82 | 477.18 | SP1 uses 1.4x less |
| RETURN (Prague, 1M gas, 1MiB of non, zero data) | 15.4 | 71.2 | **RISC0 is 4.6x faster** | 651.76 | 476.24 | SP1 uses 1.4x less |
| LOG3 (Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB non zero data) | 15.2 | 70.3 | **RISC0 is 4.6x faster** | 643.32 | 522.65 | SP1 uses 1.2x less |
| SLOAD, SSTORE (Prague, 1M gas, absent slots: False) | 20.7 | 96.1 | **RISC0 is 4.6x faster** | 621.76 | 352.16 | SP1 uses 1.8x less |
| LOG0 (Prague, 1M gas, fixed offset: True, zeros topic, 0 bytes data) | 16.1 | 74.5 | **RISC0 is 4.6x faster** | 698.63 | 450.93 | SP1 uses 1.5x less |
| PUSH28 (Prague, 1M gas) | 57.4 | 265.9 | **RISC0 is 4.6x faster** | 602.07 | 340.91 | SP1 uses 1.8x less |
| LOG3 (Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB non zero data) | 15.0 | 69.7 | **RISC0 is 4.6x faster** | 718.32 | 341.85 | SP1 uses 2.1x less |
| LOG3 (Prague, 1M gas, fixed offset: True, zeros topic, 0 bytes data) | 15.7 | 72.7 | **RISC0 is 4.6x faster** | 668.63 | 376.07 | SP1 uses 1.8x less |
| MCOPY (Prague, 1M gas, fixed src dst: True, 1MiB) | 15.3 | 70.6 | **RISC0 is 4.6x faster** | 713.16 | 373.25 | SP1 uses 1.9x less |
| LOG4 (Prague, 1M gas, fixed offset: False, zeros topic, 0 bytes data) | 16.1 | 74.2 | **RISC0 is 4.6x faster** | 593.63 | 526.40 | SP1 uses 1.1x less |
| LOG3 (Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB zeros data) | 15.1 | 69.8 | **RISC0 is 4.6x faster** | 698.63 | 450.93 | SP1 uses 1.5x less |
| GAS (Prague, 1M gas) | 37.8 | 174.7 | **RISC0 is 4.6x faster** | 699.10 | 392.94 | SP1 uses 1.8x less |
| GASLIMIT (Prague, 1M gas) | 39.5 | 182.4 | **RISC0 is 4.6x faster** | 672.38 | 373.25 | SP1 uses 1.8x less |
| LOG4 (Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB zeros data) | 15.0 | 69.2 | **RISC0 is 4.6x faster** | 362.07 | 392.94 | **RISC0 uses 1.1x less** |
| RETURNDATACOPY (Prague, 1M gas, fixed dst: False, 1MiB) | 16.0 | 73.7 | **RISC0 is 4.6x faster** | 712.70 | 522.65 | SP1 uses 1.4x less |
| LOG4 (Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB non zero data) | 15.3 | 70.1 | **RISC0 is 4.6x faster** | 522.38 | 476.24 | SP1 uses 1.1x less |
| CREATE (Prague, 1M gas, zero data) | 15.3 | 70.4 | **RISC0 is 4.6x faster** | 613.32 | 522.65 | SP1 uses 1.2x less |
| LOG2 (Prague, 1M gas, fixed offset: True, non zero topic, 0 bytes data) | 15.9 | 73.2 | **RISC0 is 4.6x faster** | 670.04 | 479.99 | SP1 uses 1.4x less |
| LOG1 (Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB zeros data) | 15.1 | 69.2 | **RISC0 is 4.6x faster** | 592.70 | 449.05 | SP1 uses 1.3x less |
| PUSH29 (Prague, 1M gas) | 60.3 | 276.9 | **RISC0 is 4.6x faster** | 713.16 | 474.84 | SP1 uses 1.5x less |
| TLOAD (Prague, 1M gas, val mut: False, key mut: True) | 18.3 | 83.8 | **RISC0 is 4.6x faster** | 670.04 | 449.99 | SP1 uses 1.5x less |
| CREATE (Prague, 1M gas, 0 bytes) | 16.4 | 75.0 | **RISC0 is 4.6x faster** | 295.98 | 479.52 | **RISC0 uses 1.6x less** |
| TLOAD (Prague, 1M gas, val mut: False, key mut: False) | 18.7 | 85.8 | **RISC0 is 4.6x faster** | 423.95 | 476.71 | **RISC0 uses 1.1x less** |
| CREATE2 (Prague, 1M gas, zero data) | 15.4 | 70.4 | **RISC0 is 4.6x faster** | 646.13 | 522.65 | SP1 uses 1.2x less |
| DUP16 (Prague, 1M gas) | 33.4 | 152.4 | **RISC0 is 4.6x faster** | 632.07 | 473.90 | SP1 uses 1.3x less |
| LOG4 (Prague, 1M gas, fixed offset: True, non zero topic, 0 bytes data) | 15.8 | 72.1 | **RISC0 is 4.6x faster** | 698.63 | 376.07 | SP1 uses 1.9x less |
| SLOAD, SSTORE (Prague, 1M gas, absent slots: True) | 15.1 | 68.8 | **RISC0 is 4.6x faster** | 645.66 | 477.18 | SP1 uses 1.4x less |
| PUSH25 (Prague, 1M gas) | 55.7 | 253.8 | **RISC0 is 4.6x faster** | 613.32 | 450.93 | SP1 uses 1.4x less |
| SELFDESTRUCT (Prague, 1M gas, value bearing: False) | 15.3 | 69.6 | **RISC0 is 4.6x faster** | 629.26 | 373.25 | SP1 uses 1.7x less |
| LOG3 (Prague, 1M gas, fixed offset: False, zeros topic, 0 bytes data) | 15.7 | 71.3 | **RISC0 is 4.6x faster** | 712.23 | 473.43 | SP1 uses 1.5x less |
| Block Full Data (Non Zero Byte) | 15.2 | 69.1 | **RISC0 is 4.5x faster** | 712.70 | 373.25 | SP1 uses 1.9x less |
| JUMPDEST, JUMP (Prague, 1M gas, 615b5b5b) | 27.0 | 122.8 | **RISC0 is 4.5x faster** | 611.91 | 265.91 | SP1 uses 2.3x less |
| LOG3 (Prague, 1M gas, fixed offset: True, non zero topic, 0 bytes data) | 15.9 | 72.1 | **RISC0 is 4.5x faster** | 593.16 | 524.99 | SP1 uses 1.1x less |
| BLOBHASH (Prague, 1M gas, no blobs) | 37.1 | 168.1 | **RISC0 is 4.5x faster** | 628.79 | 449.05 | SP1 uses 1.4x less |
| LOG1 (Prague, 1M gas, fixed offset: True, zeros topic, 0 bytes data) | 16.1 | 73.1 | **RISC0 is 4.5x faster** | 613.32 | 376.07 | SP1 uses 1.6x less |
| BLOCKHASH (Prague, 1M gas) | 41.2 | 186.8 | **RISC0 is 4.5x faster** | 593.16 | 450.93 | SP1 uses 1.3x less |
| PRECOMPILE_MODEXP, CALL (Prague, 1M gas, 512 mod even) | 15.1 | 68.5 | **RISC0 is 4.5x faster** | 593.16 | 500.15 | SP1 uses 1.2x less |
| PUSH31 (Prague, 1M gas) | 61.9 | 280.1 | **RISC0 is 4.5x faster** | 711.76 | 435.93 | SP1 uses 1.6x less |
| LOG3 (Prague, 1M gas, fixed offset: False, non zero topic, 0 bytes data) | 16.0 | 72.6 | **RISC0 is 4.5x faster** | 640.51 | 340.44 | SP1 uses 1.9x less |
| CREATE (Prague, 1M gas, 0 bytes) | 16.0 | 72.2 | **RISC0 is 4.5x faster** | 628.79 | 450.93 | SP1 uses 1.4x less |
| CREATE (Prague, 1M gas, zero data) | 15.6 | 70.4 | **RISC0 is 4.5x faster** | 670.04 | 435.93 | SP1 uses 1.5x less |
| CREATE2 (Prague, 1M gas, 0 bytes) | 16.2 | 73.4 | **RISC0 is 4.5x faster** | 640.51 | 500.15 | SP1 uses 1.3x less |
| CREATE2 (Prague, 1M gas, zero data) | 16.0 | 72.0 | **RISC0 is 4.5x faster** | 672.38 | 450.93 | SP1 uses 1.5x less |
| PUSH26 (Prague, 1M gas) | 57.0 | 256.8 | **RISC0 is 4.5x faster** | 669.57 | 351.69 | SP1 uses 1.9x less |
| CREATE (Prague, 1M gas, zero data) | 16.1 | 72.5 | **RISC0 is 4.5x faster** | 453.01 | 301.54 | SP1 uses 1.5x less |
| CREATE (Prague, 1M gas) | 16.1 | 72.5 | **RISC0 is 4.5x faster** | 616.60 | 479.99 | SP1 uses 1.3x less |
| PUSH12 (Prague, 1M gas) | 42.8 | 192.3 | **RISC0 is 4.5x faster** | 543.01 | 212.00 | SP1 uses 2.6x less |
| LOG2 (Prague, 1M gas, fixed offset: False, zeros topic, 0 bytes data) | 15.7 | 70.7 | **RISC0 is 4.5x faster** | 669.10 | 522.65 | SP1 uses 1.3x less |
| CREATE2 (Prague, 1M gas) | 15.9 | 71.3 | **RISC0 is 4.5x faster** | 638.16 | 450.93 | SP1 uses 1.4x less |
| CREATE2 (Prague, 1M gas, zero data) | 16.5 | 74.1 | **RISC0 is 4.5x faster** | 642.85 | 406.87 | SP1 uses 1.6x less |
| SLOAD, SSTORE (Prague, 1M gas, absent slots: False, SSTORE new value) | 21.0 | 94.2 | **RISC0 is 4.5x faster** | 628.32 | 522.18 | SP1 uses 1.2x less |
| PUSH30 (Prague, 1M gas) | 61.1 | 272.9 | **RISC0 is 4.5x faster** | 698.63 | 308.10 | SP1 uses 2.3x less |
| CREATE2 (Prague, 1M gas, zero data) | 16.3 | 72.4 | **RISC0 is 4.5x faster** | 670.04 | 522.18 | SP1 uses 1.3x less |
| PUSH18 (Prague, 1M gas) | 47.5 | 211.2 | **RISC0 is 4.4x faster** | 643.32 | 522.18 | SP1 uses 1.2x less |
| SELFDESTRUCT (Prague, 1M gas, value bearing: False) | 20.8 | 92.3 | **RISC0 is 4.4x faster** | 712.23 | 522.65 | SP1 uses 1.4x less |
| LOG4 (Prague, 1M gas, fixed offset: True, zeros topic, 0 bytes data) | 16.0 | 70.7 | **RISC0 is 4.4x faster** | 380.82 | 450.46 | **RISC0 uses 1.2x less** |
| SLOAD, SSTORE (Prague, 1M gas, absent slots: False) | 21.0 | 92.9 | **RISC0 is 4.4x faster** | 643.32 | 449.99 | SP1 uses 1.4x less |
| PRECOMPILE_BLAKE2F, CALL, STATICCALL (Prague, 1M gas) | 646.7 | 2,849.7 | **RISC0 is 4.4x faster** | 669.57 | 525.93 | SP1 uses 1.3x less |
| SELFDESTRUCT (Prague, 1M gas, value bearing: True) | 21.6 | 95.2 | **RISC0 is 4.4x faster** | 594.57 | 450.93 | SP1 uses 1.3x less |
| BLOBHASH (Prague, 1M gas, existent index) | 37.7 | 165.8 | **RISC0 is 4.4x faster** | 698.63 | 450.93 | SP1 uses 1.5x less |
| SLOAD, SSTORE (Prague, 1M gas, absent slots: True, SSLOAD) | 21.8 | 95.3 | **RISC0 is 4.4x faster** | 641.91 | 451.40 | SP1 uses 1.4x less |
| LOG4 (Prague, 1M gas, fixed offset: False, non zero topic, 0 bytes data) | 16.0 | 70.0 | **RISC0 is 4.4x faster** | 610.51 | 477.65 | SP1 uses 1.3x less |
| SLOAD, SSTORE (Prague, 1M gas, absent slots: False, SSTORE same value) | 27.4 | 119.2 | **RISC0 is 4.3x faster** | 641.91 | 372.79 | SP1 uses 1.7x less |
| PUSH32 (Prague, 1M gas) | 65.5 | 277.6 | **RISC0 is 4.2x faster** | 572.54 | 437.34 | SP1 uses 1.3x less |
| Block Full Access List And Data | 19.2 | 80.5 | **RISC0 is 4.2x faster** | 620.82 | 335.75 | SP1 uses 1.8x less |
| Block Full Data (Zero Byte) | 19.5 | 81.8 | **RISC0 is 4.2x faster** | 638.16 | 450.93 | SP1 uses 1.4x less |
| SLOAD, SSTORE (Prague, 1M gas, absent slots: False) | 27.0 | 112.7 | **RISC0 is 4.2x faster** | 672.38 | 473.90 | SP1 uses 1.4x less |
| SLOAD, SSTORE (Prague, 1M gas, absent slots: False) | 27.7 | 114.9 | **RISC0 is 4.1x faster** | 698.63 | 522.65 | SP1 uses 1.3x less |
| SLOAD, SSTORE (Prague, 1M gas, absent slots: False, SSLOAD) | 27.4 | 113.5 | **RISC0 is 4.1x faster** | 640.04 | 474.84 | SP1 uses 1.3x less |
| BALANCE (Prague, 1M gas, absent accounts: False) | 26.2 | 107.9 | **RISC0 is 4.1x faster** | 651.29 | 375.60 | SP1 uses 1.7x less |
| Empty Block | 13.3 | 51.9 | **RISC0 is 3.9x faster** | 594.57 | 265.91 | SP1 uses 2.2x less |
| CALL, STATICCALL (Prague, 1M gas) | 230.0 | 711.2 | **RISC0 is 3.1x faster** | 670.04 | 522.18 | SP1 uses 1.3x less |
| CALL, STATICCALL (Prague, 1M gas) | 229.8 | 693.4 | **RISC0 is 3.0x faster** | 713.16 | 450.46 | SP1 uses 1.6x less |
| DELEGATECALL (Prague, 1M gas) | 274.8 | 818.8 | **RISC0 is 3.0x faster** | 669.57 | 450.93 | SP1 uses 1.5x less |
| PRECOMPILE_EC_PAIRING, CALL (Prague, 1M gas) | 230.4 | 682.3 | **RISC0 is 3.0x faster** | 448.32 | 448.59 | **RISC0 uses 1.0x less** |
| EXTCODECOPY (Prague, 1M gas) | 266.8 | 786.5 | **RISC0 is 2.9x faster** | 677.54 | 449.05 | SP1 uses 1.5x less |
| CALL (Prague, 1M gas) | 277.7 | 816.9 | **RISC0 is 2.9x faster** | 613.32 | 371.38 | SP1 uses 1.7x less |
| STATICCALL (Prague, 1M gas) | 275.2 | 803.0 | **RISC0 is 2.9x faster** | 665.82 | 479.99 | SP1 uses 1.4x less |
| EXTCODEHASH (Prague, 1M gas) | 275.2 | 791.1 | **RISC0 is 2.9x faster** | 670.04 | 450.93 | SP1 uses 1.5x less |
| EXTCODESIZE (Prague, 1M gas) | 275.9 | 790.6 | **RISC0 is 2.9x faster** | 361.13 | 449.99 | **RISC0 uses 1.2x less** |
| CALLCODE (Prague, 1M gas) | 280.2 | 800.4 | **RISC0 is 2.9x faster** | 668.63 | 450.46 | SP1 uses 1.5x less |
| Block Full Of Ether Transfers (A To Diff Acc) | 47.9 | 115.4 | **RISC0 is 2.4x faster** | 613.32 | 450.93 | SP1 uses 1.4x less |
| Block Full Of Ether Transfers (A To A) | 47.7 | 112.9 | **RISC0 is 2.4x faster** | 668.63 | 451.40 | SP1 uses 1.5x less |
| Block Full Of Ether Transfers (A To B) | 46.3 | 108.3 | **RISC0 is 2.3x faster** | 698.63 | 474.84 | SP1 uses 1.5x less |
| Block Full Of Ether Transfers (Diff Acc To B) | 49.6 | 112.2 | **RISC0 is 2.3x faster** | 698.63 | 418.59 | SP1 uses 1.7x less |
| Block Full Of Ether Transfers (Diff Acc To Diff Acc) | 50.3 | 108.2 | **RISC0 is 2.2x faster** | 513.48 | 522.65 | **RISC0 uses 1.0x less** |
| SHA3, KECCAK256 (Prague, 1M gas) | 133.9 | 261.9 | **RISC0 is 2.0x faster** | 668.63 | 450.93 | SP1 uses 1.5x less |
| PRECOMPILE_ECRECOVER, CALL, STATICCALL (Prague, 1M gas) | 217.9 | 357.3 | **RISC0 is 1.6x faster** | 670.04 | 351.22 | SP1 uses 1.9x less |

---

## Notes

- **Proving Time Winner** shows which system is faster and by how much
- **Memory Winner** shows which system uses less memory and by how much
- Table is sorted by proving time performance (best RISC0 performance at top)
- Bold entries indicate RISC0 wins for that metric
