# Game 1575 (pacmania (1988)(grandslam)[a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01575). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (926400 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 57 raw-word lines |
| Functions named and described | pass, 81 of 81 named (0 author symbols), 81 of 81 described (5 by rule, 67 by the local model) |
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
| Pacmania (1988)(Grandslam)[a2].ADF:bootblock | 1 | 186 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 786 | 47 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 669 | 25 |

## Cases handled

- runtime segment an address: 494 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 169 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- `modify_input_and_set_registers` (runtime segment an address): Updates a bit in a global variable based on a masked bit from a pointer-referenced memory location and writes a constant value to a custom register. Kind: disk hardware/sync check. Pass path: bsr.w an address (the call to the routine) followed by beq.w an address (the branch taken if the check fails/returns zero, implying the non-zero/success path continues to the next instruction).

## Functions


### Pacmania (1988)(Grandslam)[a2].ADF:bootblock

- `call_routine_from_boot_entry`: One-instruction boot block entry, called by the Kickstart strap, that makes a single call to an unnamed routine.
- `initialise_display_and_hardware`: Disables all interrupts, sets CIA control bits and runs timed waits and calls to set up the display.
- `step_eleven_times_then_wait`: Calls an unnamed routine eleven times while counting a byte up.
- `start_disk_dma_and_wait`: Sets the disk sync word, the disk pointer and the ADKCON control word, then polls the disk block completion bit of the interrupt request register with a timeout before writing the disk length register.
- `scan_sync_word_read_block`: Searches memory from a fixed start address for the disk sync word, skipping repeated sync words, and checks an unnamed routine's result until it matches.
- `combine_masked_odd_even_longwords`: Masks two longwords with a mask value and returns the first masked result ORed with the second masked result shifted left by one bit.
- `signal_device_and_wait_flag`: Sets a CIA-B register bit, calls an unnamed routine, writes two bytes at fixed device offsets and busy-waits on a status flag.
- `write_word_bytes_and_wait`: Writes the low and high bytes of its 16-bit input to two device offsets, then busy-waits until a status flag bit is set.
- `set_control_bits_and_reset`: Keeps the top two bits of a control byte, sets bit three, then writes a fixed value to a second device register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_parameter_table`: Calculates a base address by summing several offsets from a structure in a6, then iterates through a table to update a series of pointers with incremental values.
- `setup_system_parameters`: Writes values to hardware registers, updates several global variables with a constant 32-bit value and its increments, calls another routine to perform a loop that stores words, updates a table of pointers with the addresses of several routines, modifies a hardware register via bitwise operations, and finally calls a routine via a trap.
- `update_global_with_offset_address`: Loads a 32-bit pointer from a global variable, stores its lower and upper 16-bit words into a series of global variables, and then performs repeated additions of 104 to the pointer to store subsequent 16-bit word pairs into more global variables.
- `read_hardware_port`: Clears register d0 and then moves a byte from the hardware port at a fixed address into it before returning.
- `update_memory_constants`: Updates a series of memory locations relative to a pointer with a sequence of constant values.
- `dispatch_state_via_pointer`: Loads a pointer from a global variable into the stack pointer, calls another routine, and then jumps to a location specified by a pointer at the same global address.
- `clear_and_add_pointer`: Clears a byte in a register through multiple logical operations, adds a word from a register to another register, and clears the byte in the register again.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `update_mouse_and_memory`: Uses the mouse port to calculate an offset, modifies a byte at a pointer-referenced location using a bitwise XOR, updates several values in a structure, and increments a value at a pointer.
- `process_input_and_hardware`: Saves registers, reads a byte from a hardware register, stores it in a global word, and if bit 1 is set, writes a value to a hardware register; if bit 3 is set, it clears d0, reads from another hardware register, modifies the byte via bitwise operations and rotation, and stores it in a global variable before performing several conditional branches based on the contents of global variables.
- `store_and_branch`: Stores a byte from a register into a memory location pointed to by a global variable and branches to a relative offset.
- `test_and_store_byte`: Tests a 16-bit word at a global address; if the result is non-zero, it branches to another routine, otherwise it stores a byte from a register into a memory location pointed to by a global variable and branches to a relative offset.
- `process_conditional_data`: Tests a 16-bit value at a global address, then either performs a loop to clear a bit in a memory location or stores a byte and restores registers before returning from an interrupt.
- `store_word_and_branch`: Stores the word value 1 into a longword at a global address and branches to another location.
- `clear_and_branch_bit`: Clears bit 7 of d0, compares d0 with a 16-bit value at a global address, and if they are not equal, branches to another routine; otherwise it stores the byte value 255 at that global address and branches to another routine.
- `compare_and_reset_byte`: Compares a byte in a global variable with the contents of a register; if they are not equal, it branches to another routine, otherwise it stores the value two hundred fifty-five in that global variable and branches to another routine.
- `check_and_set_byte`: Compares a byte at a global address with the contents of d0, then either branches to another routine or stores the value 255 at that same global address.
- `store_byte_and_branch`: Stores the byte value 255 into a longword at a global address and branches to another location.
- `mask_interrupt_requests`: Saves the current stack pointer, reads the interrupt request register, masks bits 0 through 8, writes the result to the interrupt mask registers, restores the stack pointer, and returns from interrupt.
- `initialize_memory_tables`: Clears several global variables, fills a series of memory locations with values from a table using a loop, calculates a value by shifting a register, and then updates multiple global variables and memory locations with constant values and data from a table.
- `set_blt_registers_2`: Writes specific constant values to several memory-mapped or global registers, including certain bitplane or blt-related registers, using values from two different global variables.
- `tiny_andi_ori`: Small routine of 2 instructions that masks d0 with 0, then merges 0 into d0.
- `bitwise_or_and_test`: Performs a bitwise OR on d0 with 54, then tests a word at a global address and returns if the value is non-zero.
- `update_global_from_table`: Tests two bytes at a global address, decrements a word at a global address if the first byte is non-zero, and if the second byte is zero, loads a byte from a table into a global variable using a value from that global variable as an index.
- `save_registers_and_rotate`: Saves several registers to the stack, masks the lower bits of a register, clears a byte, and rotates a word in a register right by two bits.
- `return_stack_value`: Adds d2 and d1 to d4, performs a rotate left on d0, calls a subroutine, restores multiple registers from the stack, increments d4, and returns.
- `apply_mask_to_memory`: Iterates through a sequence of instructions, applying bitwise masks or negations to certain bytes and storing the results into a memory table at large offsets.
- `tiny_ori`: Small routine of 1 instruction that merges 100 into d2.
- `calculate_and_store_table_values`: Calculates a value based on a 16-bit word and a byte from a structure in a6, then writes several 16-bit results to specific offsets within a table calculated from a byte in the structure.
- `update_structure_values`: Runs a loop five times to copy bytes from a source structure to a destination structure, performing arithmetic and conditional checks on values to determine what is stored in offsets 4 and 6 of the destination structure.
- `write_short_values`: Writes two 16-bit values to offsets 4 and 6 of the structure in a5 and branches to an earlier instruction.
- `store_count_results`: Calls another routine four times and stores each returned 16-bit value into consecutive 16-bit locations pointed to by a pointer in a1.
- `count_matching_bytes`: Iterates through a buffer pointed to by a0 for 383 iterations, incrementing d0 for every byte that equals 1, and returns the count.
- `calculate_memory_offsets`: Calculates an index based on a word, checks a value at that index, and updates several global variables with calculated offsets based on comparisons of values at that index.
- `store_four_longwords`: Compares a word at offset 2 of a0 to 272; if it is less, it adds 63424 to d2, then stores d0, d1, d2, and d3 into four consecutive longword locations starting at a global address.
- `clear_color_registers`: Calls another routine and then uses a loop to write zero to thirty-two consecutive words starting at a hardware register address.
- `execute_sequential_routines`: Calls clear_color_registers and process_structured_data in sequence, then returns.
- `process_structured_data`: Loads several values into a structure, calls four different routines, and then enters a loop that increments a byte in the structure and a word at offset 6 of the structure until the byte reaches 11, while also calling another routine based on a bit test of the word at offset 6.
- `modify_input_and_set_registers`: Updates a bit in a global variable based on a masked bit from a pointer-referenced memory location and writes a constant value to a custom register.
- `search_and_copy_data`: Searches for a specific 16-bit pattern in a memory buffer, and if found, calls another routine to process data before performing a loop that rotates d0 and stores the resulting bytes into a table.
- `combine_masked_pointer_bits`: Returns the data register ANDed with the longword at the second pointer, ORed with the data register ANDed with the first pointer's longword shifted left by one bit.
- `mask_and_increment_byte`: Clears the lowest bit of a byte at a specific offset in a CIA register, increments that byte, and calls another routine.
- `write_word_bytes_then_poll`: Writes the low and high bytes of the data register word to two consecutive memory locations, then polls a status byte until its low bit is set.
- `mask_byte_and_store_value`: Keeps the top two bits of one memory byte, sets its bit 3, and writes the value 127 to a second memory byte.
- `decrement_and_branch_loop`: Decrements a register while checking a bit in a memory location, then performs a secondary decrementing loop.
- `adjust_global_range`: Compares two words at offsets 6 and 8 of a structure in a6; if they differ, it modifies a byte at offset 256 of a structure in a1, calls two subroutines within a loop of 10 iterations, and then enters a loop that increments or decrements the word at offset 6 of the structure in a6 while calling another routine if a bit is set.
- `decrement_and_mask_register`: Decrements a register until a bit in a hardware register is cleared, then masks and modifies a byte at an offset of a global variable, followed by a call to another routine and a fixed-count loop of a third routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_state_from_table`: Checks if a global variable equals 19; if so, it clears the variable, calls another routine, and enters an infinite loop.
- `parse_control_sequence_string`: Iterates through a null-terminated string, updating a global byte to 0 or 1 based on whether it encounters a carriage return or a specific control character, while calling another routine for any other character encountered.
- `execute_infinite_loop`: Calls another routine and then enters an infinite loop.
- `set_blt_registers`: Sets several hardware registers or memory locations to specific values based on a global variable comparison.
- `load_address_and_branch`: Loads a longword from a global address into a6 and branches to another location.
- `branch_to_offset`: Loads a word into a register and branches to a relative offset.
- `fill_memory_with_routine`: If a byte at a specific address is greater than 1, it executes another routine in a loop for a number of iterations determined by that byte, incrementing a longword constant each time.
- `configure_memory_block`: Writes a sequence of constant values and calculated pointer or register values to a memory area using a pointer.
- `setup_blt_registers`: Sets certain hardware registers or custom memory-mapped registers to specific constant or calculated values.
- `save_registers_and_branch`: Saves registers d0 through d7 and a0 through a6 onto the stack and branches to another location.
- `save_and_branch`: Saves registers to the stack, loads a longword from a global variable into a register, loads a pointer into a register, sets two registers to specific values, and branches to a relative offset.
- `parse_control_sequence_string_2`: Iterates through a null-terminated string, setting a global byte to 1 if a specific control character is found, setting it to 0 if a carriage return is encountered, and calling another routine for any other character.
- `format_numeric_string`: Adds a word from d4 to a global longword, then performs a series of divisions and subtractions to store the resulting digits as characters into a series of memory locations.
- `load_word_from_table`: Loads a 16-bit word from a table using an offset calculated from registers into a register.
- `modify_and_set_blt_registers`: Sets bit 4 and 5 of a byte at an offset in a table and then sets several hardware registers to specific values.
- `parse_control_string_and_update_globals`: Iterates through a null-terminated string to update a global byte based on specific characters, calls a routine when a character is encountered, and writes a sequence of four 16-bit values to a global variable.
- `load_and_branch`: Saves registers to the stack, loads a longword from a global variable into a register, and branches to a relative offset.
- `call_and_compare`: Sets a register to zero, calls another routine, and then compares a register to zero before returning.
- `copy_data_table`: Copies eleven elements from one table to another if a comparison with a global value succeeds, using a loop.
- `initialize_and_run_game_loop`: Initializes several global variables and memory locations with constant values, then enters a loop that processes input, updates game state via multiple subroutines, and manages a countdown timer.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `clear_and_branch`: Clears a 16-bit word in a global variable and otherwise it returns.
- `write_constant_sequence`: Writes a sequence of four constant 16-bit values to a global variable.
- `load_and_jump`: Loads two 32-bit constants into registers and jumps to a long address.
