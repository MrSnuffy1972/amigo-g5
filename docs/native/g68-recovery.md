# Game 68 (AfterBurner v1.2 Sega 0947): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00068). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (941040 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 16 of 16 units identical (bytes and relocations), 89 raw-word lines |
| Functions named and described | pass, 171 of 171 named (0 author symbols), 171 of 171 described (15 by rule, 130 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2207 of 2207 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 2048 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3499 of 3499 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 256730 |
| code | 13186 |
| data | 669270 |
| padding | 78 |
| unused | 1776 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| AfterBurnerSega.Slave | 1 | 174 | 10 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 376 | 23 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2667 | 104 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 205 | 24 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 57 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 5 |
| Disk.1 bytes 11776-12032 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 chunks 282-288 (bytes 72192-73984) (code never loaded by a reference run, load address unknown) | 1 | 0 | 0 |

## Cases handled

- AfterBurner_v1.2_Sega_0947/AfterBurnerSega/Disk.1: raw disk image (892400 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- 8 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 1654 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 42 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 14 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- AfterBurner_v1.2_Sega_0947/AfterBurnerSega/Disk.1: bytes 11776-12032 look like code no run loaded: recovered position-independently (load address unknown)
- AfterBurner_v1.2_Sega_0947/AfterBurnerSega/Disk.1: 9 logged loads cover 251684 bytes of the image
- Disk.1 chunks 282-288 (bytes 72192-73984) (code never loaded by a reference run, load address unknown): never-loaded code-like bytes recovered position-independently (load address unknown, 0 of 1792 bytes decode)
- AfterBurner_v1.2_Sega_0947/AfterBurnerSega/Disk.1: chunks 3475-3476 (bytes 889600-890112) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.16)
- AfterBurner_v1.2_Sega_0947/AfterBurnerSega/Disk.1: chunks 3479-3483 (bytes 890624-891904) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.45)
- 2 never-loaded region(s), 2048 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 2 packed track region(s), 634624 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### AfterBurnerSega.Slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `copy_xor_block_install_jumps`: Calls three routines from a context block, then copies a counted block of longwords into a destination while XORing each with a key, and writes absolute jump opcodes and routine addresses into fixed RAM locations.
- `install_code_patches_on_success`: Calls a context routine with a fixed action code and, when the result is non-zero, writes a series of code patches into RAM: clearing words, writing no-operation pairs, subroutine call and return opcodes, and jump opcodes to routines.
- `halt_when_port_flags_set`: Calls a helper, then calls a context routine with a fixed code; if two status flag bits in the port mirror are both set it spins forever, otherwise it returns the helper result.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `store_longword_at_masked_address`: Writes its second input longword to the address formed by masking the first input to a window of about 512 KB, then returns that address split into a word pair with the following word.
- `open_scores_file_if_available`: Calls a context routine that reports whether the scores file is available and, when it is, calls a second context routine, then returns its two input values.
- `run_frame_loop_until_marker`: Runs a loop that calls several helpers and advances timers and table pointers each pass, ending when a table entry equals the end marker, then copies twelve words of color data into the custom color registers from the background color register.
- `wait_blitter_return_pair`: Calls a helper that waits for the blitter to finish, then calls a context routine and returns the two input data values as a pair.
- `patch_words_when_marker_matches`: Calls a helper and a context routine, then compares a longword in RAM with a fixed marker value and, when it matches, rewrites that word and two others with replacement values.

### logged disk load from Disk.1

- `update_volume_ramp_run_steps`: Moves a volume-like byte one step toward a target each time an accumulator carries, flipping a direction flag at the target, then runs the per-channel step routine four times and again four times when a second flag and carry condition hold.
- `step_channel_countdown_table`: Decrements a per-channel countdown byte; when it reaches zero it reads the next duration from a table, ending the step if that value is zero.
- `update_channel_envelope_step`: Advances one channel's envelope and modulation state: it calls the countdown routine, scales a table value by a volume byte, applies optional sign and offset adjustments under flag bits, and stores the result as the channel's output word.
- `reset_audio_channel_state`: Clears five per-channel state bytes, then writes two values to a shared control word, first all bits set and then a low nibble, which the facts show as audio DMA and ADKCON register writes.
- `read_channel_sequence_bytes`: Reads bytes from a channel's sequence stream; bytes in the top range end the step and save the stream position, while the other values are stored as note or instrument fields or dispatched through a handler table.
- `store_channel_header_and_parse`: Stores the first byte of a sequence stream into a song-level field and then runs the same sequence byte loop that ends by saving the stream position.
- `set_flags_and_branch`: Sets bit 5 and then bit 1 of the byte pointed to by a0, then branches to the routine at offset 1116.
- `set_flag_and_branch`: Sets bit 5 of the byte pointed to by a0, then branches to the routine at offset 1116.
- `copy_byte_and_branch`: Copies the byte at the address in a1 to offset 24 of the structure in a0 while incrementing that address, then branches to the routine at offset 1116.
- `init_pitch_slide_and_parse`: Stores a step value and a count into channel fields, clears the accumulator, and sets the slide flag, then parses the remaining sequence bytes.
- `init_channel_oscillator_and_parse`: Stores two header bytes as oscillator parameters, one used as a phase counter and one doubled as its range, sets the oscillator-enable flag, and then parses the remaining sequence bytes.
- `store_channel_byte_and_parse`: Stores the first byte of a sequence into a channel field and then runs the same sequence byte loop that ends by saving the stream position.
- `set_channel_flags_and_parse`: Sets two flag bits in a channel control byte and then parses the sequence bytes until the step ends.
- `clear_channel_control_and_parse`: Clears a channel control byte and then parses the sequence bytes until the step ends.
- `store_masked_global_and_parse`: Stores a masked value from the first sequence byte into a global, then parses the rest of the sequence stream.
- `tick_channel_sequence_timer`: Counts down the channel's step timer; while it runs, a ramp flag moves a level byte up or down.
- `copy_byte_to_offset`: Copies the byte at the address in a1 to offset 172 of the structure in a3 while incrementing that address, then branches to the routine at offset 1116.
- `decode_channel_sequence_timer`: Copies two bytes from the source buffer into the channel structure at offsets 168 and 169, stores the incremented step counter at offset 7600, and then decodes the next MIDI event byte, branching to a routine from a jump table when the byte is a channel message.
- `reset_step_and_save_position`: When the channel's hold flag is clear, it zeroes the channel's extra counter, sets its start count and writes the channel's output byte.
- `load_table_entry_into_channel`: Looks up a table entry at the given index, saves the index in the channel's table position field, stores the looked-up value as the channel's current level, and sets its counter to one.
- `update_channel_sample_pointer_length`: For a channel whose start delay is running, it counts the delay down, clamps a negative offset to zero on expiry, and writes the sample pointer and word length into the channel's audio hardware slot, then writes a DMA control value.
- `reset_and_load_channel_table`: Clears the channel state table of sixteen records, marks two global bytes with a full-set sentinel value, then loads four channel start entries from a table selected by the argument, setting each channel's pointer, length and sentinel fields.
- `write_table_data`: Uses the low byte of a register to index a table, then stores a byte, a word, and a long word from that table location into a global structure, and finally sets a word at offset 36 to -1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_registers_global`: Stores bytes from d7 and d6 into a global variable at six consecutive offsets, then jumps into the middle of another routine.
- `update_global_with_register_offsets`: Compares d0 against a series of values and, for each case, writes a set of bytes derived from d7 into a global variable at different offsets before calling a helper routine.
- `store_register_bytes`: Stores the lower and upper halves of three registers into a global variable, writing each 16-bit half to a separate location.
- `clear_global_variable`: Clears twelve 16-bit words in a global variable to zero, one after another, then returns.
- `set_d5_to_one`: Sets d5 to 1 and then branches back into the calling routine.
- `set_d5_to_two`: Sets d5 to 2 and then branches back into the calling routine.
- `copy_entry_data_from_table`: Reads a byte from the table pointed to by a0 and, using the index in d0 and d1, computes an offset into a structure in a2.
- `init_table_scan_loop_2`: Writes the constant an address to the long at offset 252 of the structure in a6 and clears the word counter at offset 270, then loops thirty-nine times calling init_global_and_call_routines each iteration.
- `init_global_and_call_routines`: Writes three constant words into a global variable and points a global pointer at it, then repeatedly calls another routine in a loop that counts down from 700 and stores a negative constant into a structure member when a flag is false, and finally calls two more routines before returning.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `check_input_flags`: Reads bit 6 of the CIA port A register and bit 7 of the same register, then bit 2 of the CIA CIApam register, and returns 1 if all three are set, otherwise it returns 0.
- `call_and_test_flag`: Saves registers d1-d7 and a0-a6 on the stack, calls the routine at 10606, restores the saved registers, then returns 1 if the global variable at 10144 is nonzero and 0 otherwise.
- `initialize_then_loop_routines`: Initializes a structure in a6 and a global variable, then repeatedly calls several routines while decrementing a counter until it reaches negative one.
- `tiny_ori`: Small routine of 1 instruction that merges 106 into d0.
- `lookup_table_byte`: Loads a byte from offset 59 of the structure in a6 and returns immediately if it is zero, then compares it against successive bytes of a table starting at offset 106 of the address stored in a global word.
- `wrap_buffer_block_fill`: Saves registers on the stack, calls a routine that fills a buffer with two blocks of seven entries, restores the registers, and returns the two values passed in d0 and d1.
- `fill_buffer_blocks`: Fills a buffer at a fixed address with two blocks of seven entries, each block copying a word from a source table, writing a character, a byte, a fixed long word, and the copied word, then writes a terminating entry.
- `process_enter_name_input`: Waits for the raster beam to reach a target line, then repeatedly reads a character from a text buffer and stores it into a nine-byte name field, advancing or moving the cursor for alphanumeric characters and clearing the field when the terminator is reached.
- `process_text_character`: Reads a character from a text buffer and looks it up in a lookup table, then branches to one of two routines depending on a value read from a global variable at offset 136.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `noop_instruction`: Performs a no-op by ORing zero into register d0 and is only referenced by its address.
- `init_table_scan_loop`: Writes the constant an address to a long at offset 252 of the structure in a6 and clears a word counter at offset 270, then loops thirty-nine times reading a long from a table, comparing it to the marker an address and otherwise adding a value from a global word.
- `load_seven_and_branch`: Loads the value 7 into a register and branches into the middle of another routine, contributing only these two instructions to its own flow.
- `convert_nibbles_ascii`: Converts the low nibbles of a value in d2 into ASCII digits, writing a space for the first nibble and a digit character for the second, then returns.
- `wrap_counter_update`: Saves registers on the stack, calls a routine that decrements a global counter and resets it when it reaches zero, restores the registers, and returns the two values passed in d0 and d1.
- `update_counters_buffer`: Decrements a global counter and resets it to 3 when it reaches zero, adjusting a value in another global by a step with wraparound, then writes a block of entries to a buffer using a table lookup, decrementing a counter with wraparound.
- `store_and_call_pointer`: Loads a pointer from offset 140 of the structure in a6, stores it into a table indexed by d3, calls the routine at 55904, stores the returned pointer back to offset 140, and returns.
- `copy_pointer_and_call_pages`: Copies a pointer from one structure member to another, then calls a routine twice with different index values and invokes a display routine with two different page addresses before jumping to a handler routine.
- `loop_adjusted_counter_update`: Calls two setup routines and a third helper, sets several fixed values in a structure, then loops decrementing a counter while reading entries, adjusting counters with range wrapping and calling helpers on each pass.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 1 into d1.
- `adjust_globals_wrap`: Adjusts global values based on the value in d0, wrapping them within a range.
- `process_command_byte`: Reads a byte from offset 58 of the structure in a6 and, depending on its value, adjusts 16-bit counters at offsets 44, 46 and 50 of a global structure, toggles the 16-bit flags at offsets 222 and 224 of another global, increments or decrements a 16-bit counter at offset 246 of a third global, or clears seven of those 16-bit values, then returns.
- `process_matrix_transform_loop`: Initializes a global long at offset 252 with a marker and clears a counter at offset 270, then loops sixty-three times reading four words from source tables, transforming them, and storing the results into a destination table.
- `empty_zero_word`: This routine is a single zero word that only decodes as ori.b #0, d0 and contains no branches or calls, so it performs no work and simply returns.
- `walk_table_until_marker`: Stores the value -1 at offset 252 of the structure in a6, calls a helper routine, then walks a table in a fixed address in four-byte steps calling a handler for each entry until it finds one equal to -1.
- `store_zero_byte`: It writes a zero byte to the memory location pointed to by d0 and then returns.
- `init_and_update_struct_flags`: Writes 255 to byte at offset 76 of the structure in a6, then, if a global flag is zero, initializes several global variables and counters and returns.
- `select_and_store_state_words`: Reads the low word of d0 and d1 and the words in d2 and d3, calls a difference-packing routine twice storing the results in state words at offsets 2ede and 2ee0, then branches based on their bits and zero status to either call another routine or copy the original register values into state words at offsets 2ed2 and 2ed4 before calling a third routine.
- `compute_differences_and_pack`: Computes four differences between the words in d7 and d6 and four globals at offsets 2EE6, 2EEA, 2EE8 and 2EE4, doubles each result, and returns the four packed into d1 with the last swapped into the high half.
- `midpoint_subdivision_search`: Repeatedly averages two pairs of 16-bit values by adding each to a base and shifting right by one, calling a difference-packing routine each iteration, and returns when the averages stop changing, the input values match, or a flag mask test succeeds.
- `lookup_table_by_index`: Tests d0 and branches when non-negative, then adds 360 to d0 and looks up a word in a table at offset 2BFA using d0 as an index.
- `gap_decodes_as_code`: This routine contains a run of bytes that only decodes as instructions because it falls in a gap between real code; the seven instructions perform no work and simply return.
- `reset_identity_matrix_words`: Loads the fixed-point value for one into the three diagonal positions of a block of 16-bit words and zeros the remaining entries, matching a 3x3 identity matrix reset.
- `rotate_pair_by_matrix`: Calls a routine, then computes six scaled 16-bit products of two inputs against stored matrix words, writes them into a state table, and copies eight words into the shared block.
- `apply_negated_matrix_transform`: Applies the same kind of two-input matrix transform as its neighbour but with one input negated, writes six results into a state table, and copies eight words into the shared block.
- `transform_pair_and_copy_state`: Applies a two-input matrix transform, writing six products into a state table with some entries copied directly from matrix words, then copies eight words into the shared block.
- `compute_three_input_matrix_pair`: Multiplies three 16-bit inputs by stored matrix words, accumulates the products, scales each row by four, and packs the four scaled results into the low halves of the data registers.
- `interpolate_scaled_pair_or_zero`: Computes a fixed-point ratio from two differences and uses it to scale two inputs and add fixed offsets, returning both as packed 32-bit values; it returns zeros when the span is zero.
- `clear_d0_low_byte`: It is a zero-filled data area that other code refers to by address, containing only a single instruction that clears the low byte of register d0.
- `clear_d0_low_byte_2`: This routine is a single instruction that clears the low byte of the 32-bit register d0 by OR-ing it with zero, then returns.
- `store_zero_byte_2`: It writes a zero byte to the memory location pointed to by the address in a0, then returns.
- `process_table_entries`: Initializes a global long at offset 252 with the marker and clears a counter at offset 270, then loops thirty-nine times reading a 16-bit value from a table and skipping it when the value exceeds 700.
- `advance_timer_read_records`: Subtracts a stored step from a 16-bit timer in a global variable and, when it reaches zero, reloads the period from a table and scans 16-bit records there.
- `fill_table_free_marker`: Loops forty times writing the marker spelling Free into consecutive long-word slots of a shared table, then returns.
- `run_sentinel_terminated_scan`: Marks a slot at offset 252 of the structure in a6 with the all-ones value, calls one routine, then walks four-word records in a fixed table calling another routine for each until a record starts with the all-ones sentinel.
- `clear_table_with_zeros`: Loops 224 times writing zero into consecutive long-word slots of a table starting at the given address, then returns.
- `set_high_bits_on_byte`: Sets a fixed set of bit positions in the byte register d0, including the lowest bit, and returns without modifying anything else.
- `advance_frame_and_decrement_offset`: Calls a frame-index advancement routine and then subtracts six from the word at offset two of the structure in a5 before returning.
- `advance_record_and_increment_offset`: Calls a record-index decrementing routine and then adds six to the word at offset two of the structure in a5 before returning.
- `decrement_offset2_by8`: Subtracts eight from the 16-bit value at offset 2 of the structure in a5 and returns.
- `increment_offset2_by8`: Adds eight to the 16-bit value at offset 2 of the structure in a5 and returns.
- `branch_to_frame_index_forward`: Branches to a frame-index advancement routine that increments the byte at offset one of the structure in a5 by two or one depending on bit six of the byte at offset zero.
- `branch_to_record_index_backward`: Branches to a record-index decrementing routine that decrements the byte at offset one of the structure in a5 by two or one depending on bit six of the byte at offset zero.
- `decrement_offset4_by4`: Subtracts four from the 16-bit value at offset 4 of the structure in a5 and returns.
- `increment_offset4_by4`: Adds four to the 16-bit value at offset 4 of the structure in a5 and returns.
- `decrement_offset4_by7`: Subtracts seven from the 16-bit value at offset 4 of the structure in a5 and returns.
- `increment_offset4_by7`: Reads a 16-bit value at offset 4 of the structure in a5, adds 7 to it, and writes the result back before returning.
- `decrement_two_offset16`: Subtracts 3 from the 16-bit value at offset 2 of the structure in a5 and subtracts 4 from the 16-bit value at offset 4, then returns.
- `adjust_two_offset16`: Adds 3 to the 16-bit value at offset 2 of the structure in a5 and subtracts 4 from the 16-bit value at offset 4, then returns.
- `adjust_offset2_and4`: Subtracts 3 from the 16-bit value at offset 2 of the structure in a5 and adds 4 to the 16-bit value at offset 4, then returns.
- `increment_two_offset16`: Adds 3 to the 16-bit value at offset 2 of the structure in a5 and adds 4 to the 16-bit value at offset 4, then returns.
- `decrement_two_by4`: Subtracts 4 from the 16-bit value at offset 2 of the structure in a5 and subtracts 4 from the 16-bit value at offset 4, then returns.
- `adjust_offset2_by4`: Adds 4 to the 16-bit value at offset 2 of the structure in a5 and subtracts 4 from the 16-bit value at offset 4, then returns.
- `adjust_offset2_and4_v2`: Subtracts 6 from the 16-bit value at offset 2 of the structure in a5 and adds 5 to the 16-bit value at offset 4, then returns.
- `increment_two_by6`: Adds 6 to the 16-bit value at offset 2 of the structure in a5 and adds 5 to the 16-bit value at offset 4, then returns.
- `step_frame_index_forward`: Advances the byte at offset 1 by 2 when bit 6 of the byte at offset 0 is clear, or by 1 when it is set, wrapping to zero past the upper limit.
- `decrement_record_index_backward`: Decrements the byte at offset one of the record by two when bit six of the first byte is clear, wrapping it to the value fourteen if it drops below two, and by one when bit six is set, wrapping it to fifteen if it drops below one.
- `reset_record_from_table_entry`: Selects a pointer from a table using a record index, copies a word and a long from it into the record, stores the pointer plus three into another field, zeroes a long counter, and clears 137 consecutive long words of shared memory.
- `advance_timed_event_queue`: Returns early if a busy flag is set, otherwise increments a counter and, when it matches the scheduled value, either flags the record to advance or pops the next queued entry and copies its fields into a 24-entry table of 11-word records.
- `update_script_slot_table`: Walks a 24-slot table adjusting each active slot's timer; expired slots read a byte stream and dispatch each byte through an indirect routine table, and slots that leave their range are deactivated.
- `build_tiered_record_entry`: Selects a tier from thresholds on a position word, adds a base offset chosen by the type byte, calls index-advancing routines based on a global byte, appends a four-word record to an output list, and stores computed values and table lookups back into the source record.
- `lookup_table_value_with_offset`: Reads a shared index word, selects a 16-bit entry from a table by that index, and returns it added to the input value and to a fixed offset of 176.
- `clear_thirty_long_table`: Writes zero into 30 consecutive long-word slots of the shared table and returns.
- `find_empty_table_slot`: Searches the table for a null pointer slot, copying a word from offsets two and four of a source location and a value from offset six into that slot along with the fixed constant six, and returns if no free slot is found.
- `search_and_insert_list_node`: Iterates through the table looking for a null terminator, then inserts a new entry by copying two words and a value from a source location into that slot and storing the fixed constant six.
- `emit_active_entries_and_age`: Walks fifteen four-word slots, decrementing each occupied slot's counter and clearing those that run out, while appending a four-word record to the output list for the slots that remain active.
- `call_reset_routine_four_times`: Calls another routine four times in sequence, loading the values four, five, six, and four into a register before each call, then returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `walk_slots_with_counter`: Loads an address into a5, sets a counter to fourteen, and loops decrementing the address by four until the counter reaches zero, otherwise it returns.
- `store_slot_record_fields`: Stores the long value passed in an address register into a slot, sets the word at offset six to five, and clears the word at offset four.
- `advance_slot_animation_and_emit`: Walks 15 slots, where each active slot either counts down a delay or steps its frame index and deactivates past the last frame, then appends a four-word record built from the slot's referenced data to the output list.
- `clear_slot_area_with_counter`: Loops writing zero to four consecutive bytes of a data area while decrementing a counter from thirty-nine until it reaches zero, otherwise it returns.
- `test_and_adjust_bytes`: Tests bit d2 of the bytes in d5 and d4, then decrements d4 by 4 and d3 by 3.
- `advance_counter_match_result`: Increments a byte counter at offset 238 of the structure in a6 and reads a table entry selected by the 16-bit value at offset 204, returning zero and resetting the counter when the entry matches the counter, otherwise returning minus one.
- `advance_counter_against_global`: Increments the counter byte at offset 110 of the structure in a6 and compares it with a shared global byte.
- `find_and_fill_table_entry`: Tests whether the word at offset 186 of the structure in a6 is zero and, if so, calls a lookup routine, then loops four times reading a word from a table at offset an address and, on the first zero entry, stores two words from offsets 2 and 4 of the structure in a5, writes a marker and several constants to a structure, and returns a computed character.
- `update_moving_slots_and_emit`: Walks ten 16-byte slots, advancing each slot's position words by its velocity words and nudging them toward a target by a fixed step, clearing slots that leave the allowed range, and appending a four-word record for slots that stay.
- `reset_and_step_slots_four_times`: Checks a 16-bit value at offset 168 of the structure in a6 and returns immediately if it is zero, otherwise it calls init_and_update_struct_flags and then calls another routine four times in sequence.
- `append_four_word_record`: Writes the four 16-bit values from d3, d0, d1 and d2 to the next free position in the list pointed to by offset 252 of the structure in a6, then advances that pointer by four words and increments the record count at offset 270.
- `empty_no_op_routine`: This routine is empty, containing only a no-op instruction that other code refers to by address.
- `store_zero_byte_3`: It is a single instruction that writes a zero byte to the memory location pointed to by d0, then returns.
- `update_shared_counter_state_records`: Adjusts a shared 16-bit value in a global variable based on a counter in the a6 structure, then appends two four-word records to a buffer pointed to by the a6 structure.
- `clear_four_long_table`: Writes zero into four consecutive long-word slots of the shared table starting at offset an address and then returns.
- `decrement_counter_and_flag`: Returns immediately when the short at offset 236 of the structure in a6 is zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_and_call_routine`: Stores the pointer in a6 at offset 20 of the structure in a3, adds 13000 to it and stores the result at offset 24, then calls another routine and returns.
- `lookup_and_call_routine`: Stores the pointer in a6 at offset 20 of the structure in a3, saves the incoming a0 pointer at offset 24, looks up a value in a table, masks it to 24 bits, and calls another routine with it.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `init_ciab_timer`: Saves d0, clears the top two bits of the CIAB parallel port and sets bit 3, then writes the bytes 127, 202 and 9 to the CIAB timer registers and waits for bit 0 of the interface control register to become set.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `copy_split_byte_blocks`: Saves a return address, stores a source pointer and length in a structure, divides the length by a fixed divisor to split it into two parts, then repeatedly copies byte blocks until the remaining amount falls below the divisor.
- `copy_block_and_clear`: Copies the byte block whose length is in d0 from the source table in a3 to the destination table, then clears d0 and returns.
- `load_pointer_copy_block`: Loads the pointer stored at offset 16 of the structure in a3 into d0, calls a routine to copy a byte block, then jumps back into the middle of another routine.
- `copy_byte_block`: Copies a number of bytes given in d0 from a source table to a destination table, advancing the destination pointer stored at offset 12 of the structure in a3 as it goes.
- `interleave_bit_block_loop`: Reads a word from the structure in a3, calls two helper routines, increments that word by one, then loops one thousand five hundred forty-nine times interleaving the odd bits of consecutive longwords into a destination block.
- `conditionally_call_bit0`: Tests bit 0 of d0, calls a helper routine when the bit is clear, and otherwise returns without modifying any data.
- `shift_and_loop_subtracting_counter`: Shifts the value in d0 right by one bit, then compares it with the 16-bit counter at offset 0 of the structure in a3.
- `loop_decremented_counter`: Computes a decremented value from the counter at offset 0 of the structure in a3, then repeatedly calls a routine while that value is not zero.
- `init_device_loop`: Initializes a hardware device by writing a sequence of words to its registers, then loops until a status bit clears, running a checksum over a data block and repeating until the computed value matches.
- `decrement_counter_set_bit`: Decrements the counter at offset 0 of the structure in a3, sets bit 1 of the byte pointed to by a1, and then branches back into the calling routine.
- `toggle_bits_and_call`: Increments a counter in the structure in a3, toggles three bits of the byte pointed to by a1, then calls another routine that configures the CIAB and returns.
- `wait_for_bit_clear`: Waits in a loop while bit 5 of the byte pointed to by a2 stays set, then returns.
- `clear_bit_and_wait`: Clears bit 2 of the byte pointed to by a1, then waits in a loop while bit 5 of the byte pointed to by a2 stays set before returning.
- `set_bit_and_wait`: Sets bit 2 of the byte pointed to by a1, then waits in a loop while bit 5 of the byte pointed to by a2 stays set before returning.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `write_parallel_if_nonzero`: Returns immediately if the byte in d1 is zero, otherwise writes 119 to the CIAB parallel data register and waits in a loop while bit 5 of the CIAB control register stays set.
- `set_global_byte`: Writes the byte 255 to the CIAB parallel data register, then writes 135 and 255 to the same register before returning.
- `interleave_bit_block`: Skips past a marker in a data block, then loops 1549 times interleaving the odd bits of consecutive longwords into the destination block and returns.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_store_byte`: Clears a 16-bit value at offset 8 of the structure in a4, then stores the high byte of the incoming word together with the byte at offset 39 of the structure in a0 into a global variable, and returns.
- `update_sequence_counters`: Stores the incoming word at offset 8 of the structure in a4, then decrements the counter at offset 33 and reloads it from a byte in the structure in a1 when it reaches zero.
- `advance_table_stepped_counter`: Decrements a counter at offset 14 of the addressed structure and reloads it from a lookup table when it reaches zero, then adds a table entry to a byte at offset 15.
- `update_table_stepped_counter`: Stores the value in d1 into the short at offset 8 of the structure in a4, then calls a helper routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_display_dma_and_interrupts`: Writes to display, DMA and interrupt-enable registers, with bit 15 of each write selecting set or clear.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `return_then_branch`: Returns immediately, then branches to the routine at offset 192.
