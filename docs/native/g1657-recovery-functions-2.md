# Game 1657 (sim city (1989)(infogrames)[cr tristar - dcs][a]): code recovery summary

## Functions (part 2 of 2)

### info.library
(continued)

- `setup_string_template`: Library code calculates a bitmask from a value in a table and calls a string-formatting routine multiple times with different bit-masked arguments and a global pointer.
- `format_string_with_prefix`: Library code that prepares a string by applying a specific prefix and formatting arguments into a buffer.
- `format_string_arguments`: Library code that prepares a pointer-based argument list or a temporary buffer for a string-formatting routine.
- `calculate_string_length_3`: Library code that calculates the length of a null-terminated string by incrementing a counter until a null terminator is reached.
- `calculate_string_length`: Library code that increments a register while checking if a null terminator has been reached at an offset from a pointer.
- `copy_cstring_to_buffer`: Library code that copies a null-terminated byte string from a source pointer to a destination pointer, advancing both pointers and writing each byte until a zero byte is encountered.
- `copy_memory_block_4`: Library code that copies a specified number of bytes from a source memory location to a destination memory location.
- `copy_memory_block_2`: Library code that copies a fixed number of bytes from a source pointer to a destination pointer.
- `compare_strings_2`: Library code that compares two null-terminated strings for equality.
- `compare_strings`: Library code that compares two null-terminated strings and returns true if they are equal.
- `get_string_length`: Library code that counts the number of bytes in a null-terminated string.
- `execute_library_operations`: Library code that calls several other library functions, including a routine that initializes a structure array and routines that use conditional logic or call library vectors.
- `call_string_formatter`: Library code that calls another routine using a pointer to a global variable and a local buffer, then calls another routine using the populated buffer.
- `copy_string_data_2`: Library code that copies a sequence of bytes from a source location to a destination location and appends a null terminator.
- `copy_string_data`: Library code that copies bytes from a source location to a destination location and appends a null terminator.
- `initialize_record_structure`: Library code that populates a record structure with various parameters and calls a function to process it.
- `execute_library_function`: Library code that calls a library function using a pointer retrieved from a global variable offset.
- `execute_library_calls`: Library code that calls a library function or another routine multiple times using a pointer retrieved from a structure member.
- `invoke_table_entry_3`: The library code retrieves a function pointer from an offset within a global variable and executes it.
- `store_byte`: Library code that stores a byte from a register into a memory location pointed to by another register.
- `call_indirect_routine_via_table`: The routine calls another routine indirectly through a pointer stored in a global variable table.
- `call_indirect_via_table_7`: The routine calls another routine through an indirect pointer taken from a global table.

### version.library

- `return_zero_11`: This library routine returns zero.
- `store_library_pointers`: The routine writes two 32-bit values from registers into a structure pointed to by a register, storing one value at an offset and another at a different offset.
- `increment_counter_and_clear_flag`: The routine increments a short value at an offset from a base pointer and clears a bit in a byte at another offset from the same base pointer.
- `decrement_counter_and_call`: Library code that decrements a short counter at a fixed offset from a pointer and calls another routine if the counter reaches zero and a specific bit in a flag byte is set.
- `execute_library_callbacks`: Library code that checks a flag in a structure and either sets a bit or executes two functions through pointers retrieved from a table.
- `get_library_value`: As part lack of a shared library, this routine retrieves a value from an offset in a passed pointer and calls two other functions via pointers.
- `return_zero_12`: This routine is an empty function that returns zero to the caller.

### Sim City (1989)(Infogrames)[cr Tristar - DCS][a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
