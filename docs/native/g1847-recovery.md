# Game 1847 (f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][h gollum][m goonies][t +4 metal force]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01847). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (906352 top-level bytes in 97 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 90 of 90 units identical (bytes and relocations), 170 raw-word lines |
| Functions named and described | pass, 159 of 159 named (0 author symbols), 159 of 159 described (13 by rule, 104 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 272 of 272 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 146678 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2673 of 2673 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 163848 |
| code | 8616 |
| data | 683529 |
| padding | 4791 |
| unused | 45568 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:library | 3 | 17 | 1 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:frontend | 3 | 11 | 1 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:type | 2 | 363 | 12 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:GOLLUM! | 3 | 26 | 2 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:assign | 4 | 18 | 1 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:f15 | 3 | 16 | 1 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:end | 3 | 9 | 1 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:select | 3 | 18 | 2 |
| explode.library | 1 | 402 | 25 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:game | 3 | 19 | 2 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:Crystal | 1 | 166 | 6 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:wait | 2 | 395 | 13 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:bootblock | 1 | 138 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 36 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 35 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 27 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 27 | 2 |
| LB.WLD (raw code file, load address unknown) | 1 | 0 | 0 |
| PG.WLD (raw code file, load address unknown) | 1 | 0 | 0 |
| CE.WLD (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 18 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 15 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 13 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 16 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 18 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 27 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][h gollum][m goonies][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:library: Imploder Library (xfd) stream found by ancient scan (11976 -> 49684 bytes) but only 0 of 16 sampled windows are in reference RAM: not used
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][h gollum][m goonies][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:frontend: Imploder Library (xfd) stream found by ancient scan (105648 -> 233072 bytes) but only 0 of 87 sampled windows are in reference RAM: not used
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][h gollum][m goonies][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:assign: Imploder Library (xfd) stream found by ancient scan (1912 -> 2960 bytes) but only 0 of 2 sampled windows are in reference RAM: not used
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][h gollum][m goonies][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:f15: Imploder Library (xfd) stream found by ancient scan (44668 -> 58984 bytes) but only 0 of 29 sampled windows are in reference RAM: not used
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][h gollum][m goonies][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:end: Imploder Library (xfd) stream found by ancient scan (49104 -> 55432 bytes) but only 0 of 23 sampled windows are in reference RAM: not used
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][h gollum][m goonies][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:select: Imploder Library (xfd) stream found by ancient scan (121388 -> 165612 bytes) but only 0 of 70 sampled windows are in reference RAM: not used
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][h gollum][m goonies][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:game: Imploder Library (xfd) stream found by ancient scan (135324 -> 267092 bytes) but only 0 of 91 sampled windows are in reference RAM: not used
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][h gollum][m goonies][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:F15II(scenario)/LB.WLD: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][h gollum][m goonies][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:F15II(scenario)/PG.WLD: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][h gollum][m goonies][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:F15II(scenario)/CE.WLD: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying code at an address (17 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (54 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (54 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (54 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (38 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (38 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (11 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (54 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (54 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (38 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (38 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (11 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (11 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (54 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (54 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (38 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (38 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (11 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (54 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (38 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (38 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (11 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (11 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (54 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (38 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (38 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (11 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (38 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (38 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (11 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (11 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (29 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (29 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (58 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (58 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (58 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (58 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (58 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (58 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (60 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 3 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 immediate, 1 operand
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][h gollum][m goonies][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:frontend: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][h gollum][m goonies][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:f15: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][h gollum][m goonies][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:select: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 3 never-loaded region(s), 146678 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `clear_memory_then_setup_display` (Crystal): Program entry that sets several global values, clears a block of longwords from a fixed address up to a fixed end address, and makes six routine calls. Kind: loader/protection loop. Pass path: the loop breaks when u10__d0_0570 == '@', proceeding to display setup.

## Functions


### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:library

- `open_explode_library_loop`: Program entry that repeatedly calls a library-open routine until it returns a non-zero library pointer, then makes one more library call.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:frontend

- `open_library_loop_then_halt`: Repeatedly calls a library-open routine until it returns a non-zero library pointer, then makes one library call.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: If a pointer argument is non-null, copies sixteen longwords from a table at a fixed offset of a base register into it.
- `update_entity_state_and_process_events`: Increments an index in a structure, clears several fields in a global structure, and performs a series of conditional calls to update entity properties, status flags, and event data based on values retrieved from tables.
- `branch_to_offset`: Performs a short branch to the instruction at offset 508.
- `set_long_and_branch`: Sets register d1 to negative 256, stores it as a long value at offset 612 in a2, and branches to another routine.
- `set_offset_long_and_branch`: Sets register d1 to negative 256, stores it as a long value at offset 52 in a1, and branches to another routine.
- `fetch_byte_and_call`: Calculates an index using a value from offset 220 in a1 and an offset of 224 in a1, fetches a byte from a0 at that index into d2, and calls another routine before branching to another routine.
- `fetch_and_compare_byte`: Calculates an index using values from a1, fetches a byte from a0 at that index, calls a routine with a pointer from a2, and compares the result to the fetched byte before branching.
- `process_indexed_data_and_call`: Increments a long value at offset 224 of a1, then checks a byte from a table using an index derived from offset 220 of a1.
- `update_status_and_call`: Stores 1 in offset 616 of a2, calls a routine using a pointer from a2, updates offset 208 of a1, and performs several conditional calls to other routines using pointers from a2 before branching.
- `branch_short`: Performs a short branch to another routine.
- `execute_indirect_calls`: Loads a value into d0, loads a pointer from a table at offset 150 of a2 into a4, calls a routine via a5, and then jumps to a routine via a6.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:GOLLUM!

- `open_dos_library_then_run`: Program entry that opens the dos library by name, makes three library calls, then one further call, and returns zero.
- `wait_while_flag_bit_set`: Busy-waits while bit six of a global byte stays set, then returns.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:assign

- `open_library_then_bad_tail`: Program entry that calls a library-open routine until it returns a non-zero library pointer, makes one library call, and then performs a no-op byte copy.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:f15

- `open_library_then_store_flags`: Program entry that opens a library by calling a library-open routine until it returns a non-zero pointer, then makes one library call and stores a value into a table.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:end

- `open_library_bad_tail`: Calls a library-open routine until it returns a non-zero library pointer, makes one library call, and then reaches bad instruction data.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:select

- `open_library_call_bad_tail`: Calls a library-open routine until it returns a non-zero library pointer, makes one library call and one further routine call, and then reaches bad instruction data.
- `return_from_subroutine`: The routine is empty and performs no operations.

### explode.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `init_explode_library_base`: Library initialisation routine in the explode library.
- `increment_library_open_count`: Adds one to a counter field inside the explode library base structure.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `scan_for_explode_signature`: Looks up an index through a library function and checks the table entry at that index for a fixed instruction pattern followed by the name explode.library.
- `increment_counter_and_call`: Increments a global counter in the explode library data, calls one routine, and returns the saved first argument.
- `find_task_set_priority_around`: Finds the current task, calls the priority-setting exec routine on it, makes routine calls, sets the priority again, and returns its first argument unchanged.
- `decode_packed_record_fields`: Fills a structure from a packed input record, copying nibble-coded fields, two longwords from a table, and words from the input.
- `decode_explode_command_loop`: Decompression loop in the explode library.
- `dispatch_explode_block_loop`: Outer loop of the explode library decompressor.
- `copy_explode_window_bits`: Bit-level decoder in the explode library.
- `shift_explode_data_blocks`: Performs a bitwise left shift on a series of data blocks by processing bytes through a series of carry-propagating operations.
- `copy_explode_runs_backward`: Explode library copy routine.
- `read_explode_bits_accumulator`: Explode library bit reader.
- `enter_explode_bit_decode_five`: Entry point into the explode library bit decoder that starts with a count of five and a field width of three.
- `enter_explode_bit_decode_four`: Entry point into the explode library bit decoder that starts with a count of four and a field width of two.
- `enter_explode_bit_decode_three`: Entry point into the explode library bit decoder that starts with a count of three and a field width of one.
- `enter_explode_bit_decode_two`: Entry point into the explode library bit decoder that starts with a count of two and no extra field width.
- `expand_explode_bit_fields`: Explode library routine that expands bit fields from the input into the output.
- `copy_explode_bit_field_chunks`: Explode library routine that copies bit-field chunks from the input window into the output, shifting control bits as it goes and calling one routine for each chunk.
- `fill_explode_output_span`: Explode library routine that loops over a count field and fills output bytes through two routines, shifting control bits and adding copied values into the output.
- `store_explode_state_record`: Explode library routine that stores the current control and bit values into a state record, then loops over the input.
- `release_explode_buffer_memory`: Loops over a count field, calling a routine and a library function on each pass.
- `add_into_global_total`: Adds a register value into a global running total and returns.
- `delay_countdown_loop`: Counts a register down from its starting value to minus one as a delay, then returns.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:game

- `open_library_bad_data_end`: Program entry that calls a library-open routine until it gets a non-zero result and makes one more call.
- `modify_explosion_parameters`: Updates a memory location via a pointer using a negation and addition, decrements a second value at an offset, and performs a bitwise left shift on a value at a third offset.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:Crystal

- `clear_memory_then_setup_display`: Program entry that sets several global values, clears a block of longwords from a fixed address up to a fixed end address, and makes six routine calls.
- `copy_74_bytes_low_memory`: Copies 74 bytes from a block of code into a fixed low-memory area and returns.
- `patch_branch_return_when_flagged`: When a global flag is set, overwrites several instruction words with a short branch and a return instruction, then clears a byte at another offset.
- `update_keyboard_state_byte`: Reads a global keyboard-state byte, clears related bits, then computes an inverted, shifted value and stores it back when the shifted value is zero.
- `fill_word_pairs_from_source`: Writes 48 pairs of 16-bit words into a buffer at a fixed address.
- `draw_string_glyphs_into_plane`: Draws each character of a zero-terminated string by copying eight glyph bytes from a font table into a buffer, advancing 40 bytes per row across a 320-pixel-wide plane.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:wait

- `scan_linked_table_for_max`: Program entry that sets up a 140-byte stack buffer and calls one routine through a register.
- `copy_sixteen_longs_from_table`: If a pointer argument is non-null, copies sixteen longwords from a table at a fixed offset of a base register into it.
- `update_entity_input_state`: Updates a structure by clearing three fields at offsets 208, 212, and 216, then calls process_entity_event_table and check_table_and_branch to determine if a numeric input string should be parsed via parse_numeric_input_string.
- `process_entity_event_table`: Fetches an index from a structure, uses it to look up a table entry, and retrieves a byte value from that table.
- `check_table_and_branch`: Loads a long value from a1 into d1, shifts it left by 2, and tests a long value at offset 8 of a0 plus the shifted d1.
- `calculate_timer_decrement`: Shifts a value from offset 216 of a1, calls a routine, stores the result in offset 212 of a1, and then subtracts 60 from a value in a2 to update offset 208 of a1.
- `parse_numeric_input_string`: Fetches a value from a table using an index in a1, clears two fields in a structure, and calls another routine.
- `validate_and_call_subroutine`: Checks if a byte at a calculated offset in a table falls within the range of 49 to 56 inclusive; if it does, it calls another routine with a specific parameter and returns, otherwise it performs bitwise operations on two calculated indices and returns.
- `process_input_buffer_values`: Checks a value at an offset in a structure against a constant, fetches a byte from a table using an index from the structure, and performs arithmetic to update several offsets within that structure.
- `process_buffer_values`: Fetches a byte from a0 using an index derived from a1, performs arithmetic to update offset 212 of a1, and enters a loop that calls a routine multiple times while updating various offsets in a1.
- `execute_loop_logic`: Stores 60 and 1 into offsets 228 and 232 of a structure in a1, then enters a loop that calls a routine multiple times based on values in a2 and a1, eventually adding a value from offset 232 of a1 to d1 and branching.
- `add_and_branch`: Sets register d1 to 1, adds a long value from an offset of 224 in a1 to d1, and branches to another routine.
- `indirect_jump_stub`: Jump stub: passes control to an address held in or computed from register a6, so the target is only known at run time.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][h Gollum][m Goonies][t +4 Metal Force].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `set_boot_library_request`: Boot-block routine called from the boot entry.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant_2`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_custom_stack_pointer`: Adjusts the stack pointer by four bytes, writes a specific memory address to a global variable, and saves several registers to the stack before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_dma_and_interrupts`: Sets the DMA control register, the interrupt enable register, and the video hardware pointer, then modifies the copper list and custom chip registers before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_chip_ram_buffer`: Disables interrupts and enables DMA channels, then clears a block of memory starting at an address and ending at an address by writing zeros in a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_video_buffer_and_wait`: Updates the copper list, clears a memory block from 70000 to 7c000 with zeros, and enters a loop waiting for the vertical beam position to reach a specific value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_dma_buffer_and_update_cop`: Enables DMA channels, clears a memory block from an address to an address by writing zeros, and updates a coprocessor register before repeating the process with different parameters and a final polling loop on a character at offset 6 of a structure in a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_and_set_cop`: Sets a value in a coprocessor register, configures a DMA controller, and clears a block of memory starting at an address and ending at an address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_chip_ram_and_enable_dma`: Disables all DMA channels by writing to the DMA control register, clears a block of memory starting at an address and ending at an address by writing zero to each word, and then calls several subroutines before entering a loop that polls a character at offset 6 of a structure pointed to by a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_cop_condition`: Calls a routine three times, updates the COP1LCH register, and loops until a byte at offset 6 of a structure in a6 is an underscore.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_sprite_position_list`: Iterates through a list of sprite data structures, updating their horizontal and vertical coordinates based on velocity values stored within each structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_coprocessor_status`: Modifies the status of the coprocessor by writing to its control register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_sprite_position_offset`: Calculates a new position for a sprite by adding a signed offset to a coordinate stored in a structure and writes the result back to that structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_coprocessor_status_2`: Modifies a value in a hardware register associated with the coprocessor to reflect a change in status.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_large_player_data`: Iterates through a data structure in a loop, performing various conditional branches and data manipulations based on the contents of the memory being processed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_coprocessor_status_3`: Modifies the status register of the coprocessor by writing to the hardware register identified as COP1LCH.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_large_play_segment`: The routine performs a series of conditional branches and jumps based on data values, likely acting as a dispatcher for a large table of instructions or state transitions.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_input_buffer`: Reads a byte from a buffer pointed to by a register, compares it against a constant, and branches to different code paths based on the result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_sprite_position_from_table`: Iterates through a table of data to read coordinate values and updates the corresponding sprite position registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_play_rs_x25k_state`: The routine performs a series of operations on data within the runtime segment and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_coprocessor_status_4`: Modifies the status register of the coprocessor by writing a value derived from a register to the hardware register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_runtime_segment_data`: The routine performs a series of conditional branches and data manipulations within a runtime segment, though the provided instruction stream is heavily fragmented by overlapping data and unreachable blocks.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_coprocessor_status_5`: Modifies a value in the coprocessor status register and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_obfuscated_data_stream`: Iterates through a data stream using a series of conditional branches and jumps to perform complex transformations on the data, likely as part of a decryption or decompression routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_cop_status`: Modifies a value in the coprocessor status register via a memory-mapped write.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_sprite_position_table`: Iterates through a table of sprite data, updating the horizontal and vertical coordinates for each entry based on values stored in a global structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_coprocessor_status_6`: Modifies a value in a coprocessor register and then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_address_and_branch`: Loads an address relative to the program counter into a1 and branches to it.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `mask_structure_byte`: Performs a bitwise AND on a byte at offset 8 of a structure pointed to by a6, using the constant 4, and calls several other routines before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_address_and_branch`: Loads an address relative to the program counter into a1 and branches to it.
- `mask_memory_status_byte`: Performs a bitwise AND on a byte at an offset within a structure pointed to by a3 using the constant 4, then calls several other routines before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `toggle_global_variable_on_constant_match`: Compares a value to several constants and, if a match is found, performs a bitwise NOT operation on a global variable and clears that value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `toggle_global_variable_on_state_match`: Performs a bitwise AND on a byte pointed to by a3, calls two subroutines, and then checks if a specific value is held in a routine pointer; if the value matches one of three constants, it inverts a global variable and clears that pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_complex_sequence`: Sets the COP1LCH register, loops until a byte at offset 6 of a pointer in a6 is an underscore, calls four different routines, performs bitwise operations on a byte at offset 8 of a table, calls two more routines, performs bitwise operations on a byte at a pointer in a3, calls three more routines, and modifies a global variable based on comparisons with three specific values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_partial_sequence`: Calls three routines, performs bitwise operations on a byte at offset 8 of a table, calls two more routines, performs bitwise operations on a byte at a pointer in a3, calls three more routines, and modifies a global variable based on comparisons with three specific values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_unspecified_sequence`: This routine contains 20 instructions but the specific operations are not provided.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_reduced_sequence`: Calls three routines, modifies a global variable if it equals a specific character, loops until a byte at offset 6 of a pointer in a6 is an underscore, calls four routines, performs bitwise operations on a byte at a pointer in a3, calls three more routines, and modifies a global variable based on comparisons with three specific values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_minimal_sequence`: Loops until a byte at offset 6 of a pointer in a6 is an underscore, calls four routines, performs bitwise operations on a byte at a pointer in a3, calls three more routines, and modifies a global variable based on comparisons with three specific values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_and_clear_values`: Performs a series of checks on a global variable, and if it matches specific values, it performs a bitwise NOT on a word and clears the global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_address_and_branch_2`: Loads an address relative to the program counter into a1 and branches to it.
- `execute_conditional_sequence`: Calls three routines and modifies a global variable based on comparisons with three specific values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_hardware_and_call`: Branches to a routine if a condition is met, calls a routine, writes a value to the DMA controller, loads a pointer from a stack offset into a6, loads a value from an offset of a6 into a6, writes that value to the custom chip, and writes a value to the interrupt enable register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_and_clear_global`: Compares a global variable to three specific values, and if a match is found, it performs a bitwise NOT on a word and clears the global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_to_address`: Performs an unconditional branch to a specific address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_and_modify_byte`: Compares a byte in a global variable to 82, and if it matches, performs a bitwise NOT on a 16-bit value at a specific location, clears the global variable, and then repeats this logic for the value 83 and the value 64.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_dma_and_coprocessor`: Writes a long value to the coprocessor register, enables DMA channels, and calls another routine.
- `copy_data_loop`: Copies 73 bytes from a source location relative to the program counter to a destination at offset 256 using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `evaluate_and_call_routine`: Checks a global variable against 82, 83, and 64, performing bitwise NOT operations and clearing the variable if matches occur, and calls another routine if the value is 64.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `copy_data_loop`: Copies 73 bytes from a source location relative to the program counter to a destination at offset 256 using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_and_call_routine`: Compares a global variable to 83 and 64, performing bitwise NOT operations and clearing the variable if matches occur, and calls another routine if the value is 64.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `enable_dma_channels`: Writes two 16-bit values to the DMA control register and calls another routine.
- `copy_data_loop_2`: Copies 73 bytes from a source location relative to the program counter to a destination at offset 256 using a loop and then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_hardware_and_call`: Compares a global variable to 83 and 64, calls another routine if the value is 64, and if not, writes to the interrupt enable register, loads a pointer from a stack-based structure, and writes a value to the DMA mask register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `function_epilogue`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `copy_data_loop_3`: Copies 73 bytes from a source location relative to the program counter to a destination at offset 256 using a loop and then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_hardware_conditional`: Compares a global variable to 64, calls another routine if it does not match, and if it does, writes to the interrupt enable register, loads a pointer from a stack-based structure, and writes a value to the DMA mask register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_bytes_in_loop_2`: Copies 73 bytes from a location relative to the program counter to the address in a0 using a loop.
- `write_hardware_registers`: Calculates an address from a pointer on the stack, tests a global byte, and writes specific 16-bit values to offsets of that calculated address based on the results of two subsequent byte tests.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_hardware_and_call_2`: Sets the DMA controller and interrupt enable registers, updates a pointer from a structure in a6, and calls two other routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_memory_block`: Copies a block of memory from the address in a1 to the address in a0 one byte at a time, using a dbra loop counter, and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_interrupts_and_dma`: Writes to the interrupt enable register, sets the coprocessor register, enables DMA channels, and calls another routine.
- `copy_data_loop_4`: Copies 73 bytes from a source table to a destination at offset 256 using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `read_serial_port`: Enables a serial port via a control register, reads a byte from the serial data register, and stores a processed version of that byte in a global variable.
- `interleave_word_pairs_into_buffer`: Writes 48 pairs of 16-bit words into a destination buffer, each pair holding a value taken from a register and the next word read from the source pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `read_serial_port_to_global`: Enables a serial port via a control register, reads a byte from the serial data register, and stores a processed version of that byte in a global variable at offset 160.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `copy_and_read_serial`: If a byte in a pointer is non-zero, it copies 8 blocks of 40 bytes from a source table to a destination calculated from registers, then performs a serial port read and stores the result in a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `poll_keyboard_bit`: Enables a CIA interrupt, reads the keyboard serial data register, clears the serial data register, and then disables the interrupt.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_data_blocks_loop`: Iterates through a list of bytes, using each byte to calculate a destination address and copying 8 blocks of 40 bytes from a source table to that destination.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_serial_data`: Reads a byte from the keyboard serial register, clears it, modifies bit 6 of the CIAA control register, and writes a processed byte to a global variable if a specific bit is set in the modified byte.
- `save_status_and_interleave`: Saves the status register to the stack, then writes 48 pairs of 16-bit words into a destination buffer, each pair holding a value taken from a register and the next word read from the source pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_data_blocks_infinite`: Enters an infinite loop that calculates a destination address and copies 8 blocks of 40 bytes from a source table to that destination based on values in a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_control_register`: Clears bit 6 of the CIAA control register and writes a processed byte to a global variable if a specific bit is set in the modified byte.
- `push_status_and_interleave_words`: Saves the status register to a location pointed to by a4, then writes 48 pairs of 16-bit words into a destination buffer, where each pair consists of a value from d1 and a word from the source pointer in a0, starting with the 32-bit value in d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_offset_and_copy_bytes`: Calculates a destination address using d0, d2, and d3, then enters a loop that copies a byte from a0 to a2, increments a2 by 40, and repeats 8 times before incrementing d0 and branching.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_and_store_byte`: Processes a byte in d0 and writes it to a global variable if a specific bit is set in the modified byte.
- `push_status_and_interleave_words_alt`: Saves the status register to a location pointed to by a4, then writes 48 pairs of 16-bit words into a destination buffer, where each pair consists of a value from d1 and a word from the source pointer in a0, starting with the 32-bit value in d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_bytes_in_loop`: Enters a loop that copies a byte from a0 to a2, increments a2 by 40, and repeats 8 times before incrementing d0 and branching.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `conditionally_store_byte`: If the zero flag is set, stores the byte in d0 into a global variable; otherwise it returns.
- `push_status_and_interleave_words_short`: Saves the status register to a location pointed to by a4, then writes 48 pairs of 16-bit words into a destination buffer, where each pair consists of a value from d1 and a word from the source pointer in a0, starting with the 32-bit value in d0, but the loop runs 48 times.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_bytes_and_increment`: Enters a loop that copies a byte from a0 to a2, increments a2 by 40, and repeats a number of times determined by d3 before incrementing d0 and branching.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `push_status_and_interleave_words_short`: Saves the status register to a location pointed to by a4, then writes 48 pairs of 16-bit words into a destination buffer, where each pair consists of a value from d1 and a word from the source pointer in a0, starting with the 32-bit value in d0, but the loop runs 48 times.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_and_branch`: Increments d0 by 1 and branches to another location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `interleave_word_pairs_into_buffer_2`: Writes 48 pairs of 16-bit words into a destination buffer, each pair holding a value taken from a register and the next word read from the source pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `interleave_words_into_buffer`: Writes 48 pairs of 16-bit words into a destination buffer, where each pair consists of a value from d1 and a word from the source pointer in a0, starting with the 32-bit value in d0.
- `draw_text_glyphs_to_bitplane`: Iterates through a null-terminated string, fetching eight-row glyphs from a font table based on character values and copying them into a bitplane at a calculated memory location.
