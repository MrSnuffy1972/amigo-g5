# Game 1667 (space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +1 miracle]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01667). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (911776 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 9 raw-word lines |
| Functions named and described | pass, 61 of 61 named (0 author symbols), 61 of 61 described (1 by rule, 58 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2273 of 2273 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3169 of 3169 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 11953 |
| code | 18918 |
| data | 824585 |
| unused | 56320 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +1 Miracle].adf:bootblock | 1 | 18 | 1 |
| logged disk load from Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +1 Miracle].adf boot-block load | 1 | 163 | 2 |
| logged disk load from Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +1 Miracle].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2637 | 41 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 351 | 17 |

## Cases handled

- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +1 miracle]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +1 Miracle].adf: custom-format (non-DOS) disk, 1642 non-empty blocks: matched against track-loaded segments
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +1 miracle]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +1 Miracle].adf: the boot block reads 11264 bytes from disk offset 788480 to an address (found statically); recovered as a unit
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +1 miracle]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +1 Miracle].adf: the boot block reads 1536 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 1939 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 334 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +1 miracle]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +1 Miracle].adf: 2 logged loads cover 12800 bytes of the image
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +1 miracle]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +1 Miracle].adf: 20 track-loaded blocks found in recovered runtime segments and classified through them
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +1 miracle]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +1 Miracle].adf: blocks 11-32 (bytes 5632-16896) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.86)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +1 miracle]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +1 Miracle].adf: blocks 44-311 (bytes 22528-159744) outside the filesystem hold data, not code (opcode density 0.007, entropy 7.14)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +1 miracle]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +1 Miracle].adf: blocks 329-358 (bytes 168448-183808) outside the filesystem hold data, not code (opcode density 0.007, entropy 5.14)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +1 miracle]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +1 Miracle].adf: blocks 397-407 (bytes 203264-208896) outside the filesystem hold data, not code (opcode density 0.013, entropy 6.56)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +1 miracle]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +1 Miracle].adf: blocks 409-418 (bytes 209408-214528) outside the filesystem hold data, not code (opcode density 0.015, entropy 6.66)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +1 miracle]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +1 Miracle].adf: blocks 421-492 (bytes 215552-252416) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.67)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +1 miracle]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +1 Miracle].adf: blocks 504-753 (bytes 258048-386048) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.31)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +1 miracle]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +1 Miracle].adf: blocks 770-945 (bytes 394240-484352) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.48)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +1 miracle]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +1 Miracle].adf: blocks 968-1172 (bytes 495616-600576) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.83)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +1 miracle]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +1 Miracle].adf: blocks 1174-1539 (bytes 601088-788480) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.86)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +1 miracle]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +1 Miracle].adf: blocks 1650-1759 (bytes 844800-901120) hold ASCII text
- 1 packed track region(s), 42496 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `execute_timed_loop` (runtime segment an address): Calculates a loop count based on a global value and executes a sequence of routine calls while modifying a global variable. Kind: loop-count/timing-based check. Pass path: sVar1 == -1 (loop completion or specific iteration count).

## Functions


### Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +1 Miracle].adf:bootblock

- `initialize_boot_device`: Initializes a device structure, then calls a library routine through a global pointer to obtain a handle and pass it to another function.

### logged disk load from Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +1 Miracle].adf boot-block load

- `read_longword_then_shift_out`: Reads a longword just before the input pointer, stores its low byte at the destination, and returns the longword shifted right by one with its top bit forced on, so the caller receives the next bit.
- `shift_bits_refill_when_empty`: Shifts a bit register right one bit at a time for a counted number of bits, and reloads the register from the bit reader when it runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_object_list_and_transform_params`: Updates a linked list of objects by recalculating or removing entries based on a lifetime or distance threshold, and updates scaling and positioning parameters for blitter-based transformations.
- `prepare_blitter_frame_buffer`: Fills a column of positions with incrementing byte values, zeroes several memory blocks in rows while polling the blitter busy flag, and sets the blitter control, size and pointer registers.
- `run_conditional_object_updates`: Executes several routines and then conditionally executes further routines based on values found in an object structure.
- `traverse_string_list`: Recursively traverses a list of pointers, checking if the value at each pointer is non-zero.
- `update_string_buffer_and_glyph_data`: Copies a block of data from a source table to a destination buffer and modifies a character buffer or string-related memory based on certain character checks.
- `remove_duplicate_elements`: Searches a list for duplicate values and shifts the remaining elements or clears a trailing entry to remove them.
- `call_object_handler_pointer`: Calls the routine whose pointer is stored at a fixed offset in an object structure.
- `update_object_recursive_state`: Recursively updates an object's internal state and function pointer, calculating new timing or position-related values and conditionally updating or resetting object members and a global variable.
- `update_object_state`: Checks a condition based on a difference between two values in an object structure and updates object members or a global variable accordingly.
- `add_bcd_digit_pairs`: Adds four pairs of bytes into decimal-coded counter bytes, applying a decimal adjustment after each addition.
- `find_empty_list_slot`: Searches a list for a null entry and replaces it with a new structure containing a pointer and a constant.
- `render_masked_tiles`: Invokes a routine that draws eight rows of tile data into four bitplanes, clearing masked bits and then OR-ing in the tile pattern for each plane.
- `render_masked_tile_data`: Invokes a routine to draw masked tile data into the bitplanes.
- `draw_masked_tiles_to_planes`: Draws eight rows of tile data into four bitplanes, clearing masked bits and then OR-ing in the tile pattern for each plane.
- `setup_blitter_copy_rows`: Waits for the blitter to be free, then programs its control, modulo, first and last word mask, pointer and size registers for each pair of rows, decrementing a count until done.
- `set_global_pointer`: Sets a global variable to a specific routine address and writes a constant value to a memory location relative to a register.
- `update_button_state`: Checks a hardware register for a button press, updates a global variable, and updates a table or pointer based on a bit-rotated value.
- `update_palette_and_interrupt_state`: Copies eight longwords of color data from a pointer into the hardware color registers, increments a frame counter, and clamps a value to a certain range.
- `init_four_audio_channels`: Sets the period and volume registers on all four audio channels and points each channel at a shared one-word sample with length one, waiting on audio interrupt requests.
- `update_audio_frequency_envelope`: Updates frequency or volume-related parameters or envelopes by checking or modifying certain global values.
- `update_blt_transform_parameters`: Calculates new scaling and positioning parameters based on a pointer-referenced structure and updates a series of global or or local registers, including a destination address and a bitmask, intended for use with the or-to-blt hardware or a similar bitplane-based drawing routine.
- `prepare_blitter_for_row_copy`: Waits for the blitter to be free, clears its certain registers, and then sets its pointers, size, and a modulo value computed from two inputs.
- `program_blitter_fixed_control`: Waits for the blitter, then writes fixed control words and the first and last word masks, along with source and destination modulo values of 42, and sets the pointer and size registers.
- `initialize_render_table_and_copy_data`: Zeroes a large region of memory, then copies rows from a table into it while recording each row's start pointer in a list.
- `generate_pattern_bitmaps`: Calculates and writes bitmask or pattern data into two separate memory buffers using bit-shifting and bit-replication or bit-reduction logic.
- `build_scaled_row_table`: Copies header values into an object, calculates a row stride rounded up to a multiple of sixteen, and populates a lookup table by scaling coordinates using a division-based lookup.
- `execute_timed_loop`: Calculates a loop count based on a global value and executes a sequence of routine calls while modifying a global variable.
- `modify_buffer_bytes`: Executes a loop that sets a global variable and then writes certain byte values to a buffer.
- `update_global_and_clear_buffer`: Checks a global variable and a pointer, then conditionally flips the global variable and clears a certain range of a buffer or table.
- `wait_for_bit_clear`: Loops until a specific bit in a memory location is cleared.
- `wait_for_flag_clear`: Waits for a specific bit in a memory location to be cleared and writes several values to a structure before looping on a flag.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `set_hardware_and_loop_on_flag`: Writes several values to a structure, then loops while a certain bit in a global variable is not set, finally writing more values to a structure.
- `update_global_and_bitmask`: Checks a global variable and a pointer, then conditionally modifies a bit in a buffer and recursively calls itself based on the difference between a global and a shifted version of itself.
- `reset_global_and_loop`: Sets a bit in a buffer and loops while a certain bit in a pointer-referenced byte is set, finally clearing a global variable.
- `search_and_interleave_bits`: Searches for a specific word in a table, then performs a bit-interleaving operation and writes the result to a destination buffer.
- `draw_glyph_into_bitplanes`: Maps a character to a glyph index, using separate ranges for digits and letters.
- `call_routine_for_each_char`: Loops over a null-terminated string, calling another routine for each character, and returns the input register unchanged.
- `call_routine_twice`: Executes another routine twice before returning.
- `call_helper_twice`: Calls another routine twice in a loop and then returns.
- `emit_nibbles_skipping_leading_zeros`: Walks the eight nibbles of a 32-bit value from the top, rotating one nibble per pass, and calls another routine for each non-zero nibble or the last nibble.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_object_lifetime_and_transform_2`: Updates a pointer-based object property and a lifetime counter, conditionally toggling a bit or decrementing/incrementing a counter based on a comparison.
- `advance_table_stepped_counter`: Decrements a counter at offset fourteen of the addressed structure and reloads it from a lookup table when it reaches zero, then adds a table entry to a byte at offset fifteen.
- `update_object_lifetime_and_transform`: Decrements object lifetime counters and updates object position, scaling, and hardware register or global variable values based on certain thresholds.
- `reset_state_and_control_words`: Clears five state words in program data, then writes two control values to registers listed in the facts as the disk-and-audio control register and the DMA control register.
- `update_object_attribute_from_table`: Calculates an address from a table and updates object fields with values or pointers derived from that table.
- `update_object_and_pointer`: Sets certain bits in an object, then iterates through a list to update object fields using a table-based lookup.
- `set_object_bit_and_iterate`: Sets a bit in an object and then iterates through a list to update object fields using a table-based lookup.
- `update_object_from_source`: Copies bytes from a source buffer to an object, then iterates through a list to update object fields using a table-based lookup.
- `update_object_attribute_loop`: Copies a byte from a source to an object and then iterates through a list to update object fields using a table-based lookup.
- `update_object_attribute_table`: Iterates through a list of object properties, updating or replacing attribute bytes based on a lookup table and conditional range checks.
- `update_object_properties`: Decrements a byte in an object structure and conditionally updates other object fields or jumps to a new routine based on a table lookup.
- `update_object_state_and_timer`: Updates an object's mode and timer or pointer-based properties, and conditionally executes a jump or recursive call based on a value-dependent jump table or mode check.
- `reset_object_timer_from_input`: When an object's mode byte is clear, it zeroes a counter, sets a mode value, and stores a byte in a global, then copies a timer byte and stores a difference of two inputs while touching the DMA control register.
- `load_sequence_entry_fields`: Looks up a byte from a table indexed by an input value and stores it in an object field together with a second table-derived byte, then sets a countdown to one.
- `update_value_from_table_record`: When a mode byte isset, it looks up a ten-byte table record by index, decrements a counter, and stores a difference and an offset in the output fields.
- `copy_word_table_passes`: Runs a loop of passes over a table in program data, clearing certain fields, and copies 16-bit words between pointers or writes certain values to a memory location.
- `copy_table_to_memory`: Calculates an offset based on an input value, zeroes a byte at that location, and then copies a series of words from a table to a destination block.
