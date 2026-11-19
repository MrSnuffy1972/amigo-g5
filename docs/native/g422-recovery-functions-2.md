# Game 422 (Xtreme Racing V1.0): code recovery summary

## Functions (part 2 of 2)

### XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:FrontEnd#unpacked
(continued)

- `free_structured_memory_3`: If d1 is not zero, it calls a routine to free a memory block pointed to at offset 4 of the structure in d1 using the size at offset 8, and then calls a routine to free the structure itself.
- `parse_line_to_buffer_2`: Copies bytes from a source string to a destination buffer, incrementing a counter and checking for a newline character or a specific byte value.
- `compare_case_insensitive_strings_2`: The routine compares two byte strings character by character, converting uppercase letters to lowercase for comparison, and returns the difference between the first mismatching characters or zero if equal.
- `find_substring_2`: Searches for the first occurrence of one null-terminated string within another null-terminated string.
- `copy_n_terminated_bytes_2`: The routine copies bytes from a source pointer to a destination pointer until a zero byte is encountered, stopping early if a counter reaches zero and returning the original destination pointer.
- `initialize_parsing_context`: Clears a longword at offset 24268 of a structure in a4, stores a value from the stack at offset 24272 of the structure, and calls another routine.
- `pop_byte_from_list`: Increments a pointer in a table, reads a byte from the location pointed to by the original pointer, updates the table with the new pointer, and returns the byte or negative one if the byte was zero.
- `decrement_two_structure_members`: Decrements two longwords located at offsets 24268 and 24272 from a pointer held in a4 and returns.
- `parse_config_string`: Parses a string to extract a numeric value by iterating through characters and checking for specific delimiters and range constraints.
- `parse_config_string_2`: As library code, it parses a string to extract numeric values by iterating through characters and checking for specific delimiters and range constraints.
- `convert_bits_to_ascii`: Performs a bitwise rotation on d0 and stores the resulting bits as ASCII characters in a buffer pointed to by a1 through a loop that runs 31 times.
- `format_numeric_string`: Performs repeated division and modulo operations on a 16-bit value using divisors from a table, storing the resulting digits as ASCII characters in a buffer.
- `modify_memory_and_registers`: Moves a word from a table to a destination calculated from a5, a4, and a2, sets a bit in a byte at an offset in a5, and performs several bitwise OR operations on d0, d1, and a memory location pointed to by a0.
- `copy_rotated_bytes`: Performs a loop seven times, rotating the value in d0 left by four bits and using the lower byte to index into a table to copy bytes to the destination pointed to by a1.
- `set_timer_and_wait`: Sets bit 3 of a byte at offset 1 of a global variable, stores a 16-bit value from d0 into the CIAA timer registers, and loops until bit 0 of the CIAA interrupt control register is set.
- `reset_timer_hardware`: Sets bit 7 of a byte at offset 1 of a global variable, writes a 16-bit value from d0 into the CIAA timer registers, clears bit 7 of that same byte, clears the timer registers, and writes 132 to the CIAA interrupt control register.
- `set_timer_via_pointer`: Sets bit 3 of a byte at offset 3584 of a structure in a5, stores a 16-bit value from d0 into offsets 1024 and 1280 of the structure, and loops until bit 0 of the byte at offset 3328 is set.
- `set_timer_via_pointer_alt`: Sets bit 3 of a byte at offset 3840 of a structure in a5, stores a 16-bit value from d0 into offsets 1536 and 1792 of the structure, and loops until bit 1 of the byte at offset 3328 is set.
- `reset_hardware_timers`: Calls another routine, then reads bytes from CIAA and CIAB registers to store them in a table, modifies CIAA and CIAB control registers to set bit 4, and finally stores 16-bit timer values from CIAA and CIAB into a table.
- `restore_hardware_timers`: Writes 16-bit values from a table into CIAA and CIAB timer registers and writes bytes from a table into CIAA and CIAB control registers, then calls another routine.
- `write_constant_values`: Stores 255 into offsets 1024 and 1280 of a structure in a5 and stores 24 into offset 3584 of the same structure.
- `set_bit_zero`: Sets bit 0 of a byte at offset 3584 of a structure in a5.
- `clear_bit_zero`: Clears bit 0 of a byte at offset 3584 of a structure in a5.
- `calculate_and_call_subroutines`: Saves registers, loads a byte from an offset of 1280 of a structure in a5, shifts it left by 8 bits, adds a byte from offset 1024 of the same structure, negates the result, subtracts 1, calls another routine with a pointer in a1, sets d1 to 5, calls another routine, and restores registers before returning.
- `open_hardware_resources`: Calls a function to open a resource and stores the result in a global variable, then if successful, calls another function to open a second resource and stores that result in another global variable.
- `close_hardware_resources`: Calls a function via a pointer in a global variable and stores the result in another global variable, repeats this for a second global pointer, and then writes 127 to the CIAA and CIAB interrupt control registers.
- `call_two_functions`: Checks two pointers in a structure; if a pointer is non-zero, it calls a function at an offset of negative 18 from that pointer after setting bit 7 of a word read from an offset of 20 from the pointer.
- `empty_stub_13`: Empty routine: returns to the caller immediately without doing anything.
- `compare_bytes_and_check_zero`: Compares two bytes from a1 and a2, and if they are equal and the first byte is zero, sets d0 to one; otherwise it returns.
- `function_epilogue_48`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `calculate_string_length_2`: Calculates the length of a null-terminated string pointed to by a1 and returns the result in d0.
- `copy_byte_conditionally`: Copies a byte from a1 to a2 and branches based on whether the byte is zero.
- `function_epilogue_49`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `copy_string_with_limit_3`: Copies bytes from a1 to a2 up to a specified count in d0, and if the loop completes without finding a null terminator, writes a zero to a2.
- `convert_lowercase_to_uppercase`: Checks if a byte from a1 is a lowercase letter and, if so, converts it to uppercase by clearing bit 5 and writes it to the memory location before a1.
- `function_epilogue_50`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `convert_uppercase_to_lowercase`: Checks if a byte from a1 is an uppercase letter and, if so, converts it to lowercase by setting bit 5 and writes it to the memory location before a1.
- `function_epilogue_51`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `process_string_segment`: Saves registers, clears d1, loads a pointer from a1 into a2, calls another routine, loads a byte from a1 into d2, and if that byte is not a space or a tab, checks if d1 is non-zero, subtracts d1 from d0, and calls a routine at an address stored in a table.
- `increment_and_branch_2`: Increments d1 by 1 and branches to another location.
