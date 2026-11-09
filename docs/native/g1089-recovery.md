# Game 1089 (future wars - time travellers (1989)(u. s. gold)(disk 1 of 2)[cr avenger]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01089). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903728 top-level bytes in 25 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 323 raw-word lines |
| Functions named and described | pass, 538 of 538 named (0 author symbols), 538 of 538 described (76 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 5332 of 5332 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 18348 of 18348 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 746180 |
| code | 62158 |
| data | 86847 |
| padding | 5983 |
| unused | 2560 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Future Wars - Time Travellers (1989)(U. S. Gold)(Disk 1 of 2)[cr Avenger].adf:type | 2 | 117 | 2 |
| Future Wars - Time Travellers (1989)(U. S. Gold)(Disk 1 of 2)[cr Avenger].adf:setpatch | 24 | 833 | 45 |
| Future Wars - Time Travellers (1989)(U. S. Gold)(Disk 1 of 2)[cr Avenger].adf:endcli | 2 | 117 | 2 |
| Future Wars - Time Travellers (1989)(U. S. Gold)(Disk 1 of 2)[cr Avenger].adf:delphine | 3 | 16318 | 444 |
| Future Wars - Time Travellers (1989)(U. S. Gold)(Disk 1 of 2)[cr Avenger].adf:add21k | 3 | 480 | 33 |
| Future Wars - Time Travellers (1989)(U. S. Gold)(Disk 1 of 2)[cr Avenger].adf:AVENGER | 2 | 83 | 3 |
| Future Wars - Time Travellers (1989)(U. S. Gold)(Disk 1 of 2)[cr Avenger].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 381 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 2 |

## Cases handled

- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 58 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Future Wars - Time Travellers (1989)(U. S. Gold)(Disk 1 of 2)[cr Avenger].adf:type

- `load_and_parse_type_file`: Opens a file via the dos library, reads its contents into memory, parses the header and records, and returns zero on success or minus one on failure.
- `return_d1_in_d0_tail_branch`: Copies d1 into d0 and branches into the middle of the caller's code, so the value is returned through the shared exit path.

### Future Wars - Time Travellers (1989)(U. S. Gold)(Disk 1 of 2)[cr Avenger].adf:setpatch

- `setpatch_open_dos_and_parse_args`: Opens dos.library, allocates a task structure, parses the command line into argument pointers, opens the named patch file and runs it, returning a status code.
- `close_task_and_free_segment`: Looks up the current task, closes its segment list and library, frees the memory segment and replies the message, returning the original argument.
- `run_patch_then_optional_second_pass`: Saves d2, calls the patch entry with the argument, and if the global flag is set runs a second patch pass before restoring d2 and returning.
- `setpatch_main_install_patches`: Parses arguments, prints version and status messages, then installs patches for layers, trackdisk, user state, 68881 exceptions and RAD memory, finally closing the patch library.
- `zero_filled_data_area`: A single zero word that decodes as ori.b #0,d0; this address is a zero-filled data area referenced by other code, not an executable routine.
- `call_library_vector_with_stack_pointer`: Saves registers, computes a stack-derived pointer with bit 1 cleared into a1, loads a library base from memory, adds 42 to get a structure, loads a2/a5/a6 from it, then calls a library vector with d0=12 and returns d1.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `patch_exec_library_memory_list`: Forbids the exec library, checks version and memory size, allocates and copies 44 bytes of patch code into exec's memory list, recomputes a checksum into the library, permits exec and returns zero or 20.
- `checksum_rom_words_and_jump_patch`: Sums 23 words starting at offset 34 from a6 into d1, stores the complement at offset 82, then jumps through a5 after adding 30 if a version longword matches.
- `open_disk_resource_and_hook_vectors`: Opens disk.resource, checks its version, disables interrupts, increments the exec nest count, installs two function hooks via SetFunction, then restores the nest count and interrupt state.
- `call_hook_and_copy_exec_vector`: Calls the function pointer stored at offset 180, and if it returns nonzero copies the exec library longword at offset 276 into the global at offset 176.
- `check_exec_base_and_jump_patch`: Loads a longword from pc-relative memory into d1, reads the exec library base from address 4, compares it with the longword at offset 276 of that base, and if equal jumps through another pc-relative pointer.
- `compare_d1_with_ff_and_return`: Sets d0 to 255, compares d1 with d0, branches away if equal, otherwise returns immediately to the caller.
- `disable_intena_and_install_patch_vector`: Writes to INTENA to disable interrupts, checks and stores a magic longword at address zero, saves d7 and a5 data at address 256, then jumps through a pc-relative table offset by 48 or 136.
- `jump_to_offset_from_pc_relative_vector`: Loads a longword from the address four bytes past the program counter into a0, adds 136 to it, then jumps to that computed address.
- `install_alternate_exception_vectors`: Reads the exception vector base from address 20, computes an offset, stores a pointer, then fills 16 longwords starting at address 192 with successive code addresses.
- `zero_filled_data_area_2`: This single zero word decodes as an ori.b instruction but is a zero-filled data area referenced by address, not executable code.
- `branch_to_patch_body`: Executes a single bsr branch to the patch body routine, pushing the return address onto the stack before transferring control.
- `repeat_patch_call_fifteen_times`: Calls the patch body routine fifteen times in sequence via bsr, then executes a nop before falling through to the next code.
- `adjust_return_address_by_pc_offset`: Saves d0, pushes the address of the patch body, subtracts it from the caller return address on the stack, adds sixty, then returns via a pushed longword.
- `setfunction_patch_guard`: Reads the exec library base from address 4, compares its version word at offset 20 with 35, returns if newer, otherwise loads a patch address into d0 and jumps to SetFunction with function number -222.
- `clone_tagged_list_with_alloc`: Allocates a block sized from the word count at offset 14 of the source list, copies the count, then loops copying each pair of longwords, allocating memory for nonzero second entries and freeing on failure, returning the new block or a failure flag.
- `free_copied_list_entries`: Decrements d2, and while it stays non-negative steps a3 back by 8, loads the entry pointer and size into a1 and d0, calls the free routine, then branches back to continue freeing.
- `build_alloc_size_and_set_bit31`: Reads a word from offset 14 of a4, shifts it left 3, adds 16, calls a library function with a1=a4, then loads a long from (a2) and sets bit 31 before branching away.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `rte_trampoline_clear_trace_bit`: Moves a0 into a5, stores d1 at offset 2 of the stack, clears bit 14 of the stacked status word, and returns from the exception with rte.
- `chain_cleanup_three_library_calls`: Saves a2, calls a function on the long at offset 68 of a1, and if nonzero calls two more functions passing that long, returning 1 on success.
- `return_zero_and_pop_a2`: Clears the longword in d0 to zero as the return value, pops a2 from the stack and returns to the caller.
- `scan_for_bsr_opcode_word`: Walks forward through words from the caller-supplied address until a word whose high byte equals the branch-to-subroutine opcode, then returns that address in d0.
- `compute_relative_branch_target`: Reads the branch displacement word at the given address, sign-extends the shifted offset and returns the target address in d0, popping d2 first.
- `resolve_relative_offset_to_pointer`: Reads a signed word offset from two bytes past a0, halves it, sign-extends when bit 14 is set, and returns a0 plus twice that offset plus two in d0.
- `parse_three_offsets_from_string`: Saves d2/a2, reads a string pointer from the stack, advances it by two and calls a parser to fetch three values, storing them into three global longs before restoring registers and returning.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_absolute_zero_with_two_args`: Saves d2 and d3, loads two longword arguments from the stack, pushes a pointer to the caller's stack area, and jumps to absolute address zero with those three arguments.
- `format_string_into_stack_buffer`: Saves a2-a4/a6, reserves 140 bytes on the stack, calls exec RawDoFmt to format into that buffer, then passes the buffer and its length to another routine.
- `store_byte_to_a3_and_advance`: Writes the low byte of d0 to the memory pointed to by a3 and increments a3 by one, then returns to the caller.

### Future Wars - Time Travellers (1989)(U. S. Gold)(Disk 1 of 2)[cr Avenger].adf:endcli

- `setpatch_install_patch_and_restore`: Queries exec for a library base, allocates memory, builds a jump table with immediate opcodes, installs a patch vector, then restores the original vector and returns zero or minus one.
- `move_d1_to_d0_and_branch`: Copies the longword in d1 into d0, then branches to another routine; no loop or call occurs here.

### Future Wars - Time Travellers (1989)(U. S. Gold)(Disk 1 of 2)[cr Avenger].adf:delphine

- `jump_to_startup_code`: Jumps to the program startup code at a fixed absolute address, transferring control without returning.
- `data_table_not_executable`: The bytes decode as branch and immediate instructions but are data, not executable code; the final rts is incidental.
- `set_ciaa_fire_button_bit`: Sets bit 1 of the CIAA port A register, which is the mouse/joystick fire button line, then returns to the caller.
- `read_fire_and_right_buttons`: Clears d0, tests CIAA port A bit 6 and POTGOR bit 10, setting d0 bits 1 and 2 when those buttons are pressed, then returns.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `enable_interrupt_mask_4f`: Writes the value 79 to the custom chip interrupt enable register, enabling the interrupt bits in that mask, then returns.
- `enable_interrupts_via_intena`: Writes the value 32879 to the custom chip interrupt enable register, setting bit 15 to enable the selected interrupt sources, then returns.
- `enable_intena_bit3`: Writes the value 8 to the custom chip interrupt enable register at the fixed hardware address, enabling that interrupt source, then returns to the caller.
- `seeded_pseudo_random_scaled_by_argument`: Reads a longword seed from a4-relative memory, doubles it with xor feedback, stores it back, then divides the low word by the word argument and returns quotient in the high word and remainder in the low word.
- `read_ciaa_keyboard_serial_byte`: Polls CIAA interrupt control register bit 3 until the keyboard serial data is ready, reads the serial data register, inverts and rotates the byte, delays briefly, then clears the CIAA serial control bit and returns the key code in d0.
- `translate_keycode_via_lookup_table`: Calls the keyboard read routine, rejects codes with bit 7 set or zero, then scans a 49-entry table in a4-relative memory comparing words and returns the mapped value from the parallel table, or zero if unmatched.
- `build_display_list_from_screen_offsets`: Copies 180 words from a table into the display list buffer, then writes high and low words of five screen pointers plus eight sprite pointers into the list, storing each address split into two words.
- `init_copper_and_blitter_registers`: Loads the hardware base pointer from a4-relative memory and writes initial values into copper and blitter control words at offsets 142 through 422, then returns.
- `set_blitter_control_and_source_pointer`: Writes a blitter control word, stores the caller's argument as a source pointer at offset 128, clears a word at offset 136, then writes a final control word.
- `return_stored_long_value`: Loads the longword stored at the a4-relative offset minus 28012 into d0 and returns it to the caller as the result.
- `set_palette_entries_from_base_color`: Stores an optional base color into a global, then writes five palette entries at offsets 8000 apart, splitting each into high six bits and low word into the copper list at a2.
- `copy_sixteen_words_to_copper_offset`: Copies sixteen words from the caller's buffer into the copper list at offset 112, placing each source word at every other destination word.
- `fill_copper_list_from_parameter_block`: Writes eleven caller parameters into the copper list structure at fixed offsets, computes a combined value from two words into offset 88, then calls the copper setup routine.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `empty_c_function_2`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `show_error_and_cleanup_stack`: Opens a stack frame, pushes a message pointer and two -1 words, calls a display routine, then calls a cleanup routine and returns.
- `open_file_and_get_handle`: Calls an open routine with a filename and mode, stores the returned handle, and if nonzero registers and returns the handle's field.
- `copy_d0_to_d5`: Copies the low word of d0 into d5 and falls through to the following code.
- `open_dh0_file_and_get_handle`: Opens the DH0: file via a library call, stores the returned handle, and if non-null passes it to a close routine and returns the handle's word field sign-extended.
- `check_and_adjust_file_byte`: Looks up a file by name, compares a word against a routine result, and otherwise adds a byte to offset two of the file structure before rechecking and returning a status.
- `negate_d6_word_data_table`: Negates the low word of d6 and moves d0 to itself; these two words are data reached by address, not executable code.
- `compute_screen_coords_from_tile_index`: Calls a helper, stores its word result through the first argument pointer, then reads the current tile index, scales it by 74, adds table offsets and bias values, and writes two computed words through the second and third argument pointers.
- `allocate_buffers_and_clear_game_arrays`: Allocates many memory blocks via a library call, stores the pointers in globals, initializes selection state, then clears word, byte and long arrays for entities, tiles and tables before returning.
- `empty_c_function_3`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `swap_double_buffers_and_set_display_mode`: Swaps the two screen buffer pointers, calls a blit routine with the new buffer, clears a region, sets the display hardware mode word based on the argument, and resets a pending flag.
- `set_sound_chip_mode_6200`: Calls a subroutine, loads the sound chip base pointer from a4-relative storage and writes the value 25088 to offset 256 of that chip.
- `free_all_sound_slots`: Loops over 255 sound slot records, frees each occupied slot via a call, then clears its pointer and marks two fields as -1.
- `allocate_sound_slot`: Finds a free sound slot, allocates memory for width times height, stores dimensions and derived fields, marks slot used and increments a counter.
- `release_slot_and_clear_entry`: Checks the 26-byte slot record for the given index; if empty returns -1, otherwise frees its resource, clears its fields, sets two words to -1, calls the slot reset routine and decrements a counter.
- `release_slot_range`: Loops from the first index up to the second, calling the slot release routine for each index in turn, then restores d4 and returns.
- `clear_two_global_longwords`: Opens a stack frame, then clears two longword globals at fixed offsets from the a4 base register to zero, releases the frame and returns.
- `free_all_list_nodes`: Walks the linked list at the a4-relative head, saving each next pointer, frees every node with size 20 via the memory free routine, then calls the cleanup routine and returns.
- `insert_node_sorted_by_priority`: Allocates a 20-byte node, scans the list for the first node whose priority word is not less than the given index's priority, links the new node before it, stores index and value words, and fixes the tail pointer.
- `remove_node_by_index_and_value`: Searches the list for a node matching the given index and value words, unlinks it, repairs the tail pointer, frees the node, and returns zero on success or minus one if not found.
- `clear_two_global_list_heads`: Clears two longword globals at offsets minus 27940 and minus 27936 from a4, then releases the a5 frame and returns.
- `walk_and_free_global_list`: Walks the linked list whose head is the global at minus 27940 from a4, calling a free routine with size 20 on each node, then clears the list via routine A.
- `advance_global_list_pointer`: Loads the address of the global list head at minus 27940 from a4 and loops copying the next node pointer into a local until it becomes null.
- `allocate_and_link_sprite_node`: Allocates 20 bytes, fills a new node with the caller's two word arguments and four table-sourced words indexed by the first argument, then links it into a list and returns.
- `reset_global_list_and_flag`: Clears the global list pointer at offset -27920 from a4 and sets the word at -27708 from a4 to minus one, then returns.
- `free_global_list_nodes`: Walks the global list at offset -27920 from a4, freeing each node with tag 214, then calls the reset routine to clear the list head and flag.
- `allocate_and_init_entity_record`: Allocates a 214-byte record via the memory allocator, links it into the global list, copies 50 words from a table indexed by the argument, zeroes two 50-word arrays and two words, then stores a table longword and the argument index.
- `find_and_free_entity_by_index`: Walks the global entity list comparing each record's index word at offset 212 with the argument; if found, sets that word to minus one and returns zero, otherwise returns minus one.
- `purge_freed_entities_from_list`: Walks the global entity list, and for each record whose index word at offset 212 equals minus one, unlinks it, frees it through the deallocator with size 214, and continues from the next node.
- `clear_list_head_and_set_sentinel`: Clears the longword list head at offset -27706 from a4 and writes -1 to the word at offset -27494 from a4, then returns.
- `walk_list_and_free_each_node`: Walks the linked list at offset -27706 from a4, calling a free routine with tag 214 on each node, then calls the list reset routine and returns.
- `allocate_and_init_list_node`: Allocates a 214-byte node, links it at the tail of the list at offset -27706 from a4, zeroes two 50-word arrays and two words, copies a 12-byte table entry indexed by the argument, and calls a setup routine.
- `clear_object_active_flag_by_id`: Walks the linked list at the a4-relative base, compares each node's word at offset 212 with the argument, and if found sets that word to minus one and returns zero, else returns minus one.
- `purge_inactive_objects_from_list`: Iterates the a4-relative linked list, unlinks nodes whose word at offset 212 equals minus one, calls the free routine with tag 214 on each, and returns after the list is cleaned.
- `dispatch_object_render_by_index`: Uses the argument to index tables at a4-relative offsets, reads sprite and object fields, and if the object pointer is non-null calls the draw routine with those fields as arguments.
- `draw_sprite_with_bit_flags_and_index`: Pushes four bit flags from a5-12, several a5 words, a4-relative pointer and a sprite entry indexed by a5-6 times 26, calls the draw routine, then calls another with argument 8(a5).
- `set_viewport_origin_and_scroll_offsets`: Stores four word parameters into a4-relative globals and computes two derived offsets from them, one as row times 160 plus column times 8, the other as 160 minus column times 8.
- `call_render_with_two_global_pointers`: Pushes two longword globals from a4-relative memory onto the stack and calls a routine at a pc-relative address, then discards the arguments and returns.
- `check_screen_origin_and_draw_two_objects`: Reads three words from a4-relative globals; if they equal zero, 160 and 200 it draws two objects with no extra coordinates, otherwise it passes those words as coordinates to the same two draw calls.
- `draw_two_objects_with_stored_coordinates`: Pushes the three saved coordinate words plus a4-relative object pointers twice onto the stack and calls the draw routine for each of the two objects, then releases the stack and returns.
- `empty_c_function_4`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `load_word_argument_sign_extend_and_jump`: Creates an empty a5 frame, loads the word argument at offset 10 into d0, sign-extends it to a long and branches to another routine.
- `store_word_to_indexed_table_entry`: Reads word at offset 8 of a5, sign-extends it, shifts left 5 to form a byte index, then stores the word at offset 12 of a5 into the table at a4 minus 20820 and branches onward.
- `store_word_to_indexed_table_entry_b`: Reads word at offset 8 of a5, sign-extends and shifts left 5, stores the word at offset 12 of a5 into the table at a4 minus 20790, then branches to shared exit code.
- `store_four_words_and_dispatch_by_index`: Stores four words from a5 offsets 10,12,14,16 into consecutive table slots indexed by the word at offset 8, then calls two routines with index zero and returns.
- `read_four_bitplane_pixel_bits`: Computes a bitplane address from x, y and stride arguments, tests the bit in four consecutive words and returns a four-bit pixel value in d0.
- `read_single_bitplane_pixel_bit`: Computes a bitplane address from x, y and stride arguments, tests one bit in the first word and returns zero or one in d0.
- `run_game_main_loop_and_init_state`: Initializes many game state words in the a4 data area, loads text strings, then runs the main loop calling input, update and display routines until the exit flag is set.
- `poll_mouse_and_handle_menu_selection`: Reads mouse button and position into local stack variables, accumulates pressed buttons over up to 100 polls, then opens or selects a menu entry and updates the menu state words in a4.
- `open_menu_and_dispatch_choice`: Checks a flag, saves the screen, polls mouse buttons and position, builds a menu with five or four entries, then jumps through a table to handle the chosen entry.
- `find_matching_menu_entry_index`: Scans up to 255 entries of a 12-byte table in a4, comparing a type word and two coordinate words against the arguments, returning the matching index or minus one.
- `find_active_object_hit_by_point`: Walks the object list at the a4 base, checks each object's type and sprite bounds against the passed x/y point, and returns the matching object's sprite index or minus one.
- `dispatch_object_list_by_type`: Iterates the object list at the a4 base and, depending on each object's type field, calls the draw, sound, or animation routines with the object's stored coordinates and sprite data.
- `draw_type5_objects_with_bitmap`: Walks the object list, and for each type five object allocates a bitmap, computes its screen position from the passed offsets, draws it, then frees the bitmap and returns.
- `draw_sprite_list_with_offsets`: Walks a linked list of sprite nodes, allocates a bitmap for type 5 nodes, looks up per-node position and size tables, and calls the sprite blitter with computed coordinates and flags.
- `free_sprite_list_no_flag`: Iterates the sprite list at the first global list head, saving each next pointer and calling the free routine with a zero flag for every node.
- `free_sprite_list_with_flag`: Iterates the sprite list at the second global list head, saving each next pointer and calling the free routine with a flag of one for every node.
- `check_script_end_flag`: Reads the script pointer and end flag from the object at offset 8(a5); if the flag at 212 is not minus one it returns, otherwise it falls through to the script byte reader.
- `read_script_byte_pair_and_word`: Reads three script bytes via a2 and d4, stores two bytes and a big-endian word in the a5 frame, then calls the draw routine with those three values.
- `read_script_byte_and_object_word`: Reads three script bytes via a2 and d4, uses the third as an index into the object at 8(a5) to fetch a word at offset 104, then calls the draw routine with it.
- `read_five_script_words_call_1f96`: Reads five bytes from the script stream at a2 indexed by d4, stores them at a5 offsets -90 to -98, pushes them as arguments and calls the routine at 1F96, then jumps to the shared dispatcher.
- `read_five_script_words_call_6f1e`: Reads five bytes from the script stream at a2 indexed by d4, stores them at a5 offsets -90 to -98, pushes them as arguments and calls the routine at 6F1E, then jumps to the shared dispatcher.
- `script_opcode_store_field_104`: Reads an opcode and operand bytes from the script stream at a2, then writes a computed word into field 104 of the object table at a5 offset 8, using tables in a4 and calls to 009C and 07E4.
- `add_operand_to_script_field`: Reads two operand bytes from the script stream at a2, then adds either a byte-pair immediate or another object's field at offset 104 into the target object's field at offset 104.
- `subtract_operand_from_script_field`: Reads two operand bytes from the script stream at a2, then subtracts either a byte-pair immediate or another object's field at offset 104 from the target object's field at offset 104.
- `multiply_script_field_by_operand`: Reads two operand bytes from the script stream at a2, then multiplies the target object's field at offset 104 by either a byte-pair immediate or another object's field at offset 104.
- `divide_stat_by_indexed_stat`: Reads two byte indices from the a2 stream, advances d4, then divides the word at offset 104 of the first indexed object by the word at offset 104 of the second, storing the quotient back.
- `divide_stat_by_stream_word`: Reads a big-endian word from the a2 stream into d4, advances d4, then divides the word at offset 104 of the object indexed by the earlier byte by that word, storing the quotient back.
- `divide_two_stats_store_result`: Reads two byte indices from the a2 stream, pushes the words at offset 104 of both indexed objects, calls the divide routine, and stores the returned word at offset 204 of the object at a5+8.
- `parse_bytecode_assign_table_word`: Reads a destination index and a type byte from the byte stream at a2, then stores a word into the table at a4-1986, either from the object field at offset 104, from another table entry, or from a following big-endian immediate.
- `parse_bytecode_assign_object_field`: Reads a destination index and type byte from the byte stream at a2, resolves a source word from the object field at offset 104, from the table at a4-1986, or from an immediate, then calls the compare routine and stores the result at object offset 204.
- `read_byte_call_112a_and_jump`: Reads one byte from the stream at a2 into d1 and stores it at a5-90, pushes zero and that word on the stack, calls the routine at pc-relative 112a, then jumps to the shared exit.
- `read_byte_store_and_call_with_five`: Reads the byte at offset d4 in the a2 table into d1, increments d4, stores the byte at -90(a5), pushes 5 and that value, calls a routine, then adds 4 to a7 and branches away.
- `read_byte_store_to_a4_offset`: Reads the byte at offset d4 in the a2 table into d1, increments d4, stores it at -90(a5) and also at -32222(a4), then branches away.
- `read_byte_store_and_call_with_zero`: Reads the byte at offset d4 in the a2 table into d1, increments d4, stores the byte at -90(a5), pushes zero and that value, calls a routine, then adds 4 to a7 and branches away.
- `read_byte_echo_and_play_sound`: Reads the byte at offset d4 in the a2 table into d1, stores it at -90(a5), calls the console output and sound routines with that byte, then branches away.
- `read_byte_and_call_with_flag_one`: Reads the byte at offset d4 in the a2 table into d1, stores it at -90(a5), pushes the value and constant 1, calls a routine, then branches away.
- `read_byte_store_word_in_record`: Reads the byte at offset d4 in the a2 table into d1, stores it at -90(a5), scales it by two, adds the pointer at 8(a5), and writes d4 into that record's word at offset 4.
- `store_ffff_at_script_index`: Reads a byte from a2 indexed by d4, increments d4, stores the byte in a5-relative scratch, then writes the value 65535 into the word at offset 4 of the table entry pointed to by a5+8 plus twice the byte, and branches away.
- `load_word_from_script_index`: Reads a byte from a2 indexed by d4, increments d4, stores the byte in a5-relative scratch, then loads the word at offset 4 of the table entry pointed to by a5+8 plus twice the byte into d4 and branches away.
- `conditional_script_word_fetch`: Reads a byte from a2 indexed by d4 and increments d4; if flags at offset 205 of the a5+8 structure select it, loads the word at offset 4 of the indexed table entry into d4, then branches away.
- `fetch_byte_skip_if_flag_bit1`: Reads a byte from the a2 script stream at offset d4 into d1 and stores it at -90(a5), then if bit 1 of the object flag byte at 205(a0) is set, reloads d4 from the object table and jumps to the shared dispatcher.
- `fetch_byte_skip_if_flag_bit2`: Reads a byte from the a2 script stream at offset d4 into d1 and stores it at -90(a5), then if bit 2 of the object flag byte at 205(a0) is set and bit 0 clear, reloads d4 from the object table and jumps to the shared dispatcher.
- `fetch_byte_skip_if_flag_bit0`: Reads a byte from the a2 script stream at offset d4 into d1 and stores it at -90(a5), then if bit 0 of the object flag byte at 205(a0) is set, reloads d4 from the object table and jumps to the shared dispatcher.
- `fetch_byte_and_jump_via_table`: Reads a byte from the script stream at a2 indexed by d4, stores it at -90(a5), increments d4, and if bit 0 of the object flag at 205(a0) is clear jumps to the word offset from the object table.
- `decrement_counter_and_branch_on_remaining`: Reads two script bytes into -90(a5) and -92(a5), decrements the word counter at 104 of the indexed object, and if it is still non-negative loads d4 from the second object's word at offset 4.
- `read_two_bytes_call_pair_routine`: Reads two consecutive script bytes into -90(a5) and -92(a5), pushes them as arguments to the routine at 10BE, cleans the stack, then branches to the shared dispatcher.
- `read_byte_advance_d4_call_1422`: Reads the byte at a2 indexed by d4 into d1, stores it at offset -90 from a5, pushes it and calls the routine at pc offset 1422, then adds 2 to a7 and branches away.
- `read_byte_advance_d4_call_1500`: Reads the byte at a2 indexed by d4 into d1, stores it at offset -90 from a5, pushes it and calls the routine at pc offset 1500, then adds 2 to a7 and branches away.
- `copy_string_bytes_to_a5_buffer`: Calls a routine, then copies bytes from a2 indexed by d4 into the buffer at offset -88 from a5 until a zero byte, terminates it, passes the buffer to another routine and branches away.
- `load_animation_file_by_name`: Copies a null-terminated filename from the script stream into the a5 buffer, calls the animation loader, and on failure stores an error flag and formats the LoadAni error message.
- `load_background_by_index_and_name`: Reads a background index and filename from the script stream, builds the name in the a5 buffer, and calls the loader with the indexed background record at a4 offset.
- `load_background_file_by_name`: Copies a null-terminated filename from the script stream into the a5 buffer, calls the background loader, and on failure sets the error flag and formats the LoadBg message.
- `copy_string_until_nul_to_a5_buffer`: Calls a subroutine, then copies bytes from the a2 string indexed by d4 into the a5 buffer at offset minus 88, stopping when a zero byte is stored at minus 90.
- `terminate_buffer_and_load_ct_file`: Null-terminates the a5 buffer at index d5, calls the loader on it, and on failure stores error flags and prints the ECHEC LoadCt message before exiting.
- `call_three_subsystems_and_jump_back`: Calls the routine at absolute address ffc2, then the pc-relative routine at 92d2, then the routine at ffc2 again, and branches to the main loop.
- `call_19fc_then_jump_56e0`: Calls the routine at program counter offset 19fc, then branches unconditionally to the shared code at offset 56e0.
- `set_flag_at_a4_minus_85a_then_jump`: Writes the word value 1 to memory at offset minus 85a from a4, then branches unconditionally to the shared code at offset 56e0.
- `store_d4_into_a0_offset_ce_then_jump`: Loads a pointer from offset 8 of a5 into a0, stores d4 at offset ce of that pointer, sets the word at a5 minus 4 to 1, then branches to offset 56e0.
- `set_flag_word_and_branch`: Writes the value 1 to the word at offset -1456 from a4, then branches to the shared continuation at the end of the routine.
- `clear_flag_word_and_branch`: Clears the word at offset -1456 from a4 to zero, then branches to the shared continuation at the end of the routine.
- `dispatch_script_opcode_by_flag`: Tests the word at offset 12 from a5; if zero it pushes the word at offset 212 of the pointer in 8(a5) and calls one routine, otherwise calls another, then sets the word at -4(a5) to 1 and branches away.
- `set_flag_and_jump_to_56e0`: Writes the immediate value 1 to the word at offset -816 from a4, then branches unconditionally to the code at 56e0.
- `read_five_script_fields_and_call_902a`: Reads five bytes from the a2 buffer using d4 as index, stores them as words at offsets -90 to -98 from a5, pushes them and calls the routine at 902a, then advances d4 and branches to 56e0.
- `init_50_word_table_then_scan_script`: Fills 50 words of the a0 table with -1, then reads opcode bytes from the a2 script buffer, advancing d4 by each opcode's length until a terminator or end is reached.
- `read_two_bytes_and_big_endian_word`: Reads two bytes from the a2 stream at d4 into a5 offsets minus 90 and minus 92, then reads a big-endian word into offset minus 94, advancing d4 by four and branching to the shared dispatcher.
- `read_three_bytes_to_a5_offsets`: Reads three consecutive bytes from the a2 stream at d4 into a5 offsets minus 90, minus 92 and minus 94, advances d4 by three and branches to the shared dispatcher.
- `read_two_bytes_and_little_endian_word`: Reads two bytes from the a2 stream at d4 into a5 offsets minus 90 and minus 92, then reads a little-endian word into offset minus 94, advancing d4 by four and branching to the shared dispatcher.
- `read_five_word_fields_from_stream`: Reads nine bytes from the stream at a2 indexed by d4, storing one byte and four big-endian words into a5 offsets -90 through -98, then branches to the shared dispatcher.
- `read_byte_and_conditional_word_field`: Reads a byte and a flag byte from the a2 stream into a5 offsets -90 and -92, then reads either one byte or a big-endian word into -94 depending on the flag, and branches to the dispatcher.
- `read_single_byte_field_from_stream`: Reads one byte from the a2 stream at index d4 into a5 offset -90, advances d4, and branches to the shared dispatcher.
- `read_byte_store_word_and_table_entry`: Reads the byte at offset d4 in a2 into d1, stores it at -90(a5), then uses it as a word index to write d4 into the table pointed to by 12(a5).
- `read_byte_store_to_stack_slot`: Reads the byte at offset d4 in a2 into d1, increments d4, and stores the byte value at -90(a5) before branching onward.
- `read_two_bytes_store_two_slots`: Reads two consecutive bytes from a2 using d4 as index, incrementing d4 each time, and stores them at -90(a5) and -92(a5) before branching onward.
- `read_byte_to_offset_minus_5a`: Copies the byte at a2 indexed by d4 into d1, increments d4, stores the word in a5 offset minus 90, then branches to the shared dispatcher.
- `read_three_bytes_build_word_at_minus_5c`: Reads three bytes from a2 using d4, stores the first at a5 minus 90 and combines the next two big-endian into a5 minus 92, then branches to the dispatcher.
- `read_two_bytes_to_offset_minus_5a`: Reads two successive bytes from a2 indexed by d4, stores each word at a5 minus 90, then branches to the shared dispatcher.
- `branch_to_shared_5cca`: Branches unconditionally to the shared code at the target address, transferring control without touching any registers or memory.
- `set_flag_minus_four_a5_then_branch`: Writes the value one as a word to the memory four bytes below a5, then branches unconditionally to the shared target code.
- `branch_to_shared_5cca_alt_entry`: Branches unconditionally to the same shared target address as the other entry points, changing no registers or memory itself.
- `parse_bytecode_stream_into_a5_fields`: Reads successive bytes from the stream at a2 indexed by d4, packing big-endian words into a5-relative fields and advancing d4, then branches to the dispatcher or returns via movem/unlk.
- `compare_two_words_to_flag_bits`: Compares the two word arguments at a5+8 and a5+10, setting bit 0 when equal, bit 1 when the first is greater and bit 2 when less, returning the flags in d0.
- `blit_masked_span_into_bitmap`: Clips a horizontal span against the 160-pixel screen, rotates and masks four bitplane words from a0 using the shift count and mask flags, then ORs them into the bitmap at a1.
- `restore_registers_and_return`: Adds 4 to the stack pointer, pops registers d0 through d7 and a0 through a6 from the stack, releases the a5 frame and returns to the caller.
- `blit_rotated_bitmap_region`: Clips a rotated source rectangle against screen bounds, then copies shifted bit rows into four interleaved bitplanes at the destination pointer.
- `restore_all_registers_and_return`: Skips a zero immediate byte, discards four bytes of stack, pops d0-d7 and a0-a6 from the stack, releases the a5 frame and returns to the caller.
- `blit_masked_sprite_to_bitplanes`: Clips a sprite rectangle against the screen, then ANDs rotated inverted source words into four interleaved bitplanes at offsets 8000, 16000 and 24000, advancing source and destination pointers per row.
- `pop_registers_unlink_and_return`: Consumes a zero immediate byte, advances the stack pointer by four, restores d0-d7 and a0-a6 from the stack, then unlinks a5 and returns.
- `call_display_window_setup`: Pushes a fixed set of word and long arguments plus the two caller parameters onto the stack, calls the display setup routine, then discards the 42-byte argument block and returns.
- `clear_four_plane_rectangles`: Calls the rectangle fill routine four times with zeroed arguments and the base pointer from the stack frame, offset by 0, 8000, 16000 and 24000 bytes, then returns.
- `empty_c_function_5`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `fill_four_planes_with_flag_colors`: Gets a base pointer, tests four flag bits to choose fill values, then calls the fill routine four times on the base pointer and its 8000, 16000 and 24000 byte offsets.
- `draw_flagged_plane_rectangles`: Computes rectangle geometry from the arguments, selects four fill values from flag bits, calls the fill routine four times on plane offsets, then calls another routine with the original arguments.
- `draw_text_block_with_base_offset`: Computes character cell coordinates from pixel arguments, builds a blit descriptor using the global bitmap base plus a fixed offset, and calls the blit routine to draw a text block.
- `draw_text_block_with_explicit_base`: Same cell and mask computation as the other text drawer but takes the bitmap base address from its fifth argument instead of the global, then calls the blit routine.
- `render_multiline_text_string`: Looks up a string pointer by index, advances the cursor per character using the font width table, blits each glyph into both bitmaps, and handles spaces, line breaks and end of string.
- `clear_bitplane_pixels_at_offset`: Uses the x and y arguments to locate a bitplane word and row, then clears eight pixels by rotating and inverting each source word and ANDing it into memory.
- `wrap_text_into_lines`: Scans a string byte by byte, tracks line width and word lengths, breaks at spaces or terminators, and writes the resulting column and row positions to the output pointers.
- `walk_linked_list_from_global`: Loads a global list head, follows each next pointer in a loop until null, and keeps the current node in a local variable.
- `append_node_to_list_head`: Allocates a 20-byte node, copies the caller's word arguments into its fields, sets type 2, and links it at the head of the list at a4-27960, initializing the list if empty.
- `append_node_to_list_tail`: Walks the list at a4-27960 to its last node, allocates a 20-byte node, stores the caller's word and type 3, and appends it to the tail.
- `draw_text_string_with_glyphs`: Saves the display, draws a frame, then loops over the string at 8(a5), drawing each glyph from the font table at a4-27980 and advancing the cursor, finally restoring the display.
- `draw_wrapped_text_block`: Renders a wrapped text string into a bordered box on the screen, computing line widths from the font table and blitting each character glyph to both bitplanes.
- `draw_centered_text_window`: Draws a centered text window with a border, wrapping the given string into lines and blitting each character glyph to the screen bitplanes.
- `draw_text_grid_panel`: Draws a bordered panel containing a grid of text rows, blitting each character glyph from the font table to both bitplanes and clipping to screen bounds.
- `draw_text_block_with_shadow`: Renders a multi-line text block at given screen coordinates, clipping to 199 rows, drawing each character glyph from the font table and its shadow copy, then drawing border lines.
- `collect_active_objects_into_list`: Clears twenty object slots, scans 255 object records for active entries with state minus two, copies each into the slot array and stores its index, returning the count.
- `find_object_by_name_and_type`: Builds the active object list, returns minus two if empty, otherwise searches for a matching object using the given name and type and returns its stored index or minus one.
- `check_and_run_palette_fade_loop`: Calls a check routine, stores its result, and if nonzero repeatedly calls a fade routine with two arguments until the result becomes zero, returning minus two on failure.
- `add_deltas_and_clamp_rgb_nibbles`: Adds three delta words to themselves, splits a packed color word into red, green and blue nibbles, adds the deltas with clamping to zero through fifteen, and returns the repacked word.
- `fade_palette_entries_sixteen_times`: Loops sixteen times over sixteen palette words, passing each through the nibble-add routine, writing results back, updating the palette and waiting, then frees two memory blocks.
- `fill_sixteen_entry_row_table`: Loops sixteen times calling a routine with negated row and column values, stores each returned word into a table at offset minus 2102 from a4, then flushes and waits.
- `fill_table_range_from_arguments`: Reads start and end indices from the stack frame, calls a routine for each index and stores the returned word into the table at offset minus 2102 from a4.
- `load_table_word_when_index_is_one`: Checks the stack argument at offset 12 for value one, and if equal loads the word at table offset minus 836 from a4 indexed by the argument at offset 10 into d5, then copies that index to d4.
- `shift_word_array_down_one_slot`: Copies each word in a table at offset minus 2102 from a4 down one slot from index d4 until reaching the count at offset 8 of a5, then stores d5 at that count and branches to the shared tail.
- `load_last_word_and_start_down_shift`: Compares the word at offset 12 of a5 with 2, and if equal loads the word at index from offset 8 of a5 into d5, sets d4 to that index and branches into the downward shift loop.
- `shift_word_array_up_one_slot`: Copies each word in a table at offset minus 2102 from a4 up one slot from index d4 while below the count at offset 10 of a5, then stores d5 at that count and branches to the shared tail.
- `load_word_from_frame_offset_8`: Loads the 16-bit word at offset 8 from the a5 stack frame into d4, then branches forward to another routine.
- `copy_word_array_until_limit`: Copies 16-bit words from the array at a4-2134 to a4-2102 indexed by d4, incrementing d4 until it exceeds the word at a5+10, then calls a routine with the destination array.
- `reset_database_and_open_file`: Clears 255 records of 30 bytes in the database buffer, resets counters, closes any open file, opens the named database file, reads its dimensions and data, or reports failure via formatted messages.
- `clear_entity_table_and_free_buffer`: Loops over 255 entries of 30-byte records in the table pointed to by a4-relative memory, clearing the first byte and four longwords at offsets 14, 18, 22 and 26, then frees a buffer and resets its pointer to minus one.
- `deinterleave_4_planes_to_screen`: Computes a row count from the first argument divided by four times the second argument, then copies that many groups of four words from the source pointer into four separate destination pointers spaced by the row count.
- `find_matching_entity_by_name`: Returns minus one if the a4-relative flag is negative; otherwise uppercases the input string bytes via library calls, then scans up to 255 entity records comparing each against the string and returns the matching index or minus one.
- `set_object_two_sound_params`: Looks up the object record at index from the first argument, then calls the sound routine twice with the record's two word fields and the second argument.
- `copy_bytes_advance_source_pointer`: Copies the given count of bytes from the buffer pointed to by the first argument's pointer into the destination, then stores the advanced source pointer back.
- `load_object_frames_into_slots`: Resolves an object index, allocates a frame buffer, copies a header, then loops over frame slots calling the loader and filling slot tables until done.
- `expand_three_nibbles_to_odd_values`: Takes a word argument, splits its low three nibbles, doubles each plus one modulo sixteen, and returns the recombined word, or zero if the argument is zero.
- `swap_screen_buffers_and_redraw_palette`: Looks up a resource, swaps the two screen buffer pointers, clears and redraws the display, then rewrites sixteen palette entries through the nibble routine.
- `lookup_resource_index_or_return_minus_one`: Calls the resource lookup with the word argument, stores the result in d4, and returns minus one if not found, otherwise that index.
- `swap_double_buffers_and_advance_animation`: Pushes a message and calls helpers, swaps the two screen buffer pointers at offsets -28004 and -28008 in a4, then compares animation frame fields and calls the frame update routines with d4.
- `copy_second_buffer_and_scan_filename_dot`: Calls a helper with the pointer at -40(a5), copies it to -44(a5), then scans the string at 8(a5) for a period, returning the index in d4 and length in d5.
- `check_dot_n_suffix_and_set_buffer_offset`: Tests whether the character after the period in the string at 8(a5) is N, advances -44(a5) by four and sets the buffer pointer at -27996(a4) to the base plus 128, returning one.
- `init_two_ui_rects_and_advance_cursor`: Calls a rectangle setup routine twice with sizes 2 and 32, then advances the pointer at offset -27988(a4) by 34 bytes and stores it at -27996(a4), returning 1 in d0.
- `return_zero_and_jump_to_common_exit`: Sets d0 to zero and branches to the shared exit code, providing a zero return value without touching other registers or memory.
- `read_two_shorts_multiply_and_write_result`: Opens a stack frame, reads a handle from the argument, extracts two 16-bit values, multiplies them, and writes the product through the handle before closing it.
- `lzss_decompress_to_buffer`: Reads a compressed stream from memory pointed to by a0, decompresses it into the buffer at a1 using bit flags and back-references, and returns after the output pointer reaches the buffer start.
- `refill_bit_buffer_with_xor`: Pops a longword from the stack pointed to by a0, XORs it into d5, sets the carry flag, and rotates the value right by one bit into d0 before returning.
- `extract_bits_into_d2`: Shifts d0 right repeatedly, refilling from memory via a0 and XORing with d5 when empty, accumulating the shifted-out bits into d2 for the requested count in d1.
- `clear_50_sound_slots_and_load_scene`: Clears fifty 106-byte sound slot entries in a4-relative memory, stops each active sound, then loads a scene resource and processes its object list.
- `load_scene_and_swap_display_buffers`: Looks up a scene by id, loads its resource, swaps the two display buffers and blits the scene, then walks the object list writing positions.
- `clear_255_sound_slots_and_load_scene`: Clears two hundred fifty-five 12-byte sound slot entries in a4-relative memory, stops each active sound, then loads a scene and processes its object list.
- `release_all_sound_slots_and_play_sequence`: Frees every occupied sound slot by calling the sound release routine, then looks up a sequence index and plays its sound data through the audio driver.
- `flush_four_pending_sound_buffers`: Checks four byte flags in the a4 data area and, for each set flag, stops its sound, copies the buffer and queues it for playback.
- `reset_game_state_and_sound_tables`: Releases all sound slots, clears the object, sprite and tile tables in the a4 data area, then initialises the sound and game state variables.
- `prompt_load_savegame_slot`: Builds a prompt from the current save slot name, asks the player for a filename, opens it, and returns zero if the open fails or one on success.
- `load_selected_savegame_file`: Copies the chosen filename into the save buffer, opens and reads the save file, closes the dialog, and returns one.
- `prompt_restore_backup_slot`: Builds a prompt from the current slot and a caller-supplied name, asks for a backup filename, and returns minus one if the open fails.
- `reset_all_game_object_tables`: Calls seven setup routines, then walks fixed-size object tables in a4 memory, freeing each non-null entry via a release call and clearing its fields, and resets many counters and string buffers.
- `flush_stream_loop_with_c1e2`: Repeatedly calls the c1e2 routine on the stream pointer at a5-10 while the counter at a5-4 is below a5-2, then flushes the stream with a two-byte argument and clears the counter.
- `flush_stream_loop_with_c31a`: Repeatedly calls the c31a routine on the stream pointer at a5-10 while the counter at a5-4 is below a5-2, then flushes the stream with a two-byte argument and clears the counter.
- `advance_frame_counter_until_limit`: Calls a routine with the pointer at offset minus ten of a5, increments the word at minus four of a5, and repeats while it is less than the word at minus two of a5.
- `run_scene_setup_sequence`: Calls a routine with the pointer at minus ten of a5, increments the word at minus four of a5 while below the word at minus two, then calls many setup routines with a4-relative strings and returns zero in d0.
- `zero_filled_data_area_3`: Contains only a zero word that decodes as ori.b with immediate zero to d0; this is zero-filled data referenced by address, not executable code.
- `zero_filled_data_word`: A single zero word that decodes as ori.b #0,d0; it is zero-filled data referenced by address, not executable code.
- `walk_list_to_tail_before_alloc`: Walks the linked list at a4-relative base to its last node, storing the previous node pointer in the frame, then branches into the allocation routine.
- `allocate_and_init_record_from_source`: Allocates a 214-byte record, links it into the list, copies five fields from the source pointer at 8(a5), then stores a table entry indexed by the record's word field.
- `refresh_node_sprite_from_table`: Walks the linked list at the global pointer, and for each node stores the table entry indexed by its word at offset 212 into its field at offset 208.
- `empty_list_walk_return`: Empty routine that only releases the a5 stack frame and returns immediately to its caller.
- `append_new_node_to_list`: Finds the tail of the global linked list, allocates a node, links it in, initializes its fields and sets its table pointer from the word at offset 212.
- `init_object_from_template`: Allocates a 100-byte block, copies the template pointer into it, fills fields at offsets 4, 104, 204, 206 and 212 via a copy routine, then stores a table entry indexed by the word at offset 212 into offset 208.
- `refresh_object_table_links`: Walks the linked list at a global pointer, and for each node stores the table entry indexed by its word at offset 212 into its offset 208 field.
- `empty_frame_return`: Empty routine that releases the a5 stack frame and returns immediately to its caller.
- `walk_list_from_a4_offset_6d38`: Sets up a stack frame, loads the list head at offset minus 27960 from a4, then repeatedly follows the next pointer at offset zero until it is null, leaving the last node in the local slot.
- `allocate_node_and_link_into_list`: Allocates a 20-byte block, fills it via a call with the caller's argument, links it at the head of the list at a4 offset minus 27960, and initialises the list head if empty.
- `walk_list_from_a4_offset_6d24`: Sets up a stack frame, loads the list head at offset minus 27948 from a4, then repeatedly follows the next pointer at offset zero until it is null, leaving the last node in the local slot.
- `alloc_20_byte_node_and_link_list`: Allocates a 20-byte block, copies the caller's pointer into it, links it into a doubly linked list, initializes a default list head if none, and returns.
- `scan_entity_table_and_allocate_records`: Iterates a 26-byte-stride entity table, skips empty or -1 entries, compares string fields, and allocates records via memory calls while updating table slots.
- `swap_screen_buffers_and_draw_entity`: Compares two screen pointers, swaps them, sets up a display window, then computes an entity offset and draws it via graphics calls.
- `init_actor_sequence_and_scale`: Pushes the current actor pointer and d4, calls two helpers to copy the actor pointer and build a sequence, then multiplies two globals into d0 and stores the result at -4(a5) before clearing d4.
- `match_actor_slot_and_append_sequence`: Scales d6 by 26 to index the actor table, compares the slot type with d4, and when equal allocates a sequence entry and appends it via the sequence helper before advancing d6.
- `advance_sequence_offset_and_check_slot`: Adds the stored step size to the sequence offset, increments d4, and scans the actor table for a matching slot, calling the append helper or the sequence-advance helper depending on the slot state.
- `increment_d6_check_limit_and_report`: Adds one to d6, compares it with 255 and branches back if below; otherwise pushes a5-relative pointer, calls a routine, adds four to a7, restores d4-d6, releases a5 and returns.
- `walk_object_list_dispatch_by_flag`: Walks the a4-relative object list, reads each node's fields, looks up table entries indexed by node type times 26, and calls a draw routine with the gathered coordinates when the node flag is clear.
- `dispatch_object_with_flag_bits`: Pushes the low four flag bits of the node's word plus its coordinates, table pointer and screen pointer, calls a routine, then adds 24 to a7 and branches back to the list walk.
- `dispatch_script_entries_by_type`: Walks a linked list of script nodes, reads each node's type and coordinates, scales a table value by half, and calls the script dispatcher with the node data and current context.
- `empty_frame_return_2`: Releases the a5 stack frame and returns immediately; it is an empty routine with no other effect.
- `prompt_insert_disk_and_load`: Builds the insert-disk prompt strings, displays them, waits for a disk change, then loads the requested volume and runs the disk setup routines.
- `poll_input_until_abort`: Repeatedly calls the input poll routine with stack buffers, checks the result word, and returns once a non-zero status or abort flag is seen.
- `setup_and_draw_full_screen`: Calls graphics setup, clears and configures the display, then issues a large blit call using the two screen pointers plus 32000 and finally restores the display.
- `measure_text_line_widths`: Scans the string at a2 counting characters per line separated by the pipe byte, tracks the longest line, and stores the maximum width in a stack word.
- `increment_text_line_counter`: Adds one to the word at offset minus thirty from a5, then branches back into the surrounding routine.
- `clamp_line_counter_to_maximum`: Loads the word at minus thirty from a5, compares it with the word at minus thirty-two, updates the maximum if larger, copies it to d4, and returns via movem and unlk.
- `draw_wrapped_text_block_2`: Computes a clamped vertical position from d4, sets up display and blit calls, then walks the string at 8(a5) skipping spaces and drawing characters until termination.
- `advance_text_pointer_and_parse_line`: Adds d4 to the text pointer at offset 8 of a5, calls the line parser with the pointer and three output words, then divides the line width to compute character and remainder counts.
- `reset_line_counters_and_draw_text`: Sets the character counter to five and clears the remainder, calls the text drawing routine with the line position, then reads the next character from the pointer at offset 8 of a5.
- `handle_space_character_advance`: Compares the character in d5 with a space; if it is a space, adds the two column counters to the x position and clears the remainder counter.
- `draw_character_column_tiles`: Reads a byte from the character table indexed by d5, stores it in a5-relative memory, and calls tile blitting routines to draw character glyphs at computed screen coordinates.
- `prompt_for_text_input_loop`: Clears input state, calls keyboard and text routines, reads a key into d5, and loops until enter or escape sets the done flag at a5-relative memory.
- `validate_and_store_typed_character`: Checks d5 against letter, digit and space ranges, stores accepted characters into the buffer at a5+12 indexed by d4, and draws the glyph via tile routines.
- `erase_and_redraw_text_cursor`: When d5 is 8 and d4 is positive, clears the character at index d4 in the buffer at 12(a5), recomputes the cursor position at -38(a5) from -34(a5) and d4, and calls the draw and cursor routines to repaint it.
- `push_zero_call_d4a8_then_exit`: Pushes a zero word on the stack, calls the routine at d4a8, discards the argument, then jumps to the exit routine at ff72 and branches to the main loop at ccda.
- `apply_room_palette_and_clear_counters`: If the requested room index differs from the stored one, adjusts the two colour offsets, copies sixteen palette entries into the active palette buffer, clears its two counters, calls the refresh routine and stores the new room index.
- `init_sprite_control_block`: Copies two words from globals into a buffer, writes sprite control words, then fills sixteen pairs of words from a table into the buffer and clears three counters.
- `install_level3_interrupt_handler`: Saves all registers, stores a4 in a global, disables interrupts via INTENA, saves the old level 3 vector and installs a new handler, then re-enables interrupts.
- `restore_level3_interrupt_handler`: Saves all registers, disables interrupts via INTENA, restores the saved level 3 vector from a global, re-enables interrupts and clears DMA control bits.
- `zero_filled_data_area_4`: A single zero word that decodes as ori.b #0,d0; this address is a zero-filled data area referenced by pointer, not executable code.
- `vertical_blank_interrupt_handler`: Saves d0 and sr, reads INTREQR, and if the vertical blank bit is set calls the routine pointed to by the nearby longword before restoring registers and jumping to the system vector.
- `read_mouse_and_update_cursor`: Reads JOY0DAT mouse counters, computes deltas against stored positions, clamps them to screen bounds, stores the new position and writes the sprite pointer words.
- `clamp_d6_and_adjust_vertical_offset`: Compares d6 with minus one hundred twenty-eight, adds two hundred fifty-five to d6 if greater, adds d6 to a vertical offset in a4-relative memory, then computes d6 from d4 minus another offset and clamps it against one hundred twenty-eight.
- `clamp_and_write_copper_sprite_words`: Clamps horizontal and vertical sprite coordinates in a4-relative memory against limits derived from two counters, packs them into two words at the sprite pointer, and writes the pointer to the copper list at the custom chip base.
- `blitter_fill_rect_with_line_pattern`: Sets up the blitter to fill a rectangle using a line-drawing pattern: computes source address from x and y, derives octant flags, writes blitter control, modulo, data and size registers, then waits for the blitter to finish.
- `copy_pascal_string_from_table`: Reads a length byte from a table entry indexed by the first argument, copies that many following bytes into the caller's buffer, appends a null terminator and returns.
- `match_name_against_device_list`: Walks a linked list of device entries, formats each entry's name into a stack buffer and compares it with the caller's string, returning one when a match is found.
- `find_trackdisk_unit_flags`: Calls exec FindName for trackdisk.device, indexes the unit list by the argument, and returns the unit's flags with the low bit set when the unit is present.
- `open_file_and_close_on_success`: Opens the file named by the string pointer at offset 10 of the frame using mode 1005, stores the handle, and if opening succeeded closes it again and returns zero, otherwise returns minus one.
- `check_resource_id_equals_one`: Clears two local longs, pushes a data pointer and the word argument at frame offset 8, calls the resource check routine, and returns after comparing its result with one.
- `lookup_word_and_test_flag_bit`: Pushes the word argument at frame offset 8, calls the lookup routine, stores its long result in a local, then tests bit one of a local byte and sets a word local to one if set.
- `poll_disk_swap_and_retry`: Reads a status word from the a5 frame, counts 100 iterations, and on timeout shows the insert-diskette prompt, resets the retry flag and re-enters the wait loop.
- `check_diskette_present`: Builds a stack frame, pushes a message pointer and the word argument at offset 8, calls the disk check routine, and returns whether the result equals one.
- `test_disk_status_low_bits`: Calls the disk status routine with the word at offset 8, stores the result, and sets a flag word when the low two bits are nonzero.
- `read_high_word_of_stream_word`: Shifts the high word of the longword at offset -6 from a5 into the low word, stores it at -2 from a5, and if zero calls the stream reader at offset 8 from a5 to refill the longword.
- `refill_stream_when_high_word_zero`: Extracts the high word of the longword at -6 from a5 into -2 from a5, and when that word is zero calls the reader with the argument at offset 8 from a5 to reload the longword.
- `extract_high_word_and_test_zero`: Masks the longword at -6 from a5 to its high word, shifts it right by sixteen into the low word, stores it at -2 from a5, and branches away if that word is nonzero.
- `open_dos_library_and_load_file`: Saves all registers, opens dos.library, allocates a file handle buffer, opens the named file, reads its contents into memory and sets the loaded flag.
- `examine_file_and_get_size`: Allocates a file-info buffer, locks the named file, examines it to read its size from offset 124, unlocks it, frees the buffer and returns the size in d0.
- `set_flag_d20_and_return`: Saves d1-d7 and a0-a6 on the stack, writes the value 1 to memory location d20, restores the registers and returns to the caller.
- `wait_for_d20_flag_to_reach_64`: Saves registers, sets the word at memory location d20 to 1 if it is zero, then spins until that same word equals 64 before restoring registers and returning.
- `open_ciaa_and_build_sound_table`: Saves registers, clears two flag words, copies bytes from the resource base into globals, builds a 15-entry pointer table, opens ciaa.resource, calls four setup routines and returns whether the open failed.
- `close_ciaa_resource_if_open`: Saves registers, and if the flag word at d24 is set, closes the resource through the library vector, calls the teardown routine, clears the flag and returns zero in d0.
- `free_sound_buffer_if_allocated`: Saves all registers, checks the sound-active flag and buffer pointer, and if both are set calls exec FreeMem on the buffer plus eight bytes, then clears the flag.
- `find_max_byte_in_sound_table`: Scans the sound table at offset 532 for the highest byte value, then returns that maximum plus one multiplied by 1024 in d0.
- `program_cia_timer_from_table`: Writes control value 129 to CIA A control register and copies two bytes from the table at the given address into the CIA A timer high and low registers.
- `clear_sixteen_audio_channel_pointers`: Loads the address of a sixteen-entry pointer table into a0, then for each entry loads the pointed-to longword and clears it to zero, decrementing d7 until done.
- `silence_all_four_audio_channels`: Writes all ones to the audio interrupt control, clears the four audio volume registers and the audio interrupt, DMA and filter registers, muting all channels.
- `reset_audio_channels_to_silent_buffer`: Clears audio interrupt control and the four channel volumes, sets each channel length to one, points all four channel pointers at a silent buffer and enables audio DMA.
- `reset_audio_counter_and_return_zero`: Calls the counter routine, sets d0 to zero as return value and returns to the caller.
- `tick_audio_counter_and_update_volume`: Saves all registers, increments a counter at a fixed memory word, and every sixth tick advances a volume value, calling two subroutines when it reaches sixty-four.
- `call_e136_and_restore_registers`: Calls the routine at e136, then pops d0-d7 and a0-a6 from the stack and returns to the caller.
- `init_four_audio_channels`: Loads four channel structures and the audio hardware base, then calls the per-channel setup routine for each of the four channels, advancing the pointers by their structure sizes.
- `advance_audio_volume_toward_target`: Reads the current volume word, adds it to the accumulator, clamps the result against the target word, and writes the resulting volume to the audio hardware register and back to the structure.
- `ramp_audio_volume_down_to_target`: Adds the current volume to the accumulator, compares it with the target word, and if still above the target branches back to the shared volume update code.
- `dispatch_script_opcode_low_nibble`: Reads the low nibble of byte at offset 2 from a6 and branches to one of six opcode handlers, otherwise returns.
- `set_ciaa_pra_bit1`: Sets bit 1 of the CIAA port A register at the fixed hardware address, then returns to the caller.
- `clear_ciaa_pra_bit1`: Clears bit 1 of the CIAA port A register at the fixed hardware address, then returns to the caller.
- `lookup_tile_index_by_key`: Clears two words at a4, copies the word at a6 and the low nibble of byte 3 at a6 into a4, then scans a word table for the value at a6 and stores the matched table entry offset into a4.
- `store_scaled_table_word_to_a4`: Doubles d2, negates it when d4 is zero, reads the word at a0 indexed by d2 into d0 and stores d0 at offset four from a4.
- `store_d0_word_to_a4_offset_four`: Copies the low word of d0 into the word at offset four from a4 and returns immediately to the caller.
- `store_d4_byte_then_branch_back`: Writes the low byte of d4 to the address in a4 and branches unconditionally back to an earlier instruction in the caller's code.
- `jump_via_table_indexed_by_dd6_minus_one`: Loads the table base at address d26 into a0, reads the word at dd6, subtracts one, scales by four and jumps to the pointer at that table offset.
- `scan_word_table_for_value_from_a6_offset`: Doubles d0, loads the word at offset sixteen of a6 into d1, then walks the word table at e66 comparing entries to d1 until a match or table end.
- `copy_high_nibble_indexed_word`: Clears d0, loads the byte at offset 3 of a6, shifts it right by four bits, then branches to shared code that copies a word from an indexed table into offset 6 of a5.
- `copy_low_nibble_indexed_word`: Clears d0, loads the byte at offset 3 of a6, masks it with fifteen, then branches to shared code that copies a word from an indexed table into offset 6 of a5.
- `copy_a6_word_to_a5_offset_six`: Reads the word at offset sixteen of a6 and stores it at offset six of a5, then returns to the caller.
- `add_high_nibble_to_a6_word`: Clears d0, loads the byte at offset 3 of a6, shifts it right by four bits, and if the result is nonzero adds it to the word at (a6) and stores that word at offset 6 of a5.
- `subtract_low_nibble_from_a6_word`: Clears d0, loads the byte at offset 3 of a6, masks it with fifteen, and if nonzero subtracts it from the word at (a6) and stores that word at offset 6 of a5.
- `update_audio_channels_and_advance_sample`: Copies four channel register blocks to the audio hardware, enables DMA, waits, then advances the sample offset and index, wrapping when the frame count is reached.
- `load_object_record_from_table`: Reads a record pointer from the object list at offset d1, then fills the a6 structure with the record type, table entry fields, position and height from the a3 table, clamping height and storing the result in a5.
- `apply_entity_offset_and_clamp_height`: Adds d3 to the a6 base pointer to set field 10, copies a word from the a3 table into field 14, then adjusts field 18 by the low nibble of byte 2, clamping between zero and 64.
- `clamp_height_and_commit_object`: Adds the byte at offset three of a6 to d3, clamps the sum to 64 and subtracts the global height, stores it in a5, then commits the a6 record fields to a5 and sets the pending flag.
- `dma_audio_channel_swap_delay`: Tests the word at a6, clears the word at a4, copies a6 fields into a6+16 and writes a6+20 to DMACON, then busy-waits 300 iterations before storing a6 fields into a5 and ORing a6+20 into a global.
- `copy_a6_fields_to_a5_and_merge_flags`: Copies the longword at a6+4 to a5, the words at a6+8 and a6 to a5+4 and a5+6, then ORs the word at a6+20 into the global at dd8 and returns.
- `check_index_below_ten_set_defaults`: Opens an a5 frame, saves d4 through d6, compares the word at a5+8 against 10 and zero, and sets d5 to zero and d6 to one when that word is zero.
- `clear_buffer_and_draw_two_spans`: Clears the first word of the buffer at offset 10 of a5 and the word before index 14, then calls the span routine twice with color 63 and color 1.
- `shrink_height_clear_and_draw_span`: Subtracts 10 from the word at offset 8 of a5, clears the buffer start and the word before index 14, clamps offset 20 to 63, then draws one span and restores d4-d6.
- `test_flag_and_set_span_colors`: Tests the word at offset 8 of a5; if zero it sets d5 to 0 and d6 to 1, otherwise leaves them, then branches onward.
- `clear_playfield_edges_and_draw_borders`: Clears the first word of the playfield buffer and the word before the indexed offset, then calls the draw routine twice with border parameters for the top and bottom edges.
- `open_ciaa_resource_and_init_sound`: Checks the init flag, opens the ciaa resource, stores the base pointer, opens the audio device, runs two setup routines and sets the flag, returning success in d0.
- `close_audio_device_and_clear_init_flag`: If the init flag is set, closes the audio device through its library vector, calls the cleanup routine, clears the flag and returns zero in d0.
- `set_ciaa_timer_a_and_port_bits`: Writes control value 129 to CIAA control register A, then copies two bytes from a table into CIAA port B data and port B direction registers before returning.
- `silence_audio_channels_without_dma`: Clears volume for all four audio channels, points each channel's location register at a shared buffer, and zeroes four audio state longs without enabling DMA.
- `silence_audio_channels_enable_dma`: Clears all four audio channel volumes, points their location registers at a shared buffer, enables the audio DMA bits in DMACON, and zeroes four audio state longs.
- `save_all_registers_call_e682`: Pushes d0-d7 and a0-a6 onto the stack, calls the audio channel setup routine, restores all registers, sets d0 to zero and returns.
- `increment_audio_counter_setup_channels`: Compares the word at the audio counter with six, increments it if below, then loops over four audio channels pointing a5 at AUD registers and a6 at channel data, calling the channel routine.
- `reset_audio_channels_and_disable_dma`: Clears two audio counters, loops four times calling the channel reset routine over AUD registers, sets DMACON bit 15, delays 300 iterations, then clears any inactive audio channel pointers.
- `init_object_record_from_stack_args`: Computes a record address from the first stack argument times 22 plus a fixed base, clears its longword at offset 4, then copies stack words and a longword into fields at offsets 0, 8, 10, 12, 14, 16 and 18.
- `decrement_timer_if_nonzero`: Tests the word at offset 14 of the a6 structure; if it is nonzero, subtracts one from it, otherwise leaves it unchanged, then returns.
- `clamp_and_store_counter_value`: Copies the word at offset 12 of a6 to offset 14, adds the word at offset 10 to the word at offset 18, clamps the sum to the range 0 through 63, and stores it at offsets 18 of a6 and 8 of a5.
- `release_linked_list_node`: Tests the longword at offset 4 of the node in a6; if non-null, copies it to a5, copies words from a6 offsets 8, 0 and 18 into a5 offsets 4, 6 and 8, and if the word at offset 16 is zero clears the longword at offset 4 of a6, then returns the word at offset 20 in d0 ored into the longword at a4-3944.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `blit_masked_span_into_bitmap_2`: Sets up a blit descriptor from the arguments, clips width and height, then loops over rows reading words from the source, rotating and masking them with the four bit flags at offset 35 of a6, and writing them into the destination bitmap with the given row and column strides.
- `allocate_and_init_manx_buffer`: Allocates a buffer sized from a table entry, writes the MANX signature, initializes fields, optionally loads a file into it, sets up a structure at offset 92, then calls the library vector at a4-32760 and frees the temporary block before returning.
- `copy_d0_to_d5_2`: Copies the longword in d0 into d5 and then falls through into the following routine.
- `open_and_setup_dos_file`: Opens a DOS file, allocates buffers, reads header fields, stores handles and sizes in a4 globals, and returns zero in d0.
- `build_dos_file_entry_table`: Compares a name against a stored string, walks a linked list to count entries, allocates a table, fills entry pointers and sizes, and stores results in a4 globals.
- `copy_d0_to_d5_3`: Copies the longword in d0 into d5, then falls through into the following routine without returning.
- `copy_string_from_stack_args`: Loads destination and source pointers from the stack, copies bytes from source to destination until a zero byte, and returns the destination pointer in d0.
- `strlen_of_stack_argument`: Loads a string pointer from the stack, scans bytes until a zero terminator, and returns the length excluding the terminator in d0.
- `to_upper_ascii_char`: Reads the byte argument from the stack, and if it lies between 97 and 122 subtracts 32 to convert lowercase ASCII to uppercase, returning the result in d0.
- `to_lower_ascii_char`: Reads the byte argument from the stack, and if it lies between 65 and 90 adds 32 to convert uppercase ASCII to lowercase, returning the result in d0.
- `compare_strings_limited`: Compares two string pointers from the stack byte by byte up to 32767 bytes, returning zero if equal, one if the first is greater, and minus one if less.
- `copy_memory_block_forward_or_backward`: Copies a byte block of length d0 from a0 to a1, choosing forward or backward direction based on address order, and returns immediately if the pointers are equal.
- `clear_bss_open_dos_and_supervisor_init`: Clears a large bss area, saves the stack and exec base, opens dos.library, and calls Supervisor with a startup routine, alerting on failure.
- `load_a4_with_absolute_7ffe`: Loads the absolute address 32766 into address register a4 and returns immediately to the caller.
- `parse_quoted_tokens_into_argv`: Allocates a buffer, copies a template string, then splits the text into whitespace-separated tokens honoring double quotes, storing pointers in a newly allocated array and returning the count in a global.
- `self_move_d0_noop`: Copies d0 onto itself, an empty placeholder instruction that changes no register and falls through to the following code.
- `load_seven_fff_into_d0_and_branch`: Loads the immediate value 32767 into the low word of d0 and branches forward into the next routine without returning here.
- `append_string_with_limit`: Scans the destination string for its terminator, then copies bytes from the source into it, stopping at a null byte or after the given count, and returns the destination pointer in d0.
- `copy_string_pad_with_zeros`: Copies bytes from the source to the destination up to a null byte or the given count, then fills any remaining count bytes with zeros and returns the destination pointer in d0.
- `load_window_icon_images`: Opens the WINDOW library, fetches an icon library object, loads two icon images by number, stores their pointers and counts into the window structure, then closes the library.
- `copy_d0_to_d5_4`: Copies the longword in d0 into d5, then falls through into the following routine.
- `format_string_into_buffer`: Stores the destination pointer in a4-relative global, calls the formatter with the format and argument, writes a null terminator at the result, and returns the formatter's word in d0.
- `append_byte_to_output_buffer`: Loads the output pointer from a4-relative global, increments it, writes the byte argument into the old position, and returns that byte zero-extended in d0.
- `format_string_forwarder`: Builds a stack frame, pushes a pointer to a local, the caller's argument and a format string, calls the formatting routine, then cleans the stack and returns.
- `format_integer_to_buffer`: Converts a signed integer argument into a decimal string written backwards into a caller-supplied buffer, honoring width and a sign flag, and returns the buffer pointer.
- `formatted_output_engine`: Parses a printf-style format string, fetching arguments, converting numbers and strings via the integer formatter, and emitting each character through the output callback until the format ends.
- `signed_multiply_with_sign_fixup`: Saves d1 and d4, negates d0 and d1 to make them positive while tracking signs in d4, calls the multiply routine, then negates the result if the combined sign is negative and returns it in d0.
- `multiply_d1_into_d0_preserving_d1`: Pushes d1, calls the multiply routine, moves the product from d1 into d0, restores d1 from the stack and returns the result in d0.
- `multiply_d1_preserving_d1_return_d0`: Pushes d1, calls the multiply routine, restores d1 from the stack and returns with the product left in d0.
- `unsigned_32bit_divide_d0_by_d1`: Divides the 32-bit value in d0 by d1, using a fast 16-bit divu path when d1's high word is zero and a 16-step shift-subtract loop otherwise, returning quotient in d0 and remainder in d1.
- `push_string_and_word_call_f7ec`: Opens an a5 frame, pushes the address of a data string and the word argument from the stack, calls the routine at f7ec, then cleans the stack and returns.
- `dispatch_word_and_flag_byte_to_f916`: Saves d4, calls a routine with the word and pointer arguments, and if the word equals ten and the pointed byte has bit seven set, calls another routine with minus one and the pointer.
- `branch_to_shared_putc_tail`: Branches unconditionally backwards to the shared tail of the character output routine, transferring control without touching registers or memory.
- `putc_with_buffer_overflow_check`: Saves a2 and a5, loads the buffer pointer from the argument, compares the write pointer against the end pointer, and if full calls the flush routine with the byte from the argument.
- `store_byte_and_advance_pointer`: Loads the buffer write pointer from a2, increments it, writes the byte from the stack argument through that pointer, then branches back to the shared comparison tail.
- `iterate_copper_list_entries`: Walks a table of copper list entries starting at the base in a4, calling a routine for each entry sixteen bytes apart until the end pointer is reached, then restores a2 and returns.
- `check_pointer_return_zero_or_minus_one`: Loads the pointer argument from the stack into a2, sets d0 to zero if it is non-null or minus one if null, restores d4 and a2, and returns.
- `release_object_flags_and_clear_fields`: Tests the flag byte at offset 12 of the a2 object, conditionally calls cleanup routines for its subfields, ORs their results into d4, clears the object's first fields, and returns d4 in d0.
- `install_byte_reader_and_check_flag`: Sets up a5 frame, saves d4/a2, stores the address of a byte-reader routine into a global at offset -1020 from a4, tests bit 4 of the flag word at offset 12 of the structure in a2, and returns -1 in d0 if set.
- `flush_buffer_and_verify_write`: If bit 2 of the structure flag is set, computes the pending byte count, calls the write routine with buffer, count and byte, compares written count, and on mismatch sets bit 4, clears the buffer pointers and returns -1.
- `clear_buffer_when_end_marker`: Checks whether the word at offset 12 of the a5 frame equals -1; if so clears bit 2 of the structure flag, zeroes the buffer start and end pointers, sets d0 to zero and branches back to the shared exit.
- `stream_write_byte_or_flush`: Checks the stream structure at a2; if its buffer pointer is null it calls the flush routine, otherwise writes the byte from offset 13 of a5 into the buffer and returns it in d0.
- `stream_reset_and_write_byte`: Resets the stream buffer pointer and end pointer from the base and length fields at a2, sets the buffer-active flag, then stores the byte from offset 13 of a5 and returns it in d0.
- `find_free_stream_slot`: Scans the stream table starting at a4-relative base in steps of 22 bytes, returning the first entry whose byte at offset 12 is zero, or null if none found before the table end.
- `branch_to_shared_cleanup`: Branches unconditionally to another routine's code, transferring execution without touching registers or memory itself.
- `zero_three_longs_and_return_a2`: Clears three consecutive longwords at a2, a2 plus four and a2 plus eight, then loads a2 into d0 and branches onward.
- `allocate_buffer_or_use_inline_space`: Calls an allocator with size 1024; on success stores the pointer and sets a flag, otherwise marks the object with an inline buffer at offset fourteen and sets error bits.
- `configure_channel_and_test_ready`: Writes 1024 to offset 16 of the a2 structure, sets bit 1 at offset 12, stores the longword at -4(a5) to offset 8, reads byte at offset 13 sign-extended into d0, calls the routine at fb60 with d0, and if it returns zero sets bit 7 at offset 12 before branching back.
- `load_list_head_and_enter_loop`: Sets up an a5 frame, saves a2 and a3, loads the longword at -28158(a4) into a2, then branches forward into the shared list-walking loop.
- `free_list_nodes_and_clear_head`: Walks the linked list in a2, saving the next pointer in a3, calling the free routine with the node and its size plus 8, then clears the longword at -28158(a4) and restores a2-a3 before returning.
- `allocate_node_and_link_list`: Installs a callback pointer in a4-relative memory, allocates a node of the caller's size plus eight bytes, stores a list head and caller pointer in it, links it into the list, and returns the node body or zero.
- `prepend_node_to_list`: Stores the current list head from a4-relative memory into the new node, saves the caller pointer at offset four, updates the list head, and returns the node body address in d0.
- `allocate_node_from_word_arg`: Reads the word argument from the stack frame, passes it as a longword to the node allocator, discards the argument, and returns the allocator's result in d0.
- `init_a2_from_global_and_jump`: Sets up an a5 stack frame, saves a2 and a3, clears a3 to zero, loads a2 from a global offset relative to a4, then branches onward.
- `unlink_node_from_list`: Walks the linked list from a2 comparing each node against the target pointer from the stack frame, unlinking the match via a3, returning zero or minus one in d0.
- `copy_next_pointer_to_node`: Returns a3 in d0, and if a3 is nonzero copies the pointer at a2 into the node at a3, then branches onward.
- `push_a2_and_call_ff96_then_advance`: Stores a2 at offset minus 28670 from a4, loads the longword at offset 4 of a2 into d0, adds 8, pushes d0 and a2, calls the routine at pc-relative ff96, sets d0 to zero, pops 8 bytes and branches back to fb38.
- `validate_index_and_test_entry`: Checks the word argument at offset 8 of a5 against zero and a count at minus 28174 of a4, indexes a 6-byte table at minus 1058 of a4, and returns minus one with status 2 if invalid or empty, else calls the entry and returns whether it succeeded.
- `index_table_and_call_fea4`: Multiplies the word at offset 8 of a5 by 6, loads the table base from minus 1058 of a4, pushes the longword entry, calls the routine at pc-relative fea4, pops 4 bytes, and returns 1 if the result is nonzero, else falls through.
- `return_zero_and_branch_back`: Sets d0 to zero and branches backward to another routine, providing a zero return value for its caller.
- `check_pointer_and_store_word`: Pushes the argument at offset 8 of the frame, calls a test routine, and if it returns zero calls another routine and stores its word result at offset minus 1012 from a4, returning minus one.
- `return_zero_and_branch_short`: Sets d0 to zero and branches backward a short distance into another routine, supplying a zero return value.
- `check_object_slot_and_set_error`: Validates the word index in d4 against the object table at a4 offset, tests the six-byte slot pointer for a nonzero long, and sets error code 2 at a4 offset minus 1012 returning -1 when empty.
- `test_slot_flags_or_set_error_five`: Reads the word at offset 4 of the slot in a2, masks bits 0 and 1, and if either is set stores error code 5 at a4 offset minus 1012 and returns -1.
- `call_slot_action_and_report_error`: Pushes the slot pointer, a long from a5 offset 10 and a word from a5 offset 14, calls the action dispatcher, and on failure calls the error reporter storing its result at a4 offset minus 1012 and returning -1.
- `return_d5_as_result`: Moves the longword in d5 into d0 as the return value, then branches back into the shared epilogue at an earlier address.
- `allocate_thousand_byte_block_check_flag`: Opens a stack frame, calls the allocator for a 4096-byte block, stores the pointer in a local, tests bit 12 of the result and returns it or zero depending on a global flag.
- `call_fc8a_then_return_zero`: Calls the routine at the nearby address, sets d0 to zero as the return value, then branches back into the shared epilogue.
- `open_library_and_call_entry`: Pushes a library name and version, opens the library, calls its entry with argument 1, then restores the stack and returns.
- `call_hook_then_dispatch_word`: If the global hook pointer at a4 offset is nonzero, calls it through a0, then pushes the word argument and calls the dispatcher before returning.
- `shutdown_release_all_resources`: Frees allocated blocks, calls registered cleanup hooks, closes libraries, restores the saved stack pointer from a4 and returns the stored result in d0.
- `restore_d4_unlink_and_return`: Pops d4 from the stack, releases the a5 frame pointer and returns to the caller; it is an empty stub that only restores registers.
- `release_indexed_resource_slot`: Looks up a six-byte resource slot indexed by the word argument, frees its pointer via a helper unless flagged, clears the slot and returns zero or minus one on invalid index.
- `jump_to_resource_free_helper`: Immediately jumps to the resource-free helper at the relative target, forwarding the current registers and stack unchanged.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `jump_via_a4_offset_minus_126`: Loads a pointer from the a4-relative table at offset minus 126 and jumps to the code it points to, passing control onward.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_dup_lock`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `call_info`: Wrapper that calls the dos library function Info and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `jump_via_a4_offset_minus_30`: Loads a pointer from the a4-relative table at offset minus 30 and jumps to the code it points to, passing control onward.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `jump_via_a4_offset_minus_48`: Loads a pointer from the a4-relative table at offset minus 48 and jumps to the code it points to, passing control onward.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `jump_via_a4_offset_minus_1034`: Loads a pointer from the structure at a4 minus 1034, then jumps to the code at offset minus 414 within that structure.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `jump_via_a4_offset_minus_1034_alt`: Loads a pointer from the structure at a4 minus 1034, then jumps to the code at offset minus 198 within that structure.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `jump_via_a4_offset_minus_1034_third`: Loads a pointer from the structure at a4 minus 1034, then jumps to the code at offset minus 132 within that structure.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `jump_through_a4_handler_table`: Jumps indirectly to the handler pointer stored in the table referenced by a4, transferring control without returning to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.

### Future Wars - Time Travellers (1989)(U. S. Gold)(Disk 1 of 2)[cr Avenger].adf:add21k

- `entry_jump_to_main_code`: Program entry point that calls the main code at a fixed long address and never returns to this instruction.
- `open_libraries_and_setup_screen`: Opens graphics and intuition libraries, stores their bases in a4-relative globals, configures the screen structure and closes both libraries before returning.
- `init_task_stack_and_call_setup`: Reads a task pointer from the caller stack, computes its base, clears forty-two longwords, stores the stack pointer and exec base, then calls the setup routine.
- `call_optional_hook_then_forward_arg`: If the global hook pointer at offset minus 32766 of a4 is nonzero, calls it through a6, then pushes the word argument from the frame and calls the next routine.
- `open_libraries_and_parse_command_line`: Opens dos and mathffp libraries, allocates and tokenizes the command line into an argument vector, sets up file handles and calls the main game entry.
- `move_d0_to_d0_noop`: Copies d0 onto itself, an instruction that changes no register or memory and falls through into the following routine.
- `init_audio_and_play_sound`: Calls a setup routine ten times, invokes optional callbacks and frees two pointers, then either allocates two buffers and plays a sound or frees the sound pointer, returning the argument in d0.
- `load_7fff_into_d0_then_branch`: Loads the immediate value 32767 into the low word of d0 and branches forward into the middle of the following routine.
- `bounded_strcpy_after_strlen`: Scans the destination string for its terminating zero, then copies up to the given count of bytes from the source into the destination and writes a final zero byte, returning the destination pointer in d0.
- `copy_string_return_destination`: Copies bytes from the source pointer at stack offset 8 to the destination pointer at stack offset 4 until a zero byte is copied, then returns the destination pointer in d0.
- `string_length_return_in_d0`: Walks the byte string pointed to by the stack argument until a zero byte, then returns in d0 the length excluding the terminator.
- `bounded_string_copy_pad_zero`: Copies bytes from a1 to a0 until a null byte or d1 count is exhausted, then fills the remaining d1 bytes with zeros and returns a0 in d0.
- `free_sound_slot_by_index`: Looks up a 6-byte sound slot at index d4 in the table at a4 offset, frees its buffer via a call unless flagged, clears the slot and returns 0 or -1.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_30_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_108_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `jump_through_a4_vector_table`: Loads a pointer from the table at a4 offset and jumps indirectly through the entry at offset -414, transferring control without returning.
- `call_library_vector_414_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `jump_to_pc_relative_target`: Performs an unconditional jump to the code located at the program-counter-relative offset, transferring control without saving a return address.
- `call_library_vector_552_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_384_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.

### Future Wars - Time Travellers (1989)(U. S. Gold)(Disk 1 of 2)[cr Avenger].adf:AVENGER

- `decompress_payload_to_40000_and_launch`: Reads a packed bitstream from a table, expands it into memory at 40000, flashes the background color, then jumps to the decompressed code.
- `refill_bitstream_word_and_rotate`: Pops a longword from the stack pointer a0, XORs it into d5, sets the extend flag, rotates d0 right through extend and returns the shifted value.
- `shift_d0_through_a0_into_d2`: Shifts d0 right by one bit d1 times, refilling from the longword below a0 and rotating the carry into d2, then returns.

### Future Wars - Time Travellers (1989)(U. S. Gold)(Disk 1 of 2)[cr Avenger].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `data_table_not_code`: These bytes are data decoded as code, not executable instructions; the ori immediates and jump are meaningless table bytes.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_display_and_open_graphics_library`: Saves interrupt and DMA control registers, disables interrupts and DMA, patches the copper list, opens graphics.library, installs its copper pointer, closes the library and returns zero in d0.
- `fill_pointer_table_with_cycling_addresses`: Writes 320 consecutive longwords starting at the table pointed to by a0, cycling d1 from its start value back when it passes the end limit, then returns.
- `copied_byte_block_data_table`: The bytes are data, not executable code: a block of byte values that is copied to the destination buffer at 7f000 by the following routine.
- `copy_byte_block_to_7f000_and_set_vector`: Copies the byte block from the source table to address 7f000, then stores that address into the exception vector at offset 6c and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.
- `call_library_vector_144_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
