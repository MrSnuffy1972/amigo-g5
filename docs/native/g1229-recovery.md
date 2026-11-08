# Game 1229 (mad tv (1992)(rainbow arts)(de)(disk 1 of 2)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01229). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 61 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 202 of 202 named (0 author symbols), 202 of 202 described (51 by rule, 142 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 212 of 213 (99.53%) |
| Recovered without load address (counted in proof 1, flagged) | 3599 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4423 of 4423 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 184766 |
| code | 14528 |
| data | 263091 |
| padding | 14351 |
| unused | 424448 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Mad TV (1992)(Rainbow Arts)(De)(Disk 1 of 2).adf:mtv | 7 | 233 | 7 |
| Disk-Validator | 1 | 13 | 2 |
| Mad TV (1992)(Rainbow Arts)(De)(Disk 1 of 2).adf:add21 | 2 | 26 | 1 |
| Mad TV (1992)(Rainbow Arts)(De)(Disk 1 of 2).adf:rai | 2 | 83 | 3 |
| Mad TV (1992)(Rainbow Arts)(De)(Disk 1 of 2).adf:v | 3 | 3636 | 155 |
| Mad TV (1992)(Rainbow Arts)(De)(Disk 1 of 2).adf:098.HES | 1 | 422 | 33 |
| Mad TV (1992)(Rainbow Arts)(De)(Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 0 | 0 |
| Mad TV (1992)(Rainbow Arts)(De)(Disk 1 of 2).adf:044.HES (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- reference crashes at an address (HLE illegal instruction an address at an address: through vector 0, as before (vector 4 is 0 again)): 109944 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- mad tv (1992)(rainbow arts)(de)(disk 1 of 2)/Mad TV (1992)(Rainbow Arts)(De)(Disk 1 of 2).adf:044.HES: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- mad tv (1992)(rainbow arts)(de)(disk 1 of 2)/Mad TV (1992)(Rainbow Arts)(De)(Disk 1 of 2).adf:mtv: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- mad tv (1992)(rainbow arts)(de)(disk 1 of 2)/Mad TV (1992)(Rainbow Arts)(De)(Disk 1 of 2).adf:v: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 3599 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Mad TV (1992)(Rainbow Arts)(De)(Disk 1 of 2).adf:mtv

- `free_blocks_and_clear_field`: Runs a loop that releases memory blocks through the system FreeMem entry, then zeroes a longword field in the caller's structure.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `process_data_stream`: Calls a routine multiple times using a pointer in a4, then enters a loop that calls a routine and performs arithmetic on a table at an offset from a3 based on the results.
- `load_address_and_set_d0`: Loads an address into a1 from a PC-relative offset and sets d0 to 2.
- `expand_compressed_data`: Decompresses a data stream using a variable-length encoding scheme where bit-shifts and additions are used to reconstruct bytes.
- `push_registers_to_stack`: Performs a bitwise OR on a byte at the address in a6 and increments a6, then pushes the values of d0, d1, d2, d3, d5, d6, a3, and a4 onto the stack before branching.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### Disk-Validator

- `perform_bitwise_loop`: Executes a loop that performs a bitwise OR on a byte at the address in a0 using a constant, and decrements d0 for a total of 44 iterations.
- `modify_memory_and_update_pointer`: Performs a bitwise OR on a word at the address in a2, adds d7 to a longword at the address in a6, and moves a word from a table into a4.

### Mad TV (1992)(Rainbow Arts)(De)(Disk 1 of 2).adf:add21

- `initialize_libraries_and_cleanup`: The routine calls library functions via base pointers, writes a byte through a pointer chain, and then calls additional library functions before returning.

### Mad TV (1992)(Rainbow Arts)(De)(Disk 1 of 2).adf:rai

- `decompress_bitstream_data`: Decompresses a bit-packed data stream into a destination buffer using a variable-length coding scheme.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Mad TV (1992)(Rainbow Arts)(De)(Disk 1 of 2).adf:v

- `jump_to_entry`: Performs an indirect jump to a fixed address.
- `dispatch_subroutine_calls`: Acts as a central dispatcher that invokes various specialized routines including data table clearing, error handling, entity state updates, list traversal, and file content validation.
- `read_file_data`: Calls a DOS function to read data into a buffer and updates a pointer and a status flag based on the number of bytes read.
- `read_file_buffer`: Reads data from a source into a destination buffer and updates a pointer and a status flag based on the number of bytes read.
- `process_resource_header`: Checks a word at offset 12 of a structure in a5 and calls another routine with a pointer from offset 8, then performs a series of conditional calls to other routines based on the returned value.
- `parse_resource_file_header`: Parses a file structure by reading a header and iterating through entries to identify specific magic markers.
- `load_and_parse_image_data`: Validates several error conditions by calling a routine to display messages, then parses a structure to set bits in a table at a4, calculates scaled values, and iterates through a list of data to perform memory operations and pointer updates.
- `update_entity_state_and_physics`: Updates a structure by setting specific bits in a status field based on several integer comparisons, calculates new values for two different state variables using division and subtraction, and calls another routine to process a data block before updating a pointer in the structure.
- `modify_structure_values`: Adds values from a source to a structure, performs several subroutine calls, and then subtracts those same values from the structure.
- `update_scaled_data_and_process_list`: Calculates several scaled values by multiplying a short value from a global variable by 300 and storing the results in a table at various offsets.
- `format_string_to_global`: Passes a pointer to a global variable and a value to a routine that parses a format string.
- `update_physics_state`: Calculates new values for several state variables by applying multipliers and conditional logic based on existing flags.
- `calculate_scaled_value`: Performs arithmetic on values from a global table to calculate a scaled result, which is then stored in several locations.
- `process_data_list`: Checks a value at an offset in a4; if it is not negative, it iterates through a table, calls another routine for non-zero entries, and clears a global variable.
- `copy_data_from_table`: Copies data from a table into a destination buffer based on a count, or copies a specific block of data and updates a value in a global variable.
- `store_scaled_values`: If a value in a global variable is not negative, it calculates an offset using a multiplier and stores three 16-bit words from a structure into a table at that offset.
- `call_function_with_arguments`: Extends four 16-bit words from offsets 8, 10, 12, and 14 of a structure into 32-bit values, pushes them onto the stack, and calls another routine.
- `update_coordinate_table`: Calculates and stores three 16-bit values into a table at offsets determined by a multiplier, or calls another routine to update them.
- `copy_scaled_values`: If a value in a global variable is not negative, it calculates an offset using multipliers and stores three 16-bit words from a structure into a table at that offset.
- `split_word_into_nibbles`: Calls another routine, stores the result in a local variable, and then splits that word into three separate values by masking and shifting bits to be stored at offsets 18, 14, and 10 of a structure.
- `handle_cleanup_error`: Clears a data table, executes several linked functions or library calls based on non-zero pointers, and performs an indirect call.
- `set_bit_in_structure`: Calls another routine and then sets the lowest bit of a byte located at offset 25 of a structure pointed to by a value from a4, then calls another routine.
- `clear_bit_in_structure`: Calls another routine and then clears the lowest bit of a byte located at offset 25 of a structure in a0, which is retrieved from a global variable via an offset from a4.
- `free_buffer_pointer`: If a global value is negative and a pointer is non-zero, it calls a library function to free the memory at that pointer and clears the pointer.
- `update_entity_state_and_physics_2`: Updates an entity's status flags and position based on conditional checks and external routine calls.
- `check_list_for_value`: Traverses a linked list and returns a boolean value if a specific pattern is found within the list elements.
- `calculate_offset_address`: If a value is less than a value at an offset in a structure, it returns the sum of a field in that structure and the input value multiplied by 8; otherwise it returns zero.
- `execute_nested_calls`: If a pointer at offset 8 of a structure in a5 is not null, it calls another routine with the value at that pointer, then calls another routine with a value from offset 4 of that same pointer, and finally calls another routine using the result of the first call.
- `check_status_conditions`: Returns 1 if four specific conditions in a global table are met, otherwise it returns 0.
- `allocate_and_copy_list`: Allocates memory for a new structure, copies data from a source structure, and performs a loop to copy elements from one memory region to another while potentially appending a terminator based on global variables.
- `process_linked_list`: Checks a value in a structure, and if it meets a threshold, it performs a series of memory allocations and recursive calls to process and update elements in a linked list.
- `process_nested_data_structure`: Checks a pointer at offset 8 of a structure in a5; if not null, it enters a loop that reads data from a source, validates a four-byte magic constant, and iterates through a list of segments by incrementing a pointer and calling other routines to process the data.
- `parse_and_split_text_string`: Reads a string from a file, splits it into a list of pointers based on non-printable characters, and then iterates through these segments to populate a structure with text data and metadata.
- `apply_xor_transformation`: Iterates through a memory buffer, performing a bitwise XOR operation on each byte using a value that increments with every iteration, then calls another routine.
- `load_constant_to_a4`: Loads the constant value 32766 into register a4 and returns.
- `initialize_structured_data`: Parses a string into tokens, populates a structure with values from a table and a global variable, and performs multiple library calls to handle file and directory operations.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `tokenize_string_into_array`: Parses a string by identifying words separated by whitespace or enclosed in double quotes, copying each token into a destination buffer.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `safe_memory_copy`: Copies a specified number of bytes from a source to a destination, handling overlapping memory regions by copying backwards if the destination address is higher than the source.
- `set_d0_and_branch`: Sets register d0 to 32767 and branches to another routine.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `compare_two_byte_strings`: The routine compares two byte strings byte by byte.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `update_window_structure`: Calls another routine to obtain a value, stores it in a global variable, and if non-zero, performs several calls to update various fields in a structure pointed to by a2, including setting a pointer and a value derived from a bit-shifted register.
- `move_d0_to_d5`: Copies the contents of register d0 to register d5.
- `call_parse_format_string`: Pushes a frame offset and a value from offset 8 of the frame onto the stack, then calls another routine via a PC-relative address, cleans up the stack, restores the frame and returns.
- `invoke_two_subroutines`: A routine consisting of 48 instructions that calls two other routines and returns a 64-bit value.
- `parse_format_string`: Parses a format string to extract values such as integers, floating-point numbers, or strings, and writes them into a destination buffer.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `prepare_signed_division`: Clears register d4, checks the sign of d0 and d1, negates them if necessary, and calls another routine.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `perform_32bit_division`: The routine calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `trampoline_call_and_return`: Calls one other routine and returns, passing its own stack arguments along.
- `conditional_process_and_call`: Calls another routine with a pointer from offset 10 of a structure in a5, then if a word from offset 8 of the structure is 10 and bit 7 of a byte at offset 12 of the structure pointed to by offset 10 is set, calls another routine with a longword of negative one and the pointer from offset 10.
- `branch_to_previous_address`: Branches to an earlier address.
- `compare_and_call_process`: Loads a pointer from offset 10 of a structure in a5 into a2, compares a longword at the address pointed to by a2 with a longword at offset 4 of the structure pointed to by a2, and if the first is not smaller, calls another routine with a word from offset 8 of the structure in a5.
- `increment_and_update_byte`: Increments a long value at the address in a2, copies a byte from offset 9 of a structure in a5 to the address pointed to by the original value in a2, and then loops back to an earlier instruction.
- `call_routine_for_each_record`: Walks an array of 22-byte records from a start address up to a fixed end address, calling another routine with each record's address.
- `load_pointer_from_a5`: Loads a pointer from offset 8 of a structure in a5 into a2, then sets d0 to that pointer value if it is non-zero, otherwise it sets d0 to 255.
- `process_structure_elements`: Checks a byte at offset 12 of a structure in a2; if it is non-zero, it performs several conditional operations including calling another routine with a pointer from a2, calling another routine with a value from offset 8 of a2, calling another routine twice with a value from offset 18 of a2, and finally clears several fields in the structure at a2 before returning.
- `test_structure_bit_and_store_pointer`: Tests bit 4 of a byte at offset 12 of a structure pointed to by a value at offset 8 of a frame in a5; if the bit is set, it stores a pointer to a constant location into a global variable and otherwise returns.
- `process_structure_byte`: Tests bit 2 of a byte at offset 12 of a structure in a2; if set, it calculates a difference between a longword at the start of the structure and a longword at offset 8, calls another routine with a signed byte from offset 13, and if the returned value matches the difference, sets bit 2 of offset 12, clears the first 8 bytes of the structure, and returns -1; otherwise it returns immediately.
- `clear_structure_fields`: Compares a 16-bit word at offset 12 of a structure in a5 to 65535; if it is not equal to that value, it branches to another routine, otherwise it clears bit 2 of a byte at offset 12 of a2, clears the 32-bit value pointed to by a2, clears the 32-bit value at offset 4 of the structure in a2, and branches to another routine.
- `process_structure_state_and_copy_byte`: Checks a pointer at offset 8 of a structure; if non-zero, it calls another routine to update the structure, otherwise it updates the structure's pointer and flags and copies a byte from offset 13 of a second structure to the memory location pointed to by the first structure.
- `update_structure_and_copy_byte`: Updates a structure by adding a 16-bit value from offset 16 to a 32-bit value at offset 8, sets bit 2 of a byte at offset 12, and copies a byte from offset 13 of a second structure to a destination pointed to by a pointer stored at offset 0 of the first structure.
- `search_structure_for_null`: Iterates through a structure by increments of 22 bytes, checking a byte at offset 12, and returns when a zero byte is found or a boundary is reached.
- `branch_to_offset_2b14`: Performs a short branch to a specific offset.
- `clear_structure_and_branch`: Clears the 32-bit values at offsets 0, 4, and 8 of a structure in a2, then moves a2 to d0 and performs a short branch.
- `update_pointer_and_flag`: Calls another routine with a word 1024 passed on the stack, stores the result in offset -4 of a5, and if the result is zero, sets a word at offset 16 of a2 to 1 and sets a long at offset 8 of a2 to the value of a2 plus 14.
- `set_structure_flags_and_call`: Writes 1024 to offset 16 of a2, sets bit 1 of offset 12 of a2, loads a 32-bit value from offset -4 of a5 into offset 8 of a2, extends a byte from offset 13 of a2 to a word, pushes it onto the stack, calls another routine, and if the extended value was not zero, sets bit 7 of offset 12 of a2 before branching.
- `setup_and_branch`: Performs a link instruction, saves registers a2 and a3 to the stack, loads a pointer from a global variable into a2, and performs a short branch.
- `traverse_linked_list`: The routine iterates through a linked list of pointers, calling a library function for each element until a null pointer is encountered.
- `insert_into_list`: Stores the address of the current routine in a global variable, calls another routine with a value from a structure, and updates a pointer if the result is non-zero.
- `store_and_increment_pointer`: Stores a pointer from a global variable into a structure at a2, copies a 32-bit value from offset 8 of a5 to offset 4 of the structure, updates the global variable with the address in a2, and branches to an offset after adding 8 to d0.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `setup_pointer_and_branch`: Saves registers a2 and a3 to the stack, subtracts a3 from itself, loads a pointer from offset -27810 of a4 into a2, and branches.
- `move_pointer_list_elements`: Compares a pointer in a2 with a value at offset 8 of a5, then enters a loop that moves a 32-bit value from the location pointed to by a2 into a3, updates a2 to point to the next element in the list, and repeats if the new a2 is not zero, otherwise it sets d0 to 255 and returns.
- `move_pointer_data`: If d0 is not zero, it moves the 32-bit value pointed to by a2 into the location pointed to by a3.
- `call_library_function_via_pointer`: Loads a pointer from a structure in a2, retrieves a value from offset 4 of that structure, pushes two values onto the stack, calls a routine at a specific offset, cleans up the stack, and performs a short branch.
- `validate_table_index`: Calculates an offset by multiplying a 16-bit value from offset 8 of a5 by 6, adds this to a base address in a4, and checks if the resulting pointer is null; if the value at offset 8 is within a specific range and the pointer is not null, it calls another routine, otherwise it writes 2 to a global variable and returns -1.
- `call_library_function_8`: Calculates an offset from a value in a structure, pushes a 32-bit value from a table onto the stack, calls another routine, and sets d0 based on the result.
- `clear_d0_and_branch`: Sets d0 to zero and performs a short branch.
- `execute_file_operation`: Calls a routine using parameters from the stack, then checks the result in d0 to either call another routine or store the value in a global variable at an offset from a6.
- `clear_d0_and_branch_2`: Sets register d0 to zero and performs a short branch to another routine.
- `calculate_scaled_address`: Calls another routine with a 16-bit value from offset 8 of a structure in a5, multiplies that value by 6, adds it to a pointer from a global variable, and if the resulting pointer is null and the original value is within a specific range, writes 2 to a global variable and sets d0 to -1.
- `update_conditional_value`: Checks if the 16-bit value at offset 4 of a2 bitwise-ANDed with 3 is non-zero; if it is zero, it writes 5 to a global variable at offset -25216 of a4, sets d0 to 255, and branches back to the start of the routine.
- `perform_conditional_write`: Pushes three parameters onto the stack and calls a routine, then if d0 is negative one, it stores d0 into a global variable at an offset from a4 and returns.
- `move_d5_to_d0_and_branch`: Moves the contents of d5 to d0 and performs a short branch.
- `check_result_and_return`: Calls another routine with the value 4096 passed on the stack, stores the result in offset -4 of a5, and if bit 12 of the result is set, checks a word at a specific offset from a4 and either calls another routine or returns the value from offset -4 of a5.
- `call_and_clear_d0`: Calls another routine, sets d0 to zero, and branches back to the start of the routine.
- `write_fixed_text_then_callback`: Obtains an output handle, writes a four-byte text fragment from a fixed address via DOS calls, and then executes a conditional function.
- `execute_linked_function`: Checks a global pointer and, if it is not zero, executes the subroutine located at that address, then calls another routine before returning.
- `traverse_and_execute_linked_structures`: Iterates through a series of nested pointers and tables to recursively call other routines.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `access_table_element`: Calculates an offset into a table based on a value from the stack, checks if the index is within bounds, and if the element is not null, calls another routine and clears the element.
- `jump_to_close`: Performs an indirect jump to another routine.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `indirectly_jump_to_function`: Performs an indirect jump to a function address retrieved from a pointer located within a structure at a negative offset from a value in a register.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `execute_indirect_function`: Loads a pointer from a table located at an offset from a base address and calls the function at that address.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_function`: Loads a library base from a global variable and calls the function at offset minus 414 from that base.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_function_2`: Loads a library base from a global variable and calls the function at offset minus 198 from that base.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_function_3`: Loads a library base from a global variable and calls the function at offset minus 132 from that base.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_function_4`: Loads a library base from a global variable and calls the function at offset minus 210 from that base.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_function_5`: Loads a library base from a global variable and calls the function at offset minus 372 from that base.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_function_6`: Loads a library base from a global variable and calls the function at offset minus 552 from that base.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_function_7`: Loads a library base from a global variable and calls the function at offset minus 378 from that base.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_564`: Library stub: loads a library base from a global variable and calls the library function at offset minus 564 of that base, returning its result to the caller.
- `call_library_vector_582`: Library stub: loads a library base from a global variable and calls the library function at offset minus 582 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_318_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_324`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_216_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.

### Mad TV (1992)(Rainbow Arts)(De)(Disk 1 of 2).adf:098.HES

- `branch_to_reset_audio_parameters`: Performs a short branch to the reset_audio_parameters routine.
- `jump_to_offset_sixty_four`: Performs a relative jump to an instruction at offset sixty-four.
- `update_audio_period_registers`: Loads a word from offset 88 of a structure in a6, writes it to the DMA control register, and then conditionally writes the value 9 to various audio period registers based on bits set in that word.
- `update_audio_and_dma_registers`: Writes values to multiple audio period registers and enables DMA via the DMA control register.
- `decrement_and_execute_callbacks`: Decrements a word at offset 22 of a structure in a6, copies several words from a structure in a4 to a new location, clears a byte at offset 9 of the structure in a6, and then calls another routine up to eight times based on the value of that byte.
- `tiny_addq`: Small routine of 1 instruction that adds 4 to a0.
- `compare_and_store_byte`: Compares a byte at offset 72 of a structure in a0 to 144 and 254; if the byte is 254, it stores that byte at the same offset, moves a byte from offset 73 of the structure into d0, and if a word at offset 8 of the structure is not zero, it returns.
- `decrement_structure_byte_2`: Loads a pointer from a relative address into a1 and stores a byte at offset 21 of that pointer, then tests a byte at offset 106 of a structure in a0; if it is zero, it returns, otherwise it decrements the byte at offset 106 of the structure and returns.
- `update_state_via_lookup_table`: Fetches a value from a table using an index derived from a 16-bit word at offset 104 of a structure, then updates the structure's state or performs a jump based on the retrieved byte.
- `dispatch_via_nibble_index`: Masks the lower four bits of a register, multiplies the result by four, and performs a jump to a destination calculated by adding this value to a relative offset.
- `increment_index_and_branch`: Increments a 16-bit word at offset 104 of a structure pointed to by a0 and branches to another location.
- `increment_word_at_offset`: Increments a 16-bit word at offset 104 of the structure pointed to by a0 and returns.
- `call_multiple_subroutines`: Calls another routine three times, each time with a different pointer loaded into a5 from a PC-relative table.
- `decrement_structure_counter`: Loads a pointer from offset 76 of a structure in a5 into a4, tests a word at offset 62 of the structure, and if it is not negative, decrements it at that offset.
- `update_structure_via_lookup_table_2`: Clears bytes at offsets 60 and 61 of a structure, then fetches a pointer from offset 84 to retrieve a byte from a table using an index at offset 16.
- `process_event_table_entry`: Fetches a pointer from a table using an index stored at offset 16 of a structure, retrieves a byte from the pointed-to location, clears that byte, and performs a jump to a destination calculated from the retrieved value.
- `increment_structure_counter`: Tests a byte at offset 90 of a structure in a5; if it is zero, it increments a 16-bit word at offset 16 of the structure and branches to another routine, otherwise it returns.
- `increment_event_counter`: Sets a byte at offset 90 of a structure in a5 to 255, increments a 16-bit counter at offset 16 of the same structure, and branches to another routine.
- `update_byte_at_offset_one`: Checks the byte at offset 1 of a structure in a5; if it is negative or non-zero, it branches to another routine, otherwise it sets the byte at offset 1 of the structure in a5 to 1 and branches to another routine.
- `update_animated_structure_values`: Updates multiple counters and scaled values within a structure by decrementing timers, performing fixed-point multiplication for scaling, and handling wrap-around logic for several byte and word fields.
- `calculate_scaled_value_2`: Loads a 16-bit word from offset 48 of a structure in a5, adds 256 to it, multiplies the result by d0, and compares the upper 16 bits of the product with d1.
- `decrement_structure_byte`: Checks if the byte at offset 28 of a5 is zero, then checks if the byte at offset 29 of a5 is zero; if both are non-zero, it decrements the byte at offset 29 of a5 and branches.
- `update_structure_bytes_2`: Copies a byte from offset 28 of a5 to offset 29 of a5, then compares a byte from offset 30 of a5 with a byte from offset 24 of a5; if the latter is greater, it branches, otherwise it subtracts a byte from offset 31 of a5 from offset 24 of a5 and performs further conditional branches based on the result.
- `update_structure_bytes`: Copies a byte from offset 30 of a5 to offset 24 of a5, clears the byte at offset 28 of a5, and performs a branch.
- `update_structure_and_calculate_offset`: Adds a byte from offset 31 of a5 to offset 24 of a5, performs several conditional updates to a structure in a6 involving byte shifts and additions, and finally calculates a value by multiplying a byte from offset 24 of a5 with a byte from offset 30 of a6, storing the result at offset 8 of a structure in a4.
- `update_structure_via_lookup_table`: Selects a structure from a table using a masked value from a register, then either updates specific fields at offsets 60 and 62 or performs a series of conditional field updates based on the value of a byte at offset 28 of that structure.
- `update_structure_fields_2`: If a byte at offset 60 of a structure in a5 is zero, it checks if d0 is negative and if it equals 247; if so, it stores a byte from offset 19 of a structure in a6 into offset 31 of the structure in a5, calculates a value by shifting a byte from offset 20 of a6 and adding 1, stores that value in offsets 28 and 29 of the structure in a5, and stores a byte from offset 21 of a6 into offset 30 of the structure in a5.
- `update_structure_fields`: If a byte in d0 is not 245, it branches; otherwise, it extracts a byte from offset 19 of a structure in a6, masks it, and stores the result in offset 38 and offset 39 of a structure in a5, then stores a byte from offset 21 of a6 into offset 33 of a5, clears a word at offset 36 of a5, and branches.
- `clear_byte_on_match`: Compares a byte in d0 with 245; if they are not equal, it branches, otherwise it clears a byte at offset 54 of a5 and branches.
- `initialize_structure_data`: Updates various fields in a structure pointed to by a5 using data from a structure in a6, including performing arithmetic on a pointer and clearing several fields.
- `initialize_table_index`: Copies a byte from offset 19 of a6 to offset 34 of a5, sets offset 35 of a5 to 1, and if a 16-bit word at offset 48 of a5 is zero, it copies a word from offset 40 to offset 50 of a5, clears offset 48, and then uses a masked byte from offset 18 of a6 to index into a table and store a word at offset 40 of a5.
- `update_table_entry`: Checks a byte at offset 60 within a table entry; if it is zero, it updates a global variable with a value from offset 22 of that entry and clears the entry's first byte.
- `reset_audio_parameters`: Clears a value at a pointer and sets a 32-bit value at a specific offset to zero while interacting with DMA and audio volume registers.

### Mad TV (1992)(Rainbow Arts)(De)(Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
