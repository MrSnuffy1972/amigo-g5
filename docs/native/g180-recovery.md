# Game 180 (Hunter Killer): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00180). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (922240 top-level bytes in 23 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 416 of 416 named (0 author symbols), 416 of 416 described (95 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3367 of 3367 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9671 of 9671 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 104764 |
| code | 32800 |
| data | 292480 |
| padding | 5284 |
| unused | 486912 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| endcli | 2 | 117 | 2 |
| Hunter Killer.adf:vf.obj | 2 | 83 | 3 |
| Hunter Killer.adf:hk | 33 | 5895 | 300 |
| Hunter Killer.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 597 | 16 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2513 | 54 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 147 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 300 | 36 |

## Cases handled

- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 420 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 502 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 147 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 134 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### endcli

- `load_and_run_overlay_with_relocation`: Calls the library through a5 to allocate memory, walks a relocation list to find the highest offset, copies a header and relocation entries into the new block, then calls the library again to load and relocate the overlay before returning zero or minus one.
- `return_d1_in_d0_and_branch`: Moves d1 into d0 as the return value and branches forward into the middle of the caller's following code, so it returns d1 to the caller.

### Hunter Killer.adf:vf.obj

- `decompress_packed_data_to_chip_ram`: Reads a packed bitstream from the data after this routine, expands it byte by byte into the buffer at the chip memory address, writes the source pointer to the color register, and jumps to the buffer when the checksum matches.
- `shift_right_through_carry_with_xor`: Pops a longword from the stack pointed to by a0, XORs it into d5, sets the extend flag via ccr, and rotates d0 right through extend, returning the shifted value in d0.
- `bitstream_shift_loop_refill_from_stack`: Repeats a count in d1: shifts d0 right, refilling from longwords popped via a0 and XORed into d5, accumulating shifted-out bits in d2, then returns.

### Hunter Killer.adf:hk

- `branch_to_main_entry_point`: Unconditionally branches to the main entry point of the program, transferring control without touching any registers or memory.
- `dispatch_command_table_entry`: Saves registers, indexes a command table by the low word of d0, clears stale entries, opens dos.library, allocates memory, then jumps through a computed table entry.
- `open_workbench_window_and_startup`: Clears a global structure, opens the Workbench window, builds a title string from the startup message, and calls initialization routines before returning zero.
- `shutdown_window_and_reply_startup`: Calls an optional callback, closes the Workbench window, replies the startup message, forbids and exits the interrupt, then restores the saved stack and returns.
- `open_dos_library_store_base`: Loads the address of the dos.library name string into a1, sets d0 to zero, calls the library open function via a6, and stores the returned base pointer at offset 956 in a4.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `lookup_and_run_object_method`: Saves registers, looks up an object from the first argument, optionally clears a flag, then calls a method with the object's field and two arguments, returning minus one on failure or if a4 offset 344 is set.
- `buffer_string_bytes_with_flush`: Measures the null-terminated string in a3, then copies each byte into the buffer at a4 offset 98 while decrementing the counter at offset 106, flushing through a call when the counter runs out, and returns the string length.
- `mask_d7_to_byte_and_print`: Copies d7 into d0, masks it to the low byte in d1, pushes the string at offset 94 of a4 and d1, calls a print routine, then moves d0 to d1 and branches back.
- `print_error_and_restore_registers`: Pushes the string at offset 94 of a4 and minus one, calls a print routine, moves d6 to d0, restores d6-d7/a3 from the stack and returns.
- `parse_command_line_and_open_console`: Tokenizes the argument string into up to 32 words, then opens a console window and input/output streams, sets up file handles and jumps to the main game routine.
- `copy_d0_to_d5_then_pad_nops`: Copies the longword in d0 into d5, then executes three no-operation ori.b instructions that leave d0 unchanged before falling through to the next routine.
- `jump_to_absolute_zero_longword`: Performs an unconditional jump through the absolute longword at address zero, transferring control away without saving any registers or returning.
- `allocate_buffer_for_stream_record`: Saves a3, loads the record pointer from the stack, and if its buffer field is empty or flagged, allocates a buffer via a library call, storing it in the record and clearing its offset and length fields.
- `buffered_char_output_with_wrap`: Writes a character through the stream at a3, handling newline expansion, buffer flush and wrap countdown, returning the character or minus one on error.
- `buffered_char_input_with_echo`: Reads the next character from the input buffer at offset 4 of the structure in a3, handling carriage return and end-of-file markers, refilling via the read callback and echoing characters when the echo flag is set.
- `restore_stack_and_chain_to_exit`: Loads the saved stack pointer from offset 384 of a4, calls the cleanup routine, pushes the value 20 and calls the exit routine, then falls through into following code.
- `bounded_strcpy_append`: Appends up to a limit of characters from the second string argument onto the end of the first string, writes a terminating null byte, and returns the first string pointer in d0.
- `strlen_return_length`: Walks the string pointer passed on the stack until a null byte, then returns the character count in d0.
- `strcpy_copy_until_null`: Copies bytes from the source string pointer to the destination pointer one at a time until a null byte is copied, then returns the destination pointer in d0.
- `compare_two_strings_return_zero_if_equal`: Loads two string pointers from the stack, compares bytes one at a time, and returns zero in d0 when both strings end together; otherwise falls through to the mismatch path.
- `return_minus_one_on_string_mismatch`: Branches away when the previous byte comparison was greater, otherwise sets d0 to minus one and returns to signal a string mismatch.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `append_second_string_to_first`: Finds the end of the first string, copies bytes from the second string there including the terminator, and returns the original first pointer in d0.
- `allocate_memory_block_in_free_list`: Rounds the requested size up to a multiple of four with a minimum of eight, walks the free block list at offset 48 of a4, inserts or merges the new block, updates the total allocated counter at offset 52, and returns zero on success or minus one on failure.
- `allocate_memory_block_thunk`: Saves d7 and a3, loads the size and pointer arguments from the stack, pushes them back in swapped order and calls the block allocation routine, then restores registers and returns its result.
- `allocate_and_initialize_memory_block`: Looks up a block by size via the allocator, then calls the initialization routine with the block's size field and the two caller arguments, returning minus one if lookup fails or the error flag at offset 344 of a4 is set.
- `masked_add_to_a4_offset`: ORs an immediate into memory at offset 3677 of a0, calls a subroutine, then adds three to its result, masks off the top bit and stores it at offset 168 of a4.
- `store_stack_arg_to_a4_offset`: Saves d7, loads the longword argument from the stack into d7, stores it at offset 168 of a4, restores d7 and returns.
- `select_and_place_object_slot`: Scans object slots from index three, picks the first free one, resolves direction and flags, calls placement subroutines, stores the chosen index and result into the slot and returns it or minus one.
- `allocate_and_init_resource_node`: Saves d4-d7/a3, reads three stack arguments, calls the node allocator with the first, then calls the initializer with the node's field and the other two arguments; returns minus one on failure or when the global at offset 344 of a4 is set, else the initializer result.
- `allocate_aligned_memory_block`: Rounds the requested size up to a multiple of four with a minimum of eight, walks the free list at offset 48 of a4 to split or reuse a block, updates the free byte count at offset 52, and falls back to expanding the heap before returning the block pointer or zero.
- `allocate_block_via_trampoline`: Saves d7, loads the size argument from the stack into d7, pushes it and jumps to the aligned block allocator, then restores d7 and returns the allocator's result.
- `write_bytes_to_output_stream`: Copies up to the requested count of bytes from the source pointer into the output buffer, decrementing the remaining space and flushing each byte through a callback when the buffer is full, returning the number of bytes written.
- `flush_byte_and_check_error`: Moves the byte in d4 to d0, calls the byte-output routine with it, then increments the returned value and branches if nonzero, otherwise returns the loop counter in d0.
- `decrement_remaining_and_repeat_copy`: Subtracts one from the remaining byte count in d5 and branches back into the byte-copy loop to continue processing.
- `increment_d6_and_branch_back`: Adds one to data register d6 and branches unconditionally back into the preceding routine's code.
- `return_d6_in_d0_restore_regs`: Copies d6 into d0 as the return value, restores d4-d7 and a2-a3 from the stack, releases the a5 frame and returns.
- `seek_stream_with_direction_flag`: Reads a stream object from the stack, adjusts the offset by its step size according to direction and mode flags, resets its counters, calls the seek routine and returns zero or minus one.
- `read_bytes_until_eof_into_buffer`: Copies up to a requested count of bytes from a stream buffer into the destination pointer, refilling via the stream read routine and stopping early on end-of-file, returning the number of bytes stored.
- `find_or_allocate_stream_node`: Walks the linked list at offset 60 of the global structure to the last node, allocates a 34-byte node if none is free, zeroes it, links it in and passes the two arguments to the stream setup routine.
- `open_stream_from_mode_string`: Parses a mode string such as w, r or a with optional b and plus flags, opens the file through the open routine with the matching access flags, initialises the stream fields and returns the stream pointer or zero.
- `check_node_flags_and_run_children`: Saves d6-d7/a2-a3, reads a node pointer from the stack, tests bit 1 of its flags at offset 27, optionally calls a routine with the node and minus one, then calls two routines on its fields and returns zero or minus one.
- `walk_list_and_flush_pending_ranges`: Saves d6-d7/a3, walks a linked list from a4 offset 60, and for each node with bit 2 clear, bit 1 set and a nonzero length, calls a routine with its buffer, offset and length, then calls a final routine with the argument.
- `write_buffer_via_dos_and_record_error`: Saves registers, optionally calls a routine when a flag is set, clears an error field, calls dos.Write with the file handle, buffer and length, and on failure stores dos.IoErr and sets a status of five.
- `seek_file_with_mode_offset`: Saves d2-d7, reads three stack arguments, optionally calls a helper, then calls the dos Seek vector with the file handle, offset and mode, returning the resulting position or -1 on error.
- `read_file_into_buffer`: Saves registers, loads file handle, buffer pointer and length from the stack, calls the dos Read vector, and returns the byte count or -1 while storing the error code.
- `open_file_by_name`: Saves registers, takes a filename pointer and access mode from the stack, calls the dos Open vector, and returns the file handle or -1 after storing the error code.
- `open_file_for_read_with_lock`: Locks the named file with shared lock, opens it for reading, stores the file handle at offset 344 of the a4 base and sets mode 2 at offset 952, returning -1 on failure or the handle.
- `delete_then_reopen_file`: Locks and opens the named file, deletes it via the dos library, then reopens it for reading, storing the new handle at offset 344 and mode 2 at offset 952, returning -1 on failure.
- `close_file_handle`: Takes a file handle argument, optionally calls the cleanup routine when the flag at offset 368 is set, closes the handle through the dos library and returns zero.
- `open_intuition_and_setup_window`: Opens intuition.library once, copies the current string into a local buffer, stores the buffer pointer, then calls two setup routines with window parameters and returns.
- `multiply_32bit_unsigned`: Multiplies the 32-bit values in d0 and d1 using partial 16-bit products, returns the low 32-bit product in d0 and restores d2-d3.
- `signed_32bit_divide_with_sign_fixup`: Divides d0 by d1, negating operands as needed, calls the unsigned division core, then applies the correct sign to the quotient and returns it in d0.
- `unsigned_32bit_divide_by_16bit`: Divides the 32-bit value in d0 by the 16-bit value in d1, normalizing the divisor with shifts and using divu and mulu, returning quotient in d0 and remainder in d1.
- `clamp_task_name_length_to_79`: Saves registers, reads a byte from the task structure pointed to by a4 offset, clamps it to at most 79 in d7, and sets up a1 for a following copy.
- `copy_break_message_and_write_to_dos`: Copies d0 bytes from a0 to a1, terminates the buffer, finds the current task, and writes the break message plus newline via dos.Write, returning -1.
- `open_intuition_and_init_window`: Opens intuition.library once into a4 offset 944, stores a5-relative pointer at 268(a4), then pushes window parameters and calls the setup routine, returning zero on success or minus one.
- `return_zero_restore_registers`: Sets d0 to zero as the return value, restores d2-d3, d6-d7, a2-a3 and a6 from the stack, releases the a5 frame and returns.
- `free_linked_node_and_clear`: Looks up a node from the passed argument, clears its first longword, frees its second longword via a call, and returns minus one on failure or based on a4 offset 344.
- `get_slot_pointer_or_set_error`: Clears the long at offset 344 in a4, validates the index argument against the count at offset 32, and returns a pointer to the eight-byte slot at offset 428 or zero while storing error code 9 at offset 952.
- `check_signals_and_call_poll`: Calls exec SetSignal with mask 12288, keeps only those bits in d7, and if set and the callback at offset 368 returns nonzero, calls the routine at 20 before returning the masked signals.
- `forward_autorequest_arguments`: Loads the IntuitionBase pointer from offset 944 in a4 and jumps through AutoRequest, passing the six stack arguments as requester, window, body, posText, negText and flags.
- `free_marked_resource_entries`: Walks the resource table at offset 428 of a4 from the last index down to zero, calling the free routine for each entry whose first byte is set and whose flag bit 2 is clear, then calls the final cleanup routine with the saved argument.
- `free_all_list_nodes_via_freemem`: Walks the linked list at offset 960 of a4, calling exec FreeMem on each node's memory block, then clears the list head and tail pointers at offsets 960 and 964 of a4.
- `allocate_and_append_list_node`: Allocates a block of the requested size plus twelve bytes with exec AllocMem, links it at the tail of the list at offset 960 of a4, stores the size, and returns a pointer past the twelve-byte header.
- `call_library_offset_minus_552`: Saves d2-d6/a6, loads the library base from offset 956 of a4 into a6, copies stack arguments into registers and calls the library function at offset minus 552, then restores registers and returns.
- `allocate_and_draw_sprite_layers`: Reads a sprite count from the object header, allocates a buffer, fills an array of pointers, then calls the drawing routine several times with position and size arguments before freeing the buffer.
- `init_sprite_frame_pointer_table`: Calls the setup routine with the object's dimensions, computes the frame stride from the width and height words, then stores a pointer to each frame row into the table at offset 8 of a2.
- `setup_stream_and_draw_border`: Initialises the stream at the global pointer with buffer, flags and pen values, then draws a border rectangle and returns zero.
- `clear_and_redraw_object_rect`: Clears the graphics structure, copies the a3 rectangle into stack words, sets flags and color, then draws and fills the rectangle with adjusted coordinates.
- `draw_object_rect_and_return_zero`: Reads the four words at a3 as a rectangle, sign-extends them, pushes them with the graphics structure pointer and calls the rectangle draw routine, then returns zero.
- `setup_sound_channel_from_params`: Copies the current sound index into the channel structure, sets its sample pointer, volume, period and flags, then calls the play and stop helpers and returns zero.
- `stop_channel_and_play_four_word_params`: Stops the sound channel via the stop helper, then if the enable word is nonzero pushes four words from a3 and the channel pointer to the play helper, returning zero.
- `dispatch_sound_command_by_index`: Reads two stack words, and if the second is minus one calls the stop helper with the first; otherwise validates the index against a table and calls the play helper with nibble fields.
- `scale_coords_and_draw_sprite`: Reads four word arguments from the stack, multiplies three by 15 and divides by 1000 via a helper, then pushes the scaled coordinates and a global pointer to a draw routine and returns zero.
- `unpack_nibbles_and_draw_tiles`: Loops over a global count, reads a word from the passed array, splits it into three nibbles, and calls a draw routine with the index and nibble values for each entry.
- `set_palette_and_call_draw`: Loads a palette entry indexed by a global into the graphics structure, calls a draw routine with two word arguments, stores the result, then restores the previous palette entry.
- `blit_region_and_swap_bitmap_buffers`: Copies two source bitmaps into the destination bitmap, allocates a temporary buffer, then fills the destination bitmap's plane pointers with either the new buffer or the saved original depending on a per-column flag table, and dispatches on the mode word.
- `jump_to_mode_one_branch`: Branches unconditionally to the code at the mode-one target, transferring control without touching any registers or memory.
- `jump_to_mode_two_branch`: Branches unconditionally to the code at the mode-two target, transferring control without touching any registers or memory.
- `draw_sprite_with_optional_clip_flag`: Pushes sprite coordinates and a clip flag onto the stack, calls the blit routine, then either passes zero or one as the clip argument depending on the word at the sprite pointer.
- `blit_sprite_and_restore_saved_registers`: Pushes sprite coordinates with a fixed blit flag, calls the blit routine, computes a rounded even byte offset from a2 fields, calls another routine, then restores d2-d7/a2-a3 and returns zero.
- `dispatch_sprite_draw_by_mode_index`: Saves registers, reads sprite bounds from a3, computes width and height, calls two setup routines, then jumps through a table indexed by the mode word to draw the sprite.
- `jump_to_32bc_vector`: Performs an unconditional jump to the fixed target address 32bc, transferring execution without saving registers or returning.
- `jump_through_global_minus_f0`: Loads a code pointer from the global base minus f0 and jumps indirectly to it, transferring control without returning.
- `jump_through_global_minus_156`: Loads a code pointer from the global base minus 156 and jumps indirectly to it, transferring control without returning.
- `jump_through_vector_at_128`: Jumps indirectly to the code address stored at absolute memory location 128, transferring control without saving a return address.
- `jump_through_vector_at_24`: Jumps indirectly to the code address stored at absolute memory location 24, transferring control without saving a return address.
- `jump_through_vector_at_348`: Jumps indirectly to the code address stored at absolute memory location 348, transferring control without saving a return address.
- `jump_via_a4_offset_table`: Jumps to the code pointer stored at offset -552 from the table pointed to by the a4-relative longword at offset 948.
- `jump_via_global_offset_table`: Jumps to the code pointer stored at offset -306 from the base address held in the global variable.
- `jump_via_global_offset_c4`: Jumps to the code pointer stored at offset -336 from the base address held in the global variable.
- `jump_to_110_vector`: Jumps indirectly through the longword stored at address 110, transferring control to the routine whose pointer is held there.
- `jump_to_68_vector`: Jumps indirectly through the longword stored at address 68, transferring control to the routine whose pointer is held there.
- `jump_to_f8_vector`: Jumps indirectly through the longword stored at address 248, transferring control to the routine whose pointer is held there.
- `jump_to_signed_multiply_routine`: Jumps to the routine at absolute address 329c, which computes a signed 32-bit product from the high and low halves of d0 and d1.
- `jump_through_vector_at_global_minus_c6`: Jumps indirectly through the longword pointer stored at the global base minus 198, transferring control to the routine it addresses.
- `jump_through_vector_at_global_minus_13e`: Jumps indirectly through the longword pointer stored at the global base minus 318, transferring control to the routine it addresses.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `zero_filled_data_word_at_10_0070`: This single zero word decodes as an ori.b immediate instruction but is actually zero-filled data referenced by a handler pointer, not executable code.
- `zero_filled_data_word_at_10_0074`: This single zero word decodes as an ori.b immediate instruction but is zero-filled data pointed to by a handler pointer, not executable code.
- `build_path_from_three_strings`: Copies the a0 string into a buffer at offset 678 of the a6 base, appends a slash unless it ends in colon or slash, then appends the a1 string and optional a2 string, returning the buffer in a0.
- `dispatch_io_stdout_hook`: Saves registers, loads the io library base, and calls the stdout hook at offset 24 if set, otherwise opens the library and invokes its open entry.
- `dispatch_io_stdin_hook`: Saves registers, loads the io library base, and calls the stdin hook at offset 28 if set, otherwise opens the library and invokes its open entry.
- `dispatch_io_error_hook`: Saves registers including d3, loads the io library base, and calls the error hook at offset 32 if set, otherwise opens the library and invokes its open entry.
- `get_io_stdout_or_open_library`: Saves registers, loads the library base from pc-relative memory, reads the longword at offset 36 into d0, and if zero opens the library via a jsr to offset -66 before restoring registers and returning.
- `jump_to_address_in_d0`: Moves d0 into a0 and jumps to that address with jsr, then branches back to the preceding instruction; it transfers control to the routine whose address is passed in d0.
- `find_free_slot_and_run_command`: Scans 64 six-byte slots at offset 74 of a6 for an empty entry, opens a command file, matches its name against a table, calls the matched handler, copies the name into the slot and returns the slot pointer.
- `clear_forty_entries_at_a6_offset`: Saves all registers, then loops 64 times over a table at offset 74 from a6, clearing the word at offset 4 of each 6-byte entry and calling a subroutine per entry.
- `decrement_refcount_and_release_object`: Saves registers, decrements the word count at offset 4 of the a0 object, and when it reaches zero calls a release routine, invokes the object's method at offset 8, then clears the pointer and count.
- `find_and_release_matching_slot`: Saves registers, scans four 24-byte slots at offset 466 from a6, and for each active slot whose pointer at offset 4 equals a0 calls a release subroutine.
- `load_form_smus_instrument_bank`: Saves registers and a1, allocates a 1005-byte buffer, clears four instrument slots, then parses a FORM/SMUS file chunk by chunk, dispatching SHD R, INS1, TRAK and SNX chunks to fill the bank.
- `read_sample_rate_and_clamp`: Calls the word reader, compares the returned value against 3601, and if below it clears d0 to zero before branching onward.
- `decode_shdr_sample_header`: Divides a fixed constant by the sample rate in d0, scans a 256-entry table for the matching period index, stores it at offset 2 of a5, then reads volume and up to four sample pointers into the a5 structure.
- `skip_four_bytes_then_jump_back`: Subtracts four from the d6 byte counter and branches unconditionally back to the shared continuation at the earlier address.
- `read_two_words_and_fill_eight_longs`: Calls the byte reader twice storing results at offsets four and six of a5, then reads eight longwords into the buffer at offset thirty-two of a5, decrements d6 by twenty-four and jumps back.
- `parse_ship_record_from_stream`: Reads a type byte, checks the a5 table entry is free, reads a length byte, copies up to twenty-four bytes into the a6 buffer at offset seven hundred eighty-six, then calls the record loader and sets the busy flag on failure.
- `compute_slot_index_from_offset`: Subtracts the address of a6 offset 74 from d0, divides the result by 6, adds one, stores the low byte to (a3), then branches away.
- `parse_sound_command_stream`: Reads a command word stream, decodes opcodes into a5 fields, stores a terminator, and branches to shared exit paths when the slot counter is full or a word is negative.
- `decode_pitch_and_volume_bits`: When d1 equals the pitch opcode, extracts pitch from d0 bits into a5 offset 8 and sets a bit mask into a5 offset 10, then branches on.
- `store_shifted_d0_and_clear_word`: Compares d1 with the immediate value 33792 and branches away if not equal; otherwise masks d0 to seven bits, shifts it left once, stores it at offset 18 of a4, then clears d0 and writes it to the address in a0, advancing a0.
- `load_first_node_from_a6_list`: Loads the longword at offset 880 of a6 into d0, and if nonzero uses it as a0 and adds 128 to a0 before branching onward.
- `store_a5_and_release_list_node`: Stores a5 at the address in a0, checks the longword at offset 858 of a6, calls a subroutine if nonzero, clears that longword, returns a5 in d0 and restores registers from the stack.
- `restore_stack_and_call_5da`: Sets bit 6 of the byte at offset 48 from a6, loads a7 from offset 854 from a6, moves a5 into a0, calls the routine at 5da, clears a5 and branches to 558.
- `call_5a0_with_long_zero`: Loads 4 into d0, pushes a long zero on the stack, calls the routine at 5a0, pops the long into d0 and returns.
- `call_5a0_with_word_zero`: Loads 2 into d0, pushes a word zero on the stack, calls the routine at 5a0, pops the word into d0 and returns.
- `set_d0_flag_and_call_5a0`: Loads 1 into d0, pushes a zero byte on the stack, calls the routine at offset 5a0, then pops that byte into d0 and returns.
- `pass_stack_pointer_to_5ac`: Saves a0 on the stack, loads the address just above the saved a0 into a0, calls the routine at offset 5ac, restores a0 and returns.
- `compare_d0_with_35a_a6_value`: Saves d0 and d1, copies d0 to d1, loads the longword at offset 35a of a6 into d0, calls the routine at offset 112, compares d1 with d0 and returns with d0 and d1 restored.
- `save_regs_and_dispatch_a6_offset_370`: Saves all data and address registers on the stack, loads the pointer at offset 880 from a6 into d0, and if non-zero calls the routine at the following address with that pointer in a0, then loops back.
- `restore_regs_and_return`: Restores all data and address registers from the stack and returns immediately to the caller.
- `release_object_and_unlink_from_list`: Saves registers, clears the active object pointer and counter if a0 matches, frees 64 byte entries and four pointers, then unlinks a0 from the linked list at offset 880 of a6.
- `store_a1_to_a0_and_call_1e9c`: Writes the longword in a1 to the address in a0, moves a5 into a0, calls the routine at offset 1e9c, then restores all registers from the stack and returns.
- `reset_state_and_start_if_d0_zero`: Saves d0-d1, clears the word at offset 1d0 of a6, sets bit 0 at offset 30 of a6, and if d0 is zero sets d0 to one, clears d1 and calls the routine at offset c1c.
- `open_stream_from_resource`: Finds a resource via the lookup call, copies its four longs into the stream at a6, sets up fields and returns after initialising the stream.
- `clear_timer_and_flag_bit`: Clears the longword at offset 368 from a6 and clears bit 0 of the byte at offset 48 from a6, then falls through to the next routine.
- `reset_counter_and_scan_four_slots`: Saves d0 and a0, and if the word at offset 10 from a6 is set, clears it and scans four longword slots from offset 58, calling a subroutine for each nonzero slot.
- `parse_four_stream_headers`: Saves all registers, then for four entries starting at offset 610 from a6 clears fields and walks the word stream pointed to by each object's offset 48, recording counts and a marker byte.
- `increment_d0_store_at_a1_offset_16`: Adds one to the low byte of d0, writes that byte to memory at offset 16 from a1, then branches to another routine.
- `advance_pointers_and_count_four_entries`: Adds four to a0 and a1, increments d7, compares d7 with four and branches back if not equal, otherwise restores all registers and returns.
- `copy_state_and_init_four_slots`: Saves all registers, copies eight longs from a6 offset 610 to 642, copies offset 12 to 16, then loops four times clearing slot fields and setting pointers from a0 and a6 tables before restoring registers.
- `dispatch_audio_tick_and_three_subroutines`: Saves all registers, loads the audio base pointer from a pc-relative long, calls the function pointer at offset 44 if non-zero, then calls three subroutines before restoring registers and returning.
- `sound_channel_timer_and_volume_update`: Decrements the channel timer at offset 50 in a6, recomputes period and volume from the note table, writes volume bytes to the CIA timer registers, then walks four channel slots advancing their sequence pointers.
- `channel_sequence_delay_and_pointer_fetch`: Tests the delay counter at offset 594 in a5, decrements it and, when it reaches zero, loads the sequence pointer at offset 642 into d0 or increments d6 if that pointer is null.
- `parse_sound_command_stream_2`: Reads a word from the stream at d0, stores the advanced pointer at 642(a5), splits opcode and argument, and for opcode 129 looks up a byte in the table at 64(a2) to set the pointer at 562(a5).
- `advance_voice_and_track_position`: For each of four voices, checks the active flag at 58(a5), computes a pitch step from the note table, stores fractional and remainder values at 580(a5) and 596(a5), then advances the track counter and sequence position.
- `compare_track_counter_and_decrement_delay`: Compares d0 with d1, and if equal decrements the word at 10(a6) when non-negative, calling the routine at 774 with a0 set to a2 and incrementing d6 before branching back.
- `flush_pending_sound_request`: Checks the word at offset 10 of the a6 structure and the long at offset 872; if the first is zero and the second nonzero, calls a subroutine then clears that long.
- `signal_sound_task_if_pending`: Saves d0-d1/a0-a1, loads the long at offset 872 of a6; if nonzero, calls exec Signal via the library vector with a6 swapped to the exec base, then restores registers.
- `poll_four_audio_channels`: Loops over four 24-byte channel entries at offset 466 of a6, calling per-channel callbacks, setting bits in d6, and writing the result to DMACON and AUD0LCH-based audio registers.
- `enable_audio_dma_channel`: Clears d0, sets bit 15 in d0, writes d0 to DMACON to enable DMA, then writes value 2 to the audio period register of channel selected by d7.
- `save_regs_call_audio_dma_enable`: Saves d0, d7 and a0 on the stack, copies d0 into d7, calls the audio DMA enable routine, then restores the saved registers and returns.
- `allocate_sound_slot_and_start_sample`: Saves registers, scans four sound slots for a free matching one, stores the sample pointer, volume and priority, marks the slot active and returns its index or minus one.
- `clear_state_and_poll_four_times`: Saves d0, clears the longword at offset 920 and word at offset 10 of a6, then calls a subroutine four times via dbra before restoring d0 and returning.
- `reset_slot_and_mark_pending`: Saves d0 and a1, indexes a 24-byte slot table at offset 466 of a6 using d0, clears the slot's first byte and sets it to 2 if its second byte equals 1.
- `clear_state_and_poll_four_times_alt`: Saves d0, clears the longword at offset 920 and word at offset 10 of a6, then calls a different subroutine four times via dbra before restoring d0 and returning.
- `reset_slot_and_release_object`: Clears the byte and word fields of the 24-byte slot indexed by d0 in the a6 table, and if the slot was active, calls the object's release method through its vtable before clearing the flags.
- `read_slot_word_at_offset_22`: Indexes the 24-byte slot table at a6 by the sign-extended d0, reads the word at offset 22 into d0, sign-extends it to a long and restores a1.
- `compute_scaled_step_from_d0_d1`: Stores the high byte of the a6 word into offset 460, divides the difference between that value and d1 shifted left by eight by d0, and stores the quotient with d1 into offset 462.
- `store_d1_to_a6_offset_zero`: Writes the low word of d1 into the word at offset zero of the a6 structure, then branches to another routine.
- `advance_scroll_position_by_speed`: Multiplies the speed word at a6 offset 464 by the factor at offset 54, scales it, and adds the signed step to the position at offset 460, storing the high byte at offset zero.
- `check_flag_and_clear_bit_zero`: Tests the word at a6 offset 462; if zero, clears bit zero of the byte at a6 offset 48 and, when that bit was set, calls a routine and clears the word at a6 offset 464.
- `clear_active_slot_after_callback`: Saves d1 and a0, indexes a word table at offset 60 from a6 by the byte in d0 times four, and if that word is nonzero calls a routine then clears it.
- `copy_slot_word_from_object_table`: Saves d0 and a0, sign-extends the byte in d0, scales it by four, and copies a word from offset 34 of the object pointer at a6+6 into the slot table at a6+60.
- `add_seventy_eight_to_d4_and_branch`: Adds the immediate value 122 to the low byte of d4 and then branches unconditionally to a shared continuation routine.
- `branch_to_shared_cleanup`: Branches unconditionally to a shared cleanup routine that restores registers and returns to the caller.
- `find_object_with_flag_and_cleanup`: Saves registers, looks up an object by two identifiers, checks a field at offset 470, and restores registers before returning.
- `call_cleanup_clear_a0_branch_back`: Calls the shared cleanup routine, clears a0 to zero, then branches back to the earlier comparison path.
- `decrement_sample_refcount_and_free`: Saves all registers, loads a sample pointer from offset 470 of a0, calls the release routine, and if the sample's reference count at offset 8192 drops to zero calls the release routine again.
- `copy_sound_struct_if_channel_low`: Clears d0, and when d7 is below 4 copies fields from a1 into a2, reads the word at offset 10 of a1 into d0 and zeroes that word before returning.
- `match_and_swap_sample_by_header`: Scans 64 entries at offset 74 of a6 for a sample whose header matches a0, then swaps sample pointers and adjusts reference counts at offset 8192, finally mixing 64 rows of 128 bytes into the sample buffer.
- `render_voice_waveform_to_buffer`: Walks the active voice list, allocates a buffer, then for each of 64 entries computes 128 interpolated sample bytes from the voice table and writes them sequentially through a1.
- `set_d0_bit6_then_branch`: Sets bit 6 of d0 with an or-immediate and branches unconditionally into another routine; it does not return on its own.
- `load_8svx_voice_sample`: Parses a FORM 8SVX file, reads VHDR and BODY chunks, allocates a sample buffer, and stores the computed octave and sample pointer into the voice structure.
- `read_four_byte_stack_value`: Reserves four bytes on the stack, passes that buffer to a subroutine with length four, then pops the resulting longword into d0 and returns.
- `read_two_byte_stack_value`: Reserves two bytes on the stack, passes that buffer to a subroutine with length two, then pops the resulting word into d0 and returns.
- `copy_words_until_count_or_d5_exhausted`: Copies words from the address in a5 to the address in a0 while decrementing d5 and d0 by two, calling a comparison routine when d5 runs out and returning when d0 reaches zero.
- `close_two_audio_channels`: Saves all registers, then calls the channel-close routine twice using a0 and the pointer at offset 58 of a1, and restores registers before returning.
- `open_audio_and_cia_resources`: Allocates memory, opens audio.device, ciaa.resource and dos.library, sets up audio and CIA timer structures, writes CIA timer and interrupt registers, and returns the base pointer.
- `shutdown_audio_if_open`: Saves registers, checks the global audio base pointer, and if nonzero calls three teardown routines plus a cleanup routine before restoring registers and returning.
- `close_device_and_free_task_resources`: Closes the open device and library, frees the signal, finds the task, clears the CIA control register and releases the task and device structures before returning.
- `wait_signal_with_mask_and_restore`: Allocates a signal bit, waits for it combined with the caller's mask, then frees the signal and returns the received mask in d0.
- `store_task_and_signal_mask`: Clears the stored signal mask, saves d0 and d1 into the task structure, calls FindTask and stores the returned task pointer.
- `case_insensitive_string_compare`: Compares two null-terminated strings byte by byte, folding lowercase ASCII letters to uppercase, and returns with d0 zero when they match or nonzero on the first difference.
- `allocate_memory_and_return_pointer`: Saves a0 on the stack, calls the memory allocation routine, moves the returned pointer from a0 into d0, restores a0 and returns.
- `allocate_memory_block_with_size_header`: Adds four to the requested size in d0, checks available memory via exec.AvailMem, allocates with exec.AllocMem, stores the size in the first longword and returns the block pointer in a0.
- `free_mem_block_via_exec_or_hook`: Saves d0-d1/a0-a1, returns immediately if a0 is zero, otherwise frees the block four bytes before a0 through exec FreeMem or a custom hook, then restores registers.
- `wrap_alloc_call_return_a0_in_d0`: Pushes a0, calls the allocation routine at the next address, moves the returned a0 into d0, pops a0 and returns.
- `allocate_and_compare_buffer_blocks`: Saves registers, allocates a block of size 1005, configures it, compares two derived pointers, frees the block on mismatch, and returns the kept pointer in a0.
- `init_level_and_copy_sprite_tables`: Checks stack, saves d7, stores a constant pointer, calls setup routines, copies four word pairs between the sprite tables at the two global arrays, then invokes a display routine and waits for the flag before returning.
- `update_game_state_and_input_flags`: Saves d2-d3/d7, calls several setup routines, tests counters and input bytes to set a flag, clears or copies the position globals, resets ten bytes and clamps a counter to seven, then returns zero.
- `allocate_screen_buffer_via_library`: Opens a stack frame, clears a local word, stores a constant in a global, calls a library function with an offset from a structure, saves the returned pointer and frees it if non-null, returning zero.
- `toggle_active_slot_and_reload_pointers`: Checks stack limit, saves a2-a3, swaps the active slot index, copies slot pointers into the display and object structures, calls two subroutines and returns 1 in d0.
- `init_string_pointer_tables_and_draw_lists`: Stores text pointers into global tables, then loops over rows and columns calling a draw routine for each entry and finally draws the last string.
- `delay_loop_counting_to_125`: Reads a count from the stack, and for each unit runs an inner loop counting d6 up to 125, then returns 1 in d0.
- `point_in_rect_bounds_check`: Pops three arguments from the stack, loads a rectangle pointer into a3 and two coordinates into d7 and d6, compares them against the rectangle edges and returns 1 in d0 if inside, otherwise falls through to return 0.
- `return_zero_restore_registers_2`: Sets d0 to zero as the return value, restores d6, d7 and a3 from the stack and returns to the caller.
- `dispatch_by_short_offset_jump_table`: Builds a stack frame, saves registers, loads a pointer and two shorts from the stack, then uses the second short plus two as an index into a jump table to branch to one of several handlers.
- `branch_to_palette_init`: Unconditionally branches to the palette setup routine, transferring control without touching any registers or memory itself.
- `load_palette_words_and_jump`: Clears d0, stores sixteen palette words into the a5 frame at offsets from minus thirty-four to minus four, then branches to the nibble unpacker.
- `unpack_palette_nibbles_to_frame`: Loops over sixteen entries and three nibbles, shifting bits from a3 and the a5 palette words into the frame arrays at minus one hundred thirty and minus two hundred twenty-six.
- `increment_frame_counter_and_branch`: Adds one to the word counter at offset -232 from a5, then branches unconditionally to another routine.
- `compare_and_adjust_three_column_table`: Loops over sixteen rows and three columns, comparing words at -130(a5) with -226(a5), incrementing or decrementing mismatched entries and setting a flag word at -230(a5).
- `decrement_and_merge_bitmask_columns`: Decrements a word in the -130(a5) table, compares it with the -226(a5) table, clears the flag at -230(a5) and merges bit masks from tables at fixed addresses into the a3 array.
- `increment_counter_and_branch`: Adds one to the word at offset -232 from a5, then branches unconditionally to another routine.
- `check_flag_and_call_update`: Tests the word at -230 from a5; if set, stores one at -228, sign-extends d6 into d0, pushes it, calls a subroutine, pushes a3 and calls another, then branches away.
- `stack_check_and_load_args`: Sets up an a5 frame, checks the stack pointer against a limit, saves registers, loads arguments from the stack into a3, d7, d6, d5 and a2, then compares a global word with six.
- `init_text_cursor_and_check_flag`: Loads the address of a text buffer into a0, stores it at offset -32 from a5, tests the word at 3b10 and branches away if zero, otherwise sets d0 to 2 and branches on.
- `uppercase_and_convert_letter_to_index`: Stores a3 and computes column and row offsets from the word at 3b12 into a5 locals, then uppercases the byte at a0 and converts letters A to Z into an index via a call.
- `convert_digit_char_to_column_offset`: Checks whether d0 holds an ASCII digit, subtracts 48, multiplies by the value at 3b12 through a call, adds the stored column base and saves the result at -16 from a5.
- `clear_word_and_dispatch_char_class`: Clears the word at offset -22 from a5, copies d0 to d1, subtracts 32 and, if the result is between 0 and 32, jumps through a table indexed by that value.
- `compute_letter_span_and_draw_two_glyphs`: Computes a character cell width and end offset from the global spacing value, stores them in the a5 frame, and when d7 equals one draws the glyph twice via the renderer using two different base pointers.
- `return_zero_and_restore_registers`: Sets d0 to zero as the return value, pops d2-d3, d5-d7 and a2-a3 from the stack, releases the a5 frame and returns to the caller.
- `open_file_and_store_handle`: Reads width, height and depth from the parameter block, computes a byte size, calls the open routine with that size and mode, stores the returned handle in the block and global, and returns it.
- `open_file_and_close_previous`: Computes a byte size from the parameter block fields, opens the file, stores the handle globally, closes the previous handle, and returns one.
- `read_into_buffer_with_limits`: Pushes the buffer, count and offset arguments onto the stack, calls the read routine, returns the byte count sign-extended in d0 and compares it with the requested count.
- `jump_through_global_vector_table`: Loads a code pointer from a fixed offset in the global vector table and jumps to it, transferring control without saving a return address.
- `jump_to_sound_and_graphics_entries`: Jumps to the code at the first fixed target, then to the second fixed target, chaining control to two external routines in sequence.
- `jump_to_sprite_draw_entry`: Jumps directly to the code at the fixed target address, transferring control to the sprite or object drawing routine without returning.
- `jump_to_243a_dispatch`: Performs an unconditional jump to the code at address 243a, transferring execution without saving any registers or returning.
- `jump_to_a12_library_vector`: Performs an unconditional jump to the library vector at address a12, handing control to that external routine without touching registers.
- `jump_to_2430_dispatch`: Performs an unconditional jump to the code at address 2430, transferring execution without saving any registers or returning.
- `jump_to_string_compare`: Jumps directly to the string comparison routine at the fixed target, which compares two strings byte by byte and returns a sign result.
- `jump_through_global_offset_table`: Jumps to the code pointer stored at a fixed negative offset from the global base register, transferring control to that routine.
- `jump_to_resource_release_routine`: Jumps directly to the routine that walks resource lists, clears entries and releases allocated blocks referenced through the global base register.
- `jump_to_string_length_loop`: Jumps to the routine at address 9940, which scans a string for its terminating null and returns its length.
- `jump_to_vector_at_negative_d2`: Jumps to the code pointer stored at the global base minus 210, transferring control to that indirect target.
- `jump_to_vector_at_negative_174`: Jumps to the code pointer stored at the global base minus 372, transferring control to that indirect target.
- `jump_to_74_vector`: Performs an unconditional jump to the absolute address 74, transferring execution to the routine stored there without saving a return address.
- `jump_to_2c38_entry`: Performs an unconditional jump to the absolute address 2c38, transferring control to the code at that location without pushing a return address.
- `jump_to_2476_entry`: Performs an unconditional jump to the absolute address 2476, transferring execution to the code at that location without saving a return address.
- `jump_to_244e_dispatch`: Jumps directly to the code at address 244e, transferring execution without touching registers or the stack.
- `jump_to_2462_dispatch`: Jumps directly to the code at address 2462, transferring execution without touching registers or the stack.
- `jump_to_1e10_dispatch`: Jumps directly to the code at address 1e10, transferring execution without touching registers or the stack.
- `jump_to_anim_frame_advance`: Jumps to the animation frame advance routine at a fixed target, transferring control without returning to the caller.
- `jump_to_anim_frame_rewind`: Jumps to the animation frame rewind routine at a fixed target, transferring control without returning to the caller.
- `jump_to_clear_display_list`: Jumps to the display list clearing routine at a fixed target, transferring control without returning to the caller.
- `jump_to_dea_vector`: Jumps to the absolute address dea, transferring execution to the routine located there without saving any registers.
- `jump_to_7c8_vector`: Jumps to the absolute address 7c8, transferring execution to the routine located there without saving any registers.
- `jump_to_2cbc_vector`: Jumps to the absolute address 2cbc, transferring execution to the routine located there without saving any registers.
- `jump_to_absolute_102`: Jumps to the absolute address 102, transferring execution there without saving a return address.
- `jump_to_absolute_246c`: Jumps to the absolute address 246c, transferring execution there without saving a return address.
- `jump_to_absolute_329c`: Jumps to the absolute address 329c, transferring execution there without saving a return address.
- `jump_to_2480_dispatch`: Performs an unconditional jump to the code at address 2480, transferring execution without saving a return address.
- `jump_to_26e8_string_copy`: Performs an unconditional jump to the string copy code at address 26e8, which copies bytes from a source pointer to a destination until a null terminator.
- `jump_to_zero_vector`: Performs an unconditional jump to address zero, transferring execution to the handler stored at the start of memory.
- `set_d0_zero_jump_table_entry`: Loads the immediate value zero into d0 and jumps to the shared table dispatcher at address 36, selecting the first handler slot.
- `set_d0_eight_jump_table_entry`: Loads the immediate value eight into d0 and jumps to the shared table dispatcher at address 36, selecting the second handler slot.
- `set_d0_sixteen_jump_table_entry`: Loads the immediate value sixteen into d0 and jumps to the shared table dispatcher at address 36, selecting the third handler slot.
- `set_d0_24_and_jump_dispatch`: Loads the immediate value 24 into the low word of d0, then jumps to the fixed dispatch address at 36, passing that selector onward.
- `set_d0_32_and_jump_dispatch`: Loads the immediate value 32 into the low word of d0, then jumps to the fixed dispatch address at 36, passing that selector onward.
- `set_d0_40_and_jump_dispatch`: Loads the immediate value 40 into the low word of d0, then jumps to the fixed dispatch address at 36, passing that selector onward.
- `dispatch_handler_index_48`: Loads the word value 48 into d0 and jumps to the shared dispatch routine at absolute address 36, selecting handler slot 48.
- `dispatch_handler_index_56`: Loads the word value 56 into d0 and jumps to the shared dispatch routine at absolute address 36, selecting handler slot 56.
- `dispatch_handler_index_64`: Loads the word value 64 into d0 and jumps to the shared dispatch routine at absolute address 36, selecting handler slot 64.
- `set_d0_72_and_jump_vector`: Loads the immediate value 72 into d0 and jumps to the absolute address stored at memory location 36, transferring control to a system vector.
- `init_four_player_slots_and_state`: Checks the stack against the saved limit, allocates a resource, then fills four ten-byte player slots with formatted names and sets initial game state words and flags.
- `check_stack_and_clear_four_slots`: Compares the stack pointer against a stored limit and returns early if below it; otherwise saves d7, tests a global flag, and if clear calls the slot routine for indices zero through three before returning zero.
- `dispatch_slot_action_by_index`: Checks the stack pointer against a limit, reads a slot index from the stack, looks up a ten-byte slot entry in the slot table, and calls the slot action routine with the entry fields and a computed count.
- `jump_to_string_append`: Jumps to the external string append routine, which finds the end of the first string and copies the second string onto it.
- `jump_to_external_5a_routine`: Jumps directly to the external routine at address 5a, transferring control without saving registers or returning here.
- `jump_to_absolute_154`: Performs an unconditional jump to the absolute address 154, transferring execution without saving a return address.
- `jump_to_absolute_92`: Performs an unconditional jump to the absolute address 92, transferring execution without saving a return address.
- `jump_to_absolute_26e8`: Performs an unconditional jump to the absolute address 26e8, transferring execution without saving a return address.
- `jump_to_zero_address`: Jumps unconditionally to the absolute address zero, transferring control away from this stub without touching any registers or memory.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_378_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_318_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_390_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_570`: Library stub: loads a library base from a global variable and calls the library function at offset minus 570 of that base, returning its result to the caller.
- `call_library_vector_582`: Library stub: loads a library base from a global variable and calls the library function at offset minus 582 of that base, returning its result to the caller.
- `jump_to_handler_at_1b30`: Jumps unconditionally to the handler code at offset 1b30, passing control from this data-referenced entry point without altering registers.
- `jump_to_code_at_1ca8`: Jumps unconditionally to the code at offset 1ca8, an unreached gap entry that transfers control without touching registers or memory.
- `call_aa2_with_three_args`: Saves a6, loads three longword arguments from the stack into a0, d0, d1 and d2, sets a6 from absolute address zero, calls the routine at aa2, restores a6 and returns.
- `call_b72_with_one_arg`: Saves a6, loads one longword argument from the stack into d0, sets a6 from absolute address zero, calls the routine at b72, restores a6 and returns.
- `call_bb0_with_one_arg`: Saves a6, loads one longword argument from the stack into d0, sets a6 from absolute address zero, calls the routine at bb0, restores a6 and returns.
- `save_a6_call_b5c_restore_a6`: Pushes a6 onto the stack, loads a6 from absolute address zero, calls the routine at address b5c, then restores a6 from the stack and returns.
- `load_two_args_call_164_restore_a6`: Pushes a6, loads a0 and a1 from the caller's stack arguments at offsets 8 and 12, sets a6 from address zero, calls the routine at 164, restores a6 and returns.

### Hunter Killer.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `load_two_args_call_fb2c_restore_a6`: Pushes a6, loads a0 and a1 from the caller's stack arguments at offsets 8 and 12, sets a6 from address f988, calls the routine at fb2c, restores a6 and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `main_game_loop_frame_update`: Saves all registers, restores working set from a global save area, points a6 at custom chip base, calls ten subroutines, updates scroll and level counters, polls CIAA fire button and returns zero in d0.
- `vertical_blank_interrupt_acknowledge`: Saves all registers and status register, writes 32 to custom register offset 156 to acknowledge the interrupt, restores status and registers, then returns from exception.
- `clear_memory_and_init_buffers`: Saves registers, sets four global pointers to buffer areas, clears a large block of longwords with a dbra loop, then calls a copy routine with source, destination and size arguments.
- `build_eleven_shifted_sprite_copies`: Saves all registers, then for eleven source/destination pairs calls the shift routine with decreasing offsets from 4200 down to zero, restoring registers and returning.
- `copy_shifted_sprite_words`: Reads words from a0, subtracts d0 from the first two words, writes them and following words to a1, stopping after two consecutive 255 markers.
- `draw_rotated_sprite_segments`: Computes a rotation mask from a1 offset, then loops over a0 segment list, reading a1/a2 coordinate tables and writing sprite control words into a6 offsets.
- `scale_sprite_coordinate_tables`: Saves all registers, reloads them from a saved block, then walks two word tables multiplying entries by 84 or 2 until sentinel values, writing results back in place.
- `init_eleven_sprite_triplets`: Loads three table pointers into a0, a1 and a2 for eleven successive groups and calls the shared setup routine once per group before returning.
- `adjust_sprite_offsets_and_rebuild`: Checks the first word of the a0 table for a marker, subtracts 42 from entries stepping by six words until a sentinel, then calls the rebuild and update routines.
- `decode_script_command_to_a0`: Reads a byte from the script stream at a1, handles the s and a command prefixes by copying words into globals, then looks up a word offset table and copies words into a0 until a terminator, returning via restored registers.
- `project_vertices_through_matrix`: Reads triples of words from a0, multiplies them by the matrix at a3, divides by the transformed depth, adds screen offsets, and writes x and y words to a1 and a2 until a terminator, saving a0.
- `build_rotation_matrix_from_angles`: Loads three angle indices from a2, looks up sine and cosine words, and fills the matrix fields at offsets 24 through 56 of a3 with scaled products and a depth divisor.
- `toggle_and_install_copper_list`: Saves all registers, flips bit one of a flag to pick one of two copper lists, copies the chosen list pointer into the copper pointer, sets display and control registers, then advances three scroll counters modulo the screen width.
- `save_old_interrupt_vectors_install_new`: Copies two words from the a6 structure to globals, sets interrupt enable masks, saves the old level 3 vector, installs a new vector, sets the audio interrupt mask and a new copper list pointer, and clears a counter.
- `restore_interrupt_vectors_and_masks`: Restores the saved level 3 interrupt vector, sets the interrupt enable words from the saved globals with the high bit set, and writes the audio interrupt mask back to the a6 structure.
- `copy_planar_words_with_offsets`: Loops d0 times, writing d2 and d2+2 as offsets into (a1) while copying a longword from (a0) split into two words, advancing d2 by four each pass.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_game_state_and_screens`: Clears several game arrays and globals, sets up display and sprite tables, allocates a bitmap via library calls, and stores the result in a global.
- `load_and_blit_title_bitmap`: Copies a parameter block, opens a bitmap, blits it in two strips to screen memory, then frees the bitmap and returns zero.
- `draw_two_player_weapon_sprites`: Loops twice, sets up a small coordinate table, writes sprite control words to chip memory and calls blitter routines to draw player weapon graphics for the given player index.
- `animate_weapon_bits_toward_target`: Extracts three 3-bit fields from two words, steps each field toward its target one at a time, updates the sprite data word and redraws it until all fields match.
- `draw_five_direction_weapon_frames`: Loops over five direction cases, computes sprite coordinates from direction offsets, draws weapon rectangles and blits sprite frames into the display for each direction.
- `clip_negative_x_rect_and_blit_two_tiles`: Reads two word arguments and a rectangle pointer, computes clipped horizontal bounds against the screen width, and if the rectangle is valid issues two blits with tile numbers 13 and 14.
- `clip_positive_x_rect_and_blit_tile_five`: Reads two word arguments and a rectangle pointer, sets blit tile 5 and a zero flag, computes clipped horizontal bounds, and if the rectangle is valid issues one blit.
- `clip_vertical_rect_and_blit_six_tiles`: Reads two word arguments, computes a clipped vertical rectangle against the screen height, blits it, then loops over six tile entries and blits each matching one.
- `init_playfield_and_draw_scene`: Checks the stack pointer against a limit, saves d2, sets up playfield scroll and window registers from the display dimensions, then calls the draw routine with the screen parameters and returns zero in d0.
- `set_scroll_window_and_clear_flags`: Builds a four-word parameter block on the stack, writes display control flags to the hardware registers, then calls the setup routine with the current screen value and returns zero in d0.
- `advance_scroll_index_and_update_palette`: Adds the scroll step to the current index, wraps it between zero and fifteen, then calls the palette routine with the indexed colour value and the screen base, returning zero in d0.
- `select_level_and_init_play_state`: Reads the level number argument, looks up its entry in the level pointer table, stores the level pointer, marks the level active, copies the level data pointer to the current-level slot, formats the level name and counters, then calls the level setup routine and returns zero.
- `decode_rle_bitmap_to_screen`: Reads a packed bitmap descriptor, decodes its run-length bit stream into the destination buffer using shift and mask operations on words, tracks bit position and row counters, and returns zero when the stream is exhausted.
- `reset_level_tables_and_clear_names`: Formats two fixed text strings, then loops over twenty-five level slots clearing the first byte of each name buffer, storing each name pointer in the level table and formatting five numeric fields per entry before returning zero.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_to_ee9a_trampoline`: Jumps unconditionally to the code at address ee9a, transferring control away from this two-instruction stub.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_9`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_10`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_11`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_12`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_13`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_14`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_15`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_to_segment_entry`: Transfers control unconditionally to the entry point at the start of the runtime segment, acting as a one-instruction tail jump with no register changes.
- `jump_vector_entry_16`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_17`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_18`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_19`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_20`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_21`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_22`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_23`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_24`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_25`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_26`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_27`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_28`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_29`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_30`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_31`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_32`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_33`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_34`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_35`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_36`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_37`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_38`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_39`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_to_5348_after_stack_check`: Jumps to address 5348, then links a5 with a negative frame, compares a7 against memory at 1cdd4 and branches below to 4df0, saving d2 and d4-d7.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_game_resource_tables`: Checks a stack guard, then fills a series of global resource descriptors with sizes, counts and pointers, returning one on success.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `allocate_and_setup_display_buffers`: Opens two libraries, allocates two bitmaps and their planes, creates two rasters, sets up copper lists and copies palette words into the display structure, returning one.
- `release_display_and_sound_resources`: Frees the display structure, closes libraries, releases allocated bitmaps and sound buffers, and clears the resource pointers before returning one.
- `reset_and_free_game_structures`: Calls setup routines, stores computed sizes into globals, frees two arrays of six ten-byte entries, then iterates seventeen word flags freeing associated pointers and returns one.
- `free_active_flag_pointers`: Loops over seventeen word flags, and for each nonzero flag copies the matching long pointer to a global buffer and frees it, then returns.
- `free_sound_and_cleanup_buffers`: Frees a global buffer if present, calls two cleanup routines, frees two more global pointers if nonzero, sets d0 to one, pops d7 and returns.
- `allocate_9e0e_block_and_store_pointer`: Checks the stack against the saved limit, calls a subroutine to allocate memory, stores the returned pointer at a fixed global, writes the value 9e0e into offset 32 of that block, loads the address of a nearby table into a0, stores it at offset 9e0a, and returns zero in d0.
- `dispatch_index_from_9e06_table`: Checks the stack against the saved limit, loads the word at a fixed global, subtracts two, rejects values below zero or at least ten, scales the remainder by four and jumps through a nearby table indexed by that value.
- `branch_to_shared_dispatch_target`: Unconditionally branches forward to a shared target address, transferring control without touching any registers or memory.
- `print_message_and_branch_to_578`: Pushes the longword at memory location 9e06 and a fixed string pointer, calls the print routine, cleans the stack, prints a second fixed string, then branches to the code at 578.
- `print_message_call_23e_and_clear_long`: Prints a fixed string via the print routine, calls the routine at 23e, pushes a zero longword and calls another routine, then returns to its caller.
- `jump_to_1d144_then_67a0`: Immediately jumps to the code at 1d144, which then jumps to the code at 67a0; no registers or memory are touched here.
- `jump_vector_entry_40`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_41`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_42`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_43`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_44`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_45`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_46`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_47`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_48`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_49`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_50`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_51`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_52`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_53`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_54`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_55`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_56`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_57`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_58`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_59`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_60`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_61`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_62`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_63`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_64`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
