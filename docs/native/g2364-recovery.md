# Game 2364 (jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02364). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2708672 top-level bytes in 106 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 80 of 80 named (3 author symbols), 80 of 80 described (5 by rule, 17 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1639 of 1639 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2964 of 2979 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 516547 |
| code | 11270 |
| data | 1946829 |
| padding | 26666 |
| unused | 207360 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 1 of 3).adf:JStrike | 1 | 222 | 20 |
| Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 1 of 3).adf:Frs.exe | 2 | 1500 | 37 |
| Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 1 of 3).adf:TRDINTRO | 5 | 205 | 7 |
| Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 1 of 3).adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1032 | 14 |

## Cases handled

- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 1 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 1 of 3).adf:status: high-entropy data file (7.62 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 1 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 1 of 3).adf:copilots: high-entropy data file (7.69 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 1 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 1 of 3).adf:JSC: high-entropy data file (7.73 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 1 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 1 of 3).adf:JSF: high-entropy data file (7.73 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 1 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 1 of 3).adf:intro: high-entropy data file (7.68 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 1 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 1 of 3).adf:menubgd: high-entropy data file (7.68 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 1 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 1 of 3).adf:music: high-entropy data file (7.82 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 1 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 1 of 3).adf:heli_sprite: high-entropy data file (7.66 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 1 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 1 of 3).adf:sfx: high-entropy data file (7.81 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 1 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 1 of 3).adf:menubgd2: high-entropy data file (7.68 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 1 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 1 of 3).adf:brieffilm: high-entropy data file (7.71 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 1 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 1 of 3).adf:slides: high-entropy data file (7.75 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 2 of 3).adf:world4blks: high-entropy data file (7.72 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 2 of 3).adf:scientist: high-entropy data file (7.66 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 2 of 3).adf:world2blks: high-entropy data file (7.64 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 2 of 3).adf:mission4stat: high-entropy data file (7.73 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 2 of 3).adf:mission2stat: high-entropy data file (7.73 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 2 of 3).adf:brief: high-entropy data file (7.69 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 2 of 3).adf:newsfilm: high-entropy data file (7.71 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 2 of 3).adf:deathfilm: high-entropy data file (7.70 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 2 of 3).adf:madtalk: high-entropy data file (7.60 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 2 of 3).adf:world3blks: high-entropy data file (7.80 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 2 of 3).adf:objects3: high-entropy data file (7.60 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 2 of 3).adf:objects4: high-entropy data file (7.61 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 2 of 3).adf:mission3stat: high-entropy data file (7.72 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 2 of 3).adf:music: high-entropy data file (7.82 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 2 of 3).adf:winfilm: high-entropy data file (7.68 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 2 of 3).adf:scorefilm: high-entropy data file (7.63 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 2 of 3).adf:mission1stat: high-entropy data file (7.70 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 2 of 3).adf:sprites1: high-entropy data file (7.68 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 2 of 3).adf:sprites2: high-entropy data file (7.61 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 2 of 3).adf:sprites3: high-entropy data file (7.61 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 2 of 3).adf:sprites4: high-entropy data file (7.63 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 2 of 3).adf:awac: high-entropy data file (7.73 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 2 of 3).adf:brieffilm: high-entropy data file (7.71 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 2 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 2 of 3).adf:slides: high-entropy data file (7.75 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 3 of 3).adf:mission6stat: high-entropy data file (7.75 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 3 of 3).adf:russian: high-entropy data file (7.66 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 3 of 3).adf:brief: high-entropy data file (7.69 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 3 of 3).adf:world7blks: high-entropy data file (7.76 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 3 of 3).adf:deathfilm: high-entropy data file (7.70 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 3 of 3).adf:mission9stat: high-entropy data file (7.65 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 3 of 3).adf:world5blks: high-entropy data file (7.62 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 3 of 3).adf:mission7stat: high-entropy data file (7.64 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 3 of 3).adf:mission5stat: high-entropy data file (7.64 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 3 of 3).adf:objects5: high-entropy data file (7.63 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 3 of 3).adf:music: high-entropy data file (7.82 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 3 of 3).adf:world8blks: high-entropy data file (7.70 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 3 of 3).adf:sprites5: high-entropy data file (7.62 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 3 of 3).adf:awac: high-entropy data file (7.73 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 3 of 3).adf:sprites6: high-entropy data file (7.63 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 3 of 3).adf:sprites8: high-entropy data file (7.63 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 3 of 3).adf:sprites9: high-entropy data file (7.66 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 3 of 3).adf:world6blks: high-entropy data file (7.77 bits/byte), classified as packed data
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 3 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 3 of 3).adf:mission8stat: high-entropy data file (7.71 bits/byte), classified as packed data
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 712 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- jungle strike (1994)(ocean)[cr trader's dream][t +6 freezers](disk 1 of 3)/Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 1 of 3).adf:TRDINTRO: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 1 of 3).adf:TRDINTRO

## Copy protection found

- none found

## Functions


### Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 1 of 3).adf:JStrike

- `BEGINING`: Program entry point with three instructions and no decompiled body.
- `SUPER`: Decompiled body unavailable, so only facts are known: it writes the DMA and interrupt control registers, calls one other routine, and is called from the entry point.
- `PROGGY`: Chooses a value from a stored field and sets a few globals.
- `setup_copper_sequence_variant`: Repeats the copper and memory setup sequence used by its neighbour, then calls helpers and jumps through a table entry.
- `setup_copper_sequence_variant_2`: Variant of the copper and memory setup that first stores a fixed high-range constant, then runs the same setup and jumps through a table entry.
- `finish_setup_and_jump_table`: Calls helpers, stores a fixed word in a global, writes a copper pointer register value, and finishes by jumping through a table entry.
- `enter_infinite_loop`: Spins forever with no work, so control never returns to the caller.
- `execute_jump_table`: Calls two helper routines and then performs an indirect jump through a table entry.
- `call_helpers_then_jump_table`: Calls two helpers and then jumps through a table entry, returning only if that entry returns.
- `unpack_run_length_bytes_forward`: Decodes a run-length stream into a fixed destination.
- `unpack_run_length_block_copy`: Copies a block of bytes using a count from the input word, then decodes run-length control bytes.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `step_colour_nibbles_to_targets`: Walks sixteen rows of control words.
- `write_colour_nibbles_row_entries`: Writes packed four-bit colour values for a row of entries, stepping fields toward targets and waiting on a band-limited list value before each write.
- `step_colour_nibbles_hi_field`: Steps the high-nibble fields of a stored colour toward their targets for each entry, waiting on a band-limited list value before each write.
- `step_colour_nibbles_row_variant`: Steps the four-bit colour fields toward targets for each row entry, waiting on a band-limited list value before each write.
- `step_colour_nibbles_row_loop`: Loops over rows, stepping each four-bit colour field toward its target and writing the packed result into the list.
- `decrement_colour_nibbles_to_zero`: Repeats sixteen rows, waiting until a list value falls within a fixed band, then decrements each four-bit colour field toward zero and writes it back.
- `decrement_colour_nibbles_loop`: Calls helpers and then, while entries remain, decrements each four-bit colour field toward zero and writes the packed result into the list.
- `wait_interrupt_bit_ack`: Polls the interrupt request register until bit four is set, then writes bit four to the interrupt request clear register and returns the bit value.

### Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 1 of 3).adf:Frs.exe

- `program_entry_copies_vector_block`: Program entry point.
- `open_graphics_save_state`: Calls system library routines through the exec base to open the graphics library and read its base.
- `nop_routine`: This routine is empty and performs no operations.
- `restore_custom_state_close`: Copies a nine-longword block back from a global area, writes saved values to the copper pointer, DMA control and interrupt enable registers, and calls library routines to close the library and release the system.
- `update_conditional_values`: Checks if a global variable matches a specific constant and, if so, writes several word and longword values to various offsets relative to that global variable.
- `wait_masked_position_return`: Loops until bits eight to sixteen of a hardware position field match a fixed pattern, then calls a helper and returns zero.
- `build_bitplane_pointer_copper_list`: Fills two sets of copper move entries in a global table, storing the high and low words of bitplane addresses at the bitplane pointer register offsets.
- `write_marked_entries_through_pointers`: Walks a table backward from a fixed index and, for each entry equal to two, writes that value through the pointer stored for that entry.
- `build_copper_list_for_row`: Divides a stored value by eleven to get a row index, derives a row address offset from it, and writes pairs of bitplane pointer words with plane-stride steps into the list.
- `update_position_from_mouse_port`: Reads a joystick and mouse counter word, extracts two movement bytes and adds them to stored X and Y position values.
- `render_text_into_bitplanes`: Renders a zero-terminated string into a bitplane area, mapping each character to a glyph index counted from the space character.
- `render_text_into_bitplanes_2`: Renders a zero-terminated string into a second bitplane area using the same glyph mapping, fixed row stride and line-break handling as its neighbour.
- `copy_global_block_to_pointer`: Copies a fixed block of longwords from a global area to the destination held in a global pointer.
- `fill_table_with_stepped_values`: Fills four consecutive table entries with word values that start at a fixed value and step by two, copying a global word into the partner slot of each entry.
- `render_text_row_variant`: Renders characters into bitplane rows at an offset chosen from the input byte, with a fixed starting row and stride.
- `start_blitter_copy_two_passes`: Waits for the blitter to be idle, then loads its control, mask, modulo and pointer registers and starts a blit of a fixed size.
- `swap_buffers_and_start_blit`: Swaps two stored buffer addresses, then waits for the blitter to be idle and starts a blit of fixed size with the destination pointer set to the saved address plus eight.
- `start_blit_to_computed_address`: Waits for the blitter to be idle, then sets its control word, source and destination pointers to an address computed from a stored value, sets the modulo fields and starts a fixed-size blit.
- `set_bits_from_product_index`: For each of 256 steps, multiplies two stored words, derives an index from the product and sets a single bit in a byte of the buffer at that index.
- `fill_word_table_scaled`: Fills 256 words in a global table, each set to a scaled table entry plus six, walking the source table with a wrapping index.
- `render_glyph_row_and_blit`: Every fourth call, advances to the next text byte, maps it to a glyph index counted from the space character and writes fixed-stride glyph rows into two plane addresses.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `build_chunk_pointer_table`: Scans a byte table for successive values and, for each value, fills thirty pointer slots with buffer chunk addresses spaced by a per-entry offset.
- `clear_memory_table`: Iterates through a memory range to find a specific byte value, then uses a pointer from a global variable to zero out a series of longwords in a table.
- `clear_audio_dma`: The routine writes to the four audio volume registers and to the DMA control register, setting the audio channels off and clearing DMA enables.
- `update_audio_pointers_from_table`: Advances a frame counter.
- `select_audio_channel_pointer_value`: Calls helpers and chooses one of three values from a counter taken modulo three.
- `decode_audio_channel_entries`: Reads packed four-byte audio entries from a stream and selects a table row from their high nibbles.
- `select_channel_value_from_table`: Selects a value from a global table using a counter taken modulo three and a nibble of a record byte, and stores it in a record field.
- `copy_word_field_between_records`: Copies a sixteen-bit field from one record offset into another offset in a second record, then returns.
- `find_table_threshold_entry`: Scans a row of a global table for the first entry not above a masked input value, falling back to a fixed entry when none qualifies.
- `step_value_toward_target`: Moves a record value toward a stored target by a step byte, adding or subtracting according to a direction flag, and snaps it to the target when it arrives.
- `compute_oscillating_offset`: Selects a value from a table or a sign-alternating ramp according to a mode nibble, scales it by an amplitude nibble, applies its sign flag and adds it to a base position.
- `decode_channel_effect_command`: Decodes the low nibble of a command byte into a channel effect.
- `copy_low_nibble_record_byte`: Copies the low nibble of one record byte into another record byte.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.

### Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 1 of 3).adf:TRDINTRO

- `program_entry_calls_free_memory`: Program entry point.
- `decrement_to_minus_one`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.
- `unpack_bitstream_into_tables`: Clears a 16-entry table on each pass, then decodes a bit stream using a shift routine, a masked table lookup and two table-filling routines.
- `shift_bit_stream`: Shifts bits from a register and updates a value using a bitmask or a value from a table.
- `lookup_table_masked_value`: The routine reads a 16-bit value from a table using an index derived from the input word, and returns that value masked with an input mask.
- `fill_two_tables`: The routine calls another routine to obtain a count, then fills a short table with that many values returned by the same routine, accumulating the values.
- `initialize_table_values`: Iteratively calculates and stores new values into a table using a pointer.

### Jungle Strike (1994)(Ocean)[cr Trader's Dream][t +6 Freezers](Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `generate_sine_lookup_tables`: Calculates a two-dimensional array of values using a nested loop of multiplication and division to create a trigonometric approximation.
- `split_long_word`: stores the high 16 bits of a 32-bit value from a register into a word at an offset from a pointer and stores the low 16 bits into another word at a different offset from that same pointer.
- `configure_blitter_and_rotate_sprites`: Sets up the Blitter hardware registers and copper instructions for a graphics operation.
- `parse_command_stream`: Iterates through a data stream to execute specific commands based on opcode bytes.
- `initialize_audio_and_entity_state`: Populates a table with incremental values and finds the maximum byte value within a memory range.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `update_audio_hardware_state`: Manages audio channel registers and interrupt settings through a recursive sequence of state updates.
- `update_dma_and_interrupt_registers`: Stores three values into a global variable and modifies DMA and interrupt control registers.
- `set_audio_registers`: Stores a series of specific values into several custom audio registers and sets a global variable.
- `process_entity_command_data`: Parses a command byte to update an entity's properties, including sprite or object dimensions, animation frames, and state flags.
- `update_entity_state_flags`: Updates an entity's internal state and properties based on a command byte.
- `update_position_accumulator`: Adjusts a signed value in a structure based on a signed increment and performs boundary checks to clamp the value.
- `calculate_scaled_offset`: Performs bitwise operations and multiplication on values from a table and a structure to update a destination word and a byte in a structure.
- `process_entity_state_updates`: Processes various state transitions for an entity based on a command byte, including updating position accumulators, calculating scaled offsets from a lookup table, and performing boundary clamping.
