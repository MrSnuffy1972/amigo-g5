# Game 1999 (operation stealth (1990)(u. s. gold)(sp)[cr pdx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01999). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2735824 top-level bytes in 45 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 14 of 14 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 222 of 222 named (0 author symbols), 222 of 222 described (51 by rule, 169 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1667 of 1667 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5151 of 5151 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2343382 |
| code | 16662 |
| data | 273886 |
| padding | 8710 |
| unused | 93184 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Message | 2 | 24 | 4 |
| Operation Stealth (1990)(U. S. Gold)(Sp)[cr PDX](Disk 1 of 3).adf:endcli | 2 | 117 | 2 |
| Operation Stealth (1990)(U. S. Gold)(Sp)[cr PDX](Disk 1 of 3).adf:intro | 2 | 174 | 12 |
| Operation Stealth (1990)(U. S. Gold)(Sp)[cr PDX](Disk 1 of 3).adf:delphine | 2 | 207 | 8 |
| Operation Stealth (1990)(U. S. Gold)(Sp)[cr PDX](Disk 1 of 3).adf:add21k | 3 | 637 | 38 |
| Operation Stealth (1990)(U. S. Gold)(Sp)[cr PDX](Disk 1 of 3).adf:FABS | 1 | 155 | 3 |
| Operation Stealth (1990)(U. S. Gold)(Sp)[cr PDX](Disk 1 of 3).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1030 | 34 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 416 | 15 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2373 | 100 |

## Cases handled

- operation stealth (1990)(u. s. gold)(sp)[cr pdx](disk 1 of 3)/Operation Stealth (1990)(U. S. Gold)(Sp)[cr PDX](Disk 1 of 3).adf: file S/TxEd.Backup (header 164 bytes, blocks hold 171): its blocks are judged as raw blocks outside the filesystem
- operation stealth (1990)(u. s. gold)(sp)[cr pdx](disk 1 of 3)/Operation Stealth (1990)(U. S. Gold)(Sp)[cr PDX](Disk 1 of 3).adf: file BootGirl.data (header 21340 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address (f100): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (ext): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (ext): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f100): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (ext): 407 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (ext): 130 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (ext): 904 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- operation stealth (1990)(u. s. gold)(sp)[cr pdx](disk 1 of 3)/Operation Stealth (1990)(U. S. Gold)(Sp)[cr PDX](Disk 1 of 3).adf: blocks 11-21 (bytes 5632-11264) outside the filesystem hold data, not code (opcode density 0.044, entropy 5.81)
- operation stealth (1990)(u. s. gold)(sp)[cr pdx](disk 1 of 3)/Operation Stealth (1990)(U. S. Gold)(Sp)[cr PDX](Disk 1 of 3).adf: blocks 950-950 (bytes 486400-486912) outside the filesystem hold data, not code (opcode density 0.023, entropy 3.21)
- operation stealth (1990)(u. s. gold)(sp)[cr pdx](disk 1 of 3)/Operation Stealth (1990)(U. S. Gold)(Sp)[cr PDX](Disk 1 of 3).adf: blocks 1004-1045 (bytes 514048-535552) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.04)
- operation stealth (1990)(u. s. gold)(sp)[cr pdx](disk 2 of 3)/Operation Stealth (1990)(U. S. Gold)(Sp)[cr PDX](Disk 2 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- operation stealth (1990)(u. s. gold)(sp)[cr pdx](disk 3 of 3)/Operation Stealth (1990)(U. S. Gold)(Sp)[cr PDX](Disk 3 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- operation stealth (1990)(u. s. gold)(sp)[cr pdx](disk 1 of 3)/Operation Stealth (1990)(U. S. Gold)(Sp)[cr PDX](Disk 1 of 3).adf:delphine: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Operation Stealth (1990)(U. S. Gold)(Sp)[cr PDX](Disk 1 of 3).adf:intro

## Copy protection found

- none found

## Functions


### Message

- `output_message`: Performs library operations to write data to an output device.
- `open_library`: Opens a library using the exec library.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.

### Operation Stealth (1990)(U. S. Gold)(Sp)[cr PDX](Disk 1 of 3).adf:endcli

- `invoke_output_routines`: Calls routines to perform library operations for writing data to an output device.
- `invoke_output_message`: This routine calls another routine that performs library operations to write data to an output device.

### Operation Stealth (1990)(U. S. Gold)(Sp)[cr PDX](Disk 1 of 3).adf:intro

- `update_hardware_color`: This routine modifies a hardware color register and references the text IMP!.
- `update_background_color`: This routine modifies a hardware color register.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `execute_subroutine_sequence`: This routine executes a sequence of subroutines and a small block of instructions.
- `execute_multiple_subroutines_2`: This routine performs a sequence of subroutine calls and executes a small block of instructions.
- `call_subroutine_3`: This routine executes a single subroutine call.
- `execute_three_instructions_2`: A routine consisting of three instructions.
- `call_subroutine_wrapper`: This routine acts as a bridge that invokes another subroutine via a short wrapper.
- `invoke_subroutine_wrapper`: This routine performs a single call to another subroutine.
- `execute_subroutine_call`: A short routine consisting of seventeen instructions that calls another routine.
- `call_subroutine_2`: Calls another routine using twenty-eight instructions.
- `call_subroutine`: This routine calls another routine.

### Operation Stealth (1990)(U. S. Gold)(Sp)[cr PDX](Disk 1 of 3).adf:delphine

- `manage_memory_and_tasks`: A program entry routine that utilizes memory allocation and task finding services.
- `execute_five_instructions`: This routine consists of five instructions and is called by one unnamed routine.
- `output_data`: Calls a routine to write data to an output device and calls another unnamed routine.
- `tiny_ori`: Small routine of 1 instruction that merges 152 into d0.
- `manipulate_color_registers`: This routine modifies the color hardware register and calls two other routines containing a specific number of instructions.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `execute_ten_instructions`: This routine consists of ten instructions and is called by one unnamed routine.
- `execute_three_instructions`: This routine consists of three instructions and is accessed via a relative address.

### Operation Stealth (1990)(U. S. Gold)(Sp)[cr PDX](Disk 1 of 3).adf:add21k

- `initialize_system_libraries`: This routine serves as the program entry point and calls a routine to manage system libraries.
- `manage_system_libraries`: A large routine that interacts with various system libraries to open, read, and release files or resources.
- `execute_two_instructions`: This routine consists of two instructions and is called by one unnamed routine.
- `access_system_resources`: A routine that interacts with multiple library vectors and DOS functions to open, read, and release files or system resources.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `execute_multiple_subroutines`: Executes several subroutines of varying instruction counts and calls a library function via a global variable.
- `execute_thirteen_instructions_2`: This routine executes thirteen instructions.
- `execute_thirteen_instructions`: This routine consists of thirteen instructions and is called by two unnamed routines.
- `execute_twelve_instructions`: Performs twelve instructions and returns to its caller.
- `open_system_resources`: This routine uses icon.library and DOS library functions to open files or resources and interacts with various library vectors.
- `execute_single_instruction`: Executes a single instruction via a relative jump or call.
- `release_file_resources_2`: Uses library functions to close a file handle and release resources.
- `release_file_resources`: This routine uses DOS library functions to unlock and close a file handle and calls several other library vectors.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_file_handle`: Calls a wrapper for a DOS library function to close a file and returns the result.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `output_dos_data`: Uses a DOS library function to output data.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_function`: Loads a library base from a global variable and calls a function at a specific offset.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_function_offset`: Invokes a library function using a base address stored in a global variable.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_offset_negative`: Calls a library function using a negative offset from a library base.

### Operation Stealth (1990)(U. S. Gold)(Sp)[cr PDX](Disk 1 of 3).adf:FABS

- `modify_color_hardware`: A program entry routine that touches a color hardware register and calls several unnamed routines.
- `execute_unnamed_calls_2`: Calls three unnamed routines using twelve instructions.
- `execute_unnamed_calls`: This routine calls three unnamed routines.

### Operation Stealth (1990)(U. S. Gold)(Sp)[cr PDX](Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_and_call_functions`: This routine initializes a block of memory to zero using a loop and then executes a series of conditional function calls or assignments involving a global variable and a pointer.
- `initialize_game_state_tables`: Initializes a series of global pointers and clears or populates memory-resident tables, including a loop that zeros out a large block of memory and a loop that searches through a list of pointers to assign a specific index or value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_update_globals`: Clears a block of memory using a loop, updates a global variable, and conditionally calls a routine using a pointer from a global variable.
- `traverse_and_call_pointers_2`: Recursively checks a memory location and calls a routine if a certain value is non-zero, then updates a global variable with the return value.
- `traverse_and_call_pointers`: Checks a memory location and calls a routine if a certain value is non-zero, then recursively calls itself and calls other routines based on pointer checks.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `do_nothing_routine`: the routine is empty and performs no operations before returning.
- `update_memory_from_pointer`: Calls another routine and copies a value from a pointer-based address into a destination, while zeroing out other memory locations.
- `execute_conditional_logic_chain`: Performs a series of conditional checks on a global pointer, calling various subroutines, and updating memory or configuration tables based on certain values.
- `pass_through_routine`: This routine calls another routine with the same arguments and then returns.
- `read_input_flags`: Returns a byte derived from a global variable, testing certain bits and combining the results with a logical OR. It references the CIAA_PRA and POTGOR hardware registers.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `set_global_value`: Sets a global variable to a specific constant value and modifies a hardware register.
- `update_and_scale_value`: Checks a value, multiplies a global variable by two, applies an XOR mask if a carry or zero condition occurs, and returns a value involving a division and remainder calculation.
- `handle_keyboard_serial`: Returns a byte computed from the bitwise complement of a global variable when a flag bit is set, performs a decrementing delay loop, and clears a flag bit in the global variable.
- `populate_table_with_offsets`: Copies a block of data from a source table and populates a destination table with a series of calculated values or offsets derived from input parameters.
- `write_config_table`: Stores a series of hardcoded word values into a table located via a pointer in a global variable.
- `write_config_table_2`: Stores hardcoded word and long values into a table located via a pointer in a global variable.
- `load_pointer_from_table`: Returns a value loaded from a memory address calculated from a global variable.
- `update_raster_offsets`: Updates a table with new offset and bitmask values, using a routine that waits for a certain raster line.
- `copy_source_to_dest`: Copies a sequence of words from a source buffer to a destination buffer located in a table.
- `null_operation`: The routine is empty and performs no instructions or state changes.
- `no_operation`: the routine is empty and performs no instructions.
- `update_display_raster_parameters`: Updates a table with new offset and bitmask values by calling a routine that waits for a specific raster line.
- `execute_nested_calls`: Performs a sequence of calls to another routine using values passed or stored on the stack.
- `get_and_increment_index`: Reads a byte from a table using an index, then increments that index stored in a pointer.
- `initialize_and_fill_table`: Clears a block of memory, then iterates through a table to clear or update entries with values retrieved from a recursive call or or other routine.
- `retrieve_pointer_data_2`: Loads a 16-bit value from a memory location or table offset if a preceding routine returns a non-zero value.
- `retrieve_pointer_data`: loads a 16-bit value from a memory location or table offset if a preceding routine returns a non-zero value.
- `fetch_value_via_logic`: Calls another routine to check a condition, then potentially retrieves a short value from a table and calls another routine.
- `retrieve_input_flags`: Calls a routine to check a condition, then potentially retrieves a short value from a table and calls another routine, while also returning a byte from a global variable with certain bits tested.
- `read_input_flags_2`: Returns a byte derived from a global variable while testing certain bits and combining results with a logical OR.
- `initialize_game_state_tables_2`: Initializes a series of global pointers and clears or populates memory-resident tables, including a loop that zeros out a large block of memory.
- `reset_memory_values`: Sets a 4-byte memory location to zero and a 2-byte memory location to all ones.
- `reset_memory_values_2`: Iterates through a linked list, calling a routine for each node, and then sets a 4-byte memory location to zero and a 2-byte memory location to all ones.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_linked_list`: Recursively calls itself using a pointer or value from a table and updates a global variable before setting a certain memory location to a maximum value.
- `populate_buffer_with_function_results`: Fills a buffer with results from another routine and checks if a certain amount of data has been processed using a table-driven loop.
- `search_table_for_matching_value`: Iterates through a table to find an entry where a specific part of a value matches, then recursively checks a sub-range or linked structure.
- `search_and_recurse_table`: Iterates through a table of structures to find a matching key, then recursively calls itself or another routine on a calculated range of values if a match is found.
- `search_and_recurse_table_2`: Iterates through a table of structures to find a matching key and recursively calls another routine or updates a pointer with a calculated offset.
- `execute_recursive_operations`: Calls another routine with a fixed value, and if successful, performs several recursive calls involving calculated or passed parameters.
- `decompress_bitstream_to_buffer`: Decodes a bit-level stream to reconstruct and write byte values into a destination memory buffer.
- `process_bitstream_to_buffer`: Unpacks bits from a routine, shifts them into a register, and stores the resulting bytes into a buffer via a pointer.
- `unpack_bitstream_to_memory`: Reads bits from a bitstream to reconstruct values, which are then written to a memory buffer.
- `fill_memory_and_depack`: Fills a memory range with a repeated short value, modifies a hardware color register, and implements a bit-reading routine that extracts a certain number of bits from a buffer.
- `fill_memory_with_value`: Fills a range of memory with a repeated short value using a decrementing loop and modifies a hardware color register.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `return_stack_value`: This routine is empty and simply returns the value currently on the stack.
- `iterate_table_and_recurse`: Iterates through a table of structures to find a matching key, recursively calls another routine, and clears certain table entries if they are non-zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `print_formatted_message`: Checks a parameter value before calling a routine to handle string copying and display-related or graphics-related updates or loops.
- `infinite_loop_with_function_call`: Executes a loop that calls itself and another routine repeatedly without a termination condition.
- `update_graphics_parameters`: Performs several routine calls and calls a function with calculated offsets and a fixed value, likely intended for a display or certain hardware parameters.
- `calculate_string_render_bounds`: Calculates a centered horizontal bounding box and spacing for a string or a sequence of string segments delimited by a special character.
- `parse_string_and_calculate_dimensions`: Iterates through a null-terminated string to calculate a string length and then uses the string content to calculate a new width and height or offset, while updating a global counter and updating a display or window area.
- `calculate_string_layout_and_render`: Calculates a centered horizontal position and width-scaled spacing for a string of characters, then iterates through a character buffer to render each glyph.
- `render_string_with_spacing_2`: Calculates a centered horizontal position for a string and iterates through a character buffer to draw glyphs with variable spacing.
- `update_string_render_position_2`: Iterates through a character buffer to calculate and update horizontal text offsets and glyph-based drawing positions.
- `update_string_render_position`: Iterates through a string or character buffer to calculate and update horizontal text offsets.
- `render_character_with_spacing`: Iterates through a character or tile property table to calculate or write character or tile offsets, applying spacing or padding offsets while updating a pointer or counter.
- `render_string_with_spacing`: Iterates through a character table to calculate or write character or tile offsets, applying spacing or padding offsets while updating a render buffer.
- `parse_string_to_render_buffer`: Iterates through a sequence of characters, checking if they are alphanumeric or special, and updates a render buffer with character values and calculated screen offsets.
- `process_string_characters`: Iterates through a character sequence to check for alphanumeric or specific special characters, updating a render buffer with new character values or null terminators.
- `update_string_render_buffer`: Iterates through a character sequence, calculating or updating an offset or pointer, and writes character values or zero-terminators to a buffer while calling a routine to update display or graphic parameters.
- `update_position_and_buffer_2`: Calculates a difference between a table value and a global variable, updates a global offset, and copies a sequence of 16 words from a source.
- `update_position_and_buffer`: Calculates a difference between a table value and a global variable, updates a global offset, and copies a sequence of 16 words from a source table to a destination buffer.
- `copy_cstring`: Copies a null-terminated byte string from a source pointer to a destination pointer, byte by byte, and terminates the destination with a zero byte.
- `search_string_table`: Iterates through a table of pointers to strings and uses string comparison to find a match for a provided string.
- `find_trackdisk_device`: Calls a library function to locate a device named trackdisk.device and extracts a value from the resulting device structure, returning it combined with an input register.
- `traverse_linked_list`: Traverses a linked list or structure, checking flag bits in the returned pointer and incrementing a counter if a specific bit is set.
- `examine_and_retrieve_pointer`: Uses library calls to examine or lock a resource and, if successful, retrieves a pointer or value from an offset of a retrieved address into a pointer parameter.
- `reinitialize_string_structure`: Resets a string structure's metadata, sets certain flags, and writes a constant string or a new pointer back to a global structure.
- `compare_and_build_string_table`: Compares two null-terminated strings and, if they match, traverses a linked list of pointers to count or store string references in a newly allocated table or structure.
- `verify_and_cache_string_metadata`: Compares a string from a structure or table against another string, and if they match, traverses a linked list of pointers to calculate a size and populate a new metadata or header structure with the count and pointer references.
- `execute_indirect_calls`: Performs indirect calls to a target address retrieved from a table using a negative offset and calls a library function via a global base.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator, and returns the original destination pointer.
- `copy_string_to_buffer_2`: Copies bytes from a source pointer to a null-terminated destination buffer, returning the original destination pointer.
- `compute_string_length`: Receives a pointer to a zero-terminated byte string and returns the number of bytes before the terminator without modifying memory.
- `compare_two_byte_strings`: Compares two byte strings byte by byte and returns a value indicating if the first is lexicographically smaller, larger, or identical.
- `safe_memory_copy`: Copies a specified number of bytes from a source to a destination, handling overlapping memory regions by choosing a copy direction based on pointer addresses.
- `clear_and_update_globals_2`: Zeroes out a block of memory, updates a global or a table entry with a constant, and then copies a global variable to a new memory location.
- `return_void`: This routine is empty and performs no operations before returning.
- `parse_string_tokens`: Parses a source string into individual tokens, handling quoted strings and skipping whitespace or certain control characters.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_with_limit_2`: Calculates the length of a source string and copies bytes to a destination until a null terminator or a maximum count is reached.
- `copy_string_with_limit`: Copies bytes from a source to a destination until a null terminator or a maximum count is reached, ensuring a null terminator is placed at the end of the destination range.
- `copy_string_pad_zeros`: Copies a string from a source to a destination up to a certain count or until a null terminator is found, then fills any remaining count with zero bytes.
- `update_window_structure`: Updates a structure or object by storing a returned value into a memory offset, and recursively calls other routines to assign values to certain structure members.
- `save_and_recurse_global`: stores a value into a global variable before calling itself recursively and then clears that global variable to zero.
- `store_and_parse_string`: Stores a passed parameter into a global variable and then calls a routine to parse a format string.
- `increment_and_store_byte`: Loads a pointer from a global variable, increments that global variable, and stores a byte into the memory location pointed to by the loaded pointer.
- `recurse_with_concatenation`: Recursively calls itself while returning a 64-bit value formed by concatenating a register's previous contents with the new return value.
- `parse_printf_format_string`: Parses a format string to extract or substitute or format numeric or string arguments.
- `perform_signed_division`: Reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `call_and_return_concatenated`: Calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine, which is the same as the caller, so the routine recurses without performing any other operation.
- `perform_32bit_division`: Calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `execute_conditional_pointer`: Checks if a value at a certain offset from a pointer is non-zero and, if so, executes the function pointer located at that memory location.
- `execute_conditional_table_logic`: Checks various memory locations or table-based pointers and executes corresponding functions or jumps if the pointers or certain flags are non-null.
- `empty_routine`: The routine contains no executable instructions and performs no operations.
- `execute_table_function`: Checks if an index is within a valid range and if a pointer in a table is non-null, then executes a function pointer from a table and updates a global status variable.
- `call_indirect_offset`: Calculates an address by applying a negative offset to a pointer retrieved from a table and jumps to that location.
- `call_close_function`: Executes a function pointer retrieved from a table offset by a constant.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `execute_indirect_table_call`: Retrieves a pointer from a table using an offset, applies a negative displacement, and executes the resulting address.
- `execute_indirect_call`: calculates an address by applying an offset to a pointer retrieved from a table and jumps to that location.
- `call_dup_lock`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `call_examine`: Wrapper that calls the dos library function Examine and returns its result to the caller.
- `call_info`: Wrapper that calls the dos library function Info and returns its result to the caller.
- `execute_indirect_function`: calculates a target address by adding an offset to a pointer retrieved from a table and then jumps to that address.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `execute_indirect_function_pointer`: calculates a target address by adding an offset to a value loaded from a table and then executes the resulting pointer.
- `execute_indirect_call_2`: loads a function pointer from a table offset and executes it.
- `execute_indirect_function_2`: loads a function pointer from a table and calls it.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek_wrapper`: Executes a function pointer retrieved from a table offset by a constant, acting as a wrapper for a file seeking function.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `execute_indirect_function_3`: calculates a function pointer or jump target by adding an offset to a value retrieved from a table and executes it.
- `call_write_wrapper`: Executes a function pointer retrieved from a table offset by a constant, acting as a wrapper for a file writing function.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_indirect_function`: Loads a function pointer from a table offset and calls it.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `execute_indirect_table_pointer`: Calculates a pointer by applying a negative offset to a value retrieved from a table and executes the resulting address.
- `call_indirect_function_2`: Resolves an indirect function address from a table and executes it.
- `update_buffer_structure`: Calls a library function to get a pointer and then populates a memory structure with passed parameters and a return value.
- `update_buffer_data`: Updates a destination memory buffer with bytes and words copied from a source location, while also calling a library function and an unrelated routine to store a new pointer.
- `write_to_memory_buffer`: Writes several bytes or words to a memory location and then calls another routine, potentially branching or calling a routine with an incremented pointer.
- `recursive_offset_call`: Calls another routine with an incremented pointer value.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `execute_indirect_table_call_2`: Fetches a function pointer from a table using an offset and executes it via an indirect call.
- `call_indirect_function_3`: Performs an indirect call to a function located via a table offset.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `resolve_indirect_function_pointer`: calculates a destination address by adding a negative offset to a value loaded from a global table, then calls that address.
- `call_indirect_function_4`: Calculates a destination address using a pointer and a table offset before calling.
- `dereference_indirect_function_pointer`: Retrieves a function pointer from a table using an offset from a global or persistent base address and executes it.
- `call_indirect_function_5`: Executes a function pointer retrieved from a table relative to a base address.
- `execute_indirect_pointer`: loads a function pointer from a table using an offset and an external base address, then transfers control to that address.
- `call_indirect_function_6`: Uses a table-based pointer to perform an indirect jump or call.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_indirect_function_7`: Accesses a function address from a memory offset and branches to it.
- `setup_pointer_chain`: Stores a pointer, increments that pointer's value by four, clears a subsequent word, and stores the original pointer in a following word.
- `invoke_indirect_function`: calculates a target address by adding a negative offset to a pointer retrieved from a table and executes it.
- `call_indirect_function_8`: Retrieves an indirect address from a pointer-offset calculation and calls it.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_indirect_function_9`: Performs an indirect call using a value fetched from a table-based offset.
- `empty_routine_2`: This routine is empty and contains no instructions.
