# Game 1635 (robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01635). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902320 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 14 of 14 named (0 author symbols), 14 of 14 described (0 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 50 of 50 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 351 of 351 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 127838 |
| code | 1120 |
| data | 413938 |
| unused | 359424 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf:bootblock | 1 | 34 | 3 |
| logged disk load from Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf boot-block load | 1 | 139 | 3 |
| logged disk load from Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf boot-block load | 1 | 0 | 0 |
| logged disk load from Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf boot-block load | 1 | 89 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 89 | 6 |

## Cases handled

- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: custom-format (non-DOS) disk, 1056 non-empty blocks: matched against track-loaded segments
- reference run failed (Karl 4 Nov 2026): run play-rs-m1500 slid through memory (an address: 115 64 KB pages each with >= 29491 distinct PCs executed, 1884040 of its 1980388 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m3000 slid through memory (an address: 115 64 KB pages each with >= 29491 distinct PCs executed, 1884040 of its 1980388 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-menu slid through memory (an address: 115 64 KB pages each with >= 29491 distinct PCs executed, 1884040 of its 1980388 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-mixed slid through memory (an address: 115 64 KB pages each with >= 29491 distinct PCs executed, 1884040 of its 1980388 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-x25k slid through memory (an address: 115 64 KB pages each with >= 29491 distinct PCs executed, 1884040 of its 1980388 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-x35k slid through memory (an address: 115 64 KB pages each with >= 29491 distinct PCs executed, 1884040 of its 1980388 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: the boot block reads 112640 bytes from disk offset 788480 to an address (found statically); recovered as a unit
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: the boot block reads 14848 bytes from disk offset 11264 to an address (found statically); recovered as a unit
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: the boot block reads 4096 bytes from disk offset 2048 to an address (found statically); recovered as a unit
- runtime segment an address: 50 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: 3 logged loads cover 131584 bytes of the image
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 2-2 (bytes 1024-1536) outside the filesystem hold data, not code (opcode density 0.020, entropy 4.81)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 154-166 (bytes 78848-85504) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.09)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 176-188 (bytes 90112-96768) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.07)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 198-210 (bytes 101376-108032) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.90)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 220-232 (bytes 112640-119296) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.03)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 242-254 (bytes 123904-130560) outside the filesystem hold data, not code (opcode density 0.008, entropy 7.04)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 264-275 (bytes 135168-141312) outside the filesystem hold data, not code (opcode density 0.047, entropy 4.45)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 286-298 (bytes 146432-153088) outside the filesystem hold data, not code (opcode density 0.007, entropy 7.07)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 308-320 (bytes 157696-164352) outside the filesystem hold data, not code (opcode density 0.004, entropy 7.04)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 330-342 (bytes 168960-175616) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.01)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 352-364 (bytes 180224-186880) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.77)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 374-386 (bytes 191488-198144) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.93)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 396-408 (bytes 202752-209408) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.01)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 418-430 (bytes 214016-220672) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.92)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 440-452 (bytes 225280-231936) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.98)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 462-474 (bytes 236544-243200) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.91)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 484-496 (bytes 247808-254464) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.01)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 506-518 (bytes 259072-265728) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.76)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 528-540 (bytes 270336-276992) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.70)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 550-562 (bytes 281600-288256) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.96)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 572-584 (bytes 292864-299520) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.79)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 594-606 (bytes 304128-310784) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.96)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 616-628 (bytes 315392-322048) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.51)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 638-643 (bytes 326656-329728) outside the filesystem hold data, not code (opcode density 0.011, entropy 4.49)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 646-650 (bytes 330752-333312) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.16)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 660-672 (bytes 337920-344576) outside the filesystem hold data, not code (opcode density 0.002, entropy 7.04)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 704-716 (bytes 360448-367104) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.02)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 726-729 (bytes 371712-373760) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.02)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 735-738 (bytes 376320-378368) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.02)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 748-760 (bytes 382976-389632) outside the filesystem hold data, not code (opcode density 0.004, entropy 7.08)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 770-782 (bytes 394240-400896) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.86)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 814-826 (bytes 416768-423424) outside the filesystem hold data, not code (opcode density 0.001, entropy 7.09)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 836-848 (bytes 428032-434688) outside the filesystem hold data, not code (opcode density 0.004, entropy 7.10)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 858-870 (bytes 439296-445952) outside the filesystem hold data, not code (opcode density 0.024, entropy 5.94)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 880-892 (bytes 450560-457216) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.82)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 902-914 (bytes 461824-468480) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.86)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 924-936 (bytes 473088-479744) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.80)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 946-958 (bytes 484352-491008) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.93)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 968-980 (bytes 495616-502272) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.89)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 990-1002 (bytes 506880-513536) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.76)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 1012-1024 (bytes 518144-524800) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.87)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 1034-1046 (bytes 529408-536064) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.92)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 1056-1068 (bytes 540672-547328) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.81)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 1078-1090 (bytes 551936-558592) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.77)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 1100-1112 (bytes 563200-569856) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.82)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 1122-1134 (bytes 574464-581120) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.93)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 1144-1156 (bytes 585728-592384) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.93)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 1166-1178 (bytes 596992-603648) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.03)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 1188-1200 (bytes 608256-614912) outside the filesystem hold data, not code (opcode density 0.003, entropy 7.03)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 1210-1222 (bytes 619520-626176) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.97)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 1232-1244 (bytes 630784-637440) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.04)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 1254-1266 (bytes 642048-648704) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.82)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 1276-1288 (bytes 653312-659968) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.03)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 1298-1310 (bytes 664576-671232) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.79)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 1320-1332 (bytes 675840-682496) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.99)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 1342-1354 (bytes 687104-693760) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.98)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 1364-1376 (bytes 698368-705024) outside the filesystem hold data, not code (opcode density 0.011, entropy 7.10)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 1386-1398 (bytes 709632-716288) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.87)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 1408-1420 (bytes 720896-727552) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.88)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 1430-1442 (bytes 732160-738816) outside the filesystem hold data, not code (opcode density 0.011, entropy 7.08)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 1452-1464 (bytes 743424-750080) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.92)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 1474-1486 (bytes 754688-761344) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.88)
- robocop (1987)(ocean)[cr qtx][h hellraisers][t +1 freestyle]/Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf: blocks 1496-1508 (bytes 765952-772608) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.78)
- 2 packed track region(s), 13312 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `set_cia_port_and_dma` (bootblock): Touches the CIA B port B register and the DMA control register, then calls one unnamed routine. Kind: disk hardware/drive validation. Pass path: the instructions immediately following the call.

## Functions


### Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf:bootblock

- `issue_io_then_call_routine`: Boot block entry that issues an exec DoIO request and calls one unnamed routine.
- `set_cia_port_and_dma`: Touches the CIA B port B register and the DMA control register, then calls one unnamed routine.
- `fill_io_request_then_call`: Fills in fields of an exec IO request structure with sizes and a buffer address, issues it through the DoIO library routine, and then calls a further routine.

### logged disk load from Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf boot-block load

- `invoke_two_unnamed_routines`: Calls two unnamed routines in sequence.
- `return_stack_argument`: Returns the value found on top of the stack unchanged as its result.
- `decode_backward_lz_stream`: Decodes a compressed bit stream by reading bits from the end of a buffer and copying earlier output bytes backwards, using length tables and a bit-count table.

### logged disk load from Robocop (1987)(Ocean)[cr QTX][h Hellraisers][t +1 Freestyle].adf boot-block load

- `store_byte_from_longword`: Reads a 32-bit value from just before the address register, stores a word built from the high byte of a data register and the low byte of that value, and returns the value shifted right by one with its top bit set.
- `shift_bits_reloading_on_empty`: Shifts a data register right by one bit per step for a count, calling an unnamed routine to reload it from memory whenever it empties.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `poll_fire_button_then_close`: Loops reading the fire-button bit of the CIA A port and the potentiometer register until a state cell reaches three, then closes a library and copies 64 bytes of a routine into a fixed low-memory buffer.
- `conditionally_write_two_words`: When a flag cell is non-zero, writes a fixed 16-bit word into two cells, and then part of the routine is unreadable in the decompiler output.
- `wait_while_status_nonnegative`: Calls one unnamed routine repeatedly while a status cell is non-negative, then returns.
- `scan_string_then_call_library`: Calls the library routine at offset 240, then measures the length of a zero-terminated string that starts at a fixed offset from a base register, and finally calls the library routine at offset 60.
- `derive_state_from_counter`: Takes the top two bits of a 16-bit counter as a state value, scales that state by sixteen, and writes the result into four fixed memory cells.
- `copy_four_bytes_from_table`: Copies four consecutive bytes from a table entry, selected by a data register and scaled by four, into the buffer that the address register points to.
