# Game 2014 (populous (1989)(electronic arts)[cr vf][a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02014). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (905136 top-level bytes in 35 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 14 of 14 units identical (bytes and relocations), 37 raw-word lines |
| Functions named and described | pass, 624 of 624 named (0 author symbols), 624 of 624 described (84 by rule, 174 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1107 of 1107 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 131072 bytes, 85 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9820 of 10587 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 588988 |
| code | 80626 |
| data | 209079 |
| padding | 10059 |
| unused | 16384 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| endcli | 2 | 117 | 2 |
| echo | 2 | 117 | 2 |
| loadwb | 12 | 536 | 30 |
| Disk-Validator | 1 | 386 | 1 |
| Populous (1989)(Electronic Arts)[cr VF][a2].adf:POP | 2 | 83 | 3 |
| Populous (1989)(Electronic Arts)[cr VF][a2].adf:populous | 3 | 4314 | 145 |
| Populous (1989)(Electronic Arts)[cr VF][a2].adf:populous.prg | 2 | 83 | 3 |
| icon.library | 10 | 1375 | 109 |
| translator.library | 2 | 490 | 50 |
| info.library | 25 | 2597 | 167 |
| version.library | 2 | 41 | 7 |
| Populous (1989)(Electronic Arts)[cr VF][a2].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 438 | 19 |
| Populous (1989)(Electronic Arts)[cr VF][a2].adf:DAD (raw code file, load address unknown) | 1 | 12826 | 85 |

## Cases handled

- populous (1989)(electronic arts)[cr vf][a2]/Populous (1989)(Electronic Arts)[cr VF][a2].adf: file lord.pic (header 32032 bytes, blocks hold 6344): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address: 226 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- populous (1989)(electronic arts)[cr vf][a2]/Populous (1989)(Electronic Arts)[cr VF][a2].adf:DAD: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- populous (1989)(electronic arts)[cr vf][a2]/Populous (1989)(Electronic Arts)[cr VF][a2].adf: blocks 11-21 (bytes 5632-11264) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.17)
- populous (1989)(electronic arts)[cr vf][a2]/Populous (1989)(Electronic Arts)[cr VF][a2].adf: blocks 32-32 (bytes 16384-16896) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.34)
- populous (1989)(electronic arts)[cr vf][a2]/Populous (1989)(Electronic Arts)[cr VF][a2].adf: blocks 938-938 (bytes 480256-480768) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.37)
- populous (1989)(electronic arts)[cr vf][a2]/Populous (1989)(Electronic Arts)[cr VF][a2].adf: blocks 1716-1759 (bytes 878592-901120) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.95)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `reset_global_flags` (populous): Clears one byte and writes the constants fifteen and 255 into global variables, then returns. Kind: checksum or state reset trap. Pass path: the code continues to the instructions immediately following the call, assuming the flags are cleared for a fresh validation cycle.

## Functions


### endcli

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_block_then_call_handler`: Copies sixteen longwords from a fixed offset in a data block into the caller's buffer when one is given, then calls the routine pointer in the address register.

### echo

- `scan_max_and_build_buffer`: Walks the entry list to find the largest stored value, then sizes and fills a buffer with a counter-based header pattern.
- `copy_memory_block`: Copies a fixed number of words from a source pointer to a destination pointer.

### loadwb

- `startup_collect_dos_message`: Program entry that opens dos.library, looks up its own task, and waits for a startup message.
- `restore_saved_state_and_cleanup`: Swaps the passed parameter into the task's state record and runs whichever cleanup calls the record's fields call for, including closing a library, replying to a message and freeing the record.
- `launch_named_tool_from_options`: Checks the arguments for a debug switch, opens the icon and intuition libraries, and starts the named tool.
- `compare_strings`: Compares two null-terminated strings byte by byte.
- `build_linked_list_from_table`: Walks a table of pointer and value pairs, allocating an eight-byte node for each pair and storing a value looked up from it in the node.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `pad_matching_release_name`: Compares a name buffer against a global string one character at a time, and when the character at a fixed position is a two and the prefix matches, writes dot-and-space padding over the end of the name.
- `write_dot_space_padding`: Writes a period followed by four spaces into five consecutive bytes of the buffer, starting at a fixed offset.
- `match_prefix_against_global`: Advances through the buffer while its bytes match the global string, stopping at the global string's terminator.
- `empty_routine`: The routine contains four instructions and performs no observable operation; it ends with a return.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `forward_call_with_stack_address`: Forwards its two parameters to another routine together with the address of a stack slot in its own frame, and returns nothing.
- `format_message_and_emit`: Formats a message into a 140-byte stack buffer through exec's formatting call, measures the resulting text, and passes it on with its length to an output routine when it is nonempty.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Disk-Validator

- `scan_bit_transitions_and_report`: Writes a two-digit count into a text line, then scans ranges of 32-bit words bit by bit, counting every place where a bit changes value between neighbours and storing the running totals in the state record.

### Populous (1989)(Electronic Arts)[cr VF][a2].adf:POP

- `intro_bytekiller_depack`: Cracktro start: ByteKiller depacker that decodes the packed stream after it backwards into 50000 hex onwards, showing progress on COLOR00, checks the checksum and jumps to the unpacked cracktro at 50000 hex.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Populous (1989)(Electronic Arts)[cr VF][a2].adf:populous

- `clear_globals_and_start_program`: Zeroes a large block of global variables and records the startup value, then runs a setup routine when a flag bit is set.
- `allocate_picture_chip_buffers`: Allocates several chip-memory blocks of fixed sizes and stores their handles in globals, then runs a series of initialization calls and fills a structure with values derived from the arguments.
- `start_copper_list_from_state`: Reads the copper list pointer stored in the state block and passes it to the routine that points the copper at a new list and starts it immediately.
- `configure_resources_set_status`: Calls routines that configure four stored resources with fixed sizes and rates, then stores a constant status word in a global.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `allocate_chip_row_pointer_table`: Allocates a zeroed chip-memory block and fills an eight-entry table with pointers spaced 144 bytes apart.
- `store_constant_then_call`: Stores the constant 399 in a 16-bit global and then calls a routine.
- `call_with_row_table_block`: Passes the block pointer from a global table slot, together with the constant size of 1152 bytes, to another routine.
- `call_two_global_pointers`: Calls a routine with the pointer held in one global slot and then with the pointer held in a second global slot.
- `call_library_with_low_word`: Extracts the low word of its argument and passes it, together with a global value, to a library stub routine.
- `forward_call_without_arguments`: Calls one routine with no arguments and returns, passing nothing through.
- `match_pair_table_and_call`: Searches a two-entry global table for the second argument and, on a match, calls a routine with the paired value.
- `swap_globals_and_call`: Swaps two 32-bit global values, then passes the new contents of both slots to another routine.
- `initialize_graphics_and_intuition_handles`: Stores a library handle in a global, sets a sentinel value in one of its fields, and obtains two more handles from library calls, each followed by a failure path when the result is zero.
- `wait_for_code_then_call`: Calls a routine repeatedly until it returns one, two or three, then passes a global pointer and a record address to another routine.
- `load_pic_and_wait_sync`: It then sets a global flag based on comparisons of global shorts and returns the flag.
- `initialize_game_state`: Installs two callback pointers and sets three 16-bit state values to 75, 50 and zero.
- `advance_coded_track_sequence`: Runs one step of a byte-coded sequence for sixteen tracks.
- `stub_returns_immediately`: Empty routine that does no work and returns at once, used only as a callback address.
- `run_scene_position_loop`: Fills eighteen 16-bit position entries from a counter reduced modulo eight, then loops drawing them at positions measured from 168.
- `fill_short_arrays_modulo`: Loops for the count given in its argument, filling one 16-bit array with a value from a called routine reduced modulo 13 and offset by 232, and a second array with the same value reduced modulo 9.
- `step_bouncing_positions`: Advances a set of bouncing values: each velocity gains one, and where the position would fall below zero the velocity is halved, reversed and the position is clamped to zero.
- `process_demo_pic_sequence`: Runs the demo.pic presentation, making a series of library and drawing calls and looping while a routine keeps returning nonzero.
- `build_four_buffer_pointer_pairs`: Computes four pairs of buffer addresses spaced 300 and 32000 bytes apart, passes each pair to a routine with two output slots, and copies the final pair into four globals.
- `read_asset_file_into_buffer`: Opens the file named in its argument and reads 32000 bytes into the buffer held in a global, copying the result into a second global before closing the file.
- `load_font_and_sprite_assets`: Loads the font.dat, land0 and sprites0.dat data files one after another, reading fixed byte counts from each into its own buffer, and closes each file after reading.
- `conditional_load_with_params`: Opens the file named in its first argument and, if the open succeeds, reads 40000 bytes into the buffer passed second and 64 bytes into a global buffer before closing it.
- `rotate_and_combine_word_table`: Clips the target row range, then reads a table of five-word groups, rotates each group by the low nibble of the position value and combines the rotated words with the destination's interleaved word planes using AND and OR. Rows outside the clip limits are skipped.
- `apply_rotated_mask_to_tiles`: Clips the row range, then reads five-word mask groups from a table selected by an index, rotates them by the low nibble of the column value and combines them with the destination tiles using AND and OR, repeating for a fixed number of rows.
- `call_and_return_register_pair`: Calls one routine and returns the values left in its two data registers combined into a single 64-bit result.
- `apply_tile_mask_table`: Clips the row range, then combines a table entry selected by a global base and an index with the destination words using AND and OR across four word groups.
- `reorder_record_word_fields`: Repeats 1680 times over a table of twenty-byte records, shifting the 16-bit words of each record along its fields and writing the reassembled 32-bit values back in place.
- `shift_byte_buffer_left_three`: Shifts each of the 100 bytes at a fixed buffer address left by three bits, in place.
- `scan_grid_positions`: Walks a ten-by-ten grid, calling a routine with each cell's computed position offset by a stored table value, and makes an extra call when the matching flag byte is set.
- `concatenate_registers`: The routine calls another routine and returns a 32-bit value formed by concatenating the contents of two data registers.
- `set_text_buffer_pointer`: Stores the address of a global buffer in a global pointer variable and returns.
- `initialize_buffer_from_string`: Fills an 80-longword area with all-ones and clears the 320 longwords after it, then measures the string at the global text pointer and writes each character's font entry, indexed from the space character and 40 bytes apart, into the buffer.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `open_library_and_mask_interrupts`: Allocates a memory block through exec and, when that succeeds, opens a library, reads and closes a file, and writes interrupt-control values into globals.
- `initialize_engine_subsystems`: Queries a function pointer to check a status condition, then populates several global variables with a maximum value and calls another routine.
- `stub_pointer_to_library_init`: One-instruction entry whose recorded pseudo-code is the shared library setup body, so no behaviour specific to this entry is evidenced beyond a call into exec.
- `execute_library_call`: An instruction that acts as a jump or call to a function pointer stored in a global variable.
- `access_function_pointer`: An instruction that retrieves a value from a memory location to be used as a function pointer.
- `load_popsam_file_with_errors`: Opens the dos library and the popsam file, allocates memory and reads and writes through dos, printing a not-enough-memory or cannot-find-file message on failure.
- `stub_returns_immediately_2`: Empty routine that does no work and returns at once.
- `clear_byte_and_return`: Clears a byte at a fixed offset from the register pointer and returns the value passed on the stack.
- `disable_interrupts_then_restore`: Writes a disable mask for the interrupt bits, calls a routine and frees a block through exec, then restores three saved interrupt values and re-enables the saved set with its top two enable bits set.
- `clear_byte_after_mode_calls`: Calls one routine when the mode byte is ten and another when it is one, then clears the byte at a fixed offset and returns the value passed on the stack.
- `clear_byte_after_mode_one`: When the mode byte is one it calls a routine, then clears the byte at a fixed offset and returns the value passed on the stack.
- `clear_memory_byte_2`: This routine sets a byte to zero at a fixed offset from a pointer held in a register.
- `reset_memory_byte_2`: Sets a single byte to zero at a fixed offset from a pointer held in a register.
- `clear_memory_byte`: Sets a byte at a fixed offset from a pointer to zero.
- `reset_memory_byte`: Sets a byte at a specific offset from a pointer to zero and returns the value that was previously on the stack.
- `acknowledge_interrupt_flag_bit`: When bit five of a global flag word is set, it writes 255 to a byte, keeps only bit six of the flag word and calls a routine.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `build_dispatch_tables`: Reads a table of stream descriptors from a global area and writes computed pointers and constants into the dispatch tables, dividing a fixed constant by each period value.
- `build_dispatch_tables_2`: Reads a table of stream descriptors from a global area and writes computed pointers and constants into the dispatch tables, then initializes additional tables and global variables.
- `reset_global_flags`: Clears one byte and writes the constants fifteen and 255 into global variables, then returns.
- `update_audio_channel_registers`: For each audio channel whose pending flag is set, loads its sample pointer and length into the custom audio registers.
- `advance_sound_channel_stream`: Counts down a sound channel's gate and, when it expires, reads the next event from the channel's byte stream.
- `advance_sound_channel_stream_2`: Resumes the sound event decoder for one channel, reading the next stream event and storing its length, volume and stream position in the channel state.
- `load_instrument_pointers_for_channel`: Looks up the instrument entry selected by the next stream byte and stores pointers to its data in the channel state.
- `start_sound_channel_event`: Starts a sound event on a channel: scales the first stream byte by a table value, stores the stream position and sets the channel's pitch value to 256 before entering the event loop.
- `decode_next_sound_event`: Reads the next sound event byte; non-negative values set the channel's length, volume and stream position, and other values are handled as control codes.
- `set_channel_flag_and_decode`: Stores a stream byte in the channel state, sets bit one of the channel's status byte, and continues decoding events.
- `step_channel_pointer_list`: Steps through the channel's pointer list, wrapping to the start at a zero entry, and resolves the next data pointer from the base address.
- `program_dma_channel_from_event`: Scales the event by the channel's stored factor, stores the stream position, and programs the DMA channel's pointer and length registers from the channel table before continuing to decode.
- `run_sound_event_loop`: Runs the sound event loop for one channel, dispatching on each event byte and updating the channel's stream position and volume until the stream ends.
- `forward_sound_event_decode`: Short entry point that loads the stream and channel pointers and continues into the sound event decoder.
- `step_sound_channel_period`: Reads the next value from a channel's stream, updates the channel's period field from it and calls a routine.
- `update_audio_dma_channel`: Updates the Amiga audio hardware registers and internal sound channel structures by calculating new period and amplitude values.
- `combine_word_planes`: Loops over 64 rows of words, combining each word with entries from a global table using AND, OR and NOT, and writes the results to four spaced locations per iteration.
- `mask_word_planes`: Loops 160 times over source and destination word arrays, masking each destination word with the source and a global table value using AND, NOT and OR, and updating four spaced planes per iteration.
- `store_stack_word_and_call`: Copies a word from the caller's stack into a global variable, calls another routine, and returns the values left in its two data registers as one 64-bit result.
- `forward_call_to_routine`: Calls another routine and returns, with no other effect.
- `poll_mouse_and_keyboard_input`: Polls the mouse port movement and the button register, updating the stored position with the scaled movement clamped to a minimum of zero, and reads the keyboard serial register when the flag is set.
- `split_state_word_and_call`: Splits a 16-bit global into its low and high bytes, stores each in its own global, and calls another routine.
- `store_state_and_enable_interrupts`: Copies a global into two storage slots and writes a constant with the top two bits set into the interrupt control global.
- `manage_interrupt_flag`: When the pending flag is nonzero, writes a disable mask for the low fourteen interrupt bits, copies the pending value into the interrupt control global with the top two enable bits set, and clears the flag.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `build_copper_list_entries`: Writes a sequence of copper list words into the list buffer, including display window start and stop values, with a branch on a global mode flag choosing between two sets.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `wait_copper_sync`: Writes a parameter and the parameter plus four to a global, then loops until bit five of that global is set and returns the masked value.
- `wait_for_interrupt_bit`: Writes the constant 32 to a global and spins until bit five of that global is set, then returns the masked bit.
- `copy_shorts_after_marker`: Scans a short array for the marker value 384, then copies a counted run of shorts from a source into the array just after the marker.
- `unrecovered_routine_no_pseudocode`: No pseudo-code was recovered for this routine, so its behaviour cannot be described; the listed facts show one caller and 21 instructions.
- `update_byte_buffer_from_params`: Looks up a byte buffer pointer in a table using the first argument, then writes several bytes derived from the second argument and a global, setting flag bits conditionally.
- `decode_keyboard_serial_key`: Clears two globals and, when a flag is set, decodes the keyboard serial data into a character through a lookup table covering the number-row symbols, storing it and setting a ready flag.
- `copy_32000_byte_block`: Copies 8000 longwords, 32000 bytes, from a source pointer to a destination pointer one longword at a time.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `decrement_nibble_counters`: Copies a block of words into a 32-word stack buffer, then repeatedly decrements each nibble until all are zero, waiting on the vertical position register between passes.
- `call_then_return_result`: Calls one routine with no arguments, then calls another routine with its two parameters and returns that second result.
- `call_then_conditional_call`: Calls one routine with its parameter, then calls a second routine only when a global counter is nonzero.
- `store_offset_and_extract_bits`: Calls a routine, adds the constant 12345 to its result, stores the sum in a global and returns the high 15 bits of the stored value.
- `store_high_word_to_global`: Stores the high word of its argument in a global variable and returns.
- `do_nothing`: This routine is empty and performs no operations before returning.
- `setup_dos_streams_from_table`: Allocates a cleared block sized from a six-byte table entry and returns early on failure.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `parse_quoted_argument_string`: Parses a command-line string: it copies the next argument into a buffer, skipping whitespace, and treats double-quoted text as one argument in which doubled quotes stand for a single quote.
- `append_string_with_limit`: Finds the end of the first string and appends the second to it, copying at most 32767 characters and terminating the result.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when the counter in the third argument expires.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a count or until a null terminator, then fills any remaining count with zeros.
- `multiply_32_bit_values`: Multiplies two 32-bit values from the data registers using their 16-bit halves and returns the full 64-bit product.
- `open_window_icon_file`: Opens the WINDOW file through dos and makes a sequence of library calls, including icon library lookups, checking each result for zero before continuing.
- `check_global_flags_and_cleanup`: Calls the routine pointer held in a global when it is set, then calls another routine.
- `call_indirect_function`: The routine checks a global variable for non-zero and calls an indirect function obtained from it, then calls another routine.
- `process_global_flags_and_cleanup`: Loops over a counted list calling a routine for each entry, then makes a further call with a computed size when a flag global is set.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_indexed_file_entry`: Checks a table index and entry, setting error code two and returning minus one when the index is out of range or the handle is empty.
- `close_dos_handle_from_global`: Closes the file handle held in a global by calling dos's close function through the library base.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_load_seg`: Wrapper that calls the dos library function LoadSeg and returns its result to the caller.
- `open_file_via_dos_base`: Opens a file through dos's open function, using the library base stored in a global.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `close_library_from_global`: Calls exec's close-library function on the library base stored in a global.
- `call_library_function`: The routine calls a function from a library using an offset from a pointer stored in a global variable.
- `allocate_memory_via_exec_base`: Calls exec's memory allocation function through the exec library base stored in a global.
- `call_indirect_function_via_offset`: Retrieves a function pointer from a table using an offset from a base address and executes it.
- `find_task_via_exec_base`: Calls exec's find-task function through the library base stored in a global.
- `dereference_nested_pointer`: The routine follows a chain of pointers stored in memory to execute a function at the final destination.
- `execute_indirect_pointer`: Retrieves a function pointer from a nested structure within a global data block and executes it.
- `execute_indirect_function_pointer`: Retrieves a function pointer from a nested structure within a global data block and executes it.
- `free_memory_via_exec_base`: Calls exec's free-memory function through the exec library base stored in a global.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_stored_exec_function`: Calls one exec library function through the library base stored in a global.
- `open_library_via_exec_base`: Calls exec's library-open function through the exec base stored in a global.
- `call_indirect_via_offset`: Retrieves a function pointer from a nested structure within a table and executes it.
- `execute_indirect_function`: Fetches a function pointer from a nested table structure using an offset from a base address and executes it.
- `execute_indirect_function_pointer_2`: Retrieves a function pointer from a nested structure within a table and executes it.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_3`: Retrieves a function pointer from a nested structure within a table and executes it.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `execute_indirect_call`: Performs an indirect call to a function pointer retrieved through multiple levels of pointer indirection.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_stored_library_function`: Calls one library function through a base stored in a global.

### Populous (1989)(Electronic Arts)[cr VF][a2].adf:populous.prg

- `intro_bytekiller_depack_2`: Cracktro start: ByteKiller depacker that decodes the packed stream after it backwards into 50000 hex onwards, showing progress on COLOR00, checks the checksum and jumps to the unpacked cracktro at 50000 hex.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### icon.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `increment_counter_clear_flag`: Library code from icon.library: increments a 16-bit counter in its base structure and clears bit three of a flag byte in the same structure.
- `decrement_library_refcount`: Library code from icon.library: decrements a 16-bit reference counter in its structure and, when it reaches zero and bit three of the flag byte is set, calls a cleanup routine.
- `unlink_library_and_free`: Library code from icon.library: when its 16-bit count is zero, unlinks its node from the list it belongs to and frees the structure through exec's free-memory call.
- `set_library_flag_bit`: Library code from icon.library: sets bit three of a flag byte in its structure and returns zero.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `init_icon_library_base`: Library code from icon.library: stores its two arguments in the library base, makes an exec call through the base, and raises an exec alert when the result is zero.
- `copy_base_fields_to_globals`: Library code from icon.library: copies two 32-bit fields from its base structure into two global variables.
- `library_handler_stub`: This library routine is empty.
- `return_from_subroutine`: This routine performs a single return instruction to exit the current function.
- `call_library_handler`: This library routine simply calls another routine and then returns.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `forward_call_to_routine_2`: Library code from icon.library: calls one routine and returns, with no other effect.
- `call_path_buffer_routine`: This library routine unconditionally calls a helper routine to scan a path and then returns.
- `invoke_handler_pointer`: This library routine retrieves a function pointer from a data structure and calls it.
- `call_library_handler_2`: This library routine calls another routine and then returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `call_icon_init_routine`: This library routine invokes a single initialization function to set up icon data structures and then returns.
- `allocate_checked_memory`: This library routine allocates a cleared memory block, stores a pointer to a field within it, and performs two lookups on the provided arguments.
- `call_library_handler_3`: This library routine calls another routine with a modified second argument and then returns.
- `call_other_routine`: The routine calls another routine and then returns.
- `call_subroutine_and_return`: The routine calls another routine and then returns.
- `call_and_return`: This routine calls another routine and then returns.
- `store_value_in_found_task`: Library code from icon.library: looks up a task through exec's find-task call using the first argument, then stores the second argument at a fixed offset in the returned structure.
- `try_chain_then_return_zero`: Library code from icon.library: calls a routine to obtain a value and, when it is nonzero, calls another routine with it; if that second call fails it calls a third routine with the value and returns zero.
- `build_icon_object_from_header`: Library code from icon.library: allocates an object, reads a header into a stack buffer, copies ten longwords and several fields from it into the object, then links the object into a parent list when one exists.
- `copy_ten_longwords_block`: Library code from icon.library: copies ten longwords from one structure block into another, then makes one further call.
- `return_zero_stub`: Library code from icon.library: an empty stub that returns zero.
- `copy_icon_data_to_stack`: Library code from icon.library: copies ten longwords from a structure into a local array on the stack and several more fields into other stack slots, then calls another routine with the first two arguments and a pointer to the stack header.
- `load_icon_info_file`: Library code from icon.library: opens an .info file by name, checks its header signature and version, allocates the image data it describes and reads each part.
- `read_icon_header_and_validate`: Library code from icon.library: calls a read routine and checks the returned header for the icon signature and version one before filling the record fields.
- `load_icon_optional_sections`: Library code from icon.library: stores two values read from the file in the record, then loads each optional section at its fixed record offset when that pointer is absent, reading the section data and linking it in.
- `iterate_icon_callback_until_false`: Library code from icon.library: repeatedly calls a callback until it returns zero, calling further routines on each pass to process the icon's entries.
- `call_then_return_zero`: Library code from icon.library: calls one routine and returns zero.
- `empty_routine_2`: The routine contains four instructions and performs no observable work; it ends immediately with a return.
- `allocate_path_buffer_and_scan`: Library code from icon.library: allocates a 260-byte buffer through exec, then fills it with several calls, yielding to other tasks through a reschedule call between steps.
- `open_info_file_and_read`: Library code from icon.library: opens an .info file through dos, reads its header and fills the record, calling several routines to process each section.
- `check_icon_section_and_read`: Library code from icon.library: calls two routines in turn and returns the result of the second, stopping early when the first fails.
- `walk_pointer_table_calling_routine`: Library code from icon.library: steps through a pointer table, calling a routine on each entry until the table is exhausted.
- `iterate_icon_table_entries`: Library code from icon.library: walks a null-terminated table of pointers, calling a routine for each entry, updating a counter and invoking a callback while possibly processing a secondary entry.
- `walk_icon_list_nodes`: Library code from icon.library: walks a linked list of nodes, calling a routine on each until the list ends or a null link is reached, then calls it once more on the last node found.
- `iterate_linked_list_nodes`: Library code from icon.library: follows linked nodes, calling a routine on each and fetching the next node, until none is left or the list loops back to its start.
- `increment_param_and_recurse`: Library code from icon.library: calls another routine with the same first argument and the second argument increased by 140, then returns.
- `allocate_cleared_record_and_fill`: Library code from icon.library: allocates a cleared 168-byte record through exec, fills it with calls, frees it again when the fill fails, and makes a reschedule call on the way.
- `return_zero_constant_2`: This routine returns a constant zero value to its caller.
- `return_zero_constant`: This library routine returns a constant zero value to its caller.
- `decrement_count_and_refill`: Library code from icon.library: decrements a 16-bit count through a pointer and, once it reaches its limit, calls a routine on the structure.
- `decrement_counter_and_store_pair`: Library code from icon.library: decrements a 16-bit counter through a pointer, then stores two register values into consecutive table entries indexed by the counter and returns one.
- `allocate_checked_memory_block`: Allocates a 94-byte cleared memory block, stores a pointer to a field inside it, and runs two lookups on the caller's arguments.
- `return_without_action`: Empty library routine that returns at once without doing any work.
- `call_routine_and_return_zero`: Calls one other library routine, discards its result, and returns zero to the caller.
- `call_with_stepped_offset`: Calls one other library routine with the second argument increased by a fixed step of 78, and returns without a value.
- `find_string_value_after_equals`: Walks a NULL-terminated list of strings and returns the text after the equals sign of the first entry that passes the match check.
- `scan_list_for_equals_entry`: Scans a NULL-terminated list of pointers for an entry that passes the match check and has an equals sign at the matched position, and returns the text after that sign, or zero when the list ends.
- `return_text_after_list_match`: Walks a NULL-terminated list of strings, calling a match check on each entry, and returns the text after the first equals sign of the entry that passes.
- `check_bar_segments_against_text`: Walks vertical-bar separated segments of a string and returns one when a segment has the length of the target text and the check on its contents passes.
- `check_bar_segments_by_length`: Walks vertical-bar separated segments of a string, comparing each segment length with a target length and calling a check on the text, and returns one on the first full match or zero at the end.
- `scan_bar_segments_with_check`: Steps through vertical-bar separated segments of a string, compares each segment length with a target and calls a text check, and returns one on a match or zero when the string runs out.
- `test_bar_segment_text`: Tests one vertical-bar separated segment at a time, returning one if its length equals the target and the text check passes, or zero when the end of the string is reached.
- `find_bar_segment_match`: Starts at the beginning of a vertical-bar separated string and searches its segments for one with the target length and a passing text check.
- `build_next_copy_name`: Builds a new name for a copied file in a buffer of at most 30 bytes.
- `parse_copy_number_prefix`: Reads a decimal number that follows a five-character prefix, checks that a fixed three-character marker follows, then calls two routines to finish the work.
- `parse_number_then_finish`: Reads a run of decimal digits at the current text position and checks that a fixed four-character string follows.
- `read_copy_number_digits`: Accumulates decimal digits from the current text position, then checks that the following text matches the copy suffix.
- `convert_char_to_digit`: Returns the numeric value of a character when it is a digit from zero to nine, and minus one for any other character.
- `stub_return_minus_one`: Returns minus one as a fixed negative result and performs no other work.
- `call_library_entry_no_args`: Calls one function of an Amiga library through its jump table and returns, passing no arguments and returning no value.
- `call_library_entry_then_return`: Calls a different function of an Amiga library through its jump table and returns, passing no arguments and returning no value.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `try_pair_and_release`: Calls one routine with two arguments and, if it returns a value, passes that value to a second routine.
- `allocate_record_with_defaults`: Allocates a 96-byte cleared block, stores the value ten in one of its 16-bit fields and in the first word of the caller's buffer, then passes the block to a second routine that fills the buffer.
- `return_zero_2`: This library routine returns zero.
- `forward_result_if_matching`: Calls a library function with three arguments and returns its result only when that result equals the fourth argument; otherwise it returns zero.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `check_library_result_equals_argument`: Calls one routine with three arguments and returns the result when it equals the fourth argument, otherwise zero.
- `return_immediately_no_op`: Empty routine that returns at once without doing any work.
- `divide_with_quotient_and_remainder`: Divides a 32-bit value by another, using a bit-by-bit loop when the divisor is large and 16-bit divisions otherwise, and returns the quotient and the remainder together.
- `divide_by_shift_subtract_loop`: Runs sixteen shift-and-subtract steps that divide a 32-bit dividend by a 16-bit divisor, leaving the quotient and remainder in the working registers.
- `multiply_32_bit_low_product`: Multiplies two 32-bit values from the working registers using 16-bit partial products and returns the low 32 bits of the product.
- `call_raw_format_function`: Calls the exec library raw formatting routine through a function pointer kept in the global data, then returns.
- `store_low_byte_at_pointer`: Stores the low byte of the data register at the address held in the address register, then returns.
- `measure_c_string_length`: Counts the characters of a zero-terminated string by decrementing a 32-bit counter once per character, and returns the count recovered from that counter.
- `find_char_in_c_string`: Scans a zero-terminated string for a given character and returns a pointer to its first occurrence, or zero if the terminator is reached first.
- `stub_return_zero_value`: Returns zero and performs no other work.
- `copy_c_string`: Copies a zero-terminated source string to the destination, including the terminating zero byte.
- `append_c_string`: Moves to the end of the destination string and copies the zero-terminated source onto it, including its terminator.
- `compare_strings_up_to_limit`: Compares two strings for at most a given number of characters.
- `return_one_no_op`: Returns one and performs no other work.
- `walk_string_then_finish`: Steps through a string for up to the given count of characters, and if the zero terminator is reached calls a finishing routine.
- `call_routine_and_return`: Calls one other routine with no arguments and returns to its caller without a value.
- `copy_string_with_zero_padding`: Copies characters from a source string into a fixed-size buffer until the zero terminator or the buffer size is reached, then fills the rest of the buffer with zero bytes.
- `empty_routine_3`: This library routine performs no operations and returns immediately.
- `initialise_empty_list_header`: Initialises an empty list header in the given block: the head points to the tail slot, the tail is cleared, and the tail predecessor points back to the head.
- `process_record_if_valid`: Returns one for an empty request.
- `walk_node_list_checking_each`: Walks a linked list of nodes, calling a check and then a second routine on each node, with a size taken from one byte field.
- `check_linked_nodes_in_sequence`: Steps through a linked list of nodes, running two checks on each node.
- `return_zero_value`: This routine is an empty function that returns zero to the caller.
- `allocate_and_link_record`: Returns zero when the count argument is zero.
- `return_failure_minus_one`: Returns minus one, a failure value, and does no other work.
- `build_record_chain`: Builds a chain of cleared records, one per step of the requested count, linking each to the previous one through a pointer field and validating it.
- `link_record_chain_until_end`: Allocates records through helper routines, links each into a chain, and continues until the link field of the newest record is empty.
- `fill_record_chain_from_stack`: Allocates and links records in a loop driven by a value taken from the stack, with a check after each step.
- `return_minus_one_value`: Returns minus one and performs no other work.

### translator.library

- `create_and_add_library`: Program entry for the translator library: it creates a library base with the exec MakeLibrary call, stores two values in it, and registers it with AddLibrary.
- `increment_counter_clear_flag_2`: Increments a 16-bit usage counter in the library base and clears bit five of a flags byte.
- `decrement_counter_and_maybe_finish`: Decrements the 16-bit counter in the library base.
- `remove_node_or_mark_pending`: If the node's count field is zero, unlinks the node from its list and calls a removal routine with a stored argument.
- `unlink_node_and_call_library`: Unlinks a node from its list by joining its neighbours, then calls one library function and returns a value stored in the node.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `parse_text_into_padded_buffer`: Reads a text stream one character at a time into a 26-character buffer padded with spaces, handles hash-marked fields through helper routines, and returns the parsed result.
- `return_low_word_argument`: Returns the low 16-bit word of the D0 register unchanged as the result.
- `clear_last_byte_return_word`: Calls one routine, writes a zero byte just before the output pointer, and returns the 16-bit value left in the D1 register.
- `scan_translation_entries`: Walks six-byte table entries, comparing input characters with each entry's text and calling helper routines.
- `match_translation_entry_text`: Compares input characters with the text of a table entry, using helper routines to advance through the entry.
- `skip_to_markup_delimiter`: Reads characters up to a backslash or backtick delimiter and calls helper routines to check the text.
- `scan_delimited_text_items`: Reads text character by character until a backslash or backtick, then calls helper routines for each delimited item and returns when the final check passes.
- `stub_immediate_return`: The decompiled body contains only a return, so the routine does no visible work in the library.
- `match_bracketed_text_prefix`: Skips text up to an opening square bracket, then compares the following characters with a stored string until a closing bracket, and returns.
- `scan_backwards_with_char_table`: Scans characters backwards through a text buffer, looking each one up in a character-class table, and calls a helper while the class flag is set.
- `return_no_op_stub`: Empty routine that returns immediately without doing any work.
- `compare_backwards_through_table`: Scans a text buffer backwards, checking each character against a character-class table and against a stored string.
- `scan_to_equals_with_table`: Reads characters until an equals sign, looking each one up in a character-class table and calling a helper for flagged characters.
- `match_until_equals_sign`: Compares input characters with a stored string until an equals sign, using a character-class table and a helper for flagged characters.
- `copy_word_wrapped_text`: Copies characters up to a backslash or backtick marker into an output buffer.
- `copy_fixed_length_text_wrapped`: Copies a fixed number of characters into the output buffer, backing up to a break-class character and ending the buffer there if the text would overflow.
- `copy_counted_bytes`: Copies a counted run of bytes to the destination, then updates a state flag from the byte that follows the run.
- `trim_trailing_break_characters`: Moves backward from the end of the output over characters that have a class flag set, and writes a zero terminator at that position.
- `dispatch_by_special_character`: Looks up a character in a twelve-entry table of special symbols and calls the handler routine stored at the matching position.
- `library_empty_return`: This library code routine performs no visible actions and simply returns.
- `stub_empty_return_slot`: Contains only a return in the decompiled output, so it is an empty stub with no visible effect on the library.
- `return_from_subroutine_4`: This library routine performs a single return from a subroutine.
- `repeat_call_while_flag_set`: Calls one routine, then keeps calling it while bit five of a flags word is set, and returns without a value.
- `call_empty_return`: This library code routine calls another routine that performs no visible actions and simply returns.
- `check_flag_and_letter_code`: Calls one routine, then if bit five of a flags word is set, calls another and compares the returned character with the letters I and E. Returns in every case without a value.
- `return_immediately`: This library routine performs an immediate return.
- `match_two_letter_suffix_codes`: Reads characters through helper routines and compares them with fixed letters, following one of several letter sequences such as E then S or D, R, L then Y, or N then G. Returns when a letter fails to match or the sequence is complete.
- `match_ng_suffix`: Checks that the next characters are the letters N and G, and when a flag bit is set also checks for the letter S, then returns.
- `return_from_subroutine_2`: This routine is empty and performs only a return instruction.
- `return_from_subroutine_5`: This library routine executes a single instruction to return from the current subroutine.
- `return_immediately_3`: This library routine performs no functional operations and returns to its caller immediately.
- `return_from_subroutine_6`: This library routine performs a single return from a subroutine, effectively exiting the current execution flow.
- `return_immediately_4`: This library routine performs no operations and returns to its caller immediately.
- `poll_status_bit_until_clear`: This library code routine repeatedly calls another routine while bit 5 of a value remains set, returning once that bit is clear.
- `return_immediately_5`: This library routine performs no actions and returns to the caller immediately.
- `repeat_call_while_bit_set`: Calls one routine repeatedly for as long as bit two of a flags word is set, then returns without a value.
- `call_then_test_flag_bit`: Calls one routine, then tests bit seven of a flags word and returns in either case without a value.
- `return_from_subroutine_3`: This routine is an empty function that performs no operations and immediately returns to its caller.
- `normalise_text_into_line_buffer`: Copies input text into a line buffer one character at a time, replacing control characters and closing brackets with spaces and converting lower-case letters to upper-case.
- `write_space_before_pointer`: Writes a space character two bytes before the address held in the address register, then returns.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 253 into d1, then returns.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies 32 to (a2)+, then copies 32 to (a2)+, then returns.
- `mark_vowel_pair_position`: Scans the buffer for a letter with a special class flag, then compares the text with fourteen two-letter vowel pairs.
- `insert_marker_character`: Opens a gap at a position in the buffer by moving the following bytes up one place, writes a fixed marker character into the gap, and updates the buffer end pointer.

### info.library

- `return_empty_routine`: This library routine is empty and performs no operations.
- `store_library_state`: Stores the incoming register values in global variables, writes a pointer into a field of the structure passed in, and keeps that pointer in another global variable.
- `call_try_pair_and_release`: This library routine calls another routine with two arguments and passes its return value to a second routine.
- `call_decrement_counter_routine`: Library code that calls another routine to decrement a 16-bit counter within a structure.
- `execute_callback_function`: Retrieves a function pointer from a data structure and executes it.
- `return_immediately_2`: This library routine is empty and performs no operations before returning.
- `reset_library_state_recursive`: This library code calls itself indefinitely, resulting in infinite recursion, before finally calling a routine that resets the library state.
- `clear_flag_and_increment_counter`: Clears bit three of a flags byte in the structure given as argument, increments the 16-bit counter in a later field, and returns the structure pointer.
- `release_block_or_mark_pending`: If the counter field is zero, calls routines that unlink and release the block and returns the stored value.
- `save_global_and_call_helpers`: Saves a global value, calls one library function, calls one routine with two arguments computed from fields of the structure, then calls another routine and returns the saved global value.
- `decrement_counter_release_flagged`: Decrements the 16-bit counter in the structure.
- `return_zero_no_op`: Does nothing and returns zero.
- `release_nonnull_global_pointers`: Calls a release routine on each of several global pointers and on the argument when it is non-zero, skipping null values, and returns without a value.
- `allocate_info_library_structure`: Allocates a public, cleared block of 944 bytes, stores pointers to its sub-fields in global variables, copies template data into several of them, and sets up list links.
- `initialise_info_structure_pointers`: Computes pointers to the fields of a structure from its base address, stores them in global variables, copies template blocks into several of the fields, and returns one.
- `open_required_libraries`: Opens three system libraries in sequence by name, keeping each base in a global variable.
- `open_libraries_via_pointers`: Opens libraries through a function pointer, storing each base in a global variable.
- `open_intuition_and_second_library`: Opens the intuition library and a second library through a function pointer, storing each base in a global.
- `open_graphics_library`: Opens the graphics library through a function pointer, stores its base in a global variable, and reports error code five if the open fails.
- `return_one_value`: This library code returns a constant integer value of one.
- `get_info_entry_value`: Returns the value stored in a structure's field when it is set.
- `get_info_value_from_pointer`: Reads a value from the structure reached through the address-register pointer, using a stored field first and otherwise following a parent pointer.
- `get_info_value_or_default`: Returns a stored value from the structure if one is present, otherwise reads it from a parent structure or returns zero.
- `read_info_field_from_parent`: Reads a value from a structure's parent when a flag byte is one, otherwise from a supplied pointer, and returns zero when no pointer is available.
- `read_nested_info_value`: Returns a value read through two pointers: a field of the structure in the address register and a field of the structure it points to.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a2, then returns.
- `read_disk_info_entry`: Reads a disk information entry into a temporary 116-byte buffer, using either a global name or a name held in the structure, and returns the value.
- `lookup_info_with_status_flag`: Looks up an entry in the structure's stored table using a temporary buffer, clears a status flag, and returns the found value while setting the flag when the entry is valid.
- `allocate_info_entry_blocks`: Allocates two cleared blocks of 36 and 260 bytes, validates them, allocates a third block of at least 256 bytes sized from a counter, copies data into it, and returns one on success or zero if any step fails.
- `fill_info_entry_from_pointers`: Fills in an info entry from the supplied pointers: it computes a size, checks the entries, allocates a further block, copies data into it, and returns one on success or zero on failure.
- `complete_info_entry_setup`: Completes setup of an info entry by copying data, storing a size of at least 256, and allocating a further block.
- `finish_info_entry_allocation`: Finishes an info entry by copying data, storing a size of at least 256, and allocating a further block.
- `setup_info_entry_return_flag`: Performs the info-entry setup steps, copying data, computing a size of at least 256, allocating a block and recording its fields, and returns true when the allocation succeeded.
- `copy_info_entry_fields`: Copies data from one structure into another, then copies two word-sized fields from the source structures and returns one.
- `build_info_entry_reporting_errors`: Clears its globals, then allocates a 246-byte public cleared block, checks and fills several sub-structures, and reports failure codes five, six, seven or one through a routine when a step fails.
- `build_info_entry_via_callback`: Builds an info entry like its neighbour, allocating a 246-byte cleared block and filling sub-structures, and reports failure codes through a function pointer when a step fails.
- `validate_and_fill_info_entry`: Checks and fills an info entry from a supplied structure, copying data between sub-structures and reporting error codes five or seven through a pointer when a step fails.
- `fill_info_entry_from_global`: Looks up an info entry, copies its data into a supplied structure, and reports error codes five or seven through a pointer when a step fails.
- `fill_info_entry_fields`: Copies a global entry into the supplied structure, then performs four copy steps, and reports error seven through a pointer when the lookup fails.
- `finish_info_entry_copy`: Copies data into the entry, runs a check and four follow-up steps when the check passes, and reports status zero on success or one on failure through a pointer.
- `report_info_entry_status`: Runs a check and, if it passes, copies data and performs four follow-up steps.
- `run_steps_call_library`: Optionally runs a helper when a flag is set and the argument is non-zero, runs four helper routines, then calls a library function with zero and the stack argument, and returns.
- `invoke_library_jump_entry`: Calls a function through a jump-table entry of the library whose base is kept in a global variable, passing no arguments, then returns.
- `call_library_function_2`: This library code calls a function through a pointer stored at an offset from a global variable.
- `call_library_function_alt`: This library code executes a routine via a pointer located at a specific offset within a global structure.
- `call_library_function_alt2`: This library code performs a call to another routine using a pointer retrieved from a global variable.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_function_alt3`: This library code uses a pointer from a global variable to call another routine.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_function_alt4`: This library code invokes a routine by dereferencing a pointer found at an offset from a global variable.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `execute_indirect_call_2`: Library code that calls a function pointer stored at an offset within a global table.
- `execute_indirect_call_3`: Library code that performs an indirect call to a function pointer located in a global table.
- `execute_indirect_call_4`: Library code that retrieves a function pointer from a global table and executes it.
- `execute_indirect_call_5`: Library code that executes a function via a pointer found in a global table.
- `invoke_secondary_library_entry`: Calls a function through a jump-table entry of a second library whose base pointer is kept in a global variable, passing no arguments, then returns.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `execute_indirect_call_11`: Library code that executes a subroutine via a pointer stored in a global table.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `invoke_callback_from_table`: The library routine calls another routine obtained by indexing a global table of function pointers with a fixed offset and then invoking it.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `invoke_callback_from_table_2`: The routine calls another routine obtained by indexing a global table of function pointers with a fixed offset and then invoking it.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `execute_indirect_function_2`: Fetches a function pointer from a location calculated by adding an offset to a global variable and executes it.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `execute_indirect_call_6`: Library code that calls a subroutine through a pointer stored in a global table.
- `execute_indirect_call_7`: Library code that performs an indirect jump to a function pointer within a global table.
- `execute_indirect_call_8`: Library code that executes a function pointer retrieved from a global table.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `execute_indirect_call_9`: Library code that calls a function through a pointer located in a global table.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `execute_indirect_call_10`: Library code that performs an indirect call using a function pointer from a global table.
- `create_info_with_fixed_parameters`: Fills a stack-allocated parameter block with fixed size, position and flag values, passes it to a routine that creates an object, and if that succeeds passes the result to a second routine.
- `dispatch_info_state_action`: Dispatches on a state code stored in a structure, calling a helper with stored values for each case.
- `return_one_value_alt`: This library code returns a single integer value.
- `toggle_info_flag_bit`: Maps a type code to one of several bit values, toggles that bit in a stored flags word, and calls a helper with a field of the structure.
- `store_flags_and_call_helper`: Stores the value in the data register into a stored flags field, then calls a helper with a pointer taken from the structure.
- `build_and_check_icon_file`: Formats an icon file name with the info extension in a 40-byte stack buffer, checks or creates the file through library calls, and returns whether it is usable.
- `check_disk_info_fields`: Checks disk-related fields of a structure through helper routines, clears the result when any check fails, and returns the result after applying a mask to one stored field.
- `check_info_file_attributes`: Copies a name into a stack buffer, checks the structure's fields through library calls and helper routines, and returns zero if any check fails, otherwise one.
- `check_three_info_entries`: Calls three helper routines to test four stored entries against a sentinel value.
- `check_three_indirect_results`: Calls one function pointer three times, returning zero if any result equals minus one.
- `check_two_indirect_results`: Calls one function pointer twice, returning zero if either result equals minus one.
- `call_indirect_and_report_success`: Calls a function through a pointer and, if the result is not minus one, calls a helper.
- `call_other_and_return_one`: Calls one other routine and then returns one.
- `initialise_info_field_table`: Registers a long list of field entries by calling a helper with constant range values, using the text labels NAME, STATUS and TYPE, then returns.
- `register_info_range_entries`: Calls one helper seven times with a fixed first argument and constant ranges, registering a set of entries.
- `register_second_info_range_set`: Calls one helper seven times with a fixed first argument and constant ranges, registering a second set of entries.
- `register_info_entry_group`: Calls one helper nine times with a fixed first argument and constant ranges to register a group of entries.
- `register_remaining_info_entries`: Calls one helper nine times with a fixed first argument and constant ranges to register another group of entries.
- `setup_entry_from_state_size`: Chooses a size value from a state byte, then either calls helpers to set up an empty entry or passes the value and a list field to a helper.
- `finish_list_update_and_notify`: Calls one routine when a list is empty and another with a field of the structure otherwise, then calls a further routine with a stored pointer when a flag argument is non-zero.
- `notify_entry_and_optional_pointer`: Calls one routine, passes a field of the structure to a second routine, and calls a third with a stored pointer when a flag argument is non-zero.
- `allocate_list_header_and_entries`: Allocates a 14-byte cleared list header and links it to itself, then walks a NULL-terminated table allocating a 128-byte cleared entry for each item and linking it into the list.
- `add_table_entries_to_list`: Links the list header to itself, then walks a NULL-terminated table allocating a 128-byte entry for each item and adding it to the list.
- `build_list_from_table_items`: Walks a NULL-terminated table, creating and linking one list entry per item through helper routines.
- `append_list_entries_from_table`: Walks a NULL-terminated table and appends one entry per item to the list through helper routines.
- `step_list_cursor_and_notify`: Either notifies the end of the list and calls two helpers with the structure's fields, or moves the list cursor to the next entry and calls helpers for that entry.
- `advance_list_cursor_and_notify`: Passes the current list entry's field to a helper, calls another routine, advances the list cursor to the next entry, and calls a final routine.
- `step_list_cursor_back`: Either notifies the end of the list and calls helpers, or passes the current entry to a helper and moves the list cursor backwards to the entry linked from its first word.
- `move_cursor_to_first_link`: Passes the current list entry's field to a helper, moves the list cursor to the entry linked from its first word, and calls a final routine.
- `insert_new_list_entry`: Sets a state flag, optionally calls two helpers when the list has entries, allocates a 14-byte header and a 128-byte cleared block for it, links the new entry into the list, and calls a final routine.
- `remove_list_entry_and_relink`: Sets a flag and either notifies an empty list or passes the current entry to helpers, selects a neighbouring entry, moves the list cursor onto it, and calls a final routine.
- `advance_list_cursor_by_link`: Calls a helper with the current cursor's link, moves the cursor onto that link or onto the list head at the end, and calls a final routine.
- `call_two_entry_fields`: Calls one routine, then calls a second routine with one field of the structure and a third with another field.
- `allocate_entry_pointer_array`: When a flag is set, counts the list entries, allocates a cleared array with one pointer slot per entry, then allocates a block for each entry and stores its pointer in the array.
- `build_entry_pointer_array`: Counts the list entries, allocates a cleared array with one pointer slot per entry, and fills each slot with a newly allocated block copied from the entry.
- `build_entry_pointer_array_checked`: Counts the list entries, allocates a cleared pointer array, and fills each slot with a newly allocated block copied from the entry.
- `create_entry_pointer_array`: Counts the list entries, allocates a cleared pointer array with one slot per entry, and stores a copied block for each entry in its slot.
- `fill_entry_pointer_array_slots`: Copies the array pointer, then allocates a block for each remaining list entry, storing each block's pointer in the array and copying data into it.
- `fill_remaining_entry_slots`: Walks the remaining list entries, allocating a block for each, storing its pointer and copying the entry data into it.
- `walk_linked_list_dispatch_nodes`: Walks a linked list of nodes, passing each node's fields to unnamed routines and storing an unnamed routine result in an array slot, and stops at the end of the list or when an unnamed routine returns zero.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `jump_through_library_vector`: Loads a library base pointer from a global variable and calls the library function at offset minus 90 from that base.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `call_library_vector_114`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.
- `call_library_vector_180`: Library stub: loads a library base from a global variable and calls the library function at offset minus 180 of that base, returning its result to the caller.
- `call_library_vector_186`: Library stub: loads a library base from a global variable and calls the library function at offset minus 186 of that base, returning its result to the caller.
- `draw_layout_by_state`: Selects one of six screen layouts from a state byte and issues a fixed sequence of drawing and text unnamed routine calls with coordinates and resource pointers for each layout.
- `stub_empty_return`: Does nothing and returns immediately, so it is an empty stub with no body.
- `acquire_resources_and_render_mode`: Requests four library objects, checks that all four were obtained, then requests four more and dispatches on a mode byte to a per-mode sequence of drawing and unnamed routine calls.
- `run_mode_sequence_if_ready`: Fetches four values through unnamed routines and stores one in a pointer target, and returns zero if any value is zero.
- `dispatch_mode_routine_calls`: Switches on a mode byte and calls a fixed count of unnamed routines for each mode, with more calls for higher modes.
- `return_nothing`: This library code performs no operations and returns.
- `wait_and_normalize_result`: Writes a result slot from a structure field or clears it when the field is zero, then calls a library function with an all-ones argument.
- `send_and_normalize_result`: Passes a value from a global structure to an unnamed routine, then calls a second unnamed routine and maps a minus one return to zero.
- `return_nothing_alt`: This library code is empty.
- `configure_and_check_four_records`: If a mode byte is not four, writes fixed 16-bit values into four records.
- `check_three_calls_succeed`: Calls the same indirect function up to three times, stopping at the first call that returns minus one.
- `check_two_calls_succeed`: Calls the same indirect function twice, stopping at the first call that returns minus one.
- `check_call_not_failed`: Calls an indirect function once and returns true unless it reports minus one.
- `stub_return_true`: Returns one without doing anything else, so it is a constant-true stub.
- `wait_for_item_signal_mask`: Loops indefinitely, taking an item from a structure and passing it through an unnamed routine.
- `stub_return_zero`: Returns zero without doing anything else, so it is a constant-zero stub.
- `dispatch_by_request_type`: Reads a type code from a structure.
- `call_routine_then_return`: Calls one unnamed routine with no arguments and returns, so it is a thin wrapper around that unnamed routine.
- `call_routine_return_zero`: Calls one unnamed routine with no arguments and then returns zero.
- `execute_two_calls`: Calls the same unnamed routine twice in sequence and then returns one.
- `run_callback_sequence`: Calls three unnamed routines with the same argument, adds a fourth call unless a mode byte equals one, then makes two more calls, one with a pointer taken from a field of the argument.
- `format_protection_flag_lines`: Computes a flags value from a stored field masked to its low bits, then calls a routine four times with one flag bit each and the labels ARCHIVED, DELETEABLE, READABLE and WRITEABLE. It produces the text lines for a file protection display.
- `format_status_with_negation`: Chooses between an empty prefix and the word NOT depending on a flag argument, then calls a routine to write a status line that combines the prefix with a supplied string and a suffix.
- `format_text_into_stack_buffer`: Formats a template with two string arguments and a field value into a local stack buffer, then calls two routines that use the buffer.
- `measure_cstring_length`: Counts the bytes of a null-terminated string from the start of the pointer until the zero byte and returns that count.
- `find_string_terminator_index`: Scans forward from the second byte of a base pointer until a zero byte, then returns the index of that zero byte.
- `copy_cstring_to_buffer`: Copies a null-terminated byte string from a source pointer to a destination pointer, including the terminating zero byte.
- `copy_fixed_byte_count`: Copies a given number of bytes one at a time from a source pointer to a destination pointer.
- `copy_bytes_with_register_count`: Copies bytes from a source pointer to a destination pointer in a do-while loop, using a count held in a register, so at least one byte is always copied.
- `compare_cstrings_until_mismatch`: Advances two null-terminated strings together while their bytes match and neither reaches a zero byte, then returns whether the bytes at the stopping point are equal.
- `compare_cstrings_for_equality`: Compares two null-terminated strings byte by byte and returns true if they are equal, stopping at the first zero byte of either string.
- `count_bytes_through_terminator`: Walks a byte string until the first zero byte and returns the position after it, so the result counts the terminator as well as the characters.
- `run_three_library_operations`: Calls an unnamed routine to obtain a value, then makes three library function calls on a field of a structure, passing the parameter with ten subtracted in one of them.
- `call_string_formatter`: Formats into a 12-byte local buffer from a global template and an argument, then passes that buffer with the other parameters to a second routine.
- `copy_pascal_string_to_cstring`: Reads a length byte from the source, copies that many following bytes to the destination, and appends a zero terminator.
- `copy_bytes_and_terminate`: Copies bytes from a source to a destination up to a given end index and then writes a zero terminator after the copied data.
- `initialize_request_and_send`: Fills two structures with the supplied values and zeroed defaults, then calls unnamed routines and a library routine with an all-ones argument.
- `call_library_with_field_pointer`: Passes a pointer taken from a field of a structure to a library function and returns one.
- `run_library_setup_calls`: Makes two library calls on a field of a structure, one with a zero argument, then a third call that passes two parameters with ten subtracted from each of them.
- `call_raw_do_fmt`: Calls the exec formatted-output library routine through a pointer loaded from a global base, then returns.
- `store_byte_at_pointer`: Writes the low byte of a register to the memory location held in an address register and returns.
- `invoke_library_entry_minus_36`: Calls a library function through a pointer loaded from a global base at an offset of minus 36 bytes, then returns.
- `invoke_library_entry_minus_72`: Calls a library function through a pointer loaded from a global base at an offset of minus 72 bytes, then returns.

### version.library

- `library_entry_return_zero`: Is the program entry point of the version library and returns zero immediately, so it performs no work.
- `store_two_library_pointers`: Writes two register values into a structure at offsets 34 and 38 bytes and returns without further processing.
- `increment_counter_and_clear_flag`: Adds one to a 16-bit field at offset 32 of a structure and clears bit value 8 in a flag byte at offset 42.
- `decrement_counter_and_call`: Decrements a 16-bit counter at offset 32 of a structure.
- `finish_or_set_pending_flag`: If the 16-bit counter at offset 32 is zero, it calls two library functions and returns the value stored at offset 38.
- `read_value_after_library_calls`: Reads a 32-bit value from offset 38 of a structure, calls two library functions, and returns the value it read.
- `return_zero`: Library code that returns zero.

### Populous (1989)(Electronic Arts)[cr VF][a2].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_graphics_and_wait_frames`: Forbids multitasking, opens the graphics library, and configures the display with library calls and a fixed buffer address.
- `close_library_and_permit`: Restores the saved library base into the global pointer, closes that library through the exec close routine, restores a saved field at offset 50 of the base, and re-enables multitasking with the exec permit routine.
- `reset_tick_handler_state`: Backs up the active per-tick handler pointer, clears the active pointer, clears a flag and a counter, and points two text pointers at a shared buffer.
- `advance_text_tick_timer`: Runs once per tick.
- `shift_and_copy_bit_columns`: Copies the high bit of six source bytes into destination rows with a stride of 88 bytes, ORs in a second bit from each source byte, and then shifts a table of six 16-bit words left by one bit, repeated per count.
- `copy_six_bytes_with_stride`: Calls an unnamed routine, then copies six bytes from the source pointer into a table with a stride of 44 bytes.
- `match_byte_in_table`: Compares the byte at a text pointer with a current byte and steps through a table of entries spaced eight bytes apart until a zero entry, returning early on a match.
- `invoke_library_entry_minus_396`: Calls a library function through a pointer loaded from a global base at an offset of minus 396 bytes, then returns.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d1.
- `clear_block_and_copy_rows`: Clears a block of longwords, then copies a row of bytes into rows spaced 44 bytes apart from a source buffer, advancing through a cycling pointer table.
- `tiny_ori_3`: Small routine of 1 instruction that merges 202 into d5.
- `tiny_ori_4`: Small routine of 1 instruction that merges 44 into d0.
- `copy_table_until_sentinel`: Copies longword entries from a source table to a destination table until the next source entry equals 65535, then writes a saved value after the last copied entry.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 44 into d0, then merges 255 into d0.
- `copy_table_until_sentinel_2`: Copies longword entries from a source table to a destination table until a sentinel value is encountered, then writes a final value.
- `compute_row_index_code`: Divides a stored value by 44, the row stride, and returns the quotient with 31 subtracted from its low byte.
- `interleave_source_bits_into_rows`: Copies a block of longwords into a work area, then for each of 32 rows shifts bits from three source bytes through two 16-bit accumulators and stores two words per row in a table.
- `fill_row_value_table`: Fills 60 pairs of longwords in a table, where the first longword of each pair steps by a fixed increment per row and the second is a constant.
- `copy_column_with_wrap`: Copies 60 bytes from a source table into a destination spaced eight bytes apart, resetting the source pointer to a fixed address when the next source byte is 255, then advances a global pointer.

### Populous (1989)(Electronic Arts)[cr VF][a2].adf:DAD (raw code file, load address unknown)

- `toggle_cpu_interrupts_2`: Enables or disables CPU interrupts by writing to the interrupt enable register, then exchanges data with an external serial device.
- `toggle_cpu_interrupts`: Enables or disables CPU interrupts by writing to the interrupt enable register, using bit 15 to enable and its absence to disable.
- `process_text_reference`: References a string of text in the program's data, indicating the routine operates on or displays that textual content.
- `run_short_operation`: A short routine of ten instructions invoked by a single caller, with no referenced text or hardware access shown.
- `handle_serial_transfer`: Invokes serial read and serial write operations, so the routine exchanges data with an external serial device.
- `control_interrupt_enable`: Writes to the interrupt enable register, using a set bit fifteen to enable interrupts and its absence to disable them.
- `delegate_to_helper`: A twelve-instruction routine that delegates its work to a brief six-instruction helper routine, with no direct text or hardware access.
- `run_common_helper`: A brief six-instruction routine invoked by two different callers, with no referenced text or hardware access shown.
- `reference_graphics_text`: A thirty-four-instruction routine that references graphics and intuition library text and delegates to a brief six-instruction helper routine.
- `store_byte_via_table_2`: Stores a byte from a register through a pointer, then loads a byte from a table indexed by a value and stores it through another pointer.
- `scan_table_max_value_2`: Walks through a table of entries, reading a byte from each one and using it to index into a second table of values, then stores the selected value through another pointer.
- `invoke_medium_processes`: A lengthy one-hundred-fifty-two-instruction routine that delegates to a medium sixty-two-instruction routine and a medium thirty-seven-instruction routine.
- `invoke_complex_procedure_2`: Delegates its work to a single larger routine that performs the actual processing before returning.
- `invoke_complex_procedure`: A fifty-six-instruction routine that delegates its work to a lengthy one-hundred-twenty-seven-instruction procedure invoked by four callers.
- `run_complex_procedure`: A lengthy routine of 127 instructions that calls one other routine and is invoked by four different callers.
- `run_extended_operation`: A lengthy routine of 103 instructions invoked by a single caller, with no referenced text or hardware access shown.
- `run_detailed_process`: A lengthy routine of 95 instructions invoked by a single caller, with no referenced text or hardware access shown.
- `invoke_shared_routines`: A very long six-hundred-ninety-three-instruction routine that delegates to a medium thirty-seven-instruction routine and a short eight-instruction routine.
- `run_medium_process`: A medium routine of 62 instructions invoked by a single caller, with no referenced text or hardware access shown.
- `run_shared_subroutine`: A medium routine of 37 instructions invoked by two different callers, with no referenced text or hardware access shown.
- `call_routine_trio`: A routine that is invoked by three other routines and, in turn, calls one additional routine.
- `invoke_routine_pair`: A forty-instruction routine that delegates its work to a short eight-instruction routine invoked by two other routines.
- `call_routine_single`: A routine that is invoked by a single caller and contains a large body of instructions.
- `call_routine_pair`: A very short routine consisting of only eight instructions, invoked by two other routines.
- `call_routine_small`: A routine that is invoked by a single caller and contains a moderate body of instructions.
- `copy_table_entries_2_2`: Walks through a table of entries, reading a byte from each one and using it to index into a second table of values, then stores the selected value through a pointer.
- `copy_table_entries_2`: Walks through a table of entries, reading a byte from each and storing it at a running pointer while advancing that pointer.
- `copy_table_entries`: Walks through a table of entries, reading a byte from each entry and storing it at a running pointer, then advances that pointer by the entry's size field.
- `invoke_tiny_routine`: A five-hundred-five-instruction routine that delegates its work to a short routine invoked by a single caller.
- `call_routine_tiny`: A routine that is invoked by a single caller and contains a small body of instructions.
- `process_table_entries`: Iterates over a table of 32-bit values, loading each one and passing it to another routine, then stores the returned value back into the same table slot.
- `index_and_store_table_value`: Loads a value from a table using an index, then stores that value through a pointer.
- `call_routine_six`: A routine that is invoked by six other routines and contains a substantial body of instructions.
- `call_routine_four`: A routine that is invoked by four other routines and contains a moderate body of instructions.
- `invoke_shared_helpers_2`: Invokes two helper routines, one shared by six callers and one shared by four, to perform a combined block of processing.
- `invoke_shared_helpers`: Invokes two other routines, one shared by six callers and one shared by four callers, to perform a combined block of processing.
- `call_six_caller_helper`: Runs a moderately sized routine shared by six callers to complete its processing block.
- `do_nothing_2_2`: This routine is empty and contains no instructions.
- `chain_multiple_helpers`: Chains together three helper routines, including a lengthy one and two shared helpers, to carry out a larger block of processing.
- `scan_table_max_value`: Walks through a table of entries, reading a value from each one and storing it through a pointer, then compares that value against a running value and keeps the larger one, updating the stored result as it goes.
- `execute_processing_chain`: Runs a lengthy 127-instruction routine that in turn invokes one other routine, then executes a three-routine trio that calls one additional routine, and finally chains three helper routines (one lengthy routine plus two shared helpers) to carry out a larger block of processing.
- `do_nothing_2`: This routine is empty and contains no instructions.
- `store_byte_via_table`: Stores a byte from a register through a pointer, then loads a byte from a table indexed by a value and stores it through another pointer, repeating this load-and-store cycle for a fixed number of iterations.
- `store_byte_from_table`: Loads a pointer from a table, fetches a byte from the pointed-to location, and stores that byte into a global variable.
- `adjust_table_value`: Loads a value from a table using an index, adds a constant to it, stores the result back through a pointer, and returns the computed value.
- `copy_data_block`: Copies a block of data from a source address to a destination address, using a loop that decrements a counter while transferring word-sized values through a pointer.
- `index_table_via_byte`: Walks a table of entries, reading a byte from each one and using it to index into a second table of values, then stores the selected value through a pointer.
- `store_table_byte_4`: Loads a byte from a table using an index and stores the resulting byte through a pointer.
- `store_table_byte`: Loads a value from a table using an index, then stores the resulting byte through a pointer.
- `iterate_linked_list_nodes_2`: Walks a linked list of nodes, each pointed to by a stored address, and for every node calls a routine that reads a value from the node and stores it through a pointer.
- `process_table_entries_2`: Iterates over a table of 376 entries, reading a value from each slot and storing a computed result back through a pointer.
- `delegate_to_shared_helper`: Called by three routines and delegates its work to a shared helper routine used from multiple callers.
- `index_table_multiply_store_2`: Loads a value from a table using an index, multiplies that index by a second value, and stores the resulting product through a pointer.
- `index_table_multiply_store`: Loads a value from a table using an index, multiplies that index by a second value, and stores the resulting product through a pointer.
- `process_table_entries_4`: Processes a table of 376 entries by reading a value from each slot and storing a computed result back through a pointer.
- `process_table_entries_3`: Iterates over a table of entries, reading a value from each slot and storing a computed result back through a pointer, while referencing the text CONNECTION MADE and TRY NUMBER.
- `serve_multiple_callers`: A 50-instruction routine that is invoked by four different unnamed callers, indicating it performs a shared helper task used from multiple places in the program.
- `handle_complex_calculation_2`: Implements a substantial calculation routine called by a single unnamed routine.
- `handle_complex_calculation`: A large 273-instruction routine, the biggest in this set, that is called by a single unnamed routine and therefore implements a substantial self-contained block of logic.
- `mark_task_nodes_active`: Walks a linked list of task nodes by following each node's next-pointer, writing a marker byte into each node to flag it as active, and stopping when the end of the chain is reached.
- `iterate_task_nodes`: It walks a linked list of task nodes, marking each one as active and advancing to the next entry until it reaches the end of the chain.
- `perform_small_task`: A short 23-instruction routine invoked by one unnamed caller, so it carries out a small, focused piece of work.
- `run_modest_processing_2`: Runs a short block of processing from a single helper routine before returning.
- `run_modest_processing`: A 38-instruction routine called by a single unnamed routine that executes a modest amount of processing before returning.
- `store_table_byte_and_call_dos`: Loads a byte from a table using an index and stores it through a pointer, while invoking DOS and Exec library services and referencing the text .GAM.
- `execute_short_sequence`: A compact 20-instruction routine invoked by one unnamed caller that performs a brief sequence of operations.
- `run_brief_operation`: A compact 20-instruction routine called by a single unnamed routine that runs a short block of instructions to completion.
- `fill_table_and_call_libraries_2`: Initializes a data table by writing a byte from a register through a pointer and repeating until the table is filled.
- `do_minimal_work`: A very short 16-instruction routine invoked by one unnamed caller, so it does only a little work before returning.
- `load_game_data_file`: A 262-instruction routine that references data from a .GAM file and is called by a single unnamed routine, so it loads or processes external game data.
- `manage_interrupt_flags`: A 20-instruction routine that reads and writes the 68000 interrupt request registers, so it manipulates interrupt flags.
- `control_custom_interrupt`: A 22-instruction routine that touches the interrupt request register and a custom chip register, and is called by two unnamed routines, so it coordinates a hardware interrupt or custom-chip setting.
- `invoke_undocumented_routine`: It is invoked by a single unnamed routine and contains 49 instructions, but the decompiler failed so the exact operations are not visible.
- `transfer_serial_data_byte`: It writes to and reads from the serial data register while being called by two unnamed routines, indicating a serial-port data transfer.
- `store_table_byte_2`: Loads a byte from a table using an index and stores it through a pointer.
- `call_dos_and_exec_library`: It calls the DOS library together with the Exec library, which handle file-system and core operating-system services, and is entered once by an unnamed routine.
- `call_dos_library`: It calls the DOS library, which handles file-system access, and is entered once by an unnamed routine.
- `store_byte_from_register`: Stores a byte from a register through a pointer, then loads a value from a table indexed by a register and stores it through another pointer.
- `call_single_library_service`: It calls a single library service and is entered once by an unnamed routine.
- `fill_table_and_call_libraries`: Initializes a data table by writing a byte from a register through a pointer and repeating until the table is filled, while invoking several Amiga OS libraries.
- `store_table_byte_3`: Loads a byte from a table indexed by a value in a register and stores that byte through a pointer held in another register.
- `fill_table_with_byte`: Initializes a data table by writing a byte value from a register through a pointer, then increments the pointer and repeats the store until a counter reaches zero.
- `call_two_libraries`: It calls two different libraries and is entered once by an unnamed routine.
- `invoke_os_libraries`: Invokes three different Amiga OS libraries, identified by negative library numbers, during a single call from one unnamed routine.
- `invoke_multiple_libraries`: Invokes seven different Amiga OS libraries, identified by negative library numbers, within a single call from one unnamed routine.
