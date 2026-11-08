# Game 345 (hellzone - carnage extreme (1991)(interscan)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00345). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (906128 top-level bytes in 14 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 20 raw-word lines |
| Functions named and described | pass, 128 of 128 named (0 author symbols), 128 of 128 described (31 by rule, 88 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1043 of 1044 (99.90%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4201 of 4201 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 232 |
| code | 14036 |
| data | 509537 |
| padding | 2931 |
| unused | 379392 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Hellzone - Carnage Extreme (1991)(Interscan).adf:Level2 | 5 | 45 | 2 |
| More | 19 | 2290 | 76 |
| Type | 2 | 117 | 2 |
| X | 2 | 117 | 2 |
| Hellzone - Carnage Extreme (1991)(Interscan).adf:Intro | 5 | 253 | 7 |
| Hellzone - Carnage Extreme (1991)(Interscan).adf:Level1 | 5 | 45 | 2 |
| Hellzone - Carnage Extreme (1991)(Interscan).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 940 | 30 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 384 | 6 |

## Cases handled

- runtime segment an address: 584 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 211 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- hellzone - carnage extreme (1991)(interscan)/Hellzone - Carnage Extreme (1991)(Interscan).adf:Level2: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- hellzone - carnage extreme (1991)(interscan)/Hellzone - Carnage Extreme (1991)(Interscan).adf:Intro: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- hellzone - carnage extreme (1991)(interscan)/Hellzone - Carnage Extreme (1991)(Interscan).adf:Level1: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Hellzone - Carnage Extreme (1991)(Interscan).adf:Intro

## Copy protection found

- `process_text_buffer_and_environment` (More): Parses a string to check for a specific character, retrieves an environment variable, and performs a series of operations on a structure in a2 including parsing signed decimals, managing a buffer of 128 bytes, and calling various subroutines to handle text formatting and memory offsets. Kind: indirect function call/jump table trap. Pass path: the execution of the function pointed to by unaff_A3.

## Functions


### Hellzone - Carnage Extreme (1991)(Interscan).adf:Level2

- `init_and_check_pattern`: Invokes several routines and loops three times, then clears a memory location and, when a value matches a specific pattern, stores it and zeroes a global variable.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.

### More

- `load_and_initialize_file_data_2`: Loads a file into memory and parses its contents, including a string processing loop that handles quoted text and converts specific character sequences into newline or escape characters.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `load_and_initialize_file_data`: Parses a file header to populate a structure with metadata and then loads associated data into memory.
- `iterate_table_with_function`: Iterates through a table of four values, passing a pointer from a structure and each value from the table to another routine.
- `validate_input_buffer`: Calls another routine twice with arguments derived from an input pointer and then scans a range of bytes to check if they fall within certain allowed ranges.
- `process_input_character_and_dispatch`: Reads a character from a buffer and performs a series of range checks and comparisons to determine a response.
- `process_text_buffer_and_environment`: Parses a string to check for a specific character, retrieves an environment variable, and performs a series of operations on a structure in a2 including parsing signed decimals, managing a buffer of 128 bytes, and calling various subroutines to handle text formatting and memory offsets.
- `initialize_entity_state`: Sets a 32-bit field at offset 60 and a 16-bit field at offset 94 of a structure in a2, calls another routine with a pointer from offset 52, calls itself recursively if offset 100 is non-zero, and enters a loop that calls another routine while certain conditions regarding offset 60, offset 8 of a pointed-to structure, and offset 94 are met.
- `update_entity_and_increment_counter`: Calls another routine twice with arguments derived from the input pointer, stores the second result into a field of the pointed-to structure, and if a flag field at offset 4 is zero it increments a short located at an offset from a pointer stored in the structure.
- `parse_buffer_content`: Uses another routine to check a buffer, updates a size field, and scans for a newline or form-feed character while potentially modifying a flag or string content.
- `process_entity_list`: Determines a loop limit based on fields in a passed structure and iterates to call another routine using a pointer to a table and a global variable; it also recursively calls itself, calls another routine with a pointer to a stack buffer, and calls a library routine to reschedule.
- `handle_search_loop`: Uses a flag in an argument block to select a global variable and repeatedly calls another routine, updating a counter and looping while a search result is zero.
- `initialize_handler`: Calls another routine with zero, then with a parameter, then with a pointer from offset 16 of a structure, a global variable, and a parameter, conditionally calls another routine with the same arguments but without the parameter, calls another routine with two arguments, calls another routine with a constant, calls another routine with a parameter, and returns the result of the call involving two arguments.
- `check_flag_and_call`: Calls another routine with two arguments, then checks a short value at an offset from a pointer and conditionally calls another routine with one argument.
- `draw_scroll_window_bounds`: The routine calls a drawing routine using a pointer and a certain value, then calls another routine using a value or offset from a pointer-based structure.
- `draw_text_window_range`: The routine calls a drawing routine with a fixed or zeroed argument, then calls another routine with a decremented value derived from a pointer-based structure.
- `draw_text_line`: Uses a handler pointer to call drawing primitives while iterating over a text buffer, tracking line count and stopping if a certain character or flag is encountered.
- `dispatch_recursive_pointer`: The routine calls another routine with a single argument, then calls another routine using a pointer from a structure and a global variable.
- `set_flag_and_forward`: Checks a field in a passed structure, optionally sets a flag in a remote structure, and calls another routine using a global variable.
- `dispatch_by_flag`: Selects two pointers based on a flag in a parameter block and calls another routine using either a global variable or a table entry.
- `dispatch_handler_by_flag`: Reads a flag from a parameter block and calls another routine with either three references to a global variable or a value from a table entry.
- `clamp_negative_result`: The routine calls another routine twice with the same first argument and different third arguments, then clamps a negative return value to zero before returning it.
- `consume_from_buffer`: Checks a size field in a structure and, if a certain condition or partial way is met, updates several offset or remaining size fields and calls another routine with a pointer derived from the structure.
- `recursively_process_buffer_segments`: If a field at offset 120 is non-zero and a field at offset 116 is not 4096, it calls itself recursively with a pointer into the structure and a size from offset 120.
- `update_scroll_window`: Calculates a target position based on provided offsets and compares it against the current window bounds.
- `read_line_from_buffer`: Repeatedly calls another routine to read bytes from a source buffer into a destination buffer, stopping at a newline or a fixed limit, and appends a null terminator.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `convert_string_to_uppercase`: Iterates through a null-terminated string and converts any lowercase ASCII letters to uppercase by subtracting a constant.
- `parse_signed_decimal`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `call_library_function_with_mask`: Checks if a pointer in a structure or table is non-null, then calls a library function using a base address from a global variable and a provided mask.
- `process_message`: Recursively processes a structure by invoking itself on various fields and string arguments, conditionally updates fields at offsets 8, 6, and 7, and performs a loop based on a return value.
- `build_message_buffer`: The routine obtains a pointer from another routine and then calls another routine with that pointer, a constant, a stack-allocated buffer containing the input parameter, and a size of one.
- `copy_entity_data`: The routine obtains a handle from another routine and, if valid, reads a value from that handle, passes it to another routine together with a small stack buffer, and copies two fields from the handle into the caller-provided structure.
- `initialize_object_from_template`: Obtains a template identifier, allocates an object, links its fields to a template and source block, and copies words from a source address into the new object.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_alloc_abs`: Library stub: loads its arguments and the exec library base, calls AllocAbs and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `extract_byte_from_call`: Calls another routine with a pointer to a small stack buffer and a constant, returning a byte from that buffer if the call returns 1, otherwise returning negative one.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `compute_modular_result`: Computes a 32-bit result by repeatedly doubling and subtracting the divisor when the running remainder exceeds it, otherwise it performs a 16-bit modulo on the low half of the dividend and returns the combined quotient and remainder.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `forward_call_to_helper`: Calls one helper and returns, with no other work.
- `call_and_negate_on_flag`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_and_increment`: Stores a byte from register d0 into the memory location pointed to by a3 and increments a3.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `allocate_message_entry`: The routine attempts to allocate a message buffer, and if successful, populates several fields of that buffer with parameters and a value from another routine before dispatching it.
- `delete_port`: The routine removes the port from a global list if a specific field is non-zero, sets status fields to indicate deletion, frees the associated signal bit, and releases the memory block.
- `compare_strings_with_wrap`: Compares two null-terminated strings byte by byte, skipping the check if the first string contains a newline character.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.

### Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_and_branch`: Copies the contents of register d1 to d0 and branches to another location.

### X

- `find_max_and_prepare_buffer_2`: Walks a linked list to find the maximum value in a field, then writes a header and a sequence of words into a buffer.
- `transfer_register_and_branch`: Moves a longword from register d1 to register d0 and then performs a short relative branch to another routine.

### Hellzone - Carnage Extreme (1991)(Interscan).adf:Intro

- `init_and_check_pattern_2`: Invokes another routine and a function pointer three times in a loop, then clears a value at a pointer derived from a global variable.
- `decrement_until_minus_one_2`: Decrements a local variable until it reaches negative one and then returns.
- `decompress_and_expand_data`: Calls a routine via a pointer multiple times, uses the results to perform a loop that adds values to a destination buffer, and conditionally calls another routine via a pointer from a global structure.
- `load_address_and_set_register`: Loads a constant into register a1 using the program counter and sets register d0 to 2.
- `decompress_bitstream_data`: Decompresses a bitstream by reading bytes from a source buffer and expanding them into a destination buffer using a variable-length encoding scheme involving bit shifts and additions.
- `push_and_branch`: Performs a bitwise OR on a byte at a pointer, stores three values onto the stack, and branches to another location.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Hellzone - Carnage Extreme (1991)(Interscan).adf:Level1

- `invoke_routines_and_verify_pattern`: Calls a routine three times within a loop, then calls another routine and checks if a value at a specific offset within a structure matches a constant pattern.
- `decrement_register_until_negative_one`: Copies a 16-bit value from one register to another and decrements it in a loop until the value reaches negative one, then returns.

### Hellzone - Carnage Extreme (1991)(Interscan).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_blt_hardware_sequence`: Allocates memory and retrieves several pointers from a global structure, then populates a table with four sets of word values derived from a longword.
- `manage_memory_and_resources`: Runs a loop sixteen times calling two routines, updates two global variables from memory, calls a function via a pointer, performs four library calls to free or permit memory, and updates three shorts in a structure before calling a final library routine.
- `update_game_state`: Calls another routine and then modifies several global variables and a bit in a hardware register based on the value of a global variable.
- `populate_table_with_offsets`: If a byte at an offset of a5 is 36, it populates a table with a series of values derived from a long value in d0, incrementing that value by 10240 in each step.
- `copy_word_block`: Copies words from a source pointed to by a1 to a destination pointed to by a0, incrementing both pointers by their respective sizes in a loop controlled by d0.
- `zero_word_array`: Increments a pointer in a0 by 2, stores a 16-bit zero at that location, and loops using d0 as a counter.
- `wait_for_byte_zero`: Loops until a byte at offset 6 of a structure pointed to by a5 is zero, using d0 as a counter.
- `wait_for_byte_zero_2`: Loops until a byte at offset 6 of a structure in a5 is zero, then either moves a longword from the stack to a specific memory location and returns, or returns immediately if a bit in a global variable is set.
- `wait_for_byte_zero_d6`: Loops until a byte at offset 6 of a structure pointed to by a5 is zero, using d6 as a counter.
- `loop_and_pop_stack`: Loops until a byte at offset 6 of a structure in a5 is zero, then pops a longword from the stack into d6 and moves a longword from the stack to a specific memory location before returning.
- `copy_and_process_data`: Performs a nested loop structure that copies words from one memory location to another, calls another routine multiple times, and performs a second loop that waits for a byte at offset 6 of a structure in a5 to become zero before copying more words to a different destination.
- `decrement_encoded_values`: Iterates through a list of 16-bit words pointed to by a3, subtracting 256, 16, or 1 from each non-zero word based on its bit patterns, and loops using d1 as a counter.
- `execute_nested_data_copy`: Executes a series of nested loops that copy words from a fixed memory location to a function pointer area, calls another routine multiple times, and updates several global variables from values retrieved via another routine.
- `decrement_variable_length_list`: Iterates through a list of words pointed to by a register, decrementing the pointer by one, ten, or one hundred hundredths based on the high-order bits of each word, and repeats this process for a specified number of iterations.
- `decrement_encoded_values_a0`: Iterates through a list of 16-bit words starting at a0 plus 2, subtracting 256, 16, or 1 from each non-zero word based on its bit patterns, and loops using d0 as a counter.
- `decompress_bit_stream_data`: Reads a bit-stream from a source pointer in a0 and expands it into a destination buffer in a2 using variable-length bit sequences.
- `rotate_and_return_word`: Moves a longword from the address in a0 to d0, rotates the bits of d0 to the right by one, and returns.
- `process_bit_stream_data`: Iterates through a bit-stream by shifting a value in a register and calls another routine when the value reaches zero.
- `setup_data_blocks`: Initializes several fields in a structure at a5 with specific constants and pointers, then performs five repeated updates to the structure's fields using incremented pointer values.
- `configure_hardware_sequence`: Sets up a structure at a5 with various constants and pointers, then waits for a bit in a structure at offset 2 to clear before performing a second configuration update and waiting again.
- `configure_graphics_structure_parameters`: Updates multiple fields in a structure pointed to by a5, including setting two different memory address pointers and two different control words, while polling bit 14 of a byte at offset 2 of the structure before applying a final set of parameters.
- `configure_hardware_sequence_minimal`: Sets up a structure at a5 with specific constants and pointers, then waits for a bit in a structure at offset 2 to clear before performing a second configuration update and waiting again.
- `update_structure_fields`: Sets a 16-bit value at offset 102 of a structure in a6, a 32-bit value at offset 64, a pointer at offset 84, and a 16-bit value at offset 88, then checks bit 14 of a byte at offset 2 of the structure and repeats a similar update with a different pointer and value if the bit is not set.
- `initialize_structure_fields`: Sets a 16-bit field at offset 102 and a 32-bit field at offset 64 of a structure in a5, then enters nested loops that call two different routines and one other routine while updating a 16-bit field at offset 6 of a structure in a5.
- `clear_memory_blocks`: Loops a specified number of times, clearing bytes at various offsets relative to a pointer in a1 and a2 using values from d1 and d2.
- `update_structure_sequentially`: Performs a loop to calculate a value, then sequentially updates a 32-bit field at offset 84 and a 16-bit field at offset 88 of a structure in a5, repeating the update with an incremented pointer and checking bit 14 of a byte at offset 2 after each step.
- `update_structure_parameters`: Sets 16-bit fields at offsets 100 and 102 and a 32-bit field at offset 64 of a structure in a5, then enters nested loops that call two different routines and one other routine if a byte at offset 6 of a structure in a5 is zero.
- `copy_memory_blocks`: Loops a specified number of times, copying bytes from a source structure in a1 to a destination structure in a0 using offsets provided in d1 and d2.
- `update_dual_structure_fields`: Loops a specified number of times to calculate offsets, then sequentially updates a 32-bit field at offset 84 and a 32-bit field at offset 80 of a structure in a5, repeating the update with incremented pointers and a 16-bit value at offset 88.
- `process_data_and_loop`: Performs bitwise operations on d0 and a3, then enters a loop that checks values in a0 and performs bitwise operations and register changes.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_audio_and_hardware`: Loops through a table to clear memory, then clears several hardware registers including audio volumes and CIAA registers.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `update_audio_hardware`: Increments a global byte, then either calls four different routines to update audio hardware registers or performs arithmetic and table lookups to update four different audio hardware registers based on a global byte value.
- `update_audio_parameter_structure`: Reads a value from a structure to determine an index into a table, then updates several fields in the structure including a pointer, two 16-bit values, and a 32-bit value.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `update_object_state_and_hardware`: Updates a 16-bit value in a structure based on a command byte, performing arithmetic on its low 12 bits or clamping its value.
