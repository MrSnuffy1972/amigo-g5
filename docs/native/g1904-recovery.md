# Game 1904 (hunter (1991)(activision)[cr company][t +5 legend]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01904). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (935678 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 14 of 14 units identical (bytes and relocations), 30 raw-word lines |
| Functions named and described | pass, 161 of 161 named (0 author symbols), 161 of 161 described (10 by rule, 21 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1494 of 1495 (99.93%) |
| Recovered without load address (counted in proof 1, flagged) | 14848 bytes, 5 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2695 of 2850 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 46054 |
| code | 13980 |
| data | 509196 |
| padding | 880 |
| unused | 365568 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Hunter (1991)(Activision)[cr Company][t +5 Legend].adf:bootblock | 1 | 26 | 1 |
| logged disk load from Hunter (1991)(Activision)[cr Company][t +5 Legend].adf boot-block load | 1 | 92 | 3 |
| logged disk load from Hunter (1991)(Activision)[cr Company][t +5 Legend].adf boot-block load | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1590 | 92 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 545 | 44 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 104 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 124 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 218 | 5 |
| Hunter (1991)(Activision)[cr Company][t +5 Legend].adf bytes 33792-44544 (code never loaded by a reference run) | 1 | 103 | 1 |
| Hunter (1991)(Activision)[cr Company][t +5 Legend].adf bytes 56320-56832 (code never loaded by a reference run) | 1 | 0 | 0 |
| Hunter (1991)(Activision)[cr Company][t +5 Legend].adf bytes 81920-82944 (code never loaded by a reference run) | 1 | 25 | 2 |
| Hunter (1991)(Activision)[cr Company][t +5 Legend].adf bytes 146432-146944 (code never loaded by a reference run) | 1 | 0 | 0 |
| Hunter (1991)(Activision)[cr Company][t +5 Legend].adf bytes 148480-148992 (code never loaded by a reference run) | 1 | 1 | 1 |
| Hunter (1991)(Activision)[cr Company][t +5 Legend].adf bytes 149504-151040 (code never loaded by a reference run) | 1 | 13 | 1 |

## Cases handled

- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: custom-format (non-DOS) disk, 1034 non-empty blocks: matched against track-loaded segments
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: the boot block reads 17920 bytes from disk offset 675840 to an address (found statically); recovered as a unit
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: the boot block reads 5632 bytes from disk offset 22528 to an address (found statically); recovered as a unit
- runtime segment an address: 725 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 235 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 426 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 3 first-run image unit(s) in 1 version group(s); 1074 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: bytes 33792-44544 look like code no run loaded: recovered position-independently (load address unknown)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: bytes 56320-56832 look like code no run loaded: recovered position-independently (load address unknown)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: bytes 81920-82944 look like code no run loaded: recovered position-independently (load address unknown)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: bytes 146432-146944 look like code no run loaded: recovered position-independently (load address unknown)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: bytes 148480-148992 look like code no run loaded: recovered position-independently (load address unknown)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: bytes 149504-151040 look like code no run loaded: recovered position-independently (load address unknown)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: 8 logged loads cover 38400 bytes of the image
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: 47 track-loaded blocks found in recovered runtime segments and classified through them
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 33-42 (bytes 16896-22016) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.67)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 55-64 (bytes 28160-33280) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.81)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 88-108 (bytes 45056-55808) outside the filesystem hold data, not code (opcode density 0.054, entropy 7.09)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 112-119 (bytes 57344-61440) outside the filesystem hold data, not code (opcode density 0.046, entropy 4.63)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 121-130 (bytes 61952-67072) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.56)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 133-134 (bytes 68096-69120) outside the filesystem hold data, not code (opcode density 0.051, entropy 5.24)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 136-136 (bytes 69632-70144) outside the filesystem hold data, not code (opcode density 0.051, entropy 2.50)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 143-152 (bytes 73216-78336) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.78)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 202-202 (bytes 103424-103936) outside the filesystem hold data, not code (opcode density 0.027, entropy 4.41)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 208-208 (bytes 106496-107008) outside the filesystem hold data, not code (opcode density 0.023, entropy 3.67)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 229-230 (bytes 117248-118272) outside the filesystem hold data, not code (opcode density 0.045, entropy 4.50)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 242-252 (bytes 123904-129536) outside the filesystem hold data, not code (opcode density 0.033, entropy 3.67)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 264-274 (bytes 135168-140800) outside the filesystem hold data, not code (opcode density 0.013, entropy 2.33)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 297-306 (bytes 152064-157184) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.39)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 319-328 (bytes 163328-168448) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.54)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 341-350 (bytes 174592-179712) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.64)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 385-394 (bytes 197120-202240) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.56)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 407-416 (bytes 208384-213504) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.78)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 429-438 (bytes 219648-224768) outside the filesystem hold data, not code (opcode density 0.002, entropy 7.16)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 473-481 (bytes 242176-246784) outside the filesystem hold data, not code (opcode density 0.019, entropy 5.64)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 504-504 (bytes 258048-258560) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.39)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 517-526 (bytes 264704-269824) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.38)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 539-548 (bytes 275968-281088) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.07)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 627-636 (bytes 321024-326144) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.78)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 649-658 (bytes 332288-337408) outside the filesystem hold data, not code (opcode density 0.000, entropy 7.10)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 660-669 (bytes 337920-343040) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.12)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 671-680 (bytes 343552-348672) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.44)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 682-691 (bytes 349184-354304) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.89)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 693-702 (bytes 354816-359936) outside the filesystem hold data, not code (opcode density 0.009, entropy 7.12)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 704-713 (bytes 360448-365568) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.03)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 715-724 (bytes 366080-371200) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.79)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 726-735 (bytes 371712-376832) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.95)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 748-757 (bytes 382976-388096) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.96)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 770-779 (bytes 394240-399360) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.25)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 781-790 (bytes 399872-404992) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.56)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 792-801 (bytes 405504-410624) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.25)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 803-812 (bytes 411136-416256) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.78)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 814-823 (bytes 416768-421888) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.75)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 825-834 (bytes 422400-427520) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.73)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 836-845 (bytes 428032-433152) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.78)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 847-856 (bytes 433664-438784) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.61)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 858-867 (bytes 439296-444416) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.90)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 869-878 (bytes 444928-450048) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.58)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 880-889 (bytes 450560-455680) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.98)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 891-900 (bytes 456192-461312) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.60)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 902-903 (bytes 461824-462848) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.28)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 913-922 (bytes 467456-472576) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.93)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 935-944 (bytes 478720-483840) outside the filesystem hold data, not code (opcode density 0.001, entropy 7.06)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 946-955 (bytes 484352-489472) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.51)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 957-966 (bytes 489984-495104) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.69)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 968-977 (bytes 495616-500736) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.90)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 979-988 (bytes 501248-506368) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.00)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 990-993 (bytes 506880-508928) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.59)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 998-999 (bytes 510976-512000) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.74)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1001-1002 (bytes 512512-513536) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.48)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1012-1021 (bytes 518144-523264) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.21)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1034-1043 (bytes 529408-534528) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.79)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1056-1065 (bytes 540672-545792) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.01)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1078-1087 (bytes 551936-557056) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.40)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1100-1109 (bytes 563200-568320) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.37)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1111-1120 (bytes 568832-573952) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.72)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1122-1131 (bytes 574464-579584) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.78)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1133-1142 (bytes 580096-585216) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.55)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1144-1153 (bytes 585728-590848) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.05)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1155-1164 (bytes 591360-596480) outside the filesystem hold data, not code (opcode density 0.005, entropy 1.82)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1166-1175 (bytes 596992-602112) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.39)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1177-1186 (bytes 602624-607744) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.25)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1188-1197 (bytes 608256-613376) outside the filesystem hold data, not code (opcode density 0.008, entropy 5.18)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1199-1208 (bytes 613888-619008) outside the filesystem hold data, not code (opcode density 0.001, entropy 1.50)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1210-1219 (bytes 619520-624640) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.16)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1221-1230 (bytes 625152-630272) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.00)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1232-1241 (bytes 630784-635904) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.06)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1243-1252 (bytes 636416-641536) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.77)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1254-1263 (bytes 642048-647168) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.64)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1265-1274 (bytes 647680-652800) outside the filesystem hold data, not code (opcode density 0.004, entropy 2.94)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1276-1285 (bytes 653312-658432) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.13)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1287-1296 (bytes 658944-664064) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.39)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1309-1311 (bytes 670208-671744) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.15)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1316-1318 (bytes 673792-675328) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.90)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1375-1384 (bytes 704000-709120) outside the filesystem hold data, not code (opcode density 0.005, entropy 1.10)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1397-1406 (bytes 715264-720384) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.81)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1419-1428 (bytes 726528-731648) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.72)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1441-1450 (bytes 737792-742912) outside the filesystem hold data, not code (opcode density 0.002, entropy 1.54)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1463-1472 (bytes 749056-754176) outside the filesystem hold data, not code (opcode density 0.003, entropy 1.35)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1485-1489 (bytes 760320-762880) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.40)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1536-1538 (bytes 786432-787968) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.23)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1551-1560 (bytes 794112-799232) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.71)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1573-1582 (bytes 805376-810496) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.15)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1595-1604 (bytes 816640-821760) outside the filesystem hold data, not code (opcode density 0.005, entropy 2.49)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1617-1626 (bytes 827904-833024) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.14)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1639-1648 (bytes 839168-844288) outside the filesystem hold data, not code (opcode density 0.001, entropy 1.76)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1661-1670 (bytes 850432-855552) outside the filesystem hold data, not code (opcode density 0.001, entropy 1.98)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1683-1692 (bytes 861696-866816) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.87)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1705-1711 (bytes 872960-876544) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.45)
- hunter (1991)(activision)[cr company][t +5 legend]/Hunter (1991)(Activision)[cr Company][t +5 Legend].adf: blocks 1756-1758 (bytes 899072-900608) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.11)
- 6 never-loaded region(s), 14848 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 8 packed track region(s), 40960 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `select_port_mask_from_flag` (runtime segment an address): Sets a state byte to one of two fixed mask values based on a flag, then writes the complement of that mask, limited to seven bits, to a hardware port. Kind: disk hardware/port configuration check. Pass path: the branch where the mask is applied correctly to allow subsequent hardware access.
- `step_counter_and_toggle_flag` (runtime segment an address): Steps a wrapping counter down by two, resetting it to a fixed maximum when it drops below zero, then flips bit four of a state word between two calls to other routines. Kind: checksum or state-based integrity check. Pass path: the branch following the call that does not trigger the failure/exit logic (in this context, the code continues execution if the flag/counter state is valid).

## Functions


### Hunter (1991)(Activision)[cr Company][t +5 Legend].adf:bootblock

- `boot_block_io_request`: Boot-block entry that sends an I/O request through the operating system's device-I/O library call; only that call could be recovered, as the full logic did not decompile.

### logged disk load from Hunter (1991)(Activision)[cr Company][t +5 Legend].adf boot-block load

- `decompress_packed_data_stream`: Reads a compressed bitstream from a source buffer, uses a bit-reader to extract variable-length bit sequences, and writes the resulting bytes to a destination buffer.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from Hunter (1991)(Activision)[cr Company][t +5 Legend].adf boot-block load

- `disable_interrupts_and_copy_block`: Disables all interrupts and DMA channels by writing the clear mask to their control registers, then copies a block of longwords into low memory up to a fixed end address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_record_fields_by_index`: Looks up a 22-byte record by index and copies its five fields, one of them adjusted by one, into the state block.
- `derive_state_fields_from_record`: Calls three unnamed routines, copies six fields from a source record into state words, derives several scaled and multiplied values from them, and clears one longword in the state block.
- `store_coordinates_with_offsets`: Stores two input values and their offset-by-constant variants in the state block, plus two further words supplied by the caller.
- `copy_record_into_state_slot`: Copies eleven words from a 22-byte table record selected by one input into a state slot selected by another input.
- `load_indexed_record_into_state`: Selects a 46-byte record by index, copies its header longs into the state block and a counted list of longs into a working buffer, stores its position and computes two packed bit words from state fields.
- `frag_subq_bmi_nop_rts`: Short code fragment that subtracts 1 from d3, then branches if the last result was negative, then pauses for one no-operation step, then returns.
- `fill_four_spaced_values`: Writes four longword values into a buffer at eight-byte spacing, each one 2400 more than the last, starting from a global base value.
- `fill_stepped_values_in_records`: Writes a series of longword values into records at eight-byte spacing, each value increased by a step taken from the state block, for a count held in the state block.
- `copy_sixteen_longs_into_state`: Copies sixteen longwords from a 64-byte-spaced table block selected by an index into the state block at a fixed offset.
- `copy_sixteen_longs_to_base`: Stores the global base address in the state block and copies sixteen longwords from a table block selected by an index into the global base area.
- `increment_mark_and_clear_flag`: Adds eight to a counter word, sets the top bits of a word in a table entry, clears a state flag bit and a state longword, and returns its input with the low byte set.
- `set_top_bits_clear_flag`: Sets the top bits of a word in a table entry, clears a state flag bit and a state longword, and returns its input with a fixed mask of bits set.
- `clear_flag_and_return_masked`: Clears a state flag bit and a state longword, and returns its input with a fixed mask of bits set.
- `absolute_difference_from_target`: Returns the absolute difference between a state word and a target value: the target is fifteen when a flag bit is set, otherwise another state word.
- `run_routines_and_await_idle`: Clears a state longword, runs three unnamed routines, runs one more with a temporarily set state word, and then spins while a status bit stays set.
- `run_cleared_limit_then_wait`: Temporarily clears a state word, calls an unnamed routine, restores the word, calls another unnamed routine, then spins while a status bit stays set.
- `wait_while_state_bit_set`: Spins while bit five of a state byte stays set, returning once it is clear.
- `set_or_clear_status_bits`: When the input is nonzero, calls an unnamed routine and sets a status bit; otherwise clears two status bits.
- `store_four_parameter_words`: Stores four input values into consecutive state words, with the first input also copied into a neighbouring word.
- `negate_and_reload_parameter`: Negates one state word, copies another state word into a neighbouring word, and sets a status bit.
- `advance_sequence_step`: Advances a sequenced step by reloading a step counter and decrementing a repeat counter, clearing or reversing status bits when the repeat count runs out.
- `step_and_write_masked_entries`: Moves a state counter by the input amount one step at a time while it stays within a range, calling an unnamed routine on each step, then writes a value into table entries not masked by a state longword.
- `advance_counter_and_run_steps`: Adds an input value to a state word, then calls an unnamed routine sixteen times, and once more for each unit by which a state count exceeds sixteen.
- `add_signed_delta_to_bytes`: Adds a value taken from a second table to each of two bytes at a pointer, negating that value when the input is negative.
- `read_third_byte_high_nibble`: Reads three bytes from a pointer, advancing past them, and returns the high nibble of the last byte read.
- `compute_nibble_deltas_between_tables`: Writes, for each entry of a counted range, the shifted difference between a byte from one table and the byte at the same offset in a second table.
- `copy_three_byte_groups`: Copies a counted number of three-byte groups from a fixed source table into the working buffer.
- `clear_table_after_busy_wait`: Spins until a busy flag in the state block clears, then zeroes a table of three-byte groups whose count comes from the state block, and finally clears a word field in the state block.
- `expand_nibbles_into_bytes`: Takes a 16-bit value and writes its four nibbles into the high halves of consecutive bytes, two bytes per value, for two separate buffers.
- `extract_high_nibbles_from_table`: Initializes two state fields, calls three unnamed routines, then copies the high nibble of each byte in a counted table of three-byte groups into a buffer.
- `build_slot_index_from_list`: Clears a 16-byte slot table and calls an unnamed routine per slot, then walks a null-terminated linked list, marking slots for records whose key falls in a 64-value window and storing the match count back in the state block.
- `update_status_flag_timers`: Counts down several timers in the state block, each gated by a bit in a status byte; expired timers set flags or call an unnamed routine, and the status byte is written back.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 184 into d0, then merges 0 into d0.
- `invert_flagged_table_entries`: Walks a table backwards in eight-byte steps for seven entries, and for each entry that is zero or has its matching bit set in a state byte, inverts the entry and calls an unnamed routine.
- `write_strided_nibble_column`: Sets a flag in the state block, then merges two masked bits into fourteen bytes spaced forty bytes apart when a descriptor flag is set, otherwise writes a single byte.
- `step_counter_toward_target`: When a flag bit is set and two guard bytes are clear, decays a byte counter toward zero and moves a word counter one step toward a fixed target value.
- `write_running_sums_in_pairs`: Walks a list of value pairs until the second value of a pair is negative, storing a running total in the first slot of each pair and adding each pair's second value to the total.
- `skip_records_until_negative`: Advances through eight-byte records while the long value at offset four of each record is non-negative, doing no other work, and stops at the first negative record.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `zero_record_table_with_tag`: Calls two unnamed routines, then zeroes consecutive thirty-two-byte records up to a computed end position and writes a fixed four-byte signature into the first word of low memory.
- `repeat_calls_while_negative_entry`: Calls an unnamed routine, looks up a signed value from a table by an index, and repeats a pair of unnamed routine calls while that value is negative and a status bit in the state block stays set, then returns zero.
- `pack_sprite_position_words`: Computes a sprite's screen position from two pairs of state values and, when a sprite data pointer is set, stores its address and packs the start and stop positions into the two sprite control words.
- `save_registers_and_disable_interrupts`: Saves several custom-chip interrupt, DMA and audio/disk control registers into the state block, disables all interrupts, points seven interrupt vectors at a shared routine, clears a state area and sets a bit in a control byte.
- `initialize_record_list_buffer`: Initializes a four-word buffer with two starting values and a negative end marker, records the buffer's position in the state block, and sets a control word.
- `tiny_nop`: Small routine of 1 instruction that does nothing for one instruction.
- `mask_status_and_set_limit`: Copies a masked bit pattern from one state word into another and sets a second state word to a fixed maximum value.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (nop) and then falls through into the code that follows, with no return of its own.
- `mask_and_store_word`: Reads a word from an offset of thirty in a structure pointed to by a6, applies a bitwise AND mask, stores the result at offset one hundred fifty-six in the same structure, sets a word at offset three hundred eighty-four to all ones, and returns.
- `repeat_call_until_zero`: Calls an unnamed routine repeatedly until it returns zero.
- `test_and_set_busy_flag`: Spins while the top bit of a state byte is already set, setting it on each pass, so it returns only once the flag was previously clear.
- `acquire_status_flag_lock`: Spins while a status-bit mask in the state block is already set, setting it on each pass, so it returns only once that bit was clear.
- `read_masked_status_bits`: Returns a state-block byte ANDed with a fixed two-bit mask.
- `repeat_call_until_flag_set`: Repeatedly calls an unnamed routine until the condition flag it leaves behind is set.
- `wait_until_status_bit_clears`: Spins while one particular bit in a state byte stays set, returning once that bit is clear.
- `find_region_containing_point`: Scans a table of rectangular region records for the one containing the current coordinates and stores its index.
- `mask_status_conditional_limit`: Stores a masked copy of one state word into another, and when the masked result is nonzero also sets a third state word to a fixed maximum value.
- `decode_serial_key_event`: When a controller interrupt flag is set, pauses briefly, clears the handshake bit and decodes the serial input byte into a key code and up or down state; pressed codes below a limit set a flag and call an unnamed routine, and other codes go to a second unnamed routine.
- `forward_call_to_routine`: Calls one unnamed routine and returns, with no logic of its own.
- `translate_input_and_match_list`: Stores an input byte and a table-translated copy in the state block, then, when a flag is set, searches a list of eight stored bytes; with no match it copies four words from a computed table entry into the state block, and a match leads to a second region search.
- `store_char_in_ring_buffer`: Writes a byte into a 32-entry ring buffer at the current write index and advances the index.
- `copy_text_passing_control_codes`: Copies a text string into a buffer until a zero byte; control characters and high-bit codes are passed to unnamed routines, other bytes are copied, and the end is marked with an asterisk.
- `format_number_or_plain_text`: Writes either a signed decimal number, suppressing leading zeros and adding a minus sign when requested, or copies text until an asterisk, then ends the output with a space.
- `copy_text_until_asterisk`: Copies bytes from one buffer to another until an asterisk is reached, then writes a space.
- `copy_asterisk_terminated_string`: Copies bytes from one buffer to another up to and including an asterisk terminator, subject to a count limit, and writes an asterisk at the end.
- `skip_asterisk_terminated_strings`: Looks up a start position in a table of 32 offsets and advances past a given number of asterisk-terminated strings to reach the wanted one.
- `advance_position_offset`: Calls an unnamed routine, clears a state long if it is set, then calls a second unnamed routine and advances a position word by a step value while leaving its companion word unchanged.
- `draw_text_glyph_with_codes`: Interprets a text string with embedded control codes; for ordinary characters it draws an eight-row glyph from a font table into the bitplane, shifted and masked by a per-character width and merged or inverted according to state flags, then returns the advance.
- `clear_rectangle_bytes`: Zeroes a rectangle of bytes in a bitplane, with the row count taken from a state word plus an input value and the row width and row stride supplied by the caller.
- `calculate_offset_via_multiplication`: Performs a series of arithmetic operations involving bit shifts, subtractions, and a multiplication of two values from a structure to calculate a result.
- `forward_call_preserving_registers`: Calls one unnamed routine and returns the two result registers it left, passing them through unchanged.
- `sum_glyph_widths_until_byte`: Walks a byte string until it reaches a given terminator byte, adding each character's width from a width table plus one to a running total, and returns that total.
- `format_signed_number_digits`: Converts a signed 16-bit value to its absolute value, writes its five decimal digits as characters into a fixed buffer, and returns a state word.
- `write_signed_number_digits`: Converts a signed 16-bit value to its absolute value and writes its five decimal digits as characters into a fixed buffer.
- `write_long_number_digits`: Writes the absolute value of a 32-bit signed number as its last five decimal digits into a fixed buffer.
- `convert_integer_to_ascii_string`: Converts a signed longword in d2 into a sequence of decimal digits and stores them in a table starting at an offset from a pointer calculated from d3, otherwise it stores two ASCII digits at that location and returns.
- `convert_digit_string_to_value`: Sets a flag bit in the state block, then converts a string of decimal digit characters into a number by weighting each digit with a value from a table.
- `update_input_state_flags`: Copies a masked status word into a state word, then, depending on which flag bits remain set, calls unnamed routines, decrements two counters and dispatches through a stored handler pointer.
- `dispatch_active_handler`: Calls one of two handlers held in a dispatch table, chosen by the top bit of a state byte, and returns the handler's result, or the all-ones value when neither bit is set.
- `update_timed_input_state`: Runs one update tick: calls an unnamed routine, advances a counter when a state bit is set and a limit check passes, runs a sub-routine, and calls an unnamed routine on the rising edge of a latch bit in a state byte.
- `read_controller_and_clamp_position`: Reads the controller input port bits into a direction code, moves two position words by the change in the readings and clamps them between zero and per-axis limits, then packs button and direction bits into a state byte.
- `step_through_pending_entries`: Reads entries from a pointer-driven list, counting each down and advancing to the next when its count is exhausted, calls unnamed routines for entries whose top bit is clear, and stores the first entry with that bit set in the state block before a short countdown delay.
- `spin_sixteen_bit_delay`: Spins a 16-bit counter through a full cycle, creating a short fixed delay.
- `clear_state_flag_bit`: Clears a single flag bit in a state byte.
- `write_pixel_pattern_and_update`: Writes a longword pattern into four byte slots spaced two apart, loads state words from a table selected by the input, and when a table flag is set, updates a state bit and copies a set of counters and positions into the state block.
- `advance_position_counters`: Decrements a state byte counter, copies a set of counters and positions from the state block into the working block, advances two position values and sets a flag word.
- `check_bounds_and_setup_segment`: Checks that an input position lies inside the state block's bounds, stores a derived offset and count, and calls an unnamed routine to set up the segment; returns zero when a segment was set up, otherwise the all-ones value.
- `setup_segment_from_inputs`: Stores derived offset and width values from its inputs into the state block, computes a counted sum of the row stride, and calls an unnamed routine to set up the segment; returns zero when a segment is set up, otherwise the all-ones value.
- `split_value_count_and_mask`: Splits an input value into a doubled count taken from its upper bits and a bit mask derived from its lower bits, stores the mask and shifted values in the state block, and returns the count.
- `store_mask_and_return_count`: Stores a value from the caller in the state block and returns the input shifted right by four bits and doubled.
- `start_channel_with_busy_lock`: Waits for a busy bit in the state block to clear while setting it, installs a handler pointer and initialises a set of words in an associated block from state values.
- `start_channel_then_await_release`: Acquires the busy bit by spinning until it is clear, installs a handler, initialises a block of words from state values with a different fixed value, then spins until the busy bit is clear again.
- `clear_buffer_words_and_tail`: Zeroes a buffer of 16-bit words, with the word count taken as half the input value, and clears one extra byte when the input is odd.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_four_routines_while_negative`: Calls four unnamed routines in sequence and repeats the sequence for as long as the negative condition flag left by the previous pass remains set.
- `set_error_flag_and_tag`: Sets a flag bit in a state byte, writes a fixed four-byte error marker to a global pointer, calls two unnamed routines, clears a second flag bit and returns zero.
- `compare_long_sequences`: Compares two sequences of 32-bit words for the given count, returning zero when all words match and the all-ones value at the first mismatch.
- `load_table_entry_into_state`: Stores its inputs in the state block, then looks up a 32-byte table entry by index, copying a shifted word and a byte from that entry into the state block and returning a word from the entry.
- `assign_slot_base_addresses`: Walks up to fifteen slot records, clearing a request flag in each; for requested slots it calls an unnamed routine, stores the running base address in the slot's pointer table and advances the base by a stride, stopping once the base passes a fixed limit.
- `repeat_calls_while_negative_entry_2`: Repeats a pair of unnamed routine calls while a negative value looked up from a table by an index and a status bit both remain set, then returns zero.
- `check_table_entry_and_call`: Saves registers to the stack, checks a bit in a memory location, and calls another routine if certain conditions are met, otherwise it calls another routine and restores registers.
- `return_zero`: Clears register d0 and returns.
- `check_and_decrement_value`: Branches if a value is negative, otherwise it pushes a pointer to the stack, masks a byte from a global variable, and decrements it twice, branching to other routines if the result is zero.
- `copy_or_draw_table_segment`: If a state flag is set, copies a counted run of longs from a source table into the destination; otherwise it wraps a column coordinate at eighty and calls an unnamed routine with the adjusted coordinates to draw the segment.
- `scan_rows_or_run_handlers`: Saves its arguments, then either writes six row-select patterns to an input port and returns, or runs repeated handler calls over a counted span, and clears a table of 4096 words once the counted loop finishes.
- `write_row_select_patterns`: Writes six fixed row-select patterns to an input port in sequence, ending on the idle pattern.
- `call_three_routines_in_order`: Calls three unnamed routines in sequence.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d4, then returns.
- `wait_while_port_bit_set`: Waits while bit five of an input port byte stays set, counting down from a 16-bit value and returning when the bit clears or the count runs out.
- `select_port_mask_from_flag`: Sets a state byte to one of two fixed mask values based on a flag, then writes the complement of that mask, limited to seven bits, to a hardware port.
- `step_counter_and_toggle_flag`: Steps a wrapping counter down by two, resetting it to a fixed maximum when it drops below zero, then flips bit four of a state word between two calls to other routines.
- `alternate_calls_until_flag_clear`: Calls two subroutines in a loop whose repetition depends on a saved condition flag; when the flag is set it enters an inner loop that calls a routine indefinitely and only returns if the callee does.
- `check_joystick_bit`: Tests bit 4 of a hardware register and, if set, clears a 16-bit word in a global variable before returning; otherwise it returns.
- `step_value_toward_target`: Compares a stored target word with a current word; when they differ it sets the current word to the target and then calls a subroutine and runs a loop of two subroutine calls across the difference.
- `count_down_loop_with_calls`: Makes one subroutine call, then runs a count-down loop over a register value, calling two subroutines on each pass until the counter reaches minus one.
- `delay_by_counting_down`: Spins a 16-bit down-counter from a fixed starting value of about 4600 to minus one, consuming time with no other effect.
- `write_byte_twice_to_address`: Writes two byte values in sequence to the same memory location through an address register, so only the second value remains stored.
- `stub_empty_return`: Contains only a return instruction, so calling it has no effect on program state.
- `init_sync_parameter_block`: Stores a value passed in one register into a block of state fields together with several constants, including a marker word that later decoding routines search for, two code addresses, and a long value copied from a global.
- `program_dma_transfer_registers`: Writes a sequence of values through a hardware register base pointer: a data pointer taken from a stored address, a transfer length, a sync word and control bits, then sets an enable word to start the transfer.
- `poll_status_bit_with_timeout`: Calls a subroutine, then polls a hardware status word for a bit until a bounded count runs out, and finally clears the transfer length register.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `halt_in_infinite_loop`: Loops forever without returning, so any code after it is unreachable.
- `start_transfer_and_wait_status`: Programs the same transfer registers as its neighbour, starts a transfer using a value returned by a subroutine, polls a status bit with a bounded count, then clears the transfer length register.
- `copy_sector_table_into_buffer`: Copies ten blocks of 512 bytes each from a table of source pointers into a destination buffer, stopping and returning zero when the destination reaches a limit computed from two stored values; returns minus one when a preceding check fails.
- `decode_sync_delimited_sectors`: Scans a bit stream for the marker word, reads each sector header and decodes 128 longwords by merging odd and even bit halves, then checks a checksum and stores each decoded buffer in a table indexed by sector number.
- `decode_sector_records_until_error`: Reads sector records one after another, decodes 128 longwords per sector by merging odd and even bit halves under a mask held in a register, verifies an exclusive-or checksum and stores each decoded buffer by sector number; stops at the first bad checksum or out-of-range header.
- `merge_odd_even_bits`: Recombines a pair of longwords into one by masking the second with a register mask and merging in the first, shifted left one bit, so the odd and even bit halves are interleaved again.
- `stub_return_stack_value`: Returns the eight-byte value found on the stack without change; the code has no other effect.
- `stub_empty_return_2`: The bytes here decode as a bare return with no effect; the routine is not reached in the reference run and most likely sits in a data gap.
- `set_global_byte_and_call`: Sets a global byte to the sum of a stored byte and twice a stored word, then calls a subroutine.
- `fill_track_buffer_sync_pattern`: Fills a buffer with a fixed alternating-bit pattern, then writes the sync marker pair at regular positions across the buffer, storing a running value and calling subroutines for each segment.
- `clear_high_bit_if_odd`: Clears the top bit of a byte when the byte just before it has its lowest bit set; otherwise leaves it unchanged.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `clear_sign_bits_in_pairs`: Walks two parallel longword buffers, clearing the top bit of each stored value when the byte before the matching location has its lowest bit set, then calls a subroutine once after the loop.
- `call_two_routines_in_sequence`: Calls two subroutines one after the other and returns, with no other work.
- `frag_move_lsl_lsr_or_eor_rts`: Short code fragment that copies d7 to d3, then shifts d3 left by 1, then copies d0 to d1, then copies d0 to d2, then shifts d1 left by 1, then shifts d2 right by 1, then merges d1 into d2, then exclusive-ors d2 into d3, then merges d0 into d3, then returns.
- `build_crc16_lookup_table`: Fills a 256-entry lookup table in the global area: each entry holds the CRC-16 remainder of its index after eight shift steps using the standard CCITT polynomial, stored as separate high-byte and low-byte arrays.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `save_registers_and_call_exec`: Saves registers d0 through d7 and a0 through a6 to the stack, calls a routine at an offset from a pointer in a6, and sets d0 to zero.
- `call_library_function`: Calls a routine at an offset from a6 and sets d0 to zero.
- `initialize_memory_and_copy_tables`: Sets a series of 16-bit values and a 5 in a global variable, clears a memory block, and executes a nested loop to copy data from one table to another based on a calculated offset.
- `call_multiple_subroutines`: Saves registers to the stack, calls six different subroutines, restores the registers, and jumps to a specific address.
- `compare_keyboard_data`: Reads a byte from the keyboard serial data register and compares it to a byte in a global variable, otherwise it returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `copy_and_reorder_tables`: Runs two loops to copy and reorder data between several tables using various pointers, then performs a second loop to copy words from a table to another location before storing a value into a global variable.
- `update_table_and_shift_bits`: Decrements a global counter and, if it reaches zero, selects a value from a table based on a byte comparison to update two global variables before copying a block of data to a destination.
- `decrement_and_test_word`: Decrements a 16-bit value in a global variable and, if the result is not zero, branches to another routine; otherwise it decrements a register and returns.
- `copy_and_modify_tables`: Copies 683 words from a source table to a destination table using a loop, then performs a second loop that copies 57 words from a different source to a destination while decrementing a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_music_sample_table`: Searches a byte table for the smallest value, then builds a pointer table by linking zeroed entries to sample start addresses indexed from a second table, sets bit 1 of a global variable, clears several global variables, and returns.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `update_audio_channel_parameters`: Increments a global counter and, if it reaches a threshold, calls another routine four times to initialize audio hardware registers with specific data from a table.
- `process_command_packet_data`: Parses a command packet to fetch data from a table using an index derived from the packet bytes, then updates hardware registers including the audio controller and DMA controller.
- `parse_command_packet`: Reads a 16-bit value from a structure in a6 and uses the lower 4 bits of the first byte to determine an action.

### Hunter (1991)(Activision)[cr Company][t +5 Legend].adf bytes 33792-44544 (code never loaded by a reference run)

- `zero_fill_memory_block`: Writes zero into a block of about 2850 consecutive 16-bit words starting at a fixed memory location, then returns.

### Hunter (1991)(Activision)[cr Company][t +5 Legend].adf bytes 81920-82944 (code never loaded by a reference run)

- `stub_empty_return_3`: The decompiler shows only a return with no body, so calling it has no effect in the data region.
- `scan_table_for_larger_entry`: When the input value is below a fixed limit, scans forward through a table in the global area until it finds an entry greater than the input; otherwise returns at once.

### Hunter (1991)(Activision)[cr Company][t +5 Legend].adf bytes 148480-148992 (code never loaded by a reference run)

- `stub_empty_return_4`: Contains only a return with no operations; the bytes of this data region decode to a no-op.

### Hunter (1991)(Activision)[cr Company][t +5 Legend].adf bytes 149504-151040 (code never loaded by a reference run)

- `scale_and_rotate_value`: Doubles its input value several times in succession, then returns the product shifted left and combined with its own shifted upper bits.
