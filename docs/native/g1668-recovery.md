# Game 1668 (space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +2 setrox]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01668). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (910160 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 56 of 56 named (0 author symbols), 56 of 56 described (1 by rule, 34 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 949 of 949 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2085 of 2085 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 15248 |
| code | 12174 |
| data | 832050 |
| unused | 50688 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +2 Setrox].adf:bootblock | 1 | 41 | 3 |
| logged disk load from Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +2 Setrox].adf boot-block load | 1 | 163 | 2 |
| logged disk load from Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +2 Setrox].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1573 | 36 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 308 | 15 |

## Cases handled

- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +2 setrox]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +2 Setrox].adf: custom-format (non-DOS) disk, 1659 non-empty blocks: matched against track-loaded segments
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +2 setrox]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +2 Setrox].adf: the boot block reads 11264 bytes from disk offset 788480 to an address (found statically); recovered as a unit
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +2 setrox]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +2 Setrox].adf: the boot block reads 1536 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 725 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 224 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +2 setrox]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +2 Setrox].adf: 2 logged loads cover 12800 bytes of the image
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +2 setrox]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +2 Setrox].adf: 16 track-loaded blocks found in recovered runtime segments and classified through them
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +2 setrox]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +2 Setrox].adf: blocks 11-314 (bytes 5632-161280) outside the filesystem hold data, not code (opcode density 0.007, entropy 7.12)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +2 setrox]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +2 Setrox].adf: blocks 329-358 (bytes 168448-183808) outside the filesystem hold data, not code (opcode density 0.007, entropy 5.13)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +2 setrox]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +2 Setrox].adf: blocks 397-407 (bytes 203264-208896) outside the filesystem hold data, not code (opcode density 0.013, entropy 6.56)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +2 setrox]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +2 Setrox].adf: blocks 409-418 (bytes 209408-214528) outside the filesystem hold data, not code (opcode density 0.015, entropy 6.66)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +2 setrox]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +2 Setrox].adf: blocks 420-492 (bytes 215040-252416) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.68)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +2 setrox]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +2 Setrox].adf: blocks 504-753 (bytes 258048-386048) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.31)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +2 setrox]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +2 Setrox].adf: blocks 770-945 (bytes 394240-484352) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.48)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +2 setrox]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +2 Setrox].adf: blocks 968-1172 (bytes 495616-600576) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.83)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +2 setrox]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +2 Setrox].adf: blocks 1174-1539 (bytes 601088-788480) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.86)
- space harrier - return to the fantasy zone (1989)(elite)[cr power connection][t +2 setrox]/Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +2 Setrox].adf: blocks 1650-1759 (bytes 844800-901120) hold ASCII text
- 1 packed track region(s), 42496 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `execute_timed_loop` (runtime segment an address): Calculates a loop count based on a global value and executes a sequence of routine calls while modifying a global variable. Kind: loop-count/timing-based check. Pass path: sVar1 == -1 (loop completion without deviation).

## Functions


### Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +2 Setrox].adf:bootblock

- `start_boot_block_program`: The boot-block entry point, called by the Kickstart strap.
- `initialize_vector_table`: Writes the trap vector table start address, the trap return instruction, and the exception vector table start address into the corresponding vector slots, then returns.
- `write_clr_patch_instructions`: Stores a global word into one structure field and zeroes another, then writes two machine-code words that clear memory words at fixed offsets.

### logged disk load from Space Harrier - Return to the Fantasy Zone (1989)(Elite)[cr Power Connection][t +2 Setrox].adf boot-block load

- `pack_byte_and_shift_longword`: Reads the longword just before its input pointer, stores its low byte combined with a high byte into a memory word, and returns that longword shifted right by one with the top bit set.
- `shift_bits_with_refill`: Shifts a bit register right one position per pass and calls an unnamed helper to reload it whenever it empties.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_up_blitter_operation`: Sets the blitter control, modulo, pointer, first-word mask and size registers, and the second copper list pointer for one blitter operation.
- `run_conditional_object_updates`: Calls four routines in order, then calls further routines depending on signed fields of an object structure and a threshold comparison on a second field.
- `traverse_glyph_list`: Recursively traverses a list of glyphs and calls a routine to draw each glyph into the bitplanes using the blitter.
- `shift_memory_block`: Decrements a counter while checking a memory range, then shifts a block of memory backwards by copying or overwriting values.
- `store_register_to_memory`: Stores a 32-bit register value to a memory location and clears three bytes at a different memory location.
- `call_object_handler_pointer`: Calls the routine whose pointer is stored at a fixed offset in an object structure.
- `update_object_properties`: Modifies certain fields in an object structure, updates a function pointer, and conditionally updates a global variable.
- `update_object_state`: Modifies certain values in an object structure and updates a global variable based on a conditional check.
- `add_bcd_digit_pairs`: Adds four pairs of bytes into decimal-coded counter bytes, applying a decimal adjustment after each addition.
- `draw_masked_tiles`: Invokes a routine that draws masked tile data into the bitplanes.
- `update_object_graphics`: Updates the visual state of an object by invoking a routine that renders masked tile data into the bitplanes.
- `draw_masked_tiles_to_planes`: Draws eight rows of tile data into four bitplanes, clearing masked bits and then OR-ing in the tile pattern for each plane.
- `setup_blitter_copy_rows`: Waits for the blitter to be free, then programs its control, modulo, first and last word mask, pointer and size registers for each pair of rows, decrementing a count until done.
- `update_palette_and_interrupt_state`: Reads the interrupt request state, bumps a frame counter, and copies eight longwords of colour data into the colour registers when the palette pointer is set.
- `init_four_audio_channels`: Sets the period and volume registers on all four audio channels and points each channel at a shared one-word sample with length one.
- `poll_fire_button_and_update`: A 258-instruction routine that works on pointers and a stored value and reads the fire-button port.
- `update_line_scaling_parameters`: Calculates and stores new scaling-related parameters, including a source pointer, width, and bitmask, into a set of global variables.
- `calculate_scaled_line_parameters`: Calculates a new source pointer, width, and blitter-compatible offset or mask based on a scaling factor and a target or source coordinate.
- `clip_and_blit_scaled_rows`: Clips a row offset against a limit of 248 and returns when nothing is left, then programs the blitter control, modulo, pointer and size registers.
- `program_blitter_for_row_copy`: Waits for the blitter to be free, clears its shift and modulo registers, and then sets its pointers, size and a modulo value computed from two inputs.
- `program_blitter_fixed_control`: Waits for the blitter, then writes fixed control words and the first and last word masks, along with source and destination modulo values of 42.
- `execute_timed_loop`: Calculates a loop count based on a global value and executes a sequence of routine calls while modifying a global variable.
- `execute_masked_write`: Executes a sequence of routine calls while modifying a global variable and then writes a masked value to a memory location.
- `update_global_and_clear_buffer`: Checks a global variable and a pointer, then conditionally flips the global variable and clears a certain range of a buffer or table.
- `wait_for_bit_clear`: Polls a bit in a memory location until that bit is cleared.
- `set_hardware_and_loop`: Writes several values to a structure and loops while a certain bit in a global variable is not set, after waiting for a bit in a memory location to clear.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `set_hardware_and_loop_on_flag`: Writes several values to a structure, then loops while a certain bit in a global variable is not set, finally writing more values to a structure.
- `update_global_and_bitmask`: Checks a global variable and a pointer, then conditionally modifies a bit in a buffer and recursively calls itself based on the difference between a global and a shifted version of itself.
- `reset_global_and_loop`: Sets a bit in a buffer and loops while a certain bit in a pointer-referenced byte is set, finally clearing a global variable.
- `search_and_interleave_bits`: Searches for a specific word in a table, then performs a bit-interleaving operation and writes the result to a destination buffer.
- `draw_glyph_into_bitplanes`: Maps a character to a glyph index, using separate ranges for digits and letters.
- `call_routine_for_each_char`: Loops over a null-terminated string, calling one unnamed routine for each character, and returns the input register unchanged.
- `call_helper_pass_through`: Calls one unnamed routine and then returns the input register unchanged.
- `call_helper_twice`: Calls one unnamed routine twice in a loop and then returns.
- `emit_nibbles_skipping_leading_zeros`: Walks the eight nibbles of a 32-bit value from the top, rotating one nibble per pass.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_audio_dma_and_interrupts`: Updates several status bytes using a mask of 1920, which selects the audio interrupt bits, and adds a value to a stored byte with carry.
- `decrement_timer_step_value`: Decrements a countdown byte in an object; when it reaches zero it reloads it from a table, and a zero table entry stops the update.
- `process_byte_block_with_helpers`: A long routine that works on a block of bytes through several pointers and calls three unnamed routines.
- `reset_state_and_control_words`: Clears five state words in program data, then writes two control values to registers listed in the facts as the disk-and-audio control register and the DMA control register.
- `update_object_from_stream`: Reads a value from a pointer, then uses it to look up and write values into a structure or object field.
- `expand_and_update_object`: Sets certain flags in a structure and then processes a stream of bytes to update object fields or pointers.
- `process_stream_with_flags`: Sets a bit in a structure and then iterates through a byte stream to update object properties or call another routine.
- `initialize_and_expand_object`: Writes certain bytes to a structure and then processes a byte stream to update object fields or call another routine.
- `populate_object_from_stream`: Writes a byte from a pointer to a structure and then processes a byte stream to update object fields or call another routine.
- `set_flags_and_expand_object`: Sets certain flags in a structure and then processes a byte stream to update object fields or call another routine.
- `advance_counted_byte_copy`: Decrements a count byte in a structure and, while it stays non-zero, copies bytes between pointers, calling one unnamed routine.
- `initialize_object_from_table`: Loads object properties from a source pointer and updates an object's internal state or timer.
- `reset_object_timer_from_input`: When an object's mode byte is clear, it zeroes a counter, sets the mode value, and stores a byte in a global.
- `lookup_table_into_object_fields`: Looks up a byte in a table indexed by a 16-bit value and stores it, along with a second table byte, into two fields of an object.
- `update_value_from_table_record`: When a mode byte is set, it looks up a ten-byte table record by index, decrements a counter, and stores a difference and an offset in the output fields.
