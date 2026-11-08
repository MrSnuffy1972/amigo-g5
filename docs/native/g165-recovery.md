# Game 165 (GridStart v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00165). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1603639 top-level bytes in 38 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 30 raw-word lines |
| Functions named and described | pass, 250 of 250 named (17 author symbols), 250 of 250 described (20 by rule, 230 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5933 of 5937 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 606107 |
| code | 24546 |
| data | 972665 |
| padding | 321 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| GridStart | 35 | 5935 | 249 |

## Cases handled

- GridStart_v1.0/GridStart_v1.0.hdf: hard-disk image (AmigaDOS OFS, 1792 blocks): its filesystem parses and all 36 files in it are byte-identical to the extracted tree GridStart_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_hardware_and_copy` (GridStart): Copies 20 bytes from a global table to a destination pointed to by a1, modifies a byte at offset 32242 of a hardware register using a bitwise AND, sets several hardware registers to specific values, and writes a bit-shifted value to a destination in a6. Kind: disk hardware/track check. Pass path: the routine continues to load the game data after the hardware/track validation succeeds.
- `update_hardware_status` (GridStart): Compares the stack pointer with a global variable and branches to another routine if it is less. Kind: disk hardware/integrity check. Pass path: the loop continues or the routine proceeds to the next instruction after the call (u1__sub_2_0570/u1__sub_2_0604).
- `process_input_delta` (GridStart): Calculates differences between values stored in a structure and updates a CIAA register based on those differences. Kind: track/sync check. Pass path: uVar2 == param_2.
- `set_hardware_bit_and_call` (GridStart): Reads a byte from a hardware register, clears its lowest bit, sets its lowest bit, and then calls another routine with a constant value. Kind: disk hardware/sync check. Pass path: the branch following the call (the code continues to the next instruction after the stack cleanup).
- `modify_hardware_byte` (GridStart): Sets bit 7 of a hardware register at offset 32753, then performs bitwise operations using a byte from a6 and a bit-shifted byte from offset 1 of a6 to update the same hardware register. Kind: checksum/bit-manipulation trap. Pass path: the branch where DAT_00bfd100 is not modified by the specific bitwise logic.
- `set_hardware_bits` (GridStart): Sets bits 6 and 7 of a hardware register at offset 32753 by performing an OR operation with 120. Kind: disk hardware/track check. Pass path: the instruction immediately following the call (e.g., addq.l #an address, a7).

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### GridStart

- `initialize_system_and_task_environment`: Initializes several global variables and library pointers, then calls FindTask to retrieve a task structure.
- `dispatch_cleanup_callbacks`: Executes a series of conditional function pointers stored in a structure, including a primary callback, a secondary callback, and several optional cleanup routines, before calling a routine to free linked memory blocks and another to manage system state and hardware.
- `invoke_two_function_pointers`: Loads a pointer from an offset of 92 from a base register into a0, calls a function pointer at a negative offset from a6, loads the same pointer into a0, calls a second function pointer at a different negative offset from a6, then returns.
- `call_dos_function_and_store_result`: Loads a pointer into a1, calls a function via an offset in a6 with zero in d0, stores the returned value into a global variable, and either branches to another routine or returns.
- `_AllocMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `_FreeMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `_FindTask`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `_SetSignal`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `_OpenLibrary`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `validate_and_store_library_pointers`: Loads a library base address from a pointer, then sequentially calls a specific offset within that library using four different argument structures, storing each returned pointer into four consecutive global memory locations; if any call returns zero, it branches to a cleanup routine, otherwise it continues to another routine.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `clear_d0_byte`: Performs a bitwise OR on register d0 with zero.
- `zeroes_data_register_zero`: Performs a bitwise OR operation on data register zero with zero.
- `clears_data_register_zero`: Performs a logical OR operation on data register zero with zero.
- `manage_system_state_and_hardware`: Performs a series of conditional routine calls and hardware register updates, including setting audio volume and DMA control registers.
- `close_screens_and_cleanup`: Checks two global variables and calls a routine via a pointer in a library if they are non-zero, then performs several calls to other routines using pointers from a structure in a6 and a global variable.
- `update_hardware_and_counters`: Checks a bit in a global variable and, if set, modifies hardware registers at a fixed address and updates several global counters, including a signed counter and a value that is clamped when it exceeds 43.
- `calculate_offset_values`: Calculates a value based on a global pointer and an offset, then uses that to fetch data from a table to update two 16-bit values at offsets 12 and 20 of a structure pointed to by a0.
- `get_structure_pointer`: Calculates a pointer by adding a constant to a global variable, stores it at offset 72 of a structure, and returns either a truncated version of the pointer or a value of 65535 based on a comparison with a global variable.
- `update_structure_values`: Calls another routine, and if the zero flag is not set, clears a word at offset 70 of a structure in a0, then calculates and stores two words at offsets 12 and 20 of that structure using a value from a table at offset 27164.
- `set_structure_values`: Calls another routine, and if the zero flag is not set, stores the value 1 at offset 70 of a structure in a0, then calculates and stores two words at offsets 12 and 20 of that structure using a constant and a value from a table at offset 27164.
- `set_structure_values_two`: Calls another routine, and if the zero flag is not set, stores the value 2 at offset 70 of a structure in a0, then calculates and stores two words at offsets 12 and 20 of that structure using a constant and a value from a table at offset 28692.
- `set_structure_values_three`: Calls another routine, and if the zero flag is not set, stores the value 3 at offset 70 of a structure in a0, then calculates and stores two words at offsets 12 and 20 of that structure using a constant and a value from a table at offset 28692.
- `update_structure_with_clamped_index`: Updates a structure at offset 72 with a pointer, clamps an index using a value from a stack frame, and calculates two 16-bit values at offsets 12 and 20 of the structure using data from a table and values from the stack.
- `execute_library_calls`: Loads a library base address into a6, then performs two identical sequences where it sets d1 to 2 and d0 to 800, calls a routine at an offset from a6, and stores the result in a global variable before branching if the result is zero.
- `update_audio_volume_2`: Checks a global longword, calls another routine, and performs arithmetic to calculate a volume level, which is then clamped and stored in a global variable or a hardware register.
- `update_entity_list_and_structures`: Iterates through a list of entities, performing arithmetic on global counters and updating a structure's properties including several offset-based pointer or value fields.
- `initialize_structure_and_copy_list`: Calls a routine via a pointer, then if a register is non-zero, stores it in a global variable, sets a word at offset 0 of the structure to 10, sets a pointer at offset 4, clears offset 8, and enters a loop to add a constant to elements from a table and store them in a destination starting at offset 72 of the structure before clearing offset 70 and returning.
- `process_range_and_populate_list`: Sets a counter to 22, calls a routine, and if the result is non-zero, initializes a structure with a word at offset 0 and a pointer from a table, calls a range routine, and if bit 0 of the result is set, enters a loop to add a constant to elements from a table and store them in a destination starting at offset 72, then clears offset 70 and performs a loop 22 times to set specific values in the structure.
- `set_multiple_bits_d0`: Performs a series of bitwise OR operations on register d0 with various immediate byte values.
- `apply_bitwise_or_masks_to_d0`: Performs a series of bitwise OR operations on the byte in register d0 using a sequence of eight specific bitmask constants.
- `set_d0_values`: Performs a series of bitwise OR operations on register d0 using several immediate byte values and then returns.
- `check_keyboard_status`: Checks bit 6 of a global variable and bit 7 of the keyboard serial register; if specific conditions are met, it clears a long value at offset 4694 and a byte at offset 469c of a global variable.
- `handle_joystick_input`: Checks bits in a global variable, and if set, modifies a global longword, otherwise it reads a value from a hardware register, performs bitwise operations, and calls various routines based on the result.
- `update_audio_volume`: Reads a byte from a hardware register and a global variable, updates the global variable, and if both bytes have bit 7 set, returns 65535; otherwise, it calls a routine to calculate a value, and if certain conditions regarding global variables are met, clears a hardware register or updates a hardware register with a scaled value.
- `decrement_global_pointer`: Subtracts various large values from a long value pointed to by a global variable, clearing the value if the result is negative, and otherwise increments a 16-bit global variable and sets bit 0 of another global variable.
- `update_counters_on_input`: Checks bits in the CIAA serial register and the CIAA peripheral address register to decrement or increment various 16-bit global variables and set bit 0 of a global variable.
- `jump_to_address_2`: Loads a word into register d7 and a longword into d0, then jumps to an address in a table.
- `toggle_bitfield_bits_2`: Sets a value in register d7 and a base address in d0, then jumps to another routine.
- `jump_to_address_cleared`: Loads a word into register d7, clears register d0, and jumps to an address in a table.
- `initialize_bitfield_toggle_parameters`: Loads the constant 140 into register d7, clears register d0, and jumps to the toggle_bitfield_bits routine.
- `process_table_and_call_subroutines`: Iterates through a table, and if an element is non-negative, performs arithmetic to calculate values, then calls a subroutine four times with different parameters, and finally updates a global byte using a BCD adjustment if a specific bit is set.
- `calculate_offset_and_call`: Checks d6 and d2, and if d6 is non-negative and d2 is positive, performs several arithmetic operations including multiplication and bit shifts on d1, d5, and d6 before calling another routine and returning.
- `toggle_bitfield_bits`: Calculates a target memory address using a base pointer, an offset, and a signed count, then enters a loop to toggle specific bits within a bitfield using bitwise XOR operations.
- `modify_bitfield_data`: Uses a loop to toggle bits within a bitfield at a memory location calculated from a base address and a table of offsets, using bitwise XOR operations.
- `update_collision_flags`: Iterates through a list of structures to find a match based on specific field values and spatial bounds, then sets bit 0 of a byte at offset 10 of the matching structure.
- `update_offset_value`: Calculates a divisor using a global word and a value from a structure, compares it to a value at offset 28 of a0, and if the comparison passes, updates the value at offset 28 and clears bit 0 of a byte at offset 10 of a0.
- `add_square_root_to_offset`: Checks a global word, and if it is non-negative, loads two longwords from a structure, calls a routine to calculate a square root, and adds the result to a longword at offset 28 of the structure.
- `clamp_offset_value`: Compares a 16-bit value at offset 28 of a structure in a0 against a constant; if it is greater than or equal to that constant, it adjusts a 32-bit value at offset 16 of the structure by a fixed amount or resets it using a value from offset 48.
- `update_structure_offset_and_bounds`: Checks if a 16-bit value at offset 28 of the structure in a0 is greater than 50 and if a calculated value derived from a global longword and offset 60 of the structure is non-negative; if both conditions are met, it sets a 16-bit value at offset 48 of the structure based on a global sign bit, potentially sets a 16-bit value at offset 16, sets a 16-bit value at offset 52 to 100, and calls another routine.
- `process_structure_list`: Iterates through a null-terminated list of pointers, performing different operations based on the value at the start of each structure.
- `populate_interpolation_table`: Calculates and stores a series of interpolated values into a table based on a global index and a structure containing scale factors.
- `construct_command_buffer`: Calls another routine, clears several bytes in a hardware register, and populates a buffer with a sequence of words including calculated values, constants, and data from a table.
- `initialize_and_populate_data_table`: Initializes a table with several longwords, clears eight consecutive bytes in a hardware register area, and calls two other routines.
- `initialize_and_populate_data_table_2`: Initializes a table with several longword constants and clears eight consecutive longwords in a hardware register area.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `build_command_buffer_and_append`: Populates a table with several longwords, clears a sequence of bytes in a hardware register, calls two routines to generate data, calculates two values based on a global word and stores them in the table, and finally updates a global pointer to point to the next available entry in the table before calling another routine.
- `build_display_command_buffer`: Configures display registers, clears a memory area, and populates a buffer with a sequence of words including calculated values and pointers to data tables.
- `generate_data_table`: Loops four times to read a longword from a source table, adds it to a value in d0, and writes the high and low words along with an incrementing counter to a destination table.
- `fill_incrementing_table`: Loops 31 times to write an incrementing word starting from 384 to a destination table, while also copying words from a source table into the same destination.
- `calculate_and_write_buffer`: Performs arithmetic on a global longword to update a counter, then enters a loop to write a sequence of words to a buffer based on a calculated size and a specific pattern.
- `configure_blt_registers`: Checks a bit in a hardware register and then performs several sequences of writing values to Blitter registers and calling subroutines to configure DMA and bitfield parameters.
- `configure_blitter_hardware`: Waits for a bit to clear in a hardware register, then clears two hardware registers and sets the blitter width, source address, and control registers.
- `configure_blitter_with_d0`: Waits for a bit to clear in a hardware register, then clears two hardware registers, sets the blitter width, uses d0 as the source address, and sets the control registers.
- `setup_interrupt_handler`: Writes bytes to a specific memory range, loads a pointer into a6, calls a routine via an offset, clears a bit in a global variable, and calls another routine via an offset before performing an indirect jump.
- `set_audio_and_dma_registers`: Calls a routine via a pointer, writes several words to hardware registers at offset 142 of the hardware base address, sets and clears bits in two global variables, calls another routine via a pointer, and stores a longword from d0 into offset 128 of a structure in a0.
- `update_game_state_and_buffer`: Updates a global counter and a 16-bit value using a table of multipliers, then populates a structure in a0 with various calculated words and values.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `perform_logical_or`: Performs a bitwise OR operation on register d0 with zero.
- `update_list_and_blit_params`: Stores a pointer from d0 into a global variable, then if d0 is non-zero, it updates a structure at a1 with values from d1, d2, and d3, modifies several global variables, and calls another routine via a pointer before calling a second routine and returning.
- `initialize_and_update_table`: Writes the value 15 to six consecutive memory locations, then uses a value from a global variable to index into a table and write the value 2527 to a specific location.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 4 into d3, then merges 2 into d1, then merges 8 into d7.
- `execute_conditional_routines`: Calls several routines and sets bit 5 of a global variable, provided bit 7 of the CIAA peripheral register is not set, otherwise it returns.
- `update_game_state_and_display`: Clears specific bits in a global variable, updates a hardware register with a constant, and executes a sequence of conditional routines based on the state of a global variable and joystick input.
- `handle_joystick_input_2`: Checks bits 12 and 13 of the joystick status register and updates two global variables based on their state and a counter, then calls two routines via pointers and swaps two global longwords.
- `process_joystick_input`: Calls a routine, and if a global word is 1, checks bits in a hardware register to either decrement or increment a global word and call two other routines via pointers.
- `write_character_to_buffer`: Checks if a global word is equal to 64; if so, it writes the value 46 to a buffer at an offset determined by another global word, then calls two other routines via pointers.
- `update_buffer_from_table`: Copies a byte from a buffer to another location, clears a byte in a global variable, and then calls a routine via a pointer retrieved from a table.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_and_swap_pointers`: Increments a word in a global variable, loops until a value in a table is not negative one, calls two routines, swaps two longwords in a global structure, clears a word in a global variable, and returns.
- `update_blit_parameters`: Sets d0 to 1, then checks a long at a global address; if not zero, it sets d0 to 2, increments that global long, and compares it to d0, otherwise it clears the global long.
- `update_blit_parameters_fixed`: Loads a specific pointer into d0, sets d1 to 8, d2 to 1, and d3 from a global word, then jumps to another routine.
- `set_status_bit`: Sets bit 6 of a global byte.
- `update_blit_parameters_conditional`: Loads a pointer into d0, sets d1 to 8, d2 to 12, and clears d3, then if bit 6 of a global variable is set, sets d3 to 1 before jumping to another routine; otherwise it jumps to another routine.
- `increment_and_jump`: Calls a routine, compares a word in a global variable to two, increments it if it is not two, calls another routine, and jumps to a third routine.
- `check_and_update_byte`: Reads a byte from a table using an index from a global variable; if the byte is 46, it updates a global word to 64.
- `clear_and_call_routines`: Clears a word in a global variable, calls two routines via pointers, and then branches to another location.
- `loop_and_swap_values`: Sets a global word to 196, then enters a loop that calls two routines, swaps the values of two longwords in global variables, clears a bit in a global variable, and decrements the global word until it is no longer positive.
- `process_and_swap_longwords`: Calls a routine, calls another routine, swaps two longwords in a global structure, clears a word in a global variable, and returns if a value retrieved from the stack plus four is not greater than 192.
- `swap_pointers_and_loop`: Swaps the contents of two global longword pointers, clears a global word, and calls two other routines before incrementing a counter and looping if the counter is less than 197.
- `format_and_sort_list`: Checks a bit in a global variable, calls two routines, iterates through a list to calculate a value, writes that value to two tables, and calls a routine multiple times with various constants and values retrieved from a global structure.
- `process_string_table`: Loops through a table to find entries with values 1 or 10, then for each match, calls another routine twice while setting specific global pointers to a buffer address.
- `format_numeric_string`: Converts a number into a string of characters by performing division and modulo operations, writing the resulting digits or spaces to a destination buffer.
- `clamp_word_value`: Clamps a 16-bit value in a global variable between 4096 and 61440.
- `update_joystick_value`: Checks a global variable and the status of joystick bits 12 and 13; it then adds or subtracts 180 from a global word or clears it based on the joystick direction and the current value.
- `allocate_graphics_resources`: Allocates several bitmapped graphics buffers and screens, populates tables with memory addresses using loops, and performs cleanup via a routine that closes screens and clears data if specific global variables are non-zero.
- `set_rgb_colors`: Loops 32 times to extract nibbles from bytes in a table and calls another routine to set colors using those values.
- `execute_sequence_of_routines`: Performs six consecutive calls to different routines and then jumps to another routine.
- `call_offset_routines`: Performs five consecutive calls to a routine using different values in register d1, where d1 is derived from a byte in a global variable and a bit shift.
- `call_offset_routines_short`: Calls a routine with d1 set to 177, then jumps to the same routine with d1 set to 184, using a value from a global variable as the base for d0.
- `count_and_copy_bits`: Checks a word in a global variable, iterates through a list to count entries matching specific criteria, and then calls a routine twice to copy bits from a source table to a destination buffer.
- `calculate_scaled_value_2`: Calls a routine, calculates a value using a byte from a table, a multiplication, and a division by a value derived from a global variable, then updates a global variable and returns.
- `apply_masked_buffer_updates`: Performs three sequential operations involving division and calls to another routine to calculate offsets and indices for applying bitmasks to a buffer.
- `jump_to_offset_routine`: Loads a pointer from a global variable, retrieves a second pointer from an offset of the first, adds 80 to it, and branches to that address.
- `apply_bitmask_to_buffer`: Iterates through a table to perform bitwise operations on bytes in a destination buffer using values from a source table.
- `decrement_and_update_blit`: Compares two global words divided by 8; if the second is smaller, it decrements the first word, clears a word at offset 70 of a structure in a0, and calculates a new value for offset 12 of that structure before calling another routine; otherwise it returns.
- `loop_blit_updates`: Sets three global words to specific values, updates a structure at a0 with values, calls another routine, and then enters a loop that increments a word at offset 12 of the structure in a0 until it reaches 187.
- `process_blit_structure`: Calculates values by dividing two global words by 6, compares them, and enters a loop that updates a structure in a0 at offsets 12 and 70 and modifies a global word until a condition is met.
- `set_blit_parameter_short`: Writes 6 to offset 20 of the structure in a0 and jumps to another routine.
- `set_blit_parameters`: Adds a value from a global variable to d0, stores the result in a structure at offset 72 of a0, clears a word at offset 68 of a0, updates a global variable, and calls two different routines before returning.
- `loop_and_store_results`: Decrements a word in d4, loops while the value is non-zero, calls a routine, stores the result in a table, and returns if the result is zero.
- `configure_blitter`: Waits for a bit in a hardware register to clear, then sets several Blitter registers including the source address and size.
- `calculate_integer_square_root`: Calculates a value using a subroutine and bit shifts based on the difference between two registers, then performs additional shifts and additions depending on whether a global variable is less than an address.
- `calculate_scaled_value`: Calls a routine to calculate an interpolated value, then performs scaling and arithmetic operations on the result based on a global variable and a table, finally returning a modified value.
- `calculate_interpolated_value`: Performs a calculation involving division and multiplication to return a value based on a global variable and a table of values.
- `find_integer_square_root`: Calculates an integer square root of a value derived from a 32-bit register and stores the result in a global variable.
- `open_and_process_file`: Calls a routine to open a file, and if successful, calls another routine to examine it, then calls a third routine to read it, and finally calls a routine to close it while returning a pointer to a memory location offset by a global value.
- `open_and_read_file`: Opens a file, checks the result, and if successful, performs several library calls including a file read and a file close, returning a value from the stack.
- `loop_and_call_routine`: Executes a loop that runs 69 times, calling another routine in each iteration if a specific condition is met.
- `process_list_pointer`: Checks if a global pointer is non-zero, then calls a routine to remove an element from a list and another to append an element to it.
- `branch_to_list_logic`: Loads a global pointer into a register and branches to another routine.
- `append_to_linked_list`: Searches for the end of a null-terminated list in memory and appends the value from a register to it.
- `remove_element_from_list`: Searches for a specific 32-bit value in a null-terminated list and removes it by shifting subsequent elements forward.
- `update_list_offsets`: Stores a value into a structure at offset 56, then iterates through a list to update various values in a table based on elements in the list.
- `sort_list_by_long_offset`: Performs a bubble sort on a list of pointers based on a 32-bit value located at offset 56 of each pointed structure.
- `sort_list_by_word_offset`: Performs a bubble sort on a list of pointers based on a 16-bit value located at offset 8 of each pointed structure.
- `sort_list_by_long_offset_descending`: Performs a bubble sort on a list of pointers based on a 32-bit value at offset 40, swapping elements if the first is greater than or equal to the second.
- `sort_linked_list`: Iterates through a linked list of structures, comparing a 16-bit value at offset 20 of each structure and swapping the nodes if they are out of order, setting bit 0 of d0 if any swaps occur.
- `open_file_and_read`: Calls a routine to open a file, and if successful, calls two other routines to perform subsequent operations.
- `sort_top_scores`: Checks specific global conditions and, if met, performs a bubble sort on a list of scores by swapping elements in a table and calling library routines.
- `format_and_print_list`: Iterates through a list of structures, performing string formatting and calling a routine to print data for each entry.
- `initialize_data_structure`: Opens a file and, if successful, stores a value from a global structure into another global variable and calls a library routine.
- `seek_and_read_file`: Calls a routine to open a file, performs a seek operation using values from a global structure, and then calls another routine before jumping to a final routine.
- `open_and_seek_file`: Calls a routine to open a file, performs a seek operation, and then jumps to another routine.
- `store_result_after_call`: Calls a routine using an address from a table, then stores the returned value in a global variable and returns if the value is not zero.
- `process_blit_clipping_parameters`: Sets a bit in a global variable, updates a structure via a pointer, and iterates through a list of pointers to call various subroutines.
- `compare_and_branch`: Moves d0 to a6, compares the word at offset 0 of a6 with ten, and branches if equal.
- `calculate_blit_parameters`: Calculates spatial bounds and clipping parameters for a Blitter operation based on a structure's dimensions and a coordinate offset, then updates several global variables and hardware registers including the Blitter control and DMA registers.
- `initialize_and_sort_list`: Calculates a value using a global byte and a global word, clears several global variables, and enters a loop that calls a routine to update structure offsets and another routine to perform a loop-based copy.
- `update_range_data`: Calls a routine, calculates a value based on the result, and writes that value and a constant to a global variable and a structure field.
- `update_structure_fields`: Calls another routine and, if it returns a non-zero pointer, writes several values and offsets into the memory pointed to by that pointer.
- `decrement_and_update_counters`: Decrements one global counter and, if it reaches zero, calls a routine to update a structure field and another global counter; otherwise, it decrements a second global counter and updates a structure field.
- `clear_d0_register`: Sets the least significant byte of d0 to zero.
- `allocate_list_node`: Adds 12 to a value at offset 8 of a pointer in a6, calls a memory allocation routine with that value, and if the result is non-zero, stores the result in a local variable and returns it.
- `update_list_node`: Calculates a new value by adding 12 to a value at an offset of 8 from a pointer, stores this in a structure, and calls another routine with a global variable and a pointer.
- `free_linked_memory_blocks`: Walks a singly linked list whose head is held in a global, freeing each node with the size stored at its offset and following the next pointer until the list ends.
- `append_node_to_list`: Appends a node to a list whose head and tail pointers are held in the first structure by updating the next and head/tail pointers.
- `allocate_and_test_memory_buffer`: Allocates a memory block and performs a series of subroutines to initialize and validate its contents, including two sequential tests that check if a specific value range is maintained within the buffer.
- `initialize_hardware_and_copy`: Copies 20 bytes from a global table to a destination pointed to by a1, modifies a byte at offset 32242 of a hardware register using a bitwise AND, sets several hardware registers to specific values, and writes a bit-shifted value to a destination in a6.
- `update_hardware_register`: If the stack pointer is valid, it writes 32767 to a hardware register at offset 394 of a6, then performs an OR operation between a value from a6 and 49152 and writes the result to the same register.
- `update_hardware_status`: Compares the stack pointer with a global variable and branches to another routine if it is less.
- `check_stack_and_return`: Compares the stack pointer with a value at offset 4, branches if less, otherwise calls another routine and returns 0.
- `decrement_and_call_routine`: Compares a7 with a global pointer, and if not less, decrements a value at offset 8 of the stack frame by a factor of 4 and calls another routine if the result is not zero.
- `clear_and_call_subroutines`: Compares a7 with a global pointer, and if not less, checks a byte at offset 1 of a structure in a0; if non-zero, it clears that byte and calls two different routines using the structure pointer on the stack.
- `set_byte_and_call_routines`: Checks a byte at offset 1 of a structure in a0; if zero, it sets it to 1 and calls two different routines and a third routine with a constant value.
- `handle_hardware_and_interrupts`: Initializes local variables, writes to a hardware register, and enters a loop that checks a hardware register and calls another routine if a specific bit condition is met.
- `process_input_delta`: Calculates differences between values stored in a structure and updates a CIAA register based on those differences.
- `set_hardware_bit_and_call`: Reads a byte from a hardware register, clears its lowest bit, sets its lowest bit, and then calls another routine with a constant value.
- `modify_hardware_byte`: Sets bit 7 of a hardware register at offset 32753, then performs bitwise operations using a byte from a6 and a bit-shifted byte from offset 1 of a6 to update the same hardware register.
- `set_hardware_bits`: Sets bits 6 and 7 of a hardware register at offset 32753 by performing an OR operation with 120.
- `perform_signed_division`: Calculates a value using two calls to another routine, then enters a loop that performs bitwise shifts and subtractions to determine a quotient based on a comparison of magnitudes.
- `parse_scoreboard_data`: Loads an address into a0, calls another routine, clears a long word at offset 4 of a2, moves a value from a global variable into d7, calls another routine, and jumps to a new location.
- `repack_bitplane_nibbles`: Calls a routine to obtain a pointer, clears a byte at offset 4 of a structure, and then enters a loop that performs bitwise shifts and masks to repack nibbles from three consecutive bytes into a 16-bit word stored in a global variable.
- `repack_bitplane_nibbles_2`: Calls a routine to obtain a pointer, clears a byte at offset 4 of a structure, and then enters a loop that performs bitwise manipulation to repack three bytes into a sequence of 16-bit words by shifting and masking nibbles before storing them in a global variable.
- `repack_bitplane_nibbles_3`: Calls a routine to obtain a pointer, clears a byte at offset 4 of a structure, and then enters a loop that performs bitwise shifts and masks to repack three bytes of data into a sequence of 16-bit words before returning.
- `reorder_bitplane_nibbles`: Calls two routines to retrieve data, clears a byte at offset 4 of a global structure, and then executes a loop that performs bitwise shifts and masks to reorder nibbles from three consecutive bytes into a single 16-bit word stored in a global variable.
- `repack_bitplane_nibbles_4`: Clears a byte at offset 4 of a global variable, calls another routine, and then enters a loop that performs bitwise manipulation to repack three bytes into a sequence of 16-bit words by shifting and masking nibbles before storing them back to a global variable.
- `fetch_and_call_pointers`: Pushes four pointers from a global table onto the stack, then performs four calls to another routine using pointers retrieved from a table at offset 24 of a6, before restoring the stack and jumping.
- `increment_table_elements`: Calls another routine, sets a word at offset 2 and a long at offset 4 of a2, calls another routine, and then enters a loop that adds 40 to three consecutive long words starting at a structure in a3.
- `unpack_ilbm_bitplane_data`: Checks a byte at offset 5 of a structure to ensure it equals 1, then iterates through a list of pointers to expand compressed bitplane data using a run-length encoding scheme.
- `configure_audio_channels`: Calls another routine three times to load data into a global variable, then writes the resulting pointer, a shifted period value, a length value, and a volume value to the hardware audio registers.
- `configure_audio_hardware_2`: Calls two routines to process a file, stores the result in a global longword, and then updates the Amiga audio hardware registers with values from a global structure.
- `configure_audio_hardware`: Sets bytes in a global structure, assigns two routines to global pointers, calls three routines to configure settings, and finally writes a value to a hardware register.
- `clear_d0_register_twice`: Sets the least significant byte of d0 to zero twice.
- `clear_data_register_d0`: Performs a logical OR operation on data register d0 with zero.
- `write_constant_to_offset`: Writes the word value 512 to offset 156 of the structure pointed to by a0 and returns with d0 set to 1.
- `call_open_and_read`: Calls a routine and, if the return value in d0 is not zero, branches to another location.
- `parse_8svx_audio_header`: Validates the 8SVX format by comparing string tags and then calculates a value by dividing a constant by a 16-bit integer found at an offset from a pointer.
- `compare_and_copy_strings`: Compares bytes from two source locations and copies them to a destination until a mismatch is found or a null byte is encountered in the source at a1.
- `parse_ilbm_file_structure`: Calls a routine to obtain a pointer, then validates three consecutive string tags by comparing them against known constants.
- `_RangeRan`: Steps a 32-bit shift-register pseudo-random generator once per bit of the argument, reseeding it with a fixed feedback constant when the carry is set or the state is zero.
- `_FastRand`: Doubles the input parameter and conditionally XORs the result with a constant if a carry occurs or the result is zero.
- `compute_signed_magnitude_division`: Divides two register inputs by a restoring division loop over 32 bit iterations on their magnitudes, giving the quotient the sign of the xor of the inputs.
- `init_intuition_request`: Opens the intuition library, stores the handle in a global variable, performs an auto request with fixed dimensions, and then calls another routine with twenty as an argument.
- `visit_global_table_indices`: Loops twenty times calling one routine to get an index and passing a global table address offset by that index to a second routine, then loops a variable number of times passing each counter value to a second routine, and finally forwards two parameters to a third routine.
- `_AutoRequest`: Loads a library base from a global variable and calls a function at a negative offset using several arguments passed from the stack.
- `empty_stub_function`: The routine contains a block of 31 68000 instructions but ends by returning without performing any observable effect on memory, registers, or I/O.
- `_fclose`: The routine receives a pointer argument and forwards it unchanged to another routine that deletes an extended I/O. It performs no other work and then returns.
- `check_flag_and_return_pointer`: Optionally calls a helper, then, if the index provided in a parameter is in range and the table entry has its flag bit set, returns a pointer into the table; otherwise it stores an error code and returns a null pointer.
- `search_and_set_byte`: Compares a long at a global address with a value in d23, then enters a loop that calls another routine and checks a byte at an offset from a returned pointer to decide whether to increment a local counter and continue.
- `compare_and_set_value`: Compares a long value at offset -4 of a6 with zero; if not zero, it branches; otherwise it sets a value at offset 0 of a6 to 24, sets a value at offset 0 of a6 to 255, and returns.
- `update_status_and_pointer`: Calls another routine, updates a value at offset 12 of the structure in a6 by XORing it with a value from a global table, and stores a pointer at offset minus 12 of the structure in a6.
- `update_status_byte`: Sets bit 7 of a byte at an offset of minus 12 from a6, then checks a value at offset 12 of a6 to branch to one of three locations; if it reaches the final block, it sets bit 6 of the byte at offset minus 12 from a6.
- `test_bit_three_and_branch`: Tests bit 3 of a byte at an offset from a pointer in a6; if the bit is not set, it loads 8 into d0 and branches to another location, otherwise it branches to a different location.
- `set_file_status_bit`: Sets bit 5 of a byte at an offset from a pointer, and then branches to another routine.
- `test_bit_three_and_branch_alt`: Tests bit 3 of a byte at an offset from a pointer in a6; if the bit is not set, it loads 8 into d0 and branches to a different location, otherwise it branches to another location.
- `set_file_status_bits`: Sets bits 5 and 6 of a byte at an offset from a pointer, and then branches to another routine.
- `fail_slot_with_invalid_mode`: Clears the slot's status byte, records an invalid-argument error code, and returns minus one.
- `process_file_error`: Checks bit 1 of a value at an offset of 14 from a6, and if set, calls a routine with a pointer from a6 plus 8, then checks if a value at offset 12 of a6 masked with 768 is zero, and if so, moves 750 to a location at offset -8 of a6.
- `open_file_with_error_handling`: Calls a routine to open a file, stores the result in a structure, and if the result is zero, calls an error routine and updates global status values before returning.
- `return_value_from_stack`: Returns a 32-bit value stored at an offset of minus four from the stack frame pointer.
- `close_handle_if_flag_clear`: Obtains a structure from its argument and returns negative one if another routine returns zero.
- `delete_file_with_error_handling`: Calls a routine to delete a file, and if it fails, calls an error handler, stores the error code in a global variable, sets a global status to two, and returns minus one; otherwise it returns zero.
- `tiny_moveq_unlk_rts`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a6, then returns.
- `open_file_handle`: Opens the file named in the global text buffer with a fixed mode value and stores the resulting handle in a global variable.
- `read_byte_from_file`: Optionally calls a helper, then checks a global byte; if the byte is non-zero, it clears it and returns the original value, otherwise it reads a byte from a file handle into the stack and returns it.
- `read_single_file_byte`: Reads one byte from the file handle into a stack slot with a single-byte read call, and returns that byte.
- `write_single_byte_file`: Checks a global longword and a global longword, calls another routine, and if a global longword is non-zero, calls a write routine to output a single byte from a stack location to a file handle stored in a global variable.
- `check_byte_and_return`: Compares a byte at a specific memory location to zero and, if it is zero, returns with the value negative one in d0; otherwise it returns immediately.
- `store_byte_from_argument`: Copies the low byte of its stack argument into a global variable and returns.
- `close_active_handles`: Loops over a table of six-byte entries up to a global count, closing each entry that is non-zero and does not have flag bit four set, then calls another routine with its original parameters.
- `_Open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `_Close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `_Read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `_Write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `_Seek`: Library stub: loads its arguments and the dos library base, calls Seek and returns its result to the caller.
- `_DeleteFile`: Library stub: loads its arguments and the dos library base, calls DeleteFile and returns its result to the caller.
- `_Rename`: Library stub: loads its arguments and the dos library base, calls Rename and returns its result to the caller.
- `_IoErr`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `check_signal_and_dispatch`: Calls a signal routine and, if the result masked with 12288 is non-zero, checks a global longword and calls another routine with 67 or 68, then calls another routine with 10, and finally loops from 0 up to a global count to call another routine with each index.
- `clears_data_register_zero_2`: Performs a logical OR operation on data register zero using zero as the operand.
- `clears_data_register_zero_3`: Performs a bitwise OR operation on data register zero using a zero constant.
- `performs_logical_or_on_data_register_zero`: Performs a logical OR operation on data register zero using the constant zero.
- `clears_data_register_zero_4`: Performs a logical OR operation on the lowest byte of data register zero with zero.
- `allocate_and_expand_data_table`: Allocates memory using a library call, then enters a loop that expands a data table by calculating a scaled stride and copying elements into a new structure.
- `execute_animation_sequence`: Copies values from a table into a structure, calculates several fields including a scaled value and a pointer offset, and then loops through a table to call a routine with different parameters before jumping to another routine.
- `execute_library_sequence`: Calls a routine via a pointer from a table, then calls two other routines using offsets from a base address retrieved from that same table.
- `process_library_chain`: Calls a routine via a pointer from a table, and if the result is non-zero, calls two more routines via offsets from that same table, then calls three additional routines using offsets from the base address and a global pointer before returning a global value.
- `clear_d0_and_return`: Restores registers d2 through d7 and a2 through a6 from the stack, clears d0, and returns.
- `bitwise_or_tables`: Performs a nested loop that reads 16-bit values from two tables, bitwise ORs them together, and writes the result back to a third table at an offset determined by a byte at offset 8 of a structure in a4.
- `expand_and_copy_data_table`: Checks a pointer in a2, then performs a data expansion and copy operation where a source table is processed based on a count and a flag, either copying words with a calculated stride or expanding bytes into words, and finally calls copy_data_with_expansion.
- `calculate_copy_count`: Compares a pointer to a limit, checks a byte at an offset from a second pointer, and if it is not one, calculates a new count based on a word at that offset and branches to another routine.
- `copy_signed_bytes`: Reads a signed byte from a source, and if it is negative and not negative 128, copies the following byte to a destination and loops using the absolute value of the first byte as a counter.
- `copy_byte_loop`: Copies bytes from a source address in a3 to a destination address in a5 using a loop controlled by the counter in d0, then branches to another routine.
- `process_bitwise_table_updates`: Performs a bitwise AND operation on 16-bit values from two tables and writes the result to a third table, then executes a secondary loop to copy data from a table to a stack-based buffer before calling another routine.
- `copy_data_with_expansion`: Saves registers to the stack, copies a longword to a destination, and then enters a loop that either copies words from a table or performs an expansion copy based on a flag in a structure.
- `copy_bytes_with_counters_2`: Checks if d2 and d5 are positive, calculates a new counter from a table entry, and then loops to copy bytes from a source to a destination, handling negative values by repeating bytes based on a signed byte count.
- `copy_bytes_with_counters`: Copies a byte from the source pointed to by a3 to the destination pointed to by a5, decrements d2 and d5, and loops using d0 until d0 reaches negative one, then jumps to another routine.
- `restore_registers_and_return`: Restores registers d2 through d7 and a2 through a6 from the stack and returns.
- `parse_ilbm_header_and_data`: Calls another routine to retrieve a pointer, then validates the header by comparing strings against specific constants.
- `process_data_list`: Copies a byte from a source at a0 to a destination at a1, decrements a count in d0, and loops if the count is not zero; it also calls another routine twice using a pointer from a0 and a global variable.
- `process_string_data`: Divides a longword from a0 by 3, stores the result in a global variable, copies a byte from a0 to a local buffer, and then calls another routine to handle further processing based on the result.
- `update_global_pointer`: Adds 4 to a0 and stores the resulting address into a global variable.
- `jump_to_address`: Jumps to the address stored at the location 1482.
- `compare_and_copy_strings_2`: Compares bytes from a source at a1 to a destination at a0, copying them to the destination as long as they are equal and the source byte is not zero.
- `load_stack_pointer`: Loads the value from a global variable into the stack pointer register and returns.
- `call_library_functions`: Loads a pointer from offset 76 into a7, calls another routine, loads 20 into the stack, and calls another routine.
- `increment_stack_counter`: Loops while a byte at an address calculated from a pointer in a6 and a value at negative four relative to a6 is zero, incrementing a counter at negative four relative to a6 on each iteration.
- `fetch_stack_pointer_value`: Loads a longword from an offset of negative four relative to the current stack frame into a data register and returns.
