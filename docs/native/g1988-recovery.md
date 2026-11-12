# Game 1988 (midwinter (1990)(microprose)(de)[cr supplex]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01988). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904016 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 25 of 25 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 63 of 63 named (0 author symbols), 63 of 63 described (3 by rule, 37 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 223 of 223 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 96256 bytes, 15 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1965 of 2498 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 11728 |
| data | 660351 |
| padding | 1 |
| unused | 231936 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Midwinter (1990)(MicroProse)(De)[cr Supplex].adf:bootblock | 1 | 192 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 310 | 15 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 237 | 22 |
| Midwinter (1990)(MicroProse)(De)[cr Supplex].adf bytes 11264-16384 (code never loaded by a reference run) | 1 | 1125 | 7 |
| Midwinter (1990)(MicroProse)(De)[cr Supplex].adf bytes 247808-252928 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(De)[cr Supplex].adf bytes 259072-264192 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(De)[cr Supplex].adf bytes 270336-275456 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(De)[cr Supplex].adf bytes 461824-466944 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(De)[cr Supplex].adf bytes 473088-478208 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(De)[cr Supplex].adf bytes 484352-489472 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(De)[cr Supplex].adf bytes 495616-500736 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(De)[cr Supplex].adf bytes 506880-510464 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(De)[cr Supplex].adf bytes 511488-512000 (code never loaded by a reference run) | 1 | 106 | 1 |
| Midwinter (1990)(MicroProse)(De)[cr Supplex].adf bytes 529408-534528 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(De)[cr Supplex].adf bytes 574464-579584 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(De)[cr Supplex].adf bytes 585728-590336 (code never loaded by a reference run) | 1 | 3 | 1 |
| Midwinter (1990)(MicroProse)(De)[cr Supplex].adf bytes 597504-602112 (code never loaded by a reference run) | 1 | 11 | 1 |
| Midwinter (1990)(MicroProse)(De)[cr Supplex].adf bytes 608256-613376 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(De)[cr Supplex].adf bytes 619520-624640 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(De)[cr Supplex].adf bytes 642048-647168 (code never loaded by a reference run) | 1 | 69 | 1 |
| Midwinter (1990)(MicroProse)(De)[cr Supplex].adf bytes 657408-658432 (code never loaded by a reference run) | 1 | 73 | 1 |
| Midwinter (1990)(MicroProse)(De)[cr Supplex].adf bytes 687104-692224 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(De)[cr Supplex].adf bytes 709632-714752 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(De)[cr Supplex].adf bytes 720896-726016 (code never loaded by a reference run) | 1 | 367 | 3 |

## Cases handled

- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: custom-format (non-DOS) disk, 1305 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 163 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 55 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: bytes 11264-16384 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: bytes 247808-252928 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: bytes 259072-264192 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: bytes 270336-275456 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: bytes 461824-466944 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: bytes 473088-478208 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: bytes 484352-489472 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: bytes 495616-500736 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: bytes 506880-510464 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: bytes 511488-512000 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: bytes 529408-534528 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: bytes 574464-579584 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: bytes 585728-590336 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: bytes 597504-602112 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: bytes 608256-613376 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: bytes 619520-624640 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: bytes 642048-647168 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: bytes 657408-658432 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: bytes 687104-692224 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: bytes 709632-714752 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: bytes 720896-726016 look like code no run loaded: recovered position-independently (load address unknown)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: 21 logged loads cover 96256 bytes of the image
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: 4 track-loaded blocks found in recovered runtime segments and classified through them
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 11-20 (bytes 5632-10752) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.88)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 33-42 (bytes 16896-22016) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.93)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 44-53 (bytes 22528-27648) outside the filesystem hold data, not code (opcode density 0.019, entropy 4.21)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 55-64 (bytes 28160-33280) outside the filesystem hold data, not code (opcode density 0.011, entropy 6.92)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 66-75 (bytes 33792-38912) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.50)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 77-86 (bytes 39424-44544) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.80)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 88-97 (bytes 45056-50176) outside the filesystem hold data, not code (opcode density 0.007, entropy 5.69)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 99-108 (bytes 50688-55808) outside the filesystem hold data, not code (opcode density 0.013, entropy 6.83)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 110-119 (bytes 56320-61440) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.79)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 121-130 (bytes 61952-67072) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.59)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 132-135 (bytes 67584-69632) outside the filesystem hold data, not code (opcode density 0.011, entropy 5.68)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 137-141 (bytes 70144-72704) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.17)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 143-152 (bytes 73216-78336) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.35)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 154-163 (bytes 78848-83968) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.68)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 165-174 (bytes 84480-89600) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.98)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 176-185 (bytes 90112-95232) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.89)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 187-196 (bytes 95744-100864) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.88)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 198-207 (bytes 101376-106496) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.88)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 209-218 (bytes 107008-112128) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.39)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 220-229 (bytes 112640-117760) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.33)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 231-240 (bytes 118272-123392) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.47)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 242-251 (bytes 123904-129024) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.97)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 253-262 (bytes 129536-134656) outside the filesystem hold data, not code (opcode density 0.011, entropy 5.86)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 264-273 (bytes 135168-140288) outside the filesystem hold data, not code (opcode density 0.007, entropy 5.64)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 275-284 (bytes 140800-145920) outside the filesystem hold data, not code (opcode density 0.014, entropy 6.32)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 286-295 (bytes 146432-151552) outside the filesystem hold data, not code (opcode density 0.033, entropy 6.68)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 297-306 (bytes 152064-157184) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.69)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 319-328 (bytes 163328-168448) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.04)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 341-350 (bytes 174592-179712) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.68)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 352-361 (bytes 180224-185344) outside the filesystem hold data, not code (opcode density 0.037, entropy 7.07)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 363-372 (bytes 185856-190976) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.29)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 374-383 (bytes 191488-196608) outside the filesystem hold data, not code (opcode density 0.012, entropy 6.85)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 385-394 (bytes 197120-202240) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.51)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 396-405 (bytes 202752-207872) outside the filesystem hold data, not code (opcode density 0.018, entropy 6.80)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 407-416 (bytes 208384-213504) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.52)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 418-427 (bytes 214016-219136) outside the filesystem hold data, not code (opcode density 0.012, entropy 6.79)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 429-438 (bytes 219648-224768) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.40)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 440-449 (bytes 225280-230400) outside the filesystem hold data, not code (opcode density 0.025, entropy 6.93)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 451-460 (bytes 230912-236032) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.25)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 462-471 (bytes 236544-241664) outside the filesystem hold data, not code (opcode density 0.045, entropy 6.72)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 473-482 (bytes 242176-247296) outside the filesystem hold data, not code (opcode density 0.011, entropy 6.16)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 495-504 (bytes 253440-258560) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.21)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 517-526 (bytes 264704-269824) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.25)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 539-548 (bytes 275968-281088) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.97)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 550-559 (bytes 281600-286720) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.98)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 561-570 (bytes 287232-292352) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.73)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 572-581 (bytes 292864-297984) outside the filesystem hold data, not code (opcode density 0.011, entropy 6.22)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 583-592 (bytes 298496-303616) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.89)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 594-603 (bytes 304128-309248) outside the filesystem hold data, not code (opcode density 0.014, entropy 6.47)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 605-614 (bytes 309760-314880) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.70)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 616-625 (bytes 315392-320512) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.42)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 627-636 (bytes 321024-326144) outside the filesystem hold data, not code (opcode density 0.015, entropy 6.86)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 638-647 (bytes 326656-331776) outside the filesystem hold data, not code (opcode density 0.019, entropy 6.68)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 649-658 (bytes 332288-337408) outside the filesystem hold data, not code (opcode density 0.016, entropy 6.76)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 660-669 (bytes 337920-343040) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.49)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 671-680 (bytes 343552-348672) outside the filesystem hold data, not code (opcode density 0.012, entropy 6.76)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 682-691 (bytes 349184-354304) outside the filesystem hold data, not code (opcode density 0.014, entropy 6.56)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 693-702 (bytes 354816-359936) outside the filesystem hold data, not code (opcode density 0.015, entropy 6.92)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 704-713 (bytes 360448-365568) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.28)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 715-724 (bytes 366080-371200) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.15)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 726-735 (bytes 371712-376832) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.85)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 737-746 (bytes 377344-382464) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.91)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 748-757 (bytes 382976-388096) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.16)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 759-768 (bytes 388608-393728) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.66)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 770-779 (bytes 394240-399360) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.15)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 781-790 (bytes 399872-404992) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.53)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 792-801 (bytes 405504-410624) outside the filesystem hold data, not code (opcode density 0.013, entropy 6.79)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 803-812 (bytes 411136-416256) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.50)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 814-823 (bytes 416768-421888) outside the filesystem hold data, not code (opcode density 0.013, entropy 6.61)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 825-834 (bytes 422400-427520) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.49)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 836-845 (bytes 428032-433152) outside the filesystem hold data, not code (opcode density 0.010, entropy 5.80)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 847-856 (bytes 433664-438784) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.91)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 858-867 (bytes 439296-444416) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.67)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 869-878 (bytes 444928-450048) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.78)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 880-889 (bytes 450560-455680) outside the filesystem hold data, not code (opcode density 0.008, entropy 5.33)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 891-900 (bytes 456192-461312) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.56)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 913-922 (bytes 467456-472576) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.97)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 935-944 (bytes 478720-483840) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.53)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 957-966 (bytes 489984-495104) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.19)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 979-988 (bytes 501248-506368) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.30)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1001-1010 (bytes 512512-517632) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.55)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1012-1021 (bytes 518144-523264) outside the filesystem hold data, not code (opcode density 0.056, entropy 6.46)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1023-1032 (bytes 523776-528896) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.51)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1045-1054 (bytes 535040-540160) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.31)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1056-1084 (bytes 540672-555520) outside the filesystem hold data, not code (opcode density 0.048, entropy 5.78)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1087-1120 (bytes 556544-573952) outside the filesystem hold data, not code (opcode density 0.039, entropy 5.30)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1232-1241 (bytes 630784-635904) outside the filesystem hold data, not code (opcode density 0.032, entropy 6.42)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1276-1279 (bytes 653312-655360) outside the filesystem hold data, not code (opcode density 0.044, entropy 4.87)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1298-1304 (bytes 664576-668160) outside the filesystem hold data, not code (opcode density 0.045, entropy 5.22)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1306-1307 (bytes 668672-669696) outside the filesystem hold data, not code (opcode density 0.014, entropy 4.70)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1320-1329 (bytes 675840-680960) outside the filesystem hold data, not code (opcode density 0.043, entropy 6.02)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1364-1366 (bytes 698368-699904) outside the filesystem hold data, not code (opcode density 0.053, entropy 4.48)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1368-1373 (bytes 700416-703488) outside the filesystem hold data, not code (opcode density 0.050, entropy 4.82)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1429-1439 (bytes 731648-737280) outside the filesystem hold data, not code (opcode density 0.016, entropy 5.87)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1441-1444 (bytes 737792-739840) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.65)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1450-1459 (bytes 742400-747520) outside the filesystem hold data, not code (opcode density 0.025, entropy 3.82)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1461-1505 (bytes 748032-771072) outside the filesystem hold data, not code (opcode density 0.016, entropy 5.98)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1518-1528 (bytes 777216-782848) hold ASCII text
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1534-1535 (bytes 785408-786432) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.09)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1537-1537 (bytes 786944-787456) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.10)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1540-1549 (bytes 788480-793600) hold ASCII text
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1552-1555 (bytes 794624-796672) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.70)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1562-1571 (bytes 799744-804864) hold ASCII text
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1584-1593 (bytes 811008-816128) outside the filesystem hold data, not code (opcode density 0.039, entropy 5.97)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1650-1659 (bytes 844800-849920) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.04)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1672-1674 (bytes 856064-857600) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.68)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1676-1676 (bytes 858112-858624) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.11)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1697-1698 (bytes 868864-869888) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1749-1751 (bytes 895488-897024) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.44)
- midwinter (1990)(microprose)(de)[cr supplex]/Midwinter (1990)(MicroProse)(De)[cr Supplex].adf: blocks 1753-1754 (bytes 897536-898560) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.13)
- 17 never-loaded region(s), 84480 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 4 packed track region(s), 20480 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Midwinter (1990)(MicroProse)(De)[cr Supplex].adf:bootblock

- `stub_decompile_timeout_boot_entry`: Pseudo-code is unavailable because decompilation timed out.
- `store_pending_interrupt_mask`: If the master interrupt-enable bit of the interrupt enable register is set, stores the bits set in both the enable and request registers, limited to the low 14 bits, in a global variable.
- `set_port_mask_wait_flag`: Writes a fixed value to a global, builds a bit mask from a field in the structure and stores it there, then writes the inverted mask to the second CIA port.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `wait_port_bit_clear_word`: Calls a routine repeatedly while bit four of a byte at a fixed offset from the register pointer stays set, then clears a word field in the structure addressed by the second register.
- `step_drive_head_to_track`: Compares a target value with a current position field and, when they differ, updates the position and issues a series of control-port pulses.
- `pulse_control_port_count`: For each count in its input, writes a pair of control-port values one bit apart so that each pair produces a single pulse, then waits a fixed delay loop before the next pair.
- `store_params_then_loop_calls`: Stores two register values in the structure and makes a sequence of routine calls in a loop, setting a counter field on each pass and checking the sign of a value.
- `start_disk_read_wait_sync`: Sets up a disk transfer by writing fixed values to a structure and to the disk sync word and length registers, then polls a status bit until it sets or a 16-bit counter wraps.
- `decode_disk_sync_sectors`: Scans the buffer for disk sync words and decodes the interleaved odd and even bit fields of each sector, using XOR for the checksum.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `do_nothing`: the routine is empty and performs no operations before returning.
- `process_complex_state`: Checks a bit in a global variable and, if set, clears it and executes a loop that doubles values from a pointer-based source into a memory range.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `branch_locally`: Performs a short branch to a local address.
- `update_global_from_table`: If a certain condition is met, it writes a sequence of bytes from a table into a memory range, or otherwise fills a memory range with an incrementing sequence of bytes.
- `modify_global_with_table_lookup`: Uses a bit-manipulated version of a global variable as an index into a table to update a global variable.
- `update_pointer_with_calculated_value`: Calculates a new value based on a global pointer and writes it to the memory location pointed to by that global pointer.
- `clamp_global_value`: Checks a global variable and, if certain conditions are met, clamps its value within a specific range or sets it to a constant.
- `set_global_to_max`: Sets a global variable to a maximum unsigned value.
- `accumulate_modified_pointer_values`: Checks a byte at an offset from a global pointer, modifies that byte, and then adds the signed or unsigned version of that pointer to two different global counters.
- `accumulate_joystick_input_to_counters`: Reads certain bits from a joystick or mouse register and, based on the state of those bits, adds a modified version of the input value to two global counters.
- `update_flags_from_input`: Checks a global variable and sets specific bits in a global byte if certain bits are absent.
- `branch_to_table_based_on_input`: Reads a byte from a pointer, updates a global variable based on specific bit patterns, and then calls a routine from a table using a calculated address.
- `add_registers_to_global`: Adds the contents of two registers to a global variable.
- `update_global_with_clamping`: Adds a value to a global variable and clamps the result so it does not drop below zero or exceed a certain threshold.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_global_state`: Loops until a global variable is non-zero, then loops until it becomes zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_table_index`: Sets a global variable to a constant and returns a value calculated using a modulo and a lookup in a table based on the global variable.
- `execute_conditional_pointers`: Checks if a global variable is zero, then sets it to a specific address and calls a routine at that address, followed by several other routine calls.
- `set_dma_and_audio_registers`: Sets a global variable to a constant and then loads its address into itself while touching audio and DMA registers.
- `setup_audio_channels_two_three`: Stores a routine address in a global variable, then copies channel, length, volume, and period data from a table into audio channels two and three and enables DMA channels two and three.
- `update_audio_channels`: Copies two 16-bit values from a table into the period and volume registers of audio channels two and three.
- `reset_audio_and_dma`: Sets the DMA controller to enable specific channels, clears the volume of audio channels two and three, and stores a routine address in a global variable.
- `clear_audio_and_settings`: Sets the DMA controller, clears the volume of all four audio channels, stores several routine addresses in global variables, and clears six bytes in a global table.
- `do_nothing_2`: this routine is empty and performs no operations before returning.
- `return_without_action`: The routine consists of a single return instruction and performs no work, so it does nothing beyond returning to its caller.
- `find_bit_and_load_table`: Checks a bit in a value constructed from a global variable, then loads a value from a table into a global variable based on the bit position.
- `load_sequence_from_table`: Loads a sequence of values from a table into a global variable and other memory locations using an offset calculated from a constant.
- `load_audio_channel_zero_data`: Calculates an offset based on a global variable, loads five values from a table at that offset into audio channel zero and other global variables, and enables DMA channel one.
- `configure_audio_channel_zero`: Copies a channel, length, and volume from a table, sets the period of audio channel zero, and enables DMA channel one.
- `modify_global_bit_pattern`: Performs bitwise operations and conditional checks on a global variable, potentially setting it to a constant or a routine address.
- `set_bit_from_table`: Loads a sequence of values from a table into a global variable and then sets a bit in a global variable based on a bitmask.
- `find_bit_and_load_table_alt`: Checks a bit in a value constructed from a global variable, then loads a value from a different table into a global variable based on the bit position.
- `load_sequence_from_table_alt`: Loads a sequence of values from a different table into a global variable and other memory locations using an offset calculated from a constant.
- `load_audio_channel_one_data`: Calculates an offset based on a global variable, loads five values from a table at that offset into audio channel one and other global variables, and enables DMA channel two.
- `configure_audio_channel_one`: Copies a channel, length, volume, and period from a table into audio channel one and enables DMA channel two.
- `modify_global_bit_state`: Tests a bit within a global variable and, if certain conditions are met, resets the global variable to a fixed value and clears it.
- `set_bit_from_table_alt`: Loads a sequence of values from a different table into a global variable and then sets a bit in a global variable based on a bitmask.
- `return_without_effect`: The routine is empty and performs no operations before returning.

### Midwinter (1990)(MicroProse)(De)[cr Supplex].adf bytes 11264-16384 (code never loaded by a reference run)

- `configure_display_and_copper`: Pseudo-code is unavailable because the decompiler timed out, so this describes only the listed facts.
- `initialize_interrupt_state`: Fills seven consecutive table entries with the same constant, sets two state words to fixed values, and writes an interrupt enable mask with the high bit set.
- `decompress_run_length_bitplanes`: Decodes a run-length packed image: a negative count byte repeats the next byte, a positive count copies literal bytes, and skip counts advance over blank rows.
- `compute_integer_square_root`: Computes the integer square root of its input by the bit-pair method, consuming two input bits per iteration and building a partial result.
- `blit_masked_bitplane_rectangle`: Copies a rectangle across four bitplanes from a source address, combining each word with a mask and its complement so that unmasked pixels keep the destination.
- `write_joystick_test_sequence`: Stores one word constant eight times, then a small constant eight times, and finally zero, matching writes to the joystick test register.
- `copy_sixteen_longs_from_table`: Copies sixteen consecutive 32-bit values from a fixed constant table into the buffer that the address register points to.

### Midwinter (1990)(MicroProse)(De)[cr Supplex].adf bytes 511488-512000 (code never loaded by a reference run)

- `update_interrupt_and_dispatch`: Modifies hardware interrupt registers and updates a global variable based on bitwise checks of a word.

### Midwinter (1990)(MicroProse)(De)[cr Supplex].adf bytes 585728-590336 (code never loaded by a reference run)

- `tiny_moveq_tst_rts`: Small routine of 3 instructions that loads 0 into d4, then tests d4, then returns.

### Midwinter (1990)(MicroProse)(De)[cr Supplex].adf bytes 597504-602112 (code never loaded by a reference run)

- `clear_audio_channel_one_volume`: Clears a set of state words through a masked sequence that ends at zero, and touches the audio channel one volume register.

### Midwinter (1990)(MicroProse)(De)[cr Supplex].adf bytes 642048-647168 (code never loaded by a reference run)

- `project_point_with_clip_flags`: Applies a two-by-two rotation from stored coefficients to a point and writes the transformed vector.

### Midwinter (1990)(MicroProse)(De)[cr Supplex].adf bytes 657408-658432 (code never loaded by a reference run)

- `copy_indexed_table_pair`: Masks the input word into an even index, then copies two 16-bit entries from two constant tables into fixed destination words.

### Midwinter (1990)(MicroProse)(De)[cr Supplex].adf bytes 720896-726016 (code never loaded by a reference run)

- `compute_weighted_table_adjustment`: Divides a state word by a table-derived divisor and combines it with the result of an unnamed routine.
- `lookup_even_index_table_word`: Masks the input word to an even index and returns the 16-bit entry of a constant table at that index.
- `update_object_coordinate_fields`: Looks up a value from an array indexed by the low three bits of a computed value and stores it in an object field.
