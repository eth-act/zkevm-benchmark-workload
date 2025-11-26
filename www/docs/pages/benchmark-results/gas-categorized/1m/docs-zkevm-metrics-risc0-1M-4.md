# zkEVM Benchmark Results

Generated on: 2025-11-26 13:07:24

## Folder: zkevm-metrics-risc0-1M-4

**zkVM:** risc0-v3.0.3 (4 GPUs)

**Hardware Configuration:** CPU: AMD EPYC 7B13 64-Core Processor | RAM: 396 GiB | GPU: NVIDIA GeForce RTX 4090, NVIDIA GeForce RTX 4090, NVIDIA GeForce RTX 4090, NVIDIA GeForce RTX 4090

## Proving Metrics

| Benchmark | Gas Used | Proof Size (bytes) | Proving Time (ms) | Proving Time (s) | Peak Memory (MB) | Avg Memory (MB) | Initial Memory (MB) |
|---|---|---|---|---|---|---|---|
| SLOAD, SSTORE, BALANCE, EXTCODESIZE, EXTCODEHASH, CALLDATALOAD, CALLDATACOPY, CALLDATASIZE (fork Prague, 1M gas) | 999,980 | 223,239 | 6,001 | 6.00 | 3,555,652 | 3,554,920 | 3,550,772 |
| CALLDATALOAD, CALLDATACOPY, CALLDATASIZE (fork Prague, 1M gas, zero byte: False) | 1,000,000 | 223,239 | 4,548 | 4.55 | 4,696,320 | 4,696,320 | 4,694,368 |
| CALLDATALOAD, CALLDATACOPY, CALLDATASIZE (fork Prague, 1M gas, zero byte: True) | 1,000,000 | 223,239 | 8,506 | 8.51 | 3,841,572 | 3,841,572 | 3,839,620 |
| CALL (fork Prague, 1M gas, case id a to a) | 987,000 | 223,239 | 12,481 | 12.48 | 3,734,240 | 3,734,118 | 3,733,264 |
| CALL (fork Prague, 1M gas, case id a to b) | 987,000 | 223,239 | 13,173 | 13.17 | 4,376,332 | 4,376,332 | 4,375,356 |
| CALL (fork Prague, 1M gas) | 987,000 | 223,239 | 13,312 | 13.31 | 4,679,744 | 4,678,890 | 4,678,768 |
| CALL (fork Prague, 1M gas) | 987,000 | 223,239 | 12,961 | 12.96 | 3,772,292 | 3,770,733.6 | 3,768,392 |
| CALL (fork Prague, 1M gas) | 987,000 | 223,239 | 14,477 | 14.48 | 4,102,128 | 4,100,371.2 | 4,094,320 |
| CALLCODE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 97,037 | 97.04 | 3,661,044 | 3,660,107.8 | 3,653,236 |
| CALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 97,257 | 97.26 | 2,573,820 | 2,528,826.4 | 2,491,836 |
| DELEGATECALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 94,896 | 94.90 | 4,292,416 | 4,291,810.2 | 4,290,464 |
| EXTCODECOPY (fork Prague, 1M gas) | 1,000,000 | 223,239 | 93,863 | 93.86 | 4,705,104 | 4,704,848.4 | 4,704,128 |
| EXTCODEHASH (fork Prague, 1M gas) | 1,000,000 | 223,239 | 97,273 | 97.27 | 3,525,400 | 3,524,357.8 | 3,518,568 |
| EXTCODESIZE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 97,861 | 97.86 | 4,476,780 | 4,474,473.1 | 4,472,876 |
| STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 96,548 | 96.55 | 4,792,892 | 4,792,824.7 | 4,791,916 |
| CREATE2 (fork Prague, 1M gas, 0 bytes) | 1,000,000 | 223,239 | 4,534 | 4.53 | 2,616,764 | 2,615,788 | 2,611,884 |
| CREATE (fork Prague, 1M gas, 0 bytes) | 1,000,000 | 223,239 | 4,521 | 4.52 | 677,480 | 675,388.6 | 669,672 |
| CREATE2 (fork Prague, 1M gas, 0 bytes) | 1,000,000 | 223,239 | 4,651 | 4.65 | 4,713,872 | 4,713,872 | 4,712,896 |
| CREATE (fork Prague, 1M gas, 0 bytes) | 1,000,000 | 223,239 | 4,993 | 4.99 | 4,234,844 | 4,234,844 | 4,233,868 |
| CREATE2 (fork Prague, 1M gas, zero data) | 1,000,000 | 223,239 | 4,962 | 4.96 | 846,324 | 837,662 | 823,876 |
| CREATE (fork Prague, 1M gas, zero data) | 1,000,000 | 223,239 | 5,033 | 5.03 | 4,433,864 | 4,433,864 | 4,431,912 |
| CREATE2 (fork Prague, 1M gas) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| CREATE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 4,928 | 4.93 | 3,592,732 | 3,590,536 | 3,586,876 |
| CREATE2 (fork Prague, 1M gas, zero data) | 1,000,000 | 223,239 | 5,031 | 5.03 | 4,467,024 | 4,467,024 | 4,466,048 |
| CREATE (fork Prague, 1M gas, zero data) | 1,000,000 | 223,239 | 4,676 | 4.68 | 4,684,624 | 4,684,624 | 4,683,648 |
| CREATE2 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 5,655 | 5.66 | 3,779,116 | 3,778,920 | 3,779,116 |
| CREATE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 5,271 | 5.27 | 3,364,376 | 3,363,400 | 3,358,520 |
| CREATE2 (fork Prague, 1M gas, zero data) | 1,000,000 | 223,239 | 6,393 | 6.39 | 1,841,832 | 1,839,099.2 | 1,833,048 |
| CREATE (fork Prague, 1M gas, zero data) | 1,000,000 | 223,239 | 4,860 | 4.86 | 4,233,868 | 4,233,868 | 4,233,868 |
| CREATE2 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 4,748 | 4.75 | 3,263,852 | 3,263,201.3 | 3,257,996 |
| CREATE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 4,872 | 4.87 | 3,571,268 | 3,571,268 | 3,568,340 |
| CREATE2 (fork Prague, 1M gas, zero data) | 1,000,000 | 223,239 | 5,538 | 5.54 | 4,581,176 | 4,581,176 | 4,580,200 |
| CREATE (fork Prague, 1M gas, zero data) | 1,000,000 | 223,239 | 5,661 | 5.66 | 3,271,660 | 3,271,660 | 3,269,708 |
| CREATE2 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 5,148 | 5.15 | 4,439,716 | 4,439,065.3 | 4,438,740 |
| CREATE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 5,096 | 5.10 | 4,358,768 | 4,356,816 | 4,352,912 |
| CREATE2 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 4,797 | 4.80 | 4,428,988 | 4,428,988 | 4,428,012 |
| CREATE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 6,070 | 6.07 | 4,466,052 | 4,465,856 | 4,466,052 |
| JUMPDEST, JUMP (fork Prague, 1M gas, 00) | 1,000,000 | 223,239 | 12,699 | 12.70 | 3,962,564 | 3,962,320 | 3,958,660 |
| JUMPDEST, JUMP (fork Prague, 1M gas, 5b) | 1,000,000 | 223,239 | 13,694 | 13.69 | 4,719,728 | 4,718,752 | 4,716,800 |
| JUMPDEST, JUMP (fork Prague, 1M gas, 605b5b) | 1,000,000 | 223,239 | 12,038 | 12.04 | 982,964 | 967,868.5 | 946,852 |
| JUMPDEST, JUMP (fork Prague, 1M gas, 605b) | 1,000,000 | 223,239 | 10,434 | 10.43 | 4,293,392 | 4,293,392 | 4,292,416 |
| JUMPDEST, JUMP (fork Prague, 1M gas, 615b5b5b) | 1,000,000 | 223,239 | 11,434 | 11.43 | 946,852 | 928,633.3 | 904,884 |
| JUMPDEST, JUMP (fork Prague, 1M gas, 615b5b) | 1,000,000 | 223,239 | 9,786 | 9.79 | 4,564,588 | 4,564,588 | 4,563,612 |
| PRECOMPILE_EC_PAIRING, CALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 69,374 | 69.37 | 3,765,472 | 3,764,783.1 | 3,759,616 |
| UNKNOWN (fork Prague, 1M gas) | 0 | 223,239 | 3,968 | 3.97 | 2,950,556 | 2,949,824 | 2,944,700 |
| ADD (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 15,224 | 15.22 | 4,410,456 | 4,408,113.6 | 4,404,600 |
| AND (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 13,419 | 13.42 | 4,074,800 | 4,073,092 | 4,067,968 |
| BYTE (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 14,251 | 14.25 | 4,793,868 | 4,793,868 | 4,792,892 |
| DIV (fork Prague, 1M gas, 0) | 1,000,000 | 223,239 | 54,593 | 54.59 | 4,477,756 | 4,476,894.8 | 4,475,804 |
| DIV (fork Prague, 1M gas, 1) | 1,000,000 | 223,239 | 51,548 | 51.55 | 4,247,532 | 4,245,580 | 4,242,652 |
| EQ (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 24,223 | 24.22 | 2,462,560 | 2,454,996 | 2,443,040 |
| EXP (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 34,062 | 34.06 | 3,598,588 | 3,596,148 | 3,592,732 |
| GT (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 13,932 | 13.93 | 4,350,964 | 4,350,964 | 4,349,012 |
| LT (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 13,747 | 13.75 | 4,731,424 | 4,731,260.7 | 4,731,424 |
| MOD (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 19,756 | 19.76 | 4,349,008 | 4,349,008 | 4,349,008 |
| MUL (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 20,341 | 20.34 | 4,170,436 | 4,170,436 | 4,161,652 |
| OR (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 13,769 | 13.77 | 4,793,868 | 4,793,868 | 4,792,892 |
| SAR (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 24,051 | 24.05 | 3,880,604 | 3,880,604 | 3,880,604 |
| SDIV (fork Prague, 1M gas, 0) | 1,000,000 | 223,239 | 61,651 | 61.65 | 4,480,668 | 4,480,606.8 | 4,479,692 |
| SDIV (fork Prague, 1M gas, 1) | 1,000,000 | 223,239 | 62,264 | 62.26 | 1,746,184 | 1,725,604.3 | 1,693,480 |
| SGT (fork Prague, 1M gas, ) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| SHL (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 21,589 | 21.59 | 4,726,552 | 4,726,552 | 4,725,576 |
| SHR (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 21,705 | 21.70 | 4,588,004 | 4,587,895.1 | 4,588,004 |
| SIGNEXTEND (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 19,550 | 19.55 | 4,479,700 | 4,479,700 | 4,477,748 |
| SLT (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 15,516 | 15.52 | 4,207,516 | 4,206,784 | 4,202,636 |
| SMOD (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 22,700 | 22.70 | 4,826,076 | 4,826,076 | 4,825,100 |
| SUB (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 16,171 | 16.17 | 4,684,624 | 4,684,624 | 4,684,624 |
| XOR (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 13,371 | 13.37 | 4,263,144 | 4,263,144 | 4,262,168 |
| BLOBHASH (fork Prague, 1M gas, no blobs) | 1,000,000 | 223,239 | 13,760 | 13.76 | 3,529,300 | 3,529,300 | 3,528,324 |
| BLOBHASH (fork Prague, 1M gas) | 1,000,000 | 223,239 | 18,126 | 18.13 | 1,260,148 | 1,248,741 | 1,225,988 |
| BLOBHASH (fork Prague, 1M gas, existent index) | 1,000,000 | 223,239 | 15,027 | 15.03 | 4,726,552 | 4,726,552 | 4,726,552 |
| BLOBHASH (fork Prague, 1M gas) | 1,000,000 | 223,239 | 18,840 | 18.84 | 2,589,436 | 2,588,134.7 | 2,585,532 |
| CALLDATALOAD (fork Prague, 1M gas, empty) | 1,000,000 | 223,239 | 24,311 | 24.31 | 4,264,120 | 4,264,120 | 4,264,120 |
| CALLDATALOAD (fork Prague, 1M gas, one, loop) | 1,000,000 | 223,239 | 25,914 | 25.91 | 4,449,464 | 4,449,464 | 4,449,464 |
| CALLDATALOAD (fork Prague, 1M gas, zero, loop) | 1,000,000 | 223,239 | 26,108 | 26.11 | 3,660,068 | 3,660,068 | 3,660,068 |
| CALLDATASIZE (fork Prague, 1M gas, 0 calldata length) | 1,000,000 | 223,239 | 13,501 | 13.50 | 4,828,028 | 4,828,028 | 4,827,052 |
| CALLDATASIZE (fork Prague, 1M gas, 10000 calldata length) | 1,000,000 | 223,239 | 13,675 | 13.68 | 4,824,124 | 4,824,124 | 4,823,148 |
| CALLDATASIZE (fork Prague, 1M gas, 1000 calldata length) | 1,000,000 | 223,239 | 14,515 | 14.52 | 3,738,144 | 3,737,534 | 3,737,168 |
| CALLVALUE (fork Prague, 1M gas, from origin: False, non zero value: False) | 1,000,000 | 223,239 | 15,378 | 15.38 | 4,421,192 | 4,421,192 | 4,419,240 |
| CALLVALUE (fork Prague, 1M gas, from origin: False, non zero value: True) | 1,000,000 | 223,239 | 15,676 | 15.68 | 4,818,268 | 4,818,268 | 4,817,292 |
| CALLVALUE (fork Prague, 1M gas, from origin: True, non zero value: False) | 1,000,000 | 223,239 | 15,438 | 15.44 | 4,706,076 | 4,706,076 | 4,704,124 |
| CALLVALUE (fork Prague, 1M gas, from origin: True, non zero value: True) | 1,000,000 | 223,239 | 15,441 | 15.44 | 4,691,440 | 4,691,440 | 4,691,440 |
| DUP10 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 13,328 | 13.33 | 4,349,008 | 4,349,008 | 4,349,008 |
| DUP11 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 12,889 | 12.89 | 3,787,892 | 3,787,892 | 3,787,892 |
| DUP12 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 13,315 | 13.31 | 4,567,516 | 4,566,540 | 4,564,588 |
| DUP13 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 13,150 | 13.15 | 3,774,244 | 3,774,122 | 3,773,268 |
| DUP14 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 13,146 | 13.15 | 4,431,912 | 4,431,912 | 4,431,912 |
| DUP15 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 12,923 | 12.92 | 2,490,864 | 2,486,262.3 | 2,478,176 |
| DUP16 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 13,176 | 13.18 | 4,679,744 | 4,679,744 | 4,678,768 |
| DUP1 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 13,351 | 13.35 | 4,352,912 | 4,352,830.7 | 4,352,912 |
| DUP2 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 12,897 | 12.90 | 4,460,196 | 4,459,708 | 4,456,292 |
| DUP3 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 13,139 | 13.14 | 4,714,848 | 4,714,848 | 4,714,848 |
| DUP4 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 13,061 | 13.06 | 3,750,832 | 3,749,313.8 | 3,744,000 |
| DUP5 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 13,005 | 13.01 | 4,177,268 | 4,176,194 | 4,170,436 |
| DUP6 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 12,783 | 12.78 | 2,958,364 | 2,954,460 | 2,951,532 |
| DUP7 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 13,525 | 13.53 | 3,527,352 | 3,527,352 | 3,527,352 |
| DUP8 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 13,056 | 13.06 | 4,349,008 | 4,349,008 | 4,349,008 |
| DUP9 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 13,262 | 13.26 | 2,618,716 | 2,618,716 | 2,616,764 |
| JUMPDEST (fork Prague, 1M gas) | 1,000,000 | 223,239 | 14,907 | 14.91 | 4,446,548 | 4,446,548 | 4,445,572 |
| JUMPI (fork Prague, 1M gas) | 1,000,000 | 223,239 | 16,623 | 16.62 | 1,771,560 | 1,759,604 | 1,747,160 |
| JUMPI (fork Prague, 1M gas) | 1,000,000 | 223,239 | 12,793 | 12.79 | 3,953,788 | 3,953,678.7 | 3,953,788 |
| JUMP (fork Prague, 1M gas) | 1,000,000 | 223,239 | 11,905 | 11.90 | 4,808,508 | 4,806,946.4 | 4,803,628 |
| SHA3, KECCAK256 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 45,410 | 45.41 | 4,755,820 | 4,755,771 | 4,754,844 |
| MLOAD (fork Prague, 1M gas, big memory expansion: False, offset initialized: False, 0 offset) | 1,000,000 | 223,239 | 16,926 | 16.93 | 3,480,508 | 3,477,580 | 3,473,676 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: False, offset initialized: False, 0 offset) | 1,000,000 | 223,239 | 13,438 | 13.44 | 1,861,352 | 1,856,332.6 | 1,843,784 |
| MSTORE (fork Prague, 1M gas, big memory expansion: False, offset initialized: False, 0 offset) | 1,000,000 | 223,239 | 16,787 | 16.79 | 1,060,068 | 1,028,564.9 | 982,964 |
| MLOAD (fork Prague, 1M gas, big memory expansion: False, offset initialized: False, 1 offset) | 1,000,000 | 223,239 | 17,230 | 17.23 | 3,953,788 | 3,953,788 | 3,950,860 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: False, offset initialized: False, 1 offset) | 1,000,000 | 223,239 | 12,602 | 12.60 | 4,772,400 | 4,772,400 | 4,772,400 |
| MSTORE (fork Prague, 1M gas, big memory expansion: False, offset initialized: False, 1 offset) | 1,000,000 | 223,239 | 17,296 | 17.30 | 4,405,580 | 4,405,580 | 4,404,604 |
| MLOAD (fork Prague, 1M gas, big memory expansion: False, offset initialized: False, 31 offset) | 1,000,000 | 223,239 | 17,093 | 17.09 | 4,313,880 | 4,312,708.8 | 4,310,952 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: False, offset initialized: False, 31 offset) | 1,000,000 | 223,239 | 12,826 | 12.83 | 2,010,672 | 1,992,738 | 1,975,536 |
| MSTORE (fork Prague, 1M gas, big memory expansion: False, offset initialized: False, 31 offset) | 1,000,000 | 223,239 | 17,139 | 17.14 | 3,612,244 | 3,612,244 | 3,611,268 |
| MLOAD (fork Prague, 1M gas, big memory expansion: False, offset initialized: True, 0 offset) | 1,000,000 | 223,239 | 15,240 | 15.24 | 2,930,060 | 2,929,206 | 2,925,180 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: False, offset initialized: True, 0 offset) | 1,000,000 | 223,239 | 13,276 | 13.28 | 4,268,024 | 4,268,024 | 4,267,048 |
| MSTORE (fork Prague, 1M gas, big memory expansion: False, offset initialized: True, 0 offset) | 1,000,000 | 223,239 | 17,264 | 17.26 | 4,704,128 | 4,704,128 | 4,703,152 |
| MLOAD (fork Prague, 1M gas, big memory expansion: False, offset initialized: True, 1 offset) | 1,000,000 | 223,239 | 16,049 | 16.05 | 4,494,324 | 4,494,324 | 4,494,324 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: False, offset initialized: True, 1 offset) | 1,000,000 | 223,239 | 13,702 | 13.70 | 3,890,360 | 3,890,197.3 | 3,890,360 |
| MSTORE (fork Prague, 1M gas, big memory expansion: False, offset initialized: True, 1 offset) | 1,000,000 | 223,239 | 16,721 | 16.72 | 4,202,636 | 4,202,440.8 | 4,196,780 |
| MLOAD (fork Prague, 1M gas, big memory expansion: False, offset initialized: True, 31 offset) | 1,000,000 | 223,239 | 16,526 | 16.53 | 4,764,600 | 4,764,600 | 4,764,600 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: False, offset initialized: True, 31 offset) | 1,000,000 | 223,239 | 13,341 | 13.34 | 4,803,628 | 4,802,652 | 4,800,700 |
| MSTORE (fork Prague, 1M gas, big memory expansion: False, offset initialized: True, 31 offset) | 1,000,000 | 223,239 | 16,684 | 16.68 | 4,424,120 | 4,423,794.7 | 4,421,192 |
| MLOAD (fork Prague, 1M gas, big memory expansion: True, offset initialized: False, 0 offset) | 1,000,000 | 223,239 | 16,672 | 16.67 | 4,563,612 | 4,563,612 | 4,563,612 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: True, offset initialized: False, 0 offset) | 1,000,000 | 223,239 | 13,788 | 13.79 | 1,960,896 | 1,950,282 | 1,937,472 |
| MSTORE (fork Prague, 1M gas, big memory expansion: True, offset initialized: False, 0 offset) | 1,000,000 | 223,239 | 16,552 | 16.55 | 3,462,944 | 3,460,459.6 | 3,451,232 |
| MLOAD (fork Prague, 1M gas, big memory expansion: True, offset initialized: False, 1 offset) | 1,000,000 | 223,239 | 16,973 | 16.97 | 3,977,204 | 3,974,973.1 | 3,968,420 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: True, offset initialized: False, 1 offset) | 1,000,000 | 223,239 | 13,875 | 13.88 | 4,435,816 | 4,435,652.7 | 4,434,840 |
| MSTORE (fork Prague, 1M gas, big memory expansion: True, offset initialized: False, 1 offset) | 1,000,000 | 223,239 | 16,917 | 16.92 | 3,569,316 | 3,568,479.4 | 3,565,412 |
| MLOAD (fork Prague, 1M gas, big memory expansion: True, offset initialized: False, 31 offset) | 1,000,000 | 223,239 | 16,637 | 16.64 | 3,257,996 | 3,254,970.4 | 3,251,164 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: True, offset initialized: False, 31 offset) | 1,000,000 | 223,239 | 13,008 | 13.01 | 4,830,956 | 4,830,956 | 4,829,980 |
| MSTORE (fork Prague, 1M gas, big memory expansion: True, offset initialized: False, 31 offset) | 1,000,000 | 223,239 | 17,219 | 17.22 | 3,936,232 | 3,935,622 | 3,932,328 |
| MLOAD (fork Prague, 1M gas, big memory expansion: True, offset initialized: True, 0 offset) | 1,000,000 | 223,239 | 16,793 | 16.79 | 806,308 | 787,764 | 759,460 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: True, offset initialized: True, 0 offset) | 1,000,000 | 223,239 | 13,185 | 13.19 | 4,349,984 | 4,349,984 | 4,349,008 |
| MSTORE (fork Prague, 1M gas, big memory expansion: True, offset initialized: True, 0 offset) | 1,000,000 | 223,239 | 16,732 | 16.73 | 4,831,932 | 4,831,932 | 4,830,956 |
| MLOAD (fork Prague, 1M gas, big memory expansion: True, offset initialized: True, 1 offset) | 1,000,000 | 223,239 | 16,339 | 16.34 | 4,215,324 | 4,214,487.4 | 4,210,444 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: True, offset initialized: True, 1 offset) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| MSTORE (fork Prague, 1M gas, big memory expansion: True, offset initialized: True, 1 offset) | 1,000,000 | 223,239 | 16,865 | 16.86 | 4,383,140 | 4,383,140 | 4,383,140 |
| MLOAD (fork Prague, 1M gas, big memory expansion: True, offset initialized: True, 31 offset) | 1,000,000 | 223,239 | 16,990 | 16.99 | 3,855,232 | 3,855,232 | 3,855,232 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: True, offset initialized: True, 31 offset) | 1,000,000 | 223,239 | 12,803 | 12.80 | 3,908,904 | 3,907,440 | 3,905,000 |
| MSTORE (fork Prague, 1M gas, big memory expansion: True, offset initialized: True, 31 offset) | 1,000,000 | 223,239 | 16,357 | 16.36 | 3,451,232 | 3,451,232 | 3,451,232 |
| MOD, SMOD (fork Prague, 1M gas, op MOD, 127 mod bits) | 1,000,000 | 223,239 | 50,182 | 50.18 | 4,554,828 | 4,552,237.8 | 4,548,972 |
| MOD, SMOD (fork Prague, 1M gas, op MOD, 191 mod bits) | 1,000,000 | 223,239 | 62,792 | 62.79 | 4,712,896 | 4,711,432 | 4,708,016 |
| MOD, SMOD (fork Prague, 1M gas, op MOD, 255 mod bits) | 1,000,000 | 223,239 | 50,299 | 50.30 | 4,497,252 | 4,495,625.3 | 4,494,324 |
| MOD, SMOD (fork Prague, 1M gas, op MOD, 63 mod bits) | 1,000,000 | 223,239 | 36,402 | 36.40 | 3,468,800 | 3,468,311.8 | 3,464,896 |
| MOD, SMOD (fork Prague, 1M gas, op SMOD, 127 mod bits) | 1,000,000 | 223,239 | 53,799 | 53.80 | 4,451,424 | 4,451,424 | 4,451,424 |
| MOD, SMOD (fork Prague, 1M gas, op SMOD, 191 mod bits) | 1,000,000 | 223,239 | 65,536 | 65.54 | 1,368,484 | 1,321,242.2 | 1,261,124 |
| MOD, SMOD (fork Prague, 1M gas, op SMOD, 255 mod bits) | 1,000,000 | 223,239 | 51,067 | 51.07 | 3,488,316 | 3,488,218.4 | 3,481,484 |
| MOD, SMOD (fork Prague, 1M gas, op SMOD, 63 mod bits) | 1,000,000 | 223,239 | 38,558 | 38.56 | 4,767,528 | 4,766,035.3 | 4,763,624 |
| ADDMOD, MULMOD (fork Prague, 1M gas, op ADDMOD, 127 mod bits) | 1,000,000 | 223,239 | 41,897 | 41.90 | 4,183,120 | 4,183,120 | 4,182,144 |
| ADDMOD, MULMOD (fork Prague, 1M gas, op ADDMOD, 191 mod bits) | 1,000,000 | 223,239 | 48,019 | 48.02 | 4,220,204 | 4,220,204 | 4,218,252 |
| ADDMOD, MULMOD (fork Prague, 1M gas, op ADDMOD, 255 mod bits) | 1,000,000 | 223,239 | 41,561 | 41.56 | 3,346,808 | 3,343,229.3 | 3,339,000 |
| ADDMOD, MULMOD (fork Prague, 1M gas, op ADDMOD, 63 mod bits) | 1,000,000 | 223,239 | 30,494 | 30.49 | 4,224,108 | 4,222,741.6 | 4,220,204 |
| ADDMOD, MULMOD (fork Prague, 1M gas, op MULMOD, 127 mod bits) | 1,000,000 | 223,239 | 69,186 | 69.19 | 2,677,276 | 2,651,839 | 2,629,452 |
| ADDMOD, MULMOD (fork Prague, 1M gas, op MULMOD, 191 mod bits) | 1,000,000 | 223,239 | 94,095 | 94.09 | 4,240,700 | 4,239,658.9 | 4,236,796 |
| ADDMOD, MULMOD (fork Prague, 1M gas, op MULMOD, 255 mod bits) | 1,000,000 | 223,239 | 91,244 | 91.24 | 1,456,324 | 1,421,528.1 | 1,368,484 |
| ADDMOD, MULMOD (fork Prague, 1M gas, op MULMOD, 63 mod bits) | 1,000,000 | 223,239 | 55,331 | 55.33 | 3,338,028 | 3,336,608.4 | 3,331,196 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1045 mod) | 1,000,000 | 223,239 | 927,323 | 927.32 | 4,677,792 | 4,662,667.5 | 4,631,920 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1360 mod) | 1,000,000 | 223,239 | 236,440 | 236.44 | 3,874,748 | 3,872,485.5 | 3,867,916 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 400 mod) | 1,000,000 | 223,239 | 290,522 | 290.52 | 4,488,472 | 4,486,389.9 | 4,479,688 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 408 mod) | 1,000,000 | 223,239 | 206,555 | 206.56 | 4,349,988 | 4,349,988 | 4,349,988 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 408 mod) | 1,000,000 | 223,239 | 835,401 | 835.40 | 4,789,968 | 4,781,109.2 | 4,772,400 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 600 mod) | 1,000,000 | 223,239 | 213,113 | 213.11 | 3,839,620 | 3,834,471.6 | 3,824,004 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 600 mod) | 1,000,000 | 223,239 | 312,385 | 312.38 | 3,497,100 | 3,495,468.9 | 3,488,316 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 616 mod) | 1,000,000 | 223,239 | 880,656 | 880.66 | 4,392,896 | 4,392,108.6 | 4,390,944 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 677 mod) | 1,000,000 | 223,239 | 224,015 | 224.01 | 3,926,472 | 3,921,917.3 | 3,909,880 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 765 mod) | 1,000,000 | 223,239 | 251,464 | 251.46 | 4,754,844 | 4,746,463.4 | 4,739,228 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 767 mod) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 800 mod) | 1,000,000 | 223,239 | 906,051 | 906.05 | 4,404,604 | 4,403,449.3 | 4,402,652 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 800 mod) | 1,000,000 | 223,239 | 320,802 | 320.80 | 4,703,152 | 4,702,657.0 | 4,700,224 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 852 mod) | 1,000,000 | 223,239 | 320,753 | 320.75 | 4,764,600 | 4,760,122.3 | 4,754,840 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 867 mod) | 1,000,000 | 223,239 | 911,315 | 911.32 | 4,375,344 | 4,375,344 | 4,375,344 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 996 mod) | 1,000,000 | 223,239 | 203,625 | 203.62 | 4,378,268 | 4,378,229.3 | 4,375,340 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1024 mod even) | 1,000,000 | 223,239 | 4,937 | 4.94 | 4,568,492 | 4,568,492 | 4,568,492 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 128 mod even) | 1,000,000 | 223,239 | 188,689 | 188.69 | 4,267,048 | 4,266,813.8 | 4,264,120 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 16 mod even) | 1,000,000 | 223,239 | 305,705 | 305.70 | 4,146,040 | 4,146,040 | 4,142,136 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 24 mod even) | 1,000,000 | 223,239 | 250,194 | 250.19 | 4,418,264 | 4,417,295.1 | 4,415,336 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 256 mod even) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 32 mod even) | 1,000,000 | 223,239 | 236,847 | 236.85 | 2,924,204 | 2,769,389.3 | 2,738,764 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 32 mod even) | 1,000,000 | 223,239 | 196,415 | 196.41 | 4,829,980 | 4,829,426.9 | 4,828,028 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 32 mod even) | 1,000,000 | 223,239 | 224,979 | 224.98 | 4,545,076 | 4,541,979.7 | 4,541,172 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 512 mod even) | 1,000,000 | 223,239 | 5,061 | 5.06 | 2,627,500 | 2,627,500 | 2,623,596 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 64 mod even) | 1,000,000 | 223,239 | 204,846 | 204.85 | 3,326,316 | 3,311,180.7 | 3,287,276 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 8 mod even) | 1,000,000 | 223,239 | 513,827 | 513.83 | 3,516,616 | 3,512,726.5 | 3,504,904 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 208 mod exp) | 1,000,000 | 223,239 | 186,045 | 186.04 | 4,366,572 | 4,366,572 | 4,366,572 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 215 mod exp) | 1,000,000 | 223,239 | 452,451 | 452.45 | 4,469,952 | 4,468,967.4 | 4,467,024 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 298 mod exp) | 1,000,000 | 223,239 | 497,315 | 497.31 | 4,348,036 | 4,345,548.6 | 4,341,204 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, mod min as balanced) | 1,000,000 | 223,239 | 196,760 | 196.76 | 4,725,576 | 4,724,754.1 | 4,723,624 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, mod min as base heavy) | 1,000,000 | 223,239 | 682,767 | 682.77 | 4,125,552 | 4,120,493.5 | 4,110,912 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, mod min as exp heavy) | 1,000,000 | 223,239 | 445,771 | 445.77 | 4,395,820 | 4,395,820 | 4,395,820 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1024 mod odd) | 1,000,000 | 223,239 | 5,392 | 5.39 | 823,876 | 819,386.4 | 806,308 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 128 mod odd) | 1,000,000 | 223,239 | 190,141 | 190.14 | 3,618,100 | 3,617,013.5 | 3,614,196 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 256 mod odd) | 1,000,000 | 223,239 | 135,018 | 135.02 | 4,231,916 | 4,230,163.2 | 4,226,060 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 32 mod odd) | 1,000,000 | 223,239 | 236,647 | 236.65 | 1,648,592 | 1,584,046.3 | 1,462,176 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 32 mod odd) | 1,000,000 | 223,239 | 215,015 | 215.01 | 4,471,900 | 4,471,900 | 4,471,900 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 32 mod odd) | 1,000,000 | 223,239 | 174,335 | 174.34 | 4,155,796 | 4,155,726.3 | 4,150,916 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 512 mod odd) | 1,000,000 | 223,239 | 4,741 | 4.74 | 4,264,120 | 4,264,120 | 4,264,120 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 64 mod odd) | 1,000,000 | 223,239 | 206,635 | 206.63 | 3,587,856 | 3,586,975.2 | 3,579,072 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 2 mod pawel) | 1,000,000 | 223,239 | 294,020 | 294.02 | 4,234,844 | 4,234,844 | 4,233,868 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 3 mod pawel) | 1,000,000 | 223,239 | 240,630 | 240.63 | 4,375,344 | 4,375,344 | 4,375,344 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 4 mod pawel) | 1,000,000 | 223,239 | 216,529 | 216.53 | 4,067,968 | 4,062,849.0 | 4,043,568 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1152 mod vul common) | 1,000,000 | 223,239 | 131,680 | 131.68 | 4,155,796 | 4,155,796 | 4,155,796 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1349 mod vul common) | 1,000,000 | 223,239 | 187,522 | 187.52 | 4,789,968 | 4,789,967.9 | 4,789,968 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1360 mod vul common) | 1,000,000 | 223,239 | 208,064 | 208.06 | 4,290,464 | 4,289,842.9 | 4,289,488 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1360 mod vul common) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 200 mod vul common) | 1,000,000 | 223,239 | 105,580 | 105.58 | 3,883,528 | 3,881,753.5 | 3,881,576 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 200 mod vul common) | 1,000,000 | 223,239 | 134,508 | 134.51 | 3,665,924 | 3,664,317.7 | 3,660,068 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 200 mod vul common) | 1,000,000 | 223,239 | 134,321 | 134.32 | 4,391,920 | 4,391,920 | 4,391,920 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1 mod vul example) | 1,000,000 | 223,239 | 234,594 | 234.59 | 2,443,040 | 2,303,326.2 | 2,055,568 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 2 mod vul example) | 1,000,000 | 223,239 | 223,470 | 223.47 | 3,426,836 | 3,412,147.8 | 3,393,652 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1 mod vul guido) | 1,000,000 | 223,239 | 160,273 | 160.27 | 4,734,360 | 4,731,657.2 | 4,728,504 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 2 mod vul guido) | 1,000,000 | 223,239 | 275,165 | 275.17 | 3,868,896 | 3,866,808.4 | 3,861,088 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 3 mod vul guido) | 1,000,000 | 223,239 | 450,858 | 450.86 | 4,340,232 | 4,326,485.2 | 4,313,880 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1 mod vul marius) | 1,000,000 | 223,239 | 248,234 | 248.23 | 4,025,028 | 4,002,326.5 | 3,981,108 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1 mod vul nagydani) | 1,000,000 | 223,239 | 168,224 | 168.22 | 4,079,680 | 4,079,680 | 4,078,704 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1 mod vul nagydani) | 1,000,000 | 223,239 | 125,235 | 125.23 | 4,822,172 | 4,820,490.0 | 4,817,292 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1 mod vul nagydani) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 2 mod vul nagydani) | 1,000,000 | 223,239 | 174,655 | 174.66 | 4,605,568 | 4,599,146.2 | 4,593,856 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 2 mod vul nagydani) | 1,000,000 | 223,239 | 382,687 | 382.69 | 3,681,540 | 3,680,289.3 | 3,675,684 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 2 mod vul nagydani) | 1,000,000 | 223,239 | 351,660 | 351.66 | 4,539,220 | 4,528,897.6 | 4,516,772 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 3 mod vul nagydani) | 1,000,000 | 223,239 | 166,625 | 166.62 | 4,150,916 | 4,150,177.4 | 4,146,036 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 3 mod vul nagydani) | 1,000,000 | 223,239 | 913,565 | 913.57 | 3,728,384 | 3,722,290.0 | 3,707,888 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 3 mod vul nagydani) | 1,000,000 | 223,239 | 847,039 | 847.04 | 4,376,332 | 4,375,624.1 | 4,367,548 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 4 mod vul nagydani) | 1,000,000 | 223,239 | 158,192 | 158.19 | 4,464,100 | 4,462,730.7 | 4,462,148 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 4 mod vul nagydani) | 1,000,000 | 223,239 | 1,041,130 | 1,041.13 | 4,288,516 | 4,278,387.6 | 4,267,044 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 4 mod vul nagydani) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 5 mod vul nagydani) | 1,000,000 | 223,239 | 144,272 | 144.27 | 4,440,692 | 4,440,692 | 4,440,692 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 5 mod vul nagydani) | 1,000,000 | 223,239 | 1,045,543 | 1,045.54 | 3,451,236 | 3,447,667.0 | 3,434,644 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 5 mod vul nagydani) | 1,000,000 | 223,239 | 959,953 | 959.95 | 3,228,716 | 3,133,973.7 | 2,962,268 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1 mod vul pawel) | 1,000,000 | 223,239 | 476,660 | 476.66 | 4,140,188 | 4,137,614.3 | 4,128,476 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 2 mod vul pawel) | 1,000,000 | 223,239 | 277,992 | 277.99 | 4,388,992 | 4,388,992 | 4,387,040 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 3 mod vul pawel) | 1,000,000 | 223,239 | 231,149 | 231.15 | 3,824,004 | 3,821,505.9 | 3,806,436 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 4 mod vul pawel) | 1,000,000 | 223,239 | 207,168 | 207.17 | 4,623,136 | 4,618,338.3 | 4,607,520 |
| MSIZE (fork Prague, 1M gas, 0 mem size) | 1,000,000 | 223,239 | 16,908 | 16.91 | 4,441,668 | 4,441,668 | 4,440,692 |
| MSIZE (fork Prague, 1M gas, 1000000 mem size) | 1,000,000 | 223,239 | 16,250 | 16.25 | 4,814,364 | 4,814,255.6 | 4,812,412 |
| MSIZE (fork Prague, 1M gas, 100000 mem size) | 1,000,000 | 223,239 | 17,457 | 17.46 | 3,844,500 | 3,841,897.3 | 3,840,596 |
| MSIZE (fork Prague, 1M gas, 1000 mem size) | 1,000,000 | 223,239 | 17,252 | 17.25 | 4,455,316 | 4,455,316 | 4,455,316 |
| MSIZE (fork Prague, 1M gas, 1 mem size) | 1,000,000 | 223,239 | 17,617 | 17.62 | 3,237,500 | 3,235,548 | 3,230,668 |
| PRECOMPILE_BLAKE2F, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 193,313 | 193.31 | 667,720 | 642,141.5 | 487,160 |
| PRECOMPILE_BLS12_MAP_FP_TO_G1, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 26,769 | 26.77 | 4,703,152 | 4,703,152 | 4,703,152 |
| PRECOMPILE_BLS12_MAP_FP2_TO_G2, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 20,407 | 20.41 | 4,706,076 | 4,706,076 | 4,705,100 |
| PRECOMPILE_BLS12_G1ADD, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 20,862 | 20.86 | 3,875,724 | 3,874,910.7 | 3,873,772 |
| PRECOMPILE_BLS12_G1MSM, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 28,144 | 28.14 | 4,157,748 | 4,157,357.6 | 4,154,820 |
| PRECOMPILE_BLS12_G2ADD, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 24,536 | 24.54 | 4,629,968 | 4,629,263.1 | 4,623,136 |
| PRECOMPILE_BLS12_G2MSM, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 22,938 | 22.94 | 3,737,168 | 3,737,168 | 3,734,240 |
| PRECOMPILE_BLS12_PAIRING, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 35,928 | 35.93 | 3,673,732 | 3,673,022.2 | 3,667,876 |
| PRECOMPILE_EC_ADD, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 26,148 | 26.15 | 4,308,028 | 4,308,028 | 4,306,076 |
| CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 25,524 | 25.52 | 4,381,196 | 4,380,708 | 4,380,220 |
| CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 17,748 | 17.75 | 4,475,804 | 4,475,804 | 4,475,804 |
| PRECOMPILE_EC_MUL, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 75,876 | 75.88 | 3,940,136 | 3,940,108.9 | 3,939,160 |
| CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 7,939 | 7.94 | 4,379,244 | 4,379,244 | 4,378,268 |
| CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 73,318 | 73.32 | 4,686,576 | 4,686,576 | 4,685,600 |
| CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 7,277 | 7.28 | 3,608,344 | 3,608,344 | 3,604,440 |
| CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 72,781 | 72.78 | 4,770,452 | 4,770,452 | 4,769,476 |
| CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 6,291 | 6.29 | 4,427,040 | 4,427,040 | 4,426,064 |
| CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 47,767 | 47.77 | 4,193,856 | 4,193,856 | 4,191,904 |
| CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 69,169 | 69.17 | 3,550,772 | 3,549,768.9 | 3,544,916 |
| CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 68,328 | 68.33 | 4,561,660 | 4,559,115.4 | 4,555,804 |
| CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 4,589 | 4.59 | 4,462,148 | 4,462,148 | 4,459,220 |
| PRECOMPILE_ECRECOVER, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 72,669 | 72.67 | 4,091,392 | 4,086,334.5 | 4,079,680 |
| PRECOMPILE_POINT_EVALUATION, CALL, STATICCALL (fork Prague, 1M gas, point evaluation) | 1,000,000 | 223,239 | 78,410 | 78.41 | 3,394,632 | 3,388,731.6 | 3,367,304 |
| PRECOMPILE_IDENTITY, CALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 12,746 | 12.75 | 3,937,208 | 3,937,208 | 3,937,208 |
| PRECOMPILE_RIPEMD-160, CALL (fork Prague, 1M gas, 160) | 1,000,000 | 223,239 | 10,487 | 10.49 | 2,055,568 | 2,043,978 | 2,029,216 |
| PRECOMPILE_SHA2-256, CALL (fork Prague, 1M gas, SHA2, 256) | 1,000,000 | 223,239 | 7,577 | 7.58 | 1,972,608 | 1,970,981.3 | 1,963,824 |
| PUSH0 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 16,893 | 16.89 | 4,216,300 | 4,216,300 | 4,216,300 |
| PUSH10 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 15,158 | 15.16 | 4,472,880 | 4,472,880 | 4,471,904 |
| PUSH11 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 15,638 | 15.64 | 4,375,340 | 4,374,852 | 4,374,364 |
| PUSH12 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 15,860 | 15.86 | 4,577,272 | 4,577,272 | 4,577,272 |
| PUSH13 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 16,095 | 16.09 | 3,538,084 | 3,536,912.8 | 3,531,252 |
| PUSH14 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 16,796 | 16.80 | 4,181,168 | 4,181,168 | 4,174,336 |
| PUSH15 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 16,639 | 16.64 | 4,190,928 | 4,190,928 | 4,186,048 |
| PUSH16 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 17,208 | 17.21 | 4,677,792 | 4,677,792 | 4,677,792 |
| PUSH17 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 17,740 | 17.74 | 4,367,552 | 4,366,753.1 | 4,361,696 |
| PUSH18 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 17,195 | 17.20 | 4,394,848 | 4,394,766.3 | 4,394,848 |
| PUSH19 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 17,583 | 17.58 | 4,394,844 | 4,394,844 | 4,393,868 |
| PUSH1 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 12,293 | 12.29 | 3,780,088 | 3,780,088 | 3,778,136 |
| PUSH20 (fork Prague, 1M gas) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| PUSH21 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 18,523 | 18.52 | 1,933,568 | 1,910,498.9 | 1,892,576 |
| PUSH22 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 18,391 | 18.39 | 3,268,732 | 3,268,146.4 | 3,263,852 |
| PUSH23 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 18,907 | 18.91 | 4,175,312 | 4,175,312 | 4,175,312 |
| PUSH24 (fork Prague, 1M gas) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| PUSH25 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 19,399 | 19.40 | 4,415,336 | 4,415,336 | 4,414,360 |
| PUSH26 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 20,136 | 20.14 | 4,815,340 | 4,814,689.3 | 4,813,388 |
| PUSH27 (fork Prague, 1M gas) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| PUSH28 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 20,974 | 20.97 | 4,549,952 | 4,549,685.8 | 4,546,048 |
| PUSH29 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 20,548 | 20.55 | 3,434,644 | 3,432,016.3 | 3,428,788 |
| PUSH2 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 12,372 | 12.37 | 3,688,368 | 3,686,744.7 | 3,682,516 |
| PUSH30 (fork Prague, 1M gas) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| PUSH31 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 22,096 | 22.10 | 4,466,048 | 4,466,048 | 4,465,072 |
| PUSH32 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 22,592 | 22.59 | 4,607,520 | 4,607,520 | 4,605,568 |
| PUSH3 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 13,420 | 13.42 | 3,943,060 | 3,943,060 | 3,939,156 |
| PUSH4 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 13,839 | 13.84 | 4,349,988 | 4,349,988 | 4,349,988 |
| PUSH5 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 14,119 | 14.12 | 4,700,224 | 4,700,224 | 4,697,296 |
| PUSH6 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 14,331 | 14.33 | 4,593,856 | 4,589,812.6 | 4,587,024 |
| PUSH7 (fork Prague, 1M gas) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| PUSH8 (fork Prague, 1M gas) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| PUSH9 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 14,909 | 14.91 | 4,723,624 | 4,722,736.7 | 4,722,648 |
| RETURN (fork Prague, 1M gas, 1KiB of non, zero data) | 1,000,000 | 223,239 | 17,559 | 17.56 | 1,880,872 | 1,875,260 | 1,862,328 |
| REVERT (fork Prague, 1M gas, 1KiB of non, zero data) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| RETURN (fork Prague, 1M gas, 1KiB of zero data) | 1,000,000 | 223,239 | 20,418 | 20.42 | 4,451,416 | 4,451,416 | 4,450,440 |
| REVERT (fork Prague, 1M gas, 1KiB of zero data) | 1,000,000 | 223,239 | 20,968 | 20.97 | 4,470,928 | 4,470,928 | 4,469,952 |
| RETURN (fork Prague, 1M gas, 1MiB of non, zero data) | 1,000,000 | 223,239 | 5,060 | 5.06 | 4,146,036 | 4,146,036 | 4,146,036 |
| REVERT (fork Prague, 1M gas, 1MiB of non, zero data) | 1,000,000 | 223,239 | 5,334 | 5.33 | 4,799,724 | 4,798,748 | 4,793,868 |
| RETURN (fork Prague, 1M gas, 1MiB of zero data) | 1,000,000 | 223,239 | 4,073 | 4.07 | 4,687,540 | 4,687,540 | 4,686,564 |
| REVERT (fork Prague, 1M gas, 1MiB of zero data) | 1,000,000 | 223,239 | 3,859 | 3.86 | 3,891,336 | 3,891,336 | 3,891,336 |
| RETURN (fork Prague, 1M gas, empty) | 1,000,000 | 223,239 | 24,978 | 24.98 | 4,437,764 | 4,436,997.1 | 4,434,836 |
| REVERT (fork Prague, 1M gas, empty) | 1,000,000 | 223,239 | 27,149 | 27.15 | 4,042,596 | 4,041,131.7 | 4,039,668 |
| PRECOMPILE_IDENTITY (fork Prague, 1M gas, 0 returned size) | 1,000,000 | 223,239 | 14,062 | 14.06 | 4,374,368 | 4,374,368 | 4,374,368 |
| RETURNDATASIZE (fork Prague, 1M gas, 0 returned size) | 1,000,000 | 223,239 | 14,043 | 14.04 | 4,723,628 | 4,723,628 | 4,722,652 |
| RETURNDATASIZE (fork Prague, 1M gas, 0 returned size) | 1,000,000 | 223,239 | 13,770 | 13.77 | 4,733,380 | 4,733,380 | 4,733,380 |
| PRECOMPILE_IDENTITY (fork Prague, 1M gas, 1 returned size) | 1,000,000 | 223,239 | 14,492 | 14.49 | 4,514,820 | 4,511,013.6 | 4,507,988 |
| RETURNDATASIZE (fork Prague, 1M gas, 1 returned size) | 1,000,000 | 223,239 | 14,595 | 14.60 | 3,653,236 | 3,653,236 | 3,653,236 |
| RETURNDATASIZE (fork Prague, 1M gas, 1 returned size) | 1,000,000 | 223,239 | 14,811 | 14.81 | 3,694,224 | 3,694,224 | 3,689,344 |
| RETURNDATASIZE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 13,863 | 13.86 | 4,791,916 | 4,791,916 | 4,790,940 |
| SHL, SHR, SAR (fork Prague, 1M gas, shift right SAR) | 1,000,000 | 223,239 | 22,660 | 22.66 | 3,599,564 | 3,599,564 | 3,597,612 |
| SHL, SHR, SAR (fork Prague, 1M gas, shift right SHR) | 1,000,000 | 223,239 | 21,341 | 21.34 | 4,588,004 | 4,586,540 | 4,585,076 |
| SWAP10 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 22,403 | 22.40 | 1,797,912 | 1,785,907.2 | 1,772,536 |
| SWAP11 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 21,886 | 21.89 | 3,768,396 | 3,768,396 | 3,768,396 |
| SWAP12 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 22,668 | 22.67 | 4,577,276 | 4,575,324 | 4,573,372 |
| SWAP13 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 22,372 | 22.37 | 4,186,048 | 4,185,885.3 | 4,183,120 |
| SWAP14 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 22,283 | 22.28 | 4,793,868 | 4,793,868 | 4,793,868 |
| SWAP15 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 22,505 | 22.50 | 4,707,052 | 4,707,052 | 4,707,052 |
| SWAP16 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 22,643 | 22.64 | 4,585,076 | 4,582,287.4 | 4,581,172 |
| SWAP1 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 22,578 | 22.58 | 4,251,436 | 4,250,338 | 4,247,532 |
| SWAP2 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 22,334 | 22.33 | 3,962,564 | 3,962,564 | 3,962,564 |
| SWAP3 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 22,579 | 22.58 | 4,466,052 | 4,466,052 | 4,464,100 |
| SWAP4 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 22,317 | 22.32 | 3,800,580 | 3,800,384.8 | 3,794,724 |
| SWAP5 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 22,080 | 22.08 | 4,293,392 | 4,293,392 | 4,292,416 |
| SWAP6 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 22,075 | 22.07 | 4,690,468 | 4,690,468 | 4,688,516 |
| SWAP7 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 22,389 | 22.39 | 3,504,908 | 3,503,037 | 3,499,052 |
| SWAP8 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 22,333 | 22.33 | 4,366,572 | 4,366,572 | 4,366,572 |
| SWAP9 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 21,910 | 21.91 | 4,716,800 | 4,716,800 | 4,714,848 |
| TLOAD (fork Prague, 1M gas, val mut: False, key mut: False) | 1,000,000 | 223,239 | 5,957 | 5.96 | 4,704,124 | 4,704,124 | 4,704,124 |
| TLOAD (fork Prague, 1M gas, val mut: False, key mut: True) | 1,000,000 | 223,239 | 5,588 | 5.59 | 3,250,188 | 3,250,188 | 3,244,332 |
| TLOAD (fork Prague, 1M gas, val mut: True, key mut: False) | 1,000,000 | 223,239 | 7,043 | 7.04 | 4,390,944 | 4,390,944 | 4,390,944 |
| TLOAD (fork Prague, 1M gas, val mut: True, key mut: True) | 1,000,000 | 223,239 | 7,048 | 7.05 | 3,792,772 | 3,792,528 | 3,789,844 |
| TSTORE (fork Prague, 1M gas, dense val mut: False, key mut: False) | 1,000,000 | 223,239 | 12,039 | 12.04 | 4,790,940 | 4,790,940 | 4,790,940 |
| TSTORE (fork Prague, 1M gas, dense val mut: False, key mut: True) | 1,000,000 | 223,239 | 12,234 | 12.23 | 4,454,340 | 4,454,340 | 4,452,388 |
| TSTORE (fork Prague, 1M gas, dense val mut: True, key mut: False) | 1,000,000 | 223,239 | 18,187 | 18.19 | 3,473,676 | 3,471,457.8 | 3,467,820 |
| TSTORE (fork Prague, 1M gas, dense val mut: True, key mut: True) | 1,000,000 | 223,239 | 18,038 | 18.04 | 4,447,520 | 4,447,520 | 4,445,568 |
| ISZERO (fork Prague, 1M gas) | 1,000,000 | 223,239 | 23,813 | 23.81 | 4,472,880 | 4,472,880 | 4,472,880 |
| NOT (fork Prague, 1M gas) | 1,000,000 | 223,239 | 12,207 | 12.21 | 4,791,916 | 4,791,916 | 4,791,916 |
| ADDRESS (fork Prague, 1M gas) | 1,000,000 | 223,239 | 25,578 | 25.58 | 4,568,492 | 4,567,666.2 | 4,567,516 |
| BASEFEE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 16,886 | 16.89 | 4,202,636 | 4,202,636 | 4,202,636 |
| BLOBBASEFEE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 21,134 | 21.13 | 1,683,720 | 1,676,533.1 | 1,665,176 |
| CALLER (fork Prague, 1M gas) | 1,000,000 | 223,239 | 25,674 | 25.67 | 4,562,636 | 4,562,636 | 4,561,660 |
| CHAINID (fork Prague, 1M gas) | 1,000,000 | 223,239 | 18,138 | 18.14 | 4,567,516 | 4,567,516 | 4,567,516 |
| CODESIZE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 16,511 | 16.51 | 4,705,100 | 4,705,100 | 4,705,100 |
| COINBASE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 25,536 | 25.54 | 4,479,688 | 4,479,688 | 4,479,688 |
| GASLIMIT (fork Prague, 1M gas) | 1,000,000 | 223,239 | 17,376 | 17.38 | 4,478,728 | 4,478,449.1 | 4,476,776 |
| GASPRICE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 20,085 | 20.09 | 4,445,572 | 4,445,572 | 4,444,596 |
| GAS (fork Prague, 1M gas) | 1,000,000 | 223,239 | 17,654 | 17.65 | 3,854,256 | 3,853,280 | 3,850,352 |
| NUMBER (fork Prague, 1M gas) | 1,000,000 | 223,239 | 17,868 | 17.87 | 4,429,968 | 4,429,073.3 | 4,428,016 |
| ORIGIN (fork Prague, 1M gas) | 1,000,000 | 223,239 | 26,250 | 26.25 | 4,694,368 | 4,694,368 | 4,694,368 |
| PREVRANDAO (fork Prague, 1M gas) | 1,000,000 | 223,239 | 30,777 | 30.78 | 4,380,220 | 4,380,220 | 4,379,244 |
| TIMESTAMP (fork Prague, 1M gas) | 1,000,000 | 223,239 | 17,567 | 17.57 | 3,848,404 | 3,848,404 | 3,846,452 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: False, 0 bytes, call) | 1,000,000 | 223,239 | 18,284 | 18.28 | 4,705,100 | 4,705,100 | 4,705,100 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: False, 0 bytes, transaction) | 1,000,000 | 223,239 | 18,959 | 18.96 | 3,950,860 | 3,947,118.7 | 3,944,028 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: False, 100 bytes, call) | 1,000,000 | 223,239 | 16,923 | 16.92 | 4,262,168 | 4,262,168 | 4,262,168 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: False, 100 bytes, transaction) | 1,000,000 | 223,239 | 16,430 | 16.43 | 3,744,000 | 3,741,490.3 | 3,739,120 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: False, 10KiB, call) | 1,000,000 | 223,239 | 12,518 | 12.52 | 4,445,568 | 4,445,568 | 4,445,568 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: False, 10KiB, transaction) | 1,000,000 | 223,239 | 7,119 | 7.12 | 3,543,940 | 3,542,801.3 | 3,538,084 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: False, 1MiB, call) | 1,000,000 | 223,239 | 3,819 | 3.82 | 4,397,772 | 4,397,772 | 4,396,796 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: False, 1MiB, transaction) | 1,000,000 | 223,239 | 4,653 | 4.65 | 4,381,192 | 4,381,192 | 4,381,192 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: True, 0 bytes, call) | 1,000,000 | 223,239 | 14,745 | 14.74 | 2,474,272 | 2,469,531.4 | 2,463,536 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: True, 0 bytes, transaction) | 1,000,000 | 223,239 | 13,555 | 13.55 | 1,655,420 | 1,653,379.3 | 1,648,588 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: True, 100 bytes, call) | 1,000,000 | 223,239 | 12,230 | 12.23 | 4,309,000 | 4,309,000 | 4,307,048 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: True, 100 bytes, transaction) | 1,000,000 | 223,239 | 11,874 | 11.87 | 4,492,376 | 4,492,376 | 4,491,400 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: True, 10KiB, call) | 1,000,000 | 223,239 | 10,357 | 10.36 | 4,109,936 | 4,109,936 | 4,109,936 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: True, 10KiB, transaction) | 1,000,000 | 223,239 | 7,251 | 7.25 | 4,443,620 | 4,443,620 | 4,443,620 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: True, 1MiB, call) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: True, 1MiB, transaction) | 1,000,000 | 223,239 | 4,098 | 4.10 | 4,730,444 | 4,730,444 | 4,730,444 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: True, fixed src dst: False, 100 bytes, call) | 1,000,000 | 223,239 | 16,754 | 16.75 | 4,104,080 | 4,102,428.3 | 4,102,128 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: True, fixed src dst: False, 100 bytes, transaction) | 1,000,000 | 223,239 | 17,698 | 17.70 | 4,708,016 | 4,708,016 | 4,707,040 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: True, fixed src dst: False, 10KiB, call) | 1,000,000 | 223,239 | 11,394 | 11.39 | 4,301,200 | 4,300,956 | 4,299,248 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: True, fixed src dst: False, 10KiB, transaction) | 1,000,000 | 223,239 | 10,765 | 10.77 | 4,506,036 | 4,504,641.7 | 4,503,108 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: True, fixed src dst: True, 100 bytes, call) | 1,000,000 | 223,239 | 12,954 | 12.95 | 3,899,144 | 3,899,144 | 3,897,192 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: True, fixed src dst: True, 100 bytes, transaction) | 1,000,000 | 223,239 | 12,784 | 12.78 | 748,724 | 739,827.4 | 727,252 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: True, fixed src dst: True, 10KiB, call) | 1,000,000 | 223,239 | 8,688 | 8.69 | 4,341,208 | 4,341,012 | 4,340,232 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: True, fixed src dst: True, 10KiB, transaction) | 1,000,000 | 223,239 | 9,168 | 9.17 | 4,815,340 | 4,815,340 | 4,815,340 |
| CODECOPY (fork Prague, 1M gas, fixed src dst: False, 0 bytes) | 1,000,000 | 223,239 | 17,747 | 17.75 | 3,577,120 | 3,577,120 | 3,571,264 |
| CODECOPY (fork Prague, 1M gas, fixed src dst: False, 0.25x max code size) | 1,000,000 | 223,239 | 12,178 | 12.18 | 4,733,380 | 4,733,380 | 4,733,380 |
| CODECOPY (fork Prague, 1M gas, fixed src dst: False, 0.50x max code size) | 1,000,000 | 223,239 | 10,936 | 10.94 | 4,688,516 | 4,688,516 | 4,687,540 |
| CODECOPY (fork Prague, 1M gas, fixed src dst: False, 0.75x max code size) | 1,000,000 | 223,239 | 11,889 | 11.89 | 4,791,916 | 4,791,916 | 4,791,916 |
| CODECOPY (fork Prague, 1M gas, fixed src dst: False, max code size) | 1,000,000 | 223,239 | 10,533 | 10.53 | 4,202,636 | 4,202,636 | 4,202,636 |
| CODECOPY (fork Prague, 1M gas, fixed src dst: True, 0 bytes) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| CODECOPY (fork Prague, 1M gas, fixed src dst: True, 0.25x max code size) | 1,000,000 | 223,239 | 9,384 | 9.38 | 1,225,988 | 1,224,621.6 | 1,217,204 |
| CODECOPY (fork Prague, 1M gas, fixed src dst: True, 0.50x max code size) | 1,000,000 | 223,239 | 10,402 | 10.40 | 4,310,952 | 4,310,952 | 4,309,976 |
| CODECOPY (fork Prague, 1M gas, fixed src dst: True, 0.75x max code size) | 1,000,000 | 223,239 | 9,595 | 9.60 | 4,732,400 | 4,732,237.3 | 4,732,400 |
| CODECOPY (fork Prague, 1M gas, fixed src dst: True, max code size) | 1,000,000 | 223,239 | 8,743 | 8.74 | 3,612,248 | 3,612,085.3 | 3,609,320 |
| MCOPY (fork Prague, 1M gas, fixed src dst: False, 0 bytes) | 1,000,000 | 223,239 | 18,513 | 18.51 | 4,772,400 | 4,772,400 | 4,772,400 |
| MCOPY (fork Prague, 1M gas, fixed src dst: False, 100 bytes) | 1,000,000 | 223,239 | 17,113 | 17.11 | 4,265,096 | 4,264,998.4 | 4,264,120 |
| MCOPY (fork Prague, 1M gas, fixed src dst: False, 10KiB) | 1,000,000 | 223,239 | 15,310 | 15.31 | 3,939,160 | 3,939,160 | 3,939,160 |
| MCOPY (fork Prague, 1M gas, fixed src dst: False, 1MiB) | 1,000,000 | 223,239 | 4,950 | 4.95 | 4,390,944 | 4,390,944 | 4,388,992 |
| MCOPY (fork Prague, 1M gas, fixed src dst: True, 0 bytes) | 1,000,000 | 223,239 | 13,516 | 13.52 | 4,381,196 | 4,381,196 | 4,381,196 |
| MCOPY (fork Prague, 1M gas, fixed src dst: True, 100 bytes) | 1,000,000 | 223,239 | 13,610 | 13.61 | 4,573,372 | 4,570,304.6 | 4,568,492 |
| MCOPY (fork Prague, 1M gas, fixed src dst: True, 10KiB) | 1,000,000 | 223,239 | 11,256 | 11.26 | 483,256 | 411,379.0 | 250,968 |
| MCOPY (fork Prague, 1M gas, fixed src dst: True, 1MiB) | 1,000,000 | 223,239 | 4,971 | 4.97 | 1,688,600 | 1,687,868 | 1,686,648 |
| RETURNDATACOPY (fork Prague, 1M gas, fixed dst: False, 0 bytes) | 1,000,000 | 223,239 | 17,921 | 17.92 | 1,666,156 | 1,663,002.8 | 1,657,372 |
| RETURNDATACOPY (fork Prague, 1M gas, fixed dst: False, 100 bytes) | 1,000,000 | 223,239 | 15,992 | 15.99 | 3,943,052 | 3,943,052 | 3,943,052 |
| RETURNDATACOPY (fork Prague, 1M gas, fixed dst: False, 10KiB) | 1,000,000 | 223,239 | 10,898 | 10.90 | 4,502,132 | 4,502,132 | 4,500,180 |
| RETURNDATACOPY (fork Prague, 1M gas, fixed dst: False, 1MiB) | 1,000,000 | 223,239 | 6,179 | 6.18 | 4,234,844 | 4,234,844 | 4,231,916 |
| RETURNDATACOPY (fork Prague, 1M gas, fixed dst: True, 0 bytes) | 1,000,000 | 223,239 | 15,866 | 15.87 | 3,906,952 | 3,904,756 | 3,899,144 |
| RETURNDATACOPY (fork Prague, 1M gas, fixed dst: True, 100 bytes) | 1,000,000 | 223,239 | 12,879 | 12.88 | 3,943,056 | 3,942,470.4 | 3,942,080 |
| RETURNDATACOPY (fork Prague, 1M gas, fixed dst: True, 10KiB) | 1,000,000 | 223,239 | 10,113 | 10.11 | 4,256,312 | 4,256,190 | 4,256,312 |
| RETURNDATACOPY (fork Prague, 1M gas, fixed dst: True, 1MiB) | 1,000,000 | 223,239 | 6,999 | 7.00 | 3,969,396 | 3,968,029.6 | 3,963,540 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 0 bytes data, log0) | 1,000,000 | 223,239 | 6,702 | 6.70 | 4,823,148 | 4,823,148 | 4,822,172 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 0 bytes data, log1) | 1,000,000 | 223,239 | 5,818 | 5.82 | 3,881,580 | 3,881,580 | 3,881,580 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 0 bytes data, log2) | 1,000,000 | 223,239 | 5,829 | 5.83 | 4,385,088 | 4,385,088 | 4,384,112 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 0 bytes data, log3) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 0 bytes data, log4) | 1,000,000 | 223,239 | 4,793 | 4.79 | 4,402,652 | 4,402,456.8 | 4,397,772 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB non zero data, log0) | 1,000,000 | 223,239 | 3,994 | 3.99 | 4,547,028 | 4,547,028 | 4,547,028 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB non zero data, log1) | 1,000,000 | 223,239 | 3,932 | 3.93 | 4,077,728 | 4,077,728 | 4,075,776 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB non zero data, log2) | 1,000,000 | 223,239 | 4,927 | 4.93 | 4,685,588 | 4,685,588 | 4,685,588 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB non zero data, log3) | 1,000,000 | 223,239 | 4,401 | 4.40 | 2,609,932 | 2,608,956 | 2,604,076 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB non zero data, log4) | 1,000,000 | 223,239 | 4,440 | 4.44 | 3,779,116 | 3,779,116 | 3,773,268 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB zeros data, log0) | 1,000,000 | 223,239 | 4,187 | 4.19 | 3,625,908 | 3,625,127.2 | 3,618,100 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB zeros data, log1) | 1,000,000 | 223,239 | 5,007 | 5.01 | 3,862,064 | 3,862,064 | 3,856,208 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB zeros data, log2) | 1,000,000 | 223,239 | 4,444 | 4.44 | 4,428,988 | 4,428,988 | 4,428,988 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB zeros data, log3) | 1,000,000 | 223,239 | 4,539 | 4.54 | 4,181,168 | 4,181,168 | 4,181,168 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB zeros data, log4) | 1,000,000 | 223,239 | 4,584 | 4.58 | 4,093,344 | 4,093,344 | 4,091,392 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 0 bytes data, log0) | 1,000,000 | 223,239 | 5,663 | 5.66 | 4,770,456 | 4,770,456 | 4,770,456 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 0 bytes data, log1) | 1,000,000 | 223,239 | 5,441 | 5.44 | 4,380,220 | 4,380,220 | 4,379,244 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 0 bytes data, log2) | 1,000,000 | 223,239 | 5,649 | 5.65 | 4,176,288 | 4,176,288 | 4,176,288 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 0 bytes data, log3) | 1,000,000 | 223,239 | 5,812 | 5.81 | 4,723,628 | 4,723,301.3 | 4,723,628 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 0 bytes data, log4) | 1,000,000 | 223,239 | 4,834 | 4.83 | 3,731,312 | 3,731,312 | 3,728,384 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB non zero data, log0) | 1,000,000 | 223,239 | 4,543 | 4.54 | 4,255,336 | 4,255,336 | 4,254,360 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB non zero data, log1) | 1,000,000 | 223,239 | 4,092 | 4.09 | 4,349,984 | 4,349,984 | 4,349,008 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB non zero data, log2) | 1,000,000 | 223,239 | 4,666 | 4.67 | 3,845,476 | 3,845,476 | 3,844,500 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB non zero data, log3) | 1,000,000 | 223,239 | 3,918 | 3.92 | 4,359,744 | 4,359,744 | 4,357,792 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB non zero data, log4) | 1,000,000 | 223,239 | 4,149 | 4.15 | 3,879,628 | 3,879,628 | 3,876,700 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB zeros data, log0) | 1,000,000 | 223,239 | 4,398 | 4.40 | 3,905,976 | 3,905,650.7 | 3,905,976 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB zeros data, log1) | 1,000,000 | 223,239 | 4,285 | 4.29 | 3,940,136 | 3,940,136 | 3,939,160 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB zeros data, log2) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB zeros data, log3) | 1,000,000 | 223,239 | 4,674 | 4.67 | 3,701,056 | 3,700,568 | 3,695,200 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB zeros data, log4) | 1,000,000 | 223,239 | 4,369 | 4.37 | 4,109,936 | 4,108,960 | 4,104,080 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 0 bytes data, log0) | 1,000,000 | 223,239 | 5,635 | 5.63 | 4,791,916 | 4,791,916 | 4,790,940 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 0 bytes data, log1) | 1,000,000 | 223,239 | 4,359 | 4.36 | 4,426,064 | 4,424,502.4 | 4,424,112 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 0 bytes data, log2) | 1,000,000 | 223,239 | 5,114 | 5.11 | 4,507,988 | 4,507,988 | 4,507,012 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 0 bytes data, log3) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 0 bytes data, log4) | 1,000,000 | 223,239 | 5,003 | 5.00 | 3,651,284 | 3,651,284 | 3,625,908 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB non zero data, log0) | 1,000,000 | 223,239 | 4,080 | 4.08 | 4,256,312 | 4,256,312 | 4,255,336 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB non zero data, log1) | 1,000,000 | 223,239 | 4,261 | 4.26 | 4,507,012 | 4,507,012 | 4,507,012 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB non zero data, log2) | 1,000,000 | 223,239 | 4,517 | 4.52 | 4,719,724 | 4,719,724 | 4,718,748 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB non zero data, log3) | 1,000,000 | 223,239 | 3,892 | 3.89 | 4,681,696 | 4,681,696 | 4,680,720 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB non zero data, log4) | 1,000,000 | 223,239 | 5,049 | 5.05 | 2,028,240 | 2,028,240 | 2,024,336 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB zeros data, log0) | 1,000,000 | 223,239 | 4,497 | 4.50 | 3,850,356 | 3,850,356 | 3,849,380 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB zeros data, log1) | 1,000,000 | 223,239 | 4,926 | 4.93 | 4,568,492 | 4,568,492 | 4,568,492 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB zeros data, log2) | 1,000,000 | 223,239 | 4,077 | 4.08 | 3,806,436 | 3,806,436 | 3,799,604 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB zeros data, log3) | 1,000,000 | 223,239 | 4,699 | 4.70 | 1,463,156 | 1,461,984.8 | 1,456,324 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB zeros data, log4) | 1,000,000 | 223,239 | 4,061 | 4.06 | 3,351,688 | 3,351,688 | 3,346,808 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 0 bytes data, log0) | 1,000,000 | 223,239 | 5,229 | 5.23 | 1,888,676 | 1,887,700 | 1,883,796 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 0 bytes data, log1) | 1,000,000 | 223,239 | 4,802 | 4.80 | 4,218,252 | 4,218,056.8 | 4,216,300 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 0 bytes data, log2) | 1,000,000 | 223,239 | 4,735 | 4.74 | 4,291,440 | 4,291,440 | 4,290,464 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 0 bytes data, log3) | 1,000,000 | 223,239 | 4,686 | 4.69 | 4,306,080 | 4,306,080 | 4,305,104 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 0 bytes data, log4) | 1,000,000 | 223,239 | 4,834 | 4.83 | 4,430,936 | 4,430,288 | 4,429,964 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB non zero data, log0) | 1,000,000 | 223,239 | 4,869 | 4.87 | 3,768,396 | 3,768,396 | 3,768,396 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB non zero data, log1) | 1,000,000 | 223,239 | 4,237 | 4.24 | 3,705,936 | 3,705,936 | 3,704,960 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB non zero data, log2) | 1,000,000 | 223,239 | 4,583 | 4.58 | 4,215,324 | 4,215,324 | 4,214,348 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB non zero data, log3) | 1,000,000 | 223,239 | 4,761 | 4.76 | 4,492,372 | 4,492,372 | 4,491,396 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB non zero data, log4) | 1,000,000 | 223,239 | 4,551 | 4.55 | 4,361,696 | 4,361,696 | 4,360,720 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB zeros data, log0) | 1,000,000 | 223,239 | 4,881 | 4.88 | 2,024,336 | 2,021,733.3 | 2,011,648 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB zeros data, log1) | 1,000,000 | 223,239 | 4,823 | 4.82 | 4,210,444 | 4,210,444 | 4,206,540 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB zeros data, log2) | 1,000,000 | 223,239 | 4,402 | 4.40 | 4,730,444 | 4,730,444 | 4,730,444 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB zeros data, log3) | 1,000,000 | 223,239 | 4,491 | 4.49 | 4,262,168 | 4,259,728 | 4,257,288 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB zeros data, log4) | 1,000,000 | 223,239 | 4,752 | 4.75 | 4,218,252 | 4,218,252 | 4,217,276 |
| BALANCE (fork Prague, 1M gas, absent accounts: False) | 1,000,000 | 223,239 | 9,574 | 9.57 | 4,582,152 | 4,582,152 | 4,581,176 |
| BALANCE (fork Prague, 1M gas, absent accounts: True) | 1,000,000 | 223,239 | 7,928 | 7.93 | 4,507,988 | 4,507,988 | 4,507,988 |
| BALANCE (fork Prague, 1M gas, absent target: False) | 1,000,000 | 223,239 | 11,574 | 11.57 | 4,731,424 | 4,731,424 | 4,731,424 |
| CALLCODE (fork Prague, 1M gas, absent target: False) | 1,000,000 | 223,239 | 25,371 | 25.37 | 4,803,628 | 4,803,628 | 4,803,628 |
| CALL (fork Prague, 1M gas, absent target: False) | 1,000,000 | 223,239 | 25,521 | 25.52 | 4,540,196 | 4,539,382.7 | 4,539,220 |
| DELEGATECALL (fork Prague, 1M gas, absent target: False) | 1,000,000 | 223,239 | 23,021 | 23.02 | 4,159,700 | 4,159,374.7 | 4,157,748 |
| EXTCODEHASH (fork Prague, 1M gas, absent target: False) | 1,000,000 | 223,239 | 11,591 | 11.59 | 3,358,520 | 3,357,544 | 3,351,688 |
| EXTCODESIZE (fork Prague, 1M gas, absent target: False) | 1,000,000 | 223,239 | 11,522 | 11.52 | 4,429,964 | 4,429,964 | 4,428,988 |
| STATICCALL (fork Prague, 1M gas, absent target: False) | 1,000,000 | 223,239 | 25,035 | 25.04 | 4,793,868 | 4,793,868 | 4,792,892 |
| BALANCE (fork Prague, 1M gas, absent target: True) | 1,000,000 | 223,239 | 12,243 | 12.24 | 3,565,412 | 3,561,386 | 3,555,652 |
| CALLCODE (fork Prague, 1M gas, absent target: True) | 1,000,000 | 223,239 | 24,241 | 24.24 | 1,834,024 | 1,821,754.3 | 1,799,864 |
| CALL (fork Prague, 1M gas, absent target: True) | 1,000,000 | 223,239 | 23,285 | 23.29 | 4,722,652 | 4,722,652 | 4,719,724 |
| DELEGATECALL (fork Prague, 1M gas, absent target: True) | 1,000,000 | 223,239 | 20,794 | 20.79 | 4,226,060 | 4,224,324.9 | 4,224,108 |
| EXTCODEHASH (fork Prague, 1M gas, absent target: True) | 1,000,000 | 223,239 | 13,660 | 13.66 | 4,494,324 | 4,494,324 | 4,492,372 |
| EXTCODESIZE (fork Prague, 1M gas, absent target: True) | 1,000,000 | 223,239 | 12,338 | 12.34 | 2,943,724 | 2,938,681.3 | 2,933,964 |
| STATICCALL (fork Prague, 1M gas, absent target: True) | 1,000,000 | 223,239 | 22,786 | 22.79 | 4,451,424 | 4,450,838.4 | 4,447,520 |
| BLOCKHASH (fork Prague, 1M gas) | 1,000,000 | 223,239 | 15,067 | 15.07 | 4,725,576 | 4,725,576 | 4,725,576 |
| EXTCODECOPY (fork Prague, 1M gas, 1KiB) | 1,000,000 | 223,239 | 10,152 | 10.15 | 726,276 | 718,305.3 | 701,876 |
| EXTCODECOPY (fork Prague, 1M gas, 512) | 1,000,000 | 223,239 | 10,569 | 10.57 | 4,495,300 | 4,495,300 | 4,494,324 |
| EXTCODECOPY (fork Prague, 1M gas, 5KiB) | 1,000,000 | 223,239 | 9,422 | 9.42 | 4,414,360 | 4,414,360 | 4,409,480 |
| SELFBALANCE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 75,664 | 75.66 | 3,788,872 | 3,782,932.3 | 3,780,088 |
| SELFDESTRUCT (fork Prague, 1M gas, value bearing: False) | 965,720 | 223,239 | 5,588 | 5.59 | 3,768,396 | 3,768,396 | 3,765,468 |
| SELFDESTRUCT (fork Prague, 1M gas, value bearing: True) | 965,720 | 223,239 | 4,858 | 4.86 | 4,309,976 | 4,309,976 | 4,309,976 |
| SELFDESTRUCT (fork Prague, 1M gas, value bearing: False) | 998,771 | 223,239 | 7,579 | 7.58 | 3,890,360 | 3,889,384 | 3,884,504 |
| SELFDESTRUCT (fork Prague, 1M gas, value bearing: True) | 998,771 | 223,239 | 10,107 | 10.11 | 3,604,440 | 3,602,976 | 3,598,584 |
| SELFDESTRUCT (fork Prague, 1M gas, value bearing: False) | 989,164 | 223,239 | 4,181 | 4.18 | 4,772,400 | 4,772,400 | 4,772,400 |
| SELFDESTRUCT (fork Prague, 1M gas, value bearing: True) | 989,164 | 223,239 | 4,658 | 4.66 | 4,450,444 | 4,450,199 | 4,450,444 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: False, SSLOAD) | 999,749 | 223,239 | 9,829 | 9.83 | 3,759,616 | 3,759,616 | 3,751,808 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: False) | 1,000,000 | 223,239 | 8,081 | 8.08 | 4,375,344 | 4,375,344 | 4,375,344 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: False) | 998,963 | 223,239 | 7,331 | 7.33 | 4,381,188 | 4,381,188 | 4,381,188 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: False, SSTORE new value) | 998,957 | 223,239 | 7,462 | 7.46 | 3,707,888 | 3,707,888 | 3,705,936 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: False) | 1,000,000 | 223,239 | 11,195 | 11.20 | 4,737,276 | 4,736,544 | 4,731,420 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: False) | 999,015 | 223,239 | 10,831 | 10.83 | 4,817,292 | 4,817,292 | 4,816,316 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: False, SSTORE same value) | 999,009 | 223,239 | 11,536 | 11.54 | 904,884 | 881,643 | 847,300 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: True, SSLOAD) | 999,749 | N/A | N/A | N/A | N/A | N/A | N/A |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: True) | 1,000,000 | 223,239 | 4,581 | 4.58 | 3,428,788 | 3,428,788 | 3,426,836 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: True) | 995,213 | 223,239 | 4,679 | 4.68 | 759,460 | 756,044 | 749,700 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: True, SSTORE new value) | 995,207 | 223,239 | 4,785 | 4.79 | 3,244,332 | 3,244,006.7 | 3,239,452 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: True) | 1,000,000 | 223,239 | 4,508 | 4.51 | 4,809,484 | 4,809,484 | 4,808,508 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: True) | 995,081 | 223,239 | 4,754 | 4.75 | 4,700,224 | 4,700,224 | 4,700,224 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: True, SSTORE same value) | 995,075 | N/A | N/A | N/A | N/A | N/A | N/A |
| SLOAD, SSTORE (fork Prague, 1M gas, SLOAD) | 1,000,000 | 223,239 | 11,261 | 11.26 | 701,880 | 695,308.1 | 676,504 |
| SLOAD, SSTORE (fork Prague, 1M gas, SSTORE new value) | 1,000,000 | 223,239 | 20,416 | 20.42 | 4,375,352 | 4,375,352 | 4,375,352 |
| SLOAD, SSTORE (fork Prague, 1M gas, SSTORE same value) | 1,000,000 | 223,239 | 16,440 | 16.44 | 3,958,660 | 3,956,870.7 | 3,952,804 |

## Summary Statistics

- **Total Tests:** 508
- **Successful Tests:** 486
- **Failed Tests:** 22

### Proving Time (ms)
- **Average:** 64,506.1
- **Minimum:** 3,819
- **Maximum:** 1,045,543

### Peak Memory Usage (MB)
- **Average:** 3,970,303.1
- **Minimum:** 483,256
- **Maximum:** 4,831,932

### Proof Size (bytes)
- **Average:** 223,239
- **Minimum:** 223,239
- **Maximum:** 223,239
