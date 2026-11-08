# Game 1169 (fiendish freddy's big top 'o fun (demo) (1990)(mindscape)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01169). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1392688 top-level bytes in 41 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 245 of 245 named (12 author symbols), 245 of 245 described (20 by rule, 225 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 164188 of 164189 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 167483 of 167569 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 481904 |
| code | 508144 |
| data | 121257 |
| padding | 10023 |
| unused | 271360 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Fiendish Freddy's Big Top 'O Fun (demo) (1990)(Mindscape).adf:demo | 1 | 96 | 9 |
| Fiendish Freddy's Big Top 'O Fun (demo) (1990)(Mindscape).adf:circus | 1 | 2792 | 182 |
| Fiendish Freddy's Big Top 'O Fun (demo) (1990)(Mindscape).adf:songs | 3 | 90 | 8 |
| Fiendish Freddy's Big Top 'O Fun (demo) (1990)(Mindscape).adf:songplay | 3 | 653 | 36 |
| Fiendish Freddy's Big Top 'O Fun (demo) (1990)(Mindscape).adf:tsongs | 3 | 90 | 8 |
| Fiendish Freddy's Big Top 'O Fun (demo) (1990)(Mindscape).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 163838 | 1 |

## Cases handled

- fiendish freddy's big top 'o fun (demo) (1990)(mindscape)/Fiendish Freddy's Big Top 'O Fun (demo) (1990)(Mindscape).adf:freddy2.cel: high-entropy data file (7.82 bits/byte), classified as packed data
- fiendish freddy's big top 'o fun (demo) (1990)(mindscape)/Fiendish Freddy's Big Top 'O Fun (demo) (1990)(Mindscape).adf:deed2.cel: high-entropy data file (7.79 bits/byte), classified as packed data
- fiendish freddy's big top 'o fun (demo) (1990)(mindscape)/Fiendish Freddy's Big Top 'O Fun (demo) (1990)(Mindscape).adf:freddy1.cel: high-entropy data file (7.84 bits/byte), classified as packed data
- fiendish freddy's big top 'o fun (demo) (1990)(mindscape)/Fiendish Freddy's Big Top 'O Fun (demo) (1990)(Mindscape).adf:deed3.cel: high-entropy data file (7.68 bits/byte), classified as packed data
- fiendish freddy's big top 'o fun (demo) (1990)(mindscape)/Fiendish Freddy's Big Top 'O Fun (demo) (1990)(Mindscape).adf:deed1.cel: high-entropy data file (7.86 bits/byte), classified as packed data
- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 00C0) A7=an address (A7)=an address, slid to exec.library -an address): 26 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 163839 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Fiendish Freddy's Big Top 'O Fun (demo) (1990)(Mindscape).adf:demo

- `initialize_hardware_and_segments`: Sets specific values in the DMACON and INTENA registers, calls a routine to open a library, and executes several routines that load and process segments.
- `load_and_execute_segment_2`: Stores d1 into a global variable, checks a condition, calls two routines, calls a DOS routine to load a segment, calls a routine from a table using a modified d0, and finally calls a DOS routine to unload the segment.
- `call_open_library`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `calculate_segment_offset`: Calls a DOS routine, shifts the returned value left by two bits, adds four, and stores the result in a global variable at offset 1e2.
- `load_and_execute_segment`: Calls a DOS routine, stores the result in a global variable at offset 1ea, calculates a new address by shifting the result left by two bits and adding four, stores this address in a global variable at offset 1e6, calls the routine at that address, and finally stores the returned address from a0 back into the global variable at offset 1e6.
- `execute_calculated_address`: Calls a DOS routine, stores the result in a global variable at offset 1f2, calculates a new address by shifting the result left by two bits and adding four, stores this address in a global variable at offset 1ee, calls the routine at that address, and finally stores the returned address from a0 back into the global variable at offset 1ee.
- `call_unloading_routine`: Calls a routine at a fixed address, then calls a DOS routine to unload a segment using a pointer from a global variable.
- `call_stored_routine_twice`: Stores a 16-bit value from d5 into a global variable at offset 1c6, then calls a routine stored at offset 1e2 twice, using the value from offset 1c6 to set d1 during the second call.
- `call_routine_with_parameter`: Sets d0 to three and calls a routine stored at offset 1e2.

### Fiendish Freddy's Big Top 'O Fun (demo) (1990)(Mindscape).adf:circus

- `initialize_asset_and_graphics_tables_2`: Performs a sequence of subroutine calls to initialize various tables, including a loop that iterates through a table up to thirteen entries to update values based on nibble comparisons.
- `update_sprite_and_index_tables`: Decrements a global 16-bit value by 4 until it reaches zero, then iterates through a table of 8 entries to update sprite positions and dimensions.
- `update_sprite_position`: Updates a 16-bit value at offset 6 of a structure in a0 by adding a value derived from offset 2, applies bounds checking to negate the value if it exceeds certain limits, and calls another routine before incrementing a counter in a global variable.
- `render_sequence_with_bitmaps`: Calls several routines to initialize state, performs four specific bitmap blit operations using a routine with different parameters, and then enters a loop that decrements a global counter and performs multiple routine calls.
- `process_table_data`: Iterates through a table using a counter, performs checks on values retrieved from the table, calls several routines, and updates a global pointer.
- `initialize_index_table_and_update_values`: Inverts a series of 16-bit values from a table into a destination buffer using a loop, then enters a nested loop that iterates through eight entries to increment or decrement two separate 16-bit values within a structure based on the contents of a table, finally updating two global variables with the results and calling another routine.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_handler_twice`: Stores a 16-bit value from d5 into a global variable, then calls a routine from a table twice with d0 set to 1 and 2 respectively.
- `execute_handler_twice_offset_9fc`: Stores a 16-bit value from d5 into a global variable, then calls a routine from a table twice with d0 set to 1 and 2 respectively using a different pointer table.
- `execute_handler_loop`: Calls a routine with d0 set to 6, then calls it with d0 set to 5 in a loop while d0 is non-zero, and finally calls it with d0 set to 3.
- `execute_repeated_subroutine`: Calls five different routines in succession and then returns.
- `execute_cleanup_sequence`: Calls five different routines in succession and returns.
- `setup_graphics_environment`: Calls several library functions to initialize a view and raster port, clears a block of memory, and updates several global variables with calculated values.
- `allocate_and_initialize_rasters`: Allocates memory for multiple rasters using a library call, calculates several memory addresses by adding increments of 8000 to a base value, and stores these addresses in a series of global variables.
- `allocate_and_clear_raster`: Calls a routine to allocate a raster, stores the result in a global variable, and then calls another routine to clear a portion of that raster using a specific size and bitmask.
- `allocate_and_clear_large_raster`: Calls a routine to allocate a raster, stores the result in a global variable, and then calls another routine to clear a larger portion of that raster using a specific size and bitmask.
- `free_all_graphics_resources`: Calls several routines to free various pointers stored in a table, then calls a routine to clear a specific area and finally calls another routine to free a raster.
- `call_free_raster`: Library stub: loads its arguments and the graphics library base, calls FreeRaster and returns its result to the caller.
- `call_free_raster_2`: Library stub: loads its arguments and the graphics library base, calls FreeRaster and returns its result to the caller.
- `initialize_memory_and_pointers`: Calls routines to allocate three different memory blocks, stores the results in global variables, calls a library routine, and then populates four global pointers with values from a structure and offset increments.
- `close_library_pointers`: Calls a routine three times, passing three different global pointers as arguments to close the associated resources.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `clear_d0_byte`: Performs a bitwise OR on d0 with zero.
- `clear_data_register_zero`: Performs a logical OR operation on the lowest byte of data register zero with zero.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `clear_d0_register`: Performs a logical OR operation on the d0 register with zero.
- `clear_d0_register_2`: Clears the least significant byte of register d0.
- `clear_d0_register_3`: Clears the least significant byte of the d0 data register.
- `perform_logical_or`: Performs a bitwise OR operation on register d0 with zero.
- `perform_four_logical_ors`: Performs four consecutive bitwise OR operations on register d0 with zero.
- `perform_two_logical_ors`: Performs two consecutive bitwise OR operations on register d0 with zero.
- `clear_d0_register_4`: Performs a logical OR operation on the least significant byte of register d0 with zero.
- `clear_d0_register_5`: Clears the least significant byte of the d0 register.
- `clear_d0_register_6`: Sets the least significant byte of the d0 register to zero.
- `clear_data_register_d0`: Performs a logical OR operation on data register d0 with zero.
- `clears_data_byte`: Clears the lowest byte of register d0 four times in succession.
- `clears_data_register_d0`: Sets the least significant byte of data register d0 to zero twice.
- `clear_d0_register_7`: Performs a logical OR operation on the least significant byte of the d0 register with zero.
- `clear_d0_register_8`: Sets the least significant byte of the d0 data register to zero.
- `clear_data_register_zero_2`: Clears the least significant byte of data register zero using a logical OR operation with zero.
- `clear_data_register_zero_3`: Performs a logical OR operation on data register zero with zero.
- `clear_d0_byte_quad`: Performs a bitwise OR operation on register d0 with zero four times.
- `clear_d0_byte_double`: Performs a bitwise OR operation on register d0 with zero two times.
- `clear_d0_byte_2`: Performs a bitwise OR operation on the least significant byte of register d0 with zero.
- `clear_data_register_zero_4`: Performs a logical OR operation on the least significant byte of data register zero with zero.
- `clear_d0_register_9`: Sets the least significant byte of register d0 to zero.
- `clear_d0_register_10`: Performs a logical OR operation on the d0 register using a zero constant.
- `clears_data_register_d0_2`: Performs four consecutive logical OR operations with zero on register d0.
- `clear_d0_twice`: Performs two logical OR operations with zero on register d0.
- `clear_d0_once`: Performs a single logical OR operation with zero on register d0.
- `apply_zero_to_d0`: Performs a logical OR operation with zero on register d0.
- `clear_data_register_zero_5`: Clears the least significant byte of data register zero.
- `clear_data_register_d0_2`: Clears the least significant byte of data register d0.
- `clears_d0_register`: Performs four consecutive logical OR operations with zero on the d0 register.
- `clears_data_register_d0_3`: Performs two consecutive logical OR operations with zero on data register d0.
- `clear_d0_register_15`: Performs a logical OR operation on the d0 register with zero, effectively clearing the register.
- `clear_data_register_zero_6`: Performs a bitwise OR operation on data register zero with zero.
- `clear_data_register_zero_7`: Performs a logical OR operation with zero on data register zero.
- `clear_data_register_d0_3`: Performs a logical OR operation on register d0 with zero.
- `repeat_or_data_d0`: Performs the bitwise OR operation on register d0 with zero four times.
- `clear_d0_register_11`: Performs a logical OR operation on the least significant byte of the d0 register using zero.
- `repeat_or_data_d0_five`: Performs the bitwise OR operation on register d0 with zero five times.
- `clear_d0_register_12`: Performs a logical OR operation on the data register zero with zero.
- `nullify_d0_register`: Performs a logical OR operation on the data register zero with zero, effectively ensuring the register contains zero.
- `zero_d0_register`: Performs a bitwise OR operation on the contents of register d0 with zero.
- `clear_d0_register_13`: Performs a logical OR operation on the data register 0 with zero.
- `clears_data_register_zero`: Performs a logical OR operation on register d0 using zero as a constant.
- `clear_data_byte`: Performs a logical OR operation on a byte in a data register with zero, resulting in no change to the register's value.
- `fill_zero_data`: Performs nine consecutive logical OR operations with zero on register d0.
- `fill_zero_data_long`: Performs twenty-six consecutive logical OR operations with zero on register d0.
- `update_interrupt_vectors`: Calls a routine, updates two global variables with new values, and calls another routine before returning.
- `update_interrupt_state`: Calls a routine to disable interrupts, moves a value from a global variable to another, and then calls a routine to configure interrupts.
- `check_joystick_bits`: Saves the current data register to the stack, reads the interrupt enable register, and branches based on the status of specific bits, potentially incrementing a global counter and calling another routine.
- `process_joystick_input`: Reads a long value from the joystick port, performs several bitwise tests and shifts to calculate a bitmask, and stores the resulting word into a global variable before returning.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `disable_interrupts`: Writes the value 16384 to the interrupt enable register.
- `configure_interrupts`: Writes the value 49152 to the interrupt enable register.
- `compare_and_branch`: Adds the value in register d0 to a long value from a global variable, compares the result to the original global value, and branches if the result is greater.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `dispatch_data_type`: Stores a word from d0 into a global variable, calls a routine, and then branches to different routines based on whether the value in the global variable is 0, 1, or 2.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `free_if_pointer_exists`: Checks if a global pointer is non-zero; if so, it calls a routine using the pointer and then clears the global variable.
- `process_file_and_data`: Calls a routine to read data, then performs a series of checks on the loaded data to identify specific magic numbers, potentially executing a loop that processes bytes into a table or calls a routine to close a file or free memory.
- `copy_compressed_data_blocks`: Initializes a memory location with a constant value and then performs a multi-level loop to copy data from a source buffer to a destination buffer, supporting both standard byte-by-byte copying and a compressed format where bytes represent the length of subsequent data runs.
- `fill_zero_data_short`: Performs two consecutive logical OR operations with zero on register d0.
- `fill_zero_data_triple`: Performs three consecutive logical OR operations with zero on register d0.
- `clear_data_register_d0_4`: Performs a bitwise OR operation on data register d0 with zero.
- `zero_data_padding`: This routine contains a single instruction that performs a logical OR operation on a data register with zero, effectively leaving the register unchanged.
- `clear_d0_register_16`: Clears the d0 register by performing a bitwise OR operation with zero.
- `clear_data_register_zero_8`: Performs a bitwise OR operation with zero on data register zero.
- `clear_data_register_d0_5`: Performs a logical OR operation on data register d0 with zero, effectively clearing the register.
- `perform_null_operations`: Performs a sequence of logical OR operations with zero on register d0.
- `call_indexed_subroutine`: Calculates two different addresses by adding scaled versions of d7 to two tables, loads them into a0 and a2, and calls a subroutine at the address in a2 with d0 set to 7, d1 and d2 loaded from the second table, and d4 set to 255.
- `call_indexed_subroutine_alt`: Calculates two different addresses by adding scaled versions of d7 to two tables, loads them into a0 and a2, and calls a subroutine at the address in a2 with d0 set to 7, d1 and d2 loaded from the second table, and d4 set to 127.
- `iterate_file_table`: Enters a loop that reads a word from a global variable, uses it to fetch a longword and a word from two different tables, calls a routine, increments the global variable, and repeats until the variable reaches 4.
- `perform_file_operations`: Calls a subroutine to pass a value from a global variable, calls another subroutine to open a file, updates a table entry using a value from a global variable, and calls two more subroutines using a value from a global variable.
- `execute_callback_loop`: Increments a global variable and calls a routine via a pointer in a6 up to four times in a loop.
- `clears_d0_register_2`: Performs a bitwise OR operation on the d0 register with zero.
- `initialize_handler_data`: Calls three different routines in succession and then returns.
- `clear_memory_block`: Clears 32 words of memory starting at an offset of 4 from a pointer stored at a global address using a loop.
- `copy_handler_table`: Copies 31 words from a source table starting at offset 4 of a pointer in a0 to a destination table starting at offset 0 of a pointer in a1 using a loop, then calls two other routines.
- `update_index_table`: Stores a word in a global variable, calls another routine, and then enters a loop that compares nibbles from two tables to increment a pointer in a1 before calling another routine.
- `decrement_index_table`: Stores a word in a global variable, enters a loop that decrements values in a table based on nibble comparisons, and calls several routines.
- `clear_data_register_d0_6`: Performs a logical OR operation on data register d0 with zero to clear its value.
- `clear_data_d0`: Clears the least significant byte of register d0 using a logical OR operation with zero.
- `map_input_bits_to_index`: Evaluates a bitmask from a global variable through a series of conditional checks to assign a value between one and eight to another global variable, then updates a separate global variable based on the state of the fifth bit of the input mask.
- `set_table_values_by_bits`: Clears two words in a table, then performs a series of bit tests on a word from a global variable to write various specific word values into the same table before setting a bit in another global variable and returning.
- `branch_on_d0`: Performs a bitwise OR on d0 with zero, tests d0, and branches to a specific address if non-zero; otherwise, it loads two addresses into a0 and a1 and branches to another location.
- `branch_on_d0_value`: Compares d0 to 1 and, if not equal, loads two addresses into a0 and a1 and branches to another location.
- `branch_on_data_value`: Compares d0 to 2 and branches if not equal, otherwise it loads two addresses into a0 and a1 and branches to another location.
- `blit_bitmaps_zero`: Loads two addresses into a0 and a1, sets d0 through d3 to zero, and calls a subroutine at address 8658 with d4 set to 320 and d5 set to 200.
- `blit_bitmaps_fixed`: Loads an address into a0, loads an address from a table into a1, sets d0 through d3 to zero, and calls a subroutine at address 8658 with d4 set to 320 and d5 set to 200.
- `blit_bitmaps_offset`: Loads an address into a0, loads an address from a table into a1, sets d0 to 95, d1 and d3 to zero, d2 to 95, and calls a subroutine at address 8658 with d4 set to 160 and d5 set to 200.
- `blit_bitmaps_height`: Loads an address into a0, loads an address from a table into a1, sets d0 through d3 to zero, and calls a subroutine at address 8658 with d4 set to 320 and d5 set to 90.
- `blit_two_bitmaps`: Calls a subroutine at address 8658 using a0 from one address and a1 from another with d4 set to 320 and d5 set to 200, then repeats this process using a different address for a1.
- `branch_on_test_result`: Tests d0 and branches if non-zero, otherwise it loads an address into a0 and branches to another location.
- `update_function_pointers`: Loads an address into a0 and stores it into two different global variables.
- `toggle_global_state`: Tests a word at a global address; if non-zero, it sets that word to 1 and calls another routine, otherwise it sets the word to 0 and calls a different routine.
- `update_handler_pointers_first`: Calls another routine, copies two longwords from a table into global variables, loads an address into a0, stores that address into three global variables, and calls another routine.
- `update_handler_pointers_second`: Calls another routine, copies two longwords from a different table into global variables, loads an address into a0, stores that address into three global variables, and calls another routine.
- `select_handler_data`: Tests a word in a global variable; if it is zero, it loads an address into a0, stores it in two global variables, and copies two longwords from a table into other global variables; otherwise, it loads a different address into a0, stores it in the same two global variables, and copies two longwords from a different table into the other global variables, finally calling another routine.
- `update_table_pointers`: Checks a global word; if it is zero, it copies an address from a table into three other global locations using offsets, otherwise it copies an address from a different table into those same global locations.
- `toggle_state_from_d0`: Tests d0; if non-zero, it sets a word at a global address to 1 and calls another routine, otherwise it sets the word to 0 and calls a different routine.
- `call_library_functions`: Loads a library base from a global variable and calls three different functions at offsets minus 216, minus 226, and minus 222 from that base using registers a0 and a1.
- `dispatch_four_values`: Calls another routine four times, passing the values zero, one, two, and three in register d0 sequentially.
- `dispatch_by_d0_value`: Checks the value of d0 and calls one of several different routines based on whether d0 is zero, one, two, or another value.
- `call_blt_clear`: Library stub: loads its arguments and the graphics library base, calls BltClear and returns its result to the caller.
- `clears_data_register_zero_2`: Performs three consecutive logical OR operations with zero on data register zero.
- `perform_bitmapped_blits`: Sets up registers and calls a routine three times to perform bitmapped operations using various source and destination addresses and parameters.
- `call_blt_bit_map`: Library stub: loads its arguments and the graphics library base, calls BltBitMap and returns its result to the caller.
- `clear_d0_register_17`: Performs a logical OR operation on the d0 register with zero, effectively ensuring the register contains zero.
- `clear_d0_triple_alt`: Executes three logical OR operations with zero on register d0 in succession.
- `process_exec_message`: Calls a routine to retrieve a message, then if the result is not zero, extracts a pointer and two values from the message structure, calls another routine, and returns a masked word if a specific constant is matched.
- `wait_for_message`: Calls another routine and loops until the returned value in register d0 is non-zero.
- `call_open_window`: Library stub: loads its arguments and the intuition library base, calls OpenWindow and returns its result to the caller.
- `call_close_window`: Wrapper that calls the intuition library function CloseWindow and returns its result to the caller.
- `clear_data_register_zero_9`: Sets the least significant byte of data register zero to zero.
- `clears_data_register_zero_3`: Performs a logical OR operation on data register zero using a zero constant.
- `clears_data_register_zero_4`: Performs two consecutive logical OR operations with zero on data register zero.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_function`: Loads a library base address from a global variable into a6, moves several registers into d1, d2, and d3, calls a function at an offset from the library base, and then increments a pointer in a0 before returning.
- `process_bitmask_data_blocks`: Iterates through a series of data blocks based on a count in a long variable, performing bitwise operations on memory pointed to by a0 and a1.
- `check_bit_and_call`: Tests a bit in a memory location pointed to by a0; if the bit is set, it sets d0 to one and calls another routine, otherwise it returns.
- `loop_bit_test_and_shift`: Tests a bit in a memory location pointed to by a0; if the bit is set, it sets d3 to one, calls another routine, and enters a loop that decrements d4 while performing logical shifts on d3, otherwise it returns.
- `decrement_and_adjust_d2`: Decrements d2 and if it is not negative, sets d2 to seven and increments a2 before returning; otherwise it returns.
- `decrement_and_adjust_d5`: Decrements d5 and if it is not negative, sets d5 to seven and increments a0 before returning; otherwise it returns.
- `decrement_counters`: Decrements a long value in d7 and a word in d6, and if d7 is not negative, sets d6 to 7 and increments a pointer in a1 before returning; otherwise it returns.
- `save_context_to_globals`: Stores the current library base in a6, register d0, register d1, a global variable, and the decremented value of a word in d7 into five consecutive global variables.
- `store_registers_to_globals`: Stores the contents of registers a1 through a5 into five consecutive global variables.
- `calculate_scaled_index`: Calls three different routines, then performs arithmetic on a value from a global variable involving a bit shift, a multiplication by ten, and additions to produce a final result.
- `calculate_memory_offsets`: Calls several routines to retrieve data, copies values from a pointer to a new location, and calculates a new address for a pointer in a2 using multiplication and bit shifts.
- `update_calculated_values`: Subtracts d0 from d2 and stores the result in a longword at offset 2a40, subtracts d1 from d3 and stores the result in a longword at offset 2a3e, then multiplies d1 by 40 and adds d0 to the result before storing it in a longword at offset 2a42.
- `wait_and_set_flag`: Checks a word in a global variable and if it is not zero, calls two routines and sets that global variable to one; otherwise it calls two routines and returns.
- `wait_and_clear_flag`: Checks a word in a global variable and if it is not zero, calls two routines and clears that global variable; otherwise it calls two routines and returns.
- `update_sprite_and_hardware_registers`: Calculates sprite dimensions and positions based on values from a table, updates several global variables, and writes values to custom hardware registers to configure sprite attributes and DMA settings.
- `update_hardware_registers`: Calls a routine, writes several values to offsets relative to a pointer in a6, and conditionally updates two word values based on bit-shifted versions of a value from a global variable.
- `wait_for_bit_clear`: Loops until bit 6 of a byte at offset -50e from the address in a6 is cleared.
- `perform_zero_or_instruction`: Performs an or operation on d0 with zero.
- `clear_d0_register_18`: Clears the d0 register using a logical OR operation with zero.
- `allocate_and_copy_data`: Calls a routine to allocate memory, stores the result in a global variable, calls another routine using a pointer from a global variable, and finally calls a third routine with a pointer and a second address.
- `allocate_multiple_data_blocks`: Performs two pairs of memory allocation and pointer-based routine calls, storing the results in four different global variables.
- `allocate_triple_data_blocks`: Performs three pairs of memory allocation and pointer-based routine calls, storing the results in six different global variables.
- `load_and_store_registers`: Loads five longwords from a table into registers a1 through a5 and calls another routine.
- `load_and_store_registers_alt`: Loads five longwords from a different table into registers a1 through a5 and calls another routine.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_free_mem_thrice`: Retrieves three sets of pointers from a table and calls a routine at an offset from a library function three times.
- `call_free_mem_twice`: Retrieves two sets of pointers from a table and calls a routine at an offset from a library function twice.
- `initialize_asset_and_graphics_tables`: Initializes various data structures and graphics parameters by calling several routines to set up memory blocks, update index tables, and load bitmap data.
- `setup_sprite_parameters`: Calls two subroutines, checks a word in a global variable, and if it is not zero, calls another subroutine, sets registers d0, d1, and d2 to specific values, loads a pointer from a global variable into a0, calls a third subroutine, and returns.
- `update_sprite_parameters`: Sets three 16-bit values into registers d0, d1, and d2, loads a pointer from a global variable into a0, and calls another routine; it then repeats this process with a different set of three 16-bit values before returning.
- `process_message_and_blit`: Calls a routine to check a table, then if a condition is met, iterates through a loop to perform several blit operations and calls a routine with specific coordinate offsets.
- `calculate_blit_parameters`: Compares register d6 against zero and 383, then sets various registers based on the result and calls another routine using a pointer from a table.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `setup_blit_parameters`: Sets registers d0 through d5 with specific constant values and calls another routine using a pointer from a table.
- `clamp_d0_value`: Decrements d0 by 8 and, if the result is non-negative, sets d0 to 0 and performs several assignments to d1, d2, and d4 based on comparisons with d7.
- `calculate_blit_offset`: Calculates a value using registers d7, d0, and d4, then sets d5 and calls another routine using a pointer from a table.
- `execute_graphics_loop`: Calls several routines, then enters a loop that performs two bitwise operations and calls a routine to draw graphics using parameters from a global variable, repeating until a counter reaches a constant.
- `add_and_compare_longword`: Adds the value in d0 to a longword at offset 3832, then compares a longword at offset 13b6 with the value at offset 3832 and loops if the first is less than the second.
- `clear_d0_register_19`: Sets the d0 register to zero.
- `perform_zero_or_instruction_2`: Performs an or operation on d0 with zero twice.
- `zero_data_padding_2`: This routine contains no executable instructions and serves as a data area containing zeroed bytes.

### Fiendish Freddy's Big Top 'O Fun (demo) (1990)(Mindscape).adf:songs

- `__H0_org`: Performs a short branch to an offset of 24.
- `clear_d0_register_14`: Clears the d0 register and loads a zero into a0 before returning.
- `_segload`: Saves registers d0 through d2 and a0 through a4 onto the stack, loads a pointer from an offset of 36 from the stack pointer into a1, compares the word at that pointer to a constant, and branches to another routine based on the result.
- `_segload_2`: Iterates through a list of segment descriptors, calling another routine and then performing a loop that writes a sequence of words and longwords to a destination table based on values retrieved from a structure.
- `restore_registers`: Restores d0 through d2 and a0 through a4 from the stack and returns.
- `open_library_and_traverse_list`: Calls a routine via a pointer from the exec library, then if the result is non-zero, performs several calls through pointers in a structure and traverses a linked list to store a value.
- `store_d0_at_pointer`: Stores the value of d0 into the memory location pointed to by a0 and restores d2, d3, and a6 from the stack before returning.
- `call_alert`: Wrapper that calls the exec library function Alert and returns its result to the caller.

### Fiendish Freddy's Big Top 'O Fun (demo) (1990)(Mindscape).adf:songplay

- `__H0_org`: Compares a value in register d0 against several constants to branch to specific subroutines for managing music and sound effects playback, pausing, or fading.
- `start_sfx_channel`: Sets a bit in a global status byte, calculates DMA and interrupt enable values, and calls routines to load a sample pointer, volume, and count into a track structure while programming the hardware period.
- `copy_track_data`: Copies a 32-bit longword from an offset within a structure pointed to by a1 into a destination at offset 6 of a structure pointed to by a0, then clears a 16-bit word at offset 14 of the structure in a0.
- `PlaySong`: Checks if a register contains a non-zero value and, if so, updates a global state variable and a global song address variable with that value.
- `_GetSongStat`: Returns the 16-bit value stored in a global variable.
- `SongInt`: Saves registers to the stack, loads a constant into a4, and if a global pointer is not zero, calls another routine.
- `clear_word_and_call`: Clears a word at a memory location, calls SongInt, and then branches to another routine.
- `handle_audio_fade`: Decrements a global counter and, if it reaches zero, loads a speed value from a global variable and calls a routine four times; otherwise, it calls a routine four times with specific parameters and checks a global state.
- `decrement_wait_counter`: Counts down a 16-bit delay counter in the record while it is non-zero, and leaves it alone once it reaches zero.
- `initialize_audio_tracks`: Initializes four track information structures by reading from a table, clearing various fields, and calling a routine four times with different parameters, then checks a global state and calls another routine if a condition is met.
- `disable_audio_channels`: Turns off the four audio DMA channels and their interrupts by writing clear masks to the DMA and interrupt control registers.
- `process_audio_command_stream_2`: Iterates through a command stream by reading a command byte and an associated byte, performing various actions such as updating a pointer, decrementing a counter, or loading sample data and parameters from a table based on the command value, and calls another routine to handle specific note or volume updates.
- `update_track_volume`: Decrements a 16-bit value at offset 28 of the structure in a0 and, if it is not greater than 0, performs arithmetic using values from a3 and calls another routine and another routine.
- `compare_and_branch_2`: Compares a word at an offset of 20 from a0 with register d0 and branches based on the result.
- `configure_dma_and_volume`: Checks three conditions in a structure; if they match, it moves a word from a pointer into the DMA controller register, clears a word at an offset of 40 in the structure, sets a word at offset 44 to 124, and sets a word at offset 24 to 65535.
- `parse_audio_command_stream`: Iterates through a command stream by reading a command byte and an associated byte, performing various actions such as updating a pointer, resetting the stream, loading sample data from a table, or updating playback parameters based on the command value, and calls update_sample_playback_parameters_2 and process_audio_command_stream.
- `update_sample_playback_parameters_2`: Clears a 16-bit word at offset 58 of a structure, masks a value in d1, adds a 16-bit value from a table at offset 4 of a0 using d0 as an index, stores the result in offset 64 of the structure, adds d3 to d1, and branches to another location.
- `process_audio_command_stream`: Iterates through a stream of command bytes, performing specific actions based on the command value such as incrementing a pointer, resetting a structure, loading sample parameters from a table, or updating volume and hardware registers.
- `update_pointer_and_load_data`: Increments a 32-bit value at offset 6 of the structure in a3 by 6, increments d0 by 6, loads a 32-bit value from a table using d0 as an offset into offset 6 of a3, moves that value to a1, and branches to another routine.
- `clear_and_load_structure_data`: Clears a 32-bit value at offset a of the structure in a3, loads a pointer from offset 2 of a3 into a0, loads a 32-bit value from that pointer into offset 6 of a3, moves that value to a1, moves the value at offset a of a3 into d0, and branches to another routine.
- `clear_word_and_branch`: Clears a 16-bit value at offset 0 of the structure in a3 and branches to another routine.
- `load_sample_parameters`: Shifts d2 left by 2, loads a pointer from a global variable into a2, loads a pointer from offset d2 of that table into a2, stores a2 into offset 3c of a3, stores a 32-bit value from a2 into offset 14 of a3, stores a 32-bit value from offset 4 of a2 into offset 18 of a3, stores a 16-bit value from offset 8 of a2 into offset 1c of a3, and branches to another routine.
- `write_cia_register`: Writes the byte in d2 to the CIAA data register, writes 16 to the CIAA control register, writes 1 to the CIAA control register, and branches to another routine.
- `write_cia_register_alternative`: Writes the byte in d2 to the CIAO data register, writes 16 to the CIAA control register, writes 1 to the CIAA control register, and branches to another routine.
- `update_volume_conditionally`: Compares a 16-bit value at offset 27c to 4; if equal it branches, otherwise it checks offset 46 of a3; if offset 46 is not zero it branches, otherwise it loads a pointer from offset 28 of a3, writes d2 to that pointer, writes d2 to offset 1e of a3, and branches to another routine.
- `store_d2_to_offset`: Stores the 16-bit value in d2 into offset 38 of the structure in a3 and branches to another routine.
- `update_sample_playback_parameters`: Reads a volume and pitch value from a table using an index, updates a structure with these values, and if a condition is met, calculates a new sample position and period using a loop and writes them to pointers stored within the structure.
- `write_scaled_track_volume`: Scales a volume value by the sound effect music volume and writes the result through a stored pointer.
- `compute_note_period`: Computes the audio period for the current note by dividing a frequency table entry by a scaled sample rate.
- `SongIntHandler`: Calls another routine four consecutive times with different parameters and track information pointers, then returns via an interrupt return.
- `CheckChannelInt`: Checks bit 0 of a longword in d1, and if set, performs a series of comparisons and conditional branches on values at offsets 70 and 18 of a structure in a0, potentially clearing a 16-bit value at offset 18.
- `configure_dma_and_channel`: Decrements d1 by 7, clears a bit in a hardware register, loads a pointer from offset 28 of a0 into a1, clears a 16-bit value at that pointer, loads a pointer from offset 2c of a0 into a1, writes 124 to that pointer, writes d2 to a hardware register, writes 65535 to offset 12 of a0, clears offsets 48 and 46 of a0, and branches to another routine.
- `load_pending_channel_sample`: Copies the pending sample address and period into the channel's pointer and period fields and resets the gate value so the channel plays it.
- `update_voice_and_status`: Decrements a 16-bit value at offset 72 of a structure in a0, and if the value is non-zero, it calls a routine to update a voice pointer and calculate a period value using a division by 83.
- `find_voice_header_chunk`: Scans forward two bytes at a time until it reaches the four-character voice header tag, then leaves the pointer on it.
- `find_body_chunk_tag`: Scans forward two bytes at a time through a pointer in a2 until it finds the four-character tag BODY, then returns with the pointer on that tag.

### Fiendish Freddy's Big Top 'O Fun (demo) (1990)(Mindscape).adf:tsongs

- `__H0_org`: Branches to an offset of 24.
- `clear_d0_and_return`: Clears the least significant byte of register d0 four times and returns.
- `_segload`: Saves registers to the stack, compares a word at an offset from a pointer in a1 to a constant, and branches to another routine or continues based on the result.
- `_segload_2`: Iterates through a loop to copy data from a table into a destination area, using a counter from a second table to determine the number of iterations, and calls another routine during each loop.
- `restore_registers_and_return`: Restores several data and address registers from the stack and returns immediately.
- `open_dos_file_and_traverse_table`: Opens a file using a library call, then checks the result to call an alert routine if it fails.
- `store_d0_to_pointer`: Stores the value from d0 into the memory location pointed to by a0 and returns after restoring d2, d3, and a6 from the stack.
- `call_alert_2`: Wrapper that calls the exec library function Alert and returns its result to the caller.

### Fiendish Freddy's Big Top 'O Fun (demo) (1990)(Mindscape).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `perform_repetitive_data_manipulation`: Performs a long sequence of moving a0 to d0 and performing a bitwise OR with a constant.
