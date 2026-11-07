# Game 1471 (boston bomb club (1990)(silmarils)[cr company][a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01471). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901200 top-level bytes in 41 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 259 of 259 named (0 author symbols), 259 of 259 described (11 by rule, 52 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 858 of 858 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 115628 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6327 of 7238 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 369637 |
| code | 29060 |
| data | 215329 |
| padding | 8646 |
| unused | 278528 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Boston Bomb Club (1990)(Silmarils)[cr Company][a2].adf:Type | 2 | 117 | 2 |
| Disk-Validator | 1 | 386 | 1 |
| Boston Bomb Club (1990)(Silmarils)[cr Company][a2].adf:t.x | 4 | 6704 | 254 |
| Boston Bomb Club (1990)(Silmarils)[cr Company][a2].adf:bootblock | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| Boston Bomb Club (1990)(Silmarils)[cr Company][a2].adf:PRESENT.CO (raw code file, load address unknown) | 1 | 0 | 0 |
| Boston Bomb Club (1990)(Silmarils)[cr Company][a2].adf:BOBOI.CO (raw code file, load address unknown) | 1 | 0 | 0 |
| Boston Bomb Club (1990)(Silmarils)[cr Company][a2].adf:BOUM.CO (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op FFFF) A7=an address (A7)=an address, slid to exec.library -an address): 16656 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- boston bomb club (1990)(silmarils)[cr company][a2]/Boston Bomb Club (1990)(Silmarils)[cr Company][a2].adf:PRESENT.CO: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- boston bomb club (1990)(silmarils)[cr company][a2]/Boston Bomb Club (1990)(Silmarils)[cr Company][a2].adf:BOBOI.CO: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- boston bomb club (1990)(silmarils)[cr company][a2]/Boston Bomb Club (1990)(Silmarils)[cr Company][a2].adf:BOUM.CO: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- boston bomb club (1990)(silmarils)[cr company][a2]/Boston Bomb Club (1990)(Silmarils)[cr Company][a2].adf: blocks 1738-1759 (bytes 889856-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.86)
- 3 never-loaded region(s), 115628 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Boston Bomb Club (1990)(Silmarils)[cr Company][a2].adf:Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_values_then_call_pointer`: If the buffer pointer argument is non-null, copies sixteen 32-bit values from a fixed offset in the base register into that buffer.

### Disk-Validator

- `compare_disk_bitmap_bits`: Walks a range of indexes, computing a word index and bit mask for each and comparing a bit from one table with a bit from another.

### Boston Bomb Club (1990)(Silmarils)[cr Company][a2].adf:t.x

- `update_countdown_list_entries`: Stores routine addresses and data pointers into global slots, then calls three unnamed routines.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `return_immediately_4`: The routine contains no instructions and does nothing but return.
- `open_libraries_and_allocate_memory`: Sets a global flag to one, opens the required libraries, and checks the amount of free memory before allocating working buffers and setting pointers into them.
- `open_keyboard_device_request`: Stores two values, then calls the library function that finds the current task and the one that adds a port, and finally opens the keyboard device.
- `call_four_routines_in_order`: Calls four unnamed routines one after another and then returns.
- `copy_global_to_slot`: Copies one global variable into another global slot and returns.
- `do_nothing_routine`: This routine contains no instructions and does nothing; it simply returns to its caller.
- `allocate_screen_and_window_buffers`: Sets several global buffer pointers from a base address, allocating memory through a library call and laying out buffers at fixed offsets such as 16000 and 32000 bytes.
- `copy_block_and_restart_copper`: Copies a block of words between memory locations inside a library pair that disables and re-enables multitasking, and calls one unnamed routine.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `find_value_in_table`: Searches a table of 16-bit values for the value in the data register and returns its index, or minus one when the value is absent.
- `return_immediately`: Returns at once without any operations; the decompiled body contains only a return.
- `multiply_add_into_table`: Multiplies a stored 16-bit factor by a selected value, adds an input value, and stores each of four results as a 32-bit value in a table.
- `update_pointer_and_string_tables`: Fills up to four table entries from a list of 16-bit values, stopping at any value above 200.
- `empty_return_routine`: The routine contains no instructions and does nothing; it returns immediately without modifying any data or registers.
- `call_graphics_set_rast`: Calls the graphics library's SetRast function through a stored library base and returns the data register unchanged.
- `wait_until_flag_bit_set`: Loops until bit value 16 of a global variable is set, then returns.
- `set_flag_then_call_routine`: Sets a global variable to 16, calls one unnamed routine, and returns.
- `update_pointer_table`: Iterates through a list of pointers and writes four 32-bit results into each target, computed from a 16-bit value, a stored factor and an offset.
- `run_unnamed_routines_in_supervisor`: Sets a bit in a global flag, enters supervisor state through a library call, runs five unnamed routines, and returns to user state.
- `run_routines_around_supervisor_call`: Calls two unnamed routines, sets two global values to 15 and 1920, then calls the supervisor-state library function and the user-state library function.
- `enter_supervisor_then_restore`: Clears three global variables, calls the supervisor-state library function and keeps its return value, then calls the user-state library function with a routine pointer.
- `return_to_user_state`: Calls the supervisor-state library function, then sets a global variable from a saved value and calls the user-state library function with it.
- `increment_counter_and_call_four`: Increments a 32-bit global counter, then calls one routine four times in a countdown loop from three down to zero, followed by three further routine calls.
- `set_bit_when_countdown_expires`: If a global flag is clear, it decrements a per-index countdown byte and, when that reaches zero, sets the index's bit in a global mask.
- `update_channel_state_from_table`: If a global flag is clear, it reads a per-index table entry: a zero status byte sets a global to one shifted by the index, otherwise it computes and writes a 16-bit value.
- `do_nothing_return`: The routine contains no instructions and returns immediately without reading or writing any data, calling anything, or touching hardware.
- `empty_return_routine_2`: The routine contains no instructions; it does nothing and returns immediately.
- `return_immediately_2`: Returns at once without any operations.
- `do_nothing_return_2`: The routine contains no instructions and does nothing; it simply returns to its caller.
- `decode_packed_nibble_strings`: Walks a list of entries and, when a global flag bit is set, decodes runs of packed bytes into character pairs.
- `split_source_words_into_planes`: Walks a list of compressed row records and copies their 16-bit words into four separate destination buffers, one per plane, in rows of 40 words.
- `return_constant_3000`: Returns the constant 3000 as a 16-bit value and performs no other work.
- `tiny_move_rts`: Small routine of 2 instructions that copies 100 to d0, then returns.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies 0 to d0, then returns.
- `replicate_global_byte`: Returns a 16-bit value made by copying a global byte into both halves.
- `return_immediately_5`: This routine does nothing; it returns immediately without reading or writing any data, calling anything, or touching hardware.
- `copy_pointer_list_blocks`: Walks up to four source and destination pointer pairs held in globals, stopping at a null source.
- `call_write_pixel`: Calls one unnamed routine, then calls the graphics library's WritePixel function through a stored library base.
- `call_draw_and_move`: Calls one unnamed routine, then calls the graphics library's Draw and Move functions through library pointers stored in a global variable.
- `call_rect_fill`: Calls one unnamed routine, then calls the graphics library's RectFill function through a stored library base.
- `empty_routine`: This routine contains no executable instructions and performs no operations.
- `store_byte_and_set_pen`: Stores the low byte of its argument in a global variable, calls the graphics library's SetAPen function through a stored base, and returns the argument.
- `store_byte_and_set_bpen`: Stores the low byte of its argument in a global variable, calls the graphics library's SetBPen function through a stored base, and returns the argument.
- `set_draw_mode_with_remap`: Maps a drawing argument of three to four, stores the low byte in a global variable, and calls the graphics library's SetDrMd function.
- `wait_for_value_then_clear`: Calls one unnamed routine repeatedly until it returns a nonzero byte, then calls it again until it returns zero.
- `print_character_with_cursor`: Handles one character code.
- `call_routine_per_character`: Walks a null-terminated string and calls one unnamed routine for each non-null character.
- `clamp_and_scale_cell_position`: Clamps the first argument to at most 39 and the second to at most 24, then stores the results multiplied by nine and eight into two global variables.
- `empty_return_routine_3`: This routine contains no instructions and returns immediately without performing any operation.
- `update_mouse_position_clamped`: Adds the mouse movement reading to a stored horizontal and vertical position, clamps the horizontal value to zero through 639 and the vertical value to a fixed range, then clears bits in a third global.
- `scale_input_values`: Multiplies its two input register values by two and stores the results in two global variables.
- `return_halved_global_value`: Returns a stored 16-bit value shifted right by one bit.
- `compare_table_bit_changes`: Walks twelve byte pairs in a table, 16 bytes apart, and finds the bits that differ within each pair by XOR, then calls one unnamed routine.
- `set_status_bits_from_flags`: Calls the library's DoIO function through a stored base, then sets bits in a global status value according to individual bits of a flag byte.
- `build_button_status_bits`: Combines bits from a global control byte into a 32-bit status value, setting bits for button and flag combinations, and returns it with the second register unchanged.
- `build_mouse_status_bits`: Combines bits from a global control byte with the upper half of a data register to set button status bits, plus a further bit from a sign test on a second register.
- `build_joystick_status_bits`: Combines bits from a global control byte with the upper half of a data register to set button status bits, then sets a further bit from a sign test on a second register.
- `copy_control_byte`: Loads a byte from a global variable and stores it into a second global variable through a pointer, then returns.
- `return_immediately_3`: The decompiled body contains only a return, although the listed facts record one unnamed call and one caller.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d0, then returns.
- `open_file_or_fail`: Calls one unnamed routine, then calls the library's Open function through a stored base.
- `return_zero_value`: Returns zero and performs no other work.
- `lock_and_unlock_conditionally`: Calls the library's Lock function through a stored base and, when the lock is obtained, calls the library's UnLock function with the argument register.
- `close_file_or_halt`: Closes a file through the library close function, then returns if a global flag byte is zero; otherwise it reopens and closes a file and loops forever.
- `call_and_normalize_result`: Invokes a routine through a function pointer stored in a table, then returns its result, substituting negative one when that result equals zero.
- `call_and_normalize_result_2`: Invokes a routine through a function pointer loaded from a table, then returns its result, substituting negative one when that result equals zero.
- `call_delete_file`: Library stub: loads its arguments and the dos library base, calls DeleteFile and returns its result to the caller.
- `call_dos_read`: Calls the library's Read function through a stored base pointer taken from a global variable and returns nothing.
- `write_then_set_flag`: Calls one unnamed routine, then calls the library's Write function through a stored base, and sets a global variable to one.
- `wait_for_return_key_prompt`: Loops on an input state until a key value other than two specific codes is seen, calling nine unnamed routines for display steps.
- `copy_and_clear_planes`: Gathers words from five source plane buffers into one interleaved destination buffer and clears each source word after copying it.
- `copy_from_interleaved`: Splits an interleaved buffer back into five plane buffers, copying one word per plane for each position over a grid of 20 words per row and 18 rows.
- `empty_routine_2`: An empty routine that returns immediately without touching any data or registers.
- `poll_message_until_flag_clear`: Polls the library's GetMsg function for a message and, when that message has specific flag bits set, keeps polling until a further flag bit is clear.
- `return_stack_argument`: Returns the 64-bit value passed on the stack without changing it.
- `lock_directory_and_set_current`: Locks a directory through the library's Lock function.
- `call_two_routines_for_error`: Calls two unnamed routines and returns the 64-bit stack argument unchanged.
- `return_zero_routine`: An empty routine that unconditionally returns the constant zero.
- `wait_for_flag_and_compute`: Calls one unnamed routine repeatedly until a global value is no longer minus one, then computes further values from globals and makes four unnamed calls.
- `lock_volume_and_query_info`: Locks a volume through the library Lock function, returning minus one on failure; otherwise it stores the lock, calls the library Info function and releases the lock with UnLock.
- `reset_display_bounds_if_changed`: Chooses one of two stored pointers by the sign of a global value.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `reset_display_bounds_if_changed_2`: Shares its code with the neighbouring entry points in this block.
- `swap_globals_when_changed`: Compares a global value against a conditionally selected alternative, and when they differ it resets a group of globals, swaps two globals, invokes another routine, and restores the saved globals.
- `swap_screen_coordinates`: Swaps two screen coordinates, then if a stored value differs from a current one it zeroes four coordinate slots, loads them with fixed values, invokes another routine twice, and swaps two more stored values around the calls.
- `swap_and_call_routine`: It swaps the contents of two adjacent global words, then sets four other global words to fixed values, invokes another routine, and finally restores the four globals to their original values before invoking that routine two more times.
- `reset_and_swap_globals`: It loads a group of global values, then compares one of them against a value selected by a flag.
- `swap_and_initialize_params`: Swaps several values in a table or global group and initializes a new set of parameters when a condition is met.
- `copy_block_and_toggle_globals`: Calls one unnamed routine and copies five 32-bit values between global buffers.
- `update_table_values`: Computes a shift of a negated global value and adds it to five consecutive 32-bit table entries.
- `setup_block_pointers_if_allocated`: Calls one unnamed routine that returns a pointer.
- `init_block_from_pointer`: Fetches a pointer from another routine and clears a global when it is null, otherwise it derives a size from a register value, stores several computed offsets and pointers into a group of globals, and writes values into the pointed-to block.
- `init_segmented_memory_block`: Allocates a block of memory and initializes it as a linked list of segments.
- `compute_row_bytes_and_sizes`: Computes a row size in bytes from a structure field, then derives sequential offsets and sizes from global values and stores them.
- `allocate_downward_block`: Computes a rounded size from its argument, moves a downward pointer by that size, and stores the size in the new block when the result stays above a limit.
- `store_computed_value_at_pointer`: Computes a value from a register input, then decrements a global pointer and stores the computed value at the resulting location, advancing the pointer when a second global remains greater than it, otherwise clearing the pointer.
- `push_row_entry`: Computes a value from the incoming D0 register and derives a pointer into a global buffer.
- `push_row_size_record`: Computes a size value from the input and walks a linked list of size records stored in a global table.
- `store_row_entry`: Computes a pointer into a global table and, when the computed address is greater than the previously stored one, records that address together with the value in D0.
- `allocate_downward_block_2`: Computes a rounded size from its argument, moves a downward pointer by that size, and stores the size in the new block when the result stays above a limit.
- `return_stack_argument_2`: Returns the 32-bit value passed on the stack without changing it.
- `free_block_if_top`: If the global allocation pointer equals the block address passed in, it advances the pointer by the size word stored at the start of that block.
- `return_zero`: This routine is a leaf function that returns the constant value zero to its caller without reading or writing any memory or registers.
- `return_minus_one_value`: Returns the 16-bit value minus one and performs no other work.
- `tiny_nop`: Small routine of 1 instruction that does nothing for one instruction.
- `install_handler_pointers`: Stores routine addresses and buffer pointers into several global slots, then makes a sequence of calls to unnamed routines.
- `dispatch_by_table_byte`: Reads a byte through a register pointer, doubles it, uses it to index a 16-bit offset table, and jumps to the computed address through the base register.
- `clear_flag_and_return`: Loops while a 16-bit value read through a pointer is nonzero, calling one unnamed routine on each pass.
- `traverse_and_update_linked_list`: Walks a linked list of nodes, comparing a value in each node against a byte, and replaces or relinks the nodes that match.
- `update_linked_list_node`: Follows a 16-bit link from a node and, when the index matches the back-link of the next node, rewrites that back-link.
- `copy_two_longwords_to_globals`: Copies two 32-bit values from the record in the address register into two global variables and returns.
- `apply_coordinate_offset`: Computes the difference between two points held in a pointer record and adds that difference to a list of coordinate values.
- `add_offset_fields_to_coordinates`: Adds offset fields stored in one part of a record to coordinate fields in another part, one pair per coordinate, then returns.
- `compute_scaled_position_values`: Computes position values from a vector with shift-based scaling and conditional adjustments, then makes one unnamed call.
- `return_record_field_when_enabled`: If a flag byte in the record in the address register is negative, it returns the data registers unchanged.
- `wait_and_process_object_list`: Walks a linked list of object entries, copying their bounds into global variables and relinking or freeing nodes.
- `process_object_state_updates`: A long routine of 174 instructions that makes fifteen unnamed calls while reading and updating global variables and a table of object values.
- `write_values_to_offset_table`: Calls one unnamed routine, then writes two 16-bit values and one 32-bit value into a memory area at offsets from the address register and a register index.
- `relink_node_and_check_bound`: Copies a next-pointer field from one node to another, pushes the node onto a global free list, and calls one unnamed routine.
- `update_bounds_and_store_value`: Calls one unnamed routine, then sets a global flag when another global is set and a stored 16-bit value reaches a bound.
- `initialize_global_bounds`: Sets two global variables to 32767 and two to minus 32768, the largest and smallest 16-bit values, as starting bounds.
- `expand_global_bounds`: Compares a 16-bit value stored in a record against global bound variables and widens those bounds when the value falls outside them.
- `insert_sorted_list_node`: Inserts a node into a linked list ordered by priority fields, comparing a 16-bit value and two bytes against existing nodes.
- `process_flagged_list_entries`: A long routine of 147 instructions that tests flag bits in a record and makes ten unnamed calls while updating global variables and table entries.
- `set_flag_from_bounds`: Compares two global bound values against two others and stores a flag value in a global variable, depending on whether the ranges overlap.
- `copy_four_values_to_buffer`: Copies four constant values from one memory location to another and calls one unnamed routine.
- `copy_table_block`: Iterates through a table of 16-bit values, copying each into a set of global variables until a limit is reached.
- `clamp_rect_bounds`: Clears a global variable, then clamps rectangle bounds read through a pointer-offset table to a minimum threshold when they are too small.
- `expand_rect_bounds`: Checks whether a rectangle defined by a table and global values lies within the valid range, then widens the global bounds to include it.
- `update_clipping_region`: Updates a global clipping rectangle by comparing a new area with the existing bounds, applying bit masks and computing an offset or size from the result.
- `empty_return_routine_4`: This routine contains no instructions and performs no operations on any data.
- `reset_global_variable`: Writes a global variable with 255 and then with zero, leaving it at zero.
- `check_nibble_sum`: Adds the low nibbles of two 16-bit inputs and compares the sum with 15.
- `check_nibble_difference`: Subtracts the low nibble of one 16-bit input from the other and returns in every branch, so the routine has no visible effect.
- `reconstruct_nibble_data`: Scans a buffer comparing memory values in pairs.
- `forward_single_call`: Calls one unnamed routine and then returns, with no other work.
- `call_nibble_difference`: Invokes another routine that subtracts the low nibble of one 16-bit input from the other and returns in every branch, so this wrapper produces no visible effect.
- `call_sequence_of_three_routines`: Calls one unnamed routine, passes its result to a second unnamed routine, then calls a third unnamed routine and returns.
- `initialize_data_structures`: Sets several global variables to fixed values, then fills a table with calculated pointers in a loop.
- `copy_name_and_track_extension`: Copies a text string into a buffer one character at a time and records the position just after the last full stop.
- `decode_rle_stream`: Reads control bytes from a global stream: a control byte with its high bit set repeats the next byte, and one without copies that many following byte pairs, with the count taken from a global value.
- `read_next_byte_with_refill`: Compares a position with a limit and calls an unnamed routine to refill the buffer when the limit is reached.
- `write_byte_with_bounds_check`: Writes its byte argument to a computed destination when a range check passes.
- `copy_run_into_buffer`: Computes a destination pointer from its length argument and copies bytes between buffers using data registers.
- `tiny_moveq`: Small routine of 1 instruction that loads 3 into d0.
- `return_absolute_byte_difference`: Returns the first byte argument when the difference of the two bytes is non-negative, otherwise returns its negation.
- `invoke_and_return_pair`: Calls one unnamed routine and returns the two 32-bit data registers unchanged as a 64-bit value.
- `search_sorted_table`: Performs a binary search over a table of 16-bit values for a given value.
- `initialize_list_structure`: Writes incrementing offsets and zero values across a memory range to set up a list.
- `copy_pointer_table_values`: Works through pointer tables held in address registers, copying 16-bit values between them.
- `update_record_fields_from_globals`: A long routine of 109 instructions that reads and writes 16-bit fields of a record through an address register, combining them with global values.
- `process_status_flag_loop`: Tests bits 256 and 512 of an input word, then loops reading status values through unnamed routines and storing the latest in a global.
- `call_then_return_input`: Calls one unnamed routine and returns the data register unchanged.
- `retry_until_nonnegative`: Calls one unnamed routine repeatedly until it returns a non-negative value, calling a second unnamed routine after each failed pass.
- `retry_store_until_nonnegative`: Calls one unnamed routine and stores its result in a global variable.
- `loop_until_nonnegative_result`: It repeatedly invokes a routine through a function pointer stored in a table, replacing negative one results with a stub that returns only.
- `retry_until_nonnegative_pair`: Loops calling one unnamed routine until its result is non-negative, passing register values through.
- `update_state_byte_and_countdown`: Updates a global state byte and a countdown, making eleven unnamed calls on different transitions.
- `update_global_and_copy`: Updates a global variable, checks it against a boundary, and copies a sequence of memory blocks from a source to a destination.
- `decrement_counter_and_clear_fields`: Decrements a counter word in a structure while it is nonzero, combining two fields with clamping and calling an unnamed routine.
- `run_four_subroutines_in_sequence`: Calls four other routines one after another and returns, with no other work in between.
- `clear_structure_fields`: Writes zero through the address register to three byte offsets (0, 1, 2) and two word offsets (6 and 10) of the block it points to, then returns.
- `update_clamped_channel_state`: Advances a 16-bit accumulator by a step value and clamps it to a maximum of 32767, while counting down counter variables that choose the branch.
- `stub_return_only`: Routine consisting only of a return, with no calls, memory access or hardware register use.
- `empty_return_routine_5`: This routine contains no instructions and returns immediately without executing any code, modifying any registers, touching any hardware registers, or accessing any memory.
- `add_offsets_on_nonzero_result`: Calls a routine that returns a 16-bit value and, if that value is non-zero, adds fixed amounts to two global variables.
- `return_registers_after_call`: Calls one other routine and then returns the two 32-bit data registers as they stand after that call, without modifying them.
- `forward_call_then_return`: Calls one other routine and returns right after it, with no conditional or data work visible in this listing.
- `blit_masked_region_planes`: Returns early when the region lies outside a bounds check; otherwise it clips the region and copies 16 rows through five bitplane destinations.
- `copy_five_bitplane_rows`: Copies 16 rows of five bitplane words from a source table into five destination pointers, advancing each by a stride.
- `tiny_ori_2`: Small routine of 1 instruction that merges 117 into d0.
- `blit_rectangle_with_masks`: Programs the blitter with fixed first and last word masks and modulo values, waits for it to go idle, then sets two plane pointers from computed row and column offsets.
- `blit_rows_in_loop`: Sets up the blitter masks and modulo once, then loops over the region's rows.
- `copy_five_planes_interleaved`: Checks the block width and hands off to another routine when it exceeds 199 and is not exactly 320.
- `blit_block_with_modulo`: Sets blitter control, mask and modulo values, waits for the blitter to go idle, and points a source pointer at a computed offset from a plane table.
- `blit_when_flags_clear`: Reads two flag bits from a record; when neither is set it sets blitter masks and offsets, waits for idle and starts a blit using the record's size words.
- `clip_and_remap_region_bytes`: Clips a rectangle against global bounds and derives its width and row counts from the record's size fields.
- `clip_record_and_blit_result`: Returns the incoming value unchanged when the record lies outside the global bounds.
- `clip_record_and_blit`: Returns at once when the record lies outside the global bounds.
- `store_flag_for_matching_handler`: Calls another routine unless a global flag is clear and the address register matches none of four known handlers, in which case it returns at once.
- `store_byte_through_pointer`: Writes a global byte value at offset 1 from the address register, then calls another routine and returns.
- `configure_audio_channel`: Calls a system library routine to enter supervisor state, then sets a channel's volume from an input byte, clamped to 100, and writes its sample pointer, length and period to the audio hardware.
- `clear_channel_flag_if_negative`: Tests a per-channel flag byte and, when its sign bit is set, clears that flag and a byte at offset 1 of the structure at the address register.
- `call_routines_for_set_bits`: Tests four bits of a global mask and calls a routine for each bit that is set.
- `decrement_channel_counter`: Decrements a 16-bit counter for a channel through the pointer in the A1 register.
- `execute_sequential_routines`: Calls four other routines one after another and returns, doing nothing else in this listing.
- `pass_registers_through_call`: Calls one neighbouring routine and returns the two 32-bit data registers as they stand afterwards, with no work of its own.
- `return_data_register_after_call`: Calls another routine and then returns the 32-bit value held in the first data register, without changing it in this listing.
- `forward_call_keep_registers`: Calls one other routine with no work of its own and returns the two data registers unchanged.
- `scale_seven_bit_value`: Keeps the low seven bits of the first data word, multiplies that by eight, subtracts seven and stores the result in a global word.
- `forward_call_return_value`: Calls one other routine and returns the first data register as it stands afterwards.
- `clamp_and_store_volume`: Clamps the incoming value to a maximum of 100.
- `program_audio_channel_three`: Writes a sample pointer, length and period to the fourth audio channel registers, then sets or clears its DMA enable and interrupt bits depending on the sign of an input word.
- `store_fixed_control_words`: Writes two fixed 16-bit constants into the structure at the address register, at byte offsets 150 and 154, then returns.
- `setup_audio_interrupt_state`: Saves four register inputs into global variables and enters supervisor state through a library call.
- `toggle_audio_and_interrupt_state`: Returns at once unless a state byte is set.
- `conditionally_set_control_bits`: When one global byte is non-zero it stores a handler pointer and two fixed control words in globals.
- `dispatch_handler_for_request`: If a status flag in a global word is set, it checks four interrupt request bits in that word and calls the handler stored for the first one found, then returns.
- `dispatch_handler_for_bits`: Checks three request bits in the second data register, in turn, and calls the handler stored for the first one set.
- `dispatch_handler_for_two_bits`: Checks two request bits in the second data register and calls the handler stored for the first one set.
- `dispatch_handler_for_bit`: Checks one request bit in the second data register and calls the handler stored for it when that bit is set.
- `dispatch_handler_for_masked_bits`: Combines two status words of the structure at the address register and keeps four interrupt request bits.
- `initialize_interrupt_handler_slot`: Writes fixed control constants to globals, stores a handler pointer and a saved word into the structure at the address register, and clears those globals and a handler slot.
- `set_handlers_and_masks`: Sets a global variable to a fixed value, then stores handler pointers into several offset locations of a structure while clearing the previous handlers, and returns a register value.
- `init_interrupt_vectors`: Initializes a custom hardware register by writing a byte, then installs two new interrupt vectors and clears the previous ones.
- `store_bit1_handler`: It sets a global variable to a value with the top bits of a hardware register, then stores a handler address at an offset in a structure pointed to by an argument, clearing the original handler slot.
- `install_vblank_interrupts`: Initializes a hardware register by writing a control value to it, then installs three interrupt service routines by storing their addresses into the vertical-blank, INTENSET, and VERTB handler vectors.
- `install_vertical_blank_handler`: Writes the current vertical blank interrupt handler address into a copy of the IntVec table, clears the old handler slot, and installs a new handler at a different vector slot.
- `init_audio_channel_three`: Initializes the third sound channel by configuring its hardware registers, then installs a periodic timer interrupt that reloads the channel's sample pointer and volume.
- `initialize_timer_and_pointers`: Writes fixed control constants to globals, stores a handler pointer and a saved word into the structure at the address register, and clears two globals and a handler slot.
- `step_audio_command_stream`: Counts down a timer and, when it expires, reads the next command byte from a stream and dispatches on it.
- `process_audio_command`: Reads the next byte from a sound command stream and dispatches to a handler based on the command code, updating playback pointers and timer registers as needed.
- `run_audio_command_loop`: Resets the timer and loops over the sound command stream, handling each command in turn.
- `resume_audio_command_stream`: Continues the sound command stream from its saved position, handling commands in a loop until a wait byte.
- `start_audio_command_stream`: Starts the sound command stream from its table pointer and processes commands until a wait byte.
- `store_stream_wait_value`: Stores a wait value taken from the low byte of the first data register in the handler slot, increments a counter, saves the stream position and stores a handler pointer before returning.
- `copy_table_entry_then_resume`: Copies a table entry into the global table at an index taken from the data register, then continues processing the sound command stream until a wait byte.
- `program_audio_channel_then_resume`: Looks up a sample table entry and, when its length is valid, programs an audio channel's pointer, length and period.
- `continue_audio_command_loop`: Copies a table word into a structure slot and continues processing the sound command stream.
- `program_channel_sample_in_loop`: Programs an audio channel's pointer, length, period and volume from a sample table entry when its length is valid, then continues the sound command stream until a wait byte, when it stores a handler pointer and returns.
- `continue_stream_store_channel`: Continues the sound command stream and stores channel-related table values into the structure at the address register.
- `skip_stream_bytes_and_continue`: Skips two bytes of the sound command stream and continues processing commands until a wait byte.
- `set_bit0_pointer_and_width`: When bit 0 of the flag byte at offset 3 is set, it stores a width computed from two fields before the pointer at offset 164, stores the pointer plus a field at offset 160, and sets the word at offset 154 to 128.
- `set_bit0_handler_and_mask`: When bit 0 of the flag byte at offset 3 is set, it stores a routine pointer in a global variable.
- `set_mode1_and_masks`: Stores the value 1 at offset 150 and the value 128 at offsets 154 and 156 of the object at the address register.
- `set_bit1_pointer_and_width`: When bit 1 of the flag byte at offset 3 is set, it stores a width computed from two fields before the pointer at offset 180, stores the pointer plus a field at offset 176, and sets the word at offset 154 to 256.
- `set_bit1_handler_and_mask`: When bit 1 of the flag byte at offset 3 is set, it stores a routine pointer in a global variable.
- `set_mode2_and_masks`: Stores the value 2 at offset 150 and the value 256 at offsets 154 and 156 of the object at the address register.
- `set_bit2_pointer_and_width`: When bit 2 of the flag byte at offset 3 is set, it stores a width computed from two fields before the pointer at offset 196, stores the pointer plus a field at offset 192, and sets the word at offset 154 to 512.
- `set_bit2_handler_and_mask`: When bit 2 of the flag byte at offset 3 is set, it stores a routine pointer in a global variable.
- `set_mode4_and_masks`: Stores the value 4 at offset 150 and the value 512 at offsets 154 and 156 of the object at the address register.
- `decrement_counter_set_bit3_mode`: When bit 3 of the flag byte at offset 3 is set, it decrements a global counter and, when the counter goes negative, sets the word at offset 150 to 8 and the word at offset 154 to 1024.
- `do_nothing_routine_2`: This routine does nothing at all; it returns immediately without reading or writing any data, calling anything, or touching hardware.
- `stub_bad_instruction_data`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `empty_data_block`: The routine contains no executable instructions, only data that the disassembler cannot decode as valid 68000 code.
- `stub_bad_instruction_data_2`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `stub_bad_instruction_data_3`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `identify_invalid_code`: The routine contains no valid 68000 instructions; the bytes at its location are data rather than executable code, so it cannot be disassembled as a functioning routine.
- `stub_bad_instruction_data_4`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `halt_baddata`: The routine's body consists of an instruction that is not valid 68000 code, so no real operation can be decoded from it.
- `stub_bad_instruction_data_5`: The listing shows only an undecodable instruction in this entry, so there is no real routine and no logic to recover.
- `flag_invalid_routine`: The routine contains no valid instructions; the bytes at its address are not executable code, so it cannot be interpreted as a real routine.
- `undecodable_routine`: The routine's instructions are not valid 68000 code, so it cannot be disassembled or understood from the binary.
- `empty_data_block_2`: This routine contains no executable instructions, only non-code data that the disassembler flagged as invalid.
- `copy_table_block_2`: Copies a block of data from one table to another using a pointer, advancing the source and destination pointers by a fixed size each iteration until the block is fully transferred.
- `empty_routine_3`: This routine is empty and performs no operations.
- `empty_routine_4`: This routine is empty and performs no operations.
- `halt_baddata_2`: The routine is empty, containing only an instruction that is not valid 68000 code, so no operations can be recovered from it.
- `decode_bad_data`: The routine contains four instructions that appear to be non-executable data rather than valid code at this location.
- `install_handler_pointer`: Installs a handler pointer into a table by storing a routine address at a table slot.
- `noop_routine`: This routine is empty; it contains no instructions and performs no operations on any data.
- `install_handler_pointer_2`: Installs a handler pointer into a data structure by writing the address of another routine through a pointer, as part of setting up the handler table.

### Boston Bomb Club (1990)(Silmarils)[cr Company][a2].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `add_offsets_on_nonzero_result_2`: It calls another routine that returns a 16-bit value and, when that value is non-zero, adds fixed constants to two global 32-bit values.
