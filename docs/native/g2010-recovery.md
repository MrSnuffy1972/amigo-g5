# Game 2010 (populous (1989)(electronic arts)[cr qtx][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02010). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 35 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 350 raw-word lines |
| Functions named and described | pass, 525 of 525 named (0 author symbols), 525 of 525 described (80 by rule, 77 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 708 of 708 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 11131 of 11898 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 649065 |
| code | 38826 |
| data | 187379 |
| padding | 9978 |
| unused | 15872 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| type | 2 | 117 | 2 |
| endcli | 2 | 117 | 2 |
| loadwb | 12 | 536 | 30 |
| wait | 2 | 117 | 2 |
| Disk-Validator | 1 | 386 | 1 |
| Populous (1989)(Electronic Arts)[cr QTX][a].adf:populous | 3 | 4352 | 145 |
| Populous (1989)(Electronic Arts)[cr QTX][a].adf:populous.prg | 3 | 1760 | 9 |
| icon.library | 10 | 1375 | 109 |
| translator.library | 2 | 490 | 50 |
| info.library | 25 | 2597 | 167 |
| version.library | 2 | 41 | 7 |
| Populous (1989)(Electronic Arts)[cr QTX][a].adf:bootblock | 1 | 10 | 1 |

## Cases handled

- populous (1989)(electronic arts)[cr qtx][a]/Populous (1989)(Electronic Arts)[cr QTX][a].adf: file WOW2.GAM (header 4 bytes, blocks hold 488): its blocks are judged as raw blocks outside the filesystem
- populous (1989)(electronic arts)[cr qtx][a]/Populous (1989)(Electronic Arts)[cr QTX][a].adf: blocks 1025-1033 (bytes 524800-529408) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.72)
- populous (1989)(electronic arts)[cr qtx][a]/Populous (1989)(Electronic Arts)[cr QTX][a].adf: blocks 1037-1044 (bytes 530944-535040) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.68)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_header_block_to_buffer`: If the output pointer is non-null, copies sixteen words from a fixed offset in a structure into the output buffer.

### endcli

- `run_endcli_main_program`: Main entry of the command shell.
- `copy_words_then_call`: Copies four-byte words from a fixed base address plus offset into a caller-supplied destination buffer, repeating the store for a fixed count, then jumps to a routine whose address is held in a global.

### loadwb

- `run_loadwb_startup_sequence`: Program entry for the workbench loader: finds its own task, waits for and reads its startup message, opens the dos library and sets the current directory.
- `release_resources_and_reply`: Cleanup routine that finds the current task, restores a saved handler pointer and, if a stored resource exists, closes its file and library, frees its memory and replies to a message.
- `launch_workbench_tool_with_messages`: Starts a Workbench tool by looking up its resident tag and launching it, printing a 'Cannot find resident tag' or 'Cannot start tool' message on failure.
- `compare_null_terminated_strings`: Compares two null-terminated strings byte by byte.
- `search_resident_list_for_tag`: Walks a list of entries, looking up a resident tag for each through a library call and allocating memory for matches.
- `call_library_vector_96_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_132_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_366_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `pad_name_with_separator`: Copies a short separator string onto a name buffer when its type byte is two, and compares a global string against the buffer.
- `write_dot_and_spaces_suffix`: Writes a dot followed by four spaces into a buffer at a fixed offset, forming a separator suffix.
- `compare_global_string_prefix`: Compares a global string with a buffer character by character, stopping at the global string's terminator or at the first mismatch.
- `stub_return_only`: An empty stub that only returns to its caller and does no work.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `forward_call_with_stack_pointer`: Calls another routine, passing its two arguments and a pointer to the caller's stack argument, then returns.
- `format_string_and_measure_length`: Formats text into a 140-byte stack buffer using the exec formatting library call, then counts characters up to the terminator and returns the length.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### wait

- `run_wait_program_main`: Main entry of the wait command.
- `copy_block_and_call`: Copies a fixed-size block of words from one location to another when the destination is not null, then invokes another routine.

### Disk-Validator

- `run_disk_validator_main`: Entry routine of the disk validator program.

### Populous (1989)(Electronic Arts)[cr QTX][a].adf:populous

- `clear_bss_block_at_entry`: Program entry that calls an unnamed routine and then zeroes a block of memory by writing zero words across a fixed count.
- `setup_display_handles_and_fields`: Stores handles returned by library calls into globals and initialises several fields of a display structure through a series of calls.
- `start_copper_list_from_record`: Loads a copper list address from a field of a record and passes it to the copper start routine.
- `enable_dma_and_interrupt_channels`: Calls several unnamed routines with constant channel, size and enable values, and writes the interrupt enable register.
- `create_process_and_load_segment`: Allocates a structure, loads the program segment from named files and creates a process for it through library wrappers.
- `allocate_and_fill_offset_table`: Allocates a block of 1152 bytes, stores its address in a global, and fills an array with offsets stepping by 144 for each row.
- `store_constant_and_call`: Stores the constant 399 into a word field of a global structure and then calls an unnamed routine.
- `call_global_pointer_with_size`: Loads a pointer from a global table and calls an unnamed routine with it and a fixed size of 1152 bytes.
- `call_two_global_pointers`: Calls two unnamed routines, passing each the pointer found in a global table entry.
- `call_library_vector_with_word`: Calls a library function through a base loaded from a global, passing its argument and the low word of the parameter.
- `forward_call_and_return`: Calls one unnamed routine and returns.
- `lookup_pair_and_call`: Searches a two-entry table of globals for the second argument, and when it is found calls an unnamed routine with the paired value and stores the parameter.
- `swap_globals_and_call`: Swaps two global words, then calls an unnamed routine with the swapped values.
- `open_graphics_and_init_fields`: Opens the graphics and intuition libraries through calls, stores the returned bases in globals, sets a status field of a structure to minus one and initialises several other fields.
- `wait_for_result_then_call`: Repeatedly calls an unnamed routine until it returns one, two or three, then calls another routine with a global pointer and a fixed offset.
- `load_picture_file_with_status`: Clears a status global, calls four unnamed routines to load the picture file named in the referenced text, and returns a status value derived from the calls.
- `install_handlers_and_init_state`: Installs two routine pointers in a global table, writes several constants into globals and clears a short array.
- `update_state_tables_branching`: Large routine that updates several state variables and tables in globals through eleven unnamed calls, branching on the values it reads.
- `empty_routine`: Empty routine that returns immediately without touching any data or register.
- `fill_row_table_when_zero`: Calls an unnamed routine in a loop and, when it returns zero, fills a row table with the value one hundred and sixty before processing further entries.
- `fill_modular_offset_arrays`: Loops over the count argument and stores two short arrays, each value reduced modulo thirteen and offset by two hundred thirty-two, using results from an unnamed routine.
- `update_offset_arrays_return_first`: Loops over a count and updates two short arrays by differences and increments, then returns the first updated value.
- `prepare_demo_picture_display`: Calls several library routines to prepare the display state, then loads the demo picture named in the referenced text and updates display values.
- `build_offset_stepped_entry_list`: Fills a list of entries in memory with offsets that step by three hundred for one field and by thirty-two thousand for another, copying parameter values into each entry.
- `read_block_from_file`: Reads a block of 1005 bytes from the open file and, when the read succeeds, copies 32000 bytes into two buffers.
- `load_font_and_sprite_data`: Reads a block from the file and passes the data to several unnamed routines that set up font and sprite data.
- `read_block_and_dispatch`: Reads a block of 1005 bytes and, when the read succeeds, passes it to unnamed routines with sizes of 40000 and 64.
- `rotate_and_merge_word_tables`: Rotates a 16-bit word table by an amount derived from a nibble, then combines the rotated words with a destination buffer using bitwise AND and OR operations.
- `apply_rotated_mask_to_block`: Processes a block of words using a mask table indexed by an input value, rotating a 32-bit mask on each iteration and writing combined words to the destination.
- `call_and_return_register_pair`: Calls one unnamed routine and returns its two input registers unchanged.
- `combine_masked_table_words`: Indexes a table with a global value and a register value, then uses the entry to mask and combine words at offsets derived from two registers.
- `reorder_table_entry_fields`: Iterates over a table of entries and reorders fields within each one by copying and shifting words and bytes, writing the transformed values back into the same table.
- `shift_byte_buffer_left_three`: Shifts each byte of a fixed hundred-byte buffer left by three bits in a simple loop.
- `run_nested_index_loops`: Runs nested loops over a range of indices, calling an unnamed routine for each index that looks up a table with a global value and a register value.
- `call_and_concatenate_registers`: Calls an unnamed routine and returns a 64-bit value formed from two data registers.
- `store_global_address_pointer`: Stores the address of a global buffer into a pointer variable and returns.
- `copy_eighty_character_entries`: Loops over eighty entries, copying character bytes from a source into a destination and clearing them.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `open_display_with_mode_setup`: Opens the graphics library through a library vector and, on success, sets display mode values and clears a status byte.
- `dispatch_handler_branch`: Invokes a routine from a global, and depending on its result branches through a table of handlers, saving and restoring a global pointer, zeroing a byte at a fixed offset, and returning a value.
- `stub_bad_instruction_data`: A one-instruction fragment decoded from a gap in the code.
- `install_timer_interrupt_handler`: Loads a sound sample pointer from a table and, when the sample is present, registers a timer interrupt by installing a handler and setting the interrupt enable register, then clears a flag at a fixed offset in the handler's data.
- `resolve_and_store_structure`: It resolves a chain of pointers from a global table to obtain a data structure, then stores a fresh value into a field of that structure.
- `load_popsam_sound_sample`: Opens the sample file named popsam, allocates memory for its contents and reads it in.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_status_byte_and_return`: Clears one status byte in a structure at a fixed offset and returns the stack argument unchanged.
- `free_memory_and_restore_state`: Restores saved interrupt and display globals, calls a library routine to free memory, and resets the state.
- `dispatch_by_character_code`: Compares a character argument with two codes and calls one of two unnamed routines for each, then clears a status byte in the structure.
- `clear_status_after_conditional_call`: Calls an unnamed routine when the character argument equals one, then clears a status byte in the structure.
- `clear_struct_status_byte_2`: Writes a zero to a byte at a fixed offset within the structure pointed to by register A0 and returns the stack argument unchanged.
- `clear_struct_status_byte`: It writes a zero to a byte located at a fixed offset within a structure pointed to by a register, then returns the stack argument unchanged.
- `clear_struct_status_via_a0`: Writes a zero to a byte located at a fixed offset within a structure pointed to by register A0, then returns the value that was originally on the stack unchanged.
- `clear_status_byte_return`: Writes a zero to a byte at a fixed offset in a structure passed on the stack, then returns that stack argument unchanged.
- `stub_bad_instruction_data_2`: A one-instruction fragment decoded from a gap.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `build_dispatch_tables_from_globals`: Builds dispatch tables from values in a global variable table, calls two unnamed routines, and ends in a return from exception.
- `clear_state_and_set_defaults`: Clears a state byte and sets two global words to fifteen and two hundred fifty-five before calling an unnamed routine.
- `start_audio_channel_from_record`: Selects an audio channel from a table and writes its location, length, period and volume registers from a sample record.
- `update_audio_channel_from_samples`: Copies sample bytes from two sources and updates an audio channel's location and length registers, then sets the DMA control register.
- `check_flag_bits_and_signal`: Tests two bits in a global variable and, when either is set, writes a byte of 255 and stores the masked bits before calling an unnamed routine.
- `write_audio_channel_registers`: Copies sample bytes between two buffers while updating an audio channel's location, length and period registers.
- `update_audio_channel_period`: Updates an audio channel's period and location registers from a sample pointer and calls one unnamed routine.
- `update_audio_channel_volume`: Updates an audio channel's location, length and volume registers from a sample record and calls one unnamed routine.
- `stub_bad_instruction_data_3`: A one-instruction fragment decoded from a gap in the code.
- `stub_bad_instruction_data_4`: A three-instruction fragment decoded from a gap in the code.
- `copy_sample_bytes_to_buffer`: Copies bytes from a sample source into a buffer and sets an audio channel's location and period registers.
- `enable_audio_dma_for_channel`: Writes an audio channel's pointer, length and period registers and sets the DMA control register to enable that channel.
- `load_audio_sample_for_channel`: Loads an audio sample for a channel by writing its location, length and period registers, calling four unnamed routines to prepare the data.
- `advance_audio_fragment`: Updates a playing audio fragment by decrementing its sample counter and, when it reaches zero, loads the next sample and period from a lookup table.
- `update_audio_channel_fragment`: A short fragment that writes audio channel location and length registers from a sample pointer and calls one unnamed routine.
- `update_audio_channel_fragment_2`: Updates a playing audio channel by decrementing its sample position and, when the fragment ends, advancing to the next sample and reconfiguring the channel's pointer, length, period and volume registers.
- `combine_words_with_mask_table`: Iterates over a block of words, combining each with entries from a global table using bitwise AND, OR and NOT. It updates four spaced locations per pass.
- `mask_destination_words_from_source`: Loops over source and destination word arrays, masking each destination word with its source word and a global table using AND, NOT and OR. It updates four spaced locations per pass.
- `store_stack_word_and_call`: Copies a word from the stack into a global variable, calls an unnamed routine, and returns its two input registers unchanged.
- `call_other_routine`: Invokes another routine and then returns without performing any other observable operation.
- `read_mouse_movement_deltas`: Reads the mouse port movement and button state, converting the change since the last sample into clamped signed deltas.
- `split_word_into_byte_globals`: Stores the low byte and high byte of a global word into two separate globals, then calls an unnamed routine.
- `save_and_enable_interrupt_bits`: Saves two globals and writes an interrupt enable value with the set bit, then calls nothing further and returns.
- `clear_interrupts_when_flagged`: If a global flag is set, writes the interrupt clear mask and then the enable value, clears the flag and returns the register value.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `init_dma_list_structure`: Initialises a structure from its parameters: zeroes its first word, packs fields into the second, and copies shorts from the source through a loop.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `wait_for_copper_bit_set`: Stores a parameter in a global, then loops until bit five of the global is set and returns the combined value.
- `spin_until_status_bit_set`: Writes a constant to a global variable and spins until bit five of that global is set, then returns the bit.
- `copy_shorts_after_marker`: Searches a short array for the marker value of 384, then copies a count of shorts from a source pointer into the array just after the marker.
- `routine_pseudocode_unavailable`: Decompiled pseudo-code is unavailable for this routine, so only its layout is known: it is 21 instructions long and called from one place.
- `select_byte_buffer_by_index`: Looks up a byte buffer pointer from a table using a parameter, then updates a byte in that buffer using bit masks.
- `decode_keyboard_serial_byte`: Reads the keyboard serial register and decodes the key code against a character table of digits and symbols, then updates keyboard state globals.
- `copy_eight_thousand_words`: Copies eight thousand words from a source pointer to a destination pointer, one word per step.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `copy_words_via_stack_buffer`: Copies a block of words from a source pointer into a 32-word stack buffer, then processes it in rows while reading the video beam position register.
- `call_then_forward_with_result`: Calls one unnamed routine with no arguments, then calls another with its two parameters and returns that second result.
- `call_then_conditional_follow_up`: Calls an unnamed routine with its parameter, then calls a second unnamed routine only when a global variable is nonzero.
- `advance_global_return_high_bits`: Calls an unnamed routine, adds a fixed constant to its result, stores the sum in a global, and returns the high fifteen bits of it.
- `open_dos_library_or_alert`: Stores the high word of its argument in a global.
- `return_immediately_2`: Returns immediately to its caller without executing any instruction.
- `setup_dos_input_and_directory`: Allocates a buffer, reads input through the dos library and sets the current directory.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `scan_text_commands_and_dispatch`: Scans a text buffer character by character and calls one of four unnamed routines for each matched entry.
- `append_string_with_limit`: Finds the end of the destination string, then copies characters from the source into it, stopping at the terminator or after a limit of 32766 characters.
- `append_bytes_up_to_count`: Finds the end of the destination string and appends bytes from the source until a null terminator is reached or the count runs out.
- `copy_string_and_zero_pad`: Copies bytes from the source to the destination up to a count or until a null terminator, then fills the rest of the count with zeros.
- `multiply_long_pair_to_quad`: Multiplies two 32-bit values from the registers, building a 64-bit product from the high and low halves of each operand.
- `open_icon_library_and_file`: Opens a file through the dos library and then the icon library through a library vector, storing the handle in a global.
- `call_optional_handler_then_routine`: Calls a stored function pointer when it is set, then calls one unnamed routine.
- `call_optional_handler_then_routine_2`: Checks a global variable and calls the stored function pointer when it is nonzero, then calls one unnamed routine.
- `process_global_flags_and_cleanup`: Loops over a six-byte table of entries using its count and releases each one, then calls the release routine for each other non-zero pointer slot in the state block.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `release_table_slot_by_index`: Checks that the index is inside the table count and that the slot holds a non-zero entry, otherwise it records error code 2 and returns minus one.
- `dispatch_via_jumptable`: Reads a dispatch offset from a table via a base pointer and jumps to the referenced routine.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_load_seg`: Wrapper that calls the dos library function LoadSeg and returns its result to the caller.
- `invoke_indirect_routine`: It reads a pointer from a table indexed by a value derived from a global variable, then calls the routine at that address.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Calls the function at offset minus 414 from a library base stored in a global variable, with no arguments, and returns.
- `invoke_indexed_routine`: Looks up a routine in a jump table indexed by a value stored in a global variable and then calls it.
- `call_library_vector_198_2`: Calls the function at offset minus 198 from a library base stored in a global variable, with no arguments, and returns.
- `invoke_jumptable_entry`: Looks up a function in a jump table and invokes it, passing the value stored at a computed table index.
- `call_library_vector_294_2`: Calls the function at offset minus 294 from a library base stored in a global variable, with no arguments, and returns.
- `free_input_requestable`: Looks up a pointer in a table indexed by an offset and jumps to it.
- `call_library_vector_132_2`: Calls the function at offset minus 132 from a library base stored in a global variable, with no arguments, and returns.
- `invoke_library_routine`: It looks up a routine in a jump table within an Amiga OS library and calls it, passing the library's base address in register A4.
- `call_library_vector_210_2`: Calls the function at offset minus 210 from a library base stored in a global variable, with no arguments, and returns.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372_2`: Calls the function at offset minus 372 from a library base stored in a global variable, with no arguments, and returns.
- `call_library_vector_552_2`: Calls the function at offset minus 552 from a library base stored in a global variable, with no arguments, and returns.
- `invoke_vector_from_table`: Looks up a function in a jump table and calls it, passing the value stored at a fixed offset within a structure pointed to by a4.
- `call_library_vector_138_2`: Calls the function at offset minus 138 from a library base stored in a global variable, with no arguments, and returns.
- `call_library_vector_366`: Calls the function at offset minus 366 from a library base stored in a global variable, with no arguments, and returns.
- `call_library_vector_378_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_library_vector_384`: Calls the function at offset minus 384 from a library base stored in a global variable, with no arguments, and returns.
- `call_library_vector_462_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_96_2`: Calls the function at offset minus 96 from a library base stored in a global variable, with no arguments, and returns.
- `call_library_vector_90_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_78_2`: Calls the function at offset minus 78 from a library base stored in a global variable, with no arguments, and returns.

### Populous (1989)(Electronic Arts)[cr QTX][a].adf:populous.prg

- `save_registers_then_stop`: Copies the incoming register values into fixed memory slots and stores the address of a routine in a global variable.
- `apply_base_offset_fixups`: Walks a table of block offsets and adds a base value to each longword that the table names inside the block.
- `process_entries_copy_words`: Walks a linked list of entries while executing per-entry code and accumulating values into a buffer, then copies thirteen 16-bit words from a table into a fixed memory location.
- `process_entries_and_copy_words`: Walks a linked list of entries while executing per-entry code and accumulating values into a buffer, then copies thirteen 16-bit words from a code table into a fixed memory location.
- `copy_words_to_fixed_memory`: Copies thirteen 16-bit words from a table in the program's code to the fixed memory address 998 and returns.
- `delay_loop_count_down`: Counts a register value down to zero in an empty loop, which wastes time in proportion to the value, and then returns.
- `copy_words_to_fixed_memory_2`: Copies eight 16-bit values from a source table into a fixed destination location, decrementing a counter until the block is fully transferred.
- `copy_and_zero_longword_block`: Copies longwords from one buffer to another up to a bound, then zeroes the longwords that follow in the destination.
- `record_undecompiled_routine`: The decompiler produced no pseudo-code for this 403-instruction routine, so its behaviour cannot be recovered from the listing.

### icon.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `increment_counter_clear_bit`: Library code that increments a 16-bit field at offset 32 of a structure and clears bit 3 of the byte at offset 14 of the same structure, then returns.
- `decrement_library_refcount`: Decrements a 16-bit counter at offset 32 of a structure.
- `unlink_node_or_set_flag`: If the word at offset 8 of the structure is zero, it removes the structure from its doubly linked list by joining the neighbouring links, then calls the library function at offset minus 210 from a pointer at offset 34 and returns the longword at offset 42.
- `set_structure_flag_return_zero`: Sets bit 3 of the byte at offset 14 of the structure addressed by a register and returns zero.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `store_library_base_or_alert`: Saves a pointer and a library base into a structure, then calls the library function at offset minus 408 from that base and stores its result at offset 38.
- `copy_two_pointers_from_table`: Copies the longword at offset 34 and the longword at offset 38 of the structure addressed by a register into two global variables.
- `forward_call_and_return_2`: Calls another routine and then returns, with no other work on registers or memory.
- `forward_call_and_return_3`: Calls another routine and then returns, with no other work on registers or memory.
- `call_library_handler`: Calls another routine and then returns, performing no other operations.
- `forward_call_and_return_4`: Unconditionally calls another routine and returns, with no other work on registers or memory.
- `forward_call_and_return_5`: Calls another routine and then returns, with no other work on registers or memory.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `call_library_handler_2`: This library routine simply calls another routine and then returns.
- `forward_call_with_offset`: This library routine calls another routine with the same first argument and the second argument increased by a fixed offset, then returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `call_icon_init_routine`: This library routine invokes a single initialization function to set up icon data structures and then returns.
- `initialize_icon_header_record`: It allocates a 94-byte cleared record, invokes a routine on the field at offset 80, then calls another routine on the field at offset 78 to store a value there.
- `call_library_handler_3`: This library routine invokes another routine and then returns.
- `call_other_routine_2`: The routine calls another routine and then returns.
- `call_other_routine_3`: This routine invokes another routine and then returns.
- `call_other_routine_4`: This routine invokes another routine and then returns without performing any other operations.
- `store_param_at_returned_offset`: Calls the function at offset minus 294 from a base pointer kept at offset 34 of the structure, passing the structure, then stores the second argument as a longword at offset 148 of the pointer it returns.
- `obtain_pointer_or_release`: Calls one routine with the last two arguments to obtain a pointer.
- `load_icon_object_from_header`: Obtains an object and reads a header into a stack buffer through a library call.
- `copy_header_fields_to_object`: Copies ten longwords and several individual fields from a stack buffer into the object at fixed offsets, then links the object into its parent when one is set.
- `return_zero_stub_2`: Returns the constant zero and does no other work.
- `copy_icon_data_to_stack`: Copies ten longwords and several fields from the object at offset 96 onward into a local buffer on the stack, sets two leading constant words in that buffer, and passes its address to another routine together with the first two arguments.
- `load_info_icon_file`: Works with the .info icon file suffix, opens the file through a library call and reads its header into the caller's structure.
- `read_icon_file_sections`: Reads the optional icon sections from an already open file, checking the header magic and version first.
- `read_icon_optional_fields`: Reads two longwords from an open file and stores them in the structure, then reads optional values for the fields at offsets 50, 54 and 70, where the field at 54 receives a counted list of longwords.
- `read_counted_longword_list`: Reads a counted list of longwords through a callback pointer, reading each length, obtaining a buffer for it and reading its data, until the remaining count runs out.
- `call_routine_and_return_zero`: Calls another routine and then returns zero, with no other work.
- `empty_routine_3`: It executes four instructions and then returns without performing any observable work.
- `load_icon_with_work_buffer`: Obtains a 260-byte work buffer, then reads the icon header and its optional sections into the structure through the routines it calls.
- `open_and_read_info_icon`: Saves the structure's field at offset 26, opens the .info icon file through a library call at offset minus 30, and reads its header and optional sections through callbacks.
- `read_optional_icon_fields`: Reads two longwords from an open file and stores them in the structure, then reads optional values for the fields at offsets 50, 54 and 70, where the field at 54 receives a counted list of longwords.
- `write_icon_pointer_list`: Walks a null-terminated list of pointers to measure the total length of its entries, then passes each entry's length and data to a callback and handles the optional field at offset 70 the same way.
- `iterate_icon_table_entries`: Walks a null-terminated table of pointers, computing the length of each entry with a routine call and passing it through two callbacks, then handles the optional field at offset 70 the same way.
- `visit_linked_list_nodes`: Follows the chain of node pointers stored in the structure, passing each node to another routine, and stops when the list returns to its start or no next node exists.
- `iterate_linked_list_nodes`: Repeatedly passes the current node to another routine and advances to the next node, stopping when no node is returned or the structure's offset-8 link is reached.
- `forward_call_with_offset_140`: Calls another routine with the same first argument and the second argument increased by 140, then returns.
- `allocate_and_initialize_record`: Obtains a 168-byte cleared record, calls a routine on its field at offset 142, then calls another routine with the field at offset 140 to set it up.
- `invoke_handler_and_extend_table`: It invokes a table handler and then extends a growing table by decrementing a counter, returning zero if that extension fails.
- `empty_no_operation`: This routine is completely empty; it performs no operations and simply returns without touching any data or registers.
- `store_pair_growing_table`: Decrements a short counter and, when it runs out, calls a routine to extend the table first, returning zero if that extension fails.
- `decrement_counter_and_store_pair`: Decrements a short counter through a pointer, then stores two register values into the eight-byte entry at that index of a table referenced from offset 10 of the same block, and returns one.
- `allocate_and_initialize_header_record`: Obtains a 94-byte cleared record, calls a routine on its field at offset 80, and then calls another routine with the field at offset 78 to set it up.
- `return_immediately`: Returns immediately without doing any work.
- `call_library_routine_and_return_zero`: It calls another routine through a table handler and then returns zero to its caller.
- `forward_call_with_offset_78`: Calls another routine with the same first argument and the second argument increased by 78, then returns.
- `find_list_entry_after_equals`: Searches a null-terminated list of pointers, testing each entry with a routine call, and returns the address just after the equals sign found in the matching string.
- `find_string_after_equals_sign`: Scans a null-terminated list of pointers and returns the address just after the first equals sign found at the matched position.
- `scan_pointer_list_for_value`: Walks a null-terminated list of pointers, calling a routine on each entry, and returns the address just after the equals sign when the check passes.
- `match_pipe_separated_item`: Checks whether a string equals one of the items in a string separated by the pipe character, returning one on a match and zero otherwise.
- `find_pipe_item_of_length`: Walks the items of a pipe-separated string and returns one when an item has the expected length and passes a check routine, otherwise zero.
- `find_pipe_item_by_formatted_length`: It repeatedly scans items of a pipe-separated string, comparing each against an expected length.
- `scan_pipe_items_by_length`: It repeatedly scans items of a pipe-separated string, comparing each against an expected length.
- `find_matching_pipe_item`: This routine repeatedly scans the items of a pipe-separated string, comparing each item against an expected length, and returns a nonzero value when a matching item is found.
- `build_copy_name_with_number`: Checks whether a name already begins with the copy prefix text, reading its copy number when one is present, and then writes the next copy label followed by the rest of the name into the structure.
- `parse_decimal_copy_number`: Reads a decimal number from the text at offset 5 of the structure after matching a three-character marker, and calls two routines to finish the update whether or not a number was read.
- `parse_decimal_digits`: Accumulates decimal digits from the text pointer until a non-digit is reached, then checks the four characters after the number when one was read.
- `parse_copy_number_suffix`: Reads the decimal copy number from the text, checks the four characters that follow it, and finishes with two routine calls whichever way the check goes.
- `convert_digit_char_to_value`: Returns the numeric value of a decimal digit character, or minus one when the character is not a digit.
- `return_minus_one_stub`: Returns the constant minus one and does no other work.
- `forward_call_indirect`: This library routine forwards a call to another routine through a computed pointer and then returns without further work.
- `invoke_table_handler`: It looks up a handler pointer from a table and invokes that routine, then returns without touching any registers or memory.
- `call_library_vector_228_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: It calls a routine with two arguments, and when that call returns non-zero it calls the same routine again with four arguments.
- `allocate_block_with_count_ten`: Obtains a 96-byte cleared block, sets its field at offset 14 and the caller's counter word to ten, then calls a routine to initialize the block.
- `return_zero_value`: This library routine returns a zero value to its caller without performing any other work.
- `return_result_if_equal`: Calls another routine with the last three arguments and returns its result if it equals the fourth argument, otherwise returns zero.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `compare_and_clear_result`: It forwards three arguments to another routine and compares the returned value with the fourth argument.
- `empty_routine_2`: This empty library routine performs no work and returns immediately to its caller.
- `divide_unsigned_32_with_remainder`: Divides an unsigned 32-bit value by an unsigned divisor and returns the quotient and remainder together.
- `divide_by_shift_subtract_loop`: Runs a sixteen-step shift-and-subtract loop over a 32-bit value, shifting one bit per step and subtracting the divisor from the high half whenever it is not larger.
- `multiply_32_bit_integers`: Computes the low 32 bits of the product of two 32-bit values from 16-bit partial products, and returns it together with the second input with its 16-bit halves swapped.
- `call_raw_do_fmt_vector`: Calls the function at offset minus 522 from a library base stored in a global variable, with no arguments.
- `store_byte_through_pointer`: Stores the low byte of a data register at the address held in an address register and returns.
- `count_string_length`: Counts the bytes of a null-terminated string by decrementing a 16-bit counter for each non-zero byte, then returns the count.
- `find_character_in_string`: Scans a null-terminated string for the given character and returns a pointer to its first occurrence, or zero if the terminator is reached first.
- `call_and_return_zero`: This routine invokes another routine and then returns a zero value to its caller without performing any other work.
- `copy_c_string`: Copies bytes from a source string to a destination until a zero byte has been copied, advancing both pointers on each step.
- `append_c_string`: Advances to the terminator of the destination string and copies the source string, including its terminator, after it.
- `compare_strings_up_to_length`: Compares two strings for at most the given number of characters, returning zero when they match up to a terminator or the limit and one when they differ.
- `return_one_stub_2`: Returns the constant one and does no other work.
- `call_routine_at_string_end`: Walks a string up to a limit and, when it reaches the terminator, calls another routine before returning.
- `call_indirect_routine`: This library routine invokes another routine via a pointer and then returns, performing no other visible work on registers or memory.
- `copy_string_with_zero_padding`: Copies a string into a fixed-size buffer up to the given length, stopping after the terminator, and fills the rest of the buffer with zero bytes.
- `stub_empty_return`: This is an empty library routine that contains only a return and performs no visible work on its data.
- `setup_pointer_chain`: Stores the address just past the block's first longword into that longword, clears the next longword, and stores the block's own address in the following longword.
- `process_icon_request`: Returns one when given no structure.
- `walk_node_list_calling_handlers`: Walks a linked list of twelve-byte nodes, passing each node to one routine and its data pointer with a size derived from a byte field to another.
- `walk_node_chain_calling_pair`: Repeatedly calls two routines for the current node and follows the link at offset 12 to the next node.
- `empty_return_stub`: This routine contains no instructions; it is an empty stub that simply returns without performing any operation.
- `allocate_entry_and_store_result`: Returns zero when no entry is requested.
- `return_nonzero_value`: This library routine returns a nonzero value to its caller without performing any other work.
- `build_node_chain_with_blocks`: Builds a chain of 16-byte nodes for the requested count, linking each new node at offset 12 of the previous one and obtaining a data block at offset 8 whose size comes from a byte field.
- `link_node_chain_blocks`: Repeats an allocation step while a chain continues, linking each new node at offset 12 of the previous one and recording a data block at offset 8 of each node.
- `fill_node_chain_blocks`: Fills in the data blocks of an existing node chain, linking each node at offset 12 of the previous one and reading a block for each node at offset 8 until the chain ends.
- `return_negative_one`: Returns the value -1 (all bits set) to the caller.

### translator.library

- `make_and_register_library`: Creates a library structure by calling the exec MakeLibrary function, stores the library base and the value passed in an address register into its fields at offsets 34 and 38, and then registers it with AddLibrary.
- `increment_counter_clear_flag_bit5`: Increments a 16-bit field at offset 32 of a library structure and clears bit 5 of the byte at offset 14.
- `decrement_counter_call_if_flagged`: Decrements the 16-bit field at offset 32 and, when it reaches zero with bit 5 of the byte at offset 14 set, calls another routine and returns its result.
- `unlink_node_or_mark_flag`: If the word at offset 8 of the structure is zero, it unlinks the structure from its neighbours and calls a library function at offset minus 210 from the library base, returning the longword at offset 38.
- `unlink_node_and_call_vector`: Unlinks the structure addressed by an address register from its neighbouring list entries, calls the library function at offset minus 210 from the library base, and returns the longword at offset 38.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `parse_text_with_hash_markers`: Fills a 26-byte stack buffer with spaces, then scans the input text for hash markers and space characters, calling other routines to process each marker and the text between them.
- `return_data_register_word`: Returns the low 16-bit word of a data register as an integer and does no other work.
- `terminate_string_and_return_word`: Calls another routine, writes a zero byte one position before the address it leaves in an address register to terminate a string, and returns the 16-bit value left in a data register.
- `scan_text_until_hash_marker`: Walks table entries and the text they point to, comparing characters with the hash marker and the space character, and calls routines to advance through the text.
- `scan_text_field_separators`: Walks a text field one character at a time, stopping at a hash marker or space separator, and calls routines to advance through the table entries.
- `scan_escape_sequences_in_text`: Scans text for a backslash escape and a backtick terminator, calling other routines for each piece it finds, and returns when the end is reached.
- `scan_text_for_escape_markers`: Reads text one character at a time until a backslash or backtick is found, calling another routine at each match, and returns once its end test passes.
- `stub_immediate_return`: The decompiled body contains only a return, so this library routine does no visible work; its instruction count suggests it may be padding rather than logic.
- `match_bracket_tag_prefix`: Skips forward to an opening bracket in the text, then compares the characters that follow against a second string until a closing bracket is reached or a character differs.
- `scan_backward_with_char_table`: Scans a string backward one character at a time, looking up each character in a character-class table and calling another routine for characters of the marked class.
- `do_nothing_returned`: This routine contains no instructions and performs no operations; it simply returns immediately.
- `match_backward_text_characters`: Walks a string backward, looking up each character in a character-class table, and compares unmarked characters against a second string walking backward.
- `match_text_until_equals_sign`: Reads characters from the text until an equals sign, calling another routine for each character of the marked class and comparing the others against a second string.
- `compare_text_until_equals_sign`: Reads characters until an equals sign.
- `scan_and_copy_text_run`: Measures a run of text up to a backslash or backtick, then either copies it to the output buffer or, if it would pass a width limit, backs up to a break character and ends the line there.
- `copy_text_run_with_wrapping`: Copies a counted run of text to the output buffer, or if the run would pass a width limit, backs up to a break character and ends the line there.
- `copy_fixed_length_text_run`: Copies a fixed number of characters from the source text to the output buffer, then sets bit 0 of a status byte, clearing it again when the next source character is a backtick.
- `terminate_text_at_break_character`: Walks backward from the current output position until a character of the break class is found, then writes a zero byte there to end the line.
- `dispatch_special_character_handler`: Searches a table of twelve special characters for the byte in a data register and calls the handler at the matching position of a table of four-byte entries.
- `library_empty_return`: It performs no operations and simply returns without touching any data or registers.
- `stub_empty_return_slot`: It performs no operations and simply returns, matching the decompiled output which contains only a return statement.
- `invoke_table_handler_and_return`: It invokes a table handler and then returns to its caller without modifying any registers or memory.
- `call_while_flag_bit5_set`: Calls another routine once, then repeats that call for as long as bit 5 of a data register remains set.
- `library_empty_return_2`: It performs no visible actions and simply returns to its caller without touching any data or registers.
- `call_and_check_flag_char`: Calls another routine and, when bit 5 of a data register is set, calls it once more and compares the character it returns with the letters I and E. Both paths return without passing a result back.
- `empty_return_routine`: This routine contains no instructions and performs no operations.
- `match_suffix_letter_sequence`: Reads a sequence of letters through a routine and tests them against fixed letter pairs, such as E followed by S, D or R, and N followed by G. A flag bit selects between the two branches, and each accepted letter leads to another routine call before the routine returns.
- `match_ng_letter_sequence`: Reads letters through a routine and accepts them only when they are N followed by G, then optionally checks for an S when a flag bit is set.
- `empty_return_routine_2`: This routine contains no instructions and does nothing.
- `empty_library_return_2`: This is an empty library routine that performs no operations and simply returns without touching any data or registers.
- `return_without_changes`: It executes three instructions and then returns without modifying any data or registers.
- `empty_return_routine_3`: It executes a single RTS instruction and returns to its caller without modifying any registers or memory.
- `empty_return_routine_4`: This routine performs no operations and simply returns without touching any data or registers.
- `poll_status_bit_until_clear`: It repeatedly calls a library routine while bit 5 of a status value stays set, then returns once that bit becomes clear.
- `return_without_effects`: It executes a short three-instruction sequence and then returns without altering any registers or data.
- `call_while_flag_bit1_set`: Calls another routine repeatedly for as long as bit 1 of a data register remains set.
- `call_pair_then_return`: Calls two routines in sequence and then returns.
- `do_nothing_return`: This routine performs no operations whatsoever and simply returns to its caller without modifying any data or registers.
- `normalize_text_into_buffer`: Copies text into a fixed buffer one character at a time, replacing control characters and closing brackets with spaces and converting lowercase letters to uppercase.
- `write_space_before_pointer`: Writes a space character two bytes before the address held in an address register and returns.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 253 into d1, then returns.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies 32 to (a2)+, then copies 32 to (a2)+, then returns.
- `insert_marker_after_vowel_pair`: Checks the current output position against a table of fourteen two-letter vowel codes held in the data.
- `shift_text_and_insert_four`: Moves the output text one place along from the current position to open a gap, writes a digit four into the gap, and stores the new position in the state.

### info.library

- `stub_bad_instruction_data_5`: The listing contains only an instruction the decompiler cannot translate, so no behaviour is recovered for this routine.
- `store_library_state`: Stores the caller's frame register and a data register into two fixed global slots, writes the address register value at offset 34 of the structure in the data register, and copies that value into a third global slot.
- `invoke_helper_twice`: It invokes a helper routine with two arguments, and when that helper returns a non-zero value it invokes the same helper again with four arguments.
- `decrement_library_refcount_3`: It decrements a 16-bit counter and, when that counter reaches zero with bit 3 of a byte set, it calls another routine to release the structure.
- `free_structure_when_unused`: Library code that checks a reference counter in a structure and, when it reaches zero, calls another routine to release the memory block that precedes the structure.
- `empty_library_return`: Empty library code that returns immediately without performing any operation.
- `reset_library_state_recursive`: Library code that clears shared globals, stores an argument, requests a resource tagged 246, and calls itself repeatedly before finally invoking a routine that resets the library state.
- `clear_flag_and_increment_counter`: Clears bit 3 of the byte at offset 14 of the structure, increments the 16-bit field at offset 32, and returns the structure pointer.
- `free_structure_region_when_unused`: When the 16-bit counter at offset 32 is zero, it calls a routine on the structure, releases the memory block that starts before the structure, sized by two 16-bit fields at offsets 16 and 18, and calls a routine again before returning a value saved in a global.
- `release_structure_block`: Releases the memory block that starts before the structure, sized by two 16-bit fields at offsets 16 and 18, between two calls to other routines, then returns a value saved in a global.
- `decrement_library_refcount_2`: Decrements the 16-bit counter at offset 32 of the structure and, when it reaches zero with bit 3 of the byte at offset 14 set, calls another routine and returns its result.
- `return_zero_value_2`: Library code that returns a constant zero value to its caller.
- `process_nonzero_pointer_slots`: Passes each non-zero pointer among its arguments and several global slots to another routine, passing a flag of one for the second argument when it is set.
- `allocate_info_block_and_layout`: Allocates a 944-byte cleared block and stores pointers to fixed offsets inside it in several globals.
- `initialize_info_block_layout`: Sets pointers to fixed offsets inside the structure addressed by an address register and copies template values from a global area into each sub-block.
- `open_four_library_bases`: Calls a routine with the shared library name buffer and a zero argument four times, storing each result in a global.
- `open_three_library_bases`: Calls a routine through a pointer in an address register with the shared library name buffer and a zero argument three times, storing each result in a global.
- `open_two_library_bases`: Calls a routine with the shared library name buffer and a zero argument twice, storing each result in a global.
- `open_one_library_base`: Calls a routine with the shared library name buffer and a zero argument once, storing the result in a global.
- `return_one_value`: Library code that returns a constant one value to its caller.
- `lookup_info_field_value`: Returns the value at offset 164 of the global structure when it is set.
- `lookup_info_field_value_direct`: Works on the structure addressed by an address register and returns zero when its type byte at offset 61 is seven.
- `lookup_info_field_untyped`: Returns the value at offset 164 when it is set.
- `lookup_info_field_from_register`: Returns the longword at offset 442 of the structure linked at offset 76, using the structure itself when the type byte at offset 61 is one, or the structure passed in an address register otherwise.
- `read_nested_field_or_zero`: Returns zero when the pointer passed in the address register is null.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a2, then returns.
- `resolve_disk_info_path`: Builds a path in a stack buffer from a base string and the text disk.info that the library references, then runs a lookup on it.
- `check_status_and_set_flag`: Runs a library call on a stack buffer using a field from a structure, then checks its return value and a flag byte.
- `allocate_and_fill_record`: Allocates a cleared block through a library call and a second block after it, then checks them against a shared state structure.
- `populate_record_from_allocation`: Stores a freshly obtained handle, then if the shared state and the optional argument pass their checks it copies a block into the record, sizes it to at least 256 and allocates a buffer.
- `populate_record_fields`: Checks a shared-state flag and an optional argument, then copies a block into the record, sizes it to at least 256 and allocates a buffer.
- `populate_record_if_valid`: Runs only when the optional argument is clear or a library check passes, then copies a block into the record, sizes it to at least 256, allocates a buffer and fills its fields.
- `fill_record_and_report_success`: Copies a block into the record, sizes it to at least 256 and allocates a buffer.
- `copy_library_strings_and_data`: Copies a string from the shared state into the record through a library routine, then copies two longword fields from the state and the entry into the record.
- `setup_library_state_error_codes`: Clears a set of shared globals and stores the argument, then requests a resource tagged 246 and runs further checks and library calls to set up the working buffers.
- `setup_library_buffers_via_callback`: Requests a resource tagged 246, then obtains a second buffer and a shared global.
- `finish_library_buffer_setup`: Checks a stack record, then obtains a shared global and links it to a buffer using library calls.
- `link_library_buffer_or_fail`: Obtains a handle and, if it is valid, allocates a shared buffer and links it to the handle through library calls.
- `attach_shared_buffer_callback`: Obtains a shared buffer and copies a field from the record into it.
- `finish_library_link_and_notify`: Copies a field from the record into a shared buffer and runs a library call.
- `notify_library_link_result`: Obtains a handle and checks it.
- `complete_library_link_success`: When a shared flag is clear and an argument is set, it runs a library call on the handle.
- `call_library_vector_132`: Loads a library base from a global variable and calls the library function at offset minus 132 of that base.
- `call_library_vector_138`: Loads a library base from a global variable and calls the library function at offset minus 138 of that base.
- `call_library_vector_198`: Loads a library base from a global variable and calls the library function at offset minus 198 of that base.
- `call_library_vector_210`: Loads a library base from a global variable and calls the library function at offset minus 210 of that base.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_246`: Loads a library base from a global variable and calls the library function at offset minus 246 of that base.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_vector_294`: Loads a library base from a global variable and calls the library function at offset minus 294 of that base.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_372`: Loads a library base from a global variable and calls the library function at offset minus 372 of that base.
- `call_library_vector_378`: Loads a library base from a global variable and calls the library function at offset minus 378 of that base.
- `call_library_vector_414`: Loads a library base from a global variable and calls the library function at offset minus 414 of that base.
- `call_library_vector_552`: Loads a library base from a global variable and calls the library function at offset minus 552 of that base.
- `call_library_vector_42`: Loads a library base from a global variable and calls the library function at offset minus 42 of that base.
- `call_library_vector_72_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_96`: Loads a library base from a global variable and calls the library function at offset minus 96 of that base.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_228`: Loads a library base from a global variable and calls the library function at offset minus 228 of that base.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_library_vector_354_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `invoke_callback_from_table`: Takes a routine from a global table of function pointers at a fixed offset and calls it.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `call_library_vector_462`: Loads a library base from a global variable and calls the library function at offset minus 462 of that base.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72`: Loads a library base from a global variable and calls the library function at offset minus 72 of that base.
- `call_library_vector_78`: Loads a library base from a global variable and calls the library function at offset minus 78 of that base.
- `call_indirect_through_table`: Reads a routine pointer from a global table at a fixed negative offset and calls it indirectly.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_indirect_table_entry`: Calls a routine through a pointer read from a global table at a fixed offset, then returns.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_354`: Loads a library base from a global variable and calls the library function at offset minus 354 of that base.
- `build_parameter_block_and_call`: Fills a stack-allocated block with numeric values and a pointer to a global, then passes it to a library call.
- `dispatch_state_code_actions`: Reads a state code from a nested structure and branches on it, performing the matching library call and updating a stored flag in some cases.
- `return_success_stub`: Returns one unconditionally with no other work.
- `toggle_status_bit_from_code`: Maps a code read from a structure to one bit value, flips that bit in a stored state word and saves it back.
- `store_register_to_global`: Stores the value passed in a data register into a fixed field of the shared state, then calls a routine with a pointer read from the argument structure.
- `format_info_file_and_check`: Builds a file name in a stack buffer from the referenced pattern with the suffix .info, then runs several library calls on it and on the shared state.
- `validate_disk_entry_name`: Builds a name in a stack buffer from the referenced text Disk or a stored string, then tests it with a library call.
- `copy_name_and_verify_attributes`: Copies a string from the shared state into a stack buffer and tests it with a library call.
- `verify_four_handles_then_commit`: Runs four library calls with the same argument, each checked for a failure value of minus one.
- `verify_three_indirect_calls`: Calls a routine held in a register three times, returning zero if any call returns minus one.
- `verify_two_indirect_calls`: Calls a routine held in a register twice, returning zero if either call returns minus one.
- `run_indirect_call_if_ok`: Calls a routine held in a register and, if its result is not minus one, runs one follow-up routine.
- `call_other_and_return_one`: Calls one routine and then returns one with no other checks.
- `initialize_info_library_tables`: Registers a long sequence of entries by calling one routine with fixed numeric ranges and a pointer from the argument, and passes a global as the last argument for three entries.
- `register_seven_fixed_ranges`: Calls one routine seven times with the same first argument and constant range pairs.
- `register_seven_index_ranges`: Calls one routine seven times with the same first argument and constant index pairs, several of which share bounds.
- `register_nine_index_ranges`: Calls one routine nine times with the same first argument and constant index pairs.
- `set_nine_index_ranges`: Calls one routine nine times, passing the same first argument with six constant values each time.
- `populate_entries_by_state_code`: Chooses one of two numeric values from a state code, then adds either a fixed entry set or a longer one depending on whether a list is empty.
- `finish_list_update_conditionally`: Runs one of two routine calls depending on whether a list is empty, then runs one more call with a global only when the flag argument is nonzero.
- `run_update_steps_with_option`: Calls a library function, then calls another routine with a field from a structure, and calls a third time with a global when an option register is set.
- `allocate_linked_entry_list`: Allocates a cleared entry and links it to itself as the head of a list.
- `link_entries_from_table`: Points the list head to itself and clears a counter, then for each pointer in a shared table it allocates an entry, copies the table value into it and links it in.
- `link_entries_until_null_pointer`: Walks a null-terminated table, allocating an entry for each pointer and linking it into the list with library calls.
- `link_table_entries_to_end`: Links each table entry in turn, allocating a fresh entry per pointer and copying the table value into it.
- `advance_list_cursor_or_finish`: If the current list position is at an end point, it runs closing calls using fields of the argument structure.
- `update_pointer_from_structure`: Runs a routine on the current entry and a fixed global, then copies the next link of the entry into the cursor and runs one more call.
- `advance_cursor_or_restart`: If the cursor is at an end point, it runs closing calls on the argument structure.
- `update_pointer_dereference`: Runs a routine on the current entry and a fixed global, then replaces the cursor with the value its first link points to and runs two more calls.
- `insert_new_entry_at_cursor`: Sets a flag and, when the list has entries, runs two calls on the current entry.
- `move_cursor_to_neighbour_entry`: Sets a flag, then either runs the finishing calls when the list is at its end or processes the current entry.
- `reset_cursor_to_first_entry`: Records the first link of the cursor entry and runs a call on the cursor.
- `run_three_step_callbacks`: Calls one routine with no argument, then calls two more with fields of the argument structure.
- `build_pointer_array_from_list`: When a flag is set and the list is not empty, it counts the entries, allocates an array of pointers sized to match and stores it in the shared state.
- `populate_pointer_array_for_list`: Allocates a pointer array sized to the list length and stores it in the shared state, or clears the array field when the list is empty.
- `fill_pointer_array_from_entries`: Calls a routine on the cursor, counts the list entries and allocates an array sized to the count, storing it in the shared state.
- `allocate_pointer_array_for_entries`: Counts entries from a starting count and allocates an array sized to that count, storing it in the shared state.
- `link_entries_into_pointer_array`: Calls a routine with the pointer array from the shared state, then walks the list allocating a block per entry and linking it into the array.
- `link_remaining_entries_in_array`: Continues from the current list entry, allocating a block for each remaining entry and linking it into the pointer array.
- `continue_linking_array_entries`: Links the remaining entries into the array, allocating a block for each one and stopping at the end of the list.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_90`: Loads a library base from a global variable and calls the library function at offset minus 90 of that base.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `call_library_vector_114`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.
- `call_library_vector_180`: Library stub: loads a library base from a global variable and calls the library function at offset minus 180 of that base, returning its result to the caller.
- `call_library_vector_186`: Library stub: loads a library base from a global variable and calls the library function at offset minus 186 of that base, returning its result to the caller.
- `dispatch_entries_by_state_byte`: Switches on a state byte from the shared state and runs a different set of routine calls for each case.
- `return_empty_stub`: Returns immediately with no work.
- `allocate_buffers_setup_by_state`: Allocates four buffers and, if all succeed, allocates four more.
- `store_handles_run_by_state`: Calls a routine held in a register four times, keeping three results in globals and the fourth through a pointer.
- `run_calls_by_state_byte`: Runs a different number of follow-up routine calls for each state byte value from one to five.
- `copy_register_value`: It executes three instructions that load a value from a register and store it through a pointer, then returns.
- `clear_field_then_query_status`: Clears a stored value if a field is zero, or passes the field to a library call otherwise.
- `pass_field_then_return_result`: Passes a stored field to a library call together with a global, then runs another call and returns zero when its result is minus one, otherwise that result.
- `return_nothing_stub`: Returns without doing anything.
- `set_offsets_verify_four_calls`: Writes four fixed 16-bit values into structures at a fixed offset when the state byte is not four, then runs four library calls checked against minus one.
- `verify_three_register_calls`: Calls a routine held in a register three times, returning zero if any call returns minus one.
- `verify_two_register_calls`: Calls a routine held in a register twice, returning zero if either call returns minus one.
- `check_register_call_succeeds`: Calls a routine held in a register and returns true when the result is not minus one.
- `return_one_stub`: Returns one with no other work.
- `poll_bit_signal_loop`: Repeatedly calls a library function to fetch a value from a structure.
- `return_failure_stub`: Returns zero with no other work.
- `dispatch_on_size_code`: Reads a numeric code from a structure and branches on it.
- `call_routine_and_return`: Calls one routine with no argument and returns nothing.
- `call_routine_return_zero`: Calls one routine with no argument and returns zero.
- `execute_two_calls`: Calls two routines in sequence and returns one.
- `run_callback_sequence_for_state`: Runs the same routine three times with the argument, adds a fourth call when the state byte is not one, then calls routines with a field from the argument and with a flag value of one.
- `report_protection_flag_entries`: Derives four flag bits from a stored status word and calls a routine once per bit with its index and a global pointer.
- `emit_labelled_flag_entries`: Builds a short key in a stack buffer from a constant string and a selector, then chooses one of two pairs of text arguments depending on a flag.
- `format_string_arguments`: Packs a stack argument list from the referenced pattern texts and a value taken from the caller's frame, then runs two further routines.
- `measure_string_length`: Counts the bytes of a zero-terminated string from its start and returns the count without the terminator.
- `calculate_string_length`: Advances an index from a starting register value until a zero byte is found and returns that index as the length.
- `copy_cstring_to_buffer`: Copies a zero-terminated byte string from a source pointer to a destination pointer, advancing both and writing each byte including the terminator.
- `copy_fixed_length_bytes`: Copies a requested number of bytes from a source buffer to a destination, advancing both pointers on each byte.
- `copy_memory_block`: Copies a fixed number of bytes from a source pointer to a destination pointer, one byte at a time.
- `skip_matching_prefix`: Advances through two strings while their characters match and neither has ended, then returns whether the characters at the stopping point are equal.
- `compare_strings`: Compares two zero-terminated strings from the input registers, advancing both until a mismatch or an end is reached.
- `count_bytes_including_terminator`: Counts the bytes of a zero-terminated string and returns the count including the terminating zero byte.
- `execute_library_operations`: Calls a routine to obtain a value, then passes a field of the structure to three library calls with arguments adjusted by minus ten and the value obtained earlier.
- `call_string_formatter`: Builds a formatted string in a 12-byte local buffer from a global pattern and one argument, then passes that buffer with the other arguments to a second routine.
- `copy_length_prefixed_string`: Reads a count from the first byte of the source and copies that many following bytes to the destination, then appends a zero terminator.
- `copy_string_data`: Copies bytes from a source location to a destination location for a count held in registers, then writes a zero terminator at the end.
- `initialize_two_records_and_call`: Fills two record structures with fixed field values, including the argument and a pointer from the caller, then runs two library calls and a third call that returns a status.
- `execute_library_function`: Passes a pointer stored in a structure at a fixed offset to a library call, then returns one.
- `execute_library_calls`: Calls one routine with a field of the structure, then calls it again with zero, and a third time with the remaining arguments adjusted by minus ten.
- `call_library_vector_522`: Loads a library base from a global variable and calls the library function at offset minus 522 of that base.
- `store_byte_at_pointer`: Stores the low byte of the data register into the memory location the address register points to.
- `call_library_vector_36`: Loads a library base from a global variable and calls the library function at offset minus 36 of that base.
- `call_indirect_via_table`: Library code that calls another routine through an indirect pointer taken from a global table, performs no other operation, and then returns.

### version.library

- `return_zero_entry`: Returns zero immediately with no other work.
- `store_two_values_at_offsets`: Writes two 32-bit values held in registers into a structure, one at offset 34 and the other at offset 38.
- `increment_counter_and_clear_flag`: Increments a 16-bit counter stored 32 bytes into a structure and clears the eighth bit of a flag byte 42 bytes in.
- `decrement_counter_and_call`: Decrements a 16-bit counter stored 32 bytes into a structure.
- `finish_when_counter_zero`: If the counter at offset 32 is zero, it reads the value at offset 38 and calls two library functions through the base stored at offset 34, then returns that value.
- `get_library_value`: Reads a longword at offset 38 of the structure, calls two library functions at fixed negative offsets on the base in a register, and returns the value read.
- `return_zero_stub`: Returns zero with no other work.

### Populous (1989)(Electronic Arts)[cr QTX][a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
