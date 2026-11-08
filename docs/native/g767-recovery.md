# Game 767 (another world (1991)(u. s. gold)[cr amiga box][tr hu]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00767). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802240 top-level bytes in 28 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 220 of 220 named (0 author symbols), 220 of 220 described (5 by rule, 211 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1444 of 1444 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 924354 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6623 of 6623 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 561248 |
| code | 23252 |
| data | 1097236 |
| padding | 6840 |
| unused | 113664 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Type | 2 | 117 | 2 |
| Another.Hungarian | 1 | 3087 | 106 |
| Wait | 2 | 3 | 1 |
| Another World (1991)(U. S. Gold)[cr Amiga Box][tr Hu](Disk 1 of 2).adf:intro.comp | 2 | 233 | 8 |
| Another World (1991)(U. S. Gold)[cr Amiga Box][tr Hu](Disk 1 of 2).adf:another | 1 | 3075 | 101 |
| Another World (1991)(U. S. Gold)[cr Amiga Box][tr Hu](Disk 1 of 2).adf:bootblock | 1 | 54 | 1 |
| Another World (1991)(U. S. Gold)[cr Amiga Box][tr Hu](Disk 2 of 2).adf:bootblock | 1 | 54 | 1 |
| Another World (1991)(U. S. Gold)[cr Amiga Box][tr Hu](Disk 1 of 2).adf:bank0D (raw code file, load address unknown) | 1 | 0 | 0 |
| Another World (1991)(U. S. Gold)[cr Amiga Box][tr Hu](Disk 1 of 2).adf:bank01 (raw code file, load address unknown) | 1 | 0 | 0 |
| Another World (1991)(U. S. Gold)[cr Amiga Box][tr Hu](Disk 1 of 2).adf:bank02 (raw code file, load address unknown) | 1 | 0 | 0 |
| Another World (1991)(U. S. Gold)[cr Amiga Box][tr Hu](Disk 2 of 2).adf:bank0A (raw code file, load address unknown) | 1 | 0 | 0 |
| Another World (1991)(U. S. Gold)[cr Amiga Box][tr Hu](Disk 2 of 2).adf:bank03 (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- another world (1991)(u. s. gold)[cr amiga box][tr hu](disk 1 of 2)/Another World (1991)(U. S. Gold)[cr Amiga Box][tr Hu](Disk 1 of 2).adf:bank0D: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- another world (1991)(u. s. gold)[cr amiga box][tr hu](disk 1 of 2)/Another World (1991)(U. S. Gold)[cr Amiga Box][tr Hu](Disk 1 of 2).adf:bank01: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- another world (1991)(u. s. gold)[cr amiga box][tr hu](disk 1 of 2)/Another World (1991)(U. S. Gold)[cr Amiga Box][tr Hu](Disk 1 of 2).adf:bank02: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- another world (1991)(u. s. gold)[cr amiga box][tr hu](disk 2 of 2)/Another World (1991)(U. S. Gold)[cr Amiga Box][tr Hu](Disk 2 of 2).adf:bank0A: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- another world (1991)(u. s. gold)[cr amiga box][tr hu](disk 2 of 2)/Another World (1991)(U. S. Gold)[cr Amiga Box][tr Hu](Disk 2 of 2).adf:bank03: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 5 never-loaded region(s), 924354 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- Another World (1991)(U. S. Gold)[cr Amiga Box][tr Hu](Disk 1 of 2).adf:intro.comp

## Copy protection found

- none found

## Functions


### Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of register d1 into register d0 and then performs a short branch to another location.

### Another.Hungarian

- `run_initial_setup_routines`: Calls four setup routines in sequence, then loads the exec library base into a6 and stores 300 in a global variable, storing 2304 instead if bit 1 of the status byte at offset 425 of that structure is set.
- `dispatch_command_indices`: Compares an input value against specific constants to determine a set of three or four indices, then updates a global table by setting the values at those indices to two and calling another routine to load data from a table into global variables.
- `initialize_and_process_command_buffer`: Clears a table of 127 words and a secondary table of 31 longwords, then enters a loop that copies data from one buffer to another while calling a bytecode interpreter.
- `execute_bytecode_interpreter`: Parses a stream of commands from a buffer in a0, performing arithmetic, bitwise logic, and memory manipulation on a table.
- `branch_conditionally`: Performs a conditional branch to an offset of 976 and otherwise performs an unconditional branch to an offset of 4.
- `branch_if_not_equal`: Branches to an offset if the not equal condition is met, otherwise it performs an unconditional branch to another location.
- `branch_if_greater`: Branches to an offset if the greater than condition is met, otherwise it performs an unconditional branch to another location.
- `branch_if_greater_or_equal`: Branches to an offset if the greater than or equal condition is met, otherwise it performs an unconditional branch to another location.
- `branch_if_less`: Branches to an offset if the less than condition is met, otherwise it performs an unconditional branch to another location.
- `execute_bytecode_commands`: Parses a sequence of commands from a buffer in a0, performing operations such as arithmetic, bitwise logic, and memory manipulation on a table of 16-bit values.
- `dispatch_subroutine_via_type_tag`: Reads a byte from the address in a6 and branches to different routines based on its value masked with 63; if the value is 1, it calls itself; if the value is 2, it performs coordinate scaling and calls another routine in a loop; otherwise, it performs bitwise operations on a global variable using data from a6 and calls a function pointer or jumps to another routine.
- `loop_popping_stack_registers`: Pops several 16-bit registers from the stack, then repeatedly decrements d0 and branches back to an earlier instruction until it reaches zero, before popping a final 16-bit value and returning.
- `calculate_offset_address`: Multiplies register d0 by 20, adds 24218 to the result, and returns.
- `print_text_entry_for_key`: Scans a table of word-keyed entries for the key held in a register, returning immediately when a negative key is found.
- `apply_eight_row_tile_planes`: Checks that two coordinates lie within a bounded range, then for eight rows loads a pattern byte and applies it to four bitplane locations using an OR or an AND-NOT depending on the corresponding bit of a mode word.
- `decompress_bit_stream`: Reads a bitstream from a source, performs bitwise operations and calls other routines to extract data, then writes bytes to a destination buffer pointed to by a pointer in a register.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `shift_bit_window_right`: Shifts a 32-bit value right one bit at a time for the count held in register d1, reloading from the previous longword in memory and setting its top bit whenever the window empties.
- `select_highest_priority_object`: Walks a table of twenty-byte records, keeping the entry with the highest priority byte at offset 6 whose type byte equals 2, and stops at the end marker.
- `format_word_as_digit_chars`: Converts each of the four nibbles of the input word from the top into a character, using digits zero through nine or letters A through F, and returns the packed four-character result.
- `search_and_update_records`: Calculates a value by subtracting differences of two pairs of longwords from a global variable, then loops through a table of 20-byte structures to find an entry matching specific bitwise criteria and updates its first byte with a value derived from a bitwise OR operation.
- `update_record_status`: Clears register d0, moves the value from a1 to d0, and if d0 is not zero, sets a byte at offset 0 of a1 to 4 and performs a bitwise OR with d5, then subtracts a longword from offset 16 of a1 from d6 before returning.
- `process_table_entries`: Iterates through a table starting at a global variable, and if a byte at the current pointer has a lower nibble of 4, it calls another routine and subtracts a longword from a global variable depending on the state of bit 7 of that byte.
- `modify_register_and_branch`: Subtracts the value in d1 from d1, adds 20 to a0, and branches to another routine.
- `restore_registers_and_return`: Returns immediately after restoring registers d0 through d7 and a0 through a6 from the stack.
- `calculate_address_difference`: Saves registers to the stack, calculates the difference between registers d4 and d2, loads a longword from a global variable into d1, and then performs conditional branches based on register d5 and the result of a subtraction involving d1 and a1.
- `copy_and_update_table_2`: Copies a word from a1 to a2 and increments both pointers, then performs a loop using a counter in d1, swaps d1, and branches back to the start of the routine.
- `copy_and_update_table`: Copies a block of data from one location to another in a loop, then iterates through a table to subtract a value from specific entries based on comparison and bitwise checks.
- `restore_registers_from_stack`: Restores data registers and address registers from the stack and returns immediately.
- `update_object_state_from_command`: Compares an input value against a global variable and, if they differ, checks if the input matches specific command constants to set indices in registers.
- `clear_out_of_range_records`: Walks the twenty-byte record table for the stored record count, zeroing the first byte of each record whose second byte lies outside three to six, then copies two backup globals back into their working counterparts.
- `clear_all_records_restore_values`: Zeroes the first byte of every twenty-byte record for the stored record count, then copies two backup globals back into their working counterparts.
- `count_table_entries_to_sentinel`: Counts twenty-byte table entries until one has a first word of minus one, then stores the count of entries before that sentinel, minus one, in a global variable.
- `draw_line_with_blitter`: Swaps line endpoints when a flag is set, computes endpoint deltas and octant bits, waits for the blitter to become idle, programs the blitter registers, and starts a blit before returning.
- `clip_line_then_draw`: Clears a flag in a global variable and computes outcodes for both endpoints of a line, storing them in globals.
- `compute_scaled_differences`: Computes four scaled differences from the input registers, shifts each left by one bit, and packs the results into the low word of a data register before returning.
- `subdivide_segment_at_midpoint`: Computes scaled midpoint differences, calls a helper to pack them, then repeatedly halves a segment and compares the midpoint against the endpoints, returning when a stopping condition is met.
- `process_line_coordinates_and_clip`: Reads coordinate data from a buffer in a0, scales the values using a global multiplier and bit shifts, and calls another routine to process the line.
- `process_scaled_line_coordinates`: Scales coordinate data from a buffer using a global multiplier and bit shifts, then calculates screen positions and manages a list of line endpoints.
- `setup_blitter_transfer`: Loads several values into blitter registers and then branches to another routine.
- `wait_for_blitter_idle`: Polls bit fourteen of the DMA status register in a loop, returning only once the blitter reports itself idle.
- `decode_joystick_direction_flags`: Reads the joystick data word and derives four direction and button flags from its bits, using an XOR-shift of the movement bits to compute a second direction value, then stores the results into a state record.
- `copy_four_longwords_to_globals`: Calls a helper routine, then copies four 32-bit values from the record pointed to by a0 into four consecutive global variables, and returns.
- `scale_small_value_by_sixteen`: Returns the input word unchanged when it is above three; otherwise it returns the word multiplied by sixteen.
- `compute_offset_and_return`: Computes an offset from two input words, adds it to four stored pointers, loads a constant into a register, and then returns without touching memory.
- `process_object_collision_effects`: Calculates relative coordinates and properties from a data structure, then enters a loop to call a function pointer multiple times based on a count and specific bitwise conditions derived from the data.
- `mask_four_word_targets`: ANDs four 16-bit words, each reached through its own pointer, with a mask value, clearing every bit not set in the mask.
- `mask_three_words_return_code`: ANDs three 16-bit words, each reached through a pointer, with a mask value, then stores the input byte with a fixed set of bits ORed in.
- `apply_mask_bits_to_planes`: Walks up to four stored bitplane pointers and, for each mask bit that is set, ORs the inverted word into the plane word while for each clear bit it ANDs the word into the plane.
- `allocate_memory_or_fail`: Calls the system memory allocation routine through the exec library vector, and returns minus one when it gives back a null pointer.
- `free_memory_or_fail`: Calls the system memory-free routine through the exec library vector, then returns minus one when the call's result is zero.
- `queue_row_draw_commands`: Writes draw-command records for a span of screen rows, each holding fixed mode and mask words and offsets derived from the vertical position, and takes a separate path with its own offsets when the vertical value is at the edge of the normal range.
- `apply_bit_mask_four_words`: Checks four bits of a mask value in turn, storing an all-ones or all-zero word into a record field and calling a helper for each, then finishing by calling another helper.
- `switch_display_mode_copper`: Swaps two sets of four longwords in a global table when the input is 255, otherwise copies four longwords from a source table into the target table, then waits for a specific raster position to build and load a copper list that updates bitplane pointers and colors before restarting the copper.
- `process_keyboard_input_and_update_video`: Reads a byte from the keyboard serial register, performs a bitwise NOT and a circular rotation, then branches to various routines to update video hardware parameters, clear playfield control words, or reset input states based on the resulting value.
- `set_video_hardware_parameters`: Writes several 16-bit values to specific offsets of the hardware registers to configure video settings, then jumps to another routine.
- `set_video_hardware_registers`: Writes five 16-bit values to specific offsets of the hardware register base at offset zero.
- `clear_registers_and_set_video`: Clears four 16-bit words at specific offsets, clears a 32-bit global variable, and then writes six 16-bit values to specific offsets of the hardware register base at offset zero.
- `set_playfield_modulo_forty`: Clears two playfield control words and sets the two playfield modulo words to 40, then sets a stored flag to one.
- `reset_input_state_from_table`: Sets one stored word to all ones, copies the low byte of a table entry into another stored word, and clears a third stored word.
- `wait_for_raster_line`: Polls the vertical position register until its masked value equals 4096, then returns its input value unchanged.
- `write_dma_control_register`: Writes the DMA control register, where a set top bit enables the channels named in the write, and calls one other routine.
- `store_or_lookup_record`: If the first input word is zero, it stores the second input word into a global variable at offset 525A, calling a helper when that value is also zero; otherwise, it looks up a record via a pointer in d0, and if the masked word at that pointer equals 257, it stores the record's pointer and value into globals, otherwise it calls a helper and returns.
- `open_ciaa_and_fill_table`: Fills a fifteen-entry table with longwords taken from matching records, zeroing entries that do not match, and on first use opens the ciaa resource through the exec library and returns a flag for whether that call succeeded.
- `close_ciaa_resource_if_open`: If the ciaa resource is marked open, calls the exec library's close routine through its vector and clears the open flag.
- `write_ciaa_control_register`: Writes the byte value 129 to the CIAA control register.
- `copy_stored_value_to_global`: Copies a byte from offset 1 of a table into a global variable at offset 0, and another byte from offset 0 of the table into a global variable at offset 1.
- `clear_table_entry_fields`: Walks a fifteen-entry pointer table and zeroes the longword at offset eight of each non-null record, then returns.
- `silence_audio_channels`: Sets the volume register of each of the four audio channels to zero, then clears two stored values, then returns.
- `silence_audio_channels_set_flag`: Sets the volume registers of the four audio channels to zero and stores fifteen into a global variable, then returns.
- `call_audio_handler`: Calls another routine and returns with zero in register d0.
- `increment_and_call_audio_handler`: Increments a 16-bit word in a global variable, and if the value is not 6, continues; otherwise it clears the word and calls another routine.
- `run_routine_per_audio_channel`: Sets a counter to 3 and loads the audio channel register block, a helper routine, and a structure table into registers.
- `update_value_from_pointer`: Adds a word from a pointer in a4 to d0, and if the result is not negative, compares it with a word at offset 4 of the structure in a4 and stores the result in a global variable and at offset 2 of the structure in a4.
- `compare_and_branch_on_value`: Adds a word from a pointer in a6 to a value in d0, then compares the result with a word in d1 and branches based on the comparison.
- `branch_on_nibble`: Extracts a nibble from a byte at offset 2 of a structure in a6 and branches to different locations depending on whether the value is 1, 7, or 8.
- `store_word_with_offset`: Stores a word from a6 into a table at offset 2 of a pointer in a4, using a signed offset derived from a nibble in a6, and compares the value with a global variable.
- `copy_word_with_signed_offset`: Computes a signed word offset from an input value, selects the sign based on a flag, copies a word from a source address offset by that amount, stores it at an address derived from another register, then returns.
- `store_word_at_offset`: Stores a word from register d0 into a global variable at offset 4 of the address in a6.
- `store_byte_and_branch`: Stores a byte from d4 and then branches to another routine.
- `execute_table_jump`: Clears register d0 eight times, loads a word from a global variable, subtracts one, shifts it left by two, uses the result to index into a table to load a new address into a0, and jumps to that address.
- `fetch_and_compare_word`: Shifts a longword left, clears d1, loads a word from offset 16 of a structure in a6 into d1, then fetches a word from a table at a specific address using an offset in d0 and compares it with d1.
- `shift_and_branch`: Clears d0, loads a byte from offset 3 of the structure in a6 into d0, shifts it right by 4 bits, and branches to another routine.
- `mask_and_branch`: Clears d0, loads a byte from offset 3 of the structure in a6 into d0, masks it with 15, and branches to another routine.
- `copy_word_from_struct`: Copies a 16-bit word from offset 16 of a structure in a6 to offset 6 of a structure in a5, then returns.
- `start_audio_dma_channels`: Clears a channel-enable word, runs a helper for each of the four audio channels, then writes the enable word with its top bit set to the DMA control register.
- `process_audio_parameters`: Copies a longword from a pointer in a0 to a structure in a6, then uses a nibble from offset 2 of that structure to index into a table to fetch new values, which are then written to offsets 4, 8, 14, and 18 of the structure in a6, while also calculating a value at offset 10 based on offsets 4 and 8.
- `update_audio_parameter`: Stores a longword from a pointer in d0 into a structure in a6, sets a word at offset 14 of that structure to 1, and modifies a word at offset 18 based on a nibble at offset 2 and a byte at offset 3.
- `clamp_and_transfer_audio_parameters`: Clamps a value derived from adding a byte at offset 3 of a structure to a register to a maximum of 64, then writes this value to offset 8 of a second structure.
- `update_audio_structure`: If a pointer in a6 is non-zero, it clears a word in a4, copies a word from offset 16 of the structure in a6 to itself, writes a word from offset 20 of the structure to the DMA controller register, runs a loop based on a value in d7, and then either clears a word at offset 8 of a5 or copies a longword, a word, and another word from the structure in a6 into a5.
- `copy_channel_sample_fields`: Copies a longword and two words from a structure pointed to by a6 into the output block at a5, then ORs the record's flag word at offset 20 into a running mask at address 5254.
- `fill_bitplanes_by_mask`: Calls a helper routine, then for each of four bitplanes, fills a block of eight thousand longwords with either all ones or all zeros based on the corresponding bit of an input mask.
- `wait_for_keyboard_and_pause`: Releases an earlier DOS lock if one is pending, then configures the color and blitter registers and waits for a keyboard serial code, pausing in a nested loop when the code matches a specific value.
- `wait_for_disk_status_clear`: Polls the disk-status bit in the CIAA port register until it clears, then runs a thirty-two-thousand-iteration delay loop and writes zero four thousand nine hundred and sixty times to the background colour register.
- `manage_buffered_data_stream`: Manages a data stream by checking a global buffer pointer and length, calling routines to decompress or clear data, and executing a loop that processes chunks of data through a series of function calls until a termination condition is met.
- `prepare_color_delay`: Loads 596 into d0, calls another routine, loads negative one into d0, and branches to another location.
- `fill_color_register_delay`: Writes the value in d0 to the background colour register in a nested countdown loop of eight outer and one thousand inner iterations, so the repeated writes form a timed colour pulse.
- `delay_then_zero_color_register`: Runs a long countdown loop of thirty-two thousand iterations, then writes zero four thousand nine hundred and sixty times to the background colour register, clearing it after the delay.
- `allocate_buffers_open_libs`: Opens the DOS and graphics libraries, reads the free memory size, and lays out sixteen buffers spaced 8000 bytes apart, storing their pointers in globals.
- `update_video_hardware_registers`: Calls a routine to swap two sets of four longwords in a global table when the input is 255, then writes 992 to a custom register at offset 150, calls a routine to write five 16-bit values to hardware registers, and finally writes 34752 to the same custom register at offset 150.
- `register_interrupt_server_node`: Allocates memory through the exec library vector, returning negative one when the allocation fails.
- `tiny_addq_clr_rts`: Small routine of 3 instructions that adds 1 to (a1), then clears d0, then returns.
- `forbid_and_own_blitter`: Disables multitasking through the exec library and then claims the blitter through the graphics library, using the library base pointers loaded from two global variables.
- `permit_and_disown_blitter`: Releases the blitter through the graphics library and then re-enables multitasking through the exec library, using the library base pointers loaded from two global variables.
- `start_copper_list_from_struct`: Loads the copper list pointer from a field of a stored structure into a global variable, clears the copper restart register, and enables copper DMA by writing the set bit to the DMA control register.
- `call_four_routines_return_zero`: Calls four memory-free routines in sequence, each loading a pointer and a size before returning, and finally clears d0 before returning.

### Wait

- `wait_for_disk_ready`: Polls bit 6 of the CIAA Port A register in a loop until it is cleared, then returns.

### Another World (1991)(U. S. Gold)[cr Amiga Box][tr Hu](Disk 1 of 2).adf:intro.comp

- `process_amiga_hunk_data`: Parses a list of data structures by comparing values against specific constants and performs memory allocation and relocation operations.
- `walk_linked_list_and_calculate_size`: Walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `process_bitstream_data_2`: Loads a longword from a pointer in a0, performs several conditional calls to helper routines, and uses a loop to write bytes to a destination pointed to by a3 based on values derived from the input data.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `process_bitstream_data`: Performs bitwise operations and shifts on data from a source pointer in a0, using a loop controlled by d0, and writes results to a destination pointer in a3.
- `execute_trap_and_or_result`: Moves a 16-bit word from an offset of a2 into d4, calls a trap, and then ORs the contents of d2 into d0.

### Another World (1991)(U. S. Gold)[cr Amiga Box][tr Hu](Disk 1 of 2).adf:another

- `run_initial_setup_routines_2`: Calls four setup routines in sequence, then loads the exec library base into a6 and stores 300 in a global variable, storing 2304 instead if bit 1 of the status byte at offset 425 of that structure is set.
- `dispatch_command_buffer_indices`: Compares an input value against specific constants to determine a set of indices, then updates a global table by setting the values at those indices to two and copying data from another routine into global variables.
- `initialize_and_process_command_buffer_2`: Clears a table of 127 words and a secondary table of 31 longwords, then iterates through a command buffer to copy values into the first table and calls another routine to update object states.
- `interpret_bytecode_stream`: Parses a stream of commands from a buffer in a0, performing arithmetic, bitwise logic, and memory manipulation on a table.
- `execute_bytecode_interpreter_opcodes`: Parses a stream of commands from a buffer in a0, performing arithmetic, bitwise logic, and memory manipulation on a table.
- `dispatch_subroutine_by_type_tag`: Reads a byte from the address in a6 and branches to different routines based on its value masked with 63.
- `pop_stack_registers`: Pops word values from the stack into registers d0, d2, and d3, decrements d0 in a loop, pops a word into d4, and returns.
- `calculate_table_offset`: Multiplies a 16-bit value in d0 by 20, adds 24186 to the result, and returns.
- `search_table_for_key_and_process_text`: Scans a table of word-sized entries for a key provided in a register, returning immediately if a negative key is encountered.
- `apply_eight_row_tile_planes_2`: Checks that two input coordinates lie within a specific range, then calls compute_offset_and_return to locate a pattern table.
- `decompress_bitstream`: Reads a compressed stream from a source in a0, using bit-shifting and calls to helper routines to reconstruct bytes and write them to a destination in a2.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `shift_bit_window_right_2`: Shifts a 32-bit value right one bit at a time for the count held in d1, reloading from the previous longword in memory and setting its top bit whenever the window empties.
- `select_highest_priority_object_2`: Iterates through a table of twenty-byte records to find the entry with the highest priority byte at offset 6 where the type byte at offset 0 is 2.
- `convert_nibbles_to_ascii`: Iterates four times to extract each nibble from an input word, converting each nibble into its corresponding ASCII character for digits zero through nine or uppercase letters A through F, and returns the resulting four-character packed word in the return register.
- `search_and_update_table`: Calculates a value by subtracting two longwords from a global table, then enters a loop that iterates through a table of 20-byte structures to find an entry matching specific bitwise criteria, updates that entry's first byte to 4 combined with a bit from d5, and decrements the calculated value until it is no longer negative.
- `update_record_status_2`: Clears register d0, copies the pointer in a1 to d0, and if d0 is not zero, sets bit 2 of the byte at the address in a1, performs a logical OR of d5 into that same byte, subtracts a longword from offset 16 of the structure in a1 from d6, and if the result is negative, sets d0 to negative one and calls another routine before restoring registers and returning.
- `process_table_entries_2`: Saves registers to the stack and iterates through a table; for each entry where the byte at the current pointer is not 255 and the lower nibble is 4, it calculates a new address, calls a routine, and if bit 7 of the byte is not set, subtracts a longword from a global variable.
- `advance_table_pointer`: Subtracts a longword from a global variable from d1, adds 20 to a0, and branches to another location.
- `restore_registers_and_return_2`: Restores d0 through d7 and a0 through a6 from the stack and returns.
- `calculate_pointer_offset`: Subtracts a value in a register from another register, loads a value from a global variable into a register, and performs conditional branches based on the results of a comparison and a bit shift.
- `copy_and_update_table_3`: Copies a word from the address in a1 to the address in a2, decrements d1, and loops if d1 is not zero; then it swaps the bits of d1 and branches to another routine.
- `copy_and_update_list`: Copies a block of data from a1 to a2 using a loop, then iterates through a list of structures, checking a value against a comparison and updating it if certain conditions are met.
- `restore_registers_and_return_3`: Restores the data and address registers from the stack and returns immediately.
- `update_object_group_pointers`: Compares a value in a register against a global variable and a series of constants to select a set of indices, then writes the value two into specific offsets of a table and updates several global pointers with values from a structure based on those indices.
- `clear_out_of_range_records_2`: Iterates through a table of twenty-byte records for a specified count, zeroing the first byte of any record where the second byte is not between three and six inclusive, then restores two global variables from backup locations.
- `clear_records_and_restore_globals`: Iterates through a list of records based on a count stored in a global variable, zeroing the first byte of each twenty-byte record, then restores two global variables from backup values.
- `count_table_entries_to_sentinel_2`: Iterates through a table of twenty-byte entries, checking the first word of each, until a word containing minus one is encountered.
- `draw_line_with_blitter_2`: Calculates line endpoint deltas and octant bits, programs the blitter registers with source and destination addresses and step sizes, and performs a blit operation.
- `clip_and_draw_line`: Clears a flag in a global variable and calls another routine to compute outcodes for two line endpoints, storing them in global variables.
- `pack_scaled_differences`: Calculates four differences using values from registers and a global variable, scales each by shifting left by one bit, and packs the results into the low word of a data register using a swap operation.
- `subdivide_segment_at_midpoint_2`: Computes scaled midpoint differences, calls compute_scaled_differences to pack them, and then iteratively halves a segment by comparing the midpoint against the current endpoints until a stopping condition is met or a bitmask check fails.
- `scale_and_clip_line_coordinates`: Reads coordinate data from a buffer, scales the values using a global multiplier and bit shifts, and calculates screen positions.
- `process_line_coordinates_and_clip_2`: Scales coordinate data from a buffer using a global multiplier and bit shifts, performs clipping against screen boundaries, and manages a list of line endpoints for subsequent drawing.
- `setup_blitter_parameters`: Writes several values to the blitter control structure in a6, including source and destination pointers, and then branches to another routine.
- `wait_for_blitter_idle_2`: Polls bit 14 of the DMA status register in a loop, returning only once the blitter reports itself idle.
- `decode_joystick_direction_flags_2`: Reads the joystick data word from a hardware register and derives four direction and button flags using an XOR-shift of the movement bits, then stores these values into a state record at various offsets.
- `copy_four_longwords_to_globals_2`: Calls another routine and then copies four consecutive longwords from the memory location pointed to by the returned address into four consecutive global variables.
- `select_table_pointer_by_index`: Compares a 16-bit value in a register against specific thresholds to select one of three base addresses from a table, then returns the resulting pointer in a register.
- `apply_offset_to_four_pointers`: Calculates an offset by multiplying the second input word by 12 and adding the first input word, then adds this offset to four pointers retrieved from a table and stores the constant 40 in a register before returning.
- `execute_conditional_function_pointers`: Calculates an index and bounds from a table to determine a range of function pointers to call.
- `mask_four_word_targets_2`: Applies a 16-bit mask stored in d5 to four 16-bit words located at the memory addresses pointed to by a0, a1, a2, and a3, clearing all bits not set in the mask.
- `mask_three_words_and_return_byte`: Performs a bitwise AND on three 16-bit words located at the addresses pointed to by a1, a2, and a3 using a mask in d5, then returns a byte from d0 with bits 3, 4, and 6 set.
- `apply_mask_bits_to_planes_2`: Iterates through a table of bitplane pointers and, for each bit set in a mask, ORs the inverted word into the plane word, while for each bit not set, ANDs the word into the plane word.
- `allocate_memory_or_fail_2`: Calls the system memory allocation routine via a pointer in a global variable and returns minus one if the result is null, otherwise it returns the pointer.
- `free_memory_or_fail_2`: Calls the system memory-free routine using a pointer stored at a fixed offset in memory, then returns negative one if the call fails, otherwise it returns the result of the call.
- `queue_row_draw_commands_2`: Writes draw-command records for a span of screen rows into a structure, using fixed mode and mask words and calculating offsets based on a vertical position value from a global variable.
- `apply_bit_mask_four_words_2`: Iterates four times through a bitmask provided in a pointer, setting a 16-bit value to all ones if a bit is set or all zeros if not, and writes this value and several other parameters into a structure pointed to by a5.
- `switch_display_mode_copper_2`: Swaps two sets of four longs in a global table when the input is 255, otherwise copies four longs from a source table into a destination table, waits for a specific raster position, and builds a copper list to update bitplane pointers and colors before restarting the copper.
- `process_keyboard_input_and_update_video_2`: Reads a byte from the keyboard serial register, performs a bitwise NOT and a bitwise rotation, and then branches to various routines to update video hardware parameters, reset input states, or clear specific global variables based on the resulting value.
- `set_video_hardware_parameters_2`: Writes several 16-bit values to specific offsets of the hardware registers at address a fixed address, then jumps to another routine.
- `write_hardware_registers`: Writes specific values to five hardware registers starting at offset zero of the base address in a5.
- `clear_registers_and_write_hardware`: Clears four words at offsets 102, 104, 108, and 10a of the structure in a5, clears a global variable, and then writes specific values to five hardware registers starting at offset zero of the base address in a5.
- `set_playfield_modulo_forty_2`: Clears two playfield control words and sets the two playfield modulo words to 40, the row length used by the other blitter routines, then sets a stored flag to one.
- `update_input_state_from_table`: Sets a 16-bit global variable to all ones, copies the low byte from a table entry pointed to by a global variable into another 16-bit global variable, and clears a third 16-bit global variable before returning.
- `wait_for_raster_line_2`: Polls the vertical position register until its masked value equals 4096, then returns the input value unchanged.
- `configure_dma_channels`: Checks a value in a global variable and, if valid, calculates a bitmask to enable specific DMA channels by writing to the DMA control register via a call to another routine.
- `store_or_lookup_record_2`: If the first input word is zero, it stores the second input word into a global variable at offset 525A, calling a helper when that value is also zero.
- `open_ciaa_and_fill_table_2`: Iterates through a fifteen-entry pointer table to find records with a specific two-byte value, copying a longword from each matching record into a destination table and zeroing entries that do not match.
- `close_audio_resource_if_open`: Checks if a global flag is non-zero, then calls a library function via a pointer in a table and calls another routine to silence audio channels.
- `set_ciaa_control_register`: Writes the byte 129 to a specific offset of a global variable.
- `copy_stored_value_to_global_2`: Copies a byte from a table at offset 1 to a global variable, then copies a byte from the same table at offset 0 to another global variable, then returns.
- `clear_table_entry_fields_2`: Iterates through a fifteen-entry pointer table and, for every non-null pointer found, clears the longword at offset eight of the pointed-to structure before returning.
- `silence_audio_channels_2`: Sets the volume registers of the four audio channels to zero and clears two global 32-bit values, then returns.
- `silence_audio_channels_set_flag_2`: Sets the volume of the four audio channels to zero and stores fifteen into a global variable, then returns.
- `call_audio_handler_and_clear_d0`: Calls increment_and_call_audio_handler and then clears the d0 register before returning.
- `increment_and_call_audio_helper`: Increments a 16-bit word in a global variable, clears it if the value reaches 6, and calls another routine before returning.
- `process_audio_channel_data`: Sets a counter to 3 and initializes registers with a table address, a global variable address, and the audio channel register block.
- `update_value_from_pointer_2`: Adds a word from a pointer in a4 to d0, and if the result is not negative, updates a word at offset 2 of the structure in a4 with the value from d1, then stores the result in offset 6 of a structure in a5 and offset 2 of a4 before returning.
- `update_value_from_pointer_3`: Adds a word from a pointer in a4 to d0, and if the result is not negative, compares it with a word at offset 4 of the structure in a4 and updates d0 accordingly.
- `branch_on_nibble_2`: Loads a byte from an offset of 2 in a6, masks the lower nibble, and branches to one of three different locations depending on whether the value is 1, 7, or 8; otherwise it returns.
- `search_and_copy_word_with_offset`: Calculates a signed word offset from a byte in a structure, searches a table for a matching word, and copies a word from the table at the calculated offset into a destination structure.
- `copy_word_with_conditional_sign`: Calculates a word offset by multiplying a value in d2 by two and applying a negative sign if d4 is not zero, then copies a word from that offset in a0 to a location four bytes past the address in a4 and returns.
- `store_word_at_offset_2`: Stores a word from register d0 at offset four of the address in a6, then returns.
- `store_byte_and_branch_2`: Stores a byte from register d4 and then branches to another routine.
- `execute_table_jump_2`: Clears register d0 eight times, loads a table address into a0, calculates an index from a global variable, and jumps to the address found at that index in the table.
- `search_table_for_word`: Searches a table for a 16-bit word, using a doubled index in d0 to traverse the table, and branches based on whether the word at the current table entry matches a value from a structure.
- `shift_and_branch_2`: Clears register d0, loads a byte from offset 3 of the structure in a6 into d0, shifts it right by 4 bits, and then branches to another routine.
- `mask_and_branch_2`: Clears register d0, loads a byte from offset 3 of the structure in a6 into d0, masks the lower 4 bits, and then branches to another routine.
- `copy_word_from_structure`: Copies a word from offset sixteen of a structure in a6 to offset six of a structure in a5, then returns.
- `start_audio_dma_channels_2`: Clears a channel-enable word, calls a helper routine four times to process audio parameters for each channel, and then sets the top bit of the enable word in the DMA control register.
- `process_audio_parameters_2`: Copies a longword from a table to a structure in a6, extracts a nibble from offset 2 of that structure to calculate an index into another table, and then populates the structure in a6 with values from the indexed table entries and performs arithmetic to set offset 10 and offset 14 of the structure.
- `update_audio_parameter_2`: Stores a longword from a pointer in d0 into a structure at offset 10 of a6, sets a word at offset 14 of a6 to 1, and then modifies a word in d3 based on a masked byte at offset 2 of a6 and a byte at offset 3 of a6.
- `update_audio_structure_2`: Calculates a value by adding a byte from offset 3 of a structure in a6 to a value in d2, clamps it to a maximum of 64, and stores it at offset 8 of a structure in a5.
- `copy_sample_fields`: Checks if a pointer in a6 is non-zero, then copies a word from offset 16 of that structure to itself, writes a word from offset 20 to the hardware register at a fixed address, and performs a loop based on a value from a global variable.
- `update_sample_mask_and_fields`: Copies a longword and two words from a structure pointed to by a6 into a destination block pointed to by a5, then performs a bitwise OR of a word from offset 20 of the source structure into a global variable.
- `fill_bitplanes_by_mask_2`: Calls a helper to obtain a value, then for each of four bitplanes fills a block of eight thousand longwords with all ones or all zeros, chosen by the corresponding bit of an input mask.
- `release_lock_and_wait_for_key`: Releases a pending file lock if a global pointer is non-zero, then configures color registers and waits for a specific keyboard serial code while executing nested delay loops.
- `wait_for_disk_status_clear_2`: Polls the disk-status bit in the CIA port register until it clears, then runs a thirty-two-thousand-iteration delay loop and writes zero four thousand nine hundred and sixty times to the background colour register.
- `manage_resource_file_stream`: Checks a global pointer and a length value to determine if a file resource needs to be opened, closed, or updated.
- `prepare_delay_and_reset`: Loads a word into d0, calls another routine, sets d0 to a negative value, and branches to another location.
- `pulse_background_color`: Writes the value in d0 to the background color register within a nested loop consisting of eight outer and 65535 inner iterations to create a timed color pulse.
- `delay_and_clear_background_color`: Runs a countdown loop of thirty-two thousand iterations while writing a specific value to the background color register, then runs a second loop of four thousand nine hundred and sixty iterations writing zero to the same register before clearing it.
- `allocate_and_initialize_memory_buffers`: Opens the DOS and graphics libraries, reads the free memory size, and lays out sixteen buffers spaced 8000 bytes apart, storing their pointers in globals.
- `toggle_dma_and_update_custom_register`: Calls switch_display_mode_copper, writes 992 to a custom register at offset 150, calls set_video_hardware_registers, then writes 34752 to the same custom register and returns.
- `register_interrupt_server_node_2`: Allocates memory through a routine that calls the system memory allocation vector, then populates the returned structure with a data pointer at offset 14, a priority of 100 at offset 9, a type code of 2 at offset 8, and a handler routine at offset 18, before registering the node with the system via an interrupt server call.
- `increment_global_short`: Increments a 16-bit value at a location pointed to by a1 and clears d0 before returning.
- `forbid_and_own_blitter_2`: Disables multitasking using the library base pointer in a global variable and then claims the blitter using the library base pointer in another global variable before returning.
- `permit_and_disown_blitter_2`: Calls a function at offset 486 from the library base stored in a global variable and then calls a function at offset 138 from the library base stored in another global variable before returning.
- `setup_copper_dma`: Loads the copper list pointer from an offset of 38 in a structure into the copper list pointer register, clears the copper restart register, and enables copper DMA by writing to the DMA control register.
- `free_four_memory_blocks`: Loads four pairs of pointers and sizes from global memory locations and calls a memory-free routine for each pair before returning zero in the accumulator.

### Another World (1991)(U. S. Gold)[cr Amiga Box][tr Hu](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Another World (1991)(U. S. Gold)[cr Amiga Box][tr Hu](Disk 2 of 2).adf:bootblock

- `boot_entry_2`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
