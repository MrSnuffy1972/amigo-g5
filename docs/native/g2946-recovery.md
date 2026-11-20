# Game 2946 (bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02946). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (905072 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 65 of 65 named (0 author symbols), 65 of 65 described (25 by rule, 37 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 836 of 836 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 2048 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1334 of 1334 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 8776 |
| code | 4766 |
| data | 755827 |
| padding | 23 |
| unused | 135680 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf:bootblock | 1 | 23 | 2 |
| logged disk load from Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf boot-block load | 1 | 163 | 3 |
| logged disk load from Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf boot-block load | 1 | 269 | 23 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 879 | 37 |
| Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf bytes 11264-13312 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: custom-format (non-DOS) disk, 1493 non-empty blocks: matched against track-loaded segments
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: the boot block reads 9216 bytes from disk offset 889856 to an address (found statically); recovered as a unit
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: the boot block reads 1024 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 679 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: bytes 11264-13312 look like code no run loaded: recovered position-independently (load address unknown)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: 3 logged loads cover 12288 bytes of the image
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 44-53 (bytes 22528-27648) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.77)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 165-174 (bytes 84480-89600) outside the filesystem hold data, not code (opcode density 0.004, entropy 7.17)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 264-273 (bytes 135168-140288) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.97)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 374-383 (bytes 191488-196608) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.74)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 396-405 (bytes 202752-207872) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.60)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 418-420 (bytes 214016-215552) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.07)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 423-427 (bytes 216576-219136) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.66)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 440-449 (bytes 225280-230400) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.19)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 462-471 (bytes 236544-241664) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.50)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 484-485 (bytes 247808-248832) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.13)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 490-493 (bytes 250880-252928) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.56)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 506-515 (bytes 259072-264192) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.53)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 528-536 (bytes 270336-274944) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.48)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 572-581 (bytes 292864-297984) outside the filesystem hold data, not code (opcode density 0.009, entropy 7.18)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 583-592 (bytes 298496-303616) outside the filesystem hold data, not code (opcode density 0.002, entropy 7.01)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 671-680 (bytes 343552-348672) outside the filesystem hold data, not code (opcode density 0.012, entropy 7.08)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 770-779 (bytes 394240-399360) outside the filesystem hold data, not code (opcode density 0.004, entropy 7.19)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 836-845 (bytes 428032-433152) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.06)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 968-977 (bytes 495616-500736) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.91)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1012-1021 (bytes 518144-523264) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.12)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1089-1098 (bytes 557568-562688) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.50)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1111-1120 (bytes 568832-573952) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.06)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1133-1142 (bytes 580096-585216) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.50)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1155-1164 (bytes 591360-596480) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.91)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1177-1186 (bytes 602624-607744) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.48)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1199-1208 (bytes 613888-619008) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.03)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1221-1230 (bytes 625152-630272) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.42)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1243-1252 (bytes 636416-641536) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.20)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1265-1274 (bytes 647680-652800) outside the filesystem hold data, not code (opcode density 0.014, entropy 4.43)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1287-1296 (bytes 658944-664064) outside the filesystem hold data, not code (opcode density 0.010, entropy 3.65)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1298-1307 (bytes 664576-669696) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.34)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1309-1318 (bytes 670208-675328) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.51)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1320-1329 (bytes 675840-680960) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.19)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1331-1340 (bytes 681472-686592) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.02)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1342-1351 (bytes 687104-692224) outside the filesystem hold data, not code (opcode density 0.009, entropy 4.68)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1353-1362 (bytes 692736-697856) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.43)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1366-1373 (bytes 699392-703488) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.54)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1375-1384 (bytes 704000-709120) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.92)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1386-1395 (bytes 709632-714752) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.67)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1397-1406 (bytes 715264-720384) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.13)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1408-1409 (bytes 720896-721920) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.91)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1413-1417 (bytes 723456-726016) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.83)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1420-1428 (bytes 727040-731648) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.65)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1430-1439 (bytes 732160-737280) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.51)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1441-1450 (bytes 737792-742912) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.28)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1452-1461 (bytes 743424-748544) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.25)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1463-1472 (bytes 749056-754176) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.28)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1474-1483 (bytes 754688-759808) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.39)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1485-1494 (bytes 760320-765440) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.08)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1496-1500 (bytes 765952-768512) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.90)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1507-1516 (bytes 771584-776704) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.07)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1522-1527 (bytes 779264-782336) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.07)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1529-1538 (bytes 782848-787968) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.86)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1540-1549 (bytes 788480-793600) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.44)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1551-1560 (bytes 794112-799232) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.82)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1562-1567 (bytes 799744-802816) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.81)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1573-1582 (bytes 805376-810496) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.32)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1584-1593 (bytes 811008-816128) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.18)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1595-1604 (bytes 816640-821760) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.58)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1607-1615 (bytes 822784-827392) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.49)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1628-1637 (bytes 833536-838656) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.46)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1650-1659 (bytes 844800-849920) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.62)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1672-1681 (bytes 856064-861184) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.73)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1694-1697 (bytes 867328-869376) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.11)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1700-1703 (bytes 870400-872448) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.11)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1716-1717 (bytes 878592-879616) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.26)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1719-1725 (bytes 880128-883712) outside the filesystem hold data, not code (opcode density 0.026, entropy 5.19)
- bonanza bros. (1991)(u. s. gold)[cr crack inc][t +5 scx]/Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf: blocks 1756-1759 (bytes 899072-901120) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.26)
- 1 never-loaded region(s), 2048 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 88 packed track region(s), 448000 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf:bootblock

- `initialize_dma_and_coprocessor`: Configures DMA and coprocessor registers, sets up display and color parameters, and initializes memory regions with specific values.
- `frag_move_jmp`: Short code fragment that copies an address to a fixed memory address, then jumps to another point in the code.

### logged disk load from Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf boot-block load

- `copy_and_shift_data`: Copies data from a global variable to a memory region, then performs bit shifting operations on a counter and uses the result to index into the global variable for further processing.
- `process_data_shifts`: Processes data shifts and rotations, performs bit operations on memory values, and handles recursive calls with specific register configurations.
- `process_bit_operations`: Processes bit operations using shift and rotate instructions, performs conditional branches, and handles recursive calls with loop counters.

### logged disk load from Bonanza Bros. (1991)(U. S. Gold)[cr Crack Inc][t +5 SCX].adf boot-block load

- `load_and_execute`: Loads a pointer from a program counter relative address plus 22 into a0 register, then stores that pointer to memory location 128.
- `initialize_display_dma`: Configures DMA and coprocessor registers, sets up display parameters including bitplane and window settings, and initializes color table values.
- `frag_lea_move_subq_tst_bge_jmp`: Short code fragment that loads the address of a fixed memory address into a0, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then subtracts 1 from d0, then tests d0, then branches if the comparison was greater or equal, then jumps to another point in the code.
- `copy_and_initialize`: Copies data from one memory area to another, initializes multiple global variables with specific values, and sets up a table with shifted values from a source buffer.
- `frag_andi_addq_move_bra`: Short code fragment that masks d0 with 15, then adds 1 to d0, then copies d0 to a fixed memory address, then jumps to another point in the code.
- `setup_display_bits`: Configures bit settings in a global variable based on a counter value, writes bytes to memory addresses including CIAB_PRA and a memory location at offset 34 of a structure, then performs a loop that reads from CIAA_PRA until a specific bit is set or a counter expires.
- `write_port_sequence_clearing_bits`: Writes six port values in turn to the same byte register at a fixed offset, each with a different single bit cleared, and ends with all bits set.
- `frag_btst_beq_moveq_bsr_bra`: Short code fragment that tests bit 4 of the memory at an offset from a4, then branches if the last result was zero, then loads 1 into d0, then calls another routine, then jumps to another point in the code.
- `frag_clr_rts`: Short code fragment that clears a fixed memory address, then returns.
- `frag_move_tst_bpl_bsr_sub_beq_add_neg`: Short code fragment that copies d0 to d6, then tests a fixed memory address, then branches if the last result was not negative, then calls another routine, then subtracts a fixed memory address from d6, then branches if the last result was zero, then adds d6 to a fixed memory address, then copies d6 to d0, then branches if the last result was not negative, then negates d0.
- `frag_move_ori_bra`: Short code fragment that copies a fixed memory address to d1, then merges 128 into d1, then jumps to another point in the code.
- `frag_move_ori_not_subq_bra`: Short code fragment that copies a fixed memory address to d1, then merges 130 into d1, then copies d1 to d2, then merges 1 into d2, then inverts d1, then inverts d2, then subtracts 1 from d0, then copies d2 to the memory at an offset from a4, then jumps to another point in the code.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `loop_and_store_byte`: Stores a byte value into memory, executes nested loops using multiplication and decrementing counters, then returns.
- `initialize_global_var`: Initializes a global variable to zero, performs conditional operations based on counter values, and manages a loop that decrements a register until it reaches zero.
- `frag_movea_movem_subq_bmi_bne_st_bra`: Short code fragment that copies the top of the stack to a0, then restores saved registers from the stack, then subtracts 1 from a fixed memory address, then branches if the last result was negative, then branches if the last result was not zero, then sets a fixed memory address to all ones, then jumps to another point in the code.
- `initialize_graphics_params`: Writes multiple predefined values to offsets within a data structure, sets a global variable to 6, then performs a loop that reads from a memory location until a specific bit is set or a counter expires.
- `frag_move_moveq_rts`: Short code fragment that copies 16384 to the memory at an offset from a5, then loads 255 into d4, then returns.
- `process_data_xor`: Performs XOR operations on data arrays, handles bit manipulation and logical operations, and manages conditional branching based on computed values.
- `perform_data_xor_operations`: Executes XOR operations on data arrays, performs bit manipulation and logical operations, and handles conditional branching based on computed results.
- `frag_moveq_movem_lea_rts`: Short code fragment that loads 255 into d4, then restores saved registers from the stack, then loads the address of the memory at an offset from a6 into a6, then returns.
- `frag_and_add_or_rts`: Short code fragment that masks d1 with d5, then adds d0 to d0, then merges d1 into d0, then returns.
- `process_bitwise_operations`: Processes arrays of longwords by performing bitwise AND operations with a global value, doubles the result of the first operation, combines results with OR, and stores the final values in a destination array.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_game_state`: Sets up initial game parameters including display and interrupt configurations, initializes memory areas with specific values, and configures hardware registers for video and input handling before entering a main loop.
- `frag_st_rts`: Short code fragment that sets the memory at an offset from a4 to all ones, then returns.
- `frag_lea_moveq_move_subq_bne_bra`: Short code fragment that loads the address of a location in the code into a0, then loads the address of a fixed memory address into a1, then loads 2 into d0, then loads the address of a fixed memory address into a2, then copies the memory at a2 (advancing a2) to the memory at a1 (advancing a1), then subtracts 1 from d0, then branches if the last result was not zero, then jumps to another point in the code.
- `copy_byte_loop`: Copies bytes from a global table to another location in memory using a fixed-size loop with a counter.
- `copy_byte_data_loop`: Copies bytes from one memory location to another in a fixed-size loop, with conditional checks and specific memory writes at predetermined offsets.
- `copy_byte_data_3`: Writes a fixed byte value to a memory location, performs a conditional check on another memory location, and copies data between addresses.
- `copy_byte_data_2`: Writes a fixed byte value to a memory location, performs a conditional check on another memory location, and copies data between addresses based on the condition.
- `copy_byte_data`: Writes a byte value to a fixed memory address, reads from another memory location, performs a conditional branch based on that read, and then copies bytes between two memory locations in a loop until a counter reaches zero.
- `frag_move_nop_moveq_lea_subq_bne_bra`: Short code fragment that copies 20081 to a fixed memory address, then pauses for one no-operation step, then loads 2 into d0, then loads the address of a fixed memory address into a2, then copies the memory at a2 (advancing a2) to the memory at a1 (advancing a1), then subtracts 1 from d0, then branches if the last result was not zero, then jumps to another point in the code.
- `tiny_rts_nop_rts`: Small routine of 3 instructions that returns, then does nothing for one instruction, then returns.
- `frag_cmpi_bne_bsr_tst_bpl_clr_rts`: Short code fragment that compares the memory at an offset from a6 with 255, then branches if the last result was not zero, then calls another routine, then tests a fixed memory address, then branches if the last result was not negative, then clears the memory at an offset from a4, then returns.
- `initialize_text_processing`: Sets up text processing parameters including a word value at offset -2238, toggles a global variable, initializes a counter at offset -2224, and prepares a data structure pointer at offset -2244 before entering a loop that processes character data.
- `setup_text_rendering`: Configures text rendering settings by storing a word value at offset -2238, inverts another global variable, sets up a counter at offset -2224, and initializes a data pointer at offset -2244 before beginning character processing.
- `initialize_text_processing_2`: Sets up text rendering parameters including a counter and a pointer to a data buffer, then enters a loop that processes text data until a termination condition is met.
- `load_and_process_data`: Loads a word value from a memory location at offset -5058 into a global variable at offset -2240, updates a pointer at offset -2244, and sets a flag at offset -2236 before checking for end-of-data conditions.
- `frag_move_st_rts`: Short code fragment that copies 0 to the memory at an offset from a4 (2 times in a row), then copies 319 to the memory at an offset from a4, then copies 143 to the memory at an offset from a4, then copies the memory at an offset from a4 to the memory at an offset from a4 (2 times in a row), then sets the memory at an offset from a4 to all ones, then returns.
- `update_line_endpoints`: Adjusts line endpoint coordinates based on distance comparisons and boundary checks, then writes updated values to a table before calling another routine with the modified data.
- `initialize_coordinates`: Sets up coordinate values in a data structure based on input parameters and performs boundary checks on calculated values.
- `execute_text_operations`: Calls a text processing routine, checks for end-of-data markers, and repeatedly executes text handling operations while maintaining a flag state at offset -2236 through conditional branches.
- `process_text_data_2`: Processes character data from a buffer, updates counters and coordinates, and handles line breaks and character rendering through subroutine.
- `initialize_game_data`: Sets multiple memory locations with specific constant values including a large word value at offset 64 of a6, a negative one at offset 68, and several other values at offsets 84, 102 and 88.
- `setup_display_parameters`: Initializes multiple memory locations with constant values similar to the first routine but with different word values at offsets 102, 2238 and 2224.
- `setup_collision_data`: Initializes collision-related data structures by setting memory locations to specific values and calling setup routines with different parameters.
- `frag_st_cmpi_bne_bsr_tst_rts`: Short code fragment that sets the memory at an offset from a4 to all ones, then compares the memory at an offset from a6 with 255, then branches if the last result was not zero, then calls another routine, then tests the memory at an offset from a4, then branches if the last result was not zero, then returns.
- `frag_cmpi_bne_move_bsr_subq_rts`: Short code fragment that compares the memory at an offset from a6 with 255, then branches if the last result was not zero, then compares the memory at an offset from a6 with 254, then branches if the last result was not zero, then copies d0 to the stack, then calls another routine, then copies the top of the stack to d0, then subtracts 1 from d0, then branches if the last result was not zero, then returns.
- `frag_pea_move_clr_st_rts`: Short code fragment that pushes the address of the memory at an offset from a4 onto the stack, then copies the top of the stack to the memory at an offset from a4, then clears the memory at an offset from a4 (2 times in a row), then sets the memory at an offset from a4 to all ones, then returns.
- `frag_pea_move_st_rts`: Short code fragment that pushes the address of the memory at an offset from a4 onto the stack, then copies the top of the stack to the memory at an offset from a4, then copies 280 to the memory at an offset from a4, then copies 12 to the memory at an offset from a4, then sets the memory at an offset from a4 to all ones, then returns.
- `frag_subq_bmi_bne_move_not_rts`: Short code fragment that subtracts 1 from the memory at an offset from a4, then branches if the last result was negative, then subtracts 1 from the memory at an offset from a4, then branches if the last result was not zero, then copies 9 to the memory at an offset from a4, then inverts the memory at an offset from a4, then returns.
- `frag_tst_beq_subq_bne_move_not_moveq`: Short code fragment that tests the memory at an offset from a4, then branches if the last result was zero, then subtracts 1 from the memory at an offset from a4, then branches if the last result was not zero, then copies 5 to the memory at an offset from a4, then inverts the memory at an offset from a4, then loads 95 into d2, then tests the memory at an offset from a4, then branches if the last result was zero.
- `copy_coordinates`: Copies 16-bit values from two specific offsets in a data structure to two other offsets within the same structure.
- `process_text_data`: Processes character data from a buffer, updates counters and coordinates, and handles line breaks and character rendering through subroutine calls.
- `configure_blitter_parameters`: Configures blitter-related parameters in a global structure, including setting up memory addresses, flags, and waiting for a specific bit condition.
- `copy_character_data`: Maps character codes to offsets, calculates source and destination addresses, and copies multiple bytes from one memory region to another based on the mapped offset.
- `validate_collision_bounds`: Validates collision boundaries by comparing coordinate values with stored thresholds and jumps to a function pointer if all conditions are met.
- `update_position_values`: Updates position coordinates based on input values, performs boundary checks, and adjusts values according to comparison results with stored thresholds.
- `calculate_bit_flags`: Calculates bit flags by performing arithmetic operations on coordinate values and sets specific bits in a result register based on conditional comparisons.
- `setup_blitter_line_draw`: Orders two endpoints, computes their horizontal and vertical distances and an octant code, then waits for the blitter to be free and writes line-drawing control values and a size word to its registers.
