# Game 986 (zzzep (1990)(h. jumpshoe)[cr prd][t +3 atz]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00986). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (907910 top-level bytes in 24 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 14 of 14 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 245 of 245 named (0 author symbols), 245 of 245 described (69 by rule, 162 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1892 of 1892 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 395852 bytes, 20 functions |
| Independent decoder cross-check (Ghidra against capstone) | 15692 of 15692 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 555400 |
| code | 18088 |
| data | 80489 |
| padding | 4077 |
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
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1036 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 484 | 20 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| Zzzep (1990)(H. Jumpshoe)[cr PRD][t +3 ATZ].adf:HRT#unpacked | 1 | 483 | 19 |
| Zzzep (1990)(H. Jumpshoe)[cr PRD][t +3 ATZ].adf:1#unpacked | 1 | 9749 | 1 |

## Cases handled

- extended reference run ignored: it does not re-execute 6 of the 1917 PCs the canonical run logged (11698 PCs logged; the harness no longer reproduces the game), so its PCs and RAM are not used
- reference run failed (Karl 4 Nov 2026): run ext slid through memory (an address: 1 64 KB pages each with >= 29491 distinct PCs executed, 9787 of its 11698 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m1500 slid through memory (an address: 1 64 KB pages each with >= 29491 distinct PCs executed, 9787 of its 11722 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m3000 slid through memory (an address: 1 64 KB pages each with >= 29491 distinct PCs executed, 9787 of its 11722 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-menu slid through memory (an address: 1 64 KB pages each with >= 29491 distinct PCs executed, 9787 of its 11722 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-mixed slid through memory (an address: 1 64 KB pages each with >= 29491 distinct PCs executed, 9787 of its 11722 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- runtime segment an address: 1024 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f100): 368 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 4 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 0 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- zzzep (1990)(h. jumpshoe)[cr prd][t +3 atz]/Zzzep (1990)(H. Jumpshoe)[cr PRD][t +3 ATZ].adf:c/shades: PowerPacker 2.3 (xfd) stream found by ancient scan (3576 -> 4844 bytes) but only 0 of 3 sampled windows are in reference RAM: not used
- zzzep (1990)(h. jumpshoe)[cr prd][t +3 atz]/Zzzep (1990)(H. Jumpshoe)[cr PRD][t +3 ATZ].adf:c/border: TNM Cruncher 1.1 (xfd) stream found by ancient scan (5576 -> 6752 bytes) but only 0 of 3 sampled windows are in reference RAM: not used
- zzzep (1990)(h. jumpshoe)[cr prd][t +3 atz]/Zzzep (1990)(H. Jumpshoe)[cr PRD][t +3 ATZ].adf:HRT: crunched executable unpacked with ancient scan (ByteKiller 1.3 (xfd), 3888 -> 6732 bytes), 4 of 4 windows confirmed in reference RAM, loaded at an address: recovered as u12
- zzzep (1990)(h. jumpshoe)[cr prd][t +3 atz]/Zzzep (1990)(H. Jumpshoe)[cr PRD][t +3 ATZ].adf:1: crunched executable unpacked with ancient scan (TetraPack 2.1 (xfd), 11600 -> 389120 bytes), 10 of 10 windows confirmed in reference RAM, loaded at an address: recovered as u13
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 3 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 jump target, 1 displacement
- SLIDEOK (fix-r4, 19 Nov 2026): the reference run slide was redone and slid again; its PCs are excluded and the 1892 clean executed game-code addresses pass proof 4, so the slide no longer blocks R (it still needs the G0 harness fix): reference run slides reproducibly (harness runaway at an address in the canonical and the extended run): re-running does not help; fix the reference harness at G0 (Karl 11 Nov 2026: send it back to the gate that can fix it)

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
- `update_font_pointer_and_verify`: Interacts with a font library to load character sets and manages data structures by executing indirect function calls and traversing linked structures.
- `manage_font_library_data`: It interacts with a font library to load character sets and manage data structures related to font files.
- `parse_signed_integer`: Converts a numeric string into a signed integer by skipping whitespace and handling optional sign characters.
- `parse_signed_integer_extended`: Parses a signed integer from a string by skipping whitespace and handling optional sign characters.
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

- `store_null_data_table`: Contains a sequence of instructions that perform logical OR operations with zero on data registers, serving as a static table of null values.

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

- `set_custom_stack_pointer`: Adjusts the stack pointer by four bytes, writes a specific memory address to a global variable, and saves several registers to the stack before returning.

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
