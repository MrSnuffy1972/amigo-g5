# Game 295 (giger-tetris (1993)(panic production)(aga)(pd)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00295). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 294 of 294 named (0 author symbols), 294 of 294 described (24 by rule, 268 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 163 of 163 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3675 of 3712 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 152048 |
| code | 13560 |
| data | 116310 |
| padding | 1730 |
| unused | 617472 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Giger-Tetris (1993)(Panic Production)(AGA)(PD).adf:TETRIS | 2 | 2268 | 175 |
| Giger-Tetris (1993)(Panic Production)(AGA)(PD).adf:Setpatch | 2 | 1426 | 118 |
| Giger-Tetris (1993)(Panic Production)(AGA)(PD).adf:bootblock | 1 | 18 | 1 |

## Cases handled

- reference crashes at wild return (HLE exec.library execPrivate15 (-an address) not emulated (D0=an address D1=an address D2=an address A0=an address A1=an address ret an address) [wild: no JSR): 2342950 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Giger-Tetris (1993)(Panic Production)(AGA)(PD).adf:TETRIS

- `initialize_system_state`: Copies the stack pointer to a global variable, calls three different routines, stores a long value to a global variable, calls a fourth routine, and if d0 matches a specific constant, calls a fifth routine before clearing d0 through d7 and loading a0 through a6 with zero.
- `execute_initialization_sequence`: Calls thirteen different routines in sequence and then returns.
- `execute_sequence_of_subroutines`: Calls nine different subroutines in sequence and returns zero.
- `execute_audio_sequence`: Saves registers to the stack, calls one routine via a jump to a long address, calls two other routines via branch instructions, restores the registers, and returns.
- `initialize_game_state`: Calls six different routines and clears or writes various word and long values to several global memory locations.
- `initialize_memory_buffers`: Clears a 36-word long block of memory starting at a global variable and fills another 36-word long block starting at a different global variable with all ones using a loop.
- `process_input_and_dispatch_message`: Calls a routine to retrieve a message and enters a loop that checks the message content; it updates several global bytes based on whether the message character matches specific values and calls another routine if the message indicates a specific command.
- `execute_repeated_blitter_setup`: Calls another routine multiple times with varying values in d0 and d1 to perform a series of operations.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `perform_zero_operation`: Performs a bitwise OR operation on register d0 with zero.
- `test_flag_and_call_routine`: Tests a byte in a global variable; if it is non-zero, it calls another routine, otherwise it returns.
- `check_flag_and_call_routine`: Tests a byte in a global variable; if it is non-zero, it calls another routine and otherwise returns.
- `update_timer_and_dispatch_callbacks`: Increments a 16-bit value in a global structure by adding one and the values of three CIA timer registers, then checks four consecutive bytes in a global variable to conditionally call four different routines.
- `decrement_counter_and_execute_callbacks`: Clears a byte at a global offset, checks a byte at another global offset, and if non-zero, calls a routine; it then decrements a word in a global variable and, if it reaches zero, calls a routine, checks a long at another global offset, and if non-zero, calls two subsequent routines before returning.
- `increment_and_check_word_pointer`: Tests a 16-bit word at a pointer, increments it by one if it is not negative, and if the result is non-zero, calls two separate routines and sets the sign flag before returning.
- `decrement_global_word`: Decrements a 16-bit word at a global address and, if the result is non-zero, calls another routine before returning.
- `process_table_indices`: Copies two words from memory to a global table, increments a counter, calls another routine, and then enters a loop that either increments or decrements a pointer based on a global byte value while calling another routine.
- `loop_over_word_count`: Decrements a word at a memory location, calls another routine, and then enters a loop that iterates d6 times, performing a test on a byte at a specific address before branching.
- `move_word_and_branch`: Moves a word from the stack into d6 and branches to another routine.
- `copy_word_to_globals`: Stores two words from specific memory locations into global variables and branches to another routine.
- `clears_data_register_zero`: Performs a bitwise OR operation on data register zero with zero.
- `set_global_word_and_branch`: Sets the sign bit of a global byte, stores a word from a global variable into another global variable, and branches to another routine.
- `set_global_word_only`: Sets the sign bit of a global byte and stores a word from a global variable into another global variable before branching to another routine.
- `increment_and_call_nested`: Sets the sign bit of a global byte, increments a global word, calls another routine, and if the global word is not zero, decrements it before calling another routine.
- `decrement_and_call_nested`: Sets the sign bit of a global byte, decrements a global word, calls another routine, and if the global word is not zero, increments it before calling another routine.
- `execute_conditional_sequence`: Copies a word between two global locations, compares a global word to another global word, and if they are equal, increments a global long and a global word; it then calls another routine and performs several other calls and stores based on the results of those calls.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `initialize_structured_data`: Reads a byte from a pointer-based table, uses it to calculate offsets to write the constant value an address into three 32-bit words at a global variable and another location, then enters a loop that calls another routine eleven times.
- `set_global_constant`: Stores the word value four into a global variable.
- `process_data_table`: Reads a byte from a pointer-based table, increments a counter, and enters a loop that calls another routine eleven times while calculating various offsets for data manipulation.
- `subtract_from_global`: Subtracts the value in register d5 from the word value minus four and stores the result in a global variable.
- `apply_bitmask_to_table`: Calculates a destination address using a table and offsets, then performs a series of bitwise OR operations to merge bytes from a source table into specific bit positions within a destination table, finally setting two global words to all ones.
- `clear_and_setup_pointers`: Clears a long value at a global address, loads two addresses into a5 and a0, and enters a loop that iterates 19 times based on d6, branching to another routine if a byte at an offset of d6 from a5 is zero.
- `update_pointer_and_status`: Adds 12 to a1, performs a loop based on d7, sets the sign bit of a byte at a5, loads a word from a pointer into d0, and returns.
- `copy_and_update_global`: If a global word is non-zero, it copies 60 words from one location to another; otherwise, it calculates a value, stores it in a byte, copies three 32-bit words from a global variable to another location, and increments a global word.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `compare_and_copy_tables`: Compares bytes from two tables in a nested loop, calling another routine if the bytes differ or if a byte is zero, and uses two loops to manage the iteration counts.
- `copy_data_blocks`: Runs a loop eight times to copy bytes from a source table to a destination table at specific intervals, using offsets provided in registers and a global variable.
- `copy_structured_data`: Runs a loop eight times to copy a series of bytes from a source location determined by a global variable and a register into a destination table at specific offsets.
- `initialize_state_parameters`: Checks a global byte, and if it is zero, performs bitwise operations on a global word, clears two global words, sets a global word to four, sets a global word to sixty-five thousand five hundred thirty-five, calls another routine, and if it does not return zero, stores a value and copies two words from a global variable.
- `set_word_to_max`: Stores the word value minus one into a global variable.
- `fetch_and_mask_table_data`: Calculates a pointer to a table entry using offsets and a multiplier, then performs a series of bitwise AND operations between the contents of four 32-bit words and a bitmask.
- `validate_and_update_bitmask`: Checks if four 32-bit values in a structure satisfy a bitmask condition; if they do, it copies bytes from a source table to a destination buffer, performs bitwise operations on several 32-bit global variables using a mask, and returns the mask.
- `increment_global_counter`: Increments a word in a global variable by one and returns.
- `process_nested_data_blocks`: Calculates a destination address using values from a table, performs nested loops to copy bytes from a source to that destination, and calls another routine.
- `copy_data_blocks_2`: Calculates an offset into a table based on the lower 3 bits of a register, then loops eight times to copy bytes from a source table to a destination table at specific offsets.
- `copy_nested_data`: Calculates a destination address using values from a table and performs nested loops to copy bytes from a source to that destination, calling another routine for each non-zero byte.
- `copy_data_with_offset`: Loops eight times to copy bytes from a source table to a destination table, where the source address is incremented by a constant value of 1952 each iteration.
- `process_bcd_table`: Performs BCD adjustments on values from a table and loops through a sequence to call another routine, then stores a pointer from a table into a global variable.
- `clear_d0_bits`: Performs four logical OR operations on register d0 with zero.
- `clear_bcd_table`: Performs BCD adjustments on a value from a table and executes a loop that calls another routine to process data, finally clearing a long value in a global variable.
- `decrement_and_adjust_counter`: Decrements a byte in a global variable and, if it reaches zero, compares two other values, potentially performs a BCD adjustment and calls another routine before updating the original variable.
- `execute_conditional_calls`: Checks a byte from a table and calls another routine once or twice based on the value of the high nibble.
- `setup_blt_loop`: Disables interrupts, configures Blitter registers via custom hardware addresses, and loops eight times to perform Blitter operations using data from a table and a calculated destination address.
- `execute_three_subroutines`: Calls three different routines in sequence and then returns.
- `update_joystick_counters`: Reads values from the mouse/joystick port, compares them to values in a table, and updates two 16-bit counters in a global variable based on the difference.
- `clamp_global_values`: Clamps two different global variables within specific ranges, specifically zero to 255 and zero to 383.
- `construct_bitfield_pointer`: Calculates a value by combining several pieces of data from a table and bitwise operations, then stores the result in a global pointer.
- `tiny_ori_2`: Small routine of 1 instruction that merges 200 into d0.
- `set_d0_bits`: Performs a logical OR operation on register d0 with the constant value 200.
- `write_to_stack`: Performs a logical OR operation on a 16-bit word at an offset from the stack pointer with the constant value 232.
- `setup_blitter_tables`: Copies data from two tables into new locations with swapped word components, then populates a series of Blitter command structures and sets the hardware DMA pointers.
- `execute_blt_sequence`: Configures Blitter registers, calculates a destination address using a value from a table, and loops eight times to perform Blitter operations with a specific source address and height.
- `call_add_int_server`: Library stub: loads its arguments and the exec library base, calls AddIntServer and returns its result to the caller.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `negate_and_clear`: Negates a 16-bit word at an offset from register a1 and performs a logical OR operation on register d0 with zero.
- `find_task_and_store`: Decrements register a1, calls a function from a library pointer, and stores the resulting value in a global variable.
- `call_open_library`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_open_library_2`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `call_close_library_2`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_open_library_3`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `call_close_library_3`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_own_blitter`: Wrapper that calls the graphics library function OwnBlitter and returns its result to the caller.
- `call_disown_blitter`: Wrapper that calls the graphics library function DisownBlitter and returns its result to the caller.
- `load_view_and_wait`: Loads a view from a library and calls two other library routines via pointers stored in a global variable.
- `call_load_view`: Wrapper that calls the graphics library function LoadView and returns its result to the caller.
- `perform_no_operation`: Performs a logical OR operation on register d0 with zero.
- `initialize_global_pointers`: Stores a pointer to a global variable into a location at offset 187c, then writes the value 256 to offset 1804 and the value 40 to offset 1806 of a structure.
- `copy_data_from_library_call`: Calls a routine via a pointer in a6, stores the result in a global variable, and then copies eight 32-bit words from a source buffer starting at offset 184 to a destination buffer starting at a location defined by a global variable.
- `call_close_screen`: Wrapper that calls the intuition library function CloseScreen and returns its result to the caller.
- `update_window_parameters`: Loads a word from offset 4 of a structure into d0, writes this value to offsets 4, 38, and 42 of a new structure, calls a routine via a pointer in a6, stores the result in a global variable, and then updates a global pointer using a value from a structure at offset 184.
- `call_close_window`: Library stub: loads its arguments and the intuition library base, calls CloseWindow and returns its result to the caller.
- `enable_dma_channel`: Calls a routine via a pointer in a6 and then writes the value 32800 to the DMA controller register to enable a channel.
- `update_hardware_registers`: Loads a pointer from offset 4, accesses a structure at offset 9c, and then writes two 32-bit words from that structure into the hardware registers at offsets 38 and 50.
- `process_message_and_reply`: Calls a routine via a pointer in a6 to retrieve a message, and if the message is not null, copies several fields from the message into global variables before calling another routine via a pointer in a6 to reply to the message.
- `loop_exec_msg`: Calls a routine via a pointer and enters a loop that calls another routine if the return value is non-zero.
- `clear_global_long`: Clears a 32-bit global variable.
- `initialize_pointer_and_value`: Stores a pointer to a global variable into a memory location and sets a word at another location to four.
- `initialize_data_structures`: Searches through a buffer to find a specific byte value, then clears a series of memory locations and sets a bit in the CIAA Port A register.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `calculate_scaled_value`: Reads two words from a table, clamps the sum of these words between 0 and 256, and then stores the result of multiplying this clamped value by register d0 and shifting right by 8 bits into a global variable at offset 8 of a structure pointed to by a5.
- `process_audio_event`: Increments a byte in a global variable, compares it to another byte, and if the comparison does not carry, clears the global variable and either returns immediately or calls another routine before branching to another routine.
- `call_and_branch`: Calls another routine and then performs an unconditional branch to another location.
- `update_audio_channel_pointers`: Sets the destination address for each of the four audio channels by loading the hardware registers with values from a table and calling a nibble command dispatcher for each channel.
- `configure_audio_channels`: Fetches a byte from a table using an index derived from a structure, calculates a value by shifting and adding, clears a global word, and then calls another routine four times to load specific values into the four audio channel registers.
- `initialize_object_from_data_table`: Loads a pointer from a table into a structure, extracts a nibble-based index from the structure's bytes, and uses that index to fetch and store multiple values—including a pointer, two 16-bit words, and two bytes—from a data table into the structure.
- `update_object_state_and_dispatch_logic`: Calculates and stores new pointer values in a structure, updates a field from a table using a nibble-based index, and branches to various subroutines based on a command nibble extracted from a byte in the structure.
- `process_object_command_and_update_state`: Iterates through a table to find a matching index, updates a structure with values from a data block, and modifies a DMA control register.
- `dispatch_nibble_command_logic`: Extracts a nibble from a byte at offset 2 of a structure in a6 and executes different logic based on its value, including updating a 16-bit value at offset 4, modifying bytes at offsets 15, 16, 19, or 31, or updating a global variable.
- `update_object_state_and_dispatch`: Searches a table for a matching index, updates a structure with values from a data block, and modifies a DMA control register.
- `update_audio_and_counters`: Calls another routine, sets a bit in a global word, and updates several structure offsets by copying values from tables.
- `dispatch_nibble_command_2`: Extracts a nibble from a byte at offset 2 of a structure in a6 and branches to different routines based on its value.
- `copy_word_to_structure`: Copies a 16-bit word from offset 16 of a structure pointed to by a6 into offset 6 of a structure pointed to by a5.
- `calculate_index_from_pc_relative`: Loads a byte from a PC-relative address, divides it by three, swaps the bytes in d0, and branches based on whether the result is zero or two; otherwise, it loads a byte from offset three of a6, shifts it right by four, and branches.
- `search_table_for_value_2`: Calculates a starting address for a table based on a byte at offset 18 of a6, then loops up to 37 times to find a 16-bit word in that table.
- `copy_word_from_offset`: Copies a 16-bit word from offset 16 of the structure in a6 to offset 6 of the structure in a6.
- `search_table_for_value`: Calculates a starting address for a table based on a byte at offset 18 of a6, then loops up to 37 times to find a 16-bit word in that table that is less than or equal to the word at offset 16 of a6, storing the result in a structure at a6.
- `store_word_at_offset`: Stores a 16-bit value from register d2 into a word at offset 6 of the structure pointed to by a6.
- `decrement_and_clamp_value`: If a global byte is zero, it subtracts a masked byte from offset 3 of a6 from a 16-bit word at offset 16 of a6, then clamps the result to a minimum of 113 and stores the masked 12-bit portion into a structure at a5.
- `increment_and_clamp_value`: If a global byte is zero, it adds a masked byte from offset 3 of a6 to a 16-bit word at offset 16 of a6, then clamps the result to a maximum of 856 and stores the masked 12-bit portion into a structure at a5.
- `update_table_value`: Calculates an offset into a table using a byte from offset 18 of the structure in a6, searches for a matching 16-bit value from offset 0 of a6, and if found, stores that value at offset 24 of the structure in a6 and sets a byte at offset 22 based on a comparison with a value at offset 16.
- `clear_word_at_offset`: Clears a 16-bit word at offset 24 of the structure pointed to by a6.
- `update_index_via_table_2`: If a byte at offset 3 of a6 is not zero, it stores that byte at offset 23 of a6 and clears the byte at offset 3 of a6; otherwise it returns.
- `update_index_via_table`: Updates a 16-bit value at offset 16 of a6 by adding or subtracting a byte from offset 23 of a6 depending on a flag at offset 22, then performs a table lookup to update a value in a structure at a5.
- `calculate_index_and_store`: Subtracts a value from a word, performs a loop to find a value in a table based on a calculated index, and stores the result into a structure.
- `update_structure_byte`: If a byte at offset 3 of the structure in a6 is non-zero, it updates the byte at offset 26 by merging its lower and upper nibbles with the nibbles from the byte at offset 3.
- `calculate_scaled_value_2`: Loads a byte from offset 27 of a6, performs bitwise operations on it, loads a byte from offset 30 of a6, and branches to another routine based on the value of the second byte.
- `calculate_byte_difference`: Tests a byte at offset twenty-seven of a6; if the byte is negative, it loads the value negative one into d2, subtracts d0 from d2, and branches to another routine; otherwise, it branches to another routine.
- `transfer_byte_to_register`: Moves a byte from a register into d2 and then branches to another location.
- `calculate_scaled_offset`: Multiplies a byte from offset 18 of a6 by a byte from a4, shifts the result, and adds it to a 16-bit word from offset 16 of a6 if a flag at offset 27 of a6 is not set.
- `subtract_and_advance_phase`: Subtracts one input value from another and stores the difference in a global variable.
- `call_update_index_routine`: Calls another routine and then branches to another location.
- `adjust_clamped_value`: Calls another routine, then either adds the high nibble of a byte at offset 3 of a6 to a byte at offset 19 of a6 and clamps it to 64, or subtracts the low nibble of that same byte and clamps the result to 0.
- `modify_offset_byte`: Updates the byte at offset 28 of a6 by merging its nibbles with those from the byte at offset 3 of a6, then adds a value derived from the byte at offset 30 to the byte at offset 29.
- `branch_on_signed_byte`: Tests a byte at offset 27 of a6; if the byte is positive, it branches to another routine, otherwise it sets d2 to negative one, subtracts d0 from d2, and branches to another routine.
- `move_byte_to_register`: Moves a byte from register d0 to register d2 and then branches to another location.
- `calculate_scaled_offset_2`: Multiplies a byte from a table by a value in d0, shifts the result, and adds it to a byte from offset 13 of a6 if a flag at offset 1d of a6 is not set.
- `add_scaled_byte`: Subtracts a 16-bit word in d2 from d0, clamps the result between 0 and 64, calls another routine, and then adds a bit-shifted byte from offset 28 of a6 to a byte at offset 29 of a6.
- `adjust_long_value`: If a byte at offset 3 of a6 is non-zero, it stores that byte at offset 32 of a6, then subtracts a shifted version of that byte from a 16-bit word at offset 8 of a6 and adds a value to a 32-bit long at offset 4 of a6.
- `set_word_to_one`: Sets a 16-bit word at offset 8 of the structure pointed to by a6 to one.
- `decrement_and_call_routine_2`: Subtracts a masked byte from offset 3 of a6 from a byte at offset 19 of a6, clamps the result to 64, and calls another routine.
- `decrement_and_call_routine`: Subtracts a masked byte from offset 3 of a6 from a byte at offset 19 of a6, clears the byte if the result is negative, and then calls another routine.
- `clear_global_byte_flag`: It stores one less than the value of a byte in a flag, writes zero to a global variable, and finally sets the flag to all ones.
- `clamp_byte_value`: It compares a byte from a structure to sixty-four, stores the smaller of the two values into a different offset in the structure, and calls another routine.
- `calculate_and_store_scaled_byte`: Calculates a value by multiplying a shifted byte by ten and adding a masked byte, then stores the result in a global variable if it is not greater than 63.
- `write_byte_to_memory`: Checks if a byte at offset 3 of the structure in a6 is zero; if not, it clears a byte at a global address and stores the byte from offset 3 into another global address.
- `dispatch_nibble_command_3`: Extracts a nibble from a byte at offset 2 of a structure in a6 and branches to different routines based on its value.
- `dispatch_nibble_command`: Extracts the high nibble from a byte at offset 3 of a structure in a6 and branches to different routines based on its value.
- `update_global_from_ciaa_pra`: Reads a byte from the CIAA_PRA register, masks bit zero, shifts it left by one, and ORs the result into a global variable after clearing bit one of that variable.
- `mask_and_merge_byte`: It masks the low nibble of a byte at a fixed offset from a pointer in a register and then merges the low nibble of another byte at a different offset from the same pointer into it.
- `clear_low_nibbles_of_byte`: It clears the low nibble of a byte at a fixed offset from an address in a register and then ORs in the low nibble of another byte from the same base.
- `mask_low_nibble_to_offset`: The routine stores the low nibble of a byte from an address derived from an uninitialized pointer into a byte at a fixed offset from that pointer.
- `process_nibble_and_pointer`: Checks a byte at a global pointer, and if it is not zero, extracts a nibble from a byte at offset 3 of a structure, decrements a byte at offset 34 of the structure, and if that byte is not zero, stores a byte from a register into a location derived from a pointer at offset 33 and a constant offset 2.
- `store_byte_and_branch`: It stores a byte from register d0 into an offset of a structure and branches to another routine.
- `store_nibble_via_pointer`: It writes a byte derived from a register value shifted right by four positions into a location computed from a pointer plus a constant offset and then returns.
- `mask_and_merge_nibble`: Masks the low nibble of a byte at offset 3 of a pointer in a6 and merges the high nibble from a byte at offset 30 of the same pointer, storing the result back at offset 30.
- `update_dma_parameters`: It checks if a value in a structure is divisible by a byte from the same structure, then updates a destination pointer and two word values from the structure, calls another routine twice with different bit settings in a global variable, and finally updates the destination pointer again.
- `test_and_mask_byte`: Tests a byte at a specific memory location, branches if it is not zero, otherwise it clears d0, moves a masked byte from offset 3 of a6 into d0, and branches to another location.
- `extract_nibble_and_branch`: Checks a byte at a global pointer, and if it is not zero, clears register d0 and loads a masked byte from offset 3 of a structure into it before branching.
- `clear_structure_fields`: It compares a byte from a structure with a value at a specific memory location, and if they are not equal, clears a byte at an offset in the structure and a word at an offset in a6.
- `compare_nibble_and_branch`: Clears register d0, loads a masked byte from offset 3 of a structure into it, and compares it to a byte at a global pointer; if they are not equal, it branches, otherwise it checks if a word at a pointer in a6 is zero and branches accordingly.
- `increment_flag_byte`: It checks two memory locations; if both are zero, it increments a byte from a structure and stores the result in a global variable.
- `set_structure_bit_field`: It checks a memory location, and if it is zero, it takes a byte from a structure, shifts it left by four, and performs an OR operation with a byte at an offset in the structure.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
- `wait_for_vpos_change`: Loops until the value at the VPOS register changes, then loops until the value at the VPOS register is equal to the original value.
- `fill_data_table`: Performs a series of operations that effectively act as a data table of zeroed bytes.
- `tiny_addi`: Small routine of 1 instruction that adds 0 to d0.
- `store_zero_byte`: Contains a single zero byte used as data.
- `clears_data_register_zero_2`: Performs a logical OR operation on data register zero with zero.
- `copy_and_initialize_data_table`: Copies a series of data blocks from a global variable to a destination pointed to by a value within that same variable using nested loops.
- `copy_and_blit_data`: Copies a block of data from one location to another using a loop, then configures the Blitter to perform multiple transfers using parameters from a table.
- `execute_library_calls`: Calls five different routines in sequence and then returns.
- `blit_from_table`: Configures the Blitter to perform seven transfers using source and destination addresses and offsets derived from a table.
- `perform_blitter_transfers`: It configures the blitter with specific width, height, and color values, then performs seven transfers using source and destination addresses from a table.
- `blit_with_offsets`: Configures the Blitter with specific width, height, and color values, then performs seven transfers using source and destination addresses and offsets derived from a table.
- `copy_structured_data_2`: Uses nested loops to copy bytes from a source table to a destination table at specific intervals and offsets.
- `update_scaled_values`: Decrements a counter in a global variable and uses it to iterate through a table, reading a byte from a source table and storing a calculated value into a destination table.
- `compare_and_zero_byte`: Compares a byte in d5 to 32, branches if it is not equal, otherwise it clears d5 and branches to another location.
- `transform_byte_value`: Compares a byte in d5 to a constant, and if it is not greater than that constant, subtracts 48 from it, adds 27, and shifts it left by 4 bits before branching.
- `write_scaled_values_loop`: It subtracts sixty-four from a register, shifts it left by four, stores the result at a pointer in a0, increments a register, and loops based on a counter in d0.
- `decrement_global_pointer`: Decrements a long value at a global variable and, if the result is zero, branches; otherwise it returns.
- `check_table_byte`: It stores a word in a global variable, increments a value from a global variable, and checks a byte at the resulting offset in a table; otherwise it branches to another routine.
- `increment_and_check_byte`: Increments a 16-bit value in a register by one, checks the byte at the resulting offset in a table pointed to by a6, and either sets a global word to zero or to the incremented value based on whether that byte is zero.
- `reset_global_words`: Writes the value fifteen to one global word and zero to another global word before returning.

### Giger-Tetris (1993)(Panic Production)(AGA)(PD).adf:Setpatch

- `patch_dos_directory_hook`: Locates the DOS library and installs a hook to intercept directory listing requests.
- `call_and_test_d0`: Loads an address into a1, calls another routine, tests d0, and returns if d0 is not zero.
- `check_bit_and_return`: Tests bit 3 of a byte at offset 185 of a structure in a6 and branches if it is set; otherwise it clears d0 and returns.
- `allocate_copy_stub_and_install`: Copies a 51-word code stub into an allocated memory block and then invokes several library routines to install it.
- `probe_memory_size_and_mask`: Verifies a fixed low-memory byte holds an expected value, then probes memory downward from a fixed high address to locate where a self-referencing pointer test stops matching, and finally stores a bit mask derived from that result into a fixed-address byte.
- `mask_word_bit_eight`: Performs a bitwise AND on a word at offset 360 of a structure in a6 with the constant eight and returns the result.
- `free_patch_slots_clear_fields`: Clears a pointer field in a record and releases it through a library routine at offset minus 210 when it was set, then walks six consecutive pointer slots freeing each non-zero one while clearing a companion field, and returns 1.
- `clear_d0_byte`: Performs a bitwise OR on register d0 with zero.
- `branch_on_offset_values`: Compares a word at offset 1c of a1 to 28 and branches if equal, compares a word at offset 1c of a1 to 11 and branches if equal, otherwise it jumps to an address loaded from a memory location.
- `modify_structure_and_reply`: If a byte at offset 12 of a structure pointed to by offset 40 of a1 is 4, it updates several fields within that structure and calls a routine at an offset from a pointer at offset 4.
- `test_pointer_and_branch`: Tests if a long value at offset 44 of a structure in a1 is zero and branches if it is not, otherwise it branches to another location.
- `call_two_cia_routines`: Calls two other routines in turn and returns zero.
- `relink_flagged_list_entry`: Looks up an entry through the library routine at offset minus 276 and, when its top flag bit is set, unlinks it, clears the flag, and relinks it between two library calls that disable and enable the system, returning the entry it found.
- `compare_and_branch`: Compares a word at offset 28 of a1 with 5 and branches if they are equal.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `call_jump_vector_stub`: Calls the routine reached through a jump-table entry, which passes control on to a fixed address, and returns with no result.
- `save_console_lookup_result`: Calls the library routine at offset minus 96 and, when it returns a pointer, saves that pointer, the 32-bit value at offset 6 inside it, and the incoming base pointer into three global variables.
- `compare_stack_and_branch`: Loads a long value from the stack into d0 and compares it to a global constant, branching if d0 is higher than that constant.
- `jump_to_indirect_address`: Loads an address from a global variable and jumps to it.
- `execute_conditional_library_calls`: If a value in d0 is less than or equal to a global constant, it calls a library routine and, if the result is non-zero, calls another library routine and a third routine via a branch.
- `clears_data_register_d0`: Performs a logical OR operation on data register d0 with zero.
- `store_incoming_pointer_a1`: It saves the pointer value passed in register A1 into a global variable and then returns.
- `iterate_and_clear_structure_members`: Iterates through a linked list of structures, calling a routine for each element and clearing a value at offset 36 if a specific condition is met.
- `invoke_two_library_routines`: Invokes two library routines in sequence and returns one.
- `clears_data_register_d0_2`: Performs a bitwise OR operation on data register d0 with zero.
- `call_indirect_handler`: Invokes the routine stored at a fixed offset from a base pointer and returns.
- `store_offset_adjusted_lookup`: Selects one of two constant offsets depending on whether a structure field at offset 22 in a1 equals 44, then calls the library routine at a fixed offset from a6 and, if it returns a pointer, stores the chosen offset plus that pointer into a global variable.
- `clears_data_register_zero_3`: Sets the least significant byte of data register zero to zero.
- `clears_data_register_zero_4`: Performs a bitwise OR operation on data register zero using a zero constant.
- `update_scaled_pointer_data`: Calculates a new address by doubling a pointer in d1 three times, loads a long value from offset 32 of that address, compares it to a global constant, and if they are equal, stores that value into offsets 16 and 20 of the address.
- `jump_to_indirect_address_2`: Jumps to an address stored in a global variable.
- `disable_one_interrupt_source`: Calls two routines at fixed offsets from a base pointer, writes the value sixty-four to the interrupt enable register to disable an interrupt source, then calls a third routine and returns one.
- `store_argument_and_return_one`: Stores the value in a1 into a global variable and returns one.
- `zeroes_data_byte`: Performs a logical OR operation on a byte in register d0 with zero.
- `clears_data_register_zero_5`: Performs a logical OR operation on data register zero using a zero constant.
- `perform_multiple_zero_ors`: Performs a bitwise OR operation on register d0 with zero four times.
- `update_table_and_fill_data`: Calls a routine, compares the result with a value at an offset from a table, and if they match and a specific bit is set, fills a block of memory with the constant a fixed address using a loop.
- `add_lookup_result_to_cell`: Calls the library routine at offset minus 96 and, if it returns a non-zero value, adds that value into a fixed global cell twice.
- `increment_structure_byte`: Compares a long value on the stack to a constant and, if they are equal, increments a byte at offset 295 of a structure in a6 before returning.
- `modify_linked_list_node`: Compares a value on the stack with a constant, and if it matches, checks a bit in a value at an offset from a0 and potentially updates a pointer in a structure.
- `clear_flag_bit_then_call`: Takes a flag word from a library lookup when a configuration bit is set, or from a fixed low-memory address otherwise.
- `clears_data_register_d0_3`: Performs a bitwise OR operation on register d0 with the constant zero.
- `push_and_branch_if_not_zero`: Pushes a long value onto the stack, compares a0 to zero, branches if not zero, otherwise it clears the top of the stack, pushes d2 onto the stack, and branches to another location.
- `clears_d0_register`: Performs a logical OR operation on the data register zero with zero.
- `push_and_branch`: Pushes a long value from a memory location onto the stack and branches to another routine.
- `clear_data_register_d0`: Performs a bitwise OR operation on register d0 with a zero constant.
- `push_and_branch_if_not_zero_alt`: Pushes a long value onto the stack, compares a1 to zero, branches if not zero, otherwise it clears the top of the stack, pushes d0 onto the stack, and branches to another location.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `performs_logical_or_operation`: Performs a logical OR operation on register d0 with a constant value of zero.
- `push_two_values_and_branch`: Pushes a long value and then a constant value of two onto the stack before branching to another routine.
- `clear_d0_register`: Performs a logical OR operation on the d0 register with zero.
- `branch_conditionally_with_stack`: Pushes a long value from a memory location onto the stack, compares d0 to negative one, and either pushes a constant or branches to another routine.
- `clear_d0_register_2`: Clears the d0 register by performing a logical OR operation with zero.
- `push_address_and_branch`: Pushes a longword from a memory location into the stack and branches to another routine.
- `traverse_linked_list`: Calls a routine at an offset from a6, then traverses a linked list by comparing a word at offset 18 of a structure with a swapped value in d0, following pointers until a condition is met.
- `call_library_function_with_exchange`: Exchanges d0 and a6, calls a routine at an offset from a6, exchanges them back, and returns d0.
- `return_parameter`: Takes a single parameter and returns it unchanged without modifying it.
- `search_and_update_list`: Checks a bit in a structure, calls a library function, and then iterates through a list to find a specific value, updating a pointer in the list structure accordingly.
- `call_library_vector_636`: Library stub: loads a library base from a global variable and calls the library function at offset minus 636 of that base, returning its result to the caller.
- `null_operation_padding`: Performs a logical OR operation on a byte in a register with zero, effectively doing nothing.
- `jump_via_table_pointer`: Loads a pointer from a memory location, compares the word at that pointer to 24832, and either branches or jumps to the address at that pointer.
- `call_offset_routine`: Pushes a4 onto the stack, loads a word from offset 2 of a0 into d0, calculates an offset based on that word and offset 2 of a0 into a4, calls another routine, moves d0 to d1, restores a4 from the stack, and returns.
- `allocate_and_copy_string`: Allocates a vector using a library call, checks if the allocation succeeded, and if so, calculates the length of a string in a3, copies it to a new location, appends the length as a byte, and calls another routine.
- `count_bytes_and_copy_string`: Counts non-zero bytes in a string starting at a3, then copies the string to a new location and appends the count as a byte.
- `copy_and_clear_byte`: Copies a block of bytes from one memory location to another using a counter, stores a byte from register d2 into the destination, clears a byte at an offset from a pointer, and performs an indirect jump.
- `restore_registers_and_return`: Restores registers d2 through d7 and a2 through a3 from the stack, unlocks register a5, and returns.
- `clears_data_register_zero_6`: Sets the lowest byte of data register zero to zero.
- `call_set_signal`: Library stub: loads its arguments and the exec library base, calls SetSignal and returns its result to the caller.
- `decrement_and_jump_on_zero`: Increments a byte at offset 127 of a6, subtracts 1 from a word at offset 14 of a0 and branches if not zero, otherwise it clears a long at offset 40 of a0, subtracts 1 from a word at offset 44 of a0, and jumps to an address relative to a6.
- `decrement_and_branch`: If the negative bit is set, it decrements a word at offset 44 of a structure in a0 and branches to an earlier instruction; otherwise it returns.
- `call_two_routines_and_branch`: Calls a routine at an offset from a6, then calls another routine using a pointer from the stack, and finally branches to an earlier instruction.
- `insert_into_list`: Updates a doubly linked list by modifying pointers at offsets 4 and 16 of a structure, and calls a routine at an offset from a6 if a bit in a value at offset 8 is not set.
- `insert_into_doubly_linked_list`: Increments a value at an offset from a2, calls a routine, and if the result is non-zero and a bit is set, inserts a node into a doubly linked list.
- `increment_counters`: Increments a byte at offset 359 of a6, increments a word at offset 44 of a0, and if the word is not zero, increments a word at offset 14 of a0 before jumping to a routine at an offset from a6.
- `clear_bit_and_call_routines`: If a value at an offset from a0 is non-zero, it clears bit 4 of a byte at an offset from a global pointer, then calls two routines and a third routine via a jump.
- `clears_data_byte`: Performs a bitwise OR operation on a byte in register d0 using a zero constant.
- `call_stack_check_handler`: Performs the same stack-bounds and status-word test as its neighbours.
- `clear_data_register_d0_3`: Clears the data register d0 by performing a logical OR operation with zero.
- `verify_stack_bounds_and_call_handler`: Checks if the current stack pointer is within the bounds defined by a structure at offset 420 of a6, and verifies that a 16-bit value at offset 44 of a structure at offset 412 of a6 is not equal to negative one; if these conditions are met, it calls a routine through a pointer located at a relative offset from the program counter, otherwise it returns.
- `check_and_jump_record`: Checks if a word at offset 32 of the structure in a0 has bit 15 set; if not, it checks if a byte at offset 5 of a structure pointed to by a pointer at offset 4 of a0 is equal to 8, and if so, jumps to a new address.
- `rotate_record_words_around_call`: Moves eight 32-bit words inside a record by one pair of words, calls a routine through a global pointer, moves the words back the other way, and then calls a library routine.
- `check_byte_and_bit`: Compares a byte at offset 5 of a1 to 8, and if equal, checks if bit 8 of a word at offset 32 of a0 is set before jumping to a zero address.
- `rotate_two_record_blocks`: Calls a library routine, then moves eight 32-bit words by one pair inside two separate records: one reached through a pointer in the input record and one at the address the library call leaves in its address register.
- `rotate_two_blocks_then_library`: Moves eight 32-bit words by one pair inside two records addressed by two address registers, then calls a library routine and returns.
- `compare_record_word`: Compares a word at offset 6 of the structure in a0 to a constant and branches if they are equal.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `forward_record_fields_as_arguments`: Reads ten 32-bit fields from the record in the input address register, masks the upper word of the second field, and passes all ten values as arguments to one unnamed routine.
- `call_routine_only`: A routine that simply calls another routine and returns without performing any additional operation.
- `update_linked_list_node`: If d0 is not zero, it writes a constant to offset 34 of a structure in a1, then follows a pointer chain to update a value at an offset calculated from a global variable and a pointer in a structure.
- `restore_registers_and_return_2`: Restores registers d2, d3, and a3 from the stack and returns.
- `clear_data_register_d0_2`: Performs a logical OR operation on data register d0 with a zero constant.
- `insert_into_list_and_permit`: Calls a routine to calculate a value, increments a byte at a fixed address, and then inserts a structure into a linked list at a location determined by a calculated index before calling a library routine.
- `store_value_then_permit`: Stores a 32-bit value from one address register into the location held in another address register, then calls the exec Permit routine through the library base.
- `sum_longword_bytes_masked`: Adds the four bytes of the 32-bit value in the data register together and returns the sum masked to its low three bits.
- `update_list_pointer`: Increments a byte at offset 199 of a2, then if a pointer at offset 4 of a structure in a3 is not zero, stores a3 into the location pointed to by the value at offset 4 of a3.
- `set_bucket_head_record`: Calls another routine to get an index, sets a record at that index in a list to the value in a3, and if a3 is not zero, sets the next pointer of that record to the value in a2, finally calling a library function.
- `perform_zero_operation_2`: Performs a logical OR on register d0 with zero.
- `search_list_for_matching_record_2`: Calculates an index, increments a byte at a fixed address, and then retrieves a pointer from a list at that index.
- `search_list_for_matching_record`: Walks a linked list starting at the node passed in and stops at the first node whose key field equals the search value.
- `process_result_record_if_present`: Calls a library routine that returns a pointer, then if the pointer is not null, passes a field from that pointer and the pointer itself to another routine.
- `clears_data_register_with_zero`: Performs a logical OR operation on the data register zero with a zero constant.
- `validate_header_data`: Checks if a byte at offset 8 of a structure in a0 is 18, a byte at offset 14 is 2, and a byte at offset 15 is either 1 or 2; if these conditions are met, it calls a routine via a pointer in a global variable, otherwise it calls another routine.
- `process_result_with_fallback`: Pushes a3 onto the stack, calls another routine, then pushes a6 and a3 to call a library routine via a pointer at an offset from a6, and finally branches to another routine.
- `process_result_with_different_constant`: Pushes a3 onto the stack, calls another routine, then pushes a6 and a3 to call a library routine via a pointer at an offset from a6 using a constant of 58 in d0, and finally branches to another routine.
- `forward_to_fallback_handler`: Calls a routine via a pointer stored at a fixed offset from the program counter and returns.
- `execute_library_function_via_pointer`: Calls a library function using a pointer from a structure, then uses the result to jump to a function pointer stored at an offset in the returned structure.
- `clears_data_register_d0_4`: Performs a logical OR operation on register d0 using a zero constant.
- `update_function_pointer`: Calls a routine via a pointer, and if the returned word is 4, stores the address of this routine into an offset of 148 of the returned structure.
- `store_library_base_globals`: Copies the second address register and the base register into two global slots and returns the value one.
- `perform_zero_operation_3`: Performs a logical OR on register d0 with zero twice.
- `compare_long_value`: Compares a long value at offset 4 of a1 to 2000 and branches to an earlier instruction if they are equal; otherwise it jumps to a zero address.
- `verify_and_patch_pointer`: Compares a word at an offset of 28 from a pointer in a1 with 9, and if it matches and the pointer in a1 is non-zero, compares a long value from a table with a value from another table before writing 2968 to an offset of 4 of the structure in a1.
- `setup_stack_frame`: Links a stack frame, saves several registers to the stack, moves the value from d0 to d7, saves a0 and a1 to the stack, clears d6, and branches to another routine.
- `compare_string_bytes`: Calculates two addresses using a base pointer in a5 and an offset in d6, compares the bytes at those addresses, and if they are non-zero and unequal, calculates the difference between them as a long value and branches.
- `increment_and_compare`: Increments d6, compares it to d7, and if d6 is less than d7, clears d0, restores registers from the stack, unlinks a5, and returns.
- `save_pointer_and_zero_register`: Saves d7 and a3 to the stack, moves the value in a0 to a3, and sets d7 to zero before branching.
- `find_terminator_index_from_offset`: Advances from an index held in a data register through a byte array and returns the index of the first zero byte after it.
- `probe_scsi_device_inquiry`: Opens library resources and sends a six-byte SCSI command with opcode 18 through scsi.device using a 256-byte reply buffer; if successful, it may adjust an offset using a 16-bit record field and clear one byte at that offset before returning 1, otherwise it returns 0.

### Giger-Tetris (1993)(Panic Production)(AGA)(PD).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
