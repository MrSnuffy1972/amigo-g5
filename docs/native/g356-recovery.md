# Game 356 (AdvancedDestroyerSimulator v1.0 2151): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00356). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1355080 top-level bytes in 31 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 22 raw-word lines |
| Functions named and described | pass, 559 of 559 named (0 author symbols), 559 of 559 described (208 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2006 of 2006 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 16525 of 16525 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 418838 |
| code | 59394 |
| data | 876649 |
| padding | 199 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 958 | 72 |
| dt | 12 | 15020 | 239 |
| echo | 2 | 117 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 268 | 212 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 77 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 78 | 19 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 7 |

## Cases handled

- AdvancedDestroyerSimulator_v1.0_2151/AdvancedDestroyerSimulator_v1.0_2151.hdf: hard-disk image (AmigaDOS OFS, 1472 blocks): its filesystem parses and all 25 files in it are byte-identical to the extracted tree AdvancedDestroyerSimulator_v1.0_2151_hdf, which is recovered file by file; no block outside the filesystem holds data
- 8 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 193 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 13 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `return_minus_one_constant`: Loads the immediate value minus one into d0 and returns immediately to the caller, providing a constant return value.
- `store_base_and_init_vectors`: Saves the incoming a0 base pointer to a global, sets a5 to it, calls offset routines to install a handler and a data pointer, then jumps through the base table.
- `set_two_vectors_and_jump_base`: Loads the base pointer from memory, adds a large offset, stores it at two offsets from a5, adds another offset and branches before jumping through a5.
- `set_a3_to_1mb_and_jump_a5`: Loads the immediate value an address into address register a3, then jumps to the address held in a5, transferring control without saving a return address.
- `read_and_mask_pc_relative_word`: Loads the longword stored at the program-counter-relative offset into d0, clears bits 4 through 6 with a mask, and returns that value in d0.
- `test_intreqr_and_check_stack_bit5`: Reads the interrupt request register at the hardware address, then tests bit 5 of a byte on the stack, returning without changing any register.
- `acknowledge_interrupt_and_return`: Reads the interrupt request word from the stack and writes it to the custom chip interrupt request register, then advances the stack pointer by two and returns.
- `push_a3_and_set_return_zero`: Allocates eight bytes on the stack, copies the caller return address and a3, loads a3 with a nearby code address, sets d0 to zero and d1 to a2, then returns.
- `set_bit1_in_a1_offset_e`: Pops d1 from the stack, pushes a nearby code address and d1 back, sets bit one of the byte at offset fourteen from a1, and returns.
- `load_custom_chip_base_and_branch`: Loads the custom chip register base address into a0 and branches to another routine that continues setup.
- `store_d0_to_scratch_and_load_ciab_base`: Stores the incoming d0 value into a scratch memory location, loads the CIAB base address into a6, then returns.
- `copy_scratch_to_cop2lch_and_jump`: Copies the stored scratch value into the COP2LCH hardware register and jumps through the address in a0.
- `copy_a0_offset6_byte_to_a6_800`: Reads the byte at offset 6 from the structure pointed to by a0 and stores it at offset 2048 from a6, then returns.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `check_pc_relative_long_for_21000_or_a1000`: Loads the long at the pc-relative location into d1 and returns 4 in d0 if it equals either an address or an address, otherwise returns 1.
- `init_two_minus_two_sentinels`: Writes the value minus two to two consecutive longwords at a3, stores the advanced a3 pointer into offsets 132 of a4 and 50 and 54 of a2, clears the word at offset 386 of a4, then skips fourteen bytes of return address and returns.
- `copy_node_offsets_to_a3_fields`: Reads pointers at offsets 4 and 8 of the structure in a1, and if non-null copies the longword at offset 4 of each pointed node into offsets 50 and 54 of the a3 structure, then pops the return address.
- `return_minus_one_if_d0_nonzero`: Decrements d0, sets the byte to all ones if the result is nonzero, sign-extends it to word and long, returning minus one when d0 was not zero and zero otherwise.
- `compare_rotated_not_d0_against_table`: Inverts and rotates d0, compares it with two bytes near the program counter, and on no match loads the address of a table into a1 and stores the vertical position register byte there.
- `push_ffff_and_branch_to_330`: Pushes the immediate value -1 (word) onto the stack, then branches unconditionally to the code at offset 330.
- `restore_registers_and_disable_interrupts`: Tests d0, and if nonzero unwinds saved registers from the stack, writes the stacked word to INTENA to disable interrupts, then rearranges stack words and returns.
- `compute_scaled_time_into_a2_offset`: Reads three longs from the structure pointed to by the pc-relative pointer, multiplies the first by 1440, adds the second, scales by 60, adds the third divided by 50, and stores the result at offset 198 of a2.
- `select_disk_menu_action`: Reads the disk index from offset 67 of a3, clears the status byte at offset 31 of a1, and depending on flag bits and the mode byte at offset 29 of a1 either calls a disk routine or sets status codes and invokes a jump-table entry.
- `disk_flag_data_table`: The bytes decode as data, not executable code: they negate a word, copy d0 to d7, set bits and toggle a flag byte, forming a table referenced by the disk menu routine.
- `disk_flag_test_data_table`: The bytes are data rather than code: they compute a flag test against a bit mask and store the result, forming a table entry referenced by the disk menu routine.
- `push_pc_relative_pointer_offset_29e30`: Pushes the longword stored at the program counter relative location onto the stack, adds the immediate value an address to it, and returns.
- `push_pc_relative_pointer_offset_296e8`: Pushes the longword stored at the program counter relative location onto the stack, adds the immediate value an address to it, and returns.
- `clear_bit_and_toggle_flag_counter`: Reads a bit index from offset 67 of a3, clears that bit in a shared byte, and if it was set toggles bit 1 of offset 64 and increments the counter at offset 294.
- `load_d1_with_65537_and_branch`: Loads the immediate value 65537 into data register d1 and branches unconditionally to another routine.
- `read_byte_at_a0_offset_d2`: Tests d2 for a negative value, and if non-negative reads the byte at a0 plus d2 into d3, copies d3 to d1 and returns.
- `load_d1_with_280_and_branch`: Loads the immediate value 280 into data register d1 and branches unconditionally to another routine.
- `clear_a1_offset_c_and_store_a4_global`: Clears the longword at offset 12 of the structure pointed to by a1, loads 12 into d4, then stores a4 into the global at the pc-relative address and returns.
- `zero_filled_data_area`: This single zero word decodes as ori.b #0,d0 but is a zero-filled data area referenced by address, not executable code.
- `compare_a4_to_global_then_dispatch`: Compares a4 with the stored global; if equal it branches to the shared tail, otherwise jumps through the pointer at offset minus 12 of a5.
- `store_pc_relative_pointers_in_a3`: Loads two program-counter-relative addresses into a3 and writes the first into memory at offset minus 314 from a3, then returns.
- `open_whdload_device_and_start_slave`: Allocates memory, opens expansion.library and whdload.device, sets up a slave task structure with entry point and stack, enqueues it, and returns.
- `open_dos_library_and_jump_entry`: Calls dos.library open at offset minus 96 with a pc-relative name, then jumps through offset 22 of the returned structure.
- `open_dos_and_wait_for_message_port`: Opens dos.library, builds a message port on the stack, finds the current task, installs the port, then waits for and dispatches incoming messages by comparing a word against a jump table.
- `read_two_values_and_compare_third`: Calls three helper routines, storing results in d0 and d1, loads a value from offset 28 of a4 into d2, calls another helper, shifts d0 right by two and branches based on whether it is zero.
- `call_two_helpers_then_return_minus_one`: Calls two helper routines, sets d0 to minus one, and branches to a common exit point without any loop.
- `copy_stream_bytes_to_buffer`: Copies up to the requested byte count from the stream position into the caller's buffer, advancing the stream offset and handling a wrap-around ring buffer, then returns the copied count.
- `append_bytes_to_stream_buffer`: Appends the caller's bytes to the stream buffer, growing the stream end offset, allocating a ring buffer when needed and flushing wrapped data through the write callback.
- `flush_remaining_bytes_and_restart`: Loads the remaining byte count and buffer pointer into d0, d1, a0 and a1, calls the write callback, then returns the requested length and jumps back to the main loop.
- `set_d2_minus_two_and_branch`: Loads the immediate value minus two into data register d2 and branches unconditionally to the following routine, sharing its code.
- `open_file_and_store_handle`: Calls setup routines with d7, tests the returned longword, and on success stores the handle at offset 36 of the structure pointed to by a0, returning minus one in d0.
- `open_file_via_dos_and_chain`: Calls setup routines with d7, opens a file through the a2 jump table and dos library, then branches into the shared file-handle storing code.
- `release_object_and_clear_d0`: Loads the object pointer from offset 20 of a4, checks its field at offset 256, and if nonzero calls the object's release routine through a2 with arguments from the object, then clears d0 and branches away.
- `accumulate_velocity_into_d2`: Loads the object pointer from offset 20 of a4, copies d2 and d1 from a4 offsets 24 and 28, and if d1 is positive adds the object's offset 148 value to d2 before branching.
- `clamp_position_to_limit`: Adds the object's offset 20 value to d2, compares d2 with the object's offset 148 limit, and if not above it stores d2 into offset 20, clears d1 and branches away.
- `set_d0_ff_and_d1_e4_branch`: Loads 255 into d0 and 36 into d1, complements the low byte of d1 to 219, then branches to another routine.
- `load_a4_offset_20_shift_left_two`: Reads the longword at offset 20 from a4 into d7, shifts d7 left by two bits, and returns to the caller.
- `load_a4_offset_24_shift_left_two`: Reads the longword at offset 24 from a4 into d7, shifts d7 left by two bits, and returns to the caller.
- `scale_a4_offset_28_by_four`: Loads the longword at offset 28 from a4 into d7, shifts it left by two bits, and returns the scaled value.
- `shift_string_left_and_store_length`: Copies bytes from a0 forward one position until a zero byte, then writes the byte count into the slot before the string start.
- `open_file_and_build_io_request`: Calls a setup routine, opens a file with mode 256, fills an IO block via a2 offset 120, stores d2, a5 and a3 into the block, and returns it in d0.
- `close_file_and_free_buffer`: If d0 holds a file handle, saves its buffer pointer and length, closes the file via the dos library, frees the buffer, then closes the handle again and returns.
- `build_path_and_trim_slashes`: Allocates memory, concatenates a directory name from d0 with a path from d1 using a slash separator, then collapses duplicate slashes and strips a trailing slash, returning the buffer or an error code.
- `check_whdboot_files_present`: Calls a comparison routine twice to test for the WHDBoot.exe and WHDBoot.ss files, returning zero if both are found or the failing result otherwise.
- `compare_null_terminated_strings`: Saves a0 on the stack, compares bytes from a0 and a1 until a mismatch or a zero byte, then restores a0 and returns.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `push_dos_library_base_plus_offset`: Pushes the value stored at the program counter relative longword plus thirty-two onto the stack and returns, yielding a pointer used by callers.
- `open_dos_library_and_run_main`: Calls a library open vector, opens dos.library via exec, runs a setup routine, invokes a subroutine with arguments, then jumps through a library close vector.
- `install_trap_vectors_and_show_titles`: Patches three absolute trap vector slots with jump instructions to local handlers, then calls a setup routine with d2 zero, displays the first message via a2 offset 256, calls setup with d2 ten, and jumps to display the second message.
- `wait_for_beam_position_and_button_release`: Calls a library routine with a0 string and a2 base, compares d0 to a fixed value, then polls VPOSR beam position and VPOSR/VPOSR bit zero until the beam leaves a range and the button is released.
- `branch_to_1228_entry`: Unconditionally branches to the code at address 1228, transferring execution there without touching registers or memory.
- `call_1228_with_d2_as_argument`: Saves d0 on the stack, moves d2 into d0 as the argument, calls the routine at 1228, restores d0 and returns.
- `call_1228_with_d6_as_argument`: Saves d0 on the stack, moves d6 into d0 as the argument, calls the routine at 1228, restores d0 and returns.
- `wait_for_disk_ready_and_fire_release`: Polls the custom chip register until bit six clears, checks the CIA port A fire button, rechecks the custom register bit, and returns once the disk or button condition is satisfied.
- `skip_d2_pointer_chain_and_advance`: Loads d7 into a1, decrements d2 and, while non-negative, follows the pointer at (a1) and doubles it twice, then adds four to a1 and returns.
- `allocate_and_call_audio_channel_callback`: Saves all registers, calls the audio allocation library function with a0, invokes the a5 callback and a helper, then calls the free function with the returned handle and restores registers.
- `count_vhposr_changes_into_global`: Loops ten times reading the vertical position register and counting changes in d1, then stores the count through the address loaded from the following code word and returns.
- `divide_d0_by_40_keep_high_word`: Swaps d0 to move its high word into the low word, clears the low word, divides by 40, then swaps back and clears the low word before branching away.

### dt

- `open_dos_library_and_setup_game`: Saves registers, clears a table of 154 longs, opens dos.library, reads the current directory and command line, opens the data file, allocates memory and calls two setup routines before returning a status code.
- `close_library_and_reply_exit_message`: Calls an optional cleanup pointer, closes the dos.library base, frees the allocated memory, replies the startup message, restores the saved stack frame and registers, and returns.
- `jump_to_de36_entry`: Performs an unconditional jump to the code at address de36, transferring execution without saving a return address.
- `jump_to_de34_entry`: Performs an unconditional jump to the code at address de34, transferring execution without saving a return address.
- `jump_to_e150_entry`: Performs an unconditional jump to the code at address e150, transferring execution without saving a return address.
- `jump_to_input_poll_vector`: Jumps directly to the absolute address e1e8, transferring control to the input polling routine without saving any registers.
- `main_game_loop_and_setup`: Copies 97 bytes into a buffer, initializes display and sound, then runs the main loop polling input, updating counters, timers and ship state until quit.
- `poll_input_and_count_fire_buttons`: Reads input via the poll routine into d0, stores it, and increments the primary or secondary fire counters when the corresponding bit is newly set.
- `increment_three_byte_counters`: Increments a byte counter, and when it passes fifty-nine resets it and increments a second counter, which past twenty-three resets and increments a third counter; restores d2 and returns.
- `dispatch_sonar_mode_actions`: Stores a flag and a computed difference, then branches on a mode word to call sonar subroutines, test table entries, and write scaled screen coordinates before restoring registers.
- `handle_keyboard_input_and_menu`: Reads input bits and key codes to switch sonar modes, adjust depth and speed values, move a menu cursor, and call the related subroutines before returning.
- `random_heading_change_timer`: Increments a frame counter, and once it exceeds 3000 with the two enable flags set, calls the random routine to pick a new heading value and trigger a sound, then clears the counter.
- `compute_angle_screen_offset`: Takes two signed arguments, shifts the second left by eight, computes an angle via the math routine, clamps it, and maps the quadrant to a screen offset between 320 and 1280.
- `draw_string_advance_seven`: Walks a null-terminated byte string at a3, calling the draw routine for each character with x advanced by seven pixels, using d7 and d6 as coordinates.
- `draw_ship_name_letters`: Copies 48 bytes of a character table to a local buffer, then for each character in the passed string calls the glyph drawing routine at x offset stepping by 7 and y 194, substituting the first table byte when the global flag is set or characters match.
- `format_number_into_buffer`: Validates a digit count between 1 and 8 against a table limit, writes a sign or space, then repeatedly divides the value by 10 storing digits into a global buffer and calls the text output routine with the given coordinates.
- `call_text_output_with_defaults`: Pushes the constants 200 and 319 plus three zero longwords onto the stack and calls the text output routine, then discards the arguments and returns.
- `load_picture_pages_into_bitplanes`: Copies 97 bytes of a picture name buffer, prints labels, then blits four 4000-byte picture strips into successive bitplane rows of the screen buffer and copies the name buffer again.
- `switch_picture_page_and_reset_state`: Reads a page number argument, clears counters, blits picture strips between screen buffers, resets torpedo and timer state, and clears 24 status entries.
- `blit_four_strips_from_picture_pointer`: Takes a picture pointer from the stack and blits four 4000-byte strips into four bitplane rows of the screen buffer, then calls the screen update routine.
- `reset_game_state_and_load_scenario`: Clears many game state words, sets initial counters and limits, copies scenario entries into the active table based on the scenario index, then calls setup and sound routines.
- `animate_radar_sweep_and_blips`: Advances a frame counter, draws the rotating radar sweep line and plots each tracked contact as a small rectangle, plus periodic sweep and range-ring blits.
- `advance_heading_pointer_and_scale`: Saves the old heading, steps the heading index modulo twenty-six or counts down a turn, reads the new heading from a table and stores heading times forty as the scaled value.
- `draw_hud_gauges_and_ship_list`: Calls the display setup, then loops over thirteen six-byte ship entries drawing each with a flag, and draws digit pairs for speed, heading and depth gauges at fixed screen coordinates.
- `main_game_loop_update_and_input`: Runs the main simulation loop: sets up the view, draws the horizon and gauges, polls input bits, handles pause, speed-up and menu keys, and updates game state until exit.
- `end_speed_up_and_reset_state`: Optionally shows the end-of-speed-up message, restores real-time piloting, resets the speed counter, clears several game state words and sets the mode flag to one.
- `init_ship_status_display_fields`: Pushes zeroed parameters and several fixed values, calls the display setup routine, then writes four status values from memory into display slots and returns.
- `wait_for_nonzero_status_flag`: Repeatedly tests the longword status flag at the fixed memory location, calling the read routine with argument 255 until the flag becomes nonzero, then returns.
- `set_screen_window_and_refresh`: Saves d5-d7, stores three word arguments into the display parameter words, sets a byte flag to 128, then calls two routines with the same pointer and returns.
- `draw_ship_sprite_pair`: Saves registers, reads four word arguments from the stack, calls a rectangle fill for the first sprite, writes position and control words to the sprite structure, then draws and frees a second sprite before restoring registers.
- `draw_wide_ship_sprite`: Sets up a stack frame, reads three word arguments, computes a combined width, calls a rectangle fill with computed bounds, writes sprite position and control words, then draws and frees the sprite and returns.
- `update_entity_sprite_table`: Loops over twenty-four 14-byte entity records, checks visibility and type, dispatches to sprite draw routines, handles text drawing for type 255, and updates each record's previous position.
- `draw_prompt_and_swap_screen_buffers`: Sets up display via graphics calls, prints three text lines at given coordinates, conditionally draws a key-press prompt, swaps the two screen buffer pointers, then restores the original pointer and returns.
- `draw_two_line_message_and_refresh`: Initializes display, draws two text strings passed as arguments at fixed coordinates, calls the screen refresh routine, then waits for a key and returns.
- `select_sprite_from_index_and_shift_queue`: Looks up a pointer from the index table, stores it as the current object, calls the draw routine, sets a delay and shifts the queue entries down when the counter expires.
- `store_value_in_first_free_slot`: Saves d6-d7, scans nine word slots for a zero entry, writes the incoming word argument there, and sets a status word to 2 if another flag is zero; returns that flag.
- `step_through_sprite_table_and_draw`: Increments a counter modulo 30, walks a table of word entries until a terminator value, pushing each entry's fields as arguments to a drawing routine, then restores registers and returns.
- `call_routine_repeatedly_count_times`: Saves d6-d7, loops from zero up to the word argument on the stack, calling a fixed routine each iteration, then restores registers and returns.
- `dispatch_a3_with_two_globals`: Saves a3, loads the first stack argument into a3, calls the routine at 5506, then pushes the global at 9122 and a3 and calls two more routines before restoring a3 and returning.
- `call_2908_with_global_239e`: Saves a3, loads the first stack argument into a3, pushes the global at 9118 and a3, calls the routine at 10504, then restores a3 and returns.
- `call_2908_with_arg_and_global`: Builds an a5 frame, saves d7 and a3, loads the first argument into a3 and the second into d7, pushes d7 and a3, calls the routine at 10504, then restores registers and returns.
- `init_zeroed_sprite_pair_and_bounds`: Sets d0 to zero, writes zero to the two words at 540 and 542, then writes 319 and 199 to the next two words before returning.
- `set_sprite_pair_seven_with_bounds`: Clears the word at 540, writes 7 to 542, then writes 319 and 93 to the following two words and returns.
- `set_sprite_pair_three_with_bounds`: Writes 32 to 540, 3 to 542, then 287 and 93 to the next two words and returns to the caller.
- `set_copper_list_a`: Writes four immediate words to the custom chip registers at offsets 28, 30, 32 and 34, then returns to the caller.
- `set_copper_list_b`: Writes four different immediate words to the same four custom chip registers at offsets 28, 30, 32 and 34, then returns.
- `show_read_error_retry_screen`: Clears and draws the read error retry screen, prints the message, then polls a key read until the returned value equals 8192 before returning.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `show_disk_error_message`: Calls a setup routine, pushes the disk error string and constants, calls the message display routine, then cleans the stack and returns.
- `spawn_sonar_contact_blip`: Computes a relative bearing from two coordinates, clamps it, converts to screen position, and if within range stores a new contact entry in the contact table and sets the alert flag.
- `update_contact_tracks`: Loops over up to forty contact slots, either advancing each track's position and calling the draw routine, or respawning it at a random offset around the player position.
- `update_radar_blips_and_contacts`: Walks the object table at the global list, computes each object's offset from the player position, marks nearby contacts, plays sound effects and dispatches per-object update calls, then clears the radar blip counter.
- `scale_two_vectors_by_shifted_sine`: Calls the sine routine with the first word argument, multiplies both pointed-to long values by the returned sine, then shifts each result right by fourteen bits and stores them back.
- `draw_radar_sweep_echoes`: Computes the sweep angle from the player heading, converts it to screen coordinates, looks up a sine table scaled by the range, and writes up to three radar echo sprite entries into the display list.
- `update_depth_charge_target_and_fire`: Computes a projected screen position from the object's coordinates and camera offset, calls the sprite draw routine, and for object type 16 clamps a depth value into a global minimum, then triggers an effect if the result is below a threshold.
- `render_object_sprite_list_with_state`: Walks the object's sprite list, offsets each entry by camera and object position, draws each sprite, updates the object's animation and state fields, and randomly resets a counter when it reaches 200.
- `draw_object_sprites_with_vertical_offset`: Walks the object's sprite list, computes screen positions with a vertical offset from a table, clamps the offset at minus fifty, and draws each sprite using the passed coordinates.
- `spawn_objects_from_ship_slot_table`: Reads a ship's slot table, computes each object's screen position from the ship coordinates and scroll offsets, and calls the draw routine for every non-empty slot.
- `draw_sonar_target_and_update_range`: Sets a mode flag, draws a sonar blip at a fixed vertical offset, converts the returned distance to a range value and stores it if smaller than the current minimum.
- `draw_sprite_at_scrolled_position`: Computes a vertical screen position from the scroll offset plus an extra offset, then calls the draw routine with the given coordinates and sprite numbers.
- `spawn_sonar_contact_marker`: Computes a relative bearing and range from the passed coordinates, looks up a bearing table scaled by a global factor, and if within the detection window writes a new 18-byte contact entry into the contact array, incrementing the contact counters.
- `check_contacts_for_collisions`: Scans the six contact slots against the player position, plays a collision sound and marks the slot when a contact overlaps, then processes the second contact table with per-slot offsets and range checks.
- `reset_or_abort_sonar_ping`: When the argument is one, copies the saved sonar origin into the active position, clears the ping counters and heading, and sets the mode flag; otherwise sets the mode flag to two.
- `show_end_of_round_score_messages`: Reads the round result code, draws the ship and generator graphics, prints the matching score and damage messages, then clears the result and message counters.
- `draw_radar_sweep_blips`: Reads the player position, computes five range cells from the map data, masks the bits and calls the blip plotter for each set direction bit.
- `find_and_clear_highest_score_entry`: Scans the score table for the entry with the largest value, calls the score routine for it, then marks that entry as cleared and repeats until none remain.
- `find_nearest_waypoint_index`: Scans a null-terminated array of coordinate pairs, computes scaled absolute distances from two global reference points, and returns the index of the closest point, choosing the minimum or maximum depending on the mode argument.
- `update_entity_positions_and_timers`: Advances global frame counters and timers, then walks the entity list updating each entity's position, waypoint index and heading based on its state and proximity to reference coordinates.
- `spawn_random_pickup_objects`: Seeds a random draw, picks a random object type, and when its slot is free sets global pickup coordinates and calls drawing and sound routines before marking the slot used.
- `decrement_three_timers_and_trigger_sounds`: Decrements three countdown words in memory, and when each reaches zero clears a related flag and queues a sound effect; also plays a positional sound using a computed offset into a table.
- `select_sound_id_from_active_flags`: Checks groups of three flag words in memory and, when all three are nonzero, writes a corresponding sound identifier into a shared word; returns without changing anything if no group matches.
- `advance_heading_and_update_ship_motion`: Increments a modulo counter, computes sine and cosine offsets to update position accumulators, decays a speed value, toggles a turn counter, and periodically queues a sound and refreshes a random delay word.
- `spawn_depth_charge_slot`: Scans six 28-byte slots for a free one, fills its position, velocity and depth fields, calls the trajectory setup, then arms the current slot and plays a sound.
- `update_launched_depth_charge`: Advances a launched depth charge's animation counter, moves its world position by the stored velocity, calls the sprite draw and collision routines, and resets the slot when it expires.
- `update_depth_charge_slots`: Decrements the four depth-charge counters, arms the current slot when free, then iterates six slots updating falling charges, drawing sprites and triggering explosions.
- `spawn_sonar_contact_marker_2`: Scans six 22-byte contact records for a free slot, then fills it with position, velocity and a 250-frame lifetime and queues a sound effect.
- `update_sonar_contact_lifetimes`: Advances two contact timers, clears them past 90 frames or 30 without a signal, and moves or expires each of six active contact records.
- `select_and_launch_torpedo`: Scans six 28-byte entries for a type-2 slot, compares target tables, writes a 20-byte torpedo record and triggers the launch sound.
- `append_ship_record_to_active_list`: Copies seven words from a source record indexed by the argument into the next slot of the active list, stores a pointer, clears a flag word, then increments both list counters and returns.
- `setup_ally_base_resupply_state`: When the passed object's flag word is zero and a global counter is below two, sets position and timer globals, clears thirteen six-byte entries, resets a flag, calls two routines and prints base text; otherwise sets a stop state.
- `build_contact_list_from_tables`: Uses the two stack arguments to index position and contact tables, fills the display coordinate globals, then copies contact records into the output buffer until a zero or ninety-nine terminator.
- `mission_result_message_dispatch`: Reads a mission state word, jumps through a table of message cases, and calls the text display routine with error or excuse strings when counters match.
- `mission_menu_input_loop`: Sets up the mission menu screen, polls the input device each frame, updates the selection index, and draws the highlighted menu entry until a choice is made.
- `reset_menu_selection_flags`: Writes zero to the first word and ones to four following words in low memory, then zeros two more words, initializing menu selection flags.
- `run_menu_sequence_with_sound_and_pictures`: Sets a flag, clears a word, calls many subroutines to show menu.bin and PRESBLACK pictures, play tout.bin sounds, then clears the flag and returns.
- `adjust_column_with_wrap_and_sound`: Reads two word arguments, checks bits in a status longword, plays a sound effect, calls a coordinate routine, and wraps the column value when it goes below zero.
- `compare_word_arg_to_one_and_zero_d7`: Saves d7 and a2, loads a word argument into d7, compares it with one, and sets d7 to zero if equal before branching onward.
- `call_graphics_op_with_screen_offsets`: Saves d0 in d7, loads a base pointer from a global, passes two offsets into that base plus fixed parameters to a graphics routine, then restores d7 and returns d0.
- `wait_for_input_then_set_flag`: Loads a picture, waits for a key press, polls a status longword until nonzero, then loops up to 500 times calling a routine and sets a flag word to one.
- `draw_ship_hud_and_wait_loop`: Saves registers, sets up display, draws a box and five markers around scaled coordinates, calls a drawing routine twice, then loops until a status longword equals one.
- `draw_sonar_contact_list_and_prompt`: Scans the contact table for entries matching the player marker, draws sonar blips and range rings via graphics calls, then polls the keyboard and advances the selected contact index.
- `move_sonar_cursor_to_target`: Steps the sonar cursor position stored in d8 globals toward the requested coordinates in eight-pixel increments, redraws the cursor, then waits for a keypress before returning.
- `render_sonar_screen_with_contacts`: Sets the sonar cursor globals, draws the grid and range lines, iterates the contact table drawing each visible blip, then draws the player marker and status panels.
- `run_intro_sequence_script`: Saves d7/a2, then drives a scripted intro: sets display and speed globals, blits text, loops counters calling the frame updater, and aborts early when the skip bit is set.
- `per_frame_game_update`: Advances frame counters, computes scroll and speed values into globals, calls the many per-frame subsystem updates, handles sound and end-of-level checks, then returns.
- `show_mission_briefing_screen`: Loads the futura font, clears the screen, prints a briefing text, loops eleven lines drawing them, waits for input and restores d7 before returning.
- `copy_229c_block_to_4c`: Loads the longword pointer at absolute address 229c into a0, sets a1 to absolute address 4c, then copies 877 longwords from (a0)+ to (a1)+ using a dbra loop.
- `blit_sprite_rows_into_bitplanes`: Stores three arguments in globals, adjusts the row offset, then loops six times reading masked sprite data and ORing it into three bitplane buffers at offsets 8000, 16000 and 24000.
- `save_stack_and_run_frame_loop`: Saves all registers, stores the stack pointer and a value at absolute address 210, calls five subroutines, then branches into an endless loop that restores the saved stack pointer and registers.
- `load_next_script_command_block`: Reads a command word and following parameters from a script pointer, stores them into globals, sets flag bits for nonzero fields, copies a sixteen-byte entry indexed by the command, advances the pointer and returns.
- `update_projected_ship_points`: Checks a flag and a mode word, then either adds camera offsets to each stored point or rotates each point by sine and cosine values before writing the projected coordinates to three buffers.
- `init_three_copper_color_registers`: Writes the value 16384 into three pairs of words starting at address 956, clearing the words between them, to set up three color registers.
- `compute_rotation_matrix_entries`: Calls the sine routine, then multiplies the sine and cosine values by six angle constants to fill six rotation matrix words in memory.
- `build_rotation_matrix_and_copy`: Calls the sine routine, scales the sine and cosine values by two angle constants into matrix words, then copies five words to a second matrix buffer.
- `update_trigonometric_table_parameters`: Calls a routine to calculate a sine or cosine-related value, then calculates a bit-shifted or scaled version of the result and stores a series of cleared or updated 16-bit and 8-bit values into a global structure or table, finally copying these values back to a separate global memory area.
- `transform_vertices_by_rotation_matrix`: Loops over the vertex count, reads three words per vertex, and either adds the translation offsets or multiplies by the rotation matrix at the matrix pointer, storing results into three output arrays.
- `project_vertices_with_perspective_scale`: Loops over the vertex count, clamps depth, computes a perspective scale via division, optionally clamps it, then scales x and y and stores screen coordinates into two output arrays.
- `cull_and_average_polygon_vertices`: Loops over polygons, tests edge orientation against the view arrays to cull back faces, averages vertex depths into a table with polygon pointers, and calls the sort routine.
- `insertion_sort_entries_by_first_longword`: Sorts the array of eight-byte entries at the table pointer using insertion sort, comparing the first longword of each entry and shifting entries up while larger, then returns.
- `build_display_lists_from_entry_table`: For each entry in the table, reads a word index to fetch a value into a display register, then copies pairs of words from two lookup tables into a buffer and calls the display routine.
- `copy_params_to_globals_and_vectors`: Saves registers, reads two longwords from the caller stack into the two global word variables, then copies a count of longword pairs into the global vector area before calling the next routine.
- `sort_and_render_vectors`: Saves all registers, shifts the global vector table up by one entry, finds the smallest vector, then walks the table calling the line and point drawing routines to render it.
- `rasterize_rising_sonar_line`: Draws a line into the a3 buffer from d4 upward, stepping d3 by the slope d5 and half-step d6, writing only where the new value is less than the stored word.
- `rasterize_falling_sonar_line`: Draws a line into the a3 buffer from d4 upward, stepping d3 by the slope d5 and half-step d6, writing only where the new value is greater than the stored word.
- `blit_span_into_bitplanes`: Clips a horizontal span between d4 and d5 to the stored bounds, computes bitplane addresses from d6 and a0, then sets or clears the span bits across four bitplanes using the masks in d0 and d2.
- `lookup_sine_table_pair`: Adjusts d0 by adding 1280 when negative, reads a word from the table at a2 indexed by d0, and reads a second word from the same table indexed by d0 plus 320, returning both in d1 and d2.
- `fetch_two_table_words_to_pointers`: Loads d0 and two pointer arguments from the stack, calls the table lookup routine, sign-extends d1 and d2, stores them through a0 and a1, then restores d2 and a2.
- `decode_hex_digit_from_value`: Takes a value from the stack, scans byte positions from high to low to find the first nonzero byte, then calls the nibble classifier and digit lookup to build a packed digit result in d0, restoring d2-d7/a0-a7.
- `classify_value_into_nibble_range`: Compares d4 against a ladder of thresholds and sets d2 to a range index and d3 to the matching lower bound, returning immediately without touching memory.
- `refine_nibble_index_by_scaled_thresholds`: Uses d2 shifted left by four as a base and compares d4 against scaled sub-thresholds to select a finer index in d6, returning without memory access.
- `binary_search_sorted_word_table`: Reads a signed word argument from the stack and binary-searches a sorted word table at a fixed base, halving the offset five times, then scans forward until a table entry is not less than the argument; returns the entry index in d0.
- `init_game_state_and_forbid`: Saves registers, calls two setup routines, sets the CIAA fire-button bit, stores two pointers derived from a base address into globals, calls an init routine, and if it returns zero calls another routine then Forbid via the exec library vector.
- `read_input_flag_into_global`: Clears the word at a fixed global, calls a routine with d0 zeroed, then if a stored pointer is nonzero copies it to d0 and sets bit zero of the global from bit four of that pointer.
- `call_2ae4_with_stack_arg`: Loads the longword at offset four from the stack into a1, calls the routine at address 2ae4, then returns to the caller.
- `store_229c_and_call_2a16`: Loads the stack argument into a1, writes the immediate value 229c to memory at 2a0c, calls the routine at 2a16 and returns.
- `seed_and_advance_random_generator`: Seeds a global from VPOSR if zero, multiplies it by a constant via a called routine, increments the seed, and returns bits eight through thirty-one.
- `ori_d0_with_dd_data_byte`: Performs a bitwise OR of the immediate byte dd into the low byte of d0; this single instruction is data decoded as code, not a real routine.
- `signed_32bit_multiply_via_16bit_parts`: Multiplies two signed 32-bit arguments from the stack using 16-bit partial products, negating inputs and result as needed, and returns the product in d0.
- `wait_for_copper_bit_toggle`: Polls bit zero of the custom chip register at offset five until it is set, then polls again until it is clear, then returns to the caller.
- `wait_for_vposr_line_step`: Reads VPOSR, masks the vertical position bits, waits until it is within the valid range, then waits for it to drop by 256 before returning.
- `check_beam_position_window`: Calls the beam position reader, swaps the result into the high word, compares it against a window and returns zero or one in d0 depending on whether it falls inside.
- `clear_buffer_and_read_beam_position`: Fills 50 bytes at the buffer pointed to by a0 with 255, then calls the beam position reader and stores its word result in memory.
- `init_sound_and_call_sequence`: Increments a long counter, doubles a1, stores a0 into a global, then calls six subroutines in sequence and returns d6 in d0.
- `configure_paula_and_trigger_irq`: Reads audio and interrupt registers from the custom chip base, programs four audio registers, writes control bytes to a1 offset 4352, increments a counter and returns.
- `clear_audio_bits_with_delay`: Clears bits 7 and 3 of the byte at a1 offset 4352, loops 101 times clearing bit 3, then calls two subroutines and returns.
- `delay_3000_iterations`: Loads 3000 into d6 and decrements it with dbra until it reaches minus one, creating a busy-wait delay, then returns to the caller.
- `wait_clear_bit5_at_a1_offset`: Repeatedly tests bit 5 of the byte at offset 8193 from a1 and branches back while it is set, returning once that bit clears.
- `toggle_bit0_while_bit4_set`: While bit 4 of the byte at offset 8193 from a1 is set, sets then clears bit 0 at offset 4352, calls the delay, and increments a word counter before returning.
- `wait_audio_flag_toggle_output_bits`: Polls bit 4 of the byte at offset 8193 of a1, calling a wait routine while set; then loops 79 times clearing bits 1 and 0 and setting bit 0 of the byte at offset 4352 of a1, calling a delay routine each pass, then busy-waits 15000 iterations.
- `reset_audio_channel_and_wait_ready`: Calls the audio wait routine, clears bit 2 at offset 4352 of a1, programs a6 audio registers (period, sample pointer, volume), then loops up to an address times checking bit 1 of the status word at offset 30 of a6 and sets offset 156 to 2 on ready.
- `clear_volume_set_period_increment_counter`: Clears the word at offset 36 of a6, sets the word at offset 150 of a6 to 16, increments the longword counter at the fixed address d47e, and returns.
- `wait_for_ay_register_ready`: Reads byte at offset 7424 from a1 twice, tests bit 4, and returns once that bit is set, otherwise loops.
- `set_ship_status_bits_and_ports`: Sets bits 7 and 3 then clears bit 3 at offset 4352 of a1, and writes four table words with bit 15 set into a6 offsets 154, 156, 150 and 158.
- `scan_sync_words_for_floppy_track`: Scans up to 6784 words from the table pointed to by the d0 table pointer for a sync word, shifting each word 16 times, then calls the decode routine and updates counters.
- `scan_words_for_sync_marker`: Scans d3 words from (a0), shifting each 32-bit value left up to 16 times to find the sync word, storing the bit index in d5 and writing it to (a4) while setting a flag byte and incrementing a counter.
- `zero_filled_data_area_2`: A single zero word that decodes as ori.b #0,d0; this is zero-filled data referenced by address, not executable code.
- `zero_word_flag_storage`: A single zero word that decodes as ori.b #0,d0; this is zero-filled data used as a flag byte by the sync scanner, not executable code.
- `zero_filled_data_area_3`: Three zero words that decode as ori.b #0,d0; this range is zero-filled data referenced by address, not executable code.
- `set_copper_list_four_registers`: Reads four stack arguments, masks the first two to multiples of sixteen, adds fifteen to the second, and writes them to the custom chip registers at dc90, dc92, dc94 and dc96.
- `install_copper_list_and_run_chain`: Saves the old copper list pointer, installs a new list at dc88 with zero at dc8c, calls the copper setup, restores the pointer, then saves registers and calls the next routine.
- `walk_list_call_each_entry`: Stores the incoming pointer in a global, saves all registers, then walks a null-terminated list of pointers calling the routine at each entry and advancing by four bytes.
- `run_six_stage_update_chain`: Saves all registers, calls six fixed subroutines in sequence, restores all registers and returns to the caller.
- `load_ship_state_from_record`: Reads a record pointed to by a1, copies flags, dimensions and four longwords into globals, sets counters and returns with a0 and a1 pointing at the copied data.
- `setup_blit_pointer_table_and_clear_row`: Tests the enable flag, computes a row offset from the width and pitch globals, stores a destination pointer into the table at a0, and clears a word at the end of the row.
- `expand_16_bytes_through_lookup_table`: Reads sixteen bytes from a5, translates each through the table at a4, writes them backwards to a3, advances a5 by three, adds d7 to a3 and loops via dbra d2 and d4.
- `blit_two_playfield_bitmap_strips`: Computes a blit size from the width and height globals, programs the blitter pointers, modulo, control and size registers twice to copy two bitmap strips, then returns.
- `adjust_viewport_and_blit_offsets`: Compares d1 and d0 against viewport bounds in memory, clamps the visible width and height, updates the source and destination blit offsets and modulo values, and returns early if the requested scroll exceeds the buffer.
- `compute_blit_pointer_and_modulo`: Builds a screen pointer from the row and column globals into d0, stores the row stride in d1, and computes the destination modulo in d2 and d3 for the blitter setup.
- `blit_scroll_strip_with_four_passes`: Reorders four source pointers into the blit table, programs the blitter pointers, modulos, control and size registers, then repeats the blit four times advancing the destination by 8000 bytes before restoring DMA.
- `wait_dmaconr_blitter_busy_clear`: Executes two no-ops, then repeatedly tests bit 6 of the DMACONR register at the custom chip base until it clears, then returns.
- `reset_audio_and_call_da20`: Saves all registers, calls the routine at da20, then writes zero to the audio period and volume registers and sets the audio control and modulation registers before restoring registers.
- `fill_byte_table_with_bit_reversed_index`: Loads the table address into a0, then for each byte value from zero to 255 stores a bit-reversed transformation of the index into the table and returns.
- `store_stack_arg_to_dc84_pointer`: Copies the longword at offset four from the stack pointer into the global pointer at dc84, then falls through into the following routine.
- `blit_sprite_frames_to_bitplanes`: Walks the sprite list, copies frame data into a buffer, programs the blitter registers and blits each sprite into the display bitplanes, then restores registers and returns.
- `rle_decode_sprite_row_bytes`: Decodes run-length encoded bytes from a4 into a5 using d6 and d7 as escape markers, writing zeros or repeated bytes until d3 pixels are filled, then returns.
- `unpack_bits_to_bytes_with_zero_fill`: Reads a bit mask byte from (a4), and for each of d3 bits copies the next byte from the offset table at a6 when the bit is set, otherwise writes zero to (a5), advancing both pointers.
- `relocate_hunk_pointer_table`: Walks the longword pointer table at the address stored in the global pointer, adding the base plus 2048 to each entry and to the four longs it points at, then writes a terminating zero.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `bounded_string_copy_into_buffer`: Measures the source and destination strings, copies at most the given length from source to the end of destination, null-terminates it, and returns the destination pointer in d0.
- `open_file_and_attach_stream`: Pops three arguments, opens a file via the first helper, stores the returned handle in a3, then passes the handle's field and two arguments to the second helper and returns its result or minus one on failure.
- `flush_pending_stream_buffers`: Walks the a4-relative linked list, and for each node whose flags allow it and whose write range is non-empty, calls the write helper with the node's buffer, offset and length, then flushes the passed argument.
- `write_buffer_to_dos_file`: Clears the error flag, calls dos.Write with the passed file handle, buffer and length, and on failure stores dos.IoErr into the error slot and sets a status of five before returning the write result.
- `seek_dos_file_with_mode_offset`: Saves d2-d7, optionally calls a cleanup, clears offset 216 of a4, calls dos Seek on the file handle at 592(a4) with mode-dependent offset, stores IoErr on failure and returns the resulting position.
- `close_dos_file_handle`: Saves d7, loads the file handle from the stack, optionally calls a cleanup routine, then calls dos Close through the a6 library vector at offset 250(a4) and returns zero in d0.
- `multiply_32bit_unsigned_words`: Multiplies the 32-bit values in d0 and d1 using partial 16-bit products, combining high and low halves into a full 32-bit result in d0, preserving d2 and d3.
- `signed_divide_32_by_32`: Takes signed 32-bit dividend in d0 and divisor in d1, negates operands to make them positive, calls the unsigned division routine, then negates the quotient and remainder to restore signs and returns them in d0 and d1.
- `unsigned_divide_32_by_32`: Divides unsigned 32-bit d0 by d1 using shift-and-subtract normalization, returning quotient in d0 and remainder in d1 while preserving d2 and d3 on the stack.
- `lookup_object_by_index`: Reads an index from the stack, clears the error field at offset 216 of a4, validates it against the count at a4 and the table at offset 268, returning the entry pointer in d0 or setting error 9 at offset 588.
- `free_command_list_nodes`: Walks the linked list at offset 596 of the a4 base, freeing each node's payload via exec FreeMem, then clears the list head and the count at offset 600.
- `open_and_run_command_file`: Looks up a file entry, optionally opens it with mode 2, then passes its handle and the two caller arguments to a run routine, returning minus one on failure or if the error flag at offset 216 is set.
- `parse_command_line_and_open_console`: Tokenizes the argument string into up to 32 pointers, then opens a console window or uses the existing one, sets up file handles and signal bits, and installs the command handler.
- `copy_d0_to_d5`: Copies the longword in d0 into d5, then falls through into the following routine.
- `jump_to_absolute_1c0`: Jumps to the absolute address 448 in low memory, transferring control without saving a return address.
- `call_library_offset_348_via_a6`: Saves d2-d3 and a2-a3/a6, loads a6 and argument registers a0-a3 and d0-d3 from the stack, calls library offset -348 through a6, restores registers and returns.
- `flush_pending_sound_slots`: Saves d5-d7, walks the sound slot table at offset 268 of a4 from the last index down to zero, calls the stop routine for each active non-flagged slot, then calls the final flush routine with the argument.
- `jump_through_audio_vector`: Performs a single indirect jump to the code pointer stored at offset 332 of the a4 base register, transferring control to the audio driver entry.
- `clamp_sound_index_and_setup_frame`: Builds an a5 stack frame, saves d2-d3/d6-d7/a6, reads a byte before the pointer at offset 264 of a4 into d7, clamps it to at most 79, and loads a1 with a local buffer.
- `copy_bytes_and_open_dos_output`: Copies d0 bytes from a0 to a1, clears a byte in the a5 frame, calls exec FindTask, then opens a dos output file and writes a break message into it, returning -1.
- `open_intuition_library_store_base`: Loads the exec library base from address 4, opens intuition.library with version 0, stores the base at -102(a5) and returns -1 if opening failed.
- `push_args_and_call_sim_setup`: Stores the a5 buffer pointer at 144(a4), pushes nine arguments including 250 and 60 onto the stack, calls the setup routine, cleans 36 bytes and returns 0 on success or -1.
- `return_zero_restore_regs_and_frame`: Sets d0 to zero as the return value, restores d2-d3/d6-d7/a6 from the stack, releases the a5 frame and returns.
- `poll_signal_and_run_pending_callback`: Reads the exec signal mask, and if the 12288 bits are set and a callback pointer at offset 240 of a4 exists, calls it and clears the pointer, then calls a cleanup routine with argument 20.
- `blitter_line_draw_setup`: Stores five arguments, computes a bitplane address from x and y, derives octant and error terms, then programs blitter data, modulo, control, pointer and size registers to draw a line.
- `blit_scanline_range_with_saved_clip`: Saves five stack arguments and the current blit clip rectangle, sets the clip to a full-screen area, then repeatedly calls the blit routine for each scanline from the start to the end argument before restoring the clip.
- `blit_three_band_scanline_range`: Saves arguments and the blit clip rectangle, sets a full-screen clip, then for each scanline calls the blit routine three times using the left, right and center coordinates before restoring the clip.
- `expand_tilemap_to_three_planes`: Reads tile indices from a source buffer and writes three color bytes per tile into three interleaved bitplane rows of a destination buffer, repeating across a grid of tiles and four screens.
- `toggle_flag_and_branch_if_zero`: Exclusive-ORs the byte at absolute address 1574 with 255, then branches to 15a8 if the result is zero.
- `set_flag_and_swap_two_pointers`: Writes 255 to byte 1574, loads address 229c into a1, copies longwords 22ac to 23a2 and 229c to 239e, then branches to 15c8.
- `clear_flag_and_copy_four_pairs`: Clears byte 1574, swaps the two pointer longwords, then copies four word pairs from 22ac into the buffer at 28fc, advancing eight bytes each time.
- `blit_ship_sprite_rows_to_bitplanes`: Computes source and destination bitplane pointers from row and column arguments, programs blitter pointers, modulo, control and size registers, then blits four rows advancing each pointer by eight thousand bytes.
- `copy_longwords_count_bytes`: Copies a block of memory from the source pointer to the destination pointer, moving one longword per iteration for the given byte count divided by four.
- `subtract_speed_delta_from_sprite_offsets`: Loads a base pointer and computes a byte delta from two globals, then subtracts that delta from eight byte fields spaced eight bytes apart in the structure.
- `draw_ship_radar_blips_and_clear_columns`: Saves all registers, computes a ship position from the packed coordinate word, draws two radar blips via the blit helper, then fills three screen columns with ones and one column with zeros.
- `blit_sweep_line_across_four_bitplanes`: Waits for the blitter, programs BLTCON0/1, modulo, pointers and size registers, then repeats the blit four times advancing both source and destination by the plane stride.
- `clear_four_bitplanes_with_blitter`: Waits for the blitter, sets BLTCON0 to 256 and BLTSIZE to 3220, points BLTDPTH at a6 and repeats the clear blit four times stepping a6 by the plane stride.
- `draw_radar_sweep_line_40px`: Advances the radar sweep angle by 64 units, wraps at 640, computes sweep line endpoints and blits the sweep line into the radar bitmap via the line routine, then clears the radar display rows to all-ones and zeroes.
- `blit_radar_background_planes`: For four planes, sets blitter control registers, destination modulo and pointer, computes blit size from the vertical offset, and starts a blit advancing the pointer by 8000 bytes each iteration.
- `draw_radar_sweep_line_32px`: Advances the radar sweep angle by 32 units, wraps at 640, computes sweep line endpoints and blits the sweep line into the radar bitmap via the line routine, then clears the radar display rows to all-ones and zeroes.
- `blit_three_radar_columns`: Saves d0 and a0, calls the blit setup helper three times, and programs BLTCON0, BLTCON1, BLTDMOD, BLTDPTH and BLTSIZE for each of three descending screen columns.
- `set_blit_pointers_from_237a_offset`: Loads the longword at 237a, adds 12000 to it, stores the result at 1fdc, copies the original value to 1fe0 and the longword at 23a2 to 1fe4, then branches onward.
- `set_blit_pointers_from_2372_offset`: Loads the longword at 2372, adds 12000 to it, stores the result at 1fdc, copies the original value to 1fe0 and the longword at 23a2 to 1fe4, then branches onward.
- `setup_blit_pointers_and_jump`: Loads the base pointer, adds 24000 to form a destination pointer, stores three pointers into globals, then branches to another routine.
- `draw_three_band_blits_with_offsets`: Runs three blitter passes over four rows each, setting blitter pointers, modulos, control and size registers, with different offsets and sizes per pass.
- `blit_four_rows_at_offset`: Sets up four blitter rows starting at the base pointer plus offset, writing blitter control, modulo, pointer and size registers, then restores registers.
- `blit_three_plane_rows_to_bitmap`: Reads source and destination offsets and a modulo from the caller's stack frame, computes blitter pointers and size, then writes BLTCON0, BLTAPTH, BLTDPTH, modulos, masks and BLTSIZE to blit three rows of bitplanes.
- `open_libs_alloc_chip_memory_and_init`: Opens dos.library and graphics.library via OldOpenLibrary, allocates a large chip memory block with AllocMem, stores the base and derived pointers in globals, then calls the setup routine.
- `compute_playfield_plane_pointers`: Saves all registers, then derives a series of plane and buffer pointers by adding fixed offsets to the base pointer and stores them along with two copied globals, restoring registers before returning.
- `close_audio_and_free_sound_buffers`: Opens the exec library base, closes two audio resources with CloseLibrary, frees two memory blocks with FreeMem, then jumps to RemIntServer to remove an interrupt server.
- `stop_copper_dma_and_close_library`: Saves registers, writes the copper list pointer to COP1LCH, strobes COPJMP1, disables and re-enables DMA channels via DMACON, closes the exec library, restores registers and returns zero in d0.
- `build_copper_list_and_enable_display_dma`: Copies and offsets sprite and bitplane pointer tables, fills a copper list with display window and bitplane registers, then writes COP1LCH, COPJMP1 and DMACON to start the display.
- `alloc_two_ports_and_add_interrupt_servers`: Opens exec library, allocates 350 bytes of memory, stores two port pointers, fills port fields with message pointers and handler addresses, then adds two interrupt servers.
- `clear_d0_after_timer_tick`: Calls the countdown routine at the next address, then sets d0 to zero and returns to the caller.
- `decrement_three_timers_and_reset_ports`: Decrements three word counters, and when each reaches zero clears its flag, writes a message pointer into the port and sets a word field to ten.
- `open_read_close_dos_file_with_retry`: Opens a DOS file by name with mode 1005, reads up to an address bytes into the caller's buffer, closes the handle, and retries after calling a wait routine when opening fails.
- `read_joystick_port_and_fire_button`: Reads JOY1DAT and CIAA_PRA to build a four-bit direction value in d0, and sets a flag word when the fire button bit is clear.
- `stop_dma_clear_copper_and_audio_registers`: Disables DMA via DMACON, writes a stop value through the copper list pointer, re-enables DMA, calls a sound routine, then clears sixteen audio registers.
- `set_color08_color09_from_palette`: Calls a helper, then writes the immediate color values to the COLOR08 and COLOR09 hardware registers and returns.
- `decompress_bodyy_rle_into_bitmap`: Saves registers, scans for the BODY marker, then expands run-length encoded bytes into a 40-byte-stride bitmap plane using d7 row and d2 plane counters.
- `load_palette_colors_into_color00`: Saves registers, reads a count from the structure, divides by three, then converts packed nibble color bytes and writes each word to consecutive COLOR registers starting at COLOR00.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `adjust_nibbles_toward_target_table`: Loops over 134 bytes, comparing low and high nibbles of each byte in the first table against the second table and incrementing or decrementing each nibble to move it toward the target value.
- `build_copper_list_from_sprite_data`: Reads a sprite descriptor selected by the argument, disables DMA via DMACON, writes a copper list of sprite position and data words into the buffer at the global pointer, terminates it, then re-enables DMA.
- `reset_copper_list_and_call_display_init`: Saves all registers, disables DMA through DMACON, writes an end marker to the copper list buffer, re-enables DMA, calls the display initialization routine, then restores registers and returns.
- `dispatch_indexed_handler_via_stack_arg`: Loads the longword at offset four from the stack into d0, branches to a shared tail, then calls two subroutines and returns.
- `init_four_audio_channels_from_table`: Enables DMA, loops four times writing a base pointer, period, and volume into channel structures, computes a divisor-based period, then sets ADKCON and returns zero in d1.
- `load_period_from_word_table_by_index`: Takes the word index in d0, doubles it, reads a word from the table at a fixed address and stores it into the shared period variable before returning.
- `update_four_channel_minimums`: Reads flags and a value from the caller's stack frame, updates four global minimum thresholds and calls a channel setup routine for each newly lowered channel, returning the last channel index.
- `setup_audio_channel_from_sample`: Enables one DMA audio channel, stores the sample pointer and length, computes the period from the sample rate, writes the volume and returns zero.

### echo

- `load_and_play_echo_sample`: Allocates memory, builds a sample header, loads the echo sample through library calls, copies it into the audio structure and starts playback, returning zero or minus one.
- `copy_d1_to_d0_and_branch`: Moves the longword in d1 into d0, then branches unconditionally to another routine at a fixed offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_to_runtime_entry_point`: Unconditionally jumps to the runtime entry point at the fixed address, transferring control without saving any registers or returning.
- `undecodable_bytes_8`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_9`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_10`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_11`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_12`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_13`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_14`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_15`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_16`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_to_shared_runtime_entry`: Performs an unconditional jump to another routine in the runtime segment, transferring execution without touching registers or memory.
- `jump_to_alternate_runtime_entry`: Performs an unconditional jump to a different routine in the runtime segment, passing control onward without modifying registers or memory.
- `jump_to_unreached_runtime_entry`: Performs an unconditional jump to a further routine in the runtime segment; this entry is not reached in the reference run.
- `undecodable_bytes_17`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_18`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_19`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_to_shared_runtime_entry_2`: Performs an unconditional jump to another routine in the runtime segment, transferring control without touching registers or memory.
- `undecodable_bytes_20`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_to_unreached_runtime_target`: Performs an unconditional jump to a target routine in the runtime segment; this stub is not reached during normal execution.
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
- `jump_to_runtime_dispatch_target`: Performs an unconditional jump to another routine in the runtime segment, passing control onward without modifying registers or memory.
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
- `undecodable_bytes_175`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_176`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_177`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_178`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_179`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_180`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_181`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_182`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_183`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_184`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_185`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_186`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `load_ed_into_d0_then_branch`: Loads the immediate value 237 into data register d0, then branches unconditionally to the code at address 4cee.
- `set_d0_to_2_and_branch`: Loads the immediate value 2 into data register d0, then branches unconditionally to a shared routine elsewhere in the segment.
- `set_d0_to_66_and_branch`: Loads the immediate value 66 into data register d0, then branches unconditionally to a shared routine elsewhere in the segment.
- `set_d0_to_65_and_branch`: Loads the immediate value 65 into data register d0, then branches unconditionally to a shared routine elsewhere in the segment.
- `load_minus_six_into_d0_and_branch`: Loads the immediate value minus six into d0 and then branches unconditionally to a shared routine elsewhere in the segment.
- `load_minus_three_into_d0_and_branch`: Loads the immediate value minus three into d0 and then branches unconditionally to a shared routine elsewhere in the segment.
- `load_ninety_three_into_d0_and_branch`: Loads the immediate value ninety-three into d0 and then branches unconditionally to a shared routine elsewhere in the segment.
- `set_d0_ff_and_branch_away`: Loads the immediate value 255 into data register d0 and then branches unconditionally to another part of the segment.
- `nop_then_branch_away`: Executes a no-operation instruction and then branches unconditionally to another location in the segment.
- `increment_a6_offset_20_word`: Adds one to the 16-bit word at offset 32 from address register a6, then copies a6 into d0 and returns.
- `decrement_counter_and_return_zero`: Decrements the 16-bit word at offset 32 of a6, then sets d0 to zero and returns to the caller.
- `undecodable_bytes_187`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_188`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_189`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_190`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_191`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `push_return_address_and_branch`: Subtracts 8 from the stack pointer a7, pushes the immediate long value, then branches to another routine.
- `push_frame_and_load_pointer`: Subtracts 8 from a7, pushes an immediate long, branches away, then stores a5 and loads new a5 pointers before returning.
- `pop_a5_and_return`: Pops a5 from the stack and returns immediately to the caller, restoring the saved frame pointer.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `forbid_and_call_cleanup_then_restore_regs`: Calls two subroutines, loads the exec library base from address 4, calls Forbid via offset -132, then restores d1-d7 and a0-a6 from the stack and returns.
- `clear_flag_and_test_bit_four`: Clears the word at the flag address and d0, calls a subroutine, then if the returned longword is nonzero extracts bit 4 into the flag word and returns it in d0.
- `jump_to_two_blitter_helpers`: Transfers control with a jump to one external address, and the following jump to a second external address is unreachable after the first.
- `wait_for_vposr_window_match`: Masks d0 to the vertical position bits, checks it lies within a range, then polls VPOSR until the masked value equals d0 minus 256, returning when matched.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `blit_two_playfield_bitmap_passes`: Loads blitter pointers, modulos and control words from a0 and a global, starts a first blit via a helper, then reprograms the blitter for a second pass and returns.
- `blitter_setup_and_blit_loop`: Writes blitter control, modulo, pointer and size registers from globals and registers, calls a helper, then advances a1 by 8000 and repeats dbra d4 times.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `ori_byte_immediate_d5`: Performs a bitwise OR of the immediate byte value 30 into the low byte of d5, leaving other bits unchanged.
- `ori_byte_immediate_d2`: Performs a bitwise OR of the immediate byte value 164 into the low byte of d2, leaving other bits unchanged.
- `set_d2_low_bits_24`: ORs the immediate value 36 into the low byte of data register d2, setting those bits and leaving the rest of d2 unchanged.
- `set_d2_low_bits_24_alt_entry`: ORs the immediate value 36 into the low byte of data register d2, setting those bits; entered via a separate taken address from the same caller.
- `set_d2_low_bits_a4`: ORs the immediate value 164 into the low byte of data register d2, setting those bits and leaving the rest of d2 unchanged.
- `ori_b_24_into_d2`: Performs a bitwise OR of the immediate byte 36 into the low byte of data register d2, leaving the rest of d2 unchanged.
- `ori_b_a4_into_d2`: Performs a bitwise OR of the immediate byte 164 into the low byte of data register d2, leaving the rest of d2 unchanged.
- `ori_b_3c_into_d3`: Performs a bitwise OR of the immediate byte 60 into the low byte of data register d3, leaving the rest of d3 unchanged.
- `ori_byte_immediate_into_d3`: Performs a bitwise OR of the immediate byte value 36 with the low byte of data register d3, leaving the result in d3.
- `ori_byte_immediate_into_d4`: Performs a bitwise OR of the immediate byte value 44 with the low byte of data register d4, leaving the result in d4.
- `ori_byte_immediate_into_d4_alt`: Performs a bitwise OR of the immediate byte value 126 with the low byte of data register d4, leaving the result in d4.
- `ori_b_7e_into_d5`: ORs the immediate byte value 126 into the low byte of data register d5, leaving the rest of d5 unchanged.
- `ori_b_7e_into_d5_second_entry`: A second entry point that ORs the immediate byte value 126 into the low byte of data register d5, identical in effect to the earlier entry.
- `ori_b_b0_into_d1`: ORs the immediate byte value 176 into the low byte of data register d1, leaving the upper bytes of d1 unchanged.
- `ori_b_immediate_into_d1`: Performs a bitwise OR of the immediate byte value 176 into the low byte of data register d1, then falls through into the following code.
- `init_audio_channel_pointers_and_dma`: Saves all registers, fills a table of audio sample pointers by adding fixed offsets to a base address, then programs four audio channel structures with period and volume and enables DMA.
- `allocate_sample_buffer_from_table`: Calls a system routine, indexes a pointer table by d0, stores the chosen pointer into the a2 list, copies sample start and length fields from a0 into the buffer header, and sets a flag bit in d1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_11`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_12`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_13`: Empty routine: returns to the caller immediately without doing anything.
