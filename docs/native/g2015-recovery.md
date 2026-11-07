# Game 2015 (populous (1989)(electronic arts)[cr vf][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02015). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (905616 top-level bytes in 38 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 15 raw-word lines |
| Functions named and described | pass, 664 of 664 named (0 author symbols), 664 of 664 described (87 by rule, 148 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1147 of 1147 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 131072 bytes, 85 functions |
| Independent decoder cross-check (Ghidra against capstone) | 10122 of 10891 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 623075 |
| code | 81332 |
| data | 190120 |
| padding | 11089 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| endcli | 2 | 117 | 2 |
| echo | 2 | 117 | 2 |
| loadwb | 12 | 536 | 30 |
| Disk-Validator | 1 | 386 | 1 |
| Populous (1989)(Electronic Arts)[cr VF][a].adf:POP | 2 | 83 | 3 |
| Populous (1989)(Electronic Arts)[cr VF][a].adf:populous | 3 | 4314 | 145 |
| audio.device | 1 | 448 | 42 |
| Populous (1989)(Electronic Arts)[cr VF][a].adf:populous.prg | 2 | 83 | 3 |
| icon.library | 10 | 1375 | 109 |
| translator.library | 2 | 490 | 50 |
| info.library | 25 | 2597 | 167 |
| version.library | 2 | 41 | 7 |
| Populous (1989)(Electronic Arts)[cr VF][a].adf:bootblock | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 297 | 17 |
| Populous (1989)(Electronic Arts)[cr VF][a].adf:DAD (raw code file, load address unknown) | 1 | 12830 | 85 |

## Cases handled

- runtime segment an address: 266 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- populous (1989)(electronic arts)[cr vf][a]/Populous (1989)(Electronic Arts)[cr VF][a].adf:DAD: raw code file no run loaded verbatim: recovered position-independently (load address unknown)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `write_table_value_to_adkcon` (audio.device): Calls two routines to set up the argument, stores a fixed longword in an indexed slot, and writes a value from a byte table into the audio control register. Kind: checksum or data integrity trap. Pass path: the code continues to the next iteration of the loop (addq.w #an address, d3) if the value matches, or proceeds to the next track/data block.

## Functions


### endcli

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_words_to_buffer`: Copies sixteen consecutive 32-bit words from a fixed offset in the base register into a caller-supplied buffer, if the buffer pointer is non-null.

### echo

- `install_program_handler_record`: Finds the largest value stored in a linked list of entries and writes it into a new record, then fills that record with incrementing words.
- `copy_data_block`: Copies a block of sixteen bytes from a source pointer to a destination pointer and then calls another routine.

### loadwb

- `parse_command_line_and_run`: Splits the command line string into a table of argument pointers, handling quoted arguments and star escapes for newline and escape characters.
- `release_startup_resources`: Stores the parameter into a slot of the state structure reached through a global library pointer.
- `start_workbench_tool`: Opens the required system libraries and loads a tool entry, decoding global strings by XOR with a fixed byte before use.
- `compare_null_terminated_strings`: Compares two zero-terminated strings byte by byte.
- `build_resource_list_from_table`: Walks a chain of table entries, allocating an eight-byte node for each and storing in it the result of a lookup on the entry's value.
- `call_library_vector_96_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_366_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `match_name_and_pad_field`: Compares a global zero-terminated name against the text at the buffer pointer.
- `write_dot_and_padding`: Writes a period followed by four spaces into five consecutive bytes of the buffer starting at offset seventeen.
- `check_name_prefix_match`: Walks the global zero-terminated name and the buffer pointer together, advancing both, until the name ends or the bytes differ.
- `return_without_action`: Contains only a return, so calling it performs no work and passes control straight back to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `forward_call_with_stack_slot`: Forwards its two parameters to another routine together with the address of a stack slot in the current frame, then returns.
- `format_buffer_and_forward_text`: Calls a routine through a global pointer to fill a 140-byte stack buffer, then measures the zero-terminated text in it.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Disk-Validator

- `scan_disk_block_bitmap`: Writes a one- or two-digit count into a text field, clears a table of block entries, then walks a bit map of disk blocks, comparing flags of consecutive blocks and invoking routines for each change.

### Populous (1989)(Electronic Arts)[cr VF][a].adf:POP

- `intro_bytekiller_depack`: Cracktro start: ByteKiller depacker that decodes the packed stream after it backwards into 50000 hex onwards, showing progress on COLOR00, checks the checksum and jumps to the unpacked cracktro at 50000 hex.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Populous (1989)(Electronic Arts)[cr VF][a].adf:populous

- `clear_state_block_and_dispatch`: Calls a routine, zeroes a block of roughly seven thousand words addressed through the base register, and stores a process pointer and a global pointer.
- `allocate_buffers_and_init_state`: Allocates several memory blocks of fixed sizes and stores the returned handles in globals, after referencing the text demo.pic.
- `start_copper_list_from_record`: Reads a field from a record reached through a global base pointer and passes it to the routine that starts a copper list, which points the display co-processor at that list.
- `run_fixed_setup_calls`: Makes four setup calls with fixed arguments taken from globals and constants, touches the interrupt enable register, and writes a constant to a global variable.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `allocate_block_and_fill_slots`: Allocates a block of about eleven hundred bytes and stores its address in a global.
- `store_constant_and_call`: Stores a fixed decimal constant into a field of a global-relative record and then calls another routine with no arguments.
- `call_with_global_pointer`: Loads a pointer from a global table and calls another routine with that pointer and the fixed size of 1152, the size of the block allocated earlier.
- `call_two_global_pointers`: Passes two pointers read from global variables, one after the other, to another routine.
- `forward_argument_to_library_call`: Passes a global value and the low word of its argument to another routine, which invokes a library function through a vector.
- `forward_call_without_arguments`: Calls another routine with no arguments and returns without any further action.
- `lookup_pair_and_dispatch`: Searches a two-entry table in globals for the value passed as a parameter.
- `swap_globals_and_call`: Exchanges the values of two global variables and then calls another routine with the swapped values as arguments.
- `open_graphics_and_intuition`: Stores the base returned by an initial call and marks a field in it, then requests two further library calls referenced by the texts graphics.library and intuition.library.
- `wait_for_status_then_call`: Repeatedly calls a routine until it returns one of the values one, two or three, then passes a global pointer and a frame-relative address to another routine.
- `load_pic_and_wait_sync`: It then sets a global flag based on comparisons of global shorts and returns the flag.
- `init_callbacks_and_run_loop`: Stores two routine addresses and three constants in globals, then clears a word table and initializes sixteen slot records.
- `interpret_script_opcodes`: Reads bytes from a script buffer one at a time.
- `return_immediately`: Contains only a return, so calling it performs no work.
- `init_slots_and_animate`: Sets up eighteen slot values, each masked to a small range, then repeatedly repositions the slots at offsets computed from the stored values.
- `fill_two_slot_arrays`: For each of the given entries, stores one value reduced modulo thirteen and offset by 232 in one array, and one value reduced modulo nine in a second array.
- `advance_slot_arrays`: Walks the given number of entries and updates each paired counter and value.
- `run_demo_pic_loop`: Makes a series of setup calls, including one with the fixed value 20072, then calls routines repeatedly while a call returns nonzero and the stop flags stay clear.
- `setup_four_plane_pointers`: Computes four pairs of addresses from a base, stepping the first by three hundred and the second by thirty-two thousand each time, and passes each pair to another routine.
- `read_fixed_block_into_buffer`: Obtains a handle through a call with a fixed code and returns minus one when it is zero.
- `load_asset_data_files`: Opens handles and reads fixed-size blocks into buffers held in globals, referencing the texts font.dat, land0 and sprites0.dat.
- `read_data_block_with_header`: Obtains a handle through a call with a fixed code, and when it is valid reads forty thousand bytes into the caller's buffer and sixty-four bytes into a second buffer.
- `draw_shifted_words_to_planes`: Returns early when the vertical position is outside the playfield.
- `draw_shifted_tile_block`: Checks the row against the playfield bounds, then for sixteen rows reads source words from a fixed table address indexed by the parameter.
- `forward_call_keep_registers`: Calls another routine and returns whatever the two data registers hold, passing the register-pair result through unchanged.
- `draw_masked_block_to_planes`: Checks the row against the playfield bounds, then for up to twenty-four rows combines words from a block in globals into four bitplanes.
- `rotate_entry_fields_in_table`: Walks 1680 records of twenty bytes in a global table and rearranges their words and bytes by shifting and copying fields between positions.
- `shift_byte_buffer_left_three`: Shifts each of one hundred bytes in a fixed buffer left by three bits, in place.
- `iterate_ten_by_ten_grid`: Loops over a ten-by-ten grid of cells and calls another routine for each cell, with coordinates derived from the cell's row and column, a global byte offset and a value from a table.
- `forward_call_return_registers`: Calls another routine and returns the values left in the two data registers as one 64-bit result.
- `store_global_address_pointer`: Stores the address of a global data area into another global pointer variable and has no other effect.
- `render_string_into_bitplanes`: Clears a table and a block of words, measures the zero-terminated text held in a global pointer, and draws each character as eight rows of glyph bytes from a font table into five planes spaced by three hundred twenty bytes.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `install_interrupt_handler_and_enable`: Obtains a base through a library call and, when it is valid, writes a mask that disables all interrupts, stores a handler routine and then writes a mask that re-enables selected interrupts.
- `initialize_callback_structure`: It retrieves a pointer from a global table and, if valid, initializes several global variables with a constant and calls another routine.
- `transfer_to_interrupt_setup`: Single-instruction entry that transfers control to another routine.
- `access_callback_pointer`: This routine is an entry point used to retrieve a pointer from a global table.
- `access_callback_offset`: This routine is an entry point used to retrieve a pointer from a global table at a specific offset.
- `load_popsam_sample_file`: Opens the dos library and the file named popsam, allocates memory for it and reads it in.
- `nop_padding_routine`: This routine is empty and performs no operations.
- `clear_flag_byte_and_return`: Clears one byte at a fixed offset from the pointer argument and returns the eight-byte value passed on the stack unchanged.
- `restore_saved_interrupt_masks`: Writes a mask that disables all interrupts, calls a routine through a function pointer, then restores three saved register values.
- `dispatch_and_clear_flag`: Compares a one-byte argument with the values ten and one and calls a routine for each match.
- `clear_flag_after_conditional_call`: Calls a routine only when the one-byte argument equals one, then clears a flag byte at a fixed offset from the object pointer and returns the eight-byte value passed on the stack.
- `clear_flag_byte_and_return_2`: Clears one byte at a fixed offset from the object pointer and returns the eight-byte value passed on the stack unchanged.
- `clear_byte_at_offset_3`: Stores a zero byte at a fixed offset from a pointer held in a register.
- `clear_byte_at_offset_2`: Stores a zero byte at a fixed offset from a pointer held in a register and returns the original stack values.
- `clear_byte_at_offset`: It stores zero at a fixed offset from a pointer argument and returns the original stack values.
- `acknowledge_status_bit_interrupt`: Tests one status bit in a global word and, when it is set, writes all ones to a fixed byte address, stores masked values back into globals and calls another routine.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `build_period_tables_from_records`: Reads eight six-byte records from a table, storing for each a link value, a halved size and a period computed by dividing a fixed clock constant by the record's word.
- `build_dispatch_tables`: The routine builds dispatch tables by reading values from a global variable area and writing computed pointers and constants into memory, then initializes additional tables and global variables.
- `clear_audio_control_state`: Clears one global byte and writes two constant values to a global control word.
- `update_sound_channels`: When its enable byte is set, steps through four sound channels from the last to the first.
- `step_sound_channel_sequence`: Advances one channel's sequence by decoding script bytes that start a sample, set a flag, jump to a loop position or change the envelope.
- `restart_sound_channel_sequence`: Resets a channel's state bytes and reads its sequence from the start.
- `start_sound_channel_sequence`: Takes a table index from the sequence, points the channel's envelope at the matching entry, and then decodes the sequence.
- `play_sound_channel_sequence`: Begins a channel's sequence from a stored offset and feeds the same opcode decoder, writing the hardware channel registers and updating the sequence position.
- `enter_sound_sequencer_loop`: Single-instruction entry that jumps into the shared sound sequencer loop, which decodes per-channel script bytes and writes the audio channel registers.
- `enter_sound_sequencer_loop_2`: Stores a control byte in a channel record, marks the channel, and then joins the shared sound sequencer loop, which decodes script bytes and writes the audio channel registers.
- `read_sequence_pointer_and_decode`: Loads the next sequence position for a channel from a stored pointer table and continues into the shared sound sequencer decoder, which writes the audio registers.
- `start_sample_on_channel`: Writes the sample address, length, period and volume for a channel to its audio registers, sets the matching DMA bit, and then runs the sequencer loop.
- `apply_envelope_to_channel`: Each tick, adds a stored slope to the channel volume when a flag is set, scales a period value by a table entry, and writes the volume register.
- `clear_channel_buffer_and_sequence`: Entry that clears one byte of a channel's stored buffer at the current position and records the position, then joins the shared sound sequencer loop that writes the audio channel registers.
- `clear_buffer_to_limit`: Steps the buffer position down by one and, while it is still above a stored limit, clears a flag bit and writes a zero byte into the buffer at that position.
- `write_buffer_byte_and_sequence`: Writes the value sixty into the channel buffer at the current position, records the position, and joins the shared sound sequencer loop that writes the audio registers.
- `blend_bitplanes_with_mask_cycle`: Iterates over sixty-four rows of four bitplane words, choosing a mask word from a cycle of three globals on each step.
- `blend_source_row_into_planes`: Walks 160 source words in order, combining each with a mask taken from a three-entry cycle in globals.
- `store_stack_word_and_call`: Copies a word passed on the stack into a global variable and then calls another routine.
- `forward_call_and_return`: Calls another routine and returns.
- `update_god_state_entry_2`: The routine checks a global variable and, if set, updates several global variables with scaled values derived from a global variable and a table, then calls another routine and updates a global variable based on a computed byte.
- `save_mouse_counter_bytes`: Splits the mouse counter word into its low and high bytes and saves them in globals for the next movement update.
- `save_handler_and_enable_interrupts`: Copies a global word into a stored routine slot and a memory cell, then writes an enable mask with the enable flag bit set.
- `restore_pending_interrupt_mask`: When a stored value is nonzero, writes a mask that disables all interrupts, writes the stored value back, then writes it again with the enable flag bit set.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `build_copper_list`: Fills a memory block with a copper list.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `set_copper_location_and_wait`: Stores its parameter in a global slot, writes a value four greater to the copper location register, then polls the interrupt request read-back until the vertical blank bit is set.
- `wait_for_vertical_blank`: Clears the vertical blank request by writing its bit to the interrupt request register, then polls the read-back until the bit is set again.
- `fill_copper_color_entries`: Scans a register-value list until it reaches the marker word for the first colour register, then writes a counted run of values from a second array into the following register slots, stepping by two words each time.
- `process_undecoded_routine_body`: The decompiler produced no pseudo-code for this twenty-one instruction routine, so its behaviour cannot be recovered from the evidence.
- `set_entry_position_control_bytes`: Computes start and stop values from its two position arguments, adjusting the vertical start by an offset that depends on a global flag.
- `decode_keyboard_serial_byte`: Reads keyboard codes from the keyboard controller's serial data register and tracks a flag for the key state.
- `copy_eight_thousand_longwords`: Copies eight thousand 32-bit values from a source block to a destination block, one at a time.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `decrement_nibble_fields_until_zero`: Copies thirty-two words of input into a stack buffer and repeatedly decrements each nonzero four-bit field in each word, looping while any field changed.
- `call_then_forward_with_args`: Calls one routine with no arguments, then calls a second routine with its two parameters and returns the second routine's result.
- `forward_then_conditional_call`: Passes its parameter to one routine, then calls a second routine only when a stored word in the global frame is nonzero.
- `advance_seed_word`: Adds the constant twelve thousand three hundred forty-five to a value returned by a routine, stores the sum in a global longword, and returns its high word with the top bit cleared.
- `store_high_word_argument`: Stores the upper sixteen bits of its argument into a global longword.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning.
- `allocate_tagged_record_block`: Allocates a record sized from a global index, and returns early when the allocation fails.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `allocate_and_build_name_string`: Chooses a base string, either a global default or one taken from the object's table entry, and allocates a buffer sized to the base length plus the parameter.
- `append_string_bounded`: Appends the second zero-terminated string to the end of the first, copying at most about thirty-two thousand characters and terminating the result.
- `append_string_with_count`: Appends the second zero-terminated string to the first, copying at most the count given in the third argument and then terminating the result.
- `copy_string_and_zero_pad`: Copies a zero-terminated string up to the given count, then fills the rest of the destination with zero bytes.
- `compute_high_half_product`: Computes the cross-product sum of the 16-bit halves of its two arguments, shifted into the high word, and returns it as the upper half of a 64-bit result.
- `open_icon_library_window_entry`: Opens the icon library, then uses it to obtain a window-related entry and a file handle, storing both in the object.
- `call_hook_then_forward`: Calls a function pointer held in a global when it is set, then calls another routine with no arguments.
- `check_global_flags_and_cleanup`: Checks a global variable for non-zero and calls an indirect function obtained from it, then calls another routine.
- `release_table_and_locks`: Frees a table of entries when present by calling a release routine for each one, then releases a second held resource.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_indexed_file_slot`: Checks a slot index against the table size and whether the slot holds a handle.
- `call_close_wrapper`: Jumps through a library vector at a fixed offset of minus thirty-six from a library base held in a global.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_load_seg`: Wrapper that calls the dos library function LoadSeg and returns its result to the caller.
- `call_open_wrapper`: Calls the library open function through a vector at a fixed offset of minus thirty from a library base held in a global, passing the caller's arguments on.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_414`: Calls the library function at a vector offset of minus four hundred fourteen from a library base held in a global.
- `call_library_vector_414_3`: Calls an indirect function located at an offset within a structure pointed to by a global variable.
- `call_library_vector_198`: Calls the library function at a vector offset of minus one hundred ninety-eight from a library base held in a global.
- `execute_indirect_library_function`: Fetches a function pointer from a nested offset within a structure and executes it.
- `call_library_vector_294`: Calls the library function at a vector offset of minus two hundred ninety-four from a library base held in a global.
- `execute_library_function`: Retrieves a function pointer from a structure within a table and executes it.
- `call_library_vector_132`: Calls the library function at a vector offset of minus one hundred thirty-two from a library base held in a global.
- `execute_library_function_2`: Fetches a function pointer from a nested table structure and executes it.
- `call_library_vector_210`: Calls the library function at a vector offset of minus two hundred ten from a library base held in a global.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Calls the library function at a vector offset of minus three hundred seventy-two from a library base held in a global.
- `call_library_vector_552`: Calls the library function at a vector offset of minus five hundred fifty-two from a library base held in a global.
- `execute_library_function_3`: Retrieves a function pointer from a nested table structure and executes it.
- `call_library_vector_138`: Calls the library function at a vector offset of minus one hundred thirty-eight from a library base held in a global.
- `call_library_vector_366`: Calls the library function at a vector offset of minus three hundred sixty-six from a library base held in a global.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_library_vector_384`: Calls the library function at a vector offset of minus three hundred eighty-four from a library base held in a global.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_96`: Calls the library function at a vector offset of minus ninety-six from a library base held in a different global.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_78`: Calls the library function at a vector offset of minus seventy-eight from a library base held in a global.

### audio.device

- `return_from_interrupt`: The routine is empty and performs no operations.
- `call_audio_configuration`: Calls another routine to configure audio DMA channels.
- `call_pointer_from_structure`: Loads the first longword of the structure passed in the address register and calls the routine it points to.
- `update_countdown_entry`: Writes a stored value into a field reached through the entry's link pointer, installs the handler, and decrements a countdown word in the entry.
- `unlink_entry_and_update_counts`: Writes a stored value through the entry's link pointer when a flag is set, then unlinks the entry from a doubly linked list and decrements a countdown word.
- `advance_countdown_list_entry`: Decrements a countdown word in the entry.
- `store_entry_status_and_notify`: Stores the entry's link value and a status word with the top bit set into an owner structure.
- `unlink_and_store_entry_status`: Copies two status words into the owner structure and unlinks the entry from its list when a link is set.
- `call_cause`: Wrapper that calls the exec library function Cause and returns its result to the caller.
- `reset_entry_and_notify`: Clears a field of the entry, stores the handler, and copies a status word to the owner.
- `select_list_record_or_reset`: Resets the entry's count and copies a status word to the owner when the carry flag is set.
- `take_first_list_record`: Takes the first record from the entry's linked list, copies its size and data words into the entry, relinks the list, and stores the handler.
- `forward_call_and_return_2`: Calls another routine and returns.
- `call_routine_with_chip_base`: Calls one other routine and returns, and the decoder records a register access to the custom chip base.
- `call_routine_and_return`: Calls one other routine and returns with no other work.
- `call_gap_routine`: Calls one other routine and returns; the bytes sit between known routines and decode as code.
- `clear_audio_interrupt_flag`: Clears the second bit of a byte located at an offset from a pointer held in a register.
- `remove_list_node_and_reset`: Removes the node from the list it belongs to by relinking its neighbours, then resets the node's own links to point at itself and clears its second field.
- `drain_list_with_interrupt_guard`: While the list head is non-null, removes the first node and raises a nesting counter in the base structure.
- `decrement_counter_and_enable_interrupts`: Reads a byte at an offset in the base structure, decrements it and writes it back.
- `configure_audio_dma_channels`: Initializes a data structure and a list of DMA channel configurations by iterating through a table.
- `set_field_and_forward`: Writes the byte value 254 into a field of the first argument's structure and calls another routine with the second argument's offset structure and the first argument.
- `add_request_to_list_tail`: Writes the byte value 254 into a field of the first argument, then adds that structure to the tail of a list through the add-tail wrapper.
- `link_request_and_update_registers`: When the request has a non-null link and its flag bit is clear, moves the link into the second word, installs the handler, and writes a status word to the control register.
- `store_request_status_and_enable`: Moves the request's link into its second word and installs the handler.
- `process_linked_request_chain`: Follows a link field of the structure, calling a routine on the current structure and then on the linked structure when that link differs from another field.
- `move_listed_request_to_tail`: Walks the list for the given node.
- `scan_list_then_move_request`: Walks a list from a given node looking for a target node.
- `locate_request_then_move`: Walks a list from its head looking for the target node.
- `mark_list_requests_and_dispatch`: Walks a list and writes the byte value 254 into a flag field of every node.
- `mark_chain_flags_and_dispatch`: Walks a chain of nodes from a given node, writing the byte value 254 into the flag field of each.
- `stop_request_and_clear_bit`: Calls the disable wrapper, then calls routines on the argument and on fields at two offsets of it.
- `write_table_value_to_adkcon`: Calls two routines to set up the argument, stores a fixed longword in an indexed slot, and writes a value from a byte table into the audio control register.
- `store_table_byte_and_call`: Stores a byte from a table indexed by a register into the memory location one byte before the argument, then calls a library function through a vector offset of minus thirty from a library base held in the structure.
- `initialize_list_head`: Initializes an empty list head: points its first word at itself, clears the second word and points the third word at itself.
- `call_disable`: Library stub: loads its arguments and the exec library base, calls Disable and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `forbid_multitasking`: Calls the exec Forbid function through the library vector at offset minus one hundred thirty-two from the exec base held in a global, which suspends task switching.
- `call_set_int_vector`: Library stub: loads its arguments and the exec library base, calls SetIntVector and returns its result to the caller.
- `free_memory_block_via_exec`: Calls the exec FreeMem function through the library vector at offset minus two hundred ten from the exec base held in a global.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `call_remove`: Library stub: loads its arguments and the exec library base, calls Remove and returns its result to the caller.

### Populous (1989)(Electronic Arts)[cr VF][a].adf:populous.prg

- `intro_bytekiller_depack_2`: Cracktro start: ByteKiller depacker that decodes the packed stream after it backwards into 50000 hex onwards, showing progress on COLOR00, checks the checksum and jumps to the unpacked cracktro at 50000 hex.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### icon.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `increment_count_clear_bit`: Increments a word counter at a fixed offset of the object and clears bit three of a status byte at another offset.
- `decrement_count_and_check_flag`: Decrements a word counter at a fixed offset of the object.
- `unlink_and_free_request`: When the counter at a fixed offset is zero, unlinks the object from its list, reads its stored value and frees its memory through the library vector at offset minus two hundred ten.
- `set_flag_bit_and_return`: Sets bit three of a status byte in the object and returns zero.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `record_request_pointer_or_alert`: Stores its arguments in fields of the object, then calls a library function at offset minus four hundred eight.
- `copy_two_fields_to_globals`: Copies two longword fields from the object into two global variables.
- `execute_empty_routine`: This library routine is empty.
- `return_from_subroutine_3`: This routine performs a single return instruction to exit the current execution flow.
- `call_library_handler`: This library routine simply calls another routine and then returns.
- `forward_icon_library_call`: Library entry in the icon library that calls one other routine and returns, with no other operations.
- `call_icon_init_routine`: This library routine invokes a single initialization function to set up icon data structures and then returns.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `call_handler_pointer`: This library routine retrieves a function pointer from a data structure and executes it.
- `call_library_handler_2`: This library routine calls another routine and then returns.
- `allocate_and_initialize_record`: Calls a routine to allocate a zero-filled 168-byte record and initializes a field within that record using library calls.
- `invoke_icon_initialization`: This library routine calls a single initialization function to set up icon data structures and then returns.
- `call_handler_pointer_2`: This library routine fetches a function pointer from a data structure and calls it.
- `forward_icon_library_call_2`: Library code in the icon library that calls one other routine and returns, with no other operations.
- `call_other_routine`: The library routine calls another routine and then returns.
- `call_other_routine_2`: This library routine calls another routine and then returns without performing any other operations.
- `call_other_routine_3`: The routine calls another routine and then returns, performing no additional tasks.
- `store_argument_in_result_field`: Library code that calls a function through a vector offset of minus two hundred ninety-four from a base taken from the first argument's structure, then stores the second argument at offset one hundred forty-eight of the returned object.
- `chain_two_calls_with_cleanup`: Library code that calls one routine and, when it returns a nonzero value, calls a second routine with that value.
- `read_icon_record_fields`: Library code that obtains a record through one call and fills a local buffer through a second call.
- `copy_frame_fields_into_object`: Library code that copies a block of ten longwords from the stack frame into the object, then copies several further single fields into fixed offsets of the object, and calls one other routine.
- `return_zero_value`: Returns zero and performs no other work.
- `build_icon_header_and_copy`: Library code that writes a fixed magic word and a version word into a local buffer, copies ten longwords and several fields from the object into it, then calls one other routine.
- `load_info_file_into_icon`: Library code that builds a name in a 260-byte buffer from its arguments and the text .info, opens it through the library vector at offset minus thirty, and loads its contents with several further calls.
- `validate_icon_header_and_load`: Library code that checks a header word against the icon magic value and a version word of one, then clears two result words.
- `load_icon_record_fields`: Reads two integer fields into the record, then conditionally reads a counted block of pointers and a further value, undoing partial work on failure.
- `read_icon_pointer_table`: Reads values into a pointer table one by one while a remaining-length counter stays above a threshold, then reads one trailing value.
- `return_zero_after_call`: Calls one library routine and then returns zero regardless of its result.
- `empty_routine`: The routine contains four instructions and performs no observable work; it ends immediately with a return.
- `write_icon_record_to_stream`: Allocates a temporary buffer, writes the record's header fields and its optional data blocks in order as size-prefixed chunks, then frees the buffer and restores a saved field.
- `write_info_icon_contents`: Writes an icon record's fields and optional blocks to a stream using the library's write routine, referencing the .info text.
- `write_icon_optional_blocks`: Writes the record's optional blocks, a pointer table and a trailing block, each preceded by its length, and returns 1 only if every write succeeds.
- `write_table_length_and_entries`: Counts the entries of a null-terminated pointer table, writes that total length, then writes each entry with its length prefix followed by a trailing block when present.
- `write_icon_table_entries`: Walks a null-terminated table of pointers, writing each entry with a length prefix and its contents through the write routine, then writes a trailing block if present.
- `free_node_chain_entries`: Walks a linked list from a fixed offset in the node, releasing each node through a library call until it reaches the end or an empty link, then releases the final node.
- `iterate_linked_list_nodes`: Walks a linked list by following the link stored at a fixed offset in each node, calling a library routine on each node until the list ends or a stop value appears, then processes the final node.
- `forward_call_at_offset`: Calls the same routine again with the first argument unchanged and the second advanced by a fixed 140-byte offset.
- `allocate_zeroed_record_block`: Allocates a 168-byte zero-filled record, sets up a field at a fixed offset with library calls, and returns the record; on setup failure it frees the block and returns null.
- `call_library_and_return`: Calls one library routine with no arguments and returns without producing a value.
- `return_zero_value_2`: Returns zero with no other work, serving as a constant-result stub.
- `append_pair_with_refill`: Decrements an entry counter, refilling the buffer through a library routine when it runs out, then stores two values into the next pair slot.
- `decrement_counter_and_store_pair`: Decrements a 16-bit counter and stores two register values into the next pair slot of the table referenced by the buffer, then returns 1.
- `allocate_zeroed_control_block`: Allocates a 94-byte zero-filled block, initializes a region at a fixed offset with two library calls, and returns the block; on failure it releases the block and returns null.
- `stub_return_immediately`: Contains only a return instruction, so it does no work and acts as an empty stub.
- `call_then_return_zero`: Calls one library routine and then returns zero.
- `chain_next_record_call`: Calls the same routine for the next record block by adding a fixed 78-byte offset to the second argument, passing the first argument unchanged.
- `find_value_after_equals`: Searches a null-terminated list of entries, testing each against a key through a library routine, and returns a pointer to the text after the equals sign of the matching entry, or null when none match.
- `scan_list_for_equals_sign`: Scans a null-terminated list of strings for an entry whose text contains an equals sign at the matched position, returning a pointer to the value after it, or null when the list ends.
- `search_list_for_key_value`: Iterates a null-terminated list of pointers, calling a library routine for each entry, and returns the text following an equals sign when the test succeeds, or null when the list ends.
- `find_matching_delimited_segment`: Walks the separator-delimited segments of a string, looking for one whose length matches a reference length and that a library routine confirms as equal to the given text.
- `scan_delimited_segments_for_match`: Steps through the separator-delimited segments of a string and checks each segment's length against a reference value, confirming matches through a library routine.
- `search_segments_for_reference_length`: Repeatedly locates the next separator in a string, measures each segment, and returns 1 when a segment of the reference length is confirmed by a library routine, or 0 when no segment qualifies.
- `check_segment_length_match`: Compares the length of each separator-delimited segment with a reference length and confirms candidate segments with a library call.
- `test_segment_list_for_match`: Tests separator-delimited segments in sequence, treating a segment as matching when its length equals the reference and a library routine reports equality.
- `parse_copy_label_count`: Reads an optional copy label from the start of a text record, followed by a decimal count when present, defaulting the count to one, then stores the count and the remaining text in the output record.
- `parse_decimal_count_after_label`: Checks for a three-character label at a fixed position in a text record, then reads the decimal number that follows and finishes with two library cleanup calls.
- `parse_decimal_digits_then_check`: Reads consecutive decimal digits from the text, accumulating their value, then tests the following four characters with a library routine when the count is non-zero, and finishes with two cleanup calls.
- `parse_count_before_of_text`: Accumulates the decimal digits of a count in the text, then checks the following ' of ' wording through a library routine when the count is non-zero, and ends with two cleanup calls.
- `convert_char_to_digit`: Maps a character code to its decimal value from zero to nine, or returns minus one when the character is not a decimal digit.
- `return_negative_one`: Returns minus one as a fixed failure value and does nothing else.
- `call_library_function_198`: Calls a library function through the library base held in a global variable, at a fixed negative offset, and returns nothing.
- `execute_indirect_call`: This library routine performs an indirect call to a function pointer retrieved from a table.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `allocate_and_register_with_cleanup`: Allocates a value from the first pair of arguments, passes it with the remaining arguments to a second routine, and releases the value with a cleanup call if that second step fails, returning zero in that case.
- `allocate_initialized_record`: Allocates a 96-byte zero-filled block, stores ten in its field at a fixed offset and in the caller's word, and calls a library routine on the block, returning whether allocation succeeded.
- `return_zero`: This library routine returns zero.
- `verify_result_matches_argument`: Calls a library routine with three arguments and returns its result only when it equals the fourth argument, otherwise returns zero.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `confirm_library_result_match`: Calls another library routine with three arguments and returns the result only when it equals the fourth argument, otherwise returns zero.
- `return_empty_routine`: Contains only a return instruction, so it does no work.
- `divide_32_bit_unsigned`: Divides an unsigned 32-bit dividend by a divisor, using a shift-and-subtract loop when the divisor is large and direct 16-bit division otherwise, and returns the quotient and remainder.
- `perform_division_step_loop`: Runs sixteen shift-and-subtract iterations that shift a 32-bit value and subtract the divisor from its high half whenever it fits, leaving the quotient and remainder in registers.
- `multiply_32_bit_values`: Multiplies two 32-bit values using 16-bit partial products and returns the low 32 bits of the product.
- `invoke_formatting_library_entry`: Calls the library's raw formatting routine through a function pointer kept at a fixed offset in the global library base, with no arguments and no return value.
- `store_byte_to_pointer`: Writes the low byte of the data register through the address register, storing one byte at the target pointer.
- `measure_string_length`: Counts the characters of a null-terminated string by decrementing an accumulator for each character and returns the complemented result as the length.
- `find_character_in_string`: Walks a null-terminated string and returns a pointer to the first occurrence of the given character, or null when it is absent.
- `return_zero_constant`: Returns zero with no other work.
- `copy_null_terminated_string`: Copies bytes from the source string into the destination, including the terminating zero byte.
- `append_string_to_destination`: Advances to the end of the destination string, then copies the source string there including its terminating zero byte.
- `compare_strings_up_to_limit`: Compares two strings character by character up to a given count, returning 1 when a difference is found and 0 when they match within the limit or the count runs out.
- `return_one_constant`: Returns one with no other work.
- `terminate_bounded_string_scan`: Walks a string up to a given count, and when the terminating zero byte is reached makes a library call; otherwise it returns without the call.
- `forward_to_library_routine`: Calls a library routine with no arguments and returns without a value.
- `copy_string_with_zero_padding`: Copies at most a fixed number of bytes from source to destination, stopping after the terminating zero, then fills any remaining space up to the limit with zero bytes.
- `return_from_subroutine_4`: This is an empty routine that performs no operations and immediately returns to its caller.
- `initialize_empty_list_header`: Sets up an empty linked list header, pointing its first link at the following slot, clearing the next link, and pointing the last link back at the header itself.
- `process_record_header_and_payload`: Transfers a 20-byte header through a library routine, then transfers the payload whose address is stored in the record, returning 1 when there is nothing to transfer and 0 when the header transfer fails.
- `transfer_record_chain_entries`: Walks a linked chain of records, transferring each record's 16-byte header and then its data block sized by a count stored in the header, returning 1 when the chain completes and 0 on any failure.
- `walk_record_chain_pairs`: Walks a linked chain, calling two library routines on each record and following its next link at a fixed offset, returning 1 when the chain ends and 0 when a routine fails.
- `return_zero_2`: The library routine returns a value of zero.
- `allocate_configured_record`: Allocates a 20-byte zero-filled record when the flag argument is set, fills its field with a value from a library call, and returns the record; it returns zero when the flag is clear and minus one on failure.
- `return_failure_minus_one`: Returns minus one as a failure value with no other work.
- `allocate_record_chain`: Allocates a chain of 16-byte records, linking each to the previous one and filling each with data read through library calls, and returns the head of the chain or minus one when any step fails.
- `read_record_chain_into_list`: Reads a chain of 16-byte records through a library call, storing each record's data block pointer and linking the records together, and returns the head record or minus one on a read failure.
- `read_record_chain_links`: Reads a sequence of records, linking each to the next and storing their data blocks, and returns the first record when the sequence ends or minus one when a read fails.
- `return_error_code_minus_one`: Returns minus one with no other work, serving as an error code stub.

### translator.library

- `create_and_register_library`: Creates a library structure through a library call, and if that succeeds stores the global base and a parameter in it and registers it with the system library list.
- `increment_counter_clear_flag`: Increments a 16-bit counter in the structure and clears one flag bit in its status byte.
- `decrement_counter_and_signal`: Decrements a 16-bit counter in the structure, and when it reaches zero while a flag bit is set, makes a library call and returns its result; otherwise returns zero.
- `unlink_or_mark_pending_node`: If a counter in the node is zero, unlinks the node from its list and makes a library call, returning that call's value; otherwise it sets a pending flag on the node and returns zero.
- `unlink_and_release_node`: Unlinks a node from its doubly linked list, makes a library call on it, and returns the value stored in the node.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `parse_padded_text_field`: Fills a local line buffer with spaces, then copies characters from the input text into it, calling library routines per field until a hash character ends the input, and returns the last status value.
- `return_low_word_register`: Returns the low 16 bits of the data register as the result.
- `terminate_string_and_return_status`: Makes a library call, writes a zero byte just before the address register, and returns the 16-bit result value.
- `scan_delimited_text_fields`: Scans input text for space-separated fields with a hash character as an end marker, calling a library routine for each field and returning a 16-bit status value when the scan finishes or fails.
- `scan_text_fields_with_markers`: Walks input text field by field, treating spaces and hash characters as markers, calls a library routine for each field, and returns a 16-bit status value when the text ends.
- `scan_escaped_text_segments`: Scans text for backslash escapes and backquote delimiters, copying segments and calling library routines for each, and returns once the final delimiter is reached.
- `scan_escaped_text_loop`: Reads text character by character, treating backslash and backquote as markers, and calls library routines when a marker closes a segment, returning when the closing marker of the final segment is found.
- `stub_immediate_return`: The decompiled body contains only a return, so the routine does no visible work in the library.
- `match_bracketed_text_prefix`: Skips ahead to an opening bracket in the text, then compares the following characters against a reference string until a closing bracket, returning when the comparison ends.
- `match_backward_character_class`: Walks characters backwards from the end of a buffer while they belong to a character-class table, comparing each with the pattern, and returns when a non-matching character or the buffer start is reached.
- `return_without_effect`: Contains only a return instruction and changes nothing.
- `match_backward_pattern_segment`: Walks a buffer backwards using a character-class table, returning when the position reaches the start or a character fails to match the pattern.
- `scan_forward_to_equals`: Reads characters forward from the start of the text, passing over characters in the class table and comparing the rest with the reference text, and returns at an equals sign or when a mismatch occurs.
- `match_text_until_equals`: Compares input characters against a reference string forward until an equals sign appears, skipping characters flagged in the class table, and returns when they differ.
- `copy_escaped_token_text`: Copies a token from the text up to a backslash or backquote marker into the output buffer, adjusting the remaining length, and returns the updated position pair.
- `copy_token_and_update_flag`: Copies a run of characters into the output buffer, sets a flag when the last copied character belongs to a marker class, and returns the updated length pair.
- `copy_fixed_length_token`: Copies a fixed number of bytes into the output buffer, then sets a flag bit under a condition and clears it again when the following character is a backquote.
- `terminate_token_at_class_boundary`: Walks backwards from the current position while characters stay in a class, then writes a zero byte at the boundary to terminate the token.
- `dispatch_special_character_handler`: Looks up the input character in a table of eleven symbol characters and calls the handler at the matching position in a jump table.
- `return_from_subroutine_8`: Performs a standard subroutine return to the caller.
- `stub_empty_return_slot`: Contains only a return in the decompiled output, so it is an empty stub with no visible effect on the library.
- `call_one_routine_then_return`: Calls a single routine with no arguments and returns.
- `poll_until_status_clear`: Repeatedly calls a routine while bit 5 of a status register remains set, then returns.
- `forward_single_call`: Makes one routine call with no arguments and returns its outcome without further work.
- `examine_status_character_code`: Calls a routine when a status bit is set and examines the returned character against two code letters, but takes no further action.
- `return_from_subroutine_5`: This routine is an empty function that performs a return from a subroutine.
- `match_keyword_letters_sequence`: Matches a short keyword spelled by successive input characters against expected letters, using a flag bit to choose the valid spellings, and returns once the keyword is recognised or rejected.
- `match_negated_keyword_sequence`: Matches the letters N and G from successive input characters, requiring a trailing S when a flag bit is set, and returns when the sequence ends.
- `return_from_subroutine_6`: This routine performs a single return instruction to exit the current function.
- `pass_control_to_routine`: Calls one routine with no arguments and returns, performing no other work.
- `return_to_caller`: This routine performs a standard subroutine return to the caller.
- `return_to_caller_2`: This routine performs a standard subroutine return to the calling function.
- `library_return_wrapper`: This library code consists of three instructions and immediately returns from a subroutine.
- `repeat_call_while_flag_set`: Calls a routine repeatedly, checking after each call whether bit 5 of a status register is still set, and returns when it clears.
- `execute_callback_pointer`: Retrieves a function pointer from a data structure and executes it.
- `poll_until_second_flag_clear`: Repeatedly calls a routine while bit 1 of a status register remains set, then returns.
- `check_status_bit_after_call`: Calls a routine and then tests bit 7 of a status word, taking no further action in either case.
- `return_from_subroutine_7`: This routine is an empty function that performs a return instruction.
- `normalize_text_into_buffer`: Copies input characters into a fixed-width text buffer, mapping control characters, the closing bracket and lowercase letters to buffer-safe forms, stopping at a hash pair or after 100 characters, and returning minus three when the limit is exceeded.
- `blank_previous_character`: Writes a space character two bytes before the current buffer position.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 253 into d1, then returns.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies 32 to (a2)+, then copies 32 to (a2)+, then returns.
- `insert_phoneme_marker_on_match`: Scans phonetic text for a match in a table of fourteen entries and, on a match, shifts the buffer one place and inserts a marker character, then stores the new end position.
- `shift_and_insert_marker`: Shifts the buffer contents one place forward from the current end, inserts a marker character at the insertion point, and stores the new end position.

### info.library

- `stub_unimplemented_instruction`: The decompiler could not translate its instructions, so the routine has no recovered behaviour beyond a stop marker.
- `store_registers_into_globals`: Stores two incoming register values into global variables and into a field of the structure given as the first argument.
- `forward_call_and_return_3`: Library code that makes a single call to another routine and then returns, adding nothing of its own.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `release_unused_region`: Library code that calls another routine to release a memory region and then returns.
- `return_from_subroutine`: This routine is an empty function that performs no operations and immediately returns to its caller.
- `pass_through_single_call`: Makes a single routine call and returns, performing no other work.
- `clear_bit_and_count_use`: Clears bit 3 of a status byte in the structure, increments its 16-bit counter, and returns the structure pointer.
- `release_region_when_unused`: When a counter in the structure is zero, releases a memory region derived from two of its size fields through library calls and returns the stored result; otherwise it sets bit 3 of a status byte and returns zero.
- `release_structure_region`: Calls a library routine on the structure, then on a region derived from two of its size fields, then makes another library call, and returns the stored result.
- `decrement_counter_and_release`: Decrements a 16-bit counter in the structure, and when it reaches zero while a status bit is set, calls a routine on the structure and returns its result; otherwise returns zero.
- `return_zero_constant_2`: Library code that returns zero.
- `release_global_resources`: Releases several global resource pointers and an argument pointer by calling a routine on each non-null entry, with a recursive variant for a second argument, and returns nothing.
- `allocate_and_link_workspace`: Allocates a 944-byte cleared workspace, stores pointers to its internal sub-blocks in global slots, copies default values into several of them, links the lists together, and returns one on success or zero when allocation fails.
- `initialize_workspace_pointers`: Points the global slots at fixed offsets within an existing workspace, copies default values into its sub-blocks, links its lists together, and returns one.
- `open_system_libraries_or_fail`: Opens three system libraries including dos.library through library calls, storing each base in a global slot; if any open fails it reports a status code through a cleanup routine and returns zero, otherwise one.
- `open_three_libraries_via_pointer`: Opens three libraries through a function pointer, storing each base in a global slot, and reports a status code through a second pointer when one fails; returns one on success and zero on failure.
- `open_intuition_and_second_library`: Opens the intuition library and a second library through a function pointer, storing their bases, and reports a failure status through a second pointer; returns one when both open.
- `open_graphics_library`: Opens the graphics library through a function pointer and stores its base in a global slot; on failure it reports status five through a second pointer and returns false.
- `return_success_one`: Returns one as a success status with no other work.
- `get_structure_field_value`: Returns a cached value from the structure when one is set; otherwise reads a field from a structure chosen by a type code or from its parent, and returns zero when no structure is present.
- `get_structure_value_via_pointer`: Reads a value from the structure referenced by a pointer argument, returning a cached field when set, otherwise a field from the structure's linked entry, or zero when the link is missing.
- `read_cached_or_linked_value`: Returns the cached value from the structure if it is set, otherwise a value from its linked entry, or zero when no link exists.
- `read_type_based_value`: Returns a value from a structure selected by a type code, either the structure itself or its linked entry, or zero when the link is missing.
- `read_linked_entry_value`: Returns a value stored in a linked entry, or zero when the entry pointer is missing.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a2, then returns.
- `resolve_disk_info_entry`: Creates an entry record, copies either the disk information name or a stored name into a 116-byte buffer, looks up the matching entry, and returns its value; for one type it falls back to a stored value and sets a flag.
- `lookup_name_with_fallback_flag`: Copies a stored name into a local buffer, looks it up through a library call, and returns the result, or a fallback value with a flag set when the lookup fails for a particular type.
- `allocate_paired_buffers`: Allocates a 36-byte zero-filled block and a 260-byte zero-filled block, copies data into the first, stores a computed size, allocates a region of that size, and returns one on success or zero when any allocation fails.
- `fill_buffer_record_from_pointers`: Stores a new block pointer, copies a name into the buffer region, computes a size from the stored data, allocates a region of that size, and returns one on success.
- `populate_record_fields`: Copies a name into a record region, derives a size from stored data, allocates a region of that size, and fills two record fields, returning one on success and zero otherwise.
- `populate_record_data`: Updates a structure by calling another routine to retrieve a value, adjusting a field in the structure, and copying data from several pointers.
- `populate_record_fields_unchecked`: Copies a name into a record region, derives a size, allocates a region, records its address, and returns whether the allocation succeeded.
- `store_record_fields_tail`: Copies a name into a record region and stores two dword fields taken from the source structures, returning one.
- `open_display_resources`: Clears global resource slots, allocates a 246-byte cleared block, opens the required libraries and resources through library calls, and attaches them; on failure it reports one of several numbered error codes.
- `setup_display_resources_via_pointer`: Allocates a 246-byte cleared block, acquires dependent resources, links them into the display structure, and reports numbered failure codes through a function pointer.
- `attach_display_resources_partial`: Allocates a 246-byte cleared block and links its dependent resources into the display structure, reporting numbered failure codes through a function pointer when a step fails.
- `attach_resource_to_display`: Acquires a resource through a library call, stores it in a global slot, links it into the given structure, and reports an error status through a function pointer on failure.
- `finalize_resource_attachment`: Obtains a resource, links it into the structure, performs four cleanup calls, and reports the completion status through a function pointer.
- `release_partial_resource_setup`: Links an acquired resource into the structure, performs four cleanup calls, and reports a status code through a function pointer; the failure path reports code one.
- `report_resource_setup_status`: Checks the outcome of a setup step and reports success or failure through a function pointer, running cleanup calls first when the step succeeds.
- `report_setup_status_and_cleanup`: Runs a cleanup call when a flag is clear and an argument is set, performs four cleanup calls, and reports a success status through a function pointer.
- `call_library_function_132`: Calls the library function at a fixed negative offset of 132 bytes from the library base kept in a global slot, passing no arguments and returning nothing.
- `call_library_function_138`: Calls the library function at a fixed negative offset of 138 bytes from the library base kept in a global slot, passing no arguments and returning nothing.
- `call_library_function_198_2`: Calls the library function at a fixed negative offset of 198 bytes from the library base kept in a global slot, passing no arguments and returning nothing.
- `call_library_function_210`: Calls the library function at a fixed negative offset of 210 bytes from the library base kept in a global slot, passing no arguments and returning nothing.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_function_246`: Calls the library function at a fixed negative offset of 246 bytes from the library base kept in a global slot, passing no arguments and returning nothing.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_function_294`: Calls the library function at a fixed negative offset of 294 bytes from the library base kept in a global slot, passing no arguments and returning nothing.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_function_372`: Calls the library function at a fixed negative offset of 372 bytes from the library base kept in a global slot, passing no arguments and returning nothing.
- `call_library_function_378`: Calls the library function at a fixed negative offset of 378 bytes from the library base kept in a global slot, passing no arguments and returning nothing.
- `call_library_function_414`: Calls the library function at a fixed negative offset of 414 bytes from the library base kept in a global slot, passing no arguments and returning nothing.
- `call_library_function_552`: Calls the library function at a fixed negative offset of 552 bytes from the library base kept in a global slot, passing no arguments and returning nothing.
- `call_library_function_42`: Calls the library function at a fixed negative offset of 42 bytes from the library base kept in a global slot, passing no arguments and returning nothing.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_function_96`: Calls the library function at a fixed negative offset of 96 bytes from the library base kept in a global slot, passing no arguments and returning nothing.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_function_228`: Calls the library function at a fixed negative offset of 228 bytes from the library base kept in a global slot, passing no arguments and returning nothing.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_function_366`: Calls the library function at a fixed negative offset of 366 bytes from the library base kept in a global slot, passing no arguments and returning nothing.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `call_library_function_462`: Calls the library function at a fixed negative offset of 462 bytes from the library base kept in a global slot, passing no arguments and returning nothing.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_function_72`: Calls the library function at a fixed negative offset of 72 bytes from the library base kept in a global slot, passing no arguments and returning nothing.
- `call_library_function_78`: Calls the library function at a fixed negative offset of 78 bytes from the library base kept in a global slot, passing no arguments and returning nothing.
- `call_library_function_234`: Calls the library function at a fixed negative offset of 234 bytes from the library base kept in a global slot, passing no arguments and returning nothing.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_function_270`: Calls the library function at a fixed negative offset of 270 bytes from the library base kept in a global slot, passing no arguments and returning nothing.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_function_354`: Calls the library function at a fixed negative offset of 354 bytes from the library base kept in a global slot, passing no arguments and returning nothing.
- `invoke_library_functions`: Library code that calls two different library functions using a base address from a global variable and a pointer passed via a stack-allocated structure.
- `dispatch_info_event_code`: Dispatches on a 16-bit event code read from a nested structure, calling a routine with the associated argument, and returns fixed zero or one results for codes that end processing.
- `return_one_constant_2`: Library code that returns one.
- `toggle_status_bit_for_code`: Maps a 16-bit code to one of seven bit values, XORs that bit into the status word stored in the global block, and calls a routine on the associated structure.
- `store_status_and_notify`: Stores a register value into the status word in the global block and calls a routine on a field of the given structure.
- `build_info_file_path`: Formats a directory and file name into a path buffer with the .info suffix, checks related entries through library calls, and returns whether the operation succeeded.
- `validate_disk_icon_flags`: Formats a name from disk text and checks it through library routines, clearing the result when a check fails or the status bits remain set, and returns that result.
- `check_icon_name_and_flags`: Formats a name with the .info suffix, checks the entry's type and name through library routines, and clears the result when a check or the status bit test fails.
- `verify_four_library_steps`: Runs four library calls with a minus-one argument and returns one only when none of them fails; on success it makes one further call on the parameter.
- `verify_three_library_steps`: Runs three library calls through a function pointer with a minus-one argument and returns one only when none of them fails, calling a routine on success.
- `verify_two_library_steps`: Runs two library calls through a function pointer with a minus-one argument and returns one only when neither fails, calling a routine on success.
- `verify_single_library_step`: Makes one call through a function pointer and returns whether it succeeded, calling a routine on success.
- `return_success_after_call`: Calls one routine and returns one.
- `build_file_panel_gadgets`: Adds a fixed set of entries at pixel rectangles to the file panel, including the column labels NAME, STATUS and TYPE, by calling a routine for each entry.
- `build_secondary_panel_gadgets`: Adds seven fixed-position entries with pixel rectangles to the secondary panel by calling a routine for each.
- `build_compact_panel_gadgets`: Adds seven fixed-position entries with pixel rectangles to a compact panel by calling a routine for each.
- `build_toolbar_gadgets`: Adds nine fixed-position entries with pixel rectangles to the toolbar by calling a routine for each, with some entries flagged as optional.
- `build_dialog_gadgets`: Adds nine fixed-position entries with pixel rectangles to the dialog by calling a routine for each.
- `draw_panel_frame_for_mode`: Chooses a position value from the display mode, draws a frame at fixed coordinates when the list is empty or its entries otherwise, then redraws an optional element when the flag is set.
- `redraw_list_contents_if_flagged`: Redraws the list's entries when it is empty or the item-count check holds, otherwise draws the items, and repaints a second element when a flag is set.
- `redraw_list_element_optional`: Redraws a list element through a library call and repaints an optional second element when the flag argument is set.
- `allocate_list_entry_nodes`: Allocates a list header, then for each entry of a null-terminated table allocates a node and a 128-byte cleared data block, linking them together; returns the header on success or zero when the header allocation fails.
- `build_node_list_from_table`: Library code that walks a null-terminated pointer table taken from a global, creating a node for each entry through library calls and linking the nodes into a list.
- `link_nodes_from_pointer_table`: Library code that loops over a null-terminated table of pointers, allocating a node per entry via library calls, storing each entry in its node and linking the node into the list.
- `link_nodes_until_table_end`: Library code that allocates a node for each table entry, stores the entry in the node and links it onto the list, looping until the table terminator is reached.
- `retire_current_list_head`: Library code that checks whether the list head is the only remaining entry; if so it finishes that entry through three library calls, otherwise it sends two messages for the current entry and advances the head to the next linked node.
- `advance_cursor_pointer_and_notify`: Library code that passes the entry referenced by the current cursor to a subroutine together with a global value, then moves the cursor to the entry's next link.
- `step_list_cursor_or_finish`: Library code that tests whether the cursor has reached the end of the list; at the end it finishes the entry through library calls, otherwise it notifies two subroutines and moves the cursor to the entry stored in the next link.
- `step_cursor_via_dereference`: Library code that passes the current cursor entry to a subroutine with a global value, replaces the cursor with the entry pointed to by its link, and makes a final subroutine call.
- `append_entry_to_global_list`: Library code that sets a global status value, notifies subroutines when the list is not empty, allocates a new entry through a library call, copies its data and inserts it at the tail of the global list.
- `release_entry_and_relink_list`: Library code that sets a global status value and, for a list with more than one entry, notifies subroutines and selects the neighbouring link as the new list head.
- `advance_cursor_with_fallback`: Library code that passes the current cursor entry to a subroutine and moves the cursor to the entry in its next link, falling back to the first list entry when the list end is reached.
- `notify_entry_callbacks_in_sequence`: Library code that makes one subroutine call and then two more subroutine calls, passing values read from fixed fields of the current entry at offsets 46 and 62 bytes.
- `build_entry_array_if_flagged`: When a global flag is set, library code counts the list entries, allocates an array with one pointer slot per entry, and allocates a block for each entry sized from the entry's own value with a default of four bytes.
- `build_entry_pointer_array`: Library code that counts the list entries, allocates a pointer array sized to that count, and fills each slot with a newly allocated block for the matching entry.
- `allocate_entry_array_after_notify`: Library code that first passes a value from the current entry to a subroutine, then counts the list entries and allocates an array of pointers, one per entry.
- `count_and_allocate_entry_array`: Library code that counts the links in a pointer chain, allocates an array with one long-word slot per link through a library call, and fills each slot with a block for the matching entry.
- `fill_entry_array_from_list`: Library code that passes the array base to a subroutine, then walks the list allocating a block for each entry and storing its pointer in the array.
- `fill_entry_blocks_until_end`: Library code that walks the list from a given entry, allocating a block for each entry and storing its pointer at the current array index.
- `copy_entries_into_array_slots`: Library code that walks entries from the current one, passing each to library calls that allocate a block and copy the entry's data into the matching array slot.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_offset_minus_90`: Library stub that loads a library base from a global variable and calls the library function at offset minus 90 with no arguments.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `call_library_vector_114`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.
- `call_library_vector_180`: Library stub: loads a library base from a global variable and calls the library function at offset minus 180 of that base, returning its result to the caller.
- `call_library_vector_186`: Library stub: loads a library base from a global variable and calls the library function at offset minus 186 of that base, returning its result to the caller.
- `dispatch_layout_by_mode`: Library code that reads a mode byte from a global block and selects one of six fixed sequences of subroutine calls.
- `stub_empty_return`: Empty library routine whose only instruction is a return, so calling it has no effect.
- `allocate_mode_buffers_and_run`: Library code that allocates four fixed-size buffers through library calls and, if all succeed, allocates four more.
- `run_mode_sequence_after_setup`: Library code that reads four values through indirect calls, stores the last one through a pointer, and returns zero if any value is zero.
- `run_mode_helper_sequence`: Library code that switches on a mode byte and issues a fixed count of subroutine calls for that mode, from two calls up to five.
- `stub_return_only`: Empty library routine that contains only a return instruction, so it performs no work when called.
- `clear_or_copy_then_check`: Library code that either zeroes a longword through a pointer or passes a global buffer and a length to a subroutine, depending on a global value.
- `copy_buffer_and_check_status`: Library code that passes a global buffer and a length to a subroutine, then calls a second subroutine and maps its failure value of minus one to zero.
- `stub_return_immediately_2`: Empty library routine that returns at once without any operations.
- `update_four_entries_and_check`: Unless the mode byte equals four, library code stores fixed 16-bit values into a field of four structures and then calls a subroutine on each structure.
- `check_three_calls_succeeded`: Library code that calls a function pointer three times in sequence and returns one only when none of the calls returns minus one.
- `check_two_calls_succeeded`: Library code that calls a function pointer twice and returns one only when neither call returns minus one.
- `check_single_call_not_failed`: Library code that calls a function pointer once and returns true unless the result is minus one.
- `stub_return_success`: Library routine that returns the constant one and performs no other work.
- `process_entries_until_failure`: Library code that loops indefinitely, taking an entry from a structure and passing it with a parameter to a subroutine.
- `return_zero_value_3`: This library routine is empty and simply returns zero to the caller.
- `dispatch_state_handler_result`: Library code that branches on a numeric state field: state four calls one subroutine, state 64 returns the result of a second subroutine, and state 512 calls a subroutine and returns zero.
- `forward_single_helper_call`: Routine that invokes one subroutine with no arguments and returns without a value.
- `call_helper_return_zero`: Calls one subroutine with no arguments and returns zero regardless of what the subroutine returns.
- `call_two_helpers_return_one`: Calls two subroutines in sequence with no arguments and then returns one.
- `send_mode_dependent_callbacks`: Library code that calls subroutines three times with the given parameter, adds a fourth call unless the mode byte equals one, then makes two final subroutine calls.
- `format_four_access_flags`: Library code that derives a four-bit flag value from a byte field and calls a subroutine once per flag with the flag position and a shared text table.
- `format_indexed_flag_label`: Copies a fixed short string into a stack buffer, selects one of two label strings according to a flag argument, and formats the result with a string routine.
- `prepare_format_argument_buffer`: Builds a stack argument block from a parameter field and passes it, with a template string, to a string-formatting routine.
- `measure_null_terminated_length`: Counts the bytes of a null-terminated string from the given address and returns the count without the terminator.
- `count_bytes_until_null`: Register-based variant that advances an index from a base address until it reaches a zero byte, then returns that index as the string length.
- `copy_c_string_to_buffer`: Copies bytes from a source string to a destination buffer one at a time, including the terminating zero byte, and stops after writing it.
- `copy_fixed_byte_count`: Copies a fixed number of bytes from a source address to a destination address, one byte at a time, using the count passed in.
- `copy_bytes_until_count`: Register-based byte copy that moves bytes from one buffer to another until a counter reaches the limit held in a register.
- `compare_null_terminated_strings_2`: Compares two null-terminated strings character by character, stopping at the first difference or at a terminator, and returns whether the final characters match.
- `match_strings_at_terminator`: Register-based string comparison that advances through two strings together and stops at the first zero byte in either one.
- `count_bytes_including_terminator`: Counts bytes from the given address up to and including the terminating zero byte and returns that count.
- `issue_three_library_calls`: Calls a subroutine to obtain a value, then issues three library calls through a library base held in a structure.
- `fill_local_buffer_and_forward`: Fills a 12-byte local buffer from a global source through a subroutine, then forwards the five arguments together with that buffer to a second subroutine.
- `copy_length_prefixed_string`: Copies a length-prefixed string into a destination buffer byte by byte and appends a zero terminator.
- `copy_bytes_and_append_zero`: Register-based loop that copies bytes from a source to a destination up to a limit, then writes a zero terminator after the copied bytes.
- `initialise_two_records_and_run`: Fills two structures with field values taken from its parameters, including a pointer linking the second record to the first, then calls two subroutines.
- `invoke_member_callback_return_one`: Calls a subroutine with a function pointer read from a structure member at offset 62 and then returns one.
- `issue_offset_adjusted_library_calls`: Issues three calls through a library base stored in a structure: one with the second argument, one with zero, and one with the third argument plus two values derived from the fifth and sixth parameters, each reduced by ten.
- `call_raw_format_routine`: Library stub that calls the system RawDoFmt routine through a global base pointer.
- `store_byte_at_pointer`: Writes the low byte of a data register to the address held in a pointer register.
- `call_library_offset_minus_36`: Library stub that calls the library function at offset minus 36 from a base pointer read from a global table.
- `call_library_offset_minus_72`: Library stub that calls the library function at offset minus 72 from a base pointer read from a global table.

### version.library

- `stub_entry_return_zero`: Program entry routine that returns zero immediately, with no other operations.
- `store_two_fields_in_record`: Stores two register values into fixed fields, at byte offsets 34 and 38, of the record addressed by a data register, then returns.
- `increment_counter_clear_flag_bit`: Increments a 16-bit counter at one fixed offset of a record and clears bit three of a flag byte at another offset.
- `decrement_counter_call_helper`: Decrements a 16-bit counter at offset 32 of a record.
- `finish_counter_and_call_library`: When the counter at offset 32 is zero, reads a value from offset 38 and calls two library functions through the base pointer at offset 34, then returns that value.
- `load_value_and_call_libraries`: Reads a value from offset 38 of a passed record, calls two library functions through a library base held in a register, and returns the value.
- `stub_return_zero_handler`: Routine that returns zero immediately and performs no other work.

### Populous (1989)(Electronic Arts)[cr VF][a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialise_hardware_and_graphics`: Start-up routine that clears a block of memory, writes initial values to the interrupt-enable, DMA-control and copper-pointer registers, and waits on hardware status bits.
- `locate_character_and_fill_table`: Walks a text table of letters, digits and punctuation one character at a time, using a step of two per character, to find the index of a given character.
- `run_blitter_draw_loop`: Updates two wrapping accumulators, then programs the blitter control, mask, pointer and modulo registers in a nested loop.
- `toggle_bit_then_wait_loop`: Toggles one bit, chosen by the low three bits of a data register, in a byte at the address held in a pointer register.
- `mask_word_then_wait_loop`: Keeps only bits ten and eleven of a 16-bit word at a fixed offset from the address register and then runs the thirty-step wait loop.
- `run_thirty_step_wait_loop`: Runs thirty iterations, each calling one subroutine and then calling a second subroutine repeatedly until it returns zero.
- `start_blitter_copy_setup`: Waits for the blitter busy flag to clear, then sets the blitter control to a plain copy using all four channels with zero shift and clears one modulo register.
- `fill_block_from_table_count`: Looks up a count in a byte table for the index in a data register, or uses sixteen when the index is zero.
- `fill_longs_with_constant`: Fills a run of longwords at the destination address with a fixed constant, with the number of entries taken from the data register.
- `poll_until_helper_returns_zero`: Repeatedly calls a subroutine and loops until the subroutine returns zero.
- `store_sum_through_pointer`: It adds two values held in registers and stores the result through a pointer argument.
- `swap_long_and_split_halves`: Exchanges a 32-bit value between two globals, then stores the low and high words of the value taken from the first global into two further globals.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 32 into d5, then merges 192 into d5.
- `swap_long_and_split_halves_2`: Exchanges a 32-bit value between two globals, then stores the low and high words of the value taken from the first global into two further globals.
- `setup_four_audio_channels`: Programs the four audio channel register blocks in turn: each gets the same sample pointer, a length of thirteen words and a volume of 64, and a period that rises by four per channel from 2000.
- `increment_strided_byte_triplets`: Adds one, two and three to three bytes at offsets 0, 8 and 16 of each 24-byte record, walking 110 records from a fixed starting address.
- `initialise_two_lookup_tables`: Fills 140 six-longword entries with fixed longword constants, then fills 110 eight-byte entries with a counting pattern.

### Populous (1989)(Electronic Arts)[cr VF][a].adf:DAD (raw code file, load address unknown)

- `initialize_hardware_and_graphics`: Modifies the interrupt enable register, performs serial communication, and accesses graphics and intuition libraries.
- `toggle_interrupt_enable_2`: Modifies the interrupt enable bit in the hardware interrupt enable register and processes a string.
- `process_petersint_string`: Processes a routine that references the text petersint.
- `execute_short_routine`: A short routine consisting of ten instructions.
- `handle_serial_communication`: Performs serial reading and serial writing operations.
- `toggle_interrupt_enable`: Modifies the interrupt enable bit in the hardware interrupt enable register.
- `execute_minimal_wrapper`: Executes twelve instructions and calls a short six-instruction routine.
- `execute_minimal_routine`: A very short routine consisting of six instructions.
- `access_graphics_intuition`: Executes thirty-four instructions, references graphics and intuition libraries, and calls a short six-instruction routine.
- `execute_medium_routines_2`: Calls several medium-length routines and processes a short sequence.
- `process_logic_sequences_2`: Executes various medium, large, and minimal routines while processing logic sequences and referencing a text string.
- `execute_medium_routines`: Executes one hundred fifty-two instructions and calls two medium-length routines.
- `execute_complex_routine_3`: Calls a large complex routine within a short sequence of instructions.
- `execute_complex_routine_2`: Executes fifty-six instructions and calls a large complex routine.
- `execute_complex_routine`: A large routine consisting of one hundred twenty-seven instructions that calls another routine.
- `execute_medium_routine`: A routine consisting of one hundred three instructions.
- `execute_medium_routine_2`: A routine consisting of ninety-five instructions.
- `execute_large_routine`: Executes six hundred ninety-three instructions and calls a medium-length routine and a minimal task routine.
- `execute_medium_routine_3`: A routine consisting of sixty-two instructions.
- `execute_medium_routine_4`: A routine consisting of thirty-seven instructions.
- `execute_subroutine_call`: Performs a single subroutine call and consists of one hundred twenty-seven instructions.
- `execute_minimal_task_wrapper`: Executes forty instructions and calls a minimal task routine.
- `process_complex_logic`: Executes a sequence of three hundred forty-five instructions and is called by one routine.
- `perform_minimal_task`: Executes eight instructions and is called by two different routines.
- `run_short_sequence`: Contains seventy-three instructions and is invoked by a single routine.
- `process_logic_sequences`: A large routine that executes numerous logic blocks, sequences, and frequent tasks while referencing a specific text string.
- `manage_various_logic_blocks`: Executes a large number of instructions involving multiple frequent and extended logic tasks.
- `handle_medium_routine`: Processes two hundred fifty-four instructions within a single call chain.
- `execute_small_block_wrapper`: Executes five hundred five instructions and calls a sixty-instruction routine.
- `execute_small_block`: Consists of sixty instructions and is called by one routine.
- `run_intermediate_logic`: Executes ninety-two instructions and is triggered by one routine.
- `execute_logic_sequences`: This routine consists of sixty-nine instructions and calls two different logic processing routines.
- `manage_frequent_task`: Performs one hundred twelve instructions and is called by six different routines.
- `execute_extended_logic`: Contains one hundred seventy-eight instructions and is called by four routines.
- `manage_task_chains_2`: Invokes a frequent task routine and an extended logic routine.
- `manage_task_chains`: This routine contains three hundred twenty-eight instructions and invokes a frequent task routine and an extended logic routine.
- `invoke_frequent_task`: This routine consists of eighty-six instructions and calls a routine that is used by six different callers.
- `process_subroutine_logic`: This routine contains two hundred seventy-nine instructions and calls a routine that is invoked by four different callers.
- `execute_complex_tasks`: This routine consists of two hundred fifty-eight instructions and calls a complex routine, a single-call subroutine, and a frequent task routine.
- `process_large_routine`: Executes three hundred fifty-seven instructions and is called by one routine.
- `execute_complex_tasks_2`: Performs a large sequence of instructions including several complex and frequent task subroutines.
- `execute_subroutine_logic`: A routine consisting of 223 instructions that calls another routine and is invoked by four different callers.
- `process_short_sequence`: A routine containing 85 instructions that is called by one other routine.
- `execute_extended_logic_2`: A routine consisting of 213 instructions that is called by one other routine.
- `perform_intermediate_task`: A routine containing 145 instructions that is called by two different routines.
- `execute_medium_sequence`: A routine consisting of 120 instructions that is called by one other routine.
- `process_logic_block`: A routine containing 138 instructions that is called by one other routine.
- `execute_brief_sequence_2`: This routine contains two hundred five instructions and calls a brief sequence routine.
- `execute_brief_sequence`: A routine consisting of 67 instructions that is called by one other routine.
- `execute_large_sequence`: A routine containing 266 instructions that is called by one other routine.
- `perform_complex_logic`: A routine consisting of 376 instructions that is called by four different routines.
- `call_instruction_block`: This routine consists of one hundred fifty-nine instructions and is called by three different routines and calls a fifty-instruction routine.
- `manage_file_operations`: Executes complex logic and library calls while referencing text strings related to saving, loading, and file names.
- `execute_short_task`: A routine containing 71 instructions that is called by one other routine.
- `handle_hardware_communication_2`: Performs complex logic and memory transfers while accessing serial data, custom hardware registers, and interrupt registers.
- `handle_hardware_communication`: This routine contains one hundred seventy-four instructions and performs complex logic, memory transfers, and accesses serial data and interrupt hardware registers.
- `execute_fifty_instructions`: Performs fifty instructions and is called by four different routines.
- `process_game_state_logic`: Executes complex logic and instruction blocks while referencing text strings related to starting or loading a game.
- `undefined_routine`: This routine consists of two hundred seventy-three instructions and is invoked by two unnamed routines.
- `execute_short_sequences_2`: Calls a routine that executes a series of instruction sequences and short routines.
- `execute_short_sequences`: Executes a series of instruction sequences and short routines.
- `execute_twenty_three_instructions`: Performs twenty-three instructions and is called by one routine.
- `execute_short_sequence`: This routine consists of two hundred forty-six instructions and calls a routine containing thirty-eight instructions.
- `execute_thirty_eight_instructions`: Performs thirty-eight instructions and is called by one routine.
- `perform_library_calls`: This routine contains ninety-six instructions and executes multiple library calls using various negative offsets.
- `execute_twenty_instructions`: Performs twenty instructions and is called by one routine.
- `transfer_memory_block`: Copies a sequence of bytes from one memory location to another using a register as a counter.
- `process_game_text_2`: Executes a routine that references a specific text string.
- `execute_sixteen_instructions`: Performs sixteen instructions and is called by one routine.
- `process_game_text`: This routine processes a string of text referenced from a data segment.
- `manipulate_interrupt_registers`: Performs twenty instructions and accesses the interrupt request and interrupt request mask registers.
- `access_interrupt_and_custom_registers`: Performs twenty-two instructions and accesses the interrupt request register and a custom hardware register.
- `execute_forty_nine_instructions`: A routine consisting of forty-nine instructions called by one other routine.
- `access_serdat_register`: A routine of forty-six instructions that accesses the serial data hardware register and is called by two other routines.
- `call_library_offset_negative_102`: A four-instruction routine that performs a library call with an offset of negative one hundred and two.
- `call_multiple_library_offsets`: A five-instruction routine that performs library calls with offsets of negative one hundred and eight and negative fifty-four.
- `call_library_offset_negative_132`: A three-instruction routine that performs a library call with an offset of negative one hundred and thirty-two.
- `call_five_library_offsets`: An eighteen-instruction routine that performs library calls with offsets of negative thirty, negative forty-two, negative sixty, negative sixty-six, and negative eighty-four.
- `call_library_offset_negative_90`: A four-instruction routine that performs a library call with an offset of negative ninety.
- `call_multiple_libraries_2`: Performs several library calls using various offsets and multiple library interfaces.
- `call_library_offset_negative_354`: A three-instruction routine that performs a library call with an offset of negative three hundred and fifty-four.
- `call_five_library_offsets_large`: An eighteen-instruction routine that performs library calls with offsets of negative one hundred and thirty-two, negative one hundred and ninety-eight, negative two hundred and ten, negative two hundred and ninety-four, and negative three hundred and thirty.
- `call_two_library_offsets`: A six-instruction routine that performs library calls with offsets of negative three hundred and thirty-six and negative three hundred and seventy-two.
- `call_multiple_libraries`: Executes nineteen instructions and makes three library calls.
- `call_seven_libraries`: Executes twenty-three instructions and makes seven library calls.
