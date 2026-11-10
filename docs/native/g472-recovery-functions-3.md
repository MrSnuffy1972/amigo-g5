# Game 472 (Championship Manager '93): code recovery summary

## Functions (part 3 of 3)

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)
(continued)

- `decrement_and_branch`: Decrements a 16-bit value at offset 26 of a structure in a6, branches if non-zero, otherwise tests bit 7 of offset e of a6 and branches if zero or jumps to an offset relative to a6.
- `call_load_and_install_disk_font`: Pushes the return address onto the stack, calls load_and_install_disk_font, adjusts the stack pointer, and returns.
- `call_dispatch_font_request_2`: Pushes the contents of register a1 onto the stack, calls dispatch_font_request_2, cleans up four bytes from the stack, and returns.
- `dispatch_font_request_2`: As part of library code, this routine reads a pointer and an argument from the stack and passes them to another routine.
- `compute_scaled_offset_2`: Calculates a base value by subtracting a 16-bit word at offset 4 of the first structure from a 16-bit word at offset 256 of the second structure, then scales this difference by either 64 or -256.
- `load_and_install_disk_font`: Opens a font file, reads its header, and allocates memory for the font data.
- `copy_null_terminated_string_2`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `branch_to_offset`: Loads a pointer from offset 4 of the stack into a0, loads a long value from offset 8 of the stack into d0, and then branches to a relative offset.
- `skip_to_string_terminator_and_call_2`: The routine increments a pointer until it reaches the terminating zero byte of a zero-terminated string and then calls another routine with the pointer to that terminator.
- `count_string_length_2`: The routine receives a pointer to a zero-terminated byte string and returns the number of bytes before the terminating zero.
- `compare_memory_blocks`: Loads two pointers from the stack and branches to another routine.
- `test_and_branch`: Tests a byte at the address in a0 and increments it, then either branches to another location or clears d0 and branches to a different location.
- `compare_bytes_and_branch`: Compares a byte from a memory location in a0 with a byte from a memory location in a1, and branches based on the comparison result.
- `return_constant_18`: Returns the constant an address in d0 to the caller without any other effect.
- `find_last_occurrence_in_string_2`: Receives a pointer to a string and a byte value, determines the string length via a routine call, then walks backwards from the end to find the last occurrence of the byte and returns its pointer.
- `find_delimiter_and_advance_pointer_2`: Searches for the first occurrence of either a slash or a colon in a string by calling another routine twice, and returns a pointer advanced past the found delimiter.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_126_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_150`: Library stub: loads a library base from a global variable and calls the library function at offset minus 150 of that base, returning its result to the caller.
- `call_library_vector_156`: Library stub: loads a library base from a global variable and calls the library function at offset minus 156 of that base, returning its result to the caller.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `jump_to_dispatch`: Clears the low byte of d0 and jumps to a long address.
- `jump_to_font_installation`: Jumps to the routine that loads and installs a disk font.
- `jump_to_subroutine`: Performs a long jump to another routine.
- `mask_byte_and_increment_short`: Clears the most significant bit of a byte at offset 14 of a structure pointed to by a6 and increments a 16-bit value at offset 38 of the same structure.
