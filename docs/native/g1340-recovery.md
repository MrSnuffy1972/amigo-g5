# Game 1340 (total eclipse (1989)(incentive software)[b]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01340). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 12 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 14 raw-word lines |
| Functions named and described | pass, 142 of 142 named (0 author symbols), 142 of 142 described (40 by rule, 95 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 609 of 609 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2884 of 2892 instructions identical in start and length, 3 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 232 |
| code | 9350 |
| data | 112749 |
| padding | 3173 |
| unused | 775680 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| SetMap | 3 | 228 | 7 |
| type | 2 | 117 | 2 |
| cls | 2 | 20 | 1 |
| border | 3 | 1788 | 113 |
| DISK-VALIDATOR | 1 | 386 | 1 |
| nitro | 1 | 13 | 1 |
| cosy | 2 | 24 | 4 |
| Total Eclipse (1989)(Incentive Software)[b].adf:2 | 1 | 296 | 11 |
| Total Eclipse (1989)(Incentive Software)[b].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### SetMap

- `initialize_and_jump`: Calls a routine and then jumps to a longword address.
- `process_amiga_hunk_relocations`: Parses a list of data structures by comparing values against specific constants to identify hunk types, then performs pointer arithmetic and memory updates to handle relocations and data segments.
- `update_map_pointers`: Uses a loop to update a series of pointers in a table based on values read from a source list.
- `set_d0_bits`: Performs a series of bitwise OR operations on register d0 using the constants 248, 148, 0, 0, and 0.
- `unpack_stream_with_pointer_shift`: Decompresses a packed stream by reading bits through a shift routine that takes a 32-bit value from the four bytes before the source pointer, shifts it right and sets its top bit, and writes the resulting bytes to a destination buffer.
- `shift_pointer_value`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `shift_bitstream_store_pointer`: Repeats a shift of the bit buffer right by one place for a count in d1, storing the current pointer's low word into a global on each pass.

### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves a longword from d1 to d0 and then branches to a new location.

### cls

- `open_dos_library`: Opens the dos.library through the library vector table and stores the returned base pointer in a local variable.

### border

- `jump_to_address`: Performs a jump to a PC-relative address.
- `initialise_border_state`: Clears the stack, pushes a PC-relative address, and calls another routine; if the return value is not zero, it stores it in a global variable and calls two other routines; it then conditionally calls several other routines using pointers from a structure in a5 and PC-relative addresses, finally calling another routine and clearing a word on the stack before returning.
- `check_intuition_library_and_report`: Checks for the presence of a version string and the intuition library name, then performs seven routine calls.
- `call_present_resource_routines`: Calls routines only for resources that are present, checking a stored pointer and a status byte first.
- `setup_console_device_request`: Calls library vector functions and routines in sequence, referencing the console device text.
- `configure_border_structure_fields`: Initializes a structure by setting pointers from a table, clearing specific memory locations, and updating flag bits.
- `move_data_to_address`: Moves a longword from a location pointed to by a combination of a2 and a3 into a destination at an offset of a2.
- `call_border_routines_with_coordinates`: Computes four offsets from stored structure fields and calls routines with those coordinates.
- `delay_after_coordinate_calls`: Calls routines with coordinates computed from stored structure fields, then makes a delay call through the DOS library and calls a library vector function.
- `store_parameter_and_call_routine`: Pushes a pointer from a global variable and a word from offset 8 of a5 onto the stack, calls another routine, and then performs a sequence of operations including a loop that writes zeros to a memory range, calls several library functions via a pointer in a6, and calls another routine.
- `load_a4_constant`: Loads the constant value 32766 into register a4 and returns.
- `initialize_border_state`: Pushes a pointer from a2 onto the stack, multiplies a word from a global variable by 6, calls another routine, and if the result is non-zero, it stores it in a global variable and proceeds to initialize a structure with specific word values, a constant string, and several conditional routine calls, eventually calling another routine and returning.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `tokenize_and_allocate_string_array`: Parses a null-terminated string into individual tokens by handling whitespace and quoted substrings, then stores these tokens into a newly allocated array of pointers.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `load_and_branch`: Loads the word value 32767 into register D0 and then branches to a different address.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `compare_two_byte_strings`: The routine compares two byte strings byte by byte.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `manage_window_structure`: Performs a sequence of library calls to manipulate a window structure, including updating offsets and clearing a global variable.
- `move_d0_to_d5`: Copies the long value from register D0 to register D5.
- `call_and_terminate_buffer`: Stores its first parameter as a pointer, calls a routine with the second parameter and a stack address, then writes a zero byte at the stored pointer and returns the call's result.
- `increment_pointer_and_store_byte`: Increments a global pointer and stores a byte from the first parameter at the original address.
- `call_parse_format_string`: Pushes a pointer from offset 8 of a5 onto the stack, pushes a PC-relative address onto the stack, calls another routine, cleans the stack, and returns.
- `process_conditional_data_transfer`: Checks a value at offset 18 of a structure in a5; if it is 4, it copies a longword from a pointer in a0 to d4, otherwise it performs a series of sign extensions, negations, and calls two different routines using values from a structure and a table before potentially writing a byte to a memory location.
- `parse_formatted_string`: Parses a string containing format specifiers to extract and convert values into a local buffer, supporting integer, floating-point, and string types.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `perform_32bit_division`: The routine calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `compute_string_length_2`: Pushes a PC-relative address and a word from offset 8 of a5 onto the stack, calls another routine, and returns.
- `check_and_call_routine`: Pushes a word from offset 8 of a5 and a pointer from offset 10 of a5 onto the stack, calls another routine, and if a comparison of a word from offset 8 of a5 with 10 fails, it checks bit 7 of a byte at offset 12 of a pointer from offset 10 of a5; if that bit is set, it calls another routine using the pointer from offset 10 of a5.
- `branch_to_offset`: Performs a relative branch to a specific offset.
- `call_check_routine`: Loads a pointer from offset 10 of a5, compares the longword at offset 0 of that pointer to the longword at offset 4 of that pointer, and if the first is not less than the second, calls another routine with the pointer and a byte from offset 8 of a5.
- `copy_byte_and_increment_pointer`: Copies a byte from an offset of a structure pointed to by a5 into a location pointed to by a2, increments the pointer in a2 by one, and branches to another location.
- `apply_routine_to_record_table`: Steps through a fixed table of 22-byte records from its first entry up to its end, calling another routine once for each record's address.
- `return_constant_or_pointer`: Loads a pointer from offset 8 of a5 into a2, and if a2 is not zero, returns the value of a2 in d0; otherwise, it returns negative 255 in d0.
- `close_file_from_table_2`: Checks a byte at offset 12 of a structure, and if bit 2 is set, calls a routine with a pointer from offset 8 of a2.
- `check_bit_and_branch`: Loads a pointer from offset 8 of a5, stores a constant at a specific offset from a4, and if bit 2 of a byte at offset 12 of a2 is not set, it returns negative 255.
- `process_buffer_and_clear`: If bit 2 of a byte at offset 12 of a2 is set, it calculates a difference between a longword at offset 0 and a longword at offset 8 of a2, calls another routine with that difference and a word from offset 8 of a2, and if the result matches the difference, it sets bit 4 of a byte at offset 12 of a2 and clears two longwords at offsets 0 and 4 of a2.
- `clear_structure_fields_2`: If a word at offset 12 of a5 is not 65535, it clears bit 2 of a byte at offset 12 of a2 and clears two longwords at offsets 0 and 4 of a2.
- `update_and_copy_byte`: If a longword at offset 8 of a2 is zero, it calls another routine; if a word at offset 16 of a2 is 1, it copies a byte from offset 13 of a5 to offset -1 of a5 and calls another routine; otherwise, it updates a structure and copies a byte from offset 13 of a2 to a location pointed to by a5.
- `update_structure_and_copy_byte`: Updates a structure at a2 by setting its first longword to the value of its third longword, adding a 16-bit value from offset 16 to that third longword, setting bit 2 of offset 12, incrementing the pointer in a2, and copying a byte from offset 13 of a structure in a5 to the new pointer location before branching.
- `search_linked_list`: Iterates through a list of structures starting at a pointer derived from a4, checking a byte at offset 12 of each structure and comparing the current pointer against a limit, returning a pointer or zero.
- `branch_to_middle_of_routine`: Performs a short branch into the middle of another routine.
- `clear_structure_fields`: Clears the longword at a2, the longword at offset 4 of a2, and the longword at offset 8 of a2, then moves a2 to d0 and branches to another location.
- `update_structure_field`: Calls another routine, and if the result is zero, sets a word at offset 16 of a structure via a2 and sets a longword at offset 8 of that structure to the result plus 14; otherwise, it sets a word at offset 16 of a2 to 1024, sets bit 1 of a byte at offset 12 of a2, and sets a longword at offset 8 of a2 to the result.
- `fetch_table_entry_2`: Sets a word at offset 16 of a structure via a2, sets bit 1 of a byte at offset 12 of a2, moves a longword from a pointer at offset -4 of a5 to offset 8 of a2, then calls another routine and branches based on the result.
- `setup_list_traversal`: Sets the stack pointer using a5, saves registers a2 and a3 to the stack, loads a pointer from a global variable into a2, and branches to another location.
- `free_list_nodes_from_pointer`: Releases each node of a singly linked list starting at the node in the address register and stopping after the node whose link is empty, using each node's stored size plus eight, then clears the global head pointer.
- `store_and_call_routine`: Stores a pointer to itself in a global variable, calls another routine with a modified argument, and if the result is not zero, updates a structure with the result and a value from a5.
- `replace_global_pointer`: Copies a pointer from a global variable to a location in a2, moves a value from offset 8 of a5 to offset 4 of a2, updates the global variable with the value in a2, and branches to another location after setting d0 to the value in a2 plus eight.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `update_pointer_table_2`: Clears a3, loads a pointer from a global variable into a2, and branches.
- `update_pointer_table`: Compares a value at offset 8 of a5 with a2, and if they are not equal, moves the longword at the address pointed to by a2 into a3 and then moves that longword into a2 if it is non-zero, otherwise it moves the value negative one into d0.
- `move_longword_conditionally`: Moves the contents of a3 into d0, and if d0 is not zero, moves the longword at the address pointed to by a2 into the address pointed to by a3.
- `setup_library_call`: Loads a longword from a pointer in a2 at an offset of minus 32018 from a4, increments d0 by 8, pushes d0 and a2 onto the stack, calls a routine, cleans the stack, clears d0, and branches to another location.
- `fetch_table_entry`: Multiplies a word from offset 8 of a5 by 6 and adds it to a base address in a4 to fetch a longword from a table, then calls another routine if the word at offset 8 is within a specific range and the fetched longword is non-zero, otherwise it writes the value 2 to an offset in a4 and returns negative one.
- `call_with_scaled_index`: Multiplies a word from offset 8 of a5 by 6, uses the result to fetch a longword from a table, calls another routine, and returns either 1 or the original value in d0.
- `clear_d0_and_branch`: Clears d0 and branches to another routine.
- `call_and_store_error`: Calls a routine using a longword from offset 8 of a5, and if the result in d0 is not zero, calls another routine and stores the result in a global variable.
- `clear_d0_and_branch_alt`: Clears d0 and branches to a different location.
- `calculate_offset_and_test`: Calls another routine, multiplies the result by 6, and uses that value to calculate an address via a pointer at a specific offset from a4; it then tests the longword at that address and, if it is non-zero, writes 2 to a specific offset from a4 and returns negative 1.
- `check_bit_mask`: Tests if bits 0 and 1 are set in a word at offset 4 of a2, and if they are not, writes the value 5 to an offset in a4 and returns negative one; otherwise it continues.
- `execute_with_parameters`: Pushes a word from offset 14 of a5, a word from offset 10 of a5, and a pointer from a2 onto the stack to call another routine, then calls a second routine if the first result is negative one.
- `move_d5_to_d0_and_branch`: Moves the contents of d5 into d0 and branches to another routine.
- `check_library_result`: Calls another routine with a pointer to a constant, then checks bit 3 of the result; if bit 3 is set, it checks a byte at a specific offset from a4 and calls another routine if that byte is non-zero.
- `write_fixed_text_then_callback_2`: Calls another routine, clears d0, and branches.
- `write_fixed_text_then_callback`: Obtains an output handle, writes a four-byte text fragment from a fixed address via DOS calls, and then executes a conditional function.
- `run_optional_hook_then_finish`: Calls a global hook pointer if it is non-zero, then calls another routine and returns.
- `release_all_resource_tables`: Releases the program's resource tables by iterating through a table to free entries, running a global hook, and freeing various buffers.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_file_from_table`: Checks if an index is within the bounds of a table, then if the entry at that index is non-zero and has a specific bit set, calls another routine to close it and clears the entry; otherwise it returns a failure code.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_function_414`: Performs an indirect jump to a library function located at an offset from a base address stored in a global variable.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `execute_indirect_jump`: Performs an indirect jump to a function address retrieved by following a pointer stored in a global variable and applying a negative offset of 198.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `jump_to_indirect_address`: Performs an indirect jump to an address calculated by adding an offset to a value retrieved from a pointer stored in a global variable.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_function`: Loads a library base from a global variable and calls the function at an offset of minus 210 from that base.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_function_offset_552`: Loads a library base from a global variable and calls the function at an offset of minus 552 from that base.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_function_offset_384`: Loads a library base from a global variable and calls the function at an offset of minus 384 from that base.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.

### DISK-VALIDATOR

- `format_and_update_display_buffer`: Calculates a character and digit value based on a global variable, writes them to a buffer at an offset determined by another global variable, and then performs a series of bitwise operations and calls to update a structured data set including bitmask calculations and memory writes.

### nitro

- `subtract_byte_at_offset`: Subtracts 8 from a byte stored at offset 256 of the structure pointed to by a6, then returns.

### cosy

- `call_library_entry`: Invokes another routine and checks the zero flag.
- `open_dos_library_2`: Calls the exec library function OldOpenLibrary to load dos.library and returns the resulting library handle in d0.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_output_2`: Wrapper that calls the dos library function Output and returns its result to the caller.

### Total Eclipse (1989)(Incentive Software)[b].adf:2

- `allocate_and_initialize_display_buffer`: Allocates memory and initializes a display buffer by performing bitwise transformations and swaps on data, then copies a block of 32000 bytes from a global area into the buffer.
- `update_display_registers`: Checks a bit in a hardware register, decrements a counter in a global variable, and if not zero, enters a loop that calls a routine three times to process bytes from three different pointers before updating hardware color registers with swapped values from a table.
- `process_display_buffer`: Allocates memory, performs several library calls to manage data, executes a loop that performs bitwise transformations and swaps on data, and uses a jump table to call a routine via a pointer.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `clear_d0_byte`: Performs a bitwise OR on register d0 with zero.
- `clears_data_register_zero`: Performs a logical OR operation on data register zero with zero.
- `clears_data_register_zero_2`: Performs a bitwise OR operation on data register zero with zero.
- `clear_data_register_d0`: Performs a logical OR operation on data register d0 with zero.
- `copy_nested_data`: This routine performs nested loops to transfer data between two memory areas using word-based assignments and bit manipulation.
- `double_word_table_entries`: Doubles each of sixteen words from one global table into a second table, then copies four further words into separate globals.
- `copy_global_block_to_buffer`: Copies a block of 8000 longwords, 32000 bytes, from a global area into the buffer held in a global pointer.

### Total Eclipse (1989)(Incentive Software)[b].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.
