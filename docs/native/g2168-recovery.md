# Game 2168 (black crypt (1992)(electronic arts)[cr trsi](disk 1 of 3)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02168). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (916816 top-level bytes in 131 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 119 of 119 units identical (bytes and relocations), 182 raw-word lines |
| Functions named and described | pass, 678 of 678 named (0 author symbols), 678 of 678 described (59 by rule, 266 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 958 of 958 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 18280 of 18281 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 654708 |
| code | 58276 |
| data | 194897 |
| padding | 6887 |
| unused | 2048 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Run | 2 | 458 | 12 |
| Stack | 2 | 117 | 2 |
| avail | 11 | 376 | 12 |
| runback | 3 | 709 | 39 |
| endcli | 2 | 182 | 3 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:TRSI! | 1 | 116 | 3 |
| Port-Handler | 1 | 306 | 1 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:InstallCrypt | 3 | 3235 | 57 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:BlackCrypt | 3 | 2384 | 104 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:bcdfp | 3 | 6602 | 223 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:bcdfq | 2 | 1120 | 70 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:bcdft | 5 | 160 | 15 |
| Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 180 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 113 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 37 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 191 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 28 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 28 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 26 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 27 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 26 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 37 | 2 |

## Cases handled

- runtime segment an address: 25 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 64 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 10 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 21 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 115 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 11 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 11 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 10 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 12 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 12 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 13 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- black crypt (1992)(electronic arts)[cr trsi](disk 1 of 3)[a]/Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:TRSI!: CrunchMania Addr (xfd) stream found by ancient scan (13088 -> 32446 bytes) but only 0 of 7 sampled windows are in reference RAM: not used
- black crypt (1992)(electronic arts)[cr trsi](disk 1 of 3)[a]/Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:bcdft: CrunchMania Normal (xfd) stream found by ancient scan (83200 -> 180004 bytes) but only 0 of 86 sampled windows are in reference RAM: not used
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- black crypt (1992)(electronic arts)[cr trsi](disk 1 of 3)[a]/Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:TRSI!: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- black crypt (1992)(electronic arts)[cr trsi](disk 1 of 3)[a]/Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:bcdft: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_block_then_call_pointer`: When the destination pointer is non-null, it copies sixteen 32-bit words from a fixed block in the address register area into that destination, advancing both pointers each step.
- `initialize_entity_structure`: Initializes a large structure by zeroing various offsets, fetching values from tables into specific offsets, and performing a loop to copy a sequence of bytes from one memory location to another.
- `update_entity_attribute_from_table_2`: Fetches a byte from a table using a value from offset 0 of a structure, stores it at offset 20 of that structure, and calculates a new value for offset 24.
- `update_entity_attribute_from_table`: Calculates a target offset in a structure by adding a value from offset 12 to a constant and stores it at offset 16, then calculates a source offset from offset 20 and stores it at offset 80.
- `update_entity_state_and_call_handler`: Updates a structure in a0 by setting a byte at an offset calculated from a1, then copies several longwords from a structure in a1 to a structure in a0.
- `branch_to_offset`: Adds the value at offset 84 of a1 to 1 and then branches to a different location.
- `test_longword_and_branch`: Tests a longword at offset 40 of the structure in a1 and branches to another routine if the value is non-zero, otherwise it branches to a third routine.
- `modify_bitmask_in_structure`: Performs a bitwise AND between a longword in d1 and a longword at offset 48 of a1, and if the result is non-zero, performs a bitwise OR on the value at offset 48 of a1 with a mask.
- `update_entity_pointer_and_call_subroutine`: Copies a longword from offset 64 to offset 108 of a structure, prepares arguments for a subroutine call using values from offset 48, 52, and 132 of two different structures, and stores the returned longword into offset 28 of the first structure before conditionally branching.
- `reset_entity_state_and_update_pointers`: Updates a pointer in a table using values from a structure, calls two routines, and clears several fields in a structure at a calculated offset in a0.
- `update_structure_and_decrement_counters`: Calls several routines using values from a structure in a2, stores a longword from d1 into a global variable, and conditionally decrements a value at an offset in a table pointed to by a0.

### Stack

- `find_max_and_prepare_buffer_2`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_stack_words`: Copies a fixed number of words from a source pointer to a destination pointer on the stack.

### avail

- `setup_task_and_open_libraries`: Allocates a record and returns error code 20 if that fails.
- `process_message_reply`: Handles message processing by updating message fields, calling various functions based on flags, and returning a value from the message structure.
- `sum_memory_region_sizes`: Iterates through a linked list of memory regions and sums the size of any region whose attribute flags match a provided mask.
- `print_memory_availability_table`: Parses a command string to determine if memory statistics should be filtered by chip, fast, or total types, then calculates and prints a table showing available, in-use, and maximum memory block sizes for the selected memory configuration.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `forward_args_with_stack_slot`: Passes its two arguments on to another routine together with the address of a fixed stack slot, then returns.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### runback

- `call_one_unnamed_routine`: The routine is a single instruction that passes control to one other routine.
- `run_background_task`: The routine parses a command line for an optional load delay and a program name with parameters, then starts the named program as a background task, handling errors such as missing command, missing file, directory instead of file, out of memory and usage errors, and prints status messages.
- `clear_table_and_recurse`: Clears a block of forty words starting at an offset from a base pointer, stores a local stack address and a global variable into subsequent fields, and calls another routine.
- `call_flagged_hook_then_continue`: Checks a global pointer and, when it is non-zero, calls the function it points to.
- `process_dialogue_text`: Parses a string to extract individual words and updates global state fields.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `run_back_loop_handler`: Performs a short loop calling another routine ten times and then inspects several global variables.
- `append_string_to_destination`: Appends a zero-terminated source string to the end of a destination string, copying at most 32767 bytes.
- `append_string_with_count`: Walks to the end of the destination string and appends bytes from the source, stopping at the source terminator or when the count argument runs out, then null-terminates the destination.
- `copy_string_to_destination`: Copies a zero-terminated source string into a destination buffer, including the terminator, and returns the destination pointer.
- `measure_zero_terminated_string`: Walks a zero-terminated byte string and returns how many bytes come before the terminator.
- `copy_string_with_zero_padding`: Copies bytes from a source string into a destination for a count taken from a 32-bit argument, stopping at the first zero byte.
- `release_indexed_slot_entry`: Looks up an entry in a table of twenty slots by index; an out-of-range index or empty slot stores error code 3 and returns minus one.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_function_102`: Loads a library base from a field in the global block and calls the routine 102 bytes below that base, then returns.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_function_84`: Loads a library base from a field in the global block and calls the routine 84 bytes below that base, then returns.
- `call_library_function_30`: Loads a library base from a field in the global block and calls the routine 30 bytes below that base, then returns.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_function_60`: Loads a library base from a field in the global block and calls the routine 60 bytes below that base, then returns.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_function_90`: Loads a library base from a field in the global block and calls the routine 90 bytes below that base, then returns.
- `call_library_function_48`: Loads a library base from a field in the global block and calls the routine 48 bytes below that base, then returns.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_function_198`: Loads a library base from a field in the global block and calls the routine 198 bytes below that base, then returns.
- `call_indirect_function`: Loads a function pointer from a global table and executes it.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_function_132`: Loads a library base from a field in the global block and calls the routine 132 bytes below that base, then returns.
- `call_library_function_210`: Loads a library base from a field in the global block and calls the routine 210 bytes below that base, then returns.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.

### endcli

- `find_max_and_prepare_buffer_3`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_fixed_size_data`: Copies a fixed number of longwords from a source pointer to a destination pointer.
- `update_state_and_jump`: Modifies a value in a2, updates a pointer in a1, calls a routine in a5 multiple times with various parameters from a2 and a1, decrements a value in a table at a0, and finally performs an indirect jump using a6.

### Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:TRSI!

- `call_three_routines_at_entry`: A program entry of 91 instructions that calls three other routines.
- `select_half_or_input_word`: Returns the first input word unchanged unless a second argument equals one, in which case it returns half of a third word.
- `mask_word_with_table_entry`: Looks up a 16-bit entry in a fixed word table, indexed by the second argument, and returns that entry ANDed with the first argument.

### Port-Handler

- `setup_hunk_record_from_table`: Fills a record from a table entry chosen by the first argument, sets several following pointer and size fields from computed addresses, and branches on the record's type value to pick one of three base offsets.

### Disk-Validator

- `format_counter_as_digit_char`: A program entry of 386 instructions that checks a counter field and, when it is ten or more, calls a routine and turns the result into a digit character; otherwise it uses a space.

### Ram-Handler

- `copy_memory_then_call_pointer`: Calls the exec CopyMem routine through a global library base, then makes an indirect call to a target it could not recover from the jump table, and returns.

### Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:InstallCrypt

- `clear_global_block_and_init`: Zeroes eighty 32-bit words in a block of global state and stores a register value into a field.
- `wait_for_installer_window_event`: Locks the current directory and waits for a window message port event, where certain gadget or close-window messages terminate the loop and release the lock.
- `open_install_window_and_buffers`: Clears several state fields, then opens an installer window and calls a routine with a specific value, storing the result in a state field and calling a failure routine if a required result is zero.
- `draw_install_path_prompt`: Draws the install path prompt with graphics calls that move the pen, set its colour and write text.
- `show_install_failure_and_close`: If an error code exceeds a certain threshold, it clears a portion of the screen before drawing an error message, then closes a window and cleans up a linked list.
- `copy_counted_name_with_colon`: Copies the name stored at an indexed table entry into the destination, using the length byte stored there and resetting the length when it exceeds 33.
- `check_entries_under_forbid`: Forbids multitasking, walks a linked list of entries and calls two routines for each entry with a non or non-zero field, setting the return value to 1 when a comparison fails.
- `prompt_insert_disk_and_wait`: If a check on the given path fails, it draws a prompt asking the user to insert a disk, with a short delay, and copies the path name into a buffer ending in a colon.
- `copy_file_with_progress_message`: Copies a file from a source to a destination by opening both, reading and writing blocks in a loop, and closing both.
- `install_game_files_to_destination`: Prepares the destination directory, creating it if missing, and iterates through a table of files to copy from a source disk.
- `forward_base_offset_pointer`: Calls another routine with a pointer into a fixed offset of the data base, the argument it received, and the address of its own stack argument, then returns that result.
- `copy_byte_array`: Copies a given number of bytes from a source pointer to a destination pointer, one byte at a time, and returns the destination pointer.
- `compare_byte_strings_ordered`: Compares two byte strings and returns a status code indicating if one is greater, smaller, or if they are identical.
- `return_without_action`: The routine does nothing and returns at once, with no calls and no effect on its arguments.
- `init_control_block_with_streams`: Allocates a control block, initializes certain fields, and writes a tag into a message area using current input and output streams.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d5.
- `parse_quoted_arguments_into_buffer`: Allocates a buffer with a library call and, when that succeeds, copies a name and an argument string into it.
- `append_string_with_count_limit_2`: Appends a source string to the end of a destination string, copying at most the count given in the third argument.
- `copy_string_with_limit_3`: The routine finds the end of the first string, then copies characters from the second string into the first, decrementing a counter for each character copied.
- `append_string_to_destination_2`: Appends a zero-terminated source string to the end of a destination string, with no length limit, and returns the destination pointer.
- `append_string_to_buffer_2`: Finds the end of a string and appends a second string to that location.
- `append_string_to_null_terminated_string`: finds the end of the first string and copies the second string to that location, effectively appending it.
- `copy_string_or_pad`: Copies characters from a source string into a destination buffer up to a count given as the third argument.
- `multiply_32_bit_values`: Multiplies two 32-bit register values modulo 2 to the power 32, using 16-bit partial products.
- `format_string_with_percent_codes`: Copies literal bytes from a format string into a destination buffer and expands percent codes using flags, widths and digit tables for upper and lower case number output.
- `calculate_string_length_2`: Calculates the number of bytes between the start of a string and its null terminator.
- `run_routine_over_table_entries`: Walks a table of 22-byte entries in the data area and calls another routine once for each entry until the end of the table.
- `write_byte_to_buffered_record`: Writes one byte to a buffered record after checking its status flags, and a null record gives minus one.
- `close_record_and_reset_fields`: Closes a valid record that has a non-zero status, calling routines to flush or release its buffer, and writes a set numeric field as five decimal digits first.
- `close_record_and_reset_fields_2`: Closes a record with a non-zero status, calling routines to flush or release its buffer, and writes a set numeric field as five decimal digits first.
- `return_zero_value`: Returns zero without doing anything else, and it makes no calls.
- `tiny_addq_addq`: Small routine of 2 instructions that adds 2 to a5, then adds 8 to d0.
- `apply_to_record_or_all`: With no argument, it walks the record table and calls a routine with a minus-one argument on each active record that lacks certain flag bits, returning minus one if any such call returns minus one.
- `scan_record_table_for_failure`: Scans the record table from its start and calls a routine with a minus-one argument on each active record that lacks certain flag bits.
- `forward_call_and_return`: Calls one routine and returns without using its result or changing anything else.
- `perform_signed_division`: Checks the signs of d0 and d1, calls another routine, and optionally negates the result before combining it with d1 into a 32-bit value.
- `apply_sign_around_call`: Records the sign of its two register arguments, calls another routine on the values, and negates that result when the signs call for it.
- `negate_result_by_sign`: Calls another routine and then negates its second result when the first register argument is negative.
- `recursive_call`: The routine calls itself indefinitely without performing any other operation.
- `perform_32bit_division`: Calculates the quotient and remainder of a 32-bit integer division using inputs in registers d0 and d1, returning the result in a concatenated 64-bit value.
- `init_stream_buffer`: Initialises a stream record's buffer pointers and flags.
- `unlink_and_free_list_entry`: Searches a global linked list for the given entry and unlinks it when found, then calls a library routine on it.
- `walk_list_and_clear_head`: Walks a global linked list and makes a library call for each entry with the same argument register, then clears the list head and returns zero.
- `free_all_list_nodes`: Frees every node of a linked list through a library call, then clears the list head and returns zero.
- `allocate_and_link_list_record`: When given a non-zero argument, it allocates a record through a library call and, if that succeeds, links the record onto the front of a global list and stores the argument in it.
- `copy_string_simple_2`: Copies characters from a source string to a destination buffer until a null terminator is reached.
- `check_entry_is_interactive`: Checks a numbered table entry: an out-of-range or empty index stores error code 3 and returns minus one.
- `seek_entry_and_record_error`: Checks a numbered table entry, storing error code 3 and returning minus one when the index is out of range or the entry is empty.
- `delete_file_or_record_error`: Calls the dos library DeleteFile routine with its argument.
- `return_zero_2`: The routine is empty and simply returns a zero value to the caller.
- `write_record_and_report_error`: Checks a numbered table entry, storing error code 3 for an invalid index or code 6 when the entry's mode bits are clear.
- `check_flag_bits_and_return`: Calls a library routine that returns a flag word and tests bit 4096.
- `write_four_bytes_to_output`: Writes four bytes from a global buffer to the standard output through the operating system, then flshes the output and calls another routine with the value one.
- `drain_list_then_run_cleanup`: Repeatedly removes the head of a linked list and calls the cleanup routine stored in each node until the list is empty.
- `shutdown_installer_resources`: Closes each stream entry, releases a directory lock, and frees the window, screen and library resources held in global fields, restoring a saved value along a way.
- `do_nothing_2`: The routine is empty and performs no operations.
- `close_indexed_entry_handle`: Checks a numbered table entry, storing error code 3 and returning minus one when the index is out of range or the entry is empty.

### Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:BlackCrypt

- `clear_state_block_and_start`: Zeroes sixty-seven 32-bit words of global state, stores a register value in a field, and tests a flag bit that may call a library routine.
- `handle_workbench_and_disk_loop`: Closes and reopens the Workbench interface around a lock on the game disk volume, taking the current directory first.
- `call_routine_and_branch`: Sets d2 to negative 1, calls a routine at an offset from the program counter, and then branches to another routine.
- `push_value_and_call`: Calls a routine, pushes the word 4 onto the stack, pushes a longword from an offset of the program counter onto the stack, calls another routine, cleans the stack, and branches to another routine.
- `push_value_and_call_alternate`: Calls a routine, pushes the word 5 onto the stack, pushes a longword from a different offset of the program counter onto the stack, calls another routine, cleans the stack, and branches to another routine.
- `prompt_disk_and_dispatch_loop`: Calls a routine to prompt for a disk insertion, updates a status structure with a code of 6, and then enters a loop that evaluates a status code from a structure to perform various system operations including loading segments, delaying, or unloading segments.
- `dispatch_event_loop_handler`: Executes a loop that evaluates a status code from a structure to perform various system operations including loading segments, delaying, unloading, and handling disk insertion or input events.
- `execute_system_sequence`: Calls two routines, copies a 16-bit word from d0 to d2, and branches to another location.
- `reset_and_reload_segment`: Calls two routines, clears a longword at offset 82 of a structure pointed to by a4, pushes two addresses onto the stack to call a third routine, sets a longword at offset 82 of the structure to 33799, and branches to another location.
- `dispatch_event_loop`: Enters a loop that evaluates a status code from a structure to perform various system operations including loading segments, delaying, unloading segments, or polling input events.
- `initialize_graphics_and_config`: Opens the graphics and intuition libraries, loads the configuration file, and initializes bitmaps and screen structures for the main game window.
- `execute_shutdown_callbacks_3`: The routine checks multiple global flags and executes corresponding function pointers from library bases.
- `delay_and_unload_segment`: Sets a value in a global variable and calls several indirect function pointers.
- `poll_input_events_until_done`: Loops calling an input routine and switches on the event type.
- `dispatch_and_load_process`: Calls a dispatch routine, then checks a byte at an offset of a structure in a6; if certain conditions are met, it calculates a new value by shifting and adding 16384, stores it in a global variable, and calls a routine to load a segment and create a process.
- `clear_register_and_branch`: Clears register d2 and branches to another location.
- `execute_jump_table_call`: Pushes a value onto the stack, calls another routine, cleans the stack, and branches to another location.
- `branch_after_call`: Calls a routine, clears d2, and branches to an earlier instruction in this routine.
- `execute_jump_table`: Performs a conditional jump based on a comparison and uses a table to determine the destination of a jump.
- `convert_string_case`: Copies a byte array and converts lowercase characters to uppercase by subtracting a constant.
- `scan_entries_for_failure_2`: Walks a linked list of entries and, for each entry with a non-zero flag, calls a routine with a stored field and a second routine with the argument through a 120-byte stack buffer.
- `prompt_insert_disk_wait_messages`: Returns a stored value when a global flag is set; otherwise it checks the path and, when that fails, copies the path into a buffer with a colon, shows the PLEASE INSERT DISK prompt with a delay, and loops over port messages until a message of class 32768 triggers another check.
- `wait_for_library_call_2`: Loops calling a function pointer from a library base until a second function pointer returns a non-zero value.
- `load_segment_and_create_process`: Loads a code segment and, when that succeeds, creates a process from it, storing the process and segment handles and clearing three fields.
- `execute_library_shutdown_callbacks`: Invokes function pointers from library bases and global structures to perform shutdown sequences, checking flags and iterating through callback lists.
- `handle_segment_load_unload`: Calls routines to load or unload memory segments, checks the return value, and if successful, invokes a function pointer with calculated parameters before cleaning up.
- `call_routine_per_character`: Makes two calls, then calls another routine once for each character of a zero-terminated string argument before returning.
- `do_nothing_routine`: The routine is empty and performs no operations before returning.
- `call_routine_per_character_2`: Calls one routine with its argument, then calls another routine once for each character of a zero-terminated string argument before returning.
- `call_two_routines_in_order`: Calls two routines one after the other and returns, without any other work.
- `clear_blit_area_then_call`: Calls one routine, then makes a call through a global block pointer at minus 300 bytes and returns.
- `process_string_with_tiles`: Iterates through null-terminated strings like 'CONFIGURATION MENU' and 'MAIN MENU', copying tile data for each character.
- `copy_menu_tiles`: Calls call_routine_per_character_2 and call_routine_per_character multiple times using various word constants and pointers to process several strings.
- `render_save_menu`: Calls two different character routines with specific pointers and constants, then checks a byte at an offset of a structure in a4 to decide whether to call a fourth character routine before returning a value in d0.
- `process_save_game_strings`: Calls another routine four times with specific word constants and pointers, then sets d0 to 6 and branches to another location.
- `render_menu_options`: Calls a routine to display a sequence of text strings based on a conditional check of a global byte.
- `render_menu_text_strings`: Calls a routine for each character in a sequence of four text strings, passing a specific word value and a pointer to each string.
- `copy_text_tiles`: Calls call_routine_per_character_2 four times with different word constants and pointers to process a set of strings.
- `check_input_event`: Polls an input event queue, checking message types and data fields to determine a return value based on specific conditions.
- `dispatch_by_event_code`: Clears a blit area through a graphics function pointer, then dispatches on a numeric code: codes one, two, three, four and six each call a different subroutine.
- `dispatch_input_and_branch`: Calls a routine, moves the result from d0 to d2, and branches to another location.
- `dispatch_and_branch`: Calls a routine, moves the result from d0 to d2, and branches to another location.
- `call_external_routine`: Calls a routine, moves a word from d0 to d2, and branches to another location.
- `call_menu_routine`: Calls a routine, moves a word from d0 to d2, and branches to another location.
- `dispatch_input_event`: Calls a routine, saves d0 in d2, and branches to another location; if d0 is not less than 7, it uses d0 to jump to a location in a table; it then calls another routine, polls input for a slot, and calls another routine before returning.
- `process_menu_strings`: Calls another routine, calls another routine, and if a byte at an offset of a5 is not zero, calls another routine before branching to another location.
- `copy_config_tiles`: Calls call_routine_per_character four times with different word constants and pointers to process a set of strings.
- `prompt_keyboard_input`: Displays a series of text strings and enters a loop that waits for keyboard input, storing up to six characters into a local buffer while filtering for specific characters and checking for an escape key to abort.
- `display_instruction_text`: Copies a string from a data table to a buffer, calls a routine to display the text character by character, and then branches to another routine.
- `display_input_prompt_and_process_sequence`: Copies a string to a buffer, calls a routine to display it, and then calls a routine to process the character sequence.
- `display_input_prompt_sequence`: Copies two text strings into a buffer at an offset from a pointer in a5, then calls another routine to process each character of that buffer.
- `display_input_prompt_and_validate`: Copies a string to a buffer, displays it, and then enters a loop that waits for keyboard input to validate a character sequence.
- `display_input_prompt_and_validate_2`: Copies strings to a buffer located at an offset from a5, calls a routine to process each character, and enters a loop that reads input characters to validate them against a set of criteria before storing them in a sequence.
- `calculate_jump_target`: Compares a word in d0 to 6, and if it is not equal, adds d0 to itself and loads a word from a table using d0 as an offset before jumping to the resulting address.
- `validate_input_character_sequence`: Checks if a character from a structure matches a specific byte value and evaluates a sequence of characters stored in a buffer.
- `clear_and_branch`: Clears d4 and branches to another location.
- `validate_and_store_input_character`: Checks if a character in a register already exists within a buffer by looping through the buffer, then checks if the character is a vowel.
- `store_byte_and_call_libraries`: Stores a byte from register d2 into a buffer at an offset determined by d5, increments d5, and then calls two library functions using pointers retrieved from a global table.
- `branch_to_offset_11a6`: Branches to a specific offset within another routine.
- `branch_to_offset_1090`: Branches to a specific offset within another routine.
- `copy_config_tile_bytes`: If a word in d6 is zero, it copies 6 bytes from a source buffer at an offset of negative 6 from a5 to a destination buffer at an offset of negative 32586 from a4, then calls three other routines before returning.
- `copy_and_poll_input`: Copies two longwords from a location offset from a4 to a location offset from a5, calls three routines, and if d3 is zero, calls another routine and branches back to the start of this routine.
- `handle_menu_selection`: Calls prompt_keyboard_input and enters a loop that branches based on the returned value.
- `process_item_pointer`: Checks a byte at an offset of a4, and if it is zero, pushes a word and a pointer onto the stack, calls another routine, cleans up the stack, and branches to another location.
- `push_string_and_call_routine`: Pushes a 16-bit value onto the stack, pushes the address of a string onto the stack, calls another routine, cleans up the stack, and branches to a different location.
- `manage_game_save_file`: Clears a screen area and prompts the user to insert a disk, then attempts to open a file for writing or reading.
- `execute_disk_io_loop`: Pushes the current stack pointer to the stack, calls four consecutive routines, adjusts the stack pointer, and branches to a nearby location.
- `manage_disk_io_menu`: Clears a screen area and enters a loop to poll for input to manage a menu for reading or writing data to a disk.
- `handle_disk_io_menu`: Enters a loop that polls for input to manage a menu for reading or writing data to a disk.
- `clear_area_and_set_flag`: Clears a blit area, sets a register to one, and branches to a later instruction to update two values from a structure using a pointer in a5 before returning.
- `branch_on_comparison`: Compares a word in d0 to six, branches if it is less than six, otherwise it adds d0 to itself, loads a word from a table using d0 as an index, and jumps to a location from that table.
- `branch_to_offset_4928`: Performs a short branch to a location 4928 bytes ahead in the instruction stream.
- `copy_two_longwords`: If d3 is equal to 1, copies two longwords from a location offset from a5 to a location offset from a4.
- `load_pointer_from_structure`: Loads a pointer from offset 8 of a structure in a5 into a3, and if a3 is not zero, moves a3 to a2 and branches; otherwise it returns.
- `process_string_sequence`: Loads a pointer from a structure in a4, then iterates through a string by calling a routine for each character and calculating the string length to advance the pointer.
- `process_event_sequence`: Checks a value at an offset of a structure pointed to by a4, and if it is 1 and d3 is not 0, calls call_routine_per_character_2, call_two_routines_in_order, check_input_event, and clear_blit_area_then_call.
- `store_registers_into_globals`: Writes the first two data-register values and a third register into three fixed global locations.
- `copy_tile_data`: Copies a block of data from a source pointer to multiple destination addresses with fixed offsets, iterating five times.
- `scan_global_variable_array`: Iterates through a global variable array in two passes, stepping by two bytes.
- `decrement_nibble_loop`: Executes a loop sixteen times, calling indirect function pointers and a routine to decrement nibble values in a list of words.
- `decrement_nibble_values`: Iterates through a list of words, decrementing each of the three nibble components if they are non-zero.
- `increment_nibble_loop`: Executes a loop sixteen times, calling indirect function pointers and a routine to increment nibble values in lists of words.
- `increment_mismatched_nibbles`: Iterates through two lists of words, incrementing the high-order nibble, middle nibble, or low-order nibble of the first list if that nibble does not match the corresponding nibble in the second list.
- `copy_byte_array_2`: Copies a specified number of bytes from a source pointer to a destination pointer.
- `compare_string_bytes`: The routine compares two byte strings character by character, returning a status code based on the first difference or equality.
- `return_empty_routine`: This routine is empty and performs no operations before returning.
- `handle_workbench_disk_access`: The routine interacts with the Workbench interface, locks the game disk volume, parses strings, opens icon libraries, and processes input events and state changes.
- `parse_string_tokens`: Parses a string or quoted-string, extracting tokens separated by whitespace and storing them into a global table.
- `parse_string_tokens_2`: Parses a source string into individual tokens, handling quoted strings and skipping whitespace, then stores pointers to these tokens in a destination array.
- `copy_string_with_limit`: Finds the end of a destination string and copies characters from a source string, decrementing a counter until a null terminator or a zeroed counter is reached.
- `copy_string_with_limit_2`: Finds the end of a destination string and then copies characters from a source string while decrementing a counter.
- `load_icon_data`: Loads an icon from a library and stores a pointer or value from a table into a parameter-relative or global location.
- `append_string_to_buffer`: Advances a pointer to the end of a destination string and appends characters from a source string until a null terminator is copied.
- `copy_string_to_null`: Locates the null terminator of the first string parameter by incrementing a pointer until a zero byte is found, then copies the second string to that location.
- `copy_string_or_pad_2`: Copies a source string to a destination, or if a count is zero, fills the destination with null bytes, ensuring the destination is null-terminated.
- `allocate_and_init_record`: Calls a library routine for an identifier and returns zero when that is minus one; otherwise it allocates a record, calls a library routine on it and returns zero if the allocation fails.
- `initialize_library_context`: The routine checks a field in a structure; if non-zero, it calls a function from a library base.
- `calculate_string_length`: Calculates the number of bytes between a starting pointer and the null terminator of a string.
- `setup_self_linked_header`: Stores the record's own address plus four in its first word, clears its second word, and stores the record address in its third word.
- `copy_string_simple`: Copies characters from a source string to a destination buffer until a null terminator is encountered.
- `execute_shutdown_callbacks_2`: Iterates through a linked list of function pointers, executing each one and updating the list pointer to the next element.
- `execute_shutdown_callbacks`: Iterates through a list of indices to invoke a helper routine, then executes a series or conditional function pointers stored in global memory.
- `return_empty_routine_2`: the routine is empty and performs no operations before returning.
- `close_file_with_validation`: Checks if a file handle or index is valid and within a certain range before calling a library function to close a file and clearing a status flag.

### Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:bcdfp

- `clear_state_block_and_start_2`: Zeroes 494 32-bit words of global state, stores a register value in a field, and tests a flag bit that may call a library routine.
- `open_graphics_intuition_libraries`: Opens the graphics and intuition libraries and, when either open fails, calls an error routine with a message from the text table.
- `initialize_data_block`: Calls a routine to initialize a data structure, then retrieves a pointer from a global variable.
- `process_linked_list`: Traverses a linked list of structures using an index.
- `initialize_entity_structures`: Iterates four times to initialize distinct data structures, setting specific byte flags, clearing arrays, and copying configuration values from a global table.
- `initialize_entity_arrays`: Loops four times filling per-entry 16-bit fields with values computed from the loop index, and sums attribute values from a global table into a per-entry long.
- `serialize_character_data`: Copies character configuration blocks, including linked list entries and attribute tables, from memory buffers into a file handle.
- `wait_for_library_call`: Repeatedly calls a function pointer from a global structure until a second function pointer from the same structure returns a non-zero value.
- `initialize_library_call_state_zero`: Sets a global state variable to zero and invokes a function pointer from a global structure.
- `initialize_library_call_state_two`: Sets a global state variable to two and invokes a function pointer from a global structure.
- `initialize_library_call_state_eight`: Sets a global state variable to eight and invokes a function pointer from a global structure.
- `initialize_library_call_state_three`: Sets a global state variable to three and stores a parameter in a global variable.
- `initialize_library_call_state_seven`: Sets a global state variable to seven and stores a parameter in a global variable.
- `initialize_library_call_state_ten`: Sets a global state variable to ten and stores two parameters in global variables.
- `transfer_dungeon_save_files`: Reads and writes save data through the dos library, opening files on the game disk and save paths, transferring blocks, and waiting briefly.
- `manage_disk_access_and_saves`: Uses DOS library calls to lock and unlock the game disk, checks disk status with referenced text, and calls routines to manage dungeon save files and handle workbench disk access.
- `dispatch_sprite_by_type`: Loops over a sprite record and dispatches on its type value.
- `dispatch_sprite_logic_variant_zero`: Loads a word from an offset of a pointer in a4, performs a bitwise operation to isolate the lower two bits after adding 65535, pushes the result to the stack, calls three different routines, adjusts the stack pointer, and branches to another location.
- `dispatch_sprite_logic_variant_one`: Loads a word from an offset of a pointer in a4, adds 1, performs a bitwise operation to isolate the lower two bits, pushes the result to the stack, calls three different routines, adjusts the stack pointer, and branches to another location.
- `call_and_branch`: Calls another routine and then branches to another location.
- `dispatch_sprite_logic_2`: Performs a multi-way branch based on a value from a global variable at an offset from a pointer in a4, potentially calling render_text_string and process_hunk_header twice with a value from the stack.
- `setup_sprite_rendering_loop`: Stores zero in a global field and makes a series of subroutine calls.
- `loop_calculate_sprite_offsets`: Iterates nine times, calling a routine that calculates a memory offset by multiplying a register value by a constant and adding a base, storing the result in a global variable.
- `call_sprite_offset_calculator`: Calls a routine that calculates a memory offset by multiplying a register value by a constant and adding a base, storing the result in a global variable.
- `calculate_sprite_offset_value`: Computes a memory offset based on a register value and a constant, stores the result in a global structure, and calls helper routines for sprite processing.
- `configure_sprite_rendering_params`: Calls subroutines with fixed numeric arguments and with a value computed from a global index, stores its argument into a global index field, then makes further subroutine calls.
- `dispatch_sprite_rendering`: Checks a global index against a field and calls a subroutine when they differ.
- `call_sprite_logic_zero`: Clears the stack top, calls another routine, adjusts the stack pointer, and branches to a previous location in a loop.
- `call_sprite_logic_zero_alt`: Clears the stack top, calls another routine, adjusts the stack pointer, and branches to a previous location in a loop.
- `call_sprite_logic_one`: Pushes 1 onto the stack, calls another routine, adjusts the stack pointer, and branches to a previous location in a loop.
- `call_sprite_logic_one_alt`: Pushes 1 onto the stack, calls another routine, adjusts the stack pointer, and branches to a previous location in a loop.
- `call_sprite_logic_two`: Pushes 2 onto the stack, calls another routine, adjusts the stack pointer, and branches to a previous location in a loop.
- `call_sprite_logic_two_alt`: Pushes 2 onto the stack, calls another routine, adjusts the stack pointer, and branches to a previous location in a loop.
- `call_sprite_logic_with_three`: Pushes the word three onto the stack, calls a routine at a relative offset, increments the stack pointer by two, and branches to another routine.
- `call_sprite_logic_with_three_alt`: Pushes the word three onto the stack, calls a routine at a relative offset, increments the stack pointer by two, and branches to another routine.
- `call_sprite_logic_with_four`: Pushes the word four onto the stack, calls a routine at a relative offset, increments the stack pointer by two, and branches to another routine.
- `dispatch_sprite_logic`: Calls check_and_update_sprite_condition, then calls store_word_in_state, process_hunk_header, and call_two_routines_then_stop.
- `check_range_overlap`: Searches ten entries in a global table of 16-bit coordinate pairs for one whose box contains the argument, testing the low half of the argument within thirteen units of the entry's first value and the high half within nine of its second.
- `check_and_update_sprite_condition`: Checks values in a global table indexed by parameters, increments or decrements specific fields if conditions are met, and calls helper routines for further processing.
- `update_sprite_conditions`: Calculates offsets into a table, compares values, and if they differ, decrements one value and increments another in a table before calling check_sprite_condition twice.
- `check_sprite_condition`: Reads a value from a global table indexed by a parameter, divides it by ten, and branches to different helper routines based on whether the result is zero.
- `render_text_string`: Calculates string length and renders text by copying character data from global tables into a buffer, clearing the buffer beforehand.
- `clear_entry_flag_and_refresh`: Reads a value for the current entry and, when it is non-zero, checks two flag bits returned by subroutine calls.
- `clear_entry_flag_and_refresh_2`: Checks two flag bits from subroutine calls and clears either the entry's byte at the index before a stored value or its first byte, then calls subroutines to refresh the entry.
- `clear_entry_byte_and_refresh`: Clears the entry byte just before a stored index, then calls subroutines with a value of one to refresh the entry.
- `store_entry_byte_and_refresh`: Reads a subroutine value for the current entry and, when it is not twelve and a second subroutine returns non-zero, stores that second value as a byte in the entry and writes a zero byte after it.
- `search_short_table`: Searches a global table of 26 16-bit values for the argument and returns its index plus 65 when found, or zero when not found.
- `calculate_string_offset`: Calculates an offset by multiplying a word from offset -29132 of a4 by 26 and adding it to a base address, calls a routine to measure a string length at that location, and returns if the length is zero or twelve.
- `update_field_and_sync`: Calculates an offset using a value from a global variable and d2, writes the byte 32 to that offset, then repeats a similar calculation to clear a byte at a different offset, calls a routine with a value from the stack, calls a routine to store state, calls a routine to swap buffers, and calls the first routine again with a value from a global variable before branching to another routine.
- `allocate_memory_and_copy`: Copies a byte from a source pointed to by a0 to a destination pointed to by a2, then calls a routine using a pointer from a system library.
- `initialize_graphics_library`: Allocates memory, opens the graphics library, and populates a data structure with pointers and values from global variables and library calls, returning a status code.
- `process_integer_list`: Iterates through a list of integers until a terminator value is found, invoking a library function for each entry.
- `process_list_with_lib`: Calls a library function, then iterates through a list of integers until a value of -1 is found, calling another library function for each entry.
- `empty_stub_2`: This routine is empty and performs no operations.
- `cleanup_allocated_memory`: Walks two tables of function pointers and global slots, calling a routine for each entry that is set.
- `initialize_game_state_structures`: Copies two coordinate words from one state block into another and disables all interrupt and DMA bits through the custom chip enable words.
- `configure_cia_registers`: Writes specific values to CIAA control and interrupt registers and updates global status flags and memory offsets.
- `wait_for_vertical_blank`: Polls the interrupt request word until its vertical blank bit is set, then writes that bit back to the interrupt request register to acknowledge it.
- `wait_for_vertical_blank_frames`: Waits for the vertical blank bit once per frame in the requested count, acknowledging each one through the interrupt request register.
- `wait_for_flag_repeatedly`: Repeatedly clears a flag byte in a state block and spins until a handler sets it non-zero.
- `forward_call_to_subroutine`: Calls one other subroutine and returns without doing any work of its own.
- `initialize_copper_list`: Constructs a sequence of graphics hardware commands in memory by writing control words and color data from global variables.
- `copy_block_data`: Copies a block of 17 words from a source address derived from a global pointer to a destination buffer.
- `get_state_field_pointer`: Returns the address of one fixed field inside the shared state block.
- `store_word_in_state`: Stores the word passed in the data register into a fixed word field of the shared state block.
- `initialize_buffer_values`: Checks a condition and either clears a buffer with a specific constant or fills a series of memory locations with another constant value.
- `copy_global_to_buffer`: Stores a value to a local offset and copies a sequence of words from a global variable area into a buffer structure.
- `update_sprite_motion_frame`: Toggles a frame bit and increments a frame counter, then applies pending table data to a copper list.
- `dispatch_through_unrecovered_pointer`: Jumps through a pointer whose target was not recovered, so it only forwards control to code that is not present here.
- `acknowledge_selected_interrupt_requests`: Reads the interrupt request word, keeps only the copper, vertical blank and blitter bits, and writes them back to clear those requests.
- `acknowledge_ports_interrupt`: Calls one subroutine, then writes the ports interrupt bit to the interrupt request register to acknowledge it.
- `jump_through_unrecovered_pointer`: Forwards execution through a jump pointer that was not recovered, so its behaviour depends on code missing from this dump.
- `cycle_copper_pattern_timer`: Counts down a timer while enabled and toggles a phase word each time it expires.
- `reload_copper_table_timer`: Sets the frame timer to a fixed value, copies six words from a global table into the copper list, and forwards to an unrecovered jump target.
- `flush_pending_copper_entries`: Copies eleven words of pending data into the copper list when a flag is set, and fills six entries from a table when a stored index is non-negative.
- `initialize_input_state`: Resets input counters and queue entries, reads hardware status bytes, and initializes local structure fields with specific constant values.
- `feed_table_word_into_copper`: Every third frame, when a flag is set, takes the next word from a cursor in a table and writes it to a fixed copper list slot.
- `store_result_and_forward`: Stores a subroutine result and a second returned value into the state block, calls one more subroutine, and forwards to an unrecovered jump target.
- `check_regions_on_state_change`: Compares two stored state bytes with newly read values.
- `scan_regions_for_position_flags`: Scans a list of rectangular regions for one with the first flag bit set that contains the current position.
- `update_second_state_flag`: Compares a stored state byte with a newly read value and, when it changed to non-zero, scans the region list for a match on the high flag bit.
- `apply_second_state_change`: Stores a newly read state byte and a second returned value, then checks a region list for a match on the high flag bit when the new byte is non-zero.
- `scan_regions_for_second_flag`: Walks a list of rectangular regions until it finds one with the high flag bit that contains the current position.
- `store_state_and_tick_counter`: Stores a subroutine result and a second returned value into the state block, calls one more subroutine, and decrements a non-zero counter in the block.
- `store_state_and_run_tail`: Runs an extra subroutine call before storing a result and a second value into the state block, then decrements a non-zero counter.
- `decode_keyboard_serial_byte`: Handles the keyboard serial register handshake by clearing the previous acknowledge and reading a byte when the strobe bits are set.
- `set_key_bitmap_bit`: Sets one bit in a held-key bitmap in the state block, using the low three bits of the input to select it.
- `read_key_bitmap_byte`: Returns one byte of the held-key bitmap from the state block.
- `initialize_structure_fields`: Sets several byte and word fields within a structure to zero or specific constants, and then clears a block of 335 words pointed to by a pointer within that structure.
- `scroll_bitplane_buffer_frame`: Advances a wrapping offset by two bytes out of forty-two and copies byte pairs for eight bit planes from two source addresses into a ring buffer at that offset.
- `advance_scroll_buffer_countdown`: Decrements a countdown held at the address passed in a register when a flag is set.
- `advance_copper_table_cursor`: Moves a table cursor forward by one row unless it has reached its limit.
- `call_and_return_first_word`: Calls one subroutine, calls a second one, and returns the word produced by the first call.
- `read_next_text_character`: Reads the next byte from a circular text buffer, moving backwards with wrap-around at eighty entries.
- `clear_text_active_flag`: Clears a flag in the state block and returns zero.
- `scan_string_for_nonzero_result`: Walks a zero-terminated byte string, calling a subroutine on each character and setting an active flag.
- `write_byte_to_text_ring`: Stores one byte at the write index of a circular text buffer and then advances the index with wrap-around.
- `set_text_write_index`: Sets the write index of a circular text buffer to the value passed in a register and returns zero.
- `copy_and_process_data`: Calls a routine, clears a memory area pointed to by a value from a structure in a5, loops to find a null terminator in a string, uses that length to copy seven bytes from a source table to a destination table, and finally writes 2400 to a location in a5 and performs a multiplication and assignment.
- `copy_data_and_calculate`: Copies eight words from a source table to a destination table using a loop, sets a word at offset 974 of a structure in a5 to 2400, and stores a product of two values into offset 1084 of that structure.
- `decrement_timer_array`: Decrements each non-zero counter in a block of nine words by one.
- `update_input_state`: Initializes a structure with specific constant values for coordinates and flags, rotates a byte counter, and stores word values into indexed fields.
- `step_position_within_bounds`: Either clears two blocks of object data or advances the position by velocity or by a scaled step with fractional accumulators.
- `clamp_position_to_bounds`: Clamps the x and y values passed in registers to the stored bounds and saves them as the current position.
- `clamp_y_and_store_position`: Clamps the vertical value passed in a register to its stored bound, saves it with the horizontal value as the current position, calls two subroutines, and forwards to an unrecovered jump target.
- `offset_and_clamp_position`: Adds the stored offset to the current position and clamps the result to the stored bounds.
- `clear_object_data_pointers`: Clears two blocks of object data through pointers held in the state block and forwards to an unrecovered jump target.
- `compute_wrapped_byte_deltas`: Stores two newly read bytes as the previous values and returns the signed differences from the old values.
- `compute_direction_and_delta`: Returns two values: the complement of the first input, negated when a flag is set, and the second input minus a stored value, wrapped into the signed byte range.
- `return_immediately`: The routine contains instructions but performs no data manipulation or hardware access before returning control to the caller.
- `record_position_history_entry`: Stores the current position in a four-entry history ring, steps the ring index back with wrap-around, and writes a subroutine value or zeros into two output slots.
- `process_secondary_input`: Decrement a byte counter with a lower bound check, then call another routine to compute values.
- `compute_input_flags`: Calculates a composite value by adding input registers to global offsets, checking specific bit conditions, and combining the results into a structured return value.
- `read_fire_button_pressed`: Returns minus one when a bit of the cached port A status byte is clear, which means the fire button is pressed, and zero otherwise.
- `store_mouse_data`: Writes byte values from registers and memory locations into specific offsets of a global structure, while managing interrupt enable flags based on a condition.
- `store_word_data`: Stores two word-sized values from registers into specific offsets of a global structure and updates a global flag based on a memory condition.
- `store_velocity_components`: Stores two input words into the velocity fields of a state block.
- `negate_and_store_values`: Negates two 16-bit values in d0 and d1, subtracts 12 from each, and stores the results into a table at offsets 162 and 164 from a base address.
- `store_registers_to_table`: Stores the values of d0, d1, a0, and the stack pointer into a table at offsets 168, 170, 172, and 174 from a base address.
- `compare_d0_value`: Loads a word from a relative offset into a0, compares d0 against the values one, two, three, and four, and either branches to another routine or sets d0 to negative one and returns.
- `get_byte_from_offset_c4`: Returns the byte stored at offset 196 of the structure pointed to by a0.
- `get_byte_from_offset_c5`: Returns the byte stored at offset 197 of the structure pointed to by a0.
- `get_word_from_offset_82`: Returns the word stored at offset 130 of the structure pointed to by a0.
- `get_word_from_offset_84`: Returns the word stored at offset 132 of the structure pointed to by a0.
- `update_scaled_value`: Stores a 16-bit word from d0 into offset 178 of a structure, calculates a new value by multiplying d0 by 46 and dividing by 100 into offset 176 of the same structure, and modifies the interrupt enable register based on a bit test of a value from the stack.
- `update_status_and_copy`: Calls another routine, checks a byte at offset 200 of a5, and if it is not zero, writes a word of 1 to a0 incremented, clears a word at a0 incremented, writes two words from a5 to a0, and decrements a byte at offset 200 of a5.
- `clear_list_node`: Pushes a word from a6 plus 28 onto the stack, sets a word at offset 154 of a6 to 4096, checks a word at offset 190 of a5, and if zero, clears two longwords at a0, checks bit 14 of a value from the stack, and either returns or sets offset 154 of a6 to 32768 and returns.
- `transfer_and_clear_list_node`: Decrements a 16-bit value at offset 190 of a structure, traverses a table using a value from offset 188 and offset 182, and if a comparison with 128 is met, copies a 32-bit doubleword from a table to a destination and clears the source entries, otherwise it returns.
- `manage_queue_operations`: Checks a state byte and a counter; if active, it waits for a flag to clear while decrementing a value.
- `clear_queue_entries`: Resets the queue head, tail and count words and zeroes 128 two-long entries of a buffer.
- `enable_queue_processing`: Sets a state word to one, which enables queue processing.
- `disable_queue_processing`: Sets a state word to zero, which disables queue processing.
- `store_mask_and_test_bit`: Stores the input word, masked to a fixed set of bits, as a control word.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `clear_bits_in_structure`: Clears specific bits in a 16-bit word at offset 194 of a structure by performing a bitwise AND with the inverse of a masked version of d0.
- `set_bits_in_structure`: Sets specific bits in a 16-bit word at offset 194 of a structure by performing a bitwise OR with a masked version of d0.
- `enqueue_entry_with_position`: When the input bits match the stored control word and the queue is not full, it appends an entry to the end of a list.
- `append_to_buffer_list`: Stores a pointer into a table indexed by a scaled global counter, increments the counter with a wrap-around check, and writes four words of data to the pointed-to buffer.
- `scale_value_to_counter`: Multiplies the input by a stored word and divides by sixty.
- `copy_byte_within_structure`: Copies a byte from offset 201 of a structure to offset 200 of the same structure.
- `transform_linked_list`: Iterates through a linked list of short values, copying and transforming them into a contiguous buffer.
- `render_text_string_2`: Clears a buffer and copies character data from two global tables based on input string characters.
- `return_immediately_2`: The routine performs no operations and returns to the caller immediately.
- `initialize_struct_fields`: Sets multiple fields within two global structures to specific constant values, including pointers to offsets within the structure and a reference to a global variable.
- `process_hunk_header`: Reads six words from a pointer stored in a global structure, copies them to other fields in the structure, toggles a status byte, and swaps two pointer fields based on that status.
- `call_two_subroutines`: Calls two other subroutines in order and returns.
- `compute_global_offset_and_call`: Calls one subroutine, stores a global address computed from the low word of a register as a scaled offset, and then calls another subroutine.
- `process_entries_until_negative`: Walks a list of five-word entries, calling a subroutine for each one, until it reaches an entry whose first word is negative.
- `render_sprite_stripes`: The routine configures a graphics structure with source and destination coordinates derived from input parameters.
- `store_parameters_and_call`: Stores three input values into global variables and then calls two subroutines.
- `call_blit_routines`: Saves registers, calls two different routines, and then restores the registers before returning.
- `process_list_with_loop`: Loads a word from a pointer in a2, enters a loop that calls another routine five times, and uses a branch to repeat the loop.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `call_two_routines_then_stop`: Calls two subroutines in order.
- `blit_list_entry_and_dispatch`: Stores a pointer argument and an advanced copy of it for later use, then computes blitter width and offsets from the referenced record.
- `prepare_sprite_render_state`: Configures global rendering parameters and calculates memory offsets for sprite drawing based on position flags and clipping bounds.
- `load_handler_pointer`: Loads a pointer from a structure in a5 into a0 and branches to another routine.
- `calculate_and_jump_to_handler`: Performs several arithmetic operations to calculate values for a structure in a5 and a6, then executes an indirect jump using a pointer from a structure in a5.
- `populate_and_loop_structure`: Populates a local structure in a6 with values from a structure and a list, then enters a loop that updates a6 entries using data from the list until a counter reaches zero.
- `wait_for_bit_clear`: Loops while bit 0 of a 16-bit word at offset 4 of the structure in a6 is set, then compares a 16-bit word at offset 6 of a6 with d0 after shifting it right by 8 bits.
- `rotate_byte_buffer`: Calculates a sum of three bytes from a table using carry propagation, stores the result in the first byte of the table, and then shifts five bytes from the table to the five preceding bytes in memory.
- `wait_and_update_nibbles`: Waits for specific byte flags at offsets 1112 and 1113 from a base pointer to become non-zero, then calls helper routines to decrement or increment nibble values in tables.
- `decrement_nibble_values_3`: Iterates through a table of 16-bit words, decrementing each non-zero nibble by one.
- `increment_nibble_if_different`: Compares nibbles of words in a destination table against corresponding words in a source table.
- `clear_five_buffers`: Zeroes five buffers of two thousand long words each, whose addresses are listed in the state block.
- `service_four_audio_channels`: Applies a pending command to each of four audio channels.
- `service_audio_channels_loop`: Services audio channels from the highest index down.
- `service_audio_channels_default`: Writes a default length and sample address for the current channel, then walks the channels downward.
- `write_jump_table_entry`: Writes a constant value of 2, a 16-bit parameter, and a calculated code pointer into a global table at an index determined by the second argument.
- `format_string_output`: Calls a routine to parse a format string and convert arguments into a formatted output buffer.
- `copy_string_buffer`: Copies bytes from a source pointer to a destination pointer in a loop based on a count parameter, using exec library calls.
- `do_nothing`: this routine is empty and performs no operations before returning.
- `parse_icon_library_data`: Opens the icon library, parses string tokens, and processes data structures using DOS library calls for input and output.
- `split_command_line_into_args`: Allocates an argument buffer and copies the command line into it, splitting on spaces and tabs while honouring double-quoted arguments.
- `parse_command_line_arguments`: Copies the command line into a newly allocated buffer, splitting it into arguments on whitespace and keeping quoted text together.
- `append_bounded_string`: Appends a source string to the end of a destination string, copying at most the given number of characters.
- `copy_string_with_limit_4`: The routine finds the end of the first string, then copies characters from the second string into the first, decrementing a counter for each character copied.
- `load_icon_tooltype_list`: Opens the icon library and reads the tool types of an icon, including the window entry, into an array.
- `append_string_to_end`: Appends a zero-terminated source string to the end of a destination string and returns the destination address.
- `concatenate_strings_unbounded`: Appends a zero-terminated source string to the end of a destination string and returns the destination address.
- `copy_string_or_pad_3`: The routine copies characters from a source string to a destination buffer up to a specified count or until the source ends.
- `create_named_message_port`: Allocates a signal bit and a port structure, then fills in its name, type and signal fields and records the owning task.
- `initialize_data_structure`: Sets specific bytes and words within a data structure pointed to by an argument, conditionally calls a function from a global table, and invokes two other functions from that table with a register value.
- `format_text_into_buffer`: Formats text from a template into an output buffer, handling flag characters, width and precision fields and length modifiers.
- `measure_string_length`: Counts the characters before the zero terminator of a string and returns the count.
- `initialize_empty_list_head`: Sets up a list header so that it points to itself with an empty list.
- `iterate_node_list`: The routine loops through a range of memory addresses, calling a node removal and formatting routine at each step.
- `write_byte_to_stream`: Writes one byte to a buffered stream, flushing the buffer when it fills or a newline is written, and returns the byte.
- `close_stream_and_release`: Flushes and closes a buffered stream, releases its buffer if one was allocated, and clears its fields.
- `finish_stream_close_status`: Flushes and closes a buffered stream as the routine above does, but also combines a previously computed status value to decide success.
- `return_success_zero`: Returns zero without doing anything else.
- `flush_all_open_streams`: Flushes every open stream in the stream table when given a null pointer, or flushes only the stream passed in.
- `flush_streams_or_one`: Behaves like the stream flush routine above, flushing all open streams for a null pointer and otherwise only the one stream given.
- `flush_remaining_open_streams`: Walks the stream table from a given position, flushing each open stream in turn.
- `call_single_subroutine`: Calls one subroutine and returns.
- `empty_stub_routine`: The routine consists of six instructions that perform no observable work and end with a return.
- `negate_result_if_negative`: Calls one subroutine and returns its second result, negated when the first input is negative.
- `call_and_pass_through_pair`: Calls one subroutine and returns the second input passed through together with the subroutine's second result.
- `forward_call_and_return_2`: Calls one subroutine and returns its result unchanged.
- `perform_32bit_division_2`: Calculates the quotient and remainder of a 32-bit integer division using inputs in registers d0 and d1, returning the result in a concatenated 64-bit value.
- `allocate_and_validate_node`: Allocates memory for a linked list node and validates an index against bounds.
- `remove_node_and_free`: Searches a linked list of allocated nodes for the given node, unlinks it when found and frees its memory.
- `find_and_free_list_node`: Searches the allocation list starting at the given node for the node passed in a register, then unlinks and frees it.
- `unlink_node_at_position`: Unlinks the node held in an address register from the allocation list and frees it through the memory release call.
- `pop_head_and_free`: Advances the allocation list head to the next node and frees the old head node.
- `free_all_allocated_nodes`: Frees every node in the allocation list, then clears the list head and returns zero.
- `iterate_and_call_function`: It then clears a global status variable.
- `allocate_linked_memory_node`: Allocates a block of the requested size with a link word in front, pushes it onto the allocation list, and returns the usable area after the header.
- `copy_zero_terminated_string`: Copies a zero-terminated source string into the destination, including the terminator, and returns the destination address.
- `check_stream_is_interactive`: Checks that a stream index is valid and open, then asks DOS whether that stream is interactive.
- `seek_stream_position`: Checks a stream index and seeks the stream to the given position through DOS. If the seek fails it stores the DOS error code and returns minus one.
- `delete_file_by_name`: Deletes a file through DOS. When the deletion fails it records the DOS error code and returns minus one, otherwise it returns zero.
- `return_zero`: This routine returns a zero value.
- `write_block_to_stream`: Checks a stream index and access mode, optionally repositions the stream before writing, then writes a block of data through DOS. It stores the DOS error code and returns minus one on failure.
- `check_break_signal_bits`: Reads the task signal bits from the system library.
- `write_global_word_to_output_2`: Writes a four-byte global value to the standard output through DOS, then calls one other subroutine with the value one.
- `execute_cleanup_callbacks`: The routine iterates through a list of function pointers stored in a global structure, calling each one.
- `execute_cleanup_callbacks_2`: The routine iterates through a count stored in a global structure, calling a validation routine for each index.
- `empty_routine`: the routine performs no operations and returns immediately.
- `close_stream_by_index`: Checks a stream index, closes the DOS handle unless it is marked as a standard stream, and clears the slot.

### Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:bcdfq

- `main_program_entry`: Opens system libraries, initializes global variables, and enters a main loop that processes graphics, input, and hardware DMA channels, waiting for conditions before executing further routines.
- `check_fire_button_pressed`: Returns true when a bit of the cached port A status byte is clear, which means the fire button is pressed, and false otherwise.
- `check_hardware_status_bit`: Returns true when one bit of a hardware status byte at a fixed chip address is clear, and false otherwise.
- `wait_on_second_flag`: Returns at once when a first flag is clear.
- `update_graphics_object_properties`: Disables interrupts, waits for the top of frame, and iterates through a table of objects to decrement a counter and update a value via a pointer.
- `open_file_resource`: It returns true if the open operation failed.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `copy_second_region_to_start`: Copies forty-eight thousand bytes from a region that starts forty-eight thousand bytes into a buffer to the start of that same buffer.
- `clear_and_copy_block`: Calls a function pointer from a global structure, then copies 8000 words from a source buffer to a destination buffer.
- `execute_table_callbacks`: Iterates through a table of short values, calling a function pointer from a global structure for each non-zero entry.
- `read_entries_until_flag_2`: Calls the DOS read routine once for each entry in a table of values.
- `read_entries_until_flag_3`: Calls the DOS read routine once for each entry in a table of values.
- `read_block_from_file`: Calls the DOS read routine once through the handle stored in the block.
- `loop_decrement_nibbles`: The routine executes a countdown loop from fifteen to zero.
- `decrement_nibble_values_2`: Iterates through a list of words, decrementing each of the three nibble components if they are non-zero.
- `fade_palette_up_steps`: Runs sixteen steps, each waiting for two top-of-frame events, incrementing the color nibbles of a palette toward its target and loading the palette.
- `increment_mismatched_nibbles_2`: Iterates through two lists of words, incrementing the high-order nibble, middle nibble, or low-order nibble of the first list if that nibble does not match the corresponding nibble in the second list.
- `blit_multi_source_blocks`: Configures and executes multiple blitter operations using hardware registers.
- `select_audio_channel_mask`: Sets a global value to a one-bit mask for the given channel number, but only for channel numbers below four.
- `call_routine_sixteen_times`: Calls one subroutine sixteen times and then clears a global value.
- `start_audio_channel_sample`: Starts playback on an audio channel when its sample table entry is present.
- `store_channel_sample_fields`: Writes a sample address and a length into an audio channel record, using a fixed value when the length argument is zero.
- `set_channel_length_and_pointer`: Stores a length in an audio record, using a default when the length argument is zero, and updates the channel's pointer and count fields.
- `update_channel_pointer_fields`: Stores a length, a pointer offset derived from a count, and a second value into the channel record, updating the channel's count fields.
- `spin_forever_loop`: Loops indefinitely without an exit and never returns to its caller.
- `latch_channel_pointer_if_set`: When the low bit of the input is set and a channel's stored pointer is non-zero, copies that pointer and a related word into the channel record.
- `process_channel_updates`: When the update flag is set, calls four subroutines and then reads a channel entry.
- `advance_pattern_playback_tick`: Advances the playback tick for a pattern sequence: it steps the tick counter, reads each channel's pattern bytes, and applies note, volume and effect commands.
- `decode_pattern_row_for_channels`: Decodes one row of pattern data for each of four channels, updating each channel's sample, volume and effect fields and applying the effect commands.
- `continue_pattern_row_decode`: Handles the remaining per-channel pattern commands, including end-of-pattern and jump commands, and advances the position in the sequence table.
- `continue_pattern_decode_entry`: Handles the remaining per-channel pattern commands after the row setup and updates the sequence position.
- `decode_pattern_effect_commands`: Continues the per-channel pattern decode and applies the remaining effect codes, including the repeat, end-of-row and stop commands.
- `decode_pattern_channel_loop`: Loops over the channels of a pattern row, decoding each channel's note bytes and applying effect commands such as sample pointer, end-of-pattern and stop codes.
- `advance_pattern_channel_counter`: Handles the remaining per-channel pattern codes for one row, advancing a row counter on the zero code and applying volume and effect codes.
- `apply_channel_effect_code`: Applies one effect code from a pattern row to the current channel, setting its volume, speed, offset, sequence jump or slide fields.
- `apply_channel_volume_and_tempo`: Sets a channel's volume, tempo, panning and sequence jump fields from effect codes in a pattern row, scaling volume to a maximum of sixty-four.
- `apply_pattern_effect_fields`: Applies the tempo, panning, sequence jump, slide and volume effects of a pattern row to the channel record.
- `apply_pattern_row_effects`: Applies the panning, sequence jump, slide, end-of-row and volume effects of a pattern row to the channel record.
- `apply_pattern_jump_and_volume`: Applies the sequence jump, tempo, panning, slide and volume effects of a pattern row to the channel record.
- `apply_pattern_slide_effect`: Sets a channel's slide target from a pattern effect code and its offset, and updates the channel's effect fields.
- `apply_channel_slide_target`: Sets a channel's slide target from a period table when the channel index is below four.
- `process_channel_effect_tick`: Runs the per-tick effect step for one channel, applying slide, period offset and volume changes.
- `apply_bit_masks`: Performs several bitwise operations on memory locations pointed to by a0, a2, and a6, and branches to another routine.
- `increment_and_branch`: Adds a word from d4 to a word at offset 18 of a5, moves that word into d5, and branches to another routine.
- `calculate_index_and_offset`: Tests a byte in d4, and if non-zero, divides d3 by 3, swaps the nibbles of the result, and if the result is non-zero, performs a bitwise AND on d4 and adds a byte from a5 to d4 before branching.
- `decrement_and_shift`: Decrements d0 by one, and if it is not zero, branches; otherwise it shifts d4 right by four bits and adds a byte from a5 to d4 before branching.
- `fetch_table_value`: Calculates an index by adding a byte from a structure, a byte from another structure, and a byte from a pointer, then loads a word from a table at that index into d5 and branches.
- `update_byte_with_offset`: Moves a byte from d4 to d1, then moves a byte from offset 2 of a5 to d0; if the lower nibble of d1 is zero, it subtracts d4 from d0 and stores the result in offset 2 of a5, otherwise it stores zero in offset 2 of a5 and d1.
- `clamp_and_store_byte`: Adds the lower nibble of d1 to d0, and if d0 is less than or equal to sixty-four, stores d0 in offset 2 of a5; otherwise it sets d0 to sixty-four and stores it in offset 2 of a5 before branching.
- `subtract_and_branch`: Loads a word from a structure into d5, compares d3 to 3, and if d3 is less than 3, subtracts d4 from d5 before branching.
- `compare_and_increment_values`: Compares a word from offset 30 of a5 to a word from offset 18 of a5, and if the latter is higher, adds a byte from offset 70 of a5 to it and compares the result to the word from offset 30.
- `update_and_clear_value`: Subtracts d4 from d1 and compares the result to d0; if d1 is not higher than d0, it loads a word from a structure into d1, clears that structure location, and then stores d1 into both a structure offset and d5.
- `update_channel_data_2`: Checks if a 16-bit value in register d5 is non-zero, and if so, writes it to offset 6 of a structure pointed to by a value at offset 20 of another structure.
- `update_structure_state`: If d4 is non-zero, it stores d4 into a structure, then performs several arithmetic operations involving a table lookup and bit shifts to update values in a structure and a register before branching.
- `compare_audio_parameters_2`: Compares a byte in d4 to 241, and if it is not equal, compares a byte in d3 to 3, and if that is not equal, branches to another location.
- `compare_audio_parameters`: Compares a byte in d4 to 242, and if equal, compares a byte in d3 to 3; otherwise it branches to another location.
- `trigger_audio_sample`: Compares d4 to 243, and if equal, checks if bits 1 or 2 of d3 are set; if so, it pushes d3 to the stack, loads several bytes from a structure pointed to by a5, and calls start_audio_channel_sample using a pointer from offset 12 of that structure.
- `clear_cia_bit`: Compares d4 to 248 and 249, and if d4 is 249, clears bit 1 of the CIAA Port A Data Register before branching.
- `set_cia_bit`: Sets bit 1 of the CIAA Port A Data Register and branches.
- `update_channel_data`: Checks a word in d5, and if non-zero, writes it to an offset in a structure, writes a byte from a structure to another offset, increments d7, and calls another routine if d7 is below a constant value.
- `compute_period_from_note_3`: Converts an input value into a period in the same way as the routines above, storing the low and high bytes of the result through two pointers.
- `store_global_and_return_offset`: Stores the address of a global variable into a slot in a structure in a5 and returns the sum of the value pointed to by a0 and the value of a0 plus six.
- `add_offset_to_nonzero_long`: Adds the value in the data register to a long word at the address passed in an address register, but only when that long word is non-zero.
- `adjust_pointer_offsets`: Runs a helper routine in a loop until it reports completion, then walks an array of pointers adding a base address to each entry when a specific field is negative.
- `scale_palette_from_source`: The routine reads a scaling byte from a structure pointed to by the argument and multiplies a 16-byte source table by that byte, storing the scaled 16-bit results into a global variable table.
- `start_module_playback_if_ready`: Returns a fixed failure code when an initial status value is non-zero.
- `check_ciab_and_call_handler`: When a global flag is set it calls one subroutine, then always calls a second subroutine and returns.
- `clear_flag_bit_if_active`: When a global flag is set, clears the low bit of a byte through a pointer and calls one subroutine, returning its result.
- `open_cia_resource_and_map`: Opens the CIA resource through the system and sets register addresses for the timer and port hardware, depending on which CIA chip responds.
- `close_ciab_resource`: The routine checks if a global resource base pointer is non-zero.

### Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:bcdft

- `initialize_program_environment`: Performs a sequence of subroutine calls and frees memory using a system library.
- `decrement_to_minus_one`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.
- `unpack_and_populate_data_table`: Loads a table header from memory to determine offsets and counts, then iterates through a loop to call a routine and store bytes into a destination buffer.
- `execute_conditional_calls`: Sets d4 to 14 and d1 to 5, calls another routine, and if the carry flag is set, sets d1 to 5, calls another routine, adds d0 to d4, and branches.
- `branch_and_initialize_registers`: Calls another routine and, if the carry flag is set, branches to a different location; otherwise, it loads the value one into registers d1 and d4 and branches to another location.
- `branch_on_carry_and_load`: Calls another routine, and if the carry flag is set, branches to another location; otherwise it loads 2 into d1, 3 into d4, and branches to another location.
- `branch_on_carry`: Calls another routine, and if the carry flag is set, branches to another location; otherwise it loads 4 into d1, 7 into d4, and branches to another location.
- `validate_and_branch_on_index`: Sets d4 to 23 and d1 to 8, calls another routine, and adds the result in d0 to d4.
- `call_and_initialize_registers`: Calls another routine, and if the carry flag is not set, sets d1 to five and d2 to zero before branching.
- `copy_table_data`: Sets d1 to 14 and d2 to 544, calls another routine, adds d2 to d0, loads a pointer into a3 from a table, and then enters a loop that moves bytes from the table to a destination while decrementing d4.
- `shift_and_swap_d6`: Decrements d7 and, if not zero, performs a series of operations including shifting d6 right, swapping d6, loading a new word from a pointer in a2 into d6, and swapping d6 again before returning.
- `tiny_lsr_rts`: Small routine of 2 instructions that shifts d6 right by 1, then returns.
- `lookup_table_masked_value`: The routine reads a 16-bit value from a table using an index derived from the input word, and returns that value masked with an input mask.
- `process_nested_data_tables`: Iterates through a table in a1 using a count from d5, copying data from a5 to a1 and calling a routine in a4, then enters a second loop that processes a list of offsets from a5 to add values to a table at a1.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d3 to d6, then returns.

### Black Crypt (1992)(Electronic Arts)[cr TRSi](Disk 1 of 3)[a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_complex_sequence`: Performs a series of conditional calls to various function pointers and library functions, updates several global variables, and eventually calls a library function to set a value at a specific memory location.
- `wait_for_library_call_3`: Repeatedly calls function pointers from a global structure until a second function pointer returns a non-zero value.
- `initialize_library_call_state_three_2`: Sets a global state variable to three and stores a parameter in a global variable.
- `setup_and_execute_library_calls`: Configures library call parameters, executes multiple library functions, and handles conditional execution based on return values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_directory_structure`: Clears a memory block of 493 words starting at a specific offset, sets up global pointers to system structures, checks for supervisor mode bit, and opens a library with error handling that may trigger an alert message.
- `frag_lea_rts`: Short code fragment that loads the address of a fixed memory address into a4, then returns.
- `initialize_file_system_structure`: Initializes file system data structures, sets up memory pointers, and manages library calls for directory operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_graphics_library`: Opens graphics-related libraries using specific version strings and stores the returned library pointers in a global structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_audio_library`: Opens audio-related libraries with version checking and stores the library handles in a global data structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_input_library`: Opens input device libraries and stores the library pointers in a global structure, then performs additional setup operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_network_library`: Opens network-related libraries with version information and stores the library handles in a global data structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_display_library`: Opens display-related libraries using specific strings, stores the library pointers, and performs additional initialization operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_library_handles`: Loads library handles by opening specific libraries and storing the results in global variables, with a conditional check based on a structure offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_library_pointers`: Initializes library pointers by calling OpenLibrary with various string arguments and storing the results in a global data structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_exec_library`: Opens the Exec library using a function pointer from the system base and stores the returned library handle.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_library_handles`: Calls a function through a pointer at offset -552 of a register, stores the result, and then uses OpenLibrary to load additional libraries.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_library_handles`: Initializes library handles by opening specific libraries and storing their pointers in a data structure, then performs several operations including setting up global variables and calling routines based on library function results.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_library_handles_2`: Initializes library handles by opening two libraries and storing their pointers at offsets 1 and 2 of a structure pointed to by a5, then loads several values from a structure at offset 38 of the same structure into offsets 4, 5, 6, and 3 of that structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_library`: Opens a library using a global pointer, stores the returned library handle, and updates several structure fields with values from the library's interface.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_exec_library_2`: Opens the Exec library and stores its base address in a structure, then performs several memory management operations including setting up DMA control and calling other routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_library_2`: Opens a library using a global pointer, stores the returned library handle into a structure at offset 34 of the structure in a6, and then calls functions from the library to set up system variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_wide_data`: Loads wide graphics data from a table into global variables, then jumps to another routine using a pointer from the global structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_display_list`: Updates graphics display list entries using values from a structure, then performs memory allocation and hardware register setup before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_jsr_move_movea`: Short code fragment that calls another routine, then copies d0 to the memory at an offset from a5, then copies the memory at an offset from a5 to a4, then copies the memory at an offset from a4 to the memory at an offset from a5 (4 times in a row), then copies the memory at an offset from a5 to a0, then copies the memory at an offset from a5 to a6, then calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_graphics`: Sets up graphics-related data structures by copying values from one memory location to another and calling library functions to initialize display parameters.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_dma_channels`: Configures DMA channel settings by loading values from a specific memory area and writing a control word to the DMA control register to enable channels.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `handle_display_update`: Updates display parameters by copying data from a global structure and calling hardware initialization routines, then conditionally executes additional setup functions based on a status check.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_video_dma`: Configures video DMA settings by copying memory values, calling display initialization functions, and executing conditional setup routines based on hardware status checks.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_status_byte`: Stores a value from the D0 register to a memory location pointed to by A0, then calls library functions to update display parameters and conditionally executes another routine based on a global flag.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_video_output`: Sets up video output parameters by copying data values between memory locations, calling initialization routines, and performing conditional operations based on hardware status checks.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_dma_control`: Sets the DMA control register to enable bit 5, then performs several library function calls and updates values in a structure pointed to by a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_dma_enable`: Clears bit 15 of the DMACON register to disable DMA channels, then calls a routine that handles audio processing.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_audio_data`: Updates audio buffer pointers, calls multiple audio processing routines, and manages DMA channel enabling based on conditional checks.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_dma_and_audio`: Enables DMA channels, performs audio processing checks, and updates global audio state variables before calling final processing functions.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_audio_buffer`: Stores a value at offset 6 of a memory location, performs conditional audio processing based on system state, and calls audio processing functions.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `handle_game_logic`: Checks game state conditions, calls various game logic routines, updates global flags, and manages execution flow based on multiple conditional checks.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move_jsr_addq_movem_rts`: Short code fragment that copies the memory at an offset from a4 to the stack (2 times in a row), then calls another routine, then adds 8 to a7, then copies d0 to the stack, then calls another routine, then adds 4 to a7, then restores saved registers from the stack, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_game_state`: Updates game state variables, performs conditional checks on game flags, calls state management functions, and stores results to a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move_jsr_addq_movem_rts_2`: Short code fragment that copies the memory at an offset from a4 to the stack, then calls another routine, then adds 8 to a7, then copies d0 to the stack, then calls another routine, then adds 4 to a7, then restores saved registers from the stack, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `manage_display_update`: Sets display update flags, calls rendering functions, and manages screen refresh operations based on system conditions.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_jsr_addq_move_movem_rts`: Short code fragment that calls another routine, then adds 8 to a7, then copies d0 to the stack, then calls another routine, then adds 4 to a7, then restores saved registers from the stack, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_collision_status`: Performs collision detection checks, calls various game object routines, and returns a boolean value indicating collision status.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_components`: Initializes system components by calling multiple setup routines, manages memory allocation, and handles error conditions during initialization.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_moveq_btst_bne_rts_tst_beq`: Short code fragment that loads 0 into d0, then tests bit 2 of the memory at an offset from a0, then branches if the last result was not zero, then loads 1 into d0, then returns, then tests the memory at an offset from a5, then branches if the last result was zero, then tests the memory at an offset from a5, then branches if the last result was zero, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move_movea_jsr_clr_bra`: Short code fragment that copies a6 to the stack, then copies the memory at an offset from a4 to a6, then copies the memory at an offset from a4 to a1, then calls another routine, then copies the top of the stack to a6, then clears the memory at an offset from a4, then jumps to another point in the code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_file_resource_2`: Opens the file named by a text reference through DOS, stores the returned handle in a structure field, and returns true when the open failed.
- `clear_and_copy_block_2`: Calls the graphics block-clear routine through a pointer, then copies eight thousand longwords from a block offset by twelve thousand longwords into the buffer's start.
- `read_table_entries_until_zero`: Walks a table of nonzero short entries, calling a library routine through a pointer for each one.
- `read_table_entries_until_zero_2`: Walks a table of nonzero short entries, calling a library routine through a pointer for each one.
- `fade_palette_up_steps_2`: Runs sixteen steps, each waiting for two top-of-frame events, incrementing the color nibbles of a palette toward its target and loading the palette.
- `increment_nibble_values`: Walks two word lists in step, and for each word increments each of its three 4-bit fields in the first list wherever that field differs from the corresponding field in the second list.
- `blit_multi_source_blocks_2`: Takes exclusive use of the blitter, then for sixteen steps waits for frame timing and programs the blitter with source and destination addresses, modulo values and sizes, waiting for each block to finish.
- `lookup_or_divide_table_entry`: Checks if the input index is within bounds to read a 16-bit value from a global table; otherwise, it divides a global constant by the input.
- `add_to_pointer_if_nonzero`: Reads the long at the address in a0; if it is non-zero, adds d1 to it and stores the result back through a0.
- `fill_stack_with_addresses`: Fills a stack area with return addresses in a loop until the stack is completely filled, performing no other operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_cleanup_callbacks_3`: The routine iterates through a list of function pointers stored in a global structure, calling each one.
- `frag_movem_move_tst_beq`: Short code fragment that saves registers on the stack, then copies the memory at an offset from a7 to d3, then tests the memory at an offset from a4, then branches if the last result was zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_input_until_zero`: Repeatedly calls a function to process input values until it returns zero, then checks a global condition and calls the function twice if true.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_tst_beq_movea_jsr_move_addq_rts`: Short code fragment that tests the memory at an offset from a4, then branches if the last result was zero, then copies the memory at an offset from a4 to a0, then calls the routine whose address is in a0, then copies the memory at an offset from a7 to the stack, then calls another routine, then adds 4 to a7, then returns.
- `frag_movem_move_tst_beq_moveq_bra`: Short code fragment that saves registers on the stack, then copies the memory at an offset from a7 to d3, then tests the memory at an offset from a4, then branches if the last result was zero, then loads 0 into d2, then jumps to another point in the code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_sprite_positions`: Adjusts sprite coordinates based on velocity values and updates their positions in a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_beq_movea_jsr_move_addq_rts`: Short code fragment that branches if the last result was zero, then copies the memory at an offset from a4 to a0, then calls the routine whose address is in a0, then copies the memory at an offset from a7 to the stack, then calls another routine, then adds 4 to a7, then returns.
- `frag_movem_move_tst_beq_moveq_bra`: Short code fragment that saves registers on the stack, then copies the memory at an offset from a7 to d3, then tests the memory at an offset from a4, then branches if the last result was zero, then loads 0 into d2, then jumps to another point in the code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_input_loop`: Reads input states in a loop until a zero value is returned, then performs additional input processing and returns a value based on a global flag.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_and_process`: Processes a series of updates by calling functions with loop indices, then performs additional processing based on global conditions and modifies data through pointer arithmetic.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_and_process`: Decrements a value and returns the result, or calls functions to process input when the decremented value equals zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_updates_and_stack`: Processes updates by calling functions with loop indices, then performs additional processing based on global conditions and manages stack operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_input_completion`: Waits for input completion by repeatedly calling a function until it returns zero, then processes additional input based on global conditions.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_updates_and_stack`: Processes updates by calling functions with loop indices, then performs additional processing based on global conditions and manages stack operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_input_until_zero_2`: Reads values from a function in a loop until zero is returned, then conditionally reads and returns another value based on a global variable's state.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_input_queue`: Reads input events from a queue and processes them until the queue is empty, updating a counter variable during the loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_and_call`: Loops while a global variable is non-zero, calling another routine each iteration, then conditionally calls the routine twice based on a structure field's value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_indirect_routines`: Calls routines indirectly through function pointers stored at specific offsets in a structure, with conditional execution based on the values of those pointers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `scale_and_store_values`: Performs fixed-point multiplication of values from a buffer with a scalar, storing the results at specific memory locations, and processes exactly 15 iterations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_color_tables`: Processes color data from multiple sources, applies scaling operations, and updates table entries with computed values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `scale_color_components`: Processes color scaling values from multiple sources and applies them to a table of color data, updating component values based on multiplication and bit shifting operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_keyboard_buffer`: Clears a 64-byte keyboard buffer by setting each byte to zero, or copies data from a source to the buffer based on a counter condition.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `scale_colors`: Processes color data by scaling values and updating a table, then performs conditional data copying based on structure fields.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_audio_buffer_2`: Processes audio data by multiplying a value with a register content, shifting right by four bits, and storing the result at an offset in a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_audio_buffer`: Clears a buffer of 62 words by setting each word to zero, starting at a specific memory address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `handle_audio_callbacks`: Executes callback routines conditionally based on flags in a global table, storing and restoring a return address during execution.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move_adda_dbra_bra_movem_rts`: Short code fragment that copies the memory at an offset from a0 to the memory at an offset from a2, then copies the memory at a0 to the memory at a2 (advancing a2), then adds d0 to a0, then loops back while counting down d2, then jumps to another point in the code, then restores saved registers from the stack, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_audio_state`: Calls audio-related routines based on flags in a global table, then clears a 62-word buffer with zeros before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `process_audio_fragment`: Contains a loop that executes a series of operations including conditional branches, routine calls, and memory address loading.
- `frag_move_beq_bsr_rts_movea`: Short code fragment that copies a location in the code to d0, then branches if the last result was zero, then calls another routine (2 times in a row), then returns, then copies a0 to the stack, then calls another routine, then copies the top of the stack to a0, then copies a location in the code to d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_supervisor_mode`: Checks if the supervisor mode bit is set in a global variable, and if so, calls a routine that may be related to system-level operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_display_flag`: Updates a display flag byte and clears bit 1 of a status byte, then performs conditional operations based on global variables and calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_and_execute`: Evaluates conditions based on values in a structured data block and executes function pointers from a table, performing different code sequences depending on the evaluation results.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_audio_status`: Updates audio-related global variables and status bytes based on conditional checks, then sets bits in a hardware register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_jsr_bra`: Short code fragment that calls another routine, then jumps to another point in the code.
- `initialize_display`: Initializes display-related variables and sets up hardware state based on a global structure's flags, then calls a rendering function and updates a status byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `handle_input_event`: Processes input events by updating global variables, modifying CIAA_PRA register bits based on structure flags, setting up global state variables, and calling a function with a pointer value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_jsr_movea_bra`: Short code fragment that calls another routine, then copies the top of the stack to a6, then jumps to another point in the code.
- `setup_audio_subsystem`: Sets up audio subsystem by calling three library functions with different offsets from a4 register, then returns without modifying any global state.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_game_state_2`: Updates game state variables by copying byte values from an input pointer to global variables, setting up a global variable with stack data, calling a function, and updating a bit in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_jsr_movea_bra`: Short code fragment that calls another routine, then copies the top of the stack to a6, then jumps to another point in the code.
- `load_library_data`: Loads library data by calling three functions with specific offsets from global variables, then restores the stack and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_moveq_move_lea_bsr_bset_rts`: Short code fragment that loads 0 into d1, then copies the memory at an offset from a0 to d1, then loads the address of a location in the code into a0, then copies d1 to the memory at a0 (advancing a0), then loads 2 into d1, then copies d1 to the memory at a0, then copies the top of the stack to a fixed memory address, then calls another routine, then sets bit 0 of the memory at a1, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_rendering_context`: Sets up rendering context by calling two library functions with different offsets from a4 register, then returns without modifying any global state.
- `function_epilogue_3`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_lea_move_moveq_bsr_bset_rts`: Short code fragment that loads the address of a location in the code into a0, then copies d1 to the memory at a0 (advancing a0), then loads 2 into d1, then copies d1 to the memory at a0, then copies the top of the stack to a fixed memory address, then calls another routine, then sets bit 0 of the memory at a1, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `function_epilogue_4`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_bsr_bset_rts`: Short code fragment that calls another routine, then sets bit 0 of the memory at a1, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move_movea_rts`: Short code fragment that copies d3 to d0, then copies the memory at an offset from a4 to a7, then returns.
- `function_epilogue_5`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `validate_and_process_index`: Validates an index against bounds and checks if a table entry is non-zero, updates a global variable with a constant value, and returns an error code or calls another function.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_resource_file`: Calls the exec.OpenResource library function to open a resource file and returns immediately.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `validate_and_process_entry`: Validates an index against bounds and checks if a table entry is non-zero, calls another function with parameters, and returns an error code or value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_resource`: Attempts to open a resource and initializes global pointers if successful, otherwise sets an error code and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_timer_state`: Updates a timer state variable based on interrupt timing and checks conditions before modifying a table entry.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_input_event`: Handles input event processing by checking a global flag, calling an interrupt handler, and managing timer register updates based on interrupt timing.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `validate_and_clear_entry`: Validates array bounds and entry state before clearing a table entry, returning an error code if validation fails or success code upon completion.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_and_clear_entry`: Updates a counter and sets a memory location based on a condition flag, then clears a table entry if the counter is valid and the entry is not zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_table_entry`: Clears a table entry after validating bounds and non-zero state, returning an error code if validation fails or success code upon completion.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_timer_pointers`: Initializes timer register pointers and stores the return value from a library call into a global variable when the return value is non-zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `function_epilogue_6`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `conditional_execute`: Performs a conditional check on a word value, executes a function call if the condition is met, clears a memory location, and returns zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_timer_pointers_2`: Sets up pointers to hardware timer control and register addresses in global variables, stores a value from a register, and calls a library function through a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `function_epilogue_7`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `conditional_execute`: Performs a conditional check on a word value, executes a function call if the condition is met, clears a memory location, and returns zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_timer_interrupts`: Configures timer interrupt settings by calling library functions multiple times, updating register pointers based on interrupt conditions, and managing loop control.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_bne_move_movea_jsr_clr_moveq_bra`: Short code fragment that branches if the last result was not zero, then copies a6 to the stack, then copies the memory at an offset from a4 to a6, then copies the memory at a2 to d1, then calls another routine, then copies the top of the stack to a6, then clears the memory at a2, then loads 0 into d0, then jumps to another point in the code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_timer_registers`: Initializes timer register pointers in a table and sets up a global variable to zero before entering an infinite loop that calls a routine based on a conditional offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_movea_move_jsr_clr_moveq_bra`: Short code fragment that copies the memory at an offset from a4 to a6, then copies the memory at a2 to d1, then calls another routine, then copies the top of the stack to a6, then clears the memory at a2, then loads 0 into d0, then jumps to another point in the code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_timer_registers_2`: Sets timer register pointers in a table and initializes a global variable to zero before entering an infinite loop that calls a routine based on a conditional offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_jsr_movea_clr_moveq_bra`: Short code fragment that calls another routine, then copies the top of the stack to a6, then clears the memory at a2, then loads 0 into d0, then jumps to another point in the code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_timer_registers`: Sets up timer registers based on library call results and enters an infinite loop that clears a memory location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_clr_moveq_bra`: Short code fragment that clears the memory at a2, then loads 0 into d0, then jumps to another point in the code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_timer_state`: Initializes timer state variables and sets up register values based on library call outcomes, then returns.
- `close_ciab_resource_2`: If a stored resource base is set, clears that global and calls a library function through the base, returning its result.
