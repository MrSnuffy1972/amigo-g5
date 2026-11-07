# Game 2341 (blades of steel (1990)(novotrade)[cr pna][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02341). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (916432 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 35 raw-word lines |
| Functions named and described | pass, 51 of 51 named (0 author symbols), 51 of 51 described (0 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 321 of 321 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1432 of 1519 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 26349 |
| code | 9632 |
| data | 847683 |
| unused | 32768 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Blades of Steel (1990)(Novotrade)[cr PNA][a].adf:bootblock | 1 | 135 | 2 |
| logged disk load from Blades of Steel (1990)(Novotrade)[cr PNA][a].adf boot-block load | 1 | 85 | 2 |
| logged disk load from Blades of Steel (1990)(Novotrade)[cr PNA][a].adf boot-block load | 1 | 0 | 0 |
| logged disk load from Blades of Steel (1990)(Novotrade)[cr PNA][a].adf boot-block load | 1 | 283 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1016 | 36 |

## Cases handled

- blades of steel (1990)(novotrade)[cr pna][a]/Blades of Steel (1990)(Novotrade)[cr PNA][a].adf: custom-format (non-DOS) disk, 1677 non-empty blocks: matched against track-loaded segments
- blades of steel (1990)(novotrade)[cr pna][a]/Blades of Steel (1990)(Novotrade)[cr PNA][a].adf: the boot block reads 11264 bytes from disk offset 889856 to an address (found statically); recovered as a unit
- blades of steel (1990)(novotrade)[cr pna][a]/Blades of Steel (1990)(Novotrade)[cr PNA][a].adf: the boot block reads 1024 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- blades of steel (1990)(novotrade)[cr pna][a]/Blades of Steel (1990)(Novotrade)[cr PNA][a].adf: the boot block reads 4096 bytes from disk offset 2048 to an address (found statically); recovered as a unit
- runtime segment an address: 321 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- blades of steel (1990)(novotrade)[cr pna][a]/Blades of Steel (1990)(Novotrade)[cr PNA][a].adf: 3 logged loads cover 16384 bytes of the image
- blades of steel (1990)(novotrade)[cr pna][a]/Blades of Steel (1990)(Novotrade)[cr PNA][a].adf: 29 track-loaded blocks found in recovered runtime segments and classified through them
- blades of steel (1990)(novotrade)[cr pna][a]/Blades of Steel (1990)(Novotrade)[cr PNA][a].adf: blocks 22-111 (bytes 11264-57344) outside the filesystem hold data, not code (opcode density 0.047, entropy 6.48)
- blades of steel (1990)(novotrade)[cr pna][a]/Blades of Steel (1990)(Novotrade)[cr PNA][a].adf: blocks 141-158 (bytes 72192-81408) outside the filesystem hold data, not code (opcode density 0.036, entropy 5.38)
- blades of steel (1990)(novotrade)[cr pna][a]/Blades of Steel (1990)(Novotrade)[cr PNA][a].adf: blocks 160-180 (bytes 81920-92672) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.54)
- blades of steel (1990)(novotrade)[cr pna][a]/Blades of Steel (1990)(Novotrade)[cr PNA][a].adf: blocks 182-219 (bytes 93184-112640) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.26)
- blades of steel (1990)(novotrade)[cr pna][a]/Blades of Steel (1990)(Novotrade)[cr PNA][a].adf: blocks 221-235 (bytes 113152-120832) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.49)
- blades of steel (1990)(novotrade)[cr pna][a]/Blades of Steel (1990)(Novotrade)[cr PNA][a].adf: blocks 237-266 (bytes 121344-136704) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.81)
- blades of steel (1990)(novotrade)[cr pna][a]/Blades of Steel (1990)(Novotrade)[cr PNA][a].adf: blocks 268-282 (bytes 137216-144896) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.58)
- blades of steel (1990)(novotrade)[cr pna][a]/Blades of Steel (1990)(Novotrade)[cr PNA][a].adf: blocks 284-297 (bytes 145408-152576) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.84)
- blades of steel (1990)(novotrade)[cr pna][a]/Blades of Steel (1990)(Novotrade)[cr PNA][a].adf: blocks 299-369 (bytes 153088-189440) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.32)
- blades of steel (1990)(novotrade)[cr pna][a]/Blades of Steel (1990)(Novotrade)[cr PNA][a].adf: blocks 372-383 (bytes 190464-196608) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.71)
- blades of steel (1990)(novotrade)[cr pna][a]/Blades of Steel (1990)(Novotrade)[cr PNA][a].adf: blocks 387-482 (bytes 198144-247296) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.32)
- blades of steel (1990)(novotrade)[cr pna][a]/Blades of Steel (1990)(Novotrade)[cr PNA][a].adf: blocks 485-919 (bytes 248320-471040) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.74)
- blades of steel (1990)(novotrade)[cr pna][a]/Blades of Steel (1990)(Novotrade)[cr PNA][a].adf: blocks 922-934 (bytes 472064-478720) outside the filesystem hold data, not code (opcode density 0.001, entropy 1.58)
- blades of steel (1990)(novotrade)[cr pna][a]/Blades of Steel (1990)(Novotrade)[cr PNA][a].adf: blocks 937-948 (bytes 479744-485888) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.40)
- blades of steel (1990)(novotrade)[cr pna][a]/Blades of Steel (1990)(Novotrade)[cr PNA][a].adf: blocks 952-964 (bytes 487424-494080) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.05)
- blades of steel (1990)(novotrade)[cr pna][a]/Blades of Steel (1990)(Novotrade)[cr PNA][a].adf: blocks 967-979 (bytes 495104-501760) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.14)
- blades of steel (1990)(novotrade)[cr pna][a]/Blades of Steel (1990)(Novotrade)[cr PNA][a].adf: blocks 982-1103 (bytes 502784-565248) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.30)
- blades of steel (1990)(novotrade)[cr pna][a]/Blades of Steel (1990)(Novotrade)[cr PNA][a].adf: blocks 1105-1119 (bytes 565760-573440) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.27)
- blades of steel (1990)(novotrade)[cr pna][a]/Blades of Steel (1990)(Novotrade)[cr PNA][a].adf: blocks 1121-1139 (bytes 573952-583680) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.46)
- blades of steel (1990)(novotrade)[cr pna][a]/Blades of Steel (1990)(Novotrade)[cr PNA][a].adf: blocks 1149-1168 (bytes 588288-598528) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.04)
- blades of steel (1990)(novotrade)[cr pna][a]/Blades of Steel (1990)(Novotrade)[cr PNA][a].adf: blocks 1178-1183 (bytes 603136-606208) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.06)
- blades of steel (1990)(novotrade)[cr pna][a]/Blades of Steel (1990)(Novotrade)[cr PNA][a].adf: blocks 1193-1737 (bytes 610816-889856) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.00)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `step_head_to_track` (Blades of Steel (1990)(Novotrade)[cr PNA][a].adf boot-block load +an address to an address (4096 bytes)): Steps the drive head one track at a time toward the target track, setting the direction bit in the disk control port before each step and updating the stored track count until the current track matches the target. Kind: Disk hardware (CIAB) based track/load synchronization check. Pass path: The routine waits until the disk controller's ready/track00 status bit (via CIAB port B) stabilizes; the passing path is the branch taken after u3__sub_03D0() returns once the hardware condition is satisfied (the loop exits when in_D0w == $.

## Functions


### Blades of Steel (1990)(Novotrade)[cr PNA][a].adf:bootblock

- `load_boot_image_via_doio`: Issues a chain of exec device I/O requests with DoIO to fetch the boot data, then writes the interrupt and DMA disable mask to the global control register and copies a 4 KB block of the loaded image to low memory.
- `decrement_a4_pointed_value`: Decrements the 16-bit value stored at the location pointed to by register A4 by one.

### logged disk load from Blades of Steel (1990)(Novotrade)[cr PNA][a].adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from Blades of Steel (1990)(Novotrade)[cr PNA][a].adf boot-block load

- `seek_track_find_sync`: Compares the requested track with the current track and, when they differ, steps the drive head toward it, then reads raw data until the disk sync marker appears.
- `copy_words_from_offset`: Copies a block of words from a source address made of a stored base plus an offset into the destination pointer, with the word count taken from the data register.
- `decode_mfm_sector_blocks`: Runs a fixed number of passes that take the odd and even bit fields of raw disk data, recombine them with the alternating-bit mask into decoded words, and store each pass at a destination chosen by the sector number.
- `skip_to_disk_sync_word`: Advances through raw disk data until the disk sync marker is found, then skips any further consecutive sync markers, leaving the read pointer on the first non-sync word.
- `step_head_to_track`: Steps the drive head one track at a time toward the target track, setting the direction bit in the disk control port before each step and updating the stored track count until the current track matches the target.
- `wait_disk_block_complete`: Polls the cached interrupt request value until the disk block complete flag is set, then clears that flag by writing it back to the interrupt request register.
- `wait_drive_ready_after_delay`: Runs a short countdown delay loop and then polls the disk ready bit of the CIA port until the drive reports ready.
- `recalibrate_head_to_track_zero`: Steps the drive head outward one track at a time while the track zero sensor bit in the CIA port is still set, then resets the stored track count to zero.
- `wait_disk_index_flag`: Polls the CIA B interrupt control register until its disk index flag bit is set, then returns the value passed in.
- `select_disk_drive_line`: Derives a drive number from the low bits of its argument, sets that drive's select line in the disk control port, and holds it through a 256-count loop before calling a wait routine and clearing the control value.
- `stop_disk_motor`: Sets the motor control bit in the disk control port, pulses the selected drive's select line through a 256-count loop, and then marks the stored control value as fully set.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `start_blitter_from_parameters`: Waits for the blitter to go idle, then copies a block of parameter words into the blitter pointer, modulo, mask, control and size registers, which starts the blit.
- `poll_status_until_clear`: Returns early when a status word is negative; otherwise it repeatedly calls device access routines, stores each returned value, and loops until the high part of the returned value is zero.
- `forward_call_to_routine`: Invokes another routine and then returns, adding no logic of its own.
- `copy_table_entry_to_buffers`: When an enabled status word is non-negative, reads an entry from a lookup table selected by a stored index and a flag bit, then writes its fields into two parallel buffers at a position offset and updates a shadow copy.
- `tick_counter_with_helpers`: Calls two helper routines, increments a 16-bit counter stored in the runtime block, and then calls a third helper.
- `store_long_pair_into_tables`: Writes two 32-bit values, each split into high and low words, into two parallel tables at a shared index, filling four words per table entry.
- `load_table_entry_into_record`: Reads a long from a pointer table chosen by an index, adds a word from the same table entry to it, stores the sum in a record, then sets two 16-bit fields to all ones and clears another.
- `setup_paired_blit_rows`: Fills a blit parameter record with a fixed size and source and destination addresses, starts a blit for the first set of rows, and when a stored status word is non-negative and a condition holds, repeats the setup for a second set of rows at an offset.
- `update_deltas_and_clamp`: Updates the x and y deltas by subtracting stored reference values when the matching flag bits are set, then forces the x value to minus 128 whenever it falls outside a fixed window.
- `write_sprite_control_words`: Computes vertical start and end values from a Y position and an offset, and a horizontal start from an X position, then writes them as control bytes into a sprite data block and clears the trailing words once a stored count reaches zero.
- `setup_shifted_blit_copy`: Writes a fixed set of blitter parameter words, with a shift taken from the low nibble of an argument placed into the control word, then starts the blit and clears a flag.
- `start_conditional_blit`: When a state mask is nonzero, programs a blit with a control word built from a stored shift, adjusts the source address by a fixed amount when one flag is set, and then starts the blit.
- `copy_pointer_when_flagged`: When a flag bit is set, copies a stored pointer into a record field, calls one helper, and clears a word in the record.
- `write_split_long_to_tables`: Reads a 32-bit value through a stored pointer, writes its high and low words into two parallel tables at an offset position, then calls two helper routines.
- `setup_cia_timer_interrupt`: Stores a handler routine address together with timer, control and interrupt values for the CIA A timer and interrupt registers, and enables the selected interrupt sources through the interrupt enable register.
- `handle_vblank_frame_update`: Runs when the vertical blank request flag is set: it advances a frame counter and sequence timing values, calls several update routines when a stored step is reached, and acknowledges the interrupt by writing the request bits back.
- `stub_return_stack_value`: Returns the value found at the top of the stack without doing anything else, so it is a stub with no other behaviour.
- `swap_double_buffer_pointers`: When a stored status word is non-negative, resets a counter, swaps each pair of current and alternate buffer pointers, and stores the new front pointer, or zero when a flag is set.
- `compute_position_deltas`: Computes the x and y differences between target and current positions, copies several state fields into working fields, and clears a counter.
- `fill_eight_strided_bytes`: Calculates a byte offset from an x and y position using a fixed row stride, calls a helper, then writes all-ones bytes into eight locations spaced by a fixed row step.
- `step_position_within_bounds`: Moves a coordinate by two in the direction selected by flag bits, clamps it at its limits, and clears the matching direction bits when a limit is reached.
- `mark_table_entries_empty`: Computes a table index from a row stride and position, streams helper output until a zero is returned, then sets two table entries to all ones and marks a pending flag.
- `blit_four_plane_copies`: Runs four blits in sequence, waiting for the blitter to go idle before each one, with a copy control word, full edge masks, and source and destination addresses that advance by a row step and a plane step.
- `blit_four_plane_pairs_swapped`: Runs four blits like its neighbour but loads the source and destination address registers in the opposite order.
- `copy_tile_into_planes_indexed`: Copies a 32-byte tile from a table selected by an index into four bitplanes, writing eight bytes per plane with each byte stepping down the rows at a fixed row stride.
- `copy_tile_bytes_to_planes`: Copies a 32-byte block from a source pointer into four bitplanes, writing eight bytes per plane with each byte stepping down the rows at a fixed row stride.
- `write_four_coordinate_pairs`: Computes four positions from a shared vertical base and per-entry values, adjusted by a small offset when a low nibble is zero, then writes each as high and low words into two parallel tables.
- `wait_for_flag_then_clear`: Busy-waits until a flag word in the runtime block becomes nonzero, then resets that flag to zero.
- `start_plane_blit_from_table`: Looks up a value from a table by its argument, waits for the blitter to go idle, then sets destination pointers at three plane offsets, edge masks to all ones and modulo and control values that start the blit.
- `copy_words_to_three_tables`: Copies 32 words from a source buffer into one working table and into two parallel tables, advancing a shared offset by four bytes at each step.
- `clear_state_word`: Stores zero into a single 16-bit state word, which resets that flag.
- `reset_runtime_state_block`: Sets four 32-bit position entries to all ones, clears several pointer-based and state words, and sets two state words to their initial values.
- `run_table_setup_sequence`: Passes a computed table address to a helper, then calls two further helpers in sequence, so it is a short orchestration routine.
- `copy_words_to_runtime_buffer`: Copies sixteen words from an argument block into a fixed runtime buffer and passes that buffer to a helper routine.
- `emit_rows_until_terminator`: When a count is nonzero, sets up a record with fixed values and a computed address, then calls a helper and advances the address by a row step until the helper returns zero.
- `process_flagged_list_entries`: Walks a list of six-byte entries and, for each entry whose flag is negative, computes destination addresses from a base and per-entry offsets, sets up a record, and fills it through helper calls.
