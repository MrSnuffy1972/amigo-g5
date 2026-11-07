# Game 1666 (space harrier - return to the fantasy zone (1989)(elite)[cr power connection][h acu]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01666). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (911776 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 9 raw-word lines |
| Functions named and described | pass, 61 of 61 named (0 author symbols), 61 of 61 described (1 by rule, 57 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2273 of 2273 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3113 of 3113 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 53844 |
| code | 18774 |
| data | 785518 |
| padding | 904 |
| unused | 52736 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][h ACU].adf:bootblock | 1 | 18 | 1 |
| logged disk load from Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][h ACU].adf boot-block load | 1 | 107 | 2 |
| logged disk load from Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][h ACU].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2637 | 41 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 351 | 17 |

## Cases handled

- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][h acu]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][h ACU].adf: custom-format (non-DOS) disk, 1655 non-empty blocks: matched against track-loaded segments
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][h acu]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][h ACU].adf: the boot block reads 53760 bytes from disk offset 788480 to an address (found statically); recovered as a unit
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][h acu]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][h ACU].adf: the boot block reads 1536 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 1939 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 334 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][h acu]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][h ACU].adf: 2 logged loads cover 55296 bytes of the image
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][h acu]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][h ACU].adf: 20 track-loaded blocks found in recovered runtime segments and classified through them
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][h acu]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][h ACU].adf: blocks 11-311 (bytes 5632-159744) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.12)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][h acu]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][h ACU].adf: blocks 329-358 (bytes 168448-183808) outside the filesystem hold data, not code (opcode density 0.007, entropy 5.13)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][h acu]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][h ACU].adf: blocks 397-407 (bytes 203264-208896) outside the filesystem hold data, not code (opcode density 0.013, entropy 6.56)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][h acu]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][h ACU].adf: blocks 409-418 (bytes 209408-214528) outside the filesystem hold data, not code (opcode density 0.015, entropy 6.66)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][h acu]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][h ACU].adf: blocks 421-492 (bytes 215552-252416) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.67)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][h acu]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][h ACU].adf: blocks 504-753 (bytes 258048-386048) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.31)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][h acu]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][h ACU].adf: blocks 770-945 (bytes 394240-484352) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.48)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][h acu]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][h ACU].adf: blocks 968-1172 (bytes 495616-600576) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.86)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][h acu]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][h ACU].adf: blocks 1174-1539 (bytes 601088-788480) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.90)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][h acu]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][h ACU].adf: blocks 1650-1693 (bytes 844800-867328) hold ASCII text
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][h acu]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][h ACU].adf: blocks 1698-1759 (bytes 869376-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.60)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `execute_timed_loop` (runtime segment an address): Calculates a loop count based on a global value and executes a sequence of routine calls while modifying a global variable. Kind: loop-count/timing-based check. Pass path: sVar1 == -1 (loop completion or specific iteration count).

## Functions


### Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][h ACU].adf:bootblock

- `load_boot_data_init_video`: Fills in an exec I/O request and calls the exec request function twice to load data, then waits on the interrupt request register while changing the background colour register.

### logged disk load from Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][h ACU].adf boot-block load

- `return_shifted_longword_before_source`: Stores one byte from a register into the destination pointer and returns the 32-bit word just before the source pointer, shifted right with its top bit set.
- `shift_bits_count_loop`: Shifts a bit register right a set number of times, reloading it through one other routine whenever it empties.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_object_list_and_transformations`: Updates a linked list of objects by recalculating or pruning entries based on a certain threshold and modifies blitter transformation parameters or scaling values.
- `initialize_blitter_buffers_and_table`: Fills a column of positions with incrementing byte values, zeroes several memory blocks in rows while polling the blitter busy flag, and sets the blitter control, size and pointer registers.
- `run_conditional_object_updates`: Calls four routines in order, then calls further routines depending on signed fields of an object structure and a threshold comparison on a second field.
- `traverse_string_list`: Iterates through a list of pointers, calling another routine for any non-zero pointer found.
- `update_string_buffer_and_glyph_data`: Copies a block of data from a source table to a destination buffer and modifies a character buffer or string-related memory based on certain character checks.
- `remove_duplicate_elements`: Searches a list for duplicate values and shifts the remaining elements or clears a trailing entry to remove them.
- `call_object_handler_pointer`: Calls the routine whose pointer is stored at a fixed offset in an object structure, then returns.
- `update_object_handler_and_state`: Updates an object's function pointer and certain state-related fields, including a conditional update to a secondary pointer and a global variable based on a comparison of two values.
- `update_object_state`: Compares two values in a structure, then conditionally updates a structure and a global variable, and finally updates a function pointer in a structure.
- `add_bcd_digit_pairs`: Adds four pairs of bytes into decimal-coded counter bytes, applying a decimal adjustment after each addition.
- `find_empty_list_slot`: Searches a list for a null entry and replaces it with a new structure containing a pointer and a constant.
- `draw_tile_rows`: Draws a sequence of tile rows into the bitplanes using a masking process.
- `update_object_graphics`: Updates the visual state of an object by invoking a routine that renders masked tile data into the bitplanes.
- `draw_masked_tiles_to_planes`: Draws eight rows of tile data into four bitplanes, clearing masked bits and then OR-ing in the tile pattern for each plane.
- `setup_blitter_copy_rows`: Waits for the blitter to be free, then programs its control, modulo, first and last word mask, pointer and size registers for each pair of rows, decrementing a certain count until done.
- `set_global_pointer`: Sets a global variable to a routine address and writes constant values to memory locations relative to a register.
- `update_button_state`: Checks a hardware register for a button press, updates a global variable, and updates a table or pointer based on a bit-rotated value.
- `update_palette_and_interrupt_state`: Reads the interrupt request state, increments a frame counter, and copies eight longwords of colour data into the colour registers when a palette pointer is set.
- `init_four_audio_channels`: Sets the period and volume registers on all four audio channels and points each channel at a shared one-word sample with length one.
- `update_audio_frequency_and_envelope`: Updates frequency and volume or envelope parameters for audio channels based on certain flags and input or a global state.
- `update_blt_transform_parameters`: Calculates new scaling and positioning parameters based on a pointer-referenced structure and updates a series of global or local registers, including a destination address and a bitmask, intended for use with the blt hardware or a similar bitplane-based drawing routine.
- `setup_blitter_for_row_copy`: Waits for the blitter to be free, clears its certain registers, and then sets its pointers, size, and a modulo value computed from two inputs.
- `program_blitter_fixed_control`: Waits for the blitter, then writes fixed control words and the first and last word masks, along with source and destination modulo values of 42, and sets the pointer and size registers.
- `initialize_sprite_attribute_tables`: Zeroes a large memory region and copies row data from a table into it while recording row start pointers.
- `generate_pattern_bitmaps`: Generates bitmask or pattern data by processing a source buffer using bit-shifting and bit-replication or bit-reduction logic.
- `build_scaled_row_table`: Copies header values into an object and calculates a row stride based on a width rounded up to sixteen.
- `execute_timed_loop`: Calculates a loop count based on a global value and executes a sequence of routine calls while modifying a global variable.
- `loop_and_modify_byte`: Executes a loop that increments a global variable and then writes a constant value to a memory location, followed by a bit-clearing write to the same location.
- `update_global_and_clear_buffer`: Checks a global variable and a pointer, then conditionally flips the global variable and clears a certain range of a buffer or table.
- `wait_for_bit_clear`: Loops until a specific bit in a byte pointed to by a register is cleared.
- `wait_for_bit_clear_and_set_hardware`: Executes a bit-clearing wait loop followed by a routine that writes a structure to hardware or memory and loops until a flag bit is set.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `set_hardware_and_loop_on_flag`: Writes several values to a structure, then loops while a certain bit in a global variable is not set, finally writing more values to a structure.
- `update_global_and_bitmask`: Checks a global variable and a pointer, then conditionally modifies a bit in a buffer and recursively calls itself based on the difference between a global and a shifted version of itself.
- `reset_global_and_loop`: Sets a bit in a buffer and loops while a certain bit in a pointer-referenced byte is set, finally clearing a global variable.
- `search_and_interleave_bits`: Searches for a specific word in a table, then performs a bit-interleaving operation and writes the result to a destination buffer.
- `draw_glyph_into_bitplanes`: Maps a character to a glyph index, using separate ranges for digits and letters.
- `call_routine_for_each_char`: Loops over a null-terminated string, calling another routine for each character, and returns the input register unchanged.
- `call_routine_twice`: Calls another routine twice and returns.
- `call_helper_twice`: Calls another routine twice in a loop and then returns.
- `emit_nibbles_skipping_leading_zeros`: Walks the eight nibbles of a 32-bit value from the top, rotating one nibble per pass, and calls another routine for each non-zero nibble or the last nibble.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `manage_object_lifetime_and_state`: Decrements a counter and compares it against a threshold to update a status byte and toggle a flag.
- `advance_table_stepped_counter`: Decrements a counter at offset fourteen of the addressed structure and reloads it from a lookup table when it reaches zero, then adds a table entry to a byte at offset fifteen.
- `update_object_motion_and_status`: Decrements object lifetime counters and updates spatial coordinates by applying offsets from a lookup table.
- `reset_state_and_control_words`: Clears five state words in program data, then writes two control values to registers listed in the facts as the disk-and-audio control register and the DMA control register.
- `process_object_timer_complex`: Iterates through a data table to update object fields and performs an indirect call based on a calculated value.
- `update_object_flags`: Sets specific bits in an object structure and updates its fields by iterating through a data table.
- `update_object_flags_minimal`: Sets a bit in an object structure and performs a table-based update of object fields.
- `update_object_with_scaling`: Stores values from a table into an object structure after performing arithmetic scaling on a byte value.
- `update_object_offset`: Copies a byte from a table into an object structure at a specific offset and iterates through a data list.
- `set_object_status_bits`: Sets multiple status bits in an object structure and updates its internal fields using a data table.
- `process_object_timer`: Decrements a byte in an object structure and, if it reaches zero, modifies several fields within that structure based on a loop through a data table.
- `update_object_state_and_timer`: Updates an object's internal state by copying data from a source buffer and resetting its timer.
- `reset_object_timer_from_input`: When an object's mode byte is clear, it zeroes a counter, sets the mode value, and stores a byte in a global.
- `load_sequence_entry_fields`: Looks up a byte from a table indexed by an input value and stores it in an object field together with a second table-derived byte, then sets a countdown to one.
- `update_value_from_table_record`: When a mode byte is set, it looks up a ten-byte table record by index, decrements a counter, and stores a difference and an offset in the output fields.
- `copy_word_table_passes`: Runs a loop of passes over a table in program data, clearing certain fields, and copies 16-bit words between pointers or writes certain values to a memory location.
- `copy_table_to_block`: Calculates an offset based on an input value, zeroes a byte in a block, and then copies a series of words from a table to that block before setting a final word to all ones.
