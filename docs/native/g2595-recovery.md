# Game 2595 (a-10 tank killer - extra missions (1990)(sierra)[data disk]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02595). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901200 top-level bytes in 24 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 122 of 122 named (0 author symbols), 122 of 122 described (43 by rule, 62 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 832 of 832 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5854 of 5854 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 73956 |
| code | 18244 |
| data | 68975 |
| padding | 5305 |
| unused | 734720 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| A-10 Tank Killer - Extra Missions (1990)(Sierra)[data disk].adf:sim_install | 2 | 5838 | 120 |
| A-10 Tank Killer - Extra Missions (1990)(Sierra)[data disk].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |

## Cases handled

- a-10 tank killer - extra missions (1990)(sierra)[data disk]/A-10 Tank Killer - Extra Missions (1990)(Sierra)[data disk].adf:disk2/campaign2/s1.sss: high-entropy data file (7.67 bits/byte), classified as packed data
- a-10 tank killer - extra missions (1990)(sierra)[data disk]/A-10 Tank Killer - Extra Missions (1990)(Sierra)[data disk].adf:disk2/campaign2/s4.sss: high-entropy data file (7.75 bits/byte), classified as packed data
- a-10 tank killer - extra missions (1990)(sierra)[data disk]/A-10 Tank Killer - Extra Missions (1990)(Sierra)[data disk].adf:disk2/campaign2/s2.sss: high-entropy data file (7.74 bits/byte), classified as packed data
- a-10 tank killer - extra missions (1990)(sierra)[data disk]/A-10 Tank Killer - Extra Missions (1990)(Sierra)[data disk].adf:disk2/campaign2/s0.sss: high-entropy data file (7.69 bits/byte), classified as packed data
- a-10 tank killer - extra missions (1990)(sierra)[data disk]/A-10 Tank Killer - Extra Missions (1990)(Sierra)[data disk].adf:disk2/campaign2/s5.sss: high-entropy data file (7.72 bits/byte), classified as packed data
- a-10 tank killer - extra missions (1990)(sierra)[data disk]/A-10 Tank Killer - Extra Missions (1990)(Sierra)[data disk].adf:disk2/campaign2/s3.sss: high-entropy data file (7.65 bits/byte), classified as packed data
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### A-10 Tank Killer - Extra Missions (1990)(Sierra)[data disk].adf:sim_install

- `initialize_installation`: Initializes the installation process by setting up global variables, allocating memory, and preparing for file operations.
- `cleanup_and_exit`: Executes cleanup routines including calling a function pointer, freeing memory, closing libraries, replying messages, and exiting with a return value from global variables.
- `initialize_game_resources`: Initializes multiple global data structures with hardcoded parameters and processes table entries, setting up object properties and memory blocks before proceeding with game setup.
- `load_and_process_data`: Loads data from a global structure, processes it through multiple library calls, and returns a 16-bit value based on specific conditions involving dimensions and offsets.
- `initialize_structure_fields`: Copies input parameters to various fields in a structure at offset 4884 of a global data area, including coordinates, flags, and pointers.
- `initialize_object_structure`: Initializes a memory structure with fixed offsets for object properties, sets up linked list pointers, and updates a global counter before returning a pointer to the initialized block.
- `process_table_entries_2`: Iterates through a table starting at offset 4940, calling a library routine for each entry until reaching a counter limit, then processes qualifying entries with parameters from the table and calls additional library routines.
- `initialize_memory_block`: Allocates a memory block using another routine, stores the block pointer in a global variable, and initializes the first 17 words of the allocated block with specific values including zeros, negative ones, and various constants.
- `initialize_object_block`: Allocates and initializes memory for an object block, setting up its header and data fields including dimensions, flags, and pointers, then links it into a global table.
- `initialize_memory_block_2`: Allocates and initializes a memory block structure, setting its header fields including size, type, and flags, then returns a pointer to the allocated block.
- `process_string_and_call`: Processes a null-terminated string by calculating its length, then calls multiple library routines with different parameter combinations.
- `initialize_object_structure_2`: Initializes a memory structure with fixed offsets for object properties, sets up linked list pointers, and updates a global counter before returning a pointer to the allocated block.
- `initialize_and_process_3`: Initializes global variables with library function results and processes linked list entries, updating structure fields based on calculated values.
- `copy_and_process_strings`: Copies strings from a table into a local buffer and processes each string with two different parameters, returning a pointer from the table when processing succeeds.
- `initialize_and_process_4`: Initializes global variables with library function results and processes linked list entries, updating structure fields based on calculated values.
- `process_string_data`: Processes string data by copying bytes between buffers, updating pointers, and handling special conditions based on string content.
- `allocate_memory_and_process`: Allocates a memory block using a library function, processes the block through multiple conditional paths based on allocation success and data validation, and returns either a calculated difference or an error code.
- `perform_calculations_and_calls`: Performs arithmetic operations on input parameters and makes several library calls with derived values, returning a calculated result.
- `initialize_and_process`: Initializes global variables with library function results and processes linked list entries, updating structure fields based on calculated values.
- `process_data_structure_loop`: Initializes various structure fields with predefined values, enters a loop that processes data entries from a linked list, performs conditional operations based on entry values, and handles memory allocation and data transfer operations.
- `manage_memory_blocks`: Allocates memory blocks when conditions are met, updates linked list pointers, and returns a pointer to the allocated block or zero.
- `free_allocated_blocks`: Processes a linked list of allocated memory blocks by calling a library routine on each block and updating the head pointer.
- `update_and_free_block_2`: Updates a global pointer with data from a memory block, then calls another routine with the block contents before returning.
- `frag_tst_beq_movea_move_bra`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was zero, then tests the memory at an offset from a5, then branches if the last result was zero, then copies the memory at an offset from a5 to a1, then copies the memory at a1 to a0, then copies the memory at an offset from a5 to a2, then copies a0 to the memory at a2, then jumps to another point in the code.
- `update_and_free_block`: Updates a global pointer with data from a memory block, calls a library routine with block contents, and returns immediately.
- `iterate_table_entries_a`: Iterates through entries in a table starting at offset 4940, calling a library routine for each entry until reaching a counter limit.
- `iterate_table_entries_b`: Iterates through entries in a table starting at offset 5020, calling a library routine for each entry until reaching a counter limit.
- `call_library_offset_neg348`: Saves d2-d3 and a2-a3/a6, loads a6 from the stack, copies six stack arguments into a0-a3 and d0-d3, calls the library vector at offset minus 348 from a6, restores registers and returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `process_file_handle`: Processes a file handle by checking a bit in its header, calling another routine with specific parameters, and returning a result based on global state.
- `copy_string_data`: Processes a null-terminated string by copying bytes to a destination buffer while updating counters, and handles special conditions based on a global variable's value.
- `parse_format_specifier`: Parses a format string for a printf-like operation, handling flags, width, precision, and length modifiers, then retrieves a variable argument from a pointer and converts it to a string buffer using helper routines.
- `parse_format_string`: The routine walks a format string character by character.
- `parse_command_line`: Parses command line arguments into a table of string pointers, handling quoted strings and escape sequences, then initializes global variables based on whether any arguments were parsed.
- `initialize_memory_block_3`: Initializes a memory block structure by allocating memory, setting up pointers, and updating status flags based on conditions checked at offsets 16 and 19 of the structure.
- `process_file_entry`: Processes a file entry by validating its header bits, updating counters and status flags based on conditions, and writing data bytes to a buffer while managing entry state.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (movep.w) and then falls through into the code that follows, with no return of its own.
- `process_file_entry_2`: Processes a file entry by validating its header bits, updating counters and status flags based on conditions, and writing data bytes to a buffer.
- `process_table_entries`: Iterates through a table of entries, processes non-empty entries that don't have bit 4 set, and calls another routine with data from the table.
- `convert_character_case`: Converts uppercase characters to lowercase by subtracting 32 from their ASCII values when a flag in a lookup table is set, then returns the modified string.
- `copy_string_with_limit`: Copies a null-terminated string from a source pointer to a destination pointer, truncating the copy if it exceeds a specified maximum length and ensuring null-termination.
- `append_second_string_to_first`: Finds the end of the first string, copies bytes from the second string there including the terminator, and returns the original first pointer in d0.
- `frag_clr_move_bra`: Short code fragment that clears d0, then copies the memory at an offset from a7 to d0, then jumps to another point in the code.
- `format_number_as_reversed_string`: Calls another routine in a loop, writing each returned digit as an ASCII character into a stack buffer, then copies the characters back in reverse order into the output pointer, null-terminates it, and returns the length of the string.
- `frag_clr_move_bra_2`: Short code fragment that clears d0, then copies the memory at an offset from a7 to d0, then jumps to another point in the code.
- `format_number_as_octal`: Converts an unsigned value to octal digits by repeatedly taking its low three bits, stores the digits in a temporary stack buffer, then copies them in reverse order into the buffer given in the first argument, terminates the string with a zero byte, and returns the digit count.
- `select_stack_word_and_format_value`: Loads a word indexed by d3, calls a comparison routine, follows condition-code branches, otherwise clears d0, loads a stack word, and branches to formatting code.
- `format_uint_as_hex_string`: Converts the unsigned long in the second argument to lowercase hexadecimal digits using a lookup table, writes them reversed into the caller's buffer, null-terminates it and returns the character count.
- `parse_signed_decimal_word`: Skips an optional leading sign and parses decimal digits into a signed 16-bit value stored at the output.
- `accumulate_signed_decimal_word`: Updates d1 as a decimal accumulator, conditionally negates it for a minus sign, restores d2, stores the low word through a stack argument, and returns consumed length.
- `update_memory_link`: Updates linked list pointers in a global structure when allocating memory, and initializes the allocated block's header with size and next pointer values.
- `read_entry_and_data`: Validates a table entry, reads data from a file using another routine, and returns the result of the read operation or -1 on error.
- `update_counter_and_store_byte`: Increments a counter at offset 1630 in a global structure, decrements another counter at offset 113a, and stores a byte value from d7 into memory at an address calculated from offset 1134.
- `frag_move_moveq_pea_jsr_addq_rts`: Short code fragment that copies d7 to d0, then loads 0 into d1, then copies d0 to d1, then pushes the address of the memory at an offset from a4 onto the stack, then copies d1 to the stack, then calls another routine, then adds 6 to a7, then copies d0 to d1, then copies the top of the stack to d7, then returns.
- `initialize_and_process_2`: Initializes a global variable to zero, calls another routine with specific parameters, and then returns the value of that global variable.
- `update_file_handle`: Updates a file handle in a table based on input parameters and returns the handle or -1 on failure, with error handling for file operations.
- `copy_byte_block_overlap_safe`: Copies a count of bytes from the source to the destination, copying from the end backwards when the destination is at or below the source, otherwise copying forwards from the start.
- `free_memory_list`: Traverses a linked list of memory blocks starting at offset 5684 in a global structure, frees each block using the exec.FreeMem library call, and clears the head pointer.
- `validate_file_access`: Validates file access by checking a global flag and returning an error code if the flag is set, otherwise proceeding with file operations.
- `allocate_memory_with_header`: Allocates a memory block of specified size plus 12 bytes, initializes the block's header with size and next pointer values, and updates linked list pointers in global variables.
- `find_free_memory_block`: Searches a linked list for a free memory block of specified size, splits the block if necessary, and updates pointers in the global structure.
- `copy_and_validate_data`: Processes data bytes from a source buffer, updates counters in a structure, stores each byte to a destination, and calls another routine with the byte value until a condition is met or all data is processed.
- `compute_adjusted_offset`: The routine checks a flag byte in a structure and conditionally calls another routine with a derived index.
- `reset_structure_and_seek`: The routine conditionally adjusts a parameter using a short field from a structure based on flag bits, then clears several fields in the structure and masks flag bits.
- `decode_variable_length_bytes`: Iterates through a count of items, repeatedly calling another routine to fetch bytes from a buffer pointer while decrementing a counter.
- `allocate_memory_block`: Searches a linked list for a free memory block, allocates a new block if none is found, clears 23 bytes starting at the block address, and returns the block address.
- `parse_command_string`: Parses a command string starting with 'a', 'r', or 'w' characters, calls a routine to resolve parameters, and updates a structure with flags and pointers based on the command type and presence of a '+' character.
- `process_entry_and_clear_flags`: The routine conditionally obtains a value from another routine using a derived index, then calls another routine with fields from a structure.
- `validate_and_process_entries`: Checks entries in a table for specific bit conditions and non-zero differences, then processes qualifying entries with parameters from the table.
- `write_data_to_file`: Writes data to a file handle using the DOS library, handles errors by setting global variables, and returns the result of the write operation.
- `seek_file_position`: Seeks to a position in a file handle using the DOS library, handles errors by setting global variables, and calculates new positions based on input parameters.
- `read_data_from_file`: Reads data from a file handle using the DOS library, handles errors by setting global variables, and returns the result of the read operation.
- `open_file_handle`: Opens a file using the DOS library, handles errors by setting global variables, and returns the file handle or -1 on failure.
- `lock_or_open_directory`: Attempts to lock a directory, unlocks it if successful, and opens the directory as a file if locking fails, returning appropriate handles or -1.
- `delete_and_open_file`: Locks a directory, unlocks it and deletes a file if locking succeeds, then opens the file and returns the handle or -1 on failure.
- `close_file_and_exit`: Checks a global variable and calls another routine if it's non-zero, then closes a file handle and returns zero.
- `multiply_fixed_point_halves`: Multiplies the high and low 16-bit halves of d0 and d1 as signed and unsigned words, sums the two cross products, shifts that sum into the upper 16 bits, and adds the low-word product, leaving the combined value in d0.
- `divide_with_sign_handling`: Handles signed division by checking signs of dividend and divisor, negating values when necessary, calling a normalization routine, and returning the result with proper sign handling.
- `divide_with_normalization`: Divides the 32-bit value in d0 by the 32-bit value in d1, shifting both operands to normalize the divisor, and returns a 64-bit value holding the quotient and remainder, with a fast path when the high word of the divisor is zero.
- `validate_version_byte`: Reads a byte from a global variable, compares it with 79, and stores 79 in the byte if it's less than 79.
- `write_debug_output`: Copies characters from one memory location to another using a loop, writes debug information to a file handle obtained through library calls, and returns an error code.
- `frag_lea_moveq_movea_jsr_move_bne_bra`: Short code fragment that loads the address of a location in the code into a1, then loads 0 into d0, then copies a fixed memory address to a6, then calls another routine, then copies d0 to the memory at an offset from a5, then branches if the last result was not zero, then loads 255 into d0, then jumps to another point in the code.
- `setup_and_call_library`: Prepares multiple arguments including pointers to data structures and integer values, pushes them onto the stack, and invokes a library routine with these parameters.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, removes the stack frame with unlk, then returns to the caller.
- `validate_and_process_entry`: Validates a table entry, checks if bit 4 is clear, calls another routine with entry data, clears the entry, and returns an error code based on global flags.
- `validate_and_access_table_entry`: Validates an index against a global limit, multiplies it by six to calculate an offset, accesses a table entry at that location, and returns the address or zero based on validation.
- `handle_io_operation`: Checks for specific signal conditions, executes a function pointer if one exists, and clears it when the function returns a non-zero value.
- `frag_bsr_rts`: Short code fragment that calls another routine, then returns.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_114`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_60_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_108_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.

### A-10 Tank Killer - Extra Missions (1990)(Sierra)[data disk].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move_moveq_movea_jsr_bra`: Short code fragment that copies 18440 to d1, then loads 0 into d2, then loads 0 into d3, then copies 5120 to a6, then calls another routine, then jumps to another point in the code.
