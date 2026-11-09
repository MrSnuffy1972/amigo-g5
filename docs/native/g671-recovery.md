# Game 671 (SuperTwintris v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00671). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (639963 top-level bytes in 12 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 30 raw-word lines |
| Functions named and described | pass, 132 of 132 named (0 author symbols), 132 of 132 described (19 by rule, 111 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2267 of 2267 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 268288 bytes, 8 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3679 of 3679 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 5778 |
| code | 18346 |
| data | 615772 |
| padding | 67 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 410 | 36 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 151 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2941 | 70 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Super-Twintris (raw code file, load address unknown) | 1 | 155 | 8 |

## Cases handled

- SuperTwintris_v1.0/SuperTwintris_v1.0.hdf: hard-disk image (AmigaDOS OFS, 672 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree SuperTwintris_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 115 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1832 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 1 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 0 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- SuperTwintris_v1.0/SuperTwintris_v1.0_hdf/SuperTwintris/data/Super-Twintris: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 268288 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `dispatch_cop2_routines`: The routine stores a pointer in a global variable and executes indirect calls through offsets in a structure passed in register A0.
- `dispatch_indirect_jump`: The routine performs an indirect jump through a pointer to a jump table, transferring control to an address determined at runtime.
- `dispatch_indirect_jump_2`: Performs an indirect jump to a target address retrieved from a jump table.
- `mask_global_value`: The routine reads a global variable, applies a bitwise mask to clear specific bits, and returns the resulting value.
- `read_interrupt_request`: The routine reads the INTREQR hardware register and returns the value, likely checking for pending interrupt requests.
- `store_interrupt_request`: The routine writes a value from the stack into the INTREQ hardware register, likely acknowledging or clearing an interrupt request.
- `return_zero_value`: The routine returns a zero value immediately without performing any other operations or modifying data.
- `set_flag_in_struct`: The routine receives a pointer in register A1 and sets bit 1 in the byte at offset 14 of the pointed-to structure.
- `empty_stub_routine_2`: The routine is empty and returns immediately without performing any operations or modifying data.
- `patch_self_modifying_code`: The routine checks a global status variable and, if a specific condition is met, overwrites numerous memory locations with jump instructions pointing to new targets.
- `store_register_to_global`: The routine copies the value from a CPU register into a global variable associated with CIAB_PRA and then returns without further processing.
- `setup_cop2_dispatch`: The routine stores a global variable value, likely related to COP2LCH, and then performs an indirect jump through a jump table.
- `copy_byte_via_pointers`: The routine copies a single byte from a source address calculated via one pointer to a destination address calculated via another pointer.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `check_global_condition`: The routine checks if a global variable equals one of two specific values; if not, it returns 1, otherwise it returns 4.
- `initialize_pointer_table_entries`: The routine writes a sentinel value into a global variable and then stores the address of the next entry in that variable into three pointer fields located in two different structures, and also clears a word field in one of those structures.
- `copy_pointer_values`: Checks two pointers derived from an input address; if non-null, it dereferences them to copy 32-bit values into specific offsets of a global structure.
- `negate_nonzero_flag`: Returns zero if the input value in D0 is non-zero, and returns negative one if the input value is zero.
- `rotate_and_compare_byte`: Performs a bitwise rotation on a byte and compares the result against a global variable.
- `enter_infinite_loop_2`: Executes an infinite loop by repeatedly checking if a global variable is equal to itself.
- `calculate_scaled_offset`: Computes a value using division and multiplication on elements of a global array, then stores the result into a specific offset of a global structure.
- `check_bit_and_store_char_2`: The routine reads a byte from a global structure and checks a flag bit; if clear, it performs bit-testing logic to either store a constant or calculate a character value, potentially invoking a function pointer.
- `process_character_bit_logic`: The routine tests a specific bit in a global variable; if the bit is clear, it stores a constant byte at a memory offset, otherwise it calculates a character value based on a table index.
- `check_bit_and_store_char`: Tests a specific bit in a global variable; if clear, stores a constant byte at an offset in memory, otherwise calculates a character value from a table and calls a function pointer.
- `process_disk_data_byte`: The routine calculates a character value based on an index into a global table, calls another routine via a pointer, and returns a byte from an offset in a passed pointer.
- `empty_stub_2`: The routine is empty and immediately returns without performing any operations.
- `return_immediately`: The routine contains no instructions other than the return statement.
- `patch_code_with_self_modifying_jumps`: The routine checks a global status and, if a specific condition is met, overwrites numerous memory locations with jump instructions pointing back to itself.
- `check_flag_and_dispatch`: Compares a register value against a global variable; if they match, it executes an indirect function call from a global table.
- `set_global_constants`: Sets three consecutive global variables to the values 102, 20, and 1 respectively.
- `wait_and_set_flags`: Sets two global variables to 102 and 22, sets a third to 1, then waits in a loop until a specific bit in a hardware register is cleared.
- `wait_for_bit_clear`: Waits in a loop until a specific bit in a hardware register is cleared.
- `wait_for_flag_and_loop`: Polls a global status byte until a specific bit is cleared, checks another bit, and then executes a decrementing loop on a register value.
- `enter_infinite_loop`: The routine consists of an unconditional infinite loop that prevents execution from returning to the caller.
- `loop_forever`: The routine enters an infinite loop without performing any operations or returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_register_value`: The routine immediately returns the value currently held in the D0 register.
- `modify_memory_bytes`: The routine clears a specific bit in a byte pointed to by A4, sets bits in a byte at an offset from A2, and returns a constructed value based on D0 and A6.
- `return_d0_value`: Returns the value currently held in the D0 register to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_exec_and_memory_2`: The routine calls a function pointer from a global structure, writes specific constant values to global variables, and modifies memory locations.
- `initialize_exec_and_memory`: The routine calls a function pointer from a global structure, writes specific constant values to global variables, and modifies memory locations via pointers.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `process_input_and_timers`: Reads from the CIAA parallel port A register to handle keyboard or joystick input.
- `read_keyboard_serial`: The routine accesses the CIAA serial data register, indicating it handles input from the keyboard serial port.
- `fill_memory_buffer_loop`: Iterates through a memory range, writing specific constant values and results from other routine calls into consecutive memory locations.
- `calculate_offset_value`: The routine adds a constant to the input in D0, performs bitwise operations, and conditionally modifies the result based on a comparison with the A0 register.
- `decrement_table_entries`: The routine iterates through a table in memory, decrementing specific fields at fixed offsets within each entry until a limit is reached.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_display_and_copy_tables`: Sets hardware interrupt enable bits and video synchronization parameters, updates several global variables with constant values, and performs two loops to copy data from source tables into destination tables.
- `initialize_game_data_structures`: Clears several memory areas using nested loops, including a block of 13 words, a large 2D array of bytes, and a specific sequence of bytes at a fixed offset.
- `update_graphics_and_blitter_state`: Performs multiple blitter operations to clear or set bitmasks in memory tables using loops, then updates various global state variables and hardware registers including the blitter and copper.
- `update_display_bitmasks`: Updates bitmasks in a memory table using a loop based on a global value, modifies several global variables, and performs arithmetic on a data structure.
- `scale_and_store_index`: Checks if a word in a register is between 2000 and 2127 inclusive; if so, it subtracts 2000, multiplies the result by two, and stores it in a global variable before branching to another routine; otherwise, it branches to another routine.
- `update_graphics_and_state_tables`: Updates bitmasks in a table using a loop based on a global value, modifies several global variables, and performs arithmetic on a data structure.
- `update_table_entry`: Checks if a global variable is non-zero, calculates an offset into a table using a value from a global variable, and if a comparison passes, copies several words from the table to a destination structure and updates a bit in a global variable.
- `format_score_string`: Converts a numeric value from a memory location into an ASCII string by repeatedly dividing by ten and storing the resulting digits into a buffer.
- `initialize_render_parameters`: Clears a 16-bit value in a global variable, stores the constant 48 at an offset of 56 in a structure, moves a byte from a global variable to an offset of 57 in that structure, adds 48 to the value at offset 57, sets a pointer to offset 56, and calls another routine.
- `update_system_state`: Updates several global variables by adding or subtracting values based on comparisons, calls two routines, and writes values to hardware registers and a global variable before returning.
- `tiny_addi`: Small routine of 1 instruction that adds 16 to d5.
- `tiny_ori`: Small routine of 1 instruction that merges 69 into d0.
- `set_d0_bits`: Sets bit 6 and bit 2 of d0 using a bitwise OR operation.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into (a0).
- `tiny_addi_2`: Small routine of 1 instruction that adds 0 to d5.
- `increment_data_register_d5`: Adds sixteen to the value stored in data register five.
- `set_data_0_register`: Performs a bitwise OR operation on the low byte of register d0 with the constant sixty-nine.
- `configure_blt_and_draw`: Calculates several values based on global variables, configures Blitter registers, and calls another routine within a loop that runs 5 times.
- `configure_blt_and_jump`: Calculates values from global variables, configures Blitter registers, and jumps to another routine.
- `branch_on_loop_counter`: Decrements a loop counter in d7 and branches to an earlier instruction if it is not zero, otherwise it branches to another routine.
- `swap_and_update_globals`: Clears a 16-bit value in a global variable, performs a series of 16-bit word moves between several global memory locations, and branches to another routine.
- `scale_color_data`: Performs a nested loop that scales 16-bit values by multiplying components with a decreasing multiplier and stores the results in a table.
- `process_global_data`: Enters a loop that repeats while the lowest 3 bits of d0 are not 7, clears four 16-bit global variables, performs a multiplication, updates several global variables and pointers, and finally writes 65535 to a global variable.
- `update_sprite_attribute_buffer`: Calculates an offset into a table based on a value divided by 512, then writes a series of constant bytes to specific offsets within a memory block.
- `tiny_ori_3`: Small routine of 1 instruction that merges 255 into d0.
- `render_sprite_batch_2`: Sets up the blitter to draw multiple blocks of data, then enters a loop that decrements a counter, performs bitwise checks on a data structure, and writes specific byte patterns to memory based on those checks.
- `execute_blt_loop`: Decrements a global 16-bit value, calculates Blitter parameters using a multiplier, and calls another routine within a loop controlled by d1.
- `update_global_with_xor`: Reads a 16-bit value from a global variable, performs an XOR operation with two bytes from a table, and updates the global variable with the result.
- `tiny_ori_4`: Small routine of 1 instruction that merges 0 into d0.
- `clear_data_register_zero`: Performs a logical OR operation on data register zero with zero.
- `configure_blitter_loop`: Sets several hardware registers for the blitter, then enters a loop that runs four times, calling another routine and incrementing a value from a global variable by 2800 each iteration.
- `perform_logical_or`: Performs a logical OR operation on register d0 with zero.
- `modify_registers`: Performs a logical OR operation on register d0 with zero, a logical OR operation on register d0 with six, and a bitwise rotate right on register d2.
- `performs_logical_or_operation`: Performs a logical OR operation on data register zero using zero as the operand.
- `clear_data_register_zero_2`: Sets the least significant byte of data register zero to zero.
- `tiny_ori_5`: Small routine of 1 instruction that merges 0 into d6.
- `clear_data_register_d0`: Performs a logical OR operation on data register d0 with zero.
- `perform_logical_or_on_d0`: Performs a bitwise logical OR operation on the lowest byte of the d0 register using a zero constant.
- `tiny_ori_6`: Small routine of 1 instruction that merges 26 into d6.
- `tiny_ori_7`: Small routine of 1 instruction that merges 64 into d4.
- `copy_state_blocks`: Performs a large series of 16-bit and 32-bit moves between various global variables and structures, including updating a pointer and a constant value.
- `swap_state_blocks`: Copies multiple 16-bit and 32-bit values between global variables and structures, effectively swapping or updating large blocks of state data.
- `perform_no_operation`: Performs a logical OR operation on register d0 with zero, resulting in no change to the register's value.
- `save_data_table`: Copies several 16-bit and 32-bit values from various global variables and structures into a specific table in memory.
- `update_frame_state_and_dispatch`: Clears a series of 16-bit values in a global structure, increments a counter, and if the counter reaches ten, resets it and updates two values from a table.
- `update_and_loop_indices`: Checks a global variable and, if non-zero, enters a loop that calls another routine to update a global pointer until a value is less than two, then sets a value at a hardware register and executes a second loop that calls another routine twenty-nine times.
- `calculate_and_copy_table`: Clears a 16-bit global variable, sets a hardware register, calls another routine, performs a division and a multiplication to calculate offsets, and then copies a large number of 16-bit and 32-bit values into various global variables and structures.
- `nullify_data_register_zero`: Performs a logical OR operation on data register zero with zero, effectively leaving the register unchanged.
- `render_sprite_batch`: Reads a sequence of bytes from a buffer, calculates memory offsets based on the byte values, and performs multiple Blitter operations to draw sprites using a loop.
- `process_keyboard_input`: Reads a byte from the keyboard serial register, performs a bitwise rotation and negation, stores the result in a global variable, and jumps to another routine.
- `clear_byte_at_offset`: Clears bit 7 of d0, stores the byte 0 at the memory location pointed to by a0 plus the offset in d0, and branches to an earlier instruction.
- `write_byte_at_offset`: Extends the sign of a word in d0 and stores the byte value of 255 at the memory location pointed to by a0 plus the offset in d0, then returns.
- `modify_memory_values`: Tests a bit in a memory location pointed to by a0, performs bitwise OR operations on values at specific offsets using d0 and a2, and moves a longword from a table into d0.
- `update_data_tables`: Runs a loop to populate a table with values derived from a longword, then runs a second loop that calculates an index based on a byte from a table and copies multiple words from a source table to a destination table.
- `update_buffer_with_logic`: Enters a loop that runs 5 times, reading values from a table, performing bitwise rotations and arithmetic, and writing the results to a buffer at a specific offset.
- `decrement_and_copy_indices`: Enters a loop that runs 5 times, reading values from a table using an index from a global variable, adding constants to those values, and writing them to a buffer, then decrements the global index and wraps it to 98 if it becomes negative.
- `reset_audio_and_tables`: Sets a bit in a hardware register, searches through a table to find a specific byte, uses that byte to calculate addresses for a loop that fills a table with values, and finally clears several hardware registers and global variables.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `process_audio_parameters`: Decrements four 16-bit global variables and ensures they do not fall below zero, calls two routines, and then updates four other global variables based on the value of a fifth global variable and bit tests.
- `update_audio_channels`: Increments a counter in a global variable and, upon reaching five, resets it and performs a series of updates to audio hardware registers via a sub-routine.
- `update_audio_and_dma_parameters`: Updates audio channel parameters and DMA settings by modifying values in a structure based on a command byte, including scaling a value via multiplication and updating hardware registers for four audio channels and the DMA controller.
- `increment_and_clear_counters`: Clears a longword, increments a longword in a global variable, and if that variable matches a constant, clears it and checks a word in another global variable, which it clears before looping back to the start of the routine.
- `update_sprite_and_command_data`: Processes a structure to update sprite coordinates, attributes, and DMA-related values by reading from a table and writing to a destination structure.
- `wait_for_vblank`: Loops while the value at the video port register is zero, then loops again while it is non-zero, and finally waits for bit 0 of the register at offset 5 to be set before returning.
- `update_audio_volumes`: Checks four 16-bit values at specific offsets in a table; if a value is zero, it copies a corresponding 16-bit word from another location in that table to the corresponding audio volume register.
- `update_game_state_and_render_sprites`: Checks various status flags and compares values in a global variable to determine if a state change is required.
- `execute_conditional_sequence`: Performs a series of checks by calling another routine and comparing a value at an offset of a global variable, then updates several hardware registers and calls a function at a specific address if certain conditions are met.
- `tiny_ori_8`: Small routine of 1 instruction that merges 1 into d0.
- `process_pointer_and_data`: Compares a pointer in a global variable to a constant, writes a byte to a memory location, increments the pointer, performs bitwise operations on a value from the pointer, and stores a 16-bit result into a global variable.
- `process_conditional_data_updates`: Compares a value in a global variable to a constant and calls another routine that writes a byte to a memory location via a pointer, performs a bitwise operation, and increments that pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `return_immediately_stub`: The routine consists of a single return instruction followed by an unconditional branch, so it performs no work and returns to its caller immediately.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.

### Super-Twintris (raw code file, load address unknown)

- `initialize_program_flow`: Serves as an entry point that calls three other routines.
- `manage_library_access`: Forbids execution and opens a library through the operating system.
- `call_library_function`: Executes a library function call using a specific library offset.
- `access_hardware_registers`: Manipulates the CIAA port register and the video hardware vertical/horizontal position register.
- `read_keyboard_serial_2`: Interacts with the keyboard serial data register.
- `call_subroutine_and_process`: Executes another routine before completing its own sequence of instructions.
- `execute_short_sequence`: Performs a brief sequence of nine instructions.
- `execute_minimal_logic`: Performs a short sequence of eight instructions.
