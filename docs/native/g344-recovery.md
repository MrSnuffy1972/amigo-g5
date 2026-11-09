# Game 344 (hellraisers (1988)(exocet software)[cr mwb]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00344). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 96 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 19 of 19 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 361 of 361 named (0 author symbols), 361 of 361 described (13 by rule, 347 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4094 of 4094 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 69148 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 13557 of 13557 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 510256 |
| code | 59650 |
| data | 223040 |
| padding | 25806 |
| unused | 82432 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| type | 2 | 117 | 2 |
| a1.prg | 3 | 13099 | 346 |
| mouse | 2 | 4 | 1 |
| border | 2 | 209 | 8 |
| Hellraisers (1988)(Exocet Software)[cr MWB].adf:copy | 2 | 117 | 2 |
| Hellraisers (1988)(Exocet Software)[cr MWB].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Hellraisers (1988)(Exocet Software)[cr MWB].adf:BIGPANEL.DAT (raw code file, load address unknown) | 1 | 0 | 0 |
| Hellraisers (1988)(Exocet Software)[cr MWB].adf:PMAP3.MAP (raw code file, load address unknown) | 1 | 0 | 0 |
| Hellraisers (1988)(Exocet Software)[cr MWB].adf:REDMAP.DAT (raw code file, load address unknown) | 1 | 0 | 0 |
| Hellraisers (1988)(Exocet Software)[cr MWB].adf:HITXT.TXT (raw code file, load address unknown) | 1 | 0 | 0 |
| Hellraisers (1988)(Exocet Software)[cr MWB].adf:PMAP2.MAP (raw code file, load address unknown) | 1 | 0 | 0 |
| Hellraisers (1988)(Exocet Software)[cr MWB].adf:NEWMAP.DAT (raw code file, load address unknown) | 1 | 0 | 0 |
| Hellraisers (1988)(Exocet Software)[cr MWB].adf:HILIGHT.DAT (raw code file, load address unknown) | 1 | 0 | 0 |
| Hellraisers (1988)(Exocet Software)[cr MWB].adf:BIGEX.SPR (raw code file, load address unknown) | 1 | 0 | 0 |
| Hellraisers (1988)(Exocet Software)[cr MWB].adf:LOGO.DAT (raw code file, load address unknown) | 1 | 0 | 0 |
| Hellraisers (1988)(Exocet Software)[cr MWB].adf:BIOMAP.DAT (raw code file, load address unknown) | 1 | 0 | 0 |
| Hellraisers (1988)(Exocet Software)[cr MWB].adf:STRIP.DAT (raw code file, load address unknown) | 1 | 0 | 0 |
| Hellraisers (1988)(Exocet Software)[cr MWB].adf:PMAP1.MAP (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- hellraisers (1988)(exocet software)[cr mwb]/Hellraisers (1988)(Exocet Software)[cr MWB].adf:BIGPANEL.DAT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- hellraisers (1988)(exocet software)[cr mwb]/Hellraisers (1988)(Exocet Software)[cr MWB].adf:PMAP3.MAP: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- hellraisers (1988)(exocet software)[cr mwb]/Hellraisers (1988)(Exocet Software)[cr MWB].adf:REDMAP.DAT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- hellraisers (1988)(exocet software)[cr mwb]/Hellraisers (1988)(Exocet Software)[cr MWB].adf:HITXT.TXT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- hellraisers (1988)(exocet software)[cr mwb]/Hellraisers (1988)(Exocet Software)[cr MWB].adf:PMAP2.MAP: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- hellraisers (1988)(exocet software)[cr mwb]/Hellraisers (1988)(Exocet Software)[cr MWB].adf:NEWMAP.DAT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- hellraisers (1988)(exocet software)[cr mwb]/Hellraisers (1988)(Exocet Software)[cr MWB].adf:HILIGHT.DAT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- hellraisers (1988)(exocet software)[cr mwb]/Hellraisers (1988)(Exocet Software)[cr MWB].adf:BIGEX.SPR: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- hellraisers (1988)(exocet software)[cr mwb]/Hellraisers (1988)(Exocet Software)[cr MWB].adf:LOGO.DAT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- hellraisers (1988)(exocet software)[cr mwb]/Hellraisers (1988)(Exocet Software)[cr MWB].adf:BIOMAP.DAT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- hellraisers (1988)(exocet software)[cr mwb]/Hellraisers (1988)(Exocet Software)[cr MWB].adf:STRIP.DAT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- hellraisers (1988)(exocet software)[cr mwb]/Hellraisers (1988)(Exocet Software)[cr MWB].adf:PMAP1.MAP: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 12 never-loaded region(s), 69148 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of one register to another and then performs a relative branch.

### a1.prg

- `initialize_system_state`: Calls several routines and then stores two byte values into specific global memory locations before branching to another location.
- `load_audio_parameters`: Loads several values into a global variable and another memory location before calling another routine multiple times.
- `handle_interrupt_and_update_state`: Performs a recursive call, stores a global variable into a memory location, and then sets a global variable to a specific constant value while enabling interrupts via a hardware register.
- `perform_recursive_call`: Checks a global variable and performs a recursive call to itself before returning a concatenated 64-bit value from two data registers.
- `iterate_and_decrement_global_counter`: Iterates backwards through a table, calling another routine for each element based on a comparison.
- `update_audio_dma`: Copies data from a source table to a destination, updates audio hardware registers, and configures DMA channels.
- `modify_linked_list`: Manipulates elements within a data structure by updating pointers and bitmask flags based on specific byte values.
- `set_audio_registers`: Writes several values to specific memory locations and modifies a global variable using bitwise operations.
- `update_audio_registers`: Iterates through a loop to update audio hardware registers using values from a table.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `fill_zero_data`: This routine is empty as it consists of a series of instructions that perform no functional operation.
- `clear_audio_buffers`: Sets several memory locations and a global variable to zero.
- `reset_global_state`: Calls another routine and then sets a global variable to zero twice.
- `return_concatenated_registers`: Returns a 64-bit value formed by concatenating the contents of two 32-bit data registers.
- `update_audio_hardware_parameters`: Processes a table of audio data to update hardware registers and DMA settings.
- `update_audio_hardware`: Processes a table to update audio hardware registers and DMA settings, involving element shifting, arithmetic calculations, and data copying.
- `reset_memory_values`: Clears four memory locations, a global variable, and two additional memory locations to zero.
- `clear_global_variables`: Sets a global variable to zero twice.
- `shift_table_elements`: Iterates through a table, shifting elements based on a bitmask and appending a byte to the end.
- `calculate_difference`: Performs arithmetic operations on values retrieved from a table and stores the result in a global variable.
- `copy_table_data`: Copies several bytes from a table into a structure based on an index and conditionally updates a global variable.
- `configure_audio_parameters`: Loads values from a table into memory and updates audio hardware registers using calculated offsets.
- `copy_word_to_global`: Reads a word from a memory location calculated using a pointer and an offset, then stores it into a global variable.
- `update_dma_and_audio`: Writes several values to memory locations, modifies a global variable using bitwise operations, and interacts with audio and DMA hardware registers.
- `process_asset_data`: Shifts a series of words left by one bit and performs a nested loop to copy data from a source table into multiple destination memory locations.
- `populate_asset_resource_tables`: Populates a series of global tables by iteratively storing function pointers and associated integer values into specific memory offsets.
- `initialize_data_tables_2`: Sets up pointers and executes a series of loops that call other routines and modify a global variable.
- `process_data_and_copy`: Manages a state machine and performs a large block memory copy by interleaving data from a source table into a destination table at specific offsets.
- `reset_global_variable`: Sets a global variable to zero and then immediately to one.
- `verify_routine_pointer`: Sets a global variable to one and then compares the value at a pointer to the address of the current routine.
- `execute_conditional_loop`: Manages a state machine that uses a global variable to control a loop and executes various routines based on specific value comparisons.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `initialize_and_clear_tables`: Executes several function pointers, clears a large memory block, and sets various global pointers to specific memory locations.
- `copy_and_mask_data`: Copies data from one memory location to another using nested loops and performs bitwise AND and OR operations on the destination buffers.
- `initialize_asset_resource_tables_2`: Populates various global tables with function pointers and configuration parameters by executing a sequence of specialized subroutines.
- `recursively_check_global`: Performs recursive calls to itself based on the value of a global variable.
- `check_global_condition`: Checks if a global variable is zero and, if not, calls another routine multiple times.
- `decrement_global_counter`: Assigns a value to a global variable and decrements it if it is not zero.
- `recursively_call_routine`: Checks if a global variable is non-zero and, if so, performs ten consecutive recursive calls to itself.
- `decrement_global_counter_alt`: Assigns a different value to a global variable and decrements it if it is not zero.
- `process_memory_block`: Iterates through a memory block, modifying word values and calling another routine to blend data if certain conditions are met.
- `search_and_modify_table`: Iterates through a table looking for specific word values, and if a condition is met, modifies a byte in the table and writes a value to a memory location.
- `iterate_and_update_bounds`: Iterates through a table of values, performing complex bounds validation on each entry.
- `process_text_file_data`: Reads data from a text file and processes it through a series of state-driven loops.
- `initialize_data_expansion`: Sets a value based on a global variable and calls routines to expand a data table and convert an integer to a string.
- `initialize_alternate_expansion`: Sets a different value based on a global variable and calls routines to expand a data table and convert an integer to a string.
- `expand_data_table`: Reads a source table and copies blocks of data to a destination address based on values and offsets found within the source.
- `convert_integer_to_string`: Converts a numeric value into a sequence of ASCII characters stored in a buffer.
- `update_state_and_expand_tables`: Updates a global variable and sets several parameters before calling another routine to expand or shift data tables.
- `process_table_bounds_and_update_counters`: Iterates through a table of values to identify specific markers and perform range-based calculations.
- `update_blitter_state`: Updates a global variable and modifies hardware registers based on a calculated value derived from a bitwise operation.
- `process_memory_list`: Iterates through a list of words, performing conditional arithmetic and pointer updates based on specific values.
- `update_display_pointer_offsets`: Calculates and updates memory offsets based on a global state and writes them to a table.
- `apply_bitmask_to_buffer`: Performs bitwise AND and OR operations on chunks of memory using a source buffer to mask or combine pixel data.
- `blend_bitplane_data`: Performs bitwise logical operations to blend or mask bitplane data from a source table into a destination buffer.
- `clear_memory_block`: Fills a block of memory with zeros using a loop.
- `initialize_asset_parameters`: Stores several values from a pointer into global variables and then sets multiple global variables to specific constant values.
- `execute_file_initialization`: Calls several routines via pointers to initialize data and performs file-related operations while managing system permissions.
- `recursive_check_byte`: Recursively calls itself based on the value of a global variable.
- `decrement_global_counter_3`: Checks a global variable against a constant and performs a recursive call if certain conditions are met.
- `decrement_global_counter_4`: Performs a series of calls and decrements a global variable if it is non-zero.
- `update_bounds_and_flag`: Calculates several values from a global variable, performs bounds checks, and sets a global flag to one.
- `update_coordinate_tables`: Derives new values from a global variable to populate several memory locations through concatenation and arithmetic, then recursively calls itself based on specific conditional checks.
- `search_and_assign_values`: Iterates through a table to find a specific character and then stores two values into provided pointers.
- `search_and_assign_table_values`: Iterates through a table to find a specific character and then stores values into provided pointers.
- `process_table_elements`: Iterates through a table, performing bitwise operations and updating bytes based on complex conditional logic.
- `process_alternate_table_elements`: Iterates through a table to perform bitwise operations and update bytes using a different set of offsets.
- `apply_bitmask_to_table`: Performs bitwise operations on specific memory locations within a table using values derived from a global variable.
- `process_control_table`: Scans a table for specific byte values and updates three different memory locations based on a toggling boolean state.
- `update_state_via_table`: Iterates through a table of code pointers and updates a byte at a specific address based on a global variable's value.
- `apply_complex_bitmask`: Uses nested loops to perform bitwise AND and OR operations on multiple memory locations within a table.
- `mask_and_clear_data`: Applies a bitwise AND mask to a word at a pointer and clears a word at another pointer.
- `clear_register_byte`: Performs a bitwise OR operation on a byte in a data register with zero.
- `tiny_andi_andi_andi`: Small routine of 3 instructions that masks (a0)+ with 0, then masks (a0)+ with 0, then masks (a1)+ with 0.
- `mask_and_clear_memory`: Applies a bitwise AND mask to a word at a pointer and then clears a word at a second pointer by zeroing its bits.
- `clear_data_register_byte`: Performs a logical OR operation on a byte in a data register using zero to clear its value.
- `clear_multiple_words`: Clears two words at one pointer and one word at another pointer using bitwise AND operations.
- `process_table_data`: Iterates through a memory range, performing conditional updates to bytes and words based on values in a table and a global variable.
- `write_table_entry`: Searches a table for a terminator, then writes a byte and two words to the location following the terminator.
- `traverse_and_modify_list`: Iterates through a list of words, modifying values by adding or subtracting offsets and updating several global variables.
- `store_word_from_pointer`: Stores a word from a pointer into a global variable.
- `search_and_update_table`: Searches a table for a specific short value and, upon finding it, updates the table with a value from a register and increments a subsequent entry.
- `traverse_and_modify_table`: Iterates through a table of shortwords, performing conditional arithmetic increments or decrements on the values based on specific byte comparisons.
- `store_long_to_global`: Loads a longword from the address pointed to by a register and stores it into a global variable.
- `scan_and_insert_data`: Iterates through a buffer looking for specific byte markers to insert a sequence of three values into three different memory streams.
- `process_command_list`: Parses a list of commands from a global table, modifying the table pointer and updating a global variable based on the byte value found at a specific offset.
- `update_global_list_a`: Iterates through a list of structures, modifying values based on a global flag and calling another routine for each valid entry.
- `parse_data_table_sequence`: Iterates through a table of words until a specific terminator is found.
- `update_global_list_b`: Iterates through a list of structures, modifying values based on a global flag and calling another routine for each valid entry using different constant values.
- `process_command_sequence`: Iterates through a table of words, updating a global variable and a status byte based on comparisons with a constant and a global value.
- `process_data_table_sequence`: Iterates through a table of words until a specific terminator is found.
- `apply_bitmask_to_buffer_2`: Performs a complex bitwise operation on multiple memory buffers using values from a table and a toggleable global variable.
- `find_byte_marker`: Scans a memory buffer for one of two specific byte values and stores the resulting offset into a destination pointer.
- `search_table_and_update_state`: Iterates through a table of words to find a specific value and updates a global variable based on various conditional checks.
- `compare_long_value`: Compares a long value at a specific memory location to a constant and branches if they are not equal.
- `setup_bitwise_mask`: Sets two values and calls a routine to apply a bitwise mask.
- `process_state_machine_transitions`: Updates a global state variable through a series of conditional checks and recursive calls.
- `compare_global_word`: Compares a word stored at a global memory location with a constant value and branches if they are not equal.
- `setup_mask_parameters`: Stores a value from a global variable into a data register and sets another register to a constant value before calling a bitwise mask routine.
- `check_range_bounds`: Checks if a global variable falls within a specific range relative to another global variable and sets a flag if it does.
- `compare_word_and_branch`: Compares a word at a memory location to a constant value and branches to another routine if they are not equal.
- `setup_mask_parameters_alt`: Loads a value from a global variable into a data register and sets a second register to a constant value before calling a bitwise mask routine.
- `validate_range_bounds`: Compares a global variable against two bounds and sets a global variable to one if the value falls within a specific range.
- `compare_long_value_2`: Compares a longword at a specific memory location to a constant and branches if they are not equal.
- `setup_mask_parameters_third`: Assigns a value from a global variable to a data register and sets another register to a constant value before calling a bitwise mask routine.
- `copy_interleaved_data`: Decrements a value from a global variable and uses it to control a nested loop that copies data from a source table into three separate destination buffers at specific offsets.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_buffer_pointers`: Loads several addresses from memory into registers and performs a conditional branch based on a byte comparison.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `distribute_source_data`: Copies data from a source buffer into three separate destination buffers using a specific interleaved pattern.
- `set_global_flag`: Checks if a global variable is zero and, if so, sets it to one.
- `update_global_state`: Checks a global variable against several constant values and performs arithmetic or assignments to update it and other memory locations.
- `set_flag_if_zero`: Checks if a global variable is zero and, if so, sets it to one.
- `update_global_state_alt`: Compares a global variable to specific constants and modifies it or other memory locations based on the result.
- `reset_global_parameters`: Modifies a 32-bit value based on several range checks and then sets a large number of global variables to a constant value.
- `initialize_global_values`: Sets several global variables to a constant value and performs a conditional word assignment based on a global flag.
- `update_parameter_bounds`: Modifies several global variables and performs conditional logic based on the value of a longword.
- `initialize_global_table_2`: Sets multiple global variables to a constant value and performs conditional word assignments based on a global variable.
- `rotate_bit_stream`: Performs a bitwise rotation on groups of words within a nested loop structure and writes the results to a destination table.
- `rotate_bit_stream_extended`: Processes five words at a time through a bitwise rotation loop and stores the shifted values into a destination buffer.
- `scatter_data_to_buffer`: Reads bytes from a source table using an index and scatters them into specific non-contiguous offsets within a destination buffer.
- `scatter_bytes_to_memory`: Calculates a source address based on a value and then copies a long sequence of bytes from that source to various non-contiguous offsets in a destination buffer.
- `decrement_local_variable`: Initializes a local stack variable to five and enters a loop that decrements the value until it reaches negative one.
- `check_buffer_value`: Calculates an offset into a buffer using two values and compares the byte at that location against a global variable and two constant values.
- `call_scatter_routine`: Pushes the contents of a register onto the stack, calls another routine, and then restores the register from the stack.
- `copy_structured_data_blocks`: Copies multiple non-contiguous blocks of data from a source buffer to a destination buffer.
- `update_state_variable`: Checks if a global variable equals a specific value and, if not, copies that variable to another memory location.
- `update_state_counters_and_buffer_checks`: Increments or decrements a global variable and modifies several global counters based on comparisons between a global variable and values retrieved from a buffer.
- `sync_data_field`: Tests a global variable against a constant and updates a different memory location with the variable's value if the condition is met.
- `update_state_machine_and_buffer_offsets`: Manages a complex state machine by incrementing or decrementing a global variable and updating two offset values based on comparisons against a buffer.
- `calculate_offset_address`: Calculates a value using a multiplication and addition formula and stores the result through a pointer.
- `process_state_logic_a`: Uses a recursive loop to update a local counter and modifies a global variable and a memory value based on specific state checks.
- `process_state_logic_b`: Performs recursive updates to a local counter and modifies a global variable and a memory value through conditional branching.
- `decrement_global_counter_2`: Checks a global variable against a threshold and decrements it if it is within a specific range, while also swapping a global value with another.
- `search_and_modify_table_2`: Searches a table for a specific word and, upon finding a match or a specific value, modifies several entries and a global variable.
- `traverse_and_modify_data_table`: Iterates through a table of values, performing recursive conditional checks and arithmetic updates on the elements.
- `update_joystick_logic`: Updates a global bitmask based on joystick input and modifies hardware display registers and other global variables through various state transitions.
- `process_buffer_logic_a`: Recursively iterates through a buffer using a counter and modifies a global variable and a memory value based on the contents of a structure.
- `update_state_machine_logic`: Manages a complex state machine by evaluating a bitmask and a byte-based counter within a structure.
- `decrement_and_swap_register_values`: Decrements a global variable if it is less than six and not zero, then performs a swap of a value between two different locations using a register as temporary storage.
- `search_and_modify_table_alt`: Iterates through a table looking for specific values to perform conditional updates to table entries and a global variable.
- `traverse_and_modify_data_table_2`: Iterates through a table of values, performing recursive conditional checks and arithmetic adjustments on elements.
- `update_mouse_logic`: Updates a global bitmask based on mouse input and modifies hardware display registers and other global variables through various state transitions.
- `process_recursive_buffer_state`: A recursive routine that traverses a structure to update a global counter and modify specific bytes within a buffer based on conditional checks.
- `update_recursive_buffer_state`: A recursive routine that traverses a data structure to modify bytes within a buffer and update a global counter.
- `update_state_machine_logic_2`: Updates a global state variable through a series of conditional checks and bitwise operations.
- `update_table_entry`: Scans a table for a terminator or a specific value, then modifies table elements based on the value of a global variable.
- `traverse_and_modify_buffer`: Iterates through a buffer using conditional increments and decrements based on the values encountered.
- `update_state_machine_logic_3`: Manages a complex state machine by evaluating bits within a global variable to perform conditional increments, decrements, and resets.
- `modify_table_data`: Searches a table for a specific marker and performs arithmetic updates on table entries based on a global variable's value.
- `recursively_adjust_global_value`: Recursively modifies a global short value by incrementing or decrementing it based on its current state and specific boundary conditions.
- `load_and_decompress_asset_files`: Iterates through a list of data files to perform file operations, bitstream rotation, and data block reformatting.
- `update_limit_values`: Checks a global variable and updates a memory location with a long word, while adjusting a value based on a comparison between two other variables.
- `update_clamped_value`: Checks a global variable and updates a 16-bit value in a table using clamped arithmetic based on a range check.
- `search_and_replace_words`: Iterates through a list of words to find specific values and replaces them with a constant or shifts data within the list.
- `search_and_replace_words_2`: Traverses a list of words, calling another routine for each entry and modifying the list contents if a condition is met.
- `search_with_offset_adjustment`: Performs a series of conditional searches in a list while adjusting two global offset variables.
- `reset_table_parameters`: Sets several global variables and memory locations to specific constant values after checking a condition.
- `reset_table_parameters_2`: Initializes a set of global variables with specific constants and returns a value based on a series of checks.
- `search_with_alternative_offset`: Executes a sequence of conditional searches using a different set of global offset variables.
- `find_character_in_list`: Iterates through a list of bytes to find a specific character determined by a calculation involving two global variables.
- `iterate_and_modify_table`: Iterates through a table of words looking for specific values and performs conditional updates.
- `iterate_and_modify_table_2`: Iterates through a table of words looking for specific values and conditions, then modifies entries within the table and updates global variables.
- `clear_dual_tables`: Fills two parallel tables with zeros for a fixed number of iterations.
- `clear_large_tables`: Fills two parallel tables with zeros for a large number of iterations.
- `store_byte_to_global`: Stores a single byte from a pointer into a global variable.
- `process_recursive_list`: Decrements a word value and recursively calls itself multiple times while updating various global variables and registers.
- `increment_list_elements`: Iterates through a list of words, incrementing byte values at specific offsets and marking entries with a constant if a character match occurs.
- `increment_list_elements_2`: Processes a list by incrementing byte values and calling another routine, potentially marking entries as processed.
- `execute_loop_until_zero`: Increments a global variable and repeatedly calls another routine until a specific register value becomes zero.
- `execute_nested_loops`: Performs nested loops that call another routine multiple times based on a counter and a return value.
- `configure_blt_hardware`: Sets various registers for the Blitter hardware and waits for the operation to complete.
- `setup_blt_registers`: Sets several Blitter registers and waits for the Blitter to finish processing.
- `configure_blt_transfer`: Sets Blitter registers and performs arithmetic using values from a table before waiting for the Blitter to finish.
- `clear_d0_register`: Performs a logical OR operation on a data register using a zero immediate value.
- `increment_global_counter`: Increments a global variable if it is zero and then calls another routine multiple times.
- `execute_indirect_call`: Compares two values and, if a condition is met, increments a pointer and calls a function at the new address.
- `populate_data_structure`: Searches a list for a specific byte value and, upon finding it, inserts a new entry containing a constant and a calculated value into the list.
- `update_list_values`: Iterates through a list of words until a terminator is found, decrementing word values and updating state bytes based on specific thresholds.
- `search_and_modify_table_3`: Iterates through a table to find a specific value, then modifies the table entries and stores a pointer.
- `clear_d0_register_2`: Performs a logical operation to clear the contents of a data register.
- `modify_table_entries`: Iterates through a table of words, decrementing values and updating pointers and state bytes based on the contents of the table.
- `load_pointer_addresses`: Loads several memory addresses into registers and compares a word at the first address to a constant.
- `increment_pointers_conditionally`: Compares a word at a pointer to a constant and, if not equal, increments several pointers and a register by specific amounts.
- `copy_data_block`: Copies three longwords from a source to a destination and stores a constant value.
- `blend_table_data`: Iterates through a table, following pointers to skip certain values and adding data from those pointers to the table entries.
- `validate_and_prepare_pointers`: Loads several memory addresses into registers and compares a word at a specific location to a constant.
- `increment_pointers_and_registers`: Compares a word at a pointer to a constant and, if not equal, increments several pointers and registers by one or four.
- `copy_and_initialize_data`: Copies three longwords from a source to a destination, clears a byte, and sets two other values.
- `update_graphics_table`: Iterates through a table, following pointers to skip certain values and adding data from those pointers to the table entries while updating a state byte.
- `initialize_global_table`: Populates a large number of global variables and table entries with values derived from a source block.
- `update_object_list_coordinates`: Iterates through a linked list of data structures, updating their position values by adding offsets stored within the list elements.
- `traverse_and_replace_table`: Iterates through a table of words, performing a search and replacement operation or calling another routine based on the values found.
- `find_and_replace_value`: Searches a memory block for a specific word, then replaces it with a longword and clears a byte.
- `search_table_for_value`: Iterates through a table of words until it finds a specific value or a terminator, then writes a value to the table.
- `increment_list_bytes`: Iterates through a list of words, incrementing byte values and calling another routine until a specific byte value is reached.
- `increment_list_counters`: Iterates through a list of words, incrementing a byte value and calling another routine until a specific byte value is reached.
- `modify_memory_blocks`: Performs bitwise operations on specific memory locations using values retrieved from a table and updates several memory offsets.
- `apply_bitwise_mask`: Uses a nested loop to apply bitwise OR and AND operations on a large block of memory using values from a source table.
- `return_from_subroutine`: The routine is empty and performs no operations before returning.
- `apply_bitwise_mask_to_memory`: Performs nested loops to combine data from one memory area with another using bitwise AND and OR operations.
- `shift_bits_in_table`: Iterates through a table of words, performing a circular left shift on the bits of each word across multiple elements.
- `merge_word_tables`: Iterates through a table of words, performing a bitwise OR on groups of four words and storing the result in a new table.
- `shift_bits_in_subset`: Performs a circular left shift on the bits of words within a specific memory range through nested loops.
- `merge_subset_tables`: Iterates through a table of words, performing a bitwise OR on groups of three words and storing the result in a new table.
- `setup_blt_transfer`: Configures various hardware registers for a bitblock transfer and waits for a DMA flag to clear.
- `conditional_blt_transfer`: Checks if certain values are within range, then configures hardware registers for a bitblock transfer and waits for the operation to complete.
- `blend_bitplane_data_2`: Performs bitwise logical operations to blend or mask data across multiple bitplanes.
- `apply_bitwise_mask_2`: Performs bitwise AND and OR operations on memory blocks using values from a source table.
- `blend_graphics_data`: Combines data from two sources using bitwise operations to update specific memory regions.
- `move_long_word_to_address`: Copies a long word from a memory location pointed to by a register to a specific memory address.
- `blend_graphics_data_2`: Iterates through data to apply bitwise masking and logical OR operations to memory buffers.
- `fill_table_with_multiples`: Fills a table with values that increase by a fixed increment in a loop.
- `add_table_values`: Iterates through a table, adding values from another table to the current elements and applying range constraints.
- `search_table_for_value_2`: Scans a table for specific word values and writes a new value if a match is found.
- `search_and_update_table_2`: Searches a table for a specific word or terminator and then updates multiple entries in that table with new values.
- `calculate_coordinate_difference`: Calculates the absolute difference between two sets of values and returns a value based on their relative magnitudes.
- `check_range_overlap`: Compares several global values to determine if they fall within specific bounds and returns a boolean result.
- `validate_complex_bounds`: Performs multiple conditional checks on a set of global values to determine if they satisfy a complex intersection of ranges.
- `update_joystick_state`: Reads the joystick port and CIAA register to update a global bitmask representing button and direction states.
- `update_mouse_state`: Reads the mouse port and CIAA register to update a global bitmask representing button and movement states.
- `process_joystick_input`: Updates a global bitmask and several counters based on the state of a bitmask and specific threshold values.
- `update_input_state_counters`: Updates several global counters and state flags based on the current value of a bitmask.
- `manage_recursive_initialization`: Increments a global variable and uses it as a counter to control a recursive sequence of calls.
- `initialize_table_entry`: Searches a table for a specific value and, upon finding a match, writes several values to that location and updates a global variable.
- `decrement_and_process_table`: Iterates through a table, calling another routine to process data and decrementing a byte-sized counter for each element.
- `find_valid_table_entry`: Iterates through a table to find an entry that satisfies specific range conditions and updates global values.
- `modify_table_attributes`: Searches a table for a specific value and modifies multiple fields within the found entry based on the current value of a global variable.
- `update_table_data`: Searches a table for a specific value and writes a set of values to the entry, adjusting a global variable and a specific memory location.
- `update_table_elements`: Modifies values within a table based on arithmetic offsets and calls another routine if certain conditions are met.
- `search_and_replace_table`: Iterates through a table to find a specific value and replaces the entry with new data from registers.
- `add_table_with_limit`: Iterates through a table, adding values from another table to the current elements and capping the result at a specific value.
- `manage_recursive_initialization_2`: Increments a global variable and performs a recursive call if the variable is zero.
- `reset_table_entry`: Searches a table for a specific value and performs a series of writes to that entry, including arithmetic adjustments.
- `iterate_and_modify_data_table`: Iterates through a memory table until a specific termination value is encountered.
- `find_valid_table_entry_2`: Searches a table for entries matching specific criteria and updates global registers upon success.
- `update_table_elements_2`: Adjusts table entries using arithmetic operations and performs conditional routine calls based on the results.
- `replace_table_entry_with_id`: Searches a table for a specific value and replaces the entry with new data, while also updating a global variable with a specific identifier.
- `accumulate_table_values`: Iterates through a table of short integers, adding the value of a preceding element to the current element.
- `toggle_register_values`: Selects between two values to store in a register based on whether a global variable is zero, then updates that global variable.
- `rotate_and_mask_bitstream`: Performs bitwise rotations on sets of three 32-bit words and applies the resulting bitmask to a destination buffer.
- `rotate_and_loop_word_data`: Loads multiple words from a memory location into registers and performs a logical right rotation on three of those registers.
- `perform_bitmask_logical_operations`: Iterates through a data structure to perform bitwise AND and OR operations on words using values shifted from three source registers.
- `rotate_bitfield_data`: Performs bitwise circular shifts on multiple data fields stored in a structure.
- `perform_bit_rotation_on_data_table`: Retrieves a block of data from a table using an offset and performs a series of bitwise shifts and rotations on the values.
- `shift_bitmask_table_data`: Fetches a data structure from a table and performs bitwise shifts and logical AND operations on its contents.
- `extract_bitstream_to_registers`: Reads a bitstream from a memory table and performs bitwise shifts to populate a set of registers.
- `test_and_branch_byte`: Tests a byte at a pointer and branches if it is non-zero.
- `rotate_bitstream_buffer`: Calculates a memory offset based on a value and performs bitwise rotation and masking on a series of words within a buffer.
- `calculate_buffer_offset`: Performs arithmetic on a register to derive a value, stores the result in a longword at a specific memory location, and calculates a new address in a register using shifts and additions.
- `compare_memory_value`: Compares a word at a specific memory location against a constant and branches if the value is less than that constant.
- `apply_bitmask_to_buffer_3`: Performs bitwise rotation and logical masking on a series of 16-bit words within a buffer.
- `rotate_and_mask_bitbuffer_2`: Performs bitwise rotation and logical masking on a series of 16-bit words within a buffer.
- `rotate_and_mask_bitbuffer`: Performs bitwise rotation and logical masking on a series of 16-bit words within a buffer.
- `update_bitmask_buffer`: Performs bitwise operations on multiple 16-bit words within a buffer using shifted values from several registers.
- `update_recursive_state_counter`: Increments a counter and performs a recursive state update based on a comparison with a threshold.
- `tiny_andi`: Small routine of 1 instruction that masks (a0)+ with 0.
- `perform_logical_or`: Performs a logical OR operation on a data register using zero as the operand.
- `manage_timer_and_bounds`: Updates a global counter and performs conditional logic involving bounds checking and memory clearing.
- `clear_memory_word`: Performs a bitwise AND on a word at the address pointed to by a register and increments the pointer.
- `clear_data_register`: Clears the data register zero by performing a logical OR with zero.
- `compare_and_store_byte`: Compares two bytes from memory and, depending on the result, calls another routine and stores a specific byte into a global variable before clearing it.
- `wait_for_hardware_status`: Polls a global variable until specific bit patterns are met and then executes a descending loop.
- `copy_color_data`: Copies sixteen words from a source pointer to the hardware color registers.
- `initialize_function_pointer_table`: Sets several global variables to specific values and function addresses, then repeatedly calls a routine with incrementing pointer offsets.
- `setup_function_pointers`: Stores several routine addresses and constant values into a table of global variables.
- `initialize_data_blocks`: Populates a series of memory locations with specific offsets and routine addresses in a repetitive pattern.
- `initialize_asset_resource_tables`: Populates a series of global tables with function pointers and configuration parameters.
- `initialize_asset_data_structures`: Loads a series of pointers and constant values from a data block into global variables and function pointer tables.
- `clear_d0_register_3`: performs a logical OR operation on a data register with zero to ensure its value is zeroed.
- `concatenate_registers`: Combines the contents of two data registers into a single 64-bit value returned to the caller.
- `reorder_data_elements`: Iterates through a source table to reorder and copy word-sized elements into a destination table using specific offsets.
- `execute_file_operations`: Calls several functions through pointers to perform file-related tasks.
- `initialize_global_tables`: Sets several global pointers to memory addresses and clears a specific range of memory with zeros.
- `allocate_and_initialize_memory`: Allocates memory via a library call, sets up several global pointers, and clears a small block of memory.
- `assign_memory_pointers`: Calculates and stores a large number of memory addresses into various global variables and clears a memory block.
- `setup_hardware_registers`: Writes specific values to various hardware registers including bitplane, copper, and display control registers.
- `configure_hardware_and_tables`: Sets numerous global pointers to memory addresses, clears a memory block, and interacts with hardware registers including DMA and display control.
- `update_conditional_byte`: Selects a byte from one of two memory locations to store in a global variable based on a comparison, then updates a flag.
- `update_conditional_pointers`: Compares two global values and, depending on the result, calls another routine and updates a global variable.
- `update_function_pointers`: Polls a global variable for specific bit patterns and then stores a routine address into two global variables.
- `initialize_data_tables`: Populates two tables with structured data including incrementing values and specific constant sequences.
- `process_color_state_machine`: Manages a state machine that handles color value decrementation, data buffering, and recursive processing of color tables.
- `decrement_color_values`: Iterates through a table of words, decrementing non-zero values based on their bit patterns and counting zero entries.
- `reformat_and_bitshift_data_blocks`: The routine first reorganizes a structured table of data by combining and reordering elements into a new memory layout.
- `expand_and_shift_bitstream`: Expands a source table into a larger buffer by combining and masking bitwise values, then performs a multi-pass bitwise right-shift across the resulting data structure.
- `expand_and_shift_bitmask_table`: This routine expands a source table of compressed bitmask data into a larger destination table by performing bitwise OR operations to reconstruct full words.
- `copy_and_transform_data_table`: Copies a block of data from a source table to a destination buffer, applying an offset to each byte during the transfer.
- `process_input_sequence_buffer`: Updates directional and button state variables based on input bitmasks and uses those values as indices to fetch bytes from a data table.
- `clear_dual_tables_2`: Clears two large memory blocks by iterating through nested loops and setting specific offsets to zero.
- `clear_single_table`: Clears a large memory block by iterating through nested loops and setting specific offsets to zero.
- `execute_sequential_routines`: Calls another routine seven times in succession.
- `clear_memory_and_loop`: Clears a block of memory by iterating through a pointer and then enters a loop that calls another routine based on the state of a global variable.
- `rotate_data_blocks`: Performs a series of block memory shifts by iterating through tables and copying chunks of data from one memory location to another.
- `rotate_global_values`: Shifts the values of two adjacent global variables by moving the contents of one into the other and temporarily storing the original value.
- `copy_indexed_data`: Reads a sequence of bytes to determine an index, then copies blocks of data from a table to a destination based on that index.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `wait_for_flag`: Enters a loop that continues as long as a specific bit in a global variable remains set.
- `clear_d0_register_4`: Performs a logical OR operation on the data register zero with zero.
- `clear_d0_register_5`: Performs a logical OR operation on the data register zero with zero.
- `clear_data_register_byte_2`: Performs a logical OR operation on a byte within a data register using a zero value.
- `clear_data_register_byte_3`: Performs a bitwise OR operation on a byte in a data register using zero.
- `clear_data_register_byte_4`: Performs a logical OR operation on a byte in a data register using a zero value.
- `clear_data_byte`: performs a logical OR operation on a byte in a data register using zero to ensure the value is cleared.
- `clear_data_byte_2`: performs a logical OR operation on a byte in a data register with zero.
- `zero_data_register`: Applies a bitwise OR operation with zero to a byte in a data register.
- `apply_zero_mask`: Uses a bitwise OR instruction to clear the bits of a byte in a data register.
- `or_zero_byte`: Executes a bitwise OR operation on a byte within a data register using zero as the operand.
- `set_register_zero`: Performs a bitwise OR with zero on a byte in a data register.
- `clear_data_register_byte_5`: Performs a bitwise OR operation on the least significant byte of a data register using a zero value.
- `clear_data_register_byte_6`: Performs a logical OR operation on the least significant byte of a data register using a zero value.
- `zero_byte_register`: Uses a bitwise OR instruction to apply zero to a byte in a data register.
- `or_byte_zero`: Executes a bitwise OR operation with zero on a byte in a data register.
- `apply_zero_to_register`: Performs a bitwise OR operation on a byte in a data register using zero.
- `clear_data_register_byte_7`: Sets the least significant byte of a data register to zero.
- `clear_data_register_2`: Sets the lowest byte of a data register to zero using a bitwise OR instruction.
- `clear_data_register_3`: Applies a bitwise OR with zero to a byte in a data register.
- `clear_data_register_4`: Uses a bitwise OR instruction to clear a byte in a data register.
- `clear_d0_register_6`: Performs a logical OR operation on a data register with zero to clear its value.
- `perform_no_operation`: Performs a bitwise OR operation on a byte in a data register using a zero immediate value.
- `clear_data_register_5`: Executes a bitwise OR instruction on a byte in a data register using zero as the operand.
- `clear_data_register_6`: Performs a bitwise OR with zero on a byte within a data register.
- `clear_data_register_7`: Sets a byte in a data register to zero via a bitwise OR instruction.
- `clear_data_register_byte_8`: Performs a logical OR operation on a byte within a data register using a zero value.
- `clear_data_register_byte_9`: Clears the lowest byte of a data register using a logical OR operation with zero.
- `perform_logical_or_zero`: Performs a logical OR operation on the lowest byte of a data register using zero.
- `apply_zero_mask_2`: Uses a bitwise OR instruction to modify a byte in a data register with zero.
- `set_register_byte_zero`: Performs a bitwise OR on a byte within a data register using zero as the operand.
- `clear_d0_register_7`: Performs a logical OR operation on a data register with zero to clear its contents.
- `clear_d0_register_8`: Performs a logical OR operation on a byte of the data register with zero.
- `clear_data_byte_3`: Performs a logical OR operation on a byte in a data register with zero.
- `clear_data_register_byte_10`: performs a logical OR operation on a byte in a data register with zero.
- `execute_or_zero`: Uses a bitwise OR instruction to process a byte in a data register with zero.
- `clear_d0_register_9`: performs a logical OR operation on a data register with zero to clear its value.
- `clear_data_register_byte_11`: performs a bitwise OR operation on a data register with zero to clear its contents.
- `clear_data_byte_4`: performs a logical OR operation on a data register with zero.
- `clear_d0_register_10`: performs a logical OR operation on a data register with zero to clear its contents.
- `clear_data_byte_5`: performs a logical OR operation on a byte in a data register using zero.
- `clear_data_byte_6`: performs a logical OR operation on a byte in a data register using zero.
- `clear_d0_register_11`: Performs a logical OR operation on a data register using a zero immediate value.
- `clear_data_byte_7`: Sets the least significant byte of a data register to zero.
- `clear_data_byte_8`: performs a logical OR operation on a byte in a data register with zero.
- `perform_bitwise_or_on_d0`: Performs a bitwise OR operation on the lowest byte of the data register zero using a zero value.
- `clear_d0_register_12`: Performs a logical OR operation on the data register zero with zero to clear its contents.
- `perform_zero_operation`: Performs a logical OR operation on a data register with zero.

### mouse

- `wait_for_fire_button_press`: Polls the CIA port register until bit six reads zero, which the register note ties to the mouse or joystick fire line, then returns zero.

### border

- `parse_amiga_hunk_loader`: Parses a memory structure containing AmigaDOS hunk type codes to perform data relocation and memory copying.
- `stub_empty_return_routine`: This routine is empty and contains only a return instruction.
- `decrement_value_loop`: Decrements a register value in a loop until it reaches zero.
- `tiny_ori_2`: Small routine of 1 instruction that merges 208 into d0.
- `decompress_back_reference_stream`: Decodes a packed byte stream by reading control values that either copy literal bytes or repeat earlier output with a length and offset from the stream.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_byte_cycle_counter`: Decrements a byte register once for each count unit, wrapping to a space character when it reaches zero, and returns.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Hellraisers (1988)(Exocet Software)[cr MWB].adf:copy

- `find_max_and_populate_buffer`: Traverses a linked list to find the maximum value in a specific field and then populates a buffer with a header and a sequence of incrementing values.
- `move_and_branch_2`: Moves the contents of one register to another and then performs a short branch.

### Hellraisers (1988)(Exocet Software)[cr MWB].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
