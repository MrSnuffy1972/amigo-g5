# Game 364 (Airball): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00364). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904560 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 40 raw-word lines |
| Functions named and described | pass, 39 of 39 named (0 author symbols), 39 of 39 described (2 by rule, 34 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 466 of 466 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 836 of 836 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 5222 |
| code | 3150 |
| data | 747771 |
| padding | 961 |
| unused | 147456 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Airball.adf:bootblock | 1 | 20 | 1 |
| logged disk load from Airball.adf boot-block load | 1 | 107 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 709 | 36 |

## Cases handled

- Airball/Airball.adf: custom-format (non-DOS) disk, 1470 non-empty blocks: matched against track-loaded segments
- Airball/Airball.adf: the boot block reads 5632 bytes from disk offset 11264 to an address (found statically); recovered as a unit
- runtime segment an address: 466 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Airball/Airball.adf: 1 logged loads cover 5632 bytes of the image
- Airball/Airball.adf: blocks 12-21 (bytes 6144-11264) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.98)
- Airball/Airball.adf: blocks 34-54 (bytes 17408-28160) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.72)
- Airball/Airball.adf: blocks 100-115 (bytes 51200-59392) outside the filesystem hold data, not code (opcode density 0.008, entropy 7.09)
- Airball/Airball.adf: blocks 122-131 (bytes 62464-67584) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.50)
- Airball/Airball.adf: blocks 144-153 (bytes 73728-78848) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.31)
- Airball/Airball.adf: blocks 166-175 (bytes 84992-90112) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.25)
- Airball/Airball.adf: blocks 188-197 (bytes 96256-101376) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.02)
- Airball/Airball.adf: blocks 210-219 (bytes 107520-112640) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.06)
- Airball/Airball.adf: blocks 232-241 (bytes 118784-123904) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.29)
- Airball/Airball.adf: blocks 243-252 (bytes 124416-129536) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.61)
- Airball/Airball.adf: blocks 254-263 (bytes 130048-135168) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.40)
- Airball/Airball.adf: blocks 265-274 (bytes 135680-140800) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.86)
- Airball/Airball.adf: blocks 276-285 (bytes 141312-146432) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.82)
- Airball/Airball.adf: blocks 287-296 (bytes 146944-152064) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.89)
- Airball/Airball.adf: blocks 298-307 (bytes 152576-157696) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.72)
- Airball/Airball.adf: blocks 309-318 (bytes 158208-163328) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.13)
- Airball/Airball.adf: blocks 320-329 (bytes 163840-168960) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.90)
- Airball/Airball.adf: blocks 331-340 (bytes 169472-174592) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.44)
- Airball/Airball.adf: blocks 342-351 (bytes 175104-180224) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.40)
- Airball/Airball.adf: blocks 353-362 (bytes 180736-185856) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.43)
- Airball/Airball.adf: blocks 364-373 (bytes 186368-191488) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.44)
- Airball/Airball.adf: blocks 375-384 (bytes 192000-197120) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.43)
- Airball/Airball.adf: blocks 386-395 (bytes 197632-202752) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.54)
- Airball/Airball.adf: blocks 397-406 (bytes 203264-208384) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.06)
- Airball/Airball.adf: blocks 408-417 (bytes 208896-214016) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.12)
- Airball/Airball.adf: blocks 419-428 (bytes 214528-219648) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.10)
- Airball/Airball.adf: blocks 430-439 (bytes 220160-225280) outside the filesystem hold data, not code (opcode density 0.001, entropy 1.54)
- Airball/Airball.adf: blocks 441-442 (bytes 225792-226816) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.72)
- Airball/Airball.adf: blocks 445-450 (bytes 227840-230912) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.66)
- Airball/Airball.adf: blocks 452-461 (bytes 231424-236544) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.11)
- Airball/Airball.adf: blocks 463-469 (bytes 237056-240640) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.82)
- Airball/Airball.adf: blocks 474-483 (bytes 242688-247808) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.73)
- Airball/Airball.adf: blocks 485-494 (bytes 248320-253440) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.18)
- Airball/Airball.adf: blocks 496-505 (bytes 253952-259072) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.75)
- Airball/Airball.adf: blocks 507-509 (bytes 259584-261120) outside the filesystem hold data, not code (opcode density 0.001, entropy 1.63)
- Airball/Airball.adf: blocks 513-516 (bytes 262656-264704) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.33)
- Airball/Airball.adf: blocks 518-527 (bytes 265216-270336) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.97)
- Airball/Airball.adf: blocks 529-537 (bytes 270848-275456) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.19)
- Airball/Airball.adf: blocks 540-549 (bytes 276480-281600) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.65)
- Airball/Airball.adf: blocks 552-560 (bytes 282624-287232) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.11)
- Airball/Airball.adf: blocks 562-571 (bytes 287744-292864) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.41)
- Airball/Airball.adf: blocks 573-582 (bytes 293376-298496) outside the filesystem hold data, not code (opcode density 0.009, entropy 5.29)
- Airball/Airball.adf: blocks 584-593 (bytes 299008-304128) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.14)
- Airball/Airball.adf: blocks 595-604 (bytes 304640-309760) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.33)
- Airball/Airball.adf: blocks 606-615 (bytes 310272-315392) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.05)
- Airball/Airball.adf: blocks 617-626 (bytes 315904-321024) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.38)
- Airball/Airball.adf: blocks 628-637 (bytes 321536-326656) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.63)
- Airball/Airball.adf: blocks 639-648 (bytes 327168-332288) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.35)
- Airball/Airball.adf: blocks 650-659 (bytes 332800-337920) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.45)
- Airball/Airball.adf: blocks 661-670 (bytes 338432-343552) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.34)
- Airball/Airball.adf: blocks 672-681 (bytes 344064-349184) outside the filesystem hold data, not code (opcode density 0.007, entropy 5.89)
- Airball/Airball.adf: blocks 683-692 (bytes 349696-354816) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.16)
- Airball/Airball.adf: blocks 694-703 (bytes 355328-360448) outside the filesystem hold data, not code (opcode density 0.006, entropy 5.91)
- Airball/Airball.adf: blocks 705-714 (bytes 360960-366080) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.98)
- Airball/Airball.adf: blocks 716-725 (bytes 366592-371712) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.93)
- Airball/Airball.adf: blocks 727-736 (bytes 372224-377344) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.09)
- Airball/Airball.adf: blocks 738-747 (bytes 377856-382976) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.51)
- Airball/Airball.adf: blocks 749-758 (bytes 383488-388608) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.73)
- Airball/Airball.adf: blocks 760-769 (bytes 389120-394240) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.86)
- Airball/Airball.adf: blocks 771-780 (bytes 394752-399872) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.99)
- Airball/Airball.adf: blocks 782-791 (bytes 400384-405504) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.77)
- Airball/Airball.adf: blocks 793-802 (bytes 406016-411136) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.79)
- Airball/Airball.adf: blocks 804-813 (bytes 411648-416768) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.04)
- Airball/Airball.adf: blocks 815-824 (bytes 417280-422400) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.58)
- Airball/Airball.adf: blocks 826-830 (bytes 422912-425472) outside the filesystem hold data, not code (opcode density 0.001, entropy 1.79)
- Airball/Airball.adf: blocks 832-835 (bytes 425984-428032) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.95)
- Airball/Airball.adf: blocks 837-846 (bytes 428544-433664) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.09)
- Airball/Airball.adf: blocks 848-857 (bytes 434176-439296) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.68)
- Airball/Airball.adf: blocks 870-879 (bytes 445440-450560) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.85)
- Airball/Airball.adf: blocks 881-890 (bytes 451072-456192) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.44)
- Airball/Airball.adf: blocks 892-901 (bytes 456704-461824) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.61)
- Airball/Airball.adf: blocks 903-912 (bytes 462336-467456) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.21)
- Airball/Airball.adf: blocks 914-923 (bytes 467968-473088) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.77)
- Airball/Airball.adf: blocks 925-934 (bytes 473600-478720) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.22)
- Airball/Airball.adf: blocks 936-945 (bytes 479232-484352) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.41)
- Airball/Airball.adf: blocks 947-956 (bytes 484864-489984) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.31)
- Airball/Airball.adf: blocks 958-967 (bytes 490496-495616) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.05)
- Airball/Airball.adf: blocks 969-978 (bytes 496128-501248) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.72)
- Airball/Airball.adf: blocks 980-989 (bytes 501760-506880) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.07)
- Airball/Airball.adf: blocks 991-1000 (bytes 507392-512512) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.06)
- Airball/Airball.adf: blocks 1002-1011 (bytes 513024-518144) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.73)
- Airball/Airball.adf: blocks 1013-1022 (bytes 518656-523776) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.86)
- Airball/Airball.adf: blocks 1024-1033 (bytes 524288-529408) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.36)
- Airball/Airball.adf: blocks 1035-1044 (bytes 529920-535040) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.11)
- Airball/Airball.adf: blocks 1046-1055 (bytes 535552-540672) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.42)
- Airball/Airball.adf: blocks 1057-1066 (bytes 541184-546304) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.72)
- Airball/Airball.adf: blocks 1068-1077 (bytes 546816-551936) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.15)
- Airball/Airball.adf: blocks 1079-1088 (bytes 552448-557568) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.27)
- Airball/Airball.adf: blocks 1090-1099 (bytes 558080-563200) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.55)
- Airball/Airball.adf: blocks 1101-1110 (bytes 563712-568832) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.95)
- Airball/Airball.adf: blocks 1112-1121 (bytes 569344-574464) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.57)
- Airball/Airball.adf: blocks 1123-1131 (bytes 574976-579584) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.37)
- Airball/Airball.adf: blocks 1134-1143 (bytes 580608-585728) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.68)
- Airball/Airball.adf: blocks 1151-1154 (bytes 589312-591360) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.86)
- Airball/Airball.adf: blocks 1156-1165 (bytes 591872-596992) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.57)
- Airball/Airball.adf: blocks 1167-1171 (bytes 597504-600064) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.30)
- Airball/Airball.adf: blocks 1178-1187 (bytes 603136-608256) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.48)
- Airball/Airball.adf: blocks 1191-1198 (bytes 609792-613888) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.20)
- Airball/Airball.adf: blocks 1200-1209 (bytes 614400-619520) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.36)
- Airball/Airball.adf: blocks 1219-1220 (bytes 624128-625152) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.88)
- Airball/Airball.adf: blocks 1222-1231 (bytes 625664-630784) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.16)
- Airball/Airball.adf: blocks 1233-1238 (bytes 631296-634368) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.62)
- Airball/Airball.adf: blocks 1244-1253 (bytes 636928-642048) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.59)
- Airball/Airball.adf: blocks 1258-1264 (bytes 644096-647680) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.94)
- Airball/Airball.adf: blocks 1277-1286 (bytes 653824-658944) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.69)
- Airball/Airball.adf: blocks 1288-1297 (bytes 659456-664576) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.93)
- Airball/Airball.adf: blocks 1299-1308 (bytes 665088-670208) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.99)
- Airball/Airball.adf: blocks 1310-1319 (bytes 670720-675840) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.54)
- Airball/Airball.adf: blocks 1321-1330 (bytes 676352-681472) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.62)
- Airball/Airball.adf: blocks 1332-1341 (bytes 681984-687104) outside the filesystem hold data, not code (opcode density 0.000, entropy 7.03)
- Airball/Airball.adf: blocks 1343-1352 (bytes 687616-692736) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.82)
- Airball/Airball.adf: blocks 1365-1374 (bytes 698880-704000) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.18)
- Airball/Airball.adf: blocks 1387-1396 (bytes 710144-715264) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.85)
- Airball/Airball.adf: blocks 1398-1407 (bytes 715776-720896) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.95)
- Airball/Airball.adf: blocks 1409-1418 (bytes 721408-726528) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.75)
- Airball/Airball.adf: blocks 1420-1429 (bytes 727040-732160) outside the filesystem hold data, not code (opcode density 0.001, entropy 7.10)
- Airball/Airball.adf: blocks 1431-1440 (bytes 732672-737792) outside the filesystem hold data, not code (opcode density 0.010, entropy 3.28)
- Airball/Airball.adf: blocks 1442-1451 (bytes 738304-743424) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.01)
- Airball/Airball.adf: blocks 1453-1462 (bytes 743936-749056) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.18)
- Airball/Airball.adf: blocks 1464-1473 (bytes 749568-754688) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.80)
- Airball/Airball.adf: blocks 1475-1484 (bytes 755200-760320) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.66)
- Airball/Airball.adf: blocks 1486-1495 (bytes 760832-765952) outside the filesystem hold data, not code (opcode density 0.001, entropy 7.09)
- Airball/Airball.adf: blocks 1497-1506 (bytes 766464-771584) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.58)
- Airball/Airball.adf: blocks 1508-1517 (bytes 772096-777216) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.40)
- Airball/Airball.adf: blocks 1519-1528 (bytes 777728-782848) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.37)
- Airball/Airball.adf: blocks 1530-1539 (bytes 783360-788480) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.27)
- Airball/Airball.adf: blocks 1541-1550 (bytes 788992-794112) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.30)
- Airball/Airball.adf: blocks 1552-1561 (bytes 794624-799744) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.49)
- Airball/Airball.adf: blocks 1563-1572 (bytes 800256-805376) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.20)
- Airball/Airball.adf: blocks 1574-1583 (bytes 805888-811008) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.37)
- Airball/Airball.adf: blocks 1585-1594 (bytes 811520-816640) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.12)
- Airball/Airball.adf: blocks 1596-1605 (bytes 817152-822272) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.51)
- Airball/Airball.adf: blocks 1607-1616 (bytes 822784-827904) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.17)
- Airball/Airball.adf: blocks 1618-1627 (bytes 828416-833536) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.48)
- Airball/Airball.adf: blocks 1629-1638 (bytes 834048-839168) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.92)
- Airball/Airball.adf: blocks 1640-1649 (bytes 839680-844800) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.24)
- Airball/Airball.adf: blocks 1651-1660 (bytes 845312-850432) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.32)
- Airball/Airball.adf: blocks 1662-1671 (bytes 850944-856064) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.96)
- Airball/Airball.adf: blocks 1673-1682 (bytes 856576-861696) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.56)
- Airball/Airball.adf: blocks 1684-1693 (bytes 862208-867328) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.80)
- Airball/Airball.adf: blocks 1695-1704 (bytes 867840-872960) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.29)
- Airball/Airball.adf: blocks 1706-1715 (bytes 873472-878592) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.74)
- Airball/Airball.adf: blocks 1717-1726 (bytes 879104-884224) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.25)
- Airball/Airball.adf: blocks 1739-1748 (bytes 890368-895488) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.34)
- 5 packed track region(s), 36864 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `enter_infinite_loop_trap` (runtime segment an address): Modifies a global variable through a series of bitwise operations and conditional increments before calling itself recursively to enter an infinite loop. Kind: sabotage path / infinite loop trap. Pass path: the instructions following the call (bsr.w an address, bsr.w an address, subi.l #an address, d7).
- `set_drive_control_bits` (runtime segment an address): Clears bit three of a control byte and sets bits zero and seven in the CIAB port B register. Kind: disk hardware control/drive parameter manipulation. Pass path: the instruction immediately following the call (the code continues execution normally).
- `set_control_byte_and_call` (runtime segment an address): Writes fixed values into a control byte, sets bit zero while clearing other bits, then calls a routine. Kind: disk hardware/control byte check. Pass path: the code following the call (the instructions provided in the hunk).
- `modify_control_byte` (runtime segment an address): Clears bit one of a global variable and, if the result is non-zero, sets bit two. Kind: checksum/integrity trap. Pass path: the branch where DAT_00006dac == 0 (allowing execution to continue past the check).
- `initialize_dma_structure` (runtime segment an address): Initializes a series of records with values from a table and sets several hardware-related control flags. Kind: disk hardware/DMA initialization for custom track reading. Pass path: not determined.

## Functions


### Airball.adf:bootblock

- `issue_boot_io_request`: Boot block entry that issues one exec input/output request through the DoIO library call.

### logged disk load from Airball.adf boot-block load

- `refill_bit_reader_longword`: Reads the longword just before its input pointer and returns it shifted right one bit with the top bit set.
- `advance_bit_buffer_by_count`: Shifts a bit buffer right one bit per iteration for the number of times given in a count register, calling the refill routine whenever the buffer runs out.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_table_entry_then_call`: Calls a routine, copies a word from a table into a global variable using an index multiplied by twenty, and calls two more routines.
- `execute_recursive_logic_loop`: Performs a series of recursive calls to itself while decrementing a value and incrementing a counter within nested loops.
- `search_and_transform_data`: Searches a table for a specific word, then performs a bitwise transformation on a sequence of bytes and stores them in a destination buffer.
- `enter_infinite_loop_trap`: Modifies a global variable through a series of bitwise operations and conditional increments before calling itself recursively to enter an infinite loop.
- `enter_infinite_loop`: Enters an infinite loop that prevents further execution.
- `wait_status_then_set_word`: Waits until a status bit in a record is set, then stores the value two in a record word at a fixed offset and returns.
- `set_drive_control_bits`: Clears bit three of a control byte and sets bits zero and seven in the CIAB port B register.
- `set_control_byte_and_call`: Writes fixed values into a control byte, sets bit zero while clearing other bits, then calls a routine.
- `wait_for_bit_change`: The routine polls a global variable until a specific bit changes state.
- `wait_for_button_release`: Polls a global variable until a specific bit is cleared.
- `update_control_state_and_data`: Checks a global variable against a register value, and if they differ, updates the global variable and performs a series of recursive calls and memory writes to a structure via a pointer.
- `modify_control_byte`: Clears bit one of a global variable and, if the result is non-zero, sets bit two.
- `tiny_ori_ori_ori_ori`: Small routine of 4 instructions that merges 128 into d0, then merges 202 into d1, then merges 166 into d0, then merges 184 into d0.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `copy_table_entries_with_index`: Iterates through a table to copy and modify data into a global structure using an index derived from a register.
- `populate_global_structure_from_table`: Iterates through a source table to populate a global structure with data.
- `initialize_dma_structure`: Initializes a series of records with values from a table and sets several hardware-related control flags.
- `initialize_struct_fields`: Writes several values and pointers from registers and memory into a structure or a block of memory.
- `update_audio_pointers`: Updates several global pointers with values from a table based on bits set in a global variable.
- `update_entity_physics_and_counters_3`: Iterates through a list of entity structures to decrement internal timers and update coordinate values.
- `update_entity_motion_and_position`: Updates an entity's velocity and position by decrementing internal counters and iterating through linked lists of coordinate offsets.
- `copy_record_and_call`: Copies multiple words from a table into a record using an offset derived from a concatenated value, then calls another routine.
- `store_byte_and_call`: Stores a byte from a pointer into an offset of a record and performs an indirect call.
- `process_entity_command_list`: Decrements a life or duration counter within an entity structure and, if zero, iterates through a linked list of command opcodes to update entity properties, coordinates, or global state variables.
- `update_entity_physics_and_counters_2`: Iterates through a list of entities to update their position and velocity values based on incremental changes.
- `assign_entity_attribute_pointer`: Fetches a value from a table using an offset and stores it into an entity structure.
- `clear_bits_and_update_pointer`: Clears specific bits in a byte within a record and updates several fields with a value and a pointer retrieved from a table lookup.
- `update_entity_physics_and_timers`: Updates an entity's position using calculated velocity and decrements internal countdown timers.
- `update_entity_state_and_timers_2`: Decrements internal counters within an entity structure and updates coordinate values by reading from linked lists of data.
- `update_entity_timers_and_coordinates`: Iterates through a list of entity structures to decrement internal counters and update coordinate values based on data read from linked lists.
- `update_entity_state_and_timers`: Decrements internal timers and status flags within an entity structure, handling expiration by updating coordinate values or clearing specific bits.
- `set_bit_and_update_pointer`: Sets a specific bit in a byte within a record and updates several fields with a value and a pointer retrieved from a table lookup.
- `update_entity_attribute_pointer`: Clears specific bits in a byte within an entity record and updates several fields with a value and a pointer retrieved from a table lookup.
- `update_entity_physics_and_counters`: Updates position and velocity values for multiple entities by iterating through a list and applying incremental changes.
- `lookup_shifted_table_value`: Sums two record fields with an input value, counts how many steps of ninety-six it takes to drop below zero, then returns a base field plus a table value shifted right by that count.
- `store_concatenated_word`: Concatenates a byte from a register and a byte from a pointer, then stores the resulting word into a record.
