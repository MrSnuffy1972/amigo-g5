# Game 613 (Castle Master): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00613). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (922128 top-level bytes in 16 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 89 of 89 named (0 author symbols), 89 of 89 described (5 by rule, 81 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1851 of 1851 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 618110 bytes, 26 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4250 of 4290 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 165171 |
| code | 15308 |
| data | 568951 |
| padding | 2714 |
| unused | 169984 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Castle Master.adf:paraintro | 1 | 99 | 3 |
| Castle Master.adf:cm | 1 | 116 | 2 |
| Castle Master.adf:bootblock | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 342 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 485 | 18 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1635 | 34 |
| Castle Master.adf:paraintro#unpacked | 1 | 349 | 4 |
| Castle Master.adf:x (raw code file, load address unknown) | 1 | 2 | 1 |
| Castle Master.adf:intro (raw code file, load address unknown) | 1 | 1253 | 21 |

## Cases handled

- runtime segment an address (play-rs-m1500): 170 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 415 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1081 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Castle Master/Castle Master.adf:paraintro: crunched executable unpacked with ancient scan (TetraPack 2.2 (xfd), 49804 -> 145248 bytes), 32 of 32 windows confirmed in reference RAM, loaded at an address: recovered as u7
- Castle Master/Castle Master.adf:x: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Castle Master/Castle Master.adf:intro: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 3 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 call target, 1 immediate+address, 1 jump target
- Castle Master/Castle Master.adf:paraintro: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 3 never-loaded region(s), 618110 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- Castle Master.adf:paraintro
- Castle Master.adf:paraintro#unpacked
- Castle Master.adf:intro (raw code file, load address unknown)

## Copy protection found

- none found

## Functions


### Castle Master.adf:paraintro

- `perform_bit_stream_decompression`: Decompresses a bitstream by reading bits and using them to determine the length and value of subsequent data segments.
- `shift_and_store_word`: Performs a logical right shift on a value and stores the result into a memory location.
- `perform_recursive_bit_shift_loop`: Decrements a value in a register while right-shifting another register until it reaches zero, at which point it calls another routine and continues the loop based on the returned value.

### Castle Master.adf:cm

- `initialize_system_environment`: Initializes the system environment by opening files and allocating memory.
- `print_null_terminated_string`: Measures a null-terminated string and passes its length through a library entry at offset -60.

### Castle Master.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_audio_parameters_2`: Retrieves sound properties from a table and updates audio hardware registers and target structures.
- `update_audio_parameter_structure`: Retrieves an index from a data structure to look up sound properties in a table and updates a target structure with new pitch, duration, and volume values.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `update_cursor_position`: Updates a coordinate value based on input flags, handling movement, boundary clamping, and table-based lookups.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `perform_sequence_of_calls`: Executes a series of indirect calls and updates several global variables, including one via a pointer, before entering a loop that waits for a specific bit in a global variable to clear.
- `clear_memory_block`: Clears a large block of memory by setting every byte to zero starting from a specific address up to a defined limit.
- `initialize_and_copy_data_tables`: Clears a memory region and populates it by interleaving bytes from a source table.
- `clear_memory_and_call_function`: Clears a block of memory by writing zeros through a pointer and then calls a function pointer retrieved from a global variable.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `store_global_values`: Copies a global variable into two other memory locations, then updates three global variables with specific constants and a function pointer.
- `enable_hardware_interrupts`: Sets the most significant bit in two hardware registers to enable interrupts.
- `execute_conditional_calls`: Checks a bit in a global variable and calls another routine multiple times if the bit is set, then updates two global variables with specific constants.
- `update_blt_parameters`: Performs calculations based on a global value and updates several hardware registers associated with the Blitter.
- `update_sprite_animation_frames`: Iterates through a table of objects to update their animation state and frame indices.
- `toggle_global_state`: Toggles a global variable between two specific values and updates four other memory locations with different sets of constants based on that state.
- `set_audio_channel_volumes`: Walks a sound table in a loop, selecting entries against a running threshold, and writes the four audio channel volume registers from the selected values.
- `update_timer_counters`: Decrements a global variable and, if it reaches zero, resets it and updates three separate counter sets by incrementing them based on threshold comparisons.
- `increment_timer_counters`: Increments three separate counters if their corresponding global threshold variables exceed a specific value, while also incrementing those threshold variables by a fixed amount.
- `decrement_timer_counter_2`: Decrements a global variable and, upon reaching zero, updates several other variables and potentially modifies a function pointer.
- `decrement_timer_counter`: Decrements a global variable and resets it to a fixed value when it reaches zero.
- `populate_data_table`: Fills a table in memory with a series of structured short integer values using a loop.
- `execute_library_sequence`: Calls a function pointer stored in a global variable and performs a sequence of indirect function calls through the returned pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_and_store_register`: Performs a short branch, stores a word from a register into a memory location, and then performs a long branch.
- `clear_memory_block_2`: Clears a block of memory by writing zero to a series of words.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (movea.w) and then falls through into the code that follows, with no return of its own.
- `process_string_data`: Iterates through a source string and copies characters to a destination buffer while performing bitwise operations and checking for a specific character.
- `clear_memory_range`: Fills a range of 111 words starting at a specific memory address with zeros.
- `copy_byte_to_global`: Stores a byte from a memory location into a global variable.
- `update_audio_parameters`: Checks a signature in memory and, if matched, performs complex data manipulation involving multiple tables and hardware audio volume registers.
- `reset_globals_to_constant`: Writes zero to five global variables and writes a constant value to a fifth global variable.
- `update_audio_hardware_parameters`: Updates audio hardware registers and internal volume or pitch counters based on command identifiers and table lookups.
- `update_audio_parameter_state`: Modifies internal timing or volume counters based on command identifiers and updates a global state.
- `parse_input_command_stream`: Reads a byte stream to extract command identifiers and parameters, updating internal state and global variables based on the decoded values.
- `calculate_fixed_point_multiplication`: Performs a multiplication of a short integer by a global value and returns the result as a 32-bit value split across two registers.
- `load_music_data_and_update_audio`: Loads music data from a memory location into several global pointers, opens a library, and updates audio parameters.
- `fill_multiple_buffers`: Clears various offsets within a memory block by writing zero to them in a loop.
- `write_shifted_values_to_buffers`: Reads values from a table, shifts them based on a global value, and writes them to multiple destination buffers.
- `copy_shifted_longs_to_planes`: Reads four source longwords per step from a table, shifts them left by the low four bits of a position value, and writes each one to a different plane buffer.
- `copy_table_to_buffers`: Copies data from a source table into multiple destination buffers using nested loops.
- `fill_buffers_from_table`: Reads values from a table and writes them into multiple destination buffers in a repeating pattern.
- `blend_table_to_buffers`: Performs bitwise logical operations between a source table and a destination buffer before clearing the remaining buffer space.
- `clamp_and_update_global`: Checks a global variable against specific bounds and performs conditional updates or calls to other routines.
- `clamp_global_value`: Checks a global variable against specific bounds and performs conditional updates or calls to other routines.
- `draw_shifted_planes_four`: Draws bit-shifted data into four destination planes using a mask, with the specific plane update logic determined by a table of four source words.
- `blend_bitplane_data`: Performs bitwise logical operations to merge multiple source words into a destination buffer using variable bit-shifting.
- `draw_masked_planes`: Draws bit-shifted data into four destination planes using a mask, with the specific plane update logic determined by a global variable.
- `draw_shifted_row_with_spill`: Draws up to eight source rows into four planes, each shifted by a value derived from an x position with spilled bits carried into the following word.
- `clear_destination_area`: Iterates through a source table to perform bitwise masking and OR operations on a destination buffer to clear specific pixel patterns.
- `clear_destination_buffer`: Clears a destination buffer by applying bitwise operations from a source table to the memory pointed to by a global variable.
- `execute_repeated_routine`: Calls another routine four times in a loop.
- `blend_nibble_data_to_buffers`: Iterates through a source table of data to perform bitwise masking and logical OR operations on multiple destination buffers.
- `resolve_indirect_pointer`: Traverses multiple levels of indirection through a pointer structure to retrieve a short integer value.
- `interpolate_nibble_values`: Iterates through two tables of 16-bit words to incrementally adjust three nibbles per word toward target values.
- `execute_infinite_loop`: This routine is empty and consists only of an infinite loop.
- `clear_memory_block_3`: Iterates through a memory block pointed to by a global variable and writes zero to various offsets and indices in a loop.
- `copy_table_data`: Decrements a global variable, calls another routine, and then copies a sequence of words from a table to a destination pointed to by a global variable until a specific terminator value is encountered.

### Castle Master.adf:paraintro#unpacked

- `update_audio_channels`: Clears the four audio channel registers and sets the volume for all channels to a fixed value.
- `process_object_command_and_audio`: Reads a control structure from a pointer, uses a nibble from the structure to index into a table to update object properties, and modifies audio hardware registers and DMA control.
- `mask_and_compare_word_2`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `update_object_state`: Processes a control structure in a6 by reading a command byte and performing various arithmetic or bitwise operations on values at offsets 2, 3, 16, and 18.

### Castle Master.adf:x (raw code file, load address unknown)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Castle Master.adf:intro (raw code file, load address unknown)

- `initialize_system_resources`: Opens a library, checks a return value, and copies a block of data from a source to a destination based on a loop terminated by a specific word.
- `clear_long_slots_across_planes`: Zeroes pairs of longword slots spread across four plane-sized regions over a fixed run of passes.
- `blit_shifted_source_to_planes`: Copies rows of a source table into four plane buffers, shifting each word left by the low four bits of a position value.
- `copy_shifted_longs_to_planes_2`: Reads four source longwords per step from a table, shifts them left by the low four bits of a position value, and writes each one to a different plane buffer.
- `copy_four_plane_block`: Copies a twenty-word-wide block from a source whose four sub-blocks sit twenty words apart into four plane buffers spaced 8000 bytes apart.
- `fill_run_length_plane_rows`: Reads records of a row count and three words from a table, then writes the three words across twenty columns of planes 0 to 2 and zeroes plane 3 for count plus one rows.
- `merge_masked_planes_and_clear`: Merges a twenty-column block from four source planes into four destination planes, keeping destination pixels wherever all four source planes are zero.
- `clamp_and_update_global_2`: Validates a global variable against specific numerical limits and performs conditional updates or calls to other routines.
- `advance_phase_counter_once`: Resets a signed counter to zero when it is below minus twelve, otherwise calls an unnamed routine and, when the counter equals three, resets it and calls the routine again.
- `blit_shifted_bitplanes`: Performs a bitplane blit by shifting source words and masking destination data using a table of four source words to update multiple bitplanes simultaneously.
- `blend_bitplane_data_2`: Performs bitwise logical operations to merge multiple source words into a destination buffer using variable bit-shifting.
- `draw_mode_selected_plane_row`: Draws one source word per row into destination planes after a sub-word shift, clamping the column count to two and adding an extra plane write when the mode value equals two.
- `draw_shifted_row_with_spill_2`: Draws up to eight source rows into four planes, each shifted by the low four bits of an x position with spilled bits carried into the following word.
- `merge_sparse_table_at_offset`: Walks a fixed source table four words at a time, skipping all-zero groups and merging the others into four destination planes at an offset while keeping background pixels where the source is zero.
- `merge_sparse_table_at_base`: Walks a second fixed source table four words at a time, skipping all-zero groups and merging the others into the base of the plane area.
- `call_routine_five_times`: Calls another routine five times in a loop.
- `blend_pixel_data_tables`: Iterates through nested loops to perform bitwise masking and logical OR operations between source table values and multiple destination buffers.
- `wrap_position_and_read_words`: Adds an offset to a 16-bit position and wraps it when it passes 318, repeating for a second offset.
- `nudge_palette_components_toward_source`: Compares the 4-bit red, green and blue components of sixteen words with those of a second table, nudging a component one step toward it only when the two differ by fifteen.
- `spin_in_infinite_loop`: An empty infinite loop that performs no operations and contains no hardware access or function calls.
- `clear_memory_blocks`: Iterates through a loop to clear multiple specific memory locations and offsets by writing zero to them.
