# Game 203 (SensibleSoccer9293 v1.2 2391): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00203). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4242597 top-level bytes in 95 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 14 of 14 units identical (bytes and relocations), 116 raw-word lines |
| Functions named and described | pass, 549 of 549 named (0 author symbols), 549 of 549 described (32 by rule, 500 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 7330 of 7331 (99.99%) |
| Recovered without load address (counted in proof 1, flagged) | 673964 bytes, 66 functions |
| Independent decoder cross-check (Ghidra against capstone) | 12693 of 13067 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1096282 |
| code | 48286 |
| data | 3097348 |
| padding | 681 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| SensibleSoccer9293512K.Slave | 1 | 176 | 12 |
| game.slave | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2290 | 90 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 206 | 20 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7191 | 237 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2131 | 114 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 6 |
| doscode (raw code file, load address unknown) | 1 | 1054 | 61 |
| soccer (raw code file, load address unknown) | 1 | 1 | 1 |
| soccerc (raw code file, load address unknown) | 1 | 1 | 1 |
| soccerf (raw code file, load address unknown) | 1 | 1 | 1 |
| soccers (raw code file, load address unknown) | 1 | 1 | 1 |
| socceru (raw code file, load address unknown) | 1 | 1 | 1 |

## Cases handled

- SensibleSoccer9293_v1.2_2391/SensibleSoccer9293_v1.2_2391_hdf/SensibleSoccer9293/data/disk2: raw disk image (3362 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- SensibleSoccer9293_v1.2_2391/SensibleSoccer9293_v1.2_2391.hdf: hard-disk image (AmigaDOS OFS, 4640 blocks): its filesystem parses and all 87 files in it are byte-identical to the extracted tree SensibleSoccer9293_v1.2_2391_hdf, which is recovered file by file; no block outside the filesystem holds data
- runtime segment an address (play-long-joy): 1261 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 57 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 4767 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1095 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- SensibleSoccer9293_v1.2_2391/SensibleSoccer9293_v1.2_2391_hdf/SensibleSoccer9293/data/doscode: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- SensibleSoccer9293_v1.2_2391/SensibleSoccer9293_v1.2_2391_hdf/SensibleSoccer9293/data/soccer: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- SensibleSoccer9293_v1.2_2391/SensibleSoccer9293_v1.2_2391_hdf/SensibleSoccer9293/data/soccerc: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- SensibleSoccer9293_v1.2_2391/SensibleSoccer9293_v1.2_2391_hdf/SensibleSoccer9293/data/soccerf: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- SensibleSoccer9293_v1.2_2391/SensibleSoccer9293_v1.2_2391_hdf/SensibleSoccer9293/data/soccers: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- SensibleSoccer9293_v1.2_2391/SensibleSoccer9293_v1.2_2391_hdf/SensibleSoccer9293/data/socceru: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- SensibleSoccer9293_v1.2_2391/SensibleSoccer9293_v1.2_2391_hdf/SensibleSoccer9293/data/disk2: chunks 0-13 (bytes 0-3362) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.87)
- 5 never-loaded region(s), 670698 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `modify_port_bitmask` (runtime segment an address (play-long-joy)): Performs bitwise operations to update a global variable using a value from a table and a bit from another global variable, then writes the result to a hardware port. Kind: disk hardware/track check. Pass path: beq.b an address (at 20D6) or beq.b an address (at 2092).
- `decrement_port_bitmask` (runtime segment an address (play-long-joy)): Calculates a bitwise value from a table and a global variable, then decrements the result before storing it in a global variable. Kind: bitmask manipulation for hardware/input validation. Pass path: the instruction following the bsr.w an address call, which proceeds to move #an address into an address.w.
- `initialize_dma_and_ciab` (runtime segment an address): The routine configures hardware by writing to DMACON, DSKLEN, and CIAB registers to enable DMA and set interrupt settings. Kind: disk hardware timing/checksum trap. Pass path: returns 0.
- `set_ciab_port_bits` (runtime segment an address): Checks a global variable and, if zero, performs a series of assignments to global variables and calls another routine to modify the CIAB port B register. Kind: disk hardware check (CIAB PRB). Pass path: returns from u2__sub_23F0.
- `wait_cia_timer_flag` (doscode (raw code file, load address unknown)): The routine sets a flag, checks a stack byte, and repeatedly calls helpers to configure CIA timer registers and update custom chip registers until a counter expires or a condition is met. Kind: disk hardware timing check. Pass path: returns from the function (loop breaks when DAT_00bfe001 bit 3 is cleared or counter expires).
- `configure_cia_timer_loop` (doscode (raw code file, load address unknown)): Checks a global flag and input register, then writes constant values to CIA timer hardware registers in a loop until a specific bit condition is met. Kind: not judged: Unterminated string starting at: line 4 column 16 (char 81). Pass path: not determined.
- `set_cia_port_and_wait` (doscode (raw code file, load address unknown)): Sets a global variable to a specific value, calls a routine to configure CIA timer registers, and waits in a loop until a bit in the global variable is clear. Kind: disk hardware timing check. Pass path: returns 0.

## Functions


### SensibleSoccer9293512K.Slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `check_cheat_codes`: Retrieves input codes and compares them against specific strings like 'soccer' and 'socwc' to detect cheat entries.
- `process_input_command_table`: Calls function pointers from a structure to retrieve an input code, then checks if the code matches specific values like 'socceru' or 'socwcu'. If matched, it updates global state variables and calls another function pointer before executing a jump to a fixed address.
- `execute_infinite_loop`: Enters an infinite loop after performing a conditional table branch.
- `enter_infinite_loop`: This routine enters an infinite loop that prevents further execution.
- `store_constant_value`: Stores a constant 32-bit value into a global variable.
- `invoke_table_function_pointer`: The routine invokes a function pointer located at an offset within a global table and returns the value in register D1.
- `dispatch_interrupt_handler`: Checks an input value; if it equals eight, it enables interrupts, calls a function pointer at offset 28, and then disables interrupts.
- `call_offset_30_function`: The routine calls a function pointer located at an offset of 48 from a base address and returns without a value.
- `call_function_pointer`: Invokes a function pointer located at an offset within a global table and returns the value in register D1.
- `conditional_table_call`: Checks a global variable for non-zero; if true, calls a function pointer from a global table, then returns a combined value from registers D0 and D1.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### game.slave

- `return_constant_value`: Returns the constant value -1 (a fixed address) immediately upon entry.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_and_input_state`: Clears a memory range, initializes multiple tables with specific patterns, and configures hardware registers including interrupts and DMA. It then enters a polling loop that monitors specific values before executing a function pointer from a calculated offset in a table.
- `execute_callback_loop_2`: Repeatedly calls another routine and updates a global variable until a zero flag is set, then performs two more calls with different global values and enables interrupts via the INTENA register.
- `update_global_via_call`: Calls another routine and updates a global variable using the returned value and a constant.
- `execute_conditional_loop_3`: Checks a global variable and, if non-zero, executes a series of subroutine calls including a loop that decrements a value derived from a returned result.
- `clear_global_variable`: Sets a global variable to zero twice.
- `reset_global_variable`: Sets a global variable to zero and performs arithmetic operations if the variable was non-zero.
- `shift_memory_block_2`: Iterates through a memory structure to compare bytes and performs block moves and arithmetic updates on table entries.
- `increment_and_compare`: Increments a word, compares it against a constant, and branches based on the result.
- `shift_memory_block`: Copies a block of memory from one location to another and performs arithmetic updates on a series of values within a table.
- `nop_padding_sequence`: A sequence of three instructions that performs no functional operation and serves as padding within the executable.
- `copy_data_structure`: Checks a global variable against specific bounds and, if valid, copies a null-terminated string from a pointer to a calculated memory location.
- `clear_data_register`: Performs a bitwise OR operation on a data register with zero.
- `compare_and_branch_table`: Compares strings within a table and jumps to an address stored in the table if a match is found.
- `load_pointer_and_data`: Tests a word at a memory location, branches if it is zero, and otherwise loads a pointer into a register and a long value into another register before branching.
- `handle_conditional_copy`: Evaluates a register against several constants to decide whether to copy a block of memory, clear a pointer, or enter an infinite loop.
- `load_table_bytes`: Calculates an offset using a global variable and loads two bytes from a table into a global variable.
- `reset_global_variable_2`: Sets a global variable to zero and executes another routine.
- `set_flag_and_branch`: Writes a constant value to a memory location and performs an unconditional branch.
- `call_subroutine_with_context`: Clears a memory location, saves registers to the stack, updates several memory locations, and calls another routine.
- `validate_and_copy_signature`: Compares a global variable against several four-byte constant values.
- `update_flag_and_branch`: Saves registers to the stack, loads a word from a global variable into a register, loads a pointer from another global variable into a register, writes a constant to a global variable, and performs a short branch.
- `scan_list_with_limit`: Sets up a pointer and a counter, then loops through a sequence of subroutine calls until a condition is met or the counter reaches four.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `search_for_sequence`: Iterates through a sequence using a subroutine, incrementing a counter until a specific value is returned or a limit of four is reached.
- `execute_callback_via_pointer`: Checks a global variable and a register, then executes a routine through a pointer stored in a table.
- `call_subroutines_and_branch`: Calls two subroutines, loads a pointer from a table into a register, and performs a relative branch.
- `call_empty_routine`: calls another routine that performs no operation and then returns to the caller.
- `update_global_pointers_and_constants`: Updates several global variables using values from registers if they are not negative, and sets a global pointer and two constant values.
- `copy_memory_block`: Copies bytes from a source table to a destination address in a loop until a counter reaches zero.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `load_bytes_from_table`: Calculates an offset based on a global variable and loads two consecutive bytes from a table into that global variable.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `write_hardware_registers`: Writes specific values to hardware registers to manipulate interrupts and custom chip settings.
- `clear_hardware_register`: Writes zero to a hardware register.
- `configure_interrupt_registers`: Writes several values to global memory and modifies hardware interrupt control and request registers.
- `set_interrupt_request_and_clear_interrupt`: Sets a global variable to a fixed value and modifies the interrupt request and interrupt clear registers.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `decrement_hardware_counters`: Tests two memory locations and decrements them if they are non-zero.
- `test_hardware_status`: Tests a bit in a hardware register and performs conditional branches based on a bitwise mask applied to a word from memory.
- `check_joystick_button`: Checks a bit in a hardware register and branches based on the result of a bitwise operation on a word from memory.
- `update_bit_status`: Extracts the lowest bit from a word in memory, stores it into a different memory location, and calls another routine.
- `decrement_global_counter`: Tests a word in memory and, if it is not zero, decrements it by one.
- `set_hardware_register`: Writes one to a hardware register.
- `increment_global_and_branch`: Loads two constant values into registers, increments a global word by one, and performs an unconditional short branch to another location.
- `modify_global_word`: Stores a zero in a global variable, calls another routine, and then sets a different global variable to a specific value based on a comparison.
- `add_to_global_variable`: Adds a short value to a global variable and then calls two other routines before setting a global variable to a constant.
- `modify_port_bitmask`: Performs bitwise operations to update a global variable using a value from a table and a bit from another global variable, then writes the result to a hardware port.
- `decrement_port_bitmask`: Calculates a bitwise value from a table and a global variable, then decrements the result before storing it in a global variable.
- `test_cia_bit`: Tests a specific bit in the CIAA Port A register and branches based on the result.
- `check_disk_and_input`: Clears a longword in memory and performs conditional branches based on the status of bits in the CIAA Port A register.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `manage_interrupt_state`: Updates a global variable and recursively calls itself based on bitwise checks, eventually writing to hardware interrupt control registers.
- `execute_empty_calls_2`: Calls two subroutines in succession and returns.
- `reset_input_flag`: Checks specific bits in a global variable and either sets a bit or resets the variable to a constant value.
- `update_global_state_2`: Modifies a global variable using bitwise operations and stores the result into a table at an incrementing index.
- `perform_null_operation`: Performs a logical OR operation on a register with zero.
- `clear_registers`: Clears the lower bits of two registers and loads a byte into a third register.
- `process_conditional_callbacks`: Updates a table and synchronizes two indices, then conditionally executes a series of subroutines and a function pointer stored in a global variable.
- `shift_buffer_elements_2`: Shifts a series of bytes through a circular buffer and performs a conditional loop based on table values.
- `initialize_hardware_tables`: Copies data to a specific memory range, clears several tables, and writes values to hardware registers.
- `update_table_data`: Splits a global variable into multiple parts, calculates new values based on its content, and writes these values into a table using a loop.
- `toggle_global_pointer`: Checks a global variable and updates it with either its own address or a specific constant value, followed by several writes to the same location.
- `wait_for_value_and_set_registers`: Polls a global variable until it reaches a specific threshold, then writes several constant values to it and modifies hardware registers including DMA and interrupt controls.
- `delay_loop`: Calculates a value based on a register and executes a decrementing loop for that duration.
- `check_joy_and_buttons_2`: The routine reads the joystick port movement register and the CIAA port A register, then performs a series of bitwise tests and assignments on a global variable, ending with a conditional write of a constant to that variable.
- `read_joystick_and_mouse_input`: Reads hardware registers for joystick fire buttons and mouse port movement to determine input states.
- `load_blt_parameter_table`: Iterates through a list of pointers, loading values from each pointed-to structure into specific memory locations used for Blitter operations.
- `set_long_value`: Writes a 32-bit constant value through a pointer stored in a global variable.
- `synchronize_two_indices`: The routine adjusts a global variable toward a first short value in D1 and then toward a second short value in D2 by stepping the variable by two each iteration, wrapping it at the limits implied by the code.
- `apply_bit_mask`: Calculates a bitwise mask from two different sources and applies it to a table of values.
- `fill_table_with_masked_data`: Iterates through a table, calling another routine and storing a masked version of a register value into the table entries.
- `scale_table_index_by_factor_2`: The routine reads a byte from a pointer, masks the low four bits, multiplies the result by an unsigned word from D2, and returns a 32-bit value formed by the high 16 bits of the product and the product shifted right by seven bits.
- `clamp_global_value_down`: Checks if a global variable is within a specific range and performs arithmetic to reset it if it falls below zero.
- `clamp_global_value_up`: Checks if a global variable is within a specific range and performs arithmetic to reset it if it exceeds a threshold.
- `wait_for_flag`: Polls a global variable and calls another routine if a specific condition is met.
- `wait_for_flag_change`: Polls a global flag variable until it changes state, initializing another global variable if necessary, and waits for the flag to clear.
- `parse_data_structure`: Iterates through a data structure by checking specific values and jumping through various offsets to perform a block copy of data from a global variable to a destination.
- `update_sprite_attribute_buffer`: Updates a memory structure containing sprite dimensions and position data, then performs a block copy or zero-fill operation to synchronize sprite attribute data with a source buffer.
- `reset_interrupt_and_callback`: Writes a value to a global variable, sets a global pointer to a routine address, and writes a constant to a register.
- `reset_interrupt_state`: Writes values to the interrupt control register and conditionally clears a global variable.
- `write_constant_to_memory`: Writes a constant sixteen-bit value to a global variable.
- `execute_callback_loop`: Saves the current stack pointer to a global variable and jumps to a stored address.
- `save_stack_and_jump`: Saves the current stack pointer to a global variable and jumps to a location stored in a pointer.
- `return_stack_value_3`: Returns a value currently held on the stack.
- `wait_for_flag_toggle`: Executes another routine and enters a loop that waits for a global variable to change from zero to one, followed by a second loop that waits for that same variable to return to zero.
- `setup_blt_graphics_parameters`: Calculates source and destination addresses, dimensions, and offsets for a bitplane linear transfer.
- `rotate_quad_bitmask_array`: Performs a four-dimensional bitwise rotation on a multi-dimensional array of 16-bit values.
- `process_quad_bitmask_rotation`: Performs a nested four-dimensional bitwise transformation on a multi-dimensional array of 16-bit values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `check_global_flag`: Checks if a specific global variable is non-zero and returns immediately if it is.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_game_state`: Sets multiple global words and longwords to specific values and eventually performs an indirect call using a value derived from a register.
- `get_obfuscated_value`: Updates a global variable using a linear calculation and returns a value retrieved from a table using an offset, XORed with the updated global variable.
- `process_joystick_input_events`: Checks multiple global flags to determine if input processing is allowed, then branches based on a returned value to execute specific state changes, toggle flags, or call other routines.
- `check_conditions_and_call`: Compares a register to a constant and checks three memory locations before calling another routine if all conditions are met.
- `compare_and_toggle_bit`: Compares a register to a constant and, if it matches, calls a subroutine and toggles a bit in a global variable.
- `compare_and_call_routines`: Compares a register to a constant and, if they match, calls two different subroutines and modifies a global word.
- `compare_and_set_flag`: Compares a register to a constant and, if certain long words in memory are non-zero, writes a value to a memory location.
- `check_conditional_flags`: Compares a register against a constant and checks if two different long words in memory are non-zero.
- `process_input_commands`: Compares a register against specific constants to trigger different state updates.
- `clear_conditional_words`: Checks a word at an offset from a pointer and, if it is non-zero, clears two consecutive longwords in memory.
- `initialize_global_constants`: If a value at a pointer is two, it sets a global word to five and clears three other global words.
- `initialize_global_values_2`: Sets four different global variables to specific constant values.
- `clear_global_variable_2`: Sets a global variable to zero if its current value is not zero.
- `update_control_flags`: Compares a register against a constant and, if several global words are zero, writes specific values to two global memory locations.
- `toggle_global_flag`: If a value in a register is not eighty, it branches; otherwise, it toggles a bit in a global variable and conditionally copies a value from one global variable to another.
- `copy_global_value`: Copies the value from one global variable into another.
- `copy_register_pair`: moves the contents of two 32-bit registers into a single 64-bit return value.
- `test_global_word`: Tests a global word and always branches to another location.
- `execute_repeated_calls`: Calls another routine multiple times and then enters a loop that calls that same routine repeatedly based on a zero flag.
- `execute_conditional_loop`: Calls another routine within a loop based on the status of a zero flag.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_iterative_sequence`: Sets global flags, calls another routine, and enters a loop that executes up to four times while updating global values based on return results.
- `format_score_string`: Converts two numeric values into a formatted ASCII string containing a hyphen separator.
- `get_table_value`: Calculates a table index using a bitwise XOR and a modulo operation, then returns a value from a table at that index.
- `clear_d0_register`: Performs a bitwise OR operation on the data register zero with zero.
- `format_numeric_string_2`: Converts two numeric values into a formatted string of ASCII characters and stores them in memory.
- `update_joystick_input_buffer`: Clears several global variables and updates a series of memory locations with values derived from a global variable.
- `update_data_tables`: Sets several global variables based on a global value, calculates new values, and uses a loop to store these values into two separate memory tables.
- `prepare_address_registers`: Performs a no-operation, loads a relative address into a register, calls another routine, and then loads a different relative address into the same register.
- `update_sprite_attributes`: Iterates through a list of data to perform bitwise operations and updates values in a memory buffer, affecting hardware color registers.
- `bitwise_and_linked_list`: Iterates through a linked list and performs a bitwise AND operation between a value in a register and the data at each node.
- `rotate_left_linked_list`: Iterates through a linked list and performs a bitwise left rotation on a word at each node.
- `shift_right_linked_list`: Iterates through a linked list and performs a bitwise right shift on a word at each node.
- `increment_table_values`: Reads values from a list of pointers and increments the contents of a global table at the indices specified by those values.
- `search_table_values`: Iterates through a list of pointers and checks or retrieves values from a table.
- `load_register_and_branch`: Loads a small constant into a data register and performs a short branch to another routine.
- `apply_absolute_value`: Calculates an index into a global table, retrieves a value, and stores its absolute value back into a memory location.
- `return_empty_routine`: The routine performs no operations and immediately returns to its caller.
- `null_operation_stub`: The routine is empty and performs no operations.
- `branch_on_value`: Checks if a value in a register is less than two and calls another routine or performs an indirect jump if the condition is not met.
- `dispatch_indirect_call`: Initializes several global variables, executes a sequence of subroutines, and performs an indirect call using a value calculated from a global variable and an offset.
- `wait_for_zero_and_set_flag`: If a global variable is non-zero, it enters a loop until that variable becomes zero, then sets another global variable to a specific value.
- `spin_forever_if_zero`: If a global variable is zero, it enters an endless loop that never returns; otherwise it returns at once.
- `enter_infinite_loop_2`: The routine executes a tight infinite loop without performing any operations or returning control.
- `apply_bitwise_or_masks`: Performs bitwise OR operations using two different immediate masks on the memory locations pointed to by two registers.
- `tiny_ori_3`: Small routine of 1 instruction that merges 222 into d7.
- `tiny_ori_4`: Small routine of 1 instruction that merges 78 into d7.
- `set_data_registers`: Sets specific bits in data registers zero, one, and three using logical OR operations.
- `clear_data_register_d0`: Performs three consecutive logical OR operations with zero on a data register.
- `traverse_pointer_list`: Iterates through a list of pointers by following a chain of addresses while decrementing a counter.
- `clear_memory_byte`: Clears a single byte at the location pointed to by a register using a logical OR operation with zero.
- `iterate_pointer_list`: The routine iterates through a list of pointers while decrementing a counter and checking a value against a constant.
- `copy_memory_block_4`: Copies a block of bytes from a source pointer to a destination pointer using a counter derived from a value at the source.
- `populate_memory_range`: Decrements a counter while writing a decreasing sequence of values to a memory range, then copies bytes from a source to a destination.
- `find_negative_value`: Iterates through a list of values and calls another routine for each entry until a negative value is encountered.
- `copy_memory_block_2`: The routine copies a block of bytes from a source pointer to a destination pointer while decrementing a counter.
- `iterate_pointer_list_2`: Iterates through a list of pointers, calling another routine for each entry until a negative value is encountered.
- `copy_calculated_block`: The routine calculates a block size from a table and copies a certain number of words from a source to a destination.
- `copy_memory_range_2`: Compares two memory addresses and performs either a forward or backward block copy to synchronize a range of memory, followed by a byte-wise copy from a global source.
- `rotate_and_copy_bitstream`: Copies a block of memory and then performs bitwise rotations and shifts on the data, writing the transformed results back to a destination buffer.
- `extract_bitstream_data`: Decodes a bitstream by extracting or shifting bits from a source table and storing the resulting bytes into a destination buffer.
- `pack_bitstream_from_lookup`: Converts a sequence of indices from a lookup table into a packed bitstream by performing bitwise rotations and shifts.
- `calculate_input_modifier_offset`: Iterates through a list of command identifiers to accumulate a running total based on values retrieved from a table.
- `traverse_and_accumulate_table_values`: Iterates through a list of identifiers, calculating an offset into a table to accumulate or decrement a running total.
- `calculate_input_modifier_offset_2`: Iterates through a data table to accumulate a running total based on command identifiers and bitstream values.
- `process_bitstream_data`: Iterates through a data stream to extract values from a table and update a running total based on specific byte values.
- `accumulate_input_modifier_values`: Iterates through a sequence of command identifiers to calculate a cumulative offset.
- `process_event_loop_with_timeout`: Iterates through a table of function pointers and executes them within a nested loop controlled by a countdown timer.
- `update_global_long`: Stores a word into a global memory location and performs a short branch to another routine.
- `set_globals_and_call`: Stores a byte and a word into global memory locations and calls two subroutines before clearing a global word.
- `process_event_timer_and_callback`: Decrements a global counter and executes a function pointer when it reaches zero.
- `decrement_and_execute_timer`: Decrements a global counter value in a loop and executes a function pointer when the counter reaches zero.
- `update_status_bytes`: Updates a series of bytes in a global variable based on bitmask checks and specific character values returned from another routine.
- `copy_struct_and_find_end`: Copies several values from a source pointer into global variables and then iterates through a pointer-based list until a terminator is found.
- `cache_table_elements`: Copies several fields from a source structure into a series of global variables.
- `cache_joystick_data_table`: Copies a block of data from a source pointer into several global variables and then iterates through a list to find a sentinel value.
- `empty_data_area`: This routine contains a single instruction that performs a logical operation on a data register.
- `empty_data_area_2`: This routine contains four identical instructions that perform logical operations on a data register.
- `null_operation`: This routine performs a logical OR operation on the least significant byte of a data register with zero.
- `copy_struct_and_find_end_2`: Copies several fields from a source pointer into global variables and then iterates through a pointer-based list until a terminator is found.
- `update_joystick_bitstream`: Updates a bitstream by extracting bits from a source table and shifting them into a destination buffer.
- `configure_blt_hardware`: Stores several values into memory locations and enters infinite loops while waiting for a bit in a global variable to change.
- `initialize_registers_and_branch`: Clears register d0, sets register d1 to zero, and performs a short branch to another location.
- `initialize_registers_and_branch_2`: Clears register d0, sets register d1 to all ones, and performs a short branch to another location.
- `initialize_registers_and_branch_3`: Sets register d0 to all ones, clears register d1, and performs a short branch to another location.
- `traverse_data_table`: Calculates an index into a table and iterates through it, modifying a counter based on various encoded byte values.
- `decrement_and_branch`: Decrements two registers and performs a relative branch.
- `call_subroutine_wrapper`: Performs a branch to a subroutine and continues execution into the following code.
- `call_subroutine`: Performs a branch to another routine.
- `wait_for_non_zero`: Enters a loop that continues until a specific global variable is no longer zero.
- `empty_loop_routine`: The routine contains no reachable instructions and consists of an infinite loop that never executes any operation.
- `set_global_to_max`: Calls another routine and then stores a maximum value into the memory location pointed to by a global variable.
- `check_and_configure_blt`: Copies a value from a global variable to a register and performs several subroutine calls, including one that configures hardware registers and another that enters an infinite loop if a specific global variable is zero.
- `execute_conditional_branch`: Moves a value from a memory location to a register and performs a conditional branch or a subroutine call.
- `configure_blt_registers`: Checks a bit in a global variable and, if certain conditions are met, writes a series of values to a set of registers including BLT and DMA control registers.
- `execute_conditional_call`: Checks if a global function pointer is non-null and calls it if so.
- `execute_indirect_call_2`: Checks a global variable and performs a series of conditional calls before executing a function pointer retrieved from a table.
- `calculate_offset_from_table`: Loads two words from a table, multiplies one by eight, adds it to the other, and adds a constant offset.
- `call_other_routine`: This routine calls another routine and then returns.
- `recurse_without_side_effects`: The routine calls itself twice before returning, creating an infinite or unbounded recursive loop.
- `branch_to_offset`: Pushes a long value onto the stack and then performs a short branch to a different location.
- `call_empty_routines`: Calls two consecutive routines that perform no operations before returning.
- `save_registers_and_branch`: Loads a long value into a register, saves several word-sized registers to the stack, and branches to another location.
- `call_input_modifier_offset_calculation`: Calls a routine that calculates an offset by iterating through command identifiers and accumulating values from a table.
- `execute_empty_calls`: Calls two consecutive routines that perform no operations before returning to the caller.
- `clear_register_and_branch`: Sets a data register to zero and branches to another location.
- `validate_and_recurse_on_condition`: Checks if a value at a specific offset within a pointer-indexed table is negative.
- `call_and_return`: Calls another routine and then returns.
- `store_byte_and_branch`: Stores a byte from a data register into memory and branches to another location.
- `accumulate_table_pointers`: Iterates through a list of pointers while decrementing a counter and checking a value against a constant.
- `return_immediately_2`: The routine performs no operations and returns to its caller immediately.
- `process_complex_logic`: Performs conditional recursion, iterates through a table to find a specific entry, and executes several routines based on bitwise flags in the table data.
- `draw_sprite_quad`: The routine reads four 16-bit values from a structure pointed to by A5, loads a 16-bit value from D0, and calls another routine several times with those values and the A5 pointer.
- `setup_blitter_parameters_2`: Calculates source and destination addresses and certain blitter parameters, then writes them to hardware registers.
- `setup_blitter_parameters`: Calculates values and writes to several memory locations and hardware registers to prepare for a blitter operation.
- `set_blitter_registers`: The routine loads specific values into two registers and performs a conditional branch to set different values in those registers.
- `set_bob_dma_parameters`: The routine prepares a structure in memory for a blitter operation by writing colour and control words from tables and registers into fixed offsets of a structure pointed to by a register, and it sets a global variable and waits for a hardware flag before each write.
- `process_data_table`: The routine updates global variables, iterates through a table of values, and calls another routine repeatedly using a decreasing counter.
- `format_status_string`: Converts a value into a string by checking a global variable and writing specific character sequences to a buffer.
- `format_numeric_string`: Converts a number into a string of ASCII digits by calculating a quotient and remainder, then storing the resulting characters in a buffer.
- `wrap_numeric_string`: Calls a routine to format a number and then writes parentheses characters to a buffer.
- `clear_buffer_via_formatting`: Calls a routine to format a numeric value into a string and then writes zero bytes to the resulting destination addresses.
- `clear_string_buffer`: Calls a routine to format a number and then writes zero bytes to a buffer.
- `null_terminate_buffer`: Writes null terminators to two memory locations via pointers returned by other routines.
- `format_spaced_string`: Calls multiple routines to write space characters and null terminators into a buffer.
- `execute_sequential_calls`: Calls another routine four times in succession.
- `write_right_parenthesis`: Writes a right parenthesis and a null terminator to a memory buffer.
- `process_data_sequence`: Calls three different routines and returns a concatenated result from two registers.
- `execute_sequence_of_calls`: Executes a sequence of five subroutine calls.
- `format_numeric_data`: Converts a number into an ASCII string and copies the resulting buffer to a destination.
- `load_pc_relative_address`: Loads the address of the next instruction into a register using PC-relative addressing.
- `copy_string_buffer`: Copies a null-terminated string from a source buffer to a destination buffer.
- `write_conditional_space`: Checks a return value from another routine and writes a space character to a memory buffer if the value is not one.
- `replace_char_with_space`: Checks if a character at a specific pointer location is a space or an open parenthesis, and if not, replaces it with a space.
- `apply_bitwise_or_sequence`: Performs a series of bitwise OR operations on a byte in a register using several constant values.
- `modify_register_byte_sequence`: Performs a series of bitwise OR operations on a single byte within a register.
- `tiny_ori_ori_ori_2`: Small routine of 3 instructions that merges 39 into d1, then merges 52 into d1, then merges 66 into d1.
- `apply_bitmask_sequence`: Performs a series of bitwise OR operations on a byte in a data register using a specific sequence of constant values.
- `update_global_value`: Calculates a difference between two memory locations and updates a global variable based on whether that difference exceeds a certain value.
- `format_match_statistics_string`: Constructs a formatted string representing match data by retrieving values from a data structure and converting them into ASCII characters.
- `write_space_byte`: Writes a space character to a memory location offset from a register.
- `load_address_and_branch`: Loads a relative address into a register and performs an unconditional branch.
- `copy_memory_block_5`: Copies a block of memory from a source address to a destination address using a loop.
- `process_bitstream_list`: Iterates through a list of pointers, extracts bits from a source, and updates a bitmask if certain conditions or bit values are met.
- `set_global_if_nonzero`: Checks if two registers contain non-zero values and, if so, sets a global variable to a constant value.
- `process_bitfield_table`: Iterates through two 32-bit bitfields to check specific bits and stores the resulting index and a status value into a table.
- `clear_memory_block`: Clears seven consecutive words starting from a specific offset in a memory block by setting them to zero.
- `clear_short_block`: Clears three consecutive words starting from a specific offset in a memory block by setting them to zero.
- `process_bitstream_list_2`: Iterates through a list of pointers to extract bits from a source, updates a bitmask, and performs a secondary pass to build a bitmask based on values found in the list.
- `dispatch_indirect_call_2`: Executes several subroutines and then performs an indirect call using a pointer retrieved from a table based on a value in a register.
- `update_memory_pointers`: Sets several memory locations to specific values and enters a loop that calls another routine repeatedly based on a global variable's state.
- `apply_bitwise_or_sequence_2`: Performs a series of bitwise OR operations with specific byte constants on a register.
- `tiny_ori_moveq_move`: Small routine of 3 instructions that merges 101 into d3, then loads 116 into d4, then copies d0 to -(a1).
- `copy_memory_block_3`: Copies a block of memory from a source address to a destination address using a counter-based loop.
- `set_routine_table_entries`: Assigns several routine addresses to a set of global memory locations.
- `assign_indirect_jump_targets`: Assigns routine addresses to a set of global memory locations and updates a global variable.
- `execute_recursive_calls`: Checks if a value equals a specific constant and then calls another routine three times.
- `branch_if_non_zero`: Checks if a global variable is non-zero and, if so, executes an indirect call to a target address stored in a jump table.
- `update_input_event_buffer`: Iterates through a list of input event pointers to populate a structured buffer with event data and status flags.
- `update_input_state_and_flags`: Processes input events by iterating through a list of pointers to check bitmasks against global state and updating status flags.
- `compare_pointer_to_global`: Compares a register containing a pointer to a global variable and returns if they are equal.
- `update_state_and_jump`: Performs conditional logic to update a global variable and then executes an indirect jump.
- `perform_zero_or_operation`: Performs a bitwise OR operation on a register with zero.
- `clear_data_register_2`: Sets the least significant byte of a data register to zero.
- `clear_data_d0`: Performs a logical OR operation on register d0 with zero.
- `update_status_and_toggle_flag`: Updates several global status variables and flags based on the state of a conditional pointer list and a global variable.
- `update_struct_and_copy_memory`: Modifies specific fields within a data structure, including an offset and several byte values, before performing a memory block copy from one location to another.
- `update_table_values`: Iterates through a table to write specific values and performs arithmetic or conditional updates on table entries based on a global flag.
- `populate_string_table`: Iterates through a loop to populate a table with various values, including character increments and bit-shifted data.
- `process_state_conditional_logic`: Evaluates a value against multiple constants to determine a return status or execute specific data manipulation.
- `null_operation_2`: Performs a logical OR operation on a data register with zero, resulting in no change to the register's value.
- `merge_bitfield_values`: Extracts bitfields from two word-sized values and recombines them by masking and ORing the components.
- `extract_masked_word`: Fetches a word from a table using an index and performs bitwise masking and shifting to return a calculated value.
- `find_byte_in_table`: Searches a table for a specific byte and calculates a new address or offset if a match is found.
- `update_conditional_pointer_list`: Iterates through a list of pointers to check specific bitmask conditions and, if met, updates a secondary list with new pointer values.
- `filter_and_swap_color_attributes`: Iterates through a list of data structures to identify entries meeting specific bitmask and value criteria, subsequently populating a temporary table with selected values.
- `scan_list_for_condition`: Iterates through a list of pointers, checking if a value is non-negative and executing a routine until a certain condition or pointer or null is met.
- `swap_color_attributes`: Swaps the lower five bits of two different word values located at specific offsets from two provided pointers.
- `swap_color_attributes_2`: Swaps the lower ten bits of two different word values located at specific offsets from two provided pointers.
- `do_nothing`: This routine is empty and performs no operations before returning to its caller.
- `load_immediate_values`: Performs a series of logical OR operations to load specific byte values into register d1.
- `populate_data_register`: Performs a series of bitwise OR operations to set specific bits within a single byte in a data register.
- `copy_string_until_space`: Copies a string from a source address to a destination, stopping when a space character is encountered, and potentially null-terminates a preceding byte.
- `update_game_state_and_format_strings`: Updates several global state variables and performs conditional logic based on specific flag values.
- `sync_global_table_entry`: Iterates through a table of global variables to find a matching value, then updates two consecutive memory locations using the found value and the next entry in the table.
- `load_immediate_and_branch`: Loads a constant value into a register and branches to another location.
- `calculate_bit_mask_limit`: Checks a bitmask against a range of indices and calculates a new limit or pointer value based on certain bit conditions.
- `shuffle_memory_block`: Performs a nested loop to swap bytes between two memory regions.
- `shuffle_memory_block_2`: Performs a nested loop to swap bytes between a calculated memory location and a fixed memory region.
- `update_counter_via_bitmask`: Modifies a global variable by incrementing or decrementing it based on specific conditions and bitmask checks within a loop.
- `calculate_coordinate_flags`: Processes values from a pointer to determine bitwise flags and performs arithmetic on coordinate pairs before returning a combined result.
- `process_input_state_and_update_flags`: Evaluates input event data by checking bitmasks against global state and updates status flags.
- `process_input_event_list`: Iterates through a list of input events, checking bitmasks against global state and updating a status flag.
- `compare_and_branch`: Compares a register to a constant and branches based on the result.
- `process_input_event_list_2`: Iterates through a linked list of input events, checking bitmasks against global state and updating status flags.
- `copy_memory_range`: Copies a range of bytes from a source address to a destination address, then decrements a global counter.
- `empty_instruction`: The routine contains a single instruction that performs a bitwise OR on a data register with zero.
- `nop_padding`: This routine consists of a single instruction that performs a logical OR operation on a data register with zero, effectively acting as a no-operation.
- `process_table_dispatch`: Loops through subroutines while checking a register and a return value, then performs an indirect call using a table offset.
- `branch_to_subroutine`: Performs a relative subroutine call followed by an unconditional short branch.
- `execute_recursive_call`: Sets a boolean flag to false and calls itself; if the flag remains false after the call returns, the routine calls itself again.
- `update_bitstream_and_hardware`: Updates a bit-packed bitstream, configures hardware registers, and updates a global string buffer.
- `build_formatted_string`: Sets global parameters and formats a numeric value into a string before appending a constant string to it.
- `store_global_value`: Stores the value of a global variable into another global variable.
- `configure_hardware_registers`: Writes a series of values to a set of memory locations and enters loops that wait for a specific bit in a global variable to clear.
- `configure_hardware_and_globals`: Waits for a bit in a global variable to clear, then writes several values to a series of global memory locations.
- `clear_data_byte`: Performs a logical OR operation on a data register with zero.
- `clear_data_register_byte`: Performs a logical OR operation on the least significant byte of a data register using zero.
- `load_immediate_values_2`: Performs a sequence of bitwise OR operations to load several different constant values into a data register.
- `update_joystick_state_table`: Checks a value against a range and updates several global variables and memory table entries.
- `write_byte_to_table_list`: Iterates three times, writing a specific byte value to a memory location determined by adding an offset to a pointer retrieved from a table.
- `write_data_to_table`: Calculates an offset using a global variable and writes a series of specific byte values into a memory table.
- `return_immediately_3`: The routine is empty and performs no operations before returning.
- `write_array_elements`: Iterates through a pointer-based list to write a specific value to an offset of each element.
- `loop_with_subroutine`: Calls a subroutine within a loop that decrements a counter.
- `validate_and_sync_input_state`: Checks if a value falls within specific ranges to perform table lookups and updates global state variables.
- `load_and_branch`: Loads a constant value into a data register and performs a short branch to another location.
- `copy_and_format_string`: Searches for a character match in a buffer, updates a string, and performs several memory operations and hardware register initializations.
- `modify_global_values`: Sets certain global variables to zero and returns a decremented value.
- `copy_data_blocks`: Writes several multi-byte constants to memory and enters a loop that calls another routine until a specific condition is met.
- `process_conditional_loop`: Executes a loop that calls another routine and returns a value based on the results of those calls and a signed input.
- `parse_data_structure_2`: Validates a data structure by checking for specific magic constants, performs conditional subroutine calls, and initializes several global variables and memory locations.
- `execute_conditional_loop_2`: Checks a status flag and enters a loop that calls another routine repeatedly until a specific value is zeroed.
- `initialize_player_input_state`: Initializes a structured table of input data by iterating through a loop to populate entries with sequential values.
- `write_constant_to_indexed_list`: Iterates through a table of offsets and writes a constant value to a destination buffer using those offsets.
- `write_register_to_indexed_list`: Iterates through a table of offsets and writes a register value to a destination buffer using those offsets.
- `copy_and_increment_table`: Copies a block of data from a source table to a destination table while incrementing a counter and a value.
- `write_register_with_stride`: Writes a register value to a destination buffer at regular intervals determined by a stride.
- `swap_memory_blocks`: Swaps two blocks of memory byte by byte for a fixed length.
- `process_conditional_string_logic`: Updates values in a data structure based on a global flag and either copies a null-terminated string from a fixed location or writes a specific sequence of bytes to a pointer returned by another routine.
- `process_string_result`: loads a pointer into a register, calls another routine, and performs conditional checks on a data register before decrementing a pointer and returning.
- `evaluate_conditional_pointer`: loads a pointer into a register, tests a data register, and conditionally loads a different pointer or branches to another routine based on a memory value.
- `process_conditional_branch`: Moves a value, conditionally negates it, and branches to other routines or memory locations.
- `save_and_restore_data`: Saves values from a memory block, performs hardware configuration, and then restores the original values before an indirect jump.
- `initialize_routine_table`: Assigns several routine addresses and zero values to a table in memory and performs multiple subroutine calls.
- `clear_global_variables`: Sets several specific global variables to zero and potentially sets one to a constant if a condition is met.
- `clear_extended_global_variables`: Sets a larger set of specific global variables to zero and potentially sets one to a constant if a condition is met.
- `update_player_input_buffer`: Copies a string into a global buffer and iterates through a table to populate a structure with input data and status flags.
- `call_subroutine_via_offset`: Executes a branch to a subroutine using a word-sized relative offset.
- `unpack_player_attribute_bitfields`: Extracts packed attribute data from a source table using bitwise shifts and masks, then distributes the values into a destination buffer.
- `execute_nested_conditionals`: Executes a routine multiple times based on the return value of a previous call.
- `search_for_sentinel_bytes`: Iterates through a table looking for specific byte values, performing two separate searches for different sentinel bytes.
- `compare_and_zero_register`: Compares two bytes and, if they are not equal, sets a register to zero.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `evaluate_conditional_pointers`: Checks if certain bytes in a memory range are zero and executes an indirect routine if certain conditions or flags are met.
- `execute_indirect_call`: Performs an indirect call to a function pointer based on a condition involving a register.
- `unpack_bitfield_data_table`: Extracts bit-packed values from a source table and distributes them into a destination buffer.
- `copy_source_to_destination`: Copies words from a source table to a destination buffer until a zero-word is encountered, then pads the remaining destination bytes with zero.
- `copy_string_with_padding`: Copies a string from a source to a destination and pads the remaining destination bytes with null values.
- `store_constant_and_byte`: Writes a constant byte to a memory location pointed to by a register and copies a byte to another memory location.
- `update_table_from_index`: Checks a value in a table, and if it matches a constant, calculates a new index and stores a value from a secondary table into a destination offset.
- `modify_register_and_test_bit`: Performs a bitwise OR on a register and tests a specific bit in another register.
- `initialize_joystick_buffer_structures`: Clears and initializes multiple memory structures by iterating through a list of pointers and writing specific byte patterns.
- `iterate_and_test_bit`: Iterates through a list of pointers while checking a bit in a register based on a value from a dereferenced pointer.
- `update_pointer_list`: Iterates through a list of pointers, clearing a specific bit in the value at a certain offset or updating a global variable based on a signed input.
- `check_pointer_value`: Checks if a value at a pointer location equals a constant and potentially calls another routine based on a bitmask check.
- `modify_state_and_pointer`: Updates a bitfield in a data structure and stores a value from a table into a global variable based on conditional logic.
- `update_global_state`: Checks a global variable and writes a series of constant values to several memory locations based on whether that variable is zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_interrupt_dma`: The routine configures hardware by writing to CIA, DMACON, INTENA, and COLOR00 registers.
- `store_global_constant`: Stores a specific constant value into a global variable and returns the contents of registers D0 and D1.
- `set_magic_constant_global`: Writes a specific 32-bit integer constant to a global variable.
- `set_global_constant`: Writes a specific 32-bit constant value to a global variable.
- `store_constant_return_stack`: The routine stores a constant value to a global variable and returns a 64-bit value retrieved directly from the stack.
- `return_stack_value`: Returns a 64-bit value retrieved directly from the stack.
- `initialize_soccer_globals`: Checks a global flag and, if set, clears it and executes a sequence of calls involving a jump table and a decrementing loop.
- `calculate_memory_bounds`: Initializes several global variables to zero.
- `remove_matching_table_entry`: Iterates through a table of structures to find an entry with a matching string.
- `compare_and_remove_entry`: Iterates through a table of structures, comparing strings at specific offsets.
- `shift_table_entries_down`: Performs a block copy of data from a source pointer to a destination pointer within a table structure.
- `return_stack_value_2`: Returns the 32-bit value located at the current stack pointer.
- `add_entry_to_table`: Searches a string table for a match.
- `search_string_table`: Iterates through a table of strings to find a match for an input string; if found, it loads two pointers from the associated table entry and jumps to the first pointer.
- `lookup_string_in_table`: Iterates through a table of structures, comparing an input string against strings stored in the table.
- `check_and_load_table_byte`: The routine checks a global flag and returns early if set, otherwise it calculates an address from a global variable and loads two bytes from a specific offset into global variables.
- `configure_display_colors`: Sets the COLOR00 hardware register and updates global variables by calling helper routines.
- `load_byte_from_table`: The routine calculates a memory address using a global variable and a table offset, then loads two bytes from that location into global variables.
- `jump_to_vector`: Unconditionally jumps to a fixed address defined by a jump table entry.
- `parse_ibm_bitmap_header`: Reads and validates IBM bitmap file headers by checking for FORM, ILBM, BMHD, and CMAP magic numbers.
- `decode_ibm_bitmap_chunks`: Parses IBM bitmap file structures by validating FORM, ILBM, BMHD, and CMAP magic numbers.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `wait_for_credit_flag`: References the text string 'credit3' and initializes global variables.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `loop_until_condition_met`: Iterates by calling another routine and incrementing a counter until a global variable becomes non-zero or the counter reaches four.
- `initialize_global_values`: Sets two global variables to zero and conditionally assigns a third global variable a value from a register if that register is non-negative.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `tiny_tst_rts`: Small routine of 2 instructions that tests d0, then returns.
- `reset_ciab_control`: Calls a routine to test a register and return, then invokes a routine to clear the CIAB control register.
- `load_from_calculated_offset`: The routine checks if a global variable is invalid, calculates an offset, and loads bytes from a memory location into a global variable.
- `verify_sos6_signature`: The routine checks for the 'SOS6' signature in a memory structure and validates associated values.
- `check_sos6_signature`: The routine decrements a counter and checks a memory structure for the 'SOS6' signature and matching values.
- `reset_ciab_control_registers`: Invokes a routine to clear the CIAB control register and then calls a helper routine that tests a register before clearing the CIAB control register again.
- `clear_global_variable_and_call`: Sets a global variable to zero and invokes another routine.
- `initialize_dma_and_ciab`: The routine configures hardware by writing to DMACON, DSKLEN, and CIAB registers to enable DMA and set interrupt settings.
- `reset_ciab_and_set_flag`: Calls a routine to clear the CIAB control register and then writes a constant value to a memory location offset from an address register.
- `configure_ciab_interrupts`: Writes constant values to the CIAB control register and the interrupt enable register to configure hardware interrupt settings.
- `clear_ciab_control`: Writes zero to the CIAB control register to reset its configuration.
- `initialize_hardware_registers`: Writes several constant values to global variables and modifies hardware interrupt enable and timer registers to configure the system state.
- `initialize_hardware_state`: Writes several constant values to global variables and modifies hardware interrupt enable and timer registers.
- `configure_ciab_interrupt_loop`: Writes constant values to global variables and hardware interrupt registers to configure the system state.
- `call_empty_stub`: The routine executes three instructions and calls an empty stub routine that performs no operations.
- `empty_stub_2`: This routine is empty and performs no operations.
- `check_flag_and_clear_register`: Checks if a specific bit is set in a global variable and returns early if it is.
- `clear_ciab_control_register`: Sets a global variable to zero and invokes a helper routine that tests a register before clearing the CIAB control register.
- `set_ciab_port_bits`: Checks a global variable and, if zero, performs a series of assignments to global variables and calls another routine to modify the CIAB port B register.
- `decrement_global_to_zero`: Initializes a global byte to ten and decrements it in a loop until it reaches zero.
- `check_joystick_and_update`: Checks a global variable and a memory value before updating a global variable and calling another routine, looping while a specific bit in a global variable is set.
- `unroll_bit_manipulation_loop`: Uses an unrolled loop to perform bitwise masking and bit-shifting on a block of memory, accumulating certain values into a register.
- `call_ciab_reset`: The routine invokes a helper to test a register and then calls another routine to clear the CIAB control register.
- `call_reset_ciab_control`: Invokes the routine to reset the CIAB control register.
- `call_reset_ciab_control_2`: Invokes a routine to clear the CIAB control register.
- `clear_ciab_control_2`: Invokes a helper routine to test a register and then calls a routine to clear the CIAB control register.
- `update_input_state`: Checks a bit in a global variable and updates it or calls another routine based on specific bitwise conditions.
- `process_input_bits`: Performs bitwise manipulations on a global variable and stores the result into a table if a specific condition is met, then writes a value to a memory location.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `process_interrupt_requests`: The routine manages interrupt request flags by reading and writing to custom chip registers, specifically INTREQ and INTREQR. It coordinates with several helper routines to update global state variables, shift buffer elements, and handle audio volume scaling based on hardware status.
- `shift_buffer_elements`: Shifts a series of bytes stored in a buffer while decrementing a counter, and checks a value in a buffer or a global variable.
- `setup_dma_and_copper`: Copies data from a global variable to a fixed memory address, waits for a condition, writes values to a table, clears memory regions, and configures DMA and copper registers.
- `return_immediately`: The routine is empty and returns immediately without performing any operations or modifying data.
- `initialize_memory_pointers`: Assigns specific constant values and routine addresses to global variables.
- `populate_memory_tables`: Populates a series of memory addresses with incrementing or fixed values and updates several global variables.
- `modify_stack_flags`: Sets specific bits in a word and a byte located on the stack relative to the address register, then writes several values to global variables.
- `modify_stack_flags_2`: Sets specific bits in two bytes located on the stack relative to the address register.
- `set_byte_and_globals`: Sets specific bits in a byte located on the stack relative to an address register and assigns several values to global variables.
- `empty_stub_3`: This routine is empty and returns immediately without performing any operations.
- `wait_and_clear_interrupts`: Waits in a loop until a specific bit condition is met in a global variable.
- `decrement_register_to_zero`: Multiplies an input value by ten and decrements the result in a loop until it reaches zero.
- `check_joy_and_buttons`: The routine reads the joystick port movement register and the CIAA port A register, then performs a series of bitwise tests and assignments on a global variable, ending with a conditional write of a constant to that variable.
- `process_joystick_input`: Reads from the CIAA_PRA and JOY0DAT hardware registers to detect joystick and mouse states.
- `set_global_flag_2`: Checks if a global variable is non-zero and sets another global variable to a negative value or constant accordingly.
- `set_global_flag`: Checks if a global variable is non-zero and sets another global variable to a negative value or constant accordingly.
- `loop_until_zero`: If a global variable is non-zero, it sets that variable to a constant and loops while it remains non-zero.
- `extract_bit_pattern_loop`: Iterates sixteen times to extract values from a memory table using a bitmask and shift amount, storing the results into a global variable and a fixed memory address.
- `store_pattern_to_memory`: Writes a pattern with a specific bitmask to a memory table using a pointer.
- `scale_table_index_by_factor`: The routine reads a byte from a pointer, masks the low four bits, multiplies the result by an unsigned word from D2, and returns a 32-bit value formed by the high 16 bits of the product and the product shifted right by seven bits.
- `reset_global_accumulator`: The routine checks a global variable and, if it is not a sentinel value, clamps a negative value, recurses into another routine, then clears the variable and, if the result is negative, writes a sentinel and zero to the global variable.
- `clamp_and_double_global`: The routine checks a global variable and, if it is not equal to a sentinel value, clamps it to zero when negative, recursively calls another routine, doubles the variable, and if the result exceeds a threshold it sets the variable to the sentinel value and then to one.
- `copy_and_or_value`: Copies a word from a global variable to a location on the stack.
- `wait_and_set_flag`: Loops while a global variable is non-zero.
- `tiny_ori_2`: Small routine of 1 instruction that merges 7 into d0.
- `wait_and_clear_flag`: Loops until a specific global variable becomes zero, then sets another global variable to zero.
- `check_and_decrement_counter`: Checks if a global variable is non-zero and if a bitwise operation on a memory location results in zero, then decrements a counter variable.
- `apply_volume_scaling`: The routine scales a global variable by four successive table lookups, each multiplied and shifted right by six, and returns the combined input registers.
- `initialize_audio_state`: Sets a global variable to a constant and stores an input register value.
- `reset_audio_dma_state`: Initializes audio state variables, calls a routine to set up audio channels, clears several global variables, and writes to audio volume and DMA control registers.
- `decrement_counter_with_wrap`: The routine calls another routine, then checks a global variable for a flag.
- `empty_stub_return`: The routine is empty and returns immediately without performing any operations.
- `do_nothing_stub`: The routine is empty and returns immediately.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `find_max_and_clear_audio`: It scans a byte table to find the maximum value, then uses that value to calculate an offset into a global structure where it zeroes out a linked list of pointers and resets several global variables to zero.
- `find_max_byte_and_init_table`: Scans a memory region to find the maximum byte value, then uses that value to calculate an offset into a lookup table.
- `clear_audio_dma_buffers`: Iterates through a linked list of audio DMA buffers to zero out their data and reset pointers.
- `init_player_team_data`: Calculates a team index based on a character value and initializes a linked list of player data structures.
- `clear_audio_dma_buffers_2`: Iterates through a linked list of audio DMA buffers to zero out their data and reset pointers.
- `find_max_byte_and_init_table_2`: Scans a memory region to find the maximum byte value, then uses that value to calculate an offset into a large table.
- `clear_audio_dma_buffers_3`: Iterates through a linked list of audio DMA buffers to zero out their data and reset pointers.
- `initialize_audio_channels`: The routine walks a byte table derived from another routine and, when the walk ends, clears a block of pointers in a table, zeroes several global variables and two other routines, and writes zero to four audio volume hardware registers.
- `reset_audio_dma`: The routine clears four audio volume registers and four two-byte globals, then writes a value to a global variable and returns.
- `process_audio_dma_commands_2`: The routine increments a global counter, initializes audio state variables, and processes DMA command bytes to configure audio channels and update hardware registers.
- `process_audio_dma_commands`: Reads command bytes from a structure to configure DMA audio channels, updating global state and hardware registers based on specific opcodes.
- `enable_dma_channel`: Loads a word from memory, sets its highest bit, writes it to the DMA control register, and calls another routine.
- `initialize_audio_buffers`: Copies several memory addresses into a set of destination locations and increments a counter, potentially resetting it based on a comparison.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `process_entity_state_flags`: Reads a status word from a structure pointer, masks it, and branches on the lower nibble to update fields, check thresholds, or modify counters.
- `set_flag_and_call_routines`: It sets a global variable to three, clears another global variable, calls several other routines, and conditionally loops on a routine call if a local flag remains false.
- `empty_stub_placeholder`: The routine is empty and contains no executable instructions.
- `set_flag_and_loop`: Sets a global variable to 3 and a secondary variable to 0, then calls another routine.
- `set_global_three_and_loop`: Sets a global variable to 3 and a flag to 0, then calls another routine.
- `set_global_constant_and_loop`: Writes a constant value of 3 to a global variable and sets a flag to zero.
- `initialize_game_loader_sequence`: Sets up global state variables and initializes audio DMA hardware.
- `decode_ibm_bitmap_header`: This routine processes IBM bitmap data structures by validating magic numbers and parsing header information.
- `initialize_audio_dma`: The routine sets global variables to zero and calls other routines to configure system state.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_11`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_12`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_13`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_14`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_15`: Empty routine: returns to the caller immediately without doing anything.

### doscode (raw code file, load address unknown)

- `dispatch_decompression_task`: This routine acts as a dispatcher that branches based on an input value to call either decompression routines or initialization helpers.
- `decompress_block_data`: Decompresses a data block by reading length and offset values from a source pointer and copying bytes to a destination, handling both repeat and literal copy operations.
- `copy_compressed_block`: Decompresses a data block by reading length and offset values from a source pointer and copying bytes to a destination, handling both repeated byte sequences and backward references.
- `decrement_counter_and_call`: The routine checks a counter and a flag in a structure; if the counter is zero and the flag is set, it calls another routine and returns its result, otherwise it decrements the counter and returns zero.
- `initialize_disk_transfer_block`: The routine initializes a memory block for disk operations by clearing a buffer, copying a path string, and setting up control fields.
- `clear_restore_field_sum`: The routine clears a field at offset 20 in a structure, calls another routine to sum an array of words, stores the result at that offset, and restores the field.
- `process_disk_path_segments_2`: Checks if a pointer is null and returns early if so.
- `set_flag_in_bitfield`: The routine computes an index from the high and low parts of an input value, selects a 32-bit word from a table pointed to by A6, and sets a single bit within that word corresponding to the low bits of the index.
- `decrement_loop_until_match`: The routine decrements a short value and repeatedly calls another routine with the updated value until it equals a specific constant, then returns.
- `wait_for_register_value`: Calls a routine once, then enters a loop that repeatedly calls another routine, checking if a short register value equals a specific constant until the condition is met.
- `combine_registers_into_return_value`: The routine receives two 32-bit values in registers and returns a 64-bit value formed by concatenating the first value as the high part and the second value as the low part.
- `clear_bit_in_table`: The routine clears a specific bit in a 32-bit word table indexed by a value in D0.
- `sum_array_or_return_constant`: Checks a condition and calls a routine to sum an array of words; if the zero flag is not set, it returns a constant value instead of the sum.
- `process_disk_path_segments`: Checks if a pointer is null and returns early if so.
- `empty_routine`: The routine consists of seven instructions that perform no observable work; it does not access memory, registers, or call other routines, and ends with a return.
- `copy_path_segment_to_buffer`: The routine finds a slash character in a string, copies characters to a buffer, and computes a return value based on the processed data.
- `check_extraout_flag`: The routine calls another routine and stores its result, then tests a flag and two fields of a pointer; if they match, it overwrites a global variable with a value from memory and returns zero, otherwise it returns the stored result.
- `sum_array_or_return_constant_2`: Calls a routine to sum an array of words and returns the result, or returns a constant if the zero flag is not set.
- `sum_words_or_return_constant`: The routine calls another routine to sum an array of words and returns the result, or returns a constant if the zero flag is not set.
- `sum_negate_array_words`: The routine adds 128 consecutive 16-bit words pointed to by a pointer in A0, accumulating the sum in a register.
- `empty_routine_2`: The routine is empty, containing only a return instruction with no observable work performed.
- `clear_and_restore_field_0x14`: Clears a 4-byte field at offset 20 in a structure pointed to by register A0, calls another routine, and then restores that field with the returned value before calling the routine again.
- `clear_and_restore_field_0x14_2`: Zeros a 32-bit value at offset 20 from a base pointer, calls another routine, and then restores that memory location with the returned value before calling the routine again.
- `clear_and_restore_offset_20`: Clears the long word at offset 20 in the A0 register, calls another routine, and then restores the value returned by that call to the same offset before calling the routine again.
- `clear_store_sum_result`: The routine clears a long word at an offset in a pointer, calls another routine to sum an array of words, stores the result at the same offset, and calls another routine.
- `recursive_call_stub`: The routine calls itself recursively without performing any other operations or modifying memory.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d2.
- `tiny_moveq_2`: Small routine of 1 instruction that loads 0 into d3.
- `calculate_table_index`: The routine performs arithmetic on input registers to compute indices and offsets, likely for table lookups or memory addressing.
- `initialize_input_and_clear_tables`: The routine loops to clear table entries and validate input flags using CIA timer checks.
- `wait_cia_timer_flag`: The routine sets a flag, checks a stack byte, and repeatedly calls helpers to configure CIA timer registers and update custom chip registers until a counter expires or a condition is met.
- `render_park_view`: The routine writes a constant to a memory location derived from a pointer, then repeatedly calls other routines to update a flag, toggle a flag, and compute a checksum using a decrementing counter.
- `validate_input_with_retry`: The routine repeatedly calls helpers to check a flag and decrement a counter, returning early if the flag is clear or the counter expires; if the loop completes, it validates register values against memory, updates a short variable, and returns a status code.
- `validate_input_with_cia_timer`: Loops while checking flags and counters, calling routines to toggle bytes and mix words.
- `empty_stub_routine`: The routine consists of five instructions that perform no observable work and then return.
- `set_flag_and_increment_counter`: The routine sets a bit in a 16-bit word located a few bytes before a pointer in A6, using a bit index derived from D3.
- `clear_table_entries`: The routine clears a table by writing zero to successive entries.
- `extract_high_byte_pair`: The routine calls another routine to obtain a 32-bit value and returns a new 32-bit value formed by the low 16 bits of the result in the low half and the original high 8 bits in the high half.
- `interleave_word_bits`: The routine reads two consecutive words from a pointer in A0 and returns a value formed by taking the low bits of the second word and the low bits of the first word shifted left by one.
- `apply_interleave_xor_mask`: The routine calls a routine to interleave word bits and obtains a 32-bit value, then passes that value to a second routine that performs an XOR sum and mask operation.
- `compute_fixed_xor_checksum`: The routine iterates through a fixed sequence of nine words starting from an offset in a structure, XORing each word into an accumulator.
- `compute_variable_xor_checksum`: The routine iterates through a sequence of words pointed to by an input pointer, XORing each word into an accumulator.
- `fill_memory_pattern`: The routine clears a block of memory by writing a repeated 32-bit pattern to each word.
- `interleave_word_streams`: The routine interleaves the low and high bits of two 32-bit word streams pointed to by two input pointers.
- `loop_toggle_and_mix_flags`: The routine loops while decrementing a counter, calling routines to toggle a byte flag and mix/store a word, then calls two more routines and returns a concatenated value.
- `adjust_byte_flag_state`: The routine reads a byte pointed to by a pointer and a flag from the preceding byte.
- `toggle_byte_flag_conditional`: Reads a byte via a pointer and checks a flag in the preceding byte.
- `bitwise_mix_store_word`: The routine mixes the input word with a transformed mask derived from the low bits of the input, optionally clears the sign bit based on a flag byte before the output pointer, and stores the resulting word through the pointer.
- `set_audio_dma_control`: The routine writes to a global variable, another global variable, a custom chip register, a custom chip register, a global variable, and a global variable.
- `configure_cia_timer_loop`: Checks a global flag and input register, then writes constant values to CIA timer hardware registers in a loop until a specific bit condition is met.
- `set_adkcon_bit_conditional`: The routine sets a global variable to a constant, and if a short value is negative, it computes a bit mask from a table and a constant, then updates the global variable with the inverted mask.
- `set_bit_in_global_flag`: The routine reads a 16-bit value from a table pointed to by a register, adds a constant, masks the result to 5 bits, and uses it as a bit index.
- `set_cia_port_and_wait`: Sets a global variable to a specific value, calls a routine to configure CIA timer registers, and waits in a loop until a bit in the global variable is clear.
- `update_table_entry_with_mask`: Reads a word from a global table using an index, conditionally calls another routine if the value is negative, and loops while decrementing a counter.
- `wait_for_input_flag_clear`: Polls a global variable flag in a loop, decrementing a counter until the flag is cleared or the counter expires.
- `mask_input_and_wait_cia`: Builds a mask from a global variable and a byte from a structure, clears a bit in the mask, and waits in a loop until a bit in the global variable is clear.
- `store_masked_input_byte`: The routine calls another routine to mask an input byte and stores the result in a global variable.
- `mask_input_byte`: The routine builds a mask from a global variable and a byte taken from a structure, clears one bit in that mask based on a byte from the structure, and optionally clears another bit if a flag in a table indexed by the structure is set.
- `wait_for_cia_timer_flag`: Repeatedly calls a routine to configure CIA timer registers until it returns a success value.
- `set_cia_timer_constants`: Checks if a global flag is clear and an input register is not one; if so, it writes three specific constant values to a global variable in sequence, which corresponds to setting CIA timer registers.
- `write_ciab_timer_values`: The routine writes three specific constant values to the CIAB_CRA, CIAB_TAHI, and CIAB_TALO hardware registers in sequence.

### soccer (raw code file, load address unknown)

- `empty_soccer_entry`: The routine is empty, consisting of a single instruction at the start of a raw code region with no further operations.

### soccerc (raw code file, load address unknown)

- `init_code_region`: The routine is empty, containing no instructions.

### soccerf (raw code file, load address unknown)

- `mark_code_region_start`: This routine consists of a single instruction at the beginning of a raw code region.

### soccers (raw code file, load address unknown)

- `init_code_region_2`: The routine contains a single instruction at the start of a raw code region.

### socceru (raw code file, load address unknown)

- `empty_routine_placeholder`: The routine is empty and contains no instructions.
