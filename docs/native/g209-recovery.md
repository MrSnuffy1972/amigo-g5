# Game 209 (SensibleWorldOfSoccer v1.7 0840): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00209). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3516730 top-level bytes in 242 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 13738 raw-word lines |
| Functions named and described | pass, 222 of 222 named (0 author symbols), 222 of 222 described (20 by rule, 200 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1297 of 1297 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 390042 bytes, 2 functions |
| Independent decoder cross-check (Ghidra against capstone) | 18117 of 18126 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 231786 |
| code | 16222 |
| data | 3268716 |
| padding | 6 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 557 | 26 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3394 | 175 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 354 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 53 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 3 |
| SWOS2#unpacked (raw code file, load address unknown) | 1 | 1 | 1 |
| SWOS2.REL#unpacked (raw code file, load address unknown) | 1 | 13759 | 1 |

## Cases handled

- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840.hdf: hard-disk image (AmigaDOS OFS, 4000 blocks): its filesystem parses and all 125 files in it are byte-identical to the extracted tree SensibleWorldOfSoccer_v1.7_0840_hdf, which is recovered file by file; no block outside the filesystem holds data
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/SWOS2: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (149420 -> 335006 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/SWOS2.REL: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (35980 -> 55036 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/EUROCUP.TMD: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5913 -> 10944 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/EUROCWC.TMD: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (11655 -> 21888 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/POOLPLYR.DAT: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (27025 -> 52200 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.000: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5270 -> 11630 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.001: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4208 -> 8210 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.002: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7888 -> 15050 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.003: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5488 -> 12314 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.004: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5707 -> 12314 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.005: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5055 -> 10946 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.006: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5062 -> 10946 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.007: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4065 -> 8894 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.008: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (29556 -> 63614 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.010: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4477 -> 9578 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.011: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3350 -> 8210 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.012: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4528 -> 9578 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.013: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (15781 -> 32150 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.014: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (13889 -> 28046 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.015: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6891 -> 14366 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.016: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5845 -> 12998 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.017: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3040 -> 6842 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.018: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4569 -> 10262 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.019: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4874 -> 10262 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.020: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (15076 -> 31466 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.021: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3207 -> 6842 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.022: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3842 -> 8210 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.023: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3641 -> 7526 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.024: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3858 -> 8210 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.025: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12348 -> 24626 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.026: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4977 -> 10946 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.027: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5223 -> 10946 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.028: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7097 -> 15050 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.029: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6456 -> 15050 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.030: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6426 -> 13682 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.031: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7188 -> 15734 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.032: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3293 -> 8210 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.033: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12281 -> 27362 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.034: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6532 -> 13682 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.035: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (14015 -> 30098 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.036: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4978 -> 10946 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.037: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5544 -> 10946 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.038: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6427 -> 12998 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.039: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5798 -> 12314 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.040: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6285 -> 15050 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.041: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5767 -> 12998 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.042: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4167 -> 10946 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.043: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12661 -> 28730 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.044: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (15408 -> 34886 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.045: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5049 -> 10946 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.046: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (8839 -> 22574 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.048: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5927 -> 12998 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.049: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5383 -> 11630 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.050: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3821 -> 8210 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.051: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12094 -> 31466 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.055: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (9007 -> 19154 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.060: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (11485 -> 27362 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.062: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (9591 -> 21206 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.064: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6958 -> 16418 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.065: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5346 -> 12314 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.066: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3223 -> 6842 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.067: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3361 -> 8210 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.069: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7173 -> 14366 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.071: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7208 -> 17786 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.072: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (15503 -> 32834 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.073: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2816 -> 6158 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.075: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (1915 -> 4106 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.076: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5558 -> 12314 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.077: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4866 -> 10946 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.078: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3849 -> 8210 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.079: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2413 -> 5474 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.080: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (17076 -> 32834 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.081: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (10790 -> 23258 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.082: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3477 -> 6842 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.083: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4338 -> 9578 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.084: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (10125 -> 21206 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/TEAM.085: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (1492 -> 2738 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/data/UEFACUP.TMD: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (11321 -> 21888 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/CHARSET.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2143 -> 40960 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/CJCBENCH.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4593 -> 40960 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/CJCBITS.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (972 -> 40960 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/CJCGRAFS.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2928 -> 40960 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/CJCTEAM1.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6338 -> 40960 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/CJCTEAM2.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6158 -> 40960 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/CJCTEAM3.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6251 -> 40960 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/CJCTEAMG.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4608 -> 40960 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/DISK2.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (8324 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/LOADER00.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2144 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/LOADER01.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (1252 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/LOADER1.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (9011 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/LOADER10.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2258 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/LOADER2.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3434 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/LOADER3.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2929 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/LOADER4.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2892 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/LOADER5.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3347 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/LOADER6.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6073 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/LOADER7.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6583 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/LOADER8.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6825 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/LOADER9.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2885 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/MENUBG.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7927 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/MENUBG2.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (18119 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/MENUS.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4067 -> 40960 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/MENUS2.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5267 -> 40960 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/SOCCER_S.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5771 -> 40960 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/SWCPICH1.MAP: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (13440 -> 45464 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/SWCPICH2.MAP: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12574 -> 39320 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/SWCPICH3.MAP: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (14378 -> 42904 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/SWCPICH4.MAP: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12186 -> 39704 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/grafs/SWCPICH5.MAP: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (13632 -> 44184 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/sound/HERO.INS: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (250368 -> 292984 bytes)
- runtime segment an address: 1078 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 66 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 28 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/SWOS2#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/SWOS2.REL#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer_v1.7_0840_hdf/SensibleWorldOfSoccer/swos11.dku: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- 1 never-loaded region(s), 335006 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `mask_input_byte` (runtime segment an address): Builds a mask from a global variable and a byte from a structure, clears one bit based on the structure byte, and optionally clears another bit if a flag in a table indexed by the doubled structure value is set, then returns a value combining the original words and the modified mask. Kind: CIAB PRB disk hardware check (disk hardware identity/track check). Pass path: The routine reads CIAB_PRB (disk hardware) and returns a byte mask in d0; callers test it via tst.b d1 / bclr.b #an address, d0, so a passing check is the branch taken when the disk hardware matches the expected value..

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `store_constant_via_pointer`: Stores the constant an address through the pointer in a3, then returns.
- `patch_version_specific_code`: Initializes a 349-word block in fast memory to zero, then compares a 32-bit value read from a global variable against several known magic constants and, when one matches, patches a large table of code pointers and instruction words in fast memory.
- `scan_words_check_marker`: Scans a table of words, decrementing a counter, and when it finds two consecutive words equal to 13308 and 16384 with a pointer at offset 4 equal to a global variable, it writes 24582 to the first word and returns, otherwise it advances and continues.
- `jump_global_function_pointer`: Loads a function pointer from a global variable and jumps to it, transferring control to the routine stored there.
- `masked_offset_stub`: Adds an offset from a5 to a1, masks the result to 24 bits, stores it back in a1, clears d1, and returns without altering any caller-visible state.
- `validate_and_copy_path`: Checks if a pointer points to one of the magic values data, graf, or soun; if not, it copies the null-terminated string from that pointer to a global buffer, then calls another routine.
- `copy_string_to_buffer`: Copies a null-terminated string from a source pointer to a global buffer, then calls another routine and returns a register value.
- `compare_and_shift_buffer`: Saves all registers, sets up two pointers from a global table, then compares bytes at those pointers in a nested loop, decrementing a counter and shifting buffer contents when a match is found before restoring registers and falling through.
- `restore_registers_and_branch`: Restisters registers a0 and a1 from the stack, computes a pointer 32 bytes past a1, then branches to a fixed address.
- `shift_buffer_and_copy`: Decrements a counter pointed to by a register, shifts bytes forward in a buffer by copying from offset 32 until a limit is reached, copies a null-terminated string to a global buffer, then calls two other routines before returning.
- `search_and_concatenate_path`: Saves all registers, initializes a counter and sets up pointers, then searches for matching bytes across a table of entries until a limit is reached, incrementing a counter and copying a string into a buffer before falling through.
- `advance_pointer_on_comma`: Advances a pointer by offset 32 in a2, increments a short register, and returns if that register equals 44, otherwise it jumps back to continue processing.
- `concatenate_path_strings`: Copies a source string into a buffer, then appends a second string (referenced as 'SAVE/') to it.
- `subtract_constant_to_d7`: Subtracts the constant an address from the value at a5, adds the result to d7, and falls through into the following routine.
- `call_routine_and_return`: Calls another routine via a pointer dereference and then returns zero.
- `call_library_function`: Calls a single unnamed routine and returns the value passed back in the D0 register.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `zero_fill_data_area`: Performs a no-op OR of zero into d0, so this range acts as a zero-filled data area referenced by address rather than executable code.
- `zero_memory_word`: Fills a 32-bit word in memory with zero.
- `tiny_ori_nop`: Small routine of 2 instructions that merges 0 into d0, then does nothing for one instruction.
- `call_offset_36_function`: Saves d1 and a0 through a2, loads a function pointer from a global variable, jumps to the routine at offset 36, restores the saved registers, and returns.
- `dereference_and_call_pointer`: The routine loads a pointer from a global variable and calls the routine located at that address.
- `call_offset_28_function`: Saves d1 and a0 through a2, loads a function pointer from a global variable, jumps to the routine at offset 28, restores the saved registers, and returns.
- `call_offset_88_function`: Saves d1 and a0 through a2, loads a function pointer from a global variable, jumps to the routine at offset 88, restores the saved registers, and returns.
- `call_offset_12_function`: Saves d0, d1 and a0 through a2, loads a function pointer from a global variable, jumps to the routine at offset 12, restores the saved registers, and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_tables_and_call_routines`: Initializes a global variable to its own address and stores a return address, then fills several tables with the repeating patterns 21829, 48059, 61677 and 61680, and finally calls a series of other routines.
- `fill_tables_and_jump_to_routine`: Fills a table with the value 21829 and another with 48059, then stores the values an address, the contents of registers d5, d6 and d7 into a global variable, and writes a routine address into the vector table before jumping to that routine.
- `clear_registers_and_fill_tables`: Disables interrupts and clears a hardware register, then writes the value 61680 to three consecutive 16-bit tables starting at the addresses in a0, a1 and a2 by looping four times, and returns.
- `dispatch_function_pointer`: Saves register a6, computes a dispatch address from a register shifted left by one, calls the function located there, sets a global variable to 65535, and restores a6 before returning.
- `branch_into_routine`: Unconditionally branches into the middle of another routine, transferring control without executing any instructions of its own.
- `jump_to_routine`: Jumps to another routine and does not return here.
- `return_zero_via_call`: Jumps to another routine, which returns zero, and then returns that zero value.
- `return_zero`: Loads zero into the result register and returns.
- `store_pointers_and_clear_memory`: Stores the values in registers a0 through a4 into a table of five consecutive pointers, then stores two more pointer values into a table at offsets 19712 and 20 bytes away, and calls another routine with the first pointer and the value 3.
- `modify_memory_and_call_routines`: ORs four consecutive bytes pointed to by a0 with the constants 144, 149, 154 and 159, inverts the byte at offset 2 times d3 from a1 and the byte at offset 2 times d3 from a3, then calls two other routines.
- `jump_then_return_zero`: Jumps to a fixed address, loads zero into d0, and returns.
- `tiny_moveq_tst_rts`: Small routine of 3 instructions that loads 0 into d0, then tests d0, then returns.
- `load_grafs_resource`: References a text string for a grafs loader resource and calls an external function twice.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `save_registers_validate_drive`: Copies the values of registers d1, a0, a1 and a2 into registers d7, a4, a5 and a6, then calls another routine and returns if it reports failure.
- `dispatch_compare_d6_one`: Compares the byte in register d6 with the decimal value 1 and returns when they differ, otherwise it calls another routine and returns.
- `dispatch_compare_d6_two`: Compares the byte in register d6 with the decimal value 2 and returns when they differ, otherwise it calls another routine and returns.
- `dispatch_compare_d6_three`: Compares the byte in register d6 with the decimal value 3 and returns when they differ, otherwise it calls another routine, pushes the value of register d1 onto the stack, and returns.
- `dispatch_compare_d6_four`: Compares the byte in register d6 with the decimal value 4 and returns when they differ, otherwise it calls another routine and returns.
- `dispatch_compare_d6_five`: Compares the byte in register d6 with the decimal value 5 and returns when they differ, otherwise it calls another routine, saves registers d0 and d1 onto the stack, and returns.
- `dispatch_compare_d6_seven`: Compares the byte in register d6 with the decimal value 7 and returns without doing anything when they differ, otherwise it calls another routine and returns.
- `compare_and_call_routine_2`: Compares register d6 with the value 7 and branches away when they differ.
- `compare_and_call_routine`: Compares register d6 with the value eight and branches away when they differ, otherwise pushes d0 onto the stack, calls a routine, and branches to another location.
- `init_disk_transfer`: Pushes the input parameter onto the stack, loads zero into registers d1 and d2 and the value 32768 into d3, then calls a routine and returns the parameter combined with the result in d0.
- `check_table_pointer`: Loads a pointer to a table, reads the long at its base, and returns immediately if that value is zero.
- `store_result_into_structure`: Calls a routine and, if it reports success, stores a global word into a structure at offset 0, clears the words at offsets 2 and 6, writes the value -1 to offset 4, copies a5 to offset 10 and copies a value from offset 324 of another structure to offset 14, then returns that value; otherwise it returns.
- `clear_value_at_offset_ten`: Calls a routine using the value in a4 to compute a table index, then stores zero at offset 10 of the structure in a4.
- `store_min_value_to_offset`: Calls a routine, compares the value at offset 14 of a4 with the returned value, stores the smaller of the two at offset 6 of a4, and returns zero.
- `update_structure_counters_2`: Computes the difference between two 32-bit counters in the structure and, if it is negative, records the new minimum, stores the first counter into a global variable, and calls the price index routine.
- `update_structure_counters`: Increments a 16-bit counter at offset 2 of the structure in a4 and calls a routine, returning to a loop if that counter overflows.
- `mask_low_bits`: Reads an input value from the D0 register and returns the bitwise AND of that value with a constant mask, effectively keeping only the lowest ten bits.
- `compute_price_index_from_a4`: Reads a 16-bit value derived from the A4 register, multiplies it by 18, and uses the result as an index into a table.
- `check_and_prepare_structure`: Calls a routine and returns if it reports failure, then loads a value from offset 228 of a structure into a register, copies a6 to a1, clears the long at offset 508 of that structure and loads the address of the value at offset 312 into a2, then returns.
- `process_and_loop_values`: Calls a routine and returns if it reports failure, then saves a value and clears the long at the top of the stack, computes an even address from a register and stores it, shifts a value right by nine bits and returns early when the result is nonzero, and returns early when a value is greater than 488, otherwise it jumps back to an earlier instruction.
- `loop_check_counter_decrement`: Calls a routine and returns if it reports failure, then increments a counter and compares it against another value, returning early when they match.
- `check_pointer_bits_then_rotate`: Adds 24 to a0 and clears its lowest bit, returning if set; then adds 5 to a5 and clears its lowest bit, returning if set; otherwise rotates d0 right by two bits and falls through.
- `copy_word_then_shift`: Copies a 32-bit word from the address in a0 to the address in a5, incrementing both pointers, looping while d0 counts down.
- `copy_memory_block`: Copies a block of memory from the address in a0 to the address in a5 one byte at a time, using a dbra loop counter, and returns.
- `compare_fields_return_id`: Reads two 32-bit fields from a structure and compares them, and if they match and a flag field is non-zero, calls another routine to obtain an identifier and clears that flag, otherwise it increments a byte field and returns zero.
- `validate_and_copy_string_block`: Verifies a value against the constants 205 and 221, then stores 2 and a second value into a table, clears two structure counters and copies a length-prefixed byte string into a destination buffer, updating the counters as it goes.
- `store_counter_and_sum_array`: Reads a global counter and stores its value into a table indexed by another global, then clears a flag at offset 20 of a structure and calls a routine that sums an array of 128 words, storing the negated result back at offset 20.
- `store_sum_and_branch`: Tests a global flag and reads several global words, storing a computed value into a table indexed by another global, then sets up a buffer and calls multiple routines, returning through a branch when a condition is met.
- `return_constant`: Returns the constant 221 in d0 to the caller without any other effect.
- `call_routines_indexed_loop`: Calls a routine and returns on failure, then repeatedly calls another routine while decrementing a counter indexed from a structure.
- `set_flag_in_bitfield`: Computes an index from the high and low parts of an input value, selects a 32-bit word from a table pointed to by A6, and sets a single bit within that word corresponding to the low bits of the index.
- `decrement_and_loop_until_match`: Copies d6 into d0, decrements it, then repeatedly calls a routine with the updated value until it equals 1759, at which point it stores the value back in d6 and returns.
- `wait_for_specific_value`: Calls a routine once and returns if it is zero, then enters a loop that increments two registers and repeatedly calls another routine until a short register equals 1760, after which it returns.
- `combine_registers_into_return_value`: Receives two 32-bit values in registers and returns a 64-bit value formed by concatenating the first value as the high part and the second value as the low part.
- `clear_bit_from_table`: Clears a single bit in a 32-bit word table indexed from a 32-bit value in D0.
- `call_sum_with_default`: Loads a value from a global variable into a register, then calls a routine that sums an array and returns either that sum or a default value, and calls another routine that loads a constant, returning afterward.
- `copy_name_bytes`: Copies a null-terminated byte string from a source pointer to a destination buffer located at an offset from a base pointer, advancing both pointers each iteration.
- `copy_blocks_from_table`: Calls another routine and returns early if it fails, then iterates over a table of pointers, copying a fixed-size block from a global table into a buffer and advancing through entries until a counter reaches its limit, returning the accumulated value.
- `clear_zero_table`: Clears a block of memory by writing zeros to consecutive 32-bit words.
- `initialize_transfer_block`: Writes the constants 2, 72, -1 and 881 to offsets 0, 12, 312 and 316 of the structure pointed to by a0, then stores 1 at offset 508.
- `tiny_movea`: Small routine of 1 instruction that copies a6 to a0.
- `index_table_call_routine`: Saves a pointer, calls a routine, and stores its result into a global word.
- `check_byte_and_store`: Compares a register against a constant and branches away if they match, then tests a byte at a pointer and branches away if it is nonzero, otherwise stores a constant into a register before falling through.
- `save_buffer_if_byte_set`: Tests the byte at a4 and returns without changes if nonzero; otherwise writes the word at offset 500 of a0 to a global buffer, restores a4, clears d0, and returns.
- `validate_drive_identifier`: Verifies that a two-character drive identifier begins with the letters D and F, then checks that the following byte is a decimal digit followed by a colon, storing the digit value at offset 1 of a table if valid.
- `tiny_move`: Small routine of 1 instruction that copies (a0)+ to d0.
- `convert_lowercase_to_upper`: Tests whether the byte in d0 falls within the lowercase ASCII range and converts it to uppercase, otherwise it returns without changing anything.
- `copy_path_segment_to_buffer`: Scans a null-terminated string from a4 until it reaches a null terminator or a slash, copies the preceding characters into a buffer, terminates that buffer with a zero byte, and computes a return value from the copied data.
- `check_flag_and_store_value`: Calls another routine and stores its result, then tests a flag and two fields of a pointer passed in a register, overwriting a global word with a value read via that pointer and returning zero when they match, otherwise returning the stored result.
- `call_sum_preserve_registers`: Saves two registers on the stack, calls a routine that sums an array and returns either that sum or a default value, restores the registers, and branches away if the returned value is nonzero.
- `return_sum_or_default`: Calls a routine that sums an array, and if the result is nonzero it returns that value, otherwise it returns 405.
- `sum_negate_array_words`: Accumulates 128 consecutive 32-bit words pointed to by a0 into a register, then returns the negated sum in the high 32 bits while passing the low 16 bits unchanged.
- `store_array_sum_at_offset`: Clears the value at offset 20 of a0, calls a routine that sums an array, and stores the result back at offset 20 of a0.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d2.
- `call_and_check_result`: Pushes register d3 onto the stack, loads the value one into d3, calls a routine, restores d3 from the stack, and returns the result in d0.
- `load_one_into_d2`: Loads the constant 1 into register d2 and returns, performing no other work.
- `tiny_moveq_2`: Small routine of 1 instruction that loads 0 into d3.
- `save_and_call_divider`: Saves register a1, loads a1 with the value of a6, reads a 16-bit value from the global variable at offset 2262, calls a routine, restores a1, and returns.
- `process_dma_transfer`: Saves its registers and compares d3 to three, calling a routine and returning when that matches.
- `loop_accumulate_cia_bits`: Stores a constant into a stack slot, then loops a fixed number of times calling two routines, accumulating a bit into a register when another routine returns a specific value, decrementing the slot each iteration and returning the accumulated value.
- `loop_toggle_cia_bit`: Initializes a counter to 31, then loops thirty times calling a routine that toggles a CIA port bit, and finally returns a value formed from the last result and the carry flag.
- `toggle_cia_port_bit`: Builds a single-bit mask from a bit index in d1, writes it to the CIA A port register, reads the CIA B port register, then writes the same bit set in the CIA A port register.
- `poll_and_configure_dma_retry`: Repeatedly polls a hardware register bit and a global flag while decrementing a counter, clearing three 16-bit fields in the passed structure and calling helper routines to check conditions and configure DMA, returning early when a flag is clear or the counter expires.
- `configure_dma_transfer_loop`: Initializes a counter and pointer, then loops a fixed number of times writing a constant word to a memory address while calling helper routines and polling a status flag, returning a result.
- `loop_write_memory_block`: Initializes a counter and pointer, then loops a fixed number of times writing a constant word to a memory address and calling other routines.
- `validate_input_with_retry`: Repeatedly calls helper routines to check a flag and decrement a counter, returning early if the flag is clear or the counter expires, otherwise storing values into a structure and returning a status code.
- `poll_bit_range_check`: Computes a pointer from a structure field and a scaled offset, then calls a routine and polls a hardware register bit, returning early when a flag is set.
- `scale_offset_by_512`: Subtracts a 16-bit value from the structure in a6 from d3, multiplies the result by 512, and leaves the product in d1 before returning.
- `set_flag_and_increment_counter`: Sets a bit derived from d3 in a 16-bit word at offset -4 of the structure in a6, then increments the 16-bit counter at offset -6 of the same structure.
- `clear_table_entries`: Advances a pointer by 336 bytes, writes zero to that location, then repeats ten times total, advancing by the same stride each pass to clear successive entries.
- `extract_high_byte_pair`: Calls a routine to read two consecutive 32-bit words and reassembles the result so the low 16 bits come from the returned value and the high 8 bits come from its original high byte, then stores the combined value in d3 and d2 before returning.
- `interleave_word_bits`: Reads two consecutive 32-bit words from a pointer, masks each with an alternating-bit pattern, doubles the first, and returns the second ORed with the doubled first.
- `apply_interleave_xor_mask`: Calls a routine to interleave word bits and pushes the 32-bit result on the stack, then calls a second routine that performs an XOR sum and mask operation on the value at offset 48 of the structure in a5, and compares the two results before returning.
- `load_address_and_constant`: Loads the address at offset 8 of the structure in a5 into a0 and loads the constant 40 into d1, then returns.
- `compute_xor_checksum`: XORs each 32-bit word from the block pointed to by a0 into an accumulator, looping once for each of the count in d1 shifted right by two, then returns the result masked to keep alternating bits.
- `fill_memory_pattern`: Loads a base address from a structure and loads a 32-bit pattern into all data registers, then selects a block length from a flag in the structure.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `interleave_word_bits_2`: Processes 128 pairs of 32-bit words from two input pointers, masking each with an alternating-bit pattern, shifting one left by one, and ORing them together before storing the interleaved result to a third pointer.
- `loop_clear_flag_bitwise_mix`: Computes a loop count from the first argument by shifting it right by two and subtracting one, then repeatedly calls routines to clear a flag byte and mix or store a word, walking backward and forward through a table, and finally returns a combined value of the two arguments.
- `shift_and_call_routine`: Pushes a register onto the stack, shifts a value right by one bit, then calls another routine twice, restoring the register from the stack before falling through to the following code.
- `clear_flag_byte`: Reads a byte from a pointer and inspects bit 0 of the preceding byte; if it is clear it returns early when bit 6 of the target is set, otherwise it sets bit 7, and if the flag bit is set it clears bit 7 instead, then writes the modified byte back.
- `bitwise_mix_store_word`: The routine mixes the input word with a transformed mask derived from the low bits of the input, optionally clears the sign bit based on a flag byte before the output pointer, and stores the resulting word through the pointer.
- `configure_dma_registers`: Writes fixed values to the DMA length, DMA pointer, and sync custom registers, sets a DMA control word, then shifts a passed value right by one, sets its high bit, and writes it to the audio control register.
- `poll_flag_and_set_timer`: Calls a routine to read a flag, then loops while the flag is not set, writing two values to global variables and returning zero when the flag is found, otherwise returning a nonzero value.
- `write_audio_control_check`: Writes the constant four thousand to the audio control register, inspects a byte at offset 30 of the structure in a6, and returns when that byte is negative, otherwise it branches to continue execution.
- `tiny_moveq_3`: Small routine of 1 instruction that loads 255 into d1.
- `set_bit_in_global_flag`: Reads a 16-bit value from a structure, adds three, and uses the low bits as a bit index to clear then set the corresponding bit in a CIAB port byte, writing the result back before returning.
- `set_bit_and_wait_for_flag`: Writes a constant byte to a port register, clears a bit in a register, and calls a routine to set a timer, then loops while a global flag is set and repeats until a returned value matches.
- `update_table_entry`: Reads a word from a global table using an index, calls a routine if the value is negative, computes the absolute difference against a register, and writes the result back to the table before returning zero.
- `wait_for_input_flag_clear`: Polls a global variable flag in a loop, decrementing a counter until the flag is cleared or the counter expires, then clears a byte in a table indexed by a register value and returns a status code.
- `mask_byte_and_wait_for_flag`: Calls a routine to read a byte, masks specific bits based on the result, writes the modified byte to a port register, then loops while a global flag is set and repeats until a returned value matches.
- `store_masked_input_byte`: Calls a routine to mask an input byte and stores the resulting byte into the CIAB parallel register B before returning.
- `mask_input_byte`: Builds a mask from a global variable and a byte from a structure, clears one bit based on the structure byte, and optionally clears another bit if a flag in a table indexed by the doubled structure value is set, then returns a value combining the original words and the modified mask.
- `wait_for_flag_and_result`: Calls a routine to set a CIAB timer and obtain a value, then spins while the least-significant bit of a global variable is set, and repeats the whole process until the obtained value equals one, otherwise it returns zero.
- `check_flag_and_counter`: Inspects the least-significant bit of a byte in the CIAB port register B and returns when it is set, then decrements a counter and returns when that counter reaches zero.
- `set_ciab_timer`: Writes the constant eight to the CIAB control register, the constant two hundred and four to the timer high register, and the constant two to the timer low register, then returns.
- `trigger_version_patch`: Loads five addresses into registers and a constant into d7, then calls a routine that detects the game version and patches code based on magic values found in memory before returning.
- `verify_and_copy_version_block`: Verifies a magic value in a global structure and, when it matches, copies the following block of bytes into a fast-memory buffer.
- `handle_version_check`: Saves its registers and returns a value when a global word equals four.
- `copy_and_verify_block`: Copies 2225 bytes from a source table into a block of fast memory, then verifies a magic value stored in a global variable and returns without restoring the processor status when the value matches.
- `store_constant_value`: Stores a fixed 32-bit constant through a pointer, then returns; the remaining instructions belong to the following routine.
- `call_version_patch_routine`: Saves its registers, sets a constant in d0 and another in d7, then calls a routine that detects the game version and patches code based on magic values before restoring the registers and returning.
- `patch_version_code`: Saves its registers, sets a different constant in d0 and another in d7, then calls a routine that detects the game version and patches code based on magic values before restoring the registers and returning.
- `clear_global_counters`: Clears the 32-bit value at offset 1712 of the global structure and the 32-bit value at offset 1716 of the same structure, then returns.
- `allocate_memory_block`: Aligns the requested size in d0 up to the next even number and walks a linked list of memory blocks looking for one whose free space fits the request.
- `lookup_string_entry`: Walks a linked list of entries and compares the string pointed to by a0 against the stored string of each entry byte by byte.
- `copy_memory_block_3`: Calls a routine to obtain a combined size and two pointers, returns early when the size is the sentinel value, then copies a block of memory word by word when both pointers are aligned and the size fits, otherwise it copies the block byte by byte, and finally restores the registers before returning.
- `allocate_and_copy_block`: Saves its registers and calls a routine, returning early when it reports failure.
- `return_one_value`: Loads the constant 1 into d0, restores the saved registers, and returns.
- `return_max_byte`: Loads the constant 255 into d0, restores the saved registers, and returns.
- `initialize_string_block`: Calls a routine to allocate a memory block, copies a fixed number of bytes into the structure at offset 48, then copies additional bytes into the structure at offset 14 until a source byte is zero, and finally restores the registers before returning.
- `jump_to_decode_routine`: Contains a single unconditional branch to another routine and does not execute any instructions of its own.
- `check_input_result`: Clears d0 and calls a routine, returning when the result is zero, negative one, or negative two.
- `decode_compressed_chunk`: Verifies a data header signature, then expands a Huffman-coded compressed stream by copying raw byte blocks and reconstructing literal runs into a destination buffer.
- `parse_bit_table`: Scans a table of words for a matching condition, then reads a byte at offset 60 and decrements a counter, calling a routine when the subtraction is non-negative, and reads a byte at offset 61 to compute a bit mask based on register contents, setting a bit in the result before returning.
- `mask_and_conditional_call`: Masks a word in d0 with the word in d6, then, if the byte in d7 is less than the byte in d1, calls another routine and uses its returned word as the result, otherwise it returns the masked word.
- `shift_and_combine_bytes`: Combines d1 into d6, rotates d6 to extract a byte, and adjusts d1 by the shift amount, leaving a computed byte in d1 before returning.
- `accumulate_32bit_from_bytes`: Reads four consecutive bytes from the pointer in a0 and concatenates them into a 32-bit value in d0, returning that value.
- `decode_huffman_data`: Processes a sequence of words using a stack-based algorithm.
- `handle_keyboard_interrupt`: Checks a bit in the CIA control register and, if set, waits for a flag in the keyboard serial register, then disables interrupts and spins a loop counting down from 300.
- `no_effect_instruction`: Contains a single ori.b #0, d0 instruction that has no observable effect, so the routine does nothing and is only referenced by address.
- `set_d1_to_152`: Performs three no-op ori.b #0, d0 instructions followed by ori.b #152, d1, which loads the constant 152 into d1, so the routine only sets d1 and is referenced by address.
- `handle_interrupt_state`: Tests a global flag and, if set, compares a hardware register against a stored value, calling routines and updating the flag and a secondary counter.
- `initialize_dma_channels`: Initializes several global tables by filling them with constants and clearing buffers, then configures DMA channels by writing control registers and waiting for the vertical blank flag to clear before returning.
- `calculate_table_offsets`: Copies four 32-bit values from a global table into a series of memory locations, then computes a pair of offset values from a hardware register and a global flag, storing them in a loop of three iterations before writing a final flag value.
- `swap_memory_blocks`: Checks a global word at offset 2b47a and, depending on whether it equals 4836, takes one of two branches that each store four pointer constants and copy a source pointer into them.
- `wait_and_clear_interrupts`: Clears the CIA interrupt request register, then polls the vertical position register until the screen row reaches at least ten.
- `restore_registers_and_return`: Restores the CPU registers from a saved context on the stack, then executes a return from exception.
- `process_input_and_store_flags`: Reads the fire buttons and joystick/mouse movement via CIA port A and the mouse data register, then stores the resulting boolean flags into a global array.
- `decrement_loop_until_zero`: Pops a longword from the stack into a destination pointer, then multiplies a register value by ten and decrements it in a loop until it reaches zero.
- `read_joystick_one`: Reads the fire buttons from CIA port A and the movement from joystick port one, then converts each signal into a boolean flag stored in a global variable.
- `read_and_process_mouse_buttons`: Reads the CIA port A register and the mouse data register to sample button and movement bits, then derives eight debounced button flags from them.
- `synchronize_two_indices`: The routine adjusts a global variable toward a first short value in D1 and then toward a second short value in D2 by stepping the variable by two each iteration, wrapping it at the limits implied by the code.
- `update_palette_colors_2`: Branches away if a global flag is set, otherwise indexes a lookup table with a shifted value and loops four times masking palette data into two memory blocks, advancing pointers each iteration.
- `update_palette_colors`: Builds bit patterns in registers d0 through d4, indexes a lookup table with a shifted value, and loops four times masking palette data into two global tables while forcing low nibble values below two up to two.
- `set_register_bit_patterns`: Sets specific bit patterns into registers d0 through d4 using immediate OR operations, then falls through to the following routine.
- `decrement_palette_timer`: Decrements a global 16-bit counter by a value from a table, resetting it to 128 when it is negative, and sets it to a sentinel value with a flag at offset 4 of a structure at offset 2 while the counter is below zero.
- `increment_palette_timer`: Increments a global 16-bit counter by a value from a table, resetting it to zero when it is negative, and sets it to a sentinel value with a flag at offset 4 of a structure at offset 2 once it exceeds 128.
- `wait_for_flag_clear`: Checks a global variable against the value 999, and if it does not match, waits for another flag to become zero before calling another routine.
- `wait_for_flag_change`: Waits for a global flag to become nonzero, then sets another variable to all ones if needed and waits for the flag to clear before returning.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `configure_interrupt_state`: Copies a routine address into a global variable, stores another routine address in a second global variable, writes the value 51236 to the interrupt enable register to enable interrupts, and returns.
- `handle_dma_interrupt`: Writes two values to the DMA control register, then checks a global flag and, if it is zero, clears it and jumps to another routine before returning from the interrupt.
- `switch_to_supervisor_mode_2`: Writes a constant to a custom register and stores a value into a global variable, then calls a routine that switches to supervisor mode and returns.
- `switch_to_supervisor_mode`: Pushes the registers onto the stack, swaps in a new stack pointer, sets a flag, switches to supervisor mode, and jumps to another routine that returns to the caller without doing anything.
- `return_stack_value`: Restores the CPU registers from the top of the stack, executes a return from exception, then saves the current registers back onto the stack, swaps the stack pointer, restores the registers again, and returns from exception.
- `jump_to_empty_stub`: Jumps to another routine, which returns to the caller immediately without doing anything.
- `pack_nibbles_into_words`: Reads three bytes from a source buffer, splits each into nibbles, and accumulates the high nibble into a running total in a register.
- `configure_blitter_dma`: Waits for a global flag bit to clear, then writes setup values to Blitter and DMA control registers and clears several Blitter size and offset registers.
- `spin_on_global_flag`: Sets a global variable to one, tests that same variable, and branches back to re-test it while it remains nonzero.
- `process_string_sprites`: Reads bytes from a string until a null terminator is found, accumulating a running total while translating each byte through a lookup table and adding table values, and calls a sprite setup routine when a counter exceeds a threshold.
- `call_and_adjust_register`: Saves registers, calls a routine, restores the registers, subtracts a value from another register, increments it, and branches to another routine.
- `call_and_shift_register`: Saves registers, calls a routine, restores the registers, increments a register, shifts it right by one bit, subtracts it from another register, and branches to another routine.
- `setup_sprite_blitter_2`: Looks up a sprite in a table using the index in d0, then sets up a copper list that draws it by configuring the blitter registers and waiting for the vertical blank.
- `no_op_placeholder`: Contains a single instruction that performs a no-op bitwise OR on a register, so the routine does nothing and is only referenced by address as a data placeholder.
- `setup_sprite_blitter`: Looks up a sprite in a table using an index in d0, then sets up a copper list that draws it by configuring the blitter registers and waiting for the beam position.
- `store_zero_byte`: Writes a zero byte to the byte addressed by the register pair, leaving the value unchanged, and then falls through into the following routine.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `tiny_ori_rts`: Small routine of 2 instructions that merges 0 into d0, then returns.
- `zero_fill_data_area_2`: Contains only zero-fill words that other code references by address, so it does nothing and returns immediately.
- `initialize_sound_state`: Clears several global words to zero, sets a global word at offset 3545 to 65535, and returns the original value held in register d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_buffers_run_routines`: Calls a series of routines, clears several global words, sets a flag, and initializes a buffer.
- `modify_buffer_with_constants`: Clears a bit in a global variable based on a register value, then ORs a series of fixed byte constants into consecutive bytes of a buffer pointed to by a register.
- `modify_buffer_tail_and_check`: Adds a register value to a short at the start of a buffer, then ORs a sequence of fixed byte constants into the following bytes.
- `check_flag_and_call_pointer`: Tests a global variable and returns immediately if it is zero, otherwise loads a pointer from a table in a6, returns if that pointer is negative one, and calls the routine at that address before jumping back.
- `loop_on_error_call`: Calls another routine and, if the result is negative one, enters a loop that repeatedly calls a second routine until a boolean flag becomes false.
- `process_table_entry_if_flag_set`: Checks a global flag and, if it is clear, returns without doing anything.
- `loop_on_error_call_2`: Calls a routine and, if the result is -1, enters a loop that repeatedly calls another routine until a boolean flag becomes false, otherwise it returns.
- `process_input_sequence`: Initializes the input buffer and a status flag, then runs a sequence of input-processing routines while checking a global flag to decide whether to return early.
- `call_sequence_check_zero`: Saves d7, then calls four routines in sequence, restoring d7 before each call, and after the fourth call branches to another routine, returning if that routine sets the zero flag.
- `call_sequence_clear_return`: Saves d7, then calls five routines in sequence, restoring d7 before each call, and after the fourth call branches to another routine, clearing the return register to zero and returning if that routine sets the zero flag.
- `call_sequence_return_zero`: Saves d7, then calls four routines in sequence, restoring d7 before each call, and after the fourth call branches to another routine, returning zero if that routine sets the zero flag and otherwise returning its result.
- `call_and_check_error`: Calls a routine passing a register value, then checks a global variable; if the variable is zero, it enters an infinite loop, otherwise it returns the result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_memory_block_2`: Copies a block of bytes from one memory location to another, using the length in a global variable, but switches to a special path when the destination starts with the marker value an address.
- `configure_blitter_dma_2`: Waits for a global flag bit to clear, then writes configuration values to a block of memory representing Blitter registers, including source/destination pointers, sizes, and control words.
- `convert_integer_to_string`: Takes a 16-bit integer value and converts it into a null-terminated ASCII string representation.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_global_pointers`: Sets four specific global variables to fixed constant values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.

### SWOS2#unpacked (raw code file, load address unknown)

- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### SWOS2.REL#unpacked (raw code file, load address unknown)

- `apply_byte_or_sequence`: Performs a long series of byte-wise logical OR operations on register d0 using immediate constants, with no branches or calls present in the listed instructions.
