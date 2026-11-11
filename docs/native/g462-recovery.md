# Game 462 (Amberstar): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00462). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2725632 top-level bytes in 79 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 24 of 24 units identical (bytes and relocations), 29 raw-word lines |
| Functions named and described | pass, 773 of 773 named (0 author symbols), 773 of 773 described (93 by rule, 636 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1797 of 1797 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 967094 bytes, 47 functions |
| Independent decoder cross-check (Ghidra against capstone) | 16732 of 17068 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1470211 |
| code | 49744 |
| data | 847042 |
| padding | 17643 |
| unused | 340992 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| More | 15 | 2682 | 73 |
| Type | 2 | 117 | 2 |
| EndCLI | 2 | 117 | 2 |
| LoadWB | 12 | 538 | 30 |
| WaitForMouse | 1 | 4 | 1 |
| Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:Format Game Disks | 3 | 2625 | 141 |
| icon.library | 10 | 1375 | 109 |
| info.library | 25 | 2598 | 167 |
| Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:add21k | 1 | 117 | 4 |
| Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:IconX | 12 | 806 | 37 |
| Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:Amber*_Install | 2 | 576 | 28 |
| Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:FILE01 | 2 | 788 | 33 |
| Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 397 | 18 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 668 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 260 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 735 | 27 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 378 | 29 |
| Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:AMBERDEV.UDO#unpacked (raw code file, load address unknown) | 1 | 417 | 3 |
| Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:INSTALL.UDO#unpacked (raw code file, load address unknown) | 1 | 216 | 1 |
| Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:EXTRO.UDO#unpacked (raw code file, load address unknown) | 1 | 1217 | 28 |
| Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:INTRO_P.UDO#unpacked (raw code file, load address unknown) | 1 | 392 | 15 |

## Cases handled

- amberstar (1992)(thalion)(disk 1 of 3)[fd-hd]/Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:AMBERDEV.UDO: LOB: LOB's File Compressor-packed, unpacked with ancient (193554 -> 370408 bytes)
- amberstar (1992)(thalion)(disk 1 of 3)[fd-hd]/Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:TH_LOGO.UDO: LOB: LOB's File Compressor-packed, unpacked with ancient (25118 -> 40000 bytes)
- amberstar (1992)(thalion)(disk 1 of 3)[fd-hd]/Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:INSTALL.UDO: LOB: LOB's File Compressor-packed, unpacked with ancient (71746 -> 117520 bytes)
- amberstar (1992)(thalion)(disk 1 of 3)[fd-hd]/Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:INTRO.UDO: LOB: LOB's File Compressor-packed, unpacked with ancient (38674 -> 46400 bytes)
- amberstar (1992)(thalion)(disk 1 of 3)[fd-hd]/Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:EXTRO.UDO: LOB: LOB's File Compressor-packed, unpacked with ancient (136080 -> 238074 bytes)
- amberstar (1992)(thalion)(disk 1 of 3)[fd-hd]/Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:INTRO_P.UDO: LOB: LOB's File Compressor-packed, unpacked with ancient (168540 -> 241092 bytes)
- runtime segment an address (play-rs-m1500): 14 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 349 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 538 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 249 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 541 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 11 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 91 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- amberstar (1992)(thalion)(disk 1 of 3)[fd-hd]/Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:AMBERDEV.UDO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- amberstar (1992)(thalion)(disk 1 of 3)[fd-hd]/Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:INSTALL.UDO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- amberstar (1992)(thalion)(disk 1 of 3)[fd-hd]/Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:EXTRO.UDO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- amberstar (1992)(thalion)(disk 1 of 3)[fd-hd]/Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:INTRO_P.UDO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- amberstar (1992)(thalion)(disk 2 of 3)[fd-hd]/Amberstar (1992)(Thalion)(Disk 2 of 3)[FD-HD].adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- amberstar (1992)(thalion)(disk 3 of 3)[fd-hd]/Amberstar (1992)(Thalion)(Disk 3 of 3)[FD-HD].adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- 4 never-loaded region(s), 967094 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_subsystems` (AMBERDEV.UDO#unpacked (raw code file, load address unknown)): Performs a sequence of function calls while updating various global variables and memory locations. Kind: sabotage path / integrity trap. Pass path: the loop continues indefinitely, but the 'Too much modifications' error suggests the protection is triggered by an external integrity check or a checksum mismatch during the execution of these subroutines..

## Functions


### More

- `parse_command_string_2`: Parses a null-terminated string into a structured array of pointers and lengths by splitting the input based on whitespace and special control characters.
- `invoke_indirect_pointers`: Invokes two function pointers stored at specific offsets within a structure pointed to by a register.
- `call_indirect_pointers`: Invokes two function pointers stored at offsets within a structure pointed to by a register.
- `manage_task_state`: Updates a value in a structure and calls several function pointers based on the results of pointer dereferencing.
- `process_text_buffer_stream`: Parses and manages a text buffer by updating its internal state, control sequences, and record chains.
- `iterate_global_table`: Executes a loop five times, calling another routine with a value from a pointer and a value from a global table.
- `validate_buffer_content`: Adjusts a buffer via a pointer and then scans a range of bytes to check if any byte falls within specific numeric ranges.
- `parse_text_control_sequences`: Parses a stream of characters to identify and execute control sequences such as form feeds, carriage returns, and specific formatting symbols.
- `update_buffer_state_2`: Modifies a structure by setting specific flags and updating buffer offsets and lengths.
- `update_buffer_state`: Modifies a buffer using a text parsing routine, stores a return value into a structure, and increments a value in a linked structure if a specific field is zero.
- `parse_line_and_replace_formfeed`: Adjusts a buffer to find a line of text, then searches for a form feed character to replace it with a specific sequence of characters.
- `process_text_buffer_sequence`: Determines a loop count based on specific flags and values within a structure, then iterates through a table to process data elements.
- `process_text_buffer`: Iterates through a buffer, performing string comparisons and character masking while checking for specific status flags in a loop.
- `execute_conditional_chain`: Calls several routines using values from a pointer and global variables, including reading a string into a buffer and managing a chain of records.
- `process_string_data`: Copies a null-terminated string and then performs a bitwise AND operation on specific characters within a buffer.
- `copy_text_with_offset`: Calls a text copying routine with specific parameters and then calls another routine to adjust a buffer offset.
- `copy_text_half_offset`: Calls a text copying routine and then calls another routine to adjust the buffer offset by half of a value found in a linked structure.
- `copy_text_remaining_half`: Calls a text copying routine with an offset calculated by subtracting half of a value from itself.
- `reset_buffer_offset`: Calls a text copying routine with a zero offset and then calls another routine to adjust the buffer offset to a value minus one.
- `copy_text_with_line_limits`: Copies a block of text into a buffer in chunks, monitoring for newline or form feed characters to manage buffer state and line counts.
- `pass_parameters_to_stack_2`: Calls another routine and then passes a pointer from a structure along with a parameter to a third routine.
- `set_flag_and_call`: Sets a word in a linked structure to one if a specific field is zero, then calls another routine using a value from a pointer and a global variable.
- `call_with_conditional_index`: Calls another routine using a value from a pointer and a global variable, optionally providing a third parameter derived from a linked structure.
- `call_with_conditional_count`: Calls another routine using a value from a pointer and multiple global variables, or uses a value from a linked structure as a parameter.
- `execute_indirect_calls`: Retrieves a pointer from a global variable, calls the function at a specific offset, and then calls another function using the result of the first call.
- `update_structure_counters`: Compares a value against a field in a structure and, if conditions are met, updates three different fields within that structure using arithmetic operations.
- `update_structure_values`: Checks specific values within a structure and, if conditions are met, calls a function and updates several fields in the structure using values from other fields and a function return value.
- `adjust_buffer_offset`: Calculates a new index based on a provided offset and mode, then updates a structure containing buffer pointers, lengths, and current positions.
- `parse_control_sequence`: Reads a sequence of bytes from a source via another routine and stores them into a buffer, returning a status code based on the sequence content.
- `read_string_to_buffer`: Reads bytes from a source into a destination buffer until a newline or termination character is encountered, up to a maximum length.
- `invoke_library_function`: Checks if two parameters are non-zero and then calls a library function using a base address from a global variable.
- `copy_string`: Copies a null-terminated string from a source buffer to a destination buffer.
- `mask_string_characters`: Iterates through a buffer and performs a bitwise AND operation on specific byte values to clear a bit.
- `parse_numeric_expression`: Parses a string containing a sign and numeric characters, converting the characters into an integer value.
- `process_task_structure`: Updates a structure by executing various function pointers, parsing control sequences, and modifying specific fields within the structure.
- `manage_record_chain_2`: Calls a library function to retrieve a pointer and then uses that pointer to manage a chain of records through another routine.
- `extract_record_data`: Retrieves a pointer from a structure, calls a routine to process it, and if successful, copies specific data from the resulting structure into a destination parameter.
- `manage_record_chain`: Recursively manages a chain of records by allocating memory, populating a structure with data from a pointer, and updating linked list pointers.
- `process_text_input`: Reads a string into a buffer and uses a function pointer to dispatch a task.
- `call_dispatch_function`: Calls a function pointer located at an offset from a global variable.
- `allocate_record_block`: Allocates a 34-byte block of memory and initializes it with provided parameters and a unique identifier.
- `call_library_function`: Calls a function from a library using a base address stored in a global variable.
- `divide_fixed_point`: Performs signed 32-bit fixed-point division using bit shifting and normalization logic.
- `pass_parameters_to_stack`: Passes two parameters to another routine using a pointer to a local stack location.
- `release_record_chain_2`: Recursively processes a linked chain of records by setting specific bytes and words to a constant value and then calling itself on the next link.
- `execute_scheduled_routine`: Calls another routine with three arguments and returns either a byte from the stack or a negative value based on the result.
- `update_task_state`: Updates a structure and executes various function pointers stored within that structure and a global variable.
- `compare_strings_with_newline`: Compares two null-terminated strings byte by byte, provided the first string does not contain a newline character.
- `invoke_table_entry_3`: Invokes a function pointer retrieved from an offset within a global variable.
- `multiply_fixed_point_values`: Calculates the product of two 32-bit fixed-point values using their high and low 16-bit components.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_at_pointer`: Stores a byte from a register into the memory location pointed to by a register.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `initialize_linked_list_node`: Sets the second and third elements of a structure to zero and its own address, then sets the first element to point to the next node.
- `compute_fixed_point_product`: The routine takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.
- `divide_fixed_point_values`: Performs a normalized fixed-point division of a 32-bit dividend by a 32-bit divisor, handling sign logic and bit shifting.
- `normalize_fixed_point_divide`: The routine divides a 32-bit dividend in d0 by a 32-bit divisor in d1 using a normalized fixed-point method.
- `compare_strings_with_wrap`: Compares two null-terminated strings byte by byte, skipping the check if the first string contains a newline character.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_scheduled_routine`: Calls another routine with three arguments and processes the result based on the returned value.
- `call_other_routine_with_stack`: Calls another routine, passing two incoming parameters and a pointer to a local stack slot.
- `allocate_indexed_record_block`: Obtains a slot index from an unnamed routine and, when valid, allocates a 34-byte block through a library call with flags 65537.
- `release_record_chain`: Recursively releases a linked chain of records, processing the linked record first when its link field is set.

### Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_data_block_2`: Copies a fixed number of longwords from a source table to a destination pointer and then calls another routine.

### EndCLI

- `initialize_command_context`: Traverses a linked list to find the maximum value in a set of entries and populates a structured data block.
- `copy_data_and_call_function`: Copies sixteen bytes from a memory location to a destination pointer and then executes a function via a pointer.

### LoadWB

- `initialize_workbench_environment`: Initializes the application environment by retrieving system directory information and parsing command-line arguments.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `initialize_workbench_data`: Performs complex data initialization involving string comparisons, linked list construction, and multiple library function calls to manage system resources and memory.
- `compare_null_terminated_strings`: Compares two zero-terminated strings byte by byte and returns minus one when the first is lower, one when it is higher, and zero when they match.
- `build_linked_list_from_table`: Walks a table of pointer and value pairs, allocating a fixed-size node for each entry through a memory allocation call and storing the looked-up value in it.
- `execute_indirect_function`: Fetches a function pointer from a table using a fixed offset and executes it.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_indirect_function`: Fetches a function pointer from an offset within a global structure and executes it.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `compare_workbench_version`: Compares a string in a register with a global string.
- `write_space_sequence`: Stores five specific bytes into a memory location starting at an offset from a pointer held in a register.
- `compare_null_terminated_strings_2`: Compares two strings byte by byte until a mismatch is found or a null terminator is reached.
- `stub_return_only`: Contains only a return instruction, with no calls, memory access or hardware use, so it does no work.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_reschedule_callback`: Calls a function pointer located at a specific offset within a global structure.
- `call_other_routine_with_stack_2`: The routine calls another routine, passing the two incoming parameters and a pointer to a local stack slot.
- `invoke_global_and_checksum_stack`: Invokes a function pointer stored in a global variable and computes a rolling checksum over the local stack buffer until a null terminator is found.
- `store_byte_to_address`: Stores a single byte from a register into a location pointed to by another register.

### WaitForMouse

- `wait_for_bit_clear`: Enters a loop that continues as long as a specific bit in a global variable remains set.

### Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:Format Game Disks

- `initialize_global_state`: Clears a block of memory using a loop and performs indirect calls to functions retrieved from a global variable.
- `format_disk_tracks`: Initializes data structures and performs a sequence of disk operations including track formatting, verification, and status updates.
- `execute_callback_chain_2`: Checks several memory offsets for non-zero values and executes the corresponding function pointers with specific arguments.
- `set_status_and_call`: Stores a word value into a structure offset and calls another routine using a pointer from a global variable.
- `set_status_and_check`: Stores a word value into a structure offset, calls another routine, and returns whether a specific offset in that structure is non-zero.
- `store_parameter_and_call`: Stores a longword and a word into a structure at specific offsets before calling another routine.
- `initialize_structure_fields`: Stores a constant longword, a word, a calculated product, and a value from a global variable into a structure.
- `update_table_and_loop`: Writes values to a structure via a pointer, calls a library function, and then executes a loop that calls another routine multiple times based on an input parameter.
- `clear_bit_in_table`: Calculates an index to clear a specific bit within a longword in a table based on a provided value.
- `set_bit_and_increment`: Calculates an index to set a specific bit within a longword in a table and increments a global counter.
- `call_library_function_2`: This routine invokes a function from a library by retrieving a base address from a global variable and applying a specific offset.
- `sum_table_elements`: Iterates through a table of longwords to calculate a sum and stores the result in a specific index of that table.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `initialize_and_parse_command`: Clears a memory location by storing zero and then calls a routine to parse a command string.
- `store_conditional_word`: Checks a bit in a byte at a memory location and either stores a word from another routine or a constant value into a destination memory location.
- `store_returned_word`: Obtains a word from a routine and stores it into a memory location calculated from a register.
- `copy_and_return_word`: Calls another routine to obtain a word, stores that word into a memory location calculated from a register, and returns the stored value.
- `parse_command_string`: Parses a command string by identifying specific character sequences and control characters to extract numeric values or command identifiers.
- `poll_function_loop`: Repeatedly calls a function pointer from a global variable until a bitwise condition is met, then returns a value based on the final result.
- `process_string_and_multiply`: A routine consisting of 83 instructions that calls a routine to find a byte in a string and another to multiply 32-bit values.
- `loop_and_modify_byte_2`: Repeatedly calls another routine and, if a specific value is returned, decrements a longword and sets a bit in a byte.
- `decrement_and_set_bit`: Decrements a longword at a pointer and sets a specific bit in a byte located at an offset from that pointer.
- `loop_and_modify_byte`: Repeatedly calls another routine in a loop, and if a specific value is returned, decrements a value at a pointer and sets a bit in a byte at an offset.
- `decrement_counter_and_set_flag`: Decrements a longword at a pointer and sets the fourth bit of a byte located at an offset of three bytes from that pointer.
- `fetch_and_increment_byte`: Fetches a byte from a location pointed to by a pointer stored in a structure and increments that pointer.
- `get_and_increment_pointer`: Fetches a byte from a location pointed to by a pointer stored in a global variable, then increments that global variable.
- `traverse_and_update_linked_structure`: Checks bit flags within a structure and recursively processes linked elements or iterates through a memory range of similar structures.
- `update_structure_status`: Iterates through a memory range to check bit flags and updates a structure's pointers and status bits based on the results of a subroutine.
- `set_bit_in_byte`: Sets a specific bit in a byte at an offset from a pointer and returns a value of all ones.
- `update_pointer_and_return_word`: Updates two values at a pointer location using a third value and a short, increments the pointer, and returns a concatenated 32-bit value masked to a specific pattern.
- `call_decrement_and_copy_byte`: This routine calls another routine that decrements a value through a pointer and copies a byte from an offset of a second pointer to a new location.
- `decrement_and_copy_byte`: Decrements a value through a pointer and then copies a byte from an offset of a second pointer to a new location.
- `find_byte_in_string`: Scans a zero-terminated string from its start for the first occurrence of the given byte and returns its position, or zero when the terminator comes first.
- `validate_disk_status`: A nineteen-instruction routine that calls a library function to check a status bit and performs conditional logic based on a global variable.
- `validate_and_read_disk_data`: Checks a bitmask in a data structure to determine if a specific condition is met, then either returns an error code or recursively calls another routine to read data.
- `execute_read_with_error_handling`: Calls a routine to read data and, if the result indicates an error, calls another routine to retrieve an error message and stores it in a global variable.
- `dispatch_disk_operations`: This routine acts as a central dispatcher that validates the current disk status and then either initializes the disk formatting process or returns without action.
- `return_from_subroutine_4`: The routine is empty and performs no operations before returning.
- `initialize_disk_format`: Allocates memory, initializes a structure with specific bit flags, and performs a sequence of disk operations including track formatting and status updates.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `parse_quoted_string_list`: Parses a null-terminated string into a list of individual tokens, handling whitespace and quoted substrings.
- `copy_string_with_limit_2`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `manage_window_resources`: Performs a series of library calls involving icon and window structures to retrieve and store specific values into a structure.
- `push_stack_pointer`: The routine pushes the current stack pointer onto the stack before returning.
- `parse_format_string_and_copy`: Parses a format string and copies a byte from a table into a destination pointed to by a global pointer.
- `concatenate_and_return`: Calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the called routine.
- `parse_format_string`: Parses a format string containing percent-encoded specifiers to extract and convert data from a source buffer.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `perform_32bit_division`: Calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `invoke_linked_list_management`: This routine calls another routine to modify a structure by updating its pointers and bitwise flags.
- `manage_linked_list_node_2`: This routine modifies a structure by updating pointers and bitwise flags, or clears the structure and returns a failure code if certain conditions are met.
- `return_to_caller`: This routine performs no operations and immediately returns to the calling function.
- `increment_pointer_and_store`: A fifteen-instruction routine called by another routine to perform an increment and store operation.
- `increment_and_store_byte`: The routine increments a pointer stored in a global variable, reads a byte from an offset of another location, and stores that byte at the address pointed to by the original pointer.
- `iterate_and_clear_structure`: Traverses a memory range using a loop and calls another routine to clear contents within that range.
- `recursively_clear_structure`: Traverses a nested structure based on bit flags and clears the contents of the structure by zeroing out its fields.
- `clear_structure_data`: Recursively traverses a structure based on bit flags and calls subroutines to process nested elements.
- `manage_linked_list_node`: Modifies a structure by updating pointers and bitwise flags, or clears the structure and returns a failure code if certain conditions are met.
- `update_linked_list_node_3`: Modifies fields within a structure pointed to by a register, including updating pointers and bitwise flags.
- `update_linked_list_node_2`: Modifies a structure's fields and pointers based on conditional checks of its internal values.
- `process_linked_list_node`: Updates a structure by modifying pointers, counters, and status bits based on specific field values and the results of other routines.
- `update_linked_list_node`: The routine updates a pointer and a counter within a structure, modifies a bit in a status byte, and stores a byte from another location into the memory pointed to by the structure.
- `search_and_clear_list_2`: Iterates through a list of structures, checking a byte at a specific offset, and clears three consecutive longwords in a structure if the byte is zero.
- `search_and_clear_list`: The routine iterates through a list of structures, checking a byte at a specific offset, and clears three consecutive longwords in a structure if the byte is zero.
- `clear_structure_fields`: The routine sets three consecutive longwords at a location pointed to by a global variable to zero.
- `update_status_flags`: Checks the return value of a subroutine to modify specific fields and bit flags within a structure.
- `update_structure_flags_2`: Writes a value to a structure offset, modifies a byte via a bitwise OR, and updates another field based on the result of another routine.
- `traverse_and_call_list_2`: Iterates through a linked list, calling another routine for each element, and finally sets a global variable to zero.
- `traverse_and_call_list`: The routine iterates through a linked list, calling another routine for each element, and finally sets a global variable to zero.
- `insert_linked_node`: Inserts a new structure into a linked list by copying two longwords and updating a global pointer.
- `update_global_pointer_list`: The routine copies two longwords from two different locations into a new structure, updates a global variable with this structure, and returns a pointer to the third element.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `remove_linked_list_node`: Traverses a linked list to find a specific node and removes it by updating the pointers of the preceding node.
- `search_linked_list`: Traverses a linked list to find a specific node and copies a value to a destination pointer or a global variable.
- `conditionally_copy_pointer_value`: Checks if a pointer is null; if so, it stores a value into a memory location calculated from a constant offset and a register.
- `store_value_indirect`: Stores a value from a register into a memory location calculated using a pointer and an offset.
- `validate_index_range`: Checks if a provided index is within the bounds of a table and executes a function via a pointer retrieved from that table.
- `check_is_interactive`: Calls a routine using a calculated address from a table and returns whether the result is non-zero.
- `return_zero_5`: This routine is empty and simply returns a zero value to the caller.
- `delete_file_with_error_check`: Calls a routine to delete a file and, if the result is zero, retrieves an error code and stores it in a global variable.
- `return_zero_6`: This routine is empty and simply returns zero to the caller.
- `perform_write_operation_2`: A 19-instruction routine that is called by several functions including those for writing data and error checking.
- `perform_write_operation`: Attempts to write data and updates a global variable with an error code if the operation fails or if specific bit flags are set.
- `write_data_with_error_check`: Attempts to write data using a DOS function and stores the result in a global variable if an error occurs.
- `return_nothing`: Returns without performing any action.
- `check_library_status`: Calls a library function and checks a specific bit in the return value, potentially calling another routine if a global variable is non-zero.
- `call_nested_routine`: Calls another routine and returns zero.
- `chain_result_to_output`: Passes the result of a function call to a write wrapper followed by an output wrapper.
- `execute_callback_pointer`: Checks if a value at a specific offset in a structure is non-zero and, if so, executes the function pointed to by that value.
- `execute_callback_chain`: Iterates through a loop and executes several conditional function pointers and library routines stored within a data structure.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_file_handle`: Validates an index against a table, calls a routine to close the associated file, and clears the entry in the table.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_offset_function`: Calls a function located at a specific offset within a structure pointed to by a value in a register.
- `call_library_function_3`: Calls a function from a library using an offset retrieved from a global pointer.
- `initialize_data_structure`: Performs multiple nested function calls to populate a structure with various parameters and a pointer retrieved from another routine.
- `release_and_reset_record`: If a pointer at offset 10 is non-zero, calls a routine to release it, then sets a byte at offset 8 and a long at offset 14 to all ones before calling two other routines.
- `execute_indirect_function_2`: Retrieves a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `execute_indirect_function_3`: Retrieves a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `create_display_entry`: Calls another routine passing the input parameter and a fixed constant value of 48.
- `destroy_display_entry`: Calls another routine passing the input pointer as an argument.
- `update_structure_fields`: Calls a routine to obtain a pointer and, if successful, populates that pointer's memory with a byte, a word, and a long.
- `reset_structure_values`: Sets a byte at offset 8 and two long values at offsets 14 and 18 to all ones, then calls another routine using a word from offset 12.
- `execute_nested_function_pointer`: Accesses a global variable to retrieve a pointer, applies a negative offset to that pointer, and then executes a function located at a second nested offset within the resulting structure.
- `execute_indirect_function_4`: Retrieves a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `execute_indirect_library_function`: Retrieves a function pointer from a nested offset within a structure pointed to by a base address and executes it.
- `call_library_function_4`: Performs an indirect call to a function located within a library structure.
- `call_library_function_5`: Executes a function pointer retrieved from a specific offset in a library table.
- `execute_indirect_library_function_2`: Retrieves a function pointer from a nested table structure using an offset from a base address and executes it.
- `call_library_function_6`: Triggers an indirect jump to a routine stored in a library data structure.
- `call_library_function_7`: Calls a function by accessing a pointer through a multi-level offset calculation.
- `call_library_function_8`: Invokes a routine via a pointer found within a library table.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_function_9`: Performs an indirect call to a function using an offset from a base address.
- `call_library_function_10`: Executes a function pointer retrieved from a library structure.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_function_11`: Calls a routine through an indirect pointer located in a library table.
- `call_library_function_12`: Performs an indirect call to a function using a pointer from a table.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.

### icon.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `set_icon_flag_2`: As library code, this routine sets the eighth bit of a byte within a structure or performs a linked list manipulation if a specific condition is met.
- `set_icon_flag`: As part of the icon library, this routine sets the eighth bit of a byte at a specific offset within a structure.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_handler_and_store_results`: This library code calls a function through a pointer and stores the result and two other values into a memory structure.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `return_from_routine`: This library code is an empty routine that immediately returns.
- `manage_icon_chain`: This library routine calls another routine to manage an icon chain.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `invoke_icon_structure_update`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `manage_icon_chain_2`: This library routine calls another routine to manage an icon chain.
- `recurse_with_incremented_parameter`: Increments a parameter and calls another routine with the updated value before returning.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_icon_structure_update_2`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `execute_recursive_call_2`: The routine performs a single call to another routine before returning.
- `trigger_recursive_call`: Initiates a recursive sequence by calling itself without modifying any state or registers before the return.
- `call_other_routine`: This library code calls another routine and then returns.
- `search_pattern_list_2`: As part of library code, this routine calls another routine and returns.
- `convert_string_to_numeric`: This routine invokes a library function to transform a sequence of characters into a numeric value.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `traverse_linked_list`: Recursively traverses a linked data structure.
- `rebuild_icon_structure_2`: As part of library code, this routine manages data copying and updates a structure's internal pointers and fields based on the results of other routines.
- `copy_and_update_structure`: Library code that copies a block of data from one location to another and updates a pointer in a structure if it is non-zero.
- `return_zero_stub`: The routine is an empty stub that immediately returns the constant zero.
- `copy_icon_data_to_stack`: This library routine copies ten longwords from an offset within a structure pointed to by the third argument into a local array on the stack, then copies several additional fields from the same structure into other stack slots and finally calls another routine with the first two arguments and a pointer to a stack-resident header.
- `update_icon_image_data`: Rebuilds an icon structure by allocating or updating image buffers and managing associated linked lists.
- `rebuild_icon_structure`: Updates an icon structure by re-evaluating its internal members and linked list elements.
- `update_icon_list`: Library code that updates icon structure members and executes iterative callbacks while traversing linked lists.
- `execute_iterative_callbacks`: This library routine repeatedly calls a function pointer while decrementing a counter in a structure.
- `call_and_return_zero`: This library routine calls another routine and then returns zero.
- `empty_routine`: The routine contains four instructions and performs no observable work; it ends immediately with a return.
- `process_icon_info_structure_2`: Iterates through a complex icon information structure, executing callback routines for various linked lists and pointer arrays contained within the structure.
- `process_icon_info_structure`: Iterates through a series of pointers and linked lists within an icon information structure to execute associated callback routines.
- `process_icon_callbacks`: Library code that iterates through a table of pointers and executes callbacks while managing structure offsets.
- `iterate_and_callback_table`: Library code that traverses a null-terminated table of pointers, executes a callback for each entry, and performs conditional logic based on the results.
- `iterate_icon_table_entries`: Library code that walks a null-terminated table of pointers, calling a routine for each entry, updating a counter, and invoking a callback while potentially processing a secondary entry.
- `traverse_linked_list_2`: Library code that recursively traverses a linked list structure.
- `iterate_linked_list_nodes`: The routine traverses a linked list structure by following pointers stored at an offset from the current node.
- `increment_param_and_recurse`: Library code that calls another routine with the same first argument and the second argument increased by a constant, then returns.
- `execute_conditional_logic`: As part of library code, this routine calls a function and, if the result is non-zero, executes two subsequent routines with modified parameters before potentially calling a third routine with four parameters.
- `return_zero_constant`: Library code that consists of a single call to another routine and then returns.
- `return_zero_3`: Library code that returns a constant value of zero.
- `update_icon_data`: Library code that decrements a short counter and uses the result to store two longwords into a table.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, then uses the counter value as an index into a table pointed to by a global variable.
- `manage_library_resource`: As part of library code, this routine calls several other routines to manage data and handle conditional logic involving a retrieved value.
- `return_immediately_2`: Library code that returns to the caller without performing any operations.
- `call_library_routine_and_return_zero`: Library code that calls another routine and then returns zero.
- `perform_incremental_recursion`: Library code that performs a recursive call with an incremented parameter until the callee stops the recursion.
- `find_delimiter_in_list_2`: As part of library code, this routine traverses a list of pointers to find a string containing a specific delimiter character.
- `find_string_delimiter`: Library code that traverses a list of pointers to find a character matching a specific delimiter.
- `find_delimiter_in_list`: Library code that traverses a list of pointers to find a string containing a specific delimiter character.
- `search_pattern_list`: Library code that iterates through a list to find a matching pattern and returns a calculated offset or a success flag.
- `search_string_for_pattern`: Library code that iterates through a string to find a specific character sequence or pattern.
- `search_list_elements`: This library routine iterates through a list to find specific matching elements.
- `calculate_list_offset`: This library routine uses a loop to perform calculations involving a counter and other routine calls, eventually returning one or zero.
- `search_pattern_in_list`: As part of library code, this routine iterates through a list of values to find a specific pattern or match based on calculated offsets.
- `parse_numeric_string_4`: Library code that converts a character sequence into a numeric value and copies the resulting data into a destination buffer.
- `parse_numeric_string_3`: As part of library code, this routine compares memory blocks and converts a sequence of characters into a numeric value.
- `parse_numeric_string_2`: Library code that converts a string of characters into a numeric value by iterating through the string and multiplying the accumulator by ten for each digit.
- `parse_numeric_string`: This library routine converts a string of characters into a numeric value.
- `validate_range`: This library routine checks if a parameter falls within a specific range and returns an offset or negative one.
- `return_all_ones`: This routine returns a value consisting of all bits set to one.
- `invoke_library_offset_function`: This library routine calls a function located at a specific offset from a global library base.
- `invoke_alternative_library_offset`: This library routine calls a different function located at a different offset from a global library base.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: The library routine forwards a set of four parameters to another routine, checks the returned value, and if non-zero calls the same routine again with a modified argument list.
- `set_library_value`: Library code that calls a function and, if successful, writes a value of 10 to a specific offset in a structure and a provided pointer.
- `return_zero`: This library code returns zero.
- `validate_parameter_match`: This library code calls another routine and returns zero if the returned value does not match the fourth parameter.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `compare_and_zero`: This library routine compares a returned value with a parameter and returns zero if they are not equal.
- `return_from_subroutine_5`: This routine is empty and performs only a return instruction.
- `perform_large_math_division`: Library code that performs division on a 64-bit value using a loop or modular arithmetic depending on the input size.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry_2`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_at_pointer_2`: This library routine stores a byte from a register into a location pointed to by a register.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `find_character_in_string`: This library routine searches for a specific character within a string and returns a pointer to its location.
- `return_zero_7`: This library routine returns zero.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `copy_string_data`: Library code that finds the end of a string and then copies the characters from a source to a destination.
- `compare_memory_blocks`: This library routine compares two memory blocks up to a specified length and returns one if they match or zero otherwise.
- `return_one_2`: This library routine returns one.
- `decrement_and_call`: This library routine iterates through a string and calls another routine for each character until a null terminator or a counter limit is reached.
- `call_and_return`: This library routine calls another routine and then returns.
- `copy_and_pad_string`: Library code that copies a string up to a specified length and then pads the remaining space with null bytes.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `process_icon_request`: Library code that validates a parameter, calls another routine to check a return value, and if successful, performs further calls using a value from a pointer.
- `traverse_linked_list_nodes`: Iterates through a linked list by following pointers stored within each node.
- `loop_until_zero`: Library code that repeatedly calls a function and checks if a value at a specific offset in a structure becomes zero.
- `return_zero_2`: This routine is an empty function that returns zero to the caller.
- `update_icon_structure`: This library routine performs a series of checks and calls to update values within a structure based on multiple parameters.
- `return_negative_one`: This library code returns a value of all bits set to one.
- `relink_icon_list_nodes`: Iterates through a linked list to update node pointers and recalculate offsets based on structure member values.
- `traverse_icon_list`: This library routine iterates through a linked list, updating structure members and returning a value or negative one based on the traversal results.
- `traverse_icon_list_2`: Library code that iterates through a linked list, performing several checks and updating structure members during the traversal.
- `return_negative_one_2`: Library code that returns a constant value of negative one.

### info.library

- `return_empty_routine`: This routine is empty and performs no operations.
- `store_library_state`: The routine stores the incoming register values into a global variable and a table, and writes a pointer through an offset in a structure, then stores that pointer into another global variable.
- `propagate_icon_change_2`: This library routine forwards a set of four parameters to another routine and checks the returned value.
- `call_library_helper_2`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_structure_flag_update`: This library routine acts as a wrapper that calls a function to update specific flags within a data structure.
- `empty_routine_2`: This library routine is empty.
- `reset_library_state_recursive`: This library code calls itself indefinitely, resulting in infinite recursion, before finally calling a routine that resets the library state.
- `clear_flag_and_increment_counter`: The routine clears a specific bit in a byte field of a structure pointed to by its argument and increments a 16-bit counter field within the same structure.
- `update_structure_flags`: This library routine checks a value within a structure and either performs a series of calls using offsets or sets a specific bit in a byte within that structure.
- `call_library_functions_with_offset_args`: This library routine retrieves a library base from a global variable and calls two library functions using offsets and calculated arguments.
- `decrement_structure_counter`: This library routine decrements a short integer stored at an offset within a structure.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `recursively_call_pointers`: The routine checks several global pointers and arguments, then calls itself recursively if a pointer is non-zero.
- `initialize_info_library_structure`: Allocates a memory block and populates a complex internal data structure by calculating various offsets and copying data from a global template.
- `initialize_info_library_struct`: Initializes a structure or table by calculating and storing several offsets or pointers derived from a base address.
- `execute_conditional_callback_3`: Library code that calls a function via a pointer multiple times with different arguments, storing the results in global variables and calling another routine based on the returned values.
- `execute_conditional_callback`: Library code that calls a function via a pointer multiple times with different arguments, storing the results in global variables and calling another routine with a status value if the initial calls return zero.
- `call_function_via_pointer`: Library code that calls a function via a pointer and executes another routine with specific arguments if the first call returns zero.
- `check_pointer_and_call_function`: This library routine calls a function via a pointer and, if the result is zero, calls another function with specific arguments.
- `return_constant_one`: This library routine returns the value one.
- `retrieve_nested_structure_value_2`: Library code that checks a flag byte within a structure to return a value from a direct offset, a secondary pointer, or a nested structure.
- `retrieve_nested_structure_value`: Checks a flag byte within a structure to determine whether to return a direct value, a value from a secondary pointer, or a value nested within a third-level pointer.
- `fetch_nested_value`: This library routine traverses multiple levels of pointers within a structure to retrieve a longword value.
- `dereference_nested_pointer_via_flag`: Checks a flag byte within a structure to determine whether to use a primary pointer or an alternative address stored in a register.
- `get_value_from_pointer`: This library routine retrieves a longword from a location determined by an offset within a pointer stored in a structure.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a2, then returns.
- `retrieve_nested_structure_value_3`: Library code that retrieves values from a structure or a nested structure based on a flag, updates a global variable, and returns a value from a specific offset within a structure.
- `check_library_status_and_set_flag`: Library code that checks a boolean flag and a function return value to decide whether to update a global flag and retrieve a value from a nested pointer.
- `update_structure_data_3`: As part of library code, this routine updates a structure with values from a table and performs conditional logic based on the results of several subroutines.
- `update_structure_data_2`: Library code that updates a structure with values from a table, performs bounds checking, and copies data into specific offsets within the structure.
- `update_structure_data`: Library code that updates a structure by calling other routines to retrieve values, performs bounds checking on a field, and copies data from a table into the structure.
- `update_structure_and_iterate_2`: Library code that updates a structure with values from a table and performs conditional logic based on a function return.
- `update_structure_and_iterate`: This library routine updates a structure with values from a table and performs conditional logic based on a function return.
- `copy_library_strings_and_data`: This library routine copies a string using a library function and then copies certain dwords or even dwords from one structure to another.
- `manage_library_icon_data`: Library code that manages data copying and updates a structure's internal pointers and fields based on the results of other routines.
- `process_library_logic_2`: Library code that performs conditional checks on global variables and executes a function pointer with specific arguments.
- `process_library_logic`: As library code, it executes a sequence of conditional checks and function calls involving global variables and a callback function.
- `dispatch_library_callback`: As library code, it performs conditional checks on global variables and executes a function pointer with specific arguments.
- `dispatch_callback_logic`: Library code that performs conditional checks on global variables and executes a function pointer with specific arguments.
- `execute_callback_logic`: As part of library code, this routine performs conditional checks on global variables and executes a function pointer with specific arguments.
- `execute_conditional_callback_2`: Library code that performs conditional logic based on a retrieved value and executes a function pointer from a structure.
- `validate_library_parameters`: As part of library code, this routine checks a global variable, calls a library function via a global base, and executes a parameter-validated routine.
- `call_library_function_via_offset_2`: Library code that executes a function found by applying a negative offset to a pointer in a global variable.
- `call_library_function_via_offset_3`: Library code that performs a call to a function at a fixed offset from a global pointer.
- `call_library_function_via_offset_4`: Library code that invokes a function using a pointer stored in a global variable with a specific offset applied.
- `call_library_function_via_offset_5`: Library code that calls a function by accessing a pointer in a global variable and subtracting an offset.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_function_via_offset_6`: Library code that retrieves a function address from a global variable using a negative offset and executes it.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_function_via_offset_7`: Library code that calls a function located at a specific offset relative to a pointer in a global variable.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_indirect_handler`: This library routine fetches a pointer from a global table and executes the code at that address.
- `call_indirect_handler_2`: This library routine performs an indirect call to a function pointer stored in a global table.
- `call_indirect_handler_3`: This library routine retrieves a function pointer from a table and jumps to it.
- `call_indirect_handler_4`: This library routine executes a subroutine located at an address retrieved from a global table.
- `execute_indirect_callback`: Fetches a function pointer from a location within a data structure and executes it.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `execute_indirect_function_5`: Fetches a function pointer from a location offset from a global variable and executes it.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_function_13`: Fetches a function pointer from an offset within a global structure and executes it.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `execute_indirect_function_6`: Fetches a function pointer from a location calculated by adding an offset to a global variable and executes it.
- `execute_indirect_function_7`: Fetches a function pointer from a global table using a calculated offset and executes it.
- `execute_indirect_function_8`: Fetches a function pointer from an offset within a global variable and executes it.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_indirect_handler_5`: This library routine loads a pointer from a global table and transfers control to that address.
- `call_indirect_handler_6`: This library routine calls a function by dereferencing a pointer found within a global table.
- `call_indirect_handler_7`: This library routine accesses a global table to find and execute a specific function pointer.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_indirect_handler_8`: This library routine performs an indirect jump to a destination address stored in a global table.
- `call_indirect_handler_9`: This library routine fetches a target address from a global table and executes it.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_indirect_handler_10`: This library routine executes a routine by following a pointer stored in a global table.
- `invoke_library_functions`: Library code that calls two different library functions using a base address from a global variable and a pointer passed via a stack-allocated structure.
- `dispatch_via_switch_statement`: This library routine performs a switch on a value within a structure and executes different library calls based on the result.
- `return_constant_one_2`: This routine returns the integer value one to its caller.
- `modify_global_bitmask`: Library code that performs a bitwise XOR operation on a global variable using a value determined by a switch statement.
- `store_register_to_global`: As part of library code, this routine stores a register value into a global variable offset by a fixed amount and then calls another routine using a pointer from a structure.
- `validate_info_structure_state`: Checks the type and integrity of an information structure by verifying specific status bytes and validating associated data pointers.
- `validate_structure_state`: Library code that checks a flag to determine a pointer source, performs several function calls using that pointer, and sets a local variable to zero if any of those calls return zero.
- `process_string_and_attributes`: Library code that copies a string, calls a library function, and performs several conditional checks involving pointer offsets and string copies.
- `verify_multiple_callbacks`: As library code, it repeatedly calls a series of function pointers from a table until one returns a negative value.
- `execute_function_until_failure`: Library code that repeatedly calls a function pointer until it returns a value of negative one.
- `repeat_function_call`: As part of library code, this routine calls a function pointer repeatedly until it returns a specific value.
- `call_and_repeat`: Library code that calls a function pointer and recursively calls itself if the return value is not negative one.
- `call_other_and_return_one`: As part of library code, this routine calls another routine and then returns one.
- `initialize_info_library_tables`: Library code that initializes internal tables by repeatedly calling another routine with various constant arguments and a pointer derived from the input parameter.
- `execute_list_of_library_calls`: Library code that calls another routine multiple times with a fixed first parameter and varying other constant arguments.
- `initialize_display_palette_entries`: Library code that calls another routine or a sequence of calls with a fixed set of arguments, passing the same first parameter each time and varying the remaining constants.
- `register_info_table_entries`: Library code that calls another routine ten times with a fixed set of integer arguments, passing the same first parameter each time and varying the remaining constants.
- `set_info_table_entries`: Library code that calls another routine ten times with a fixed set of six integer arguments each time, passing the same first parameter each time.
- `select_conditional_parameter`: Determines a value based on a byte comparison and passes it to another routine.
- `process_structure_conditional`: Library code that performs conditional operations based on a structure field and executes a callback if a register is non-zero.
- `execute_multiple_library_calls`: Library code that calls a library function or routine multiple times using a pointer from a structure.
- `update_list_elements_2`: Library code that iterates through a linked list of data structures and updates fields within them using values from a secondary source.
- `update_list_elements`: Library code that iterates through a list of data structures and updates fields within them using values from a secondary source.
- `process_data_list`: Library code that iterates through a list, calling functions to update values in a structure based on elements in a table.
- `update_linked_list_elements`: Iterates through a list of data structures, updating a field in each structure using a value from a secondary source.
- `update_structure_pointer_2`: Library code that compares structure members and either executes specific callbacks or updates a pointer within a global structure.
- `update_pointer_from_structure`: As part of library code, this routine calls a routine with a pointer from a structure, then updates that structure member with a value from another pointer.
- `dereference_and_update`: Library code that checks structure conditions and updates a global pointer by dereferencing a value from a structure member.
- `update_pointer_dereference`: Library code that calls a routine and then replaces a structure member with a value retrieved by dereferencing a pointer.
- `modify_global_state_and_execute`: Library code that sets a global variable, checks a condition, calls several library functions, and updates a global pointer.
- `sync_structure_data`: Library code that updates a global flag and synchronizes a structure member by retrieving a value from a dereferenced pointer.
- `update_structure_pointer`: Library code that fetches a value from a dereferenced pointer and updates a structure member based on a conditional check.
- `execute_sequential_callbacks`: Library code that calls another routine twice using pointers or offsets or registers.
- `build_string_pointer_table_2`: Library code that iterates through a linked list to count elements, allocates a table, and populates it with pointers to string data.
- `build_string_pointer_table`: Library code that counts elements in a linked list, allocates a table, and populates it with pointers to data from the list.
- `allocate_and_populate_string_table`: Library code that calculates the size of a linked list of strings, allocates memory, and populates a table with pointers to those strings.
- `populate_string_metadata_table`: Traverses a linked list of string pointers to calculate their lengths and populates a destination table with these values.
- `process_string_list_2`: Library code that traverses a linked list of pointers to populate a table with values derived from each entry.
- `process_string_list`: Library code that iterates through a list of strings, calculates their lengths, and stores the results into a table via a pointer.
- `process_string_list_elements`: Iterates through a list of pointers to retrieve string data and calculates lengths or properties for each entry.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_function_via_offset`: Library code that calls a function located at a specific offset from a pointer stored in a global variable.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `call_library_vector_114`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.
- `call_library_vector_180`: Library stub: loads a library base from a global variable and calls the library function at offset minus 180 of that base, returning its result to the caller.
- `call_library_vector_186`: Library stub: loads a library base from a global variable and calls the library function at offset minus 186 of that base, returning its result to the caller.
- `dispatch_formatted_output_sequences`: Selects a sequence of formatting operations based on a state value and executes multiple calls to a formatting routine with varying parameters and data pointers.
- `return_empty`: Library code that is empty.
- `initialize_info_substructures`: Populates a series of global pointers by calling a routine with specific constant arguments and values retrieved from a global structure.
- `execute_repeated_calls_2`: Library code that performs a variable number of calls to a function based on a value retrieved from a global structure.
- `execute_repeated_calls`: This library routine performs a variable number of calls to another routine based on a byte value in a global structure.
- `frag_movem_unlk_rts`: Short code fragment that copies several registers to or from memory, then removes the stack frame held in a6, then returns.
- `execute_callback_safely`: Library code that executes a function pointer from a data structure and returns zero if the result is negative one.
- `invoke_callback_and_check`: Library code that executes a function pointer from a data structure and returns a value, ensuring it is not negative one.
- `return_from_subroutine_6`: This routine is an empty function that performs a single return instruction.
- `verify_triple_call_2`: Library code that attempts to execute a function pointer up to three times, returning a success value only if all calls do not return negative one.
- `verify_triple_call`: Library code that calls a function pointer up to three times and returns one only if all calls do not return negative one.
- `verify_double_call`: Library code that calls a function twice via a pointer and returns one if both calls do not return negative one.
- `check_function_return`: Library code that calls a function via a pointer and returns true if the result is not negative one.
- `return_one`: Library code that returns the value one.
- `execute_conditional_dispatch`: As library code, it iterates through a loop, calling a routine based on a value in a structure and performing conditional calls.
- `return_zero_4`: Library code that returns the value zero.
- `dispatch_conditional_calls`: Library code that inspects a value at an offset within a structure to determine which routine to call.
- `execute_recursive_call`: This routine performs a single call to another routine before returning.
- `execute_indirect_call`: Library code that performs an indirect call to a function pointer and returns zero.
- `execute_two_calls`: Library code that calls another routine twice and returns a constant value.
- `execute_multiple_callbacks`: As part of library code, this routine calls another routine or a global function pointer multiple times based on a condition checked against a global variable, then calls another routine with a specific pointer and constant, and finally calls another routine with a certain pointer and a constant.
- `setup_string_template`: Library code calculates a bitmask from a value in a table and calls a string-formatting routine multiple times with different bit-masked arguments and a global pointer.
- `format_complex_string`: Library code that prepares arguments and calls a string-formatting routine using a specific pattern string.
- `format_string_arguments`: Library code that prepares a pointer-based argument list or a temporary buffer for a string-formatting routine using a pattern string.
- `calculate_string_length_2`: Library code that calculates the length of a null-terminated string.
- `calculate_string_length`: Library code that increments a register while checking if a null terminator has been reached at an offset from a pointer.
- `copy_cstring_to_buffer`: Library code that copies a null-terminated byte string from a source pointer to a destination pointer, advancing both pointers and writing each byte until a zero byte is encountered.
- `copy_memory_block_2`: Library code that copies a specified number of bytes from a source pointer to a destination pointer.
- `copy_memory_block`: Library code that copies a fixed number of bytes from a source pointer to a destination pointer.
- `compare_strings_2`: Library code that compares two null-terminated strings for equality.
- `compare_strings`: Library code that compares two null-terminated strings and returns true if they are equal.
- `get_string_length`: Library code that counts the number of bytes in a null-terminated string.
- `execute_library_operations`: Library code that calls several other library functions, including a routine that initializes a structure array and routines that use conditional logic or call library vectors.
- `call_string_formatter`: Library code that calls another routine using a pointer to a global variable and a local buffer, then calls another routine using the populated buffer.
- `copy_prefixed_string`: Library code that reads a length byte from a source, copies that many bytes to a destination, and appends a null terminator.
- `copy_string_data_2`: Library code that copies bytes from a source location to a destination location and appends a null terminator.
- `initialize_complex_structure`: Library code that populates two structures with various values and then calls a library function and an indirect callback.
- `execute_library_function`: Library code that calls a library function using a pointer retrieved from a global variable offset.
- `execute_library_calls`: Library code that calls a library function or another routine multiple times using a pointer retrieved from a structure member.
- `call_global_function`: Library code that invokes a function pointer retrieved from a global variable offset.
- `store_byte`: Library code that stores a byte into a memory location pointed to by a register.
- `call_indirect_routine_via_table`: The library code calls another routine indirectly through a pointer stored in a global variable table.
- `call_indirect_via_table`: The library code calls another routine through an indirect pointer taken from a global table.

### Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:add21k

- `initialize_system_structures`: Initializes various memory structures and resets multiple fields to zero or specific values within a data table.
- `return_immediately_3`: This routine is empty and returns immediately.
- `execute_infinite_loop_4`: This routine enters an infinite loop that performs no operations.
- `reset_dma_counters`: Resets specific memory values to zero or maximum values and invokes another routine if a provided value is non-zero.

### Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:IconX

- `initialize_system_environment`: Initializes core application structures by retrieving system pointers, managing memory allocation, and parsing command-line arguments or configuration strings.
- `update_state_and_trigger_callbacks`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `dispatch_cli_or_workbench_start`: Chooses between a command-line start and a Workbench start.
- `build_wait_and_endcli_command`: Builds a command string from a template and the given name, including a wait command with a long timeout and an endcli redirected to NIL, after checking whether the name starts with a dot.
- `install_library_functions`: Creates and installs a library from a descriptor: it makes the library structure, sets up its function table, and returns the result.
- `scan_ram_entries_for_names`: Allocates a 260-byte buffer and walks a linked list of entries, copying the entries whose name bytes are above the control range into it and passing them to helper routines.
- `join_strings_into_buffer`: Copies one string into a destination buffer and joins a second string onto it.
- `count_string_length`: The routine receives a pointer to a zero-terminated byte string and returns the number of bytes before the terminating zero.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `append_string_end_and_call`: Advances a pointer to the terminating zero of a string, then passes that position and a second argument to another routine.
- `parse_signed_decimal`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule_function`: Calls a function pointer retrieved from an offset within a global structure.
- `execute_indirect_function_9`: Fetches a function pointer from a global variable and executes it.
- `execute_indirect_function_10`: Fetches a function pointer from a location offset from a global variable and executes it.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `execute_indirect_function_11`: Fetches a function pointer from a global table and executes it.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `call_indirect_pointer`: Calls a function pointer located at a specific offset from a global variable.
- `execute_indirect_function_12`: Fetches a function pointer from a location relative to a global variable and executes it.
- `execute_indirect_function_13`: Fetches a function pointer from a location calculated by an offset from a global variable and executes it.
- `execute_indirect_function_14`: Fetches a function pointer from a location calculated by an offset from a global variable and executes it.
- `call_indirect_function_2`: Fetches a function pointer from a specific offset within a global structure and executes it.
- `execute_indirect_function_15`: Fetches a function pointer from an offset within a global structure and executes it.
- `execute_indirect_function_16`: Fetches a function pointer from an offset within a global variable and executes it.
- `execute_indirect_function_17`: Fetches a function pointer from a global variable and executes it.
- `invoke_and_checksum_stack_buffer`: Invokes a function pointer stored in a global variable and computes a rolling checksum over the local stack buffer until a null terminator is reached.
- `store_byte_to_address_2`: Stores a byte from a register into a memory location pointed to by a register.
- `execute_indirect_function_18`: Fetches a function pointer from a global table and executes it.
- `store_byte_via_pointer`: Stores a single byte from a data register into a memory location pointed to by an address register.

### Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:Amber*_Install

- `unpack_and_initialize_installer`: Performs a relocation and jump table reconstruction from a data block before initializing the environment.
- `initialize_installer_environment_3`: Opens libraries, allocates memory, and initializes several global variables and hardware registers.
- `initialize_installer_environment_2`: Sets up the installation environment by clearing a table of values and updating global pointers.
- `handle_disk_read_error`: If a file reading operation fails, the routine clears a memory buffer and resets several global variables.
- `check_flag_bit`: Checks a specific bit in a value at an offset from a pointer and updates a global variable accordingly.
- `decrement_counter`: Decrements a value initialized from a register down to a specific constant using a loop.
- `initialize_installer_environment`: Performs a series of setup tasks including clearing a table of values and updating global pointers.
- `execute_dos_function_chain`: Repeatedly invokes function pointers from a global structure to perform operations, potentially handling errors or specific return codes.
- `relocate_memory_block`: Traverses a list of pointers to update memory addresses and fills a destination buffer with values from a source table if a null terminator is encountered.
- `capture_register_state`: Saves the current contents of two registers into global memory locations and retrieves a value from a third global location to be returned as a concatenated 32-bit result.
- `store_registers_to_globals_2`: Stores the contents of two registers into two global memory locations and returns the value stored in a third global location.
- `expand_null_terminated_stream`: Repeatedly calls another routine to process data until a null byte is encountered in the stream.
- `decompress_data_loop_2`: Repeatedly calls another routine to expand a data stream until a null byte is encountered.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `decompress_data_stream_2`: Expands a compressed data stream into a destination buffer using bitwise expansion and length-encoded instructions.
- `decompress_data_stream`: Iterates through a source buffer to copy and expand data into a destination buffer based on encoded length and control bits.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `copy_data_table`: copies a series of 32-bit words from a source table to a destination table while interleaving 16-bit values from a register into the destination.
- `calculate_memory_offset`: Compares multiple global values against specific thresholds to determine a calculated offset.
- `configure_hardware_registers`: writes values to bitplane, display, and CIA registers and polls a bit in a global variable.
- `set_pointer`: stores the address of a global variable into itself.
- `open_input_device`: opens a device and adds a port to an execution list using system library calls.
- `close_input_device`: removes a port and closes a device using system library calls.
- `close_graphics_library`: accesses a library structure to retrieve a specific offset and then calls a function to close it.
- `allocate_memory_block`: calls a memory allocation routine multiple times and performs arithmetic on the returned addresses to determine a final pointer.
- `execute_recursive_subroutine_calls`: Calls a routine, then calls itself recursively, followed by two calls to another routine.
- `clear_global_variable`: Sets a global variable to zero twice.
- `parse_command_stream`: Iterates through a data stream, executing different routines or updating a global variable based on specific byte values.

### Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:FILE01

- `relocate_and_initialize_environment`: Performs a relocation of code segments by updating pointers in a table and then initializes system libraries and hardware registers.
- `setup_installer_system`: Initializes system libraries and memory, clears specific global memory areas, and configures hardware registers.
- `configure_bitplane_display_hardware`: Initializes display hardware by writing to bitplane, copper, and display enable registers.
- `initialize_color_palette_and_copy_data_2`: Sets hardware color registers and populates a memory area by copying structured data from various offsets within a source table.
- `initialize_system_memory_and_tables`: Checks a global variable against a threshold and, if a subsequent routine returns a negative value, performs a large memory zeroing operation using a loop and resets various global variables.
- `initialize_color_palette_table_3`: Initializes a color palette by writing values to hardware color registers and populating a large memory table with repeated sequences of color data.
- `initialize_color_palette_table_4`: Clears a block of memory and populates a large table with repeated sequences of values retrieved from global memory.
- `decrement_counter_2`: Decrements a local loop counter from four thousand nine hundred sixteen down to zero.
- `initialize_data_table`: Performs a sequence of hardware and memory operations, including zeroing out a large block of memory pointed to by a table.
- `perform_dos_operation`: Iterates through a loop to perform operations via a function pointer, checking for specific error codes and returning a result.
- `parse_command_stream_3`: Iterates through a buffer until a specific character is found, then performs a series of operations including setting global variables and calling multiple subroutines.
- `update_memory_table`: Processes a list of pointers to update a destination table, potentially filling it with data from several global sources if a zero value is encountered.
- `retrieve_global_registers`: Returns a concatenated 64-bit value composed of two 32-bit registers stored in global memory.
- `store_registers_to_globals_3`: Stores the contents of two registers into two global memory locations and returns a value from a third global location.
- `execute_block_decompression_2`: Executes a loop that calls another routine to expand data until a null byte is detected.
- `execute_block_decompression`: Calls a routine to copy and expand a compressed block of data within a loop that terminates upon finding a null byte.
- `return_no_op`: The routine consists of a single return instruction and performs no work.
- `copy_compressed_block`: Copies and expands a block of data from a source into a destination using a bit-stream decompression algorithm.
- `memmove_block_data`: Copies a block of memory from a source to a destination.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning to the caller.
- `copy_structured_data`: Copies data from a source table into a destination buffer, splitting long values into two parts and appending a terminator.
- `generate_pattern_data`: Populates a memory area with a series of calculated values and characters in a nested loop structure.
- `calculate_offset_index`: Performs complex arithmetic on several global values to determine and return a specific offset or index.
- `configure_hardware_registers_2`: Writes values to various hardware registers, including bitplane and display control registers, while polling a status bit.
- `set_global_pointer`: Stores the address of a global variable into itself.
- `open_input_device_2`: Performs device opening operations for an input device and handles potential errors.
- `close_input_device_2`: Performs device closing operations and removes a port from a list.
- `close_graphics_library_2`: Closes a library and retrieves a specific offset from its base structure.
- `allocate_memory_block_2`: Calculates memory requirements and performs memory allocation using system calls.
- `execute_sequence_of_functions`: Calls three different subroutines in a fixed sequence before returning.
- `copy_data_block`: Repeatedly copies a sequence of values from one memory location to another using a loop.
- `update_global_value`: Sets a global variable to zero and then conditionally updates it based on a comparison.
- `parse_command_stream_2`: Iterates through a buffer of bytes to execute different subroutines based on specific command values.

### Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_subsystems`: Performs a complex initialization sequence involving the relocation of a function table and the setup of multiple global data structures.
- `return_constant_2`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_graphics_tables`: Initializes several global tables by calling various setup routines and performing bulk data copies.
- `copy_and_poll_hardware`: Copies a block of data from a source to a destination and then enters a loop that polls a bit in a hardware register.
- `copy_memory_blocks`: Performs nested loops to copy large blocks of data between memory locations using specific offsets and stride increments.
- `update_scaled_value`: Multiplies a global variable by a constant, adds one, and stores the result back to the same global variable.
- `apply_bitmask_table`: Iterates through a list of values to perform bitwise operations using data from a table to update multiple memory locations.
- `setup_blt_transfer`: Configures hardware registers for a Blitter transfer using a calculated source address and destination parameters.
- `setup_blt_copy`: Sets up Blitter registers to copy data from a source address derived from a global variable to a destination address.
- `store_split_values`: Splits a 32-bit value into two 16-bit parts and stores them into a table at specific offsets.
- `copy_short_words`: Copies a sequence of 16-bit words from a source pointer to two different destinations within a global structure.
- `initialize_jump_table`: Sets a series of global pointers to various routine addresses and populates a table with these values.
- `reset_global_counter`: Increments a global variable and then immediately sets it to a fixed value.
- `tiny_ori_2`: Small routine of 1 instruction that merges 64 into d0.
- `execute_infinite_loop`: The routine is empty and performs an infinite loop.
- `configure_blitter_parameters`: Writes specific bit patterns and calculated offsets to hardware registers and memory locations to prepare a Blitter operation.
- `clear_memory_block`: Fills a block of 8000 words starting at a global address with zeros.
- `adjust_nibble_values`: Iterates through two tables of words, modifying specific nibbles in the destination table based on the values in the source table.
- `decrement_nibbles`: Iterates through a table of words and decrements each non-zero nibble by one.
- `modify_memory_and_increment`: Performs bitwise OR operations on two memory locations and increments a value stored at a global variable address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_sprite_attributes`: Checks a signed value and updates a global variable if it is non-zero.
- `calculate_memory_offsets`: Calculates and stores a series of memory offsets into global variables based on values read from a pointer.
- `update_audio_and_entity_state`: Updates audio hardware registers and entity attributes by parsing compressed data streams.
- `parse_compressed_data_stream`: Reads a stream of bytes from a memory buffer, performing bitwise shifts and conditional logic to extract values.
- `initialize_sprite_data_structures`: Initializes a series of data structures by clearing memory and setting specific attribute fields, including bitmask calculations for sprite visibility.
- `parse_entity_attribute_stream`: Iterates through a byte stream to update an entity structure by interpreting control codes.
- `update_entity_from_stream`: Parses a byte stream of control codes to update an entity structure by fetching values from a lookup table.
- `parse_entity_data_stream`: Parses a byte-stream to update an entity structure by reading attribute values and scaling factors from a lookup table.
- `update_entity_attribute_and_scale`: Calculates and updates a value based on a combination of arithmetic operations, bitwise flags, and fixed-point scaling.
- `transform_global_value`: Adds a constant to a global variable and then performs a bitwise XOR with a shifted and masked version of itself.
- `populate_array_from_nested_tables`: Iterates through a destination array to populate each element by performing multiple levels of indirect indexed lookups into several data tables.
- `populate_array_from_indexed_table`: This routine iterates through a loop to populate a destination array by performing multi-level indexed lookups into several data tables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_audio_and_interrupts`: Sets audio channel length and level registers, then modifies interrupt enable and request registers.
- `update_scaled_pointer`: Calculates a scaled offset using division and bitwise shifts to update a destination pointer and a secondary pointer.
- `calculate_scaled_offset`: Performs division and bitwise shifts to store a calculated value into a destination pointer and updates another pointer using a scaled offset.
- `update_buffer_metadata_wrapper`: Calls another routine multiple times and returns a concatenated result of two 32-bit values.
- `update_buffer_metadata`: Checks a bitmask in a byte, then updates a pointer and a length value within a structure, potentially resetting values if a condition is met.
- `fill_memory_patterns`: Fills a memory area with alternating patterns of bytes and specific constant values using two different loop structures.
- `generate_gradient_table`: Populates a memory area with values calculated from a nested loop that performs multiplication and division to create a gradient.
- `initialize_system_hardware`: Clears memory areas, populates a gradient table, and configures audio hardware registers and DMA settings.
- `update_audio_parameters`: Calculates values based on an input and a table, then writes the results to specific global variables and hardware registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `relocate_and_initialize_runtime`: Performs a self-relocation by parsing a jump table and updating internal pointers.
- `initialize_system_environment_2`: Interacts with system libraries to manage memory, check task status, and configure hardware registers and memory blocks.
- `initialize_display_and_clear_buffer`: Configures Amiga bitplane and display control registers and populates a memory table with incrementing values.
- `initialize_color_palette_and_copy_data`: Sets hardware color registers and copies large blocks of structured data from one memory location to another using a loop.
- `initialize_and_clear_data_buffer`: Aligns a global pointer to a word boundary and fills a large block of memory with zeros.
- `initialize_color_palette_table`: Initializes a memory block by clearing it and then populating it with a repeating sequence of color values.
- `initialize_color_palette_table_2`: Clears a block of memory and populates a large table with repeated sequences of color data.
- `decrement_counter_3`: Decrements a local counter from a fixed value down to zero using a loop.
- `clear_memory_block_3`: Aligns a global pointer to a word boundary and fills a large block of memory with zeros.
- `handle_file_error`: Traverses a structure through function pointers to check for specific error codes and returns a status value.
- `search_string_and_align`: Searches for a specific character in a string, updates a global pointer to the following character, and performs several hardware and memory operations.
- `initialize_or_patch_table`: Either initializes a table with values from a source or applies relative offsets to entries within a table based on a list of values.
- `fetch_and_concatenate_registers`: Retrieves values from two global memory locations and combines them into a single 64-bit result.
- `store_registers_to_globals`: Stores the contents of two registers into global memory locations and returns a value from a third global location.
- `decompress_data_loop`: Copies a block of memory while performing bitwise operations on data from a source table.
- `process_unnamed_routine_call`: The specific operations performed on registers or memory are not visible due to the lack of decompiled code.
- `copy_structured_data_2`: Copies data from a source to a destination in chunks, while also splitting a 32-bit integer into two 16-bit halves within a structure.
- `generate_pattern_table`: Populates a table with repeating sequences of incrementing values and specific character constants using nested loops.
- `calculate_offset_range`: Performs bounds checking and arithmetic on a global variable to determine a specific offset within a defined range.
- `configure_hardware_registers_3`: Writes specific values to hardware registers including bitplane and display control registers, while polling a status bit.
- `assign_self_address`: Stores the memory address of a global variable into itself.
- `open_device_and_port`: Uses system library calls to open a device, add a port, and perform I/O operations.
- `close_device_and_port`: Uses system library calls to close a device, remove a port, and perform I/O operations.
- `access_library_data`: Opens a library and follows a chain of pointers to retrieve a value from a specific offset within the library structure.
- `query_available_memory`: Calls system functions to check available memory and performs arithmetic on the returned values.
- `execute_sequential_functions`: Calls one specific routine once followed by another routine three times.
- `execute_sequence_with_state`: Updates a global variable with various addresses before calling several different routines in sequence.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_sequence_of_calls`: Updates a global variable with several values and calls three different routines sequentially before returning a bitwise OR result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `nop_padding_sequence`: This routine is empty and consists of two instructions that perform no operation, serving as padding between functional code blocks.
- `store_self_address_global`: The routine stores the address of a global variable into that same global variable and then returns, performing no other computation.
- `configure_interrupt_registers`: Modifies the interrupt enable and acknowledge registers.
- `setup_function_pointers`: Writes the address of the routine into a series of consecutive memory locations and performs arithmetic on a global variable.
- `clamp_value_within_range`: Performs bitwise operations on a global variable and clamps a calculated value between the bounds stored in a table.
- `update_clamped_input_state`: Reads input status from a hardware register and updates a global variable based on bitwise comparisons.
- `set_flag_on_comparison`: Compares two values and stores a specific byte in a global variable based on the result.
- `set_flag_on_comparison_2`: Compares two values and stores zero or a specific byte in a global variable based on the result.
- `generate_coordinate_table_2`: Processes a source table to calculate interleaved values and writes them into a destination memory block with specific offsets.
- `generate_coordinate_table`: Calculates and stores a series of interleaved values into a destination table based on input data.
- `initialize_memory_block`: Stores a specific value into a pointer, clears a global variable, and writes a value to a memory offset.
- `initialize_hardware_registers`: Sets multiple hardware registers for DMA, display, and sprite positioning, then performs I/O operations and enters an infinite loop.
- `perform_io_operations`: Calls functions via pointers stored in a global variable and performs conditional logic based on the return value.
- `update_clamped_values`: Updates a global variable by adding values from a source and clamping the results against bounds stored in a table.
- `update_clamped_value`: Updates a global variable by adding a value to it and then clamping the result between two specified bounds.
- `update_max_value_2`: Compares a calculated value against two values in a table and stores the largest result in a global variable.
- `update_max_value`: Compares a value from a table with a provided value and stores the larger of the two in a global variable.
- `decrement_and_call_callback`: Decrements a global variable and calls another routine if the variable equals one.
- `increment_and_call`: Increments a global variable and calls another routine if the variable equals one.
- `call_function_pointer`: Calls a routine through a pointer stored in a global variable and returns the concatenated results of two registers.
- `execute_calculated_callback`: Calculates an offset based on global data and calls a function pointer located at a derived address.
- `calculate_and_call_function_2`: Sets a global variable based on the value of a register and calls a routine through a pointer.
- `calculate_and_call_function`: Calculates a value based on a subtraction from a constant and stores it in a global variable before calling a function pointer.
- `initialize_memory_block_2`: Clears a block of memory with zeros and calculates a value to be stored in several global variables.
- `modify_global_pointer`: Performs bitwise masking on a global variable and adds a constant offset to it.
- `copy_memory_block_3`: Copies a large block of data from a source to a destination using a loop that moves chunks of memory.
- `validate_screen_input`: Checks a byte for a non-zero value and triggers an error message if the condition is not met.
- `execute_infinite_loop_2`: Enters an infinite loop that prevents further execution of the calling routine.
- `populate_data_table`: Writes a series of 32-bit values and associated 16-bit constants into a table pointed to by a register.

### Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:AMBERDEV.UDO#unpacked (raw code file, load address unknown)

- `initialize_subsystems`: Performs a sequence of function calls while updating various global variables and memory locations.
- `return_from_subroutine_3`: This routine consists of two instructions that perform a return from a subroutine.
- `update_music_state`: Checks a value in a register and calls different routines based on whether the value is zero, negative, or within a specific range.

### Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:INSTALL.UDO#unpacked (raw code file, load address unknown)

- `execute_initialization_sequence`: Sets several global variables and calls a long sequence of functions before entering a loop that executes functions stored in a table.

### Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:EXTRO.UDO#unpacked (raw code file, load address unknown)

- `initialize_graphics_data_tables`: Populates several global structures and tables with specific data values and pointers.
- `process_graphics_data_stream`: Parses a structured data stream to perform memory copies, table updates, and bitmask merging.
- `increment_and_wrap_counter`: Increments a global variable and resets it to zero if it reaches six, then calls another routine.
- `calculate_display_offsets`: Derives memory offsets and dimensions through arithmetic calculations on values retrieved from tables.
- `merge_graphics_data`: Performs arithmetic calculations to derive memory offsets and then merges bitmask data into a buffer.
- `synchronize_hardware_status`: Monitors a global variable and hardware status bits through a loop until a specific condition is met.
- `process_structured_data`: Applies bitwise operations to a destination table based on values found in a source buffer.
- `blit_graphics_data`: Performs a bitwise logical operation to copy and blend source pixel data into a destination buffer.
- `wait_for_bit_clear_check`: Monitors a specific bit within a memory location until it is cleared.
- `iterate_and_call_routine`: Iterates through five consecutive memory locations and calls a routine if the value at each location is non-negative.
- `write_split_values_to_table`: Splits a 32-bit value into two 16-bit parts and writes them into a table at specific offsets.
- `copy_data_to_globals`: Copies nine 16-bit values from a source table into two identical locations within a global structure at specific offsets.
- `fill_global_structure`: Performs two sequential loops to copy data from two different source tables into two identical locations within a global structure at various offsets.
- `copy_source_to_globals`: Copies sixteen 16-bit values from a source pointer into two identical locations within a global structure at a specific offset.
- `setup_hardware_registers`: Configures hardware registers for display and blitter settings while filling a block of memory with data from a table.
- `empty_loop_routine`: The routine contains no reachable instructions and consists of an infinite loop that never executes any operation.
- `configure_blitter_hardware`: Writes values to several hardware registers, including bitplane and display parameters, to set up display or blitter settings.
- `clear_global_memory`: Fills a large block of memory starting at a global variable with zeros for eight thousand iterations.
- `transform_pixel_data`: Iterates through two tables of 16-bit values, performing bitwise manipulations on specific nibbles and bits of the source data before storing the result in the destination table.
- `decrement_pixel_values`: Iterates through two tables, subtracting values from the nibbles of each 16-bit word in the source table and storing the result in the destination table.
- `apply_structured_bitmask_to_array`: Applies a bitwise OR and AND mask to specific word offsets within a series of data structures.
- `apply_bitmask_to_memory`: Applies bitwise OR and AND operations to specific offsets within a memory range using provided values.
- `apply_bitmask_to_tables`: Iterates through a table, applying a bitwise OR with one value and a bitwise AND with another value to specific offsets within each entry.
- `blit_bitmapped_graphics`: Performs a bitwise logical operation to copy and blend source pixel data into a destination buffer using variable bit-depth shifts.
- `calculate_blt_parameters`: Performs arithmetic calculations on input values to derive memory offsets and dimensions.
- `merge_bitmask_data`: Performs a bitwise OR operation between a source buffer and a destination buffer using a mask to preserve existing bits.
- `merge_pixel_values`: Iterates through a buffer and bitwise-ORs specific byte values into a destination table based on conditional checks.
- `merge_pixel_data`: Iterates through a source buffer to bitwise-OR specific byte values into a destination table at fixed offsets.

### Amberstar (1992)(Thalion)(Disk 1 of 3)[FD-HD].adf:INTRO_P.UDO#unpacked (raw code file, load address unknown)

- `initialize_intro_graphics_and_sequences`: Configures display hardware and bitplane settings before executing a series of memory transfers and data transformations.
- `copy_and_wait_for_input`: Copies a block of memory from a source to a destination and enters a loop that waits for a specific bit in a hardware register to clear.
- `copy_memory_blocks_2`: Performs nested loops to copy structured data blocks between two memory locations.
- `update_linear_sequence`: Calculates a new value based on a global variable and a multiplier, then stores the result back into that global variable.
- `apply_palette_mask`: Parses a data stream to find specific markers and applies bitwise operations using a lookup table to update a destination buffer.
- `setup_blt_transfer_2`: Configures hardware registers for a Blitter transfer using a calculated source address and destination parameters based on an input value.
- `setup_blt_transfer_offset`: Sets up Blitter registers for a memory transfer using a global variable to determine the source and destination addresses.
- `store_split_words`: Splits a 32-bit value into two 16-bit parts and stores them into a table at specific intervals.
- `copy_word_sequence`: Copies a sequence of words from a source pointer into two different locations within a global table.
- `configure_display_hardware`: Configures bitplane hardware, clears a memory block, and writes a series of values to several global variables and hardware registers.
- `execute_infinite_loop_3`: This routine enters an infinite loop that prevents further execution of the calling code.
- `configure_bitplane_hardware`: Writes values to hardware registers to configure bitplane modes, display window start/stop positions, and DMA parameters.
- `clear_memory_block_2`: Fills a block of memory starting at a global variable with zeros for a fixed number of iterations.
- `transform_nibble_data`: Iterates through two memory buffers and modifies the nibbles of the destination buffer based on the values in the source buffer.
- `decrement_nibbles_2`: Iterates through two memory buffers and subtracts values from the nibbles of the destination buffer based on the source buffer.
