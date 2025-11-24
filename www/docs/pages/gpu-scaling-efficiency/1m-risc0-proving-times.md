# RISC0 Proving Times Report

## Overview

This report shows the actual proving times for each test across different GPU configurations.

**Hardware:** AMD EPYC 7543 (32-Core), 472 GiB RAM, 4x NVIDIA RTX 5090

---

## Summary

| GPUs | Tests | Mean Time | Median Time | Min Time | Max Time |
| --- | --- | --- | --- | --- | --- |
| 1 GPU | 503 | 3m 11.0s | 40.64s | 13.34s | 53m 5.8s |
| 2 GPUs | 504 | 1m 51.4s | 29.33s | 12.01s | 29m 18.8s |
| 3 GPUs | 501 | 1m 10.9s | 24.38s | 10.67s | 19m 29.5s |
| 4 GPUs | 502 | 1m 2.9s | 22.40s | 13.83s | 14m 41.2s |

---

## Proving Times by Test Category

### Worst Blocks.Py

**8 tests**

| Test Name | 1 GPU | 2 GPUs | 3 GPUs | 4 GPUs |
| --- | --- | --- | --- | --- |
| Block Full Access List And Data: benchmark: gas: value 1M: blockchain test from state test | 19.21s | 15.55s | 14.85s | 20.36s |
| Block Full Data: False | 15.20s | 13.16s | 12.28s | 17.04s |
| Block Full Data: True | 19.53s | 15.88s | 19.78s | 21.55s |
| Block Full Of Ether Transfers: a to a | 47.67s | 33.41s | 27.37s | 24.93s |
| Block Full Of Ether Transfers: a to b | 46.27s | 33.28s | 28.27s | 24.80s |
| Block Full Of Ether Transfers: a to diff acc | 47.86s | 34.23s | 27.40s | 24.91s |
| Block Full Of Ether Transfers: diff acc to b | 49.61s | 33.77s | 27.96s | 24.36s |
| Block Full Of Ether Transfers: diff acc to diff acc | 50.30s | 34.37s | 27.68s | 24.40s |

**Category Statistics:**

| GPUs | Mean | Median |
| --- | --- | --- |
| 1 GPU | 36.95s | 46.97s |
| 2 GPUs | 26.71s | 33.34s |
| 3 GPUs | 23.20s | 27.38s |
| 4 GPUs | 22.79s | 24.38s |

---

### Worst Bytecode.Py

**35 tests**

| Test Name | 1 GPU | 2 GPUs | 3 GPUs | 4 GPUs |
| --- | --- | --- | --- | --- |
| Bytecode Single Opcode: CALLCODE | 4m 40.2s | 2m 30.7s | 1m 45.6s | 1m 27.1s |
| Bytecode Single Opcode: CALL | 4m 37.7s | 2m 31.7s | 1m 47.0s | 1m 27.4s |
| Bytecode Single Opcode: DELEGATECALL | 4m 34.8s | 2m 30.5s | 1m 46.2s | 1m 25.4s |
| Bytecode Single Opcode: EXTCODECOPY | 4m 26.8s | 2m 24.0s | 1m 44.8s | 1m 23.4s |
| Bytecode Single Opcode: EXTCODEHASH | 4m 35.2s | 2m 30.2s | 1m 46.9s | 1m 25.8s |
| Bytecode Single Opcode: EXTCODESIZE | 4m 35.9s | 2m 29.5s | 1m 46.0s | 1m 25.3s |
| Bytecode Single Opcode: STATICCALL | 4m 35.2s | 2m 31.3s | 1m 46.3s | 1m 26.4s |
| Create: 0 BYTEs with value: CREATE2 | 16.25s | 16.67s | 13.01s | 16.99s |
| Create: 0 BYTEs with value: CREATE | 15.97s | 16.38s | 13.35s | 14.87s |
| Create: 0 BYTEs without value: CREATE2 | 15.90s | 14.53s | 13.37s | 16.47s |
| Create: 0 BYTEs without value: CREATE | 16.38s | 13.61s | 11.59s | 15.18s |
| Create: 0.25x max code size with non: zero data: CREATE2 | 15.38s | 16.15s | 12.49s | 18.12s |
| Create: 0.25x max code size with non: zero data: CREATE | 15.32s | 13.31s | 13.09s | 16.57s |
| Create: 0.25x max code size with zero data: CREATE2 | 15.02s | 12.95s | 13.22s | 16.39s |
| Create: 0.25x max code size with zero data: CREATE | 15.14s | 16.06s | 13.28s | 14.17s |
| Create: 0.50x max code size with non: zero data: CREATE2 | 15.96s | 13.29s | 13.58s | 15.23s |
| Create: 0.50x max code size with non: zero data: CREATE | 15.29s | 15.23s | 13.08s | 16.22s |
| Create: 0.50x max code size with zero data: CREATE2 | 15.25s | 19.12s | 12.43s | 15.85s |
| Create: 0.50x max code size with zero data: CREATE | 15.11s | 12.48s | 13.29s | 15.54s |
| Create: 0.75x max code size with non: zero data: CREATE2 | 16.54s | 13.52s | 16.12s | 16.20s |
| Create: 0.75x max code size with non: zero data: CREATE | 15.58s | 13.67s | 11.71s | 17.37s |
| Create: 0.75x max code size with zero data: CREATE2 | 14.94s | 13.10s | 13.60s | 16.65s |
| Create: 0.75x max code size with zero data: CREATE | 15.36s | 13.52s | 12.63s | 15.57s |
| Create: max code size with non: zero data: CREATE2 | 16.26s | 18.84s | 15.82s | 16.34s |
| Create: max code size with non: zero data: CREATE | 16.12s | 15.93s | 13.25s | 16.56s |
| Create: max code size with zero data: CREATE2 | 15.91s | 13.06s | 12.85s | 18.11s |
| Create: max code size with zero data: CREATE | 15.25s | 12.31s | 15.69s | 15.96s |
| Creates Collisions: CREATE2 | 15.10s | 13.25s | 13.26s | 16.57s |
| Creates Collisions: CREATE | 16.13s | 13.74s | 13.66s | 17.72s |
| Initcode Jumpdest Analysis: 00 | 30.18s | 24.69s | 20.31s | 19.16s |
| Initcode Jumpdest Analysis: 5b | 37.37s | 25.22s | 23.76s | 21.30s |
| Initcode Jumpdest Analysis: 605b5b | 28.84s | 23.74s | 20.17s | 19.09s |
| Initcode Jumpdest Analysis: 605b | 25.68s | 23.18s | 17.36s | 18.95s |
| Initcode Jumpdest Analysis: 615b5b5b | 27.03s | 22.63s | 19.95s | 19.03s |
| Initcode Jumpdest Analysis: 615b5b | 23.57s | 20.83s | 18.02s | 17.35s |

**Category Statistics:**

| GPUs | Mean | Median |
| --- | --- | --- |
| 1 GPU | 1m 9.8s | 16.13s |
| 2 GPUs | 43.11s | 16.15s |
| 3 GPUs | 33.05s | 13.60s |
| 4 GPUs | 30.70s | 16.99s |

---

### Worst Compute.Py

**311 tests**

| Test Name | 1 GPU | 2 GPUs | 3 GPUs | 4 GPUs |
| --- | --- | --- | --- | --- |
| Amortized Bn128 Pairings: benchmark: gas: value 1M: blockchain test from state test | 3m 50.4s | 2m 6.0s | 1m 28.6s | 1m 11.8s |
| Empty Block: benchmark: gas: value 1M: blockchain test | 13.34s | 13.62s | 10.87s | 16.60s |
| Binop Simple: ADD: | 38.33s | 28.54s | 24.10s | 21.30s |
| Binop Simple: AND: | 34.45s | 25.66s | 21.93s | 20.57s |
| Binop Simple: BYTE: | 35.46s | 26.48s | 22.47s | 21.53s |
| Binop Simple: DIV: 0 | 3m 12.7s | 1m 45.0s | 1m 13.3s | 1m 0.3s |
| Binop Simple: DIV: 1 | 2m 58.5s | 1m 38.6s | 1m 11.2s | 56.41s |
| Binop Simple: EQ: | 1m 11.7s | 43.87s | 33.83s | 29.11s |
| Binop Simple: EXP: | 1m 52.8s | N/A | 47.72s | 40.98s |
| Binop Simple: GT: | 36.06s | 26.76s | 22.52s | 21.08s |
| Binop Simple: LT: | 35.81s | 26.35s | 23.60s | 21.42s |
| Binop Simple: MOD: | 57.43s | 38.58s | 30.36s | 26.57s |
| Binop Simple: MUL: | 58.75s | 38.16s | 30.70s | 26.19s |
| Binop Simple: OR: | 33.76s | 25.70s | 23.04s | 20.04s |
| Binop Simple: SAR: | 1m 7.3s | 42.64s | 33.73s | 29.25s |
| Binop Simple: SDIV: 0 | 3m 34.6s | 1m 55.3s | 1m 22.7s | 1m 4.8s |
| Binop Simple: SDIV: 1 | 3m 42.0s | 1m 56.1s | 1m 21.9s | 1m 5.3s |
| Binop Simple: SGT: | 1m 13.4s | 47.13s | 36.20s | 31.23s |
| Binop Simple: SHL: | 59.19s | 38.90s | 30.72s | 26.14s |
| Binop Simple: SHR: | 1m 0.9s | 35.51s | 29.59s | 27.18s |
| Binop Simple: SIGNEXTEND: | 54.59s | 32.34s | 28.49s | 25.66s |
| Binop Simple: SLT: | 40.64s | 28.28s | 24.70s | 22.06s |
| Binop Simple: SMOD: | 1m 7.3s | 41.55s | 32.30s | 27.81s |
| Binop Simple: SUB: | 40.89s | 29.23s | 24.49s | 22.64s |
| Binop Simple: XOR: | 33.77s | 26.71s | 21.68s | 21.16s |
| Blobhash: no blobs | 37.06s | 27.64s | 23.02s | 21.64s |
| Blobhash: one blob AND accessed | 51.08s | 34.25s | 28.30s | 24.85s |
| Blobhash: one blob but access non: existent index | 37.71s | 27.79s | 23.79s | 21.03s |
| Blobhash: six blobs, access latest | 50.99s | 34.51s | 27.51s | 24.13s |
| Calldataload: empty | 1m 7.2s | 43.47s | 33.94s | 29.82s |
| Calldataload: one: loop | 1m 18.7s | 48.32s | 36.03s | 32.34s |
| Calldataload: zero: loop | 1m 19.3s | 49.51s | 37.51s | 32.62s |
| Calldatasize: CALLdata lenGTh 0 | 34.86s | 25.69s | 22.90s | 20.31s |
| Calldatasize: CALLdata lenGTh 10000 | 34.21s | 27.06s | 21.25s | 20.80s |
| Calldatasize: CALLdata lenGTh 1000 | 35.05s | 26.43s | 20.96s | 20.52s |
| Callvalue: from ORigin False: non zero value False | 32.95s | 26.17s | 22.86s | 20.93s |
| Callvalue: from ORigin False: non zero value True | 32.47s | 25.44s | 22.62s | 20.42s |
| Callvalue: from ORigin True: non zero value False | 33.09s | 24.87s | 20.83s | 20.46s |
| Callvalue: from ORigin True: non zero value True | 32.70s | 25.60s | 22.41s | 20.23s |
| Dup: DUP10 | 32.48s | 25.80s | 22.18s | 20.85s |
| Dup: DUP11 | 33.45s | 25.27s | 21.48s | 20.90s |
| Dup: DUP12 | 33.74s | 25.46s | 22.57s | 19.70s |
| Dup: DUP13 | 33.78s | 25.73s | 21.57s | 20.83s |
| Dup: DUP14 | 32.82s | 25.25s | 21.49s | 20.71s |
| Dup: DUP15 | 33.16s | 25.32s | 20.61s | 21.09s |
| Dup: DUP16 | 33.37s | 25.48s | 22.34s | 20.34s |
| Dup: DUP1 | 33.83s | 26.22s | 22.63s | 20.40s |
| Dup: DUP2 | 33.55s | 25.30s | 21.48s | 20.43s |
| Dup: DUP3 | 33.50s | 24.95s | 22.50s | 20.70s |
| Dup: DUP4 | 33.15s | 25.25s | 21.34s | 20.10s |
| Dup: DUP5 | 33.18s | 25.78s | 22.02s | 20.88s |
| Dup: DUP6 | 33.04s | 25.77s | 20.47s | 20.78s |
| Dup: DUP7 | 32.85s | 25.26s | 21.29s | 20.29s |
| Dup: DUP8 | 31.50s | 24.42s | 22.41s | 21.71s |
| Dup: DUP9 | 33.85s | 25.99s | 22.15s | 20.53s |
| Jumpdests: benchmark: gas: value 1M: blockchain test from state test | 33.55s | 25.40s | 22.50s | 19.70s |
| Jumpi Fallthrough: benchmark: gas: value 1M: blockchain test from state test | 42.61s | 30.42s | 25.57s | 22.34s |
| Jumpis: benchmark: gas: value 1M: blockchain test from state test | 28.79s | 21.02s | 18.97s | 19.59s |
| Jumps: benchmark: gas: value 1M: blockchain test from state test | 24.71s | 21.84s | 17.46s | 18.70s |
| Keccak: benchmark: gas: value 1M: blockchain test from state test | 2m 13.9s | 1m 19.0s | 58.48s | 48.91s |
| Memory Access: big memORy EXPansion False: offset initialized False: offset 0: MLOAD | 46.53s | 32.07s | 26.84s | 24.02s |
| Memory Access: big memORy EXPansion False: offset initialized False: offset 0: MSTORE8 | 40.67s | 29.09s | 24.86s | 21.18s |
| Memory Access: big memORy EXPansion False: offset initialized False: offset 0: MSTORE | 52.53s | 34.71s | 27.50s | 23.81s |
| Memory Access: big memORy EXPansion False: offset initialized False: offset 1: MLOAD | 47.32s | 32.63s | 25.24s | 23.55s |
| Memory Access: big memORy EXPansion False: offset initialized False: offset 1: MSTORE8 | 40.92s | 29.00s | 24.38s | 22.13s |
| Memory Access: big memORy EXPansion False: offset initialized False: offset 1: MSTORE | 58.77s | 36.87s | 30.17s | 26.65s |
| Memory Access: big memORy EXPansion False: offset initialized False: offset 31: MLOAD | 47.10s | 33.24s | 26.71s | 23.90s |
| Memory Access: big memORy EXPansion False: offset initialized False: offset 31: MSTORE8 | 39.26s | 28.73s | 22.26s | 22.50s |
| Memory Access: big memORy EXPansion False: offset initialized False: offset 31: MSTORE | 58.12s | 38.55s | 30.53s | 27.47s |
| Memory Access: big memORy EXPansion False: offset initialized True: offset 0: MLOAD | 47.26s | 32.01s | 26.49s | 23.50s |
| Memory Access: big memORy EXPansion False: offset initialized True: offset 0: MSTORE8 | 40.29s | 29.23s | 22.36s | 22.27s |
| Memory Access: big memORy EXPansion False: offset initialized True: offset 0: MSTORE | 52.98s | 33.82s | 26.77s | 24.32s |
| Memory Access: big memORy EXPansion False: offset initialized True: offset 1: MLOAD | 46.73s | 32.60s | 27.02s | 23.15s |
| Memory Access: big memORy EXPansion False: offset initialized True: offset 1: MSTORE8 | 39.78s | 29.30s | 24.59s | 22.77s |
| Memory Access: big memORy EXPansion False: offset initialized True: offset 1: MSTORE | 59.27s | 37.87s | 28.52s | 26.27s |
| Memory Access: big memORy EXPansion False: offset initialized True: offset 31: MLOAD | 47.00s | 31.12s | 26.55s | 22.74s |
| Memory Access: big memORy EXPansion False: offset initialized True: offset 31: MSTORE8 | 38.87s | 28.67s | 24.27s | 22.61s |
| Memory Access: big memORy EXPansion False: offset initialized True: offset 31: MSTORE | 57.65s | 38.28s | 31.01s | 26.68s |
| Memory Access: big memORy EXPansion True: offset initialized False: offset 0: MLOAD | 46.82s | 31.25s | 26.71s | 23.63s |
| Memory Access: big memORy EXPansion True: offset initialized False: offset 0: MSTORE8 | 39.97s | 27.98s | 22.01s | 22.03s |
| Memory Access: big memORy EXPansion True: offset initialized False: offset 0: MSTORE | 51.47s | 34.26s | 28.57s | 25.24s |
| Memory Access: big memORy EXPansion True: offset initialized False: offset 1: MLOAD | 47.20s | 32.26s | 26.55s | 23.87s |
| Memory Access: big memORy EXPansion True: offset initialized False: offset 1: MSTORE8 | 40.04s | 29.43s | 24.52s | 21.32s |
| Memory Access: big memORy EXPansion True: offset initialized False: offset 1: MSTORE | 57.71s | 35.82s | 30.43s | 26.76s |
| Memory Access: big memORy EXPansion True: offset initialized False: offset 31: MLOAD | 46.49s | 32.35s | 25.57s | 22.84s |
| Memory Access: big memORy EXPansion True: offset initialized False: offset 31: MSTORE8 | 38.72s | 28.30s | 23.77s | 22.18s |
| Memory Access: big memORy EXPansion True: offset initialized False: offset 31: MSTORE | 57.52s | 38.27s | 29.16s | 26.59s |
| Memory Access: big memORy EXPansion True: offset initialized True: offset 0: MLOAD | 46.26s | 32.70s | 26.63s | 24.45s |
| Memory Access: big memORy EXPansion True: offset initialized True: offset 0: MSTORE8 | 40.25s | 29.40s | 24.74s | 21.58s |
| Memory Access: big memORy EXPansion True: offset initialized True: offset 0: MSTORE | 51.87s | 34.56s | 28.74s | 24.15s |
| Memory Access: big memORy EXPansion True: offset initialized True: offset 1: MLOAD | 46.16s | 32.43s | 24.95s | 23.90s |
| Memory Access: big memORy EXPansion True: offset initialized True: offset 1: MSTORE8 | 39.27s | 29.36s | 24.56s | 22.06s |
| Memory Access: big memORy EXPansion True: offset initialized True: offset 1: MSTORE | 58.14s | 38.42s | 29.28s | 26.82s |
| Memory Access: big memORy EXPansion True: offset initialized True: offset 31: MLOAD | 47.21s | 33.04s | 25.69s | 23.47s |
| Memory Access: big memORy EXPansion True: offset initialized True: offset 31: MSTORE8 | 39.37s | 29.44s | 23.67s | 22.55s |
| Memory Access: big memORy EXPansion True: offset initialized True: offset 31: MSTORE | 58.33s | 38.98s | 30.44s | 25.92s |
| Mod: op MOD: MOD bits 127 | 2m 55.8s | 1m 37.4s | 1m 9.8s | 56.45s |
| Mod: op MOD: MOD bits 191 | 3m 44.6s | 2m 2.7s | 1m 25.9s | 1m 7.7s |
| Mod: op MOD: MOD bits 255 | 2m 50.2s | 1m 30.0s | 1m 6.6s | 53.58s |
| Mod: op MOD: MOD bits 63 | 1m 59.0s | 1m 8.8s | 51.16s | 42.12s |
| Mod: op SMOD: MOD bits 127 | 3m 4.4s | 1m 39.1s | 1m 10.9s | 57.78s |
| Mod: op SMOD: MOD bits 191 | 3m 53.9s | 2m 4.9s | 1m 27.7s | 1m 10.6s |
| Mod: op SMOD: MOD bits 255 | 2m 55.1s | 1m 35.8s | 1m 8.1s | 55.55s |
| Mod: op SMOD: MOD bits 63 | 2m 8.4s | 1m 12.1s | 51.83s | 44.35s |
| Modarith: op ADDMOD: MOD bits 127 | 3m 27.1s | 1m 53.6s | 1m 19.7s | 1m 4.6s |
| Modarith: op ADDMOD: MOD bits 191 | 4m 19.8s | 2m 20.9s | 1m 38.8s | N/A |
| Modarith: op ADDMOD: MOD bits 255 | 3m 22.3s | 1m 49.2s | 1m 14.9s | 1m 2.5s |
| Modarith: op ADDMOD: MOD bits 63 | 2m 23.6s | N/A | 58.90s | 48.48s |
| Modarith: op MULMOD: MOD bits 127 | 4m 7.7s | 2m 11.7s | 1m 33.2s | 1m 14.1s |
| Modarith: op MULMOD: MOD bits 191 | 5m 59.1s | 3m 9.1s | 2m 9.4s | 1m 42.1s |
| Modarith: op MULMOD: MOD bits 255 | 5m 45.8s | 2m 56.4s | 2m 4.8s | 1m 38.1s |
| Modarith: op MULMOD: MOD bits 63 | 3m 10.6s | 1m 44.0s | 1m 14.9s | 1m 0.1s |
| Modexp: MOD 1045 gas base heavy | 48m 33.4s | 24m 49.3s | 16m 55.7s | 12m 57.6s |
| Modexp: MOD 1360 gas BALANCEd | 12m 50.4s | 6m 28.9s | 4m 27.1s | 3m 27.1s |
| Modexp: MOD 400 gas EXP heavy | 15m 59.7s | 8m 19.4s | 5m 37.4s | 4m 19.1s |
| Modexp: MOD 408 gas BALANCEd | 11m 16.7s | 5m 44.1s | 3m 53.1s | 3m 2.4s |
| Modexp: MOD 408 gas base heavy | N/A | 22m 37.6s | 15m 36.0s | 11m 54.1s |
| Modexp: MOD 600 as BALANCEd | 11m 16.4s | 5m 57.6s | 3m 56.8s | 3m 4.8s |
| Modexp: MOD 600 gas EXP heavy | 17m 36.4s | 8m 55.4s | 6m 0.3s | 4m 35.6s |
| Modexp: MOD 616 gas base heavy | 47m 55.6s | 24m 17.6s | 16m 34.3s | 12m 20.2s |
| Modexp: MOD 677 gas base heavy | 12m 5.1s | 6m 12.4s | 4m 14.6s | 3m 16.4s |
| Modexp: MOD 765 gas EXP heavy | 14m 23.2s | 7m 7.1s | 4m 48.2s | 3m 42.1s |
| Modexp: MOD 767 gas BALANCEd | 10m 50.7s | 5m 38.3s | 3m 51.3s | 3m 0.5s |
| Modexp: MOD 800 gas base heavy | 48m 41.8s | 25m 11.9s | 16m 43.8s | 12m 39.9s |
| Modexp: MOD 800 gas EXP heavy | 17m 38.1s | 9m 8.0s | 6m 12.3s | 4m 42.2s |
| Modexp: MOD 852 gas EXP heavy | 17m 38.8s | 9m 16.7s | 6m 5.8s | 4m 41.1s |
| Modexp: MOD 867 gas base heavy | 49m 5.1s | 25m 32.6s | N/A | 12m 53.6s |
| Modexp: MOD 996 gas BALANCEd | N/A | 5m 44.6s | 3m 52.3s | 3m 1.2s |
| Modexp: MOD even 1024b EXP 1024 | 15.02s | 12.30s | 12.68s | 16.63s |
| Modexp: MOD even 128b EXP 1024 | 10m 4.7s | 5m 17.4s | 3m 36.3s | 2m 47.5s |
| Modexp: MOD even 16b EXP 320 | 17m 20.4s | 8m 38.1s | 5m 52.6s | 4m 30.5s |
| Modexp: MOD even 24b EXP 168 | 14m 5.6s | 7m 4.1s | 4m 47.8s | 3m 41.3s |
| Modexp: MOD even 256b EXP 1024 | 7m 10.2s | 3m 43.7s | 2m 34.3s | 2m 1.9s |
| Modexp: MOD even 32b EXP 256 | 13m 18.9s | 6m 45.7s | 4m 28.9s | 3m 31.9s |
| Modexp: MOD even 32b EXP 40 | 10m 33.9s | 5m 29.9s | 3m 44.0s | 2m 55.6s |
| Modexp: MOD even 32b EXP 96 | 12m 39.3s | 6m 24.0s | 4m 17.5s | 3m 16.8s |
| Modexp: MOD even 512b EXP 1024 | 15.13s | 15.57s | 14.09s | 18.32s |
| Modexp: MOD even 64b EXP 512 | 11m 10.8s | 5m 37.8s | 3m 56.3s | 3m 1.2s |
| Modexp: MOD even 8b EXP 896 | 31m 2.8s | 15m 6.1s | 10m 15.5s | 7m 42.0s |
| Modexp: MOD EXP 208 gas BALANCEd | 9m 55.1s | 5m 9.5s | 3m 29.5s | 2m 44.5s |
| Modexp: MOD EXP 215 gas EXP heavy | 27m 20.5s | 13m 27.8s | 9m 7.3s | 6m 59.7s |
| Modexp: MOD EXP 298 gas EXP heavy | 29m 27.4s | 14m 45.5s | 10m 2.1s | 7m 36.9s |
| Modexp: MOD min as BALANCEd | 10m 45.1s | 5m 34.9s | 3m 47.1s | 2m 53.9s |
| Modexp: MOD min as base heavy | 38m 8.5s | 19m 2.1s | 12m 38.8s | N/A |
| Modexp: MOD min as EXP heavy | 25m 40.1s | 13m 14.3s | 9m 5.2s | 6m 47.1s |
| Modexp: MOD odd 1024b EXP 1024 | 14.66s | 13.14s | 14.64s | 15.75s |
| Modexp: MOD odd 128b EXP 1024 | 10m 28.4s | 5m 12.1s | 3m 32.8s | N/A |
| Modexp: MOD odd 256b EXP 1024 | 7m 11.6s | 3m 49.0s | 2m 35.9s | 2m 3.5s |
| Modexp: MOD odd 32b EXP 256 | 13m 6.3s | 6m 40.1s | 4m 31.6s | 3m 27.2s |
| Modexp: MOD odd 32b EXP 96 | 11m 56.3s | 5m 58.2s | 4m 8.2s | N/A |
| Modexp: MOD odd 32b EXP cover windows | 9m 30.8s | 4m 55.0s | 3m 18.8s | 2m 35.2s |
| Modexp: MOD odd 512b EXP 1024 | 14.92s | 12.44s | 12.19s | 17.58s |
| Modexp: MOD odd 64b EXP 512 | 11m 11.6s | 5m 42.2s | 3m 54.0s | 3m 0.6s |
| Modexp: MOD pawel 2 | 16m 25.1s | 8m 15.5s | 5m 39.7s | 4m 23.0s |
| Modexp: MOD pawel 3 | 13m 9.0s | 6m 51.5s | 4m 32.1s | 3m 33.7s |
| Modexp: MOD pawel 4 | 11m 55.4s | 6m 3.8s | 4m 10.0s | 3m 9.6s |
| Modexp: MOD vul common 1152n1 | 7m 11.0s | 3m 41.6s | 2m 32.9s | 1m 57.8s |
| Modexp: MOD vul common 1349n1 | 10m 23.8s | 5m 20.4s | 3m 35.3s | 2m 46.7s |
| Modexp: MOD vul common 1360n1 | 11m 33.9s | 5m 51.0s | 3m 57.7s | 3m 2.5s |
| Modexp: MOD vul common 1360n2 | 10m 7.1s | 5m 14.4s | 3m 39.0s | 2m 46.1s |
| Modexp: MOD vul common 200n1 | 5m 46.3s | 2m 58.0s | 2m 7.5s | 1m 38.6s |
| Modexp: MOD vul common 200n2 | 7m 18.2s | 3m 49.5s | 2m 38.3s | 2m 2.6s |
| Modexp: MOD vul common 200n3 | 7m 25.5s | 3m 48.3s | 2m 38.3s | 2m 3.8s |
| Modexp: MOD vul example 1 | N/A | 6m 34.6s | 4m 27.2s | 3m 28.4s |
| Modexp: MOD vul example 2 | 12m 16.0s | 6m 14.5s | 4m 14.5s | 3m 13.8s |
| Modexp: MOD vul guido 1 even | 8m 53.3s | 4m 48.2s | 3m 18.5s | 2m 35.1s |
| Modexp: MOD vul guido 2 even | 14m 56.1s | 8m 0.9s | 5m 21.5s | 4m 6.6s |
| Modexp: MOD vul guido 3 even | 24m 34.4s | 12m 27.8s | 8m 42.1s | 6m 36.0s |
| Modexp: MOD vul marius 1 even | 14m 15.9s | 7m 4.9s | 4m 55.5s | 3m 44.5s |
| Modexp: MOD vul nagydani 1 pow 0x10001 | 9m 17.7s | 4m 38.9s | 3m 15.4s | 2m 29.2s |
| Modexp: MOD vul nagydani 1 qube | 6m 47.3s | 3m 35.1s | 2m 28.9s | 1m 55.9s |
| Modexp: MOD vul nagydani 1 square | 6m 34.8s | 3m 20.8s | 2m 20.6s | 1m 49.3s |
| Modexp: MOD vul nagydani 2 pow 0x10001 | 9m 23.0s | 4m 49.7s | 3m 20.9s | 2m 34.4s |
| Modexp: MOD vul nagydani 2 qube | 20m 33.2s | 10m 29.0s | 7m 12.7s | 5m 27.3s |
| Modexp: MOD vul nagydani 2 square | 19m 19.3s | 9m 38.0s | 6m 42.8s | 5m 4.5s |
| Modexp: MOD vul nagydani 3 pow 0x10001 | 8m 58.4s | 4m 31.8s | 3m 9.3s | 2m 27.5s |
| Modexp: MOD vul nagydani 3 qube | 50m 38.9s | N/A | N/A | 12m 54.9s |
| Modexp: MOD vul nagydani 3 square | 44m 59.4s | 23m 38.2s | N/A | N/A |
| Modexp: MOD vul nagydani 4 pow 0x10001 | 8m 29.1s | 4m 26.3s | 2m 58.6s | 2m 20.5s |
| Modexp: MOD vul nagydani 4 qube | N/A | 28m 51.7s | N/A | 14m 40.6s |
| Modexp: MOD vul nagydani 4 square | 53m 5.8s | 25m 38.4s | 17m 49.3s | 13m 15.3s |
| Modexp: MOD vul nagydani 5 pow 0x10001 | 7m 34.3s | 4m 4.9s | 2m 46.4s | 2m 8.7s |
| Modexp: MOD vul nagydani 5 qube | N/A | 29m 18.8s | 19m 29.5s | 14m 41.2s |
| Modexp: MOD vul nagydani 5 square | 52m 21.2s | 26m 35.8s | N/A | 13m 26.4s |
| Modexp: MOD vul pawel 1 EXP heavy | 28m 11.4s | 14m 23.2s | 9m 45.3s | 7m 19.5s |
| Modexp: MOD vul pawel 2 EXP heavy | 15m 23.0s | 8m 7.3s | N/A | 4m 10.0s |
| Modexp: MOD vul pawel 3 EXP heavy | 12m 48.4s | 6m 32.3s | 4m 23.2s | 3m 22.3s |
| Modexp: MOD vul pawel 4 EXP heavy | 11m 2.6s | 5m 48.9s | 3m 57.8s | 3m 1.7s |
| Msize: mem size 0 | 45.10s | 31.44s | 26.53s | 23.40s |
| Msize: mem size 1000000 | 44.29s | 31.55s | 25.62s | 23.86s |
| Msize: mem size 100000 | 44.28s | 30.54s | 26.29s | 23.63s |
| Msize: mem size 1000 | 44.97s | 31.47s | 24.85s | 23.09s |
| Msize: mem size 1 | 44.39s | 32.39s | 25.23s | 23.21s |
| Precompile Fixed Cost: blake2f | 10m 46.7s | 5m 35.5s | 3m 49.6s | 2m 54.7s |
| Precompile Fixed Cost: bls12 fp to g1 | 1m 18.7s | 50.38s | 37.77s | 34.09s |
| Precompile Fixed Cost: bls12 fp to g2 | 1m 1.8s | 39.95s | 32.43s | 27.91s |
| Precompile Fixed Cost: bls12 g1ADD | 1m 0.9s | 39.73s | 31.11s | 28.08s |
| Precompile Fixed Cost: bls12 g1msm | 1m 26.9s | 54.12s | 41.37s | 34.96s |
| Precompile Fixed Cost: bls12 g2ADD | 1m 8.8s | 44.36s | 34.76s | 29.57s |
| Precompile Fixed Cost: bls12 g2msm | 1m 3.8s | 42.72s | 33.92s | 29.05s |
| Precompile Fixed Cost: bls12 pairing check | 1m 53.5s | 1m 7.2s | 49.67s | 42.43s |
| Precompile Fixed Cost: bn128 ADD | 1m 19.8s | 49.74s | 37.79s | 33.24s |
| Precompile Fixed Cost: bn128 ADD 1 2 | 1m 24.9s | 51.20s | 38.56s | 33.15s |
| Precompile Fixed Cost: bn128 ADD infinities | 52.83s | 35.82s | 27.62s | 25.86s |
| Precompile Fixed Cost: bn128 MUL | 4m 2.3s | 2m 13.8s | 1m 34.3s | 1m 15.7s |
| Precompile Fixed Cost: bn128 MUL 1 2 2 scalar | 19.37s | 16.69s | 13.23s | 20.80s |
| Precompile Fixed Cost: bn128 MUL 1 2 32 BYTE scalar | 3m 59.8s | N/A | 1m 32.4s | 1m 15.9s |
| Precompile Fixed Cost: bn128 MUL 32 BYTE coORd AND 2 scalar | 19.48s | 15.27s | 14.57s | 16.20s |
| Precompile Fixed Cost: bn128 MUL 32 BYTE coORd AND scalar | 3m 57.0s | 2m 8.8s | 1m 32.5s | 1m 15.1s |
| Precompile Fixed Cost: bn128 MUL infinities 2 scalar | 18.49s | 14.70s | 14.65s | 21.80s |
| Precompile Fixed Cost: bn128 MUL infinities 32 BYTE scalar | 2m 36.4s | 1m 26.8s | 1m 4.2s | 51.87s |
| Precompile Fixed Cost: bn128 one pairing | 3m 50.0s | 2m 6.2s | 1m 29.7s | 1m 12.0s |
| Precompile Fixed Cost: bn128 two pairings | 3m 49.8s | 2m 5.4s | 1m 29.7s | 1m 11.6s |
| Precompile Fixed Cost: bn128 two pairings empty | 15.03s | 12.66s | 12.63s | 14.42s |
| Precompile Fixed Cost: ecrecover | 3m 37.9s | 2m 2.6s | 1m 26.4s | 1m 12.0s |
| Precompile Fixed Cost: point evaluation | 4m 17.2s | 2m 19.9s | 1m 40.0s | 1m 21.2s |
| Precompile Only Data Input: IDENTITY | 31.02s | 24.43s | 20.76s | 19.57s |
| Precompile Only Data Input: RIPEMD: 160 | 24.21s | 21.00s | 17.36s | 17.93s |
| Precompile Only Data Input: SHA2: 256 | 19.96s | 15.73s | 16.72s | 19.55s |
| Push: PUSH0 | 38.26s | 28.45s | 23.41s | 21.41s |
| Push: PUSH10 | 40.80s | 29.11s | 23.46s | 22.74s |
| Push: PUSH11 | 41.85s | 29.84s | 25.12s | 23.02s |
| Push: PUSH12 | 42.78s | 30.03s | 24.08s | 21.45s |
| Push: PUSH13 | 42.01s | 30.59s | 24.02s | 22.83s |
| Push: PUSH14 | 45.31s | 31.95s | 25.98s | 23.43s |
| Push: PUSH15 | 46.62s | 31.99s | 26.07s | 23.81s |
| Push: PUSH16 | 45.95s | 30.71s | 24.55s | 22.30s |
| Push: PUSH17 | 46.09s | 32.07s | 26.17s | 23.65s |
| Push: PUSH18 | 47.52s | 32.69s | 26.76s | 22.92s |
| Push: PUSH19 | 49.13s | 33.00s | 27.12s | 24.17s |
| Push: PUSH1 | 31.49s | 25.84s | 22.16s | 19.60s |
| Push: PUSH20 | 47.21s | 32.40s | 26.20s | 24.19s |
| Push: PUSH21 | 49.98s | 33.70s | 27.81s | 25.09s |
| Push: PUSH22 | 51.71s | 34.20s | 28.34s | 24.81s |
| Push: PUSH23 | 53.43s | 32.22s | 28.13s | 24.34s |
| Push: PUSH24 | 54.09s | 35.37s | 29.08s | 24.46s |
| Push: PUSH25 | 55.69s | 37.61s | 28.96s | 25.92s |
| Push: PUSH26 | 56.95s | 38.03s | 29.49s | 26.54s |
| Push: PUSH27 | 57.92s | 38.72s | 30.59s | 25.99s |
| Push: PUSH28 | 57.43s | 38.76s | 30.68s | 25.91s |
| Push: PUSH29 | 1m 0.3s | 39.36s | 30.84s | 27.59s |
| Push: PUSH2 | 33.07s | 24.95s | 21.57s | 20.44s |
| Push: PUSH30 | 1m 1.1s | 39.39s | 31.31s | 27.51s |
| Push: PUSH31 | 1m 1.9s | 40.09s | 31.53s | 26.86s |
| Push: PUSH32 | 1m 5.5s | 40.80s | 32.52s | 28.39s |
| Push: PUSH3 | 34.94s | 26.73s | 20.99s | 21.33s |
| Push: PUSH4 | 34.19s | 25.54s | 23.62s | 21.23s |
| Push: PUSH5 | 36.20s | 27.31s | 23.16s | 21.43s |
| Push: PUSH6 | 37.55s | 28.34s | 23.73s | 22.24s |
| Push: PUSH7 | 39.14s | 28.41s | 23.71s | 21.81s |
| Push: PUSH8 | 37.63s | 28.27s | 24.03s | 22.08s |
| Push: PUSH9 | 39.56s | 28.42s | 23.78s | 21.55s |
| Return Revert: 1KiB of non: zero data: RETURN | 50.16s | 34.36s | 27.89s | 24.84s |
| Return Revert: 1KiB of non: zero data: REVERT | 52.35s | 35.32s | 27.44s | 25.74s |
| Return Revert: 1KiB of zero data: RETURN | 1m 1.0s | 39.61s | 30.09s | 27.60s |
| Return Revert: 1KiB of zero data: REVERT | 1m 3.8s | 40.57s | 33.37s | 27.89s |
| Return Revert: 1MiB of non: zero data: RETURN | 15.36s | 13.29s | 13.28s | 15.65s |
| Return Revert: 1MiB of non: zero data: REVERT | 15.26s | 13.05s | 14.54s | 15.73s |
| Return Revert: 1MiB of zero data: RETURN | 14.97s | 16.01s | 16.06s | 15.59s |
| Return Revert: 1MiB of zero data: REVERT | 15.01s | 12.85s | 10.91s | 17.37s |
| Return Revert: empty: RETURN | 1m 20.9s | 49.14s | 37.14s | 32.32s |
| Return Revert: empty: REVERT | 1m 28.4s | 53.63s | 40.79s | 34.45s |
| Returndatasize Nonzero: RETURNed size 0: RETURN data style ReturnDataStyle.IDENTITY | 31.92s | 24.32s | 21.37s | 20.77s |
| Returndatasize Nonzero: RETURNed size 0: RETURN data style ReturnDataStyle.RETURN | 32.53s | 25.27s | 22.46s | 19.96s |
| Returndatasize Nonzero: RETURNed size 0: RETURN data style ReturnDataStyle.REVERT | 32.37s | 25.04s | 19.46s | 20.29s |
| Returndatasize Nonzero: RETURNed size 1: RETURN data style ReturnDataStyle.IDENTITY | 31.86s | 25.17s | 21.82s | 19.45s |
| Returndatasize Nonzero: RETURNed size 1: RETURN data style ReturnDataStyle.RETURN | 32.48s | 23.93s | 21.97s | 19.92s |
| Returndatasize Nonzero: RETURNed size 1: RETURN data style ReturnDataStyle.REVERT | 32.06s | 25.59s | 21.79s | 18.61s |
| Returndatasize Zero: benchmark: gas: value 1M: blockchain test from state test | 32.26s | 25.15s | 21.79s | 20.75s |
| Shifts: shift right SAR | 1m 3.8s | 41.32s | 30.65s | 27.75s |
| Shifts: shift right SHR | 59.96s | 38.98s | 30.28s | 26.63s |
| Swap: SWAP10 | 1m 5.9s | 41.29s | 32.54s | 28.24s |
| Swap: SWAP11 | 1m 6.7s | 42.13s | 31.02s | 28.74s |
| Swap: SWAP12 | 1m 4.4s | 42.11s | 33.33s | 28.11s |
| Swap: SWAP13 | 1m 5.1s | 42.18s | 33.07s | 27.84s |
| Swap: SWAP14 | 1m 5.0s | 42.75s | 32.85s | 27.58s |
| Swap: SWAP15 | 1m 5.0s | 41.95s | 33.17s | 28.15s |
| Swap: SWAP16 | 1m 6.1s | 42.11s | 32.98s | 28.89s |
| Swap: SWAP1 | 1m 5.9s | 41.06s | 32.78s | 27.73s |
| Swap: SWAP2 | 1m 5.0s | 41.59s | 32.90s | 28.89s |
| Swap: SWAP3 | 1m 6.2s | 41.49s | 33.16s | 28.59s |
| Swap: SWAP4 | 1m 4.6s | 41.47s | 32.13s | 28.54s |
| Swap: SWAP5 | 1m 5.3s | 41.98s | 32.15s | 28.19s |
| Swap: SWAP6 | 1m 5.5s | 41.88s | 33.03s | 28.67s |
| Swap: SWAP7 | 1m 5.9s | 42.08s | 32.91s | 27.71s |
| Swap: SWAP8 | 1m 6.3s | 42.23s | 32.45s | 28.05s |
| Swap: SWAP9 | 1m 5.1s | 41.61s | 32.97s | 28.14s |
| Tload: val mut False: key mut False | 18.73s | 17.33s | 14.12s | 18.76s |
| Tload: val mut False: key mut True | 18.28s | 18.29s | 15.69s | 19.20s |
| Tload: val mut True: key mut False | 18.85s | 15.23s | 16.34s | 19.85s |
| Tload: val mut True: key mut True | 19.39s | 15.28s | 15.24s | 16.75s |
| Tstore: dense val mut False: key mut False | 28.91s | 23.27s | 19.98s | 19.00s |
| Tstore: dense val mut False: key mut True | 29.03s | 23.51s | 19.20s | 19.36s |
| Tstore: dense val mut True: key mut False | 45.60s | 31.39s | 24.92s | 23.23s |
| Tstore: dense val mut True: key mut True | 45.23s | 31.55s | 26.77s | 23.80s |
| Unop: ISZERO | 1m 8.9s | 44.01s | 34.65s | 29.81s |
| Unop: NOT | 32.65s | 24.97s | 22.16s | 20.06s |
| Zero Param: ADDRESS | 1m 5.0s | 41.42s | 31.57s | 27.15s |
| Zero Param: BASEFEE | 39.20s | 29.44s | 23.03s | 21.16s |
| Zero Param: BLOBBASEFEE | 50.47s | 34.17s | 26.86s | 24.69s |
| Zero Param: CALLER | 1m 2.8s | 41.97s | 33.57s | 27.48s |
| Zero Param: CHAINID | 39.94s | 29.39s | 24.33s | 22.55s |
| Zero Param: CODESIZE | 49.59s | 33.87s | 26.91s | 23.95s |
| Zero Param: COINBASE | 1m 3.5s | 41.55s | 31.53s | 27.26s |
| Zero Param: GASLIMIT | 39.53s | 25.96s | 24.38s | 22.02s |
| Zero Param: GASPRICE | 46.77s | 32.87s | 25.89s | 24.66s |
| Zero Param: GAS | 37.83s | 27.66s | 23.95s | 23.00s |
| Zero Param: NUMBER | 41.66s | 30.73s | 25.26s | 22.38s |
| Zero Param: ORIGIN | 1m 4.4s | 41.24s | 33.64s | 28.45s |
| Zero Param: PREVRANDAO | 2m 0.7s | 1m 11.1s | 51.75s | 43.02s |
| Zero Param: TIMESTAMP | 42.24s | 27.45s | 24.48s | 23.43s |

**Category Statistics:**

| GPUs | Mean | Median |
| --- | --- | --- |
| 1 GPU | 4m 51.8s | 58.24s |
| 2 GPUs | 2m 46.9s | 38.55s |
| 3 GPUs | 1m 43.1s | 30.36s |
| 4 GPUs | 1m 29.6s | 26.61s |

---

### Worst Memory.Py

**50 tests**

| Test Name | 1 GPU | 2 GPUs | 3 GPUs | 4 GPUs |
| --- | --- | --- | --- | --- |
| Calldatacopy: non zero data False: fixed src dst False: 0 BYTEs: CALL | 52.01s | 35.12s | 26.19s | 25.49s |
| Calldatacopy: non zero data False: fixed src dst False: 0 BYTEs: transaction | 52.32s | 35.09s | N/A | 24.79s |
| Calldatacopy: non zero data False: fixed src dst False: 100 BYTEs: CALL | 46.76s | 31.48s | 24.71s | 23.36s |
| Calldatacopy: non zero data False: fixed src dst False: 100 BYTEs: transaction | 43.63s | 30.10s | 25.96s | 22.88s |
| Calldatacopy: non zero data False: fixed src dst False: 10KiB: CALL | 27.33s | 22.37s | 18.49s | 18.44s |
| Calldatacopy: non zero data False: fixed src dst False: 10KiB: transaction | 19.51s | 18.54s | 14.59s | 18.16s |
| Calldatacopy: non zero data False: fixed src dst False: 1MiB: CALL | 14.16s | 12.01s | 13.86s | 17.23s |
| Calldatacopy: non zero data False: fixed src dst False: 1MiB: transaction | 14.71s | 12.78s | 10.70s | 19.38s |
| Calldatacopy: non zero data False: fixed src dst True: 0 BYTEs: CALL | 37.58s | 27.73s | 23.09s | 21.76s |
| Calldatacopy: non zero data False: fixed src dst True: 0 BYTEs: transaction | 36.43s | 26.99s | 21.41s | 21.43s |
| Calldatacopy: non zero data False: fixed src dst True: 100 BYTEs: CALL | 31.34s | 26.11s | 21.73s | 19.87s |
| Calldatacopy: non zero data False: fixed src dst True: 100 BYTEs: transaction | 30.07s | 21.69s | 20.59s | 19.76s |
| Calldatacopy: non zero data False: fixed src dst True: 10KiB: CALL | 22.96s | 20.75s | 17.04s | 20.18s |
| Calldatacopy: non zero data False: fixed src dst True: 10KiB: transaction | 18.25s | 16.40s | 13.60s | 18.91s |
| Calldatacopy: non zero data False: fixed src dst True: 1MiB: CALL | 14.14s | 12.09s | 12.38s | 15.72s |
| Calldatacopy: non zero data False: fixed src dst True: 1MiB: transaction | 14.95s | 15.62s | 10.67s | 14.34s |
| Calldatacopy: non zero data True: fixed src dst False: 100 BYTEs: CALL | 46.77s | 32.35s | 26.58s | 23.77s |
| Calldatacopy: non zero data True: fixed src dst False: 100 BYTEs: transaction | 45.78s | 31.78s | 26.66s | 23.88s |
| Calldatacopy: non zero data True: fixed src dst False: 10KiB: CALL | 26.61s | 22.06s | 18.47s | 17.46s |
| Calldatacopy: non zero data True: fixed src dst False: 10KiB: transaction | 25.46s | 22.07s | 17.07s | 18.26s |
| Calldatacopy: non zero data True: fixed src dst True: 100 BYTEs: CALL | 33.12s | 25.63s | 22.47s | 20.37s |
| Calldatacopy: non zero data True: fixed src dst True: 100 BYTEs: transaction | 32.80s | 25.34s | 22.33s | 19.57s |
| Calldatacopy: non zero data True: fixed src dst True: 10KiB: CALL | 22.44s | 20.58s | 18.16s | 20.84s |
| Calldatacopy: non zero data True: fixed src dst True: 10KiB: transaction | 21.66s | 18.16s | 16.66s | 19.68s |
| Codecopy: fixed src dst False: 0 BYTEs | 50.60s | 34.68s | 26.45s | 24.06s |
| Codecopy: fixed src dst False: 0.25x max code size | 31.76s | 22.50s | 21.29s | 20.21s |
| Codecopy: fixed src dst False: 0.50x max code size | 27.92s | 22.96s | 16.59s | 19.79s |
| Codecopy: fixed src dst False: 0.75x max code size | 28.07s | 23.85s | 20.66s | 18.95s |
| Codecopy: fixed src dst False: max code size | 28.16s | 22.70s | 21.01s | 18.84s |
| Codecopy: fixed src dst True: 0 BYTEs | 34.02s | 26.21s | 22.48s | 20.68s |
| Codecopy: fixed src dst True: 0.25x max code size | 23.08s | 21.20s | 17.43s | 19.80s |
| Codecopy: fixed src dst True: 0.50x max code size | 22.40s | 20.40s | 16.44s | 18.67s |
| Codecopy: fixed src dst True: 0.75x max code size | 22.68s | 20.95s | 17.17s | 20.25s |
| Codecopy: fixed src dst True: max code size | 22.26s | 21.43s | 16.18s | 20.32s |
| Mcopy: fixed src dst False: 0 BYTEs | 49.93s | 34.72s | 27.44s | 24.37s |
| Mcopy: fixed src dst False: 100 BYTEs | 49.04s | 34.05s | 27.16s | 24.31s |
| Mcopy: fixed src dst False: 10KiB | 39.42s | 27.76s | 23.78s | 22.23s |
| Mcopy: fixed src dst False: 1MiB | 15.39s | 16.18s | 12.36s | 16.17s |
| Mcopy: fixed src dst True: 0 BYTEs | 33.53s | 26.12s | 22.16s | 20.69s |
| Mcopy: fixed src dst True: 100 BYTEs | 34.54s | 26.18s | 21.88s | 21.22s |
| Mcopy: fixed src dst True: 10KiB | 29.38s | 24.94s | 20.22s | 19.58s |
| Mcopy: fixed src dst True: 1MiB | 15.27s | 12.83s | 13.87s | 15.53s |
| Returndatacopy: fixed dst False: 0 BYTEs | 50.29s | 33.83s | 26.78s | 23.65s |
| Returndatacopy: fixed dst False: 100 BYTEs | 41.94s | 27.78s | 24.32s | 22.60s |
| Returndatacopy: fixed dst False: 10KiB | 29.20s | 24.18s | 19.50s | 19.26s |
| Returndatacopy: fixed dst False: 1MiB | 16.02s | 13.55s | 15.52s | 19.69s |
| Returndatacopy: fixed dst True: 0 BYTEs | 39.76s | 28.79s | 24.18s | 22.20s |
| Returndatacopy: fixed dst True: 100 BYTEs | 33.20s | 25.00s | 21.66s | 20.39s |
| Returndatacopy: fixed dst True: 10KiB | 22.24s | 19.78s | 17.33s | 18.37s |
| Returndatacopy: fixed dst True: 1MiB | 15.88s | 16.22s | 13.47s | 15.52s |

**Category Statistics:**

| GPUs | Mean | Median |
| --- | --- | --- |
| 1 GPU | 30.74s | 29.29s |
| 2 GPUs | 23.83s | 23.41s |
| 3 GPUs | 19.81s | 20.59s |
| 4 GPUs | 20.26s | 20.02s |

---

### Worst Opcode.Py

**60 tests**

| Test Name | 1 GPU | 2 GPUs | 3 GPUs | 4 GPUs |
| --- | --- | --- | --- | --- |
| Log Opcodes: fixed offset False: non zero topic: 0 BYTEs data: LOG0 | 16.96s | 17.38s | 15.50s | 15.83s |
| Log Opcodes: fixed offset False: non zero topic: 0 BYTEs data: LOG1 | 16.75s | 13.94s | 13.68s | 20.08s |
| Log Opcodes: fixed offset False: non zero topic: 0 BYTEs data: LOG2 | 15.85s | 15.98s | 14.02s | 17.46s |
| Log Opcodes: fixed offset False: non zero topic: 0 BYTEs data: LOG3 | 16.05s | 13.37s | 12.12s | 17.03s |
| Log Opcodes: fixed offset False: non zero topic: 0 BYTEs data: LOG4 | 16.05s | 12.95s | 13.37s | 16.11s |
| Log Opcodes: fixed offset False: non zero topic: 1 MiB non zero data: LOG0 | 15.07s | 15.92s | 11.15s | 14.05s |
| Log Opcodes: fixed offset False: non zero topic: 1 MiB non zero data: LOG1 | 15.02s | 12.34s | 12.69s | 17.39s |
| Log Opcodes: fixed offset False: non zero topic: 1 MiB non zero data: LOG2 | 14.88s | 12.42s | 12.73s | 17.69s |
| Log Opcodes: fixed offset False: non zero topic: 1 MiB non zero data: LOG3 | 15.04s | 12.52s | 15.80s | 15.82s |
| Log Opcodes: fixed offset False: non zero topic: 1 MiB non zero data: LOG4 | 14.95s | 12.93s | 15.65s | 15.83s |
| Log Opcodes: fixed offset False: non zero topic: 1 MiB zeros data: LOG0 | 15.57s | 15.50s | 12.43s | 17.25s |
| Log Opcodes: fixed offset False: non zero topic: 1 MiB zeros data: LOG1 | 15.08s | 15.87s | 15.62s | 17.21s |
| Log Opcodes: fixed offset False: non zero topic: 1 MiB zeros data: LOG2 | 15.05s | 15.54s | 12.75s | 19.10s |
| Log Opcodes: fixed offset False: non zero topic: 1 MiB zeros data: LOG3 | 15.06s | 12.84s | 15.05s | 17.46s |
| Log Opcodes: fixed offset False: non zero topic: 1 MiB zeros data: LOG4 | 14.92s | 12.30s | 15.38s | 14.00s |
| Log Opcodes: fixed offset False: zeros topic: 0 BYTEs data: LOG0 | 17.59s | 14.73s | 15.80s | 15.56s |
| Log Opcodes: fixed offset False: zeros topic: 0 BYTEs data: LOG1 | 16.33s | 14.09s | 15.81s | 17.91s |
| Log Opcodes: fixed offset False: zeros topic: 0 BYTEs data: LOG2 | 15.74s | 13.55s | 13.02s | 16.88s |
| Log Opcodes: fixed offset False: zeros topic: 0 BYTEs data: LOG3 | 15.65s | 16.72s | 15.96s | 17.47s |
| Log Opcodes: fixed offset False: zeros topic: 0 BYTEs data: LOG4 | 16.06s | 13.48s | 13.89s | 15.98s |
| Log Opcodes: fixed offset False: zeros topic: 1 MiB non zero data: LOG0 | 14.64s | 12.13s | 12.77s | 18.93s |
| Log Opcodes: fixed offset False: zeros topic: 1 MiB non zero data: LOG1 | 14.76s | 12.53s | 12.45s | 15.95s |
| Log Opcodes: fixed offset False: zeros topic: 1 MiB non zero data: LOG2 | 14.93s | 16.02s | 13.30s | 16.01s |
| Log Opcodes: fixed offset False: zeros topic: 1 MiB non zero data: LOG3 | 14.93s | 12.28s | 14.06s | 17.39s |
| Log Opcodes: fixed offset False: zeros topic: 1 MiB non zero data: LOG4 | 15.26s | 12.57s | 12.51s | 17.39s |
| Log Opcodes: fixed offset False: zeros topic: 1 MiB zeros data: LOG0 | 15.48s | 12.28s | 15.50s | 21.07s |
| Log Opcodes: fixed offset False: zeros topic: 1 MiB zeros data: LOG1 | 14.71s | 12.45s | 11.88s | 15.70s |
| Log Opcodes: fixed offset False: zeros topic: 1 MiB zeros data: LOG2 | 15.06s | 15.54s | 12.52s | 14.10s |
| Log Opcodes: fixed offset False: zeros topic: 1 MiB zeros data: LOG3 | 14.51s | 15.76s | 15.43s | 16.24s |
| Log Opcodes: fixed offset False: zeros topic: 1 MiB zeros data: LOG4 | 15.04s | 15.77s | 12.52s | 15.95s |
| Log Opcodes: fixed offset True: non zero topic: 0 BYTEs data: LOG0 | 15.35s | 12.86s | 12.93s | 16.52s |
| Log Opcodes: fixed offset True: non zero topic: 0 BYTEs data: LOG1 | 16.08s | 13.29s | 13.68s | 14.80s |
| Log Opcodes: fixed offset True: non zero topic: 0 BYTEs data: LOG2 | 15.94s | 16.28s | 11.65s | 14.35s |
| Log Opcodes: fixed offset True: non zero topic: 0 BYTEs data: LOG3 | 15.88s | 13.65s | 12.19s | 16.51s |
| Log Opcodes: fixed offset True: non zero topic: 0 BYTEs data: LOG4 | 15.79s | 13.12s | 11.83s | 17.11s |
| Log Opcodes: fixed offset True: non zero topic: 1 MiB non zero data: LOG0 | 14.72s | 12.38s | 13.95s | 15.64s |
| Log Opcodes: fixed offset True: non zero topic: 1 MiB non zero data: LOG1 | 14.97s | 12.55s | 16.30s | 15.63s |
| Log Opcodes: fixed offset True: non zero topic: 1 MiB non zero data: LOG2 | 14.94s | 12.63s | 10.90s | 15.62s |
| Log Opcodes: fixed offset True: non zero topic: 1 MiB non zero data: LOG3 | 15.01s | 12.40s | 12.35s | 16.26s |
| Log Opcodes: fixed offset True: non zero topic: 1 MiB non zero data: LOG4 | 14.89s | 12.33s | 15.33s | 14.08s |
| Log Opcodes: fixed offset True: non zero topic: 1 MiB zeros data: LOG0 | 15.10s | 15.67s | 14.52s | 17.62s |
| Log Opcodes: fixed offset True: non zero topic: 1 MiB zeros data: LOG1 | 15.32s | 15.45s | 12.80s | 15.39s |
| Log Opcodes: fixed offset True: non zero topic: 1 MiB zeros data: LOG2 | 15.21s | 12.23s | 15.76s | 18.81s |
| Log Opcodes: fixed offset True: non zero topic: 1 MiB zeros data: LOG3 | 15.12s | 13.11s | 15.58s | 18.70s |
| Log Opcodes: fixed offset True: non zero topic: 1 MiB zeros data: LOG4 | 15.03s | 12.72s | 12.72s | 15.36s |
| Log Opcodes: fixed offset True: zeros topic: 0 BYTEs data: LOG0 | 16.09s | 13.37s | 13.59s | 16.47s |
| Log Opcodes: fixed offset True: zeros topic: 0 BYTEs data: LOG1 | 16.14s | 16.49s | 14.90s | 15.20s |
| Log Opcodes: fixed offset True: zeros topic: 0 BYTEs data: LOG2 | 15.99s | 13.04s | 16.11s | 16.46s |
| Log Opcodes: fixed offset True: zeros topic: 0 BYTEs data: LOG3 | 15.69s | 13.49s | 16.73s | 15.38s |
| Log Opcodes: fixed offset True: zeros topic: 0 BYTEs data: LOG4 | 15.96s | 14.00s | 11.75s | 15.69s |
| Log Opcodes: fixed offset True: zeros topic: 1 MiB non zero data: LOG0 | 14.98s | 12.29s | 11.01s | 15.44s |
| Log Opcodes: fixed offset True: zeros topic: 1 MiB non zero data: LOG1 | 15.05s | 12.93s | 12.70s | 15.76s |
| Log Opcodes: fixed offset True: zeros topic: 1 MiB non zero data: LOG2 | 15.03s | 12.65s | 12.76s | 16.32s |
| Log Opcodes: fixed offset True: zeros topic: 1 MiB non zero data: LOG3 | 15.17s | 15.62s | 12.60s | 17.86s |
| Log Opcodes: fixed offset True: zeros topic: 1 MiB non zero data: LOG4 | 14.99s | 13.67s | 12.76s | 15.52s |
| Log Opcodes: fixed offset True: zeros topic: 1 MiB zeros data: LOG0 | 14.88s | 12.69s | 12.34s | 16.18s |
| Log Opcodes: fixed offset True: zeros topic: 1 MiB zeros data: LOG1 | 15.11s | 16.02s | 15.62s | 15.56s |
| Log Opcodes: fixed offset True: zeros topic: 1 MiB zeros data: LOG2 | 15.05s | 13.35s | 12.70s | 15.34s |
| Log Opcodes: fixed offset True: zeros topic: 1 MiB zeros data: LOG3 | 15.04s | 12.51s | 12.23s | 16.06s |
| Log Opcodes: fixed offset True: zeros topic: 1 MiB zeros data: LOG4 | 15.00s | 13.31s | 12.65s | 13.83s |

**Category Statistics:**

| GPUs | Mean | Median |
| --- | --- | --- |
| 1 GPU | 15.38s | 15.07s |
| 2 GPUs | 13.80s | 13.30s |
| 3 GPUs | 13.64s | 12.98s |
| 4 GPUs | 16.41s | 16.08s |

---

### Worst Stateful Opcodes.Py

**44 tests**

| Test Name | 1 GPU | 2 GPUs | 3 GPUs | 4 GPUs |
| --- | --- | --- | --- | --- |
| Address State Cold: absent accounts False: BALANCE | 26.23s | 22.07s | 18.33s | 18.48s |
| Address State Cold: absent accounts True: BALANCE | 17.82s | 15.99s | 15.88s | 18.45s |
| Address State Warm: absent target False: BALANCE | 24.19s | 20.54s | 17.34s | 17.31s |
| Address State Warm: absent target False: CALLCODE | 1m 21.8s | 49.59s | 38.45s | 32.37s |
| Address State Warm: absent target False: CALL | 1m 19.9s | 50.32s | 38.63s | 32.13s |
| Address State Warm: absent target False: DELEGATECALL | 1m 10.1s | 44.44s | 34.67s | 28.29s |
| Address State Warm: absent target False: EXTCODEHASH | 28.28s | 23.18s | 20.02s | 19.09s |
| Address State Warm: absent target False: EXTCODESIZE | 25.80s | 22.08s | 19.66s | 18.73s |
| Address State Warm: absent target False: STATICCALL | 1m 22.2s | 49.33s | 37.40s | 32.88s |
| Address State Warm: absent target True: BALANCE | 23.22s | 20.82s | 17.52s | 18.26s |
| Address State Warm: absent target True: CALLCODE | 1m 17.6s | 48.55s | 37.78s | 31.70s |
| Address State Warm: absent target True: CALL | 1m 16.1s | 48.53s | 37.07s | 30.87s |
| Address State Warm: absent target True: DELEGATECALL | 1m 7.4s | 42.09s | 34.24s | 28.71s |
| Address State Warm: absent target True: EXTCODEHASH | 30.71s | 24.30s | 20.99s | 20.20s |
| Address State Warm: absent target True: EXTCODESIZE | 25.01s | 22.51s | 17.67s | 18.23s |
| Address State Warm: absent target True: STATICCALL | 1m 18.1s | 48.10s | 37.68s | 32.45s |
| Blockhash: benchmark: gas: value 1M: blockchain test | 41.24s | 30.48s | 24.88s | 22.71s |
| Extcodecopy Warm: 1KiB | 24.50s | 22.02s | 17.72s | 18.54s |
| Extcodecopy Warm: 512 | 24.80s | 22.01s | 16.17s | 22.39s |
| Extcodecopy Warm: 5KiB | 22.98s | 21.17s | 17.93s | 20.66s |
| Selfbalance: benchmark: gas: value 1M: blockchain test from state test | 2m 30.2s | 1m 27.3s | 1m 2.7s | 51.26s |
| Selfdestruct Created: value bearing False | 15.28s | 13.33s | 13.29s | 16.01s |
| Selfdestruct Created: value bearing True | 15.17s | 12.95s | 12.54s | 18.41s |
| Selfdestruct Existing: value bearing False | 20.80s | 19.37s | 17.48s | 18.60s |
| Selfdestruct Existing: value bearing True | 21.63s | 19.78s | 18.79s | 20.14s |
| Selfdestruct Initcode: value bearing False | 15.00s | 12.28s | 12.67s | 17.02s |
| Selfdestruct Initcode: value bearing True | 14.97s | 12.39s | 13.01s | 15.76s |
| Storage Access Cold: absent slots False: SSLOAD | 27.42s | 23.04s | 20.01s | 18.70s |
| Storage Access Cold: absent slots False: SSTORE new value, out of gas | 20.74s | 16.90s | 15.76s | 18.43s |
| Storage Access Cold: absent slots False: SSTORE new value, REVERT | 20.99s | 19.31s | 16.09s | 16.01s |
| Storage Access Cold: absent slots False: SSTORE new value | 21.04s | 19.94s | 14.39s | 17.94s |
| Storage Access Cold: absent slots False: SSTORE same value, out of gas | 26.97s | 22.82s | 19.16s | 17.74s |
| Storage Access Cold: absent slots False: SSTORE same value, REVERT | 27.69s | 23.58s | 18.45s | 17.97s |
| Storage Access Cold: absent slots False: SSTORE same value | 27.44s | 23.32s | 18.81s | N/A |
| Storage Access Cold: absent slots True: SSLOAD | 21.82s | 19.15s | 16.17s | 18.90s |
| Storage Access Cold: absent slots True: SSTORE new value, out of gas | 15.11s | 13.12s | 13.07s | 15.67s |
| Storage Access Cold: absent slots True: SSTORE new value, REVERT | 15.06s | 12.51s | 14.59s | 15.77s |
| Storage Access Cold: absent slots True: SSTORE new value | 15.18s | 15.84s | 15.96s | 15.48s |
| Storage Access Cold: absent slots True: SSTORE same value, out of gas | 15.06s | 15.69s | 15.81s | 16.38s |
| Storage Access Cold: absent slots True: SSTORE same value, REVERT | 15.02s | 13.10s | 13.05s | 16.14s |
| Storage Access Cold: absent slots True: SSTORE same value | 15.05s | 13.41s | 13.33s | 16.51s |
| Storage Access Warm: SLOAD | 26.51s | 23.55s | 16.41s | 19.13s |
| Storage Access Warm: SSTORE new value | 51.45s | 34.96s | 28.34s | 24.99s |
| Storage Access Warm: SSTORE same value | 43.73s | 29.79s | 25.15s | 22.42s |

**Category Statistics:**

| GPUs | Mean | Median |
| --- | --- | --- |
| 1 GPU | 36.08s | 24.91s |
| 2 GPUs | 26.49s | 22.04s |
| 3 GPUs | 21.93s | 17.83s |
| 4 GPUs | 21.58s | 18.60s |

---
