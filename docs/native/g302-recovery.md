# Game 302 (Zzzep): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00302). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (907776 top-level bytes in 24 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 32 raw-word lines |
| Functions named and described | pass, 273 of 273 named (0 author symbols), 273 of 273 described (55 by rule, 211 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 426 of 427 (99.77%) |
| Recovered without load address (counted in proof 1, flagged) | 32768 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5355 of 5355 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 556675 |
| code | 15954 |
| data | 78042 |
| padding | 4177 |
| unused | 252928 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| type | 2 | 117 | 2 |
| starsmittel | 2 | 177 | 7 |
| shades | 2 | 208 | 7 |
| add21K | 3 | 480 | 33 |
| border | 3 | 223 | 6 |
| Zzzep.adf:mad | 2 | 79 | 3 |
| Zzzep.adf:cls | 3 | 2549 | 141 |
| Zzzep.adf:1 | 1 | 155 | 3 |
| Zzzep.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 381 | 36 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 219 | 25 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 217 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| Zzzep.adf:1#unpacked | 1 | 533 | 1 |

## Cases handled

- runtime segment an address: 11 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk03): 29 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 159 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Zzzep/Zzzep.adf:c/shades: PowerPacker 2.3 (xfd) stream found by ancient scan (3576 -> 4844 bytes) but only 0 of 3 sampled windows are in reference RAM: not used
- Zzzep/Zzzep.adf:c/border: TNM Cruncher 1.1 (xfd) stream found by ancient scan (5576 -> 6752 bytes) but only 0 of 3 sampled windows are in reference RAM: not used
- Zzzep/Zzzep.adf:mad: ByteKiller 2.0 (xfd) stream found by ancient scan (1612 -> 2496 bytes) but only 1 of 1 sampled windows are in reference RAM: not used
- Zzzep/Zzzep.adf:1: crunched executable unpacked with ancient scan (DefJam Cruncher 3.2 (xfd), 11420 -> 32768 bytes), 10 of 10 windows confirmed in reference RAM, loaded at an address: recovered as u14
- self-modifying code at an address (6 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: runtime segment an address (play-rs-m1500-unpk03) +286, runtime segment an address (play-rs-m1500-unpk03) +298, runtime segment an address (play-rs-m1500-unpk03) +2AA, runtime segment an address (play-rs-m1500-unpk03) +2BC
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 3 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 jump target, 1 displacement
- Zzzep/Zzzep.adf:1: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 32768 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_memory_block`: Copies sixteen words from a source address to a destination address and then calls another routine.

### starsmittel

- `initialize_system_environment`: Initializes system libraries and allocates memory by checking for specific magic numbers and library availability.
- `call_two_functions`: Calls two different functions located at offsets from a provided pointer.
- `call_two_library_functions`: Calls two library functions using offsets from a provided pointer and returns a concatenated result.
- `call_two_library_functions_alt`: Calls two library functions using different offsets from a provided pointer and returns a concatenated result.
- `process_data_table`: Iterates through a table of data, performing calculations to determine a bit position and a memory address, then sets a bit at that address and stores the address in the table.
- `write_and_branch`: Writes a 16-bit word to an offset of a pointer in a2, increments the pointer by 4, and branches to another routine.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.

### shades

- `parse_amigados_hunk_table`: Parses a table of AmigaDOS hunk types to perform memory relocation and data copying.
- `stub_empty_return_routine`: This routine is empty and contains only a return instruction.
- `decrement_integer_loop`: Decrements a value in a register until it reaches zero.
- `decompress_back_reference_stream`: Decodes a packed byte stream by reading control values that either copy literal bytes or repeat earlier output with a length and offset from the stream.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_byte_cycle_counter`: Decrements a byte register once for each count unit, wrapping to a space character when it reaches zero, and returns.
- `invalidate_cache_lines`: The routine performs a cache invalidation operation using two parameters passed via registers.

### add21K

- `initialize_global_state`: Performs a series of checks and calls involving a global variable and a stack buffer, eventually setting a byte to one through a nested pointer structure.
- `configure_display_surface`: Looks up two handles through a lookup routine, releasing them via library stubs if a lookup fails, then sets a flag byte in an associated display structure and calls a routine with the sizes 640 and 256.
- `clear_block_and_store_pointers`: Zeroes forty-one 32-bit words starting at an offset from a base pointer taken from the stack, then stores the address of a small stack buffer and a global value into two fields of that same block.
- `call_optional_handler_and_recurse`: Checks if a global variable is non-zero to call a function pointer, then calls itself recursively.
- `parse_script_and_initialize_objects`: Splits a text buffer into a null-terminated array of word pointers using a character class table and replaces separators with null bytes.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `initialize_state_then_return`: Calls another routine ten times, then calls an optional handler from a global and conditionally calls routines with values taken from globals.
- `append_string_to_buffer`: Finds the end of a destination string and appends bytes from a source string until a null terminator is reached or a maximum length is hit.
- `append_string_with_count`: Walks to the end of the destination string and appends bytes from the source, stopping at the source terminator or when the count argument runs out, then null-terminates the destination and returns it.
- `copy_string_to_buffer`: The routine copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: The routine receives a pointer to a zero-terminated byte string, walks forward byte by byte until it finds the terminating zero byte, and returns the number of bytes from the start pointer up to but not including the terminator.
- `copy_bytes_zero_padded`: Copies up to a counted number of bytes from the source, stopping early after a zero byte, then fills the rest of the destination up to that count with zeros.
- `lookup_and_clear_table_entry`: Checks if an index is within a valid range and if a table entry is non-zero, then calls another routine with the entry value before clearing the entry in the table.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_function`: Calls a function from a library using an offset derived from a global base pointer.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `execute_indirect_function`: Retrieves a function pointer from a nested table structure using offsets from a base address and executes it.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_indirect_function`: Fetches a function pointer from a nested offset within a structure pointed to by a global variable and executes it.

### border

- `initialize_system_and_main_loop`: Allocates memory segments and initializes several global variables and counters.
- `reconstruct_amiga_hunk_segments`: Parses a data stream containing AmigaDOS hunk type codes to allocate memory and relocate segments.
- `nop_sequence_execution`: This routine executes a series of no-operation instructions and returns without modifying any registers or memory.
- `unpack_stream_with_pointer_shift`: Decompresses a packed stream like its neighbour, reading bits through a shift routine that takes a 32-bit value from the four bytes before the source pointer, shifts it right and sets its top bit.
- `shift_pointer_value`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `shift_bitmask_and_store_pointer`: Shifts a bit mask right one bit per iteration, reloading it from a memory location when it becomes zero, and stores the low word of a pointer into a global variable on each step.

### Zzzep.adf:mad

- `unpack_compressed_data`: Decompresses a bitstream by reading bits to determine control codes for either copying literal bytes or repeating previously unpacked data.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Zzzep.adf:cls

- `initialize_program_entry`: Clears a block of memory, initializes a global variable with a value from a specific memory location, and performs indirect library calls based on system status.
- `initialize_system_environment_2`: Processes command line arguments, initializes a global structure using library calls, and configures hardware registers and status bits.
- `traverse_linked_data_structures`: Recursively traverses a tree or linked structure by checking offsets within a data block and calling subsequent routines based on the values found at those locations.
- `process_command_line_arguments`: It traverses a linked structure to process various command line options and settings.
- `parse_console_input_string`: Reads a character from the console device and checks if it matches a carriage return.
- `check_and_reset_status`: Checks a byte at a specific offset within a structure and, if it matches a certain value, updates it and calls two other routines with specific parameters.
- `process_structure_data`: Checks a byte in a structure and, if it matches a specific value, performs a series of library calls and updates various fields within the structure.
- `call_library_functions`: Calls three different library functions using base addresses and offsets retrieved from a global variable.
- `update_structure_fields`: Allocates memory via library calls and, if successful, copies several word values from a structure into local variables before updating a global pointer and another structure field.
- `execute_library_calls`: Calls three different routines, including a delay function, using parameters derived from a global structure.
- `update_border_data`: Compares strings and, if a condition is met, updates several fields in a structure and modifies specific bits in a bitmask.
- `initialize_global_structure`: Allocates or updates a global structure by calling several routines to set pointers, clear values, and modify hardware register bits via a table.
- `update_hardware_register`: Stores values from a table into memory and sets a specific bit in a hardware register via a pointer.
- `set_status_bit`: Sets the third bit of a byte located at a specific offset from a pointer retrieved from a table.
- `clear_status_bit`: Clears the third bit of a byte located at a specific offset from a pointer retrieved from a table.
- `parse_font_string`: Parses a string to extract font names and sizes, handling errors if required information is missing and copying substrings into a local buffer.
- `update_font_library_data`: Updates font data structures by loading character sets and managing pointers within a font library.
- `manage_font_data`: Interacts with a font library to load character sets and manages data structures by executing indirect function calls and traversing linked structures.
- `manage_font_library_data`: It interacts with a font library to load character sets and manage data structures related to font files.
- `parse_signed_integer`: Converts a numeric string into a signed integer by skipping whitespace and handling optional sign characters.
- `parse_signed_integer_2`: Parses a signed integer from a string by skipping whitespace and handling optional sign characters.
- `store_parameter_and_call`: Stores a parameter into a stack-based location and calls another routine.
- `return_immediately`: This routine is empty and performs no operations before returning to the caller.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `process_window_data`: Tokenizes strings, interacts with windowing and DOS libraries to retrieve pointers, and updates various fields within a structure based on indirect function results.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d5.
- `tokenize_string_into_array`: Parses a string by skipping whitespace and extracting individual tokens, handling both quoted substrings and space-delimited words.
- `copy_string_pad_zeros`: The routine copies bytes from a source to a destination up to a count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `copy_string_and_pad_nulls`: Copies bytes from a source pointer to a destination pointer for a specified length, stopping early if a null terminator is encountered.
- `copy_string_with_limit_2`: The routine calculates the length of a source string and then copies bytes from a source pointer to a destination pointer until a null terminator is reached or a maximum count is exceeded.
- `copy_string_with_limit`: The routine copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter in the third argument reaches -1.
- `compare_two_byte_strings`: The routine compares two byte strings byte by byte.
- `copy_string_including_null`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `multiply_32_bit_values`: The routine takes two 32-bit values from registers and returns a 64-bit product formed from the high and low 16-bit halves of each operand.
- `access_window_data`: Interacts with icon.library and DOS to retrieve a pointer, then performs nested library calls to extract and store specific data into a provided structure.
- `store_pointer_and_call`: The routine stores a 32-bit value into a memory location pointed to by a global variable, calls another routine, and then clears that memory location.
- `clear_pointer_and_call`: Stores a parameter into a stack-based location, calls another routine, and then zeroes out the value at the pointer provided by the first parameter.
- `increment_and_store_byte`: The routine increments a value at a global variable location and stores a byte at the address previously held in that location.
- `parse_format_string_and_copy`: It parses a format string and copies a byte from a table into a destination pointed to by a global pointer.
- `execute_recursive_call`: A routine consisting of 48 instructions that performs a recursive call and returns a concatenated 64-bit value.
- `parse_printf_format_string`: Parses a format string to process conversion specifiers, width, precision, and length modifiers.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `prepare_signed_operands`: Clears a register, checks the sign of two different registers, negates them if they are negative, increments a counter if the first register was negative, and calls another routine before branching.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine.
- `perform_32bit_division`: The routine calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `compute_string_length_2`: The routine receives a pointer to a zero-terminated byte string, walks forward byte by byte until it finds the terminating zero byte, and returns the number of bytes from the start pointer up to but not including the terminator.
- `execute_placeholder_logic_2`: This routine performs a fixed sequence of twenty-one instructions.
- `execute_placeholder_logic`: A twenty-one instruction routine that is called by several routines, including one that performs no operations.
- `nop_placeholder`: This routine is empty and performs no operations.
- `increment_and_store`: A fifteen instruction routine called by a routine that increments a value through a pointer and stores a byte.
- `increment_and_store_byte_2`: The routine increments a value through a pointer and stores a byte from a source offset into the location pointed to by that value.
- `iterate_memory_range`: Iterates through a range of memory using a loop, calling another routine at each step based on a calculated offset.
- `clear_recursive_structure`: Recursively traverses a structure based on bit flags, calling other routines to handle nodes or files, and zeroes out the structure's fields.
- `clear_data_structure`: Checks flags within a structure to conditionally call other routines for node removal or file closing, then zeroes out the structure's fields.
- `handle_data_request`: Processes a request by checking flags and performing conditional logic to either return a value, clear a structure, or update a pointer with a byte.
- `evaluate_conditional_update`: Evaluates a condition involving a subtraction and a routine call to decide whether to clear a structure or update it with a byte.
- `process_status_flags`: Checks a short value to conditionally clear flags and zero a structure, or performs an update similar to a table entry routine.
- `update_table_entry_2`: Updates a structure by either performing a conditional call or updating a pointer and status flags with a byte from a source.
- `update_table_entry`: The routine updates a table entry by adding a short value to an existing value, sets a bit in a status byte, and stores a byte from a source offset into a pointer found within the table.
- `search_table_entries`: Iterates through a table of 22-byte structures to find an entry where a specific byte is zero, then clears three longwords at that location.
- `search_and_clear_table`: The routine iterates through a table of 22-byte structures, checking a byte at a specific offset, and clears three consecutive longwords in the entry if a null byte is found.
- `clear_table_entry`: The routine clears three consecutive longwords at the location pointed to by a pointer.
- `update_status_state`: Updates a structure's status flags and pointer fields based on the return value of another routine.
- `update_status_flags`: Sets specific bits in a byte at an offset from a pointer and updates a long value from a stack location.
- `traverse_linked_list_2`: Traverses a linked list of pointers, calling another routine for each element, and finally clears a pointer in a structure.
- `traverse_linked_list`: The routine traverses a linked list of pointers, calling another routine for each element, and finally clears a value at a specific offset from a global pointer.
- `insert_into_linked_list`: Inserts a new node into a linked list by updating pointers and copying data from a source location.
- `copy_pointer_data`: Copies two longwords from a memory location to another and returns a pointer incremented by eight bytes.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `remove_linked_node`: Traverses a linked list to find a specific node and removes it by updating the preceding node's pointer.
- `search_linked_list`: Traverses a linked list to find a specific node and stores a value into a global variable or a provided location.
- `store_pointer_and_call_2`: Stores a longword from a pointer into either a provided memory location or a global variable before calling another routine.
- `store_and_call`: Stores a longword from a pointer into a memory location and then calls another routine.
- `validate_table_index`: Checks if an index is within the bounds of a table and verifies that the entry at that index is non-zero before calling a function.
- `check_interactive_status`: Retrieves a function pointer from a table using an offset and calls it to return a boolean result.
- `return_zero`: This routine is empty and simply returns a value of zero to the caller.
- `handle_delete_result`: Calls a routine and, if the result is zero, calls a DOS function to retrieve an error and stores it in a global variable.
- `return_zero_lea`: Returns zero using a relative address calculation.
- `execute_conditional_logic`: A routine consisting of nineteen instructions that is called by several other routines, including those performing conditional writes.
- `execute_conditional_write`: Performs a write operation if a specific bitmask check passes, otherwise stores an error code in a global variable.
- `perform_write_operation`: Calls a write function and, if it returns a failure code, calls a DOS function to retrieve an error and stores it in a global variable.
- `return_from_subroutine_2`: The routine is empty and performs no operations before returning.
- `execute_conditional_callback`: Calls a routine with specific parameters and executes a callback if a bit is set in the return value, provided a global short value is non-zero.
- `call_callback_routine`: Executes another routine before returning zero.
- `write_fixed_text_then_callback`: Obtains an output handle, writes a four-byte text fragment from a fixed address via DOS calls, and then executes a conditional function.
- `execute_indirect_function_4`: Checks if a value at a specific offset from a pointer is non-zero and, if so, calls the function pointed to by that value.
- `process_complex_structure`: Iterates through a loop based on a table value, performs multiple indirect function calls using various offsets, and checks a bit in a system register before executing a final indirect call.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_file_handle`: The routine validates an index against a table, checks if the associated file pointer is non-zero, and if valid, calls a routine to close the file before zeroing the entry.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_function_2`: The routine calls a function from a library using an offset retrieved from a global variable.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `execute_indirect_offset`: Executes a function pointer retrieved from a nested offset within a structure.
- `execute_indirect_function_2`: Retrieves a function pointer from a nested table structure and executes it.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `execute_indirect_offset_long`: Executes a function pointer retrieved from a large negative offset within a structure.
- `execute_indirect_function_3`: Fetches a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `execute_indirect_table_entry`: Executes a function pointer retrieved from a specific offset within a structure.
- `call_indirect_function_via_table`: Accesses a table through an offset from a base address and executes the function pointer found at a secondary offset within that table.
- `call_library_function_3`: Performs an indirect call to a function located within a library via an offset in a table.
- `execute_indirect_library_call`: Executes a function pointer retrieved from a specific offset within a structure via an indirect call.
- `call_library_function_4`: Executes a function call by dereferencing a pointer found at a specific offset in a table.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_function_5`: Invokes a routine through an indirect pointer retrieved from a table.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_indirect_table_entry`: Performs an indirect call to a routine using an address retrieved from a table via a pointer and a constant offset.
- `call_library_function_6`: Calls a function using an address calculated from a table entry and a constant offset.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_function_7`: Triggers a function call by accessing a pointer through a table and an offset.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_indirect_table_offset`: Executes an indirect jump to a routine located at a specific offset within a table.
- `call_library_function_8`: Performs an indirect jump to a routine stored in a table at a specific offset.
- `call_library_vector_492`: Library stub: loads a library base from a global variable and calls the library function at offset minus 492 of that base, returning its result to the caller.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_576`: Library stub: loads a library base from a global variable and calls the library function at offset minus 576 of that base, returning its result to the caller.
- `call_library_function_9`: Calls a function by dereferencing a pointer from a table using a large negative offset.
- `call_library_vector_570`: Library stub: loads a library base from a global variable and calls the library function at offset minus 570 of that base, returning its result to the caller.
- `call_library_vector_582`: Library stub: loads a library base from a global variable and calls the library function at offset minus 582 of that base, returning its result to the caller.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_function_10`: Executes a library routine via an indirect pointer located in a table.
- `call_library_function_11`: Performs a function call using a pointer retrieved from a table with a specific offset.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_function_12`: Invokes a routine by dereferencing a pointer from a table at a given offset.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `execute_indirect_call`: Performs an indirect call through a pointer located at an offset from a value stored in a register.

### Zzzep.adf:1

- `copy_and_process_data_stream`: Copies a block of data from a global source to a destination and then iterates through the copied data, performing bitwise operations and conditional lookups to modify values.
- `combine_and_shift_data`: Combines two bytes into a word, stores the result at a pointer, and returns a value shifted right by one with a specific bit set.
- `loop_bit_shift`: Repeatedly shifts a value right until it reaches zero, then calls another routine a specified number of times.

### Zzzep.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_dma_and_audio_loop`: Enables DMA channels via the DMACON register and initializes audio hardware before entering a loop that manages timing counters and conditional execution.
- `call_routine_thrice`: Calls another routine three consecutive times.
- `clear_memory_blocks_2`: Depending on a global flag, either clears a series of memory blocks at fixed intervals or sets several global variables to zero.
- `fill_memory_table`: Fills a memory area with a repeating pattern of values derived from a global table and specific constants.
- `iterate_and_call`: Iterates through a list of values pointed to by a register and calls another routine for each non-negative value encountered.
- `copy_data_block`: Performs a series of indirect calls and then copies a sequence of data blocks from one memory location to another using a loop.
- `dispatch_via_table`: Checks a longword in memory, and if it is not negative, uses a value from a global variable to index into a table and jumps to the address found there.
- `execute_recursive_calls`: This routine performs two consecutive calls to itself before returning.
- `update_dual_counters`: Decrements and wraps one global variable before adjusting the value of another global variable based on its current sign.
- `call_two_subroutines_2`: Calls two subroutines in sequence and then returns.
- `call_two_subroutines_3`: Calls two subroutines in sequence and then returns.
- `increment_two_clamped_globals`: Calls increment_clamped_global and increment_clamped_global_alt to increment two separate 16-bit global variables, each capped at a value of 16, then returns.
- `update_clamped_counters`: Calls a routine to adjust a global counter and then calls another routine to increment a different 16-bit global variable while capping its value at 16.
- `decrement_and_increment_clamped_values`: Decrements a clamped value and then increments a different 16-bit global variable capped at 16, before returning.
- `call_two_subroutines`: Calls two different subroutines in sequence and then returns.
- `call_two_subroutines_alternate`: Calls two different subroutines in sequence and then returns.
- `increment_clamped_global`: Increments a 16-bit global variable and caps its value at 16.
- `decrement_clamped_value`: Decrements a global variable and ensures it does not fall below a specific minimum value.
- `increment_clamped_global_alt`: Increments a different 16-bit global variable and caps its value at 16.
- `decrement_and_wrap_variable`: Decrements a global variable and resets it to a specific negative value if it falls below zero.
- `adjust_global_counter`: Decrements a global variable if it is positive, or increments it if it is negative.
- `increment_global_variable`: Increments a global variable by one.
- `modify_global_value`: Changes a global variable by decrementing it if positive or incrementing it if negative.
- `increment_global_variable_2`: Increments a specific global variable by one.
- `update_global_from_table`: Checks if a global variable is zero, and if so, adds a value from a table-based pointer to another global variable; otherwise, it adds a value from a table-based pointer to a different global variable.
- `add_to_global_variables`: Performs bitwise OR operations on a register, adds a register to a global variable, and then adds the value of another global variable to a third global variable.
- `modify_ciaa_registers`: Clears a 32-bit value at a global address, reads a byte from a CIAA register, sets bit 6 of that register, performs a bitwise NOT and a rotate right on the byte, stores the result in the global address, and clears bit 5 of the same CIAA register.
- `decrement_global_word`: Checks if a 16-bit word at a global address is zero; if not, it decrements the word by one and branches to another routine.
- `update_conditional_globals`: Sets a global variable to a constant value and clamps another global variable to zero or negative based on a condition.
- `clear_global_variable`: Sets a global variable to zero.
- `configure_audio_hardware`: Writes several constant values to audio hardware registers and enables DMA channels.
- `configure_audio_channels`: Sets the DMA controller to enable audio channels, performs two delay loops, configures both audio channels with specific starting addresses, lengths, periods, and volume levels, enables DMA, performs another delay loop, and finally updates the audio channel addresses and lengths.
- `invert_global_word`: Performs a bitwise NOT on a 16-bit word at a global address and returns.
- `store_constant_and_call`: Stores a pointer to a global variable and a constant value into a global location, then calls another routine.
- `perform_multiple_stores_and_calls`: Stores a pointer and constant values into a global location multiple times, calling various routines between operations.
- `execute_loop_with_bitwise_ops`: Performs bitwise OR operations on a value and enters a loop that continues until a specific memory location contains a specific value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_functions_2`: Calls a library function using a pointer from a global variable, stores the result in a global variable, then calls another library function using a pointer from a global variable and stores that result in a different global variable before returning.
- `close_library_handles`: Loads the library base into a6 and calls a routine at offset -414, then repeats the process with a second library base and routine before returning.
- `execute_library_calls_conditionally`: Loads a routine pointer from a table, calls a library routine, and if the return value in d0 is not zero, calls two more library routines using pointers from a table, otherwise it clears a global variable and returns.
- `calculate_scaled_value`: Multiplies the lower 16 bits of a register by 2560, subtracts 1 from the result, and enters a loop that decrements the result until it is no longer negative.
- `decompress_bitstream_data`: Reads a bitstream from a source pointer, using a bit reader to extract variable-length bit sequences that determine how many bytes to copy from a lookup table into a destination buffer.
- `jump_with_registers`: Loads the values 8 into registers d1 and d4 and jumps to another routine.
- `branch_on_register_value`: Calls a subroutine, compares the value in d2 to 2 or 3, and either branches to another routine or calls a subroutine again before jumping to a third routine.
- `copy_bytes_from_table`: Sets d1 to 9 plus the value in d2, increments d2 by 2, moves the new d2 into d3, calls another routine, decrements a2, copies a byte from a table at a2 plus d2 into a2, and loops using d3 until the loop finishes; it then compares a1 and a2 and either branches or restores registers and returns.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `decompress_data_block`: Searches for a specific four-byte marker in memory, then enters a loop that either copies a sequence of bytes or repeats a single byte a specified number of times based on the value of a byte in a buffer.
- `loop_negate_and_store`: Reads a byte from a0 into d0, negates it, stores the byte from the next address in a0 into the address in a1 and increments a1, and loops using d0 as a counter; it then increments a0 by 2 and branches to another routine.
- `increment_and_branch`: Increments the pointer in a0 by 4 and branches to another routine.
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `unpack_and_store_data`: Searches for a specific four-byte pattern in memory, then iterates a number of times defined by a register to unpack three bytes into a single word and store it along with a counter into a destination table.
- `reset_hardware_and_globals`: Clears two global variables, writes 1 to a hardware register, checks a bit in a hardware register, and branches based on the status of a global variable and a bit test.
- `initialize_global_values`: Writes 1 to a global variable, writes 65535 to another global variable, and branches to another routine.
- `clear_and_set_threshold_three`: Clears two 16-bit global variables, compares d0 to 3, and if equal, writes the value 3 to a global variable before branching to another routine; otherwise it branches to another routine.
- `update_global_threshold_768`: Compares d0 to 768, and if equal, writes the value 7 to a global variable before branching to another routine; otherwise it branches to another routine.
- `update_global_threshold_256`: Compares d0 to 256, and if equal, writes the value 1 to a global variable before branching to another routine; otherwise it branches to another routine.
- `update_global_threshold_one`: Compares d0 to 1, and if equal, writes the value 5 to a global variable before branching to another routine; otherwise it branches to another routine.
- `update_global_threshold_two`: Compares d0 to 2, and if equal, writes the value 4 to a global variable before branching to another routine; otherwise it branches to another routine.
- `compare_and_write_value`: Compares d0 to 259; if they are not equal, it branches to another routine, otherwise it writes the value 2 to a global variable and branches to another routine.
- `check_threshold_and_write`: Compares a 16-bit register to 481; if they are not equal, it branches to another routine, otherwise it writes the value 6 to a global address and branches to another routine.
- `update_global_on_threshold`: Compares a 16-bit register to 512; if they are equal, it writes 8 to a global variable, otherwise it continues to copy data from one memory location to another until a specific word is encountered.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_loop_variable`: Increments a word value by a constant amount in a loop until a value pointed to by a register equals negative one.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_game_state_and_display`: Reads hardware input from joysticks, mice, and paddles to update internal state and refreshes the display.
- `process_input_and_update_display`: Reads joystick, mouse, and paddle input via hardware registers to update internal state.
- `convert_digit_to_char`: Converts a byte value by adding the character zero to it and storing the result through a pointer.
- `copy_multiple_data_blocks`: Performs a nested loop to copy seven blocks of thirty-two bytes each from a table based on values in a source list.
- `clear_memory_blocks`: Clears a large block of memory by writing zero to a range of addresses and also clears several bytes at a specific hardware register location.
- `copy_nested_data_structures`: Executes a triple-nested loop to copy data blocks from a table into a global variable based on character offsets.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### Zzzep.adf:1#unpacked

- `fills_with_zeros`: Fills memory with zero bytes by repeatedly executing a no-operation instruction on each data register.
