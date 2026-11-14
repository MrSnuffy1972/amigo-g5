# Game 1054 (formula one grand prix v1.02 (1991)(microprose)(disk 1 of 4)[cr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01054). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 14 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 298 of 298 named (0 author symbols), 298 of 298 described (18 by rule, 165 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3244 of 3244 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6299 of 6299 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 794346 |
| code | 21524 |
| data | 71831 |
| padding | 2731 |
| unused | 10752 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Formula One Grand Prix v1.02 (1991)(MicroProse)(Disk 1 of 4)[cr].adf:frontend | 4 | 6091 | 293 |
| Formula One Grand Prix v1.02 (1991)(MicroProse)(Disk 1 of 4)[cr].adf:formula one grand prix | 1 | 197 | 3 |
| Formula One Grand Prix v1.02 (1991)(MicroProse)(Disk 1 of 4)[cr].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op ABAD) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 immediate

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
- `calculate_scaled_offsets`: Performs arithmetic on values from a structure in a2, including subtractions and logical shifts, and stores the results in offsets 2892, 2894, and 2898 of a6.
- `compare_and_call_subroutine`: Loads four words from a pointer in a2 into d0 through d3, sets d4, d5, and d6 to specific values, and if a bit in offset 44 of a2 is set, exchanges d4 and d6, calls a subroutine, and if a byte at offset 2968 of a6 matches a byte at offset 0 of a2, sets d5 to 16 and calls another subroutine.
- `set_flag_from_byte_value`: If the carry flag is set, stores 255 into the byte at the address in a0.
- `clear_bit_and_set_if_zero`: Branches to another routine when a condition holds; otherwise writes a zero byte through the pointer in a0 and branches onward to another routine.
- `set_flag_if_zero`: Clears bit 0 of the byte at offset 26 of the structure in a2, then sets that bit if the byte at the address in a0 is zero, and returns.
- `modify_registers_and_call`: Increments d0 and d1, decrements d2 and d3, calculates a value from d5 and a table at offset 80 of the structure in a6, and calls a subroutine using an address derived from that value.
- `apply_masked_updates`: Uses values from d4, d5, and d6 to fetch offsets from a table at offset 80 of a6, then performs five calls to different routines using those offsets and various register combinations.
- `draw_masked_range_twice`: Reads a short from a table at offset 80 of the structure in a6, indexed by twice the value in d4, and adds it to a fixed offset of 3428 to form a buffer address.
- `copy_block_of_data_structures`: Calculates a destination offset based on input values and performs a nested loop to copy blocks of five longwords from a source memory area to five separate destination buffers.
- `process_joystick_input`: Reads a byte from the CIAA register, performs bitwise manipulations and shifts on the value, and combines the results into a single byte returned in d0.
- `wait_for_input_bit`: Calls another routine and loops while bit 7 of the returned value in d0 is set.
- `poll_message_port_loop`: Waits for input, then repeatedly polls the joystick input routine and fetches messages from a message port, replying to each one and returning when a particular message value is seen or the joystick input routine signals an exit flag.
- `check_status_and_return_code`: Compares register d7 against the constant 1024 and, if equal, calls a status query routine and compares the returned byte with 32, otherwise it branches into another routine and returns.
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
- `initialise_sprite_record_from_table`: Fills a 32-byte record in a table, indexed by the word at the pointer in a0 times 32 plus a base offset from a6, with fixed values: a type of 3, copies of the words at a0 offsets 2 and 6 and a long from a second table entry, and zeros in the remaining fields.
- `initialise_sprite_record_from_table_2`: Fills a 32-byte record, located at an index taken from the word at a0 times 32 plus a6 plus 962, with a type value of 14 and copies of input words from a0, with some fields shifted left by 6 and one long word loaded from a table indexed by a0's fifth word times 4, then clears several fields.
- `init_object_slot_record`: Takes a 16-bit record from the pointer in a0 and fills a 32-byte slot in a table based at a6 plus 962, which is indexed by the first record word times 32.
- `clear_entry_and_set_marker`: Computes an entry address from the short at a0 times 32 plus a fixed offset in a6, clears the word at its start, and stores -1 into a word at offset 866 of a6, then returns.
- `copy_block_then_advance_pointer`: Reads a pointer from a table indexed by a word in the buffer at a0, and if the first word there equals 1 it copies 32 words from an offset-computed source block into a fixed global buffer at offset 3300 in a6.
- `clear_long_at_offset`: Stores zero into the longword at offset 1986 of the structure in a6, then returns.
- `set_word_and_copy_blocks`: Stores -1 into the word at offset 868 of a6, then saves a0, loads a word from offset 2864 of a6 into d0, calls the data block copy routine, and restores a0 before returning.
- `store_word_from_source`: Reads the word at a0, advances a0 by two bytes, and stores that word into the word at offset 1990 of the structure in a6, then returns.
- `store_scaled_position_and_steps`: Reads three words from the stream in a0, scales the first two by 64 and stores them at offsets 4 and 6 of an entry in a table at a6, then stores the third word as a divisor and computes per-step increments into offsets 8 and 10, or clears offsets 4 to 12 when the third word is zero.
- `compute_scaled_offsets_divide`: Stores the value in register d2 at offset 12 of the structure in a1, then subtracts the words at offsets 4 and 6 from registers d0 and d1, divides both differences by d2, and stores the quotients at offsets 8 and 10.
- `read_scaled_pair_into_frame_fields`: Reads two words from the stream in a0, shifts each left by 6, and stores them at offsets 832 and 834 of the structure in a6, then clears the word at offset 840 when the third word is zero, otherwise it branches into another routine.
- `compute_scaled_offsets_frame`: Stores the value in register d2 at offset 840 of the structure in a6, then subtracts the words at offsets 832 and 834 from registers d0 and d1, divides both differences by d2, and stores the quotients at offsets 836 and 838.
- `store_pointer_and_skip_string`: Reads a word from the stream in a0 and uses it to select a 32-byte slot at offset 962 of a6, stores the address following that word into the slot at offset 20, then advances a0 past the zero-terminated word sequence.
- `load_palette_and_init_state_words`: Clears a word in a6's structure and reads a word from the stream in a0; if that word is zero it copies 32 consecutive words into a palette table and into the color registers, then stores 65535 in a global word, otherwise it repeatedly calls a helper and sets two state words to 1 and 16 or calls another routine.
- `set_two_state_words`: Writes the constant 1 to the word at offset 3040 of the structure in a6 and the constant 16 to the word at offset 3038, then returns.
- `copy_32_words_to_palette`: Copies 32 consecutive words from the stream in a0 into a table at offset 3236 of a6 and into the color registers beginning at the custom chip base, then sets the word at offset 2830 to 65535 and calls another routine via a branch if the word at offset 872 is nonzero.
- `init_palette_and_copy_words`: Reads a word count from the buffer in a0; when it is nonzero it sets two global variables and repeatedly calls a routine until that returns zero, then copies the variable at offset 1 into a 32-entry block at offset 3236 and the colour register table, otherwise it writes 1 and 16 to globals and returns.
- `dispatch_on_leading_word_stream`: Clears the word at offset 3038 of a6 and copies the leading word from a0 into offset 3042; if that word is zero it copies 32 words from the table at offset 3300 into the color table and color registers, sets the word at offset 2830 to 65535, and conditionally calls a library routine, otherwise it loops over the stream calling a routine until zero and then sets two state words or calls a nested-loop routine.
- `execute_nested_loops`: Saves registers to the stack, calls another routine, then executes a loop that calls a library function a variable number of times based on a value at offset 3042 of the structure in a6, followed by a second loop that calls another routine, and finally sets a 16-bit word at offset 2830 of the structure in a6 to 0.
- `calculate_byte_differences`: Performs bitwise shifts and subtractions using values from a pointer in a1 and a value in d4, then stores the resulting six bytes into a destination pointed to by a2.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `fill_slot_from_two_tables`: Uses the word at a0 to select a 32-byte slot at offset 962 of a6, stores 9 in its second word and the word at a0 plus 2 in its first word, reads a long from a table indexed by the word at a0 plus 6, stores two words and a long derived from a second table entry into the slot, clears its word at offset 14, and sets the word at offset 866 to 65535.
- `fill_sprite_entry_from_table`: Reads four words from the structure at a0 and writes them into a 32-byte record in the table based at a6 plus 962, indexed by the first word, setting the record's second word to 12.
- `fill_slot_with_shifted_fields`: Uses the word at a0 to select a 32-byte slot at offset 962 of a6, stores 8 in its second word, copies the word at a0 plus 2 into the first word, shifts the words at a0 plus 4, 6, 8, and 10 left by 6 into four slot words, copies the word at a0 plus 12 into a slot word, clears six further slot words and the word at offset 14, and sets the word at offset 866 to 65535.
- `store_divisor_and_quotients`: Reads four words from the stream in a0 and locates a record in a6's table by the first word, then if the fourth word is nonzero stores it at offset 22 and writes the signed quotients of two scaled differences divided by it to offsets 18 and 20, otherwise it returns.
- `stores_quotients_and_divisor`: Writes the 16-bit divisor from register d2 to a structure at offset 22, then stores the signed quotients of d0 and d1 divided by that divisor at offsets 18 and 20 of the same structure in a1.
- `copies_word_into_frame_slot`: Reads one 16-bit word from the address in a0, advances a0 by two bytes, and stores the word at offset 1994 from the base in a6.
- `fills_record_from_five_words`: Computes a 32-byte record slot from the first word pointed to by a0 and the base in a6, stores the next words shifted left by 6 into fields at offsets 4, 6, 14 and 16, zeroes the remaining fields, and writes -1 to a global variable at offset 866.
- `fills_record_with_pointer_field`: Computes a 32-byte record slot from the first word pointed to by a0 and the base in a6, stores shifted coordinate words in two fields, stores the word at offset 8 multiplied by 6 as the return value, saves the address of the remaining data in a0 as a long at offset 14, zeroes four fields, and writes -1 to a global variable at offset 866.
- `fills_record_with_string_length`: Computes a 32-byte record slot from the base in a6 and the first word at a0, stores shifted coordinates and a flag value, saves a pointer to the null-terminated string that follows the header, counts that string's length into a field at offset 18, sets fixed fields at offsets 24, 26 and 28, and writes -1 to a global variable at offset 866.
- `set_record_type_ten_and_fill`: Reads a word from the stream in a0, uses it to index a 32-byte record in a6's table, writes the value 10 to the word at offset 2 of that record, then calls another routine to fill the record from the stream.
- `set_record_type_eleven_and_fill`: Reads a word from the stream in a0, uses it to index a 32-byte record in a6's table, writes the value 11 to the word at offset 2 of that record, then calls another routine to fill the record from the stream.
- `fills_record_with_string_and_code`: Builds a 32-byte record slot from the base in a6 and the first word at a0 using a code value of 16, stores shifted coordinates and a flag value, saves a pointer to the null-terminated string after the header, counts the string length into a field at offset 18, sets fixed fields at offsets 24, 26 and 28, and writes -1 to a global variable at offset 866.
- `fills_record_from_string_header`: Writes the first word from a0 into a record at a1, stores shifted values of the next two words in fields at offsets 4 and 6, saves a pointer to the string that follows as a long at offset 14, counts that string's length into a field at offset 18, sets fixed fields at offsets 24, 26 and 28, and writes -1 to a global variable at offset 866.
- `stores_two_words_in_record`: Computes a 32-byte record slot from the first word at a0 and the base in a6, then stores the second and third words from a0 into fields at offsets 24 and 26 of that slot.
- `ors_word_into_record_field`: Computes a 32-byte record slot from the first word at a0 and the base in a6, then ORs the second word from a0 into the field at offset 28 of that slot.
- `stores_word_into_global_variable`: Reads one 16-bit word from the address in a0, advances a0 by two bytes, and stores the word in a global variable at offset 2870 from the base in a6.
- `forward_word_to_clear`: Reads the word at the address in a0 and advances a0 by two bytes, then calls another routine that clears and compares a word, and returns.
- `start_audio_channel_playback`: Looks up an entry through a table indexed by the word in a0 and loads that channel's audio volume, period, pointer and length from the structure, then sets the channel's bit in the status byte at offset 2410 of the structure in a6 and enables that channel's DMA and interrupt bits in the custom chip registers.
- `clear_status_bit_from_word`: Reads the word at a0, advances a0 by two bytes, and clears the bit numbered by the low three bits of that word in the status byte at offset 2410 of the structure in a6.
- `disable_channel_interrupt_and_dma`: Clears the bit selected by the word at a0 in the status byte at offset 2410 of the structure in a6, then writes the interrupt-disable and DMA-disable values for that channel to the custom chip interrupt and DMA control registers.
- `run_frame_update_loop`: Repeatedly advances two position values in the structure in a6 by their step values while a counter at offset 840 is nonzero, copies data blocks or table entries depending on flags at offsets 1994 and 1978, copies a 400-word block pointed to by the global at offset 2080 into four destination pointers, dispatches a jump table of handlers, and loops until the joystick button check returns a value below zero.
- `shift_memory_block`: Shifts a block of memory starting at a0 by a length specified in d0, handling signed 16-bit values by either moving them or performing a block copy.
- `decrement_and_branch_loop`: Decrements a word at offset 22 of a structure in a1, and if it is not zero, it continues a loop that clears a word at offset 0 of a1, writes a 16-bit value of all ones to offset 546 of a6, and branches back to the start of the routine.
- `update_coordinate_offsets`: Copies a word from offset 20 of a1 to offset 22 of a1, performs arithmetic on values from offsets 4 and 6 of a1 using bit shifts and values from a6, stores the results in a6, calls another routine, and finally updates offsets 4 and 6 of a1.
- `advance_and_render_object_entries`: Decrements a counter at offset 12 of the structure at a1, adds the velocity fields at offsets 8 and 10 to the position fields at offsets 4 and 6 when that counter is nonzero, and when the flag at offset 30 is nonzero stores the scaled positions and two words into global variables before calling a test routine once per iteration in a loop that runs until a counter reaches minus one.
- `advances_moving_entries_and_stores`: Decrements a counter word in the structure at a1 and, while it is nonzero, adds two offset words to the position words, sets a flag word to minus one and loops over the next entries in the list, then stores a global variable into the flag word.
- `decrements_counter_and_resets_pointer`: Decrements the word at offset 22 of the structure in a1 and, when it has not yet reached zero, branches away; otherwise it reads a byte through the pointer at offset 14, clears the word at offset 0, stores the value 65535 at offset 866 of the structure in a6, and branches away, or continues when the byte read is zero.
- `copies_position_and_scales_by_sixty_four`: Copies a word from offset 20 to offset 22 of the structure at a1, shifts two position words right by 6 with an optional offset from globals, stores them along with two words from offset 24 and 26 into globals, calls another routine, then scales the globals by 64 and writes them back into the structure at a1.
- `waits_then_disables_interrupts`: If a global flag at offset 872 is nonzero, it waits until another flag at offset 2878 clears, calls two other routines, clears the flag at offset 872, and then forbids multitasking via the system library.
- `configure_hardware_and_call_library`: If a word at offset 872 of the structure in a6 is zero, it calls three subroutines, copies seven longwords from a structure to a destination, modifies the INTENA register to change interrupt settings, calls a library routine via a pointer, calls another library routine, and calls two more subroutines before setting the word at offset 872 to 65535.
- `loop_library_call`: Iterates 31 times, calling a function via a pointer at offset 2490 minus 288 from a6 with a value from a table at offset 3300 and a decreasing index.
- `sort_table_elements`: Performs a bubble sort on a table of 30 elements starting at offset 898 from a6 by comparing values against a reference table starting at offset 962 from a6.
- `transfer_index_and_call`: Pushes d0 to the stack, calls a routine, pops the value back into d0, calls another routine, and returns.
- `branch_to_offset`: Loads the address of the 16-bit word at offset 60 of the structure in a6 into a0 and branches to another routine.
- `copy_table_entries_to_pointers`: Loads five pointers from a structure at offset 40 of a6, calculates an index into a table using a value in d0, and then copies five 16-bit words from that table entry into the memory locations pointed to by the loaded pointers through a loop of 4000 iterations.
- `update_bitmapped_sprite_data`: Calculates an offset into a data table based on input values and performs a series of bitwise rotations and logical operations to update five consecutive memory locations using values fetched from a source buffer.
- `update_sprite_attribute_table`: Calculates offsets and bitmask values based on several input parameters and global variables, then enters a nested loop to perform bitwise operations on a series of 16-bit values stored in a table pointed to by a6.
- `clip_and_copy_scanline_block`: Clips a rectangle against two bounds stored in a structure at A6 offsets 2916 and 2918, then if any clipped height remains copies 16-bit words from the source in A0 into five separate destination planes (pointers at A6 offsets 40, 44, 48, 52 and 56), advancing the source by 5 words per group and by a row stride per row.
- `clip_and_merge_bitplane_rows`: Clips a rectangle against the bounds stored in a structure at offsets 2916 and 2918, then for each row combines a 6-word source pattern from a0 into five bitplane pointers (offsets 40 through 56 of the structure in a6) by AND-masking the destination with the first source word and OR-ing in the next one, advancing row by row.
- `copies_four_blocks_via_pointer`: Loads a word count of 4 and a pointer taken from a structure at offset 40 in a6 into a1, then calls the word-copy routine and repeats that call four times in a loop, with the pointer advanced after each call.
- `copies_four_offset_blocks`: Sets a counter to 4 and reads a pointer from a global variable at offset 2080 in a6, then calls the word-copy routine four times, advancing the destination pointer by 8000 bytes before each repeat.
- `copy_words_with_run_expansion`: Reads 16-bit words from the source pointer in a0 and writes them to the destination in a1 until 4000 destination words are filled.
- `process_byte_table`: Runs a loop 32 times where it reads three bytes from a table, adds each byte to its predecessor, and packs the results into a 16-bit word stored in another table.
- `fills_entries_from_nibble_pairs`: Walks a 32-entry table of 6-byte records in a6 at offset 3044, calling the nibble extractor twice per entry to write scaled byte values into the first five bytes and clearing the sixth byte of each record.
- `extract_low_nibble`: Reads a byte from the buffer in a0 and returns its low four bits as the result.
- `pack_nibbles_into_two_bytes`: Stores the byte in d0 at the location in a1, then stores into the following byte the value of d2 ORed with d1 shifted left by 4 bits, leaving a1 unchanged.
- `copy_32_words_to_color_registers`: Copies 32 words from a table at a6 offset 3236 into the hardware color palette registers starting at the color 0 register.
- `execute_subroutine`: Saves registers d0 through d7 and a0 through a3 to the stack, calls a routine, restores the registers, and returns.
- `update_parallel_data_streams`: Calculates offsets into five parallel data arrays based on input values and performs a bitwise update on 16-bit words using masks retrieved from tables.
- `update_buffer_range_with_masks`: Checks if a range defined by three values in a structure at a6 is valid, then updates five 16-bit words in a table at a1-a5 by applying bitwise masks derived from a lookup table.
- `update_masked_data_range`: Checks if a value is within a specific range, and if so, calculates an offset to update five 16-bit words in a table using a mask derived from a source table.
- `update_masked_data_wrapper`: Saves registers d0 through d3 and a0 through a1 on the stack, calls a routine that updates masked data, then restores those registers and returns.
- `update_masked_data_direct`: Checks if a value is within a specific range, and if so, updates five 16-bit words in a table using a mask retrieved from a table via an index.
- `blend_masked_tile_into_planes`: Checks that the coordinates in d0 and d1 lie within four bounds stored in a6-relative variables, and if they do, computes a screen offset and applies a masked word merge from a tile table into five bitplane pointers, each followed by a second word 20 words further on; otherwise it returns without writing.
- `handle_hardware_and_callbacks`: Increments a word at offset 2872 of a6, clears a word at offset 2868 of a6, and if a word at offset 2878 of a6 is non-zero, decrements it, writes a value to a hardware register at offset 128, calls a routine, and then calls a function pointer stored at offset 2842 of a6.
- `update_display_buffer`: Checks a word at offset 872 of a structure, and if it is zero, decrements a value at offset 3040 and potentially updates a value at offset 3042 to offset 3040.
- `handle_interrupt_logic`: Checks bits in a hardware register to determine an action, potentially increments two values at offsets 2872 and 2874 of a structure, and if a counter at offset 3038 is non-zero, performs a series of decrements and a memory copy loop to a fixed address.
- `handle_keyboard_serial_interrupt`: Checks the CIA-A interrupt control register for a serial data flag, and if set, toggles the serial handshake bit in the CIA-A control register on, waits a short countdown, then clears it.
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
- `dispatches_buffer_byte_by_range`: Reads the next byte from the buffer in a0 into d0, branches to the end path when it is zero, and calls a routine to handle the byte when it is at most 31 otherwise it branches to a later instruction.
- `copy_four_bytes_into_structure`: Branches away when the byte in d0 is not 30, otherwise it copies four consecutive bytes from the buffer at a0 into the structure at a6 at offsets 2882 through 2885, then branches onward to a shared routine.
- `stores_sign_extended_byte_at_offset_2888`: If the byte in d0 is not 30, it branches to another routine.
- `stores_sign_extended_byte_offset`: If the byte in d0 is not 29, it branches to another routine.
- `handles_carriage_return_code`: Compares the byte in d0 with 13 and, when it is equal, clears the word at offset 2882 and adds 8 to the word at offset 2884 in the structure in a6.
- `parses_control_codes_from_buffer`: Reads bytes from the buffer in a0 and stops at a zero byte; codes 29, 30 and 31 store the following bytes into global variables, code 13 resets one variable and adds 8 to another, and other bytes are passed to a drawing routine.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `swaps_word_around_call`: Swaps the two halves of d0, calls another routine, and then swaps the halves of d0 back.
- `rotates_word_around_subroutine`: Pushes the word in d0 onto the stack, rotates d0 right by eight bits, and calls another routine.
- `pushes_value_and_rotates_nibble`: Saves the word in d0 on the stack, rotates d0 right by four bits, calls another routine, and restores the saved word into d0.
- `compute_blitter_mask_and_draw`: Reduces the value in d0 to a 4-bit index, maps indices 10 and above by adding 7, then looks up a 32-bit value from a table at the structure in a6 to form a bit mask, which it shifts by a scroll offset read from the structure before combining it into a draw loop.
- `dispatches_byte_to_handler_or_skips`: Reads the next byte through a0 into d0 and branches away when it is zero; when the byte is 32 or greater it calls another routine with a0 and then branches back onward, otherwise it branches to the other routine as the next step.
- `copies_four_bytes_to_offset_2892`: Compares d0 with 31 and branches away when they differ; when equal it copies four consecutive bytes from the source pointer in a0 to the destination starting at offset 2892 of the structure in a6, advancing both pointers, then branches back onward.
- `stores_sign_extended_byte_at_offset_2898`: Compares d0 with 30 and branches away when they differ; when equal it reads the next byte through a0, sign-extends it to a word, and stores it at offset 2898 of the structure in a6, then branches back onward.
- `stores_sign_extended_byte_at_offset_2896`: Compares d0 with 29 and branches away when they differ; when equal it reads the next byte through a0, sign-extends it to a word, and stores it at offset 2896 of the structure in a6, then branches back onward.
- `clears_word_and_sets_word_on_byte_twenty`: Compares the byte in d0 with 28 and branches away when it differs, otherwise it clears the word at offset 2896 of the structure in a6, sets the word at offset 2898 to 31 and branches to a later instruction.
- `copies_two_bytes_on_escape_code`: Compares the byte in d0 with 27 and branches away when it differs, otherwise it copies the next two bytes from the buffer in a0 into the structure at a6 at offset 2892 and 2893 and branches to a later instruction.
- `clears_word_and_adds_twelve_on_byte`: Compares the byte in d0 with 13 and branches away when it differs, otherwise it clears the word at offset 2924 of the structure in a6, adds 12 to the word at offset 2926 and branches to a later instruction.
- `decrement_offset_on_byte_eleven`: Compares the byte in d0 with 11 and branches away when it differs, otherwise it subtracts 8 from the word at offset 2926 of the structure in a6 and branches onward to a shared routine.
- `adds_offset_or_branches_onward`: If the byte in d0 is not 10, it branches to another routine.
- `adjusts_offset_or_branches`: If the byte in d0 is not 9, it branches to another routine.
- `decrements_horizontal_offset_word`: If the byte in d0 equals 8, it subtracts 8 from the word at offset 2892 in the structure in a6.
- `parses_positioning_control_codes`: Reads bytes from the buffer in a0 and stops at a zero byte; codes 27, 28 and 29 store following bytes into global variables, codes 8 through 13 adjust two global variables by fixed offsets, and code 28 resets variables to zero and 31.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `sum_table_values`: Iterates through a buffer in a0, using each byte as an index into a table at a fixed address to accumulate a sum in d0 until a zero byte is encountered.
- `skip_null_terminated_strings`: Decrements d0 and enters a loop that iterates through a sequence of null-terminated strings in a0, skipping each one until d0 reaches zero.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (ori.l, clr.w) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_4`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_5`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `saves_registers_and_decomposes_value`: Ors the low byte of d0 with 10, saves registers d0 through d4 on the stack, loads the address of a table into a0, sets d4 to 6, and branches into another routine's shared code.
- `saves_registers_and_decomposes_five`: Saves registers d0 through d4 on the stack, loads the address of a table into a0, sets d4 to 5, and branches into another routine's shared code.
- `saves_registers_and_decomposes_four`: Saves registers d0 through d4 on the stack, loads the address of a table into a0, sets d4 to 4, and branches into another routine's shared code.
- `saves_registers_and_decomposes_three`: Saves registers d0 through d4 on the stack, loads the address of a table into a0, sets d4 to 3, and branches into another routine's shared code.
- `masks_word_and_decomposes_two`: Saves registers d0 through d4 on the stack, masks d0 to its low 16 bits, loads the address of a table into a0, sets d4 to 2, and branches into another routine's shared code.
- `masks_word_and_decomposes_one`: Saves registers d0 through d4 on the stack, masks d0 to its low 16 bits, loads the address of a table into a0, sets d4 to 1, and branches into another routine's shared code.
- `decomposes_value_into_signed_bytes`: Takes the word in d0 and repeatedly subtracts entries from a table of 32-bit values, storing each negated count as a byte at the buffer in a1 and writing the remainder plus offset and a zero byte at the end.
- `clamps_and_stores_byte_pair`: Compares the word in d0 with 10 and, when it is not below 10, branches to another routine.
- `saves_registers_and_loads_table`: Saves registers d0 through d4 on the stack, loads the address of a table into a0, sets d4 to 6, and branches into the middle of another routine.
- `load_table_base_and_branch`: Saves registers d0 through d4 on the stack, loads the address of a table into a0, sets d4 to 5, and branches to a shared routine further on.
- `save_registers_set_count_four`: Saves registers d0 through d4 on the stack, loads a table address into a0, sets d4 to 4, and branches to a shared routine further on.
- `save_registers_set_count_three`: Saves registers d0 through d4 on the stack, loads a table address into a0, sets d4 to 3, and branches to a shared routine further on.
- `mask_low_word_set_count_two`: Saves registers d0 through d4 on the stack, clears the upper 16 bits of d0 so only its low word remains, loads a table address into a0, sets d4 to 2, and branches to a shared routine further on.
- `mask_low_word_set_count_one`: Saves registers d0 through d4 on the stack, clears the upper 16 bits of d0, loads a table address into a0, sets d4 to 1, and branches to a shared routine further on.
- `decodes_table_values_into_bytes`: Reads 32-bit values from a table in a loop, repeatedly subtracting them from the word in d0 and counting how many subtractions succeed for each of several byte positions.
- `clamp_offset_and_blit_rows`: Compares the word in d0 with 10 and branches away when it is 10 or higher, otherwise it adds 128 to d0 and calls the masked bitplane blit routine, then returns.
- `advance_offset_and_blit_rows`: Compares the word in d0 with 10 and branches away when it is 10 or higher, otherwise it adds 8 to the word at offset 2924 of the structure in a6, adds 128 to d0, calls the masked bitplane blit routine, and returns.
- `adds_eight_to_word_when_below_hundred`: Compares the word in d0 with 100 and branches away when it is 100 or higher, otherwise it adds 8 to the word at offset 2924 of the structure in a6 and branches to a later instruction.
- `dispatches_high_byte_with_range_check`: Tests the byte in d0 for negative sign and branches away when it is set, otherwise it compares the byte with 32 and branches away when it is below 32, otherwise it calls another routine and branches to a later instruction.
- `subtracts_eight_on_byte_eight`: Compares the byte in d0 with 8 and branches away when it differs, otherwise it subtracts 8 from the word at offset 2924 of the structure in a6 and branches to a later instruction.
- `adds_eight_to_word_on_byte_nine`: Compares the byte in d0 with 9 and branches away when it differs, otherwise it adds 8 to the word at offset 2926 of the structure in a6 and branches to a later instruction.
- `advance_offset_on_byte_eleven`: Compares the byte in d0 with 11 and branches away when it differs, otherwise it adds 8 to the word at offset 2926 of the structure in a6 and branches onward to a shared routine.
- `decrements_offset_or_branches`: If the byte in d0 is not 11, it branches to another routine.
- `branches_to_sign_extension_path`: If the byte in d0 is not 13, it branches to the sign-extension path of another routine.
- `branches_into_sign_extension_code`: Contains a single unconditional branch to a point inside another routine, with no other instructions.
- `accumulate_sign_extended_byte`: Takes the low byte of d0, shifts it left then arithmetically right by one to sign-extend bit 6 while dropping bit 7, widens it to a 16-bit word, and adds that word to the 16-bit global variable at offset 2892 from a6, storing the sum back there before returning.
- `blit_masked_bitplane_rows`: Reads 1-bit mask and source bytes from a table and a global variable, shifts them by a horizontal bit offset held in the structure at a6, and merges them with four bitplane rows of 20-word stride using a masked combine of the form dest = (src & mask) / (dest & ~mask) per plane.
- `process_command_stream`: Calls two routines, then enters a loop that reads a byte from a buffer and performs different actions based on its value: if zero it exits, if 31 it updates two words in a structure and calls a routine, if less than 31 it updates a word at offset 2898 and calls a routine, or otherwise it calls a routine and updates a word at offset 2898.
- `apply_bitmask_to_buffers`: Loads a byte from a table, uses it to calculate a bitmask via a rotation, and then applies this mask to five different buffers using bitwise AND and OR operations within a loop.
- `offset_five_pointers`: Calculates a 16-bit offset based on d7 and d6, then adds this offset to five pointers retrieved from the stack.
- `rotate_and_subtract_longs`: Performs a series of bit rotations on d5 followed by long-word subtractions using d1, d2, d3, d4, and d5.
- `increment_and_sync_values`: Loads three words from offsets 2892, 2894, and 2898 of a6, increments the first two and clears the third, calls a routine, restores the original values to those offsets, and calls the routine again.
- `copy_string_with_limit`: Copies bytes from a0 to a1 in a loop that runs at most 127 times or until a zero byte is found, then ensures the destination is null-terminated.
- `process_byte_stream`: Iterates through a buffer in a0, calling a routine if a byte is between 32 and 63, and if a byte is 31, it copies four bytes from the buffer to offsets 2904 through 2907 of a6.
- `update_sprite_index_and_offset`: Updates two 16-bit values at offsets 2908 and 2910 within a structure pointed to by a6 using an 8-bit input value.
- `store_and_check_byte`: Saves registers to the stack, sets a word at offset 2972 of a6 to 65535, clears d0, loads a byte from a pointer in a0 into d1, and if it is non-zero, masks d1, stores it in offset 2972 of a6, and if d1 is not 32, branches.
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
- `submit_io_request_and_clear`: Stores a pointer to a buffer and the value 10 and 13 into fields of a request structure in a6, submits it through the library's I/O call, and if the result is nonzero zeroes 12 bytes starting at the buffer address before returning.
- `polls_bit_array_until_clear`: Calls another routine repeatedly and scans a bit array in a global structure at offset 2606, setting a flag if any bit at positions other than the first 98 is set, and repeats until no such bit is found.
- `returns_flag_from_status_byte`: Calls another routine, then tests bit 5 of the byte at offset 2614 in a6 and returns 65535 if it is set, otherwise returns 0.
- `call_message_loop_handler`: Calls the routine that processes the message loop and then returns.
- `initialize_d0_and_call`: Sets d0 to zero and calls another routine before returning.
- `process_message_loop`: Iterates up to four times, checking a bit in a global variable and calling two different routines if the bit is set.
- `allocate_signal`: Calls a routine to allocate a signal, stores the result in a global variable at offset 144 of a6, and if the result is not negative one, calculates a bitmask, populates a structure at offset 152 of a6 with various values including a pointer from a global variable, and updates a linked list.
- `process_messages`: If a value at offset 186 of a6 is non-zero, it enters a loop that calls a routine to get a message and another to reply to it until the message is null, then clears the value at offset 186 and calls a routine if a value at offset 144 of a6 is not negative one.
- `update_structure_pointers`: Calls a routine at an offset from a6, writes 200 to offset 2878 of a6, and if the returned value is non-zero, updates a structure at the returned address with three values from that address and sets a function pointer to this routine.
- `clear_structure_pointer`: If the value at the address in a0 is non-zero, it copies the values at offsets 8 and 12 of a0 into offsets 176 and 178 of the structure pointed to by the value at a0, then clears the value at a0.
- `wait_for_message_loop`: Initializes a structure at offset 252 of a6, calls two routines, and then enters a loop that waits for messages and processes them by calling routines until a specific value is encountered or a message is null.
- `tiny_tst_rte`: Small routine of 2 instructions that tests (a7)+, then returns from the exception.
- `branch_to_address`: Performs a short branch to the address 43104.
- `jump_to_configure_audio_hardware`: Jump stub: passes control straight on to configure_audio_hardware.
- `jump_to_configure_audio_hardware_2`: Jump stub: passes control straight on to configure_audio_hardware.
- `jump_to_configure_audio_hardware_3`: Jump stub: passes control straight on to configure_audio_hardware.
- `configure_audio_hardware`: Sets the Amiga audio registers for four channels by writing specific values for frequency, period, volume, and location from a table, and clears a word at an offset from a routine.
- `test_and_return`: Tests a word at offset 0 of a structure in a0 and returns immediately if it is non-zero; otherwise it returns.
- `calculate_clamped_value_2`: Loads a pointer from offset 18 of a0, checks if the value at that pointer is 255, and if not, moves a word from the pointer to d0, subtracts a value from offset 62 of a0, and conditionally sets d0 to zero or a value from a global variable before storing it at offset 8 of a2.
- `decrement_and_move`: Decrements a long value in a1 by two, moves a word from the address pointed to by a1 into d0, and branches to another routine.
- `process_data_table`: Loads a word from a pointer in a1, compares it to 100, and if it is less than or equal to 100, performs multiple data transfers between structures in a0 and a2, updates a counter at offset 2 in a0, and eventually branches to another routine or executes a jump table entry.
- `update_structure_pointers_2`: Copies a word from a pointer in a1 to offset 2 of a structure in a0, decrements that word, stores the pointer from a1 into offset 16 of the structure in a0, stores the constant an address in offset 12 of the structure in a0, sets a word at offset 8 of a2 to 0, and branches to another routine.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `tiny_nop_rts`: Small routine of 2 instructions that does nothing for one instruction, then returns.
- `load_pointer_and_jump`: Loads a longword from the address pointed to by a1 into a1 and then branches to another routine.
- `clear_bit_and_jump`: Clears bit 1 of the CIAA_PRA register and branches to another routine.
- `set_bit_and_jump`: Sets bit 1 of the CIAA_PRA register and branches to another routine.
- `copy_data_to_structure`: Loads a pointer from a0 into a3, checks if the word at the address in a3 is zero, and if it is, copies a word from a3 to offset 4 of a structure in a0, copies a longword from a3 to a location in a2, and copies a word from a3 to offset 4 of the structure in a2 before jumping to another routine.
- `check_and_increment_pointer`: Loads a longword from offset 26 of a0 into a1, compares the value in a1 to zero, branches if it is zero, otherwise adds 4 to offset 26 of a0, loads the value at the address in a1 into a1, and jumps to a long address.
- `transfer_pointer_data`: Loads a pointer from a1 into a4, copies a longword from the address in a4 to offset 3e of a0, copies the next longword from a4 to offset 22 of a0, and branches to a word address.
- `copy_two_longwords`: Copies a longword from a1 to offset 8 of a0, copies the next longword from a1 to offset c of a0, and branches to a word address.
- `set_flag_and_copy_longwords`: Sets a word at offset 10 of a0 to 1, copies three longwords from a1 to offsets 32, 36, and 3a of a0, and branches to a word address.
- `clear_offset_eight`: Clears a word at offset 8 of a0 and branches to a word address.
- `clear_offset_sixteen`: Clears a word at offset 10 of a0 and branches to a word address.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `set_offset_sixty_six`: Sets a word at offset 42 of a0 to 1 and branches to a word address.
- `advance_pointer_and_jump`: Loads a pointer from offset 26 of a0 into a1, adds 4 to offset 26 of a0, loads the value at the address in a1 into a1, and jumps to a long address.

### Formula One Grand Prix v1.02 (1991)(MicroProse)(Disk 1 of 4)[cr].adf:formula one grand prix

- `initialize_system_resources`: Clears a structure at a6, calls several routines via pointers in a6 to initialize various components, performs conditional calls to routines based on returned values, and finally calls several routines to clean up or finalize the state.
- `initialize_screen_and_window`: Copies a value from offset 20 of a6 to offset 68 of a6, calls a routine using a pointer from offset 20 of a6, and if the result is non-zero, calls another routine using a pointer from offset 20 of a6 and stores the result in offset 32 of a6.
- `close_intuition_resources`: If a pointer at offset 50 of a6 is non-zero, it calls a routine using a pointer from offset 20 of a6 and clears the value at offset 50; then, if a pointer at offset 162 of a6 is non-zero, it calls another routine using a pointer from offset 20 of a6 and clears the value at offset 162.

### Formula One Grand Prix v1.02 (1991)(MicroProse)(Disk 1 of 4)[cr].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
