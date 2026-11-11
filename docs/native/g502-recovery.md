# Game 502 (Gobliins 2 - the Prince Buffoon): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00502). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2703872 top-level bytes in 43 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 14 of 14 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 799 of 799 named (1 author symbols), 799 of 799 described (67 by rule, 719 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 18395 of 18395 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 554014 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 31392 of 31392 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 242115 |
| code | 110308 |
| data | 2199596 |
| padding | 8493 |
| unused | 143360 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Gobliins 2 - The Prince Buffoon (1992)(Coktel Vision)[cr SKR](Disk 1 of 3).adf:stack | 2 | 117 | 2 |
| Gobliins 2 - The Prince Buffoon (1992)(Coktel Vision)[cr SKR](Disk 1 of 3).adf:SkidRow | 1 | 12 | 2 |
| Disk-Validator | 1 | 9 | 1 |
| skidrow.exe | 2 | 24 | 4 |
| Gobliins 2 - The Prince Buffoon (1992)(Coktel Vision)[cr SKR](Disk 1 of 3).adf:leftone | 2 | 3 | 1 |
| Gobliins 2 - The Prince Buffoon (1992)(Coktel Vision)[cr SKR](Disk 1 of 3).adf:mailman | 2 | 24 | 4 |
| Gobliins 2 - The Prince Buffoon (1992)(Coktel Vision)[cr SKR](Disk 1 of 3).adf:loader | 3 | 30977 | 727 |
| Gobliins 2 - The Prince Buffoon (1992)(Coktel Vision)[cr SKR](Disk 1 of 3).adf:bootblock | 1 | 10 | 1 |
| Gobliins 2 - The Prince Buffoon (1992)(Coktel Vision)[cr SKR](Disk 2 of 3).adf:bootblock | 1 | 83 | 5 |
| Gobliins 2 - The Prince Buffoon (1992)(Coktel Vision)[cr SKR](Disk 3 of 3).adf:bootblock | 1 | 83 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 47 | 45 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Gobliins 2 - The Prince Buffoon (1992)(Coktel Vision)[cr SKR](Disk 1 of 3).adf:INTRO.STK (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- gobliins 2 - the prince buffoon (1992)(coktel vision)[cr skr](disk 2 of 3)/Gobliins 2 - The Prince Buffoon (1992)(Coktel Vision)[cr SKR](Disk 2 of 3).adf:DISK2.STK: high-entropy data file (7.68 bits/byte), classified as packed data
- gobliins 2 - the prince buffoon (1992)(coktel vision)[cr skr](disk 3 of 3)/Gobliins 2 - The Prince Buffoon (1992)(Coktel Vision)[cr SKR](Disk 3 of 3).adf:DISK2.STK: high-entropy data file (7.68 bits/byte), classified as packed data
- runtime segment an address: 34 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- gobliins 2 - the prince buffoon (1992)(coktel vision)[cr skr](disk 1 of 3)/Gobliins 2 - The Prince Buffoon (1992)(Coktel Vision)[cr SKR](Disk 1 of 3).adf:INTRO.STK: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 554014 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- Gobliins 2 - The Prince Buffoon (1992)(Coktel Vision)[cr SKR](Disk 1 of 3).adf:INTRO.STK (raw code file, load address unknown)

## Copy protection found

- none found

## Functions


### Gobliins 2 - The Prince Buffoon (1992)(Coktel Vision)[cr SKR](Disk 1 of 3).adf:stack

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_memory_block`: Copies sixteen bytes from a source address to a destination address using a loop.

### Gobliins 2 - The Prince Buffoon (1992)(Coktel Vision)[cr SKR](Disk 1 of 3).adf:SkidRow

- `apply_memory_patch`: Copies a sequence of bytes from one memory location to another to patch code.
- `patch`: Writes a specific constant value to a memory location.

### Disk-Validator

- `xor_data_block`: Iterates through a block of memory, performing a bitwise XOR on each byte with a decreasing value.

### skidrow.exe

- `call_library_entry`: Invokes another routine and checks the zero flag.
- `open_dos_library`: Calls a function via a pointer in a global variable to open the DOS library and stores the resulting handle.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.

### Gobliins 2 - The Prince Buffoon (1992)(Coktel Vision)[cr SKR](Disk 1 of 3).adf:leftone

- `wait_for_disk_status`: Polls a global variable, which represents a hardware register, until a specific bit is cleared.

### Gobliins 2 - The Prince Buffoon (1992)(Coktel Vision)[cr SKR](Disk 1 of 3).adf:mailman

- `initialize_dos_output`: Opens the DOS library, checks the zero flag, and if not set, executes a function pointer from a global table before calling a final routine.
- `open_dos_library_2`: Calls the exec library function OldOpenLibrary to load dos.library.
- `close_library_handle`: Calls the exec library function CloseLibrary using a handle stored in a global variable.
- `call_dos_output`: Calls a function from the dos.library using a handle stored in a global variable.

### Gobliins 2 - The Prince Buffoon (1992)(Coktel Vision)[cr SKR](Disk 1 of 3).adf:loader

- `initialize_memory_and_execute_loader`: Clears a specific memory block with zeros and then performs conditional execution of subroutines stored in a global structure.
- `parse_asset_type_header`: Reads a byte from a data stream to determine an asset type and validates it against specific constraints.
- `parse_asset_stream`: Processes a data stream by reading type identifiers and extracting associated metadata or raw bytes into local buffers.
- `parse_asset_data_stream`: Validates asset type constraints and parses metadata from a data stream based on a type identifier.
- `process_asset_data_stream`: Parses asset metadata and branches into specific data handling routines based on the identified asset type.
- `parse_asset_header`: Reads a data stream to identify asset types and parse associated metadata.
- `parse_resource_metadata_block`: Reads a structured data block to extract coordinate offsets, dimensions, and attribute flags into a global data structure.
- `process_resource_stream_3`: Checks a status flag to initialize a data structure and then enters a loop that reads, validates, and clears segments of a memory buffer.
- `process_resource_stream_4`: Iterates through a data structure to clear memory segments and load values from a table into a buffer.
- `initialize_and_process_resource_buffer`: Initializes a data structure by clearing specific memory offsets and setting default values.
- `process_resource_stream_5`: Iterates through a data structure to extract and validate segments of information based on length and offset parameters.
- `initialize_resource_buffer`: Clears a series of memory offsets and initializes a data structure with default values.
- `process_resource_sub_elements`: Iterates through a structured data stream to extract and validate sub-resource properties such as dimensions and offsets.
- `validate_resource_bounds`: Recursively verifies that resource dimensions and coordinate offsets fall within defined boundary limits.
- `parse_resource_metadata_stream`: Recursively traverses a structured data stream to extract dimensions, offsets, and properties.
- `parse_resource_metadata`: Recursively parses a structured data stream to extract dimensions, offsets, and properties.
- `branch_to_resource_loader`: This routine performs a jump to a resource management routine using a relative address.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d7.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d6.
- `update_entity_spatial_coordinates`: Updates coordinate offsets and position values within a data structure based on specific threshold checks and conditional increments.
- `call_function_with_offsets`: Calls a function using two values retrieved from a table via offsets and conditionally performs coordinate subtractions based on a bitmask and specific table values.
- `call_function_with_short`: Calls a function using a single short value from a memory location and conditionally performs coordinate subtractions based on a bitmask and specific table values.
- `call_function_with_calculated_width`: Calls a function using a short value from a table, a short value, and a calculated width, then conditionally performs coordinate subtractions.
- `call_function_with_coordinates`: Calls a function using a short value from a table and two short values, followed by conditional coordinate subtractions based on a bitmask.
- `update_entity_position_and_bounds`: Calculates new coordinate offsets and bounding box dimensions based on conditional logic and table lookups.
- `process_sprite_tile_transform`: Iterates through a sequence of data to calculate scaled coordinate offsets and pixel positions based on values retrieved from a lookup table.
- `execute_callback_and_update_offsets`: Calls a function pointer multiple times with various parameters derived from a table and conditionally subtracts values from memory locations based on a flag.
- `update_coordinate_offsets`: Checks specific values in a table to conditionally call another routine or subtract values from various memory locations using offsets stored in a global variable.
- `process_conditional_data`: Executes a series of function calls based on values returned from a pointer, potentially updating a global integer and modifying a value in a table.
- `process_buffer_data`: Performs multiple function calls using pointers from a table, updates a global integer by adding a value, and calls another function with two arguments derived from a table and a local variable.
- `reset_global_counter`: Calls a function using a pointer from a table and then sets a global integer to zero.
- `populate_data_table`: Reads multiple sixteen-bit values from a source and stores them into a series of offsets within a data structure.
- `store_partial_values`: Reads a sixteen-bit value and stores it at one offset, then calls another routine to fetch and store three more values into a table.
- `fill_table_entries`: Fetches a sixteen-bit value and stores it, followed by several calls to another routine to populate additional consecutive table entries.
- `write_sequence_values`: Stores a sixteen-bit value at an offset and then sequentially calls another routine to write four more values into a table.
- `load_table_data`: Stores a sixteen-bit value and then performs four consecutive calls to another routine to fill remaining table slots.
- `store_function_result`: Calls a function to get a value, uses that value to offset a pointer to a table, calls another function to write data to that location, and then stores the result back into the table.
- `update_table_entry`: Retrieves a value from a table using an offset, then stores that value back into the same table at a location determined by another offset.
- `execute_sequential_calls`: Calls two different functions using pointers retrieved from a table.
- `assign_and_conditionally_call`: Stores the result of a function call into two consecutive global word locations and calls another function if a specific global byte is non-zero.
- `transfer_table_data`: Calls a function with two arguments, then calls another function using a pointer, a calculated offset, and a value from a table.
- `call_indirect_functions`: Calls three different functions via pointers stored in a table and passes a calculated value and a combined word to a fourth function.
- `update_table_entry_2`: Calls two functions to retrieve values, uses a third function to calculate a result, and stores the decremented result into a table entry.
- `call_function_with_offset`: Calls a function using a value retrieved from a pointer and an offset from a global variable.
- `write_memory_values`: Retrieves values from multiple function calls and stores them at specific offsets within a memory block, then updates a global variable with a shifted value from another routine.
- `dispatch_bytecode_instructions`: Reads a byte from a data stream and executes a specific routine or sub-routine based on the value.
- `execute_indirect_call_8`: calls another routine through a pointer.
- `call_indirect_function_2`: Calls a function through a pointer stored in memory.
- `perform_infinite_recursion`: This routine calls itself indefinitely without performing any other operations.
- `call_indirect_function`: Calls a function through a pointer stored at a specific offset from a base address.
- `invoke_indirect_resource_parser`: Executes a function via a pointer and subsequently parses resource header metadata to update global variables and a memory pointer.
- `execute_indirect_write`: Performs an indirect call and writes bytes to a memory location using an offset.
- `execute_indirect_initialization`: Performs an indirect call and initializes memory offsets using values from a data stack.
- `call_and_store_result`: Calls a function through a pointer and stores the returned word into a global variable.
- `execute_indirect_call_and_manage_table`: Performs an indirect function call and then searches or updates a resource lookup table.
- `call_indirect_resource_handler`: Calls a function through a pointer and then iterates through a table to decrement values at specific memory locations based on a bitmask.
- `execute_indirect_call_6`: An empty routine that serves as a wrapper for an indirect call.
- `dispatch_and_clear_pointers`: Executes a function via an indirect pointer and then iterates through several global memory locations to call any non-zero routines found before clearing those locations.
- `call_indirect_via_offset`: Calculates an address by adding a negative offset to a value in a register and executes the function pointer stored at that location.
- `call_indirect_via_offset_2`: Executes a function by dereferencing a pointer located at a fixed offset from a value held in a register.
- `execute_indirect_pointer_call`: Retrieves a function pointer from an offset relative to a value in a register and executes it.
- `call_indirect_via_offset_3`: Executes a function through a pointer located at a fixed offset from a value held in a register.
- `initialize_resource_entry`: Executes an indirect call and then registers or initializes a new entry within a resource table.
- `execute_indirect_pointer_call_2`: Calculates an address by adding a negative offset to a value in a register and calls the function located at that resulting pointer.
- `perform_indirect_call`: Executes a function by calling the address stored at a specific location in memory.
- `process_resource_context`: Checks a bitmask in a pointer, increments a global index, and either copies a sequence of bytes into a table via a loop or executes two indirect calls and a recursive call.
- `read_concatenated_values`: Reads two consecutive sixteen-bit values from a source and passes them as a single concatenated parameter to another routine.
- `process_data_stream_segments`: Iterates through a data stream to extract byte counts and dispatch specific subroutines based on marker values.
- `decode_data_stream`: Reads values from a stream and writes them into a buffer, updating various counters and indices based on the data type encountered.
- `process_buffer_loop_2`: Iterates through a loop, calling a routine to fetch data, checking specific values, and writing bytes or words into a buffer based on conditional logic.
- `process_data_loop`: Iterates through a buffer, calling functions to process data and updating various counters and pointers based on specific byte values.
- `update_buffer_loop`: Performs a loop that calls functions to write data into a buffer and updates internal state based on conditional checks.
- `process_buffer_loop`: Iterates through a loop to call functions and write data into a buffer while updating counters and checking specific byte values.
- `decode_data_stream_2`: Iterates through a data stream, performing indirect calls based on specific byte values to update buffers and counters.
- `execute_indirect_call_7`: An empty routine that performs an indirect call.
- `call_indirect_function_alt`: Executes a function via an indirect call through a pointer.
- `call_indirect_via_offset_4`: Calls a function through a pointer located at a fixed offset from a value held in a register.
- `call_indirect_pointer`: Executes a function via a pointer located at a specific offset relative to a base address.
- `call_indirect_pointer_2`: Performs an indirect call to a function using a pointer at a given offset.
- `call_indirect_pointer_3`: Uses a pointer stored at a specific offset to call another routine.
- `execute_indirect_call`: Calls a function through a pointer stored at a specific offset.
- `register_table_entry`: Calculates an index and stores several parameters into a table, then calls another routine to process the data and increments a counter.
- `remove_table_entry`: Decrements a counter and, if a condition is met, copies values from the last table entry into a temporary structure before clearing the entry.
- `call_function_with_parameter`: Calls a function through a pointer and passes a single parameter to it.
- `clear_table_entry`: Passes a value from a table to a function and then sets that table entry to zero.
- `store_and_call_result`: Combines input parameters into two words, calls a function with them, stores the result in a table, and then calls another function with that result.
- `load_word_from_pointer`: Reads four bytes from a memory location pointed to by a parameter and returns them as a single longword.
- `combine_bytes_to_word`: Takes two consecutive bytes from a memory location and combines them into a single unsigned sixteen-bit value.
- `read_and_advance_pointer`: Reads two bytes from a memory location, increments the pointer by two, and returns the bytes combined into a sixteen-bit value.
- `check_conditional_callback`: Checks if a byte at a specific offset is zero and if a short value matches certain constants before executing a function pointer stored in a table.
- `update_structure_and_call`: Writes two values to a structure, conditionally updates a pointer within that structure based on a short value, calls a function with a pointer, and resets a byte in memory.
- `intersect_and_clip_rectangles`: Compares two rectangular areas to check for intersection and clips them against a defined boundary.
- `process_conditional_callbacks`: Checks multiple status flags and, depending on their values, either executes a loop of function calls with parameters from a table or calls other routines to update state.
- `update_entity_spatial_attributes`: This routine updates an entity's spatial data structure by extracting values from a linked list or table and performing coordinate calculations.
- `calculate_grid_proximity_offset`: Calculates the nearest valid grid cell coordinates by iterating through a 2D array of bytes to find the closest non-zero entry relative to a given position.
- `map_input_value`: Compares a byte from a table at a specific offset and stores a corresponding byte into a second location based on the match.
- `map_input_value_alt`: Checks a byte value against several constants and writes a different set of mapped byte values to a destination.
- `process_input_character_states`: Evaluates character data from two separate structures to update state bytes based on specific character matches and numeric conditions.
- `clear_linked_list_node`: Checks if a global pointer is non-zero, calls another routine with that pointer, and then clears the global pointer.
- `clear_alternative_node`: Checks a different global pointer, calls another routine using the value at that pointer, and then sets the global pointer to zero.
- `read_conditional_data_block`: Performs a conditional read of data and uses the result to determine whether to call another routine with specific offsets or a fixed value.
- `process_nested_resource_data`: Performs a series of conditional reads and resource evaluations, storing a result into a global variable and potentially executing a secondary routine based on the returned value.
- `parse_resource_data_stream`: Reads a type identifier from a data stream and executes specific sub-routines to load or manipulate memory buffers based on the identifier.
- `load_engine_resources`: A large routine that manages memory allocation, parses filenames, handles file loading, and initializes engine subsystems through a complex chain of recursive and conditional calls.
- `parse_resource_header_data`: Parses a structured data block by reading length-prefixed strings and arrays of values.
- `clear_multiple_pointers`: Checks several global memory locations, calls another routine for each non-zero value found, and then clears those locations.
- `process_complex_resource_table`: Iterates through multiple tables to call routines based on non-zero pointers, processes a command byte from a table, and clears a global flag and a memory location.
- `initialize_memory_blocks`: Fills four consecutive word-sized memory locations with alternating values of all ones and all zeros.
- `check_recursive_condition`: Checks a value at a memory offset and calls itself recursively if the value is not negative one.
- `parse_and_reconstruct_sprite_data`: Parses a custom bytecode stream to extract dimensions and pixel data, then performs a coordinate transformation to reshape the data into a structured memory layout.
- `process_linked_list_resources`: Iterates through a list to call routines using pointers from a table, checks a bitmask in another table, and resets a global short value to all ones.
- `load_and_decompress_resource`: Allocates a decompression buffer based on a read size, copies a string into it, and executes a function through a pointer while managing linked list nodes and file operations.
- `calculate_offset_value`: Performs arithmetic operations involving longwords and words retrieved from memory to calculate and return a value.
- `initialize_entity_attribute_table`: Allocates a memory block for an array of data structures based on a count and populates each entry with calculated spatial coordinates and default status flags.
- `manage_resource_lookup_table`: Searches a small table for a matching identifier or an empty slot.
- `decrement_reference_count`: Searches a table of ten entries for a matching value, decrements a byte at the corresponding index, and recurses if the byte does not reach zero.
- `register_and_initialize_resource_entry`: Searches a table for an existing entry matching a specific identifier or finds an empty slot to allocate a new one.
- `process_resource_table`: Validates an index, checks a bitmask, and iterates through a table to decrement values at specific memory locations, potentially calling other routines and clearing table entries.
- `parse_entity_attribute_table`: Iterates through a structured data table to extract and combine byte values into coordinate and attribute offsets.
- `execute_recursive_calls`: Performs a sequence of recursive calls where the result of one call is passed as an argument to the next, eventually concatenating two results and passing them to a final call.
- `manage_resource_lookup_table_3`: Searches a table for an existing entry matching a specific identifier or finds the first empty slot to insert a new record.
- `process_resource_table_alt`: Iterates through a table to decrement values at memory locations based on a bitmask and a loop counter, while calling other routines to handle specific conditions and clearing table entries.
- `execute_repeated_routines`: Executes a routine once, followed by two separate loops that each execute a routine ten times.
- `clear_data_tables`: Resets multiple memory areas by writing zeros or all ones to various byte and word arrays.
- `process_recursive_data_2`: Performs recursive calls and stores sixteen-bit values into a table based on the returned integer value.
- `process_recursive_data`: Performs recursive calls and stores results into a table based on a calculated offset.
- `fill_and_process_stack`: Fills a memory block with a specific byte value and then processes a stream of bytecode to perform arithmetic and unary operations on a data stack.
- `process_resource_header_metadata`: Reads a sixteen-bit value from a memory location and passes that value as an argument to a routine that parses structured data blocks.
- `initialize_attribute_from_pointer`: Reads a sixteen-bit value from a memory location and uses it to initialize a structured data table.
- `execute_recursive_call_2`: Calls another routine, passes its return value as an argument to a subsequent call of that same routine, and then returns.
- `parse_resource_header_metadata`: Reads multiple values from a stream, performs arithmetic to calculate dimensions, and updates several global variables and a pointer to a memory block based on the parsed data.
- `write_bytes_to_offset`: Reads sixteen-bit and eight-bit values from a source and stores three separate bytes into a memory area using an offset derived from a previously read value.
- `initialize_memory_offsets`: Uses a value from a data stack to calculate an offset and writes three specific byte values to that location in memory.
- `calculate_and_apply_offset_transform`: Calculates a value based on the difference between two variables and applies a scaled offset to a destination buffer.
- `call_function_with_parameter_2`: A routine consisting of 54 instructions that calls a function through a pointer while passing a single parameter.
- `manage_resource_lookup_table_2`: Searches a table for an existing entry matching a specific identifier or inserts a new entry if no match is found.
- `dispatch_bytecode_command`: Evaluates a command byte from a table and executes different routines based on the bitmask and value, eventually clearing a pointer in a table.
- `populate_tree_table_2`: Iterates through a loop to copy data from a source location into a destination table using an offset and a multiplier.
- `store_copied_bytes`: Calls another routine and then stores three bytes from the stack into a memory location pointed to by a global variable.
- `execute_stored_function`: Reads values from memory, calls a function with those values, stores the result in a table, and then calls another routine using a value from a table.
- `read_and_conditional_call`: Reads a sixteen-bit value from a memory location and, if the value at a calculated offset in a table is non-zero, calls another routine.
- `clear_table_entry_2`: Passes a value from a table to another routine and then sets that table entry to zero.
- `populate_data_structure_from_stream`: Reads three consecutive sixteen-bit values from a memory stream and stores them into specific offsets within a data structure, followed by zeroing out two additional adjacent fields.
- `process_recursive_data_table_2`: Reads a value from a table and uses it to call another routine recursively.
- `process_indexed_pointer`: Reads a sixteen-bit value, uses it as an index into a table, calls another routine with the value found there if it is non-zero, and then clears the table entry.
- `call_multiplication_routine`: Checks a value in a table and, if non-zero, calls another routine to perform a calculation.
- `process_bytecode_execution_context`: Parses a bytecode stream to evaluate arithmetic and stack operations, managing memory allocation and rectangular clipping boundaries.
- `update_data_structure`: Retrieves values from memory and stores them into a structure at an offset determined by a previously retrieved value.
- `interpret_bytecode_logic_2`: Processes a complex sequence of data by performing arithmetic, traversing hierarchical structures, and executing operations based on values retrieved from a table.
- `parse_recursive_resource_tree`: Traverses a hierarchical data structure by recursively calling itself to process nodes and calculate offsets.
- `process_recursive_data_structure`: Performs recursive operations on a data structure, including conditional node removal, rectangle clipping, and iterative updates to a value based on results from another routine.
- `update_conditional_value`: Stores a returned value into a memory location and compares it to another value.
- `load_resource_file`: Loads a file by parsing its extension and performs byte-swapping on the loaded data block.
- `push_pop_resource_context`: Saves the current resource state and index into a stack-based array before recursively processing a new resource context.
- `manage_resource_stack`: Manipulates a stack of resource metadata by shifting entries and updating pointers.
- `update_stack_and_hardware`: Sets the stack pointer using a long value from a register, increments a value at a pointer in a register, and performs a bitwise OR on a byte at a memory location.
- `copy_string_to_stack`: Copies a string from a source pointer into a local stack buffer.
- `initialize_memory_values`: Sets several memory locations to zero or specific values and conditionally fills a memory block with a byte value.
- `allocate_multiple_memory_blocks`: Allocates three distinct memory blocks using different size parameters and stores the resulting pointers into a table.
- `initialize_resource_data`: Allocates multiple memory blocks, populates a table with pointers, and initializes a series of memory locations and a loop-based data block with specific byte values.
- `update_global_pointer`: Sets a global word value to all ones and then updates a global long value based on the result of another routine.
- `manage_string_buffer`: Copies a string to a buffer, calculates its length, and null-terminates it.
- `append_resource_string`: Copies a string to a buffer and stores the resulting length into a memory location.
- `fetch_resource_data`: Retrieves a value from a memory location and stores it in a global long variable if the retrieved value is non-negative.
- `read_and_store_data`: Reads a word from memory, and if the value is positive, performs a series of conditional reads and function calls to store a calculated four-byte value into a global variable.
- `parse_status_string`: Checks specific bytes within a data structure and, if certain conditions are met, converts a character to a numeric string and stores the resulting value in a global long variable.
- `initialize_resource_tables`: Resets various memory areas by filling tables with specific byte and word patterns and clearing linked list nodes.
- `execute_sequential_calls_2`: Calls two routines using pointers from a table and then performs conditional calls based on comparisons between global values and a short value.
- `initialize_resource_structures_2`: Calls several routines to initialize resource tables and data, clears a series of memory locations using a loop, and performs a sequence of indirect calls and pointer assignments.
- `interpret_bytecode_stream`: Iterates through a data stream, interpreting bytes as opcodes to perform variable-length increments of a pointer.
- `parse_bytecode_stream_3`: Interprets a stream of bytes as opcodes to manipulate a pointer and a stack-like structure.
- `parse_bytecode_stream`: Iterates through a data stream, interpreting bytes as opcodes to perform variable-length increments of a pointer.
- `interpret_bytecode_stream_4`: This routine is called by a bytecode processing routine and executes instructions that parse and interpret a custom stream of bytes.
- `execute_virtual_bytecode`: Parses and executes a custom instruction stream by performing arithmetic, bitwise, and logical operations on data.
- `execute_bytecode_stream_2`: Parses and executes a custom instruction stream by reading opcodes and performing arithmetic, bitwise, or logical operations on data.
- `execute_bytecode_stream_3`: Parses and executes a custom instruction stream by reading opcodes and performing arithmetic, bitwise, or logical operations on a data stack.
- `execute_bytecode_stream_4`: Parses and executes a custom instruction stream by performing arithmetic, bitwise, and logical operations on a data stack.
- `execute_bytecode_stream_5`: Parses a custom instruction stream by reading opcodes and processing associated data operands.
- `execute_bytecode_stream_6`: Parses and executes a custom instruction stream by performing arithmetic, bitwise, and logical operations on data.
- `execute_bytecode_stream_7`: Parses and executes a custom instruction stream by performing arithmetic, bitwise, and logical operations on data.
- `execute_bytecode_interpreter_2`: Interprets a custom instruction stream by processing a data stack through a series of opcode-driven operations.
- `interpret_bytecode_stream_5`: Parses a custom bytecode stream to perform arithmetic and memory access operations.
- `interpret_bytecode_opcodes`: Fetches an opcode from a data stream and executes a corresponding branch of logic that performs arithmetic, logical, or stack-based operations.
- `execute_virtual_bytecode_2`: Parses and executes a custom instruction stream by performing arithmetic, bitwise, and logical operations on data.
- `execute_bytecode_instructions`: Parses and executes a custom instruction stream by performing arithmetic, bitwise, and logical operations on data.
- `parse_bytecode_instruction_stream`: Iterates through a stream of bytes to interpret a custom instruction set.
- `execute_custom_bytecode`: Parses and executes a stream of custom instruction bytes to perform data manipulation and conditional logic.
- `interpret_bytecode_stream_2`: Parses a stream of bytes as a custom instruction set to perform data manipulation and conditional logic.
- `interpret_bytecode_stream_3`: Processes a stream of bytes as a custom instruction set to perform arithmetic, logical operations, and recursive function calls.
- `parse_bytecode_stream_5`: Iterates through a stream of bytes to interpret specific control characters as instructions.
- `decompress_bytecode_stream`: Iterates through a stream of bytes to perform in-place decompression and transformation.
- `process_bytecode_stream_2`: Interprets a stream of bytes as opcodes to perform conditional logic, pointer increments, and data manipulation.
- `process_bytecode_stream_3`: Iterates through a stream of bytes to interpret custom opcodes for conditional logic, pointer manipulation, and data updates.
- `execute_bytecode_logic`: Interprets a stream of opcodes to perform conditional branching, pointer arithmetic, and data manipulation.
- `execute_bytecode_logic_2`: Interprets a stream of opcodes to perform conditional branching, pointer arithmetic, and data manipulation.
- `execute_bytecode_logic_3`: Interprets a stream of opcodes to perform conditional branching, pointer arithmetic, and data manipulation.
- `execute_bytecode_logic_4`: Interprets a stream of opcodes to perform conditional branching, pointer arithmetic, and data manipulation.
- `interpret_bytecode_logic`: Interprets a stream of opcodes to perform conditional branching, pointer arithmetic, and data manipulation.
- `process_bytecode_stream`: Interprets a stream of bytes as opcodes to perform conditional logic, pointer increments, and data manipulation.
- `execute_bytecode_stream`: This routine serves as a wrapper that invokes a bytecode interpreter to process a sequence of instructions for logic and data manipulation.
- `parse_bytecode_stream_6`: Recursively parses a stream of bytes to evaluate a custom bytecode language.
- `store_and_return_word`: Reads a sixteen-bit value from a memory location via a pointer and stores that value back into a different memory location calculated from a register.
- `process_recursive_bytecode`: Executes a bytecode operation and conditionally performs a recursive call if specific byte patterns are matched in the data stream.
- `execute_bytecode_loop`: Iterates through a sequence of bytes defined by a count, performing bytecode operations and accumulating a value through multiplication.
- `interpret_bytecode_stack_operations`: Processes a stream of bytecode to perform arithmetic and unary operations on a data stack.
- `execute_bytecode_stack_interpreter`: Interprets a stream of bytecode to perform arithmetic and unary operations on a data stack.
- `execute_bytecode_interpreter`: Processes a stream of bytecode to perform arithmetic and unary operations on a data stack.
- `execute_bytecode_stack_operations_2`: Interprets a stream of bytecode instructions to perform arithmetic and logical operations on a data stack.
- `execute_bytecode_stack_operations`: Processes a stream of bytecode to perform arithmetic and unary operations on a data stack.
- `interpret_bytecode_arithmetic`: Processes a stream of bytecode to perform arithmetic operations including multiplication, division, modulo, and bitwise AND. It uses a switch-based dispatch mechanism to manipulate values stored in memory based on command codes.
- `parse_bytecode_stream_4`: Interprets a stream of command codes by reading bytes and executing a switch-based dispatch logic.
- `parse_bytecode_stream_2`: Reads a stream of bytes and executes a switch-based dispatch logic to interpret command codes.
- `evaluate_condition_status`: Calls another routine and checks if the returned byte matches specific values or if a value at a memory offset is non-zero.
- `adjust_value_range`: Checks if a value is within a specific range and subtracts a constant if it is.
- `update_parameter_value`: Calls another routine and uses the returned byte to select one of several operations.
- `store_byte_and_return`: Stores a word from a memory location into a location pointed to by a table and returns a byte from a nearby address.
- `reset_and_get_byte`: Writes a zero to a memory location pointed to by a table and returns a byte from a nearby address.
- `parse_bytecode_and_write_data`: Parses a bytecode stream to determine the number of elements to process.
- `write_byte_to_buffer`: Writes a single byte from a source to a calculated destination offset within a buffer.
- `write_word_to_buffer`: Writes a two-byte word from a source to a calculated destination offset within a buffer.
- `write_longword_to_buffer`: Writes a four-byte longword from a source to a calculated destination offset within a buffer.
- `process_data_table`: Iterates through a table to write different sized values, such as bytes, words, or longwords, into a destination buffer based on specific opcode values.
- `evaluate_and_increment_counter`: Processes a series of bytecode operations and parameter sets before incrementing a value in a global table.
- `decrement_and_recurse_table_pointer`: Checks if a value pointed to by a global pointer is non-zero, decrements that value, and calls itself recursively until the value reaches zero.
- `parse_scripted_command_stream_4`: Iterates through a character stream to extract command tokens delimited by specific control characters.
- `parse_scripted_command_stream_3`: Iterates through a character stream to identify command tokens and control characters, executing specific subroutines based on the identified byte value.
- `execute_scripted_command_stream`: Parses a character stream to identify command tokens and control characters, executing specific subroutines based on the identified byte values.
- `parse_scripted_command_stream_2`: Iterates through a character stream to identify specific command tokens and control characters.
- `parse_scripted_command_stream`: Iterates through a character stream to identify specific command tokens and control characters.
- `increment_global_counter`: Increments a longword value stored at a specific memory location.
- `rotate_list_elements`: Iterates through a table to find specific entries and performs a cyclic shift of three-byte data structures within a linked list.
- `update_data_structure_fields`: Updates several fields in a data structure using values from a global variable and performs conditional mapping of a parameter value to a new constant.
- `build_status_bitmask`: Evaluates a series of boolean flags from a data structure and aggregates them into a single bitmask.
- `evaluate_bytecode_logic_stream`: Iterates through a custom bytecode stream to perform conditional comparisons and logical evaluations.
- `parse_resource_loop`: Increments a global counter and enters a loop that calls another routine to parse data and updates a pointer based on returned lengths.
- `process_resource_stream`: Iterates through a data stream by calling another routine to check status and updating a pointer based on the results of subsequent calls.
- `parse_resource_entry_stream`: Iterates through a data stream to extract structured resource definitions, handling bit-masked flags and variable-length field lengths.
- `parse_resource_data_table`: Iterates through a byte stream to extract structured data, using bitwise masking and shifts to reconstruct values.
- `parse_resource_data_stream_6`: Iterates through a byte stream to extract structured data using a switch-based opcode system.
- `parse_and_populate_data_table`: Iterates through a byte stream to extract structured data, performing bitwise masking and shifts to reconstruct values.
- `update_state_from_counter`: Reads three values from another routine, stores them in a table, and performs arithmetic on a value derived from a global counter.
- `update_global_state`: Stores several values into a global table and modifies a value at a pointer location.
- `update_global_state_alt`: An identical routine to R1 that performs the same memory writes and pointer increments.
- `load_and_modify_values`: Reads three consecutive sixteen-bit values from other routines into a data structure and performs bitwise and arithmetic transformations on a value retrieved from a pointer.
- `parse_nested_resource_stream`: Recursively parses a data stream by reading header bytes that define block lengths and types.
- `process_resource_stream_2`: Recursively parses a custom data stream by reading a header byte to determine the block type and length.
- `parse_nested_resource_stream_3`: Recursively decodes a hierarchical data stream by reading header bytes to determine block types and lengths.
- `process_resource_data_stream`: Iterates through a data stream by reading header bytes to determine block types and lengths.
- `parse_resource_stream_header_4`: Decodes a custom data stream by reading a header byte to determine the block type and length.
- `parse_nested_resource_stream_4`: Recursively traverses a data stream by reading header bytes to determine block types and lengths.
- `parse_resource_stream`: Decodes a custom data stream by reading a header byte to determine block types and lengths.
- `parse_resource_stream_2`: Iterates through a data stream by reading header bytes to determine block types and lengths.
- `parse_nested_resource_stream_5`: Recursively decodes a custom data stream by reading header bytes to determine block types and lengths.
- `parse_resource_stream_header`: Decodes a custom data stream by reading a header byte to determine the block type and length.
- `dispatch_bytecode_instruction`: Parses a custom bytecode stream by reading instruction headers to determine operation types and operand lengths.
- `parse_bytecode_instruction_stream_2`: Decodes a custom bytecode stream by extracting instruction headers and calculating operand lengths based on high nibble values.
- `parse_bytecode_stream_7`: Iterates through a custom bytecode stream by reading instruction headers and dispatching based on operation types and operand lengths.
- `parse_bytecode_instruction_stream_3`: Decodes a custom instruction stream by extracting an opcode and operand length from a byte.
- `parse_bytecode_instruction_stream_5`: Iterates through a custom bytecode stream by decoding instruction headers into operation types and operand lengths.
- `parse_bytecode_instruction_stream_4`: Iterates through a custom bytecode stream by decoding instruction headers into operation types and operand lengths.
- `dispatch_bytecode_instruction_2`: Parses a custom bytecode stream by extracting an instruction header and determining the operand length.
- `parse_bytecode_stream_8`: Decodes a custom instruction stream by extracting operation codes and operand lengths from a data buffer.
- `execute_bytecode_stream_8`: Parses and executes a custom bytecode stream by reading instruction headers that define operation types and operand lengths.
- `process_compressed_resource_stream`: Iterates through a data stream by reading command headers and payload lengths to execute specific decoding operations.
- `validate_resource_header`: Checks if a specific value at an offset within a data structure equals two.
- `decode_compressed_resource_stream`: Decodes a data stream by reading a header byte that defines a command type and a payload length.
- `parse_resource_stream_header_5`: Extracts a command identifier and length from a data stream, using the high nibble of the identifier to determine a padding or command type.
- `parse_compressed_resource_stream`: Decodes a data stream by reading a header byte that defines a command type and a payload length.
- `process_resource_stream_blocks`: Iterates through a data stream by reading a header byte to determine block types and lengths.
- `parse_nested_resource_stream_7`: Recursively parses a data stream by reading header bytes that define block lengths and types.
- `parse_resource_stream_block_5`: Recursively processes a data stream by reading header bytes to determine block types and lengths.
- `parse_resource_stream_block_2`: Iterates through a data stream by reading a header byte to determine block type and length.
- `process_resource_stream_blocks_2`: Iterates through a data stream by reading header bytes to determine block types and lengths.
- `parse_resource_stream_blocks_4`: Iterates through a data stream by reading header bytes to determine block types and lengths.
- `process_resource_stream_block`: Parses a data stream by reading header bytes to determine block types and lengths.
- `parse_resource_stream_block_3`: Iterates through a data stream by reading a header byte to determine block type and length.
- `parse_resource_stream_blocks_5`: Iterates through a data stream by reading a header byte to determine block types and lengths.
- `parse_resource_stream_block_4`: Iterates through a data stream by reading a header byte to determine block type and length.
- `parse_resource_stream_blocks_6`: Iterates through a data stream by reading header bytes to determine block types and lengths.
- `parse_resource_stream_block`: Iterates through a data stream by reading a header byte to determine block type and length.
- `parse_resource_stream_blocks`: Iterates through a data stream by reading a header byte that encodes both a block type and a length.
- `parse_resource_stream_blocks_2`: Iterates through a data stream by reading a header byte that encodes both a block type and a length.
- `parse_resource_stream_header_2`: Reads a header byte from a data stream to determine the block type and length.
- `parse_resource_data_stream_3`: Iterates through a data stream by reading a header byte that determines the length and type of subsequent data blocks.
- `parse_resource_stream_blocks_3`: Iterates through a data stream by reading a header byte that defines the block type and length.
- `parse_nested_resource_stream_2`: Iterates through a data stream by reading a header byte to determine the length and type of subsequent data blocks.
- `parse_resource_data_stream_4`: Iterates through a data stream by reading a header byte to determine block type and length.
- `parse_resource_data_stream_5`: Iterates through a data stream by reading a header byte that determines the length and type of subsequent data blocks.
- `parse_nested_resource_stream_6`: Iterates through a data stream by reading a header byte that defines the type and length of subsequent data blocks.
- `parse_resource_stream_header_3`: Reads a header byte to determine the length and type of subsequent data blocks.
- `parse_resource_bytecode_stream`: Iterates through a data stream by reading a header byte that determines the length and type of subsequent data blocks.
- `parse_resource_data_stream_2`: Iterates through a data stream by reading a header byte that determines the length and type of subsequent data blocks.
- `invoke_resource_parser`: This routine serves as a wrapper that calls a function to recursively parse a nested data stream based on block lengths and types.
- `traverse_nested_resource_stream`: Recursively processes a data stream by checking header bytes and calling itself when specific type flags are encountered.
- `update_recursive_state`: Manages recursive data traversal, updates a counter, processes timer-based data, and maps specific return values to a new set of constants.
- `process_nested_state`: Iteratively calls another routine and, if a specific bit is set in a parameter, calls a third routine while checking a loop condition.
- `merge_two_strings`: Combines two byte strings by copying parts of each into a destination buffer based on their lengths.
- `truncate_string`: Modifies a string by either inserting a null terminator or copying a segment from another string based on length parameters.
- `merge_byte_strings`: Calculates the length of a string and then merges it into another string by shifting characters and inserting a null terminator.
- `clip_rectangular_areas`: Calculates the intersection of two rectangular boundaries and recursively calls itself to adjust dimensions based on a clipping area.
- `update_clipping_and_callback_state`: Adjusts rectangular clipping boundaries through recursive intersection and updates a state variable by searching through a table of rectangular areas.
- `initialize_table_entries`: Sets a specific global value to zero and then fills a table with a repeating sequence of values.
- `process_bytecode_data_table`: Iterates through a table of data structures until a terminator is reached.
- `insert_table_record`: Searches a table for a matching key and, if not found, inserts a new record containing multiple data fields.
- `invalidate_table_entries`: Iterates through a table and sets a specific field to a constant value for every entry matching a given key.
- `update_resource_list`: Counts specific entries in a table, allocates memory via another routine, and updates a global list with new data and metadata.
- `process_table_entry`: Decrements a counter, updates a flag based on a bitmask, copies data from a table into global variables, and calls another routine with a pointer found in the table.
- `copy_memory_block_2`: Transfers a block of data from one memory location to another using a loop.
- `process_recursive_clipping_and_callbacks`: Manages a recursive hierarchy of rectangular clipping areas and associated state updates.
- `execute_clipping_hierarchy`: This routine calls a function that manages a recursive hierarchy of rectangular clipping areas and their associated state updates.
- `process_string_data`: Manipulates a string by performing character substitutions from a lookup table and removing redundant spaces.
- `populate_sprite_coordinate_table`: Calculates and stores spatial coordinate offsets and position values into a data structure based on a scaling factor and a loop index.
- `copy_register_to_itself`: Performs a redundant move of a long value from one register to the same register.
- `load_table_values`: Loads two words from offset locations in tables using an index, loads a long value from a global variable, and performs a redundant move.
- `fetch_coordinate_data`: Retrieves two words from memory using indexed offsets, loads a long value from a global variable, and performs a redundant move.
- `process_entity_collision_events`: Iterates through an entity table to identify specific active entities based on bitmask checks and attribute ranges.
- `return_no_op`: The routine consists of a single instruction that immediately returns, performing no work.
- `preserve_and_recurse_state`: Saves a value from a global variable, updates that variable using a calculation involving a parameter, sets a flag, and enters a recursive loop that calls itself until a specific short value remains unchanged.
- `recursively_initialize_parameter`: Recursively calls itself while either negating a parameter or passing it through, based on a comparison of a value in a structure.
- `traverse_parameter_table`: Iterates through a table of structures to find an entry matching a specific parameter.
- `initialize_entity_attribute_table_2`: Initializes a structured data table by allocating memory and populating multiple entries with specific coordinate offsets, attribute values, and status flags.
- `parse_bytecode_command_parameters`: Decodes a command header to determine a mode and extracts a sequence of four parameters from a data stream.
- `update_parameter_state`: Iterates through a table of structures to find an entry matching a specific identifier and updates its associated state.
- `store_bytecode_operands`: Retrieves two consecutive values from a stack and stores them into a memory location using a pointer derived from a register.
- `reconstruct_resource_node_geometry`: Recursively traverses a hierarchical data structure to calculate and update bounding box coordinates and dimensions for resource components.
- `parse_resource_node_hierarchy`: Recursively traverses a hierarchical data structure to extract and assemble compressed resource components.
- `reconstruct_resource_tree_node`: Recursively traverses a hierarchical data structure to extract and assemble compressed resource components.
- `parse_resource_tree`: Recursively traverses a structured data tree to extract and validate nested elements.
- `parse_compressed_tree_structure`: Recursively traverses a data structure by reading offsets and child pointers to reconstruct a hierarchical tree.
- `traverse_compressed_resource_tree`: Recursively traverses a hierarchical data structure by reading offsets and child pointers to reconstruct a tree.
- `process_entity_attribute_list`: Iterates through a list of data structures associated with an entity to calculate and update spatial offsets and attribute values.
- `execute_postfix_bytecode_stream`: Processes a stream of data using a stack-based postfix notation system, word fetching, and collision data processing.
- `populate_destination_table`: Retrieves values from a source data structure based on conditional logic and stores them into a destination table at specific offsets.
- `execute_bytecode_sequence`: Recursively parses a bytecode stream to retrieve values and store them into a destination table at specific offsets.
- `update_attribute_values`: Clamps four global short values to a minimum of zero and then stores them into a table at indices provided by another routine.
- `refresh_entity_spatial_state`: Iterates through an entity list to reset specific attribute values and spatial offsets.
- `update_entity_collision_states`: Iterates through an entity list to perform bounding box overlap checks between pairs of active entities.
- `update_entity_spatial_bounds`: Iterates through a list of entity pointers to identify specific active entities and updates their spatial coordinate values.
- `detect_entity_collisions`: Iterates through an entity list to perform bounding box intersection tests between multiple objects.
- `search_and_update_entity_list`: Iterates through a list of pointers to compare spatial coordinates or bounding box values between two entities.
- `update_entity_state_from_bytecode`: Updates an entity's attributes and spatial offsets by interpreting a sequence of bytecode instructions.
- `find_minimum_index`: Iterates through a table to find an index that satisfies specific boundary conditions and minimizes a sum of values returned by another routine.
- `find_minimum_index_offset`: Iterates through a table to find an index that satisfies specific boundary conditions and minimizes a sum of values returned by another routine.
- `fetch_table_element`: calculates an offset from a base address using two input parameters and returns a single byte from the resulting memory location.
- `check_grid_adjacency`: Compares two coordinate pairs against a grid structure to determine their relative spatial relationship.
- `traverse_grid_path`: Uses a grid adjacency check to navigate through a table, modifying coordinate parameters based on returned values until a target coordinate is reached or a boundary is hit.
- `execute_empty_sequence`: The routine is empty and performs no operations.
- `search_grid_range`: iterates through a range of values defined in a structure and calls another routine to check conditions.
- `update_grid_coordinate_data`: Updates a structure by copying bytes from one offset to another and performing recursive calls to process related data.
- `store_bytecode_bytes`: Retrieves two bytes from a structure at a specific offset and stores them as integers into a table at indices provided by other routine calls.
- `update_bytecode_operand_state`: Calculates an offset into a table based on a return value and a global pointer to write a constant byte to a specific memory location.
- `count_odd_indices`: Iterates from zero up to a value provided in a parameter and counts how many odd indices exist in that range.
- `update_entity_spatial_state`: Calculates and updates an entity's spatial attributes and coordinate data by iterating through associated data structures.
- `update_entity_state_and_position_2`: Updates an entity's attribute structure by extracting values from a linked list or table and calculating new spatial or state offsets.
- `synchronize_entity_state_data`: Updates an entity's attribute structure by copying specific values from a linked list or table into a target memory block.
- `update_entity_state_and_position`: Updates an entity's attribute structure by extracting values from a linked list or table and calculating new spatial or state offsets.
- `calculate_grid_coordinate_offset`: Converts absolute spatial coordinates into grid indices and calculates the nearest valid cell offset within a 2D array.
- `relink_linked_list_nodes`: Updates a node's pointer and data values by calculating an offset and copying bytes from a source structure.
- `update_recursive_grid_node`: Calculates a relative offset and copies byte values between linked structures.
- `traverse_and_process_data_table`: Iterates through a structured table of data entries, performing recursive calls based on comparisons between specific byte values and parameters.
- `update_entity_movement_state`: Updates an entity's internal state flags and target coordinates based on grid adjacency and pathfinding results.
- `update_entity_state_and_animation_2`: Updates an entity's internal state flags and animation frame index based on a series of conditional checks and a lookup table.
- `update_entity_behavior_and_state`: Updates an entity's internal state flags, animation frame index, and specific behavioral properties based on a series of conditional checks and a lookup table.
- `update_entity_state_and_animation_3`: Updates an entity's internal state flags and animation frame index based on a series of conditional checks and a lookup table.
- `update_entity_state_and_animation`: Updates an entity's internal state flags and animation frame index based on a series of conditional checks and a lookup table.
- `set_entity_animation_and_position`: Sets an entity's animation frame based on its current state and updates its spatial coordinates.
- `update_entity_movement_and_animation`: Updates an entity's spatial coordinates and animation frame based on a state check and a direction byte.
- `update_entity_animation_and_position`: Updates an entity's animation frame and spatial coordinates based on a state change.
- `update_entity_animation_and_position_2`: Updates an entity's animation frame and modifies its spatial coordinates based on a specific state value.
- `update_entity_movement_vectors`: Modifies an entity's coordinate offsets and direction state based on a lookup table of movement patterns.
- `update_entity_movement_and_animation_2`: Calculates new spatial coordinates and updates animation frame indices for an entity based on its current state and direction.
- `update_entity_movement_offsets`: Updates an entity's spatial coordinates and state flags based on a direction byte.
- `update_entity_movement_and_state`: Updates an entity's spatial coordinates and state flags based on a conditional check of its current attributes.
- `update_entity_sprite_animation`: Updates an entity's animation frame and modifies its coordinate offsets based on a specific state value.
- `update_entity_animation_and_position_3`: Updates an entity's animation frame and spatial coordinates based on a state check.
- `update_entity_spatial_state_2`: Checks for changes in an entity's state or position and updates its coordinate offsets.
- `update_entity_animation_frame`: Updates an entity's current animation frame and modifies its spatial coordinate offsets based on a specific state value.
- `update_entity_movement_and_state_2`: Updates an entity's spatial coordinates and state flags based on a conditional check of its current attributes.
- `update_entity_animation_and_position_4`: Updates an entity's animation frame and modifies its spatial coordinates based on a specific state value.
- `update_entity_movement_and_direction`: Updates an entity's spatial coordinates and direction based on a state check and a series of conditional offsets.
- `update_entity_movement_and_animation_3`: Updates an entity's spatial coordinates and animation frame based on a direction byte and specific state conditions.
- `update_entity_movement_and_direction_2`: Updates an entity's spatial coordinates and directional orientation based on a state check.
- `update_entity_direction_and_position`: Modifies an entity's spatial coordinates and direction byte based on a specific state value.
- `update_entity_animation_frame_2`: Updates an entity's current animation frame and modifies its coordinate offsets based on a specific state value.
- `update_entity_directional_offset`: Modifies an entity's coordinate offsets based on a specific state byte and a set of directional rules.
- `update_entity_state_and_coordinates`: Updates an entity's attribute values and spatial offsets based on a conditional state check.
- `evaluate_script_node`: Recursively traverses a structured data table to execute logic based on byte comparisons and conditional branching.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning.
- `nop_padding_sequence`: This routine is empty and contains no executable instructions.
- `process_date_stamp_data`: Retrieves a date stamp and performs arithmetic calculations to store several derived values into a table.
- `copy_date_data`: Calls another routine and then copies four bytes from a global memory location into a provided buffer.
- `copy_modified_data`: Calls another routine and then copies three bytes from a global memory location into a provided buffer, with the first word being incremented by a constant value.
- `copy_interleaved_data_blocks`: Copies data from a source buffer into five separate destination buffers using an interleaved pattern.
- `return_from_subroutine_3`: Returns from the current subroutine.
- `return_from_subroutine_4`: The routine performs no operations and immediately returns to its caller.
- `return_no_op_handler`: The routine is a three-instruction stub whose Ghidra rendering shows only an immediate return, so it performs no visible work in the decompiled view.
- `nop_operation`: The routine is empty and performs no operations before returning.
- `return_no_op_handler_2`: The routine is a three-instruction stub whose body is only a return with no operations performed.
- `return_immediately`: This routine performs no operations and returns to its caller immediately.
- `initialize_handler_table`: Populates a large number of memory locations with a pointer to another routine and a global variable address.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `configure_hardware_bitmask_table`: Iterates through a four-entry table to update hardware-related control structures based on bits set in an input parameter.
- `search_three_word_record_list`: Iterates through a list of three-word records until a zero-word is encountered or a match is found for the value in a data register.
- `store_two_bytes`: Stores a single byte into two consecutive memory locations.
- `store_single_byte`: Stores a byte from a register into a memory location at an offset from a pointer.
- `store_byte_via_pointer`: Stores a single byte from a register into a memory location calculated by adding an offset to a value held in another register.
- `dispatch_nested_resource_handler`: Invoked by the nested resource data processor through a handler pointer stored in data, this 49-instruction routine is entered from outside the static control flow and was observed executing in the reference run.
- `update_hardware_control_flags`: Iterates through a bitmask to update various fields in a global structure, modifying hardware control bits and specific data members based on the state of the input value.
- `decode_bitstream_and_update_table`: Decodes a bitstream and updates a global table with derived values and pointers.
- `decode_bitstream_data`: Iterates through a loop to decode a bitstream, updating a global table with values and pointers derived from a source structure.
- `configure_dma_channels`: Sets a global variable to enable specific DMA channels by writing to the DMA controller register.
- `traverse_linked_data_structure`: Iterates through a linked list or table of data structures by following pointers stored within the elements.
- `update_sequence_state`: Increments internal counters and compares them against values stored in a data structure to manage state transitions.
- `set_byte_bit`: Sets the fourth bit of a byte located at an offset from a pointer.
- `process_compressed_bitstream_state`: Updates a bitstream decoding state machine by evaluating bitmask flags to manage counters, perform table lookups, and handle data stream termination or transitions.
- `decode_bitstream_control_structure_2`: Parses a compressed data stream by evaluating bitmask flags to perform table lookups, update counter values, and initialize data structures.
- `parse_compressed_data_stream`: Iterates through a data stream, identifying control bytes to perform lookups in a table or manipulate structure members.
- `parse_compressed_data_stream_2`: Iterates through a byte stream to decode values based on bitmask flags.
- `decode_bitstream_control_structure`: Parses a compressed data stream by evaluating bitmask flags to perform table lookups, update counter values, and initialize data structures.
- `decode_compressed_data_stream`: Iterates through a bitstream to extract values using bitmask checks and table lookups.
- `parse_compressed_data_stream_3`: Iterates through a data stream to decode values based on bitmask checks and table lookups.
- `parse_compressed_data_stream_4`: Iterates through a data stream, performing bitwise operations and pointer arithmetic to extract values.
- `frag_lea_and_lsl_move_rts`: Short code fragment that loads the address of a location in the code into a0, then masks d0 with 7, then shifts d0 left by 2, then copies an indexed entry from a0 to the stack, then returns.
- `update_counter_values`: Calculates a new value based on a comparison between a byte from a data structure and a register, then stores the result into a table.
- `clear_dma_control_register`: The routine writes zero to the DMACON hardware register and sets a global variable to a bit mask derived from an input value.
- `process_compressed_bitstream_flags`: Iterates through a data stream, evaluating bitmask flags to perform table lookups and update internal counters.
- `parse_compressed_data_stream_5`: Parses a compressed data stream by evaluating bitmask flags to perform table lookups, update counter values, and initialize data structures.
- `parse_compressed_data_stream_6`: Parses a compressed data stream by evaluating bitmask flags to perform table lookups, update counter values, and initialize data structures.
- `parse_compressed_bitstream_control`: Parses a compressed data stream by evaluating bitmask flags to perform table lookups, update counter values, and initialize data structures.
- `execute_sequence_loop`: Executes another routine twenty times within a loop.
- `select_value_by_byte`: Compares a byte from a parameter against several constants to return a specific short value, or evaluates several flags in memory to determine a secondary value.
- `map_byte_values`: Compares a byte from a parameter against several constants and returns a specific short value based on the combination of two input bytes.
- `process_recursive_data_table`: Traverses a data structure using recursive calls and pointer arithmetic based on values retrieved from a table.
- `wait_for_non_zero_result`: Repeatedly calls another routine until it returns a non-zero value.
- `update_coordinate_values`: Splits a global word into two bytes, performs arithmetic with wrapping logic on two memory locations, clamps the results within specific ranges, and updates those locations with the split bytes.
- `calculate_status_and_coordinates`: Increments a value based on bits in a global variable and populates three pointers with a value derived from two bytes in memory.
- `store_parameter_to_memory`: Stores a 32-bit parameter into several memory locations as a combination of two shorts and four bytes.
- `initialize_resource_structures`: Allocates memory, copies a block of data from a source to a destination, and populates several structures with specific values and pointers.
- `call_library_functions`: Loads values from memory and passes them as arguments to another routine and a library function.
- `copy_memory_block_3`: Copies a block of memory from a source to a destination using either word-sized or byte-sized increments based on a count.
- `update_cursor_position`: Updates a coordinate pair by calculating a value based on a ratio and clamping the resulting X and Y coordinates within a specific range.
- `update_state_flags`: Compares a byte from a global variable with another byte, then updates a table and several flag bytes based on the value of the global variable.
- `apply_bitmask_and_library_call`: Performs bitwise operations on a buffer using a bitmask and calls a library function.
- `apply_bitmask_to_buffer`: Performs bitwise operations on a source buffer using a bitmask and stores the results into a destination memory area.
- `fill_memory_blocks`: Iterates through a list of memory offsets to fill specific blocks of memory with either all ones or all zeros based on a bitmask.
- `fill_memory_with_constants`: Fills a block of memory with zeros and then, for each index whose corresponding bit is set, fills another block with all ones based on a count.
- `process_bitmask_layers`: Iteratively modifies memory values by performing bitwise operations controlled by a bitmask and a series of nested loops.
- `modify_bitmask_via_logical_operations`: Updates specific bits within four separate memory locations using bitwise OR or AND operations.
- `update_bitmask_flags`: Modifies specific bits within four different bytes based on a bitmask and a bit index.
- `merge_source_buffers`: Copies data from one source buffer to a destination and performs bitwise OR operations with three other source buffers during the copy.
- `process_nested_tables`: Recursively clears nested tables and merges four word streams into a destination table.
- `process_nested_data_tables`: Recursively traverses and manipulates nested data structures by clearing tables and merging word streams into a destination.
- `clear_nested_tables`: Iterates through a nested structure to set multiple memory locations to zero.
- `merge_four_word_streams`: Performs a bitwise OR operation on four word streams and stores the result in a destination table.
- `allocate_complex_object`: Calculates a size based on input parameters, allocates a memory block, and initializes it with various values and a loop-based zero-fill.
- `merge_bitmask_layers`: Iteratively modifies memory values using bitwise operations controlled by a bitmask and merges multiple source buffers into a destination via bitwise OR operations.
- `allocate_and_initialize_buffer`: Allocates a memory block and populates it with data from another routine, followed by a calculation that stores a value into a specific offset within that block.
- `decrement_and_recurse`: Subtracts four from a value pointed to by a parameter and then calls itself twice using the updated value and the original pointer.
- `allocate_memory_block`: Searches a linked list of free memory blocks to find a suitable segment based on a requested size.
- `remove_node_from_linked_list_2`: Removes an element from a sorted linked list and updates the list structure.
- `remove_node_from_linked_list`: Removes an element from a sorted linked list and updates the list structure.
- `increment_and_bitwise_not_memory`: Increments a value at a memory location pointed to by a register and performs a bitwise NOT operation on a word at another memory location.
- `allocate_memory_block_2`: Calls a routine to find and return a suitable memory segment from a list of free blocks.
- `manage_memory_allocation_error`: Attempts to allocate memory and, if unsuccessful, performs error handling and potential retry logic based on a global flag.
- `process_recursive_timer_data`: Performs recursive calls and arithmetic operations on values retrieved from a global variable and a timer device.
- `combine_arithmetic_results`: Calls another routine and combines its result with a divisor using bitwise shifts and modulo operations.
- `fill_memory_block`: Fills a memory area with a specific byte value for a specified number of iterations.
- `wait_for_timer_interval`: Repeatedly calls another routine until the difference between two returned values meets or exceeds a specified threshold.
- `compute_absolute_value`: Checks if the lower two bytes of a 32-bit parameter are less than one.
- `return_absolute_value`: Returns the absolute value of a passed integer.
- `negate_memory_value`: Loads a 32-bit value from a memory location offset from a register and returns its negation.
- `select_maximum_word`: Compares the lower two bytes and the upper two bytes of a 32-bit parameter.
- `return_minimum_value_alt`: Compares the upper two bytes and the lower two bytes of a 32-bit parameter.
- `return_minimum_value`: Returns the smaller of two provided integers.
- `load_word_from_offset`: Loads a 32-bit word from a memory location determined by adding an offset to a register.
- `convert_string_case`: Iterates through a null-terminated string and subtracts 32 from any character within the range of backtick to curly brace.
- `loop_until_zero`: Repeatedly calls another routine and stores its results in local buffers until a specific value reaches zero.
- `execute_loop_with_offset`: Executes another routine within a loop where the second parameter is calculated by multiplying a value from a table by the current loop index.
- `find_maximum_node_size`: Traverses a linked list of nodes and calculates the maximum distance between a node and its successor based on a bit flag.
- `sum_node_sizes`: Traverses a linked list and calculates the total sum of distances between nodes marked with a specific bit flag.
- `nop_padding_byte`: This routine is empty and consists of a single instruction that performs no operation.
- `call_wrapper_routine`: Calls another routine and returns.
- `execute_wrapper_routine`: Calls another routine and returns.
- `forward_call_to_routine`: Calls another routine with no arguments and returns, acting as a simple pass-through.
- `log_system_error`: Formats and outputs a system error message using a provided error code and string.
- `check_empty_stub`: The routine contains four instructions but performs no observable work and returns zero, acting as an empty stub that always returns zero.
- `call_library_function_2`: Checks a value at a specific offset from a pointer and calls a library function using a base address from a global variable.
- `update_nested_pointers`: Calculates values using signed division and updates multiple memory locations through indirect function calls and pointer arithmetic.
- `initialize_engine_subsystems`: Initializes multiple data structures by clearing memory blocks, populating tables with zeroed values, and setting up nested pointers.
- `execute_initialization_chain`: Recursively executes functions retrieved from a table and clears a specific memory location if a condition is met.
- `compare_and_recurse`: Compares a string from a provided pointer against a global variable and calls itself if the comparison result is zero.
- `collect_and_process_lists`: Traverses two linked lists from a global structure, stores their pointers in a local array, and then calls another routine for each pointer.
- `process_string_lists`: Iterates through two linked lists from a global structure and processes each element by converting string cases and performing byte string comparisons.
- `reorder_byte_blocks`: Iterates through a series of data blocks and reorders nine bytes within each block using a temporary buffer.
- `search_data_table`: Iterates through a table of entries, comparing data against a parameter, and returns a pointer to a matching or copied entry.
- `process_string_data_2`: Iterates through a list of items, comparing byte strings and performing conditional branches and memory writes based on the results.
- `search_and_update_table`: Iterates through a table to find a matching entry via string comparison and, if found, updates several fields in a destination table.
- `set_table_entry`: Checks if a value is within a specific range and, if so, writes a 32-bit value to a table entry.
- `update_table_values`: Validates an index against a range, checks a flag in a table, and performs arithmetic to update values stored in a table.
- `execute_unspecified_routine`: A routine consisting of 26 instructions that is called by another routine and is entered from outside the static flow.
- `parse_filename_and_load_resource`: Scans a string for a period character to determine a file extension, then performs byte-swapping on a data block and manages memory allocation for the resulting resource.
- `release_resource_node`: Checks a global short value, and if it is not negative one, recursively calls another routine with a pointer from a table, clears that pointer, calls another routine, and sets the global short to negative one.
- `manage_resource_allocation`: Handles memory allocation and resource management by checking global status flags and iterating through data in fixed-size increments.
- `update_resource_data`: Stores a value from a routine into a global variable and performs conditional updates to other global memory locations based on a byte value.
- `store_and_calculate_value`: Stores a value from a routine into a global variable and performs a calculation using bytes from a pointer to update a word.
- `check_range_condition`: Checks if a global byte is zero and a global integer is non-zero, then determines if a modified parameter falls within a specific range defined by other global integers.
- `process_resource_chunks`: Attempts to manage resource allocation through multiple conditional checks and processes data in fixed-size increments if a specific threshold is met.
- `toggle_global_flag`: Saves a word from a global variable, sets it to one, calls another routine, and then restores the original value.
- `xor_data_block_4`: Performs a bitwise XOR operation on four separate memory locations using a bit-shifted value calculated from loop indices.
- `xor_data_block_2`: Performs a nested loop to XOR bytes in a memory block using a value derived from a table and bitwise shifts.
- `xor_data_block_3`: Performs a bitwise XOR operation on a memory block using a calculated index and a bit-shifted value.
- `insert_into_list`: Stores a value into a memory location and calls another routine if the value is not negative one.
- `call_subroutine_with_scaled_input`: Calculates a value based on a 32-bit input and passes it to another routine.
- `traverse_and_remove_node`: Compares a parameter to a global integer and, if they differ, recursively calls itself with a pointer from the parameter before calling itself with the parameter.
- `clamp_value_minimum`: Decrements a value based on a second parameter and ensures the result is not less than zero.
- `intersect_and_clip_rectangles_2`: Recursively calculates the intersection of two-dimensional rectangular areas and adjusts their boundaries.
- `update_bounding_box`: Calculates and updates a bounding box or coordinate range by comparing values and calling an indirect function.
- `execute_pixel_modification`: A five-instruction routine that calls another routine to modify pixel data.
- `modify_pixel_data`: Iteratively modifies four longwords of data by performing bitwise operations based on values retrieved from a table and a bitmask.
- `apply_bitmask_to_pixels`: Iteratively modifies four bytes of data by applying bitwise AND or OR operations based on a bitmask and a calculated shift value.
- `apply_bitmask_to_pixel_data`: Iteratively modifies four bytes of data by applying bitwise AND or OR operations based on a bitmask and a calculated shift value.
- `modify_memory_bits`: Iteratively modifies a byte in memory by applying bitwise OR or AND operations based on a bitmask and a table.
- `modify_bitmask_in_memory`: Iteratively modifies bits in a memory location by applying bitwise OR and AND operations based on values from a table.
- `apply_bitmask_and_update`: A routine consisting of 66 instructions that applies a bitmask to a buffer and updates a bounding box.
- `initiate_recursive_process`: Calls another routine with a starting index of zero and a calculated end index derived from a value at an offset.
- `pass_parameters_recursively`: Calls another routine using several values retrieved from a structure via offsets.
- `update_layer_parameters`: Writes a constant value to a memory location and then calls another routine using several values retrieved from a structure via offsets.
- `store_processed_coordinates`: Performs arithmetic on input values and stores the results into three consecutive byte locations in a global table.
- `iterate_coordinate_pairs`: Loops sixteen times to pass pairs of bytes extracted from a pointer to another routine.
- `forward_list_insertion`: The routine passes its argument directly to a linked list insertion helper and returns.
- `call_another_routine`: The routine calls another routine and then returns.
- `update_structure_members`: Saves a value from a pointer, calls another routine three times with various parameters, and restores the original value.
- `set_global_value`: The routine stores the value two into a global variable.
- `initialize_global_parameters`: Sets several global variables and performs arithmetic using 32-bit multiplication and signed division.
- `decrement_and_recurse_pointer`: Decrements a value pointed to by a parameter by six and calls itself recursively until the value reaches zero.
- `wait_for_zero`: The routine loops until a specific word in memory becomes zero.
- `nop_padding_sequence_2`: This routine is empty and performs no operations.
- `reset_hardware_parameters`: Calls two library routines and then writes specific values to several memory locations.
- `setup_interrupt_handler`: Initializes a structure with a pointer, a constant, and a callback routine, then performs further assignments to a global location.
- `clear_memory_block`: Clears several consecutive memory locations and a table of fifty words to zero.
- `setup_audio_dma`: Initializes several global values and calls library functions to configure audio and DMA parameters.
- `execute_initialization_sequence`: Calls multiple library functions and other routines in a specific sequence using values from global variables.
- `process_parameter_logic`: Updates several global variables and memory locations based on the values of two input parameters and the result of a recursive call.
- `populate_tree_table`: Recursively traverses a hierarchical structure to copy data and pointers into a destination table.
- `traverse_and_populate_tree_table`: This routine recursively traverses a hierarchical data structure and populates a destination table with pointers and associated metadata.
- `traverse_and_recursively_process_table`: This routine traverses a data structure using a pointer and an index, performing recursive calls to itself while storing results into a table.
- `update_audio_sample_rate`: Checks a status flag and a sample counter before calculating a new frequency value by multiplying a word by five.
- `trigger_audio_initialization`: Calls two routines to set up audio DMA and configure audio parameters.
- `configure_audio_parameters_2`: Calculates a value by multiplying a word by five and stores it in a memory location.
- `configure_audio_parameters`: Calculates a value by multiplying a word by five and stores it in a memory location.
- `reset_hardware_state`: Writes specific values to several memory locations and calls two library routines.
- `configure_audio_and_dma`: The routine stores three parameters into a global variable and then writes specific values to audio and DMA control registers.
- `update_resource_counters`: Checks bits in a register to determine an index, then increments a counter in a table.
- `process_interrupt_data`: Processes a data structure to increment counters and copy data from a source table to a destination, while interacting with hardware registers.
- `store_word_to_offset`: Stores a word into a memory location calculated by adding an offset to a value held in a register.
- `manage_file_table_2`: Iterates through a table of entries to perform various file operations including opening, writing, locking, and deleting files.
- `decrement_resource_index`: Decrements a short value at a memory offset after validating a resource index and calling a routine to close a resource.
- `branch_on_flag`: Checks a byte at a specific memory offset and calls another routine based on whether that byte is zero or non-zero.
- `branch_based_on_flag`: Checks a byte at a specific offset from a pointer and calls another routine based on whether that byte is zero or non-zero.
- `evaluate_resource_status`: Checks a global variable and performs conditional logic based on its value and other memory flags, potentially calling several routines to process data or handle errors.
- `conditionally_retry_file_operation`: Checks a global status flag and specific bitmasks in a parameter to decide whether to retry a routine.
- `update_conditional_state`: Checks a returned character value and calls another routine if the value is non-zero.
- `perform_conditional_read`: Calls another routine and may call it again if the returned value is negative.
- `execute_write_operation`: Calls another routine to perform a write operation and check the results.
- `retry_on_failure`: Calls another routine and checks its return value.
- `calculate_recursive_timer_interval`: Calculates a value by dividing a constant by a parameter and stores it in a memory location.
- `update_timer_reference`: Compares a value against a global variable and updates that variable based on a loop that checks against a calculated threshold.
- `adjust_value_by_four`: Adjusts a signed value by adding or subtracting four based on a comparison of its high bits.
- `verify_coordinate_transformations`: Iterates through a table of sixteen entries to perform various coordinate arithmetic operations based on an input mode.
- `loop_coordinate_processing`: Executes a loop sixteen times, passing a value from a table to another routine that performs arithmetic and stores the results.
- `process_coordinate_data`: Copies sixteen triplets of bytes from a source to a destination and performs coordinate arithmetic operations based on an input mode.
- `decode_bitstream_loop`: Performs a loop that processes a data stream by shifting values and calling another routine multiple times with varying offsets.
- `modify_pixel_data_loop`: Performs a loop that modifies data at a specific offset using bit shifts and arithmetic.
- `return_from_subroutine_5`: This routine performs no operations and immediately returns to its caller.
- `return_constant_one`: The routine returns the constant value one.
- `allocate_decompression_buffer`: Reads a four-byte integer from a pointer to determine a size, then calls routines to allocate and prepare a memory block of that size.
- `decompress_data_stream`: Expands a compressed data stream into a destination buffer using a dictionary-based decompression algorithm.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `convert_string_to_signed_integer_2`: Parses a string by skipping whitespace and checking for a sign before calculating a signed integer value from numeric characters.
- `convert_string_to_signed_integer`: Checks a string for a leading plus or minus sign and iterates through numeric characters to calculate a signed integer value.
- `parse_signed_integer`: Converts a string of characters into a signed integer by checking for a sign character and iterating through numeric digits.
- `convert_string_to_integer`: Parses a string of characters into a signed integer by iterating through the data and applying a multiplier.
- `return_immediately_2`: The routine consists of four instructions that perform no functional operations and return immediately.
- `parse_signed_integer_string`: Parses a string to extract a signed integer value by skipping whitespace and handling sign characters.
- `update_global_with_offset`: Calls another routine and stores the result plus a constant into a global variable.
- `store_high_word`: Stores the upper sixteen bits of a provided value into a global variable.
- `compare_two_byte_strings`: The routine compares two byte strings byte by byte.
- `perform_file_seek`: Executes a seek operation and handles potential errors by calling a routine to report I/O errors.
- `pass_argument_unchanged`: Passes its argument to another routine unchanged and returns.
- `manage_file_table`: Iterates through a table of entries to perform various file operations including opening, writing, locking, and deleting files based on specific flags.
- `read_file_data`: Performs a read operation and handles potential errors by calling a routine to report I/O errors.
- `clear_memory_block_2`: Fills a memory area with zeros using a loop.
- `return_from_subroutine_6`: The routine performs no operations and returns immediately to its caller.
- `allocate_and_initialize_resource`: Allocates a memory block and initializes its header with specific magic values and flags.
- `tiny_move_4`: Small routine of 1 instruction that copies d0 to d5.
- `parse_command_string`: Parses a source string into individual tokens by handling whitespace and quoted substrings.
- `copy_string_with_limit_2`: copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one.
- `append_string_to_buffer`: Finds the end of a destination string and copies a source string into that location.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `examine_and_extract_data`: Examines a file or resource, locks it, and extracts specific values from a structure into a provided pointer before unlocking.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `initialize_window_structure`: Opens a file, retrieves a library base, and populates a structure with data from a window or icon resource.
- `transfer_register_value`: Copies a long value from one register to another.
- `format_and_store_data`: Stores a parameter into a global location, calls a formatting routine, and then clears the stored pointer.
- `increment_and_store_byte`: increments a global integer and stores a byte at the location pointed to by a previously stored pointer.
- `parse_format_string_and_copy`: Parses a format string and copies a byte from a table into a destination pointed to by a global pointer.
- `execute_concatenated_recursive_calls`: This routine invokes two different subroutines, one of which returns a 64-bit value by concatenating a register with a return value, and another that performs a recursive call.
- `parse_format_string`: Parses a string containing format specifiers to extract and convert values from a source list.
- `perform_signed_division`: Calculates a signed division by determining the sign from two inputs, calling a division routine, and combining the result with the second input into a 64-bit value.
- `perform_signed_32bit_division`: Calculates a 32-bit division and applies the sign from the input to the quotient.
- `call_and_return_concatenated`: Calls another routine and returns a 64-bit value formed by concatenating the original contents of a register with the value returned by the call.
- `call_recursive_routine`: Calls another routine and then returns.
- `perform_32bit_division`: calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `execute_recursive_call`: A routine that calls itself and then returns.
- `manage_io_buffer_status_3`: This routine checks status flags within a structure to conditionally update buffer pointers and memory contents.
- `return_to_caller`: This routine is empty and performs no operations before returning to the calling function.
- `execute_unnamed_routine`: A routine consisting of fifteen instructions that calls another routine to increment and store a byte.
- `increment_and_store_byte_2`: Stores a byte from a structure into a memory location pointed to by a global variable and increments that variable.
- `clear_resource_structure_range`: Iterates through a range of memory locations and calls another routine to clear the fields within each structure.
- `clear_resource_structure_2`: Recursively traverses a structure and clears its fields, including zeroing out several longwords and a byte.
- `clear_resource_structure`: Clears a structure by zeroing its fields and potentially removing a node from a linked list or closing a file handle based on bit flags.
- `manage_io_buffer_status_2`: Checks status flags in a structure and performs conditional updates to buffer pointers and memory contents.
- `manage_io_buffer`: Updates buffer pointers and status flags based on the contents of an I/O structure.
- `manage_io_buffer_status`: Updates a status byte and manages a buffer by either copying a byte from a source or incrementing a pointer and counter based on specific conditions.
- `process_io_structure_3`: Updates a status byte and manages a buffer by either copying a byte from a source or incrementing a pointer and counter.
- `update_buffer_and_byte`: Updates a pointer and a counter in a structure, modifies a bit flag, and stores a byte from another location into the pointed memory.
- `search_and_clear_table_entry`: Iterates through a table of structures to find an entry with a zeroed byte and clears three consecutive 32-bit words in that entry.
- `search_and_clear_table`: Iterates through a table of structures to find an entry with a zeroed byte, then clears several fields in that entry.
- `clear_structure_fields`: Clears three consecutive 32-bit words in a structure pointed to by a register.
- `update_parameter_status`: Updates fields within a parameter structure based on the return value of another routine and modifies status bits in a byte.
- `update_status_flags`: Sets specific bits in a byte at a global offset and updates a long value based on the result of another routine.
- `iterate_linked_list`: Iterates through a linked list of pointers and calls another routine for each element until a null pointer is reached.
- `traverse_and_call_list`: iterates through a linked list of pointers and calls another routine for each element until a null pointer is reached.
- `insert_into_linked_list`: Inserts a new node into a linked list by updating pointers and assigning values from a global variable.
- `update_list_node`: Updates a linked list by setting a node's pointer to a value from a global variable and updating its successor.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `remove_linked_list_node`: Searches for a specific node in a linked list and removes it by updating the pointers of the surrounding nodes.
- `traverse_linked_list`: Iterates through a linked list to find a specific node and copies its value to a global variable or another location.
- `copy_pointer_to_list`: Copies a value from a pointer into either a global variable or another location based on a condition.
- `copy_list_head`: Copies a value from a pointer into a global variable and calls another routine.
- `check_status_index`: Validates an index against a range and checks a value in a table, returning a boolean result based on the outcome.
- `check_interactive_status`: Calls another routine to check a status and returns a boolean result.
- `return_zero`: This routine is empty and simply returns a value of zero to the caller.
- `handle_io_error`: Checks the result of a routine and, if zero, calls another routine to store an error value into a global variable.
- `return_zero_value`: This routine performs no operations and returns zero to the caller.
- `process_io_structure_2`: A routine consisting of nineteen instructions that calls routines to process an I/O structure and check results.
- `process_io_structure`: Checks a bitmask in a structure and recursively calls another routine to process data, updating a global status variable with the result.
- `process_io_result`: Checks the result of an operation and, if it indicates an error, calls an error routine and stores a value into a global variable.
- `return_from_subroutine_7`: This routine is an empty function that performs no operations before returning.
- `check_result_and_call`: calls another routine and, if a specific bit is set in the result, calls a third routine if a global short is non-zero.
- `chain_result_to_output`: Passes the result of a function call to a write wrapper followed by an output wrapper.
- `execute_callback`: Checks if a pointer stored in a global variable is non-zero and, if so, executes the routine at that address.
- `execute_cleanup_sequence`: Performs a series of conditional checks and calls various routines, including library functions and indirect function calls, based on values in a table or global variable.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_file_handle`: validates an index against a table, checks a flag, and calls another routine to close a resource before clearing the entry.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_date_stamp`: Wrapper that calls the dos library function DateStamp and returns its result to the caller.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_examine`: Wrapper that calls the dos library function Examine and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `execute_library_function`: calls a function via an offset from a pointer stored in a global variable.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `execute_indirect_function`: Retrieves a function pointer from a nested table structure and executes it.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_function`: Loads a library base from a global variable and calls a function at a specific offset.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `execute_indirect_function_pointer`: Retrieves a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `execute_indirect_function_2`: Retrieves a pointer from a table located at an offset from a global variable and executes the function at the resulting address.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_2`: Retrieves a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_162`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_3`: Retrieves a function pointer from a nested offset within a structure and executes it.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `invoke_indirect_function`: Retrieves a function pointer from a nested offset within a table and executes it.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_4`: Fetches a pointer from a table located at an offset from a base address and executes the function at that location.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_5`: Retrieves a function pointer from a nested offset within a structure and executes it.
- `execute_indirect_function_pointer_6`: Fetches a pointer from a nested offset within a structure and executes the function located at that address.
- `execute_indirect_call_2`: Performs an indirect call to a function pointer retrieved from a nested offset within a table.
- `execute_indirect_call_3`: Calls a function located at an offset within a structure pointed to by a table entry.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `execute_indirect_call_4`: Executes a function pointer found by traversing a pointer within a data structure.
- `execute_indirect_call_5`: Retrieves and executes a function pointer from a memory location offset from a table entry.
- `call_library_vector_324`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.

### Gobliins 2 - The Prince Buffoon (1992)(Coktel Vision)[cr SKR](Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Gobliins 2 - The Prince Buffoon (1992)(Coktel Vision)[cr SKR](Disk 2 of 3).adf:bootblock

- `copy_boot_code`: Copies a block of code to a new location and performs system initialization by calling library functions and modifying a byte in memory.
- `initialize_system_state`: Clears a block of memory, resets a range of hardware registers to zero, and waits for a specific bit in a global variable to change.
- `initialize_hardware_and_memory`: Clears a range of memory, resets hardware registers, copies data blocks, and waits for a specific bit in a global variable to change.
- `copy_data_blocks`: Iterates through a null-terminated string to copy blocks of data from a table to a calculated memory destination.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### Gobliins 2 - The Prince Buffoon (1992)(Coktel Vision)[cr SKR](Disk 3 of 3).adf:bootblock

- `copy_and_execute_payload`: Copies a block of data from the current location to a new memory address and then executes the code at that destination.
- `clear_memory_and_reset_hardware`: Clears a block of memory starting from a high address up to a specific limit and resets a range of hardware registers to zero.
- `initialize_hardware_and_clear_memory`: Clears a range of memory starting from a high address and zeroes out a block of hardware registers.
- `copy_data_blocks_2`: Copies multiple bytes of data from a source table to a destination address based on string characters.
- `return_empty_stub`: The routine consists of a single instruction, a return, and performs no work.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_infinite_loop`: The routine performs an infinite loop that prevents further execution.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `execute_infinite_loop_2`: The routine enters an infinite loop that prevents further execution of the calling code.
- `store_word_at_offset`: Stores a value from a register into a memory location calculated from a base and an offset.
- `store_word_via_pointer`: Stores a 16-bit value from a register into a memory location calculated by adding a register offset to a pointer held in another register.
- `store_word_via_pointer_2`: Stores a 16-bit value from a register into a memory location calculated by adding an offset to a pointer held in another register.
- `store_word_via_pointer_3`: Stores a 16-bit value from a register into a memory location calculated by adding an offset to another register's value.
- `execute_infinite_loop_3`: The routine enters an infinite loop that prevents further execution.
- `store_word_via_pointer_4`: Stores a 16-bit value from a register into a memory location calculated by adding an offset to a pointer.
- `store_word_to_memory`: Stores a word from a register into a memory location calculated by adding a register value to another register.
- `write_data_to_address`: Performs a memory write using a pointer and an offset stored in registers.
- `execute_infinite_loop_4`: Enters an infinite loop that never exits.
- `copy_value_to_pointer`: Writes a value from a source register to a destination address determined by register arithmetic.
- `loop_indefinitely`: Contains a continuous loop that prevents further execution.
- `trap_in_loop`: Runs a loop that repeats indefinitely.
- `stay_in_loop`: An infinite loop that traps the processor.
- `transfer_register_to_memory`: Moves data from a register to a memory address derived from other registers.
- `perform_memory_write`: Writes a value to a memory location using an offset and a pointer.
- `update_memory_via_registers`: Uses register values to calculate a destination and stores data there.
- `store_word_to_memory_2`: Stores a word from a register into a memory location calculated from a base address and an offset.
- `execute_infinite_loop_5`: The routine enters an endless loop that prevents further execution of subsequent instructions.
- `write_data_to_address_2`: Writes a value from a register to a destination address determined by adding an offset to a base register.
- `perform_memory_store`: Copies a value from a source register to a memory address derived from an offset and a base pointer.
- `execute_infinite_loop_6`: The routine enters an endless loop that prevents further execution of the current code path.
- `transfer_word_to_memory`: Moves a word from a register to a memory location using a calculated address.
- `trap_in_loop_2`: Runs an endless loop of instructions.
- `copy_value_to_offset`: Stores a register value into a memory address formed by a base and an offset.
- `update_memory_at_offset`: Writes a word to a memory location using an offset added to a base address register.
- `save_register_to_memory`: Stores a value from a register into a memory address calculated from a base and an offset.
- `store_word_via_pointer_5`: Stores a 16-bit value from a register into a memory location calculated by adding a register offset to another register's value.
- `execute_infinite_loop_7`: The routine enters an endless loop that prevents further execution of the current thread.
- `write_data_to_pointer`: Writes a value from a register to a memory address determined by a pointer and an offset.
- `loop_indefinitely_2`: Performs a continuous loop that traps the processor.
- `trap_in_loop_3`: Contains a loop that repeats indefinitely.
- `move_data_to_address`: Moves a value from a register to a destination address formed by two other registers.
- `run_infinite_loop`: Executes a loop that repeats without end.
- `transfer_register_to_memory_2`: Transfers a value from a register to a memory location using an offset and a base address.
- `perform_memory_write_2`: Writes a value to a memory address derived from a register and an offset.
- `enter_unending_loop`: Enters a loop that repeats forever.
- `execute_infinite_loop_8`: This routine enters an infinite loop that prevents further execution of the calling code.
- `execute_infinite_loop_9`: Enters a loop that never terminates.
- `execute_infinite_loop_10`: Contains an infinite loop structure.
- `execute_infinite_loop_11`: Runs a continuous loop without exit.
- `execute_infinite_loop_12`: Stays in a loop indefinitely.
- `store_word_to_memory_3`: Stores a value from a register to a memory location calculated from other registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_subroutine`: This routine is empty and performs no operations before returning to the caller.
