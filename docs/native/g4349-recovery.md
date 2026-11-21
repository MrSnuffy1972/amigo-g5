# Game 4349 (maze, the (1990)(ensignia)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04349). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (907440 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 42 of 42 named (0 author symbols), 42 of 42 described (9 by rule, 22 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 167 of 167 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 512 bytes, 3 functions |
| Independent decoder cross-check (Ghidra against capstone) | 687 of 687 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 9316 |
| code | 2632 |
| data | 626673 |
| padding | 19 |
| unused | 268800 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Maze, The (1990)(Ensignia).adf:bootblock | 1 | 7 | 1 |
| logged disk load from Maze, The (1990)(Ensignia).adf boot-block load | 1 | 47 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 129 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 407 | 33 |
| Maze, The (1990)(Ensignia).adf bytes 12288-12800 (code never loaded by a reference run) | 1 | 97 | 3 |

## Cases handled

- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: custom-format (non-DOS) disk, 1232 non-empty blocks: matched against track-loaded segments
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: the boot block reads 2048 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 125 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 42 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: bytes 12288-12800 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 jump target
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: 2 logged loads cover 2560 bytes of the image
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: 7 track-loaded blocks found in recovered runtime segments and classified through them
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 6-22 (bytes 3072-11776) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.69)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 180-250 (bytes 92160-128512) outside the filesystem hold data, not code (opcode density 0.006, entropy 3.22)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 416-458 (bytes 212992-235008) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.09)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 468-468 (bytes 239616-240128) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.11)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 478-478 (bytes 244736-245248) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.10)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 480-502 (bytes 245760-257536) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.30)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 507-519 (bytes 259584-266240) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.23)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 533-533 (bytes 272896-273408) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.10)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 539-539 (bytes 275968-276480) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.11)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 544-566 (bytes 278528-290304) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.28)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 568-581 (bytes 290816-297984) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.17)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 588-588 (bytes 301056-301568) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.10)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 601-602 (bytes 307712-308736) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 608-630 (bytes 311296-323072) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.28)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 632-645 (bytes 323584-330752) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.17)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 652-652 (bytes 333824-334336) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.10)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 665-666 (bytes 340480-341504) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 672-694 (bytes 344064-355840) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.28)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 696-709 (bytes 356352-363520) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.17)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 716-716 (bytes 366592-367104) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.10)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 729-730 (bytes 373248-374272) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 736-758 (bytes 376832-388608) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.28)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 760-773 (bytes 389120-396288) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.17)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 780-780 (bytes 399360-399872) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.10)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 793-794 (bytes 406016-407040) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 800-822 (bytes 409600-421376) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.28)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 824-837 (bytes 421888-429056) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.17)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 844-844 (bytes 432128-432640) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.10)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 857-858 (bytes 438784-439808) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 864-886 (bytes 442368-454144) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.28)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 888-901 (bytes 454656-461824) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.17)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 908-908 (bytes 464896-465408) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.10)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 921-922 (bytes 471552-472576) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 928-950 (bytes 475136-486912) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.28)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 952-965 (bytes 487424-494592) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.17)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 972-972 (bytes 497664-498176) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.10)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 985-986 (bytes 504320-505344) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 992-1014 (bytes 507904-519680) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.28)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1016-1029 (bytes 520192-527360) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.17)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1036-1036 (bytes 530432-530944) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.10)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1049-1050 (bytes 537088-538112) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1056-1078 (bytes 540672-552448) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.28)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1080-1093 (bytes 552960-560128) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.17)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1100-1100 (bytes 563200-563712) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.10)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1113-1114 (bytes 569856-570880) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1120-1142 (bytes 573440-585216) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.28)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1144-1157 (bytes 585728-592896) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.17)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1164-1164 (bytes 595968-596480) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.10)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1177-1178 (bytes 602624-603648) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1184-1206 (bytes 606208-617984) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.28)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1208-1221 (bytes 618496-625664) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.17)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1228-1228 (bytes 628736-629248) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.10)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1241-1242 (bytes 635392-636416) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1248-1270 (bytes 638976-650752) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.28)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1272-1285 (bytes 651264-658432) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.17)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1292-1292 (bytes 661504-662016) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.10)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1305-1306 (bytes 668160-669184) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1312-1334 (bytes 671744-683520) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.28)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1336-1349 (bytes 684032-691200) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.17)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1356-1356 (bytes 694272-694784) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.10)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1369-1370 (bytes 700928-701952) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1376-1398 (bytes 704512-716288) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.28)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1400-1413 (bytes 716800-723968) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.17)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1420-1420 (bytes 727040-727552) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.10)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1433-1434 (bytes 733696-734720) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1440-1462 (bytes 737280-749056) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.28)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1464-1477 (bytes 749568-756736) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.17)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1484-1484 (bytes 759808-760320) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.10)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1497-1498 (bytes 766464-767488) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1504-1526 (bytes 770048-781824) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.28)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1528-1541 (bytes 782336-789504) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.17)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1548-1548 (bytes 792576-793088) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.10)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1561-1562 (bytes 799232-800256) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1568-1590 (bytes 802816-814592) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.28)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1592-1605 (bytes 815104-822272) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.17)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1612-1612 (bytes 825344-825856) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.10)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1625-1626 (bytes 832000-833024) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1632-1632 (bytes 835584-836096) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1634-1654 (bytes 836608-847360) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.05)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1674-1674 (bytes 857088-857600) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.10)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1694-1694 (bytes 867328-867840) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.11)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1696-1738 (bytes 868352-890368) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.53)
- maze, the (1990)(ensignia)/Maze, The (1990)(Ensignia).adf: blocks 1757-1758 (bytes 899584-900608) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- 2 packed track region(s), 154112 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Maze, The (1990)(Ensignia).adf:bootblock

- `frag_movea_move_jsr_jmp`: Short code fragment that copies a fixed memory address to a6, then copies 2 to the memory at an offset from a1, then copies an address to the memory at an offset from a1, then copies 2048 to the memory at an offset from a1, then copies 1024 to the memory at an offset from a1, then calls another routine, then jumps to another point in the code.

### logged disk load from Maze, The (1990)(Ensignia).adf boot-block load

- `initialize_video_mode`: Sets up video display registers with specific values for screen mode and memory addresses, then calls a system function to configure hardware components.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `shift_and_store`: Performs bit shifting operations on a value in a register, stores results to memory locations, and manages control flow based on shift outcomes.
- `rotate_and_store`: Loads a long word from memory, rotates it right through carry, stores the lower byte to a global variable, and returns the rotated value with the most significant bit set.
- `recursive_bit_shift_loop`: Performs bit shifting operations in a loop that decrements a counter, calls another routine when the shift result is zero, and continues until the counter reaches zero.
- `initialize_graphics_system`: Initializes graphics hardware by setting up DMA channels, configuring interrupt enables, and copying data structures to specific memory locations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_music_sample_table`: Searches a byte table for the smallest value, then builds a pointer table by linking zeroed entries to sample start addresses indexed from a second table, sets bit 1 of a global variable, clears several global variables, and returns.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `frag_movem_lea_addq_move_cmp_blt_clr_br`: Short code fragment that saves registers on the stack, then loads the address of a fixed memory address into a0, then adds 1 to a fixed memory address, then copies a fixed memory address to d0, then compares d0 with a fixed memory address, then branches if the comparison was less, then clears a fixed memory address, then jumps to another point in the code.
- `update_audio_channels`: Configures audio hardware channels by loading DMA addresses and lengths from lookup tables, then updates the audio registers for four channels in sequence.
- `compute_division_result`: Loads a byte value, performs division by three, swaps the result, compares against zero or two, and branches to different code paths based on comparison results.
- `frag_moveq_move_andi_bra`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_move_bra`: Short code fragment that copies the memory at an offset from a6 to d2, then jumps to another point in the code.
- `calculate_lookup_index`: Performs bit manipulation operations on a word value from the stack, uses it as an index into a lookup table, and compares against another value.
- `frag_move_rts`: Short code fragment that copies d2 to the memory at an offset from a5, then returns.
- `process_maze_audio_data`: Processes maze-related data to compute audio parameters, clears a global variable, and writes computed values to multiple audio hardware registers while performing loop delays.
- `update_maze_data`: Processes maze-related data by extracting and combining byte values, performing calculations based on a lookup table, and updating multiple memory locations with computed results.
- `update_and_compare`: Updates several memory locations with calculated values, performs bitwise operations on a word value, checks for zero condition, and branches based on comparison results with specific constants.
- `update_dma_and_registers`: Copies word values between stack locations, updates hardware registers including DMA control, clears a byte flag, and performs bit operations before branching to another routine.
- `update_audio_hardware`: Updates audio hardware registers by setting up DMA control and channel addresses/lengths from lookup tables, increments a global counter, and resets it when reaching 400 while updating other global variables.
- `frag_move_andi_clr_cmp_beq_bge_rts`: Short code fragment that copies the memory at a6 to d2, then masks d2 with 4095, then copies d2 to the memory at an offset from a6, then copies the memory at an offset from a6 to d0, then clears the memory at an offset from a6, then compares d2 with d0, then branches if the last result was zero, then branches if the comparison was greater or equal, then copies 1 to the memory at an offset from a6, then returns.
- `frag_clr_rts`: Short code fragment that clears the memory at an offset from a6, then returns.
- `adjust_and_store_value_2`: Moves a byte value to another location, clears the source byte, tests a word value, and performs arithmetic operations on a counter based on comparisons with another value.
- `adjust_and_store_value`: Subtracts a value from a counter, compares the result with another value, updates the counter if necessary, clears a second counter, and stores the final counter value.
- `compute_scaled_value`: Processes byte values through bit shifting, masking, and multiplication to compute a scaled result that gets added to a word value.
- `subtract_and_advance_phase`: Subtracts d2 from d0 and stores the result in offset 6 of a structure in a5, then advances a phase byte in the structure by an amount taken from the high nibble of a neighbouring byte.
- `frag_move_rts_2`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a5, then returns.
- `process_direction_input`: Processes a direction input value from a structure, performs conditional operations based on the input, and stores computed results into another structure.
- `update_counter_with_clamp`: Loads a byte value, shifts it right by four bits, adds it to a counter, clamps the counter at maximum value of sixty-four, and stores the result.
- `decrement_counter_clamped_zero`: Subtracts the low nibble of a parameter byte from a 16-bit counter, clamps the counter at zero, and copies it to an output field.
- `subtract_and_clamp_word`: Subtracts a byte from a word, clamps the result to a minimum threshold, and stores the lower twelve bits of the result into a global variable.
- `add_and_clamp_word`: Adds a byte to a word, clamps the result to a maximum threshold, and stores the lower twelve bits of the result into a global variable.
- `handle_input_state`: Compares a byte value against several constants and performs different operations based on the comparison results, including bit manipulation and global variable updates.
- `update_global_from_ciaa_pra`: Masks bit zero of the byte at offset 3 of the structure in a6, shifts it left by one, and ORs the result into a global variable after clearing bit one of that variable.
- `frag_not_rts`: Short code fragment that inverts a fixed memory address, then returns.
- `invert_byte_from_pointer`: The routine reads a byte from an address computed from an uninitialized pointer plus a small offset, subtracts one from it, then stores the bitwise NOT of the result into a global variable.
- `clamp_parameter_to_maximum`: Clamps a parameter byte to a maximum of 64 and copies the result to an output field.
- `update_global_with_mask`: Checks if the lower five bits of a byte at offset 3 of a structure in a6 are non-zero; if so, it clears a global longword and updates another global longword with the masked value.
- `validate_and_process_input`: Checks word values against specific ranges and jumps to different code locations based on comparisons, with a final return after conditional processing.

### Maze, The (1990)(Ensignia).adf bytes 12288-12800 (code never loaded by a reference run)

- `shift_and_branch_2`: Performs bitwise right shift operations on a value while decrementing a counter, calls another routine when the shifted result equals zero.
- `load_rotate_store`: Loads a long word from memory, extracts its lower byte, stores it to a global variable, sets the processor status register, and returns the rotated value with the most significant bit set.
- `shift_and_branch`: Performs bitwise right shift operations on a value while decrementing a counter, calls another routine when the shifted result equals zero, and continues until the counter reaches zero.
