# Game 1504 (dyna blaster (1991)(hudson soft)[cr vf][t +11 znt]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01504). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (912064 top-level bytes in 21 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 19 raw-word lines |
| Functions named and described | pass, 172 of 172 named (0 author symbols), 172 of 172 described (4 by rule, 158 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1529 of 1529 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 700564 bytes, 19 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3518 of 3518 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 37678 |
| code | 8714 |
| data | 820054 |
| padding | 50 |
| unused | 45568 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf:bootblock | 1 | 20 | 2 |
| logged disk load from Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf boot-block load | 1 | 196 | 35 |
| logged disk load from Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf boot-block load | 1 | 194 | 36 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 691 | 24 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1298 | 54 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 2 |
| Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf:blocks 242-289 (bytes 123904-148480)#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf:blocks 297-341 (bytes 152064-175104)#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf:blocks 352-458 (bytes 180224-235008)#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf:blocks 1100-1372 (bytes 563200-702976)#unpacked (raw code file, load address unknown) | 1 | 1105 | 19 |

## Cases handled

- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf: custom-format (non-DOS) disk, 1669 non-empty blocks: matched against track-loaded segments
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf: the boot block reads 19968 bytes from disk offset 253952 to an address (found statically); recovered as a unit
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf: the boot block reads 17408 bytes from disk offset 236544 to an address (found statically); recovered as a unit
- runtime segment an address: 659 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 859 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 11 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf: 2 logged loads cover 37376 bytes of the image
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf: blocks 2-8 (bytes 1024-4608) outside the filesystem hold data, not code (opcode density 0.039, entropy 6.87)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf: blocks 10-235 (bytes 5120-120832) hold 1 packed streams (IMP: File Imploder) found by scanning, 99% of the bytes
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf: blocks 242-289 (bytes 123904-148480) hold a IMP: File Imploder stream (24576 -> 113460 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf:blocks 242-289 (bytes 123904-148480)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf: blocks 297-341 (bytes 152064-175104) hold a IMP: File Imploder stream (23040 -> 209510 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf:blocks 297-341 (bytes 152064-175104)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf: blocks 352-458 (bytes 180224-235008) hold a IMP: File Imploder stream (54784 -> 177594 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf:blocks 352-458 (bytes 180224-235008)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf: blocks 1067-1098 (bytes 546304-562688) outside the filesystem hold data, not code (opcode density 0.013, entropy 7.09)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf: blocks 1100-1372 (bytes 563200-702976) hold a IMP: File Imploder stream (139776 -> 200000 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf:blocks 1100-1372 (bytes 563200-702976)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf: blocks 1375-1405 (bytes 704000-719872) hold a IMP: File Imploder stream (15872 -> 12960 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf: blocks 1408-1480 (bytes 720896-758272) hold a IMP: File Imploder stream (37376 -> 39240 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf: blocks 1485-1535 (bytes 760320-786432) hold a IMP: File Imploder stream (26112 -> 60160 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf: blocks 1540-1557 (bytes 788480-797696) hold a IMP: File Imploder stream (9216 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf: blocks 1562-1580 (bytes 799744-809472) hold a IMP: File Imploder stream (9728 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf: blocks 1584-1604 (bytes 811008-821760) hold a IMP: File Imploder stream (10752 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf: blocks 1606-1626 (bytes 822272-833024) hold a IMP: File Imploder stream (10752 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf: blocks 1628-1647 (bytes 833536-843776) hold a IMP: File Imploder stream (10240 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf: blocks 1650-1667 (bytes 844800-854016) hold a IMP: File Imploder stream (9216 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf: blocks 1672-1686 (bytes 856064-863744) hold a IMP: File Imploder stream (7680 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf: blocks 1694-1710 (bytes 867328-876032) hold a IMP: File Imploder stream (8704 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +11 znt]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf: blocks 1716-1741 (bytes 878592-891904) hold a IMP: File Imploder stream (13312 -> 41760 bytes)
- 4 never-loaded region(s), 700564 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 270336 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_dma_and_copy` (Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf boot-block load +an address to an address (17408 bytes)): Sets up DMA and interrupt control registers, writes specific values to CIAB_PRB, modifies a bit in CIAA_PRA, and copies a block of memory from one location to another using a loop. Kind: disk hardware check. Pass path: The routine initializes DMA and copies data, but the presence of CIAB_PRB manipulation and a call to an unresolved jumptable suggests a hardware-based check (likely track or sync validation). If the check passes, execution continues normall.
- `pulse_drive_control_port` (runtime segment an address): Saves registers to the stack, calls several routines within a loop that runs based on a value in a register, pulses a byte at the address in a5 between 255 and 247 using calls to another routine, and stores a constant in a global variable before returning. Kind: disk hardware/sync check. Pass path: the loop continues (extraout_D1w == 0) or the routine returns normally without triggering a crash/reset.

## Functions


### Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf:bootblock

- `execute_boot_sequence`: Performs several library calls to configure hardware, copies a block of data to a destination, and then performs a second sequence of hardware configuration and a jump to a new address.
- `set_boot_vector_and_jump`: Stores a longword value into a memory location, writes a byte to another memory location, and performs a long jump to a specific address.

### logged disk load from Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf boot-block load

- `copy_data_block_2`: Compares a value in a0 to a destination address, then enters a loop that copies 4-byte words from a0 to a1 while decrementing a counter in d0 until the counter reaches zero, otherwise it branches to another routine.
- `copy_block_and_validate_checksum`: Copies a block of data from a source to a destination using a loop that decrements a counter by four each iteration, then performs a bitwise validation check on the copied data before jumping to another routine.
- `decrement_and_jump_indirect`: Decrements a 16-bit value in d6, copies it to d3, loads a pointer into a6 from a relative offset, and performs an indirect jump using the address in a4.
- `store_byte_and_branch`: Stores a byte from d6 into the address pointed to by a2, then loads a pointer into a6 from a relative offset and branches to another location.
- `branch_on_d6_status_2`: Tests the byte in d6 and branches to one of two locations depending on whether the byte is zero or not.
- `jump_to_indirect_address`: Sets d4 to 3, loads a value into a6 from a relative offset, and jumps to the address contained in a4.
- `increment_and_branch`: Adds 6 to d6 and then branches to another location.
- `indirect_jump_via_register`: Loads a value into d4 and performs an indirect jump using the address in a4.
- `prepare_jump_parameter`: Sets d4 to 16 and performs a relative branch.
- `copy_and_validate_boot_data`: Copies a block of data from a source address to a destination address using a loop, then performs a bitwise validation check on the data before executing an indirect jump to a location specified in a table.
- `compare_and_jump_indirect_multi`: Compares a byte in d6 to 6 and branches if it is less than 6, otherwise it branches to a specific location if d6 is 6, or loads a pointer into a6 from a relative offset and performs an indirect jump using the address in a5.
- `load_pointer_and_jump`: Loads a pointer into a6 from a relative offset and performs an indirect jump using the address in a5.
- `branch_on_d6_value`: Compares d6 to 2 and branches if it is less than 2 or equal to 2, otherwise it loads a pointer into a6 from a relative offset, sets d4 to 1, and performs an indirect jump using the address in a4.
- `prepare_jump_via_a4`: Sets register d4 to 2 and performs an indirect jump using the address in a4.
- `load_d3_and_branch`: Loads the value 3 into d3 and branches to another routine.
- `test_d6_and_branch`: Loads 1 into d3, tests d6, and branches to another routine if d6 is zero, otherwise it branches to another routine.
- `branch_on_d6_status`: Tests d6 and branches to another routine if zero, otherwise it loads a pointer into a6 from a relative offset and branches to another routine.
- `branch_on_status`: Tests the value in d6 and branches to one of two locations.
- `compare_byte_and_jump_indirect`: Compares a byte in register d6 to three and, if the value is three or greater, loads a pointer from a relative offset into register a6 and performs an indirect jump via register a5.
- `compare_and_jump_indirect`: Compares a byte in d6 to 3 and branches if it is less than 3, otherwise it loads a pointer into a6 from a relative offset and performs an indirect jump using the address in a5.
- `compare_and_jump`: Compares d6 to 3 and branches if it is less than 3, otherwise it loads a pointer into a6 and performs an indirect jump using the address in a4.
- `add_and_branch_relative`: Adds the word in d6 to d3, loads a pointer into a6 from a relative offset, and branches to another location.
- `set_jump_parameters`: Tests d6 and, if zero, sets d4 to 7 and d2 to 2 before branching.
- `setup_jump_table`: Sets d4 to 8, sets d2 to 130, loads a pointer into a6 from a relative offset, and performs an indirect jump using the address in a4.
- `loop_and_push_bytes`: Adds d2 to d6, calculates an address using a6, and enters a loop that pushes bytes from two different locations onto the stacks of a6 and a2 until a counter in d3 reaches zero.
- `load_d3_and_branch_2`: Loads the value 7 into d3 and branches to another location.
- `load_d3_and_branch_alt`: Loads the value 4 into d3 and branches to another location.
- `compare_d6_and_branch`: Loads 2 into d3, compares d6 to 1, and branches to another routine if d6 is less than 1, otherwise it branches to another routine if d6 is 1, or loads 3 into d3 and branches to another routine.
- `clear_d6_and_branch`: Clears d6 and branches to another location.
- `loop_and_shift_d5`: Shifts d5 left by 1, branches if the result is 0, otherwise it performs a rotate through carry on d6, decrements d4, and loops if d4 is not 0 before performing an indirect jump using the address in a6.
- `process_memory_block`: Compares a1 and a0, and if they are not equal, moves a longword from the address in a0 to a location at offset 448 of the structure in a3, performs a longword rotate through carry, and branches to another routine.
- `copy_and_process_bytes`: Copies a block of bytes from one memory location to another, then iterates through a source buffer to perform conditional byte pushes and updates a structure at offset 384.
- `push_byte_and_jump`: Stores a byte from d0 to the location pointed to by a2 decremented by 1, writes 33248 to offset 150 of the structure in a4, writes 49152 to offset 154 of the structure in a4, and performs an indirect jump using an address loaded from a table into a2.
- `copy_bytes_to_stack_and_table`: Enters a loop that pops two bytes from a source, pushes the second byte onto the stack pointed to by a2, and when the counter in d2 reaches zero, writes the last byte to an offset of a4, then branches.
- `fetch_byte_and_branch`: Clears d1, fetches a byte from the location pointed to by a0 and decrements a0, then performs a relative branch.

### logged disk load from Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf boot-block load

- `initialize_dma_and_copy`: Sets up DMA and interrupt control registers, writes specific values to CIAB_PRB, modifies a bit in CIAA_PRA, and copies a block of memory from one location to another using a loop.
- `copy_memory_block`: Calculates the difference between two addresses and compares a pointer to a third address, then loops to copy 4-byte words from the source to the destination until a counter reaches zero.
- `copy_and_reorganize_data`: Copies a block of 4-byte words from one memory location to another using a loop, then performs a series of bitwise operations and comparisons on the data before executing an indirect jump through a table.
- `execute_indirect_jump`: Decrements d6, moves it to d3, loads a pointer into a6 from a relative offset, and performs an indirect jump using the address in a4.
- `store_and_loop`: Stores a byte from d6 into the address pointed to by a2, loops based on the value in d3, loads a pointer into a6 from a relative offset, and performs a relative branch.
- `test_and_branch`: Tests a byte in d6 and branches if it is zero or performs a relative branch otherwise.
- `load_jump_target`: Loads a constant value of 3 into a register and jumps to an address stored in a table located at a relative offset from the program counter.
- `increment_d6_and_branch`: Adds 6 to d6 and branches to another routine.
- `execute_indirect_jump_2`: Loads 10 into d4, loads a pointer into a6 from a relative offset, and performs an indirect jump using the address in a4.
- `set_d4_and_branch_2`: Sets d4 to 16 and branches to another location.
- `reorganize_and_dispatch_data`: Copies a block of data from one memory location to another using a loop, then performs a series of comparisons and conditional branches on the data to determine a jump target, eventually performing an indirect jump to a value from a table.
- `compare_and_jump_2`: Compares a byte in d6 to 6, then branches if it is less than 6 or equal to 6, otherwise it loads an address into a6 and performs an indirect jump using the address in a5.
- `load_address_and_jump`: Loads an address into a6 from a relative offset and performs an indirect jump using the address in a5.
- `branch_on_byte_value`: Compares a byte in d6 to 2, then branches if it is less than 2 or equal to 2, otherwise it loads an address into a6 and performs an indirect jump using the address in a4.
- `prepare_and_dispatch_indirect_jump`: Loads the value two into a register and performs an indirect jump using the address stored in a register.
- `load_d3_and_jump`: Loads 3 into d3 and branches to another routine.
- `test_and_branch_conditionally`: Loads the constant one into register d3, tests the value of register d6, and branches to one of two different locations depending on whether d6 is zero.
- `branch_on_byte_zero`: Tests a byte in d6 and branches to another routine if the byte is zero, otherwise it loads an address into a6 from a relative offset and performs an unconditional branch.
- `test_d6_and_branch_2`: Tests the byte in d6 and branches to another location if it is zero, otherwise it branches to another location.
- `branch_on_byte_comparison`: Compares a byte in register d6 to three; if the value is less than three, it branches to another routine, otherwise it loads a pointer from a relative offset into register a6 and performs an indirect jump using the address in register a5.
- `jump_if_d6_not_low`: Compares d6 to 3 and branches if it is less than 3, otherwise it loads a pointer into a6 and performs an indirect jump using the address in a5.
- `jump_based_on_d6`: Compares d6 to 3, and if d6 is less than 3, branches to another routine, otherwise it loads 8 into d4, loads 10 into d3, loads a pointer into a6 from a relative offset, and performs an indirect jump using the address in a4.
- `add_and_branch_2`: Adds the value in d6 to d3, loads an address into a6 from a relative offset, and performs a relative branch.
- `frag_moveq_bra`: Short code fragment that loads 7 into d4, then loads 2 into d2, then jumps to another point in the code.
- `prepare_indirect_jump`: Loads a constant value into a register, sets a pointer to a location relative to the program counter, and performs an indirect jump using the address in a register.
- `transfer_data_loop`: Adds the value in d2 to d6, sets a6 to the sum of a2 and d6, copies a byte from the location pointed to by the new a6 into the location pointed to by a2, and loops using d3 until it reaches zero before branching to another location.
- `set_d3_and_branch`: Sets d3 to 7 and branches to another routine.
- `set_d3_and_branch_alt`: Sets d3 to 4 and branches to another routine.
- `select_d3_value`: Sets d3 to 2, then compares d6 to 1; if d6 is less than 1, it returns, otherwise if d6 is 1 it returns, otherwise it sets d3 to 3 and branches.
- `clear_register_and_branch`: Clears register d6 and performs a relative branch to another routine.
- `iterates_and_jumps_via_pointer`: Shifts a longword left by one bit and, if the result is not zero, performs a bitwise rotation on a 16-bit value and decrements a counter to repeat the loop; otherwise, it jumps to the address stored in a register.
- `copy_and_modify_data`: Compares a1 and a0, and if they are not equal, it loads a longword from the address in a0 into d5, writes the word in d4 to offset 384 of a3, performs a longword rotation with a carry, and branches.
- `copy_and_dispatch_bytes`: Copies a block of bytes, then processes a second buffer by checking bytes against two constants and performing conditional stack pushes and structure updates.
- `save_byte_and_dispatch_task`: Stores a byte from a register onto the stack via a pointer in a second register, writes two 16-bit values to offsets 150 and 154 of a structure pointed to by a fourth register, and performs an indirect jump to a location retrieved from a table.
- `copy_bytes_decrementing`: Copies bytes from a source pointed to by a0 into a destination pointed to by a2, stores the first byte in d2 into a word at offset 384 of a structure in a4, and loops while d2 is not zero before branching to another location.
- `fetch_byte_and_branch_2`: Clears register d1, loads a byte from the address in a0 into d2, and performs a relative branch to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_display_and_idle_loop`: Sets up interrupt and DMA control through a sequence of custom register writes and helper calls, then waits on a status value in an endless loop.
- `bulk_memory_copy`: Copies 2560 words from a source to a destination, calls another routine, then copies 9216 words from two different sources into two different destinations, and finally branches to another routine.
- `or_combine_bitplane_rows`: Combines five source bitplane rows with a bitwise OR over two rectangular regions and writes the combined word into five destination bitplanes at fixed row offsets.
- `write_split_pointer_entries`: Writes four pairs of split high and low address words into two pointer tables, advancing each source address by a fixed step.
- `scale_palette_and_increment_counter`: If a specific global word is not all ones, it loads two pointers into a0 and a1, then checks a global word; if it is not sixteen, it increments that word and branches, otherwise it calls another routine.
- `scale_palette_twice_and_increment`: If a specific global word is not all ones, it loads two pointers into a0 and a1, calls another routine, increments a global word, and then reloads the pointers and sets d7 to thirty-one.
- `scale_palette_block_down`: When the fade counter is nonzero it decrements the counter and writes a block of colour words, each nibble multiplied by the counter and divided by sixteen.
- `program_blitter_registers`: Waits while the blitter busy bit is set, writes its control, mask and address registers from fixed values and one argument, and sets the blit size.
- `program_blitter_secondary_setup`: Waits for the blitter to be idle, writes its control, mask and address registers from a different set of fixed constants, and starts a block of a given size.
- `program_blitter_modulo_setup`: Waits for the blitter to be idle, programs its control, mask, modulo and address registers from fixed constants, and starts a block operation.
- `advance_blit_script_entry`: Counts down a delay and, when it expires, reads the next three-part entry from a command table and programs the blitter with its values.
- `advance_blit_script_entry_2`: Decrements a global word and, if it reaches zero, increments it by four, adds twenty to another global word, reads a word from a table at an offset, and if the word is not all ones, loads a long value from a pointer and decrements the second global word.
- `animate_offsets_and_blit`: Advances a tick counter, toggles a phase value every sixth tick, steps a second value at intervals, and programs blitter registers from offsets computed off the phase.
- `reprogram_blit_from_offset`: Waits for the blitter to be idle, then reprograms its address, modulo and size registers from a stored offset and fixed constants.
- `update_table_index_and_counter`: Decrements a 16-bit counter in a global variable and, when it reaches zero, reloads four words from a table into other global variables and updates a second counter.
- `cycle_four_entry_table_index`: Decrements a countdown and, when it expires, advances a four-entry index modulo four and reloads two table words from that entry.
- `update_table_index_and_counter_2`: Decrements a 16-bit counter in a global variable and, when it reaches zero, reloads five words from a table into other global variables and updates a second counter.
- `initialize_graphics_data`: Initializes multiple graphics-related data structures with various constants and computed values, then returns.
- `update_quad_index_table_2`: Decrements a global 16-bit counter and, if it reaches zero, increments a 4-entry index counter masked to 3, then updates two global variables by reading values from a table using that index.
- `load_and_update_animation_data`: Decrements a counter in a global variable and, if zero, loads a block of data from a table into several global variables including two 16-bit values and a pointer.
- `update_quad_index_table`: Decrements a global counter and, if it reaches zero, increments and masks a second global index with three, then updates two global variables by reading values from a table using the new index.
- `read_debounced_port_nibble`: Reads a CIA port several times until the input is stable, then looks up a six-bit value in a table using the low four bits of the reading and returns it with a flag bit.
- `write_cia_port_bits`: Sets a control bit and a two-bit selector in a CIA port register from its argument, then returns the inverse of the low six bits.
- `acknowledge_pending_interrupt_bits`: Reads the interrupt request register and, when bit 4 is set, updates a global variable by adding a constant to it and writes a value to a custom register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_system_sequence`: Calls a routine at a fixed address, writes values to offsets 150 and 154 of a structure in a6, tests a byte at a specific address, and performs a series of calls to other routines before an indirect jump.
- `load_address_and_branch_2`: Loads a 32-bit address into a1 from an offset relative to the program counter and branches to another location.
- `initialize_audio_system_2`: Sets up audio channel state and configures copper list pointers while managing display line pacing and interrupt handling.
- `frag_move_moveq_lea_bsr_jmp`: Short code fragment that copies an address to a fixed memory address, then loads 100 into d0, then loads 23 into d1, then loads the address of a fixed memory address into a0, then calls another routine (4 times in a row), then jumps to another point in the code.
- `configure_hardware_and_branch`: Sets several custom hardware registers, compares a byte at an offset of a6 to 42 and 41, and if both match, performs several register writes, calls two routines, and jumps to the address in a5.
- `step_raster_rows_to_seventeen`: Sets a DMA control value, stores a byte at a global address, calls two routines, and loops from a global counter up to 17, decrementing the counter on each pass.
- `count_rows_down_to_zero`: Calls two routines, decrements a global word, and loops while the value is positive, otherwise it clears the global word and returns.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `scale_thirty_two_colour_words`: Scales thirty-two colour words by a counter value, multiplying each nibble by the counter and dividing by sixteen, and writes the results to a block.
- `pulse_drive_control_port`: Saves registers to the stack, calls several routines within a loop that runs based on a value in a register, pulses a byte at the address in a5 between 255 and 247 using calls to another routine, and stores a constant in a global variable before returning.
- `decode_mfm_track_sectors_b`: Performs the same MFM track decode as the boot track reader: it sets DMA parameters, searches for the sync marker, verifies checksums, and copies sector data up to a limit.
- `step_head_target_track_b`: Steps a track counter toward a target and sets a control bit from the low bit of an argument, calling a helper on each pass.
- `check_ciaa_status`: Tests bit 4 of the CIAA_PRA register and either calls another routine or clears a word at a location in a4 and sets bit 2 of a byte at a5.
- `pulse_control_bits_high`: Sets bit 1 of a byte at the address in a5, calls another routine, clears bit 0 of that same byte, calls another routine, sets bit 0, decrements a 16-bit counter in a4, and branches to another routine.
- `pulse_control_bits_low`: Clears bit 1 of a byte at the address in a5, calls another routine, clears bit 0 of that same byte, calls another routine, sets bit 0, increments a 16-bit counter in a4, and branches to another routine.
- `pulse_control_byte_bits_b`: Writes 127 to a byte at a location in a5, calls another routine, writes 119 to the same location, and returns.
- `wait_for_raster_position_b`: Busy-waits on a byte in a loop whose exit test compares it with its own increment, so the decompiled loop does not finish under normal arithmetic.
- `check_and_call_subroutines`: Saves d0, d1, a2, and a5 to the stack, copies a0 to a5, compares the longword at a0 to a constant, and if not equal, branches to another location; otherwise, it calls two different subroutines, restores the saved registers, and returns.
- `unpack_cmap_palette_words`: Searches a chunk list for the four-letter tag CMAP, then expands its packed three-byte colour entries into twelve-bit colour words in the palette area.
- `unpack_body_chunk_rle`: Finds the BODY chunk and copies its data to the output buffer, either as raw bytes or as run-length data where a negative count repeats one byte and a positive count copies literal bytes, depending on a header flag.
- `copy_data_block`: Copies a block of data from a source table to a destination table using a loop that runs 1655 times.
- `decompress_imp_data_block`: Checks for a specific four-byte signature at the start of a data structure, then performs a decompression routine using a series of bit-shifting and byte-copying operations to expand data into a destination buffer.
- `branch_to_another_routine`: Performs a short branch to another routine.
- `branch_to_audio_update`: Performs a short branch to a location within another routine.
- `update_four_channel_audio`: Processes audio playback for four channels, updating sample pointers, applying volume slides, triggering notes, and managing playback step counters.
- `advance_sample_loop_for_channels`: Walks three sound channels and advances each sample pointer by a step, handling one-shot, forward loop and ping-pong reversal modes.
- `load_channel_pattern_fields`: Loads a channel's pattern entry from a table indexed by a row byte, copying a word pointer and two byte parameters into the channel record and clearing a flag.
- `apply_channel_slide_step`: Updates a pointer in a2, calls another routine, and if a word at offset 120 of a2 is nonzero, adds a value from a5 to a value at offset 14 of a2 and updates a word at offset 70 of a2.
- `read_next_channel_command_byte`: Clears two channel counters, then either counts down a hold delay or reads the next command byte from a stream and stores its parameter bytes, storing a hold count when the byte is negative.
- `start_channel_note_from_table`: When a channel's trigger field is set, clears its effect state and loads sample header values from a table row, setting the initial volume, period and sample pointer from that row.
- `update_channel_period_with_clamp`: Runs the effect helpers for one tick, adds the slide amount to the channel period, clamps it to a floor of 95 and a ceiling 5760, and stores the result to the period output.
- `increment_counter_or_add_offset`: Checks a byte at an offset from a pointer; if that byte is non-zero, it compares it against a limit byte in the structure.
- `update_entity_state_attributes`: Extracts a command index from a word at offset 50 of the structure in a2 and performs a series of conditional operations.
- `slide_period_toward_note_target`: Looks up a target period from a table and moves the channel period toward it by a stored step, snapping to the target when reached, and returns the current period.
- `set_pitch_from_command_table`: Computes a table index from a word at offset 82 of a2 and a byte at offset 6 of a0, then loads a word from a table into offset 10 of a2.
- `frag_move_neg_add_rts`: Short code fragment that copies the memory at an offset from a2 to d0, then negates d0, then copies d0 to the memory at an offset from a2, then adds d0 to the memory at an offset from a2, then returns.
- `copy_value_and_accumulate`: Copies one record field into another and adds the same value to a third field.
- `ramp_level_from_control_2`: Compares a byte at offset 4 of a0 to zero and branches to either offset 1684 or 1694.
- `ramp_level_from_control`: Adjusts a level counter from a control word: when the low nibble is zero it adds four times the high nibble capped at 255, otherwise it subtracts four times the control value floored at zero, and mirrors the result into a second field.
- `frag_bsr_bra`: Short code fragment that calls ramp_level_from_control, then jumps to another point in the code.
- `calculate_and_update`: Performs a lookup-based calculation using values from a global table, applies a multiplier and bit shift, negates the result if needed, subtracts it from a value in memory, and updates two byte values in the structure.
- `advance_position_by_speed`: Latches a new speed when the control field is nonzero, then moves a 32-bit position by the speed scaled by 256 and reduces a fractional field by the speed scaled by 128.
- `latch_speed_and_clear_flag`: When the control field is nonzero, clears a long field, copies the control value into a speed field, and clears the bits of a flag word selected by a mask field.
- `frag_move_beq_cmp_bne_clr_rts`: Short code fragment that copies the memory at an offset from a2 to d0, then branches if the last result was zero, then compares d0 with the memory at an offset from a0, then branches if the last result was not zero, then clears the memory at an offset from a2 (2 times in a row), then copies 0 to an indexed entry from a6, then returns.
- `frag_move_beq_clr_rts`: Short code fragment that copies the memory at an offset from a2 to d0, then branches if the last result was zero, then clears the memory at an offset from a2, then copies d0 to the memory at an offset from a2, then copies the memory at an offset from a2 to the memory at an offset from a2 (2 times in a row), then returns.
- `decrement_byte_at_offset`: Copies a byte from offset 5 of a structure to offset 3 of the same structure and then decrements the byte at offset 3.
- `scale_and_store_value`: Multiplies a value from a table by four, caps it at 255, stores it in a structure, and then stores a scaled version of that value into a destination pointed to by a6.
- `store_global_offset_if_small`: If a byte field is below a fixed limit, stores the sum of a global byte and that field into a byte of the linked record.
- `step_secondary_envelope_pitch`: Advances the secondary envelope counter: when it reaches the segment length it reloads from length minus step and adds a table value to the pitch, otherwise it increments the counter.
- `step_primary_envelope_pitch`: Advances the primary envelope counter using its own record fields, reloading the counter and adding a lookup-table value to the pitch when the segment ends, otherwise incrementing the counter.
- `copy_sequence_entry_to_buffer`: Advances a sequence counter and, on a frame boundary, fetches a 64-byte entry from a table, copies its long and word fields into the output buffer at a running offset, and records the entry index.
- `scale_position_into_buffer`: Calls another routine, copies a word from offset 12 of a2 to offset 80 of a2, and writes the high word of the product of that word and a word at offset 96 of a2 into a buffer at an offset in a6.
- `run_envelope_state_machine`: Advances the record's level envelope one step according to its state: a ramp up, a ramp down, a counted pause, or a timed hold, moving to the next state when each one finishes.
- `clear_global_flag_if_zero`: Writes zero back to a global variable only when it already holds zero, so the routine has no visible effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `search_and_replace_video_port`: Loops through a memory range starting at a specific address, checks if a word matches a constant, and if so, writes a new word to that location, then updates a global pointer and returns.
- `frag_move_rts`: Short code fragment that copies an address to a fixed memory address, then returns.

### Dyna Blaster (1991)(Hudson Soft)[cr VF][t +11 ZNT].adf:blocks 1100-1372 (bytes 563200-702976)#unpacked (raw code file, load address unknown)

- `initialize_system_and_tables`: Sets hardware registers, calls three routines, clears a block of memory using a loop, and processes a data table to populate various memory locations based on specific byte values.
- `calculate_table_offset`: Calculates two different offsets based on the input in d0 and adds them to a base address in a5, then returns.
- `clear_memory_blocks`: Calls another routine, then uses a loop to fill two memory blocks starting at offsets of a5 and a5 plus 8400 with zeros.
- `update_table_values`: Calls another routine, then uses a loop to read values from a table using an offset derived from a byte at offset 7 of a structure and performs a bitwise OR to store these values into two 32-bit words at offsets 0 and 8400 of a destination structure in a5.
- `fill_table_from_source`: Copies bytes from a source table to a destination table, incrementing a byte value for each entry until a byte of all ones is encountered.
- `initialize_hardware_and_tables`: Sets a CIA register, configures several hardware registers, and populates a table with specific values derived from a calculation involving a nibble-based bit manipulation.
- `generate_lookup_tables`: Fills a table starting at offset 192 with values calculated via division, populates a second table starting at offset 8116 with bit-rotated bytes, and populates a third table starting at offset 8772 with incrementing 16-bit values.
- `perform_bit_manipulation_and_scaling`: Performs a loop of bitwise XOR operations and shifts on a value, then returns a result that is either a bitwise mask or a scaled product of the input.
- `reset_audio_and_data`: Initializes a structure, loops to clear a table of values, sets several hardware registers including audio volumes and DMA control, and clears several bytes and words in a global area.
- `update_audio_hardware_channels`: Updates the four Amiga audio channel registers by calling a routine for each channel with specific parameters, then calculates a scaled value from a table and writes it to a global variable.
- `process_command_and_update_audio_hardware`: Extracts a command type from a structure in a6, uses it to index a table to update various fields in the structure, and performs conditional logic to update audio hardware registers and DMA settings.
- `dispatch_scaled_value_command`: Extracts a nibble from a byte at offset 2 of a structure in a6 to determine a command type, then branches to specific handlers for values 1 through 6, 7, a, or e, or performs a data transfer from offset 16 of the structure to offset 6 of a structure in a5.
- `process_scaled_command_value`: Extracts a nibble from a byte at offset 3 of a structure in a6 to determine a command type, then performs arithmetic on a 16-bit value at offset 16 of the structure based on a lookup table or specific constant bounds, and finally stores the result at offset 6 of a structure in a5.
- `update_scaled_value_and_table_2`: Searches a table for a value matching a masked 16-bit field, updates a 16-bit value in a structure, and sets a byte at offset 22 based on a comparison.
- `update_scaled_value_and_table`: Updates a 16-bit value in a structure by adding or subtracting a byte from offset 23, manages a counter at offset 24, and performs a table lookup to store a 16-bit result into offset 6 of a structure in a5.
- `update_scaled_value`: Calculates a value by scaling a byte from a table or applying a bit-shift multiplier based on flags in a structure, adds this result to a 16-bit integer at offset 16 of another structure, and updates a byte at offset 27 of the input structure using a shifted value from offset 26.
- `process_input_command_data`: Reads a command byte from a structure, branches to specific handlers based on its value, and updates various structure fields or hardware status bits.
- `modify_structure_fields`: Updates various fields in a structure, including bitwise operations on offset 3 and offset 30, and performs conditional logic involving a global variable and DMA controller registers.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
