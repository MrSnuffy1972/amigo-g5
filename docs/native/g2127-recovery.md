# Game 2127 (killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02127). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (939536 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 12 raw-word lines |
| Functions named and described | pass, 188 of 188 named (0 author symbols), 188 of 188 described (5 by rule, 178 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1283 of 1283 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3434 of 3658 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 24022 |
| code | 11586 |
| data | 865522 |
| padding | 6 |
| unused | 38400 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company].adf:bootblock | 1 | 249 | 1 |
| logged disk load from Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company].adf boot-block load | 1 | 85 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2582 | 141 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 415 | 36 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 310 | 5 |

## Cases handled

- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company].adf: custom-format (non-DOS) disk, 1683 non-empty blocks: matched against track-loaded segments
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company].adf: the boot block reads 6144 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 14 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1109 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 139 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 21 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company].adf: 1 logged loads cover 6144 bytes of the image
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company].adf: blocks 22-1032 (bytes 11264-528896) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.30)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company].adf: blocks 1540-1549 (bytes 788480-793600) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.32)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company].adf: blocks 1551-1560 (bytes 794112-799232) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.64)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company].adf: blocks 1562-1571 (bytes 799744-804864) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.62)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company].adf: blocks 1573-1582 (bytes 805376-810496) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.64)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company].adf: blocks 1584-1593 (bytes 811008-816128) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.63)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company].adf: blocks 1595-1604 (bytes 816640-821760) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.63)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company].adf: blocks 1606-1615 (bytes 822272-827392) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.62)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company].adf: blocks 1617-1626 (bytes 827904-833024) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.64)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company].adf: blocks 1628-1637 (bytes 833536-838656) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.62)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company].adf: blocks 1639-1648 (bytes 839168-844288) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.63)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company].adf: blocks 1650-1659 (bytes 844800-849920) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.62)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company].adf: blocks 1661-1670 (bytes 850432-855552) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.64)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company].adf: blocks 1672-1681 (bytes 856064-861184) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.63)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company].adf: blocks 1683-1692 (bytes 861696-866816) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.64)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company].adf: blocks 1694-1703 (bytes 867328-872448) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.62)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company].adf: blocks 1705-1714 (bytes 872960-878080) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.63)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company].adf: blocks 1716-1725 (bytes 878592-883712) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.62)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company].adf: blocks 1727-1736 (bytes 884224-889344) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.63)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company].adf: blocks 1738-1747 (bytes 889856-894976) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.63)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company].adf: blocks 1749-1758 (bytes 895488-900608) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.64)
- 46 packed track region(s), 235520 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `update_button_state` (runtime segment an address): Checks the status of a bit in a register and updates a global variable to reflect the change. Kind: disk hardware/track check. Pass path: the branch where bVar1 != 0 (the condition that skips the loop).
- `update_dma_hardware_parameters` (runtime segment an address): Iterates through a table of data to update custom hardware registers and DMA control settings. Kind: disk hardware/track validation. Pass path: the loop completes and returns via the uVar6 == 4 branch after processing the sequence.

## Functions


### Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company].adf:bootblock

- `initialize_boot_parameters`: Sets several values in a structure, calls a function via a pointer, updates a global variable, and executes a software interrupt.

### logged disk load from Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company].adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `nop_padding_sequence`: A sequence of three no-operation instructions used as padding within the executable segment.
- `execute_recursive_calls`: calls itself twice before returning.
- `push_registers_to_stack`: pushes several processor registers onto the stack.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_interrupt_status`: Reads bits from hardware interrupt registers and conditionally executes a function pointer stored in a global variable based on the result.
- `call_function_pointer_repeatedly`: Calls a function through a pointer stored in a global variable multiple times, performing multiplications and conditional re-calls based on the returned values.
- `execute_conditional_calls`: Repeatedly calls a routine through a pointer and performs arithmetic on the result to determine if subsequent calls are necessary.
- `call_and_multiply_repeatedly`: Calls a function through a pointer and performs multiple multiplications and conditional re-calls based on the returned value.
- `call_and_multiply_conditionally`: Calls a function through a pointer and performs conditional multiplications and re-calls based on the result.
- `call_and_check_result`: Calls a function through a pointer and performs a conditional re-call if the result meets a specific criteria.
- `call_function_pointer`: Calls a function through a pointer stored in a global variable.
- `insert_into_linked_list`: Inserts a new node into a linked list by traversing it until a specific condition is met.
- `traverse_and_call_list`: Iterates through a linked list and executes a function pointer contained within each node.
- `set_non_zero_global`: Stores a bitwise masked value into a global variable, ensuring the value is at least one.
- `clear_global_variable`: Performs a bitwise XOR on a global variable with itself to set it to zero.
- `increment_and_call_list`: Increments a global variable, calls another routine, and then sets the global variable to a specific value.
- `toggle_interrupt_enable_high`: Updates the interrupt enable register with a specific value, calls another routine, and then restores the register to a different value.
- `update_list_pointers`: Manipulates pointers within a structure to link two elements and updates a global variable.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `toggle_interrupt_enable`: Sets a global variable to a specific value, calls another routine, and then updates the global variable to a different value before returning.
- `execute_infinite_loop`: Enters an infinite loop that prevents further execution.
- `update_global_pointer`: Traverses a pointer structure twice, performs a logical negation on the result, and sets a global variable to a constant value.
- `dereference_pointer_chain`: Performs multiple levels of pointer dereferencing using a global variable and then clears it.
- `wait_for_global_non_zero`: Enters a loop that waits for a global variable to be non-zero, calls another routine, and then enters another loop waiting for the same condition.
- `execute_looping_callback_2`: Executes a loop that waits for a global variable to become non-zero, calls a function through a pointer, and performs arithmetic on values.
- `execute_callback_loop`: Monitors a global variable until it is non-zero, then executes a function via a pointer and performs arithmetic on specific values.
- `execute_looping_callback`: Executes a loop that waits for a global variable to become non-zero, calls a function through a pointer, and performs arithmetic on values retrieved from a structure.
- `copy_memory_block`: Copies a block of 8000 words from one memory location to another based on a pointer stored in a global variable.
- `execute_callback_routine`: Calls a function pointer provided in a register and updates several fields in a structure pointed to by a global variable.
- `update_structure_fields`: Checks if a value is within a specific range and, if so, updates several multi-byte fields within a structure.
- `call_initialization_and_update_routines`: This routine calls a routine to populate a memory range with incrementing values and then calls a routine to update multi-byte fields within a structure based on range checks.
- `store_value_through_pointer`: Stores a value from a register into a memory location through a pointer.
- `initialize_data_table`: Fills a memory range with incrementing values in a loop based on a count from a table.
- `apply_table_masks`: iterates through tables to either perform bitwise AND operations or accumulate values based on conditional checks.
- `accumulate_masked_table_values`: Iterates through a source table and accumulates values into a destination table based on a conditional check.
- `mask_table_values`: Iterates through a table and stores a bitwise AND of a value with a mask into another table location.
- `fill_table_with_results`: Iterates a set number of times based on a value from a table, calling another routine and storing the returned results into two different offsets within a structure.
- `sum_routine_results`: Calls another routine twice and returns the sum of the results.
- `calculate_scaled_product`: Performs a complex multiplication involving bitwise masking and shifts to produce a 32-bit result.
- `decrement_and_update_state`: Decrements a global variable and, if it reaches zero, writes a value to a table entry and updates a global variable.
- `modify_global_and_flags`: Updates a global variable using arithmetic from registers and modifies two specific bit-field variables based on XOR logic.
- `fetch_table_entry`: Uses a value from a register to calculate an offset and fetch a 32-bit value from a table into a global variable.
- `clear_bits_in_range`: Iterates through 128 bytes of memory starting at a fixed address and clears the second least significant bit of each byte.
- `calculate_bitmask_value`: Calculates a byte by performing an XOR operation between a bit-shifted value derived from a register and a constant, then stores the result in a global variable.
- `decrement_global_counter`: Increments a global counter until it reaches a threshold, then decrements a sequence of bytes in memory until they reach zero.
- `update_button_state`: Checks the status of a bit in a register and updates a global variable to reflect the change.
- `update_coordinate_data`: Increments a pointer and writes a byte to a memory location.
- `merge_sorted_sub_arrays`: This routine performs a merge operation on two sorted sequences of 16-bit signed integers.
- `merge_sorted_integer_arrays_3`: Merges two sorted sequences of short integers into a single sorted sequence.
- `merge_sorted_sequences`: Merges two sorted sequences of short integers into a single sorted sequence.
- `merge_sorted_short_arrays`: Merges two sorted sequences of short integers into a single sorted sequence.
- `merge_sorted_integer_arrays_2`: Merges two sorted sequences of short integers into a single sorted sequence.
- `merge_sorted_integer_arrays`: Merges two sorted lists of short integers into a single sorted sequence.
- `merge_sorted_intervals`: Takes two lists of sorted integer pairs and merges them into a single sorted list of non-overlapping intervals.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `store_register_to_global`: stores the value from a register into a global variable.
- `concatenate_registers`: Concatenates two 32-bit registers into a single 64-bit value.
- `return_stack_value`: Returns a value from the stack.
- `return_stack_value_2`: Returns the value currently stored at the top of the stack.
- `nop_sequence_padding`: This routine is empty and consists of a sequence of instructions that do not perform any operations.
- `subtract_offset_value`: subtracts a 16-bit value located at an offset from a pointer from a register and returns the result.
- `add_offset_value`: adds a 16-bit value located at an offset from a pointer to a register and returns the result.
- `return_without_effect`: The routine consists of a single return instruction and performs no computation, hardware access, or library calls.
- `return_empty_routine`: This routine performs no operations and immediately returns to its caller.
- `null_operation_routine`: The routine is empty and performs no operations.
- `return_immediately_3`: the routine executes a sequence of instructions that results in an immediate return to the caller.
- `return_immediately_2`: This routine is empty and performs no operations before returning to its caller.
- `return_immediately`: This routine performs no operations and returns to its caller immediately.
- `nop_padding_sequence_2`: This routine is empty and performs no operations.
- `update_boolean_flag`: checks a byte at a memory location and writes a zero or one to a different offset based on whether that byte is zero.
- `update_accumulated_values`: Merges two sorted sequences of 16-bit signed integers and adds elements from a pointer-based array to two separate memory locations based on a boolean check.
- `update_global_bounds`: Updates a global variable and several memory locations using values derived from previous states and comparisons against a table of values.
- `increment_wrap_global`: increments a global variable and sets it to a specific value if it wraps to zero.
- `reset_global_variable`: checks a global variable and sets it to zero or negative one depending on its current value before finally setting it to zero.
- `update_table_with_calculated_values`: calculates values based on registers and a global variable, then writes them into a table via pointers until a termination address is reached.
- `set_global_constant`: stores a constant value into a global variable and enables DMA channels.
- `compare_and_return_constant`: compares a 16-bit value against a global variable and returns either zero or a specific negative constant based on the result.
- `fetch_byte_with_offset`: Returns a single byte from a memory location using an offset added to a register value.
- `lookup_value_with_bounds_check`: Checks if a byte value falls within a specific range and returns either a bit-shifted value from a table or a value from a secondary table based on a flag.
- `render_bitmask_sprite_graphics`: Performs bitwise rotation and masking to blend source pixel data into a target memory buffer.
- `apply_bitmask_transformations_2`: Performs bitwise OR and AND operations on data at specific offsets within a memory block while applying a bitwise rotation to a source byte.
- `apply_bitmask_logic_to_structure`: Performs a bitwise OR on a word at the location pointed to by a0 using d0, then applies bitwise AND operations to words at offsets 40 and 80 of the same structure using d1.
- `apply_bitmask_transformations`: performs bitwise operations and rotations on data from one memory location and stores the results into another memory location using a loop.
- `execute_nested_loops`: performs a series of decrements within nested loops using values from registers.
- `decrement_nested_counters`: decrements two 16-bit values using nested loops until they reach zero.
- `decrement_register_values`: Decrements two 16-bit values stored in registers within a nested loop structure.
- `decrement_registers_in_loops`: Decrements two values stored in registers within nested loops until they reach zero.
- `populate_sprite_attribute_table`: Calculates a base offset from a bitwise combination of input values and populates a memory table with repeated data.
- `configure_blitter_hardware`: Interacts with numerous Blitter registers and DMA control registers, and calls a routine to populate a memory table.
- `traverse_bitstream_table`: Reads a sequence of bytes from a table using an index derived from bit manipulation and bitwise flags.
- `execute_blitter_line_sequence`: Configures and triggers a series of Blitter operations to draw lines or shapes by iterating through coordinate pairs.
- `traverse_lookup_table_2`: Performs a hierarchical traversal of a lookup table using bitwise operations and indirect jumps based on calculated offsets.
- `execute_indirect_jump_2`: Calculates an offset from a register value and performs an indirect jump to a destination address.
- `traverse_lookup_table`: Performs a multi-step bitwise traversal through a hierarchical lookup table.
- `dispatch_offset_based_jump`: Calculates a difference between two values from a pointer-indexed table and, if the difference is positive, performs an indirect jump to an address derived from a global variable and that difference.
- `execute_indirect_jump`: Calculates an offset from a global variable using a value from a register and jumps to the resulting address.
- `nop_padding_instruction`: This routine is empty and consists of two instructions that perform no operation.
- `traverse_bitmapped_lookup_table`: Performs a multi-step bitwise traversal through a hierarchical lookup table.
- `update_blitter_sprite_parameters`: Calculates new coordinates and dimensions for a graphical object and updates Blitter registers and memory-resident control structures.
- `return_d0_register`: Returns the current value of a data register.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_data_structure`: Sets several consecutive global variables to zero and stores a value from a register into a global variable.
- `parse_structured_stream`: Initializes a data structure from a pointer and iterates through a stream of bytes, executing routines via a table and handling specific control characters.
- `fetch_word_at_pointer`: Loads a word from the memory location pointed to by a register.
- `fetch_word_via_pointer`: Loads a word from a memory location stored in a register.
- `store_byte_via_pointer`: Stores a byte from a memory location into a global variable through a pointer.
- `store_byte_and_recurse`: Stores a byte from a memory location into a destination offset by a fixed value and then calls itself.
- `fetch_words_via_pointer`: Loads words from memory locations using pointers stored in registers.
- `store_byte_at_offset_2`: Stores a byte from a pointer into a memory location at a specific offset.
- `store_byte_at_offset`: Stores a byte from a register into a memory location at a specific offset from a pointer.
- `set_byte_to_max`: Stores the value zero-ff into a memory location at a specific offset.
- `clear_byte_and_global`: Stores zero into a memory location at a specific offset and sets a global variable to zero.
- `set_byte_and_recurse`: Stores a constant byte into a memory location determined by an offset from a register and then calls itself indefinitely.
- `store_byte_and_recurse_2`: Stores a constant byte into a memory location offset from a register and then calls itself indefinitely.
- `populate_and_store_data`: Copies a data structure into global variables and stores two words from a recursive routine into a memory location.
- `store_recursive_words`: Stores two words returned by a recursive routine into a memory location at specific offsets.
- `populate_data_structure`: Copies multiple bytes and performs arithmetic from a source structure into a series of global variables.
- `clear_global_variable_2`: Sets a global variable to zero.
- `calculate_offset_difference`: Fetches a value from a memory location using an offset, performs arithmetic involving a byte from another offset, and stores the result in a global variable.
- `process_conditional_buffer`: Performs table lookups and arithmetic to modify a buffer or update global variables based on a byte value.
- `store_two_bytes_increment`: Stores a constant byte and a register byte into a global variable, then increments the pointer.
- `store_two_bytes_increment_alt`: Stores a different constant byte and a register byte into a global variable, then increments the pointer.
- `increment_global_variable_recursively`: If a global variable is non-zero, it is incremented by one through a series of arithmetic operations, and then the routine calls itself twice recursively.
- `process_structured_data`: Iterates through a data structure to perform arithmetic and bitwise operations, eventually storing a calculated value into a memory location.
- `decrement_global_with_bounds`: Decrements a global variable twice if it remains above zero, otherwise sets it to a specific value.
- `execute_indirect_table`: Iterates through a table of pointers and executes the code located at each address.
- `update_global_variables`: Stores the values of two registers into two consecutive global variables.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `add_offset_to_routine_result`: Calls another routine and adds a constant value to its returned lower sixteen bits.
- `populate_data_structure_2`: Depending on a global variable, it calls several subroutines to fetch values and stores them in a specific order into a memory block, performing additions on some values in one branch.
- `call_and_store_result`: Calls another routine and stores its return value into a global variable.
- `execute_indirect_calls`: Calls another routine three times and then executes a routine via an indirect pointer found in a table.
- `update_object_graphics`: Calls a routine to recalculate the position and size of a graphical object and refresh its associated control structures and hardware registers.
- `store_subroutine_result`: Calls another routine and stores its return value into a memory location calculated from a register and an offset.
- `call_routine_and_add_constant`: Calls another routine and adds a constant value to the returned lower sixteen bits.
- `nop_padding_sequence_3`: This routine is empty and consists only of padding instructions.
- `reset_and_offset_data`: Clears a series of global variables to zero and stores a register value into a global variable before adding a constant to the result of another routine.
- `calculate_offset_value`: Calculates a value by adding a scaled result to a base value and stores it in a memory location.
- `process_conditional_data`: Merges sorted 16-bit signed integers and executes a series of conditional subroutines, including Blitter operations and data structure population, based on the value at a memory location.
- `execute_conditional_chain`: calls several subroutines in a sequence, where the specific number and order of calls are determined by the value stored in a memory location.
- `execute_subroutine_chain`: calls another routine multiple times in succession.
- `wait_for_flag`: Performs a sequence of operations and then enters a loop that waits for a global variable to become non-zero.
- `call_increment_routine`: calls a routine that increments a global variable and wraps it to a specific value upon reaching zero.
- `execute_indirect_call`: Loads a function address from a table using an offset and executes it.
- `tiny_move_rts`: Small routine of 2 instructions that copies (a3)+ to d0, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_complex_control_flow`: Performs a series of recursive calls, updates status flags, and modifies global variables based on conditional checks and table lookups.
- `call_another_routine`: Calls another routine and returns.
- `process_decompressed_data`: reads a bitstream to expand data into a buffer and updates several memory locations based on the results.
- `update_status_flags_2`: Calls another routine to update status flags.
- `decompress_bitstream`: Reads bits from a stream to expand data into a buffer using a variable-length decoding algorithm.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `call_update_flag_and_set_bit`: Calls another routine that updates a table with a boolean value derived from the zero flag and performs a bitwise OR on a global variable.
- `execute_subroutine_calls`: Calls another routine twice in succession.
- `process_quiz_code_result`: Retrieves a value from a data file and stores the result into a global variable.
- `execute_recursive_calls_2`: The routine performs two consecutive calls to itself before returning.
- `update_status_flags_3`: Executes two consecutive calls to other routines and stores the resulting status flags into a global variable and a bitmask.
- `store_subroutine_result_2`: stores the return value of a subroutine into a global variable.
- `tiny_ori`: Small routine of 1 instruction that merges 88 into d4.
- `update_flag_and_set_bit`: Stores a boolean value derived from a zero flag into a table and performs a bitwise OR on a global variable.
- `call_offset_routine`: Calls a routine located at a specific offset within a global table.
- `call_indirect_offset_routine`: Calls a routine through a pointer stored at a specific offset within a global table.
- `call_indirect_offset_routine_alt`: Calls a routine through a pointer stored at a different offset within a global table.
- `trigger_status_flag_update`: Calls a routine that updates status flags based on the value of a global variable.
- `update_status_flags`: Sets a byte in a table based on whether a global variable is zero and modifies a global variable using a bitwise OR.
- `update_status_and_fill`: Sets a byte in a table based on a zero flag, performs a bitwise OR on a global variable, and fills a memory range with a pattern.
- `call_indirect_offset_routine_alt2`: Calls a routine through a pointer stored at another specific offset within a global table.
- `set_table_byte`: Calculates an offset into a table using a value from a register and stores a byte at that location.
- `call_indirect_pointer`: Loads a pointer from a table and executes the routine located at that address.
- `call_update_status_flags`: Calls a routine that updates status flags based on a global variable's value.
- `execute_indirect_call_2`: Calls a routine through a double pointer and conditionally calls another routine based on a global variable.
- `fill_memory_pattern`: Fills a memory range with a repeating pattern of bytes and then clears a global variable.
- `process_data_table`: Iterates through a table to perform comparisons, updates a global variable, and calls a routine through a pointer based on specific conditions.
- `compare_and_update_value`: Compares a byte from a table with another byte and, if the condition is met, stores a multi-byte value into a table location.
- `clear_global_variable_3`: Checks if a global variable is non-zero and, if so, sets it to zero.
- `evaluate_logical_conditions`: Performs a series of comparisons on a global variable and calls another routine if the conditions are met.
- `calculate_audio_period`: Calculates a new value based on the absolute value of a global variable and stores it in that variable, while interacting with an audio hardware register.
- `call_subroutine_twice`: calls a subroutine twice before returning.
- `push_registers_and_return`: pushes several registers onto the stack before returning.
- `execute_repeated_calls`: Sets a global variable to negative one and then calls another routine multiple times if the variable was initially zero.
- `assign_global_variable`: Copies a value from a memory location into a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `nop_padding_sequence_4`: This routine is empty and consists of two instructions that do not perform any operations, serving as padding between other code segments.
- `call_another_routine_2`: This routine calls another routine and returns the concatenated results of the two data registers used by the called routine.
- `update_dma_hardware`: Updates custom hardware registers and DMA control settings by iterating through a data table.
- `update_dma_hardware_parameters`: Iterates through a table of data to update custom hardware registers and DMA control settings.
- `manipulate_dma_control`: A routine consisting of eighteen instructions that interacts with the DMA control register.
