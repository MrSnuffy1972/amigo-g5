# Game 1989 (midwinter (1990)(microprose)(de)[cr tt]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01989). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904016 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 29 of 29 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 66 of 66 named (0 author symbols), 66 of 66 described (5 by rule, 34 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 223 of 223 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 115200 bytes, 19 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2036 of 2622 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 12258 |
| data | 639338 |
| padding | 4 |
| unused | 252416 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Midwinter (1990)(MicroProse)(De)[cr TT].adf:bootblock | 1 | 152 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 310 | 15 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 237 | 22 |
| Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 11264-16384 (code never loaded by a reference run) | 1 | 1125 | 7 |
| Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 247808-252928 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 259072-264192 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 270336-275456 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 461824-466944 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 473088-478208 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 484352-489472 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 495616-500736 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 506880-510464 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 511488-512000 (code never loaded by a reference run) | 1 | 106 | 1 |
| Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 529408-534528 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 540672-545792 (code never loaded by a reference run) | 1 | 60 | 1 |
| Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 551936-555520 (code never loaded by a reference run) | 1 | 83 | 1 |
| Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 563200-568320 (code never loaded by a reference run) | 1 | 21 | 2 |
| Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 574464-579584 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 585728-590336 (code never loaded by a reference run) | 1 | 3 | 1 |
| Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 597504-602112 (code never loaded by a reference run) | 1 | 11 | 1 |
| Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 608256-613376 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 619520-624640 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 642048-647168 (code never loaded by a reference run) | 1 | 69 | 1 |
| Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 657408-658432 (code never loaded by a reference run) | 1 | 73 | 1 |
| Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 687104-692224 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 709632-714752 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 720896-726016 (code never loaded by a reference run) | 1 | 367 | 3 |
| Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 765952-771072 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: custom-format (non-DOS) disk, 1265 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 163 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 55 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: bytes 11264-16384 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: bytes 247808-252928 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: bytes 259072-264192 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: bytes 270336-275456 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: bytes 461824-466944 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: bytes 473088-478208 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: bytes 484352-489472 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: bytes 495616-500736 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: bytes 506880-510464 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: bytes 511488-512000 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: bytes 529408-534528 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: bytes 540672-545792 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: bytes 551936-555520 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: bytes 563200-568320 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: bytes 574464-579584 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: bytes 585728-590336 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: bytes 597504-602112 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: bytes 608256-613376 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: bytes 619520-624640 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: bytes 642048-647168 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: bytes 657408-658432 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: bytes 687104-692224 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: bytes 709632-714752 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: bytes 720896-726016 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: bytes 765952-771072 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: 25 logged loads cover 115200 bytes of the image
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: 4 track-loaded blocks found in recovered runtime segments and classified through them
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 2-20 (bytes 1024-10752) outside the filesystem hold data, not code (opcode density 0.040, entropy 6.97)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 33-42 (bytes 16896-22016) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.93)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 44-53 (bytes 22528-27648) outside the filesystem hold data, not code (opcode density 0.018, entropy 4.19)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 55-64 (bytes 28160-33280) outside the filesystem hold data, not code (opcode density 0.011, entropy 6.92)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 66-75 (bytes 33792-38912) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.50)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 77-86 (bytes 39424-44544) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.80)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 88-97 (bytes 45056-50176) outside the filesystem hold data, not code (opcode density 0.007, entropy 5.69)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 99-108 (bytes 50688-55808) outside the filesystem hold data, not code (opcode density 0.013, entropy 6.83)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 110-119 (bytes 56320-61440) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.79)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 121-130 (bytes 61952-67072) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.59)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 132-135 (bytes 67584-69632) outside the filesystem hold data, not code (opcode density 0.011, entropy 5.68)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 137-141 (bytes 70144-72704) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.17)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 143-152 (bytes 73216-78336) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.35)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 154-163 (bytes 78848-83968) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.68)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 165-174 (bytes 84480-89600) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.98)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 176-185 (bytes 90112-95232) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.89)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 187-196 (bytes 95744-100864) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.88)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 198-207 (bytes 101376-106496) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.88)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 209-218 (bytes 107008-112128) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.39)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 220-229 (bytes 112640-117760) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.33)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 231-240 (bytes 118272-123392) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.47)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 242-251 (bytes 123904-129024) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.97)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 253-262 (bytes 129536-134656) outside the filesystem hold data, not code (opcode density 0.011, entropy 5.86)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 264-273 (bytes 135168-140288) outside the filesystem hold data, not code (opcode density 0.007, entropy 5.64)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 275-284 (bytes 140800-145920) outside the filesystem hold data, not code (opcode density 0.014, entropy 6.32)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 286-295 (bytes 146432-151552) outside the filesystem hold data, not code (opcode density 0.033, entropy 6.68)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 297-306 (bytes 152064-157184) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.69)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 319-328 (bytes 163328-168448) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.04)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 341-350 (bytes 174592-179712) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.68)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 352-361 (bytes 180224-185344) outside the filesystem hold data, not code (opcode density 0.037, entropy 7.07)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 363-372 (bytes 185856-190976) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.29)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 374-383 (bytes 191488-196608) outside the filesystem hold data, not code (opcode density 0.012, entropy 6.85)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 385-394 (bytes 197120-202240) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.51)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 396-405 (bytes 202752-207872) outside the filesystem hold data, not code (opcode density 0.018, entropy 6.80)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 407-416 (bytes 208384-213504) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.52)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 418-427 (bytes 214016-219136) outside the filesystem hold data, not code (opcode density 0.012, entropy 6.79)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 429-438 (bytes 219648-224768) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.40)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 440-449 (bytes 225280-230400) outside the filesystem hold data, not code (opcode density 0.025, entropy 6.93)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 451-460 (bytes 230912-236032) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.25)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 462-471 (bytes 236544-241664) outside the filesystem hold data, not code (opcode density 0.045, entropy 6.72)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 473-482 (bytes 242176-247296) outside the filesystem hold data, not code (opcode density 0.011, entropy 6.16)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 495-504 (bytes 253440-258560) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.21)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 517-526 (bytes 264704-269824) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.25)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 539-548 (bytes 275968-281088) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.97)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 550-559 (bytes 281600-286720) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.98)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 561-570 (bytes 287232-292352) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.73)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 572-581 (bytes 292864-297984) outside the filesystem hold data, not code (opcode density 0.011, entropy 6.22)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 583-592 (bytes 298496-303616) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.89)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 594-603 (bytes 304128-309248) outside the filesystem hold data, not code (opcode density 0.014, entropy 6.47)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 605-614 (bytes 309760-314880) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.70)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 616-625 (bytes 315392-320512) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.42)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 627-636 (bytes 321024-326144) outside the filesystem hold data, not code (opcode density 0.015, entropy 6.86)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 638-647 (bytes 326656-331776) outside the filesystem hold data, not code (opcode density 0.019, entropy 6.68)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 649-658 (bytes 332288-337408) outside the filesystem hold data, not code (opcode density 0.016, entropy 6.76)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 660-669 (bytes 337920-343040) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.49)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 671-680 (bytes 343552-348672) outside the filesystem hold data, not code (opcode density 0.012, entropy 6.76)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 682-691 (bytes 349184-354304) outside the filesystem hold data, not code (opcode density 0.014, entropy 6.56)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 693-702 (bytes 354816-359936) outside the filesystem hold data, not code (opcode density 0.015, entropy 6.92)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 704-713 (bytes 360448-365568) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.28)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 715-724 (bytes 366080-371200) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.15)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 726-735 (bytes 371712-376832) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.85)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 737-746 (bytes 377344-382464) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.91)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 748-757 (bytes 382976-388096) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.16)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 759-768 (bytes 388608-393728) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.66)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 770-779 (bytes 394240-399360) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.15)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 781-790 (bytes 399872-404992) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.53)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 792-801 (bytes 405504-410624) outside the filesystem hold data, not code (opcode density 0.013, entropy 6.79)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 803-812 (bytes 411136-416256) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.50)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 814-823 (bytes 416768-421888) outside the filesystem hold data, not code (opcode density 0.013, entropy 6.61)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 825-834 (bytes 422400-427520) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.49)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 836-845 (bytes 428032-433152) outside the filesystem hold data, not code (opcode density 0.010, entropy 5.80)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 847-856 (bytes 433664-438784) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.91)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 858-867 (bytes 439296-444416) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.67)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 869-878 (bytes 444928-450048) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.78)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 880-889 (bytes 450560-455680) outside the filesystem hold data, not code (opcode density 0.008, entropy 5.33)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 891-900 (bytes 456192-461312) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.56)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 913-922 (bytes 467456-472576) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.97)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 935-944 (bytes 478720-483840) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.53)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 957-966 (bytes 489984-495104) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.19)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 979-988 (bytes 501248-506368) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.30)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 1001-1010 (bytes 512512-517632) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.55)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 1012-1021 (bytes 518144-523264) outside the filesystem hold data, not code (opcode density 0.056, entropy 6.46)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 1023-1032 (bytes 523776-528896) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.51)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 1045-1054 (bytes 535040-540160) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.31)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 1067-1076 (bytes 546304-551424) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.88)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 1087-1087 (bytes 556544-557056) outside the filesystem hold data, not code (opcode density 0.008, entropy 0.38)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 1089-1098 (bytes 557568-562688) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.44)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 1111-1120 (bytes 568832-573952) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.37)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 1232-1241 (bytes 630784-635904) outside the filesystem hold data, not code (opcode density 0.032, entropy 6.42)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 1276-1279 (bytes 653312-655360) outside the filesystem hold data, not code (opcode density 0.044, entropy 4.87)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 1298-1304 (bytes 664576-668160) outside the filesystem hold data, not code (opcode density 0.045, entropy 5.22)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 1306-1307 (bytes 668672-669696) outside the filesystem hold data, not code (opcode density 0.014, entropy 4.70)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 1320-1329 (bytes 675840-680960) outside the filesystem hold data, not code (opcode density 0.043, entropy 6.02)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 1364-1366 (bytes 698368-699904) outside the filesystem hold data, not code (opcode density 0.053, entropy 4.48)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 1368-1373 (bytes 700416-703488) outside the filesystem hold data, not code (opcode density 0.050, entropy 4.82)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 1430-1439 (bytes 732160-737280) outside the filesystem hold data, not code (opcode density 0.013, entropy 5.82)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 1452-1459 (bytes 743424-747520) outside the filesystem hold data, not code (opcode density 0.031, entropy 4.33)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 1461-1461 (bytes 748032-748544) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.51)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 1474-1483 (bytes 754688-759808) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.30)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 1518-1527 (bytes 777216-782336) hold ASCII text
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 1540-1549 (bytes 788480-793600) hold ASCII text
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 1562-1571 (bytes 799744-804864) hold ASCII text
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 1584-1593 (bytes 811008-816128) outside the filesystem hold data, not code (opcode density 0.038, entropy 5.96)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 1650-1659 (bytes 844800-849920) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.04)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 1672-1674 (bytes 856064-857600) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.68)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 1676-1676 (bytes 858112-858624) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.11)
- midwinter (1990)(microprose)(de)[cr tt]/Midwinter (1990)(MicroProse)(De)[cr TT].adf: blocks 1697-1698 (bytes 868864-869888) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- 21 never-loaded region(s), 103424 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 4 packed track region(s), 20480 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Midwinter (1990)(MicroProse)(De)[cr TT].adf:bootblock

- `boot_entry_mask_interrupts`: Pseudo-code is unavailable because the decompiler timed out, so this describes only the listed facts.
- `capture_pending_interrupt_bits`: When the interrupt master-enable bit is set, it stores the bits common to the interrupt enable and request registers, masked to their low fourteen bits, into a variable.
- `configure_disk_dma_and_wait`: Configures a hardware register with a specific bitmask based on values in a structure, sets three bytes in a memory buffer, and enters a loop that performs a large multiplication calculation before checking a bit in the buffer to decide whether to return or branch to another routine.
- `pulse_control_port_bits`: Writes six fixed byte values to a control port, clearing one of five bits in turn and then restoring all bits.
- `wait_status_bit_then_clear`: Calls an unnamed routine repeatedly while a status bit on a port stays set, then clears a word field of a structure.
- `step_pulses_toward_target`: Compares an input value with a stored position and, when they differ, updates the stored value and selects a direction bit.
- `pulse_control_port_repeatedly`: Repeats a two-stage port write for the count in its input, with a fixed delay after each pair.
- `poll_with_retry_count`: Stores two input words in a structure, then loops over a retry counter starting at two while calling unnamed routines to test a condition.
- `forbid_and_configure_dma`: The pseudo-code is mostly a pass-through of its input and does not show the listed behaviour.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_registers_and_return`: Restores registers a0 and d1 through d6 from the stack and returns.
- `process_complex_state`: Checks a bit in a hardware register and, if set, increments two global counters, calls a routine, compares a global variable to a constant to call one of two other routines, and if a pointer in a global variable is non-zero, loops to double sixteen values from that pointer into a hardware register range before clearing the pointer and calling several other routines.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_to_offset`: Performs a short branch to a specific offset.
- `update_global_from_table`: If a global variable matches a specific constant, it either fills a memory range with an incrementing sequence of bytes or, if another condition is met, writes a sequence of bytes from a table into a memory range.
- `update_global_via_table_lookup`: Uses a bit-manipulated version of a global variable as an index into a table to update two global variables.
- `update_joystick_position_values`: Checks a global variable; if it does not equal two, it calculates two new 16-bit values using arithmetic on that variable and two other global values, then writes these values to a memory location pointed to by a third global variable.
- `clamp_global_value`: Checks a global variable and, if certain conditions are met, clamps its value within a specific range or sets it to a constant.
- `set_global_to_max`: Sets a global variable to a constant value of sixty-five thousand five hundred thirty-five and returns.
- `accumulate_modified_pointer_values`: Checks a byte at an offset from a global pointer, modifies that byte, and then adds the signed or unsigned version of that pointer to two different global counters.
- `accumulate_joystick_input_to_counters`: Reads certain bits from a joystick or mouse register and, based on the state of those bits, adds a modified version of the input value to two global counters.
- `update_input_coordinates`: Checks a bit in a global variable and a bit in a hardware register to set or clear bits in a structure at offset 0, then calculates two values from a hardware register and adds them to the structure at offsets 2 and 6.
- `dispatch_via_table`: Sets several global variables based on bits in a byte from a pointer, and if two other longwords are zero, calls a routine from a table using an index derived from a global variable.
- `add_to_globals`: Adds the contents of d1 and d0 to two different global variables and returns.
- `update_global_with_clamping`: Adds a value to a global variable and clamps the result so it does not drop below zero or exceed 198.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_global_state`: Loops until a global variable is non-zero, then loops until it becomes zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_table_index`: Calls another routine, sets a global variable to 65535, and then enters a loop that waits for a global variable to fall within the range of 90 to 170.
- `execute_conditional_pointers`: Checks if a global variable is zero, then sets it to a specific value, calls a routine at an address from a pointer, and performs several other routine calls; otherwise, it clears four consecutive longwords.
- `set_dma_and_audio_registers`: Sets three global variables to constant values and writes to audio and DMA control registers.
- `setup_audio_channels_two_three`: Stores a routine address in a global variable, then copies channel, length, volume, and period data from a table into audio channels two and three and enables DMA channels two and three.
- `update_audio_channels`: Copies two 16-bit values from a table into the period and volume registers of audio channels two and three.
- `reset_audio_and_dma`: Sets the DMA controller to enable specific channels, clears the volume of audio channels two and three, and stores a routine address in a global variable.
- `clear_audio_and_settings`: Sets the DMA controller, clears the volume of all four audio channels, stores several routine addresses in global variables, and clears six bytes in a global table.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `find_bit_and_load_table`: Checks a bit in a value constructed from a global variable, then loads a value from a table into a global variable based on the bit position.
- `load_sequence_from_table`: Enables audio DMA and copper DMA, then calculates an offset from a global variable to load a sequence of two 32-bit and three 16-bit values from a table into several global variables.
- `load_sequence_from_table_3`: Loads a sequence of values from a table into a global variable and other memory locations using an offset calculated from a constant.
- `configure_audio_channel_zero`: Copies a channel, length, and volume from a table, sets the period of audio channel zero, and enables DMA channel one.
- `modify_global_bit_pattern`: Performs bitwise operations and conditional checks on a global variable, potentially setting it to a constant or a routine address while modifying audio and DMA registers.
- `set_bit_from_table`: Calculates a bit position from a word in a global variable, sets that bit in a byte from another global variable, and calls another routine.
- `find_bit_and_load_table_alt`: Checks a bit in a value constructed from a global variable, then loads a value from a different table into a global variable based on the bit position.
- `load_sequence_from_table_2`: Sets two hardware registers, calculates an offset from a global variable by masking it with seven and multiplying by fourteen, then loads a sequence of two 32-bit and three 16-bit values from a table into several global variables using that offset.
- `load_sequence_from_table_alt`: Loads a sequence of values from a different table into a global variable and other memory locations using an offset calculated from a constant.
- `configure_audio_channel_one`: Copies a channel, length, volume, and period from a table into audio channel one and enables DMA channel two.
- `modify_global_bit_state`: Tests a bit within a global variable and, if certain conditions are met, resets the global variable to a fixed value and clears it while modifying audio and DMA registers.
- `set_bit_from_masked_index`: Extracts a bit index by masking a word from a global variable with seven, sets that bit in a byte from another global variable, and calls a subroutine.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.

### Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 11264-16384 (code never loaded by a reference run)

- `configure_display_and_copper`: Pseudo-code is unavailable because the decompiler timed out, so this describes only the listed facts.
- `initialize_interrupt_state`: Fills seven consecutive table entries with the same constant, sets two state words to fixed values, and writes an interrupt enable mask with the high bit set.
- `decompress_run_length_bitplanes`: Decodes a run-length packed image: a negative count byte repeats the next byte, a positive count copies literal bytes, and skip counts advance over blank rows.
- `compute_integer_square_root`: Computes the integer square root of its input by the bit-pair method, consuming two input bits per iteration and building a partial result.
- `blit_masked_bitplane_rectangle`: Copies a rectangle across four bitplanes from a source address, combining each word with a mask and its complement so that unmasked pixels keep the destination.
- `write_joystick_test_sequence`: Stores one word constant eight times, then a small constant eight times, and finally zero, matching writes to the joystick test register.
- `copy_sixteen_longs_from_table`: Copies sixteen consecutive 32-bit values from a fixed constant table into the buffer that the address register points to.

### Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 511488-512000 (code never loaded by a reference run)

- `update_interrupt_and_dispatch`: Modifies hardware interrupt registers and updates a global variable based on bitwise checks of a word.

### Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 540672-545792 (code never loaded by a reference run)

- `advance_cycle_counter_modulo_three`: Sets a size field from an input word offset by a fixed amount, copies two words between structure offsets, and calls an unnamed routine.

### Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 551936-555520 (code never loaded by a reference run)

- `select_quadrant_from_signs`: XORs a value into a stored longword and compares an input word with another word.

### Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 563200-568320 (code never loaded by a reference run)

- `call_unnamed_routine_then_return`: Calls one unnamed routine and returns, with no other work of its own.
- `multiply_add_fold_store`: Multiplies two words, adds the first word to the product, and subtracts the high half of the sum from its low half.

### Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 585728-590336 (code never loaded by a reference run)

- `tiny_moveq_tst_rts`: Small routine of 3 instructions that loads 0 into d4, then tests d4, then returns.

### Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 597504-602112 (code never loaded by a reference run)

- `clear_audio_channel_one_volume`: Clears a set of state words through a masked sequence that ends at zero, and touches the audio channel one volume register.

### Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 642048-647168 (code never loaded by a reference run)

- `project_point_with_clip_flags`: Applies a two-by-two rotation from stored coefficients to a point and writes the transformed vector.

### Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 657408-658432 (code never loaded by a reference run)

- `copy_indexed_table_pair`: Masks the input word into an even index, then copies two 16-bit entries from two constant tables into fixed destination words.

### Midwinter (1990)(MicroProse)(De)[cr TT].adf bytes 720896-726016 (code never loaded by a reference run)

- `compute_weighted_table_adjustment`: Divides a state word by a table-derived divisor and combines it with the result of an unnamed routine.
- `lookup_even_index_table_word`: Masks the input word to an even index and returns the 16-bit entry of a constant table at that index.
- `update_object_coordinate_fields`: Looks up a value from an array indexed by the low three bits of a computed value and stores it in an object field.
