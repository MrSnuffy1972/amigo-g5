# Game 352 (Ace the Space-Case): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00352). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 77 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 344 of 344 named (0 author symbols), 344 of 344 described (51 by rule, 253 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 83 of 83 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4263 of 4263 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 353588 |
| code | 12838 |
| data | 148953 |
| padding | 16077 |
| unused | 369664 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| FastMemFirst | 7 | 207 | 6 |
| mathtrans.library | 17 | 830 | 97 |
| icon.library | 10 | 1375 | 109 |
| mathieeedoubbas.library | 3 | 568 | 47 |
| diskfont.library | 15 | 1040 | 76 |
| Ace the Space-Case.adf:ACEDEMO! | 2 | 233 | 8 |
| Ace the Space-Case.adf:bootblock | 1 | 10 | 1 |

## Cases handled

- Ace the Space-Case/Ace the Space-Case.adf:ACEDEMO!: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### FastMemFirst

- `allocate_fast_memory_and_parse_args`: Allocates a block of fast memory and stores the requested size, a pointer to the block, and the address of the caller's stack frame into it, then calls a routine that parses the command line from dos.library into a linked list of null-terminated argument strings, handling quoted text and asterisk conversions.
- `find_and_copy_fastmem_block`: Walks a linked list of memory blocks starting from a global variable, storing the first block whose type tag at offset 14 equals 5 and whose value is between an address and an address in d2, and the next block after the first whose type tag equals 3 and whose value is below an address in d3, then returns.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.

### mathtrans.library

- `emit_illegal_instruction`: Contains a single illegal instruction and does nothing else, so it never executes normally.
- `store_a0_in_global`: Stores the value passed in a0 into a global variable at offset 198, then returns.
- `unlink_node_if_zero`: Checks a 16-bit value at offset 32 of the structure in a6 and, if it is zero, unlinks a node from a doubly linked list by rewriting the surrounding pointers before returning.
- `set_bit3_in_struct`: Sets bit 3 of the byte at offset 14 in the structure in a6, loads zero into d0, and returns.
- `init_fpu_if_needed`: If bit 3 of the byte at offset 14 in the structure in a6 is clear, increments the word at offset 32, and when that word first becomes 1 it stores a global variable at offset 190 and calls another routine storing its result at offset 194, then returns the structure pointer.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `decrement_counter_and_call`: Library code decrements a 16-bit counter at offset 32 of the structure in a6 and returns unless the counter reaches zero and bit 3 of the flag byte at offset 14 is set, in which case it calls another routine.
- `mask_and_forward_byte`: Library code clears the sign bit of d0 and calls a helper routine, then sets bit 1 of the condition code register and returns, so it masks the byte and forwards it to the helper before signaling a carry.
- `adjust_bitfield_value_2`: Returns immediately when the byte in d0 is zero, otherwise branches to a helper routine when it is negative.
- `adjust_bitfield_value`: Sets the bit selected by d1 in d0, then loops doubling an accumulator and shifting a running difference until a termination condition is met, at which point it increments d0 under certain conditions and returns the modified value combined with the high word of d1.
- `test_and_call_vector`: Copies the value in d0 into d7 and tests it, then calls a routine via a vector at offset 20 of the library base in a5.
- `call_jump_table_entries`: Saves d6, then calls a series of function pointers at fixed offsets from the base in a5 in an alternating pattern, calling one entry once more than the others, restores d6, and returns.
- `clear_register_and_branch`: Clears the 32-bit value in register d1, then branches unconditionally into the following routine.
- `dispatch_math_trans_vectors`: Saves several registers, loads the library base into a5, and calls a series of function pointers at fixed offsets while tracking a flag in d6.
- `call_math_trans_routines`: Calls a library routine at a fixed offset of the base in a5, and when the result is negative it calls a second library routine at another offset, then calls a third library routine and returns the result with bits 2 and 3 cleared.
- `call_vector_functions`: Copies d5 into d2, returns early when d6 is nonzero, otherwise calls function pointers at offsets 66, 48, and 60 of the library base depending on the sign of d7 and the value of d4.
- `call_vector_and_mask_result`: Library code moves d7 into d1 and calls a routine via a vector at offset 60 of the library base, returning early when the result is negative.
- `call_vector_store_d2`: Moves the value from d2 into d0, calls a routine via a vector at offset 60 of the library base in a5, stores the result back into d2, and falls through.
- `load_constant_if_d4_set`: Moves d2 into d0 and tests d4, returning early when that value is nonzero.
- `mask_return_value`: Loads a constant into d2, calls a function pointer at offset 66 from the base in a5 twice, calls another routine, then returns the result with bits 2 and 4 cleared.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `scale_and_convert_value`: Saves registers and a byte, masks the sign bit off d7, and when the value is too large it stores a sentinel and calls a helper routine, otherwise it computes a scaled fixed-point value in a loop that shifts and subtracts until the remainder is exhausted.
- `shift_and_setup_accumulate`: Arithmetically shifts d7 right by d7, clears d6, loads a constant into d5, sets up a pointer and shift count, and jumps into the middle of another routine.
- `shift_and_accumulate_loop`: It runs a loop that arithmetically shifts d4 right by a count in d2, adds it to d5, and adds the value at a0 into d6, and it branches back into the loop while the shifted result is negative.
- `push_constant_branch`: Pushes the 16-bit constant 65534 onto the stack and then branches forward into the middle of another routine, so it only sets up the stack and hands off control.
- `push_maxword_branch`: Pushes the 16-bit constant 65535 onto the stack and then branches forward into the middle of another routine, so it only sets up the stack and hands off control.
- `push_one_branch`: Pushes the 16-bit constant 1 onto the stack and then branches forward into the middle of another routine, so it only sets up the stack and hands off control.
- `compare_byte_and_adjust_stack`: Compares the byte in d7 against the constant 184, and if the value is greater it adds 2 to the stack pointer; otherwise it tests the byte in d7 and returns.
- `test_byte_range_branch`: Tests the byte in d7 and returns immediately unless it falls within one of the ranges that pass the range checks.
- `accumulate_table_signed_2`: Loads a table address into a0 and accumulates signed table entries into two 32-bit accumulators by shifting a value based on a byte, then loops 23 times adding and subtracting table entries.
- `accumulate_table_signed`: It adjusts a byte in d2, then shifts and negates a value into d7 before comparing it against a second accumulator in d4.
- `accumulate_table_values`: Adjusts two 32-bit accumulator registers based on the sign of a shifted value, then loops 23 times accumulating entries from a table into a running sum while decrementing a counter.
- `accumulate_table_with_sign_branch`: Accumulates values from a table pointed to by a0 into d7 while updating running sums in d5 and d6, decrementing a counter in d1 each iteration.
- `store_constant_and_branch`: Stores the constant 127 into d7, then branches into another routine without returning.
- `recursive_call_wrapper`: Returns immediately when a byte is zero, otherwise calls another routine, saves d6 into d7, restores registers d1 through d6 and a0 from the stack, and returns.
- `push_call_and_branch`: Pushes d5 on the stack, calls a routine, stores d6 at offset 24 of the stack, restores d6, and branches to a later routine without returning.
- `clamp_d6_to_255`: It moves d6 into d4 and returns early when that value is negative, then compares d6 with 255 and returns early when it is greater, otherwise it clears d6 and returns, so it clamps d6 to a non-negative value of at most 255.
- `shift_and_check_d4`: Arithmetically shifts d4 right by 8, increments d4, and if the result is zero it branches away; otherwise it jumps to the address zero.
- `return_without_effect`: Clears the sign bit of the byte in d7 and calls a routine via a callback pointer.
- `call_other_routine_conditionally`: Clears the sign bit of the byte in d7 and calls a routine via a callback pointer.
- `increment_d7_then_call_callbacks`: Tests the byte in d7 and returns immediately when it is zero, otherwise increments it and calls a routine via a callback pointer.
- `store_constant_and_rotate`: Stores the constant an address into d7, rotates that byte right by one bit, then branches into another routine without returning.
- `load_accumulator_base`: Loads the constant an address into register d7 and then branches to a later routine to continue the calculation.
- `restore_registers_and_return_3`: Pops a word off the stack into register d6 and returns immediately when that value is negative, otherwise loads zero into register d7 and branches to the following routine.
- `restore_registers_and_return`: It clears the carry flag by shifting register d7 right by one bit, sets the overflow and negative condition-code bits, restores registers d1 through d6 and a0 from the stack, and then returns.
- `store_constant_and_return`: Stores the value an address into register d7, loads the address of a byte at offset 30 from the stack pointer into register a7, and returns.
- `convert_fixed_point_value`: Returns immediately when d7 is zero, otherwise masks the sign bit off d7 and calls a library routine, then shifts d7 by a count, stores a byte, and calls two more library routines before branching away.
- `normalize_fixed_point`: Clears a stack byte, adjusts and negates a shift count, shifts d7 right, loads a fixed-point constant into d6, calls another routine three times with different counts, and returns.
- `subtract_table_then_shift`: Shifts d5 and d6 right by the count in d2, adds them into d5 and d6, subtracts the next table entry pointed to by d7 from d7, and loops while d1 is nonzero.
- `subtract_table_from_accumulator`: Subtracts the contents of registers d4 and d3 from registers d5 and d6, then adds the next entry from the table pointed to by d7 while decrementing a counter, looping until the counter reaches zero.
- `test_and_normalize_d7`: Tests d7 and returns immediately when it is zero, otherwise clears its sign bit, calls a routine, sets the carry flag, and returns.
- `load_accumulator_max`: Loads the constant 255 into register d7 and then branches to a later routine to continue the calculation.
- `load_byte_and_call_routines`: Loads a byte from a global variable into d7, then calls three different routines in sequence, saving and restoring d7 around the calls.
- `shift_subtract_normalize`: Shifts and subtracts values in registers, loops while a shifted result is negative while decrementing a counter and calling another routine when it underflows, then normalizes the fixed-point value in d6.
- `call_library_vector_and_routines`: Loads a library base pointer from a global variable, calls the library function at offset 528, then calls two other routines in succession before returning.
- `test_sign_and_call`: Tests the sign bit of the byte in register d7; if it is positive it clears the sign bit and calls a routine, otherwise it sets the overflow condition flag and returns.
- `save_registers_then_call_routine`: Saves registers d3 through d5 on the stack, calls a routine, then restores those registers from the stack and branches into another routine without returning.
- `manipulate_d7_register`: It performs a series of arithmetic and bit manipulation operations on the value in d7, then returns.
- `rotate_and_check_d7`: Swaps the halves of d7, rotates it right by seven bits, toggles its top bit, then repeatedly adds and rotates d7 while checking the carry flag before returning.
- `compare_and_rotate_d7`: Compares the byte in d7 against 124 and 126, and when it equals 126 it rotates d7 right by one bit, then loops while d7 stays nonzero.
- `clear_low_byte_flag`: Clears bit 9 of register d7, then returns zero when the result is zero, otherwise clears register d7 and branches to the following routine.
- `shift_if_equal_254`: Compares d7 against 254, then if equal shifts d7 right by eight bits and once more, looping back while d7 stays nonzero.
- `shift_and_rotate_value`: Shifts register d7 left by eight bits, loads the constant 255 into it, rotates the carry flag into the low bit, sets bit 1 of the condition code register, and then branches to a later routine.
- `load_zero_register`: Loads the constant zero into register d7 and then branches to a later routine to continue the calculation.
- `invoke_callback_and_return`: Saves d3 through d7, copies the value in d0 into d7, calls the routine pointed to by a0, restores the saved registers, and returns the value originally in d0 unchanged.
- `call_indirect_return_d0`: Saves d3 through d7, copies d1 into d6 and d0 into d7, calls the routine pointed to by a0, restores the saved registers, and returns the value originally in d0 unchanged.
- `load_offset_and_branch_2`: Library code loads the address of a global variable into a0 and falls through to another routine reached by adding an offset to a pointer stored in a global variable, returning whatever that routine returns.
- `load_global_and_branch_2`: This library code loads the address of a global variable into a0 and branches into another routine, returning the value it produced.
- `load_address_and_branch`: Loads the address at data offset 28c into register a0, then branches unconditionally into the following routine.
- `call_global_vector_a0`: Loads the address of a global variable into a0 and falls through to the routine it points to, returning whatever value that routine produces.
- `invoke_global_callback`: Loads the address of a global variable into a0 and calls the routine pointed to by that address, then returns its result.
- `load_address_and_branch_6`: Loads the address of a global variable into a0 and then branches away without executing any further instructions.
- `invoke_mathtrans_handler`: Loads the address of a global variable into a0, then calls the routine pointed to by a0, passing the value in d0 and returning its result.
- `store_register_via_pointer`: Saves d3 through d7, copies d0 into d7, sets d1 to zero, calls the routine pointed to by a0, loads a pointer from the stack into a0, stores d7 through that pointer, restores d6 into d0, restores the saved registers, and returns.
- `load_address_and_branch_2`: Loads the address at data offset 34 into register a0, then branches unconditionally into the following routine.
- `load_address_and_branch_3`: Loads the address at data offset 0 into register a0, then branches unconditionally into the following routine.
- `load_address_and_branch_4`: Loads the address at data offset 110 into register a0, then branches unconditionally into the following routine.
- `forward_d0_to_callback`: Loads the address of a callback into a0 and branches to it, passing the incoming value in d0 unchanged so the routine simply forwards the argument to the callback and returns its result.
- `call_indirect_via_pointer`: Loads the address of a global variable into register a0 and jumps to the routine pointed to by that global variable.
- `load_address_and_branch_5`: Loads the address at data offset 204 into register a0, then branches unconditionally into the following routine.
- `load_base_and_branch`: This library code loads a0 with the base of a structure at offset zero and branches into another routine, returning the value it produced.
- `call_global_routine`: Loads the address of a global variable into register a0 and jumps to the routine pointed to by that variable, returning the value it produces.
- `return_immediately`: Does nothing; it immediately returns without modifying any data.
- `call_indirect_with_offset_2`: Loads an offset into a0 and falls through to a routine reached by adding that offset to a pointer stored in a global variable, returning whatever value that routine produces.
- `invoke_indirect_d0`: Reads a 32-bit value from a global variable, subtracts 36 from it to form an address, invokes the routine stored at that address passing the original value, and returns the value in D0.
- `branch_to_concat_helper`: Loads the address of a global variable at offset 214 into a0 and jumps to another routine, which then invokes a callback stored in a global variable passing the values in d0 and d1.
- `load_global_and_branch`: Loads the address of a global variable located 48 bytes below the program counter into a0, then branches into another routine without returning.
- `load_offset_and_branch`: Loads the address of a global variable located 50 bytes below the program counter into a0, then branches into another routine that calls a routine at an offset from that address.
- `load_offset_and_branch_again`: Loads the address of a global variable located 56 bytes below the program counter into a0, then branches into another routine that calls a routine at an offset from that address.
- `load_global_and_branch_again`: Loads the address of a global variable located 62 bytes below the program counter into a0, then branches into another routine without returning.
- `load_global_and_branch_third`: Loads the address of a global variable located 68 bytes below the program counter into a0, then branches into another routine without returning.
- `load_global_and_branch_fourth`: Loads the address of a global variable located 74 bytes below the program counter into a0, then branches into another routine without returning.
- `invoke_callback_concat_values`: This library code invokes a routine via a pointer stored in a global variable, passing the values from D0 and D1, then returns the concatenation of the two values now held in D0 and D1.
- `call_indirect_with_offset`: This library code calls a routine at an address formed by adding an offset in A0 to a pointer stored in a global variable, passing the value in D0, then returns the value now in D0.
- `normalize_fixed_point_value`: This library code normalizes a signed 32-bit fixed-point value in D6 by swapping its high and low words, adjusting a shift count based on sign and magnitude, then left-shifting the value while decrementing the count until the count reaches zero or the value overflows, returning early on overflow.

### icon.library

- `return_negative_one`: This library code loads the value negative one into D0 and returns immediately.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `add_to_reference_count`: Checks whether the word at offset 32 of the structure in a6 is zero, and if so inserts the structure into a doubly linked list by updating neighboring links, then calls a library function to adjust a reference count and branches to a later routine.
- `set_flag_in_structure`: This library code sets bit 3 of the byte at offset 14 of the structure in A6, loads zero into D0, and returns.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `store_pointers_and_call_vector`: This library code stores the pointer in A0 at offset 42 and the library base pointer at offset 34 of the structure in D0, calls a routine through the library vector, stores its result at offset 38, and otherwise copies both pointers into two global variables.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `zero_filled_data_area`: This range contains a single instruction, ori.b #0, d0, which is a zero-filled data area referenced by address rather than executed.
- `manage_icon_chain`: This library routine calls another routine to manage an icon chain.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `invoke_icon_structure_update`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `manage_icon_chain_2`: This library routine calls another routine to manage an icon chain.
- `call_incremented_recursion`: Saves the stack pointer and a6 to the stack, calls another routine with the same first argument and the second argument increased by a constant, then restores the stack and returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_icon_structure_update_2`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `manage_icon_chain_3`: Saves a0 and a6 onto the stack, calls the icon management routine via a subroutine link, removes the two saved arguments from the stack, and returns.
- `perform_incremental_recursion_2`: Saves the stack pointer and a6 to the stack, performs a recursive call with an incremented parameter, then restores the stack and returns.
- `call_other_routine`: This library code calls another routine and then returns.
- `call_vector_and_return`: This library code saves two address registers on the stack, calls another routine through a vector, restores the registers, and returns.
- `call_and_return_registers`: Saves registers a0 and a1 on the stack, calls another routine, restores eight bytes from the stack, and returns.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `zero_filled_data_area_2`: Contains a single instruction that reads a zero word from a data area by address, so it is most likely a zero-filled data area referenced by other code rather than an executable routine.
- `process_icon_chain`: Reads two pointers from offsets 8 and 12 of the structure in a6, saves them on the stack, and calls a library routine with the first pointer.
- `copy_structure_fields_from_source`: Copies ten longwords from a source area in a6 to a destination area in a2 using a decrementing loop counter, then copies eight more individual fields from a6 into specific offsets of the structure in a2.
- `return_zero_stub`: Loads the constant zero into d0, restores the caller's registers from the stack, restores the frame pointer, and returns the zero value.
- `copy_icon_data_to_stack`: This library routine copies ten longwords from an offset within a structure pointed to by the third argument into a local array on the stack, then copies several additional fields from the same structure into other stack slots and finally calls another routine with the first two arguments and a pointer to a stack-resident header.
- `update_icon_structure`: Initializes an icon structure by calling the icon library to populate it, then returns zero if the icon does not exist, otherwise it processes the icon's callbacks and updates its associated fields.
- `invoke_and_process_icon_callback`: Invokes a callback and returns its result, then, when a structure in a2 has a specific ID and flag, clears two fields and repeatedly calls library routines to fill pointer fields, looping while a counter is positive.
- `load_and_call_icon_routine`: Loads two arguments from the stack and calls a library routine, storing its result when nonzero, then branches into a larger routine that walks a table of pointers and invokes callbacks for each entry.
- `call_routine_loop_store`: Calls a routine through a register and, if the result is nonzero, calls a library function storing its result, looping while a counter stays above four and finally calling a third routine.
- `branch_with_zero_value`: Loads the constant zero into a register and then branches into the middle of another routine, so it performs no work of its own.
- `restore_regs_and_return`: Restores the caller's registers from the stack and returns without modifying anything.
- `process_icon_table_entry`: Loads a pointer from a global table and, if it is non-zero, calls a routine with a second pointer and the value 103, then calls another routine with the first pointer and the value 103.
- `process_icon_table_entries`: Saves the 26-byte field of a structure in a2, calls three library routines, and clears that field if the third returns zero.
- `load_and_call_icon_table`: Loads three arguments from the stack and calls a library routine twice, storing each result when nonzero, then branches into a larger routine that walks a null-terminated table and invokes callbacks for each entry.
- `iterate_table_and_call_callback`: Advances a pointer in a6 by four bytes and loops until the pointed-to value is zero.
- `iterate_icon_table_entries`: Library code that walks a null-terminated table of pointers, calling a routine for each entry, updating a counter, and invoking a callback while potentially processing a secondary entry.
- `push_a2_and_branch_forward`: Pushes a2 onto the stack, loads a0 from offset 12 of the stack, increments that value by two into a2, and then branches into another routine.
- `iterate_linked_list_nodes`: The routine traverses a linked list structure by following pointers stored at an offset from the current node.
- `increment_param_and_recurse`: Library code that calls another routine with the same first argument and the second argument increased by a constant, then returns.
- `allocate_and_init_object`: Allocates a new object from a global allocator and, if the allocation succeeds, calls a library function to initialize the object, then calls a second library function when the initialization fails, and finally returns the object pointer or zero.
- `call_library_and_check_result`: Pushes a global address and a structure offset onto the stack, calls a library function, then returns zero if the result is zero otherwise returning the value in a2.
- `return_zero_and_restore_regs`: Loads d0 with zero, restores the caller's registers from the stack, and returns.
- `decrement_counter_store_pair`: Decrements a short counter through a pointer and, if it stays positive, calls a library function; otherwise it decrements again and stores two stack arguments into a table indexed by the counter.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, then uses the counter value as an index into a table pointed to by a global variable.
- `allocate_and_init_icon`: Reads two pointers from offsets 20 and 24 of the stack, allocates a new object with two constants, and branches to the exit if the allocation fails.
- `branch_into_other_routine`: Branches into another routine without executing any other instruction.
- `call_library_routine_and_return_zero`: Library code that calls another routine and then returns zero.
- `perform_incremental_recursion`: Library code that performs a recursive call with an incremented parameter until the callee stops the recursion.
- `call_pointer_vector`: Loads a pointer from the stack into a register, pushes it on the stack, and calls another routine through a vector at address zero, then moves the result into a register and returns.
- `call_library_then_compare_byte`: Saves registers on the stack, calls a library function, and if the result is nonzero it advances a pointer by a saved value then compares a byte at that pointer against 61.
- `load_pointer_and_check_zero`: Loads a pointer from the stack into a3, copies it into d0, and returns zero when that pointer is null, otherwise restoring registers and returning.
- `restore_registers_call_vector`: Restores five registers from the stack, pushes a saved long word, and calls a routine through the vector at address zero, then jumps to a nearby routine.
- `call_and_subtract_result`: Pushes a constant on the stack, pushes d2, calls a routine, and when the result is zero returns immediately, otherwise subtracts d2 from the result and branches to a later routine.
- `call_routines_and_set_d2`: Calls a routine with the value in d2, compares its result with d5, and if they match and the second call returns zero it sets d2 to one and returns it.
- `maybe_increment_d2`: Tests whether the value in d4 is zero and, if so, returns without changes, otherwise increments the value in d2 by one and returns.
- `return_when_register_zero`: Clears a register and returns immediately when it is zero, otherwise it loads saved registers from the stack and returns.
- `format_message_branch_when_zero`: Clears a byte in a structure, then calls a formatting routine and branches away when its result is zero.
- `push_addresses_call_routine`: Increments a pointer by five, pushes two addresses on the stack, and calls a routine through a vector at address forty-eight, then adjusts the stack pointer and returns.
- `clear_d3_and_branch`: Clears register d3 by loading the value zero into it, then branches to the routine at offset 2396.
- `accumulate_and_format_message`: Multiplies a counter by ten and adds it to another value while reading bytes from an incrementing pointer, then calls several routines to format a numbered message.
- `map_byte_value_into_range`: Reads a stack argument and compares it against the constant forty-eight and the constant fifty-seven, mapping values within that range directly to themselves and values outside it into the range by subtraction, then returns the mapped value.
- `load_ff_and_return`: Loads d0 with the byte value 255 and returns.
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
- `allocate_new_object`: Calls a library function with the arguments 96 and 65536, and if the returned pointer is nonzero stores the constant 10 into the word at offset 14 of that structure and into the pointed-to word, then calls another library function with the adjusted pointer and returns nonzero.
- `return_zero_value`: Loads the constant zero into d0, restores the caller's registers from the stack, and returns the zero value.
- `compare_and_select_result`: Loads two stack arguments into d0 and d1, pushes them for a call, compares the result with a third argument, and returns either the original value or zero depending on whether they are equal.
- `tiny_move_rts`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `compare_library_result`: Copies the second stack argument into d1 and the first into d0, pushes all three stack arguments onto the stack, and calls a library function.
- `copy_d1_to_d0`: Copies the value in d1 to d0 and returns immediately.
- `divide_32bit_by_16bit`: If d2 exceeds 65535 it splits the 32-bit value in d1 into its halves and performs a long division routine over sixteen iterations using the carry flag.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `compare_byte_against_count`: Loads a pointer and a byte count, then reads bytes from an incrementing pointer and compares each against the count, decrementing the pointer and returning when a byte is zero or the bytes differ.
- `load_zero_and_branch_forward`: Loads d0 with zero and then branches into another routine.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `check_byte_and_branch`: Loads a0 and a1 from the stack, then checks the byte at a0.
- `compare_byte_tables`: Compares bytes from two incrementing tables, returns zero when they differ or the compared byte is zero, and loops while the counter stays positive.
- `select_clamp_value_by_comparison`: Loads the constant one into a register and returns immediately if the previous comparison was greater, otherwise loads the constant one hundred and fifty-five and returns.
- `test_byte_then_decrement`: Tests a byte at an incrementing pointer and returns immediately if it is zero, otherwise decrements a counter and loops while it stays positive.
- `decrement_pointer_then_call`: Decrements a pointer register and then calls another routine before returning, so it simply advances the pointer and delegates the rest.
- `copy_bytes_until_zero`: Decrements d0 and branches if the result is negative.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `process_icon_request`: Library code that validates a parameter, calls another routine to check a return value, and if successful, performs further calls using a value from a pointer.
- `load_args_and_branch`: Loads three long-word arguments from the stack into d2, d3 and a2, then branches into another routine without returning.
- `call_library_double_byte`: Pushes three stack arguments and calls a library function, then if the result is nonzero reads a byte from the structure, doubles it twice, and calls the function again before returning one or zero.
- `return_zero_and_restore_registers`: Returns zero in d0 and restores d2, d3 and a2 from the stack before returning.
- `invoke_callback_with_clamped_index`: Loads three stack arguments and a flag, then calls the first callback with the two values and the constant sixteen thousand and the pointer to a fixed location, storing the result.
- `return_negative_one_2`: Loads the constant negative one into d0, restores the caller's registers from the stack, and returns that value.
- `load_stack_args_and_branch`: Loads four long-word arguments from the stack into d2, d3, d4 and d6, setting d6 to zero, then branches into another routine without returning.
- `call_reset_vector_callback`: Pushes the constant sixteen thousand and the pointer to a fixed location onto the stack, then pushes the third and second stack arguments and calls the routine at the vector in the reset vector table.
- `call_library_store_pointer`: Copies a stack argument into a structure, calls a library function, and if nonzero reads a byte, doubles it twice, calls another function, and finally stores a pointer from the object into the caller's stack.
- `restore_registers_and_return_2`: Restores the caller's registers d2 through d6 and a2 from the stack, then returns.

### mathieeedoubbas.library

- `return_constant_2`: Returns the constant -1 in d0 to the caller without any other effect.
- `store_and_dispatch_ieee`: Stores the incoming pointer in a global variable, then reads a flag byte at offset 297 of the structure in a6 and, when that flag is clear, calls the routine at offset -498 and saves its result.
- `init_ieee_operation_struct`: Allocates a structure from a library vector, stores two routine addresses and the caller's arguments into it, and sets a flag byte from the caller's flag bit, then calls a stored routine if a caller flag is set and finally calls a second library vector.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `raise_ieee_exception_code`: Stores two into a 16-bit field of the structure at A0, then, depending on the low byte of the exception code in D7, deliberately divides by zero or raises the overflow trap.
- `compare_d7_and_return`: Compares the word in register d7 against the decimal value 7680 and branches away when d7 is at least that value, otherwise it returns.
- `compare_d7_and_branch`: Compares the word in d7 against 7680 and branches away when it is at least that value, otherwise it branches to handle the exception.
- `compare_exception_code`: Compares a word register against 23552 and branches away if it is at least that value, otherwise returns immediately.
- `route_ieee_exception_code`: Compares the exception code in d7 with 22523 and, if it is at least that value, jumps to another routine; otherwise it jumps to a third routine.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `store_value_branch_when_valid`: Pushes a register on the stack, loads a base pointer, writes a constant into a structure field, reads a value from that structure, and branches to a routine when the value is neither the compared constant nor negative.
- `compare_and_store_double_value`: Compares the word in d7 against the decimal value 51456 and branches away when they match.
- `compare_and_load_double`: Compares the 16-bit value in d7 with 51653 and jumps away if they match, otherwise loads two long words from the address in a1 into d0 and d1, checks the signed flag of the word at a0, pops a return address into d7, and returns.
- `clear_field_check_high_byte`: Saves register d7, loads a base pointer from a global variable, and computes an offset within that structure.
- `test_word_branch_when_valid`: Tests a word at a pointer and returns when it is negative, writes a constant into a structure field, reads a value from that structure, and branches to a routine when the value is neither the compared constant nor negative.
- `handle_exception_by_code`: Compares a register against the constant fifty thousand one hundred and sixty and returns immediately if equal, otherwise stores the constant ninety into a structure field, tests another field and branches if negative, clears a return register, restores a caller register from the stack, and returns.
- `split_double_components`: Extracts the high and low words of a 64-bit double from registers d0 and d2, isolates their sign, exponent and mantissa fields, then branches to a comparison routine when either exponent is below 32752 and to another routine when the values are zero.
- `compare_registers_and_branch`: Compares d7 against d2, then compares a0 with d2 and returns immediately if d3 is zero, otherwise it loads d2 and d3 into d0 and d1 and branches to a handler.
- `return_hardcoded_double`: This is an empty routine that does nothing and returns immediately.
- `xor_and_restore_context`: Returns immediately if both long words in d2 and d3 are nonzero, otherwise loads the address from a0 into d0, XORs it with the 16-bit value in d4, clears d1, restores the caller's registers, and returns the adjusted value.
- `shift_xor_double_left`: Xors a 16-bit value held in a register with the constant 32752, shifts the result left by 48 bits, and returns it as a 64-bit value.
- `compare_registers_store`: Compares d2 with d7 and branches to 1908 when it is smaller, then compares d2 with a0 and branches to 1896 when they differ, otherwise loads d2 into d0 and d3 into d1 before jumping to 4186.
- `shift_double_left`: Takes a 16-bit value held in a register, shifts it left by 48 bits, and returns the result as a 64-bit value.
- `combine_registers_branch`: Combines d0 and d7 into d5, then tests d0 through d3 and branches to 1938 when the first is nonzero and to 1934 when the last is nonzero, otherwise jumps to 4172.
- `shift_d4_left_64`: Takes the 16-bit value from register d4, shifts it left by 48 bits, and returns the result as a 64-bit value.
- `branch_to_two_routines`: Performs a branch to the routine at offset 3704 and then branches to the routine at offset 1950, doing nothing else with its own instructions.
- `adjust_registers_branch`: XORs d5 into d0, toggles bit 4 of d0, ANDs d2 into d7, and branches away if d7 is nonzero; it then tests d2 and d3 and branches on each, so it only adjusts registers and branches.
- `swap_registers_branch`: Swaps the register pairs d0 with d2, d1 with d3, and d5 with d7, then calls a library routine via a subroutine link.
- `adjust_and_branch_double`: Adjusts d2 by toggling bit 4 and clearing a mask bit, then decrements d7 and subtracts it from d5.
- `compute_64bit_division_2`: Computes the quotient and remainder of a 64-bit by 64-bit unsigned division using a long-division algorithm on the 16-bit halves of the operands, correcting the estimated quotient by adding or subtracting the divisor when the intermediate products overflow.
- `compute_64bit_division`: Computes the quotient and remainder of a 64-bit by 64-bit unsigned division using a long-division algorithm on the 16-bit halves of the operands, then calls divide_double_words to finish the double-precision result.
- `divide_double_words`: Performs a 64-bit by 64-bit double-precision division.
- `multiply_double_words`: Multiplies the two 32-bit values passed in d0 and d1 by splitting them into 16-bit halves, combines the partial products with a borrow-propagating add, and adjusts the result when the inputs are negative.
- `adjust_shift_count_branch`: Negates a 16-bit value in a register, shifts it right by 4 bits, adds the constant 2, and compares the result against 57.
- `shift_right_pair_by_count`: Shifts a 64-bit register pair right by a count taken from the 16-bit value in D5 and returns the shifted pair, with the exponent word of the operand XORed into it.
- `shift_and_branch_signed`: Arithmetic-shifts d0 right by one bit, rotate-shifts d1 right by one bit, tests the word in d5, and branches away when it is non-negative, otherwise falling through to another routine.
- `shift_and_branch_signed_2`: Tests bit 23 of the 32-bit value in d0 and returns when it is clear, then decrements the 16-bit value in d5 and returns when it is negative.
- `build_fixed_double_constant_2`: Xors a 16-bit value held in a register with the constant 32750, combines it with a constant mask, scales the result, and returns a fixed 64-bit constant pattern.
- `normalize_double_shift_left`: Normalizes a 64-bit value split across d0 and d1 by repeatedly swapping halves and shifting left until the top bit is set, counting the shifts made, then applies a lookup-table offset and final rotations to return the normalized value.
- `shift_double_exponent_xor`: Computes a leading-zero count for a 32-bit value using a lookup table and a small loop, then shifts a 64-bit value right by the resulting count while XORing the exponent word into the result.
- `shift_right_pair_exponent_xor`: Treats D0 and D1 as a 64-bit value, shifts it right by the count in D5, XORs the exponent word of the operand into the result, and returns the shifted pair.
- `shift_word_left`: Library code that shifts a 16-bit value left by 48 bits and returns it as a 64-bit result.
- `promote_half_to_upper`: Library code that takes the low 16 bits of a 3and value held in a register and places them into the upper half of a 64-bit result, leaving the lower half zero.
- `build_fixed_double_constant`: Library code that combines a 16-bit value with a constant mask, scales it, and returns a fixed 64-bit constant pattern.
- `shift_xor_word_left`: Library code that xors a 16-bit value with a constant, shifts the result left by 48 bits, and returns it as a 64-bit value.
- `return_hardcoded_64bit`: Library code that returns a single hardcoded 64-bit constant.
- `set_signaling_bit_return_pattern`: If bit three of D0 is set, clears it and sets bit one, then returns a 32-bit pattern formed from the value with a fixed mask OR-ed in and its two halves swapped.

### diskfont.library

- `check_and_set_font_handle_2`: Calls the font handler via a table entry using the library base, stores the returned handle at offset 34 of the library structure when non-zero, invokes the cleanup routine, and returns whether a handle was obtained.
- `check_and_set_font_handle`: This library routine calls a routine via a table entry using a library base.
- `open_and_init_font`: Saves a6, installs a new library base pointer and its ID, calls two library entry points in turn, and if both succeed it allocates and initializes a small node on the heap before returning the base address.
- `close_and_free_library`: Saves a6, loads a library base pointer, and calls the entry point at offset -414.
- `unload_and_free_fonts`: Walks a doubly linked list of font structures, unlinking each entry whose flag word at offset 84 is not greater than zero and decrementing a counter at offset 40 of the structure in a6.
- `close_and_free_font`: Restores a2, then checks two flag words at offsets 38 and 40 of the structure in a6 and branches away if either is nonzero.
- `set_flag_and_clear_d0`: Sets bit 7 of the byte at offset 14 of the structure in a6 and places the value zero into d0.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `decrement_and_check_flag`: Decrements a 16-bit counter at offset 26 of the structure in a6 and returns if it is not zero, otherwise inspects bit 7 of the byte at offset 14 and jumps away when that bit is set.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `copy_font_string_and_scan_directory_2`: Copies the next byte from the source string into the destination buffer, advancing both pointers, and when the end of the string is reached it returns.
- `load_string_and_process`: Loads the address at offset 10 of the structure in a4 into a1, sets d4 to one, and branches into another routine to process the string.
- `process_string_and_copy_bytes`: Increments a counter and scans the string pointed to by a1 byte by byte until it finds a zero byte, then subtracts the counter from a value in d5 and compares it against a value in d2.
- `copy_font_string_and_scan_directory`: Copies the next byte from the source string into a destination buffer, advancing both pointers, and when the end of the string is reached writes a new entry into a table.
- `copy_diskfont_string_2`: Copies a null-terminated string from a source buffer into a destination buffer, storing the destination length and a few fields from a structure, then returns the absolute value of a decremented counter.
- `copy_diskfont_string`: Increments a byte counter and reads bytes from the source buffer until a zero byte is found, then subtracts the counter from two lengths and copies the counter bytes from the structure at offset 8 into the destination.
- `handle_diskfont_copy`: Copies one byte from the source buffer pointed to by a1 into the destination at a3 plus d5, then advances a3.
- `copy_directory_entry_name`: Reads a directory entry and, if it is a file, copies its name into a destination buffer while tracking the remaining space, then continues scanning entries until none are left, returning the number of bytes still available.
- `restore_registers_return_zero`: Loads sixteen registers from just below the structure in a6, restores the stack pointer, places zero into d0, and returns.
- `compute_clamped_index_delta`: Loads a 16-bit value from offset 256 of the structure in a1 and subtracts a 16-bit value at offset 4 of the structure in a0, then multiplies the difference by 64 when it stays non-negative or by -256 when it borrows, adding the result to a value initialized to 32767 in d2.
- `subtract_indexed_table_entry_2`: Computes a value from d2 and d5, then loops seven times comparing bytes at offsets 6 and 7 of a0 with bytes at offsets 258 and 259 of a1.
- `subtract_indexed_table_entry`: Subtracts a table entry selected by a shifted counter from a value, then adjusts the operation based on bits of a register and bytes at offsets 259 and 258 of a structure in a1, looping while a shifted mask is set and returning when the counter reaches 6.
- `iterate_table_entries`: Iterates over a table of 32-bit entries indexed by a counter, subtracting the selected entry from a value and incrementing the counter, then returns a constant value.
- `return_constant_value`: This library routine returns a constant value without using any arguments or registers.
- `load_font_and_find_glyph`: Loads a font structure pointer, copies its header fields into a local buffer, locks the font resource, and then searches for the next glyph by comparing bitmap data, storing glyph metrics and updating the glyph table entry.
- `lock_font_resource`: Loads the constant 254 into d2, pushes it onto the stack, pushes a zero long word, and calls the lock routine via a subroutine link; it stores the result into a structure and branches based on the returned value.
- `find_and_link_glyph_bitmap`: Allocates a bitmap sized from a 16-bit value in a structure, then scans a table of 260-entry records to find the one whose height is greatest, storing its index and linking it into the table while clearing or reusing the previous bitmap.
- `clear_registers_branch`: Clears the 16-bit values in d4 and d3, loads the byte value 255 into d2, and then branches into the middle of another routine.
- `find_or_allocate_glyph`: Searches a font glyph table for the glyph whose scaled width is smallest and, if its width field equals 6320, allocates a new glyph entry, links it into the table, and registers it with the graphics library.
- `load_segment_and_scale`: Pushes a register on the stack, calls a library routine via a vector, scales a 16-bit value by 260, and jumps back into the middle of another routine.
- `allocate_next_glyph`: Clears the glyph bitmap pointer at offset 256 of the font glyph table entry selected by the glyph index, then searches that table for the next unused glyph and, if it is a valid bitmap font glyph, marks it as allocated and records its index.
- `build_disk_font`: Locks the diskfont directory, opens the .font file, allocates a bitmap and a font header, then scans the file's glyph table to build a disk font, returning a pointer to the font header or zero on failure.
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
- `load_pointers_and_branch`: Copies the pointer at offset 4 of the stack into a0, copies the value at offset 8 of the stack into d0, and jumps into another routine.
- `skip_to_string_terminator_and_call`: The routine increments a pointer until it reaches the terminating zero byte of a zero-terminated string and then calls another routine with the pointer to that terminator.
- `load_pointer_and_branch`: Copies the pointer at offset 4 of the stack into a0, places zero into d0, and jumps into another routine.
- `count_null_terminated_bytes`: As part of library code, this routine increments a counter while advancing a pointer and reading bytes until a zero byte is encountered, returning the final count.
- `compare_byte_sequences_2`: Loads the two pointers from the stack and compares the byte sequences they point to.
- `skip_byte_if_nonzero`: Library code clears d0 when the byte at the a0 pointer is zero, otherwise it advances a0 past that byte and returns, so it skips one byte only when the byte is nonzero.
- `compare_byte_sequences`: Compares a byte at the a0 pointer with an incremented byte at the a1 pointer, returns zero when they match, otherwise compares a decremented a1 byte and returns 255 when it is greater.
- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.
- `load_pointer_and_branch_2`: Loads a pointer from the stack into a0, copies a byte from the stack into d0, and jumps into another routine.
- `forward_pointer_if_bytes_match`: Library code compares d0 with d1 and returns immediately when they differ, otherwise it copies the a0 pointer into d0 and returns, so it forwards the pointer only when the bytes match.
- `advance_pointer_if_nonzero`: Advances the a0 pointer by one, loads the next byte into d1, and returns immediately when that byte is zero, otherwise falling through.
- `restore_registers_if_below_2`: Saves d2 and d3, loads a value and a byte from the stack into d3 and d2, pushes d3, and calls the routine at offset 2c.
- `forward_pointer_if_byte_matches`: Library code compares the byte at the a0 pointer with d2 and returns immediately when they differ, otherwise it copies the a0 pointer into d0 and returns, so it forwards the pointer only when the byte matches.
- `restore_registers_if_below`: Decrements the a0 pointer, returns immediately when it is not at or above the d3 value, otherwise restores registers d2 through d3 from the stack and returns.
- `call_vector_and_increment`: Reads a pointer from offset 8 of the stack, pushes the constant 47, and calls the routine at offset 7E, then pushes the constant 58 and calls it again if the first result is zero.
- `increment_returned_value`: This library routine returns the value passed in D0 incremented by one.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.
- `call_library_vector_132_2`: Saves registers a0 and a1 on the stack, calls the library function at offset minus 132 of a base loaded from a global variable, restores the stack, and returns.
- `call_another_routine`: This library routine is library code that acts as a thin wrapper which immediately invokes another routine and then returns without performing any additional operations.

### Ace the Space-Case.adf:ACEDEMO!

- `process_hunk_stream`: Parses an AmigaDOS hunk stream, tracking hunk sizes and relocating their contents in memory, and allocates additional memory when a hunk exceeds the size limit.
- `decrement_until_zero`: Loops decrementing a 16-bit value until it becomes zero and returns, then later compares a value against 1010 and calls a routine to allocate fast memory.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `store_bytes_from_length`: It loads a length from four bytes before a0 into d1, returns early when that length is zero, then subtracts one from it and shifts it right by eight to compute an offset added to a3.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `shift_bits_and_set_color`: Loops over a bit count, shifting bits from a source register into an accumulator while reloading the source when it empties, then stores a derived value into the COLOR00 hardware register before comparing two pointers and returning.
- `read_word_trap_or_d0`: Reads a word from a memory location derived from a2 into d4, raises a trap, and bitwise-ORs d2 into d0.

### Ace the Space-Case.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
