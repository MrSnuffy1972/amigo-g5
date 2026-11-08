# Game 532 (Mad Tv): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00532). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802304 top-level bytes in 222 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 233 of 233 named (0 author symbols), 233 of 233 described (51 by rule, 173 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 212 of 213 (99.53%) |
| Recovered without load address (counted in proof 1, flagged) | 25944 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4832 of 4832 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 847118 |
| code | 16000 |
| data | 576125 |
| padding | 52789 |
| unused | 310272 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Mad TV (1992)(Rainbow Arts)(Disk 1 of 2).adf:mtv | 7 | 233 | 7 |
| Mad TV (1992)(Rainbow Arts)(Disk 1 of 2).adf:add21 | 2 | 26 | 1 |
| Mad TV (1992)(Rainbow Arts)(Disk 1 of 2).adf:rai | 2 | 83 | 3 |
| Mad TV (1992)(Rainbow Arts)(Disk 1 of 2).adf:v | 3 | 3636 | 155 |
| Mad TV (1992)(Rainbow Arts)(Disk 1 of 2).adf:098.HES | 1 | 422 | 33 |
| Mad TV (1992)(Rainbow Arts)(Disk 2 of 2).adf:098.HES | 1 | 422 | 33 |
| Mad TV (1992)(Rainbow Arts)(Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 0 | 0 |
| Mad TV (1992)(Rainbow Arts)(Disk 1 of 2).adf:247.HES (raw code file, load address unknown) | 1 | 0 | 0 |
| Mad TV (1992)(Rainbow Arts)(Disk 1 of 2).adf:046.hes (raw code file, load address unknown) | 1 | 0 | 0 |
| Mad TV (1992)(Rainbow Arts)(Disk 1 of 2).adf:044.hes (raw code file, load address unknown) | 1 | 0 | 0 |
| Mad TV (1992)(Rainbow Arts)(Disk 2 of 2).adf:247.HES (raw code file, load address unknown) | 1 | 0 | 0 |
| Mad TV (1992)(Rainbow Arts)(Disk 2 of 2).adf:088.HES (raw code file, load address unknown) | 1 | 0 | 0 |
| Mad TV (1992)(Rainbow Arts)(Disk 2 of 2).adf:045.hes (raw code file, load address unknown) | 1 | 0 | 0 |
| Mad TV (1992)(Rainbow Arts)(Disk 2 of 2).adf:250.HES (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- mad tv (1992)(rainbow arts)(disk 2 of 2)/Mad TV (1992)(Rainbow Arts)(Disk 2 of 2).adf:172.HES: high-entropy data file (7.83 bits/byte), classified as packed data
- mad tv (1992)(rainbow arts)(disk 2 of 2)/Mad TV (1992)(Rainbow Arts)(Disk 2 of 2).adf:074.HES: high-entropy data file (7.64 bits/byte), classified as packed data
- mad tv (1992)(rainbow arts)(disk 2 of 2)/Mad TV (1992)(Rainbow Arts)(Disk 2 of 2).adf:337.HES: high-entropy data file (7.91 bits/byte), classified as packed data
- mad tv (1992)(rainbow arts)(disk 2 of 2)/Mad TV (1992)(Rainbow Arts)(Disk 2 of 2).adf:101.HES: high-entropy data file (7.71 bits/byte), classified as packed data
- mad tv (1992)(rainbow arts)(disk 2 of 2)/Mad TV (1992)(Rainbow Arts)(Disk 2 of 2).adf:187.HES: high-entropy data file (7.62 bits/byte), classified as packed data
- mad tv (1992)(rainbow arts)(disk 2 of 2)/Mad TV (1992)(Rainbow Arts)(Disk 2 of 2).adf:123.HES: high-entropy data file (7.61 bits/byte), classified as packed data
- mad tv (1992)(rainbow arts)(disk 2 of 2)/Mad TV (1992)(Rainbow Arts)(Disk 2 of 2).adf:167.HES: high-entropy data file (7.75 bits/byte), classified as packed data
- mad tv (1992)(rainbow arts)(disk 2 of 2)/Mad TV (1992)(Rainbow Arts)(Disk 2 of 2).adf:059.HES: high-entropy data file (7.67 bits/byte), classified as packed data
- reference crashes at an address (HLE illegal instruction an address at an address: through vector 0, as before (vector 4 is 0 again)): 109944 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- mad tv (1992)(rainbow arts)(disk 1 of 2)/Mad TV (1992)(Rainbow Arts)(Disk 1 of 2).adf:247.HES: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- mad tv (1992)(rainbow arts)(disk 1 of 2)/Mad TV (1992)(Rainbow Arts)(Disk 1 of 2).adf:046.hes: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- mad tv (1992)(rainbow arts)(disk 1 of 2)/Mad TV (1992)(Rainbow Arts)(Disk 1 of 2).adf:044.hes: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- mad tv (1992)(rainbow arts)(disk 2 of 2)/Mad TV (1992)(Rainbow Arts)(Disk 2 of 2).adf:247.HES: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- mad tv (1992)(rainbow arts)(disk 2 of 2)/Mad TV (1992)(Rainbow Arts)(Disk 2 of 2).adf:088.HES: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- mad tv (1992)(rainbow arts)(disk 2 of 2)/Mad TV (1992)(Rainbow Arts)(Disk 2 of 2).adf:045.hes: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- mad tv (1992)(rainbow arts)(disk 2 of 2)/Mad TV (1992)(Rainbow Arts)(Disk 2 of 2).adf:250.HES: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- mad tv (1992)(rainbow arts)(disk 1 of 2)/Mad TV (1992)(Rainbow Arts)(Disk 1 of 2).adf:mtv: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- mad tv (1992)(rainbow arts)(disk 1 of 2)/Mad TV (1992)(Rainbow Arts)(Disk 1 of 2).adf:v: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 7 never-loaded region(s), 25944 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Mad TV (1992)(Rainbow Arts)(Disk 1 of 2).adf:mtv

- `free_blocks_and_clear_field`: Runs a short loop that releases memory blocks through the system FreeMem entry, then zeroes a longword field in the caller's structure.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `expand_bitstream_data`: Calls a routine multiple times to load data, then enters a loop that calls a routine and potentially adds values to a table at a3 using offsets derived from d0.
- `load_address_and_set_register`: Loads an address into a1 and sets d0 to 2.
- `expand_and_copy_bitstream`: Expands a compressed bitstream by reading a control byte and performing variable-length bit shifts and additions to reconstruct data into a destination buffer.
- `push_registers_and_branch`: Clears a byte at the end of a structure pointed to by a6, pushes the values of d0, d1, and d2 onto the stack, and branches to another routine.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### Mad TV (1992)(Rainbow Arts)(Disk 1 of 2).adf:add21

- `initialize_libraries_and_cleanup`: The routine calls library functions via base pointers, writes a byte through a pointer chain, and then calls additional library functions before returning.

### Mad TV (1992)(Rainbow Arts)(Disk 1 of 2).adf:rai

- `decompress_bitstream_data`: Decompresses a bit-packed data stream into a destination buffer using a variable-length coding scheme.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Mad TV (1992)(Rainbow Arts)(Disk 1 of 2).adf:v

- `jump_to_entry`: Performs an indirect jump to an address.
- `dispatch_subroutine_calls`: Iterates through a structure to process data elements by calling various routines based on values at specific offsets, including clearing tables, updating entity states, and validating file structures.
- `read_file_data`: Calls a DOS function to read data into a buffer and updates a pointer and a status flag based on the number of bytes read.
- `read_file_buffer`: Reads data from a source into a destination buffer and updates a pointer and a status flag based on the number of bytes read.
- `process_resource_data`: Checks a 16-bit value at offset 12 of a structure, then calls another routine with a pointer from offset 8, or calls a routine with a constant value and a pointer from offset 8.
- `parse_resource_file`: Parses a file by reading chunks of data to identify specific four-byte magic markers, such as those representing 'MILB', 'DHMB', or 'MORM'. It manages a buffer and updates a global table with status flags, coordinate values, and dimensions based on the identified resource types.
- `load_image_data`: Attempts to read various image-related data from a file, updates a global table with status flags and coordinate values, and manages a buffer by shifting bytes based on signed values.
- `update_entity_status_and_physics`: Updates a status word in a structure based on several flags, then performs conditional arithmetic to update physics-related values such as velocity or position.
- `modify_structure_values`: Adds values from a source to a structure, performs several subroutine calls, and then subtracts those same values from the structure, finally calling another routine if a result is non-zero.
- `update_scaled_entity_data`: Multiplies a value from a global variable by 300 and uses the result as an index to store several scaled 16-bit values into a table.
- `format_string_to_global`: Passes a pointer to a global variable and a value to a routine that parses a format string.
- `update_physics_state`: Calculates new values for several state variables by applying multipliers and conditional logic based on existing flags.
- `calculate_scaled_value`: Performs arithmetic on values from a global table to calculate a scaled result, which is then stored in several locations.
- `process_list_elements`: Checks a value at a global offset, and if non-negative, iterates through a table to clear entries and calls another routine if an entry is non-zero.
- `copy_table_data`: Copies data from a source table to a destination buffer using a loop, or performs a single copy and updates a status value if a condition is met.
- `store_scaled_values`: If a value in a global variable is non-negative, it calculates an offset using a multiplier and stores three 16-bit words from a structure into a table at that offset; otherwise, it calls another routine.
- `call_routine_with_parameters`: Extends four 16-bit words from a structure into 32-bit registers, pushes them onto the stack, and calls another routine.
- `update_coordinate_table`: Calculates offsets based on a value and a multiplier to store three 16-bit words into a table, or calls another routine to update the table and then stores three 16-bit words at different calculated offsets.
- `copy_scaled_values`: If a value in a global variable is non-negative, it calculates an offset using multipliers and stores three 16-bit words from a structure into three different locations pointed to by values in a structure.
- `split_word_into_nibbles`: Calls another routine, then takes the resulting 16-bit word and stores its four nibbles into four different locations pointed to by offsets 24, 20, and 16 of a structure.
- `handle_cleanup_error`: Clears a data table, executes several linked functions or library calls based on non-zero pointers, and performs an indirect call.
- `set_bit_in_structure`: Calls another routine and then sets the lowest bit of a byte located at offset 25 of a structure.
- `clear_bit_in_structure`: Calls another routine and then clears the lowest bit of a byte located at offset 25 of a structure.
- `clear_buffer_pointer`: If a condition is met and a pointer in a global variable is non-zero, it calls another routine and then clears that pointer.
- `update_entity_state_and_physics`: Updates an entity's status flags and position based on conditional checks and external routine calls.
- `check_list_for_value`: Traverses a linked list and returns a boolean value if a specific pattern is found within the list elements.
- `get_offset_from_index`: If a value from a register is less than a value at an offset of a structure in a6, it returns the sum of a value at an offset of that structure and the register value multiplied by eight; otherwise it returns zero.
- `execute_nested_calls`: Checks if a pointer at offset 8 of a structure is non-zero, then performs an indirect jump to a function, an indirect call to another function using a nested pointer, and a final indirect jump to a third function, returning the result of the second call.
- `check_status_conditions`: Returns one if four specific conditions involving values at offsets of a4 are met, otherwise it returns zero.
- `manage_memory_buffer`: Calls a routine with two arguments, then if the result is non-zero, performs several operations including a memory copy, updates a structure at offset 12, and enters a loop that copies data from one location to another while setting specific values in a table based on global variables.
- `process_tree_nodes`: If a value in a structure is less than 35, it performs a recursive traversal of a tree-like structure, updating values at specific offsets and calling several other routines based on conditional flags.
- `parse_file_structure`: Checks a pointer at offset 8 of a structure and, if non-zero, enters a loop that reads data from a file, compares it against a specific four-byte constant, and performs conditional jumps or calls based on the data retrieved.
- `parse_and_tokenize_string`: Reads a string from a file, calculates its length, and splits it into a table of pointers by replacing non-printable characters with null terminators.
- `apply_xor_transformation`: Iterates through a memory buffer, performing a bitwise XOR operation on each byte using a value that increments with every iteration.
- `load_constant_address`: Loads the constant value 32766 into register a4 and returns.
- `initialize_structured_data`: Calculates a value by multiplying a 16-bit word from a global variable by 6, calls another routine, and stores the result in a global variable.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `tokenize_and_reallocate_string`: Parses a string by identifying whitespace and quoted substrings, copying the resulting tokens into a new buffer.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `safe_memory_copy`: Copies a specified number of bytes from a source to a destination, handling overlapping memory regions by copying backwards if the destination address is higher than the source.
- `set_d0_and_branch`: Sets register d0 to 32767 and branches to another routine.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `compare_two_byte_strings`: The routine compares two byte strings byte by byte.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `initialize_window_structure`: Calls a routine to store a value in a global variable, then if non-zero, accesses a structure to perform several library calls including an icon-related call, updates structure offsets with calculated values, and finally calls another routine to clear a global variable.
- `move_d0_to_d5`: Copies the contents of register d0 to register d5.
- `call_parse_format_string`: Pushes a frame offset and a value from offset 8 of the frame onto the stack, then calls another routine via a PC-relative address, cleans up the stack, restores the frame and returns.
- `invoke_two_subroutines`: A routine consisting of 48 instructions that calls two other routines and returns a 64-bit value.
- `parse_format_string`: Parses a format string to extract and process various data types including integers, floating-point numbers, and strings.
- `perform_signed_division`: Reads two signed 32-bit inputs, determines the sign of the result, calls another routine to perform the division, and optionally negates the result before returning a 32-bit value combined with the second input.
- `prepare_signed_division`: Clears a 32-bit register, checks the sign of two 32-bit values, negates them if they are negative, and calls another routine to perform division.
- `call_and_return_concatenated`: Calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine.
- `perform_32bit_division`: The routine calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `trampoline_call_and_return`: Calls another routine with a pointer from offset 10 of a structure in a5, compares a word in d4 to 10, and if it matches, checks bit 7 of a status byte, then returns.
- `call_subroutines_conditionally`: Calls another routine with a pointer from offset 10 of a structure in a5, compares a word in d4 to 10, and if it matches, checks bit 7 of a byte at offset 12 of the pointer to decide whether to call another routine with a 16-bit value of 65535.
- `branch_to_offset`: Performs a relative branch to another location.
- `call_routine_with_masked_word`: Loads a pointer from offset 10 of a structure in a5, compares a value at the pointer to a value at offset 4 of the pointer, and if the pointer value is not smaller, calls another routine with a 16-bit word from offset 8 of the structure in a5 masked with 255.
- `update_byte_via_pointer`: Loads a pointer from a table, increments the pointer in the table, and then stores a byte from a structure into the memory location pointed to by the original pointer, then enters a loop.
- `walk_record_array`: Walks an array of 22-byte records from a start address up to a fixed end address, calling another routine with each record's address.
- `load_pointer_from_offset`: Saves registers to the stack, loads a pointer from offset 8 of a structure in a5 into a2, and if that pointer is not zero, sets d0 to the value of that pointer; otherwise, it sets d0 to 255 and returns.
- `process_and_clear_resource_structure`: Checks a status byte at offset 12 of a structure in a2; if non-zero, it conditionally calls three different routines based on bits 1, 2, and 5 of that byte, then clears the first 12 bytes of the structure and returns a combined result in d0.
- `test_record_status_bit`: Tests bit 4 of a byte at offset 12 of a structure pointed to by a value loaded from an offset of a5, then either stores a constant value into a global variable or returns.
- `update_linked_list_node`: Checks a status bit at offset 12, calculates a difference between a pointer and a value, and calls another routine to verify the result.
- `clear_bits_and_memory`: Compares a 16-bit word at offset 12 of a structure in a5 to 65535; if they are not equal, it branches to another routine, otherwise it clears bit 2 of a byte at offset 12 of a structure in a2, clears a 32-bit longword at the address in a2, clears a 32-bit longword at offset 4 of that address, and branches to another routine.
- `process_table_entry`: Checks a pointer at offset 8 of a structure, calls another routine if it is zero, and then either updates a byte at offset 13 of a structure and calls another routine or updates several fields in a table based on a comparison at offset 16.
- `update_table_and_byte`: Updates a pointer in a table by adding a value from an offset, updates a field in that table with the sum, sets bit 2 of a field at offset 12, increments the table pointer, and stores a byte from a structure into the new pointer location before branching.
- `search_and_clear_list`: Iterates through a list of structures, checking a byte at offset 12 of each, and if it is non-zero, it continues to the next structure; if it is zero, it clears the structure and returns the pointer.
- `branch_to_another_routine`: Performs a short branch to another routine.
- `clear_a2_offsets`: Clears the a2 register and the 12-byte area starting at offset 4 of the structure pointed to by a2, then moves a2 into d0 before branching.
- `loop_and_update_pointer`: Creates a stack frame, calls another routine, and if the result in d0 is not zero, it branches to the start of this routine.
- `update_structure_and_set_flags`: Writes a word to offset 16 of a structure in a2, sets bit 1 of a byte at offset 12 of the same structure, copies a long value from a pointer at offset -4 of a5 to offset 8 of the structure, and calls another routine.
- `update_pointer_list`: Clears a5, saves a2 and a3 to the stack, loads a pointer from a table using a4, and branches.
- `traverse_linked_list`: The routine iterates through a linked list of pointers, calling a library function for each element until a null pointer is encountered.
- `update_pointer_list_2`: Stores the address of this routine into a global variable, calls another routine with a modified parameter, and if the result is non-zero, updates a pointer in a structure.
- `update_structure_data`: Copies a longword from a table into the structure at a2, moves a value from offset 8 of a5 into offset 4 of a2, updates the table with the new a2 pointer, and moves a2 into d0 with an added offset of 8 before branching.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `setup_pointer_and_branch`: Creates a stack frame, saves registers a2 and a3, subtracts a3 from a3, loads a pointer from an offset of a4 into a2, and branches to another location.
- `process_pointer_list`: Compares a pointer in a structure with a register, and if they are not equal, enters a loop that moves a pointer from a table to a register and updates the table pointer until the register is zero, otherwise it sets the register to 255 and returns.
- `move_data_conditionally`: If a register is non-zero, it moves the contents of a pointer in a structure to a destination pointed to by another register.
- `call_library_function_8`: Loads a pointer from a table using a4, moves a value from offset 4 of a2 into d0 and adds 8, pushes d0 and a2 onto the stack, calls a routine, cleans up the stack, and sets d0 to zero before branching.
- `check_table_index`: Calculates an offset by multiplying a word from a structure by 6, adds it to a base address, and checks if the resulting pointer is non-zero; if the word is within a specific range and the pointer is zero, it writes 2 to a global variable and returns -1, otherwise it calls another routine.
- `check_interactive_status`: Calculates an offset by multiplying a 16-bit value by 6, pushes a value from a table at that offset onto the stack, calls another routine, and returns either zero or one based on the result.
- `clear_d0_and_branch`: Sets d0 to zero and branches.
- `execute_file_operation`: Calls a routine using a pointer from a structure, then checks the result in d0 to either call another routine or store the value in a global variable at a negative offset from a4.
- `clear_d0_and_branch_2`: Sets register d0 to zero and performs a short branch to another routine.
- `calculate_table_index`: Saves registers to the stack, loads a word from offset 8 of a structure in a5 into d4, calls another routine, and then calculates a pointer by multiplying d4 by 6 and adding it to a value from a global variable; if d4 is within a specific range and the value at the calculated pointer is zero, it writes 2 to a global variable and sets d0 to -1, otherwise it returns.
- `test_and_set_value`: Performs a bitwise AND on a word from a structure at offset 4 of a2, and if the result is non-zero, it writes 5 to a global variable and returns via a branch.
- `process_data_with_error_check`: Loads a word from a5 offset 14, pushes several values onto the stack, calls a routine, and if the result is not negative one, calls another routine before storing d0 in a global variable and returning -1.
- `move_d5_to_d0`: Moves the contents of d5 into d0 and branches.
- `call_library_and_check_result`: Creates a stack frame, calls a library routine with a value from a constant, and if bit 12 of the result is set, checks a word at a negative offset from a4 and potentially calls another routine before returning the result.
- `call_and_clear_d0`: Calls another routine, clears d0, and branches to another location.
- `write_fixed_text_then_callback`: Obtains an output handle, writes a four-byte text fragment from a fixed address via DOS calls, and then executes a conditional function.
- `execute_linked_function`: Checks a global pointer and, if it is not zero, executes the subroutine located at that address, then calls another routine.
- `traverse_and_execute_linked_structures`: Iterates through a series of nested pointers and tables to recursively call other routines.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `validate_and_close_handle`: Checks if an index is within a valid range in a table, then either clears a value at that index or calls another routine to close a handle based on a bit test.
- `jump_to_close`: Performs an indirect jump to another routine.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `execute_indirect_jump`: Performs an indirect jump to a location retrieved by dereferencing a pointer stored at an offset within a structure pointed to by a value in a register.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `indirectly_call_file_opener`: Performs an indirect jump to a function address retrieved from a nested pointer structure located via an offset from a value in a register.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_function`: Calls a library function using a pointer retrieved from a global variable at an offset of minus 414 from the library base.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_function_2`: Calls a library function using a pointer retrieved from a global variable at an offset of minus 198 from the library base.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_function_3`: Calls a library function using a pointer retrieved from a global variable at an offset of minus 132 from the library base.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_function_4`: Calls a library function using a pointer retrieved from a global variable at an offset of minus 210 from the library base.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_function_5`: Calls a library function using a pointer retrieved from a global variable at an offset of minus 372 from the library base.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_function_6`: Calls a library function using a pointer retrieved from a global variable at an offset of minus 552 from the library base.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_function_7`: Calls a library function using a pointer retrieved from a global variable at an offset of minus 378 from the library base.
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

### Mad TV (1992)(Rainbow Arts)(Disk 1 of 2).adf:098.HES

- `branch_to_entry`: Performs a short branch to a specific address.
- `jump_offset_64`: Performs a short branch to an offset of 64.
- `update_audio_and_dma_registers_2`: Calls two subroutines, then writes values from a table into four audio period registers and writes a word from offset 54 of a structure in a6.
- `update_audio_and_dma_registers`: Calls two subroutines, then writes values from a table into four audio period registers, writes a word from offset 54 of a structure in a6 to a DMA control register, and clears that word.
- `process_data_and_call_subroutines`: Decrements a 16-bit value at offset 22 of a structure in a6, copies several 16-bit words from a structure in a4 to a table in a5, and then calls several subroutines conditionally based on a byte at offset 9 of the structure in a6.
- `tiny_addq`: Small routine of 1 instruction that adds 4 to a0.
- `check_byte_range`: Compares a byte at offset 72 of a structure in a0 to 144 and 254; if the byte is between 144 and 254 inclusive, it stores the byte back to offset 72, moves the byte from offset 73 into d0, and if a word at offset 8 of the structure is not zero, it returns.
- `decrement_byte_at_offset`: Loads an address into a1, stores a byte at offset 21 of that address, tests a byte at offset 106 of a structure in a0; if it is zero, it branches to another routine, otherwise it decrements the byte at offset 106 of the structure and returns.
- `update_object_state_metadata_2`: Fetches a value from a jump table using an index derived from a 16-bit word at offset 104 of a structure in a0, then updates a byte at offset 18 of a structure in a6 based on a range check and an offset value from offset 73 of the structure in a0.
- `dispatch_jump_table_entry`: Masks the lower nibble of a register, multiplies it by four, and performs a jump to an address in a jump table using the resulting value as an offset.
- `increment_index_and_branch`: Increments a 16-bit word at offset 104 of a structure pointed to by a0 and branches to another location.
- `increment_word_at_offset`: Increments a 16-bit word at offset 104 of the structure pointed to by a0.
- `execute_sequential_subroutine_calls`: Loads four different addresses into a5 and calls another routine three times sequentially without returning.
- `decrement_counter_and_load_pointer`: Loads a pointer from offset 4c of a structure in a5 into a4, checks if a 16-bit value at offset 3e of the structure is negative, and if not, decrements that value and branches.
- `process_command_data_sequence`: Clears bytes at offsets 60 and 61 of a structure in a5, then fetches a pointer from offset 84 of the same structure.
- `dispatch_command_data_via_table`: Fetches a pointer from a table using an index from a structure, retrieves a byte from that pointer, clears the original byte, and performs a jump to a location determined by the retrieved value.
- `increment_status_counter_conditionally`: Checks if a byte at offset 90 of the structure in a5 is zero; if not zero, it increments a 16-bit counter at offset 16 of the structure and branches to another routine, otherwise it branches to another routine.
- `increment_command_index_and_update_status`: Sets a byte at offset 90 of a structure to 255, increments a 16-bit counter at offset 16 of the same structure, and branches to another routine.
- `increment_status_byte`: Checks the byte at offset 1 of a structure in a5; if the byte is negative or zero, it branches to another routine, otherwise it writes 1 to offset 1 of the structure and branches to another routine.
- `update_structure_counters_and_scaling`: Decrements several counters within a structure and, if a counter reaches zero, resets it using a value from another offset and performs arithmetic on related fields.
- `calculate_scaled_value_2`: Adds 256 to a 16-bit value from a structure offset in a5, multiplies it by d0, shifts the result right by 8 bits, and compares it to d1 before branching.
- `decrement_structure_byte`: Checks a byte at offset 28 of a structure in a5; if it is not zero, it checks a byte at offset 29 and, if that is also not zero, decrements the byte at offset 29 and branches to another location.
- `update_structure_bytes_2`: Copies a byte from offset 28 to offset 29 of a structure, compares a byte at offset 24 with a byte at offset 30, and if the byte at offset 24 is not greater than the byte at offset 30, subtracts a byte from offset 31 from offset 24.
- `update_structure_bytes`: Copies a byte from a5 offset 30 to a5 offset 24, clears a byte at a5 offset 28, and branches to another location.
- `update_structure_and_calculate`: Adds a byte from a5 offset 31 to a5 offset 24, performs several conditional updates to a6 based on its contents, and finally calculates a value by multiplying d1 by d0 and storing the result in a4 offset 8.
- `update_object_state_metadata`: Selects a structure from a table using a nibble from a byte, then checks if a byte at offset 60 of that structure is zero.
- `update_object_metadata`: Checks if a byte at offset 60 of a structure is non-zero; if zero, it performs conditional data transfers between a source structure in a6 and a destination structure in a5 based on the value in d0.
- `populate_structure_from_data`: Compares d0 to 245; if not equal, it reads a byte from offset 19 of a6, masks it, and writes the result to offset 38 of a5, then performs a series of shifts and writes to offsets 38, 39, 32, and 33 of a5 before branching to another location.
- `clear_byte_on_match`: Compares a byte in d0 with 245, and if they are equal, clears a byte at offset 54 of a structure.
- `update_table_data`: Depending on d0, either populates a structure at a5 with values from a6 and calculates a pointer to a table, or populates a structure with values from a6 and a global table based on a masked value.
- `update_table_pointer`: Copies a byte from a6 offset 19 to a5 offset 34, sets a5 offset 35 to 1, and if a 16-bit value at a5 offset 48 is zero, copies a 16-bit value from a5 offset 40 to a5 offset 50 and uses a masked byte from a6 offset 18 to fetch a 16-bit value from a table into a5 offset 40.
- `update_table_entry`: Uses a value in d0 to select an entry from a table, checks a byte at offset 60 of that entry, and if it is zero, updates a global variable with a 16-bit value from offset 22 of that entry and clears the first byte of the entry.
- `reset_audio_parameters`: Calls another routine four times with different pointers, clears four 16-bit audio volume registers, sets a value in a DMA register, and clears a byte at offset 21 of a structure.

### Mad TV (1992)(Rainbow Arts)(Disk 2 of 2).adf:098.HES

- `branch_to_reset_audio_parameters`: Performs a short branch to the reset_audio_parameters routine.
- `branch_to_offset_100`: Returns immediately by performing a short branch to an instruction 100 bytes ahead.
- `update_audio_period_registers`: Reads a 16-bit value from offset 56 of a structure in a6 and uses its bits to selectively write the value 9 into the four audio period registers.
- `update_audio_period_and_dma_registers`: Calls two other routines, then writes four 16-bit values from a table into the audio period registers and a fifth value into the DMA control register.
- `process_data_list`: Decrements a 16-bit value at offset 22 of a structure in a6, copies eight 16-bit words from a structure in a4 to a new location, and then calls several subroutines sequentially if a byte at offset 9 of a structure in a6 remains zero.
- `add_long_to_pointer`: Adds 4 to the long value in a0.
- `validate_and_store_byte`: Checks if a byte at offset 72 of the structure in a0 is between 144 and 254 inclusive; if it matches 254, it stores that byte back to the same offset, moves the byte from offset 73 into d0, and if a word at offset 8 of the structure is not zero, it branches to another routine; otherwise it returns.
- `decrement_and_store_byte`: Stores a byte from a memory location into a byte at offset 21 of a pointer in a1, then decrements a byte at offset 106 of the structure in a0 if it is not zero; otherwise it returns.
- `update_object_state_via_jump_table`: Fetches a value from a table using an index derived from a 16-bit word at offset 104 of a structure in a0, then performs a jump to a routine selected from a jump table based on a byte value.
- `dispatch_jump_table_via_nibble`: Masks the lower nibble of a register, scales it by four, and performs a jump to a location calculated from the program counter and the resulting offset.
- `increment_metadata_index`: Increments a 16-bit word at offset 104 of the structure pointed to by a0 and branches to another location.
- `increment_word_at_offset_2`: Increments a 16-bit word at offset 104 of the structure pointed to by a0 and returns.
- `load_and_call_subroutine_sequence`: Loads four different addresses into a5 using PC-relative addressing and calls another routine after each of the first three loads.
- `decrement_and_branch_on_counter`: Loads a pointer from offset 76 of the structure in a5 into a4, checks a 16-bit value at offset 62 of the structure, and if that value is negative, branches to another routine; otherwise, it decrements the value at offset 62 by one and branches to another routine.
- `process_command_data_and_update_state`: Clears bytes at offsets 60 and 61 of a structure in a5, then fetches a pointer from offset 84 of the same structure.
- `dispatch_command_event_data`: Fetches a byte from a table using a 16-bit index from a structure, clears that byte in a local buffer, and performs a jump table dispatch based on the retrieved value.
- `increment_counter_on_non_zero_byte`: Tests a byte at offset 90 of a structure in a5; if it is zero, it branches to another routine, otherwise it increments a 16-bit word at offset 16 of the structure and branches to another routine.
- `update_status_and_increment_index`: Sets a byte at offset 90 of a structure to 255, increments a 16-bit word at offset 16 of the same structure, and branches to another routine.
- `update_status_byte`: Checks a byte at offset 1 of a structure in a5; if the byte is negative it branches to another routine, if it is non-zero it branches to another routine, otherwise it sets the byte at offset 1 to 1 and branches to another routine.
- `update_structure_counters_and_scaling_2`: Decrements several counters within a structure, performing wrap-around logic by loading new values from specific offsets and toggling bytes when counters reach zero.
- `calculate_scaled_value_4`: Loads a word from offset 48 of a structure in a5, adds 256 to it, multiplies this value by d0, shifts the result right by 8 bits, and compares the result to d1 before branching to another routine.
- `decrement_conditional_byte`: Checks if the byte at offset 28 of the structure in a5 is zero, and if not, checks if the byte at offset 29 is zero; if the latter is non-zero, it decrements the byte at offset 29 by one and branches to another routine, otherwise it branches to another routine.
- `update_structure_byte_threshold`: Copies a byte from offset 28 to offset 29 of a structure, compares a byte at offset 30 with a byte at offset 24, and if the value at offset 30 is not greater than the value at offset 24, subtracts a byte from offset 31 from the byte at offset 24 before branching to another routine.
- `copy_and_clear_byte`: Copies a byte from offset 30 of a structure to offset 24 of the same structure, clears the byte at offset 28, and branches to another routine.
- `calculate_scaled_value_3`: Adds a byte from offset 31 of a structure to offset 24, performs several conditional updates to a structure at a6 including decrementing and shifting bytes, and finally calculates a 16-bit value using a multiplication and bit shift, storing the result in a global variable at offset 8 of a4.
- `update_object_state_from_metadata`: Selects a structure from a table using a nibble from a byte in a6, then either updates specific fields at offsets 60 and 62 if a control byte matches a specific value, or performs a series of conditional data transfers to the structure based on various byte values.
- `update_object_attribute_structure`: Checks if a byte at offset 60 of a structure is zero, then branches to another routine if it is not.
- `update_table_entry_on_match`: If the byte in d0 equals 246, it extracts a bit-masked value from a byte at offset 19 of a structure in a6, stores the masked byte and its shifted version into offsets 38 and 39 of a structure in a5, copies a byte from offset 19 of the a6 structure to offset 32 of the a5 structure, sets offset 33 of the a5 structure to 1, clears a 16-bit word at offset 36 of the a5 structure, and otherwise returns.
- `clear_byte_on_match_2`: Compares a byte in a data register to 245; if they match, it clears a byte at offset 54 of a structure pointed to by a5, otherwise it branches to another routine.
- `populate_object_structure`: If a byte in d0 is less than 191, it populates a structure in a5 with values from a6, including a calculated pointer at offset 12; otherwise, it updates the structure in a5 with different values from a6, including a pointer derived from a table at offset 46 of a6, and sets a byte at offset 54 to 1.
- `populate_table_entry`: Copies bytes from a6 to a5, clears a word at offset 48 of a5, and uses a masked byte from a6 to fetch a word from a table into offset 40 of a5.
- `update_table_entry_2`: Uses a value derived from a masked register to index into a table, then checks a byte at offset 60 of the retrieved entry; if that byte is zero, it updates a global variable with a value from offset 22 of the entry and clears the entry's first byte.
- `reset_audio_parameters_2`: Clears a byte at a6 offset 0 and a 32-bit value at a6 offset 60, then returns.

### Mad TV (1992)(Rainbow Arts)(Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
