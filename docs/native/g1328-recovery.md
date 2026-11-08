# Game 1328 (pacmania (1988)(grandslam)[cr band][t +2 band]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01328). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (927632 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 39 raw-word lines |
| Functions named and described | pass, 81 of 81 named (0 author symbols), 81 of 81 described (5 by rule, 76 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 990 of 990 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1584 of 1594 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 12620 |
| code | 6510 |
| data | 9425 |
| padding | 5 |
| unused | 899072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Pacmania (1988)(Grandslam)[cr Band][t +2 Band].adf:bootblock | 1 | 159 | 6 |
| logged disk load from Pacmania (1988)(Grandslam)[cr Band][t +2 Band].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 786 | 47 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 454 | 18 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 181 | 8 |

## Cases handled

- pacmania (1988)(grandslam)[cr band][t +2 band]/Pacmania (1988)(Grandslam)[cr Band][t +2 Band].adf: the boot block reads 512 bytes from disk offset 0 to an address (found statically); recovered as a unit
- runtime segment an address: 494 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 169 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 12 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 173 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- pacmania (1988)(grandslam)[cr band][t +2 band]/Pacmania (1988)(Grandslam)[cr Band][t +2 Band].adf: 1 logged loads cover 0 bytes of the image

## Cracktros (kept, not protection)

- none

## Copy protection found

- `modify_input_and_set_registers` (runtime segment an address): Updates a bit in a global variable based on a masked bit from a pointer-referenced memory location and writes a constant value to a custom register. Kind: disk hardware/sync check. Pass path: bsr.w an address (the call to the routine) followed by beq.w an address (the branch taken if the check fails/returns zero, implying the 'success' path continues linearly or via a different branch).
- `setup_disk_dma_wait` (runtime segment an address): Clears bit 2 of the global byte at a fixed address using the joystick fire state from offset 6 of the structure in a6, then configures the disk controller by writing the sync word, pointer and control registers and polls the interrupt request register with a countdown timeout before returning or writing the disk length register on failure. Kind: disk sync/track check. Pass path: the branch following the call where the code continues normal execution (the 'after' instructions provided indicate the routine is called, and then the code proceeds to clear a flag and branch/call elsewhere, implying the routine is a block.

## Functions


### Pacmania (1988)(Grandslam)[cr Band][t +2 Band].adf:bootblock

- `initialize_boot_sequence_2`: Sets a global variable to 32512, then enters a loop that iterates through bits of a value, setting the DMACON register to enable DMA and configuring a transfer with a pointer, a count, and a destination before calling a routine via a pointer.
- `initialize_boot_sequence`: Clears a block of memory starting at a specific address for 32768 words, sets a global byte to 255, calls several routines via pointers, and enters a complex conditional loop involving global variables and library-related data.
- `check_byte_and_branch`: Calls another routine, checks if the byte at the address in a5 is positive, and otherwise returns.
- `calculate_string_length`: Calls a function via a pointer at an offset from a structure in a6, increments a counter in a loop until a null byte is found at an offset from a pointer in a5, and then calls another function via a pointer at a different offset from the structure in a6 before returning.
- `derive_state_from_counter`: Takes the top two bits of a 16-bit counter from the mouse port as a state value, scales that state by sixteen, and writes the result into four fixed memory cells.
- `copy_four_bytes_from_table`: Copies four consecutive bytes from a table entry, selected by a data register scaled by four, into the buffer that the address register points to.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `populate_address_offset_table`: Calculates a series of memory addresses by adding offsets stored in a table to a base address and stores the resulting 16-bit words into a global variable array.
- `initialize_memory_address_tables`: Calculates a series of memory addresses by adding an offset to a base value and stores the resulting 16-bit words into multiple global variables.
- `update_global_with_offset_address`: Calculates a new address by adding an offset to a pointer and stores the result in a series of global variables split into 16-bit words.
- `read_hardware_register`: Loads a 32-bit value from a hardware register into a register and returns.
- `update_memory_constants`: Updates a series of memory locations relative to a pointer with a sequence of constant values.
- `dispatch_function_via_pointer`: Loads a pointer from a global variable into the stack pointer, calls another routine, and then jumps to a destination address loaded from the same global variable.
- `clear_and_add_pointer`: Compares a longword at a specific address with a value in d0, performs several bitwise OR operations on d0 with zero, adds the value in a2 to a4, and performs one final bitwise OR on d0 with zero.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `update_mouse_input_state`: Reads the mouse port movement, performs a bitwise XOR on a byte in a table using a bitmask derived from a register, and modifies several values in memory structures via offsets and calculated indices.
- `handle_input_and_update_state`: Reads a byte from a hardware register, stores it in a global word, and if bit 3 is set, performs bitwise operations on a byte from a hardware register and writes the result to a global address after checking several sign flags.
- `store_byte_and_branch`: Stores a byte from d0 into a longword at a global address and then branches to another routine.
- `test_and_store_byte`: Tests a word at a global address, branches if non-zero, otherwise stores a byte from d0 into a global address and branches to another routine.
- `process_conditional_data`: Tests a longword at a global address, and if non-zero, moves a byte from d0 to a global address, performs a loop based on d0, modifies a byte at offset 3584 of the structure in a1, and writes two words to offsets 156 and 152 of the structure in a0 before returning from a subroutine.
- `write_constant_word`: Writes the word 1 to a longword at a global address and then branches to another routine.
- `clear_bit_and_compare`: Clears bit 7 of d0, compares d0 with a byte at a global address, and if they are equal, writes 255 to that global address and branches to another routine.
- `compare_and_reset_byte`: Compares a byte in d0 with a byte at a global address; if they are not equal, it writes the value 255 to that global address and branches to another routine.
- `compare_and_write_byte`: Compares a byte in d0 with a byte at a global address, and if they are not equal, it writes the byte 255 to that global address and branches to another routine; otherwise, it branches to another routine.
- `write_constant_byte`: Writes the byte 255 to a longword at a global address and then branches to another routine.
- `mask_interrupt_requests`: Saves the current stack pointer, reads the interrupt request register, masks bits 0 through 8, writes the result to the interrupt mask registers, restores the stack pointer, and returns from interrupt.
- `initialize_memory_tables`: Clears a block of memory, fills a table with repeating values, updates several global variables with constant values, and copies data from a table into a series of global variables.
- `set_blt_registers_2`: Writes specific constant values to several memory-mapped or global registers, including certain bitplane or blt-related registers.
- `tiny_andi_ori`: Small routine of 2 instructions that masks d0 with 0, then merges 0 into d0.
- `modify_and_test_global`: Performs a bitwise OR on d0 with 54, tests a word at a global address, and returns if the word is non-zero.
- `update_global_from_table`: Tests two bytes at a global address, decrements a word at a global address if the first byte is non-zero, and if the second byte is zero, loads a byte from a table into a global variable using a value from that global variable as an index.
- `save_registers_and_rotate`: Saves d0-d7 and a0-a3 to the stack, masks d0 with a constant, clears d1, moves a byte from d4 to d1, clears d4, and rotates the word in d2 left by two bits.
- `return_stack_value`: Adds two words to d4, performs a bitwise rotation on d0, calls a subroutine, restores several registers from the stack, increments d4, and returns the value that was present on the stack at the start of the routine.
- `apply_mask_to_memory`: Iterates through a sequence of instructions, applying bitwise masks or negations to certain bytes and storing the results into a memory table at large offsets.
- `tiny_ori`: Small routine of 1 instruction that merges 100 into d2.
- `calculate_and_store_table_values`: Calculates a value based on a 16-bit word and a byte from a structure in a6, then writes several 16-bit results to specific offsets within a table calculated from a byte in the structure.
- `update_structured_data`: Runs a loop five times, calculating two new values based on a source table and a global variable, and writes them into a structure, or writes 65535 and 65534 if certain conditions are not met.
- `write_short_values`: Writes 65535 to offset 4 of the structure in a5, writes 65534 to offset 6 of the structure in a5, and branches to another routine.
- `store_count_results`: Calls another routine four times and stores each returned 16-bit value into consecutive 16-bit locations pointed to by a pointer in a1.
- `count_matching_bytes`: Iterates through a buffer pointed to by a0 for 383 iterations, incrementing d0 for every byte that equals 1, and returns the count.
- `calculate_and_store_offsets`: Calculates a value based on a memory address, then stores four longwords into consecutive memory locations, potentially adding specific offsets to d1, d2, and d3 based on comparisons.
- `store_four_longwords`: Compares a word at offset 2 of a0 to 272; if it is less, it adds 63424 to d2, then stores d0, d1, d2, and d3 into four consecutive longword locations starting at a global address.
- `clear_color_registers`: Calls another routine, then clears 32 words starting at the color register address in a0 using a loop, and returns.
- `execute_setup_sequence`: Saves registers to the stack, calls the routine at 3b1a, restores the registers, calls another routine, and returns.
- `process_structured_data`: Stores several longwords into a structure at a0, calls other routines, and enters a loop that increments a byte in a structure and calls another routine until the byte reaches 11.
- `modify_input_and_set_registers`: Updates a bit in a global variable based on a masked bit from a pointer-referenced memory location and writes a constant value to a custom register.
- `search_and_copy_data`: Searches for a specific 16-bit word in memory, and if found, calls another routine to process data and then enters a loop that copies bytes to a destination pointed to by a structure in a6, potentially calling another routine and updating pointers and counters.
- `combine_masked_pointer_bits`: Returns the data register ANDed with the longword at the second pointer, ORed with the data register ANDed with the first pointer's longword shifted left by one bit.
- `modify_cia_register`: Clears bit 0 of a byte at an offset from a pointer, increments that byte, and then calls another routine.
- `write_word_bytes_then_poll`: Writes the low and high bytes of the data register word to two consecutive memory locations, then polls a status byte until its low bit is set.
- `mask_byte_and_store_value`: Keeps the top two bits of one memory byte, sets its bit 3, and writes the value 127 to a second memory byte.
- `decrement_and_branch_loop`: Decrements a register while checking a bit in a memory location, then performs a secondary decrementing loop.
- `adjust_global_range`: Compares two global words and, depending on which is larger, modifies a byte at a specific address, calls a subroutine, enters a loop to decrement or increment the smaller value, and calls another routine conditionally during the loop.
- `decrement_and_mask_register`: Decrements a register until a bit in a global variable is cleared, then masks a bit and writes a value to a hardware register or memory, followed by a call and a fixed-count loop of another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_entity_attributes_from_table`: Checks if a global variable equals 19; if so, it resets the variable to 0, calls another routine, and enters an infinite loop.
- `set_blt_registers`: Sets several hardware registers or memory locations to specific values based on a global variable comparison.
- `load_address_and_branch`: Loads a longword into a4 and branches to another routine.
- `move_and_branch`: Moves the value 3626 to d4 and branches to another routine.
- `fill_memory_with_routine`: Executes a routine repeatedly while decrementing a value, effectively performing a loop-based memory write.
- `configure_memory_block`: Writes a sequence of constant values and calculated pointer or register values to a memory area using a pointer.
- `setup_blt_registers`: Sets certain hardware registers or custom memory-mapped registers to specific constant or calculated values.
- `set_blt_and_modify_byte_2`: Saves registers to the stack, loads a longword from a global variable into d0, and branches to another routine.
- `load_and_branch`: Loads the value 5147 into d4, loads a longword into a4, and branches to another routine.
- `format_numeric_string`: Calculates digits and stores them as characters into a series of memory locations.
- `move_word_from_table`: Moves a word from a table at an offset determined by a register into a register.
- `set_blt_and_modify_byte`: Sets several hardware registers to specific values and performs a bitwise OR on a byte at an offset from a pointer.
- `update_game_state_and_render_loop`: Initializes several global variables and memory locations, then enters a loop that repeatedly calls various subroutines to update game logic, process input, and perform bitwise block transfers.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `enter_infinite_loop`: The routine enters an infinite loop that does not exit.
- `write_constant_sequence`: Writes a sequence of constant values to a global variable.
- `jump_to_address`: Loads a pointer from a local constant and jumps to a long address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `select_and_copy_data_block`: Checks a byte at a specific memory location; if the value is one, it writes a constant to a global variable, otherwise it writes the value from that location to the global variable, then calls a routine to copy a block of data.
- `copy_data_block`: Copies a block of 512 words from a source table to a destination table using a loop, then jumps to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `manage_system_state`: Copies a block of memory, modifies a byte at a fixed address using a bitwise mask, and enters several loops that increment a counter in a structure, update hardware registers, and call other routines based on specific counter values.
- `increment_and_check_counter`: Calls another routine, increments a byte in a structure at offset 0 of a6, and if that byte is not 11, loops back to the increment; otherwise, it increments a word at offset 6 of a6 and returns if bit 0 of that word is 0, otherwise it branches to another routine.
- `setup_disk_dma_wait`: Clears bit 2 of the global byte at a fixed address using the joystick fire state from offset 6 of the structure in a6, then configures the disk controller by writing the sync word, pointer and control registers and polls the interrupt request register with a countdown timeout before returning or writing the disk length register on failure.
- `scan_sync_word_read_block`: Scans memory from a fixed start address for the sync word 17545, skipping repeated sync words and calling a routine until its result matches, then stores 128 longwords from another routine into a buffer and advances the buffer pointer.
- `combine_masked_pointer_bits_2`: Fetches two longwords from memory using pointers in a0 and a1, masks both with d7, and returns the result of ORing the second masked value with the first masked value shifted left by one bit.
- `derive_state_from_counter_2`: Takes the top two bits of a 16-bit counter from the mouse port as a state value, scales that state by sixteen, and writes the result into fo.
- `write_word_bytes_then_poll_2`: Writes the low byte and high byte of a data register to offsets 1024 and 1280 of a pointer in a1, then polls a status byte at offset 3328 until its low bit is set before returning.
- `modify_status_and_byte`: Sets bit 3 of a byte at offset 3584 of the structure in a6 while masking bits 7 and 6, and writes 127 to offset 3328 of the structure in a6.
