# Game 2409 (dragon's breath (1990)(palace)[b]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02409). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802320 top-level bytes in 16 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 103 of 103 named (0 author symbols), 103 of 103 described (26 by rule, 76 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 118 of 118 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1438 of 1438 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2057 |
| code | 4658 |
| data | 1037464 |
| padding | 3965 |
| unused | 754176 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| run | 2 | 117 | 2 |
| dB | 2 | 228 | 12 |
| joy_acc-handler | 3 | 224 | 17 |
| 9 | 1 | 2 | 1 |
| 9 | 1 | 2 | 1 |
| diskfont.library | 13 | 830 | 67 |
| Dragon's Breath (1990)(Palace)[b](Disk 1 of 2).adf:bootblock | 1 | 29 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |

## Cases handled

- dragon's breath (1990)(palace)[b](disk 2 of 2)/Dragon's Breath (1990)(Palace)[b](Disk 2 of 2).adf: custom-format (non-DOS) disk, 1747 non-empty blocks: matched against track-loaded segments
- dragon's breath (1990)(palace)[b](disk 1 of 2)/Dragon's Breath (1990)(Palace)[b](Disk 1 of 2).adf:ddata: high-entropy data file (7.99 bits/byte), classified as packed data
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- dragon's breath (1990)(palace)[b](disk 2 of 2)/Dragon's Breath (1990)(Palace)[b](Disk 2 of 2).adf: blocks 1749-1759 (bytes 895488-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.97)
- 1 packed track region(s), 888832 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_memory_block`: Copies sixteen words from a source address to a destination address and then calls another routine.

### dB

- `initialize_program_environment`: This routine checks a global value, copies a block of data from one location to another, and then calls a decryption routine.
- `decrypt_and_jump`: This routine checks a memory condition, modifies a stack-based structure, and then executes code at a pointer before entering an infinite loop.
- `decrypt_and_execute_data`: Retrieves a data block from a file, performs an XOR-based decryption loop on its contents, and then executes the resulting code at the new memory location.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `call_library_function`: Performs an indirect call to a function located within a library structure.
- `call_library_function_2`: Executes a function pointer retrieved from a specific offset within a library structure.
- `call_library_function_3`: Calls a routine by accessing a function pointer through a library base and an offset.
- `call_library_function_4`: Uses a library base to call a function at a specific offset.
- `call_library_function_5`: Retrieves a function pointer from a library and executes it.
- `call_library_function_6`: Performs an indirect call to a routine using an offset from a library base.
- `call_library_function_7`: Calls a function via a pointer found at a specific offset within a library structure.
- `call_library_function_8`: Executes a routine by dereferencing a pointer at a specific offset from a library base.

### joy_acc-handler

- `trigger_infinite_recursion`: Performs a direct call to itself.
- `execute_recursive_call`: The routine performs a direct recursive call to itself before returning.
- `process_joystick_input_and_score`: Updates a score counter with overflow handling and modifies movement vectors based on joystick register bits.
- `update_joystick_state`: This routine reads a bit from a hardware register and updates several fields in a data structure based on the result.
- `initialize_handler_data`: This routine populates a memory block with various values and performs several indirect calls to other routines.
- `initialize_data_structures`: Stores several values into a memory block and performs multiple indirect calls to library functions and other routines.
- `process_indirect_list`: This routine stores a value from a pointer and then iterates through a list of pointers, calling another routine for each until it returns zero.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `execute_indirect_call`: Performs an indirect call to a location stored in a jump table.
- `return_from_handler`: The routine is empty and performs no operations before returning.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `perform_long_division`: Calculates a quotient and remainder using a loop of shifts and subtractions or a division algorithm based on the input values.
- `swap_and_rotate_words`: The routine swaps the high and low words of a value in a register, then repeatedly doubles a pair of words while propagating carry, subtracting a constant if the intermediate value exceeds a threshold, and decrements a counter fifteen times.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `execute_library_call`: Performs an indirect call to a function retrieved through a series of pointers.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.

### 9

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.

### 9

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 0 into d0, then returns.

### diskfont.library

- `update_font_handle_context`: Calls a function via a table entry and, if the return value is non-zero, stores a value from a register into an offset within the returned handle before calling another routine via a table entry.
- `check_and_set_font_handle`: This library code calls a routine via a table entry and, if the result is non-zero, stores an argument into an offset within the returned handle before calling another routine via a table entry.
- `release_library_resources_2`: This library routine attempts to close two libraries and, if successful, modifies a structure to manage memory or resource tracking.
- `release_library_resources`: As library code, it calls functions to close a library and free memory.
- `unlink_and_cleanup_font_resources`: Iterates through a linked list of font resource structures to unlink and free those marked for removal.
- `manage_library_state`: As library code, it either updates a linked list and frees memory or sets a high bit in a byte within a structure.
- `set_high_bit`: Library code that sets the most significant bit of a byte at a fixed offset from a pointer.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `decrement_and_call_handler`: As library code, it decrements a value in a structure and, if certain conditions are met, performs an indirect call to a function pointer.
- `return_zero`: This routine is an empty function that simply returns zero to the caller.
- `unpack_font_data_list`: Iterates through a linked list of font string structures to extract character data and metadata into a destination buffer.
- `unpack_font_string_list`: Iterates through a linked list of string structures to copy character data into a destination buffer.
- `copy_string_to_buffer`: Copies a sequence of bytes from a source pointer to a destination buffer.
- `parse_font_file_data`: Iterates through a font data file to extract character dimensions and metadata.
- `populate_font_metadata_table`: Iterates through structured font data to extract character dimensions and memory pointers.
- `parse_font_header_data`: Parses structured font data from a disk source by reading specific byte sequences and populating a buffer with character metadata.
- `read_and_parse_font_data`: Reads data from a disk source and parses it into a structured buffer.
- `validate_and_get_absolute_value`: This routine invokes several system functions to manage file locks, directory paths, and memory, then returns the absolute value of a register.
- `return_zero_constant`: Library code that immediately returns a zero value without performing any other operations or accessing memory.
- `calculate_bitmask_difference`: This library code calculates a value based on the difference between two word values and adjusts it using a loop that performs bitwise comparisons against a global table.
- `subtract_bitmask_values`: This library routine iterates through a loop seven times to perform bitwise comparisons between two sets of data.
- `apply_bitmask_subtractions`: Iterates through a bitmask up to seven times, subtracting values from a register based on bitwise comparisons against a table and specific byte flags.
- `process_bitmask_table`: This library routine iterates through a loop up to seven times to subtract values from a register.
- `return_constant_value`: Library code that returns a specific constant value.
- `update_font_index_metadata`: Iterates through a table of font entries to find the entry with the highest attribute value.
- `load_and_parse_font_data`: Opens a font file from disk, allocates memory, and reads its contents into a buffer.
- `update_font_selection`: This library routine iterates through a table of font entries to find the entry with the highest attribute value and updates a tracking variable.
- `update_font_selection_table`: Iterates through a table of font entries to find the entry with the highest attribute value and updates a tracking variable.
- `find_best_font_index`: Iterates through a table of font entries to find the index of the entry with the highest attribute value.
- `search_font_index`: Iterates through a table of font entries to find the entry with the maximum value for a specific attribute.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `copy_null_terminated_string`: Library code that copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `find_string_end`: This library code iterates through a null-terminated string until it reaches the terminator.
- `copy_string_stub`: This library routine acts as a wrapper that calls another routine to copy a null-terminated string from a source to a destination pointer.
- `count_string_length`: This library routine calculates the number of bytes in a null-terminated string.
- `return_empty`: This library routine is empty.
- `compare_strings`: This library routine compares two strings and returns a status value based on their lexicographical order.
- `compare_bytes`: This library routine compares a byte from a pointer with a byte at an offset from another pointer and returns a status value.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `find_byte_in_string`: This library code searches for a specific byte within a null-terminated string and returns a pointer to its location or zero if not found.
- `find_char_in_string`: This library routine searches for a specific byte within a null-terminated string and returns a pointer to its location.
- `return_zero_2`: This library routine returns zero.
- `search_string_backward_2`: This library code calculates the length of a string and then searches backwards from the end for a specific byte.
- `search_string_backward`: This library routine searches for a specific byte within a string by traversing backwards from a starting pointer.
- `return_zero_3`: This library routine performs no operations and returns zero to the caller.
- `search_for_delimiters`: This library routine searches for specific byte values within a string using a backward search.
- `return_from_subroutine_2`: This routine is an empty function that performs a return.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: This routine acts as a dispatcher that calls an external function pointer stored in data and then returns.

### Dragon's Breath (1990)(Palace)[b](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `disable_interrupts_and_set_register`: This routine loops to write zero to several offsets of a hardware register and then sets a global variable to a specific value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_and_loop`: This routine calls a function through a pointer and then enters an infinite loop.
