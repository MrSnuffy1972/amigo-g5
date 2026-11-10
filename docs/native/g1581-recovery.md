# Game 1581 (pacmania (1988)(grandslam)[cr dc]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01581). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (926400 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 58 raw-word lines |
| Functions named and described | pass, 87 of 87 named (0 author symbols), 87 of 87 described (6 by rule, 68 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 874 of 874 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1681 of 1691 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 11649 |
| code | 6166 |
| data | 9507 |
| padding | 6 |
| unused | 899072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Pacmania (1988)(Grandslam)[cr DC].adf:bootblock | 1 | 48 | 5 |
| logged disk load from Pacmania (1988)(Grandslam)[cr DC].adf boot-block load | 1 | 188 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 786 | 47 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 669 | 25 |

## Cases handled

- pacmania (1988)(grandslam)[cr dc]/Pacmania (1988)(Grandslam)[cr DC].adf: the boot block reads 6144 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 494 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 169 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- pacmania (1988)(grandslam)[cr dc]/Pacmania (1988)(Grandslam)[cr DC].adf: 1 logged loads cover 0 bytes of the image

## Cracktros (kept, not protection)

- none

## Copy protection found

- `start_disk_read_wait` (Pacmania (1988)(Grandslam)[cr DC].adf boot-block load +an address to an address (6144 bytes)): Programs the disk controller to read: it sets the sync word, the data pointer and the length registers along with the disk control bits, then polls the interrupt request register until the disk block-done bit is set and clears it. Kind: disk hardware sync/status check. Pass path: the branch where bVar3 is false (the loop breaks and execution continues to the logic following the call).
- `modify_input_and_set_registers` (runtime segment an address): Updates a bit in a global variable based on a masked bit from a pointer-referenced memory location and writes a constant value to a custom register. Kind: disk hardware/sync check. Pass path: bsr.w an address (the call to the routine) followed by beq.w an address (the branch taken if the check fails/returns zero, implying the 'pass' is the fall-through or non-zero result).

## Functions


### Pacmania (1988)(Grandslam)[cr DC].adf:bootblock

- `stub_unavailable_boot_entry`: Boot block entry whose decompiled pseudo-code could not be produced, so its behaviour is not confirmed.
- `write_word_then_wait_blitter`: Writes a 16-bit value into a register at a fixed offset from the address register, then waits for the blitter to finish.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `check_dos_library_resident`: Looks up the DOS library in the system's resident module list and returns zero when it is present, or minus one when it is not.
- `open_intuition_and_run_action`: Opens intuition.library through the older open call when a condition holds, sets a status bit while one library routine runs, then clears the bit and closes the library.

### logged disk load from Pacmania (1988)(Grandslam)[cr DC].adf boot-block load

- `stub_unavailable_entry_call`: The decompiled pseudo-code could not be produced for this boot-segment routine, so its behaviour is not confirmed.
- `initialize_disk_and_counters`: Copies a block of data from a source address to a destination address, modifies a byte at a specific memory location, and then enters loops that increment a counter in a global variable while periodically calling routines to wait for disk reads or CIA status changes.
- `count_frames_wait_flag`: Repeats a call ten times while counting a byte up to eleven, then increments a word counter.
- `start_disk_read_wait`: Programs the disk controller to read: it sets the sync word, the data pointer and the length registers along with the disk control bits, then polls the interrupt request register until the disk block-done bit is set and clears it.
- `poll_disk_block_with_retries`: Polls the disk status bit in nested counted loops, restarting the disk read registers whenever a wait expires.
- `find_sync_decode_sectors`: Searches a buffer for the disk sync pattern, then decodes 128 longwords through a routine and stores each result sequentially in a destination buffer.
- `combine_masked_longs_shifted`: Combines two longwords under a mask: bits from the second longword are kept, and bits from the first longword are shifted up by one position and kept, then the two are ORed together.
- `set_cia_bit_wait_status`: Sets the low bit of a CIA control register, calls a routine, writes two bytes at fixed offsets from a base address, and waits until a status bit in the same device is set.
- `write_byte_pair_then_wait`: Writes the low and high bytes of a word to two fixed offsets from a base address, then waits until a status bit at a third offset is set.
- `set_control_bits_clear_status`: Keeps the top two bits of a control byte, sets bit three, and writes a fixed byte to a second register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_offset_table_to_pointers`: Calculates a base address by summing several offsets from a structure in a6 and then iterates through a table to update a list of pointers by adding values retrieved from that table.
- `configure_system_state`: Updates several global variables with swapped and offset versions of a constant, calls another routine, performs a loop to write words to memory, updates several function pointers in a table, modifies hardware registers, and finally calls a routine via a trap.
- `update_global_with_offset_address`: Calculates a new address by adding an offset to a pointer and stores the result in a series of global variables as 16-bit words.
- `read_cia_status_byte`: Reads a byte from a specific memory location into register d0 and returns.
- `update_memory_constants`: Updates a series of memory locations relative to a pointer with a sequence of constant values.
- `dispatch_via_function_pointer`: Loads a function pointer from a global location into the stack pointer, calls that function, and then jumps to another function pointer loaded from the same global location.
- `clear_and_add_pointer`: Compares a byte at a global address with a register, performs several bitwise OR operations on that register, adds a word from a register to another register, and performs a final bitwise OR on the register.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `update_mouse_input_state`: Reads the mouse port movement, performs bitwise operations on a table entry using a calculated index, and modifies several values in memory structures via offsets in registers a2 and a6.
- `process_joystick_input`: Reads a byte from a hardware register, stores it as a word in a global variable, and if bit 1 is set, writes a value to a hardware register; if bit 3 is also set, it performs bitwise manipulations on a byte from a hardware register, stores the result in a global variable, and performs several conditional branches based on the sign of the result and other global bytes.
- `store_byte_and_branch`: Stores a byte from a register into a global memory location and branches to another routine.
- `test_and_store_byte`: Tests a 16-bit word at a global address; if it is non-zero, it branches to another routine, otherwise it stores a byte from a register into a global memory location and branches to another routine.
- `process_conditional_data`: Tests a 16-bit value at a global address, otherwise it performs a loop that modifies a byte at an offset of a1, stores a byte from d0 to a global variable, and returns via an exception return.
- `write_constant_and_branch`: Writes the word one to a global variable and branches to another routine.
- `clear_and_compare_byte`: Clears bit 7 of d0, compares d0 with a 16-bit value at a global address, and if they are not equal, writes 255 to that global address and branches to another routine; otherwise it branches to a different location.
- `compare_and_set_byte_2`: Compares a byte in d0 with a byte at a global address; if they are not equal, it writes 255 to that global address and branches to another routine.
- `compare_and_set_byte`: Compares a byte in d0 with a byte at a global address; if they are not equal, it writes the value 255 to that global address and branches to another routine, otherwise it branches to another routine.
- `write_byte_and_branch`: Writes the byte ff to a global variable and branches to another routine.
- `mask_interrupt_requests`: Saves the current stack pointer, reads the interrupt request register, masks bits 0 through 8, writes the result to the interrupt mask registers, restores the stack pointer, and returns from interrupt.
- `initialize_memory_and_hardware`: Clears several global variables, fills a table with zeroed words, updates a series of memory locations relative to a pointer with constant values, performs bitwise operations on a value from a global address, and writes several values to hardware registers and memory locations.
- `update_blitter_registers`: Writes two longwords from a global address to offsets 80 and 84 of the hardware registers, clears offset 102, sets offset 64 to 3840, clears offset 66, calls another routine three times, then repeats the process using a different global address and calls that routine three more times.
- `tiny_andi_ori`: Small routine of 2 instructions that masks d0 with 0, then merges 0 into d0.
- `modify_and_return`: Performs a bitwise OR on d0 with 36, then tests a word at a global address and returns if it is non-zero; otherwise it returns.
- `update_global_from_table`: Tests two bytes at a global address, decrements a word at a global address if the first byte is non-zero, and if the second byte is zero, loads a byte from a table into a global variable using a value from that global variable as an index.
- `save_registers_and_rotate`: Saves several registers to the stack, masks the lower bits of a register, clears a register, moves a byte to another register, clears that byte, and then rotates a word in a register twice.
- `return_stack_value`: Adds d2 and d1 to d4, performs a rotate left on d0, calls a subroutine, restores multiple registers from the stack, increments d4, and returns.
- `apply_mask_to_memory`: Iterates through a sequence of instructions, applying bitwise masks or negations to certain bytes and storing the results into a memory table at large offsets.
- `tiny_ori`: Small routine of 1 instruction that merges 100 into d2.
- `calculate_and_store_table_values`: Calculates a value based on a 16-bit word and a byte from a structure in a6, then writes several 16-bit results to specific offsets within a table calculated from a byte in the structure.
- `update_structure_values`: Runs a loop five times to copy bytes from a source structure to a destination structure, performing calculations on d0 and d1 to determine values for offsets 4 and 6 of the destination structure, and writes to offset 2 of the destination structure.
- `write_short_values`: Writes two constant 16-bit values to offsets 4 and 6 of the structure in a5 and branches to another routine.
- `store_count_results`: Calls another routine four times and stores each returned 16-bit value into consecutive 16-bit locations pointed to by a pointer in a1.
- `count_matching_bytes`: Iterates through a buffer pointed to by a0 for 383 iterations, incrementing d0 for every byte that equals 1, and returns the count.
- `calculate_memory_offsets`: Performs calculations based on values at an offset of a0 and updates several global variables with derived addresses and values.
- `store_four_longwords`: Compares a word at offset 2 of a0 to 272; if it is less, it adds 63424 to d2, then stores d0, d1, d2, and d3 into four consecutive longword locations starting at a global address.
- `clear_color_registers`: Calls another routine and then uses a loop to write zero to thirty-two consecutive words starting at a hardware register address.
- `execute_sequential_subroutines`: Calls a subroutine, then calls another subroutine before returning.
- `process_table_data`: Loads three values into a structure at a6, calls two routines, clears a byte at offset 0 of the structure, calls two more routines, and then enters a loop that increments the byte at offset 0 until it reaches 11, at which point it clears the byte and increments a word at offset 6 before checking a bit in a word from offset 6 and calling another routine.
- `modify_input_and_set_registers`: Updates a bit in a global variable based on a masked bit from a pointer-referenced memory location and writes a constant value to a custom register.
- `search_and_update_table`: Searches for a sequence of 16-bit words in a table, calls another routine, and then enters a loop that rotates d0 and writes the resulting bytes to a destination table pointed to by a global variable.
- `combine_masked_pointer_bits`: Returns the data register ANDed with the longword at the second pointer, ORed with the data register ANDed with the first pointer's longword shifted left by one bit.
- `modify_cia_register`: Clears the lowest bit of a byte at an offset of a1, increments that byte, and calls another routine.
- `write_word_bytes_then_poll`: Writes the low and high bytes of the data register word to two consecutive memory locations, then polls a status byte until its low bit is set.
- `mask_byte_and_store_value`: Keeps the top two bits of one memory byte, sets its bit 3, and writes the value 127 to a second memory byte.
- `decrement_and_branch_loop`: Decrements a register while checking a bit in a memory location, then performs a secondary decrementing loop.
- `adjust_global_range`: Compares two words at offsets 6 and 8 of a structure in a6; if they differ, it modifies a byte at offset 256 of a structure in a1, calls two subroutines within a loop of 10 iterations, and then enters a loop that increments or decrements the word at offset 6 of the structure in a6 while calling another routine if a bit is set.
- `decrement_and_mask_register`: Decrements a register until a bit in a hardware register is cleared, then masks and modifies a byte at an offset of a global variable, followed by a call to another routine and a fixed-count loop of a third routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_state_from_table`: Checks if a global variable equals 19; if so, it clears the variable, calls another routine, and enters an infinite loop.
- `parse_control_sequence_string`: Iterates through a null-terminated string pointed to by a register, updating a global byte to 0 or 1 based on whether it encounters a carriage return or a specific control character, and calls another routine when encountering any other character.
- `execute_infinite_loop`: Calls another routine and then enters an infinite loop.
- `configure_blitter_sequences`: Calculates an offset from a global value to select a data source, then performs multiple blitter operations to draw graphics with varying parameters.
- `load_address_and_branch`: Loads a longword from a global address into a6 and branches to another routine.
- `branch_to_offset`: Loads a word into a register and branches to a different location.
- `fill_memory_with_routine`: If a byte at a specific address is greater than 1, it executes another routine in a loop for a number of iterations determined by that byte, incrementing a longword constant each time.
- `execute_blitter_draws`: Sets up a blitter structure with specific dimensions and colors, then calls another routine three times with incrementing destination offsets.
- `perform_blitter_updates`: Sets up a blitter structure and calls another routine three times with incrementing destination offsets.
- `save_registers_and_branch`: Saves registers d0 through d7 and a0 through a6 onto the stack and branches to another routine.
- `save_and_branch`: Saves registers to the stack, loads a longword from a global variable into a register, sets two registers, and branches to another routine.
- `parse_control_sequence_string_2`: Iterates through a null-terminated string to update a global byte to 0 or 1 based on whether it encounters a carriage return or a specific control character, while calling another routine if an unrecognized character is found.
- `format_numeric_string`: Adds a word from d4 to a global longword, then performs a series of divisions and subtractions to store the resulting digits as characters into a series of memory locations.
- `load_word_from_table`: Loads a 16-bit word from a table at an offset determined by a register into a register.
- `update_bit_and_draw`: Sets bits 4 and 5 in a byte at a table offset and calls another routine twice with incrementing destination offsets.
- `parse_control_string_and_update_globals`: Iterates through a null-terminated string to update a global byte based on specific characters, calls parse_control_sequence_string, and writes a sequence of four 16-bit values to a global variable.
- `load_and_branch`: Saves registers to the stack, loads a longword from a global variable into a register, and branches to another routine.
- `call_copy_routine`: Calls another routine and then compares the result in a register against zero.
- `copy_data_table`: Copies eleven elements from one table to another if a comparison with a global value succeeds, using a loop.
- `initialize_and_run_game_loop`: Initializes several global variables and memory buffers with constant values, then enters a main loop that processes input, updates game state via multiple subroutines, and manages a countdown timer.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `clear_and_branch`: Clears a 16-bit word in a global variable and otherwise it returns.
- `write_constant_sequence`: Writes a sequence of four constant 16-bit values to a global variable.
- `transfer_data_and_jump`: Loads two 32-bit constants into registers and jumps to another routine.
