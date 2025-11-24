# RISC0 GPU Speedup & Efficiency Report

## Overview

This report shows speedup factors and parallel efficiency for each test.

- **Speedup**: How much faster compared to 1 GPU (higher is better)
- **Efficiency**: Speedup ÷ Number of GPUs (closer to 100% is better)

**Hardware:** AMD EPYC 7543 (32-Core), 472 GiB RAM, 4x NVIDIA RTX 5090

---

## Summary

| Configuration | Tests | Mean Speedup | Median Speedup | Mean Efficiency | Efficiency Range |
| --- | --- | --- | --- | --- | --- |
| 2 GPUs | 499 | **1.43x** | 1.39x | **71.7%** | 39.9% - 103.5% |
| 3 GPUs | 497 | **1.79x** | 1.66x | **59.6%** | 30.6% - 100.9% |
| 4 GPUs | 497 | **1.98x** | 1.81x | **49.5%** | 18.4% - 100.8% |

---

## Speedup & Efficiency by Test Category

### Worst Blocks.Py

**8 tests**

| Test Name | 2 GPUs | Eff | 3 GPUs | Eff | 4 GPUs | Eff |
| --- | --- | --- | --- | --- | --- | --- |
| Block Full Access List And Data: benchmark: gas: value 1M: blockchain test from state test | **1.23x** | 62% | **1.29x** | 43% | **0.94x** | 24% |
| Block Full Data: False | **1.16x** | 58% | **1.24x** | 41% | **0.89x** | 22% |
| Block Full Data: True | **1.23x** | 61% | **0.99x** | 33% | **0.91x** | 23% |
| Block Full Of Ether Transfers: a to a | **1.43x** | 71% | **1.74x** | 58% | **1.91x** | 48% |
| Block Full Of Ether Transfers: a to b | **1.39x** | 70% | **1.64x** | 55% | **1.87x** | 47% |
| Block Full Of Ether Transfers: a to diff acc | **1.40x** | 70% | **1.75x** | 58% | **1.92x** | 48% |
| Block Full Of Ether Transfers: diff acc to b | **1.47x** | 73% | **1.77x** | 59% | **2.04x** | 51% |
| Block Full Of Ether Transfers: diff acc to diff acc | **1.46x** | 73% | **1.82x** | 61% | **2.06x** | 52% |

**Category Statistics:**

| Configuration | Mean Speedup | Mean Efficiency |
| --- | --- | --- |
| 2 GPUs | 1.35x | 67.3% |
| 3 GPUs | 1.53x | 51.0% |
| 4 GPUs | 1.57x | 39.2% |

---

### Worst Bytecode.Py

**35 tests**

| Test Name | 2 GPUs | Eff | 3 GPUs | Eff | 4 GPUs | Eff |
| --- | --- | --- | --- | --- | --- | --- |
| Bytecode Single Opcode: CALLCODE | **1.86x** | 93% | **2.65x** | 88% | **3.22x** | 80% |
| Bytecode Single Opcode: CALL | **1.83x** | 91% | **2.59x** | 86% | **3.18x** | 79% |
| Bytecode Single Opcode: DELEGATECALL | **1.83x** | 91% | **2.59x** | 86% | **3.22x** | 80% |
| Bytecode Single Opcode: EXTCODECOPY | **1.85x** | 93% | **2.55x** | 85% | **3.20x** | 80% |
| Bytecode Single Opcode: EXTCODEHASH | **1.83x** | 92% | **2.57x** | 86% | **3.21x** | 80% |
| Bytecode Single Opcode: EXTCODESIZE | **1.85x** | 92% | **2.60x** | 87% | **3.23x** | 81% |
| Bytecode Single Opcode: STATICCALL | **1.82x** | 91% | **2.59x** | 86% | **3.18x** | 80% |
| Create: 0 BYTEs with value: CREATE2 | **0.97x** | 49% | **1.25x** | 42% | **0.96x** | 24% |
| Create: 0 BYTEs with value: CREATE | **0.98x** | 49% | **1.20x** | 40% | **1.07x** | 27% |
| Create: 0 BYTEs without value: CREATE2 | **1.09x** | 55% | **1.19x** | 40% | **0.97x** | 24% |
| Create: 0 BYTEs without value: CREATE | **1.20x** | 60% | **1.41x** | 47% | **1.08x** | 27% |
| Create: 0.25x max code size with non: zero data: CREATE2 | **0.95x** | 48% | **1.23x** | 41% | **0.85x** | 21% |
| Create: 0.25x max code size with non: zero data: CREATE | **1.15x** | 58% | **1.17x** | 39% | **0.92x** | 23% |
| Create: 0.25x max code size with zero data: CREATE2 | **1.16x** | 58% | **1.14x** | 38% | **0.92x** | 23% |
| Create: 0.25x max code size with zero data: CREATE | **0.94x** | 47% | **1.14x** | 38% | **1.07x** | 27% |
| Create: 0.50x max code size with non: zero data: CREATE2 | **1.20x** | 60% | **1.18x** | 39% | **1.05x** | 26% |
| Create: 0.50x max code size with non: zero data: CREATE | **1.00x** | 50% | **1.17x** | 39% | **0.94x** | 24% |
| Create: 0.50x max code size with zero data: CREATE2 | **0.80x** | 40% | **1.23x** | 41% | **0.96x** | 24% |
| Create: 0.50x max code size with zero data: CREATE | **1.21x** | 61% | **1.14x** | 38% | **0.97x** | 24% |
| Create: 0.75x max code size with non: zero data: CREATE2 | **1.22x** | 61% | **1.03x** | 34% | **1.02x** | 26% |
| Create: 0.75x max code size with non: zero data: CREATE | **1.14x** | 57% | **1.33x** | 44% | **0.90x** | 22% |
| Create: 0.75x max code size with zero data: CREATE2 | **1.14x** | 57% | **1.10x** | 37% | **0.90x** | 22% |
| Create: 0.75x max code size with zero data: CREATE | **1.14x** | 57% | **1.22x** | 41% | **0.99x** | 25% |
| Create: max code size with non: zero data: CREATE2 | **0.86x** | 43% | **1.03x** | 34% | **0.99x** | 25% |
| Create: max code size with non: zero data: CREATE | **1.01x** | 51% | **1.22x** | 41% | **0.97x** | 24% |
| Create: max code size with zero data: CREATE2 | **1.22x** | 61% | **1.24x** | 41% | **0.88x** | 22% |
| Create: max code size with zero data: CREATE | **1.24x** | 62% | **0.97x** | 32% | **0.96x** | 24% |
| Creates Collisions: CREATE2 | **1.14x** | 57% | **1.14x** | 38% | **0.91x** | 23% |
| Creates Collisions: CREATE | **1.17x** | 59% | **1.18x** | 39% | **0.91x** | 23% |
| Initcode Jumpdest Analysis: 00 | **1.22x** | 61% | **1.49x** | 50% | **1.57x** | 39% |
| Initcode Jumpdest Analysis: 5b | **1.48x** | 74% | **1.57x** | 52% | **1.75x** | 44% |
| Initcode Jumpdest Analysis: 605b5b | **1.21x** | 61% | **1.43x** | 48% | **1.51x** | 38% |
| Initcode Jumpdest Analysis: 605b | **1.11x** | 55% | **1.48x** | 49% | **1.36x** | 34% |
| Initcode Jumpdest Analysis: 615b5b5b | **1.19x** | 60% | **1.35x** | 45% | **1.42x** | 36% |
| Initcode Jumpdest Analysis: 615b5b | **1.13x** | 57% | **1.31x** | 44% | **1.36x** | 34% |

**Category Statistics:**

| Configuration | Mean Speedup | Mean Efficiency |
| --- | --- | --- |
| 2 GPUs | 1.26x | 63.1% |
| 3 GPUs | 1.50x | 50.1% |
| 4 GPUs | 1.50x | 37.6% |

---

### Worst Compute.Py

**311 tests**

| Test Name | 2 GPUs | Eff | 3 GPUs | Eff | 4 GPUs | Eff |
| --- | --- | --- | --- | --- | --- | --- |
| Amortized Bn128 Pairings: benchmark: gas: value 1M: blockchain test from state test | **1.83x** | 91% | **2.60x** | 87% | **3.21x** | 80% |
| Empty Block: benchmark: gas: value 1M: blockchain test | **0.98x** | 49% | **1.23x** | 41% | **0.80x** | 20% |
| Binop Simple: ADD: | **1.34x** | 67% | **1.59x** | 53% | **1.80x** | 45% |
| Binop Simple: AND: | **1.34x** | 67% | **1.57x** | 52% | **1.67x** | 42% |
| Binop Simple: BYTE: | **1.34x** | 67% | **1.58x** | 53% | **1.65x** | 41% |
| Binop Simple: DIV: 0 | **1.84x** | 92% | **2.63x** | 88% | **3.20x** | 80% |
| Binop Simple: DIV: 1 | **1.81x** | 91% | **2.51x** | 84% | **3.17x** | 79% |
| Binop Simple: EQ: | **1.63x** | 82% | **2.12x** | 71% | **2.46x** | 62% |
| Binop Simple: EXP: | N/A | N/A | **2.36x** | 79% | **2.75x** | 69% |
| Binop Simple: GT: | **1.35x** | 67% | **1.60x** | 53% | **1.71x** | 43% |
| Binop Simple: LT: | **1.36x** | 68% | **1.52x** | 51% | **1.67x** | 42% |
| Binop Simple: MOD: | **1.49x** | 74% | **1.89x** | 63% | **2.16x** | 54% |
| Binop Simple: MUL: | **1.54x** | 77% | **1.91x** | 64% | **2.24x** | 56% |
| Binop Simple: OR: | **1.31x** | 66% | **1.47x** | 49% | **1.68x** | 42% |
| Binop Simple: SAR: | **1.58x** | 79% | **2.00x** | 67% | **2.30x** | 58% |
| Binop Simple: SDIV: 0 | **1.86x** | 93% | **2.60x** | 87% | **3.31x** | 83% |
| Binop Simple: SDIV: 1 | **1.91x** | 96% | **2.71x** | 90% | **3.40x** | 85% |
| Binop Simple: SGT: | **1.56x** | 78% | **2.03x** | 68% | **2.35x** | 59% |
| Binop Simple: SHL: | **1.52x** | 76% | **1.93x** | 64% | **2.26x** | 57% |
| Binop Simple: SHR: | **1.72x** | 86% | **2.06x** | 69% | **2.24x** | 56% |
| Binop Simple: SIGNEXTEND: | **1.69x** | 84% | **1.92x** | 64% | **2.13x** | 53% |
| Binop Simple: SLT: | **1.44x** | 72% | **1.65x** | 55% | **1.84x** | 46% |
| Binop Simple: SMOD: | **1.62x** | 81% | **2.08x** | 69% | **2.42x** | 61% |
| Binop Simple: SUB: | **1.40x** | 70% | **1.67x** | 56% | **1.81x** | 45% |
| Binop Simple: XOR: | **1.26x** | 63% | **1.56x** | 52% | **1.60x** | 40% |
| Blobhash: no blobs | **1.34x** | 67% | **1.61x** | 54% | **1.71x** | 43% |
| Blobhash: one blob AND accessed | **1.49x** | 75% | **1.81x** | 60% | **2.06x** | 51% |
| Blobhash: one blob but access non: existent index | **1.36x** | 68% | **1.59x** | 53% | **1.79x** | 45% |
| Blobhash: six blobs, access latest | **1.48x** | 74% | **1.85x** | 62% | **2.11x** | 53% |
| Calldataload: empty | **1.55x** | 77% | **1.98x** | 66% | **2.25x** | 56% |
| Calldataload: one: loop | **1.63x** | 81% | **2.19x** | 73% | **2.44x** | 61% |
| Calldataload: zero: loop | **1.60x** | 80% | **2.11x** | 70% | **2.43x** | 61% |
| Calldatasize: CALLdata lenGTh 0 | **1.36x** | 68% | **1.52x** | 51% | **1.72x** | 43% |
| Calldatasize: CALLdata lenGTh 10000 | **1.26x** | 63% | **1.61x** | 54% | **1.64x** | 41% |
| Calldatasize: CALLdata lenGTh 1000 | **1.33x** | 66% | **1.67x** | 56% | **1.71x** | 43% |
| Callvalue: from ORigin False: non zero value False | **1.26x** | 63% | **1.44x** | 48% | **1.57x** | 39% |
| Callvalue: from ORigin False: non zero value True | **1.28x** | 64% | **1.44x** | 48% | **1.59x** | 40% |
| Callvalue: from ORigin True: non zero value False | **1.33x** | 67% | **1.59x** | 53% | **1.62x** | 40% |
| Callvalue: from ORigin True: non zero value True | **1.28x** | 64% | **1.46x** | 49% | **1.62x** | 40% |
| Dup: DUP10 | **1.26x** | 63% | **1.46x** | 49% | **1.56x** | 39% |
| Dup: DUP11 | **1.32x** | 66% | **1.56x** | 52% | **1.60x** | 40% |
| Dup: DUP12 | **1.33x** | 66% | **1.49x** | 50% | **1.71x** | 43% |
| Dup: DUP13 | **1.31x** | 66% | **1.57x** | 52% | **1.62x** | 41% |
| Dup: DUP14 | **1.30x** | 65% | **1.53x** | 51% | **1.58x** | 40% |
| Dup: DUP15 | **1.31x** | 65% | **1.61x** | 54% | **1.57x** | 39% |
| Dup: DUP16 | **1.31x** | 65% | **1.49x** | 50% | **1.64x** | 41% |
| Dup: DUP1 | **1.29x** | 65% | **1.49x** | 50% | **1.66x** | 41% |
| Dup: DUP2 | **1.33x** | 66% | **1.56x** | 52% | **1.64x** | 41% |
| Dup: DUP3 | **1.34x** | 67% | **1.49x** | 50% | **1.62x** | 40% |
| Dup: DUP4 | **1.31x** | 66% | **1.55x** | 52% | **1.65x** | 41% |
| Dup: DUP5 | **1.29x** | 64% | **1.51x** | 50% | **1.59x** | 40% |
| Dup: DUP6 | **1.28x** | 64% | **1.61x** | 54% | **1.59x** | 40% |
| Dup: DUP7 | **1.30x** | 65% | **1.54x** | 51% | **1.62x** | 40% |
| Dup: DUP8 | **1.29x** | 64% | **1.41x** | 47% | **1.45x** | 36% |
| Dup: DUP9 | **1.30x** | 65% | **1.53x** | 51% | **1.65x** | 41% |
| Jumpdests: benchmark: gas: value 1M: blockchain test from state test | **1.32x** | 66% | **1.49x** | 50% | **1.70x** | 43% |
| Jumpi Fallthrough: benchmark: gas: value 1M: blockchain test from state test | **1.40x** | 70% | **1.67x** | 56% | **1.91x** | 48% |
| Jumpis: benchmark: gas: value 1M: blockchain test from state test | **1.37x** | 68% | **1.52x** | 51% | **1.47x** | 37% |
| Jumps: benchmark: gas: value 1M: blockchain test from state test | **1.13x** | 57% | **1.42x** | 47% | **1.32x** | 33% |
| Keccak: benchmark: gas: value 1M: blockchain test from state test | **1.70x** | 85% | **2.29x** | 76% | **2.74x** | 68% |
| Memory Access: big memORy EXPansion False: offset initialized False: offset 0: MLOAD | **1.45x** | 73% | **1.73x** | 58% | **1.94x** | 48% |
| Memory Access: big memORy EXPansion False: offset initialized False: offset 0: MSTORE8 | **1.40x** | 70% | **1.64x** | 55% | **1.92x** | 48% |
| Memory Access: big memORy EXPansion False: offset initialized False: offset 0: MSTORE | **1.51x** | 76% | **1.91x** | 64% | **2.21x** | 55% |
| Memory Access: big memORy EXPansion False: offset initialized False: offset 1: MLOAD | **1.45x** | 73% | **1.88x** | 63% | **2.01x** | 50% |
| Memory Access: big memORy EXPansion False: offset initialized False: offset 1: MSTORE8 | **1.41x** | 71% | **1.68x** | 56% | **1.85x** | 46% |
| Memory Access: big memORy EXPansion False: offset initialized False: offset 1: MSTORE | **1.59x** | 80% | **1.95x** | 65% | **2.21x** | 55% |
| Memory Access: big memORy EXPansion False: offset initialized False: offset 31: MLOAD | **1.42x** | 71% | **1.76x** | 59% | **1.97x** | 49% |
| Memory Access: big memORy EXPansion False: offset initialized False: offset 31: MSTORE8 | **1.37x** | 68% | **1.76x** | 59% | **1.74x** | 44% |
| Memory Access: big memORy EXPansion False: offset initialized False: offset 31: MSTORE | **1.51x** | 75% | **1.90x** | 63% | **2.12x** | 53% |
| Memory Access: big memORy EXPansion False: offset initialized True: offset 0: MLOAD | **1.48x** | 74% | **1.78x** | 59% | **2.01x** | 50% |
| Memory Access: big memORy EXPansion False: offset initialized True: offset 0: MSTORE8 | **1.38x** | 69% | **1.80x** | 60% | **1.81x** | 45% |
| Memory Access: big memORy EXPansion False: offset initialized True: offset 0: MSTORE | **1.57x** | 78% | **1.98x** | 66% | **2.18x** | 54% |
| Memory Access: big memORy EXPansion False: offset initialized True: offset 1: MLOAD | **1.43x** | 72% | **1.73x** | 58% | **2.02x** | 50% |
| Memory Access: big memORy EXPansion False: offset initialized True: offset 1: MSTORE8 | **1.36x** | 68% | **1.62x** | 54% | **1.75x** | 44% |
| Memory Access: big memORy EXPansion False: offset initialized True: offset 1: MSTORE | **1.57x** | 78% | **2.08x** | 69% | **2.26x** | 56% |
| Memory Access: big memORy EXPansion False: offset initialized True: offset 31: MLOAD | **1.51x** | 76% | **1.77x** | 59% | **2.07x** | 52% |
| Memory Access: big memORy EXPansion False: offset initialized True: offset 31: MSTORE8 | **1.36x** | 68% | **1.60x** | 53% | **1.72x** | 43% |
| Memory Access: big memORy EXPansion False: offset initialized True: offset 31: MSTORE | **1.51x** | 75% | **1.86x** | 62% | **2.16x** | 54% |
| Memory Access: big memORy EXPansion True: offset initialized False: offset 0: MLOAD | **1.50x** | 75% | **1.75x** | 58% | **1.98x** | 50% |
| Memory Access: big memORy EXPansion True: offset initialized False: offset 0: MSTORE8 | **1.43x** | 71% | **1.82x** | 61% | **1.81x** | 45% |
| Memory Access: big memORy EXPansion True: offset initialized False: offset 0: MSTORE | **1.50x** | 75% | **1.80x** | 60% | **2.04x** | 51% |
| Memory Access: big memORy EXPansion True: offset initialized False: offset 1: MLOAD | **1.46x** | 73% | **1.78x** | 59% | **1.98x** | 49% |
| Memory Access: big memORy EXPansion True: offset initialized False: offset 1: MSTORE8 | **1.36x** | 68% | **1.63x** | 54% | **1.88x** | 47% |
| Memory Access: big memORy EXPansion True: offset initialized False: offset 1: MSTORE | **1.61x** | 81% | **1.90x** | 63% | **2.16x** | 54% |
| Memory Access: big memORy EXPansion True: offset initialized False: offset 31: MLOAD | **1.44x** | 72% | **1.82x** | 61% | **2.04x** | 51% |
| Memory Access: big memORy EXPansion True: offset initialized False: offset 31: MSTORE8 | **1.37x** | 68% | **1.63x** | 54% | **1.75x** | 44% |
| Memory Access: big memORy EXPansion True: offset initialized False: offset 31: MSTORE | **1.50x** | 75% | **1.97x** | 66% | **2.16x** | 54% |
| Memory Access: big memORy EXPansion True: offset initialized True: offset 0: MLOAD | **1.41x** | 71% | **1.74x** | 58% | **1.89x** | 47% |
| Memory Access: big memORy EXPansion True: offset initialized True: offset 0: MSTORE8 | **1.37x** | 68% | **1.63x** | 54% | **1.87x** | 47% |
| Memory Access: big memORy EXPansion True: offset initialized True: offset 0: MSTORE | **1.50x** | 75% | **1.81x** | 60% | **2.15x** | 54% |
| Memory Access: big memORy EXPansion True: offset initialized True: offset 1: MLOAD | **1.42x** | 71% | **1.85x** | 62% | **1.93x** | 48% |
| Memory Access: big memORy EXPansion True: offset initialized True: offset 1: MSTORE8 | **1.34x** | 67% | **1.60x** | 53% | **1.78x** | 45% |
| Memory Access: big memORy EXPansion True: offset initialized True: offset 1: MSTORE | **1.51x** | 76% | **1.99x** | 66% | **2.17x** | 54% |
| Memory Access: big memORy EXPansion True: offset initialized True: offset 31: MLOAD | **1.43x** | 71% | **1.84x** | 61% | **2.01x** | 50% |
| Memory Access: big memORy EXPansion True: offset initialized True: offset 31: MSTORE8 | **1.34x** | 67% | **1.66x** | 55% | **1.75x** | 44% |
| Memory Access: big memORy EXPansion True: offset initialized True: offset 31: MSTORE | **1.50x** | 75% | **1.92x** | 64% | **2.25x** | 56% |
| Mod: op MOD: MOD bits 127 | **1.80x** | 90% | **2.52x** | 84% | **3.11x** | 78% |
| Mod: op MOD: MOD bits 191 | **1.83x** | 92% | **2.61x** | 87% | **3.32x** | 83% |
| Mod: op MOD: MOD bits 255 | **1.89x** | 95% | **2.55x** | 85% | **3.18x** | 79% |
| Mod: op MOD: MOD bits 63 | **1.73x** | 86% | **2.33x** | 78% | **2.82x** | 71% |
| Mod: op SMOD: MOD bits 127 | **1.86x** | 93% | **2.60x** | 87% | **3.19x** | 80% |
| Mod: op SMOD: MOD bits 191 | **1.87x** | 94% | **2.67x** | 89% | **3.31x** | 83% |
| Mod: op SMOD: MOD bits 255 | **1.83x** | 91% | **2.57x** | 86% | **3.15x** | 79% |
| Mod: op SMOD: MOD bits 63 | **1.78x** | 89% | **2.48x** | 83% | **2.89x** | 72% |
| Modarith: op ADDMOD: MOD bits 127 | **1.82x** | 91% | **2.60x** | 87% | **3.21x** | 80% |
| Modarith: op ADDMOD: MOD bits 191 | **1.84x** | 92% | **2.63x** | 88% | N/A | N/A |
| Modarith: op ADDMOD: MOD bits 255 | **1.85x** | 93% | **2.70x** | 90% | **3.24x** | 81% |
| Modarith: op ADDMOD: MOD bits 63 | N/A | N/A | **2.44x** | 81% | **2.96x** | 74% |
| Modarith: op MULMOD: MOD bits 127 | **1.88x** | 94% | **2.66x** | 89% | **3.34x** | 84% |
| Modarith: op MULMOD: MOD bits 191 | **1.90x** | 95% | **2.77x** | 92% | **3.52x** | 88% |
| Modarith: op MULMOD: MOD bits 255 | **1.96x** | 98% | **2.77x** | 92% | **3.52x** | 88% |
| Modarith: op MULMOD: MOD bits 63 | **1.83x** | 92% | **2.54x** | 85% | **3.17x** | 79% |
| Modexp: MOD 1045 gas base heavy | **1.96x** | 98% | **2.87x** | 96% | **3.75x** | 94% |
| Modexp: MOD 1360 gas BALANCEd | **1.98x** | 99% | **2.88x** | 96% | **3.72x** | 93% |
| Modexp: MOD 400 gas EXP heavy | **1.92x** | 96% | **2.84x** | 95% | **3.70x** | 93% |
| Modexp: MOD 408 gas BALANCEd | **1.97x** | 98% | **2.90x** | 97% | **3.71x** | 93% |
| Modexp: MOD 408 gas base heavy | N/A | N/A | N/A | N/A | N/A | N/A |
| Modexp: MOD 600 as BALANCEd | **1.89x** | 95% | **2.86x** | 95% | **3.66x** | 92% |
| Modexp: MOD 600 gas EXP heavy | **1.97x** | 99% | **2.93x** | 98% | **3.83x** | 96% |
| Modexp: MOD 616 gas base heavy | **1.97x** | 99% | **2.89x** | 96% | **3.89x** | 97% |
| Modexp: MOD 677 gas base heavy | **1.95x** | 97% | **2.85x** | 95% | **3.69x** | 92% |
| Modexp: MOD 765 gas EXP heavy | **2.02x** | 101% | **3.00x** | 100% | **3.89x** | 97% |
| Modexp: MOD 767 gas BALANCEd | **1.92x** | 96% | **2.81x** | 94% | **3.60x** | 90% |
| Modexp: MOD 800 gas base heavy | **1.93x** | 97% | **2.91x** | 97% | **3.84x** | 96% |
| Modexp: MOD 800 gas EXP heavy | **1.93x** | 97% | **2.84x** | 95% | **3.75x** | 94% |
| Modexp: MOD 852 gas EXP heavy | **1.90x** | 95% | **2.89x** | 96% | **3.77x** | 94% |
| Modexp: MOD 867 gas base heavy | **1.92x** | 96% | N/A | N/A | **3.81x** | 95% |
| Modexp: MOD 996 gas BALANCEd | N/A | N/A | N/A | N/A | N/A | N/A |
| Modexp: MOD even 1024b EXP 1024 | **1.22x** | 61% | **1.18x** | 39% | **0.90x** | 23% |
| Modexp: MOD even 128b EXP 1024 | **1.91x** | 95% | **2.80x** | 93% | **3.61x** | 90% |
| Modexp: MOD even 16b EXP 320 | **2.01x** | 100% | **2.95x** | 98% | **3.85x** | 96% |
| Modexp: MOD even 24b EXP 168 | **1.99x** | 100% | **2.94x** | 98% | **3.82x** | 96% |
| Modexp: MOD even 256b EXP 1024 | **1.92x** | 96% | **2.79x** | 93% | **3.53x** | 88% |
| Modexp: MOD even 32b EXP 256 | **1.97x** | 98% | **2.97x** | 99% | **3.77x** | 94% |
| Modexp: MOD even 32b EXP 40 | **1.92x** | 96% | **2.83x** | 94% | **3.61x** | 90% |
| Modexp: MOD even 32b EXP 96 | **1.98x** | 99% | **2.95x** | 98% | **3.86x** | 96% |
| Modexp: MOD even 512b EXP 1024 | **0.97x** | 49% | **1.07x** | 36% | **0.83x** | 21% |
| Modexp: MOD even 64b EXP 512 | **1.99x** | 99% | **2.84x** | 95% | **3.70x** | 93% |
| Modexp: MOD even 8b EXP 896 | **2.06x** | 103% | **3.03x** | 101% | **4.03x** | 101% |
| Modexp: MOD EXP 208 gas BALANCEd | **1.92x** | 96% | **2.84x** | 95% | **3.62x** | 90% |
| Modexp: MOD EXP 215 gas EXP heavy | **2.03x** | 102% | **3.00x** | 100% | **3.91x** | 98% |
| Modexp: MOD EXP 298 gas EXP heavy | **2.00x** | 100% | **2.94x** | 98% | **3.87x** | 97% |
| Modexp: MOD min as BALANCEd | **1.93x** | 96% | **2.84x** | 95% | **3.71x** | 93% |
| Modexp: MOD min as base heavy | **2.00x** | 100% | **3.02x** | 101% | N/A | N/A |
| Modexp: MOD min as EXP heavy | **1.94x** | 97% | **2.82x** | 94% | **3.78x** | 95% |
| Modexp: MOD odd 1024b EXP 1024 | **1.12x** | 56% | **1.00x** | 33% | **0.93x** | 23% |
| Modexp: MOD odd 128b EXP 1024 | **2.01x** | 101% | **2.95x** | 98% | N/A | N/A |
| Modexp: MOD odd 256b EXP 1024 | **1.88x** | 94% | **2.77x** | 92% | **3.49x** | 87% |
| Modexp: MOD odd 32b EXP 256 | **1.97x** | 98% | **2.90x** | 97% | **3.80x** | 95% |
| Modexp: MOD odd 32b EXP 96 | **2.00x** | 100% | **2.89x** | 96% | N/A | N/A |
| Modexp: MOD odd 32b EXP cover windows | **1.94x** | 97% | **2.87x** | 96% | **3.68x** | 92% |
| Modexp: MOD odd 512b EXP 1024 | **1.20x** | 60% | **1.22x** | 41% | **0.85x** | 21% |
| Modexp: MOD odd 64b EXP 512 | **1.96x** | 98% | **2.87x** | 96% | **3.72x** | 93% |
| Modexp: MOD pawel 2 | **1.99x** | 99% | **2.90x** | 97% | **3.74x** | 94% |
| Modexp: MOD pawel 3 | **1.92x** | 96% | **2.90x** | 97% | **3.69x** | 92% |
| Modexp: MOD pawel 4 | **1.97x** | 98% | **2.86x** | 95% | **3.77x** | 94% |
| Modexp: MOD vul common 1152n1 | **1.95x** | 97% | **2.82x** | 94% | **3.66x** | 92% |
| Modexp: MOD vul common 1349n1 | **1.95x** | 97% | **2.90x** | 97% | **3.74x** | 94% |
| Modexp: MOD vul common 1360n1 | **1.98x** | 99% | **2.92x** | 97% | **3.80x** | 95% |
| Modexp: MOD vul common 1360n2 | **1.93x** | 97% | **2.77x** | 92% | **3.66x** | 91% |
| Modexp: MOD vul common 200n1 | **1.95x** | 97% | **2.72x** | 91% | **3.51x** | 88% |
| Modexp: MOD vul common 200n2 | **1.91x** | 95% | **2.77x** | 92% | **3.57x** | 89% |
| Modexp: MOD vul common 200n3 | **1.95x** | 98% | **2.81x** | 94% | **3.60x** | 90% |
| Modexp: MOD vul example 1 | N/A | N/A | N/A | N/A | N/A | N/A |
| Modexp: MOD vul example 2 | **1.97x** | 98% | **2.89x** | 96% | **3.80x** | 95% |
| Modexp: MOD vul guido 1 even | **1.85x** | 93% | **2.69x** | 90% | **3.44x** | 86% |
| Modexp: MOD vul guido 2 even | **1.86x** | 93% | **2.79x** | 93% | **3.63x** | 91% |
| Modexp: MOD vul guido 3 even | **1.97x** | 99% | **2.82x** | 94% | **3.72x** | 93% |
| Modexp: MOD vul marius 1 even | **2.01x** | 101% | **2.90x** | 97% | **3.81x** | 95% |
| Modexp: MOD vul nagydani 1 pow 0x10001 | **2.00x** | 100% | **2.85x** | 95% | **3.74x** | 93% |
| Modexp: MOD vul nagydani 1 qube | **1.89x** | 95% | **2.74x** | 91% | **3.51x** | 88% |
| Modexp: MOD vul nagydani 1 square | **1.97x** | 98% | **2.81x** | 94% | **3.61x** | 90% |
| Modexp: MOD vul nagydani 2 pow 0x10001 | **1.94x** | 97% | **2.80x** | 93% | **3.65x** | 91% |
| Modexp: MOD vul nagydani 2 qube | **1.96x** | 98% | **2.85x** | 95% | **3.77x** | 94% |
| Modexp: MOD vul nagydani 2 square | **2.01x** | 100% | **2.88x** | 96% | **3.81x** | 95% |
| Modexp: MOD vul nagydani 3 pow 0x10001 | **1.98x** | 99% | **2.84x** | 95% | **3.65x** | 91% |
| Modexp: MOD vul nagydani 3 qube | N/A | N/A | N/A | N/A | **3.92x** | 98% |
| Modexp: MOD vul nagydani 3 square | **1.90x** | 95% | N/A | N/A | N/A | N/A |
| Modexp: MOD vul nagydani 4 pow 0x10001 | **1.91x** | 96% | **2.85x** | 95% | **3.62x** | 91% |
| Modexp: MOD vul nagydani 4 qube | N/A | N/A | N/A | N/A | N/A | N/A |
| Modexp: MOD vul nagydani 4 square | **2.07x** | 104% | **2.98x** | 99% | **4.01x** | 100% |
| Modexp: MOD vul nagydani 5 pow 0x10001 | **1.86x** | 93% | **2.73x** | 91% | **3.53x** | 88% |
| Modexp: MOD vul nagydani 5 qube | N/A | N/A | N/A | N/A | N/A | N/A |
| Modexp: MOD vul nagydani 5 square | **1.97x** | 98% | N/A | N/A | **3.90x** | 97% |
| Modexp: MOD vul pawel 1 EXP heavy | **1.96x** | 98% | **2.89x** | 96% | **3.85x** | 96% |
| Modexp: MOD vul pawel 2 EXP heavy | **1.89x** | 95% | N/A | N/A | **3.69x** | 92% |
| Modexp: MOD vul pawel 3 EXP heavy | **1.96x** | 98% | **2.92x** | 97% | **3.80x** | 95% |
| Modexp: MOD vul pawel 4 EXP heavy | **1.90x** | 95% | **2.79x** | 93% | **3.65x** | 91% |
| Msize: mem size 0 | **1.43x** | 72% | **1.70x** | 57% | **1.93x** | 48% |
| Msize: mem size 1000000 | **1.40x** | 70% | **1.73x** | 58% | **1.86x** | 46% |
| Msize: mem size 100000 | **1.45x** | 73% | **1.68x** | 56% | **1.87x** | 47% |
| Msize: mem size 1000 | **1.43x** | 71% | **1.81x** | 60% | **1.95x** | 49% |
| Msize: mem size 1 | **1.37x** | 69% | **1.76x** | 59% | **1.91x** | 48% |
| Precompile Fixed Cost: blake2f | **1.93x** | 96% | **2.82x** | 94% | **3.70x** | 93% |
| Precompile Fixed Cost: bls12 fp to g1 | **1.56x** | 78% | **2.08x** | 69% | **2.31x** | 58% |
| Precompile Fixed Cost: bls12 fp to g2 | **1.55x** | 77% | **1.91x** | 64% | **2.21x** | 55% |
| Precompile Fixed Cost: bls12 g1ADD | **1.53x** | 77% | **1.96x** | 65% | **2.17x** | 54% |
| Precompile Fixed Cost: bls12 g1msm | **1.60x** | 80% | **2.10x** | 70% | **2.48x** | 62% |
| Precompile Fixed Cost: bls12 g2ADD | **1.55x** | 78% | **1.98x** | 66% | **2.33x** | 58% |
| Precompile Fixed Cost: bls12 g2msm | **1.49x** | 75% | **1.88x** | 63% | **2.20x** | 55% |
| Precompile Fixed Cost: bls12 pairing check | **1.69x** | 85% | **2.29x** | 76% | **2.68x** | 67% |
| Precompile Fixed Cost: bn128 ADD | **1.60x** | 80% | **2.11x** | 70% | **2.40x** | 60% |
| Precompile Fixed Cost: bn128 ADD 1 2 | **1.66x** | 83% | **2.20x** | 73% | **2.56x** | 64% |
| Precompile Fixed Cost: bn128 ADD infinities | **1.47x** | 74% | **1.91x** | 64% | **2.04x** | 51% |
| Precompile Fixed Cost: bn128 MUL | **1.81x** | 91% | **2.57x** | 86% | **3.20x** | 80% |
| Precompile Fixed Cost: bn128 MUL 1 2 2 scalar | **1.16x** | 58% | **1.46x** | 49% | **0.93x** | 23% |
| Precompile Fixed Cost: bn128 MUL 1 2 32 BYTE scalar | N/A | N/A | **2.59x** | 86% | **3.16x** | 79% |
| Precompile Fixed Cost: bn128 MUL 32 BYTE coORd AND 2 scalar | **1.28x** | 64% | **1.34x** | 45% | **1.20x** | 30% |
| Precompile Fixed Cost: bn128 MUL 32 BYTE coORd AND scalar | **1.84x** | 92% | **2.56x** | 85% | **3.16x** | 79% |
| Precompile Fixed Cost: bn128 MUL infinities 2 scalar | **1.26x** | 63% | **1.26x** | 42% | **0.85x** | 21% |
| Precompile Fixed Cost: bn128 MUL infinities 32 BYTE scalar | **1.80x** | 90% | **2.43x** | 81% | **3.02x** | 75% |
| Precompile Fixed Cost: bn128 one pairing | **1.82x** | 91% | **2.56x** | 85% | **3.20x** | 80% |
| Precompile Fixed Cost: bn128 two pairings | **1.83x** | 92% | **2.56x** | 85% | **3.21x** | 80% |
| Precompile Fixed Cost: bn128 two pairings empty | **1.19x** | 59% | **1.19x** | 40% | **1.04x** | 26% |
| Precompile Fixed Cost: ecrecover | **1.78x** | 89% | **2.52x** | 84% | **3.03x** | 76% |
| Precompile Fixed Cost: point evaluation | **1.84x** | 92% | **2.57x** | 86% | **3.17x** | 79% |
| Precompile Only Data Input: IDENTITY | **1.27x** | 63% | **1.49x** | 50% | **1.58x** | 40% |
| Precompile Only Data Input: RIPEMD: 160 | **1.15x** | 58% | **1.39x** | 46% | **1.35x** | 34% |
| Precompile Only Data Input: SHA2: 256 | **1.27x** | 63% | **1.19x** | 40% | **1.02x** | 26% |
| Push: PUSH0 | **1.34x** | 67% | **1.63x** | 54% | **1.79x** | 45% |
| Push: PUSH10 | **1.40x** | 70% | **1.74x** | 58% | **1.79x** | 45% |
| Push: PUSH11 | **1.40x** | 70% | **1.67x** | 56% | **1.82x** | 45% |
| Push: PUSH12 | **1.42x** | 71% | **1.78x** | 59% | **1.99x** | 50% |
| Push: PUSH13 | **1.37x** | 69% | **1.75x** | 58% | **1.84x** | 46% |
| Push: PUSH14 | **1.42x** | 71% | **1.74x** | 58% | **1.93x** | 48% |
| Push: PUSH15 | **1.46x** | 73% | **1.79x** | 60% | **1.96x** | 49% |
| Push: PUSH16 | **1.50x** | 75% | **1.87x** | 62% | **2.06x** | 52% |
| Push: PUSH17 | **1.44x** | 72% | **1.76x** | 59% | **1.95x** | 49% |
| Push: PUSH18 | **1.45x** | 73% | **1.78x** | 59% | **2.07x** | 52% |
| Push: PUSH19 | **1.49x** | 74% | **1.81x** | 60% | **2.03x** | 51% |
| Push: PUSH1 | **1.22x** | 61% | **1.42x** | 47% | **1.61x** | 40% |
| Push: PUSH20 | **1.46x** | 73% | **1.80x** | 60% | **1.95x** | 49% |
| Push: PUSH21 | **1.48x** | 74% | **1.80x** | 60% | **1.99x** | 50% |
| Push: PUSH22 | **1.51x** | 76% | **1.82x** | 61% | **2.08x** | 52% |
| Push: PUSH23 | **1.66x** | 83% | **1.90x** | 63% | **2.20x** | 55% |
| Push: PUSH24 | **1.53x** | 76% | **1.86x** | 62% | **2.21x** | 55% |
| Push: PUSH25 | **1.48x** | 74% | **1.92x** | 64% | **2.15x** | 54% |
| Push: PUSH26 | **1.50x** | 75% | **1.93x** | 64% | **2.15x** | 54% |
| Push: PUSH27 | **1.50x** | 75% | **1.89x** | 63% | **2.23x** | 56% |
| Push: PUSH28 | **1.48x** | 74% | **1.87x** | 62% | **2.22x** | 55% |
| Push: PUSH29 | **1.53x** | 77% | **1.96x** | 65% | **2.19x** | 55% |
| Push: PUSH2 | **1.33x** | 66% | **1.53x** | 51% | **1.62x** | 40% |
| Push: PUSH30 | **1.55x** | 78% | **1.95x** | 65% | **2.22x** | 56% |
| Push: PUSH31 | **1.54x** | 77% | **1.96x** | 65% | **2.30x** | 58% |
| Push: PUSH32 | **1.61x** | 80% | **2.01x** | 67% | **2.31x** | 58% |
| Push: PUSH3 | **1.31x** | 65% | **1.66x** | 55% | **1.64x** | 41% |
| Push: PUSH4 | **1.34x** | 67% | **1.45x** | 48% | **1.61x** | 40% |
| Push: PUSH5 | **1.33x** | 66% | **1.56x** | 52% | **1.69x** | 42% |
| Push: PUSH6 | **1.33x** | 66% | **1.58x** | 53% | **1.69x** | 42% |
| Push: PUSH7 | **1.38x** | 69% | **1.65x** | 55% | **1.79x** | 45% |
| Push: PUSH8 | **1.33x** | 67% | **1.57x** | 52% | **1.70x** | 43% |
| Push: PUSH9 | **1.39x** | 70% | **1.66x** | 55% | **1.84x** | 46% |
| Return Revert: 1KiB of non: zero data: RETURN | **1.46x** | 73% | **1.80x** | 60% | **2.02x** | 50% |
| Return Revert: 1KiB of non: zero data: REVERT | **1.48x** | 74% | **1.91x** | 64% | **2.03x** | 51% |
| Return Revert: 1KiB of zero data: RETURN | **1.54x** | 77% | **2.03x** | 68% | **2.21x** | 55% |
| Return Revert: 1KiB of zero data: REVERT | **1.57x** | 79% | **1.91x** | 64% | **2.29x** | 57% |
| Return Revert: 1MiB of non: zero data: RETURN | **1.16x** | 58% | **1.16x** | 39% | **0.98x** | 25% |
| Return Revert: 1MiB of non: zero data: REVERT | **1.17x** | 58% | **1.05x** | 35% | **0.97x** | 24% |
| Return Revert: 1MiB of zero data: RETURN | **0.93x** | 47% | **0.93x** | 31% | **0.96x** | 24% |
| Return Revert: 1MiB of zero data: REVERT | **1.17x** | 58% | **1.38x** | 46% | **0.86x** | 22% |
| Return Revert: empty: RETURN | **1.65x** | 82% | **2.18x** | 73% | **2.50x** | 63% |
| Return Revert: empty: REVERT | **1.65x** | 82% | **2.17x** | 72% | **2.57x** | 64% |
| Returndatasize Nonzero: RETURNed size 0: RETURN data style ReturnDataStyle.IDENTITY | **1.31x** | 66% | **1.49x** | 50% | **1.54x** | 38% |
| Returndatasize Nonzero: RETURNed size 0: RETURN data style ReturnDataStyle.RETURN | **1.29x** | 64% | **1.45x** | 48% | **1.63x** | 41% |
| Returndatasize Nonzero: RETURNed size 0: RETURN data style ReturnDataStyle.REVERT | **1.29x** | 65% | **1.66x** | 55% | **1.60x** | 40% |
| Returndatasize Nonzero: RETURNed size 1: RETURN data style ReturnDataStyle.IDENTITY | **1.27x** | 63% | **1.46x** | 49% | **1.64x** | 41% |
| Returndatasize Nonzero: RETURNed size 1: RETURN data style ReturnDataStyle.RETURN | **1.36x** | 68% | **1.48x** | 49% | **1.63x** | 41% |
| Returndatasize Nonzero: RETURNed size 1: RETURN data style ReturnDataStyle.REVERT | **1.25x** | 63% | **1.47x** | 49% | **1.72x** | 43% |
| Returndatasize Zero: benchmark: gas: value 1M: blockchain test from state test | **1.28x** | 64% | **1.48x** | 49% | **1.55x** | 39% |
| Shifts: shift right SAR | **1.54x** | 77% | **2.08x** | 69% | **2.30x** | 57% |
| Shifts: shift right SHR | **1.54x** | 77% | **1.98x** | 66% | **2.25x** | 56% |
| Swap: SWAP10 | **1.60x** | 80% | **2.03x** | 68% | **2.33x** | 58% |
| Swap: SWAP11 | **1.58x** | 79% | **2.15x** | 72% | **2.32x** | 58% |
| Swap: SWAP12 | **1.53x** | 76% | **1.93x** | 64% | **2.29x** | 57% |
| Swap: SWAP13 | **1.54x** | 77% | **1.97x** | 66% | **2.34x** | 58% |
| Swap: SWAP14 | **1.52x** | 76% | **1.98x** | 66% | **2.36x** | 59% |
| Swap: SWAP15 | **1.55x** | 78% | **1.96x** | 65% | **2.31x** | 58% |
| Swap: SWAP16 | **1.57x** | 78% | **2.00x** | 67% | **2.29x** | 57% |
| Swap: SWAP1 | **1.60x** | 80% | **2.01x** | 67% | **2.38x** | 59% |
| Swap: SWAP2 | **1.56x** | 78% | **1.98x** | 66% | **2.25x** | 56% |
| Swap: SWAP3 | **1.60x** | 80% | **2.00x** | 67% | **2.32x** | 58% |
| Swap: SWAP4 | **1.56x** | 78% | **2.01x** | 67% | **2.26x** | 57% |
| Swap: SWAP5 | **1.56x** | 78% | **2.03x** | 68% | **2.32x** | 58% |
| Swap: SWAP6 | **1.56x** | 78% | **1.98x** | 66% | **2.28x** | 57% |
| Swap: SWAP7 | **1.57x** | 78% | **2.00x** | 67% | **2.38x** | 59% |
| Swap: SWAP8 | **1.57x** | 79% | **2.04x** | 68% | **2.37x** | 59% |
| Swap: SWAP9 | **1.56x** | 78% | **1.97x** | 66% | **2.31x** | 58% |
| Tload: val mut False: key mut False | **1.08x** | 54% | **1.33x** | 44% | **1.00x** | 25% |
| Tload: val mut False: key mut True | **1.00x** | 50% | **1.17x** | 39% | **0.95x** | 24% |
| Tload: val mut True: key mut False | **1.24x** | 62% | **1.15x** | 38% | **0.95x** | 24% |
| Tload: val mut True: key mut True | **1.27x** | 63% | **1.27x** | 42% | **1.16x** | 29% |
| Tstore: dense val mut False: key mut False | **1.24x** | 62% | **1.45x** | 48% | **1.52x** | 38% |
| Tstore: dense val mut False: key mut True | **1.23x** | 62% | **1.51x** | 50% | **1.50x** | 37% |
| Tstore: dense val mut True: key mut False | **1.45x** | 73% | **1.83x** | 61% | **1.96x** | 49% |
| Tstore: dense val mut True: key mut True | **1.43x** | 72% | **1.69x** | 56% | **1.90x** | 48% |
| Unop: ISZERO | **1.57x** | 78% | **1.99x** | 66% | **2.31x** | 58% |
| Unop: NOT | **1.31x** | 65% | **1.47x** | 49% | **1.63x** | 41% |
| Zero Param: ADDRESS | **1.57x** | 79% | **2.06x** | 69% | **2.40x** | 60% |
| Zero Param: BASEFEE | **1.33x** | 67% | **1.70x** | 57% | **1.85x** | 46% |
| Zero Param: BLOBBASEFEE | **1.48x** | 74% | **1.88x** | 63% | **2.04x** | 51% |
| Zero Param: CALLER | **1.50x** | 75% | **1.87x** | 62% | **2.29x** | 57% |
| Zero Param: CHAINID | **1.36x** | 68% | **1.64x** | 55% | **1.77x** | 44% |
| Zero Param: CODESIZE | **1.46x** | 73% | **1.84x** | 61% | **2.07x** | 52% |
| Zero Param: COINBASE | **1.53x** | 76% | **2.01x** | 67% | **2.33x** | 58% |
| Zero Param: GASLIMIT | **1.52x** | 76% | **1.62x** | 54% | **1.80x** | 45% |
| Zero Param: GASPRICE | **1.42x** | 71% | **1.81x** | 60% | **1.90x** | 47% |
| Zero Param: GAS | **1.37x** | 68% | **1.58x** | 53% | **1.64x** | 41% |
| Zero Param: NUMBER | **1.36x** | 68% | **1.65x** | 55% | **1.86x** | 47% |
| Zero Param: ORIGIN | **1.56x** | 78% | **1.91x** | 64% | **2.26x** | 57% |
| Zero Param: PREVRANDAO | **1.70x** | 85% | **2.33x** | 78% | **2.80x** | 70% |
| Zero Param: TIMESTAMP | **1.54x** | 77% | **1.73x** | 58% | **1.80x** | 45% |

**Category Statistics:**

| Configuration | Mean Speedup | Mean Efficiency |
| --- | --- | --- |
| 2 GPUs | 1.57x | 78.7% |
| 3 GPUs | 2.05x | 68.3% |
| 4 GPUs | 2.40x | 60.1% |

---

### Worst Memory.Py

**50 tests**

| Test Name | 2 GPUs | Eff | 3 GPUs | Eff | 4 GPUs | Eff |
| --- | --- | --- | --- | --- | --- | --- |
| Calldatacopy: non zero data False: fixed src dst False: 0 BYTEs: CALL | **1.48x** | 74% | **1.99x** | 66% | **2.04x** | 51% |
| Calldatacopy: non zero data False: fixed src dst False: 0 BYTEs: transaction | **1.49x** | 75% | N/A | N/A | **2.11x** | 53% |
| Calldatacopy: non zero data False: fixed src dst False: 100 BYTEs: CALL | **1.49x** | 74% | **1.89x** | 63% | **2.00x** | 50% |
| Calldatacopy: non zero data False: fixed src dst False: 100 BYTEs: transaction | **1.45x** | 72% | **1.68x** | 56% | **1.91x** | 48% |
| Calldatacopy: non zero data False: fixed src dst False: 10KiB: CALL | **1.22x** | 61% | **1.48x** | 49% | **1.48x** | 37% |
| Calldatacopy: non zero data False: fixed src dst False: 10KiB: transaction | **1.05x** | 53% | **1.34x** | 45% | **1.07x** | 27% |
| Calldatacopy: non zero data False: fixed src dst False: 1MiB: CALL | **1.18x** | 59% | **1.02x** | 34% | **0.82x** | 21% |
| Calldatacopy: non zero data False: fixed src dst False: 1MiB: transaction | **1.15x** | 58% | **1.37x** | 46% | **0.76x** | 19% |
| Calldatacopy: non zero data False: fixed src dst True: 0 BYTEs: CALL | **1.36x** | 68% | **1.63x** | 54% | **1.73x** | 43% |
| Calldatacopy: non zero data False: fixed src dst True: 0 BYTEs: transaction | **1.35x** | 67% | **1.70x** | 57% | **1.70x** | 42% |
| Calldatacopy: non zero data False: fixed src dst True: 100 BYTEs: CALL | **1.20x** | 60% | **1.44x** | 48% | **1.58x** | 39% |
| Calldatacopy: non zero data False: fixed src dst True: 100 BYTEs: transaction | **1.39x** | 69% | **1.46x** | 49% | **1.52x** | 38% |
| Calldatacopy: non zero data False: fixed src dst True: 10KiB: CALL | **1.11x** | 55% | **1.35x** | 45% | **1.14x** | 28% |
| Calldatacopy: non zero data False: fixed src dst True: 10KiB: transaction | **1.11x** | 56% | **1.34x** | 45% | **0.96x** | 24% |
| Calldatacopy: non zero data False: fixed src dst True: 1MiB: CALL | **1.17x** | 58% | **1.14x** | 38% | **0.90x** | 22% |
| Calldatacopy: non zero data False: fixed src dst True: 1MiB: transaction | **0.96x** | 48% | **1.40x** | 47% | **1.04x** | 26% |
| Calldatacopy: non zero data True: fixed src dst False: 100 BYTEs: CALL | **1.45x** | 72% | **1.76x** | 59% | **1.97x** | 49% |
| Calldatacopy: non zero data True: fixed src dst False: 100 BYTEs: transaction | **1.44x** | 72% | **1.72x** | 57% | **1.92x** | 48% |
| Calldatacopy: non zero data True: fixed src dst False: 10KiB: CALL | **1.21x** | 60% | **1.44x** | 48% | **1.52x** | 38% |
| Calldatacopy: non zero data True: fixed src dst False: 10KiB: transaction | **1.15x** | 58% | **1.49x** | 50% | **1.39x** | 35% |
| Calldatacopy: non zero data True: fixed src dst True: 100 BYTEs: CALL | **1.29x** | 65% | **1.47x** | 49% | **1.63x** | 41% |
| Calldatacopy: non zero data True: fixed src dst True: 100 BYTEs: transaction | **1.29x** | 65% | **1.47x** | 49% | **1.68x** | 42% |
| Calldatacopy: non zero data True: fixed src dst True: 10KiB: CALL | **1.09x** | 55% | **1.24x** | 41% | **1.08x** | 27% |
| Calldatacopy: non zero data True: fixed src dst True: 10KiB: transaction | **1.19x** | 60% | **1.30x** | 43% | **1.10x** | 28% |
| Codecopy: fixed src dst False: 0 BYTEs | **1.46x** | 73% | **1.91x** | 64% | **2.10x** | 53% |
| Codecopy: fixed src dst False: 0.25x max code size | **1.41x** | 71% | **1.49x** | 50% | **1.57x** | 39% |
| Codecopy: fixed src dst False: 0.50x max code size | **1.22x** | 61% | **1.68x** | 56% | **1.41x** | 35% |
| Codecopy: fixed src dst False: 0.75x max code size | **1.18x** | 59% | **1.36x** | 45% | **1.48x** | 37% |
| Codecopy: fixed src dst False: max code size | **1.24x** | 62% | **1.34x** | 45% | **1.50x** | 37% |
| Codecopy: fixed src dst True: 0 BYTEs | **1.30x** | 65% | **1.51x** | 50% | **1.65x** | 41% |
| Codecopy: fixed src dst True: 0.25x max code size | **1.09x** | 54% | **1.32x** | 44% | **1.17x** | 29% |
| Codecopy: fixed src dst True: 0.50x max code size | **1.10x** | 55% | **1.36x** | 45% | **1.20x** | 30% |
| Codecopy: fixed src dst True: 0.75x max code size | **1.08x** | 54% | **1.32x** | 44% | **1.12x** | 28% |
| Codecopy: fixed src dst True: max code size | **1.04x** | 52% | **1.38x** | 46% | **1.10x** | 27% |
| Mcopy: fixed src dst False: 0 BYTEs | **1.44x** | 72% | **1.82x** | 61% | **2.05x** | 51% |
| Mcopy: fixed src dst False: 100 BYTEs | **1.44x** | 72% | **1.81x** | 60% | **2.02x** | 50% |
| Mcopy: fixed src dst False: 10KiB | **1.42x** | 71% | **1.66x** | 55% | **1.77x** | 44% |
| Mcopy: fixed src dst False: 1MiB | **0.95x** | 48% | **1.25x** | 42% | **0.95x** | 24% |
| Mcopy: fixed src dst True: 0 BYTEs | **1.28x** | 64% | **1.51x** | 50% | **1.62x** | 41% |
| Mcopy: fixed src dst True: 100 BYTEs | **1.32x** | 66% | **1.58x** | 53% | **1.63x** | 41% |
| Mcopy: fixed src dst True: 10KiB | **1.18x** | 59% | **1.45x** | 48% | **1.50x** | 38% |
| Mcopy: fixed src dst True: 1MiB | **1.19x** | 60% | **1.10x** | 37% | **0.98x** | 25% |
| Returndatacopy: fixed dst False: 0 BYTEs | **1.49x** | 74% | **1.88x** | 63% | **2.13x** | 53% |
| Returndatacopy: fixed dst False: 100 BYTEs | **1.51x** | 75% | **1.72x** | 57% | **1.86x** | 46% |
| Returndatacopy: fixed dst False: 10KiB | **1.21x** | 60% | **1.50x** | 50% | **1.52x** | 38% |
| Returndatacopy: fixed dst False: 1MiB | **1.18x** | 59% | **1.03x** | 34% | **0.81x** | 20% |
| Returndatacopy: fixed dst True: 0 BYTEs | **1.38x** | 69% | **1.64x** | 55% | **1.79x** | 45% |
| Returndatacopy: fixed dst True: 100 BYTEs | **1.33x** | 66% | **1.53x** | 51% | **1.63x** | 41% |
| Returndatacopy: fixed dst True: 10KiB | **1.12x** | 56% | **1.28x** | 43% | **1.21x** | 30% |
| Returndatacopy: fixed dst True: 1MiB | **0.98x** | 49% | **1.18x** | 39% | **1.02x** | 26% |

**Category Statistics:**

| Configuration | Mean Speedup | Mean Efficiency |
| --- | --- | --- |
| 2 GPUs | 1.26x | 62.8% |
| 3 GPUs | 1.48x | 49.5% |
| 4 GPUs | 1.48x | 36.9% |

---

### Worst Opcode.Py

**60 tests**

| Test Name | 2 GPUs | Eff | 3 GPUs | Eff | 4 GPUs | Eff |
| --- | --- | --- | --- | --- | --- | --- |
| Log Opcodes: fixed offset False: non zero topic: 0 BYTEs data: LOG0 | **0.98x** | 49% | **1.09x** | 36% | **1.07x** | 27% |
| Log Opcodes: fixed offset False: non zero topic: 0 BYTEs data: LOG1 | **1.20x** | 60% | **1.22x** | 41% | **0.83x** | 21% |
| Log Opcodes: fixed offset False: non zero topic: 0 BYTEs data: LOG2 | **0.99x** | 50% | **1.13x** | 38% | **0.91x** | 23% |
| Log Opcodes: fixed offset False: non zero topic: 0 BYTEs data: LOG3 | **1.20x** | 60% | **1.32x** | 44% | **0.94x** | 24% |
| Log Opcodes: fixed offset False: non zero topic: 0 BYTEs data: LOG4 | **1.24x** | 62% | **1.20x** | 40% | **1.00x** | 25% |
| Log Opcodes: fixed offset False: non zero topic: 1 MiB non zero data: LOG0 | **0.95x** | 47% | **1.35x** | 45% | **1.07x** | 27% |
| Log Opcodes: fixed offset False: non zero topic: 1 MiB non zero data: LOG1 | **1.22x** | 61% | **1.18x** | 39% | **0.86x** | 22% |
| Log Opcodes: fixed offset False: non zero topic: 1 MiB non zero data: LOG2 | **1.20x** | 60% | **1.17x** | 39% | **0.84x** | 21% |
| Log Opcodes: fixed offset False: non zero topic: 1 MiB non zero data: LOG3 | **1.20x** | 60% | **0.95x** | 32% | **0.95x** | 24% |
| Log Opcodes: fixed offset False: non zero topic: 1 MiB non zero data: LOG4 | **1.16x** | 58% | **0.96x** | 32% | **0.94x** | 24% |
| Log Opcodes: fixed offset False: non zero topic: 1 MiB zeros data: LOG0 | **1.00x** | 50% | **1.25x** | 42% | **0.90x** | 23% |
| Log Opcodes: fixed offset False: non zero topic: 1 MiB zeros data: LOG1 | **0.95x** | 47% | **0.97x** | 32% | **0.88x** | 22% |
| Log Opcodes: fixed offset False: non zero topic: 1 MiB zeros data: LOG2 | **0.97x** | 48% | **1.18x** | 39% | **0.79x** | 20% |
| Log Opcodes: fixed offset False: non zero topic: 1 MiB zeros data: LOG3 | **1.17x** | 59% | **1.00x** | 33% | **0.86x** | 22% |
| Log Opcodes: fixed offset False: non zero topic: 1 MiB zeros data: LOG4 | **1.21x** | 61% | **0.97x** | 32% | **1.07x** | 27% |
| Log Opcodes: fixed offset False: zeros topic: 0 BYTEs data: LOG0 | **1.19x** | 60% | **1.11x** | 37% | **1.13x** | 28% |
| Log Opcodes: fixed offset False: zeros topic: 0 BYTEs data: LOG1 | **1.16x** | 58% | **1.03x** | 34% | **0.91x** | 23% |
| Log Opcodes: fixed offset False: zeros topic: 0 BYTEs data: LOG2 | **1.16x** | 58% | **1.21x** | 40% | **0.93x** | 23% |
| Log Opcodes: fixed offset False: zeros topic: 0 BYTEs data: LOG3 | **0.94x** | 47% | **0.98x** | 33% | **0.90x** | 22% |
| Log Opcodes: fixed offset False: zeros topic: 0 BYTEs data: LOG4 | **1.19x** | 60% | **1.16x** | 39% | **1.00x** | 25% |
| Log Opcodes: fixed offset False: zeros topic: 1 MiB non zero data: LOG0 | **1.21x** | 60% | **1.15x** | 38% | **0.77x** | 19% |
| Log Opcodes: fixed offset False: zeros topic: 1 MiB non zero data: LOG1 | **1.18x** | 59% | **1.19x** | 40% | **0.93x** | 23% |
| Log Opcodes: fixed offset False: zeros topic: 1 MiB non zero data: LOG2 | **0.93x** | 47% | **1.12x** | 37% | **0.93x** | 23% |
| Log Opcodes: fixed offset False: zeros topic: 1 MiB non zero data: LOG3 | **1.22x** | 61% | **1.06x** | 35% | **0.86x** | 21% |
| Log Opcodes: fixed offset False: zeros topic: 1 MiB non zero data: LOG4 | **1.21x** | 61% | **1.22x** | 41% | **0.88x** | 22% |
| Log Opcodes: fixed offset False: zeros topic: 1 MiB zeros data: LOG0 | **1.26x** | 63% | **1.00x** | 33% | **0.73x** | 18% |
| Log Opcodes: fixed offset False: zeros topic: 1 MiB zeros data: LOG1 | **1.18x** | 59% | **1.24x** | 41% | **0.94x** | 23% |
| Log Opcodes: fixed offset False: zeros topic: 1 MiB zeros data: LOG2 | **0.97x** | 48% | **1.20x** | 40% | **1.07x** | 27% |
| Log Opcodes: fixed offset False: zeros topic: 1 MiB zeros data: LOG3 | **0.92x** | 46% | **0.94x** | 31% | **0.89x** | 22% |
| Log Opcodes: fixed offset False: zeros topic: 1 MiB zeros data: LOG4 | **0.95x** | 48% | **1.20x** | 40% | **0.94x** | 24% |
| Log Opcodes: fixed offset True: non zero topic: 0 BYTEs data: LOG0 | **1.19x** | 60% | **1.19x** | 40% | **0.93x** | 23% |
| Log Opcodes: fixed offset True: non zero topic: 0 BYTEs data: LOG1 | **1.21x** | 61% | **1.18x** | 39% | **1.09x** | 27% |
| Log Opcodes: fixed offset True: non zero topic: 0 BYTEs data: LOG2 | **0.98x** | 49% | **1.37x** | 46% | **1.11x** | 28% |
| Log Opcodes: fixed offset True: non zero topic: 0 BYTEs data: LOG3 | **1.16x** | 58% | **1.30x** | 43% | **0.96x** | 24% |
| Log Opcodes: fixed offset True: non zero topic: 0 BYTEs data: LOG4 | **1.20x** | 60% | **1.33x** | 44% | **0.92x** | 23% |
| Log Opcodes: fixed offset True: non zero topic: 1 MiB non zero data: LOG0 | **1.19x** | 59% | **1.06x** | 35% | **0.94x** | 24% |
| Log Opcodes: fixed offset True: non zero topic: 1 MiB non zero data: LOG1 | **1.19x** | 60% | **0.92x** | 31% | **0.96x** | 24% |
| Log Opcodes: fixed offset True: non zero topic: 1 MiB non zero data: LOG2 | **1.18x** | 59% | **1.37x** | 46% | **0.96x** | 24% |
| Log Opcodes: fixed offset True: non zero topic: 1 MiB non zero data: LOG3 | **1.21x** | 61% | **1.22x** | 41% | **0.92x** | 23% |
| Log Opcodes: fixed offset True: non zero topic: 1 MiB non zero data: LOG4 | **1.21x** | 60% | **0.97x** | 32% | **1.06x** | 26% |
| Log Opcodes: fixed offset True: non zero topic: 1 MiB zeros data: LOG0 | **0.96x** | 48% | **1.04x** | 35% | **0.86x** | 21% |
| Log Opcodes: fixed offset True: non zero topic: 1 MiB zeros data: LOG1 | **0.99x** | 50% | **1.20x** | 40% | **1.00x** | 25% |
| Log Opcodes: fixed offset True: non zero topic: 1 MiB zeros data: LOG2 | **1.24x** | 62% | **0.97x** | 32% | **0.81x** | 20% |
| Log Opcodes: fixed offset True: non zero topic: 1 MiB zeros data: LOG3 | **1.15x** | 58% | **0.97x** | 32% | **0.81x** | 20% |
| Log Opcodes: fixed offset True: non zero topic: 1 MiB zeros data: LOG4 | **1.18x** | 59% | **1.18x** | 39% | **0.98x** | 24% |
| Log Opcodes: fixed offset True: zeros topic: 0 BYTEs data: LOG0 | **1.20x** | 60% | **1.18x** | 39% | **0.98x** | 24% |
| Log Opcodes: fixed offset True: zeros topic: 0 BYTEs data: LOG1 | **0.98x** | 49% | **1.08x** | 36% | **1.06x** | 27% |
| Log Opcodes: fixed offset True: zeros topic: 0 BYTEs data: LOG2 | **1.23x** | 61% | **0.99x** | 33% | **0.97x** | 24% |
| Log Opcodes: fixed offset True: zeros topic: 0 BYTEs data: LOG3 | **1.16x** | 58% | **0.94x** | 31% | **1.02x** | 26% |
| Log Opcodes: fixed offset True: zeros topic: 0 BYTEs data: LOG4 | **1.14x** | 57% | **1.36x** | 45% | **1.02x** | 25% |
| Log Opcodes: fixed offset True: zeros topic: 1 MiB non zero data: LOG0 | **1.22x** | 61% | **1.36x** | 45% | **0.97x** | 24% |
| Log Opcodes: fixed offset True: zeros topic: 1 MiB non zero data: LOG1 | **1.16x** | 58% | **1.19x** | 40% | **0.96x** | 24% |
| Log Opcodes: fixed offset True: zeros topic: 1 MiB non zero data: LOG2 | **1.19x** | 59% | **1.18x** | 39% | **0.92x** | 23% |
| Log Opcodes: fixed offset True: zeros topic: 1 MiB non zero data: LOG3 | **0.97x** | 49% | **1.20x** | 40% | **0.85x** | 21% |
| Log Opcodes: fixed offset True: zeros topic: 1 MiB non zero data: LOG4 | **1.10x** | 55% | **1.17x** | 39% | **0.97x** | 24% |
| Log Opcodes: fixed offset True: zeros topic: 1 MiB zeros data: LOG0 | **1.17x** | 59% | **1.21x** | 40% | **0.92x** | 23% |
| Log Opcodes: fixed offset True: zeros topic: 1 MiB zeros data: LOG1 | **0.94x** | 47% | **0.97x** | 32% | **0.97x** | 24% |
| Log Opcodes: fixed offset True: zeros topic: 1 MiB zeros data: LOG2 | **1.13x** | 56% | **1.19x** | 40% | **0.98x** | 25% |
| Log Opcodes: fixed offset True: zeros topic: 1 MiB zeros data: LOG3 | **1.20x** | 60% | **1.23x** | 41% | **0.94x** | 23% |
| Log Opcodes: fixed offset True: zeros topic: 1 MiB zeros data: LOG4 | **1.13x** | 56% | **1.19x** | 40% | **1.09x** | 27% |

**Category Statistics:**

| Configuration | Mean Speedup | Mean Efficiency |
| --- | --- | --- |
| 2 GPUs | 1.13x | 56.3% |
| 3 GPUs | 1.14x | 38.0% |
| 4 GPUs | 0.94x | 23.6% |

---

### Worst Stateful Opcodes.Py

**44 tests**

| Test Name | 2 GPUs | Eff | 3 GPUs | Eff | 4 GPUs | Eff |
| --- | --- | --- | --- | --- | --- | --- |
| Address State Cold: absent accounts False: BALANCE | **1.19x** | 59% | **1.43x** | 48% | **1.42x** | 35% |
| Address State Cold: absent accounts True: BALANCE | **1.11x** | 56% | **1.12x** | 37% | **0.97x** | 24% |
| Address State Warm: absent target False: BALANCE | **1.18x** | 59% | **1.40x** | 47% | **1.40x** | 35% |
| Address State Warm: absent target False: CALLCODE | **1.65x** | 83% | **2.13x** | 71% | **2.53x** | 63% |
| Address State Warm: absent target False: CALL | **1.59x** | 79% | **2.07x** | 69% | **2.49x** | 62% |
| Address State Warm: absent target False: DELEGATECALL | **1.58x** | 79% | **2.02x** | 67% | **2.48x** | 62% |
| Address State Warm: absent target False: EXTCODEHASH | **1.22x** | 61% | **1.41x** | 47% | **1.48x** | 37% |
| Address State Warm: absent target False: EXTCODESIZE | **1.17x** | 58% | **1.31x** | 44% | **1.38x** | 34% |
| Address State Warm: absent target False: STATICCALL | **1.67x** | 83% | **2.20x** | 73% | **2.50x** | 62% |
| Address State Warm: absent target True: BALANCE | **1.12x** | 56% | **1.33x** | 44% | **1.27x** | 32% |
| Address State Warm: absent target True: CALLCODE | **1.60x** | 80% | **2.05x** | 68% | **2.45x** | 61% |
| Address State Warm: absent target True: CALL | **1.57x** | 78% | **2.05x** | 68% | **2.47x** | 62% |
| Address State Warm: absent target True: DELEGATECALL | **1.60x** | 80% | **1.97x** | 66% | **2.35x** | 59% |
| Address State Warm: absent target True: EXTCODEHASH | **1.26x** | 63% | **1.46x** | 49% | **1.52x** | 38% |
| Address State Warm: absent target True: EXTCODESIZE | **1.11x** | 56% | **1.42x** | 47% | **1.37x** | 34% |
| Address State Warm: absent target True: STATICCALL | **1.62x** | 81% | **2.07x** | 69% | **2.41x** | 60% |
| Blockhash: benchmark: gas: value 1M: blockchain test | **1.35x** | 68% | **1.66x** | 55% | **1.82x** | 45% |
| Extcodecopy Warm: 1KiB | **1.11x** | 56% | **1.38x** | 46% | **1.32x** | 33% |
| Extcodecopy Warm: 512 | **1.13x** | 56% | **1.53x** | 51% | **1.11x** | 28% |
| Extcodecopy Warm: 5KiB | **1.09x** | 54% | **1.28x** | 43% | **1.11x** | 28% |
| Selfbalance: benchmark: gas: value 1M: blockchain test from state test | **1.72x** | 86% | **2.40x** | 80% | **2.93x** | 73% |
| Selfdestruct Created: value bearing False | **1.15x** | 57% | **1.15x** | 38% | **0.95x** | 24% |
| Selfdestruct Created: value bearing True | **1.17x** | 59% | **1.21x** | 40% | **0.82x** | 21% |
| Selfdestruct Existing: value bearing False | **1.07x** | 54% | **1.19x** | 40% | **1.12x** | 28% |
| Selfdestruct Existing: value bearing True | **1.09x** | 55% | **1.15x** | 38% | **1.07x** | 27% |
| Selfdestruct Initcode: value bearing False | **1.22x** | 61% | **1.18x** | 39% | **0.88x** | 22% |
| Selfdestruct Initcode: value bearing True | **1.21x** | 60% | **1.15x** | 38% | **0.95x** | 24% |
| Storage Access Cold: absent slots False: SSLOAD | **1.19x** | 60% | **1.37x** | 46% | **1.47x** | 37% |
| Storage Access Cold: absent slots False: SSTORE new value, out of gas | **1.23x** | 61% | **1.32x** | 44% | **1.12x** | 28% |
| Storage Access Cold: absent slots False: SSTORE new value, REVERT | **1.09x** | 54% | **1.30x** | 43% | **1.31x** | 33% |
| Storage Access Cold: absent slots False: SSTORE new value | **1.05x** | 53% | **1.46x** | 49% | **1.17x** | 29% |
| Storage Access Cold: absent slots False: SSTORE same value, out of gas | **1.18x** | 59% | **1.41x** | 47% | **1.52x** | 38% |
| Storage Access Cold: absent slots False: SSTORE same value, REVERT | **1.17x** | 59% | **1.50x** | 50% | **1.54x** | 39% |
| Storage Access Cold: absent slots False: SSTORE same value | **1.18x** | 59% | **1.46x** | 49% | N/A | N/A |
| Storage Access Cold: absent slots True: SSLOAD | **1.14x** | 57% | **1.35x** | 45% | **1.15x** | 29% |
| Storage Access Cold: absent slots True: SSTORE new value, out of gas | **1.15x** | 58% | **1.16x** | 39% | **0.96x** | 24% |
| Storage Access Cold: absent slots True: SSTORE new value, REVERT | **1.20x** | 60% | **1.03x** | 34% | **0.96x** | 24% |
| Storage Access Cold: absent slots True: SSTORE new value | **0.96x** | 48% | **0.95x** | 32% | **0.98x** | 25% |
| Storage Access Cold: absent slots True: SSTORE same value, out of gas | **0.96x** | 48% | **0.95x** | 32% | **0.92x** | 23% |
| Storage Access Cold: absent slots True: SSTORE same value, REVERT | **1.15x** | 57% | **1.15x** | 38% | **0.93x** | 23% |
| Storage Access Cold: absent slots True: SSTORE same value | **1.12x** | 56% | **1.13x** | 38% | **0.91x** | 23% |
| Storage Access Warm: SLOAD | **1.13x** | 56% | **1.62x** | 54% | **1.39x** | 35% |
| Storage Access Warm: SSTORE new value | **1.47x** | 74% | **1.82x** | 61% | **2.06x** | 51% |
| Storage Access Warm: SSTORE same value | **1.47x** | 73% | **1.74x** | 58% | **1.95x** | 49% |

**Category Statistics:**

| Configuration | Mean Speedup | Mean Efficiency |
| --- | --- | --- |
| 2 GPUs | 1.26x | 62.9% |
| 3 GPUs | 1.49x | 49.6% |
| 4 GPUs | 1.51x | 37.7% |

---
