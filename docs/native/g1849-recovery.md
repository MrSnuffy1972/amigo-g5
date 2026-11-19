# Game 1849 (f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01849). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (906288 top-level bytes in 96 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 89 of 89 units identical (bytes and relocations), 170 raw-word lines |
| Functions named and described | pass, 158 of 158 named (0 author symbols), 158 of 158 described (39 by rule, 100 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 272 of 272 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 146678 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2672 of 2672 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 163848 |
| code | 8614 |
| data | 683467 |
| padding | 4791 |
| unused | 45568 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:library | 3 | 17 | 1 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:frontend | 3 | 11 | 1 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:type | 2 | 363 | 12 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:GOLLUM! | 3 | 26 | 2 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:assign | 4 | 18 | 1 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:f15 | 3 | 16 | 1 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:end | 3 | 9 | 1 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:select | 3 | 18 | 2 |
| explode.library | 1 | 402 | 25 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:game | 3 | 19 | 2 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:Crystal | 1 | 166 | 6 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:wait | 2 | 395 | 13 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:bootblock | 1 | 138 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
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

- extended reference run ignored: it does not re-execute 214 of the 235 PCs the canonical run logged (131341 PCs logged; the harness no longer reproduces the game), so its PCs and RAM are not used
- reference run failed (Karl 4 Nov 2026): run ext slid through memory (an address: 8 64 KB pages each with >= 29491 distinct PCs executed, 131198 of its 131341 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- runtime segment an address (play-lgpwide): 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 18 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 15 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 13 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 16 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 18 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 27 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:library: Imploder Library (xfd) stream found by ancient scan (11976 -> 49684 bytes) but only 0 of 16 sampled windows are in reference RAM: not used
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:frontend: Imploder Library (xfd) stream found by ancient scan (105648 -> 233072 bytes) but only 0 of 87 sampled windows are in reference RAM: not used
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:assign: Imploder Library (xfd) stream found by ancient scan (1912 -> 2960 bytes) but only 0 of 2 sampled windows are in reference RAM: not used
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:f15: Imploder Library (xfd) stream found by ancient scan (44668 -> 58984 bytes) but only 0 of 29 sampled windows are in reference RAM: not used
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:end: Imploder Library (xfd) stream found by ancient scan (49104 -> 55432 bytes) but only 0 of 23 sampled windows are in reference RAM: not used
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:select: Imploder Library (xfd) stream found by ancient scan (121388 -> 165612 bytes) but only 0 of 70 sampled windows are in reference RAM: not used
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:game: Imploder Library (xfd) stream found by ancient scan (135324 -> 267092 bytes) but only 0 of 91 sampled windows are in reference RAM: not used
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:F15II(scenario)/LB.WLD: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:F15II(scenario)/PG.WLD: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:F15II(scenario)/CE.WLD: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (54 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (54 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (54 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (38 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (38 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (11 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (54 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (54 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (38 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (38 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (11 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (11 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (54 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (54 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (38 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (38 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (11 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (54 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (38 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (38 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (11 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (11 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (54 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (38 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (38 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (11 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (38 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (38 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (11 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (11 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (29 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (29 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (58 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (58 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (58 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (58 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (58 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (58 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (60 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-lgpwide)): both forms recovered; writer: runtime segment an address (play-lgpwide) +42, runtime segment an address (play-lgpwide) +56, runtime segment an address (play-lgpwide) +2E, runtime segment an address (play-lgpwide) +2
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 3 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 immediate, 1 operand
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:frontend: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:f15: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:select: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 3 never-loaded region(s), 146678 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:library

- `invoke_object_method_loop`: Loops calling an object's method through a table at offset -552 of a global structure until the returned value is nonzero, then invokes a method at offset -30 of that returned object, and finally masks a byte in a global variable at offset 676 with 142 and stores a global byte at offset a fixed address through a pointer.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:frontend

- `save_registers_call_library`: Saves registers d0-d7 and a0-a7 on the stack, loads the value 3 into d0, computes an address from a table, and calls a routine in the library found in a6.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_d1_to_d0`: Copies the value from register d1 into register d0 and then jumps to another routine.
- `update_entity_state_and_process_events`: Increments an index in a structure, clears several fields in a global structure, and performs a series of conditional calls to update entity properties, status flags, and event data based on values retrieved from tables.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `frag_moveq_move_bra`: Short code fragment that loads 255 into d1, then copies d1 to the memory at an offset from a2, then jumps to another point in the code.
- `frag_moveq_move_bra_2`: Short code fragment that loads 255 into d1, then copies d1 to the memory at an offset from a1, then jumps to another point in the code.
- `fetch_byte_and_call`: Calculates an index using a value from offset 220 in a1 and an offset of 224 in a1, fetches a byte from a0 at that index into d2, and calls another routine before branching to another routine.
- `fetch_and_compare_byte`: Calculates an index using values from a1, fetches a byte from a0 at that index, calls a routine with a pointer from a2, and compares the result to the fetched byte before branching.
- `process_indexed_data_and_call`: Increments a long value at offset 224 of a1, then checks a byte from a table using an index derived from offset 220 of a1.
- `update_status_and_call`: Stores 1 in offset 616 of a2, calls a routine using a pointer from a2, updates offset 208 of a1, and performs several conditional calls to other routines using pointers from a2 before branching.
- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `frag_move_movea_jsr_jmp`: Short code fragment that copies 232 to d0, then copies the memory at an offset from a2 to a4, then calls the routine whose address is in a5, then jumps to the address in a6.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:GOLLUM!

- `open_dos_library_then_run`: Program entry that opens the dos library by name, makes three library calls, then one further call, and returns zero.
- `wait_for_button_release`: Waits in a loop while bit 6 of the joystick fire button register is set and returns as soon as that bit becomes clear.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:assign

- `poll_library_until_ready`: Calls the routine at the function pointer stored at offset -552 of a global structure in a6 until it returns a nonzero result, then invokes the routine at the function pointer at offset -30 of that returned value, then stores a byte through a1 and returns.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:f15

- `poll_library_until_ready_2`: Calls a function pointer from a global table until it returns nonzero, then invokes a routine at offset -30 of that result and stores the value in d5 into a buffer at offset an address of a5, handling negative results by writing bytes and setting a flag bit in a RAM location.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:end

- `init_explode_library`: Saves the caller's registers on the stack, loads value 3 into a register, gets the address of a table, fetches the base pointer of an Amiga OS library from a global variable, and calls a routine inside the explode library, then returns.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:select

- `push_registers_and_call_library`: Pushes all registers on the stack, loads the value one into a data register, computes a call target, copies a global base address into a register, and calls a library entry point, then tests the result and returns when it is zero.
- `branch_on_status_flags`: Evaluates a chain of conditional branches based on the status flags, jumping to one of several target locations, and finally calls the routine at the following address via a subroutine call, then returns.

### explode.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `init_explode_library_base`: Initializes fields of the new library base structure, reads a global variable to obtain a related structure, and stores zero into two of its function vectors.
- `increment_structure_counter_2`: Increments the 32-bit value stored at offset 42 of the structure in a6 by one, then returns.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `check_library_and_increment`: Calls another routine to obtain a pointer, multiplies the result by four, and if the value is nonzero and the pointed-to structure contains the expected library identification strings, increments a global counter and calls another routine, otherwise it returns the original pointer.
- `increment_counter_and_call`: Increments a 32-bit global variable in the explode library data by one, calls another routine, then restores the saved registers and returns the original first argument unchanged.
- `find_task_set_priority_around`: Reads a global variable, calls the exec FindTask routine to obtain the current task, then calls the priority-setting exec routine on that task, invokes another routine, applies the priority again, and returns its first argument unchanged.
- `init_packed_field_and_accumulate`: Initializes a 48-byte field in the structure pointed to by a4 from a packed input buffer, builds a lookup table and a set of cumulative values, then repeatedly calls a routine to accumulate a signed sum into a field while a value is negative.
- `accumulate_signed_loop`: Repeatedly runs a loop body while the incoming value in d0 is negative, accumulating a signed sum into a field through the pointer in a3 and storing the running total back.
- `branch_and_call_routine`: Branches to a nearby routine when the condition code is equal, then copies the address in a3 into a1 and calls the routine at offset 334.
- `load_table_base_and_two`: Loads the address of the 48-byte field at offset 48 of the structure in a4 into a1, then loads the 2 into d0, and falls through into the following routine.
- `copy_run_and_loop`: Copies four bytes from the source buffer in a2 to the destination in a1, advancing both pointers, then decrements a counter at offset 58 of the structure in a4 and a loop counter in d0.
- `copy_run_and_decode_code`: Copies a run of bytes from the source buffer in a0 to the destination in a2, decrementing the run counter, then reads and decodes the next run-length code byte from the buffer in a0.
- `add_run_length_code_to_counter_2`: Decodes a run-length code byte from the buffer in a0, adds its value to the 16-bit counter in d2, then reads a second byte and multiplies the counter by that value, and returns.
- `setup_run_length_five`: Loads the run-length code value five into d0 and the multiplier three into d7, then branches into the run-length decoder routine to process the next buffer byte.
- `setup_run_length_four`: Loads the run-length code value four into d0 and the multiplier two into d7, then branches into the run-length decoder routine to process the next buffer byte.
- `setup_run_length_three`: Loads the run-length code value three into d0 and the multiplier one into d7, then branches into the run-length decoder routine to process the next buffer byte.
- `add_run_length_code_to_counter`: Reads a run-length code byte from the buffer in a0 and adds its value to the 16-bit counter in d2, then reads a second byte and multiplies that counter by two for each bit set, and returns.
- `read_run_length_code`: Reads a run-length code byte from the buffer in a0 into d1, then uses that code to increment the 16-bit value in d2 by the code's value before returning.
- `copy_run_length_bytes`: Reads a run-length code from the buffer in a0, then copies the specified number of bytes from the output buffer back to itself through the pointer in a3, decrementing the byte counter in d0 until the run is complete.
- `decode_run_length_stream`: Decodes a run-length encoded byte stream from the buffer in a0, expanding each run into the destination buffer in a2, then copies the remaining bytes and stores the updated counters and pointers back into the structure in a4.
- `store_registers_and_branch_back`: Stores a byte from a data register at offset three of a structure, stores another byte at offset two, stores a word at the base of the structure, restores several registers from the stack, and branches back into the library.
- `loop_and_update_global`: Increments a word in a structure, runs a loop that calls a routine and another routine while decrementing a counter, then adds a value from the structure to a global variable and returns.
- `add_register_to_global`: Adds the 32-bit value from d0 to the global 32-bit variable and returns.
- `delay_countdown_loop`: Copies the 16-bit value from d6 into d7, decrements it by one, and if it was not originally zero branches into a loop that repeatedly decrements d7 until it reaches minus one, then returns.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:game

- `push_registers_and_call_library_again`: Pushes all registers on the stack, loads the value three into a data register, computes a call target, copies a global base address into a register, and calls a library entry point, then tests the result and returns when it is zero.
- `compare_structure_byte`: Compares the byte at offset 6 of the structure in a6 with zero and branches based on the result, then compares it with 108 and branches based on the result.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:Crystal

- `clear_memory_then_setup_display`: Sets several global values, clears a block of longwords from a fixed address up to a fixed end address in a loop, calls six routines, then waits for a key press and handles the pressed keys before returning.
- `copy_bytes_into_low_memory`: Copies 74 bytes from a block of code into a fixed low-memory area starting at address 256, using a counter initialized to 73 and decrementing until it underflows, then returns.
- `patch_branch_return_when_flagged`: When a global flag is set, overwrites instruction words at offsets 37396 and 37430 with 24580 and 20085, then sets 24580 at offsets 36742, 36656 and 36696, and finally stores 255 at offset 676 of a structure.
- `update_keyboard_state_byte`: Reads a global keyboard-state byte, clears it and clears bit 6 of the CIA control register, then computes an inverted, right-rotated value and stores it back into the global byte when that value is zero.
- `interleave_word_pairs_into_buffer`: Writes 48 pairs of 16-bit words into a destination buffer, each pair holding a value from a register and the next word read from a source pointer, looping until the counter reaches negative one.
- `draw_text_glyphs_to_bitplane`: Copies each eight-row glyph of a zero-terminated string from a font table into a bitplane buffer, advancing the destination by 40 bytes per row and the source by one byte per glyph, until the end of the string is reached.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:wait

- `find_max_and_prepare_buffer_2`: Walks a linked list of entries to find the maximum value stored in a field, then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_d1_to_d0_and_branch`: Copies the 32-bit value from d1 into d0, then branches to the target address without returning on its own.
- `initialize_entity`: Initializes entity data by clearing specific fields in a structure, updating a value through pointer arithmetic, and setting up conditional branches based on table lookups.
- `process_entity_event_table`: Fetches an index from a structure, uses it to look up a table entry, and retrieves a byte value from that table.
- `check_table_and_branch`: Loads a long value from a1 into d1, shifts it left by 2, and tests a long value at offset 8 of a0 plus the shifted d1.
- `calculate_timer_decrement`: Shifts a value from offset 216 of a1, calls a routine, stores the result in offset 212 of a1, and then subtracts 60 from a value in a2 to update offset 208 of a1.
- `parse_numeric_input_string`: Fetches a value from a table using an index in a1, clears two fields in a structure, and calls another routine.
- `validate_and_call_subroutine`: Checks if a byte at a calculated offset in a table falls within the range of 49 to 56 inclusive; if it does, it calls another routine with a specific parameter and returns, otherwise it performs bitwise operations on two calculated indices and returns.
- `process_input_buffer_values`: Checks a value at an offset in a structure against a constant, fetches a byte from a table using an index from the structure, and performs arithmetic to update several offsets within that structure.
- `process_buffer_values`: Fetches a byte from a0 using an index derived from a1, performs arithmetic to update offset 212 of a1, and enters a loop that calls a routine multiple times while updating various offsets in a1.
- `execute_loop_logic`: Stores 60 and 1 into offsets 228 and 232 of a structure in a1, then enters a loop that calls a routine multiple times based on values in a2 and a1, eventually adding a value from offset 232 of a1 to d1 and branching.
- `frag_moveq_add_bra`: Short code fragment that loads 1 into d1, then adds the memory at an offset from a1 to d1, then jumps to another point in the code.
- `indirect_jump_stub`: Jump stub: passes control to an address held in or computed from register a6, so the target is only known at run time.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `set_boot_library_request`: It clears a field and sets a second field in a structure, then makes one library call.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `sets_global_pointer_returns_minus_one`: Stores a constant into a global variable, removes 4 bytes from the stack, restores registers from memory, and returns the value -1 in d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_hardware_registers`: Writes to the coprocessor loop address, DMA controller, interrupt enable register, and video beam position register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_range`: Sets the interrupt enable and DMA controller registers, clears a global variable, and loops to fill the memory range from a fixed address to a fixed address with zeros.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `perform_multiple_memory_clears`: Calls several routines, writes to the coprocessor loop address, sets interrupt and DMA registers, clears a global variable, and loops to fill the memory range from a fixed address to a fixed address with zeros; it then calls more routines, writes to the coprocessor loop address, and loops until the vertical beam position register contains the character underscore.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_dma_buffer_and_update_cop`: Enables DMA channels, clears a memory block from an address to an address, updates a coprocessor register, and repeats the process with different parameters before polling a byte at offset 6 of a structure in a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_and_wait_for_structure_byte`: Calls several routines, writes to the coprocessor loop address, sets the DMA controller and a global variable, and loops to fill the memory range from a fixed address to a fixed address with zeros; it then calls more routines, writes to the coprocessor loop address, and loops until a byte at offset 6 of a structure in a6 is the character underscore.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_and_wait_for_structure_2`: Sets the DMA controller register, clears a global variable, and loops to fill the memory range from a fixed address to a fixed address with zeros; it then calls several routines, writes to the coprocessor loop address, and loops until a byte at offset 6 of a structure in a6 is the character underscore.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_structure_byte_only`: Calls several routines, writes to the coprocessor loop address, and loops until a byte at offset 6 of a structure in a6 is the character underscore.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_sprite_position_from_table`: Iterates through a table of sprite data, reading coordinate values and updating the corresponding hardware sprite registers or memory-mapped sprite positions.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_coprocessor_status`: Modifies the status of the coprocessor by writing to the hardware register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_large_polygon_vertex`: Calculates the screen coordinates for a vertex of a large polygon by applying a transformation matrix to a 3D coordinate and stores the result in a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_coprocessor_register`: Writes a value from a register to the coprocessor register via a pointer stored in a register, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_large_wide_display_data`: The routine performs a series of conditional branches and data manipulations based on a large set of instruction data, though the specific data transformations are obscured by overlapping instruction errors in the disassembly.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_coprocessor_status_2`: Writes a value to the coprocessor status register and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_large_viewport_data`: Iterates through a data structure using a loop to perform operations on a large viewport, involving multiple conditional branches and potential jumps to other routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_unreachable_data_blocks`: The routine attempts to interpret a sequence of data as instructions, resulting in numerous invalid control flow warnings and unreachable code blocks due to the data not being valid executable code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_play_lgpwide_state`: Modifies a value within a data structure located at a specific offset from a pointer in a register and then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_runtime_segment_data`: The routine performs a series of operations within the runtime segment and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_coprocessor_registers`: Modifies the coprocessor registers by writing values to the hardware registers identified by the COP1LCH identifier.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_input_event_stream`: Iterates through a sequence of input data bytes, comparing each byte against a set of predefined values and calling a handler routine when a match is found.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_coprocessor_status_3`: Writes a value from a register to the coprocessor status register and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_input_buffer_stream`: Iterates through a data stream, performing conditional branches based on byte values to execute various subroutines or update state, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_coprocessor_status_4`: Modifies a value in the coprocessor status register and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_input_event_stream_2`: Iterates through a sequence of input data bytes, performing a series of conditional checks and branches based on the byte values to execute specific logic for different input types, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_coprocessor_status_5`: Modifies the status register of the coprocessor by performing bitwise operations and stores the result into a specific hardware register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `poll_and_modify_memory`: Loops until a specific character is found at an offset of 6 from a pointer in a register, then calls several subroutines, performs a bitwise AND on a value in a table, and modifies a global variable based on a character comparison.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `mask_bit_in_table_entry`: Performs a bitwise AND operation on a byte at a specific offset within a table, using the value 4 as a mask, and calls several other routines before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `poll_and_modify_memory`: Loops until a specific character is found at an offset of 6 from a pointer in a register, then calls several subroutines, performs a bitwise AND on a value in a table, and modifies a global variable based on a character comparison.
- `modify_structure_bit_and_loop`: Performs a bitwise AND on a byte at offset 8 of a structure pointed to by a3 using the value 4, then enters a loop that waits until a character at offset 6 of a structure pointed to by a6 is an underscore, before calling another routine and performing further bitwise operations on the structure in a3.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `toggle_global_variable_on_state_match`: Compares a value to several constants and, if a match is found, performs a bitwise NOT operation on a global variable and clears that value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `toggle_global_variable_on_state_match_2`: Performs a bitwise AND on a byte pointed to by a3, calls two subroutines, and then checks if a specific value matches one of three constants; if a match is found, it performs a bitwise NOT on a global variable and clears that value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_bit_and_call_subroutines`: Writes to a byte at offset 8 of a structure pointed to by a register using a bitwise AND with 4, calls several subroutines, and performs conditional bitwise inversions on a global variable based on its value or the value of a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_sequence_subset`: Calls three different routines, masks a byte at offset 8 of a structure in a3 with 4, calls two more routines, loops until a byte at offset 6 of a structure in a6 is an underscore, calls another routine, calls three more routines, masks a byte at the pointer in a3 with 4, calls three more routines, and performs conditional checks on a global variable and a code value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_unspecified_routine`: This routine consists of 20 instructions.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_sequence_partial`: Calls two different routines, loops until a byte at offset 6 of a structure in a6 is an underscore, calls another routine, calls three more routines, masks a byte at the pointer in a3 with 4, calls three more routines, and performs conditional checks on a global variable and a code value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_sequence_minimal`: Loops until a byte at offset 6 of a structure in a6 is an underscore, calls another routine, calls three more routines, masks a byte at the pointer in a3 with 4, calls three more routines, and performs conditional checks on a global variable and a code value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_and_clear_values`: Performs a series of checks on a byte at a global address, clearing it and performing a bitwise NOT on a word at a specific location if certain values are not met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_lea_bra_2`: Short code fragment that loads the address of a location in the code into a1, then jumps to another point in the code.
- `process_conditional_checks`: Calls two different routines and performs conditional checks on a global variable and a code value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_beq_bsr_bra_move_movea`: Short code fragment that branches if the last result was zero, then calls another routine, then jumps to another point in the code, then copies 57388 to a fixed memory address, then copies a fixed memory address to a6, then copies the memory at an offset from a6 to a6, then copies the memory at an offset from a6 to a fixed memory address, then copies 32800 to a fixed memory address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `compare_and_clear_values`: Compares a byte at a global address to specific values, clearing it and performing a bitwise NOT on a word at a specific location if the comparison fails.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_bra_3`: Short code fragment that jumps to another point in the code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `test_and_flip_status_bits`: Checks a longword in a global variable against two different byte values; if a match is found, it clears the longword and flips a specific bit in a byte at a given offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_movea_move_bsr_movem_moveq_rts`: Short code fragment that copies a fixed memory address to a6, then copies the memory at an offset from a6 to a6, then copies the memory at an offset from a6 to a fixed memory address, then copies 32800 to a fixed memory address, then copies 15 to a fixed memory address, then calls another routine, then restores saved registers from the stack, then loads 0 into d0, then returns.
- `frag_lea_move_dbra`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a location in the code into a1, then copies 73 to d0, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then loops back while counting down d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `compare_and_call_routine`: Compares a byte at a global address to specific values, clearing it and performing a bitwise NOT on a word at a specific location if the comparison fails, and calls another routine if a specific value is matched.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `frag_lea_move_dbra`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a location in the code into a1, then copies 73 to d0, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then loops back while counting down d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_and_clear_byte`: Compares a byte at a global variable to 83, and if equal, performs a bitwise NOT on a word at offset 204 of a structure in a6, clears the global variable, and then compares it to 64 before calling another routine if it equals 64.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move_bsr_movem_moveq_rts`: Short code fragment that copies 32800 to a fixed memory address, then copies 15 to a fixed memory address, then calls another routine, then restores saved registers from the stack, then loads 0 into d0, then returns.
- `frag_lea_move_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a location in the code into a1, then copies 73 to d0, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then loops back while counting down d0, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_hardware_and_call`: Compares a byte at a global variable to 83, and if equal, performs a bitwise NOT on a word at offset 205 of a structure in a6, clears the global variable, and then compares it to 64 before calling another routine if it equals 64; otherwise, it writes to the interrupt enable register, loads a structure from a pointer in a6, and writes a value from that structure to the custom DMA address register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `function_epilogue`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `frag_lea_move_dbra_rts_2`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a location in the code into a1, then copies 73 to d0, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then loops back while counting down d0, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_cmpi_beq_bsr_bra_move_movea`: Short code fragment that compares a fixed memory address with 64, then branches if the last result was zero, then calls another routine, then jumps to another point in the code, then copies 57388 to a fixed memory address, then copies a fixed memory address to a6, then copies the memory at an offset from a6 to a6, then copies the memory at an offset from a6 to a fixed memory address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_lea_move_dbra_rts_3`: Short code fragment that loads the address of a location in the code into a1, then copies 73 to d0, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then loops back while counting down d0, then returns.
- `write_memory_values`: Loads a pointer from a stack-based structure, adds a constant to it, and then writes two 16-bit values to specific offsets of that pointer if certain bytes at offsets 330 and 331 are non-zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_interrupts_and_dma`: Writes to the interrupt enable register, loads a structure from a pointer in a6, writes a value from that structure to the custom DMA address register, writes two values to the interrupt mask register, and calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_memory_block`: Copies a block of memory from the address in a1 to the address in a0 one byte at a time, using a dbra loop counter, and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_interrupts_and_dma_with_restore`: Writes to the interrupt enable register, loads a structure from a pointer in a6, writes a value from that structure to the custom DMA address register, writes two values to the interrupt mask register, calls another routine, and restores registers from the stack.
- `frag_lea_move_dbra_rts_4`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a location in the code into a1, then copies 73 to d0, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then loops back while counting down d0, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_keyboard_state_byte_2`: Clears a global variable, enables a CIA interrupt, reads the keyboard serial data register, clears the serial data register, disables the interrupt, and if bit 7 of the inverted and rotated read value is zero, stores that value in a global variable at offset 80858.
- `interleave_word_pairs_into_buffer_2`: Writes 48 pairs of 16-bit words into a destination buffer, each pair holding a value taken from a register and the next word read from the source pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_keyboard_state_byte_alt`: Clears a global variable, enables a CIA interrupt, reads the keyboard serial data register, clears the serial data register, disables the interrupt, and if bit 7 of the inverted and rotated read value is zero, stores that value in a global variable at offset 808a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `copy_data_and_poll_keyboard`: If a byte pointed to by a1 is non-zero, it copies 8 blocks of 40 bytes from a source table to a destination calculated from several registers, then performs the keyboard polling routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `poll_keyboard_bit`: Enables a CIA interrupt, reads the keyboard serial data register, clears the serial data register, disables the interrupt, and if bit 7 of the inverted and rotated read value is zero, stores the result in a global variable at offset 808a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_data_blocks_loop`: Iterates through a list of bytes, using each byte to calculate a destination address and copying 8 blocks of 40 bytes from a source table to that destination, repeating the process as long as the byte in a1 is non-zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move_clr_bclr_not_ror_btst_bne_rts`: Short code fragment that copies a fixed memory address to d0, then clears a fixed memory address, then clears bit 6 of a fixed memory address, then inverts d0, then rotates d0 right by 1, then tests bit 7 of d0, then branches if the last result was not zero, then copies d0 to a fixed memory address, then returns.
- `frag_move_lea_dbra_rts`: Short code fragment that copies the status register to the memory just below a4 (moving a4 down), then loads the address of a location in the code into a1, then copies 47 to d2, then copies d0 to the memory at a1 (advancing a1), then copies d1 to the memory at a1 (advancing a1), then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d2, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_structured_data`: Performs a nested loop where it calculates a destination address using a multiplication and addition, then copies 8 bytes from a source pointer to that destination before incrementing the source pointer and the outer loop counter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_bclr_not_ror_btst_bne_move_rts`: Short code fragment that clears bit 6 of a fixed memory address, then inverts d0, then rotates d0 right by 1, then tests bit 7 of d0, then branches if the last result was not zero, then copies d0 to a fixed memory address, then returns.
- `save_status_and_interleave`: Saves the status register to the stack, then writes 48 pairs of 16-bit words into a destination buffer, each pair holding a value taken from a register and the next word read from the source pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_source_and_copy`: Calculates an offset by shifting a value, adds it to a pointer, and uses a loop to copy 8 bytes from a source to a destination, incrementing a counter and jumping back to the start of the routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_not_ror_btst_bne_move_rts`: Short code fragment that inverts d0, then rotates d0 right by 1, then tests bit 7 of d0, then branches if the last result was not zero, then copies d0 to a fixed memory address, then returns.
- `save_status_and_interleave_data`: Saves the status register to the stack and then performs a loop 48 times to write a 16-bit word from d0, a 16-bit word from d1, and a 16-bit word from the source pointer in a0 into a destination buffer, incrementing the destination and source pointers accordingly.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_moveq_move_adda_dbra_addq_bra`: Short code fragment that loads 7 into d3, then copies the memory at a0 (advancing a0) to the memory at a2, then adds 40 to a2, then loops back while counting down d3, then adds 1 to d0, then jumps to another point in the code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_bne_move_rts`: Short code fragment that branches if the last result was not zero, then copies d0 to a fixed memory address, then returns.
- `save_status_and_interleave_2`: Saves the status register to the stack, then writes 48 pairs of 16-bit words into a destination buffer, where each pair consists of a value from a register followed by a word read from the source pointer in a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move_adda_dbra_addq_bra`: Short code fragment that copies the memory at a0 (advancing a0) to the memory at a2, then adds 40 to a2, then loops back while counting down d3, then adds 1 to d0, then jumps to another point in the code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `save_status_and_interleave_2`: Saves the status register to the stack, then writes 48 pairs of 16-bit words into a destination buffer, where each pair consists of a value from a register followed by a word read from the source pointer in a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_addq_bra`: Short code fragment that adds 1 to d0, then jumps to another point in the code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `interleave_word_pairs_into_buffer_3`: Writes 48 pairs of 16-bit words into a destination buffer, each pair holding a value taken from a register and the next word read from the source pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move_dbra_rts`: Short code fragment that copies 47 to d2, then copies d0 to the memory at a1 (advancing a1), then copies d1 to the memory at a1 (advancing a1), then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d2, then returns.
- `draw_text_glyphs_to_bitplane_2`: Draws each character of a zero-terminated string into a 320-pixel-wide bitplane by copying its eight-row glyph from a font table.
