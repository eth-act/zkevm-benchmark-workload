# zkEVM Benchmark Results

Generated on: 2025-11-26 13:07:24

## Folder: zkevm-metrics-risc0-1M

**zkVM:** risc0-v3.0.3

**Hardware Configuration:** CPU: AMD EPYC 7B13 64-Core Processor | RAM: 396 GiB | GPU: NVIDIA GeForce RTX 4090, NVIDIA GeForce RTX 4090, NVIDIA GeForce RTX 4090, NVIDIA GeForce RTX 4090

## Proving Metrics

| Benchmark | Gas Used | Proof Size (bytes) | Proving Time (ms) | Proving Time (s) | Peak Memory (MB) | Avg Memory (MB) | Initial Memory (MB) |
|---|---|---|---|---|---|---|---|
| SLOAD, SSTORE, BALANCE, EXTCODESIZE, EXTCODEHASH, CALLDATALOAD, CALLDATACOPY, CALLDATASIZE (fork Prague, 1M gas) | 999,980 | 223,239 | 9,319 | 9.32 | 2,594,728 | 2,594,498.4 | 2,589,848 |
| CALLDATALOAD, CALLDATACOPY, CALLDATASIZE (fork Prague, 1M gas, zero byte: True) | 1,000,000 | 223,239 | 13,337 | 13.34 | 2,850,388 | 2,850,062.7 | 2,847,460 |
| CALL (fork Prague, 1M gas, case id a to a) | 987,000 | 223,239 | 27,636 | 27.64 | 2,739,144 | 2,739,144 | 2,739,144 |
| CALL (fork Prague, 1M gas, case id a to b) | 987,000 | 223,239 | 24,194 | 24.19 | 3,287,488 | 3,287,488 | 3,286,512 |
| CALL (fork Prague, 1M gas) | 987,000 | 223,239 | 29,730 | 29.73 | 2,787,936 | 2,787,913.2 | 2,786,960 |
| CALL (fork Prague, 1M gas) | 987,000 | 223,239 | 30,677 | 30.68 | 3,071,908 | 3,071,908 | 3,069,956 |
| CALLCODE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 394,593 | 394.59 | 2,691,332 | 2,690,032.0 | 2,679,620 |
| CALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 389,736 | 389.74 | 2,216,080 | 2,211,706.2 | 2,184,848 |
| DELEGATECALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 327,463 | 327.46 | 3,251,436 | 3,251,436 | 3,248,508 |
| EXTCODEHASH (fork Prague, 1M gas) | 1,000,000 | 223,239 | 391,661 | 391.66 | 2,560,568 | 2,560,449.0 | 2,550,808 |
| CREATE2 (fork Prague, 1M gas, 0 bytes) | 1,000,000 | 223,239 | 6,927 | 6.93 | 2,233,648 | 2,232,672 | 2,224,864 |
| CREATE (fork Prague, 1M gas, 0 bytes) | 1,000,000 | 223,239 | 6,483 | 6.48 | 707,204 | 705,876.6 | 695,492 |
| CREATE (fork Prague, 1M gas, 0 bytes) | 1,000,000 | 223,239 | 5,763 | 5.76 | 3,176,292 | 3,176,048 | 3,174,340 |
| CREATE2 (fork Prague, 1M gas, zero data) | 1,000,000 | 223,239 | 7,203 | 7.20 | 877,996 | 871,094.3 | 853,596 |
| CREATE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 6,093 | 6.09 | 2,612,284 | 2,612,067.1 | 2,608,380 |
| CREATE2 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 6,865 | 6.87 | 2,804,520 | 2,802,177.6 | 2,800,616 |
| CREATE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 6,622 | 6.62 | 2,439,556 | 2,438,934.5 | 2,433,700 |
| CREATE2 (fork Prague, 1M gas, zero data) | 1,000,000 | 223,239 | 8,304 | 8.30 | 1,842,280 | 1,840,653.3 | 1,837,400 |
| CREATE (fork Prague, 1M gas, zero data) | 1,000,000 | 223,239 | 6,004 | 6.00 | 3,141,160 | 3,141,160 | 3,138,232 |
| CREATE2 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 6,875 | 6.88 | 2,390,764 | 2,390,764 | 2,390,764 |
| CREATE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 6,447 | 6.45 | 2,603,508 | 2,603,418.9 | 2,603,508 |
| CREATE (fork Prague, 1M gas, zero data) | 1,000,000 | 223,239 | 8,312 | 8.31 | 2,397,592 | 2,397,592 | 2,397,592 |
| CREATE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 5,675 | 5.67 | 3,275,788 | 3,275,788 | 3,274,812 |
| JUMPDEST, JUMP (fork Prague, 1M gas, 00) | 1,000,000 | 223,239 | 26,002 | 26.00 | 3,004,568 | 3,004,513.8 | 3,002,616 |
| JUMPDEST, JUMP (fork Prague, 1M gas, 605b5b) | 1,000,000 | 223,239 | 26,430 | 26.43 | 1,032,204 | 1,009,487.6 | 968,764 |
| JUMPDEST, JUMP (fork Prague, 1M gas, 605b) | 1,000,000 | 223,239 | 16,698 | 16.70 | 3,259,240 | 3,259,240 | 3,259,240 |
| JUMPDEST, JUMP (fork Prague, 1M gas, 615b5b5b) | 1,000,000 | 223,239 | 22,315 | 22.32 | 968,764 | 954,899.1 | 921,916 |
| PRECOMPILE_EC_PAIRING, CALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 291,410 | 291.41 | 2,767,440 | 2,766,846.4 | 2,760,608 |
| UNKNOWN (fork Prague, 1M gas) | 0 | 223,239 | 4,617 | 4.62 | 2,331,240 | 2,331,240 | 2,331,240 |
| AND (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 32,652 | 32.65 | 3,059,224 | 3,057,058.5 | 3,052,392 |
| DIV (fork Prague, 1M gas, 1) | 1,000,000 | 223,239 | 170,049 | 170.05 | 3,186,052 | 3,185,869.7 | 3,183,124 |
| EQ (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 82,915 | 82.92 | 2,167,284 | 2,167,105.1 | 2,159,476 |
| EXP (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 127,391 | 127.39 | 2,616,188 | 2,616,188 | 2,612,284 |
| GT (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 29,183 | 29.18 | 3,266,040 | 3,266,040 | 3,265,064 |
| MOD (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 51,342 | 51.34 | 3,268,956 | 3,268,956 | 3,268,956 |
| MUL (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 64,340 | 64.34 | 3,103,128 | 3,103,057.3 | 3,097,272 |
| SAR (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 78,839 | 78.84 | 2,910,888 | 2,910,471.2 | 2,901,128 |
| SDIV (fork Prague, 1M gas, 1) | 1,000,000 | 223,239 | 251,953 | 251.95 | 1,774,936 | 1,761,792.2 | 1,723,208 |
| SLT (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 35,935 | 35.94 | 3,117,748 | 3,117,748 | 3,117,748 |
| XOR (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 27,832 | 27.83 | 3,197,756 | 3,197,365.6 | 3,196,780 |
| BLOBHASH (fork Prague, 1M gas, no blobs) | 1,000,000 | 223,239 | 37,313 | 37.31 | 2,576,184 | 2,576,066.9 | 2,573,256 |
| BLOBHASH (fork Prague, 1M gas) | 1,000,000 | 223,239 | 55,889 | 55.89 | 1,328,908 | 1,317,125.3 | 1,295,724 |
| BLOBHASH (fork Prague, 1M gas) | 1,000,000 | 223,239 | 56,158 | 56.16 | 2,219,984 | 2,219,633.6 | 2,216,080 |
| CALLDATALOAD (fork Prague, 1M gas, empty) | 1,000,000 | 223,239 | 64,591 | 64.59 | 3,203,612 | 3,201,538 | 3,197,756 |
| CALLDATALOAD (fork Prague, 1M gas, zero, loop) | 1,000,000 | 223,239 | 93,894 | 93.89 | 2,692,304 | 2,692,304 | 2,692,304 |
| CALLDATASIZE (fork Prague, 1M gas, 1000 calldata length) | 1,000,000 | 223,239 | 34,984 | 34.98 | 2,743,044 | 2,743,044 | 2,742,068 |
| DUP10 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 26,174 | 26.17 | 3,271,884 | 3,271,884 | 3,271,884 |
| DUP11 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 32,244 | 32.24 | 2,820,136 | 2,820,136 | 2,817,208 |
| DUP13 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 31,732 | 31.73 | 2,796,716 | 2,796,576.6 | 2,786,956 |
| DUP15 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 31,836 | 31.84 | 2,184,848 | 2,184,418.6 | 2,176,064 |
| DUP1 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 26,886 | 26.89 | 3,275,788 | 3,275,788 | 3,275,788 |
| DUP4 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 31,506 | 31.51 | 2,748,896 | 2,748,733.3 | 2,742,064 |
| DUP5 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 30,593 | 30.59 | 3,107,032 | 3,107,032 | 3,102,152 |
| DUP6 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 32,476 | 32.48 | 2,334,164 | 2,333,263.1 | 2,330,260 |
| DUP7 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 31,509 | 31.51 | 2,573,256 | 2,570,982.1 | 2,560,568 |
| DUP8 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 27,502 | 27.50 | 3,268,960 | 3,268,960 | 3,268,960 |
| DUP9 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 31,295 | 31.30 | 2,237,548 | 2,236,454.9 | 2,233,644 |
| JUMPI (fork Prague, 1M gas) | 1,000,000 | 223,239 | 46,124 | 46.12 | 1,793,480 | 1,789,942 | 1,775,912 |
| JUMPI (fork Prague, 1M gas) | 1,000,000 | 223,239 | 28,931 | 28.93 | 3,001,640 | 3,001,640 | 3,001,640 |
| MLOAD (fork Prague, 1M gas, big memory expansion: False, offset initialized: False, 0 offset) | 1,000,000 | 223,239 | 47,163 | 47.16 | 2,509,820 | 2,509,754.9 | 2,503,964 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: False, offset initialized: False, 0 offset) | 1,000,000 | 223,239 | 32,706 | 32.71 | 1,857,896 | 1,853,899.0 | 1,843,256 |
| MSTORE (fork Prague, 1M gas, big memory expansion: False, offset initialized: False, 0 offset) | 1,000,000 | 223,239 | 49,592 | 49.59 | 1,084,908 | 1,062,888.8 | 1,032,204 |
| MLOAD (fork Prague, 1M gas, big memory expansion: False, offset initialized: False, 1 offset) | 1,000,000 | 223,239 | 48,072 | 48.07 | 3,002,616 | 3,002,616 | 3,002,616 |
| MLOAD (fork Prague, 1M gas, big memory expansion: False, offset initialized: False, 31 offset) | 1,000,000 | 223,239 | 41,188 | 41.19 | 3,263,132 | 3,263,132 | 3,262,156 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: False, offset initialized: False, 31 offset) | 1,000,000 | 223,239 | 31,903 | 31.90 | 1,982,824 | 1,967,068.6 | 1,952,568 |
| MSTORE (fork Prague, 1M gas, big memory expansion: False, offset initialized: False, 31 offset) | 1,000,000 | 223,239 | 48,883 | 48.88 | 2,640,584 | 2,640,106.4 | 2,638,632 |
| MLOAD (fork Prague, 1M gas, big memory expansion: False, offset initialized: True, 0 offset) | 1,000,000 | 223,239 | 47,227 | 47.23 | 2,327,336 | 2,327,140.8 | 2,319,528 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: False, offset initialized: True, 0 offset) | 1,000,000 | 223,239 | 27,717 | 27.72 | 3,242,652 | 3,242,652 | 3,242,652 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: False, offset initialized: True, 1 offset) | 1,000,000 | 223,239 | 32,803 | 32.80 | 2,933,332 | 2,931,629.2 | 2,931,380 |
| MSTORE (fork Prague, 1M gas, big memory expansion: False, offset initialized: True, 1 offset) | 1,000,000 | 223,239 | 40,053 | 40.05 | 3,117,748 | 3,117,748 | 3,115,796 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: True, offset initialized: False, 0 offset) | 1,000,000 | 223,239 | 33,090 | 33.09 | 1,939,880 | 1,931,974.4 | 1,916,456 |
| MSTORE (fork Prague, 1M gas, big memory expansion: True, offset initialized: False, 0 offset) | 1,000,000 | 223,239 | 48,450 | 48.45 | 2,491,276 | 2,491,276 | 2,490,300 |
| MLOAD (fork Prague, 1M gas, big memory expansion: True, offset initialized: False, 1 offset) | 1,000,000 | 223,239 | 48,505 | 48.51 | 3,014,328 | 3,011,949 | 3,008,472 |
| MSTORE (fork Prague, 1M gas, big memory expansion: True, offset initialized: False, 1 offset) | 1,000,000 | 223,239 | 47,890 | 47.89 | 2,603,508 | 2,601,666.0 | 2,594,724 |
| MLOAD (fork Prague, 1M gas, big memory expansion: True, offset initialized: False, 31 offset) | 1,000,000 | 223,239 | 47,434 | 47.43 | 2,390,764 | 2,390,431.3 | 2,385,884 |
| MSTORE (fork Prague, 1M gas, big memory expansion: True, offset initialized: False, 31 offset) | 1,000,000 | 223,239 | 48,627 | 48.63 | 2,976,268 | 2,976,185.3 | 2,970,412 |
| MLOAD (fork Prague, 1M gas, big memory expansion: True, offset initialized: True, 0 offset) | 1,000,000 | 223,239 | 47,724 | 47.72 | 842,864 | 820,281.0 | 794,064 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: True, offset initialized: True, 0 offset) | 1,000,000 | 223,239 | 27,131 | 27.13 | 3,274,812 | 3,274,742.3 | 3,271,884 |
| MLOAD (fork Prague, 1M gas, big memory expansion: True, offset initialized: True, 1 offset) | 1,000,000 | 223,239 | 38,766 | 38.77 | 3,125,556 | 3,125,464.5 | 3,119,700 |
| MLOAD (fork Prague, 1M gas, big memory expansion: True, offset initialized: True, 31 offset) | 1,000,000 | 223,239 | 46,940 | 46.94 | 2,883,568 | 2,883,196.2 | 2,879,664 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: True, offset initialized: True, 31 offset) | 1,000,000 | 223,239 | 31,947 | 31.95 | 2,952,848 | 2,952,437.1 | 2,946,992 |
| MSTORE (fork Prague, 1M gas, big memory expansion: True, offset initialized: True, 31 offset) | 1,000,000 | 223,239 | 48,674 | 48.67 | 2,488,348 | 2,487,786.3 | 2,482,492 |
| MOD, SMOD (fork Prague, 1M gas, op MOD, 63 mod bits) | 1,000,000 | 223,239 | 138,392 | 138.39 | 2,496,156 | 2,495,950.7 | 2,491,276 |
| MOD, SMOD (fork Prague, 1M gas, op SMOD, 191 mod bits) | 1,000,000 | 223,239 | 273,410 | 273.41 | 1,430,408 | 1,384,181.4 | 1,329,880 |
| MOD, SMOD (fork Prague, 1M gas, op SMOD, 255 mod bits) | 1,000,000 | 223,239 | 202,537 | 202.54 | 2,523,484 | 2,522,313.9 | 2,510,796 |
| ADDMOD, MULMOD (fork Prague, 1M gas, op ADDMOD, 127 mod bits) | 1,000,000 | 223,239 | 136,849 | 136.85 | 3,113,856 | 3,113,832.5 | 3,109,952 |
| ADDMOD, MULMOD (fork Prague, 1M gas, op ADDMOD, 191 mod bits) | 1,000,000 | 223,239 | 160,906 | 160.91 | 3,128,476 | 3,128,076.3 | 3,127,500 |
| ADDMOD, MULMOD (fork Prague, 1M gas, op ADDMOD, 255 mod bits) | 1,000,000 | 223,239 | 162,894 | 162.89 | 2,427,848 | 2,427,822.7 | 2,421,016 |
| ADDMOD, MULMOD (fork Prague, 1M gas, op ADDMOD, 63 mod bits) | 1,000,000 | 223,239 | 94,067 | 94.07 | 3,128,476 | 3,128,476 | 3,128,476 |
| ADDMOD, MULMOD (fork Prague, 1M gas, op MULMOD, 127 mod bits) | 1,000,000 | 223,239 | 283,789 | 283.79 | 2,275,608 | 2,264,692.7 | 2,243,400 |
| ADDMOD, MULMOD (fork Prague, 1M gas, op MULMOD, 191 mod bits) | 1,000,000 | 223,239 | 335,931 | 335.93 | 3,183,124 | 3,178,909.0 | 3,175,316 |
| ADDMOD, MULMOD (fork Prague, 1M gas, op MULMOD, 255 mod bits) | 1,000,000 | 223,239 | 381,740 | 381.74 | 1,506,536 | 1,476,687.1 | 1,430,408 |
| ADDMOD, MULMOD (fork Prague, 1M gas, op MULMOD, 63 mod bits) | 1,000,000 | 223,239 | 219,634 | 219.63 | 2,421,016 | 2,420,839.5 | 2,414,184 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1360 mod) | 1,000,000 | 223,239 | 1,026,234 | 1,026.23 | 2,890,396 | 2,890,396 | 2,890,396 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 408 mod) | 1,000,000 | 223,239 | 761,007 | 761.01 | 3,267,984 | 3,267,088.6 | 3,266,032 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 600 mod) | 1,000,000 | 223,239 | 913,363 | 913.36 | 2,847,460 | 2,843,817.2 | 2,840,628 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 600 mod) | 1,000,000 | 223,239 | 1,368,908 | 1,368.91 | 2,526,408 | 2,525,638.8 | 2,522,504 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 677 mod) | 1,000,000 | 223,239 | 985,138 | 985.14 | 2,969,436 | 2,965,906.4 | 2,951,868 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 128 mod even) | 1,000,000 | 223,239 | 690,538 | 690.54 | 3,243,628 | 3,239,389.2 | 3,218,252 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 16 mod even) | 1,000,000 | 223,239 | 1,357,282 | 1,357.28 | 3,094,352 | 3,094,085.8 | 3,085,568 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 32 mod even) | 1,000,000 | 223,239 | 1,054,206 | 1,054.21 | 2,317,576 | 2,307,138.9 | 2,275,608 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 512 mod even) | 1,000,000 | 223,239 | 6,585 | 6.58 | 2,243,404 | 2,242,567.4 | 2,237,548 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 64 mod even) | 1,000,000 | 223,239 | 908,737 | 908.74 | 2,413,208 | 2,412,157.1 | 2,398,568 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 8 mod even) | 1,000,000 | 223,239 | 2,281,888 | 2,281.89 | 2,550,808 | 2,549,779.6 | 2,534,216 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 208 mod exp) | 1,000,000 | 223,239 | 678,173 | 678.17 | 3,279,684 | 3,278,605.9 | 3,276,756 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 298 mod exp) | 1,000,000 | 223,239 | 1,877,381 | 1,877.38 | 3,267,036 | 3,265,675.0 | 3,264,108 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, mod min as base heavy) | 1,000,000 | 223,239 | 3,078,726 | 3,078.73 | 3,078,736 | 3,077,919.4 | 3,075,808 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1024 mod odd) | 1,000,000 | 223,239 | 6,379 | 6.38 | 853,596 | 851,418.8 | 843,836 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 128 mod odd) | 1,000,000 | 223,239 | 820,295 | 820.29 | 2,651,316 | 2,649,580.3 | 2,639,604 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 256 mod odd) | 1,000,000 | 223,239 | 492,093 | 492.09 | 3,137,256 | 3,130,686.4 | 3,127,496 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 32 mod odd) | 1,000,000 | 223,239 | 1,052,249 | 1,052.25 | 1,666,600 | 1,612,631.4 | 1,512,392 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 32 mod odd) | 1,000,000 | 223,239 | 751,167 | 751.17 | 3,098,256 | 3,097,621.9 | 3,095,328 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 512 mod odd) | 1,000,000 | 223,239 | 5,225 | 5.22 | 3,216,300 | 3,215,128.8 | 3,214,348 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 64 mod odd) | 1,000,000 | 223,239 | 897,579 | 897.58 | 2,609,360 | 2,609,005.5 | 2,603,504 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 2 mod pawel) | 1,000,000 | 223,239 | 1,098,648 | 1,098.65 | 3,172,388 | 3,160,062.7 | 3,140,180 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 4 mod pawel) | 1,000,000 | 223,239 | 948,608 | 948.61 | 3,052,392 | 3,050,860.7 | 3,045,560 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1152 mod vul common) | 1,000,000 | 223,239 | 568,672 | 568.67 | 3,097,276 | 3,097,276 | 3,097,276 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1360 mod vul common) | 1,000,000 | 223,239 | 763,049 | 763.05 | 3,249,484 | 3,249,484 | 3,247,532 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1360 mod vul common) | 1,000,000 | 223,239 | 819,730 | 819.73 | 2,878,688 | 2,876,530.9 | 2,860,144 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 200 mod vul common) | 1,000,000 | 223,239 | 443,964 | 443.96 | 2,929,428 | 2,924,697.3 | 2,916,740 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 200 mod vul common) | 1,000,000 | 223,239 | 577,112 | 577.11 | 2,692,304 | 2,692,304 | 2,692,304 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1 mod vul example) | 1,000,000 | 223,239 | 1,028,921 | 1,028.92 | 2,160,452 | 2,139,080.1 | 2,027,716 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 2 mod vul example) | 1,000,000 | 223,239 | 976,261 | 976.26 | 2,452,236 | 2,452,086.9 | 2,450,284 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 2 mod vul guido) | 1,000,000 | 223,239 | 1,219,499 | 1,219.50 | 2,889,420 | 2,888,313.9 | 2,885,516 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 3 mod vul guido) | 1,000,000 | 223,239 | 1,719,570 | 1,719.57 | 3,264,108 | 3,263,216.5 | 3,263,132 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1 mod vul marius) | 1,000,000 | 223,239 | 1,096,325 | 1,096.33 | 3,037,752 | 3,028,202.0 | 3,015,304 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1 mod vul nagydani) | 1,000,000 | 223,239 | 728,858 | 728.86 | 3,070,936 | 3,068,472.1 | 3,061,176 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1 mod vul nagydani) | 1,000,000 | 223,239 | 515,055 | 515.05 | 1,287,916 | 1,224,186.4 | 1,085,884 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 2 mod vul nagydani) | 1,000,000 | 223,239 | 1,710,897 | 1,710.90 | 2,703,040 | 2,702,670.9 | 2,700,112 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 3 mod vul nagydani) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 3 mod vul nagydani) | 1,000,000 | 223,239 | 4,092,095 | 4,092.09 | 2,740,124 | 2,737,371.6 | 2,722,556 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 3 mod vul nagydani) | 1,000,000 | 223,239 | 3,200,980 | 3,200.98 | 3,286,512 | 3,285,556.0 | 3,280,656 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 4 mod vul nagydani) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 4 mod vul nagydani) | 1,000,000 | 223,239 | 4,222,531 | 4,222.53 | 2,677,668 | 2,674,612.2 | 2,659,124 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 5 mod vul nagydani) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 5 mod vul nagydani) | 1,000,000 | 223,239 | 4,247,664 | 4,247.66 | 2,363,444 | 2,358,833.9 | 2,334,164 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1 mod vul pawel) | 1,000,000 | 223,239 | 2,109,591 | 2,109.59 | 3,085,568 | 3,085,412.6 | 3,077,760 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 3 mod vul pawel) | 1,000,000 | 223,239 | 996,610 | 996.61 | 2,839,656 | 2,837,120.4 | 2,828,920 |
| MSIZE (fork Prague, 1M gas, 100000 mem size) | 1,000,000 | 223,239 | 49,922 | 49.92 | 2,849,412 | 2,849,412 | 2,849,412 |
| MSIZE (fork Prague, 1M gas, 1 mem size) | 1,000,000 | 223,239 | 48,787 | 48.79 | 2,373,200 | 2,372,307.1 | 2,362,464 |
| PRECOMPILE_BLAKE2F, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| PRECOMPILE_BLS12_G1ADD, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 63,417 | 63.42 | 2,898,204 | 2,896,870.1 | 2,891,372 |
| PRECOMPILE_BLS12_G1MSM, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 96,590 | 96.59 | 3,098,252 | 3,098,252 | 3,097,276 |
| PRECOMPILE_BLS12_G2MSM, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 68,955 | 68.95 | 2,742,068 | 2,741,915.8 | 2,738,164 |
| PRECOMPILE_BLS12_PAIRING, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 127,227 | 127.23 | 2,701,088 | 2,701,021.6 | 2,692,304 |
| PRECOMPILE_EC_ADD, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 71,986 | 71.99 | 3,261,184 | 3,261,184 | 3,261,184 |
| PRECOMPILE_EC_MUL, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 312,262 | 312.26 | 2,977,244 | 2,975,494.7 | 2,975,292 |
| CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 11,592 | 11.59 | 2,627,900 | 2,627,900 | 2,623,020 |
| CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 157,970 | 157.97 | 3,116,776 | 3,116,776 | 3,116,776 |
| CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 289,463 | 289.46 | 2,589,848 | 2,588,510.4 | 2,585,944 |
| PRECOMPILE_ECRECOVER, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 294,874 | 294.87 | 3,069,956 | 3,069,956 | 3,069,956 |
| PRECOMPILE_POINT_EVALUATION, CALL, STATICCALL (fork Prague, 1M gas, point evaluation) | 1,000,000 | 223,239 | 325,230 | 325.23 | 2,451,264 | 2,450,144.5 | 2,438,576 |
| PRECOMPILE_IDENTITY, CALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 26,810 | 26.81 | 2,976,268 | 2,976,268 | 2,976,268 |
| PRECOMPILE_RIPEMD-160, CALL (fork Prague, 1M gas, 160) | 1,000,000 | 223,239 | 18,575 | 18.57 | 2,024,792 | 2,020,888 | 2,001,368 |
| PRECOMPILE_SHA2-256, CALL (fork Prague, 1M gas, SHA2, 256) | 1,000,000 | 223,239 | 11,507 | 11.51 | 1,952,568 | 1,945,178.3 | 1,939,880 |
| PUSH0 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 39,776 | 39.78 | 3,127,504 | 3,127,504 | 3,126,528 |
| PUSH13 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 43,723 | 43.72 | 2,581,064 | 2,581,044.3 | 2,576,184 |
| PUSH14 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 39,653 | 39.65 | 3,110,936 | 3,110,918.5 | 3,110,936 |
| PUSH15 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 39,275 | 39.27 | 3,117,756 | 3,117,756 | 3,116,780 |
| PUSH17 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 39,680 | 39.68 | 3,277,732 | 3,277,732 | 3,275,780 |
| PUSH1 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 28,779 | 28.78 | 2,814,280 | 2,812,626.2 | 2,804,520 |
| PUSH21 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 53,335 | 53.34 | 1,916,456 | 1,908,753.5 | 1,894,984 |
| PUSH22 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 56,596 | 56.60 | 2,398,572 | 2,397,847.6 | 2,391,740 |
| PUSH23 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 48,682 | 48.68 | 3,110,936 | 3,110,936 | 3,110,936 |
| PUSH29 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 64,266 | 64.27 | 2,467,852 | 2,467,775.8 | 2,466,876 |
| PUSH2 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 30,611 | 30.61 | 2,702,064 | 2,702,064 | 2,702,064 |
| PUSH3 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 33,469 | 33.47 | 2,996,760 | 2,996,760 | 2,987,976 |
| PUSH4 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 27,642 | 27.64 | 3,266,036 | 3,266,036 | 3,265,060 |
| RETURN (fork Prague, 1M gas, 1KiB of non, zero data) | 1,000,000 | 223,239 | 48,806 | 48.81 | 1,891,080 | 1,875,240.9 | 1,858,872 |
| RETURN (fork Prague, 1M gas, 1MiB of non, zero data) | 1,000,000 | 223,239 | 6,818 | 6.82 | 3,094,352 | 3,094,352 | 3,094,352 |
| REVERT (fork Prague, 1M gas, 1MiB of zero data) | 1,000,000 | 223,239 | 6,016 | 6.02 | 2,932,356 | 2,932,160.8 | 2,928,452 |
| REVERT (fork Prague, 1M gas, empty) | 1,000,000 | 223,239 | 95,963 | 95.96 | 3,044,584 | 3,043,282.7 | 3,037,752 |
| PRECOMPILE_IDENTITY (fork Prague, 1M gas, 0 returned size) | 1,000,000 | 223,239 | 30,411 | 30.41 | 3,289,440 | 3,289,440 | 3,289,440 |
| RETURNDATASIZE (fork Prague, 1M gas, 1 returned size) | 1,000,000 | 223,239 | 36,381 | 36.38 | 2,678,644 | 2,678,644 | 2,677,668 |
| RETURNDATASIZE (fork Prague, 1M gas, 1 returned size) | 1,000,000 | 223,239 | 36,402 | 36.40 | 2,708,896 | 2,706,996.8 | 2,703,040 |
| SHL, SHR, SAR (fork Prague, 1M gas, shift right SAR) | 1,000,000 | 223,239 | 73,283 | 73.28 | 2,622,044 | 2,622,044 | 2,617,164 |
| SWAP10 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 75,972 | 75.97 | 1,815,928 | 1,807,597.1 | 1,793,480 |
| SWAP11 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 73,645 | 73.64 | 2,783,056 | 2,782,964.2 | 2,772,320 |
| SWAP13 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 61,311 | 61.31 | 3,116,784 | 3,116,784 | 3,115,808 |
| SWAP1 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 60,225 | 60.23 | 3,191,908 | 3,189,124.6 | 3,187,028 |
| SWAP2 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 72,902 | 72.90 | 3,007,496 | 3,006,461.4 | 3,004,568 |
| SWAP4 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 74,049 | 74.05 | 2,827,944 | 2,826,888.9 | 2,825,992 |
| SWAP5 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 61,749 | 61.75 | 3,258,268 | 3,258,230.2 | 3,251,436 |
| SWAP7 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 74,224 | 74.22 | 2,534,216 | 2,534,159.4 | 2,526,408 |
| SWAP8 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 62,775 | 62.77 | 3,281,636 | 3,281,636 | 3,280,660 |
| TLOAD (fork Prague, 1M gas, val mut: False, key mut: True) | 1,000,000 | 223,239 | 9,103 | 9.10 | 2,385,888 | 2,385,278 | 2,380,032 |
| TLOAD (fork Prague, 1M gas, val mut: True, key mut: True) | 1,000,000 | 223,239 | 10,871 | 10.87 | 2,825,016 | 2,824,483.6 | 2,820,136 |
| TSTORE (fork Prague, 1M gas, dense val mut: True, key mut: False) | 1,000,000 | 223,239 | 53,475 | 53.48 | 2,503,964 | 2,503,229.8 | 2,497,132 |
| BASEFEE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 41,740 | 41.74 | 3,118,724 | 3,117,770.7 | 3,117,748 |
| BLOBBASEFEE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 65,999 | 66.00 | 1,714,424 | 1,705,659.5 | 1,693,928 |
| GAS (fork Prague, 1M gas) | 1,000,000 | 223,239 | 50,732 | 50.73 | 2,861,124 | 2,860,570.1 | 2,855,268 |
| TIMESTAMP (fork Prague, 1M gas) | 1,000,000 | 223,239 | 51,617 | 51.62 | 2,854,292 | 2,853,966.7 | 2,851,364 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: False, 0 bytes, transaction) | 1,000,000 | 223,239 | 56,260 | 56.26 | 3,002,616 | 3,002,616 | 3,001,640 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: False, 100 bytes, call) | 1,000,000 | 223,239 | 39,967 | 39.97 | 3,197,756 | 3,197,058.9 | 3,195,804 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: False, 100 bytes, transaction) | 1,000,000 | 223,239 | 41,927 | 41.93 | 2,743,044 | 2,743,044 | 2,743,044 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: False, 10KiB, transaction) | 1,000,000 | 223,239 | 12,598 | 12.60 | 2,586,920 | 2,586,815.4 | 2,582,040 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: True, 0 bytes, call) | 1,000,000 | 223,239 | 38,063 | 38.06 | 2,177,044 | 2,175,917.8 | 2,166,308 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: True, 0 bytes, transaction) | 1,000,000 | 223,239 | 37,877 | 37.88 | 1,680,264 | 1,676,116 | 1,667,576 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: True, 100 bytes, call) | 1,000,000 | 223,239 | 23,694 | 23.69 | 3,263,136 | 3,263,136 | 3,261,184 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: True, 10KiB, call) | 1,000,000 | 223,239 | 17,299 | 17.30 | 3,074,832 | 3,074,832 | 3,074,832 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: True, fixed src dst: False, 100 bytes, call) | 1,000,000 | 223,239 | 48,750 | 48.75 | 3,072,884 | 3,072,884 | 3,071,908 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: True, fixed src dst: False, 10KiB, call) | 1,000,000 | 223,239 | 18,273 | 18.27 | 3,261,192 | 3,261,162.3 | 3,260,216 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: True, fixed src dst: True, 100 bytes, call) | 1,000,000 | 223,239 | 30,354 | 30.35 | 2,940,164 | 2,938,537.3 | 2,934,308 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: True, fixed src dst: True, 100 bytes, transaction) | 1,000,000 | 223,239 | 30,234 | 30.23 | 775,524 | 768,296.3 | 746,244 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: True, fixed src dst: True, 10KiB, call) | 1,000,000 | 223,239 | 13,798 | 13.80 | 3,264,108 | 3,264,108 | 3,264,108 |
| CODECOPY (fork Prague, 1M gas, fixed src dst: False, 0 bytes) | 1,000,000 | 223,239 | 55,008 | 55.01 | 2,604,480 | 2,604,480 | 2,603,504 |
| CODECOPY (fork Prague, 1M gas, fixed src dst: False, max code size) | 1,000,000 | 223,239 | 18,665 | 18.66 | 3,117,748 | 3,117,748 | 3,117,748 |
| CODECOPY (fork Prague, 1M gas, fixed src dst: True, 0.25x max code size) | 1,000,000 | 223,239 | 16,370 | 16.37 | 1,296,700 | 1,293,040 | 1,287,916 |
| CODECOPY (fork Prague, 1M gas, fixed src dst: True, 0.50x max code size) | 1,000,000 | 223,239 | 14,291 | 14.29 | 3,263,136 | 3,263,136 | 3,263,136 |
| CODECOPY (fork Prague, 1M gas, fixed src dst: True, max code size) | 1,000,000 | 223,239 | 16,940 | 16.94 | 2,639,612 | 2,637,430.1 | 2,627,900 |
| MCOPY (fork Prague, 1M gas, fixed src dst: False, 100 bytes) | 1,000,000 | 223,239 | 42,854 | 42.85 | 3,214,348 | 3,210,915.2 | 3,206,540 |
| MCOPY (fork Prague, 1M gas, fixed src dst: False, 10KiB) | 1,000,000 | 223,239 | 39,801 | 39.80 | 2,988,952 | 2,987,844.1 | 2,982,120 |
| MCOPY (fork Prague, 1M gas, fixed src dst: True, 10KiB) | 1,000,000 | 223,239 | 25,239 | 25.24 | 554,948 | 467,295.7 | 250,436 |
| MCOPY (fork Prague, 1M gas, fixed src dst: True, 1MiB) | 1,000,000 | 223,239 | 6,831 | 6.83 | 1,723,208 | 1,719,182 | 1,714,424 |
| RETURNDATACOPY (fork Prague, 1M gas, fixed dst: False, 0 bytes) | 1,000,000 | 223,239 | 54,889 | 54.89 | 1,693,928 | 1,687,551.5 | 1,681,240 |
| RETURNDATACOPY (fork Prague, 1M gas, fixed dst: False, 100 bytes) | 1,000,000 | 223,239 | 44,479 | 44.48 | 2,998,712 | 2,998,712 | 2,998,712 |
| RETURNDATACOPY (fork Prague, 1M gas, fixed dst: False, 1MiB) | 1,000,000 | 223,239 | 6,545 | 6.54 | 3,139,208 | 3,139,208 | 3,137,256 |
| RETURNDATACOPY (fork Prague, 1M gas, fixed dst: True, 0 bytes) | 1,000,000 | 223,239 | 45,248 | 45.25 | 2,947,972 | 2,947,383.0 | 2,940,164 |
| RETURNDATACOPY (fork Prague, 1M gas, fixed dst: True, 100 bytes) | 1,000,000 | 223,239 | 32,349 | 32.35 | 2,997,736 | 2,997,736 | 2,997,736 |
| RETURNDATACOPY (fork Prague, 1M gas, fixed dst: True, 10KiB) | 1,000,000 | 223,239 | 14,557 | 14.56 | 3,196,784 | 3,194,122.2 | 3,193,856 |
| RETURNDATACOPY (fork Prague, 1M gas, fixed dst: True, 1MiB) | 1,000,000 | 223,239 | 7,818 | 7.82 | 3,008,472 | 3,008,472 | 3,008,472 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 0 bytes data, log1) | 1,000,000 | 223,239 | 7,684 | 7.68 | 2,917,720 | 2,915,768 | 2,910,888 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB non zero data, log1) | 1,000,000 | 223,239 | 6,411 | 6.41 | 3,062,152 | 3,062,152 | 3,060,200 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB non zero data, log3) | 1,000,000 | 223,239 | 6,271 | 6.27 | 2,222,912 | 2,222,912 | 2,219,984 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB non zero data, log4) | 1,000,000 | 223,239 | 6,070 | 6.07 | 2,800,616 | 2,800,182.2 | 2,795,736 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB zeros data, log0) | 1,000,000 | 223,239 | 5,326 | 5.33 | 2,651,316 | 2,651,316 | 2,651,316 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB zeros data, log1) | 1,000,000 | 223,239 | 6,049 | 6.05 | 2,883,568 | 2,883,568 | 2,883,568 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB zeros data, log3) | 1,000,000 | 223,239 | 5,041 | 5.04 | 3,110,932 | 3,110,932 | 3,110,932 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB zeros data, log4) | 1,000,000 | 223,239 | 6,186 | 6.19 | 3,069,956 | 3,069,956 | 3,069,956 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 0 bytes data, log2) | 1,000,000 | 223,239 | 6,420 | 6.42 | 3,111,912 | 3,111,505.3 | 3,106,056 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 0 bytes data, log4) | 1,000,000 | 223,239 | 7,204 | 7.20 | 2,740,120 | 2,740,120 | 2,739,144 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB non zero data, log0) | 1,000,000 | 223,239 | 5,055 | 5.05 | 3,191,904 | 3,191,904 | 3,190,928 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB non zero data, log1) | 1,000,000 | 223,239 | 5,228 | 5.23 | 3,271,884 | 3,271,617.8 | 3,267,980 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB non zero data, log2) | 1,000,000 | 223,239 | 6,592 | 6.59 | 2,851,364 | 2,851,364 | 2,851,364 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB non zero data, log3) | 1,000,000 | 223,239 | 5,240 | 5.24 | 3,276,764 | 3,276,764 | 3,276,764 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB non zero data, log4) | 1,000,000 | 223,239 | 6,213 | 6.21 | 2,901,132 | 2,900,481.3 | 2,897,228 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB zeros data, log0) | 1,000,000 | 223,239 | 6,206 | 6.21 | 2,946,992 | 2,946,992 | 2,946,992 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB zeros data, log1) | 1,000,000 | 223,239 | 5,685 | 5.68 | 2,981,148 | 2,981,148 | 2,978,220 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB zeros data, log3) | 1,000,000 | 223,239 | 6,064 | 6.06 | 2,710,848 | 2,710,848 | 2,708,896 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB zeros data, log4) | 1,000,000 | 223,239 | 5,534 | 5.53 | 3,075,812 | 3,075,812 | 3,073,860 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 0 bytes data, log4) | 1,000,000 | 223,239 | 6,960 | 6.96 | 2,659,124 | 2,657,904 | 2,651,316 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB non zero data, log0) | 1,000,000 | 223,239 | 5,303 | 5.30 | 3,194,832 | 3,194,636.8 | 3,192,880 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB non zero data, log4) | 1,000,000 | 223,239 | 5,917 | 5.92 | 2,001,368 | 1,997,220 | 1,992,584 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB zeros data, log0) | 1,000,000 | 223,239 | 6,413 | 6.41 | 2,855,268 | 2,855,024 | 2,854,292 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB zeros data, log2) | 1,000,000 | 223,239 | 6,529 | 6.53 | 2,828,920 | 2,828,920 | 2,827,944 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB zeros data, log3) | 1,000,000 | 223,239 | 6,639 | 6.64 | 1,512,392 | 1,510,928 | 1,507,512 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB zeros data, log4) | 1,000,000 | 223,239 | 5,979 | 5.98 | 2,433,700 | 2,433,433.8 | 2,427,844 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 0 bytes data, log0) | 1,000,000 | 223,239 | 8,207 | 8.21 | 1,894,008 | 1,893,520 | 1,891,080 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 0 bytes data, log1) | 1,000,000 | 223,239 | 5,833 | 5.83 | 3,127,504 | 3,127,504 | 3,127,504 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 0 bytes data, log2) | 1,000,000 | 223,239 | 5,667 | 5.67 | 3,248,508 | 3,248,450.6 | 3,248,508 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 0 bytes data, log3) | 1,000,000 | 223,239 | 5,666 | 5.67 | 3,261,188 | 3,261,188 | 3,260,212 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB non zero data, log0) | 1,000,000 | 223,239 | 6,514 | 6.51 | 2,786,960 | 2,786,693.8 | 2,783,056 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB non zero data, log1) | 1,000,000 | 223,239 | 6,210 | 6.21 | 2,716,700 | 2,716,130.7 | 2,710,844 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB non zero data, log2) | 1,000,000 | 223,239 | 5,540 | 5.54 | 3,127,508 | 3,127,312 | 3,126,532 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB non zero data, log4) | 1,000,000 | 223,239 | 5,280 | 5.28 | 3,276,760 | 3,276,760 | 3,275,784 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB zeros data, log0) | 1,000,000 | 223,239 | 6,378 | 6.38 | 1,992,584 | 1,992,584 | 1,982,824 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB zeros data, log1) | 1,000,000 | 223,239 | 5,381 | 5.38 | 3,119,700 | 3,119,700 | 3,117,748 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB zeros data, log3) | 1,000,000 | 223,239 | 5,045 | 5.04 | 3,196,784 | 3,196,784 | 3,196,784 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB zeros data, log4) | 1,000,000 | 223,239 | 4,991 | 4.99 | 3,127,500 | 3,127,500 | 3,126,524 |
| DELEGATECALL (fork Prague, 1M gas, absent target: False) | 1,000,000 | 223,239 | 77,789 | 77.79 | 3,097,272 | 3,097,272 | 3,097,272 |
| EXTCODEHASH (fork Prague, 1M gas, absent target: False) | 1,000,000 | 223,239 | 25,894 | 25.89 | 2,433,700 | 2,433,700 | 2,432,724 |
| BALANCE (fork Prague, 1M gas, absent target: True) | 1,000,000 | 223,239 | 29,650 | 29.65 | 2,593,748 | 2,593,748 | 2,593,748 |
| CALLCODE (fork Prague, 1M gas, absent target: True) | 1,000,000 | 223,239 | 77,731 | 77.73 | 1,837,400 | 1,826,971.3 | 1,815,928 |
| DELEGATECALL (fork Prague, 1M gas, absent target: True) | 1,000,000 | 223,239 | 56,023 | 56.02 | 3,127,500 | 3,127,500 | 3,127,500 |
| EXTCODESIZE (fork Prague, 1M gas, absent target: True) | 1,000,000 | 223,239 | 30,043 | 30.04 | 2,330,264 | 2,329,468.7 | 2,327,336 |
| EXTCODECOPY (fork Prague, 1M gas, 1KiB) | 1,000,000 | 223,239 | 20,967 | 20.97 | 746,244 | 739,296.2 | 727,700 |
| SELFBALANCE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 321,628 | 321.63 | 2,817,208 | 2,817,014.5 | 2,815,256 |
| SELFDESTRUCT (fork Prague, 1M gas, value bearing: False) | 965,720 | 223,239 | 6,423 | 6.42 | 2,771,344 | 2,771,344 | 2,769,392 |
| SELFDESTRUCT (fork Prague, 1M gas, value bearing: True) | 965,720 | 223,239 | 5,768 | 5.77 | 3,263,136 | 3,263,136 | 3,263,136 |
| SELFDESTRUCT (fork Prague, 1M gas, value bearing: False) | 998,771 | 223,239 | 13,023 | 13.02 | 2,928,452 | 2,928,452 | 2,928,452 |
| SELFDESTRUCT (fork Prague, 1M gas, value bearing: True) | 998,771 | 223,239 | 14,336 | 14.34 | 2,622,044 | 2,622,044 | 2,622,044 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: False, SSLOAD) | 999,749 | 223,239 | 22,151 | 22.15 | 2,759,632 | 2,758,572.9 | 2,748,896 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: False, SSTORE new value) | 998,957 | 223,239 | 15,488 | 15.49 | 2,720,604 | 2,720,378.8 | 2,716,700 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: False, SSTORE same value) | 999,009 | 223,239 | 22,042 | 22.04 | 919,964 | 901,474.2 | 877,020 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: True) | 1,000,000 | 223,239 | 7,061 | 7.06 | 2,465,900 | 2,464,815.6 | 2,452,236 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: True) | 995,213 | 223,239 | 6,832 | 6.83 | 795,040 | 783,902.1 | 776,496 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: True, SSTORE new value) | 995,207 | 223,239 | 6,220 | 6.22 | 2,378,080 | 2,377,917.3 | 2,372,224 |
| SLOAD, SSTORE (fork Prague, 1M gas, SLOAD) | 1,000,000 | 223,239 | 26,544 | 26.54 | 726,724 | 718,807.6 | 707,204 |
| SLOAD, SSTORE (fork Prague, 1M gas, SSTORE new value) | 1,000,000 | 223,239 | 54,817 | 54.82 | 3,288,464 | 3,288,464 | 3,288,464 |
| SLOAD, SSTORE (fork Prague, 1M gas, SSTORE same value) | 1,000,000 | 223,239 | 48,408 | 48.41 | 3,002,616 | 3,002,616 | 3,001,640 |

## Summary Statistics

- **Total Tests:** 280
- **Successful Tests:** 276
- **Failed Tests:** 4

### Proving Time (ms)
- **Average:** 242,723.8
- **Minimum:** 4,617
- **Maximum:** 4,247,664

### Peak Memory Usage (MB)
- **Average:** 2,672,573.4
- **Minimum:** 554,948
- **Maximum:** 3,289,440

### Proof Size (bytes)
- **Average:** 223,239
- **Minimum:** 223,239
- **Maximum:** 223,239
