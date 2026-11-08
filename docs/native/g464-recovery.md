# Game 464 (Bargon Attack): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00464). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3604480 top-level bytes in 31 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 16 of 16 units identical (bytes and relocations), 65 raw-word lines |
| Functions named and described | pass, 557 of 557 named (0 author symbols), 557 of 557 described (61 by rule, 488 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 5331 of 5331 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 1013158 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 26556 of 26609 instructions identical in start and length, 5 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 629626 |
| code | 93064 |
| data | 2096886 |
| padding | 7688 |
| unused | 777216 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Bargon Attack (1992)(Coktel Vision)[cr FSN](Disk 1 of 4).adf:stack | 2 | 117 | 2 |
| suisse | 1 | 13 | 1 |
| german | 1 | 13 | 1 |
| ESPAGNE | 1 | 13 | 1 |
| Rename | 2 | 117 | 2 |
| qwerty | 1 | 13 | 1 |
| azerty | 1 | 13 | 1 |
| Bargon Attack (1992)(Coktel Vision)[cr FSN](Disk 1 of 4).adf:type | 2 | 117 | 2 |
| Bargon Attack (1992)(Coktel Vision)[cr FSN](Disk 1 of 4).adf:intro | 2 | 257 | 8 |
| Bargon Attack (1992)(Coktel Vision)[cr FSN](Disk 1 of 4).adf:echo | 2 | 117 | 2 |
| Bargon Attack (1992)(Coktel Vision)[cr FSN](Disk 1 of 4).adf:azerty | 1 | 13 | 1 |
| Bargon Attack (1992)(Coktel Vision)[cr FSN](Disk 1 of 4).adf:loader | 3 | 25796 | 534 |
| Bargon Attack (1992)(Coktel Vision)[cr FSN](Disk 1 of 4).adf:bootblock | 1 | 10 | 1 |
| Bargon Attack (1992)(Coktel Vision)[cr FSN](Disk 3 of 4).adf:DISK5.STK (raw code file, load address unknown) | 1 | 0 | 0 |
| Bargon Attack (1992)(Coktel Vision)[cr FSN](Disk 3 of 4).adf:DISK4.STK (raw code file, load address unknown) | 1 | 0 | 0 |
| Bargon Attack (1992)(Coktel Vision)[cr FSN](Disk 4 of 4).adf:DISK7.STK (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- bargon attack (1992)(coktel vision)[cr fsn](disk 1 of 4)/Bargon Attack (1992)(Coktel Vision)[cr FSN](Disk 1 of 4).adf:DISK1.STK: high-entropy data file (7.62 bits/byte), classified as packed data
- bargon attack (1992)(coktel vision)[cr fsn](disk 1 of 4)/Bargon Attack (1992)(Coktel Vision)[cr FSN](Disk 1 of 4).adf:INTRO.STK: high-entropy data file (7.69 bits/byte), classified as packed data
- bargon attack (1992)(coktel vision)[cr fsn](disk 2 of 4)/Bargon Attack (1992)(Coktel Vision)[cr FSN](Disk 2 of 4).adf:DISK2.STK: high-entropy data file (7.66 bits/byte), classified as packed data
- bargon attack (1992)(coktel vision)[cr fsn](disk 3 of 4)/Bargon Attack (1992)(Coktel Vision)[cr FSN](Disk 3 of 4).adf:DISK4.STK: high-entropy data file (7.66 bits/byte), classified as packed data
- bargon attack (1992)(coktel vision)[cr fsn](disk 3 of 4)/Bargon Attack (1992)(Coktel Vision)[cr FSN](Disk 3 of 4).adf:DISK5.STK: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- bargon attack (1992)(coktel vision)[cr fsn](disk 3 of 4)/Bargon Attack (1992)(Coktel Vision)[cr FSN](Disk 3 of 4).adf:DISK4.STK: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- bargon attack (1992)(coktel vision)[cr fsn](disk 4 of 4)/Bargon Attack (1992)(Coktel Vision)[cr FSN](Disk 4 of 4).adf:DISK7.STK: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- bargon attack (1992)(coktel vision)[cr fsn](disk 2 of 4)/Bargon Attack (1992)(Coktel Vision)[cr FSN](Disk 2 of 4).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- bargon attack (1992)(coktel vision)[cr fsn](disk 3 of 4)/Bargon Attack (1992)(Coktel Vision)[cr FSN](Disk 3 of 4).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- bargon attack (1992)(coktel vision)[cr fsn](disk 4 of 4)/Bargon Attack (1992)(Coktel Vision)[cr FSN](Disk 4 of 4).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- 3 never-loaded region(s), 1013158 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- Bargon Attack (1992)(Coktel Vision)[cr FSN](Disk 1 of 4).adf:intro

## Copy protection found

- none found

## Functions


### Bargon Attack (1992)(Coktel Vision)[cr FSN](Disk 1 of 4).adf:stack

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_memory_block`: Copies sixteen bytes from a source address to a destination address via a loop.

### suisse

- `terminate_execution`: The routine is empty and contains no executable instructions.

### german

- `initialize_program_execution`: This routine serves as the initial entry point for the executable.

### ESPAGNE

- `terminate_execution_2`: The routine is empty and contains no valid instructions.

### Rename

- `update_linked_list_and_copy_data`: Traverses a linked list to find the maximum value in a specific field and then updates a target structure with this value and a generated sequence.
- `copy_data_block`: Copies sixteen words from a source location to a destination via a loop and then calls another routine.

### qwerty

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### azerty

- `terminate_execution_3`: The routine is empty and contains no valid executable instructions.

### Bargon Attack (1992)(Coktel Vision)[cr FSN](Disk 1 of 4).adf:type

- `update_linked_list_and_copy_data_2`: Traverses a linked list to find the maximum value in a specific field and then updates a buffer with a header and a sequence of incrementing values.
- `copy_memory_block_and_call`: Copies sixteen bytes from a memory location offset from a register to a destination pointed to by a stack pointer, then executes a function pointer.

### Bargon Attack (1992)(Coktel Vision)[cr FSN](Disk 1 of 4).adf:intro

- `setup_copper_and_restore_state`: Saves several hardware registers, sets interrupt and DMA masks, points the copper at a list, waits on a joystick port line for a short count, then restores the saved DMA, interrupt and disk values and returns.
- `copy_table_with_word_stride`: Writes four control words into a block, then copies a table of 142 words into the block, stepping the destination by eight words each time.
- `fill_strided_rows_with_words`: For thirty-two rows, waits on a status bit, then stores 143 words returned by another routine into a destination at a stride of eight words, choosing one of two bases from a flag byte.
- `fill_strided_rows_with_words_2`: Iterates thirty-two times to wait for a hardware status bit, then populates a destination buffer with 143 words from another routine using a stride of eight words and a conditional base address.
- `interpolate_rgb_colour`: Blends two twelve-bit colour values channel by channel, each split into three 4-bit parts, using a signed fraction, and returns the blended colour packed back into twelve bits.
- `init_object_control_fields`: Stores a table pointer chosen by a flag byte and writes six fixed control words into the object record.
- `reset_object_and_toggle_flag`: Calls a sequence of routines, stores three pointer fields and several control words in an object record, then flips a flag byte.
- `lay_out_text_characters`: Sums a width per character from a table to measure a string, then writes a glyph pointer and position words for each character into the object record, advancing across the line.

### Bargon Attack (1992)(Coktel Vision)[cr FSN](Disk 1 of 4).adf:echo

- `update_linked_list_and_copy_data_3`: Traverses a linked list to find the maximum value in a specific field and then updates a buffer with a header and a sequence of incrementing values.
- `copy_sixteen_bytes`: Copies sixteen bytes from a source pointer to a destination pointer and then calls another routine.

### Bargon Attack (1992)(Coktel Vision)[cr FSN](Disk 1 of 4).adf:azerty

- `initialize_system_state`: The routine is empty and performs no operations.

### Bargon Attack (1992)(Coktel Vision)[cr FSN](Disk 1 of 4).adf:loader

- `clear_memory_and_execute_library_function`: Zeros out a block of memory using a loop and then performs conditional calls to functions retrieved from a library base stored in a global variable.
- `initialize_language_and_resource_tables`: Initializes language-specific string structures and clears various coordinate and status tables.
- `parse_resource_data_stream`: Parses a structured data stream by reading length-prefixed blocks and populating internal tables.
- `execute_complex_callback_chain`: Executes a sequence of routines derived from multiple tables, including loops that check bit flags and specific pointer-based calls.
- `initialize_short_array`: Fills a series of memory locations with alternating values of all ones and zero, then clears a single word.
- `check_callback_flag`: checks if a value at a global memory location is non-zero and calls another routine if it is.
- `initialize_entity_attribute_table`: Allocates a memory block for a collection of entities and populates their coordinate and state data through a loop.
- `clear_multiple_table_entries`: Checks three global table entries and, if they are non-zero, calls routines using those pointers before clearing the entries to zero.
- `register_and_initialize_resource_entry`: Searches a table for an existing entry matching a specific identifier, or allocates a new slot if none is found.
- `process_indexed_list_nodes`: Iterates through a table of nodes, executing specific routines based on bitmask flags and decrementing counters stored in a secondary table.
- `parse_hierarchical_geometry_data`: Recursively traverses a structured data table to calculate bounding box dimensions and spatial properties.
- `execute_recursive_calls`: Performs a sequence of recursive calls where the result of one call is passed as an argument to the next, eventually concatenating two results and passing them to a final call.
- `upsert_and_initialize_table_entry`: Searches a table for an entry matching a specific value; if found, it updates a cumulative total in a global variable.
- `process_indexed_list_nodes_alt`: Iterates through a table of nodes, executing routines based on bitmask flags and decrementing counters, then clearing the table entry and setting a count to zero.
- `execute_repeated_routines`: Executes a routine once, followed by two separate loops that each execute a routine ten times.
- `clear_memory_tables`: Clears several memory locations and fills multiple arrays with zero or all ones.
- `store_postfix_words`: Fetches three consecutive words from a source and stores them into three consecutive locations in a global variable.
- `execute_recursive_call`: Calls another routine, passes its return value as an argument to a subsequent call of that same routine, and then returns.
- `initialize_entity_tables`: Fetches a word from a location pointed to by a global variable and then resets global state variables and populates an array of structures with default spatial coordinates and status flags.
- `process_word_pointer`: fetches a word from a location pointed to by a global variable and passes that word to another routine.
- `validate_and_update_buffer_metadata`: Compares fetched dimensions and area against stored values in a structure, updating the metadata and reallocating or recalculating buffer pointers if a mismatch is detected.
- `store_indexed_bytes`: Fetches a signed word and three bytes, then stores the bytes into a table at offsets determined by the signed word.
- `clear_indexed_bytes`: Fetches a signed word and stores three specific byte values into a table at offsets determined by that word.
- `process_complex_data`: Performs multiple data fetches and arithmetic operations to calculate a value and store it in a memory location.
- `update_coordinate_data`: Calculates a new value based on a pointer and an offset, stores it in a local variable, and calls other routines with modified parameters.
- `pass_argument_to_helper`: This routine receives an argument and passes it directly to another routine before returning.
- `search_and_update_table`: Searches through a table for a matching value and, if found, returns a modified index.
- `process_indexed_data_table`: Uses an index to access a table, checks a character flag to determine which routine to call, and then clears the table entry.
- `copy_postfix_data`: Calls two routines to evaluate expressions and then copies a sequence of words from a source table to a destination table.
- `store_stack_bytes`: Stores three bytes from the stack into a structure located via a pointer in a global variable.
- `fetch_and_store_words`: Reads two words from memory and stores their combined result into a table at a location pointed to by a global variable.
- `return_immediately_3`: This routine is empty and performs no operations before returning to its caller.
- `check_table_value_and_call`: Fetches a value from a table and, if the value is non-zero, calls another routine.
- `call_clear_table_entry`: Calls a routine that clears a table entry and then calls another routine.
- `store_fetched_words`: Fetches three consecutive words from a location pointed to by a global variable and stores them into a data structure.
- `dispatch_table_entries`: Retrieves a value from a table and calls another routine using that value.
- `execute_table_callback`: Fetches an index, checks a table at that index for a non-zero value, calls the routine at that location, and then clears the table entry.
- `calculate_table_product`: Retrieves a pointer from a table and calculates a product by multiplying three values found at specific offsets from that pointer.
- `calculate_product_from_table`: Calculates a value by multiplying a short from a table by a value at a pointer and another short from the same table location.
- `evaluate_complex_expression`: Executes a sequence of calls to process data, performs arithmetic on results, and updates a flag in a structure based on comparison results.
- `dispatch_bytecode_logic`: Executes a series of conditional checks and bytecode dispatches, updating various memory locations and global variables based on the results.
- `update_conditional_state_2`: Performs a series of conditional checks and updates a global variable based on the results of other routines.
- `store_evaluated_result`: Executes a sequence of operations to obtain values and stores them into a destination table at an offset determined by a previous result.
- `process_bytecode_logic`: Calls several routines to process data and performs conditional logic to update a flag in a structure based on the outcome of a comparison.
- `execute_conditional_bytecode`: Performs bytecode dispatch and conditional logic, including comparing values and resetting a status flag in a structure if a match is found.
- `compare_and_reset_value`: Stores a returned value into a memory location and, if that value matches another stored value, writes zero to a specific offset in a destination buffer.
- `copy_conditional_data`: Checks a byte at an offset, and if non-zero, copies data between two memory locations based on a calculated offset.
- `copy_data_between_buffers`: Calls a routine to copy data from one memory location to another based on offsets from a register.
- `push_pop_context_stack`: Saves a set of state variables and pointers into a table indexed by a counter, then increments that counter.
- `remove_linked_list_element`: Removes an entry from a linked list by updating table pointers and decrementing a counter.
- `process_data_stream`: Increments a pointer, modifies a hardware register, and conditionally copies bytes from a source into a buffer based on a bit test.
- `unpack_string_data`: Reads a length byte from a pointer and copies that many bytes into a local stack buffer, or executes alternative routines if a specific bit is set.
- `unpack_data_stream`: Reads a byte from a pointer, increments that pointer, and then uses the byte's value to write a sequence of bytes into a destination buffer.
- `unpack_compressed_data_stream`: Iterates through a source table to extract bytes into a destination buffer using an index derived from the current byte value.
- `reset_global_state`: Sets multiple memory locations and words to zero or all ones.
- `initialize_game_state_structures`: Initializes a series of data structures by setting specific memory offsets to zero or fixed values and populating a table with repeating byte patterns.
- `increment_and_store_value`: Increments a global counter, sets a word to all ones, and conditionally stores a longword into a global variable.
- `dispatch_by_type`: Calculates a length, null-terminates a byte, and uses a switch statement to copy varying amounts of data based on a global value.
- `process_conditional_data_2`: Performs a recursive search and conditional reads to combine bytes into a word, then stores the result in a global variable.
- `format_and_store_string`: Checks specific bytes in a string, potentially appends a character, and stores a resulting longword into a global variable.
- `reset_and_reinitialize_data_structures`: Clears a 4x4 grid of memory with all ones, zeros out several tables and arrays, and updates various global pointers and status flags.
- `initialize_and_configure_state`: Sets a value in a global table and clears several memory offsets to zero.
- `parse_bytecode_stream`: Parses a custom bytecode stream using a recursive descent approach.
- `execute_bytecode_interpreter_3`: Processes a custom bytecode stream using a recursive descent mechanism to perform arithmetic, bitwise logic, and stack-based operations.
- `execute_bytecode_interpreter_4`: Processes a custom bytecode stream using a recursive descent mechanism to perform arithmetic, bitwise logic, and stack-based operations.
- `execute_bytecode_interpreter_5`: Processes a custom bytecode stream using a recursive descent mechanism to perform arithmetic, bitwise logic, and stack-based operations.
- `execute_bytecode_interpreter_6`: Processes a custom bytecode stream using a stack-based architecture to perform arithmetic, bitwise logic, and pointer manipulations.
- `execute_bytecode_interpreter`: Processes a custom bytecode stream using a recursive descent mechanism to perform arithmetic, bitwise logic, and stack-based operations.
- `execute_bytecode_interpreter_2`: Parses and executes a custom bytecode stream using a recursive descent approach.
- `decode_compressed_data`: Iterates through a data buffer to decompress or expand values based on control characters.
- `process_control_characters`: Parses a buffer for specific control characters and performs corresponding operations such as string manipulation, bitwise logic, or recursive calls to another routine.
- `execute_bytecode_instructions`: Parses a custom bytecode stream using a recursive descent approach to perform arithmetic, bitwise logic, and string manipulations.
- `nop_padding_gap`: This routine is empty and contains no executable instructions.
- `evaluate_postfix_expression_4`: Parses a stream of data using a switch statement to perform arithmetic operations and stack manipulations.
- `evaluate_postfix_expression_3`: Processes a postfix expression and updates a value in a table by shifting it left and adding a result from another routine.
- `evaluate_postfix_expression_2`: Processes a stream of data using a stack-based postfix notation system.
- `evaluate_postfix_stack`: Processes a stream of data using a stack-based postfix notation system.
- `evaluate_postfix_expression`: Processes a stream of data as a postfix notation stack machine.
- `execute_postfix_logic`: This routine invokes a stack-based evaluator to process a stream of data formatted in postfix notation.
- `adjust_value_range`: Checks if a value is within a specific range and subtracts a constant if it is.
- `dispatch_bytecode_commands`: Parses a data stream and uses a switch statement to write bytes, words, or longwords into a table based on specific command codes.
- `update_data_table_2`: updates a table entry by writing a byte, word, or longword based on a switch statement and increments a counter.
- `update_data_table`: Updates a table entry by writing a longword or byte based on a switch statement and increments a counter.
- `evaluate_and_increment_counter`: Processes postfix expressions and parameter sets before incrementing a value in a global table.
- `parse_resource_stream`: Reads a stream of data from a pointer and interprets command bytes to perform specific operations.
- `parse_bytecode_control_stream`: Reads a stream of data into a local buffer until a delimiter is encountered.
- `process_bytecode_control_stream`: Iterates through a data stream to identify control opcodes and execute corresponding subroutines.
- `decode_bytecode_stream`: Iterates through a data stream, extracting variable-length byte sequences into a buffer until a terminator is reached.
- `parse_bytecode_stream_2`: Iterates through a data stream, extracting variable-length byte sequences into a buffer until a terminator is reached.
- `parse_data_stream`: Iterates through a data stream, copying bytes to a buffer and executing specific routines based on encountered byte values.
- `increment_global_counter`: Increments a longword value stored at a location relative to a pointer.
- `rotate_rgb_element_list`: Rotates a list of three-byte elements within a buffer by moving the first element to the end or the last element to the beginning based on a direction flag.
- `shift_rgb_element_list`: Shifts a list of three-byte elements within a buffer either forward or backward.
- `shift_list_elements_2`: Shifts three-byte elements within a buffer forward by copying data from one index to the next in a loop.
- `shift_list_elements_backward`: Shifts three-byte elements within a buffer backward by copying data from one index to the previous one in a loop.
- `shift_list_elements`: Moves three-byte elements within a buffer by decrementing an index in a loop.
- `combine_arithmetic_results_2`: Calculates a difference between two values, updates several fields in a structure, and performs conditional masking and storage of a parameter.
- `normalize_and_recursively_store_value`: Compares a word at an offset within a structure against several constants to assign a specific replacement value, or masks the word to its lower byte.
- `update_masked_value`: Checks a word against specific constants, applies a mask if necessary, and stores the result in a location pointed to by a global variable.
- `mask_and_store_word`: Checks a word at a memory location against specific constants and masks it with a byte value before storing the result in a pointer location.
- `mask_and_store_value`: Masks a word with a byte mask and stores the result in a pointer location.
- `update_global_value`: Masks a value with a byte mask, stores it in a global pointer, and calls another routine if the value is non-zero.
- `aggregate_status_bitmasks_2`: Evaluates a series of boolean flags and bitwise conditions from a data structure to construct a bitmask.
- `update_status_bitmasks`: Aggregates boolean flags from various memory locations into two separate status bytes using bitwise OR operations.
- `update_status_values`: Calls a routine to retrieve a word, stores it in a global location, and conditionally clears a byte in a global variable based on a comparison.
- `process_nested_data`: Recursively processes a hierarchical data stream by decoding nibbles and evaluating conditional branches.
- `call_other_routine`: This routine calls another routine and then returns.
- `aggregate_status_bitmasks`: Recursively processes a data stream to evaluate conditional flags and bitwise conditions.
- `update_status_bitmasks_2`: Calls another routine and conditionally executes a second routine before clearing a byte in a global location.
- `process_object_collision_hierarchy`: Iterates through a table of object data to identify valid entities based on specific bitmask and range criteria.
- `call_collision_hierarchy_processor`: This routine serves as a wrapper that calls a procedure to iterate through a table of object data to identify valid entities based on specific bitmask and range criteria.
- `parse_data_stream_table`: Iterates through a data stream to populate a table of structures by decoding a sequence of control bytes and postfix expressions.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `update_table_from_counter`: Reads a value from a global counter to populate several entries in a table and performs arithmetic on those entries.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d7.
- `process_byte_and_calculate`: Fetches a byte from a global pointer, performs bitwise operations and arithmetic to calculate a value, and stores the results in a data structure.
- `process_complex_data_2`: Calls several routines to retrieve values and then conditionally updates multiple global memory locations based on the results of those calls.
- `calculate_and_update_values`: Performs several calculations and calls other routines to process data.
- `execute_two_routines`: Calls two separate routines in succession.
- `store_postfix_results`: Stores two results from a postfix evaluation into a table and calls another routine if a specific byte is non-zero.
- `execute_recursive_expression`: Calls another routine and uses its return value to perform an arithmetic operation with a value retrieved from a global table.
- `execute_bytecode_interpreter_7`: Iterates through a stream of bytes, using each value as an opcode to recursively call itself or other routines.
- `decode_nibble_stream_2`: Decodes a stream of data by extracting nibbles to determine control flow and branching logic.
- `execute_nibble_opcode_stream`: Processes a stream of nibbles to drive a virtual machine or interpreter.
- `execute_nibble_encoded_bytecode_2`: Interprets a stream of nibble-encoded data to perform conditional branching and control flow operations.
- `process_nibble_encoded_stream`: Iterates through a data stream by extracting high and low nibbles from bytes.
- `decode_nibble_stream`: Extracts nibbles from a data stream to determine control flow and execute specific subroutines.
- `execute_virtual_bytecode`: Interprets a custom instruction stream by fetching opcodes and decoding their length and operands.
- `execute_bytecode_stream`: Processes a stream of encoded instructions by extracting opcodes and determining instruction lengths via nibble manipulation.
- `execute_bytecode_stream_2`: Decodes a stream of command bytes and executes corresponding subroutines based on the instruction length and opcode.
- `execute_bytecode_stream_3`: Parses a stream of nibbles to determine instruction lengths and opcodes, then dispatches corresponding subroutines based on the decoded values.
- `decode_bytecode_stream_2`: Extracts nibbles from a data stream to determine instruction length and opcode values.
- `execute_nibble_encoded_bytecode`: Decodes a stream of nibble-based instructions to drive a virtual machine or interpreter.
- `process_encoded_instruction_stream`: Decodes a stream of data by extracting nibbles to drive a state machine that manages control flow, arithmetic operations, and recursive routine calls.
- `parse_nibble_encoded_data`: Decodes a stream of data by extracting nibbles and using them to determine control flow or branching logic.
- `traverse_recursive_data_stream`: Iterates through a data stream by comparing a pointer against a termination address.
- `process_game_state`: Updates a global counter based on memory values, processes an input sequence from a table, and performs a series of conditional checks on a returned value to match specific constants.
- `loop_view_rectangles`: Repeatedly calls another routine to update bounding rectangles until a specific value is returned as zero.
- `process_resource_data_stream`: This routine manages data loading by allocating decompression buffers, calculating string lengths, and combining bytes into words.
- `update_bounding_box_coordinates`: Updates geometric coordinate values in a data structure by applying offsets and performing recursive calculations to determine bounding box dimensions.
- `process_collision_rectangles`: Recursively partitions rectangles and calculates bounding boxes.
- `recursively_calculate_bounding_rectangles`: This routine performs recursive geometric calculations to determine bounding box coordinates based on input parameters and table values.
- `merge_two_strings`: Combines two byte strings by copying parts of each into a destination buffer based on their lengths.
- `copy_string_segment`: Copies a portion of one null-terminated string into another based on a specified length and offset.
- `merge_byte_strings`: Merges two null-terminated byte strings by calculating lengths and shifting characters.
- `combine_bytes_to_word`: Takes two consecutive bytes from a memory location and combines them into a single unsigned sixteen-bit value.
- `fetch_and_advance_word`: Reads a word from a location pointed to by a global variable and increments that pointer by two bytes.
- `check_status_flags`: Checks a byte and two short values against specific constants and calls another routine if conditions are met.
- `update_structure_pointers`: Writes two values to specific offsets within a structure and conditionally updates a pointer.
- `update_collision_bounding_box`: Calculates the intersection or union of two rectangular areas and updates a list of active collision volumes.
- `process_conditional_structure`: Checks various status flags and, if specific conditions are met, either iterates through a table to call another routine or performs a sequence of calls to update structure members and reset flags.
- `calculate_clamped_dimensions`: Computes dimension values by comparing stored shorts against values in a table and applying limits.
- `update_clamped_view_rectangles`: Updates a bounding rectangle by comparing its dimensions against a set of constraints and adjusting them to minimum thresholds.
- `initialize_table_entries`: Sets a word at a specific offset to zero and then loops through a table to set a word at a fixed offset within each entry to all ones.
- `process_data_table`: Iterates through a table of structures, performing postfix evaluations and storing four calculated values into each entry.
- `find_empty_table_slot`: iterates through a table to find an entry where a specific field equals negative one and populates it with provided parameters.
- `search_and_copy_table_entry`: Increments a counter and iterates through a table until a specific value is not found.
- `update_table_on_match`: Iterates through a table and sets a word at a fixed offset within an entry to all ones if a specific value matches the entry's first word.
- `enumerate_table_entries`: Iterates through a table to count specific entries, updates a global index, and copies data into a structured memory area.
- `update_list_from_table`: Decrements a counter in a global structure and uses the new value to fetch three words from a table.
- `process_data_block`: The routine performs a series of operations over one hundred instructions to manipulate data.
- `register_parameter_set`: Stores two sets of parameters into a table and calculates a new value to be processed by another routine.
- `decrement_and_retrieve_table_data`: Decrements a global counter and, if non-zero, retrieves four words from a table based on the new counter value to store in a data structure.
- `execute_nested_data_structures`: Recursively processes a hierarchical data stream by decoding nibbles and evaluating conditional branches.
- `perform_no_operation_2`: The routine is empty and performs no actions.
- `process_string_data`: performs character substitution on a string using a lookup table and manipulates whitespace and string segments.
- `populate_sprite_attribute_table`: Iterates through a loop to calculate and store coordinate and attribute data into a structured memory table.
- `call_sprite_attribute_population`: This routine calls another routine that iterates through a loop to calculate and store coordinate and attribute data into a structured memory table.
- `return_to_caller`: This routine performs a single return instruction to the calling function.
- `perform_no_operation`: This routine is empty and performs no operations.
- `perform_empty_jump`: This routine contains no executable instructions and immediately returns to the caller.
- `forward_argument_to_helper`: Passes its argument straight to a helper and returns.
- `pass_argument_to_memory_manager`: Passes its single argument directly to a routine that manages memory state and returns.
- `clear_table_entry`: Calls a routine using a pointer retrieved from a table and then sets that table entry to zero.
- `calculate_and_store_parameters`: Combines input parameters into new values, calls a routine to calculate them, and stores the result in a table.
- `dispatch_bytecode_result`: Executes a bytecode interpreter and returns a value, then uses a switch statement to store a specific constant or a value from a global variable into a destination pointed to by a parameter.
- `store_and_return_byte`: Stores a word from a global variable into a location pointed to by a pointer in a register and returns a byte from another location.
- `load_word_from_pointer`: Reads four bytes from a memory location pointed to by a parameter and returns them as a single longword.
- `calculate_offset_value`: Computes a value based on a base offset and a multiplier, potentially performing a table lookup.
- `increment_and_shift`: Increments a global pointer by two and updates it by adding a bit-shifted value.
- `recursively_initialize_parameter`: Recursively calls itself while either negating a parameter or passing it through, based on a comparison of a value in a structure.
- `traverse_parameter_tree`: Iterates through a table of structures to find a matching identifier.
- `initialize_entity_attribute_tables`: Resets a series of global state variables and populates an array of structures with default spatial coordinates, status flags, and boundary values.
- `update_object_properties`: Determines an index based on bit flags, populates a table with values from a postfix evaluation, and updates memory via a pointer.
- `update_object_property_from_tree`: Iterates through a hierarchical table to find a matching identifier and updates an object's properties.
- `store_two_values`: Stores two consecutive results from a postfix evaluation into a table.
- `recalculate_bounding_box_dimensions`: Iterates through a collection of geometric entities to update a global bounding box by comparing current coordinate extents against new object boundaries.
- `process_sprite_collision_data`: Iterates through a list of data structures to calculate spatial offsets and update bounding box coordinates.
- `execute_postfix_operations`: Processes a stream of data using a stack-based postfix notation system, word fetching, and collision data processing.
- `populate_data_from_postfix`: Evaluates multiple postfix expressions to retrieve values and stores the resulting unsigned integers into a destination buffer at offsets determined by the results of the evaluations.
- `update_table_values`: Evaluates postfix expressions and uses the results to store multiple values from a table into a global array.
- `initialize_and_update_entity_arrays`: Initializes multiple parallel arrays with default values and then iterates through a table of entity data to update their spatial coordinates and properties.
- `update_entity_state_and_physics`: Updates an entity's properties by reading from a data stream and applying transformations based on specific character flags.
- `copy_variable_data`: Copies a word, longword, or byte from a source location to a destination based on a size parameter.
- `reformat_and_populate_tile_data`: Calculates a new memory offset based on screen dimensions and performs a nested loop to reorder bytes from a source buffer into a destination table.
- `execute_nested_table_routines`: Iterates through a table to execute up to one hundred nested routines per entry, then processes a second table based on a bit flag.
- `clear_memory_block_2`: Fills a four-by-four byte area with the value two-hundred-fifty-five.
- `store_bytes_from_table`: Evaluates three postfix expressions and uses the result of the third to index into a table, storing two consecutive bytes from that table into a destination buffer.
- `update_table_entry_2`: Calls another routine and uses its return value to calculate an offset into a table, then stores a constant byte into a memory location reached through multiple pointer dereferences.
- `count_odd_indices`: Iterates from zero up to a value provided in a parameter and counts how many odd indices exist in that range.
- `update_entity_state_and_bounds`: Updates an entity's spatial data and bounding box coordinates by retrieving values from a lookup table and performing arithmetic calculations.
- `parse_expression_tree`: Recursively traverses and parses a data structure to build or evaluate a tree based on postfix notation.
- `update_structure_fields`: Copies two bytes from one location to another within a structure and updates several fields based on the results of various table searches and coordinate calculations.
- `calculate_grid_coordinate_offsets`: Calculates grid-based indices by dividing global values by scaling factors and performs a search to find the nearest valid entry within a two-dimensional data table.
- `fetch_byte_from_calculated_offset`: Calculates an offset using two halves of a parameter and returns a byte from that calculated position in a global table.
- `validate_grid_movement`: Checks if a movement from one coordinate to another is valid within a 2D grid structure.
- `find_minimum_index_a`: Searches a table for an entry that satisfies specific range conditions and stores the index in a specific offset.
- `find_minimum_index_b`: Searches a table for an entry that satisfies specific range conditions and stores the index in a different offset.
- `navigate_grid_coordinates`: Adjusts coordinate parameters based on return values from another routine while checking boundary conditions.
- `call_coordinate_navigation`: Calls a routine to navigate grid coordinates and subsequently modifies coordinate parameters based on the returned values while performing boundary checks.
- `search_grid_range`: iterates through a range of values to find and update a specific index based on values in a table.
- `process_parameter_table`: Iterates through a table to extract and compare values, calling other routines to update global parameters and hardware state based on the results.
- `update_entity_state_and_coordinates`: Processes input data to update a structure's coordinate and status fields.
- `process_parameter_table_recursively`: Iterates through a table of parameters to extract and compare values, updating global state and hardware parameters based on the results.
- `calculate_date_based_offset`: Retrieves the current date and performs a series of arithmetic loops and table lookups to derive a value.
- `load_byte_sequence`: Retrieves a sequence of bytes from a memory location and stores them into a provided buffer.
- `update_date_derived_data`: Calls a routine to calculate a date-based offset and then stores modified values from a memory location into a provided pointer.
- `copy_interleaved_data_blocks`: Copies data from a source buffer into five separate destination buffers using an interleaved pattern.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `return_from_subroutine_7`: This routine is empty and performs only a return instruction.
- `return_from_subroutine_2`: This routine performs no operations and immediately returns to the caller.
- `return_immediately_5`: This routine is empty and performs no operations before returning to the caller.
- `return_from_subroutine_3`: The routine performs no operations and immediately returns to its caller.
- `update_pointer_table`: Calls another routine to obtain a pointer and then populates a structure at that location with a value and several bytes from other memory locations.
- `decrement_and_recurse`: Subtracts four from a value pointed to by a parameter and then calls itself twice using the updated value and the original pointer.
- `allocate_memory_block`: Searches a linked list of free memory blocks to find a suitable segment based on a requested size.
- `unlink_list_node`: Decrements a counter in a table and removes an element from a linked list by updating pointers and status bits.
- `manage_linked_list_node`: Decrements a value in a table and manipulates a linked list by updating pointers and setting status bits.
- `increment_and_recurse_counter`: Increments a value at a memory location and performs a bitwise NOT operation on a word via a pointer.
- `pass_through_argument`: Passes an argument to another routine and returns the result.
- `manage_memory_state`: modifies a bit in a global variable, calls another routine, and performs conditional logic based on the returned value while restoring the original bit state.
- `update_elapsed_time_accumulator`: Calculates a time delta by subtracting a stored value from a current timer reading and updates a global accumulator if the stored value is zero.
- `combine_arithmetic_results`: Calls another routine and combines its result with a divisor using bitwise shifts and modulo operations.
- `fill_memory_block`: Fills a memory area with a specific byte value for a specified number of iterations.
- `wait_for_timer_interval`: Repeatedly calls another routine until the difference between two returned values meets or exceeds a specified threshold.
- `compute_absolute_value`: Checks if the lower two bytes of a 32-bit parameter are less than one.
- `ensure_positive_value`: Returns the absolute value of a passed integer.
- `negate_memory_value`: Loads a 32-bit value from a memory location offset from a register and returns its negation.
- `select_maximum_word`: Compares the lower two bytes and the upper two bytes of a 32-bit parameter.
- `return_minimum_value_alt`: Compares the upper two bytes and the lower two bytes of a 32-bit parameter.
- `return_minimum_value`: Compares two integers and returns the smaller of the two.
- `dereference_offset_pointer`: Returns a 32-bit value stored at an offset from a pointer held in a register.
- `convert_string_case`: Iterates through a null-terminated string and subtracts 32 from any character within the range of backtick to curly brace.
- `loop_until_zero`: Repeatedly calls another routine and stores its results in local buffers until a specific value reaches zero.
- `traverse_linked_list`: Traverses a linked list of 32-bit values and returns the largest difference between a pointer and a value marked with a high bit.
- `traverse_pointer_chain`: follows a chain of pointers and calculates a value by checking a bitmask at each step.
- `process_pointer_chain`: Traverses a chain of pointers, performing arithmetic on a value based on a bitmask check at each step.
- `execute_empty_routine`: This routine performs no operations and returns immediately.
- `call_wrapper_routine`: Calls another routine and returns.
- `execute_indirect_call`: Calls another routine via a pointer retrieved from a data structure and returns.
- `forward_call_to_routine`: Calls another routine with no arguments and returns, acting as a simple pass-through.
- `log_system_error`: Formats and outputs a system error message using a provided error code and string.
- `check_empty_stub`: The routine contains four instructions but performs no observable work and returns zero, acting as an empty stub that always returns zero.
- `branch_on_value_threshold`: Checks a value at an offset from a pointer and calls another routine with different arguments depending on whether the value is below a threshold.
- `allocate_memory_blocks`: calculates an offset, performs a loop to find a valid address, and then initializes two new memory blocks.
- `initialize_ui_and_language_settings`: Initializes language-specific string structures and clears coordinate tables.
- `execute_sequence_of_calls`: Performs a series of indirect calls to various routines and library functions, including a conditional block that clears a series of consecutive memory words.
- `compare_and_recurse`: Compares a string from a provided pointer against a global variable and calls itself if the comparison result is zero.
- `collect_pointer_lists`: traverses two different null-terminated pointer lists, copies them into a local array, and then processes each pointer.
- `initialize_resource_and_validate_strings`: Initializes language-specific string structures and coordinate tables before processing null-terminated pointer lists.
- `reorder_byte_blocks`: Iterates through a series of data blocks and reorders nine bytes within each block using a temporary buffer.
- `search_data_table`: iterates through a table of entries, comparing strings and returning a pointer to a specific memory location within the table based on a match.
- `process_conditional_data`: Iterates through a list of entries, performing conditional reads and byte-string comparisons to update a global value and manipulate data buffers.
- `search_string_table`: Iterates through a table of data to find a match for a byte string and stores associated values from the matched entry into a global table.
- `update_table_entry`: Checks if a value is within a specific range and, if so, writes a 32-bit value of all ones to a table entry.
- `adjust_offset_value`: Validates an index against a range, checks a status byte in a table, and then modifies a value in a table by adding an offset or adjusting it based on a comparison.
- `execute_unspecified_routine`: A routine consisting of 26 instructions that is called by one other routine and entered from outside the static flow.
- `process_string_and_data`: Parses a string to find a period, performs byte swaps on a value, and initializes a series of four longwords to all ones.
- `traverse_and_clear_list`: Recursively traverses a linked list by checking a value against a constant, then clears the pointer and sets a status field to a specific value.
- `process_segmented_data_blocks`: Recursively processes data segments by iterating through a table in fixed-size increments and calling various subroutines to manage linked lists, search string tables, and allocate decompression buffers.
- `update_reordered_data`: Calls another routine to retrieve a value and stores it in a global variable.
- `store_reordered_data`: Stores a value from another routine into a global variable and calculates a new word by combining bytes from the stored value and a subsequent memory location.
- `check_value_range`: Compares a provided value against a range defined by two global variables, provided a third global variable is zero.
- `process_segmented_data`: Performs conditional checks and, if successful, iterates through a data block in sixteen thousand three hundred eighty-four byte increments.
- `toggle_global_flag`: Saves the current value of a global word, sets that word to one, calls another routine, and then restores the original value.
- `insert_linked_list_node`: Stores a passed value into a memory location and recursively calls another routine if the value is not negative one.
- `call_subroutine_with_scaled_input`: Calculates a value based on a 32-bit input and passes it to another routine.
- `calculate_buffer_parameters`: performs arithmetic operations on input values to populate a structure with calculated offsets and sizes, then executes a routine using that structure.
- `recursively_compare_and_call`: Compares a parameter against a global value and, if they differ, recursively calls itself using a pointer found within the parameter.
- `clamp_value_minimum`: Decrements a value based on a second parameter and ensures the result is not less than zero.
- `recursively_partition_rectangles`: Recursively divides rectangular areas into smaller sub-rectangles based on nested data structures.
- `clamp_and_recursively_update_coordinates`: Compares two sets of coordinate pairs and swaps them to ensure a specific order, then recursively applies a clamping function against a reference boundary if certain conditions are met.
- `apply_bitwise_mask_to_buffer`: Performs bitwise operations on a buffer by iterating through a set of values.
- `apply_bitmask_to_buffer_3`: Checks if a set of values are within valid bounds and then iterates four times to modify a buffer by performing bitwise operations based on a bitmask and a calculated value.
- `apply_bitmask_to_buffer_2`: Iterates through a four-element buffer to modify each byte using a combination of a table lookup and bitwise operations.
- `apply_bitmask_to_buffer`: Iterates through a four-element buffer to modify each byte using a combination of a table lookup and bitwise operations.
- `modify_byte_stream`: Iterates through a buffer to perform bitwise operations on bytes based on values retrieved from a table and a bitmask.
- `apply_bitmask_to_data`: Performs bitwise operations on a byte stream using a mask and a counter to modify values in a buffer.
- `execute_handler_routine`: An eleven-instruction routine called by several routines including a recursive data processor.
- `process_recursive_data_variant`: Compares several signed short values in a structure and performs recursive calls based on the results.
- `process_recursive_data_2`: Checks multiple signed short values within a structure and copies a byte between locations before calling itself recursively.
- `process_recursive_data`: Checks two signed short values in a structure and then copies a byte from one location to another before calling itself recursively using values from the structure.
- `recursively_process_data`: Copies a byte from one location to another and then calls itself recursively using values from a structure.
- `initiate_recursive_process`: Calls another routine with a starting index and a calculated end index to perform a recursive operation.
- `apply_recursive_parameter_passing`: Passes its arguments to another routine in a different order and then returns.
- `recurse_with_parameters`: A recursive routine that passes a value to another routine and then calls itself twice with different combinations of arguments.
- `store_coordinate_data`: Calculates derived values from input parameters and stores several bytes into a global array before calling another routine.
- `iterate_coordinate_pairs`: Loops sixteen times to pass pairs of bytes extracted from a pointer to another routine.
- `forward_list_insertion`: The routine passes its argument directly to a linked list insertion helper and returns.
- `call_another_routine`: The routine calls another routine and then returns.
- `update_and_call_routines`: This routine saves a value from a memory location, updates it with a parameter, calls another routine three times with different arguments, and restores the original value.
- `set_global_value`: The routine stores the value two into a global variable.
- `update_global_parameters`: Clamps a value to a minimum of ten and then updates several global variables using results from other routines.
- `assign_global_values`: Updates several global variables using results from other routines and stores a value from a pointer into a global variable.
- `store_masked_pointer`: Obtains a pointer from another routine and stores it in a table, masking the lower bits of the value and copying a word from a secondary location.
- `decrement_and_recurse_pointer`: Decrements a value pointed to by a parameter by six and calls itself recursively until the value reaches zero.
- `wait_for_zero`: This routine enters a loop that continues as long as a specific word value at a memory location is not zero.
- `return_from_subroutine_4`: The routine performs a single return instruction.
- `reset_hardware_parameters`: The routine calls two other routines and then clears several memory locations and sets specific word values.
- `initialize_interrupt_handler`: populates a structure with a pointer to a global variable, a specific byte value, and a routine address, then calls a library function.
- `clear_memory_block`: This routine clears several consecutive memory locations and then fills a table with fifty zeros using a loop.
- `initialize_audio_dma`: Sets up audio and DMA parameters by writing specific values to a global structure and calling a configuration routine.
- `execute_library_sequence`: Calls a sequence of library functions using pointers retrieved from a global table.
- `initialize_parameter_structure`: Sets up a global structure by either performing a single call to another routine or, if parameters are provided, storing several values and performing multiple calls to update the structure.
- `flatten_linked_structure`: Traverses a nested structure to copy values from a source table into a destination table, incrementing a global counter.
- `flatten_tree_structure`: Traverses a linked data structure and copies specific values from its nodes into a destination table.
- `traverse_and_copy_tree_nodes`: This recursive routine traverses a linked data structure, copying values from nodes into a destination table.
- `initialize_audio_parameters`: Updates a global structure with calculated values and calls a routine to configure audio and DMA parameters.
- `invoke_audio_initialization`: Calls a routine to configure audio and DMA parameters.
- `update_linked_list_nodes_2`: Calculates a value by multiplying a short integer by five and stores it in a memory location.
- `update_linked_list_nodes`: Calculates a value by multiplying a short integer by five and stores it in a memory location.
- `reset_hardware_state`: Clears several memory locations and sets specific word values based on a provided parameter.
- `configure_audio_and_dma`: The routine stores three parameters into a global variable and then writes specific values to audio and DMA control registers.
- `process_event_data`: Checks a bitmask in a register to extract an index, then updates counters and copies data from a source table to a destination table based on specific conditions.
- `process_event_queue`: Processes a list of entries by checking bitmasks and incrementing counters, potentially copying data from a source table to a destination table.
- `write_word_to_memory`: The routine stores the value zero seven eight zero into a memory location at an offset from a register.
- `increment_counter_on_success`: Checks a flag at a memory offset and calls another routine recursively.
- `decrement_resource_counter`: Checks a global flag and then decrements a value stored in a global variable.
- `perform_conditional_read`: Checks a byte at a specific offset from a pointer and calls another routine to perform a read operation.
- `execute_conditional_operation`: Evaluates a byte at an offset from a pointer and calls another routine to perform file operations.
- `evaluate_conditional_logic`: Performs a series of conditional checks based on values in a global structure and executes various subroutines depending on the results.
- `execute_recursive_search`: Performs a recursive search or operation based on bitwise flags in a parameter and a global status.
- `update_conditional_state`: Checks a returned character value and calls another routine if the value is non-zero.
- `perform_conditional_read_2`: Calls another routine and may call it again if the returned value is negative.
- `execute_write_operation_4`: Calls another routine to perform a write operation and check the results.
- `retry_on_failure`: Calls another routine and checks its return value.
- `calculate_bitmask_status`: This routine performs bitwise operations on values from two memory locations to calculate and return a combined status byte.
- `check_bit_status`: Extracts a bit from a byte through a pointer and returns a constant value based on whether that bit is set.
- `return_zero`: This routine is empty and simply returns a value of zero to the caller.
- `update_elapsed_time_accumulator_2`: Calculates a time delta by subtracting a stored value from a current timer reading and updates a global accumulator if the stored value is zero.
- `wait_for_timer_threshold`: Compares a timer value against a stored value and waits until a specific elapsed time threshold is met.
- `adjust_value_by_four`: Compares two values and returns the first value plus or minus four depending on their relative magnitudes.
- `verify_coordinate_table_integrity`: Iterates through a table of sixteen entries to perform various coordinate transformations and comparisons based on an input mode.
- `iterate_coordinate_data`: Executes a loop sixteen times, calling another routine with derived values from a global array and input parameters.
- `update_coordinate_table`: Copies data from a pointer into a series of global buffers and performs iterative transformations based on a mode parameter.
- `process_bit_shifted_recursive_offsets`: Performs a recursive loop that modifies a value through bit shifts and arithmetic, calling another routine multiple times with varying positive and negative offsets derived from the current state.
- `iterate_bit_shifted_values`: Performs a loop that calculates new values using bit shifts and arithmetic, then calls another routine multiple times with modified offsets.
- `return_from_subroutine_5`: The routine is empty and performs no operations before returning.
- `return_constant_one`: The routine returns the constant value one.
- `process_input_sequence`: Executes another routine twenty times within a loop.
- `map_input_to_value`: Maps an input byte to a specific short value using a series of conditional checks, or performs logic based on several global bytes.
- `map_input_to_code`: Maps a pair of input bytes to a specific two-byte constant based on a series of conditional checks.
- `process_input_sequence_and_map_values`: Processes a sequence of input bytes from a table by incrementing an index and performing bitwise transformations.
- `wait_for_non_zero_result`: Repeatedly calls another routine until it returns a non-zero value.
- `update_clamped_coordinates`: Calculates the difference between a global value and stored coordinates, applies this difference to a target with clamping, and updates the stored coordinates.
- `calculate_status_and_coordinates`: Increments a counter based on bits in a global variable and copies two bytes from memory into two provided pointers.
- `store_packed_data`: Stores a 32-bit value into several memory locations as a series of bytes and two short words.
- `setup_driver_structures`: copies a block of data into a series of memory locations and initializes two separate structures with specific byte values and routine pointers.
- `call_library_functions`: Calls a library function multiple times using values retrieved from a table.
- `copy_memory_block_2`: Copies a block of memory from a source to a destination using either word-sized or byte-sized increments based on a count.
- `update_input_and_cursor`: Updates a timer or counter based on a direction flag and increments or decrements a value until a limit is reached.
- `update_bitmask_state`: Compares a byte from a global variable to a value in a table, then updates memory locations and bitmask bits based on specific character matches.
- `map_character_to_bitmask`: Translates a character in a register to a specific value and stores either a zero or a one at a memory location determined by that value.
- `clear_byte_at_offset`: Stores a zero byte at a location determined by adding an offset and a value to a pointer.
- `process_data_structures`: Iterates through a list of structures to apply bitmasks to memory and calls a library function.
- `execute_empty_routine_2`: This routine performs no operations and immediately returns to its caller.
- `iterate_data_structure_list`: Iterates through a linked list or array of structures, calling another routine for each element and updating a counter and a value within the structure.
- `apply_bitmask_loop`: Iterates through a data structure to apply bitwise operations to a memory range.
- `apply_bitmask_to_memory_2`: Performs bitwise OR or AND operations on longwords in a destination memory range using values extracted from a source data stream.
- `apply_bitmask_to_memory`: Performs bitwise operations to set or clear bits in a memory range using values from a data stream.
- `fill_memory_blocks`: Fills multiple memory blocks with either all zeros or all ones based on a count and a bitmask check.
- `fill_memory_with_constants`: Clears a block of memory with zeros and then fills subsequent blocks with all ones based on a count.
- `process_bitmask_layers`: Iteratively modifies memory values by performing bitwise operations controlled by a bitmask and a series of nested loops.
- `update_bitmask_layers`: Modifies specific bits within four different memory locations using a bitwise OR or AND operation based on a control value.
- `update_bitmask_flags`: Modifies specific bits within four different bytes based on a bitmask and a bit index.
- `process_nested_tables`: Recursively processes data by clearing nested tables and merging four word streams into a destination table.
- `process_nested_data_tables`: Recursively traverses and processes nested data structures by clearing tables and merging multiple word streams into a destination table.
- `clear_nested_tables`: Iterates through a nested structure to set multiple memory locations to zero.
- `merge_four_word_streams`: Performs a bitwise OR operation on four word streams and stores the result in a destination table.
- `allocate_decompression_buffer`: reads a four-byte integer from a pointer to determine a size, then calls routines to allocate and prepare a memory block of that size.
- `decompress_data_stream`: Expands a compressed data stream into a destination buffer using a dictionary-based decompression algorithm.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `parse_signed_integer_3`: Converts a numeric character string into a signed short integer.
- `parse_signed_integer_2`: Converts a numeric character string into a signed short integer.
- `parse_signed_integer`: Converts a character string into a signed integer by iterating through characters and performing arithmetic.
- `parse_numeric_string`: Converts a character string into a signed integer by iterating through the characters and performing arithmetic.
- `return_immediately`: The routine consists of four instructions that perform no observable work and return immediately.
- `parse_signed_long`: Converts a numeric character string into a signed long integer.
- `convert_string_to_long_3`: Converts a numeric character string into a signed long integer.
- `convert_string_to_long_2`: Iterates through a string to convert characters into a 32-bit numeric value.
- `convert_string_to_long`: Iterates through a string to convert characters into a numeric value, utilizing 32-bit multiplication and longword loading.
- `load_indirect_longword`: Returns a longword stored at a specific offset relative to a register.
- `calculate_modified_product`: Calls a 64-bit multiplication routine and returns a masked portion of the result stored in a global variable.
- `store_high_word`: Stores the upper sixteen bits of a provided value into a global variable.
- `compare_two_byte_strings`: The routine compares two byte strings byte by byte.
- `execute_file_operations`: Performs file operations including seeking and error handling based on the results of conditional routine calls.
- `call_routine_with_same_argument`: Passes its argument to another routine unchanged and returns.
- `update_file_entry_table`: Iterates through a table of entries to find an empty slot and attempts to populate it by performing a sequence of file operations including opening, seeking, and writing.
- `perform_read_operation`: Executes a file read operation and handles potential input/output errors through conditional calls.
- `clear_memory_and_call_library`: Zeros out a block of memory using a loop and then executes a function from a library base stored in a global variable.
- `return_immediately_2`: This routine performs no operations and returns to the caller immediately.
- `initialize_resource_structure`: Allocates memory, initializes a structure with specific flags and a magic number, and performs conditional logic based on the results of library calls and pointer checks.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d5.
- `parse_command_string`: Parses a string into individual tokens by handling whitespace and quoted substrings.
- `copy_string_with_limit_2`: copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one.
- `copy_string_with_limit_3`: Calculates the length of a source string and copies it to a destination buffer.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `examine_and_lock_file`: Performs a sequence of operations involving examining, locking, and unlocking a file, and retrieving data from a structure.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `open_and_recurse_window`: This routine recursively opens files or library resources, stores results in a global variable, and populates a structure with retrieved data.
- `save_and_recurse_parameter`: Stores a parameter into a global variable before calling itself recursively.
- `execute_format_string`: Stores a pointer in a global variable and calls another routine using a format string.
- `increment_and_store_byte`: increments a value at a global variable and stores a byte at the location pointed to by that value.
- `parse_format_and_call`: Parses a format string and subsequently calls another routine.
- `concatenate_and_return_result`: Calls another routine and returns a 64-bit value formed by concatenating the original contents of a register with the value left by another routine.
- `parse_format_string`: Parses a format string containing specifiers to extract and convert values from a data stream.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `perform_signed_division_2`: Performs a 32-bit division and adjusts the sign of the quotient based on the input sign.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `perform_32bit_division`: calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `call_unnamed_routine`: Calls another routine and then returns.
- `execute_sequential_subroutine_calls`: This routine executes a sequence of three subroutines.
- `return_immediately_4`: This routine performs no operations and returns to its caller immediately.
- `call_subroutines_sequentially`: A fifteen-instruction routine that calls two other routines in succession.
- `increment_and_store_byte_2`: The routine increments a pointer stored in a global variable and stores a byte from another location into the address pointed to by that global variable.
- `iterate_memory_range`: Iterates through a range of memory by calling another routine at increments of a fixed size.
- `clear_node_structure`: Recursively clears a structure and its associated linked nodes by zeroing out memory and checking bit flags.
- `clear_structure_data`: Checks various bit flags in a structure to perform conditional operations, including removing a linked list node and handling file errors, before zeroing out the structure's fields.
- `execute_write_operation_6`: Iterates through a memory range and performs conditional writes or updates a structure based on specific flag checks.
- `update_buffer_write_state`: Updates a data structure by conditionally writing a byte from a source to a destination and modifying status flags and pointers.
- `execute_conditional_write`: Performs conditional writes to a buffer or updates a pointer based on the contents of a structure and a status flag.
- `execute_write_operation_5`: Performs conditional write operations by either updating a global pointer or copying a byte from a structure to a memory location.
- `update_and_return_byte`: The routine updates a global pointer and a value within a structure, then returns a 32-bit value constructed from a byte and a modified short.
- `search_and_clear_entry`: Iterates through a list of structures to find an entry with a specific byte value and clears three consecutive longwords at that location.
- `search_and_clear_list`: The routine iterates through a list of structures, checking a byte at a specific offset, and clears three consecutive longwords in the first structure where that byte is zero.
- `clear_three_longwords`: The routine stores zero into three consecutive longword locations starting from a pointer.
- `update_parameter_status`: Updates a parameter structure by setting specific flags and values based on the return value of another routine.
- `set_control_flags`: Sets specific bits in a byte at a memory offset and updates a longword from a stack value.
- `traverse_and_call_list_2`: The routine iterates through a linked list and calls another routine for each element until a null pointer is reached.
- `traverse_and_call_list`: iterates through a linked list and calls another routine for each element until a null pointer is reached.
- `insert_into_linked_list`: Traverses a linked list and inserts a new element by updating pointers and copying data.
- `copy_pointer_data`: Copies two longwords from a memory location to a destination and returns a pointer to the data offset by two longwords.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `remove_linked_list_node`: Traverses a linked list to find a specific node and removes it by updating pointers.
- `traverse_linked_list_2`: Traverses a linked list of pointers and stores a longword from a pointer into a global variable or another location if a specific condition is met.
- `store_and_call_library_2`: Stores a longword from a pointer into a global variable or another location and then calls a library function.
- `store_and_call_library`: Stores a longword from a pointer into a global variable and then calls a library function.
- `validate_index_range`: Checks if a provided index is within a specific range and if a value at a calculated offset is non-zero, returning a boolean result.
- `check_interactive_status`: Calls a routine to check a value at a calculated offset and returns a boolean result.
- `return_zero_2`: This routine performs no operations and returns a value of zero to the caller.
- `handle_file_deletion_error`: Calls a routine to delete a file and, if the result is zero, calls another routine to retrieve an error and stores a word in a global variable.
- `return_zero_lea`: Returns zero after performing a memory address calculation.
- `execute_write_operation_3`: A routine with nineteen instructions that calls other routines to perform write operations and check results.
- `execute_write_operation_2`: Performs a write operation using parameters from a table and, if it fails, retrieves an error and stores a word in a global variable.
- `execute_write_operation`: Performs a write operation using parameters from a table and, if it fails, calls a routine to retrieve an error and stores a word in a global variable.
- `return_from_subroutine_6`: This routine performs a simple return from a subroutine.
- `check_result_and_call`: calls another routine and, if a specific bit is set in the result, conditionally calls a third routine.
- `chain_result_to_output`: Passes the result of a function call to a write wrapper followed by an output wrapper.
- `execute_callback_pointer`: Checks a global variable for a non-zero value and, if found, executes the function pointed to by that value.
- `execute_callback_chain`: Iterates through a loop to call a routine multiple times and executes several conditional function calls or pointer assignments based on values stored in a table.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_file_handle`: checks if an index is valid within a table and then calls a routine to close a file if the entry is not null.
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
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_indirect_function`: calls a function via a pointer retrieved from a library offset.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_function`: Calls a function from a library using a pointer retrieved from a global variable.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_function_2`: Executes a library function by accessing an offset from a base address stored in a global variable.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_function_3`: Performs an indirect call to a library routine via a pointer found in a global variable.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_function_4`: Uses a global variable to locate a library base and calls a specific function offset.
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
- `call_library_function_5`: Invokes a library function by dereferencing a pointer located at an offset from a global variable.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_library_function_6`: Calls a library routine through an indirect pointer stored within a global structure.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_function_7`: Executes a library function using an offset relative to a base address in a global variable.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_function_8`: Performs an indirect call to a library function based on a pointer in a global variable.
- `call_library_function_9`: Calls a library function by calculating an offset from a base address held in a global variable.
- `call_library_function_10`: Uses a global variable to find a library base and calls a function at a specific offset.
- `call_indirect_pointer`: Performs an indirect call to a function pointer retrieved from a table using an offset.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_indirect_pointer_2`: Retrieves a function pointer from a table via an offset and executes it.
- `call_indirect_pointer_3`: Executes a function by dereferencing a pointer located within a table structure.
- `call_library_vector_324`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.

### Bargon Attack (1992)(Coktel Vision)[cr FSN](Disk 1 of 4).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
