# Game 2009 (populous (1989)(electronic arts)[cr qtx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02009). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 34 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 350 raw-word lines |
| Functions named and described | pass, 525 of 525 named (0 author symbols), 525 of 525 described (80 by rule, 59 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 708 of 708 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 11131 of 11898 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 619427 |
| code | 38826 |
| data | 192059 |
| padding | 9848 |
| unused | 40960 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| type | 2 | 117 | 2 |
| endcli | 2 | 117 | 2 |
| loadwb | 12 | 536 | 30 |
| wait | 2 | 117 | 2 |
| Disk-Validator | 1 | 386 | 1 |
| Populous (1989)(Electronic Arts)[cr QTX].adf:populous | 3 | 4352 | 145 |
| Populous (1989)(Electronic Arts)[cr QTX].adf:populous.prg | 3 | 1760 | 9 |
| icon.library | 10 | 1375 | 109 |
| translator.library | 2 | 490 | 50 |
| info.library | 25 | 2597 | 167 |
| version.library | 2 | 41 | 7 |
| Populous (1989)(Electronic Arts)[cr QTX].adf:bootblock | 1 | 10 | 1 |

## Cases handled

- populous (1989)(electronic arts)[cr qtx]/Populous (1989)(Electronic Arts)[cr QTX].adf: blocks 11-21 (bytes 5632-11264) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.70)
- populous (1989)(electronic arts)[cr qtx]/Populous (1989)(Electronic Arts)[cr QTX].adf: blocks 88-96 (bytes 45056-49664) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.99)
- populous (1989)(electronic arts)[cr qtx]/Populous (1989)(Electronic Arts)[cr QTX].adf: blocks 99-109 (bytes 50688-56320) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.48)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: If the stack argument pointer is not null, copies 16 longwords from a fixed block in the structure into it.

### endcli

- `setup_command_shell_buffer`: Walks a list of entries to find the largest stored value, allocates a buffer sized from it and fills it with an increasing series of values.
- `copy_sixteen_longs_then_call_2`: Copies 16 longwords from a fixed block in the structure into the buffer passed on the stack, then calls a routine through a register pointer and returns.

### loadwb

- `start_workbench_loader`: Main program entry of the Workbench loader.
- `release_task_resources_and_return`: Stores the argument in a slot of a shared structure, then conditionally calls cleanup routines that close a file and a library, free memory and reply to a message, with the task locked while it works.
- `start_resident_workbench_tool`: XORs a global byte string with a fixed key in two passes, which cancels out, then allocates a buffer and fills it with the decoded text.
- `compare_strings_ordered`: Compares two zero-terminated strings byte by byte.
- `build_linked_list_from_table`: Walks a table of entries, allocating an 8-byte node for each and copying a table value into it, then linking the nodes in order.
- `call_library_vector_96_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_366_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `compare_release_text_and_pad`: Compares a version string with a global string matching the referenced text Workbench release 1.2, and when the version digit is two and the strings match it writes a dot and four spaces into the buffer.
- `write_dot_and_spaces`: Writes a period followed by four space characters at fixed positions in a buffer.
- `compare_prefix_with_global`: Compares a string in a register with a global string until they differ or the global string ends.
- `empty_routine_2`: The routine contains four instructions and performs no observable operation; it ends with a return.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_with_stack_slot_pointer`: Calls another routine with the two arguments it received plus a pointer to a fixed stack slot.
- `format_buffer_and_pass_length`: Calls a library formatting function through a global base, then measures the length of a 140-byte stack buffer up to its terminator.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### wait

- `find_max_and_prepare_buffer_2`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header into a buffer and copies sixteen longwords from a fixed block into a caller-provided stack location.
- `copy_longs_then_call`: Copies a fixed number of longwords from a source address into a destination address pointed to by a register, then calls another routine.

### Disk-Validator

- `run_disk_validator_program`: Program entry of the disk validator, shown here only in part because the listing is truncated.

### Populous (1989)(Electronic Arts)[cr QTX].adf:populous

- `clear_state_and_start_game`: Clears a block of about 28 KB of the program state, stores a library base and a global pointer, and calls a library function when a flag bit is set.
- `initialize_demo_resources`: Stores handles obtained from library calls into globals, allocates a block of 300 items of size 2 and fills several fields of a structure from register values.
- `start_copper_list_from_field`: Passes a pointer read from a structure at a fixed offset to a routine that points the copper at a new copper list by writing the list address to the COP1LC register and strobing COPJMP1 so the list starts at once.
- `manage_interrupt_enable_flags`: Calls a series of routines with constant values, including a value for the interrupt enable register INTENA where a set top bit enables interrupts and a cleared top bit disables them.
- `create_program_process`: Creates a 40-byte structure, fills its fields from a global value and a fixed name, then calls library routines that create a process and load a program segment.
- `setup_memory_table`: Allocates a block of 1152 bytes and stores its address in a global.
- `store_constant_and_call`: Stores the constant 399 as a word in a global variable through a register-based pointer, then calls one routine.
- `call_routine_from_table`: Passes the block allocated earlier, of size 1152 bytes, to a routine.
- `call_two_global_pointers`: Calls one routine with each of two global pointer values, one after the other.
- `call_library_vector_300`: Extracts a field from its argument and calls the library function at offset minus 300 of a library base read from a global, passing that field value.
- `forward_call_to_routine`: Calls one routine with no argument and returns nothing.
- `find_matching_entry_and_call`: Searches a two-entry table in the globals for the argument.
- `swap_globals_and_call`: Swaps two 32-bit values stored in globals, then calls one routine with the swapped values.
- `initialize_graphics_resources`: Obtains a base from one routine and stores it in a field, then requests two resources with fixed numbers and makes a further call when either is zero.
- `wait_for_code_then_dispatch`: Repeatedly calls a routine until it returns one, two or three, then passes a global and a pointer to a fixed offset to a further routine.
- `wait_and_classify_state`: Clears a global longword, invokes several helper routines, waits in a bounded loop for a flag to clear, then classifies a pair of shorts into one of three outcomes stored in a global.
- `initialize_game_state`: Stores two function pointers in globals, writes three constants, and clears a short array until a zero entry is found.
- `update_game_tick_entries`: Runs one tick of updates: it toggles a flag, decrements two timers that reload at 50, and processes 16 entries from a byte table by calling routines for each.
- `return_without_action`: Returns immediately without doing anything.
- `run_offset_update_loop`: Fills 18 slots of a table with values masked to three bits, then loops calling routines with positions offset from 160 or 168 minus stored values until an abort byte is set or a counter reaches zero.
- `fill_short_arrays_modulo`: Loops over the count given in its argument, filling one 16-bit array with values from 232 to 244 chosen by a routine's result modulo 13, and a second array with values from 0 to 8 chosen modulo 9.
- `update_short_arrays`: Loops over the count given in its argument, increasing each entry of one array by one and subtracting it from the matching entry of another.
- `process_demo_pic_sequence`: Makes a sequence of library and routine calls, one of them with a fixed value, then repeats a loop that calls routines while a routine returns nonzero and the abort flags are clear.
- `initialize_dma_list_entries`: Fills four list entries with offsets from a base pointer, stepping by 300 for one table and by 32000 for the other, and calls a routine for each entry.
- `read_block_then_release`: Obtains a handle through a routine with code 1005, reads 32000 bytes into a global buffer through the read wrapper, copies a second region with another routine and releases the handle.
- `load_game_data_files`: Opens three data files named in the referenced text font.dat, land0 and sprites0.dat, reading several blocks of fixed byte counts from each one and closing each handle.
- `load_optional_file_blocks`: Opens a file through a routine with code 1005.
- `rotate_and_combine_word_table`: Rotates a set of 16-bit words by an amount taken from a nibble of the parameter and combines the results into four interleaved planes of a destination buffer using AND and OR. It repeats this for 32 rows and runs only when the row argument is in range.
- `apply_rotated_mask_to_tiles`: Uses a value derived from its input to pick a 32-bit mask from a fixed table, rotates it by a nibble amount, then ANDs and ORs the rotated words into four planes of the destination for 16 rows.
- `call_routine_and_return_pair`: Calls one routine and returns the two data registers unchanged as a combined 64-bit value.
- `apply_tile_mask_table`: Applies a mask table entry, indexed by a global and a register value, to four planes of destination words.
- `reorder_struct_fields`: Walks 1680 entries of 20 bytes in a global table and rotates fields between them, moving pairs of 16-bit words and bytes within each entry.
- `shift_byte_buffer`: Shifts each of the 100 bytes of a fixed buffer left by three bits.
- `place_grid_cells_with_offsets`: Loops over a 10 by 10 grid, calling a routine with coordinates computed from the row and column and an offset table entry.
- `concatenate_registers`: Calls one routine, then returns the two data registers combined into one 64-bit value.
- `set_global_pointer_to_buffer`: Stores the address of a global buffer in a global pointer variable.
- `initialize_buffer_from_string`: Fills 80 longwords with minus one and then 320 longwords with zero.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `open_library_and_set_defaults`: Opens a library through a global function pointer and, if that fails, runs two cleanup calls through the library base and returns.
- `open_library_and_set_defaults_2`: Same body as the routine above; the listing repeats the open-and-set-defaults code, so this copy carries no separate logic.
- `stub_single_instruction_entry`: Single-instruction entry whose decompiled body repeats the open-and-set-defaults code.
- `dispatch_handler_by_status`: Dispatches through a table of handler pointers, calling one of several setup routines based on a status value and configuring a set of global pointers.
- `dispatch_handler_by_status_2`: Identical handler-dispatch routine that branches on a status value to invoke one of several setup routines and configure global pointers.
- `load_popsam_file`: The pseudo-code in the job is a copy of the open-and-set-defaults body, so this name rests on the facts list.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_byte_and_return`: Stores a zero byte at a fixed offset from the address register, then returns the eight-byte value passed on the stack unchanged.
- `restore_interrupt_state_and_free`: Sets two globals to 16383, calls a routine and a library function at minus 210 through the base, then writes saved globals back, with the first one OR-ed with 49152.
- `dispatch_code_then_clear_byte`: Calls a routine when the code byte is ten or one, then clears the byte at a fixed offset of the structure and returns the stack value unchanged.
- `clear_byte_after_code_one`: Calls a routine when the code byte is one, then clears the byte at a fixed offset of the structure and returns the stack value unchanged.
- `clear_status_byte_and_return`: Clears the byte at a fixed offset of the structure and returns the stack value unchanged.
- `clear_status_byte_and_return_2`: Clears the byte at a fixed offset of the structure and returns the stack value unchanged.
- `clear_byte_at_offset`: This routine stores a zero byte at a fixed offset from an address register, then returns the eight-byte value passed on the stack unchanged.
- `clear_byte_and_return_2`: Writes a zero byte at a fixed offset from a register address, then returns the eight-byte value passed on the stack unchanged.
- `service_interrupt_flag_bit`: Tests bit five, the value 32, of a global flag and, when it is set, writes 255 to a hardware-related address, stores a masked value back in the flag and calls one routine.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `build_handler_vector_table`: Walks a linked list building derived entries in a table, then installs a vector table and a per-entry handler chain into fixed memory locations.
- `build_dispatch_tables`: This routine builds dispatch tables by reading values from a global variable area and writing computed pointers and constants into memory, then initializes additional tables and global variables.
- `reset_global_flags`: Clears one byte and writes the values 15 and 255 to two globals, then returns.
- `step_sound_channel_sequences`: Advances four audio channel sequences by one tick, reading note bytes and control codes for instrument select, slide, jump and end.
- `resume_channel_sequence_stepping`: Re-enters the channel sequence loop at the point where the next channel's note byte is read, writing audio channel registers and DMA enable bits when a note starts and stopping when the channel counter underflows.
- `start_channel_sequences_and_step`: Clears one channel's sequence state, reads its first note and stores the sequence pointer, then steps every remaining channel until the channel counter underflows, writing audio registers as notes start.
- `load_instrument_and_start_note`: Selects an instrument from a table using the byte at the sequence pointer, stores its envelope pointers in the channel state, then reads the following note and continues the channel stepping loop.
- `start_first_note_and_step`: Starts the first note on a channel by scaling its period from the instrument table, then steps all channels and writes their audio registers whenever a note begins.
- `continue_channel_note_stepping`: Continues the per-channel note loop without setting up a new note, reading the next sequence byte and writing audio channel registers when a note begins.
- `set_channel_slide_and_step`: Stores a per-tick slide value for the channel, sets its slide-enable flag, and continues the note-stepping loop across all channels.
- `reload_channel_sequence_and_step`: Reads the next sequence address from the channel's pointer table, restarting the table when its entry is zero, then reads the next note and continues stepping all channels.
- `start_note_with_scaled_period`: Computes a note period by multiplying a note value by a per-channel scale byte, stores it with the instrument parameters, and writes sample address, length, period and volume to the audio registers before enabling DMA.
- `update_channel_pitch_and_volume`: Each tick adds the channel slide to its audio period and applies an optional modulation step, writes the period to the audio hardware, and reloads a duration countdown when it reaches its limit.
- `clear_channel_flag_and_step`: Clears a per-channel state byte and stores its index in the channel table, then continues the stepping loop over all channels.
- `decrement_channel_counter_and_clear`: Decrements a channel counter and, while it is within the limit, clears the channel's bit 2 flag and zeroes its state byte; otherwise it only zeroes the state byte.
- `reset_channel_counter_and_step`: Writes the value 60 into a per-channel counter byte, stores its index, and continues stepping all channels.
- `merge_masked_bitplane_words`: Loops over a block of source words and writes four bitplane words per iteration, where each destination bit is kept or replaced according to a selector word taken from a global table.
- `mask_destination_bitplane_words`: For each of 160 words, forms a mask from the source word and a global table entry, clears the four destination plane words under the mask, then ORs in the matching source plane bits.
- `store_stack_word_and_call`: Copies a 16-bit argument from the stack into a global variable, calls another routine, and returns the data and address registers exactly as that routine leaves them.
- `forward_call_to_routine_2`: The decompiled body only calls another routine and returns, so this listing shows no other effect of its own.
- `update_mouse_position_clamped`: Computes signed horizontal and vertical changes from the mouse counter bytes, adds them to the stored position, and clamps the horizontal position to 0 through 639 before halving it.
- `store_mouse_counters_and_call`: Splits a 16-bit state word into its low and high bytes, stores them in two global counter variables, and then calls another routine.
- `save_state_and_enable_interrupts`: Saves a 16-bit state word in two global variables and writes a fixed interrupt enable mask to the interrupt enable register.
- `restore_saved_interrupt_enables`: If a saved flag is set, disables all interrupts, writes back the saved interrupt enable word with its set bit, clears the flag and returns the restored word; otherwise it returns zero.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `build_copper_display_list`: Fills a copper instruction list with display window and fetch settings, a 32-entry colour register table, bitplane modulo values, one pointer per bitplane and sprite pointers.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `set_copper_pointer_then_wait`: Stores the given address as the copper list pointer, writes its high and low words to the copper location register, then polls the interrupt request read register until bit five is set and returns that masked value.
- `request_and_wait_interrupt_bit`: Writes the bit five value to the interrupt request register and polls the read-back register until that bit reads as set, then returns the masked bit.
- `patch_copper_colour_entries`: Scans a copper list for the colour register marker word, then writes a counted block of source words into the entries that follow the marker.
- `report_decompile_timeout`: No pseudo-code was produced because the decompiler timed out, so only its call relationships are known: it has 21 instructions, is called by one routine and calls one routine.
- `update_record_position_bytes`: Looks up a record by index and writes its start and end coordinate bytes, a flag byte and their mirrored copies.
- `decode_keyboard_key_press`: Turns a keyboard serial byte into an index into a key table of text such as digits and symbols, storing the character once per press and clearing its pending flag on release.
- `copy_eight_thousand_longwords`: Copies 8000 32-bit values from a source address to a destination address, one after the other.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `step_nibbles_after_raster_wait`: Copies a block of words into a stack buffer, then repeatedly waits for a raster position and decrements each nibble of those words, calling another routine on each pass until every nibble is zero.
- `call_routine_then_return_result`: Calls one routine with no arguments, then calls a second routine with the two parameters and returns that second routine's result.
- `call_routine_then_optional_callback`: Calls a routine with its argument, then calls a second routine only when a global value is nonzero.
- `store_offset_return_high_bits`: Calls a routine, adds the fixed offset 12345 to its result, stores the sum in a global variable, and returns the upper 15 bits of that stored value.
- `store_high_word_to_global`: Writes the upper 16 bits of its argument into a global variable and returns.
- `stub_return_immediately`: Empty routine that performs a single return and no other operation.
- `allocate_and_initialise_handle_record`: Allocates a record sized from a table entry, initialises its flags and fields, writes a four-character tag into it and looks up a second record.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `build_argument_text_record`: Copies a text template into a buffer, sets the record's index field and performs a list of library and wrapper calls that set up its name and data pointers.
- `append_string_bounded`: Appends a source string to the end of a destination string, copying at most 32767 characters, and writes a terminator if the copy was cut short.
- `append_string_with_limit`: Finds the end of a destination string, then appends up to a counted number of characters from a source string, writing a terminator when the source was longer than the count.
- `copy_string_padded_to_count`: Copies a source string into a fixed-size field, stopping at its terminator or after the count, then fills the rest of the field with zero bytes.
- `multiply_low_32_bit_products`: Multiplies two 32-bit register values using their 16-bit halves and returns the low 32 bits of the product in one register, leaving the second operand in the other.
- `open_named_libraries_and_resources`: Opens a library named by a global string, looks up a second structure from the argument, and opens further named resources such as a window and the icon library.
- `call_optional_pointer_then_return`: Calls a stored function pointer if it is set, then calls another routine and returns.
- `call_optional_pointer_then_routine`: Calls a stored function pointer if it is set, then calls another routine.
- `release_program_resources`: Calls the stored release routines when they are set, runs the library release calls for each handle that is present, and writes a saved value through a pointer.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_indexed_handle_entry`: Checks the index against the table size and that its handle is nonzero, closes the handle unless the entry's flag bit is set, then clears the entry and returns zero.
- `call_library_close_vector`: Calls a library routine at a fixed vector offset from a stored library base, the same mechanism the close wrapper uses.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_load_seg`: Wrapper that calls the dos library function LoadSeg and returns its result to the caller.
- `call_library_open_vector`: Calls a library routine at a fixed vector offset from a stored library base, the same mechanism the open wrapper uses.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_via_pointer`: Calls a library routine at a fixed vector offset from a stored library base pointer, returning nothing.
- `call_library_vector_tail`: Entry point that enters the middle of the library vector call sequence and calls the routine at a fixed offset from a stored library base.
- `call_library_vector_minus_198`: Calls the library routine stored 198 bytes below the library base held in a global variable, passing on its result.
- `enter_library_vector_minus_198`: Enters the same library call sequence as the previous entry, calling the routine 198 bytes below the stored library base.
- `call_library_vector_minus_294`: Calls the library routine stored 294 bytes below the library base held in a global variable, passing on its result.
- `enter_library_vector_minus_294`: Enters the same library call sequence as the previous entry, calling the routine 294 bytes below the stored library base.
- `call_library_vector_minus_132`: Calls the library routine stored 132 bytes below the library base held in a global variable, passing on its result.
- `enter_library_vector_minus_132`: Enters the same library call sequence as the previous entry, calling the routine 132 bytes below the stored library base.
- `call_library_vector_minus_210`: Calls the library routine stored 210 bytes below the library base held in a global variable, the same routine the library call wrapper for that offset uses.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_minus_372`: Calls the library routine stored 372 bytes below the library base held in a global variable, passing on its result.
- `call_library_vector_minus_552`: Calls the library routine stored 552 bytes below the library base held in a global variable, passing on its result.
- `enter_library_vector_minus_552`: Enters the same library call sequence as the previous entry, calling the routine 552 bytes below the stored library base.
- `call_library_vector_minus_138`: Calls the library routine stored 138 bytes below the library base held in a global variable, passing on its result.
- `call_library_vector_minus_366`: Calls the library routine stored 366 bytes below the library base held in a global variable, passing on its result.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_300_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_library_vector_minus_384`: Calls the library routine stored 384 bytes below the library base held in a global variable, passing on its result.
- `call_library_vector_462_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_minus_96`: Calls the library routine stored 96 bytes below a library base pointer held in a different global variable, passing on its result.
- `call_library_vector_90_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_minus_78`: Calls the library routine stored 78 bytes below a library base pointer held in a global variable, passing on its result.

### Populous (1989)(Electronic Arts)[cr QTX].adf:populous.prg

- `save_registers_then_halt_stub`: Program entry that copies the incoming register contents into global variables, then reaches an unimplemented instruction, so the decompiled control flow stops there.
- `save_frame_then_apply_relocations`: Saves the incoming stack pointer and a set of registers into a frame, then walks a table of offset blocks and adds a base value to the longword at each listed offset until a zero entry ends the block.
- `apply_relocations_from_table`: Walks a table of offset blocks and adds a base value to the longword at each listed offset, ending each block at a zero entry.
- `apply_relocations_before_max_scan`: Applies the same offset fixup loop and 13-word copy as its neighbour.
- `copy_words_to_low_memory`: Copies 13 16-bit words from a fixed code location to a fixed low memory address.
- `spin_delay_counter`: Busy-waits by decrementing a register counter until it reaches zero, then returns.
- `copy_block_then_scan_list`: Copies the same 13-word block to a fixed low memory address as its neighbour.
- `copy_longwords_then_zero_fill`: Copies 32-bit values from a source region into a destination region until the destination reaches a limit, zero-fills the remainder, saves the register values into fixed globals and returns the stacked value.
- `report_large_routine_decompile_timeout`: No pseudo-code was produced because the decompiler timed out on this 403-instruction routine, so only its callers and instruction count are known.

### icon.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `increment_counter_clear_flag_bit`: Library code that increments a 16-bit counter in a structure and clears bit three of a flags byte in the same structure.
- `decrement_counter_release_if_flagged`: Library code that decrements a 16-bit counter in a structure.
- `unlink_node_or_mark_pending`: If a count field is zero, removes the structure from a doubly linked list and calls a library vector 210 bytes below the library base.
- `set_pending_flag_bit`: Library code that sets bit three of a flags byte in a structure and returns zero.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `open_library_or_alert`: Stores its arguments in a structure and calls a library function 408 bytes below the library base.
- `copy_two_longs_to_globals`: Library code that copies two 32-bit values from a structure into two global variables.
- `forward_to_library_stub`: Calls another library routine and returns, with no other operation in its listing.
- `call_library_helper`: Empty library routine that calls a single helper and returns without performing any other operation.
- `forward_to_library_stub_410`: Library code that calls one other routine and then returns.
- `call_icon_data_copy`: Library routine that unconditionally calls a helper routine to copy icon data to the stack and then returns, accessing no registers or memory itself.
- `call_icon_init_routine`: Library routine that invokes a single initialization function to set up icon data structures and then returns, serving as a wrapper to trigger the setup process.
- `call_icon_data_copy_2`: Library routine that unconditionally calls a helper routine to copy icon data to the stack and then returns, performing no other operations.
- `call_library_handler`: This library routine simply calls another routine and then returns.
- `call_library_handler_2`: This library routine invokes another routine via a pointer stored in the library's data section and then returns.
- `call_library_helper_2`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `call_icon_init_routine_2`: This library routine invokes a single initialization function to set up icon data structures and then returns.
- `call_library_handler_3`: This library routine calls another routine and then returns.
- `call_library_handler_4`: This library routine invokes another routine by advancing a pointer by 78 bytes and passing it as the first argument, then returns without performing any other operations.
- `call_other_routine`: The routine calls another routine and then returns.
- `call_other_routine_2`: It invokes another routine and then returns without performing any other operations.
- `call_other_routine_3`: This routine invokes another routine and then returns.
- `call_vector_then_store_longword`: Library code that calls a routine through a function pointer taken from a fixed table entry, using its first parameter.
- `attempt_two_step_call`: Calls one routine and, when it returns a nonzero value, calls a second routine with that value.
- `fill_icon_structure_from_stack`: Library code that obtains a structure from one routine, fills it with a second routine, copies ten longwords from a stack buffer into it and stores several fields.
- `restore_icon_structure_from_stack`: Library code that copies ten longwords and several fields from a stack buffer back into a structure, then writes a back pointer into a linked structure when one is present.
- `return_zero_stub`: Empty library stub that returns zero without doing any other work.
- `build_icon_header_on_stack`: Library code that writes two fixed words into a stack header, copies ten longwords and several fields from a structure into it, then calls another routine with its first two arguments and a pointer to the header.
- `load_icon_info_file`: Library code that builds a file name with the info suffix in a 260-byte stack buffer, opens and reads the file through library calls, checks a magic word and version, then fills the icon structure.
- `read_icon_fields_checked`: Library code that checks the icon magic word and version, then reads the icon's buffers and fields through repeated library calls, stopping at the first failure.
- `read_icon_optional_fields`: Library code that reads further icon fields through library calls and stores them in the icon structure.
- `read_icon_word_array`: Library code that reads a counted series of words into an array through library calls, then reads one more field.
- `return_zero_after_call`: Calls another routine and returns zero.
- `empty_routine_4`: This routine contains four instructions and performs no observable work; it ends immediately with a return.
- `read_icon_records_into_buffer`: Library code that allocates a 260-byte buffer and reads the icon's records into it through library calls, then returns a status.
- `transfer_icon_fields_via_buffer`: Library code that allocates a 260-byte buffer, transfers the icon's fields and counted lists through library calls on a handle, and returns a status.
- `transfer_icon_fields_info`: Library code that transfers the icon's fields and counted lists through library calls in sequence, using the info suffix text.
- `finish_icon_field_transfer`: Library code that transfers the optional word lists and pointer fields of an icon structure, then restores a saved value and returns a status.
- `transfer_icon_pointer_table`: Library code that walks a null-terminated pointer table, transferring each entry and an optional extra entry through library calls, then restores a saved value and returns a status.
- `iterate_icon_table_entries`: Library code that walks a null-terminated table of pointers, calls a routine for each entry, updates a counter, and calls a callback while it may process a secondary entry.
- `walk_icon_node_list`: Library code that follows a chain of nodes, calling a routine on each one until the chain ends or returns to its head, then calls a routine on the last node found.
- `forward_call_with_offset_140`: Library code that calls another routine with the same first argument and the second argument advanced by 140 bytes, then returns.
- `allocate_icon_record_or_release`: Library code that allocates a 168-byte record, sets a field inside it, and calls routines to fill it.
- `forward_to_two_routines`: Library code that calls two routines in sequence and then returns.
- `return_zero_status`: Library routine that returns zero without other work.
- `store_pair_in_counted_slot`: Library code that decrements a counted slot index in a table header, asks a routine for a new block when the count runs out, then stores two longwords in the slot.
- `store_pair_in_next_slot`: Library code that decrements a counted slot index, stores two longwords in the table at that slot and returns one.
- `allocate_icon_node_record`: Library code that allocates a 94-byte record, links it through its fields, and calls routines to initialise it.
- `empty_library_stub_2018`: Empty library routine with a single return and no other operation.
- `call_routine_and_return_zero`: Library code that calls another routine and then returns zero.
- `forward_call_with_offset_78`: Library code that calls another routine with the same first argument and the second argument advanced by 78 bytes, then returns.
- `find_key_equals_value_entry`: Library code that walks a null-terminated list of string pointers and returns the text after the equals sign of the entry whose key matches the requested key, or zero when no entry matches.
- `scan_list_for_key_equals`: Library code that scans a null-terminated list of pointers for an entry matching the current key followed by an equals sign, and returns the text after the sign, or zero if none matches.
- `scan_pointer_list_for_key`: Library code that walks a null-terminated list of pointers and returns the text after the equals sign of the first entry whose key matches, or zero when the list has no match.
- `match_chain_entries_by_count`: Library code that walks a chain of entries, calling a routine on each, and returns one when a computed count matches the expected value and a final check passes, or zero when the chain ends without a match.
- `verify_chain_entry_count`: Library code that walks a chain of entries and returns one when the count of entries matches the expected value and the final check passes, or zero otherwise.
- `validate_chain_entry_sequence`: Library code that walks a chain of entries, checking each entry's computed count against the expected value and returning one when the chain is consistent, or zero when it ends first.
- `confirm_chain_count_match`: Library code that walks a chain of entries, comparing each computed count with an expected value.
- `search_chain_for_count_match`: Library code that searches a chain of entries for one whose computed count matches the expected value and passes a final check.
- `build_copy_name_for_entry`: Library code that builds the name for a copied item.
- `parse_copy_prefix_number`: Library code that checks for a three-character copy prefix and parses the decimal digits after it, then calls two routines to finish the update.
- `parse_decimal_digits_check`: Parses decimal digits at the current text position, then checks a four-character suffix and calls two routines to finish the update.
- `parse_decimal_suffix_for_copy`: Library code that parses a decimal number from text and, when it is nonzero and the four-character suffix check fails, calls routines to finish.
- `decimal_digit_value`: Converts one character code to its decimal digit value, returning minus one when the character is not a digit.
- `return_minus_one_stub`: Library routine that returns minus one without other work.
- `call_icon_vector_minus_198`: Library code that calls the routine 198 bytes below the icon library base stored in a global variable.
- `call_icon_vector_minus_210`: Library code that calls the routine 210 bytes below the icon library base stored in a global variable.
- `call_library_vector_228_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: Library code that calls a routine with four arguments and, when the result is nonzero, calls it again with the result.
- `allocate_icon_list_entry`: Library code that allocates a 96-byte record, stores a count of 10 in it and in its first argument, initialises the record through a routine, and returns whether the allocation succeeded.
- `empty_library_routine`: This library routine is empty and returns without performing any operations.
- `check_result_equals_argument`: Library code that calls another routine with three arguments and returns its result when that result equals the fourth argument.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `call_match_result_or_zero`: Library code that calls another routine with three arguments and returns its result when it equals the fourth argument, otherwise zero.
- `empty_routine_3`: This routine is empty and performs no work; it returns immediately without touching any data or registers.
- `divide_unsigned_quotient_remainder`: Unsigned division that returns the quotient and remainder in a register pair.
- `shift_subtract_division_loop`: Runs the 16-step shift and subtract loop used by the division routine on its register pair, leaving the partial quotient and remainder in place.
- `multiply_low_32_bits_swapped`: Multiplies two 32-bit register values and returns the low 32 bits of the product in the high register, with the second operand's halves swapped in the low register.
- `call_raw_format_vector`: Library code that calls a routine through a fixed vector 522 bytes below a library base held in a global variable.
- `store_byte_through_pointer`: Stores the low byte of the data register through a pointer register, then returns.
- `return_string_length`: Returns the number of characters before the null terminator of a byte string.
- `find_character_in_string`: Scans a null-terminated string for a given character and returns a pointer to its first occurrence, or zero when the character is not present.
- `do_nothing_routine`: This routine performs no operations and returns without touching any data or registers.
- `copy_c_string`: Library code that copies bytes from a source string to a destination until the zero terminator has been copied.
- `append_c_string`: Library code that moves to the end of the destination string, then copies the source string onto it including its terminator.
- `compare_strings_up_to_count`: Compares two byte strings for up to a counted number of characters.
- `return_one_stub`: Library routine that returns one without other work.
- `scan_limited_string_then_call`: Walks a string while decrementing a limit counter, stopping at the terminator or when the limit runs out.
- `forward_to_string_helper`: Calls another routine and returns, with no other operation in its listing.
- `copy_string_with_zero_pad`: Copies up to a counted number of bytes from a source string into a destination, stopping at the terminator and filling the rest of the count with zero bytes.
- `do_nothing`: This routine is empty; it contains no instructions and simply returns without performing any operation.
- `initialise_list_header_pointers`: Library code that points a two-pointer list header at itself.
- `process_icon_request`: Library code that returns one for a null argument.
- `process_linked_node_list`: Library code that walks a linked list of nodes, calling routines on each node's pointer and size fields.
- `check_all_list_nodes`: Library code that walks a linked list until its end, calling two routines on each node.
- `return_zero_value`: This routine returns a zero value without performing any visible work on its data.
- `allocate_icon_block_with_data`: Library code that allocates a 20-byte record and, when its flag argument is nonzero, allocates a data block and stores its pointer in the record.
- `return_nonzero_sentinel`: This routine returns a nonzero sentinel value without performing any visible work on its data.
- `build_linked_node_list`: Library code that loops over a count, allocating a 16-byte node per iteration and linking it to the previous one.
- `build_linked_node_chain`: Library code that allocates nodes in a loop, linking each one to the previous node and storing a pointer in each.
- `build_node_chain_from_arg`: Library code that builds a chain of nodes from a stack argument, allocating and linking each node until the chain ends.
- `return_empty_result`: This routine is an empty library function that immediately returns without performing any operations.

### translator.library

- `create_translator_library_base`: Library initialisation that allocates a library base through an exec routine, stores the segment and library pointers in it, and then calls another exec routine.
- `decrement_counter_clear_bit_five`: Library code that increments a 16-bit counter in a structure and clears bit five of a flags byte in the same structure.
- `decrement_counter_call_if_flagged`: Library code that decrements a 16-bit counter in a structure.
- `unlink_node_or_set_flag`: If a count field is zero, removes the structure from a linked list and calls a library vector 210 bytes below the base.
- `unlink_node_and_call_vector`: Removes the structure from a doubly linked list, calls a library vector 210 bytes below the base, then returns a value stored in the structure.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `parse_text_fields_into_buffer`: Library code that clears a 104-byte stack buffer to spaces, then scans input text for a hash terminator and space-separated numeric fields.
- `return_low_word_argument`: Returns the low 16-bit word of its data register as a signed value.
- `terminate_text_and_return_word`: Calls another routine, writes a zero byte just before the output pointer to end the text, and returns the 16-bit value from the data register.
- `scan_text_for_hash_space`: Scans a text string for a hash terminator and space delimiters while counting down a limit.
- `scan_text_tokens_until_hash`: Reads space-separated text tokens until it reaches a hash terminator or runs out of count, then terminates the output string and returns the last value.
- `copy_text_handling_marker_codes`: Copies text while checking for backslash and backtick marker characters, calling output routines for each character until the end of the text, then calls two routines to finish.
- `copy_text_until_marker_character`: Copies text characters until a backslash or backtick is found, calling output routines for each marker and returning when the end of the text is reached.
- `stub_immediate_return`: The decompiled body contains only a return, so the routine does no visible work in the library.
- `match_bracketed_pattern`: Scans text to an opening bracket, then compares the following characters with a pattern string until the closing bracket is reached.
- `scan_backward_class_match`: Scans text backwards while characters belong to a class in a lookup table, comparing them with a pattern string.
- `do_nothing_routine_2`: This routine contains no instructions and performs no operations.
- `scan_class_backward_then_call`: Scans backward through text while characters fall into a lookup class, calling another routine on each pass.
- `match_text_until_equals`: Scans text forward, comparing each character with a pattern string and calling a routine on classified characters.
- `scan_forward_to_equals_sign`: Scans forward through text until an equals sign, calling routines on classified characters and returning when a character fails to match the pattern.
- `copy_text_to_marker_width`: Copies text up to a backslash or backtick marker into an output buffer, checking it against a width limit.
- `copy_text_run_within_width`: Copies a run of text into an output buffer while checking a width limit.
- `copy_bytes_set_marker_flag`: Copies a counted run of bytes from source to destination, then sets bit zero of a state byte and clears it again when the following source byte is a backtick marker.
- `trim_output_to_class_boundary`: Moves backward from the end of the output while characters are not in a selected class, then writes a zero terminator at that boundary.
- `dispatch_symbol_character_handler`: Looks up a character in a table of twelve symbol characters and calls the handler stored at the matching index in a parallel table of routine pointers.
- `return_immediately_2`: This routine contains no instructions and performs no operations on any data.
- `stub_empty_return_slot`: Contains only a return in the decompiled output, so it is an empty stub with no visible effect on the library.
- `forward_to_library_stub_980`: Library code that calls another routine and then returns, with no other operation in its listing.
- `repeat_call_while_bit5_set`: Calls an unnamed library routine once, then calls it again for as long as bit 5 of the D4 register stays set.
- `call_single_helper_and_return`: Calls one other library routine and then returns with no further work.
- `poll_status_and_compare_characters`: Calls a library routine and, when bit 5 of the D4 register is set, calls another one and compares the character it returns with I and then E. Every path returns without writing a result back.
- `return_immediately_empty_stub`: Returns straight away without doing any work.
- `match_letter_sequence_with_status`: Reads characters through library routines and checks them against the letter pairs E, N, G, S, D, R, L and Y, testing bit 7 of the D4 register between steps.
- `match_ng_then_check_flag`: Checks that the first character is N and the next is G. If so it reads another character and tests bit 7 of the D4 register, then checks for an S with a further bit 7 test; every outcome returns without writing anything.
- `return_immediately`: This library code is an empty routine that returns immediately without performing any operation.
- `store_byte_via_ptr`: This routine is part of translator.library, a shared library on the disk rather than the game program.
- `empty_routine_5`: This routine is empty, containing no instructions and performing no operations on any data.
- `execute_three_instructions`: This routine executes three instructions and then returns.
- `trivial_return_routine`: This routine contains three instructions and terminates with a return, but it performs no visible operations on any data and calls no other routine.
- `poll_status_bit_until_clear`: It repeatedly calls a library routine while bit 5 of a status value stays set, then returns once that bit becomes clear.
- `return_immediately_3`: It executes a single branch instruction that returns directly without modifying any data.
- `repeat_call_while_bit1_set`: Calls an unnamed library routine repeatedly for as long as bit 1 of the D4 register stays set, then returns.
- `test_bit7_after_helper_call`: Calls a library routine and then tests bit 7 of the D4 register, but both outcomes return without any further action.
- `empty_return_routine`: This routine contains no instructions and simply returns without performing any operation.
- `copy_uppercase_text_to_buffer`: Copies characters from a source text into a buffer, turning control codes, the delete code and closing brackets into spaces and changing lowercase letters to capitals.
- `write_space_before_pointer`: Writes a space character two bytes before the address held in the A2 register and returns.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 253 into d1, then returns.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies 32 to (a2)+, then copies 32 to (a2)+, then returns.
- `insert_digit_four_after_letter`: Scans the buffer for a character in a flagged class and, when that character matches one of fifteen table letters, shifts the text after it one place and writes the digit four into the gap.
- `shift_buffer_insert_digit_four`: Shifts the bytes from an insertion point up to the write position one place toward the end of the buffer, writes the digit four into the opened gap, and advances the stored write pointer by one.

### info.library

- `stub_bad_instruction_data`: Holds only an instruction the decompiler could not translate, so the routine cannot be followed and recovers no behaviour beyond that.
- `store_library_state`: Stores the incoming register values into two global variables, writes the A0 value into a field of the structure whose address arrives in D0, and keeps a copy of A0 in a third global.
- `call_library_helper_3`: This library code calls another routine to allocate memory and initialize a structure, then returns.
- `call_helper_and_return`: A short routine that invokes another routine and then returns without modifying any data.
- `dispatch_on_counter_state_2`: This library code calls an unnamed routine and then branches on the state of a 16-bit counter to invoke further handlers.
- `do_nothing_2`: This routine does nothing; it returns immediately without reading or writing any data or registers.
- `build_record_with_error_codes_2`: This library code clears several global pointers, allocates a block through a library call, and links two additional blocks together.
- `clear_flag_and_increment_counter`: Clears bit 3 of the flag byte at offset 14 in the structure passed in, increments the 16-bit counter at offset 32 in the same structure, and returns the structure pointer.
- `dispatch_on_counter_state`: Calls an unnamed library routine, then branches on the 16-bit counter at offset 32: when it is zero it calls routines using the fields at offsets 16 and 18, and otherwise it sets bit 3 of the flag byte and returns zero after one more call.
- `call_library_functions_offset_args`: Loads a value from a global variable, calls one library function with two arguments derived from fields of the structure at A2, makes a further library call, and returns the saved global value.
- `decrement_counter_then_dispatch`: Decrements the 16-bit counter at offset 32 of the structure and, when it reaches zero with bit 3 of the flag byte set, calls another routine on the same structure and returns its result.
- `return_constant_zero`: Returns zero without touching any data, giving a fixed no-result value.
- `call_routine_for_nonzero_pointers`: Calls a routine on each of several global pointers and on two parameter values, skipping any pointer that is zero.
- `allocate_and_initialize_structure_tables`: Allocates a block of memory through a library call and, when the allocation succeeds, stores pointers at fixed offsets inside the block and copies runs of values from global data into its first parts.
- `initialize_info_library_struct`: Fills a structure with pointers computed as fixed offsets from a base address, then copies blocks of values from a global table into the structure's arrays and links the pointers together.
- `open_required_system_libraries`: Opens four system libraries in turn, storing each returned base in a global variable, and the routine references the dos.library name.
- `open_icon_library_report_error`: Opens the icon library through three library calls in turn, storing each result in a global variable.
- `open_intuition_library_base`: Opens the intuition library through two library calls, storing each result in a global variable.
- `open_graphics_library_base`: Opens the graphics library through a library call and stores the base in a global variable.
- `return_constant_one`: Returns one without touching any data, giving a fixed success value.
- `fetch_nested_field_value`: Returns the value cached in a structure field when it is set.
- `fetch_nested_field_from_pointer`: Reads the structure pointer from A1 and returns its cached value when set.
- `fetch_cached_or_chained_value`: Returns the cached value from a structure field when it is set.
- `lookup_value_via_type_chain`: Follows a pointer chain chosen by a type byte in the structure at A1, or through a second pointer when that byte is not the special value, and returns the long value at a fixed offset.
- `return_field_from_pointer`: Returns the long value at a fixed offset inside the structure referenced by the pointer passed in, or zero when that pointer is null.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a2, then returns.
- `build_name_lookup_disk_info`: Stores a value from one library call into a structure field, builds a local name buffer from either a fixed name or a stored one, and passes it to another routine, referencing the disk.info name.
- `check_library_status_set_flag`: Calls a library routine to check status using a local buffer, then clears a global flag.
- `allocate_and_fill_record_block`: Allocates two memory blocks through library calls and checks stored values, then copies data between them and sets their size and link fields.
- `fill_record_block_after_checks`: Stores a value passed in from a call, checks it together with conditional flags, and then copies data into a record and sets its size and link fields.
- `setup_record_when_checks_pass`: Runs up to two status checks, and when they pass it copies data into a record, sets its size from a stored count with a minimum of 256, obtains a further block through a library call and links the fields.
- `setup_record_fields_optional_check`: Runs an optional status check first, then copies data into a record, sets its size from a stored count with a minimum of 256, obtains a further block through a library call and links the fields.
- `copy_record_fields_and_allocate`: Copies data into a record, sets its size from a stored count with a minimum of 256, obtains a further block through a library call and links the fields.
- `copy_fields_between_records`: Passes a stored block and a second structure field to a library routine, copies two fields from one record to another, and returns one.
- `build_record_with_error_codes`: Clears several global pointers, allocates a 246-byte block through a library call, and obtains two more blocks that are linked together.
- `build_record_via_error_stub`: Allocates a 246-byte block through a library call and, if any step fails, reports error code 5, 6, 7 or 1 through a library vector.
- `link_record_or_report_error`: Links a looked-up block into a record and reports the outcome through a library vector.
- `link_record_report_status_vector`: Looks up a block and stores a global pointer from a library call, then links the record and reports the outcome through a library vector.
- `store_record_pointer_and_report`: Stores a global pointer from a library lookup, links the record and reports status through a library vector.
- `finish_record_link_and_report`: Links the record together through several library calls and reports zero to a library vector on success.
- `report_link_result_via_vector`: Looks up a value and, when the follow-up step succeeds, runs a sequence of library calls and reports zero through a library vector.
- `finish_with_zero_status_report`: Makes one call when a global flag is clear and a parameter is set, then makes four further calls and reports a zero status through a library vector.
- `call_exec_vector_132`: Calls the library function at offset 132 below the Amiga library base kept in a global variable, then returns.
- `call_exec_vector_138`: Calls the library function at offset 138 below the Amiga library base kept in a global variable, then returns.
- `call_exec_vector_198`: Calls the library function at offset 198 below the Amiga library base kept in a global variable, then returns.
- `call_exec_vector_210`: Calls the library function at offset 210 below the Amiga library base kept in a global variable, then returns.
- `call_library_vector_234_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_exec_vector_246`: Calls the library function at offset 246 below the Amiga library base kept in a global variable, then returns.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_exec_vector_294`: Calls the library function at offset 294 below the Amiga library base kept in a global variable, then returns.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_exec_vector_372`: Calls the library function at offset 372 below the Amiga library base kept in a global variable, and is reached from the reschedule stub.
- `call_exec_vector_378`: Calls the library function at offset 378 below the Amiga library base kept in a global variable, then returns.
- `call_exec_vector_414`: Calls the library function at offset 414 below the Amiga library base kept in a global variable, then returns.
- `call_exec_vector_552`: Calls the library function at offset 552 below the Amiga library base kept in a global variable, then returns.
- `call_library_vector_42`: Calls the library function at offset 42 below the library base kept in a global variable, then returns.
- `call_library_vector_72_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_96`: Calls the library function at offset 96 below the library base kept in a global variable, then returns.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_222_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_228`: Calls the library function at offset 228 below the library base kept in a global variable, then returns.
- `call_library_vector_270_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_library_vector_354_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_366`: Calls the library function at offset 366 below the library base kept in a global variable, then returns.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `call_library_vector_462`: Calls the library function at offset 462 below the library base kept in a global variable, then returns.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72`: Calls the library function at offset 72 below the library base kept in a global variable, then returns.
- `call_library_vector_78`: Calls the library function at offset 78 below the library base kept in a global variable, then returns.
- `call_library_vector_234`: Calls the library function at offset 234 below the library base kept in a global variable, then returns.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_270`: Calls the library function at offset 270 below the library base kept in a global variable, then returns.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_354`: Calls the library function at offset 354 below the library base kept in a global variable, then returns.
- `invoke_library_functions`: This library code calls two different library functions using a base address from a global variable and a pointer passed via a stack-allocated structure.
- `dispatch_on_state_code`: Switches on a 16-bit state code held in a structure, calling a different routine with the relevant fields for each code and setting a global flag for some of them.
- `return_nonzero_value`: This library code returns a constant nonzero value to the caller.
- `toggle_mask_bit_from_code`: Maps a 16-bit code from the second argument to a single-bit mask and toggles that bit in a flags word held in a global record.
- `store_flags_then_call_routine`: Stores the value in the D1 register into a flags field of a global record, then calls a routine with the pointer at offset 44 of the structure in A1.
- `build_info_file_name_check`: Formats a name from two parts and the suffix .info through a library call, then uses library routines to look up and check the related entries and store results in the record.
- `check_disk_name_and_attributes`: Formats a name from a stored string and the referenced Disk text, checks a lookup and two attribute values, then updates an attribute through a library call.
- `validate_info_file_attributes`: Formats a name from two parts with the .info suffix, looks up the matching entry and runs a series of attribute checks through library calls.
- `check_four_fields_then_act`: Calls a routine for each of four global fields in turn, and if any call returns minus one it returns zero.
- `check_three_fields_then_act`: Calls a routine for three values in turn and returns zero if any of them returns minus one.
- `check_two_fields_then_act`: Calls a routine for two values in turn and returns zero if either returns minus one.
- `call_report_not_minus_one`: Calls a routine and, when its result is not minus one, calls a second routine.
- `call_other_and_return_one`: Calls one other library routine and then returns one.
- `initialize_info_library_tables`: Registers a fixed sequence of table entries by making twenty-four calls with numeric ranges and a pointer from the record, and the routine refers to the labels NAME, STATUS and TYPE. Some calls pass a global variable as well.
- `register_seven_fixed_entries`: Makes seven calls to one routine with a shared first argument and fixed numeric ranges.
- `register_seven_index_entries`: Makes seven calls to one routine with a shared first argument and fixed index ranges in the last two arguments.
- `register_info_table_entries`: Makes nine calls to one routine with a shared first argument and fixed index ranges, registering entries that cover single indices or ranges.
- `set_info_table_entries`: Makes nine calls to one routine with a shared first argument and six fixed values each time, covering single indices and ranges.
- `call_by_state_fallback_entry`: Chooses one of two index values from a type byte in the global record, then calls a routine with a short argument list when the list is empty or a fuller one otherwise.
- `call_entry_then_optional_notify`: Makes one call on a list entry, choosing between a form that also passes a structure field when the list is empty and a plain call otherwise.
- `call_field_then_optional_pointer`: Makes a call and then a second call using a field from the structure at A2, followed by a third call on a stored pointer when a flag is non-zero.
- `build_linked_entry_list`: Obtains a head block through a library call and, when that succeeds, links it to itself and clears a counter.
- `link_entries_from_pointer_table`: Links the list head to itself and clears a counter, then for each item in a stored pointer table obtains an entry through a call and attaches data to it.
- `append_entries_until_terminator`: Walks a zero-terminated pointer table, obtaining an entry for each item and attaching data to it.
- `append_entries_until_null_entry`: Walks a zero-terminated table, obtaining an entry for each item and attaching it to the list.
- `step_list_entry_or_finish`: Either finishes the current list entry with calls that use fields of the structure, or moves the list position on to the next entry and makes two calls for it.
- `update_list_position_from_entry`: Makes a call on the current list entry's data, advances the list position to the following slot, and makes two more calls.
- `advance_list_position_or_finish`: When the list is at its end, calls routines with structure fields and a zero flag.
- `advance_list_pointer_by_dereference`: Makes a call on the current list entry's data, replaces the list position with the value its pointer refers to, and makes two more calls.
- `append_new_entry_to_list`: Sets a state flag and, when the list is not empty, makes two calls on the current list entry.
- `advance_list_entry_with_links`: Sets a state flag, then either finishes with calls on the structure when the list is empty or moves the list position to a neighbouring entry chosen by the links.
- `step_back_list_position`: Reads the current list entry, calls a routine on it, and moves the list position to the entry linked from the current one, or to the first entry when the list is empty.
- `run_entry_callbacks_in_order`: Makes one call and then two more calls on fields at offsets 46 and 62 of the structure in A2, in that order.
- `allocate_pointer_array_per_entry`: Counts the list entries and, when the list is not empty, obtains an array of pointers sized to that count and fills it with an entry and data block for each list item.
- `allocate_pointer_array_for_list`: Clears the array pointer for an empty list, or otherwise sizes a new array from the entry count, fills it with entries and data blocks for each list item, and returns zero when an allocation fails and one on success.
- `fill_pointer_array_from_list`: Counts the list entries, obtains an array sized to the count, and fills it with an entry and data block for each list item.
- `fill_array_per_list_entry`: Obtains an array sized to the list length and fills each slot with an entry and data block for every list item.
- `attach_data_per_list_item`: Starts a data attachment pass by calling a routine on the array, then for each list item obtains an entry and a data block and records them in the array.
- `attach_data_until_list_end`: Walks the list from its current position, obtaining an entry and a data block for each item and recording them in the array.
- `attach_data_loop_until_end`: Continues a loop that records an entry and a data block for each remaining list item, returning one when the list ends and zero when an allocation fails.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_90`: Calls the library function at offset 90 below the library base kept in a global variable, then returns.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `call_library_vector_114`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.
- `call_library_vector_180`: Library stub: loads a library base from a global variable and calls the library function at offset minus 180 of that base, returning its result to the caller.
- `call_library_vector_186`: Library stub: loads a library base from a global variable and calls the library function at offset minus 186 of that base, returning its result to the caller.
- `dispatch_layout_calls_by_type`: Switches on a type byte in a global record and, for each of six type codes, makes a fixed sequence of calls that pass numeric arguments, sometimes with a pointer from the record.
- `return_nothing`: It is an empty routine that performs no operations and simply returns.
- `allocate_blocks_dispatch_by_type`: Allocates a set of memory blocks through library calls and, if they all succeed, allocates a second set and runs a sequence of calls chosen by the type byte.
- `obtain_blocks_run_type_steps`: Obtains three blocks and stores a fourth value through library calls, returning zero if any is missing.
- `run_type_dependent_call_sequence`: Makes between two and five calls to a routine, with the number depending on the type byte in a global record.
- `empty_library_return`: This is an empty library routine that contains only a return and performs no operations.
- `clear_or_copy_check_result`: Either clears a stored word or copies a value through a call, depending on a field of a global record.
- `copy_value_then_check_result`: Copies a stored value through a call, then calls a routine and returns zero when its result is minus one, otherwise the result.
- `do_nothing_return_2`: This routine performs no operations and returns immediately without touching any data or registers.
- `set_field_offsets_check_four`: When the type byte is not four, writes four fixed word values at offset 6 of four stored records.
- `check_three_calls_sentinel`: Calls one routine three times in turn and returns zero if any call returns minus one, otherwise one.
- `check_two_calls_sentinel`: Calls one routine twice in turn and returns zero if either call returns minus one, otherwise one.
- `call_and_test_sentinel`: Makes one call and returns true when its result is not minus one.
- `return_constant_one_2`: This library routine returns the constant value one without performing any other operations.
- `loop_process_handles_until_failure`: Loops indefinitely: each pass calls a routine on a structure pointer and, when that returns a non-zero handle, processes the handle with the second argument and returns zero on failure.
- `return_constant_zero_2`: This library routine returns the constant value zero without performing any other operations.
- `dispatch_by_mode_value`: Reads a mode value from offset 20 of the structure.
- `call_once_and_return`: This library routine calls another routine once and then returns without performing any other operations.
- `call_once_return_zero`: Calls one routine and returns zero regardless of the result.
- `call_twice_return_one`: Calls one routine twice in turn and returns one.
- `run_sequence_of_entry_calls`: Makes three calls on the structure passed in, with one more call unless the type byte is one.
- `format_protection_flag_labels`: Masks and combines the bits of a flags word from a global record, then makes four calls that each pass one bit state together with a label.
- `build_protection_label_text`: Builds a short string from a fixed text and a character lookup, uses a flag argument to choose one of two prefixes, and passes the results to two output routines.
- `build_formatted_argument_list`: Builds a formatted text buffer on the stack from the referenced pattern of spaces, a comma and two string slots, then makes two further calls.
- `measure_c_string_length`: Returns the number of bytes before the zero terminator in a string, counting from the start pointer.
- `measure_string_length_from_offset`: Advances an index from its starting value until the byte at that index plus a base is zero, then returns that index.
- `copy_c_string_to_buffer`: Copies bytes from a source string to a destination buffer, one at a time, up to and including the zero terminator.
- `copy_fixed_byte_count`: Copies a given number of bytes from one buffer to another, one byte at a time.
- `copy_counted_bytes_do_while`: Copies bytes from a source to a destination in a loop that always runs at least once, stopping when the count reaches the limit held in another register.
- `compare_c_strings_for_equality`: Walks two zero-terminated strings together while their bytes match, stopping at the first difference or terminator.
- `compare_strings_until_mismatch`: Compares two zero-terminated strings from their start, advancing both until a mismatch or terminator, and returns whether the final bytes are equal.
- `count_c_string_bytes`: Counts the bytes of a zero-terminated string from the given pointer up to and including the terminator, and returns that count.
- `make_three_library_calls`: Calls a helper on its last argument, then makes three library calls through the base pointer at offset 50 of the structure, passing the arguments in turn with one reduced by ten.
- `format_text_then_call_routine`: Formats a text from a global string and one argument into a local twelve-byte buffer, then passes that buffer with the other arguments to another routine.
- `copy_counted_bytes_then_terminate`: Copies a counted run of bytes into a destination buffer and appends a zero terminator after the last byte.
- `copy_bytes_then_terminate_string`: Copies bytes into a destination buffer up to a count, and writes a zero terminator after the last byte.
- `fill_two_records_and_register`: Fills two records with the supplied values and fixed defaults, makes two setup calls, then passes the first record to a routine.
- `call_library_vector_222`: Calls the library function at offset 222 below the library base kept in a global variable, passing a pointer from offset 62 of the structure, and returns one.
- `make_three_base_library_calls`: Makes three calls through the library base pointer at offset 50 of the structure.
- `call_rawdofmt_from_base`: Calls the exec formatting function RawDoFmt through the library base stored in a global variable, then returns.
- `store_byte_at_pointer`: Writes the low byte of the D0 register to the address held in the A3 register, then returns.
- `call_library_vector_36`: Calls the library function at offset 36 below the library base kept in a global variable, then returns.
- `call_indirect_via_table`: This library routine calls another routine through an indirect pointer taken from a global table.

### version.library

- `empty_routine`: This routine is empty and simply returns without performing any operation.
- `store_two_longs_in_record`: Stores the values held in two registers into the structure at offsets 34 and 38, then returns.
- `clear_flag_and_increment_counter_2`: Increments the 16-bit counter at offset 32 of the structure and clears bit 3 of the flag byte at offset 42, then returns.
- `decrement_counter_and_call`: Decrements the 16-bit counter at offset 32 of the structure and, when it reaches zero with bit 3 of the flag byte at offset 42 set, calls another routine and returns its result.
- `finish_entry_with_library_calls`: When the counter at offset 32 is zero, reads a value from offset 38 and makes two library calls through the base pointer at offset 34, at offsets 252 and 210 below it, then returns the value read.
- `read_field_two_library_calls`: Reads the value at offset 38 of the structure, makes two library calls through the base pointer at offsets 210 and 252 below it, and returns the value read.
- `do_nothing_return`: This routine returns immediately without executing any instructions or touching any data.

### Populous (1989)(Electronic Arts)[cr QTX].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
