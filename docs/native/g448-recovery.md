# Game 448 (Lords of the Realm): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00448). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2765072 top-level bytes in 127 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 18 of 18 units identical (bytes and relocations), 34 raw-word lines |
| Functions named and described | pass, 696 of 696 named (0 author symbols), 696 of 696 described (45 by rule, 619 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 5076 of 5097 (99.59%) |
| Recovered without load address (counted in proof 1, flagged) | 8946 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 20691 of 20705 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1589793 |
| code | 70936 |
| data | 751360 |
| padding | 27351 |
| unused | 325632 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Lords of the Realm (1995)(Impressions)(Disk 1 of 3).adf:HDInstall | 3 | 5224 | 141 |
| icon.library | 10 | 1375 | 108 |
| Lords of the Realm (1995)(Impressions)(Disk 1 of 3).adf:lords | 9 | 251 | 6 |
| Lords of the Realm (1995)(Impressions)(Disk 1 of 3).adf:message | 2 | 1472 | 30 |
| Lords of the Realm (1995)(Impressions)(Disk 2 of 3).adf:game.bm5 | 2 | 3270 | 93 |
| Lords of the Realm (1995)(Impressions)(Disk 1 of 3).adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4690 | 202 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 107 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 158 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 46 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 40 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4009 | 92 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| Lords of the Realm (1995)(Impressions)(Disk 2 of 3).adf:casnames.eng (raw code file, load address unknown) | 1 | 0 | 0 |
| Lords of the Realm (1995)(Impressions)(Disk 2 of 3).adf:casnames.fre (raw code file, load address unknown) | 1 | 0 | 0 |
| Lords of the Realm (1995)(Impressions)(Disk 2 of 3).adf:casnames.ger (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- lords of the realm (1995)(impressions)(disk 1 of 3)/Lords of the Realm (1995)(Impressions)(Disk 1 of 3).adf:mod.lords: high-entropy data file (7.75 bits/byte), classified as packed data
- lords of the realm (1995)(impressions)(disk 1 of 3)/Lords of the Realm (1995)(Impressions)(Disk 1 of 3).adf:messages.eng: high-entropy data file (7.72 bits/byte), classified as packed data
- lords of the realm (1995)(impressions)(disk 1 of 3)/Lords of the Realm (1995)(Impressions)(Disk 1 of 3).adf:mod.title: high-entropy data file (7.79 bits/byte), classified as packed data
- lords of the realm (1995)(impressions)(disk 1 of 3)/Lords of the Realm (1995)(Impressions)(Disk 1 of 3).adf:conquest.map: high-entropy data file (7.70 bits/byte), classified as packed data
- lords of the realm (1995)(impressions)(disk 1 of 3)/Lords of the Realm (1995)(Impressions)(Disk 1 of 3).adf:textcast.eng: high-entropy data file (7.75 bits/byte), classified as packed data
- lords of the realm (1995)(impressions)(disk 3 of 3)/Lords of the Realm (1995)(Impressions)(Disk 3 of 3).adf:cast.bm5: high-entropy data file (7.63 bits/byte), classified as packed data
- lords of the realm (1995)(impressions)(disk 3 of 3)/Lords of the Realm (1995)(Impressions)(Disk 3 of 3).adf:thewalls.am5: high-entropy data file (7.67 bits/byte), classified as packed data
- lords of the realm (1995)(impressions)(disk 3 of 3)/Lords of the Realm (1995)(Impressions)(Disk 3 of 3).adf:port.bm5: high-entropy data file (7.64 bits/byte), classified as packed data
- lords of the realm (1995)(impressions)(disk 3 of 3)/Lords of the Realm (1995)(Impressions)(Disk 3 of 3).adf:back3a.am5: high-entropy data file (7.68 bits/byte), classified as packed data
- lords of the realm (1995)(impressions)(disk 3 of 3)/Lords of the Realm (1995)(Impressions)(Disk 3 of 3).adf:retr.bm5: high-entropy data file (7.66 bits/byte), classified as packed data
- lords of the realm (1995)(impressions)(disk 3 of 3)/Lords of the Realm (1995)(Impressions)(Disk 3 of 3).adf:back1.am5: high-entropy data file (7.65 bits/byte), classified as packed data
- lords of the realm (1995)(impressions)(disk 3 of 3)/Lords of the Realm (1995)(Impressions)(Disk 3 of 3).adf:sieg.bm5: high-entropy data file (7.65 bits/byte), classified as packed data
- lords of the realm (1995)(impressions)(disk 3 of 3)/Lords of the Realm (1995)(Impressions)(Disk 3 of 3).adf:back3.am5: high-entropy data file (7.64 bits/byte), classified as packed data
- runtime segment an address: 1867 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 107 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 75 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 30 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 37 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 16 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 25 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2084 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- lords of the realm (1995)(impressions)(disk 2 of 3)/Lords of the Realm (1995)(Impressions)(Disk 2 of 3).adf:casnames.eng: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- lords of the realm (1995)(impressions)(disk 2 of 3)/Lords of the Realm (1995)(Impressions)(Disk 2 of 3).adf:casnames.fre: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- lords of the realm (1995)(impressions)(disk 2 of 3)/Lords of the Realm (1995)(Impressions)(Disk 2 of 3).adf:casnames.ger: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- lords of the realm (1995)(impressions)(disk 2 of 3)/Lords of the Realm (1995)(Impressions)(Disk 2 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- lords of the realm (1995)(impressions)(disk 3 of 3)/Lords of the Realm (1995)(Impressions)(Disk 3 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- lords of the realm (1995)(impressions)(disk 1 of 3)/Lords of the Realm (1995)(Impressions)(Disk 1 of 3).adf:lords: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 3 never-loaded region(s), 8946 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Lords of the Realm (1995)(Impressions)(Disk 1 of 3).adf:HDInstall

- `dispatch_global_function`: Uses a global pointer to call a subroutine and then performs conditional logic based on the return value.
- `install_game_files_to_disk`: Iterates through a list of source files to create directories and copy data to a selected installation device.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `negate_and_load`: Negates a 16-bit word in a register and loads another 16-bit word from a memory location relative to the program counter into a register.
- `copy_string_with_offset`: Copies a sequence of bytes from a source buffer to a destination buffer starting at a one-byte offset from the destination pointer.
- `execute_sequential_routines`: Executes a series of subroutine calls and loops based on returned values.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_graphics_and_display`: Initializes graphics and intuition resources by creating a view, a viewport, and a screen.
- `cleanup_system_resources`: Checks various global pointers and calls associated cleanup or release routines to free resources.
- `load_rgb4_twice`: Calls a routine to load a 4-bit RGB color twice using a pointer from a global table.
- `set_rgb4_twice`: Performs bitwise manipulations on a parameter to create two values and calls a routine to set 4-bit RGB colors twice.
- `initialize_color_buffer`: Fills a local stack buffer with zero values using a loop and then calls another routine.
- `populate_color_tables`: Fills two local stack buffers by repeatedly calling a function to retrieve color data and then calls another routine.
- `fill_single_color_table`: Fills a local stack buffer with color data retrieved from a function call and then calls another routine.
- `interpolate_color_tables`: Copies a source table to a local buffer and iteratively adjusts color values to match a target table.
- `update_global_offset`: Calculates a value through multiple function calls and stores the result in a global variable.
- `prepare_signed_value`: Saves registers, loads a value from a global variable, and performs bitwise operations to set a sign bit based on a test of that value.
- `perform_signed_division_2`: Performs a signed division operation and stores the result in a global variable.
- `synchronize_display_state`: Waits for a specific vertical beam position and updates several global pointers based on a boolean flag.
- `validate_display_mode`: Queries the system for current display parameters and validates the resolution and bitplane depth against a set of supported hardware configurations.
- `initialize_structure`: Populates a structure with specific constant values and data from parameters before passing a pointer to that structure to another routine.
- `call_clear_pointer`: Library stub: loads its arguments and the intuition library base, calls ClearPointer and returns its result to the caller.
- `set_pointer`: Calls a routine to set a pointer using an address retrieved from a global table.
- `update_global_pointers`: Updates several global pointers with different sets of values based on the value of a provided 16-bit integer.
- `validate_display_mode_2`: Calls a routine to validate display parameters and branches based on the result of a word test.
- `branch_on_test_result`: Calls a routine via a pointer in a library and branches based on the result of a word test.
- `retry_until_success`: Repeatedly calls another routine in a loop until it returns a non-zero value.
- `execute_graphics_calls`: Calls two separate routines retrieved from a global table to perform graphics operations.
- `call_blt_clear`: Library stub: loads its arguments and the graphics library base, calls BltClear and returns its result to the caller.
- `execute_blt_call`: Calls a routine retrieved from a global table to perform a bitmapped block transfer.
- `validate_coordinate_range`: Checks if two sets of coordinate values fall within specific boundaries defined in a table and calls another routine if they do.
- `check_table_boundary`: Compares a word from a table with a register and performs conditional branches based on the result of the comparison and a decrement.
- `compare_range_boundary`: Compares a word from a table with a register and branches based on the comparison result.
- `validate_range_boundary`: Compares a word at a specific offset from a pointer to a register and branches if it is less than that value.
- `load_pointers_on_match`: Compares a value on the stack to one and, if they match, loads two pointers from a global table into registers.
- `call_bitmapped_function`: Calls a routine through a pointer stored in a global table.
- `conditional_rect_fill`: Checks if two sets of coordinates fall within a specific range defined by global variables and, if so, calls two graphics routines.
- `compare_global_threshold`: Saves registers, loads two words from the stack, and compares the first word against a global variable before returning.
- `compare_memory_value`: Compares a 16-bit value at a memory location with a register and branches accordingly.
- `compare_and_branch_conditionally`: Compares a 16-bit value from a memory location with a register and performs conditional or unconditional branches based on the result.
- `compare_and_branch`: Compares a 16-bit value from a memory location with a register and performs conditional branches.
- `set_bitmask_in_memory`: Calculates a memory offset and performs bitwise OR operations to set specific bits in a buffer at fixed intervals.
- `move_graphics_data`: Stores two 16-bit values from a parameter into global variables and calls a routine via a pointer in a global table.
- `set_pen_and_draw`: Calls two routines via a global table, then stores two 16-bit values from a parameter into global variables.
- `execute_two_routines`: Calls two different routines via pointers in a global table and then stores two 16-bit values from a parameter into global variables.
- `process_parameter_segments`: Calls another routine multiple times using various bitwise masks and segments of the provided parameters.
- `update_boundary_values`: Stores two 16-bit values and two incremented 16-bit values from parameters into global variables.
- `initialize_resource_metadata_structure`: Calculates memory offsets and dimensions for a data structure based on bitmask flags and populates it with parsed values.
- `traverse_and_clear_list`: Checks a value at an offset from a pointer, calls a routine via a pointer, clears that value, and may recursively call itself based on a bit test.
- `calculate_bitmap_offset`: Performs complex arithmetic using fixed-point multiplication and global values to calculate and store a series of related 32-bit values, then calls a routine via a pointer in a global table.
- `accumulate_string_values`: Iterates through a null-terminated string and accumulates a running sum of values, skipping carriage return and newline characters.
- `format_signed_string`: Converts a signed value into a null-terminated string of numeric characters and copies it to a destination.
- `accumulate_string_sums`: Calls two routines to calculate a running sum and a maximum segment sum from string data.
- `count_string_segments`: Iterates through a byte string to calculate a sum based on character values or a global table, returning the maximum segment sum found.
- `process_string_buffer_and_graphics`: Iterates through a string to calculate segment sums and lengths while performing conditional bitmapped area fills and coordinate-based rectangle updates.
- `process_masked_data`: Calls several routines to transform a string buffer, unpack masked data, perform XOR operations on memory blocks, and fetch masked values.
- `transform_string_buffer`: Iterates through a source data stream to perform character substitution and expansion within a destination buffer.
- `unpack_masked_data`: Decodes a sequence of values from a global structure by reading bytes and using their high bits to determine how many subsequent bytes to copy into a destination buffer.
- `xor_memory_blocks`: Iterates through a series of memory blocks defined by a global count and performs a bitwise XOR operation between pairs of blocks.
- `fetch_masked_value`: Increments a pointer within a global structure if a condition is met, otherwise calls another routine, and returns a value with the lower byte masked out.
- `retrieve_dos_data`: Calls two routines via a global table, checks a return value, and if non-zero, calls two more routines and returns a value from a memory offset.
- `match_and_update_table_entries`: Compares a string against a pattern using wildcard matching and updates a table of entries by setting specific status values based on provided bit flags.
- `clear_pointer_field`: Checks if a 32-bit value at an offset in a parameter is non-zero, then calls a routine via a global table and clears that value.
- `test_table_entry`: Checks a longword at an offset from a pointer and branches based on whether it is zero or a specific small value.
- `clear_registers_and_branch`: Clears two data registers and performs a relative branch.
- `traverse_sorted_table`: Recursively compares adjacent elements in a table of shortwords to find a specific value or condition.
- `traverse_sorted_table_2`: Iterates through a table of shortwords to compare adjacent elements and performs recursive calls based on the results.
- `restore_stack_data`: Restores four longwords from the stack into registers and performs a short branch.
- `compare_strings_with_wildcards`: Compares two strings, supporting a wildcard character that allows for character skipping or matching during the comparison process.
- `move_long_register`: Copies a 32-bit value from one data register to another.
- `enumerate_directory_contents`: Uses system calls to iterate through a directory and copies string data into a provided buffer.
- `move_word_register`: Copies a 16-bit value from one data register to another.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to -(a7).
- `check_directory_status`: Checks the result of a directory operation and calls two other routines based on the return value.
- `copy_and_check_directory`: Copies a small amount of data from a global variable to a local buffer and checks a filesystem status.
- `clear_register_and_branch`: Clears a data register and performs a relative branch.
- `update_structure_member`: Writes a constant value to an offset within a structure and returns a value from another offset in that same structure.
- `manage_structure_lifecycle`: Allocates, initializes, and iterates through a structure while performing conditional cleanup and field resets.
- `compare_handler_results`: Executes several subroutines using parameters and a global variable, then returns the result of a comparison between two integer values.
- `get_current_directory`: Calls a routine to get the current directory and stores the result in a global variable.
- `release_directory_lock`: Calls two routines to perform cleanup or unlock operations.
- `copy_bytes_from_list`: Iterates through nested loops to extract bytes from a structure list via a pointer and stores them into a destination buffer until a specific value is encountered.
- `process_stream_buffer`: Iterates through a loop to transfer bytes from a source pointer to a destination pointer stored within a structure.
- `store_upper_word`: Shifts a passed value and stores the upper sixteen bits into a global variable.
- `calculate_offset_value`: Calls another routine, adds a constant to the result, and stores the upper sixteen bits of the sum into a global variable.
- `transfer_pointer_data`: Restores four longwords from the stack to registers and performs a short branch to another location.
- `copy_memory_block`: Copies a block of memory from a source to a destination using a counter.
- `compare_memory_buffers`: Compares two memory buffers byte by byte and returns zero if they are identical, or a comparison result if they differ.
- `compare_memory_with_limit`: Compares two memory buffers up to a specified length and returns a comparison result if a difference is found.
- `search_string_recursively`: Searches for a substring within a string or a global string and modifies a global pointer based on the result.
- `find_substring`: Searches for the first occurrence of a null-terminated string within another null-terminated string.
- `find_substring_index`: Searches for the first occurrence of a second string within a first string.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `initialize_installation_environment`: Uses system library calls to manage file paths, windows, and command strings while updating various global pointers and status flags.
- `move_register_to_register`: copies the contents of one data register to another.
- `tokenize_command_string`: Parses a string into individual tokens by splitting it based on whitespace and handling quoted substrings.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_with_limit`: The routine finds the end of the first string, then copies characters from the second string into the first, decrementing a counter for each character copied.
- `open_window_and_get_icon`: Opens a window using a library and retrieves icon information to store in a provided structure.
- `copy_data_register`: Copies the contents of one data register to another data register.
- `process_structure_list`: Validates a pointer, traverses a list of structures to check specific bit flags, and updates a structure with new data or returns a value from a pointer.
- `parse_access_mode`: Calls a routine to parse a string for access flags and then updates a table with the results.
- `parse_file_access_mode`: Parses a string to determine file access flags and stores the resulting mode and index into a table.
- `search_and_clear_table`: Iterates through a table of structures to find an entry where a specific field is zero, then clears the remaining fields of that entry.
- `copy_string_to_null`: Locates the null terminator of the first string parameter by incrementing a pointer until a zero byte is found.
- `copy_string_or_pad`: The routine copies characters from a source string to a destination buffer up to a specified count or until the source ends.
- `forward_single_argument_call`: Passes its single argument to another routine, discards the result, and returns.
- `update_file_table`: Iterates through a table to find an available slot and updates it with file information using various system calls.
- `read_file_from_table`: Checks a table for a valid entry and attempts to perform a read operation using a system call.
- `multiply_32bit_fixed_point`: The routine takes two 32-bit fixed-point values from registers and returns a 32-bit fixed-point product.
- `allocate_structure_and_assign`: Allocates a structure via a system call and populates its fields with provided parameters and system-derived values.
- `reset_structure_fields`: Checks a value at an offset from a pointer, then sets a byte to all ones and a long to all ones before calling two other routines.
- `initialize_data_structure`: If the first parameter is non-zero, it calls another routine to obtain a pointer and then populates that pointer's memory with a byte, a word, and a long.
- `clear_structure_fields`: If the provided pointer is non-zero, it sets a byte and two long values at specific offsets to all ones and calls another routine.
- `calculate_signed_division`: Performs a division on two values and applies the correct sign to the result based on the input signs.
- `perform_signed_division`: Performs a division operation and applies the correct sign to the result based on the input values.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `perform_64bit_division`: Performs division and modulo operations on 64-bit values split across two 32-bit registers.
- `perform_bit_rotation`: Performs a series of arithmetic shifts and subtractions on two values within a loop controlled by a counter.
- `clear_and_format_structure`: Clears a structure, converts a numeric value into a five-digit string, and performs several conditional cleanup operations.
- `tiny_addq_addq`: Small routine of 2 instructions that adds 2 to a5, then adds 8 to d0.
- `iterate_structure_array`: Iterates through an array of structures by incrementing a pointer by a fixed offset and calling a routine on each element.
- `process_buffered_stream_data`: Manages a stateful data stream by reading or writing bytes through a pointer stored within a structure.
- `iterate_structure_list`: Traverses a list of structures in memory by incrementing a pointer by a fixed offset.
- `update_handler_parameter`: Updates a structure with a new value and a pointer, then performs bitwise operations on a field within that structure.
- `update_linked_list`: Traverses a linked list to find a specific node and then modifies the list structure by updating a pointer.
- `load_and_branch_to_address`: Saves registers to the stack, loads a value from a global variable into a register, and branches to a new location.
- `clear_linked_list`: Iterates through a linked list, calling another routine for each element, and finally sets a global pointer to zero.
- `register_handler_pointer`: Checks a parameter and, if non-zero, updates a global pointer and a table with a new routine and a parameter before branching to a new location.
- `copy_string_simple`: The routine copies characters from a source string to a destination buffer until the null terminator is copied.
- `calculate_string_length`: The routine receives a pointer to a zero-terminated byte string and advances a pointer until it finds the terminating zero byte.
- `check_interactive_status`: Validates an index against a global range and a table, then calls another routine to check if the device is interactive.
- `validate_and_read_file_entry`: Checks if an index is within bounds and points to a valid entry in a table before attempting to read data.
- `delete_file_with_error_check`: Attempts to delete a file and updates a global variable with an error status if the operation fails.
- `write_data_with_validation`: Validates an index against a table, checks specific bit flags, and performs a write operation or handles an error.
- `traverse_linked_list`: Iterates through a linked list of pointers, executing a function stored at an offset within each node.
- `cleanup_system_resources_2`: Iterates through a table to call routines, performs several conditional calls to functions via a global pointer, and executes various cleanup tasks based on global flags.
- `nop_padding_sequence`: This routine is empty and performs no operations.
- `close_file_entry`: Checks if an index is within valid bounds and then clears a pointer in a table.
- `close_file_and_clear_entry`: Closes a file if certain conditions are met and sets a table entry to zero.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.

### icon.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `modify_library_status`: As library code, it either inserts a structure into a linked list or sets a specific bit in a status byte.
- `set_library_flag`: As library code, it sets the eighth bit of a byte at a specific offset from a pointer.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `update_library_status`: As library code, it copies two values into a structure, calls another routine, and updates global variables based on the result.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `clear_register_d0`: As library code, it performs a bitwise AND operation on register d0 with zero.
- `manage_icon_chain`: This library routine calls another routine to manage an icon chain.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `invoke_icon_structure_update`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `manage_icon_chain_2`: This library routine calls another routine to manage an icon chain.
- `recurse_library_function`: As library code, it calls another routine immediately.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_icon_structure_update_2`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `execute_empty_library_call`: This library code routine is empty.
- `trigger_infinite_recursion`: This routine performs a direct call to itself without any conditional branching or termination logic.
- `call_other_routine`: This library code calls another routine and then returns.
- `execute_checksum_library_call`: As library code, this routine calls another routine.
- `convert_string_to_numeric`: This routine acts as a wrapper that invokes a function to parse a sequence of characters into a numeric value and store it in a destination buffer.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `clear_data_register_byte`: Performs a logical OR operation on a byte within a data register using a zero value.
- `update_icon_attributes`: As library code, this routine calls multiple other routines and performs a block copy of data into a structure at a specific offset.
- `copy_and_assign_data`: As library code, this routine copies a block of data from one location to another using a loop and then performs several individual word and longword assignments to a destination structure.
- `return_zero_stub`: The routine is an empty stub that immediately returns the constant zero.
- `copy_icon_data_to_stack`: This library routine copies ten longwords from an offset within a structure pointed to by the third argument into a local array on the stack, then copies several additional fields from the same structure into other stack slots and finally calls another routine with the first two arguments and a pointer to a stack-resident header.
- `update_icon_resource_pointers`: Updates an icon's internal attribute structure by allocating and assigning new memory pointers for its associated resources.
- `sync_icon_attribute_chain`: Updates an icon's attribute structure and manages a linked list or array of related icon data.
- `update_icon_data_2`: Library code that updates several offsets in a structure by calling another routine and storing the results.
- `process_icon_changes`: As library code, it executes a loop that calls routines to update a pointer and a counter, then performs conditional logic to update a global value.
- `clear_register_and_branch_2`: This library routine clears a register and performs an unconditional branch to another location.
- `empty_routine`: The routine contains four instructions and performs no observable work; it ends immediately with a return.
- `dispatch_icon_callback_chain`: Iterates through a structured set of icon data and executes a sequence of callback functions stored at various offsets within a data structure.
- `execute_icon_callback_chain`: Iterates through a series of icon-related callback pointers and executes them sequentially.
- `execute_icon_callbacks`: Library code that checks multiple pointers in a structure and executes a callback routine for each valid entry.
- `iterate_and_callback`: As library code, this routine iterates through a null-terminated table of pointers, executes a callback for each entry, and performs conditional logic based on the results of multiple function calls.
- `iterate_icon_table_entries`: Library code that walks a null-terminated table of pointers, calling a routine for each entry, updating a counter, and invoking a callback while potentially processing a secondary entry.
- `prepare_list_traversal`: As library code, this routine pushes a pointer onto the stack, loads a value into a register, increments that value, and branches to another location.
- `iterate_linked_list_nodes`: The routine traverses a linked list structure by following pointers stored at an offset from the current node.
- `increment_param_and_recurse`: Library code that calls another routine with the same first argument and the second argument increased by a constant, then returns.
- `execute_library_chain`: As library code, it calls multiple library functions using specific offsets to process data through a sequence of conditional checks.
- `execute_icon_update`: As library code, it pushes registers to the stack and calls another routine, then returns a value based on the result of that call.
- `return_zero_value`: This library routine is empty and simply returns zero to the caller.
- `update_icon_structure`: This library routine decrements a counter through a pointer and, if the counter reaches zero or is less than one, calls itself recursively before storing two values into a table using an index and a base address.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, then uses the counter value as an index into a table pointed to by a global variable.
- `manage_icon_chain_3`: As library code, this routine calls several other routines to manage data structures and handles conditional logic based on their return values.
- `branch_to_offset`: Library code that performs a short branch to a specific offset.
- `call_library_routine_and_return_zero`: Library code that calls another routine and then returns zero.
- `perform_incremental_recursion`: Library code that performs a recursive call with an incremented parameter until the callee stops the recursion.
- `verify_string_checksum`: Library code that loads a pointer from a table, calls a routine to calculate a string checksum, and performs conditional branching based on the result.
- `search_for_byte`: As library code, it calls another routine, checks a result, and then enters a loop to find a specific byte value in a memory range.
- `load_and_test_pointer`: Library code that loads a pointer from a table into a register and performs a conditional branch based on its value.
- `execute_checksum_routine`: This library code saves several registers to the stack, retrieves two values from the stack, pushes one of those values onto the stack, calls another routine, and then performs a short branch.
- `call_subroutine_and_calculate_difference`: Library code that pushes a pointer onto the stack, calls another routine, and then calculates the difference between the returned value and a value in a register.
- `iterate_with_conditional_check`: As library code, it uses a loop to iterate through a sequence, performing comparisons and increments based on returned values.
- `increment_pointer_value`: This library routine checks a register and, if non-zero, increments its value and branches to another location.
- `return_zero_if_empty`: Library code that checks if a register is zero and returns zero, otherwise restores registers from the stack and returns.
- `parse_numeric_string_to_buffer`: Converts a sequence of characters into a numeric value and writes the result into a destination buffer.
- `parse_numeric_data`: Library code that pushes parameters onto the stack, calls a routine to parse a numeric string, and performs conditional branching based on the result.
- `branch_with_zero_register`: Library code that clears a register and performs an unconditional branch.
- `parse_numeric_string`: As library code, it converts a sequence of characters into a numeric value and performs subsequent checks on the resulting data.
- `calculate_value_difference`: This library routine compares a value from the stack against two constants and returns either a difference or a specific constant value.
- `return_all_ones`: Library code that returns a value consisting of all bits set to one.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: The library routine forwards a set of four parameters to another routine, checks the returned value, and if non-zero calls the same routine again with a modified argument list.
- `update_icon_data`: As library code, this routine calls two library functions and, if the first returns non-zero, updates a value in a memory location and recursively calls itself with an incremented pointer.
- `execute_library_call`: As library code, this routine pushes several values onto the stack, calls a subroutine, and then performs conditional logic based on the result before returning.
- `tiny_move_rts`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `call_library_function_with_stack_args`: As library code, it retrieves three values from the stack, calls a subroutine, and returns a result based on a comparison.
- `return_empty`: As library code, this routine is empty.
- `rotate_32bit_value_2`: Library code that performs a bitwise rotation on a 32-bit value.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `compare_and_decrement_pointer`: As library code, it retrieves a pointer and a byte value from the stack, compares a byte at the pointer location to the value, and decrements the pointer if they are equal.
- `clear_register_and_branch_3`: This library routine clears a register and performs an unconditional branch.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `search_null_byte`: Library code that iterates through a memory range to find a zero byte and adjusts a pointer accordingly.
- `check_byte_equality`: As library code, it compares a byte from one memory location to a byte from another and returns zero if they are equal and the byte is non-zero.
- `set_register_constant`: This library routine sets a register to one or negative one based on a comparison and returns.
- `branch_on_byte_value`: As library code, it checks a byte at a pointer location and branches to different locations based on whether the byte is zero or within a specific range.
- `call_copy_routine`: This library routine calls another routine.
- `copy_limited_bytes`: Library code that copies bytes from one location to another using a counter and a loop.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `process_icon_request`: Library code that validates a parameter, calls another routine to check a return value, and if successful, performs further calls using a value from a pointer.
- `load_registers_from_stack`: As library code, it saves registers to the stack and loads three values from stack offsets into registers before branching.
- `verify_linked_list_nodes`: As library code, it repeatedly calls a routine and checks the results to traverse a structure, returning one if a specific offset in a node is zero.
- `return_zero`: As library code, this routine returns zero.
- `calculate_and_store_product`: As library code, it performs a series of calculations involving multiple parameters and stores a computed 32-bit value into a structure.
- `return_negative_one`: As library code, this routine returns a value of all bits set.
- `save_registers_and_branch`: As library code, it saves several registers to the stack, loads three values into registers, and performs a short branch.
- `call_icon_propagation`: This library routine pushes several parameters onto the stack, calls another routine, and then performs conditional branching based on the result and a register value.
- `propagate_icon_updates`: As library code, it performs a nested loop that calls routines to update values in a structure, returning error codes if specific conditions are not met.
- `return_maximum_value`: This library routine returns a value consisting of all bits set to one.

### Lords of the Realm (1995)(Impressions)(Disk 1 of 3).adf:lords

- `free_blocks_and_restore_patch`: Calls two unnamed routines, loops to free five memory blocks, and restores a saved word in a table if a specific masked pattern is matched.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `process_data_loop`: Executes a series of nested loops that perform arithmetic additions to a memory location based on values returned by other routines.
- `load_address_and_constant`: Loads a relative address into a register and moves a small constant into another register.
- `rotate_32bit_integer`: Performs a bitwise rotation on a 32-bit value by shifting bits through the carry flag.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Lords of the Realm (1995)(Impressions)(Disk 1 of 3).adf:message

- `initialize_message_parsing_environment`: Allocates memory and initializes internal structures for string processing.
- `dispatch_message_callbacks`: Stores a parameter into a memory location and conditionally executes several function pointers retrieved from a structure and a global variable.
- `conditionally_execute_callback`: Compares a value derived from the stack pointer against a value in a structure and, if the comparison fails, pushes a pointer from that structure onto the stack and calls another routine.
- `iterate_and_call_handlers`: The routine iterates through a list of function pointers stored in a table, executing each one until a null pointer is encountered.
- `call_indirect_handler_return_constant`: The routine calls an indirect handler through a pointer stored in a global variable, passing the address of a global variable, then returns a constant value.
- `invoke_global_callback`: The routine calls a function pointer stored in a global table, passing the address of a global variable as argument.
- `calculate_string_length_2`: Iterates through a null-terminated string to call another routine for each character and returns the total length of the string.
- `traverse_and_free_list`: The routine traverses a linked list of pointers, calling a function pointer stored in a global variable for each element, and finally clears the list head.
- `copy_string_to_buffer`: The routine copies bytes from a source pointer in A1 to a destination pointer in A0 until a zero byte is encountered, writing each byte to the destination and advancing both pointers.
- `initialize_message_structure`: Initializes a data structure by setting multiple internal pointers and status flags.
- `move_data_to_address_register`: The routine moves a value from a data register to another data register and loads a value from a stack offset into an address register.
- `initialize_message_structure_2`: Checks a bit flag in a structure and, if not set, calls a routine to allocate memory.
- `execute_callback_chains`: Iterates through two separate linked lists of pointers stored within a structure.
- `process_callback_chain_2`: Iterates through a sequence of function pointers retrieved from a structure and a global pointer.
- `test_bit_and_load_stack`: The routine tests a bit in a data register and then performs several stack-based move operations to manipulate data registers and stack contents.
- `execute_callback_chain_3`: Executes a sequence of function pointers retrieved from a structure and a global variable based on specific conditional logic.
- `trigger_message_callbacks`: Stores a parameter into a memory location and executes a series of function pointers retrieved from a structure and a global variable.
- `parse_and_tokenize_string`: Parses a string to count words and identifies quoted segments or special escape sequences.
- `execute_conditional_callback`: The routine checks if two values at specific offsets from a pointer are non-zero, and if so, calls a function pointer stored in a global variable.
- `dispatch_message_callback_chain`: Executes a function pointer retrieved from a global variable and processes a sequence of conditional calls based on values found in a structure.
- `validate_list_index`: The routine checks if a value is within a specific range and then traverses a linked list a specified number of times, updating status values in a structure based on the outcome.
- `process_callback_chain`: Retrieves a function pointer from a global variable and executes a sequence of conditional calls based on a structure.
- `find_byte_in_buffer`: The routine searches for a specific byte value within a memory buffer of a given length.
- `unlink_linked_list_node`: Traverses a linked list to find a memory block containing a specific address, then modifies the block's metadata and removes the node from the list.
- `allocate_message_buffer`: Searches a linked list of memory blocks to find a segment that can accommodate a requested size, potentially splitting an existing block into two smaller parts.
- `execute_callback_chain`: Iterates through a linked list of pointers and executes a function located at the second word of each entry.
- `update_text_input_buffer`: Processes a byte from a register into a buffer managed by a structure, updating status flags, length counters, and buffer pointers.
- `process_ride_object`: Walks a linked list via a global pointer when the first argument is zero and recursively processes each node, returning a non-zero flag if any call returns non-zero.
- `return_from_subroutine_2`: The routine is empty and performs no operations before returning.
- `call_another_routine`: The routine calls another routine and then returns.

### Lords of the Realm (1995)(Impressions)(Disk 2 of 3).adf:game.bm5

- `initialize_engine_subsystems`: Performs a comprehensive system startup by initializing hardware polling, pattern tables, and BCD timers.
- `initialize_system_subsystems`: Performs a comprehensive initialization sequence including hardware polling configuration, pattern table generation, BCD timer setup, and pseudo-random seed transformation.
- `find_longest_sequence`: The routine scans a table to find the maximum length of consecutive identical bytes and stores the result in a global variable.
- `populate_pattern_lookup_table`: Performs nested loops to calculate and store derived values into a destination table based on a source byte table.
- `fill_pattern_table`: The routine performs nested loops to combine specific bytes from a source table and store the result into a destination table.
- `update_bcd_counter_and_initialize_struct`: Performs multi-byte BCD (Binary Coded Decimal) addition on a four-byte value stored in memory and subsequently populates a data structure with several constant values and calculated offsets.
- `update_timer_and_struct_fields`: Performs BCD arithmetic on global variables and populates a data structure with various constant and calculated values.
- `fill_pattern_table_2`: Performs nested loops to combine specific bytes from a source table and store the result into a destination table.
- `initialize_data_structure_2`: The routine waits for a specific bit in a byte to clear and then populates a large structure with various constant and calculated values.
- `transform_pseudo_random_seed`: Takes a value from a pseudo-random generator, performs a modulo operation and bitwise concatenation, and stores the resulting bit-shifted value into a global variable.
- `setup_display_parameters`: The routine calculates a value based on global variables and populates a structure with specific constants and calculated offsets.
- `initialize_data_structure_3`: The routine waits for a specific bit in a memory location to clear before writing several constant values and two calculated integers into a structure pointed to by a register.
- `setup_display_parameters_2`: Calculates a value based on global variables and populates a structure with specific constants and calculated offsets.
- `spread_pixel_values`: The routine reads values from a source table at specific intervals and writes the combined result into a destination table.
- `enter_infinite_loop`: The routine enters an infinite loop.
- `execute_infinite_loop_2`: The routine enters an infinite loop that prevents further execution.
- `parse_string_data`: Iterates through a string, performing character substitutions and calculating values based on character offsets to store in a table.
- `enter_infinite_loop_2`: Sets a global variable to one, waits for a hardware bit to clear, writes a value to a memory location, and then sets the global variable to zero.
- `apply_linear_feedback`: Performs a series of multiplications and conditional XOR operations on a global variable to generate a pseudo-random value.
- `read_joystick_state`: Reads bits from a hardware register to determine the state of a joystick and updates a global variable with bitwise flags.
- `reset_audio_tables`: Searches through a data structure to find specific values and then clears several memory locations and global variables.
- `clear_hardware_registers`: Sets several global variables and hardware-related memory locations to zero or a fixed value.
- `increment_and_branch_logic`: Increments a byte in a structure, compares it to another value, and branches to different routines based on the result.
- `call_subroutine_and_branch`: Executes a subroutine and then performs a short branch to another location.
- `update_clamped_attribute_value`: Performs arithmetic updates on a value using command bytes to apply additions, subtractions, or table-based lookups.
- `initialize_and_update_structure`: Sets specific values in a structure, updates several fields from global memory, and performs arithmetic on a counter and byte flags.
- `update_entity_data`: Updates a structure by writing several multi-byte values from global memory and performing arithmetic on a counter and byte flags.
- `parse_command_parameter_data`: Parses a command byte stream to extract type identifiers and associated numeric values.
- `apply_command_parameter_updates`: Updates entity properties and state variables by parsing command bytes and applying arithmetic or bitwise operations to data structures.
- `update_entity_state_from_command`: Iterates through a table to find a matching entry and then updates various entity properties based on a command byte.
- `process_command_byte_operations`: Parses a command byte to perform various arithmetic, bitwise, and clamping operations on data structures.
- `process_command_byte_data`: Parses a command byte from a data structure to perform various arithmetic, bitwise, and clamping operations on associated values.
- `update_entity_state_and_timers`: Modifies a structure by adjusting offset values, clearing specific byte flags, and decrementing a counter.
- `process_command_byte_logic`: Evaluates a command byte from a structure to perform various arithmetic updates, including clamped additions, subtractions, or table-based lookups.
- `copy_word_to_table`: Copies a word from a source address plus an offset into a destination address plus an offset.
- `calculate_scaled_index`: Divides a byte from a table by three, swaps the high and low bytes, and performs conditional branches based on the result.
- `search_and_store_word_2`: Iterates through a table to find a word less than or equal to a comparison value, then stores a calculated word from that table into a destination.
- `copy_word_to_offset_table`: Copies a word from a memory location to a destination address plus a specific offset and a variable index.
- `search_and_store_word`: Iterates through a table to find a value less than or equal to a comparison value, then stores a word from that table into a destination.
- `store_word_to_table`: Stores a word from a register into a destination address plus an offset.
- `decrement_and_update_table_entry`: Checks a flag in a structure, performs a bitwise AND between two bytes, and subtracts the result from a word in another structure.
- `increment_and_clamp_value`: Adds a bitwise masked value to a word in a structure and clamps the result to a maximum threshold.
- `update_table_value`: Iterates through a table to find a value based on an offset and compares it against a threshold.
- `clear_memory_word`: Sets a word at a specific offset within a memory block to zero.
- `update_flag_byte`: Moves a byte from a structure to a register, and if it is not zero, stores that byte into another location in the structure and clears the original byte.
- `update_value_from_table_2`: Modifies a word in a structure by adding or subtracting a byte based on a flag, clamps it to a minimum value, and then searches a table to update a destination word.
- `update_value_from_table`: Subtracts a value from a variable, checks bounds, and then performs a search through a table to update a destination word.
- `calculate_scaled_offset_2`: Computes a scaled value using a table or arithmetic and stores the result in a destination memory location.
- `calculate_scaled_offset`: Computes a value by scaling a byte from a table or performing arithmetic based on a mode, then adds this to a value and stores the result in a destination word.
- `update_difference_value`: Tests a byte at a specific offset and, if negative, stores a value in a register and subtracts a byte from it before branching.
- `copy_register_byte`: Copies a byte from a register to another register and performs an unconditional branch.
- `calculate_offset_increment`: Calculates a value by multiplying a masked byte by a value from a structure, shifts it, and adds it to a word if a sign flag is not set.
- `calculate_difference_and_offset`: Calculates the difference between two values to store in a table and updates a byte using a bit-shifted value from another byte.
- `adjust_clamped_byte`: Modifies a byte in a structure by adding or subtracting a value derived from another byte, clamps the result between zero and sixty-four, and stores the result in a table.
- `update_clamped_value_2`: Increments or decrements a byte in a structure and clamps the result within a specific range before storing it in a table.
- `update_clamped_value`: Modifies bits within a byte of a structure and calculates a new value using scaled arithmetic and a base byte.
- `store_subtracted_byte`: Tests a byte at a specific offset and, if negative, stores a register value and subtracts a byte from it before branching.
- `move_byte_and_branch`: Copies a byte from one data register to another and performs an unconditional short branch.
- `calculate_scaled_difference`: Computes a value by multiplying a masked byte by a value from a structure, shifts it, and adds it to a word if a sign flag is not set.
- `clamp_and_update_value`: Calculates a difference between two values, clamps the result between zero and sixty-four, and stores it in a table and a structure.
- `adjust_structure_offset`: Checks a byte in a structure, and if non-zero, modifies a word at a specific offset by subtracting a bit-shifted version of that byte and adding the result to a long value.
- `set_status_flag`: Sets a two-byte value at a specific offset within a structure to one.
- `increment_clamped_byte`: Adds a bit-shifted byte to a value in a structure, clamps the result to a maximum of sixty-four, and stores the result in a table using an offset.
- `decrement_and_store_value`: Subtracts a masked byte from a value in a structure, clamps the result to zero, and stores it in a table and the structure.
- `initialize_structure_fields`: Decrements a byte from one structure and stores it in another, while also clearing two adjacent bytes.
- `limit_and_store_byte`: Clamps a byte value to a maximum of sixty-four and stores the result in both a structure and a table.
- `calculate_scaled_byte`: Calculates a value by multiplying a bit-shifted byte by ten, adds a masked byte to it, and stores the result in a structure if it is below sixty-three.
- `store_byte_with_range_check`: Checks if a byte from a structure is within a specific range.
- `dispatch_command_byte_operations`: Parses a command byte from a data structure to execute specific arithmetic, bitwise, or clamping operations.
- `process_masked_command_byte`: Reads a command byte from a structure and executes different bitwise operations, arithmetic updates, or nibble manipulations based on its value.
- `update_global_bit`: Modifies a global variable by setting or clearing a specific bit based on a masked byte from a structure.
- `mask_and_update_byte`: Clears the lower nibble of a byte in a structure and replaces it with a masked value from another source.
- `update_nibble_in_structure`: Performs a bitwise operation to replace the lower four bits of a byte in a structure with a masked value.
- `store_masked_byte`: Stores the lower four bits of a byte from a structure into another location within a structure.
- `decrement_and_store_byte`: Checks a byte in a structure, and if it is non-zero, performs a series of checks and decrements a counter before storing a byte from a structure into a table.
- `store_byte_and_branch`: Stores a byte from a register into a structure and branches to another location.
- `shift_and_store_byte`: Shifts a two-byte value right by four bits and stores the resulting byte into a structure.
- `combine_byte_nibbles`: Masks the lower nibble of a byte in a structure and then replaces it with the upper nibble of another byte from the same structure.
- `update_data_on_modulo`: Checks if a byte in a structure is non-zero and if another byte is divisible by it, then performs several multi-word transfers to a destination structure.
- `mask_and_branch`: Tests a byte in a structure, and if it is non-zero, loads a byte from another structure into a register with a bitmask applied before branching.
- `branch_on_byte_test`: Tests a byte in a structure and, if non-zero, loads a masked byte from another structure into a register before branching.
- `clear_structure_fields_2`: Compares a masked byte from a structure to a byte in another structure and, if they match, clears a byte and a word in a structure.
- `compare_and_branch_2`: Loads a byte from a structure, masks it, compares it to another byte, and if they match and a word is non-zero, pushes a register onto the stack and branches.
- `increment_masked_byte`: Checks two bytes in a structure and, if both are zero, increments a masked byte from another source and stores it in the structure.
- `update_masked_nibble`: Checks a byte in a structure and, if zero, extracts a nibble from another source, shifts it, and merges it into a byte in that source.
- `update_buffer_index_2`: Modifies a byte in a structure using a lookup table and increments a pointer within a buffer if a specific bit is set.
- `loop_compare_byte_three_times`: Performs a fixed-count loop that executes three times, comparing a byte at a specific offset within a memory structure to itself.
- `configure_polling_parameters`: Updates a structure with various word values and sets a global variable based on the current state of that variable.
- `set_audio_registers`: Sets two hardware registers to specific values after waiting for a change.
- `wait_for_value_change`: Sets a value in a structure and enters multiple loops that poll a word in memory until specific bitwise patterns change.
- `wait_for_bit_change`: Sets a value in a structure and enters loops that poll a word in memory until the bits at a specific offset change.
- `process_audio_sequence`: Updates a global counter and iterates through a table to store incrementing values into a series of short integers.
- `store_byte_to_global`: Stores a single byte into a global variable and conditionally calls another routine if the value is zero.

### Lords of the Realm (1995)(Impressions)(Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `save_registers_and_initialize`: Saves several registers to the stack and initializes several registers with values from other registers or memory.
- `initialize_file_system_environment`: Retrieves the current directory and duplicates a file lock to prepare for file operations.
- `process_system_data`: Performs a conditional branch, calls a routine, and then moves a value into a location pointed to by a structure while performing comparisons on a global variable.
- `release_system_resources_2`: Executes a series of function calls via pointers stored in a structure to perform cleanup operations.
- `release_system_resources`: Performs several operations to unlock files, close libraries, forbid execution, free memory, and reply to a message.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `verify_function_pointers`: Calls several functions via pointers stored in a structure and handles errors if any of the calls return zero.
- `execute_function_pointers`: Calls several functions via pointers stored in a structure and updates a global variable with the results.
- `update_state_and_bounds`: Updates various global variables and state flags based on a primary status value.
- `update_minimum_value`: Compares a provided value against a global variable and updates the global variable if the provided value is smaller.
- `tiny_cmp`: Small routine of 1 instruction that compares d0 with d2.
- `store_register_to_global`: Stores the contents of a 16-bit register into a global variable if a specific condition is met.
- `clamp_global_values`: Clamps two global variables to specific minimum and maximum ranges and returns the previous value of the second global.
- `query_grid_cell_attributes`: Checks if a coordinate pair falls within a specific grid boundary and retrieves attribute data from a two-dimensional table.
- `iterate_entity_grid_table`: Iterates through a two-dimensional grid of data using nested loops to perform conditional checks on entity attributes.
- `update_entity_position`: Updates coordinate values by comparing a target value against a set of offsets in a table.
- `update_entity_state_and_coordinates`: Updates a structure's internal counters and coordinate values based on conditional checks of stack-based parameters.
- `execute_flagged_tasks`: Sets a global variable to one, calls two different routines, and then resets the global variable to zero.
- `set_global_flag`: Sets a global variable to one and returns an undefined value.
- `initialize_and_process_game_state`: Initializes the file system environment and copies a block of data from one memory location to another.
- `execute_complex_sequence`: Clears two global variables, performs several subroutine calls, and then calls another routine with a concatenated value.
- `initialize_global_variables`: Sets two global variables to zero and another to one before calling two other routines.
- `retry_function_calls`: Repeatedly calls a routine up to nine times, updating a global variable with the result each time, and returns one if all calls failed.
- `initialize_memory_buffer`: Allocates and initializes a memory buffer by calculating offsets from a base pointer and a stack-relative value.
- `process_conditional_flag_sequence`: Calls several routines and then repeatedly executes a specific routine as long as a global variable remains non-zero.
- `compare_string_patterns`: Compares two strings by evaluating characters through a lookup table and checking for specific wildcard or termination characters.
- `check_and_restore_registers`: Tests a byte at an offset from a pointer and branches based on the result.
- `call_stored_function`: Checks a pointer within a structure, calls a function pointer retrieved from a global offset, and clears the pointer in the structure.
- `update_global_via_loop`: Performs a loop up to thirty times to modify a global variable using bitwise shifts and logical operations.
- `update_global_with_modulo`: Updates a global variable by adding a masked version of a returned value and applies a modulo operation if the result exceeds a threshold.
- `process_bit_sequence`: Performs bitwise operations on a value in a loop and stores the resulting lower seven bits into a global variable.
- `loop_with_conditional_exit`: Executes nested loops that call a function pointer and another routine, exiting early if a global variable becomes non-zero.
- `execute_sequence_and_hang`: Calls several routines with various arguments and then enters an infinite loop.
- `process_input_character`: Updates a global counter based on a character input and executes specific subroutines or modifies state depending on the character's value.
- `return_zero_2`: The routine is empty and returns zero.
- `clamp_global_value`: Updates a global variable to match another value if it is smaller, ensuring the result is not negative.
- `update_array_state`: Checks a condition based on two global variables and a function return value, then conditionally copies data between a global variable and an array while updating several other global variables.
- `update_array_and_state`: Conditionally copies a global variable into an array at a specific index and updates several global variables based on function return values.
- `update_state_with_offset`: Increments a global variable and updates another based on a comparison with a value returned from a routine.
- `sync_state_flags`: Compares global variables against bitmasks to update several other global variables and state flags.
- `increment_multiple_counters`: Increments several global variables by two and wraps them around specific maximum values.
- `copy_string_to_buffer_2`: Calculates a destination address using a global variable and parameters, fills it with spaces, and then copies characters from a source string.
- `copy_memory_with_offset`: Copies a block of bytes from a source address to a destination address, where the source address is calculated using an offset from a global variable.
- `copy_memory_block_2`: Copies a block of bytes from a source address to a destination address using a specified count.
- `calculate_table_offset`: Calculates an index into a global table using a parameter and returns a value derived from two bytes at that location plus a constant.
- `calculate_offset_and_store_sum`: Calculates an offset by scaling an input and adding a global variable, then computes a sum of two bytes from that offset plus a constant and adds another global variable to store the result in a third global variable.
- `perform_multiple_word_copies`: A 28-instruction routine that calculates a table offset and performs three consecutive word copies and flag updates.
- `add_offset_to_global`: Adds the result of another routine to a global variable and passes the sum to a third routine along with two parameters.
- `call_subroutine_wrapper`: Passes three parameters to another routine.
- `manipulate_stack_and_registers`: Adjusts the stack pointer, saves registers, and performs conditional moves between registers based on a value from the stack.
- `execute_subroutine_wrapper`: A 24-instruction routine that calls another subroutine.
- `decrement_and_loop_2`: Decrements a register and branches back to a previous instruction if the result is non-zero, performing a similar decrement and move operation on other registers.
- `execute_subroutine_wrapper_2`: A 20-instruction routine that calls another subroutine.
- `decrement_and_loop`: Decrements a register and branches back to a previous instruction if the result is non-zero.
- `unnamed_routine_execution`: This routine consists of 27 instructions and is called by two separate routines.
- `call_subroutine_2`: A 14-instruction routine that calls another routine.
- `process_input_stream`: Iterates through a buffer of bytes, performing conditional transformations and lookups based on specific byte values.
- `parse_input_stream_buffer`: Iterates through a byte buffer, performing conditional transformations and lookups based on byte values.
- `tiny_move_rts_3`: Small routine of 2 instructions that copies (a7)+ to d7, then returns.
- `execute_sequential_table_updates`: Calls several subroutines in sequence, including multiple calls to a routine that performs table data updates using varying parameters and a global variable.
- `retry_function_sequence`: Recursively calls itself and attempts to execute a sequence of different routines if the previous return value is zero.
- `retry_function_calls_2`: Recursively calls itself and repeatedly executes a sequence of two different routines if the return value is zero.
- `copy_structured_data_table`: Performs a loop that copies bytes from a source pointer into a destination table at specific offsets, repeating until a counter reaches a limit.
- `execute_callback_chain_2`: Retrieves a pointer from a global variable and executes a subroutine through it, then conditionally executes three other subroutines via pointers stored at specific offsets within that same global variable.
- `execute_partial_callback_chain`: Retrieves a pointer from a global variable and executes a subroutine through it, then conditionally executes two other subroutines via pointers at specific offsets and calls another routine.
- `initialize_system_state_3`: Sets numerous global variables to zero or specific constants and executes a sequence of subroutines to release resources and perform system operations.
- `initialize_memory_table`: Clears a block of memory using a loop and then assigns specific values to several global variables.
- `clear_memory_buffers`: Sets two global variables to represent ranges and clears two distinct memory buffers using loops.
- `execute_repeated_calls`: Executes another routine twenty-one times within a loop.
- `iterate_with_offset`: Calls another routine one hundred and twenty-one times, incrementing a pointer by a fixed amount each iteration.
- `loop_with_increment`: Executes another routine fifty times while incrementing a value by a fixed amount in each step.
- `perform_looping_calls`: Calls another routine one hundred and forty-nine times using an incrementing value.
- `execute_routine_list`: Iterates through a list of code pointers, executing each one with an incrementing index.
- `iterate_function_pointers`: Increments a counter and calls a subroutine using a pointer that advances by a fixed offset in a loop.
- `iterate_function_table`: Executes a routine ten times, incrementing a pointer by a fixed offset during each iteration.
- `process_data_sequence`: Runs a loop fifty times that calls another routine with an incrementing value.
- `initialize_data_structure_4`: Stores several words and bytes into a memory block pointed to by a register, copies values between table entries, and modifies a global byte and a value at a calculated offset.
- `copy_table_values`: Copies two words from one location in a memory block to another and updates a global byte and a value at a calculated offset.
- `clear_memory_blocks`: Writes zero to a specific memory location in increments of three bytes for six iterations.
- `test_and_call_subroutine`: Tests a byte in a register and, if non-zero, executes a subroutine and performs bitwise operations on a global variable.
- `process_table_entries`: Iterates through a table to find a specific entry, updates a pointer and a byte, and performs a search through a string until a null terminator is found.
- `store_bytes_in_loop`: Saves registers to the stack and executes a loop that stores a zero byte into a stack-based memory location five times.
- `update_indexed_table`: Checks a condition based on a stack value and a table entry, then updates a byte in a table and increments a local variable.
- `call_external_subroutine`: Pushes a register onto the stack, calls a subroutine via a pointer, and adjusts the stack pointer.
- `search_and_update_table`: Iterates through a data table to find a matching byte, then updates several fields in a structure based on values found in the table.
- `validate_and_update_entity_state`: Performs bounds checking on multiple coordinate or index values against stored limits before updating specific state bytes and pointers within a data structure.
- `update_grid_elements`: Iterates through a table of twelve entries, clearing a word at a specific offset and copying a word from another offset within each entry.
- `initialize_data_table`: Iterates through a series of memory locations, calling a subroutine for each entry and then writing a constant byte value to that location.
- `populate_data_array`: Clears a series of memory locations in a loop, sets several global variables to specific constants, and copies values within a data array.
- `search_and_update_entity_table`: Performs a multi-level search through a hierarchical data structure or table to locate a specific entry.
- `update_table_data`: Iterates through a nested structure of bytes to perform lookups and stores resulting values into a table at a fixed memory location.
- `process_indirect_data_table`: Iterates through a data structure by fetching bytes from calculated offsets to perform conditional branching and logic.
- `process_state_and_data`: Calculates a value based on a global variable, copies data from a stack array into a global location, and executes several subroutine calls.
- `tiny_moveq`: Small routine of 1 instruction that loads 0 into d3.
- `execute_sequence_of_subroutines`: Calls several subroutines in a fixed order and resets two global variables to zero.
- `copy_word_and_set_flag`: Copies a word from a memory location pointed to by a register to another location and performs a bitwise OR on a data register.
- `push_address`: Pushes a specific longword address onto the stack.
- `execute_state_transition`: Pushes a constant onto the stack, calls another routine, updates a global word with the result, sets a global byte to one, and branches to another location.
- `store_word_to_memory`: Stores a word from a data register into a longword memory location.
- `write_and_branch`: Writes a word to a memory location, clears a word at the following memory location, and performs a short branch to another routine.
- `update_state_via_conditional_calls`: Executes a sequence of calls to a routine using specific parameters, updates a global variable with the return values, and modifies a global byte based on the value of a register through a conditional branching structure.
- `fetch_bytes_from_table`: Adjusts the stack pointer and uses a doubled value from a data register as an index to fetch two bytes from a table into global memory locations.
- `compare_and_set_register`: Compares a word from memory against a constant and sets a data register to a specific value if the comparison fails and the word is non-zero.
- `update_state_and_load_table`: Updates a global variable with a value from a stack offset and then conditionally executes a series of calls to another routine to populate a global variable based on the value of a register.
- `tiny_ext`: Small routine of 1 instruction that sign-extends d0.
- `tiny_move_4`: Small routine of 1 instruction that copies d0 to d1.
- `tiny_asl`: Small routine of 1 instruction that shifts d1 left by 2.
- `tiny_sub`: Small routine of 1 instruction that subtracts d0 from d1.
- `shift_left_data`: Shifts the bits in a data register left by two positions.
- `subtract_longword`: Subtracts the value of one data register from another.
- `tiny_asl_2`: Small routine of 1 instruction that shifts d1 left by 4.
- `fetch_offset_byte`: Calculates an address by adding a data register to a base address and loads a byte from an offset of six bytes into a data register.
- `tiny_ext_2`: Small routine of 1 instruction that sign-extends d6.
- `push_pointer_to_stack`: Pushes the contents of a specific memory location onto the stack.
- `push_constant_long`: Pushes a constant long value onto the stack.
- `setup_stack_and_push`: Copies a long value from the stack to the stack pointer and pushes a long value onto the stack.
- `update_state_via_logic`: Evaluates a register to select between several different function calls, updates a global variable with the result, and modifies a global byte value.
- `call_subroutine`: This routine performs a single subroutine call to another routine and then continues execution into the subsequent code block.
- `adjust_stack_and_store`: Adjusts the stack pointer by an offset and stores a word from a register into a global variable.
- `store_constant_word`: Stores the constant value five into a global variable.
- `branch_to_routine`: Performs a short branch to another location.
- `tiny_move_5`: Small routine of 1 instruction that copies d7 to d0.
- `decrement_and_push_stack`: Decrements a register and branches if non-zero, otherwise pushes a pointer and two values onto the stack.
- `execute_sequence_of_calls`: Calls a routine multiple times, storing the results in a global variable and updating a global byte value.
- `offset_stack_pointer`: Adds an offset to the stack pointer.
- `update_status_and_branch`: Stores a word from a register into a global variable, sets a global byte to six, and performs a short branch.
- `decrement_and_branch`: Decrements a register and branches if it is not zero; otherwise, it pushes a pointer to the stack and pushes a constant onto the stack.
- `execute_limited_calls`: Calls a routine twice, storing the results in a global variable and updating a global byte value.
- `fill_memory_with_byte`: Fills a range of memory starting at a specific address with the value two using a loop, then stores a value in a global variable.
- `copy_memory_block_3`: Checks a global byte and a parameter against several values, then copies sixty-four bytes from a calculated memory location to a fixed destination using a loop.
- `restore_registers_from_stack`: Restores multiple longwords from the stack into registers d2 and d7.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `tiny_move_6`: Small routine of 1 instruction that copies d7 to -(a7).
- `load_word_from_stack`: Loads a word from an offset on the stack into register d7.
- `modify_register_based_on_flag`: Loads a word from a global variable into a register, branches if it is not zero, and then decrements that register if another register is zero.
- `dispatch_parameter_logic`: Calls another routine with specific parameters and updates a global byte based on the value of a register.
- `execute_conditional_function`: Calls a routine with four arguments and updates a global variable if a specific value is held in a register.
- `branch_if_not_equal`: Checks a word from a stack offset against zero and branches if non-zero; if zero, it pushes three pointers onto the stack and branches to another routine.
- `conditionally_call_function`: Checks if a global variable equals one, and if so, executes another routine using several parameters and stores the result in a global variable.
- `test_and_push_stack_frame`: Tests a word in a global variable and branches if it is non-zero.
- `process_input_and_store`: Pushes an address onto the stack, calls a subroutine, stores a word from a register into a global variable, and conditionally calls another subroutine after comparing a register to a value in memory.
- `adjust_stack_pointer`: Adds a constant offset to the stack pointer register.
- `store_word_and_load_constant`: Stores a word into a global variable and loads a small constant into register d0.
- `compare_and_branch_4`: Compares a word in a register with a value in a global variable and branches if they are not equal.
- `call_subroutine_with_arguments`: Pushes several values onto the stack and calls another routine.
- `update_global_from_register`: Adjusts the stack pointer, stores a word from a register into a global variable, and pushes an address onto the stack if a comparison between a register and a memory value fails.
- `push_constant_to_stack`: Pushes a constant longword onto the stack.
- `push_value_from_stack`: Pushes a longword from a stack offset onto the stack.
- `call_function_with_one_argument`: Calls a routine with a single argument and updates a global variable with the result.
- `update_global_table_entries`: Calls another routine five times to store results into a global variable using various parameters and offsets.
- `call_and_test`: Calls a routine and then tests a word from a global variable to determine whether to branch to another location.
- `call_via_stack`: Adjusts the stack pointer, pushes a word from a global variable onto the stack, and calls another routine.
- `tiny_addq`: Small routine of 1 instruction that adds 4 to a7.
- `store_word_and_branch`: Stores a word from a register into a global variable and then performs a short branch.
- `compare_and_adjust_stack`: Compares a word from a global variable with a register and adjusts the stack pointer if they are not equal.
- `tiny_moveq_2`: Small routine of 1 instruction that loads 12 into d0.
- `push_data_word_to_stack`: Pushes a 16-bit word from a data register onto the stack.
- `call_input_parser`: Calls another routine via a subroutine jump.
- `call_external_subroutine_2`: Calls another routine at a specific memory address.
- `call_and_adjust_stack`: Calls a subroutine and then increments the stack pointer by four.
- `branch_to_subroutine`: Performs a short relative branch into the middle of another routine.
- `compare_and_test`: Compares a register with a value from a global variable and branches if they are not equal, then tests a second global variable.
- `branch_if_equal`: Performs a short branch if a zero flag is set.
- `call_subroutine_3`: Calls a routine via a long jump to a specific address.
- `call_subroutine_via_long_jump`: Executes a subroutine call using a long jump to a specific address.
- `branch_to_offset_2`: Performs a short relative branch to a specific location within another routine.
- `execute_sequence_of_calls_2`: Calls three consecutive routines in succession.
- `check_memory_conditions`: Calculates an offset based on two input values and checks if specific bytes at that location are zero before calling another routine.
- `transform_data_table`: Iterates through a multi-dimensional data structure, applying conditional arithmetic transformations to byte values based on bitmask checks and specific value ranges.
- `transform_table_data`: Iterates through a data table using nested loops to modify byte values based on bitwise flags and specific range checks.
- `increment_global_conditionally`: Compares a register against two constants and, if the value falls within a specific range, calculates a value using a register, a constant, and a bitwise mask to add to a global variable.
- `increment_global_value`: Compares a word in a register to a constant and, if it is less than that constant, adds a value derived from a global variable and a register to a global variable.
- `conditionally_increment_global_value`: Compares a word in a register against a constant and, if it is less than that constant, calculates a value by adding a word from a global variable to another register, masking the result, and adding that to a different global variable.
- `modify_global_by_range_3`: Compares a word in a register against a constant and, if it is not greater than or equal to that constant, adds a bitwise modified value to a global variable.
- `modify_global_by_range_2`: Checks if a word in a register falls within a specific range and, if so, adds a bitwise-modified value to a global variable.
- `modify_global_by_range`: Checks if a register falls within a specific range and, if so, performs a bitwise operation on a value and adds it to a global variable.
- `modify_global_with_mask`: Compares a register against two different constants and, if it falls within a specific range, performs a bitwise AND on a value and adds the result to a global variable.
- `modify_global_via_mask`: Compares a register to a constant, and if it is less than that constant, performs a bitwise AND on a value and adds the result to a global variable.
- `traverse_data_table`: Iterates through a structured data table using nested conditional checks and pointer arithmetic.
- `modify_global_counters`: Conditionally increments global longwords and performs a subroutine call based on a comparison.
- `update_and_restore_registers`: Adds a word to a global longword, compares a value, and restores multiple registers from the stack before returning.
- `calculate_and_store_offset`: Copies a word to a global variable, compares a register to a constant, and if not equal, calculates a scaled value and stores it in another global variable.
- `copy_and_branch`: Copies two words from memory locations to a global table and then performs a short branch.
- `copy_and_increment_table`: Iterates through a memory region by copying values and incrementing a pointer and a global variable until a specific memory threshold is reached.
- `move_word_and_branch`: Moves a word from a memory location to another memory location and performs a short branch.
- `clear_and_branch_2`: Copies a word from memory to a global table, clears another global word, and performs a short branch.
- `iterate_and_update_globals`: Loops through a memory region, copying values from a table into global variables and calling another routine during each iteration.
- `update_state_and_dispatch_logic`: Increments a global counter and performs a multi-way branch based on the value of a state variable.
- `calculate_absolute_value`: Performs a series of conditional checks and arithmetic operations on global variables to return an absolute value.
- `calculate_absolute_difference`: Takes a value from a global variable and returns its absolute value after applying a conditional subtraction.
- `fetch_byte_from_indexed_table`: Calculates an offset using two parameters and returns a byte from that calculated position in memory.
- `fetch_table_byte`: Calculates an offset by shifting a parameter and adding another, then retrieves a byte from the resulting memory address plus a fixed offset.
- `toggle_state_and_update`: Checks a global variable to toggle between two states, updating various global variables and performing a calculation based on a constant.
- `update_global_state_and_branch`: Decrements a global variable if it is non-zero, otherwise it performs a conditional check to store a sequence of values into a global variable and calls another routine.
- `test_and_load_word`: Tests a register and a longword in memory, then conditionally loads a word from that longword into a register.
- `process_indexed_table`: Iterates through a table of data, performing arithmetic on elements and calling another routine.
- `execute_recursive_calls`: Performs a series of recursive calls to itself with varying parameters and constant values, interspersed with calls to another routine.
- `update_entity_states`: Iterates through a list of structures to update state bytes and call specific routines based on internal values.
- `store_register_contents_to_global`: Extracts specific bytes from a passed parameter and stores them into a global variable.
- `process_hunk_type_codes`: Iterates through a nested loop to compare values against AmigaDOS hunk type codes and updates a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_state`: Executes a series of routines and performs bitwise shifts on a global variable before setting a global value to two.
- `initialize_system_state_2`: Sets up several parameters and calls multiple subroutines to configure system state using values from a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_sequence_loop`: Performs a series of subroutine calls and enters an infinite loop.
- `perform_complex_initialization`: Executes a loop one hundred times, modifies several global variables, and calls multiple routines before entering an infinite loop.
- `call_subroutine_with_calculated_address`: Calculates an address by multiplying an input value by a constant and adding a fixed offset, then passes this address as an argument to another routine.
- `clear_memory_at_offset`: Sets a byte to zero at a location determined by an offset from a pointer, then calls another subroutine.
- `clear_fifty_four_bytes`: Fills fifty-four bytes of memory starting at a given pointer with zeros.
- `clear_forty_seven_bytes`: Fills forty-seven bytes of memory starting at a given pointer with zeros.
- `clear_twenty_three_bytes`: Fills twenty-three bytes of memory starting at a given pointer with zeros.
- `clear_seven_bytes`: Fills seven bytes of memory starting at a given pointer with zeros.
- `clear_one_hundred_thirteen_bytes`: Fills one hundred and thirteen bytes of memory starting at a given pointer with zeros.
- `clear_memory_block_small`: Fills a memory block pointed to by a parameter with zeros for twenty-three iterations.
- `clear_memory_block_medium`: Fills a memory block pointed to by a parameter with zeros for fifty-one iterations.
- `clear_memory_block_large`: Fills a memory block pointed to by a parameter with zeros for three hundred thirty-three iterations.
- `clear_memory_block_tiny`: Fills a memory block pointed to by a parameter with zeros for three iterations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `nop_padding_sequence_3`: The routine is empty and consists of two instructions that perform no operation.
- `process_data_structures`: Iterates through a list of structures, updating internal counters and executing a function pointer if specific conditions are met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_global_and_call`: Sets a global variable to zero and performs a series of subroutine calls with specific values.
- `loop_until_flag`: Sets a global variable to zero and enters a loop that executes subroutines until a global variable reaches a specific state.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_addq_rts`: Small routine of 2 instructions that adds 4 to a7, then returns.
- `fill_memory_pattern`: Iterates through a memory range and stores a constant byte value at specific offsets within a repeating pattern.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `populate_data_table`: Uses nested loops to write values derived from a global variable into a large memory table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `nop_padding_sequence_2`: This routine is empty and contains no executable instructions.
- `clear_four_state_words`: Writes zero to four 16-bit state words in a fixed group.
- `compare_and_branch_3`: Loads a word from a global memory location into a register and branches if it is not equal to a specific constant.
- `compare_and_branch_on_inequality`: Loads a small immediate value into a register and compares it to a word stored at a global memory location.
- `decrement_and_compare_global`: Decrements a global word, compares it to a constant, and branches or returns based on the comparison result.
- `compare_register_with_constants`: Compares a register with a global variable and branches if they are not equal.
- `update_state_and_check_flag`: Calls two routines with specific memory addresses and a concatenated value from a global variable, then returns either one or two based on whether that global variable is zero.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `update_game_state`: Determines a value based on a global variable and performs multiple conditional calls to other routines to update various parameters and global state.
- `process_stack_data_2`: This routine uses values from the stack to conditionally call another routine multiple times, updating a global variable and potentially calling a separate routine based on stack values.
- `process_stack_data`: Performs several calls to another routine using values from the stack and a global variable, then conditionally calls a separate routine.
- `initialize_object_sequence`: Calls several routines to allocate memory, initialize objects with specific flags, and updates a global variable based on conditional checks.
- `copy_data_blocks`: Copies six blocks of eight bytes each from one memory location to another and then writes a series of constant values to a global variable.
- `execute_block_copies`: Calls another routine six times, passing various memory addresses as arguments.
- `execute_repetitive_allocations`: Performs a series of loops that call a routine to allocate and initialize objects a specific number of times.
- `apply_relocation_hunks`: Iterates through a list of relocation data to patch memory addresses.
- `copy_strided_data_block`: Copies data from a source buffer to a destination buffer using non-linear strides and offsets.
- `copy_structured_data_2`: This routine copies data from a source to a destination using a loop that applies specific offsets and strides to the data being moved.
- `copy_structured_data`: Copies data from a source buffer to a destination buffer using a stride of sixteen bytes and a specific offset pattern.
- `transfer_table_elements`: Copies data from a source table to a destination table using two different stride patterns based on a conditional check.
- `copy_structured_array_elements`: Iterates through a source array and copies specific non-contiguous fields to a destination array.
- `blend_bitplane_data`: Performs bitwise logical operations to blend source data into a destination buffer using variable bit-shifts and bitwise AND/OR operations.
- `blend_bitmapped_layers_3`: Performs a nested loop to combine multiple source data blocks into a destination buffer using bitwise OR and NOT operations.
- `blend_bitmapped_layers_2`: Performs a nested loop to combine multiple source data blocks into a destination buffer using bitwise OR and NOT operations.
- `perform_bitmask_blitting`: Performs a nested loop to combine bytes from a source buffer using bitwise OR and NOT operations, storing the results into a destination buffer at specific offsets.
- `copy_and_bitwise_or_data`: Copies a block of memory to a destination and then performs a bitwise OR operation between the destination and subsequent source blocks.
- `blend_bitmapped_layers_with_bit_shift`: Performs a bitwise combination of multiple data blocks into a destination buffer by applying bitwise AND with inverted values and bitwise OR with shifted values.
- `blend_bitmapped_layers`: Performs a bitwise combination of multiple data blocks into a destination buffer using a combination of bitwise AND with inverted values and bitwise OR operations.
- `clear_and_load_registers`: Clears the contents of a memory location pointed to by a register and loads two specific byte values into a data register.
- `search_and_reset_table`: Iterates through a memory range to find a specific byte value, then uses a pointer derived from a global variable to zero out a series of memory entries.
- `reset_global_parameters`: Sets several global variables and memory locations to zero or a fixed value.
- `update_state_counters`: Increments a global byte, performs conditional checks against other global values, and updates several memory locations with new data or pointers.
- `modify_entity_attribute_value`: Updates a numeric attribute within an entity structure by applying a modifier based on a type identifier.
- `update_entity_attributes`: Updates a structure's properties by looking up values in a data table based on a masked type identifier.
- `process_field_modifier`: Updates a numeric value in a structure based on a mode flag, applying either direct addition, subtraction, or a lookup from a table.
- `copy_structure_field`: Copies a 16-bit word from one memory location to another within a structure.
- `search_and_update_table_value`: Iterates through a table to find an entry that satisfies a comparison against a masked value, then updates a structure member with the found value.
- `update_scaled_value`: Adjusts a value in a structure by adding or subtracting a byte based on a flag, clamps it to a specific range, and then performs a table lookup to store a result in another location.
- `calculate_scaled_value`: Computes a value based on bit-shifted fields from a source structure and stores the result in a destination table.
- `process_input_command_data`: Extracts a command type from a bit-field in a structure and executes specific logic based on the value.
- `mask_byte_field`: Extracts the lower four bits from a byte at a specific offset in a structure and stores them in another offset within the same structure.
- `update_buffer_index`: Modifies a byte in a structure based on a lookup table and increments a pointer within a buffer if certain conditions are met.
- `loop_through_memory_bytes`: The routine performs a fixed-count loop that repeatedly compares a byte at a specific offset within a memory structure to itself.
- `configure_audio_dma`: Sets various hardware registers for audio and DMA, then updates several global variables and pointers based on values retrieved from a structure.
- `execute_chained_callbacks`: Repeatedly calls function pointers stored within a data structure, updating a global variable with the returned pointer until a null result is encountered.
- `execute_callback_via_pointer`: Checks if a global variable is non-zero and, if so, calls a function pointer located at a fixed offset from that variable.
- `initialize_hardware_and_state`: Configures various hardware registers including bitplane and display parameters, allocates system resources, and populates a table with values from a global variable using a loop.
- `process_object_cleanup`: Checks a global flag and, if set, executes a sequence of function calls through pointers stored within various memory structures.
- `update_state_counters_2`: Updates a global variable, writes values to a series of memory locations in a loop, and conditionally resets a global variable and a function pointer.
- `copy_two_word_pairs_to_globals`: The routine copies the contents of two global variables into four other global variables, pairing the first source with two destinations and the second source with two destinations.
- `update_mouse_coordinates`: Calculates the difference between a hardware input value and a previous value to update two global coordinate variables, clamping them within a specific range.
- `execute_infinite_loop`: The routine enters an infinite loop that prevents further execution of the calling thread.
- `setup_blt_parameters`: sets several global values and calls other routines to configure bitplane transfer parameters.
- `setup_blt_parameters_2`: configures bitplane transfer parameters by writing to global variables and calling other routines.
- `copy_color_data`: copies sixteen words from a source pointer to the hardware color registers.
- `copy_and_shift_data`: copies sixty-four bytes from a source to a destination, then copies a subset of that data to a new location and clears the trailing bytes.
- `return_parameter`: returns the value passed in a register.
- `copy_and_align_memory_block`: Copies a block of memory from a source to a destination while performing alignment adjustments.
- `accumulate_32bit_from_bytes`: reads four consecutive bytes from a pointer in a register and builds a 32-bit value in a register by concatenating each byte into the result.
- `clear_color_registers`: writes zero to thirty-two consecutive words in the hardware color registers.
- `halt_execution`: The routine enters an infinite loop and performs no operations.
- `update_color_palette`: Updates a color register and writes a value to a custom hardware register.
- `execute_function_list`: Iterates through a list of function pointers stored in a global variable, calling each until a null pointer is encountered, and returns the result of the last successful call.
- `call_global_function_with_return`: Calls a function pointer stored in a global table using a global variable as an argument and returns a constant value.
- `load_stack_parameters`: Loads two 16-bit values from the stack into registers and moves a third value from the stack into a pointer register.
- `calculate_string_length_delta`: Iterates through a sequence of nodes to calculate a difference between values while updating a global variable.
- `test_bit_in_register`: Tests a specific bit within a register.
- `process_string_data`: Checks a pointer chain and performs conditional logic based on a byte flag.
- `execute_conditional_task`: Checks a value and executes a function pointer from a global variable if certain conditions are met.
- `move_stack_word`: Moves a 16-bit word from the stack into a data register.
- `validate_node_status`: Checks the status of a memory block by inspecting specific offsets.
- `allocate_and_initialize_object`: Allocates a memory block from a linked list and initializes it with a string and a set of flags derived from a bitmask.
- `execute_callback`: Passes a signed 16-bit integer as an argument to another routine.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `parse_script_string`: Parses a string to identify command tokens and arguments, handling quoted text and escape sequences.
- `manage_task_state`: Checks a global byte value, retrieves a task pointer, and conditionally calls a routine while updating a global variable and a function pointer table.
- `traverse_pointer_chain`: Traverses a chain of pointers a specified number of times and returns the resulting pointer or a specific constant if the end of the chain is reached.
- `execute_task_callback`: Retrieves a function pointer from a global structure, executes it, and performs conditional logic involving further function calls and signal management.
- `find_character_in_range`: Searches for a specific character within a string, limited by a provided count, and returns a pointer to the character found.
- `unlink_and_reorganize_list_node`: Traverses a linked list to locate a specific memory block, modifies its internal header and size fields, and then removes the block from the list by updating pointers and calling a callback function.
- `branch_with_argument`: Moves a longword from the stack into a register and performs a short branch.
- `clear_and_branch`: Clears a register, loads a word from the stack into it, tests the value, and performs a short branch.
- `allocate_memory_block`: Manages a linked list of memory blocks to satisfy a request for a specific size.
- `execute_linked_callbacks`: Traverses a linked list of pointers and executes the routine pointed to by the second word of each element.
- `update_node_status`: Updates various fields within a linked list of structures based on bitmask checks and comparisons with the result of another routine.
- `load_stack_parameter`: Loads a value from a location on the stack into a register.
- `validate_structure_status`: Checks bit flags and pointer values within a structure to determine a return status.
- `parse_command_string`: Parses a command string to set specific bit flags and integer values within a structure.
- `manage_linked_list`: Traverses a linked list to find the end and allocates a new block of memory if the list is empty or reaches the end.
- `update_global_pointer`: Retrieves a value from a global variable and updates that variable by calling functions through pointers stored at specific offsets from the original value.
- `lookup_value_by_range`: Performs a range check on an input value and returns a byte retrieved from one of several memory tables based on that range.
- `call_other_routine_2`: Calls another routine and then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant_3`: Returns the constant -1 in d0 to the caller without any other effect.
