# Game 2564 (odyssey (19xx)(mirage software)(pl)[m lsd][wb]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02564). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904896 top-level bytes in 19 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 131 of 131 named (0 author symbols), 131 of 131 described (4 by rule, 125 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1256 of 1256 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 356854 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4510 of 4510 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 258823 |
| code | 13976 |
| data | 592240 |
| padding | 5553 |
| unused | 34304 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Odyssey (19xx)(Mirage Software)(Pl)[m LSD][WB].adf:MagicznaKsiega | 2 | 28 | 2 |
| Odyssey (19xx)(Mirage Software)(Pl)[m LSD][WB].adf:PPMore | 3 | 2613 | 57 |
| Odyssey (19xx)(Mirage Software)(Pl)[m LSD][WB].adf:Installer | 3 | 235 | 8 |
| Odyssey (19xx)(Mirage Software)(Pl)[m LSD][WB].adf:NP | 2 | 267 | 8 |
| Odyssey (19xx)(Mirage Software)(Pl)[m LSD][WB].adf:Cracktro.exe | 8 | 472 | 17 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 883 | 38 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| A (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- odyssey (19xx)(mirage software)(pl)[m lsd][wb]/Odyssey (19xx)(Mirage Software)(Pl)[m LSD][WB].adf:module: high-entropy data file (7.75 bits/byte), classified as packed data
- runtime segment an address: 839 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-keys): 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- odyssey (19xx)(mirage software)(pl)[m lsd][wb]/Odyssey (19xx)(Mirage Software)(Pl)[m LSD][WB].adf:PM/A: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- odyssey (19xx)(mirage software)(pl)[m lsd][wb]/Odyssey (19xx)(Mirage Software)(Pl)[m LSD][WB].adf:MagicznaKsiega: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- odyssey (19xx)(mirage software)(pl)[m lsd][wb]/Odyssey (19xx)(Mirage Software)(Pl)[m LSD][WB].adf:Installer: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- odyssey (19xx)(mirage software)(pl)[m lsd][wb]/Odyssey (19xx)(Mirage Software)(Pl)[m LSD][WB].adf:NP: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- odyssey (19xx)(mirage software)(pl)[m lsd][wb]/Odyssey (19xx)(Mirage Software)(Pl)[m LSD][WB].adf:Cracktro.exe: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 356854 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- Odyssey (19xx)(Mirage Software)(Pl)[m LSD][WB].adf:Cracktro.exe

## Copy protection found

- `recursively_process_text_buffer` (PPMore): A recursive routine that parses a data buffer by checking specific flags and performing conditional nested calls. Kind: sabotage path. Pass path: the 'else' branch (the second block of code).
- `set_hardware_registers` (runtime segment an address): Writes specific bitmask values to custom hardware registers and DMA control registers. Kind: disk hardware/track check. Pass path: the branch where btst.b #an address, an address.l evaluates to 0 (not taken).

## Functions


### Odyssey (19xx)(Mirage Software)(Pl)[m LSD][WB].adf:MagicznaKsiega

- `initialize_system_pointers`: Calls a function via a pointer from a table and, if the result is non-zero, updates a global pointer and stores a value into a memory location.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### Odyssey (19xx)(Mirage Software)(Pl)[m LSD][WB].adf:PPMore

- `initialize_system_environment`: Allocates memory and initializes a large data structure by copying a global block and applying pointer relocations.
- `execute_callback_chain`: Checks and executes a function pointer from a structure, then performs a sequence of calls to other function pointers based on the contents of a global variable and a structure.
- `parse_command_string`: Iterates through a provided string to identify command tokens, handling whitespace, quoted substrings, and specific prefix characters like hyphens or question marks.
- `populate_data_tables`: Initializes several bytes in a structure and then enters a loop to copy data from various sources into a destination table, performing arithmetic on specific offsets.
- `initialize_graphics_context`: Retrieves graphics information from a structure, performs a swap of elements in a list if a condition is met, and updates a pointer with a value from a library call.
- `execute_conditional_loop`: Executes a loop that calls functions via pointers stored in a structure.
- `release_memory_pointer`: Checks if a pointer in a structure is non-zero, calls a function via a global pointer, and then clears that pointer.
- `cleanup_string_resources`: Checks if a pointer is non-zero and then either executes two function calls from a structure or iterates through a string and executes two function calls from a different structure.
- `cleanup_resource_structures`: Clears a bit in a word, then executes a series of function pointers stored within a structure, including routines for freeing memory and cleaning up a message port.
- `call_display_functions`: Retrieves a pointer from a structure and calls two different functions located at specific offsets from that pointer.
- `manage_print_job_window`: Manages the creation, status updates, and destruction of a window used for displaying text listing progress.
- `update_window_status`: Executes a function pointer from a structure and then writes two specific values to a structure.
- `update_status_display`: Compares values in a structure, executes a function pointer if they differ, and uses fixed-point arithmetic to update a value in a structure.
- `parse_text_control_characters`: Parses a stream of characters to identify and execute specific control sequences, including escape codes and formatting commands.
- `recursively_process_text_buffer`: A recursive routine that parses a data buffer by checking specific flags and performing conditional nested calls.
- `process_text_buffer`: Parses a buffer for control characters and line endings while managing an internal cursor and character count through various function calls.
- `process_text_buffer_2`: Checks a buffer length, adjusts a cursor position based on newline or form feed characters, and performs several I/O or text processing operations.
- `loop_buffer_elements`: Decrements a value from a structure field in a loop and conditionally calls another routine.
- `decrement_and_call`: Decrements a value from a structure in a loop and executes a routine, potentially performing one additional call based on a bit flag.
- `assign_returned_value`: Calls another routine and stores the returned value into a field within a structure.
- `reset_display_flags`: Calls a function and then clears three bytes in a structure at specific offsets.
- `wait_for_io_condition`: Enters a loop that repeatedly calls functions via a global pointer until a specific condition in a structure is met.
- `process_data_loop`: Resets several fields in a structure and then enters a loop to process data by calling another routine and updating structure values based on the results.
- `check_structure_changes`: Compares values in a structure against local variables and calls another routine if they differ.
- `calculate_scaled_values`: Performs integer division on values derived from a structure and a secondary pointer to store two new results in a different structure.
- `dispatch_display_logic`: Resets two bytes in a structure and then executes different branches of logic based on an input value, involving calls to other routines.
- `manage_printer_device`: Attempts to open a printer device and manages a message port and associated memory based on the success or failure of the operation.
- `validate_resource_string`: Checks for the existence of a resource, retrieves a string from a library call or structure, and validates the string content by checking for a specific character.
- `setup_device_parameters`: Writes two values to specific offsets within a structure pointed to by a global variable and calls a function via a global pointer.
- `execute_io_request`: Sets a command value and two data parameters in a structure before calling a function via a global pointer to perform an I/O operation.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `find_line_end_2`: Scans a buffer for newline or form feed characters and returns a pointer to the last occurrence found within a specified count.
- `count_line_feed_characters`: Iterates through a string of a specified length and counts the occurrences of newline or form-feed characters.
- `copy_string_to_destination`: Finds the end of a source string and then copies characters from a second string into the position following the first string's null terminator.
- `copy_string`: Copies a null-terminated string from a source pointer to a destination pointer.
- `find_string_forward`: Searches for a specific character sequence within a buffer by comparing bytes and returning a pointer to the match.
- `convert_ascii`: Adjusts a byte value by subtracting specific offsets to convert between character sets.
- `find_string_backward`: Searches for a character sequence by scanning backwards from a given pointer within a buffer.
- `count_special_characters`: Iterates through a memory range to count occurrences of bytes within a specific range and returns a status based on that count.
- `parse_hex_string`: Converts a hexadecimal string into its integer representation and updates a pointer to the end of the processed string.
- `scan_text_buffer`: Parses a buffer for specific control characters and line endings while managing an internal cursor and character count.
- `find_line_end`: Scans a buffer backwards to find the first occurrence of a newline or form feed character.
- `call_external_function`: Executes a function pointer stored in a global table.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `sum_nibbles`: Calculates the sum of the four nibbles contained within a 16-bit integer.
- `compare_color_depth`: Compares two values retrieved from a graphics-related routine and stores a result in a provided pointer and a global variable.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `execute_null_routine`: An empty routine that performs no operations and returns immediately.
- `prepare_and_free_memory`: Sets specific bytes and words in a memory block before calling a routine to free that memory.
- `cleanup_message_port`: Frees memory associated with a message port, clears signal bits, removes the port from the system, and resets related global state variables.
- `compute_fixed_point_product`: The routine takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.
- `calculate_fixed_point_division_2`: Performs a normalized fixed-point division of two 32-bit values, handling sign changes and bit shifting for normalization.
- `swap_and_negate_registers`: Swaps the high and low 32-bit words of a 64-bit value and negates both resulting halves.
- `calculate_fixed_point_division`: Performs a complex division operation on two values using normalization and bit shifting to produce a fixed-point result.
- `normalize_fixed_point_divide`: The routine divides a 32-bit dividend in d0 by a 32-bit divisor in d1 using a normalized fixed-point method.
- `allocate_message_block`: The routine allocates a message block using an OS library call, sets its type and length fields, and stores a pointer in a global variable if allocation succeeds.
- `initialize_message_port`: The routine allocates memory and a signal, initializes a message port structure with specific fields, and registers the port with the operating system.

### Odyssey (19xx)(Mirage Software)(Pl)[m LSD][WB].adf:Installer

- `traverse_and_copy_linked_data`: Traverses a linked list of data structures, identifying specific magic markers to perform block memory copies and data reordering.
- `extract_flag_bit_from_table`: Scans a table of word pairs until a condition on a global mask is met, then reads a byte from a table entry and uses it as an index into a global mask to extract a bit, calling another routine if a borrow condition is detected.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 255 into d0, then loads 16 into d1.
- `mask_and_conditional_call`: The routine masks a word from a register with a word from a register and, if a byte from a register is less than a byte from a register, it calls another routine and uses its returned word as the result; otherwise it returns the masked word.
- `return_empty_routine`: The routine is empty; it contains no instructions that modify memory or registers and immediately returns.
- `fill_figure_table`: Reads a count, fills a table with values from a stack buffer, and then iterates through a loop to write bit masks and derived values into a table based on a key list.
- `free_linked_list_elements`: Iterates through a list of pointers, performing calculations based on a value retrieved through a pointer chain, and updates a global table with the results.
- `return_void`: The routine is empty and performs no operations before returning.

### Odyssey (19xx)(Mirage Software)(Pl)[m LSD][WB].adf:NP

- `parse_amiga_hunk_loader`: Parses a data stream containing AmigaDOS hunk type codes to allocate memory and relocate or copy data segments.
- `decrement_and_loop`: Decrements a 16-bit value in a register until it reaches zero.
- `nop_placeholder`: This routine is empty and performs no operations.
- `extract_bitstream_logic`: Processes a bitstream by checking a flag and performing conditional memory copies and bitwise extractions based on variable lengths.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `extract_bitstream_loop`: Performs bitwise extraction from a source buffer into a destination using variable bit lengths and updates a pointer.
- `execute_trap`: Executes a software trap instruction.

### Odyssey (19xx)(Mirage Software)(Pl)[m LSD][WB].adf:Cracktro.exe

- `trigger_infinite_recursion`: Calls itself twice before returning zero, resulting in an infinite sequence of calls that eventually exhausts the stack.
- `parse_amiga_hunk_format`: Parses a memory structure containing AmigaDOS hunk type codes to perform relocation and memory allocation.
- `decrement_and_loop_2`: Decrements a value in a register until it reaches zero.
- `nop_placeholder_2`: The routine is empty and performs no operations.
- `extract_bitstream_variant`: Iteratively extracts bits from a memory source using a flag to determine whether to perform specific memory copy and bit manipulation sequences.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `extract_bitstream`: performs bitwise operations to extract bits from a source based on a variable length and updates a pointer and a destination value.
- `extract_bitstream_loop_2`: Iteratively extracts bits from a memory source based on a bitmask and updates a pointer.
- `execute_recursive_calls_2`: Calls itself twice before returning zero.
- `parse_hunk_relocation_data`: Parses a memory buffer containing hunk type codes to perform memory allocation and data relocation.
- `decrement_integer_value`: decrements a 16-bit integer stored in a register until it reaches zero.
- `return_from_relocation`: This routine is an empty function that performs no operations before returning.
- `decompress_bitstream_data`: Processes a bitstream by reading bit counts and copying or shifting data blocks within a memory buffer.
- `check_bit_status`: checks if a specific bit within a value is set and returns.
- `extract_bitstream_to_register`: Reads a sequence of bits from a memory source and reconstructs them into a target register.
- `extract_bitstream_loop_3`: Processes a bitstream by reading specific bit counts from a source and reconstructing them into a target register.
- `wait_for_fire_button_press`: polls a CIA port register until bit six reads zero, then returns zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `dispatch_global_callbacks`: Checks bits in a global variable to determine whether to call specific functions via pointers.
- `configure_hardware_registers`: Polls a status bit and then writes specific values to custom hardware registers and DMA control registers.
- `set_hardware_registers`: Writes specific bitmask values to custom hardware registers and DMA control registers.
- `initialize_data_tables`: Allocates memory and populates several tables with values derived from library calls and internal calculations.
- `initialize_hardware_parameters`: waits for a bit in a global variable to clear, then writes specific values to three memory locations.
- `setup_blitter_parameters`: waits for a bit in a global variable to clear, then writes specific values to five memory locations.
- `toggle_global_value`: toggles a bit in a global variable and assigns one of two constant values to it based on the result.
- `wait_for_memory_status_change`: Polls a global variable until its first bit changes state from zero to one and then back to zero.
- `fill_memory_with_zeros`: Repeatedly writes zero values into a series of memory locations within a loop.
- `parse_and_expand_text_stream`: Parses a stream of encoded characters to expand them into a destination buffer.
- `update_table_values`: Clamps a global variable within a specific range and then updates a series of values in a table using arithmetic derived from other table entries.
- `process_command_list`: Iterates through a list of short integers and performs different memory or register operations based on the value of each element.
- `iterate_short_array`: Compares a value against a global variable and, if a condition is met, iterates through a list of short integers.
- `store_recursive_word_sequence`: Performs a series of recursive calls and stores the resulting word values into a memory buffer at six-byte increments.
- `write_recursive_data`: Performs a series of recursive calls and stores word values into a memory buffer at increments of six bytes.
- `execute_recursive_calls`: This routine calls itself three times in succession before returning.
- `perform_no_operation`: The routine is empty and performs no actions before returning to the caller.
- `wait_and_set_registers`: Polls a global variable bit until it clears, then writes specific values to several hardware registers.
- `calculate_bounding_box`: Performs bounds checking on input parameters and calculates dimensions and coordinates to store in a structure pointed to by a register.
- `build_data_structure`: Calculates a running sum from a table of words and stores the results into a new memory area.
- `clear_audio_and_dma`: Clears four audio volume registers, masks a global variable, writes a constant to a global variable, writes a constant to a custom chip register, and copies a value from a table into a global variable.
- `update_audio_parameters`: Iterates through a table of data to modify volume, pitch, or frequency values based on specific command types.
- `execute_indirect_jump`: Performs an indirect jump to an address contained within a jump table.
- `decode_compressed_data`: Decompresses a bitstream using a variable-length encoding scheme.
- `decode_bitstream_data`: Parses a compressed bitstream by shifting bits and performing indirect jumps through a jump table based on decoded values.
- `decode_bitstream_data_2`: Parses a compressed bitstream by shifting bits and performing lookups in a table.
- `decode_bitstream_data_3`: Parses a compressed bitstream by shifting bits from a data register and performing lookups in a table.
- `process_joystick_input`: Reads the status of the joystick or fire buttons from the CIAA port and updates a global state.
- `decrement_and_dispatch`: Subtracts a value from a field in a structure, clamps the result to a minimum value, and then calls a function from a table using an index.
- `increment_and_dispatch`: Adds a value to a field in a structure, clamps the result to a maximum value, and then calls a function from a table using an index.
- `dispatch_indexed_function`: Calculates an index based on values from a structure and a table to select and call a function.
- `update_state_from_table`: Calculates a value using a table lookup and arithmetic, updates a global variable, and modifies a field in a structure before calling another routine.
- `calculate_and_update_state`: Calculates a value using a table lookup and arithmetic, updates a global variable, and modifies a field in a structure before calling a function from a table.
- `update_accumulator_and_dispatch`: Adds a value to a field in a structure, clamps it, and then calls a function from a table using an index.
- `update_accumulator_value_2`: Modifies a value in a structure by adding or subtracting a constant based on a flag, clamps the result within a specific range, and updates a global variable.
- `update_accumulator_value`: Adds or subtracts a value from a field in a structure, clamps the result, and then calls a function from a table using an index.
- `configure_cia_dma_interrupts`: Initializes a sequence of global variables with constants, a self-reference, a pointer to a RAM table, and a mask.
- `set_audio_channel_lengths`: Writes eight 16-bit values from a table into a global variable sequentially, then writes a 16-bit constant to that same global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `invoke_library_functions`: Retrieves a pointer from a global variable to call a function, then uses the returned value to call two subsequent functions via offsets.
