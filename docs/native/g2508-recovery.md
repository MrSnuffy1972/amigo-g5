# Game 2508 (Deliverance v1.2 1500): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02508). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4636062 top-level bytes in 57 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 45 raw-word lines |
| Functions named and described | pass, 307 of 307 named (0 author symbols), 307 of 307 described (9 by rule, 292 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 5545 of 5545 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 36840 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7727 of 7729 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 893015 |
| code | 27376 |
| data | 3715649 |
| padding | 22 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 272 | 15 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6261 | 255 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 467 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 665 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 3 |
| D_03 (raw code file, load address unknown) | 1 | 1 | 1 |
| D_07 (raw code file, load address unknown) | 1 | 0 | 0 |
| D_11 (raw code file, load address unknown) | 1 | 0 | 0 |
| D_15 (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- Deliverance_v1.2_1500/Deliverance_v1.2_1500.hdf: hard-disk image (AmigaDOS OFS, 5024 blocks): its filesystem parses and all 45 files in it are byte-identical to the extracted tree Deliverance_v1.2_1500_hdf, which is recovered file by file; no block outside the filesystem holds data
- Deliverance_v1.2_1500/Deliverance_v1.2_1500_hdf/Deliverance/data/D_10: high-entropy data file (7.96 bits/byte), classified as packed data
- Deliverance_v1.2_1500/Deliverance_v1.2_1500_hdf/Deliverance/data/D_14: high-entropy data file (7.97 bits/byte), classified as packed data
- Deliverance_v1.2_1500/Deliverance_v1.2_1500_hdf/Deliverance/data/D_18: high-entropy data file (7.97 bits/byte), classified as packed data
- Deliverance_v1.2_1500/Deliverance_v1.2_1500_hdf/Deliverance/data/D_22: high-entropy data file (7.97 bits/byte), classified as packed data
- Deliverance_v1.2_1500/Deliverance_v1.2_1500_hdf/Deliverance/data/D_26: high-entropy data file (7.98 bits/byte), classified as packed data
- Deliverance_v1.2_1500/Deliverance_v1.2_1500_hdf/Deliverance/data/D_30: high-entropy data file (7.96 bits/byte), classified as packed data
- Deliverance_v1.2_1500/Deliverance_v1.2_1500_hdf/Deliverance/data/D_34: high-entropy data file (7.96 bits/byte), classified as packed data
- Deliverance_v1.2_1500/Deliverance_v1.2_1500_hdf/Deliverance/data/D_38: high-entropy data file (7.98 bits/byte), classified as packed data
- runtime segment an address: 4674 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 21 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 418 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 156 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 11 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 3 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 106 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- Deliverance_v1.2_1500/Deliverance_v1.2_1500_hdf/Deliverance/data/D_03: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Deliverance_v1.2_1500/Deliverance_v1.2_1500_hdf/Deliverance/data/D_07: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Deliverance_v1.2_1500/Deliverance_v1.2_1500_hdf/Deliverance/data/D_11: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Deliverance_v1.2_1500/Deliverance_v1.2_1500_hdf/Deliverance/data/D_15: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- 4 never-loaded region(s), 36840 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `program_entry`: The routine is the program entry point.
- `init_dma_and_jump`: The routine writes to a custom hardware register and DMACON to enable DMA, copies 164 bytes from a global variable to a table, then repeatedly calls another routine and writes a small code stub through a pointer until a null check stops the loop.
- `store_flag_result`: Calls another routine, then stores the returned word into a 16-bit value at offset 68 of the structure in a6.
- `set_hardware_pointer`: Calls another routine and then loads the base address of the custom hardware registers into a6.
- `wait_for_global_flag`: The routine spins until a global variable clears a specific bit, then returns.
- `resolve_pointer_table`: The routine walks a table of offsets in a pointer passed on the stack, replacing each entry with an address computed from a base pointer and the original value.
- `format_two_digit_ascii`: The routine converts a value in a register into two ASCII decimal digits and stores them into a global variable, then calls a table entry and another routine before returning a combined value from two registers.
- `format_two_digit_counter`: The routine computes a zero-based index from a pointer difference, divides it by ten and takes the remainder, converts both digits to ASCII and stores them into two consecutive bytes of a global variable.
- `convert_value_to_ascii`: Calculates a value based on the difference between two numbers and stores the result as two ASCII characters in a global variable.
- `compare_strings_until_mismatch`: The routine compares two byte strings byte by byte, advancing a counter for each matching byte and stopping early if the counter reaches the length in D0.
- `decode_and_shift_bytes`: The routine decodes a byte stream in place by reading control bytes from a source area and copying or repeating bytes into a destination buffer that starts at a given base address, moving both pointers backwards.
- `handle_keyboard_input`: Compares d0 to a global byte; if d0 equals 88, it writes 24582 to a location at offset 28144 from a pointer in a0, calls another routine, and clears a byte at a fixed address; otherwise, it clears a global byte and returns.
- `clear_four_pointers_and_patch_table`: The routine checks a byte in a register and, if it matches, writes a NOP return opcode to four long-word locations derived from a global variable.
- `empty_routine`: The routine contains no executable instructions and is effectively empty.
- `call_indirect_from_table`: The routine calls another routine through an indirect pointer obtained by adding an offset to a global variable, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `initialize_hardware_and_tables`: Loads the custom chip base address into a6, sets three 16-bit values at offsets 154, 156, and 150 of the structure in a6 to 32767, enables DMA via a bit set in a custom register, loads a value into a global variable, and calls a trap.
- `initialize_system_tables_and_stack`: Sets the stack pointer to a fixed address, populates a table of 64 function pointers starting at a global address, and clears a 96-byte block of memory.
- `clear_global_state`: Writes zero to five different global bytes at offsets 630, 631, 630, 635, and 636 and returns.
- `set_flag_and_table_byte`: Sets a global byte to 255, then stores a byte derived from its sign into a table at an offset given by the low seven bits of that variable.
- `decode_joystick_port_movement`: Reads the joystick port movement register into a global, converts its bits with a Gray-code style step and a sign-extended shift and mask sequence, and stores the resulting direction value.
- `read_joy0dat_and_accumulate`: Reads the mouse port movement register, splits it into low and high bytes, passes each through a helper, accumulates the results into a global, and returns the sum in both halves of a 64-bit value.
- `wrap_short_difference`: Subtracts one short value from another and wraps a difference outside the range around plus or minus 126 by adding or subtracting 256, then returns the wrapped value.
- `process_interrupt_and_callback`: Checks a bit in the interrupt mask; if set, it performs bitwise operations on a byte from the keyboard serial register, uses a table to determine a value, and updates a global byte; otherwise, it calls a function pointer stored in a global variable.
- `setup_stack_and_branch`: Loads the value 38746 into d0, loads the current stack pointer into a7, and branches to another location.
- `initialize_system_state`: Performs a large sequence of hardware initializations, table setups, and memory clears, including calls to many subroutines for audio, DMA, and joystick processing, and contains a loop that iterates 19 times.
- `clear_zero_longword_block`: Writes zero to a block of 185 longwords starting at a fixed low-memory address, one entry at a time.
- `configure_blitter_registers`: Writes specific values to several hardware registers at offsets 64, 100, 98, 96, 102, and 100 of the custom hardware base.
- `process_data_table_loop`: Calculates an offset into a table starting at a constant address, stores modified values into a structure in a6, and calls another routine within a loop that decrements a counter in d2.
- `process_data_table_loop_2`: Compares two global words, and if they differ, calls another routine and enters a loop that processes a table of values using a helper routine.
- `calculate_table_offsets`: Compares two global words, and if they differ, calls another routine and calculates two different offsets into a table using division and multiplication before calling a helper routine twice.
- `process_table_data`: Compares a word from a global variable with another word at a different global location, and if they are not equal, updates the second global variable, calls another routine, and then performs two separate table-based operations by calculating offsets using a division and multiplication of a value from a global variable and calling another routine twice.
- `perform_bitwise_or_operations`: Performs a series of bitwise OR operations on registers d0 and d1 using various immediate byte values.
- `update_buffer_bits`: Updates five memory locations by masking bits with a negated byte and then conditionally setting bits based on the bit pattern of a byte read from a calculated address, repeating this process in a loop until a value in a table is negative.
- `update_entity_bitmask_state`: Calculates a delta based on a global value and a scaling factor, then enters a loop to iterate through a list of structures.
- `execute_blt_block_copy`: Configures the Amiga Blitter chipset registers to perform a series of area fills or transfers, then enters a loop that calls another routine to process multiple blocks of data based on a counter.
- `execute_blitter_setup`: Calls another routine and then calls a second routine with specific values in registers d0 through d4.
- `perform_blitter_transfers`: Calls another routine with specific register values, then calculates an offset into a table using a value from a global variable, and calls that same routine a second time with different register parameters.
- `copy_memory_block`: Copies a large block of data from the memory location pointed to by a0 to the memory location pointed to by a1 using a series of longword move operations.
- `copy_and_patch_data`: Copies a block of data from a source to a destination, then calculates an offset to write two specific 16-bit values into a memory location.
- `setup_structure_data`: Sets two global bytes based on the sign flag, calls another routine, and populates a structure in a0 with several constants and pointers before branching to another routine.
- `update_global_counters`: Loads a pointer from a global variable, performs arithmetic on a global variable, and updates two 16-bit values at offsets 43a and 43c of a global variable by fetching values from a table based on calculated indices.
- `dispatch_structure_parameter`: Stores 64 at offset 12 of a structure in a6, calls another routine, and then branches to a location determined by a value in d0 or returns after storing two longwords.
- `branch_to_offset_470`: Performs a short branch to an instruction located 470 bytes ahead in the code segment.
- `branch_to_offset_5924`: Performs a short relative branch to a location 5924 bytes ahead.
- `branch_to_offset_1d0a`: Branches to an offset of 1d0a.
- `jump_to_shared_tail`: This routine consists solely of an unconditional branch to a point inside another routine, so it does nothing itself and transfers control there.
- `branch_to_offset_16`: Performs a short relative branch to a specific instruction offset.
- `branch_to_structure_update_3`: Performs a short branch into another routine that executes dispatch_structure_update.
- `branch_to_offset_1642`: Branches to an offset of 1642.
- `branch_to_offset_11`: Performs a short branch to an instruction located at an offset of 5698 bytes from the current program counter.
- `branch_to_offset_12`: Performs a short branch to an instruction located at an offset within another routine.
- `branch_to_tile_update`: Performs a short branch to a location inside another routine.
- `branch_to_middle_of_routine`: Performs a short branch into the middle of another routine.
- `branch_to_offset_5`: Performs a short branch to a specific offset.
- `branch_to_tile_update_2`: Performs a short branch into another routine.
- `branch_to_offset_2`: Performs a short relative branch to a specific instruction located elsewhere in the code.
- `branch_to_offset_1f00`: Branches to an offset of 7936.
- `branch_and_load_parameters`: Branches to another routine, loads two addresses into a0 and a1, and calls two subroutines.
- `set_and_branch`: Writes a 16-bit value to offset 18 of a structure in a6, calls another routine, and then performs an indirect jump based on a value from a global variable.
- `branch_to_offset_5212`: Performs a short branch to a location 5212 bytes ahead in the code.
- `branch_to_offset_14`: Performs a short branch to a specific location in the code.
- `branch_to_offset_1dbc`: Branches to an offset of 1dbc.
- `branch_to_offset_13`: Branches to an offset of 5642.
- `branch_to_offset_5634`: Performs a short branch to an offset of 5634 within the code segment.
- `branch_to_offset_5634_2`: Performs a short branch to an offset of 5634 within another routine.
- `branch_to_middle_of_routine_3`: Performs a short branch to an instruction located inside another routine.
- `branch_to_offset_6096`: Performs a short branch to an instruction located 6096 bytes ahead in the code.
- `branch_to_sequence_handler`: Performs a short branch into the middle of execute_sequence_and_update_structure.
- `branch_to_offset_1f00_2`: Performs a relative branch to an offset of 7936.
- `branch_to_offset_6`: Performs a short branch to a specific instruction offset.
- `branch_to_update_structure_and_pointer_2`: Performs a short branch to update_structure_and_pointer_2.
- `branch_to_tile_update_logic`: Performs a short branch to a location within the update_tile_buffer_and_structure_2 routine.
- `branch_to_middle_of_routine_2`: Performs a short relative branch into the middle of another routine.
- `branch_to_offset_1f00_3`: Executes a short branch to an offset of 7936.
- `search_tile_buffer_and_update_offset_2`: Checks if a 16-bit value at offset 74 of the structure in a6 is between 10 and 60; if so, it performs a search through a tile-based buffer using calculated offsets and bitmasking, then updates a 32-bit value at offset 8 of the structure in a6 based on the search result.
- `execute_jump_table`: Loads a 16-bit word from offset 628 into d0 and performs an indirect jump using d0 as an offset from the current program counter.
- `branch_to_shared_handler`: The routine consists of a single unconditional long branch that transfers control into the middle of another routine, so it does nothing itself.
- `branch_to_relative_offset`: Performs a short relative branch to a different location in the code.
- `branch_to_tile_search_routine`: Performs a short relative branch into the middle of search_tile_buffer_and_update_offset_2.
- `search_tile_buffer_and_update_offset`: Searches through a tile-based data structure for a non-zero value using a bitmask from a structure in a6, then updates a 32-bit value at offset 8 of that structure based on the search result.
- `jump_via_offset`: Loads a 16-bit value from a global variable into d0 and performs an indirect jump using d0 as an offset.
- `branch_to_offset_8`: Performs a short branch to an instruction located at a fixed offset within another routine.
- `branch_to_offset_9`: Performs a short branch to a relative offset within another routine.
- `branch_to_target_2`: Branches to a specific instruction offset.
- `load_addresses_and_branch_2`: Branches to an offset of 17d4, loads two longwords into a global variable, loads two addresses into a0 and a1, and branches to an offset of 45ea.
- `initialize_and_branch_2`: Calls another routine, otherwise it returns; if not, it loads two addresses into a0 and a1 and branches to an offset of 45ea.
- `branch_and_load_constants`: Calls another routine and returns if the result is not negative; otherwise it stores two long constants into global variables, loads two addresses into a0 and a1, and branches to another routine.
- `branch_and_load_addresses`: Calls another routine and returns if the result is not negative; otherwise it loads two addresses into a0 and a1 and branches to another routine.
- `call_and_return_conditionally`: Calls another routine and returns if the result is not negative; otherwise it returns.
- `load_addresses`: Loads a relative address into a0 and a constant address into a1, then branches to another routine.
- `store_long_value`: Stores the constant value a fixed address into a global variable at offset 50 of the structure in a6.
- `execute_jump_table_entry`: Loads a 16-bit word from a global variable into a data register and performs an indirect jump to an address calculated by adding that value to the current program counter.
- `initialize_and_branch`: Clears a 16-bit word at offset 22288, stores the values 256 and negative 256 at offsets 20322 and 20326, loads a relative address into a0 and a pointer into a1, calls another routine, and then branches to another location.
- `execute_dispatch_sequence`: Calls another routine and returns if the result is negative; otherwise, it writes two longwords to specific offsets, loads two addresses into a0 and a1, and calls another routine.
- `update_status_and_dispatch_jump`: Writes the constant sixty-four to a 16-bit word at offset eighteen of the structure in a6, calls another routine, tests bit seven of d6, and if set, performs a jump to an address calculated by adding a value from a 16-bit word at offset thirty-nine to the program counter; otherwise, it branches to another location.
- `branch_to_offset`: Performs a short branch to a specific instruction located at a fixed offset within another routine.
- `branch_to_target`: Branches to a specific offset.
- `branch_to_distant_routine`: Performs a short relative branch to another routine.
- `branch_to_shared_tail`: This routine consists of a single unconditional word-displacement branch that transfers control to a fixed location inside another routine, with no other instructions, no loop, and no subroutine call.
- `jump_to_init_stub_2`: The routine consists of a single unconditional word-displacement branch that transfers control to a point inside another routine, so it does no work itself.
- `branch_to_offset_10`: Branches to a specific offset in another routine.
- `branch_to_offset_6350`: Performs a short branch to a location 6350 bytes ahead in the code.
- `branch_to_offset_15`: Performs a short branch to a specific instruction location.
- `branch_to_offset_3`: Performs a short relative branch to a location within the same segment.
- `branch_to_subroutine_entry`: Performs a short branch to an offset within another routine.
- `branch_to_offset_4`: Performs a short relative branch to a location within another routine.
- `branch_to_offset_7604`: Performs a short conditional branch to a location within another routine.
- `branch_to_offset_1f00_4`: Jumps to an offset of 7936 via a relative branch.
- `setup_and_branch_routine`: Clears a 16-bit word at offset 1266 of a global variable, writes 256 to offset 1266 and -256 to another global variable, sets two longwords at offsets 1266 and 1270, loads a pointer into a0 and a pointer to a table into a1, calls another routine, and branches to another routine.
- `call_and_setup_pointers`: Calls a subroutine, then if the result is not negative, writes 256 to a global variable at offset 1266, writes -256 to a global variable at offset 1270, loads a constant address into a0, loads a global address into a1, and calls another subroutine.
- `update_and_jump_via_table`: Writes 64 to offset 18 of a structure in a6, calls a subroutine, tests bit 7 of d6, and if bit 7 is set, loads a value from a global variable into d0 and branches to a location determined by an offset in d0.
- `branch_to_subroutine_2`: Branches to a location within another routine.
- `branch_to_update_counters`: Performs a short branch to another routine that updates global counters and two 16-bit values at offsets 43a and 43c of a structure.
- `branch_to_data_initialization`: Branches to another routine that calls a subroutine and, if the result is not negative, stores 2560 and 1280 into two global variables and then branches to another routine.
- `branch_to_structure_update`: Performs a short branch into a middle section of another routine.
- `branch_to_structure_update_2`: Performs a short branch into the middle of the process_structure_update routine.
- `branch_to_initialization_sequence`: Branches to a location within another routine that initializes data and sets two global variables to 2560 and 1280.
- `branch_to_data_initialization_2`: Branches to another routine that calls a subroutine and, if the result is not negative, writes 2560 and 1280 to two global variables and performs a branch to another routine.
- `branch_to_update_global_counters`: Performs a short branch to another routine that updates global counters and two 16-bit values at offsets 1082 and 1084 of a structure.
- `jump_to_init_stub`: This routine consists of a single unconditional word-displacement branch, bra.w, to a fixed target elsewhere in the segment; it performs no computation, loop, or subroutine call of its own.
- `branch_to_initialization`: Branches to another routine to initialize data, then writes 2560 and 1280 to global variables at offsets 1266 and 1270.
- `branch_to_offset_7`: Performs a short branch to a specific location in another routine.
- `branch_to_initialization_2`: Branches to another routine and, if the result is not negative, writes 2560 and 1280 to global variables at offsets 1266 and 1270.
- `branch_to_distant_offset`: Performs a short relative branch to a location in another part of the executable.
- `process_structure_update`: Increments a 16-bit word at offset 554 of a global variable, calls another routine, loads two pointers into a0 and a1, and branches to another routine.
- `increment_and_dispatch`: Calls another routine, otherwise it returns; if the result is not negative, it increments a 16-bit word at offset 1266 of a global variable, compares that value to 4, branches to another routine if not equal, calls another routine, loads a pointer into a0 and a pointer to a table into a1, and branches to another routine.
- `call_and_initialize_data`: Calls another routine, otherwise it returns; if the result is not negative, it writes 256 to offset 1266 and -256 to another global variable, writes 2560 and 1280 to global variables at offsets 1266 and 1270, loads a pointer into a0 and a pointer to a table into a1, and branches to another routine.
- `branch_and_initialize_data`: Calls a subroutine, then if the result is not negative, writes the values 2560 and 1280 to global variables at offsets 1266 and 1270, loads a constant address into a0, loads a global address into a1, and branches to another routine.
- `check_and_return`: Calls a subroutine, and if the result is not negative, returns; otherwise, it branches to another routine.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_structure_values_2`: Adds 21 to a 16-bit value at offset 82 of a structure in a6, modifies it based on a byte at offset 29 of the structure, subtracts the result from a global variable, and performs several conditional arithmetic operations on values at offsets 56, 58, 16, and 86 of the structure in a6 before returning or branching.
- `sync_structure_and_tile_buffer`: Calls four subroutines, checks bit 7 of d6 and bit 4 of a global variable, and if certain conditions are met, clears a 16-bit value at offset 82 and a 32-bit value at offset 86 of the structure in a6 before calling another routine via a jump table.
- `update_tile_buffer_and_structure`: Calculates a memory offset based on values in a structure and a table to check specific bits in a tile buffer, then updates 32-bit and 16-bit counters at offsets 8 and 12 of the structure in a6.
- `jump_via_table`: Loads a 16-bit value from a global variable into d0 and performs an indirect jump using that value as an offset.
- `store_constant_to_global`: Stores the constant value 1280 into a global variable and performs a short branch.
- `set_global_value`: Stores the constant value 1280 into a global variable and returns.
- `decrement_and_test_bits`: Decrements a byte at a global variable and branches if it is positive; otherwise it performs a series of checks on bits of a global variable and branches to other routines or returns.
- `dispatch_structure_update`: Clears a 32-bit value and sets a 16-bit value at specific offsets in a structure, then loads two addresses into registers and branches to another routine; it may also perform a conditional check on a global variable and a byte to execute a second similar update with different parameters.
- `execute_conditional_initialization`: Calls three different routines, checks bit 4 of a byte at offset 39 of a global variable, and if bit 4 is not set, it clears a 16-bit value at offset 130 of a structure in a6, sets four registers to specific values, calls another routine, writes a 32-bit value to offset 56 of a structure in a6, and then calls a final routine.
- `reset_structure_state`: Calls another routine, checks a 16-bit value at offset 18 of the structure in a6, and if it is negative or if a specific bit is set in d6, clears 16-bit values at offsets 82 and 88 of the structure, then loads a0 and a1 with new values and calls another routine.
- `execute_sequence_and_update_structure`: Sets four registers, calls another routine, clears a 32-bit value at offset 134 of the structure in a6, sets a 16-bit value at offset 130 of the structure in a6, loads two addresses into a0 and a1 and branches to another routine; otherwise it calls another routine, tests a bit in a global variable, tests a long value in a global variable, and performs a similar update to the structure in a6 with different addresses.
- `execute_conditional_sequence`: Calls another routine, then if the result is negative, branches to an earlier instruction; otherwise, it tests bit 4 of a global variable and branches to another routine if the bit is not set, otherwise it returns.
- `update_tile_buffer_and_structure_2`: Calls another routine, checks a signed 16-bit value at offset 18 of the structure in a6, and if it is non-negative, clears 16-bit counters at offsets 82 and 88 of the structure, then performs a series of data manipulations involving a table and a global variable to update a 32-bit value at offset 8 of the structure in a6.
- `update_structure_data`: Calls two routines, checks a byte at offset 77 of a structure in a6, and if it is zero, stores 255 at that offset and calls two more routines; it then uses a value from offset 74 of a6 to index a table, updates a structure in a3 with values from a global table and a6, and if the index is 8, sets a pointer at offset 50 of a6 and two 16-bit values at offsets 82 and 84 of a3 before branching.
- `setup_structure_parameters_40`: Calls another routine, and if the result is not negative, sets four 16-bit registers, calls another routine, stores a 16-bit value at a global address, sets a byte at offset 77 of a structure in a6, and calls another routine with a structure in a0 and a pointer in a1.
- `process_tile_index_40`: Calls two routines, checks a byte at offset 77 of a structure in a6, and if it is not greater than zero, sets it to 255, calls two other routines, calculates an index from a 16-bit value at offset 74 of a structure in a6, retrieves data from a table, updates a structure in a3 with values from a6 and the table, and if a specific index is 40, updates a6 and a3 with new values and performs a linked list insertion.
- `setup_structure_parameters`: Calls another routine, and if the result is not negative, sets four 16-bit registers, calls another routine, sets a byte at offset 77 of a structure in a6, and calls another routine with a structure in a0 and a pointer in a1.
- `update_tile_buffer_data`: Calls two routines, checks a byte at offset 77 of a structure in a6, and if it is not greater than zero, sets it to 255, calls two other routines, calculates an index from a 16-bit value at offset 74 of a structure in a6, retrieves data from a table, updates a structure in a3 with values from a6 and the table, and if a specific index is 8, updates a6 and a3 with new values and performs a linked list insertion.
- `initialize_structure_state_2`: Calls another routine and returns if the result is negative; otherwise, it sets four registers to specific constants, calls another routine, writes a byte to offset 77 of the structure in a6, loads the address of itself and a global address into a0 and a1, calls another routine, and branches to another routine.
- `update_structure_with_table_data`: Calls two subroutines, checks a byte at offset 77 of a structure in a6, and if it is zero, sets it to 255 and calls another routine; otherwise, it uses a value from a structure in a6 to index into a table, updates a structure pointed to by a3 with values from a6 and the table, and if a specific index matches 40, updates a structure in a6 and branches to another routine.
- `initialize_structure_state`: Calls a subroutine, and if the result is not negative, sets d0, d1, d2, and d3 to specific values, calls another routine, sets a byte at offset 77 of a structure in a6 to 2, loads the address of the current routine into a0 and a specific address into a1, calls another routine, and branches to another routine.
- `update_structure_and_link_list`: Performs several subroutine calls, checks bit 6 of d6, and if set, clears a 16-bit value at offset 82 of a structure in a6.
- `reconfigure_tile_buffer_structure`: Checks a 16-bit value at offset 18 of the structure in a6; if it is negative, it updates the structure with a function pointer and data from a table, otherwise it performs a complex search through a tile buffer to update bitmasks and counters based on signed values at offsets 8, 12, and 18.
- `update_structure_from_table_2`: Calls three routines, checks bit 6 of d6, and if set, clears a 16-bit value at offset 130 of a structure in a6.
- `initialize_tile_structure_and_buffer`: Calls several routines to determine a data source based on a signed 16-bit value at offset 18 of a structure in a6, then updates the structure with a new function pointer, an integer, a 32-bit value, and several 16-bit and 8-bit values.
- `update_structure_and_pointer_2`: Calls two routines, modifies offsets 8 and 12 of a structure pointed to by a global variable by adding specific constants, and if a bit is not set in a word at offset 36, updates offset 54 of that structure using a value from a table indexed by a value at offset 74 of the structure in a6.
- `update_structure_and_pointer`: Calls three different routines, modifies values at offsets 8 and 12 of a structure pointed to by a global variable, and updates offset 54 of that structure using a value from a table and an offset from a structure in a6.
- `branch_on_byte_test`: Loads a pointer from a global variable into a3, checks if the byte at offset 33 of that pointer is zero, and otherwise calls another routine.
- `test_and_clear_global`: Loads a pointer from a global variable into a3, checks if the byte at offset 33 is zero, and otherwise calls another routine; it also decrements a byte at a global address and clears a global word if the result is not greater than zero.
- `decrement_global_counter`: Calls another routine, decrements a 16-bit global variable, and if the value is zero or was one, clears that global variable before returning.
- `check_bit_and_branch`: Loads a pointer from offset 42 of a structure in a6 into a0, checks if bit 4 of the byte at offset 34 of that structure is set, and branches to one of two locations based on the result.
- `test_bit_four_of_structure_byte`: Loads a pointer from a global variable into a0, reads a byte from offset 34 of that structure, and branches to another routine if bit 4 of that byte is not set; otherwise, it branches to a different routine.
- `initialize_new_node`: Calls another routine, updates a global variable, and populates a structure pointed to by a0 with specific 8-bit and 16-bit values at various offsets.
- `initialize_structure_fields_2`: Calls another routine, updates a global pointer, increments a byte at offset 677 of a global variable, and sets several fields in a structure in a0 including a 16-bit value at offset 40, a 16-bit value at offset 38, and bytes at offsets 34 and 35.
- `update_structure_and_fetch_pointer_2`: Checks a flag at offset 33 of the structure in a6 and calls another routine if set; otherwise, it verifies a 16-bit value at offset 40 and a byte at offset 29, calls another routine, and if the result is not set, updates several offsets in the structure and fetches a pointer from a table using an index derived from a value at offset 86.
- `update_structure_and_fetch_pointer`: Checks a flag at offset 33 of the structure in a6 and calls another routine if set; if not, it verifies a count and a bit flag, then performs a calculation involving a range check and table lookup to update several fields in the structure and store a new pointer at offset 54.
- `test_and_call_subroutine`: Sets the status flags based on a byte in a global variable, loads a pointer into a0, saves four words from a memory location into registers, and calls another routine.
- `process_and_link_data`: Compares a global variable with a value at offset 58 of a structure in a1, and if they are equal, calls three routines, decrements a global word, modifies a value at offset 40 of a structure pointed to by a global variable, and updates offset 72 of a structure in a2 with calculated values.
- `populate_structure_data`: Calls another routine, stores a pointer from a2 into offset 62 of a structure in a0, stores the constant an address in decimal into offset 50 of a structure in a0, stores the constant 1 into offset 30 of a structure in a0, loads four longwords from offset 72 of a structure in a2 into d0 and d1, loads two word addresses into a1 and a2, and branches to another routine.
- `update_linked_list`: Calls several routines to manipulate a structure in a6, updates a pointer at offset 76 of a structure in a2, and modifies a global variable.
- `set_status_flag`: Loads a 32-bit value from an offset of a6 into a2, sets the byte at offset 56 of the structure in a2 based on the signed overflow flag, and branches to another routine.
- `reset_audio_hardware`: Clears the DMA controller, clears four audio volume registers, and sets a fifth register to 15.
- `check_status_flag`: Sets the byte at a global variable address using the signed overflow flag and returns if the resulting byte is not zero.
- `configure_dma_tables`: Sets a hardware register, performs a loop to write values from three different tables into three destination structures, then performs a second loop to copy 16-bit values from a table into two destination structures, calls another routine, and clears several memory locations and hardware registers.
- `reset_dma_parameters`: Clears a 16-bit global variable, calls another routine, and then sets a hardware register with a 16-bit value, a 32-bit pointer, and a 16-bit zero value.
- `update_dma_pointer`: Sets a byte based on the signed overflow flag, calls another routine, and then sets a hardware register with a 16-bit value, a 32-bit pointer from a global variable, and a 16-bit zero value.
- `initialize_tables`: Clears a 16-bit global variable, calls another routine, and then uses two loops to copy data from one table to another and populate a third table with calculated 32-bit values.
- `copy_table_data`: Tests a byte at offset 1379 of a global variable and branches if it is zero; otherwise, it loads two addresses into a0 and a1, then loops 32 times to copy a word from the address in a1 to offset 2 of the address in a0 and increments a0 by 4 before branching to another routine.
- `clear_and_setup_dma`: Saves a byte, clears a 16-bit global variable, runs a loop to clear 16-bit values at offset 2 of a 32-entry table, runs a loop to write 32-bit values into a 4-entry table, calls another routine, and sets a 16-bit global variable and hardware registers.
- `setup_dma_with_table`: Runs a loop to clear 16-bit values at offset 2 of a 32-entry table, runs a loop to write 32-bit values into a 5-entry table, sets a 16-bit global variable, calls another routine, and configures hardware registers.
- `setup_dma_with_five_entries`: Runs a loop to clear 16-bit values at offset 2 of a 32-entry table, runs a loop to write 32-bit values into a 5-entry table, sets a 16-bit global variable to a different value, calls another routine, and configures hardware registers.
- `update_data_tables`: Loops 199 times to extract nibbles from two 16-bit values in a list of structures, clears the source values, and writes the nibbles into two separate tables.
- `process_nibble_buffer`: Loops 32 times to extract nibbles from a list of 16-bit values and writes them into a table, while also writing a constant value of 7 to another table.
- `decrement_and_update_attribute_tables`: Decrements a global counter and, if non-zero, performs two loops to update two separate tables of values.
- `process_nibble_buffer_2`: Decrements a 16-bit global variable and, if non-zero, enters a loop that performs subtractions between bytes from two tables and updates a third table, adding a constant to the table values and a 16-bit value from a structure when a borrow occurs.
- `load_buffer_pointers`: Loads two memory addresses into registers a0 and a1 and performs a short branch.
- `load_pointers_and_branch`: Loads a pointer into a0 from a specific memory location, loads a second pointer into a1 from another memory location, and performs a short branch to another routine.
- `load_address_pointers`: Loads two long values into registers a0 and a1, then branches to another routine.
- `clear_table_entries`: Loops 32 times to clear a 16-bit word at offset 2 of two different memory locations, incrementing each pointer by 4 in every iteration.
- `setup_nibble_extraction`: Loads a pointer from a global variable, increments it by 4, loads another pointer from a global variable, and branches to another routine.
- `write_constant_values`: Writes four specific 16-bit values to four consecutive global memory locations.
- `reset_audio_and_check_status`: Calls a routine to reset audio hardware, writes four values to specific memory locations, calls another routine, and returns if a status flag is non-zero.
- `fill_memory_with_zeros`: Loops 5119 times to write a 16-bit zero to five consecutive words starting at address 2048, incrementing the pointer by 10 in each iteration.
- `calculate_and_populate_table`: Writes four 32-bit values to specific memory locations, copies two 16-bit words from a pointer found at a global address to another location, performs several arithmetic operations to calculate new values, and then loops to fill a table starting at address an address with incrementing values.
- `update_entity_attribute_blocks_2`: Calculates an offset using a multiplication of two global values, updates several global pointers and counters, configures hardware registers, and runs nested loops that call another routine to process data across three tables.
- `update_entity_coordinate_and_sprite_data`: Calculates a new coordinate value based on a global variable and updates multiple 16-bit fields in two entity structures using a table lookup, then calls another routine.
- `update_entity_attribute_blocks`: Reads a block of data from a table based on an index and copies it into a structure at specific offsets, then calls another routine to process the updated data for multiple entities.
- `write_constants`: Writes two 32-bit constants to specific memory locations and returns.
- `initialize_linked_list`: Sets a global pointer to a memory address, enters a loop that runs 60 times to update a pointer at an offset of 4 within a structure by adding 114 to the current address, then updates a global pointer and clears a 16-bit value at a specific location.
- `traverse_and_call_pointers`: Follows a chain of pointers starting from a global address, and if a pointer at offset 50 of the current structure is not zero, calls the routine at that address.
- `load_address`: Loads the address stored at a specific location into register a0.
- `insert_into_list`: If a 16-bit value at a specific location is less than 60, it performs a linked list insertion by updating pointers between a structure in a0 and a structure at a global pointer, then clears 53 words starting at an offset of 8 from the new structure and sets a byte at offset 29 to 255.
- `load_address_to_a0`: Loads the address of a word located at offset 1856 into register a0.
- `insert_node_into_linked_list`: Checks if a global counter is less than sixty; if so, it performs a circular linked list insertion by updating pointers between a new structure and existing nodes, increments the counter, clears fifty-three words in the new structure starting at offset eight, and sets a byte at offset twenty-nine to twenty-nine.
- `remove_from_list`: Copies two 32-bit values from a structure pointed to by a0 into registers, updates the structure's next pointer using a value from a global pointer, updates the global pointer to the current structure, decrements a 16-bit value at a specific location, and returns.
- `update_timer_and_value`: If a 16-bit value at offset 70 of a structure in a6 is non-zero and its decremented value is zero or less, it updates that value from an offset of 68, adds a value from offset 74 plus 38 to a 32-bit value at offset 54, and decrements a 16-bit value at offset 70.
- `populate_structure_from_data`: Calls another routine, stores a byte at offset 28 of the structure in a0, and populates several fields in the structure in a0 using bit-shifted values from a2 and values from a pointer in a3.
- `manage_list_and_counters`: Calls another routine and, if the zero flag is not set, performs a linked list removal and updates a global variable; otherwise, it decrements two counters in a structure in a6 and conditionally updates pointers and values.
- `process_structure_data`: Decrements a 16-bit value at offset 112 of a structure in a6; if the result is not zero and the original value was greater than 2, it returns; otherwise, it calculates an offset based on a value at offset 72, updates several fields in the structure, and returns a 32-bit value from a structure at a calculated offset.
- `update_counter_and_pointer`: Decrements a 16-bit value at offset 70 of the structure in a6 and, if it reaches zero or less, updates offset 72 and 74 with a new value derived from a table, then updates offset 54 and offsets 16 and 18 using data from that table.
- `decrement_counter_and_table_data`: Decrements a 16-bit value at offset 70 of the structure in a6 and, if it reaches zero or less, updates offset 72 and 74 with a value from a table, then subtracts values from offsets 16 and 18 using data from that table.
- `tiny_moveq_moveq_rts`: Small routine of 3 instructions that loads 0 into d0, then loads 0 into d1, then returns.
- `store_table_data`: Stores d0 and d1 into offsets 8 and 12 of the structure in a0, stores a1 into offset 58, stores a pointer from a table into offset 66, stores a value from that table into offset 54, and clears offsets 72 and 33 of the structure in a0.
- `update_structure_from_table`: Stores a0 and a1 into offsets 50 and 66 of the structure in a6, then updates offset 54 using a value from a table at offset 12, updates offset 70 with a value from that table at offset 10, and clears offsets 72 and 33 of the structure in a6.
- `clear_structure_fields`: Stores a0 and a1 into offsets 50 and 66 of the structure in a6, updates offset 54 using a value from a table at offset 12, and clears offsets 70, 72, and 33 of the structure in a6.
- `update_offset_from_table`: Calculates a new value for offset 8 or 12 of a structure in a6 by subtracting values from a table pointed to by a2 from existing values in the structure, then adds the result shifted left by 4.
- `calculate_blt_parameters`: Performs complex arithmetic to update a structure in a6 and writes several values to a memory location pointed to by a3, then branches to a calculated address based on a register value.
- `update_structure_offsets`: Writes several values to a structure in a6 using offsets, calls another routine, and then updates several offsets in the same structure before branching to another routine.
- `branch_to_complex_data_processing`: Performs a short unconditional branch to another routine.
- `branch_to_complex_data_processing_2`: Performs a short branch to a location within the process_complex_data_blocks routine.
- `branch_to_data_processor`: Performs a short branch to another routine that executes multiple calculations using registers d1 through d7, updates various offsets within a structure pointed to by a6, and contains several loops.
- `process_complex_data_blocks`: Performs multiple calculations involving registers d1 through d7, updates various offsets in a structure in a6, and contains several loops that call a routine at a long address.
- `process_obfuscated_table`: Calls a routine, performs an XOR operation on a global pointer, and enters a loop that processes a table of data to update offsets 160, 162, 52, 54, 80, and 88 of the structure in a6.
- `calculate_indexed_value`: Calculates an index using values from offsets 8 and 12 of the structure in a6, uses this index to fetch a value from a table, and returns the result of a bitwise AND between that value and a 16-bit value from offset 46 of the structure in a6.
- `search_and_update_tile_buffer`: Calculates a position within a tile-based data structure using values from a structure in a6 and a table, then performs a search through a range of words to find a non-zero value.
- `update_collision_and_check_bitmaps`: Updates 32-bit counters and 16-bit counters within a structure in a6 by adding signed values from offsets 16 and 18, then performs a series of nested loops to check bitmasks within a bitmap table for specific bit patterns.
- `resolve_entity_collision`: Iterates through a linked list of structures to decrement a timer at offset 9 and checks for spatial overlap between two entities using coordinates and dimensions stored in a sub-structure.
- `validate_and_execute_callback`: Iterates through a linked list by following a pointer at a global address, checking if four input values fall within a calculated coordinate range derived from a structure, and if so, calls a function pointer in a0.
- `conditional_call_and_return`: Calls another routine and returns if the negative flag is set; otherwise it returns.
- `update_structure_with_table`: Clears two bytes in a structure in a6, performs arithmetic on values from a structure pointed to by a0, and updates several offsets in a6 before branching to another routine.
- `initialize_linked_list_2`: Sets a global pointer to a memory address, then enters a loop that repeats 15 times to write a pointer to the next element into a structure at offset 4 of the current element, finally clearing a 16-bit value at a global address and updating a global pointer.
- `insert_into_list_2`: If a 16-bit global value is less than 15, it updates a global pointer to point to the next element in a list, inserts the current list head into a new node, links the new node to the previous head, and increments the global value.
- `traverse_and_call_list`: Follows a chain of pointers starting from a global address and calls a subroutine via a pointer found at offset 8 of the current structure if that pointer is not zero, repeating this process as long as the next pointer in the chain is not zero.
- `initialize_multiple_lists`: Initializes three separate lists by setting up circular linked structures with specific counts and offsets, then clears a 16-bit global value and updates a global pointer.
- `compare_and_initialize_lists`: Compares four registers against the constant 1024 and 1026, calls another routine, and stores the values of d0 through d3 into a structure in a2.
- `update_list_nodes`: Performs four separate operations where it calculates an offset into a table using a value from a structure in a2, retrieves a node from that table, and replaces the node's contents with a new structure pointed to by a2.
- `swap_and_update_list`: Updates a global pointer using a value from a structure, performs a pointer swap between a structure and a local address, and calls another routine after storing four words into a structure in a2.
- `link_structure_elements`: Copies four pairs of pointers from a structure in a2 into a destination structure, where each pair consists of a value from a specific offset and a pointer to the next element in a chain.
- `link_nodes_via_global`: Calls another routine, copies two pointers from a structure in a2 into registers, links them, and updates a global pointer with the address from a2.
- `iterate_and_call_nodes`: Sets up a pointer to a local address, iterates through a list, links nodes, calls another routine, and conditionally calls a function pointer stored at offset 60 of a structure in a2.
- `process_conditional_callbacks`: Iterates through a list of structures, checking if a set of four 16-bit values and a byte at offset 14 fall within specific ranges; if the conditions are met and a byte at offset 14 is zero, it sets that byte to 255 and calls a subroutine via a pointer at offset 14.
- `check_bounds_with_offset`: Calculates two values by right-shifting two 32-bit values from a structure in a6 and subtracting two global 16-bit values, then compares these against thresholds and checks if additional values from another structure are within a specific range, returning zero if all conditions are met or a negative value otherwise.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `check_range_conditions`: Compares four different combinations of a global 16-bit value added to a constant and values from a structure in a2 against specific thresholds, returning zero if all four comparisons pass or a negative value otherwise.
- `update_entity_bounds_and_trigger_events`: Performs a loop to update a global pointer and checks if a value at an offset within that structure falls within specific coordinate ranges, clearing a byte at offset 56 if it does.
- `toggle_byte_and_call`: Inverts a byte at a global address, calls another routine, and then inverts the byte again before returning.
- `update_entity_state`: Validates an input byte against several threshold values in a structure and, if valid, updates the structure with the new value and calculates a pointer to a data entry from a table.
- `initialize_audio_hardware`: Clears a byte in a table using a loop, populates several hardware registers and a structure in a5 with values from a table, and calls another routine twice.
- `parse_and_dispatch_table_entries`: Reads a series of offsets and values from a data table to populate a structure in a6, then calls another routine four times, passing a different identifier and a pointer to each of the four calculated values.
- `initialize_structure_fields`: Copies a longword from a location offset 16 in a6 to offset 10 in a0, then clears several fields at offsets 16, 51, 107, 40, 34, 36, 38, 42, 32, 31 in a0, copies a longword from offset 20 in a6 to offset 18 in a0, and finally stores the values from d5, d6, and d7 into offsets 14, 54, and 52 of a0 respectively.
- `perform_bitwise_or_sequence`: Performs a sequence of bitwise OR operations on register d0 using the constants two and zero.
- `initialize_hardware_parameters`: Calls another routine four times, then loads various values from a structure in a6 into registers and stores them into a structure at the hardware register base address, provided certain byte checks in the structure in a6 pass.
- `parse_command_stream_byte_decoder`: Decrements length counters in a structure pointed to by a0 and enters a loop that reads bytes from a stream to decode command identifiers.
- `branch_to_command_parser`: Performs a short branch to a location within another routine.
- `parse_dma_stream_commands`: Reads a stream of bytes to update a structure's DMA configuration by interpreting command opcodes that modify pointers, counters, and hardware-related values.
- `branch_to_mid_routine`: Performs a short branch to an instruction located within another routine.
- `parse_command_stream_dispatch`: Processes a byte-stream to update a structure in a0 by reading command codes and performing arithmetic, bitwise operations, and memory copies.
- `update_dma_configuration`: Checks a status byte in a structure, and if zero, updates the DMA controller configuration by reading a value from a stream, calculating an offset from a table, and writing the resulting address and length to the structure and the DMA controller register.
- `parse_command_stream`: Parses a byte-stream to update a structure in a0 by reading command codes and performing arithmetic, bitwise operations, and memory copies.
- `apply_relocations_to_table`: Iterates through a list in a0 to add a base address in a4 to values at specific offsets, using a loop that decrements a counter and performs jumps to a table.
- `decompress_bitstream_data_3`: Calculates a source address using a value in d0 and a constant, performs a bitstream decompression via a call, and then calls another routine after incrementing a0 by 8.
- `decompress_bitstream_data`: Decompresses a data stream from a source pointed to by a2 into a destination buffer in a0 using a variable-length encoding scheme.
- `decompress_bitstream_data_2`: Copies a block of data from a source buffer to a temporary work area and then performs a bitstream decompression by reading values from the work area using a series of offsets determined by a bit-index, writing the resulting bytes to a destination buffer until a counter reaches zero.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `calculate_table_index`: Sets the carry flag based on a byte at a memory location, loads a word from a global variable into d0, doubles it twice, and then loads a long value from a table using d0 as an offset.
- `initialize_address_pointers`: Loads several addresses into registers, increments d0 three times, calls a routine at an indirect address, and performs a branch to another routine.
- `branch_to_sub_79f0`: Performs a short branch to a specific address.
- `branch_to_subroutine`: Performs a short branch to another routine.
- `write_constant_values_2`: Writes the value 10 to a global word, calls two other routines, writes a long value to a global word, and then writes a long value and its swapped version to a table starting at a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_constant_values`: Stores a series of sixteen constant long and word values into a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_and_mask_hardware_parameters`: Calls initialize_hardware_parameters_2 and process_string_and_data, then clears the two least significant bits of two global variables before returning if a specific global variable is zero.
- `initialize_hardware_parameters_2`: Performs a loop that writes values to two memory locations at offsets 2 and 6 of structures pointed to by a6, updates a global variable multiple times, and enters two loops that wait for a byte at an offset of a structure to reach specific values.
- `process_string_and_data`: Contains a loop that calls three different routines, including one that parses a null-terminated string, and another that copies eight long words between global locations.
- `copy_and_loop_calls`: Copies eight long words from one global location to another, calls three different routines in a loop until the zero flag is set, then sets a global variable to fifteen and negates it three times before returning.
- `configure_blt_hardware`: Saves registers, sets several hardware registers via a pointer to the custom chip, calculates a value based on d0, stores it in a hardware register, and waits for a specific bit in a hardware register to clear before returning.
- `loop_blt_transfers`: Saves registers, configures hardware registers, and enters a loop that performs three iterations of updating a hardware register with an incrementing address and waiting for a specific bit in a hardware register to clear.
- `parse_string_to_value`: Iterates through a null-terminated string in a0, modifying a counter based on specific character matches or looking up values in a table, and returns the final counter value.
- `parse_string_and_update_blt`: Parses a null-terminated string to calculate a value based on character matches or table lookups, then uses that value to configure Blitter registers and perform a series of Blitter transfers using data from a table structure.
- `search_and_initialize_table`: Searches a table for a byte greater than zero using a loop, then calculates an offset based on that byte to clear a block of memory and writes a pointer to that memory into another table using a loop.
- `update_function_pointer`: Stores a longword constant into a global variable at offset 516.
- `update_audio_volume_scaling`: Decrements a global variable and, if it remains non-negative, calls another routine and scales four values from a table by the global variable using 16-bit multiplication and bit shifting before storing them in the four audio volume registers; otherwise, it clears the four audio volume registers, clears a global variable, and sets the audio volume register for channel zero to fifteen.
- `update_conditional_status`: Saves registers to the stack, increments a byte at a global address, compares a byte from a PC-relative offset to another, and either branches or calls another routine.
- `update_audio_registers`: Stores a value derived from a register into a structure at offset 50 and another value into a structure at offset 54, then updates the hardware audio channel registers.
- `update_object_attributes`: Reads data from a table based on a value derived from a pointer, then updates a structure in a6 with new coordinate and property values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `shift_and_store_registers`: Shifts the contents of d0 left by 8 bits and d1 left by 7 bits, then stores these values into a structure at offset 80 of a2.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_input_data`: Checks a value at offset 2 of a6; if it matches a specific pattern, it calls another routine and stores results in a structure at a2, otherwise it performs arithmetic on values from a6 and updates a structure at a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_registers_to_structure`: Stores the 32-bit contents of d0 and d1 into a structure at offset 80 and offset 84 of a2.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_object_attribute_and_dma`: Checks a status byte at offset 2 of a structure in a6 to determine which attribute update routine to call, then performs a table lookup to update a 16-bit value at offset 16 of the structure.
- `update_audio_and_dma_parameters`: Modifies audio or DMA-related registers and a global variable using a loop that checks a value against a table or a threshold.
- `update_structure_attribute_value`: Reads a nibble from offset 2 of a structure in a6 to select a logic branch, then updates a 16-bit value at offset 16 of the structure using either table lookups, arithmetic based on a byte at offset 3, or direct addition/subtraction.
- `update_attribute_from_table`: Calculates an offset using a byte from offset 18 of a structure in a6, searches a table for a value that satisfies a comparison against a 12-bit mask from offset 0, and writes the result to offset 24 of the structure in a6 while updating offset 22 based on a comparison with offset 16.
- `update_structure_values`: Modifies a 16-bit value at offset 16 of a structure in a6 based on a byte at offset 23, performs a table lookup using a value from offset 18, and writes a result to offset 6 of a structure in a5.
- `process_object_attributes`: Performs complex arithmetic and table lookups to update various offsets in a structure in a6, including values at offsets 6, 8, 19, 27, 28, 29, and 32, and modifies several global variables.
- `update_object_attributes_2`: Reads a byte from offset 2 of a structure in a6 and branches based on its lower nibble to perform various attribute updates.
- `extract_attribute_nibble`: Extracts the lower nibble from a byte at offset 3 of a structure and stores it in offset 18 of the same structure before returning.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `shift_stored_shorts`: Loads two 16-bit values from a structure at offset 64 of a2, shifts them left by 8 and 7 bits respectively, and stores the results at offset 80 and 84 of a2.
- `load_addresses_and_branch`: Loads two addresses into a1 and a3 and then branches to another routine.
- `load_addresses_and_branch_alt`: Loads two different addresses into a1 and a3 and then branches to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `unimplemented_stub`: The routine contains no instructions and returns immediately.
- `empty_stub_3`: The routine contains no instructions and performs no operation.
- `empty_routine_2`: The routine is empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.

### D_03 (raw code file, load address unknown)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
