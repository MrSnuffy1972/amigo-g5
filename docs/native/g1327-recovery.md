# Game 1327 (pacmania (1988)(grandslam)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01327). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (926400 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 57 raw-word lines |
| Functions named and described | pass, 81 of 81 named (0 author symbols), 81 of 81 described (5 by rule, 76 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 837 of 837 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1631 of 1641 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 11649 |
| code | 6674 |
| data | 8999 |
| padding | 6 |
| unused | 899072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Pacmania (1988)(Grandslam).adf:bootblock | 1 | 186 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 786 | 47 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 669 | 25 |

## Cases handled

- runtime segment an address: 494 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 169 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- `setup_disk_dma_wait` (bootblock): Clears bit 2 of the global byte at a fixed address using the joystick fire state from offset 6 of the structure in a6, then configures the disk controller by writing the sync word, pointer and control registers and polls the interrupt request register with a countdown timeout before returning or writing the disk length register on failure. Kind: disk sync/status check. Pass path: the branch following the call where the code continues normal execution (e.g., bra.w an address or bra.w an address).
- `modify_input_and_set_registers` (runtime segment an address): Updates a bit in a global variable based on a masked bit from a pointer-referenced memory location and writes a constant value to a custom register. Kind: disk hardware/sync check. Pass path: the branch following the bsr.w an address call (the code continues to addq.b #an address, an address(a6) if the check succeeds).

## Functions


### Pacmania (1988)(Grandslam).adf:bootblock

- `branch_to_offset`: Performs a short branch to an offset.
- `manage_disk_loading_sequence`: Performs a sequence of disk operations including copying data, modifying a byte in a specific memory location, and updating a counter in a structure in a6.
- `increment_disk_status_loop`: Calls a routine to scan for a sync word, increments a byte at offset 0 of a structure in a6 until it reaches 11, increments a word at offset 6 of that structure, and if the lowest bit of that word is 0, it modifies a byte at a hardware register and calls another routine before branching to a different location.
- `setup_disk_dma_wait`: Clears bit 2 of the global byte at a fixed address using the joystick fire state from offset 6 of the structure in a6, then configures the disk controller by writing the sync word, pointer and control registers and polls the interrupt request register with a countdown timeout before returning or writing the disk length register on failure.
- `scan_sync_word_read_block`: Scans memory from a fixed start address for the sync word 17545, skipping repeated sync words and calling a routine until its result matches, then stores 128 longwords from another routine into a buffer and advances the buffer pointer.
- `combine_masked_pointer_bits`: Returns the data register ANDed with the longword at the second pointer, ORed with the data register ANDed with the first pointer's longword shifted left by one bit.
- `modify_disk_status`: Clears bit 0 of a byte at an offset of a hardware register and then stores the incremented value back to that same location, followed by a call to another routine and a move of a constant into d0.
- `write_word_bytes_then_poll`: Writes the low and high bytes of the data register word to two consecutive memory locations, then polls a status byte until its low bit is set.
- `mask_byte_and_store_value`: Keeps the top two bits of one memory byte, sets its bit 3, and writes the value 127 to a second memory byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer_table_2`: Calculates a base address by summing several offsets from a structure in a6, then iterates through a table to update a series of pointers.
- `setup_system_state`: Writes values to custom hardware registers, updates several global variables with calculated longword components, calls other routines, populates a table of pointers, modifies a byte in a custom register, and executes a trap.
- `update_global_with_offset_address`: Calculates a new address by adding an offset to a pointer and stores the result in a series of global variables as 16-bit words.
- `load_byte_from_address`: Clears the data register and then loads a byte from a memory location into it before returning.
- `update_memory_constants`: Updates a series of memory locations relative to a pointer with a sequence of constant values.
- `dispatch_state_variable`: Loads a pointer from a global variable into a7, calls another routine, and then jumps to a different routine via a pointer stored at the same global variable location.
- `clear_and_add_pointer`: Compares a longword at a global address with a data register, performs several bitwise OR operations on that register, adds a word from a register to a pointer, and performs a final bitwise OR on the register.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `update_mouse_and_memory`: Calculates an offset using a value from a register and the JOY0DAT register, modifies a byte at a pointer-referenced location using a bitwise XOR, modifies another byte using a bitwise OR, increments a value at a pointer-referenced location, and performs several other arithmetic and memory operations before the routine terminates unexpectedly.
- `process_joystick_input`: Reads a byte from a hardware register, stores it in a global variable, and if bit 1 is set, writes a value to a custom register; if bit 3 is set, it clears d0, reads another hardware register, performs bitwise operations, and stores the result in a global variable before checking several other global variables and potentially calling other routines.
- `store_byte_to_address`: Stores the byte from d0 into a memory location and branches to another routine.
- `conditionally_store_byte`: Tests a word at a global address and, if it is non-zero, stores the byte from d0 into a memory location and branches to another routine.
- `test_and_store_with_loop`: Tests a 16-bit global variable and, if non-zero, stores a byte from d0 into a global variable, performs a loop that clears a bit in a byte at an offset of a pointer, and finally restores registers before returning from an interrupt.
- `set_word_and_branch`: Stores the word 1 into a memory location and branches to another routine.
- `clear_and_compare_byte`: Clears bit 7 of d0, compares d0 with a byte from a global variable, and if they are not equal, stores 255 in that global variable before branching to another routine.
- `compare_and_set_byte_2`: Compares a byte in d0 with a byte at a global variable; if they are not equal, it stores the value 255 at that global variable and branches to another routine, otherwise it branches to a third location.
- `compare_and_set_byte`: Compares a byte in d0 with a byte at a specific memory location; if they are not equal, it stores the value 255 at that location and branches to another routine.
- `set_byte_and_branch`: Stores the byte 255 into a memory location and branches to another routine.
- `mask_interrupt_requests`: Saves the current stack pointer, reads the interrupt request register, masks bits 0 through 8, writes the result to the interrupt mask registers, restores the stack pointer, and returns from interrupt.
- `initialize_memory_tables`: Clears several global variables, fills a series of memory locations with zero using a loop, populates a table with specific values using a loop, and performs several memory writes and calls to other routines.
- `set_blt_registers_2`: Writes specific constant values to several memory-mapped or global registers, including certain bitplane or blt-related registers, using values from two different global variables.
- `tiny_andi_ori`: Small routine of 2 instructions that masks d0 with 0, then merges 0 into d0.
- `bitwise_or_and_test`: Performs a bitwise OR on d0 with 54, tests a word at a global address, and returns if the word is non-zero.
- `update_global_from_table`: Tests two bytes at a global address, decrements a word at a global address if the first byte is non-zero, and if the second byte is zero, loads a byte from a table into a global variable using a value from that global variable as an index.
- `save_registers_and_rotate`: Saves several registers to the stack, masks the lower bits of a data register, clears a register, moves a byte from one register to another, clears that register, and performs a bitwise rotation on a word.
- `return_stack_value`: Adds d2 and d1 to d4, performs a rotate left on d0, calls a subroutine, restores multiple registers from the stack, increments d4, and returns.
- `apply_mask_to_memory`: Iterates through a sequence of instructions, applying bitwise masks or negations to certain bytes and storing the results into a memory table at large offsets.
- `tiny_ori`: Small routine of 1 instruction that merges 100 into d2.
- `calculate_and_store_table_values`: Calculates a value using a 16-bit word and a byte from a structure in a6, then writes several 16-bit results to specific offsets within a table calculated from a byte in the structure.
- `calculate_and_store_values`: Performs a loop five times to calculate two values based on a source table and a global offset, then stores these values and a byte into a structure pointed to by a5.
- `write_short_values`: Writes two 16-bit constant values to offsets of 4 and 6 of a pointer in a5 and branches to an earlier instruction.
- `store_count_results`: Calls another routine four times and stores each returned 16-bit value into consecutive 16-bit locations pointed to by a pointer in a1.
- `count_matching_bytes`: Iterates through a buffer pointed to by a0 for 383 iterations, incrementing d0 for every byte that equals 1, and returns the count.
- `update_pointer_table`: Calculates an offset using a value in d0, loads a longword from a table at that offset, and if not zero, updates several global longwords with values derived from the loaded data and subsequent table entries.
- `store_four_longwords`: Compares a word at offset 2 of a0 to 272; if it is less, it adds 63424 to d2, then stores d0, d1, d2, and d3 into four consecutive longword locations starting at a global address.
- `clear_color_registers`: Calls another routine and then uses a loop to write zero to thirty-two consecutive words starting at a hardware register address.
- `execute_wrapped_subroutines`: Saves registers to the stack, calls a routine, restores the registers, calls another routine, and returns.
- `process_data_structure`: Loads several long words into a structure in a6 from d1, d2, and a memory location pointed to by a0, calls four different routines, and then enters a loop that increments a byte at offset 0 of the structure and a word at offset 6 of the structure until the byte reaches 11, while also testing bit 0 of the word at offset 6 to conditionally call another routine.
- `modify_input_and_set_registers`: Updates a bit in a global variable based on a masked bit from a pointer-referenced memory location and writes a constant value to a custom register.
- `scan_and_copy_data_blocks`: Searches for a specific 16-bit pattern in memory, then enters a loop to copy data from one location to another using a routine that performs bitwise shifts, while updating a pointer and a counter in a structure in a6.
- `combine_masked_pointer_bits_2`: Fetches two longwords from two incrementing pointers, masks both with a value in a data register, shifts the first masked value left by one bit, and returns the bitwise OR of the two results.
- `toggle_joystick_bit`: Reads a byte from a hardware register, clears the lowest bit, increments the value, and writes it back to the same register before calling another routine.
- `write_word_bytes_then_poll_2`: Writes the low byte and high byte of a data register to offsets 1024 and 1280 of a structure pointed to by a1, then polls a status byte at offset 3328 until its low bit is set before returning.
- `mask_byte_and_store_value_2`: Keeps the top two bits of a byte at offset 3584 of the structure in a1, sets its bit 3, and writes the value 127 to a byte at offset 3328 of the same structure.
- `decrement_and_branch_loop`: Decrements a register while checking a bit in a memory location, then performs a secondary decrementing loop.
- `adjust_global_range`: Compares two words at offsets 6 and 8 of a structure in a6; if they differ, it modifies a byte at offset 256 of a structure in a1, calls two subroutines within a loop of 10 iterations, and then enters a loop that increments or decrements the word at offset 6 of the structure in a6 while calling another routine if a bit is set.
- `decrement_and_mask_register`: Decrements a register until a bit in a hardware register is cleared, then masks and modifies a byte at an offset of a global variable, followed by a call to another routine and a fixed-count loop of a third routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_entity_attributes_from_table`: Checks if a global variable equals 19; if so, it clears the variable, calls another routine, and enters an infinite loop.
- `parse_control_string`: Iterates through a string, updating a global byte based on specific characters and calling another routine when a character is encountered.
- `execute_infinite_loop`: Calls another routine and then enters an infinite loop.
- `set_blt_registers`: Sets several hardware registers or memory locations to specific values based on a global variable comparison.
- `load_address_and_branch`: Loads a long address from a memory location into a4 and branches to another routine.
- `branch_to_address`: Loads a word into d4 and branches to another address.
- `fill_memory_with_routine`: If a byte at a specific address is greater than 1, it executes another routine in a loop for a number of iterations determined by that byte, incrementing a longword constant each time.
- `configure_memory_block`: Writes a sequence of constant values and calculated pointer or register values to a memory area using a pointer.
- `setup_blt_registers`: Sets certain hardware registers or custom memory-mapped registers to specific constant or calculated values.
- `save_registers_and_branch_to_offset`: Saves the contents of eight data registers and six address registers onto the stack and branches to an offset within another routine.
- `save_registers_and_branch`: Saves registers to the stack, loads a longword from a global variable into d0, and branches to another routine.
- `branch_to_address_alt`: Loads a word into d4 and branches to another address using a different constant.
- `format_numeric_string`: Adds a word from d4 to a global longword, then performs a series of divisions and subtractions to store the resulting digits as characters into a series of memory locations.
- `move_word_from_table`: Moves a word from a table at an offset determined by a register into a register.
- `set_blt_and_modify_byte`: Sets several hardware registers to specific values and performs a bitwise OR on a byte at an offset from a pointer.
- `parse_control_string_and_update_globals`: Iterates through a string to update a global byte based on specific characters, calls a routine when a character is encountered, and writes a sequence of four 16-bit values to a global variable.
- `load_and_branch_from_global`: Saves the current processor state, loads a longword from a global variable into register d0, and performs a short branch to another routine.
- `call_copy_routine`: Calls another routine and returns if the returned value in a3 is not zero.
- `copy_data_table`: Copies eleven elements from one table to another if a comparison with a global value succeeds, using a loop.
- `initialize_and_process_game_loop`: Initializes several global variables and memory locations with constant values, then enters a loop that processes input, updates game state through various subroutines, and manages a countdown timer.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `clear_and_branch`: Clears a 16-bit word in a global variable and otherwise it returns.
- `write_constant_sequence`: Writes a sequence of four constant 16-bit values to a global variable.
- `jump_with_constants`: Loads two 32-bit constants into registers and performs a long jump.
