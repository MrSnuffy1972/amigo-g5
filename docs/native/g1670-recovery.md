# Game 1670 (starglider 2 - the egron's strike back (1988)(rainbird)[cr unit a - wow][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01670). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903664 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 31 of 31 named (0 author symbols), 31 of 31 described (3 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 463 of 463 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 52224 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 688 of 958 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 5906 |
| data | 787150 |
| padding | 16 |
| unused | 110592 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Starglider 2 - The Egron's Strike Back (1988)(Rainbird)[cr Unit A - WOW][a].adf:bootblock | 1 | 246 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 594 | 22 |
| Starglider 2 - The Egron's Strike Back (1988)(Rainbird)[cr Unit A - WOW][a].adf bytes 5632-11776 (code never loaded by a reference run) | 1 | 0 | 0 |
| Starglider 2 - The Egron's Strike Back (1988)(Rainbird)[cr Unit A - WOW][a].adf bytes 67584-98304 (code never loaded by a reference run) | 1 | 0 | 0 |
| Starglider 2 - The Egron's Strike Back (1988)(Rainbird)[cr Unit A - WOW][a].adf bytes 98816-114176 (code never loaded by a reference run) | 1 | 118 | 1 |

## Cases handled

- starglider 2 - the egron's strike back (1988)(rainbird)[cr unit a - wow][a]/Starglider 2 - The Egron's Strike Back (1988)(Rainbird)[cr Unit A - WOW][a].adf: custom-format (non-DOS) disk, 1542 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 463 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- starglider 2 - the egron's strike back (1988)(rainbird)[cr unit a - wow][a]/Starglider 2 - The Egron's Strike Back (1988)(Rainbird)[cr Unit A - WOW][a].adf: bytes 5632-11776 look like code no run loaded: recovered position-independently (load address unknown)
- starglider 2 - the egron's strike back (1988)(rainbird)[cr unit a - wow][a]/Starglider 2 - The Egron's Strike Back (1988)(Rainbird)[cr Unit A - WOW][a].adf: bytes 67584-98304 look like code no run loaded: recovered position-independently (load address unknown)
- starglider 2 - the egron's strike back (1988)(rainbird)[cr unit a - wow][a]/Starglider 2 - The Egron's Strike Back (1988)(Rainbird)[cr Unit A - WOW][a].adf: bytes 98816-114176 look like code no run loaded: recovered position-independently (load address unknown)
- starglider 2 - the egron's strike back (1988)(rainbird)[cr unit a - wow][a]/Starglider 2 - The Egron's Strike Back (1988)(Rainbird)[cr Unit A - WOW][a].adf: 3 logged loads cover 52224 bytes of the image
- starglider 2 - the egron's strike back (1988)(rainbird)[cr unit a - wow][a]/Starglider 2 - The Egron's Strike Back (1988)(Rainbird)[cr Unit A - WOW][a].adf: 4 track-loaded blocks found in recovered runtime segments and classified through them
- starglider 2 - the egron's strike back (1988)(rainbird)[cr unit a - wow][a]/Starglider 2 - The Egron's Strike Back (1988)(Rainbird)[cr Unit A - WOW][a].adf: blocks 27-30 (bytes 13824-15872) outside the filesystem hold data, not code (opcode density 0.023, entropy 5.06)
- starglider 2 - the egron's strike back (1988)(rainbird)[cr unit a - wow][a]/Starglider 2 - The Egron's Strike Back (1988)(Rainbird)[cr Unit A - WOW][a].adf: blocks 224-229 (bytes 114688-117760) outside the filesystem hold data, not code (opcode density 0.057, entropy 5.45)
- starglider 2 - the egron's strike back (1988)(rainbird)[cr unit a - wow][a]/Starglider 2 - The Egron's Strike Back (1988)(Rainbird)[cr Unit A - WOW][a].adf: blocks 231-647 (bytes 118272-331776) outside the filesystem hold data, not code (opcode density 0.028, entropy 5.97)
- starglider 2 - the egron's strike back (1988)(rainbird)[cr unit a - wow][a]/Starglider 2 - The Egron's Strike Back (1988)(Rainbird)[cr Unit A - WOW][a].adf: blocks 651-679 (bytes 333312-348160) outside the filesystem hold data, not code (opcode density 0.017, entropy 6.53)
- starglider 2 - the egron's strike back (1988)(rainbird)[cr unit a - wow][a]/Starglider 2 - The Egron's Strike Back (1988)(Rainbird)[cr Unit A - WOW][a].adf: blocks 688-692 (bytes 352256-354816) outside the filesystem hold data, not code (opcode density 0.020, entropy 4.71)
- starglider 2 - the egron's strike back (1988)(rainbird)[cr unit a - wow][a]/Starglider 2 - The Egron's Strike Back (1988)(Rainbird)[cr Unit A - WOW][a].adf: blocks 772-1197 (bytes 395264-613376) outside the filesystem hold data, not code (opcode density 0.016, entropy 5.82)
- starglider 2 - the egron's strike back (1988)(rainbird)[cr unit a - wow][a]/Starglider 2 - The Egron's Strike Back (1988)(Rainbird)[cr Unit A - WOW][a].adf: blocks 1199-1261 (bytes 613888-646144) outside the filesystem hold data, not code (opcode density 0.034, entropy 4.54)
- starglider 2 - the egron's strike back (1988)(rainbird)[cr unit a - wow][a]/Starglider 2 - The Egron's Strike Back (1988)(Rainbird)[cr Unit A - WOW][a].adf: blocks 1364-1366 (bytes 698368-699904) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.35)
- starglider 2 - the egron's strike back (1988)(rainbird)[cr unit a - wow][a]/Starglider 2 - The Egron's Strike Back (1988)(Rainbird)[cr Unit A - WOW][a].adf: blocks 1375-1759 (bytes 704000-901120) outside the filesystem hold data, not code (opcode density 0.045, entropy 5.94)
- 3 never-loaded region(s), 52224 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 50176 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Starglider 2 - The Egron's Strike Back (1988)(Rainbird)[cr Unit A - WOW][a].adf:bootblock

- `boot_copy_and_poll_raster`: Boot block entry that copies a block of code and a short table into low memory, patches one copied longword, and polls a video position register before writing custom-chip setup registers and storing a four-character tag.
- `enable_dma_after_raster_wait`: Patches a longword at the argument pointer, waits for a video position, writes custom-chip sprite and DMA registers, then sets the DMA control and interrupt enable registers.
- `return_constant`: Returns the constant 145 in d0 to the caller without any other effect.
- `boot_program_custom_chip_state`: Boot block routine that calls a library function at a fixed offset and writes the colour, DMA control, interrupt enable and interrupt request registers along with one more custom-chip register.
- `copy_longs_and_set_colour`: Copies a long run of longwords from a source address to low memory, then writes a fixed value to a colour register and patches a byte in the copied area.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `stub_bad_instruction_data`: The decompiled listing is a single undecodable-bytes marker for 185 instructions, so no behaviour is recovered from this entry.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_raster_and_setup_registers`: Waits for a video position, then writes custom-chip registers and calls a series of nine routines in a loop until a busy bit clears.
- `shift_table_entries`: Decrements a counter in a global; when it reaches zero it resets the counter to three and shifts seven word entries of a table forward by one, storing the saved first word in the last slot.
- `write_pair_then_call_loop`: Calls a routine, writes the same constant value to two offsets of a structure, then calls another routine 24 times in a loop.
- `advance_countdown_timer_entry`: Writes two constant values into a structure, then decrements a countdown word.
- `copy_longs_to_fixed_block`: Copies sixteen longwords from a global area to a fixed low-memory address and then sets a global to that address.
- `step_frame_state_counter`: Decrements a small state counter held in globals and, when it reloads, steps a state word through a fixed sequence with wrap-around.
- `store_two_buffer_pointers`: Stores two buffer addresses and two counts into four global words and returns.
- `pack_input_bits_value`: Combines flag bits taken from two input words and a shifted coordinate into a packed 32-bit value, and returns a 16-bit result derived from one input.
- `store_struct_from_registers`: Calls a routine, then writes shifted short values from registers and two longs from the stack into fields of a structure, duplicating one long into another field.
- `reset_object_state`: Calls a routine, clears a block of memory pointed to by a global, then stores fixed values into fields of a structure and copies a pointer into another field.
- `wait_blitter_idle`: Spins until the blitter busy flag in the DMA status register clears.
- `clear_table_and_copy_block`: Calls a routine and clears two words in each of 24 sixteen-byte table entries, then copies sixteen longwords from a global area to a fixed low-memory address.
- `update_entries_by_division`: Iterates over 24 entries of 16-bit records, decrements a per-entry counter, and recomputes each record's position by division until the result lies within fixed bounds.
- `update_seed_and_modulo`: Advances a 16-bit state held in a global by a multiply, an add and a reduction, looping until no carry remains, then returns the new state modulo the input argument.
- `scale_and_store_word_pair`: Calls a routine twice; stores the first result, offset by 176 and shifted left by eight, into a word, then stores the second result shifted left by eight into the next word, followed by a constant word.
- `initialize_display_list`: When a global counter is zero, copies pairs of words from a table into a buffer and recurses to fill it; it then writes fields of a structure, including a word derived from the counter, and calls another routine.
- `find_next_nonzero_entry`: Scans a byte table from a rotating global index for the next non-zero entry, decodes it by XOR and tests whether it lies in a fixed range; in-range values trigger a block copy of table entries.
- `initialize_global_table`: Writes a series of fixed addresses into global variables, clears fifteen entries of a global table to zero, and copies one global pointer into another.
- `clear_audio_volume_state`: Writes zero to six shared globals and fifteen to one more.
- `tick_audio_step_counter`: Returns at once when a global counter is zero; otherwise it increments the counter and, on reaching six, resets it, calls four routines, runs a short delay loop and adjusts a global.
- `program_audio_channel_registers`: Decodes a packed control byte for a sound sample and programs the location and length registers of the four audio channels, enabling their DMA through the DMA control register.
- `update_nibble_indexed_record`: Reads a byte from a source record, uses its high nibble to index a table, and copies table values into fields of a destination record.

### Starglider 2 - The Egron's Strike Back (1988)(Rainbird)[cr Unit A - WOW][a].adf bytes 98816-114176 (code never loaded by a reference run)

- `report_unrecovered_routine`: The decompiler timed out on this 118-instruction routine, so its behaviour is not recovered from the listing.
