# Game 783 (bionic commando (1987)(go!)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00783). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 31 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 1070 raw-word lines |
| Functions named and described | pass, 591 of 591 named (0 author symbols), 591 of 591 described (16 by rule, 570 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 887 of 887 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 10624 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7618 of 7654 instructions identical in start and length, 3 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 494224 |
| code | 35280 |
| data | 163516 |
| padding | 7908 |
| unused | 200192 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| boot | 1 | 17 | 1 |
| endcli | 2 | 117 | 2 |
| Bionic Commando (1987)(GO!).adf:tune4.dat | 1 | 14 | 1 |
| Bionic Commando (1987)(GO!).adf:tune2.dat | 1 | 13 | 1 |
| Bionic Commando (1987)(GO!).adf:bionic | 1 | 7437 | 582 |
| Bionic Commando (1987)(GO!).adf:tune5.dat | 1 | 12 | 1 |
| Bionic Commando (1987)(GO!).adf:tune3.dat | 1 | 14 | 1 |
| Bionic Commando (1987)(GO!).adf:tune1.dat | 1 | 20 | 1 |
| Bionic Commando (1987)(GO!).adf:bootblock | 1 | 10 | 1 |
| Bionic Commando (1987)(GO!).adf:level1.spr (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- bionic commando (1987)(go!)/Bionic Commando (1987)(GO!).adf:level1.spr: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 10624 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### boot

- `initialize_system_environment`: Loads segments and manages library handles by calling system functions and storing a returned value in a global variable.

### endcli

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_data_block`: Copies sixteen words from a source address to a destination address via a loop.

### Bionic Commando (1987)(GO!).adf:tune4.dat

- `null_routine`: This routine is empty and contains no executable instructions.

### Bionic Commando (1987)(GO!).adf:tune2.dat

- `modify_memory_byte`: Performs arithmetic on a byte at a specific offset within a buffer and performs redundant reads.

### Bionic Commando (1987)(GO!).adf:bionic

- `initialize_system_and_load_assets`: Opens required libraries, initializes global data structures, and performs a sequence of file operations to load sprite and panel data.
- `initialize_dispatch_table`: Clears several global variables and then populates a table with a sequence of function pointers.
- `populate_dispatch_table`: Iteratively writes a sequence of function pointers into a memory location.
- `check_and_update_state`: Checks a bitmask in a global variable and updates another global variable if specific conditions are met.
- `initialize_data_tables`: Populates several memory locations and tables with specific constants and routine pointers.
- `call_other_routine`: Calls another routine and returns.
- `copy_memory_blocks_2`: Copies data from a source pointer to multiple destination locations at fixed offsets in a loop.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `recurse_indefinitely`: This routine calls itself without performing any other operations.
- `write_table_values`: Writes specific byte values into two different locations within a table.
- `update_global_values`: Checks a global variable against a constant and updates two other global variables with specific values based on a comparison.
- `replace_character_sequence`: Iterates through a table and replaces occurrences of a specific character with two new byte values if a global variable matches certain criteria.
- `update_global_pointer`: Updates a global variable using an index derived from a global value to select an entry from a table and then loads a byte from a memory address.
- `conditional_loop_execution`: Checks a global variable and, if it equals one, enters a loop calling another routine until it returns zero.
- `load_and_loop`: Sets a global variable to six and calls another routine before entering an infinite loop.
- `copy_memory_range`: Copies one hundred words from a global variable to a destination address specified by a pointer.
- `return_immediately_2`: The routine is empty and performs no operations before returning.
- `configure_hardware_registers`: Waits for a global variable to reach a specific value before writing a sequence of values to various hardware registers, including display and DMA controls.
- `configure_hardware_registers_3`: Polls a global variable and then writes specific values to hardware registers including DMA and interrupt control.
- `configure_hardware_registers_2`: Writes specific values to the DMA and interrupt enable hardware registers.
- `update_global_state`: Checks bits in a global variable and updates it to specific values, potentially triggering multiple recursive calls.
- `set_global_bit`: Checks a bit in a global variable and sets a different bit in that same variable if it is not already set.
- `decode_word_flags_into_byte`: Reads a word from the first pointer, tests specific bit pairs and individual bits, and sets corresponding bits in the byte pointed to by the second pointer.
- `process_serial_data`: Manipulates a global variable through bitwise operations and performs a busy loop based on the state of a bit.
- `pulse_keyboard_handshake`: Converts the stored keyboard serial byte, pulses the handshake line in the serial control register and waits until it reads back as set.
- `copy_memory_block`: Copies a block of data from a source address to a destination address using a fixed count.
- `calculate_pointer_address`: Loads a longword from a global variable, shifts it left by one, uses it as an index into a table to load a pointer into a1, and then calculates a new address in a0 by adding a value from a global variable and the value in a1.
- `update_iterative_pointer_offsets`: Iterates through a loop that calls multiple subroutines while incrementing three separate pointer or index values by fixed amounts.
- `copy_memory_blocks`: Copies multiple non-contiguous blocks of data from a source pointer to a destination pointer in a loop.
- `fetch_table_value`: Uses a value from a pointer and an index to fetch a byte from a table and store it in a destination.
- `process_nested_loops`: Executes nested loops that perform conditional calls and manipulate data based on return values.
- `fill_memory_with_ones`: Fills a block of memory with the value negative one.
- `clear_list_elements`: Iterates through a linked list or array, zeroing out specific bytes and copying values between offsets.
- `traverse_pointer_list`: Iterates through a list of pointers until a terminator value is encountered.
- `initialize_object_from_input`: Validates input values against global constants and, if successful, populates a new data structure with processed values and pointers.
- `validate_input_string`: Performs multiple checks on characters within a string and various global states to determine a return value.
- `process_complex_logic`: Evaluates a series of conditional branches based on string contents and global variables to return a calculated value.
- `validate_input_buffer_ranges`: Checks a global variable and a data table against several numerical bounds and non-zero constraints.
- `apply_amiga_hunk_relocations`: Checks a value against a specific hunk type code and, if matched, iterates through a table to perform memory relocations.
- `relocate_amigados_hunks`: Parses a table of data to identify and process relocation or executable code segments.
- `relocate_and_patch_hunks`: Checks for specific hunk type codes to perform memory relocation and then iterates through a data table to copy and patch longword values between memory blocks.
- `apply_amigados_relocations`: Parses a data structure containing hunk type codes to perform memory relocation and data copying.
- `execute_subroutine_chain`: Calls three consecutive instances of another routine and returns the resulting value in a register.
- `execute_callback_chain`: Calls three consecutive subroutines in sequence and returns the value held in a data register.
- `execute_subroutine_chain_2`: Calls three consecutive subroutines in sequence before returning.
- `execute_nested_subroutines`: Calls another routine three times in succession before returning.
- `calculate_scaled_values`: Performs arithmetic on an input value to return two concatenated results based on bit shifts and multiplications.
- `do_nothing`: This routine is empty and performs no operations before returning.
- `increment_table_values`: Iterates through a table and adds values to the elements if a specific byte is non-zero.
- `increment_offset_twenty_six`: Adds a value to a longword at a specific offset within a structure if it does not match a constant.
- `update_conditional_value`: Checks if a value at a specific offset within a structure is not equal to a constant.
- `reset_data_structure`: Checks specific character values and range constraints, then resets a structure to default values if conditions are met.
- `iterate_and_increment_offsets`: Iterates through a list of structures and adds register values to specific word offsets if they are non-zero.
- `increment_offset_four`: Adds a value to a longword at a specific offset within a structure if it is not zero.
- `accumulate_pointer_offset_value`: Checks if a value at a specific offset within a structure is non-zero, and if so, adds a register value to that offset location.
- `modify_global_structure`: Checks a global variable and performs a series of operations including bitwise masking and updating a structure with a value from another location.
- `set_bits_in_d0`: Performs a series of bitwise OR operations on the byte in d0 using constants from 1 to 12.
- `perform_data_copy`: Transfers data from a source address to a destination address by reading from various offsets within a block.
- `set_even_bits_in_d0`: Performs a series of bitwise OR operations on the byte in d0 using even constants from 0 to 22.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `increment_address`: Increments a pointer or address value.
- `decrement_address`: Decrements a pointer or address value.
- `initialize_data_tables_3`: Performs complex initialization including memory block copies, table population, and pointer updates based on global state.
- `process_character_data`: Evaluates character data against specific ranges and bit flags to update a global variable and copy data to multiple buffers.
- `increment_pointer_value`: Increments a short value at a pointer address by four if a global variable equals two.
- `decrement_pointer_value`: Decrements a short value at a pointer address by four if a global variable equals one.
- `decrement_offset_value`: Decrements a short value located at an offset of two bytes from a pointer if a global variable equals four.
- `increment_offset_value`: Increments a short value located at an offset of two bytes from a pointer if a global variable equals eight.
- `store_global_values`: Stores the value of a global variable into three consecutive memory locations pointed to by another global variable.
- `copy_memory_block_2`: Performs a series of memory moves and assignments using a loop to shift data between memory locations.
- `convert_to_hex_string`: Converts bytes into a hexadecimal string representation by splitting nibbles and adding an offset.
- `update_maximum_value`: Updates a global variable with the value of another global variable if the latter is greater than or equal to the former.
- `increment_index_with_limit`: Increments a value at a calculated offset within a global variable until it reaches a specific threshold.
- `execute_looping_logic`: Enters an infinite loop that repeatedly calls another routine while incrementing a global counter and checking bit flags.
- `reset_global_variable`: Sets a global variable to zero if it is not already zero and then calls another routine.
- `initialize_and_loop_logic`: Initializes data tables and enters an infinite loop that repeatedly calls another routine while incrementing a global counter and checking a bit flag.
- `check_global_condition`: Checks if a global variable equals one and, if so, executes a series of recursive calls.
- `check_specific_value`: Checks if a global variable matches a specific constant value and, if so, executes a series of recursive calls.
- `check_and_recurse`: Checks if a global variable matches a specific constant and, if so, calls itself multiple times.
- `load_pointer_from_table`: Retrieves a short value from a table using an index, then uses that value to load a longword from a memory location pointed to by a modified address.
- `check_status_flags`: Checks various bits and byte values within a structure and a global variable to determine whether to return or execute further logic.
- `return_masked_byte`: Returns a masked version of a global variable if a specific bit is not set, otherwise calls another routine.
- `update_status_byte_2`: Sets a byte at a specific offset within a structure to a value based on the state of a global variable's bits.
- `set_conditional_status`: Updates a byte in a structure with different values based on a combination of bit flags in a global variable.
- `evaluate_conditional_state`: Checks specific bytes and bit patterns within a global structure to conditionally update a byte in another global structure and perform recursive calls.
- `store_register_values`: Stores various register values into specific offsets of two different global structures based on a condition.
- `check_threshold_and_set_byte`: Sets a byte in a structure to one and then conditionally performs other actions if a value in a structure exceeds a threshold.
- `set_structure_values`: Writes specific byte values to two different structures and checks if a value is below a threshold and a bit is clear.
- `call_other_routines`: Calls another routine twice before returning.
- `modify_field_conditionally_2`: Updates a data field by adding a value, unless the field matches a specific constant, in which case it subtracts a different value and potentially updates a global variable.
- `check_global_status`: Checks a global variable against specific character values and enters an infinite loop if a condition is met.
- `check_global_status_2`: Evaluates a global variable and enters a loop if it matches a specific character.
- `check_and_halt_on_char`: Checks if a global variable matches a specific character; if it does, it updates the variable to a different value and enters an infinite loop if the variable matches another specific character.
- `nop_routine`: This routine is empty and performs no operations.
- `wait_for_zero`: Repeatedly checks a global variable until it becomes zero.
- `wait_for_null`: Loops until a specific global variable is equal to zero.
- `process_data_table_loop`: Iterates through a table to perform conditional longword transfers and block copies based on values retrieved from memory.
- `process_data_table`: Performs a complex loop that fetches data from a table, performs block transfers of longwords, and updates global values.
- `copy_memory_blocks_3`: Performs iterative block copies of data from a source table to a destination, with the source address determined by a global structure.
- `copy_table_blocks`: Iteratively copies blocks of data from one memory location to another based on values retrieved from a table.
- `copy_data_blocks`: Copies multiple values from a source table into two separate destination memory areas using a loop.
- `fetch_table_entry`: Returns a longword from a table using a signed offset if the offset falls within a specific range.
- `calculate_offset_value`: Checks if a signed word is within a specific range and returns a calculated difference if it is.
- `fetch_table_entry_2`: Checks if a signed word is within a specific range and returns a value from a table using a pointer calculation.
- `validate_range_call`: Invokes a routine that verifies if two signed values are within a specific non-negative range.
- `validate_range`: Checks if two signed values fall within a specific non-negative range.
- `process_pointer_offset_updates`: Checks a condition based on a pointer offset and updates a pointer and an integer value within a data structure.
- `update_pointer_data`: Checks a condition based on a pointer offset and updates a pointer and an integer value in a data structure.
- `modify_data_structure`: Decrements a value from a pointer, calculates an offset using a global variable, and updates a pointer and an integer in a structure.
- `initialize_memory_tables`: Clears a block of memory and populates a series of table entries with specific constant values.
- `set_structure_fields`: Writes several constant values to a structure and clears two bytes at an offset if a specific field is non-zero.
- `fill_code_table`: Fills a memory range with incrementing integer values and updates a global pointer.
- `search_table_for_zero`: Iterates through a table to find an entry where a specific byte is zero, then returns a calculated value.
- `find_and_reset_table_entry`: Iterates through a table of structures to find an entry where a specific byte is zero.
- `check_table_entry`: Checks if specific bytes at offsets within a table entry are non-zero and updates a value in the table if they are not.
- `search_and_clear_table_entry`: Iterates through a table of structures to find the first entry where a specific byte is zero.
- `process_status_flags`: Updates a data structure by checking bitmasks and copying values from a table based on calculated offsets.
- `update_status_flags`: Checks specific bits within a byte at an offset from a pointer.
- `call_if_bit_clear`: Checks if the high bit of a byte at a specific offset is clear before calling another routine.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `combine_table_values`: Iterates through a large table to perform bitwise OR operations on specific elements and stores the result in a new table.
- `process_table_data`: Performs bitwise OR combinations of four values from a source table and writes them to a destination table through nested loops.
- `merge_table_elements`: Computes bitwise OR results from specific offsets in a source table and populates a destination table.
- `accumulate_table_bits`: Uses nested loops to combine multiple fields from a source table into a single value per entry in a destination table.
- `tiny_ori_2`: Small routine of 1 instruction that merges 19 into d0.
- `clear_d0_byte`: Performs a bitwise OR operation on the byte in d0 with zero.
- `clears_data_register_zero`: Performs a logical OR operation on data register zero with zero.
- `clears_data_byte`: Performs a logical OR operation on a byte in register d0 with zero, effectively leaving the register value unchanged.
- `process_bitmask_data`: Reads multiple values from a source pointer and stores their bitwise OR result into a destination pointer.
- `performs_null_logical_operation`: Performs a logical OR operation on the lowest byte of a data register using a zero constant.
- `accumulate_source_bits`: Traverses a large table to perform bitwise OR operations on specific offsets and stores the result.
- `clears_data_register_zero_2`: Performs a logical OR operation on the least significant byte of data register zero with zero.
- `bitwise_or_tables`: Combines values from four different offsets within a source structure into a destination memory address.
- `zeroes_data_byte`: Performs a logical OR operation on a byte in register d0 with zero.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `map_source_to_destination`: Performs a repetitive bitwise OR of four values from a source table into a destination table.
- `clears_data_register_zero_3`: Performs a bitwise OR operation on data register zero with zero.
- `or_byte_zero`: Performs a bitwise OR operation on register d0 with zero.
- `clears_data_register_zero_4`: Performs a logical OR operation on the lowest byte of data register zero with zero.
- `clears_data_register_zero_5`: Performs a bitwise OR operation on the lowest byte of data register zero using a zero constant.
- `clears_data_register_zero_6`: Performs a logical OR operation on data register zero using a zero constant.
- `performs_bitwise_or_on_data_register_zero`: Applies a bitwise OR operation with zero to the data register zero.
- `bitwise_or_tables_2`: Processes a sequence of data by ORing four values from a source structure into a destination structure.
- `clears_data_register_zero_7`: Sets the least significant byte of data register zero to zero.
- `consolidate_table_data`: Performs a repetitive bitwise OR operation on four values from a source table and writes to a destination.
- `transform_data_structures`: Executes a nested loop that combines four values from a source table into a destination table via bitwise OR.
- `initialize_data_tables_2`: Performs complex initialization of multiple data structures, including clearing memory, setting specific offsets to constants, and executing a loop of function calls from a table.
- `execute_decrement_loop`: Executes a loop that decrements a value from a constant until it reaches a specific threshold.
- `initialize_bitmask_table`: Sets specific offsets within a table to a constant value and clears others to zero.
- `execute_repeated_callback`: Executes a routine stored at a location pointed to by a register five times.
- `copy_and_offset_table`: Copies data from a source table to a destination table while adding an offset to the first element and populating subsequent fields.
- `check_global_limits`: Compares values in a table against specific thresholds based on bits set in a global variable.
- `update_table_with_offset`: Iterates through a table to update entries by adding a value from a global variable and copying specific data fields.
- `increment_value_with_limit`: Adds a value to a field in a structure and copies a value from another field, provided the field is below a certain threshold.
- `decrement_value_with_limit`: Subtracts a value from a field in a structure and copies a value from another field, provided the field is above a certain threshold.
- `modify_field_conditionally`: Adds a value to a field unless it matches a specific constant, in which case it subtracts a different constant and potentially sets a global flag.
- `subtract_from_value`: Subtracts a short value from a short value stored at an offset of two bytes from a pointer.
- `decrement_and_copy_field`: Decrements a byte in a global structure and, if a certain threshold is met, copies a longword from a global variable to a structure offset.
- `execute_indirect_calls`: Loads a function pointer from a structure and executes it twice.
- `apply_bitwise_or_to_buffer`: Iterates through a list of structures to perform bitwise OR operations between a source buffer and a destination buffer.
- `copy_data_blocks_2`: Copies multiple blocks of data from a source table to a destination buffer using a loop and specific offsets.
- `clear_memory_range`: Fills a large block of memory with zeros using a decrementing loop.
- `return_immediately_3`: The routine performs no operations and returns to its caller immediately.
- `call_indirect_handler`: Executes a subroutine located at an address retrieved from a table using an index.
- `update_conditional_state`: Checks multiple conditions in a structure and a global variable to update a byte and a global status.
- `load_table_entry`: Loads a value from a table into a global variable and calculates two new values based on table contents and other globals.
- `populate_destination_structure`: Calculates new values from a table and populates a destination structure with these values and data from another memory location.
- `copy_three_words`: Copies three consecutive longwords from a source address to a destination address.
- `update_counter_and_data`: Validates a range for a global counter and either updates a destination with values from a pointer-based table or clears data and decrements the counter.
- `update_counter_and_clear_data`: Checks a global variable for specific values to trigger a recursive call, otherwise increments a global counter if certain bits in a pointed-to value are unset.
- `clear_and_decrement_counter`: Clears two words at a destination pointer, decrements a global counter, and sets a global variable to four.
- `execute_recursive_calls_5`: This routine consists of a series of eight identical, consecutive calls to itself.
- `process_table_updates`: Executes a sequence of tasks involving copying longwords from a table and updating table indices based on global conditions.
- `fetch_and_toggle_state`: Fetches a word from a table based on an index and an offset, increments the index, and toggles specific bytes in two different locations if a condition is met.
- `update_coordinate_bounds`: If two global counters differ, it reads two words from a table via a pointer and updates two different sets of pointers with modified values.
- `add_offset_to_values`: Adds a word fetched from a table at a specific index to two different memory locations.
- `copy_longword_from_table`: Copies a longword from a table at an index determined by a global variable into two different memory locations.
- `store_word_from_table`: Stores a longword from a table at an index determined by a global variable into another memory location.
- `loop_copy_table_data`: Iterates while a counter is not equal to another, copying longwords from a table into a destination while updating specific words within that data.
- `decrement_and_copy_data`: If a counter is non-negative, it clears two words at a destination, copies two words from a global location, and decrements the counter.
- `update_table_indices`: Checks several conditions against global variables and either clears two words in a table at a specific index or decrements two short values in a structure.
- `decrement_table_offsets_2`: Clears two words at a pointer retrieved from a table and, if a loop condition is met, subtracts eight from two different short values in a structure.
- `decrement_table_offsets`: Subtracts eight from two different short values located at specific offsets within a structure and a table.
- `copy_two_words`: Copies two consecutive words from a source pointer to a destination pointer.
- `reset_and_increment_index`: Sets a global variable to zero, modifies two memory locations by subtracting eight, and copies a longword from a table into two locations before incrementing a counter, or resets counters if a specific value is reached.
- `reset_status_flags`: Sets two global variables to specific constant values.
- `reset_global_structures`: If a global variable is negative, it clears several global variables and performs a loop to zero out specific offsets within a table.
- `initialize_data_table`: Sets a specific value in a pointer and then executes a loop to clear specific byte offsets within a table.
- `update_position_values`: Calculates new values by adding offsets from a table to a pointer-based structure and increments a counter until a limit is reached.
- `set_structure_byte_2`: Checks specific bits in a byte at a pointer offset and, if certain conditions are met, sets a byte at a different offset to one.
- `set_byte_constant`: Sets a byte at a specific offset within a structure to a constant value.
- `return_uninitialized_register`: The routine is empty and returns the current value held in a data register without performing any operations.
- `return_register_value`: The routine returns the current value held in a data register.
- `modify_word_alignment`: Adds and subtracts a value from two different memory locations and performs a bitwise AND operation to mask the lower nibble.
- `clear_two_words`: Sets two consecutive 16-bit values at a memory location pointed to by a register to zero.
- `copy_pointer_data`: Fetches a pointer from a table using an index and copies several words from that location into two different global structures.
- `check_first_condition`: Checks if a byte at a specific offset in a global structure equals one and if a global variable equals eleven, then clears two global variables and returns.
- `check_second_condition`: Checks if a byte at a specific offset in a global structure equals two and if a global variable equals eleven, then clears two global variables and returns.
- `update_status_logic`: Evaluates multiple bits in a global variable to conditionally reset other global variables or update pointers within a structure.
- `update_status_flags_2`: Evaluates multiple bits in a global variable to either reset several global variables, update a status value, or write a pointer to a global variable.
- `check_bit_recursion`: Checks specific bits in a global variable and either recurses or sets a global status value based on the bit state.
- `check_and_recurse_bit`: Checks if a specific bit is set in a global variable; if the bit is set, the routine calls itself recursively before returning.
- `return_immediately_4`: This routine performs no operations and returns immediately to its caller.
- `reset_global_values`: sets three global variables to zero or a constant value.
- `update_structure_pointers`: Updates a structure with a byte value and pointers to a global variable based on which bits are set in that global variable.
- `update_conditional_globals_3`: Updates a structure with a global variable pointer and specific values based on different bits being set in a global variable.
- `update_conditional_globals_2`: Checks bits in a global variable and updates two other global variables with specific values if those bits are set.
- `update_conditional_globals`: checks a bit in a global variable and, if set, updates two other global variables.
- `set_conditional_registers`: Checks various bits in a global variable and, depending on which bits are set, resets a counter or sets a register to a specific constant.
- `set_conditional_globals`: Sets various global status values or resets counters based on the state of specific bits in a global variable.
- `set_conditional_values`: Sets a global variable to zero and another to five, or sets a global variable to all ones, depending on which bit is set in a global variable.
- `set_global_limit`: Sets a global variable to a specific value if a bit in another global variable is set.
- `return_immediately_stub_2`: The routine consists of a single return instruction and performs no work.
- `set_register_constant`: Checks specific bits in a global variable and sets a register to a constant value if a specific bit is set.
- `check_global_bits`: Checks specific bits in a global variable and either calls another routine or sets a register to a constant value.
- `check_bit_and_set`: Sets a global variable to a specific value if a bit in a global variable is set.
- `return_immediately_stub`: The routine consists of a single instruction that returns to the caller, performing no work.
- `move_word_twice`: Loads a 16-bit word from an offset of 48 plus the value in d3 of the structure in a0 into register d0 twice.
- `copy_data_table`: Copies data from one table to another using a loop that performs multi-byte transfers at specific intervals.
- `copy_data_blocks_5`: Waits for a global variable to reach a specific character value before performing a multi-block memory copy from one table to another.
- `initialize_structure_fields_2`: Writes a global variable pointer and two constant byte values into a structure.
- `decrement_structure_byte`: Decrements a byte at a specific offset within a structure and returns if the resulting value is zero.
- `update_structure_pointers_2`: Modifies values within a structure, including setting bytes to constants and updating a pointer to a global variable based on specific comparison conditions.
- `clear_structure_data`: Writes a global variable pointer and two constant bytes into a structure, then clears two bytes at a location pointed to by a structure member.
- `decrement_structure_byte_2`: Decrements a byte in a structure and, if it reaches zero, populates a structure with a global variable pointer and several constant values.
- `process_structure_input`: Increments a word in a structure, and if it exceeds a threshold, performs several writes to a structure and a separate memory location based on conditional checks.
- `process_status_condition`: Checks a value in a table and, if it matches a specific constant, updates a structure with a constant and modifies bytes in another structure.
- `decrement_and_initialize`: Decrements a byte in a structure and, if it falls below zero, resets the byte to a constant and performs several other routine calls.
- `compare_and_modify_byte`: Compares two values pointed to by registers and returns a byte with specific bits set based on the comparison results.
- `compare_and_return_byte`: Compares two values pointed to by registers and returns a byte with a specific bit set based on the comparison result.
- `increment_structure_word`: Clears two bytes in a structure and then increments a word at a specific offset based on the value of a byte in a different structure.
- `increment_conditional_word`: Increments a word at a specific offset within a structure if a bit in a byte at another offset is not set, provided a value is within range.
- `increment_offset_value_2`: Increments a word at a specific offset within a structure if a bit in a byte at another offset is not set.
- `adjust_word_value`: Increments or decrements a word at a pointer address by four depending on the value of a byte at a specific offset.
- `increment_and_check_word`: Increments a word at a pointer address and returns if the new value exceeds a specific threshold.
- `increment_and_decrement_fields`: Increments a word and decrements a byte at specific offsets within a structure, checking against a threshold.
- `check_word_threshold`: Compares a word at a pointer address against a threshold and returns if it is greater than that value.
- `search_and_populate_structure`: Iterates through a table to find a specific character, then populates a structure with multiple global variable pointers and constant values.
- `setup_data_structure`: Populates a structure with several pointers to a global variable and sets various byte and word values.
- `initialize_structure_conditionally`: Initializes a structure with a global pointer and several byte values if a specific byte at an offset is negative.
- `decrement_and_initialize_structure`: Decrements a byte in a structure and, if it reaches zero, initializes several fields within that structure and sets a pointer to a global variable.
- `initialize_structure`: writes a pointer and several byte values into a structure pointed to by a register.
- `modify_structure_state`: Checks specific byte offsets within a structure and, if certain conditions are met, performs a bitwise XOR on one byte and updates several other byte values within that structure.
- `set_structure_byte`: Sets a byte at a specific offset in a structure to one if two other byte offsets in that structure are both zero.
- `modify_structure_data`: Checks two byte offsets in a structure and, if both are zero, performs a bitwise XOR on one byte, sets another byte to a specific value, clears two other bytes, and updates a global variable.
- `update_structure_and_global`: Performs a bitwise XOR on a byte in a structure, sets several other bytes to constant values, and updates a global variable.
- `assign_global_pointers`: Assigns a global variable address to multiple offsets within a structure if a bitmask check passes and a condition is met.
- `search_memory_for_byte`: Iterates through a memory range using a pointer, checking if a byte at a specific offset equals a certain value, and stops if it is found or the loop counter reaches zero.
- `process_bitmask_accumulation`: Performs bitwise OR operations by reading multiple values from a source pointer and storing the combined result into a destination pointer.
- `loop_through_table_and_call`: Iterates through a table using a pointer, calling another routine and potentially updating the pointer based on a comparison result.
- `call_indexed_function`: Uses a byte from a structure to calculate an index into a table of pointers and calls the resulting function address.
- `loop_with_conditional_calls`: Iterates through a table and calls another routine if specific byte values at certain offsets do not match specific constants.
- `call_indirect_via_table`: uses a value from a structure to index into a table and calls the resulting function pointer.
- `iterate_through_list`: Iterates through a list of structures using a pointer, performing conditional subroutine calls and updating the pointer based on a fixed increment.
- `execute_indirect_function_call`: Retrieves a function pointer from a table using an index stored in a memory location and executes it.
- `compare_pointer_values`: Subtracts a value pointed to by one register from a value pointed to by another and returns if the difference falls within a specific range.
- `compare_offset_values`: Subtracts a 16-bit value at a specific offset in one structure from a 16-bit value at the same offset in another structure and returns if the difference is within a specific range.
- `calculate_offset_difference`: Subtracts a 16-bit value at a specific offset in one structure from a 16-bit value at the same offset in another and returns the result.
- `initialize_and_update_structures`: Initializes two structures by setting a pointer to a global variable and clearing byte offsets, then performs arithmetic on structure fields and a decimal counter.
- `initialize_structure_fields`: Calls another routine and then sets a pointer within a structure to a global variable, masks a value, and clears two byte offsets.
- `update_structure_counters`: If a byte at a specific offset is zero, it increments a 16-bit value at another offset, increments a 32-bit value at a third offset, and sets the original byte to one.
- `update_structure_and_pointer`: If a byte at a specific offset is zero, it increments a 16-bit value at an offset, sets a pointer in a structure to a calculated address based on a global variable, and sets a byte to one.
- `set_global_one`: If a specific byte in a structure is non-zero, it sets a global variable to one and calls three other routines.
- `set_global_zero`: If a specific byte in a structure is non-zero, it sets a global variable to zero and calls three other routines.
- `conditionally_execute_subroutines`: Checks a byte at an offset within a structure pointed to by a register.
- `set_global_two`: If a specific byte in a structure is non-zero, it sets a global variable to two and calls three other routines.
- `set_global_three`: If a specific byte in a structure is non-zero, it sets a global variable to three and calls three other routines.
- `execute_conditional_routines`: If a specific byte in a structure is non-zero, it calls two other routines.
- `update_multiple_structures`: Performs structure initialization and arithmetic, then conditionally sets two structure pointers to a global variable and updates a global variable to one.
- `check_structure_and_global`: Checks if two bits in a structure byte are set and if a byte in a global variable equals four, then calls another routine.
- `execute_table_functions`: Iterates through a memory range and calls functions retrieved from a table using an index found at a specific offset in each entry.
- `execute_table_callback`: Checks a condition via another routine and, if the result is zero, executes a function pointer retrieved from a table using an index.
- `initialize_structure_data_3`: Initializes a structure with bit-shifted values, constants, and pointers to a global variable based on a comparison of a global value.
- `populate_structure_fields_2`: Populates various fields of a structure with calculated values and pointers to a global variable if certain conditions are met.
- `check_table_bounds`: Compares two global values against specific thresholds and returns without action if the conditions are met.
- `dispatch_conditional_calls`: Performs a sequence of subroutine calls based on comparisons of two global values against specific thresholds.
- `execute_conditional_calls`: Performs a series of subroutine calls based on a comparison of a global value against a threshold.
- `dispatch_conditional_calls_alt`: Performs a sequence of subroutine calls based on comparisons of two global values against different thresholds than the previous routine.
- `execute_conditional_calls_alt`: Performs a series of subroutine calls based on a comparison of a global value against a different threshold.
- `verify_table_entry`: Checks if specific bytes at offsets within a global structure are non-zero and returns a boolean result.
- `initialize_structure_data`: Initializes a structure by storing a bit-shifted value, a constant, several pointers to a global variable, and three byte values.
- `execute_indirect_calls_2`: Performs a series of indirect function calls to various subroutines.
- `call_indirect_function`: Checks multiple byte values within a structure and calls a function pointer stored in that structure if conditions are met.
- `update_status_byte`: Checks specific conditions within a structure and performs a bitwise XOR on a byte at offset three if certain criteria are met.
- `xor_byte_value`: performs a bitwise XOR operation on a byte at a specific offset within a structure.
- `calculate_masked_value`: Sets a byte in a structure and returns a value derived from a short integer masked with a constant, or zero if specific conditions are met.
- `modify_structure_values`: Clears bits in a word and updates a structure with a pointer to a global variable and an incremented value.
- `update_structure_from_table`: Updates a structure by adding a value from a table and updating a pointer and an index based on a global variable.
- `set_conditional_byte`: Checks multiple conditions in a structure and, if met, stores a constant byte at a specific offset.
- `check_and_update_status`: Verifies byte values and bit flags within a structure and, if conditions are met, calls another routine to determine status flags.
- `compute_status_flags`: Compares values in a structure against a global table and returns a byte representing combined status flags.
- `populate_structure_data`: writes multiple pointers, a word, and several byte or word values into a structure.
- `update_table_entry`: Stores a pointer to a global variable at an offset within a structure, where the pointer is offset by a scaled value from a register, and writes two byte values to other offsets in that structure.
- `decrement_and_check_byte`: Decrements a byte at a specific offset and stores a constant if the resulting value reaches zero.
- `configure_structure_parameters`: Validates multiple fields in a structure and, if they match specific criteria, stores a pointer to a global variable and several constant bytes.
- `initialize_structure_from_status`: Checks structure flags and, if a condition is met, populates a new structure with values and pointers from a global variable.
- `reset_structure_fields`: Sets a pointer to a global variable within a structure, clears specific byte offsets, and performs a bitwise AND on a word.
- `initialize_specific_structure`: Checks structure flags and, if valid, assigns a global variable pointer and sets several byte values and a masked word.
- `toggle_structure_byte`: Compares two short integers and, if they match or a global variable is zero, performs a bitwise XOR on a byte within a structure.
- `toggle_flag_on_match`: Compares two short integers and, if they are equal, performs a bitwise XOR on a byte at a specific offset.
- `toggle_byte_value`: Performs a bitwise XOR operation on a byte located at a specific offset within a structure using a constant value.
- `set_structure_pointer`: Writes a pointer to a global variable into a structure and sets a byte value at a specific offset within that structure.
- `update_structure_values`: Checks if a value in a structure is within a specific range, then adds values from a table to that structure and increments a byte.
- `test_structure_flag_bit`: Reads a byte at offset 6 from the address in A0 and tests bit 1 of that byte, returning without any other effect.
- `clear_and_load_data`: Fills a memory range with zeros using a loop and then loads several longwords from a calculated offset into global variables.
- `invert_global_bits`: Performs a bitwise NOT operation on a global variable by XORing it with a constant value.
- `loop_four_times`: Executes a routine four times using a loop controlled by a byte counter.
- `configure_dma_transfer`: Updates a structure with a pointer and a value, then potentially resets several longword fields in a memory block with a specific pattern if a condition is met.
- `write_word_indirect`: Writes a 16-bit word to a memory location pointed to by a value stored in a structure.
- `write_shifted_word`: Writes a 16-bit word to a memory location, where the value is derived from the lower byte of a register shifted left by two bits.
- `perform_recursive_calls`: This routine calls itself twice before returning.
- `update_sprite_animation_state`: Manages sprite animation frames and cycle counters by decrementing values in a data structure and updating memory locations with specific byte patterns.
- `update_entity_state_data`: Updates a series of data fields within an entity structure using offsets and values derived from a pointer.
- `process_linked_animation_frames`: Iterates through a linked list of animation data structures to update frame indices and coordinate offsets.
- `update_sprite_animation_state_2`: Decrements a frame counter and, if non-zero, updates sprite animation frame indices and pointer offsets within a data structure.
- `update_entity_state_and_animation`: Updates an entity's state by modifying its animation frame, cycle counters, and coordinate data through a pointer-based structure.
- `increment_and_copy_byte`: Increments a byte at a calculated memory location and copies a word from a table into a global variable.
- `process_animation_sequence_list`: Iterates through a linked list of animation data to update sprite frame indices and cycle counters.
- `update_sprite_animation_state_3`: Updates sprite animation frame indices and cycle counters by reading from a data structure and writing values to memory locations.
- `update_entity_state_data_2`: Updates an entity's state by modifying various offsets within a data structure based on input values and a lookup table.
- `update_entity_attribute_data`: Updates a data structure by extracting nibbles and bytes from a source buffer to populate various attribute fields.
- `update_sprite_animation_state_4`: Updates sprite animation frames and cycle counters by decrementing values in a data structure and updating memory locations with specific byte values.
- `update_entity_state_data_3`: Updates an entity's data structure by copying values from a source, modifying specific byte and word offsets, and performing conditional pointer updates.
- `update_sprite_animation_state_5`: Updates sprite animation frame indices and cycle counters by reading from a data structure and applying offsets from a global table.
- `update_entity_state_attributes`: Copies a block of data from a source pointer into an offset within an entity structure.
- `update_entity_state_data_4`: Updates an entity's state by calculating new values from a table and shifting data between various offsets within a structure.
- `update_sprite_attribute_buffer`: Iterates through a data stream to update sprite property structures, including frame indices, animation offsets, and coordinate data.
- `update_sprite_frame_data`: Updates sprite animation properties by reading values from a source structure and writing them to a target data structure using offsets.
- `update_sprite_frame_data_2`: Updates sprite animation frame indices and calculates new memory offsets by adding values from a lookup table to a base value.
- `update_entity_state_and_animation_2`: Updates an entity's state by modifying various offsets within a data structure based on input values and a lookup table.
- `update_entity_attribute_state`: Updates an entity's state by reading values from a source buffer and writing them to a structured data block.
- `update_entity_state_and_attributes`: Updates an entity's internal state by modifying various byte and word values within a data structure based on conditional checks.
- `initialize_data_structure`: If a condition is met, it copies a byte, clears two bytes, and fills a series of longwords at a pointer with specific patterns or zeros.
- `update_pointer_and_bitmask`: Stores a register value into a memory location calculated from a pointer and an offset, then performs a bitwise OR operation on a byte at a specific memory address.
- `process_conditional_recursion`: Checks a byte in a structure and performs multiple recursive calls if certain conditions are met.
- `check_status_flag`: Checks a byte in a global structure and returns if the byte is zero.
- `calculate_offset_index`: Sets a byte in a structure, calculates a masked index based on a short value, and returns it if certain bit conditions are met.
- `increment_table_indices`: If certain byte conditions are met within a structure, it masks bits of two fields and increments two other fields by four.
- `update_flag_byte`: Updates a global variable and, if a specific byte in a structure is non-zero, copies one byte to another position within that structure.
- `copy_and_call_function`: Copies a longword from one offset to another within a structure based on a byte value, then executes a function pointer.
- `xor_status_byte_3`: Checks specific conditions in a structure and performs an XOR operation on a byte within that structure using the value three.
- `xor_status_byte_2`: Performs an XOR operation on a byte in a structure using the value three, depending on whether a specific field matches a value or a bit in another structure is set.
- `xor_status_byte`: Selects a byte from one of two locations based on a condition and performs an XOR operation on a byte in a structure.
- `process_offset_decrement`: Adds a value from a global table to a structure field based on an offset, then decrements that offset.
- `modify_table_indices`: Stores a pointer to a global variable at two different offsets in a structure and updates two consecutive entries in that structure using a constant increment.
- `conditional_store_byte`: Checks multiple conditions involving a pointer and a global variable, then stores a constant byte into a structure if they are satisfied.
- `process_complex_state`: Evaluates several conditions and, if met, updates multiple structures with values from a global variable and performs several multi-word assignments.
- `update_structure_counters_2`: Decrements short values within a structure and returns a calculated index based on bitwise operations and comparisons.
- `update_counter_values`: Decrements two short values within a structure or updates a global structure based on bitwise checks and comparisons.
- `set_constant_bytes`: Sets a byte in a global structure to thirty-one if a bit is set in another structure, or sets a byte to thirty-five if a comparison between two short values fails.
- `set_constant_byte`: Stores the value thirty-one into a byte at a fixed offset from a global pointer.
- `set_different_constant_byte`: Stores the value thirty-five into a byte at a fixed offset from a global pointer.
- `synchronize_structure_offsets`: Compares short values between two structures and increments them or resets specific bytes if they do not match.
- `reset_structure_offsets`: Increments two short values in a structure if they differ from another structure, otherwise clears two bytes and sets a third byte to a constant.
- `reset_structure_fields_3`: Calls another routine twice and then clears two bytes and sets a third byte in a global structure to a constant value.
- `update_pointer_and_offsets`: Updates a structure with a pointer to a global variable and adjusts short values based on bitwise checks and comparisons.
- `update_pointer_table_2`: Decrements a short value, synchronizes two offsets, and assigns a pointer to a global table based on a calculated index.
- `compare_short_values`: Compares a short value in a structure against a short value in a global structure and returns if the first is smaller.
- `update_structure_state`: Checks a bit in a structure field to either write constant values and call itself recursively, or decrement a short value, update a secondary offset, and assign a global pointer.
- `decrement_and_store_offset`: Decrements a 16-bit value at an offset within a structure and then stores the resulting value at a different offset within the same structure.
- `decrement_structure_field`: Checks two byte offsets in a structure and either decrements a byte at a pointer location or clears two bytes within the structure.
- `clear_structure_bytes_3`: Writes zero to two specific byte offsets within a structure.
- `initialize_structure_data_2`: Checks two byte offsets in a structure and, if both are zero, initializes several fields and pointers within that structure using a global variable.
- `update_status_structure`: Sets several pointers in a structure to a global variable and populates fields with calculated values based on a bitmask of that global variable.
- `update_entity_coordinates`: Validates a structure's coordinate bounds before applying incremental changes to its primary value, pointer, and secondary offset.
- `update_entity_state_and_offset`: Validates a structure's coordinate bounds before applying incremental changes to its primary value, pointer, and secondary offset.
- `modify_structure_state_2`: Adjusts a short value and a pointer within a structure based on bit flags and performs a table-based calculation.
- `process_pointer_table`: Modifies a structure based on bit flags, potentially adjusting a pointer and a short value, and returns a value derived from a table lookup.
- `modify_pointer_table`: Adjusts a value in a structure based on a bit flag, modifies a byte in the structure, and potentially returns a value derived from a pointer and a calculation.
- `process_conditional_pointer`: Updates a structure's pointer and short value, potentially calling a function pointer based on a comparison with a global value.
- `update_offset_pointer`: Compares a short value against a global, conditionally executes a function pointer, and updates a pointer in the structure using a calculated offset from a global variable.
- `set_flag_byte`: Checks if a specific byte in a structure is zero and, if so, sets that byte to one.
- `decrement_structure_counter`: Copies a 32-bit value to another offset within a structure and decrements a short value until it reaches a specific threshold.
- `populate_structure_fields`: Checks a byte in a global structure and, if zero, populates multiple fields in a structure with calculated values and pointers to a global variable.
- `execute_chained_structure_updates`: This routine performs a sequence of four consecutive calls to the same function.
- `check_structure_condition`: Sets a byte in a structure to one and returns a calculated value if a specific condition involving a short value and a bitmask is met.
- `increment_structure_field`: If a byte in the structure is non-zero, it increments a short value, updates a secondary offset, and assigns a global pointer to two different structure offsets.
- `modify_structure_data_2`: If a byte in the structure is zero, it copies values between offsets, adds a constant to a byte, and conditionally increments a short value.
- `increment_structure_offset`: Checks two conditions in a structure and, if met, increments a short value at a specific offset if a global variable is below a threshold.
- `initialize_structure_fields_3`: Calls another routine, assigns a global pointer to a structure offset, and clears several bytes within the structure.
- `check_table_bounds_2`: Compares short values in two global structures and checks a character at an offset within a structure.
- `verify_structure_equality`: Checks if two short values in global structures are equal and verifies a character at a specific offset.
- `initialize_table_entry`: Searches for a specific byte in a structure and, if found, initializes several fields and pointers using a global table.
- `apply_table_offsets`: Calculates a pointer to a global table, adds values from that table to the structure, and increments a byte offset.
- `add_offset_values`: Adds two word values from a table to a destination and increments a byte at a specific offset.
- `accumulate_table_values`: Iteratively adds short values from a global table to a structure and increments an index byte until a threshold is met.
- `increment_short_value`: Increments a short value within a structure and calls itself recursively if the value exceeds a specific threshold.
- `add_offset_values_2`: Adds two short values from a table to a structure and increments a byte at a specific offset.
- `store_pointer_sequence_2`: Retrieves a value from a table using an index and an offset, then stores that value and its increments into several locations in a structure.
- `store_pointer_sequence`: Stores a series of incremented pointer values into a structure and increments a byte at a specific offset.
- `call_routine_recursively`: Calls another routine without performing any other operations.
- `execute_recursive_call_2`: Calls another routine without performing any additional operations before returning.
- `check_structure_byte`: Compares a byte at a pointer-derived offset within a structure against zero and executes another routine multiple times if the condition is met.
- `decrement_and_return`: Tests a byte at offset 11 of the structure in a0; if the byte is not zero, it decrements that byte by one and returns, otherwise it branches to another routine.
- `execute_recursive_calls`: This routine performs a series of recursive calls to itself before returning.
- `update_structure_with_callback`: If a byte at a specific offset is zero, it modifies a short value in the structure and calls a function pointer selected from a table.
- `assign_global_pointer`: Checks two conditions in a structure and, if met, assigns a global variable to a pointer in the structure and sets a byte value.
- `compare_and_reset_structure`: Compares a structure against a global table and, if they match, resets several fields in the table and updates the structure with a global variable and new values.
- `increment_structure_field_2`: Sets a byte to zero and, if a specific bit in a bitfield is not set, increments a short value and a byte field.
- `update_object_state_and_timer`: Decrements a byte at a specific offset within a structure and, if it reaches a threshold, updates a pointer to a global variable, increments a short value, and performs conditional data initialization.
- `initialize_object_data_2`: Decrements a byte at an offset, and if it reaches a specific value, assigns a global pointer, increments a short value, and populates several fields with constants.
- `initialize_object_data`: Increments a short value in a structure and, if it remains below a threshold, populates several fields with constant values and a pointer to a global variable.
- `update_table_entries`: Iterates through a table to update multiple fields in each entry based on a global variable and a subtraction operation.
- `handle_recursive_decrement`: Decrements a byte in a structure through multiple recursive calls if a specific pointer matches a global address.
- `execute_recursive_calls_2`: This routine calls itself repeatedly in a recursive sequence before returning.
- `increment_structure_value`: Increments a word in a structure if specific byte conditions are met and a bit in a related structure is set.
- `decrement_structure_value`: Decrements a word in a structure if specific byte conditions are met and a bit in a related structure is set.
- `xor_byte_value_2`: Performs a bitwise XOR operation on a byte at a specific offset within a structure.
- `increment_offset_value_3`: Increments a word and sets a byte within a structure if several boundary and bitwise conditions are satisfied.
- `apply_table_offset`: Modifies a structure by adding or subtracting values based on a byte field and a value retrieved from a global table.
- `set_status_byte`: Sets a byte at a specific offset within a structure if a global variable is less than six.
- `copy_global_data`: Copies four longwords from a global variable into a structure if several byte conditions and a bitmask check are met.
- `copy_global_pointers`: Copies four longwords from a global variable into a structure if a specific byte condition is met.
- `synchronize_structure_fields`: Updates multiple offsets within a structure by copying values from other locations and adding constant offsets to specific words.
- `clear_structure_bytes_2`: Sets four specific byte offsets within a structure to zero if a word within that structure is outside a specific range.
- `clear_structure_bytes`: Sets four specific byte offsets within a structure to zero.
- `process_subtraction_and_reset`: Subtracts a value from a byte in a structure and, if the result is negative, initializes a large structure with various constants and function pointers.
- `execute_sequence_of_callbacks`: Repeatedly calls another routine four times in succession.
- `execute_conditional_callback`: Checks if two specific bytes in a structure are zero and then calls a function pointer determined by a bitmask in another field.
- `update_field_with_limit`: Resets a byte field and, if a bit in a bitfield is clear, increments a short value in the structure provided it does not exceed a threshold.
- `update_structure_and_callback`: Either sets a byte based on a global variable or adds a value from a table to a structure field and executes a function pointer.
- `set_status_byte_alt`: Sets a byte at a specific offset within a structure if a global variable is less than eight.
- `check_status_byte`: Checks if a byte at a specific offset within a structure is non-zero.
- `call_sequential_subroutines`: Calls three consecutive subroutines in sequence before returning.
- `patch_memory_pattern_2`: Searches for a specific byte pattern in memory and replaces occurrences with two specific byte values.
- `patch_memory_pattern_alt`: Iterates through memory to find a specific byte sequence and overwrites it with different constant values.
- `patch_memory_pattern`: Calculates a memory address using offsets from a structure and a table, then iterates through a sequence of bytes to replace specific values with a two-byte pattern.
- `process_memory_pattern`: Calculates offsets from a table to populate a structure and iterates through memory to write specific byte patterns based on a search condition.
- `fill_memory_pattern`: Iterates through a memory range, writing two specific bytes repeatedly as long as a byte at the current pointer matches a specific value.
- `copy_data_blocks_4`: Copies large blocks of data from one table to another using a loop, then performs calculations to update a global pointer and a byte in a structure.
- `update_pointer_table`: Calculates an offset based on a byte value, fetches a longword from a table, and populates a large sequence of offsets in a structure.
- `update_pointer_list`: Fetches a longword from a pointer-based table and stores it into several offsets within a global structure while incrementing a counter.
- `initialize_pointer_list`: Stores a series of incremented pointer values into various offsets within a structure and increments a byte at a specific offset.
- `compare_global_pointer`: Compares a register value against a global variable.
- `compare_global_pointer_alt`: Compares a register value against a different global variable.
- `update_offset_pointers`: Fetches a longword from a table using an index, stores it and subsequent incremented values into a structure, and increments a byte.
- `store_partial_pointers`: Stores three incremented pointer values into a structure and increments a byte at a specific offset.
- `check_byte_threshold`: Compares a byte at a specific offset within a global data structure against a constant value.
- `call_structure_modifier`: Calls a routine that modifies a data structure based on specific byte offset conditions.
- `check_byte_thresholds`: Compares two bytes located at specific offsets within two different global variables against a constant value.
- `decrement_and_recurse_on_byte`: Checks if a byte at a specific offset from a pointer is non-zero; if so, it decrements that byte.
- `execute_recursive_calls_4`: Performs a series of recursive calls to itself.
- `execute_recursive_call_3`: Checks a byte at an offset within a structure and performs a recursive call to itself.
- `execute_recursive_call`: This routine performs a direct recursive call to itself before returning.
- `update_table_entry_2`: Checks a table entry against a global variable and a specific value, then updates several fields in the table with a pointer to a global variable and other constant values.
- `decrement_and_reset_structure`: Decrements a byte within a structure and, if it reaches zero, resets several fields and decrements a global variable.
- `reset_structure_fields_2`: Sets specific bytes within a structure to zero or fixed values and decrements a global variable.
- `populate_table_from_global`: Iterates through a table to check for zero bytes, then copies data from a global variable into the table with specific byte assignments.
- `execute_recursive_calls_3`: Compares a value in a register to a specific address and, if they match, performs a series of seven consecutive calls to another routine.
- `modify_short_value`: Adds a signed value of either two or negative two to a short integer based on the contents of a byte at a specific offset.
- `toggle_status_bits`: Performs a bitwise XOR on a byte at a specific offset if a signed value meets certain threshold criteria.
- `toggle_byte_bits`: Performs a bitwise XOR on a byte at a specific offset if the value pointed to by a register exceeds a certain threshold.
- `decrement_short_value`: Decrements a short integer by two if a byte at a specific offset is zero and the short integer itself is non-zero.
- `update_callback_offsets`: Updates several function pointers and integer values within a structure based on a control byte, or modifies a byte based on a global variable.
- `store_masked_value`: Checks specific bits in a structure and, if conditions are met, stores a masked version of a global variable into a byte at a specific offset.
- `update_structure_pointer`: Checks a condition based on a pointer and a byte value, then updates a structure with values from another table and a routine pointer.
- `increment_structure_word_2`: Checks if a pointer within a structure is non-zero and if a specific bit is set, then increments a word at a fixed offset within the structure if it remains below a certain threshold.
- `increment_structure_field_3`: Increments a word at a specific offset within a structure and recursively calls itself if the value exceeds a threshold.
- `process_subtraction_and_copy`: Subtracts a value from a byte in a structure and, if the result is negative, performs a loop to copy data from a global variable into a table.
- `process_global_state`: Decrements a global variable or performs conditional logic and recursive calls based on various bitwise checks and table values.
- `check_table_elements`: Iterates through a table for a fixed number of steps, checking if a byte at a specific offset is zero.
- `copy_table_to_structure`: Copies data from a table into a structure, applying conditional arithmetic and setting specific byte values based on a global variable.
- `reset_structure_fields_4`: Decrements a byte at a specific offset if it is non-zero, or resets several fields in a structure to zero and sets a pointer to a global variable.
- `initialize_structure_on_bit`: Checks a specific bit in a structure and, if set, initializes a structure with a pointer to a global variable and several zeroed byte fields.
- `initialize_structure_on_bit_2`: Checks a different bit in a structure and, if set, initializes a structure with a pointer to a global variable and several zeroed byte fields.
- `initialize_structure_on_flag`: Checks a bit in a byte, and if set, initializes a structure with a pointer to a global variable and clears several offsets.
- `initialize_structure_on_bit_3`: Checks a third bit in a structure and, if set, initializes a structure with a pointer to a global variable and several zeroed byte fields.
- `initialize_structure_2`: Sets a pointer to a global variable and clears several byte offsets within a structure.
- `transfer_table_data`: Calls a routine to copy multiple values from a source table into two separate destination memory areas.
- `call_data_copy_routine`: Calls a routine that copies multiple values from a source table into two separate destination memory areas.
- `subtract_from_pointer_target`: Subtracts a byte value, retrieved from a table using an offset, from a byte located at a pointer-derived address.
- `advance_sequence_cursor`: Reads the byte at a pointer-derived position and steps a shared index forward, with a wrap-around at its limit that resets it and advances a secondary index.
- `parse_instruction_table`: Iterates through a table of pointers to fetch and decode instruction bytes.
- `shift_and_store_values`: Shifts two values left by four bits and stores them into a table pointed to by a register.
- `initialize_instruction_dispatch_table`: Clears several global variables and populates a dispatch table with function pointers.
- `populate_dispatch_table_2`: Iteratively writes a sequence of function pointers into a memory location.
- `call_subroutines_recursively`: Calls itself twice before returning.
- `perform_bcd_update_two`: Performs BCD arithmetic on global variables and sets two specific global variables to two.
- `perform_bcd_update_three`: Performs BCD arithmetic on global variables and sets two specific global variables to three.
- `call_subroutines_and_return`: Calls two different subroutines in succession before returning.
- `loop_until_zero`: Repeatedly calls another routine in a loop until it returns zero, then sets a global variable to one.
- `register_callback_pointers`: This routine populates two different data tables by writing a sequence of values and a function pointer into them.
- `increment_and_clear_memory`: Increments a global variable and clears memory blocks based on a terminator character.
- `add_bcd_value`: Adds a 32-bit value to a multi-byte decimal counter using binary coded decimal adjustments.
- `decrement_and_calculate_bcd`: Decrements a global variable and, if it reaches zero, performs BCD arithmetic using values from two other memory locations to update a multi-byte global variable.
- `double_global_value`: Doubles the value of a global variable using addition and a conditional increment, then performs a BCD adjustment.
- `subtract_bcd_value`: Subtracts a byte from a decimal value stored in a global variable and applies a decimal adjustment.
- `process_bcd_data`: Performs BCD arithmetic on global variables and enters an infinite loop after updating a value.
- `call_library_function`: Loads a function pointer from an offset within a library structure and executes it.
- `load_various_files`: Performs multiple file operations including reading sprite data, panel data, and complex data through indirect function calls.
- `clear_and_initialize_globals`: Clears a memory range with zeros and then calls other routines while setting two global variables to zero.
- `set_globals_to_one`: Sets two specific global variables to one and calls another routine.
- `set_globals_to_two`: Sets two specific global variables to two and calls another routine.
- `set_globals_to_three`: Sets two specific global variables to three and calls another routine.
- `set_multiple_globals`: Sets two specific global variables to four and updates two other global variables with the values three and one.
- `execute_indirect_file_routines`: Executes several routines that involve reading data from files and performing indirect function calls via a table.
- `execute_indirect_file_operations`: Executes a function via a table offset and performs file reading and closing operations.
- `execute_indirect_calls_3`: Calls another routine, executes a function pointer retrieved from a table via an offset, reads data from a file, and closes a file handle.
- `execute_indirect_calls_4`: Calls another routine and then executes a function pointer retrieved from a table via an offset.
- `load_sprite_data`: Reads a sprite file into memory and executes a function through a table pointer.
- `load_large_sprite`: Reads data from a specific sprite file and calls an indirect function.
- `load_level_sprite`: Reads a sprite file associated with a level and executes a function from a table.
- `read_and_execute_file`: Reads data from a file and performs an indirect function call using a table offset.
- `load_panel_data`: Reads data from a panel file and executes a function via an indirect pointer.
- `load_complex_data`: Reads a file and executes a series of nested loops and indirect function calls to process the data.
- `read_file_indirect`: Reads data from a file and executes a function retrieved from a table.
- `execute_indirect_function`: Calls another routine and then executes a function pointer retrieved from a table via an offset.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `close_file_handle`: Calls a library function to close a file handle.
- `blit_sprite_tile_data`: Copies tile or sprite data from a source table to multiple destination memory buffers using bitwise OR or zeroing operations.
- `apply_sprite_tile_mask`: Iterates through a tile data table to perform bitwise OR or zeroing operations on specific memory buffers.
- `update_sprite_attribute_table`: Iterates through a data structure to perform bitwise OR or assignment operations on memory locations.
- `blit_tile_pattern_to_bitplanes`: Copies a tile pattern from a data table into multiple bitplanes by performing bitwise OR or zeroing operations.
- `update_sprite_attribute_table_2`: Iterates through a data stream to modify sprite or object attribute tables by performing bitwise OR or zeroing operations.
- `apply_sprite_tile_mask_2`: Iterates through a pattern table to perform bitwise OR or zeroing operations on memory buffers.
- `apply_sprite_attribute_mask`: Iterates through a data table to perform bitwise OR or clear operations on specific memory locations.
- `update_sprite_attribute_table_3`: Iterates through a data stream to update sprite or tile attributes by performing bitwise OR or zeroing operations on memory buffers.
- `blit_sprite_tile_data_2`: Processes a command stream to perform bitwise OR or zeroing operations on memory buffers.
- `apply_sprite_tile_mask_3`: Iterates through a small data block to perform bitwise OR or zeroing operations on multiple memory buffers.
- `blit_sprite_tile_data_3`: Copies tile or sprite data from a source table to multiple destination memory buffers using bitwise OR or replacement operations.
- `apply_sprite_tile_mask_4`: Iterates through a tile data table to perform bitwise OR or zeroing operations on specific memory buffers.
- `apply_sprite_tile_mask_5`: Iterates through a source data table to perform bitwise OR or zeroing operations on two destination memory buffers.
- `apply_tile_pattern_mask`: Reads pattern data from a table and applies it to multiple memory buffers using bitwise OR or zeroing operations based on a bitmask.
- `apply_sprite_tile_mask_6`: Reads tile data from a table and applies it to multiple memory buffers using bitwise OR or zeroing operations based on a mask.
- `apply_sprite_tile_mask_7`: Reads tile data from a table and applies it to multiple memory buffers using bitwise OR or zeroing operations based on a bitmask.
- `tiny_ori_3`: Small routine of 1 instruction that merges 32 into d2.
- `tiny_ori_4`: Small routine of 1 instruction that merges 128 into d2.
- `tiny_ori_5`: Small routine of 1 instruction that merges 224 into d2.
- `tiny_ori_6`: Small routine of 1 instruction that merges 64 into d2.
- `tiny_ori_7`: Small routine of 1 instruction that merges 160 into d2.
- `tiny_ori_8`: Small routine of 1 instruction that merges 0 into d2.
- `tiny_ori_9`: Small routine of 1 instruction that merges 96 into d2.
- `tiny_ori_10`: Small routine of 1 instruction that merges 192 into d2.
- `process_command_stream`: Parses a sequence of commands from a data stream to perform bitwise OR operations or clear values across multiple memory buffers.
- `blit_sprite_tile_data_4`: Copies bitmask data from a source table to multiple destination memory buffers using bitwise OR or zeroing operations.
- `blit_sprite_tile_data_5`: Copies sprite or tile data from a source table to multiple memory locations using bitwise OR or direct assignment.
- `blit_sprite_tile_data_6`: Copies tile or sprite data from a source table to two destination memory buffers using bitwise OR or zeroing operations based on a control byte.
- `blit_sprite_tile_data_7`: Copies sprite or tile data from a source table to multiple destination memory buffers using bitwise OR or direct assignment.
- `blit_sprite_tile_data_8`: Copies pixel data from a source table to multiple destination memory buffers using bitwise OR or zeroing operations.
- `blit_sprite_tile_data_9`: Copies sprite or tile data from a source table to multiple destination memory buffers using bitwise OR or clearing operations based on a control byte.
- `blit_sprite_tile_data_10`: Copies sprite or tile data from a source table to multiple destination memory buffers using bitwise OR or zeroing operations based on a control byte.
- `apply_sprite_tile_mask_8`: Processes a sequence of command bytes to perform bitwise OR or zeroing operations on memory buffers.
- `update_sprite_attribute_table_4`: Reads sprite data from a table and applies bitwise OR or zeroing operations to multiple memory buffers using fixed offsets.
- `blit_sprite_tile_data_11`: Copies sprite or tile pixel data into multiple memory planes by performing bitwise OR or clearing operations based on a control byte.
- `blit_sprite_tile_data_12`: Copies sprite or tile data from a source table to multiple destination memory buffers using bitwise OR or clearing operations based on a control byte.
- `update_sprite_bitmaps`: Iterates through a data table to perform bitwise OR or clearing operations on memory buffers.
- `apply_sprite_bitmask`: Iterates through a bitmask table to perform bitwise OR or AND operations on memory buffers.
- `blit_sprite_tile_data_13`: Copies pixel data from a source table to multiple destination memory buffers using bitwise OR or zeroing operations.
- `blit_sprite_tile_data_14`: Copies pixel data from a source table to multiple destination memory buffers using bitwise OR or zeroing operations based on a control byte.
- `blit_sprite_tile_data_15`: Copies tile or sprite data from a source table to multiple memory locations using bitwise OR or zeroing operations.
- `blit_tile_pattern`: Copies a pattern from a source table to multiple memory destinations using bitwise OR or zeroing operations.
- `blit_sprite_tile_data_16`: Copies sprite or tile data from a source table to multiple destination memory buffers using bitwise OR operations or clearing based on a control byte.
- `apply_sprite_tile_mask_9`: Parses a command stream to perform bitwise OR or clear operations on memory blocks using a lookup table.
- `blit_sprite_to_multiple_buffers`: Copies or bitwise-ORs sprite pixel data into multiple destination memory buffers simultaneously.
- `update_bitmapped_graphics_buffer`: Iterates through a source data table to perform bitwise OR or assignment operations on multiple destination memory buffers.
- `return_immediately_5`: The routine performs no operations and returns.
- `store_bytes_and_loop_calls`: Splits a word into two bytes and stores them in consecutive global variables, then enters a loop that repeatedly calls itself and increments one of those global variables until a specific byte in a memory range is no longer a terminator.
- `iterate_and_clear_memory`: Increments a value in a loop and calls routines to clear or fill memory blocks based on a terminator character.
- `calculate_table_offset_value`: Calculates a value by adding a constant to an entry retrieved from a table, where the table index is determined by a shifted value from another routine.
- `convert_hex_digit`: Converts a character to its numeric value or returns a constant value based on specific conditions.
- `convert_hex_digit_to_integer`: This routine converts a single character from a hexadecimal string into its corresponding numeric value.
- `fill_memory_blocks`: The routine copies a byte from a source to multiple destination locations at fixed offsets.
- `copy_data_blocks_3`: Copies multiple blocks of data from a source table to a destination table using a loop.
- `fill_data_table`: Stores a series of values and a pointer into a table at various offsets.
- `fill_data_table_2`: Writes a sequence of values and a routine pointer into a memory table.
- `fill_data_table_3`: Populates a table with a set of values and a routine address at specific intervals.
- `fill_data_table_4`: Copies a set of values and a routine pointer into a large data structure.
- `swap_global_pointers`: Swaps the contents of two global variables.
- `copy_word_block`: Copies eight words from a source location to a destination location.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `update_data_structure`: Writes several multi-byte values into a destination table at specific offsets.
- `shift_wide_value_left`: Shifts a 32-bit value held in two words left by one bit, feeding in a bit from a status byte and returning the carried-out byte.
- `loop_and_write_constants`: Executes a series of nested loops that call other routines and perform conditional checks.
- `decrement_nibble_value`: Decrements a value pointed to by a register based on the position of its highest non-zero nibble.
- `increment_nibble_loop`: Executes a loop that repeatedly calls another routine based on register values.
- `increment_nibble_value`: Increments a value pointed to by a register based on the position of its highest non-zero nibble.
- `set_blt_registers`: Stores several constant values into hardware registers used for bitplane manipulation.
- `setup_dma_transfer`: Configures DMA registers and performs a loop to transfer data using a specified count.
- `execute_blt_transfer`: Configures Blitter registers and performs a loop to transfer data using a specified count.
- `wait_for_bit`: Performs a busy loop that waits until a specific bit in a global variable is cleared.
- `clear_memory_block`: Clears a range of memory by writing zero to a destination specified by a pointer.

### Bionic Commando (1987)(GO!).adf:tune5.dat

- `initialize_program_execution`: This routine serves as the initial entry point for the file execution.

### Bionic Commando (1987)(GO!).adf:tune3.dat

- `negate_byte_at_offset`: Calculates an address by adding an offset to a register, negates the byte stored at that location, and performs redundant reads from a memory address.

### Bionic Commando (1987)(GO!).adf:tune1.dat

- `negate_byte_at_offset_2`: Calculates an address using a register and an offset, negates the byte at that location, and performs two redundant reads from a pointer.

### Bionic Commando (1987)(GO!).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
