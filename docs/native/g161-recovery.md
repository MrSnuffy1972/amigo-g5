# Game 161 (AmazingSpiderMan v1.1 1006): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00161). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1187488 top-level bytes in 29 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 612 of 612 named (0 author symbols), 612 of 612 described (206 by rule, 4 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1986 of 1986 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9581 of 9711 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 289383 |
| code | 33850 |
| data | 864151 |
| padding | 104 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 1930 | 122 |
| Boot | 1 | 43 | 1 |
| Comic | 5 | 848 | 58 |
| Exit | 2 | 117 | 2 |
| Loader | 1 | 68 | 1 |
| ReadMe | 1 | 47 | 1 |
| Spidey | 8 | 6062 | 224 |
| Music | 2 | 396 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 193 | 188 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 7 |

## Cases handled

- AmazingSpiderMan_v1.1_1006/AmazingSpiderMan_v1.1_1006.hdf: hard-disk image (AmigaDOS OFS, 1280 blocks): its filesystem parses and all 25 files in it are byte-identical to the extracted tree AmazingSpiderMan_v1.1_1006_hdf, which is recovered file by file; no block outside the filesystem holds data
- 5 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 131 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `return_minus_one_constant`: Loads the immediate value minus one into d0 and returns immediately to the caller, providing a constant return value.
- `branch_to_loader_entry`: Executes a single unconditional branch instruction that transfers control to another location in the loader code.
- `spidey_loader_setup_and_exit`: Calls library functions through a2 to set display and colors, loads the spidey text, runs setup routines, stores a result, then jumps to the library exit vector.
- `patch_allocated_memory_with_rts`: Walks a small pointer chain five times, doubles the resulting address, adds 32512, calls an allocation routine with size 8 and a data pointer, then writes an rts opcode into the returned memory.
- `open_intuition_and_run_main_loop`: Opens intuition.library via exec, stores the base pointer, calls a library function with a data address, saves the result, then jumps into the library's main loop entry.
- `zero_filled_data_word`: Contains a single zero word that decodes as ori.b with immediate zero; this is zero-filled data referenced by address, not executable code.
- `call_2ec_then_branch_21a`: Calls the subroutine at offset 2ec, then branches unconditionally to offset 21a, continuing execution there.
- `allocate_task_stack_and_setup_task`: Allocates memory, fills task fields at offsets 58, 62, 54, 172 and 176 of the task structure, calls AddTask via a6, and restores registers before returning.
- `find_task_and_build_return_frame`: Calls FindTask with a null name, computes the task's stack size from offsets 62 and 58, pushes a return address and arguments, clears registers a1-a4 and a6 plus d1 and d3-d7, then returns.
- `push_pea_and_pc_relative_long`: Pushes the immediate word value minus one onto the stack, then pushes the longword stored at the pc-relative offset, adjusts the stack pointer by four and returns.
- `discard_four_bytes_and_return`: Adds four to the stack pointer a7, discarding one longword, then returns immediately to the caller.
- `strlen_via_library_call`: Saves d1 and a0-a1, calls the library routine at offset -534 from a6, then counts the bytes of the returned string until a null terminator, returning the length in d0.
- `alloc_and_init_dos_process_struct`: Calls the allocator with size 68, copies 68 bytes of a template into the new block, stores d4, d3, d1 and d2 into fields at offsets 132, 128, 10, 8 and 9, and returns the block pointer in d0.
- `create_kickdos_process_and_wait`: Calls setup routines, creates a process via exec, stores d3 at offset 60 of the new process, registers a callback, then calls Wait with zero on the exec base.
- `measure_and_store_clock_ticks`: Saves all registers, calls the timing library function through a6, shifts the returned d0 right by two, stores it at offset 172 of the template area, restores registers and returns.
- `build_trampoline_and_fill_eeee`: Allocates memory via a helper, writes jsr instructions to a target routine followed by a stored pointer, then fills the remaining space with eeee words and returns the block.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `zero_filled_data_area`: Contains a single zero word that decodes as ori.b #0,d0; this is a zero-filled data area referenced by address, not executable code.
- `call_library_offset_20`: Saves d0, d1, a0 and a1 on the stack, loads a library base from pc-relative memory, calls the routine at offset 32 of it, then restores registers and returns.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `append_d0_plus_four_to_linked_list`: Saves d2, adds four to d0, calls a routine to allocate or find a node, stores the adjusted d0 into the returned node, advances d0 by four, restores d2 and returns.
- `predecrement_a1_load_and_branch`: Loads a longword from the address just below a1 into d0 and branches to a shared routine; the bytes are reached only as a gap, not normal code.
- `store_a0_to_global_197e`: Saves a1 on the stack, loads the address of the global at pc-relative offset 197e into a1, stores a0 there, restores a1 and returns.
- `scale_d0_by_four_into_a1`: Calls a subroutine, shifts the returned d0 left by two bits, copies it to a1 and adds four to a1 before returning.
- `open_library_and_copy_name_string`: Saves registers, calls a setup routine, opens a library via its base, and if successful copies a null-terminated name string into a buffer while counting its length.
- `install_three_vectors_and_set_reloc_count`: Saves three old exception vector entries into globals, replaces each vector with a new routine address, then stores a shifted pointer value into a global field and returns.
- `load_pc_relative_longword_to_d0`: Loads the longword stored at the program-counter-relative offset 6526 into d0 and returns it to the caller.
- `save_registers_call_sub_0668_return_a6`: Pushes d0-d7 and a0-a5 onto the stack, calls the routine at 1640, moves its d0 result into a6, restores the saved registers and returns.
- `compare_bytes_until_mismatch_or_count`: Compares d0 bytes of the block at a0 against the block at a2, advancing a0 until a mismatch or a0 exceeds a1, then returns a0 or zero.
- `push_a6_offset_and_error_vector`: Exchanges d0 with a6, subtracts d0 from the stack pointer, subtracts six more from the stack, pushes the longword at offset minus four from a6, pushes address ten, then pushes the longword from the program-counter-relative table and returns.
- `check_dos_magic_and_call_loader`: Saves d2 and d3, copies a1 to d2 and d0 to d3, compares the longword at a1 with the DOS magic value, and if equal saves all registers, calls the loader, restores them and branches; otherwise pushes an error string and returns.
- `test_d0_and_push_error_or_restore`: Tests d0 and branches if nonzero; otherwise pushes address one, d2, address ten and the program-counter-relative longword, adjusts the stack and returns; the tail restores d2 and d3, tests d0 and returns.
- `check_dos_magic_and_call`: Compares the longword at address in a1 with the bytes for '.sod', and if equal pushes registers d1-d7/a0-a6, calls a subroutine, restores them and returns; otherwise returns immediately.
- `compare_a1_to_stored_pointer`: Loads a longword from a program-relative location into d0, compares it with a1, and if equal sets d0 to zero before returning.
- `jump_through_stored_vector`: Pushes a longword loaded from a program-relative location onto the stack and returns, transferring control to that stored address.
- `build_dos_vector_stub_table`: Allocates a 1050-byte block via dos.library, stores it in a global, then writes jump instructions and handler addresses into fixed offsets of that block and links several small data tables before calling a setup routine.
- `convert_spaces_and_run_dos_command`: Copies a string from d1 into a buffer replacing spaces with zero bytes, appends a newline, calls the dos.library function at offset -150 with the buffer, and if it returns nonzero indexes a table by the result.
- `call_a1_and_return_minus_one`: Calls the subroutine whose address is in a1, then sets d0 and d1 to minus one and returns to the caller.
- `return_zero_in_d0_and_d1`: Loads zero into d0 with moveq, copies d0 into d1, then returns to the caller.
- `zero_filled_data_area_after_return_stub`: Twelve zero words decoded as ori.b instructions; this is a zero-filled data area referenced by address, not executable code.
- `zero_filled_data_table_sixteen_words`: Sixteen zero words decoded as ori.b instructions; this is a data table referenced by address, not working code.
- `store_d1_to_shared_slot_and_shift`: Loads the address of a shared variable into a0, stores d1 there, then copies d0 into d1 before returning.
- `load_shared_slot_into_d0_and_d1`: Reads the shared variable at the fixed address into d0, copies d0 into d1, and returns to the caller.
- `zero_filled_data_area_2`: This single zero word decodes as an ori.b immediate instruction but is a zero-filled data area referenced by address, not executable code.
- `call_secondary_allocator_via_library`: Moves d1 into a0, calls a subroutine, then jumps through offset 88 of the library base at the pc-relative pointer, calls another subroutine and returns its result in d1.
- `parse_and_build_linked_record_list`: Saves registers, allocates a 28-byte stack frame, reads tagged records through a stream reader, allocates nodes, links them into a list and dispatches on record type codes.
- `load_and_apply_relocation_blocks`: Reads relocation records from the file handle, validates block headers, allocates memory, patches longword offsets, and frees the temporary buffer before returning.
- `select_bit30_bit31_flag_value`: Tests bits 30 and 31 of d0 and returns 2, 4, or a fixed address in d1 depending on which bit is set, defaulting to bit 16 set.
- `walk_linked_list_via_d1_index`: Saves a2, indexes a table by d1 shifted left twice, follows the pointer chain calling a subroutine until a null pointer, then returns the last value in d0 and d1.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `allocate_progdir_named_structure`: Loads the PROGDIR string address into a1, overwrites it with d1, and branches to a shared allocation routine that builds a named structure from the string.
- `create_message_port_node`: Allocates a memory node sized from the string in d1, stores the size, d2 and cleared fields, copies the string into offset 52, then creates a message port and returns the node address shifted right by two.
- `return_magic_marker_in_d0_and_d1`: Loads a fixed marker value into d0, copies it to d1, and returns to the caller.
- `read_port_message_into_buffer`: Scales the port index in d1 by four, clamps the requested length d3 to the remaining message bytes, calls the port read function with the buffer and length, advances the read offset and returns the count in d0 and d1.
- `add_offset_and_store_result_in_object`: Checks d1 against a sentinel, then adds d3 to the longword at offset 48 of the object indexed by d1 and stores the result of a call at offset 20, returning d3 in d0 and d1.
- `return_d3_in_d0_and_d1`: Copies d3 into d0, then copies d0 into d1, so both registers return the value of d3 to the caller.
- `dispatch_object_method_by_index`: Validates d1 against a sentinel and zero, indexes an object table by d1, and calls a method at offset 32 with the object's field at offset 24 as argument.
- `dispatch_sound_by_index_or_return_zero`: If d1 is zero returns zero in d0 and d1; otherwise scales d1 by four, loads the longword at that offset into d0, calls the sound routine, then returns one in d0 and d1.
- `adjust_object_offset_by_mode`: Scales d1 by four into a1, then depending on d3 being minus one, one or zero sets or adds d2 to the longword at offset 48, clamped against offset 20, returning the old value in d0.
- `init_dos_structure_defaults`: Saves a1, points a1 at d2, clears the first two longwords, stores 82, 10000 and the DOS signature, clears three more longwords, restores a1 and returns minus one in d0 and d1.
- `repeat_call_d1_times`: Tests d1; if nonzero, calls the routine at the fixed target d1 times, decrementing d1 after each call, then returns.
- `zero_three_longs_at_d1`: Copies d1 into a0, writes zero to three consecutive longwords at a0, a0+4 and a0+8, returns d1 in d0.
- `allocate_and_init_object_from_strings`: Saves registers, measures the d1 string, allocates a block, stores its size, copies two strings into the new object, opens a library resource and returns the object pointer shifted right by two in d0 and d1.
- `return_zero_for_failed_alloc`: Sets d0 to zero and branches to the shared exit, returning a null result to the caller that requested an allocation.
- `duplicate_object_with_copied_name`: Allocates a new object, copies the name string from a3 into its offset 52 field, stores d2 at offset 44, sets word at offset 20 to minus two, and links it before returning.
- `duplicate_object_with_alternate_name`: Frees the old object, allocates a replacement, copies a name from d5 into offset 52, stores d2 at offset 44, sets word at offset 20 to minus two, and links it.
- `read_byte_via_io_with_restore`: Calls the byte-read helper with a masked selector, saves the result in d5, reads a longword through a3 and calls another helper, then restores d5 into d0 and branches onward.
- `store_magic_and_shifted_pc_address`: Writes a magic longword to offset twelve of a3, saves d0 and a0, loads a program-counter-relative address, shifts it right by two and stores it at offset sixteen of a3.
- `load_global_plus_offset`: Loads a longword from a program-counter-relative global into d0, adds a fixed offset to it and returns the resulting value in d0.
- `copy_block_after_alloc`: Tests d1, allocates a block of d1*4 bytes via a helper, copies d1*4 bytes from the pointed-to memory into it, then returns the byte count shifted right by two in d0 and d1.
- `return_zero_in_d0_and_d1_2`: Loads zero into d0, copies d0 into d1, then returns to the caller.
- `allocate_and_mark_error_block`: Allocates a 62-byte block via a helper, stores the value 62 at its start and minus two at offset twenty of a1, then branches back to the shared exit.
- `check_string_at_indexed_pointer`: Scales d1 and d2 by four, loads the pointer at offset 48 of the indexed structure into a2, tests the byte it points to, calls the string routine if non-zero, and returns -1 in d0 and d1.
- `set_d1_to_negative_eighteen_and_call`: Loads 23 into d1, complements its low byte, calls the routine at the given offset, then sets d0 to zero and branches back to the caller's continuation.
- `copy_string_and_set_sprite_state`: Copies the string at offset 48 of a3 into d2's buffer, increments the counter at offset 20 of a3, calls the library routine at offset 36, and sets state fields in d2 based on the result.
- `setup_io_request_and_copy_name`: Stores d0 and fixed command words into the a2 request block, computes a length from d0, then copies a null-terminated string from the pointer at 48(a3) into 8(a2) and advances that pointer.
- `return_minus_one_constant_2`: Loads minus one into d0 and returns immediately, providing a constant negative result to the caller.
- `path_join_data_table`: The bytes are a data table of zero words decoded as no-op instructions, not executable code; callers reference it by address for path joining.
- `append_name_to_path_buffer`: Copies the null-terminated string at a0 into the buffer at a2, appends a slash if the last copied character is not one, then appends the string at a1 and returns.
- `open_file_and_fill_io_block`: Scans the filename at offset 52 of the a3 entry, clears the remaining buffer, calls the open routine, and on success stores the file handle and fills the io block at d2 with mode, length and filename.
- `open_file_store_handle_in_io_block`: Calls the open routine with the filename at offset 52 of a3, and on success stores the returned handle, mode, length and filename into the io block at d2, returning minus one in d0 and d1.
- `push_dos_error_and_string_then_branch`: Pushes the word at address -102 and the program counter relative address of a string onto the stack, then branches to another routine.
- `run_dos_command_if_path_has_colon`: Loads the DOS base, checks a flag, scans the a0 string for a colon, and if found builds a command from the DOS structure and calls the execute routine.
- `colon_scan_data_table`: The bytes here are data words decoded as no-op ori instructions, forming a table referenced by address rather than executable code.
- `scan_string_until_colon_or_nul`: Saves d0 and a1, then advances a0 through the string until it reads a colon or a zero byte, returning with a0 pointing past the colon or at the terminator.
- `build_and_register_resource_entry`: Scales d1 by four into a3, allocates a 1005-byte block at offset 52, then passes the block, d3 and the block pointer through three helper calls before restoring d3 and a3.
- `check_d1_magic_return_flag`: Sets d0 to zero, compares d1 against the magic value, and if equal sets d0 to minus one before returning.
- `set_task_userdata_or_clear`: Calls exec.FindTask, reads the task's user data field, and if the given value is zero or its flag is negative stores it back into the task structure, returning the old value in d1.
- `return_pointer_to_static_data`: Loads the address of a static data area into a0 and copies it into d0 as the return value, then returns to the caller.
- `truncate_path_at_last_slash`: Gets a record pointer, scans its path string to the terminator, then walks backward replacing the last slash with a null or, if none, writing a colon and null at the start.
- `clear_byte_at_a0`: Writes zero to the byte pointed to by a0, copies d0 into d1, and returns to the caller.
- `copy_string_from_d1_offset_to_d2`: Selects a string from a fixed table or from an offset based on d1, then copies bytes into the buffer at a0 until a null byte or d3 count expires, returning status in d0 and d1.
- `copy_path_and_read_file_via_a2`: Copies the null-terminated string at a1 into a 256-byte stack buffer, strips a trailing slash, then calls the a2 library vector to open and read up to d0 bytes into a1, returning the result in d0 and d1.
- `pop_a1_zero_d0_and_branch`: Pops a1 from the stack, sets d0 to zero, then branches to another routine to continue execution.
- `store_base_and_init_library_calls`: Saves the incoming a0 as both a5 and a global, stores a routine pointer, then calls library offsets 52, 104 and 100 and jumps through offset 254.
- `set_screen_buffers_and_jump_a5`: Loads a base pointer from pc-relative memory, adds an address to form a buffer, stores it at offsets 6 and 18 of a5, adds an address, calls a routine, then jumps to a5.
- `set_a3_to_80000_and_jump_a5`: Loads the immediate value an address into address register a3, then jumps to the address held in a5, transferring control without returning.
- `read_and_mask_status_word`: Reads the longword at the program-counter-relative location into d0, clears bits 4 through 6 with a mask, and returns the result in d0.
- `acknowledge_interrupt_from_stack`: Writes the word at the top of the stack to the INTREQ hardware register, advances the stack pointer by two, and returns to the caller.
- `push_return_and_set_a3_to_1cd6`: Shrinks the stack by eight bytes, copies the caller's return address down, saves a3, loads the address 1cd6 into a3 and the stack slot, then returns zero in d0 with a2 in d1.
- `set_bit1_at_a1_offset_e`: Pops d1, pushes the address 1cd6, restores d1 on the stack, sets bit 1 of the byte at offset 14 from a1, and returns.
- `load_custom_base_and_branch_1cd6`: Loads the custom chip register base address into a0 and branches to the code at 1cd6, touching the custom hardware register area.
- `store_d0_to_scratch_and_set_ciab_base`: Writes the incoming d0 longword to the scratch longword at pc-relative offset, loads the CIAB base address into a6, then returns.
- `copy_scratch_longword_to_cop2lch`: Reads the scratch longword at pc-relative offset and writes it to the COP2LCH hardware register, then jumps through a0.
- `copy_a0_offset6_byte_to_a6_offset_2048`: Loads the byte at offset 6 from a0 into d0, stores that byte to offset 2048 from a6, and returns.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `check_loaded_segment_id`: Loads the longword at the program counter relative address into d1, returns 4 in d0 if it equals an address, otherwise returns 1.
- `init_two_sentinel_lists`: Writes the value minus two at a3, stores the advanced a3 pointer into a4 offset 132 and a2 offsets 50 and 54, writes minus two again, clears a4 offset 386 and skips the return address.
- `copy_two_node_pointers`: If a1 offset 4 is nonzero, copies the longword at that node's offset 4 into a3 offset 50; likewise copies a1 offset 8's node offset 4 into a3 offset 54, then skips the return address.
- `fill_three_longs_with_minus_one`: Advances a3 by twelve bytes, then writes the value minus one into three consecutive longwords below a3 using a dbra count of two, and returns.
- `compute_scroll_offset_from_level_data`: Reads three fields from the structure pointed to by the pc-relative pointer, multiplies the first by 1440, adds the second, scales by 60, adds the third divided by 50, and stores the result at offset 198 of a2.
- `select_sprite_frame_from_flags`: Clears byte 31 of a1, tests bit 1 of byte 64 of a3 to set frame 29, otherwise calls a routine and returns the byte at offset 31 of a1 in d0.
- `dispatch_joystick_state_to_library`: Checks byte at offset 29 of a1 for value 2, loads a table byte indexed by d1 into d2, then loads d0, d1 and a0 from a1 and jumps through offset 40 of the library at the pc-relative pointer.
- `test_bit_in_byte_and_set_status`: Loads a pc-relative byte into d0, tests the bit selected by d1, and if clear stores 28 into byte offset 31 of a1 before branching back.
- `format_digit_and_call_library_offset_56`: Builds a pc-relative string, converts the table byte indexed by d1 to an ascii digit at offset 5, loads d0, d1 and a1 from the structure and jumps through offset 56 of the library.
- `push_pc_relative_long_plus_offset`: Pushes the longword stored at the program-counter-relative location onto the stack, adds a fixed offset to that pushed value, and returns.
- `push_pc_relative_long_plus_small_offset`: Pushes the longword stored at the program-counter-relative location onto the stack, adds a small fixed offset to the pushed value, and returns.
- `wait_for_vpos_bit_low_then_high`: Polls bit zero of the custom chip register at offset five until it reads low, then polls again until it reads high, then returns.
- `zero_filled_data_word_at_1d28`: A single zero word that decodes as ori.b #0,d0; it is zero-filled data referenced by address, not executable code.
- `zero_filled_data_word_at_1d2c`: Another single zero word decoding as ori.b #0,d0; zero-filled data pointed to by two callers, not a real routine.

### Boot

- `boot_open_loader_and_patch_task`: Opens Loader library, finds the current task, opens dos.library, patches a field in the task structure, then closes libraries and returns a status value.

### Comic

- `jump_to_zero_vector`: Jumps to absolute address zero, transferring execution to the vector at the start of memory.
- `comic_startup_open_dos_and_task`: Saves the stack pointer, clears the error flag, checks for the CHMP signature, opens dos.library, finds the current task and gets its message port, then calls two setup routines.
- `comic_shutdown_close_and_reply`: Calls a cleanup routine, closes the opened library, forbids and replies the saved message, restores a0 and the saved stack pointer, then returns.
- `open_libs_and_start_comic_music`: Opens graphics and intuition libraries, loads the music segment, sets up the interrupt server and task, reads VHPOSR into a saved word, then jumps to the main comic loop.
- `close_libs_and_free_comic_resources`: Removes the interrupt server, stops the music, unloads the segment, closes the two opened libraries and clears the active flag.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `comic_vblank_counter_tick`: Saves registers, conditionally calls the setup routine, runs the copper and music updates, increments two frame counters and returns zero in d0.
- `poll_vblank_flag_until_clear`: Clears d0, writes zero to the vertical blank flag at address 790, then tests that flag and loops with dbra until it reads zero, returning afterward.
- `allocate_chip_memory_or_exit`: Loads d1 with a memory size and flags, calls exec AllocMem through a6, and if the returned pointer in d0 is null jumps to the exit vector.
- `close_workbench_and_load_comic_buffers`: Closes the WorkBench screen, copies a comic data block into the buffer at 65536, allocates three memory blocks, saves the copper pointer, and clears the vector base.
- `setup_comic_display_and_copy_palette`: Frees two display buffers, optionally frees a third, opens the WorkBench screen, allocates 280 bytes, stores the buffer pointer, copies 138 words of palette data into it, and sets two copper list pointers.
- `free_memory_if_pointer_nonzero`: Tests d1 and, when nonzero, moves it to a1 and jumps to the exec FreeMem vector through the library base at address 4; otherwise returns immediately.
- `swap_display_buffers_and_reset_flags`: Saves a0, swaps the two display buffer pointers at the global slots, toggles a byte flag, sets a word to 1, clears another word, restores a0 and returns.
- `refresh_comic_sprite_pointers`: If a pending flag word is set, clears it, writes four pairs of words from a stored long value into a table, then stores the table address into two fields of the copper or sprite structure.
- `copy_sixteen_words_to_two_buffers`: Saves d0 and a0-a1, then copies sixteen words from the buffer at a0 into the word at address 146 and into offset 64 of that same base, advancing both source and destination, and restores the registers.
- `alloc_mem_for_48_byte_node`: If a0 is nonzero, calls exec AllocMem for 48 bytes with flags 65537, then stores the a0 pointer at offset 14, the size at offset 18 and byte 5 at offset 8 of the new block, returning it in a0.
- `free_comic_memory_block`: If a0 is non-null, marks the block at offset 8 and clears fields at offsets 20 and 24, then calls exec FreeMem with size from offset 18 and returns the size in d0.
- `open_seek_read_comic_file`: Opens the file named by a0 with mode 1005, seeks to the start and end to get its size, reads up to d0 bytes into a1 or a newly allocated buffer, closes the file and returns the byte count.
- `close_library_and_return_minus_one`: If a2 is zero it calls a subroutine, then calls the library vector at offset -36 of a6, sets d0 to -1, clears a0, restores d1-d7/a1-a6 and returns.
- `comic_init_two_stage_setup`: Calls two subroutines in sequence then returns, acting as a small initialiser that runs both setup stages for the comic data.
- `comic_setup_display_and_swap_buffers`: Writes the value 3750 to a hardware register, calls a library routine, then runs several setup subroutines and swaps two word globals before returning.
- `comic_load_resource_with_params`: Pushes a size, a second size, zero, a data pointer and the caller's a0 onto the stack, calls a loader, then discards sixteen bytes and returns.
- `poll_five_comic_inputs_until_break`: Loads five pointers from a table, calls a setup routine on each, waits for input and returns early if the word is 10 or the swapped word is 64 or 69, otherwise draws and repeats.
- `draw_comic_frame_columns`: Saves d6, sets up a source pointer and column offset, then repeatedly calls blit routines with masks 15, 240, 3840 and 61440 to draw 18 columns before a final refresh.
- `copy_tile_rows_with_or_mask`: Copies 200 rows of 160-byte tiles from d4 into the buffer at the d4 global plus d5, ORing d3 into four words per row, then calls two subroutines and sets d7 to 2.
- `blit_sprite_rows_with_byte_and_word`: Copies 200 rows of sprite data from d4 to the destination at d5 plus the base pointer, writing byte and word fields and ORing each word with d3, then calls the blit routine and jumps with d7 set to 2.
- `blit_sprite_rows_byte_only`: Copies 200 rows of sprite data from d4 to the destination at d5 plus the base pointer, writing only byte fields and ORing each word with d3, then calls the blit routine and jumps with d7 set to 2.
- `blit_sprite_rows_word_only`: Copies 200 rows of sprite data from d4 to the destination at d5 plus the base pointer, writing only word fields and ORing each word with d3, then returns after the row loop.
- `retry_seed_three_times`: Calls the seed routine, sets d7 to 2, then branches into the retry loop that runs the random step up to three times.
- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.
- `advance_pseudo_random_seed`: Adds the 32-bit value at pc-relative offset 2398 to the running seed at 2396, stores the sum back, and returns the swapped high word in d0.
- `poll_random_until_positive`: Advances the seed, calls the system routine, tests the result and returns it if positive, otherwise retries while d7 counts down, returning zero when exhausted.
- `poll_input_bits_return_command`: Shifts a byte from the input table, tests bits 1, 3 and 4, sets or clears the flag word at the d4 global and returns a command value in d0.
- `load_pointer_from_386_into_a0`: Loads the longword stored at absolute address 386 into address register a0 and falls through to the following routine.
- `copy_2000_longwords_to_390_buffer`: Loads the destination pointer from address 390 into a1, then copies 2000 longwords from the a0 source to a1 using a dbra loop and returns.
- `comic_zero_fill_word_pair`: Writes an immediate zero byte into the low byte of d0, then tests d0 against itself with a bit test; the two words are zero-filled data, not executable code.
- `comic_ff_then_zero_fill_words`: Writes an immediate byte of all ones into d0's low byte, then three immediate zero bytes into d0; these four words are data bytes, not executable code.
- `comic_single_zero_word_data`: A single zero word that decodes as an immediate zero byte write to d0; it is a zero-filled data area referenced by address, not executable code.
- `comic_zero_fill_data_word_a`: A single zero word that decodes as ori.b with immediate zero on d0; it is zero-filled data referenced by address, not executable code.
- `comic_zero_word_data_slot_a`: A single zero word that decodes as an ori.b immediate instruction; it is zero-filled data referenced by address, not executable code.
- `comic_zero_word_data_slot_b`: Another single zero word in the same zero-filled data area, taken as an address by callers; the bytes are data rather than real instructions.
- `comic_page_advance_check`: Saves d1/a0-a2/a6, calls two helpers, tests the signed byte at the comic state address, and conditionally calls two more helpers before restoring registers and returning.
- `read_comic_input_event`: Checks the pending input word, calls the input library vector with a one-byte buffer, converts the result to a key code, clears the pending word and returns it in d0.
- `set_two_comic_globals_and_return`: Writes the value 65535 to one absolute longword global, clears a second absolute word global to zero, then returns to the caller.
- `open_console_and_input_devices`: Opens console.device and input.device via OpenDevice, stores the IO request pointers in globals, sets the input request's command to 9 and sends it with DoIO, then jumps to the exit path.
- `close_console_and_input_devices`: Sends a final DoIO on the input request, closes both devices with CloseDevice, frees their IO requests and clears the stored pointers before returning.
- `check_byte4_equals_two_and_flag`: Loads byte 4 of the structure at a0, compares it with 2, and if equal returns the word at offset a48 in d1, otherwise returns zero in d0.
- `return_a0_in_d0`: Copies the address in a0 into d0 and returns, giving callers the pointer as a return value.
- `comic_command_dispatch_by_d0`: Checks d0 equals one, reads words at offsets 6 and 8 of a0, rejects negative or bit-6-set values, and stores the pair into the shared comic state words when the busy flag is clear.
- `comic_command_36_37_or_store`: Compares d1 with 54 and 55 to branch to command handling, otherwise stores d1 and d0 into the comic state words and returns zero if the busy flag is clear.
- `set_view_flag_and_load_view`: Writes -1 to the word at address a48, then calls the graphics LoadView vector at offset -222 in the graphics library base with a1 as the view pointer, preserving a6 and returning a0.
- `clear_view_flag_and_zero_offsets`: Clears the word at a48, loads the graphics base into a0, and stores zero into the longwords at offsets 50 and 54 of that base, returning zero in d0.
- `compute_tilemap_byte_address`: Adds the word at offset 4 of a0 to d0 and the word at offset 6 to d1, multiplies d1 by the row stride at offset 6 of the pointed structure, then shifts d0 right by 3 and adds both to the base pointer, returning the byte address in a0.
- `blit_compressed_tilemap_to_bitplanes`: Expands a run-length encoded tile map from a2 into a0, writing rows of width d5 and height d2 using stride and row offsets from a1, then returns.
- `read_joystick_build_input_byte`: Reads word at offset 12 of a0, samples CIAA_PRA fire button, builds a direction/button bitmask in d1 and ORs it into the input byte in memory.
- `zero_filled_data_area_3`: Single zero word that decodes as ori.b #0,d0; this is a zero-filled data area referenced by address, not executable code.
- `alloc_and_init_msg_port`: Allocates a 34-byte message port via exec, stores the signal number, task pointer and name, initializes the port list, and returns the port pointer in d0.
- `free_msg_port_and_signal`: If the port pointer in a0 is non-null, removes its port, marks it closed, frees its signal and releases the memory block.

### Exit

- `exit_cleanup_and_restore_vectors`: Restores saved interrupt vectors and stack, frees allocated memory, then returns zero or minus one depending on allocation success.
- `copy_d1_to_d0_and_branch`: Moves the longword in d1 into d0 as the return value, then branches to another routine without touching memory or the stack.

### Loader

- `open_libraries_and_run_comic_player`: Opens dos and intuition libraries, loads the Comic and Spidey modules, invokes their entry code, closes libraries, and exits via the OS when the stack depth check passes.

### ReadMe

- `open_dos_and_play_spiderman_raw_screen`: Saves the stack pointer, finds the current task, opens dos.library, opens and reads the Spiderman raw screen file, replies to the message, closes the library and restores the stack.

### Spidey

- `jump_to_program_start`: Jumps to the absolute address zero, transferring execution to the program entry point without returning.
- `init_library_and_task_setup`: Saves the stack pointer, clears the message pointer, checks the magic value in d0, opens dos.library, finds the current task, opens the current directory and calls two setup routines.
- `close_library_reply_and_restore_stack`: Calls a cleanup routine, closes the opened library, forbids and replies the message, restores a0 from saved data and the stack pointer from the saved value, then returns.
- `open_libraries_and_start_game`: Opens dos, graphics and intuition libraries, loads music and level data, sets up interrupt server and copper, then jumps to the main game loop.
- `close_libraries_and_free_resources`: Removes the interrupt server, unloads the music segment, closes the opened libraries and frees the loaded data before returning.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `vertical_blank_counter_increment`: Saves registers, calls the copper and display update routines, increments two frame counters and returns zero in d0.
- `poll_vblank_flag_until_clear_2`: Clears d0, writes zero to the word at absolute address 316, then tests that word and branches if zero, decrementing d0 in a delay loop before returning.
- `allocate_chip_memory_or_exit_2`: Loads allocation size and flags into d1, calls exec AllocMem through a6, and if the returned pointer in d0 is null jumps to the exit vector.
- `initialize_memory_and_sprites`: Calls a routine via a pointer, then if a certain pointer is not null, copies a block of words from a source to a destination, calls another routine, and sets several global variables to specific values or results of other routines before updating a pointer-based table with a loop.
- `setup_double_buffer_and_display`: Frees two memory blocks, conditionally frees a third, then allocates a buffer, copies 138 words of display data into it and stores the pointer at offsets 50 and 54 of the graphics structure.
- `free_memory_if_pointer_valid`: Tests d1 for zero and, if non-zero, jumps to the exec FreeMem vector through the library base at address 4 to release that memory block.
- `swap_display_buffers_and_reset_flags_2`: Swaps the two longword display pointers at the global buffer addresses, toggles a byte flag, sets a word to one, clears another word and restores a0.
- `rebuild_sprite_pointer_table`: If the flag word is set, clears it, writes four pairs of sprite coordinate words from the stored long value into the table, then points two copper list slots at the table base.
- `copy_sixteen_words_to_two_buffers_2`: Copies sixteen words from the address in a0 into a buffer at a fixed low address and also into the same buffer offset by 64 bytes, advancing a0 and the destination.
- `allocate_mem_and_init_struct`: Calls exec AllocMem for 48 bytes with attributes, then stores the caller's pointer, the size and a type byte into the new structure and returns it in a0.
- `free_mem_block_and_mark_inactive`: If a0 is non-null, writes -1 to offsets 8, 20 and 24, then frees the block at a0 via exec FreeMem using size from offset 18; returns 48 when a0 is null.
- `open_seek_read_file_into_buffer`: Opens the file named by a0 with mode 1005, seeks to offset d0, optionally allocates a buffer via a helper, reads d5 bytes, closes the file and returns the buffer in a0 with length in d0.
- `open_file_and_return_handle_or_minus_one`: Saves registers, opens the file named by a0 with mode 1006 through dos Open, stores the handle in d4 and returns it in d0, or returns -1 if opening failed.
- `compare_d3_to_library_result`: Calls the a6 library offset minus 48 with d4/d1 and a2/d2, subtracts the result from d3, sets d3 to -1 if unequal, calls offset minus 36, returns d3 in d0 and restores registers.
- `call_four_setup_subroutines_then_jump`: Calls four subroutines in sequence via bsr, then branches unconditionally to another routine without returning to its caller.
- `load_spidey_level_graphics_blocks`: Copies scenery, block and sprite data into chip memory using the copy and decompress helpers, patches sprite pointers, and loops over sprite tables to fill level graphics.
- `load_charset_and_blit_three_rows`: Loads the charset data into memory, then copies three rows of 26 characters each into the destination buffer, adjusting the source pointer between rows.
- `call_open_library_and_check_result`: Calls the library open routine at low memory, tests the returned d0 value, branches away if negative, and otherwise clears a0 before returning.
- `copy_128_six_byte_glyph_rows`: Copies 128 iterations of four bytes from a0 into a1, advancing a0 by six plus d2 each time, producing a packed glyph row buffer.
- `copy_168_columns_of_4_longwords`: Copies 168 rows of four longwords from a0 to a1, advancing a0 by 160 bytes per row and rewinding a0 by 26876 between the eight outer passes.
- `return_constant_6`: Returns the constant an address in d0 to the caller without any other effect.
- `init_and_run_spidey_game_loop`: Clears state tables, sets up display and level data, then repeatedly calls the game update routines and handles level transitions until returning.
- `compute_rotated_seed_into_1194`: Reads the byte at address 4640, rotates it right by seven bits, adds 21168 and stores the longword result at 4500.
- `dispatch_key_or_fill_screen_words`: Sign-extends d0 and, if negative, jumps to a nearby handler; otherwise calls four subroutines, then fills 352 words of memory at a0 with the value 49344 using d1 as the loop counter.
- `poll_input_and_toggle_flag`: Reads input, handles the H, h and escape keys, compares the current position with the stored one, updates the stored position and inverts the byte at the flag address before returning.
- `check_underscore_key_and_copy_buffer`: Swaps d0, and if the status word is non-negative and the key is underscore, sets a word, calls a routine, then copies 2000 longword groups from one buffer to another and branches away.
- `update_sprite_offsets_and_anim_frame`: Walks a table of sprite entries, compares each entry's size against its bounds and adjusts the two sprite position words and a third offset by 256 or 176, then stores an animation frame byte and clears a bit in the sprite control byte.
- `zero_filled_data_word_at_e66`: A single zero word that decodes as ori.b with zero immediate; it is zero-filled data referenced by address, not executable code.
- `zero_filled_data_word_at_e6a`: Another single zero word decoding as ori.b with zero immediate; it is zero-filled data pointed to by address, not executable code.
- `update_entity_activation_and_animation`: Scans entity records, checks positions against active slots, toggles bit flags, sets animation frame counters and calls sound routines when entities activate.
- `scroll_and_blit_tile_column`: Decrements a delay counter, updates horizontal scroll offset, copies a 44-word tile column into the screen buffer and blits shifted tile rows.
- `init_scroll_and_blit_tiles`: Sets scroll counters and offsets, points a1 at the screen buffer plus offset and a0 at tile data, then jumps into the tile blit loop.
- `spidey_aim_direction_and_shot_setup`: Reads the aim byte, picks a direction from a table, checks the tile map at the shifted coordinates for walls, stores the chosen direction and screen position, sets the aim flag and calls the shot routine.
- `zero_filled_aim_scratch_data`: A single zero word that decodes as an ori instruction; it is zero-filled data referenced by address, not executable code.
- `aim_scratch_word_then_play_sound`: Starts with a zero word used as scratch data, then tests a flag bit, and if clear loads sound number nine and calls the sound routine.
- `spidey_raycast_visibility_scan`: Clears two display words, tests flag bits, then walks a ray across the tile map checking bytes against 63 to detect blocked line of sight, returning early when blocked.
- `update_parameter_state`: Loads two addresses into a1 and a2, calls another routine, stores d4 and d5 into the locations pointed to by a1, calls another routine, loads a pointer from a global variable into a0, compares d0 to 3, performs a table lookup in d0 if it is not less than 3, calls another routine, loads a pointer from a global variable into a0, calls another routine, and stores d7 into a global variable.
- `compute_screen_offset_from_scroll`: Builds d4 from the word at 2156 plus scroll and 217c plus ten, and d5 from byte 2158 plus scroll and 217e plus ten, returning both offsets.
- `calculate_vector_intersection`: Calculates intersection coordinates by performing arithmetic on two vectors derived from global pointers and input registers.
- `update_all_spider_slots`: Iterates over the spider slot array, clears each slot's flag byte, then for each active slot runs its update calls, collision checks and position updates before advancing to the next slot.
- `redraw_active_spider_slots`: Walks the spider slot array, and for each slot whose object flag bit is set, calls the slot draw routine and updates its screen position via the position routine.
- `setup_blit_coords_from_sprite_and_scroll`: Loads sprite pointer from a6 offset 72, reads its width and height, adds scroll offsets at a6+40 and a6+42, then jumps to the blitter setup routine with a1 pointing at a fixed table.
- `check_player_collision_tiles`: Tests a global flag, reads the player object flags at a5+12, and probes tile maps around the player position to set the state byte at a6 to 0, 1 or 4.
- `find_tile_at_screen_position`: Reads a position from a0, scans the object table at a fixed base for a matching bounding box, increments its hit counter at offset 13 and returns the tile offset, or zero if none match.
- `move_entity_tiles_by_delta`: Uses d3/d4 pixel deltas to compute tile-aligned offsets, then updates tilemap and sprite position fields for each matching entity slot in the a6 table, calling the collision helper when needed.
- `build_level_tile_and_sprite_tables`: Reads level header bytes at offsets 714/715 from a0, fills the a5 tile descriptor table and a1 sprite records, links tile graphics pointers, and calls the sprite setup routines for each entry.
- `blit_tile_columns_to_bitplanes`: Copies four interleaved tile rows from a0 and a6 source pointers into four bitplane destinations a5/a4/a3/a2, stepping 30 bytes per row across three by three tile blocks.
- `copy_sprite_blocks_to_buffer`: Copies eight blocks of ten registers from the source table at a fixed low address into the destination buffer, advancing both pointers by forty-four bytes each iteration.
- `scan_entity_table_call_active_entries`: Walks the entity table entries of one hundred thirty-eight bytes, calling a subroutine for each entry whose first byte is non-negative, decrementing the count in d7.
- `build_entity_screen_coordinates`: Loops over entity records, computes screen x and y from sprite offsets and stored positions, writes three bytes per record to the pointer at offset four, then sets a flag byte.
- `update_all_sprite_positions_and_scroll`: Iterates over 64 sprite records, clears their pixel masks, applies velocity deltas to x and y, clamps them to screen bounds, and updates the scroll offset counter.
- `build_sprite_collision_cell_lists`: Scans sprite pixel rows and columns for solid cells, writes cell coordinates to the a5 list, and checks candidate sprites for overlap against the current sprite bounds.
- `call_two_sprite_helpers_and_clear_d3_d4`: Calls the two sprite helper routines, then sets d3 and d4 to zero and returns to the caller.
- `compute_sprite_cell_offset`: Reads two words at offsets 8 and 10 of the a2 structure, rounds the first to a multiple of 8 and scales it, adds the second shifted right by 3, then sets a3 to a0 plus that offset and loads bytes at offsets 2 and 3 into d1 and sign-extended d2.
- `blit_sprite_rows_from_a2`: Calls the offset routine, then copies d2+1 bytes per row from a4 stepping 6 bytes into a3 stepping 1 byte, advancing a3 by 31 minus d2 and a4 by 42 minus 6*d2 each of d1+1 rows.
- `toggle_bit7_shifted_into_byte_c`: Loads byte at offset 12 of a2, isolates bit 7, shifts it right by one, exclusive-ors it back into that byte, clears d4 to zero and returns.
- `toggle_bit3_into_bit2_of_offset_c`: Reads byte at offset 12 from a2, isolates bit 3, shifts it right by one, XORs it back into that byte, clears d3 and returns.
- `copy_entity_pointer_and_dispatch_39a`: Saves registers, stores the longword at offset 14 of a2 to the global at 117e, loads a1 with 39a, passes words from offsets 8 and 10 of a2 to the routine at 15bd0, restores registers and returns.
- `decode_entity_script_stream_to_d1_buffer`: Walks a byte stream at offset 742 of a0 using the count and flags at offsets 715 and 716, writing decoded words through a1 and calling the two subroutines for special bytes until the counter expires.
- `decode_tile_object_record`: Reads two bytes from a2, computes a tilemap offset from a0, checks the tile value range, toggles bit 0, and writes a record of pointers and fields to a1.
- `apply_tilemap_edit_commands`: Clears a bit in a6, calls the tile refresh routine, then loops over command bytes from a2 setting, clearing or toggling bits in a tilemap and updating tile records.
- `refresh_tile_after_edit`: Increments d6, checks the edited tile's value range, toggles its bit 0, then copies eight rows of four bytes from the tile graphics table into the destination buffer.
- `blit_tile_column_to_bitplanes`: Reads a tile index from (a2), looks up its 32-byte graphics pattern and copies eight rows of four bytes into the four bitplane buffers at a5, advancing by d2 and d4 per column.
- `dispatch_active_entity_updates`: Reads a count and table of entity records, tests each record's bit flag against its object byte, and calls the entity update routine for every record whose bit is set.
- `call_entity_update_with_null_list`: Saves a0 on the stack, loads a1 with address zero, calls the entity update routine at 2478, then restores a0 and returns.
- `load_sprite_bank_pointers`: Reads a byte from absolute memory into d0, sign-extends it to a word, then loads a0 with one absolute address and a1 with another, setting up source and destination pointers.
- `decompress_rle_tile_stream`: Uses d0 as an index into a table at a1 to find a compressed stream, then expands run-length and back-reference tokens into the buffer at a0 until the end pointer in d7 is reached.
- `run_animation_script_interpreter`: Checks a global flag and object status bits, then walks the script pointer at offset 14 of a6, decoding opcodes into sprite table lookups and calling subroutines to update the object's animation state.
- `parse_script_commands_until_end_marker`: Reads command bytes from the pointer in a0, resolving coordinates through helper routines and updating the entity state at offsets in a6 until the end marker byte is reached.
- `compute_entity_bounding_box`: Uses the entity index at offset 6 in a6 to look up flags, then writes four bounding-box words at offsets 30 to 36 in a6 from d0, d1 and the size bytes at offsets 28 and 29.
- `save_script_pointer_and_skip_to_marker`: Stores a0 and the entity position fields at offsets 2, 4 and 6 into a6 offsets 60 to 68, then advances a0 past bytes until the end marker is found.
- `load_sprite_frame_and_apply_velocity`: Reads a byte from a0 as an index into a pointer table, stores the selected pointer at offset 18 of a6, optionally loads a second pointer into global memory, then adds velocity fields to position and snaps coordinates to an 8-pixel grid.
- `scan_entity_list_for_collision_flag`: Clears counters at offsets 22 and 1 of a6, walks the entity list at offset 8 using count at offset 12, calling a collision test and stopping when a byte has its top two bits set or clear.
- `advance_animation_and_apply_velocity`: Loads a byte from the pointer at offset 18 of a6 into the flags byte, calls two subroutines, adds doubled velocity fields to position, and clears counters at offsets 40, 42, 54 and 56.
- `read_three_bytes_from_a0`: Reads a word into d2 from the address in a0, clears d3, then reads two bytes into d3 and d5, advancing a0 past each read.
- `select_tilemap_and_fetch_byte`: Chooses one of several tilemap bases in a1 based on the signs and ranges of d2 and d3, then indexes the table with d3 times 32 plus d2 and returns the byte in d0.
- `update_entity_motion_and_collision`: Adds d2 and d3 to the entity's position at a6, clamps its velocity, computes screen coordinates, probes the tilemap in both axes and sets collision flags in the entity's status byte.
- `negate_d0_d1_if_negative`: Tests d0 and negates its low byte if negative, then tests d1 and negates its low byte if negative, then returns.
- `load_d4_word_and_d5_byte_from_a6`: Loads the word at offset 2 of a6 into d4, clears d5, then loads the byte at offset 4 of a6 into d5.
- `add_ten_compare_d5_shift_d4_d5`: Adds ten to d4 and d5, compares d5 with 210 and branches if less, otherwise sign-extends d5 and arithmetic-shifts d4 and d5 right by three.
- `apply_direction_flags_to_delta`: Tests bits 0-2 of d1 to optionally swap d2 and d3, negate d3 and negate d2, then sign-extends d2 to a word and returns.
- `build_sprite_attr_from_d0_and_a6`: Saves a0, masks d0 to low nibble, calls the routine at 27de with d1 from offset 7 of a6, ORs bits 4-5 of a6 into d0, restores a0 and returns.
- `call_27ce_with_saved_a0`: Pushes a0, loads the address 27ce into a0, calls the routine at 3e52, restores a0 from the stack and returns.
- `call_nibble_lookup_with_table_a0`: Saves a0 on the stack, loads the address of a nibble lookup table into a0, calls the nibble extraction routine, then restores a0 and returns.
- `extract_nibble_from_table_by_index`: Combines d0 shifted left three bits with d1 to form a byte index, halves it, reads a byte from the table at a0, and returns either its high or low nibble depending on the low bit of d1.
- `check_player_overlap_and_advance_counter`: Saves registers, verifies a6 is not the player object and has a flag set, tests the player's bounds against the object's rectangle, and if overlapping adds the object's word at offset 52 to a global counter and calls a routine with d0 set to 12.
- `find_nearest_active_entity_to_point`: Scans entity bounds at offsets 30-36 of a5 against two coordinate pairs in a1 and a2, computes perpendicular distances, and stores the closest entity's center and pointer in globals.
- `scan_entities_clear_bit7_and_dispatch`: Reads a count from memory, walks entity records stepping 438 bytes, and for each whose flags byte has bit 7 set calls the sub-handler; if any were processed it stores a marker, clamps a counter to 3, sets a byte to 255, clears bit 3 of a flags byte and jumps to the next state with d0=4.
- `zero_filled_data_area_4`: Contains only a zero word that decodes as ori.b #0,d0; this address is a zero-filled data area referenced by other code, not executable logic.
- `zero_filled_data_area_5`: Contains only a zero word that decodes as an ori.b instruction; the bytes are data, not executable code, referenced by address from other code.
- `scan_tile_rows_for_collision`: Scans two perpendicular tile rows using d2 and d5 offsets, calls collision test routines, and advances the a6 offset by two when a hit is found.
- `blit_tilemap_into_bitplanes`: Reads tile indices from two tables, selects tile graphics from either the d6 base or the d1 pointer, and copies four bitplane bytes per tile into the a1 destination.
- `blit_tilemap_to_bitmap_and_copy_palette`: If the flag byte is set, remaps tile indices in the tilemap and copies 32 rows of 22 tiles into the bitmap buffer, then copies 704 longwords of palette data to the copper area.
- `blit_sprite_frames_to_bitmap`: Reads sprite frame data from d7 and writes 21 rows of pixels into the bitmap at a6+72, optionally flipping, offsetting and expanding nibbles through a lookup table, then stores d0 and d1 at offsets 18 and 20.
- `sprite_lookup_table_data`: The bytes are data, not executable code: an immediate value loaded into d0 followed by a branch and jump that are table entries, used as a pixel lookup table by the sprite blitter.
- `jump_to_b30c_entry`: Performs an unconditional jump to the fixed code at address b30c, transferring execution there without saving any registers.
- `line_editor_key_dispatch`: Reads a key code, then edits the 24-byte text buffer at the cursor: inserts, deletes, backspaces, moves the cursor, or clears the line, updating cursor and length globals.
- `redraw_line_from_cursor`: Clears the cursor, computes the remaining character count from the cursor offset into the text buffer, then redraws the line and refreshes the cursor.
- `set_byte_119b_to_ff`: Writes the immediate byte value 255 to the absolute long memory location at 4507, then falls through into the following routine.
- `clear_d0_and_branch_to_48a6`: Sets d0 to zero with a quick move and then branches unconditionally to the code at address 18598, transferring control there.
- `clear_byte_119b_and_return`: Clears the byte at absolute long memory location 4507 to zero and returns immediately to the caller.
- `decrement_two_sprite_x_positions`: Subtracts 8 from the word values at the two longword-addressed globals, moving both positions left, then returns.
- `increment_two_sprite_x_positions`: Adds 8 to the word values at the two longword-addressed globals, moving both positions right, then returns.
- `set_two_positions_from_byte_index`: Reads a signed byte global, multiplies it by 8, adds 64 and 71, and stores the two results into the two position globals before returning.
- `fill_seven_rows_from_two_tables`: Uses a global index to pick an offset into two word tables, then for seven rows copies four words from each table into a screen buffer at offsets 0, 40, 80 and 120, zero-filling the remaining columns and advancing the destination by 144 bytes per row.
- `setup_sprite_blit_and_loop_counter`: Calls the tile column blit routine, points a2 at bitmap memory plus offset, loads d1 and d2 with blit sizes, reads a word from d0, calls the blit routine, then sets a0 and d3 for a following loop.
- `copy_tile_column_bytes_to_bitmap`: Reads a byte index from a0, scales it by thirty-two, adds d1, then copies eight rows of four bytes into a2 at strides of forty and one hundred sixty, advancing a2 and repeating d3 times.
- `draw_digit_glyphs_to_screen`: Converts d0 into four decimal digit characters stored backwards at address 4945, then copies six 8-row glyphs from the table indexed by d1 into the screen buffer at a2.
- `render_score_panel_rows`: Calls the setup routine, opens a display window, then loops sixteen times drawing score digits and glyph rows into the screen buffer at a5, advancing a2 and stepping a5 down by 1600 each pass.
- `select_level_and_build_name_buffer`: Compares a value against a table of 16 thresholds to pick a level index, sets four display words, copies 24 words and 24 bytes per level into buffers, appends the value and spaces, then calls three subroutines, clears the value and jumps to the next state when the index is non-negative.
- `advance_animation_frame_and_jump`: Increments a byte counter at a fixed long address, masks it to even values, reads a word from a table indexed by that counter, loads three fixed long addresses into a1, a2 and a0, then jumps to a fixed routine.
- `blit_sprite_tiles_to_screen`: Computes a screen destination from the row counter and column index, then copies eight rows of four-byte tile data per column into the bitmap at offsets of forty and one hundred twenty bytes.
- `confirm_quit_prompt_loop`: Draws the quit confirmation text, waits for a key, loops until Y, N or Escape is pressed, sets a flag and clears a counter, then copies two thousand longwords from one buffer to another.
- `set_d3_sprite_row_count`: Loads the immediate value 19 into data register d3, setting the row counter used by the following sprite blit routine.
- `blit_sprite_rows_to_bitplanes`: Reads sprite indices from (a0), looks up 32-byte sprite data at d1, and copies 8 rows of 4 bytes into bitplane columns at d4, advancing d4 by 1920 bytes per sprite until d3 expires.
- `init_sprite_blit_and_play_game_text`: Calls setup routines, loads 4 into d0 with a text pointer, calls a text routine, then loads d4 from the frame counter plus an offset and runs three sprite blits before loading a2 from the frame counter.
- `blit_five_tile_rows_into_bitplanes`: Advances a2 by a fixed offset, computes a source pointer from two globals, then calls the tile-copy routine five times with a2 stepped by 640 bytes between calls.
- `copy_eight_tile_rows_to_bitplanes`: Adds a word offset from (a0) to a base pointer, then copies eight groups of four bytes into a2 at offsets 0, 40, 80 and 120, advancing a2 by 160 each iteration.
- `accumulate_32bit_seed_and_swap`: Adds the longword at a pc-relative location to a running longword accumulator in memory, stores the sum back, and returns the accumulator with its halves swapped in d0.
- `set_d7_zero_and_branch_to_loop`: Loads zero into d7 and branches unconditionally to the loop body at the following routine, setting up the counter for that code.
- `poll_input_until_condition_or_d7_expires`: Calls three subroutines, returns if the zero and overflow flags differ, otherwise decrements d7 and repeats until d7 reaches minus one, then returns zero in d0.
- `shift_input_bits_and_select_command_code`: Shifts a byte from the input buffer, tests bits 1, 3 and 4, sets or clears the word flag at d6 offset, and returns a command code in d0.
- `set_palette_and_clear_d0_return`: Loads address 922 into a0, calls the routine at 91782, advances a1 by 8, sets d0 to zero and jumps to the same routine again.
- `push_rect_args_and_call_blit`: Pushes height 200, width 320, zero, address 922 and address 60898 onto the stack, calls the routine at 45836, then discards 16 bytes and returns.
- `clear_4000_longwords_at_pointer`: Loads the longword pointer at address 912 into a1, then writes zero to 4000 consecutive longwords starting there and returns.
- `load_two_global_pointers`: Loads the longword at absolute address 902 into a0 and the longword at absolute address 912 into a1, then falls through to the next routine.
- `copy_2000_longwords_a0_to_a1`: Copies 2000 longwords from the address in a0 to the address in a1, advancing both pointers, then returns to the caller.
- `data_bytes_decoded_as_ori_btst`: The bytes here are data, not executable code; decoded as instructions they would set a zero immediate on d0 and test d0 against itself.
- `spidey_zero_fill_data_table`: The bytes decode as ori.b immediate instructions on d0, but this is zero-filled data referenced by address, not executable code.
- `spidey_zero_word_data_slot`: A single zero word that decodes as ori.b #0,d0; it is a zero-filled data slot pointed to by other code, not a routine.
- `spidey_zero_word_handler_slot`: One zero word decoding as ori.b #0,d0, used as a handler pointer target in data; the bytes are data, not executable code.
- `zero_filled_data_word_at_4c3c`: This single zero word decodes as ori.b with immediate zero to d0; it is zero-filled data referenced by address, not executable code.
- `zero_filled_data_word_at_4c40`: This one zero word appears as ori.b immediate zero to d0; the bytes are a zero-filled data area pointed to by absolute moves and handler pointers.
- `zero_filled_data_word_at_4c44`: A lone zero word that decodes as ori.b with zero immediate to d0; it is data, not code, reached through absolute address references.
- `poll_input_until_condition_met`: Saves d1 and a0-a2/a6, calls the input poll and a system routine, tests the byte at a global flag, and repeats the poll and checks until the flag condition is satisfied, then restores registers and returns.
- `read_serial_port_byte`: Saves registers, checks a status word, calls the serial device read routine via library offset, converts the returned byte, clears the status word and returns the byte in d0.
- `set_screen_flag_and_clear_next_word`: Writes the value minus one to the longword at offset 4c24 and clears the word at offset 4c26, then returns to the caller.
- `open_console_and_input_devices_2`: Opens console.device and input.device via exec.OpenDevice, stores the device pointers and console unit, sets the input device's handler and signal, then jumps to the main loop.
- `close_console_and_input_devices_2`: Sends a close command to the console, closes the input and console devices with exec.CloseDevice, frees their structures and clears the stored device pointers.
- `handle_input_event_and_menu_keys`: Reads an input event structure, records key codes, toggles the pause flag, and processes menu, cursor and typed-character keys into the cheat entry buffer.
- `init_level_and_wait_for_two_inputs`: Calls setup routines, stores -1 at a fixed longword, then repeatedly calls a check routine until it returns nonzero twice before returning.
- `load_level_tiles_and_select_tilemap`: Calls graphics setup routines, draws repeated tiles, reads a value, subtracts 49 and if in range 0 to 4 stores a tilemap pointer offset by 64-byte entries.
- `clear_text_buffer_with_spaces`: Stores the address of a buffer at a fixed pointer, clears a byte flag, then fills 24 bytes with space characters using a decrementing loop.
- `randomize_tile_and_play_sound`: Writes sound and tile parameters to fixed memory, calls several subroutines, picks a random value masked to a multiple of four, stores it, and plays up to five tile entries.
- `compare_uppercase_string_skip_spaces`: Skips spaces at a0, then compares bytes from a1 against a0, converting lowercase a0 bytes to uppercase, returning the mismatching byte or the a0 byte when a1 ends.
- `compute_tilemap_byte_offset`: Adds d0 to word at offset four of a0 and d1 to word at offset six, multiplies d1 by a word from the pointed structure, then returns a0 advanced by the scaled d1 plus d0 shifted right three.
- `blit_compressed_tilemap_to_bitmap`: Expands a run-length encoded tile map from a2 into the bitmap at a0, using tile width and stride from a1, writing rows and columns with dbra loops.
- `jump_to_absolute_vector`: Unconditionally jumps to the absolute address stored in the long immediate operand, transferring control to an external routine.
- `blit_masked_sprite_with_blitter`: Owns the blitter, computes source and destination pointers and modulo values, programs BLTCON and BLTSIZE for a masked sprite blit, then disowns the blitter.
- `clip_line_to_viewport_bounds`: Clips a line segment between two points against the rectangle bounds at offsets 8 and 10 of the structure pointed to by a0, adjusting the endpoint coordinates in place and calling the line draw routine.
- `setup_and_dispatch_line_draw`: Computes the line start address in the bitmap from the coordinates and structure fields, selects a slope-specific drawing routine, builds a pixel mask from d4, waits on DMACONR bit 6, then jumps to the chosen routine.
- `steep_line_pixel_loop`: Repeatedly calls the pixel plot routine at a1, rotates the mask words d0 and d4, advances a3 by the row stride d5, and adjusts the error term d1 until d3 counts down to minus one, then restores registers and returns.
- `blit_mask_shift_loop`: Calls the routine in a1, advances a3 by d5, subtracts d7 from d1, rotates d0 and d4 right with carry, conditionally adds 2 to a3, and loops on d3 before restoring registers.
- `sprite_clip_and_draw_dispatch`: Computes clipped sprite coordinates from a0/a1/a2, selects a draw routine via a jump table indexed by d0, fills sprite fields at offsets in a6, and calls the blit helpers.
- `mask_and_advance_a2_loop`: Repeatedly ANDs d2 into bytes at a2, advancing a2 by d4 after each of four stores, looping on d3 and returning.
- `blit_mask_or_first_then_and_rest`: For each of d3+1 rows, ORs d1 into the byte at a2, then ANDs d2 into the next three bytes spaced d4 apart, advancing a2 by four d4 steps.
- `blit_mask_and_first_then_or_second`: For each of d3+1 rows, ANDs d2 into the byte at a2, ORs d1 into the next byte spaced d4 away, then ANDs d2 into two more bytes, advancing a2 by four d4 steps.
- `blit_mask_or_two_then_and_two`: For each of d3+1 rows, ORs d1 into the first two bytes spaced d4 apart from a2, then ANDs d2 into the next two bytes, advancing a2 by four d4 steps.
- `mask_or_mask_plane_loop`: For each of d3+1 rows, ANDs d2 into the byte at a2, then ORs d1 into the next two bytes and ANDs d2 into the fourth, advancing a2 by d4 each time.
- `or_mask_or_mask_plane_loop`: For each of d3+1 rows, ORs d1 into the byte at a2, ANDs d2 into the next byte, ORs d1 into the third and ANDs d2 into the fourth, stepping a2 by d4.
- `mask_or_or_mask_plane_loop`: For each of d3+1 rows, ANDs d2 into the byte at a2, ORs d1 into the next two bytes, ANDs d2 into the fourth, and advances a2 by d4.
- `blit_mask_or_or_or_and_stride`: Loops d3+1 times, ORing d1 into three bytes and ANDing d2 into a fourth, advancing a2 by d4 each time, then returns.
- `blit_mask_and_and_and_or_stride`: Loops d3+1 times, ANDing d2 into three bytes and ORing d1 into a fourth, advancing a2 by d4 each time, then returns.
- `blit_mask_or_and_and_or_stride`: Loops d3+1 times, ORing d1 into the first byte, ANDing d2 into two middle bytes and ORing d1 into the last, stepping a2 by d4, then returns.
- `mask_or_plane_pattern_a2_stride_d4`: For each of d3+1 iterations, ANDs d2 into the byte at a2, ORs d1 into the byte d4 bytes later, then repeats AND and OR at further d4 strides, advancing a2.
- `or_or_mask_or_plane_pattern_a2_stride_d4`: For each of d3+1 iterations, ORs d1 into the byte at a2 and the next d4-strided byte, ANDs d2 into the third, ORs d1 into the fourth, advancing a2 by four d4 strides.
- `mask_and_set_bytes_with_stride`: Repeatedly ANDs d2 into bytes at a2, ORs d1 into later bytes, advancing a2 by d4 each step, until d3 counts down to minus one, then returns.
- `or_and_or_or_stride_blit`: For each of d3+1 iterations, ORs d1 into the byte at a2, ANDs d2 into the next byte, then ORs d1 into two more bytes, advancing a2 by d4 each time.
- `and_or_or_or_stride_blit`: For each of d3+1 iterations, ANDs d2 into the byte at a2, then ORs d1 into the next three bytes, advancing a2 by d4 after each byte.
- `or_four_bytes_stride_blit`: For each of d3+1 iterations, ORs d1 into four consecutive bytes starting at a2, advancing a2 by d4 after each byte, then returns.
- `and_d0_mask_three_words_stride_d2`: ANDs d0 into the word at a3, then advances a3 by d2 and ANDs d0 into two more words, finally ANDs d0 into the word at a3 plus d2 and returns.
- `or_d4_then_and_d0_mask_stride_d2`: ORs d4 into the word at a3, then advances a3 by d2 and ANDs d0 into two further words, then ANDs d0 into the word at a3 plus d2 and returns.
- `and_d0_then_or_d4_mask_stride_d2`: ANDs d0 into the word at a3, advances a3 by d2, ORs d4 into that word, then ANDs d0 into two more words and returns.
- `or_then_and_word_blit_a3_stride_d2`: ORs d4 into the word at a3, advances a3 by d2, ORs d4 again, then ANDs d0 into two further words at stride d2 before restoring a3 and returning.
- `and_then_or_word_blit_a3_stride_d2`: ANDs d0 into the word at a3, advances a3 by d2, ANDs d0 again, then ORs d4 into one word and ANDs d0 into the next at stride d2 before restoring a3 and returning.
- `or_and_or_word_blit_a3_stride_d2`: ORs d4 into the word at a3, advances a3 by d2, ANDs d0 into the next word, ORs d4 into the third, ANDs d0 into the fourth at stride d2, then restores a3 and returns.
- `mask_or_or_mask_strided_words`: Clears bits of the word at a3 using d0, then ORs d4 into the next two words spaced by d2, and finally masks the word at a3 plus twice d2 with d0.
- `or_or_or_mask_strided_words`: ORs d4 into the word at a3 and into the next two words spaced by d2, then masks the word at a3 plus twice d2 with d0.
- `mask_mask_mask_or_strided_words`: Masks the word at a3 and the next two words spaced by d2 with d0, then ORs d4 into the word at a3 plus twice d2.
- `blit_mask_or_and_pattern_a3`: ORs d4 into the word at a3, then ANDs d0 into words at a3+d2 and a3+2*d2, ORs d4 into the word at a3+3*d2, and returns.
- `blit_mask_and_or_pattern_a3`: ANDs d0 into the word at a3, ORs d4 into the word at a3+d2, ANDs d0 into a3+2*d2, ORs d4 into a3+3*d2, and returns.
- `blit_mask_or_or_and_pattern_a3`: ORs d4 into words at a3 and a3+d2, ANDs d0 into a3+2*d2, ORs d4 into a3+3*d2, then returns to the caller.
- `mask_or_four_words_at_a3_stride_d2`: Masks the word at a3 with d0, then advances a3 by d2 and masks again, then ORs d4 into two further words spaced by d2 and restores a3.
- `or_mask_or_four_words_at_a3_stride_d2`: ORs d4 into the word at a3, advances a3 by d2 and masks that word with d0, then ORs d4 into two more words spaced by d2 before restoring a3.
- `mask_then_or_three_words_at_a3_stride_d2`: Masks the word at a3 with d0, advances a3 by d2, then ORs d4 into three successive words spaced by d2 and restores a3 before returning.
- `or_d4_into_a3_stepped_words`: ORs the low word of d4 into four words spaced by the signed word in d2 starting at a3, then returns.
- `own_blitter_via_graphics_library`: Loads the graphics library base, saves d0-d1/a0-a1, calls OwnBlitter, restores them and loads the custom chip base into a6.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `call_disown_blitter`: Library stub: loads its arguments and the graphics library base, calls DisownBlitter and returns its result to the caller.
- `blit_masked_tile_rows_to_bitmap`: Saves registers, computes destination and source offsets from a1 and a0, then loops 28 times ORing rotated masked source longs into the destination bitmap.
- `draw_sprite_tile_strip_with_offsets`: Reads tile coordinates from a1 and a0, computes a bitplane address, sets sprite control words at offsets 64-116 of a6, and writes four successive sprite positions while waiting on the blitter busy bit.
- `draw_sprite_line_from_tilemap`: Uses a0 tilemap entry to compute a source address in a2, sets sprite control words at offsets 64-112 of a6, and writes three successive sprite positions while polling the blitter busy bit.
- `zero_filled_data_table`: A run of ori.b zero instructions that change nothing; the bytes are data, not executable code, and the routine returns without clearing or calling anything.
- `dispatch_handler_from_table`: Checks if a 16-bit value is non-zero, then uses it as anindex into a table to fetch a pointer and calls another routine with that pointer; otherwise it returns.
- `allocate_sound_slot_from_stream`: Scans four sound slots for a free one, fills its fields from the data stream at a2 (id, volume, sample pointer, priority) and marks it active, returning minus one if none free.
- `stop_sound_slot_by_id`: Loops over four sound slots, and for each whose id matches the word argument and whose flags are set, clears the flags and calls the stop routine for that slot.
- `update_four_animation_slots`: Walks four 42-byte actor slots from a base pointer, calls the per-slot updater, and dispatches script opcodes that set position, speed, and frame data.
- `advance_actor_position_and_speed`: Decrements the actor's horizontal and vertical timers, adds speed to x and y, clamps to bounds, and reverses or clears speed according to flag bits.
- `random_in_range_between_two_args`: Calls the random generator, takes the low word of the result, divides by the difference of the two stack arguments, and returns the remainder plus the lower bound.
- `add_pc_relative_32bit_counter`: Loads the address of a nearby long counter, adds the longword at offset two to the longword at offset zero, stores the sum back, and returns the swapped sum in d0.
- `set_entity_flag_and_state_byte`: Tests byte at offset two of the a1 structure; if nonzero, sets bit five there, writes one to offset twenty-two, and returns one in d0.
- `build_joystick_direction_bits_from_word`: Reads word at offset twelve of a0, folds bits, tests CIAA_PRA fire bit, sets direction bits in d1, then ORs them into two adjacent bytes.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `zero_fill_data_block_at_170d8`: Four zero words decoding as ori.b #0,d0; this is a zero-filled data block referenced by address, with no branch or call.
- `zero_fill_data_block_at_170e8`: Four consecutive zero words decoding as ori.b #0,d0; a zero-filled data area pointed to by address, containing no executable logic.
- `zero_fill_data_table_a`: Four zero words that decode as ori.b instructions; this is a zero-filled data area referenced by address, not executable code.
- `zero_fill_data_at_17108`: Four zero words that decode as ori.b #0,d0; this is a zero-filled data area referenced by address, not executable code.
- `zero_fill_data_at_17118`: Another four zero words decoding as ori.b #0,d0; a separate zero-filled data block whose address is taken as a handler pointer.
- `zero_filled_handler_slot_a`: Four zero words that decode as ori.b #0,d0, so this is a zero-filled data area referenced by an address-taken handler pointer, not executable code.
- `zero_filled_handler_slot_b`: Another four zero words decoding as ori.b #0,d0, marking a second zero-filled data slot pointed to by a handler pointer rather than real instructions.
- `zero_filled_handler_slot_c`: Two zero words decoding as btst.l d0,d2 and btst.b #0,d4, indicating a short zero-filled data area referenced by an address-taken handler pointer.
- `open_audio_and_init_three_channels`: Sets the CIAA port bit, opens audio.device, allocates three channel structures and their IO blocks, links them, and initializes volume and pointers for each of three channels.
- `close_audio_and_free_channel_blocks`: Loops over three channel slots, closes the audio device, frees each allocated memory block, releases the IO structures, and clears the CIAA port bit if no other user remains.
- `set_channel_period_and_trigger_output`: Reads a channel index argument, selects the matching audio IO block, stores period 11 and clears the volume byte, then calls the device output routine through its library vector.
- `configure_two_anim_objects_by_index`: Looks up two animation objects by index from tables, sets their type and flags, copies caller fields into the second object, and calls each object's update vector.
- `set_anim_object_type_and_position`: Looks up an animation object by index, sets its type to twelve, clears its flag byte, stores two caller words as position fields, and calls its update vector.
- `allocate_and_init_message_port`: Allocates a signal, memory block and message port, fills port fields from arguments, initializes the message list, and returns the port pointer or zero on failure.
- `free_signal_port_and_memory`: Saves registers, then if a0 is non-null frees its signal via exec.FreeSignal, marks bytes at offsets 8 and 20 as 255, frees the signal number in byte 15, and calls exec.RemPort and exec.FreeMem.

### Music

- `branch_forward_to_offset_eight`: Branches unconditionally forward by eight bytes, transferring execution to a later instruction without touching any registers or memory.
- `add_int_server_then_return`: Branches forward, calls a subroutine, loads exec base into a6, loads a string address into a1, sets d0 to 5 and calls exec.AddIntServer before returning.
- `set_music_flag_to_ff`: Loads the address of a music flag via pc-relative lea into a0 and stores byte 255 there, then returns.
- `music_tick_sequencer`: Increments a music tick counter, and when it reaches the tempo limit resets it, calls the four voice update routines for the audio channels, waits, and reloads channel pointers and period registers.
- `music_voice_update`: Reads a note entry from the song data, looks up instrument and sample tables, sets the audio channel pointer, length and period registers, and handles effect bytes such as volume and tempo.
- `music_track_seek_target_setup`: Reads the requested track from (a6), masks it to twelve bits, stores it as the seek target, and sets a direction flag byte when the target is below the current position.
- `music_command_dispatch_and_volume_control`: Dispatches on the low nibble of the command word at offset two of a6, adjusting volume at offset 18, position at offset 16, and toggling CIAA_PRA bit for the filter and channel enable flags.
- `music_init_allocate_channels_and_silence`: Scans the song header for the highest channel index, clears thirty channel structures, stores their pointers, sets CIAA_PRA bit one, and silences all four AUDxVOL registers before resetting the player state.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_8`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_9`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_10`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_11`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_12`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_13`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_14`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_15`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_16`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_17`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_18`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_19`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_20`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_21`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_22`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_23`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_24`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_25`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_26`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_27`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_28`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_29`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_30`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_31`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_32`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_33`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_34`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_35`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_36`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_37`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_38`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_39`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_40`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_41`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_42`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_43`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_44`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_45`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_46`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_47`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_48`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_49`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_50`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_51`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_52`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_53`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_54`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_55`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_56`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_57`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_58`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_59`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_60`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_61`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_62`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_63`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_64`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_65`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_66`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_67`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_68`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_69`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_70`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_71`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_72`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_73`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_74`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_75`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_76`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_77`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_78`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_79`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_80`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_81`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_82`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_83`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_84`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_85`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_86`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_87`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_88`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_89`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_90`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_91`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_92`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_93`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_94`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_95`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_96`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_97`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_98`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_99`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_100`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_101`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_102`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_103`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_104`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_105`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_106`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_107`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_108`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_109`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_110`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_111`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_112`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_113`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_114`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_115`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_116`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_117`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_118`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_119`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_120`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_121`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_122`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_123`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_124`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_125`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_126`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_127`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_128`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_129`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_130`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_131`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_132`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_133`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_134`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_135`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_136`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_137`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_138`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_139`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_140`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_141`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_142`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_143`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_144`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_145`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_146`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_147`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_148`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_149`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_150`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_151`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_152`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_153`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_154`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_155`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_156`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_157`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_158`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_159`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_160`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_161`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_162`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_163`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_164`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_165`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_166`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_167`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_168`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_169`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_170`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_171`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_172`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_173`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_174`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `four_calls_then_tail_jump`: Calls the same absolute subroutine four times in succession, then jumps to another absolute address instead of returning.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `single_call_then_tail_jump`: Calls one absolute subroutine once, then jumps to a different absolute address rather than returning to the caller.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_9`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_10`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_11`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_12`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
