# Game 828 (paradroid 90 (demo) (1990)(hewson)[h black monks]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00828). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 18 raw-word lines |
| Functions named and described | pass, 268 of 268 named (0 author symbols), 268 of 268 described (11 by rule, 256 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1953 of 2002 (97.55%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5210 of 5210 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 232 |
| code | 17538 |
| data | 327854 |
| padding | 1512 |
| unused | 553984 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Type | 2 | 117 | 2 |
| Print | 2 | 117 | 2 |
| Paradroid 90 (demo) (1990)(Hewson)[h Black Monks].adf:ParadroidDemo | 1 | 4946 | 263 |
| Paradroid 90 (demo) (1990)(Hewson)[h Black Monks].adf:bootblock | 1 | 10 | 1 |

## Cases handled

- 6 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from

## Cracktros (kept, not protection)

- none

## Copy protection found

- `update_engine_state_and_pointers` (ParadroidDemo): Updates several pointers and status flags in a structure, clears a range of memory using a loop, and executes a sequence of callbacks and conditional logic checks while updating or clearing various status bits and pointers in the structure. Kind: sabotage path. Pass path: the instructions following the conditional branch (the code that continues execution or proceeds to the next state-update logic if the check does not trigger a jump/trap or if the check returns a non-zero/non-error value).

## Functions


### Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_long_register`: Copies a long-word from register d1 to d0 and then branches to another routine.

### Print

- `find_max_and_prepare_buffer_2`: Walks a linked list of entries to find the maximum value in a field, then if a condition is met, writes a header and a sequence of incrementing words into a buffer, sets up a stack frame, and copies a block of sixteen words from a temporary area to a global buffer before installing a function pointer and returning success or failure.
- `copy_register_and_branch`: Copies the contents of register d1 to register d0 and then branches to another routine.

### Paradroid 90 (demo) (1990)(Hewson)[h Black Monks].adf:ParadroidDemo

- `initialize_address_register`: Loads a relative address into a0, stores a0 into a global address, and calls a trap.
- `branch_to_address`: Performs a relative branch to a nearby address.
- `update_pointer_and_loop`: Checks a byte at a certain offset, and if it is not zero, copies a long-word from a pointer in a0 to a pointer in a3, then potentially enters a loop that copies or increments pointers and values or calls another routine.
- `copy_and_decrement_loop`: Copies a long-word from a pointer in a0 to a pointer in a3, decrements d2, and loops if d2 is not negative, then jumps to another routine.
- `call_and_branch_offset`: Copies a pointer from a structure, calls a routine, and then branches to another routine.
- `setup_hardware_and_jump`: Iterates through a list, configures certain hardware registers, calls several routines, and finally jumps to an indirect address.
- `initialize_memory_range_2`: Clears a range of memory, updates a global pointer, and sets a value in a structure before returning.
- `initialize_memory_range`: Writes a pointer to a global location, initializes a memory range starting at a certain address, and updates a value in a structure before returning.
- `set_pointer_and_branch`: Loads a constant into a3 and a constant into d2, then branches to another routine.
- `update_buffer_structure`: If d0 is greater than 19, it calls a trap, writes a byte and a zero-byte to a buffer, updates a count and a pointer in a structure, writes a value and a pointer to the buffer, and calls another trap before returning.
- `compare_pointers_and_return`: Loads several pointers into a0 and a1, calls another routine, and if the zero flag is not set, repeats this process with different pointers before returning with a constant value in d0.
- `write_constant_to_pointer`: Copies a0 to d0, copies a long-word from a pointer in a1 to d1, writes a constant long-word to the pointer in a1, compares a0 to the pointer in a1, and if not equal, copies d1 back to the pointer in a1 before returning.
- `tiny_move_moveq_rts`: Small routine of 3 instructions that copies d1 to (a1), then loads 1 into d1, then returns.
- `process_d0_and_branch_2`: Compares d0 to a constant, and if it is not less than that constant, modifies d0 and calls a trap, then potentially calls another routine.
- `process_d0_and_branch`: Compares d0 to a constant, and if it is not less than that constant, modifies d0 and calls a trap, then potentially calls another routine and returns or calls a trap and returns.
- `manage_linked_list_node_insertion`: Searches through a linked list or stack-like structure via a pointer in a structure, and if a certain condition is met, inserts a new node or updates an existing node by storing a byte from a register and updating pointers and a size counter in a global structure.
- `manage_linked_list_nodes`: Performs complex pointer and value updates or linked list insertions or deletions based on checking bytes and long-words, and calls traps or other routines.
- `copy_data_with_conditional_shifts`: Saves d0 to the stack, calls another routine, restores d0, sets a1 to a0, clears d1, and then checks bits in d0 (after a right shift) to decide whether to copy a word from a1 to a destination or skip a copy or branch back to a previous instruction.
- `swap_and_copy_loop`: Swaps the high and low words of d0, then copies d1 to a destination pointed to by a1 twice, decrementing d0 and using a nested loop that decrements d0 again, then returns.
- `update_structure_values_2`: Loads a pointer from a structure, and then loops to call a routine via a table entry while storing a value from a register into a structure, then modifies hardware registers and a structure, and returns.
- `execute_indirect_call_offset_8`: Performs a trap, saves registers, loads a pointer from a structure, loads a sub-function pointer from an offset of 8, calls that sub-function, restores registers, and performs a trap before returning.
- `execute_indirect_call_offset_108`: Performs a trap, saves registers, loads a pointer from a structure, loads a sub-function pointer from an offset of 108, calls that sub-function, restores registers, and performs a trap before returning.
- `update_structure_values`: Reads from hardware registers, modifies d2 based on certain bits, and updates several values in a structure at certain offsets, potentially calling other routines.
- `update_structure_with_subtracted_values`: Calculates a value from d0 and d1, stores it in a structure at offset 4486, calculates another value using d0 and d1, stores it in a structure at offset 4488, stores d2 in a structure at offset 4490, and calls a sub-function via a pointer in a1.
- `load_table_entry_to_structure`: Calculates an index using d0, loads a 32-bit value from a table into a structure at offset 4492, stores d2 in a structure at offset 4496, and calls a sub-function via a pointer in a1.
- `set_value_and_branch`: Loads a pointer from a structure into a1, writes a constant to a structure, and then branches to another routine.
- `set_value_and_call_indirect`: Sets a 16-bit value in a structure at offset 4758 to 2, performs a trap, loads a pointer from a structure, calls a sub-function via an offset of 8, and performs a trap before returning.
- `execute_table_function_chain`: Executes a sequence of functions retrieved from a table or structure, updates several pointers in a structure, modifies certain flags in a structure, and performs a loop that copies data from a list of pointers until a zero-word is encountered.
- `set_hardware_registers`: Writes a value to the CIAA_CRB register and writes a byte to a global variable.
- `execute_indirect_function_pointer`: Loads a pointer from a table located at an offset from a global pointer, retrieves a function offset or jump table entry, and calls the resulting address via an indirect jump.
- `execute_indirect_function_pointer_2`: Loads a pointer from a table located at an offset of 4534 from a base address in a5, then calls a function pointer found at an offset of 108 from that loaded pointer, and returns.
- `increment_global_variable`: Increments a 16-bit global variable and returns via rte.
- `indirect_jump_stub`: Jump stub: passes control to an address held in or computed from register a1, so the target is only known at run time.
- `execute_indirect_function`: Calls a function pointer located at an offset of 8 bytes within a structure pointed to by a pointer at offset 4506 of a structure in a5, sets bit 0 of a byte at offset 15 of that structure, and if a byte at offset 14 of that structure is not less than a byte at offset 4514 of a5, writes 9 to a global variable, then returns.
- `execute_function_with_stack_setup`: Sets the unable-to-be-determined stack pointer to a1, pushes a1 to the stack, and if a pointer at offset 4510 of a5 does not equal a6, loads a pointer from offset 84 of a6 into a1, restores the stack pointer, restores the status register, calls a function in a0, and then performs further stack and trap operations before returning.
- `swap_linked_list_node`: Checks a flag in a structure, and if certain conditions are met, calls another routine via a function pointer, otherwise it swaps two nodes in a linked list, updates a pointer in a global structure, and restores the stack and registers before returning via a trap.
- `traverse_linked_list_and_swap_nodes`: Traverses a linked list until a certain condition is met, calls a routine via a function pointer found in the list, swaps the current and next node pointers, updates a global pointer to the new current node, and restores the stack or or certain flags before returning via a trap.
- `execute_indirect_function_2`: Loads a pointer from a table using an offset from a structure in a5, calls the address via a trap, and if the result in d0 is not zero, increments a value at an offset of 4 in a structure pointed to by a0 and jumps to another instruction.
- `call_indirect_function`: Loads a pointer from a table, uses an offset from a structure in a6, calls the resulting address, and executes a trap.
- `execute_table_function`: Loads a pointer from a structure in a5, uses an offset from a table, calls the resulting address, and then branches to a following instruction.
- `set_hardware_registers_2`: Sets several hardware registers, including a custom register and a DMA or interrupt register, and modifies a global variable or a pointer-based value.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `update_global_pointer`: Calls a function via a pointer found in a structure, updates a global pointer with a value from a register, and modifies a bit in a hardware register.
- `increment_and_call_function`: Increments a global variable and calls a function using a pointer and an offset from a structure in a6.
- `execute_trap_function`: Executes a trap, calls a function using a pointer and an offset from a structure, and then executes another trap.
- `execute_indirect_function_pointer_3`: Loads a pointer from a table located at an offset from a global pointer, retrieves a function offset from that pointer's target, and calls the resulting address via a jump, wrapping the execution between two traps.
- `load_value_and_call`: Loads a word from an offset of 4 in a structure in a5 into d0 and calls another routine.
- `fill_table_with_lcr`: Fills a table with values calculated using a multiplication and division loop, then updates a global variable with the last calculated value.
- `update_table_with_lcr_sum`: Modifies bytes in a table, calculates a new value using multiplication and division, and stores a sum of this value and values from another table into a third table.
- `copy_and_update_buffer_2`: Recursively traverses a linked list of pointers, and if a certain condition is met, copies 142 bytes from a source buffer to a destination, then performs conditional pointer-based copies and arithmetic updates to a destination buffer based on values in a source.
- `copy_and_update_buffer`: Recursively traverses a linked list of pointers, and if a certain condition is met, copies 142 bytes from a source buffer to a destination buffer while adding an offset to each byte, then updates a pointer in a structure and clears a flag.
- `process_table_updates`: Iterates through a table, performing conditional pointer-based copies and arithmetic updates to a destination buffer based on values in a source table.
- `update_list_nodes_2`: Copies a list of pointers and associated data, including a counter and a status byte, to a new structure, and recursively processes a list of pointers if a null pointer is encountered.
- `recursively_copy_list_node`: If a pointer or its target is null, it calls itself recursively, otherwise it calls another routine and copies a 32-bit value and a 32-bit pointer from a source structure to a destination structure.
- `calculate_jump_address`: Saves registers to the stack, calculates a jump target based on a bitmask of a value at an offset from a pointer in a0, and jumps to that address.
- `set_address_and_branch`: Sets a pointer in a6 to the value in a0 and branches to a relative address.
- `decrement_counter_and_call_indirect`: Checks a flag in a global variable and, if clear, calls an indirect routine via a table entry and decrements a short counter in the same structure.
- `manage_linked_list_nodes_2`: Checks a flag in a structure; if clear, it calls a trap, calls an indirect routine, decrements a counter, sets a flag, calls a trap, decrements another counter, and if a borrow occurs, calls another routine before returning.
- `initialize_self_copying_data_block`: Performs a series of bitwise OR operations on a register, modifies certain hardware registers including the disablement of interrupts, and copies a block of 6 words (starting from the routine's own address) to a destination table.
- `update_interrupt_mask_with_xor`: Sets the interrupt enable register to a specific value, then reads a byte from a global variable, or-ing it with a value, and writes a modified version back to a global variable offset.
- `execute_conditional_handlers`: Checks certain bits in the interrupt or status register, and if certain conditions or bit-shifts result in negative values, calls a sequence of different handlers stored in a table.
- `execute_cascaded_handlers_on_bit_set`: Sets a hardware register, or-ing a byte from a global variable, and if certain bits in a table are set, it executes a chain of up to five different handlers.
- `configure_interrupt_mask`: Reads a bitmask from the custom chip, masks certain bits, and writes the result back to a custom chip register, then conditionally calls other routines based on the sign or overflow of the shifted bitmask or subsequent arithmetic, otherwise it returns.
- `call_indirect_address`: Loads an address from an offset of 5458 in a5 into a0, calls that address, and jumps to another instruction.
- `call_indirect_address_alt`: Loads an address from an offset of 5462 in a5 into a0, calls that address, and jumps to another instruction.
- `call_indirect_address_alt2`: Loads an address from an offset of 5466 in a5 into a0, calls that address, and jumps to another instruction.
- `increment_global_and_update_interrupts`: Increments a global variable and writes its value to the interrupt enable register.
- `decrement_global_counter`: Decrements a global counter and, if the result is zero, sets the top bit of that global and writes it to the interrupt enable register.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `traverse_signed_table`: Walks a linked list of signed table entries that starts at a fixed address, advancing by an offset stored with each entry.
- `iterate_table_and_call`: Walks a short table in a record until a negative sentinel, calling another routine for each non-negative entry whose referenced value is also non-negative.
- `store_function_result`: Calls another routine, stores the returned value into a global variable, and writes a byte to a global variable offset.
- `update_engine_state_and_pointers`: Updates several pointers and status flags in a structure, clears a range of memory using a loop, and executes a sequence of callbacks and conditional logic checks while updating or clearing various status bits and pointers in the structure.
- `update_function_pointer_and_state_2`: Calls a routine, checks a word in a structure, and if non-zero calls a function pointer, then loops back to the start of the routine.
- `execute_sequence_and_loop`: Calls a sequence of three routines and loops if a certain 16-bit value at an offset in a5 is not zero.
- `call_and_branch`: Calls a routine and then branches to another routine.
- `process_conditional_logic`: Executes a sequence of routines that modify a global byte and a counter, and potentially calls a routine via a pointer in a structure, while also handling a conditional branch based on a bit test.
- `loop_until_condition_met`: Calls another routine, then checks a global variable, and if it is non-zero, calls the same routine again before returning.
- `modify_buffer_and_pointers`: Performs a series of routine calls and hardware register setups, and updates certain offsets in a structure or a pointer-based buffer based on a loop counter.
- `update_pointer_and_value`: Checks if a 16-bit value at a certain offset in a structure is zero or negative, and if so, compares a byte from a pointer in a3 to a byte in a2.
- `check_and_call_routine`: Checks if a 16-bit value at an offset in a structure is non-zero and if adding 12 to it results in a byte that does not match a byte in another structure, then calls a routine.
- `process_complex_logic`: Performs a series of checks on a 16-bit value, potentially decrements a value in a structure, calls two routines, and if certain conditions involving a table and bit manipulation are met, updates a structure with new pointers and bitflags.
- `execute_multiple_callbacks`: Performs several loops that copy or calculate pointer-based data into a destination buffer, and then calls several different routines using pointers retrieved from a structure.
- `calculate_and_set_pointers`: Checks a value in a structure and, if certain conditions are met, calls a routine, calculates a new pointer, and updates several offsets in a structure with new pointer or bit-shifted values.
- `set_dma_control`: Writes a 16-bit value to the DMA control register.
- `set_structure_values`: Calls another routine, then writes a constant word to an offset of 150 in a1, writes a constant word to an offset of 260 in a1, and clears a sequence of four 16-bit values at offsets 418, 426, 450, and 442 in a1.
- `update_sprite_pointer_table`: Checks a bit in a structure, then loads either a routine address or a pointer to a table into a register, and copies a sequence of longwords from that table into the custom hardware sprite pointer registers.
- `apply_constant_or_logic`: Performs a series of or-with-immediate instructions on a register.
- `update_callback_sequence_and_state`: Executes a sequence of callbacks using pointers and offsets from a structure, while updating several global state pointers and counters with the return values or or new pointers.
- `execute_sequence_of_callbacks`: Calls a sequence of routines using pointers from a structure and updates a bit in a global variable based on a returned value.
- `call_and_store_result`: Calls a routine using a pointer from a structure and stores the returned value into a global variable.
- `execute_chained_callbacks`: Executes a sequence of routines that use pointers or values from a structure, updating several different pointers in a structure with the results of those calls.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `execute_indirect_call`: Loads a pointer from a structure offset, retrieves a function address from a secondary pointer, and calls that function.
- `update_scaled_value`: Copies a longword from one structure offset to another, calls a routine, and then calculates a bit-shifted, scaled value or a result of a multiplication into a structure offset.
- `execute_indirect_calls`: Performs several indirect calls, checks a value in a structure, and calls a routine if a condition is met.
- `update_status_and_call`: Calls several routines, decrements a value in a structure, calls two more routines, and sets bit 4 of a byte in a structure before returning.
- `update_status_and_call_functions`: Calls another routine, decrements a 16-bit value at offset 598 in a5, calls a routine at a longword address, calls another routine, calls a third routine, calls a fourth routine, and sets bit 4 of a byte at offset 716 in a5.
- `execute_loop_and_set_status`: Performs a loop that calls a routine, and sets a certain value in a structure before checking a condition and returning.
- `modify_bitset_flags`: Iterates through a list of pointers, checking specific bits in a target buffer and setting or clearing bits at certain offsets based on a control byte.
- `increment_pointer`: Increments a pointer in a register by one and returns.
- `update_structure_and_call`: Decrements a value in a structure, potentially performing a loop with bcd-adjusted arithmetic, updates structure members, and calls a routine via a pointer in a structure.
- `tiny_ori`: Small routine of 1 instruction that merges 16 into d0.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 15 into d0.
- `set_global_values`: Writes a constant to a global variable, copies a word from a pointer, and updates a global pointer with the incremented address.
- `populate_global_table`: Iterates through a loop, calling a routine and updating a global pointer with values from a table, while checking bit certain flags.
- `process_table_recursively`: Saves registers, shifts d0 left by 2 and writes it to an offset of 216 in a5, loads a pointer from a global address, calls another routine, and then performs a complex loop that updates a pointer, calls a routine, and writes values from a table to memory while checking flags and updating a counter.
- `update_table_recursively`: If a value at a pointer in a0 is greater than 1024, it replaces that value with a value from a table, calls another routine, and then recursively calls itself while potentially searching for a certain byte and clearing a bit in a register.
- `search_table_for_value`: Searches a table using a pointer, and if a value is found that is greater than a certain amount, sets certain bits in a register.
- `set_flag_on_condition`: Checks if a word at a certain offset in a structure is non-zero, and if so, sets a word at another structure offset to one.
- `process_structure_and_loop`: Executes a loop calling a routine, modifies a structure member based on a clamped calculation, and calls a routine via a pointer in a structure.
- `initialize_memory_buffer`: Clears a block of memory starting at a pointer from a global variable, then populates a second block with a pattern of shifted values from a table, and finally updates a global counter.
- `loop_on_value_range`: Checks if a value in a structure is within a certain range and calls a routine repeatedly if so, otherwise it calls a routine and updates structure members.
- `update_structure_pointer`: Updates a pointer in a structure and then executes a loop that calls other routines and updates a counter.
- `process_pattern_buffer`: Iterates through a loop that calls several routines and performs bitwise or arithmetic operations on values or pointers derived from a structure.
- `set_pointer_bit`: Sets bit 0 of a value at an offset in a global structure and, if bit 0 was already set, updates a pointer in that structure to point to the next element in a list.
- `clear_pointer_bit`: Clears bit 0 of a value at an offset in a global structure and, if bit 0 was previously clear, updates a pointer in that structure to point to an offset in a list.
- `copy_pattern_to_buffer`: Copies bytes from a source table, where the source address is calculated using a multiplier, to a destination buffer in a global structure.
- `update_pointer_list`: Calculates an offset, iterates through a list, and performs bitwise operations on a target buffer using values from a source table, while also calling a routine and updating structure members.
- `traverse_and_update_pointers`: Checks a value in a structure and, if it meets certain criteria, updates pointers and counters, potentially calling a sequence of routines and traps within a loop.
- `evaluate_conditional_logic`: Saves registers, reads two words from a pointer in a1, and if not zero, checks certain bits in a structure at a fixed address, potentially calling several different routines or updating a pointer or a value at a fixed address based on bit tests or masking d0.
- `scatter_bytes_to_buffer`: Copies bytes from a source pointer to a destination buffer at a fixed stride of 40 or 48 bytes.
- `copy_data_block`: Copies 8 dwords from a source pointer in a3 to a destination pointer at a fixed address, then returns.
- `apply_bitmask_to_code`: Iterates 8 times, reading bytes from a pointer in a3 and performing bitwise AND and OR operations on a target memory area, then returns.
- `apply_bitmask_to_code_limited`: Iterates 8 times, reading a byte from a pointer in a3 and performing a bitwise AND with a byte at a target memory area, then returns.
- `zero_data_area`: Contains a series of instructions that perform a logical OR with zero on register d0, effectively acting as a zero-filled data area.
- `advance_pointer_forward`: Calculates a value from a structure member, and if it meets certain criteria, iterates through a list of pointers, updating a pointer in a structure if a certain byte-value is met, and calls another routine.
- `advance_pointer_backward`: Calculates a value from a structure member, and if it meets certain criteria, iterates backwards through a list of pointers, updating a pointer in a structure if a certain byte-value is met, and calls another or another routine.
- `traverse_pointer_list`: Iterates through a list of pointers, and if a certain byte at an offset of 1 from the pointer is negative, calls another routine.
- `update_structure_state`: Performs a system trap, then checks a bit in a global structure to decide whether to update a pointer or modify a structure member with a calculated offset and bit-manipulated values.
- `populate_pointer_list`: Checks a value in a global structure and, if certain conditions are met, populates a list of pointers with calculated addresses and updates a global counter.
- `call_two_subroutines_2`: Calls another routine and then calls another routine, then returns.
- `configure_blt_params`: Configures certain hardware registers, including a block transfer or or-mask, using values derived from a global variable and a calculated offset, then loops to update a register.
- `load_address_and_branch`: Loads an address from a structure offset into a1 and branches to another routine.
- `setup_blt_transfer`: Sets up hardware registers for a block transfer, including setting a source and destination, and loops to set a register to a constant value.
- `offset_address_and_branch`: Loads an address from a structure offset into a1, adds a constant offset to a1, and branches to another routine.
- `decrement_and_execute_callback`: Decrements a 16-bit value at a certain offset in a structure, and if it reaches zero, calls a function pointer from a table, otherwise it clears bit 4 or bit 7 of a status byte and potentially calls another function.
- `clear_d0_and_call`: Calls another routine, clears d0, and then returns.
- `initialize_memory_blocks`: Uses a loop to clear or initialize several 32-bit or 16-bit memory locations using pointers or a counter.
- `populate_pointer_table`: Checks a flag in a structure and, if set, iterates through two source buffers to build a table of pointers and associated constant values.
- `compute_masked_indices`: Performs a series of arithmetic and bitwise operations involving multiplication and masking, storing a result in a register.
- `set_pointer_table`: Sets a series of pointers in a structure to point to a sequence of labels or addresses.
- `update_state_flags`: Checks bits in a structure at offset 1814 in a5, and if certain conditions are met, calls a routine, modifies bits in a structure at offset 716 in a5, or calls a function pointer and clears or sets bits in a structure at offset 716 in a5.
- `update_function_pointer_and_state`: Copies a word from a structure, calls a function pointer retrieved from a table, and updates a global variable or a status bit and counter in a structure based on the return value and certain conditions.
- `execute_and_negate_result`: Calls a function pointer retrieved from a structure and returns the negation of its return value.
- `initialize_word_list_pointers`: Sets up a pointer to a word list, calls a routine to populate it, and then calls two other routines using pointers and offsets found in a structure.
- `swap_pointers`: Swaps two 32-bit values located at certain offsets in a structure.
- `fill_memory_with_pattern`: Fills a range of memory with a sequence of 16-bit values that increment by a constant amount.
- `update_and_call_function`: Copies a value from a structure, calls a function, and then writes a constant value to a memory location pointed to by a register.
- `copy_word_list_with_early_exit`: Iterates over a word list terminated by -1; for each entry it copies a word from the list into a destination area indexed by the entry, aborting if the entry matches in d0w.
- `update_structure_pointers`: Copies a 32-bit value from a structure to a new location and updates two pointers in that structure to point to that new location.
- `update_list_nodes`: Iterates through a list of nodes, updating a pointer to a new address and decrementing a counter or value in each node, then updates the list head.
- `set_four_pointers`: Stores four specific long-word constants into a structure at offsets relative to a pointer in a6, then calls another routine.
- `execute_complex_setup`: Increments a value, stores a pointer, calls a routine, updates several structure members, copies a block of bytes from a global source, and performs an indirect call through a table-based address.
- `store_pointer_after_call`: Calls a routine and stores the returned pointer into a structure at an offset relative to a6.
- `increment_byte_in_structure`: Increments a byte at an offset of 56 in a structure pointed to by a6 and then returns.
- `decrement_byte_in_structure`: Decrements a byte at an offset of 56 in a structure pointed to by a6 and then returns.
- `indirect_table_call`: The routine loads a pointer from a global variable, reads a short offset from that structure, adds the offset to the pointer and calls the resulting address.
- `call_subroutine_with_a0`: Moves the value in a0 to a0 and calls another routine.
- `update_global_pointer_logic`: Checks a byte in a structure; if certain conditions are met, it calculates a new pointer, updates a global pointer, and sets a pointer to a custom hardware register address.
- `initialize_structure_and_hardware`: Increments a value, copies a block of bytes, performs an indirect call, updates a structure with a new pointer, sets a bit in a structure, and clears a custom hardware register or a value at a specific offset.
- `update_structure_fields`: Loads a pointer from a structure, adds a byte from d0 to an offset of 16 and an offset of 74 of that pointer, calls another routine, and then returns.
- `invoke_callback_with_optional_extra`: Checks if a pointer in a structure is non-null, if so calls a callback through a table using an offset, then calls another callback through a table passing a structure pointer and a captured value.
- `decrement_and_dispatch_callback`: Decrements a counter in a structure pointed to by a global variable and then invokes a callback stored in a table via an index.
- `decrement_and_trap`: Calls a routine, then decrements a 16-bit value at an offset of 4 from a pointer loaded from a structure, and triggers a trap if a borrow occurs.
- `initialize_structure_and_loop`: Sets certain fields in a structure, calls a routine, and then uses a loop to write a sequence of specific values into a buffer, before calling a callback and clearing several fields in a structure.
- `conditional_callback_dispatch`: Compares a short value in d1 with a short stored at an offset from a pointer and, if they differ, calls a routine obtained by indexing a table through a pointer.
- `process_message_queue`: Walks a table of 32-bit message entries pointed to by a pointer in a0, using a count from a6, and processes each non-zero entry.
- `apply_delta_to_entity_fields`: The routine adds a signed short delta to three 16-bit fields at fixed offsets from a base pointer and adds the same delta to a series of bytes spaced by a fixed stride, the count being taken from a 16-bit field in the structure.
- `store_callback_and_dispatch`: The routine stores an argument pointer and a word into fields of a structure pointed to by a register, then checks a flag byte in that structure.
- `set_pointer_to_constant`: Calls a routine at address zero and then writes a constant value to a pointer located at an offset from a pointer in a structure.
- `increment_and_set_pointer`: Increments a register, calls a routine at address zero, and then writes a constant value to a pointer located at an unable-to-determine offset from a pointer in a structure.
- `read_offset_from_pointer`: Reads a 32-bit value from ten bytes after the address held in a pointer register and returns that value minus one.
- `call_indirect_table_entries`: The routine loads a pointer from a global variable and calls an indirect routine selected by a short offset from a table entry, then loads a second pointer from the same global variable and calls another indirect routine selected by a short offset.
- `initialize_structure_and_dispatch`: The routine copies several values from a source pointer into a structure, then calculates a new value using multiplication and division, and finally calls a routine via a function pointer and a table offset.
- `update_pseudo_random_state`: The routine updates several short fields in a structure pointed to by A6 by adding input values and mixing a short field with a mask to produce a pseudo-random index.
- `call_indirect_via_table`: The routine reads a short value from an offset within a pointer in a register, adds it to that pointer, and calls the resulting address as code.
- `get_short_from_structure`: Loads a word from an offset of 44 from a pointer in a structure into a register and returns.
- `return_state_block_pointer`: Returns a 64-bit value constructed from four 16-bit words located at specific offsets within a global state block.
- `invoke_indirect_through_table`: The routine loads a pointer from a global variable, reads a short offset from that pointer, adds it to the pointer and calls the resulting address.
- `populate_buffer_with_addresses`: Calls two routines, then loops to write a sequence of incrementing addresses (starting at a large value and increasing by a fixed amount) into a destination buffer, finally writing a constant address and calling another routine.
- `call_function_and_load_pointer`: Calls a function at address zero and then loads a pointer from a global variable into a1.
- `execute_initial_routine`: Calls a routine and then calls another routine before returning.
- `copy_word_table`: The routine copies a sequence of 16-bit words from a source pointer to a destination buffer.
- `copy_words_if_not_null`: Calls a function, then if the return value is not null, copies a sequence of 16-bit words from a source pointer to a destination buffer, advancing both pointers until the count is exhausted.
- `set_byte_after_call`: Calls a function and then writes the byte value 4 to an offset of 28 in a structure pointed to by a6.
- `tiny_movea_rts`: Small routine of 2 instructions that copies a6 to a1, then returns.
- `clear_and_set_values`: Calls a function, clears a 32-bit value at offset 24 of a structure in a6, and writes the byte 2 to offset 28 of that structure.
- `update_pointer_if_not_zero`: Checks if a 32-bit value at offset 24 of a structure in a6 is zero; if not, it returns.
- `parse_string_and_update_state`: Parses a string, potentially calling a function if a backslash is encountered, and updates a pointer or a counter in a structure.
- `dispatch_pointer_and_branch`: If d0 is not zero, it saves a6 to the stack, loads a pointer from an offset of a6, loads a pointer from that pointer, loads a word from it, and then branches to another routine.
- `dispatch_table_entry`: Tests d0, and if not zero, saves a6 to the stack, loads a pointer from an offset of a6, loads a pointer from that pointer, loads a word from an offset of that pointer, calls a routine using that word as an offset, restores a6, and branches to another routine.
- `dispatch_function_with_conditional_setup`: Saves a register to the stack, checks a condition, and if not met, updates registers and calls a routine using a table offset, otherwise it updates a register and branches to another routine.
- `invoke_dispatch_functions_twice`: Calls a function via a dispatch table using an offset from a pointer in d1.
- `store_word_triple`: Copies three consecutive 16-bit values from a source pointer into three consecutive 16-bit fields of a record at offset 30.
- `clear_record_fields`: Clears three 16-bit fields at offset 30 of the record in a6 and calls a routine at address zero.
- `read_long_field_at_offset`: Returns the 32-bit value stored at offset 30 in the record passed through an address register.
- `clear_long_fields`: Clears four 32-bit fields starting at offset 48 of the record in a6 and calls a routine at address zero.
- `test_bit_from_shifted_index`: Copies a word from one register to another, shifts the second register right by three, and tests a bit in a memory location at an offset of forty-eight plus the shifted value from a pointer in a6 before returning.
- `update_bit_flags`: Sets a byte at offset 48 of the record, then loops through a list of pointers to flip a bit in a bitset at offset 32 of the record based on a value, and calls a routine at address zero.
- `set_range_pointers`: Checks if a byte in d1 is zero, then sets two 32-bit pointers at offset 20 and 24 of the record to certain values before calling a routine at address zero.
- `execute_conditional_function_chain`: Checks a value in a structure, and if it equals one, calls a sequence of up to four different routines using offsets from a structure, potentially updating a counter and calling a final routine if certain return values or conditions are met.
- `decrement_and_call`: Decrements a 16-bit value at an offset from a pointer in a0, checks for a signed overflow, calls another routine, and decrements a 16-bit counter at offset 30 of the record.
- `increment_counters_and_call`: Stores a byte at offset 36 of the record, increments a 16-bit value at offset 4 of a pointer in a0, updates a pointer in the record from a table, increments a 16-bit value at offset 4 of that pointer, clears a 16-bit counter at offset 30, and calls another routine.
- `compare_byte_to_stored_flag`: Compares the byte in a data register with the flag byte stored at offset 36 in the record.
- `store_word_and_call`: Stores a 16-bit value from d0 into a field at offset 38 of the structure in a6, then calls another routine and returns.
- `populate_structure_from_source`: Saves registers, calls two routines, and if certain conditions are met, copies several bytes and words from a source structure to a destination structure, performs a calculation involving multiplication and division, and calls two more routines.
- `call_two_subroutines`: Saves d0 to the stack, calls two different routines using a pointer in a3, restores d0 and any modified registers, then returns.
- `clear_record_fields_2`: Clears a 16-bit field at offset 28 of the structure in a6 and clears a byte at offset 26 of the structure in a6, then returns.
- `set_flag_and_increment_stage`: Sets the flag byte at offset 26 to all ones, increments the 16-bit stage word at offset 28, and returns zero.
- `store_long_and_word_fields`: Stores a 32-bit value and a 16-bit value into two fields of the record at offsets 10 and 8, and clears the 16-bit field at offset 6.
- `execute_looping_function_calls`: Saves a register, checks a condition, and if met, executes a loop that calls a sequence of routines using pointers from a structure, updates a value in a structure, and increments a counter.
- `execute_loop_of_calls`: Executes a loop that calls two different routines via pointers found in a structure, decrements a counter in a structure, and increments a field at offset 6 of a structure before returning.
- `call_indirect_via_offset`: Calls a routine, then calls another routine using a pointer and a 16-bit offset found in a structure in a4, then returns.
- `call_indirect_and_store_result`: The routine calls another routine through an indirect pointer formed from a global variable and a short offset, then stores the returned value from D1 into a location addressed by another global variable plus a constant offset.
- `set_path_form_status`: Sets a status value of zero, two or three in the record at offset 18, depending on the length of the text at the pointer in the record and whether its second character is a colon and its third character is nonzero.
- `call_subroutine_from_pointer`: Loads a 32-bit pointer from offset 92 of the structure in a6 into d0, calls the routine at that address, and returns.
- `initialize_structure_and_call_subroutine`: Stores several values into a structure, calls a subroutine, updates a pointer and a value in the structure, clears a range of 15 words, and calls two trap handlers.
- `match_text_prefix_to_stored`: Compares the zero-terminated text at an address register with the stored text at offset 120 of the record.
- `execute_conditional_trap_logic`: Calls a routine via a table, or-registers the result into a global variable, and if a certain bitmask is met, clears a value and compares a byte before calling two trap handlers.
- `update_value_from_offset`: Calls a trap, adds a value from an offset in a structure to d0, stores the result in a structure, and calls two trap handlers.
- `reset_and_query_status`: Clears a 32-bit value at offset 108 of the structure in a6, sets bit 2 of a byte at offset 15 of the structure, and calls a routine via a trap if a certain pointer is non-zero, followed by a conditional call to another routine via a trap before returning.
- `toggle_bit_and_clear_value`: Calls a trap, and if a bitmask check passes, copies a value, clears a long, sets a long to all ones, calls a trap, and then clears and eor-updates a bitmask.
- `call_nested_routine`: Calls a routine at address zero, then calls another routine using an offset from a pointer in a6.
- `decrement_counter_and_call`: The routine reads a short value from an offset in a structure pointed to by a register, decrements it, writes it back, and if the result is zero it calls a routine whose address is taken from an offset in another structure pointed to by a register.
- `clear_counter_and_call_zero`: Calls a routine at address zero and clears a short value at an offset in a6.
- `increment_flag_and_call`: The routine checks a short flag at an offset from a pointer in register A6.
- `follow_first_node_entry_if_linked`: Follows the first link from the record and, when that node has a successor, calls the entry routine stored at offset 16 of the node.
- `call_stored_entry_point`: Calls the routine whose entry address is stored at offset 20 in the record, passing no arguments.
- `store_byte_and_call_indirect`: The routine stores a byte from a register and a pointer from a register into a structure pointed to by a register, then calls another routine indirectly via a table indexed by a short value.
- `call_stored_entry_at`: Calls the routine whose entry address is stored at offset 24 in the record, passing no arguments.
- `insert_node_by_key_byte`: Increments the counter of the node passed in an address register, walks the list in the record until it finds a node whose key byte at offset 14 is lower than the new node's key, and splices the new node after it.
- `insert_node_at_front`: Increments the counter of the node in an address register and inserts it at the front of the list headed in this record, rewriting the neighbour links.
- `increment_node_and_prepend_to_list`: Increments a 16-bit counter at offset 4 of a structure pointed to by a0, then updates the pointers of the new node and the existing list head to insert the new node at the front of the list.
- `append_node_at_stored_tail`: Increments the counter of the node in an address register, links it after the tail node stored at offset 14 in this record, and makes it the new tail.
- `swap_linked_list_nodes`: The routine swaps the next and previous pointers of two linked list nodes referenced through a pointer in A0 and a global variable.
- `call_method_on_each_node`: Walks a linked list starting at the record's first link and calls one method, selected by an offset in a data register, on every node that has a successor.
- `setup_dispatch_pointer`: Saves registers to the stack, loads a pointer from a structure, clears a register, and branches to another instruction.
- `loop_dispatch_until_nonzero`: Repeatedly calls a function via a dispatch table using an offset from a pointer.
- `reset_linked_list_head`: It then writes the address of the first node back into the head pointer and stores the base address of the list structure into another field of the structure.
- `load_next_node_pointer`: Loads a pointer from a structure into a register and branches to another instruction.
- `decrement_linked_counter_chain`: The routine walks a singly linked list via a pointer in d0 and a node pointer in a0.
- `initialize_list_head`: Points the list head link and the tail link of the record at the record's own fields and clears the 32-bit field at offset 10.
- `update_linked_list_pointers`: Increments a counter in a structure and calls a dispatch function.
- `unlink_node_and_decrement`: Unlinks the node in an address register by joining its two neighbours, clears its two link fields, and decrements its 16-bit counter with signed overflow trapping.
- `update_node_links`: Loads a pointer from a structure, updates a link field in a target node to point to that target node, and clears the link field in the target node.
- `tiny_movea_rts_2`: Small routine of 2 instructions that copies d0 to a0, then returns.
- `store_byte_to_structure`: Stores a byte from d0 into a field at offset 14 of the structure pointed to by a6.
- `unlink_double_linked_node`: Joins the two neighbouring nodes referenced at offsets 6 and 10 of the record so that each points past this node, then clears those two references in the record.
- `process_string_and_update_offset`: If d0 is negative, it searches for a null terminator in a string, calculates a new value, and calls a function.
- `load_pointer_from_stack`: Loads a 32-bit value from an offset of 6 in a structure pointed to by a6 into a0 and returns.
- `shift_and_call_with_store`: Stores d0 into a structure at offset 10, shifts d0 left by 2, calls another routine, and stores the new a0 back into a structure at offset 6 before returning.
- `multiply_and_call_with_store`: Stores d0 into a structure at offset 10, doubles d0, calls another routine, and stores the new a0 back into a structure at offset 6 before returning.
- `call_with_store`: Stores d0 into a structure at offset 10, calls another routine, and stores the new a0 back into a structure at offset 6 before returning.
- `call_with_pointer_setup`: Loads a 32-bit value from a structure at offset 6 into a0 and calls another routine.
- `load_value_to_register`: Loads a 32-bit value from a structure at offset 10 into a0 and loads a 32-bit value from a structure at offset 10 into d0 before returning.
- `store_node_fields_with_flag`: Stores two 32-bit values and a 16-bit value into the record at offsets 10, 14 and 18, then clears the flag byte at offset 20.
- `execute_table_and_branch`: Saves registers, checks a bit in a structure, calls a routine or a function pointer, and if a pointer is non-zero, copies a value from a pointer to another location, increments a value, and then enters a loop that copies bytes from a buffer and performs an indirect jump through a table.
- `unroll_and_jump_to_table`: Loads a byte from a buffer, stores a 32-bit value from a buffer into a table at an offset, loads another byte from the buffer, and jumps to a location calculated using a register and a byte.
- `unroll_and_jump_to_table_2`: Loads a byte from a buffer, stores a 16-bit value from a buffer into a table at an offset, loads another byte from a buffer, and jumps to a location using a table or offset.
- `dispatch_by_table_or_pointer`: Checks a pointer; if non-zero it performs an indirect jump through a table indexed by a byte from the pointer, otherwise it calls a routine via a pointer stored in a pointer plus an offset.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `compare_and_zero_pointer`: Loads a pointer from a table, loads another pointer from a table, compares it to a constant, and if not equal, clears the pointer before returning.

### Paradroid 90 (demo) (1990)(Hewson)[h Black Monks].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
