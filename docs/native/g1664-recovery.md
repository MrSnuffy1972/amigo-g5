# Game 1664 (space harrier - return to the fantasy zone (1989)(elite)[cr power connection]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01664). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (910560 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 28 raw-word lines |
| Functions named and described | pass, 45 of 45 named (0 author symbols), 45 of 45 described (0 by rule, 42 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1648 of 1648 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2848 of 2848 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 53844 |
| code | 17274 |
| data | 787850 |
| padding | 904 |
| unused | 50688 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection].adf:bootblock | 1 | 18 | 1 |
| logged disk load from Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection].adf boot-block load | 1 | 107 | 2 |
| logged disk load from Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2399 | 26 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 324 | 16 |

## Cases handled

- space harrier - return to the fantasy zone (1989)(elite)[cr power connection]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection].adf: custom-format (non-DOS) disk, 1659 non-empty blocks: matched against track-loaded segments
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection].adf: the boot block reads 53760 bytes from disk offset 788480 to an address (found statically); recovered as a unit
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection].adf: the boot block reads 1536 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 1355 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 293 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection].adf: 2 logged loads cover 55296 bytes of the image
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection].adf: 18 track-loaded blocks found in recovered runtime segments and classified through them
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection].adf: blocks 11-311 (bytes 5632-159744) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.12)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection].adf: blocks 327-358 (bytes 167424-183808) outside the filesystem hold data, not code (opcode density 0.016, entropy 5.30)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection].adf: blocks 397-407 (bytes 203264-208896) outside the filesystem hold data, not code (opcode density 0.013, entropy 6.56)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection].adf: blocks 409-418 (bytes 209408-214528) outside the filesystem hold data, not code (opcode density 0.015, entropy 6.66)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection].adf: blocks 421-492 (bytes 215552-252416) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.67)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection].adf: blocks 504-753 (bytes 258048-386048) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.31)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection].adf: blocks 770-945 (bytes 394240-484352) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.48)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection].adf: blocks 968-1172 (bytes 495616-600576) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.83)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection].adf: blocks 1174-1539 (bytes 601088-788480) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.86)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection].adf: blocks 1650-1759 (bytes 844800-901120) hold ASCII text

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection].adf:bootblock

- `load_boot_data_init_video`: Fills in an exec I/O request and calls the exec request function twice to load data, then waits on the interrupt request register while changing the background colour register.

### logged disk load from Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection].adf boot-block load

- `return_shifted_longword_before_source`: Stores one byte from a register into the destination pointer and returns the 32-bit word just before the source pointer, shifted right with its top bit set.
- `shift_bits_count_loop`: Shifts a bit register right a set number of times, reloading it through one other routine whenever it empties.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_object_lifetimes_and_positions`: Iterates through lists of object structures to decrement lifetime counters, remove expired entries by updating linked list pointers, and calculate new coordinate and scaling values based on velocity and threshold comparisons.
- `clear_buffers_and_wait_blitter`: Fills a column of positions with incrementing byte values, zeroes several memory blocks in rows while polling the blitter busy flag, and sets the blitter control, size and pointer registers.
- `run_conditional_object_updates`: Calls four routines in order, then calls further routines depending on signed fields of an object structure and a threshold comparison on a second field.
- `process_nested_list_elements`: Performs a nested loop that iterates through a list of bytes, calling another routine for every non-zero byte encountered until the end of the list is reached.
- `update_string_buffer_and_glyph_data`: Copies a block of data from a source table to a destination buffer and modifies a character buffer or string-related memory based on certain character checks.
- `remove_duplicate_elements`: Searches a list for duplicate values and shifts the remaining elements or clears a trailing entry to remove them.
- `call_object_handler_pointer`: Calls the routine whose pointer is stored at a fixed offset in an object structure, then returns.
- `update_object_state_logic`: If a 16-bit value at offset 22 of a structure in a6 is non-zero, it updates a function pointer at offset 136, calculates new values for offsets 68 and 70, calls another routine, and then conditionally calls two more routines based on non-zero values at offsets 106 and 134.
- `update_object_state`: Compares two 16-bit values at offsets 24 and 26 of a structure in a6, and if their difference is greater than 2, updates offsets 26 and 28 with the value from offset 24, bitwise inverts the value at offset 20, and sets offset 32 and 36 to specific constants while updating a global variable and a function pointer at offset 136.
- `add_bcd_digit_pairs`: Adds four pairs of bytes into decimal-coded counter bytes, applying a decimal adjustment after each addition.
- `find_empty_list_slot`: Searches a list for a null entry and replaces it with a new structure containing a pointer and a constant.
- `call_tile_renderer`: Sets d4 to 0 and d3 to 7, then calls another routine.
- `update_object_graphics`: Sets d4 to 0 and d3 to 1, clears d2, and calls another routine.
- `draw_masked_tiles_to_planes`: Draws eight rows of tile data into four bitplanes, clearing masked bits and then OR-ing in the tile pattern for each plane.
- `setup_blitter_copy_rows`: Waits for the blitter to be free, then programs its control, modulo, first and last word mask, pointer and size registers for each pair of rows, decrementing a count until done.
- `set_global_pointer`: Sets three global variables to specific routine addresses and writes constant values to memory locations relative to a register, then sets bit 1 of the CIAA_PRA register.
- `update_button_state`: Checks a hardware register for a button press, updates a global variable, and updates a table or pointer based on a bit-rotated value.
- `update_palette_and_interrupt_state`: Reads the interrupt request state, increments a frame counter, and copies eight longwords of colour data into the colour registers when a palette pointer is set.
- `init_four_audio_channels`: Sets the period and volume registers on all four audio channels and points each channel at a shared one-word sample with length one.
- `update_audio_pitch_and_volume`: Reads joystick input from the CIAA port to increment or decrement two 16-bit values stored in a structure, then applies smoothing or clamping logic to these values and updates a series of memory locations with constant values if a specific flag is zero.
- `update_blt_transform_parameters`: Calculates new scaling and positioning parameters based on a pointer-referenced structure and updates a series of registers, including destination address, bitmask, and size, intended for use with the blt hardware.
- `program_blitter_for_row_copy`: Waits for the blitter to be free, clears its shift and modulo registers, and then sets its pointers, size and a modulo value computed from two inputs.
- `program_blitter_fixed_control`: Waits for the blitter, then writes fixed control words and the first and last word masks, along with source and destination modulo values of 42.
- `clear_region_and_copy_rows`: Zeroes a large region of memory word by word, then copies rows from a table into it and records each row's start pointer in a list.
- `generate_pattern_bitmaps`: Processes a source buffer of pixel data to produce downsampled and bit-packed patterns, writing the results into two separate destination memory buffers using bit-shifting and bit-replication logic.
- `build_scaled_row_table`: Copies header values into an object, calculates a row stride based on a width rounded up to sixteen, and populates an array of entries by dividing scaled coordinates by values from a lookup table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `manage_object_state_counters`: Updates a 16-bit value and a byte at offset 22 of a structure using values from a global variable, then performs conditional increments or decrements on a byte at offset 22 based on a comparison.
- `advance_table_stepped_counter`: Decrements a counter at offset fourteen of the addressed structure and reloads it from a lookup table when it reaches zero, then adds a table entry to a byte at offset fifteen.
- `update_object_state_and_position`: Decrements counters within an object structure and, if a counter reaches zero, updates the object's position and velocity using values from a table and a secondary counter.
- `clear_hardware_and_globals`: Clears five specific bytes in memory, sets two hardware registers to specific values, and enables DMA channels.
- `modify_object_offset`: Increments a byte at offset 1 of a structure in a0 by 2, and if this value matches a byte at offset 6, it adds a byte from offset 7 to a 16-bit value from offset 4, uses the result as an index into a table pointed to by a3 to update a pointer in a1, stores the incremented byte back to offset 1, and branches to another location.
- `set_status_bits`: Sets bits 5 and 1 of a byte at the address in a0 and then branches to another location.
- `set_single_bit`: Sets bit 5 of a byte at the address in a0 and then branches to another location.
- `transfer_object_data`: Copies a byte from a pointer in a1 to offset 22 of a structure in a0, copies the next byte from a1 to d0 and then to offset 23 of the structure, adds d0 to itself, stores the result in offset 21 of the structure, sets bit 6 of the byte at offset 21, and branches to another location.
- `copy_byte_to_offset`: Copies a byte from a pointer in a1 to offset 16 of a structure in a0 and then branches to another location.
- `set_multiple_bits`: Sets bits 7 and 3 of a byte at the address in a0 and then branches to another location.
- `process_object_data_buffer`: Decrements a byte at offset 17 of a structure, and if it is non-zero, performs conditional increments or decrements to a byte at offset 20 based on bits in the first byte of the structure; otherwise, it enters a loop that iterates through a buffer to update various fields in the structure and calls another routine.
- `update_object_state_from_buffer`: Copies bytes from a source buffer into a structure and performs a series of conditional checks and arithmetic operations on the structure's fields.
- `reset_object_timer_from_input`: When a byte at offset 48 of a structure in a6 is zero, it zeroes a byte at offset 28, sets a byte at offset 49 to 2, and stores a 16-bit value formed from a byte at offset 51 and a byte from a global into a global; it then copies a byte from offset 17 to offset 18, calculates the difference between two 16-bit values and stores it at offset 2 of the structure, and sets bit 15 of the DMA control register.
- `load_sequence_entry_fields`: Looks up a byte from a table indexed by an input value and stores it in an object field together with a second table-derived byte, then sets a countdown to one.
- `update_value_from_table_record`: When a mode byte is set, it looks up a ten-byte table record by index, decrements a counter, and stores a difference and an offset in the output fields.
- `copy_table_to_memory`: Calculates an offset based on an input value, zeroes a byte at that location, and then copies a series of words from a table to a destination block.
