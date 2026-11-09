# Game 891 (karate master (demo) (1998)(f1 software)(pd)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00891). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 17 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 412 of 412 named (0 author symbols), 412 of 412 described (41 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 220 of 220 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7231 of 7231 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 22720 |
| code | 23586 |
| data | 279687 |
| padding | 4759 |
| unused | 570368 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| MMPP | 3 | 4288 | 172 |
| Loader | 2 | 228 | 11 |
| mathtrans.library | 17 | 830 | 97 |
| mathieeedoubbas.library | 3 | 568 | 47 |
| diskfont.library | 15 | 1040 | 76 |
| Karate Master (demo) (1998)(F1 Software)(PD).adf:KarateMaster | 2 | 267 | 8 |
| Karate Master (demo) (1998)(F1 Software)(PD).adf:bootblock | 1 | 10 | 1 |

## Cases handled

- karate master (demo) (1998)(f1 software)(pd)/Karate Master (demo) (1998)(F1 Software)(PD).adf:KarateMaster: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `build_and_display_score_string` (MMPP): Copies a template into a buffer, appends formatted numbers and text via helper calls, pads the buffer with spaces up to a limit, then passes it to a display routine and sets a flag. Kind: checksum/data integrity trap. Pass path: the branch where the status flag is not set (the 'not equal' or 'not zero' path, depending on the specific instruction logic, but specifically the path that avoids the failure branch indicated by the 'tst/seq' or 'sf' logic).

## Functions


### MMPP

- `jump_to_absolute_zero_entry`: Jumps to absolute address zero, transferring execution to the program entry point without saving any registers or returning.
- `allocate_memory_and_setup_task`: Saves registers, calls exec AllocMem for a large block, returns 20 on failure, otherwise stores the size, sets up a task structure with a return handler and stack pointer, and calls a setup routine.
- `restore_stack_free_memory_and_return`: Restores the stack pointer from the task structure, sets a flag byte, calls a cleanup routine, frees the allocated memory via exec FreeMem, and returns the stored result value.
- `find_string_length_up_to_256`: Scans the byte string pointed to by the caller's argument for a zero terminator, counting up to 256 bytes, and returns the length in d0.
- `copy_string_into_limited_buffer`: Calls the length scanner, then copies bytes from the source string into the destination buffer while the index stays below 256 and below the limit at offset 1592 of a5, stopping at the source terminator.
- `terminate_copied_string_at_index`: If the saved copy index is below 256, writes a zero byte at that offset in the destination buffer to terminate the copied string, then unwinds the stack and returns.
- `init_mmpp_display_and_chain`: Checks a flag at offset 19730 in a5; if set, fills two display structures at 18506 and 18466, opens a 1024x320 screen via graphics library, then clears 19716 and jumps through 19708.
- `compare_read_length_to_request`: Reads a file handle from the stack, calls the dos library read function with buffer and length, then compares the returned count to the requested length and sets d0 to one if they differ.
- `decrunch_mmpp_module_file`: Allocates a buffer, opens and reads a file, checks for PP11 or PP20 headers, decrunches the data, copies it into memory and closes the file, returning success or failure.
- `load_mmpp_file_into_buffer`: Copies a 256-byte filename buffer, strips any directory prefix up to a slash or colon, prepends the RAM: device name, opens the file with the dos library, reads it into memory and closes it, restoring the saved file handle.
- `allocate_aligned_memory_block`: Rounds the requested size up to an even number, adds it to the running heap total, and if the total stays within 4096 bytes returns a pointer into the current block, otherwise allocates a new 4100-byte block and links it into the list.
- `free_all_heap_blocks`: Walks the linked list of allocated memory blocks at offset 19692 in a5, freeing each block through the memory free routine, then resets the heap byte counter at offset 19690 to 4096.
- `close_window_and_free_screen`: Closes the window via graphics library, frees the screen, and if the flag byte on the stack is set, frees the bitmap at offset 44 of the base structure before returning.
- `reset_scroll_and_clear_playfield`: Sets both bitmaps' scroll offsets to twice the value at 1594(a5) via graphics library, clears word at 13910(a5) and word at offset 10 of the structure at 18062(a5).
- `build_playfield_and_open_window`: Computes screen dimensions from display info, allocates a screen and window, fills a 32-row by 48-column character buffer, draws it, and sets up the playfield view.
- `release_two_allocated_buffers`: Checks two longword pointers in the a5 frame, calls the Amiga library free function on each non-null one, then clears both slots to zero.
- `read_next_byte_from_file_buffer`: Refills a 512-byte buffer via the Amiga read call when exhausted, then returns the next byte in 19632(a5) and advances the buffer index and counter.
- `parse_keyboard_command_line`: Builds a command string from joystick and key state, reads characters through the buffer routine, parses digits and letters into a command table, and updates the control flags.
- `draw_karate_opponent_sprite`: Builds a sprite descriptor from the opponent's x and y offsets, calls the blit routine, and optionally advances the animation frame counter before drawing.
- `blit_sprite_rows_until_column_done`: Computes source and destination pointers from row and column counters, copies eight rows of four-byte words, increments the row counter, and resets it when it equals the column limit.
- `copy_input_and_uppercase_text`: Copies 67 longwords plus a word from the input buffer to the stack, clamps the length, uppercases bytes above 128, then calls the draw routine with the stack buffer.
- `wait_for_signal_if_pending`: Checks the pointer at offset 32 of a5 and the flag at offset 20; if the pointer is null and the flag set, sets byte 18012 of a5, calls exec.Wait with the value at 18008 of a5, clears the byte, then returns whether the pointer is now non-null in d0.
- `advance_list_node_and_accumulate_delta`: Calls the wait routine; if it returns non-zero, advances the node pointer at offset 32 of a5 to its next link, increments the counter at 1598 of a5, adds the node's signed word at offset 8 to the total at 13902 of a5, and if the counter reaches the limit at 1590 of a5 advances the pointer at offset 28 of a5 before tail-calling the routine at 0E40.
- `step_back_list_node_and_restore_scroll`: If the node at offset 28 of a5 and its next link are both non-null, moves to the next node, subtracts the current node's signed word at offset 8 from the total at 13902 of a5, decrements the counter at 1598 of a5, advances the pointer at offset 32 of a5, and when the countdown at 13910 of a5 is zero writes the shifted limit at 1590 of a5 to offset 10 of the object at 18062 of a5 and reloads the countdown.
- `decrement_row_counter_and_blit_tile_row`: Decrements the row counter at offset 13910(a5), computes a byte offset from it, calls the tile blit routine with source and destination pointers, and optionally adjusts the copper list and plays a sound before drawing the row.
- `blit_playfield_column_with_scroll_offset`: Computes source and destination addresses from the row counter, playfield width and scroll offsets, then copies eight rows of longwords between the two buffers while decrementing the copper list pointer.
- `reset_row_counter_and_blit_first_row`: If the row counter at 13910(a5) is zero, reloads it from the playfield height and sets the copper list offset, then decrements it and blits the first row of tiles.
- `blit_vertical_strip_columns`: Uses a5 globals to compute source and destination pointers, calls two helper routines, then copies 4-byte words column by column into the screen bitmap while decrementing a counter at offset 10 of the structure pointed to by 18062(a5).
- `format_number_into_buffer`: Converts the long value at 10(a7) to decimal digits using repeated division by ten, writes them backwards into the buffer at 18(a7) starting at index 14(a7), pads with spaces, and terminates with a zero byte.
- `restore_stack_and_jump_caller`: Adds six to the stack pointer, pops the return address into a0, discards twelve more bytes of stack, and jumps to that address.
- `sum_list_entry_sizes`: Clears two longwords in the a5 frame, then walks a linked list from the stack argument, adding each node's word size to the total and advancing the argument pointer until the list terminator.
- `pop_return_address_skip_arg`: Pops a return address from the stack into a0, discards one longword argument by adding four to a7, then jumps to the saved address.
- `advance_list_pointer_by_count`: Calls a setup routine, then loops up to the count at offset 1590 in a5, following the next pointers at offset 32 in a5 while nodes are non-null, and stores the resulting pointer back.
- `skip_stack_args_until_count_reached`: Reads the word at the top of stack into d7, compares it with the word at offset 636 in a5, and while below it replaces the top stack entry with the longword pointed to by the second stack entry, incrementing the top word each time.
- `store_arg_and_run_three_setup_calls`: Saves the incoming stack argument at offset 32 in a5, calls three subroutines with pushed arguments, then loops from 1 to the word at offset 1590 in a5, storing each stack longword at offset 28 in a5 and calling another subroutine.
- `clear_flag_and_pop_six_bytes`: Clears the byte at offset 4670 in a5 to zero, adds six to the stack pointer to discard three words of arguments, and returns to the caller.
- `wait_for_signal_if_pending_2`: Tests the longword at offset 20 in a5; if nonzero, sets the byte at 4653 in a5, calls exec.Wait with the pointer at 4652 in a5, then clears that byte and returns.
- `copy_node_and_insert_escaped_text`: Copies 67 longwords plus a word from the node at a5 offset 32 into the caller's structure at offset 248, links it into the list, and inserts escaped text bytes into the buffer at offset 260.
- `insert_escaped_text_into_buffer`: Writes the escape byte 17 minus the saved character at the given index in the buffer at offset 260, then copies the node's text bytes after it until a zero byte.
- `append_char_and_relink_node`: Copies a character into the node's text buffer at offset 260, appends the next character from the string pointer at offset 518, advances the stack index, then relinks the node into the list via offsets 248 and 252 before returning.
- `copy_string_pointer_to_neighbors`: Loads the string pointer at offset 518 of the node and stores it into the next node's offset 4 and the previous node's offset 0, then returns to the caller.
- `check_flag_and_call_library`: Tests the byte at offset 522 of the node; if clear, it calls an Amiga library function through a6 and then checks the longword at offset 4 of the structure at offset 28 of a5.
- `copy_second_field_and_maybe_call`: Copies the longword at offset four of the structure pointed to by a5+28 into a5+32, calls a subroutine, then if the byte at offset 522 of the caller's argument is nonzero calls another subroutine before returning via the saved return address.
- `init_record_and_compare_threshold`: Clears the word at offset twelve of the record passed on the stack, loads the word at offset eight of the a5+32 structure into d7, and if it is below the a5+1592 threshold stores it at offset six of the record.
- `match_name_against_input_string`: Copies a threshold word into the record, then scans the a5+32 string comparing uppercased characters against the record's name buffer, advancing indices until a mismatch or terminator, and on a full match calls two subroutines and sets the record's flag byte.
- `increment_caller_field_at_offset_12`: Loads the return address from the stack into a4, adds one to the word at offset 12 of that caller structure, then branches to another routine.
- `return_zero_and_discard_stack_frame`: Sets d0 to zero, pops the return address into a0, skips four more stack bytes and jumps to that address.
- `build_and_display_score_string`: Copies a template into a buffer, appends formatted numbers and text via helper calls, pads the buffer with spaces up to a limit, then passes it to a display routine and sets a flag.
- `open_audio_and_wait_for_signal`: Adds the frame delta to the counter at offset 14 of the structure in a5, opens the audio device at 18098(a5), fills the IO request fields, then waits on the signal bit of the opened device and halves the counter.
- `draw_title_text_lines`: Calls the clear routine, then repeatedly pushes text pointers and a byte onto the stack and calls the text drawing routine to render the title screen lines, then computes a delay from 1590(a5) and counts it down.
- `wait_for_quit_message`: Waits on the window port, gets and replies to messages, stores the message in 18024(a5), and loops until a message has bit 2 set and a value below 128.
- `reply_msg_and_advance_list_head`: Replies to the message in a1 via exec, copies the longword at offset 28 of a5 to offset 32, calls a subroutine, then pops a0 and skips four bytes before jumping back.
- `walk_list_skipping_entries`: Calls a subroutine, sets a5 offset 32 from 40, stores a5 word at 1590 into the caller's word at offset 12, then walks the linked list at a5 offset 32 while the count exceeds one and the next node's longword at offset 4 is nonzero.
- `call_sub_and_return_to_caller`: Calls the subroutine at the relative target, then pops a0 from the stack, adds four to the stack pointer and jumps through a0 to return to the caller.
- `check_flag_and_clear_byte`: Loads the caller's argument pointer from the stack, copies byte at offset 66 into a5 memory, tests two a5 fields and the argument's byte 5, returning minus one if set.
- `clear_argument_byte_and_branch`: Loads the caller's argument pointer from the stack, stores zero into its byte at offset 5, then branches into the shared continuation code.
- `toggle_flag_and_call_update`: Clears byte 5 of the argument, tests bytes 4 and 5 plus a5 flag, optionally calls the update routine, then toggles the a5 flag byte.
- `set_flag_and_clear_byte_then_return`: Loads the return address from the stack into a4, writes 255 to the byte at offset 4 of that address and to the byte at offset 4670 of a5, sets d0 to zero, then pops a0 and jumps to it.
- `copy_byte_and_test_flags`: Copies the byte at offset 66 of the return address into offset 4671 of a5, then tests the byte at offset 4 of the return address and the byte at offset 4670 of a5, setting that byte to 1 if both are zero, before branching.
- `set_flag_and_clear_byte_then_return_2`: Writes 255 to the byte at offset 4670 of a5, loads the return address into a4, clears the byte at offset 4 of that address, then pops a0 and jumps to it.
- `wait_for_input_and_update_state`: Allocates a large stack frame, calls exec.Wait on the message port, clears several state bytes, then tests and updates flags at offsets in a5 and calls two helper routines before branching on the result.
- `set_flag_4670_and_branch`: Stores a byte value of 255 into the flag at offset 4670 of a5 and branches forward to the shared exit point.
- `copy_byte_call_helper_test_pointer`: Pushes the byte at offset 4671 of a5, calls a helper, loads the pointer at offset 28 of a5, tests the longword it points to and stores the result as a flag at offset 4670 before branching to the exit.
- `poll_message_port_and_reply`: Waits on the message port, gets a message, stores it, and if bit 3 of the message flags is set, clears bit 3 of the port flags, replies to the message and sets the bit again.
- `copy_message_and_reply_if_low`: Copies twelve longwords from the stored message into the stack buffer, replies to the message, and if the message type is below 128 and a pending flag is set, calls the handler and clears the flag.
- `handle_key_event_type_fifteen`: For message type fifteen, calls the key handler, clears the flag, records modifier bits from the message into stack flags, and dispatches to further handlers based on those bits.
- `check_bit5_and_call_1b96`: Tests bit 5 of the byte at offset 40 from the stack pointer; if set, pushes the stack pointer and calls the routine at 1b96, then branches to 1bf2.
- `check_bit2_and_compare_80`: Tests bit 2 at offset 36 from the stack pointer, returns if clear or if the word at offset 38 is at least 128; otherwise loads that word into d7 and compares it with 64.
- `compare_d7_with_41_and_call`: Compares d7 with 65; if equal, pushes the stack pointer and calls the routine at 1b96, then branches to 1bf2.
- `handle_joystick_direction_4d_1e_1f`: Compares d7 against 77, 30 and 31; for those directions it tests stack flags at offsets 66 and 4 and toggles the byte at a5+18032, then branches to shared code.
- `set_three_flags_and_jump_shared`: Stores 255 into the two bytes at a5+18032 and a5+18033, sets the byte at stack offset 4 to 255, then branches to the shared continuation.
- `compute_direction_count_from_flags`: Tests the byte at stack offset 67 and the word at offset 38; if neither matches, it loads the word at a5+1654, decrements it and stores the result at stack offset 12.
- `repeat_sound_until_counter_zero`: Sets a word counter to one, pushes a flag byte from the stack frame, calls a sound routine, decrements the counter and loops back while it is nonzero, then clears a byte at offset 18032 in a5 and branches away.
- `dispatch_key_code_to_action`: Compares d7 against several key codes and branches to matching handlers, testing stack-frame flags and bytes at offsets 18032 and 18033 in a5 to set or clear state before continuing.
- `clear_two_flags_and_stack_byte`: Stores a byte value into offsets 4670 and 4671 of a5, clears the byte at offset 4 of the stack frame, then branches to the common exit path.
- `check_flag_and_keycode_set_delay`: Tests the byte at offset 67 of the stack frame and branches if set; otherwise compares the word at offset 38 with 63, and if equal loads the word at offset 1590 in a5, decrements it and stores it at offset 12 of the stack frame before branching.
- `set_delay_and_call_with_flag_byte`: Stores 1 at offset 12 of the stack frame, tests the byte at offset 66, pushes a byte set from that test, calls a routine, clears the byte at offset 18032 in a5, decrements the delay word and branches based on whether it reached zero.
- `compare_keycode_and_call_clear_flag`: Compares d7 with 68 and 67, and if it equals 67 pushes a zero byte, calls a routine, clears the byte at offset 18032 in a5 and branches away.
- `check_d7_3d_copy_a5_18_to_20`: Compares d7 with 61; if equal, copies the longword at offset 24 of a5 to offset 32 of a5, calls a subroutine, then branches to another routine.
- `check_d7_1d_push_stack_call`: Compares d7 with 29; if equal, pushes a longword onto the stack, calls a subroutine, then branches to another routine.
- `handle_d7_50_to_59_table_lookup`: For d7 between 80 and 89, indexes a table at offset 19066 of a5, stores the entry at offset 32 of a5, follows a linked pointer, calls a subroutine and branches away.
- `store_long_at_indexed_table_slot`: Reads the word argument at offset twelve from the stack into d7, shifts it left by two, points a4 at a table relative to a5, stores the long at offset twenty-eight of a5 into the indexed slot, then branches away.
- `check_keycode_95_and_call`: Compares d7 with the decimal value 95, and if equal pushes a long onto the stack, calls the routine at the given offset, then branches to the common exit.
- `check_keycodes_69_and_46`: Compares d7 with 69 and branches if equal, then compares with 46 and branches if not equal, otherwise tests the byte at offset 67 of the stack and branches accordingly.
- `lookup_keymap_byte_and_handle_t`: Compares the word at offset 38 of the stack with 64, then indexes the table at a5+4aa6 by that value, stores the byte at stack offset 32, and if it equals 116 copies a5+24 to a5+32 and calls a subroutine.
- `check_keymap_byte_for_b_and_call`: Compares d7 with 98 and, if equal, pushes the stack pointer and calls a subroutine before branching to the common exit.
- `dispatch_keymap_commands_f_n_p`: Checks d7 for 102, 110 or 112, and for 112 with stack byte 523 equal to 102 calls a subroutine, clears a5+19106, then uppercases bytes from the stack buffer into another buffer.
- `scan_byte_table_until_zero`: Reads a word index from the caller's stack frame, tests the byte at that offset in a table, and if zero increments the index and branches back to the scan loop.
- `advance_object_list_and_clear_flag`: Calls a subroutine, clears a byte flag in the stack frame, then walks the linked list at offset 19106 of a5 up to a count from 1590(a5), advancing the list head and decrementing the counter.
- `advance_list_head_past_sentinel`: Compares the list head at 32(a5) with the sentinel at 19106(a5), resets it to 28(a5) when equal or null, then calls two subroutines and advances the head to its next node.
- `advance_a5_offset32_list_node`: Loads the pointer at offset 32 of the a5 frame, follows its next field at offset 4, stores the result back at offset 32, and tests the new node, calling a subroutine with a7 pushed and branching based on the returned byte in d0.
- `clear_a5_offset32_and_call_then_jump`: Clears the longword at offset 32 of the a5 frame, pushes a7, calls a subroutine, then branches unconditionally to another routine.
- `sum_list_weights_when_d7_is_119`: When d7 equals 119 and several a5 flags and pointers are set, clears two a5 counters, walks the linked lists at offsets 19066 and 19070 summing word fields at offset 8 into the counters, then branches.
- `walk_list_accumulate_node_delay`: Loads the list head from a5 offset into a4, pushes each node pointer, and while non-null adds the node's signed word at offset 8 to the accumulator at a5 offset, then follows the next pointer.
- `counted_list_accumulate_node_delay`: Pushes the list head and a count from a5, then while count exceeds one and the node is non-null decrements the count and adds each node's signed word at offset 8 to the accumulator at a5 offset.
- `subtract_tick_and_signal_if_positive`: Subtracts the tick delta from the accumulator at a5 offset; if still positive it builds a message, calls two routines, sets a flag, signals the exec task via Signal, then calls another routine.
- `handle_key_o_send_printer_message`: Checks d7 for the letter o and several flag bytes, copies two longs into a5 offsets 558 and 562, calls two string routines and a signal routine, sets flag at 18015(a5) and signals the task.
- `handle_key_l_wait_for_signal`: Checks d7 for the letter l, calls two setup routines, sets flag at 18016(a5), signals the task, waits for a signal and tests the bit in d0 against d7.
- `handle_key_g_walk_linked_list`: Checks d7 for the letter g, calls a routine with stack arguments, then walks a linked list at a5 offset 24 for a count from 18450(a5), calling a test routine each node.
- `call_135c_then_jump_1bf2`: Calls the routine at relative offset 135c, then unconditionally branches to the code at relative offset 1bf2 without returning here.
- `check_d7_for_68_then_call_1918`: Compares the low byte of d7 with 104; if equal, pushes a longword from a7 and calls the routine at relative offset 1918, then branches to offset 1bf2.
- `check_d7_for_71_or_78_branch`: Compares the low byte of d7 with 113 and 120, branching to relative offset 230e when either value matches, otherwise falling through.
- `signal_mm_sleeping_port`: Checks d7 equals 's' and two flags clear, finds the MM is sleeping port, signals it, then waits on the task signal mask and clears it.
- `set_flag_signal_and_wait_task`: Sets the byte flag at offset 465e in a5, signals the task signal mask stored at 4658, then waits on the task signal mask with d0 zero.
- `split_path_open_file_requester`: Splits the path at 300(a5) into directory and filename buffers, opens arp and asl libraries, runs a file requester, then closes libraries and jumps to the stored exit vector.
- `set_sleep_flag_and_wait_signal`: Stores a pointer and sets the sleep flag at offset 19254, then waits on the signal mask in d0 via exec.Wait before restoring the pointer.
- `muchmorepopa_main_setup`: Increments a counter and, on first entry, initializes ports, signals, devices and console structures for the MuchMorePoPa text viewer, parsing arguments and opening input.device.
- `muchmorepopa_cleanup_and_exit`: Decrements the counter and, when zero, closes devices and libraries, frees signals, removes ports and tasks, then jumps to final teardown.
- `open_graphics_library_once`: Increments a counter at offset 19704 from a5; on the first call it calls a setup routine, opens graphics.library version 33 via exec, stores the base at 19700(a5), and if opening fails clears 19716(a5) and jumps through the pointer at 19708(a5).
- `close_graphics_library_on_last_release`: Decrements the counter at offset 19704 from a5; when it reaches zero it tests the library base at 19704(a5), and if non-null closes it via exec.CloseLibrary before jumping to another routine.
- `signed_32bit_multiply_with_sign_fixup`: Saves d2-d3, takes absolute values of d0 and d1 while tracking the sign in d3, swaps halves, multiplies the 16-bit halves, and returns the signed 32-bit product in d0.
- `signed_32bit_multiply_d0_d1`: Multiplies the 32-bit values in d0 and d1 using 16-bit mulu partial products, negates the result when d3 is nonzero, restores d2-d3 and returns the product in d0.
- `signed_32bit_divide_d0_by_d1`: Divides the 32-bit value in d0 by d1, using divu when the divisor fits in 16 bits and a 16-step shift-subtract loop otherwise, applies sign corrections from d4 and d5, and returns quotient in d0 and remainder in d1.
- `unsigned_divide_shift_subtract_loop`: Performs a 16-iteration shift-subtract division of the 32-bit d0 by d1, then negates the quotient and remainder according to the sign flags in d4 and d5 before restoring d2-d5 and returning.
- `alloc_mem_node_and_link_head`: Calls AllocMem for 12 bytes, stores the caller's pointer at offset 8, adds the node to the list at a5+19744 via AddHead, and returns the node body at offset 12.
- `free_mem_node_and_clear_pointer`: If the pointer at the argument is nonzero, removes the node from the list, frees the block 12 bytes before it with FreeMem, and clears the caller's pointer.
- `set_flag_and_call_stored_vector`: Writes -1 to a5+19716, loads the function pointer from a5+19708 into a4, calls it, and returns.
- `install_software_interrupt_vector`: Reads ExecBase, gets the interrupt server chain, saves the old level-3 vector, and patches it with the address of the new interrupt handler before returning via rte.
- `install_message_port_interrupt_handler`: On first call saves d0 and a0, sets up a message port and interrupt structure, swaps the task's trap handler, and checks the task's message queue.
- `decrement_timer_and_flush_message_list`: Decrements the word timer at offset 19780 from a5; when it reaches zero, walks the linked list at 19744(a5), calling ReplyMsg on each node's message, then stores three values into the structure at 19736(a5) and, if the byte at 19730(a5) is set, calls Forbid and FreeMem on 19732(a5), returning 19716(a5).
- `open_intuition_library_once`: Increments the counter at offset 19788 from a5; on the first call it runs two setup routines, opens intuition.library version 33 via exec, stores the base at 19784 from a5, and on failure clears 19716 and jumps through the pointer at 19708.
- `close_intuition_and_shutdown`: Decrements the counter at offset 19788 from a5; when it reaches zero it closes the library at 19784 from a5 if open, calls the teardown routine, and jumps to the exit routine.
- `increment_counter_and_call_setup`: Increments the counter at offset 19754 from a5 and, only when it equals one, calls the setup routine at the given target before returning.
- `decrement_counter_and_jump_to_3084`: Subtracts one from the word at offset 19804 in a5; if the result is nonzero it branches back, otherwise it jumps to the routine at 3084.
- `open_dos_library_on_first_call`: Increments the word at 19804 in a5; on the first call it calls a setup routine, opens dos.library version 33 via exec, stores the base at 19800, and jumps to a stored pointer if opening fails.
- `save_registers_call_31d4_restore`: Pushes d1 through d7 and a2 through a6 onto the stack, calls the routine at 31d4, restores those registers from the stack and returns.
- `lzss_decompress_to_buffer`: Reads compressed bytes from the stream at a0, writes decompressed bytes backwards into the buffer ending at a1, and returns when the write pointer passes the start pointer a2.
- `set_d0_to_one`: Loads the immediate value one into d0 and falls through into the following bit-reading routine.
- `read_bits_into_d1`: Reads d0 bits from the bit stream in d5 and a0 into d1, refilling d5 from memory and resetting d7 to 32 when the bit counter runs out.
- `movep_word_from_a2_offset_to_d4`: Reads a word from offset 176 of a2 using movep into d4, then executes a no-op.
- `signal_task_when_message_matches`: Walks a linked list from a1, and when a node has type 16 and word 69, clears that word, signals the task bitmask from node offset 2 via exec.Signal, and clears the node flag.
- `advance_list_and_return_node_in_d0`: Loads the next node pointer from a0 into a0 and d0, branches back if nonzero, otherwise pops d0 and a3 from the stack and returns.
- `clear_two_buffers_and_dispatch_commands`: Saves a5, sets up a frame, multiplies d1 by the word at a1, clears two buffers of that many longwords each, then reads command bytes from a0 and branches to command handlers until a zero byte.
- `convert_control_code_to_offset`: Branches if the byte in d0 is above the control range, otherwise subtracts 17, sign-extends it, and computes a scaled offset in d4 before branching back to the dispatcher.
- `decrement_command_byte_and_redirect`: Subtracts one from the byte in d0, branches if it is not zero, otherwise loads a new a5 pointer and branches back to the command dispatcher.
- `decrement_d0_or_load_a5_table_341e`: Subtracts one from the low byte of d0; if the result is nonzero it branches ahead, otherwise it loads the address of a table into a5 and branches to another routine.
- `decrement_d0_or_load_a5_table_3444`: Subtracts one from the low byte of d0; if the result is nonzero it branches ahead, otherwise it loads the address of a second table into a5 and branches to another routine.
- `decrement_d0_or_load_a5_table_346a`: Subtracts one from the low byte of d0; if the result is nonzero it branches ahead, otherwise it loads the address of a third table into a5 and branches to another routine.
- `decrement_d0_select_first_table`: Subtracts one from the low byte of d0; if the result is nonzero it branches ahead, otherwise it loads the address of the first data table into a5 and branches to shared code.
- `decrement_d0_select_second_table`: Subtracts one from the low byte of d0; if the result is nonzero it branches ahead, otherwise it loads the address of the second data table into a5 and branches to shared code.
- `decrement_d0_select_third_table`: Subtracts one from the low byte of d0; if the result is nonzero it branches ahead, otherwise it loads the address of the third data table into a5 and branches to shared code.
- `decrement_d0_select_a5_table_356c`: Subtracts one from the low byte of d0; if the result is nonzero it branches ahead, otherwise it loads the address of the table at 356c into a5 and branches to the shared code at 32fa.
- `decrement_d0_select_a5_table_35be`: Subtracts one from the low byte of d0; if the result is nonzero it branches ahead, otherwise it loads the address of the table at 35be into a5 and branches to the shared code at 32fa.
- `decrement_d0_select_a5_table_35e4`: Subtracts one from the low byte of d0; if the result is nonzero it branches ahead, otherwise it loads the address of the table at 35e4 into a5 and branches to the shared code at 32fa.
- `decrement_d0_or_load_a5_363a`: Subtracts one from the low byte of d0; if the result is nonzero it branches ahead, otherwise it loads the address 363a into a5 and branches to the shared code at 32fa.
- `decrement_d0_or_load_a5_365e`: Subtracts one from the low byte of d0; if the result is nonzero it branches ahead, otherwise it loads the address 365e into a5 and branches to the shared code at 32fa.
- `decrement_d0_or_load_a5_36b0`: Subtracts one from the low byte of d0; if the result is nonzero it branches ahead, otherwise it loads the address 36b0 into a5 and branches to the shared code at 32fa.
- `decrement_d0_then_load_a5_3706_or_branch`: Subtracts one from the low byte of d0; if the result is nonzero it branches ahead, otherwise it loads the address 3706 into a5 and branches to the shared continuation.
- `decrement_d0_then_load_a5_3758_or_branch`: Subtracts one from the low byte of d0; if the result is nonzero it branches ahead, otherwise it loads the address 3758 into a5 and branches to the shared continuation.
- `load_a5_37a8_and_branch_to_shared_code`: Loads the address 37a8 into a5 and then branches unconditionally to the shared continuation at 32fa.
- `compute_tile_row_pointer_and_jump`: Subtracts 32 from d0, masks it to a byte, shifts left by 3, adds d4, then loads a3 from a2 plus d0 and a6 from a3 plus 6144 before jumping through a5.
- `branch_to_shared_tile_dispatch`: Contains only an unconditional branch to a shared dispatch routine, transferring control without touching any registers or memory.
- `copy_eight_bytes_strided_then_branch`: Copies eight bytes from a6 into a1 with d2 added to a1 after each byte, masks the last byte with d7, subtracts d1 from a1 and branches to the shared dispatch.
- `copy_eight_bytes_strided_and_mask`: Copies eight bytes from (a6) into (a4) with stride d2, ANDs the last byte with d7, subtracts d1 from a4 and branches to the shared dispatcher.
- `copy_eight_bytes_to_two_strided_buffers`: Copies eight bytes from (a6) into both (a1) and (a4) at stride d2, ANDs the last bytes with d7, then branches to the shared dispatcher.
- `copy_eight_bytes_strided_and_or`: Copies eight bytes from (a3) into (a1) with stride d2, ORs the last byte with d6, subtracts d1 from a1 and branches to the shared dispatcher.
- `fill_seven_columns_with_ff`: Writes 255 to a1 and to seven successive columns spaced by d2, then branches to the shared command dispatcher.
- `copy_seven_columns_from_two_sources`: Copies eight bytes from a3 into a1 and from a6 into a4 at d2-spaced columns, then ORs d6 and ANDs d7 into the last column before branching to the dispatcher.
- `fill_a1_and_copy_a6_into_a4`: Writes 255 to a1 and copies eight bytes from a6 into a4 at d2-spaced columns, then ANDs d7 into the last column and branches to the dispatcher.
- `copy_eight_bytes_strided_to_a4`: Copies eight bytes from (a3)+ into a4 at stride d2, ORs d6 into the last byte, then subtracts d1 from a4 and branches to the shared dispatcher.
- `copy_eight_bytes_strided_to_a1_and_a4`: Copies eight bytes from (a6)+ into a1 and from (a3)+ into a4 at stride d2, ANDs d7 into a1's last byte, ORs d6 into a4's last byte, then branches to the dispatcher.
- `fill_eight_strided_bytes_with_ff`: Writes the byte value 255 into eight positions of a4 spaced by d2, then subtracts d1 from a4 and branches to the shared dispatcher.
- `copy_8_rows_masked_from_a6_to_a1`: Copies eight bytes from the a6 stream into a1 at stride d2, sets matching bytes in a4 to all ones, masks the last a1 byte with d7, then branches to the shared dispatcher.
- `copy_8_rows_from_a3_to_a1_and_a4`: Copies eight bytes from a3 into both a1 and a4 at stride d2, ORs the last bytes with d6, then branches to the shared dispatcher.
- `fill_a1_ones_copy_a3_to_a4`: Writes all-ones bytes into a1 at stride d2 while copying eight bytes from a3 into a4, ORs the last a4 byte with d6, then branches to the shared dispatcher.
- `copy_seven_bytes_and_set_masks`: Copies seven bytes from (a3)+ into a1 at stride d2, sets matching bytes in a4 to 255, ORs d6 into the last byte, then branches to a shared dispatcher.
- `fill_seven_bytes_with_ff_masks`: Writes 255 to a1 and a4 at stride d2 for eight positions, advances a1 and a4 by one, then branches to a shared dispatcher.
- `pop_a5_and_return`: Pops a5 from the stack and returns immediately to the caller; it is an empty routine that only restores the frame pointer.
- `expand_planar_tiles_to_bitplanes`: Reads 192 bytes from a0, expands each into four bitplane copies at offsets in a1, advances a0 at 48/96/144, then inverts 1536 longwords into a1+6144.
- `copy_two_streams_forward_after_setup`: Calls the setup routine, then copies d0+1 longwords from a1 to a3 and from a2 to a4, advancing all four pointers.
- `copy_two_streams_backward_after_setup`: Calls the setup routine, then copies d0+1 longwords from a3 to a1 and from a4 to a2, advancing all four pointers.
- `compute_tile_plane_offsets`: Scales d0 by eight, multiplies d2 and d1 by that value, and stores the resulting byte offsets from a3 and a4 into a3, a4, a1 and a2, then returns d0 shifted back down minus one.

### Loader

- `load_iff_screen_and_blit_bitmap`: Opens graphics and intuition libraries, parses the IFF FORM ILBM chunks, builds a screen from the CAMG and BODY data, blits the bitmap into it, then closes the screen and libraries and returns zero.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_8`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_9`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_10`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### mathtrans.library

- `mathtrans_library_illegal_trap`: Executes an illegal instruction word as the only instruction, which is library stub data rather than executable code and traps if run.
- `store_a0_to_library_global`: Copies the a0 register into the library global longword at offset 408, then returns to the caller.
- `close_library_and_free_memory`: Checks the library open count, unlinks the library node from its list, calls exec FreeMem on the library base, then returns the stored global longword in d0.
- `set_library_flag_bit3_and_return_zero`: Sets bit 3 of the byte at offset 14 from a6, returns zero in d0, and returns to the caller.
- `mathtrans_open_library_once`: Library open routine: if bit 3 of the flags word at 14(a6) is clear, increments the open count at 32(a6) and, on the first open, saves the base pointer and calls the mathffp library open with a null argument, storing the result; returns the library base in d0 or minus one if already open.
- `mathtrans_open_failure_return_minus_one`: Loads minus one into d0 and returns immediately, signalling a failed library open to the caller.
- `mathtrans_close_decrement_refcount`: Clears d0, decrements the word at offset 32 of the library base in a6, and when it reaches zero with bit 3 of offset 14 set calls the close routine, then calls the expunge routine with the stored pointer and returns the saved d0.
- `mask_sign_bit_and_call_core`: Clears the top bit of the byte in d0, calls the math conversion core, sets the carry flag in the condition codes and returns to the caller.
- `convert_positive_float_to_fixed`: Takes a positive single-precision float in d0, saves d4-d6, normalizes the mantissa and exponent, and branches into the shared fixed-point conversion loop.
- `fixed_point_division_loop`: Iteratively shifts d5 and d6, subtracts the divisor in d0, sets bits in d0, then packs the quotient byte and restores d4-d6 before returning.
- `save_d0_test_byte_call_and_set_flags`: Copies d0 into d7, tests the low byte of d7, calls the library vector at offset 20, then sets bit 1 and clears bit 3 of d0 before branching away.
- `mathtrans_constant_sequence_via_vectors`: Saves d6, then repeatedly loads d3, d5 and immediate constants into d0/d1 and calls library vectors at negative a5 offsets, finally restoring d6 and returning.
- `clear_d1_then_branch_to_shared_code`: Clears d1 to zero and branches forward into shared library code at a later offset, leaving the rest of the work to that target.
- `mathtrans_compare_and_transform_vectors`: Saves d2-d6/a5, loads library base from absolute memory into a5, calls math library vectors via offsets to compare and transform d5/d7 values, returning results in d2/d3.
- `mathtrans_negative_branch_transform`: Copies d6 to d4, compares d5 against zero via library call, and if positive transforms d5 through two library vectors, storing results in d3 and d2.
- `mathtrans_conditional_angle_transform`: Copies d5 to d2, tests d6 and d4, applies library vector calls with a constant angle to d2, then branches based on d7 sign and returns transformed value.
- `scale_d1_by_pi_and_divide_d2`: Calls library vector with d7 in d1, tests d4, then loads the pi constant into d0 and d1, calls two more library vectors, and stores the result in d2.
- `convert_d2_via_library_vector`: Moves d2 into d0, calls the library vector at offset -60 from a5, then copies the returned d0 back into d2.
- `negate_pi_constant_when_d4_set`: Moves d2 into d0, tests d4, and if d4 is nonzero loads the negated pi constant into d2 before branching onward.
- `math_trans_library_twice_call_and_mask`: Loads a fixed long constant into d2, calls the library vector at offset minus 66(a5) twice with d1 set from d2, moves the result through d2 and d7, calls another routine, then masks d0 with 245 and sets the condition codes from it before restoring registers.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `math_trans_library_float_decode_dispatch`: Saves d1-d6 and a0, strips the sign bit from d7, compares it against a packed constant, optionally negates and swaps values, calls a conversion routine, then computes an exponent and mantissa loop using shifts on d6 and d7.
- `mathtrans_setup_division_constants`: Shifts d7 right by one, loads zero into d6, loads the constant an address into d5, points a0 at address 4, sets d1 to 24 and d2 to 1, then branches forward to continue in another routine.
- `cordic_atan_rotation_loop`: Iteratively rotates a vector using arithmetic shifts by d2, accumulating angle in d5 and residual in d7, stepping a0 by four bytes and d2 by one until dbra d1 expires, then calls library helpers and restores d1-d6/a0.
- `push_negative_two_and_branch`: Pushes the immediate word value minus two onto the stack and branches forward into shared library code, acting as a small entry stub.
- `push_negative_one_and_branch`: Pushes the immediate word value minus one onto the stack and branches forward into shared library code, acting as a small entry stub.
- `push_one_word_and_branch_to_shared_tail`: Pushes the immediate word value one onto the stack and branches forward into shared library code, without returning here.
- `compare_d7_byte_and_discard_stack_word`: Compares the low byte of d7 with the decimal value 184, branches away if higher, otherwise adds two to a7 to drop a stack word, tests d7 and returns.
- `dispatch_d7_angle_to_sine_cosine_tail`: Tests the low byte of d7, saves registers, scales d7 and branches to shared sine or cosine code, restoring registers and returning when out of range.
- `scale_angle_by_power_of_two`: Shifts the low byte of d2 to build a scale factor in d7, calls the fixed-point multiply and divide helpers, then reduces d7 modulo twice the pi constant in d4.
- `negate_angle_scale_loop`: Negates the low byte of d2, shifts d7 right by that count, negates d7 and d4, then subtracts twice d4 from d7 until it is below d4.
- `cordic_rotate_accumulate`: Doubles d7 three times, sets up the arctangent table pointer in a0 and iterates 24 times, conditionally adding or subtracting shifted d6 and d4 into d5 and d6.
- `cordic_rotate_loop_with_quadrant_fixup`: Iteratively rotates a vector using add/sub and arithmetic shifts on d3-d7, then adjusts the angle in d1 and calls the arctangent helper before restoring registers and returning.
- `set_d7_negative_angle_limit`: Loads the immediate value -129 into d7 and branches back into the surrounding rotation loop to force a negative angle result.
- `finish_rotation_copy_and_return`: Copies d5 to d6, calls the arctangent helper, moves d6 to d7, tests the low byte of d7, then restores d1-d6/a0 and returns.
- `save_d5_call_15c_store_d6_restore`: Pushes d5 onto the stack, calls the routine at the next address, stores d6 into the caller's stack slot, then pops d6 and branches onward.
- `clamp_d6_to_byte_range`: Copies d6 to d4, branches if negative, compares d6 with 255 and branches if above, otherwise clears d6 and returns.
- `shift_d4_arith_add_one_jump`: Arithmetically shifts d4 right by eight bits, adds one to d4, branches if the result is zero, otherwise jumps to an absolute address.
- `math_trans_negate_checked`: Saves d6, masks d7 to seven bits, calls the library conversion helper, and if the result is valid negates it via the math library, returning zero on overflow.
- `math_trans_negate_keep_d7`: Saves d6, calls the conversion helper on d7, negates the value through the math library, then restores d7 from the stack and returns the negated result in d6.
- `math_trans_increment_checked`: Saves d6, tests d7, increments it and calls the conversion and math helpers, returning the incremented value or zero on overflow.
- `load_roxr_byte_constant_d7`: Loads the immediate value an address into d7, rotates d7 right through extend by one bit as a byte operation, then branches to a shared continuation.
- `load_immediate_d7_then_branch`: Loads the immediate value an address into d7 and branches unconditionally to a shared continuation address.
- `pop_word_test_sign_set_d7`: Pops a word from the stack into d6, tests its low byte, and if non-negative sets d7 to zero before branching to a shared continuation.
- `restore_registers_and_return`: Loads a small immediate into d7, shifts it right by one, sets the carry flag via ori on the condition code register, pops d1-d6 and a0 from the stack, then returns to the caller.
- `load_immediate_skip_stack_and_test`: Loads a long immediate into d7, advances the stack pointer by thirty bytes, tests the low byte of d7 setting condition codes, then returns to the caller.
- `math_trans_core_conversion`: Saves d1-d6 and a0, tests d7, calls library math routines to convert and scale the value, adjusts the stack copy, and returns the converted result in d7.
- `compute_sine_cosine_series`: Builds a fixed-point sine/cosine value in d6 from the angle in d2, calling the series routine three times with different precision counts and adjusting the sign from the stack byte before returning.
- `accumulate_positive_series_terms`: Loops d1 times adding shifted copies of d5 and d6 into themselves and subtracting successive values from d7, used for positive-angle series terms.
- `accumulate_negative_series_terms`: Loops d1 times subtracting shifted d5 and d6 values and adding successive values to d7, used for negative-angle series terms.
- `test_d7_sign_and_call_negate`: Tests byte in d7; if zero returns, if positive branches to shared code, if negative clears the sign bit, calls the negate routine and sets the carry flag before returning.
- `set_d7_minus_one_and_branch`: Loads minus one into d7 and branches forward into the shared conversion code, used as an entry point for a default value.
- `convert_angle_to_sine_cosine`: Saves registers, calls library math routines with d7 and constants, then computes a scaled sine or cosine value into d7 using shifts and subtraction.
- `decode_float_mantissa_exponent`: Shifts and subtracts to normalize a floating-point mantissa in d5/d7, adjusts exponent in d2, then calls library helpers to convert and store the result.
- `dispatch_math_op_via_pc_jsr`: Calls a nearby routine through pc-relative jsr, then two absolute library helpers, loads a constant into d6 and moves d0 into the condition codes before returning.
- `test_d7_sign_and_call_helper`: Tests bit 7 of d7, clears it if set, calls a nearby helper, sets the overflow flag in the condition codes and returns.
- `library_trampoline_via_vectors`: Calls the routine at address zero, saves d3-d5 on the stack, calls the routine at the second vector, restores d3-d5 and jumps to the third vector.
- `encode_float_exponent_byte`: Doubles d7, and if nonzero flips bit 7, shifts the byte right arithmetically, subtracts 130, swaps halves and rotates the long word left by 7 bits.
- `decode_float_exponent_byte`: Swaps d7 halves, rotates right by 7, flips bit 7, doubles the byte, adds 5 on overflow, flips bit 7 again and rotates right by 1.
- `normalize_d7_byte_and_rotate`: Branches on carry, compares d7 byte against 124 and 126, adds 133 to d7 for 126, rotates d7 right one bit and tests the low byte, branching if nonzero.
- `clear_d7_if_low_byte_zero`: Clears bit 8 of d7, tests the full longword and branches if zero, otherwise tests the low byte and sets d7 to zero before branching.
- `check_d7_byte_254_and_shift`: Compares the low byte of d7 with 254, and if equal shifts d7 right by 8 then by 1 bit, branching based on whether the result is zero.
- `shift_d7_byte_and_set_carry`: Shifts d7 left by eight bits, loads 255 into d7, rotates the low byte right through extend, sets the carry flag via ccr, then branches forward to shared code.
- `zero_d7_and_branch_forward`: Loads zero into d7 and branches forward to shared library code, providing a zero return value for the caller.
- `save_regs_call_a0_restore_d0`: Saves d3-d7 on the stack, copies d0 to d7, clears d1, calls the routine pointed to by a0, then restores d0 and d3-d7 before returning.
- `save_regs_call_a0_restore_d0_2`: Pushes d3 through d7, saves d1 and d0 in d6 and d7, clears d1, calls the routine pointed to by a0, then restores d0 and d3 through d7 from the stack and returns.
- `load_a0_offset_and_branch`: Loads a0 with the absolute address 302 and branches to a nearby target, acting as a library jump stub rather than a full routine.
- `load_a0_offset_and_branch_wide`: Loads a0 with the absolute address 648 and performs a wide branch to a nearby target, serving as a library dispatch stub.
- `load_offset_652_into_a0_and_branch`: Loads the immediate value 652 into address register a0 and branches unconditionally to another library routine.
- `load_zero_into_a0_and_branch`: Loads the immediate value zero into address register a0 and branches unconditionally to another library routine.
- `load_offset_30_into_a0_and_branch`: Loads the immediate value 30 into address register a0 and branches unconditionally to another library routine.
- `load_offset_c_table_and_branch`: Loads the address of a table at offset twelve into a0 and branches to the shared library entry point, passing that pointer.
- `load_offset_six_table_and_branch`: Loads the address of a table at offset six into a0 and branches to the shared library entry point, passing that pointer.
- `call_vector_store_result_via_a0`: Saves d3-d7, keeps d0 in d7, pushes d1, zeroes d1, calls the routine pointed to by a0, then stores d7 through the popped pointer and returns d6 in d0.
- `load_offset_52_into_a0_and_branch`: Loads the immediate value 52 into address register a0 and branches to a shared library routine, passing a0 as an argument.
- `load_zero_into_a0_and_branch_2`: Loads the immediate value zero into address register a0 and branches to a shared library routine, passing a0 as an argument.
- `load_offset_110_into_a0_and_branch`: Loads the immediate value 110 into address register a0 and branches to a shared library routine, passing a0 as an argument.
- `load_offset_38_into_a0_and_branch`: Loads the immediate value 38 into address register a0 and branches to a shared library routine, part of mathtrans.library.
- `load_zero_into_a0_and_branch_3`: Loads the immediate value zero into address register a0 and branches to a shared library routine, part of mathtrans.library.
- `load_offset_204_into_a0_and_branch`: Loads the immediate value 204 into address register a0 and branches to a shared library routine, part of mathtrans.library.
- `load_a0_zero_and_branch_forward`: Loads the immediate value zero into address register a0, then branches forward by twenty bytes to continue library execution.
- `load_a0_zero_and_branch_back`: Loads the immediate value zero into address register a0, then branches backward to an earlier point in the library code.
- `load_a0_twenty_and_branch_back`: Loads the immediate value twenty into address register a0, then branches backward to an earlier point in the library code.
- `load_math_offset_minus_thirty_into_a0`: Loads the address thirty bytes before the math library base into a0 and branches to the shared dispatch code.
- `load_math_offset_minus_thirty_six_into_a0`: Loads the address thirty-six bytes before the math library base into a0 and branches to the shared dispatch code.
- `load_math_offset_minus_forty_two_into_a0`: Loads the address forty-two bytes before the math library base into a0 and branches to the shared dispatch code.
- `load_negative_offset_into_a0_and_branch`: Loads the address formed by sign-extending the immediate negative offset into address register a0, then branches to a shared continuation in the library.
- `load_math_offset_minus54_into_a0`: Loads the address a0 with a negative offset of 54 bytes from the current position and branches forward to shared library code.
- `load_math_offset_minus60_into_a0`: Loads the address a0 with a negative offset of 60 bytes from the current position and branches forward to shared library code.
- `load_negative_offset_into_a0_and_branch_2`: Loads the address formed by sign-extending the negative immediate into a0, then branches to a shared continuation in the library.
- `load_math_offset_minus72_into_a0`: Loads the address a0 with a negative offset of 72 bytes from the current position and branches forward to shared library code.
- `load_negative_offset_into_a0_and_branch_3`: Loads the sign-extended immediate value minus seventy-eight into address register a0, then branches unconditionally to another library routine.
- `call_math_trans_dispatch_with_d6_d7`: Saves d0, d1 and a0, adds the library base from memory to a0, moves d7 into d0 and d6 into d1, calls the library vector through a0, then restores the saved registers and returns.
- `call_math_trans_dispatch_with_d7`: Saves d0 and a0, adds the library base from memory to a0, moves d7 into d0, calls the library vector through a0, stores the result back into d7, restores d0 and a0 and returns.
- `normalize_long_to_float_exponent`: Converts the long in d6 into a packed float-like value in d6, handling sign, zero, small magnitudes and normalization shifts, and returns the exponent byte in d6.

### mathieeedoubbas.library

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.
- `mathieee_open_resource_and_init_lib`: Stores a0 in a global, saves d2-d3/a2, opens the MathIEEE resource via library offset -498, reads a longword from offset 16 of the result into d2, and sets up a0 for the caller.
- `mathieee_alloc_and_fill_library_base`: Allocates 60 bytes with library offset -84, stores handler pointers at offsets 52 and 56, copies flags and saved d2/d3/a6 into the new base, optionally calls a cleanup vector, then frees it via offset -396.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `mathieee_set_error_and_trap_on_code`: Writes the value 2 to offset 2 of a0, masks d7 to a byte, and if d7 equals 50 divides by zero, or if it equals 53 sets the overflow flag and traps.
- `check_d7_against_1c00_then_return`: Compares the low word of d7 with the immediate value 7168 and branches ahead if it is greater or equal, otherwise returns to the caller.
- `check_d7_against_1e00_then_branch_back`: Compares the low word of d7 with the immediate value 7680 and branches ahead if greater or equal, otherwise branches back to an earlier library routine.
- `check_d7_against_5c00_then_return`: Compares the low word of d7 with the immediate value 23552 and branches ahead if it is greater or equal, otherwise returns to the caller.
- `check_d7_against_5e00_then_branch`: Compares d7 with the immediate value 5e00 and branches to one of two library locations depending on whether d7 is greater or equal.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `save_d7_and_poll_mathieee_status`: Pushes d7, loads the math library base from a6, sets a status word at offset 10 of the io area, reads the status word into d7 and branches based on its value, calling a helper when positive.
- `mathieee_write_result_and_poll_status`: Checks d7 against c900, stores d0 through a1, waits on the status word in a0, writes 7400 to offset 10, rereads status and branches or calls the helper accordingly.
- `check_d7_c900_and_copy_a1_longwords`: Compares d7 with the immediate value c900 and branches if equal; otherwise copies the longword at a1 into d0 and again into d1, tests the word at a0 and branches if negative, then pops d7 and returns.
- `clear_ieee_double_rounding_mode_word`: Saves d7, loads the library base from a6 offset, clears a word at offset sixteen, reads it back masked to the high byte and returns if nonzero.
- `test_a0_word_and_dispatch_on_d7_status`: Tests the word at a0 and branches if negative, writes 9000 to offset ten of a0, loads the word at a0 into d7, compares it with 8900 and branches, tests d7 and branches, otherwise calls the routine at 34e.
- `check_d7_and_store_error_code`: Compares d7 with the immediate value 51456, branches if equal, stores 144 into the longword at a1, tests the word at a0 and clears d0 if non-negative, then pops d7 and returns.
- `ieee_double_divide_core`: Library code that strips sign bits from d0 and d2, compares exponents against 32752, and performs the IEEE double division mantissa steps using d6, d7, a0 and d1.
- `compare_exponents_and_copy_result`: Compares d2 with d7 and a0, tests d3, and if the checks pass copies d2 into d0 and d3 into d1 before branching to the shared return path.
- `branch_to_ieee_double_constant`: Branches unconditionally to another library routine, transferring control without touching registers or memory itself.
- `return_zero_if_double_operands_zero`: Tests d2 and d3, branching away if either is nonzero; otherwise copies a0 to d0, xors d4 into the low word, clears d1, swaps d0 halves, restores d2-d7 and returns.
- `branch_to_ieee_double_compare`: Branches unconditionally to another library routine, transferring control without modifying registers or memory itself.
- `compare_exponents_and_mantissas`: Compares d7 with d2 as words, then compares a0 with d2 as longs, tests d3, and if d3 is nonzero copies d2 into d0 and d3 into d1 before branching away.
- `branch_to_shared_return_path`: Branches unconditionally to another library routine, transferring control without touching any registers or memory itself.
- `test_operands_for_zero_and_branch`: Masks d0 with d7 into d5, then tests d0, d1, d2 and d3 in turn, branching to shared code when any is nonzero and otherwise branching to another path.
- `branch_to_library_offset`: Branches unconditionally to another location in the mathieeedoubbas library, transferring control without touching registers or memory.
- `call_math_helper_then_branch`: Calls a mathieeedoubbas library helper via bsr, then branches unconditionally to another library location; no registers or memory are directly modified here.
- `xor_and_test_double_words`: Exclusive-ors d5 into d0, flips bit four of d0, ands d2 into d7, then tests d2 and d3 and branches based on their values.
- `swap_regs_call_swap_back`: Exchanges d0 with d2, d1 with d3 and d5 with d7, calls a library routine, then swaps the same register pairs back and branches onward.
- `adjust_exponent_and_subtract_mantissa`: XORs d7 into d2, flips bit four of d2, subtracts an exponent bias from d7, subtracts d7 from d5 and branches on overflow.
- `ieee_double_divide_mantissa_loop`: Library code that normalizes and divides the mantissas in d0/d1 by d2/d3 using divu, mulu and addx/subx steps, producing the quotient in d0/d1.
- `ieee_double_divide_quotient_round`: Library code dividing a double-precision value by another using mulu and divu on d0-d3, adjusting the quotient in d4 and returning the packed result in d0/d1.
- `ieee_double_divide_early_exit`: Library code performing the same double-precision division on d0-d3 but branching to a shared normalization path when the divu result overflows, returning the packed quotient in d0/d1.
- `ieee_double_divide_normalize_result`: Library code finishing a double-precision division by shifting the quotient in d0/d1 left with roxl, loading a1 into d4/d5 and jumping to the shared normalization routine.
- `check_shift_count_against_57`: Negates the low word of d5, shifts it right by four bits, adds two, then compares the result with 57 and branches to one of two library targets depending on whether it is less than or equal.
- `normalize_mantissa_and_xor_sign`: Loops subtracting 16 from d5 while positive, then shifts and rotates d0, d1 and d2 by d5, XORs them together, applies d4 to the sign word, restores d2-d7 and returns.
- `shift_mantissa_right_one_bit`: Shifts d0 right by one bit, rotates d1 right through the extend bit, then tests the low word of d5 and branches to one of two library targets depending on its sign.
- `ieee_double_denormal_shift_round`: Shifts the 64-bit value in d0/d1 right by the exponent in d5, adjusts the exponent, compares against a0, applies sign from d4 and restores d2-d7 before returning.
- `ieee_double_overflow_return`: Branches to the overflow handler, returning the overflow result built from d4 with the sign bit flipped and all fraction bits set.
- `ieee_double_normalize_and_pack`: Normalizes the mantissa in d0/d1 using the leading-zero table at a1, adjusts exponent d5 and d4, XORs the shifted mantissa halves and returns the packed result.
- `ieee_double_divide_normalize`: Normalizes the dividend in d0 and divisor in d1, adjusts exponent in d5, XORs sign bits from d4, and returns the quotient in d0/d1 with the sign applied.
- `ieee_double_divide_denormal_shift`: Shifts d0 and d1 right by the negated exponent in d5, XORs the shifted values together, and applies the sign from d4 before returning.
- `ieee_double_return_zero`: Sets d0 and d1 to zero, XORs the sign bit from d4 into d0, restores d2 through d7 from the stack and returns.
- `return_zero_double_with_d4_sign`: Loads zero into d0 and d1, XORs the low word of d4 into d0, swaps d0, restores d2-d7 from the stack and returns, producing a signed zero double.
- `return_negative_infinity_double`: Loads the negative infinity bit pattern into d0 and d1, XORs d4 into the low word of d0, swaps d0, restores d2-d7 and returns the double.
- `return_positive_infinity_double`: Loads the positive infinity bit pattern into d0 and d1, XORs d4 into the low word of d0, swaps d0, restores d2-d7 and returns the double.
- `return_positive_infinity_double_2`: Loads the double-precision positive infinity bit pattern into d0, clears d1, restores d2 through d7 from the stack and returns to the caller.
- `normalize_double_exponent_bits`: Tests bit 3 of d0, clears it and sets bit 1 when present, ORs the exponent bits into the low word, swaps d0 halves, restores d2 through d7 and returns.

### diskfont.library

- `diskfont_library_entry_branch`: Library entry point that branches forward to the startup code at offset 80, which then calls library base functions through a6.
- `open_diskfont_library_and_init_base`: Saves a0 and a2, calls the library open function with size 56 and a name, stores a0 at offset 34 of the new base, calls the close function and returns 1 on success.
- `init_diskfont_base_and_open_libs`: Stores the incoming base and a6 in globals, opens dos.library and graphics.library via exec.OpenLibrary, initializes a list at offset 42 of the base and returns the base pointer.
- `close_diskfont_libs_and_free_base`: Closes the library stored in the global, computes the base size from its word fields, frees the base memory via exec.FreeMem and returns zero.
- `diskfont_expunge_fonts`: Walks the font list at offset 42 of the library base, unlinks fonts with zero usage, calls UnLoadSeg on their segment, and decrements the open-font count.
- `diskfont_close_library_cleanup`: If no fonts remain open, closes the two cached libraries, frees the library base memory, unlinks the base from its list and returns the saved value.
- `diskfont_mark_lib_open_return_zero`: Sets bit 7 of the flags byte at offset 14 of the library base and returns zero in d0.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `clear_afp_bit7_and_increment_counter`: Clears bit 7 of the byte at offset 14 in the a6 library base, increments the word at offset 38, and returns the a6 base in d0.
- `decrement_counter_and_jump_cleanup`: Decrements the word at offset 38 in a6; if it reaches zero and bit 7 at offset 14 is set, jumps through the vector at offset -18 in a6.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `build_font_list_from_avail_fonts`: Allocates a stack frame, scans the available-fonts list, copies font names and attributes into the caller's buffer, and optionally queries disk fonts via library calls.
- `load_font_list_head_and_set_flag`: Loads the longword at offset 10 of the library base in a4 into a1, sets d4 to 1, then branches to a shared continuation.
- `scan_font_name_and_advance_list`: Increments d4 while scanning bytes at a1 until a zero terminator, then adjusts d5 and d2 by d4 and ten, and branches based on the comparison.
- `copy_font_name_and_build_font_entry`: Copies bytes from a1 into the buffer at a3 plus d5, writes a font entry header at a2, advances the list in a4, and may open the FONTS: directory via library calls.
- `open_font_and_check_metric`: Pushes the font name and size onto the stack, calls the library open routine, and returns the font pointer in d3 or zero if opening fails.
- `measure_font_name_length`: Scans the null-terminated font name at a1 counting bytes into d4, then subtracts that length from d5 and d2 and branches to the glyph loop.
- `copy_font_name_bytes`: Copies bytes from a1 into the buffer at a3 plus d5 until the count in d4 is reached, then reads four bytes from the font handle into the stack buffer.
- `build_avail_font_entry_list`: Repeatedly reads font records into a stack buffer, and for each valid record writes a type, pointer, size and style byte into the caller's output array, counting entries in the word at d6.
- `return_zero_restore_regs_and_frame`: Sets d0 to zero as the return value, restores d2 through d6 and a2 through a4 from the stack frame, releases the a6 frame and returns to the caller.
- `compute_font_match_score`: Compares a font request against a font record's size and style bytes, accumulating a weighted score in d2 from the style difference and returns it in d0.
- `kerning_pair_bit_accumulate`: Reads two kerning bytes from the font and glyph structures, XORs them, and for each set bit subtracts a table entry from d2, returning the adjusted advance in d2.
- `kerning_second_byte_accumulate`: Subtracts a table entry indexed by d4 from d2, then tests the second kerning byte bits and subtracts further entries, returning the adjusted value in d2.
- `kerning_table_subtract_and_advance`: Subtracts the table entry at offset d4 times four from d2, doubles d3, increments d4, and branches back while d4 is below seven.
- `return_diskfont_error_code`: Loads the constant value -32767 into d2 and copies it to d0 as the return value, then restores d2 through d6 from the stack and returns to the caller.
- `open_diskfont_and_find_best_size`: Saves registers, copies the caller's font spec, opens the font, reads its size fields, then scans available sizes for the best match and returns the chosen font pointer.
- `open_fonts_directory_and_store_result`: Pushes the FONTS: directory name and a zero argument, calls the open routine, stores the returned handle in the local frame and branches based on whether it is null.
- `load_font_file_and_pick_best_face`: Opens the font file, reads the header, allocates a buffer for all faces, loads them, then scores each face against the caller's text and returns the best matching face pointer.
- `init_face_search_registers`: Clears d4 and d3 to zero, sets d2 to minus one, then branches into the face scoring loop to start the search.
- `score_faces_and_open_best_match`: Loops over font faces scoring each against the caller's text, then opens the best face, links it into the library list and returns its pointer.
- `clear_font_entry_flag_and_chain`: Saves d4 on the stack, calls a library routine, computes a font entry address from d2 times 260 plus d5, clears the word at offset 256, then jumps onward.
- `release_font_entry_and_restore_regs`: Clears the word at offset 256 of the font entry indexed by d2, optionally frees d3, and on negative d2 frees several saved pointers before restoring registers and returning a4.
- `load_diskfont_from_file`: Opens the named font file, reads its header, allocates a 260-byte font structure, parses character entries into it, and returns the new font pointer or zero.
- `build_diskfont_cache_entry`: Allocates a font cache block, copies font name and tag data into it, fills per-character entries from font tags, and returns the cache pointer or zero.
- `free_font_cache_block`: Loads the pointer argument from the stack, pushes the longword stored just before it and the pointer itself, calls the memory free routine, then returns.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `call_parent_dir`: Library stub: loads its arguments and the dos library base, calls ParentDir and returns its result to the caller.
- `copy_null_terminated_string`: Copies bytes from the source string pointer to the destination pointer, incrementing both, until a zero byte is copied, then returns.
- `load_a0_from_stack_arg_and_branch`: Loads a0 from the first stack argument at offset four, loads d0 from the second at offset eight, then branches forward into shared library code.
- `scan_string_to_nul_then_call`: Advances a0 past bytes until a zero byte is found, then pushes d0 and a0, calls a library routine via pc-relative jsr, and returns after cleaning the stack.
- `load_a0_from_stack_and_zero_d0_branch`: Loads a0 from the first stack argument at offset four, sets d0 to zero, then branches forward into shared library code.
- `strlen_increment_d0_scan_a0`: Increments d0 once per character while scanning bytes at a0 until a zero byte, advancing a0 past the terminator, then returns.
- `strcmp_forward_args_to_compare`: Loads the two string pointers from the stack arguments into a1 and a0, then branches to the shared string comparison code.
- `scan_a0_until_zero_return_zero`: Reads bytes at a0, advancing a0, until a zero byte is found, then sets d0 to zero and branches onward.
- `compare_strings_case_insensitive`: Loads a byte from the string at a0, compares it with the byte at a1 and advances a1, branching if equal; otherwise compares the bytes again and returns minus one if a0's byte is lower, else falls through.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `load_string_pointer_and_byte_from_stack`: Loads a long pointer from four bytes above the stack pointer into a0 and a byte from eleven bytes above the stack pointer into d0, then branches onward.
- `compare_bytes_and_return_pointer`: Compares the byte in d0 with the byte in d1, branching if they differ; otherwise moves the long in a0 into d0 and branches onward.
- `advance_a0_scan_byte_to_d1`: Adds one to a0, loads the byte at the new a0 into d1, and if that byte is zero sets d0 to zero and returns; otherwise it branches onward.
- `save_regs_call_alloc_then_offset_a0`: Saves d2 and d3, loads a longword and a byte from the stack into d3 and d2, calls a routine with d3 pushed, then sets a0 to the returned pointer minus one plus d3 and branches on.
- `compare_byte_at_a0_with_d2`: Compares the byte at a0 with d2 and, if equal, copies a0 into d0 and branches onward; otherwise it branches to the alternate path.
- `scan_backward_for_char_above_d3`: Decrements a0 by one, compares it with d3, and branches back if a0 is still greater or equal; otherwise sets d0 to zero, restores d2-d3 from the stack and returns.
- `find_first_of_two_chars_in_string`: Saves d2, loads the string pointer from the stack, calls the search routine for character 47 then for character 58, and returns the match plus one or zero.
- `increment_match_index_and_return`: Adds one to d0, copies the result into d2 and back to d0, then pops d2 from the stack and returns to the caller.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `save_d0_d1_a0_call_vector`: Pushes d0, d1 and a0 onto the stack, calls the routine at address zero, then adds twelve to the stack pointer and returns.
- `save_a0_call_vector_offset`: Pushes a0 onto the stack, calls the routine at offset two hundred twenty-four, adds four to the stack pointer and returns.
- `save_a0_a1_call_vector`: Pushes a0 and a1 onto the stack, calls the routine at address zero, adds eight to the stack pointer and returns.
- `diskfont_jump_to_library_entry`: Pushes a1 onto the stack, calls the diskfont library entry at a fixed low address, then discards the pushed argument and returns.

### Karate Master (demo) (1998)(F1 Software)(PD).adf:KarateMaster

- `relocate_hunks_and_allocate_memory`: Program entry that walks AmigaDOS hunk tables, allocates memory with exec.AllocMem for each hunk, copies and relocates data, frees memory with exec.FreeMem and handles symbol hunks.
- `resolve_relocation_offset_and_free_memory`: Computes a relocation base from a5 and d1, calls the library free routine with the size in d0, stores the base in a4, and restores all registers before returning.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `unpack_rle_bitmap_to_a3`: Reads a run-length encoded stream backwards from a0, expands literal and copy runs into memory ending at a3, and returns after the destination pointer passes its start.
- `refill_bit_buffer_from_a0`: Shifts the d5 bit buffer right by d4 bits; if it becomes empty, loads the next longword from a0 into d5 and rotates in the carry bits.
- `subtract_d4_from_d0_word`: Subtracts the low word of d4 from the low word of d0, leaving the result in d0 and setting the condition codes.
- `decode_bitstream_to_playfield`: Shifts bits out of d5 using d4 as the bit count, refilling from memory at a0, and writes decoded bytes backwards into a3 while updating the COLOR17 register.
- `data_table_not_executable`: These four words are data bytes, not executable code; the movep, ori, trap and or instructions are meaningless when decoded as instructions.

### Karate Master (demo) (1998)(F1 Software)(PD).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
