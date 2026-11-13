# Game 986 (zzzep (1990)(h. jumpshoe)[cr prd][t +3 atz]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00986). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (960112 top-level bytes in 24 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 14 of 14 units identical (bytes and relocations), 46 raw-word lines |
| Functions named and described | pass, 354 of 354 named (0 author symbols), 354 of 354 described (71 by rule, 268 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 11697 of 11697 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 395852 bytes, 20 functions |
| Independent decoder cross-check (Ghidra against capstone) | 26874 of 26874 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 560690 |
| code | 63330 |
| data | 82144 |
| padding | 4092 |
| unused | 249856 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| type | 2 | 117 | 2 |
| starsmittel | 2 | 177 | 7 |
| shades | 2 | 208 | 7 |
| add21K | 3 | 480 | 33 |
| border | 3 | 223 | 6 |
| Zzzep (1990)(H. Jumpshoe)[cr PRD][t +3 ATZ].adf:HRT | 2 | 83 | 3 |
| Zzzep (1990)(H. Jumpshoe)[cr PRD][t +3 ATZ].adf:cls | 3 | 2549 | 141 |
| Zzzep (1990)(H. Jumpshoe)[cr PRD][t +3 ATZ].adf:1 | 1 | 89 | 3 |
| Zzzep (1990)(H. Jumpshoe)[cr PRD][t +3 ATZ].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12221 | 110 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 484 | 20 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Zzzep (1990)(H. Jumpshoe)[cr PRD][t +3 ATZ].adf:HRT#unpacked | 1 | 483 | 19 |
| Zzzep (1990)(H. Jumpshoe)[cr PRD][t +3 ATZ].adf:1#unpacked | 1 | 9749 | 1 |

## Cases handled

- runtime segment an address: 10811 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f100): 389 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- zzzep (1990)(h. jumpshoe)[cr prd][t +3 atz]/Zzzep (1990)(H. Jumpshoe)[cr PRD][t +3 ATZ].adf:c/shades: PowerPacker 2.3 (xfd) stream found by ancient scan (3576 -> 4844 bytes) but only 0 of 3 sampled windows are in reference RAM: not used
- zzzep (1990)(h. jumpshoe)[cr prd][t +3 atz]/Zzzep (1990)(H. Jumpshoe)[cr PRD][t +3 ATZ].adf:c/border: TNM Cruncher 1.1 (xfd) stream found by ancient scan (5576 -> 6752 bytes) but only 0 of 3 sampled windows are in reference RAM: not used
- zzzep (1990)(h. jumpshoe)[cr prd][t +3 atz]/Zzzep (1990)(H. Jumpshoe)[cr PRD][t +3 ATZ].adf:HRT: crunched executable unpacked with ancient scan (ByteKiller 1.3 (xfd), 3888 -> 6732 bytes), 4 of 4 windows confirmed in reference RAM, loaded at an address: recovered as u12
- zzzep (1990)(h. jumpshoe)[cr prd][t +3 atz]/Zzzep (1990)(H. Jumpshoe)[cr PRD][t +3 ATZ].adf:1: crunched executable unpacked with ancient scan (TetraPack 2.1 (xfd), 11600 -> 389120 bytes), 10 of 10 windows confirmed in reference RAM, loaded at an address: recovered as u13
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 3 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 jump target, 1 displacement

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of d1 to d0 and branches to a different location.

### starsmittel

- `initialize_library_structures`: Initializes library-related structures by calling library functions to allocate memory and find names, then populates a memory block with a sequence of calculated values and copies a block of data from a global source.
- `call_two_functions`: Clears d3, loads a byte from the address in a0 into d3, updates d2 with a pointer, then calls two different functions located at offsets from a provided pointer.
- `call_two_library_functions`: Saves registers, swaps a6 and a2, calls a library function using an offset from a pointer, loads an address into a1, calls another library function using an offset from a pointer, and restores registers.
- `call_two_library_functions_alt`: Saves registers, swaps a6 and a2, calls a library function using a different offset from a pointer, loads an address into a1, calls another library function using an offset from apointer, and restores registers.
- `calculate_and_set_bits`: Saves registers to the stack, loads a pointer from a table, reads a word from that pointer, and if not zero, performs arithmetic using two other words from the table to calculate two values, checks if those values are within specific bounds, and if so, sets a bit in a buffer at a calculated offset and stores the buffer pointer back into the table.
- `write_and_branch`: Stores the value 600 at an offset of negative 4 from a2, increments a2 by 4, and branches to a constant address.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.

### shades

- `process_amiga_hunk_relocations`: Parses a list of executable hunk types to perform memory allocation, relocation, and data copying.
- `increment_counter_if_nonzero`: Tests the value in d5 and returns immediately if it is zero, otherwise increments a counter in d6, sets d5 to one, and returns.
- `traverse_and_write_list`: Traverses a list of pointers, calling another routine to decrement a counter, and writes a calculated address back to a pointer in the list.
- `decompress_back_reference_stream`: Decodes a packed byte stream by reading control values that either copy literal bytes or repeat earlier output with a length and offset from the stream.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_byte_cycle_counter`: Decrements a byte counter in d7 for each iteration of a loop controlled by d0, resetting it to a space character whenever it reaches zero, and returns.
- `extend_and_or_byte`: Extends the word d0 to a long and orrs a byte from a memory location into d1.

### add21K

- `call_address_and_branch`: Calls a routine at a specific address.
- `configure_display_surface`: Looks up two handles through a lookup routine, releasing them via library stubs if a lookup fails, then sets a flag byte in an associated display structure and calls a routine with the sizes 640 and 256.
- `clear_block_and_store_pointers`: Zeroes forty-one 32-bit words starting at an offset from a base pointer taken from the stack, then stores the address of a small stack buffer and a global value into two fields of that same block.
- `call_conditional_function_and_recurse`: Checks if a global pointer is non-zero to call a function, then calls itself recursively and pushes a word from a stack-relative offset onto the stack before returning.
- `parse_script_and_initialize_objects`: Splits a text buffer into a null-terminated array of word pointers using a character class table and replaces separators with null bytes.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `initialize_state_then_return`: Calls another routine ten times, then calls an optional handler from a global and conditionally calls routines with values taken from globals.
- `set_d0_and_branch`: Sets d0 to a constant value and branches to another instruction.
- `append_string_with_count`: Walks to the end of the destination string and appends bytes from the source, stopping at the source terminator or when the count argument runs out, then null-terminates the destination.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `copy_bytes_zero_padded`: Copies up to a counted number of bytes from the source, stopping early after a zero byte, then fills the rest of the destination up to that count with zeros.
- `validate_and_clear_table_entry`: Checks if a value is within a certain range and if a pointer in a table points to a non-zero value, then calls a library function and clears the pointer if certain conditions are met.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_function_indirect`: Calls a library function using a pointer loaded from a global variable.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_function_via_offset`: Loads a library base from a global variable and calls a function at a fixed offset relative to that base.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_384_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.

### border

- `initialize_amiga_relocation`: Calls a routine to relocate Amiga executable hunks and then enters a loop that updates a global pointer, modifies a global 16-bit value, and checks a global flag before calling another routine.
- `process_amiga_hunks`: Parses a list of Amiga executable hunk headers to perform memory relocation and setup.
- `update_buffer_with_offset`: Calculates a new address based on a value in a register and a pointer, then copies a byte to that address in a loop.
- `unpack_stream_with_pointer_shift`: Decompresses a packed stream by reading bits through a shift routine that takes a 32-bit value from the four bytes before the source pointer, shifts it right and sets its top bit, and writes the resulting bytes to a destination buffer.
- `shift_pointer_value`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `shift_bitmask_and_store_pointer`: Shifts a bit mask right one bit per iteration, reloading it from a memory location when it becomes zero, and stores the low word of a pointer into a global variable on each step.

### Zzzep (1990)(H. Jumpshoe)[cr PRD][t +3 ATZ].adf:HRT

- `intro_bytekiller_depack`: Cracktro start: ByteKiller depacker that decodes the packed stream after it backwards into 50000 hex onwards, showing progress on COLOR00, checks the checksum and jumps to the unpacked cracktro at 50000 hex.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Zzzep (1990)(H. Jumpshoe)[cr PRD][t +3 ATZ].adf:cls

- `entry_jump`: Performs a jump to a PC-relative address.
- `initialize_system_environment`: Processes command line arguments, initializes a global structure using library calls, and configures hardware registers and status bits.
- `traverse_linked_data_structures`: Recursively traverses a tree or linked structure by checking if certain offsets within a data block contain non-zero pointers or valid identifiers, and calls subsequent routines or other functions based on the values found at those locations.
- `process_command_line_arguments`: It traverses a linked structure to process various command line options and settings.
- `parse_console_input_string`: Reads a character from the console device and checks if it matches a carriage return.
- `check_and_reset_status`: Checks a byte at a specific offset within a structure and, if it matches a certain value, updates it and calls two other routines with specific parameters.
- `process_structure_data`: Checks a byte in a structure and, if it matches a specific value, performs a series of library calls and updates various fields within the structure.
- `call_library_functions`: Calls three different library functions using base addresses and offsets retrieved from a global variable.
- `update_structure_fields`: Allocates memory via library calls and, if successful, copies several word values from a structure into local variables before updating a global pointer and another structure field.
- `execute_library_calls`: Calls three different routines, including a delay function, using parameters derived from a global structure.
- `update_border_data`: Compares two byte strings and, if the result is zero, clears a long-word in a structure, copies a value from a table, updates a pointer, modifies bits in a bitmask, and calls two other routines.
- `initialize_global_structure`: Checks a long-word in a structure, calls a routine, and if the result is zero, calls another routine and then calls a third routine with a parameter.
- `move_memory_blocks`: Moves a 32-bit value from a source address to a destination address using an offset and a register.
- `set_status_bit`: Sets the third bit of a byte located at a specific offset from a pointer retrieved from a table.
- `clear_status_bit`: Clears the third bit of a byte located at a specific offset from a pointer retrieved from a table.
- `parse_font_string`: Parses a string to extract font names and sizes, handling errors if required information is missing and copying substrings into a local buffer.
- `restore_stack_and_branch`: Restores a pointer from a6 to a7 and branches to another routine if a certain condition is met, otherwise it sets d2 to zero.
- `update_font_pointer_and_verify`: Traverses a linked structure and updates a pointer in a global variable with the result, then if the result is zero, calls another routine and subsequently calls a routine to load a character or font attribute from the newly stored pointer or a specific offset.
- `manage_font_library_data`: It interacts with a font library to load character sets and manage data structures related to font files.
- `parse_signed_integer`: Converts a numeric string into a signed integer by skipping whitespace and handling optional sign characters.
- `parse_signed_integer_extended`: Parses a signed integer from a string by skipping whitespace and handling optional sign characters, and calls a 32-bit multiplication routine.
- `store_parameter_and_call`: Stores a parameter into a stack-based location and calls another routine.
- `initialize_buffer_and_call_library`: Initializes a buffer with zeros using a loop, then checks a bit in a library-provided structure to decide whether to call a routine with a pointer or call another routine with a specific string or constant, otherwise it returns.
- `set_address_register`: Returns immediately after loading a constant into a register.
- `process_window_data`: Tokenizes strings, interacts with windowing and DOS libraries to retrieve pointers, and updates various fields within a structure based on indirect function results.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d5.
- `parse_string_tokens`: Parses a string or a list of strings, handling quoted or unquoted text by copying characters to a destination buffer and skipping whitespace or certain control characters.
- `no_op_move`: Performs a move of a register to itself.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `branch_to_offset`: Loads a constant into a register and then branches to an offset in another routine.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `compare_two_byte_strings`: Compares two byte strings byte by byte.
- `copy_string_to_buffer_2`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `update_window_properties`: Calls several library functions to open a resource, checks if a value in a global variable is non-zero, and if so, updates a structure in a pointer-based register with a new value, a new pointer, and a new offset, then calls a routine to clear a global variable and finally clears that global variable.
- `copy_register_to_memory`: Copies the value from d0 to d5.
- `clear_pointer_and_call`: Stores a parameter into a stack-based location, calls another routine, and then zeroes out the value at the pointer provided by the first parameter.
- `increment_and_store_byte`: Increments a value at a global variable location and stores a byte at the address previously held in that location.
- `call_parse_format_string`: Pushes a frame offset and a value from offset 8 of the frame onto the stack, then calls another routine via a PC-relative address, cleans up the stack, restores the frame and returns.
- `execute_recursive_call`: Performs a recursive call and returns a 64-bit value formed by concatenating the original d1 contents with the value le.
- `parse_printf_string`: Parses a format string to handle or skip certain format specifiers, such as width or precision, and calls another routine or updates a pointer-based buffer or counter.
- `perform_signed_division`: Checks the signs of d0 and d1, updates a register to track if the sign should be flipped, calls a 32-bit division routine, and if the sign flag or d0 is negative, negates the result and returns a 32-bit value combining the result with d1.
- `normalize_signed_values`: Calculates the absolute value of two signed longwords in registers d0 and d1, increments d4 by one if d0 is negative, and calls another routine if d1 is negative before branching to another routine.
- `call_and_return_concatenated`: Pushes d1 to the stack, calls another routine, and returns a 64-bit value formed by concatenating the original d1 contents with the value left in d1 by the call.
- `recursive_call`: Calls itself and then returns.
- `perform_32bit_division`: Calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `compute_string_length_2`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `push_structure_member_and_call`: Pushes a pointer to a structure onto the stack, pushes a word from that structure onto the stack, and calls another routine.
- `execute_conditional_branch_logic`: Sets up a stack frame, pushes a word from a structure, calls another routine, and if the result is not equal to a certain value, checks a bit in a byte from a structure and calls another routine if a certain bit is not set.
- `branch_to_target`: Performs a relative branch to a specific address.
- `copy_and_validate_range`: Sets up a stack frame, loads a pointer from a structure, compares two pointers, and if the first is not less than the second, calls another routine after pushing a word from a structure onto the stack.
- `copy_byte_and_increment`: Copies a byte from an offset of a structure in a2 to a memory location pointed to by a0, then increments the pointer in a2 and performs a branch.
- `iterate_memory_range`: Iterates through a range of memory, calling another routine at each step, and stops when a certain end-address or limit is reached.
- `branch_if_not_zero`: Loads a pointer from a structure, and if it is non-zero, branches to another routine; otherwise, it sets a register to a constant and returns.
- `validate_and_close_handle_2`: Checks a byte at an offset, and if certain bits are set, calls a routine, updates a register, and calls another routine.
- `initialize_memory_range_pointer`: Sets up a stack frame, loads a pointer from a structure, and sets a global pointer to a certain address, then checks bit 4 of a byte at an offset of a2 and returns or otherwise returns.
- `update_and_branch_on_error`: Checks bit 2 of a byte at offset 12 of a2, and if set, calculates a difference between two words from a structure, calls another routine, and if the result matches the difference, clears a long or a long at offset 4 of a2 and sets bit 4 of a byte at offset 12 of a2 before branching.
- `clear_structure_on_condition`: Checks if a word at an offset of 12 from a pointer in a5 is orf-equivalent to ffff, and if so, clears a long or a long at an offset of 4 from a2, then branches to another routine.
- `update_structure_and_branch_3`: Checks a value in a structure, calls a routine if it is zero, and if a certain condition is met, updates a byte in a structure, calls another routine with a value from a structure, and updates a global word and pointer.
- `update_structure_and_copy_byte`: Updates a pointer in a structure, calculates a new value by adding a value from a structure offset to a value from another offset, sets a bit in a structure, increments a pointer, and copies a byte from a structure offset to a memory location pointed to by a0.
- `traverse_and_clear_structure`: Checks a byte at an offset of 12 from a pointer, and if it is non-zero, increments that pointer and checks if it has exceeded a certain limit before returning.
- `branch_to_routine`: Performs a short branch to another routine.
- `clear_structure_fields`: Clears a long-word at a pointer in a2, clears a long-word at an offset of 4 from that pointer, clears a long-word at an offset of 8 from that pointer, moves the pointer to d0, and branches.
- `update_structure_and_branch_2`: Calls a routine, and if the result is zero, sets a word at an offset in a structure and updates a pointer in a structure, otherwise it sets a word at an offset and updates a pointer in a structure.
- `update_structure_and_branch`: Writes a word to an offset in a structure, sets a bit in a structure, and calls another routine before branching to a new location.
- `initialize_list_traversal`: Sets up a stack frame, saves registers, loads a pointer from a global variable into a2, and branches.
- `traverse_linked_list`: Traverses a linked list of pointers, calling another routine for each element, and finally clears a value at a specific offset from a global pointer.
- `initialize_list_traversal_2`: Stores a pointer to a routine in a memory location, pushes a value onto the stack, and calls a library function, then updates a pointer if the return value is non-zero.
- `copy_and_increment_pointer`: Copies a long-word from a location relative to a4 to a2, copies a long-word from a5 plus offset 8 to a2 plus offset 4, updates a4 with the new a2, sets d0 to a2 plus 8, and branches back to a previous instruction.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `copy_long_word_and_branch_2`: Calculates a difference between two registers and branches to a new location.
- `copy_long_word_and_branch`: Compares a pointer or address in a register against a value at an offset in a structure, and if they are not equal, copies a long-word from the address in a2 to a3 and then branches to another routine.
- `load_and_call_library_2`: If d0 is not zero, copies a long-word from the address in a2 to the address in a3 and then branches to another routine.
- `load_and_call_library`: Loads a long-word from a pointer in a2 (offset relative to a4) into a2, increments d0 by 8, pushes d0 and a2 onto the stack, calls a library function, restores the stack, clears d0, and branches back to a previous instruction.
- `validate_structure_index`: Multiplies a word from a structure by six, adds it to a base address, and checks if the resulting pointer or a value at a certain offset in a structure is within a certain range or non-zero, otherwise it writes a value to a global variable and returns.
- `call_is_interactive_2`: Multiplies a word from a structure, loads a value from a global pointer, pushes it onto the stack, calls a library function, and then either branches or sets d0 to a constant.
- `branch_with_zero_d0`: Sets d0 to zero and branches back to a previous instruction.
- `call_delete_file_or_error`: Pushes a word from a structure onto the stack, calls a routine, and if d0 is non-zero, calls another routine, updates a global word, and sets d0 to a constant.
- `clears_d0_and_branches`: Clears register d0 and branches to another routine.
- `check_and_set_error_state_2`: Loads a word from a structure, calls a routine, and if the result multiplied by six, when added to a global address, satisfies certain range and null checks, it writes a word to a global variable and returns a constant.
- `check_and_set_error_state`: Checks if certain bits are set in a word at an offset in a structure, and if so, writes a value to a global variable and returns otherwise it jumps back to the start of the routine.
- `execute_write_with_error_check`: Pushes three values onto the stack, calls a routine, and if the return value is not negative one, otherwise it calls another routine, updates a global word, and sets d0 to negative one.
- `branch_with_d5_value`: Copies the value from d5 to d0 and branches back to a previous instruction.
- `execute_callback_and_branch_2`: Calls a routine with a parameter, and if the result has bit 2 or 3 set, checks a word in a global structure and calls another routine if it is non-zero.
- `execute_callback_and_branch`: Calls another routine and then branches to a new location.
- `write_text_and_execute_callback`: Obtains an output handle, writes a four-byte text fragment from a fixed address via DOS calls, and then executes a conditional function.
- `execute_indirect_function`: Checks if a value at a specific offset from a pointer is non-zero and, if so, calls the function pointed to by that value.
- `process_complex_structure`: Iterates through a loop based on a table value, performs multiple indirect function calls using various offsets, and checks a bit in a system register before executing a final indirect call.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `validate_and_close_handle`: Checks if an index is within a certain range and if a pointer or a certain bit in a structure is valid, then calls a routine to close a handle and clears a pointer if successful.
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
- `call_library_vector_108_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_function_414`: Calls a library function at an offset of minus 414 from a library base stored in a global variable.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_function_198`: Calls a library function at an offset of minus 198 from a library base stored in a global variable.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_function_294`: Calls a library function at an offset of minus 294 from a library base stored in a global variable.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_function_210`: Calls a library function at an offset of minus 210 from a library base stored in a global variable.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_function`: Loads a library base from a global variable and calls a library function at an offset of 552 minus the base address.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_378_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_function_offset`: Loads a library base from a global variable and calls a library function at an offset of 384 minus the base address.
- `call_library_vector_384_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_492`: Library stub: loads a library base from a global variable and calls the library function at offset minus 492 of that base, returning its result to the caller.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_576`: Library stub: loads a library base from a global variable and calls the library function at offset minus 576 of that base, returning its result to the caller.
- `call_library_vector_498_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_570`: Library stub: loads a library base from a global variable and calls the library function at offset minus 570 of that base, returning its result to the caller.
- `call_library_vector_582`: Library stub: loads a library base from a global variable and calls the library function at offset minus 582 of that base, returning its result to the caller.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_132_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_384_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_30_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.

### Zzzep (1990)(H. Jumpshoe)[cr PRD][t +3 ATZ].adf:1

- `expand_bit_pattern_to_table`: Reads a long-word from a pointer, and if certain bits are set, it uses a loop to write a sequence of bytes (derived from bit-shifting and a counter) into a destination table, while also updating a secondary pointer and potentially calling another routine or updating a global variable.
- `shift_and_store_byte`: Loads a long-word from a pointer, extracts a byte, stores it in a table, sets certain flags, and performs a right shift on the loaded long-word.
- `process_bit_shifts`: Performs bit shifts on d0 and, if zero, calls another routine and updates d1, using a loop that decrements d1, and otherwise executes a routine that copies bytes from a source pointer to a destination pointer while incrementing a counter.

### Zzzep (1990)(H. Jumpshoe)[cr PRD][t +3 ATZ].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `access_data_table`: Accesses a data table containing or-ing instructions that do not affect any register or flags.
- `load_data_table`: Loads a sequence of zeroed data words into a memory range, acting as a lookup table or a block of zeroed-out constants.
- `store_null_byte_data_table`: Acts as a data table containing a sequence of instructions that perform no operation, specifically storing zero into data registers.
- `identify_data_table_pattern`: This routine consists of a sequence of instructions that clear the lower bits of data registers, acting as a data table that other code accesses via absolute move instructions.
- `access_data_table_2`: Accesses a data table containing or-ing instructions that do not modify any registers.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `initialize_runtime_state_and_loop`: Sets several global variables and routine pointers, then enters a loop that decrements a counter, checks a global condition, and calls other routines or modifies a global pointer until a specific condition or a certain value in a global variable is met, while also enabling DMA channels via DMACON.
- `fill_memory_with_pattern_2`: Sets up pointers and a counter, then calls a routine three times to fill memory areas with a pattern using a constant, a source pointer, and an incrementing value.
- `clear_memory_blocks`: Checks a global variable and if certain conditions are met, clears or writes zero to several bytes at specific offsets relative to a pointer, and uses a loop to clear a block of memory.
- `populate_array_loop`: Initializes a loop that writes a specific 32-bit pattern and a 16-bit value into a destination array, while copying or updating values from a source array, and resets the destination pointer if a negative value is encountered.
- `iterate_and_call_subroutine`: Loads pointers from a list, checks if a pointer is non-negative, and calls another routine while advancing the pointer list.
- `test_global_word`: Tests a word in a global variable and branches if non-zero; otherwise, it tests another global word and branches if zero; if both conditions are met, it loads a pointer into a5, moves a value into a0, and branches to another routine.
- `update_status_fields`: Copies several words from a structure and other global variables into a new set of locations, then checks a global word; if that word is zero, it updates several fields in the structure, performs arithmetic and bit shifts on a global pointer and its high word, and calls two other routines.
- `update_status_structure_fields`: Checks a global word value to conditionally clear or set fields in a structure, performs arithmetic and bit shifts on several values, and calls update_status_and_flags.
- `update_status_and_flags`: Sets several values in a structure pointed to by a global variable, calculates two values using arithmetic and bit shifts, calls update_status_byte_with_shift and update_flags_from_registers, and clears three specific memory locations if certain conditions are met.
- `process_state_changes`: Checks a global long value and compares a word from a table with another global word; if they are equal, it decrements a counter and, if the counter reaches zero, branches to either update_global_word, modify_global_bits_2, or store_registers_to_memory based on the value in the table.
- `update_global_word`: Checks if a global word equals 2, and if not, increments it and jumps to another routine.
- `modify_global_bits_2`: Checks if a global long-word is zero, otherwise decrements it, and then loads a pointer from a nearby address and calls another routine.
- `store_registers_to_memory`: Stores the current value of d0 and the constant 8 into two specific memory locations and then returns.
- `initialize_buffers_and_copy_3`: Sets two global long-words, calls several routines, and if a certain global long-word is non-zero, copies a block of memory from a source table to a destination table using a loop that decrements a counter.
- `execute_indirect_call`: Tests a long value at a global address, and if it is not less than or equal to zero, loads a word from a table, shifts it left by two, and jumps to the address found at that offset in another table.
- `execute_two_subroutines`: Calls two different subroutines and then returns.
- `execute_two_subroutines_2`: Calls a routine that updates a global word with a bounds check, then calls a routine that decrements a global long-word, and then returns.
- `execute_clamped_updates`: Calls two different routines and then returns.
- `execute_decrement_and_increment`: Calls two different routines and then returns.
- `execute_double_increment`: Calls the same routine twice and then returns.
- `decrement_and_increment_clamped_globals`: Calls a routine to decrement a 32-bit global variable and then calls another routine to increment a 16-bit global variable while capping its value at 16, before returning.
- `decrement_and_increment_clamped_globals_2`: Decrements one 16-bit global variable and increments a different 16-bit global variable, capping the latter at 16, then returns.
- `decrement_two_globals`: Calls decrement_and_clamp_global and then calls decrement_if_positive.
- `decrement_and_update_bounds`: Calls decrement_and_clamp_global and then calls update_global_with_bounds_check.
- `increment_clamped_global`: Increments a 16-bit global variable and caps its value at 16.
- `decrement_and_clamp_global`: Decrements a global word and, if the result or a related value is less than negative or certain threshold, sets that global word to a negative constant before returning.
- `increment_clamped_global_alt`: Increments a different 16-bit global variable and caps its value at 16.
- `update_global_with_bounds_check`: Decrements a global word, checks if it is less than a certain value, and if so, sets it to a constant value before returning.
- `decrement_global_long`: Checks if a long-word global variable is negative or zero, otherwise it decrements the long-word by one and returns.
- `increment_global_long`: Increments a long-word global variable by one and returns.
- `decrement_if_positive`: Tests a global word and, if it is not zero and not negative, decrements it and then returns.
- `increment_global_word`: Increments a global word and then returns.
- `update_global_from_table`: Checks if a global variable is zero, and if so, adds a value from a table-based pointer to another global variable; otherwise, it adds a value from a table-based pointer to a different global variable and returns.
- `add_to_global_variables`: Performs bitwise OR operations on register d2, adds a register to a global variable, and then adds the value of another global variable to a third global variable.
- `update_pointer_bounds`: Copies a word from a source address to a destination address, clamps the source value between zero and a certain limit, and updates a destination address, then calls another routine and returns.
- `calculate_and_store_pointers`: Calls another routine, then calculates a value using a multiplication and bit shifts, stores a bitwise-modified version of that value into a global long-word, and subsequently updates several memory locations using a pointer or a base address with various offsets and bit-swapped values.
- `modify_ciaa_registers`: Clears a byte at a global address, reads a byte from a CIAA register, sets bit 6 of that register, performs a bitwise NOT and a rotate right on the byte, stores the result in the global address, and clears bit 5 of the same CIAA register.
- `decrement_global_if_nonzero`: Tests a 16-bit global variable and, if it is not zero, decrements it and branches to another location.
- `check_and_set_global`: Compares a global byte to a constant, and if it matches, sets a global word to a constant and checks a certain condition on another global word, potentially setting that word to a negative constant before branching.
- `clear_global_word`: Clears a 16-bit global variable and then returns.
- `set_audio_registers`: Sets a loop counter, and then loops to write a value to a register, followed by writing several constants to audio-related registers and a DMA control register, then returns.
- `configure_audio_channels`: Sets the DMA controller to enable audio channels, performs two delay loops, configures both audio channels with specific starting addresses, lengths, periods, and volume levels, enables DMA, performs another delay loop, and finally updates the audio channel addresses and lengths.
- `invert_byte`: Inverts the bits of a byte stored in a global variable and then returns.
- `set_global_values`: Stores a constant into a global variable and another global variable before jumping to a vector.
- `set_global_values_2`: Stores a constant into a global variable and another global-pointer variable before jumping to a vector.
- `set_global_values_3`: Stores a constant into a global variable and another global variable before jumping to a routine pointer.
- `initialize_data_blocks`: Sets a global pointer and a routine pointer, calls a routine to fill a memory range, then sets a different global pointer and routine pointer, calls a routine to fill another memory range, and finally calls a routine with a specific constant and a pointer.
- `initialize_global_state`: Stores a series of constants into several global variables and calls a routine that sets a pointer to a routine, then calls other routines to update pointers and registers.
- `initialize_global_state_and_pointers`: Sets several global variables and routine pointers to zero or constant values, updates a global pointer to a specific variable, and calls a sequence of other routines to reset or update various system-wide pointers and counters before returning.
- `clear_memory_block`: Clears a sequence of 18 bytes starting at a global address and then returns.
- `copy_and_increment_data`: Copies a word from a source pointer to a destination pointer and increments the destination pointer and d1, repeating this via a loop until d0 reaches minus one.
- `fill_memory_with_pattern`: Fills a memory area with a pattern of d2, a constant, d1, and a source pointer, incrementing d2 and the source pointer, repeating this via a loop until d0 reaches minus one.
- `clear_struct_members_2`: Loads a pointer from a global variable and calls a routine multiple times with that pointer or that pointer plus a certain offset.
- `clear_struct_members`: Clears several 16-bit offsets within a structure pointed to by a1, repeating this via a loop until d7 reaches minus one.
- `update_audio_period`: Calculates a new period value based on a global variable, updates the AUD0PER and AUD1PER hardware registers, decrements a counter in a global variable, and if the counter reaches zero, shifts a table of words and stores a value from a global variable into the table.
- `update_tables_and_loop`: Loops through a list to call other routines, then updates several global pointers with a constant and calls other routines to update further global pointers.
- `perform_bitwise_operations`: Compares the value pointed to by a0 with a5 and then performs a series of bitwise OR operations on registers d2 and d4 using various immediate byte constants.
- `copy_and_swap_list`: Uses a loop to move a longword from a pointer-based list into a data register, increments the pointer, and calls another routine, repeating this for a fixed number of iterations before returning.
- `apply_bitwise_masks`: Performs a bitwise AND between d6 and d3, then performs a series of bitwise OR operations on register d2 using various immediate byte constants.
- `swap_and_store_half_word`: Swaps the high and low words of d6 and stores the result in a structure at offset 6 or 2 of a0.
- `copy_and_clear_list`: Loops through a list, copying a word from a source pointer and a word from an offset of 20, then clears a destination pointer and returns.
- `copy_and_clear_list_offset`: Loops through a list, copying a word from an offset of 80 and a word from an offset of 120, then clears a destination pointer and returns.
- `initialize_global_pointers`: Sets a pointer to a routine, calculates new values for several global words using a combination of a global word and a constant, calls a routine to update a certain global, and then copies several pointers to a series of global pointers before returning.
- `update_status_byte_with_shift`: Shifts d0 right by one, then updates a byte at offset 1 or offset 3 of a6-based structure and sets or clears bit 0 of that byte accordingly.
- `update_flags_from_registers`: Stores d0 at a structure offset, then sets or clears bit 2 of a structure offset 3 based on bit 8 of d0, then stores d1 at offset 2 and sets or clears bit 1 of a structure offset 3 based on bit 8 of d1.
- `initialize_global_table`: Loops a certain number of times, storing an incrementing value (starting at 0 and increasing by 176 each iteration) into a global table.
- `populate_and_clear_buffer`: Uses a loop to call a routine and store the result in a buffer, then uses a second loop to clear a range of memory at a fixed address, and then returns.
- `subtract_constant_from_list`: Loops through a list, subtracting a constant value from each word, until a pointer reaches a calculated end address.
- `calculate_and_set_bit_flags`: Iterates through a list of data structures, calculating two new values by dividing input words by a third value and adding constants, then stores these results in a destination buffer.
- `check_bit_flags`: Calculates an address using d7 and d1, then checks a bit in d2 at that address and several fixed offsets, and if the bit is clear at the first address, sets that bit and jumps to another routine.
- `check_and_set_bit`: Calculates an address using d7 and d1, then checks a bit in d2 at that address and several fixed offsets, and if the bit is clear at the offset of 44, sets that bit and jumps to another routine.
- `test_and_set_bit_flags`: Checks if a specific bit, determined by a register, is clear at certain offsets (0, 44, 88, and 132) within a memory block, and if all checks pass, sets that bit at the first and second checked offsets before decrementing a loop counter and returning.
- `update_global_multiplier`: Reads a 16-bit value from the orvpos register, multiplies it by a value stored in a global variable, adds 7, and writes the 32-bit result back to that same global variable before returning.
- `write_sequence_to_memory`: Decrements a pointer in a4, calls another routine and writes the result to the address in a4, calls another routine and writes the result to the address in a4, writes a constant to the address in a4, and jumps to another routine.
- `update_bit_flags`: Calculates an offset using a value from a pointer in a6, checks two specific bits in a structure at that offset, and if both are clear, clears a bit at a certain offset and another bit at a different offset in that structure before returning.
- `initialize_buffers_and_copy_2`: Sets a global variable to a constant, then enters a loop that increments five consecutive bytes in a global buffer by the character zero and calls a routine that clears a buffer and copies memory, continuing until a routine returns a non-zero value.
- `swap_list_elements`: Iterates through a list of structures, and if a certain condition is met, swaps several longwords and word-sized values between adjacent members of the structure, then repeats the process until no more swaps occur.
- `update_string_buffer`: Checks a condition or a pointer, then loops through a buffer, decrementing a counter and potentially modifying characters in the buffer or resetting a counter to 4000 if a certain condition is met.
- `jump_to_dma_update`: Tests a word in a global variable and, if non-zero, jumps to the address stored in a5.
- `update_dma_parameters`: Checks a condition involving a pointer and a value, then if certain criteria are met, writes a value to the DMA controller, clears a byte in a structure, and updates a pointer before jumping to another routine.
- `adjust_pointers_and_loop`: Increments a register, increments another register, and executes a loop that decrements a register until it reaches zero, then jumps to a location stored in a register.
- `copy_dma_parameters`: Iterates four times to copy three-word blocks from a source table to a destination table and updates the DMA control register with a value from a table with bit 15 set.
- `clear_dma_table`: Iterates four times to set a word in a destination table to a specific constant, clear a long value in a source table, and update a pointer in the destination table.
- `call_library_functions_2`: Calls a library function using a pointer from a global variable, stores the result in a global variable, then calls another library function using a pointer from a global variable and stores that result in a different global variable before returning.
- `close_library_handles`: Loads a library base into a6 and calls a routine at offset -414, then repeats the process with a second library base and routine before returning.
- `execute_library_calls_conditionally`: Loads a routine pointer from a table, calls a library routine, and if the return value in d0 is not zero, calls two more library routines using pointers from a table, otherwise it clears a global variable and returns.
- `calculate_scaled_value`: Multiplies the lower 16 bits of d7 by 2560, subtracts 1 from the result, and enters a loop that decrements the result until it is no longer negative.
- `decompress_bit_stream`: Reads a bitstream, calls routines to handle bit-refilling and bit-reading, and writes decompressed bytes to a destination buffer.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `decompress_string_data`: Searches for a specific string constant, then decompresses or copies bytes from a source buffer to a destination buffer based on a control byte.
- `unpack_and_store_data`: Searches for a specific four-byte pattern in memory, then iterates a number of times defined by a register to unpack three bytes into a single word and store it along with a counter into a destination table.
- `reset_status_and_check_input`: Clears two global words, writes a value to a custom chip register, moves a value from a custom chip register to d0, tests a bit in a hardware register and branches if set; otherwise, it tests a word in a global variable and branches if non-zero or if a specific condition is met.
- `set_status_values`: Stores the value 1 in a global variable, stores the value 65535 in another global variable, and branches to another routine.
- `clear_and_set_global`: Clears two words in a global variable, compares a word in d0 to 3 and branches if not equal; otherwise, it stores the value 3 in a global variable and branches to another routine.
- `compare_768_and_branch`: Compares a word in d0 to 768 and branches if not equal; otherwise, it stores the value 7 in a global variable and branches to another routine.
- `compare_256_and_branch`: Compares a word in d0 to 256 and branches if not equal; otherwise, it stores the value 1 in a global variable and branches to another routine.
- `compare_one_and_branch`: Compares a word in d0 to 1 and branches if not equal; otherwise, it stores the value 5 in a global variable and branches to another routine.
- `compare_word_and_branch`: Compares a word in d0 to 2 and branches if not equal; otherwise, it stores the value 4 in a global variable and branches to another routine.
- `compare_and_branch`: Compares a word in d0 to 259 and branches to another location if they are not equal; otherwise, it stores the value 2 in a global variable and branches to another location.
- `compare_and_store_short`: Compares a word in d0 to 769; if it is not equal, it branches to another location, otherwise it stores the value six into a global variable and branches to another location.
- `update_global_short`: Compares d0 to 512 and, if equal, stores the value 8 in a global variable.
- `copy_and_increment_words`: Copies a word from a source pointer to a destination pointer, increments the destination pointer by two, and increments d0, continuing until a certain value is found at the source pointer.
- `initialize_buffers_and_copy`: Clears a buffer, performs a loop that increments bytes in a buffer, and copies a block of memory from one location to another.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_and_memory`: Saves registers, configures certain hardware registers, clears a block of memory, calls several routines, updates a global pointer, and copies a block of bytes from a source to a destination before returning.
- `configure_runtime_flags`: Checks bits 2, 3, and 4 of a value or pointer, and if certain bits are set, writes a constant 16-bit or 32-bit value to a global variable or a second global variable, or clears a global word, before jumping to another routine.
- `push_and_divide`: Pushes the value of a2 onto the stack, decrements the byte at the end of a1, performs a signed division of d1 by d0, and masks d0 with itself.
- `update_buffer_and_call_subroutines`: Flips the bits of a global word, then either writes a constant to an offset of a structure in a6 or writes a different constant, followed by a sequence of calls to several subroutines and a final write to a structure offset before returning from an interrupt.
- `update_buffer_value`: Extracts a bit-shifted value from a pointer in a6, writes it to a global variable, and then either decrements or increments a counter in a global variable based on certain comparisons and a check of a global variable.
- `populate_buffer_with_data`: Clears a range of memory, then uses a nested loop to copy data from one memory area to another, using a calculated offset.
- `increment_and_search_buffer`: Increments a global variable, and if it equals two, clears it and searches a buffer for a specific value, replacing it if found.
- `shift_buffer_left`: Checks if a value in a global variable matches a value in a table, and if not, shifts a range of memory left and updates a global variable.
- `copy_and_interleave_data`: Copies data from two source pointers to a destination pointer, while decrementing a counter and adjusting source pointers based on a global variable value.
- `set_base_address`: Checks if a 16-bit value at offset 2756 of a structure in a5 is zero, then writes either a fixed address or a fixed address to offset 2752 of a5, writes a fixed address to offset 64 of a6, clears a 16-bit value at offset 102 of a6, writes 10836 to offset 88 of a6, and returns if bit 6 of a byte at offset 2 of a6 is not set.
- `update_bit_table`: Iterates 95 times, reading a pointer from a table, calculating an offset based on a value, and setting a bit in a target buffer at a calculated position.
- `update_table_values`: Iterates through a table, clearing a bit in a global bitset, and calculates a new value or bitmask to store in the table or a bitset based on a division or conditional check, then returns.
- `update_table_bounds`: Iterates through a table, calling another routine to check a value, and if that value masked with a bitmask is not greater than a certain constant, stores that masked value into a table or a different offset in the table, then returns.
- `update_value_loop`: Multiplies a 16-bit value from a structure in a5 by a 16-bit value from a structure in a6, adds 6537, and stores the result back in the structure in a5, repeating if the result is zero.
- `initialize_buffer_list`: Writes a sequence of 32-bit values, including a fixed address, a fixed address, and a fixed address, into a buffer starting at offset 4414 of a5, while incrementing a value and looping 127 times.
- `copy_data_blocks`: Copies 5 blocks of 6-byte or 10-byte data segments from a source buffer to two different destination buffers, incrementing destination pointers by 10 or 22 or 6 or 10 respectively.
- `process_string_and_reset`: Checks a counter, potentially updates a pointer, copies bytes from a source string to a destination, and if certain conditions are met, resets several values in a structure in a6 and sets a pointer to a new address.
- `map_string_to_table`: Iterates through a string, finds a character, and copies bytes from a source string to a target table at an offset calculated from a counter and a pointer, while incrementing a counter.
- `update_string_buffer_from_table`: Iterates through a sequence of bytes from a global variable, and depending on the value or a bitset check, copies or clears bytes in a destination buffer using offsets or calculated indices derived from a table-based lookup or a mathematical transformation of a value in a5.
- `update_joystick_state`: Increments a counter, checks a bit in a register, and if certain conditions involving a pointer and a bitset are met, decrements a byte at a pointer and potentially increments another byte if a bitset or comparison condition passes.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### Zzzep (1990)(H. Jumpshoe)[cr PRD][t +3 ATZ].adf:HRT#unpacked

- `initialize_hardware_and_memory_2`: Saves registers, sets custom chip registers, clears a block of memory starting at a pointer in a0 using a loop, calls five different routines, sets several custom chip registers and global variables, waits for a specific bit in a hardware register and a bit in a custom chip register, then copies a block of data from one location to another using a loop.
- `apply_conditional_memory_patterns`: Checks specific bits in a local status byte to conditionally write a 32-bit pattern and a 16-bit pattern to two different global memory locations, clears a 16-bit value in a global variable, and then jumps to another routine.
- `perform_sequence_of_calls`: Increments a byte at a global variable, bitwise inverts a global variable, sets a longword at an offset of a structure in a6 based on the value of the global variable, calls nine different routines, and sets a word at an offset of a structure in a6.
- `update_offset_value`: Extracts a value from a structure in a6, shifts it, and stores it in a structure in a5; it then conditionally increments or decrements a 16-bit value at an offset in a5 based on comparisons and a global variable.
- `fill_and_copy_tables`: Clears a block of memory and fills it with repeating values in a loop, then performs a nested loop to copy data from two source tables into a destination table using calculated offsets.
- `increment_and_check_sequence`: Increments a 16-bit value in a5, and if it reaches two, it enters a loop to check if a sequence of values in a structure matches a target value, potentially updating the last element of that sequence.
- `shift_and_insert_value`: Compares a value in a5 with a value in a structure at an offset from a0; if they differ, it shifts elements in the structure left by one and inserts the new value at the end.
- `copy_pointer_pairs`: Runs a loop that copies pairs of longwords from two source locations into a destination table, adjusting the source pointers with specific offsets and wrapping them around certain boundaries.
- `initialize_hardware_parameters`: Sets a 32-bit value in a5 based on a 16-bit check, then writes several longwords and word values into a structure in a6, and finally waits for a specific bit in a memory location to be set.
- `set_bits_from_table`: Iterates through a list of pointers, calculates a bit position using values from two tables, and sets a specific bit in a destination buffer at an offset derived from those values.
- `process_scaled_values`: Iterates 53 times to perform division and scaling calculations using values from two structures, updates a bit in a bitmask at a specific memory range, and stores results in a table.
- `store_masked_results`: Iterates 53 times to call another routine twice per loop, masking the results and storing them into two consecutive words in a table.
- `accumulate_multiplied_value`: Multiplies a value from a structure in a6 by a value in a structure in a5, adds a constant, and stores the result back in the a5 structure, repeating this until the result is non-zero.
- `populate_data_list`: Runs a loop that writes a sequence of longwords and constant values into a table starting at an offset in a5, incrementing the primary value by a large constant each iteration.
- `copy_data_blocks_2`: Copies four blocks of data from a source structure to two destination tables, where each block consists of a 32-bit word followed by a 16-bit word, using a loop that runs four times.
- `process_string_and_update_hardware`: If a counter at offset 5ec of a structure is non-zero, it decrements the counter and performs string matching to copy seven 32-byte blocks from a calculated source to a destination; otherwise, it updates several values in a structure pointed to by a6 and returns.
- `update_table_from_string`: Iterates through a string, and for each non-null character, finds its position in another table to copy seven 32-byte blocks from a source to a destination table, while incrementing two counters in a structure.
- `update_sprite_tile_data`: Iterates through a sequence of seven bytes from a global variable and, based on the value of each byte and a bitmask in a structure, writes sets of eight bytes from various tables into a destination buffer.
- `increment_counter_and_check_input`: Increments a byte at offset 2599 of a structure and, if it reaches four, resets it and checks bit 6 of the CIAA_PRA register; if the bit is clear, it decrements a value in a table at offset 2592, and if certain conditions involving a6 and a comparison between two tables are met, it increments a value in the first table.

### Zzzep (1990)(H. Jumpshoe)[cr PRD][t +3 ATZ].adf:1#unpacked

- `initialize_and_run_main_loop`: Clears a data register repeatedly, initializes various data structures and buffers, and enters a nested loop that manages memory buffers, audio registers, and custom DMA control while checking specific byte values in a global variable to trigger state changes and calls to other routines.
