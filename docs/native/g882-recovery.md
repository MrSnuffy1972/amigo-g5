# Game 882 (hunter killer (1989)(mastertronic)[cr vf]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00882). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

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
| Hunter Killer (1989)(Mastertronic)[cr VF].adf:vf.obj | 2 | 83 | 3 |
| Hunter Killer (1989)(Mastertronic)[cr VF].adf:hk | 33 | 5895 | 300 |
| Hunter Killer (1989)(Mastertronic)[cr VF].adf:bootblock | 1 | 10 | 1 |
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

- `load_and_run_overlay_with_relocs`: Reads a relocated overlay from disk into memory, patches its jump table, copies a parameter block, and jumps to the loaded code, returning zero or minus one.
- `return_d1_as_d0_then_branch`: Moves d1 into d0 as the return value and branches to a fixed offset inside the caller's code.

### Hunter Killer (1989)(Mastertronic)[cr VF].adf:vf.obj

- `decompress_and_jump_payload`: Decodes a packed bitstream into memory at the payload address, flashes the color register while waiting, then jumps to the decompressed code.
- `xor_shift_right_through_extend`: Pops a longword from the stack pointed to by a0, XORs it into d5, then rotates d0 right by one bit through the extend flag and returns.
- `shift_bits_refilling_from_stack`: Shifts d0 right by d1 minus one bits, refilling from longwords popped off the a0 stack and XORing each into d5, accumulating shifted-out bits in d2, then returns.

### Hunter Killer (1989)(Mastertronic)[cr VF].adf:hk

- `branch_to_main_entry_point`: Unconditionally branches forward to the main entry point of the program, transferring control without touching any registers or memory.
- `dispatch_script_command_table`: Saves registers, indexes a command table by the low word of d0, clears stale entries, opens dos.library, allocates memory and jumps to the selected command handler.
- `open_workbench_startup_window`: Clears a global block, saves the exec base and stack, opens the Workbench startup message port, builds a command line and calls the startup routines.
- `shutdown_and_reply_startup_msg`: Calls an optional cleanup vector, closes the startup library, forbids and replies to the startup message, then restores the saved stack and returns.
- `open_dos_library_store_base`: Loads the address of the dos.library name string into a1, sets d0 to zero, calls the library open function via a6, and stores the returned base pointer at offset 956 in a4.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `lookup_and_run_file_entry`: Saves registers, looks up a structure from the first argument, optionally clears a flag, calls a routine with the structure's field and two arguments, and returns minus one if a4 offset 344 is set.
- `write_string_to_output_buffer`: Measures the null-terminated string in a3, then copies each byte into the buffer at a4 offset 98 while decrementing the counter at offset 106, flushing when it runs out, and returns the string length.
- `convert_d7_byte_to_string_via_jsr`: Copies d7 into d0, masks it to a byte in d1, pushes a pointer from a4 offset and d1, calls a subroutine, then moves the result to d1 and branches away.
- `push_two_args_call_then_return_d6`: Pushes a pointer from a4 offset and minus one as arguments, calls a subroutine, cleans the stack, moves d6 to d0, restores d6-d7/a3 and returns.
- `parse_command_line_into_argv`: Splits the command line string into up to 32 whitespace-separated tokens, storing pointers in the argument table and handling quoted strings, then opens the console window and sets up file handles.
- `copy_d0_to_d5_then_pad_with_ori`: Copies d0 into d5, then executes three ori.b instructions with zero immediates that leave d0 unchanged before falling through to the next routine.
- `jump_to_absolute_zero_vector`: Performs an unconditional jump to absolute address zero, transferring control away without saving registers or returning to the caller.
- `allocate_buffer_for_object_if_needed`: Checks fields at offsets 20 and 27 of the object in a3; if unset or flagged, calls the allocator with the global size, stores the pointer at offsets 4 and 16, clears offsets 8 and 12, masks flags, and returns zero or minus one on failure.
- `buffered_char_output_with_line_wrap`: Writes a character to the stream buffer at offset 4 of the structure in a3, handling newline expansion, line wrap counting and flushing through the stream write routine, returning the character or minus one.
- `buffered_char_input_with_echo`: Reads the next character from the input buffer at offset 4 of the structure in a3, refilling via the read routine, handling carriage return and end-of-file, returning the character or minus one.
- `restore_stack_and_chain_exit`: Loads the stack pointer from offset 384 of a4, calls the exit routine, pushes 20 and calls another routine, then loads 97 into d0.
- `bounded_string_append`: Finds the end of the destination string, copies up to a maximum count of bytes from the source string after it, writes a terminating null byte and returns the destination pointer.
- `string_length_in_d0`: Scans the byte string passed on the stack until a null byte, computes its length and returns that count in d0.
- `copy_string_return_destination`: Copies bytes from the source string pointer to the destination pointer until a null byte is copied, then returns the destination pointer in d0.
- `compare_two_strings_until_mismatch`: Loads two string pointers from the stack, compares bytes one at a time, and returns zero in d0 only if both strings ended together with no mismatch.
- `return_string_compare_result`: Branches on the condition codes left by the previous byte comparison, returning minus one in d0 when the first string is less than the second.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `append_second_string_to_first`: Scans a0 to its terminating zero, then copies bytes from a1 including the terminator into that position, returning the original a0 pointer in d0.
- `allocate_memory_block_from_free_list`: Rounds the requested size up to a multiple of four, walks the free block list at offset 48 of a4, inserts or merges the new block, updates the total allocated counter at 52(a4), and returns zero or minus one.
- `allocate_memory_block_thunk`: Pushes the size and pointer arguments onto the stack, calls the block allocation routine at 2768, cleans up eight bytes of arguments and returns its result.
- `allocate_and_initialize_memory_block`: Looks up a block by the first argument, then calls the initialization routine with the block's field at offset four plus the two remaining arguments, returning minus one if the global at 344(a4) is set.
- `mask_and_store_result_at_a4_offset`: ORs an immediate into memory at offset 3677 of a0, calls a nearby routine, then adds 3 to its result, masks off the top bit and stores it at offset 168 of a4.
- `store_stack_argument_to_a4_offset`: Saves d7 on the stack, loads the caller's first argument from offset 8 of the stack into d7, stores it at offset 168 of a4, restores d7 and returns.
- `select_and_place_game_object_slot`: Scans object slots from index 3, picks the first free one, resolves direction and flags from the arguments, calls placement routines, stores the new state and index into the slot and returns the slot index or minus one.
- `allocate_node_and_call_second_stage`: Saves d4-d7/a3, calls an allocator with the first argument, then calls a second routine with the node's field, the second and third arguments, returning minus one on failure or if a global flag at offset 344 of a4 is set.
- `allocate_aligned_block_from_free_list`: Rounds the requested size up to a multiple of four with a minimum of eight, walks the free list at offset 48 of a4 splitting or removing blocks, and falls back to growing the heap before returning the block pointer or zero.
- `allocate_block_via_size_wrapper`: Pushes d7, loads the size argument from the stack into d7, calls the block allocator with it, then restores d7 and returns the allocator's result in d0.
- `buffered_byte_writer`: Writes up to a count of bytes from a source pointer into a buffered output structure, flushing each byte through a callback when the buffer count runs out, and returns the number of bytes written.
- `flush_byte_via_callback`: Moves the byte in d4 to d0, calls the output callback with a2 and the byte on the stack, then returns the callback result plus one in d1, or d6 in d0 if it was zero.
- `decrement_d5_and_rejoin_writer`: Subtracts one from d5 and branches back into the buffered byte writer loop to continue writing the remaining bytes.
- `increment_d6_then_branch_back`: Adds one to the longword in d6 and then branches unconditionally back to an earlier address, continuing execution elsewhere.
- `return_d6_in_d0_and_restore_registers`: Copies the longword in d6 to d0 as the return value, restores d4 through d7 and a2 through a3 from the stack, releases the a5 frame and returns.
- `update_object_position_and_flags`: Saves d6, d7 and a3, reads three arguments from the stack, adjusts the position by the object's speed depending on direction and mode flags, clears counters, calls two subroutines and returns zero or minus one.
- `read_stream_bytes_into_buffer`: Reads up to a requested count of bytes from a stream structure into a destination buffer, calling the stream refill routine when the buffer is empty, and returns the number of bytes stored or the index where a negative byte was seen.
- `find_or_allocate_stream_node`: Walks the linked list at offset 60 of the global structure to find a free node, allocates a 34-byte node if none is free, clears it, and passes the caller's two arguments to the stream setup routine.
- `open_stream_from_command_string`: Parses a command string's first letters to choose access mode flags, calls the open routine with those flags, then initializes the stream structure fields and returns the stream pointer or zero on failure.
- `check_node_flags_and_run_children`: Saves d6-d7/a2-a3, reads a node pointer from the stack, tests flag bit 1 and optionally calls a routine with -1, then runs the node's child routine and returns zero or -1 based on results.
- `walk_list_and_flush_pending_entries`: Saves d6-d7/a3, walks a linked list from a4 offset 60, and for each entry with flag bit 2 clear, bit 1 set and nonzero size, calls a routine with its fields, then calls another routine with the argument.
- `write_buffer_via_dos_and_record_error`: Saves registers, optionally calls a routine, clears a4 offset 344, calls dos.Write with the given file, buffer and length, and on failure stores dos.IoErr and sets a4 offset 952 to 5.
- `seek_file_with_mode_offset`: Saves d2-d7, reads three stack arguments, optionally calls the error reset, then calls dos.Seek with the file handle and mode, storing IoErr and error code 22 on failure, and returns the resulting offset.
- `read_file_into_buffer`: Saves registers, reads file handle, buffer and length from the stack, optionally calls the error reset, then calls dos.Read, storing IoErr and error code 5 on failure, and returns the byte count.
- `open_file_by_name`: Saves registers, reads filename and access mode from the stack, optionally calls the error reset, then calls dos.Open, storing IoErr and error code 2 on failure, and returns the file handle or minus one.
- `open_file_for_read_and_store_handle`: Locks and opens the named file for reading via dos.library, stores the returned handle in the global at offset 344, sets mode 2 at offset 952 and returns -1, or returns the error code.
- `delete_then_open_file_for_read`: Locks the named file, deletes it with dos.DeleteFile, then opens it for reading, storing the handle at offset 344 and mode 2 at offset 952, returning -1 or the error.
- `close_file_handle_from_argument`: Loads the file handle passed on the stack into d7, calls dos.Close on it through the dos library base, and returns zero in d0.
- `open_intuition_and_setup_text_buffer`: Opens intuition.library once into global storage, copies the string pointed to by the global text pointer into an 80-byte stack buffer with a terminator, stores the buffer pointer globally, then calls two setup routines with window and screen parameters.
- `multiply_32bit_unsigned`: Multiplies the 32-bit values in d0 and d1 using partial 16-bit products, accumulating cross terms into the high word and low products into the low word, returning the 32-bit product in d0 while preserving d2 and d3.
- `signed_32bit_divide_with_sign_fixup`: Computes d0 divided by d1 with sign handling: negates negative operands, calls the unsigned division core, then negates the quotient according to the original operand signs and returns it in d0.
- `unsigned_32bit_divide_by_16bit`: Divides the 32-bit value in d0 by the 16-bit value in d1, using a fast path when the divisor's high word is zero and a shift-normalized long division otherwise, returning quotient in d0 and remainder in d1.
- `clamp_task_name_length_to_79`: Saves registers, reads the byte before the task name pointer at offset 392 of a4 into d7, clamps it to at most 79, and sets a1 to a stack buffer for the caller.
- `copy_name_and_write_break_message`: Copies d0 bytes from a0 to a1, terminates the buffer, finds the current task, and writes the task name plus a break message to dos output before returning minus one.
- `open_intuition_and_init_windows`: Checks the cached intuition library base, opens intuition.library if absent, stores the base, sets a pointer from a5, then calls a setup routine with several window parameters and returns zero on success or minus one.
- `return_zero_restore_registers`: Sets d0 to zero as the return value, restores d2-d3, d6-d7, a2-a3 and a6 from the stack, releases the a5 frame and returns to the caller.
- `close_linked_list_node`: Looks up a node by the passed key, and if found clears its first longword, optionally frees its second longword via a call, and returns zero or minus one depending on a global flag.
- `get_slot_pointer_or_set_error`: Clears the longword at offset 344 in a4, validates the index argument against zero and the count at offset 32, and returns a pointer to the eight-byte slot at offset 428 or zero while storing error code 9 at offset 952.
- `poll_signals_and_run_callback`: Calls exec SetSignal for mask 12288, keeps the returned bits in d7, and if any are set and the callback pointer at offset 368 in a4 is non-null, calls it and then invokes the routine at 20 when it returns nonzero.
- `forward_args_to_autorequest`: Loads the IntuitionBase pointer from offset 944 in a4 into a6, copies six arguments from the stack into a0-a3 and d0-d2, and jumps to the AutoRequest library vector before restoring registers.
- `flush_pending_sound_entries`: Walks the sound entry table backwards from the count at offset 32 of a4, calling the routine at each active entry's second field unless its flag byte is zero or bit 2 is set, then calls the routine with the passed argument.
- `free_sound_node_list`: Walks the linked list at offset 960 of a4, freeing each node's memory block via exec FreeMem, then clears the list head and tail pointers at offsets 960 and 964.
- `allocate_sound_node`: Allocates a block of the requested size plus twelve bytes via exec AllocMem, links it into the list at offsets 960 and 964 of a4, stores the size, and returns a pointer past the twelve-byte header.
- `call_library_offset_minus_552`: Saves d2-d6/a6, loads library base from a4 offset, passes stack arguments to a library function at offset minus 552, then restores registers and returns.
- `allocate_and_draw_sprite_layers`: Allocates a memory block, fills a pointer table, and calls drawing routines to render sprite layers with given coordinates and flags, returning zero.
- `build_row_pointer_table`: Calls a setup routine with dimensions, then fills an array with pointers to successive rows of a bitmap, each row offset by width times height.
- `setup_sprite_from_globals`: Builds a sprite structure from global counters, sets its control words and flags, then calls the sprite setup and attach routines with the sprite pointer.
- `configure_sprite_rect_and_attach`: Fills a stack rectangle from the a3 coordinate words, sets sprite control flags, calls the position and rectangle routines, then attaches the sprite.
- `attach_sprite_from_a3_rect`: Reads four coordinate words from a3, sign-extends them, pushes them with the sprite pointer and calls the rectangle attach routine, returning zero.
- `setup_sound_channel_from_slot`: Reads a slot index from global word, calls two sound routines with the channel pointer, sets channel fields including sample pointer, volume, period and flags, then optionally triggers playback.
- `stop_channel_and_play_four_word_params`: Calls the sound stop routine with the channel pointer, then if the enable word is nonzero pushes four words from a3 and calls the play routine, returning zero.
- `set_channel_volume_or_stop`: Takes a channel index and a packed value; if value is minus one it stops the channel, otherwise validates the index and splits the value into three nibbles passed to the sound routine.
- `scale_three_coords_and_draw_sprite`: Reads four word arguments from the stack, scales three of them by fifteen and one thousand, stores results in local words, then pushes them with a global pointer to a drawing routine and returns zero.
- `unpack_nibbles_and_draw_tiles`: Loops over a global word count, reads packed words from an array, extracts three nibbles per word, and passes index and nibble values to a drawing routine for each entry.
- `set_palette_and_call_draw`: Loads a long from a palette table indexed by a global word into a graphics structure, calls a drawing routine with two word arguments, stores its result, then restores another palette entry.
- `blit_sprite_into_playfield_buffers`: Copies the source bitmap into the destination bitmap, allocates a temporary buffer, then fills the destination's per-column pointers with either the new buffer or the old one depending on a table flag, and dispatches on the mode word.
- `jump_to_mode_one_branch`: Branches unconditionally to the code at the mode-one target, transferring control without touching any registers or memory.
- `jump_to_mode_two_branch`: Branches unconditionally to the code at the mode-two target, transferring control without touching any registers or memory.
- `draw_sprite_with_optional_clip_flag`: Reads sprite fields from a3, formats them with the global format string, then calls the draw routine with a zero or one clip flag depending on the word at the object pointed to by 28(a5).
- `draw_sprite_then_blit_and_restore`: Formats sprite coordinates from a3 and d5 into the global buffer, draws with flag 96, computes a rounded even blit size from a2 offsets, blits it, restores registers and returns zero.
- `dispatch_sprite_draw_by_mode`: Saves registers, copies rectangle bounds from a3 into locals, formats two global strings, then jumps through a table indexed by the mode word to draw the sprite with a mode-specific flag.
- `jump_to_signed_divide_routine`: Jumps to the shared signed division routine at a fixed absolute address, passing the caller's d0 and d1 operands through unchanged.
- `jump_to_low_vector_table_entry`: Jumps to a fixed low absolute address, transferring control to the code or vector entry located there without touching registers.
- `jump_to_high_vector_table_entry`: Jumps to a fixed higher absolute address, handing control to the routine or vector entry stored there with registers unchanged.
- `jump_through_vector_at_128`: Jumps indirectly to the code address stored at absolute memory location 128, transferring control without saving a return address.
- `jump_through_vector_at_24`: Jumps indirectly to the code address stored at absolute memory location 24, transferring control without saving a return address.
- `jump_through_vector_at_348`: Jumps indirectly to the code address stored at absolute memory location 348, transferring control without saving a return address.
- `jump_via_a4_offset_table`: Jumps to the code pointer stored at offset 948 from the a4 base register, transferring control to that routine.
- `jump_via_global_offset_table`: Jumps to the code pointer stored at a fixed global offset, transferring control to the routine referenced there.
- `jump_via_global_second_offset`: Jumps to the code pointer stored at a different fixed global offset, transferring control to the routine referenced there.
- `jump_to_vector_110`: Jumps to the absolute address stored at memory location 272, transferring control to the routine whose pointer lives there.
- `jump_to_vector_104`: Jumps to the absolute address stored at memory location 104, transferring control to the routine whose pointer lives there.
- `jump_to_vector_248`: Jumps to the absolute address stored at memory location 248, transferring control to the routine whose pointer lives there.
- `jump_to_329c_signed_multiply`: Jumps to the routine at address 329c, which computes a signed 32-bit product from the high and low words of d0 and d1.
- `jump_to_zero_vector_dispatch`: Jumps to address zero, which dispatches through a pointer stored at a fixed negative offset from the global base.
- `jump_to_a8_vector_dispatch`: Jumps to address a8, which dispatches through a pointer stored at a fixed negative offset from the global base.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `zero_filled_data_word_at_0070`: A single zero word that decodes as ori.b #0,d0; this is zero-filled data referenced by pointers, not executable code.
- `zero_filled_data_word_at_0074`: Another single zero word decoding as ori.b #0,d0; part of the same zero-filled data area pointed to by handler pointers.
- `build_path_from_three_parts`: Copies a drive or directory string, appends a slash if needed, then appends a filename and optional extension into the buffer at offset 678 of the base structure, returning that buffer in a0.
- `dispatch_or_open_library_offset24`: Saves d1-d2/a0-a1/a6, stores a0 and d0, loads the library base from pc-relative memory and calls the function pointer at offset 24, or opens the library via the -30 vector if null, then restores registers and returns.
- `dispatch_or_close_library_offset28`: Saves d1/a0-a1/a6, keeps d0 in d1, loads the library base and calls the function pointer at offset 28, or invokes the -36 vector when null, then restores registers and returns.
- `dispatch_or_expunge_library_offset32`: Saves d1-d3/a0-a1/a6, preserves d1, d0 and a0 in d3/d1/d2, loads the library base and calls the pointer at offset 32, or the -42 vector if null, then restores registers and returns.
- `check_and_call_allocator`: Saves registers, loads a library base from pc-relative memory, tests the longword at offset 36 of that base, and if zero calls a function at offset -66 of a second library base before restoring registers and returning.
- `jump_to_address_in_d0`: Moves the value in d0 into a0 and jumps to that address, then branches back to an earlier instruction after the call returns.
- `scan_slots_and_dispatch_command`: Saves registers, scans 64 six-byte slots at offset 74 of a6 for a free entry, allocates a block, matches a command string against a table, calls the matched handler, copies the string into the block and returns the slot pointer.
- `clear_sixty_four_slot_fields`: Saves all registers, then loops 64 times over 6-byte records starting at offset 74 from a6, clearing the word at offset 4 of each record and calling a subroutine.
- `decrement_refcount_and_release_object`: Saves registers, decrements the word count at offset 4 of the object in a0, and when it reaches zero calls the object's release routine and clears its pointer and count.
- `find_and_release_matching_slot`: Saves registers, scans four 24-byte entries at offset 466 from a6, and for each active entry whose pointer at offset 4 matches a0 calls a release subroutine.
- `load_form_smus_instrument_bank`: Saves registers, allocates a buffer, reads FORM/SMUS chunks and dispatches SHD R, INS1, TRAK and SNX records into the a5 instrument structure.
- `read_sample_rate_check_limit`: Calls the word reader, compares the returned value against the sample-rate limit and clears d0 when it is too large, then branches onward.
- `decode_shdr_sample_header`: Divides the sample rate constant by d0, scans the period table for the matching entry, stores period and volume in the a5 header and fills the sample flag words.
- `skip_four_bytes_and_branch_back`: Subtracts four from the d6 byte counter and branches unconditionally back into the shared caller loop.
- `read_two_words_and_fill_pointer_array`: Calls the byte reader twice storing words at offsets four and six of a5, then reads four longwords into the array at offset thirty-two of a5, decrements d6 by twenty-four and branches back.
- `parse_ship_record_from_input_stream`: Reads a type byte below sixty-four, checks the a5 table entry is clear, reads a second byte, then fills the twenty-four byte buffer at offset seven hundred eighty-six of a6 and calls the record lookup, setting bit seven of a6 offset forty-eight on failure.
- `compute_slot_index_from_offset`: Subtracts the address of the a6-relative table at offset 74 from d0, divides by six, adds one, stores the byte result through a3 and branches away.
- `load_sound_slot_and_parse_commands`: Increments the a6 counter at offset 866, allocates a slot in the a5 table, loads a sound block, then scans its words for command codes and stores parsed values into the a5 structure.
- `decode_volume_and_channel_bits`: When d1 equals the volume command, extracts a five-bit volume plus one into a5 offset 8 and sets a channel bit from the low three bits of d0 into a5 offset 10.
- `set_sound_channel_period_and_clear_word`: Compares d1 with the value 33792; if equal, masks d0 to seven bits, shifts it left once, stores it at offset 18 from a4, then clears d0 and writes it to the address in a0 before branching away.
- `load_first_list_node_plus_128`: Loads the longword at offset 880 from a6 into d0; if nonzero, moves it into a0 and adds 128 to a0, then branches onward.
- `store_a5_and_release_sound_node`: Stores a5 at the address in a0, checks the longword at offset 858 from a6, calls a subroutine if nonzero, clears that longword, returns a5 in d0 and restores registers.
- `restore_stack_from_a6_and_chain`: Sets bit 6 of the byte at offset 48 from a6, loads a7 from offset 854 of a6, moves a5 into a0, calls the routine at 1498, clears a5 and branches to 1368.
- `call_long_alloc_with_size_four`: Loads 4 into d0, pushes a zero longword on the stack, calls the routine at 1440, pops the result into d0 and returns it.
- `call_word_alloc_with_size_two`: Loads 2 into d0, pushes a zero word on the stack, calls the routine at 1440, pops the result into d0 and returns it.
- `set_d0_flag_and_call_5a0`: Loads d0 with 1, pushes a zero byte on the stack, calls the routine at offset 5a0, then pops that byte into d0 and returns.
- `call_5ac_with_stack_pointer_argument`: Saves a0, loads a0 with the address of the stack slot above the return address, calls the routine at offset 5ac, restores a0 and returns.
- `compare_d0_with_35a_a6_value`: Saves d0 and d1, copies d0 to d1, loads d0 from offset 35a of a6, calls the routine at offset 112, compares d1 with d0 and returns if equal.
- `save_all_registers_and_poll_370`: Pushes every data and address register onto the stack, loads the longword at offset 880 from a6 into d0, and if nonzero calls the routine at the following address before looping back to the push.
- `restore_all_registers_and_return`: Pops all data and address registers from the stack and returns to the caller, restoring the full register set saved by the matching save routine.
- `release_object_and_unlink_from_list`: Saves all registers, then for the object in a0 clears the current-object pointer and word at offset 50 in a6, frees up to 64 byte entries, releases four longwords, and unlinks the object from the list at offset 880 in a6.
- `store_a1_to_a0_and_call_1e9c`: Writes the longword in a1 to the address in a0, moves a5 into a0, calls the routine at offset 1e9c, then restores all registers from the stack and returns.
- `reset_counter_and_set_flag_then_call`: Clears the word at offset 1d0 of a6, sets bit 0 at offset 30 of a6, and if d0 is zero sets d0 to one, clears d1 and calls the routine at offset c1c before restoring d0-d1.
- `setup_object_from_table_and_call_helpers`: Calls a lookup routine, rejects a null or out-of-range result, copies four longwords from the object into a6, calls two setup routines, initializes fields at a6 and calls the routine at offset c1c with d3 and d2.
- `clear_timer_and_ack_flag`: Clears the longword at offset 368 from a6 and clears bit 0 of the byte at offset 48 from a6, then falls through to the next routine.
- `reset_and_dispatch_four_slots`: If the word at offset 10 from a6 is set, clears it and walks four longword slots at offset 58 from a6, calling a subroutine for each non-zero slot.
- `scan_four_script_streams`: Saves all registers, then for four objects clears a status block at offset 610 from a6 and walks each object's script pointer, decoding opcodes and tracking remaining length.
- `increment_d0_store_to_a1_and_branch`: Adds one to the low byte of d0, writes that byte to offset 16 of a1, then branches to another routine.
- `advance_pointers_increment_d7_loop_check`: Adds four to a0 and a1, increments d7, compares it with four and branches back if not equal, otherwise restores all registers and returns.
- `copy_state_and_init_four_slots`: Copies eight longs from a6 offset 610 to 642, copies a6 offset 12 to 16, then for four slots clears counters, sets slot pointers from a0 and a6 tables, and returns.
- `call_audio_then_run_two_updates`: Saves all registers, loads the audio base pointer from a pc-relative long, calls the non-null callback at offset 44, then runs two update subroutines before restoring registers and returning.
- `tick_sound_timer_and_voice_sequencer`: Decrements the sound timer at offset 50 in a6, recomputes period and volume from the note table, writes CIAA timer and volume bytes, then steps four voice channels through their sequence pointers.
- `decrement_voice_delay_and_advance_pointer`: Tests the long delay at offset 594 in a5, decrements it and, when it reaches zero, loads the sequence pointer at offset 642 into d0; if null, increments d6 and loops back.
- `parse_stream_command_word`: Reads a word from the stream at d0, stores the advanced pointer at 642(a5), splits opcode and operand, and for opcode 129 looks up a byte in the table at 64(a2) to set the pointer at 562(a5).
- `advance_sound_channel_and_track`: For the current channel at a5, checks the active flag at 58(a5), computes a pitch offset from a2 and a1, calls the pitch routine, stores the scaled value at 580(a5) and remainder at 596(a5), then advances a1 and a5 by four and increments d7.
- `compare_track_counter_and_step_channel`: Compares d0 with d1 and returns if unequal; otherwise decrements the word at 10(a6) when non-negative, and if it reaches zero copies a2 to a0, calls the routine at 774 and increments d6.
- `clear_pending_sound_pointer_when_idle`: Checks word at offset 10 of a6; if zero and long at offset 872 of a6 is nonzero, calls a subroutine then clears that long to zero.
- `signal_sound_task_if_pending`: Saves d0-d1/a0-a1, loads long at offset 872 of a6; if nonzero, calls exec Signal via library vector with a1 from offset 868 of a6, then restores registers.
- `poll_four_audio_channels_and_set_dmacon`: Loops over four channel structures at offset 466 of a6, calls per-channel callbacks, builds bit masks and writes them to DMACON to enable audio DMA.
- `enable_audio_dma_channel`: Clears d0, sets bit 15 in d0, writes d0 to DMACON to enable DMA, then writes 2 to the audio period register selected by shifting d7 left four and indexing from AUD0 base.
- `save_regs_call_audio_enable`: Pushes d0, d7 and a0 onto the stack, copies d0 into d7, calls the audio DMA enable routine, then restores the saved registers and returns.
- `allocate_audio_channel_slot`: Saves registers, walks the four audio channel slots at offset 466 of a6, finds a free matching slot, stores the sample pointer, period, volume and length, marks it active and returns the slot index or minus one.
- `clear_state_and_reset_four_slots`: Saves d0, clears the longword at offset 920 and word at offset 10 of a6, then calls the slot reset routine four times before restoring d0 and returning.
- `reset_slot_state_by_index`: Uses the low word of d0 times 24 as an offset into the a6 table at 466, clears the first byte and sets it to 2 if the second byte equals 1.
- `clear_state_and_reset_four_slots_alt`: Saves d0, clears the longword at offset 920 and word at offset 10 of a6, then calls a different slot reset routine four times before restoring d0 and returning.
- `reset_slot_and_release_object`: Clears byte and word fields of the 24-byte slot indexed by d0 in the a6 table, then if the slot was active calls the object's release method through its vtable and clears the slot flags.
- `read_slot_word_field`: Computes the 24-byte slot address from the sign-extended d0 index in the a6 table and returns the word at offset 22 of that slot in d0, preserving a1.
- `compute_scaled_ratio_fields`: Stores zero in a6 word 464, and if d0 is nonzero computes a scaled difference of d1 and the a6 word at offset 0, divides by d0, forces a minimum of one, and stores the results at a6 offsets 460 and 462.
- `store_d1_to_a6_and_branch`: Writes the low word of d1 to offset zero of the a6 structure, then branches unconditionally to another routine.
- `advance_scroll_position_by_speed`: Multiplies the speed word at offset 464 by the factor at offset 54, scales it, and adds the signed step to the position at offset 460, storing the high byte at offset zero of a6.
- `check_pending_flag_and_clear_bit`: Tests the word at offset 462 of a6; if zero, clears bit zero of the byte at offset 48 and, when that bit was set, calls a routine and clears the word at offset 464.
- `clear_sound_slot_if_active`: Saves d1 and a0, indexes a word table at offset 60 from a6 by the byte in d0 times four, and if that word is nonzero calls a routine then clears it.
- `copy_sound_word_from_channel_table`: Saves d0 and a0, sign-extends the byte in d0, scales it by four, and copies a word from offset 34 of the pointer at a6+6 into the table at a6+60.
- `add_122_to_d4_and_branch`: Adds 122 to the low byte of d4 and branches unconditionally to a shared continuation routine, leaving other registers untouched.
- `branch_to_e4a`: Branches unconditionally to the code at offset e4a, transferring control without touching any registers or memory.
- `check_resource_and_flag_then_return`: Saves registers, looks up a resource by two identifiers, compares a returned value, calls a check routine, tests a longword at offset 470 in a0 and restores registers before returning.
- `call_e4a_clear_a0_branch_e3e`: Calls the routine at offset e4a, clears a0 to zero, then branches unconditionally to offset e3e.
- `decrement_sample_refcount_and_free`: Saves all registers, loads a sample pointer from offset 470 of a0, calls the release routine, and if the sample's reference count at offset 8192 drops to zero calls the release routine again.
- `copy_sound_struct_if_channel_low`: Clears d0, and if d7 is below 4 copies fields from a1 into a2, reads a word from offset 10 of a1 into d0 and zeroes that word, then returns.
- `match_and_swap_sample_reference`: Saves registers, scans 64 entries at a6+74 for a matching sound structure with equal 32-longword key, then swaps the sample pointer at offset 470, adjusting reference counts at offset 8192 and releasing the old sample.
- `render_rotated_sprite_columns`: Walks the linked sprite list at offset 470 of the object, allocates a new sprite, then for each of 64 angle entries and 128 source bytes computes a rotated byte and writes it to the sprite data.
- `set_d0_bit6_then_branch`: Sets bit 6 of the low byte of d0 with an ori and then branches unconditionally into another routine; it is a two-instruction entry stub.
- `load_iff_vhdr_body_sprite`: Parses a FORM 8SVX IFF file, reads VHDR and BODY chunks, allocates memory for the sample body, and derives a sprite dimension from the header fields.
- `read_four_byte_value_via_stack`: Reserves four bytes on the stack, passes that buffer to the shared read routine with length four, then returns the popped longword in d0.
- `read_two_byte_value_via_stack`: Reserves two bytes on the stack, passes that buffer to the shared read routine with length two, then returns the popped word in d0.
- `copy_words_until_count_or_d5_exhausted`: Copies words from a5 to a0 while decrementing d5 and d0, and when d5 runs out calls the compare routine with d2 and d0, returning early if the result differs.
- `free_two_audio_structures`: Saves all registers, copies a0 to a1, loads the pointer at offset 58 of a1 into a0, calls the free routine twice on both structures, then restores registers and returns.
- `open_audio_and_cia_resources`: Allocates memory, opens audio.device, ciaa.resource and dos.library, fills audio and CIA structures, programs CIAA timers and interrupt registers, and returns the allocated base pointer.
- `shutdown_audio_system`: Saves registers, checks the global audio base pointer, and if non-null calls three cleanup routines plus the timer teardown before restoring registers and returning.
- `shutdown_device_and_free_resources`: Closes the open device and library, clears the CIAA control register, frees the signal, closes the task, and releases remaining allocations before returning.
- `wait_signal_with_mask_and_restore`: Allocates a signal bit, waits for it combined with the caller's mask, frees the signal, and returns the masked result in d0.
- `store_task_and_signal_state`: Clears the stored signal mask, saves d0 and d1 into the task structure, finds the current task, and stores its pointer before returning.
- `case_insensitive_string_compare`: Compares two null-terminated strings byte by byte, folding lowercase ASCII letters to uppercase, and returns with d0 zero when they match or nonzero on the first difference.
- `allocate_memory_via_helper`: Saves a0 on the stack, calls the memory allocation helper, moves the returned pointer from a0 into d0, restores a0 and returns.
- `allocate_memory_block_with_header`: Rounds the requested size up by four, checks free memory through exec.AvailMem, allocates with exec.AllocMem, stores the size in the first longword and returns the pointer in a0.
- `free_memory_block_via_exec`: Saves d0-d1/a0-a1, and if a0 is nonzero, frees the block four bytes before it using exec FreeMem through a6 or a custom free vector, then restores registers and returns.
- `get_a0_after_subroutine_call`: Pushes a0 on the stack, calls the following routine, moves the returned a0 into d0, pops a0 and returns.
- `allocate_and_compare_memory_block`: Saves registers, allocates a block of size 1005, fills it with two patterns, compares against a caller-supplied value, frees the block on mismatch, and returns the block pointer in a0 or zero.
- `main_game_entry_setup`: Checks the stack pointer, installs the interrupt vector, initializes display and input tables, opens the console, copies four word pairs, and runs the main loop until the quit flag is set.
- `update_player_and_audio_state`: Checks stack, saves d2-d3/d7, calls several subroutines, then updates game state words and clears a ten-byte buffer before returning zero in d0.
- `allocate_and_clear_sound_buffer`: Sets up a stack frame, stores a size value, calls an allocation routine, saves the returned pointer, and frees it if allocation failed, returning zero in d0.
- `toggle_active_player_and_reload_views`: Swaps the active player index, copies the selected player's view pointers into the display and secondary pointer globals, then calls two unnamed routines and returns 1.
- `init_ship_and_weapon_name_tables`: Stores text pointers for ship and weapon names into global tables, then loops over fleets and ships copying each name string via an unnamed routine and returns 0.
- `delay_loop_counting_to_125`: Reads a count from the stack, and for each unit spins an inner counter up to 125, then returns 1 in d0.
- `point_in_rectangle_test`: Pops three arguments from the stack, loads a rectangle pointer into a3 and two coordinates into d7 and d6, compares them against the rectangle edges, and returns 1 in d0 if inside or 0 otherwise.
- `return_zero_and_restore_registers`: Sets d0 to zero as the return value, restores d6, d7 and a3 from the stack, and returns to the caller.
- `dispatch_on_parameter_offset`: Builds a stack frame, saves registers, loads a pointer and two words from the stack, subtracts negative two from the first word, and jumps through a table indexed by that value when it is below five.
- `branch_to_compare_setup`: Branches unconditionally to the routine that initializes the nibble comparison buffers, transferring control without touching any registers or memory.
- `init_nibble_compare_buffers`: Loads sixteen constant words into the a5 frame at offsets minus thirty-four through minus four, then branches to the nibble extraction routine.
- `extract_nibbles_into_buffers`: Loops over sixteen rows and three nibbles, shifting words from a3 and the a5 constant table to fill the two nibble buffers at minus one hundred thirty and minus two hundred twenty-six.
- `increment_counter_and_branch_back`: Adds one to the word counter at offset minus 232 from a5, then branches unconditionally back to an earlier address in the caller's loop.
- `compare_and_adjust_three_column_table`: Loops over sixteen rows and three columns, comparing words at offsets minus 130 and minus 226 from a5, incrementing or decrementing mismatched entries and setting a flag word at minus 230.
- `decrement_and_rebuild_bitmask_entry`: Decrements a table word, compares row entries at offsets minus 130 and minus 226 from a5, clears the flag at minus 230 and rebuilds a bitmask word in the table pointed to by a3.
- `increment_counter_and_branch_back_2`: Adds one to the word at offset minus 232 from a5, then branches unconditionally to another part of the game code.
- `check_flag_then_call_and_restore`: Tests the word at offset minus 230 from a5; if set, stores one at minus 228, calls two subroutines with d6 and a3, then restores registers and returns.
- `setup_stack_frame_and_load_args`: Creates a stack frame, checks the stack pointer against a limit, saves registers, loads arguments from the stack into a3, d7, d6, d5 and a2, then tests a global word.
- `store_high_score_buffer_pointer`: Loads the address of the high score buffer into a0 and stores it at offset -32 from a5, then tests the word at 15120 and sets d0 to 2 if it is nonzero before branching.
- `parse_high_score_name_character`: Computes row and column offsets from the board width, stores them in the a5 frame, then reads a character from the name buffer, converts lowercase to uppercase, and dispatches letters or digits to scoring routines.
- `convert_digit_character_to_score`: Checks whether d0 holds an ASCII digit, converts it to its numeric value, multiplies by the board width via a call, adds the stored row offset and saves the result in the a5 frame.
- `dispatch_char_via_jump_table`: Clears a word at offset -22 from a5, copies d0 to d1, sign-extends it, subtracts 32, and if the result is between 0 and 32 jumps through a table indexed by that value.
- `draw_letter_columns_with_offsets`: Computes column offsets from globals and a5 locals, optionally draws two glyph passes via a called routine, advances the pointer and column counters, then branches back to the caller loop.
- `return_zero_restore_registers_2`: Sets d0 to zero as the return value, restores d2-d3, d5-d7 and a2-a3 from the stack, releases the a5 frame and returns.
- `allocate_bitmap_and_clear_on_failure`: Reads width and height words from the structure at a3, rounds width up to a multiple of sixteen, calls the allocation helpers, stores the result in the global bitmap pointer and in the structure, and frees it if allocation failed and the flag is zero.
- `allocate_bitmap_and_draw_into_structure`: Reads width and height words from the structure at a3, rounds width up to a multiple of sixteen, calls the allocation helpers, stores the result in the global bitmap pointer, passes it with the structure pointer to the drawing helper and returns one.
- `call_draw_helper_with_four_arguments`: Pops the structure pointer, two sign-extended word arguments and a fourth pointer from the stack, pushes them back in order and calls the drawing helper, then returns the low word of its result sign-extended.
- `jump_to_vector_at_negative_global_offset`: Loads a code pointer from the global variable area at offset minus three hundred seventy-eight and jumps to it, transferring control without returning.
- `dispatch_two_stage_jump_table_entry`: Jumps to the first of two fixed code targets, with the second jump serving as an alternate entry point into the same dispatch sequence.
- `jump_to_sprite_or_object_update`: Jumps directly to the code at offset three hundred twelve, transferring control to the sprite or object update routine without saving state.
- `jump_to_243a_entry`: Performs an unconditional jump to the code at address 243a, transferring execution without saving any registers or return address.
- `jump_to_a12_entry`: Performs an unconditional jump to the code at address a12, transferring execution without saving any registers or return address.
- `jump_to_2430_entry`: Performs an unconditional jump to the code at address 2430, transferring execution without saving any registers or return address.
- `jump_to_compare_strings`: Jumps to the string comparison routine at the fixed target, which compares two null-terminated strings and returns a signed result.
- `jump_through_global_offset`: Jumps to the code pointer stored at a fixed negative offset from the global base, transferring control to that routine.
- `jump_to_sprite_animation_update`: Jumps to the routine that updates animation frames, clearing frame pointers and invoking callbacks through the global base.
- `jump_to_26d4`: Transfers execution unconditionally to the code at address 26d4, without saving registers or returning.
- `jump_to_18`: Transfers execution unconditionally to the code at address 18, without saving registers or returning.
- `jump_to_58`: Transfers execution unconditionally to the code at address 58, without saving registers or returning.
- `jump_to_absolute_74`: Performs an unconditional jump to the absolute address 116, transferring execution without saving a return address.
- `jump_to_absolute_2c38`: Performs an unconditional jump to the absolute address 11320, transferring execution without saving a return address.
- `jump_to_absolute_2476`: Performs an unconditional jump to the absolute address 9334, transferring execution without saving a return address.
- `jump_to_244e_dispatch`: Performs an unconditional jump to the routine at address 244e, transferring control without saving registers or returning.
- `jump_to_2462_dispatch`: Performs an unconditional jump to the routine at address 2462, transferring control without saving registers or returning.
- `jump_to_1e10_dispatch`: Performs an unconditional jump to the routine at address 1e10, transferring control without saving registers or returning.
- `jump_to_sprite_anim_update`: Jumps to the shared sprite animation update code that reads the object table via d0 and updates animation frames.
- `jump_to_sprite_anim_update_alt`: Jumps to the shared sprite animation update code using a different object table offset than the other entry point.
- `jump_to_object_list_render`: Jumps to code that walks the a4-relative object list and renders each active object with nonzero size.
- `jump_to_dea_vector`: Jumps to the absolute address dea, transferring execution to the routine located there without saving any registers.
- `jump_to_7c8_vector`: Jumps to the absolute address 7c8, transferring execution to the routine located there without saving any registers.
- `jump_to_2cbc_vector`: Jumps to the absolute address 2cbc, transferring execution to the routine located there without saving any registers.
- `jump_to_102_vector`: Jumps to the absolute address 258, transferring execution to the routine located there without saving a return address.
- `jump_to_246c_vector`: Jumps to the absolute address 9324, transferring execution to the routine located there without saving a return address.
- `jump_to_329c_vector`: Jumps to the absolute address 12956, transferring execution to the routine located there without saving a return address.
- `jump_to_2480_dispatch`: Performs an unconditional jump to the code at address 2480, transferring execution without saving a return address.
- `jump_to_26e8_string_copy`: Performs an unconditional jump to the string copy code at address 26e8, transferring execution without saving a return address.
- `jump_to_zero_vector`: Performs an unconditional jump to address zero, transferring execution to the handler stored at the start of memory.
- `set_d0_zero_jump_dispatch`: Loads the immediate value zero into the low word of d0, then jumps to the shared dispatch routine at absolute address 36.
- `set_d0_eight_jump_dispatch`: Loads the immediate value eight into the low word of d0, then jumps to the shared dispatch routine at absolute address 36.
- `set_d0_sixteen_jump_dispatch`: Loads the immediate value sixteen into the low word of d0, then jumps to the shared dispatch routine at absolute address 36.
- `set_d0_24_and_jump_vector`: Loads the immediate value 24 into the low word of d0, then jumps through the longword at address 36 to a shared dispatcher.
- `set_d0_32_and_jump_vector`: Loads the immediate value 32 into the low word of d0, then jumps through the longword at address 36 to a shared dispatcher.
- `set_d0_40_and_jump_vector`: Loads the immediate value 40 into the low word of d0, then jumps through the longword at address 36 to a shared dispatcher.
- `set_d0_48_and_jump_vector`: Loads the immediate value 48 into the low word of d0 and jumps through the longword at address 36, passing control to the shared dispatcher.
- `set_d0_56_and_jump_vector`: Loads the immediate value 56 into the low word of d0 and jumps through the longword at address 36, passing control to the shared dispatcher.
- `set_d0_64_and_jump_vector`: Loads the immediate value 64 into the low word of d0 and jumps through the longword at address 36, passing control to the shared dispatcher.
- `set_d0_72_and_jump_vector`: Loads the immediate value 72 into d0 and jumps through the absolute long vector at address 36, transferring control to the system routine there.
- `init_four_slot_string_table`: Checks the stack against the saved limit, allocates a string, fills four ten-byte slots with copied names and 255 markers, then stores display and timing constants in the d9 data block.
- `free_four_slots_if_active`: Verifies the stack pointer against the saved limit, then if the active flag at d9 offset 54 is set, loops over four slots calling the slot-free routine for each index.
- `dispatch_submarine_sonar_contact`: Checks the stack pointer, saves d2/d6/d7/a2, reads the contact index from the stack, looks up its ten-byte record, picks a range value and calls the sonar update routine with the record fields, then returns zero.
- `jump_to_string_copy_routine`: Unconditionally jumps to the string copy routine that appends the source string to the end of the destination string and returns the destination pointer.
- `jump_to_short_address_routine`: Unconditionally jumps to a routine at a low memory address, transferring execution without saving registers or returning to this location.
- `jump_to_absolute_154`: Performs an unconditional jump to the absolute address 154, transferring execution without saving any registers or return address.
- `jump_to_absolute_92`: Performs an unconditional jump to the absolute address 92, transferring execution without saving any registers or return address.
- `jump_to_absolute_26e8`: Performs an unconditional jump to the absolute address 26e8, transferring execution without saving any registers or return address.
- `jump_to_absolute_zero_vector_2`: Jumps to the absolute address zero, transferring execution to the vector table entry at address zero.
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
- `jump_to_handler_at_1b30`: Jumps to the absolute address 1b30, transferring control to the handler installed there.
- `jump_to_address_1ca8`: Jumps to the absolute address 1ca8, transferring execution to the code located there.
- `call_library_aa2_with_three_args`: Saves a6, loads a0, d0, d1 and d2 from the stack arguments, sets a6 to the library base at address zero, calls the library vector at offset aa2, restores a6 and returns.
- `call_library_b72_with_one_arg`: Saves a6, loads d0 from the stack argument, sets a6 to the library base at address zero, calls the library vector at offset b72, restores a6 and returns.
- `call_library_bb0_with_one_arg`: Saves a6, loads d0 from the stack argument, sets a6 to the library base at address zero, calls the library vector at offset bb0, restores a6 and returns.
- `save_a6_call_b5c_restore_a6`: Pushes a6 onto the stack, loads a6 from absolute address zero, calls the routine at address b5c, then restores a6 from the stack and returns.
- `load_two_args_call_164_restore_a6`: Pushes a6, loads a0 and a1 from the two stack arguments, sets a6 from absolute address zero, calls the routine at address 164, restores a6 and returns.

### Hunter Killer (1989)(Mastertronic)[cr VF].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `load_two_args_call_fb2c_restore_a6`: Pushes a6, loads a0 and a1 from the two stack arguments, sets a6 from the library base at f988, calls the routine at fb2c, restores a6 and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `main_game_loop_frame_update`: Saves all registers, restores the saved game register block, points a6 at custom chip base, calls six setup and update routines, then branches on the vertical position register and fire button to advance the scrolling playfield counters and redraw, finally restoring registers and returning zero in d0.
- `vertical_blank_interrupt_acknowledge`: Saves all registers and the status register, writes the value 32 to custom register offset 156 to acknowledge the vertical blank interrupt, then restores the status register and registers and returns from the exception.
- `clear_memory_and_init_buffers`: Saves all registers, sets four longword pointers to the playfield and sprite buffer areas, clears a large block of memory with a counted loop, then copies two hundred twenty-four bytes from one table to another before restoring registers and returning.
- `copy_eleven_line_tables_with_offsets`: Saves all registers, then for eleven source/destination pairs calls the line-copy routine with decreasing offset values, restoring registers and returning.
- `copy_line_table_subtracting_offset`: Reads words from a0, subtracts d0 from the first two, copies the next two unchanged, then copies trailing words until two consecutive 255 markers, writing to a1.
- `draw_polygon_lines_with_blitter`: Computes a blitter shift from a1 offset, then loops over vertex index pairs in a0, reading coordinates from a1 and a2 tables and writing blitter registers at a6.
- `scale_sprite_coordinate_tables`: Saves all registers, loads a register block from fixed memory, then walks several word tables multiplying entries by 84 or 2 until sentinel values, writing results back in place.
- `process_eleven_sprite_triplets`: Loads three table pointers into a0, a1 and a2 for eleven successive groups and calls the shared triplet processor for each group before returning.
- `adjust_triplet_offsets_and_rebuild`: Checks the first word for a marker, subtracts 42 from entries stepping by three words until a sentinel, then calls the rebuild and update routines with the advanced pointer.
- `decode_script_command_to_a0`: Saves all registers, reads a byte from the script pointer, handles the s and a command bytes by copying three words into globals, then looks up a word offset table and copies words into the buffer at a0 until two 255 markers, restoring registers.
- `transform_vertices_by_rotation_matrix`: Saves registers, reads triples of words from a0 until a 255 terminator, multiplies each by the rotation matrix at a3, divides by the transformed depth, adds screen offsets and writes x and y words to a1 and a2.
- `build_rotation_matrix_from_angles`: Loads three angle indices from a2, looks up sine and cosine values, computes the nine rotation matrix entries at offsets 24 to 52 of a3 and stores the scaled depth divisor at offsets 54 and 56.
- `toggle_display_buffers_and_wrap_coords`: Saves all registers, toggles bit 1 of a flag to pick one of two display buffers, copies the chosen buffer pointer to the copper list, sets display and color registers, then adds three velocity words to three coordinate words and wraps each past 719.
- `save_playfield_and_install_copper_list`: Copies two words from the a6 structure to globals, sets bitplane and color registers, saves the old level 3 interrupt vector and installs a new one, then points the copper list at a new list and clears a word.
- `configure_audio_period_registers`: Writes period and control values to the audio hardware registers at offsets 150 and 154 from a6, setting high bits and returning.
- `copy_words_to_bitmap_rects`: Loops d0 times, writing d2 and d2+2 as x coordinates and the high and low words of each longword from (a0)+ as y coordinates into (a1)+, advancing d2 by four each pass.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_game_state_and_open_library`: Clears score, grid and table arrays, fills many global variables with initial values, sets up display and sprite structures, then opens a library and stores the base pointer.
- `load_and_blit_three_tiles`: Copies a small parameter block, loads three tile images via graphics calls, computes scaled dimensions, blits each tile into the bitmap and advances the destination y coordinate.
- `draw_two_player_ships_and_hud`: Sets up a stack frame, stores four word constants, writes ship state to globals, then loops twice calling graphics routines to draw player ship sprites and HUD elements based on the player number argument.
- `animate_ship_bits_toward_target`: Extracts three 3-bit fields from two packed words, then repeatedly steps each field toward its target value, updating a packed word and calling a blit routine until all fields match.
- `draw_five_direction_ship_sprites`: Loops five times over direction offsets, computes sprite coordinates, calls blitter and text routines to draw ship sprites and score text for each direction.
- `clip_and_blit_two_rects`: Clips a rectangle from signed x offset and y start against screen bounds, stores the clipped edges in the caller's structure, then blits two sprite descriptors via the blitter wrapper.
- `clip_and_blit_single_rect`: Clips a rectangle from signed x offset and y start against screen bounds, stores the clipped edges in the caller's structure, then blits one sprite descriptor via the blitter wrapper.
- `clip_and_blit_six_layer_rects`: Clips a rectangle against screen bounds, blits it, then loops over six layer entries and blits each matching layer rectangle via the blitter wrapper.
- `setup_playfield_window_and_blit`: Checks the stack against a limit, saves d2, writes window and blit parameters into chip memory, then calls the blitter setup with a parameter block and restores d2.
- `init_blitter_window_parameters`: Builds a four-word parameter block on the stack, sets blitter control words in chip memory, and passes the block and a value to the blitter routine.
- `advance_animation_frame_and_blit`: Adds a step to a frame counter, wraps it between one and fifteen, then reads a table entry and calls the blit routine with the frame and table value.
- `select_level_and_start_game`: Reads level number from stack, looks up its entry in the level table, marks it active, copies level data to the current-level pointer, formats the level and score strings, then calls the level loader.
- `decode_rle_bitmap_to_screen`: Reads a packed bitmap descriptor, decodes run-length bit runs into the destination buffer using shift and mask operations on words, and returns zero when the row count is exhausted.
- `reset_level_tables_and_clear_names`: Formats two fixed strings, then loops 25 times copying the current name into each level record, clearing the first byte and storing pointers and name fields in the level table.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_to_ee9a_trampoline`: Performs an unconditional jump to the code at address ee9a, transferring execution there without saving any registers or returning.
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
- `jump_to_shared_exit_stub`: Performs an unconditional jump to a shared exit routine located elsewhere in the runtime segment, transferring control without saving registers or returning.
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
- `jump_to_5348_then_frame_setup`: Jumps to address 5348; the following instructions set up an a5 frame, compare the stack pointer with a longword at 1cdd4 and save registers, but are unreachable after the jump.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_level_resource_tables`: Fills a series of global resource descriptor words with fixed values and returns one, or returns zero if the stack pointer check fails.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_libraries_and_setup_screens`: Opens two libraries, allocates two bitplanes and two rasters, creates a viewport and window, then copies palette words into the display structure and returns one.
- `close_window_and_free_resources`: Calls two teardown routines, closes the window stored in the global pointer if present, frees the returned resource and returns.
- `reset_game_state_and_free_six_slots`: Calls setup routines, stores 8000 and scaled values into globals, then frees six pairs of ten-byte slots at two tables when non-zero, returning one.
- `free_seventeen_flag_slots`: Loops over seventeen entries, and for each non-zero word flag copies the paired long value to a scratch global and frees it.
- `shutdown_free_buffers_and_return_one`: Frees a global buffer if set, calls two teardown routines, frees two more buffers when non-zero, restores d7 from the stack and returns one in d0.
- `allocate_9e0e_struct_and_store_pointer`: Checks the stack against the saved limit, calls an allocation routine, stores the returned pointer at a fixed global, writes the value 40462 at offset 32 of that block, loads a code address into its longword field and returns zero in d0.
- `dispatch_index_from_9e06_table`: Checks the stack limit, reads the word at a fixed global, subtracts two, rejects values below zero or at least ten, scales the index by four and jumps through a nearby table.
- `branch_to_shared_dispatch_body`: Unconditionally branches forward to the shared dispatch body, transferring control without touching any registers or memory.
- `print_message_and_branch_to_main`: Pushes the longword at absolute address 9e06 and a string pointer, calls the print routine, cleans the stack, prints a second string, then branches to the main loop.
- `print_message_call_sub_and_clear_long`: Pushes the longword at absolute address 9e06 and a string pointer, calls the print routine, cleans the stack, calls a nearby subroutine, pushes a zero longword, calls another routine and returns.
- `jump_to_two_absolute_targets`: Jumps to the absolute longword address 1d144, and the following jump to 67a0 is unreachable because the first jump never returns.
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
