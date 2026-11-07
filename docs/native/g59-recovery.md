# Game 59 (Xenon v1.1 0399): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00059). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (496009 top-level bytes in 10 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 13 raw-word lines |
| Functions named and described | pass, 210 of 210 named (0 author symbols), 210 of 210 described (10 by rule, 29 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2121 of 2121 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5525 of 5525 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 419401 |
| code | 19436 |
| data | 54023 |
| padding | 77 |
| unused | 3072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Xenon.Slave | 1 | 254 | 39 |
| logged disk load from Disk.1 | 1 | 132 | 3 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 334 | 5 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 40 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1498 | 62 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3261 | 88 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 6 |

## Cases handled

- Xenon_v1.1_0399/Xenon/Disk.1: raw disk image (447488 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- 7 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 907 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 905 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Xenon_v1.1_0399/Xenon/Disk.1: 7 logged loads cover 414208 bytes of the image
- Xenon_v1.1_0399/Xenon/Disk.1: chunks 4-43 (bytes 1024-11264) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.72)
- Xenon_v1.1_0399/Xenon/Disk.1: chunks 312-351 (bytes 79872-90112) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.80)
- Xenon_v1.1_0399/Xenon/Disk.1: chunks 1392-1407 (bytes 356352-360448) outside the filesystem hold data, not code (opcode density 0.013, entropy 5.10)
- Xenon_v1.1_0399/Xenon/Disk.1: chunks 1650-1671 (bytes 422400-428032) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.85)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `reset_channel_state_flags` (Disk.1 +an address loaded at an address (61952 bytes)): Clears nine state flag bytes and writes a sequence of values into the audio control shadow, ending with zero. Kind: hardware state reset/initialization. Pass path: the instruction immediately following the call (tst.b an address(a3)).
- `shift_serial_bit_register` (Disk.1 +an address loaded at an address (61952 bytes)): Shifts one bit between two 16-bit shift registers held in memory, taking the bit from bit six of a value built from a global byte. Kind: bit-stream/serial data validation. Pass path: the branch at an address (beq.b an address) is taken if the bit check fails, meaning the code continues normally if the bit pattern matches expectations.

## Functions


### Xenon.Slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `store_hook_and_call_table`: Calls routines through a structure pointer held in a register, stores a routine address and the structure pointer into two globals, then makes two further structure-table calls and a call through a global pointer.
- `patch_jump_if_matched`: Calls a routine from a table at a fixed offset and compares its short result with two fixed values.
- `save_struct_field_around_call`: Copies a longword from a fixed offset in a structure into a global, calls a routine through a stored structure table entry, then stores the data register word into a global.
- `save_second_field_around_call`: Copies a longword from another fixed offset in a structure into a global, calls a routine through a stored structure table entry, then stores the data register word into a global.
- `call_table_entry_conditionally`: Calls a routine through a stored structure table entry, then calls it again when a global flag is nonzero.
- `handle_flag_then_call_table`: Tests a byte flag in the data register.
- `write_word_and_call_table`: Writes back a word at a fixed offset.
- `execute_indirect_calls_2`: Performs a series of indirect calls using offsets from a pointer stored in a register.
- `rewrite_word_at_offset`: Rewrites the 16-bit value at a fixed offset before the address input with the same value, then returns.
- `copy_word_to_self`: Copies a word from a memory location back to the same location.
- `execute_indirect_calls`: Stores the routine's own address and a value from a register into global variables, then performs several indirect calls using offsets from those values.
- `call_unresolved_jump_pointer`: Calls through a pointer whose target could not be recovered from the listing, then returns.
- `execute_indirect_jump`: Performs an indirect jump to an address stored in a table.
- `read_global_masked_flags`: Reads a 32-bit global value, clears bits four to six of its lowest byte, and returns the result.
- `store_stack_word_to_global`: Copies the 16-bit value passed on the stack into a global variable and returns.
- `return_zero_constant`: Returns the constant zero and performs no other work.
- `set_flag_in_struct`: Sets bit one in the byte at offset 14 of the structure whose pointer is in address register A1.
- `return_immediately`: Returns straight away without touching any data or calling anything.
- `call_subroutine_and_return`: Calls a single unnamed subroutine with no arguments and returns when that call finishes.
- `store_register_to_global`: Copies the 32-bit value in data register D0 into a global variable.
- `load_and_jump`: Loads a value into a global variable and performs an indirect jump.
- `copy_struct_byte_to_base`: Copies one byte from the structure addressed by A0 at offset six into memory at offset 2048 from base register A6.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `check_global_constant`: Compares a global variable to a specific constant value and returns 4 if they match, otherwise returns 1.
- `initialize_pointer_chain`: Writes minus two into the slot addressed by A3, then stores the address of the next slot into offset 132 of the structure in A4 and offsets 50 and 54 of the structure in A2.
- `copy_nested_pointer_fields`: Reads the pointers held at offsets four and eight of the source structure and copies the 32-bit value at offset four of each pointed-to structure into offsets 50 and 54 of the destination.
- `return_minus_one_if_nonzero`: Returns minus one when the input value in D0 is non-zero and zero when it is zero, producing a full-width mask from a single test.
- `compute_table_derived_value`: Derives one value from a three-entry global table: the third entry divided by 50, added to 60 times the sum of the second entry and 1440 times the low word of the first entry.
- `dispatch_status_by_flags`: Clears the status byte at offset 31 of the structure in A1, then sets it to 29 when a flag at offset 64 is set.
- `toggle_byte_flag`: Flips the top bit of the byte at offset 65 in the structure addressed by A3.
- `store_inverted_bit_test`: Stores 255 at offset 32 of the structure addressed by A1 when the bit numbered by the byte at offset 67 of A3 is clear in a global mask, and zero when that bit is set.
- `return_immediately_2`: Returns at once without touching any data.
- `return_from_subroutine`: The routine is empty and performs no operations before returning.
- `signal_pending_bit_event`: Clears one bit in a global flag byte, selected by the byte at offset 67 of A3, and only when that bit was set does it mark bit one at offset 64.
- `return_immediately_6`: This routine is an empty function that performs an immediate return to its caller without executing any instructions or modifying any data.
- `return_immediately_3`: Returns at once without touching any data.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `stub_bad_instruction_data`: Contains only undecodable bytes, so the decompiled flow stops immediately and no behaviour can be recovered.

### logged disk load from Disk.1

- `load_executable_from_allocated_block`: Allocates a memory block through the system library, issues two library I/O requests on a request record, and clears a fixed block of longwords.
- `load_hunk_sections`: Parses an AmigaDOS executable hunk table: code and data hunks are copied word by word into memory, and BSS hunks are zero-filled.
- `return_immediately_4`: Returns at once without touching any data.

### logged disk load from Disk.1

- `initialize_channel_records`: Fills four 38-byte channel records with default flags, pointers and a handler address taken from a fixed table.
- `reset_channel_state_flags`: Clears nine state flag bytes and writes a sequence of values into the audio control shadow, ending with zero.
- `update_audio_channel_playback`: Advances playback on four audio channels, loading each channel's sample pointer and length into the audio registers when its countdown expires.
- `shift_serial_bit_register`: Shifts one bit between two 16-bit shift registers held in memory, taking the bit from bit six of a value built from a global byte.
- `advance_audio_channel_state`: Counts down a per-channel timer and, when it reaches zero, reloads four words from a source record into the audio channel's pointer, length, period and volume registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `clear_bit_and_set_flags`: Clears one bit in the byte addressed by A4, with the bit number taken from D4, and sets bits one and five in the byte at offset one of the structure in A2.
- `return_input_byte_unchanged`: Returns the byte in D0 without changing it.
- `set_fixed_bits_in_byte`: Sets bits zero, two, four, five and six of the byte in D0 and returns the result.
- `bitwise_or_byte`: Performs a bitwise OR operation between the contents of a data register and a constant value, returning the result.
- `bitwise_or_byte_2`: Returns the result of a bitwise OR between a data register and a specific byte value.
- `check_hunk_header_and_call`: Makes one indirect call through a function pointer, then checks whether the first longword is the AmigaDOS hunk header marker and begins hunk handling if it is.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_custom_chip_state`: Runs a fixed-length delay loop, stores the custom chip base address and several structure pointers in its stack frame, then calls helper routines and copies their results back into structure fields.
- `call_library_with_saved_registers`: Saves its registers on the stack, calls a helper and then two function pointers taken from a structure at fixed offsets, and restores the registers afterwards.
- `wait_for_raster_position_sync`: Polls the raster position register until its value passes a fixed threshold, then sets a flag word and stores a set of pointers into a structure.
- `clear_buffer_and_set_pointers`: Zero-fills a buffer whose length is taken from D0, then calls a helper.
- `initialize_structure_pointer_table`: Stores a long list of pointers into a structure, each one a base address plus a fixed offset, and fills a block of 64 words with the value 65280.
- `pack_condition_flags_into_global`: Builds a flag byte from the extend, negative and zero conditions of D0, stores it together with the upper word of D0 into a global variable, and increments that variable.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `advance_positions_until_limit`: Steps two 16-bit position fields forward by four until they pass the limit 159, calling a helper on each step.
- `advance_positions_and_wrap_row`: Adds eight to a position field and, after sixteen steps, wraps it to the next row and resets two fields to 160.
- `step_positions_then_countdown`: Steps two position fields by four until a helper returns zero or the position reaches 160.
- `finish_sequence_state_reset`: Sets a position field to 276 and clears a flag byte, then runs a helper in a loop while cycling a field through the values four down to one.
- `call_four_routines_in_sequence`: Calls four unnamed routines one after another and then returns.
- `call_four_routines`: Calls four different routines in succession.
- `fill_grid_with_sequence_values`: Waits briefly, then fills a grid of 16-bit cells with values from a pseudo-random sequence whose seed is read from two CIA register bytes.
- `fill_grid_with_sequence_values_2`: Repeats the grid-filling pattern of its neighbour: a short wait, then 16-bit cells are filled from a pseudo-random sequence seeded from CIA registers, looping until a flag or counter ends the fill.
- `call_helpers_and_clear_field`: Calls a helper, then two function pointers taken from a structure at fixed offsets, and clears a 16-bit field.
- `poll_until_flag_then_call`: Repeats a loop until the flag byte at offset 1349 becomes non-zero.
- `poll_and_execute_loops`: Repeats a polling loop that checks a flag byte at a specific offset and executes a series of nested routine calls based on that flag.
- `poll_until_flag_then_call_2`: Repeats the same polling loop as its neighbours, waiting on the flag byte at offset 1349 and calling helpers in fixed counts.
- `setup_blit_transfer_sequence`: Waits while a busy bit in the byte at offset two of the block in A3 is set, then writes a set of word fields and pointers.
- `walk_entry_lists_with_helper`: Clears three state fields, then walks two lists of three-word entries, calling a helper for each entry until a negative entry ends the list.
- `select_buffer_and_compute_offset`: Chooses one of two buffer pointers according to bit zero of a flag byte, then stores that pointer minus forty times the row number into a structure field.
- `process_hunk_records_with_helpers`: Makes ten helper calls while comparing record types against AmigaDOS hunk codes, which marks it as part of executable loading.
- `call_vtable_with_saved_frame`: Saves a frame of registers on the stack, makes a call through a function pointer taken from a structure, and restores the registers afterwards.
- `call_vtable_with_saved_frame_2`: Repeats the register-frame pattern of its neighbours: it saves registers, makes a call through a structure's function pointer, and restores them.
- `parse_amiga_dos_hunk`: Identifies the type of an AmigaDOS hunk by comparing a byte from a memory buffer against specific character constants.
- `identify_hunk_type`: Compares a byte from a memory buffer against specific character constants to determine the type of an AmigaDOS hunk.
- `return_stack_argument_unchanged`: Returns the eight-byte value passed on the stack without changing it.
- `format_number_as_decimal_text`: Fills a seven-byte buffer with the less-than character, then writes the decimal digits of the value in D0 by repeated division by powers of ten from one hundred thousand down, suppressing leading zeros.
- `format_number_as_decimal_text_2`: Continues a decimal conversion using a different register layout: it writes the digits of D1 by repeated division by powers of ten, skipping leading zeros, then writes the remaining value in D0 followed by a zero character.
- `advance_ring_buffer_entry`: Advances a pointer field through a ring of entries, wrapping at two fixed end addresses and skipping one position when a flag word is zero.
- `copy_fixed_block_to_buffer`: Copies a fixed block of 154 bytes from a constant source address into a buffer in the structure addressed by A5, then points an index field at the buffer start.
- `load_graphics_view_and_copper`: Calls two graphics library functions through the library base, one of them loading a view, and stores pointers in the structure.
- `install_copper_list_view_state`: Looks up a graphics structure through the library base and records its pointers, loading the view on the first pass.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `set_copper_registers`: Stores a constant value into a global variable and copies the address of another routine into two different global variables.
- `build_copper_entry_block`: Fills four list entries with computed addresses spaced 8960 bytes apart and stores the first entry's address.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `build_copper_entry_block_2`: Performs the same block build as its neighbour: four computed entries spaced 8960 bytes apart, a 58-longword copy from a global, and ten words copied into the destination at offset 170.
- `toggle_saved_pointer_state`: Toggles a saved-pointer state: when the flag at offset 132 is set, it restores the pointer from offset 128 into offset 438 and clears the state; otherwise it saves the pointer from offset 124 and sets the state.
- `setup_interrupt_server_node`: Initializes an interrupt-server node with a handler address and a priority byte of 127, then either records a stored handler pointer or calls a library function.
- `tiny_ori_2`: Small routine of 1 instruction that merges 32 into d1.
- `access_interrupt_request_register`: The hardware facts show this routine touches the interrupt request register and calls one tiny routine that sets a bit in D1.
- `reinstall_interrupt_server_node`: Calls a library function to detach the stored interrupt-server node when its pointer is zero, then sets the node's handler and priority fields.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `copy_block_to_two_buffers`: Calls another routine, then copies 32 words from a structure field into two destination buffers.
- `clear_structure_word_block`: Zeroes a block of 32 words starting at offset eight of the structure addressed by A5, then calls one other routine.
- `clear_sixteen_word_block`: Zeroes sixteen words starting at offset eight of the structure addressed by A5, then calls one other routine and returns the value in D0.
- `decrement_nibble_matching_entries`: For each level from fifteen down to one, decrements every one of 32 words whose low, middle or high nibble equals that level.
- `decrement_entries_by_level_count`: Repeats the nibble-matching decrement pass for a level count supplied in D6, calling three other routines per pass.
- `return_stack_value_after_call`: Calls one other routine and then returns the eight-byte value passed on the stack unchanged.
- `increment_entries_matching_nibbles`: For each level from fifteen down to one, increments the destination word for every one of 32 source words whose low, middle or high nibble is at least that level.
- `decrement_sixteen_entry_nibbles`: For each level from fifteen down to one, decrements every one of sixteen words whose low, middle or high nibble equals that level.
- `copy_words_into_block`: Copies 32 words from the source address passed in A0 into the structure block at offset eight, then calls one other routine and returns D0.
- `increment_sixteen_entry_nibbles`: For each level from fifteen down to one, increments the destination word for each of sixteen source words whose nibble is at least that level.
- `decrement_sixty_three_nibbles`: For each level from fifteen down to one, decrements each of 63 words in the block at offset 1486 whose low, middle or high nibble equals that level.
- `increment_sixty_three_nibbles`: For each level from fifteen down to one, increments each of 63 words in the block at offset 1486 whose nibble is at least that level, copying its source from a global table.
- `clear_and_copy_display_tables`: Clears a block of 64 words at offset 1486, copies a 364-byte table from a global into the structure at offset 1758, and copies five longwords into split high and low words at offset 1928.
- `copy_word_blocks_with_delay`: Copies a 32-word block between two offsets of the structure, then copies two further 16-word blocks, and finishes with a short delay loop.
- `increment_state_byte_return_zero`: Increments the byte at offset 1326 of the structure addressed by A1 and returns zero.
- `read_mouse_joystick_state`: Reads the mouse and joystick port movement registers, with the choice driven by a status field, and compares the bits against a previously stored value.
- `return_from_subroutine_3`: performs a sequence of instructions that results in a return from the current subroutine without modifying any registers or memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_byte_change`: Spins in a loop that waits on the byte at offset 1326 of the structure addressed by A5.
- `call_routine_count_times`: Calls one other routine repeatedly, counting down from the low word of D0.
- `store_cia_port_bytes`: Stores a combined 16-bit reading of two CIA port bytes into the structure field at offset 1072 and returns D0 unchanged.
- `generate_scaled_random_value`: Advances a 16-bit pseudo-random seed stored at offset 1072 with a multiply-and-add step, then maps the seed into the range from the low word of D1 up to the value in D2 and returns that result.
- `show_status_text_sequence`: Large routine of 321 instructions that calls seventeen unnamed routines and reads CIA registers.
- `dispatch_hunk_record_type`: Calls three other routines and compares record types against AmigaDOS hunk codes, which marks it as part of executable loading.
- `handle_press_fire_prompt`: Large routine of 329 instructions that calls nineteen unnamed routines and refers to the text PRESS FIRE TO PLAY and to the file name record.dat.
- `do_nothing`: This routine is empty and performs no operations.
- `set_status_bits_and_mask`: Sets bits two, four and five of the byte at offset 64 of the structure in A0, then returns the byte in D0 with bits zero to five set.
- `return_constant_four`: Returns the constant four and performs no other work.
- `swap_buffer_with_fixed_area`: Clears the flag word at offset 430, then exchanges 512 words between a fixed area at offset 460 and the buffer addressed by the pointer at offset 420.
- `toggle_buffer_swap_direction`: Toggles the flag word at offset 430 and exchanges 512 words between a fixed area and one of two buffers, with the direction chosen by the new flag value.
- `configure_blitter_row_loop`: Programs the blitter's mask, modulo, pointer, size and control registers, then runs a loop of 200 operations.
- `configure_blitter_for_transfer`: Programs the blitter's mask, modulo, pointer, control and size registers for a single transfer.
- `call_two_routines_in_sequence`: Calls two other routines one after another and then returns.
- `run_four_blit_transfers`: Programs the blitter's mask, modulo, control and source registers, then starts four transfers of the same fixed size.
- `advance_counted_table_entries`: Walks a table of counted entries: each entry's counter is decremented, and when it reaches zero the entry moves to its next value, wrapping at a stored limit.
- `call_fixed_address_five_times`: Calls the routine at a fixed address five times in a row and then returns.
- `sync_paired_state_bytes`: Compares two pairs of state bytes.
- `call_routine_with_address_pairs`: Makes two calls to another routine, each with a pair of address constants, and then makes three further calls with no arguments.
- `call_fixed_address_six_times`: Calls the routine at a fixed address six times in a row and then returns.
- `adjust_clamped_level_byte`: When three state bytes are clear, adds the value in D0 to the level byte at offset 1246 and clamps the result between minus one and twenty-four.
- `sync_state_bytes_countdown`: Compares ten state bytes against a mirrored set, and each byte that differs is copied across and triggers three calls.
- `emit_decimal_digits_of_value`: Converts the value in D7 to decimal by subtracting powers of ten from one hundred thousand down to one.
- `emit_decimal_digits_from_field`: Converts the 32-bit value stored at offset 888 of the structure in A5 into decimal digits.
- `classify_digit_character`: Converts the low byte of D2 by subtracting the ASCII zero code, remapping it through a lookup table when a flag is set.
- `concatenate_registers_2`: Combines the contents of two 32-bit registers into a single 64-bit value.
- `concatenate_registers`: Combines two 32-bit registers into a single 64-bit value.
- `emit_string_characters`: Walks a zero-terminated string, skipping carriage-return characters, and calls another routine for each remaining character.
- `process_string_sequence`: Iterates through a null-terminated string and calls another routine for each character unless it is a carriage return.
- `walk_byte_data_with_handlers`: Large routine of 301 instructions that walks byte data and calls eight unnamed routines.
- `blit_rows_with_busy_waits`: Programs the blitter's mask, modulo, control and size registers, then runs 169 transfers.
- `run_guarded_blit_transfers`: When the flag byte at offset 1234 is not one, programs the blitter's mask, modulo and control registers and starts four transfers spaced 8960 bytes apart.
- `run_guarded_blit_transfers_2`: Performs the same four-transfer blit as its neighbour, but only when the flag byte at offset 1234 is not fourteen.
- `call_one_routine_and_return`: Calls one other routine and then returns.
- `call_one_routine_and_return_2`: Calls one other routine and then returns.
- `run_paired_blit_transfers`: Programs the blitter's mask, modulo, control and source registers, then runs four transfers from two pointers passed in registers.
- `call_subroutine_and_return_2`: Calls another routine and returns.
- `execute_double_calls`: Calls another routine twice and returns.
- `run_blit_transfer_series`: Programs the blitter's mask, modulo and control registers, then runs 192 transfers from a pointer, waiting on the busy flag around each one.
- `fill_longs_with_minus_one`: Fills 2240 longwords, starting at offset 35360 of the structure addressed by A1, with the value minus one.
- `nop_padding_sequence`: This routine consists of a sequence of no-operation instructions that perform no functional task.
- `run_fixed_size_blit_loop`: Programs the blitter's mask, modulo, control and size registers, then runs 56 transfers of a fixed size.
- `stub_unrecovered_body_return`: The decompiled output shows no work for this routine and returns its register inputs unchanged.
- `scan_bit_pattern_table`: Scans a table of shifted 16-bit words while tracking a running value in D5, calling two other routines within the loop.
- `process_bit_stream_loop`: Runs a loop that shifts bits out of a value held in an input register and calls two other routines.
- `accumulate_sum_with_calls`: Loops adding two 16-bit values into a running sum and masking the result, calling three other routines per pass.
- `run_counted_state_loop`: Decrements a loop counter on each pass and, when its low byte reaches minus four, clears two state bytes at offsets 12 and 13 of the structure in A0.
- `clear_two_state_bytes`: Zeroes the two state bytes at offsets 12 and 13 of the structure addressed by A0 and returns.
- `store_three_state_bytes`: Stores the low bytes of D5, D6 and D7 into offsets 20, 12 and 13 of the structure addressed by A0.
- `reset_playback_fields`: Sets a word at offset 1080 to six, sets a flag byte at offset 1242 to one and a counter byte at offset 1245 to 24, and copies two words from offsets 1004 and 1006 into offsets 996 and 998.
- `clear_playback_state_fields`: Clears the flag byte at offset 1242 and the counter byte at offset 1245, then copies two words from offsets 1000 and 1002 into offsets 996 and 998.
- `select_state_value_by_flags`: Chooses a value from several state fields according to flag bytes and a level byte, then calls two other routines.
- `configure_blitter_row_draw`: Checks state bytes, then programs the blitter's masks, modulo, pointers, control and size registers and makes one call.
- `set_flag_bit_and_call`: Sets bit zero of the byte at offset 1338 of the structure addressed by A5, then calls one other routine.
- `configure_blitter_for_position`: Computes a blit address from two position fields and a row stride, checks a column threshold, and programs the blitter's full register set before waiting on it.
- `update_object_state_and_position`: Modifies a flag byte at a specific offset and calculates new position values using nibble extraction or results from subroutines.
- `advance_object_position_and_flags`: Adds per-object signed step bytes to two position words of the structure at A0, and sets a third field when a bounds comparison holds.
- `advance_object_position_and_flags_2`: Applies the same position update as its neighbour, with the bounds comparison made without the offset subtraction.
- `stub_bad_instruction_data_2`: Contains overlapping and undecodable instructions as the decompiler reports them, with a single call to another routine.
- `stub_bad_instruction_data_3`: Holds a few instructions that the decompiler flags as overlapping or undecodable, and makes one call to another routine.
- `scan_table_entries_with_update`: Loops over a table of 16-bit entries, comparing and updating table values with register arithmetic, and calls one other routine.
- `mark_indexed_byte_full`: Calls one other routine, then writes the value 255 into a table byte at offset 1409 plus the index held in the byte at offset 1337 of the structure addressed by A5.
- `scan_table_entries_with_update_2`: Performs the same table scan as its neighbours, starting from a zero counter, and calls one other routine.
- `scan_table_entries_with_update_3`: Performs the same table scan as its neighbours, starting from a counter of 50, and calls one other routine.
- `scan_table_entries_with_update_4`: Performs a table scan similar to its neighbours, making one call to another routine.
- `clear_block_at_pointer`: Zeroes 2200 bytes starting at the address stored at offset 544 of the structure addressed by A5, then returns.
- `count_clear_flagged_entries`: Walks entries 22 bytes apart from a base pointer and calls another routine for each entry whose bit two is clear.
- `count_set_flagged_entries`: Walks entries 22 bytes apart from a base pointer and calls another routine for each entry whose bit two is set.
- `call_routine_until_marker_x`: Calls another routine repeatedly until it returns the character X, then returns.
- `call_routine_until_bracket_char`: Calls another routine repeatedly until it returns the left bracket character, then returns.
- `clear_flag_from_indexed_table`: Clears bit zero of a byte whose address comes from a table indexed by the value in D0, then makes conditional calls.
- `measure_string_and_place_entry`: Counts the entries of a zero-terminated sequence of two-byte values stored at offset 1389 of the structure, then computes a table address from the count and calls two other routines.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_store_longword`: Clears bit zero of the byte at offset 1338, computes a value from D7, and stores a longword through a pointer while calling two other routines.
- `set_flag_compute_nibble_offset`: Sets bit zero of the byte at offset 1338 and derives a value from the low and high nibbles of D1 combined with D7.
- `set_flag_and_compute_position`: Calls one other routine, sets bit zero of the byte at offset 1338, and computes a value from D7 and the call's result.
- `clear_flag_compute_nibble_offset`: Clears bit zero of the byte at offset 1338 and derives a value from the nibbles of D1 combined with D7.
- `compute_position_and_clear_flag`: Calls one other routine, clears bit zero of the byte at offset 1338, and computes a position value from D7 and the call's result.
- `find_or_claim_blit_slot`: Searches a list of longword slots starting at offset 3430 for an empty or matching entry, claims an empty one, and then programs the blitter registers.
- `set_flag_after_fixed_call`: Calls an unnamed routine at a fixed address, sets bit zero of the byte at offset 1338, and computes a value from D6 and D7.
- `disable_interrupts_around_call`: Saves the interrupt-control shadow, writes 16383 to disable the interrupt bits, records a value in a global, calls another routine, and then restores the shadow with its enable bit set.
- `configure_cia_interrupt_registers`: Writes zero and the value 136 to two CIA control registers and stores a handler address in a global variable.
- `poll_cia_serial_transfer`: Checks status bits in a CIA register and, when a transfer flag is set, clears the matching flag bit and rotates a value held in a byte.
- `initialize_device_state_bytes`: Stores fixed byte values at offsets 1024, 1280, 3072 and 3584 from the base address in A0, sets bit zero of the byte at offset 3584, and calls two other routines.
- `submit_io_request_block`: Fills in an I/O request block's command and length fields, then submits it through the system library's DoIO call with two flag bits set.
- `handle_record_with_stack_buffer`: Large routine of 207 instructions with four parameters and a local stack buffer.
- `copy_bytes_between_addresses`: Copies a block of bytes, whose length is given in the third argument, from one address to another.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately_5`: Returns at once without touching any data.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning.
- `return_immediately_7`: This routine is empty and performs no operations before returning to its caller.
- `return_immediately_8`: This routine performs no operations and returns immediately to its caller.
- `return_immediately_9`: This routine is empty and performs no operations before returning to the caller.
- `do_nothing_2`: the routine is empty and performs no operations before returning.
