# Game 987 (zzzep (1990)(h. jumpshoe)[b]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00987). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (912400 top-level bytes in 17 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 202 of 202 named (0 author symbols), 202 of 202 described (69 by rule, 121 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2950 of 2950 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 32768 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7270 of 7270 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 555400 |
| code | 23338 |
| data | 71449 |
| padding | 3653 |
| unused | 258560 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| type | 2 | 117 | 2 |
| starsmittel | 2 | 177 | 7 |
| shades | 2 | 208 | 7 |
| add21K | 3 | 480 | 33 |
| border | 3 | 223 | 6 |
| Zzzep (1990)(H. Jumpshoe)[b].adf:cls | 3 | 2549 | 141 |
| Zzzep (1990)(H. Jumpshoe)[b].adf:1 | 1 | 155 | 3 |
| Zzzep (1990)(H. Jumpshoe)[b].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2818 | 1 |
| Zzzep (1990)(H. Jumpshoe)[b].adf:1#unpacked | 1 | 533 | 1 |

## Cases handled

- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op CAFE) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address (play-rs-m1500): 2807 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- zzzep (1990)(h. jumpshoe)[b]/Zzzep (1990)(H. Jumpshoe)[b].adf:c/shades: PowerPacker 2.3 (xfd) stream found by ancient scan (3576 -> 4844 bytes) but only 0 of 3 sampled windows are in reference RAM: not used
- zzzep (1990)(h. jumpshoe)[b]/Zzzep (1990)(H. Jumpshoe)[b].adf:c/border: TNM Cruncher 1.1 (xfd) stream found by ancient scan (5576 -> 6752 bytes) but only 0 of 3 sampled windows are in reference RAM: not used
- zzzep (1990)(h. jumpshoe)[b]/Zzzep (1990)(H. Jumpshoe)[b].adf:1: crunched executable unpacked with ancient scan (DefJam Cruncher 3.2 (xfd), 11420 -> 32768 bytes), 10 of 10 windows confirmed in reference RAM, loaded at an address: recovered as u9
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 3 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 jump target, 1 displacement
- zzzep (1990)(h. jumpshoe)[b]/Zzzep (1990)(H. Jumpshoe)[b].adf:1: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 32768 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of d1 to d0 and then branches to another location.

### starsmittel

- `initialize_library_and_load_data`: Opens a library, checks for a specific magic identifier in a memory block, allocates memory, and populates a structure with data from a source buffer using a loop that performs arithmetic on bytes.
- `call_two_functions`: Copies a byte from a pointer in a0 to d3, then calls two different functions located at offsets from a provided pointer.
- `call_two_library_functions`: Calls two library functions using offsets from a provided pointer and returns a concatenated result.
- `call_two_library_functions_alt`: Calls two library functions using different offsets from a provided pointer and returns a concatenated result.
- `process_data_table`: Iterates through a table of data, performing calculations to determine a bit position and a memory address, then sets a bit at that address and stores the address in the table.
- `set_bit_and_advance`: Sets a bit in a global variable and advances a pointer by four bytes, then branches to another location.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.

### shades

- `parse_amiga_hunk_data`: Allocates memory and parses a structure containing AmigaDOS hunk type codes, performing relocations and data copying based on the hunk type.
- `increment_counter_if_nonzero`: Tests the value in d5 and returns immediately if it is zero, otherwise increments a counter in d6, sets d5 to one, and returns.
- `process_linked_list_data`: Traverses a structure via a pointer in a1, performs recursive calls to itself, and uses a loop to update values in a destination buffer based on data from a source.
- `decompress_back_reference_stream`: Decodes a packed byte stream by reading control values that either copy literal bytes or repeat earlier output with a length and offset from the stream.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_byte_cycle_counter`: Decrements a byte counter in d7 for each iteration of a loop controlled by d0, resetting it to a space character whenever it reaches zero, and returns.
- `load_and_or_data`: Loads a word from an offset of a2 into d4, extends d0 to a word, and performs a logical OR between a byte from a6 and d1.

### add21K

- `call_clear_block_and_store_pointers`: Calls a routine that zeroes forty-one 32-bit words starting at an offset from a base pointer taken from the stack and then stores the address of a small stack buffer.
- `configure_display_surface`: Looks up two handles through a lookup routine, releasing them via library stubs if a lookup fails, then sets a flag byte in an associated display structure and calls a routine with the sizes 640 and 256.
- `clear_block_and_store_pointers`: Zeroes forty-one 32-bit words starting at an offset from a base pointer taken from the stack, then stores the address of a small stack buffer and a global value into two fields of that same block.
- `call_function_pointer_and_recurse`: Checks if a global variable is non-zero to call a function pointer, then calls another routine recursively.
- `parse_script_and_initialize_objects`: Splits a text buffer into a null-terminated array of word pointers using a character class table and replaces separators with null bytes.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `initialize_state_then_return`: Calls another routine ten times, then calls an optional handler from a global and conditionally calls routines with values taken from globals.
- `set_d0_and_branch`: Sets d0 to a constant value and branches to another routine.
- `append_string_with_count`: Walks to the end of the destination string and appends bytes from the source, stopping at the source terminator or when the count argument runs out, then null-terminates the destination and returns it.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `copy_bytes_zero_padded`: Copies up to a counted number of bytes from the source, stopping early after a zero byte, then fills the rest of the destination up to that count with zeros.
- `process_indexed_table_entry`: Calculates an index into a table by multiplying a value from a structure by 6, checks if the index is within a range of 0 to 18, and if the entry at that index is zero, sets a value at a specific offset in a structure and returns -1; otherwise, it checks a bit in the entry, calls another routine if the bit is not set, clears the entry, and returns 0.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `jump_to_library_function`: Performs an indirect jump to a function address retrieved from a library base stored in a global variable.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `jump_to_library_function_alt`: Performs an indirect jump to a function address retrieved from a library base stored in a global variable using a different offset.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_384_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.

### border

- `call_relocation_handler`: Calls a routine at a fixed address and then jumps to address zero.
- `process_amiga_hunk_relocations`: Parses a list of AmigaDOS hunk types to perform memory allocation and relocation.
- `update_list_pointers`: Reads values from a source list and uses them to update a destination list by performing pointer arithmetic and copying values in a loop.
- `unpack_stream_with_pointer_shift`: Decompresses a packed stream by reading bits through a shift routine that takes a 32-bit value from the four bytes before the source pointer, shifts it right and sets its top bit, and writes the resulting bytes to a destination buffer.
- `shift_pointer_value`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `shift_bitmask_and_store_pointer`: Shifts a bit mask right one bit per iteration, reloading it from a memory location when it becomes zero, and stores the low word of a pointer into a global variable on each step.

### Zzzep (1990)(H. Jumpshoe)[b].adf:cls

- `jump_to_address`: Performs a jump to an address calculated by adding an offset to the program counter.
- `initialize_system_environment`: Processes command line arguments, initializes a global structure using library calls, and configures hardware registers and status bits.
- `traverse_linked_data_structures`: Checks a pointer at offset 32696 from a6; if non-zero, it conditionally calls a routine using a byte from offset 32735, then calls three other routines using longwords from offsets 32766, 32762, and 32754, and finally calls a routine using a word from offset 8 of a5.
- `process_command_line_arguments`: Calls another routine multiple times with various PC-relative addresses as arguments, then calls another routine with a cleared stack.
- `parse_console_input_string`: Reads a character from the console device and checks if it matches a carriage return.
- `check_and_reset_status`: Checks a byte at a specific offset within a structure and, if it matches a certain value, updates it and calls two other routines with specific parameters.
- `process_structure_data`: Checks a byte in a structure and, if it matches a specific value, performs a series of library calls and updates various fields within the structure.
- `call_library_functions`: Calls three different library functions using base addresses and offsets retrieved from a global variable.
- `update_structure_fields`: Allocates memory via library calls and, if successful, copies several word values from a structure into local variables before updating a global pointer and another structure field.
- `execute_library_calls`: Calls three different routines, including a delay function, using parameters derived from a global structure.
- `update_border_data`: Compares strings and, if a condition is met, updates several fields in a structure and modifies specific bits in a bitmask.
- `initialize_global_structure`: Allocates or updates a global structure by calling several routines to set pointers, clear values, and modify hardware register bits via a table.
- `move_data_to_memory`: Moves a longword from a location pointed to by the stack pointer to an offset of 155 from a pointer in a2, then moves a longword from an offset of 12 from a3 plus d7 to an offset of 2560 from a2.
- `set_status_bit`: Sets the third bit of a byte located at a specific offset from a pointer retrieved from a table and calls another routine.
- `clear_status_bit`: Clears the third bit of a byte located at a specific offset from a pointer retrieved from a table, then calls a library function.
- `parse_font_string`: Parses a string to extract font names and sizes, handling errors if required information is missing and copying substrings into a local buffer.
- `transfer_stack_and_branch`: Moves the value at the top of the stack into a6 and branches to a nearby instruction, otherwise it sets d2 to 0 and returns.
- `manage_font_data`: Interacts with a font library to load character sets and manages data structures by executing indirect function calls and traversing linked structures.
- `manage_font_library_data`: Interacts with a font library to load character sets and manage data structures by executing indirect function calls and traversing linked structures.
- `parse_signed_integer`: Converts a numeric string into a signed integer by skipping whitespace and handling optional sign characters.
- `parse_signed_integer_2`: Parses a signed 32-bit integer from a string by skipping whitespace and handling optional sign characters, using a 32-bit multiplication routine.
- `store_parameter_and_call`: Stores a parameter into a stack-based location and calls another routine.
- `initialize_dos_library_pointer`: Initializes a structure by zeroing out six words starting at a global variable, then retrieves a library base pointer from a global variable and calls a routine to check a bit in that library, or calls another routine to perform a task and stores the result in the structure.
- `load_address_offset`: Loads the value minus 32768 into register a4 and returns.
- `process_window_data`: Tokenizes strings, interacts with windowing and DOS libraries to retrieve pointers, and updates various fields within a structure based on indirect function results.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d5.
- `parse_string_into_tokens`: Parses a string by identifying tokens separated by whitespace or enclosed in double quotes, then stores these tokens into a new list.
- `copy_register_d0_to_d0`: Moves the contents of register d0 to register d0.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `set_limit_and_branch`: Sets register d0 to 32767 and branches to another routine.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `compare_two_byte_strings`: The routine compares two byte strings byte by byte.
- `copy_string_to_buffer_2`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `manage_window_state`: Calls several library functions to manipulate a window structure, updates a global variable with a result, and performs conditional logic based on that result before clearing the global variable.
- `move_d0_to_d5`: Moves the contents of register d0 to register d5.
- `clear_pointer_and_call`: Stores a parameter into a stack-based location, calls another routine, and then zeroes out the value at the pointer provided by the first parameter.
- `increment_and_store_byte`: The routine increments a value at a global variable location and stores a byte at the address previously held in that location.
- `call_parse_format_string`: Pushes a frame offset and a value from offset 8 of the frame onto the stack, then calls another routine via a PC-relative address, cleans up the stack, and returns.
- `execute_recursive_call`: Performs a recursive call and returns a 64-bit value formed by concatenating the original contents of register d1 with the value left in d1 by another routine.
- `parse_format_string`: Parses a format string to extract values for a printing or formatting routine, supporting integer conversion, width specifications, and various type specifiers.
- `perform_signed_division`: Reads two signed 32-bit integers, determines the sign of the result, calls another routine to perform division, and returns a 64-bit value combining the result and the second input.
- `prepare_signed_operands`: Clears a register, checks the sign of two different registers, negates them if they are negative, increments a counter if the first register was negative, and calls another routine before branching.
- `call_and_return_concatenated`: Calls another routine and returns a 64-bit value formed by concatenating the original contents of register d1 with the value left in d1 by the call.
- `recursive_call`: Calls itself and then returns.
- `perform_32bit_division`: The routine calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `compute_string_length_2`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `execute_placeholder_logic`: Pushes a value from a global variable onto the stack and calls another routine via a PC-relative address.
- `dispatch_conditional_routine`: Calls another routine, and if the value in d4 is 10, checks bit 7 of a byte at offset 12 of a structure pointed to by a value at offset 10 of the frame; if that bit is set, it calls another routine with a value of negative 65535.
- `branch_to_nearby_instruction`: Performs a short relative branch to an instruction located a few bytes before the current routine.
- `execute_conditional_update`: Loads a pointer from a table, compares its first word with a value at offset 4 of that pointer, and if the comparison does not result in a carry, calls another routine with a masked word from offset 8 of the frame.
- `update_byte_via_pointer`: Loads a pointer from a table, increments the pointer, and stores a byte from a global structure into the memory location pointed to by the original pointer.
- `iterate_memory_range`: Iterates through a range of memory using a loop, calling another routine at each step based on a calculated offset.
- `check_pointer_validity`: Loads a pointer from offset 8 of a structure into a2, and if that pointer is non-zero, sets d0 to 255; otherwise, it sets d0 to 0.
- `validate_and_clear_element_2`: Checks if an index is within a specific range and if the corresponding pointer in a table is non-zero; if valid and a specific bit is not set, it calls several routines to process the element before clearing the structure.
- `dispatch_to_iterator`: Loads a pointer from offset 8 of a structure, stores the address of another routine into a global variable, and if bit 4 of offset 12 of the structure is not set, sets d0 to 255.
- `process_structure_update`: If bit 2 of offset 12 of a2 is set, it calculates a difference between offset 0 and offset 8 of a2, calls another routine with that difference and offset 8, and if the result matches the difference, it sets bit 4 of offset 12 of a2 and clears offsets 0 and 4 of a2.
- `clear_structure_fields_3`: If a value at offset 12 of a5 is not 65535, it clears bit 2 of offset 12 of a2, clears the longwords at offset 0 and 4 of a2, and sets d0 to 0.
- `update_structure_and_byte_2`: If offset 8 of a structure is non-zero, it calls another routine, then if offset 16 of the structure is 1, it moves a byte from offset 13 of a5 to offset -1 of a5 and calls another routine; otherwise, it updates the structure with values from offset 8 and offset 13.
- `update_structure_and_byte`: Updates a 32-bit value at offset 4 of a structure by adding a 16-bit value from offset 16 to a value from offset 8, sets bit 2 of offset 12, increments the structure pointer, and stores a byte from a global structure into the new pointer location.
- `clear_structure_fields_2`: Checks a byte at offset 12 of a structure in a2, and if non-zero, compares a pointer derived from a2 against a constant address before returning.
- `branch_to_another_routine`: Performs a short branch to another routine.
- `clear_structure_fields`: Clears the 32-bit values at offsets 0, 4, and 8 of a structure pointed to by a2, then branches to another routine.
- `modify_structure_bits_2`: Calls another routine with a word value of 1024, then if the result is zero, writes 1 to offset 16 of a structure and sets offset 8 of that structure to the value of the structure pointer plus 14; otherwise, it writes 1 to offset 16 of the structure and sets offset 8 to the result of the call.
- `modify_structure_bits`: Writes a word to an offset of a structure, sets a bit in a byte, moves a pointer, and calls another routine before conditionally setting a bit.
- `traverse_linked_list_2`: Loads a pointer from a global variable into a2 and branches to another routine.
- `traverse_linked_list`: The routine traverses a linked list of pointers, calling another routine for each element, and finally clears a value at a specific offset from a global pointer.
- `traverse_linked_list_2_2`: Stores the address of this routine into a global variable, calls another routine with a value from a5 plus 8, and if the result is non-zero, updates a pointer in a structure.
- `update_pointer_table`: Copies a longword from a location offset by a negative value from a pointer in a4 to a2, then copies a longword from offset 8 of a2 to offset 4 of a2, stores a2 back to the same negative offset in a4, and branches to another routine.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `setup_pointer_and_branch`: Clears a register, subtracts a value from another register, loads a pointer from a global variable into a2, and branches to an earlier instruction.
- `copy_memory_loop`: Compares a pointer in a2 with a value derived from a global variable at offset 8 of a5, and if they are not equal, enters a loop that copies a longword from the address in a2 to the address in a3 and updates a2 to point to the next longword until a zero longword is encountered.
- `copy_memory_conditionally`: If register d0 is non-zero, copies the longword at the address in a2 to the address in a3.
- `call_library_function_2`: Loads a longword from a2 into a negative offset of a4, copies a longword from offset 4 of a2 to d0, adds 8 to d0, pushes d0 and a2 onto the stack, calls a library function, cleans the stack, and branches to another routine.
- `check_table_index`: Multiplies a word from offset 8 of a structure in a5 by 6, calculates an offset into a table using a value from a4, and if the resulting longword is non-zero and the original word is within a specific range, calls another routine; otherwise, it writes 2 to an offset in a4 and returns -1.
- `call_is_interactive_2`: Multiplies a word from offset 8 of a structure in a5 by 6, calls another routine using a pointer from a table, and returns 1 if the result in d0 is non-zero.
- `clear_d0_and_branch`: Sets d0 to zero and branches to another routine.
- `call_delete_file_2`: Calls another routine using a value from offset 8 of a5, and if the result in d0 is non-zero, calls a second routine and stores the result in a global variable.
- `clear_d0_and_branch_2`: Sets register d0 to zero and performs a short branch to another routine.
- `calculate_offset_and_test`: Calls another routine, multiplies the result by 6, adds that to a pointer derived from a global variable, and if a value at that pointer is zero and a comparison against a global variable passes, it writes 2 to a global variable and returns -1.
- `test_and_set_status`: Tests if the lower two bits of a word at offset 4 of a2 are non-zero; if they are, it branches to another location, otherwise it writes 5 to an offset in a4 and returns -1.
- `perform_io_operation`: Pushes a word from offset 14 of a5, two longwords from offset 10 of a5, and a pointer from a2 onto the stack to call another routine, then calls a second routine if the result is negative one.
- `move_d5_to_d0`: Moves the contents of d5 to d0 and branches to another routine.
- `check_status_and_call`: Calls another routine with a parameter of 4096, then if bit 12 of the result is set, it checks a value at a negative offset from a4 and calls another routine if that value is non-zero.
- `execute_jump_and_branch`: Calls another routine, clears d0, and branches to an earlier instruction.
- `write_fixed_text_then_callback`: Obtains an output handle, writes a four-byte text fragment from a fixed address via DOS calls, and then executes a conditional function.
- `execute_indirect_function`: Checks if a value at a specific offset from a pointer in a4 is non-zero and, if so, calls the function pointed to by that value, then calls another routine.
- `process_complex_structure`: Iterates through a loop based on a table value, performs multiple indirect function calls using various offsets, and checks a bit in a system register before executing a final indirect call.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `validate_and_clear_element`: Checks if an index is within a specific range and if the corresponding pointer in a table is non-zero; if valid and a specific bit is not set, it calls another routine and clears the pointer, otherwise it sets a status value and returns error code minus one.
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
- `call_library_function`: Performs an indirect jump to a function located at an offset within a library base stored in a global structure.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `dispatch_indirect_jump`: Performs an indirect jump to an address retrieved from a pointer located at a specific offset within a structure pointed to by a value in a register.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_indirect_library_function`: Loads a library base from a global variable and calls a function at a specific offset within that library.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `dispatch_indirect_function`: Loads a pointer from a global variable, applies a negative offset of 210, and calls the function located at that address.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `jump_to_indirect_address`: Performs an indirect jump to an address retrieved by accessing a pointer within a structure located at an offset from a global variable.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_378_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `jump_to_indirect_address_2`: Performs an indirect jump to an address retrieved from a pointer located at a fixed offset from a value stored in a global variable.
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

### Zzzep (1990)(H. Jumpshoe)[b].adf:1

- `decompress_bitstream_data`: Copies a block of memory to a new location and then processes a bitstream to decode data into a destination buffer using a series of bit-shifting and table-lookup operations.
- `shift_and_store_byte`: Moves a longword from a pointer in a0 to d0, stores the lower byte of d0 into a structure at offset 8 of a5, and rotates d0 right by one bit.
- `process_bit_stream`: Performs a loop that shifts d0 right and calls another routine when d0 reaches zero, then performs a second loop that calls another routine and moves bytes from a table to a stack based on a counter.

### Zzzep (1990)(H. Jumpshoe)[b].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_data_table`: Populates a data table with zero values by performing a series of bitwise OR operations with immediate zero operands on data registers, effectively initializing the table entries to zero.

### Zzzep (1990)(H. Jumpshoe)[b].adf:1#unpacked

- `fills_with_zeros`: Fills memory with zero bytes by repeatedly executing a no-operation instruction on each data register.
