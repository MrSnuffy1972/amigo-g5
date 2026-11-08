# Game 1054 (formula one grand prix v1.02 (1991)(microprose)(disk 1 of 4)[cr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01054). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901248 top-level bytes in 15 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 205 of 205 named (0 author symbols), 205 of 205 described (8 by rule, 197 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 341 of 342 (99.71%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4645 of 4645 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 796788 |
| code | 15926 |
| data | 75051 |
| padding | 2731 |
| unused | 10752 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Formula One Grand Prix v1.02 (1991)(MicroProse)(Disk 1 of 4)[cr].adf:frontend | 4 | 4437 | 200 |
| Formula One Grand Prix v1.02 (1991)(MicroProse)(Disk 1 of 4)[cr].adf:formula one grand prix | 1 | 197 | 3 |
| Formula One Grand Prix v1.02 (1991)(MicroProse)(Disk 1 of 4)[cr].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 0 | 0 |

## Cases handled

- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op ABAD) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Formula One Grand Prix v1.02 (1991)(MicroProse)(Disk 1 of 4)[cr].adf:frontend

- `initialize_engine_and_dispatch_tables_2`: Initializes several hardware and software parameters, populates a table of four longwords by incrementing values by 8000, and executes a series of calls to other routines using values stored in a register as pointers.
- `initialize_engine_and_dispatch_tables`: Initializes several hardware and software parameters, populates a table of four longwords by incrementing values by 8000, and executes a series of subroutine calls through a table of pointers while updating a structure in a6.
- `cleanup_and_release_resources`: Closes several devices and libraries, clears multiple pointers and structure fields, and executes a series of callback routines stored in the structure in a6.
- `check_joystick_button_state`: Checks a bit in a global variable and, if not zero, reads the status of the top two bits from the CIAA port and returns the result in d0; otherwise it returns.
- `call_library_functions`: Calls two different library functions using pointers from a6, stores the results in offsets 2032 and 2028 of a6, and sets a 16-bit value at offset 2878 of a6 to 200.
- `update_status_and_call_library`: Sets a 16-bit value at offset 2878 of a structure in a6, calls a library function via a pointer in a6, and if a value at offset 2032 of a6 is non-zero, calls another library function, sets the same 16-bit value again, and clears the value at offset 2032.
- `initialize_hardware_and_input_parameters`: Clears two fields in a structure, calls several routines to allocate resources and update pointers, and then performs a loop up to four times to check for specific status bits.
- `initialize_video_mode_and_buffers`: Sets several word-sized parameters in a structure based on a conditional check, calls a library function to allocate resources, copies a block of data from a table into the structure using a loop, and then calls another library function to perform a memory operation before executing two subsequent routines.
- `initialize_and_copy_data`: Fills a table with four incrementing 32-bit values starting from zero, calls another routine, copies 61 words from a source at offset 5470 of a6 to a destination at offset 2088 of a6, and clears two 16-bit values at offsets 2920 and 2928 of a6.
- `clear_structure_fields`: Clears seven 32-bit words at specific offsets within a structure pointed to by a6.
- `close_device_pointers`: If a value at offset 2364 of a6 is non-zero, calls a routine and clears it; then if a value at offset 2834 is non-zero, calls a library function and clears it; finally, if a value at offset 2490 is non-zero, calls a library function and clears it.
- `reset_structure_values_2`: Checks three different long values in a structure; if any are non-zero, it calls a routine using a pointer from offset 2356 of a6, sets a word at offset 2878 of a6 to 200, and clears the checked long value.
- `update_function_pointer`: Stores the address of the calling routine into a location at offset 2850 of a structure pointed to by a6, then returns.
- `call_and_update_pointer`: If a pointer at offset 1826 of a6 is non-zero, calls that pointer and stores the pointer incremented by 8 into offset 1786 of a6.
- `execute_callback_loop`: If a longword at offset 2850 of a6 is non-zero, it calls a routine at an offset of 12 from that value, calls another routine, and repeats this in a loop until a counter reaches a specific threshold.
- `clear_and_call_offset_pointer`: If a pointer at offset 1826 of a6 is non-zero, clears offset 1786 of a6, calls the routine at offset 1826 plus 4, and then clears offset 1786 of a6 again.
- `initialize_and_copy_buffer`: Clears 32 words in a buffer starting at offset 3236 of a6, copies 32 words from a structure pointed to by a global variable into that same buffer, and then calls another routine 6 times using a pointer and a value from a table.
- `call_library_via_offset`: If a value at offset 2458 of a6 is non-zero, calls a library function using a pointer found at offset 2458 plus an offset from a library base in a6.
- `execute_nested_loops_2`: Clears a word at offset 3038 of a6, sets a word at offset 3042 of a6 to one, executes a loop 31 times calling a routine, and then either calls another routine or sets values at offsets 3040 and 3038 before calling a third routine.
- `execute_nested_loops_3`: Calls three different routines, then loads a byte from offset 4625 of a6 into d1, and if d1 is not zero, it enters a loop that calls three routines and one other routine for a number of iterations determined by d1, eventually testing a byte from a pointer in a0 and branching based on the result.
- `call_two_subroutines`: Calls two subroutines in sequence and then returns.
- `copy_data_block`: Copies 196 words from a source table starting at offset 40 of a structure in a1 to a destination via a loop, then copies 17 words from various offsets of a6 into the destination structure.
- `render_bitmapped_data`: Clears several 32-bit words in a structure, then if a byte in d0 is less than 8, iterates through a buffer to perform bitwise operations and bit-shifting to write data into a destination buffer at a calculated offset.
- `execute_six_subroutines`: Calls six subroutines in sequence and returns a byte from offset 3890 of a structure in a6 extended to a word in d0.
- `execute_five_subroutines`: Calls five subroutines in sequence and returns a byte from offset 3890 of a structure in a6 extended to a word in d0.
- `reset_and_call_library`: Stores several values into a structure in a6, calls another routine, clears a byte at offset 3024 of a6, and calls two library functions using a pointer from offset 2834 of a6.
- `process_interleaved_data`: Calls a routine, loads four words from an offset of 2964 plus 6 in a6, performs bitwise shifts and subtractions on these words, and calls two more routines before returning.
- `update_structure_field`: Calls three routines, checks a word at offset 2972 of a6, and if non-zero, moves a word from offset 2952 of a6 to offset 4 of a pointer found at offset 2948 of a6.
- `process_message_and_clear`: Calls a library function to get a message, then enters a loop calling another library function until it returns zero, then clears offset 2406 of a6, calls a library function, and clears offset 2964 of a6.
- `execute_sequence_of_subroutines`: Sets d0 to 10, calls five subroutines in sequence, sets d0 to 10 again, calls one more subroutine, and returns.
- `process_structure_data`: Calls a subroutine using a pointer from a structure, then enters a loop that decrements a value from a structure, calls a subroutine using a table-derived pointer, and copies four words from a table into registers d0 through d3 before calling another subroutine.
- `process_data_structures`: If a 16-bit value at offset 872 of a6 is zero, it performs a series of operations including calling a routine, iterating through a list of structures to modify bytes and call routines based on bit flags, and finally copying a longword to a destination.
- `iterate_and_call_elements`: Iterates through a list of structures starting at an offset from a pointer in a global variable, clearing a byte at offset 26 of each structure and calling a function at an offset within the structure if bit 7 of offset 27 is not set.
- `update_input_state`: Calls another routine, then checks bits in d0 to conditionally clear 16-bit values at offsets 2976 and 2978 of a structure in a6, increments a value at offset 2978, and calls another routine with various constants based on specific bit tests in d0.
- `process_message_loop_2`: Retrieves a message from a pointer, copies values from the message into a structure at offset 3020 and 3022 of a6, calls three different routines in a loop, and conditionally calls another routine if bit 0 of a byte at offset 3024 of a6 is set.
- `manage_status_flags`: Checks d7 and d6 to conditionally call another routine, and if d7 is 8 and d6 is 104, it may set bit 0 of a byte at offset 3024 of a structure in a6 and update a byte at offset 26 of a structure in a2 using a calculated offset from a table.
- `update_structure_status`: Checks bit 3 of offset 27 of a structure in a2; if set, it performs a calculation involving a value from a6, a pointer in a2, and a division, then calls a function via a pointer in a2 and sets bit 7 of offset 26 in a2; otherwise, if bit 4 of offset 27 is set, it performs a similar calculation and function call.
- `dispatch_menu_selection`: Checks if a register equals 1024 and calls another routine; based on the returned byte, it either calls a specific function via a calculated table offset or calls another routine if the return value matches a carriage return or space.
- `set_stack_limit`: Compares d7 to 65536 and, if equal, writes 65535 to offset 2872 of a6; otherwise, if d7 equals 32768, it writes 65535 to offset 2870 of a6.
- `check_bounds_loop`: Iterates through a table starting at a pointer in a6, comparing d0 and d1 against range values stored in the table entries, and returns if a match is found; it returns 65535 if bit 2 of offset 27 of a2 is set and bit 6 is not.
- `update_entry_index`: Fetches a byte from a structure in a2 using an index in d1, compares it against a value in a6, and if it meets specific criteria, calls a function via a pointer in a3, sets bit 7 of offset 26 in both a2 and a3, and updates values in a6.
- `process_table_entries`: Calculates an offset into a table based on a value in a6, checks bits in the table entry, and if conditions are met, calls a function via a pointer in the table and potentially iterates through the table to update entries and set bits.
- `query_system_status`: Populates a structure at offset 2580 of a6 with values from d5, d6, and d7, calls a function via a pointer in a6, and returns a value derived from the result or from bytes at offset 2618 or 2619 of a6.
- `set_conditional_values`: Tests bit 6 of offset 27 of a2; if set, it branches to another routine, otherwise it calls another routine, sets d4 to 18, sets d6 to 23, compares a byte at offset 0 of a2 with a word at offset 2968 of a6, and if they are not equal, branches to another routine; if they are equal, it sets d4 to 15, d6 to 17, and d5 to 31, then tests bit 0 of offset 26 of a2 to set d5 to 14 or branch to another routine.
- `test_bit_and_branch`: Tests bit 2 of a byte at offset 1 of a structure pointed to by a2 and branches to another location if the bit is clear, otherwise it sets d5 to 20 and branches to another location.
- `update_structure_values`: Calls two routines, then conditionally performs three separate updates to offsets 2892, 2894, and 2898 of a6 using values from d0, d1, and d4, d5, or d6, before calling another routine with a6.
- `calculate_scaled_offsets`: Performs arithmetic on values from a structure in a2, including subtractions and logical shifts, and stores the results in offsets 2892, 2894, and 2898 of a6.
- `compare_and_call_subroutine`: Loads four words from a pointer in a2 into d0 through d3, sets d4, d5, and d6 to specific values, and if a bit in offset 44 of a2 is set, exchanges d4 and d6, calls a subroutine, and if a byte at offset 2968 of a6 matches a byte at offset 0 of a2, sets d5 to 16 and calls another subroutine.
- `modify_registers_and_call`: Increments d0 and d1, decrements d2 and d3, calculates a value from d5 and a table at offset 80 of the structure in a6, and calls a subroutine using an address derived from that value.
- `apply_masked_updates`: Uses values from d4, d5, and d6 to fetch offsets from a table at offset 80 of a6, then performs five calls to different routines using those offsets and various register combinations.
- `copy_block_of_data_structures`: Calculates a destination offset based on input values and performs a nested loop to copy blocks of five longwords from a source memory area to five separate destination buffers.
- `process_joystick_input`: Reads a byte from the CIAA register, performs bitwise manipulations and shifts on the value, and combines the results into a single byte returned in d0.
- `wait_for_input_bit`: Calls another routine and loops while bit 7 of the returned value in d0 is set.
- `call_library_function_conditionally`: Checks if two pointers at offsets 2834 and 2088 from a6 are non-zero, then calls a function via a pointer at offset 2352 minus 270 from a6 using parameters passed in d0 and d1.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_function_with_offset`: Checks if two pointers at offsets 2834 and 2088 from a6 are non-zero, then calls a function via a pointer at offset 2352 minus 270 from a6 using a pointer at offset 108 plus 12 as the second parameter.
- `copy_data_block_2`: Calls two routines, then copies 32 bytes from a table at offset 3364 of a6 to a destination at offset 3236 of a6 using a loop, and finally calls another routine.
- `call_subroutines_with_stack`: Saves registers to the stack, loads a word from offset 1972 of a structure in a6 into d0, loads a pointer from a calculated address into a0, calls another routine, then loads a pointer from offset 4373 of a structure in a6 into a0 and calls another routine before restoring registers and returning.
- `store_and_call_subroutines`: Stores a longword from a0 into offset 3000 of a6, stores a longword from a1 into offset 2998 of a6, sets d0 to 21, calls another routine, calculates a new address in a0 by adding a constant to a value derived from a6, and calls another routine before returning.
- `calculate_and_store_offsets`: Loads a pointer from offset 3030 of a6 into a0, calls another routine, modifies the result by shifting right, negating, and adding 160, then stores this value and the constant 68 into offsets 2904 and 2906 of a6; it repeats this process using a pointer from offset 3034 of a6 to store values into offsets 2892, 2894, and 2898 of a6 before calling another routine and returning.
- `validate_and_initialize_menu_state`: Checks a 16-bit value in a structure, and if non-zero, copies a 32-byte block from a table to a destination in the structure.
- `initialize_menu_display_parameters`: Checks a flag in a structure, then performs a series of calls to set up various data pointers and values, including a calculation that modifies a word at offset 2892 of the structure in a6 using a value from offset 876 and a constant.
- `update_event_flags_and_dispatch`: Sets a bit in a longword at offset 2406 of a structure, calls a library function via a pointer, and executes three subroutines.
- `clamp_three_values`: Checks three separate 32-bit values stored in a structure at offsets 2064, 2068, and 2072 against minimum bounds stored at offsets 2052, 2056, and 2060 respectively, updating the values if they fall below their limits and returning the new value in register d0.
- `update_global_counters`: Adds three different constant values to three separate 32-bit values stored at offsets 804, 808, and 80c from a6 and stores the results in offsets 810, 814, and 818 respectively.
- `clear_and_call_library_functions`: Checks three longwords at offsets 2044, 2040, and 2036 of the structure in a6; if non-zero, it calls a library function using a pointer from a library base, sets a 16-bit word at offset 2878 to 200, and clears the checked longword, finally calling another routine before returning.
- `process_menu_selection_and_load_data`: Checks if a provided index matches a stored value in a structure, and if not, retrieves a pointer and a count from a table to initialize a menu state.
- `reset_structure_values`: Clears a 16-bit word at offset 1970 of the structure in a6, sets a 16-bit word at offset 880 of the structure in a6 to 65535, and calls another routine.
- `store_pointer_and_call`: Stores a longword from a0 into a location at offset 886 of a structure in a6, then calls another routine.
- `execute_table_function_calls`: Iterates through a table of pointers, calling a subroutine for each entry via a jump table until a specific condition is met or the table ends.
- `call_subroutines_via_pointer`: Loads a pointer from offset 574 of the structure in a6 into a7, then calls two subroutines using that pointer before returning.
- `initialize_data_structures`: Clears a block of memory starting at offset 962 from a6, populates a table starting at offset 898 from a6 with incrementing values, and sets several other global word and long values at various offsets from a6.
- `populate_structure_from_buffer`: Reads a series of values from a buffer in a0, calculates an offset using a multiplication, and writes the processed values into a structure starting at offset 962 from a6.
- `execute_nested_loops`: Saves registers to the stack, calls another routine, then executes a loop that calls a library function a variable number of times based on a value at offset 3042 of the structure in a6, followed by a second loop that calls another routine, and finally sets a 16-bit word at offset 2830 of the structure in a6 to 0.
- `calculate_byte_differences`: Performs bitwise shifts and subtractions using values from a pointer in a1 and a value in d4, then stores the resulting six bytes into a destination pointed to by a2.
- `shift_memory_block`: Shifts a block of memory starting at a0 by a length specified in d0, handling signed 16-bit values by either moving them or performing a block copy.
- `decrement_and_branch_loop`: Decrements a word at offset 22 of a structure in a1, and if it is not zero, it continues a loop that clears a word at offset 0 of a1, writes a 16-bit value of all ones to offset 546 of a6, and branches back to the start of the routine.
- `update_coordinate_offsets`: Copies a word from offset 20 of a1 to offset 22 of a1, performs arithmetic on values from offsets 4 and 6 of a1 using bit shifts and values from a6, stores the results in a6, calls another routine, and finally updates offsets 4 and 6 of a1.
- `configure_hardware_and_call_library`: If a word at offset 872 of the structure in a6 is zero, it calls three subroutines, copies seven longwords from a structure to a destination, modifies the INTENA register to change interrupt settings, calls a library routine via a pointer, calls another library routine, and calls two more subroutines before setting the word at offset 872 to 65535.
- `loop_library_call`: Iterates 31 times, calling a function via a pointer at offset 2490 minus 288 from a6 with a value from a table at offset 3300 and a decreasing index.
- `sort_table_elements`: Performs a bubble sort on a table of 30 elements starting at offset 898 from a6 by comparing values against a reference table starting at offset 962 from a6.
- `transfer_index_and_call`: Pushes d0 to the stack, calls a routine, pops the value back into d0, calls another routine, and returns.
- `branch_to_offset`: Loads the address of the 16-bit word at offset 60 of the structure in a6 into a0 and branches to another routine.
- `copy_table_entries_to_pointers`: Loads five pointers from a structure at offset 40 of a6, calculates an index into a table using a value in d0, and then copies five 16-bit words from that table entry into the memory locations pointed to by the loaded pointers through a loop of 4000 iterations.
- `update_bitmapped_sprite_data`: Calculates an offset into a data table based on input values and performs a series of bitwise rotations and logical operations to update five consecutive memory locations using values fetched from a source buffer.
- `update_sprite_attribute_table`: Calculates offsets and bitmask values based on several input parameters and global variables, then enters a nested loop to perform bitwise operations on a series of 16-bit values stored in a table pointed to by a6.
- `process_byte_table`: Runs a loop 32 times where it reads three bytes from a table, adds each byte to its predecessor, and packs the results into a 16-bit word stored in another table.
- `execute_subroutine`: Saves registers d0 through d7 and a0 through a3 to the stack, calls a routine, restores the registers, and returns.
- `update_parallel_data_streams`: Calculates offsets into five parallel data arrays based on input values and performs a bitwise update on 16-bit words using masks retrieved from tables.
- `apply_bitmask_loop`: Performs a loop where it applies a bitwise AND and OR operation using two masks to update five different 16-bit words in a table.
- `update_buffer_range_with_masks`: Checks if a range defined by three values in a structure at a6 is valid, then updates five 16-bit words in a table at a1-a5 by applying bitwise masks derived from a lookup table.
- `apply_bitmask_single`: Updates five 16-bit words in a table by applying a bitwise AND and OR operation using two masks.
- `update_masked_data_range`: Checks if a value is within a specific range, and if so, calculates an offset to update five 16-bit words in a table using a mask derived from a source table.
- `update_masked_data_direct`: Checks if a value is within a specific range, and if so, updates five 16-bit words in a table using a mask retrieved from a table via an index.
- `handle_hardware_and_callbacks`: Increments a word at offset 2872 of a6, clears a word at offset 2868 of a6, and if a word at offset 2878 of a6 is non-zero, decrements it, writes a value to a hardware register at offset 128, calls a routine, and then calls a function pointer stored at offset 2842 of a6.
- `update_display_buffer`: Checks a word at offset 872 of a structure, and if it is zero, decrements a value at offset 3040 and potentially updates a value at offset 3042 to offset 3040.
- `handle_interrupt_logic`: Checks bits in a hardware register to determine an action, potentially increments two values at offsets 2872 and 2874 of a structure, and if a counter at offset 3038 is non-zero, performs a series of decrements and a memory copy loop to a fixed address.
- `clear_interrupt_and_return`: Restores registers from the stack, writes 64 to the interrupt request register, and returns from an interrupt.
- `clear_interrupt_and_return_alt`: Restores registers from the stack, writes 16 to the interrupt request register, and returns from an interrupt.
- `execute_sequential_tasks`: Checks a word at offset 2864 of a structure and, if non-zero, calls three different routines in sequence before returning.
- `copy_data_blocks`: Determines which of two source tables to use based on a bit in a global variable, then copies 64 longwords from the selected source to a destination table.
- `trigger_data_copy`: Loads a word from offset 2864 of a structure into d0 and calls another routine.
- `clear_value_if_greater`: Compares a word at an offset in a structure to another word at a different offset, and if the first is not lower, clears that word.
- `swap_and_copy_tables`: Toggles a bit in a structure, updates a hardware register with a calculated address, and copies five longwords between two different tables based on the toggled bit.
- `check_structure_flags`: Checks if a word at offset 368 of a structure is zero, and if so, checks if a word at offset 2868 is non-zero before calling another routine.
- `conditional_routine_call`: Checks if a word at offset 368 of a structure is zero, and if so, calls another routine.
- `set_wait_flag`: Writes a 16-bit value of all ones to an offset of 1844 of the structure in a6 and returns if the value is not zero.
- `copy_four_longwords`: Writes a 16-bit value of one to an offset of 1842 of the structure in a6, then copies four longwords from a table starting at offset 20 of the structure in a6 to a destination starting at offset 40 of the structure in a6.
- `copy_four_longwords_from_a6`: Writes a 16-bit value of zero to an offset of 1842 of the structure in a6, then copies four longwords from the start of the structure in a6 to a destination starting at offset 40 of the structure in a6.
- `clear_and_compare_word`: Clears a 16-bit value at offset 1848 of the structure in a6 and returns if the value in d0 is less than the cleared value.
- `copy_and_duplicate_longwords`: Clears 16-bit values at offsets 1840 and 1842 of the structure in a6, then copies four longwords from the start of the structure in a6 to both offset 40 and offset 60 of the structure in a6.
- `build_data_tables`: Calculates a destination address based on a value at offset 2084 of the structure in a6 and a bit in d0, then fills two separate sequences of data into that destination and subsequent memory using values from the structure in a6 and various constants.
- `update_sprite_bitplanes`: Calculates a bitmask using bitwise rotations and logical operations based on values in a table, then performs a nested loop to update multiple bitplanes in a sprite buffer using bitwise logic.
- `test_and_branch_d0`: Tests the sign bit of a byte in d0 and branches if negative; otherwise, it compares d0 to 32 and branches if it is less than 32, then calls another routine and branches to a final destination.
- `subtract_and_branch`: Compares a byte in d0 to 8; if equal, it subtracts 4 from a word at offset 1890 of a structure in a6 and branches to another routine.
- `compare_and_add_constant`: Compares a byte in d0 to 9; if equal, it adds 4 to a word at offset 1890 of a structure in a6 and branches to another routine.
- `compare_and_add_word`: Compares d0 to 10 and, if they are equal, adds 8 to a word at offset 2916 of a structure in a6, then branches.
- `compare_byte_and_subtract`: Compares d0 to 11 and branches to another location if they are not equal, otherwise it subtracts 8 from a word at offset 2916 of the structure in a6 and branches to another location.
- `update_offset_field`: Compares d0 to 13; if not equal, it branches to another location, otherwise it adds 8 to a word at offset 2958 of a6, loads a word from offset 2976 of a6 into d0, writes d0 to offset 2956 of a6, and branches to another routine.
- `branch_to_offset_2`: Performs a short branch to an instruction at offset 14 bytes later.
- `update_offset_value`: Shifts the byte in d0 left and then right by one bit, extends it to a word, and adds the result to a 16-bit value at offset 1922 of the structure in a6.
- `sum_table_values`: Iterates through a buffer in a0, using each byte as an index into a table at a fixed address to accumulate a sum in d0 until a zero byte is encountered.
- `skip_null_terminated_strings`: Decrements d0 and enters a loop that iterates through a sequence of null-terminated strings in a0, skipping each one until d0 reaches zero.
- `process_command_stream`: Calls two routines, then enters a loop that reads a byte from a buffer and performs different actions based on its value: if zero it exits, if 31 it updates two words in a structure and calls a routine, if less than 31 it updates a word at offset 2898 and calls a routine, or otherwise it calls a routine and updates a word at offset 2898.
- `apply_bitmask_to_buffers`: Loads a byte from a table, uses it to calculate a bitmask via a rotation, and then applies this mask to five different buffers using bitwise AND and OR operations within a loop.
- `offset_five_pointers`: Calculates a 16-bit offset based on d7 and d6, then adds this offset to five pointers retrieved from the stack.
- `rotate_and_subtract_longs`: Performs a series of bit rotations on d5 followed by long-word subtractions using d1, d2, d3, d4, and d5.
- `increment_and_sync_values`: Loads three words from offsets 2892, 2894, and 2898 of a6, increments the first two and clears the third, calls a routine, restores the original values to those offsets, and calls the routine again.
- `copy_string_with_limit`: Copies bytes from a0 to a1 in a loop that runs at most 127 times or until a zero byte is found, then ensures the destination is null-terminated.
- `process_byte_stream`: Iterates through a buffer in a0, calling a routine if a byte is between 32 and 63, and if a byte is 31, it copies four bytes from the buffer to offsets 2904 through 2907 of a6.
- `update_sprite_index_and_offset`: Updates two 16-bit values at offsets 2908 and 2910 within a structure pointed to by a6 using an 8-bit input value.
- `store_and_check_byte`: Saves registers to the stack, sets a word at offset 2972 of a6 to 65535, clears d0, loads a byte from a pointer in a0 into d1, and if it is non-zero, masks d1, stores it in offset 2972 of a6, and if d1 is not 32, branches.
- `compare_word_and_assign`: Compares d1 to 45 and branches to another location if they are not equal, otherwise it sets d1 to 26 and branches to another location.
- `convert_character_to_index`: Checks if d1 is within a specific range, subtracts constants from it, and uses the result to fetch a byte from a table at offset 14898 of a6, adds the extended byte to d0, and performs a comparison against a table at a relative address.
- `extend_and_add`: Extends a word from offset 2 of a pointer in a1 to a longword and adds it to d0, then branches to an instruction at offset -30 bytes later.
- `restore_registers_and_return`: Restores several data and address registers from the stack and returns immediately.
- `process_interleaved_blocks`: Performs arithmetic on d2, d3, d4, and d5 to calculate loop parameters, then enters a loop that calls three different routines multiple times while updating d1, d2, and d5, finally calling four routines before returning.
- `copy_interleaved_data_blocks`: Calculates an offset based on input values and uses it to access five destination pointers from a structure in a6.
- `populate_transform_buffer`: Calculates a base address from a table and uses nested loops to call transform_word_buffer multiple times, passing different constant values in a register to modify data in a buffer.
- `transform_word_buffer`: Performs a bitwise transformation on five words by combining values from a source buffer and a register through a loop that runs five times.
- `copy_interleaved_data_2`: Copies words from a source buffer into five destination buffers using nested loops, where the inner loop copies five words and the outer loop iterates through the buffers.
- `decrement_source_counter`: Decrements a word at offset 2920 of a6, calculates an offset into a table at offset 2922 of a6, loads four words from that table, calls three routines, and returns.
- `copy_interleaved_data`: Copies words from a source buffer into five destination buffers using nested loops, where the inner loop copies five words and the outer loop manages the destination offsets.
- `copy_source_data`: Copies words from a source buffer into five destination buffers using nested loops, where the inner loop writes values from the source to the destinations and the outer loop manages the destination offsets.
- `transform_data_blocks_2`: Decrements a word at offset 2920 of a6, calculates an offset into a table at offset 2922 of a6, loads four words, and enters a loop that calls three routines while incrementing d1 until it reaches 16.
- `transform_data_blocks`: Performs a nested loop to process multiple data blocks, applying a bitwise logical transformation to five sets of 16-bit words using values from a source table and a control word.
- `execute_library_calls`: Checks if a value at offset 2352 of a structure in a6 is non-zero, then populates three structures with pointers and calls a routine at an offset from a6, and if a value at offset 2490 of a6 is non-zero, calls another routine at an offset from a6.
- `initialize_d0_and_call`: Sets d0 to zero and calls another routine before returning.
- `process_message_loop`: Iterates up to four times, checking a bit in a global variable and calling two different routines if the bit is set.
- `allocate_signal`: Calls a routine to allocate a signal, stores the result in a global variable at offset 144 of a6, and if the result is not negative one, calculates a bitmask, populates a structure at offset 152 of a6 with various values including a pointer from a global variable, and updates a linked list.
- `process_messages`: If a value at offset 186 of a6 is non-zero, it enters a loop that calls a routine to get a message and another to reply to it until the message is null, then clears the value at offset 186 and calls a routine if a value at offset 144 of a6 is not negative one.
- `update_structure_pointers`: Calls a routine at an offset from a6, writes 200 to offset 2878 of a6, and if the returned value is non-zero, updates a structure at the returned address with three values from that address and sets a function pointer to this routine.
- `clear_structure_pointer`: If the value at the address in a0 is non-zero, it copies the values at offsets 8 and 12 of a0 into offsets 176 and 178 of the structure pointed to by the value at a0, then clears the value at a0.
- `wait_for_message_loop`: Initializes a structure at offset 252 of a6, calls two routines, and then enters a loop that waits for messages and processes them by calling routines until a specific value is encountered or a message is null.
- `tiny_tst_rte`: Small routine of 2 instructions that tests (a7)+, then returns from the exception.
- `branch_to_address`: Performs a short branch to the address 43104.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `do_nothing`: Performs an bitwise OR operation on d0 with zero.
- `do_nothing_2`: Performs a bitwise OR operation on d0 with zero.
- `configure_audio_hardware`: Sets the Amiga audio registers for four channels by writing specific values for frequency, period, volume, and location from a table, and clears a word at an offset from a routine.
- `store_d0_to_memory`: Stores the word in d0 into a memory location at offset a7be.
- `initialize_global_tables`: Stores a word in a global variable, calls another routine, clears a 135-iteration loop in a global variable, and if a global pointer is non-zero, copies four longwords from a table into several global variables and sets eight other global variables to a specific address; finally, it sets four word-sized global variables to 1, 2, 3, and 4 before returning.
- `update_audio_registers`: Checks two global words, and if both are zero, it performs four consecutive calls to another routine to load specific values into a hardware register via a pointer, then clears a global word and returns.
- `test_and_return`: Tests a word at offset 0 of a structure in a0 and returns immediately if it is non-zero; otherwise it returns.
- `update_structure_and_decrement_counters`: Decrements a counter at offset 2 of a structure and, depending on its value, updates a pointer at offset 22, modifies values at offsets 44 and 48, or resets specific fields.
- `update_structure_values_3`: Increments a longword in a global variable, loads two words from a structure in a0 into d3 and d4, compares a word at offset 8 of the structure to 1, and if not equal, subtracts d3 from d2 and branches to a destination if d2 is less than d4.
- `update_values_and_decrement`: Adds d3 to d2, compares d2 to d4, and if d2 is not greater than or equal to d4, sets d2 to d4; it then clears a word at offset 8 of a structure in a0 and checks offsets 16 and 58 of that structure to conditionally decrement a word or branch to another routine.
- `decrement_if_nonzero`: Tests a word at offset 50 of a structure in a0; if it is zero, it branches to another routine, otherwise it decrements that word and branches to another routine.
- `update_structure_values_2`: Copies a word from offset 50 of a structure in a0 to offset 50 of the same structure, increments a word at offset 16 of a0, and if that word is not 3, subtracts a word from offset 56 of a0 from d2 and increments the word at offset 16 again.
- `update_counter_and_values`: Adds a word from offset 54 of a structure to d2, increments a word at offset 16 of the structure, and if that word reaches 5, resets it to 1; if a global variable is non-zero, it stores the result in d2 at offsets 6 of two different structures.
- `calculate_clamped_value_2`: Loads a pointer from offset 18 of a0, checks if the value at that pointer is 255, and if not, moves a word from the pointer to d0, subtracts a value from offset 62 of a0, and conditionally sets d0 to zero or a value from a global variable before storing it at offset 8 of a2.
- `decrement_and_move`: Decrements a long value in a1 by two, moves a word from the address pointed to by a1 into d0, and branches to another routine.
- `update_clamped_value`: Loads a pointer from offset 18 of a0, checks if the value at that pointer is 255, and if not, loads a word from the pointer, adds a value from offset 102 of a0 and a value from a global variable, and if the result is negative, stores zero at offset 8 of a2 and updates the pointer at offset 18 of a0.
- `calculate_clamped_value`: Adds a constant from a global variable to a 16-bit value from a pointer in a1, stores the result in a structure at offset 8 of a2 if the value is non-negative, and updates the pointer in a0 to point to the next element.
- `adjust_pointer_and_branch`: Decrements a1 by two and branches to another routine.
- `write_to_pointer`: Writes the value 3 to offset 4 of a structure in a0, moves a longword from offset 38 of a0 to the location pointed to by a2, and writes a word from offset 42 of a0 to offset 4 of the location pointed to by a2 before branching.
- `update_structure_and_branch`: Moves a long value from offset 44 of a0 to a2, moves a word from offset 48 of a0 to offset 4 of a2, decrements a word at offset 2 of a0, sets a word at offset 4 of a0 to one, and branches to another routine.
- `set_zero_and_load_pointer`: Stores zero at offset 66 of a0 and loads the value from offset 22 of a0 into a1.
- `process_data_table`: Loads a word from a pointer in a1, compares it to 100, and if it is less than or equal to 100, performs multiple data transfers between structures in a0 and a2, updates a counter at offset 2 in a0, and eventually branches to another routine or executes a jump table entry.
- `branch_to_offset_3`: Performs a relative branch to a specific offset.
- `update_structure_pointers_2`: Copies a word from a pointer in a1 to offset 2 of a structure in a0, decrements that word, stores the pointer from a1 into offset 16 of the structure in a0, stores the constant an address in offset 12 of the structure in a0, sets a word at offset 8 of a2 to 0, and branches to another routine.
- `decrement_on_condition`: Compares a word at offset 4 of a0 to zero; if not zero, it moves d5 to d1, decrements a word at offset 2 of a0, and branches to another routine.
- `execute_jump_table_entry`: Loads a pointer from a table at a3 using d0 as an index and jumps to that address.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `tiny_nop_rts`: Small routine of 2 instructions that does nothing for one instruction, then returns.
- `load_pointer_and_jump`: Loads a longword from the address pointed to by a1 into a1 and then branches to another routine.
- `clear_bit_and_jump`: Clears bit 1 of the CIAA_PRA register and branches to another routine.
- `set_bit_and_jump`: Sets bit 1 of the CIAA_PRA register and branches to another routine.
- `copy_data_to_structure`: Loads a pointer from a0 into a3, checks if the word at the address in a3 is zero, and if it is, copies a word from a3 to offset 4 of a structure in a0, copies a longword from a3 to a location in a2, and copies a word from a3 to offset 4 of the structure in a2 before jumping to another routine.
- `copy_data_structure`: Copies a word from a pointer in a3 to offset 4 of a structure in a0, three longwords from a3 to offsets 26, 2a, and 2e of a0, and then jumps to a long address.
- `check_and_increment_pointer`: Loads a longword from offset 26 of a0 into a1, compares the value in a1 to zero, branches if it is zero, otherwise adds 4 to offset 26 of a0, loads the value at the address in a1 into a1, and jumps to a long address.
- `transfer_pointer_data`: Loads a pointer from a1 into a4, copies a longword from the address in a4 to offset 3e of a0, copies the next longword from a4 to offset 22 of a0, and branches to a word address.
- `copy_two_longwords`: Copies a longword from a1 to offset 8 of a0, copies the next longword from a1 to offset c of a0, and branches to a word address.
- `set_flag_and_copy_longwords`: Sets a word at offset 10 of a0 to 1, copies three longwords from a1 to offsets 32, 36, and 3a of a0, and branches to a word address.
- `clear_offset_eight`: Clears a word at offset 8 of a0 and branches to a word address.
- `clear_offset_sixteen`: Clears a word at offset 10 of a0 and branches to a word address.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `set_offset_sixty_six`: Sets a word at offset 42 of a0 to 1 and branches to a word address.
- `increment_and_copy_pointer`: Adds 4 to a1 and copies a longword from offset 30 of a0 to offset 26 of a0.
- `advance_pointer_and_jump`: Loads a pointer from offset 26 of a0 into a1, adds 4 to offset 26 of a0, loads the value at the address in a1 into a1, and jumps to a long address.

### Formula One Grand Prix v1.02 (1991)(MicroProse)(Disk 1 of 4)[cr].adf:formula one grand prix

- `initialize_system_resources`: Clears a structure at a6, calls several routines via pointers in a6 to initialize various components, performs conditional calls to routines based on returned values, and finally calls several routines to clean up or finalize the state.
- `initialize_screen_and_window`: Copies a value from offset 20 of a6 to offset 68 of a6, calls a routine using a pointer from offset 20 of a6, and if the result is non-zero, calls another routine using a pointer from offset 20 of a6 and stores the result in offset 32 of a6.
- `close_intuition_resources`: If a pointer at offset 50 of a6 is non-zero, it calls a routine using a pointer from offset 20 of a6 and clears the value at offset 50; then, if a pointer at offset 162 of a6 is non-zero, it calls another routine using a pointer from offset 20 of a6 and clears the value at offset 162.

### Formula One Grand Prix v1.02 (1991)(MicroProse)(Disk 1 of 4)[cr].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
