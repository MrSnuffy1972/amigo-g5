# Game 854 (688 attack sub (1990)(electronic arts)[cr ffm]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00854). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 17 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 162 of 162 named (0 author symbols), 162 of 162 described (39 by rule, 95 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 160 of 160 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 654390 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2686 of 2686 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 3023 |
| code | 8122 |
| data | 822528 |
| padding | 3447 |
| unused | 64000 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Type | 2 | 117 | 2 |
| endcli | 2 | 117 | 2 |
| echo | 2 | 117 | 2 |
| loadwb | 12 | 536 | 30 |
| 688 Attack Sub (1990)(Electronic Arts)[cr FFM].adf:Frankfort | 2 | 207 | 8 |
| 688 Attack Sub (1990)(Electronic Arts)[cr FFM].adf:688 | 2 | 207 | 8 |
| icon.library | 10 | 1375 | 109 |
| 688 Attack Sub (1990)(Electronic Arts)[cr FFM].adf:bootblock | 1 | 10 | 1 |
| 688 Attack Sub (1990)(Electronic Arts)[cr FFM].adf:688.mlb (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- 688 attack sub (1990)(electronic arts)[cr ffm]/688 Attack Sub (1990)(Electronic Arts)[cr FFM].adf:688.mlb: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 688 attack sub (1990)(electronic arts)[cr ffm]/688 Attack Sub (1990)(Electronic Arts)[cr FFM].adf:Frankfort: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 688 attack sub (1990)(electronic arts)[cr ffm]/688 Attack Sub (1990)(Electronic Arts)[cr FFM].adf:688: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 654390 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_d1_to_d0_and_branch`: Copies the 32-bit value from register d1 into register d0, then unconditionally branches to the routine at offset 104.

### endcli

- `find_max_and_prepare_buffer_2`: Walks a linked list of entries to find the maximum value in a field, then writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_d1_to_d0`: Copies the value in d1 into d0 and then branches into the middle of another routine.

### echo

- `find_max_and_prepare_buffer_3`: Walks a linked list of entries to find the maximum value in a field, then writes a small header and fills a buffer with sequential words.
- `move_d1_to_d0_2`: Copies the 32-bit value from register d1 into register d0 and then branches to another routine.

### loadwb

- `parse_command_line_into_tokens`: Parses a command-line string into tokens, stripping surrounding quotes and converting asterisk escapes like *N and *E into newline and escape characters, then stores the resulting token count and pointer into a structure.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `start_tool_from_resident_tag`: XOR-decodes a global string with one key applied twice, allocates a 160-byte block, and uses icon, intuition, and workbench library calls to start a tool.
- `compare_null_terminated_strings`: Compares two null-terminated strings byte by byte and returns minus one when the first is lower, one when it is higher, and zero when they match.
- `build_linked_list_from_table`: Walks a table of pointer and value pairs, allocating a fixed-size node for each entry through a memory allocation call and storing the looked-up value in it.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `compare_byte_string_wb`: Returns immediately when a pointer is negative one, otherwise compares a byte at offset 20 of the pointer against the character two.
- `write_period_and_spaces`: Writes a period followed by five spaces into consecutive bytes of a buffer pointed to by a register, then branches into another routine.
- `compare_byte_strings`: Compares the bytes of two strings pointed to by registers and branches back to repeat the comparison while they match, otherwise falls through to another routine.
- `load_pointer_and_return`: Loads a pointer from a global variable into a2, restores a0 and a1 from the stack, and returns.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_with_two_params`: Copies two 32-bit values from the caller's stack into d2 and d3, pushes a stack-slot pointer plus those values, then calls another routine with them.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### 688 Attack Sub (1990)(Electronic Arts)[cr FFM].adf:Frankfort

- `load_and_relocate_hunk`: Loads an executable hunk from disk and applies its relocations by copying and adjusting the referenced data, then calls the entry point.
- `increment_counter_if_nonzero`: Tests the value in d5 and returns immediately if it is zero, otherwise increments a counter in d6, sets d5 to one, and returns.
- `decrement_loop_counter`: Decrements a loop counter in d1 while it is non-zero and returns when it reaches zero.
- `tiny_ori`: Small routine of 1 instruction that merges 228 into d0.
- `decode_run_length_backwards`: Reads two little-endian words from the stack, combines their bytes into a bit offset and a bit count, and sets d7 to a space character minus the offset.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_byte_cycle_counter`: Decrements a byte counter in d7 for each iteration of a loop controlled by d0, resetting it to a space character whenever it reaches zero, and returns.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### 688 Attack Sub (1990)(Electronic Arts)[cr FFM].adf:688

- `process_hunk_table`: Loads the AmigaDOS hunk table, walks its entries and for each one copies the size and pointer fields into a parallel table while skipping the executable, data, bss and tags hunk types, then calls the hunk loader and clears the flag when the process is not running.
- `increment_counter_if_nonzero_2`: Returns immediately when the value in d5 is zero, otherwise increments the counter in d6, sets d5 to one, and returns.
- `decrement_loop_until_zero`: Subtracts one from a loop counter in d1 and repeats the decrement until the counter reaches zero, then returns.
- `tiny_ori_2`: Small routine of 1 instruction that merges 152 into d1.
- `decode_run_length_backwards_2`: Reads two little-endian words from the stack and combines their bytes into a bit offset and a bit count, then sets a byte counter to a space character minus the offset.
- `set_d0_to_one`: Loads the constant one into d0 and returns.
- `decrement_byte_cycle_counter_2`: Decrements the byte counter in d7 on each pass and resets it to a space character whenever it reaches zero, looping until the counter in d0 is exhausted, then returns.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### icon.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `unlink_if_flag_clear`: If the flag word at offset 32 of the structure is zero, unlinks the structure from a doubly linked list and calls a library function; otherwise it sets bit 3 of the byte at offset 14 and returns zero.
- `set_flag_bit_in_struct`: Sets bit 3 of the byte at offset 14 in the structure pointed to by a6, loads zero into d0, and returns.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `store_and_call_library`: Stores a pointer from a0 and the base address at offset 34 into a structure, calls a library routine with a fixed argument, saves the returned value, and branches to error handling when that value is nonzero.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `tiny_ori_3`: Small routine of 1 instruction that merges 0 into d0.
- `zero_filled_data_area`: This is a zero-filled data area that other code refers to by address rather than executing code.
- `manage_icon_chain`: This library routine calls another routine to manage an icon chain.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `invoke_icon_structure_update`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `manage_icon_chain_2`: This library routine calls another routine to manage an icon chain.
- `call_icon_chain_manager`: Pushes a0 and a6 on the stack, calls another routine with the argument in a0, restores the stack, and returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_icon_structure_update_2`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `manage_icon_chain_3`: This library code saves registers a0 and a6, calls another routine to manage an icon chain, restores the stack pointer, and returns.
- `save_registers_and_call`: Saves the a0 and a6 registers on the stack, calls another routine, restores the registers, and returns.
- `call_other_routine`: This library code calls another routine and then returns.
- `call_other_routine_2`: This library code saves registers a0 and a1, calls another routine, restores the stack pointer, and returns.
- `call_and_return_registers`: This library code saves registers a0 and a1, calls another routine, restores the stack, and returns.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `store_zero_byte`: This routine is a single instruction that writes a zero byte to the memory location pointed to by the d0 register, then returns.
- `allocate_and_configure_icon`: This library code sets up a frame, saves registers, loads two arguments from the frame, and calls a routine with the first argument, then if the result is zero it branches to a cleanup path, otherwise it pushes two address offsets and calls another routine, and if that fails it calls a third routine before returning.
- `copy_struct_fields`: Copies ten long words from a source table to a destination structure using a decremented counter loop, then transfers eight fields from the source to the destination and stores the destination pointer into a global variable when the destination flag is non-zero.
- `return_zero_stub_2`: Loads the constant zero into d0, restores registers d2 through d4 and a2 from the stack, undoes the frame pointer, and returns the zero value.
- `copy_icon_data_to_stack`: This library routine copies ten longwords from an offset within a structure pointed to by the third argument into a local array on the stack, then copies several additional fields from the same structure into other stack slots and finally calls another routine with the first two arguments and a pointer to a stack-resident header.
- `validate_and_propagate_icon_change_2`: Validates a pointer as an icon structure header, then propagates the change to its child and parent icons by calling the propagation routine and clearing the relevant fields.
- `validate_and_propagate_icon_change`: Validates a pointer in a2 as a 16-bit value of 58144 with a 16-bit value of 1 at offset 2, then clears two 32-bit fields at offset 0 and offset 4 of the structure in a4.
- `propagate_icon_change_3`: Calls a routine for two fields, stores non-negative results into them, then allocates memory and repeatedly invokes a callback through a vector while a counter remains positive.
- `loop_caller_store_vector`: Calls a routine via a vector in a3 and stores the result at a pointer in a4, then decrements a counter and advances the pointer while it stays greater than four.
- `set_zero_and_branch`: Loads the value zero into a register and then branches into another routine, so it only sets up the return value before handing off control.
- `copy_d4_and_return`: Copies the value in d4 into d0, restores registers d2 through d5 and a2 through a5 from the stack, undoes the frame pointer, and returns.
- `process_icon_table_entries`: Allocates a 256-byte block and, if the allocation fails, calls a cleanup routine with a 103 argument and returns zero.
- `iterate_icon_table_entries_2_2`: Walks a null-terminated table of pointers, calling a callback for each entry and accumulating a running total, returning early if any call fails.
- `iterate_icon_table_entries_2`: Calls a routine for two fields and returns early if either result is zero, then walks a null-terminated table of pointers, invoking a callback for each entry.
- `iterate_and_callback`: Advances two pointers by four until the first reaches a zero entry, then invokes a callback and walks a null-terminated table of pointers calling the callback for each entry while updating a counter, finally handling a special pointer and restoring state before returning.
- `iterate_icon_table_entries`: Library code that walks a null-terminated table of pointers, calling a routine for each entry, updating a counter, and invoking a callback while potentially processing a secondary entry.
- `advance_pointer_and_jump`: Pushes a2 onto the stack, loads a pointer from the stack into a0, advances that pointer by two into a2, and jumps into another routine without returning.
- `iterate_linked_list_nodes`: The routine traverses a linked list structure by following pointers stored at an offset from the current node.
- `increment_param_and_recurse`: Library code that calls another routine with the same first argument and the second argument increased by a constant, then returns.
- `configure_icon_fields_2`: Allocates a block, then calls a library function and a second routine, branching to a helper when the second call fails and otherwise calling a third library function.
- `call_and_return_result`: Pushes a constant word and a value from a2 onto the stack, calls another routine, and returns zero when that call fails, otherwise restoring registers and returning the result.
- `restore_registers_and_return`: Restores registers d2, d3 and a2 from the stack and returns without modifying any data.
- `decrement_counter_store_pair`: Decrements a 16-bit counter through a pointer and returns zero when it reaches zero or was already negative.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, then uses the counter value as an index into a table pointed to by a global variable.
- `validate_and_propagate_icon_change_3`: This library code saves registers, loads two arguments from the stack, calls a routine with a flag and a value, and if it returns zero it branches to a cleanup path, otherwise it computes an offset, calls another routine with that offset, and then calls a chain of routines with the arguments, branching to different cleanup paths depending on each result.
- `branch_to_target`: This routine is library code and consists of a single unconditional branch to another location, so it does nothing itself and simply transfers control to the target routine.
- `call_library_routine_and_return_zero`: Library code that calls another routine and then returns zero.
- `perform_incremental_recursion`: Library code that performs a recursive call with an incremented parameter until the callee stops the recursion.
- `push_stack_value_and_call`: Loads a pointer from the stack into a register, pushes the value it points to onto the stack, and calls a helper routine, then branches to one of two exit points depending on whether the original value was zero.
- `scan_bytes_find_marker`: Pushes three registers, calls a routine, then scans bytes from a pointer until it finds the value 61 or a zero byte, returning the matching pointer.
- `load_pointer_or_zero`: Loads a pointer from a2, returns it when non-zero, otherwise clears the result register and restores registers before returning.
- `save_registers_and_call_2`: This library code saves registers d2 through d5, loads two long words from the stack into d2 and d3, pushes d3 back onto the stack, and calls a routine with that value, then stores the result in d5, cleans up the stack, and branches to another routine.
- `subtract_and_branch_result`: Pushes a constant, calls a routine, returns when the result is zero, otherwise subtracts a saved value from it and branches to a helper.
- `compare_and_adjust_value`: Calls a routine with a value, returns one when the result equals a constant and is non-zero, otherwise computes a new value from a second constant and returns zero or one.
- `increment_if_nonzero`: Tests a register and returns without changes when it is zero, otherwise increments a copy of it by one and branches into another routine.
- `return_when_register_zero`: Clears a data register and returns immediately when it is already zero, otherwise restores the caller's registers and returns.
- `format_number_and_print`: This library code saves registers and loads two pointers from the stack, clears a byte at offset 30 of the first pointer, then calls a routine with a flag and a counter value, and if it returns zero it calls three different routines with the pointers and a constant, otherwise it branches to another routine.
- `push_addresses_and_call_helper`: Adjusts a stack pointer by a small amount, pushes two addresses onto the stack, and calls a helper routine, then branches to a setup path when the result is non-zero and otherwise sets a register and jumps to a further routine.
- `clears_register_and_jumps`: Clears the d3 register and jumps to another routine.
- `print_decimal_number`: Multiplies a counter by eleven, indexes a byte table, and calls a routine for each byte while the value is non-negative, then formats and prints the accumulated decimal number.
- `clamp_value_range`: Clamps a value read from the stack into a range by comparing it against two constants and subtracting the lower bound, then branches into another routine with the clamped result.
- `return_negative_one_2`: This routine is library code that loads the value negative one into a register and then returns to the caller, so it always yields negative one.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: The library routine forwards a set of four parameters to another routine, checks the returned value, and if non-zero calls the same routine again with a modified argument list.
- `configure_icon_fields`: Calls a library function with a flag and a value, and if it returns non-zero it stores ten into two fields and calls another library function with a pointer.
- `restore_regs_and_return`: Loads zero into d0, restores d2 and a2 from the stack, and returns.
- `call_library_with_args`: Loads three arguments from the stack into d0 and d1, pushes them back in a different order, then calls the routine whose address is stored at offset 48 in the data section and returns.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `call_library_two_args`: Loads two arguments from the stack into d0 and d1, pushes them, and calls a library function, then compares the result against a stored value.
- `copy_d1_to_d0`: Copies the value in d1 into d0 and then returns without performing any other work.
- `split_and_divide`: This routine is library code that returns immediately when the 16-bit value in a register exceeds sixty-five thousand five hundred thirty-five, otherwise it splits a 32-bit value into its halves and performs division and modulo operations on them before returning.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_advance`: This routine is library code that stores the byte in a register to the location pointed to by another pointer and then advances that pointer, after which it returns to the caller.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `compare_byte_stack_pointer`: Loads a pointer from the stack into a0 and a byte from the stack into d0, then reads a byte from the pointer, returns if it is zero, and compares it against the loaded byte.
- `set_zero_and_branch_2`: Loads the 16-bit value zero into register d0 and branches to a routine at offset 24.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `skip_if_byte_zero`: This routine is library code that loads two destination pointers and then skips the decrement when the byte at the first pointer is zero, otherwise it decrements that pointer before jumping to the shared code.
- `select_extreme_value_loop`: Loads two pointers from the stack, decrements a counter, and returns when it goes below zero.
- `select_extreme_value`: Loads one into a register and returns when a tested value is greater than zero, otherwise loads negative one into the register and returns.
- `test_byte_decrement_pointer`: Loads two pointers from the stack and tests a byte from the first pointer, returning if it is zero.
- `decrement_and_copy_byte`: Decrements a source pointer and then calls a byte-copying routine before returning to the caller.
- `copy_byte_loop`: This routine is library code that decrements a counter and branches away when it goes below zero, then copies a byte from the source pointer to the destination pointer and continues while the copied byte is not zero, otherwise it clears a byte at the destination and jumps to the shared code.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `process_icon_request`: Library code that validates a parameter, calls another routine to check a return value, and if successful, performs further calls using a value from a pointer.
- `restore_registers_branch`: Restores registers from the stack, loads two values and a pointer from the stack, and branches into another routine without returning.
- `call_library_scale_byte`: Calls a library routine with two stack arguments and returns zero if the result is zero, otherwise loads a byte from offset 7 of the structure in a2, sign-extends it, and multiplies it by four.
- `return_zero_stub`: Loads the constant zero into d0, restores registers d2 through d3 and a2 from the stack, and returns the zero value.
- `propagate_icon_change_2`: Reads four arguments from the stack, and if the last is non-zero calls a library function, then a second one with its result, and if both succeed performs further library calls and returns the first result.
- `return_negative_one`: Loads the constant negative one into d0, restores registers d2 through d5 and a2 from the stack, and returns the value negative one.
- `restore_registers_zero`: Restores registers from the stack, loads three values and a zero into a register, and branches into another routine without returning.
- `handle_icon_change`: Pushes a flag and a value, calls a library function with two stack arguments, and if the result is zero it returns; otherwise it inspects a flag and copies a value from the stack.
- `propagate_icon_change_loop`: Stores a stack value into offset 12 of the structure in a2, then calls a library routine and returns negative one if the result is zero.
- `return_negative_one_stub`: Loads the constant negative one into d0, restores registers d2 through d6 and a2 from the stack, and returns the value negative one.

### 688 Attack Sub (1990)(Electronic Arts)[cr FFM].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
