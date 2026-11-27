# Game 2317 (king's quest i - quest for the crown (1990)(sierra)(disk 1 of 4)[cr skr][h prx]): code recovery summary

## Functions (part 3 of 3)

### King's Quest I - Quest for the Crown (1990)(Sierra)(Disk 1 of 4)[cr SKR][h PRX].adf:kq1
(continued)

- `store_param_and_clear_flag`: Stores the first parameter into a global variable, calls another routine twice with the stored value and the second parameter, then clears the global variable by writing zero to it and returns the result of the inner call.
- `increment_and_store_byte`: Reads a byte pointer from a global variable, increments that global variable, stores the low byte of a parameter through the pointer, and returns the byte.
- `load_structure_pointer`: Sets up a stack frame, saves registers, loads an address from an offset in a5 into a2, compares a word at offset 18 of the structure in a5 with 4, and if they are not equal, branches to another routine; otherwise it loads a long value from the address in a0 into d4 and branches to another routine.
- `load_word_from_table`: Checks if a word at offset 3 of a structure in a5 is greater than zero, then loads a word from a table pointed to by offset 8 of that structure into d4.
- `copy_bytes_from_table`: Copies bytes from a table into a buffer using a length returned by another routine, and optionally terminates the buffer with a dash character if a flag in a structure is set.
- `parse_format_string_tokens`: Parses a format string to extract numeric values, floating-point numbers, or strings, and populates a stack or structure with the decoded data.
- `clear_and_branch_12`: Clears a 16-bit word at a negative offset from a6 and branches to another routine.
- `parse_encoded_data_stream_2`: Decodes a numeric value from a data stream by multiplying a running total by ten and adding the current byte minus forty-eight, then processes specific control characters to update a structure's length and data pointers.
- `clear_and_branch_11`: Clears a 16-bit word at an offset of 12 from a6 and then branches to another routine.
- `parse_encoded_data_stream`: Calculates a value by multiplying a 16-bit integer by ten and adding a byte from a pointer, then enters a loop that reads bytes from a stream to update a structure.
- `extract_format_type_byte`: Checks if a 16-bit value in register d4 equals 104; if so, it increments the pointer in a3, reads a byte from the new address, sign-extends it to a 16-bit word in d4, then sign-extends that result to a 32-bit long in d0 before branching to another location.
- `store_eight_at_offset`: Stores the word eight at offset negative eighteen of the structure in a5 and branches to another routine.
- `store_ten_at_offset`: Stores the word ten at offset negative eighteen of the structure in a5 and branches to another routine.
- `store_sixteen_at_offset`: Stores the word sixteen at offset negative eighteen of the structure in a5 and branches to another routine.
- `parse_formatted_string_2`: Iterates through a string to calculate a length or process content, handling specific character sequences such as signs or percent symbols by calling another routine for each character.
- `calculate_string_length`: Loads a pointer from a structure, iterates through a null-terminated string to determine its length, and stores the resulting length as a 16-bit value at offset 16 of the structure.
- `advance_string_pointer_and_store_byte`: Increments a pointer stored at offset 4 within a structure by 2, loads a word from the address pointed to by that pointer into a register, stores a pointer to a local buffer at offset 22 of the structure, and writes the previously loaded word's low byte to the first byte of that buffer before branching.
- `dispatch_string_format_type`: Compares a value in d0 against several constants to determine a format type, then stores either 8, 10, 16, or other specific values into a 16-bit field at offset negative eighteen of a structure in a5 before branching to different subroutines.
- `process_string_via_callback`: Calculates a length value based on a pointer and a limit within a structure, then iterates through a string by calling another routine for each character.
- `branch_into_another_routine`: Performs a short relative branch into the middle of another routine.
- `loop_and_compare_d0`: Pushes a word from a stack offset to the stack, calls another routine, and loops if the result in d0 is not negative one.
- `update_and_validate_counter`: Increments a 16-bit word at offset 14 of the structure in a5, compares a 16-bit word at offset 22 with a value moved into d0 from offset 22, and either clears a 16-bit word at offset 18 or branches to another location.
- `increment_and_call_d0`: Increments a value at a stack offset, loads a byte from the resulting address into d0, and calls another routine, looping if d0 is not negative one.
- `increment_and_loop_counter`: Increments a 16-bit word at offset 14 of a structure in a5, loads a pointer from offset 16 of a5, and checks if the byte at that pointer is zero.
- `call_subroutine_with_constant`: Pushes the constant 32 onto the stack, calls a subroutine at the address in a2, and enters a loop that continues as long as d0 is not equal to 65535, otherwise it sets d0 to 255 and branches to another routine.
- `update_and_compare_counter`: Increments a 16-bit value at offset negative six of the structure in a5, compares a 16-bit value at offset negative fourteen of the structure in a5 with the value in d0, and branches based on the comparison result or otherwise branches to another routine.
- `call_subroutine_with_d4`: Pushes the contents of d4 onto the stack, calls a subroutine at the address in a2, and enters a loop that continues as long as d0 is not equal to 65535, otherwise it sets d0 to 255 and branches to another routine.
- `increment_stack_offset`: Increments a 16-bit value at a specific offset in the stack frame and branches to another location.
- `branch_with_offset_value`: Loads a 16-bit word from an offset of negative 6 from a5 into d0 and branches to another routine.
- `perform_signed_division_2`: Checks the signs of d0 and d1, calls another routine, and optionally negates the result before combining it with d1 into a 32-bit value.
- `prepare_signed_division_2`: Clears d4, checks the sign of d0 and d1, negates them if they are negative, increments d4 if d0 was negative, and then calls another routine before moving d1 into d0 and branching.
- `call_and_return_concatenated_2`: Pushes the contents of register d1 onto the stack, calls perform_math_operations, and returns a 64-bit value formed by concatenating the original d1 contents with the value left in d1 by the call.
- `execute_math_with_stack_transfer`: Pushes the contents of register d1 onto the stack, calls perform_math_operations, restores the value to register d1 from the stack, tests register d0, and returns.
- `perform_math_operations`: Swaps the high and low words of register d1, performs division using register d3 as a divisor for values in d0 and d1, and returns the result.
- `swap_and_rotate_words_2`: Swaps the high and low words of a value, then repeatedly doubles a pair of words while propagating carry and performing conditional subtractions fifteen times.
- `update_global_value`: Sets up a stack frame, saves registers, loads a word from offset 8 of a structure in a5 into d4, calls another routine, multiplies d0 by 6, calculates an address using d0 and a global offset, compares d4 against a value in a global variable, and if certain conditions are met, writes 2 to a global variable and returns -1.
- `set_error_constant`: Checks if the two least significant bits of a word at offset 4 of a structure in a2 are non-zero; otherwise, it writes the value 5 to a global variable and sets d0 to 255.
- `handle_io_error`: Calls another routine with parameters from a structure, and if the result is negative, calls a second routine and stores the result in a global variable at a specific offset.
- `move_d5_to_d0`: Copies the contents of register d5 into register d0 and branches to another routine.
- `check_library_status`: Calls another routine with a pointer to a memory location, then checks bit 12 of the result and a global variable to decide whether to call a third routine before returning.
- `call_and_clear_d0`: Calls another routine, sets d0 to zero, and branches to another location.
- `chain_result_to_output`: Passes the result of a function call to a write wrapper followed by an output wrapper.
- `invoke_pointer_and_process_event`: Checks a 32-bit value at offset -32514 of a4 and, if non-zero, calls the routine pointed to by that value, then pushes a 16-bit value from offset 8 of a5 and calls another routine.
- `process_scripted_event`: The routine checks a set of flags stored in a structure pointed to by a4 and conditionally calls other routines with values taken from that structure, updates a pointer field from the structure, and optionally invokes a supervisor-mode callback and a global function pointer.
- `restore_stack_and_return`: Moves a 32-bit value from the stack into register d4, unlinks the stack frame, and returns.
- `process_table_entry`: Calculates an offset into a table based on a value from the stack, checks bounds, and if valid, calls another routine through a pointer at that table entry before clearing the entry and returning zero; otherwise it sets a global variable to two and returns negative one.
- `call_close_2`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `jump_to_current_dir_handler`: Performs an indirect jump to a routine stored in a global variable structure.
- `call_current_dir_2`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delete_file_2`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_dup_lock`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `call_examine_2`: Wrapper that calls the dos library function Examine and returns its result to the caller.
- `call_input_2`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_load_seg_2`: Wrapper that calls the dos library function LoadSeg and returns its result to the caller.
- `dispatch_indirect_call_3`: Loads a pointer from a location calculated using an offset from a value in a register and calls the routine at that address.
- `call_lock_2`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `call_open_2`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output_2`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_seek_2`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `execute_indirect_function`: Performs an indirect jump to a function pointer retrieved from a nested memory structure via an offset from a value in a register.
- `call_un_lock_2`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_write_2`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_108_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_offset_414`: Loads a library base from a global variable and calls the library function at offset 414 from that base.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_offset_198`: Loads a library base from a global variable and calls the library function at offset 198 from that base.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_offset_294`: Loads a library base from a global variable and calls the library function at offset 294 from that base.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_offset_210`: Loads a library base from a global variable and calls the library function at offset 210 from that base.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_offset_372`: Loads a library base from a global variable and calls the library function at offset 372 from that base.
- `call_library_vector_372_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_offset_552`: Loads a library base from a global variable and calls the library function at offset 552 from that base.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_378_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_offset_384`: Loads a library base from a global variable and calls the library function at offset 384 from that base.
- `call_library_vector_384_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_96_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_78_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.

### King's Quest I - Quest for the Crown (1990)(Sierra)(Disk 1 of 4)[cr SKR][h PRX].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_stub`: Call stub: calls a routine at a fixed address outside this code unit, then returns to its own caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant_7`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_custom_stack_pointer`: Adjusts the stack pointer by four bytes, writes a specific memory address to a global variable, and saves several registers to the stack before returning.
- `frag_move_ori_eori`: Short code fragment that copies d0 to d2, then merges 0 into d0, then exclusive-ors 0 into d0, then merges 0 into d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_entry`: Invokes another routine and checks the zero flag.
- `open_dos_library`: Calls the exec library function OldOpenLibrary to load dos.library and stores the resulting library handle in a global variable.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_library_vector_60_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_144_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.

### SkidRow#unpacked

- `call_library_entry_2`: Calls one routine and, if the zero flag is clear, calls a second routine and then jumps through a function pointer taken from a global table.
- `open_dos_library_2`: Calls the exec library function OldOpenLibrary to load dos.library and stores the resulting library handle in a global variable.
- `call_close_library_2`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_output_3`: Wrapper that calls the dos library function Output and returns its result to the caller.
