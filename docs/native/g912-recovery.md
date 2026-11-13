# Game 912 (tornado ground attack (1991)(energize)[cr flt][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00912). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901344 top-level bytes in 19 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 142 of 142 named (0 author symbols), 142 of 142 described (5 by rule, 133 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2773 of 2773 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4164 of 4164 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 785720 |
| code | 18660 |
| data | 92889 |
| padding | 3563 |
| unused | 512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Tornado Ground Attack (1991)(Energize)[cr FLT][a].adf:game.prg | 4 | 3867 | 133 |
| Tornado Ground Attack (1991)(Energize)[cr FLT][a].adf:echo | 2 | 117 | 2 |
| Tornado Ground Attack (1991)(Energize)[cr FLT][a].adf:FLT | 1 | 155 | 3 |
| Tornado Ground Attack (1991)(Energize)[cr FLT][a].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |

## Cases handled

- 9 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m3000): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- tornado ground attack (1991)(energize)[cr flt][a]/Tornado Ground Attack (1991)(Energize)[cr FLT][a].adf:FLT: DefJam Cruncher 3.2 (xfd) stream found by ancient scan (13588 -> 98304 bytes) but only 0 of 10 sampled windows are in reference RAM: not used
- tornado ground attack (1991)(energize)[cr flt][a]/Tornado Ground Attack (1991)(Energize)[cr FLT][a].adf:FLT: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Tornado Ground Attack (1991)(Energize)[cr FLT][a].adf:game.prg

- `dispatch_initial_routines`: Calls three different routines and then jumps to a location stored in a global pointer.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `decrement_global_loop`: Initializes a global variable to 250, then enters a loop that calls another routine and decrements the global variable until it reaches zero.
- `decrement_global_variable`: Initializes a global variable to the constant zero-fa, then decrements it in a loop until it reaches zero.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `calculate_joystick_delta`: Reads bytes from the joystick port, subtracts them from previously stored values in a global table, and updates the global table with the new port values before clamping the resulting difference into a global variable.
- `subtract_and_branch_2`: Subtracts a 16-bit value in d2 from 255, stores the result in d6, and branches to another location.
- `update_word_threshold`: Subtracts a value in d2 from 255, stores the result in a global variable, and if that variable is non-negative, sets it to 0 and branches to another location; otherwise it sets it to 0.
- `compare_and_update_word`: Compares a word at a memory location against 384; if it is less than 384, it stores 383 at that location, otherwise it performs several comparisons and conditional moves involving d3 and d6 before returning.
- `subtract_and_branch`: Subtracts the value in d3 from 65281 in d6 and then branches to another location.
- `subtract_and_store_result`: Subtracts the value in d3 from zero-ff in d6, subtracts d6 from a value at offset 190 of a structure, and if the result is not greater than or equal to zero, sets the value at offset 190 to zero.
- `compare_and_set_word`: Compares a word at a specific memory location against 200; if the value is less than 200, it stores 199 at that location and returns, otherwise it returns immediately.
- `process_input_status`: Clears three values in a structure, then reads hardware registers to update two different global variables based on the state of bits in the CIAA and custom hardware registers.
- `allocate_memory_if_needed`: Checks if a global variable is not one, and if so, sets it to zero, calls a library routine to allocate a block of memory of size 86563 with a flag of 65539, stores the resulting pointer in a global variable, and sets the first variable to one if the pointer is not zero.
- `execute_three_subroutines`: Calls three different subroutines in sequence and then returns.
- `execute_file_operations`: Performs a series of indirect function calls and updates global variables, involving file system operations.
- `execute_three_calls`: Calls three different routines via jumps and branches, then returns.
- `process_file_operations`: Stores three pointers from a0, a1, and d2 into global variables, calls a routine via a pointer, and then conditionally calls two more routines via offsets of a pointer retrieved from a structure in d0.
- `move_long_word`: Moves a long word from the address pointed to by the stack pointer to the address contained in the constant value four million five hundred sixty-one.
- `update_indexed_lookup_index`: If a global variable is zero, it initializes it by combining a value from a global variable with a bit-shifted version of itself, then calculates a new index by multiplying that global variable by a constant and adding it to a base value, finally storing the lower nine bits of the result back into the global variable.
- `wait_for_dma_completion`: Polls bit 6 of the DMA controller register until it is cleared, then returns.
- `enable_dma_channel`: Writes a value to the DMA controller register to enable a specific DMA channel.
- `wait_for_dma`: Polls the DMA controller register until the DMA completion bit is cleared, writes a value to the DMA controller register, and then polls it again.
- `save_and_restore_registers`: Saves d0 through d7 and a0 through a7 to the stack, calls another routine, restores the registers from the stack, and returns.
- `render_blt_area_sequence`: Calculates dimensions and memory offsets for a series of Blitter operations based on input coordinates.
- `dispatch_function_pointers`: Executes functions through pointers retrieved from a global structure and updates a global variable with the result.
- `initialize_system_resources`: Sets bit 0 of the CIAA peripheral register, calls a routine to obtain a pointer, writes a long value to offset 184 of the structure pointed to by that pointer, and then conditionally calls several other routines via pointers from global variables.
- `release_system_resources_2`: Calls a routine with 100 in d0, then conditionally performs several operations including calling routines through pointers from a structure and calling a library function via a pointer in a6.
- `copy_byte_and_call_routine`: Loads a pointer from a global variable, and if it is not zero, copies a byte from the address in a0 to a destination address calculated from the program counter, then calls a routine via a pointer in a6 if the byte copied was not zero.
- `release_system_resources`: Clears a byte in a global variable, calls two routines through pointers, and then uses a pointer from a global variable to call a library function to close a library and another library function to release a resource.
- `increment_global_counters`: Increments three global variables and conditionally performs a bitwise NOT operation on a fourth global variable.
- `wait_for_non_zero`: Loops until the value at a global variable is non-zero, then returns.
- `initialize_device_and_handler`: Calls two routines to store their return values in global variables, then conditionally calls a routine using a pointer from a global variable and performs further operations on a structure if a specific condition is met.
- `manage_message_ports`: If a pointer in a global variable is not zero, it checks an offset in the pointed-to structure, sets a value at offset 40 and a word at offset 28, then performs two indirect function calls; it then processes a second pointer by performing two more indirect function calls and jumping to another routine.
- `update_status_value`: Checks a byte at offset 4 of a structure in a0; if it is 1, it verifies a word at offset 6 is non-negative and a bit in a word at offset 8 is not set, then updates a global variable with the word from offset 6 or sets it to zero if the word exceeds 70.
- `allocate_and_initialize_structure`: Calls several functions to allocate memory and signals, then populates a structure with data from a global variable and other sources.
- `free_message_port`: Validates a pointer, invokes cleanup routines, sets specific bytes to zero or maximum values, and frees associated memory and signals.
- `set_handler_data`: The routine checks a pointer argument and, if non-zero, calls a function obtained from a global variable table.
- `reset_object_flags`: The routine clears a byte at an offset in a structure pointed to by a parameter and sets two 32-bit fields to all ones, then calls another routine via an indirect pointer from a global variable and returns its result, or returns a constant if the parameter is zero.
- `update_table_pointer`: Executes a function via a pointer from a global variable and stores a value from another global variable into a local variable.
- `initialize_display_environment`: Calls two functions using pointers derived from a global variable, including calls related to opening the workbench and remaking the display.
- `swap_and_toggle`: Swaps two global variables and toggles a bit in a third global variable.
- `clear_global_word`: Tests a 16-bit value in a global variable and, if non-zero, clears it and loads a pointer from another global variable into a0; otherwise it returns.
- `fill_table_with_increments`: Fills a table with four 32-bit values, incrementing the source value by a constant amount for each entry in the loop.
- `copy_data_with_offset`: Copies sixteen 16-bit words from a source to a destination, while also writing each word to an offset of thirty-two words within the destination buffer.
- `execute_main_loop`: Calls many subroutines, sets several global pointers and values, and enters a loop that increments two global counters, calls many subroutines, and conditionally increments a global word up to a maximum value based on a comparison with a global word and a byte at a specific offset.
- `wait_for_variable_change`: Enters a busy-wait loop that continues as long as a global variable is equal to one, followed by another loop that waits for it to become zero.
- `process_entity_table_and_update_state`: Iterates through a table of entries, and for each non-zero entry with a value greater than 150, clears the entry and calculates an offset into a data structure to update specific word values.
- `find_empty_table_entry`: Calls a routine, then iterates through a table of entries using a loop to find the first entry where the first word is zero, initializing that entry with several constant values and values from registers.
- `populate_data_structure`: Searches a table for an empty entry and populates it with data copied from another memory location or with specific constant values.
- `update_counter_values`: Modifies two different global counters based on the state of a bitmask in a third global variable, applying increments or decrements within specific bounds.
- `process_data_blocks`: Iterates through a series of data blocks, passing parameters to another routine to perform operations on memory locations derived from a global pointer.
- `decrement_data_counters`: Decrements values at memory locations derived from a global variable and updates a pointer and a counter based on specific conditions.
- `process_structure_list`: Iterates through a list of structures, updating a counter and conditionally calculating a pointer to a table entry to call another routine.
- `adjust_counters_based_on_thresholds_2`: Iterates through a list of structures, calling another routine and conditionally updating counters or performing a division-based table lookup.
- `adjust_counters_based_on_thresholds`: Calculates an offset from a source value to access a table, then increments or decrements two separate counters based on comparisons with values stored in that table.
- `process_table_entries`: Iterates through a table of structures; if an entry's word at offset 12 is zero, it calls another routine and, if a masked value from a global variable is within a specific range, initializes the entry with values from registers; otherwise, it increments the word at offset 12 and decrements the word at offset 10, capping the value at 100.
- `update_dynamic_table`: Iterates through a table of structures; if the first word is zero, it performs several calls to another routine to update various offsets in the structure using masked values from a global variable; otherwise, it either sets the first word to zero if a value at offset 4 is less than negative 64, or decrements the word at offset 4 by the word at offset 6.
- `iterate_linked_list`: Traverses a list of structures, performing operations on memory locations calculated from table offsets and updating a pointer to the next element in the list.
- `execute_sequence_logic`: Uses a global index to select a memory location and performs a series of operations involving another routine and multiple memory destinations.
- `distribute_data_to_sparse_table`: Iterates through a source buffer and writes each 16-bit word to a specific, non-contiguous pattern of locations within a large destination table.
- `copy_interleaved_blocks`: Copies 32-bit words from a source buffer to a destination buffer using a nested loop and an incrementing global index.
- `copy_data_blocks`: Increments a global counter, uses it to index into a table to find a source address, and then performs a nested loop to copy multiple blocks of four longwords from the source to a destination address derived from a global pointer.
- `clear_memory_block`: Fills a large block of memory starting at a global pointer with zeros using a loop.
- `initialize_value_table`: Fills a table with incrementing values starting from zero, using a fixed step size for each entry.
- `mask_return_value`: Calls another routine and stores the lower nine bits of its return value into a global variable.
- `process_table_entries_2`: Iterates through a table, performs range checks on values, and if conditions are met, updates a global structure with values from another table and calls two subroutines.
- `search_table_entry_and_initialize`: Iterates through a table to find an entry where the difference between its coordinates and two global values is within specific bounds, then calls another routine multiple times to initialize that entry and clears its fields if a specific value is met.
- `validate_table_coordinates`: Iterates through a table of entries using a loop to find the first entry where the absolute difference between the values at offset 8 and a global word plus 6 is less than 7, and the absolute difference between the values at offset 10 and a global word plus 8 is less than 9, then calls another routine four times.
- `clear_memory_buffers`: Sets a large number of consecutive memory locations to zero using two parallel pointers.
- `initialize_and_populate_dual_buffers`: Clears two large memory buffers, executes a nested loop of subroutine calls to populate specific memory offsets, and then performs a structured data copy from a global table into the buffers followed by a secondary initialization of specific memory segments within those buffers.
- `reorganize_data_buffer`: Copies specific words from various offsets within a source table into a new destination buffer in a structured pattern.
- `process_nested_tables`: Performs a nested loop to iterate through a table and call another routine with calculated offsets and values.
- `copy_sprite_data_to_buffer`: Calculates an offset into a table based on a value in a global variable and uses that offset to fetch a block of data.
- `update_indexed_data`: Updates a value in a table based on an index, clamps the value within a specific range, and calls another routine using derived pointers.
- `call_subroutine_with_stack_save`: Saves all data registers and address registers to the stack, calls another routine, and then restores the registers from the stack before returning.
- `initialize_high_score_and_data_tables`: Decrements several global counters and uses recursion to trigger a sequence of initialization tasks, including converting numeric values to ASCII characters, clearing memory, and populating a large data structure by copying words into specific offsets of a table.
- `clear_large_buffer_array`: Performs a nested loop to clear multiple large memory blocks to zero and calls another routine.
- `compare_and_swap_tables`: Compares bytes at offsets 3 through 8 between two tables in a nested loop, and if all bytes are equal, swaps a sequence of bytes starting at offset 3 in both tables.
- `execute_repeated_tasks`: Calls another routine multiple times within two separate loops that iterate ten times each.
- `render_multiple_objects`: Calls another routine multiple times to process a sequence of objects using values from a table and updates a global variable.
- `render_multiple_objects_alt`: Iterates through a set of objects using a table index to call another routine with specific parameters and updates a global variable.
- `initialize_system_state`: Sets several global variables and constants, then executes multiple loops that call other routines to perform various initialization tasks.
- `process_data_list`: Iterates through a list of data, calling another routine for each entry and incrementing a pointer.
- `process_data_table`: Iterates through a table of structures, updating a pointer and modifying various global variables or memory locations based on the values found in the table.
- `copy_and_update_globals`: Copies 4949 bytes from one location to another using a loop, then stores a pointer and a word into two global variables.
- `decrement_counter_and_adjust`: Decrements a global counter and modifies two other global values based on a conditional check.
- `update_global_pointers_recursively`: Updates several global variables with values from two other global variables and then calls itself recursively four times with varying offsets and combinations of these values.
- `check_memory_value`: Calculates an offset using global values and a base address, then checks if the word at that calculated location is less than four; if so, it calls another routine twice with specific offsets.
- `execute_dual_subroutines`: Calls two specific subroutines and returns.
- `loop_through_table_data`: Iterates through a table of bytes, using each byte to calculate a destination address for another routine.
- `loop_through_table_data_alt`: Uses a loop to process a table of bytes, passing specific values to another routine via calculated pointers.
- `iterate_and_update_table_entries`: Updates several global variables before entering a loop that calculates a pointer based on a value from a string and calls another routine.
- `increment_and_check_byte`: Increments a byte at a fixed memory location and, if it is less than 9, sets it to zero and decrements the pointer before looping.
- `increment_and_clear_byte_sequence`: Increments a byte at a specific memory location and continues to decrement the pointer and set bytes to zero in a loop until a value greater than nine is encountered, otherwise it returns.
- `swap_two_words`: Swaps the values of two adjacent sixteen-bit words stored in global memory.
- `dispatch_table_updates`: Checks a global pointer against three different constant values and, if a match is found, calls a specific subroutine twice and another subroutine twice.
- `update_table_entry`: Calculates a pointer into a table based on a global variable and updates several global values before calling another routine.
- `execute_update_sequence_one`: Calls a subroutine twice and another subroutine twice before returning.
- `update_table_entry_2`: Sets a table pointer using a global variable and updates specific global parameters before calling another routine.
- `execute_sequential_subroutines`: Saves all data registers and address registers to the stack, calls swap_two_words and calculate_indexed_address twice in alternating order, restores the registers, and returns.
- `calculate_indexed_address`: Calculates an address by adding a global variable to the product of that variable and a constant, then stores the result in a global variable.
- `execute_alternating_subroutines`: Saves all data registers and address registers to the stack, calls two specific subroutines twice in an alternating sequence, restores the registers, and returns.
- `update_table_entry_3`: Calculates a table offset from a global variable and updates global values before calling another routine.
- `update_recursive_audio_state`: Updates audio parameters through a recursive process that checks global state flags and modifies values in a table.
- `set_table_parameters`: Calculates a table pointer using a bit-shifted global variable and stores two other global values into specific locations.
- `set_table_parameters_2`: Calculates a table pointer by multiplying a bit-shifted and masked value from a global variable by 160 and adding it to a base address, stores two other global values into specific locations, sets a global value to 16, and calls another routine.
- `parse_string_data`: Iterates through a string, skipping spaces and using numeric characters to calculate table offsets and update global values.
- `parse_mission_string`: Iterates through a string to identify specific characters and updates several global variables and pointers based on the characters found.
- `execute_nested_decrement_loops`: Performs two nested loops that decrement registers until they reach a specific value.
- `parse_input_string`: Clears a global word, reads a character from a table using an offset from a global word, and either stores the character into a buffer or performs string comparisons and pointer adjustments based on the character value.
- `copy_and_interleave_data`: Copies data from a source table into a new location while interleaving the word values into a different memory structure.
- `copy_data_block`: Copies a block of data from a source table to a destination global variable using a loop.
- `copy_memory_range`: Copies a block of memory of a fixed size from a global source to a destination.
- `initialize_mission_data`: Copies 5120 bytes from a source to a destination, calls another routine four times with different parameters, and then performs a loop that copies words from a table into a destination structure at specific offsets.
- `interleave_table_data`: Copies data from a source table into a new memory structure by interleaving word values and decrementing a global counter.
- `process_byte_pair`: Uses two bytes from a pointer to calculate two different table offsets and updates global parameters.
- `load_level_data`: Compares a global variable against four constants and, if a match is found, stores a specific address and a constant into global variables before calling another routine.
- `transform_data_block`: Performs a series of bitwise inversions and byte swaps on a data block before copying a large range of memory.
- `initialize_music_parameters`: Sets a global variable to zero, assigns its address to two other globals, and calls a routine with a specific constant value.
- `reset_state_variables`: Calls two routines to reset system state and then toggles a global variable between zero and one.
- `initialize_music_parameters_alt`: Sets a global variable to zero, assigns its address to two other globals, and calls a routine with a different constant value.
- `initialize_music_system`: Walks a byte table to build a pointer table for music samples, clears several global variables, and resets various counters and values.
- `reset_hardware_registers`: Writes a sequence of values to several hardware control and volume registers.
- `cycle_audio_parameter_updates`: Increments a counter in a global variable and, if it reaches a limit, resets it and calls another routine four times to update audio properties for four different channels by reading values from a table and writing to hardware registers.
- `update_audio_parameter_structure`: Reads a command and index from a structure to look up values in a table, then updates the structure with new audio properties, including volume, pitch, and hardware register settings.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `update_structure_value_by_command`: Reads a command byte from offset 2 of a structure in a6 and performs different arithmetic or assignment operations on a 16-bit value at offset 16 based on that command.
- `stores_address_to_vector`: Stores the address of the next instruction into a global pointer at offset 112.
- `execute_recursive_audio_updates`: Compares a global word to zero and, if non-zero, calls itself twice before calling routines to update hardware audio channels and hardware registers.
- `configure_audio_channels`: Uses a value from a table to select one of four audio configurations, then updates the hardware audio registers with a specific channel number, period, and volume, and stores the resulting frequency and sample data into a set of global variables.
- `update_hardware_configuration`: Checks four word-sized values at specific offsets in a structure; if a value is one, it writes specific words to the INTENA and ANTIC registers, then clears all four values in the structure and returns.

### Tornado Ground Attack (1991)(Energize)[cr FLT][a].adf:echo

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of data register 1 to data register 0 and then performs a short branch.

### Tornado Ground Attack (1991)(Energize)[cr FLT][a].adf:FLT

- `process_recursive_bit_shifts_2`: Copies a block of memory, then enters a loop that performs bitwise shifts and calls other routines to update a value in a global variable based on the results.
- `store_shifted_long_word`: Loads a long word from a pointer in a0, extracts the lowest byte, and stores it as a word into a memory location pointed to by a5.
- `process_recursive_bit_shifts`: Performs a series of recursive calls based on the number of right shifts required to clear a value in d0, updating d2 through a loop and calling another routine.

### Tornado Ground Attack (1991)(Energize)[cr FLT][a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_function_via_pointer`: Loads a pointer from an offset of 18 of a structure in a1, loads another pointer from an offset of 18 of a table at a0, and calls the second pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_and_jump`: Clears registers d2 and d3, loads a constant into a6, calls a routine at an offset of negative 222 from a6, and then jumps to a fixed address.
