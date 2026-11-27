# Game 1993 (midwinter (1990)(microprose)(fr)[cr pdx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01993). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (911568 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 30 of 30 units identical (bytes and relocations), 9 raw-word lines |
| Functions named and described | pass, 126 of 126 named (0 author symbols), 126 of 126 described (12 by rule, 64 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 309 of 309 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 109568 bytes, 16 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3417 of 3477 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 3292 |
| code | 18014 |
| data | 632208 |
| padding | 6 |
| unused | 258048 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf:bootblock | 1 | 195 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 410 | 22 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 71 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 855 | 75 |
| Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 11264-16384 (code never loaded by a reference run) | 1 | 1125 | 7 |
| Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 247808-252928 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 259072-264192 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 270336-275456 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 461824-466944 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 473088-478208 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 484352-489472 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 495616-500736 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 506880-510464 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 529408-534528 (code never loaded by a reference run) | 1 | 197 | 1 |
| Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 540672-545792 (code never loaded by a reference run) | 1 | 112 | 1 |
| Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 551936-555520 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 563200-568320 (code never loaded by a reference run) | 1 | 9 | 1 |
| Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 574464-577024 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 578560-579584 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 586240-586752 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 589824-590336 (code never loaded by a reference run) | 1 | 5 | 0 |
| Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 597504-602112 (code never loaded by a reference run) | 1 | 1 | 0 |
| Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 608256-613376 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 619520-624640 (code never loaded by a reference run) | 1 | 19 | 1 |
| Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 642048-647168 (code never loaded by a reference run) | 1 | 58 | 1 |
| Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 657408-658432 (code never loaded by a reference run) | 1 | 63 | 1 |
| Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 687104-692224 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 709632-714752 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 720896-726016 (code never loaded by a reference run) | 1 | 357 | 3 |
| Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 765952-771072 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: custom-format (non-DOS) disk, 1254 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 207 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 47 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 55 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: bytes 11264-16384 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: bytes 247808-252928 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: bytes 259072-264192 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: bytes 270336-275456 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: bytes 461824-466944 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: bytes 473088-478208 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: bytes 484352-489472 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: bytes 495616-500736 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: bytes 506880-510464 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: bytes 529408-534528 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: bytes 540672-545792 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: bytes 551936-555520 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: bytes 563200-568320 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: bytes 574464-577024 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: bytes 578560-579584 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: bytes 586240-586752 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: bytes 589824-590336 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: bytes 597504-602112 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: bytes 608256-613376 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: bytes 619520-624640 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: bytes 642048-647168 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: bytes 657408-658432 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: bytes 687104-692224 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: bytes 709632-714752 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: bytes 720896-726016 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: bytes 765952-771072 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: 26 logged loads cover 109568 bytes of the image
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: 15 track-loaded blocks found in recovered runtime segments and classified through them
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 11-20 (bytes 5632-10752) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.88)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 33-42 (bytes 16896-22016) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.93)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 44-53 (bytes 22528-27648) outside the filesystem hold data, not code (opcode density 0.019, entropy 4.21)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 55-64 (bytes 28160-33280) outside the filesystem hold data, not code (opcode density 0.011, entropy 6.92)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 66-75 (bytes 33792-38912) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.50)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 77-86 (bytes 39424-44544) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.80)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 88-97 (bytes 45056-50176) outside the filesystem hold data, not code (opcode density 0.007, entropy 5.69)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 99-108 (bytes 50688-55808) outside the filesystem hold data, not code (opcode density 0.013, entropy 6.83)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 110-119 (bytes 56320-61440) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.79)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 121-130 (bytes 61952-67072) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.59)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 132-135 (bytes 67584-69632) outside the filesystem hold data, not code (opcode density 0.011, entropy 5.69)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 137-141 (bytes 70144-72704) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.17)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 143-152 (bytes 73216-78336) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.35)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 154-163 (bytes 78848-83968) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.68)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 165-174 (bytes 84480-89600) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.98)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 176-185 (bytes 90112-95232) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.89)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 187-196 (bytes 95744-100864) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.88)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 198-207 (bytes 101376-106496) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.88)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 209-218 (bytes 107008-112128) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.39)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 220-229 (bytes 112640-117760) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.33)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 231-240 (bytes 118272-123392) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.47)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 242-251 (bytes 123904-129024) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.97)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 253-262 (bytes 129536-134656) outside the filesystem hold data, not code (opcode density 0.011, entropy 5.86)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 264-273 (bytes 135168-140288) outside the filesystem hold data, not code (opcode density 0.007, entropy 5.64)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 275-284 (bytes 140800-145920) outside the filesystem hold data, not code (opcode density 0.014, entropy 6.32)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 286-295 (bytes 146432-151552) outside the filesystem hold data, not code (opcode density 0.033, entropy 6.68)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 297-306 (bytes 152064-157184) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.69)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 319-328 (bytes 163328-168448) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.04)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 341-350 (bytes 174592-179712) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.68)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 352-361 (bytes 180224-185344) outside the filesystem hold data, not code (opcode density 0.037, entropy 7.07)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 363-372 (bytes 185856-190976) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.29)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 374-383 (bytes 191488-196608) outside the filesystem hold data, not code (opcode density 0.012, entropy 6.85)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 385-394 (bytes 197120-202240) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.51)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 396-405 (bytes 202752-207872) outside the filesystem hold data, not code (opcode density 0.018, entropy 6.80)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 407-416 (bytes 208384-213504) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.52)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 418-427 (bytes 214016-219136) outside the filesystem hold data, not code (opcode density 0.012, entropy 6.79)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 429-438 (bytes 219648-224768) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.40)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 440-449 (bytes 225280-230400) outside the filesystem hold data, not code (opcode density 0.025, entropy 6.93)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 451-460 (bytes 230912-236032) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.25)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 462-471 (bytes 236544-241664) outside the filesystem hold data, not code (opcode density 0.045, entropy 6.72)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 473-482 (bytes 242176-247296) outside the filesystem hold data, not code (opcode density 0.011, entropy 6.16)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 495-504 (bytes 253440-258560) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.21)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 517-526 (bytes 264704-269824) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.25)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 539-548 (bytes 275968-281088) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.97)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 550-559 (bytes 281600-286720) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.98)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 561-570 (bytes 287232-292352) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.73)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 572-581 (bytes 292864-297984) outside the filesystem hold data, not code (opcode density 0.011, entropy 6.22)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 583-592 (bytes 298496-303616) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.89)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 594-603 (bytes 304128-309248) outside the filesystem hold data, not code (opcode density 0.014, entropy 6.47)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 605-614 (bytes 309760-314880) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.70)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 616-625 (bytes 315392-320512) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.42)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 627-636 (bytes 321024-326144) outside the filesystem hold data, not code (opcode density 0.015, entropy 6.86)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 638-647 (bytes 326656-331776) outside the filesystem hold data, not code (opcode density 0.019, entropy 6.68)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 649-658 (bytes 332288-337408) outside the filesystem hold data, not code (opcode density 0.016, entropy 6.76)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 660-669 (bytes 337920-343040) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.49)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 671-680 (bytes 343552-348672) outside the filesystem hold data, not code (opcode density 0.012, entropy 6.76)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 682-691 (bytes 349184-354304) outside the filesystem hold data, not code (opcode density 0.014, entropy 6.56)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 693-702 (bytes 354816-359936) outside the filesystem hold data, not code (opcode density 0.015, entropy 6.92)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 704-713 (bytes 360448-365568) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.28)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 715-724 (bytes 366080-371200) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.15)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 726-735 (bytes 371712-376832) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.85)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 737-746 (bytes 377344-382464) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.91)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 748-757 (bytes 382976-388096) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.16)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 759-768 (bytes 388608-393728) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.66)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 770-779 (bytes 394240-399360) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.15)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 781-790 (bytes 399872-404992) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.53)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 792-801 (bytes 405504-410624) outside the filesystem hold data, not code (opcode density 0.013, entropy 6.79)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 803-812 (bytes 411136-416256) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.50)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 814-823 (bytes 416768-421888) outside the filesystem hold data, not code (opcode density 0.013, entropy 6.61)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 825-834 (bytes 422400-427520) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.49)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 836-845 (bytes 428032-433152) outside the filesystem hold data, not code (opcode density 0.010, entropy 5.80)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 847-856 (bytes 433664-438784) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.91)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 858-867 (bytes 439296-444416) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.67)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 869-878 (bytes 444928-450048) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.78)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 880-889 (bytes 450560-455680) outside the filesystem hold data, not code (opcode density 0.008, entropy 5.33)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 891-900 (bytes 456192-461312) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.56)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 913-922 (bytes 467456-472576) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.97)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 935-944 (bytes 478720-483840) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.53)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 957-966 (bytes 489984-495104) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.19)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 979-988 (bytes 501248-506368) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.30)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 1001-1010 (bytes 512512-517632) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.55)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 1012-1021 (bytes 518144-523264) outside the filesystem hold data, not code (opcode density 0.055, entropy 6.46)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 1023-1032 (bytes 523776-528896) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.51)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 1045-1054 (bytes 535040-540160) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.31)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 1067-1076 (bytes 546304-551424) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.88)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 1087-1087 (bytes 556544-557056) outside the filesystem hold data, not code (opcode density 0.023, entropy 0.94)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 1089-1098 (bytes 557568-562688) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.44)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 1111-1120 (bytes 568832-573952) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.37)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 1232-1241 (bytes 630784-635904) outside the filesystem hold data, not code (opcode density 0.030, entropy 6.43)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 1276-1279 (bytes 653312-655360) outside the filesystem hold data, not code (opcode density 0.044, entropy 4.80)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 1298-1304 (bytes 664576-668160) outside the filesystem hold data, not code (opcode density 0.044, entropy 5.17)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 1306-1307 (bytes 668672-669696) outside the filesystem hold data, not code (opcode density 0.018, entropy 4.85)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 1320-1329 (bytes 675840-680960) outside the filesystem hold data, not code (opcode density 0.044, entropy 5.99)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 1364-1366 (bytes 698368-699904) outside the filesystem hold data, not code (opcode density 0.055, entropy 4.46)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 1368-1373 (bytes 700416-703488) outside the filesystem hold data, not code (opcode density 0.052, entropy 4.87)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 1430-1439 (bytes 732160-737280) outside the filesystem hold data, not code (opcode density 0.013, entropy 5.81)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 1452-1459 (bytes 743424-747520) outside the filesystem hold data, not code (opcode density 0.033, entropy 4.27)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 1461-1461 (bytes 748032-748544) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.68)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 1474-1483 (bytes 754688-759808) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.31)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 1518-1527 (bytes 777216-782336) hold ASCII text
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 1540-1549 (bytes 788480-793600) hold ASCII text
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 1562-1571 (bytes 799744-804864) hold ASCII text
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 1584-1593 (bytes 811008-816128) outside the filesystem hold data, not code (opcode density 0.035, entropy 6.19)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 1650-1659 (bytes 844800-849920) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.86)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 1672-1673 (bytes 856064-857088) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.81)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 1675-1675 (bytes 857600-858112) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.11)
- midwinter (1990)(microprose)(fr)[cr pdx]/Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf: blocks 1697-1697 (bytes 868864-869376) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.15)
- 21 never-loaded region(s), 88064 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 4 packed track region(s), 20480 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf:bootblock

- `boot_entry_forbid_interrupts`: Pseudo-code is unavailable because the decompiler timed out, so this describes only the listed facts.
- `capture_pending_interrupt_bits`: When the interrupt master-enable bit is set, it stores the bits common to the interrupt enable and request registers, masked to their low fourteen bits, into a variable.
- `set_cia_port_bit_mask`: Builds a one-bit mask from a structure field and sets an extra bit when a second field is non-zero, storing the mask back into the structure.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `wait_status_bit_then_clear`: Calls an unnamed routine repeatedly while a status bit on a port stays set, then clears a word field of a structure.
- `step_pulses_toward_target`: Compares an input value with a stored position and, when they differ, updates the stored value and selects a direction bit.
- `pulse_control_port_count`: For each count in its input, writes a pair of control-port values one bit apart so that each pair produces a single pulse, then waits a fixed delay loop before the next pair.
- `poll_with_retry_count`: Stores two input words in a structure, then loops over a retry counter starting at two while calling unnamed routines to test a condition.
- `start_disk_read_wait_sync`: Sets up a disk transfer by writing fixed values to a structure and to the disk sync word and length registers, then polls a status bit until it sets or a 16-bit counter wraps.
- `decode_mfm_sector_blocks`: Scans a track buffer for disk sync words and decodes the odd and even bit pairs of the following longwords.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_vertical_blank_sequence`: Waits for the vertical blank to start, then clears a global flag and increments two 16-bit counters before calling a handler.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `init_captains_from_table`: Returns immediately unless a global longword matches a fixed address, then returns when a byte at offset 59 is nonzero, and otherwise when a byte at offset 60 is nonzero and a global word is at least four, it indexes a table by a tripled value and stores three bytes into three global words before setting several constants.
- `update_global_pointer_via_table`: Reads a global word, transforms it with a rotate and bit change, and loads a value from a table into a structure, setting a flag word to all ones or clearing it depending on the sign, then clears the flag word when it was zero.
- `update_pointer_with_calculated_value`: If a value at offset 0 of a global pointer is not 2, it combines a shifted value from one global with a scaled value from another global, stores the sum at the pointer, writes a carry-adjusted result two bytes later, and clears the global at offset 0.
- `clamp_two_values`: Returns early unless two global words equal one and eight, then shifts and subtracts from two global words, clamps each result to a range, increments a counter for each clamp, and stores all ones into a global word only when no clamp occurred.
- `frag_move_rts`: Short code fragment that copies 65535 to a fixed memory address, then returns.
- `accumulate_modified_pointer_values`: Sets bits in a structure based on the state of bytes indexed by two global pointers, then, when a flag word is nonzero, clears one of those bytes and adds the signed or unsigned version of a global pointer to two counters.
- `accumulate_joystick_input_to_counters`: When a flag word is nonzero, it reads a joystick register, derives a gray-code difference, and based on the state of specific bits adds a signed or unsigned version of two global pointers to two counters.
- `update_input_counters`: Checks a global word, then sets bits in a global word based on CIA and POT status, and adds processed values from a joystick register to two global longwords.
- `dispatch_via_table`: Reads a byte from a pointer in a0 and masks it, then clears a global word and sets others based on the masked bits before performing an indirect jump through a table indexed by the masked value.
- `frag_add_rts`: Short code fragment that adds d1 to a fixed memory address, then adds d0 to a fixed memory address, then returns.
- `clamp_global_variable_range`: Adds the value in d0 to a 32-bit global variable at offset 1FF3E, clamps it to a minimum of zero and a maximum of 326, then stores the result back.
- `clamp_global_variable_bounds`: Adds the value in d0 to a 32-bit global variable at offset 1FF3E, clamps it to a minimum of three and a maximum of 325, then stores the result back.
- `load_table_pair_values`: Clears two 32-bit global variables at offsets 1FED8 and 1FEDC, then, when two flag bytes at offsets 2A9B8 and 2A9BA are both zero, reads a pair of 32-bit values from a table indexed by a shifted global variable and stores them into those two variables.
- `enable_interrupts_set_vectors`: Points a register at the custom chip, clears both interrupt mask words at offsets 154 and 156 to enable all interrupts, stores seven routine addresses into the interrupt vector table, sets the DMA enable word at offset 154, and restores the saved registers before returning.
- `frag_movem_lea_move_btst_beq_and_andi`: Short code fragment that saves registers on the stack, then loads the address of a fixed memory address into a6, then copies the memory at an offset from a6 to d0, then tests bit 14 of d0, then branches if the last result was zero, then masks d0 with the memory at an offset from a6, then masks d0 with 16383, then copies d0 to the memory at an offset from a6, then restores saved registers from the stack.
- `frag_move_rte`: Short code fragment that copies 3840 to a fixed memory address, then returns from the exception.
- `handle_cia_port_toggle`: Checks whether a specific interrupt bit and a status bit are both set, and if so saves a CIA port byte, runs a 100-iteration delay loop, clears that status bit, and toggles a bit in a global byte before setting or clearing a table entry based on the toggled value.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `stub_infinite_loop`: Spins forever in an empty loop, so the routine never returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `search_table_for_bounds`: Iterates through a table to find an entry where specific offsets match two global values, storing the loop count in a location at offset 100 of a6.
- `check_table_overlap`: Loops through a table to find an entry where two 16-bit values fall within a range defined by two global values, storing the index in a location at offset 100 of a6.
- `check_and_call_handler`: Checks if a global word is zero, then checks if it equals 35; if so, it calls a routine via a pointer and clears the global word.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_conditional_logic`: Performs a series of calls and checks a value at offset 100 of a6 to decide whether to call a routine from a table or continue a loop.
- `reset_status_words`: Stores the value 65535 into a word at offset 34 of a structure and clears the two words at offsets 36 and 38, then returns.
- `reset_status_words_2`: Stores the value 65535 into a word at offset 36 of a structure and clears the two words at offsets 38 and 34, then returns.
- `reset_status_words_3`: Stores the value 65535 into a word at offset 38 of a structure and clears the two words at offsets 34 and 36, then returns.
- `invert_word_at_address`: Inverts the bits of a 16-bit word at a specific memory address and then returns.
- `invert_word_at_address_2`: Inverts the bits of a 16-bit word at a specific memory address and then returns.
- `invert_word_at_memory_address`: Performs a bitwise NOT operation on a 16-bit word located at a specific memory address and returns.
- `copy_struct_blocks`: Copies 49 blocks of 64 words from the table at a fixed address into the table pointed to by a6, then sets three fields of that structure and conditionally calls a helper for each of six flags that are set.
- `clear_and_copy_blocks`: Clears a global word, stores the value 36 at offset 168 of the block in a6, and calls a routine.
- `update_scaled_value_3`: Checks a global word, calls another routine, and then uses a value from a structure multiplied by 74 to calculate an offset into a table to update a value in a structure and call another routine.
- `update_scaled_value_2`: Sets a value at offset 150 of a structure, calculates a value by multiplying a byte from a structure by 74 and adding it to a pointer, stores the difference between two global variables at offset 278 of a structure, calls two other routines, and then stores a global variable into offset 32 of a structure before jumping to a new address.
- `calculate_and_jump_2`: Checks a global word, calls another routine, and then uses a value from a structure multiplied by 74 to calculate an offset into a table, adds 30, and updates a value in a structure.
- `calculate_and_jump`: Writes 8 to a word at offset 150 of a structure, calculates an offset into a table using a value from a structure and a multiplier, adds 30 to a global variable, sets a word at offset 278 of a structure to 30, calls two routines, and then jumps to a fixed address.
- `update_scaled_value`: Calculates a value by adding a byte from a structure to the result of multiplying another byte from the structure by a word in a structure and dividing by 200, then clamps the result to 249 and stores it back into the structure at offset 3.
- `calculate_scaled_offset_2`: Calls another routine, then uses a value from a structure multiplied by 74 to index into a table, and performs comparisons and updates based on the results.
- `calculate_scaled_offset`: Calculates an offset by multiplying a byte from a structure by 74 and adding it to a pointer, then writes a value from a register to that offset; it also reads a byte from a global table, multiplies it by 74 if it is not negative, and writes the result to the same calculated offset.
- `call_and_restore_registers`: Calls another routine and restores the registers a0, a1, d0, d1, and a6 from the stack before returning.
- `update_structure_and_call`: Writes 3 to a global word, calls a routine, and if a global word is not zero, it decrements that global word and loops; if the word becomes zero, it calls another routine, updates a field in a structure using a pointer from a global variable, and resets the global word to 3.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `swap_and_render_disks`: Swaps two longword fields of the structure in a6, clears a global flag and a structure word, sets a global byte at offset 8 to 5, then calls a graphics routine twice with text pointers and register values before clearing a flag and returning.
- `load_and_verify_data_block`: Initializes several global variables and structure fields, then calls a routine to load data into a destination buffer.
- `branch_to_subroutine`: Writes the value negative 2 to a global variable and branches to another routine.
- `copy_data_block`: If a value in d4 is not negative, it copies a block of bytes from a source pointed to by a pointer in a structure to a destination pointed to by another field in that structure, then updates the destination pointer in the structure; otherwise it writes negative 2 to a global variable.
- `copy_buffered_block_bytes`: Calls a helper routine, and if its result is non-negative copies a number of bytes from a source table into a destination buffer that is read from the structure in a3, decrementing the count each iteration.
- `store_helper_pointer`: Saves registers, calls a helper routine, and if its result is non-negative stores the returned pointer into the field at offset 348 of the structure in a3 before restoring registers and returning.
- `copy_fixed_byte_count`: Saves registers, calls a helper routine, and if its result is non-negative copies a fixed number of bytes from a source table into a destination buffer read from the structure in a3, advancing both pointers each iteration.
- `xor_table_checksum`: Reads a 16-bit value from offset 360 of the structure in a6, multiplies it by sixteen, indexes a table using the result, and XORs a run of words from that table into an accumulator before storing the value and comparing it with a field at offset 6.
- `xor_table_elements`: Calculates a count based on a value at offset 360 of a structure in a6 multiplied by 16, then loops through a table starting at a pointer in a global variable to XOR its elements with a value from another global variable.
- `initialize_and_poll_hardware`: Sets bits in a control byte based on global values and writes it to a hardware register, then polls a status register in a loop that exits early when a bit clears.
- `pulse_control_port_bits`: Writes six byte values to a control port at offset 256 of a structure, each clearing one of five bits in turn and then restoring all bits, as a fixed pulse sequence that takes no data input, and returns.
- `poll_hardware_bit`: Tests bit 4 of a byte at an offset from a pointer in a6; if the bit is set, it calls another routine and loops back to the start of this routine, otherwise it sets d0 to 1 and branches to another routine.
- `clear_global_word`: Clears a 16-bit word at a global address and returns.
- `adjust_global_word`: Compares a global word to zero, calls another routine if it is negative, and then modifies the global word based on the difference between its original value and d0.
- `modify_joypad_status`: Reads a byte from a hardware register, sets the highest bit, and branches to another routine.
- `adjust_global_word_2`: Performs bitwise NOT and OR operations on bytes read from a global address and stores the result in a global variable at offset 256 of a4, then branches to another routine.
- `branch_to_next`: Performs a branch to another address.
- `execute_nested_loops`: Stores a byte from a register into a global variable, then executes two nested loops using a constant count of 520 and a value from a register, and finally returns.
- `init_and_run_status_check`: Saves all registers, initializes several global variables, calls a status routine, and if a passed word is zero calls two more routines before restoring registers and returning a combined value.
- `decrement_and_branch_2`: Writes a word to a global address, saves registers to the stack, calls two other routines, and then enters a loop that decrements d1 to repeat the process.
- `decrement_and_branch`: Restores a pointer from the stack to a0, restores two words to d0 and d1, decrements a global word at offset 38, and then branches based on the result.
- `update_structure_fields`: Writes several words and a longword into various offsets of a structure, then enters a loop that decrements a register based on a value from the structure, and finally writes a word to offset 36 of the structure and returns.
- `set_structure_word`: Stores the word 16384 at offset 36 of the structure in a6 and returns.
- `process_masked_data_2`: Iterates through a table searching for a specific word, calls two other routines, and then uses a loop to copy data from a table into a destination using a value from a register.
- `branch_with_mask`: Loads the value negative 1 into a register and branches to an earlier instruction.
- `process_masked_data`: Saves registers to the stack, calls another routine, masks the high byte of a register, compares it to a constant, and if not equal, increments a pointer and the stack pointer before branching to an earlier instruction.
- `compare_calculated_values`: Calculates a value by adding a word in d2 to itself and adding it to a pointer in a0, then compares this against a value derived from adding a word from a global address and a bit from another global address.
- `combine_masked_values_2`: Loads two words from a table via a pointer in a2, calls another routine, and then performs a loop that decrements d1 to perform bitwise operations and comparisons using d2, d5, d6, and d7.
- `combine_masked_values`: Performs bitwise AND operations on d2 and d7 using d5, then shifts the result of the first AND left by one and ORs it with the result of the second AND.
- `store_zero_byte_21`: Saves registers, stores incoming values into a global pair, then repeatedly adjusts the pair by plus or minus one while calling a helper routine eight times before storing a result into a structure.
- `check_and_store_values`: Checks a word at a global address, and if it is not zero, stores two words from global addresses into offsets 22 and 34 of a structure in a6 before calling another routine.
- `store_and_call_routine`: Stores two words from global addresses into offsets 32 and 34 of the structure in a6 and then calls another routine.
- `set_state_and_call_routine`: Clears a global longword, stores the value 43 at offset 360 of the block in a6, and calls a routine before returning.
- `update_global_pointers_2`: Compares a word from a global address to 28800; if it is not less than that value, it stores a longword and a word into two global addresses.
- `update_global_pointers`: Compares a word from a global address to 28800; if it is not less than that value, it stores a longword and a word into two global addresses and returns.
- `execute_conditional_routines`: Checks if a 16-bit word in a global variable is zero; if so, it sets a hardware register to 8200, executes a sequence of indirect calls and branch routines, otherwise it sets four consecutive words to zero and returns.
- `init_audio_registers_self`: Writes the constant 12 to the second audio period register, sets bit 0 of the DMA control register, sets bit 0 of the blit control register, and stores the address of itself at offset 63784 of a global structure.
- `setup_audio_channels_two_three`: Stores a routine address in a global variable, then copies channel, length, volume, and period data from a table into audio channels two and three and enables DMA channels two and three.
- `copy_audio_volume_registers`: Copies four consecutive words from global variables into the four audio volume registers, then returns.
- `configure_audio_channels`: Writes a value into the second audio volume register, clears the third and fourth audio volume registers, stores a pointer into a global vector table, and returns.
- `clear_audio_and_settings`: Sets the DMA controller, clears the volume of all four audio channels, stores several routine addresses in global variables, and clears six bytes in a global table.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `find_first_set_bit`: Reads a flag byte and returns immediately if it is zero, otherwise clears it, scans bits seven down to zero of a value for the first set bit while decrementing a counter, stores the counter, and loads a longword from a table using an offset derived from the bit index.
- `load_table_data_2`: Sets two hardware registers to 1, calculates an offset from a value in a global variable, and loads five values from a table into a series of global variables using that offset.
- `load_table_data`: Sets two hardware registers to 1, calculates an offset from a value in a global variable, and loads five values from a table into a series of global variables using that offset.
- `configure_audio_channel_zero`: Copies a channel, length, and volume from a table, sets the period of audio channel zero, and enables DMA channel one.
- `modify_global_variable`: Performs bitwise operations on a global variable and conditionally resets it to a specific value or a routine address.
- `set_bit_from_low_three_bits`: Reads the low three bits of a global word, sets the corresponding bit in a global byte, then calls another routine and returns.
- `find_bit_and_load`: Searches for a bit in a bitmask and loads a value from a table using a calculated offset.
- `load_audio_table_pointer`: Enables an audio channel and DMA request, then reads a counter, masks its low three bits, multiplies by fourteen and uses that offset to copy two longwords and three words from a table into a global variable, finally storing a routine address there.
- `load_audio_table_pointer_2`: Enables an audio channel and DMA request, then reads a counter, masks its low three bits, multiplies by fourteen and uses that offset to copy two longwords and three words from a table into a global variable, finally storing a routine address there.
- `configure_audio_channel_zero_2`: Copies a channel, length, and volume from a table, sets the period of audio channel zero, and enables DMA channel one.
- `modify_global_variable_alt`: Performs bitwise negation and conditional assignment to a global variable, potentially setting it to a routine address.
- `set_bit_from_low_bits_second`: Reads the low three bits of a global word, sets the corresponding bit in a different global byte, then calls another routine and returns.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.

### Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 11264-16384 (code never loaded by a reference run)

- `configure_display_and_copper`: Pseudo-code is unavailable because the decompiler timed out, so this describes only the listed facts.
- `initialize_interrupt_state`: Fills seven consecutive table entries with the same constant, sets two state words to fixed values, and writes an interrupt enable mask with the high bit set.
- `decompress_run_length_bitplanes`: Decodes a run-length packed image: a negative count byte repeats the next byte, a positive count copies literal bytes, and skip counts advance over blank rows.
- `compute_integer_square_root`: Computes the integer square root of its input by the bit-pair method, consuming two input bits per iteration and building a partial result.
- `blit_masked_bitplane_rectangle`: Copies a rectangle across four bitplanes from a source address, combining each word with a mask and its complement so that unmasked pixels keep the destination.
- `write_joystick_test_sequence`: Stores one word constant eight times, then a small constant eight times, and finally zero, matching writes to the joystick test register.
- `copy_sixteen_longs_from_table`: Copies sixteen consecutive 32-bit values from a fixed constant table into the buffer that the address register points to.

### Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 529408-534528 (code never loaded by a reference run)

- `blit_shifted_masked_bitplane_row`: Loops over source words, shifting each pair by a bit count and merging them with a mask into four bitplanes spaced evenly apart.

### Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 540672-545792 (code never loaded by a reference run)

- `advance_cycle_counter_modulo_three`: Sets a size field to a fixed value, copies two words from one structure offset to another, and calls an unnamed routine.

### Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 563200-568320 (code never loaded by a reference run)

- `store_wrapped_difference_minus_one`: Subtracts the high word of one input from the low word of another.

### Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 619520-624640 (code never loaded by a reference run)

- `store_word_return_or_byte`: Stores a word argument into a state word and returns the low byte of its input with the high seven bits forced on.

### Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 642048-647168 (code never loaded by a reference run)

- `project_point_with_clip_flags`: Divides the x and y offsets by depth to get screen coordinates, stores the point and its depth, and sets outcode bits against a bounds rectangle.

### Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 657408-658432 (code never loaded by a reference run)

- `copy_indexed_table_pair`: Masks the input word into an even index, then copies two 16-bit entries from two constant tables into fixed destination words.

### Midwinter (1990)(MicroProse)(Fr)[cr PDX].adf bytes 720896-726016 (code never loaded by a reference run)

- `compute_weighted_table_adjustment`: Multiplies two input words, combines the product with a state word using the result of an unnamed routine, and selects an additive adjustment from five table-keyed branches.
- `lookup_even_index_table_word`: Masks the input word to an even index and returns the 16-bit entry of a constant table at that index.
- `interpolate_and_store_values`: Reads a value, stores a table entry at offset 36, then splits the value into three fields and combines them with other registers to compute two quotients that are added to base values and stored at offsets 266 through 272 before calling a final routine.
