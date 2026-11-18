# Game 1508 (dyna blaster (1991)(hudson soft)[cr vf][t +5 supplex][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01508). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (905440 top-level bytes in 18 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 64 of 64 named (0 author symbols), 64 of 64 described (1 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 569 of 569 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 845248 bytes, 39 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2712 of 2713 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 29333 |
| code | 3048 |
| data | 829145 |
| padding | 906 |
| unused | 43008 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf:bootblock | 1 | 21 | 1 |
| logged disk load from Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf boot-block load | 1 | 90 | 3 |
| logged disk load from Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf boot-block load | 1 | 124 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 590 | 19 |
| Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf:blocks 11-235 (bytes 5632-120832)#unpacked (raw code file, load address unknown) | 1 | 783 | 20 |
| Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf:blocks 242-289 (bytes 123904-148480)#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf:blocks 297-341 (bytes 152064-175104)#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf:blocks 352-458 (bytes 180224-235008)#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf:blocks 1100-1372 (bytes 563200-702976)#unpacked (raw code file, load address unknown) | 1 | 1105 | 19 |

## Cases handled

- dyna blaster (1991)(hudson soft)[cr vf][t +5 supplex][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf: custom-format (non-DOS) disk, 1669 non-empty blocks: matched against track-loaded segments
- extended reference run ignored: it does not re-execute 305 of the 573 PCs the canonical run logged (121382 PCs logged; the harness no longer reproduces the game), so its PCs and RAM are not used
- reference run failed (Karl 4 Nov 2026): run ext slid through memory (an address: 8 64 KB pages each with >= 29491 distinct PCs executed, 121382 of its 121382 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- dyna blaster (1991)(hudson soft)[cr vf][t +5 supplex][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf: the boot block reads 11264 bytes from disk offset 889856 to an address (found statically); recovered as a unit
- dyna blaster (1991)(hudson soft)[cr vf][t +5 supplex][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf: the boot block reads 17408 bytes from disk offset 236544 to an address (found statically); recovered as a unit
- runtime segment an address: 473 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 jump target
- dyna blaster (1991)(hudson soft)[cr vf][t +5 supplex][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf: 2 logged loads cover 28672 bytes of the image
- dyna blaster (1991)(hudson soft)[cr vf][t +5 supplex][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf: blocks 2-8 (bytes 1024-4608) outside the filesystem hold data, not code (opcode density 0.059, entropy 6.79)
- dyna blaster (1991)(hudson soft)[cr vf][t +5 supplex][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf: blocks 11-235 (bytes 5632-120832) hold a IMP: File Imploder stream (115200 -> 144684 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +5 supplex][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf:blocks 11-235 (bytes 5632-120832)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)[cr vf][t +5 supplex][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf: blocks 242-289 (bytes 123904-148480) hold a IMP: File Imploder stream (24576 -> 113460 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +5 supplex][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf:blocks 242-289 (bytes 123904-148480)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)[cr vf][t +5 supplex][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf: blocks 297-341 (bytes 152064-175104) hold a IMP: File Imploder stream (23040 -> 209510 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +5 supplex][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf:blocks 297-341 (bytes 152064-175104)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)[cr vf][t +5 supplex][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf: blocks 352-458 (bytes 180224-235008) hold a IMP: File Imploder stream (54784 -> 177594 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +5 supplex][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf:blocks 352-458 (bytes 180224-235008)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)[cr vf][t +5 supplex][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf: blocks 506-525 (bytes 259072-269312) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.96)
- dyna blaster (1991)(hudson soft)[cr vf][t +5 supplex][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf: blocks 1067-1098 (bytes 546304-562688) outside the filesystem hold data, not code (opcode density 0.013, entropy 7.09)
- dyna blaster (1991)(hudson soft)[cr vf][t +5 supplex][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf: blocks 1100-1372 (bytes 563200-702976) hold a IMP: File Imploder stream (139776 -> 200000 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +5 supplex][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf:blocks 1100-1372 (bytes 563200-702976)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- dyna blaster (1991)(hudson soft)[cr vf][t +5 supplex][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf: blocks 1375-1405 (bytes 704000-719872) hold a IMP: File Imploder stream (15872 -> 12960 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +5 supplex][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf: blocks 1408-1480 (bytes 720896-758272) hold a IMP: File Imploder stream (37376 -> 39240 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +5 supplex][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf: blocks 1485-1535 (bytes 760320-786432) hold a IMP: File Imploder stream (26112 -> 60160 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +5 supplex][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf: blocks 1540-1557 (bytes 788480-797696) hold a IMP: File Imploder stream (9216 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +5 supplex][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf: blocks 1562-1580 (bytes 799744-809472) hold a IMP: File Imploder stream (9728 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +5 supplex][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf: blocks 1584-1604 (bytes 811008-821760) hold a IMP: File Imploder stream (10752 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +5 supplex][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf: blocks 1606-1626 (bytes 822272-833024) hold a IMP: File Imploder stream (10752 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +5 supplex][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf: blocks 1628-1647 (bytes 833536-843776) hold a IMP: File Imploder stream (10240 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +5 supplex][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf: blocks 1650-1667 (bytes 844800-854016) hold a IMP: File Imploder stream (9216 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +5 supplex][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf: blocks 1672-1686 (bytes 856064-863744) hold a IMP: File Imploder stream (7680 -> 17196 bytes)
- dyna blaster (1991)(hudson soft)[cr vf][t +5 supplex][a]/Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf: blocks 1694-1710 (bytes 867328-876032) hold a IMP: File Imploder stream (8704 -> 17196 bytes)
- 5 never-loaded region(s), 845248 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 2 packed track region(s), 285184 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf:bootblock

- `start_boot_block_program`: The boot-block entry point, called by the Kickstart strap.

### logged disk load from Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf boot-block load

- `update_background_colour_value`: Works on values through four helper calls and writes the COLOR00 background colour register.
- `pack_byte_and_shift_longword`: Reads the longword just before its input pointer, stores its low byte combined with a high byte into a memory word, and returns that longword shifted right by one with the top bit set.
- `shift_bits_with_refill`: Shifts a bit register right one position per pass and calls an unnamed helper to reload it whenever it empties.

### logged disk load from Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf boot-block load

- `select_shifted_or_passthrough`: If a selector word equals one, it returns a 16-bit value shifted right by one.
- `lookup_table_word_masked`: Looks up a word in a program table, indexed by a register value doubled, and returns it masked with a second input.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `issue_trap_zero_call`: Stores the address of a routine in a global variable, issues trap zero, and returns.
- `set_cia_timer_wait_raster`: Writes CIA-B timer and control registers and the DMA control register, reads the CIA-A port register, and calls a helper that waits for a raster line.
- `handle_interrupt_request_entry`: The decompiler gave no body for this routine.
- `wait_until_value_then_halt`: Loops until a global holds a specific negative value, clears the global, then reaches a bad instruction that the decompiler cuts off.
- `store_three_address_length_pairs`: After reading a pointer from a structure, it stores three address and length pairs as constants in memory, then returns.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `start_blitter_rectangle_operation`: Sets blitter registers including control, first and last word masks, modulo, destination pointer and size, then runs a loop that writes rows through the blitter.
- `poll_joystick_state_machine`: Runs a small state machine on the fire-button and joystick port bits.
- `find_matching_range_entry`: Scans a table of four range entries in program data, checking whether a scaled value and a second value fall inside each entry's bounds, and returns the match.
- `prepare_buffer_block_pointers`: Computes two pointers into a buffer, one a fixed 4000 entries ahead of the other, and prepares them for a block transfer.
- `fill_global_bytes_with_pattern`: Writes a fixed character value into a block of bytes in a global area across seven passes, using two pointers.
- `advance_eight_step_lookup`: Counts a global up to eight, then resets it and copies bytes from two lookup tables into fixed memory locations.
- `start_blitter_after_wait`: Waits while a global flag is set, then writes zero to a blitter register, sets the destination pointer, control, first-word mask, modulo and size registers, and sets the global to a fixed value.
- `advance_wrapping_step_counter`: Adds two to a global on each call and wraps it at 720, calling three unnamed routines along the way.
- `scan_table_until_sentinel`: Walks a table of words in program data from a fixed address until it reaches a sentinel word, then does its work per entry.
- `set_blitter_masks_and_modulo`: Waits while a global flag bit is set, then writes fixed values to the blitter mask, data and modulo registers.
- `update_word_fields_in_loop`: A 69-instruction routine that updates word-sized values through a loop.
- `start_blitter_after_wait_2`: Waits while a global flag is set, sets the global to a fixed value, and writes the blitter destination, control, first-word, modulo and size registers.
- `clear_strided_longword_table`: Writes zero into a longword every 80 bytes, 140 times, starting at a fixed address.

### Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf:blocks 11-235 (bytes 5632-120832)#unpacked (raw code file, load address unknown)

- `initialize_dma_and_object_structures`: Sets the DMA control register to enable all four DMA channels, clears a block of memory, and populates a table of four values with specific constants.
- `update_dma_and_object_states`: Checks several global status bytes and, if conditions are met, updates a series of object structures by calling routines to initialize, load parameters, and clamp values.
- `advance_sample_loop_for_channels`: Walks three sound channels and advances each sample pointer by a step, handling one-shot, forward loop and ping-pong reversal modes.
- `load_channel_pattern_fields`: Loads a channel's pattern entry from a table indexed by a row byte, copying a word pointer and two byte parameters into the channel record and clearing a flag.
- `apply_channel_slide_step`: Calls a helper to update a pointer, then when a reset flag is set clears two accumulator words, and otherwise adds a stored delta to a channel output word and advances an accumulator by a step each tick while it is nonzero.
- `read_next_channel_command_byte`: Clears two channel counters, then either counts down a hold delay or reads the next command byte from a stream and stores its parameter bytes, storing a hold count when the byte is negative.
- `start_channel_note_from_table`: Clears the effect state fields of the record when the trigger at offset 46 is set, then loads a sample header from a table row indexed by the trigger, copying the pointer and length into the record and computing the initial period from a lookup table.
- `update_channel_period_with_clamp`: Calls the eight effect helpers for one tick, adds the slide amount read at offset 58 to the channel period at offset 10, clamps the result to a floor of 95 and a ceiling of 5760, and stores the clamped value to the period output at offset 6 of the structure in a6 indexed by the repeat count saved at entry.
- `step_repeat_counter`: Reads a repeat count from offset 12 of the pointed-to record, and when it is nonzero increments the counter at offset 37 unless it already matches the value at offset 13, in which case it adds the count to the counter at offset 38.
- `update_entity_state_attributes`: Extracts a command index from a word at offset 50 of a structure in a2 and performs conditional operations based on that index.
- `slide_period_toward_note_target`: Looks up a target period from a table using a note value at offset 50, then moves the period at offset 10 toward that target by the step stored at offset 62, clearing the step and snapping the period to the target when reached, and returns the current period.
- `set_pitch_from_command_table`: Computes a table index from the command word and a byte in the caller's data, then loads the pitch value at that index into the record.
- `negate_field_and_subtract`: Stores the negated value of one record field into a second field and subtracts that value from a third field.
- `adjust_counter_by_scaling`: When the field at offset 18 is zero and the byte at offset 4 of the record in a0 is zero, updates offset 13 from another record, then either adds a scaled value to or subtracts a scaled value from the counter at offset 12 depending on a bitmask, clamping the result.
- `step_secondary_envelope_pitch`: Advances the secondary envelope counter at offset 43 by comparing it to a segment length, reloading it from the length minus the step and adding a table value to the pitch at offset 10 when the segment ends, otherwise incrementing the counter.
- `step_primary_envelope_pitch`: Advances the primary envelope counter at offset 39 using its own record fields, reloading it and adding a lookup-table value to the pitch at offset 10 when the segment ends, otherwise incrementing the counter.
- `copy_sequence_entry_to_buffer`: Advances the sequence counter at offset 35, and when the segment ends fetches a byte from a table, negating the counter when the byte is negative, otherwise copying the long and word fields of a 64-byte entry from the table into the output buffer at a running offset and recording the entry index.
- `scale_position_into_buffer`: Calls one helper, copies a position word into a second field, and writes the high half of its product with a scale field into an output buffer at a running offset.
- `run_envelope_state_machine`: Advances the record's level envelope one step according to its state: a ramp up, a ramp down, a counted pause, or a timed hold, moving to the next state when each one finishes.
- `write_dma_control_registers`: Checks a flag word and, when clear, clears it, then indexes a table using a scaled value in d0 to fetch a byte that is written to several global variables before setting bit 15 of the DMA control register to enable DMA channels.

### Dyna Blaster (1991)(Hudson Soft)[cr VF][t +5 Supplex][a].adf:blocks 1100-1372 (bytes 563200-702976)#unpacked (raw code file, load address unknown)

- `initialize_system_and_tables`: Sets hardware registers, calls three routines, clears a block of memory using a loop, and processes a data table to populate various memory locations based on specific byte values.
- `calculate_table_offset`: Calculates two different offsets based on the input in d0 and adds them to a base address in a5, then returns.
- `clear_memory_blocks`: Calls a helper routine, then uses a loop to fill two memory blocks starting at offsets of a5 and a5 plus 8400 with zeros.
- `update_table_values`: Calculates a value by reading from a table using an offset derived from a byte at a specific offset in a structure, then performs a bitwise OR to store this value into two 32-bit words at offsets 0 and 8400 of a destination structure in a5 through a loop.
- `fill_table_from_source`: Copies bytes from a source table to a destination table, incrementing a byte value for each entry until a byte of all ones is encountered.
- `initialize_hardware_and_tables`: Sets a CIA register, configures several hardware registers, and populates a table with specific values derived from a calculation involving a nibble-based bit manipulation.
- `generate_lookup_tables`: Fills a table starting at offset 192 with values calculated via division, populates a second table starting at offset 8116 with bit-rotated bytes, and populates a third table starting at offset 8772 with incrementing 16-bit values.
- `perform_bit_manipulation_and_scaling`: Performs a loop of bitwise XOR operations and shifts on a value, then returns a result that is either a bitwise mask or a scaled product of the input.
- `find_block_index_and_clear_audio`: Finds the smallest byte greater than the running minimum in the table at 952 bytes after the pointer stored at offset a fixed address, then uses that byte as an index to zero-fill and populate a table of longwords, clears several structure fields and custom-chip registers, enables DMA, and returns.
- `init_audio_and_load_block_entry`: Initializes the four audio channels by loading their volume tables and then loads a block entry from a table using an index in the structure in a6, copying its fields into offsets 4 through 40 of that structure.
- `load_and_apply_block_entry`: Loads a block entry from a table using an index in the structure in a6, copies its fields into the structure at offsets 4 through 40, and writes the resulting position values into the structure in a5.
- `dispatch_block_byte_values_2`: Reads the low nibble of the byte at offset 2 of the structure in a6 and dispatches on it, using offset 9 of that structure to index a table of 72-bit entries and offset 8 as a 16-bit value, while writing a 16-bit result to offset 6 of the structure in a5.
- `dispatch_block_byte_values_3`: Copies the 16-bit value at offset 16 of the structure in a6 to offset 6 of the structure in a5 and returns.
- `search_clamp_table`: Searches a table for the first entry greater than the value at offset 0 in a6, adjusts the index by the byte at offset 9, stores the found value at offset 24, and sets the byte at offset 11 to 1 when the value is smaller.
- `clamp_and_search_table`: Clamps the 16-bit value at offset 16 of the structure in a6 toward the value at offset 24, then reads byte 31 and the low nibble of byte 18, multiplies it by 72, and scans a table for the first entry greater than the clamped value, advancing by 2 each step until reaching 72, then stores that index in the structure in a5.
- `apply_signed_delta_value`: Computes a signed value from a byte and a table lookup, applies its sign to a value at offset 16 of a structure in a5, and adds a scaled nibble to a counter at offset 27 before returning.
- `dispatch_block_byte_values`: Reads the low nibble of the byte at offset 2 in the structure in a6 and dispatches on its value, with 9 scaling a value in a5, 12 writing a nibble into a byte in a5, and 13 returning zero or the byte at offset 3.
- `store_low_nibble_byte`: Extracts the low nibble of byte 3 in the structure in a6 and stores it at offset 18, then returns.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
