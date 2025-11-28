# zkEVM Benchmark Results

Generated on: 2025-11-28 14:14:41

## Folder: zkevm-metrics-risc0-1M-1

**zkVM:** risc0-v3.0.3 (1 GPUs)

**Hardware Configuration:** CPU: AMD EPYC 7B13 64-Core Processor | RAM: 396 GiB | GPU: NVIDIA GeForce RTX 4090, NVIDIA GeForce RTX 4090, NVIDIA GeForce RTX 4090, NVIDIA GeForce RTX 4090

## Proving Metrics

| Benchmark | Gas Used | Proof Size (bytes) | Proving Time (ms) | Proving Time (s) | Peak Memory (MB) | Avg Memory (MB) | Initial Memory (MB) |
|---|---|---|---|---|---|---|---|
| SLOAD, SSTORE, BALANCE, EXTCODESIZE, EXTCODEHASH, CALLDATALOAD, CALLDATACOPY, CALLDATASIZE (fork Prague, 1M gas) | 999,980 | 223,239 | 9,319 | 9.32 | 2,594,728 | 2,594,498.4 | 2,589,848 |
| CALLDATALOAD, CALLDATACOPY, CALLDATASIZE (fork Prague, 1M gas, zero byte: False) | 1,000,000 | 223,239 | 6,758 | 6.76 | 823,092 | 820,451.1 | 815,284 |
| CALLDATALOAD, CALLDATACOPY, CALLDATASIZE (fork Prague, 1M gas, zero byte: True) | 1,000,000 | 223,239 | 13,337 | 13.34 | 2,850,388 | 2,850,062.7 | 2,847,460 |
| CALL (fork Prague, 1M gas, case id a to a) | 987,000 | 223,239 | 27,636 | 27.64 | 2,739,144 | 2,739,144 | 2,739,144 |
| CALL (fork Prague, 1M gas, case id a to b) | 987,000 | 223,239 | 24,194 | 24.19 | 3,287,488 | 3,287,488 | 3,286,512 |
| CALL (fork Prague, 1M gas) | 987,000 | 223,239 | 24,341 | 24.34 | 3,561,652 | 3,561,047.8 | 3,549,940 |
| CALL (fork Prague, 1M gas) | 987,000 | 223,239 | 29,730 | 29.73 | 2,787,936 | 2,787,913.2 | 2,786,960 |
| CALL (fork Prague, 1M gas) | 987,000 | 223,239 | 30,677 | 30.68 | 3,071,908 | 3,071,908 | 3,069,956 |
| CALLCODE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 394,593 | 394.59 | 2,691,332 | 2,690,032.0 | 2,679,620 |
| CALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 389,736 | 389.74 | 2,216,080 | 2,211,706.2 | 2,184,848 |
| DELEGATECALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 327,463 | 327.46 | 3,251,436 | 3,251,436 | 3,248,508 |
| EXTCODECOPY (fork Prague, 1M gas) | 1,000,000 | 223,239 | 381,219 | 381.22 | 1,243,736 | 1,217,643.7 | 1,162,728 |
| EXTCODEHASH (fork Prague, 1M gas) | 1,000,000 | 223,239 | 391,661 | 391.66 | 2,560,568 | 2,560,449.0 | 2,550,808 |
| EXTCODESIZE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 332,380 | 332.38 | 3,489,464 | 3,489,464 | 3,489,464 |
| STATICCALL (fork Prague, 1M gas) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| CREATE2 (fork Prague, 1M gas, 0 bytes) | 1,000,000 | 223,239 | 6,927 | 6.93 | 2,233,648 | 2,232,672 | 2,224,864 |
| CREATE (fork Prague, 1M gas, 0 bytes) | 1,000,000 | 223,239 | 6,483 | 6.48 | 707,204 | 705,876.6 | 695,492 |
| CREATE2 (fork Prague, 1M gas, 0 bytes) | 1,000,000 | 223,239 | 6,990 | 6.99 | 1,589,240 | 1,588,068.8 | 1,583,384 |
| CREATE (fork Prague, 1M gas, 0 bytes) | 1,000,000 | 223,239 | 5,763 | 5.76 | 3,176,292 | 3,176,048 | 3,174,340 |
| CREATE2 (fork Prague, 1M gas, zero data) | 1,000,000 | 223,239 | 7,203 | 7.20 | 877,996 | 871,094.3 | 853,596 |
| CREATE (fork Prague, 1M gas, zero data) | 1,000,000 | 223,239 | 5,623 | 5.62 | 3,350,892 | 3,350,892 | 3,348,940 |
| CREATE2 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 6,332 | 6.33 | 2,721,336 | 2,721,336 | 2,721,336 |
| CREATE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 6,093 | 6.09 | 2,612,284 | 2,612,067.1 | 2,608,380 |
| CREATE2 (fork Prague, 1M gas, zero data) | 1,000,000 | 223,239 | 5,762 | 5.76 | 3,410,412 | 3,410,412 | 3,410,412 |
| CREATE (fork Prague, 1M gas, zero data) | 1,000,000 | 223,239 | 5,704 | 5.70 | 3,559,692 | 3,559,692 | 3,559,692 |
| CREATE2 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 6,865 | 6.87 | 2,804,520 | 2,802,177.6 | 2,800,616 |
| CREATE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 6,622 | 6.62 | 2,439,556 | 2,438,934.5 | 2,433,700 |
| CREATE2 (fork Prague, 1M gas, zero data) | 1,000,000 | 223,239 | 8,304 | 8.30 | 1,842,280 | 1,840,653.3 | 1,837,400 |
| CREATE (fork Prague, 1M gas, zero data) | 1,000,000 | 223,239 | 6,004 | 6.00 | 3,141,160 | 3,141,160 | 3,138,232 |
| CREATE2 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 6,875 | 6.88 | 2,390,764 | 2,390,764 | 2,390,764 |
| CREATE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 6,447 | 6.45 | 2,603,508 | 2,603,418.9 | 2,603,508 |
| CREATE2 (fork Prague, 1M gas, zero data) | 1,000,000 | 223,239 | 6,779 | 6.78 | 3,531,408 | 3,531,408 | 3,529,456 |
| CREATE (fork Prague, 1M gas, zero data) | 1,000,000 | 223,239 | 8,312 | 8.31 | 2,397,592 | 2,397,592 | 2,397,592 |
| CREATE2 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 5,602 | 5.60 | 3,352,844 | 3,352,844 | 3,352,844 |
| CREATE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 5,675 | 5.67 | 3,275,788 | 3,275,788 | 3,274,812 |
| CREATE2 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 5,753 | 5.75 | 3,346,996 | 3,346,996 | 3,346,996 |
| CREATE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 6,699 | 6.70 | 3,400,660 | 3,400,464.8 | 3,400,660 |
| JUMPDEST, JUMP (fork Prague, 1M gas, 00) | 1,000,000 | 223,239 | 26,002 | 26.00 | 3,004,568 | 3,004,513.8 | 3,002,616 |
| JUMPDEST, JUMP (fork Prague, 1M gas, 5b) | 1,000,000 | 223,239 | 36,229 | 36.23 | 1,673,172 | 1,667,578.8 | 1,647,796 |
| JUMPDEST, JUMP (fork Prague, 1M gas, 605b5b) | 1,000,000 | 223,239 | 26,430 | 26.43 | 1,032,204 | 1,009,487.6 | 968,764 |
| JUMPDEST, JUMP (fork Prague, 1M gas, 605b) | 1,000,000 | 223,239 | 16,698 | 16.70 | 3,259,240 | 3,259,240 | 3,259,240 |
| JUMPDEST, JUMP (fork Prague, 1M gas, 615b5b5b) | 1,000,000 | 223,239 | 22,315 | 22.32 | 968,764 | 954,899.1 | 921,916 |
| JUMPDEST, JUMP (fork Prague, 1M gas, 615b5b) | 1,000,000 | 223,239 | 14,917 | 14.92 | 3,509,940 | 3,509,940 | 3,508,964 |
| PRECOMPILE_EC_PAIRING, CALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 291,410 | 291.41 | 2,767,440 | 2,766,846.4 | 2,760,608 |
| UNKNOWN (fork Prague, 1M gas) | 0 | 223,239 | 4,617 | 4.62 | 2,331,240 | 2,331,240 | 2,331,240 |
| ADD (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 33,292 | 33.29 | 3,342,128 | 3,342,116.9 | 3,341,152 |
| AND (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 32,652 | 32.65 | 3,059,224 | 3,057,058.5 | 3,052,392 |
| BYTE (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 35,789 | 35.79 | 2,391,480 | 2,390,975.2 | 2,383,672 |
| DIV (fork Prague, 1M gas, 0) | 1,000,000 | 223,239 | 179,741 | 179.74 | 3,490,440 | 3,490,440 | 3,488,488 |
| DIV (fork Prague, 1M gas, 1) | 1,000,000 | 223,239 | 170,049 | 170.05 | 3,186,052 | 3,185,869.7 | 3,183,124 |
| EQ (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 82,915 | 82.92 | 2,167,284 | 2,167,105.1 | 2,159,476 |
| EXP (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 127,391 | 127.39 | 2,616,188 | 2,616,188 | 2,612,284 |
| GT (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 29,183 | 29.18 | 3,266,040 | 3,266,040 | 3,265,064 |
| LT (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 34,144 | 34.14 | 2,084,060 | 2,079,433.0 | 2,068,444 |
| MOD (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 51,342 | 51.34 | 3,268,956 | 3,268,956 | 3,268,956 |
| MUL (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 64,340 | 64.34 | 3,103,128 | 3,103,057.3 | 3,097,272 |
| OR (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 32,236 | 32.24 | 2,373,916 | 2,372,761.1 | 2,367,084 |
| SAR (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 78,839 | 78.84 | 2,910,888 | 2,910,471.2 | 2,901,128 |
| SDIV (fork Prague, 1M gas, 0) | 1,000,000 | 223,239 | 214,968 | 214.97 | 3,492,392 | 3,491,798.9 | 3,491,416 |
| SDIV (fork Prague, 1M gas, 1) | 1,000,000 | 223,239 | 251,953 | 251.95 | 1,774,936 | 1,761,792.2 | 1,723,208 |
| SGT (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 90,830 | 90.83 | 2,550,552 | 2,548,687.1 | 2,543,720 |
| SHL (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 66,396 | 66.40 | 1,994,276 | 1,993,984.8 | 1,987,444 |
| SHR (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 56,986 | 56.99 | 3,538,236 | 3,537,467.7 | 3,535,308 |
| SIGNEXTEND (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 51,420 | 51.42 | 3,491,416 | 3,491,400.3 | 3,491,416 |
| SLT (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 35,935 | 35.94 | 3,117,748 | 3,117,748 | 3,117,748 |
| SMOD (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 75,461 | 75.46 | 2,486,148 | 2,484,961.8 | 2,478,340 |
| SUB (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 35,238 | 35.24 | 3,560,668 | 3,560,668 | 3,559,692 |
| XOR (fork Prague, 1M gas, ) | 1,000,000 | 223,239 | 27,832 | 27.83 | 3,197,756 | 3,197,365.6 | 3,196,780 |
| BLOBHASH (fork Prague, 1M gas, no blobs) | 1,000,000 | 223,239 | 37,313 | 37.31 | 2,576,184 | 2,576,066.9 | 2,573,256 |
| BLOBHASH (fork Prague, 1M gas) | 1,000,000 | 223,239 | 55,889 | 55.89 | 1,328,908 | 1,317,125.3 | 1,295,724 |
| BLOBHASH (fork Prague, 1M gas, existent index) | 1,000,000 | 223,239 | 36,079 | 36.08 | 2,005,008 | 2,003,027.3 | 1,994,272 |
| BLOBHASH (fork Prague, 1M gas) | 1,000,000 | 223,239 | 56,158 | 56.16 | 2,219,984 | 2,219,633.6 | 2,216,080 |
| CALLDATALOAD (fork Prague, 1M gas, empty) | 1,000,000 | 223,239 | 64,591 | 64.59 | 3,203,612 | 3,201,538 | 3,197,756 |
| CALLDATALOAD (fork Prague, 1M gas, one, loop) | 1,000,000 | 223,239 | 76,577 | 76.58 | 3,370,408 | 3,370,408 | 3,370,408 |
| CALLDATALOAD (fork Prague, 1M gas, zero, loop) | 1,000,000 | 223,239 | 93,894 | 93.89 | 2,692,304 | 2,692,304 | 2,692,304 |
| CALLDATASIZE (fork Prague, 1M gas, 0 calldata length) | 1,000,000 | 223,239 | 33,778 | 33.78 | 2,491,028 | 2,489,145.7 | 2,486,148 |
| CALLDATASIZE (fork Prague, 1M gas, 10000 calldata length) | 1,000,000 | 223,239 | 34,749 | 34.75 | 2,479,316 | 2,477,269.5 | 2,472,484 |
| CALLDATASIZE (fork Prague, 1M gas, 1000 calldata length) | 1,000,000 | 223,239 | 34,984 | 34.98 | 2,743,044 | 2,743,044 | 2,742,068 |
| CALLVALUE (fork Prague, 1M gas, from origin: False, non zero value: False) | 1,000,000 | 223,239 | 35,408 | 35.41 | 3,345,044 | 3,345,044 | 3,345,044 |
| CALLVALUE (fork Prague, 1M gas, from origin: False, non zero value: True) | 1,000,000 | 223,239 | 39,887 | 39.89 | 2,460,772 | 2,459,991.2 | 2,449,060 |
| CALLVALUE (fork Prague, 1M gas, from origin: True, non zero value: False) | 1,000,000 | 223,239 | 41,417 | 41.42 | 1,290,584 | 1,283,556.8 | 1,265,208 |
| CALLVALUE (fork Prague, 1M gas, from origin: True, non zero value: True) | 1,000,000 | 223,239 | 41,821 | 41.82 | 746,972 | 739,652 | 724,524 |
| DUP10 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 26,174 | 26.17 | 3,271,884 | 3,271,884 | 3,271,884 |
| DUP11 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 32,244 | 32.24 | 2,820,136 | 2,820,136 | 2,817,208 |
| DUP12 (fork Prague, 1M gas) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| DUP13 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 31,732 | 31.73 | 2,796,716 | 2,796,576.6 | 2,786,956 |
| DUP14 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 26,891 | 26.89 | 3,348,948 | 3,348,948 | 3,348,948 |
| DUP15 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 31,836 | 31.84 | 2,184,848 | 2,184,418.6 | 2,176,064 |
| DUP16 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 26,813 | 26.81 | 3,550,916 | 3,550,916 | 3,550,916 |
| DUP1 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 26,886 | 26.89 | 3,275,788 | 3,275,788 | 3,275,788 |
| DUP2 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 27,668 | 27.67 | 3,374,312 | 3,374,312 | 3,373,336 |
| DUP3 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 31,746 | 31.75 | 1,610,712 | 1,602,836.7 | 1,592,168 |
| DUP4 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 31,506 | 31.51 | 2,748,896 | 2,748,733.3 | 2,742,064 |
| DUP5 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 30,593 | 30.59 | 3,107,032 | 3,107,032 | 3,102,152 |
| DUP6 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 32,476 | 32.48 | 2,334,164 | 2,333,263.1 | 2,330,260 |
| DUP7 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 31,509 | 31.51 | 2,573,256 | 2,570,982.1 | 2,560,568 |
| DUP8 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 27,502 | 27.50 | 3,268,960 | 3,268,960 | 3,268,960 |
| DUP9 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 31,295 | 31.30 | 2,237,548 | 2,236,454.9 | 2,233,644 |
| JUMPDEST (fork Prague, 1M gas) | 1,000,000 | 223,239 | 34,147 | 34.15 | 3,369,436 | 3,369,436 | 3,368,460 |
| JUMPI (fork Prague, 1M gas) | 1,000,000 | 223,239 | 46,124 | 46.12 | 1,793,480 | 1,789,942 | 1,775,912 |
| JUMPI (fork Prague, 1M gas) | 1,000,000 | 223,239 | 28,931 | 28.93 | 3,001,640 | 3,001,640 | 3,001,640 |
| JUMP (fork Prague, 1M gas) | 1,000,000 | 223,239 | 22,916 | 22.92 | 2,424,660 | 2,421,328.1 | 2,413,924 |
| SHA3, KECCAK256 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 174,428 | 174.43 | 2,227,532 | 2,225,294.5 | 2,217,772 |
| MLOAD (fork Prague, 1M gas, big memory expansion: False, offset initialized: False, 0 offset) | 1,000,000 | 223,239 | 47,163 | 47.16 | 2,509,820 | 2,509,754.9 | 2,503,964 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: False, offset initialized: False, 0 offset) | 1,000,000 | 223,239 | 32,706 | 32.71 | 1,857,896 | 1,853,899.0 | 1,843,256 |
| MSTORE (fork Prague, 1M gas, big memory expansion: False, offset initialized: False, 0 offset) | 1,000,000 | 223,239 | 49,592 | 49.59 | 1,084,908 | 1,062,888.8 | 1,032,204 |
| MLOAD (fork Prague, 1M gas, big memory expansion: False, offset initialized: False, 1 offset) | 1,000,000 | 223,239 | 48,072 | 48.07 | 3,002,616 | 3,002,616 | 3,002,616 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: False, offset initialized: False, 1 offset) | 1,000,000 | 223,239 | 32,061 | 32.06 | 2,311,464 | 2,309,414.4 | 2,303,656 |
| MSTORE (fork Prague, 1M gas, big memory expansion: False, offset initialized: False, 1 offset) | 1,000,000 | 223,239 | 41,036 | 41.04 | 3,341,152 | 3,341,152 | 3,341,152 |
| MLOAD (fork Prague, 1M gas, big memory expansion: False, offset initialized: False, 31 offset) | 1,000,000 | 223,239 | 41,188 | 41.19 | 3,263,132 | 3,263,132 | 3,262,156 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: False, offset initialized: False, 31 offset) | 1,000,000 | 223,239 | 31,903 | 31.90 | 1,982,824 | 1,967,068.6 | 1,952,568 |
| MSTORE (fork Prague, 1M gas, big memory expansion: False, offset initialized: False, 31 offset) | 1,000,000 | 223,239 | 48,883 | 48.88 | 2,640,584 | 2,640,106.4 | 2,638,632 |
| MLOAD (fork Prague, 1M gas, big memory expansion: False, offset initialized: True, 0 offset) | 1,000,000 | 223,239 | 47,227 | 47.23 | 2,327,336 | 2,327,140.8 | 2,319,528 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: False, offset initialized: True, 0 offset) | 1,000,000 | 223,239 | 27,717 | 27.72 | 3,242,652 | 3,242,652 | 3,242,652 |
| MSTORE (fork Prague, 1M gas, big memory expansion: False, offset initialized: True, 0 offset) | 1,000,000 | 223,239 | 48,098 | 48.10 | 1,162,728 | 1,157,286.1 | 1,147,112 |
| MLOAD (fork Prague, 1M gas, big memory expansion: False, offset initialized: True, 1 offset) | 1,000,000 | 223,239 | 40,407 | 40.41 | 3,494,336 | 3,494,336 | 3,493,360 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: False, offset initialized: True, 1 offset) | 1,000,000 | 223,239 | 32,803 | 32.80 | 2,933,332 | 2,931,629.2 | 2,931,380 |
| MSTORE (fork Prague, 1M gas, big memory expansion: False, offset initialized: True, 1 offset) | 1,000,000 | 223,239 | 40,053 | 40.05 | 3,117,748 | 3,117,748 | 3,115,796 |
| MLOAD (fork Prague, 1M gas, big memory expansion: False, offset initialized: True, 31 offset) | 1,000,000 | 223,239 | 46,772 | 46.77 | 2,252,908 | 2,252,893.6 | 2,250,956 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: False, offset initialized: True, 31 offset) | 1,000,000 | 223,239 | 31,523 | 31.52 | 2,403,188 | 2,403,165.8 | 2,403,188 |
| MSTORE (fork Prague, 1M gas, big memory expansion: False, offset initialized: True, 31 offset) | 1,000,000 | 223,239 | 40,398 | 40.40 | 3,345,044 | 3,345,044 | 3,345,044 |
| MLOAD (fork Prague, 1M gas, big memory expansion: True, offset initialized: False, 0 offset) | 1,000,000 | 223,239 | 40,124 | 40.12 | 3,509,944 | 3,509,860.9 | 3,504,088 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: True, offset initialized: False, 0 offset) | 1,000,000 | 223,239 | 33,090 | 33.09 | 1,939,880 | 1,931,974.4 | 1,916,456 |
| MSTORE (fork Prague, 1M gas, big memory expansion: True, offset initialized: False, 0 offset) | 1,000,000 | 223,239 | 48,450 | 48.45 | 2,491,276 | 2,491,276 | 2,490,300 |
| MLOAD (fork Prague, 1M gas, big memory expansion: True, offset initialized: False, 1 offset) | 1,000,000 | 223,239 | 48,505 | 48.51 | 3,014,328 | 3,011,949 | 3,008,472 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: True, offset initialized: False, 1 offset) | 1,000,000 | 223,239 | 27,884 | 27.88 | 3,352,844 | 3,352,844 | 3,352,844 |
| MSTORE (fork Prague, 1M gas, big memory expansion: True, offset initialized: False, 1 offset) | 1,000,000 | 223,239 | 47,890 | 47.89 | 2,603,508 | 2,601,666.0 | 2,594,724 |
| MLOAD (fork Prague, 1M gas, big memory expansion: True, offset initialized: False, 31 offset) | 1,000,000 | 223,239 | 47,434 | 47.43 | 2,390,764 | 2,390,431.3 | 2,385,884 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: True, offset initialized: False, 31 offset) | 1,000,000 | 223,239 | 33,296 | 33.30 | 2,518,356 | 2,517,028.6 | 2,512,500 |
| MSTORE (fork Prague, 1M gas, big memory expansion: True, offset initialized: False, 31 offset) | 1,000,000 | 223,239 | 48,627 | 48.63 | 2,976,268 | 2,976,185.3 | 2,970,412 |
| MLOAD (fork Prague, 1M gas, big memory expansion: True, offset initialized: True, 0 offset) | 1,000,000 | 223,239 | 47,724 | 47.72 | 842,864 | 820,281.0 | 794,064 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: True, offset initialized: True, 0 offset) | 1,000,000 | 223,239 | 27,131 | 27.13 | 3,274,812 | 3,274,742.3 | 3,271,884 |
| MSTORE (fork Prague, 1M gas, big memory expansion: True, offset initialized: True, 0 offset) | 1,000,000 | 223,239 | 48,155 | 48.16 | 2,528,116 | 2,527,465.2 | 2,518,356 |
| MLOAD (fork Prague, 1M gas, big memory expansion: True, offset initialized: True, 1 offset) | 1,000,000 | 223,239 | 38,766 | 38.77 | 3,125,556 | 3,125,464.5 | 3,119,700 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: True, offset initialized: True, 1 offset) | 1,000,000 | 223,239 | 32,840 | 32.84 | 2,575,912 | 2,575,141.5 | 2,574,936 |
| MSTORE (fork Prague, 1M gas, big memory expansion: True, offset initialized: True, 1 offset) | 1,000,000 | 223,239 | 40,421 | 40.42 | 3,310,912 | 3,310,912 | 3,310,912 |
| MLOAD (fork Prague, 1M gas, big memory expansion: True, offset initialized: True, 31 offset) | 1,000,000 | 223,239 | 46,940 | 46.94 | 2,883,568 | 2,883,196.2 | 2,879,664 |
| MSTORE8 (fork Prague, 1M gas, big memory expansion: True, offset initialized: True, 31 offset) | 1,000,000 | 223,239 | 31,947 | 31.95 | 2,952,848 | 2,952,437.1 | 2,946,992 |
| MSTORE (fork Prague, 1M gas, big memory expansion: True, offset initialized: True, 31 offset) | 1,000,000 | 223,239 | 48,674 | 48.67 | 2,488,348 | 2,487,786.3 | 2,482,492 |
| MOD, SMOD (fork Prague, 1M gas, op MOD, 127 mod bits) | 1,000,000 | 223,239 | 165,618 | 165.62 | 3,505,064 | 3,505,064 | 3,505,064 |
| MOD, SMOD (fork Prague, 1M gas, op MOD, 191 mod bits) | 1,000,000 | 223,239 | 262,104 | 262.10 | 1,583,384 | 1,526,636.0 | 1,436,984 |
| MOD, SMOD (fork Prague, 1M gas, op MOD, 255 mod bits) | 1,000,000 | 223,239 | 165,717 | 165.72 | 3,495,312 | 3,495,312 | 3,495,312 |
| MOD, SMOD (fork Prague, 1M gas, op MOD, 63 mod bits) | 1,000,000 | 223,239 | 138,392 | 138.39 | 2,496,156 | 2,495,950.7 | 2,491,276 |
| MOD, SMOD (fork Prague, 1M gas, op SMOD, 127 mod bits) | 1,000,000 | 223,239 | 179,918 | 179.92 | 3,370,408 | 3,370,408 | 3,370,408 |
| MOD, SMOD (fork Prague, 1M gas, op SMOD, 191 mod bits) | 1,000,000 | 223,239 | 273,410 | 273.41 | 1,430,408 | 1,384,181.4 | 1,329,880 |
| MOD, SMOD (fork Prague, 1M gas, op SMOD, 255 mod bits) | 1,000,000 | 223,239 | 202,537 | 202.54 | 2,523,484 | 2,522,313.9 | 2,510,796 |
| MOD, SMOD (fork Prague, 1M gas, op SMOD, 63 mod bits) | 1,000,000 | 223,239 | 148,593 | 148.59 | 2,264,620 | 2,263,073.9 | 2,252,908 |
| ADDMOD, MULMOD (fork Prague, 1M gas, op ADDMOD, 127 mod bits) | 1,000,000 | 223,239 | 136,849 | 136.85 | 3,113,856 | 3,113,832.5 | 3,109,952 |
| ADDMOD, MULMOD (fork Prague, 1M gas, op ADDMOD, 191 mod bits) | 1,000,000 | 223,239 | 160,906 | 160.91 | 3,128,476 | 3,128,076.3 | 3,127,500 |
| ADDMOD, MULMOD (fork Prague, 1M gas, op ADDMOD, 255 mod bits) | 1,000,000 | 223,239 | 162,894 | 162.89 | 2,427,848 | 2,427,822.7 | 2,421,016 |
| ADDMOD, MULMOD (fork Prague, 1M gas, op ADDMOD, 63 mod bits) | 1,000,000 | 223,239 | 94,067 | 94.07 | 3,128,476 | 3,128,476 | 3,128,476 |
| ADDMOD, MULMOD (fork Prague, 1M gas, op MULMOD, 127 mod bits) | 1,000,000 | 223,239 | 283,789 | 283.79 | 2,275,608 | 2,264,692.7 | 2,243,400 |
| ADDMOD, MULMOD (fork Prague, 1M gas, op MULMOD, 191 mod bits) | 1,000,000 | 223,239 | 335,931 | 335.93 | 3,183,124 | 3,178,909.0 | 3,175,316 |
| ADDMOD, MULMOD (fork Prague, 1M gas, op MULMOD, 255 mod bits) | 1,000,000 | 223,239 | 381,740 | 381.74 | 1,506,536 | 1,476,687.1 | 1,430,408 |
| ADDMOD, MULMOD (fork Prague, 1M gas, op MULMOD, 63 mod bits) | 1,000,000 | 223,239 | 219,634 | 219.63 | 2,421,016 | 2,420,839.5 | 2,414,184 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1045 mod) | 1,000,000 | 223,239 | 3,410,394 | 3,410.39 | 3,549,940 | 3,549,075.1 | 3,545,060 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1360 mod) | 1,000,000 | 223,239 | 1,026,234 | 1,026.23 | 2,890,396 | 2,890,396 | 2,890,396 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 400 mod) | 1,000,000 | 223,239 | 1,058,921 | 1,058.92 | 3,494,340 | 3,494,017.2 | 3,493,364 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 408 mod) | 1,000,000 | 223,239 | 761,007 | 761.01 | 3,267,984 | 3,267,088.6 | 3,266,032 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 408 mod) | 1,000,000 | 223,239 | 3,765,293 | 3,765.29 | 2,327,080 | 2,324,399.6 | 2,312,440 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 600 mod) | 1,000,000 | 223,239 | 913,363 | 913.36 | 2,847,460 | 2,843,817.2 | 2,840,628 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 600 mod) | 1,000,000 | 223,239 | 1,368,908 | 1,368.91 | 2,526,408 | 2,525,638.8 | 2,522,504 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 616 mod) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 677 mod) | 1,000,000 | 223,239 | 985,138 | 985.14 | 2,969,436 | 2,965,906.4 | 2,951,868 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 765 mod) | 1,000,000 | 223,239 | 1,113,896 | 1,113.90 | 2,216,796 | 2,189,883.6 | 2,107,484 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 767 mod) | 1,000,000 | 223,239 | 866,972 | 866.97 | 2,712,552 | 2,664,540.5 | 2,583,720 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 800 mod) | 1,000,000 | 223,239 | 3,462,131 | 3,462.13 | 3,340,180 | 3,337,722.2 | 3,334,324 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 800 mod) | 1,000,000 | 223,239 | 1,455,081 | 1,455.08 | 1,135,408 | 1,083,047.0 | 857,248 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 852 mod) | 1,000,000 | 223,239 | 1,432,624 | 1,432.62 | 2,249,980 | 2,248,568.7 | 2,229,484 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 867 mod) | 1,000,000 | 223,239 | 3,496,251 | 3,496.25 | 3,307,008 | 3,306,385.9 | 3,302,128 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 996 mod) | 1,000,000 | 223,239 | 744,028 | 744.03 | 3,307,008 | 3,307,007.6 | 3,307,008 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1024 mod even) | 1,000,000 | 223,239 | 5,130 | 5.13 | 3,523,604 | 3,523,604 | 3,522,628 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 128 mod even) | 1,000,000 | 223,239 | 690,538 | 690.54 | 3,243,628 | 3,239,389.2 | 3,218,252 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 16 mod even) | 1,000,000 | 223,239 | 1,357,282 | 1,357.28 | 3,094,352 | 3,094,085.8 | 3,085,568 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 24 mod even) | 1,000,000 | 223,239 | 941,758 | 941.76 | 3,346,024 | 3,345,353.7 | 3,343,096 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 256 mod even) | 1,000,000 | 223,239 | 495,722 | 495.72 | 3,503,116 | 3,503,116 | 3,503,116 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 32 mod even) | 1,000,000 | 223,239 | 1,054,206 | 1,054.21 | 2,317,576 | 2,307,138.9 | 2,275,608 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 32 mod even) | 1,000,000 | 223,239 | 854,192 | 854.19 | 2,513,476 | 2,503,752.9 | 2,491,028 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 32 mod even) | 1,000,000 | 223,239 | 840,013 | 840.01 | 3,503,116 | 3,502,735.9 | 3,502,140 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 512 mod even) | 1,000,000 | 223,239 | 6,585 | 6.58 | 2,243,404 | 2,242,567.4 | 2,237,548 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 64 mod even) | 1,000,000 | 223,239 | 908,737 | 908.74 | 2,413,208 | 2,412,157.1 | 2,398,568 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 8 mod even) | 1,000,000 | 223,239 | 2,281,888 | 2,281.89 | 2,550,808 | 2,549,779.6 | 2,534,216 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 208 mod exp) | 1,000,000 | 223,239 | 678,173 | 678.17 | 3,279,684 | 3,278,605.9 | 3,276,756 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 215 mod exp) | 1,000,000 | 223,239 | 1,694,451 | 1,694.45 | 3,476,780 | 3,457,568.2 | 3,410,412 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 298 mod exp) | 1,000,000 | 223,239 | 1,877,381 | 1,877.38 | 3,267,036 | 3,265,675.0 | 3,264,108 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, mod min as balanced) | 1,000,000 | 223,239 | 855,426 | 855.43 | 1,975,732 | 1,941,013.2 | 1,816,644 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, mod min as base heavy) | 1,000,000 | 223,239 | 3,078,726 | 3,078.73 | 3,078,736 | 3,077,919.4 | 3,075,808 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, mod min as exp heavy) | 1,000,000 | 223,239 | 1,753,229 | 1,753.23 | 3,328,468 | 3,328,270.2 | 3,323,588 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1024 mod odd) | 1,000,000 | 223,239 | 6,379 | 6.38 | 853,596 | 851,418.8 | 843,836 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 128 mod odd) | 1,000,000 | 223,239 | 820,295 | 820.29 | 2,651,316 | 2,649,580.3 | 2,639,604 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 256 mod odd) | 1,000,000 | 223,239 | 492,093 | 492.09 | 3,137,256 | 3,130,686.4 | 3,127,496 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 32 mod odd) | 1,000,000 | 223,239 | 1,052,249 | 1,052.25 | 1,666,600 | 1,612,631.4 | 1,512,392 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 32 mod odd) | 1,000,000 | 223,239 | 806,990 | 806.99 | 3,489,464 | 3,488,567.7 | 3,481,656 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 32 mod odd) | 1,000,000 | 223,239 | 751,167 | 751.17 | 3,098,256 | 3,097,621.9 | 3,095,328 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 512 mod odd) | 1,000,000 | 223,239 | 5,225 | 5.22 | 3,216,300 | 3,215,128.8 | 3,214,348 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 64 mod odd) | 1,000,000 | 223,239 | 897,579 | 897.58 | 2,609,360 | 2,609,005.5 | 2,603,504 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 2 mod pawel) | 1,000,000 | 223,239 | 1,098,648 | 1,098.65 | 3,172,388 | 3,160,062.7 | 3,140,180 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 3 mod pawel) | 1,000,000 | 223,239 | 892,982 | 892.98 | 3,300,176 | 3,299,822.0 | 3,289,440 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 4 mod pawel) | 1,000,000 | 223,239 | 948,608 | 948.61 | 3,052,392 | 3,050,860.7 | 3,045,560 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1152 mod vul common) | 1,000,000 | 223,239 | 568,672 | 568.67 | 3,097,276 | 3,097,276 | 3,097,276 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1349 mod vul common) | 1,000,000 | 223,239 | 806,982 | 806.98 | 2,334,888 | 2,334,544.3 | 2,326,104 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1360 mod vul common) | 1,000,000 | 223,239 | 763,049 | 763.05 | 3,249,484 | 3,249,484 | 3,247,532 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1360 mod vul common) | 1,000,000 | 223,239 | 819,730 | 819.73 | 2,878,688 | 2,876,530.9 | 2,860,144 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 200 mod vul common) | 1,000,000 | 223,239 | 443,964 | 443.96 | 2,929,428 | 2,924,697.3 | 2,916,740 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 200 mod vul common) | 1,000,000 | 223,239 | 577,112 | 577.11 | 2,692,304 | 2,692,304 | 2,692,304 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 200 mod vul common) | 1,000,000 | 223,239 | 482,800 | 482.80 | 3,313,840 | 3,313,792.5 | 3,310,912 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1 mod vul example) | 1,000,000 | 223,239 | 1,028,921 | 1,028.92 | 2,160,452 | 2,139,080.1 | 2,027,716 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 2 mod vul example) | 1,000,000 | 223,239 | 976,261 | 976.26 | 2,452,236 | 2,452,086.9 | 2,450,284 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1 mod vul guido) | 1,000,000 | 223,239 | 698,372 | 698.37 | 2,039,168 | 2,023,544.7 | 2,005,008 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 2 mod vul guido) | 1,000,000 | 223,239 | 1,219,499 | 1,219.50 | 2,889,420 | 2,888,313.9 | 2,885,516 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 3 mod vul guido) | 1,000,000 | 223,239 | 1,719,570 | 1,719.57 | 3,264,108 | 3,263,216.5 | 3,263,132 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1 mod vul marius) | 1,000,000 | 223,239 | 1,096,325 | 1,096.33 | 3,037,752 | 3,028,202.0 | 3,015,304 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1 mod vul nagydani) | 1,000,000 | 223,239 | 728,858 | 728.86 | 3,070,936 | 3,068,472.1 | 3,061,176 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1 mod vul nagydani) | 1,000,000 | 223,239 | 541,252 | 541.25 | 2,471,508 | 2,470,513.7 | 2,460,772 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1 mod vul nagydani) | 1,000,000 | 223,239 | 515,055 | 515.05 | 1,287,916 | 1,224,186.4 | 1,085,884 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 2 mod vul nagydani) | 1,000,000 | 223,239 | 654,380 | 654.38 | 3,541,160 | 3,540,441.1 | 3,539,208 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 2 mod vul nagydani) | 1,000,000 | 223,239 | 1,710,897 | 1,710.90 | 2,703,040 | 2,702,670.9 | 2,700,112 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 2 mod vul nagydani) | 1,000,000 | 223,239 | 1,322,744 | 1,322.74 | 3,499,212 | 3,499,211.2 | 3,499,212 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 3 mod vul nagydani) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 3 mod vul nagydani) | 1,000,000 | 223,239 | 4,092,095 | 4,092.09 | 2,740,124 | 2,737,371.6 | 2,722,556 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 3 mod vul nagydani) | 1,000,000 | 223,239 | 3,200,980 | 3,200.98 | 3,286,512 | 3,285,556.0 | 3,280,656 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 4 mod vul nagydani) | 1,000,000 | 223,239 | 571,845 | 571.85 | 3,398,708 | 3,388,700.9 | 3,383,092 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 4 mod vul nagydani) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 4 mod vul nagydani) | 1,000,000 | 223,239 | 4,222,531 | 4,222.53 | 2,677,668 | 2,674,612.2 | 2,659,124 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 5 mod vul nagydani) | 1,000,000 | 223,239 | 531,775 | 531.77 | 3,355,772 | 3,354,707.1 | 3,352,844 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 5 mod vul nagydani) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 5 mod vul nagydani) | 1,000,000 | 223,239 | 4,247,664 | 4,247.66 | 2,363,444 | 2,358,833.9 | 2,334,164 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 1 mod vul pawel) | 1,000,000 | 223,239 | 2,109,591 | 2,109.59 | 3,085,568 | 3,085,412.6 | 3,077,760 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 2 mod vul pawel) | 1,000,000 | 223,239 | 1,031,809 | 1,031.81 | 3,310,912 | 3,310,912 | 3,310,912 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 3 mod vul pawel) | 1,000,000 | 223,239 | 996,610 | 996.61 | 2,839,656 | 2,837,120.4 | 2,828,920 |
| PRECOMPILE_MODEXP, CALL (fork Prague, 1M gas, 4 mod vul pawel) | 1,000,000 | 223,239 | 757,689 | 757.69 | 3,545,064 | 3,544,899.5 | 3,541,160 |
| MSIZE (fork Prague, 1M gas, 0 mem size) | 1,000,000 | 223,239 | 40,919 | 40.92 | 3,357,724 | 3,357,699.6 | 3,355,772 |
| MSIZE (fork Prague, 1M gas, 1000000 mem size) | 1,000,000 | 223,239 | 49,060 | 49.06 | 2,436,372 | 2,434,850.1 | 2,432,468 |
| MSIZE (fork Prague, 1M gas, 100000 mem size) | 1,000,000 | 223,239 | 49,922 | 49.92 | 2,849,412 | 2,849,412 | 2,849,412 |
| MSIZE (fork Prague, 1M gas, 1000 mem size) | 1,000,000 | 223,239 | 41,302 | 41.30 | 3,373,336 | 3,373,257.9 | 3,372,360 |
| MSIZE (fork Prague, 1M gas, 1 mem size) | 1,000,000 | 223,239 | 48,787 | 48.79 | 2,373,200 | 2,372,307.1 | 2,362,464 |
| PRECOMPILE_BLAKE2F, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| PRECOMPILE_BLS12_MAP_FP_TO_G1, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 88,758 | 88.76 | 1,148,092 | 1,145,409.9 | 1,134,428 |
| PRECOMPILE_BLS12_MAP_FP2_TO_G2, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 64,902 | 64.90 | 1,368,664 | 1,346,871.3 | 1,316,936 |
| PRECOMPILE_BLS12_G1ADD, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 63,417 | 63.42 | 2,898,204 | 2,896,870.1 | 2,891,372 |
| PRECOMPILE_BLS12_G1MSM, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 96,590 | 96.59 | 3,098,252 | 3,098,252 | 3,097,276 |
| PRECOMPILE_BLS12_G2ADD, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 65,070 | 65.07 | 3,546,040 | 3,546,040 | 3,545,064 |
| PRECOMPILE_BLS12_G2MSM, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 68,955 | 68.95 | 2,742,068 | 2,741,915.8 | 2,738,164 |
| PRECOMPILE_BLS12_PAIRING, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 127,227 | 127.23 | 2,701,088 | 2,701,021.6 | 2,692,304 |
| PRECOMPILE_EC_ADD, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 71,986 | 71.99 | 3,261,184 | 3,261,184 | 3,261,184 |
| CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 73,102 | 73.10 | 3,311,888 | 3,311,888 | 3,310,912 |
| CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 42,468 | 42.47 | 3,488,488 | 3,488,488 | 3,488,488 |
| PRECOMPILE_EC_MUL, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 312,262 | 312.26 | 2,977,244 | 2,975,494.7 | 2,975,292 |
| CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 9,522 | 9.52 | 3,309,936 | 3,309,875 | 3,307,008 |
| CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 297,308 | 297.31 | 625,956 | 603,336.4 | 250,196 |
| CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 11,592 | 11.59 | 2,627,900 | 2,627,900 | 2,623,020 |
| CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 304,777 | 304.78 | 2,283,164 | 2,282,916.2 | 2,273,404 |
| CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 9,338 | 9.34 | 3,345,044 | 3,344,986.6 | 3,345,044 |
| CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 157,970 | 157.97 | 3,116,776 | 3,116,776 | 3,116,776 |
| CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 289,463 | 289.46 | 2,589,848 | 2,588,510.4 | 2,585,944 |
| CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 237,705 | 237.71 | 3,505,064 | 3,505,064 | 3,505,064 |
| CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 5,513 | 5.51 | 3,384,072 | 3,383,876.8 | 3,375,288 |
| PRECOMPILE_ECRECOVER, CALL, STATICCALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 294,874 | 294.87 | 3,069,956 | 3,069,956 | 3,069,956 |
| PRECOMPILE_POINT_EVALUATION, CALL, STATICCALL (fork Prague, 1M gas, point evaluation) | 1,000,000 | 223,239 | 325,230 | 325.23 | 2,451,264 | 2,450,144.5 | 2,438,576 |
| PRECOMPILE_IDENTITY, CALL (fork Prague, 1M gas) | 1,000,000 | 223,239 | 26,810 | 26.81 | 2,976,268 | 2,976,268 | 2,976,268 |
| PRECOMPILE_RIPEMD-160, CALL (fork Prague, 1M gas, 160) | 1,000,000 | 223,239 | 18,575 | 18.57 | 2,024,792 | 2,020,888 | 2,001,368 |
| PRECOMPILE_SHA2-256, CALL (fork Prague, 1M gas, SHA2, 256) | 1,000,000 | 223,239 | 11,507 | 11.51 | 1,952,568 | 1,945,178.3 | 1,939,880 |
| PUSH0 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 39,776 | 39.78 | 3,127,504 | 3,127,504 | 3,126,528 |
| PUSH10 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 33,443 | 33.44 | 3,480,684 | 3,480,684 | 3,479,708 |
| PUSH11 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 35,730 | 35.73 | 3,307,984 | 3,307,984 | 3,307,984 |
| PUSH12 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 36,002 | 36.00 | 3,530,432 | 3,530,404.1 | 3,530,432 |
| PUSH13 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 43,723 | 43.72 | 2,581,064 | 2,581,044.3 | 2,576,184 |
| PUSH14 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 39,653 | 39.65 | 3,110,936 | 3,110,918.5 | 3,110,936 |
| PUSH15 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 39,275 | 39.27 | 3,117,756 | 3,117,756 | 3,116,780 |
| PUSH16 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 39,527 | 39.53 | 3,551,892 | 3,551,466.4 | 3,550,916 |
| PUSH17 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 39,680 | 39.68 | 3,277,732 | 3,277,732 | 3,275,780 |
| PUSH18 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 40,762 | 40.76 | 3,325,544 | 3,325,534.1 | 3,324,568 |
| PUSH19 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 42,590 | 42.59 | 3,324,568 | 3,324,557.3 | 3,324,568 |
| PUSH1 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 28,779 | 28.78 | 2,814,280 | 2,812,626.2 | 2,804,520 |
| PUSH20 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 52,204 | 52.20 | 2,542,744 | 2,541,494.7 | 2,538,840 |
| PUSH21 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 53,335 | 53.34 | 1,916,456 | 1,908,753.5 | 1,894,984 |
| PUSH22 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 56,596 | 56.60 | 2,398,572 | 2,397,847.6 | 2,391,740 |
| PUSH23 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 48,682 | 48.68 | 3,110,936 | 3,110,936 | 3,110,936 |
| PUSH24 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 60,569 | 60.57 | 2,533,964 | 2,532,866 | 2,532,012 |
| PUSH25 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 51,395 | 51.40 | 3,344,076 | 3,344,076 | 3,342,124 |
| PUSH26 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 62,706 | 62.71 | 2,438,324 | 2,438,324 | 2,436,372 |
| PUSH27 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 64,312 | 64.31 | 2,563,224 | 2,561,969.1 | 2,549,560 |
| PUSH28 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 55,182 | 55.18 | 3,506,044 | 3,506,044 | 3,505,068 |
| PUSH29 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 64,266 | 64.27 | 2,467,852 | 2,467,775.8 | 2,466,876 |
| PUSH2 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 30,611 | 30.61 | 2,702,064 | 2,702,064 | 2,702,064 |
| PUSH30 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 69,107 | 69.11 | 2,537,868 | 2,536,732.7 | 2,534,940 |
| PUSH31 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 58,805 | 58.80 | 3,410,420 | 3,404,320 | 3,400,660 |
| PUSH32 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 63,232 | 63.23 | 3,541,160 | 3,541,160 | 3,541,160 |
| PUSH3 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 33,469 | 33.47 | 2,996,760 | 2,996,760 | 2,987,976 |
| PUSH4 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 27,642 | 27.64 | 3,266,036 | 3,266,036 | 3,265,060 |
| PUSH5 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 35,384 | 35.38 | 849,440 | 840,009.9 | 822,112 |
| PUSH6 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 31,307 | 31.31 | 3,540,188 | 3,540,188 | 3,540,188 |
| PUSH7 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 38,677 | 38.68 | 2,720,360 | 2,718,880.3 | 2,715,480 |
| PUSH8 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 37,400 | 37.40 | 2,532,992 | 2,531,235 | 2,528,112 |
| PUSH9 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 37,577 | 37.58 | 1,815,668 | 1,803,956 | 1,781,508 |
| RETURN (fork Prague, 1M gas, 1KiB of non, zero data) | 1,000,000 | 223,239 | 48,806 | 48.81 | 1,891,080 | 1,875,240.9 | 1,858,872 |
| REVERT (fork Prague, 1M gas, 1KiB of non, zero data) | 1,000,000 | 223,239 | 53,399 | 53.40 | 2,574,936 | 2,571,926.7 | 2,563,224 |
| RETURN (fork Prague, 1M gas, 1KiB of zero data) | 1,000,000 | 223,239 | 53,451 | 53.45 | 3,370,408 | 3,370,408 | 3,370,408 |
| REVERT (fork Prague, 1M gas, 1KiB of zero data) | 1,000,000 | 223,239 | 56,316 | 56.32 | 3,479,708 | 3,479,708 | 3,477,756 |
| RETURN (fork Prague, 1M gas, 1MiB of non, zero data) | 1,000,000 | 223,239 | 6,818 | 6.82 | 3,094,352 | 3,094,352 | 3,094,352 |
| REVERT (fork Prague, 1M gas, 1MiB of non, zero data) | 1,000,000 | 223,239 | 7,129 | 7.13 | 2,404,164 | 2,402,781.3 | 2,396,356 |
| RETURN (fork Prague, 1M gas, 1MiB of zero data) | 1,000,000 | 223,239 | 5,159 | 5.16 | 641,568 | 641,324 | 636,688 |
| REVERT (fork Prague, 1M gas, 1MiB of zero data) | 1,000,000 | 223,239 | 6,016 | 6.02 | 2,932,356 | 2,932,160.8 | 2,928,452 |
| RETURN (fork Prague, 1M gas, empty) | 1,000,000 | 223,239 | 71,112 | 71.11 | 3,353,820 | 3,353,820 | 3,352,844 |
| REVERT (fork Prague, 1M gas, empty) | 1,000,000 | 223,239 | 95,963 | 95.96 | 3,044,584 | 3,043,282.7 | 3,037,752 |
| PRECOMPILE_IDENTITY (fork Prague, 1M gas, 0 returned size) | 1,000,000 | 223,239 | 30,411 | 30.41 | 3,289,440 | 3,289,440 | 3,289,440 |
| RETURNDATASIZE (fork Prague, 1M gas, 0 returned size) | 1,000,000 | 223,239 | 36,344 | 36.34 | 1,774,676 | 1,766,443.7 | 1,753,204 |
| RETURNDATASIZE (fork Prague, 1M gas, 0 returned size) | 1,000,000 | 223,239 | 35,137 | 35.14 | 2,053,808 | 2,050,880 | 2,042,096 |
| PRECOMPILE_IDENTITY (fork Prague, 1M gas, 1 returned size) | 1,000,000 | 223,239 | 29,829 | 29.83 | 3,499,216 | 3,499,171.5 | 3,499,216 |
| RETURNDATASIZE (fork Prague, 1M gas, 1 returned size) | 1,000,000 | 223,239 | 36,381 | 36.38 | 2,678,644 | 2,678,644 | 2,677,668 |
| RETURNDATASIZE (fork Prague, 1M gas, 1 returned size) | 1,000,000 | 223,239 | 36,402 | 36.40 | 2,708,896 | 2,706,996.8 | 2,703,040 |
| RETURNDATASIZE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 36,876 | 36.88 | 2,361,228 | 2,359,197.5 | 2,347,564 |
| SHL, SHR, SAR (fork Prague, 1M gas, shift right SAR) | 1,000,000 | 223,239 | 73,283 | 73.28 | 2,622,044 | 2,622,044 | 2,617,164 |
| SHL, SHR, SAR (fork Prague, 1M gas, shift right SHR) | 1,000,000 | 223,239 | 57,032 | 57.03 | 3,536,284 | 3,536,204.3 | 3,532,380 |
| SWAP10 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 75,972 | 75.97 | 1,815,928 | 1,807,597.1 | 1,793,480 |
| SWAP11 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 73,645 | 73.64 | 2,783,056 | 2,782,964.2 | 2,772,320 |
| SWAP12 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 64,209 | 64.21 | 3,530,432 | 3,530,183.9 | 3,528,480 |
| SWAP13 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 61,311 | 61.31 | 3,116,784 | 3,116,784 | 3,115,808 |
| SWAP14 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 73,777 | 73.78 | 2,383,676 | 2,383,629.9 | 2,374,892 |
| SWAP15 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 75,045 | 75.05 | 1,408,680 | 1,389,357.8 | 1,367,688 |
| SWAP16 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 62,677 | 62.68 | 3,530,428 | 3,530,428 | 3,530,428 |
| SWAP1 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 60,225 | 60.23 | 3,191,908 | 3,189,124.6 | 3,187,028 |
| SWAP2 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 72,902 | 72.90 | 3,007,496 | 3,006,461.4 | 3,004,568 |
| SWAP3 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 61,452 | 61.45 | 3,400,660 | 3,400,017.9 | 3,399,684 |
| SWAP4 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 74,049 | 74.05 | 2,827,944 | 2,826,888.9 | 2,825,992 |
| SWAP5 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 61,749 | 61.75 | 3,258,268 | 3,258,230.2 | 3,251,436 |
| SWAP6 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 73,870 | 73.87 | 724,524 | 705,244.5 | 666,940 |
| SWAP7 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 74,224 | 74.22 | 2,534,216 | 2,534,159.4 | 2,526,408 |
| SWAP8 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 62,775 | 62.77 | 3,281,636 | 3,281,636 | 3,280,660 |
| SWAP9 (fork Prague, 1M gas) | 1,000,000 | 223,239 | 73,989 | 73.99 | 1,646,824 | 1,634,754.1 | 1,609,736 |
| TLOAD (fork Prague, 1M gas, val mut: False, key mut: False) | 1,000,000 | 223,239 | 9,757 | 9.76 | 1,265,208 | 1,264,320.7 | 1,262,280 |
| TLOAD (fork Prague, 1M gas, val mut: False, key mut: True) | 1,000,000 | 223,239 | 9,103 | 9.10 | 2,385,888 | 2,385,278 | 2,380,032 |
| TLOAD (fork Prague, 1M gas, val mut: True, key mut: False) | 1,000,000 | 223,239 | 9,574 | 9.57 | 3,317,740 | 3,317,170.7 | 3,313,836 |
| TLOAD (fork Prague, 1M gas, val mut: True, key mut: True) | 1,000,000 | 223,239 | 10,871 | 10.87 | 2,825,016 | 2,824,483.6 | 2,820,136 |
| TSTORE (fork Prague, 1M gas, dense val mut: False, key mut: False) | 1,000,000 | 223,239 | 28,185 | 28.18 | 2,341,720 | 2,341,273.8 | 2,334,888 |
| TSTORE (fork Prague, 1M gas, dense val mut: False, key mut: True) | 1,000,000 | 223,239 | 23,656 | 23.66 | 3,370,408 | 3,370,408 | 3,370,408 |
| TSTORE (fork Prague, 1M gas, dense val mut: True, key mut: False) | 1,000,000 | 223,239 | 53,475 | 53.48 | 2,503,964 | 2,503,229.8 | 2,497,132 |
| TSTORE (fork Prague, 1M gas, dense val mut: True, key mut: True) | 1,000,000 | 223,239 | 43,181 | 43.18 | 3,371,388 | 3,371,388 | 3,371,388 |
| ISZERO (fork Prague, 1M gas) | 1,000,000 | 223,239 | 65,813 | 65.81 | 3,482,636 | 3,481,679.1 | 3,480,684 |
| NOT (fork Prague, 1M gas) | 1,000,000 | 223,239 | 29,597 | 29.60 | 2,341,720 | 2,341,720 | 2,341,720 |
| ADDRESS (fork Prague, 1M gas) | 1,000,000 | 223,239 | 72,814 | 72.81 | 3,521,652 | 3,520,690.1 | 3,520,676 |
| BASEFEE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 41,740 | 41.74 | 3,118,724 | 3,117,770.7 | 3,117,748 |
| BLOBBASEFEE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 65,999 | 66.00 | 1,714,424 | 1,705,659.5 | 1,693,928 |
| CALLER (fork Prague, 1M gas) | 1,000,000 | 223,239 | 74,891 | 74.89 | 3,504,088 | 3,504,088 | 3,504,088 |
| CHAINID (fork Prague, 1M gas) | 1,000,000 | 223,239 | 42,561 | 42.56 | 3,518,724 | 3,518,724 | 3,513,844 |
| CODESIZE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 48,382 | 48.38 | 1,260,328 | 1,253,413.3 | 1,243,736 |
| COINBASE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 73,301 | 73.30 | 3,494,344 | 3,494,344 | 3,492,392 |
| GASLIMIT (fork Prague, 1M gas) | 1,000,000 | 223,239 | 42,617 | 42.62 | 3,490,440 | 3,490,440 | 3,490,440 |
| GASPRICE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 50,659 | 50.66 | 3,367,484 | 3,366,139.3 | 3,359,676 |
| GAS (fork Prague, 1M gas) | 1,000,000 | 223,239 | 50,732 | 50.73 | 2,861,124 | 2,860,570.1 | 2,855,268 |
| NUMBER (fork Prague, 1M gas) | 1,000,000 | 223,239 | 45,356 | 45.36 | 3,346,996 | 3,346,996 | 3,346,020 |
| ORIGIN (fork Prague, 1M gas) | 1,000,000 | 223,239 | 89,108 | 89.11 | 816,264 | 790,018.4 | 745,992 |
| PREVRANDAO (fork Prague, 1M gas) | 1,000,000 | 223,239 | 94,005 | 94.00 | 3,309,936 | 3,309,936 | 3,308,960 |
| TIMESTAMP (fork Prague, 1M gas) | 1,000,000 | 223,239 | 51,617 | 51.62 | 2,854,292 | 2,853,966.7 | 2,851,364 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: False, 0 bytes, call) | 1,000,000 | 223,239 | 55,684 | 55.68 | 1,316,936 | 1,310,019.9 | 1,291,560 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: False, 0 bytes, transaction) | 1,000,000 | 223,239 | 56,260 | 56.26 | 3,002,616 | 3,002,616 | 3,001,640 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: False, 100 bytes, call) | 1,000,000 | 223,239 | 39,967 | 39.97 | 3,197,756 | 3,197,058.9 | 3,195,804 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: False, 100 bytes, transaction) | 1,000,000 | 223,239 | 41,927 | 41.93 | 2,743,044 | 2,743,044 | 2,743,044 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: False, 10KiB, call) | 1,000,000 | 223,239 | 20,272 | 20.27 | 3,371,388 | 3,371,388 | 3,370,412 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: False, 10KiB, transaction) | 1,000,000 | 223,239 | 12,598 | 12.60 | 2,586,920 | 2,586,815.4 | 2,582,040 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: False, 1MiB, call) | 1,000,000 | 223,239 | 4,119 | 4.12 | 3,329,444 | 3,329,444 | 3,327,492 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: False, 1MiB, transaction) | 1,000,000 | 223,239 | 5,066 | 5.07 | 3,312,864 | 3,312,864 | 3,311,888 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: True, 0 bytes, call) | 1,000,000 | 223,239 | 38,063 | 38.06 | 2,177,044 | 2,175,917.8 | 2,166,308 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: True, 0 bytes, transaction) | 1,000,000 | 223,239 | 37,877 | 37.88 | 1,680,264 | 1,676,116 | 1,667,576 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: True, 100 bytes, call) | 1,000,000 | 223,239 | 23,694 | 23.69 | 3,263,136 | 3,263,136 | 3,261,184 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: True, 100 bytes, transaction) | 1,000,000 | 223,239 | 23,371 | 23.37 | 3,494,340 | 3,494,340 | 3,493,364 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: True, 10KiB, call) | 1,000,000 | 223,239 | 17,299 | 17.30 | 3,074,832 | 3,074,832 | 3,074,832 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: True, 10KiB, transaction) | 1,000,000 | 223,239 | 9,241 | 9.24 | 3,359,676 | 3,359,676 | 3,358,700 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: True, 1MiB, call) | 1,000,000 | 223,239 | 5,289 | 5.29 | 2,537,864 | 2,537,864 | 2,536,888 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: False, fixed src dst: True, 1MiB, transaction) | 1,000,000 | 223,239 | 5,852 | 5.85 | 2,091,868 | 2,091,705.3 | 2,085,036 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: True, fixed src dst: False, 100 bytes, call) | 1,000,000 | 223,239 | 48,750 | 48.75 | 3,072,884 | 3,072,884 | 3,071,908 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: True, fixed src dst: False, 100 bytes, transaction) | 1,000,000 | 223,239 | 48,429 | 48.43 | 1,437,960 | 1,426,906.2 | 1,408,680 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: True, fixed src dst: False, 10KiB, call) | 1,000,000 | 223,239 | 18,273 | 18.27 | 3,261,192 | 3,261,162.3 | 3,260,216 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: True, fixed src dst: False, 10KiB, transaction) | 1,000,000 | 223,239 | 17,955 | 17.95 | 3,497,264 | 3,497,264 | 3,497,264 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: True, fixed src dst: True, 100 bytes, call) | 1,000,000 | 223,239 | 30,354 | 30.35 | 2,940,164 | 2,938,537.3 | 2,934,308 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: True, fixed src dst: True, 100 bytes, transaction) | 1,000,000 | 223,239 | 30,234 | 30.23 | 775,524 | 768,296.3 | 746,244 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: True, fixed src dst: True, 10KiB, call) | 1,000,000 | 223,239 | 13,798 | 13.80 | 3,264,108 | 3,264,108 | 3,264,108 |
| CALLDATACOPY (fork Prague, 1M gas, non zero data: True, fixed src dst: True, 10KiB, transaction) | 1,000,000 | 223,239 | 14,977 | 14.98 | 2,445,156 | 2,445,156 | 2,438,324 |
| CODECOPY (fork Prague, 1M gas, fixed src dst: False, 0 bytes) | 1,000,000 | 223,239 | 55,008 | 55.01 | 2,604,480 | 2,604,480 | 2,603,504 |
| CODECOPY (fork Prague, 1M gas, fixed src dst: False, 0.25x max code size) | 1,000,000 | 223,239 | 27,848 | 27.85 | 2,043,072 | 2,042,514.3 | 2,039,168 |
| CODECOPY (fork Prague, 1M gas, fixed src dst: False, 0.50x max code size) | 1,000,000 | 223,239 | 24,362 | 24.36 | 666,944 | 658,122.9 | 640,592 |
| CODECOPY (fork Prague, 1M gas, fixed src dst: False, 0.75x max code size) | 1,000,000 | 223,239 | 22,845 | 22.84 | 2,347,564 | 2,347,224.5 | 2,340,732 |
| CODECOPY (fork Prague, 1M gas, fixed src dst: False, max code size) | 1,000,000 | 223,239 | 18,665 | 18.66 | 3,117,748 | 3,117,748 | 3,117,748 |
| CODECOPY (fork Prague, 1M gas, fixed src dst: True, 0 bytes) | 1,000,000 | 223,239 | 35,006 | 35.01 | 2,583,720 | 2,583,557.3 | 2,582,744 |
| CODECOPY (fork Prague, 1M gas, fixed src dst: True, 0.25x max code size) | 1,000,000 | 223,239 | 16,370 | 16.37 | 1,296,700 | 1,293,040 | 1,287,916 |
| CODECOPY (fork Prague, 1M gas, fixed src dst: True, 0.50x max code size) | 1,000,000 | 223,239 | 14,291 | 14.29 | 3,263,136 | 3,263,136 | 3,263,136 |
| CODECOPY (fork Prague, 1M gas, fixed src dst: True, 0.75x max code size) | 1,000,000 | 223,239 | 16,848 | 16.85 | 2,061,616 | 2,058,896.9 | 2,054,784 |
| CODECOPY (fork Prague, 1M gas, fixed src dst: True, max code size) | 1,000,000 | 223,239 | 16,940 | 16.94 | 2,639,612 | 2,637,430.1 | 2,627,900 |
| MCOPY (fork Prague, 1M gas, fixed src dst: False, 0 bytes) | 1,000,000 | 223,239 | 53,965 | 53.97 | 2,292,924 | 2,291,818.8 | 2,283,164 |
| MCOPY (fork Prague, 1M gas, fixed src dst: False, 100 bytes) | 1,000,000 | 223,239 | 42,854 | 42.85 | 3,214,348 | 3,210,915.2 | 3,206,540 |
| MCOPY (fork Prague, 1M gas, fixed src dst: False, 10KiB) | 1,000,000 | 223,239 | 39,801 | 39.80 | 2,988,952 | 2,987,844.1 | 2,982,120 |
| MCOPY (fork Prague, 1M gas, fixed src dst: False, 1MiB) | 1,000,000 | 223,239 | 5,906 | 5.91 | 3,310,912 | 3,310,912 | 3,310,912 |
| MCOPY (fork Prague, 1M gas, fixed src dst: True, 0 bytes) | 1,000,000 | 223,239 | 27,895 | 27.89 | 3,311,888 | 3,311,871.2 | 3,311,888 |
| MCOPY (fork Prague, 1M gas, fixed src dst: True, 100 bytes) | 1,000,000 | 223,239 | 27,998 | 28.00 | 3,529,456 | 3,529,043.1 | 3,527,504 |
| MCOPY (fork Prague, 1M gas, fixed src dst: True, 10KiB) | 1,000,000 | 223,239 | 25,239 | 25.24 | 554,948 | 467,295.7 | 250,436 |
| MCOPY (fork Prague, 1M gas, fixed src dst: True, 1MiB) | 1,000,000 | 223,239 | 6,831 | 6.83 | 1,723,208 | 1,719,182 | 1,714,424 |
| RETURNDATACOPY (fork Prague, 1M gas, fixed dst: False, 0 bytes) | 1,000,000 | 223,239 | 54,889 | 54.89 | 1,693,928 | 1,687,551.5 | 1,681,240 |
| RETURNDATACOPY (fork Prague, 1M gas, fixed dst: False, 100 bytes) | 1,000,000 | 223,239 | 44,479 | 44.48 | 2,998,712 | 2,998,712 | 2,998,712 |
| RETURNDATACOPY (fork Prague, 1M gas, fixed dst: False, 10KiB) | 1,000,000 | 223,239 | 21,297 | 21.30 | 3,496,288 | 3,496,288 | 3,495,312 |
| RETURNDATACOPY (fork Prague, 1M gas, fixed dst: False, 1MiB) | 1,000,000 | 223,239 | 6,545 | 6.54 | 3,139,208 | 3,139,208 | 3,137,256 |
| RETURNDATACOPY (fork Prague, 1M gas, fixed dst: True, 0 bytes) | 1,000,000 | 223,239 | 45,248 | 45.25 | 2,947,972 | 2,947,383.0 | 2,940,164 |
| RETURNDATACOPY (fork Prague, 1M gas, fixed dst: True, 100 bytes) | 1,000,000 | 223,239 | 32,349 | 32.35 | 2,997,736 | 2,997,736 | 2,997,736 |
| RETURNDATACOPY (fork Prague, 1M gas, fixed dst: True, 10KiB) | 1,000,000 | 223,239 | 14,557 | 14.56 | 3,196,784 | 3,194,122.2 | 3,193,856 |
| RETURNDATACOPY (fork Prague, 1M gas, fixed dst: True, 1MiB) | 1,000,000 | 223,239 | 7,818 | 7.82 | 3,008,472 | 3,008,472 | 3,008,472 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 0 bytes data, log0) | 1,000,000 | 223,239 | 8,528 | 8.53 | 2,472,484 | 2,472,484 | 2,471,508 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 0 bytes data, log1) | 1,000,000 | 223,239 | 7,684 | 7.68 | 2,917,720 | 2,915,768 | 2,910,888 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 0 bytes data, log2) | 1,000,000 | 223,239 | 6,400 | 6.40 | 3,310,912 | 3,310,912 | 3,310,912 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 0 bytes data, log3) | 1,000,000 | 223,239 | 7,061 | 7.06 | 2,714,504 | 2,714,016 | 2,711,576 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 0 bytes data, log4) | 1,000,000 | 223,239 | 5,806 | 5.81 | 3,334,324 | 3,334,094.4 | 3,329,444 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB non zero data, log0) | 1,000,000 | 223,239 | 5,183 | 5.18 | 3,505,068 | 3,505,068 | 3,504,092 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB non zero data, log1) | 1,000,000 | 223,239 | 6,411 | 6.41 | 3,062,152 | 3,062,152 | 3,060,200 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB non zero data, log2) | 1,000,000 | 223,239 | 6,207 | 6.21 | 637,668 | 636,184.5 | 625,956 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB non zero data, log3) | 1,000,000 | 223,239 | 6,271 | 6.27 | 2,222,912 | 2,222,912 | 2,219,984 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB non zero data, log4) | 1,000,000 | 223,239 | 6,070 | 6.07 | 2,800,616 | 2,800,182.2 | 2,795,736 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB zeros data, log0) | 1,000,000 | 223,239 | 5,326 | 5.33 | 2,651,316 | 2,651,316 | 2,651,316 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB zeros data, log1) | 1,000,000 | 223,239 | 6,049 | 6.05 | 2,883,568 | 2,883,568 | 2,883,568 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB zeros data, log2) | 1,000,000 | 223,239 | 4,621 | 4.62 | 3,346,996 | 3,346,996 | 3,346,996 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB zeros data, log3) | 1,000,000 | 223,239 | 5,041 | 5.04 | 3,110,932 | 3,110,932 | 3,110,932 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, non zero topic, 1 MiB zeros data, log4) | 1,000,000 | 223,239 | 6,186 | 6.19 | 3,069,956 | 3,069,956 | 3,069,956 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 0 bytes data, log0) | 1,000,000 | 223,239 | 8,465 | 8.46 | 2,273,404 | 2,269,569.7 | 2,264,620 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 0 bytes data, log1) | 1,000,000 | 223,239 | 6,583 | 6.58 | 3,311,888 | 3,311,888 | 3,311,888 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 0 bytes data, log2) | 1,000,000 | 223,239 | 6,420 | 6.42 | 3,111,912 | 3,111,505.3 | 3,106,056 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 0 bytes data, log3) | 1,000,000 | 223,239 | 7,507 | 7.51 | 1,781,508 | 1,780,253.1 | 1,775,652 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 0 bytes data, log4) | 1,000,000 | 223,239 | 7,204 | 7.20 | 2,740,120 | 2,740,120 | 2,739,144 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB non zero data, log0) | 1,000,000 | 223,239 | 5,055 | 5.05 | 3,191,904 | 3,191,904 | 3,190,928 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB non zero data, log1) | 1,000,000 | 223,239 | 5,228 | 5.23 | 3,271,884 | 3,271,617.8 | 3,267,980 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB non zero data, log2) | 1,000,000 | 223,239 | 6,592 | 6.59 | 2,851,364 | 2,851,364 | 2,851,364 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB non zero data, log3) | 1,000,000 | 223,239 | 5,240 | 5.24 | 3,276,764 | 3,276,764 | 3,276,764 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB non zero data, log4) | 1,000,000 | 223,239 | 6,213 | 6.21 | 2,901,132 | 2,900,481.3 | 2,897,228 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB zeros data, log0) | 1,000,000 | 223,239 | 6,206 | 6.21 | 2,946,992 | 2,946,992 | 2,946,992 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB zeros data, log1) | 1,000,000 | 223,239 | 5,685 | 5.68 | 2,981,148 | 2,981,148 | 2,978,220 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB zeros data, log2) | 1,000,000 | 223,239 | 6,422 | 6.42 | 2,734,996 | 2,733,206.7 | 2,729,140 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB zeros data, log3) | 1,000,000 | 223,239 | 6,064 | 6.06 | 2,710,848 | 2,710,848 | 2,708,896 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: False, zeros topic, 1 MiB zeros data, log4) | 1,000,000 | 223,239 | 5,534 | 5.53 | 3,075,812 | 3,075,812 | 3,073,860 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 0 bytes data, log0) | 1,000,000 | 223,239 | 7,550 | 7.55 | 2,348,540 | 2,347,785.8 | 2,346,588 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 0 bytes data, log1) | 1,000,000 | 223,239 | 5,632 | 5.63 | 3,345,044 | 3,344,230.7 | 3,344,068 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 0 bytes data, log2) | 1,000,000 | 223,239 | 5,912 | 5.91 | 3,499,216 | 3,499,216 | 3,498,240 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 0 bytes data, log3) | 1,000,000 | 223,239 | 6,995 | 7.00 | 2,722,312 | 2,722,149.3 | 2,721,336 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 0 bytes data, log4) | 1,000,000 | 223,239 | 6,960 | 6.96 | 2,659,124 | 2,657,904 | 2,651,316 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB non zero data, log0) | 1,000,000 | 223,239 | 5,303 | 5.30 | 3,194,832 | 3,194,636.8 | 3,192,880 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB non zero data, log1) | 1,000,000 | 223,239 | 5,237 | 5.24 | 3,497,264 | 3,497,264 | 3,497,264 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB non zero data, log2) | 1,000,000 | 223,239 | 6,172 | 6.17 | 1,688,788 | 1,684,396 | 1,674,148 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB non zero data, log3) | 1,000,000 | 223,239 | 5,119 | 5.12 | 3,560,672 | 3,560,672 | 3,560,672 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB non zero data, log4) | 1,000,000 | 223,239 | 5,917 | 5.92 | 2,001,368 | 1,997,220 | 1,992,584 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB zeros data, log0) | 1,000,000 | 223,239 | 6,413 | 6.41 | 2,855,268 | 2,855,024 | 2,854,292 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB zeros data, log1) | 1,000,000 | 223,239 | 5,226 | 5.23 | 3,527,504 | 3,526,853.3 | 3,525,552 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB zeros data, log2) | 1,000,000 | 223,239 | 6,529 | 6.53 | 2,828,920 | 2,828,920 | 2,827,944 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB zeros data, log3) | 1,000,000 | 223,239 | 6,639 | 6.64 | 1,512,392 | 1,510,928 | 1,507,512 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, non zero topic, 1 MiB zeros data, log4) | 1,000,000 | 223,239 | 5,979 | 5.98 | 2,433,700 | 2,433,433.8 | 2,427,844 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 0 bytes data, log0) | 1,000,000 | 223,239 | 8,207 | 8.21 | 1,894,008 | 1,893,520 | 1,891,080 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 0 bytes data, log1) | 1,000,000 | 223,239 | 5,833 | 5.83 | 3,127,504 | 3,127,504 | 3,127,504 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 0 bytes data, log2) | 1,000,000 | 223,239 | 5,667 | 5.67 | 3,248,508 | 3,248,450.6 | 3,248,508 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 0 bytes data, log3) | 1,000,000 | 223,239 | 5,666 | 5.67 | 3,261,188 | 3,261,188 | 3,260,212 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 0 bytes data, log4) | 1,000,000 | 223,239 | 5,744 | 5.74 | 3,349,924 | 3,349,802 | 3,347,972 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB non zero data, log0) | 1,000,000 | 223,239 | 6,514 | 6.51 | 2,786,960 | 2,786,693.8 | 2,783,056 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB non zero data, log1) | 1,000,000 | 223,239 | 6,210 | 6.21 | 2,716,700 | 2,716,130.7 | 2,710,844 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB non zero data, log2) | 1,000,000 | 223,239 | 5,540 | 5.54 | 3,127,508 | 3,127,312 | 3,126,532 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB non zero data, log3) | 1,000,000 | 223,239 | 5,190 | 5.19 | 3,494,340 | 3,494,340 | 3,493,364 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB non zero data, log4) | 1,000,000 | 223,239 | 5,280 | 5.28 | 3,276,760 | 3,276,760 | 3,275,784 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB zeros data, log0) | 1,000,000 | 223,239 | 6,378 | 6.38 | 1,992,584 | 1,992,584 | 1,982,824 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB zeros data, log1) | 1,000,000 | 223,239 | 5,381 | 5.38 | 3,119,700 | 3,119,700 | 3,117,748 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB zeros data, log2) | 1,000,000 | 223,239 | 6,106 | 6.11 | 2,102,604 | 2,099,838.7 | 2,093,820 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB zeros data, log3) | 1,000,000 | 223,239 | 5,045 | 5.04 | 3,196,784 | 3,196,784 | 3,196,784 |
| LOG0, LOG1, LOG2, LOG3, LOG4 (fork Prague, 1M gas, fixed offset: True, zeros topic, 1 MiB zeros data, log4) | 1,000,000 | 223,239 | 4,991 | 4.99 | 3,127,500 | 3,127,500 | 3,126,524 |
| BALANCE (fork Prague, 1M gas, absent accounts: False) | 1,000,000 | 223,239 | 16,783 | 16.78 | 3,531,408 | 3,531,408 | 3,531,408 |
| BALANCE (fork Prague, 1M gas, absent accounts: True) | 1,000,000 | 223,239 | 9,915 | 9.91 | 3,499,216 | 3,499,216 | 3,499,216 |
| BALANCE (fork Prague, 1M gas, absent target: False) | 1,000,000 | 223,239 | 24,329 | 24.33 | 2,066,492 | 2,065,570.2 | 2,060,636 |
| CALLCODE (fork Prague, 1M gas, absent target: False) | 1,000,000 | 223,239 | 87,393 | 87.39 | 2,412,948 | 2,412,464.4 | 2,402,212 |
| CALL (fork Prague, 1M gas, absent target: False) | 1,000,000 | 223,239 | 73,708 | 73.71 | 3,500,188 | 3,500,188 | 3,499,212 |
| DELEGATECALL (fork Prague, 1M gas, absent target: False) | 1,000,000 | 223,239 | 77,789 | 77.79 | 3,097,272 | 3,097,272 | 3,097,272 |
| EXTCODEHASH (fork Prague, 1M gas, absent target: False) | 1,000,000 | 223,239 | 25,894 | 25.89 | 2,433,700 | 2,433,700 | 2,432,724 |
| EXTCODESIZE (fork Prague, 1M gas, absent target: False) | 1,000,000 | 223,239 | 20,209 | 20.21 | 3,347,972 | 3,347,972 | 3,347,972 |
| STATICCALL (fork Prague, 1M gas, absent target: False) | 1,000,000 | 223,239 | 88,621 | 88.62 | 2,397,336 | 2,396,769.0 | 2,391,480 |
| BALANCE (fork Prague, 1M gas, absent target: True) | 1,000,000 | 223,239 | 29,650 | 29.65 | 2,593,748 | 2,593,748 | 2,593,748 |
| CALLCODE (fork Prague, 1M gas, absent target: True) | 1,000,000 | 223,239 | 77,731 | 77.73 | 1,837,400 | 1,826,971.3 | 1,815,928 |
| CALL (fork Prague, 1M gas, absent target: True) | 1,000,000 | 223,239 | 77,416 | 77.42 | 1,752,228 | 1,723,758.8 | 1,689,764 |
| DELEGATECALL (fork Prague, 1M gas, absent target: True) | 1,000,000 | 223,239 | 56,023 | 56.02 | 3,127,500 | 3,127,500 | 3,127,500 |
| EXTCODEHASH (fork Prague, 1M gas, absent target: True) | 1,000,000 | 223,239 | 30,080 | 30.08 | 3,493,360 | 3,493,360 | 3,493,360 |
| EXTCODESIZE (fork Prague, 1M gas, absent target: True) | 1,000,000 | 223,239 | 30,043 | 30.04 | 2,330,264 | 2,329,468.7 | 2,327,336 |
| STATICCALL (fork Prague, 1M gas, absent target: True) | 1,000,000 | 223,239 | 65,783 | 65.78 | 3,371,388 | 3,371,368 | 3,371,388 |
| BLOCKHASH (fork Prague, 1M gas) | 1,000,000 | 223,239 | 41,288 | 41.29 | 1,986,468 | 1,984,190.7 | 1,976,708 |
| EXTCODECOPY (fork Prague, 1M gas, 1KiB) | 1,000,000 | 223,239 | 20,967 | 20.97 | 746,244 | 739,296.2 | 727,700 |
| EXTCODECOPY (fork Prague, 1M gas, 512) | 1,000,000 | 223,239 | 19,549 | 19.55 | 3,495,312 | 3,495,312 | 3,494,336 |
| EXTCODECOPY (fork Prague, 1M gas, 5KiB) | 1,000,000 | 223,239 | 15,549 | 15.55 | 3,342,124 | 3,342,124 | 3,342,124 |
| SELFBALANCE (fork Prague, 1M gas) | 1,000,000 | 223,239 | 321,628 | 321.63 | 2,817,208 | 2,817,014.5 | 2,815,256 |
| SELFDESTRUCT (fork Prague, 1M gas, value bearing: False) | 965,720 | 223,239 | 6,423 | 6.42 | 2,771,344 | 2,771,344 | 2,769,392 |
| SELFDESTRUCT (fork Prague, 1M gas, value bearing: True) | 965,720 | 223,239 | 5,768 | 5.77 | 3,263,136 | 3,263,136 | 3,263,136 |
| SELFDESTRUCT (fork Prague, 1M gas, value bearing: False) | 998,771 | 223,239 | 13,023 | 13.02 | 2,928,452 | 2,928,452 | 2,928,452 |
| SELFDESTRUCT (fork Prague, 1M gas, value bearing: True) | 998,771 | 223,239 | 14,336 | 14.34 | 2,622,044 | 2,622,044 | 2,622,044 |
| SELFDESTRUCT (fork Prague, 1M gas, value bearing: False) | 989,164 | 223,239 | 6,420 | 6.42 | 2,301,708 | 2,298,454.7 | 2,292,924 |
| SELFDESTRUCT (fork Prague, 1M gas, value bearing: True) | 989,164 | 223,239 | 5,285 | 5.29 | 3,371,384 | 3,371,384 | 3,371,384 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: False, SSLOAD) | 999,749 | 223,239 | 22,151 | 22.15 | 2,759,632 | 2,758,572.9 | 2,748,896 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: False) | 1,000,000 | 223,239 | 11,729 | 11.73 | 3,302,128 | 3,302,128 | 3,300,176 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: False) | 998,963 | 223,239 | 11,774 | 11.77 | 3,311,888 | 3,311,888 | 3,311,888 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: False, SSTORE new value) | 998,957 | 223,239 | 15,488 | 15.49 | 2,720,604 | 2,720,378.8 | 2,716,700 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: False) | 1,000,000 | 223,239 | 21,918 | 21.92 | 2,107,484 | 2,105,152.4 | 2,102,604 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: False) | 999,015 | 223,239 | 22,096 | 22.10 | 2,449,060 | 2,449,060 | 2,447,108 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: False, SSTORE same value) | 999,009 | 223,239 | 22,042 | 22.04 | 919,964 | 901,474.2 | 877,020 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: True, SSLOAD) | 999,749 | 223,239 | 15,490 | 15.49 | 2,582,744 | 2,581,702.9 | 2,575,912 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: True) | 1,000,000 | 223,239 | 7,061 | 7.06 | 2,465,900 | 2,464,815.6 | 2,452,236 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: True) | 995,213 | 223,239 | 6,832 | 6.83 | 795,040 | 783,902.1 | 776,496 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: True, SSTORE new value) | 995,207 | 223,239 | 6,220 | 6.22 | 2,378,080 | 2,377,917.3 | 2,372,224 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: True) | 1,000,000 | 223,239 | 6,286 | 6.29 | 2,432,468 | 2,432,142.7 | 2,425,636 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: True) | 995,081 | 223,239 | 6,601 | 6.60 | 858,224 | 856,760 | 849,440 |
| SLOAD, SSTORE (fork Prague, 1M gas, absent slots: True, SSTORE same value) | 995,075 | 223,239 | 6,719 | 6.72 | 2,729,144 | 2,729,144 | 2,722,312 |
| SLOAD, SSTORE (fork Prague, 1M gas, SLOAD) | 1,000,000 | 223,239 | 26,544 | 26.54 | 726,724 | 718,807.6 | 707,204 |
| SLOAD, SSTORE (fork Prague, 1M gas, SSTORE new value) | 1,000,000 | 223,239 | 54,817 | 54.82 | 3,288,464 | 3,288,464 | 3,288,464 |
| SLOAD, SSTORE (fork Prague, 1M gas, SSTORE same value) | 1,000,000 | 223,239 | 48,408 | 48.41 | 3,002,616 | 3,002,616 | 3,001,640 |

## Summary Statistics

- **Total Tests:** 508
- **Successful Tests:** 501
- **Failed Tests:** 7

### Proving Time (ms)
- **Average:** 226,639.7
- **Minimum:** 4,119
- **Maximum:** 4,247,664

### Peak Memory Usage (MB)
- **Average:** 2,729,624
- **Minimum:** 554,948
- **Maximum:** 3,561,652

### Proof Size (bytes)
- **Average:** 223,239
- **Minimum:** 223,239
- **Maximum:** 223,239
