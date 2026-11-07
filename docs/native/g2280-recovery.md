# Game 2280 (star wars (1988)(system breaker)(pd)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02280). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 47 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 16 of 16 units identical (bytes and relocations), 14 raw-word lines |
| Functions named and described | pass, 621 of 621 named (0 author symbols), 621 of 621 described (66 by rule, 234 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 32 of 32 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 20356 of 20357 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 645262 |
| code | 67648 |
| data | 153741 |
| padding | 9893 |
| unused | 24576 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| run | 2 | 117 | 2 |
| type | 2 | 117 | 2 |
| ls | 1 | 440 | 20 |
| List | 2 | 117 | 2 |
| play3 | 3 | 1918 | 99 |
| WB2 | 3 | 22 | 1 |
| cs | 1 | 329 | 21 |
| Star Wars (1988)(System Breaker)(PD).adf:TSB | 1 | 137 | 13 |
| Port-Handler | 1 | 306 | 1 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
| Star Wars (1988)(System Breaker)(PD).adf:    | 1 | 641 | 35 |
| Star Wars (1988)(System Breaker)(PD).adf:starwars | 3 | 15403 | 411 |
| Star Wars (1988)(System Breaker)(PD).adf:cat | 10 | 238 | 3 |
| Star Wars (1988)(System Breaker)(PD).adf:bootblock | 1 | 176 | 9 |
| logged disk load from Star Wars (1988)(System Breaker)(PD).adf boot-block load | 1 | 0 | 0 |

## Cases handled

- star wars (1988)(system breaker)(pd)/Star Wars (1988)(System Breaker)(PD).adf: file s/startup-sequence (header 62 bytes, blocks hold 69): its blocks are judged as raw blocks outside the filesystem
- star wars (1988)(system breaker)(pd)/Star Wars (1988)(System Breaker)(PD).adf: the boot block reads 11264 bytes from disk offset 0 to an address (found statically); recovered as a unit
- star wars (1988)(system breaker)(pd)/Star Wars (1988)(System Breaker)(PD).adf: 1 logged loads cover 6656 bytes of the image
- star wars (1988)(system breaker)(pd)/Star Wars (1988)(System Breaker)(PD).adf: blocks 908-908 (bytes 464896-465408) outside the filesystem hold data, not code (opcode density 0.016, entropy 1.87)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `process_input_and_update_hardware` (   ): Reads the keyboard serial data register and modifies CIAA_PRA, CIAB_PRB, and a custom chip register. Kind: disk hardware/track check. Pass path: not determined.

## Functions


### run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_and_call`: Copies 16 longwords from a fixed offset in the data area into a buffer passed in, then calls a routine through an address register.

### type

- `find_max_and_build_table`: Program entry routine for the type command.
- `copy_memory_block`: Copies sixteen words from a source address to a destination address.

### ls

- `open_dos_library_at_entry`: Program entry for the ls command.
- `write_text_string_to_output`: Measures a text string and, when it is longer than one character, writes it out through the dos Write call.
- `lock_examine_and_read_path`: Locks a path with dos and examines it.
- `allocate_block_with_size_header`: Allocates a memory block through the exec AllocMem call and, when the allocation succeeds, stores the requested size plus four in its first longword.
- `free_block_if_allocated`: Releases a memory block through the exec FreeMem call when the block pointer is non-zero.
- `terminate_string_at_length`: When a field in the output record is set, stores the argument pointer and writes a zero byte at the end of a string of the given length, terminating it.
- `parse_next_command_argument`: Splits the next argument off a command line: it skips spaces, takes a quoted string or a word ending at a space or comma, writes a zero byte after it, and keeps the position for the next call.
- `set_task_signal_flags`: Decoded from a gap in the file, so it may be data.
- `count_string_length`: Counts the characters before the zero byte of its string argument and returns the count.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `load_iff_picture_file`: Loads an IFF picture.
- `parse_amiga_disk_asset`: Parses a structured data block, likely an Amiga disk resource or file format, by traversing a linked list of chunks.
- `parse_disk_directory`: Parses a directory structure from a disk image or file, searching for specific block markers.
- `parse_loadscreen_command`: Parses a command string to locate and extract data from a structured file format.
- `parse_command_argument_structure`: Parses a command string to locate and extract metadata from a structured data block.
- `parse_amiga_iff_resource`: Parses an IFF-formatted data structure by iterating through chunks to locate specific identifiers.
- `parse_amiga_iff_resource_2`: Parses an IFF-formatted data structure by iterating through chunks to locate specific tags.
- `parse_amiga_disk_asset_2`: Parses a structured data block, likely an Amiga disk asset or resource, by traversing a linked list of chunks.
- `parse_amiga_filesystem_structure`: Traverses a linked list of directory entries to locate specific file metadata.
- `run_loadscreen_command`: Command routine for LOADSCREEN. It prints usage text when no file is given and a memory message on failure.

### List

- `find_max_and_build_table_2`: Program entry routine for the List command.
- `copy_multiple_words`: Copies a fixed number of words from a source pointer to a destination pointer.

### play3

- `open_audio_device_at_entry`: Program entry for the play3 command.
- `open_audio_load_module`: Opens a message port and the audio device, reporting failures with the messages for an unopened file, the audio device, the port and memory.
- `play_sequence_until_finished`: Starts playback of a sequence.
- `send_buffer_to_audio_output`: Stores a buffer pointer in a global slot and calls a routine to write the buffer.
- `append_byte_to_audio_buffer`: Appends one byte to a 40-byte buffer.
- `flush_audio_buffer_contents`: Passes the buffered bytes on for output, resets the buffer pointer to its start, and returns a byte value from a field.
- `format_signed_number_text`: Converts a number to text by repeatedly dividing by the base and writing each digit from a digit table backward into a buffer.
- `format_stored_number_text`: Converts the number held in its argument block to text, writing digits backward from the end of a buffer and adding a minus sign when the value is negative.
- `format_number_digits_backward`: Writes the digits of a number backward from the end of a buffer and adds a minus sign when the value is negative.
- `format_text_with_percent_codes`: Formats text in the style of printf.
- `emit_formatted_number_field`: Writes a formatted number field by emitting a minus sign when needed and then the digits, using output library calls.
- `emit_fixed_point_digits`: Emits the integer digits of a fixed-point value through library calls, tracking a counter that places the decimal point.
- `emit_digits_with_state_flags`: Emits the digits of a number through library calls, with state flags controlling how many digits are written in each pass.
- `emit_fraction_with_leading_zeros`: When the value is below one, writes a leading zero and a decimal point, then pads the fraction with zeros as needed.
- `emit_decimal_number_text`: Writes the integer part of a value, using a zero and a decimal point when the value is below one, and then writes its digits through library calls.
- `emit_sixteen_digits`: Writes up to sixteen digit characters through library calls, one digit at a time, and writes zeros once the digit count reaches sixteen.
- `format_scientific_notation_string`: Converts a floating-point value into a string using scientific notation.
- `format_exponent_string`: Converts a value into a string representation using scientific notation with a decimal point and an exponent suffix.
- `emit_exponent_notation`: Writes a number in exponent form: a zero digit, a decimal point, the digits, then an 'e' followed by a plus or minus sign and two or three exponent digits.
- `emit_exponent_suffix`: Writes the exponent part of a number: an 'e', a sign, two or three exponent digits, and a terminating zero byte.
- `emit_positive_exponent_digits`: Writes a plus sign followed by two or three exponent digits and a terminating zero byte.
- `init_formatter_state_block`: Clears an 80-longword block in the formatter state, stores a stack pointer and a value in fixed slots, then calls a routine.
- `perform_signed_division`: Determines the sign of the result from its two inputs, calls an internal routine for unsigned division, and negates the quotient when the sign is negative.
- `perform_signed_remainder`: Determines the sign from its two inputs, calls an internal division routine, and applies the sign to the second result returned by that internal routine.
- `return_division_remainder`: Calls an internal division routine and returns its second result register unchanged.
- `perform_32_bit_division`: Calculates the quotient and remainder of a 32-bit unsigned integer division using a software algorithm, returning the results concatenated in a 64-bit value.
- `process_audio_data`: Iterates through a buffer and performs 32-bit fixed-point multiplication until a condition is met or a routine returns error.
- `insert_into_buffer`: Checks if a counter is less than a limit, then either stores a byte into a buffer and increments the counter or calls itself recursively.
- `traverse_linked_structure`: Recursively traverses a linked structure by checking bit flags in a status byte to trigger further calls.
- `clear_node_structure`: Recursively traverses a linked structure by checking bit flags in a status byte to trigger further calls.
- `recursively_clear_node_structure`: Recursively traverses a tree or linked structure by checking specific bit flags in a node's status byte to trigger further calls.
- `traverse_and_modify_node`: Recursively traverses a linked structure while checking bit flags in a status byte and performing conditional updates to node pointers and values.
- `search_and_clear_list_2`: Iterates through a list of structures and, upon finding an entry where a specific byte is zero, clears the first three words of that entry.
- `search_and_clear_list`: Iterates through a list of structures until it finds an entry where a specific byte is zero, then clears the first three words of that entry and returns its pointer.
- `clear_structure_words`: Clears the first three words of a structure pointed to by a register.
- `update_node_status`: Retrieves a value from a table using an index and performs a 32-bit fixed-point multiplication.
- `traverse_and_clear_list`: Iterates through a linked list of pointers, calling another routine for each element, and finally clears a global pointer.
- `traverse_linked_list`: Iterates through a linked list of pointers, calling another routine for each element until a null pointer is reached.
- `insert_into_list`: Calls another routine to obtain a pointer and then inserts a new node into a linked list by updating pointers.
- `forward_list_insertion`: The routine passes its argument directly to a linked list insertion helper and returns, acting as a thin wrapper for adding a node to the head of a list.
- `traverse_linked_list_2`: Iterates through a linked list to find a matching node.
- `traverse_and_store_pointer`: Traverses a linked list to find a specific node and then copies a value from that node to a global variable or another pointer before calling another routine.
- `store_and_call_routine_2`: Copies a value from one pointer to either a global variable or another pointer, then calls another routine.
- `store_and_call_routine`: Stores a value from a register into a global variable and then calls another routine.
- `lookup_and_multiply`: Retrieves a value from a table using an index and performs a 32-bit fixed-point multiplication.
- `conditionally_update_global_value`: This routine calls another routine and checks its return value.
- `return_zero_2`: An empty routine that returns zero.
- `validate_and_execute_callback`: Performs range checks on an input value and verifies a pointer from a table.
- `execute_callback_sequence_2`: Checks a global pointer to execute a callback and conditionally calls several library functions and another routine.
- `execute_callback`: Checks if a pointer in a global variable is non-zero and, if so, executes the routine at that address.
- `parse_audio_configuration_string`: Parses a string to extract configuration parameters, splitting the text into individual tokens and storing them in a newly allocated array.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `execute_conditional_callbacks_3`: Runs a loop ten times calling a routine, then executes several conditional function calls stored in global variables or tables.
- `append_string_data`: Appends characters from one string to the end of another until a null terminator is reached or a limit is hit.
- `strncat`: Appends characters from the second string to the end of the first, stopping at a zero terminator or after a given count, and terminates the result.
- `copy_string_to_buffer_2`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length_2`: Walks forward through a zero-terminated byte string to calculate the number of bytes preceding the terminator.
- `copy_string_with_padding`: Copies characters from a source string to a destination buffer up to a specified count or until a null terminator is found, then fills the remaining buffer space with null bytes.
- `validate_and_process_index`: Checks if an index is within a specific range and if the corresponding table entry is non-zero before performing a multiplication and clearing the entry.
- `multiply_32bit_fixed_point`: The routine takes two 32-bit fixed-point values from registers and returns a 32-bit fixed-point product.
- `call_library_function`: Performs an indirect call to a function located within a library retrieved from a global variable.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_function_2`: Executes a function at a specific offset within a library base found via a global variable.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_132_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_216_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_function_3`: Calls a routine by accessing a library base through a global pointer and applying an offset.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_function_4`: Invokes a library function using an offset from a library base stored in a global variable.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_108_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_function_5`: Calls a function using an offset from a library base provided through a parameter.
- `call_library_vector_450_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_414_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `allocate_and_initialize_structure`: Attempts to allocate a structure via a series of calls and, if successful, initializes its fields with provided parameters and a pointer.
- `reset_structure_fields`: Sets specific bytes and words within a structure to fixed values and calls another routine.
- `call_library_vector_354_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_function_6`: Performs an indirect call to a library function using a base address from a global variable.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_function_7`: Executes a library routine by calculating an offset from a base address stored in a global variable.
- `call_library_vector_294_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_function_8`: Calls a function at a specific offset from a library base retrieved from a global variable.
- `call_library_function_9`: Performs an indirect call to a function within a library base found via a global variable.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_library_vector_444_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_552_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_360_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_378_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_306_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `call_library_vector_384_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.

### WB2

- `initialize_system_libraries`: Opens required system libraries and performs initial memory management.

### cs

- `initialize_system_environment`: Sets up a series of global pointers and variables by calling multiple subroutines and retrieving values from a table.
- `write_null_terminated_text`: Measures a null-terminated text string and, when it is not empty, calls a system library routine to write it out.
- `process_filesystem_data`: Performs recursive operations involving file system metadata, disk resource parsing, and memory allocation.
- `execute_indirect_call_5`: Fetches a function pointer from a table and executes it, followed by a recursive call to itself.
- `allocate_tracked_memory_block`: Allocates a memory block through the system memory routine and, if the allocation succeeds, stores the requested size plus four in its first longword.
- `free_memory_block_if_set`: Frees a memory block through the system memory routine when the pointer passed in is not null.
- `terminate_text_buffer`: When a flag in a shared structure is set, records the buffer pointer and writes a zero byte at the end of the buffer, terminating the string.
- `read_next_text_token`: Advances through a shared text pointer, skipping spaces, and returns the next token.
- `call_set_signal_via_exec`: Forwards control to the exec SetSignal library function through the exec base pointer and returns the two result registers unchanged.
- `calculate_string_length`: Iterates through a string and returns the number of characters before the null terminator.
- `sanitize_and_parse_path`: Iterates backward through a string to find a directory or drive delimiter, converting uppercase characters to lowercase during the process.
- `sanitize_and_validate_path`: Iterates backward through a string to find a delimiter, converting uppercase characters to lowercase along the way.
- `sanitize_and_validate_path_2`: Iterates backward through a string to convert uppercase characters to lowercase and searches for path delimiters.
- `sanitize_and_validate_path_3`: Iterates backward through a string to find a directory or drive separator, converting uppercase characters to lowercase along the way.
- `sanitize_and_parse_path_2`: Iterates backward through a string to find a delimiter, converting uppercase characters to lowercase by clearing the fifth bit.
- `sanitize_and_parse_argument`: Scans a string backward to find a delimiter, such as a slash or colon, while converting uppercase letters to lowercase.
- `sanitize_and_validate_path_4`: Iterates backward through a string to find a directory or drive separator, converting uppercase characters to lowercase during the process.
- `process_string_path`: Checks if a string is null or a question mark, then iterates backward through a string to convert lowercase characters to uppercase until a delimiter is found.
- `parse_file_path`: Iterates backward through a string to find a directory separator or drive delimiter, converting uppercase characters to lowercase along the way.
- `normalize_and_validate_path`: Iterates backward through a string to convert uppercase characters to lowercase and searches for path delimiters.
- `process_command_argument`: Checks a string for specific characters, performs case conversion on a portion of the string, and manages several global variables based on the result of another routine.

### Star Wars (1988)(System Breaker)(PD).adf:TSB

- `dispatch_indirect_calls_2`: Calculates an offset from a global variable to execute a sequence of indirect function calls and conditional callbacks.
- `dispatch_indirect_calls`: Calculates an offset from a global variable to execute a sequence of indirect function calls and conditional callbacks.
- `execute_indirect_calls`: Calls a routine at an address calculated by adding a signed offset to a base address held in a register.
- `call_offset_function`: Calls a routine located at a fixed negative offset from a value held in a register.
- `execute_indirect_call_6`: Performs a call to a function located at an address calculated by adding a signed offset to a base address held in a register.
- `call_dynamic_offset`: Calls a routine at an address determined by adding a short integer to a value in a register.
- `execute_offset_calls_2`: Calculates an offset from a global variable, updates a pointer, and executes a sequence of indirect calls and conditional callbacks.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d1, then merges 3 into d6, then merges 0 into d0.
- `execute_conditional_callbacks`: Calls a routine, then executes up to two additional routines at specific offsets from a register if a condition is met.
- `execute_conditional_callbacks_2`: Calls a primary routine and then executes a sequence of function pointers stored at offsets from a base address.
- `call_offset_routines`: Executes a routine using an offset from a base address and conditionally calls two other routines using different offsets.
- `execute_offset_calls`: Calls a routine using an offset from a base address and, if a register is non-zero, calls two additional routines using different offsets.

### Port-Handler

- `parse_amigados_hunks`: Iterates through a data structure containing hunk type codes to identify and process executable or relocation segments.

### Disk-Validator

- `update_display_buffer`: Calculates character values and writes them into a memory buffer based on a numeric value.

### Ram-Handler

- `execute_indirect_jump`: Performs an indirect jump using a pointer retrieved from a global variable.

### Star Wars (1988)(System Breaker)(PD).adf:   

- `copy_code_block`: Calls another routine and, if the result is non-zero, copies a block of instructions from that routine to a location within a memory block.
- `execute_four_calls`: A 4-instruction routine that calls another routine four times in succession.
- `xor_table_with_mask`: Copies a block of data from a global variable to a destination table while xor-ing each word with a constant mask.
- `update_counter_and_hardware`: Performs bitwise operations on a hardware serial register and updates a global pointer or counter.
- `setup_callback_structure`: Initializes several memory locations with specific values and a routine pointer before calling a function via a pointer from a global variable.
- `call_indirect_function`: Stores a value into a global variable offset and then calls a function pointer retrieved from a global variable.
- `update_function_pointer`: Checks if a pointer from a global variable falls within a specific memory range and updates a global pointer if it does.
- `return_empty_subroutine`: The routine contains no executable instructions and performs no operations before returning.
- `calculate_checksum_and_clear_memory`: Calculates a checksum of a code block and, if the result does not match a specific value, enters an infinite loop while clearing a memory range.
- `access_hardware_registers`: Accesses the CIAA or POT hardware registers.
- `access_hardware_registers_2`: A 23-instruction routine that interacts with the CIAA serial data register and the COLOR00 register.
- `process_input_and_update_hardware`: Reads the keyboard serial data register and modifies CIAA_PRA, CIAB_PRB, and a custom chip register.
- `perform_complex_operations_2`: A single-instruction routine that is called by two other routines and leads to a large routine interacting with video hardware.
- `read_keyboard_register`: Reads a value from the keyboard serial data register.
- `dispatch_complex_operations`: Acts as a hub that redirects execution to several specialized subroutines for bitmask manipulation, string comparison, and absolute difference calculation.
- `dispatch_complex_subroutines_2`: Acts as a central hub that redirects execution to specialized subroutines for bitmask manipulation and string comparison.
- `dispatch_complex_subroutines`: This routine acts as a central dispatcher that calls several specialized subroutines including those for string comparison, bitmask manipulation, and video hardware interaction.
- `execute_jump_table_entry_2`: A 16-instruction routine that calls several subroutines including those for shifting, comparing strings, and performing complex operations.
- `execute_jump_table_entry`: A fifty-instruction routine that interacts with video hardware, performs bit shifting, and calculates sums before jumping to a fixed address.
- `perform_complex_operations`: A large routine that interacts with the video hardware register and calls several other routines including those for shifting values, accumulating sums, and jumping through a table.
- `shift_and_store_register`: Shifts the value of a register left by nine bits and stores the result into a memory location relative to a pointer.
- `store_accumulated_sum`: Calculates a sum of values from a list and stores the result into a memory location relative to a pointer.
- `accumulate_pointer_values`: Iterates through a list of values pointed to by a register, subtracting them from an accumulator and returning the final sum.
- `compare_string_with_case_insensitivity`: Iterates through a memory buffer, converting certain characters to lowercase and comparing them against a global variable until a mismatch or end-of-buffer occurs.
- `calculate_absolute_difference`: Calculates the absolute difference between a register value and a constant and returns the result.
- `clear_bits_based_on_string_nulls`: Iterates through a string, and if a null terminator is found, clears a corresponding bit in a global bitmask, repeating this for a range of memory.
- `find_string_length_and_set_pointer`: Calculates the length of a string located via a pointer and stores the resulting address into a memory location relative to another pointer.
- `execute_indirect_call_7`: Calls a routine via a pointer from a global variable and then performs an indirect call using a value returned by that routine.
- `convert_byte_to_ascii_digit`: Adds the ASCII offset for the character zero to a byte and stores the result at a memory location pointed to by a register.
- `convert_byte_to_ascii`: Adds a constant offset to a byte in a register and stores the result at a memory location pointed to by a register.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `clear_memory_block`: Sets a sequence of longwords to zero starting from a specific memory location.
- `nop_placeholder`: This routine is empty and contains no valid instructions.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 128 into d2, then merges 24 into d0.

### Star Wars (1988)(System Breaker)(PD).adf:starwars

- `initialize_system_environment_2`: Performs hardware initialization and memory setup by calling system routines, executing delay loops, and clearing segments of memory.
- `call_extract_masked_word`: This routine calls another routine to extract a masked word into a destination pointer based on specific byte conditions within a buffer.
- `extract_masked_word`: Checks two specific byte conditions within a buffer and, if met, stores a masked word into a destination pointer.
- `update_display_and_input`: Configures copper display registers including bitplane pointers, display enable, and vertical synchronization parameters.
- `calculate_entity_movement_and_timers`: Calculates velocity increments based on status bits or frame-to-frame differences and applies them to coordinate values with clamping.
- `update_player_velocity_and_timers`: Calculates velocity changes based on joystick input or previous frame differences and applies them to coordinate values with clamping.
- `update_entity_velocity_and_timers`: Calculates velocity offsets based on status bits or previous frame differences and applies them to coordinate values with clamping.
- `update_position_and_trigger_events`: Calculates movement offsets based on bitwise checks of a status word, applies them to coordinate values with clamping, and decrements timers to trigger specific event sequences when they reach zero.
- `update_coordinates_and_trigger_events`: Updates two coordinate values by adding calculated offsets and clamping them within specific minimum and maximum bounds.
- `update_and_trigger_events`: Updates two coordinate values by adding offsets and clamping them within defined minimum and maximum bounds.
- `clamp_and_dispatch_events`: Clamps two short integer values within a specific range and then checks several decrementing counters.
- `dispatch_event_data_structs`: Checks multiple decrementing counters and, upon reaching a threshold, copies specific data blocks into a destination structure.
- `process_event_counters`: Checks several decrementing counters in memory and, when a counter reaches one, copies a block of data from a source to a destination structure.
- `check_and_clear_short`: Checks if a value at a specific offset in a structure is non-zero and clears it if it is, returning the original value.
- `call_subroutines_with_stack_data`: Retrieves a value via a subroutine, populates a stack buffer with a constant, and then executes two subroutines using the retrieved value and the buffer.
- `calculate_and_call_subroutines`: Calculates a word-aligned value, stores it in a structure, and then executes two different subroutines using pointers from a global table.
- `execute_indirect_call_with_parameters`: Retrieves a value via a subroutine and, if successful, calls another subroutine using that value and two provided parameters.
- `execute_indirect_call`: Executes a subroutine through a pointer stored in a global table.
- `execute_indirect_call_alt_parameters`: Retrieves a value via a subroutine and, if successful, calls another subroutine using that value and two provided parameters in a different order.
- `execute_indirect_call_alt`: Performs an indirect call to a subroutine located via a pointer in a global table.
- `branch_on_retrieval_result`: Retrieves a value and, if it is zero, retrieves a second value to use as an argument for two subroutines; otherwise, it uses the first value for the subroutines.
- `process_parameter_and_call`: Calls a subroutine to retrieve a value, stores it, and then uses that value as an argument for two subsequent subroutine calls.
- `update_structure_elements`: Iterates through a structure using a nested loop to update two integer values at specific offsets.
- `update_counters_and_call_subroutines`: Iteratively increments a value in a structure within a loop, calls two subroutines with calculated offsets, and updates two other values in a structure.
- `update_structure_values`: Iteratively calls two subroutines using values from a structure and updates two integer values within that structure while decrementing a counter.
- `populate_structure_data_2`: Writes several values into a structure, including a conditional bitmask check, and increments a value by a constant while right-shifting a value.
- `populate_structure_data`: Writes several values into a structure, including a conditional bitmask check, and increments a value by a constant while right-shifting a counter.
- `populate_structure_in_loop`: Repeatedly writes various values and increments a counter into a structure while right-shifting a value until it reaches zero.
- `update_and_increment_counter`: Initializes a counter using a value from a table if it is zero, then increments it by calling a subroutine and returns a shifted version of the result.
- `call_dma_control_routine`: This routine calls another routine that interacts with the DMA control register.
- `manipulate_dma_control`: A 22-instruction routine that interacts with the DMACON register to enable DMA channels.
- `wait_for_zero_flag`: Enters a loop that continues to execute until a specific short value in a structure becomes zero.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `conditionally_call_and_reset`: Calls two different subroutines if their respective flags in a structure are non-zero, then sets a short value to all ones.
- `poll_and_reset_flags`: Waits for a specific value in a data structure to reach zero before conditionally executing two subroutines based on flag states and resetting a value to all ones.
- `configure_dma_blitter_transfer`: Calculates source and destination memory offsets based on input parameters and sets up DMA controller registers.
- `copy_data_and_increment_offsets`: Copies several values from one memory location to another and increments two integer values in a loop.
- `copy_structure_fields`: Copies multiple fields from one structure to another and increments two integer values within the source structure in a loop.
- `setup_blitter_transfer`: Configures a DMA blitter transfer using a source address derived from a table offset and a fixed width.
- `calculate_blitter_source`: Calculates a source address by adding a scaled sum of two short values to a base address and initiates a blitter transfer.
- `calculate_blitter_destination`: Sets the destination address of a blitter transfer based on a calculated offset from a base address and a scaled sum of two short values.
- `execute_conditional_subroutine`: Calls a subroutine with a parameter and, if the result is zero, calls another subroutine before returning.
- `return_dereferenced_value`: Returns a 32-bit value stored at a specific offset from a pointer.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `execute_indirect_call_2`: Calls a function pointer located at an offset from a base address using two passed parameters.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `configure_hardware_registers`: Writes values to a global variable and calls subroutines that interact with hardware registers to manage interrupts and DMA.
- `update_stored_value`: Calls a function with a constant argument and stores the returned value into a memory location at an offset from a base address.
- `initialize_system_resources`: Retrieves pointers from library functions, configures a structure with a callback and data, and modifies the INTENA register.
- `dispatch_callback_list`: Iterates through multiple arrays of function pointers and executes them with specific integer arguments.
- `execute_callback_list`: Executes multiple functions retrieved from a list of pointers using specific integer arguments.
- `copy_bytes_backward`: Copies a sequence of bytes from a source to a destination, handling overlapping memory by copying from the end if the destination is at a higher address.
- `copy_bytes`: Copies a sequence of bytes from a source address to a destination address using a counter.
- `copy_words_backward`: Copies a sequence of 16-bit words from a source to a destination, handling overlapping memory by copying from the end if the destination is at a higher address.
- `copy_words`: Copies a sequence of 16-bit words from a source address to a destination address using a counter.
- `return_absolute_value`: Returns the absolute value of a signed 16-bit argument by negating negative values and returning others unchanged.
- `absolute_value_long`: Returns the absolute value of a signed 32-bit integer.
- `mul_div`: Multiplies two 16-bit signed integers and divides the 32-bit product by a 16-bit signed divisor, returning the remainder and quotient.
- `calculate_squared_sum`: Calculates the sum of the squares of two 16-bit values.
- `multiply_two_shorts`: Multiplies two 16-bit values and returns the result.
- `update_table_values`: Fetches multiple 16-bit values from a table using wrapped indices and stores them into a series of global variables.
- `swap_table_entry`: Retrieves a value from a table and swaps it with a value stored in a global variable.
- `fetch_and_store_pair`: Fetches a value from a table and stores it along with an input parameter into a destination pointer.
- `return_empty_routine`: This routine is empty and performs no operations before returning.
- `process_wrapped_index`: Performs arithmetic on an input value to calculate a wrapped index and fetches a 16-bit value from a table based on that index.
- `fetch_table_value`: Calculates a wrapped index from an input value and uses it to retrieve a 16-bit value from a table.
- `calculate_scaled_coordinates`: Calculates and stores four scaled values into a table using division by a parameter incremented by a constant.
- `update_scaled_offsets`: Calculates and stores two scaled values into a table using a division by a parameter incremented by a constant.
- `increment_bcd_value`: Adds four separate byte values to a four-byte buffer and applies BCD adjustment to each byte.
- `update_bcd_and_max`: Adds four byte values to a buffer with BCD adjustment and updates a global value if it is less than another.
- `increment_bcd_counter`: Increments a single byte in a table using BCD adjustment and calls another routine.
- `scale_coordinate_list`: Iterates through a list of short values to calculate and store scaled versions in another list.
- `copy_table_entries`: Copies two consecutive word values from a table into a destination pointed to by a register.
- `apply_linear_transformations`: Performs three separate loops that apply different linear transformations to elements in a table.
- `process_multiple_tables`: Performs three separate loops that fetch data from different table offsets and write modified values into a destination buffer.
- `transform_table_coordinates`: Iterates through a table of values to perform coordinate transformations using linear scaling and division.
- `transform_coordinates`: Calls another routine to apply a linear transformation to a set of coordinates using fixed-point arithmetic.
- `transform_coordinate_table`: Applies a linear transformation to a set of coordinates using fixed-point multiplication and subtraction.
- `copy_and_modify_data`: Copies data from one memory location to another in a loop and performs several word-sized assignments to a destination buffer based on values from a source.
- `store_short_value`: Stores a two-byte value from a parameter into a global variable.
- `update_global_tables`: Checks specific bits in a parameter to set boolean flags and updates multiple tables with corresponding values.
- `calculate_interpolated_value`: Calculates a single short value through a series of nested multiplications and shifts and stores it in a global variable.
- `calculate_interpolated_value_2`: Performs a series of fixed-point multiplications and shifts to compute a value based on differences between several table entries and registers, then stores the result in a memory location.
- `interpolate_clamped_coordinate_2`: Performs linear interpolation between two boundary values based on a ratio and returns the resulting coordinate if it falls within a specific range.
- `interpolate_clamped_coordinate`: Performs linear interpolation between two boundary values based on a ratio and returns the resulting coordinate if it falls within a specified range.
- `calculate_coordinate_offset`: Performs signed arithmetic involving multiplication and division to calculate a new coordinate and checks if the result falls within a specific range.
- `return_from_subroutine`: The routine is empty and performs no operations before returning.
- `draw_bitmapped_line`: Calculates a line path between two points and performs bitwise OR operations to set pixels in a bitmapped buffer.
- `plot_line_to_bitplanes`: Calculates pixel positions and bit offsets to draw a line across four bitplanes.
- `draw_line_pixels_planes`: Plots a one-pixel line into the four-plane bitmap by setting the pixel bit in each plane for every step.
- `draw_pixel_column_planes`: Sets one pixel bit in each of the four bitplanes for a vertical run of pixels, moving down by the row stride once per pixel.
- `draw_pixel_column_at_pointer`: Sets one pixel bit in each of the four bitplanes at a pointer passed in a register, then steps the pointer by a row stride for the count given.
- `draw_sloped_line_planes`: Draws a sloped line in the four bitplanes by stepping along the major axis and advancing the minor axis on an error term started at half the major length.
- `draw_line_from_pointer_planes`: Draws a Bresenham-style line from a pointer into the four bitplanes, with the error term, bit index and row stride supplied in registers.
- `fill_span_planes_pattern`: Fills a horizontal span of pixels across the four bitplanes by setting single bits at the ragged ends and writing whole bytes of per-plane pattern values for full eight-pixel groups.
- `fill_span_from_pointer_planes`: Sets pixel bits across a horizontal span in the four bitplanes, starting from a pointer and bit index in registers.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `draw_wide_slope_line_planes`: Draws a line with a shallow slope in the four bitplanes using nested loops, where the inner loop sets pixels along a byte run and the outer loop steps rows.
- `draw_slope_line_pointer_planes`: Draws a line from a pointer into the four bitplanes using nested loops, setting the same pixel bit in each plane and advancing the error counter for each step.
- `store_point_then_call_draw`: Copies two 16-bit values from the argument into fixed slots of the global data block and then calls another routine.
- `store_point_call_and_return`: Copies the two 16-bit halves of its argument into fixed global slots, calls another routine, and returns the data registers as they stand after that call.
- `plot_pixel_in_clip_rect`: Plots one pixel in the four bitplanes only when its x and y coordinates lie inside a rectangle held as four global limits.
- `plot_pixel_at_pointer`: Sets one pixel bit in each of the four bitplanes at the address and bit index given in registers.
- `clear_bitmap_block_longwords`: Writes zero longwords backwards from a computed start address in an unrolled store loop.
- `draw_scaled_coordinate_sequence`: Divides table entries and a computed value to produce scaled coordinates, then passes them through a sequence of indirect calls.
- `project_and_plot_point_table`: Processes up to 24 groups of four 16-bit points, scaling each by a 512-based divisor with a fixed 600 offset and writing six derived values per group to a table.
- `draw_select_menu_labels`: Places a sequence of text labels at fixed screen positions, including the level select prompt, the easy, medium and hard options, the bonus choices, and the wave and point entries.
- `update_countdown_and_input_modes`: Updates a countdown digit from a frame counter divided by 50 and advances a second counter.
- `run_title_wait_loop`: Draws the prompt text 'or wait for info, pull trigger to start' and loops a three-state dispatcher until its state reaches 3.
- `wait_for_input_or_timeout`: Runs a per-frame loop for up to 500 frames until a state variable becomes nonzero, calling four routines each frame.
- `build_scaled_point_table`: Fills a 128-entry table with paired values taken from a source table, shifting one value by five and deriving the others through a conversion routine with offsets of 100 and 2000.
- `draw_credits_text_lines`: Draws the credit lines 'German voices digitized from', '1988', 'The System Breaker' and 'STAR WARS' at offsets from a base row.
- `draw_intro_text_block`: Draws eight lines of story text at increasing row offsets, including the Death Star and Force lines, then calls a block copy routine for the text area.
- `examine_undecompiled_routine`: The decompiler timed out, so no pseudo-code is available for this 47-instruction routine.
- `update_star_field_positions`: Advances 128 three-word star records by per-frame speed values derived from a lookup of the scaled input and adds offsets to their positions.
- `inspect_timed_out_routine`: The decompiler timed out, so no pseudo-code is available for this 30-instruction routine.
- `plot_indexed_coordinate_pairs`: Reads pairs of 16-bit coordinates from a table at fixed indices and passes each one, offset by a global value, to two indirect routines.
- `run_timed_sprite_sequence`: Runs three frame-synchronised passes that step a changing value through a drawing routine: one from 100 down to minus 200, one from minus 50 up to 200 with a scaled value, and one from 100 down to minus 370.
- `select_next_game_state`: Returns the next state code from a frame counter, a flag and an argument.
- `update_particle_records_random`: Advances 70 particle records by their velocities and respawns any that cross a boundary with random position and speed values.
- `initialize_particle_records`: Fills 70 three-word particle records with random starting positions taken from the high byte of two random values.
- `draw_instruction_line_by_index`: Selects one of about a dozen instruction text lines by a numeric index and draws it at a row offset from a base row.
- `draw_scoring_table_text`: Draws a scoring table with labels for Tie Fighters, Laser Towers, Trench Turrets, the Exhaust Port and the Darth Vader Ship, with point values from 1,000 to 50,000.
- `run_timed_scroll_sequence_a`: Runs a 768-frame loop that waits for each frame tick and draws a column sequence every step.
- `run_timed_scroll_sequence_b`: Runs the same 768-frame timed scroll loop as its neighbour, drawing a column sequence per step and reducing the scroll value by 256 after frame 720.
- `run_timed_descent_sequence`: Runs a frame-synchronised loop that counts down from 110 to minus 360 and draws a routine on each step while the scroll value is positive.
- `store_table_word_by_nibble`: Writes the second argument word into a 16-entry word table in the global block, indexed by the low four bits of the first argument.
- `wait_for_frame_count`: Clears a frame counter and spins until it reaches the given count, so the caller waits for that many frame ticks.
- `compute_integer_square_root`: Computes the integer square root of its argument with Newton iterations, starting from the argument divided by 200 plus two.
- `empty_routine`: This routine is empty and performs no operations.
- `set_default_bounds_fifteen`: Stores 15, minus 15 and 15 into three global 16-bit variables with no other effect.
- `set_window_limits_constants`: Stores six fixed 16-bit constants into global variables: 160, 115, minus 160, 159, minus 84 and 84.
- `scroll_depth_records_and_draw`: Moves 31 three-word records by a scroll value, wrapping each record by adding or subtracting 14500 when it passes a limit.
- `scroll_records_wrap_draw_loop`: Loops over a count of three-word records supplied in registers, moving each by a step and wrapping it by 14500 when it falls below minus 500.
- `scroll_depth_records_forward`: Moves 31 three-word records by a scroll value, wrapping each one by adding 14500 when it passes the upper limit.
- `scroll_depth_records_counted`: Moves a counted run of three-word records by a scroll value and wraps each one by 14500 at the upper limit.
- `update_starfield_with_motion`: Moves 31 starfield records in depth, negating their lateral speeds when they wrap.
- `drift_records_wrapped_twelve_bit`: Moves a counted run of records in depth and applies fixed lateral offsets, wrapping each lateral coordinate into a 12-bit range centred on zero.
- `scroll_depth_records_mirror_wrap`: Moves 31 three-word records by a scroll value and draws each one.
- `scroll_depth_records_counted_mirror`: Moves a counted run of three-word records by a scroll value and draws each one.
- `sync_wait_then_draw_pair`: Clears a frame counter, calls two routines with the same argument and then two more routines.
- `forward_single_routine_call`: Calls one routine with no arguments and returns.
- `pop_sorted_entries_above`: Walks backwards through a list of six-word entries while each entry's first word is at least the argument and the pointer stays inside the list.
- `advance_list_until_visible`: Steps through 12-byte entries while their first word exceeds the argument and a counter is in range.
- `find_next_entry_by_key`: Scans a six-word table for the first entry whose first word is greater than the argument, within a counted limit.
- `write_perspective_entries_table`: Walks a table of six-word entries and writes three words per entry to an output list: two global values and a scale equal to 25600 divided by the entry's first word plus 600.
- `update_entry_list_positions`: Moves each six-word entry by a horizontal step and its velocities, wrapping one coordinate within a range of plus or minus 2048.
- `plot_rotated_point_pairs`: Scales three matrix values by a per-entry factor and draws six points as offset pairs around a centre.
- `advance_level_parameters`: Derives four parameters from a counter capped at 99: a spacing value equal to the counter plus three capped at ten, a value of two or three, and a value of 15 minus twice the counter.
- `examine_undecompiled_routine_b`: No pseudo-code is available because the decompiler timed out, so this 10-instruction routine's behaviour cannot be stated.
- `plot_word_table_entries`: Calls an indirect routine with each word of a table in turn, advancing two bytes per word for one more entry than the count in the counter register.
- `plot_value_pair_via_jumps`: Passes a word and a long argument to two indirect routines in sequence, one per argument.
- `call_eleven_routines_in_sequence`: Calls one routine eleven times in a fixed sequence with no arguments and returns.
- `run_main_action_loop`: Clears four state variables and runs a loop until a state changes or 1500 frames pass.
- `run_action_frame_loop`: Runs the per-frame action loop until a state changes or 1500 frames pass.
- `wait_until_state_clears`: Clears a state variable, repeatedly calls a routine until it returns zero, then waits until the state stays zero.
- `draw_dashed_segment_pattern`: Draws a dashed pattern of point pairs whose length scales with the argument.
- `draw_shield_status_panel`: Draws the shield status display, choosing a bar setting from the shield value: 2 below three, 4 below five, and 1 otherwise.
- `adjust_shield_value`: When a state variable equals one, adds the argument to the shield value and redraws the status.
- `set_shield_from_parameter`: Copies the current shield value into a second state variable, sets a timer to ten, adds a word from the argument block to the shield value, and calls a routine to redraw it.
- `dispatch_pause_and_flag_commands`: Dispatches on a command code: code 64 pauses by waiting for input to clear, drawing the 'Game Paused' text, and resuming on the next input.
- `draw_blinking_status_markers`: Draws a set of marker shapes at positions taken from a stored x value, using one pattern for state one and an alternating pattern for state three that flips a toggle each call.
- `draw_alternating_marker_pattern`: Draws an alternating marker pattern followed by two blinking counters and a fixed marker shape.
- `draw_marker_shape_at_offset`: Draws a marker shape at an offset held in a register, then the two blinking counters and a fixed shape at the stored x value.
- `draw_marker_with_counter`: Draws one marker and one blinking counter, then a fixed shape at the stored x value.
- `draw_fixed_marker_pair`: Draws a fixed marker shape and then a second fixed shape at the stored x value.
- `update_ship_motion_and_flash`: Updates two motion values from a target difference, doubling them each frame and clamping them to plus or minus 270 and plus or minus 320.
- `update_motion_offsets_from_block`: Computes two offsets from a stored value and the difference between two others, then updates the motion values with the same clamping and smoothing as its neighbour.
- `smooth_motion_tail_update`: Clamps one motion value at 270, doubles and clamps a second at plus or minus 320, and smooths both offsets to three quarters of their previous values.
- `smooth_offsets_and_clear_flag`: Clamps one motion value at 320, smooths two offsets to three quarters of their previous values, and when a flag is set it clears it, toggles a bit and sets a counter to seven.
- `set_counter_seven_and_call`: Sets a counter to seven and calls another routine with the argument zero.
- `draw_digits_nibble_by_nibble`: Draws the digits of a number by extracting one four-bit group at a time, starting from the lowest, and calling a digit routine for each group.
- `tick_timer_and_draw_banner`: Decrements a timer and, when it reaches zero, swaps the two display buffer pointers and draws a fixed block at the same position in each buffer.
- `draw_timed_panel_frame`: Draws a panel of routines at fixed positions, using argument values read from global data.
- `draw_counter_indicator_block`: Draws a fixed-size block of 4 by 11 at a position taken from the screen base, drawing it once when a counter is below ten and twice otherwise.
- `draw_argument_indicator_block`: Draws a fixed-size block of 17 by 24 at the screen base position, drawing it once when the argument is below ten and twice otherwise.
- `draw_five_offset_markers`: Computes two scaled values from the differences between pairs of global positions, then calls a routine five times with offsets derived from them.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `fill_motion_lookup_tables`: Advances two accumulators and fills several global words with values from two lookup routines at offsets around a base.
- `draw_decreasing_spacing_row`: Calls a drawing routine at a fixed second coordinate of 250 for x values starting at 1300.
- `draw_glyph_shape_by_code`: Selects one of several small shape layouts by a numeric code and draws each part through a drawing routine.
- `draw_vector_text_string`: Draws a null-terminated string one character at a time.
- `read_text_entry_character`: Polls an input routine up to 30 times until it returns a value in the valid range, then stores the matched character code in a buffer.
- `append_character_to_entry_buffer`: Appends a character to a short text buffer of up to three characters.
- `display_countdown_value_via_routine`: Computes a display value from a stored counter, using 99 minus it or it minus 100 depending on a flag.
- `draw_high_score_table`: Draws a ten-row high score table with a rank, three-character initials and a score on each row, highlighting the row matching the argument.
- `insert_high_score_entry`: Sums values from a lookup routine into a score, then inserts it into a ten-entry sorted table, shifting lower entries down.
- `insert_score_with_shifted_total`: Accumulates lookup values into a score while shifting a second value right by four bits on each pass.
- `insert_entry_in_sorted_table`: Scans the ten-entry table for the first entry the score is at least as large as, moves the entries below it down one place, stores the score there and calls another routine.
- `open_timer_io_request`: Opens a library and a device, allocates a 56-byte request block, and sends it two commands numbered 9 and 10.
- `clear_global_call_constant`: Zeroes a global variable and calls another routine with a fixed 24-bit constant as its argument.
- `read_disk_track_sync`: Configures the disk controller to read a track, then waits for the read-complete flag with a bounded countdown.
- `reset_level_state_and_hints`: Resets the level state variables and timers, setting a counter to 750 and a threshold of 15 times the level index (capped at 90) plus 80.
- `rotate_and_scroll_star_records`: Moves 31 three-word records toward the viewer by 250 per frame, wrapping them at a lower limit by adding 14500 and negating their lateral speeds.
- `compute_record_lateral_positions`: Computes two lateral coordinates for a record by calling a lookup routine twice with a 360-unit angle offset.
- `spawn_random_record`: When the active count is below a limit and the speed value exceeds 100, spawns a new 80-byte record.
- `update_active_record_list`: Decrements five slot timers, then advances every active record by its velocities and runs its timed action.
- `spawn_enemy_record_from_pool`: Walks a pool of 80-byte records and, when a random roll and the active count allow, fills one with a random start position and a velocity scaled from its difference to two stored offsets.
- `send_control_word_sequence`: Issues a fixed sequence of thirteen calls alternating between two indirect routines, then returns the data registers unchanged.
- `draw_polygon_from_record`: Draws line segments between coordinates read from a record, using a move-then-draw pattern at each vertex.
- `draw_record_sprite_parts`: Draws the parts of a record's sprite according to its flags: up to two parts chosen by a mode flag, then four segments and five points at stored offsets when the last flag is set.
- `update_records_and_check_hits`: Walks the active record list, moving each record and checking whether it lies inside a horizontal and vertical band.
- `remove_one_random_record`: Scans the record list for the first entry at or above minus 401 with its flag set.
- `update_speed_and_score_state`: Recomputes two timing values from a frame difference and a level counter.
- `update_progress_indicator`: increments a value in a table and calls another routine with specific parameters if certain conditions are met.
- `draw_level_progress_indicator`: Draws a progress indicator from the level counter: below 111 it draws a value derived from the counter minus 60, and otherwise it scales two accumulators and draws the result.
- `draw_all_record_markers`: Calls a drawing routine once for each active record, passing that record's stored field and a fixed value of 300.
- `run_level_frame_budget`: Runs frames while a frame budget counter is non-negative and the state is clear, decrementing the budget each frame and waiting for the frame tick.
- `scale_lookup_to_small_range`: Calls a lookup routine with its argument, then a second lookup with the fixed value 10000, and returns the second result shifted right by 13 bits.
- `forward_call_no_arguments`: Calls another routine once with no arguments and returns, doing nothing else.
- `append_point_pair_to_buffer`: Writes two 32-bit values from its argument into the buffer at the current write pointer and advances the pointer by eight bytes.
- `build_point_table_from_lookups`: Fills a buffer with 32-bit entries: first pairs of lookup values stepping by 48 up to 1488, then further pairs and a series of entries from a second lookup plus 7500.
- `initialize_star_record_grid`: Initialises 31 three-word records with a first word starting at minus 259 and rising by 483 per record.
- `pause_after_ten_calls`: Calls one routine ten times with the same global address, then counts down two million iterations as a delay before returning.
- `load_shape_resources`: Zeroes the first word of six 22-byte shape entries, then loads six shape files by name: the four laser shapes, the nose shape and the star shape.
- `load_named_resource_into_slot`: Looks up a resource handle from its argument, stores the handle and a word in a table slot, and allocates a block of memory for the resource.
- `load_sound_and_shape_assets`: Loads the sound and shape assets by file name, including the death star, exhaust, hit, laser and missile sounds, several shape files, a pattern file and a music file.
- `allocate_game_buffers_and_tables`: Clears a set of global pointers and sizes, then allocates memory blocks of 32000 bytes (four of them), 4000 bytes, 2600 bytes, three of 1000 bytes, 800 bytes, 32 bytes and 10 bytes.
- `draw_return_key_prompt`: Clears the screen, copies an 80-byte block into a buffer, and draws the prompt text 'Return key.' at fixed positions before finishing with a further call.
- `initialize_game_state_variables`: Sets dozens of global game variables to fixed starting values, including 1024, 300, 360, 100, 160 and 17.
- `draw_score_and_wave_labels`: Resets a flag and two counters, then draws the 'score' and 'wave' labels at fixed positions and a fixed-size block, and finally calls another routine.
- `poll_menu_input_loop`: Polls a status routine in a loop, calling a dispatch routine while it returns nonzero and another routine while it returns zero.
- `load_shape_record_from_file`: Opens a resource by handle and identifier 1005, reads its ten-byte header, and computes the image size from the header fields.
- `load_shape_with_mask_planes`: Loads a shape as its neighbour does, then copies each row between end markers of minus one.
- `blit_clipped_sprite`: Draws a sprite with the Blitter: it clips the source rectangle against the playfield edges and aligns it to 16-pixel columns.
- `load_trumpet_shape_tables`: Stores twelve fixed 16-bit values and loads the 'shapes/trumpet' resource into newly allocated memory.
- `parse_chunked_resource_header`: Reads resource 650 into a buffer, then walks three chunks whose sizes are rounded up to even with an eight-byte header each.
- `play_music_stream_step`: Advances the first music stream on each tick by reading note bytes, choosing periods from a note table and loading the next delay from a lookup.
- `step_audio_channel_zero`: Advances the stream for audio channel zero: when its note timer expires it selects a period and length from tables, writes the AUD0LCH, AUD0LEN, AUD0PER and AUD0VOL registers, and enables the channel through DMACON. It then reads the next pair of stream bytes and sets the next delay.
- `step_second_music_stream`: Steps the second music stream: when its timer expires it reads two bytes and, for notes without the high bit set, enables a DMA channel.
- `start_audio_channel_one_note`: Starts a note on audio channel one by selecting a period from a table and writing the AUD1LCH, AUD1LEN, AUD1PER and AUD1VOL registers.
- `refresh_audio_channel_one_pointer`: When the second stream's delay is below three, writes the channel's address register from a stored base plus 120 and sets its length register to two.
- `initialize_audio_streams`: Resets the two music stream pointers and their timers, writes the audio volume registers, and sets the DMA control register to enable the channels.
- `set_state_three_clear_flag`: Sets a global value to 3 and clears a 16-bit flag.
- `set_flag_one`: Sets a 16-bit flag to 1 and returns.
- `return_flag_value`: Returns the current value of a 16-bit flag.
- `dispatch_code_to_routine`: Dispatches a numeric code: codes 0, 2, 4, 6, 8, 12 and 19 each call one routine and return its result.
- `draw_level_bonus_text`: Resets counters and state for a new run, sets a starting value of 512 and builds a 26-entry table of random positions.
- `update_and_compact_record_table`: Moves each six-word record by its velocities and applies a random swing to one lateral value.
- `advance_wave_and_spawn_objects`: Advances the wave counter and scrolls a 51-entry list of positions by a fixed step plus a lateral velocity.
- `build_wave_object_lists`: Fills the object list for five lanes: for each lane it places random positions across the screen, records an object type and an encoded value, and sets each object's weight.
- `draw_wave_object_by_type`: Draws a wave object according to its type code.
- `spawn_shots_from_wave_objects`: For each unflagged wave object of type 1 or 2, rolls a random number and sometimes spawns a shot record with a speed of 15000 or minus 3000.
- `check_player_object_collisions`: Checks each active object against the player's position for a hit.
- `remove_objects_entering_range`: Removes the first active object lying in a small horizontal band when a random test passes, and sets a 16-frame timer.
- `update_camera_offset_lookup`: Updates a stored offset and a step value from a 32-bit counter, using a lookup routine to decide when to reverse direction.
- `update_pitch_state_and_timer`: Recomputes two motion values from the frame difference and random limits derived from the wave counter, then updates a smoothed value.
- `draw_player_shot_indicator`: Draws the player's shot marker at one of three horizontal offsets chosen from flags and the shot counter, then advances the counter.
- `update_wave_frame_display`: Updates the wave display for one frame by passing a list pointer to two routines.
- `run_descent_scroll_until_position`: Returns zero on the first level.
- `project_point_by_depth`: Divides two 32-bit coordinates by a depth value offset by 470, and again by a second depth offset of 730, to produce two projected screen coordinates.
- `project_offset_pair_by_depth`: Projects two horizontal offsets of minus and plus 30720 by dividing each by a depth value offset by 470, then again by 730.
- `project_three_values_two_depths`: Projects three 32-bit values by dividing each by a depth offset of 450 and again by 750, storing six scaled results in globals.
- `project_pair_and_single_value`: Projects a pair of offsets of minus and plus 46080 divided by a depth offset of 750, and a single value divided by a depth offset of 450.
- `draw_level_hint_text`: Draws the in-level text, choosing a hint line by the level index from 'Points for using the FORCE', 'EXHAUST PORT AHEAD', 'EXHAUST PORT MISSED', 'avoid catwalks', 'shoot fireballs' and 'use the force'. It then fills position tables for the level.
- `initialize_wave_position_tables`: Clears timers and counters, sets the wave counter to 129 and the speed to 500.
- `update_motion_interpolation_buffers`: Updates and shifts several arrays of coordinate or motion data using a sliding window approach.
- `draw_table_rows_by_argument`: Draws a run of rows taken from a 16-point table.
- `run_bonus_tally_screen`: Runs the end-of-level bonus screen: it draws the star field and the shield number, then loops over frames while counting bonus points for remaining shields.
- `draw_masked_glyph_bits`: Uses the bits of its argument to choose drawing actions: for each of four nibbles it calls one routine when the low two bits are set and another when the next two are set, then returns the last result.
- `draw_shape_by_selector`: Draws one of three point sequences chosen by a selector value.
- `draw_bit_pattern_row`: Walks the four nibbles of a 16-bit pattern and draws a row of cells, advancing the offset by 26624 each step.
- `copy_view_values_and_call`: Calls a routine with a stored value, copies four view values into an eight-word table, then calls another routine with a value from the wave table.
- `process_fire_input_and_shots`: Checks the fire bit pattern from the input record and, when no cooldown is active and the nearest entry is clear, tries to fire up to five shots.
- `draw_wave_clear_sequence`: Draws the end-of-wave sequence when the current wave's value is positive: it draws a block of marker shapes at fixed positions, lowers a counter by 250, and draws the remaining scene elements through several drawing routines.
- `advance_entries_below_key`: Walks the entry list backwards while the current entry's first word is below the argument and the pointer stays inside the list.
- `spawn_objects_from_mask`: When the wave allows it, reads a bit mask from the current table entry and rolls a random value for each set bit.
- `draw_status_icons_by_wave`: Draws status icons according to the wave counter: before wave eight, or with the flag clear, it draws one icon at a base offset; otherwise it draws at a different offset.
- `draw_player_ship_and_shots`: Draws the player's ship as plotted points around its stored position, after first adjusting a stored value by minus 130.
- `draw_animated_point_frame`: Builds two sets of coordinate points offset by plus and minus 130 from a stored value, then plots the stored points of a figure as move-and-draw pairs.
- `run_angle_sweep_frames`: Runs a frame loop that sweeps an angle from minus 720 up to zero, advancing by 40 per frame and by 20 and then 12 once the sweep passes minus 360 and minus 120.
- `wait_frames_until_event`: Runs frames while neither of two flags is set, drawing each frame and waiting for the frame tick.
- `run_title_loop_until_flag`: Loops over frames while two flags stay clear, alternating which message is drawn according to the level index.
- `return_global_block_address`: Returns the address of a fixed offset within the global data block without reading or writing it.
- `execute_indirect_call_3`: Performs an indirect call to a function pointer retrieved from an offset of a register.
- `open_dos_library_and_start`: Clears a block of longwords of global storage and saves the task pointer.
- `stub_return_no_action`: Does nothing and returns immediately.
- `allocate_entry_list_files`: Allocates a six-byte-per-entry block for an entry list, fills in its fields and writes the four-letter marker 'MANX' into it.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d5.
- `parse_text_file_tokens`: Splits a text buffer into tokens: whitespace is skipped, double-quoted strings are copied with doubled quotes read as one, and other tokens end at a space, tab, newline or null.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a count or until a null terminator is found, then fills any remaining bytes of the count with zeros.
- `copy_string_pad_zeros_2`: Copies bytes from a source to a destination up to a count or until a null terminator is found, then fills any remaining bytes of the count with zeros.
- `append_string_bounded`: Appends a source string to the end of a destination string, copying up to about 32767 characters and writing a terminating zero when the source ends first.
- `append_string_with_limit`: Walks to the end of the destination string, then copies the source string into it until the source's terminating zero or until a count from the third argument runs out.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is copied, including the terminator.
- `compute_cross_product_halves`: Computes the cross terms of the 16-bit halves of two 32-bit register inputs and combines them into the upper longword of a 64-bit result.
- `open_icon_library_for_window`: Opens icon.library by name, finds a window-related structure through a library call, and loads resource 1005 into a field of the window record.
- `format_string_into_buffer`: Stores a destination pointer, passes the argument list on the stack to a formatting routine, and then clears the first byte of the destination buffer.
- `format_string_return_word`: Stores a destination pointer, passes the argument list on the stack to a formatting routine, clears the first byte of the destination, and returns the formatting result as a word.
- `store_byte_and_advance_pointer`: Writes the low byte of its argument to the global destination pointer, advances that pointer by one and returns the byte.
- `forward_format_call`: Passes its argument and the stack argument block to a formatting routine together with a callback, and returns nothing.
- `examine_undecompiled_routine_c`: The decompiler timed out, so no pseudo-code is available for this 48-instruction routine.
- `emit_formatted_string`: Walks a format string, passing literal bytes to a character-output callback and expanding each percent conversion.
- `perform_signed_division_2`: Records whether the two signed input registers have opposite signs, calls another routine to do the division, and negates that result when the signs differed.
- `negate_result_if_negative`: Calls another routine, then negates the value it returns when the first input was negative.
- `call_and_return_concatenated`: Calls another routine and returns a 64-bit value built from the original second input and the value the call leaves in its result register.
- `call_self_recursively`: Calls itself and then returns, with no other work, so it would recurse without end if reached.
- `perform_32bit_division`: Divides a 32-bit dividend by a 32-bit divisor.
- `compute_string_length`: Walks a zero-terminated byte string from the given pointer until it reaches the terminating zero, then returns the number of bytes before that terminator.
- `call_routine_and_return`: Calls one other routine and returns without changing any data itself.
- `execute_callback_sequence`: A 21-instruction routine that calls other routines to store a byte from a fixed offset and write a byte to a buffer.
- `stub_return_immediately`: The decompiled body contains only a return with no work, so the routine does nothing when called.
- `store_and_advance_byte`: Reads a byte from a fixed offset of a base register, writes it to a pointer, and advances that pointer.
- `store_byte_advance_pointer`: Reads one byte from a fixed offset of a base register, writes it to the location held in a pointer register, and advances that pointer by one.
- `call_routine_across_range`: Steps from a computed start address to a computed end address in fixed steps of 22 bytes, calling another routine with each position.
- `reset_buffer_descriptor_state`: Returns a failure value when given no descriptor.
- `reset_buffer_descriptor_inline`: Performs the same reset as its sibling routine but reads the descriptor directly from the address register instead of an argument.
- `write_byte_to_buffer`: Stores a handler routine in a global slot, then writes one byte into a buffer descriptor and returns it.
- `write_byte_to_buffer_inline`: Writes one byte from a frame offset into a buffer descriptor and returns it, using the same error-flag and storage-allocation checks as its sibling.
- `write_byte_or_clear_buffer`: Clears the buffer descriptor and returns zero when its size field is -1.
- `write_byte_through_buffer`: Stores one byte from a frame offset into a buffer descriptor, first requesting storage through a helper when none exists.
- `write_byte_advance_position`: Writes one byte from a frame offset into a buffer, advances the buffer position, and returns a value packing the end offset with the byte.
- `find_free_table_entry`: Scans a table of 22-byte records from a computed base and returns the first record whose flag byte is zero, after clearing its three leading words.
- `claim_free_entry_from_position`: Searches forward from the current record pointer through 22-byte records for one whose flag byte is zero, clears its three leading words and returns it.
- `clear_record_header_words`: Zeroes the first two longwords and the third longword of the record referenced by the address register.
- `initialize_buffer_record`: Initializes a buffer record passed by pointer.
- `setup_buffer_record_direct`: Sets a record's buffer size to 1024 and marks it as allocated, then stores a pointer read from a global slot into the record's buffer field.
- `release_all_list_nodes`: Walks a singly linked list whose head is held in a global slot, calling a helper with each node and its stored size plus eight.
- `release_all_list_nodes_inline`: Repeats the list-release loop starting from the node in the address register, calling a helper with each node and its stored size plus eight.
- `allocate_and_link_node`: Stores a handler routine in a global slot, then allocates a node of the requested size through a helper.
- `push_node_onto_list`: Pushes a node onto the front of a global list, storing the previous head as the node's link and the caller's pointer in its second field.
- `forward_low_word_argument`: Forwards the low 16 bits of its argument to another routine and then returns.
- `remove_node_from_list`: Searches a global linked list for the node whose data area matches the argument, unlinks it when found, and passes it to a helper with its stored size plus eight before returning zero.
- `remove_node_from_list_inline`: Searches the global list for the node matching the record in the address register, unlinks it, releases it through a helper using its size plus eight, and returns zero.
- `unlink_node_from_list`: Removes a node from a global linked list by pointing its predecessor, or the list head, at the node's successor.
- `pop_list_head_node`: Sets the head of a global list to the successor of the node in the address register, makes one helper call, and returns zero.
- `invoke_table_entry_handler`: Validates a 16-bit index against a stored table count and checks that the table entry at that index is set.
- `check_interactivity_via_table`: Retrieves a value from a table using an index and calls a routine to check if the system is interactive.
- `stub_return_zero`: Returns zero unconditionally and does no other work.
- `handle_io_result`: Calls another routine and, if the result is zero, calls a second routine to store its return value in a global variable before returning a specific value.
- `return_zero`: An empty routine that simply returns zero.
- `handle_write_error_2`: A 19-instruction routine that calls routines to check a signal mask bit and process write errors.
- `process_write_error`: Checks a bitmask in a word, and if certain bits are set, either stores a constant in a global variable or calls another routine to handle an error.
- `handle_write_error`: Calls a routine with three parameters and, if the result is negative one, calls another routine to store a value in a global variable before returning.
- `stub_return_immediately_2`: Contains only a return and does nothing when called.
- `check_signal_mask_bit`: Calls a helper with zero and a mask value, then tests the returned bits against that mask.
- `call_and_return_zero`: Calls one other routine, ignores its result and returns zero.
- `perform_io_operations`: Calls an output routine and a write routine before executing an indirect call through a pointer.
- `run_optional_handler_then_call`: Calls the handler stored in a global slot when that slot is set, then calls another routine and returns.
- `release_all_global_resources`: Works through the global resources in turn: it calls a helper for each entry of a counted table and then for the table itself, runs an optional handler, and calls a helper on each stored pointer that is set.
- `release_global_resources_with_supervisor`: Performs the same global sequence as its counterpart: a helper for each table entry and for the table itself, followed by the optional handler and helper calls on the stored pointers that are set.
- `release_paired_buffers`: When a stored primary pointer is unset, it calls a helper with a second stored pointer and a size scaled by four, provided that pointer is set.
- `call_helper_then_release_pointer`: Calls one helper, then passes a stored pointer to a second helper, and returns the value saved in the stack frame.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `release_table_entry_by_index`: Validates a 16-bit index against the stored table count and checks that the entry is in use.
- `release_and_clear_entry`: Checks a bit in a 16-bit value, calls a close routine if a condition is met, and clears a pointer in a table.
- `call_close_handler`: Executes an indirect call to a routine located at an offset within a table.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `execute_indirect_function`: Fetches a pointer from a nested memory structure and executes the code located at that address.
- `call_examine`: Wrapper that calls the dos library function Examine and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `execute_indirect_function_pointer`: Fetches a pointer from a nested structure within a global table and executes the function located at that address.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `execute_indirect_function_2`: Fetches a function pointer from a nested structure within a data table and executes it.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `execute_indirect_function_pointer_2`: Retrieves a function pointer from a nested table structure and executes it.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `execute_indirect_call_4`: Performs an indirect call to a function pointer retrieved through multiple levels of pointer indirection.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `execute_write_call`: Executes a function call via a pointer located at a specific offset within a data structure.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_108`: Calls a library entry at a fixed offset of minus 108 from a stored library base, passing the caller's registers.
- `call_library_vector_216`: Calls a library entry at a fixed offset of minus 216 from a stored library base, passing the caller's registers.
- `call_library_vector_450`: Calls a library entry at a fixed offset of minus 450 from a stored library base, passing the caller's registers.
- `call_library_vector_414`: Calls one helper in a library at a fixed offset of minus 414 from a stored library base, passing the caller's registers.
- `call_library_vector_414_2`: Calls a library entry at a fixed offset of minus 414 from a stored library base, passing the caller's registers.
- `allocate_cleared_record_block`: Returns zero if an earlier check fails.
- `allocate_cleared_frame_record`: Allocates a cleared public block of 34 bytes through a library call and returns zero if the allocation fails.
- `initialize_record_from_frame`: Fills an existing record's type, status and pointer fields from the caller's frame, then calls a helper.
- `forward_record_offset_pointer`: Passes the address twenty bytes past a record pointer to a helper and returns.
- `release_cleared_record_block`: Calls a helper with the record when its pointer field at offset ten is set, marks the record's status and secondary field as unused, passes the byte at offset fifteen to a helper, and frees the 34-byte block through a library call.
- `call_library_vector_354`: Calls a library entry at a fixed offset of minus 354 from a stored library base, passing the caller's registers.
- `call_library_vector_330`: Calls a library entry at a fixed offset of minus 330 from a stored library base, passing the caller's registers.
- `call_helper_with_value_48`: Calls one helper, passing its argument together with the constant 48, and returns nothing.
- `forward_argument_to_helper`: Passes its argument to one helper and returns nothing.
- `allocate_typed_record`: Returns zero when the first argument is null.
- `release_typed_record`: Returns zero when its argument is null.
- `call_library_vector_198`: Calls a library entry at a fixed offset of minus 198 from a stored library base, passing the caller's registers.
- `call_library_vector_198_2`: Calls a library entry at a fixed offset of minus 198 from a stored library base, passing the caller's registers.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_294`: Calls a library entry at a fixed offset of minus 294 from a stored library base, passing the caller's registers.
- `call_library_vector_294_2`: Calls a library entry at a fixed offset of minus 294 from a stored library base, passing the caller's registers.
- `call_library_vector_132`: Calls a library entry at a fixed offset of minus 132 from a stored library base, passing the caller's registers.
- `call_library_vector_132_2`: Calls a library entry at a fixed offset of minus 132 from a stored library base, passing the caller's registers.
- `call_library_vector_210`: Calls a library entry at a fixed offset of minus 210 from a stored library base, passing the caller's registers.
- `call_library_vector_210_2`: Calls a library entry at a fixed offset of minus 210 from a stored library base, passing the caller's registers.
- `call_library_vector_336`: Calls a library entry at a fixed offset of minus 336 from a stored library base, passing the caller's registers.
- `call_library_vector_372`: Calls a library entry at a fixed offset of minus 372 from a stored library base, passing the caller's registers.
- `setup_pointer_chain`: Stores the block's own address plus four in its first word, clears its second word, and stores the block's original address in its third word.
- `call_library_vector_444`: Calls a library entry at a fixed offset of minus 444 from a stored library base, passing the caller's registers.
- `call_library_vector_552`: Calls a library entry at a fixed offset of minus 552 from a stored library base, passing the caller's registers.
- `call_library_vector_552_2`: Calls a library entry at a fixed offset of minus 552 from a stored library base, passing the caller's registers.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `call_library_vector_360`: Calls a library entry at a fixed offset of minus 360 from a stored library base, passing the caller's registers.
- `call_library_vector_378`: Calls a library entry at a fixed offset of minus 378 from a stored library base, passing the caller's registers.
- `call_library_vector_306`: Calls a library entry at a fixed offset of minus 306 from a stored library base, passing the caller's registers.
- `call_library_vector_384`: Calls a library entry at a fixed offset of minus 384 from a stored library base, passing the caller's registers.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `initialize_system_memory_and_task`: Allocates a memory block, clears a segment of global storage, and saves the current task pointer.

### Star Wars (1988)(System Breaker)(PD).adf:cat

- `run_numbered_picture_loop`: Program entry point: copies a text argument into a global name buffer of up to 51 characters and appends the literal text 00.pic.
- `signal_task_if_flags_clear`: Signals a task through the library's Signal entry when two flag bits in a global word are both clear, then returns zero.
- `write_packed_iff_picture_body`: Converts a block of 32 16-bit words into bytes, run-length encodes the result with PackBits-style count bytes, and writes it to an open file as an IFF FORM with a BODY chunk.

### Star Wars (1988)(System Breaker)(PD).adf:bootblock

- `boot_allocate_and_launch_block`: Allocates memory through a library call and returns two if that fails.
- `open_dos_library_check`: Records two global base values, runs a helper and returns four if it fails, then calls another helper and makes a library call at minus 210 when a structure field is zero.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `start_copper_list_display`: Opens the graphics library through a library call and returns one on failure.
- `unpack_packbits_stream`: Decodes a PackBits-style run-length stream into a buffer until a remaining-length counter reaches zero.
- `unpack_packbits_stream_variant`: Decodes a PackBits-style stream from a source pointer into a destination pointer while counting down a total length.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d0, then returns.
- `rotate_eight_byte_entry_fields`: Writes the DMA control register value and decrements a frame counter.
