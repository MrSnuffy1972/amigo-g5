# Game 1477 (castle master (1990)(domark)(m3)[cr][m acu]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01477). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (910832 top-level bytes in 12 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 69 of 69 named (0 author symbols), 69 of 69 described (4 by rule, 35 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 593 of 593 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 472862 bytes, 22 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2816 of 2842 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 158215 |
| code | 10684 |
| data | 526844 |
| padding | 2097 |
| unused | 212992 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:dawn4 | 1 | 99 | 3 |
| Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:cm | 1 | 116 | 2 |
| Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1331 | 39 |
| Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:x (raw code file, load address unknown) | 1 | 2 | 1 |
| Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:intro (raw code file, load address unknown) | 1 | 1253 | 21 |

## Cases handled

- runtime segment an address: 19 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 392 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- castle master (1990)(domark)(m3)[cr][m acu]/Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:x: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- castle master (1990)(domark)(m3)[cr][m acu]/Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:intro: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
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
- `tiny_moveq`: Small routine of 1 instruction that loads 0 into d4.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `clear_data_register_zero`: Sets the least significant byte of data register zero to zero.
- `decode_relocation_table_and_jump`: The routine calls a library function through an offset from a base pointer, stores several pointers into fixed globals, and then walks a byte-encoded table: each non-zero byte advances a pointer by that many units and writes a base-relative address into the slot, with a value of one extending the step, and a zero byte ends the walk.
- `apply_relocation_table_then_jump`: Walks a byte-coded relocation table, adding a base offset to each marked longword pointer in the loaded segment, and then runs a fixed-count countdown delay before calling through the stored entry pointer.
- `tiny_ori_2`: Small routine of 1 instruction that merges 69 into d0.
- `relocate_function_table`: Iterates through a table of function pointers and updates them with relative offsets.
- `relocate_function_pointers`: Iterates through a table of function pointers and updates them with relative offsets based on a base address.
- `process_jump_table`: Iterates through a data structure to perform indirect jumps or calls based on table entries.
- `perform_delay_loop`: Decrements a value from a large constant in a loop until it reaches zero.
- `clear_memory_block`: Clears a block of memory by writing zero to a series of words and then updates a global variable with the value of another global variable.
- `transfer_keyboard_register`: Transfers the contents of a processor register to a memory location using a pointer and accesses the keyboard serial register.
- `transfer_register_to_memory`: This routine consists of three instructions and transfers the contents of a processor register into a memory location specified by a pointer.
- `process_string_data`: Iterates through a source string and copies characters to a destination buffer while performing bitwise operations and checking for a specific character.
- `clear_memory_range`: Fills a range of 111 words starting at a specific memory address with zeros.
- `load_global_value`: Stores the value of a global variable into another global variable.
- `update_audio_parameters`: Checks a signature in memory and, if matched, performs complex data manipulation involving multiple tables and hardware audio volume registers.
- `initialize_global_values`: Sets a global variable to zero multiple times before assigning it a final value of fifteen.
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
- `execute_looping_call`: Executes another routine four times within a decrementing loop.
- `draw_shifted_masked_planes_wide`: Draws a bit-shifted, masked block of four consecutive source words per group into four destination planes, with a separate case for the final partial word.
- `wrap_position_and_read_words`: Adds an offset to a 16-bit position and wraps it when it passes 318, repeating for a second offset.
- `nudge_palette_components_toward_source`: Compares the 4-bit red, green and blue components of sixteen words with those of a second table, nudging a component one step toward it only when the two differ by fifteen.
- `spin_in_infinite_loop`: The decompiled body is an empty infinite loop with no calls and no hardware access, and its 40 instructions are not all visible.
- `clear_ten_longs_across_planes`: Zeroes ten consecutive longwords and matching slots in four plane-sized regions on each pass, advancing ten longwords per pass over a fixed run of passes.
