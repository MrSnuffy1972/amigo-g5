# Game 1027 (ace the space case (1994)(grand master design - quartz pd)(pd)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01027). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 79 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 352 of 352 named (0 author symbols), 352 of 352 described (52 by rule, 259 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 83 of 83 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4496 of 4496 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 360008 |
| code | 13386 |
| data | 153235 |
| padding | 16603 |
| unused | 357888 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| FastMemFirst | 7 | 207 | 6 |
| mathtrans.library | 17 | 830 | 97 |
| icon.library | 10 | 1375 | 109 |
| mathieeedoubbas.library | 3 | 568 | 47 |
| diskfont.library | 15 | 1040 | 76 |
| Ace the Space Case (1994)(Grand Master Design - Quartz PD)(PD).adf:. | 2 | 233 | 8 |
| Ace the Space Case (1994)(Grand Master Design - Quartz PD)(PD).adf:ACEDEMO! | 2 | 233 | 8 |
| Ace the Space Case (1994)(Grand Master Design - Quartz PD)(PD).adf:bootblock | 1 | 10 | 1 |

## Cases handled

- ace the space case (1994)(grand master design - quartz pd)(pd)/Ace the Space Case (1994)(Grand Master Design - Quartz PD)(PD).adf:ACEDEMO!: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### FastMemFirst

- `initialize_fast_memory_and_parse_args`: Locates a fast memory block and retrieves the command line arguments from the operating system, parsing the raw string into a linked list of null-terminated argument strings while handling quoted strings and specific character conversions.
- `find_and_copy_fastmem_block`: The routine walks a linked list of memory blocks starting from a global variable, selects the first fast-memory block with a specific type tag and the first chip-memory block with another type tag, and if both are found and different it calls another routine on the fast block and then calls another routine with the list head, the fast block and the chip block.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.

### mathtrans.library

- `execute_illegal_instruction`: This library code contains an illegal instruction.
- `store_a0_to_memory`: This library code stores the contents of register A0 into a longword at an offset of 312 from the start of the library.
- `unlink_and_free_memory`: If a 16-bit value at offset 32 of the structure in A6 is zero, this library code unlinks two nodes in a linked list and calls a routine via a pointer in a global variable; otherwise, it sets bit 3 of a byte at offset 14 of the structure in A6 and returns 0.
- `set_status_bit`: This library code sets bit 3 of a byte at offset 14 of the structure pointed to by A6 and returns 0.
- `increment_and_initialize_structure`: This library code checks bit 3 of a byte at offset 14 of the structure in A6; if it is not set, it increments a word at offset 32 of the structure, and if that word becomes 1, it calls another routine using a pointer from offset 4 of the structure.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `decrement_counter_and_call`: Library code that decrements a 16-bit counter at offset 32 of the structure at a6, calls another routine if the counter reaches zero and bit 3 of the flag byte at offset 14 is set, and calls another routine with a global value when the counter is zero.
- `mask_and_set_carry`: Library code that masks d0 with 127, calls another routine, and sets the carry bit in the status register.
- `calculate_bitfield_difference`: This library code performs arithmetic operations on D0 and D1, including shifts and swaps, and prepares several long values in D4, D5, and D6 before branching to another location.
- `adjust_bitfield_value`: Library code that sets the bit selected by register D1 in D0, then repeatedly shifts a running difference with a doubling accumulator until a termination condition is met.
- `mask_and_branch`: This library code moves a long value from D0 to D7, calls a routine via a pointer in a global variable, sets bit 1 and clears bit 3 of D0, then branches to another routine.
- `call_jump_table_entries`: This library code calls a series of function pointers located at fixed offsets from a register-derived base in an alternating pattern, then calls a third entry once and calls the second entry once more.
- `clear_d1_and_branch`: Clears the long register d1 and branches to a different location in the library.
- `evaluate_conditional_math_sequence`: Loads a base address from a global variable and performs a series of conditional calls to various library functions at negative offsets from that address.
- `execute_library_dispatch`: Library code that moves d6 to d4, calls a routine at a negative offset from a5, and if the result is not negative, calls another routine at a different negative offset from a5 before calling a third routine via a branch to another routine.
- `execute_library_calls`: Library code that checks d6 and d4, then calls various routines at negative offsets from a5 using values in d1 and d2, and branches to another location.
- `call_library_functions`: Library code that moves d7 to d1, calls a routine at an offset from a5, and if the result is not negative, performs a series of calls to routines at offsets from a5 using a constant value.
- `call_and_store_result`: This library code moves a long value from D2 to D0, calls a routine at an offset from A5, moves the result back to D2, and branches to another routine.
- `test_and_load_constant`: This library code moves a long value from D2 to D0, tests a long value in D4, and if it is zero, loads a constant into D2 before branching to another routine; otherwise, it branches to another routine.
- `mask_return_value`: The library code calls a function through a pointer stored in a global variable twice, then calls another routine and returns its result with certain bits cleared.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `process_byte_transformation`: Library code that saves registers and a byte from d7 to the stack, performs bitwise operations and comparisons, and if certain conditions are met, calls a routine via a pointer in a register and performs arithmetic on d7.
- `prepare_bit_shift_loop`: Performs a logical shift right on a 32-bit value in d7, clears d6, sets d5 to a constant, loads a constant into a0, and initializes d1 and d2 with small constants before branching to another routine.
- `execute_loop_with_branching`: The library code executes a loop that performs bit-shifting and decrementing, potentially calling other routines if a counter reaches a certain value.
- `push_word_and_branch`: This library code pushes the word negative 64 onto the stack and then branches to an offset.
- `push_negative_one_and_branch`: Pushes a 16-bit value of negative one onto the stack and branches to another routine.
- `push_word_and_branch_2`: This library code pushes the word 1 onto the stack and then branches to an offset.
- `check_byte_and_adjust_stack`: Compares a byte in d7 to 184; if it is higher, it branches to another routine, otherwise it adds 2 to the stack pointer and returns.
- `adjust_byte_value`: Clears a word on the stack, then checks the signedness and value of d7; if d7 is between 56 and 167 inclusive, it saves d7 to d2, doubles d7, and calls another routine if the new value exceeds 138, otherwise it sets the carry flag and returns.
- `normalize_and_scale_fixed_point`: Normalizes a value in d2 by performing bitwise shifts and arithmetic based on its byte-level magnitude, then scales the result using a loop that processes values from a global table.
- `calculate_arithmetic_offset`: This library code performs arithmetic on d2, compares it to 31, and if the value is greater than 31, it calculates a new value for d7 by shifting d2 and performing subtractions before branching.
- `process_accumulated_values_2`: Initializes d5, d6, and d4 with specific constants, shifts d7 left by 3, and then enters a loop that performs arithmetic on d3, d4, d5, d6, and d7 using values from a table in a0 for 23 iterations.
- `process_accumulated_values`: Library code that adds d4 to d5, subtracts d3 from d6, and adds values from a pointer in a0 to d7 within a loop controlled by d1, then performs conditional calls to other routines.
- `set_d7_and_branch`: Loads the constant an address into d7 and branches to another routine.
- `recursive_call_wrapper`: Library code that returns immediately if a flag is zero, otherwise it calls another routine, moves the result to d7, and restores registers from the stack before returning.
- `push_and_call_routine`: Library code that pushes d5 onto the stack, calls a routine via a branch to another routine, loads a value from an offset of 24 from the stack into d6, and then branches to another location.
- `compare_and_branch_d6`: The library code moves d6 to d4, branches if d4 is negative, otherwise compares d6 to a constant and branches if it is higher, or clears d6 before returning.
- `shift_and_jump`: Arithmetically shifts d4 right by 8 bits, increments d4 by 1, and if d4 is zero, branches to another routine; otherwise it performs an absolute jump.
- `return_without_effect`: Library code that calls a routine and then, if the overflow flag is not set, calls it again and then calls a third routine if the overflow flag remains clear.
- `call_other_routine_conditionally`: Library code invokes another routine and then, based on a boolean flag that remains false, invokes three additional routines in sequence.
- `process_byte_and_branch`: Library code that tests a byte in d7, increments it, and if no overflow occurs, calls a sequence of routines using values from the stack and registers.
- `rotate_and_branch`: Loads a constant into d7, rotates d7 right by 1 bit, and branches to another routine.
- `load_constant_to_d7`: As library code, it loads a constant value into register d7 and branches to another routine.
- `test_and_branch_on_d6`: As library code, it pops a word from the stack into d6, tests the byte value in d6, and branches to another routine if the value is positive; otherwise, it clears d7 and branches to another routine.
- `set_negative_flag_and_restore_registers`: Sets the negative flag in the status register and restores the previously saved registers from the stack before returning.
- `load_constant_and_return`: Loads a constant value into a data register, adjusts the stack pointer by eighteen bytes, and returns.
- `normalize_fixed_point_value_2`: Library code that saves registers and a word from d7 to the stack, performs bitwise operations on d7, and calls several routines using a table pointed to by a0 and a library vector.
- `process_fixed_point_normalization`: As library code, it calculates a shift count based on a byte in d2, performs three calls to another routine using a table pointed to by a0, and adds a signed value from d5 to a constant in d6 before calling another routine.
- `subtract_and_loop_table`: The library code increments d2, shifts d3 and d4 right by the value in d2, and then loops using d1 to subtract values from a table pointed to by a0 and add them to d5 and d6.
- `subtract_table_from_accumulator`: As library code, it subtracts d4 from d5, subtracts d3 from d6, adds a value from a table pointed to by a0 to d7, and loops using d1 as a counter until it reaches a sentinel value before returning.
- `test_and_set_flag`: Library code that tests the byte in d7, branches if it is zero or negative, and if positive, performs a bitwise AND on d7 and calls a routine via a branch to another routine.
- `load_byte_to_d7`: As library code, it loads the value 255 into register d7 and branches to another routine.
- `execute_complex_math_sequence`: As library code, it saves registers to the stack, calls three different routines in sequence, and then enters a loop that performs bit-shifting and subtractions based on values in d7 and d6.
- `complex_loop_with_branching`: The library code executes a loop involving bit-shifting and conditional jumps, potentially calling other routines and performing arithmetic on a value.
- `call_other_routine_thrice`: Library code that calls another routine three times in succession and then returns.
- `test_d7_and_call`: Tests the byte in d7; if it is positive, it branches to a different location, otherwise it masks d7 with 127, calls another routine, and sets bit 1 of the status register.
- `execute_wrapped_routine`: Library code that calls another routine, saves registers d3 through d5 to the stack, calls a third routine, restores the registers, and jumps to a fourth routine.
- `transform_and_rotate_word`: Modifies a 32-bit value in a register by performing bitwise operations, a subtraction, a byte swap, and a 7-bit left rotation before returning.
- `transform_byte_bits`: Library code that swaps the high and low bytes of d7, performs a rotate through the carry, performs an exclusive OR on a byte in d7, and executes conditional branches based on the overflow flag.
- `test_byte_range`: As library code, it performs conditional branches based on the value in d7, including a comparison to 124 and 126, and may add 133 to d7 and perform a rotate left before testing the result.
- `mask_and_test_d7`: As library code, it performs a bitwise AND on d7 with a mask, tests the result, and branches to another routine if the result is zero; otherwise, it clears d7 and branches to another routine.
- `compare_and_branch_d7`: The library code compares d7 to a constant, branches if not equal, and then performs two logical shifts on d7 before branching based on the result.
- `shift_and_set_cc`: As library code, it shifts d7 left by 8 bits, loads 255 into d7, performs a rotate through carry on d7, sets the negative flag in the status register, and branches to another routine.
- `clear_d7_and_branch`: As library code, it clears register d7 and branches to another routine.
- `invoke_callback_and_return`: The library code receives a pointer in A0 and a value in D0, calls the routine pointed to by A0, and returns the value originally in D0 unchanged.
- `call_indirect_return_d0`: The library code receives a function pointer in A0 and a value in D0, calls the routine pointed to, and then returns the value that was in D0 on entry.
- `load_address_and_branch_3`: Library code that loads a long value into a0 and branches to another routine.
- `load_address_and_branch_4`: This library code loads a longword into a0 and then branches to a different location.
- `load_a0_and_branch`: Loads a long value from an offset of 444 into a0 and branches to the start of the routine.
- `load_null_pointer`: This library code loads zero into A0 and branches to another routine.
- `load_address_and_branch_5`: This library code loads a 32-bit address from a location into a0 and then branches to the next instruction.
- `load_pointer_and_branch_3`: Loads a pointer from a location in memory into a register and then branches to another routine.
- `load_callback_address_and_branch`: Loads the address of a routine from a global variable into a0 and branches to another routine.
- `store_register_via_pointer`: This library code loads an address into a0, saves several registers to the stack, calls the routine at the address in a0, stores the value from d7 into the memory location pointed to by a0, restores the registers, and returns.
- `load_a0_and_branch_2`: Loads a long value from an offset of 52 into a0 and branches to the start of the routine.
- `load_a0_and_branch_3`: Loads a long value from an offset of 0 into a0 and branches to the start of the routine.
- `load_a0_and_branch_4`: Loads a long value from an offset of 110 into a0 and branches to the start of the routine.
- `load_pointer_and_branch_2`: Library code that loads a constant address into a0 and branches to the next instruction.
- `load_null_pointer_and_branch`: Library code that loads zero into a0 and branches to the next instruction.
- `load_a0_and_branch_5`: Loads a long value from an offset of 204 into a0 and branches to the start of the routine.
- `load_null_and_branch`: This library code loads zero into a0 and then branches to a different location.
- `load_callback_pointer`: Loads a pointer from a data structure into register a0 and then branches to that address.
- `load_callback_address`: Loads the address of another routine from a location at offset 20 into a0 and then branches to that address.
- `load_offset_for_call`: This library code loads a constant into A0 and branches to another routine.
- `dispatch_indirect_routine`: Loads a pointer from a global variable into a register, adds a negative offset to it, and calls the resulting address.
- `load_address_and_branch`: Loads the address stored in a global variable into a6 and branches to another routine.
- `load_pointer_and_branch`: Loads a long value from a specific memory location into a0 and branches to another routine.
- `indirectly_call_routine_via_offset`: Loads a pointer into a register and then branches to another routine using an offset applied to a global variable.
- `dispatch_indirect_routine_2`: Loads a pointer from a global variable into a register and calls another routine at an offset from that pointer.
- `load_address_and_branch_2`: Loads a longword from a global variable into a register and branches to another routine.
- `loads_address_and_branches`: Loads the address of a global variable into a6 and performs a short branch to another routine.
- `load_address_and_branch_short`: Loads a longword from a global variable into a0 and performs a short branch to another routine.
- `invoke_callback_and_return_concat`: The library code invokes another routine via a pointer stored in a global variable and then returns the concatenation of the two input values.
- `call_indirect_with_offset`: The library code calls a routine at an address calculated by adding an offset in A0 to a pointer stored in a global variable, then returns the value in D0.
- `normalize_fixed_point_value`: The library code normalizes a signed 32-bit fixed-point value in D6 by swapping its high and low words, adjusting a shift count based on sign and magnitude, then left-shifting the value while decrementing the count until the count reaches zero or the value overflows, returning early on overflow.

### icon.library

- `return_negative_one`: The library code loads negative one into D0 and returns.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `update_library_refcount`: As library code, it checks a word at offset 32 of the structure in a6; if it is not zero, it sets bit 3 of a byte at offset 14 of the structure and returns zero; otherwise, it manipulates a linked list by updating pointers and calls another routine before returning a value from the stack.
- `set_bit_in_structure`: The library code sets bit 3 of a byte at offset 14 of a structure pointed to by A6 and returns zero in D0.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_library_call_and_handle_error`: The library code stores two values from a structure into another structure, calls a routine via a pointer, and if that routine returns zero, calls another routine via a pointer in a global variable.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `perform_bitwise_or`: The library code performs a bitwise OR on D0 with zero.
- `manage_icon_chain`: This library routine calls another routine to manage an icon chain.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `invoke_icon_structure_update`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `manage_icon_chain_2`: This library routine calls another routine to manage an icon chain.
- `recurse_icon_chain`: Saves two registers to the stack and calls another routine with the same first argument and an incremented second argument, then returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_icon_structure_update_2`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_subroutine_and_restore`: This library code saves a0 and a6 to the stack, calls a subroutine, restores the registers from the stack, and returns.
- `call_incremental_recursion`: Saves two registers to the stack and calls a routine that performs incremental recursion.
- `call_other_routine`: This library code calls another routine and then returns.
- `call_library_subroutine`: Saves two registers to the stack, calls another routine, and then returns.
- `call_other_routine_3`: Saves two registers to the stack, calls another routine, restores the registers, and returns.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `perform_bitwise_or_2`: Performs a bitwise OR operation on register d0 with zero.
- `process_icon_structure`: Sets up a stack frame, saves registers, and retrieves two values from a structure in a6; it calls another routine using the first value, and if the result is non-zero, it calls a second routine using the first and second values, then potentially calls a third routine if the second result is non-zero.
- `copy_and_assign_structure_fields`: Library code that copies 11 words from a source location to a destination location using a loop, then copies several individual bytes and words from the stack to a structure pointed to by a2, and finally updates a pointer at offset 424 of a structure.
- `return_zero_stub`: Clears the d0 register and restores the stack frame and registers from the library context before returning zero.
- `copy_icon_data_to_stack`: This library routine copies ten longwords from an offset within a structure pointed to by the third argument into a local array on the stack, then copies several additional fields from the same structure into other stack slots and finally calls another routine with the first two arguments and a pointer to a stack-resident header.
- `validate_icon_structure`: Validates an icon structure by checking specific magic numbers and flags, then performs a series of conditional calls to populate or update internal pointers and data members within the structure.
- `update_icon_gadget_state_2`: Checks if a specific icon gadget type is being processed, then performs a series of conditional calls to update gadget attributes, including populating a list of child elements and updating pointer fields within the gadget structure based on the results of those calls.
- `dispatch_callback_chain`: Library code that pushes several registers and values from a structure onto the stack, calls a routine via a pointer, and if the result is not negative one, performs a sequence of calls to a routine via a pointer and a library routine.
- `process_callback_list`: The library code executes a loop that calls a routine via a pointer in a3, stores the result in a location pointed to by a4, and decrements a counter in a6.
- `clear_d4_and_branch`: Library code that clears register d4 and branches to another location.
- `restore_registers_and_return`: Moves a longword from a register to the return value register, restores several data and address registers from a stack offset, unlocks the base pointer register, and returns.
- `update_icon_gadget_data`: Checks a status value via a library function and, if successful, updates various fields within an icon gadget structure, including potential pointer-based updates for lists and nested structures.
- `update_icon_gadget_state`: Checks the icon flags and gadget state, clearing a field at offset 26 if specific bits are set, then conditionally calls a function via a pointer to update various gadget-related properties and lists stored at offsets 34, 38, 54, and 70 of the structure.
- `iterate_icon_table_2`: Library code that pushes values from the stack, calls a routine via a pointer, and if the result is not negative, performs a sequence of calls to a routine via a pointer and a library routine.
- `iterate_icon_table`: This library code loops through a null-terminated table of pointers, increments a value at an offset in a stack frame, and calls a routine in a0 for each entry.
- `iterate_icon_table_entries`: Library code that walks a null-terminated table of pointers, calling a routine for each entry, updating a counter, and invoking a callback while potentially processing a secondary entry.
- `transfer_pointer_to_stack`: This library code pushes the value from a2 onto the stack, moves a pointer from the stack into a0, and then updates a2 with a value offset from that pointer before branching.
- `iterate_linked_list_nodes`: The routine traverses a linked list structure by following pointers stored at an offset from the current node.
- `increment_param_and_recurse`: Library code that calls another routine with the same first argument and the second argument increased by a constant, then returns.
- `process_icon_data`: As library code, it calls a library function to obtain a pointer, then if the result is non-zero, it calls another routine with an offset of 142 from that pointer, and subsequently calls another routine with an offset of 140 from that pointer using a value from d2.
- `call_subroutine_with_pointer_argument`: As library code, it pushes the address in a8, the value in a2, and an offset of 140 from a2 onto the stack, calls another routine, and if the result in d0 is zero, it returns; otherwise, it moves the value from a2 into d0 and branches to another location.
- `clear_d0_and_restore`: This library code clears d0 and restores d2, d3, and a2 from the stack before returning.
- `decrement_and_update_structure`: The library code decrements a word at a pointer in a2, and if the result is not greater than zero, it calls another routine and updates two longwords in a structure.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, then uses the counter value as an index into a table pointed to by a global variable.
- `initialize_icon_data`: Saves registers, retrieves two values from the stack, and calls another routine; if the result is non-zero, it calculates a new address in a2, calls a routine with a pointer to a2, and then performs a series of conditional calls to other routines based on the returned values.
- `branch_to_internal_offset`: This library code performs a short branch to another location within the same library.
- `call_library_routine_and_return_zero`: Library code that calls another routine and then returns zero.
- `perform_incremental_recursion`: Library code that performs a recursive call with an incremented parameter until the callee stops the recursion.
- `invoke_library_function`: As library code, it saves registers to the stack, loads a pointer from an offset of 4 in the stack frame into a2, loads a longword from an offset of 12 in the stack frame into d2, pushes d2 onto the stack, calls a routine, moves the result into d3, moves a2 into d4, and branches based on the zero flag or always.
- `search_for_character_in_table`: Saves three registers to the stack, calls another routine, and if the result is non-zero, increments a pointer in a3 and compares the byte at that location to the decimal value 61; if it matches, it returns the updated pointer in d0, otherwise it returns.
- `test_pointer_and_increment`: Loads a pointer from the address in a2 into a3, then loads the longword at that address into d0 and branches if d0 is non-zero; otherwise it clears d0 and restores registers before returning.
- `invoke_library_function_2`: As library code, it saves registers to the stack, loads two longwords from the stack into registers, pushes a third longword onto the stack, calls another routine, moves a longword from register d0 to d5, adjusts the stack pointer, and branches to another location.
- `calculate_register_difference`: Library code that pushes a longword and d2 onto the stack, calls a routine, and if the result in d0 is zero, calculates the difference between d0 and d2 before branching to another location.
- `verify_and_call_library`: As library code, it saves d2 to the stack, calls another routine, and if d0 matches d5, it saves d5, d3, and d2 to the stack to call another routine before returning 1 in d0.
- `increment_d2_conditionally`: Library code that checks if register d4 is zero, otherwise it copies d4 to d2 and increments d2 by one before branching.
- `test_and_clear_d2`: Clears register d2 and tests it; if non-zero, it branches to another routine, otherwise it clears register d0 and restores registers d2 through d5 from the stack before returning.
- `parse_numeric_string_and_update_structure`: Clears a byte at offset 30 of a structure in a2, then calls another routine to parse a numeric string from a buffer in a3; if parsing succeeds, it calculates a value based on the digits and calls another routine to update the structure in a2 with the result and a pointer to the remaining string.
- `call_external_routine`: As library code, it increments a4 by 5, pushes two pointers onto the stack, pushes a4 onto the stack, calls a routine, tests d0, adjusts the stack pointer, and then either sets d3 to 1 and branches or branches to another location.
- `clear_register_d3`: As library code, it sets register d3 to zero and branches to another location.
- `iterate_and_invoke_table`: As library code, it calculates a value in d3 using d0, d1, and d2, enters a loop that calls another routine, and if a condition is met, performs two additional calls to other routines using values from the stack.
- `calculate_d0_range_offset`: Library code that compares a value from the stack to constants, and either sets d0 to a specific constant or calculates the difference between d0 and d1.
- `set_d0_to_negative_one`: This library code sets d0 to negative one and returns.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: The library routine forwards a set of four parameters to another routine, checks the returned value, and if non-zero calls the same routine again with a modified argument list.
- `initialize_icon_structure`: As library code, it saves d2 and a2, calls another routine, and if the result is not zero, it writes the value 10 to offsets 14 and 16 of a structure pointed to by the returned address, calls another routine, and returns 1; otherwise, it returns 0.
- `return_zero_with_restore`: This library routine clears register d0 and restores registers d2, a2, and the stack pointer before returning.
- `process_library_call`: This library routine loads three values from the stack, pushes two of them onto the stack, calls another routine, and then either sets d0 to the result or clears d0 based on a comparison with a value at an offset of the stack pointer.
- `tiny_move_rts`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `call_library_function_with_args`: As library code, it loads three values from the stack into d0, d1, and the stack pointer, pushes d1 and d0 onto the stack, calls another routine, compares the result in d0 to a value on the stack, and either branches to another routine or returns zero.
- `copy_d1_to_d0`: This library routine copies the contents of register d1 into register d0 and returns.
- `perform_division_math`: This library code checks if d2 is greater than 65535; otherwise, it performs several division operations using d2 and values derived from d1 and a1, then returns.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `compare_byte_and_decrement_pointer`: Loads a pointer from offset 4 of the stack and a byte value from offset 8 of the stack.
- `branch_to_offset`: This library code clears d0 and then branches to another location within the library.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `loop_through_bytes`: This library code loops through a series of bytes pointed to by a0, decrementing a counter in d0, and branches back to the start of the loop if a byte is non-zero.
- `compare_byte_in_table`: The library code decrements d0, and if it is not less than zero, it compares a byte from a pointer in a1 with a byte from a pointer in a0, returning zero if the bytes match and d1 is zero.
- `set_d0_signed_constant`: Library code that sets register d0 to 1 if a condition is met, otherwise it sets d0 to 255 and returns.
- `test_and_decrement_pointer`: Library code that tests a byte at the address in a0 and increments a0, then decrements d0 if the byte is not zero and branches based on the value of d0.
- `decrement_a0_and_call`: Library code that decrements the long value in register a0 and calls another routine.
- `copy_bytes_with_counter`: This library code loops to copy bytes from a1 to a0 while decrementing d0, and if d0 reaches zero, it performs a second loop to clear bytes at a0 until d0 is zero or a byte is zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `process_icon_request`: Library code that validates a parameter, calls another routine to check a return value, and if successful, performs further calls using a value from a pointer.
- `load_structure_pointers`: As library code, it saves registers to the stack and loads three longwords from offsets 16, 20, and 24 of the stack into d2, d3, and a2 before branching.
- `traverse_linked_list`: The library code calls another routine, and if the result is non-zero, it performs arithmetic on a byte from a structure and calls another routine again, looping while a pointer in a2 is non-zero.
- `return_zero_with_extended_restore`: This library routine clears register d0 and restores registers d2, d3, a2, and the stack pointer before returning.
- `execute_icon_callback_sequence`: Library code that performs a series of conditional calls to other routines using parameters from the stack, including calculations involving a structure pointed to by a global variable and updating a field at offset 10 of that structure.
- `return_negative_one_with_extended_restore`: This library routine sets register d0 to negative one and restores registers d2, d3, d4, d5, a2, and the stack pointer before returning.
- `load_stack_arguments_and_branch`: Saves registers d2 through d6 and a2 to the stack, loads three longwords from offsets 28, 32, and 40 of the stack into d2, d3, and d4, clears d6, and branches to another routine.
- `invoke_callback_with_stack_setup`: Library code that pushes several values onto the stack, calls another routine, and then conditionally updates register d6 from a value at offset 28 of the stack.
- `invoke_callback_chain`: The library code performs multiple calls to other routines, using values from the stack and a structure in a2 to update a field and conditionally branch.
- `return_negative_one_with_large_restore`: This library routine sets register d0 to negative one and restores registers d2, d3, d4, d5, d6, a2, and the stack pointer before returning.

### mathieeedoubbas.library

- `return_constant_2`: Returns the constant -1 in d0 to the caller without any other effect.
- `initialize_ieee_operation_structure`: This library code retrieves a structure via a library call, stores two routine addresses and the caller's arguments into it, and then calls another routine if a specific bit is set in a value at offset 474 of a6 and a value at offset 15 of the retrieved structure is non-zero.
- `init_ieee_operation_struct`: As library code, it calls a library vector to obtain a structure, stores two routine addresses and the caller's arguments into it, and sets a flag byte from the caller's flag bit.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `raise_ieee_exception_code`: Stores two into a 16-bit field of the structure at A0, then, depending on the low byte of the exception code in D7, deliberately divides by zero or raises the overflow trap.
- `compare_and_branch_d7_3`: As library code, it compares a 16-bit value in register d7 to 7680 and branches to another location if the value is greater than or equal to 7680, otherwise it returns.
- `check_ieee_exception`: As library code, it compares a 16-bit value in d7 to 7680 and branches based on the result, otherwise it branches to another location.
- `compare_and_branch_d7_2`: Library code that compares a 16-bit value in d7 to 23552 and branches if it is greater than or equal to that value, otherwise it returns.
- `branch_on_exception_code`: Library code that compares a 16-bit value in register d7 to a constant and branches accordingly.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `test_and_call_routine`: As library code, it pushes d7 onto the stack, loads a pointer from an offset of 36 in a6 into a0, loads a word from an offset of 16 in a0 into d7, compares d7 to a constant, and if d7 is not equal to that constant and is not negative, calls another routine; otherwise it branches.
- `process_math_data`: As library code, it compares a 16-bit value in d7 to 50944; if not equal, it stores a longword from d0 at the address in a1, tests a word at the address in a0, and if that word is not negative, writes 29744 to an offset of 10 from a0, updates d7 from that address, and either calls another routine or branches to another location based on further comparisons.
- `compare_and_load_data`: Library code that compares d7 to 50944, and if equal, loads two long values from the address in a1 into d0 and d1, otherwise it tests a word at the address in a0 and branches based on the sign bit.
- `clear_word_high_byte`: As library code, it pushes d7 to the stack, loads a pointer from an offset of 36 from a6 into a0, sets a1 to point to an offset of 16 from a0, writes zero to an offset of 6 from a0, and if the high byte of the word at that offset is zero, it returns; otherwise it returns.
- `check_word_and_call`: As library code, it tests a word at the address in a0, and if it is not negative, writes a word to an offset of a0, loads that word into d7, compares d7 to a constant, and if d7 is not equal to that constant and is not negative, calls another routine; otherwise it branches.
- `check_exception_and_store_byte`: Library code that compares a 16-bit value in register d7 to a constant, and if not equal, stores the value 144 at the address in a1 and performs further checks on a value at the address in a0.
- `normalize_and_compare_double_precision`: Extracts the mantissa and exponent from two double-precision floating-point values, performs bitwise manipulation to align their components, and compares the resulting values to determine their relative order or equality.
- `compare_and_branch_registers`: Library code that compares a word in d7 with d2, compares a longword in a0 with d2, and tests d3 before moving d2 and d3 into d0 and d1 and branching to another routine.
- `return_hardcoded_constant`: Library code that branches to another routine to return a hardcoded 64-bit constant.
- `test_and_xor_d0`: Library code that tests d2 and d3, and if both are non-zero, performs an XOR operation between d0 and d4, swaps d0, and restores registers from the stack before returning.
- `xor_shift_word_left`: Library code that branches to another routine to return a 64-bit value derived from XORing a 16-bit value with 32752 and shifting it left by 48 bits.
- `compare_values_and_branch`: Compares d2 with d7 and branches if d2 is less than d7, then compares d2 with the address in a0 and branches if they are not equal, otherwise it tests d3 and either moves d2 into d0 and d3 into d1 and branches or moves d2 into d0 and d1.
- `shift_word_left_48`: Library code that branches to another routine to return a 64-bit value derived from shifting a 16-bit value left by 48 bits.
- `perform_bitwise_logic_and_test`: Performs a bitwise AND between d7 and d5, where d5 is loaded from d0, and then performs a series of conditional branches based on the zero status of d0, d1, d2, and d3.
- `branch_to_shift_routine`: Performs a short branch to another routine that shifts a 16-bit value left by 48 bits to produce a 64-bit result.
- `call_subroutine_and_branch`: Calls a subroutine and then performs an unconditional short branch to the instruction following this routine.
- `perform_bitwise_logic`: This library code performs an XOR between d5 and d0, an XOR with a constant on d0, an AND between d2 and d7, and then performs several conditional branches based on the results of testing d2 and d3.
- `swap_register_groups`: Swaps the contents of d0 and d2, d1 and d3, and d5 and d7, calls another routine, and then swaps the same register pairs back before branching to another location.
- `perform_arithmetic_modifications`: Performs bitwise XOR and AND operations on d2, subtracts d7 from d5 after calculating d7 from d2, and branches to another routine if the carry flag is not set.
- `calculate_high_precision_division`: Performs a complex division operation by first invoking multiply_and_divide_64_bit to determine an initial quotient, then iteratively refines the result through multi-word multiplication and subtraction to handle precision requirements.
- `multiply_and_divide_64_bit`: Performs a 16-bit by 32-bit multiplication to produce a 64-bit result, then divides that result by a 16-bit divisor to calculate a quotient.
- `calculate_64_bit_product_and_divide`: Performs a 16-bit by 32-bit multiplication to produce a 64-bit result, then divides that result by a 16-bit divisor to calculate a quotient and remainder.
- `multiply_and_shift_64_bit`: Performs a 16-bit by 32-bit multiplication to calculate a product, then applies a variable bitwise shift and rotation to the result before returning a 64-bit value.
- `modify_and_branch_d5`: Library code that negates D5, shifts it right by 4, adds 2, compares it to 57, and branches to another routine if it is less than or equal to 57; otherwise it branches to another routine.
- `scale_double_register_pairs`: Fragment of double-precision arithmetic that shifts register pairs in a loop until a count reaches its end, with a call to another routine.
- `shift_and_test_d5`: Library code that performs a logical shift right on d0 and a rotate right through carry on d1, then tests d5 and branches based on its sign.
- `shift_and_xor_data`: Library code that tests bit 23 of d0, subtracts 16 from d5, performs logical shifts and rotates on d0 and d1, swaps d0, adds d5 to d0, compares d0 with a0, and optionally XORs d0 with d4 before returning.
- `build_fixed_constant`: Library code that branches to another routine to return a 64-bit constant pattern derived from XORing a 16-bit value with 32751, scaling it, and combining it with a mask.
- `count_leading_zero_bits`: Library code that shifts the 32-bit value in d0 left in 16-bit and then single-bit steps until its top bit is set, and records how many shifts were made, returning the adjusted values in registers.
- `process_bit_and_shift_pair`: Library code that performs bit tests on D4 and D3, manipulates D0 and D1 through various shifts, swaps, and XOR operations, and calls another routine to shift a register pair right by a count from D5.
- `shift_right_pair_by_count`: Shifts a register pair right by a count taken from the 16-bit value in D5 and returns the shifted pair, with the exponent word of the operand XORed into it.
- `shift_word_left`: Library code that shifts a 16-bit value left by 48 bits and returns it as a 64-bit result.
- `promote_half_to_upper`: Library code that takes the low 16 bits of a 32-bit value held in a register and places them into the upper half of a 64-bit result, leaving the lower half zero.
- `build_fixed_double_constant`: Library code that combines a 16-bit value with a constant mask, scales it, and returns a fixed 64-bit constant pattern.
- `shift_xor_word_left`: Library code that xors a 16-bit value with a constant, shifts the result left by 48 bits, and returns it as a 64-bit value.
- `return_hardcoded_64bit`: Library code that returns a single hardcoded 64-bit constant.
- `set_signaling_bit_return_pattern`: If bit three of D0 is set, clears it and sets bit one, then returns a 32-bit pattern formed from the value with a fixed mask OR-ed in and its two halves swapped.

### diskfont.library

- `jump_to_font_handler`: Calls a routine via a table entry using a library base.
- `check_and_set_font_handle`: This library routine calls a routine via a table entry using a library base.
- `unlink_linked_list_nodes`: Library code that manipulates a structure in d0 using a pointer in a6, calls routines via pointers at offsets from a6, and performs pointer redirection to unlink nodes from a list.
- `execute_library_calls_2`: Library code that loads a pointer from a table into a1, calls a routine via a pointer in a6, performs arithmetic on d0 using values from a structure in a6, and calls another routine via a pointer in a6.
- `unlink_font_node`: Retrieves a pointer from an offset in a6 and checks if it is null or if a word at an offset in that pointer is greater than zero; if the value is non-positive, it performs a linked-list removal by updating pointers, calls another routine, and decrements a word in a structure before branching.
- `update_font_linked_list`: This library code checks two values at offsets 56 and 64 of a6, and if both conditions are met, calls three different library routines before updating a linked list by moving a pointer from a structure into another structure.
- `set_bit_seven`: As part of library code, this routine sets bit 7 of a byte at an offset of 14 from the address in a6 and then clears d0.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `decrement_and_jump`: As part of library code, this routine decrements a 16-bit word at offset 38 of the structure in a6 and branches to offset 44 of the structure if the result is not zero; otherwise, it checks bit 7 of a byte at offset 14 of the structure and jumps to an offset relative to a6 if that bit is set.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `parse_font_metadata_stream`: Iterates through a linked list of font data structures, calculating string lengths and copying character data into a destination buffer.
- `initialize_string_counter`: Library code that loads an address from an offset of a4 into a1 and sets d4 to 1.
- `count_null_terminated_string`: Counts the number of bytes in a null-terminated string by iterating through memory and incrementing a register for each byte until a zero is encountered.
- `parse_font_data_stream_2`: Copies bytes from a source stream into a destination buffer while iterating through a data structure to populate metadata including a type, a pointer, and bitwise flags.
- `load_and_parse_font_data`: Opens a file specified by a pointer, reads a string, and if a specific condition is met, copies the string into a buffer and parses a sequence of font data structures to update a table with character information and offsets.
- `count_null_terminated_string_2`: Increments a counter in d4 while scanning a null-terminated string in a1, then calculates two offset values based on the string length and compares them against values in d2 and d5 before branching.
- `copy_font_data_from_disk`: Copies a sequence of bytes from a source buffer to a destination buffer using a loop, then calls a routine to read data from a disk and checks if the read operation was successful and if the data matches a specific constant value.
- `parse_font_data_stream`: Iterates through a data stream to identify specific markers, populating a structure with metadata including a type, a pointer, and bitwise flags.
- `return_zero_stub_2`: Clears the d0 register and restores several registers from the stack before returning.
- `calculate_font_offset_3`: Library code that checks a word at offset 256 of a structure in a1, then calculates a value in d2 by subtracting a word at offset 4 of a structure in a0 from that word, applying a bit shift and addition if the result is negative.
- `calculate_font_offset_2`: Library code that subtracts a value from d2, performs multiple byte XOR operations between offsets of a0 and a1, and uses the results to subtract values from a global variable using a table.
- `calculate_font_offset`: Library code that subtracts a longword from a table at an offset determined by shifting a 16-bit value left by two bits, and performs bitwise checks against values in a structure pointed to by a1 before branching.
- `update_font_index_and_offset`: Shifts a 16-bit value left by two bits, subtracts a value from a table at an offset determined by the shifted result into a 32-bit register, increments a 16-bit counter, and loops if the counter is less than seven.
- `return_constant_value`: This library routine returns a constant value without using any arguments or registers.
- `update_font_glyph_metadata_2`: Retrieves a font resource, iterates through a list of glyphs to find the one with the highest value, and updates its metadata, including a counter and a status bit, within a structure.
- `call_routine_and_store`: This library code pushes a value onto the stack, calls a subroutine, stores the result in a location relative to a6, restores the stack, and then branches to another location or performs a branch if a condition is met.
- `update_font_glyph_metadata`: Searches through a font resource to find a glyph with a specific property, then updates its metadata, increments a counter in a global structure, and sets a bit in a status byte at offset 85 of the glyph structure.
- `clear_font_registers`: Clears the word in d4, sets d2 to 255, clears the word in d3, and branches to another location.
- `search_and_update_font_resource`: Iterates through a range of font indices to find the largest available size, then performs a series of updates on a font structure including setting a bit in a byte at offset 77, incrementing a word at offset 84, and updating values at offsets 42 and 40 of a structure pointed to by a global variable.
- `unload_and_clear_word`: Library code that calls another routine, calculates a new address by multiplying a 16-bit value in d2 by 260 and adding d5, clears a word at offset 256 of that address, and then branches.
- `manage_font_resource`: Calculates an offset into a font data structure using a multiplier, clears a word at a specific offset, and performs a series of cleanup operations including unlocking, freeing memory, and closing files or fonts based on the input parameters.
- `parse_disk_font_data`: Iterates through a list of font entries to build a font table, calculating a total size and allocating memory for a new structure.
- `load_disk_font_table`: As library code, this routine builds a table for a disk font.
- `dispatch_font_request`: As part of library code, this routine reads a pointer and an argument from the stack and passes them to another routine.
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
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `call_parent_dir`: Library stub: loads its arguments and the dos library base, calls ParentDir and returns its result to the caller.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `load_parameters_and_branch`: Loads a pointer from offset 4 of the stack into a0, loads a longword from offset 8 of the stack into d0, and performs a short branch to another location.
- `skip_to_string_terminator_and_call`: The routine increments a pointer until it reaches the terminating zero byte of a zero-terminated string and then calls another routine with the pointer to that terminator.
- `initialize_counter_and_branch`: As part of library code, this routine loads a pointer from offset 4 of the stack into a0, clears d0, and branches to an offset relative to the current instruction.
- `count_null_terminated_bytes`: As part of library code, this routine increments a counter while advancing a pointer and reading bytes until a zero byte is encountered, returning the final count.
- `compare_strings_lexicographically`: Compares two null-terminated strings byte by byte, returning a value based on their lexicographical order, and branches to another routine.
- `skip_zero_byte`: Checks if the byte pointed to by a0 is zero, and if it is not, branches to another routine; otherwise it sets d0 to 1 and branches to another routine.
- `compare_bytes_and_branch`: Library code that compares a byte from a0 with a byte from a1, and if they are equal, compares a byte from a0 with a byte from a1 and branches based on the result.
- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.
- `load_parameters_and_branch_2`: Library code that loads a pointer from the stack into a0, loads a byte from the stack into d0, and branches to another routine.
- `compare_and_load_pointer`: Compares d1 with d0, and if they are not equal, branches to another routine; otherwise it moves the value in a0 to d0 and branches to another routine.
- `increment_and_check_byte`: Library code that increments the address in a0, compares the byte at the new address with d1, and if it is non-zero, sets d0 to zero and returns; otherwise it returns.
- `call_and_adjust_pointer`: Library code that saves registers to the stack, loads d3 from the stack, loads d2 from the stack, calls a routine via a pointer, and then adjusts a0 by adding d3 to it before branching to another routine.
- `compare_byte_and_load_pointer`: Compares the byte at the address in a0 with d2, and if they are not equal, branches to another routine; otherwise it moves the value in a0 to d0 and branches to another routine.
- `check_address_range`: Library code that decrements the address in a0, compares it with d3, and if it is less than d3, sets d0 to zero and restores registers from the stack before returning.
- `try_font_values`: Library code that pushes a value onto the stack and calls another routine, then repeats this with a different value if the first call does not result in a non-zero return value.
- `increment_returned_value`: This library routine returns the value passed in D0 incremented by one.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.
- `call_other_routine_2`: As part of library code, this routine saves registers a0 and a1 to the stack, calls another routine, restores the registers, and returns.
- `call_another_routine`: This library routine is library code that acts as a thin wrapper which immediately invokes another routine and then returns without performing any additional operations.

### Ace the Space Case (1994)(Grand Master Design - Quartz PD)(PD).adf:.

- `parse_amiga_hunk_table`: Iterates through a table of data structures, identifying specific hunk type codes to perform memory allocation, data relocation, and pointer updates.
- `parse_and_store_address`: Library code that decrements d1 and branches if it is negative, performs arithmetic on a pointer in a1, calls a routine using a negative offset from a6, and stores a shifted value from a0 into a memory location pointed to by a4.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `process_bitstream_data`: Loads a long from the address in a0, and if the byte at that address is non-zero, calls another routine, performs arithmetic on d1, and uses a loop to copy bytes from a source to a destination pointed to by a3.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `extract_bitstream_data`: Performs bitwise extraction from a data stream by shifting and rotating values based on a bit count, then updates a hardware register and a pointer in a table.
- `execute_trap_six`: This routine moves a word from an offset of 3085 from a2 into d4, executes a trap, and performs a logical OR of d2 into d0.

### Ace the Space Case (1994)(Grand Master Design - Quartz PD)(PD).adf:ACEDEMO!

- `parse_amiga_hunk_table_2`: Parses a table of data by identifying specific hunk type codes, performing memory allocations, and applying relocations to pointers.
- `decrement_and_loop_counter`: Decrements a 16-bit value in a register and loops until the value reaches zero, otherwise it returns.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `unpack_compressed_bitstream`: Reads a length value from a pointer in a0, calculates a data offset by shifting and adding, and then enters a loop to extract and write bytes to a destination buffer in a3 based on bitwise parameters.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `subtract_word`: This routine subtracts a 16-bit word in d4 from a 16-bit word in d0.
- `extract_bits_to_buffer`: Uses a loop to extract bits from a source pointed to by a0 into a destination pointed to by a3, while updating a hardware register with a value derived from a3.
- `fetch_and_execute_trap`: Loads a 16-bit word from an offset of 1741 from a pointer in a2 into d4, calls a system trap, and performs a logical OR between the result in d2 and d0.

### Ace the Space Case (1994)(Grand Master Design - Quartz PD)(PD).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
