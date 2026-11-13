# Game 1254 (plan 9 from outer space (1992)(gremlin)(m3)[cr tkk]): code recovery summary

## Functions (part 2 of 2)

### plan9
(continued)

- `process_conditional_callbacks`: Saves registers, calls a routine, then branches on bits of a global flag to invoke callback pointers and update linked-list head pointers, restoring the saved registers before returning.
- `clear_flag_then_dispatch_callbacks`: Clears a byte in a global variable, then calls a routine that saves registers, dispatches callback pointers based on bits of a global flag, restores saved registers, and returns.
- `build_data_table`: Calls a system routine to obtain a pointer, stores it at offsets 4 and 16 of the structure in a2, then fills a table starting at that pointer with alternating constant words and data words, and finally enters a loop to append structured pairs derived from a source table.
- `copy_interleaved_words`: Computes an index from the input parameters and copies sixteen words from two source tables into a destination buffer, interleaving two words per iteration.
- `read_vposr_masked`: Reads the vertical position register, shifts it right by eight bits, masks off the low bits, and returns the result.
- `wait_vposr_exceeds`: Waits in a loop until the masked vertical position register value is greater than the value in the first input register, otherwise it returns.
- `decrement_and_check_value`: Compares d0 against a global pointer and a null value, and if neither matches it decrements the byte at offset 12 of the pointer in d0 and calls itself when the result is negative before restoring the saved a0 and returning.
- `add_base_to_elements`: Reads a word from a table that holds a count, then adds the table base address to each of the following elements, decrementing the loop until the count is exhausted.
- `process_nested_parameters`: Calls a system routine to obtain a pointer, then conditionally calls another routine using a stack argument and stores its result at offset 12 of the structure in a0, calls it again with a second stack argument and stores that at offset 8, and finally copies two words from a source structure into offsets 4 and 6.
- `initialize_global_pointers`: Saves registers, stores a value at a global longword, loads a pointer from a global variable, calls a memory setup routine, invokes a function through a table, stores the result and a zero into global variables, calls another routine, and stores its result into three global words before restoring registers and returning.
- `call_callback_and_reset`: Stores a global variable address into two other global variables, then calls the function pointer in a global variable when it is non-zero and clears it, and finally stores a constant into another global variable.
- `allocate_resource_node`: Pushes the value in d1 onto the stack, loads the decimal value 32 into d1, and calls another routine.
- `allocate_resource_node_2`: Loads the decimal value 64 into d1, calls the resource allocation routine, then restores d1 and returns.
- `allocate_resource_node_3`: Clears d1, calls the resource allocation routine, then restores d1 and returns.
- `manage_resource_node_allocation`: Rejects the call when the incoming word is negative, clears a global byte at offset 476, then reads a pointer from a routine and branches to the handler at address 958 when that pointer is zero.
- `read_data_compare_length`: Tests a status byte in a global variable and returns early when it is zero, otherwise pushes registers and calls a library routine to read data, comparing the returned length with a stored value and returning it when they match.
- `add_value_to_table`: Adds a value from a register to eight consecutive words in a table starting at an offset, using a fixed loop counter.
- `loads_magic_and_jumps`: Loads the value a fixed address into a global variable and then jumps to the routine at address 958.
- `call_and_clear_flag`: Pushes a stack frame marker and a word of all ones, then calls another routine and removes the pushed values from the stack.
- `call_and_set_flag`: Pushes a stack frame marker and a word of all ones, then calls another routine and removes the pushed values from the stack.
- `run_sequenced_setup_calls`: Saves all data and address registers, runs a long sequence of subroutine calls while clearing several global words and copying a pointer from one global to another, then branches on a global byte to conditionally allocate a block, fill its fields, and invoke more handlers before restoring the registers and returning.
- `call_via_vector_table`: Pushes a word of all ones onto the stack, calls another routine through the vector table at offset a fixed address, removes the pushed word from the stack, and returns.
- `reset_global_state`: Saves the registers, runs a series of subroutines, clears several global words, copies values between global variables, and pushes stack parameters before calling more subroutines and restoring the registers.
- `initialize_and_call_routines`: Clears several global variables, saves registers, and runs a long sequence of subroutine calls using values from a global pointer before restoring registers.
- `copy_and_process_block`: Allocates a resource, copies six words from a source pointer into a global buffer, and then runs a sequence of handler and blitter routines for two blocks.
- `process_and_copy_block`: Calls a routine to allocate a resource, clears the words at offsets 20 and 22 of the structure, and runs a sequence of handler and blitter routines.
- `init_and_fill_structure_table`: Returns immediately when a global longword is already set, otherwise saves the registers and allocates a block that it stores into that global, then allocates several more blocks and stores their pointers into fixed globals, and finally loops over a table of pointers filling each allocated node with copied fields and computed offsets.
- `clear_conditional_flags`: Tests a global longword and returns immediately when it is zero, otherwise saves eight registers, loads a pointer from that global, sets d0 to 65535, calls two routines, then checks two other global longwords and calls a routine and clears each one when non-zero, clears the first global, restores the registers, and returns.
- `init_and_scan_table`: Allocates a stack frame, saves registers, and clears two local buffers by looping six times.
- `allocate_and_jump`: Loads the constant 32 into d0, calls an allocation routine, calls another routine, and then jumps to a fixed address.
- `loop_marker_allocation`: Loops by incrementing a counter stored at offset 30 of the structure in a6, calling allocation routines and writing a marker byte into a table indexed by a computed value, and returns once the counter reaches twelve.
- `execute_sequential_calls`: Pushes two words and a global pointer onto the stack, calls a routine with them, removes the pushed values, sets d0 to ten, calls two more routines, restores the registers, restores the frame pointer, and returns.
- `reset_byte_and_call_routines`: Clears a global byte at offset 3924 to zero, then runs a sequence of eight subroutine calls including one that passes a value shifted from a global longword and a word of all ones as parameters, and finally returns the concatenated values of d0 and d1.
- `reset_byte_and_call_routines_extended`: Clears a global byte at offset 3924 to zero, then runs a sequence of ten subroutine calls including one that passes a value of eight and another that passes a value of zero, plus one using a shifted global longword and a word of all ones, and finally returns the concatenated values of d0 and d1.
- `reset_and_clear_state`: Clears a global word, runs several subroutines, clears two bytes of a structure, calls a chain of routines, and finally stores a constant into that global word.
- `reset_byte_and_call_routines_reduced`: Clears a global byte at offset 3924 to zero, then runs a sequence of seven subroutine calls including one that passes a value of zero and one using a shifted global longword and a word of all ones as parameters, and finally returns the concatenated values of d0 and d1.
- `populate_list_from_source`: Checks whether the word at offset 8 of the structure in a1 is nonzero and returns early when it is, otherwise initializes a table with derived values and enters a loop that repeatedly calls a routine, storing each returned pointer into the table.
- `iterate_and_call_list`: Saves registers d0-d1, a0 and a2, then loads a pointer from offset 8 of the structure in a1 and advances it by two.
- `copy_four_words_from_struct`: Calls a routine to obtain a pointer, then copies four consecutive words from the structure in a1 into that pointer at offsets zero, two, four, and six, and returns.
- `call_conditional_recursive_routine`: Loads a value from a structure and calls another routine using that value, then returns.
- `setup_blitter_transfer_area_2`: Pushes several values from a structure pointed to by a1 onto the stack and calls another routine.
- `load_global_address`: Checks whether the input register is zero and loads a global variable address into the return register, taking the same path in both cases before returning.

### Plan 9 From Outer Space (1992)(Gremlin)(M3)[cr TKK](Disk 1 of 4).adf:tkk

- `store_address_and_trap`: Saves registers d1-d7 and a0-a6, computes its own return address and stores it into a global variable, then executes trap zero to enter the operating system.
- `setup_interrupt_system`: Disables interrupts, configures a CIA port, disables coprocessors, sets up interrupt vectors in a table, clears a block of memory, stores handler addresses into a custom register block, waits for a flag to clear, copies a block of code, and re-enables interrupts and DMA before returning.
- `fill_memory_with_pattern`: Initializes two tables with the alternating bit patterns a fixed address and a fixed address, then repeatedly calls a series of helper routines in nested loops to fill a region of memory with the value a fixed address.
- `clear_register_zero`: Clears the low register to zero and returns.
- `shift_and_call_twice`: Copies the value in register d0 into register d3, shifts d0 right by one bit, calls a routine, restores the original value into d0, and calls the same routine again.
- `test_flags_and_set_bit`: Reads a byte from a memory location, tests a flag bit there and returns if it is set, tests bit six of the byte and sets bit seven if it is clear, then returns.
- `clear_bit_store_byte`: Clears bit seven of a register value and stores the result through a pointer, then returns.
- `apply_bit_manipulation_to_pointer`: Combines a value in d0 with a mask in d4 using shifts and bit operations, then stores the result through a pointer, clearing the sign bit only when the low bit of the preceding byte is set.
- `xor_accumulate_longword_table`: XORs each 32-bit word of a table into an accumulator a number of times taken from a size register shifted right by two, then returns the accumulator masked by a register value.
- `clear_struct_word`: Calls a helper routine, then writes zero to the signed word at offset 136 of the structure in a6, and returns.
- `store_vectors_and_words`: Calls a routine, then loads a pointer from a global vector and stores two longwords from it into consecutive memory locations while copying three word values from a table into a structure.
- `set_values_to_max`: Stores the maximum signed word value into three consecutive word-sized locations relative to a pointer.
- `initialize_memory_values`: Calls two routines, then stores three maximum signed word values from a table into consecutive locations relative to a structure, copies two longwords into global variables, and clears a word at a fixed offset.
- `set_sign_bit_and_store_word`: Sets bit 15 of a word in d0 and stores it through a pointer, advancing that pointer by one word.
- `wait_for_flag_to_clear`: Loops while a byte at offset 6 of a structure is nonzero, otherwise it returns.
- `store_byte_to_offset`: Stores the value 32 into a word at offset 92 of a structure, then restores the saved registers and returns.
- `copy_string_until_null`: Scans a string for a null terminator, then copies eight bytes from a source string into a destination and repeats the copy while advancing through the source until it reaches a terminator.
- `copy_string_to_memory`: Compares a byte from a source against a global string and then copies eight bytes from a source string to a destination using a fixed offset increment.

### Plan 9 From Outer Space (1992)(Gremlin)(M3)[cr TKK](Disk 1 of 4).adf:bootblock

- `check_library_result`: Calls a system library routine and returns negative one if the result is zero, otherwise it follows a pointer stored at a fixed offset and returns zero.
- `configure_beam_control`: Writes a value to the beam control register, loads a pointer from a system vector, and then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `reset_global_flag`: Checks if a global variable is non-zero, then calls two routines using pointers from that variable before clearing the global variable and returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_12`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `function_epilogue_5`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant_7`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_constant_to_global`: Stores a constant value into a global variable and restores registers from the stack before returning.
- `call_function_via_pointer`: Loads a pointer from an offset of a structure in a1, loads another pointer from an offset of a table at a0, and calls the second pointer.
