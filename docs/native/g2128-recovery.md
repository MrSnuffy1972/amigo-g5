# Game 2128 (killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02128). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (939536 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 12 raw-word lines |
| Functions named and described | pass, 188 of 188 named (0 author symbols), 188 of 188 described (5 by rule, 179 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1283 of 1283 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3434 of 3658 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 24022 |
| code | 11586 |
| data | 866034 |
| padding | 6 |
| unused | 37888 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company][a].adf:bootblock | 1 | 249 | 1 |
| logged disk load from Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company][a].adf boot-block load | 1 | 85 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2582 | 141 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 415 | 36 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 310 | 5 |

## Cases handled

- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company][a]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company][a].adf: custom-format (non-DOS) disk, 1684 non-empty blocks: matched against track-loaded segments
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company][a]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company][a].adf: the boot block reads 6144 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 14 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1109 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 139 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 21 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company][a]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company][a].adf: 1 logged loads cover 6144 bytes of the image
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company][a]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company][a].adf: blocks 22-1032 (bytes 11264-528896) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.30)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company][a]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company][a].adf: blocks 1265-1285 (bytes 647680-658432) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.84)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company][a]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company][a].adf: blocks 1540-1549 (bytes 788480-793600) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.32)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company][a]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company][a].adf: blocks 1551-1560 (bytes 794112-799232) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.64)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company][a]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company][a].adf: blocks 1562-1571 (bytes 799744-804864) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.62)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company][a]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company][a].adf: blocks 1573-1582 (bytes 805376-810496) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.64)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company][a]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company][a].adf: blocks 1584-1593 (bytes 811008-816128) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.63)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company][a]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company][a].adf: blocks 1595-1604 (bytes 816640-821760) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.63)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company][a]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company][a].adf: blocks 1606-1615 (bytes 822272-827392) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.62)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company][a]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company][a].adf: blocks 1617-1626 (bytes 827904-833024) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.64)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company][a]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company][a].adf: blocks 1628-1637 (bytes 833536-838656) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.62)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company][a]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company][a].adf: blocks 1639-1648 (bytes 839168-844288) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.63)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company][a]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company][a].adf: blocks 1650-1659 (bytes 844800-849920) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.62)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company][a]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company][a].adf: blocks 1661-1670 (bytes 850432-855552) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.64)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company][a]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company][a].adf: blocks 1672-1681 (bytes 856064-861184) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.63)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company][a]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company][a].adf: blocks 1683-1692 (bytes 861696-866816) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.64)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company][a]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company][a].adf: blocks 1694-1703 (bytes 867328-872448) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.62)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company][a]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company][a].adf: blocks 1705-1714 (bytes 872960-878080) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.63)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company][a]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company][a].adf: blocks 1716-1725 (bytes 878592-883712) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.62)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company][a]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company][a].adf: blocks 1727-1736 (bytes 884224-889344) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.63)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company][a]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company][a].adf: blocks 1738-1747 (bytes 889856-894976) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.63)
- killing cloud, the (1991)(vektor graphix)(m3)(disk 1 of 2)[cr company][a]/Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company][a].adf: blocks 1749-1758 (bytes 895488-900608) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.64)
- 44 packed track region(s), 225280 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company][a].adf:bootblock

- `init_struct_and_trap`: Initializes a fixed memory structure by writing several constant fields through a base pointer, then triggers a software interrupt via the trap instruction.

### logged disk load from Killing Cloud, The (1991)(Vektor Graphix)(M3)(Disk 1 of 2)[cr Company][a].adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_byte_via_pointer`: Stores a byte from register D0 through a pointer in register A0, then returns.
- `call_routine_twice`: It calls another routine twice in succession.
- `call_two_routines`: It calls two other routines in sequence and then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `read_custom_and_call`: Reads a bit field from a custom register, scales the value, and repeatedly invokes a function pointer stored in a global variable while the scaled result stays non-negative.
- `call_pointer_after_doubling`: Initializes a global variable to eight, then repeatedly doubles a running value and calls a function pointer whenever that doubled value becomes negative.
- `loop_caller_pointer`: Repeatedly calls a function pointer from a global variable, doubling a running value each time and re-calling when it goes negative, then returns a stack value.
- `call_pointer_repeatedly`: Repeatedly calls a function pointer stored in a global variable, multiplying the returned value by two and invoking the pointer again whenever the result becomes negative.
- `call_pointer_if_negative`: Calls a function pointer stored in a global variable, doubles its result, and invokes the pointer again if the doubled value goes negative.
- `call_pointer_after_double`: Reads a value from a function pointer in a global variable, doubles it, and calls the pointer again when the result is negative.
- `call_single_pointer`: Invokes a function pointer stored in a global variable once and then returns.
- `splice_node_into_list`: Walks a doubly linked list by following next pointers until reaching a node whose field is out of range, then splices a new node into the chain by rewiring the surrounding links.
- `invoke_list_callbacks`: Iterates a singly linked list until reaching a null terminator, invoking a function pointer located at an offset within each node before advancing.
- `store_unsigned_value`: Clears the sign bit of a value, stores the result in a global variable, and forces the stored value to one when it is zero.
- `clear_global_variable`: XORs a global variable with itself, which always stores zero back into it.
- `increment_and_store_255`: Increments a global variable, calls another routine, then stores the value 255 into that same global variable.
- `disable_and_enable_interrupts`: Writes a value with bit 15 set to the interrupt enable register to disable interrupts, calls another routine, then writes a value with bit 15 clear to re-enable them.
- `splice_pair_and_store`: Treats the integer in register A0 as a pointer to a two-slot structure, moves the value in that first slot into the second slot's stored value and links the two slots together, then writes a fixed constant into a global variable.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `set_global_variable`: Clears a global variable, invokes another routine, then stores a value into that same global variable before returning.
- `loop_forever`: Enters an unconditional infinite loop and never returns.
- `update_cop_status`: Reads a four-byte value from a global variable offset by eight and stores it back into the same global variable.
- `resolve_double_pointer`: Loads a pointer from a global variable and dereferences it twice to fetch two more pointers.
- `wait_for_zero_twice`: Loops until a global variable becomes zero, calls another routine, then loops again until the variable is zero once more.
- `wait_and_update_structure_2`: Loops five times, waiting for a global variable to reach zero before each step while updating it, then performs a final check on a structure.
- `wait_update_check_loop`: It loops five times, each iteration waiting for a global variable to reach zero before updating it, then performs a final check on a structure.
- `wait_and_update_structure`: Loops five times, waiting for a global variable to reach zero before each step and updating it, then performs a final check on a structure field before returning.
- `copy_table_words`: Waits in a loop until a global variable becomes zero.
- `invoke_and_mark_structure`: Waits until a global variable is zero, then stores an incoming pointer into a structure field.
- `validate_and_store_offsets`: Checks that an incoming value falls within a range and is not below a stored value.
- `invoke_two_routines`: Invokes two further routines in sequence and then returns.
- `store_value_then_call`: Stores an incoming value into a memory location addressed by a pointer and then calls another routine.
- `clamp_and_copy_count_2`: Reads a count from a structure, caps it at sixteen, and writes that clamped count into two fields of another structure, then fills those fields with incrementing values.
- `merge_and_mask_shorts`: It determines the smaller of two counts held in two structures, then walks through a run of short entries.
- `accumulate_masked_shorts`: Walks through a run of short-integer entries, and whenever a running accumulator held at the current position is greater than negative one, it adds two freshly computed values into it and stores the result.
- `copy_masked_values`: Copies masked values from one array into another for a fixed number of entries, then calls another routine when a structure field is non-zero.
- `clamp_and_copy_count`: Reads a value from a structure, caps it at sixteen, and writes that clamped count into two fields of another structure.
- `sum_two_call_results`: Calls another routine, calls it again to obtain a second value, and returns the sum of the two results.
- `combine_scaled_values`: Multiplies an incoming value by two and combines it with a stored multiplier using masked shifts.
- `decrement_global_byte`: Decrements a global byte and, when it reaches zero, writes the value seven into a byte of a table located near address 4604 and stores the value 32776 into another global byte.
- `apply_byte_flags`: Computes a difference between two incoming values and folds it into a global byte.
- `select_table_entry`: Indexes into a table of four-byte entries using bits of an incoming value and stores the selected entry into a global variable before returning.
- `clear_ram_bits`: Clears a specific bit in each of one hundred and twenty-seven consecutive bytes of a RAM region by masking and storing each value.
- `store_button_mask`: Computes a bitmask from a bit position, combines it with a fixed value, and stores the result into a global byte.
- `decrement_byte_sequence`: Increments a global counter when another reaches a fixed limit, then decrements a run of bytes and clears each one while it hits zero.
- `process_port_button`: Reads a button state from a hardware port, inverts it, and stores it into a global variable while mirroring the same bit.
- `transfer_bounds_fields`: Copies a set of coordinates and dimensions from one structure into another, marking the source as empty and copying a following word along the way.
- `merge_sorted_arrays`: Treats two arrays of signed 16-bit values as sorted lists and merges each one into a single sorted sequence using an in-place insertion approach.
- `merge_sorted_arrays_when_flag_zero`: When a flag byte at offset 17 is zero, it decrements a count at offset 6 and merges two sorted arrays of signed 16-bit values pointed to by offsets 18 and 22 into a destination starting at offset 6, using the helper routines to combine the running front values of each source in order.
- `merge_sorted_arrays_1_2`: Merges two sorted arrays of signed 16-bit values into one sorted array by comparing the running front values of each source and writing the smaller value to the destination.
- `merge_sorted_arrays_1`: Merges two sorted arrays of signed 16-bit values into one sorted array by comparing the running front values of each source and writing the smaller result.
- `merge_sorted_arrays_5`: Merges two sorted arrays of signed 16-bit values into a single sorted array by comparing the running front values of each source and writing the smaller one.
- `merge_sorted_arrays_3`: Merges two sorted arrays of signed 16-bit values into a single sorted array, comparing the running front values of each source and writing the smaller result into the destination.
- `merge_sorted_arrays_2`: Merges two sorted arrays of signed 16-bit values into a single sorted array.
- `return_immediately`: This routine is empty and returns immediately without performing any operation.
- `store_register_value`: This routine stores a value received in a register into a global variable.
- `return_register_pair`: Returns the two data registers to its caller without modifying any data or registers.
- `return_stack_value_3`: Returns the value already sitting at the top of the call stack to its caller without modifying anything.
- `return_stack_value_2`: It returns the value that is already sitting at the top of the call stack, passing it straight back to the caller without modifying anything.
- `return_stack_value`: This routine returns a value that is loaded from the top of the stack.
- `subtract_global_short`: This routine subtracts a short value stored in a global variable from a register value and returns the result.
- `add_global_short`: This routine adds a short value stored in a global variable to a register value and returns the result.
- `return_empty_routine_2`: This routine is empty and performs no operations before returning.
- `return_immediately_2`: This routine contains no instructions and simply returns without performing any operation.
- `perform_no_operation`: This routine is empty and performs no operations before returning to its caller.
- `empty_routine_2`: This routine contains no instructions and performs no operations, returning immediately to its caller.
- `do_nothing_return`: Performs no data operations and simply returns to its caller.
- `adjust_table_value_2`: It loads a value from a table using an index, adds a constant to that value, and stores the resulting sum back through a pointer.
- `adjust_table_value`: It loads a value from a table using an index, adds a constant to it, and stores the result back through a pointer.
- `store_empty_flag`: It stores a boolean flag byte at a fixed offset inside a structure, computing its value as the logical negation of whether the adjacent byte is zero.
- `merge_sorted_arrays_4`: Checks a flag byte in a structure and, when it is clear, adds the first two words of an array into two corresponding words of that structure.
- `reset_global_counter_2`: Reads a pair of global shorts, then, when a global counter is zero, compares several global shorts and stores the smallest of each pair back into the counter.
- `increment_global_byte`: This routine increments a global variable and resets it to a maximum byte value when it overflows.
- `reset_global_counter`: It resets a global variable to zero, then sets it to zero again if it is non-empty, and sets it to negative one if it is greater than negative one and not zero, using a routine that stores a fixed constant into a global variable.
- `write_sprite_registers`: This routine computes sprite position and attribute bytes, then writes them into a table of hardware registers, stopping when it reaches a sentinel entry.
- `store_constant_value`: This routine stores a fixed constant value into a global variable.
- `compare_register_value`: This routine compares a register value against a global variable and returns a value depending on which one is larger.
- `read_byte_from_global`: Reads a single byte from a global variable at a fixed offset and returns it as a byte value.
- `lookup_clamped_index`: Clamps an index into a range and returns a value looked up from a table, using either a default entry or an offset computed from the clamped index.
- `apply_rotated_mask_to_buffer`: Combines a rotated byte pattern into a screen buffer using bitwise OR while clearing corresponding mask bits at fixed offsets, handling both single-word and two-word spans depending on the pattern width.
- `rotate_and_apply_bytes_2`: It stores a word into a buffer through a pointer, then masks three further words at fixed offsets.
- `rotate_and_apply_bytes`: Processes a fixed number of byte entries, rotating each byte by a bit count and applying the result to a block of words.
- `mask_and_store_buffer`: It stores a word into a buffer through a pointer, then masks three further words at fixed offsets.
- `run_nested_countdown`: It runs a pair of nested loops that decrement two counters until each reaches its limit, swapping one counter into the other between iterations.
- `decrement_loop_counters`: It decrements two loop counters in nested loops until each reaches its sentinel value, then returns.
- `delay_wait_loops`: Waits for a delay by decrementing two nested loop counters until both reach zero, then returns.
- `run_nested_loops`: Executes two nested empty loops that only decrement their counters and then returns.
- `fill_and_program_blitter`: Fills a block of words from a source table into a screen buffer, then programs a blitter with several transfer commands while waiting for the vertical blank.
- `program_blitter_transfer`: Programs the blitter transfer registers and waits for the blitter to finish.
- `pack_table_bits`: Reads a byte from a lookup table and extracts its high bit, then uses that bit to compute the next table index in a loop.
- `blitter_fill_horizontal_spans`: Waits for the blitter to finish, then configures a horizontal blitter fill and repeatedly issues it to draw a series of horizontal spans, advancing each span by a fixed amount until the count reaches zero.
- `compare_masked_words`: Checks whether two null-terminated strings are empty and then compares masked values from two word arrays, branching into a jump table when they differ and are ascending.
- `call_via_jump_table`: Reads an 11-bit value from a global variable, shifts it right by three, and uses the result as an offset into a jump table to call the routine found there.
- `scan_tables_via_jump_table`: Loops over two parallel tables of 16-bit words, masking off the low nibble of each pair.
- `compare_words_via_jump_table`: Compares the masked low values of two words pointed to by a register, and when they differ and are ascending, indexes a jump table using the shifted difference and jumps to the selected routine.
- `call_indirect_via_jumptable`: Invokes a routine pointed to by an indirect register, then reads a global variable, shifts its value right by three, indexes into a jump table, and calls the routine selected at that computed address.
- `return_empty_routine`: Performs no operations and returns immediately to its caller.
- `reverse_bits_and_store`: Reads a 12-bit value from a table using a bit-reversed index and stores the result back into the same table.
- `configure_blitter_horizontal_copy`: Waits for the blitter to finish, then computes clipped source and destination screen coordinates and configures the blitter to copy a horizontal run of words, adjusting the transfer length and start pointers based on the low nibble alignment of each end.
- `return_input_register`: Returns the value already held in the input data register without modifying it.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_globals_and_copy`: Clears several consecutive global bytes and copies a four-byte value into a nearby global slot, then calls a helper routine.
- `dispatch_byte_sequence`: Initializes a structure from a source pointer when a flag is zero, then walks a byte sequence dispatching on special markers.
- `load_word_from_pointer`: Reads a word from the location pointed to by the address register and returns it.
- `load_and_call_twice`: Reads a word from the location pointed to by an address register, then calls another routine twice with that value.
- `store_byte_triplicate`: Writes a byte from a register into a global variable at a fixed offset, then performs the same byte-store operation two more times through another routine.
- `store_byte_and_fill_pair_alt`: Stores a register byte into a global variable at a fixed offset, then stores a different constant byte and a register byte into two consecutive global bytes while advancing the global pointer.
- `call_routine_three_times`: Calls another routine three times in succession and then returns.
- `store_byte_from_pointer`: Copies a byte from one pointer into a byte field of a structure and then calls a helper routine before returning.
- `copy_byte_at_offset`: Copies a single byte from the source pointer into a fixed offset of the destination pointer.
- `set_struct_field_to_ff`: Writes the byte value 255 into a fixed field of a structure and then calls a helper routine before returning.
- `clear_struct_field_and_global`: Writes zero into a fixed field of a structure, clears a global variable, then calls a helper routine before returning.
- `store_byte_then_call_helper`: Writes the byte 255 to the location pointed to by a pointer, then invokes another routine and returns.
- `store_byte_then_call_helper_2`: Writes the byte value 255 to a location pointed to by a pointer, then invokes another routine and returns.
- `store_words_from_pointer`: Reads two words from a data source and stores them into two consecutive 16-bit fields of a structure reached through a pointer.
- `load_words_into_structure`: Loads two words from a source pointer and stores them into consecutive fields of a global structure, then invokes a helper routine that mutates a global variable.
- `initialize_structure_from_source`: Initializes a global structure by storing constant bytes and copying several fields from a source pointer, including a computed difference between two words.
- `clear_global_words`: Clears two consecutive global words and then assigns one of them from the other.
- `adjust_short_and_byte`: Copies a short value from a global variable to a nearby global slot, computes the difference between another short and that value, adjusts a global byte by subtracting a character and a routine result, and leaves a global short in a register.
- `process_buffered_byte`: Writes a byte into a buffer based on a flag, clamps an index into a table, and adjusts a short value.
- `store_bytes_advance_pointer`: Stores a constant byte and a register byte into two consecutive global bytes, then advances the global pointer by two.
- `store_bytes_advance_pointer_alt`: Stores a different constant byte and a register byte into two consecutive global bytes, then advances the global pointer by two.
- `modify_global_and_call_helpers`: When a global variable is nonzero, doubles it twice, subtracts it from itself, increments it, then calls two helper routines and returns.
- `process_clamped_table_lookup`: Checks a global variable, computes a shifted and swapped value from it, then walks a table of negative sentinels and space markers while accumulating results and storing a computed word into a structure field.
- `decrement_global_counter`: Decrements a global counter and, under a couple of conditions, decrements it a second time.
- `call_table_infinite`: Invokes a routine with a global value twice, then repeatedly calls routines pointed to from a table while advancing a pointer without an exit condition.
- `store_two_globals`: Calls another routine, then stores two incoming parameter values into two global variables.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `offset_returned_word`: Calls a routine to obtain a value, masks it to a word, and returns that value offset by a fixed amount.
- `store_buffer_branch`: Branches on a global short, storing either raw routine results or subtracted and added short values into a buffer, then performs further global arithmetic and stores two shorts.
- `call_and_store_result`: Calls another routine and stores the returned value into a global variable.
- `call_routines_and_dispatch`: Invokes three other routines in sequence and then dispatches through an indexed jump table using a value from a global variable.
- `call_blitter_copy_once`: Calls another routine once and then returns.
- `store_value_via_pointer`: Reads a value from memory and stores it through a pointer derived from a global variable and a register offset.
- `return_offset_word`: It calls another routine to obtain a value, masks it to a word, adds a fixed amount, and returns that value.
- `empty_routine`: Contains no instructions and simply returns to its caller.
- `return_offseted_word`: Invokes a helper routine to obtain a value, masks it to a word, and returns it offset by a fixed amount.
- `index_from_flag_result`: Reads a flag byte at a pointer and, depending on whether it is zero, computes an index from a routine result or takes a returned value, then passes that computed value to another routine.
- `store_buffer_branch_2`: Calls a routine twice, then branches on a buffer value to store either raw or adjusted results before calling more routines and returning.
- `call_routines_branch`: It invokes a sequence of other routines in order, then branches on a global short to decide which of several further routines to call.
- `call_five_routines`: It invokes five other routines in sequence and then returns.
- `resolve_pointer_and_loop`: Dereferences a global pointer twice and then loops until a global variable holds the null character.
- `empty_return_routine`: An empty two-instruction routine that returns immediately without touching any data or registers.
- `call_indirect_from_table`: Reads a pointer from a global variable, adds an offset from a register, and calls the routine located there.
- `tiny_move_rts`: Small routine of 2 instructions that copies (a3)+ to d0, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `invoke_flag_and_loop_routine`: Chains a sequence of helper routines and a fixed-offset indirect call, then runs a loop that repeatedly invokes helpers, sets a global variable to a sentinel value, compares it against constants, and branches based on bits read from a custom-register-backed table.
- `call_and_return`: Invokes another routine and then returns to its caller.
- `store_value_and_check_result`: It stores a value received in a register into a global variable, then calls another routine and inspects its result.
- `invoke_flag_toggle_3`: Invokes a routine that toggles flag bits in a global variable and then calls another routine before returning.
- `decode_and_copy_run`: Decodes bits from a packed stream and copies a run of bytes into a destination buffer, refilling the bit reader as needed.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `empty_return_routine_2`: An empty routine that returns immediately without executing any instructions or modifying any data.
- `store_and_negate_compare`: It calls another routine and stores the value received in a register into a global variable.
- `call_twice_return`: It invokes another routine twice in sequence and then returns, performing no data manipulation of its own.
- `store_and_negate_null_compare`: It calls another routine and stores the value received in a register into a global variable.
- `store_and_negate_compare_2`: Stores a byte from a register through a pointer, then writes the logical negation of a comparison between a global variable and the null character into a buffer while setting a flag bit in that global variable.
- `store_and_return`: It calls another routine and stores the value received in a register into a global variable.
- `tiny_ori`: Small routine of 1 instruction that merges 88 into d4.
- `store_negated_compare`: Writes the logical negation of a comparison between a global variable and the null character into a buffer, sets a flag bit in that global variable, and then calls another routine.
- `call_structured_routine`: Calls the routine stored at a fixed offset within a structure and returns.
- `call_another_structured_routine`: Calls the routine stored at another fixed offset within the same structure and returns.
- `call_third_structured_routine`: Calls the routine stored at yet another fixed offset within the structure and returns.
- `invoke_flag_toggle_2`: Invokes another routine that clears a flag bit in a global variable and then sets a second bit in the same variable before returning.
- `toggle_global_flag_bit`: Clears a flag bit in a global variable, then sets a second bit in the same variable.
- `negate_and_set_flags`: Writes the logical negation of a global comparison into a buffer, sets two flag bits in a global variable, then touches the parallel port register via two routines.
- `call_fourth_structured_routine`: Calls the routine stored at a further fixed offset within the structure and returns.
- `set_table_byte`: Writes the value one into a single byte of a table whose slot is chosen by multiplying an input index by a fixed stride and adding an offset.
- `call_indirect_pointer`: Invokes a function pointer stored at a fixed offset from a base address, treating the fetched value as code.
- `invoke_flag_toggle`: This routine calls another routine that clears a flag bit in a global variable and then sets a second bit in the same variable, then returns.
- `invoke_structured_routine`: Invokes a routine stored in a structure and then calls another routine when a global variable is non-zero.
- `fill_buffer_and_reset`: Fills a run of bytes in a buffer with the value 255 in a repeating pattern, then resets a global variable to zero.
- `sort_and_swap_structures`: Loops a fixed number of times, comparing and swapping entries between two structures while resetting their bookkeeping fields, and sets a table slot to one when a condition holds.
- `store_if_less_than`: Compares two signed bytes and, when the first is smaller, stores a combined value built from a byte and a register into a nearby location.
- `run_and_clear_flag`: Checks a global variable and, when it is non-zero, calls another routine and then resets that variable to zero.
- `evaluate_conditional_return`: It reads a global variable and evaluates a contradictory condition that cannot be true, so it never performs its only action.
- `toggle_global_flag_bit_2`: Clears a flag bit in a global variable, then sets a second bit in the same variable.
- `call_two_routines_2`: Executes two other routines in sequence and then returns.
- `self_recursion_loop`: Performs two sequential recursive calls to itself and then returns, executing no other instructions.
- `initialize_and_repeat_helper`: Initializes a global variable to a sentinel value and then invokes a helper routine ten times in a row.
- `store_register_byte`: Copies the low byte of a register into a global variable and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_empty`: Empty routine that simply returns the value already present on the stack.
- `call_string_buffer_return`: Calls a helper routine that processes a string buffer and returns the two incoming data register values concatenated into a single 64-bit result.
- `process_string_and_clear_dma`: Invokes a routine that walks a length-prefixed string, copying its characters into a buffer while updating a table of custom-register-backed entries, and clears the DMA enable register via a DMACON write with bit 15 set.
- `process_string_buffer`: Walks a length-prefixed string, copying its characters into a buffer while advancing a pointer, and updates a table of custom-register-backed entries based on the processed data.
- `enable_dma_channels`: Writes to the DMA control register, which enables DMA channels when the high bit is set, as part of a longer sequence of instructions.
