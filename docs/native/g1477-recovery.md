# Game 1477 (castle master (1990)(domark)(m3)[cr][m acu]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01477). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (910832 top-level bytes in 13 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 105 of 105 named (0 author symbols), 105 of 105 described (20 by rule, 52 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 601 of 601 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 526110 bytes, 40 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4125 of 4131 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 157753 |
| code | 11602 |
| data | 526388 |
| padding | 2097 |
| unused | 212992 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:dawn4 | 1 | 99 | 3 |
| Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:cm | 1 | 116 | 2 |
| Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1579 | 57 |
| Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:dawn4#unpacked | 1 | 1041 | 18 |
| Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:x (raw code file, load address unknown) | 1 | 2 | 1 |
| Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:intro (raw code file, load address unknown) | 1 | 1253 | 21 |

## Cases handled

- runtime segment an address: 19 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 392 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- castle master (1990)(domark)(m3)[cr][m acu]/Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:dawn4: crunched executable unpacked with ancient scan (TetraPack 2.2 (xfd), 10568 -> 53248 bytes), 9 of 9 windows confirmed in reference RAM, loaded at an address: recovered as u5
- castle master (1990)(domark)(m3)[cr][m acu]/Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:x: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- castle master (1990)(domark)(m3)[cr][m acu]/Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:intro: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 immediate+address
- 2 never-loaded region(s), 472862 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:intro (raw code file, load address unknown)

## Copy protection found

- none found

## Functions


### Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:dawn4

- `start_program_set_colour01`: This is the program entry point.
- `store_word_and_shift_stream`: Stores a 16-bit word built from the low byte of a longword and the high byte of a register, then returns that longword shifted right by one with its top bit forced on.
- `shift_bits_refilling_when_empty`: Shifts a longword right one bit per iteration over a count taken from a register, and reloads it through a call whenever it becomes zero.

### Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:cm

- `load_intro_file_via_dos`: Opens dos.library, allocates memory with exec.AllocMem, and opens a file named intro through the DOS Open, Read and Close calls.
- `print_null_terminated_string`: Measures a null-terminated string and passes its length through the library entry at offset -60.

### Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_library_read_mouse_buttons`: Opens a library with exec.OldOpenLibrary and reads the POTGOR register, which holds the right and middle mouse buttons.
- `write_string_to_output`: Calls the library entry at offset -60, scans a string to its null terminator, and then calls the entry at offset -48 to write it.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_unnamed_routine`: This routine consists of three instructions and calls another routine.
- `frag_moveq_bra`: Short code fragment that loads 0 into d0, then loads 0 into d1, then jumps to another point in the code.
- `relocate_function_pointers`: Iterates through a table of function pointers and updates them with relative offsets based on a base address.
- `process_jump_table`: Iterates through a data structure to perform indirect jumps or calls based on table entries.
- `perform_delay_loop`: Decrements a value from a large constant in a loop until it reaches zero.
- `clear_memory_block`: Clears a block of memory by writing zero to a series of words and then updates a global variable with the value of another global variable.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (movea.w) and then falls through into the code that follows, with no return of its own.
- `process_string_data`: Iterates through a source string and copies characters to a destination buffer while performing bitwise operations and checking for a specific character.
- `clear_memory_range`: Fills a range of 111 words starting at a specific memory address with zeros.
- `load_global_value`: Stores the value of a global variable into another global variable.
- `update_audio_parameters`: Checks a signature in memory and, if matched, performs complex data manipulation involving multiple tables and hardware audio volume registers.
- `initialize_global_values`: Sets a global variable to zero multiple times before assigning it a final value of fifteen.
- `frag_addq_cmpi_bne_clr_bra`: Short code fragment that adds 1 to a fixed memory address, then compares a fixed memory address with 6, then branches if the last result was not zero, then clears a fixed memory address, then jumps to another point in the code.
- `setup_game_object`: Sets up game object properties including position, graphics data, and audio parameters using values from predefined tables, then configures the object's rendering and audio characteristics.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `process_state_byte`: Reads a byte from offset 2 of a structure, masks it with 15, tests the result, and branches to different locations based on whether the value equals 0, 1, 2, or 10.
- `subtract_byte_clamp_copy`: Subtracts the byte at offset 3 of the structure in a6 from the word at offset 22 of that structure, forces the result to a minimum of 113 if it falls below, and copies it to offset 6 of the structure in a5.
- `slide_channel_value_up`: Adds a parameter byte to a running value in a structure, clamps the result to an upper limit of one thousand three hundred and thirty-six, and copies the result to an output field.
- `update_movement_counter`: Reads a byte from memory offset 3 of a structure, shifts it right by 4 bits, and adds the result to a word value at offset 18 of the same structure.
- `frag_moveq_move_andi_sub_bpl_clr_bsr_rt`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then subtracts d0 from the memory at an offset from a6, then branches if the last result was not negative, then clears the memory at an offset from a6, then copies the memory at an offset from a6 to d0, then calls another routine, then copies d0 to the memory at an offset from a5, then returns.
- `dispatch_by_global_value`: Loads a word value from a fixed memory location, compares it against constants 1 through 5, and branches to different locations based on the comparison results.
- `frag_moveq_move_lsr_bra`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then shifts d0 right by 4, then jumps to another point in the code.
- `frag_moveq_move_andi_bra`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_move_bra`: Short code fragment that copies the memory at an offset from a6 to d2, then jumps to another point in the code.
- `frag_add_moveq_move_andi_lea_cmp_beq_ad`: Short code fragment that adds d0 to d0, then loads 0 into d1, then copies the memory at an offset from a6 to d1, then masks d1 with 4095, then loads the address of a location in the code into a0, then copies an indexed entry from a0 to d2, then compares d1 with the memory at a0, then branches if the last result was zero, then adds 2 to a0, then jumps to another point in the code.
- `frag_move_rts`: Short code fragment that copies d2 to the memory at an offset from a5, then returns.
- `initialize_sequence_2`: Writes a series of longwords and words to memory locations starting at a global variable, updating values in a loop with incrementing counters and managing sequence progression.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `process_animation_data`: Reads animation parameters from a structure, updates position values based on multiplication results, and writes updated values to multiple memory locations.
- `process_dma_and_conditions`: Performs several memory operations including adding values to a structure, copying data to various locations, and setting up DMA channels.
- `frag_not_moveq_move_rts`: Short code fragment that inverts a fixed memory address, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then copies d0 to a fixed memory address, then returns.
- `frag_move_ext_bsr_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then sign-extends d0, then calls another routine, then copies d0 to the memory at an offset from a5, then returns.
- `frag_not_rts`: Short code fragment that inverts a fixed memory address, then returns.
- `frag_moveq_move_andi_rol_or_rts`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 1, then rotates d0 left by 1, then masks a fixed memory address with 253, then merges d0 into a fixed memory address, then returns.
- `reset_counter_if_nonzero`: If the low nibble of a parameter byte is not zero, it clears a global word and stores that nibble into another global variable.
- `calculate_fixed_point_multiplication`: Performs a multiplication of a short integer by a global value and returns the result as a 32-bit value split across two registers.
- `initialize_audio_and_coprocessor`: Sets up audio system by opening libraries, configuring coprocessor pointers, and initializing memory blocks.
- `fill_multiple_buffers`: Clears various offsets within a memory block by writing zero to them in a loop.
- `write_shifted_values_to_buffers`: Reads values from a table, shifts them based on a global value, and writes them to multiple destination buffers.
- `copy_shifted_longs_to_planes_2`: Reads four source longwords per step from a table, shifts them left by the low four bits of a position value, and writes each one to a different plane buffer.
- `copy_table_to_buffers`: Copies data from a source table into multiple destination buffers using nested loops.
- `fill_buffers_from_table`: Reads values from a table and writes them into multiple destination buffers in a repeating pattern.
- `blend_table_to_buffers`: Performs bitwise logical operations between a source table and a destination buffer before clearing the remaining buffer space.
- `validate_global_range`: Performs multiple conditional checks on a global variable to determine if it falls within specific numeric ranges.
- `clamp_global_value`: Checks a global variable against specific bounds and performs conditional updates or calls to other routines.
- `draw_clipped_masked_block_2`: Draws a bit-shifted block from four source planes into four destination planes, limiting the column count by the distance to a 320-pixel edge and keeping destination pixels wherever all source planes are zero.
- `draw_wide_clipped_masked_block_2`: Draws a bit-shifted, masked four-plane block, limiting the column count to eight and reading its source planes from a wider offset.
- `draw_masked_planes`: Draws bit-shifted data into four destination planes using a mask, with the specific plane update logic determined by a global variable.
- `draw_shifted_row_with_spill_2`: Draws up to eight source rows into four planes, each shifted by a value derived from an x position with spilled bits carried into the following word.
- `clear_destination_area`: Iterates through a source table to perform bitwise masking and OR operations on a destination buffer to clear specific pixel patterns.
- `clear_destination_buffer`: Clears a destination buffer by applying bitwise operations from a source table to the memory pointed to by a global variable.
- `execute_repeated_routine`: Calls another routine four times in a loop.
- `blend_nibble_data_to_buffers`: Iterates through a source table of nibble values to perform bitwise masking and logical OR operations on multiple destination buffers.
- `resolve_indirect_pointer`: Traverses multiple levels of indirection through a pointer structure to retrieve a short integer value.
- `interpolate_nibble_values`: Iterates through two tables of 16-bit words to incrementally adjust three nibbles per word toward target values.
- `execute_infinite_loop`: This routine is empty and consists only of an infinite loop.
- `clear_large_buffer_set`: Iterates through a large memory area, writing zero to many specific offsets and indices within a loop.

### Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:dawn4#unpacked

- `frag_lea_movea_jsr_move_bsr_btst_bne_br`: Short code fragment that loads the address of a location in the code into a1, then copies a fixed memory address to a6, then calls another routine, then copies d0 to a fixed memory address, then loads the address of a location in the code into a0, then calls another routine, then tests bit 10 of a fixed memory address, then branches if the last result was not zero, then jumps to another point in the code.
- `handle_file_operations`: Handles file operations by writing data to output streams using DOS library calls.
- `process_input_sequence`: Processes input bytes from a buffer and updates various game state variables based on the byte values.
- `initialize_render_data`: Sets up rendering parameters for multiple data entries using a global variable as a base, updating offsets in a structure referenced through a6 with values derived from shifts and additions.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `update_audio_channels`: Updates four audio channel parameters by decrementing countdown timers and reading values from tables into memory registers.
- `initialize_sequence`: Writes a series of longwords and words to memory locations starting at a global variable, updating values in a loop with incrementing counters and conditional data loading from another table.
- `frag_lea_move_add_addq_dbra_rts`: Short code fragment that loads the address of a location in the code into a0, then copies an address to d0, then copies an address to d1, then copies 31 to d2, then copies d1 to the memory at a0, then adds d0 to d1, then adds 8 to a0, then loops back while counting down d2, then returns.
- `manage_audio_channel_operations`: Manages audio channel operations by checking channel status flags, updating channel parameters, and writing values to audio registers.
- `configure_audio_parameters`: Configures audio parameters for multiple channels by setting up channel addresses, updating length and period values, and managing playback state through DMA control registers.
- `fill_data_structure`: Populates a data structure with repeated sequences of longwords and words, using nested loops to write fixed values and read from a source table while incrementing pointers.
- `setup_context`: Processes input word values through bit rotation and XOR operations before storing them at specific offsets within a structure pointed to by a6 register, along with other input values.
- `frag_move_clr_rts`: Short code fragment that copies 2304 to the memory at an offset from a6, then clears the memory at an offset from a6 (3 times in a row), then copies d1 to the memory at an offset from a6, then copies 33856 to the memory at an offset from a6, then copies a1 to the memory at an offset from a6, then copies d2 to the memory at an offset from a6, then copies 1024 to the memory at an offset from a6, then returns.
- `disable_audio_playback`: Clears the audio playing flag and zeroes four audio channel period registers.
- `initialize_game_object`: Sets up a game object's properties including position, graphics data, and audio parameters using values from predefined tables, then configures DMA and interrupt settings.
- `initialize_render_data_2`: Initializes rendering data structures by setting up parameters for multiple data entries using a global variable as a base, updating offsets in referenced structures.
- `update_channel_sequence`: Reads the next event byte from a per-channel pattern, moves the pattern pointer forward when a block ends, and updates the channel's volume and playback fields from the bytes that follow.
- `parse_command_stream`: Reads a byte stream to update internal state, manipulate a global bitmask, and control pointers based on encoded commands.

### Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:x (raw code file, load address unknown)

- `stub_two_instruction_entry`: A two-instruction stub at the start of a raw code region, with no calls, callers, registers or text in the listed facts.

### Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:intro (raw code file, load address unknown)

- `open_libraries_init_copper`: Opens a system library through exec.OpenLibrary and then calls library entries at offsets -30, -36 and -42.
- `clear_long_slots_across_planes`: Zeroes pairs of longword slots spread across four plane-sized regions over a fixed run of passes.
- `blit_shifted_source_to_planes`: Copies rows of a source table into four plane buffers, shifting each word left by the low four bits of a position value.
- `copy_shifted_longs_to_planes`: Reads four source longwords per step from a table, shifts them left by the low four bits of a position value, and writes each one to a different plane buffer.
- `copy_four_plane_block`: Copies a twenty-word-wide block from a source whose four sub-blocks sit twenty words apart into four plane buffers spaced 8000 bytes apart.
- `fill_run_length_plane_rows`: Reads records of a row count and three words from a table, then writes the three words across twenty columns of planes 0 to 2 and zeroes plane 3 for count plus one rows.
- `merge_masked_planes_and_clear`: Merges a twenty-column block from four source planes into four destination planes, keeping destination pixels wherever all four source planes are zero.
- `dispatch_on_phase_counter`: Calls unnamed routines depending on a signed state value tested against fixed ranges, and resets a phase counter to zero when it equals three.
- `advance_phase_counter_once`: Resets a signed counter to zero when it is below minus twelve, otherwise calls an unnamed routine and, when the counter equals three, resets it and calls the routine again.
- `draw_clipped_masked_block`: Draws a bit-shifted block from four source planes into four destination planes, limiting the column count by the distance to a 320-pixel edge and keeping destination pixels wherever all source planes are zero.
- `draw_wide_clipped_masked_block`: Draws a bit-shifted, masked four-plane block like the narrower variant, but limits the column count to eight and reads its source planes further apart.
- `draw_mode_selected_plane_row`: Draws one source word per row into destination planes after a sub-word shift, clamping the column count to two and adding an extra plane write when the mode value equals two.
- `draw_shifted_row_with_spill`: Draws up to eight source rows into four planes, each shifted by the low four bits of an x position with spilled bits carried into the following word.
- `merge_sparse_table_at_offset`: Walks a fixed source table four words at a time, skipping all-zero groups and merging the others into four destination planes at an offset while keeping background pixels where the source is zero.
- `merge_sparse_table_at_base`: Walks a second fixed source table four words at a time, skipping all-zero groups and merging the others into the base of the plane area.
- `repeat_call_four_times`: Loops four times, calling another routine on each iteration and decrementing a counter, then returns.
- `draw_shifted_masked_planes_wide`: Draws a bit-shifted, masked block of four consecutive source words per group into four destination planes, with a separate case for the final partial word.
- `wrap_position_and_read_words`: Adds an offset to a 16-bit position and wraps it when it passes 318, repeating for a second offset.
- `nudge_palette_components_toward_source`: Compares the 4-bit red, green and blue components of sixteen words with those of a second table, nudging a component one step toward it only when the two differ by fifteen.
- `spin_in_infinite_loop`: The decompiled body is an empty infinite loop with no calls and no hardware access, and its 40 instructions are not all visible.
- `clear_ten_longs_across_planes`: Zeroes ten consecutive longwords and matching slots in four plane-sized regions on each pass, advancing ten longwords per pass over a fixed run of passes.
