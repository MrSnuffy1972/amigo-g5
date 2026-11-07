# Game 204 (Sensible World of Moon Soccer): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00204). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (937488 top-level bytes in 189 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 139 of 139 units identical (bytes and relocations), 22941 raw-word lines |
| Functions named and described | pass, 330 of 330 named (0 author symbols), 330 of 330 described (32 by rule, 294 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1127 of 1134 (99.38%) |
| Recovered without load address (counted in proof 1, flagged) | 700500 bytes, 4 functions |
| Independent decoder cross-check (Ghidra against capstone) | 28706 of 28732 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 21814 |
| code | 20866 |
| data | 562895 |
| padding | 10377 |
| unused | 321536 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Sensible World of Moon Soccer.adf:SSBOOT | 1 | 551 | 28 |
| Sensible World of Moon Soccer.adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 43 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 68 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 69 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 35 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 42 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 37 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 29 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 96 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 33 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 32 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3326 | 147 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 38 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| Sensible World of Moon Soccer.adf:SWOS2.REL#unpacked (raw code file, load address unknown) | 1 | 11417 | 1 |
| Sensible World of Moon Soccer.adf:SWOS#unpacked (raw code file, load address unknown) | 1 | 1 | 1 |
| Sensible World of Moon Soccer.adf:SWOS2#unpacked (raw code file, load address unknown) | 1 | 1 | 1 |
| Sensible World of Moon Soccer.adf:SWOS.REL#unpacked (raw code file, load address unknown) | 1 | 11416 | 1 |

## Cases handled

- Sensible World of Moon Soccer/Sensible World of Moon Soccer.adf:SWOS2.REL: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (29754 -> 45668 bytes)
- Sensible World of Moon Soccer/Sensible World of Moon Soccer.adf:SWOS: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (135957 -> 304346 bytes)
- Sensible World of Moon Soccer/Sensible World of Moon Soccer.adf:grafs/CJCTEAM3.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6251 -> 40960 bytes)
- Sensible World of Moon Soccer/Sensible World of Moon Soccer.adf:grafs/FEATS1.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (14347 -> 47872 bytes)
- Sensible World of Moon Soccer/Sensible World of Moon Soccer.adf:grafs/CJCTEAM1.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6338 -> 40960 bytes)
- Sensible World of Moon Soccer/Sensible World of Moon Soccer.adf:grafs/MENUBG.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7999 -> 47872 bytes)
- Sensible World of Moon Soccer/Sensible World of Moon Soccer.adf:grafs/CJCGRAFS.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3853 -> 40960 bytes)
- Sensible World of Moon Soccer/Sensible World of Moon Soccer.adf:grafs/CJCBITS.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7088 -> 40960 bytes)
- Sensible World of Moon Soccer/Sensible World of Moon Soccer.adf:grafs/SWCPITCH.MAP: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (9327 -> 37272 bytes)
- Sensible World of Moon Soccer/Sensible World of Moon Soccer.adf:grafs/CJCBENCH.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4593 -> 40960 bytes)
- Sensible World of Moon Soccer/Sensible World of Moon Soccer.adf:grafs/CHARSET.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2080 -> 40960 bytes)
- Sensible World of Moon Soccer/Sensible World of Moon Soccer.adf:grafs/FEATS2.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (14017 -> 47872 bytes)
- Sensible World of Moon Soccer/Sensible World of Moon Soccer.adf:grafs/CJCTEAM2.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6158 -> 40960 bytes)
- Sensible World of Moon Soccer/Sensible World of Moon Soccer.adf:grafs/CJCTEAMG.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4608 -> 40960 bytes)
- Sensible World of Moon Soccer/Sensible World of Moon Soccer.adf:grafs/TACTICS.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7846 -> 47872 bytes)
- Sensible World of Moon Soccer/Sensible World of Moon Soccer.adf:grafs/MENUS.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4284 -> 40960 bytes)
- Sensible World of Moon Soccer/Sensible World of Moon Soccer.adf:sound/MISS3C: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12713 -> 12788 bytes)
- Sensible World of Moon Soccer/Sensible World of Moon Soccer.adf:sound/MENMUSIC.MOD: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (35504 -> 47868 bytes)
- Sensible World of Moon Soccer/Sensible World of Moon Soccer.adf:data/TEAM.008: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (17913 -> 31466 bytes)
- Sensible World of Moon Soccer/Sensible World of Moon Soccer.adf:data/TEAM.020: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (16992 -> 28730 bytes)
- Sensible World of Moon Soccer/Sensible World of Moon Soccer.adf:data/TEAM.033: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7705 -> 13682 bytes)
- Sensible World of Moon Soccer/Sensible World of Moon Soccer.adf:SWOS2: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (136144 -> 304822 bytes)
- Sensible World of Moon Soccer/Sensible World of Moon Soccer.adf:SWOS.REL: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (29744 -> 45664 bytes)
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 13 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 25 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 11 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 13 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 31 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 48 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 13 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 32 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 14 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 10 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 76 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 11 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 13 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 15 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 573 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 30 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Sensible World of Moon Soccer/Sensible World of Moon Soccer.adf:SWOS2.REL#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Sensible World of Moon Soccer/Sensible World of Moon Soccer.adf:SWOS#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Sensible World of Moon Soccer/Sensible World of Moon Soccer.adf:SWOS2#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Sensible World of Moon Soccer/Sensible World of Moon Soccer.adf:SWOS.REL#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- 2 never-loaded region(s), 609168 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `clear_cia_port_bit` (runtime segment an address (play-rs-m1500)): It derives a bit index from a register and applies it to a global variable, clearing the associated CIA port bit. Kind: disk hardware/status check. Pass path: the branch following the call that continues normal execution (likely the loop/loading logic).
- `update_hardware_and_loop` (runtime segment an address (play-rs-m1500)): Sets a global variable to various values and updates a memory location using a pointer from a structure. Kind: disk track/hardware validation loop. Pass path: the loop continues (sVar2 decrementing) as long as the check returns false/success, and the routine proceeds to u132__sub_1492() when the loop terminates normally.
- `configure_dma_transfer` (runtime segment an address (play-rs-m1500)): It writes several values to hardware registers and global variables to set up a DMA transfer, including setting a source address and a control word. Kind: disk hardware/track validation. Pass path: the branch following the call that does not result in an error/halt, specifically the 'bne' (branch if not equal) failing to trigger or the subsequent instruction flow in sub_0E28/sub_1014.
- `poll_hardware_bit` (runtime segment an address (play-rs-m1500)): Repeatedly calls another routine and checks a bit in a structure until a specific bit is set, then writes three bytes to a global variable. Kind: disk hardware status check. Pass path: the loop continues (bVar2 is true).
- `modify_adkcon_bit` (runtime segment an address (play-rs-m1500)): Writes a word to a hardware register and performs a conditional branch based on a value in a global variable. Kind: disk hardware/ADKCON check. Pass path: beq.b an address.
- `modify_port_bit` (runtime segment an address (play-rs-m1500)): It calculates a bit position based on a value in a table and modifies a specific bit in a hardware port register using a provided byte. Kind: bit manipulation/checksum trap. Pass path: the branch following the call where the bit is correctly set/cleared as expected by the subsequent logic.
- `wait_for_bit_clear` (runtime segment an address (play-rs-m1500)): Sets a global variable and repeatedly calls another routine until a bit in that global variable is cleared. Kind: disk hardware/sync check. Pass path: the loop continues (sVar1 != 1) or the bit clears, allowing execution to proceed to the next instruction.
- `modify_and_wait_bit` (runtime segment an address (play-rs-m1500)): Modifies a global variable based on a returned value and then waits for a specific bit in that variable to be set. Kind: disk hardware/sync check. Pass path: sVar1 == 1.
- `update_global_variable` (runtime segment an address (play-rs-m1500)): Calls another routine and stores its return value into a global variable. Kind: disk hardware/integrity check. Pass path: The routine updates a global variable (likely a status flag or checksum result) based on the return value of a hardware-interfacing sub-routine. A successful check (passing) would result in the global variable being set to a 'valid' state, .
- `update_bitmask_from_table` (runtime segment an address (play-rs-m1500)): Calculates a bitmask by checking a bit in a table based on a value and a global variable, then returns the result. Kind: bitmask lookup/validation. Pass path: the branch where the bitmask remains unchanged or the condition `(*(ushort *)(&u132__d0_14CC + sVar1) & 1) != 0` evaluates to false.

## Functions


### Sensible World of Moon Soccer.adf:SSBOOT

- `unpack_and_initialize_engine`: Decodes a compressed data payload and performs memory management tasks including scanning memory types and allocating space.
- `decrypt_and_execute_payload`: Decodes a block of data by applying a transformation to a table of function pointers and then executes the resulting code.
- `copy_memory_block`: Copies a block of bytes from a source pointer to a destination pointer using a counter.
- `copy_memory_decrementing`: Copies a block of bytes from a source pointer to a destination pointer by decrementing the pointers and the counter.
- `call_library_functions`: Obtains a library base address from a global variable and calls two functions at specific offsets from that base.
- `scan_memory_types`: Iterates through memory segments to check their type and stores bitmask results in global variables.
- `load_compressed_data`: Opens a file, allocates memory, and processes a compressed data stream by decoding chunks and copying memory.
- `decode_compressed_chunk`: Validates a data header signature and processes a compressed data stream using a stack-based algorithm.
- `parse_bit_table`: Scans a table of words for a matching condition, then calculates a bit mask based on byte values and register contents.
- `mask_and_conditional_call`: The routine masks a word from a register with a word from a register and, if a byte from a register is less than a byte from a register, it calls another routine and uses its returned word as the result; otherwise it returns the masked word.
- `return_empty_routine`: The routine is empty; it contains no instructions that modify memory or registers and immediately returns.
- `accumulate_32bit_from_bytes`: The routine reads four consecutive bytes from a pointer in a register and builds a 32-bit value in a register by concatenating each byte into the result.
- `build_huffman_tree_nodes`: Iterates over a buffer to match bytes against a running value, writing masks, shifted values, and node properties into a destination array to build a tree structure.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_8`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_9`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_10`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_11`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_12`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_13`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_14`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_15`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Sensible World of Moon Soccer.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_string_and_fill_memory`: Copies a sequence of bytes from a source pointer to a destination buffer until a newline character is encountered or a counter reaches zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_system_vector`: Stores a memory address into a system vector and performs a signed division on a register.
- `initialize_memory_blocks`: Calls another routine, then fills two separate memory ranges with repeating word patterns using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_string_and_fill_table`: Copies a string of characters to a destination until a newline or length limit is reached, then fills a memory range with a specific word value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_parameters`: Checks a pointer against a constant, writes a word to a specific memory location, and stores several values from registers into global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_string_and_fill_memory_2`: Copies a sequence of bytes from one location to another and then fills a memory range with a specific word value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_word_to_memory_loop`: Writes a word to a memory location and increments the pointer, repeating until a specific address is reached.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_string_with_logic_2`: Copies characters from one location to another, potentially modifying a character and a global variable based on newline detection.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_memory_and_registers`: Writes a constant value to a memory location via a loop, stores several constants into specific memory addresses, and clears a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_string_with_logic`: Copies a string from a source to a destination until a newline character is encountered.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_memory_and_registers`: Writes a constant value to a memory location via a loop, stores several constants into specific memory addresses, and clears a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `construct_relocatable_filename`: Manipulates a string by stripping newline characters and appending a relative file extension.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_move`: Small routine of 1 instruction that copies 0 to d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_relocatable_library`: Copies a string from memory, appends a suffix, and calls a function pointer to open a library.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `disable_interrupts_and_clear_color`: Disables interrupts and clears the primary color register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_library_and_clear_memory`: Attempts to open a library by constructing a filename string and appending a specific extension.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_dma_control_tables`: Initializes three memory tables with repeating word patterns using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_relocatable_resource_loading`: Attempts to open a file with a specific extension and, upon failure, performs a memory fill with a pattern to clear specific buffers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_memory_and_enable_dma`: Fills three consecutive memory locations with a specific word pattern using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_library_and_clear_memory_2`: Opens a library and, if successful, performs a loop to fill specific memory ranges with a constant pattern.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_memory_and_configure_hardware`: Fills three memory locations pointed to by registers with a constant value using a loop, then writes values to several hardware registers and global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_library_and_modify_data`: Opens a library via a function pointer, performs a loop to write a constant to two memory locations, and then enters an infinite loop that performs bitwise OR operations on specific memory addresses.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_hardware_and_call_subroutine`: Decrements a counter in a loop, writes values to several hardware registers and global variables, and then calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_and_store_registers`: Calls a routine via a function pointer, stores the result in a global variable, and then performs two separate calls to another routine while saving register values to global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_hardware_and_call_routine`: Writes values to several hardware registers and global variables, calls another routine, and then modifies a hardware register and a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine_twice_and_load_pointer`: Loads an address into a register, then performs two separate calls to another routine while saving register values to global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_interrupt_parameters`: Updates three global pointers, calls another routine, writes a byte to a global variable, and modifies the hardware interrupt enable register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `close_library_and_call_routine`: Calls a routine twice while saving register values to global variables, then uses a pointer from a global variable to call a library function to close a library.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_hardware_interrupts`: Stores a value in a CIA register, calls another routine, and then updates CIA and bitplane control registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `release_library_resources`: Loads a pointer into a register, calls a subroutine, and then uses a library function to close a library and another to set the processor status register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_interrupt_settings`: Calls another routine and performs multiple writes to a global variable and hardware registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_display_parameters`: Modifies bitplane and vertical scroll registers before calling a series of subroutines and entering an infinite loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_hardware_registers`: Writes specific values to a global variable and several hardware registers, then calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_bitplane_registers`: Calls a library function, performs a second library call, and then clears a hardware register and masks a value from a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_global_variable`: Performs bitwise operations on a global variable and calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_bitmask_loop`: Calls library functions, clears hardware registers, and enters a loop that iterates through bits of a value to perform conditional writes to memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_bit_flag`: Performs a bitwise OR on a global variable and calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_hardware_interrupt_state`: Iterates through a bitmask to perform conditional memory writes and executes a function call.
- `iterate_bit_mask`: Iterates through a bitmask using a loop to check bits and performs memory writes and a function call if specific conditions are met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_offset_function`: Stores a word at a pointer location, calls a function from a table using an offset, and sets a global variable to a specific value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_block`: Fills a specified range of memory with zeros using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_indirect_address`: Calls a function at an address calculated by adding two values and sets a global variable to a specific value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_and_branch`: Decrements a register and branches based on the result or performs an unconditional branch.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_subroutine_4`: This routine performs a single return instruction to exit the current function.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `add_bytes_to_buffer`: Iterates through a list of pointers to add a byte value derived from a register to the memory locations pointed to.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `save_registers_to_table`: Performs an unconditional branch and then saves several registers into a table using PC-relative addressing.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `add_high_byte_to_buffer`: Iterates through a list of pointers to add the high byte of a register to the memory locations pointed to.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `branch_to_offset`: Performs a short relative branch to a specific offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `add_bytes_to_memory`: Adds a value to a series of bytes pointed to by a table and performs a bitwise XOR operation on a byte at a calculated address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_to_offset_2`: Performs a short relative branch to a specific location within the code segment.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_memory_block`: Iterates through a memory range, adding a byte value to each byte at the destination address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_state_and_add_bytes_2`: Stores several values into a table, adds a value to a series of bytes in memory, and updates a pointer and a byte value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_memory_block`: Iterates through a memory range, adding a byte value to each byte at the destination address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_values_to_table`: Stores four values and a global variable into a table, followed by several zeroed entries.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_jump_table_pointers`: Sets two pointers to a specific address based on a comparison and clears several entries in a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_registers_to_memory`: Stores the contents of several registers into memory locations and clears three consecutive longwords.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_execute_subroutine`: Copies a block of data from a source table to a destination area on the stack and then executes the code at that stack location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_and_clear_pointers`: Loads two values from memory into registers using PC-relative addresses and updates a table by storing a global pointer followed by two zeroed words.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_function_pointers`: Copies a block of data from a source location to a destination determined by a global variable and then executes the code at that destination.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_block_2`: Stores a longword from a register to a pointer, then clears several consecutive longword locations before branching.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_execute_data`: Compares two pointers to determine a length and source, copies a block of data to a new location, and executes the copied data.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_sequence`: Stores a longword to a pointer and clears three subsequent longword locations before branching.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_execute_block`: Compares two memory addresses and copies a block of bytes from a source to a destination before executing the destination address.
- `transfer_bytes_loop`: Copies bytes from one memory location to another in a loop until a counter reaches zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_branch`: Clears a longword at a pointer and then performs a short branch.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `compare_and_branch_block`: Compares two memory addresses and branches based on the result, potentially loading a source address and a counter into registers.
- `copy_fixed_block`: Copies sixteen bytes from a fixed source to a fixed destination and then executes the destination address.
- `copy_memory_and_jump`: Copies a block of memory from one location to another using a counter and then performs an indirect jump.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_global_variables`: Sets three global variables to zero.
- `push_registers_and_return`: Pushes several registers onto the stack before returning from the subroutine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_address_and_branch`: Loads a long value into a register and performs a short branch to another location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_subroutine_5`: The routine is empty and performs no operations before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_16`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_library_calls_and_loop`: Calls functions via a library base pointer, iterates through a null-terminated string, and enters an infinite loop.
- `execute_library_calls_and_return`: Calls functions via a library base pointer, iterates through a null-terminated string, and returns a concatenated 64-bit value.
- `allocate_memory_blocks`: Iteratively searches for available memory chunks by calling a memory type function with decreasing size increments.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_bitmask_from_table_2`: Iterates through a table of function pointers and updates a bitmask based on the return values of those functions.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_bitmask_from_linked_list`: Iterates through a linked list of structures to update specific bits in two separate bitmask variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_bitfield_from_callback`: Iterates through a series of function pointers to retrieve boolean values, which are then used to set or clear specific bits within two separate bitfield buffers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_bitfield_table`: Iterates through a loop to modify bits within a bitfield or table structure based on conditional logic.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_entity_state_table`: Iterates through a list of entities to update their internal status and properties.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_bit_tables`: Clears bits in two registers and enters a loop that increments a counter and stores two registers into a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_bit_sequence`: Iterates through a loop to modify three different values based on a condition and a bitmask, then stores the results in three global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_bits_in_registers`: Clears specific bits in two different registers and then performs an unconditional branch.
- `generate_bit_patterns`: Uses a nested loop structure to manipulate three values based on a bitwise condition and stores them in global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `iterate_and_call_routines`: Loops through a memory range, calling other routines based on data found at an offset, and eventually stores three values into global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_single_bit`: Updates a global variable by setting or clearing a bit at a specific position and then increments a counter through a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_game_state_logic`: Evaluates game state conditions by checking specific character flags and string patterns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_and_store_registers`: Increments a word, compares it to a constant, and if not equal, branches to a previous location; otherwise, it stores three registers into a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_linked_list_data`: Iterates through a structure by updating pointers and values, eventually performing a bitwise OR on a byte and updating three global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_registers_to_global`: Stores the contents of two registers into a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fetch_and_call_table_entry`: Uses a loop to mask a value, calculates an offset to fetch a pointer from a table, and branches to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `execute_library_calls`: Calls a routine via a pointer in a table, stores the result in a global variable, and if not zero, calls another routine via a different offset in the same table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_and_return_stack`: Calls another routine and returns a value composed of a constant and the previous stack pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `dispatch_via_pointer`: Calls a routine through a pointer stored in a global variable, and if the result is non-zero, calls a second routine through a different offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_external_routine_simple`: Calls another routine and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_multiple_library_calls`: Calls a routine via a pointer in a table, stores the result, and then performs a sequence of register initializations and a second routine call via the same table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_and_return_stack_2`: Calls another routine and returns a value composed of a constant and the previous stack contents.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_game_state_and_check_status`: Updates a global structure with new values and checks specific character markers in a data buffer to determine if a state transition or reset is required.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_subroutine_6`: This routine performs a single return instruction to exit the current execution flow.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_indirect_function`: The routine calls a function pointer stored in a global variable and stores the resulting value into a memory location pointed to by a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_subroutine_7`: This routine performs a single instruction to return control to the calling procedure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `allocate_and_check_memory`: Calls several library functions to allocate memory and performs checks on the returned pointer and associated values.
- `copy_string_and_fill_memory_3`: Copies a string of characters from a source address to a destination address and fills the remaining memory up to a specified length with a padding byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_input_and_update_state`: Evaluates input values and compares them against specific character constants to trigger conditional routine calls.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `validate_and_dereference_pointer`: Checks if a value is within a specific range and then uses it to dereference a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `resolve_indirect_function_pointer`: Retrieves a function address from a table using an index provided in a register, then performs a secondary lookup through an additional pointer layer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_dynamic_table_entry`: Checks a value against a threshold and updates a table entry by performing a bitwise OR operation on a word.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `resolve_indirect_pointer`: Fetches a value from a memory location specified by an offset, then performs a secondary lookup through a table to retrieve a final pointer or value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_memory_flag`: Checks a condition based on a register value and then performs a bitwise OR operation on a word at a calculated memory location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_object_state_flags`: Checks a value in a table to determine if a specific routine should be called.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_memory_word`: Calculates a pointer based on several input values and performs a bitwise OR operation on a word at that location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_conditional_callback`: Checks a boolean flag and, if false, executes a function pointer retrieved from a table and checks a specific byte for a character match.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_memory_block_2`: Copies a block of memory from a source address to a destination address using a loop that moves data in eight-byte chunks.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_subroutine`: This routine is empty and performs no operations before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_obfuscated_data`: This routine performs complex data manipulation involving multiple branches and jumps, likely serving as a decryption or deobfuscation routine for encoded game assets.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_complex_conditional`: Performs multiple conditional checks on data within a structure and calls various subroutines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_sequential_calls`: Loads two different offsets from a pointer into a register and calls a subroutine twice.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_memory_blocks`: Performs a series of memory block copies using a loop structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `prepare_register_values`: Calls a subroutine, loads a pointer from a table into a register, calls another subroutine, and then performs arithmetic on a register before branching.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_memory_block_3`: Copies a block of data from a source address to a destination address using a counter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_memory_blocks_2`: Performs multiple block memory transfers using nested loops to move data between source and destination pointers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_process_data`: Checks if a byte at a pointer offset is a numeric character and then performs a series of memory copy operations using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_memory_blocks_3`: Performs multiple memory copy operations between pointers and then copies bytes from the stack into a destination buffer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_state_structure`: Evaluates several conditions based on values in a structure and updates specific members of that structure accordingly.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_stack_data`: Copies a sequence of bytes from one memory location to another based on a count stored in a register.
- `mask_and_conditional_call_2`: Masks a word in one register with a word from another and, if a byte in one register is less than a byte in another, calls another routine to obtain a replacement word; otherwise, it returns the masked word.
- `return_empty_routine_3`: The routine has no instructions that modify memory or registers; the pseudo-code consists solely of a return.
- `build_32bit_from_bytes`: Reads four consecutive bytes from a memory location pointed to by a register and concatenates them into a single 32-bit value stored in a register.
- `generate_bitmask_sequence`: Calculates a bitmask based on a provided value and performs a bitwise rotation loop to populate a sequence of words in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_structure_members`: Updates a value in a structure based on a zero flag and conditionally modifies a structure member using a value from another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reverse_bits_and_store_data`: Performs a bit-reversal loop on a 16-bit value and stores the result into a memory structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_bitstream_state`: Performs bitwise shifts and rotations on a 16-bit value within a loop before storing a byte and a bitmask into a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_bitstream_entry`: Checks a character match, performs bitwise rotations on a 16-bit value, and stores a byte and a bitmask into a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_status_flag`: Checks specific conditions in a global structure and sets a byte at a fixed offset to one if the conditions are met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decode_bitstream_data`: Iterates through a list of values, performing bitwise rotations and arithmetic to populate a structure with decoded data and bitmasks.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_global_structure`: Updates a global structure with a provided integer and sets a specific byte to one based on a boolean condition.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `expand_bitstream_sequence`: Processes a sequence of values using bitwise shifts and rotations to populate a structure with calculated bitmasks and characters.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_subroutine_2`: This routine performs a single instruction to return control to a caller.
- `calculate_table_offsets`: Calculates several memory offsets based on input integers and stores them into a global structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `shift_and_loop`: Performs a logical shift right on a longword, increments a byte, and branches based on a comparison.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_fields`: Calculates a sum of two values and stores it along with several derived values into a structure pointed to by a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_subroutine_3`: This routine is empty and performs no operations before returning to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_subset`: Stores several values into a structure, including a bit-shifted value and a value offset by a constant.
- `copy_string_limited`: Copies a null-terminated string from one memory location to another for a specified number of bytes.
- `load_address_and_call`: Compares a byte at an offset within a structure to a constant, and if they match, loads an address from the structure into a register and calls another routine.
- `update_control_field`: Sets a value in a structure to zero and, if a signed long value is non-negative, stores that value in a different offset of the same structure before calling another routine.
- `validate_magic_word`: Checks if a value pointed to by a register matches specific constant values and returns an error code if they do not match.
- `clear_structure_members`: Sets several specific offsets within a structure to zero.
- `test_byte_and_call`: Compares a byte at an offset within a structure to a constant, and if they match, calls another routine with a specific value in a data register.
- `execute_recursive_call`: The routine performs a self-referential call to itself before returning.
- `load_constant_and_branch`: Loads a small constant into a data register and performs a short branch to another location.
- `traverse_linked_list_and_compare_string`: Iterates through a linked list of structures, updating specific fields in a control block at each node.
- `test_and_move_longword`: Tests a byte at a pointer offset and moves a longword from a pointer to a destination based on the result.
- `transfer_register_data`: Tests a byte, moves a longword from a pointer to a destination, and restores registers from a stack.
- `parse_string_path`: Searches a string for a delimiter, copies characters into a buffer via another routine, and calculates a value based on the result.
- `tiny_move_2`: Small routine of 1 instruction that copies (a0)+ to d0.
- `execute_empty_routine`: Consists of seven instructions that perform no observable work and ends with a return.
- `conditionally_update_value`: Tests a byte at an offset from a register and, if non-zero, performs a conditional update of a longword using a value from another routine.
- `update_memory_from_table`: Checks specific values within a table and, if conditions are met, stores a value from the table into a memory location offset from a register.
- `check_zero_flag`: Evaluates the zero flag and executes a sequence of conditional branches and another routine call based on the flag status.
- `sum_negate_array_words`: It adds 128 consecutive 16-bit words pointed to by a pointer in A0, accumulating the sum in a register.
- `call_dma_management_routine`: Calls a routine that manages DMA state and dispatching based on character flags.
- `manage_dma_state_and_dispatch`: Configures DMA controller registers and manages state transitions based on character flags.
- `scan_port_bits`: Loops through a count, calling another routine to test bits and building a bitmask in a return value.
- `loop_toggle_cia_bit`: It loops thirty times, each pass clearing a CIA port bit and recording the result.
- `clear_cia_port_bit`: It derives a bit index from a register and applies it to a global variable, clearing the associated CIA port bit.
- `reset_and_validate_entity_state`: Clears several fields in a data structure and performs conditional validation checks.
- `update_hardware_and_loop`: Sets a global variable to various values and updates a memory location using a pointer from a structure.
- `perform_complex_loop`: Executes a loop that performs bitwise transformations, checksum calculations, and flag updates using multiple subroutines.
- `validate_and_update_data`: Executes a loop calling multiple routines and, if certain conditions are met, updates a structure with a calculated value.
- `verify_data_integrity`: Performs a complex series of checks involving bitwise operations, memory comparisons, and checksum-like validation on a data structure.
- `nop_operation`: The routine is empty and performs no operations.
- `set_bit_and_increment`: Sets a bit in a 16-bit word within a structure and increments a 16-bit value at a different offset in the same structure.
- `clear_table_entries`: It clears a table by writing zero to successive entries.
- `extract_high_byte_pair`: It calls another routine to obtain a 32-bit value and reassembles it so the low 16 bits come from the result and the high 8 bits come from the original high byte.
- `interleave_word_bits`: It reads two consecutive words from a pointer and returns a value formed from their low bits.
- `apply_interleave_xor_mask`: It calls a routine to interleave word bits and obtains a 32-bit value.
- `initialize_registers`: It loads a value from an offset of a pointer into a register and sets another register to a constant value.
- `compute_xor_checksum`: It iterates through a sequence of words pointed to by an input pointer, XORing each into an accumulator.
- `fill_memory_pattern`: It fills a block of memory with a repeating pattern of a specific constant.
- `interleave_word_streams`: It interleaves the low and high bits of two 32-bit word streams pointed to by two input pointers.
- `loop_clear_flag_bitwise_mix`: Receives two arguments and loops a number of times derived from the first argument.
- `apply_transformation_twice`: The routine pushes a longword onto the stack, performs a logical shift right on it, and calls another routine.
- `clear_flag_byte`: Reads a byte pointed to by a pointer and a flag from the preceding byte.
- `bitwise_mix_store_word`: The routine mixes the input word with a transformed mask derived from the low bits of the input, optionally clears the sign bit based on a flag byte before the output pointer, and stores the resulting word through the pointer.
- `configure_dma_transfer`: It writes several values to hardware registers and global variables to set up a DMA transfer, including setting a source address and a control word.
- `poll_hardware_bit`: Repeatedly calls another routine and checks a bit in a structure until a specific bit is set, then writes three bytes to a global variable.
- `modify_adkcon_bit`: Writes a word to a hardware register and performs a conditional branch based on a value in a global variable.
- `tiny_moveq`: Small routine of 1 instruction that loads 255 into d1.
- `modify_port_bit`: It calculates a bit position based on a value in a table and modifies a specific bit in a hardware port register using a provided byte.
- `wait_for_bit_clear`: Sets a global variable and repeatedly calls another routine until a bit in that global variable is cleared.
- `interpolate_table_value`: Calculates the difference between a value in a table and a register, then decrements a counter through a loop to iteratively update the table entry with the register value.
- `clear_memory_via_index`: Loops a fixed number of times, checking a global flag and writing zero to a memory location indexed by a value from a pointer.
- `modify_and_wait_bit`: Modifies a global variable based on a returned value and then waits for a specific bit in that variable to be set.
- `update_global_variable`: Calls another routine and stores its return value into a global variable.
- `update_bitmask_from_table`: Calculates a bitmask by checking a bit in a table based on a value and a global variable, then returns the result.
- `wait_for_timer`: Repeatedly calls another routine and checks a bit in a global variable until a specific value is returned.
- `test_hardware_bit`: Tests a bit in a hardware register and performs conditional branches and a decrement on a register.
- `write_global_constants`: Stores three specific byte values into a global variable sequentially.
- `update_entity_physics_state`: Updates a data structure by copying values from a source object, calculating a product of two bytes, and performing a byte-swapped word assignment.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `compare_byte_with_offset`: Compares a byte in a register against a byte at a specific offset in a table and returns zero if the condition is met.
- `populate_data_structure`: Calculates several values using multiplication and bit shifting, then copies a string of characters into a destination buffer within a structure.
- `call_sum_array_elements`: Calls a routine that calculates the sum of a list of values and returns a status code.
- `update_hardware_interrupt_state_2`: Modifies two bytes in a memory structure and conditionally performs a block memory copy of a data sequence.
- `sum_array_elements`: Iterates through a list of values to calculate their sum and returns a status code based on whether the sum is non-zero.
- `check_range_bounds`: Compares a value against a threshold and checks if a specific range defined in a table falls within certain bounds.
- `process_index_components`: Clears a value in a structure and performs arithmetic operations to adjust or reconstruct values based on a loop.
- `increment_nested_counters`: Increments several bytes within a structure, including logic to handle overflows and reset specific fields based on comparisons with a table.
- `update_sprite_attribute_buffer`: Updates a memory structure by storing a byte from a register and then performing a block memory copy of 16-byte chunks.
- `process_hardware_data`: Manipulates hardware registers and memory locations through a loop that checks specific bit patterns and decrements a counter.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to d4, then returns.
- `shift_left_byte`: Returns the value of a byte from a pointer shifted left by one bit.
- `calculate_coordinates_from_index`: Performs division and modulo operations using values from a table to store quotient and remainder components into a structure.
- `reconstruct_index_from_components`: Combines multiple values from a structure and a table using multiplication and addition to calculate a single value.
- `copy_byte_on_condition`: Repeatedly calls another routine and copies a byte from one memory location to another if a specific bit is set in the return value.
- `get_signed_byte_from_offset`: Loops until a byte at a specific offset in a structure is non-negative, then returns that byte.
- `manage_hardware_interrupts`: Checks specific memory locations and modifies the INTENA register to enable or disable interrupts and set a global variable.
- `process_state_update`: Updates a global variable and a structure based on a bit test, then conditionally executes a function pointer from a table.
- `initialize_and_update_entity_state`: Checks if a state change is required based on a flag and a comparison value, then performs a multi-stage update of a data structure.
- `initialize_and_process_data_buffer`: Clears a memory structure and enters a loop that processes data through multiple subroutines, updating various status flags and bitmasks based on returned values.
- `copy_memory_block_with_status_check`: Checks a status flag in a structure and, if set, performs a multi-stage memory copy of a data buffer using optimized word and byte transfers.
- `decrement_and_clamp_structure_value`: Iterates through a loop a fixed number of times, decrementing a multi-byte value within a structure by eight in each step.
- `pass_register_value`: Returns the value currently held in a data register to the caller.
- `scan_data_table`: Iterates through a table of values to find a specific entry, updating a counter and modifying a structure based on the result.
- `update_state_structure_2`: Performs arithmetic comparisons and conditional bitwise operations to modify various fields within two different memory structures.
- `calculate_split_values_2`: Updates several memory locations by performing arithmetic and fetching concatenated values through subroutines.
- `fill_memory_with_pattern`: Fills a series of memory locations with a specific word pattern and sets a byte in a structure to a constant value.
- `calculate_weighted_sum`: Computes a value by multiplying different byte segments of a register by specific constants and summing them.
- `combine_three_values`: Calls three separate routines and combines their results into a single 32-bit value.
- `calculate_digit_weighted_sum`: Calculates a value by treating a word as two four-bit nibbles and applying weights to them.
- `calculate_split_values`: Performs arithmetic operations on a value in a register to produce a composite result containing quotients and remainders.
- `fetch_three_values`: Calls three separate routines and concatenates the results into a 32-bit return value.
- `fetch_table_byte`: Retrieves a single byte from a global table using an index provided in a register.
- `tiny_clr`: Small routine of 1 instruction that clears d0.
- `process_data_stream`: Manipulates a data buffer by copying bytes, incrementing counters, and updating hardware interrupt settings via the INTENA register.
- `call_subroutine`: Executes a branch to a subroutine.
- `read_indexed_byte`: Calls another routine and retrieves a byte from a memory location determined by a pointer and an offset stored in a global variable.
- `update_bitmask_and_interrupt`: Performs a bitwise AND operation between a value and a global variable to clear specific bits, then updates a global variable and modifies the interrupt enable register.
- `test_bit_intersection`: Checks if the bitwise intersection of two values stored in a structure is zero.
- `test_bit_flags`: Performs bitwise operations on two values within a structure and calls other routines if specific bits are set.
- `test_register_bits`: Performs a series of bit tests on the contents of registers D0, D1, and D3.
- `update_animation_frame`: Increments a frame counter using a lookup table and updates a state variable.
- `clear_high_bits`: Clears the most significant bit of two different values stored in memory.
- `count_set_bits_recursively`: Checks a bit in a structure and performs a recursive bit-counting loop on a word.
- `modify_bitmasks_and_recurse`: Clears specific bits in two memory locations via bitwise AND operations and updates a value at an offset from a pointer.
- `clear_memory_byte`: Sets a byte at a specific offset within a structure to zero.
- `get_offset_byte`: Returns a byte from a memory location calculated by adding two input values to a base address.
- `concatenate_three_bytes`: Calls three separate routines and concatenates the resulting bytes into a 24-bit value.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `poll_ciab_bit_and_decrement`: Tests a specific bit in a custom chip input register and branches based on the result.
- `set_ciab_timer`: Writes three constant values to a global variable in sequence and touches the CIAB_CRA, CIAB_TAHI, and CIAB_TALO hardware registers.
- `call_external_function`: This routine calls another function and then returns to its caller.
- `update_color_and_copy_data`: Modifies a hardware color register and performs a conditional memory copy operation based on the value of a global variable.
- `call_version_patcher`: Calls another routine and then returns.
- `reset_global_flag`: Sets a global variable to one and then immediately resets it to zero.
- `insert_into_linked_list`: Calculates a value from an input, updates a global variable, and appends a new node to a linked list.
- `allocate_memory_block`: Traverses a linked list of memory blocks to find one with sufficient free space, splits it if necessary, and returns a pointer to the allocated region.
- `lookup_string_entry`: Iterates through a linked list of entries to find one where the stored string matches an input string, returning the associated value or an error code.
- `copy_memory_block_4`: Copies a block of memory from a source pointer to a destination pointer, handling both word-aligned and byte-aligned transfers based on the alignment of the pointers and the size of the data.
- `initialize_string_block`: Allocates a memory block and copies a null-terminated string into it at a specific offset, followed by copying additional data bytes into the allocated structure.
- `branch_to_address`: Performs a short relative branch to a specific memory location.
- `copy_memory_block_optimized`: Performs a memory block transfer using a multi-stage approach that includes an initial byte-by-byte copy followed by an optimized word or longword block transfer.
- `copy_memory_blocks_nested`: Performs multi-stage memory block transfers using nested loops to move data between source and destination pointers.
- `copy_memory_blocks_4`: Performs multiple nested memory copy operations between various pointers and handles stack-based data transfers.
- `return_stack_values`: Returns two values stored on the stack.
- `match_bit_mask_table`: Iterates through a table of word-sized entries to find a match for a specific value, then calculates a bit mask by extracting a bit from a source value based on an offset and a secondary byte value.
- `mask_and_conditional_call_3`: Masks a word in one register with a word from another, then conditionally calls another routine if a byte in one register is less than a byte in another, returning either the masked word or the result of the call.
- `return_empty_routine_2`: This routine is empty; it contains no instructions that modify memory or registers and immediately returns.
- `build_32bit_from_four_bytes`: Reads four consecutive bytes from a memory location pointed to by a register and concatenates them into a single 32-bit value in a register.
- `decode_huffman_data`: Processes a sequence of words using a stack-based algorithm.
- `handle_keyboard_interrupt`: Processes keyboard input by checking global flags and hardware registers, updating state variables, and managing a queue of input events.
- `update_palette_and_state_timers`: Updates several data tables, synchronizes two indices, and manages two timer counters by incrementing, decrementing, or clamping them.
- `fill_data_tables_2`: Copies a block of data from a global variable to a destination, then populates two separate tables with a constant value and a zeroed-out sequence.
- `fill_data_tables`: It copies a global variable into several memory locations and then uses it to calculate and fill two tables with incrementing values.
- `update_global_state_pointer`: Checks a global variable and, depending on its value, updates that variable or a secondary global variable with a new pointer or a constant value.
- `wait_and_clear_interrupts`: Waits for a vertical blank condition by polling a global variable, then clears interrupt requests and disables DMA and interrupts by writing to hardware registers.
- `synchronize_two_indices`: The routine adjusts a global variable toward a first short value in D1 and then toward a second short value in D2 by stepping the variable by two each iteration, wrapping it at the limits implied by the code.
- `store_pattern_to_memory_2`: Writes a pattern with a specific bitmask to a memory table using a pointer.
- `store_pattern_to_memory`: Writes a pattern with a specific bitmask to a memory table using a pointer.
- `scale_table_index_by_factor`: The routine reads a byte from a pointer, masks the low four bits, multiplies the result by an unsigned word from D2, and returns a 32-bit value formed by the high 16 bits of the product and the product shifted right by seven bits.
- `decrement_palette_timer`: Decrements a global counter variable, clamping it to zero if it becomes negative, and sets a flag to zero when the counter reaches a specific sentinel value.
- `increment_palette_timer`: Increments a global counter variable, clamping it to zero if it is negative, and sets a flag to one when the counter exceeds a threshold value.
- `wait_for_flag_clear`: Checks a global variable against a specific constant value.
- `wait_for_flag_change`: Polls a global flag variable until it changes state, initializing another global variable if necessary, and waits for the flag to clear.
- `configure_interrupt_state`: Updates global variables and writes to the INTENA hardware register to enable or disable interrupts based on the bit 15 setting.
- `update_interrupt_requests`: Writes values to the interrupt request register and conditionally clears a global variable before returning from an interrupt.
- `set_interrupt_request`: Writes a value to the interrupt request register.
- `decode_copper_list_bytes`: Iterates through a byte array, performing bitwise manipulations to extract and combine values, then stores the resulting 16-bit words into a destination buffer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_complex_sequence`: Calls several subroutines and performs multiple writes to a global variable before entering an infinite loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_data_table`: Stores three specific values into a series of global variables.

### Sensible World of Moon Soccer.adf:SWOS2.REL#unpacked (raw code file, load address unknown)

- `process_large_code_block`: A very large routine consisting of 11417 instructions with no available pseudo-code.

### Sensible World of Moon Soccer.adf:SWOS#unpacked (raw code file, load address unknown)

- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### Sensible World of Moon Soccer.adf:SWOS2#unpacked (raw code file, load address unknown)

- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### Sensible World of Moon Soccer.adf:SWOS.REL#unpacked (raw code file, load address unknown)

- `undecodable_bytes_17`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
