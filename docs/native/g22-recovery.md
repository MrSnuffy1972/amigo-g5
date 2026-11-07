# Game 22 (WWFWrestleMania v1.0 0205): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00022). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3674913 top-level bytes in 16 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 14 raw-word lines |
| Functions named and described | pass, 166 of 166 named (0 author symbols), 166 of 166 described (8 by rule, 69 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2187 of 2190 (99.86%) |
| Recovered without load address (counted in proof 1, flagged) | 3840 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3345 of 3345 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 443825 |
| code | 12530 |
| data | 3189083 |
| padding | 35 |
| unused | 29440 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 289 | 18 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 2411 | 119 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 397 | 20 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 228 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 5 |
| Disk.2 bytes 400640-404480 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- WWFWrestleMania_v1.0_0205/WWFWrestleMania_v1.0_0205_hdf/WWFWrestleMania/Disk.1: raw disk image (814080 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- WWFWrestleMania_v1.0_0205/WWFWrestleMania_v1.0_0205_hdf/WWFWrestleMania/Disk.2: raw disk image (814080 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- WWFWrestleMania_v1.0_0205/WWFWrestleMania_v1.0_0205.hdf: hard-disk image (AmigaDOS OFS, 3936 blocks): its filesystem parses and all 8 files in it are byte-identical to the extracted tree WWFWrestleMania_v1.0_0205_hdf, which is recovered file by file; no block outside the filesystem holds data
- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- 5 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address (f150): 354 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 12 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f150): 15 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- WWFWrestleMania_v1.0_0205/WWFWrestleMania_v1.0_0205_hdf/WWFWrestleMania/Disk.2: bytes 400640-404480 look like code no run loaded: recovered position-independently (load address unknown)
- WWFWrestleMania_v1.0_0205/WWFWrestleMania_v1.0_0205_hdf/WWFWrestleMania/Disk.1: 5 logged loads cover 465920 bytes of the image
- WWFWrestleMania_v1.0_0205/WWFWrestleMania_v1.0_0205_hdf/WWFWrestleMania/Disk.1: chunks 1340-1559 (bytes 343040-399360) hold a BK: ByteKiller stream (56320 -> 32000 bytes)
- WWFWrestleMania_v1.0_0205/WWFWrestleMania_v1.0_0205_hdf/WWFWrestleMania/Disk.1: chunks 1620-2699 (bytes 414720-691200) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.42)
- WWFWrestleMania_v1.0_0205/WWFWrestleMania_v1.0_0205_hdf/WWFWrestleMania/Disk.2: 1 logged loads cover 3840 bytes of the image
- WWFWrestleMania_v1.0_0205/WWFWrestleMania_v1.0_0205_hdf/WWFWrestleMania/Disk.2: chunks 0-1364 (bytes 0-349440) hold a BK: ByteKiller stream (349440 -> 91032 bytes)
- WWFWrestleMania_v1.0_0205/WWFWrestleMania_v1.0_0205_hdf/WWFWrestleMania/Disk.2: chunks 1380-1564 (bytes 353280-400640) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.86)
- WWFWrestleMania_v1.0_0205/WWFWrestleMania_v1.0_0205_hdf/WWFWrestleMania/Disk.2: chunks 1620-3179 (bytes 414720-814080) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.56)
- 1 never-loaded region(s), 3840 bytes, recorded as UNVERIFIED DATA (under 1% of bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `clear_memory_and_probe_patterns`: Clears a block of memory with zero words up to a fixed end address, calls a routine through a library vector, then compares memory words against fixed instruction-encoding patterns to choose a value that is stored in a global, and makes two more vectored calls.
- `call_library_vector_stub`: Calls one routine through a pointer at a fixed offset from a stored base and then ends in an undecodable tail, so no further behaviour is recovered.
- `adjust_bcd_and_store`: Makes two helper calls and one vectored call, applies a decimal adjustment with a borrow test, stores a combined word, and decrements a longword in the record; the decompiled tail does not return normally.
- `call_vector_stub_tail`: Calls one routine through a pointer at a fixed offset from a stored base and ends in an undecodable tail, so no further behaviour is recovered.
- `call_indirect_vector_entry`: Calls one routine through a pointer at a fixed offset from a stored base and returns the registers unchanged.
- `patch_blitter_moves_to_calls`: Scans a range of memory and rewrites move instructions that write the blitter size register into subroutine calls to fixed addresses, including a call to the blitter wait routine for one of the patterns.
- `store_longword_and_masked_word`: Writes the incoming longword to a target address and, when that address is below a fixed bound, writes a word combined from two inputs into the word after it.
- `tiny_addq_rts`: Small routine of 2 instructions that adds 2 to a3, then returns.
- `set_global_longword_constant`: Stores a fixed 32-bit constant in a global longword and returns.
- `compute_stack_differences_table`: Initialises a global longword, fills a local six-entry table with differences between successive stack values and a doubling constant, and stores a fixed final longword.
- `call_vector_per_row_stride`: When a global state byte is 2 or 3, computes a start address from two inputs and calls a routine through a vector once per row, advancing the address and source pointer by fixed strides; otherwise it makes a single vector call.
- `stub_infinite_loop`: Contains only an infinite loop and no other operation.
- `start_blit_and_wait_flag`: Writes the incoming value to the blitter size register, then waits while a global flag bit is set.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `depack_stream_backwards`: Unpacks a compressed stream read backwards from the end of the buffer: a bit reader supplies flag bits, literal bytes are copied, and back-references are decoded and copied in reverse, returning a check value or a failure value if the output pointer does not reach the expected start.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `shift_bits_from_packed_stream`: Shifts a bit buffer right by a count, refilling it from the previous longword of the packed stream with a marker top bit whenever the buffer empties.

### logged disk load from Disk.1

- `adjust_bcd_and_store_copy`: Same decompiled body as the earlier BCD adjust routine: a decimal adjustment with a borrow test, a stored combined word, and a decremented longword, with a tail that does not return normally.
- `copy_indexed_table_rows`: Walks an index list until a negative terminator and, for each index, copies a fixed group of words from a source table into the destination, repeating one source word for several outputs.
- `call_routine_four_times`: Calls one routine four times in a loop and returns.
- `step_command_timer_state`: Counts down a timer field in a record; when it reaches zero it reloads the timer and applies one of ten commands selected by a command field, such as toggling sign bits, swapping or copying fields, or adjusting a counter within bounds.
- `step_command_timer_repeat`: Applies the same command logic as the preceding timer routine, but without the initial zero check: it decrements the timer and runs the command when the timer reaches zero.
- `apply_timer_command_step`: Reloads the timer field from its reload value and applies one of ten commands selected by a command field, using the same command set as the timer routines before it.
- `apply_record_command_from_register`: Applies one of ten commands selected by the incoming command register to the record, using the same command set as the earlier timer routines.
- `apply_record_command_from_two`: Variant of the command switch that handles commands two to nine; the copy has no case for command one.
- `apply_record_command_from_three`: Variant of the command switch that handles commands three to nine on the record.
- `apply_command_on_record_fields`: Command switch for commands four to nine, operating on record fields addressed by byte offset rather than by array index.
- `apply_command_five_to_nine`: Command switch for commands five to nine on the record, with the same effects as the matching cases of the earlier switches.
- `apply_command_six_to_nine`: Command switch for commands six to nine on the record, with the same effects as the matching cases of the earlier switches.
- `apply_command_seven_to_nine`: Command switch for commands seven to nine on the record, with the same effects as the matching cases of the earlier switches.
- `apply_command_eight_or_nine`: Command switch that handles only commands eight and nine, swapping and sign-toggling two fields.
- `apply_command_nine_only`: Handles only command nine, which toggles sign bits and swaps two fields; any other command returns without change.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `advance_two_axis_oscillator`: Counts down a delay field and, when it expires, reloads it and steps two 4-bit counters that bounce between limits; the counter values are then added to or subtracted from two position words in the target record, depending on a sign flag.
- `copy_tile_to_two_buffers`: Copies an eight-row block of source bytes into two destination buffers at fixed offsets, writing each row with a stride of 160 bytes and duplicating every source byte into both buffers.
- `copy_rows_to_two_buffers`: Copies sixteen rows of words from a source block into two destination buffers, writing each row with a stride of 80 bytes.
- `fade_palette_entries_toward_target`: Steps sixteen stored colour entries, each split into three 4-bit components, one step toward the matching components of an incoming colour table, incrementing each component that differs; it then ends in an endless loop with no return.
- `fade_palette_32_entries`: Variant of the preceding palette step for thirty-two entries, with the same per-component step and the same endless loop.
- `load_palette_and_fade_down`: Copies thirty-two words from a source table into the palette buffer, then decrements each 4-bit colour component toward zero, and ends in an endless loop.
- `load_second_palette_fade_down`: Same routine as the preceding palette loader but copying from a different source table.
- `clear_two_screen_buffers`: Zeroes two buffers at fixed addresses, each 32000 bytes long, in one pass of 8000 longwords.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `call_per_table_entry`: Calls one helper, then calls another helper for each byte of a table until a terminator value of minus one is reached, and finishes with one more helper call.
- `process_byte_script_until_end`: Walks a byte list up to a terminator; each byte other than a marker value is passed to a helper, and after the terminator it calls a helper and then ends in an endless loop.
- `configure_display_registers`: Writes the bitplane modulo and control values and a copper list pointer into the display registers, calls helpers while a flag is set, and finishes by writing two further control values.
- `call_two_then_halt_loop`: Calls two helper routines, then runs an endless loop with no return.
- `tiny_ori_2`: Small routine of 1 instruction that merges 1 into d1.
- `depack_stream_reading_bits`: Variant of the backwards depacker that reads flag bits through a bit reader and fetches literal and back-reference lengths with a separate bit-reading helper; it returns a check value, or a failure value if the output pointer does not reach the expected start.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `copy_two_tables_then_halt`: Copies two table blocks into an offset region in rows of twenty words, walks a byte list calling a helper for each value, sets a global flag, and ends in an endless loop.
- `configure_display_with_checks`: Writes modulo, control and copper pointer values for the display, checks a global flag through up to three helper calls, and then writes the final display values.
- `clear_record_blocks_then_halt`: Zeroes twelve fixed-size record blocks by writing longwords and words at set offsets, calls a helper before and after, and ends in an endless loop.
- `copy_block_and_run_grid`: Copies 8000 longwords from the buffer at the global pointer to another buffer, then calls two helpers in a grid of sixteen by eight passes and ends in an endless loop.
- `read_script_byte_dispatch`: Reads one byte from a script pointer and advances it.
- `stub_endless_loop`: Contains only an endless loop, reached from a single call site, with no other operation.
- `or_nibble_table_into_plane`: Uses the low and high nibbles of a source byte to select two table entries of four words each, then ORs four rows of table bytes into the destination with a stride of 160 bytes.
- `or_nibble_table_offset_copy`: Same nibble-table OR as the preceding routine, writing one byte further into the destination.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `cycle_bounded_offset_state`: Toggles a global bit, calls several helpers, and decrements a state counter; when the counter reaches zero it steps a global offset by 192 within a range of 0 to 1536, cycling it through three values.
- `copy_interleaved_word_rows`: Copies twenty-four groups of four words from a source table into a destination with a stride of 80 bytes, writing each word to a separate row.
- `copy_interleaved_rows_eight`: Copies fifteen groups of eight words from a source table into rows with a stride of 80 bytes.
- `copy_rows_into_two_targets`: Copies sixteen groups of six words into two destinations at once, writing each group to both destinations with a stride of 80 bytes; the final words of each group are single bytes.
- `copy_byte_rows_two_buffers`: Copies twelve bytes from each of sixteen rows of a source into two destination buffers, writing each row with a stride of 160 bytes.
- `walk_words_then_halt`: Advances a pointer through 8000 words, reading each word and writing it back in place, then ends in an endless loop.
- `shift_and_mask_scroll_rows`: Copies a sixteen-word palette block, then derives source and destination addresses from a position value; for sixteen rows it combines source words with a bit mask built from a shift value, and a second pass moves words between rows at a wider offset, shifting the image data by the position.
- `verify_sequence_or_halt`: Writes a sequence of nine values and a palette block, then compares a global value against a fixed chain of values; when every comparison matches it returns, otherwise it calls eight helpers and ends in an endless loop.
- `call_one_or_twelve_helpers`: Tests a global value: when it is zero it calls one helper and returns; otherwise it calls twelve helpers in sequence.
- `clear_block_rows_at_offset`: Clears thirty-two rows of eight longwords at an address computed from two position inputs, writing zero into each row.
- `masked_tile_copy_with_shift`: Copies thirty-two rows of tile data into a work area, then combines each row with a bit-shifted mask derived from the position, using AND and OR operations, and writes the combined words into the destination rows.
- `advance_position_with_velocity`: Returns early when a stored position equals a fixed reference pair; otherwise it adds a velocity to the position, subtracting a size value when a coordinate passes its limit, and updates the stored position.
- `call_subroutine_four_times`: Calls one routine four times in succession and returns.
- `compute_offset_from_reference_point`: Compares two input coordinates with the reference point 84 and 144, records a signed direction for each axis, and stores the larger distance from the reference as the step size.
- `initialise_display_and_copper`: Sets up the display: writes the colour zero register, display window and fetch registers, bitplane modulo and pointer values and the copper pointer, enables the DMA and interrupt registers, runs a helper-driven wait until a flag clears, clears a buffer, and ends in an endless loop.
- `copy_palette_to_two_tables`: Counts a global value down or calls a helper otherwise, then copies a 32-word palette block into two fixed tables in parallel and sets the global to sixteen.
- `decode_joystick_port_state`: Derives direction bits from joystick port movement data by exclusive-or of paired bits, adds further bits for the fire input and the other axis, and stores the combined state in a global byte.
- `swap_buffer_pointer_pair`: Toggles a global flag and writes one of two buffer addresses into five pointer slots, alternating between the two addresses on each call.
- `set_buffer_pointer_slots`: Writes one fixed buffer address into five pointer slots.
- `tiny_ori_3`: Small routine of 1 instruction that merges 0 into d7.
- `fill_pointer_list_with_stride`: Fills four pointer slots in a list with a starting address that increases by forty bytes per slot.
- `reset_audio_channels_and_call`: Clears per-channel volume and state words, sets the audio control register, computes a table pointer from an index byte, and calls a routine through a pointer.
- `dispatch_indexed_handler`: Stores fixed values into fields of a state block, reads a byte at an index from a table to derive pointers, and calls a handler through a stored pointer.
- `dispatch_indexed_handler_b`: Sets two fields of a state block to fixed values, reads a byte from the input pointer, derives a pointer from it, and calls a handler selected from a table by a following byte.
- `update_state_and_call`: stores values into a structure at an offset from a pointer, updates a pointer field, and calls a function through a pointer in that structure.
- `reset_state_bytes_block`: Clears nine state bytes, writes a word of eight into a state field, sets two more state bytes to 255, and returns.
- `clear_audio_state_fields`: Sets a global value to fifteen, clears the four audio channel volume registers and one state byte, and returns.
- `store_byte_and_dispatch`: Stores one byte from the input pointer into a state field, then calls a routine chosen from a table by the following byte.
- `store_byte_and_dispatch_b`: Stores one byte from the input pointer into a different state field, then calls a routine chosen from a table by the following byte.
- `store_two_bytes_dispatch`: Stores two bytes from the input pointer into state fields and a derived pointer, then calls a routine chosen from a table by the third byte.
- `store_bytes_and_call`: stores two bytes from a source pointer into a structure and calls a function using a value from a table.
- `dispatch_byte_table_handler`: Calls a routine chosen from a table by the byte at the input pointer.
- `store_pointer_and_dispatch`: Stores two bytes and a pointer from the input into state fields, sets a flag byte, then calls a routine chosen from a table by the byte the pointer addresses.
- `store_bytes_and_dispatch`: Stores three bytes and a longword from the input into state fields, then calls a routine chosen from a table by the fourth byte.
- `countdown_and_load_table_values`: Decrements three countdown bytes; when one goes negative it calls a handler chosen from a table, otherwise it loads four table values into record fields and sets a flag bit.
- `countdown_and_load_table_copy`: Same body as the preceding countdown routine.
- `scan_command_bytes_until_limit`: Stores a fixed word, then reads bytes from a command list, skipping entries that match a stored value, until a byte falls below a limit; it then runs the countdown and table-load steps of the countdown routines.
- `resolve_table_entry_or_reset`: Reads a table entry from a pointer: when the entry is negative it clears the state and returns; otherwise it stores a derived pointer and calls a handler selected from a table by the entry's value.
- `scan_command_list_with_init`: Initialises several fields from input bytes and a word, then scans a command list as the preceding scan routine does until a byte falls below a limit, followed by the countdown and table-load steps.
- `dispatch_entity_command`: Stores two bytes and a pointer into a data structure at specific offsets and then performs an indirect call to a routine using a value from a global table.
- `dispatch_single_byte`: Stores a single byte into a data structure at a specific offset and then performs an indirect call to a routine using a value from a global table.
- `dispatch_single_byte_offset`: Stores a single byte into a data structure at a different specific offset and then performs an indirect call to a routine using a value from a global table.
- `dispatch_with_calculated_pointer`: Stores two bytes into a data structure, calculates a new pointer by adding an offset to a base address, and then performs an indirect call to a routine.
- `dispatch_with_calculated_pointer_alt`: Stores two bytes into a data structure, calculates a new pointer using a different offset, and then performs an indirect call to a routine.
- `dispatch_via_byte`: Performs an indirect call to a routine using a value retrieved from a global table based on a byte provided in a register.
- `dispatch_entity_command_alt`: Stores two bytes, a pointer, and a byte into a data structure at specific offsets before performing an indirect call to a routine via a global table.
- `dispatch_complex_entity_command`: Stores three bytes into a data structure, calculates a pointer using a value from the structure, and performs an indirect call to a routine using a value from a global table.
- `copy_bytes_and_call_indirect`: Copies three consecutive bytes from a source memory location to a destination offset.
- `decrement_and_lookup_table_values`: Decrements specific byte values within a data structure and uses the resulting values as indices to fetch word-sized entries from a global table.
- `update_entity_state_and_table_2_2`: Updates an entity's data structure by modifying specific byte and word offsets, decrementing internal counters, and performing indirect calls based on table lookups.
- `update_entity_state_and_table_2_3`: Updates an entity's state by modifying various properties in a data structure through offsets and performing an indirect call to another routine.
- `calculate_offset_and_call`: calculates a value based on a signed byte and a table, stores it in a structure, and calls a function through a pointer in that structure.
- `update_entity_state_and_table_2`: Updates an entity's state by modifying various properties in a data structure through offsets and performing an indirect call to a routine selected from a table.
- `store_byte_and_call`: Stores a byte into a structure at a specific offset and then performs an indirect call using a calculated address.
- `store_byte_and_call_indirect`: Stores a single byte from a pointer into a memory location offset from a register and then executes a function via an indirect call using a value derived from a register and a memory location.
- `store_multiple_and_call`: Stores two bytes into a structure, updates a third value using an offset and a pointer, and performs an indirect call.
- `store_multiple_and_call_offset_89a`: Stores two bytes into a structure, updates a third value using a different offset and a pointer, and performs an indirect call.
- `execute_indirect_call`: Performs an indirect call to a routine using an address calculated from a global variable and a value from a table.
- `update_table_and_call`: Copies three consecutive bytes from a source to a destination table and updates a global value before performing an indirect call.
- `update_entity_state_table`: Decrements a byte in a structure and performs an indirect call to a routine selected from a jump table.
- `update_entity_state_table_2`: Decrements a byte in a structure and performs an indirect call to a routine from a table based on the new value.
- `update_indirect_pointer`: Calculates a value based on a signed offset from a table and stores it in a memory location.
- `update_entity_state_and_table`: Updates several fields within a data structure using values from a source pointer and then performs a series of lookups in a table to populate specific offsets in another structure based on calculated indices.
- `indirect_call_from_table`: Performs an indirect call to a routine located at an address derived from a global variable and a value from a table.
- `store_byte_and_execute_indirect`: Stores a single byte from a pointer into a memory location offset from an unspecified register.
- `transfer_and_execute_indirect`: Stores a byte from a pointer into a memory location and then performs an indirect call using a value constructed from a register and the subsequent byte in the source buffer.
- `update_attribute_and_jump`: Stores two bytes from a source pointer into a destination table and performs an indirect call using a value from a global table.
- `update_attribute_and_jump_2`: Writes two bytes to a destination table and updates a different offset within that table before executing an indirect call.
- `jump_via_table_index`: Performs an indirect call to a location calculated by adding a byte from a source pointer to a global table.
- `update_pointer_and_jump`: Stores two bytes into a table, updates a pointer and a byte at specific offsets, and executes an indirect call based on a table value.
- `copy_multiple_bytes_and_jump`: Copies three consecutive bytes from a source to a destination table and executes an indirect call using a fourth byte from the source.
- `copy_multiple_bytes_and_jump_2`: Transfers three bytes from a source to a destination and performs an indirect call using a value from the source buffer.
- `copy_data_with_table_lookup`: Writes several bytes to a destination and performs four sequential lookups from a global table to update specific offsets.
- `copy_data_with_table_lookup_2`: Updates a destination table with specific bytes and populates four offsets by fetching values from a global table based on an index.
- `update_sprite_animation_frames`: This routine updates a sprite attribute table by selecting specific animation frames from a source buffer based on a frame index.
- `update_indirect_pointer_2`: Calculates a value based on a signed offset from a memory location and stores it in a structure field.
- `update_sprite_attribute_table`: This routine updates a set of sprite or object attributes by reading values from a source buffer and writing them to a destination table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_dma_transfer_setup`: Sets up parameters for a DMA transfer and enables the DMA controller.
- `configure_dma_transfer_2`: Sets values in a global variable and enables a DMA channel via the DMA control register.
- `execute_infinite_loop`: The routine enters an infinite loop that prevents further execution.
- `increment_nibbles_in_table`: Iterates through a table and increments the nibble values of each word based on comparisons with another table.
- `copy_and_decrement_nibbles`: copies a block of data, then repeatedly decrements the nibbles of bytes in a table and copies data back and forth.
- `clear_memory_blocks`: fills two large memory blocks with zeros using a loop.
- `reorganize_data_blocks`: copies data from a source table to a destination table by rearranging bytes into a new structure format.
- `copy_structured_data`: Copies data from one table to another using a loop that performs non-contiguous word and byte writes at specific offsets.
- `copy_data_blocks`: Copies data from one memory location to another using nested loops and specific word-sized offsets.
- `initialize_display_and_copy_data`: Configures various bitplane and display registers before zeroing a memory range and performing a nested loop to copy structured data from a code source to a destination table.
- `fill_memory_block`: Copies a block of data from a source to two different destination locations within a table.
- `modify_global_state`: Checks a bit in a global variable and, if set, performs a series of bitwise operations and a function call before resetting the variable to a specific value.
- `process_input_bits`: Reads bits from a global variable, performs bitwise shifts and XOR operations, and updates the global variable based on the resulting values.
- `toggle_memory_pointers`: Toggles a global variable using an exclusive OR operation and then assigns a series of constant values to that variable based on the result.
- `set_global_values`: Assigns specific constant values to a global variable multiple times.
- `populate_data_table`: Iterates through a loop to store a 32-bit value split into two 16-bit parts into a table at specific offsets.
- `populate_split_values`: Iterates through a table to store a 32-bit value, split into two 16-bit components, at specific offsets within the table.
- `set_global_value`: Sets a global variable to a specific value and then calls another routine.
- `configure_dma_transfer`: Sets a series of values into a global variable and enables a DMA channel by writing to the DMA control register.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_and_update_sprite_attributes`: Decrements several counters and, if non-zero, updates sprite-related pointers and hardware registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_single_instruction_2`: A thirteen-instruction routine that executes a single instruction and returns.
- `execute_single_instruction`: Executes one instruction and returns.
- `return_from_subroutine`: This routine consists of a single instruction that returns control to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_subroutine_2`: This routine is empty and performs no operations before returning.
- `return_immediately_2`: This routine is empty and performs no operations before returning to the caller.
- `nop_routine`: This routine is empty and performs no operations.
- `perform_no_operation`: This routine contains no executable instructions and returns immediately upon entry.
- `return_from_subroutine_3`: This is an empty routine that performs no operations and immediately returns to the caller.
