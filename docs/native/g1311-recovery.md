# Game 1311 (castle master (1990)(domark)(m3)[cr mad][m tfs]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01311). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (915760 top-level bytes in 17 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 106 of 106 named (0 author symbols), 106 of 106 described (7 by rule, 69 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2399 of 2399 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 472862 bytes, 22 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4351 of 4377 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 154339 |
| code | 16710 |
| data | 545942 |
| padding | 2161 |
| unused | 196608 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Castle Master (1990)(Domark)(M3)[cr MAD][m TFS].adf:tfs | 6 | 231 | 6 |
| Castle Master (1990)(Domark)(M3)[cr MAD][m TFS].adf:M.A.D | 6 | 29 | 2 |
| Castle Master (1990)(Domark)(M3)[cr MAD][m TFS].adf:cm | 1 | 116 | 2 |
| Castle Master (1990)(Domark)(M3)[cr MAD][m TFS].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 231 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 241 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 47 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 536 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1667 | 41 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| Castle Master (1990)(Domark)(M3)[cr MAD][m TFS].adf:x (raw code file, load address unknown) | 1 | 2 | 1 |
| Castle Master (1990)(Domark)(M3)[cr MAD][m TFS].adf:intro (raw code file, load address unknown) | 1 | 1255 | 21 |

## Cases handled

- castle master (1990)(domark)(m3)[cr mad][m tfs]/Castle Master (1990)(Domark)(M3)[cr MAD][m TFS].adf: file P1.POS (header 2958 bytes, blocks hold 1464): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address: 180 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 156 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 47 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 363 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1089 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 10 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- castle master (1990)(domark)(m3)[cr mad][m tfs]/Castle Master (1990)(Domark)(M3)[cr MAD][m TFS].adf:x: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- castle master (1990)(domark)(m3)[cr mad][m tfs]/Castle Master (1990)(Domark)(M3)[cr MAD][m TFS].adf:intro: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- castle master (1990)(domark)(m3)[cr mad][m tfs]/Castle Master (1990)(Domark)(M3)[cr MAD][m TFS].adf: blocks 11-21 (bytes 5632-11264) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.77)
- castle master (1990)(domark)(m3)[cr mad][m tfs]/Castle Master (1990)(Domark)(M3)[cr MAD][m TFS].adf: blocks 88-89 (bytes 45056-46080) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.99)
- castle master (1990)(domark)(m3)[cr mad][m tfs]/Castle Master (1990)(Domark)(M3)[cr MAD][m TFS].adf: blocks 95-98 (bytes 48640-50688) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.82)
- castle master (1990)(domark)(m3)[cr mad][m tfs]/Castle Master (1990)(Domark)(M3)[cr MAD][m TFS].adf: blocks 1348-1348 (bytes 690176-690688) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.22)
- 2 never-loaded region(s), 472862 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- Castle Master (1990)(Domark)(M3)[cr MAD][m TFS].adf:intro (raw code file, load address unknown)

## Copy protection found

- none found

## Functions


### Castle Master (1990)(Domark)(M3)[cr MAD][m TFS].adf:tfs

- `free_memory_blocks_loop`: Program entry point that calls one unnamed routine, then loops several times through a second unnamed routine and the exec FreeMem entry at offset -210.
- `decrement_until_minus_one`: Counts a register-held 16-bit value down until it reaches minus one, doing nothing else.
- `accumulate_values_in_two_passes`: Runs two passes that read counts and values from a stream and add each value into a destination table at computed offsets.
- `merge_zero_into_d0`: performs a single instruction to merge a zero value into the d0 register.
- `copy_bytes_backward_loop`: Copies bytes backward from a source pointer into a destination while stepping a counter, branching on a flag and a sign test and reloading source positions on wrap.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### Castle Master (1990)(Domark)(M3)[cr MAD][m TFS].adf:M.A.D

- `execute_init_and_free`: Calls an initial routine and a loop that executes a function pointer from a global variable, then clears a value at an offset from a pointer and returns a concatenated register value.
- `decrement_until_minus_one_2`: Decrements a register-held 16-bit value until it reaches negative one.

### Castle Master (1990)(Domark)(M3)[cr MAD][m TFS].adf:cm

- `load_intro_file_via_dos`: Opens dos.library, allocates memory with exec.AllocMem, and opens a file named intro through the DOS Open, Read and Close calls.
- `print_null_terminated_string`: Measures a null-terminated string and passes its length through the library entry at offset -60.

### Castle Master (1990)(Domark)(M3)[cr MAD][m TFS].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_state`: Performs a series of assignments to a global variable and enters an infinite loop if a specific bitwise condition is met.
- `setup_hardware_registers`: Clears a memory range, copies data from a table, and configures various hardware registers including DMA and interrupt settings.
- `populate_data_table`: Iterates through a source table to calculate and store specific values into a destination table at fixed offsets.
- `increment_and_copy_list`: Increments a global counter and copies a sequence of short integers from a source list to a destination list.
- `search_and_configure_blt`: Searches for a specific byte in a table and, if found, configures Blitter registers with calculated values.
- `calculate_address_offset`: Performs arithmetic operations to calculate a value and stores it in a global variable.
- `copy_short_values`: Copies a series of short integers from a source pointer to two different destination locations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `trigger_software_interrupt`: Stores a routine address into a global variable and executes a software interrupt.
- `process_dynamic_data`: Calls a function pointer stored in a global variable, modifies a table of integers, and executes several other routines based on a loop controlled by a global variable.
- `fill_memory_with_pattern`: Fills a memory range with a repeating pattern of alternating long integer values.
- `copy_memory_with_wrap`: Increments a pointer with a wrap-around check and copies a block of data from that pointer to another location.
- `process_object_table`: Iterates through a table of data to calculate offsets and store properties into a structure.
- `toggle_and_load_parameters`: Toggles a bit in a global variable and uses the result to load two different sets of values into a structure.
- `update_table_entries`: Iterates through a table of pointers, performing arithmetic on values stored at offsets within each entry.
- `generate_code_data`: Uses nested loops to write specific byte patterns and values into a memory location based on a source table.
- `fill_memory_with_ones`: Iterates through a list of pointers and sets the values at those locations to all ones until a specific terminator is reached.
- `decrement_global_counter`: Decrements a global variable and resets another global variable to zero when the counter reaches zero.
- `pop_data_from_list`: Retrieves a pointer and a value from a list stored in memory, advancing the list pointer accordingly.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_nested_calls`: Repeatedly calls another routine within a loop until the returned value is zero.
- `process_bitmask_graphics`: Processes bitmask data through multiple nested loops to perform bitwise operations and update memory buffers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_update_global_from_table`: Calls a routine that updates two global variables from a table based on a register value or sets a global flag if the register is zero.
- `return_concatenated_registers`: Returns a value formed by concatenating the contents of two registers.
- `update_global_from_table`: If a register is non-zero, calculates an offset into a table to load a value into two global variables; otherwise, sets a global flag.
- `calculate_table_offset`: If a global flag is not set, it sets the flag and performs multiple arithmetic calculations using several global variables to derive a final value.
- `parse_event_stream`: Parses a data stream using multiple subroutines and updates several global variables and counters based on the results.
- `parse_bitstream_data`: Reads a stream of data from a buffer, interpreting bytes as control headers and bit-packed values.
- `parse_scripted_event_data`: Parses a custom bytecode stream to update game state, including setting pointers, loading data from a table, and modifying bitmasks.
- `configure_dma_and_interrupts`: Configures the DMA controller and interrupt enable or disable settings via the custom chip registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_global_state_via_function_pointers`: Updates several global variables and executes a sequence of indirect function calls retrieved from a table.
- `tiny_moveq`: Small routine of 1 instruction that loads 0 into d4.
- `clear_data_byte`: Sets a byte in a data register to zero.
- `relocate_jump_table`: Loads a function pointer from a structure and performs a relocation process on a table of relative offsets.
- `apply_relocation_table_then_jump`: Walks a byte-encoded relocation table, adding a base offset to each marked longword in the image, and then busy-waits a fixed count before calling the entry point stored in the routine's global pointer.
- `apply_relocation_table_and_jump`: Opens a library through a base-pointer indirection, then walks a byte-encoded relocation table, patching each entry by adding the base offset to the stored displacement before stepping to the next record.
- `tiny_ori_2`: Small routine of 1 instruction that merges 69 into d0.
- `relocate_jump_table_2`: Performs a relocation process on a table of relative offsets to convert them into absolute function pointers.
- `clear_memory_block`: Clears a block of memory by writing zero to a series of words and then updates a global variable with the value of another global variable.
- `transfer_register_to_memory_2`: A routine consisting of 22 instructions that transfers the contents of a processor register to a memory location.
- `transfer_register_to_memory`: The routine moves the contents of a processor register into a memory location specified by another register.
- `process_string_data`: Iterates through a source string and copies characters to a destination buffer while performing bitwise operations and checking for a specific character.
- `clear_memory_range`: Fills a range of 111 words starting at a specific memory address with zeros.
- `copy_global_value`: Stores a value from a global variable into another global variable.
- `update_audio_parameters`: Checks a signature in memory and, if matched, performs complex data manipulation involving multiple tables and hardware audio volume registers.
- `reset_globals_to_constant`: Writes zero to five global variables and writes a constant value to a fifth global variable.
- `update_audio_channel_parameters`: Modifies audio hardware registers by updating channel volume, frequency, or pitch values based on command identifiers and internal state counters.
- `update_audio_parameter_state`: Modifies internal timing or volume counters based on command identifiers and updates a global state.
- `parse_input_command_stream`: Reads a byte stream to extract command identifiers and parameters, updating internal state and global variables based on the decoded values.
- `scale_fixed_point_value`: Multiplies a value by a constant and performs bitwise shifts to return a result formatted as a 32-bit fixed-point value.
- `fill_multiple_buffers`: Clears various offsets within a memory block by writing zero to them in a loop.
- `write_shifted_values_to_buffers`: Reads values from a table, shifts them based on a global value, and writes them to multiple destination buffers.
- `copy_shifted_longs_to_planes_2`: Reads four source longwords per step from a table, shifts them left by the low four bits of a position value, and writes each one to a different plane buffer.
- `copy_table_to_buffers`: Copies data from a source table into multiple destination buffers using nested loops.
- `fill_buffers_from_table`: Reads values from a table and writes them into multiple destination buffers in a repeating pattern.
- `blend_table_to_buffers`: Performs bitwise logical operations between a source table and a destination buffer before clearing the remaining buffer space.
- `clamp_and_update_global`: Checks a global variable against specific bounds and performs conditional updates or increments.
- `clamp_global_value`: Checks a global variable against specific bounds and performs conditional updates or calls to other routines.
- `draw_shifted_planes_four`: Draws bit-shifted data into four destination planes using a mask, with the specific plane update logic determined by a table of four source words.
- `blend_bitplane_data`: Performs bitwise logical operations to merge multiple source words into a destination buffer using variable bit-shifting.
- `draw_masked_planes`: Draws bit-shifted data into four destination planes using a mask, with the specific plane update logic determined by a global variable.
- `draw_shifted_row_with_spill_2`: Draws up to eight source rows into four planes, each shifted by a value derived from an x position with spilled bits carried into the following word.
- `clear_destination_area`: Iterates through a source table to perform bitwise masking and OR operations on a destination buffer to clear specific pixel patterns.
- `clear_destination_buffer`: Clears a destination buffer by applying bitwise operations from a source table to the memory pointed to by a global variable.
- `execute_repeated_routine`: Calls another routine four times in a loop.
- `blend_pixel_data_tables`: Iterates through multiple memory blocks and applies bitwise masking and logical OR operations using values from a source table.
- `resolve_indirect_pointer`: Traverses multiple levels of indirection through a pointer structure to retrieve a short integer value.
- `interpolate_nibble_values`: Iterates through two tables of 16-bit words to incrementally adjust three nibbles per word toward target values.
- `execute_infinite_loop`: This routine is empty and consists only of an infinite loop.
- `clear_memory_blocks`: Iterates through a memory block to write zero to multiple consecutive words.
- `update_audio_parameters_2`: Modifies audio hardware registers by updating channel volume, frequency, or pitch values based on command identifiers and internal state.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_memory_block`: Copies a block of data from a source address to a destination address until a specific end address is reached.

### Castle Master (1990)(Domark)(M3)[cr MAD][m TFS].adf:x (raw code file, load address unknown)

- `stub_two_instruction_entry`: A two-instruction stub at the start of a raw code region, with no calls, callers, registers or text in the listed facts.

### Castle Master (1990)(Domark)(M3)[cr MAD][m TFS].adf:intro (raw code file, load address unknown)

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
- `call_routine_five_times`: Calls another routine five times in a loop.
- `draw_shifted_masked_planes_wide`: Draws a bit-shifted, masked block of four consecutive source words per group into four destination planes, with a separate case for the final partial word.
- `wrap_position_and_read_words`: Adds an offset to a 16-bit position and wraps it when it passes 318, repeating for a second offset.
- `nudge_palette_components_toward_source`: Compares the 4-bit red, green and blue components of sixteen words with those of a second table, nudging a component one step toward it only when the two differ by fifteen.
- `spin_in_infinite_loop`: The decompiled body is an empty infinite loop with no calls and no hardware access, and its 40 instructions are not all visible.
- `clear_ten_longs_across_planes`: Zeroes ten consecutive longwords and matching slots in four plane-sized regions on each pass, advancing ten longwords per pass over a fixed run of passes.
