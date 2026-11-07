# Game 1475 (castle master (1990)(domark)(m3)[cr pna]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01475). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (922128 top-level bytes in 15 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 93 of 93 named (0 author symbols), 93 of 93 described (6 by rule, 60 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1859 of 1859 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 472862 bytes, 22 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3952 of 3978 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 165173 |
| code | 15462 |
| data | 568795 |
| padding | 2714 |
| unused | 169984 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Castle Master (1990)(Domark)(M3)[cr PNA].adf:paraintro | 1 | 99 | 3 |
| Castle Master (1990)(Domark)(M3)[cr PNA].adf:cm | 1 | 116 | 2 |
| Castle Master (1990)(Domark)(M3)[cr PNA].adf:bootblock | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 342 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 492 | 19 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1665 | 41 |
| Castle Master (1990)(Domark)(M3)[cr PNA].adf:x (raw code file, load address unknown) | 1 | 2 | 1 |
| Castle Master (1990)(Domark)(M3)[cr PNA].adf:intro (raw code file, load address unknown) | 1 | 1253 | 21 |

## Cases handled

- runtime segment an address (play-rs-x25k): 170 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 415 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1089 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- castle master (1990)(domark)(m3)[cr pna]/Castle Master (1990)(Domark)(M3)[cr PNA].adf:x: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- castle master (1990)(domark)(m3)[cr pna]/Castle Master (1990)(Domark)(M3)[cr PNA].adf:intro: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- castle master (1990)(domark)(m3)[cr pna]/Castle Master (1990)(Domark)(M3)[cr PNA].adf:paraintro: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 never-loaded region(s), 472862 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- Castle Master (1990)(Domark)(M3)[cr PNA].adf:paraintro
- Castle Master (1990)(Domark)(M3)[cr PNA].adf:intro (raw code file, load address unknown)

## Copy protection found

- none found

## Functions


### Castle Master (1990)(Domark)(M3)[cr PNA].adf:paraintro

- `start_intro_with_four_calls`: Program entry of 45 instructions that writes the colour 18 register and calls four unnamed routines.
- `store_combined_byte_pair`: Reads a 32-bit value from memory just before the address register, then writes a 16-bit value built from the high byte of one register and the low byte of that value through a second pointer.
- `shift_bits_with_refill`: Shifts a bit buffer right once per iteration for a count taken from a second register, and calls an unnamed routine to refill the buffer whenever it becomes zero.

### Castle Master (1990)(Domark)(M3)[cr PNA].adf:cm

- `open_intro_file_and_load`: Program entry of 101 instructions that opens dos.library and the intro file, reads input, allocates memory and writes the background colour register.
- `write_text_string_to_output`: Measures a zero-terminated text string and writes it to the DOS output stream through the library's Write function.

### Castle Master (1990)(Domark)(M3)[cr PNA].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_audio_parameters_2`: Updates various audio-related memory locations and hardware registers.
- `update_audio_parameter_structure`: Extracts a command index from a word to look up values in a table and updates a data structure with new audio properties.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `update_cursor_position`: Updates a coordinate or index value based on a command type extracted from a word.

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
- `update_audio_volumes`: Walks a sound table in a loop, selecting entries against a running threshold, and writes the four audio channel volume registers from the selected values.
- `update_timer_counters`: Decrements a global variable and checks if it reaches zero.
- `increment_timer_counters`: Increments three separate counters if their corresponding global threshold variables exceed a specific value, while also incrementing those threshold variables by a fixed amount.
- `decrement_timer_and_update`: Decrements a global variable and, upon reaching zero, updates several other variables and potentially modifies a function pointer.
- `decrement_timer_counter`: Decrements a global variable and resets it to a fixed value when it reaches zero.
- `populate_data_table`: Fills a table in memory with a series of structured short integer values using a loop.
- `execute_library_sequence`: Calls a function pointer stored in a global variable and performs a sequence of indirect function calls through the returned pointer.
- `test_byte_and_branch`: Tests if a byte is greater than zero and returns a specific value if the condition is met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_fixed_point_state`: Updates global state variables and executes a sequence of indirect function calls via pointers retrieved from memory.
- `tiny_moveq`: Small routine of 1 instruction that loads 0 into d4.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `clear_data_register`: Performs a logical OR operation on a data register with zero.
- `clear_data_register_byte`: Performs a bitwise OR operation on the low byte of a data register with zero.
- `relocate_and_patch_function_table`: Retrieves a pointer from a global variable and uses it to populate a table of function pointers by reading offset values from a data block.
- `tiny_ori_2`: Small routine of 1 instruction that merges 69 into d0.
- `initialize_library_function_table`: Retrieves a library pointer from a global variable and populates a local function pointer table by applying relative offsets.
- `clear_memory_block_2`: Writes zero to a series of words in a memory block starting at a global variable.
- `clear_keyboard_buffer_entry`: Calculates an index based on a global variable and checks if it falls within a specific range.
- `store_byte_to_pointer`: Stores a single byte with the value 255 at a memory location calculated from a pointer and an offset.
- `process_string_data`: Iterates through a source string and copies characters to a destination buffer while performing bitwise operations and checking for a specific character.
- `clear_memory_range`: Fills a range of 111 words starting at a specific memory address with zeros.
- `copy_global_value`: Copies a value from a global variable to another global variable.
- `update_audio_parameters`: Checks a signature in memory and, if matched, performs complex data manipulation involving multiple tables and hardware audio volume registers.
- `reset_globals_to_constant`: Writes zero to four global variables and writes a constant value to a fifth global variable.
- `update_audio_hardware_parameters`: Updates audio hardware registers by modifying volume or frequency values based on command identifiers and internal counters.
- `update_audio_parameter_state`: Modifies internal timing or volume counters based on command identifiers and updates a global state.
- `parse_input_command_stream`: Reads a byte stream to extract command identifiers and parameters, updating internal state and global variables based on the decoded values.
- `calculate_fixed_point_product`: Performs a multiplication between a short integer and a global variable, then returns the result split into two parts via bit shifting.
- `fill_multiple_buffers`: Clears various offsets within a memory block by writing zero to them in a loop.
- `write_shifted_values_to_buffers`: Reads values from a table, shifts them based on a global value, and writes them to multiple destination buffers.
- `copy_shifted_longs_to_planes`: Reads four source longwords per step from a table, shifts them left by the low four bits of a position value, and writes each one to a different plane buffer.
- `copy_table_to_buffers`: Copies data from a source table into multiple destination buffers using nested loops.
- `fill_buffers_from_table`: Reads values from a table and writes them into multiple destination buffers in a repeating pattern.
- `blend_table_to_buffers`: Performs bitwise logical operations between a source table and a destination buffer before clearing the remaining buffer space.
- `clamp_global_value_2`: Checks a global variable against specific bounds and performs conditional updates or calls to other routines.
- `clamp_global_value`: Checks a global variable against specific bounds and performs conditional updates or calls to other routines.
- `draw_clipped_masked_block`: Draws a bit-shifted block from four source planes into four destination planes, limiting the column count by the distance to a 320-pixel edge and keeping destination pixels wherever all source planes are zero.
- `draw_wide_clipped_masked_block`: Draws a bit-shifted, masked four-plane block like the narrower variant, but limits the column count to eight and reads its source planes further apart.
- `draw_masked_planes`: Draws bit-shifted data into four destination planes using a mask, with the specific plane update logic determined by a global variable.
- `draw_shifted_row_with_spill`: Draws up to eight source rows into four planes, each shifted by a value derived from an x position with spilled bits carried into the following word.
- `clear_destination_area`: Iterates through a source table to perform bitwise masking and OR operations on a destination buffer to clear specific pixel patterns.
- `clear_destination_buffer`: Clears a destination buffer by applying bitwise operations from a source table to the memory pointed to by a global variable.
- `execute_repeated_routine`: Calls another routine four times in a loop.
- `blend_nibble_data_tables`: Iterates through nested loops to perform bitwise masking and logical OR operations between source data tables and destination memory buffers.
- `resolve_indirect_pointer`: Traverses multiple levels of indirection through a pointer structure to retrieve a short integer value.
- `interpolate_nibble_values`: Iterates through two tables of 16-bit words to incrementally adjust three nibbles per word toward target values.
- `execute_infinite_loop`: This routine is empty and consists only of an infinite loop.
- `clear_memory_block_3`: Iterates through a memory block pointed to by a global variable, writing zero to multiple offsets in a loop.
- `copy_data_sequence`: Decrements a global variable and, if another global variable is non-zero, copies a sequence of words from a fixed memory location to a destination pointed to by a global variable until a specific terminator value is encountered.

### Castle Master (1990)(Domark)(M3)[cr PNA].adf:x (raw code file, load address unknown)

- `stub_two_instruction_routine`: Two-instruction raw code region at the start of a file, with no calls, hardware registers or text in the facts, so its behaviour is not recovered.

### Castle Master (1990)(Domark)(M3)[cr PNA].adf:intro (raw code file, load address unknown)

- `open_libraries_and_set_copper`: Raw intro code region of 188 instructions that opens a library, calls three further library functions at fixed offsets, and writes the background colour register and the copper list pointer.
- `clear_strided_longword_rows`: Clears longwords at four offsets spaced at fixed intervals in a loop, zeroing a block of rows in each pass.
- `copy_shifted_word_rows`: Computes an index from a global value, then copies rows of 16-bit words into an output buffer, shifting the values as it goes.
- `copy_shifted_longword_rows`: Computes an index from a global value, then copies rows of longword values into an output buffer with shifts applied.
- `copy_table_rows_to_buffer`: Copies rows of 16-bit values from a fixed table into an output buffer with nested loops over row and column counts.
- `copy_table_quads_to_buffer`: Reads groups of four 16-bit values from a fixed table in a loop and stores them in sequence into an output area.
- `copy_table_words_with_offsets`: Reads 16-bit values from a fixed table, combines them with offsets, and stores them to output pointers in a loop.
- `step_clamped_counter_and_call`: Calls unnamed routines conditionally, based on comparisons of a global value against fixed thresholds.
- `advance_counter_with_limits`: Adjusts a global counter against limits, resets it in one case, and otherwise calls an unnamed routine and advances a stage value when the counter reaches a set value.
- `pack_shifted_longword_rows`: Reads longword values from an input pointer, shifts and combines them with a count from a register, and writes packed words to an output pointer across rows.
- `rotate_bits_and_apply_to_memory`: Takes a 4-bit nibble value and a shift count, then rotates the four bits of each of four 16-bit words left by that count.
- `pack_shifted_words_by_count`: Reads 16-bit words from an input pointer, combines them with shifted values, and writes them packed to an output pointer, looping by a count.
- `copy_masked_words_with_shift`: Copies words from an input pointer to an output pointer, applying a shift and mask chosen from a count, in a loop of eight passes.
- `copy_table_block_with_stride`: Copies 16-bit values from a fixed table with a stride into an output area in a loop.
- `copy_table_block_to_output`: Copies 16-bit values from another fixed table into an output area in a loop.
- `call_routine_five_times`: Calls one unnamed routine five times in a loop.
- `dispatch_twelve_unnamed_routines`: A 177-instruction routine called by one unnamed routine that calls twelve unnamed routines and manipulates word values held in registers.
- `scale_global_value_fixed_point`: Adjusts a global value using fixed-point arithmetic with a rounding offset, and stores the result back in a global.
- `update_global_state_values`: Updates several global state values from shifted and combined inputs in a 75-instruction routine.
- `stub_endless_loop`: Routine of 40 instructions whose decompiled body is an endless loop with no other work, called by two unnamed routines.
- `clear_longword_block`: Clears a block of memory in a loop of 200 passes, writing zero longwords to each group.
