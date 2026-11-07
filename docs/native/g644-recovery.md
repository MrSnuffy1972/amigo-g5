# Game 644 (VenusTheFlytrap v1.1 NoIntro): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00644). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1931442 top-level bytes in 16 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 164 raw-word lines |
| Functions named and described | pass, 179 of 179 named (0 author symbols), 179 of 179 described (5 by rule, 140 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4490 of 4498 (99.82%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6972 of 6973 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 160959 |
| code | 31792 |
| data | 1737920 |
| padding | 3 |
| unused | 768 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 276 | 20 |
| logged disk load from disk.1 | 1 | 161 | 14 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.2 | 1 | 0 | 0 |
| logged disk load from disk.2 | 1 | 0 | 0 |
| logged disk load from disk.2 | 1 | 0 | 0 |
| logged disk load from disk.2 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3871 | 77 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2177 | 50 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 177 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 173 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 135 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 3 |

## Cases handled

- VenusTheFlytrap_v1.1_NoIntro/VenusTheFlytrap_v1.1_NoIntro_hdf/VenusTheFlytrapNoIntro/disk.1: raw disk image (80708 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- VenusTheFlytrap_v1.1_NoIntro/VenusTheFlytrap_v1.1_NoIntro_hdf/VenusTheFlytrapNoIntro/disk.2: raw disk image (749568 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- VenusTheFlytrap_v1.1_NoIntro/VenusTheFlytrap_v1.1_NoIntro.hdf: hard-disk image (AmigaDOS OFS, 2048 blocks): its filesystem parses and all 8 files in it are byte-identical to the extracted tree VenusTheFlytrap_v1.1_NoIntro_hdf, which is recovered file by file; no block outside the filesystem holds data
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- 6 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 2510 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1371 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 169 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 147 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 135 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- VenusTheFlytrap_v1.1_NoIntro/VenusTheFlytrap_v1.1_NoIntro_hdf/VenusTheFlytrapNoIntro/disk.1: 2 logged loads cover 80708 bytes of the image
- VenusTheFlytrap_v1.1_NoIntro/VenusTheFlytrap_v1.1_NoIntro_hdf/VenusTheFlytrapNoIntro/disk.2: 4 logged loads cover 72976 bytes of the image
- VenusTheFlytrap_v1.1_NoIntro/VenusTheFlytrap_v1.1_NoIntro_hdf/VenusTheFlytrapNoIntro/disk.2: chunks 1860-2196 (bytes 476160-562432) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.22)
- VenusTheFlytrap_v1.1_NoIntro/VenusTheFlytrap_v1.1_NoIntro_hdf/VenusTheFlytrapNoIntro/disk.2: chunks 2198-2201 (bytes 562688-563712) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.94)
- VenusTheFlytrap_v1.1_NoIntro/VenusTheFlytrap_v1.1_NoIntro_hdf/VenusTheFlytrapNoIntro/disk.2: chunks 2203-2807 (bytes 563968-718848) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.20)
- VenusTheFlytrap_v1.1_NoIntro/VenusTheFlytrap_v1.1_NoIntro_hdf/VenusTheFlytrapNoIntro/disk.2: chunks 2882-2927 (bytes 737792-749568) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.94)
- 2 packed track region(s), 421376 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `select_cpu_variant_and_setup`: Calls two operating-system library routines through a library table and compares the result with two fixed values to choose a variant, storing the choice in a global.
- `patch_memory_instructions_to_calls`: Scans a memory region for specific move instruction encodings that write to a fixed global and replaces them with calls to a routine.
- `install_jump_hook`: Writes an absolute jump to a routine into a fixed low-memory location and writes a call instruction to a second fixed address, then returns the byte argument unchanged.
- `patch_move_instructions_range`: Scans a memory range for move instructions that write to a fixed global, replacing each with a call to a routine.
- `copy_block_and_install_jump`: Copies a block of longword pairs into low memory from a fixed address, writes an absolute jump instruction to a fixed low-memory location, then calls a routine and returns its byte result.
- `patch_move_instructions_range_2`: Scans a memory range for move instructions that write to a fixed global, replacing each with a call to a routine.
- `return_argument_unchanged`: Returns its data register argument unchanged.
- `start_blitter_and_wait`: Writes a word taken from the stack to the blitter size register, then spins while the blitter-busy bit is set before returning.
- `start_blitter_size_from_pointer`: Reads a word through a pointer passed on the stack, writes it to the blitter size register, and spins while the blitter-busy bit is set.
- `write_blitter_size_and_wait`: Writes the word in a data register to the blitter size register, then spins while the blitter-busy bit is set.
- `enable_dma_and_strobe_copper`: Clears one global and writes a fixed DMA enable value to another.
- `enter_infinite_loop`: Executes an infinite loop that traps the instruction pointer without modifying any registers or performing any computational operations.
- `execute_infinite_loop`: The routine executes an infinite loop that does not perform any computational or register-altering operations before looping back to itself.
- `execute_infinite_loop_2`: Performs an infinite loop that never exits.
- `call_object_method_passthrough`: Calls a routine through a function pointer at a fixed offset in an object held in a global, then returns the data register values it was given.
- `call_object_method_passthrough_2`: Calls a routine through a function pointer at a fixed offset in an object held in a global, then returns the stack argument it was given.
- `call_external_subroutine`: Calls a routine via a pointer stored in a table and returns the concatenated result.
- `verify_keyboard_code_or_halt`: Compares the byte argument with a stored global value.
- `update_global_interrupt_mask`: Compares a register to a global variable and, if equal, writes a value to that global variable.

### logged disk load from disk.1

- `install_vectors_and_copy_image`: Issues a system trap, clears the colour registers, and copies a small routine and a block of longwords into low memory.
- `clear_colors_and_copy_image`: Disables interrupts, clears the colour registers, and copies a small routine and a block of longwords into low memory.
- `copy_image_and_call_entry`: Copies a block of longwords from the source into low memory at a fixed address, then calls a routine through a pointer taken from the argument.
- `shuffle_marked_block_in_place`: If the first longword matches a fixed signature, it copies 120 bytes into globals, moves the block's bytes in a chunked loop with helper calls, and copies the 120 saved bytes back at the end.
- `decode_flagged_packed_chunk`: Decodes one step of a packed stream: it reads a flag bit from a bit reader, copies a run of earlier output bytes backward when the flag is set, and returns once the output pointer passes its limit.
- `spin_short_delay_loop`: Counts a short register down from three to minus one with no other work and returns, so it only consumes a few cycles as a pause.
- `read_bits_into_register`: Shifts bits out of a stored longword one at a time into a bit buffer, consuming as many bits as the count register requests and returning when that count is reached.
- `shift_out_packed_bits`: Pulls bits off a bit register one at a time, and when the register is empty it loads the next 16-bit word from the input pointer, moving that pointer backward.
- `check_stream_flags_only`: Tests whether a counter register is zero and the low bit of a pointer is clear, then returns; it changes no memory and no state beyond the processor flags.
- `test_pointer_low_bit`: Tests the lowest bit of a 16-bit address register and returns; it writes nothing to memory or hardware.
- `stub_return_only`: Routine consisting only of a return, with no calls, memory access or hardware register use; it does no work.
- `shift_bit_buffer_by_count`: Shifts a bit buffer left by a counted number of positions, and when the buffer empties it reloads a word from the input pointer while keeping the carry bit between reloads.
- `decode_packed_match_chunk`: Main unpacking loop: it decodes a match length and distance from the bit reader, copies that run of earlier output bytes backward, optionally copies a second run from the input, and returns once the output pointer reaches its limit.
- `resume_packed_decode_loop`: Alternate entry into the packed-data decoder that reads a length value and copies a run, then loops back into the same copy routine until the output limit is reached.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_blt_window_parameters_2`: Calculates and stores new window or or-pattern-related address or size values into a table or global variable based on conditional checks or pointer-based lookups.
- `copy_memory_block_2`: Checks a global variable and, if a certain condition is met, copies a block of memory from one location to another using a loop.
- `loop_until_value`: Executes a routine repeatedly until a register or a certain value is reached.
- `process_object_type_event`: Checks an object type or flag and executes a specific handler, such as updating a counter, resetting object state, or traversing a table to update or remove object properties.
- `reset_global_variable`: Sets a global variable to zero and then checks if it should be set to one.
- `initialize_data_structure`: Initializes a block of memory with specific constant values and certain offsets based on a global variable.
- `update_pointer_and_check_value`: Increments a value at a pointer offset and checks a byte at a specific memory location, potentially updating other memory or a global variable.
- `set_global_state`: Checks a global variable and sets it to a fixed value or a sequence of values.
- `reset_object_state_2`: Checks a global variable and, if it meets certain criteria, resets a pointer or object state to fixed values.
- `modify_pointer_and_value`: Modifies a pointer or a value in a structure based on a type-check or a global variable value.
- `update_structure_state_and_position`: Checks a type-identifier byte within a structure and applies conditional offsets to a pointer or a value, while potentially updating a secondary pointer or a global flag.
- `modify_structure_fields`: Checks certain offsets within a structure and, if a condition is met, writes fixed values to other offsets in that structure.
- `initialize_global_state`: Sets a global variable to certain values and writes a byte to a memory location pointed to by a register.
- `initialize_table_values_2`: Sets a global variable to certain values and writes a sequence of bytes to a memory location pointed to by a register.
- `initialize_complex_state`: Performs a series of complex assignments to a global variable and several memory-based pointers or values if certain conditions are met.
- `reset_and_initialize_state`: Resets a global variable to zero and executes a loop that terminates immediately.
- `configure_blt_registers`: Calculates an offset and configures several hardware bitplane or block transfer registers before calling another routine.
- `return_from_subroutine`: The routine is empty and performs a single return instruction to the caller.
- `set_address_pointer`: Calculates a new address based on a global variable and stores it into a pointer-sized memory location.
- `iterate_and_update_pointer`: Performs a loop that updates a global variable using a calculated offset and stores a pointer-sized value into a memory location.
- `update_global_address_logic`: Calculates a new value for a global variable using a pointer, a table, and several conditional checks or offsets.
- `calculate_global_address`: Calculates a new value for a global variable based on a pointer and a table, using several conditional checks and offsets.
- `update_blt_blitter_parameters`: Calculates and writes new source or destination addresses and control parameters to the blitter registers.
- `update_blt_window_parameters`: Calculates and stores new window or or-pattern-related address or size values into a table or global variable based on conditional checks of a pointer's target byte.
- `process_sprite_attribute_table`: Iterates through a table of attribute or sprite-related data, performing conditional pointer-based lookups and updating a global variable or a byte in the table.
- `calculate_address_from_table`: Performs a series of checks on a table and calculates a new value for a global variable based on certain byte patterns.
- `increment_and_validate_pointer`: Increments a value at a memory location pointed to by a register and calls another routine before performing a conditional check against a value in a table.
- `execute_control_flow`: Performs a sequence of function calls and updates a global variable before checking a bit in a global variable and potentially calling another routine.
- `set_global_value`: Stores the value five into a global variable and returns a constant.
- `spin_forever`: Enters an endless loop that never returns.
- `loop_spin_forever`: Repeatedly calls a routine a fixed number of times, where each iteration calls another routine multiple times.
- `decrement_table_values`: Iterates through a table of words, decrementing certain bitfields or bytes within each word and storing the result back.
- `configure_blitter_registers`: Writes values to several Blitter hardware registers including control, mode, and area registers.
- `initialize_struct_from_input`: Checks a global variable and a pointer, then searches a table to initialize a structure with certain values and pointers.
- `update_bitmask_state`: Tests individual bits of a global variable and overwrites that same variable with the boolean result of each check.
- `update_global_bitmask`: Checks specific bits of a global variable and updates a global variable with a new value based on those checks.
- `copy_memory_block_with_offset`: Checks a global flag and, if non-zero, copies a block of memory using a pointer-based loop.
- `find_empty_slot_and_initialize`: Searches a table for a null entry and initializes that entry with a constant value and a pointer to a global variable.
- `find_empty_slot`: Searches a table for a null entry and initializes that entry with several values and a pointer to a global variable.
- `execute_unnamed_routine`: Calls another routine and returns a concatenated result of two registers.
- `set_global_counter_and_pointer`: Sets a global variable to a constant and stores a register value into a global variable.
- `update_audio_parameters`: Updates a global variable using a value fetched from a table and sets certain hardware-related or global values to constants.
- `update_table_entry_from_register`: Updates a table entry or pointer using a value from a register and modifies a global variable via a table lookup.
- `set_table_entry_from_register`: Sets a pointer or a table entry to a constant and updates a global variable using a value fetched from a table.
- `modify_memory_and_decrement`: Sets specific bits in a memory location and decrements a global variable, resetting it to a constant if it reaches zero.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `decrement_global_counter`: Decrements a global variable and resets it to a constant if it reaches zero.
- `update_dma_and_counter`: Modifies the DMA controller register and decrements a global variable, resetting it to a constant if it reaches zero.
- `branch_on_global_state`: Evaluates a global variable through a series of conditional branches and potentially modifies it or calls another routine.
- `update_global_value`: Checks a global variable and either increments or decrements its value and certain bytes within it based on specific conditional checks.
- `initialize_table_structure`: Searches a table for a zeroed entry and populates its fields with constant values, pointers, and or other derived data.
- `initialize_struct_element`: Searches a memory table for the first zero-valued entry and initializes a structure or object with a set of constant values, pointers, and offsets derived from another routine or memory location.
- `initialize_table_entry_from_pointer`: Searches a table for a zeroed entry and populates that entry's fields with data or pointers derived from a provided pointer.
- `update_sprite_attribute_table`: Calculates new attribute values and writes them to a table in memory, specifically updating sprite or attribute-related data or pointers.
- `update_sprite_attribute_data`: Calculates X and Y coordinates and attribute flags for multiple sprite entries, then writes them to a sprite attribute table.
- `read_joystick_fire_state`: Clears four shared state bytes, reads the joystick port and fire-button status and records them in shared variables.
- `decode_joystick_status_bits`: Decodes bits from a memory location into direction and button flags, storing each in a shared variable.
- `update_mouse_position_counters`: Reads the mouse port movement register and subtracts the change from two stored position counters, clamping each to its range.
- `clear_register_d0`: Clears the contents of a register.
- `move_register_d0`: Moves a value into a register.
- `move_byte_d0`: Moves a byte into a register.
- `return_register_value`: the routine returns the current value held in a data register.
- `mask_register_d0`: Performs a bitwise AND on a register with a mask and clears a register.
- `mask_register_d0_alt`: Performs a bitwise AND on a register.
- `tiny_ori_rts`: Small routine of 2 instructions that merges 0 into d0, then returns.
- `update_display_table`: Calculates an offset from a global variable and writes a series of bytes to a table or memory area, then updates a pointer or global variable.
- `update_object_properties`: Iterates through a fixed-size list of object structures to reset or modify specific fields if a certain type or flag is detected.
- `initialize_object_list`: Iterates through a list of pointers until a null-terminated entry is found, then initializes that entry with specific values and clears a separate table.
- `find_and_reset_object_entry`: Iterates through a table of structures to find the first entry where a specific field is zero, then initializes that entry and a separate pointer-based buffer with a fixed set of values and a global variable.
- `reset_object_state`: Checks a value at a pointer, potentially clearing a global variable, and then writes several fixed values to that pointer or its offset.
- `search_and_copy_table_entry`: Iterates through a table or memory range looking for specific sentinel or marker values, then copies a block of data or a pointer-based entry into a destination buffer.
- `update_blt_parameters_2`: Updates a global variable with a new offset or address before calling another routine.
- `update_blt_parameters`: Calculates a memory address or offset and updates a global variable before executing a loop that increments a pointer and calls another routine.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `compute_address_table`: Calculates several memory addresses based on a global variable and stores the lower and upper halves of these addresses into a series of global variables.
- `get_mouse_button_state`: reads the state of a mouse button or a hardware register or a bitmask from a specific hardware address.
- `clear_data_register`: Clears a data register by performing a series of logical operations to set it to zero, with a single intermediate state where it is set to one.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_table_values_2`: Iterates through a table and subtracts a constant value from each word stored there.
- `traverse_and_update_table`: Iterates through a table or list, checking for specific sentinel or termination values, and copies or updates data in a secondary table or buffer.
- `calculate_table_offset`: Calculates a new address or offset based on values in a pointer and a global variable, then performs conditional checks against a table.
- `update_entity_bounds_and_state`: Updates an entity's size or position or flags based on a lookup table and modifies a secondary table or pointer.
- `update_entity_state_and_bounds`: Iterates through a table of structures to check for specific type identifiers and update state flags or pointer offsets.
- `update_entity_state_and_bounds_2`: Iterates through a table of structures to check for specific type identifiers and update state flags or pointer offsets.
- `process_opcode_and_update_state`: Checks a byte or flag and branches to other routines or updates a global variable based on specific constant values.
- `set_memory_and_global`: Stores a constant byte into a memory location pointed to by a register and resets a global variable to zero.
- `check_bounds_and_set_flag`: Checks if a certain value or a pointer-based range falls within a specific boundary and sets a global variable to one or zero accordingly.
- `validate_and_initialize_table`: Checks a range of memory or a pointer-based boundary and, if certain conditions are met, initializes a table entry with a pointer to another routine and a constant byte.
- `initialize_table_entry`: Iterates through a table and, if a zero-value sentinel is found, initializes several fields in that table entry with constant values and copies data from another source.
- `increment_short_at_offset`: Adds a constant value to a short integer stored at an offset of two bytes from a pointer.
- `traverse_and_recurse_table`: Iterates through a table of bytes, performing recursive calls and updating a global variable using bitwise XOR and arithmetic or table-based lookups.
- `decrement_global_variable`: Decrements a global variable and sets it to a constant if it reaches a certain value.
- `decrement_global_variable_2`: Checks a global variable and, if it is neither certain constant values, decrements it by one.
- `update_global_state`: Checks a global variable and, depending on its value, potentially increments it or sets it to a constant value.
- `increment_global_variable`: Checks a global variable and, if it does not equal a certain constant, increments it by one.
- `modify_global_recursively`: Performs a recursive call while applying bitwise XOR, arithmetic, and conditional checks to a global variable.
- `recursively_modify_global`: Performs a recursive call and modifies a global variable using bitwise XOR and arithmetic operations based on its current value.
- `update_global_with_table_lookup`: Calculates an offset using a global variable and a table, then checks if a value from that table or another offset is less than the current global variable to potentially update a global flag.
- `calculate_table_index`: Calculates a complex index or offset using a global variable and a table lookup, then updates a global variable based on a comparison.
- `execute_recursive_counter_increment`: Executes a recursive routine that increments a global variable by a fixed amount if it is non-zero, or resets it to zero, or executes a sequence of recursive calls with conditional increments.
- `compute_pointer_and_check_bounds`: Calculates a pointer using a global variable and a table, then checks if a value at that pointer is within a certain range relative to a global variable.
- `update_global_sequence`: Resets a global variable to zero and then calls another routine twice, potentially adding certain constant values to the global variable if it is non-zero.
- `calculate_table_index_and_validate`: Calculates a pointer into a table or array using a global variable and an offset, then checks if the retrieved value falls within a specific range relative to the global variable.
- `update_state_via_table_lookup`: Performs a table-based lookup and updates a global variable to a specific constant based on the result of a comparison.
- `execute_nested_loops`: Executes a nested loop structure that increments a global variable until certain fixed counts are reached.
- `validate_pointer_range_and_lookup`: Checks if a pointer or value is within a specific range and performs a table-based lookup to potentially update a global variable.
- `write_pattern_to_memory`: Writes a repeating sequence of two specific bytes to a fixed memory location using an incrementing pointer.
- `copy_memory_blocks_with_offset`: Copies blocks of memory from a source to a destination, including certain offsets, and initializes a structure with constant values.
- `initialize_structure_and_increment_global`: Initializes a memory structure with constant values and increments a global variable through a series of calls.
- `copy_memory_blocks_2`: Copies certain words from a source pointer to a destination pointer, where the destination address is calculated from a global variable.
- `execute_repeated_routines`: Executes a sequence of calls to another routine a fixed number of times.
- `copy_swapped_data`: Copies or swaps data between a source and destination pointer, using an offset or a conditional swap based on a global variable.
- `initialize_table_values`: Writes a constant value to a range of memory and then writes a different constant or a value from a table to another range of memory.
- `write_alternating_values`: Iterates through a memory range, writing different pairs of 16-bit values to a memory location based on whether a character in a string is null.
- `initialize_and_rearrange_memory_table`: Initializes a global variable and then iterates through several null-terminated string tables, recursively calling itself.
- `copy_swapped_memory_blocks_2`: Iterates through a memory-based string and then copies or swaps bytes between a source and destination pointer based on a global variable.
- `copy_swapped_memory_blocks`: Iterates through a string and then copies or swaps bytes between a source and destination pointer based on a global variable's value.
- `copy_swapped_table_data`: Iterates through a string, then enters a loop that copies or swaps bytes between a source and destination pointer based on the value of a global variable, using a fixed number of iterations.
- `copy_swapped_data_offset`: Copies or swaps data between a source and destination pointer, where the source pointer or certain offsets are selected based on a global variable.
- `process_input_and_copy`: Checks an input character and either clears a range of memory or copies or swaps data between a source and destination pointer using an offset or a conditional swap.
- `perform_swapped_data_copy`: Copies bytes between a source and destination pointer, swapping which source offset is used based on the value of a global variable.
- `perform_swapped_data_copy_variant`: Performs a data copy or swap between a source and destination pointer, where the source or destination or certain offsets are selected based on a global variable.
- `update_global_variable`: Performs a series of conditional checks and updates a global variable to a new or fixed value.
- `copy_displaced_data_blocks`: Copies bytes from a source pointer to a destination pointer, including certain offsets, while incrementing a counter.
- `increment_and_reset_counters`: Checks and increments or resets a series of adjacent bytes in memory if they have not reached a certain value.
- `increment_global_bytes`: Checks if a global byte equals a certain value, then increments it or resets it to zero.
- `increment_global_bytes_subset`: Checks if a subset of global bytes equals a certain value, then increments or resets them to zero.
- `increment_remaining_global_bytes`: Checks if a remaining global byte equals a certain value, then increments or resets it to zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_work_tables_and_flags`: Sets several working variables and flag fields, scans a length table for its largest entry, and zeroes a chain of fourteen fixed-size entries.
- `advance_audio_channel_state`: Advances the playback state of four audio channels each time it is called: it writes the DMA enable bits, moves each channel's data pointer and volume along, and on a counted step loads the next pattern entry and clamps the per-channel values to their limits.
- `step_channel_sequence_entry`: Processes one entry of a sequenced channel stream: a special end marker stores the next delay value and returns, while any other entry looks up a table row by a nibble index and updates that channel's pointer, length and volume fields.
- `extract_nibble_from_state`: Masks a state variable down to four bits and shifts them into the low bits, then returns the caller's data register unchanged.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_memory_range`: Sets several memory locations to fixed values, finds a maximum value in a memory range, and then clears a range of memory or updates a pointer-based list.
- `update_attribute_table_2`: Updates a global variable and a table or structure using a pointer, while also modifying hardware registers for audio, DMA, and interrupts.
- `update_attribute_table`: Checks a bitmask in a structure and, if certain bits are set, updates a table or global variable with a new value and associated data or offsets.
- `modify_global_mask`: Performs a bitwise AND and a right shift on a global variable.
- `copy_memory_block`: copies a block of memory from a source address to a destination address using a loop that decrements a counter or checks a limit.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_large_block_to_memory`: Copies a large block of longwords from a source pointer into low memory in one loop, then calls the follow-up routine and returns the byte it produces.
- `copy_memory_blocks`: The routine checks a value at a pointer, copies a block of bytes from a source to a destination, and then copies a second block of bytes between different memory locations.
- `decode_packed_stream_loop`: Main unpacking loop for a flagged bit stream: it reads a flag bit, copies a backward run of earlier output when the flag is set, and repeats until the output pointer reaches its limit, calling the bit readers for each decision.
- `decrement_counter`: The routine decrements a local register or variable a fixed number of times before returning.
- `test_register_bits`: Tests a bitmask in a register and a low bit in another register without any side effects.
- `unpack_bit_stream`: The routine processes a bit stream by checking a flag bit and either copying a value from a pointer or updating a value using a bit-shifted or modified bitmask.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_routine`: An empty routine that does nothing and returns.
- `return_nothing`: the routine is empty and performs no operations before returning.
