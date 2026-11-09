# Game 429 (Might and Magic Iii - Isles of Terror): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00429). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (5406720 top-level bytes in 56 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 341 of 341 named (0 author symbols), 341 of 341 described (10 by rule, 329 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 2701460 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 14958 of 14990 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1765936 |
| code | 47674 |
| data | 3040590 |
| padding | 12872 |
| unused | 539648 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| copy | 2 | 1685 | 5 |
| makedir | 1 | 112 | 1 |
| Might and Magic III - Isles of Terror (1992)(U. S. Gold)(Disk 1 of 6)[HD-FD install required].adf:Install-Harddrive | 3 | 3370 | 62 |
| Might and Magic III - Isles of Terror (1992)(U. S. Gold)(Disk 1 of 6)[HD-FD install required].adf:Install-Floppy | 3 | 2152 | 52 |
| 8 | 1 | 51 | 2 |
| 9 | 1 | 2 | 1 |
| Might and Magic III - Isles of Terror (1992)(U. S. Gold)(Disk 5 of 6)[HD-FD install required].adf:MM3-Intro | 3 | 7618 | 218 |
| Might and Magic III - Isles of Terror (1992)(U. S. Gold)(Disk 1 of 6)[HD-FD install required].adf:Might&MagicIII (raw code file, load address unknown) | 1 | 0 | 0 |
| Might and Magic III - Isles of Terror (1992)(U. S. Gold)(Disk 1 of 6)[HD-FD install required].adf:global.cc (raw code file, load address unknown) | 1 | 0 | 0 |
| Might and Magic III - Isles of Terror (1992)(U. S. Gold)(Disk 2 of 6)[HD-FD install required].adf:mm3-01.cc (raw code file, load address unknown) | 1 | 0 | 0 |
| Might and Magic III - Isles of Terror (1992)(U. S. Gold)(Disk 3 of 6)[HD-FD install required].adf:mm3-02.cc (raw code file, load address unknown) | 1 | 0 | 0 |
| Might and Magic III - Isles of Terror (1992)(U. S. Gold)(Disk 4 of 6)[HD-FD install required].adf:mm3-03.cc (raw code file, load address unknown) | 1 | 0 | 0 |
| Might and Magic III - Isles of Terror (1992)(U. S. Gold)(Disk 5 of 6)[HD-FD install required].adf:mm3-04.cc (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- might and magic iii - isles of terror (1992)(u. s. gold)(disk 1 of 6)[hd-fd install required]/Might and Magic III - Isles of Terror (1992)(U. S. Gold)(Disk 1 of 6)[HD-FD install required].adf:Might&MagicIII: high-entropy data file (7.62 bits/byte), classified as packed data
- might and magic iii - isles of terror (1992)(u. s. gold)(disk 1 of 6)[hd-fd install required]/Might and Magic III - Isles of Terror (1992)(U. S. Gold)(Disk 1 of 6)[HD-FD install required].adf:Might&MagicIII: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- might and magic iii - isles of terror (1992)(u. s. gold)(disk 1 of 6)[hd-fd install required]/Might and Magic III - Isles of Terror (1992)(U. S. Gold)(Disk 1 of 6)[HD-FD install required].adf:global.cc: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- might and magic iii - isles of terror (1992)(u. s. gold)(disk 2 of 6)[hd-fd install required]/Might and Magic III - Isles of Terror (1992)(U. S. Gold)(Disk 2 of 6)[HD-FD install required].adf:mm3-01.cc: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- might and magic iii - isles of terror (1992)(u. s. gold)(disk 3 of 6)[hd-fd install required]/Might and Magic III - Isles of Terror (1992)(U. S. Gold)(Disk 3 of 6)[HD-FD install required].adf:mm3-02.cc: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- might and magic iii - isles of terror (1992)(u. s. gold)(disk 4 of 6)[hd-fd install required]/Might and Magic III - Isles of Terror (1992)(U. S. Gold)(Disk 4 of 6)[HD-FD install required].adf:mm3-03.cc: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- might and magic iii - isles of terror (1992)(u. s. gold)(disk 5 of 6)[hd-fd install required]/Might and Magic III - Isles of Terror (1992)(U. S. Gold)(Disk 5 of 6)[HD-FD install required].adf:mm3-04.cc: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- might and magic iii - isles of terror (1992)(u. s. gold)(disk 1 of 6)[hd-fd install required]/Might and Magic III - Isles of Terror (1992)(U. S. Gold)(Disk 1 of 6)[HD-FD install required].adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- might and magic iii - isles of terror (1992)(u. s. gold)(disk 3 of 6)[hd-fd install required]/Might and Magic III - Isles of Terror (1992)(U. S. Gold)(Disk 3 of 6)[HD-FD install required].adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- might and magic iii - isles of terror (1992)(u. s. gold)(disk 4 of 6)[hd-fd install required]/Might and Magic III - Isles of Terror (1992)(U. S. Gold)(Disk 4 of 6)[HD-FD install required].adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- might and magic iii - isles of terror (1992)(u. s. gold)(disk 5 of 6)[hd-fd install required]/Might and Magic III - Isles of Terror (1992)(U. S. Gold)(Disk 5 of 6)[HD-FD install required].adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- might and magic iii - isles of terror (1992)(u. s. gold)(disk 6 of 6)[hd-fd install required]/Might and Magic III - Isles of Terror (1992)(U. S. Gold)(Disk 6 of 6)[HD-FD install required].adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- 6 never-loaded region(s), 2701460 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- Might and Magic III - Isles of Terror (1992)(U. S. Gold)(Disk 5 of 6)[HD-FD install required].adf:MM3-Intro

## Copy protection found

- none found

## Functions


### copy

- `perform_file_copy`: Validates source and destination file paths, checks for directory existence, and manages file handles and buffers to copy data between files.
- `call_two_callbacks_with_flag`: If the value in D0 is non-zero, it calls a callback via a pointer in A1 at offset -126 of the library base, then calls another routine from the same library at offset -828.
- `call_function_table`: The routine receives a count in D0 and a pointer to a table in A0.
- `check_bitmask_range`: The routine tests whether an input value in D0 lies within a numeric range and whether a specific bit in a bitmask is set, returning a flag indicating the result.
- `compare_strings_case_insensitive`: The routine compares two byte strings pointed to by A0 and A1 character by character, treating uppercase and lowercase letters as equal.

### makedir

- `set_error_and_cleanup`: The routine initializes an error code and, if a library base is present, it clears a small stack buffer, opens a DOS library context, and iterates over a null-terminated list of strings.

### Might and Magic III - Isles of Terror (1992)(U. S. Gold)(Disk 1 of 6)[HD-FD install required].adf:Install-Harddrive

- `branch_to_offset_2`: Performs a short branch to a specific offset.
- `do_nothing`: This routine performs four identical operations that clear the D0 register by performing a bitwise OR with zero.
- `decompress_data_stream`: Initializes a local buffer with spaces and then performs a decompression routine that reads from a source buffer and writes to a destination buffer using a lookup table and variable-length encoding.
- `increment_and_call_resource_handler`: Increments a value on the stack, pushes it and two other stack values, and calls another routine using a value from a global variable.
- `clear_and_call_function`: Checks if a pointer in a global variable is non-zero, then calls a function via a pointer retrieved from a table using values from that global variable, and finally clears the global variable.
- `allocate_aligned_memory`: Checks if a value is aligned to 8, adjusts it to the next 8-byte boundary if not, calls a routine to allocate memory, and if the result is zero, calls another routine before storing the pointer.
- `load_and_decompress_files`: Performs multiple file operations using a routine to seek and read, calls a routine to allocate memory, calls another routine to decompress data, and calls a final routine to clean up.
- `install_game_resources`: Iterates through a series of resource strings to perform file operations, including setting up various MM3-assigned files and installing fonts to a specified partition.
- `call_parse_resource_command`: Increments a value on the stack by one, pushes it and two other stack values onto the stack, and calls another routine.
- `update_linked_list_entries`: Iterates through a list of entries, potentially calling another routine to modify data at a pointer, and increments a counter and a pointer based on the result and a comparison of values at specific offsets.
- `call_parse_resource_command_with_offset`: Increments a value on the stack by one, pushes it and two other stack values onto the stack, and calls another routine using a value from a global variable as an offset.
- `calculate_stack_pointer_offset`: Calculates a value based on the current stack pointer, pushes it onto the stack, and calls another routine.
- `parse_command_string_3`: Parses a command string by identifying specific character sequences and numeric values, then updates a linked list of entries based on bitmask criteria and calls other routines to process the parsed data.
- `update_linked_list_node`: Checks several conditions on a structure in a2, and if valid, modifies the structure's fields, updates a linked list, and calls a routine via a library pointer.
- `load_constant_address`: Loads the constant value 32766 into register a4 and returns immediately.
- `allocate_and_initialize_string_buffer`: Calls a library routine to get a directory, then calculates the length of a string to allocate a buffer, writes the four-character constant 'MANX' into that buffer, and performs several library calls to manipulate the resulting string and associated data structures.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `parse_command_string`: Parses a command string by splitting it into tokens based on whitespace or quoted substrings, then stores the resulting pointers into a table.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_with_limit`: Finds the end of the first string, then copies characters from the second string into the first, decrementing a counter for each character copied.
- `adjust_value_range`: Adds 32 to a value if it is between 64 and 90 inclusive, otherwise it returns the original value.
- `find_character_in_string`: Searches for a specific byte within a string and returns a pointer to its location, or returns zero if the byte is not found.
- `manage_icon_window`: Calls several library routines to manipulate an icon and a window, stores a result in a global variable, and clears that global variable before branching back to a previous instruction.
- `move_d0_to_d5`: Copies the contents of register d0 into register d5.
- `clear_resource_status_byte`: Sets up a stack frame, modifies three values on the stack including one by adding 32767, calls another routine, and clears a byte at an offset determined by the return value in a structure pointed to by a2.
- `traverse_linked_list_3`: Validates a structure in a2, then iterates through a list of entries, calling a routine for each entry that meets specific bitmask criteria, and updates a linked list by modifying pointers and values.
- `execute_resource_parsing_sequence`: Pushes a constant value onto the stack, calls parse_resource_string, pushes two values from the stack, calls search_and_clear_structure, and returns.
- `parse_resource_string`: Parses a string in a2 to set a 32-bit value in d2 and a 16-bit value in d3, then calls another routine and, if the result is within a specific range, stores the result and the 16-bit value into a structure in a3.
- `search_and_clear_structure`: Iterates through a list of structures to find one where a word at offset 12 is zero, then clears several fields at the start of that structure and returns its pointer.
- `copy_string_or_pad`: Copies characters from a source string to a destination buffer up to a specified count or until the source ends.
- `call_resource_handler`: Pushes a value from the stack to the stack, pushes a constant to the stack, pushes another value from the stack to the stack, calls another routine, restores the stack pointer, and returns.
- `manage_resource_list`: Iterates through a list of resources, performing various library calls to lock, unlock, or manipulate files based on bit flags and status values.
- `validate_resource_index`: Checks if an index is within a valid range and if the corresponding resource is null, then calls a routine to perform an operation or updates a status value.
- `multiply_32bit_fixed_point`: The routine takes two 32-bit fixed-point values from registers and returns a 32-bit fixed-point product.
- `parse_resource_command_string`: Parses a command string to update a linked list of resources by interpreting specific byte markers and numeric values.
- `close_table_entry_2`: Checks a bit in a table entry, calls a routine to handle a resource, performs a calculation involving a swap and division, and clears several fields in the table entry.
- `tiny_addq_addq`: Small routine of 2 instructions that adds 2 to a5, then adds 8 to d0.
- `iterate_table_range`: Calls a routine with a pointer derived from a global address, then increments that pointer by 22 and repeats the process as long as the pointer is less than a second calculated address.
- `modify_linked_list_resource_entry`: Validates a structure by checking specific bits in a status word at offset 12 and then either updates a pointer at offset 8 to the next entry or modifies a byte at a location derived from a pointer within the structure.
- `search_table_entries`: Iterates through a table starting from a global address, checks a bitmask in a word at offset 12 of each entry, and calls another routine to process entries if certain conditions are met.
- `calculate_signed_division_2`: Calculates the quotient and remainder of two 32-bit values, handling signs by negating the inputs and adjusting the divisor before calling a division routine.
- `call_and_return_concatenated`: Calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: Calls another routine and then returns.
- `divide_long_integers`: Performs division on two 32-bit values, returning the quotient and remainder.
- `update_linked_list`: Calls a routine with a byte from a table entry, and if the result is non-zero, updates a pointer in a structure and performs bitwise operations on a word at a specific offset.
- `traverse_and_update_linked_list`: Traverses a linked list to find a specific node, then updates a pointer in the list and calls a function via a library call.
- `jump_to_offset`: Saves registers a2, a3, and a6 to the stack and then branches to another routine.
- `traverse_linked_list`: Follows a linked list by loading a pointer from a structure, calling a routine via a function pointer in a global table, and repeating until the pointer is zero, then clears a global variable.
- `manage_linked_list_node`: Calls a routine via a pointer in a structure, and if the result is non-zero, updates a global pointer to point to a new node containing the result and a provided value.
- `copy_string_simple`: Copies characters from a source string to a destination buffer until the null terminator is copied and returns the pointer to the start of the destination buffer.
- `calculate_string_length`: Receives a pointer to a zero-terminated byte string and advances a pointer until it finds the terminating zero byte, returning the difference between the address of the terminating byte and the original pointer.
- `check_table_entry`: Validates an index against a range and checks if a table entry is non-zero, then calls a routine and returns one if the result is non-zero or negative one if the validation fails.
- `access_resource_data`: Verifies a resource index against a limit and checks for a null pointer, then calls a routine to perform a data access operation.
- `attempt_file_deletion`: Calls a routine to perform an operation, and if it returns zero, calls another routine to check for an error and stores the result in a global variable before returning negative one.
- `perform_resource_write`: Checks resource validity and specific bit flags, then calls routines to perform a write operation or update a status value.
- `check_resource_status`: Calls a routine via a pointer from a global structure, checks bit 12 of the result, and if set, checks a global variable and calls another routine if that variable is non-zero.
- `cleanup_resource_list_2`: Calls a routine via a pointer at offset -60 of a global structure, then calls another routine via a pointer at offset -48, and finally calls a routine with a value of 1 passed on the stack.
- `cleanup_resource_list`: Loops through a linked list of structures to call a function pointer at offset 4 of each node, then calls another function pointer if a global variable is non-zero.
- `execute_resource_cleanup`: Iterates through a list of resources, calls a routine for each, and then performs a series of conditional checks to call various functions or update pointers based on values stored in a structure.
- `restore_registers_and_return`: Restores several data registers and an address register from the stack and returns immediately.
- `jump_to_next_instruction`: Performs a jump to the instruction at a fixed offset from the current program counter.
- `close_table_entry`: Validates an index against a range and checks a table entry, then if a specific bit is not set, calls a routine to close a resource before clearing the entry and returning zero.

### Might and Magic III - Isles of Terror (1992)(U. S. Gold)(Disk 1 of 6)[HD-FD install required].adf:Install-Floppy

- `branch_to_offset_1746`: Performs a short branch to a location 1746 bytes ahead in the code.
- `contains_zero_data`: Performs four identical bitwise OR operations on register d0 with zero.
- `decompress_data_stream_2`: Initializes a local buffer with spaces and then performs a decompression routine on a source data stream, using a lookup table to expand encoded bytes into a destination buffer.
- `call_with_parameter`: If d1 is not zero, it calls another routine with d1 as an argument, then calls another routine with zero.
- `call_library_function_and_clear_pointer`: Checks if a pointer in a global variable is non-zero, then calls a function via a pointer retrieved from a table using a value from a register, and finally clears the global variable.
- `align_and_fetch_resource_pointer`: Aligns a pointer to an 8-byte boundary, calls a library function to retrieve a pointer, and if that pointer is null, calls another routine before storing the aligned pointer into the memory location pointed to by the original input.
- `allocate_and_decompress_resources`: Allocates memory for a data stream and performs decompression, then allocates a second buffer for a subsequent data stream and performs a second decompression.
- `execute_resource_sequence`: Performs a long sequence of calls to various routines using a table in a6, including several calls with a pointer to a string and several calls with a constant of 65536.
- `modify_resource_node_structure_2`: Iterates through a list of structures, copying bytes from a source to a destination and calling another routine to update a pointer, then returns a counter or a specific error value.
- `execute_resource_handler`: Calls a routine with a pointer to a stack location and then calls another routine to perform a sequence of operations involving a loop and library calls.
- `load_address_into_a4`: Loads the constant value 32766 into register a4 and returns.
- `initialize_resource_structure_alt`: Calls a library routine to obtain a pointer, initializes a structure with specific values, and performs a sequence of file and window operations using library calls.
- `copy_d0_to_d5`: Copies the contents of register d0 to register d5.
- `parse_command_string_2`: Parses a string into individual tokens by handling whitespace and quoted substrings, then stores the resulting pointers into a table.
- `copy_d0_to_d0`: Copies the contents of register d0 to register d0.
- `append_string_with_count_limit`: Finds the null terminator of a destination string, then copies bytes from a source string into that location.
- `manage_window_resource`: Calls a library routine to open a resource, stores the result in a global variable, and if non-zero, performs a sequence of icon and window operations including calling another routine to set values in a structure and finally calling a library routine to close the resource.
- `copy_d0_to_d5_2`: Copies the longword value from register d0 into register d5.
- `execute_nested_calls`: Pushes a constant to the stack, calls another routine, pushes two values from the stack, and calls a third routine before returning.
- `parse_resource_string_2`: Parses a string from a pointer in a2 to determine values for d2 and d3, then calls another routine and writes the results to a structure in a3 at offsets 12 and 14.
- `find_and_clear_structure_in_list`: Iterates through a list of structures, checking a word at offset 12 of each; if a match is found where the structure is within a specific range, it clears several fields at offsets 0, 4, 8, and 20 and returns the pointer to that structure.
- `copy_string_or_pad_2`: Copies bytes from a source pointer to a destination pointer for a specified count, stopping if a null byte is encountered.
- `call_resource_handler_2`: Pushes a constant value onto the stack and calls another routine before returning.
- `process_floppy_resources`: Iterates through a list of resources, performing library calls to manage files and updating a global status based on the results of those calls.
- `parse_command_string_and_update_resources`: Parses a command string to identify specific character codes and flags, then performs linked list manipulations on a resource structure to update its status and pointers.
- `close_resource_at_index_3`: Performs a series of checks on a table entry, calls multiple routines to handle resources, and performs a loop to convert a value into a string of characters.
- `increment_registers`: Adds 2 to register a5 and adds 8 to register d0.
- `iterate_resource_table`: Iterates through a table of entries starting from a calculated offset and calls close_resource_at_index_3 for each entry until a boundary is reached.
- `modify_resource_node_structure`: Validates a resource structure via bitwise checks on a status word and then performs linked list manipulations, including updating pointers, adjusting offsets, and writing a byte to a memory location pointed to by the structure.
- `search_linked_list`: Iterates through a list of structures starting from a global address, checking a bit in a word at offset 12 of each entry and calling another routine if specific conditions are met, returning the result of the call or zero.
- `calculate_absolute_division_remainder`: Calculates the absolute value of two 32-bit integers and calls another routine to perform division, returning the result and the remainder.
- `call_and_return_concatenated_2`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call_2`: The routine calls another routine and then returns.
- `divide_32bit_integers`: Performs 32-bit integer division by checking if the divisor is larger than 16 bits; if so, it executes a bit-shifting loop to calculate the quotient and remainder, otherwise it performs 16-bit division on the lower words and returns the results in d0 and d1.
- `allocate_and_update_node`: Calls a routine with a byte from a table entry, and if non-zero, calls another routine to allocate memory and updates a linked list by modifying pointers and bitwise flags.
- `traverse_linked_list_2`: Traverses a linked list starting from a pointer in a global variable, updating the list by replacing a node or inserting a new one, and calls a function from a library via a pointer in a global variable.
- `branch_to_offset`: Saves registers a2, a3, and a6 to the stack, loads a value from a global variable into a2, and branches to another routine.
- `clear_linked_list`: Iterates through a linked list, calling a function for each node, and finally clears a global variable.
- `allocate_linked_node`: Checks if a provided parameter is non-zero, then calls a routine via a pointer in a global structure to obtain a new memory address.
- `copy_string_simple_2`: Copies characters from a source string to a destination buffer until the null terminator is copied.
- `calculate_string_length_2`: Receives a pointer to a zero-terminated byte string and advances a pointer until it finds the terminating zero byte.
- `check_index_validity`: Checks if an index is within a specific range and if a corresponding entry in a table is non-zero, then calls a function from a library and returns a boolean result.
- `update_resource_status`: Checks if a resource index is valid and not null, then calls a routine to update the resource or sets a status value if the operation fails.
- `delete_file_with_error_handling`: Calls a function to delete a file and, if it fails, calls another function to retrieve an error code and stores it in a global variable.
- `handle_resource_access`: Validates a resource index and checks specific bit flags, then calls routines to perform an operation or updates a status value.
- `query_library_function`: Calls a function from a library via a pointer stored in a structure, then checks if bit 12 of the return value is set.
- `execute_resource_release_sequence`: Calls a function pointer at offset 60 from a structure to pass a global variable and the constant 4, then calls a second function pointer at offset 48 from the same structure before calling another routine with the value 1.
- `release_resource_list`: Iterates through a linked list to call a function pointer at offset 4 of each node, then calls a final function pointer if a global variable is non-zero.
- `release_resource_and_cleanup_assets`: Iterates through a list of resources to call a release routine for each, then executes a series of conditional function calls via pointers stored in a structure.
- `restore_registers_and_return_2`: Restores registers d2, d3, and a6 from the stack and returns immediately.
- `close_resource_at_index_2`: Checks if an index is within a valid range and if a resource at that index is non-zero, then if a specific bit is not set, calls another routine and clears the resource pointer.
- `close_resource_at_index`: Validates an index against a range and a table, then if valid, checks a bit in a word at that index and calls a function to close a resource before clearing the entry in the table.

### 8

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `contains_null_data_sequence`: This routine consists of a sequence of instructions that perform logical OR operations with zero on a data register, which results in no change to the register state.

### 9

- `return_negative_one`: Sets register d0 to negative one and returns.

### Might and Magic III - Isles of Terror (1992)(U. S. Gold)(Disk 5 of 6)[HD-FD install required].adf:MM3-Intro

- `jump_to_address`: Performs a long branch to a specific address.
- `increment_and_copy_data`: Increments a 16-bit value at an offset in a table and copies four 16-bit words from one table to another, then copies five 16-bit words from a third table to a fourth table using offsets derived from a global variable.
- `decrement_and_copy_data`: Decrements a 16-bit value at an offset in a table and copies four 16-bit words from one table to another if the value at the offset is non-zero.
- `parse_string_and_update_table`: Parses a string of characters, skipping whitespace and handling specific control characters, while performing arithmetic on values retrieved from a table to update a global variable.
- `update_position_and_call_function`: Calculates a value based on table entries, updates a global variable, and calls a function pointer from a table if a specific condition is met.
- `configure_blt_hardware`: Calculates memory addresses and dimensions for a block transfer, configures several Blitter registers, and waits for a bit in a hardware register to clear multiple times.
- `draw_tiled_rectangles`: Uses a series of loops to fill rectangular areas with patterns or colors by calling a blitter configuration routine and a division routine to calculate offsets and dimensions.
- `update_entity_state_table`: Increments a global counter and populates a structure in a table with values from the stack, including several 16-bit integers and two 32-bit pointers.
- `process_bitmapped_elements`: Enters a loop that retrieves several pointers from a table based on a global counter, calls a routine with these pointers, calls another routine to update state, and decrements the global counter until it reaches zero.
- `populate_data_array`: Calculates an initial address, calls another routine to obtain a pointer, and then enters a loop that runs five times to call two routines and store a calculated address into a table.
- `call_bitmapped_function`: Loads several pointers from the stack, retrieves a pointer from a table using an offset, and calls a function via a pointer from a global structure.
- `tiny_addi_moveq_rts`: Small routine of 3 instructions that adds 1 to (a1), then loads 0 into d0, then returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `setup_blitter_parameters`: Stores several 16-bit values into global variables by splitting them into high and low bytes, and then calls another routine.
- `setup_blitter_graphics_transfer`: Parses a data structure to calculate dimensions and source offsets, performs a block memory copy using a loop, and configures the Blitter hardware registers to prepare for a graphical transfer.
- `clamp_scaled_value_into_struct`: It multiplies a starting value by two signed bytes from a structure, clamps the result to a maximum, and stores the clamped value back into the structure.
- `find_table_index`: Performs multiple table lookups using values derived from d0 and d1, compares the results, and if certain conditions are met, calls another routine and performs bitwise operations before storing a result through a pointer.
- `store_byte_masked_with_two`: It masks a byte received in a register with the value two and stores the resulting byte through a pointer.
- `compare_table_values`: It calculates two indices based on a value in d0, fetches bytes from a table at offset 49 of a structure using those indices, and compares the difference against zero and a bitmask.
- `calculate_scaled_offset`: Calculates a value using arithmetic operations and division, then updates a 32-bit field at offset 32 of a structure in a0 by adding or subtracting 65536 based on a comparison with a constant.
- `shift_scaled_table_value`: It computes a scaled, rounded value from two 16-bit inputs using a global table, then applies an arithmetic right-shift by a count derived from the inputs.
- `calculate_buffer_size`: Calculates a size by multiplying a value in d1 by a constant and a value from a table, then calls another routine and returns.
- `transfer_and_branch`: Moves the value from d2 to d0 and then performs a short branch.
- `check_structure_flags`: Loads a longword from a structure pointed to by a2 into d0, branches if d0 is zero, otherwise updates a value at offset 16 of the structure, checks bit 7 of a value at offset 16 of a3, and checks bit 1 of a byte at offset 50 of the structure.
- `branch_on_byte_match`: Compares a byte at offset 49 of a structure pointed to by a2 with 1, then branches to another routine or copies a2 to a0 and calls another routine.
- `advance_buffer_state`: Checks if a status byte at offset 49 of a structure is 6, then updates the structure by copying a pointer from a sub-structure, setting a new status byte to 5, and calculating a 32-bit value from a 16-bit word in a0 to store at offset 20 of the structure before calling another routine.
- `update_structure_and_branch`: It stores a byte at offset 49 of a structure, stores a word from a structure at offset 16 of another structure, stores a long from d2 into offset 16 of that same structure, and branches to another routine.
- `update_status_byte`: Compares a byte at offset 49 of a structure in a2 to 3; if equal, it copies a pointer from offset 8 of a5 to offset 8 of a2, otherwise it sets a byte at offset 49 of a2 to 1 and branches.
- `process_buffer_metadata_and_state`: Copies a 16-bit word from a structure to offset 36 of another structure, calculates a 32-bit value by shifting a 16-bit word from a0 and stores it at offset 20, and updates a status byte at offset 49.
- `update_structure_and_branch_short`: It stores a byte at offset 49 of a structure, stores a long from d2 into offset 16 of another structure, and branches to another routine.
- `calculate_buffer_metadata`: Stores a pointer at offset 8 of a structure in a2, calculates a 32-bit value by shifting a 16-bit word from that pointer and subtracting a 16-bit value from offset 40 of the structure, stores this result at offset 28 of the structure, and calls another routine.
- `clamp_and_assign_d0`: It adds a long from offset 28 of a structure to a word from offset 28 of another structure, then clamps the result between zero and 32768 before storing it in d0.
- `update_object_state`: Sets a 16-bit word at offset 40 of a structure to zero, subtracts d2 from offset 20, calls a routine, and if bit 1 of offset 50 is set, adds d2 to offset 24 and potentially clears bit 1 of offset 50 and moves a byte from offset 45 to offset 44.
- `update_object_state_and_call_handler`: Sets bit 4 of a byte at offset 50 of a structure, updates a global structure with a bit-shifted value and two 16-bit words, then calls a routine from a table.
- `store_words_then_call_table`: It stores a fixed word and a zero word into a structure, then calls a routine stored in a table.
- `clear_structure_high_bits`: Runs a loop three times calling another routine, then clears the high bit of sixteen consecutive bytes in a structure starting at offset 16, and finally clears the high bit of a single byte in a global variable.
- `update_flag_and_hardware`: Sets a bit in a structure at offset 24 of a global variable based on a value in d0, sets a byte at offset 30 of that same global variable to one, calls a routine from a table, and potentially writes a value to a hardware register.
- `reset_structure_and_update_hardware`: Checks if a byte at offset 15 of a structure pointed to by a0 is not 16; if so, it decrements that byte and a byte at offset 169 of a global variable, then clears several fields in the structure at a0 and calls a routine to update a bit in a global variable and a byte in another global variable.
- `init_struct_and_toggle_flag`: It initializes several fields of a structure with fixed values, then toggles and clears specific bits of a flag byte.
- `update_word_from_byte`: Compares a byte from a1 to 5; if it matches, it shifts a byte from offset 1 of a1 left by 7 bits and stores the result in a 16-bit word at offset 6 of a0, otherwise it returns.
- `update_word_with_byte`: If d0 is 37, it performs a bitwise AND on a 16-bit word at offset 6 of a structure in a0, ORs it with a byte from offset 1 of a1, and writes the result back to the structure.
- `update_structure_attribute`: If the value in d0 is 6 and the 16-bit word at offset 4 of the structure in a0 is 0, it clamps a byte from offset 1 of a1 to a maximum of 24, stores it at offset 12 of the structure, calculates a new 16-bit value using a multiplication and division involving the value at offset 8 and the clamped byte, stores the result at offset 10, and sets bit 7 of the byte at offset 16 of the structure before returning.
- `increment_and_store_byte`: Compares a byte in d0 to 7 and returns if not equal; otherwise, it reads a byte from the address in a1, increments it if the byte is not zero, and stores the result into a structure at the offset specified by d0.
- `check_byte_and_set_bit`: Compares a byte in d0 to 10, otherwise it returns.
- `set_structure_bit_2`: Sets bit 0 of a byte at offset 16 of a structure and branches to another routine.
- `test_byte_and_set_bit`: Compares a byte in d0 to 64, branches if not equal, checks bit 6 of a byte at offset 1 of a structure in a1, and if bit 6 is set, sets bit 3 of a byte at offset 16 of a structure in a0 before branching to another routine.
- `clear_structure_bits`: Clears bits 2 and 3 of a byte at offset 16 of a structure, then enters a loop that runs four times to clear bits in a structure at offset 32 and set a byte at offset 31 of a different structure, finally branching to another routine.
- `check_status_and_set_bit`: Compares a byte in d0 to 65, branches if not equal, checks bit 6 of a byte at offset 1 of a structure in a1, and if bit 6 is set, sets bit 2 of a byte at offset 16 of a structure in a0 before branching to another routine.
- `clear_byte_and_branch`: Clears bit 2 of a byte at offset 16 of a structure in a0, stores the value negative one in a byte at offset 17 of a structure in a2, and branches to another routine.
- `compare_status_bytes`: Compares a byte in d0 to 81, otherwise it branches; if the first comparison passes, it sets d0 to zero and compares a byte at offset 1 of the structure in a1 to 64, otherwise it branches; if that comparison passes, it branches; otherwise it sets d0 to 1 and branches.
- `load_pointer_and_call`: Loads a long value from a global variable into d0, calls another routine, and then branches to another routine.
- `copy_byte_if_equal_to_hundred`: Compares a byte in d0 to one hundred; if equal, it moves a byte from offset one of the structure in a1 to offset five of the structure in a0, otherwise it returns.
- `compare_and_move_byte`: Compares a byte in d0 to 101; if equal, it moves a byte from offset 1 of a structure in a1 to offset 4 of a structure in a0, otherwise it branches to another location.
- `compare_and_branch_byte`: Compares a byte in d0 to 127; if equal, it calls another routine and branches to another location, otherwise it branches to another location.
- `compare_and_branch`: Compares d0 to 127 and, if equal, calls another routine and branches to another routine; otherwise it returns.
- `set_bit_and_call`: Compares d0 to 126; if they are equal, it sets bit 1 of a byte at offset 16 of a structure in a0, calls another routine, and branches to another routine; otherwise it branches to another routine.
- `clear_structure_bit`: If the value in d0 is 127, it clears bit 1 of a byte at offset 16 of a structure in a0 and calls another routine; otherwise it returns.
- `assemble_value_and_scale_diff`: It assembles a 16-bit value from two bytes, stores it in a structure, computes a scaled difference, and sets a high bit in a flag byte.
- `perform_signed_division`: Determines the sign of the result based on input registers, calls a routine to perform unsigned 32-bit division, and negates the quotient if the sign flag indicates a negative result.
- `move_division_remainder_to_quotient`: Calls a routine to perform unsigned 32-bit division and moves the resulting remainder from d1 to d0 before returning.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `perform_32bit_division`: Calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1, returning the result in a concatenated 64-bit value.
- `multiply_32bit_fixed_point_2`: The routine takes two 32-bit fixed-point values from registers and returns a 32-bit fixed-point product.
- `calculate_bit_shifted_value`: Performs a series of arithmetic operations involving a table lookup and multiplication to derive a value, then applies bitwise transformations and a conditional shift based on the high byte of D0.
- `tiny_move_moveq_rts`: Small routine of 3 instructions that copies (a7)+ to d2, then loads 0 into d0, then returns.
- `initialize_global_state_and_callback`: Checks if a global variable is zero, and if so, calls a routine via a pointer at an offset from that global variable, stores the result in a global variable, and if the result is non-zero, sets several global variables to offsets of that result and calls another routine via a pointer at an offset from a global variable.
- `execute_callback_and_clear_flags`: Calls another routine twice, then checks a global variable; if a second global variable is non-zero, it calls a routine via a pointer at an offset from the first global variable and clears that second global variable; finally, it checks a third global variable, and if non-zero, calls a routine via a pointer at an offset from the first global variable and clears that third global variable before returning.
- `initialize_audio_data_structures`: Initializes several data structures by populating them with specific values through multiple loops, sets up linked lists, and calls init_struct_and_toggle_flag and check_global_flag_bit.
- `clear_structure_flags`: If a global variable is not zero, it clears bit 0 of two global variables, calls another routine, iterates through a table of four structures to call another routine for each structure where bit 3 of a byte at offset 50 is set and then clears that byte, calls another routine via a pointer from a global variable, calls a library function, and clears the global variable.
- `update_object_state_and_global`: Checks if an object structure is valid and meets specific bitwise criteria, then updates a global variable and modifies fields within a structure, including swapping bytes at offsets 44 and 45 and updating a 16-bit value based on a byte from a second structure.
- `validate_and_initialize_object_structure`: Checks a bit in a structure at offset 22 of a2; if set, it calls another routine to find an object, verifies the object is not null and has a non-zero value at its first word, then updates several fields in the object structure including a scaled value at offset 38, a status byte at offset 80, and various integer fields via a secondary routine call.
- `update_display_structure`: Calculates a scaled value based on a byte from a structure and a global flag, stores it in a structure at offset 38, and then performs a series of library calls to update a display or object structure with values derived from other structures.
- `update_structure_and_call_library`: Calls a library routine, and if the result is non-zero, populates a structure in a1 with specific values derived from a structure in d3 and a5, then conditionally calls another library routine.
- `update_structure_and_global`: Increments a byte at offset 14 and a global variable, then conditionally updates a 32-bit value and two bytes at offsets 44 and 45 of another structure using values from a third structure, finally updating a byte at offset 46 of the third structure and setting a global variable to a byte from offset 46 of the second structure.
- `clear_structure_fields`: Clears a longword at offset 12 of a structure in a5, clears bytes at offsets 30, 31, and 32 of that structure, and then branches to another routine.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `wait_for_hardware_condition`: Enters an infinite loop if a specific value at an offset of a global pointer is greater than a certain threshold, otherwise it modifies a value at an offset of a pointer and writes it to a hardware register.
- `compare_and_set_bit`: Checks if a pointer in a0 matches an entry in a table calculated from a global variable, then compares a byte from a1 with a value from the table and conditionally sets a bit or a byte in the table structure.
- `compare_and_set_byte`: Compares a byte at offset 44 of a structure in a2 with d0; if they are equal, it checks bit 3 of a byte at offset 16 of a structure in a0, and if that bit is set, it writes 3 to offset 49 of the structure in a2.
- `set_structure_bit`: Sets bit 2 of a byte at offset 50 of a structure pointed to by a value on the stack.
- `check_bit_and_set_flag`: Tests a byte at offset 14 of a structure in a0, and if bit 3 of offset 16 is set, sets bit 2 of offset 32 of a structure in a1.
- `process_list_with_flags`: Sets a byte at offset 49 of a structure in a1 to 3, updates the pointer in a1 to offset 52 of the structure, and loops using a counter in d0.
- `iterate_structures_and_toggle_flags`: Loops four times, calling another routine for each of four structures stored consecutively in a table whenever the first field of that structure is nonzero, advancing by a fixed stride each iteration.
- `store_table_entry_pointer`: Stores the address of a global table entry selected by masking an input value and multiplying by a fixed stride into a pointer supplied in a register.
- `store_register_and_call_division`: Stores a word taken from a register into a global variable, then calls a division routine purely for its side effects and stores the discarded result into another global variable before returning.
- `update_flag_from_input`: Copies d0 to d1, calls another routine, and then sets or clears bit 1 of the CIAA_PRA register based on the value in d1.
- `check_global_flag_bit`: Checks a flag bit in a global variable and returns one when the bit is clear, otherwise returns zero.
- `update_object_state_and_process_events`: Updates a structure's status byte by masking bits and sets a flag in a linked list of objects, then iterates through a list of event structures to check for specific type markers and executes a callback routine if certain conditions are met.
- `write_long_and_branch`: Writes a longword from a register to an offset of 2 from a pointer in a register and then branches to another routine.
- `process_object_data_and_update_state`: Checks a structure's validity against a pointer, updates a global variable, and performs a loop to modify data.
- `validate_and_process_byte_threshold`: Compares a byte in d1 to 128 and, if equal, performs a bitwise shift and comparison on a value from a structure in a4 against a word in a structure in a2, then calls another routine and clears a longword at offset 20 of the structure in a4 before branching.
- `update_stream_metadata`: Copies a 16-bit value from a structure to another location within the same structure, calculates a new 16-bit value by subtracting the original value from a byte at offset 1 of a second structure shifted left by 4, and stores this result at offset 102 of the first structure.
- `check_byte_and_clear_fields`: Compares a byte in d1 to 255 and branches if not equal, otherwise checks bit 4 of a byte at offset 111 of a structure in a4 and branches if it is zero; if bit 4 is set, it loads a pointer from offset 40 of a4 into a0, copies the value pointed to by a0 into a2, clears the 32-bit values at offsets 8 and 24 of the structure in a4, and branches to another routine.
- `clear_flag_and_branch`: Clears bit 0 of a byte at offset 111 of a4 and branches to another routine.
- `extract_and_call_scaled_offset_handler`: Checks if a byte in d1 equals 224, then extracts two 7-bit values from offsets 5 and 4 of a structure in a2 to store them on the stack.
- `fetch_data_and_call_handler`: Compares a byte in d1 to 176; if not equal, it branches to another routine, otherwise it moves a byte from offset 4 of the structure in a2 to the stack, moves a byte from offset 5 of the structure in a2 to offset 1 of the stack, extracts a value from offset 1 of the structure in a2, masks it with 15, multiplies it by 20, adds the result to a base address, and calls another routine.
- `select_table_entry_and_call`: Compares a byte in d1 to 192; if not equal, it branches to another routine, otherwise it moves a byte from offset 5 of the structure in a2 to the stack, extracts a value from offset 1 of the structure in a2, masks it with 15, multiplies it by 20, adds the result to a base address, and calls another routine.
- `process_event_type_and_dispatch`: Checks a byte in a register to determine an event type; if the type is one, it stores a byte into a structure and calls another routine; if the type is two, it increments a count in a structure and stores a byte and a pointer to the next element in a list.
- `append_element_to_list`: If the value in d0 is 2, it checks if a count at offset 108 of a structure in a4 is not 4; if so, it increments that count, stores the byte from d1 into a table at offset 104 plus the count multiplied by 1, and updates a pointer in a table at offset 48 of the structure in a4 with the address of the next element.
- `decrement_and_fetch_list_element`: Checks if a byte in d0 is equal to 3, then retrieves a count from a structure in a4 and decrements it.
- `store_byte_and_advance`: Stores a byte from d0 into offset 108 of a structure in a4 and updates the pointer in a2 to offset 6 of the structure in a2 before branching.
- `process_data_and_branch`: Copies a pointer from a4 to a2, checks if a value at offset 20 of a4 is zero, performs arithmetic on a value at offset 24 of a4, and calls another routine if a comparison fails.
- `update_voice_and_flags`: Calls two routines, adds a word to d0, calls a third routine, calls a fourth routine, sets bit 2 of a byte at offset 111 of a4, and checks a pointer in a structure before clearing bit 2 of the same byte.
- `merge_voice_flags_into_channel`: Walks a block of records, calling a routine for those with a flag set, then merges one channel flag byte into another and returns zero.
- `search_string_for_marker`: Decrements a counter in a register and enters a loop that scans a memory buffer for a specific six-byte sequence, resetting the buffer pointer from a global variable if the sequence is not found.
- `invoke_routine_wait_flag`: Stores a constant and a parameter into global variables, invokes a routine via a pointer in RAM, then busy-waits until a global flag clears before returning the stored parameter.
- `move_stack_parameter`: Moves a 32-bit value from a location on the stack into register d0.
- `initialize_and_setup_audio`: Calls a routine and, when it reports success, clears two flag bits, performs several setup and division side effects, and then stores a byte from one table into another.
- `check_param_and_set_flag`: Compares a parameter passed in a register against zero and calls a routine.
- `invoke_routine_with_parameters`: Stores five parameters from the stack into a local structure, sets a global counter to two, calls a routine from a table, and busy-waits until the global counter reaches zero before returning a pointer to the local structure.
- `invoke_routine_with_counter`: Stores a passed value and a constant of three in global variables, calls a routine from a table, and busy-waits until a global variable reaches zero.
- `update_structure_flags`: Checks bit 3 of a byte at offset 50 of a structure in a2, calls a routine via a pointer from a table, and if successful, populates a new structure with a bit-shifted value and calls another routine.
- `clear_bit_and_maybe_recurse`: Clears bit 3 of a byte at offset 80 of a structure in a2, zeroes bytes at offsets 48 and 47, and if the byte at offset 47 is not equal to the byte at offset 46, it calculates an offset and calls another routine.
- `wait_for_counter_to_zero`: Stores a passed value in a global variable, invokes a routine from a table, then busy-waits until a counter reaches zero before returning that value.
- `process_entity_event_queue`: Checks a global counter and, if it equals one, clears a pointer and calls another routine; if that routine returns zero, it calculates an offset into a table based on a value from a structure, populates an entry in that table with a byte and a 16-bit value, and sets a global pointer to one.
- `initialize_entity_structures`: If a register contains two, it clears a global pointer, calls another routine, and if successful, populates a structure with values from a table and a global pointer, including setting a bit in a field at offset 24 based on a byte at offset 46 of a structure.
- `update_linked_list_and_hardware`: Calls a routine using a pointer from a table, clears bit 3 of a byte at offset 50 of a structure in a5, and if a byte at offset 31 of a structure in a1 is zero, it compares the high words of two values from the hardware video registers and potentially writes a modified word from a structure in a1 to the hardware register at offset 150.
- `modify_linked_list_data_2`: Loads a pointer from a table into a6, calls a routine via an offset from a6, sets bit 3 of a byte at offset 50 of a structure in a1, and then branches to another routine.
- `modify_linked_list_data`: Sets bytes at offsets 48 and 47 of structures in a1 and a5, and if certain conditions are met, updates a longword in a global table and a structure in a5.
- `process_structure_updates`: If a register contains 3, it calculates an offset into a table using a global pointer, compares a value at offset 12 of the resulting structure with the global pointer, and if bit 3 of offset 50 is set, calls a routine and clears several bytes and a longword at offsets 48, 49, 50, and 12 of that structure.
- `update_structure_pointer`: Compares d2 to 4, and if equal, performs a calculation using a value from a global pointer, checks a value in a table, calls a routine, and if successful, updates the global pointer with 1.
- `clear_global_longword`: Clears a longword at offset 8fe and branches to another routine.
- `check_conditional_flag`: Checks if d2 is 5 and if bit 0 of a global variable is set; if so, it moves a longword from a global variable to d0, clears that global variable, calls another routine, and then stores 1 in the cleared global variable.
- `allocate_and_initialize_structure`: Calls a routine to allocate memory, and if successful, calls another routine to initialize the new structure with values from d2, d3, and d0, then stores the new structure pointer into the location pointed to by a0.
- `free_linked_list_entries`: Walks a doubly linked list, freeing each entry, then clears the head pointer of the list.
- `clear_table_entry_fields`: Clears several fields of a table entry and returns a value retrieved from one of those fields.
- `load_stack_parameter_to_d0`: Loads a 32-bit value from a location four bytes above the current stack pointer into a data register.
- `clear_table_and_free_entries`: Performs two loops of 63 iterations calling another routine, then iterates through a table to call another routine via a pointer and zero out entries if a value at offset 2 is non-zero, finally clearing several global variables.
- `manage_entity_structures`: Allocates and initializes a list of structures based on a count and type, managing memory through a library and updating global counters and flags.
- `check_global_pointer`: Tests a global pointer; if it is not zero, it branches to another routine, otherwise it clears register d0 and returns.
- `call_library_vector_180`: Library stub: loads a library base from a global variable and calls the library function at offset minus 180 of that base, returning its result to the caller.
- `call_subroutine_and_clear_d0`: Calls another routine and then clears register d0 before returning.
- `decompress_rle_data`: Initializes a local buffer with spaces and then decompresses a data stream into a destination buffer using a run-length encoding scheme.
- `decode_checksum_data`: Reads a byte from a pointer in a0, performs bitwise operations and arithmetic on it, and returns a 16-bit value calculated by adding the low word and high word of a 32-bit accumulator.
- `search_table_for_match`: Iterates through a table of 16-bit words, comparing a value at an offset to a target; if a match is found, it copies two 32-bit values from the table to locations specified by pointers in a1 and returns 1, otherwise it returns 0.
- `push_table_value`: Fetches a 16-bit word from a table using a2 and d0 as base and index, then pushes that word onto the stack pointed to by a1.
- `push_table_value_alt`: Fetches a 16-bit word from a table using a4 and d0 as base and index, then pushes that word onto the stack pointed to by a2.
- `initialize_system_state`: Saves registers, calls three different routines, stores the result of the second call into a global variable at an offset from a4, sets a byte at a specific offset in a4 to 1, calls a fourth routine, and restores registers.
- `execute_sequence_of_routines`: Pushes several constants onto the stack, calls three different routines, calls a fourth routine, and if the result of that fourth routine is zero, it pushes 1 onto the stack and calls a fifth routine before returning.
- `process_coordinate_table`: Loops through a table of 40 entries, performing arithmetic and signed division to calculate new values, writing results to a destination table, and calling two routines via a pointer from a table if any entries were processed.
- `manage_blitter_state`: Calls a routine to setup parameters, then if the result is non-zero, performs several library calls via a pointer from a structure, calls another routine with several arguments pushed to the stack, and performs more library calls before returning.
- `initialize_intro_sequence_data`: Initializes a structure by calling a routine to obtain a pointer, then populates several fields including a pointer to a global variable and a function pointer.
- `initialize_graphics_and_intuition`: Initializes the graphics and intuition libraries by opening a screen and a window, then proceeds to allocate memory for several bitmaps and sets up a table of pointers for graphical operations.
- `call_load_routine`: Saves registers to the stack, calls another routine, restores the registers, and returns.
- `load_global_resource_file`: Attempts to open and read a resource file, and if successful, calculates a memory allocation size based on a value from a structure and allocates that memory.
- `load_and_process_data`: Calls a routine to get a value, then uses that value to call another routine with several parameters from the stack, and finally calls a third routine before returning.
- `load_and_decompress_data`: Calls three library routines via a table, then calls another routine to decompress data into a buffer, and finally calls a routine to handle a pointer or a specific data value.
- `call_subroutine_with_parameter`: Pushes a 32-bit value from offset 20 of the stack onto the stack and calls another routine.
- `handle_load_error`: Calls a routine to get a value, then calls another routine with a value of 2 if the first result is not zero, otherwise it calls a routine with a value of 2 and enters a loop that calls another routine.
- `process_decompression_buffer`: Calls a routine to obtain a value, then calls a routine to allocate memory, calls a routine to decompress data, and finally calls a routine to process the resulting buffer.
- `call_subroutine_via_stack`: Pushes a 32-bit value from the stack and calls a routine, then returns.
- `calculate_modulo_result`: Compares two words from the stack, and if the first is not greater than the second, calls two routines to perform arithmetic and returns a masked result.
- `delay_if_small_value`: Compares a longword from the stack to 20, and if it is less than 20, performs a division and calls a routine via a pointer from a table.
- `interpolate_color_palette`: Compares two sets of 32 words from a table and performs a multi-stage linear interpolation of the nibbles within each word to transition between color values.
- `update_pointer_table`: Calculates an aligned address based on a value from the stack, calls a library routine to obtain a pointer, and stores that pointer into a table at the calculated address.
- `map_input_to_action_2`: Calls a routine via a pointer in a structure, then uses the result to conditionally update a global variable and perform various subtractions on d0 before calling another routine via a pointer.
- `map_input_to_action`: Translates an input value from a register into a specific byte or modifies several global variables based on the input value.
- `process_input_loop`: Calls a routine via a pointer, then if the result is non-zero, it checks bit 10 of a value from a structure and conditionally calls another routine in a loop.
- `clear_pointer_and_call`: Checks if a pointer in a global variable is non-zero, then calls a routine via a pointer from a table using a value derived from that global variable, clears the global variable, and returns.
- `perform_system_cleanup`: Calls several routines, then if a value in a3 is not zero, it enters a loop that copies 32 words from one table to another.
- `call_graphics_load_rgb4`: Sets up arguments and calls a routine via a pointer from a table to perform a graphics operation.
- `update_display_state`: Calls a routine via a pointer from a table, updates a global variable with a value from another global variable based on a condition, copies a value from a table into a structure, calls two more routines via pointers from a table, and toggles a bit in a global variable.
- `blit_bitmaps_and_wait`: Sets up several constants, retrieves a pointer from a table based on a value in a global variable, calls a routine via a pointer from a table to perform a bitblit, and then calls another routine via a pointer from a table to wait for the operation to complete.
- `shift_and_store_result`: Calls a routine via a pointer from a table, shifts the returned value right by 10 bits, and stores the result in a global variable.
- `set_intuition_pointer`: Calls a routine via a pointer from a table to set a pointer.
- `manage_entity_loop`: Calls another routine, then enters a loop that decrements a counter in d4 and updates two bytes in a global structure, calling another routine in each iteration until the counter reaches zero, then performs two final calls to other routines.
- `prepare_stack_for_parsing`: Creates a pointer to a location on the stack, increments it, clears bit 0 of the result, and calls another routine using that pointer.
- `store_parameter_to_global`: Stores a 32-bit value from the stack into a global variable and returns.
- `calculate_scaled_value`: Calls a routine, adds a constant to the result, stores it in a global variable, and returns the result of a bitwise shift and mask operation.
- `load_address_into_a4_2`: Loads the constant value seven thousand one hundred and ninety-eight into register a4 and returns.
- `setup_environment_and_windows`: Saves registers, performs arithmetic on a value from a4, calls several library routines via a pointer in a4, manipulates bits in a structure, and calls another routine before returning.
- `move_d0_to_d5_2`: Moves the contents of register d0 into register d5.
- `tokenize_string_into_array`: Parses a string by identifying words separated by whitespace or enclosed in double quotes, then stores pointers to these tokens into a dynamically allocated array.
- `move_d0_to_d0`: Moves the contents of register d0 into register d0.
- `copy_string_with_limit_2`: The routine finds the end of the first string, then copies characters from the second string into the first, decrementing a counter for each character copied.
- `initialize_window_and_icons`: Calls a library routine to initialize a value in a global variable, then if successful, uses that value to call several icon and window library routines, updates a structure with specific values from a pointer, and finally clears the global variable.
- `copy_d0_to_d5_3`: Copies the 32-bit value from register d0 to register d5.
- `parse_and_clear_byte`: Calculates a value based on a pointer, calls another routine, and then clears a byte at offset 0 of a structure using the returned value as an index.
- `parse_signed_integer`: Skips leading whitespace, checks for a sign character, and then parses a sequence of numeric characters into a signed integer by checking a table for valid digits.
- `copy_string_or_pad_3`: The routine copies characters from a source string to a destination buffer up to a specified count or until the source ends.
- `multiply_32bit_fixed_point_3`: Calculates the product of two 32-bit fixed-point values stored in registers d0 and d1 and returns the result in d0.
- `parse_command_string_4`: Parses a byte string to identify command characters and numeric values, updating a linked list structure and setting various flags based on the characters encountered.
- `check_structure_bit`: Loads a pointer from an offset of 8 in a structure in a5, checks if the word at offset 12 of that pointer is zero, and if not, sets d0 to 255 and returns; otherwise it returns immediately.
- `process_structure_data`: Checks a bit in a structure, calls several routines to process data, performs a loop to convert a value into a string of characters, and clears several fields in the structure before returning.
- `clear_d0_and_branch`: Sets register d0 to zero and branches to another location.
- `increment_frame_pointer_and_data_offset`: Increments a 16-bit value in a frame pointer by two and adds eight to a data register.
- `execute_loop_of_calls`: Calls another routine repeatedly in a loop, incrementing a pointer by 22 each time, until the pointer reaches a calculated limit.
- `manipulate_linked_list_nodes`: Validates a structure via bit tests on a word at offset 12, then performs complex updates to the structure's pointers and flags.
- `search_structure_list`: Iterates through a list of structures, checking a bit in a word at offset 12 of each, and calls a routine to process them if specific conditions are met.
- `compare_address_and_branch`: Adds 22 to register a2, compares it to a value from a global variable, and either branches to another location or sets register d0 to zero and branches elsewhere.
- `push_address_and_call`: Pushes the constant value 65535 onto the stack, calls another routine, cleans up the stack, and branches to another location.
- `calculate_signed_division`: Performs a division on two values and applies the correct sign to the result based on the input signs.
- `prepare_signed_division`: Pushes d1 and d4 onto the stack, negates d0 and d1 if they are negative, and calls a routine via a branch to stack pointer.
- `concatenate_call_result`: Pushes a 32-bit value from d1 onto the stack, calls another routine, and returns a 64-bit value formed by concatenating the original d1 value with the new d1 value.
- `call_divide_long_integers_2_and_return`: Pushes a 32-bit value onto the stack, calls divide_long_integers_2, pops the value back into a register, and returns if the resulting quotient is not zero.
- `divide_long_integers_2`: The routine performs division on two 32-bit values, returning the quotient and remainder.
- `update_linked_list_entry`: Extracts a byte from a structure, calls a routine with that value, and if the result is zero, performs a comparison and potentially updates a structure's offset 16 and offset 8 values, then modifies a 16-bit word at offset 15 by ORing it with 32768 and updates pointers at offsets 4 and 0.
- `branch_to_pointer_list`: Loads a value from a stack offset into d2, decrements a3, loads a value from a global structure into a2, and branches to another routine.
- `copy_pointer_list`: Loops through a list of pointers, comparing the address at offset 8 of the current structure in a2 with d2, and if not equal, sets a3 to a2, updates a2 to the value pointed to by a2, and repeats.
- `copy_pointer_to_memory`: If d0 is not zero, copies a longword from the address pointed to by a2 into the address pointed to by a3.
- `load_and_call_routine`: Loads a value from a pointer in a2 into a global variable, increments a value in d0, and calls a routine using a pointer from a global variable.
- `load_list_pointer_and_branch`: Saves registers a2, a3, and a6 to the stack, loads a pointer from a global variable into a2, and branches to another location.
- `traverse_linked_list_and_call`: Iterates through a linked list by following pointers in a structure, calls a function pointer at an offset from each element, and clears a global variable when the list ends.
- `insert_into_linked_list`: Checks a longword from the stack, calls a routine via a pointer from a table, and if the result is non-zero, updates a global pointer and a linked list structure.
- `copy_string_simple_3`: The routine copies characters from a source string to a destination buffer until the null terminator is copied.
- `calculate_string_length_3`: The routine receives a pointer to a zero-terminated byte string and advances a pointer until it finds the terminating zero byte.
- `check_table_entry_2`: Checks if an index is within a valid range and its corresponding table entry is non-zero, then calls another routine using a pointer from that entry and returns a boolean result.
- `process_index_data`: Calls another routine, calculates an offset based on the result, and if the result is within a specific range, calls a routine via a pointer from a global structure; it may also call another routine and update a global variable.
- `handle_file_error`: Calls a routine via a pointer from a table, and if the result is zero, calls another routine via a pointer and stores the result in a global variable.
- `clear_d0_and_branch_2`: Sets register d0 to zero and performs a short branch to another routine.
- `process_index_with_flags`: Calls another routine to validate an index, then checks specific bits in a word at offset 4 of a structure; depending on these bits, it calls one or two routines via pointers from a global structure and updates a global variable.
- `execute_library_callback`: Calls a routine via a pointer from a global structure, checks bit 12 of the result, and if set, calls another routine if a global variable is non-zero.
- `execute_callback_chains_2`: Loads a pointer from a global structure, calls a routine via a pointer at an offset, loads two values from the stack, calls another routine via a pointer at an offset, and calls a third routine with a pointer on the stack.
- `execute_callback_chains`: Iterates through a linked list of structures at a global address, calling a routine via a pointer at offset 4 of each node, then checks a second global address to call another routine and calls a third routine with a value from the stack.
- `process_callback_chain`: Iterates through a loop based on a value in a global variable, calls a routine via a pointer, and executes several conditional library calls and function pointers stored in a structure.
- `restore_registers_and_return_3`: Restores registers d2, d3, and a6 from the stack and returns.
- `jump_to_other_routine`: Performs a jump to another routine.
- `clear_table_entry`: Checks if an index is within a valid range and its corresponding table entry has a specific bit cleared, then calls another routine and clears the entry before returning zero.
