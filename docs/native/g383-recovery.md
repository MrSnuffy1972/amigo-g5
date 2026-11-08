# Game 383 (Another World): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00383). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1806608 top-level bytes in 25 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 140 of 140 named (0 author symbols), 140 of 140 described (3 by rule, 134 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1204 of 1204 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 924354 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3726 of 3728 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 561016 |
| code | 13548 |
| data | 1070539 |
| padding | 5345 |
| unused | 156160 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Another World (1991)(U. S. Gold)[cr CSL](Disk 1 of 2).adf:Crystal | 2 | 85 | 3 |
| Another World (1991)(U. S. Gold)[cr CSL](Disk 1 of 2).adf:another | 1 | 3081 | 103 |
| Another World (1991)(U. S. Gold)[cr CSL](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 156 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 396 | 26 |
| Another World (1991)(U. S. Gold)[cr CSL](Disk 1 of 2).adf:bank0D (raw code file, load address unknown) | 1 | 0 | 0 |
| Another World (1991)(U. S. Gold)[cr CSL](Disk 1 of 2).adf:bank01 (raw code file, load address unknown) | 1 | 0 | 0 |
| Another World (1991)(U. S. Gold)[cr CSL](Disk 1 of 2).adf:bank02 (raw code file, load address unknown) | 1 | 0 | 0 |
| Another World (1991)(U. S. Gold)[cr CSL](Disk 2 of 2).adf:bank0A (raw code file, load address unknown) | 1 | 0 | 0 |
| Another World (1991)(U. S. Gold)[cr CSL](Disk 2 of 2).adf:bank03 (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4E4F) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address: 96 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 216 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- another world (1991)(u. s. gold)[cr csl](disk 1 of 2)/Another World (1991)(U. S. Gold)[cr CSL](Disk 1 of 2).adf:bank0D: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- another world (1991)(u. s. gold)[cr csl](disk 1 of 2)/Another World (1991)(U. S. Gold)[cr CSL](Disk 1 of 2).adf:bank01: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- another world (1991)(u. s. gold)[cr csl](disk 1 of 2)/Another World (1991)(U. S. Gold)[cr CSL](Disk 1 of 2).adf:bank02: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- another world (1991)(u. s. gold)[cr csl](disk 2 of 2)/Another World (1991)(U. S. Gold)[cr CSL](Disk 2 of 2).adf:bank0A: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- another world (1991)(u. s. gold)[cr csl](disk 2 of 2)/Another World (1991)(U. S. Gold)[cr CSL](Disk 2 of 2).adf:bank03: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- another world (1991)(u. s. gold)[cr csl](disk 2 of 2)/Another World (1991)(U. S. Gold)[cr CSL](Disk 2 of 2).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- 5 never-loaded region(s), 924354 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Another World (1991)(U. S. Gold)[cr CSL](Disk 1 of 2).adf:Crystal

- `depack_packed_stream_backward`: Decodes a packed bit stream by repeatedly calling the bit-reader refill and a multi-bit read routine, then copying literal and back-reference runs backwards into memory.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Another World (1991)(U. S. Gold)[cr CSL](Disk 1 of 2).adf:another

- `run_initial_setup_routines`: Calls four setup routines in sequence, then loads the exec library base into a6 and stores 300 in a global variable, storing 2400 instead if bit 1 of the status byte at offset 425 of that structure is set.
- `select_object_from_input_value`: Clears a byte in a table at offset 120 of the structure in a6, then compares a 16-bit value in d0 against a series of constants and, when one matches, sets up indices and calls select_object_by_input_value.
- `reset_tables_and_process_input`: Clears a flag in a global variable and fills a table of 127 words with -1 and a table of 31 longs with 0, then copies 63 bytes from one table to another.
- `decode_instruction_stream_2`: Reads a byte-coded instruction from the buffer in a0 and dispatches on its opcode, handling comparison and branch commands that test values from a table at offset 5C78, variable assignment and copy commands, and a command that pushes a pointer onto a stack in a5 before returning.
- `conditional_then_unconditional_branch`: Tests whether the zero flag is set and branches when it is, then unconditionally branches to a nearby instruction and returns.
- `test_condition_branch`: Tests a condition and branches to another routine if it is not equal, otherwise falls through to a further routine.
- `decode_instruction_stream`: Decodes a stream of byte-coded instructions from a buffer in a0, dispatching on each opcode: comparing and branching on 16-bit values, copying and arithmetic-updating 16-bit values in a table indexed by a byte, and setting flag bits in a global variable, otherwise returning.
- `dispatch_blitter_command_handler`: Dispatches based on a command byte read from the buffer in a6: for command 1 it calls another routine and returns, for command 2 it walks a linked list of coordinate pairs adjusting values in d2 and d3, and otherwise it reads a byte from a global variable indexed by the clamped d2 value and calls one of four routines from a table.
- `loop_popping_stack_registers`: Pops saved registers off the stack, then repeatedly decrements a counter and branches back to an earlier instruction until it finishes, before popping a final value and returning.
- `compute_indexed_pointer`: Multiplies the low word of the input register by twenty and adds the offset of a global variable, returning a pointer to that computed location.
- `print_text_entry_for_key`: Scans a table of word-keyed entries for the key held in a register, returning immediately when a negative key is found.
- `apply_eight_row_tile_planes`: Checks that two coordinates lie within a bounded range, then for eight rows loads a pattern byte and applies it to four bitplane locations using an OR or an AND-NOT depending on the corresponding bit of a mode word.
- `decompress_bit_stream_bytes`: Reads a previous longword from a packed stream, shifts it right while refilling from memory, and copies decompressed bytes into a destination buffer, branching to different handling depending on the decoded values.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `shift_bit_window_right`: Shifts a 32-bit value right one bit at a time for the count held in the input register, reloading from the previous longword in memory and setting its top bit whenever the window empties.
- `select_highest_priority_object`: Walks a table of twenty-byte records, keeping the entry with the highest priority byte at offset 6 whose type byte equals 2, and stops at the end marker.
- `format_word_as_digit_chars`: Converts each of the four nibbles of the input word from the top into a character, using digits zero through nine or letters A through F, and returns the packed four-character result.
- `find_and_update_table_entry`: Computes a difference between two pairs of global values and subtracts it from a register, then scans a table of records advancing by 20 bytes to find the last entry whose first byte has a low nibble of 3 and whose high bit matches a flag, storing the value 4 and combining the flag into that entry before subtracting a stored word and looping while the counter stays positive.
- `store_flag_and_decrement_offset`: Clears register d0, copies the pointer in a1 into d0, and returns immediately when that pointer is null.
- `process_table_records`: Walks a table of records by advancing a pointer, and for each record whose low nibble is four it calls a helper and subtracts the returned value from a global variable depending on a sign flag.
- `subtract_then_advance_pointer`: Subtracts a register from a global variable, increments an address register, then branches to a nearby instruction and returns.
- `restore_registers_and_return`: Restores the caller's registers and returns immediately.
- `save_registers_and_compare`: Saves the data and address registers on the stack, computes the difference between a1 and a2, loads a pointer from a global variable and returns when d5 is zero, otherwise loads a second pointer from another global variable, subtracts a1 from it, shifts the result right by one bit, and jumps to the shared code at offset ade.
- `copy_words_then_update_table_2`: Copies a word from the buffer pointed to by one register into the buffer pointed to by another, advancing both pointers.
- `copy_words_then_update_table`: Copies words from one buffer to another in a loop, decrements a counter and branches back until it is exhausted, then walks a table of entries updating a stored value while a condition holds.
- `restore_registers_and_return_2`: Restores the seven data registers d0-d7 and the six address registers a0-a6 from the stack, then returns.
- `select_object_by_input_value`: Selects a game object from a table of twenty-byte records based on a 16-bit value passed in d0, mapping that value to three or four record indices and marking each with the byte 2, then copies the selected records' data into working variables.
- `clear_out_of_range_records`: Walks the twenty-byte record table for the stored record count, zeroing the first byte of each record whose second byte lies outside three to six, then copies two backup globals back into their working counterparts.
- `clear_all_records_restore_values`: Zeroes the first byte of every twenty-byte record for the stored record count, then copies two backup globals back into their working counterparts.
- `count_table_entries_to_sentinel`: Counts twenty-byte table entries until one has a first word of minus one, then stores the count of entries before that sentinel, minus one, in a global variable.
- `draw_line_with_blitter`: Swaps line endpoints when a flag is set, computes endpoint deltas and octant bits, waits for the blitter to become idle, programs the blitter registers, and starts a blit before returning.
- `clip_line_then_draw`: Clears a flag in a global variable and computes outcodes for both endpoints of a line, storing them in globals.
- `compute_scaled_differences`: Computes four scaled differences from the input registers, shifts each left by one bit, and packs the results into the low word of a data register before returning.
- `subdivide_segment_at_midpoint`: Computes scaled midpoint differences, calls a helper to pack them, then repeatedly halves a segment and compares the midpoint against the endpoints, returning when a stopping condition is met.
- `process_clamped_line_segment`: Reads a line endpoint from the buffer in a0, scales its coordinates by a table value and adds base offsets, then calls the clipping routine to compute the endpoints.
- `process_clamped_line_segment_2`: Clears a status byte and stores a 16-bit value, then polls the DMA status register until the blitter is idle.
- `program_blitter_then_start`: Programs the blitter registers with the supplied values, waits for the blitter to become idle, then branches to a routine that starts the blit and returns.
- `wait_for_blitter_idle`: Polls bit fourteen of the DMA status register in a loop, returning only once the blitter reports itself idle.
- `decode_joystick_direction_flags`: Reads the joystick data word and derives four direction and button flags from its bits, using an XOR-shift of the movement bits to compute a second direction value, then stores the results into a state record.
- `copy_four_longwords_to_globals`: Calls a helper routine, then copies four 32-bit values from the record pointed to by a0 into four consecutive global variables, and returns.
- `scale_small_value_by_sixteen`: Returns the input word unchanged when it is above three; otherwise it returns the word multiplied by sixteen.
- `compute_offset_and_return`: Computes an offset from two input words, adds it to four stored pointers, loads a constant into a register, and then returns without touching memory.
- `process_masked_word_targets`: Reads a byte and a flag byte from a global variable indexed by a passed value, then calls a routine from a table of four 16-bit-entry blocks to process each 16-bit word, skipping bits that are clear.
- `mask_four_word_targets`: ANDs four 16-bit words, each reached through its own pointer, with a mask value, clearing every bit not set in the mask.
- `mask_three_words_return_code`: ANDs three 16-bit words, each reached through a pointer, with a mask value, then stores the input byte with a fixed set of bits ORed in.
- `apply_mask_bits_to_planes`: Walks up to four stored bitplane pointers and, for each mask bit that is set, ORs the inverted word into the plane word while for each clear bit it ANDs the word into the plane.
- `allocate_memory_or_fail`: Calls the system memory allocation routine through the exec library vector, and returns minus one when it gives back a null pointer.
- `free_memory_or_fail`: Calls the system memory-free routine through the exec library vector, then returns minus one when the call's result is zero.
- `queue_row_draw_commands`: Writes draw-command records for a span of screen rows, each holding fixed mode and mask words and offsets derived from the vertical position, and takes a separate path with its own offsets when the vertical value is at the edge of the normal range.
- `apply_bit_mask_four_words`: Checks four bits of a mask value in turn, storing an all-ones or all-zero word into a record field and calling a helper for each, then finishing by calling another helper.
- `switch_display_mode_copper`: Swaps two sets of four longs in a global table when the argument is 255, otherwise copies four longs from a source table into it, then waits for a raster position and builds a copper list of bitplane pointer and colour entries before restarting the copper.
- `decode_keyboard_blitter_setup`: Reads a byte from the keyboard serial register, decodes it, and sets up three different sets of blitter playfield control and modulo registers depending on which key was pressed, otherwise it clears some blitter window registers and sets a flag.
- `setup_blitter_registers`: Sets up three different sets of playfield control and blitter register values, each followed by a jump to a different routine.
- `configure_blitter_window_registers`: Writes five blitter control register values, setting window and display dimensions and the playfield control word, then runs straight into the next routine.
- `clear_playfield_control_words`: Clears three playfield control words and a stored flag, then returns before writing five blitter control register values.
- `set_playfield_modulo_forty`: Clears two playfield control words and sets the two playfield modulo words to 40, the row length used by the other blitter routines, then sets a stored flag to one.
- `reset_input_state_from_table`: Sets one stored word to all ones, copies the low byte of a table entry into another stored word, and clears a third stored word.
- `wait_for_raster_line`: Polls the vertical position register until its masked value equals the fixed raster line, then returns its input value unchanged.
- `write_dma_control_register`: Writes the DMA control register, where a set top bit enables the channels named in the write, and calls one other routine.
- `store_or_lookup_record`: If its first input word is zero it stores the second input word into a global variable at offset 523A, calling a helper when that value is also zero.
- `open_ciaa_and_fill_table`: Fills a fifteen-entry table with longwords taken from matching records, zeroing entries that do not match, and on first use opens the ciaa resource through the exec library and returns a flag for whether that call succeeded.
- `close_ciaa_resource_if_open`: If the ciaa resource is marked open, calls the exec library's close routine through its vector and clears the open flag.
- `set_cia_control_byte`: Writes a fixed byte into a CIA control register, then returns.
- `copy_stored_value_to_global`: Copies a byte from a stored table into a global variable, then returns.
- `clear_table_entry_fields`: Walks a fifteen-entry pointer table and zeroes the longword at offset eight of each non-null record, then returns.
- `silence_audio_channels`: Sets the volume register of each of the four audio channels to zero, then clears two stored values, then returns.
- `silence_audio_channels_set_flag`: Sets the volume registers of the four audio channels to zero and stores fifteen into a global variable, then returns.
- `call_and_return_zero`: Calls another routine and then returns the constant zero without using its result.
- `increment_audio_channel_counter`: Increments a 16-bit counter at offset 5232 of a global structure and returns immediately when it has not yet reached 6.
- `run_routine_per_audio_channel`: Sets a counter to 3 and loads the audio channel register block, a helper routine, and a structure table into registers.
- `clamp_structure_value`: Reads a 16-bit value from offset 2 of the structure in a4 and adds it to itself, then compares that doubled value with the 16-bit value at offset 4.
- `sort_two_words`: Compares two words and swaps them so the smaller comes first, then returns.
- `dispatch_byte_by_nibble`: Reads a byte from offset two of the structure in a6, masks its low nibble, and branches to one of three destinations when that nibble equals one, seven, or eight, otherwise it returns.
- `search_table_store_word`: Clears a field, copies a word into another field, and searches a table of words for a match, storing a value based on the low nibble of a byte when found.
- `copy_word_with_signed_offset`: Computes a signed word offset from an input value, selects the sign based on a flag, copies a word from a source address offset by that amount, stores it at an address derived from another register, then returns.
- `store_word_from_d0`: Stores a word from a register into a field at offset four of a structure, then returns.
- `clear_then_branch_forward`: Clears the byte register d4 and then branches forward into the following routine, so it only performs a conditional setup before continuing elsewhere.
- `dispatch_jump_table`: Clears the low byte of a register, loads a jump table address, selects an entry by index, jumps to that entry, and never returns.
- `find_table_entry_by_shifted_value`: Shifts the value in d0 left by one bit and clears d1, then loads the word at offset sixteen of the structure in a6 into d1 and searches a table of words for a matching entry.
- `shift_byte_right_four`: Loads the byte at offset three of the structure in a6 into d0, shifts it right by four bits, and branches back to a shared lookup routine that uses the result as an index.
- `mask_byte_low_nibble`: Loads the byte at offset three of the structure in a6 into d0, keeps only its low nibble, and branches back to a shared lookup routine that uses the result as an index.
- `copy_word_between_structures`: Copies a word from a field at offset sixteen of one structure into a field at offset six of another structure, then returns.
- `start_audio_dma_channels`: Clears a channel-enable word, runs a helper for each of the four audio channels, then writes the enable word with its top bit set to the DMA control register.
- `process_channel_block`: Clears the structure in a6, and when the global flag is set copies a longword from a0 plus d1 into it.
- `clamp_channel_counter`: Copies a longword from a global variable to offset ten of the structure in a6 and stores one at offset fourteen, then adjusts a counter at offset nine by a masked byte and clamps it between zero and sixty-four.
- `clamp_and_process_channel`: Adds a byte from offset three of the structure in a6 to a running counter, clamps the sum at 64, and stores it at offset two of the block in a5.
- `process_channel_and_delay`: Returns immediately when the word at the start of the structure in a6 is zero, otherwise clears the word at a4 and copies the header word into offset sixteen of that structure.
- `copy_channel_sample_fields`: Copies a longword and two words from a structure pointed to by a6 into the output block at a5, then ORs the record's flag word at offset 20 into a running mask at address 5234.
- `fill_bitplanes_by_mask`: Calls a helper to obtain a value, then for each of four bitplanes fills a block of eight thousand longwords with all ones or all zeros, chosen by the corresponding bit of an input mask.
- `wait_for_keyboard_and_pause`: Releases an earlier DOS lock if one is pending, then configures the color and blitter registers and waits for a keyboard serial code, pausing in a nested loop when the code matches a specific value.
- `wait_for_disk_status_clear`: Polls the disk-status bit in the CIA port register until it clears, then runs a thirty-two-thousand-iteration delay loop and writes zero four thousand nine hundred and sixty times to the background colour register.
- `read_and_decompress_input`: Loads the DOS and Exec library vectors, then repeatedly queries a global variable for pending input, storing the returned key code and flag there until a key is available, after which it releases the DOS lock, closes the CIA timer, and copies the decompressed bytes from the packed stream to the destination.
- `load_value_then_jump_back`: Loads the immediate value 4088 into register d0 and calls the delay routine, then loads the value negative one into d0 and jumps back into the middle of another routine.
- `fill_color_register_delay`: Writes the value in d0 to the background colour register in a nested countdown loop of eight outer and one thousand inner iterations, so the repeated writes form a timed colour pulse.
- `delay_then_zero_color_register`: Runs a long countdown loop of thirty-two thousand iterations, then writes zero four thousand nine hundred and sixty times to the background colour register, clearing it after the delay.
- `allocate_buffers_open_libs`: Opens the DOS and graphics libraries, reads the free memory size, and lays out sixteen buffers spaced 8000 bytes apart, storing their pointers in globals.
- `toggle_dma_channel_bits`: Clears bit 7 of a DMA control register, then writes a value of 992 to a custom register at offset 150, then writes a value of 34752 to that same register, and returns.
- `register_interrupt_server_node`: Allocates memory through the exec library vector, returning negative one when the allocation fails.
- `tiny_addq_clr_rts`: Small routine of 3 instructions that adds 1 to (a1), then clears d0, then returns.
- `forbid_and_own_blitter`: Disables multitasking through the exec library and then claims the blitter through the graphics library, using the library base pointers loaded from two global variables.
- `permit_and_disown_blitter`: Releases the blitter through the graphics library and then re-enables multitasking through the exec library, using the library base pointers loaded from two global variables.
- `start_copper_list_from_struct`: Loads the copper list pointer from a field of a stored structure into a global variable, clears the copper restart register, and enables copper DMA by writing the set bit to the DMA control register.
- `call_four_routines_return_zero`: Calls four memory-free routines in sequence, each loading a pointer and a size before returning, and finally clears d0 before returning.

### Another World (1991)(U. S. Gold)[cr CSL](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `install_interrupt_and_copy_ram`: Installs a new interrupt vector at a fixed address and fires trap 0, then copies 39 bytes from a source table into a global variable one byte at a time, and finally returns.
- `copy_struct_fields_and_rte`: Sets two high bits in a global flag, then copies two longwords from a structure into its fields at offsets 60 and 64, clears d0, and returns to the caller via the interrupt return instruction.
- `setup_interrupt_and_fill_ram`: Disables interrupts and DMA, saves the current interrupt vector, installs a new handler, and fills several RAM regions by looping while shifting, adding, and copying words.
- `setup_and_dispatch_interrupt`: Saves the registers, configures the copper and interrupt registers, calls two helper routines, sets an interrupt mask, restores the registers, and returns from the interrupt.
- `tiny_ori`: Small routine of 1 instruction that merges 64 into d0.
- `dispatch_by_incremented_counter`: Increments a global word and branches away when it reaches one hundred and sixty, otherwise dispatches to a routine based on the current value of that global.
- `increment_global_then_call`: Tests a global word and branches away if it is nonzero, otherwise increments the same global, returns early if it reaches fifty-six, calls another routine, and returns early if the global is no longer fifty-six.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_blitter_and_update_bitmap_2`: Sets a global flag, points a copper list at a global address and clears the blitter fill register, then configures several blitter and copper registers.
- `configure_blitter_and_update_bitmap`: Sets bits in a global flag, disables DMA, configures several blitter and copper registers, waits for a vertical retrace flag to clear, sets one bit in a bitmap table, decrements a global counter, and sets two globals when it reaches zero.
- `build_data_structure`: Initializes a data structure by computing a running sum from a table of words and filling a second area with the cumulative values, then populates a following block with thirteen function pointers and performs clamping arithmetic on the sequence.
- `clear_audio_and_dma`: Clears the four audio volume registers, clears a bit in a global flag, writes a constant to another global, disables copper and DMA channels, and copies a value from a table into a global variable.
- `apply_audio_and_input_commands`: Decrements a global counter and, while it is positive, reads a command from a table and applies it to audio and input state: it adjusts a channel volume and a pitch value, writes the new volume to the audio channel register, and dispatches to a handler that reads the next input byte from a table.
- `jump_to_dispatch_table`: Jumps to a dispatch table via a pointer in a3 and never returns to the following instructions.
- `store_byte_at_offset_minus18`: Stores the value in register d4 at offset 18 below the base of the structure in a2, then branches to a dispatch routine and never returns.
- `store_byte_at_offset4`: Stores the value in register d6 at offset 4 of the structure in a6, then branches to a dispatch routine and never returns.
- `write_constant_and_store_byte`: Writes the constant 63 to offset 5 of the structure in a6, stores the value in register d4 at offset 9 below that base, then branches to a dispatch routine and never returns.
- `store_word_at_offset2`: Stores the value in register d4 at offset 2 of the structure in a6, then branches to a dispatch routine and never returns.
- `process_input_via_cia_and_dispatch`: Sets bit 0 of the CIA parallel register A in register a2 and then processes the next input byte from the table in a6, dispatching to a handler routine based on the byte value.
- `clamp_value_to_maximum`: Subtracts the word in d2 from the value at offset 12 of the structure in a0, clamps the result to a maximum of 113 by reloading that constant when the value becomes negative, then stores the clamped value at offset 6 of the structure in a4.
- `clamp_value_to_upper`: Adds the word in d2 to the value at offset 12 of the structure in a0, clamps the result to a maximum of 856 by reloading that constant when the value becomes negative, then stores the clamped value at offset 6 of the structure in a4.
- `index_byte_table_and_dispatch`: Reads the word at offset 2 of the structure in a6, subtracts the word at offset 16, negates the result, and uses it to index a byte table.
- `compute_and_clamp_dispatch_index`: Reads a word at offset 10 of the structure in a0, masks its low nibble, doubles that value, adds a word at offset 14, and if the result is 72 or less stores it at offset 6 of the structure in a4, otherwise stores 72 there instead.
- `copy_value_and_jump`: Copies the word at offset 12 of the structure in a0 to offset 6 of the structure in a4, then jumps to the shared code at offset 4c4.
- `store_word_if_nonzero`: Reads a word at offset 10 of the structure in a0, and if that value is zero it branches away, otherwise it stores that value at offset 30 of the same structure.
- `compute_indexed_value`: Derives a table index from a masked value at offset sixteen of the structure in a0, multiplies it by another masked value, and adds the result to a counter at offset twelve, negating it when the byte at offset seventeen is negative.
- `update_index_and_check`: Subtracts d3 from d2, stores the result at offset 6 of the structure in a4, derives a table index by shifting the value at offset 30 of the structure in a0 right by two bits and masking it, adds that index to the byte at offset 17 of the same structure, and returns when the byte in d1 is not 20.
- `accumulate_and_clamp_byte`: Adds a word at offset ten of the structure in a0 to the byte at offset seven, clamps the byte at offset six at sixty-four when it goes negative, stores it at offset eight of the structure in a4, then branches to shared code.
- `store_value_in_structures`: Stores the word in d6 at offset 6 of the structure in a0 and at offset 8 of the structure in a4, then jumps to the shared code at offset 4c4.
- `copy_clamped_value_between_offsets`: Reads the word at offset 26 of the structure in a0 and returns immediately when it is zero.
- `add_clamp_and_copy_value`: Reads the word at offset 26 and returns immediately when it is zero, then reads the word at offset 24 and branches away when it is nonzero.
- `subtract_store_and_clamp`: Subtracts the value in register d3 from a word at offset 12 of the structure in a0, and if the result is negative it branches away, otherwise it stores register d2 at offset 12, register d6 at offset 26, and that word at offset 6 of the structure in a4.
- `configure_cia_dma_interrupts`: Stores a constant into a global variable, records its own return address in another global, and saves a pointer to a RAM table in a third global.
- `write_audio_channel_registers`: Reads eight 16-bit values from a table and writes them into the four audio channel data and length registers, then writes a 16-bit constant into the audio combine register and returns.
