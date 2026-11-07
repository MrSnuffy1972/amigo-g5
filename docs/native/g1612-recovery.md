# Game 1612 (rainbow islands (1987)(ocean)[cr vf][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01612). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (909120 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 12 of 12 named (0 author symbols), 12 of 12 described (1 by rule, 3 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 48 of 48 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 95744 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 247 of 247 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 3977 |
| code | 750 |
| data | 707375 |
| padding | 922 |
| unused | 196096 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Rainbow Islands (1987)(Ocean)[cr VF][a].adf:bootblock | 1 | 15 | 1 |
| logged disk load from Rainbow Islands (1987)(Ocean)[cr VF][a].adf boot-block load | 1 | 167 | 2 |
| logged disk load from Rainbow Islands (1987)(Ocean)[cr VF][a].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 40 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 3 |
| Rainbow Islands (1987)(Ocean)[cr VF][a].adf bytes 34816-88576 (code never loaded by a reference run) | 1 | 0 | 0 |
| Rainbow Islands (1987)(Ocean)[cr VF][a].adf bytes 91648-120832 (code never loaded by a reference run) | 1 | 0 | 0 |
| Rainbow Islands (1987)(Ocean)[cr VF][a].adf bytes 256000-267264 (code never loaded by a reference run) | 1 | 0 | 0 |
| Rainbow Islands (1987)(Ocean)[cr VF][a].adf bytes 889344-890880 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: file cod (header 98 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: file cod (header 98 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: file cod (header 98 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: file cod (header 98 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: the boot block reads 5632 bytes from disk offset 878592 to an address (found statically); recovered as a unit
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: the boot block reads 1024 bytes from disk offset 884224 to an address (found statically); recovered as a unit
- runtime segment an address: 30 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 17 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: bytes 34816-88576 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: bytes 91648-120832 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: bytes 256000-267264 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: bytes 889344-890880 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: 6 logged loads cover 100864 bytes of the image
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: 1 track-loaded blocks found in recovered runtime segments and classified through them
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 2-2 (bytes 1024-1536) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.65)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 22-29 (bytes 11264-15360) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.12)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 32-39 (bytes 16384-20480) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.95)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 65-66 (bytes 33280-34304) outside the filesystem hold data, not code (opcode density 0.002, entropy 1.29)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 242-498 (bytes 123904-255488) outside the filesystem hold data, not code (opcode density 0.013, entropy 4.45)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 523-523 (bytes 267776-268288) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.61)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 525-525 (bytes 268800-269312) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.48)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 527-551 (bytes 269824-282624) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.26)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 553-727 (bytes 283136-372736) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.76)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 729-823 (bytes 373248-421888) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.17)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 825-879 (bytes 422400-450560) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.94)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 881-925 (bytes 451072-474112) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.26)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 927-1021 (bytes 474624-523264) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.51)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1039-1120 (bytes 531968-573952) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.59)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1122-1149 (bytes 574464-588800) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.63)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1151-1164 (bytes 589312-596480) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.87)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1166-1208 (bytes 596992-619008) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.14)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1210-1273 (bytes 619520-652288) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.25)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1275-1336 (bytes 652800-684544) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.36)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1341-1341 (bytes 686592-687104) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.65)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1347-1347 (bytes 689664-690176) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.79)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1349-1400 (bytes 690688-717312) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.77)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1407-1407 (bytes 720384-720896) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.60)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1418-1418 (bytes 726016-726528) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.50)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1429-1429 (bytes 731648-732160) hold ASCII text
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1440-1440 (bytes 737280-737792) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.95)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1451-1451 (bytes 742912-743424) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.04)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1462-1473 (bytes 748544-754688) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.05)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1484-1484 (bytes 759808-760320) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.79)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1495-1495 (bytes 765440-765952) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.92)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1506-1506 (bytes 771072-771584) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.04)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1517-1517 (bytes 776704-777216) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.06)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1528-1528 (bytes 782336-782848) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.58)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1539-1539 (bytes 787968-788480) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.57)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1550-1550 (bytes 793600-794112) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.41)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1561-1561 (bytes 799232-799744) hold ASCII text
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1572-1572 (bytes 804864-805376) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.37)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1583-1583 (bytes 810496-811008) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.65)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1594-1594 (bytes 816128-816640) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.54)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1605-1606 (bytes 821760-822784) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.84)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1616-1616 (bytes 827392-827904) outside the filesystem hold data, not code (opcode density 0.008, entropy 3.54)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1627-1627 (bytes 833024-833536) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.47)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1638-1638 (bytes 838656-839168) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.03)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1649-1649 (bytes 844288-844800) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.40)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1660-1660 (bytes 849920-850432) hold ASCII text
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1671-1671 (bytes 855552-856064) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.58)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1682-1682 (bytes 861184-861696) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.10)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1693-1693 (bytes 866816-867328) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.69)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1704-1704 (bytes 872448-872960) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.97)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1715-1715 (bytes 878080-878592) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.60)
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1748-1748 (bytes 894976-895488) hold ASCII text
- rainbow islands (1987)(ocean)[cr vf][a]/Rainbow Islands (1987)(Ocean)[cr VF][a].adf: blocks 1759-1759 (bytes 900608-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.00)
- 4 never-loaded region(s), 95744 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 7680 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Rainbow Islands (1987)(Ocean)[cr VF][a].adf:bootblock

- `issue_boot_io_request`: Boot block entry that issues an exec DoIO request and calls a library routine at offset 456.

### logged disk load from Rainbow Islands (1987)(Ocean)[cr VF][a].adf boot-block load

- `update_background_color`: This routine is called by a function that writes a background color.
- `write_background_color_then_call`: Writes the background color register and calls one unnamed routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_indirect_call`: Invokes a routine via an address formed by adding two values together.
- `increment_counter_and_call_handler`: Increments a global counter, then calls a routine whose address is read from a pointer cell plus a 16-bit offset stored 38 bytes into the structure it points to.
- `dispatch_pending_interrupt_levels`: Reads the interrupt enable and request registers, masks the result to interrupt bits four to six, writes the masked bits to the interrupt request register, and calls up to three level handlers through function cells according to those bits.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_indirect_call_2`: Loads a 32-bit address from memory by adding a word-sized value to a pointer, then jumps to that address via a subroutine call.
- `store_word_triple`: Copies three consecutive 16-bit values from a source pointer into three consecutive 16-bit fields of a structure at fixed offsets, and returns minus one.
- `execute_callback_chain`: Walks a singly linked list starting from a pointer held in a structure, calling the routine at offset 16 in each node until a node's next pointer is zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_once_then_increment_counter`: Calls a routine through a table entry when a 16-bit counter in the structure is zero, then increments the counter.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `call_structure_callback`: Calls the routine whose address is stored 20 bytes into the structure pointed to by the base register, then returns.
