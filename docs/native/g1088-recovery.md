# Game 1088 (operation stealth (demo) (1990)(u. s. gold)[rolling demo]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01088). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901776 top-level bytes in 26 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 20 of 20 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 438 of 438 named (0 author symbols), 438 of 438 described (105 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3461 of 3461 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 14562 of 14562 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 352611 |
| code | 48476 |
| data | 115418 |
| padding | 5879 |
| unused | 379392 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Operation Stealth (demo) (1990)(U. S. Gold)[rolling demo].adf:demo | 3 | 11290 | 297 |
| run | 2 | 117 | 2 |
| else | 2 | 117 | 2 |
| Binddrivers | 11 | 652 | 35 |
| delete | 2 | 117 | 2 |
| dir | 2 | 117 | 2 |
| echo | 2 | 117 | 2 |
| failat | 2 | 117 | 2 |
| copy | 2 | 117 | 2 |
| cd | 2 | 117 | 2 |
| endif | 1 | 2 | 1 |
| if | 2 | 117 | 2 |
| wait | 2 | 117 | 2 |
| ram-handler | 2 | 10 | 1 |
| Operation Stealth (demo) (1990)(U. S. Gold)[rolling demo].adf:endcli | 2 | 117 | 2 |
| Operation Stealth (demo) (1990)(U. S. Gold)[rolling demo].adf:startram | 3 | 665 | 39 |
| Operation Stealth (demo) (1990)(U. S. Gold)[rolling demo].adf:add21k | 3 | 637 | 38 |
| Operation Stealth (demo) (1990)(U. S. Gold)[rolling demo].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 2 |

## Cases handled

- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Operation Stealth (demo) (1990)(U. S. Gold)[rolling demo].adf:demo

- `jump_to_main_entry`: Jumps directly to the main program entry point at a fixed absolute address, transferring control without saving any registers or setting up a stack frame.
- `init_display_and_audio_registers`: Copies a longword from one global structure into another, then clears a word at offset 136 and writes a fixed word value to offset 150 of the destination structure.
- `allocate_memory_with_retry_and_report`: Allocates memory in a loop, retries on failure, formats allocation and free totals into a buffer, prints them, waits for a key, and returns the remaining byte count.
- `check_alloc_delta_and_report`: Samples the free-memory counter before and after a call, subtracts the delta from a global allocation total, and prints a French warning if the total goes negative or the delta differs from the expected size.
- `show_pause_message_wait_fire`: Clears the screen, draws the French return-to-game prompt, then polls the fire button byte until it is pressed and released before restoring the display and returning.
- `read_fire_and_right_buttons`: Returns a bitmask in d0 built from the CIA fire button and the POTGOR right button, setting bit zero for fire and bit one for the right button.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `enable_intena_4f_and_return`: Writes the value 79 to the custom chip interrupt enable register at the fixed hardware address, then returns to the caller.
- `xorshift_seeded_divide_by_argument`: Loads a 32-bit seed from a4-relative memory, doubles it with xor fallback, stores it back, then divides the low word by the stack argument and returns quotient and remainder packed in d0.
- `read_ciaa_keyboard_serial_byte`: Polls the CIA-A interrupt control register for the serial-ready bit, reads the keyboard serial data register, inverts and rotates the byte, delays, clears the handshake bit and returns the byte in d0.
- `lookup_byte_in_scan_code_table`: Calls a routine to read a byte, rejects it if bit 7 is set or it is zero, otherwise scans 49 word entries comparing against the byte and returns the matching word value, or zero.
- `fill_copper_list_from_offsets`: Copies 180 words from a table into the copper list pointer, then writes high and low words of five offset values plus two global values into copper registers.
- `init_copper_display_registers`: Writes fixed word values into the copper list at offsets 142 through 150, 256 through 266 and 418 through 422 to set up display and sprite registers.
- `set_audio_period_and_pointer`: Calls a subroutine, then stores the word 480 and later 34784 into offset 150 of the structure pointed to by the a4-relative global, writes the caller's argument to offset 128 and clears the word at offset 136.
- `get_global_long_value`: Returns in d0 the longword stored at the a4-relative global offset, reading that memory location and leaving it unchanged.
- `configure_five_channel_periods`: Optionally stores the first argument to a global, then writes five successive period pairs (base plus 8000, 16000, 24000 and 32000) into the channel structure after calling a setup subroutine.
- `copy_16_words_to_sprite_buffer`: Copies sixteen words from the caller's buffer at offset 8 of the frame into the sprite structure at offset 112, using d4 as the loop counter and a2 as the destination pointer.
- `store_sprite_control_block_fields`: Calls a setup routine, then writes the caller's long and word arguments into the sprite control block at offsets 64 through 116, and stores the combined vertical position at offset 88.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `empty_c_function_2`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `unreached_sprite_setup_stub`: Empty unreached code path that pushes a pointer and two -1 words, calls the sprite copy and setup routines, then returns without using any result.
- `load_vol_cnf_entries`: Clears twenty 20-byte records in a4-relative arrays, opens VOL.CNF, reads a count and two words, then reads each record and its data into memory, closing the file.
- `open_vol_cnf_and_get_handle`: Opens VOL.CNF with mode -2, stores the returned file handle, and if valid assigns it to the global handle and returns the value at offset 4 of that handle.
- `copy_d0_to_d5`: Copies the low word of d0 into d5 and falls through to the following code.
- `open_dh0_file_and_read_length`: Opens the DH0: file via the dos library, stores the handle, reads a longword at offset 4 of the file structure, sign-extends it into d0 and closes the handle.
- `check_and_append_byte_to_file`: Looks up a file by name, and if found compares a stored word with the argument, otherwise appends a byte from the argument to the file buffer and returns a status code.
- `negate_d6_word_data_stub`: Negates the low word of d6 and copies d0 into itself; these two instructions are data reached through a pea address, not a real routine.
- `compute_screen_coords_from_slot`: Calls a helper, stores its word result at the first pointer argument, then reads a slot index, scales it by 74, adds table offsets and bias values, and writes two computed coordinates to the second and third pointer arguments.
- `init_demo_resources_and_tables`: Allocates memory blocks and loads files, sets up graphics and text pointers, clears several arrays of counters and records, and initializes the demo state before the main loop runs.
- `empty_c_function_3`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `swap_buffers_and_update_palette`: Swaps two buffer pointers, calls a blit routine, copies sixteen palette words from a selected source into the active palette, submits it, and sets the display hardware color register.
- `set_display_control_word`: Calls a subroutine, loads a pointer from offset minus thirty-two thousand seven hundred and thirty in a4, then writes the immediate value twenty-five thousand and eighty-eight to offset two hundred and fifty-six of that structure.
- `reset_all_slot_records`: Loops over two hundred and fifty-five records of thirty bytes, and for each non-empty record calls a routine with its pointer and a scaled word product, then clears the pointer and sets two words to minus one.
- `scan_slot_records_for_nonempty`: Loops a counter from zero while below two hundred and fifty-five, computing a thirty-byte stride into the a4 table and branching out when a longword entry is nonzero.
- `check_slot_marker_return_ff`: Compares the word at offset minus two from a5 with 255; if equal, sets d0 to 255, releases the a5 frame and returns, otherwise branches to the next routine.
- `store_sprite_slot_fields`: Multiplies the words at offsets eight and ten from a5, calls a routine with the product, then writes the result and several a5 words into a thirty-byte record indexed by the word at minus two from a5, sets two fields to 65535 and increments a counter.
- `allocate_sprite_slot_or_fail`: Calls a routine with the word at offset twelve from a5, multiplies the words at offsets eight and ten, calls another routine with the product, stores the result in a thirty-byte record and returns minus one if that result is zero.
- `store_sprite_entry_fields`: Uses the index at offset minus two of a5 to scale by thirty and writes four words from the a5 frame into the sprite table at a4, then increments the active sprite counter and returns zero in d0.
- `check_sprite_slot_allocated`: Reads the index argument at offset eight of a5, scales it by thirty and tests the longword in the sprite table at a4; returns minus one in d0 if empty, otherwise falls through to the allocation path.
- `free_sprite_slot_and_reinit`: Computes a sprite's width times height, frees its bitmap, clears the table entry fields, marks two words as minus one, queues a callback with the entry address and decrements the active sprite counter.
- `set_d0_bits_and_start_d4_zero`: ORs the immediate bytes 85 and 4 into the low byte of d0, sets d4 to zero, then branches forward to another routine.
- `emit_words_from_a5_offsets_loop`: Loops d4 from zero while below the word at offset 10 of a5, passing the word at offset 8 plus d4 to a called routine, then restores d4 and returns.
- `clear_two_a4_relative_longs`: Sets up an a5 frame, clears two longwords at fixed negative offsets from a4, then releases the frame and returns.
- `free_all_list_nodes`: Walks the linked list at the a4-relative global head, saves each node's next pointer, calls the free routine with size 22 on each node, then calls the empty cleanup routine and returns.
- `insert_node_sorted_by_priority`: Scans the a4-relative list for the first node whose priority word is not less than the requested one and whose type is not 2 or 3, then allocates a 22-byte node and links it before that node.
- `check_node_matches_key`: Compares the node at -4(a5) against the key words at 8(a5) and 10(a5); returns -1 when both match, otherwise falls through to the insertion path.
- `allocate_and_link_list_node`: Calls an allocator with size 22, stores the returned pointer, copies the previous node pointer into it, writes two word fields from the stack frame, and links the new node into the list at offset -30532 of a4.
- `find_and_unlink_matching_node`: Walks the linked list at offset -30532 of a4 comparing word fields at offsets 8 and 10 against the caller's arguments, unlinks the matching node, frees it with size 22, and returns zero on success or minus one if not found.
- `reorder_list_by_priority_field`: Scans the list at offset -30532 of a4, detaches nodes matching the given word except types 2, 3 and 20, then reinserts them ordered by a priority word read from a table at offset -21930 of a4.
- `clear_two_global_longwords`: Pushes a5 frame, writes zero to two longword globals at offsets from a4, then releases the frame and returns.
- `walk_list_and_call_free`: Saves a5 frame, loads a list head from a4 offset, loops while non-null, reads next pointer, calls a routine with the node and a constant, then advances.
- `clear_globals_then_return`: Calls the routine that zeroes the two a4-relative longwords, then releases the a5 frame and returns to its caller.
- `walk_linked_list_until_null`: Walks a linked list starting from a global pointer, loading each node's next pointer into a local until a null pointer is found, then falls through to the following code.
- `allocate_and_init_list_node`: Allocates a 22-byte block, copies the caller's node pointer and two word fields into it, fills four word fields from tables indexed by a word argument, and links the new node into a global list.
- `clear_global_list_pointer`: Sets the longword global at the a4-relative offset to zero, clearing the list head pointer, then releases the a5 frame and returns.
- `walk_list_free_nodes_loop`: Walks a linked list from a global pointer, saving each node's next pointer, calling a routine with the node and size 32, then advancing until the pointer is null.
- `call_cleanup_then_unlink_frame`: Calls a cleanup routine, then releases the a5 stack frame and returns to the caller.
- `reset_two_globals_and_return`: Clears a longword global to zero and sets a word global to minus one, then releases the a5 frame and returns.
- `free_all_list_nodes_via_dealloc`: Walks the linked list at the a4-relative global, saving each next pointer, calls the deallocation routine with tag 214 on each node, then calls the no-argument cleanup routine and returns.
- `allocate_and_init_entity_node`: Checks the entity table entry for the index argument; if free sets a flag, otherwise allocates a node, appends it to the list, copies 50 words of table data, clears 50 words and two fields, stores the table pointer and index, then returns.
- `find_and_remove_entity_by_index`: Scans the a4-relative linked list for a node whose index field matches the argument, marks its index as -1 and returns 0, or returns -1 if no match is found.
- `check_a5_minus4_flag_return_ff`: Tests the longword at offset minus four from a5; if it is zero, sets d0 to 255, then releases the a5 frame and returns.
- `compare_a5_object_word_to_arg`: Loads the pointer at a5 minus four, compares the word at its offset 212 with the word at a5 plus eight, and if equal writes 65535 there and returns zero.
- `return_ff_after_failed_word_compare`: Sets d0 to 255 and branches back to the shared exit that releases the a5 frame and returns to the caller.
- `purge_dead_212_offset_nodes`: Walks the linked list at a4-30488, and for each node whose word at offset 212 equals minus one, unlinks it and calls the cleanup routine with argument 214.
- `purge_dead_4_offset_nodes`: Walks the linked list at a4-30060, and for each node whose word at offset 4 equals minus one, unlinks it and calls the cleanup routine with argument 32.
- `reset_list_and_flag_state`: Clears the longword at a4-30274 to zero and sets the word at a4-30062 to minus one, then returns.
- `free_list_entries_with_id`: Walks the linked list at the global pointer, calling the free routine with id 214 on each node, then calls the no-argument free routine and returns.
- `append_zeroed_node_to_list`: Allocates a node with id 214, links it at the tail of the global list, zeroes two 50-word arrays and two words, then copies a table entry and increments its counter.
- `allocate_and_init_list_node_2`: Allocates a node with id 214, links it into the global list, zeroes two 50-word arrays and two words, copies a table entry, calls the init routine and increments the table counter.
- `purge_dead_entity_list`: Walks the linked list at the base pointer, and for each node whose word at offset 212 equals minus one, unlinks it and calls the removal routine with reason 214, then continues with the next node.
- `dispatch_entity_render_by_slot`: Reads the current slot index and a passed slot number to fetch entity fields from several tables, then if the entity's flag word is zero calls the render routine with its parameters, otherwise calls the alternate render path.
- `render_entity_with_flag_bits`: Pushes the entity's flag bits, coordinates and table entry onto the stack, calls the sprite draw routine, then calls the slot update routine with the current index and slot number.
- `store_screen_rect_and_derived_offsets`: Copies four word parameters into globals at offsets from a4, then computes a row-major offset from the first two and a mirrored offset from the third, storing both.
- `blit_scroll_split_screen_rows`: Reads two row pointers from a table indexed by globals, then calls the blit routine to copy split screen bands around a 200-line scroll position using a4 globals.
- `scan_columns_for_target_tile`: Adds per-column offsets to a base position, scans a width of columns calling the tile lookup, increments a hit counter per tile and returns 1 if the target tile is found.
- `read_4bitplane_pixel_sequential`: Reads four consecutive words from a bitplane buffer at an offset computed from x, y and row stride, tests one bit per word and returns a 4-bit pixel value in d0.
- `read_4bitplane_pixel_strided`: Reads four words spaced 8000 bytes apart from a bitplane buffer at an offset computed from x, y and row stride, tests one bit per word and returns a 4-bit pixel value in d0.
- `demo_main_loop_setup_and_run`: Initializes many game state variables and text buffers, then runs the main demo loop calling setup, input, update and rendering routines until exit.
- `zero_word_data_at_35f6`: This single zero word decodes as ori.b #0,d0 but is zero-filled data referenced by address, not executable code.
- `zero_filled_data_word_at_35fa`: Contains a single zero word that decodes as an ori.b immediate instruction; it is zero-filled data referenced by address, not executable code.
- `wait_for_byte_33_and_call_sequence`: Polls the byte pointed to by the a4-relative pointer until it equals and then differs from 51, calling several subroutines, then returns the word at -2(a5) in d0.
- `update_entity_list_by_type`: Walks the linked entity list at the a4 base, dispatching type 0, 2, 3 and 4 entries to sprite, sound and animation calls while pushing their position and frame fields as arguments.
- `draw_entity_sprites_with_flags`: Iterates the entity list, allocates a sprite buffer for type 5 and matching type 21 entries, then draws each with position offsets and releases the buffer.
- `draw_entity_sprites_with_bit_flags`: Iterates the entity list, allocates a sprite buffer for type 5 and matching type 21 entries, then draws each with position offsets and four low flag bits.
- `walk_entity_list_and_dispatch`: Walks a linked list of entity records starting from a global pointer, calling the per-entity update routine for each record whose word field is not minus one.
- `update_entity_animation_and_collision`: Advances an entity's animation frame, looks up sprite offsets and movement data, issues draw calls, and tests directional collision flags, clearing global input flags on hits.
- `compute_entity_direction_code`: Reads the entity's two direction words at offsets 22 and 20 and returns a code from zero to four selecting the facing direction.
- `apply_sprite_frame_offsets_to_entity`: Reads a sprite frame index and entity pointer from the stack frame, computes a frame record, calls a collision check, and adds frame deltas to entity position and velocity fields, returning zero or one.
- `set_entity_direction_from_offsets`: Clears two direction words in the entity structure, then compares signed offset arguments against thresholds to set each direction word to one or two.
- `walk_entity_list_and_process_active`: Iterates a linked list from a global pointer, saving the next link, and calls the processing routine for each node whose field at offset 212 is not minus one.
- `walk_object_list_and_dispatch_active`: Walks a linked list from a global pointer, saving each next pointer, and calls the dispatch routine for every node whose word at offset 212 is not minus one.
- `read_script_byte_and_dispatch_opcode`: Reads a byte from the script pointer at offset 208 of the object, advances the script index, and jumps through a word table to handle the opcode, setting a flag when a global word is nonzero.
- `scan_script_for_bytecode_offsets`: Clears fifty words in the output buffer to minus one, then walks the script bytes, advancing by each opcode's length and recording offsets for opcode 30 entries.
- `blit_masked_span_with_plane_masks`: Clips a horizontal span against the 160-pixel playfield, builds rotated plane masks from four source words and the four plane-enable flags, then merges them into the destination bitplanes at a1.
- `restore_all_registers_and_unlink_frame`: Skips a zero word, adds four to the stack pointer, restores d0-d7 and a0-a6 from the stack, releases the a5 frame and returns to the caller.
- `blit_span_planes_loop`: Clips a span to the playfield, then loops over rows writing rotated plane masks and source words into four interleaved bitplanes at a1, advancing a0 and a1 each row.
- `restore_registers_skip_word_and_return`: Discards a zero word, advances the stack pointer by four, pops d0-d7 and a0-a6, releases the a5 frame and returns to the caller.
- `blit_masked_sprite_columns_to_bitplanes`: Clips a sprite rectangle against screen bounds, then for each column reads source words, rotates them by a bit offset and merges masked bits into four interleaved bitplane rows at offsets 8000, 16000 and 24000 from the destination pointers.
- `restore_all_regs_and_unlink_frame`: Adds four to the stack pointer, pops d0 through d7 and a0 through a6 from the stack, releases the a5 frame and returns to the caller.
- `blit_masked_sprite_columns_to_bitplanes_2`: Clips a sprite rectangle against the 160-pixel screen, then for each row reads source words, rotates and inverts them by the low nibble of the flags, and ANDs the mask into four interleaved bitplanes at offsets 8000, 16000 and 24000.
- `pop_saved_registers_and_return`: Skips four bytes on the stack, restores d0 through d7 and a0 through a6 from the stack, then releases the a5 frame and returns.
- `push_blit_params_and_call_blitter_setup`: Builds a fourteen-word parameter block on the stack with fixed blitter values, the two long arguments from the caller's frame, then calls the blitter setup routine and discards the arguments.
- `init_four_plane_blit_offsets`: Clears fields in the object pointed to by the a4 global, stores two long parameters at offsets 80 and 84, sets plane size and control words, then calls the blit routine four times with source offsets 0, 8000, 16000 and 24000.
- `draw_four_plane_rectangles`: Pushes identical rectangle parameters and four destination pointers spaced 8000 bytes apart onto the stack, calling the drawing routine four times to cover all four bitplanes.
- `empty_c_function_4`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `blit_four_planes_if_moved`: Gets a base pointer, builds four plane masks from flag bits, and if the two coordinate pairs differ calls the blit routine four times at offsets 0, 8000, 16000 and 24000.
- `draw_four_band_rect_with_flags`: Computes tile-aligned rectangle bounds from four word arguments, builds four blitter fill records at offsets 0, 8000, 16000 and 24000 with colors chosen by flag bits, then calls the rectangle drawer and a final helper.
- `draw_single_band_rect_at_base`: Computes tile-aligned rectangle bounds, adds the base pointer from a4-relative memory plus 32000, selects one fill color from a flag word and calls the rectangle drawer once.
- `draw_single_band_rect_at_offset`: Computes tile-aligned rectangle bounds, adds the caller-supplied long offset plus 32000, selects one fill color from a flag word and calls the rectangle drawer once.
- `render_text_block_with_kerning`: Reads a text string from a font table, computes line spacing and character widths, then draws each glyph into the bitplane buffers at the given screen coordinates.
- `measure_text_line_max_width`: Scans the string at a2 until a null byte, summing per-character widths from the font table and returning the widest space- or pipe-delimited segment in d0.
- `mask_glyph_rows_into_bitplanes`: For eight rows, reads words from the glyph source at a0, rotates by the low nibble of d0, inverts and ANDs them into the destination bitplane at a1.
- `parse_text_into_wrapped_lines`: Reads characters from the string pointer at 8(a5), tracks line width in d5 and line count in d4, uses the byte table at a4-32201 for character widths, wraps at spaces or the pipe character, and stores the resulting column and row values through the three output pointers.
- `find_or_insert_sorted_list_node`: Walks the linked list at a4-30532 comparing each node's priority word against the priority of the entry indexed by the argument, stops at nodes of type 2 or 3, and returns the matching node or inserts a new type-20 node.
- `match_list_node_by_id_and_type`: Checks the node pointer at -4(a5) for a matching id word at offset 8, type 20 at offset 10 and value at offset 12, returning minus one when all match, otherwise falling through to the insertion path.
- `allocate_and_link_16_byte_node`: Calls the allocator with size 22, stores the returned pointer, copies the caller's pointer into the new node, fills fields at offsets 8 to 18 from arguments and constants, then links the node into the list at a4-relative base.
- `render_wrapped_text_block`: Measures a null-terminated string using a character width table, computes a centered box height capped at 300, draws the box and each wrapped line of glyphs into the screen buffers, and returns when the string ends.
- `add_rgb_nibbles_clamped`: Doubles three delta words, splits the packed color word into red, green and blue nibbles, adds each delta with clamping to the range 0 to 15, and returns the repacked color word in d0.
- `fill_sprite_table_with_ff_and_clear_flag`: Calls a routine sixteen times per row for sixteen rows to fill the sprite table at offset -2350 from a4 with returned words, then frees two pointers, sets the flag at -32224 to one and returns.
- `animate_sprite_rows_from_frame_table`: Loops sixteen rows, reading words from either the base table or a frame-selected table at -2834 from a4, passing them with negated row values to a routine, storing results at -2350, then clears the flag at -32224.
- `copy_sprite_range_from_table`: Reads start and end indices from the stack frame, loops over that range, fetches words from the table at -2382 from a4, passes them with three stack arguments to a routine and stores results at -2350.
- `load_word_from_table_at_index_c`: Compares the word at offset twelve of the a5 frame with one, and if equal loads d5 from the word table at a4 minus 2350 indexed by the word at offset ten, then copies that index into d4 and branches away.
- `shift_word_table_down_and_store_d5`: Copies each word of the a4-relative table down one slot while d4 stays above the frame word at offset eight, then stores d5 at that index and branches to the caller's continuation.
- `load_word_from_table_at_index_eight`: Compares the word at offset twelve of the a5 frame with two, and if equal loads d5 from the word table at a4 minus 2350 indexed by the word at offset eight, then copies that index into d4 and branches away.
- `shift_word_array_left_by_one`: Copies each word from the array at offset -2350 in a4 one slot lower, incrementing d4 until it reaches the count at offset 10 in a5, then stores d5 at the end slot and branches onward.
- `load_count_word_into_d4`: Loads the word at offset 8 in a5 into d4 and branches to the shared loop tail, setting up the counter for the following array copy.
- `copy_counted_words_then_call`: Copies words from the array at offset -2382 in a4 into the array at offset -2350 while d4 is at most the count at offset 10 in a5, then pushes the destination, calls a routine, restores d4-d5 and returns.
- `reset_entity_table_and_open_database`: Clears 255 entity records of 30 bytes each in the table pointed to by a4-relative memory, resets two counters, then closes any open database file and opens the named database, reading width and height and allocating the record buffer.
- `clear_entity_table_and_close_database`: Clears 255 entity records of 30 bytes each in the table pointed to by a4-relative memory, then if a database file handle is open, closes it and marks the handle invalid.
- `find_entity_by_name_string`: If no database is open returns minus one; otherwise uppercases the caller's string in place, then scans up to 255 entity records comparing each against the string and returns the matching index or minus one.
- `find_object_by_excluding_type`: Scans the object table for an entry whose type field differs from the argument, then tests each of its sub-entries via a helper and returns the matching index or minus one.
- `find_object_by_matching_type`: Scans the object table for an entry whose type field equals the argument, then tests each of its sub-entries via a helper and returns the matching index or minus one.
- `lookup_object_with_config_fallback`: Looks up an object by name, tries matching and excluding type searches, and on failure formats a not-found message, sets up display colors and returns minus one.
- `set_sprite_xy_from_entity`: Looks up an entity record by index times thirty bytes, then calls the graphics routines to set its horizontal and vertical screen coordinates from the two arguments.
- `copy_bytes_advance_source_pointer`: Copies a given number of bytes from the buffer pointed to by the first argument into the destination buffer, then stores the advanced source pointer back.
- `load_iff_palette_and_set_colors`: Opens a file with mode value, reads two words as width and height, multiplies them, reads that many bytes into the given buffer, then closes the file.
- `lzss_decompress_to_buffer`: Reads compressed data from the stream at a0 and writes decompressed bytes backwards into the buffer ending at a1, using bit flags from d0 and a checksum in d5, then restores all registers and returns.
- `refill_bit_buffer_with_checksum`: Pops the next longword from a0, xors it into d5, and rotates it right through the carry to supply fresh bits in d0, returning with carry set if the stream is exhausted.
- `extract_bits_into_d2`: Shifts d0 right by one bit per iteration, refilling from a0 and xoring into d5 when empty, accumulating the extracted bits into d2 for the requested count in d1.
- `clear_50_slot_object_table`: Loops over 50 entries of 106-byte records, freeing each non-null object pointer and clearing its pointer and word fields, then opens a file and processes its records.
- `load_and_apply_32byte_record_table`: Opens a file, reads a count and 32-byte records, skips entries marked as terminator, writes each record to the file, then processes the loaded records.
- `clear_255_slot_14byte_table`: Loops over 255 entries of 14-byte records, freeing each non-null pointer and clearing its pointer and word fields, then opens a file and reads records.
- `load_and_play_sound_slots`: Clears the 255-entry sound slot table, then loads a sound bank from the resource named by the caller, plays each slot's sample through the audio routines and frees the bank.
- `update_four_channel_sound_slots`: Checks four channel-active flags in a4-relative memory and for each active one stops the old sample, rebuilds its slot entry and restarts playback with the channel's volume.
- `zero_fill_data_table`: This is a single zero word that decodes as ori.b #0,d0; it is a zero-filled data byte referenced by address, not executable code.
- `build_and_show_volume_prompt`: Stores the passed volume value, formats the volume and drive text into stack buffers, then loops calling the input and display routines until the user confirms or cancels.
- `poll_mouse_until_button_release`: Repeatedly calls the input polling routine with stack result words, checking the button flag and waiting until the mouse button is released before returning.
- `setup_screen_and_play_intro`: Calls display setup routines, copies screen pointers, configures a large copper or blit list with dimensions and pointers, then restores the display and returns.
- `apply_weapon_slot_and_update_offsets`: Saves d4, compares the requested slot with the cached one, adjusts two global offsets by the difference of the slot's stored values, copies sixteen pairs of words into the object at the pointer, clears two words and calls two helpers.
- `reset_weapon_slot_to_first_entry`: Copies the first slot's two words into the cached values, writes two marker words into the object, fills sixteen pairs of words from the first slot, clears two words and resets the cached slot index to zero.
- `install_vblank_interrupt_handler`: Saves all registers, stores a4 in a global, disables the vertical blank interrupt, saves the old level 3 vector and installs a new handler, then re-enables the interrupt.
- `install_vblank_handler_and_enable_dma`: Saves all registers, enables the vertical blank interrupt via INTENA, stores the address of the routine at the program counter offset into the level 3 vector, then enables DMA and returns.
- `zero_filled_data_area`: A single zero word that decodes as ori.b #0,d0; this is zero-filled data referenced by address, not executable code.
- `level3_interrupt_dispatch`: Saves d0 and status, reads INTREQR, and if the vertical blank bit is set it swaps in a4, calls the handler pointed to by the stored vector, restores state and jumps to the old vector.
- `read_joy0dat_mouse_delta`: Saves registers, reads JOY0DAT mouse counters, extracts vertical and horizontal bytes, subtracts previous values stored in a4-relative memory, wraps deltas past 128, and accumulates them into the mouse position words.
- `wrap_vertical_delta_accumulate`: Compares d6 against minus 128, adds 255 when below, adds the wrapped vertical delta into the a4-relative mouse position word, then computes the horizontal delta from d4 and the stored previous value.
- `clamp_mouse_and_write_copper_pointer`: Stores the latest mouse deltas, clamps the vertical and horizontal position words to screen bounds adjusted by a4-relative offsets, packs them into pointer sprite words, and writes the sprite pointer to COP1LCH.
- `blitter_line_draw_setup`: Reads line endpoints and color from the stack, computes blitter line registers including octant table lookup, writes BLTCON0/1, BLTAMOD/BMOD/CMOD/DMOD, BLTAPTH/DPTH, BLTSIZE and BLTADAT, then waits for DMACONR blitter busy to clear.
- `copy_pascal_string_from_table`: Takes a table index and destination pointer, reads the length byte at index times four, copies that many following bytes to the destination and appends a null terminator.
- `match_string_against_list`: Walks a linked list of entries from a global structure, copies each entry's string via the string copy routine and compares it with the argument, returning one when a match is found or zero otherwise.
- `find_trackdisk_device_and_read_flags`: Calls exec FindName for trackdisk.device, indexes the returned device node by the word at offset 8 of the frame, and builds a flag word from bytes 64, 66 and bit 0 of byte 34, returning -1 if absent.
- `open_trackdisk_device_and_init`: Calls a routine with the string at offset 10 of the frame and a constant, stores the result, and if non-null passes it to another routine, returning zero on success or -1 on failure.
- `blit_rotated_tile_into_bitmap`: Copies a source rectangle into a destination bitmap, then blits rows with rotation and per-plane OR/AND masks controlled by flags at offset 35 of the frame, advancing by the stored row strides.
- `load_and_play_stealth_demo_files`: Opens the demo file, allocates buffers, reads and plays fifteen chunked data blocks through the dos file handle, then marks the load-complete flag and returns.
- `read_file_size_via_examine`: Allocates a file-info buffer, locks and examines the named file, copies its size from offset 124, unlocks, frees the buffer and returns the size in d0.
- `save_regs_set_flag_and_return`: Pushes d1 through d7 and a0 through a6 onto the stack, writes the value one to the word at address 1310, restores the registers and returns.
- `wait_for_sync_flag_64`: Saves registers, sets the word at address 1310 to 1 if it is zero, then spins until that word equals 64 before restoring registers and returning.
- `open_ciaa_and_start_audio`: If the enable word at 1248 is set, clears two state words, reads chip data into 1433 and 1436, opens the ciaa resource if needed, calls four setup routines and returns nonzero on failure.
- `close_ciaa_resource_if_active`: If the active word at 1250 is set, closes the resource through the saved base pointer, calls the teardown routine, clears that word and returns zero in d0.
- `free_tracked_buffers_and_reset_flags`: Saves all registers, then if the active flag is set frees the main buffer and fourteen paired buffers through exec FreeMem, clearing each pointer and the two flag words.
- `find_max_byte_in_table_scaled`: Scans a byte table for the largest value, then returns that maximum plus one multiplied by 1024 in d0, preserving d1 and restoring saved registers.
- `set_cia_timer_and_audio_registers`: Reads CIAA control register, masks to top two bits, sets bit zero, writes it back, then copies two bytes from a table to the audio and CIAA timer registers.
- `clear_fifteen_audio_channel_pointers`: Loads the address of a fifteen-entry pointer table into a0, then for each entry loads the pointed-to longword address into a1 and clears that longword to zero, counting down d7 from fourteen to zero.
- `silence_all_four_audio_volumes`: Writes all ones to the audio interrupt enable register, then clears the volume registers of audio channels zero through three and clears three audio control words near the custom chip base.
- `reset_audio_channels_to_silent_sample`: Clears the audio interrupt enable word and all four channel volumes, sets each channel length to one, points all four channel pointers at a shared silent sample, and enables audio DMA.
- `return_zero_after_audio_tick`: Calls the audio tick routine, then sets d0 to zero and returns to the caller.
- `six_frame_audio_and_volume_step`: Saves all registers, counts frames modulo six, steps the volume counter up to sixty-four and calls the audio update routine.
- `restore_registers_after_audio_tick`: Calls the audio tick routine, then pops d0-d7 and a0-a6 from the stack and returns to the caller.
- `init_four_audio_channels`: Loads four into d7, points a4, a6 and a5 at table and audio register bases, then calls the channel setup routine four times while advancing the pointers.
- `step_audio_channel_forward`: Reads a word from a4; if zero returns, if negative branches away, otherwise adds it to the word at offset two, clamps against the word at offset four, and writes the result to a5 and a4.
- `step_audio_channel_backward`: Adds d0 to the word at offset two of a4, reads the limit at offset four, and if the new value is below it copies the limit into d0 before branching to the shared store code.
- `dispatch_command_nibble_to_actions`: Reads the low nibble of byte at offset 2 from a6 and branches to one of six action routines for values 1, 2, 3, 4, 7 or 8, otherwise returns.
- `set_ciaa_pra_bit1`: Sets bit 1 of the CIAA port A register at the fixed hardware address, then returns to the caller.
- `clear_ciaa_pra_bit1`: Clears bit 1 of the CIAA port A register at the fixed hardware address, then returns to the caller.
- `lookup_pair_index_by_first_word`: Reads a word and nibble fields from the record at a6, stores them into the output at a4, and searches a word table for a matching key to store the paired value.
- `store_indexed_word_to_a4_offset4`: Doubles d2, negates it when d4 is nonzero, loads the word at a0 plus that offset into d0 and stores it at offset four of a4 before returning.
- `store_d0_word_to_a4_offset4`: Copies the low word of d0 into the word at offset four of a4 and returns to the caller.
- `store_d4_byte_then_branch`: Stores the low byte of d4 to memory at the address in a4, then branches unconditionally to another routine's code.
- `jump_table_dispatch_by_counter`: Loads a table base into a0, reads a word counter from memory, decrements it, scales by four and jumps through the selected table entry.
- `scan_word_table_for_match`: Doubles d0, loads a word from offset sixteen of a6, then walks a word table comparing entries until one equals it, leaving a0 at the match.
- `lookup_high_nibble_indexed_value`: Clears d0, loads the byte at offset 3 of a6, shifts it right by four bits, then branches to a shared table lookup that stores the indexed word at offset 6 of a5.
- `lookup_low_nibble_indexed_value`: Clears d0, loads the byte at offset 3 of a6, masks it with fifteen, then branches to a shared table lookup that stores the indexed word at offset 6 of a5.
- `copy_word_from_a6_to_a5`: Reads the word at offset 16 of a6 into d2 and writes that same word to offset 6 of a5, then returns to the caller.
- `add_high_nibble_to_word_at_a6`: Clears d0, loads the byte at offset 3 of a6, shifts it right by four bits, and if the result is nonzero adds it to the word at (a6) and stores that word at offset 6 of a5.
- `subtract_low_nibble_from_word_at_a6`: Clears d0, loads the byte at offset 3 of a6, masks it with fifteen, and if nonzero subtracts it from the word at (a6) and copies that word to offset 6 of a5.
- `update_audio_channels_and_advance_sample`: Writes four channel sample pointers and lengths to the audio hardware, enables DMA, then advances the sample offset and frame index, wrapping when the frame count is reached.
- `build_sprite_record_from_table`: Clears the record at a6, reads a table entry indexed by d1, copies its pointer and position fields into the record, and computes the sprite address from a3 plus a scaled index.
- `apply_vertical_offset_to_sprite`: Adds d3 to the record's base pointer, stores the result at offset 10, copies a table word to offset 14, then adjusts the vertical value by the byte at offset 3 depending on the type nibble.
- `clamp_vertical_position_to_screen`: Adds the byte at offset 3 to d3, caps it at 64, subtracts the global scroll value, stores the clamped result at a5 offset 8, and clears the record type when it equals minus three.
- `audio_channel_dma_restart`: Checks the channel structure word, clears a flag, copies the period and DMA control word to the custom chip register, busy-waits on a counter, then reloads the channel pointer and sets the active bit.
- `copy_channel_pointer_and_set_active`: Copies the sample pointer, length and period from the channel structure into the active slot, doubles the period word, and ORs the DMA control value into the shared audio mask.
- `setup_audio_channel_and_start_dma`: Builds the channel register base from the channel index, clears the volume, writes period, length and pointer, sets the DMA enable bit, waits a fixed count, then installs the interrupt vector and starts playback.
- `check_param_against_ten`: Sets up an a5 frame, saves d4-d6, compares the word parameter at offset 8 with ten and branches away if greater or equal, otherwise tests it for zero.
- `clear_buffer_and_draw_two_spans`: Clears the first word of the buffer and the word before the indexed offset, then calls the span routine twice with different color and mode arguments.
- `reset_channel_volume_and_clear_buffer`: Subtracts ten from the channel volume, clears the first and last words of the sample buffer, clamps the length to sixty-three, then calls the mixer with the channel parameters.
- `test_stack_arg_10_and_return`: Opens an a5 stack frame, tests the 16-bit word at offset 16 in the frame, and if zero unlinks the frame and returns to the caller.
- `test_stack_arg_14_branch_to_common_exit`: Tests the 16-bit word at offset 14 in the a5 frame and branches to the shared exit path when it is zero, otherwise falls through.
- `clear_buffer_and_draw_digit_pair`: Clears the first word of the buffer at frame offset 10 and the word before the indexed entry, then pushes draw arguments including a value divided by ten and calls the drawing routine.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `open_ciaa_and_audio_resources`: Saves registers, checks the init flag, opens the ciaa resource and a second library, stores both bases, calls three setup routines, sets the init flag and returns nonzero on failure.
- `close_ciaa_and_audio_resources`: Saves registers, and if the init flag is set closes the library at the stored base, calls the teardown routine, clears the init flag and restores registers.
- `start_ciaa_timers_and_set_ports`: Reads the CIAA control register, forces the low bits to one, writes it back, then copies two bytes from a local table to the CIAA port registers.
- `silence_audio_channels_and_clear_voices`: Clears volume registers for all four audio channels, points each channel's location register at a shared sample, then zeroes four voice structure fields and returns.
- `silence_audio_and_enable_dma_channels`: Clears all four audio channel volumes, points their location registers at a shared sample, enables DMA channels via DMACON, then zeroes four voice structure fields and returns.
- `save_registers_call_ed60_clear_d0`: Pushes all data and address registers, calls the routine at the next address, restores the registers, sets d0 to zero and returns.
- `increment_channel_counter_and_init_audio`: Compares the word at memory offset 714 with 6, increments it if below, then loops four times calling the audio init routine with a5 stepping through AUD registers and a6 through a table.
- `reset_audio_channels_and_enable_dma`: Clears two counters, initializes four audio channels via a call, sets DMACON bit 15 to enable DMA, delays, then programs four AUD pointers from a table when their entries are empty.
- `fill_audio_channel_struct_from_stack_args`: Builds a channel record at table base plus index times 22, storing stack arguments into its fields and copying one word into another before returning.
- `decrement_timer_at_a6_offset_e`: Tests the word at offset 14 of a6; if non-zero, subtracts one from it and returns, otherwise returns without changing it.
- `clamp_and_store_vertical_position`: Copies the word at offset 12 of a6 to offset 14, adds offset 10 to offset 18, clamps the sum to 0 through 63, then stores it at a6 offset 18 and a5 offset 8.
- `copy_sprite_record_and_set_scroll_bits`: If the longword at a6 offset 4 is set, copies it to a5, fills a5 offsets 4, 6 and 8 from a6 fields, clears a6 offset 4 when a6 offset 16 is zero, and ORs a6 offset 20 into the hardware register at 1814.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `open_manx_file_and_start_playback`: Allocates a MANX-tagged buffer, opens the module file, initialises the player structure fields, starts the replay via the audio hook and returns after restoring a2 and the frame.
- `copy_d0_to_d5_2`: Copies the longword in d0 into d5 and falls through into the following code; it is used as a small callback address.
- `load_module_and_install_replay`: Reads the module header, allocates and copies sample and pattern data, sets up the replay tables and clears the module pointer before returning zero in d0.
- `build_and_install_display_list`: Compares a string against a table, walks a linked list counting nodes, allocates a block, fills node entries with pointers and counts, then calls two routines and stores results into caller-supplied pointers.
- `save_d0_into_d5`: Copies the longword in d0 into d5 and falls through into the following routine without returning.
- `copy_string_return_destination`: Loads destination and source pointers from the stack, copies bytes from source to destination until a zero byte, and returns the destination pointer in d0.
- `strlen_return_length_in_d0`: Loads the return address from the stack into a0, scans bytes until a null terminator, then returns the string length in d0.
- `to_lowercase_ascii_in_d0`: Reads the byte argument from the stack into d0 and, if it is between lowercase a and z, subtracts 32 to produce an uppercase value.
- `to_uppercase_ascii_in_d0`: Reads the byte argument from the stack into d0 and, if it is between uppercase A and Z, adds 32 to produce a lowercase value.
- `compare_strings_return_order`: Compares two null-terminated strings byte by byte up to 32767 bytes, returning zero if equal, one if the first is greater, or minus one if less.
- `copy_memory_block_forward_or_backward`: Copies a byte count from source to destination, moving forward when destination is below source and backward when above to avoid overlap.
- `clear_bss_open_dos_and_start_demo`: Clears a large data area, saves the stack and exec base, opens dos.library, alerts on failure, then jumps to the demo entry routine.
- `set_a4_to_high_memory_pointer`: Loads the absolute address 32766 into address register a4 and returns immediately to the caller.
- `tokenize_text_into_argument_vector`: Allocates a buffer, copies a string, splits it on whitespace and quotes into null-terminated tokens, builds a pointer array, and stores the token count in a4-relative globals.
- `no_operation_move_d0_to_d0`: Copies d0 onto itself, an empty no-operation instruction that leaves all registers unchanged and falls through to the following code.
- `return_7fff_in_d0`: Loads the immediate value 32767 into d0 and branches to another routine, providing that constant as an argument.
- `append_string_with_limit`: Finds the end of the destination string in a0, then copies bytes from a1 into it up to the count in d0, null-terminating, and returns the destination pointer in d0.
- `copy_string_then_pad_zeros`: Copies bytes from a1 to a0 until a null or the count in d1 is reached, then fills the remaining count with zero bytes and returns the destination in d0.
- `open_window_and_load_icon_objects`: Opens a window via a library call, stores the handle, then fetches an object and its sub-objects, saving icon data pointers into the a2 structure and freeing the temporary handles.
- `copy_d0_to_d5_3`: Copies the longword in d0 into d5 and then falls through into the following routine, so it only moves that register value.
- `set_global_and_clear_first_byte`: Stores the first argument in a global pointer, calls a routine with the second argument and a callback, then clears the first byte of the stored pointer and returns the call result in d0.
- `store_byte_to_output_pointer`: Reads the byte argument from the stack, writes it through the pointer stored at offset -7806 in a4, increments that pointer, and returns the byte sign-extended in d0.
- `format_number_into_buffer_backwards`: Converts a number from the argument block into digits using the radix and width given, writing characters backwards from a2 and calling the divide and remainder helpers, then prepends a minus sign when negative.
- `format_string_with_percent_directives`: Parses a format string, handling percent flags, width, precision and conversion letters, calling the character output function and the number formatter to build the result and returning the count or minus one.
- `signed_divide_with_sign_fixup`: Saves d1 and d4, takes absolute values of d0 and d1 while tracking signs in d4, calls the unsigned divide helper, then negates the quotient if the combined sign flag is set and returns it in d0.
- `signed_divide_returning_remainder`: Saves d1 and d4, makes d0 and d1 positive while recording signs in d4, calls the unsigned divide helper, then moves the remainder from d1 into d0 and branches into the shared sign-fixup tail.
- `divide_d1_by_d0_keep_d1`: Pushes d1, calls the unsigned divide helper with d0 and d1, moves the quotient from d1 into d0, restores the original d1 from the stack and returns with the result in d0.
- `call_divmod_preserve_d1`: Saves d1 on the stack, calls the division routine at the next address, restores d1, tests the longword result in d0 and returns.
- `unsigned_32bit_divide_d0_by_d1`: Divides the longword in d0 by the longword in d1, using a fast word divide when the divisor's high word is zero and a 16-step shift-subtract loop otherwise, returning quotient and remainder in d0 and d1.
- `invoke_optional_callback_then_word_call`: If the longword at offset minus 1028 from a4 is nonzero, calls the code it points to, then pushes the word argument from the stack frame and calls the routine at the following address.
- `shutdown_demo_and_free_resources`: Saves the argument, frees each of the six-byte entries via the free routine, calls optional callbacks and cleanup routines, invokes Supervisor when the exec flag is set, then restores the stack and returns the saved value in d0.
- `restore_d4_unlink_and_return`: Pops d4 from the stack, releases the a5 frame with unlk and returns to the caller; it is an empty epilogue reached only by fallthrough.
- `free_sound_entry_by_index`: Looks up the six-byte entry at the given index in the sound table, validates the index and pointer, frees the entry unless flagged, clears the pointer and returns zero, or sets an error and returns minus one.
- `jump_via_a4_vector_minus_36`: Loads a pointer from the table at offset minus 1194 from a4, then jumps through the entry 36 bytes before that pointer.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `jump_via_a4_vector_minus_126`: Loads a pointer from the table at offset minus 1194 from a4, then jumps through the entry 126 bytes before that pointer.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_dup_lock`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `jump_via_a4_vector_minus_102`: Loads a pointer from the table at offset minus 1194 from a4, then jumps through the entry 102 bytes before that pointer.
- `call_examine`: Wrapper that calls the dos library function Examine and returns its result to the caller.
- `call_info`: Wrapper that calls the dos library function Info and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `jump_via_a4_table_offset_1e`: Loads a pointer from the table at offset minus one thousand one hundred ninety-four from a4, then jumps to the entry at offset minus thirty within that table.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `jump_via_a4_table_offset_19e`: Loads a pointer from the table at offset minus one thousand two hundred twelve from a4, then jumps to the entry at offset minus four hundred fourteen within that table.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `jump_via_a4_table_offset_c6`: Loads a pointer from the table at offset minus one thousand two hundred twelve from a4, then jumps to the entry at offset minus one hundred ninety-eight within that table.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `jump_via_a4_offset_minus_132`: Loads a pointer from the structure at a4 minus 1212, then jumps to the routine at offset minus 132 in that table.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `jump_via_a4_offset_minus_210`: Loads a pointer from the structure at a4 minus 1212, then jumps to the routine at offset minus 210 in that table.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `jump_via_a4_offset_minus_552`: Loads a pointer from the structure at a4 minus 1212, then jumps to the routine at offset minus 552 in that table.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.

### run

- `allocate_and_init_script_task`: Looks up a free task slot via library calls, allocates memory, fills a header and instruction table, copies sixteen longs into the task block and returns zero or minus one.
- `copy_d1_to_d0_and_branch`: Moves d1 into d0 and branches forward into another routine, so it only forwards the value as the return code.

### else

- `allocate_and_init_else_task`: Same allocation and initialization sequence as the run routine but entered from the else file, filling a task block and returning zero or minus one.
- `copy_d1_to_d0_and_branch_2`: Moves the longword in d1 into d0 as the return value, then branches to another routine instead of returning here.

### Binddrivers

- `binddrivers_open_dos_and_parse_args`: Opens dos.library, allocates a task structure, parses the command line into argument pointers and strings, then runs the driver binding routine and frees resources.
- `binddrivers_scan_expansion_and_bind`: Opens expansion and icon libraries, walks the expansion board list, loads each board's driver file, binds it via the driver routine and releases all opened resources.
- `find_marker_in_driver_list`: Scans a list of driver entries for a marker word matching its own address, returning the entry pointer or zero if none found within the size limit.
- `iterate_driver_entries_by_separator`: Repeatedly finds separator-delimited driver entries starting at a base pointer, accumulating results until no further entry is found, then returns the accumulated value.
- `process_driver_entry_range`: Locates a separator within a driver entry, checks and adjusts the entry boundaries, then passes the resulting range to a final processing call and returns its result.
- `bind_driver_chain_by_priority`: Walks the driver list via the next pointer at offset 48, and for each node whose flags byte at offset 14 has bit 1 set, checks with the match routine and prepends it to the chain, returning the new head in d0.
- `find_node_in_driver_chain`: Follows the next pointers at offset 48 from the first argument, comparing each node address with the second argument, and returns 1 in d0 if found or 0 when the chain ends.
- `check_string_is_all_digits`: Scans the byte string in a0 for up to d1 characters, returning 1 in d0 if the count is zero or all bytes are between ASCII 0 and 9, otherwise 0.
- `parse_decimal_string_to_long`: Walks the null-terminated string pointed to by a0, accumulating each digit character into d0 as value times ten plus digit, and returns the result in d0.
- `bounded_string_to_number_check`: Converts the string at the first argument with the given length via a helper, then returns that value only if it does not exceed the start plus the third argument, otherwise returns zero.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_120_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_126_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_alert`: Library stub: loads its arguments and the exec library base, calls Alert and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `call_user_state`: Library stub: loads its arguments and the exec library base, calls UserState and returns its result to the caller.
- `compare_two_null_terminated_strings`: Compares bytes from a0 and a1 until a difference or a null byte, returning zero when equal, minus one when a0 is lower, and one when a0 is higher in d0.
- `load_first_arg_pointer`: Loads the longword at offset 4 from the stack pointer into a0, fetching the first argument pointer for the following code.
- `measure_string_length_up_to_255`: Scans bytes at a0 until a zero byte or 255 bytes counted, advancing a0, then returns the bitwise complement of the count in d0.
- `find_character_in_string`: Reads a character argument from the stack into d0 and scans the string at a0 for it, returning a pointer to the match or zero if the terminator is reached.
- `copy_string_until_nul`: Loads two pointers from the stack, copies bytes from the second pointer to the first until a zero byte is copied, then returns.
- `find_common_prefix_length`: Measures the lengths of two strings, compares them, and if the first is not shorter, checks whether the second is a prefix of the first, returning the remaining length or zero.
- `call_library_vector_78_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.

### delete

- `allocate_and_init_driver_node`: Calls library routines through a5 to find the largest node value, allocate a block, fill it with incrementing longwords, copy a sixteen-longword structure, and return zero or minus one.
- `copy_d1_to_d0_then_branch`: Moves the longword in d1 into d0 and then branches unconditionally to another part of the same routine.

### dir

- `dir_open_and_scan_entries`: Calls the dos library to open a directory, finds the largest entry size, allocates a buffer, fills it with entry records, and returns zero or minus one.
- `copy_d1_to_d0_and_branch_forward`: Moves the value in d1 into d0 and branches forward to another routine, acting as a small argument-passing trampoline.

### echo

- `echo_open_and_allocate_buffer`: Calls the library open vector, walks a linked list to find the largest entry, allocates a buffer, fills it with incrementing values, copies sixteen longs to and from the echo structure, installs a callback and returns zero or minus one.
- `return_d1_in_d0_and_branch`: Copies d1 into d0 as the return value and branches to another routine, transferring control without touching memory or the stack.

### failat

- `failat_open_and_allocate_buffer`: Calls the library open vector, scans a linked list for the largest entry, allocates a buffer, fills it with incrementing values, copies sixteen longs to and from the failat structure, installs a callback and returns zero or minus one.
- `copy_d1_to_d0_then_branch_2`: Copies d1 into d0 and branches to another routine, so the value in d1 becomes the return value passed along.

### copy

- `load_and_install_segment_list`: Reads a segment list, finds the largest entry, allocates space, fills it with incrementing values, copies sixteen longs into a table, installs a callback and returns zero or minus one.
- `copy_d1_to_d0_and_branch_5`: Moves the longword in d1 into d0, then branches to another routine without returning here.

### cd

- `load_and_init_module_table`: Calls library routines through a5 to allocate and fill a module table at offset 536 of a2, copies 16 longs, sets a callback at offset 8, and returns zero or minus one.
- `copy_d1_to_d0_and_branch_3`: Moves d1 into d0 and branches forward into another routine, so it returns d1 as the result via that shared tail.

### endif

- `return_zero_clear_d0`: Clears d0 to zero with eor and returns immediately to the caller.

### if

- `open_and_allocate_if_file`: Calls the library open routine through a5, walks a linked list to find the largest entry, allocates a block, fills it with incrementing values, copies sixteen longs to and from offset 536 of a2, and returns zero or minus one.
- `move_d1_to_d0_and_branch`: Copies d1 into d0 and branches to a fixed target inside another routine, so it only forwards the value and transfers control.

### wait

- `allocate_and_initialize_wait_structure`: Calls library routines to find the largest node value, allocates a block, fills it with incrementing longwords, copies sixteen longwords into a structure at offset 536 of a2, and returns zero or minus one.
- `return_d1_in_d0_then_branch`: Copies d1 into d0 as the return value, then branches to another routine instead of returning here.

### ram-handler

- `copy_mem_then_jump_to_a6`: Saves a1 and a6, moves d1 and d2 into a0 and a1, calls exec CopyMem via a6, restores registers, clears a0 and jumps through a6.

### Operation Stealth (demo) (1990)(U. S. Gold)[rolling demo].adf:endcli

- `endcli_cleanup_and_exit`: Finds the largest entry in a linked list, allocates and fills a block with marker values, copies a sixteen-long table, calls library functions through a5, and returns zero or minus one.
- `copy_d1_to_d0_and_branch_4`: Moves the longword in d1 into d0 and branches to another routine, passing d0 as its argument.

### Operation Stealth (demo) (1990)(U. S. Gold)[rolling demo].adf:startram

- `startram_entry_jump`: Program entry point that immediately jumps to the startup routine at a fixed program-counter-relative offset.
- `open_libraries_and_run_demo`: Opens dos, graphics and intuition libraries, reads display and copper structures, sets up bitplanes and copper list, runs the demo, then closes libraries and returns.
- `load_a4_scratch_pointer`: Loads the immediate long value 32766 into address register a4 and returns to the caller.
- `allocate_and_init_manx_structure`: Allocates a block sized from a4-relative word times six, stores it in a4-relative memory, fills fields, creates a MANX tag, opens resources and calls several subroutines before returning.
- `copy_d0_to_d5_4`: Copies the long value in data register d0 into data register d5, then falls through into the following routine.
- `parse_command_line_into_argv`: Allocates a buffer, copies the command-line template, splits it into whitespace-separated tokens handling quoted strings, builds a pointer array and stores the token count in globals.
- `load_7fff_immediate_and_branch`: Moves d0 to itself, loads the immediate value 32767 into d0 and branches forward into the following routine.
- `bounded_string_append`: Finds the end of the destination string, then copies up to the given count of bytes from the source, null-terminating, and returns the destination pointer in d0.
- `bounded_strncpy_pad_zero`: Copies bytes from a1 to a0 until a null byte or d1 count is exhausted, then fills the remaining d1 bytes with zero and returns a0 in d0.
- `open_window_and_load_icon_resources`: Opens a WINDOW via a library call, then fetches icon.library structures and stores icon data pointers and counts into fields of the a2 structure.
- `copy_d0_to_d5_5`: Moves the longword in d0 into d5, used as a small callback that returns the value in d5.
- `call_optional_hook_then_dispatch_arg`: Tests the longword hook pointer at offset -32704 from a4; if nonzero calls it through a0, then pushes the word argument at offset 8 of the frame and calls the following routine.
- `shutdown_audio_and_free_resources`: Restores d4, frees the sample buffer, calls per-voice cleanup, invokes several optional teardown hooks, checks the exec library flags, frees two memory blocks and returns the saved argument in d0.
- `restore_d4_unlink_and_return_2`: Pops d4 from the stack, releases the a5 frame and returns to the caller; it is a small epilogue fragment reached only by fallthrough.
- `free_sound_slot_by_index`: Looks up a six-byte sound slot by index, validates the index and slot pointer, optionally calls the slot's release routine, clears the slot pointer and returns zero, or sets an error flag and returns minus one.
- `call_close_2`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir_2`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_input_2`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_open_2`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output_2`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_un_lock_2`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_library_vector_108_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `jump_through_library_offset_19e`: Loads a library base pointer from the a4-relative global and jumps to the code entry at offset minus 414 from that base.
- `call_library_vector_414_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `jump_through_library_offset_228`: Loads a library base pointer from the a4-relative global and jumps to the code entry at offset minus 552 from that base.
- `call_library_vector_552_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_96_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_384_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.

### Operation Stealth (demo) (1990)(U. S. Gold)[rolling demo].adf:add21k

- `jump_to_startup_code`: Jumps via a program-counter relative displacement to the startup code that follows, transferring execution without returning.
- `open_libraries_and_init_display`: Opens dos, graphics and intuition libraries, walks their structures, sets up display data, calls initialization helpers, and closes the libraries on exit.
- `load_a4_with_absolute_base`: Loads the absolute address seven thousand seven hundred ninety-eight into address register a4 and returns to the caller.
- `allocate_and_init_manx_structure_2`: Allocates a MANX-tagged memory block sized from a table entry, initialises its fields, opens a resource, and stores pointers in the a4 globals before returning.
- `copy_d0_to_d5_6`: Copies the longword in d0 into d5 and falls through into the following routine; it does not return on its own.
- `tokenize_text_into_argument_array`: Allocates a buffer, copies a source string, splits it into whitespace-separated tokens handling quoted strings, and builds a null-terminated pointer array in a4 globals.
- `set_d0_to_32767_and_branch`: Copies d0 to itself, loads the immediate value 32767 into the low word of d0, then branches to another routine.
- `append_string_with_length_limit`: Finds the end of the destination string in a0, then copies bytes from a1 into it up to the word count in d0, null-terminating and returning the destination pointer in d0.
- `copy_string_pad_with_zeros`: Copies bytes from a1 to a0 until a null byte or the word count in d1 is reached, then fills the remaining count with zero bytes and returns a0 in d0.
- `open_window_and_load_icon_objects_2`: Opens a window via a library call, stores the handle in a global, then fetches an object, its child and two icon objects, saving their pointers into fields of the passed structure and freeing the temporaries.
- `copy_d0_to_d5_7`: Copies the longword in d0 into d5 and falls through into the following routine; it is a single move instruction used as a callback address.
- `call_stored_callback_then_forward_word`: If the global function pointer at the a4-relative slot is nonzero, calls it, then pushes the word argument from the stack frame and calls a following routine before returning.
- `shutdown_audio_and_free_resources_2`: Runs each voice cleanup, frees the sample buffer, calls optional hooks, releases four sound buffers, checks the CIA interrupt bit, then frees the remaining sound memory and returns the saved argument in d0.
- `restore_d4_unlink_a5_and_return`: Pops d4 from the stack, releases the a5 stack frame with unlk, and returns to the caller.
- `stop_voice_and_clear_slot`: Indexes the six-byte voice table by the argument, validates the range and slot, stops the sample via the stop routine, clears the slot pointer and returns zero, or sets an error flag and returns minus one.
- `call_close_3`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir_3`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_input_3`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_open_3`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output_3`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_un_lock_3`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_library_vector_108_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `jump_via_a4_offset_table_entry`: Loads a pointer from the table at offset minus 32726 from a4, then jumps to the routine at offset minus 414 within that table.
- `call_library_vector_414_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_198_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_210_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `jump_via_a4_offset_table_entry_alt`: Loads a pointer from the table at offset minus 32726 from a4, then jumps to the routine at offset minus 552 within that table.
- `call_library_vector_552_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_498_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_96_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_384_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.

### Operation Stealth (demo) (1990)(U. S. Gold)[rolling demo].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `data_table_decoded_as_code`: Bytes decode as two ori.b immediates and a jump to a fixed address; this is data, not executable code.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
