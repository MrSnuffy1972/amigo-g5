# Game 1758 (another world (1991)(u. s. gold)(fr)[cr company](disk 1 of 2)[tr en]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01758). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901520 top-level bytes in 14 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 116 of 116 named (0 author symbols), 116 of 116 described (4 by rule, 5 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1738 of 1738 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 565294 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3419 of 3419 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 144026 |
| code | 11984 |
| data | 658447 |
| padding | 3607 |
| unused | 83456 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 1 of 2)[tr En].adf:intro.comp | 2 | 233 | 8 |
| Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 1 of 2)[tr En].adf:another | 1 | 3087 | 106 |
| Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 1 of 2)[tr En].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 89 | 1 |
| Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 1 of 2)[tr En].adf:bank0D (raw code file, load address unknown) | 1 | 0 | 0 |
| Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 1 of 2)[tr En].adf:bank01 (raw code file, load address unknown) | 1 | 0 | 0 |
| Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 1 of 2)[tr En].adf:bank02 (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 74 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- another world (1991)(u. s. gold)(fr)[cr company](disk 1 of 2)[tr en]/Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 1 of 2)[tr En].adf:bank0D: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- another world (1991)(u. s. gold)(fr)[cr company](disk 1 of 2)[tr en]/Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 1 of 2)[tr En].adf:bank01: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- another world (1991)(u. s. gold)(fr)[cr company](disk 1 of 2)[tr en]/Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 1 of 2)[tr En].adf:bank02: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 3 never-loaded region(s), 565294 bytes, recorded as UNVERIFIED DATA (under 1% of bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 1 of 2)[tr En].adf:intro.comp

## Copy protection found

- `stub_undecoded_library_copper_setup` (runtime segment an address): The decompiler could not finish this routine, so only its listed facts are known. Kind: loader/integrity check stub. Pass path: The routine likely performs a hardware-level verification (given the name 'undecoded_library_copper_setup' and the disk hardware context) and the pass path would be the continuation of the execution flow immediately following the routine's .

## Functions


### Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 1 of 2)[tr En].adf:intro.comp

- `program_entry_alloc_and_run`: Program entry point of the intro executable.
- `busy_wait_decrement_counter`: Spins in a loop decrementing a 16-bit count until it wraps, forming a delay.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `read_stream_longword_then_check`: Reads a longword from the data stream before its pointer, runs bit tests, and calls a shifted-flag helper, returning early when that check fails.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `shift_bits_from_longword_stream`: Shifts bits out of a longword stream one count at a time, reloading the previous longword when the buffer empties, and derives a flag from the bit pattern.
- `decode_tiny_routine_pending`: A three-instruction routine reached through an address-taken reference from one unnamed caller.

### Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 1 of 2)[tr En].adf:another

- `startup_setup_timing_and_calls`: Program entry for this executable.
- `dispatch_handler_from_table`: Entered through a handler pointer held in data.
- `handle_entry_with_three_calls`: Handler reached through a data pointer that runs its logic through three unnamed calls, operating on the data and address registers it is given.
- `process_word_entry_list`: Processes entries in a 16-bit list through pointer arithmetic and byte-level tests, calling four unnamed routines.
- `interpret_byte_command_stream`: Interprets bytes from a command stream, branching on command codes that advance a count-and-value pair and update position pointers.
- `execute_bytecode_instructions`: Iterates through a list of variable-length instructions, performing arithmetic, comparisons, and memory operations on values stored in a table.
- `parse_bytecode_instruction_stream`: Iterates through a list of encoded instructions, interpreting opcodes to perform arithmetic operations, comparisons, and memory assignments on values stored in a table.
- `execute_bytecode_instructions_2`: Parses and executes a custom bytecode stream to manipulate values within a data table.
- `execute_bytecode_instructions_3`: Interprets a stream of custom bytecode instructions to perform arithmetic operations, comparisons, and memory manipulations on values stored in a table.
- `run_entry_processing_chain`: Runs a long chain of calls to eleven unnamed routines over a list of entries.
- `compute_packed_return_value`: Combines several register inputs with shifts and masks and returns a 32-bit result, calling four unnamed routines along the way.
- `walk_counted_pointer_list`: Walks a counted list of 16-bit entries from a base pointer, stepping by fixed strides and calling one unnamed routine on each entry until the counter runs out.
- `index_twenty_byte_record_table`: Returns the address of an entry in a 20-byte-per-record global table, indexed by the low 16 bits of the data register.
- `search_word_table_for_key`: Scans a global word table until it finds a terminator or a matching key pair, then calls one unnamed routine with the match.
- `copy_bounded_byte_block`: Checks coordinates against fixed bounds, then copies a bounded block of bytes between two pointers.
- `decode_depacker_symbols`: Part of a depacker: reads bit codes through the bit reader and writes output bytes through its pointers.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `shift_bit_buffer_by_count`: Shifts a 32-bit bit buffer right by the count in a data register, refilling it from the previous longword with its top bit set whenever it empties.
- `process_file_bank_record`: Works with text references to a file and bank field and calls three unnamed routines in sequence.
- `format_word_as_text_digits`: Converts a 16-bit value into four ASCII digit characters, mapping each nibble to a digit or letter and returning the packed text.
- `compare_range_values_step_table`: Compares two 32-bit range values and steps a byte pointer through a table, calling one unnamed routine.
- `mark_entries_until_budget_spent`: Marks entries of a linked list by storing a code and OR-ing a bit, subtracting a length field from a running budget until it goes negative, then scanning a global table.
- `scan_records_until_end_marker`: Walks a global table of 20-byte records until it reaches an end marker, calling three unnamed routines per record.
- `subtract_length_from_global_counter`: Subtracts a length value from a global counter in a loop that reads bytes through a pointer, calling one unnamed routine.
- `return_first_stack_argument`: Returns its first stack argument unchanged, with no other work.
- `copy_longword_pairs_between_buffers`: Copies longword pairs between two buffers and calls two unnamed routines per step.
- `copy_longword_block_once`: Copies a small longword block between buffers under a count and calls one unnamed routine.
- `fill_entries_with_max_sentinel`: Sets a 16-bit value to the maximum positive sentinel and copies entries between pointers in a loop, calling one unnamed routine.
- `return_first_stack_argument_2`: Returns its first stack argument unchanged, with no other work.
- `sync_to_video_beam_position`: Reads the video beam position register to pace work, calling five unnamed routines.
- `update_matching_twenty_byte_records`: Walks a 20-byte record table, testing a type byte against a small range and updating the matching records.
- `clear_twenty_byte_records`: Walks a global 20-byte record table and zeroes the first byte of each record for a counted number of entries.
- `count_table_entries_until_sentinel`: Counts entries in a 20-byte record table until a sentinel value is found, and returns the count.
- `setup_blitter_copy_registers`: Loads blitter source, mask and modulo registers from its arguments and programs a masked copy before starting the blit.
- `run_blitter_row_loop`: Loops over rows, calling three unnamed routines per row with 16-bit position and step values from its arguments.
- `stub_return_only`: Contains only a return, with no calls, memory access or hardware register use.
- `copy_table_rows_with_step`: Loops over 16-bit row data, copying values into destination registers with step values and calling one unnamed routine.
- `decode_packed_word_entries`: Processes a sequence of packed 16-bit entries in a loop using shifts and masks, calling two unnamed routines.
- `run_blit_with_masks`: Programs many blitter registers, including control, size, pointer and modulo registers, and reads the DMA control register before starting the blit.
- `store_blit_request_then_wait`: Stores a set of blitter request values, including two fixed values, into a record addressed by the base register.
- `wait_for_blitter_idle`: Loops reading the DMA status register and tests the blitter busy flag until it clears, then returns.
- `decode_joystick_direction_flags`: Reads a stored joystick input word and derives horizontal and vertical direction flags from its bits, using an exclusive-or of adjacent bits for the vertical direction.
- `copy_four_longwords_to_globals`: Calls a subroutine first, then copies four consecutive 32-bit values from the record at the incoming pointer into four global variables.
- `scale_small_value_by_sixteen`: Returns the input word unchanged when it exceeds three.
- `return_without_effect`: Contains only a return, so it reads and writes no memory or hardware registers and acts as a no-op.
- `draw_clipped_sprite_planes`: Checks that a sprite record lies within the visible screen, clipping its top and bottom rows as needed.
- `mask_four_word_targets`: ANDs a 16-bit mask value into each of four words, each reached through its own address register, which clears every bit not set in the mask.
- `mask_three_words_return_code`: ANDs a 16-bit mask value into three words reached through address registers, then returns the incoming byte with a fixed pattern of bits set.
- `combine_mask_bits_into_planes`: Works through up to four stored plane pointers, one per mask bit.
- `allocate_memory_or_fail`: Calls the system library's memory allocation function through its library vector.
- `free_memory_or_fail`: Calls the system library's memory release function through its library vector, then tests the returned value against zero and returns minus one when it is zero.
- `queue_row_draw_commands`: Chooses one of two paths depending on a signed input value.
- `apply_bit_mask_four_words`: Runs four passes over the bits of a mask value.
- `switch_display_mode_copper`: A sentinel code skips the stored-data step, another code swaps four long words, and any other code copies four words from the given record.
- `handle_keyboard_scancode`: Decodes a raw keyboard serial byte by rotating its inverted bits, then branches on the key code.
- `program_display_window_mode_b`: Writes fixed values to the display window start and stop registers, the data fetch start and stop registers and the bitplane control register.
- `configure_display_timing`: Sets the display parameters by writing values to the bitplane control, display enable, and vertical synchronization registers.
- `clear_four_state_words`: Writes zero to four words in the state record addressed by the base register and clears a stored flag.
- `reset_two_words_set_pair`: Clears two words in the state record, sets two other words to forty, and sets a stored flag to one.
- `reset_input_state_from_table`: Sets one stored word to all ones, loads the low byte of a table entry into a second stored value, and clears a third stored value.
- `wait_for_raster_line`: Loops reading the vertical position register until the raster position reaches a fixed line, then returns its input value unchanged.
- `write_dma_control_register`: The decompiler could not finish this routine, so only its listed facts are known: it writes the DMA control register, where setting the top bit enables the channels named in the write, and it calls one other routine.
- `lookup_record_when_code_matches`: When the first argument is zero, it stores the second argument into a global variable if that argument is non-zero, and otherwise calls a routine.
- `open_ciaa_and_fill_table`: On first use it opens the ciaa resource through the exec library and calls a second library function, saving that result.
- `close_ciaa_resource_if_open`: When the open flag is set, it calls a library close function through a stored pointer and clears the flag.
- `load_global_from_fixed_cell`: Writes a constant into a global variable, then overwrites it with the value read from a fixed memory cell.
- `copy_stored_value_to_global`: Copies a value read from a fixed memory cell into a global variable.
- `clear_field_in_table_records`: Walks a table of fifteen record pointers and, for each non-null pointer, writes zero to the longword at offset eight within that record.
- `silence_audio_channels`: Writes zero to the volume register of each of the four audio channels, then clears two stored values.
- `silence_audio_channels_store_fifteen`: Writes zero to the volume register of each of the four audio channels, then stores the value fifteen in a global variable.
- `call_then_return_zero`: Calls one other routine, then returns zero as its result.
- `cycle_audio_channel_updates`: Increments a stored counter, and until it reaches six it calls a routine once for each of the four audio channel register blocks, passing the channel index.
- `run_routine_per_audio_channel`: Calls a routine once for each of the four audio channel register blocks, counting the channel index down from three to zero, and then returns its stack argument unchanged.
- `advance_channel_effect_state`: If the stored step value is non-zero, it adds the step to a stored position, clamps the result to a limit chosen by the step's sign, and stores it.
- `advance_position_with_lower_limit`: Adds an input word to a stored position and raises the result to a stored minimum when it is at or below that minimum.
- `set_channel_effect_parameters`: For effect code one it calls a routine through a jump table indexed by a stored value, and for codes seven and eight it stores a signed low-nibble parameter, the record's key word, and a value found by searching a fixed table for the key.
- `store_key_with_table_lookup`: Stores a low-nibble parameter from a record and the record's key word, then searches a fixed table for the key.
- `copy_word_with_signed_offset`: Copies one word from a source address offset by a signed index, scaled to word size, into a field of the destination record.
- `store_input_word_in_record`: Writes the incoming word into offset four of the record addressed by the destination base register.
- `store_signed_key_with_lookup`: Stores a low-nibble parameter from a record, negated, together with the record's key word.
- `dispatch_routine_by_index`: Calls a routine chosen from a table of routine pointers by a stored index, which is decremented by one and scaled by four before use.
- `lookup_table_value_by_key`: Finds the record's key word in a fixed table, then writes the entry at the incoming index counted from that match into the output field.
- `lookup_table_high_nibble_offset`: Finds the record's key word in a fixed table, then writes the entry at an offset from that match given by the high nibble of a record byte into the output field.
- `lookup_table_low_nibble_offset`: Finds the record's key word in a fixed table, then writes the entry at an offset from that match given by the low nibble of a record byte into the output field.
- `copy_record_word_to_output`: Copies the word at offset sixteen of the record into the output field.
- `start_audio_dma_channels`: Clears an enable word, calls a routine for each of the four audio channels, then writes the enable word with the DMA enable bit set.
- `process_channel_sequence_entry`: Processes one entry of a channel's sequence record, which supplies a value word, a data pointer and a count.
- `update_channel_volume_and_flags`: Adjusts a channel's volume by the entry's parameter according to its command nibble, adding with an upper clamp or subtracting with a lower clamp.
- `add_entry_volume_and_flags`: Sets the channel volume to a base word plus a record byte, clamped at the upper limit, then applies the same special-value handling and running-mask update as the other entry routines.
- `store_active_channel_entry`: For a non-zero entry, it waits briefly and then either silences the channel for the special silence value or copies the entry's pointer, length and value into the channel's output block.
- `copy_channel_entry_fields`: Copies a longword and two words from a channel record into the output block, then ORs the record's flag word into a running mask.
- `fill_bitplanes_by_mask`: Calls a routine first, then for each of four bitplane pointers fills a block of 8000 longwords with all ones or all zeros according to the matching bit of the input mask.
- `lock_df0_volume_as_current`: Releases any previously held DOS lock, waits for a keyboard code, pauses briefly, then locks the DF0 volume and makes it the current directory.
- `wait_for_disk_status_clear`: Loops reading the status port until its disk status bit clears, then calls one other routine.
- `read_file_toggling_blitter_ownership`: Releases the blitter, permits multitasking, and loops opening a file when no handle is held or otherwise seeking and reading from it, closing and reopening the file when the requested value differs from the stored one.
- `forbid_and_own_blitter`: Calls a routine, forbids multitasking and claims the blitter, then returns minus one.
- `store_word_in_delay_loops`: Stores the incoming word into a global location repeatedly within nested countdown loops, which acts as a timed delay.
- `delay_then_zero_global`: Runs a long countdown loop as a delay, then writes zero to one global location once per step of a second loop of 4096 steps.
- `open_libraries_and_layout_buffers`: Opens the DOS and graphics libraries by name and reads the available memory size.
- `toggle_dma_channel_bits`: Calls a routine, writes a DMA control value that clears one group of channel bits, calls another routine, and then writes a second value that sets a different group with the enable bit.
- `register_interrupt_server_node`: Allocates an interrupt node through a called routine, sets its data pointer to a global, its priority to one hundred and its type to two, and stores a handler routine's address.
- `tiny_addq_clr_rts`: Small routine of 3 instructions that adds 1 to (a1), then clears d0, then returns.
- `forbid_and_own_blitter_2`: Disables multitasking through the exec library, then claims the blitter through the graphics library.
- `permit_and_disown_blitter`: Releases the blitter through the graphics library, then re-enables multitasking through the exec library.
- `start_copper_list_from_struct`: Loads the copper list pointer from a field of a stored structure, strobes the copper jump register, and enables copper DMA through the DMA control register.
- `call_four_routines_return_zero`: Calls four routines in sequence and returns zero.

### Another World (1991)(U. S. Gold)(Fr)[cr Company](Disk 1 of 2)[tr En].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_undecoded_library_copper_setup`: The decompiler could not finish this routine, so only its listed facts are known.
