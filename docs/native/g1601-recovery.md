# Game 1601 (plotting (1990)(ocean)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01601). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (940928 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 42 raw-word lines |
| Functions named and described | pass, 268 of 268 named (0 author symbols), 268 of 268 described (11 by rule, 245 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3193 of 3193 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6463 of 6467 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 6964 |
| code | 33088 |
| data | 210140 |
| padding | 1072 |
| unused | 689664 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Plotting (1990)(Ocean)[a].adf:PLOT | 2 | 155 | 9 |
| Plotting (1990)(Ocean)[a].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6302 | 258 |

## Cases handled

- runtime segment an address: 3054 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 2-10 (bytes 1024-5632) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.39)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 22-22 (bytes 11264-11776) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.20)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 44-44 (bytes 22528-23040) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.69)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 66-66 (bytes 33792-34304) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.21)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 88-88 (bytes 45056-45568) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.42)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 110-110 (bytes 56320-56832) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.36)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 132-132 (bytes 67584-68096) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.35)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 154-154 (bytes 78848-79360) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.08)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 176-176 (bytes 90112-90624) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.02)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 198-198 (bytes 101376-101888) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.11)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 220-220 (bytes 112640-113152) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.50)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 242-242 (bytes 123904-124416) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.21)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 264-264 (bytes 135168-135680) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.52)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 286-286 (bytes 146432-146944) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.54)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 308-308 (bytes 157696-158208) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.05)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 330-330 (bytes 168960-169472) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 352-352 (bytes 180224-180736) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 354-354 (bytes 181248-181760) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.14)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 374-374 (bytes 191488-192000) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 376-376 (bytes 192512-193024) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.13)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 396-396 (bytes 202752-203264) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 398-398 (bytes 203776-204288) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.73)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 418-418 (bytes 214016-214528) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 420-420 (bytes 215040-215552) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.78)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 440-440 (bytes 225280-225792) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 442-442 (bytes 226304-226816) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.07)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 462-462 (bytes 236544-237056) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 464-464 (bytes 237568-238080) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.14)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 484-484 (bytes 247808-248320) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 486-486 (bytes 248832-249344) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.22)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 506-506 (bytes 259072-259584) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 508-508 (bytes 260096-260608) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.35)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 528-528 (bytes 270336-270848) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 530-530 (bytes 271360-271872) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.26)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 550-550 (bytes 281600-282112) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 552-552 (bytes 282624-283136) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.69)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 572-572 (bytes 292864-293376) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 574-574 (bytes 293888-294400) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.93)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 594-594 (bytes 304128-304640) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 596-596 (bytes 305152-305664) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 616-616 (bytes 315392-315904) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 618-618 (bytes 316416-316928) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.09)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 638-638 (bytes 326656-327168) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 640-640 (bytes 327680-328192) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.51)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 660-660 (bytes 337920-338432) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 662-662 (bytes 338944-339456) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.76)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 682-682 (bytes 349184-349696) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 684-684 (bytes 350208-350720) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 704-704 (bytes 360448-360960) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 706-706 (bytes 361472-361984) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 712-712 (bytes 364544-365056) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.93)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 726-726 (bytes 371712-372224) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 728-728 (bytes 372736-373248) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 734-734 (bytes 375808-376320) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.82)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 748-748 (bytes 382976-383488) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 750-750 (bytes 384000-384512) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 770-770 (bytes 394240-394752) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 772-772 (bytes 395264-395776) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 792-792 (bytes 405504-406016) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 794-794 (bytes 406528-407040) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 814-814 (bytes 416768-417280) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 816-816 (bytes 417792-418304) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 836-836 (bytes 428032-428544) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 838-838 (bytes 429056-429568) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 858-858 (bytes 439296-439808) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 860-860 (bytes 440320-440832) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 882-890 (bytes 451584-456192) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 902-902 (bytes 461824-462336) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 904-904 (bytes 462848-463360) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 910-910 (bytes 465920-466432) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.31)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 924-924 (bytes 473088-473600) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 926-926 (bytes 474112-474624) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 932-932 (bytes 477184-477696) outside the filesystem hold data, not code (opcode density 0.012, entropy 6.29)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 946-946 (bytes 484352-484864) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 948-948 (bytes 485376-485888) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 954-954 (bytes 488448-488960) outside the filesystem hold data, not code (opcode density 0.016, entropy 6.25)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 968-968 (bytes 495616-496128) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 970-970 (bytes 496640-497152) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 976-976 (bytes 499712-500224) outside the filesystem hold data, not code (opcode density 0.023, entropy 6.31)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 990-990 (bytes 506880-507392) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 992-992 (bytes 507904-508416) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 998-998 (bytes 510976-511488) outside the filesystem hold data, not code (opcode density 0.016, entropy 6.31)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1012-1012 (bytes 518144-518656) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1014-1014 (bytes 519168-519680) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1020-1020 (bytes 522240-522752) outside the filesystem hold data, not code (opcode density 0.020, entropy 6.31)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1034-1034 (bytes 529408-529920) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1036-1036 (bytes 530432-530944) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1042-1042 (bytes 533504-534016) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1056-1056 (bytes 540672-541184) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.05)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1058-1058 (bytes 541696-542208) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1064-1064 (bytes 544768-545280) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1066-1066 (bytes 545792-546304) outside the filesystem hold data, not code (opcode density 0.012, entropy 6.08)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1078-1078 (bytes 551936-552448) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.05)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1080-1080 (bytes 552960-553472) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1086-1086 (bytes 556032-556544) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1088-1088 (bytes 557056-557568) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.75)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1100-1100 (bytes 563200-563712) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.05)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1102-1102 (bytes 564224-564736) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1108-1108 (bytes 567296-567808) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1110-1110 (bytes 568320-568832) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.81)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1122-1122 (bytes 574464-574976) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.05)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1124-1124 (bytes 575488-576000) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1130-1130 (bytes 578560-579072) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1132-1132 (bytes 579584-580096) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.82)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1144-1144 (bytes 585728-586240) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.05)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1146-1146 (bytes 586752-587264) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1152-1152 (bytes 589824-590336) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1154-1154 (bytes 590848-591360) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.81)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1166-1166 (bytes 596992-597504) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.05)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1168-1168 (bytes 598016-598528) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1174-1174 (bytes 601088-601600) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1176-1176 (bytes 602112-602624) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.82)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1188-1188 (bytes 608256-608768) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.05)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1190-1190 (bytes 609280-609792) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1196-1196 (bytes 612352-612864) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1198-1198 (bytes 613376-613888) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.77)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1210-1210 (bytes 619520-620032) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.05)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1212-1212 (bytes 620544-621056) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1218-1218 (bytes 623616-624128) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1220-1220 (bytes 624640-625152) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.79)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1232-1232 (bytes 630784-631296) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.05)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1234-1234 (bytes 631808-632320) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1240-1240 (bytes 634880-635392) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1254-1254 (bytes 642048-642560) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.05)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1256-1256 (bytes 643072-643584) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1262-1262 (bytes 646144-646656) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1264-1264 (bytes 647168-647680) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.80)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1276-1276 (bytes 653312-653824) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.05)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1278-1278 (bytes 654336-654848) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1284-1284 (bytes 657408-657920) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1286-1286 (bytes 658432-658944) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.82)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1298-1298 (bytes 664576-665088) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.05)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1300-1300 (bytes 665600-666112) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1306-1306 (bytes 668672-669184) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1308-1308 (bytes 669696-670208) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.78)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1320-1320 (bytes 675840-676352) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.05)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1322-1322 (bytes 676864-677376) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1328-1328 (bytes 679936-680448) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1330-1330 (bytes 680960-681472) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.81)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1342-1342 (bytes 687104-687616) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.05)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1344-1344 (bytes 688128-688640) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1350-1350 (bytes 691200-691712) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1352-1352 (bytes 692224-692736) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.78)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1364-1364 (bytes 698368-698880) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.05)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1366-1366 (bytes 699392-699904) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1372-1372 (bytes 702464-702976) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1374-1374 (bytes 703488-704000) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.79)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1386-1386 (bytes 709632-710144) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.05)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1388-1388 (bytes 710656-711168) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1394-1394 (bytes 713728-714240) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1396-1396 (bytes 714752-715264) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.26)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1410-1410 (bytes 721920-722432) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1416-1416 (bytes 724992-725504) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1418-1418 (bytes 726016-726528) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.26)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1432-1432 (bytes 733184-733696) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1438-1438 (bytes 736256-736768) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1440-1440 (bytes 737280-737792) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.26)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1454-1454 (bytes 744448-744960) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1460-1460 (bytes 747520-748032) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1462-1462 (bytes 748544-749056) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.26)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1476-1476 (bytes 755712-756224) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1482-1482 (bytes 758784-759296) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1484-1484 (bytes 759808-760320) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.26)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1498-1498 (bytes 766976-767488) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1504-1504 (bytes 770048-770560) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1506-1506 (bytes 771072-771584) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.26)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1520-1520 (bytes 778240-778752) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1526-1526 (bytes 781312-781824) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1528-1528 (bytes 782336-782848) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.26)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1542-1542 (bytes 789504-790016) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1548-1548 (bytes 792576-793088) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1550-1550 (bytes 793600-794112) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.26)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1564-1564 (bytes 800768-801280) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1570-1570 (bytes 803840-804352) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1572-1572 (bytes 804864-805376) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.26)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1586-1586 (bytes 812032-812544) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1592-1592 (bytes 815104-815616) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1594-1594 (bytes 816128-816640) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.26)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1608-1608 (bytes 823296-823808) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1614-1614 (bytes 826368-826880) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1616-1616 (bytes 827392-827904) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.26)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1630-1630 (bytes 834560-835072) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1636-1636 (bytes 837632-838144) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1638-1638 (bytes 838656-839168) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.26)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1652-1652 (bytes 845824-846336) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1658-1658 (bytes 848896-849408) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1660-1660 (bytes 849920-850432) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.26)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1674-1674 (bytes 857088-857600) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1680-1680 (bytes 860160-860672) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1682-1682 (bytes 861184-861696) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.26)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1696-1696 (bytes 868352-868864) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1702-1702 (bytes 871424-871936) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1704-1704 (bytes 872448-872960) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.26)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1718-1718 (bytes 879616-880128) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1724-1724 (bytes 882688-883200) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1726-1726 (bytes 883712-884224) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.26)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1740-1740 (bytes 890880-891392) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.97)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1746-1746 (bytes 893952-894464) outside the filesystem hold data, not code (opcode density 0.027, entropy 6.19)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf: blocks 1748-1748 (bytes 894976-895488) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.26)
- plotting (1990)(ocean)[a]/Plotting (1990)(Ocean)[a].adf:PLOT: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 7 packed track region(s), 3584 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Plotting (1990)(Ocean)[a].adf:PLOT

- `store_routine_address`: Stores the address of the routine into a global variable.
- `set_colour_dma_interrupt_state`: The decompiler timed out, so no pseudo-code is available.
- `poll_state_flags_loop`: A long polling loop that reads a state word, tests its bits to dispatch several routines and a countdown, and repeats until a terminating state is reached.
- `transform_words_if_marker_matches`: Compares a 32-bit value with the ASCII text 'Ice!' and, if it matches, runs 4000 passes over groups of four words, shifting each with carry propagation and storing the results back.
- `accumulate_32bit_from_bytes`: Reads four consecutive bytes from the address register and concatenates them into one 32-bit result.
- `unpack_backward_lz_stream`: Decodes a packed stream from its end, reading length and distance codes and copying earlier output backward into the destination.
- `short_delay_loop`: Runs a short countdown loop with a fixed count and returns; it is a brief pause routine with no other effect.
- `check_flags_no_effect`: Tests a sign bit and a low flag bit of its inputs and returns.
- `read_bits_from_word_stream`: Shifts bits out of a bit buffer one at a time.

### Plotting (1990)(Ocean)[a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `manage_system_state_machine`: Updates a complex state machine by evaluating bitwise flags in a global variable to trigger a sequence of hardware configuration, input processing, and sprite management routines.
- `execute_state_logic`: Performs a sequence of subroutine calls based on bits in a global variable and updates a function pointer and a global variable.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `dispatch_conditional_logic`: Manages a series of subroutine calls and updates a function pointer and a global variable based on bitwise flags.
- `update_hardware_and_state_parameters`: Configures audio hardware, bitplane parameters, and DMA-related memory settings based on various global state flags.
- `update_global_state`: Calls another routine and then populates several global variables with calculated values and constants.
- `sort_data_tables`: Performs a nested loop to compare and swap values between a global table and two other data tables based on a comparison condition.
- `dispatch_state_machine_logic`: Processes a complex series of conditional branches and bitwise operations on a global variable to update various system parameters and state flags.
- `update_parameter_table`: Updates a series of global memory locations with calculated values and fixed constants.
- `process_input_status`: Checks a bit in a memory location, performs a bitwise rotation and shift on a byte, and executes a delay loop.
- `initialize_blitter_dimensions`: Copies eight longwords from a source pointer into global variables and configures blitter width, height, and size registers.
- `configure_blitter_parameters`: Copies eight longwords into global variables and sets the blitter dimensions and size.
- `initialize_short_values`: Sets three consecutive global short integers to specific constant values.
- `manage_sprite_display_state`: Clears a series of global memory locations and manages sprite bitplane data through a state machine.
- `copy_six_words`: Copies seven words from a source pointer into a series of global variables.
- `update_status_flags_2`: Checks and modifies specific bits in a global byte and calls other routines to fetch and store data from tables.
- `process_data_table`: Clears eight global short integers, copies thirty-two bytes into a table, and performs a loop that modifies data at a table offset using bitwise operations and constant values.
- `process_sprite_attribute_table`: Iterates through a table of pointers to update sprite or object attributes.
- `initialize_hardware_parameters`: Sets several global variables and memory locations to specific constant values and performs a sequence of subroutine calls.
- `update_counter_and_data`: Checks a global variable and, if a specific condition is met, stores a byte from a table into a register and modifies a global variable.
- `configure_blitter_sequence`: Sets up multiple global variables with data from tables and executes a series of blitter operations including parameter configuration and waiting.
- `store_combined_values`: Combines two 16-bit values into a 32-bit word and stores it at a pointer, then updates a table with the resulting address and its high word.
- `calculate_memory_offset`: Calculates a memory address based on a register value and stores it in a global variable along with another register value.
- `populate_blitter_table`: Uses a register value to calculate various offsets and copies multiple longwords from a table into a series of global variables.
- `update_blitter_data`: Calculates offsets from a register value to copy longwords from a table into global variables, optionally performing an additional copy if a condition is met.
- `decrement_and_modify_byte`: Decrements a global short value and, if it reaches zero, modifies a global byte by clearing one bit and setting another.
- `decrement_and_set_bits`: Decrements a global short value and, if it reaches zero, sets the two highest bits of a global byte.
- `clear_eight_words`: Sets eight consecutive global short integers to zero.
- `update_sprite_bitplane_data`: Iterates through a multi-dimensional coordinate system to fetch data from tables and update global memory locations.
- `copy_thirty_two_bytes`: Copies thirty-two bytes from a source pointer into a global table using a loop.
- `copy_eight_longwords`: Copies eight longwords from a source pointer into a series of global variables.
- `initialize_global_parameters`: Sets a global variable to a specific value and performs conditional execution based on that value.
- `set_constant_value`: Assigns a constant value to a global variable and executes another routine.
- `tiny_ori_2`: Small routine of 1 instruction that merges 35 into -(a3).
- `clear_global_group_one`: Sets a global variable and several consecutive word-sized memory locations to zero.
- `clear_global_group_two`: Sets a global variable and a different set of consecutive word-sized memory locations to zero.
- `initialize_memory_block`: Clears a block of memory by writing zero to a sequence of words and then sets several global variables to specific values.
- `clear_memory_tables`: Clears a block of memory by writing zeros to a series of longwords and resets several global variables and registers.
- `fetch_table_data`: Calculates an offset based on a value, retrieves words from a table, and stores them in global variables.
- `clear_data_register`: Performs a logical OR operation on a data register with zero.
- `search_and_set_value`: Performs a search through a table to find a specific entry and updates it with a value.
- `search_table_entry`: Iterates through a table to find an entry that satisfies specific conditions and updates that entry with a constant value.
- `load_offset_data`: Retrieves words from a table using a calculated offset derived from a global variable and stores them in other global variables.
- `repack_bitstream_and_configure_blitter`: Extracts and repackulates bitstream data from a source buffer into a destination buffer.
- `initialize_blitter_table_and_repack_data`: Initializes a series of global variables and hardware configuration parameters from a data table.
- `update_digit_table`: Clamps a value to a maximum of ninety-nine, then uses its digits to fetch and store words from a table into global variables.
- `process_digit_values`: Clamps a value to ninety-nine and uses its tens and units digits to fetch and store words from a table into global variables.
- `initialize_data_table`: Stores a series of word-sized values and associated lengths into a sequence of global memory locations.
- `initialize_data_table_2`: Writes a sequence of constant values to a table in memory and waits for a global variable to no longer be negative one.
- `store_table_and_wait_flag`: Writes four constant (value, count) pairs into a fixed data table, stores a constant into a shared word, then busy-waits until a global flag reads as -1 before setting that flag to a pointer value and a constant.
- `initialize_data_table_3`: Sets multiple memory locations to predefined constant values and waits for a specific global variable state.
- `initialize_data_table_4`: Fills a memory area with constant data and enters a loop that checks a global variable.
- `configure_dma_hardware`: Manipulates DMA control, copper, and video hardware registers while waiting for a global variable to change.
- `setup_dma_parameters`: Stores several word values into a series of memory locations and modifies hardware registers for DMA and video positioning.
- `tiny_ori_cmp_ori`: Small routine of 3 instructions that merges 4 into d0, then compares d2 with (a6), then merges 0 into d0.
- `configure_blitter_buffer`: Updates hardware configuration by writing values from registers and a table into global variables.
- `shift_memory_words`: If a specific bit in a global variable is set, it performs a series of word shifts within a memory block.
- `process_pointer_list`: Iterates through a table of pointers to update global variables and increments a counter that wraps at twelve.
- `setup_blitter_parameters`: Configures blitter hardware by storing register values and table data into global variables.
- `shift_memory_conditional`: Checks two different bits in a global variable and performs word shifts in a memory block if the second bit is set.
- `execute_table_loop`: Iterates through a table of pointers to update global variables and increments a global counter that wraps at twelve.
- `execute_indirect_jump_2`: Performs bitwise checks on a global variable to determine a jump target and then calls a function pointer from a table.
- `execute_indirect_jump`: Calculates an offset from a global variable to find a pointer in a table and then calls the routine at that address.
- `dispatch_jump_table_2`: Uses a global variable to index into a table of function pointers and executes the resulting address.
- `dispatch_jump_table`: Iterates through a table using a global variable as a base to find and execute a specific function pointer.
- `update_state_data`: Calls another routine and stores the resulting long value and several other pieces of data into global variables.
- `update_state_audio`: Calls another routine and configures audio hardware pitch before storing the resulting data into global variables.
- `reset_and_update_state`: Clears a global variable before calling another routine to store the resulting data into global variables.
- `update_global_state_cache`: Retrieves a long value from another routine and distributes its high and low words into multiple global variables.
- `set_flags_and_update`: Sets specific bits in a global variable before calling another routine to store the resulting data into global variables.
- `update_state_from_table`: Loads a value from a table and calls another routine to store the resulting data into global variables.
- `update_global_state_cache_2`: Stores the result of a calculation into several global variables and performs a conditional adjustment to a global value if the result matches a specific constant.
- `update_state_and_sync_globals`: Calls a routine to calculate a long value and updates multiple global variables with the result and other state data.
- `clear_and_update_table`: Clears a global variable and loads a value from a table before calling another routine to store the resulting data into global variables.
- `update_global_state_cache_3`: Stores the result of another routine into several global variables and performs a conditional adjustment to one of those values if a specific constant is met.
- `update_state_with_decrement`: Decrements a global variable by eleven, resets it to zero, and then updates several global variables using values from a subroutine and a table.
- `update_state_with_small_decrement`: Decrements a global variable by five, resets it to zero, and then updates several global variables using values from a subroutine and a table.
- `update_state_with_large_decrement`: Decrements a global variable by nine and updates several global variables using values from a subroutine and a table.
- `update_state_with_tiny_decrement`: Decrements a global variable by three and updates several global variables using values from a subroutine and a table.
- `update_state_with_increment`: Increments a global variable by three and updates several global variables using values from a subroutine and a table.
- `update_state_with_large_increment`: Increments a global variable by nine and updates several global variables using values from a subroutine and a table.
- `update_state_with_medium_increment`: Increments a global variable by five, resets it to zero, and then updates several global variables using values from a subroutine and a table.
- `update_state_with_maximum_increment`: Increments a global variable by eleven, resets it to zero, and then updates several global variables using values from a subroutine and a table.
- `update_state_from_zero`: Sets a global variable to zero and updates several global variables using values from a subroutine and a table.
- `update_state_with_extra_call`: Calls another routine, decrements a global variable by seven, and updates several global variables using values from a subroutine and a table.
- `decrement_global_and_update_state`: Decrements a global variable and stores the result and several other values into a set of global variables.
- `set_bits_and_update_state`: Performs a bitwise OR on a global variable and stores the resulting data into a set of global variables.
- `increment_global_and_update_state`: Increments a global variable and stores the result along with values from other memory locations into a set of global variables.
- `update_state_structure_a`: Calculates a long value and stores it along with several other pieces of data into a set of global variables.
- `update_state_from_memory`: Loads a short value from a memory location and stores it along with other data into a set of global variables.
- `update_state_and_sync_audio`: Calculates a long value and updates several global variables with its components and associated register data.
- `update_state_with_different_source`: Loads a short value from a memory location and stores it along with data from different memory addresses into a set of global variables.
- `update_state_structure_b_2`: Calls another routine to retrieve a long value and distributes its components and associated register data into several global variables.
- `set_bits_and_update_state_complex`: Sets specific bits in a global variable and stores the resulting data into a set of global variables.
- `update_state_from_pointer`: Loads a short value through a pointer and stores it along with data from that pointer into a set of global variables.
- `update_state_from_specific_memory`: Loads a short value from a memory location and stores it along with data from specific memory addresses into a set of global variables.
- `update_state_from_globals`: Calls another routine and stores the resulting long value and several other global variables into a set of destination variables.
- `update_state_from_table_2`: Loads a value from a table using an offset, calls another routine, and stores the results into various global variables.
- `update_state_from_globals_alt`: Calls another routine and stores the resulting long value and several other global variables into a different set of destination variables.
- `decrement_and_update_state`: Decrements a global variable, calls another routine, and stores the results into a set of global variables.
- `subtract_five_and_update`: Subtracts five from a global variable, calls another routine, and stores the results into a set of global variables.
- `subtract_nine_and_update`: Subtracts nine from a global variable, calls another routine, and stores the results into a set of global variables.
- `subtract_three_and_update`: Subtracts three from a global variable, calls another routine, and stores the results into a set of global variables.
- `add_three_and_update`: Adds three to a global variable, calls another routine, and stores the results into a set of global variables.
- `add_nine_and_update`: Adds nine to a global variable, calls another routine, and stores the results into a set of global variables.
- `add_five_and_update`: Adds five to a global variable, calls another routine, and stores the results into a set of global variables.
- `increment_and_update_state`: Increments a global variable and updates several global variables with the results of another routine and register data.
- `update_state_from_register`: Updates several global variables using a value from another routine and a value from a global variable.
- `decrement_and_update_state_2`: Decrements a global variable and updates several global variables with the results of another routine and register data.
- `subtract_and_update_state`: Subtracts a value from a global variable and updates several global variables using data from a memory offset.
- `bitwise_or_and_update_state`: Performs a bitwise OR on a global variable and updates several global variables using data from a memory offset.
- `add_and_update_state`: Adds a value to a global variable and updates several global variables with the results of another routine and register data.
- `update_state_structure_b`: Calculates a long value and stores the result and associated register data into a different set of global variables.
- `clear_specific_memory_range`: If a global variable matches a specific value, it clears a series of consecutive longwords in memory.
- `clear_memory_block_one`: Sets a global variable and a long sequence of consecutive memory locations to zero.
- `clear_alternative_memory_range`: If a global variable matches a specific value, it clears a different series of consecutive longwords in memory.
- `clear_memory_block`: Sets a series of consecutive global memory locations to zero.
- `manage_state_machine_transitions`: Updates a global state variable through a series of conditional checks and bitwise manipulations.
- `execute_recursive_state_machine`: Manages a complex state machine by performing bitwise checks on a global variable to trigger recursive calls, arithmetic updates, or table lookups.
- `configure_audio_channel`: Checks a bit in a global variable and, if not set, configures audio hardware registers and a memory location.
- `configure_audio_pitch`: Checks a bit in a global variable and, if not set, configures audio hardware registers with a different period value.
- `update_audio_hardware`: Updates audio hardware registers and a global variable with specific values and a memory address.
- `call_table_function`: Iterates through a table of pointers using an offset, updating a pointer variable until a non-zero function pointer is found and called.
- `execute_jump_table`: Iterates through a table of function pointers and executes a routine retrieved from the table.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 198 into d4, then merges 0 into d0.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 200 into d4, then merges 0 into d0.
- `store_calculated_data`: Calculates a pointer based on a global variable and performs multiple stores of various data types into memory.
- `process_complex_data_set_a`: Calculates a pointer based on a global variable and updates a series of global variables with data from that pointer and other memory locations.
- `process_complex_data_set_b`: Calculates a pointer based on a global variable and updates a series of global variables using a different offset from that pointer.
- `process_complex_data`: Calculates a value based on a pointer derived from a global variable and stores the result and several related values into memory.
- `process_complex_data_set_c`: Calculates a pointer based on a global variable and updates a different set of global variables with data from that pointer and other memory locations.
- `update_complex_state`: Calculates a long value and stores several derived results and constants into a series of global variables.
- `update_pathfinding_grid`: Iterates through a memory table to perform a search or propagation algorithm, updating cell values based on neighbor states.
- `set_memory_bit`: Sets a specific bit in a longword located at a memory address.
- `traverse_and_update_linked_list`: Iterates through a linked structure in memory using a pointer derived from an input value.
- `update_pathfinding_grid_2`: Iterates through a memory table to update cell states based on neighbor values.
- `update_tile_grid_cells`: Walks a grid of 16-bit cell values stored in a table, rewriting each cell's flag bits and advancing a row counter while scanning until a terminating zero value.
- `traverse_and_update_linked_list_2`: Iterates through a memory structure using a pointer-based traversal, checking specific status values at each node.
- `configure_blitter_and_display_state`: Updates hardware registers and global state variables to manage blitter parameters and video display positioning.
- `configure_blitter_state`: Checks a bit in a global variable and performs recursive decrements before configuring hardware registers and bitplane parameters via several subroutines.
- `update_display_and_blitter_state`: Updates hardware display registers and manages a linked list of data structures through recursive calls.
- `update_blitter_parameters_7`: Decrements a global variable through a series of conditional checks and updates hardware register configuration values.
- `modify_state_flags`: Modifies bits in a global variable and enters a loop that waits for a specific memory value to change.
- `modify_global_flags`: Performs bitwise operations on a global variable and executes several recursive calls if a specific value is reached.
- `dispatch_state_logic`: Modifies a global byte and enters a loop that waits for a specific memory value while potentially calling other routines.
- `process_conditional_logic`: Decrements a global byte and, if it reaches zero, calls other routines and updates a global variable with a constant value.
- `execute_indirect_blitter_call`: Fetches a pointer from a table using a global variable as an index, configures blitter parameters, and performs an indirect call to the retrieved address.
- `dispatch_table_routine`: Uses a global variable to index into a table, sets up blitter parameters, and executes a function pointer retrieved from memory.
- `copy_structured_data`: Copies several words from a source table to a destination table at specific offsets.
- `update_blt_parameter_tables`: Checks a global state variable and, if a specific bit is set, calculates and stores multiple bitplane and pointer parameters into a structured table.
- `decrement_and_update_state_3`: Decrements a global counter and, upon reaching zero, performs a series of complex state transitions and pointer updates.
- `update_state_variable`: Modifies a global variable by decrementing it and performing conditional bitwise updates and assignments based on its value.
- `decrement_and_update_state_4`: Decrements a global counter and performs conditional bitwise checks on a state variable.
- `update_state_variable_alt`: Decrements a global variable and applies conditional logic to modify its bits or set it to a specific constant.
- `fetch_table_data_2`: Uses a global variable to calculate offsets into a table, loads values from those offsets into several memory locations, and performs blitter configuration.
- `load_blitter_data`: Calculates offsets from a global variable to fetch values from a table and stores them into several global variables for hardware configuration.
- `configure_blitter_parameters_3`: Decrements a global counter and, if it reaches zero, updates several memory locations with values derived from a register and a table lookup.
- `configure_blitter_parameters_2`: Clears a bit in a global variable and stores several values into memory to prepare for a blitter operation.
- `fetch_table_parameters`: Uses a global variable to index into a table and writes the resulting values into multiple global variables.
- `increment_global_counter`: Checks if a global variable is greater than another value, then increments a counter and updates a global variable.
- `increment_table_counter`: Compares a global variable against a specific address and, if the condition is met, increments a counter and updates a global variable.
- `setup_blitter_buffer`: Decrements a global counter and, upon reaching zero, stores values from a table into specific memory locations.
- `update_blt_buffer_parameters`: Clears the lowest bit of a global variable and updates several memory locations with values derived from a register and a constant offset.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 188 into d4, then merges 208 into d1, then merges 176 into d1.
- `tiny_ori_ori_ori_2`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `manage_state_and_blitter`: Updates a global variable based on bitwise checks and, if certain conditions are met, executes a routine to store multiple values into memory for blitter use.
- `update_state_and_configure_blitter`: Modifies a global state variable using bitwise logic and performs recursive calls based on specific bit flags.
- `process_loop_data`: Checks a bit in a global variable and performs a loop that updates several memory locations and global variables through multiple subroutine calls.
- `iterate_table_entries`: Executes a loop that modifies global variables and performs conditional subroutine calls based on bitwise checks and equality comparisons.
- `store_derived_data`: Retrieves a value from a global table and uses it to populate several memory locations with modified versions of other global data.
- `copy_table_data`: Fetches data from a global table and stores it along with several other derived values into a set of global variables.
- `store_calculated_values`: Calculates a value using a subtraction and a constant, then stores the result and other register values into a table.
- `process_linked_list_data`: Iterates through a list of structures to move values, updates several memory offsets, and performs conditional updates based on a global bitmask.
- `process_linked_list`: Iterates through a list of data structures, moving values between elements and updating several memory offsets and counters.
- `search_list_elements`: Scans a memory structure for specific non-zero values and updates various global variables and memory offsets accordingly.
- `search_and_configure_blitter_data`: Iterates through a memory structure to find a specific non-zero value that is neither zero nor seven, then updates global configuration variables with offset coordinates and a value from the found memory location.
- `setup_blitter_parameters_a`: Writes a set of constant values and register-derived offsets into global variables to configure blitter hardware.
- `set_blt_parameters`: Writes several values to global variables, including a constant, to configure hardware registers.
- `setup_blitter_parameters_b`: Configures blitter hardware by writing specific constants and register-derived values into global variables.
- `update_blitter_parameters`: Writes values from registers into the hardware registers used to define the source, destination, and size for a blitter operation.
- `setup_blitter_from_table`: Loads values from a table and writes them to global variables to configure blitter hardware registers.
- `set_blt_parameters_from_table`: Loads a value from a table and writes it along with other values to global variables to configure hardware registers.
- `setup_blitter_parameters_c`: Writes a specific set of constants and register-derived values into global variables for blitter configuration.
- `update_blitter_parameters_2`: Writes values from registers to specific memory locations and updates hardware registers used for blitter operations.
- `setup_blitter_parameters_d`: Updates global variables with constants and register-derived values to prepare for a blitter operation.
- `update_blitter_parameters_3`: Updates several hardware registers associated with the blitter, including source, destination, and size parameters, using values from registers and a constant.
- `setup_blitter_parameters_e`: Writes values from a pointer and several register-derived offsets into global variables to configure the blitter.
- `update_blitter_parameters_4`: Copies several values from registers or memory into global variables and writes a constant value to a register.
- `setup_blitter_parameters_f`: Copies values from a pointer and registers into global variables to set up blitter hardware parameters.
- `update_blitter_parameters_5`: Copies values from registers and memory into specific hardware registers used for blitter operations.
- `set_blt_parameters_partial_2`: Writes three values, including a word from a table, to global variables to configure hardware registers.
- `set_blt_parameters_partial`: Writes three values to global variables, including a word from a table, to configure hardware registers.
- `load_blt_table_values`: Loads multiple values from a table and writes them to global variables to configure hardware registers.
- `set_blt_parameters_from_table_2`: Loads values from a table and writes them to global variables to configure hardware registers.
- `store_blt_parameters_2`: Stores three values from registers and a memory offset into global variables to configure bitplane and destination registers.
- `store_blt_parameters`: Stores three values from registers and a memory offset into global variables and touches hardware registers for bitplane addresses, destination address, and size.
- `update_blt_parameters_6`: Stores values from registers and memory into global variables.
- `update_blitter_parameters_6`: Stores values from registers and memory into global variables.
- `return_from_subroutine`: The routine is empty and performs no operations before returning.
- `store_blt_registers_2`: Calculates values based on register inputs and stores five values into global variables to configure bitplane and color registers.
- `store_blt_registers`: Stores five values from registers into global variables and touches hardware registers for bitplane address, bitplane pointer, color pointer, destination address, and size.
- `calculate_complex_value`: Performs bitwise shifts and arithmetic operations on input registers to return a calculated long value.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning.
- `rotate_bits_and_shift`: Performs a series of bitwise circular shifts and logical shifts on a value in a register.
- `store_blt_parameters_offset_ten_2`: Stores three values from registers and a word from a memory offset into global variables to configure bitplane address registers.
- `store_blt_parameters_offset_ten`: Stores three values from registers and a word from a memory offset into global variables and touches hardware registers for bitplane addresses, destination address, and size.
- `update_blt_parameters_2`: Stores values from registers and a memory location into global variables.
- `update_blt_parameters`: Stores values from registers and a memory location into global variables.
- `store_blt_extended_parameters_2`: Stores five values from registers and memory offsets into global variables to configure bitplane and pointer registers.
- `store_blt_extended_parameters`: Stores five values from registers and memory offsets into global variables and touches hardware registers for bitplane address, bitplane pointer, color pointer, destination address, and size.
- `store_blt_dimensions_2`: Stores values from registers and a memory location into global variables to configure blitter width, height, and size registers.
- `store_blt_dimensions`: Stores values from registers and a memory location into the blitter width, height, and size registers.
- `setup_blitter_registers`: Loads several constants into the blitter mask, mode, width, control, height, and size registers.
- `wait_for_blitter_2`: Polls a bit in a hardware register until it clears.
- `wait_for_blitter`: Stores a value into a register and polls a bit in a register until it clears.
- `mask_memory_words`: Iterates through a table of pointers to perform bitwise AND operations on specific words within memory blocks.
- `wait_for_blt_completion`: Sets a value to a global variable and enters a loop that polls a specific bit of that variable until it is cleared.
- `mask_memory_nibbles`: Iterates through a table of pointers to mask specific bits within words and nibbles across multiple memory locations.
- `wait_for_blt_completion_2`: Sets a value to a global variable and enters a loop that polls a specific bit of that variable until it is cleared.
- `configure_blitter_registers`: Writes various values to blitter control and mode registers and waits for a blitter status bit to clear.
- `wait_for_blitter_3`: Polls a global variable until a specific bit is cleared.
- `wait_for_blitter_variable`: Stores two values into registers and polls a bit in a register until it clears.
- `configure_and_wait_blitter`: Sets blitter mask, mode, control, height, and size registers, then polls a bit in a register until it clears.
- `wait_for_counters`: Checks if two global counters are zero, calling another routine if they are not.
- `decrement_and_copy_table`: Decrements a global counter and, upon reaching zero, copies a sequence of bytes from one location to another.
- `decrement_and_process_table`: Decrements a global counter and, if zero, performs nibble-based arithmetic on a sequence of words in a table.
- `process_bitmask_table`: Performs a nested loop to modify values in a table using bitwise operations and increments a global counter until it reaches a threshold.
- `toggle_interrupt_enable`: Writes a value to the interrupt enable register to enable or disable interrupts.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `update_state_machine_counters`: Manages a complex state machine by checking bitflags in a global variable and decrementing or resetting various countdown timers.
- `update_state_machine_logic`: Manages a complex state machine by checking bits in a global variable to execute conditional branches, decrementing internal counters, and calling other routines.
- `update_input_state_machine`: Manages a complex state machine by checking bitflags in a global variable and decrementing or resetting various countdown timers.
- `update_status_flags`: Updates a byte at a pointer by performing bitwise operations based on a global variable and bits from a register.
- `update_input_status`: Checks the state of input bits and updates a global variable by setting specific bits based on those inputs.
- `set_status_bits`: Sets specific bits in a byte through a pointer based on the contents of a register.
- `toggle_input_state`: Updates a 16-bit global variable by setting or clearing its high bytes based on a global variable's state.
- `search_and_reset_data`: Iterates through a data structure to find a value and, if a condition is met, resets several global variables to zero or a value from a table.
- `clear_global_variables`: Writes zero to four consecutive global variables and then writes a constant value to a fifth global variable.
- `update_audio_and_counters`: Updates several counters and global variables, potentially calling other routines based on the state of specific memory locations.
- `process_command_table`: Uses a byte from a table to select one of several operations, including calling a function from a table or updating values in a structure.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `execute_indirect_call_2`: This routine performs an indirect function call using an address retrieved from a table based on an index stored in a global variable.
- `execute_indirect_call`: Performs an indirect call to a function address calculated from a global variable and a table index.
- `store_byte_masked`: Extracts a four-bit value from a memory location and stores it into a global variable.
- `load_object_parameters`: Loads data from a source into a structure, updates a pointer, and modifies several fields based on values found in a table.
- `scan_table_and_blit_entries`: Walks a grid of 16-bit table indices in a data block, converting each index into a longword offset lookup and, for entries past a threshold, invoking helper routines and storing a computed value into blitter-related globals.
- `zero_data_register`: Applies a logical OR operation to a data register using a zero value.
- `tiny_ori_3`: Small routine of 1 instruction that merges 150 into d1.
- `load_data_from_table`: Iterates through a table to fetch values and uses them to perform memory operations and calls to other routines.
- `clear_memory_block_2`: Fills a nested block of memory with zeros using two loops.
- `process_bit_manipulation`: Performs complex bitwise shifts and arithmetic on data from a pointer and modifies a byte using a bitwise XOR operation.
- `convert_chunky_to_planar`: Reads 4000 words of packed pixel data, four words at a time, and distributes each bit of the four-word group into four separate bitplane accumulators.
- `transform_and_store_data`: Iterates through a source table, performs arithmetic transformations on values, and stores the results into a destination table at specific offsets.
- `repack_bitstream_data`: Iterates through a memory block to extract and repackulate bits from source words into a new destination buffer.
- `search_bit_pattern`: Iterates backwards through a memory block, shifting values until a non-zero bit is found.
- `return_immediately`: This routine contains a single instruction and returns to the caller without performing any operations.
- `transfer_byte_via_pointer`: Stores a single byte from a register into a memory location pointed to by another register.
- `update_pointer_via_lea`: The routine performs a single load effective address operation to update a pointer.
