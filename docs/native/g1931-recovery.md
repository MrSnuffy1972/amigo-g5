# Game 1931 (Indiana Jones and the Last Crusade - The Action Game): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01931). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901232 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 2 of 2 named (0 author symbols), 2 of 2 described (0 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 14 of 14 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 21 of 21 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 78 |
| data | 527392 |
| padding | 2 |
| unused | 373760 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Indiana Jones and the Last Crusade - The Action Game.adf:bootblock | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |

## Cases handled

- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: custom-format (non-DOS) disk, 1028 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 14 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 286-330 (bytes 146432-169472) outside the filesystem hold data, not code (opcode density 0.026, entropy 6.23)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 341-348 (bytes 174592-178688) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.92)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 352-358 (bytes 180224-183808) outside the filesystem hold data, not code (opcode density 0.009, entropy 7.02)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 363-363 (bytes 185856-186368) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.61)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 374-374 (bytes 191488-192000) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.60)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 385-385 (bytes 197120-197632) outside the filesystem hold data, not code (opcode density 0.004, entropy 2.58)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 396-402 (bytes 202752-206336) outside the filesystem hold data, not code (opcode density 0.011, entropy 7.02)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 407-412 (bytes 208384-211456) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.73)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 418-424 (bytes 214016-217600) outside the filesystem hold data, not code (opcode density 0.011, entropy 7.13)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 429-435 (bytes 219648-223232) outside the filesystem hold data, not code (opcode density 0.011, entropy 7.14)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 440-445 (bytes 225280-228352) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.92)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 451-455 (bytes 230912-233472) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.82)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 462-467 (bytes 236544-239616) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.02)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 473-478 (bytes 242176-245248) outside the filesystem hold data, not code (opcode density 0.015, entropy 7.11)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 484-491 (bytes 247808-251904) outside the filesystem hold data, not code (opcode density 0.008, entropy 7.20)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 495-502 (bytes 253440-257536) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.07)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 517-525 (bytes 264704-269312) outside the filesystem hold data, not code (opcode density 0.009, entropy 7.17)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 561-564 (bytes 287232-289280) outside the filesystem hold data, not code (opcode density 0.010, entropy 6.32)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 572-574 (bytes 292864-294400) outside the filesystem hold data, not code (opcode density 0.010, entropy 7.12)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 583-586 (bytes 298496-300544) outside the filesystem hold data, not code (opcode density 0.010, entropy 6.40)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 594-596 (bytes 304128-305664) outside the filesystem hold data, not code (opcode density 0.012, entropy 5.80)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 605-606 (bytes 309760-310784) outside the filesystem hold data, not code (opcode density 0.010, entropy 6.80)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 616-616 (bytes 315392-315904) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.61)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 671-675 (bytes 343552-346112) outside the filesystem hold data, not code (opcode density 0.015, entropy 7.12)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 682-683 (bytes 349184-350208) outside the filesystem hold data, not code (opcode density 0.014, entropy 5.50)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 693-698 (bytes 354816-357888) outside the filesystem hold data, not code (opcode density 0.012, entropy 7.12)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 726-734 (bytes 371712-376320) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.14)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 737-743 (bytes 377344-380928) outside the filesystem hold data, not code (opcode density 0.013, entropy 7.09)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 759-760 (bytes 388608-389632) outside the filesystem hold data, not code (opcode density 0.006, entropy 4.70)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 770-775 (bytes 394240-397312) outside the filesystem hold data, not code (opcode density 0.010, entropy 7.11)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 847-852 (bytes 433664-436736) outside the filesystem hold data, not code (opcode density 0.011, entropy 6.58)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 858-862 (bytes 439296-441856) outside the filesystem hold data, not code (opcode density 0.010, entropy 7.16)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 869-871 (bytes 444928-446464) outside the filesystem hold data, not code (opcode density 0.007, entropy 5.94)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 990-993 (bytes 506880-508928) outside the filesystem hold data, not code (opcode density 0.018, entropy 6.74)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1056-1061 (bytes 540672-543744) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.09)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1067-1070 (bytes 546304-548352) outside the filesystem hold data, not code (opcode density 0.011, entropy 6.99)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1078-1082 (bytes 551936-554496) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.68)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1089-1093 (bytes 557568-560128) outside the filesystem hold data, not code (opcode density 0.012, entropy 6.89)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1100-1102 (bytes 563200-564736) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.14)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1111-1118 (bytes 568832-572928) outside the filesystem hold data, not code (opcode density 0.013, entropy 7.12)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1122-1128 (bytes 574464-578048) outside the filesystem hold data, not code (opcode density 0.004, entropy 7.07)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1144-1151 (bytes 585728-589824) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.96)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1155-1161 (bytes 591360-594944) outside the filesystem hold data, not code (opcode density 0.011, entropy 6.85)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1166-1173 (bytes 596992-601088) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.91)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1188-1192 (bytes 608256-610816) outside the filesystem hold data, not code (opcode density 0.014, entropy 7.07)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1199-1201 (bytes 613888-615424) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.99)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1221-1223 (bytes 625152-626688) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.91)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1298-1301 (bytes 664576-666624) outside the filesystem hold data, not code (opcode density 0.008, entropy 7.13)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1309-1312 (bytes 670208-672256) outside the filesystem hold data, not code (opcode density 0.013, entropy 6.63)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1320-1322 (bytes 675840-677376) outside the filesystem hold data, not code (opcode density 0.016, entropy 5.95)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1342-1344 (bytes 687104-688640) outside the filesystem hold data, not code (opcode density 0.013, entropy 6.61)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1353-1357 (bytes 692736-695296) outside the filesystem hold data, not code (opcode density 0.010, entropy 7.09)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1364-1367 (bytes 698368-700416) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.62)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1375-1380 (bytes 704000-707072) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.78)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1386-1391 (bytes 709632-712704) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.44)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1397-1401 (bytes 715264-717824) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.74)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1408-1410 (bytes 720896-722432) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.93)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1430-1432 (bytes 732160-733696) outside the filesystem hold data, not code (opcode density 0.007, entropy 5.97)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1441-1443 (bytes 737792-739328) outside the filesystem hold data, not code (opcode density 0.004, entropy 7.17)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1452-1453 (bytes 743424-744448) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.78)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1529-1531 (bytes 782848-784384) outside the filesystem hold data, not code (opcode density 0.010, entropy 6.40)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1540-1542 (bytes 788480-790016) outside the filesystem hold data, not code (opcode density 0.013, entropy 5.86)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1551-1552 (bytes 794112-795136) outside the filesystem hold data, not code (opcode density 0.010, entropy 5.77)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1562-1564 (bytes 799744-801280) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.03)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1584-1592 (bytes 811008-815616) outside the filesystem hold data, not code (opcode density 0.007, entropy 7.09)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1595-1602 (bytes 816640-820736) outside the filesystem hold data, not code (opcode density 0.009, entropy 7.11)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1606-1608 (bytes 822272-823808) outside the filesystem hold data, not code (opcode density 0.007, entropy 7.10)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1617-1617 (bytes 827904-828416) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.39)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1628-1629 (bytes 833536-834560) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.33)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1650-1653 (bytes 844800-846848) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.95)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1661-1663 (bytes 850432-851968) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.14)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1672-1672 (bytes 856064-856576) outside the filesystem hold data, not code (opcode density 0.020, entropy 6.55)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1683-1684 (bytes 861696-862720) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.58)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1694-1696 (bytes 867328-868864) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.16)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1705-1708 (bytes 872960-875008) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.70)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1716-1718 (bytes 878592-880128) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.16)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1727-1730 (bytes 884224-886272) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.70)
- Indiana Jones and the Last Crusade - The Action Game/Indiana Jones and the Last Crusade - The Action Game.adf: blocks 1738-1759 (bytes 889856-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.28)
- 53 packed track region(s), 314880 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Indiana Jones and the Last Crusade - The Action Game.adf:bootblock

- `call_single_routine_from_boot`: Boot block entry of seven instructions that makes one call to another routine and is reached from the Kickstart strap.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_interleaved_words_to_planes`: Deinterleaves four consecutive words per step from a source buffer into four destination planes spaced 8000 bytes apart, advancing 40 bytes per destination row and 160 bytes per source row.
