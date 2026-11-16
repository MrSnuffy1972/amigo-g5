# Game 1460 (bill's tomato game (1992)(psygnosis)[cr skr][t +4 atx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01460). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (908944 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 46 raw-word lines |
| Functions named and described | pass, 125 of 125 named (0 author symbols), 125 of 125 described (20 by rule, 15 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1269 of 1269 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2482 of 2482 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 183585 |
| code | 8526 |
| data | 509468 |
| padding | 5 |
| unused | 207360 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Bill's Tomato Game (1992)(Psygnosis)[cr SKR][t +4 ATX].adf:bootblock | 1 | 65 | 3 |
| logged disk load from Bill's Tomato Game (1992)(Psygnosis)[cr SKR][t +4 ATX].adf boot-block load | 1 | 131 | 4 |
| logged disk load from Bill's Tomato Game (1992)(Psygnosis)[cr SKR][t +4 ATX].adf boot-block load | 1 | 155 | 2 |
| logged disk load from Bill's Tomato Game (1992)(Psygnosis)[cr SKR][t +4 ATX].adf boot-block load | 1 | 398 | 21 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 607 | 29 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1126 | 66 |

## Cases handled

- bill's tomato game (1992)(psygnosis)[cr skr][t +4 atx]/Bill's Tomato Game (1992)(Psygnosis)[cr SKR][t +4 ATX].adf: custom-format (non-DOS) disk, 1339 non-empty blocks: matched against track-loaded segments
- bill's tomato game (1992)(psygnosis)[cr skr][t +4 atx]/Bill's Tomato Game (1992)(Psygnosis)[cr SKR][t +4 ATX].adf: the boot block reads 16384 bytes from disk offset 878592 to an address (found statically); recovered as a unit
- bill's tomato game (1992)(psygnosis)[cr skr][t +4 atx]/Bill's Tomato Game (1992)(Psygnosis)[cr SKR][t +4 ATX].adf: the boot block reads 25600 bytes from disk offset 844800 to an address (found statically); recovered as a unit
- bill's tomato game (1992)(psygnosis)[cr skr][t +4 atx]/Bill's Tomato Game (1992)(Psygnosis)[cr SKR][t +4 ATX].adf: the boot block reads 161792 bytes from disk offset 11264 to an address (found statically); recovered as a unit
- runtime segment an address: 528 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 616 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- bill's tomato game (1992)(psygnosis)[cr skr][t +4 atx]/Bill's Tomato Game (1992)(Psygnosis)[cr SKR][t +4 ATX].adf: 3 logged loads cover 203776 bytes of the image
- bill's tomato game (1992)(psygnosis)[cr skr][t +4 atx]/Bill's Tomato Game (1992)(Psygnosis)[cr SKR][t +4 ATX].adf: blocks 13-14 (bytes 6656-7680) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.60)
- 2 packed track region(s), 487936 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Bill's Tomato Game (1992)(Psygnosis)[cr SKR][t +4 ATX].adf:bootblock

- `probe_memory_types_then_start`: Sends two device requests through the system library, checks memory types three times and stores a flag for the first match in a global, then starts the program through its entry routine.
- `setup_interrupts`: Initializes the interrupt system by clearing pending interrupts and enabling the blitter and copper interrupts, then sets up the interrupt vector table and enables CPU interrupts before returning.
- `call_start_routine`: Calls the start routine and returns.

### logged disk load from Bill's Tomato Game (1992)(Psygnosis)[cr SKR][t +4 ATX].adf boot-block load

- `initialize_boot_structure`: Copies 75 longwords from a routine address to a global variable location, then jumps to that global variable.
- `stub_undecoded_boot_routine`: The decompiler produced no pseudo-code for this 89-instruction routine, so only its three calls are known.
- `select_half_or_input_word`: Returns half of one register word when a second register holds one, otherwise returns the first word unchanged.
- `mask_table_word_by_index`: Looks up a word in a fixed table using an index register, then returns it ANDed with a second register value.

### logged disk load from Bill's Tomato Game (1992)(Psygnosis)[cr SKR][t +4 ATX].adf boot-block load

- `store_byte_return_shifted_long`: Reads a longword just before an argument pointer, stores its low byte into a target, and returns the longword shifted right by one with its top bit set.
- `shift_bits_with_refill_call`: Shifts a bit register right by one place per iteration for a counted number of iterations, calling a routine to refill it whenever it becomes zero.

### logged disk load from Bill's Tomato Game (1992)(Psygnosis)[cr SKR][t +4 ATX].adf boot-block load

- `call_single_unnamed_routine`: Calls one unnamed routine and returns.
- `stub_undecoded_two_call_block`: The decompiler produced no pseudo-code for this nine-instruction routine.
- `relocate_block_and_jump`: Copies a block of words from the argument address to a fixed destination, choosing forward or backward order by comparing addresses.
- `relocate_words_then_jump`: Copies a counted run of word pairs from the argument address to the destination, backward or forward according to comparison flags, then copies 75 longwords into a global and calls it.
- `copy_words_forward_and_jump`: Copies a counted run of word pairs forward from the argument address to the destination, then copies 75 longwords into a global and calls it.
- `clear_memory_and_set_display`: If a global variable does not equal a specific value, it calls another routine, then loops to zero out 15000 words starting at a global pointer, and finally sets the BPLCON0 register to 25088.
- `accumulate_vertical_position`: Performs a series of additions using values from three memory locations, then adds the current VPOS register value to the result and returns it.
- `calculate_checksum_from_memory`: Updates three memory locations using a sequence of additions, then returns a value calculated by XORing the high byte of the result with its low byte.
- `update_memory_locations`: Updates three memory locations by adding the value of the second location to the first, and then adding that result to the third.
- `perform_bit_shifts`: Performs a series of bit shifts and arithmetic operations on data in registers, including swapping word halves and adding values with carry propagation.
- `perform_complex_arithmetic_loop`: Executes a loop performing various bitwise shifts, multiplications, and subtractions on register values, with the loop controlled by a decrementing counter.
- `process_unknown_data`: Processes data through a series of operations involving memory access and conditional branching without clear functional context.
- `configure_audio_dma`: Clears a global variable, enables DMA, loops to perform a delay, then sets AUD3LCH, AUD3LEN, and AUD3VOL using values from a pointer in d0, enables DMA with 32776, and potentially resets audio registers if a value at an offset is zero.
- `copy_and_clear_data`: If a condition is met, it copies a longword from a source to a destination and increments the destination pointer, then clears a longword at a location calculated from a register and a pointer.
- `initialize_game_state_2`: Initializes game state variables and video registers, then performs conditional bit setting based on input and hardware status before return.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `execute_function_and_clear_bit_2`: Calls multiple functions, manages memory locations, handles conditional logic based on input values, and sets or clears specific bits in memory.
- `frag_jsr_bclr_bra`: Short code fragment that calls another routine, then clears bit 7 of a fixed memory address, then jumps to another point in the code.
- `apply_bitmask_to_graphics`: Calculates source and destination addresses based on d0, d1, and d2, then loops 18 times to perform bitwise operations, shifts, and swaps to modify pixel data in a buffer.
- `apply_shifted_bitmask`: Calculates source and destination addresses using d0, d1, and d2, then loops 18 times to perform bitwise OR, AND, and NOT operations on a buffer using values shifted by a bit count in d0.
- `apply_masked_graphics_with_function`: Calculates source and destination addresses, then loops 18 times to perform bitwise operations and calls another routine twice per iteration to modify pixel data in a buffer using both 32-bit and 16-bit logic.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_ten_setup_routines`: Calls four setup routines and then six further routines, and returns the register values unchanged.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `open_library_and_store_field`: Calls a library-opening routine through the system library, stores a field from the returned structure into a global, then clears another global and sets a third to a constant.
- `store_interrupt_mask_constants`: Stores a few constant words into globals, one of them an interrupt mask value, and returns.
- `disable_all_interrupt_sources`: Sets the top bit of a shared word, then writes the mask that, with the top bit clear, clears every interrupt enable bit.
- `initialise_bitplane_pointer_tables`: Clears a large block of memory and several tables, fills two tables with bitplane addresses stepping 40 bytes apart, calls a routine for each nonzero entry of a ten-entry table, then scans a zero-terminated list.
- `call_routine_over_grid_cells`: Calls a routine once per cell of a grid, ten rows of forty cells each.
- `copy_character_glyph_to_plane`: Maps a character code to one of two glyph tables according to its range, then copies that glyph's 32 rows, one byte per row, into a bitplane with a 40-byte row stride.
- `pulse_global_and_call`: Clears a global, checks one of its bits, pulses the global to one and back to zero, then calls a routine.
- `call_nine_routines_then_set`: Calls nine routines in sequence, calls one more, sets a global to 32 and returns the register values.
- `advance_step_counter_and_call`: Decrements a counter and returns while it stays non-negative.
- `tiny_eori_eori_eori`: Small routine of 3 instructions that exclusive-ors 18 into (a1), then exclusive-ors 20 into (a3), then exclusive-ors 18 into (a3).
- `update_direction_counter_on_timeout`: Shifts a global right by eight and decrements it.
- `advance_slot_state_machine`: Updates a timing counter from a global, then, when it has expired and the slot's state is set, moves that state between fixed values, updates a stored byte with a bounded sum on one transition, and calls routines for each transition.
- `call_routines_for_slot_state`: Scans forward through a 40-byte record until it finds a terminator character, then calls a routine a number of times that depends on the state stored for the slot.
- `unpack_iff_body_chunk`: Locates the header, colour map and body chunks by their four-character tags, then unpacks the body into a buffer, either copying bytes or expanding run-length packed bytes according to a flag.
- `increment_nonzero_counter_to_limit`: Calls three routines, then increments a global while it is nonzero and below 192.
- `call_routine_and_pass_registers`: Calls one routine and returns the register values unchanged.
- `generate_shaded_palette_row`: Builds a row of colour words from a level value: for low levels it writes solid values, for middle levels a graded value, and for higher levels a blend of table entries weighted by the level.
- `reset_timer_and_advance_pointer`: Decrements a counter and returns while it is nonzero.
- `setup_blitter_copy_registers`: Waits for the blitter, then stores constant masks, a control value, a size value and pointer values into globals that describe a blit operation.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `call_two_routines`: Calls two routines and returns.
- `set_blitter_mask_and_size`: Waits for the blitter, then stores a set of constant words and an address into globals that describe the blit mask and size.
- `set_blitter_destination_and_start`: Waits for the blitter, then writes constant values into globals for the destination modulo and size of an operation.
- `store_offset_pair_values`: Stores a value and the same value plus 160 into globals, both as a word and as a split longword.
- `compute_rotated_points_and_clamp`: Advances a table index, then for forty points multiplies coordinate pairs by table values to rotate them, clamps each result to a 320 by 120 area and stores it in an output array.
- `call_routine_if_values_differ`: Calls a routine when two word arguments are not equal, otherwise returns.
- `build_pointer_table_from_offsets`: Fills seven pointer globals from a base address and table values, then builds a running list of pointers from a count-prefixed offset table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_pointers_and_link_list`: Populates a table of global pointers with computed addresses and then walks a linked list of integers, copying each entry into a global array until the count is exhausted.
- `open_handle_and_report_result`: Increments a counter and obtains a handle through a routine.
- `run_three_library_calls`: Calls three routines through pointers and one further routine, then returns the register value unchanged.
- `bracket_call_flag_clear_set`: Clears a global to zero, calls a routine, then sets the same global to the all-ones value.
- `clear_block_and_call`: Clears a 776-byte block of globals, sets one global to zero beforehand, then calls a routine.
- `reset_audio_state_from_table`: Sets a few audio volume registers and globals to zero, copies twelve bytes of channel settings from a table entry into globals, clears a 776-byte block of globals and calls two routines.
- `return_doubled_global_byte`: Returns a word made of the same global byte in both halves.
- `compute_and_write_timer_count`: Divides a fixed large constant by a global divisor, using a fixed default when the divisor is zero, writes the two count bytes and a control byte to the timer block, then calls a routine and stores its result.
- `silence_audio_and_clear_block`: Clears a 776-byte block of globals, sets a global control byte, and zeroes the volume of audio channel zero, then returns its input value unchanged.
- `store_pair_and_set_flag`: Copies one stored word into another slot, stores the input word into a third slot, and sets a flag word to one.
- `reset_flag_and_clear_block`: Clears a flag word, zeroes a 776-byte block of globals, then calls a routine and returns.
- `service_tick_and_four_slots`: Calls a routine for each of four slot bytes that is nonzero and clears that slot.
- `select_channel_data_pointer`: Reads the parameter record for the current channel.
- `service_four_channel_pointers`: Calls two routines, then calls a routine for each of four non-zero channel pointers, returns the result of the last call when the fourth pointer is set, and sets a status word to a top-bit value.
- `advance_sequence_step`: Advances a step counter, and at its limit resets it, moves the sequence index forward with a loop point, and loads four longword values for the next step from a table.
- `load_channel_values_from_table`: For each of four enabled channels, loads a longword from a table, indexed by a stored index and the current step, into that channel's output slot.
- `dispatch_channel_command_by_nibble`: Switches on the command's high bits: some values return immediately, some store a channel value, and others adjust a playback counter and volume-like value before calling a routine.
- `indirect_jump_stub`: Jump stub: passes control to an address held in or computed from register a0, so the target is only known at run time.
- `store_word_into_channel_field`: Stores a word from a register into a field of a structure addressed through a register.
- `frag_move_rts`: Short code fragment that copies d1 to the memory at an offset from a4, then returns.
- `update_structure_fields`: Clears a word at offset 148 of the structure in a4, calculates a value from d1 by masking and shifting to store at offset 150, and calculates another value from d1 to store at offset 152.
- `frag_move_rts_2`: Short code fragment that copies d1 to the memory at an offset from a4 (2 times in a row), then returns.
- `frag_clr_rts`: Short code fragment that clears the memory at an offset from a4 (2 times in a row), then returns.
- `frag_tst_ble_cmp_bhi_lea_move_rts`: Short code fragment that tests d1, then branches if the comparison was less or equal, then compares d1 with 64, then branches if the comparison was higher (unsigned), then loads the address of a location in the code into a6, then copies d1 to the memory at a6, then returns.
- `frag_move_rts_3`: Short code fragment that copies d0 to the memory at an offset from a4, then returns.
- `frag_lea_move_rts`: Short code fragment that loads the address of a location in the code into a6, then copies d1 to the memory at a6, then loads the address of a location in the code into a6, then copies a location in the code to the memory at a6, then returns.
- `clamp_and_store_word`: Clamps a 16-bit value to a maximum of sixty-four, stores it in one location, and stores a bit-shifted version of it in another.
- `frag_lea_move_rts_2`: Short code fragment that loads the address of a location in the code into a6, then copies a location in the code to the memory at a6, then returns.
- `frag_tst_bne_bclr_rts`: Short code fragment that tests d0, then branches if the last result was not zero, then clears bit 1 of a fixed memory address, then returns.
- `frag_bset_rts`: Short code fragment that sets bit 1 of a fixed memory address, then returns.
- `frag_tst_ble_cmp_bgt_lea_move_rts`: Short code fragment that tests d1, then branches if the comparison was less or equal, then compares d1 with 16, then branches if the comparison was greater, then loads the address of a location in the code into a6, then copies d1 to the memory at a6, then returns.
- `decode_channel_command_byte`: Decodes the top byte of a channel command.
- `bind_channel_table_entry`: Looks up a table entry for the channel's index.
- `copy_channel_block_into_buffer`: Copies a block of table data for the channel's index into a destination buffer in 16-byte groups, records the destination and length in the channel state, then clears a status byte.
- `load_channel_parameters_from_table`: Stores a channel index and copies a set of timing, volume and position fields from the indexed table record into the channel state, clearing several counters.
- `copy_channel_timing_fields`: Copies timing, scaling and volume fields from a table record into the channel state and clears the related counters.
- `clear_channel_playback_state`: Zeroes several position, length and index fields of a channel state block and clears a status byte.
- `call_four_channel_routines`: Calls another routine four times in a row, once for each of four audio channels, and returns.
- `update_channel_tick_state`: Advances one channel by a tick: applies table-driven offsets to its level, interpolates toward a target, updates the playback counters and calls a routine when a sample table entry is set.
- `run_channel_effect_on_tick`: Counts down a channel's tick counter and, when it expires, reloads it and runs an effect chosen by a mode value.
- `dispatch_effect_via_jumptable`: Dispatches control to one of many routines selected by a runtime value using an unrecovered jump table.
- `negate_sample_byte_and_advance`: Negates one byte at a channel's stored offset, advances the offset and wraps it back to the start value when it passes the end.
- `copy_and_negate_sample_block`: Copies a block of sample bytes from a table into the channel buffer, negating the values as it goes, then advances the stored offset and wraps it at the end of the block.
- `copy_block_backward_then_advance`: Copies a block backward in longword steps, then advances the channel's stored offset and wraps it at the end of the range.
- `negate_block_backward_and_advance`: Writes negated longword values backward through a buffer, then advances the stored offset and wraps it at the end of the block.
- `add_constant_to_sample_range`: Adds a channel constant to each byte in a range of the channel buffer, in longword steps where aligned, then advances and wraps the stored offset.
- `shift_sample_range_one_byte`: Shifts a range of channel bytes one place toward the start by copying each byte from the next, then advances and wraps the stored offset.
- `add_bytes_with_alignment_check`: Adds bytes from two aligned memory regions, handling odd alignment cases by processing one byte at a time before switching to word-aligned processing.
- `process_memory_blocks`: Processes memory blocks by rotating and adding bytes from two pointers, then performs additional byte-wise additions with a loop counter.
- `frag_move_add_subq_bpl_bra`: Short code fragment that copies the memory at a1 (advancing a1) to d2, then adds d2 to the memory at a0 (advancing a0), then subtracts 1 from d1, then branches if the last result was not negative, then jumps to another point in the code.
- `copy_sample_byte_and_negate`: Copies one sample byte from a table into the channel buffer at the current offset, negates a byte at a computed position, then advances and wraps the stored offset.
- `call_routine_and_advance_offset`: Calls one routine, then advances the channel's stored offset and wraps it at the end of the range.
- `add_table_byte_across_range`: Adds one table byte to every byte in a range of the channel buffer, from a start index to an end index inclusive.
- `negate_and_accumulate`: Performs a series of operations including calling another routine, loading word values from memory locations, calculating an address offset, negating a byte at that address, and updating counters with conditional branching based on comparisons.
- `compare_and_adjust_bytes`: Compares bytes at indexed locations in two structures, adjusts the first structure's byte based on comparison results, and continues until reaching a specified limit.
- `frag_addq_move_cmp_bls_bra`: Short code fragment that adds 1 to d1, then copies d1 to an indexed entry from a0, then adds 1 to d0, then compares d0 with the memory at an offset from a5, then branches if the comparison was lower or same (unsigned), then jumps to another point in the code.
- `add_bias_and_advance_counter`: If the channel's counter is below its limit, adds a channel constant to a state value and increments the counter.
- `accumulate_and_compare`: Accumulates values from memory locations, compares them with a reference value, and updates a counter based on the comparison results.
- `frag_sub_move_cmpa_bls_bra`: Short code fragment that subtracts d0 from d1, then copies d1 to the memory at a0 (advancing a0), then compares a0 with a1, then branches if the comparison was lower or same (unsigned), then jumps to another point in the code.
- `frag_lea_move_eor_bra`: Short code fragment that loads the address of the memory at a4 into a0, then copies the memory at an offset from a4 to d1, then copies a fixed memory address to d0, then exclusive-ors d0 into an indexed entry from a0, then jumps to another point in the code.
- `process_byte_sequence_with_threshold`: Processes a sequence of bytes by comparing adjacent values, adjusting current bytes based on threshold comparisons, and updating a counter until reaching an end marker.
- `adjust_byte_based_on_comparison`: Compares bytes at adjacent memory locations, adjusts a byte value based on the comparison result, and modifies another byte based on a threshold condition.
- `frag_addq_cmpa_bge_bra`: Short code fragment that adds 2 to the memory at a0, then adds 1 to a0, then compares a1 with a0, then branches if the comparison was greater or equal, then jumps to another point in the code.
- `process_bytes_with_table_lookup`: Processes a sequence of bytes by comparing adjacent values, adjusting current bytes based on table lookup values, and updating a counter until reaching an end marker.
- `adjust_bytes_based_on_table_values`: Compares bytes with table values from a pointer, adjusts byte values based on comparison results, and modifies another byte based on a threshold condition.
- `advance_and_compare`: Increments a byte value at a memory location and advances a pointer, then compares the pointer with another address and branches if the first is greater than or equal to the second.
