# Game 205 (SensibleGolf v1.2 0200): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00205). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3326625 top-level bytes in 433 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 7376 raw-word lines |
| Functions named and described | pass, 233 of 233 named (0 author symbols), 233 of 233 described (21 by rule, 210 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1857 of 1857 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 353318 bytes, 2 functions |
| Independent decoder cross-check (Ghidra against capstone) | 12228 of 12229 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 142956 |
| code | 18508 |
| data | 3165151 |
| padding | 10 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 273 | 22 |
| SSBOOT | 1 | 554 | 15 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2404 | 103 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 346 | 21 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1075 | 64 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 4 |
| MAPS28.MAP (raw code file, load address unknown) | 1 | 0 | 0 |
| MAPS32.MAP (raw code file, load address unknown) | 1 | 0 | 0 |
| MAPS9.MAP (raw code file, load address unknown) | 1 | 0 | 0 |
| GOLF#unpacked (raw code file, load address unknown) | 1 | 1 | 1 |
| GOLF.REL#unpacked (raw code file, load address unknown) | 1 | 7570 | 1 |

## Cases handled

- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200.hdf: hard-disk image (AmigaDOS OFS, 4000 blocks): its filesystem parses and all 301 files in it are byte-identical to the extracted tree SensibleGolf_v1.2_0200_hdf, which is recovered file by file; no block outside the filesystem holds data
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/1.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5556 -> 51400 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/2.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (12038 -> 51400 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/3.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4460 -> 51400 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/4.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4459 -> 51400 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/5.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6303 -> 51400 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/6.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5846 -> 51400 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/7.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4665 -> 51400 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/8.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5622 -> 51400 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/9.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5173 -> 51400 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/A.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5609 -> 51400 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/B.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5585 -> 51400 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/BGPATT.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (364 -> 32000 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/C.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (9831 -> 51400 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/CHARSET1.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4397 -> 32000 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/CHARSET2.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4058 -> 32000 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/D.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5475 -> 51400 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/DESBASE.BHT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2288 -> 5120 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/DESBASE.HIT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1300 -> 5120 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/DESBASE.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (27846 -> 51200 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/DISK2.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (940 -> 45696 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/EDIT.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1182 -> 40960 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/GOLF: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (124636 -> 318910 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/GOLF.REL: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (19402 -> 30280 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/GOLFMUS.INS: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (351355 -> 407864 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/GOLFMUS.SNG: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (920 -> 2449 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/ICEBASE.BHT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2297 -> 5120 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/ICEBASE.HIT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1282 -> 5120 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/ICEBASE.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (28141 -> 51200 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/INTBASE.BHT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2279 -> 5120 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/INTBASE.HIT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1275 -> 5120 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/INTBASE.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (28373 -> 51200 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/JUNARMY.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4139 -> 13440 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/JUNBASE.BHT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2280 -> 5120 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/JUNBASE.HIT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1287 -> 5120 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/JUNBASE.INS: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (87480 -> 112358 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/JUNBASE.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (27899 -> 51200 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/JUNBASE.SNG: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1127 -> 3507 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/JUNCOPT.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (12098 -> 43200 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/LOGO.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4037 -> 32000 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM1.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2391 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM10.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2536 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM11.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2128 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM12.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2521 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM13.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1798 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM14.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2067 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM15.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2261 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM16.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2011 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM17.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2212 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM18.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2353 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM19.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2191 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM2.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2004 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM20.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1891 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM21.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1976 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM22.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2511 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM23.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1652 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM24.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2210 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM25.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2013 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM26.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2165 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM27.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2667 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM28.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2298 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM29.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2096 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM3.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2367 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM30.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2059 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM31.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2323 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM32.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1701 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM33.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2233 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM34.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2076 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM35.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1929 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM36.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1254 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM37.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2590 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM38.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2187 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM39.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2501 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM4.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2603 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM40.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2687 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM41.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2017 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM42.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2000 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM43.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2356 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM44.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2638 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM45.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2670 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM46.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2564 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM47.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2295 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM48.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2473 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM49.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2098 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM5.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1999 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM50.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2256 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM51.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2106 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM52.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2174 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM53.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2130 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM54.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1735 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM55.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2391 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM56.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2269 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM57.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1312 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM58.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2285 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM59.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1755 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM6.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2423 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM60.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1846 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM61.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2121 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM62.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2208 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM63.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2654 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM64.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2384 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM65.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2187 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM66.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2331 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM67.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2371 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM68.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2089 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM69.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2357 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM7.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2246 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM70.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2086 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM71.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1950 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM72.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1384 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM8.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2574 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPM9.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2661 -> 7848 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MENUBG.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3053 -> 45696 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MENUPATT.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (691 -> 32000 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MMUS.INS: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (12306 -> 16302 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MMUS.SNG: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (555 -> 1136 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MONEYBG.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3040 -> 45696 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MORBASE.BHT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2290 -> 5120 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MORBASE.HIT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1282 -> 5120 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MORBASE.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (26377 -> 51200 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/POOPOO: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4699 -> 10668 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/PSTUFF.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (17430 -> 40960 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/WIN.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (7755 -> 45696 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/WOODPAT.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3856 -> 6720 bytes)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/ZCOL: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (358 -> 5858 bytes)
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 626 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 296 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 606 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPS28.MAP: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPS32.MAP: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/MAPS9.MAP: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/GOLF#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/GOLF.REL#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- SensibleGolf_v1.2_0200/SensibleGolf_v1.2_0200_hdf/SensibleGolf/data/GOLF.DK2: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- 4 never-loaded region(s), 323038 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `patch_relocation_table`: Validates AmigaDOS hunk types in a structure and, if they match specific values, performs a series of memory writes to patch a table of function pointers and data constants using a value derived from a register swap.
- `call_pointer_from_table`: Saves registers d0 and a0-a1 to the stack, loads an address into a1, loads an address into a0, calls another routine, loads a long value from a relative offset into d2, and returns if d2 is not zero.
- `push_address_to_stack`: Loads an address from a global table into a2, pushes the value 15 and the address itself onto the stack, then returns.
- `compare_and_copy_strings`: Compares a null-terminated string in a0 with a string at a fixed address, and if they match and the byte is zero, performs a series of memory operations and calls another routine.
- `restore_registers_and_branch`: Restores a0-a1 and d0-d1 from the stack, loads the address of offset 32 of the structure in a1 into a1, and branches to another routine.
- `copy_and_call_functions`: Decrements a 16-bit value in a2, copies bytes from a source at a1+32 to a destination at a1, copies a null-terminated string from a0 to a location at a3, and calls two different routines before returning.
- `call_offset_function`: Saves d0 and a0-a2 to the stack, loads the address of a local label into a0, loads a pointer from a global variable into a1, and calls another routine with d0 set to 1400 before restoring the registers and returning.
- `copy_string_to_buffer`: Copies a null-terminated string from a source pointer in a0 to a global buffer, then calls another routine and returns a register value.
- `compare_and_copy_string`: Compares a null-terminated string in a0 with a sequence of characters in a table, increments a global counter if a match is found, and then copies the string to a destination in the table.
- `increment_and_branch`: Loads a 16-bit value from a2 plus 32 into a2, increments d0 by 1, compares d0 to 44, and branches based on the result.
- `copy_string_and_call_subroutines`: Increments a 16-bit value in a global variable, copies a null-terminated string from a source to a destination, and then performs three separate calls to another routine using different source strings and parameters.
- `call_routine_and_return_zero`: Saves registers a0-a2 to the stack, loads an address into a0, calls another routine, restores registers, and returns with d0 set to zero.
- `conditional_call_via_table`: Saves registers d0-d7 and a0-a6 to the stack, checks if d0 is zero, and if so, calls two routines and loads a pointer from a relative offset into d1 before returning.
- `tiny_moveq_tst_rts`: Small routine of 3 instructions that loads 0 into d0, then tests d0, then returns.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `perform_zero_or_instruction`: Performs a bitwise OR operation on register d0 with zero.
- `call_offset_0x58_function`: Loads a function pointer from a global table at a specific offset and executes it.
- `dereference_and_call_pointer`: Loads a pointer from a global variable, calls the routine located at that address, and stores the returned value into a global variable.
- `call_indirect_function_via_table`: Loads a pointer from a table using a relative offset and calls the routine located at an offset of 24 bytes from that pointer.
- `call_global_function_pointer`: Loads a function pointer from a specific offset within a global variable and executes the routine it points to.
- `call_offset_pointer_short`: The routine calls a function located at a specific offset from a pointer stored in a global variable.

### SSBOOT

- `initialize_system_and_load_assets`: Checks memory availability and system flags, then performs a series of file operations to load and decode compressed data chunks.
- `patch_and_execute_code`: Checks a global structure for specific bit patterns to initialize a memory range, then calculates a base address by iterating through a bitmask.
- `copy_and_jump_buffer`: Copies bytes from a source pointer to a destination pointer using a loop controlled by a counter in d0, then performs an indirect jump using a2.
- `copy_and_dispatch_error_string`: Copies a block of memory, selects one of several strings based on a table, and then performs a series of operations involving a library call and setting a hardware register.
- `write_message_to_dos_output`: Opens the DOS library through a library call, measures a zero-terminated message, then calls two DOS routines to write it to the output stream.
- `probe_memory_regions_by_type`: Probes memory regions by testing up to 256 sample addresses in each through the exec memory-type call, and records region presence and type in two bit masks.
- `load_compressed_data_chunk`: Opens a file and reads a header into a buffer, validating it against a specific signature before allocating memory and decoding the compressed data stream.
- `decode_compressed_chunk`: Validates a data header signature and processes a compressed data stream using a stack-based algorithm.
- `parse_bit_table`: Scans a table of words for a matching condition, then calculates a bit mask based on byte values and register contents.
- `mask_and_conditional_call`: The routine masks a word from a register with a word from a register and, if a byte from a register is less than a byte from a register, it calls another routine and uses its returned word as the result; otherwise it returns the masked word.
- `calculate_rotated_value`: Adds a byte from d1 to d7, performs a logical shift and swap on d6, fetches two bytes from a table using a pointer in a3, and performs arithmetic on d1 and d7 before returning.
- `accumulate_32bit_from_bytes`: The routine reads four consecutive bytes from a pointer in a register and builds a 32-bit value in a register by concatenating each byte into the result.
- `build_huffman_tree_nodes`: Iterates over a buffer to match bytes against a running value, writing masks, shifted values, and node properties into a destination array to build a tree structure.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_interrupt_vector_and_jump`: Writes a 16-bit value to a hardware register, stores a 32-bit address into a global variable, and performs an unconditional branch to another location.
- `initialize_hardware_and_interrupts`: Sets the processor status register, populates a table of six pointers, configures hardware registers via a0 and a1, performs several calls, and then branches to another routine.
- `setup_system_state`: Configures hardware registers, clears multiple global variables and memory regions, performs several calls, executes a loop to copy data, and ends by jumping to an address loaded from a global variable.
- `branch_to_system_state_setup`: Performs a short branch to a location within the routine that configures the system state.
- `manage_game_state_loop`: Clears a 16-bit global variable and enters a loop that checks various state flags and counters to determine whether to reset game parameters, update internal counters, or call specific subroutines.
- `reset_game_state_and_toggle_flags`: Clears a 16-bit value and sets another 16-bit global variable to a negative constant, then performs a series of conditional checks and calls to reset_state_counters, execute_table_entry, update_global_from_table, and several other routines.
- `check_global_conditions`: Checks multiple global variables for specific values.
- `execute_infinite_loop`: Clears a word in a global variable and then enters an infinite loop if the value is zero.
- `update_game_state_and_reset_globals`: Increments a global word and clears a global word, then performs a series of conditional checks on multiple global variables and calls several subroutines to update state, reset specific long pointers to zero, and execute a sequence of state-management functions.
- `update_table_with_calculated_offsets`: Fetches a long value from a table using an index, performs bitwise operations, and then writes a series of calculated 16-bit values into a table at specific offsets.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `copy_table_and_clear_globals`: Copies a block of data from one table to another using a loop, then clears several long and word global variables and sets one global variable to a specific value.
- `clear_global_flags`: Checks two global variables; if either is non-zero, it sets them to zero.
- `update_status_and_clear_globals`: Decrements a global word if it is positive, checks other global variables to set a long global variable to either zero or a constant, and clears several other global variables.
- `process_global_values_with_clamping`: Adds values to two global variables, performs conditional checks and assignments to other global variables, and applies clamping logic to the values in the two primary global variables.
- `update_global_parameters`: Calculates a value based on a global variable, stores it in a global pointer, performs bitwise transformations on a second global variable, and writes the results to several global locations.
- `update_table_indices`: Updates several global word and long values, increments or decrements a global word based on its value, and calls another routine.
- `calculate_table_offset`: Computes an index from a global variable, handles sign adjustments, and stores a scaled offset into a memory location pointed to by a register.
- `swap_word_blocks`: Iteratively swaps words between two memory locations pointed to by registers, decrementing a counter until the loop completes.
- `swap_long_blocks`: Swaps longwords between two memory locations pointed to by registers for a number of iterations determined by a value in a register.
- `copy_memory_block_2`: Copies a sequence of long words from the source pointed to by a0 to the destination pointed to by a1 using a loop controlled by d0.
- `copy_string_until_mismatch`: Compares two strings byte-by-byte; if they match up to a null terminator, it copies the first string to the second location, otherwise it stops at the first mismatch.
- `initialize_game_environment`: Sets several hardware registers and global pointers, performs a series of calls, copies data between tables using a loop, calculates a product from two values in memory, and eventually jumps to an address stored in a global variable.
- `process_data_table`: Updates two global variables and then iterates through a table of words, performing arithmetic and conditional assignments on pairs of words based on a global variable.
- `update_global_byte`: Calls another routine, stores the value 51 in a global variable, then stores the result of the call in another global variable, and finally stores 77 in the same global variable.
- `calculate_scaled_product`: Copies two words from memory to a destination, performs a multiplication of a word in d0 and a word in d1, and stores the result in a global word.
- `shift_global_values`: Reads two global words, shifts the first left by four bits, and shifts the second left by three bits after subtracting one, then stores the results in global locations.
- `initialize_system_state`: Copies two blocks of data using another routine, clears several global variables, calls another routine, and sets two bytes at offset 40 and 41 of a structure in a5 to zero.
- `copy_data_tables`: Copies data from two different source tables to two destination tables by calling another routine twice.
- `copy_word_buffer`: Copies 1536 words from a source location to a destination location using a loop.
- `copy_large_word_block`: Copies 7807 words from a source location to a destination location using a loop.
- `clear_word_array`: Clears 352 words in a memory block by writing zero to each location in a loop.
- `fill_memory_with_zero`: Fills a memory area starting at a specific address with 3290 zero words using a loop.
- `copy_memory_block`: Copies 9 words from a source address to a destination address using a loop.
- `clear_global_array`: Fills a memory area starting at a global variable with 42 zero words using a loop.
- `copy_map_data`: Calls two routines, writes a byte to a memory location, and then copies a sequence of bytes from one table to another, potentially decrementing the destination pointer if a byte is negative.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `reset_state_counters`: Checks several global variables and, if specific conditions are met, resets them to specific values and calls three different routines, some of which are called conditionally based on the values of global variables.
- `zero_global_fields`: Sets several consecutive word-sized fields within a global variable structure to zero.
- `check_global_pointers`: Tests two long and word values at specific locations and returns if they are zero, otherwise it continues to the next instruction.
- `calculate_ball_trajectory`: Calculates a new position by performing arithmetic on values from a structure, applying scaling factors from a table via multiplication and bit shifts, and updating several global word and long values.
- `branch_on_word`: Tests a word at a specific location and branches to one location if it is non-zero, otherwise it branches to another location.
- `process_complex_data`: Performs arithmetic on a global long integer, calls two routines, and updates several global word and long values based on the results and table lookups.
- `increment_and_mask_global`: Increments a global word and then masks it with three.
- `process_loop_and_increment`: Stores a word from a register into a global variable, calls another routine, and if a word in d0 is non-zero, executes a loop that calls another routine and increments a global variable by one and masks it with three before calling another routine.
- `execute_conditional_loop`: Calls another routine, stores the result in a global variable, and if d0 is non-zero, executes a loop that calls another routine and then increments a global variable by one and masks it with three before calling another routine.
- `toggle_global_flag`: Checks a global variable against a specific constant value.
- `find_pattern_index`: Searches through a global array for a sequence of three bytes that match specific values stored in global memory.
- `decode_compressed_data`: Iterates through a source buffer to decode data into two destination tables, performing bit shifts and word writes based on the high bits of the source bytes, then updates several global pointers.
- `check_status_flags`: Tests four different memory locations and, if they are non-zero, branches to a specific offset; otherwise, it loads a value from a relative address into a0 and branches to another offset.
- `update_table_values`: Loads a word from a table using an offset in d0, shifts it right by 4, and stores it in a global variable, then iterates through a table to update global variables based on results from another routine.
- `compare_and_set_d1`: Compares a word at a global variable to four and ten, and if it is at least ten, loads a word from a table into d0 and a word from another table into d1, modifies d1 by subtracting eight and shifting left by one, and if the result is at least sixty-four, sets d1 to sixty-four.
- `calculate_offset_and_branch`: Loads two words from relative addresses, performs arithmetic on the second word involving a shift and a subtraction, and branches to another location.
- `load_constant_and_call`: Loads a word into d0 from a relative address, loads the value 40 into d1, and calls another routine before returning.
- `update_status_bytes_2`: Compares a word in d0 with a constant and, if they are not equal, updates a global word with the value in d1 and another global word with the value in d0.
- `update_status_bytes`: Writes a byte from a memory location to an offset of 40 in a global structure, and if a different global variable is zero, also writes that byte to offset 41 of the same structure.
- `adjust_byte_counter`: Modifies a byte in memory by incrementing or decrementing it based on comparisons with a global variable.
- `check_condition_and_call`: Stores a 16-bit value from d1 into a global variable, then checks a global flag and a bitmask of d0 to decide whether to call another routine.
- `execute_table_entry`: Increments a global 16-bit value by 4, loads a pointer from a table using the original value, and jumps to that address unless the pointer is all ones.
- `process_table_data`: Calls three different routines, sets several registers with specific values, and uses a pointer derived from a table to perform a final routine call.
- `update_global_from_table`: Compares a word loaded from a relative address with another word from a relative address; if they are not equal, it stores the first word into a global variable and calls another routine.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `copy_memory_blocks_2`: Copies 240 32-bit words from two source locations pointed to by a0 and a1 into the destination locations pointed to by a2, a3, a4, and a5 using a loop.
- `copy_memory_blocks`: Copies 240 32-bit words from two source locations pointed to by a0 and a1 into the destination locations pointed to by a0 and a1.
- `setup_memory_pointers`: Loads six different addresses into registers a0 through a5 and calls another routine before branching to an earlier instruction.
- `fill_multiple_tables`: Fills six different tables with 32-bit words from d2 and d3, using an index in d0 multiplied by 4 to determine the offset in each table.
- `store_pointer_to_global_2`: Loads a pointer into a1 and calls another routine.
- `store_pointer_to_global`: Stores the 32-bit value from a1 into a global variable.
- `copy_and_clear_data_block`: Checks if a global pointer is non-zero, clears that pointer, and then enters a loop that copies eight 32-bit words from the source pointer to a destination buffer and a hardware register area, while performing specific 16-bit writes to an offset within that destination buffer.
- `copy_data_with_logic`: Copies 242 words from a0 to a1, then uses d2 to select one of four source/destination pairs to perform a block copy via another routine.
- `fetch_table_values`: Calculates an offset using d0, fetches four words from a table at that offset into d0, d1, d4, and d5, and branches to another routine.
- `shift_pixel_data_2`: Calculates a destination address based on a table entry and a bit-shifted value, then calls another routine multiple times to process blocks of data using different source and destination pointer sets depending on the value of a global variable.
- `shift_pixel_data`: Performs bitwise shifts and logical operations on blocks of pixel data to adjust alignment or blend colors.
- `check_status_and_reset`: Calls another routine and a function at a fixed address, then checks several global variables and a value from a global variable to conditionally write 65535 to two different global locations or one global location and a counter.
- `clear_ciaa_interrupts`: Writes a value to the CIAA Interrupt Control Register to acknowledge or clear pending interrupts and then writes 136 to a specific memory location.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `update_interrupt_state`: Sets a global variable to 4, calls a state update routine, and returns a value constructed from registers, while touching the INTREQ hardware register.
- `handle_keyboard_interrupt`: Sets a CIAA control register, manipulates a byte from a keyboard serial register using bitwise operations, and updates a global byte before returning from an interrupt.
- `process_byte_table_entry`: The routine reads a byte from a table using an index stored in a global variable.
- `clear_memory_regions`: Calls another routine to copy data, then conditionally clears two different memory regions by filling them with zero using a loop based on the return value of another routine.
- `clear_other_regions`: Similar to the previous routine, it conditionally zeroes out two different memory blocks by iterating through word-sized pointers based on helper routine returns.
- `convert_and_map_data`: Converts uppercase characters in a string to lowercase, then iterates through a table to update a structure with values from another table.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `convert_string_to_uppercase`: The routine iterates through a null-terminated string pointed to by a register.
- `copy_string_with_lowercase`: The routine copies characters from a source string to a fixed memory location, converting uppercase letters to lowercase.
- `dispatch_string_operations`: Tests a global word and compares d0 against several constants to branch to different logic blocks that either copy strings from a table to a destination or call a routine and perform specific operations based on the value in d0.
- `execute_conditional_logic`: Sets d0 to 2 and a2 to a specific address, then branches to another routine; it also contains logic that sets d0 to 5, loads two pointers, calls another routine, and branches based on the result.
- `call_and_store_result`: The routine compares a word in d0 to negative one; if not equal, it calls another routine and stores the returned value in a global variable.
- `process_string_data`: If a value in d0 is 8 or 12, it calls another routine, copies a string from a table indexed by a global variable to a destination, and then performs bitwise manipulation on a sequence of bytes to pack nibbles into words.
- `jump_on_word_test`: Tests a word at a specific location and either jumps to a different routine or returns.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 70 into -(a0), then merges 0 into d0.
- `perform_bitwise_or`: The routine performs a bitwise OR operation on d0 with zero.
- `reset_potgo_and_flags`: Sets the POTGO hardware register to 65535, calls another routine, and then initializes four global variables or memory locations to 32, 70, 0, and 0 respectively.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `clear_memory_and_color_registers`: Clears sixteen words in a global table starting at a specific memory location and clears thirty-two words in the hardware color registers starting at the first color register, then returns.
- `update_state_counters`: The routine checks several global variables and branches to different logic paths that either call other routines, update specific global values, or decrement a counter and potentially set a global variable to negative one.
- `update_state_counters_2`: Checks four global variables and branches to different logic paths that either update a global variable by adding or subtracting 2, sets a global variable to 65535, or sets it to 16.
- `adjust_nibble_values_2`: Loads two pointers and calculates a difference between two global variables, then branches to another routine if the difference is negative.
- `call_adjust_nibble_logic`: Loads two pointers and calculates a difference between two global variables, then calls another routine if the difference is negative.
- `adjust_nibble_values`: The routine iterates through a buffer of words, extracting nibbles and adding or subtracting a value derived from a global variable, then writes the modified words to a destination buffer.
- `clear_flag_and_call`: Tests a word at a global variable; if it is non-zero, clears that word, sets up several registers with specific values and calls another routine before clearing d0 and returning.
- `set_hardware_registers`: Sets a global variable to zero and certain values, then writes to hardware registers including display, DMA, and interrupt control.
- `copy_data_block_loop`: The routine iterates through a list of shorts; if a value is less than three, it performs several calls and then enters a loop that copies data between several memory locations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `swap_table_elements`: Sets a global variable to specific values, calls two routines, and then enters a loop that swaps values between two tables using a third table as a source.
- `update_status_and_counters`: Calculates two new values by adding constants and values from a table to existing global words, updates a status byte, and performs conditional logic to update several other global bytes and a long pointer.
- `update_counters_and_copy_list`: Updates several global bytes based on arithmetic operations on words from a table, then enters a loop to copy three longwords from a table to a destination list, incrementing the values by a fixed address in each iteration.
- `copy_word_data`: Copies 32 words from a source table to a destination table using a loop.
- `copy_word_block`: Copies 32 words from a source table to a destination table using a decrementing loop and returns.
- `process_and_copy_data`: Performs a series of checks and assignments to global variables, calls several subroutines, and enters a loop that copies 16 words from one memory location to another.
- `process_complex_array`: Calls several routines, clears two global variables, and enters a loop that processes data from a table using a pointer derived from a global variable, eventually updating a global variable and calling another routine.
- `copy_word_data_with_offset`: Copies 16 words from a source table to a destination table, while simultaneously copying 16 words from an offset of 32 bytes within the same source table to a third destination table.
- `copy_word_data_with_offset_2`: Copies sixteen words from a source table to a destination table, while simultaneously copying a single word from an offset of sixteen words into a third destination location using a loop.
- `clear_and_loop_array`: Calls another routine, clears two global variables, calls three routines at a fixed address and two other routines, then enters a loop that increments two global variables until one reaches 16.
- `set_global_words`: Sets two consecutive global words to the value 15.
- `decrement_and_call_routines_2`: Calls another routine, enters a loop that calls a routine at a fixed address based on a value from a global variable, then calls another routine and checks if the sum of two global variables is non-zero before calling a final routine.
- `decrement_and_call_routines`: Decrements two global words if they are non-zero and then calls two separate routines.
- `clear_word_array_2`: Sets seven consecutive global words to zero.
- `swap_and_copy_tables`: Performs two loops: the first copies 12 words from one table to another while swapping values with a source, and the second copies 23 words from one table to another while swapping values with a source.
- `increment_pointer_and_accumulate`: Increments a pointer in a global variable by 4, and if it exceeds a specific threshold, sets it to a new address; otherwise, it adds a value from a table and two other values to a third global variable.
- `update_scaled_value`: Rotates a long value left by one bit and then adds increasing powers of two to it based on whether specific bits are set in a series of global bytes, finally storing the result back to a global variable.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `clear_multiple_globals`: Clears several 16-bit and 32-bit global variables by setting them to zero.
- `set_global_and_call_subroutines`: Writes the value 224 to a global variable, calls another routine, calls a third routine, writes 224 to the same global variable again, and then returns.
- `clear_indexed_table_entries`: Calculates an index by reading a word from a global variable and shifting it left by 2, then clears 16-bit words at seven different offsets from that calculated base address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_address_pointers`: Tests a word at a global address and branches if it is zero, otherwise it calls two routines, loads three different addresses into a0, a1, and a2 from a table, and returns.
- `copy_and_merge_buffers`: Copies 8 bytes from one buffer to another while setting a byte in a third location based on null terminators, then either copies or bitwise-ORs a 27-byte block from one memory area to another depending on a global variable.
- `check_joy_and_buttons`: The routine reads the joystick port movement register and the CIAA port A register, then performs a series of bitwise tests and assignments on a global variable, ending with a conditional write of a constant to that variable.
- `read_joystick_and_mouse_input`: Reads hardware registers for joystick fire buttons and mouse port movement to determine input states.
- `calculate_and_write_blitter_parameters`: Writes d0, d1, d2, and d3 to a table, performs arithmetic to calculate a new address in a2 based on d0 and d1, and calls another routine before returning.
- `configure_blitter_and_draw`: Sets up Blitter registers by writing to hardware ports and a structure in memory, then executes a loop to perform Blitter operations using values from registers and a global variable.
- `copy_list_until_sentinel`: Copies a list of long words from one memory location to another until it encounters the value negative two.
- `load_word_array`: Loads five consecutive 16-bit words from a memory location into five consecutive 16-bit global variables.
- `clear_memory_block`: Clears a block of memory (12850 longwords, from the count loaded into d0) starting at the address in a0 one longword at a time, using a dbra loop counter, and returns.
- `initialize_audio_and_registers`: Sets a word at a global variable to negative one, copies a word from one global variable to another, and if a word at a global variable is zero, clears that global variable and calls another routine, then sets bit 1 of a hardware register, calls another routine, calls another routine, and sets four bytes at offsets 40 to 43 of a structure in a5 to zero.
- `set_byte_values`: Writes the value 64 to four consecutive byte-sized memory locations at a fixed address.
- `clear_byte_values`: Writes zero to four consecutive byte-sized memory locations at the same fixed address used by the previous routine.
- `increment_byte_counters`: Increments four specific byte-sized memory locations by one if their current values are less than the ASCII character '@', contingent on a global variable being non-zero and divisible by four.
- `decrement_byte_counters`: Checks four specific byte-sized memory locations and decrements each by one if its current value is non-zero.
- `branch_to_offset_3`: Performs a short branch to a specific instruction offset within another routine.
- `branch_to_subroutine`: Performs a relative branch to another location.
- `update_entity_state_flags`: Checks a bit in a global variable and, if set, updates a bitmask in a byte using a bit shift, calculates an offset by multiplying a register by 176, and writes a long word and three bytes to the resulting memory location.
- `branch_to_offset_8244`: Performs a short branch to a location 8244 bytes ahead in the code.
- `branch_to_offset`: Performs a branch to a relative address.
- `initialize_audio_parameters`: Initializes a structure by setting several byte and word values, performs arithmetic on a pointer-based list, and calls four routines with specific values at offsets 46 and 134 of a structure.
- `clear_audio_dma_buffer`: Clears a block of memory by writing zeros to word-sized elements, likely preparing a buffer for audio DMA operations given the hardware register context.
- `store_masked_byte`: Saves registers to the stack, loads a relative address into a5, compares d1 to a constant, performs bitwise operations on d1 and d0, and stores a byte from d1 into a memory location calculated using a5 and d0.
- `set_byte_values_2`: Stores a single byte value into four consecutive memory locations at offsets 40 through 43 from a base pointer.
- `update_structure_pointers`: Sets a bit in a global variable, calculates an offset using a multiplication, and if a value is non-zero, copies a pointer and several bytes into a structure at that offset before calling another routine.
- `process_entity_flags`: Checks a bit in a structure, calculates an offset using multiplication, and updates a long word and several bytes in a memory location based on values from another structure.
- `dispatch_sequential_subroutines`: Checks if a long word at offset 166 of a structure in a0 is non-zero and if a byte at offset 39 of a structure in a5 is non-zero, then calls six subroutines in sequence; otherwise, it checks if a byte at offset 38 of a structure in a5 and a byte at offset 91 of a structure in a0 are non-zero and calls five subroutines in sequence.
- `process_flag_and_pointer`: Reads a byte at offset 57 from a structure, clears its lowest bit, and if the bit was set, updates a pointer and copies data from offsets 22 and 6 within the structure.
- `check_flag_and_set_status`: Reads a byte at offset 56 from a structure, clears its lowest bit, and if the bit was set, sets a byte at offset 57 to 255 and updates a word at offset 150 in another structure with a value from offset 46 ORed with 32768.
- `copy_word_value`: Copies a word from offset 46 of a structure pointed to by a0 through a0 to offset 150 of a structure pointed to by a6.
- `process_script_operand_stream`: Reads a byte from a data stream and uses it as a command to dispatch operations in a loop.
- `branch_to_variable_length_integer_parser`: Performs a short branch into another routine to continue processing a variable-length encoded integer.
- `branch_to_next_routine_2`: Performs a short branch to the instruction immediately following this routine.
- `branch_to_offset_2`: Performs a short branch to a specific offset.
- `branch_to_branch_and_copy_data`: Performs a short branch to the branch_and_copy_data routine.
- `branch_to_target`: Performs a short branch to a specific offset in another routine.
- `branch_to_next_routine`: Performs a short branch to another routine.
- `branch_and_copy_data`: Performs a branch to another location, calls another routine, and then performs several branches that either copy a byte from a pointer in a1 to an offset in a0 or copy a byte from a1 to d0 and call another routine if d0 is zero.
- `parse_variable_length_integer`: Reads a byte from a pointer in a1, stores it at offset 48 of a0, and enters a loop that processes a variable-length encoded integer.
- `execute_sequential_subroutines`: Copies a pointer from a0 into a2 and calls three consecutive subroutines in sequence before returning.
- `calculate_clamped_value`: Computes a scaled product of two word values from a structure, clamps the result between 0 and 64, and stores the result back into the structure and another location.
- `increment_index_and_scale`: Updates a value in a structure using a multiplier from a pointer and increments an index or resets it using a value from a table.
- `calculate_scaled_value`: Calculates a value by multiplying a byte by another byte and dividing by a third byte in a loop, stores the result in a structure, and calls another routine if a byte at offset 58 is not negative.
- `branch_and_clear_register`: Branches to a relative address, clears register d0, and then branches to another relative address.
- `set_word_and_clear_byte`: Sets a word at offset 50 of a structure to 64 and clears a byte at offset 58 of the same structure.
- `copy_attributes_from_pointer`: Copies a byte from offset 3 of a structure pointed to by a1 to offset 62 of a0, then derives and stores two other bytes at offsets 59, 61, and 60, and sets offset 58 to 2.
- `setup_color_attributes`: Initializes several byte fields in a structure, setting some to zero or 255, and copying a value from a table indexed by a pointer in the structure.
- `update_position_and_counter`: Updates a pointer or index using a scaling calculation, decrements a counter, and updates a third value using an addition or pointer-based lookup.
- `initialize_object_state`: Populates a structure with values from a referenced table, including pointers and byte attributes, and sets up linked list pointers within the structure.
- `load_entity_properties`: Checks if a state byte has changed; if so, it loads multiple fields of a structure from a table indexed by that byte, including pointers and configuration values.
- `clear_flag_and_update_ptr`: Clears the least significant bit of a byte flag.
- `clear_and_update_status`: Clears bit 0 of a byte at offset 144 of a structure in a0, then if that bit was set, it stores 255 at offset 145 of the structure and updates a 16-bit value at offset 150 of a structure in a6 by ORing it with a 16-bit value from offset 134 of the structure in a0.
- `decrement_and_parse_data_stream`: Decrements two byte counters at offsets 171 and 173 of a structure in a0; if both reach zero, it uses a pointer from offset 166 of a0 to parse a sequence of values through a loop, updating various fields in the structure based on opcode-like bytes read from the stream, and calls another routine to process a specific value.
- `clear_bit_and_copy_bytes`: Clears a bit in a byte at offset 44 of a structure in a5 using a bit index derived from the byte at offset 45, then clears a 32-bit value at offset 166 of a structure in a0 and copies two bytes from offset 172 to offset 171 within that same structure.
- `modify_and_clear_bit`: Subtracts the value of a1 from itself, increments a byte from a structure at offset 45 of a5 by 4, clears a bit in the structure at offset 44 of a5, and loops back to an earlier instruction.
- `execute_sequential_subroutines_2`: Copies a pointer from the structure in a0 to a2, then calls three consecutive subroutines in sequence before returning.
- `calculate_clamped_value_2`: Calculates a value by multiplying two 16-bit values from offsets 138 and 136 of a structure in a0, shifting right by 6, multiplying by a 16-bit value from offset 32 of a structure in a5, and shifting right by 6 again; it stores the result in offset 138 of the structure in a0, clamps it between 0 and 64, and stores the final value at offset 8 of a structure in a2.
- `update_index_and_value`: Updates a value in a structure using a multiplication and shift based on a byte from another memory location, then increments an index or wraps it around.
- `calculate_scaled_value_alt`: Iteratively performs multiplication and division on bytes within a structure, updates a counter at offset 148, and calls another routine if a byte at offset 146 is not negative.
- `branch_and_clear_d0`: Branches to another routine, clears register d0, and then branches to another routine.
- `set_constant_value`: Stores 64 at offset 138 of a structure in a0 and clears a byte at offset 146 of the structure.
- `copy_structure_fields`: Copies a byte from offset 3 of a structure pointed to by offset 104 of a structure in a0 into offset 150 of the structure in a0, subtracts a byte from offset 1 of the source structure and stores it in offset 147 of the structure in a0, copies a byte from offset 4 of the source structure into offsets 151 and 150 of the structure in a0, and sets offset 146 of the structure in a0 to 2.
- `update_terrain_slope_and_index`: Calculates a slope value by scaling a byte from a table, updates a 16-bit field in the structure, increments an index, and if the index reaches a limit, resets it to a value from a table.
- `update_terrain_attribute_data`: Copies two words from a register into a structure, extracts three bytes from a pointer derived from a table and a secondary structure to update several offsets in the primary structure, and clears a word at a calculated memory location.
- `apply_bit_masks`: Performs a series of bitwise OR operations on d0 with various constants, negates d0 twice, checks bit 3 of d3, and finally ORs a byte from a global variable into another global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.

### GOLF#unpacked (raw code file, load address unknown)

- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### GOLF.REL#unpacked (raw code file, load address unknown)

- `perform_bit_wise_or_loop`: The routine performs a large number of bitwise OR operations on d0 using various constant byte values.
