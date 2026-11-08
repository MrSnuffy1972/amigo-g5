# Game 895 (microprose golf (1991)(microprose)[cr flt](disk 1 of 3)[a2]): code recovery summary

## Functions (part 3 of 3)

### anim
(continued)

- `check_and_branch_logic`: Stores d0 at offset 14 of a structure in a3, pushes a PC-relative address onto the stack, compares d7 with d6, branches if d7 is less than or equal to d6, compares d0 with a constant, branches if d0 is greater than that constant, increments the stack pointer by 4, and branches to another routine.
- `tiny_addq_rts_5`: Small routine of 2 instructions that adds 6 to a7, then returns.
- `calculate_offset_and_branch_2`: Compares d6 with 97, branches if d6 is less than or equal to 97, loads a word from offset 1476 of a structure in a3 into d5, decrements d5 by 1, subtracts d5 from d0, subtracts d6 from d5, branches if d5 is greater than 0, loads an address from offset 634 of a structure in a2 into a2, adds d0 to d5, pushes a PC-relative address onto the stack, and branches to another routine.
- `execute_indirect_call_2`: Adjusts the stack pointer, loads a pointer from offset 634 of a structure in a2 into a2, decrements a 16-bit value at offset 14 of a structure in a3, and calls another routine via an indirect jump.
- `adjust_stack_pointer_11`: Adjusts the stack pointer by adding a constant offset and returns.
- `adjust_stack_and_return_4`: Adjusts the stack pointer by adding a constant offset and then returns.
- `update_pointer_and_call`: Loads a pointer from offset 634 of the structure in a2 into a2, decrements a 16-bit value from offset e of the structure in a3 by 1, and calls a routine at offset 626 of the structure in a1.
- `jump_via_a2_pointer`: Loads a constant address into a4 and jumps to the address stored in a2.
- `calculate_and_jump`: Pops three 16-bit values from the stack into d6, d5, and d1, calculates a value by subtracting d6 from d5, shifts the result right by 3, adds 1, and jumps to an address stored at offset 1356 of the structure in a3.
- `restore_stack_and_jump_2`: Adjusts the stack pointer by 12, performs a jump using a pointer from a constant address, and increments a byte at an offset within a structure.
- `restore_stack_and_jump`: Adjusts the stack pointer by 12, performs a jump using a pointer from a constant address, and increments a byte at an offset within a structure in a6.
- `adjust_stack_pointer_12`: Adjusts the stack pointer by adding 12 to it and returns.
- `restore_and_jump`: Adjusts the stack pointer by 12 and performs a jump using a pointer loaded from a constant address.
- `increment_and_jump`: Restores the stack pointer from an offset of 12, performs a jump, and increments a byte at an offset within a structure in a6 before calling another routine.
- `restore_stack_pointer_7`: Loads the stack pointer with the value stored at an offset of 12 bytes from the current stack frame and returns immediately.
- `jump_via_a1_pointer`: Calls restore_stack_pointer_7 to restore the stack pointer from an offset of 12, then pushes a4 onto the stack, loads a constant address into a4, and jumps to the address at offset 2 of a1.
- `push_a4_and_jump`: Pushes the value in a4 onto the stack, loads a constant address into a4, and jumps to the address at offset 2 of a1.
- `pop_into_a4`: Pops a long value from the stack into a4 and returns.
- `update_frame_and_jump_2`: Moves a longword from the stack to offset 20 of a structure in a3, pushes a longword from offset 1364 of a3 onto the stack, loads a longword into a2 using the program counter, stores a2 at offset 18 of a3, loads a new address from offset 1376 of a3 into a2, and branches.
- `store_and_push`: Pops a long value from the stack and stores it at offset 20 of the structure in a3, then pushes a long value from offset 1364 of the structure in a3 onto the stack.
- `update_frame_pointer`: Stores a longword from a register into offset 26 of the structure in a3, loads a new address from offset 560 of that same structure into a2, and branches to another routine.
- `transfer_stack_to_structure`: Moves a longword from the stack into a structure at offset 20 using a pointer from the stack, then pushes a longword from a global variable onto the stack.
- `load_pointer_and_branch_3`: Loads a longword from an offset of 1372 in the structure pointed to by a3 into a2 and branches to another routine.
- `update_frame_and_jump`: Writes a word from d0 to offset 14 of a structure in a3, compares a constant to d6 to calculate an offset using a value from offset 1492 of a structure in a3, and jumps to the address stored in a2.
- `update_structure_word_and_branch`: Writes a word from register d0 to an offset of 14 in a structure pointed to by a3, then performs a conditional branch based on a comparison between a constant and the difference between register d7 and d6, otherwise it adjusts the stack pointer and jumps to another routine.
- `increment_stack_pointer`: Increments the stack pointer by 6 and returns.
- `calculate_animation_frame_offset`: Compares a constant to a value in d6, and if greater, calculates a new offset by subtracting a value from a structure at offset 1492 in a3 from a constant, then updates a2 with this offset and calls another routine via the stack.
- `update_animation_index`: Adjusts the stack pointer, updates a2 with an offset from a2, decrements a 16-bit value at offset 14 of a structure in a3, and calls another routine; otherwise, it increments a byte at an offset within a structure pointed to by a6 and calls another routine.
- `adjust_stack_pointer_13`: Adjusts the stack pointer by adding 10 to it and returns.
- `adjust_stack_and_return_fixed`: Adjusts the stack pointer by adding 10 and then returns.
- `update_pointer_and_call_subroutine`: Updates a2 with an offset from a2, decrements a 16-bit value from offset 14 of the structure in a3 by 1, and calls a subroutine at an offset from a1.
- `load_address_to_a4_and_jump`: Loads a longword from a PC-relative offset into register a4 and jumps to the address stored in register a2.
- `calculate_and_jump_to_address`: Pops three 16-bit values from the stack, subtracts the first from the second, divides the result by 8, adds 1, and jumps to an address loaded from offset 1356 of the structure in a3.
- `dispatch_animation_increment_2`: Calls one of four different routines to increment either a 16-bit value at offset 44 of a pointer in a6 or a 16-bit value at offset 14 of another structure.
- `dispatch_animation_increment`: Calls one of four different routines to increment either a 16-bit value at offset 44 of a pointer in a6 or a 16-bit value at offset 14 of a structure in a6.
- `restore_stack_pointer_8`: Loads the stack pointer with the value stored at an offset of 12 from the current stack frame and returns immediately.
- `restore_and_jump_a4`: Restores the stack pointer from an offset of 12, restores a 32-bit address into a4 from the stack, and jumps.
- `increment_animation_offset_pointer`: Increments a 16-bit value at offset 44 of a pointer stored in a6 and calls increment_animation_offset.
- `restore_stack_pointer_9`: Loads the stack pointer with the value stored at an offset of twelve bytes from the current stack pointer and returns immediately.
- `restore_a4_and_jump`: Calls restore_stack_pointer_9 to restore the stack pointer from an offset of 12, restores a 32-bit address into a4 from the stack, and jumps to the address stored at the location pointed to by a4.
- `restore_a4_and_jump_to_address`: Restores a 32-bit address from the stack into register a4 and then jumps to the address stored at the location pointed to by a4.
- `pop_address_to_a4`: Moves a 32-bit value from the stack into a4 and returns.
- `move_stack_to_structure_5`: Moves a 32-bit value from the stack to offset 20 of the structure in a3 and then pushes a 32-bit value from offset 1364 of the structure in a3 onto the stack.
- `store_address_and_branch`: Stores a longword calculated from a relative offset into offset 26 of the structure in a3, loads a longword from offset 1376 of the structure in a3 into a2, and branches to another routine.
- `pop_stack_and_store_address`: Pops a longword from the stack into the structure in a3 at offset 20, pushes a longword from offset 1364 of the structure in a3 onto the stack, stores a longword calculated from a relative offset into offset 26 of the structure in a3, loads a longword from offset 1376 of the structure in a3 into a2, and branches to another routine.
- `transfer_stack_to_structure_2`: Moves a 32-bit value from the stack into a structure at offset 20 relative to a6, then pushes a 32-bit value from a global variable onto the stack.
- `process_animation_jump`: Writes a 16-bit word from d0 to offset 14 of a structure in a3, compares a register to a constant to perform arithmetic with a value from a structure in a3, and jumps to an address loaded from a table.
- `update_animation_frame_and_branch`: Writes a 16-bit word from register d0 to an offset of 14 of the structure in a3, pushes a relative address onto the stack, and performs a conditional branch based on a comparison between a constant and register d7.
- `adjust_stack_pointer_14`: Increments the stack pointer by six bytes and returns.
- `calculate_animation_frame_offset_2`: Compares a constant with a value in a register and, if the comparison fails, performs arithmetic involving a value from a structure in a3 and a register to calculate a new offset, then loads a 32-bit address from a table into a2 and calls another routine via a stack push and branch.
- `dispatch_animation_update`: Calls another routine to increment a 16-bit value at offset 44 of a pointer in a6 or at offset 14 of a structure, then adjusts the stack pointer and returns; otherwise, it loads a pointer from a structure in a2 into a2, decrements a 16-bit word at an offset of a structure in a3, and calls another routine.
- `adjust_stack_and_return_3`: Increments the stack pointer by 10 and returns.
- `update_animation_frame_index_8`: Increments a 16-bit value stored at an offset of 12 from a pointer in a6 and then calls adjust_stack_and_return_3.
- `decrement_and_call_subroutine`: Loads a pointer from an offset of a structure in a2 into a2, decrements a 16-bit word from an offset of a structure in a3 by one, and calls another routine.
- `load_animation_jump_target`: Loads a 32-bit address from a table located at a PC-relative offset into a register and jumps to the address stored in a2.
- `dispatch_animation_jump`: Subtracts a 16-bit value from another 16-bit value, divides the result by four, adds one, and then jumps to an address stored in a table at offset 1324 of a structure in a3.
- `indirect_jump_stub_26`: Jump stub: passes control to an address held in or computed from register a2, so the target is only known at run time.
- `perform_indirect_jump`: Calls an indirect jump stub that passes control to an address in a2 and calls jump_via_a2 to perform an indirect jump using the address in a2.
- `jump_via_a2`: Performs an indirect jump using the address held in register a2 and calls another routine.
- `increment_animation_pointer_2`: Increments a 16-bit value at offset 10 of a structure pointed to by a2 and calls another routine via an indirect jump stub.
- `update_animation_frame_pointer_2`: Increments a 16-bit value at an offset of 10 in a structure pointed to by a2 and calls another routine via an indirect jump stub.
- `increment_animation_index`: Increments a byte at an offset within a structure pointed to by a6 and calls update_animation_frame_index_9.
- `update_animation_frame_index_9`: Increments a byte at an offset within a structure pointed to by a6 and calls another routine via an indirect jump stub.
- `increment_animation_pointer_long`: Increments a 16-bit value at offset 104 of a structure pointed to by a6 and calls another routine via an indirect jump stub.
- `update_animation_frame_pointer_3`: Increments a 16-bit value stored at an offset of 104 from a structure in a6 and calls another routine via an indirect jump stub.
- `increment_animation_offset`: Increments a 16-bit value at offset 44 of a pointer in a6 and calls another routine via an indirect jump stub.
- `update_animation_frame_4`: Increments a 16-bit value at an offset of 44 from a pointer in a6 and calls another routine via an indirect jump stub.
- `increment_animation_frame_counter`: Increments a 16-bit value at offset 10 of the structure pointed to by a2 and calls another routine via an indirect jump stub.
- `update_animation_frame_5`: Increments a 16-bit value at an offset of 10 within a structure pointed to by a2 and calls another routine via an indirect jump stub.
- `increment_animation_pointer_short_offset`: Increments a 16-bit value at offset 14 of a structure pointed to by a6 and calls another routine via an indirect jump stub.
- `update_animation_frame_pointer_4`: Increments a 16-bit value stored at an offset of 14 in a structure pointed to by a6 and calls another routine via an indirect jump stub.
- `call_animation_logic`: Calls execute_animation_logic and then performs an indirect jump using the address held in register a2.
- `execute_animation_logic`: Performs a sequence of operations on data within the animation module.
- `execute_animation_logic_2`: Calls another routine to perform animation logic.
- `update_animation_frame_index_10`: Increments a byte at an offset within a structure pointed to by a6 and calls execute_animation_logic.

### MicroProse Golf (1991)(MicroProse)[cr FLT](Disk 1 of 3)[a2].adf:binddrivers

- `initialize_fast_memory_and_parse_args_2`: The routine attempts to locate a fast memory block and then retrieves the command line arguments from the operating system.
- `bind_expansion_drivers`: Opens expansion.library and icon.library to obtain a list of expansion devices, performs a lookup in a driver table for each, invokes the driver, and then restores the device before cleaning up the library handles.
- `find_driver_entry`: Scans a table of short values starting from an offset derived from a parameter, decrementing a counter and advancing a pointer until it finds an entry whose first short equals a constant and whose second short points back to the entry itself, then returns the pointer to that entry; otherwise it returns null.
- `iterate_driver_data`: Iterates through a buffer or list, performing arithmetic on a counter and updating a value via another routine.
- `calculate_string_index`: Uses a series of calls to determine an index and a value, then performs conditional logic to return a result or zero.
- `traverse_handler_chain`: Iterates through a linked list of handlers and updates a pointer in a structure if a certain bit is set in a memory location, then calls a library function.
- `follow_parent_chain`: The routine receives two pointers and walks a linked list by repeatedly following a field located at a fixed offset from the current node.
- `skip_non_slash_colon`: The routine checks a character string for a leading slash or colon.
- `parse_decimal_integer`: The routine reads a null-terminated string through a pointer and accumulates a decimal integer from consecutive ASCII digit characters, stopping at the first non-digit or null byte.
- `clamp_value_by_offset`: The routine calls another routine with the first two arguments and receives a result.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_alert`: Library stub: loads its arguments and the exec library base, calls Alert and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `call_user_state`: Library stub: loads its arguments and the exec library base, calls UserState and returns its result to the caller.
- `compare_two_strings`: The routine compares the byte strings pointed to by two registers byte by byte until a zero byte is found.
- `load_pointer_to_a0`: Loads a long value from offset 4 of the stack into a0.
- `compute_checksum`: Initializes a 32-bit accumulator to all ones and decrements it by one for each byte in a null-terminated string.
- `check_byte_match`: Compares a byte from a pointer in a0 with a byte in d0; if they match and the byte is not zero, it returns the pointer incremented by one, otherwise it returns zero or the original pointer depending on the comparison.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `verify_string_checksum`: The routine compares two byte strings and then computes a 32-bit checksum of a null-terminated string.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.

### MicroProse Golf (1991)(MicroProse)[cr FLT](Disk 1 of 3)[a2].adf:type

- `find_max_and_prepare_buffer_3`: Walks a linked list of entries to find the maximum value in a specific field, then optionally writes a header, fills a buffer with sequential words, and copies sixteen words from a temporary area to a global buffer.
- `transfer_and_branch`: Moves a long word from register d1 to d0 and then branches to another location.

### MicroProse Golf (1991)(MicroProse)[cr FLT](Disk 1 of 3)[a2].adf:setpatch

- `initialize_setpatch_environment`: Opens the current directory and the dos library, allocates memory, finds a task, and calls two routines to restore patch state and install a handler.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `self_calling_guard`: The routine calls another routine unconditionally, then checks a global variable and, if non-zero, calls another routine with the input parameter, passes the result to another routine, and finally calls another routine unconditionally.
- `install_setpatch_handler`: Installs a SetPatch handler and performs a series of conditional patches and library initialisations, including patching Exec exception vectors, layers.library, and trackdisk.device.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `call_indirect_via_table`: Calls another routine through a pointer stored in a global variable and returns the value produced by that indirect call.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `allocate_and_patch_memory`: Checks fields of a global structure and, if conditions are met, obtains a memory block and a code pointer via library calls, copies a small code stub from another routine into the target, updates a pointer in the global structure, computes a checksum over a short array in the structure and stores its complement, then returns.
- `process_and_jump_vector`: Iterates through a table of 22 words starting at an offset of 34 from a6, performs a bitwise NOT on the sum of these words, and stores the result at offset 82 of a6.
- `disable_interrupts_and_patch`: The routine checks a global variable and calls another routine through a table.
- `check_flag_and_copy_word`: The routine calls another routine and tests its return value.
- `execute_indirect_jump_3`: The routine loads a long word from a table into D1, compares it with a value at an offset of a structure in A0, and if they are not equal, loads a new address from a different offset of that structure into A0 and jumps to it.
- `compare_and_return`: The routine compares a long word in d1 with negative 256 in d0; if they are equal it branches to another location, otherwise it returns.
- `enable_interrupts_and_jump`: The routine sets bit 15 of the INTENA register and calls another routine that jumps to an address offset by 136 from a location relative to the program counter.
- `jump_to_offset_address_2`: Loads an address from a location relative to the program counter, adds 136 to it, and jumps to that address.
- `initialize_patch_function_table`: Writes a computed offset to a global variable and fills a 16-entry table with successive addresses of a routine, incrementing the pointer each iteration.
- `zero_register_d0`: Performs a bitwise OR on register d0 with zero.
- `call_subroutine`: Calls a subroutine at a fixed address.
- `execute_subroutine`: The routine calls another routine and then returns.
- `return_input_value`: The routine receives a value in D0 and returns it unchanged.
- `process_and_branch_logic_3`: Compares a word at offset 20 of a structure in a6 to 35, and if it is less than or equal to that value, calls another routine via a jump to an address relative to a6.
- `process_and_branch_logic_2`: Decrements a word in d2 and, if the result is not negative, decrements a long word in a3 and moves a value from offset 0 of the address in a3 to another location.
- `process_and_branch_logic`: Decrements a word in d2, and if the result is not negative, decrements a long word in a3, moves a value from offset 0 of the address in a3 to a1, moves a value from offset 4 of the address in a3 to d0, calls a routine using an offset of d2 from a6, and then branches to another location.
- `process_and_branch`: The routine loads a word from an offset of a4, shifts it left by three, adds 16, moves a4 to a1, calls another routine, loads a long word from a2, sets bit 31 of that word, and branches to another location.
- `call_library_vector_30_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `restore_processor_state`: Saves the address in register zero into a register, stores the contents of register one onto the stack at an offset of two, masks the stack pointer to a word alignment, and returns from an exception.
- `execute_and_return`: Performs a series of operations and calls another routine.
- `tiny_clr_movea_rts`: Small routine of 3 instructions that clears d0, then copies (a7)+ to a2, then returns.
- `scan_for_marker`: Iterates through an array of words starting from an input pointer.
- `calculate_word_offset`: The routine calls another routine to read a 16-bit value from two bytes after an input pointer and then moves the result into a register.
- `compute_word_offset`: Reads a 16-bit value from two bytes after an input pointer.
- `patch_global_pointers`: The routine sequentially calls a helper routine to compute offsets and stores the returned pointers into a series of global variables.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts_4`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### MicroProse Golf (1991)(MicroProse)[cr FLT](Disk 1 of 3)[a2].adf:echo

- `find_max_and_prepare_buffer_4`: Walks a linked list of entries to find the maximum value in a specific field, then optionally writes a header, fills a buffer with sequential words, and copies sixteen words between a temporary area and a global buffer.
- `transfer_register_and_branch_3`: Copies the contents of register d1 into register d0 and then performs a short branch to another routine.

### MicroProse Golf (1991)(MicroProse)[cr FLT](Disk 1 of 3)[a2].adf:failat

- `find_max_and_prepare_buffer_2`: Walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `transfer_register_and_branch`: Moves the contents of register d1 into register d0 and then performs a short branch to another routine.

### MicroProse Golf (1991)(MicroProse)[cr FLT](Disk 1 of 3)[a2].adf:cls

- `open_dos_library`: Opens the dos.library using a library base found at an offset of minus 48.

### MicroProse Golf (1991)(MicroProse)[cr FLT](Disk 1 of 3)[a2].adf:FastMemFirst

- `initialize_fast_memory_and_parse_args`: Attempts to locate a fast memory block and then retrieves the command line arguments from the operating system, parsing the raw string into a linked list of null-terminated argument strings.
- `find_and_copy_fastmem_block`: The routine walks a linked list of memory blocks starting from a global variable, selects the first fast-memory block with a specific type tag and the first chip-memory block with another type tag, and if both are found and different it calls another routine on the fast block and then calls another routine with the list head, the fast block and the or chip block.
- `call_library_vector_132_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_138_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.

### MicroProse Golf (1991)(MicroProse)[cr FLT](Disk 1 of 3)[a2].adf:Install_to_SYS

- `initialize_installation_environment`: Opens libraries, finds a task, forbids execution, manages messages, and executes a DOS command to set up the installation environment.

### MicroProse Golf (1991)(MicroProse)[cr FLT](Disk 1 of 3)[a2].adf:if

- `traverse_list_and_buffer_data`: Walks a linked list to find the maximum value in a specific field, then writes a header and fills a global buffer with sequential words.
- `transfer_register_and_branch_2`: Moves the contents of register d1 into register d0 and then branches to another routine.

### MicroProse Golf (1991)(MicroProse)[cr FLT](Disk 1 of 3)[a2].adf:Add21K

- `initialize_libraries_and_cleanup`: The routine calls library functions via base pointers, writes a byte through a pointer chain, and then calls additional library functions before returning.

### MicroProse Golf (1991)(MicroProse)[cr FLT](Disk 1 of 3)[a2].adf:Install_to_DH0

- `initialize_installation_environment_2`: Opens required libraries, finds a specific task, and forbids execution to manage messages before executing a DOS command to set up the installation environment.

### MicroProse Golf (1991)(MicroProse)[cr FLT](Disk 1 of 3)[a2].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
