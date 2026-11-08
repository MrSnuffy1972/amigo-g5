# Game 911 (tornado ground attack (1991)(energize)[cr flt]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00911). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902512 top-level bytes in 17 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 165 of 165 named (0 author symbols), 165 of 165 described (6 by rule, 154 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 306 of 306 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4521 of 6040 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 785720 |
| code | 26124 |
| data | 86647 |
| padding | 3509 |
| unused | 512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Tornado Ground Attack (1991)(Energize)[cr FLT].adf:game.prg | 4 | 5411 | 153 |
| Tornado Ground Attack (1991)(Energize)[cr FLT].adf:echo | 2 | 117 | 2 |
| Tornado Ground Attack (1991)(Energize)[cr FLT].adf:FLT | 1 | 155 | 3 |
| Tornado Ground Attack (1991)(Energize)[cr FLT].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 347 | 6 |

## Cases handled

- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- tornado ground attack (1991)(energize)[cr flt]/Tornado Ground Attack (1991)(Energize)[cr FLT].adf:FLT: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `set_adkcon_value` (runtime segment an address): Writes the value 255 to the ADKCON register and returns. Kind: disk hardware/register configuration check. Pass path: execution continues to the next instruction after the call.

## Functions


### Tornado Ground Attack (1991)(Energize)[cr FLT].adf:game.prg

- `dispatch_initial_routines`: Calls three subroutines via jump instructions and then jumps to a location stored in a global pointer.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `decrement_global_loop`: Initializes a 16-bit global variable to 250, performs a division of 24 by 3, calls another routine, and then decrements the global variable in a loop until it reaches zero.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `decrement_global_variable`: Initializes a global variable to 250 and then decrements it in a loop until it reaches zero.
- `clear_d0_register`: Performs a bitwise OR operation on register d0 with zero.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `calculate_joystick_delta`: Reads bytes from the mouse port, subtracts them from values stored in a global table, updates that table with the new port values, and then performs range-limited clamping on the resulting differences.
- `subtract_from_constant`: Subtracts a 16-bit value in d2 from a constant and branches to another location.
- `calculate_and_store_difference`: Subtracts a value in d2 from 255, stores the result in a longword at a global address, and if the result is negative, sets that global address to zero before branching.
- `clamp_and_transfer_value`: Compares a word in a global variable against 320 and sets it to 319 if it is lower, then evaluates a value in a data register to either clear a data register, move the value to another data register, or branch based on specific range checks.
- `subtract_and_branch`: Loads 65281 into d6, subtracts d3 from d6, and branches to another location.
- `subtract_and_store_result`: Subtracts the value in d6 from d3, stores the result in a global variable at offset 368, and if the result is not greater than or equal to zero, it sets that variable to zero and branches.
- `compare_and_set_word`: Compares a word at a specific memory location against 200; if the value is less than 200, it stores 199 at that location and returns, otherwise it returns immediately.
- `clears_data_register_zero`: Performs a logical OR operation on data register zero with zero.
- `clear_data_register_zero`: Performs a logical OR operation on the lowest byte of data register zero with zero.
- `process_input_status`: Clears three memory locations, then reads the CIAA and custom chip registers to update two global variables based on bitwise manipulations of the input state.
- `call_stored_function`: Checks if a byte at a specific offset in a structure is 1; if not, it sets that byte to 0, calls a function pointer retrieved from a table, stores the result in another offset of the structure, and if the result is non-zero, sets the original byte to 1.
- `clears_data_register_zero_2`: Performs a logical OR operation on the least significant byte of data register zero with zero.
- `clears_data_register_zero_3`: Performs a logical OR operation on the least significant byte of data register zero using zero as the operand.
- `clears_data_register_d0`: Performs a logical OR operation on register d0 with zero.
- `clears_data_register_zero_4`: Performs a bitwise OR operation on data register zero with zero.
- `clears_data_register_d0_2`: Performs a logical OR operation on data register d0 with zero.
- `execute_three_subroutines`: Calls three different subroutines in sequence and then returns.
- `execute_file_operations`: Performs a series of indirect function calls and updates global variables, involving file system operations.
- `execute_file_operation_sequence`: Loads three pointers from global variables, calls process_file_operations, sets a 16-bit global variable to zero, and then conditionally calls empty_stub, empty_stub_2, and another routine.
- `process_file_operations`: Loads three pointers from global variables, calls a routine, sets a 16-bit global variable to zero, and then conditionally calls three different subroutines via a table in a6 based on the results of previous calls.
- `move_long_word`: Moves a long word from the stack to a specific memory address.
- `update_indexed_lookup_index`: Calculates a new index by multiplying a global variable by a constant and adding it to a base value, then stores the lower bits of the result back into the global variable.
- `wait_for_dma_completion`: Polls a bit in the DMA controller register until it is cleared.
- `enable_dma_channel`: Writes the value 33792 to the DMA control register to enable a specific DMA channel after calling another routine.
- `wait_for_dma`: Calls another routine, writes the value 1024 to the DMA control register, and calls that routine again before returning.
- `zero_byte_data`: Performs a logical OR operation on a byte in register d0 with zero.
- `clears_data_register_zero_5`: Performs a logical OR operation on register d0 using a zero constant.
- `save_and_restore_registers`: Saves d0 through d7 and a0 through a7 to the stack, calls another routine, restores the registers from the stack, and returns.
- `render_blt_area_sequence`: Calculates dimensions and memory offsets for a series of Blitter operations based on input coordinates.
- `dispatch_function_pointers`: Calls functions through pointers retrieved from a global structure and updates a global variable with the result of a conditional routine call.
- `release_system_resources_3`: Sets bit 0 of the CIAA peripheral address register, calls a routine to obtain a pointer, writes a long value of all ones to offset 184 of that pointer, and then conditionally calls several other routines via pointers stored in global variables.
- `release_system_resources_2`: Calls a routine with 100 in d0, then checks a global variable to conditionally call a routine via a pointer, calls two more routines via pointers from a global variable, and finally calls a routine using a value from a6.
- `release_system_resources`: Checks a global variable, and if non-zero, clears it, calls two routines through pointers, and then conditionally closes a library and calls another routine through a pointer derived from a global variable.
- `increment_global_counters`: Increments three global variables and conditionally performs a bitwise NOT operation on a fourth global variable.
- `poll_long_word`: Loops until a long word at a specific memory location is non-zero.
- `initialize_device_handler`: Calls two routines to store their return values in global variables, then if the second value is non-zero, calls a routine via a pointer from a global variable and performs further operations on a structure.
- `manage_message_ports`: If a pointer in a global variable is non-zero and its offset 20 is non-zero, it sets a field at offset 40 to a constant, sets a field at offset 40 to 10, and calls two functions; it then calls a third routine and jumps to a fourth routine using a pointer from offset 2 of the original structure.
- `update_status_byte`: Checks a byte at offset 4 of a structure in a0; if it is 1, it verifies a signed word at offset 6 is non-negative, checks bit 4 of a word at offset 8, and if bit 2 of a byte at offset 9 is clear, it clears that bit and potentially updates a global word with the value from offset 6.
- `store_word_to_memory`: Stores the contents of register d1 into a memory location and returns.
- `branch_to_offset`: Performs a short branch to an offset relative to the current instruction pointer.
- `tiny_move_rts`: Small routine of 2 instructions that copies a0 to d0, then returns.
- `allocate_and_initialize_structure`: Calls several functions to allocate memory and signals, then populates a structure with data from a global variable and other sources.
- `free_message_port`: Validates a pointer, invokes cleanup routines, sets specific bytes to zero or maximum values, and frees associated memory and signals.
- `set_handler_data`: The routine checks a pointer argument and, if non-zero, calls a function obtained from a global variable table.
- `reset_object_flags`: The routine clears a byte at an offset in a structure pointed to by a parameter and sets two 32-bit fields to all ones, then calls another routine via an indirect pointer from a global variable and returns its result, or returns a constant if the parameter is zero.
- `clear_data_register_zero_2`: Performs a logical OR operation on a byte in data register zero with zero.
- `update_table_pointer`: Executes a function via a pointer from a global variable and stores a value from another global variable into a local variable.
- `initialize_display_environment`: Calls two functions using pointers derived from a global variable, including calls related to opening the workbench and remaking the display.
- `swap_and_toggle`: Swaps two global variables and toggles a bit in a third global variable.
- `check_and_clear_word`: Tests a 16-bit value in a global variable and, if non-zero, clears it and loads a pointer from another global variable into a0 before returning.
- `fill_table_with_increments`: Fills a table with four 32-bit values, incrementing the source value by a constant amount for each entry in the loop.
- `copy_data_with_offset`: Copies sixteen 16-bit words from a source to a destination, while also writing each word to an offset of thirty-two words within the destination buffer.
- `execute_main_loop`: Calls a long sequence of subroutines, updates several global counters and pointers, and enters a loop that increments a counter and a global word until a specific condition is met.
- `wait_for_variable_change`: Enters a busy-wait loop that continues as long as a global variable is equal to one, followed by another loop that waits for it to become zero.
- `process_entity_table_and_update_state`: Iterates through a table of entries, and for each non-zero entry with a value greater than 150, it clears the entry and calculates an offset into a data structure to update specific word values.
- `find_empty_table_entry`: Calls a routine, then iterates through a table starting at a global address for up to 39 entries to find a word that is zero, and if found, initializes that entry with several constant values and specific register contents.
- `populate_data_structure`: Checks a global word against a limit and returns if it is lower, otherwise it searches a table for a zero entry and populates it with data copied from a source or with specific calculated values.
- `update_counter_values`: Modifies two different global counters based on the state of a bitmask in a third global variable, applying increments or decrements within specific bounds.
- `process_data_blocks`: Iterates through a series of data blocks, passing parameters to another routine to perform operations on memory locations derived from a global pointer.
- `process_data_blocks_2`: Decrements bytes in a table based on a count in a global variable, and if a byte reaches zero, sets it to 9 and decrements the value at the previous position; it also conditionally updates a global pointer and calls another routine.
- `process_structure_list`: Iterates through a list of structures, updating a pointer and performing a routine call if a specific field in the structure is non-zero.
- `adjust_counters_based_on_thresholds_2`: Iterates through a list of structures, performing operations on memory locations calculated from table offsets and updating counters based on comparisons.
- `adjust_counters_based_on_thresholds`: Calculates an offset from a source value to access a table, then increments or decrements two separate counters based on comparisons with values stored in that table.
- `process_table_entries`: Iterates through a table, and for each entry where a word at offset 12 is zero, it calls another routine and, if a masked value from a global variable is within a specific range, updates offsets 8, 10, and 12 of the entry; otherwise, it decrements offset 10 and increments offset 12, capping the latter at 100.
- `update_audio_parameters`: Iterates through a table, and for each entry where the first word is zero, it calls another routine multiple times to update offsets 2, 4, 6, and 18 using masked values from a global variable; otherwise, it either clears the first word if a word at offset 4 is less than negative 64 or decrements offset 4 by a value from offset 6.
- `iterate_linked_list`: Traverses a list of structures, performing operations on memory locations calculated from table offsets and updating a pointer to the next element in the list.
- `execute_sequence_logic`: Uses a global index to select a memory location and performs a series of operations involving another routine and multiple memory destinations.
- `distribute_data_to_sparse_table`: Iterates through a source buffer and writes each 16-bit word to a specific, non-contiguous pattern of locations within a large destination table.
- `copy_interleaved_blocks`: Copies 32-bit words from a source buffer to a destination buffer using a nested loop and an incrementing global index.
- `copy_block_to_destination`: Copies a series of 32-bit words from a source buffer to a destination buffer, using a global index that wraps around after reaching a certain limit.
- `clear_memory_block`: Fills a large block of memory starting at a global pointer with zeros using a loop.
- `initialize_value_table`: Fills a table with incrementing values starting from zero, using a fixed step size for each entry.
- `mask_return_value`: Calls another routine and stores the lower nine bits of its return value into a global variable.
- `process_table_entries_2`: Iterates through a table, performs difference calculations between table values and other data, and if certain conditions are met, updates global pointers and calls multiple routines.
- `clear_data_register_d0`: Performs a bitwise OR operation on the least significant byte of data register d0 with zero.
- `search_table_entry`: Iterates through a table to find an entry matching specific coordinate criteria and initializes it with constant values.
- `validate_table_coordinates`: Checks a table for entries that fall within a specific range of coordinate values.
- `clear_memory_buffers`: Sets 8000 consecutive 32-bit words to zero using two parallel pointers loaded from global variables.
- `initialize_and_populate_dual_buffers`: Clears two large memory buffers, executes a nested loop of subroutine calls to populate specific memory offsets, and then performs a structured data copy from a global table into the buffers followed by a secondary initialization of specific memory segments within those buffers.
- `reorganize_data_buffer`: Copies specific words from various offsets within a source table into two destination buffers in a loop that runs 127 times.
- `process_nested_tables`: Performs a nested loop to iterate through a table and call another routine with calculated offsets and values.
- `copy_sprite_data_blocks`: Calculates offsets into two source tables based on values read from a third table and performs a block copy of data into a destination structure.
- `update_indexed_data`: Updates a value in a table based on an index, clamps the value within a specific range, and calls another routine using derived pointers.
- `clear_d0_register_2`: Performs a logical OR on register d0 with zero.
- `clears_data_register_zero_6`: Performs a logical OR operation with zero on data register zero.
- `wrapper_for_subroutine`: Saves all data and address registers to the stack, calls another routine, and then restores the registers from the stack before returning.
- `initialize_high_score_data`: Checks a global flag and a status value to perform a recursive initialization sequence that populates a table with ASCII numeric characters, clears several memory blocks, and performs a large-scale data copy into a structured array.
- `clear_large_buffer_array`: Performs a nested loop to clear multiple large memory blocks to zero and calls another routine.
- `compare_and_swap_tables`: Compares bytes at offsets 3 through 8 between two tables in a nested loop, and if all bytes are equal, swaps 11 bytes starting at offset 3 between the two tables.
- `execute_repeated_tasks`: Calls another routine ten times within two separate loops using different starting addresses and parameters, then calls two more routines before returning.
- `render_multiple_objects`: Calls another routine multiple times to process a sequence of objects using values from a table and updates a global variable.
- `render_multiple_objects_alt`: Iterates through a set of objects using a table index to call another routine with specific parameters and updates a global variable.
- `initialize_system_state`: Sets several global variables and constants, then executes multiple loops that call other routines to perform various initialization tasks.
- `clear_data_register_d0_2`: Performs a bitwise OR operation on data register d0 with zero.
- `process_data_list`: Iterates through a list of data, calling another routine for each entry and incrementing a pointer.
- `update_global_pointers_recursively_2`: Reads a word and a second word from a table pointed to by a global variable, updates a global pointer using a multiplication of the second word by 640, and conditionally calls other routines based on the value of the first word.
- `copy_data_and_set_pointers`: Copies 4949 bytes from a source table to a destination global variable, then stores that destination address and the value 330 into two other global variables before returning.
- `decrement_counter_and_adjust`: Decrements a global counter and modifies two other global values based on a conditional check.
- `update_global_pointers_recursively`: Updates several global variables with values from two other global variables and then calls itself recursively four times with varying offsets and combinations of these values.
- `check_memory_value`: Calculates an offset based on global values and checks if the word at that location is less than four.
- `execute_two_routines`: Saves all registers to the stack, calls two specific routines, and then restores the registers before returning.
- `loop_through_table_data`: Iterates through a table of bytes, using each byte to calculate a destination address for another routine.
- `loop_through_table_data_alt`: Uses a loop to process a table of bytes, passing specific values to another routine via calculated pointers.
- `iterate_and_update_table_entries`: Updates several global variables before entering a loop that calculates a pointer based on a value from a string and calls another routine.
- `clears_data_register_zero_7`: Performs a logical OR operation on the least significant byte of data register zero using a zero constant.
- `increment_and_clear_byte`: Increments a byte at a specific memory location and, if the value is greater than 9, clears the byte at that location and decrements the pointer.
- `increment_and_nullify_byte_sequence`: Increments a byte at a specific memory location and continues to decrement the pointer to write zero bytes backward until a byte greater than nine is encountered or the sequence is terminated.
- `swap_two_words`: Swaps the values of two adjacent sixteen-bit words stored in global memory.
- `update_table_entries`: Checks a global pointer against various constant values and, if a match is found, calls a specific routine twice followed by another routine twice.
- `update_table_entry`: Calculates a pointer into a table based on a global variable and updates several global values before calling another routine.
- `execute_swap_and_update`: Saves all registers to the stack, calls a routine twice, and then calls another routine twice before returning.
- `update_table_entry_2`: Sets a table pointer using a global variable and updates specific global parameters before calling another routine.
- `execute_sequential_word_swaps`: Saves all data registers and address registers to the stack, calls swap_two_words and calculate_indexed_address in an alternating sequence twice, restores the registers, and returns.
- `calculate_indexed_address`: Calculates an address by adding a global variable to the product of that variable and a constant, then stores the result in a global variable.
- `execute_sequential_updates`: Saves all data registers and address registers to the stack, calls swap_two_words and update_table_entry_3 twice in alternating order, restores the registers, and returns.
- `update_table_entry_3`: Calculates a table offset from a global variable and updates global values before calling another routine.
- `update_recursive_audio_state`: Updates audio parameters through a recursive process that checks global state flags and modifies values in a table.
- `set_table_parameters`: Calculates a table pointer using a bit-shifted global variable and stores two other global values into specific locations.
- `set_table_parameters_2`: Calculates a table pointer by scaling a bit-shifted value from a global variable and stores two other global values and a constant into specific memory locations, then calls another routine.
- `parse_string_data`: Iterates through a string, skipping spaces and using numeric characters to calculate table offsets and update global values.
- `parse_mission_string`: Sets several global pointers and values, then iterates through a string to identify specific characters and updates several global variables and pointers based on the characters found.
- `execute_nested_decrement_loops`: Performs two nested loops that decrement registers until they reach a specific value.
- `parse_input_string`: Clears a global word, calls two routines, and then enters a loop that reads a byte from a table, checks if it is a specific character, and either stores it in a buffer or performs conditional pointer and counter updates.
- `copy_and_interleave_data`: Copies data from a source table into a new location while interleaving the word values into a different memory structure.
- `copy_data_block`: Copies a block of data from a source table to a destination global variable using a loop.
- `copy_memory_range`: Copies a block of memory of a fixed size from a global source to a destination.
- `initialize_mission_data`: Copies 5120 bytes from a subroutine to a global variable, performs four calls to another routine with varying parameters, copies a block of data from a table into a structure using a loop, and then updates a global counter and potentially calls another routine.
- `interleave_table_data`: Calls a routine with a constant, then enters a loop that interleaves word values from a source table into a destination structure, decrements a byte at a specific offset, and calls another routine in a loop.
- `process_byte_pair`: Uses two bytes from a pointer to calculate two different table offsets and updates global parameters.
- `load_level_data`: Compares a global word against four constants and, if a match is found, stores one of four different longwords into a global variable and calls another routine.
- `transform_data_block`: Performs a series of bitwise inversions and byte swaps on a data block before copying a large range of memory.
- `initialize_music_parameters`: Sets a global variable to zero, assigns its address to two other globals, and calls a routine with a specific constant value.
- `reset_state_variables`: Calls two routines to reset system state and then toggles a global variable between zero and one.
- `initialize_music_parameters_alt`: Sets a global variable to zero, assigns its address to two other globals, and calls a routine with a different constant value.
- `store_data_table`: Acts as a data table containing sequences of zeroed data registers that are accessed by other routines via absolute long addressing.
- `initialize_music_system`: Walks a byte table to build a pointer table for music samples, clears several global variables, and resets various counters and values.
- `reset_hardware_registers`: Writes a sequence of values to several global variables that correspond to hardware control and volume registers.
- `update_audio_parameters_2`: Checks two global variables, then increments a byte at a memory location and performs several conditional branches to call subroutines that update audio hardware registers.
- `update_audio_parameter_structure`: Reads an index from a structure to fetch audio data from a table, updates several fields in the structure including a pointer and two 16-bit values, and modifies hardware audio registers or a global variable based on a command byte.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `update_structure_value`: Reads a 16-bit value from a structure, masks the lower 12 bits, and uses the lower 4 bits of a byte at offset 2 to determine whether to add, subtract, or clamp a value at offset 16 or 18, while updating a result field at offset 6 of another structure.
- `stores_address_to_vector`: Stores the address of the next instruction into a global variable at offset 112.
- `execute_recursive_audio_updates`: Compares a global word to zero and, if non-zero, calls itself twice and then calls routines to update audio channel parameters and hardware configuration.
- `update_audio_channel_parameters`: Selects an audio channel based on a table index and, if not already active, updates the hardware audio registers with a new frequency, period, and volume derived from a data structure.
- `update_hardware_configuration`: Checks four word-sized values in a table; if any equal one, it writes specific values to the interrupt enable and DMA control registers, then clears all four values in the table before returning.

### Tornado Ground Attack (1991)(Energize)[cr FLT].adf:echo

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `transfer_register_and_branch`: Moves the contents of register d1 to d0 and then performs a short branch to another location.

### Tornado Ground Attack (1991)(Energize)[cr FLT].adf:FLT

- `process_recursive_bit_shifts_2`: Performs a series of recursive calls based on the number of right shifts required to clear a value in d0, updating d2 through a loop and calling subroutines.
- `extract_and_store_byte`: Loads a longword from a pointer in a0, stores the lower byte of that value into a 16-bit word at a location pointed to by a5, and then performs a logical right shift on the longword in d0 before returning.
- `process_recursive_bit_shifts`: Performs a series of recursive calls based on the number of right shifts required to clear a value in d0, updating d2 through a loop and calling another routine.

### Tornado Ground Attack (1991)(Energize)[cr FLT].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_adkcon_value`: Writes the value 255 to the ADKCON register and returns.
- `update_audio_channels_via_index`: Increments a global index and, if it exceeds a limit, resets it and calls another routine four times to update the four audio hardware channels.
- `update_audio_parameter_structure_2`: Reads a value from a structure, uses it to index into a table to fetch audio properties, and updates the structure with new frequency and volume data.
- `mask_and_compare_word_2`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `update_structure_field_via_mode`: Reads a mode from a structure in a6 and performs different arithmetic or table-lookup operations on a 16-bit value at offset 16 based on that mode.
- `store_pointer_to_memory`: Stores the constant value an address in a memory location at offset 112.
