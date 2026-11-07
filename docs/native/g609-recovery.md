# Game 609 (Puzznic v1.1 0041): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00609). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (872456 top-level bytes in 29 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 704 raw-word lines |
| Functions named and described | pass, 281 of 281 named (0 author symbols), 281 of 281 described (13 by rule, 262 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4458 of 4458 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 153605 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7005 of 7490 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 200044 |
| code | 33630 |
| data | 638691 |
| padding | 91 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 178 | 15 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5993 | 201 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1298 | 52 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 5 |
| MUZ0 (raw code file, load address unknown) | 1 | 8 | 1 |
| puznic.cod (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- Puzznic_v1.1_0041/Puzznic_v1.1_0041.hdf: hard-disk image (AmigaDOS OFS, 928 blocks): its filesystem parses and all 23 files in it are byte-identical to the extracted tree Puzznic_v1.1_0041_hdf, which is recovered file by file; no block outside the filesystem holds data
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3388 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 892 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Puzznic_v1.1_0041/Puzznic_v1.1_0041_hdf/Puzznic/data/MUZ0: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Puzznic_v1.1_0041/Puzznic_v1.1_0041_hdf/Puzznic/data/puznic.cod: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 2 never-loaded region(s), 153605 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `configure_dma_registers` (runtime segment an address): Writes specific values to custom hardware registers to configure DMA settings. Kind: disk hardware/track validation. Pass path: the instructions immediately following the call (e.g., the bit manipulation on d0).
- `assign_constant_value` (runtime segment an address): Stores a constant value into a global variable after performing a recursive call to itself. Kind: checksum or hardware validation trap. Pass path: the execution continues normally if the constant is correctly assigned/validated.
- `set_disk_length` (runtime segment an address): Sets a global variable to a specific value and modifies the disk length hardware register. Kind: disk geometry/length check. Pass path: execution continues to next instruction.
- `modify_global_bits_2` (runtime segment an address): Performs bitwise manipulation on a global variable using a loop that adjusts bits based on the initial value. Kind: track/sector synchronization check. Pass path: the branch where uVar1 < uVar2 (the first do-while loop) or the final u3__sub_1A9E call if the loop is skipped.
- `modify_global_bits` (runtime segment an address): Performs bitwise operations on a global variable within a loop that continues until a register-based counter reaches a threshold. Kind: checksum/integrity trap. Pass path: the loop completes normally and returns 0.
- `modify_joystick_register` (runtime segment an address): Updates a global variable by performing bitwise operations on an input byte and a calculated bit position. Kind: joystick/input-based manual/code-wheel check. Pass path: the branch following the btst.b #an address, an address.l instruction (the bit test on the custom chip/joystick register).
- `modify_global_bitmask` (runtime segment an address): Calculates a bitmask by inverting a bit at a position determined by an offset and updates a global variable using conditional bitwise operations. Kind: bitmask-based integrity/hardware check. Pass path: the branch where d0 is non-zero (not taken by beq.w an address).
- `set_joystick_bit` (runtime segment an address): Sets a specific bit in a global variable using an input byte and a bitwise OR operation. Kind: manual/code-wheel/color-card lookup. Pass path: The routine checks a specific bit (likely from a joystick or custom hardware input) and sets a flag in memory. A successful check (the correct input being detected) allows the program to proceed to the next stage of loading or execution; fa.
- `clear_joystick_bit_and_loop` (runtime segment an address): Clears a specific bit in a global variable and executes a long decrementing loop. Kind: checksum/integrity trap. Pass path: the loop completes and execution continues to the next instruction after the call.

## Functions


### game.slave

- `return_constant_negative_one`: The routine returns the constant value -1.
- `update_memory_and_copy`: Modifies a table using a loop with incrementing or bit-rotated values, then copies data from one memory location to another and clears a block of memory with zeros.
- `clear_and_load_level_data`: The routine clears a 256-byte block in memory and then loads several 32-bit values from a table indexed by a base pointer, with an extra value loaded when a control byte equals 4.
- `return_concatenated_registers`: The routine calls another routine and then calls a function pointer taken from a table.
- `invoke_callback_and_return_concat`: The routine calls another routine through a pointer stored in a global variable and then returns the concatenation of two input values.
- `check_global_flag_and_set_byte`: The routine tests a global variable for two flag bits and conditionally writes a byte to it.
- `compare_strings_until_mismatch`: The routine calls another routine and then compares a string pointed to by a register with a null-terminated string in a global variable, advancing both pointers until a mismatch or terminator is found, then skips to the next line in the global string.
- `convert_string_to_lowercase`: Iterates through a string and subtracts a constant value from any character falling within a specific range.
- `decode_byte_flag`: The routine clears a global variable and then checks a byte from D0 against it.
- `wait_for_input`: The routine polls another routine until a specific condition on a global variable and a byte in a table is met, then waits for the global variable and the same byte to become set before calling another routine once.
- `wait_for_key_strobe`: The routine checks a global variable flag and, if set, clears a bit, calls another routine, then restores the bit.
- `check_vhposr_bit`: The routine reads the VHPOSR hardware register and tests bit 15 of its value.
- `set_global_timer`: The routine calls another routine with constant arguments and then sets a global variable to a small constant if a local integer remains zero, otherwise to a larger constant.
- `return_input_byte`: Returns the byte that was already in D0.
- `return_register_value`: This routine returns the current contents of a data register as an unsigned byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `infinite_loop_routine`: The routine contains an infinite loop with no operations.
- `clear_memory_region`: The routine clears a 256 byte memory region starting at a fixed address to zero and then reads a byte from an external location into a global variable.
- `reset_global_flag`: The routine writes a constant to a global variable and clears a second global variable, then calls another routine and returns.
- `clear_tile_table`: The routine clears a set of global variables and then iterates over a table of short entries.
- `reset_and_loop_call`: The routine clears a global variable, then conditionally resets it and calls another routine once, or otherwise calls another routine four times in a loop before setting the global variable to a constant.
- `unreachable_routine`: The routine contains no valid instructions and is never reached by the program.
- `self_recursive_loop`: The routine sets a local counter to zero and then repeatedly calls another routine four times, incrementing the counter on each iteration until it reaches four, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_and_globals`: Clears a memory range starting at a specific address, updates several global variables from a table, and modifies the interrupt enable bit in the INTENA register.
- `initialize_data_structures`: Clears several global variables and populates a table by iterating through a list of pointers and values.
- `toggle_interrupt_state`: Sets a global variable to a specific value and modifies the interrupt enable bit in the hardware register.
- `execute_four_times`: Calls another routine exactly four times using a loop counter.
- `update_pointer_data`: Checks a byte in a table, and if non-zero, updates a pointer and two values using data retrieved from a secondary table.
- `update_memory_and_dma`: Writes values to memory locations using offsets and modifies the DMA controller register.
- `reset_piece_state`: If a byte value is less than four, it copies one byte to another, clears two bytes, and fills a series of memory locations with specific patterns.
- `store_word_at_pointer`: Stores a 16-bit value into a memory location determined by adding two values together.
- `update_clamped_value`: Takes a byte and reduces it to a quarter of a global value when that is smaller.
- `decrement_and_update_state`: Returns early unless a flag byte is negative.
- `update_recursive_state_table`: Updates a series of data structures and pointers within a memory block based on a value retrieved from a table.
- `process_recursive_state_update`: Updates a series of linked data structures by copying values from a source pointer and performing conditional field assignments.
- `process_recursive_state_update_2`: Decrements a counter and, if non-zero, performs a series of pointer updates and data shifts before recursively calling itself.
- `process_recursive_state_update_3`: Updates a series of linked data structures by reading values from a pointer and performing recursive calls.
- `clear_memory_and_update_global`: Clears a byte at a memory location determined by an offset, updates a global variable from a memory location, and clears a word at a third memory location.
- `process_linked_list_node`: Pops a node from a linked list and performs a recursive update on state data.
- `process_recursive_tile_updates`: Recursively updates tile data and associated attributes within a grid structure based on a value retrieved from a pointer.
- `process_linked_list_node_2`: Traverses a linked list by popping a node from a stack and updating several state structures.
- `process_recursive_state_update_4`: Updates a series of data structures by shifting values between offsets and recursively calls itself.
- `unpack_and_update_tile_data`: Extracts bit-packed nibbles from a source buffer to populate a structured data block.
- `process_recursive_state_update_5`: Updates a series of data structures by reading values from a pointer, performing conditional swaps, and recursively calling itself.
- `update_entity_state_and_dispatch`: Updates a structure of data by copying values from a source buffer and performs conditional logic based on a byte value.
- `process_tile_transformation`: Updates tile data and state flags within a structure based on a value read from memory.
- `update_tile_and_dispatch_event`: Copies a block of data from a source into a structure and updates various state bytes based on a conditional flag.
- `process_recursive_state_update_6`: Updates a series of data structures by shifting values between memory offsets and performs a recursive call.
- `process_recursive_state_update_7`: Updates a series of linked data structures and state flags based on a byte value.
- `update_tile_state_and_recurse`: Updates tile properties and sprite data within a structure based on a value read from memory.
- `process_input_and_dispatch_event`: Reads a byte and a secondary value from a source, updates a state structure, and performs a table lookup to execute a function pointer.
- `update_piece_attributes`: Updates a piece's state and properties by copying values between memory offsets and performing lookups in a table.
- `clear_piece_state`: Copies one state byte to another, clears a second state byte and sets a third to seven.
- `initialize_system_hardware_and_state`: Configures hardware interrupts, DMA, and copper registers before clearing large blocks of memory and resetting numerous global state variables and bitmask tables to default values.
- `decompress_iff_data`: Scans for a specific tag in a data stream and decompresses subsequent chunks of data into a buffer.
- `scan_for_iff_body_chunk`: Scans forward word by word until it finds the four-character tag BODY that begins image data.
- `decompress_data_stream`: Reads a stream of bytes and expands them into a destination buffer based on length values encoded in the input.
- `load_and_decompress_data`: Writes several constant values to a global variable and a memory location, calls a decompression routine, and then clears a block of memory.
- `setup_graphics_tables`: Copies a large block of data to a destination, sets several global variables, and populates a table by shifting and masking bits from another source.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d1.
- `check_and_clear_pointer`: Checks if a global variable is zero, and if so, stores a word into two different locations and calls another routine.
- `check_tile_table_status`: Checks if a global variable is zero and if another routine's address is non-zero, then calls a routine to clear a tile table.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `toggle_and_loop_state`: Checks a global variable against a specific character, then updates that variable and a secondary global value before entering a loop that executes a routine multiple times.
- `clear_large_buffers`: Zeros out two large, separate memory regions by iterating through them in blocks.
- `toggle_state_and_clear_buffers`: Checks a global variable against a specific character and, if matched, updates a global variable, resets a flag, and calls several routines to clear or modify memory buffers.
- `clear_alternate_buffers`: Zeros out two different large memory regions by iterating through them in blocks.
- `reset_state_and_update_buffers`: Sets a global variable to zero and, if certain conditions are met, calls routines to clear large memory regions and initialize tables.
- `toggle_global_flag`: Checks several global conditions and then performs a bitwise NOT operation on a global variable before potentially updating it with a new value.
- `modify_global_flags`: Checks if a global variable is zero and another value is less than three.
- `reset_game_engine_state`: Resets a large number of global variables, bitmask tables, and collision data to default or zero values.
- `process_data_list`: Iterates through a table of pointers, checks a flag in each entry, and if set, updates a global variable and executes a routine using data from a calculated table offset.
- `adjust_bcd_values`: Reads four bytes from a table and a global array, performs a BCD adjustment on each, and stores the results back into the original locations.
- `synchronize_game_state_and_buffers`: Checks specific global flags and state variables to trigger a sequence of data synchronizations, including resetting bitmask tables, updating collision data, and performing multiple structured memory copies between various attribute and graphics buffers.
- `clear_memory_blocks`: Iterates through two memory regions, clearing specific offsets and the start of each block to zero.
- `clear_data_register`: Performs a logical OR operation on a byte in a data register with zero.
- `execute_recursive_state_logic`: Checks multiple global conditions and, if met, enters a loop that performs recursive calls and eventually enters an infinite loop.
- `execute_recursive_calls`: This routine performs two consecutive calls to itself before returning.
- `conditionally_clear_memory`: Checks a global flag and, if set, clears specific offsets within two memory regions before clearing the flag.
- `process_recursive_state_2`: Updates a global variable using a bitwise AND with a constant and enters a loop that performs recursive calls to itself if the result is non-zero.
- `copy_data_table`: Compares a global value against a stored value and, if they differ, copies a sequence of data from a table into a destination memory area.
- `update_data_buffer`: Compares a global value against a stored value and, if they differ, copies sixteen words from a table into a destination memory area using an offset derived from a global variable.
- `get_character_byte`: Checks a global flag and returns a byte from a table, incrementing the table pointer when a specific condition is met.
- `reset_game_engine_state_2`: Clears a large number of global variables, bitmask tables, and state counters to zero or default values.
- `copy_and_shift_buffers`: Copies data from one memory location to several large buffers using an offset-based loop, then performs a bitwise left shift on values from another buffer into a third.
- `initialize_game_state_tables`: Resets a large number of global variables and bitmask tables to zero or default values.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 1 into d0.
- `reset_game_state_and_bitmasks`: Resets a large number of global variables and bitmask tables to zero or default values.
- `call_buffer_bitmask_processor`: This routine calls another routine twice before returning.
- `clear_data_register_2`: Performs a logical OR operation on a data register with zero.
- `initialize_and_process_state`: Resets several global variables, performs table lookups to update memory, and enters a loop that calls various other routines.
- `reset_game_state_2`: Clears multiple global variables and bitmask tables, then enters a loop that repeatedly calls various state update and memory management routines.
- `apply_bitmask_loop`: Iterates through a list of structures, calculates a destination address based on a multiplier and a table offset, and applies a series of bitwise AND and OR operations using a fixed set of values.
- `call_other_routine`: Calls another routine three times.
- `process_recursive_state`: Decrements a global variable and performs a recursive call if a condition is met, then updates a global state and writes a value to a memory location.
- `process_recursive_state_updates`: This routine implements a recursive state machine that updates a data structure based on bitmask flags.
- `process_recursive_state_updates_2`: Updates a data structure by iterating through bitmask flags to perform conditional increments, decrements, or recursive calls.
- `resolve_pointer_and_increment`: Retrieves a value from a table using an index, performs pointer arithmetic, and increments a global byte by one multiple times.
- `load_sequence_of_constants`: Performs a series of logical OR operations on a data register using a sequence of eight different byte constants.
- `apply_bitmask_to_buffers_2`: Inverts a global byte and, if zero, performs bitwise AND and OR operations between a source table and a destination buffer using a loop.
- `clear_data_register_3`: Performs a logical OR operation on a data register with zero.
- `apply_bitmask_to_buffers_dual`: Performs bitwise AND and OR operations between a source table and two separate destination buffers based on a global value.
- `execute_sequential_subroutines`: calls two consecutive subroutines in sequence before returning to the caller.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `copy_and_transform_data`: Selects a source memory address based on a global variable and copies data into a destination buffer using a loop.
- `copy_memory_block`: Copies sixteen bytes from a global variable to a fixed memory destination.
- `transfer_and_transform_buffers`: Iterates through a large loop to copy data into multiple buffers at specific intervals and applies a bitwise left shift to a set of values.
- `duplicate_table_entries`: Iterates through a table and copies each entry's value to a later offset within the same structure.
- `clear_buffer_on_match`: Iterates through a table to find matching values and clears specific memory buffers using non-contiguous offsets.
- `copy_animated_tile_frame`: Advances a two-phase toggle counter, then copies one of two source tile-data blocks, selected by the toggle state, into a destination buffer as eight strided longword groups over a count of sixteen or eight.
- `clear_data_register_4`: Performs a logical OR operation on a data register with zero to ensure its value is cleared.
- `copy_data_blocks`: Updates a destination buffer by copying eight blocks of data from one of two possible source tables.
- `copy_data_blocks_3`: Copies eight blocks of data from one of two source tables to a destination buffer using non-contiguous memory offsets.
- `copy_data_blocks_offset`: Copies eight blocks of data from one of two source tables to a destination buffer starting at a specific offset.
- `invoke_scattered_data_copy`: This routine calls a procedure that transfers data from a source to a destination using non-contiguous memory offsets.
- `copy_scattered_data_block`: Copies data from a source to a destination using a loop with specific non-contiguous memory offsets.
- `reorder_buffer_elements`: Copies specific non-contiguous longwords from a source buffer into a destination buffer in a new sequence.
- `copy_scattered_data_blocks`: Copies eight specific longwords from a source buffer to a destination buffer using fixed, non-contiguous offsets.
- `reverse_scattered_copy`: Copies data from a source buffer to a destination buffer using a loop with specific non-contiguous memory offsets.
- `call_reverse_and_apply`: Calls another routine and then performs a reverse scattered copy from a source buffer to a destination buffer.
- `reorder_buffer_elements_2`: Copies specific non-contiguous longword elements from a source table into a destination buffer in a predefined sequence.
- `invoke_buffer_bitmask_application`: Calls a routine that performs bitwise operations on multiple memory buffers using a list of offsets and a control byte.
- `scatter_copy_data_offset`: Copies data from a source buffer to a destination buffer using a loop with specific non-contiguous memory offsets starting from a different source address.
- `scatter_copy_data_fixed`: Copies data from a source buffer to a destination buffer using a loop with specific non-contiguous memory offsets starting from a fixed source address.
- `set_global_values`: Stores a constant into a global variable and another global variable before jumping to a vector.
- `copy_data_block_with_offset`: Increments a value at a memory location pointed to by a register and performs a repetitive block copy from a source table to two destination buffers using interleaved offsets.
- `copy_multiple_buffers`: Copies data from a source to two separate destination buffers using a loop with non-contiguous offsets.
- `apply_bitmask_to_buffers`: Iterates through a list of offsets to perform bitwise AND and OR operations on several large memory buffers based on a control byte.
- `synchronize_game_state_tables`: Resets and updates multiple data tables by clearing existing values and copying new data from a source table into staggered destination offsets.
- `synchronize_data_tables`: Clears two destination tables and then populates them by copying 32-bit words and 16-bit words from a source table at specific offsets and intervals.
- `manage_state_transition_counters`: Monitors and decrements a series of global counters and function pointers to manage state transitions.
- `merge_code_buffers`: Performs a bitwise OR between a sequence of instructions and two separate memory buffers.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 0 into (a0)+, then merges 0 into d0.
- `clear_memory_buffers`: Sets a large number of bytes to zero across multiple memory buffers using a loop.
- `update_game_state_and_tables`: Updates BCD-encoded values and synchronizes multiple data tables and bitmask buffers.
- `decrement_and_test_counter`: Decrements a global byte value and checks if the result is zero.
- `update_indexed_memory_tables`: Applies bitwise masks to multiple memory buffers and writes bytes to various locations within indexed tables.
- `write_to_indexed_tables`: Writes a byte to various locations in memory calculated using an index and several pointers.
- `call_external_routine`: Calls another routine after performing an initial operation.
- `zero_memory_buffers`: Fills multiple large memory buffers with zeros using a loop.
- `copy_instruction_sequence`: Copies a sequence of 39 words from a code location into a destination buffer selected by a global variable.
- `copy_code_to_buffer`: Copies a sequence of data from a code location into a destination buffer using non-contiguous offsets.
- `copy_structured_data_block`: Copies a series of non-contiguous 32-bit words from a source table to a destination buffer using a loop.
- `copy_memory_block_2`: Copies a block of data from a source address to a destination address using a specified length.
- `invoke_buffer_bitmask_application_2`: Calls a routine that performs bitwise operations on multiple memory buffers using a list of offsets and a control byte.
- `update_global_pointer_chain`: Sequentially updates a global variable by loading values from a series of memory addresses.
- `update_global_pointer`: Loads a value from a table into a global variable and conditionally clears another global variable based on a pointer check.
- `validate_pointer_and_branch`: Checks a global variable and evaluates whether a value at a pointer is zero to determine the execution path of subsequent routines.
- `initialize_game_state_tables_2`: Clears two destination tables with zeros and then populates them by copying 32-bit and 16-bit words from a source table using specific offsets and strides.
- `tiny_ori_3`: Small routine of 1 instruction that merges 1 into d0.
- `tiny_ori_4`: Small routine of 1 instruction that merges 0 into d3.
- `copy_attribute_data_tables`: Copies blocks of data from three different source tables into a destination buffer using structured offsets.
- `reconstruct_attribute_data_tables`: This routine performs a series of structured data copies from multiple source tables into specific destination memory regions.
- `fetch_and_call_buffer_mask_routine`: Retrieves a value from a table using an index stored in a global variable and passes it as an argument to another routine.
- `reset_game_state_and_tables`: Resets multiple global variables, clears bitmask tables, and initializes several memory buffers with default values.
- `reset_game_state`: Resets a large number of global variables, pointers, and bitmask tables to their default or initial values.
- `reset_and_initialize_game_state`: Resets a large number of global variables, bitmask tables, and state counters to their default values.
- `reconstruct_data_structure`: Copies specific non-contiguous elements from a source table into a new destination block.
- `copy_structured_data_reverse`: Copies a series of non-contiguous 32-bit words from a source buffer into a destination table using a loop.
- `execute_recursive_calls_3`: This routine performs two consecutive calls to itself before returning.
- `set_global_values_2`: Stores a constant value into a global variable and another constant into a different global variable before jumping to a vector.
- `set_global_values_3`: Assigns specific constant values to two global variables and then performs a jump to a vector.
- `initialize_constant_data`: Sets a global variable to a specific value and writes a four-byte constant to a memory location before jumping to a new entry point.
- `load_string_table`: Sequentially stores a series of constant values into a global variable and another global variable before jumping to a vector for each pair.
- `update_state_pointer`: Manages a state machine by updating a global variable and several pointers based on values found in a table.
- `update_state_pointers`: Manages state transitions by reading offsets from a table to update several pointers and global variables based on bitwise flags.
- `iterate_function_table`: Iterates through a list of pointers in a table, calling each routine until a null pointer is encountered.
- `perform_no_operation`: This routine performs a logical OR operation on a data register with zero.
- `process_data_table`: Iterates through a table of short integers, calculates an offset based on the value, and executes a function pointer from a secondary table.
- `copy_data_block`: Copies a sequence of short integers from one memory location to another until a terminator value is reached.
- `copy_data_block_2`: Transfers a series of short integers from a source table to a destination table, ending when a specific value is encountered.
- `scan_table_write_grid_values`: Walks a sentinel-terminated table of fixed-size records: a negative entry ends the scan, a zero entry only skips a record and decrements a running counter, and any other entry uses its high nibbles as row and column offsets, scaled by a global stride, to store the current counter into a grid array.
- `decrement_table_values`: Uses values from a table to calculate memory addresses and writes a sequence of decreasing values into a destination table.
- `clear_data_register_5`: Performs a logical OR operation on a data register with zero to ensure it contains no set bits.
- `update_sprite_attribute_table`: Iterates through a data structure to update a memory buffer by performing bitwise shifts and OR operations on values.
- `tiny_ori_ori_3`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `validate_and_move_tile`: Checks if a specific tile position in a grid is valid and unoccupied before updating the grid state.
- `clear_data_register_byte`: Performs a logical OR operation on a byte in a data register using a zero value.
- `clear_data_register_byte_2`: Performs a logical OR operation on a data register with zero to clear its contents.
- `copy_memory_block_4`: Copies a block of data from a source to one of two destination addresses determined by a conditional check on a global variable.
- `select_destination_address`: Determines a destination address based on the value of a global variable and returns it.
- `clear_d0_register`: Performs a logical OR operation on the data register zero with zero to clear its contents.
- `copy_interleaved_data`: Copies data from a global variable to a destination address determined by a conditional check on a global variable using an interleaved memory access pattern.
- `copy_calculated_block`: Calculates a source address based on a global variable and a table index, then copies data from that source to a global variable using an interleaved memory access pattern.
- `update_bitmask_table`: Performs bitwise operations on data from a source table to update a destination table based on a bit shift amount.
- `apply_bitmask_to_buffer`: Updates a target buffer by performing a bitwise OR operation with the inverse of a combined bitmask derived from a source data block.
- `apply_bitmask_to_buffer_2`: Performs bitwise operations on a source data block by shifting its values and applying a combined bitmask to update a target buffer.
- `duplicate_memory_block`: Copies a block of data from a calculated source address into two separate destination locations using an interleaved memory access pattern.
- `initialize_level_data_structures`: Calculates memory offsets for level-specific data based on global parameters and populates a destination buffer with values derived from a source table.
- `build_tile_grid_from_script`: Walks a sequence of signed control words, and for each non-negative entry it writes the matching tile-type code into a per-cell array and fills a set of grid cells with preset table values, advancing the cell and array pointers as it goes.
- `initialize_level_grid_data`: Iterates through a control table to populate a grid structure with specific tile types and associated metadata.
- `tiny_ori_rts`: Small routine of 2 instructions that merges 0 into d0, then returns.
- `increment_and_clear_buffer`: Increments a global word and stores a zero byte at an offset from a pointer.
- `increment_and_clear_pointer`: Increments a global variable and stores a zero byte at an offset from a pointer.
- `increment_global_and_reset_offset`: Increments a global 16-bit variable and clears a byte at a specific offset relative to a pointer held in a register.
- `increment_and_clear_byte`: Increments a global variable and clears a byte at an offset from a pointer held in a register.
- `increment_and_reset_global_state`: Increments a global 16-bit integer and clears a byte at an offset from a pointer held in a register.
- `increment_and_clear_byte_2`: Increments a global variable and stores a zero byte at an offset from a value held in a register.
- `increment_global_and_clear_byte`: Increments a global word variable and clears a single byte at an offset from a pointer held in a register.
- `increment_and_clear_byte_3`: Increments a global variable and stores a zero byte at an offset from a value held in a register.
- `merge_bitmask_tables`: Performs a bitwise OR operation between values from a source table and two destination tables using specific offsets in a loop.
- `distribute_table_data`: Copies data from a source table to two different destination tables using staggered offsets and specific word intervals.
- `copy_data_blocks_2`: Copies 32-bit words from a source table to two different destination tables at various staggered offsets.
- `clear_memory_tables`: Fills two separate destination tables with zeros using a loop.
- `stride_copy_tables`: Copies specific 16-bit words from a source table to two destination tables at large intervals.
- `update_sprite_data_buffers`: Checks a table for changes and, if detected, copies data from a source table into two distinct memory buffers using indexed offsets.
- `sync_tile_graphics_buffers`: Compares a table of values against a secondary buffer and, if changes are detected, copies data from a source table into two distinct memory regions using structured offsets.
- `update_sprite_attribute_tables`: Compares a table of values against a secondary buffer to detect changes, then performs a bulk memory copy of sprite attribute data into two distinct memory regions.
- `sync_and_update_graphics_buffers`: Checks a table for changes and, if detected, synchronizes data between two sets of memory buffers by copying values from a source table into specific offsets of destination memory regions.
- `update_tile_graphics_buffers`: Compares a small table of values against a secondary buffer to detect changes, then performs a bulk memory copy of tile data into two distinct destination memory regions.
- `sync_sprite_data_tables`: Compares a local table of values against a shadow copy and, if changes are detected, performs a bulk memory copy to synchronize two sets of sprite or tile data buffers.
- `update_tile_graphics_buffers_2`: Checks a table for changes in tile indices and, if changes are detected, copies corresponding graphic data from a source table into two separate destination memory buffers.
- `update_tile_graphics_buffers_3`: Compares a table of values against a secondary buffer to detect changes, then performs bulk memory copies to update two distinct sets of graphics data buffers based on the modified entries.
- `sync_sprite_attribute_tables`: Compares a local table of values against a shadow copy to detect changes.
- `tiny_ori_5`: Small routine of 1 instruction that merges 158 into d2.
- `tiny_ori_6`: Small routine of 1 instruction that merges 228 into d2.
- `update_tile_palette_data`: Performs bitwise OR operations between two memory tables and stores the results in a third destination table.
- `initialize_level_data_and_copy_buffer`: Checks several global flags to determine if a level reset is required, then updates pointers and counters to load new data from a structure.
- `check_object_conditions`: Checks if a short value at a specific offset is neither negative two nor negative one, and if a byte at another offset is zero, before calling another routine.
- `update_object_state_via_table`: Calculates an index from a structure field to access a data table, then performs conditional updates to multiple object properties, including function pointers and status bytes, if specific value matches are met.
- `check_pointer_values`: Compares values at offsets within a structure pointed to by a register against specific constants and clears a global variable and a pointer if a condition is met.
- `update_state_and_counters`: Calculates an index based on values in a structure to look up data in a table, then modifies a byte in the structure and increments a counter.
- `modify_byte_values`: Sets a bit in a byte through a pointer and calculates a new value to store in a global variable.
- `process_table_data`: Iterates through a table using a calculated index to copy values to a global array and performs conditional updates to a structure and a counter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_global_state`: Performs bitwise manipulations on a global variable based on its current value.
- `clear_memory_blocks_2`: Fills two separate memory regions with zeros using a loop that runs for a fixed number of iterations.
- `copy_data_block_4`: Copies data from a source table to two separate destination memory buffers using nested loops.
- `clear_memory_block`: Initializes a block of memory by filling 8000 consecutive words with zero.
- `update_bitmask_tables`: Performs bitwise operations on values retrieved from a global table using calculated offsets and updates multiple memory locations via a base pointer.
- `update_collision_bitmasks_2`: Calculates new bitmask values by performing bitwise OR, NOT, and shift operations on elements retrieved from a table using calculated offsets.
- `update_collision_bitmasks_3`: Calculates new bitmask values by performing bitwise OR and NOT operations on elements from a table and applies the results to multiple memory locations via a pointer in a register.
- `update_bitmask_tables_2`: Performs bitwise logical operations on several memory blocks using values derived from a global variable.
- `update_collision_bitmasks`: Calculates new bitmask values by performing bitwise OR and NOT operations on elements from a table and applies the results to multiple memory locations using bitwise AND and OR operations.
- `execute_infinite_loop`: The routine enters an infinite loop that prevents further execution.
- `update_blitter_pointers`: Updates several global values and sets specific memory addresses based on the state of a global variable.
- `process_cia_interrupt`: Checks specific bits in a global variable and performs bitwise operations and a function call if certain conditions are met.
- `increment_and_call`: Increments a global variable and calls two other routines if a specific bit is set in a global variable.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `execute_iterative_subroutine_sequence`: Repeatedly calls a specific routine in a fixed sequence while managing a global counter.
- `process_data_blocks`: Decrements a global counter and performs a loop that copies data blocks between memory locations while checking a table for a termination value.
- `do_nothing`: This routine is empty and performs no operations before returning.
- `copy_memory_block_3`: Decrements a global counter and copies a block of data from one memory location to another using a loop.
- `compare_table_sum`: Iterates through a table of integers to calculate a sum and compares the result against a global variable.
- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.
- `copy_conditional_words`: Iterates through a memory range to copy words from a source to a destination if the source value is non-zero, repeating until a global value reaches zero.
- `store_constant_value`: Stores the value zeroxffff at the memory location pointed to by an input pointer.
- `compare_case_insensitive_strings`: Compares sequences of bytes by normalizing them to uppercase and checks for equality.
- `compare_case_insensitive`: Compares two sequences of bytes by treating them as case-insensitive characters and returns a status based on the match.
- `return_negative_one`: Returns the value negative one in the primary data register.
- `load_from_pointer`: Loads a word from the memory location pointed to by an input pointer into a global variable.
- `populate_data_table`: Checks a global variable and, if zero, iterates through a table to copy values and append specific bytes before setting the global variable to one.
- `set_global_one`: Sets a global variable to the value one.
- `copy_non_zero_words`: Iterates through a source table and copies non-zero words to a destination table, terminating with a negative one.
- `copy_fixed_length_words`: Checks if a global variable is zero and calls another routine; if successful, it copies a fixed number of words from one location to another.
- `update_conditional_global_value`: Updates a global variable by calling another routine and conditionally overwriting it with a second routine's return value if the first result is non-negative and the second result is non-zero.
- `calculate_modulo_and_range`: Performs division and modulo operations on an input value and adjusts the result based on a comparison.
- `configure_dma_registers`: Writes specific values to custom hardware registers to configure DMA settings.
- `wait_for_flag`: Enters a loop that checks a bit in a global variable and performs hardware register writes until a condition is met or a counter expires.
- `assign_constant_value`: Stores a constant value into a global variable after performing a recursive call to itself.
- `execute_conditional_loop`: Executes a loop that calls another routine and checks a bit in a global variable.
- `set_disk_length`: Sets a global variable to a specific value and modifies the disk length hardware register.
- `execute_recursive_calls_2`: The routine performs two consecutive calls to itself before returning.
- `search_table_entry`: Iterates through a table of 16-bit values until a match for a global variable is found.
- `interleave_bit_patterns`: Iterates through two memory buffers to create an interleaved bit pattern in a destination buffer.
- `process_bit_interleaving`: Reads a list of pointers and performs bitwise manipulation to interleave bits from two sources into a destination table.
- `search_and_store_markers`: Searches a memory range for a specific word pattern and stores the resulting pointers into a table.
- `modify_global_bits_2`: Performs bitwise manipulation on a global variable using a loop that adjusts bits based on the initial value.
- `modify_global_bits`: Performs bitwise operations on a global variable within a loop that continues until a register-based counter reaches a threshold.
- `return_negative_one_2`: Returns a value of negative one.
- `poll_joystick_buttons`: Reads the status of the joystick and fire buttons from the CIAA Port A register.
- `decrement_loop_counter`: Decrements a value in a register until it reaches zero.
- `modify_joystick_register`: Updates a global variable by performing bitwise operations on an input byte and a calculated bit position.
- `modify_global_bitmask`: Calculates a bitmask by inverting a bit at a position determined by an offset and updates a global variable using conditional bitwise operations.
- `set_joystick_bit`: Sets a specific bit in a global variable using an input byte and a bitwise OR operation.
- `clear_joystick_bit_and_loop`: Clears a specific bit in a global variable and executes a long decrementing loop.
- `return_immediately_2`: This routine is empty and performs no operations before returning to its caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_no_op`: The routine consists of a single instruction that returns immediately, performing no work.
- `return_immediately`: This routine is empty and performs no operations before returning to the caller.
- `return_immediately_stub_2`: The routine consists of a single return instruction and performs no work, leaving no register or memory changes.
- `return_from_subroutine`: The routine is empty and performs no operations before returning.
- `return_immediately_stub`: The routine consists of a single instruction that returns to its caller, performing no work, no register or memory writes, and no library or hardware access.

### MUZ0 (raw code file, load address unknown)

- `execute_unknown_instructions`: This routine consists of eight instructions at the start of a raw code region.
