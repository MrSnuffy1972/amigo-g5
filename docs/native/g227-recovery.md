# Game 227 (AdvancedDestroyerSimulator v1.0 2151): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00227). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (602316 top-level bytes in 30 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 22 raw-word lines |
| Functions named and described | pass, 559 of 559 named (0 author symbols), 559 of 559 described (207 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2006 of 2006 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 16525 of 16525 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 419738 |
| code | 59394 |
| data | 122985 |
| padding | 199 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| AdvancedDestroyerSim.Slave | 1 | 958 | 72 |
| dt | 12 | 15020 | 239 |
| echo | 2 | 117 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 268 | 212 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 77 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 78 | 19 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 7 |

## Cases handled

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


### AdvancedDestroyerSim.Slave

- `return_minus_one_in_d0`: Loads the immediate value minus one into data register d0 and returns immediately to the caller.
- `store_base_and_init_vectors`: Saves the incoming a0 base pointer to memory, calls library offsets through a5 to set up interrupt and reset vectors, then jumps through the base pointer.
- `set_chip_ram_vectors_and_jump`: Loads the base pointer from memory, adds chip RAM offsets, stores the resulting addresses into a5 fields, calls a setup routine and jumps to a5.
- `set_a3_to_1mb_and_jump_a5`: Loads the immediate value an address into address register a3, then jumps to the address held in a5 without saving a return address.
- `read_and_mask_pc_relative_word`: Reads a 32-bit value from memory at the current program counter offset, clears bits 4 through 6 with a mask, and returns the result in d0.
- `test_intreqr_and_stack_bit5`: Reads the interrupt request register at the hardware base, tests bit 5 of a byte on the stack, and returns without changing registers.
- `acknowledge_interrupt_from_stack`: Reads the word at the top of the stack, writes it to the interrupt request register to clear interrupts, advances the stack pointer by two and returns.
- `push_return_and_set_a3_to_e2c`: Makes room on the stack, moves the caller return address and a3 into the new frame, loads a3 with the address e2c past pc, sets d0 to zero and d1 to a2, then returns.
- `pop_d1_push_e2c_and_set_a1_bit1`: Pops d1 from the stack, pushes the address e2c past pc, pushes d1 back, sets bit 1 of the byte at offset 14 in a1 and returns.
- `point_a0_at_custom_chip_base`: Loads the custom chip register base address into a0 and branches to another routine that continues setup.
- `store_d0_to_scratch_and_load_ciab_base`: Writes d0 into a scratch longword in program memory, loads the CIAB base address into a6, then returns.
- `copy_scratch_to_cop2lch_and_jump`: Copies the stored scratch longword into the COP2LCH hardware register and jumps through the address in a0.
- `copy_a0_offset6_byte_to_a6_800`: Reads the byte at offset 6 from the structure pointed to by a0 and stores it at offset 2048 from a6, then returns.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `check_pc_relative_long_for_21000_or_a1000`: Loads the long at the pc-relative location into d1 and returns 4 in d0 if it equals either an address or an address, otherwise returns 1.
- `init_two_minus_two_markers_and_clear_flag`: Writes minus two as a long at a3 and at the next long, stores the advanced a3 into offsets 132 of a4 and 50 and 54 of a2, clears the word at offset 386 of a4, skips fourteen bytes of return address and returns.
- `copy_two_node_values_to_a3_offsets`: Reads two pointers from offsets four and eight of a1; if non-null, copies the longword at offset four of each target into offsets 50 and 54 of a3, then pops four bytes and returns.
- `return_minus_one_if_d0_nonzero`: Decrements d0, sets the byte to all ones if the result is nonzero, sign-extends it to word and long, and returns that value in d0.
- `match_byte_against_two_pc_values`: Inverts and rotates d0, compares it with two bytes stored after the code, branches on matches, and otherwise stores the VHPOSR byte into a table at a1.
- `push_minus_one_and_branch_to_0330`: Pushes the immediate value minus one as a longword onto the stack, then branches unconditionally to the code at offset 0330.
- `restore_registers_and_disable_interrupts`: Tests d0, and if nonzero unwinds several saved register sets from the stack, writes to INTENA to disable interrupts, then rearranges stack words and returns.
- `compute_scaled_time_into_a2_offset`: Reads three longwords from the structure pointed to by the PC-relative pointer, multiplies and shifts them, divides one by fifty, and stores the combined result at offset 198 of a2 before restoring registers.
- `update_disk_status_byte`: Reads byte at offset 67 of a3, clears byte at offset 31 of a1, checks bit 1 of byte at offset 64 of a3, and depending on state calls two unnamed routines and returns the byte at offset 31 of a1 in d0.
- `data_table_not_executable`: The bytes decode as negating a word at offset 736b of a1 and toggling bit 7 of byte at offset 65 of a3, but this is data referenced by a lea, not executable code.
- `data_table_bit_test_fragment`: The bytes decode as testing bit d1 of a byte and storing a flag at offset 32 of a1, but this is unreached data decoded as code, not executable instructions.
- `push_pc_relative_pointer_offset_29e30`: Pushes the longword stored at the program counter relative location onto the stack, adds the immediate value an address to it, and returns.
- `push_pc_relative_pointer_offset_296e8`: Pushes the longword stored at the program counter relative location onto the stack, adds the immediate value an address to it, and returns.
- `clear_bit_and_toggle_flag_at_a3`: Reads byte at offset 67 of a3, clears that bit in a global byte, and if it was set toggles bit 1 at offset 64 of a3 and increments the longword at offset 294.
- `load_d1_10001_and_branch`: Loads the immediate value 65537 into data register d1 and branches unconditionally to a shared routine elsewhere.
- `read_byte_at_a0_offset_d2`: Tests d2 for negative; if non-negative, reads the byte at address a0 plus d2 into d3, copies d3 to d1 and returns.
- `load_d1_280_and_branch`: Loads the immediate value 280 into data register d1 and branches unconditionally to a shared routine elsewhere.
- `clear_a1_offset_c_and_store_a4_pointer`: Clears the longword at offset 12 of the structure pointed to by a1, loads 12 into d4, then stores a4 into the absolute longword at the address loaded from pc-relative location 4c8.
- `zero_filled_data_area`: This single zero word decodes as ori.b with immediate zero to d0; the bytes are data, not executable code, referenced by address from other routines.
- `compare_a4_to_stored_pointer_and_dispatch`: Compares a4 with the longword stored at pc-relative location 4c8; if equal it branches past the indirect jump, otherwise jumps through the pointer at offset minus 12 of a5.
- `store_pc_relative_pointer_in_self`: Loads a program-counter-relative address into a3, writes that same address into memory at offset minus 314 from a3, then loads another pc-relative address into d1 and returns.
- `open_whdload_device_and_enqueue_task`: Allocates 68 bytes of chip memory, opens expansion.library and whdload.device, builds a task structure with entry point and stack, enqueues it, and returns.
- `open_dos_library_and_jump_offset`: Calls the dos.library open function with a pc-relative name, then jumps through the longword at offset 22 of the returned structure.
- `open_dos_and_wait_for_message_port`: Opens dos.library, builds a message port named DOS, finds the current task, hooks the port into its task structure, then waits for and dispatches incoming messages in a loop.
- `compare_d7_with_task_field_and_branch`: Calls two subroutines with d7, loads the task field at offset 28 into d2, calls a third subroutine, shifts d0 right by two and branches based on whether the result is zero.
- `call_two_routines_and_return_ff`: Calls the routine at the first target, moves d7 into d0, calls the second target, sets d0 to minus one and branches to the third target.
- `copy_stream_chunk_to_buffer`: Advances the stream position by the requested count, copies that many bytes from the stream buffer into the caller's buffer, and returns the copied count in d0 with d1 zero.
- `append_bytes_to_stream_buffer`: Adds the requested byte count to the stream position, grows the high-water mark, and copies bytes from the caller's buffer into the stream buffer, flushing when full.
- `flush_stream_tail_and_restart`: Moves the remaining byte count and buffer pointer into d0 and d1, calls the stream write routine through a2, then reloads the requested count and jumps back into the main loop.
- `set_d2_minus_two_and_branch`: Loads minus two into data register d2 and branches unconditionally to the following routine, sharing its continuation.
- `open_file_and_store_handle`: Calls setup routines with d7, tests the returned longword, stores the handle at offset 36 of the d7 structure, and returns minus one or zero.
- `open_library_and_read_entry`: Calls setup routines with d7, opens a library via a2, reads a longword from the returned pointer, calls the library close vector, then branches to the file-open routine.
- `release_sound_channel_and_restart`: Loads the sound structure pointer from offset 20 of a4, checks the longword at offset 256; if nonzero it loads channel data, calls the routine at offset 56 of a2, then reloads the structure and calls a cleanup routine before returning minus one.
- `accumulate_velocity_into_d2`: Loads the structure pointer from offset 20 of a4 and copies the longwords at offsets 24 and 28 of a4 into d2 and d1, adding the value at offset 148 of the structure to d2 when d1 is positive.
- `clamp_and_store_position_delta`: Adds the longword at offset 20 of a0 to d2, compares it with the limit at offset 148 of a0, and if not above stores d2 there, returning the old value in d0 with d1 cleared.
- `set_d0_ff_and_d1_to_db_branch`: Loads d0 with minus one and d1 with thirty-six, complements the low byte of d1 to two hundred nineteen, then branches away to another routine.
- `load_a4_offset_20_shifted_left_two`: Reads the longword at offset twenty from a4 into d7, shifts d7 left by two bits, and returns to the caller.
- `load_a4_offset_24_shifted_left_two`: Reads the longword at offset twenty-four from a4 into d7, shifts d7 left by two bits, and returns to the caller.
- `scale_a4_offset_28_by_four`: Loads the longword at offset 28 from a4 into d7, shifts it left by two bits, and returns the result in d7.
- `shift_string_left_and_store_length`: Copies bytes from a0 forward one position, shifting the string left until a zero byte, then writes the byte count before the start of a0.
- `open_and_init_108_byte_io_structure`: Calls a setup routine, allocates 264 bytes via exec, fills the new structure with d2, a5 and a3, and returns its pointer in d0 or zero on failure.
- `close_workbench_window_and_free`: If d0 holds a window pointer, saves its user data and screen pointer, calls the intuition close-window function, then frees the saved screen pointer and returns the result.
- `build_trimmed_path_into_allocated_buffer`: Takes a directory string in d0 and a name in d1, allocates a buffer, copies the directory, appends a slash and the name, collapses duplicate slashes, and returns the buffer or an error code.
- `verify_whdboot_files_present`: Checks for the WHDBoot.exe and WHDBoot.ss files by calling the file-check routine twice, returning zero on success or the failing result.
- `compare_null_terminated_strings`: Saves a0 on the stack, compares bytes at a0 and a1 until a mismatch or a zero byte, then restores a0 and returns.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `push_dos_base_plus_thirtytwo`: Pushes the longword stored at the program counter relative location onto the stack, adds thirty-two to it, and returns the adjusted pointer.
- `open_dos_library_and_start_main`: Opens dos.library via exec, sets up a6, calls initialization, runs the main entry with arguments, then jumps to the library close vector.
- `install_three_traps_and_show_two_texts`: Writes jump instructions and handler addresses into low memory, then calls a setup routine with d2 zero and ten and displays two strings via offset 100 of a2.
- `wait_for_beam_position_and_button`: Calls a library function, then polls VPOSR until the beam is in range and checks the VPOSR bit in custom register 5, returning when conditions match.
- `jump_to_1228_entry`: Branches unconditionally to the code at the target address, transferring execution without saving registers or returning.
- `call_1228_with_d2_as_d0`: Saves d0 on the stack, moves d2 into d0, calls the routine at 1228, restores d0 from the stack and returns.
- `call_1228_with_d6_as_d0`: Saves d0 on the stack, moves d6 into d0, calls the routine at 1228, restores d0 from the stack and returns.
- `wait_for_copper_and_fire_release`: Polls the custom chip register until bit 6 clears, then tests the CIA port and the same register again before returning.
- `skip_d7_pointer_table_entries`: Uses d7 as a base pointer, decrements d2 and walks d2+1 longword pointers by doubling and adding the base, then advances a1 past the table and returns.
- `allocate_and_call_expansion_library`: Saves all registers, calls the expansion library open routine with a0, invokes the a5 callback and a helper, then calls the library close routine and restores registers.
- `count_vhposr_changes_into_memory`: Loops ten times reading the VHPOSR hardware register, counting changes in d1, then stores the count through a0 and returns.
- `divide_d0_by_40_keep_high_word`: Swaps d0 to move its high word into the low word, clears the low word, swaps back, divides the long value by 40, then clears the low word again and branches to another routine.

### dt

- `open_dos_library_and_run_main_loop`: Saves registers, clears a table, opens dos.library, sets up signals and ports, builds a command line from arguments, opens the current directory and runs the main program loop before returning a status code.
- `shutdown_close_library_and_exit_interrupt`: Calls an optional cleanup pointer, forbids and closes the dos library, replies any pending message, exits the interrupt handler, restores the saved stack frame and registers, and returns.
- `jump_to_de36_target`: Performs an unconditional jump to the fixed target address de36, transferring control without saving any registers or return address.
- `jump_to_de34_target`: Performs an unconditional jump to the fixed target address de34, transferring control without saving any registers or return address.
- `jump_to_e150_target`: Performs an unconditional jump to the fixed target address e150, transferring control without saving any registers or return address.
- `jump_to_input_poll_vector`: Jumps directly to the external input polling routine, transferring control without touching any registers or stack.
- `main_game_loop_and_setup`: Copies a table of bytes, initialises display and sound, then runs the main loop polling input, updating counters, timers and game state until quit.
- `poll_input_and_count_events`: Calls the input routine with -1, stores the result, and increments the left and right event counters when new distinct bits are seen.
- `advance_three_byte_counters`: Increments a byte counter, wrapping it at 60 and bumping a second counter that wraps at 24, which in turn bumps a third counter; restores d2 and returns.
- `update_weapon_sight_by_mode`: Stores a flag and a computed range difference, then branches on the current mode value to call the matching sight-update routine and store a derived screen coordinate.
- `process_keyboard_input_and_steering`: Reads the key bitmasks, compares them against the previous state, and dispatches mode changes, throttle and rudder adjustments, and steering updates through the called routines.
- `random_heading_change_timer`: Increments a frame counter, and once it exceeds 3000 and two flags are set, calls a random routine to pick a new heading value and trigger a sound, then clears the counter.
- `compute_angle_screen_offset`: Takes two signed coordinates, shifts the second by eight, computes an angle via a helper, clamps it, and maps the quadrant to a screen offset value returned in d0.
- `draw_string_advancing_x`: Walks a null-terminated byte string, calling a draw routine for each character with x advanced by seven pixels, using the passed x, y and string pointer.
- `draw_string_with_highlight_override`: Copies 48 bytes of a global table to the stack, then for each character of the input string calls the draw routine at x step 7, substituting the first table byte when the character matches it or the flag is zero.
- `format_number_into_global_buffer`: Validates a digit count and value against a table, writes a sign or space plus digits into the global buffer by repeated division by ten, then calls the draw routine with the given coordinates.
- `draw_centered_text_at_fixed_position`: Pushes fixed arguments 200 and 319 plus three zero values onto the stack, calls the draw routine, then discards the arguments and returns.
- `load_picture_pages_and_copy_palette`: Calls the picture loader for several named images, copies 97 bytes from the palette buffer into two palette areas, and blits four 4000-byte strips from each source page into the screen buffer.
- `switch_picture_page_and_reset_ship_state`: Selects a picture page by argument, blits page strips into the screen, clears the ship position and speed words, sets the heading and status flags, and resets the 24 ship slots.
- `blit_four_strips_from_page_to_screen`: Takes a source page pointer from the stack and blits four 4000-byte strips into the screen buffer at 4000-byte offsets, then calls the screen update and returns.
- `init_game_state_and_clear_tables`: Saves d7/a2-a3, zeroes many game state words, sets initial counters and limits, clears four and ten and thirteen entry tables, copies selected ship data into the shared table, and calls three setup routines.
- `animate_radar_sweep_and_blips`: Increments a frame counter modulo twenty-one, draws the rotating radar sweep line, plots each tracked contact as a small box, and triggers sonar ping graphics at specific frames.
- `advance_radar_sweep_angle`: Copies the current sweep angle to the previous angle, then either steps the angle through a twenty-six entry table or reads a scripted angle, storing the angle times forty as the sweep position.
- `draw_hud_gauges_and_ship_list`: Calls the display setup, then loops over thirteen six-byte ship entries drawing each with a flag, and draws digit pairs for speed, depth and heading values at fixed screen positions.
- `main_game_loop_with_input_and_speedup`: Runs the main simulation loop: reads joystick and keyboard bits, toggles pause and speed-up modes, draws the cockpit and horizon, and exits when a quit key or collision condition is met.
- `end_speedup_and_reset_game_state`: Optionally prints the end-of-speed-up message, restores real-time piloting, resets the score and timer words, sets the game mode to nine and clears the speed-up flag.
- `init_ship_status_panels`: Pushes five zeroed parameter blocks and one stored value through a display setup call, then writes four stored values into panel slots with fixed sizes and offsets, cleaning the stack.
- `wait_for_sound_channel_free`: Tests the longword at the sound channel slot, and while it is nonzero or zero calls the sound routine with argument 255 and stores the returned longword back into that slot.
- `set_sprite_position_and_redraw`: Saves d5-d7, loads three word arguments from the stack into d7, d6 and d5, stores them into sprite fields, sets a flag byte, then calls two drawing routines on the sprite structure.
- `draw_ship_sprite_pair`: Saves registers, reads four word arguments from the stack, calls a rectangle fill for the first sprite, writes sprite position and control words to chip memory, then draws and frees two sprite frames before filling a second rectangle.
- `draw_ship_sprite_with_offset`: Sets up a stack frame, reads three word arguments, computes a combined height, calls a rectangle fill with offset coordinates, writes sprite position and control words to chip memory, then draws and frees one sprite frame.
- `update_entity_sprite_table`: Loops over twenty-four fourteen-byte entity records, checks state and type fields, dispatches to sprite drawing routines for types four, five, twelve and two hundred fifty-five, then advances or resets each record's animation counter.
- `draw_press_a_key_prompt_screen`: Sets up display and palette via library calls, draws the press-a-key text and three parameter strings, optionally plays a sound, swaps two global pointers, then restores and returns.
- `draw_two_string_prompt_screen`: Saves a2-a3, sets display and palette, draws two caller-supplied strings at fixed positions, plays a sound, then restores registers and returns.
- `select_ship_slot_and_advance_timer`: Pushes d7, reads a word index from global memory, uses it to fetch a long pointer from a table, stores that pointer into a global, calls a routine with it, sets a global word to 45, decrements a countdown word and, when it reaches zero, copies nine words from one global table to another before restoring d7 and returning.
- `store_word_in_first_free_slot`: Saves d6-d7, reads a word argument from the stack, scans nine word slots for a zero entry, writes the argument there, and sets a status word to 2 if another flag is zero, returning that flag.
- `draw_rotating_ship_sprite_list`: Increments a global counter modulo 30, then walks a list of sprite records at a global table, pushing each record's fields and a selector onto the stack and calling a drawing routine, advancing the index until a terminator value of 999 is reached, then restores registers and returns.
- `call_routine_repeatedly_count_times`: Saves d6-d7, reads a count from the stack, loops calling a subroutine that many times, then restores registers and returns.
- `dispatch_a3_with_global_and_two_calls`: Saves a3, loads the first stack argument into a3, calls three subroutines passing the global at 23a2 and a3, then restores a3 and returns.
- `call_2908_with_global_239e_and_arg`: Saves a3, loads the stack argument into a3, pushes the global at 239e and a3, calls the subroutine at 2908, then restores a3 and returns.
- `call_2908_with_two_stack_args`: Sets up an a5 frame, saves d7 and a3, loads the two stack arguments into a3 and d7, calls the subroutine at 2908 with both, then restores registers and returns.
- `set_sprite_control_words_zero_319_199`: Writes zero to the two control words at 540 and 542, then writes 319 and 199 to the following two words, and returns.
- `set_sprite_control_words_zero_7_319_93`: Clears the word at 540, writes 7 to 542, then writes 319 and 93 to the next two words, and returns.
- `set_sprite_control_words_32_3_287_93`: Writes 32 to the word at 540, 3 to 542, then 287 and 93 to the following two words, and returns.
- `set_palette_entry_40_9_ff_5b`: Writes four words into the custom chip registers at 540, 542, 544 and 546, setting values 64, 9, 255 and 91, then returns.
- `set_palette_entry_20_3_11f_5d`: Writes four words into the custom chip registers at 540, 542, 544 and 546, setting values 32, 3, 287 and 93, then returns.
- `read_error_retry_dialog_loop`: Draws the read error retry screen, shows the F1 retry text, then repeatedly calls the input routine until it returns the value 8192.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `show_disk_error_message`: Calls a setup routine, pushes the disk error string and parameters, calls the message display routine, then restores the stack and returns.
- `spawn_radar_blip_from_target`: Computes relative target position, wraps the angle, projects coordinates, and appends a blip entry to the radar table when within range.
- `update_radar_blip_positions`: Loops over a count based on the difficulty setting, advancing each radar blip's position or respawning it at a random offset.
- `update_active_entities_and_collisions`: Iterates the entity table, computes offsets from the camera, tests proximity, triggers sound effects, and dispatches per-entity update routines based on distance and type.
- `scale_two_vectors_by_sine_cosine`: Calls the sine routine with the angle, multiplies two pointed-to values by the results, then shifts both products right by fourteen bits and stores them back.
- `draw_radar_blips_for_contacts`: Computes relative bearings and ranges, looks up scaled sine values, and writes radar blip records into the display list buffer at the current pointer.
- `update_depth_charge_target_and_trigger`: Computes a projected screen position from the object's coordinates and camera offset, calls the sprite draw routine, and for object type sixteen clamps a depth value into the global limit before possibly triggering a sound.
- `update_ship_weapons_and_state`: Walks a ship's weapon table, computes screen offsets from global scroll values, calls a draw routine for each active weapon, then updates the ship's state words, timers and random-driven flags based on counters and global values before returning.
- `draw_ship_radar_blips`: Walks the ship's blip list, computes each blip's screen position from the ship heading and offsets, clamps the vertical value, and calls the blip drawing routine for each entry.
- `spawn_objects_from_ship_slot_table`: Reads a ship's slot table, checks coordinates and game state, then for each active slot computes offsets and calls the object spawn routine with position and type.
- `spawn_sonar_ping_and_update_min_range`: Sets a sonar mode word, calls the spawn routine with fixed parameters, converts the returned distance to a range value and stores it if smaller than the current minimum.
- `spawn_object_with_screen_offset`: Computes a vertical screen offset from the global scroll value plus a caller-supplied delta, then calls the spawn routine with the two coordinates and two word parameters.
- `spawn_smoke_particle_at_ship`: Computes a screen position from the ship coordinates, checks it against a sine table, and if within range writes a new 18-byte particle record into the particle array and increments the particle counters.
- `check_collisions_and_spawn_effects`: Loops over six object records comparing their positions against the ship, triggers explosion sound and effect when close, and updates the object state fields and score counters.
- `reset_ship_position_and_score`: When passed one, copies the saved start coordinates into the current position, clears the speed and score counters, and sets the game state word; otherwise sets the state word to two.
- `display_mission_result_messages`: Reads the mission result code in memory, draws the corresponding status panels and prints three text lines for each outcome, then clears the result and display flags.
- `draw_radar_sweep_blips`: Builds five radar blip bitmasks from the ship position and map bytes, masks them against the sweep, and plots each set bit as a pixel offset relative to the player.
- `pick_highest_scoring_target`: Scans the twenty-byte target table for the entry with the largest score field, calls the targeting routine for that index and marks its score as consumed.
- `find_nearest_waypoint_index`: Scans a null-terminated array of coordinate pairs, computes scaled absolute distances to two global reference points, and returns the index of the closest point matching the requested comparison mode.
- `update_entity_positions_and_collisions`: Advances frame counters, iterates entity records, moves each along its path, computes distances to the player, and adjusts heading and speed based on proximity and collision checks.
- `spawn_random_pickup_objects`: Draws random numbers to occasionally place fuel, ammo or repair pickups at random screen positions, marks their table slots active, and triggers the spawn sound.
- `decrement_sonar_timers_and_play_sounds`: Decrements three countdown words in d4 memory, and when each reaches zero clears a related flag and plays a sound effect via the sound routine.
- `select_engine_sound_from_status_flags`: Checks groups of three status words in d1 memory and writes a sound index into the d4 engine-sound word when all three are nonzero.
- `update_sonar_sweep_and_engine_noise`: Advances a 200-step sweep counter, updates sonar position vectors, decays a distance value, toggles engine noise level and periodically randomizes a timer.
- `spawn_depth_charge_slot`: Scans six 28-byte slots for a free one, initialises its position, velocity and fuse from the player ship, then arms the depth charge and updates the slot counters.
- `update_launched_depth_charge`: Advances the active depth charge's horizontal drift and vertical fall, decrements its fuse, and calls the sprite and collision routines until it expires or hits.
- `update_depth_charge_slots`: Decrements the four depth-charge counters, arms the current slot when free, then moves each active charge, applies gravity and spawns explosion sprites on expiry.
- `spawn_ship_from_free_slot`: Scans six 22-byte ship slots for an empty one, then initialises position, velocity, timer and four weapon flags and calls the sprite setup routine.
- `advance_ship_timers_and_motion`: Increments two countdown timers, resets weapon flags when they expire, then moves each of six active ships by its velocity and updates or explodes them.
- `select_target_and_build_ship_record`: Scans six ship records for a type-two entry, compares target tables, then fills a 20-byte record with type, coordinates and offsets and calls the update routine.
- `append_ship_record_to_table`: Copies a 20-byte ship record selected by the stack argument into the next slot of the record table, stores a pointer, clears a flag word, and increments two counters.
- `reset_base_state_on_arrival`: When the arrival flag is clear, resets position and speed globals, clears thirteen six-byte entries, calls a setup routine, and shows base text or sets a stop state.
- `build_contact_list_from_tables`: Reads ship and contact tables indexed by two arguments, fills display globals, and copies contact records into an output buffer until a terminator or random selection.
- `check_mission_end_conditions`: Reads the mission state word, dispatches through a jump table, and when the mission timer and ship counts match the win or loss conditions calls the message display routine with the appropriate text.
- `run_menu_input_loop`: Sets up the menu screen, polls the mouse and keyboard each frame, updates the selected menu entry, and draws the highlighted item until the menu is dismissed.
- `init_menu_selection_flags`: Clears d0 and writes zero to the word at address 8, then writes one to the words at addresses 10, 12, 14 and 16, and zero to addresses 18 and 20.
- `run_menu_sequence_and_restore_state`: Sets a status word, calls many subroutines to draw the menu and pictures, plays sounds, then clears the status word and returns.
- `adjust_column_with_wrap_and_redraw`: Reads two stack arguments, optionally redraws a screen region when direction bits are set, and returns the column index wrapped within the row width.
- `wrap_row_index_to_zero`: Saves d7 and a2, loads a word argument into d7, and if it equals one replaces it with zero before branching onward.
- `call_audio_with_ship_offsets`: Saves d0 in d7, loads a base pointer from a global, computes two offsets into it, pushes five arguments and calls a subroutine, then restores d7 and returns d0.
- `wait_for_input_then_set_flag`: Calls several subroutines, polls a status longword until nonzero, then loops up to 500 times calling a routine and checking input, setting a flag word and returning.
- `draw_ship_hud_and_wait_input`: Saves registers, sets up display and palette calls, computes screen coordinates from position globals, draws text and graphics, polls input in a loop, then restores registers and returns.
- `draw_radar_sweep_and_blips`: Scans the object table for the radar origin marker, draws the radar grid and sweep lines, then plots each visible contact and waits for a key between contacts.
- `move_radar_cursor_to_target`: Steps the radar cursor position stored in d8 globals toward the requested coordinates in eight-pixel increments, redraws the cursor, then waits for six key presses.
- `render_radar_contacts_and_cursor`: Stores the requested cursor coordinates, draws the radar background and range rings, plots each active contact and the player marker, then restores the display.
- `run_mission_phase_sequence`: Runs a scripted sequence of mission phases, setting display and position globals, calling phase helpers, and aborting each loop when the skip bit is set.
- `per_frame_game_update`: Advances frame counters, computes screen offsets and scroll values, calls the per-frame update routines, and handles the end-of-mission transition.
- `show_mission_briefing_screen`: Loads the futura font, draws the briefing screen, loops eleven times drawing text lines, then waits for input and returns.
- `copy_36c_longs_from_229c_to_4c`: Loads the pointer at absolute address 229c into a0 and the immediate address 4c into a1, then copies 36d longs from (a0)+ to (a1)+ using a dbra loop over d0.
- `blit_sprite_rows_into_bitplanes`: Stores three arguments into globals, adjusts the row offset, then loops six times reading masked sprite data and ORing it into three bitplane buffers at offsets 8000, 16000 and 24000.
- `save_stack_set_vbr_and_run_init_chain`: Saves all registers, sets the VBR to 6ee and stores the stack pointer at 13a0, then calls five initialisation routines in sequence before branching to the restore path.
- `load_ship_state_from_stream`: Reads a parameter block through a0, stores count and offsets into globals, sets flag bits for nonzero fields, copies a 16-byte record into the ship state, advances the stream pointer by 18 bytes and returns.
- `update_ship_vertices_by_heading`: Checks a flag bit and a mode word, then either adds position offsets to each vertex or rotates vertices by sine and cosine into the three coordinate buffers, returning after the loop.
- `init_three_copper_quad_words`: Writes the value 16384 and zero words to the three 8-byte blocks starting at address 956, setting up three copper list entries, then returns.
- `compute_rotation_matrix_entries`: Calls the sine routine for the angle in a global, then fills six global words with rotated sine and cosine products scaled by four and swapped into the high word.
- `build_rotated_quad_offsets_from_angle`: Calls the angle routine, then multiplies the returned sine and cosine by two scale factors, shifts and swaps to store four rotated corner offsets and two raw values into the d10 buffer, then copies five longs to the d9 output area.
- `build_axis_aligned_quad_offsets`: Calls the angle routine, then stores the raw sine and cosine as unrotated corner offsets, zeroes the other four offset words, copies the previous depth value, and moves five longs into the d9 output area.
- `transform_vertices_by_rotation_matrix`: Loops over vertex count, reads three coordinates from a1, optionally applies a 3x3 matrix at a6 with shifts, adds offsets from a5, and writes results to a2, a3 and a4.
- `project_vertices_with_perspective_scale`: Loops over vertex count, clamps depth from a3, computes a perspective scale via division, then writes scaled x and y coordinates to a4 and a5 using offsets d4 and d5.
- `cull_and_average_polygon_vertices`: Loops over polygon records, tests edge orientation against projected coordinates to cull back-facing polygons, averages vertex depths and stores depth and pointer pairs to a6.
- `insertion_sort_eight_byte_records`: Sorts the eight-byte records in the d9 table by their first long in descending order using insertion sort, saving each record's two longs in scratch memory while shifting larger records up.
- `build_edge_table_from_ship_records`: Copies each ship record's edge list into a working buffer, looks up a value in a table indexed by the record's first word, stores it and the edge count in globals, then calls the edge sorter.
- `load_edge_list_from_stack_args`: Reads two words and a list of edge pairs from the stack, stores the two words in globals, copies the pairs into the working buffer, then calls the edge sorter.
- `sort_edges_and_draw_scanlines`: Copies the edge buffer, appends a sentinel, finds the topmost edge, then walks the sorted edges calling the fill routines to draw the polygon scanlines.
- `fill_span_keep_minimum`: Draws a vertical span from d4 between a5 and a6 into the buffer at a3, stepping d3 by the slope in d5 and keeping the smaller value at each row.
- `fill_span_keep_maximum`: Draws a vertical span from d4 between a5 and a6 into the buffer at a3, stepping d3 by the slope in d5 and keeping the larger value at each row.
- `blit_masked_span_into_bitplanes`: Clips a horizontal span between the stored min and max x limits, then sets or clears masked bits across four bitplanes at the computed plane offset, advancing by the plane stride.
- `lookup_pair_from_sine_table`: Tests d0, adds 1280 if negative, then indexes a word table at a2 by d0 to load d1; wraps d2 by subtracting 960 or adding 320, doubles it and loads d2 from the table.
- `fetch_two_table_words_via_stack_args`: Loads d0 and two pointer arguments from the stack, calls the table lookup, sign-extends d1 and d2, and stores each result through the supplied pointers.
- `decode_compressed_value_to_word`: Reads a longword argument from the stack, scans its bytes from high to low, calls the nibble classifier and bit decoder, and returns the assembled word in d0 while restoring d2-d7 and a0-a7.
- `classify_value_into_nibble_pair`: Compares d4 against a ladder of thresholds and sets d2 to a nibble index and d3 to the matching lower bound, then returns to the caller.
- `refine_nibble_index_by_range`: Uses d2 shifted left by four as a base and compares d4 against scaled sub-ranges to select the exact nibble value, returning it in d6.
- `binary_search_sorted_word_table`: Reads a signed word argument from the stack and binary-searches a sorted word table, halving the index range over six steps, then scans forward until a table entry is not less than the argument and returns the entry index in d0.
- `init_game_state_and_forbid_exec`: Saves registers, calls two setup routines, sets the CIAA port A bit 1, stores two pointers derived from a base address into globals, calls an init routine, and if it returns zero calls another routine and the exec library vector at offset minus 132.
- `read_input_flag_into_global`: Clears the word at address 668, calls a routine, then if the longword at address 8898 is nonzero copies it to d0 and sets bit 0 of the word at address 668 from its bit 4, returning d0.
- `call_2ae4_with_caller_return_address`: Loads the caller's return address from the stack into a1, calls the routine at absolute address 2ae4, then returns to the caller.
- `store_229c_pointer_and_call_2a16`: Loads the caller's return address into a1, writes the immediate pointer value 229c to absolute memory 2a0c, calls the routine at 2a16, then returns.
- `seed_and_advance_vposr_random_state`: Seeds a global from VPOSR if zero, multiplies the state by a constant via a called routine, increments it, and returns bits 8 to 31 of the result.
- `data_byte_ori_dd_d0`: Contains a single ori.b immediate instruction that ORs the byte value 221 into d0; the bytes are data decoded as code, not an executable routine.
- `signed_16bit_multiply_with_sign_fix`: Multiplies two signed 32-bit arguments from the stack using 16-bit mulu partial products, tracks signs in d2, negates the result if needed, and returns it in d0.
- `wait_for_copper_bit_toggle`: Polls bit 0 of the custom chip register at offset 5 until it is set, then polls again until it is clear, then returns.
- `wait_for_vposr_line_window`: Polls the VPOSR hardware register until the vertical position field falls within a range, then waits for it to drop by 256 before returning.
- `read_beam_position_and_flag_range`: Calls the beam position reader, swaps the result into the high word, compares it against the range 6528 to 6624 and returns zero or one in d0.
- `clear_buffer_and_read_beam_position`: Fills 50 bytes at the pointer in memory with 255, saves registers, then calls the beam reader and stores its result in memory.
- `increment_counter_store_a0_and_call_six`: Increments the long counter at d47e, doubles a1, stores a0 at d46e, calls six subroutines, then moves d6 to d0 and returns.
- `init_copper_and_sound_registers`: Sets a6 to the custom chip base, copies display and audio register values into globals, programs four audio registers, writes bytes to a1 offset 4352 and increments a counter word.
- `clear_audio_bits_and_delay_loop`: Clears bits 7 and 3 of the byte at a1+4352, spins a hundred-iteration dbra delay, then calls two subroutines and returns.
- `delay_3000_iterations`: Loads 3000 into d6 and decrements it with dbra until it reaches minus one, creating a busy-wait delay, then returns.
- `wait_clear_bit5_at_2001`: Repeatedly tests bit 5 of the byte at offset 8193 from a1 and branches back while it is set, returning once the bit clears.
- `toggle_4352_bits_while_bit4_set`: While bit 4 of the byte at offset 8193 from a1 is set, sets and clears bits in the byte at offset 4352, calls the delay routine, repeats, then increments a counter word and returns.
- `wait_audio_flag_toggle_port_bit`: Polls bit 4 of the byte at offset 8193 from a1, calling a wait routine while set, then toggles bits 0 and 1 of the byte at offset 4352 from a1 seventy-nine times with short delays, then busy-waits 15000 iterations.
- `reset_audio_and_poll_status_bit`: Calls the port toggle routine, clears bit 2 at offset 4352 from a1, programs audio registers at offsets 150, 32, 36 and 156 from a6, then polls bit 1 of the word at offset 30 from a6 up to an address times, setting offset 156 to 2 on detection.
- `clear_audio_period_set_volume_ten`: Clears the word at offset 36 from a6, sets the word at offset 150 from a6 to 16, increments the longword at the fixed global address d47e, and returns.
- `wait_for_ay_port_bit4_clear`: Reads byte at offset 7424 of a1 repeatedly until bit 4 is set, then returns to the caller.
- `set_ay_register_high_bits_and_ports`: Sets and clears bits in the byte at offset 4352 of a1, then writes four table words with bit 15 set into a6 offsets 154, 156, 150 and 158.
- `search_sync_marker`: Iterates through a list of words starting at a pointer in a0, shifting each 32-bit value left up to 15 times to find the word 4489, calls another routine if found, and updates a global byte and a counter based on the search result.
- `scan_words_for_sync_marker`: Reads words from (a0), shifting each 32-bit value left up to 15 times to test for the sync word 4489, stores the bit index in (a4) and sets a flag byte when found, or clears the flag after d3 words.
- `zero_filled_data_byte`: A single zero word that decodes as ori.b #0,d0; this byte is zero-filled data referenced by address, not executable code.
- `zero_filled_flag_byte`: A single zero word that decodes as ori.b #0,d0; this byte is zero-filled data used as a flag by other code, not executable code.
- `zero_filled_data_area_2`: Three zero words that decode as ori.b instructions; this range is zero-filled data referenced by address, not executable code.
- `set_copper_list_four_registers`: Reads four stack arguments, masks the first two to multiples of sixteen, adds fifteen to the second, and writes them to the custom chip registers at dc90, dc92, dc94 and dc96.
- `install_copper_list_and_run`: Saves the old copper list pointer, installs a new list at dc88 with zero at dc8c, calls a setup routine, restores the old pointer, then calls another routine with registers d2-d7 and a2-a6 preserved.
- `iterate_list_call_each_entry`: Saves the incoming pointer to a global, pushes all registers, then walks a null-terminated list of pointers, calling the routine at each entry and advancing by four bytes.
- `run_six_stage_update_sequence`: Pushes all registers, calls six fixed subroutines in sequence, then restores all registers and returns to the caller.
- `load_ship_state_from_record`: Reads a record pointed to by a1, stores flag bits, dimensions and four copied longs into globals, and returns with a0 and a1 repointed.
- `initialize_memory_buffer`: Checks a global word and, if non-zero, enters a loop that writes a pointer to a global variable into a destination buffer and then clears subsequent word-sized entries in that buffer based on a value derived from a global word.
- `expand_16_bytes_through_lookup_table`: Reads 16 bytes from (a5), translates each through the table at a4, writes them backwards from a3, then advances a5 by three and a3 by d7, looping via d2 and d4.
- `blit_water_columns_to_bitplanes`: Computes a blit size from two globals, waits for the blitter, then programs blitter pointers, modulo, control and data registers to draw two column strips into the bitplanes.
- `adjust_playfield_scroll_and_tile_offsets`: Compares d1 and d0 against playfield bounds, updates scroll offsets and tile pointers in memory, adjusts d0 and d1, and returns or jumps to the caller's error path.
- `compute_tilemap_row_and_blit_offsets`: Builds a row offset from the playfield scroll values into memory, derives d0 and d1 from the input coordinates, and returns the computed blit modulo in d3.
- `blit_playfield_tiles_with_copper_list`: Reorders four longwords into the copper list, calls the copper setup, programs blitter pointers, modulos, size and control registers, repeats four times, then disables blitter DMA.
- `wait_dmaconr_bit6_clear`: Executes two no-ops, then repeatedly tests bit 6 of the DMACONR register at the custom chip base until it is clear, then returns.
- `reset_audio_and_call_init`: Saves all registers, calls the audio initialization routine, then writes zero to the audio period and volume registers and sets the audio control and modulation registers before restoring registers and returning.
- `fill_bit_reverse_table`: Builds a 256-byte table at the data address in a0, storing the bit-reversed value of each byte index from zero through 255, then returns.
- `store_return_address_to_dc84`: Copies the longword return address at offset 4 of the stack into the global longword at dc84, then falls through into the following routine.
- `build_blitter_sprite_entries`: Walks the sprite list at dc84, reads dimensions and flags from each sprite header, copies pixel data into a buffer, programs the blitter registers and stores blit pointers back into the sprite entries.
- `rle_decode_sprite_rows`: Decodes run-length encoded bytes from a4 into a5 using d6 and d7 as repeat markers, writing zero pairs or literal bytes until d3 pixels are produced.
- `unpack_bits_to_bytes_with_offset_table`: Reads a bit mask byte from (a4), and for each set bit copies a byte from a table at a4 plus the offset in d7 to (a5), otherwise writes zero, repeating d3+1 times.
- `relocate_pointer_table_by_2048`: Walks a pointer table at the address stored in the global, adds the table base plus 2048 to each entry and to four following longs, then writes a zero terminator.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `bounded_string_copy_append`: Measures the lengths of two strings, appends up to the given maximum bytes of the second onto the end of the first, null-terminates it and returns the first string pointer in d0.
- `open_file_and_append_data`: Saves registers, opens the named file via a helper, then writes the given buffer and length through another helper, returning minus one on failure or if the global error flag at offset 216 of a4 is set.
- `flush_pending_buffers_in_list`: Walks the linked list at offset 4 of a4, and for each node whose flags at offset 27 have bit 1 set but bit 2 clear and whose length differs from its position, writes the pending bytes, then flushes the passed buffer.
- `write_buffer_via_dos_and_record_error`: Optionally flushes pending output, clears the error flag at offset 216 of a4, calls dos.Write with the given file, buffer and length, and on failure stores dos.IoErr and sets the retry counter at offset 588 to five.
- `seek_file_with_mode_and_offset`: Saves d2-d7, reads three stack arguments, optionally calls a cleanup routine, then calls dos.Seek with a mode and offset, storing the result or error in globals and returning the computed position.
- `close_file_handle_and_return_zero`: Saves d7, loads the file handle from the stack, optionally calls a cleanup routine, then calls dos.Close on the handle and returns zero in d0.
- `unsigned_32bit_multiply_d0_by_d1`: Multiplies the 32-bit values in d0 and d1 using partial 16-bit products, leaves the low 32-bit product in d0, and restores d2 and d3.
- `signed_divide_d0_by_d1`: Tests and negates d0 and d1 to make them positive, calls the unsigned division routine, then negates the quotient and remainder to restore the correct signs before returning.
- `unsigned_32bit_divide_d0_by_d1`: Divides the 32-bit value in d0 by the 32-bit value in d1 using shift-and-subtract normalization, returning the quotient in d0 and remainder in d1 while preserving d2 and d3.
- `lookup_indexed_entry_pointer`: Clears the error field at offset 216 of a4, validates the index in d7 against the count at a4, and returns a pointer to the eight-byte entry at offset 268 or sets error code 9.
- `free_command_list_nodes`: Walks the linked list at offset 596 of a4, freeing each node's payload via exec.FreeMem, then clears the list head and the count at offset 600.
- `lookup_and_run_command_entry`: Looks up a command by name, optionally resets it, then invokes its handler with the given arguments and returns minus one on failure or if a flag at offset 216 of a4 is set.
- `parse_command_line_and_open_console`: Tokenizes the argument string into up to 32 pointers, then opens a console window and sets up input and output file handles and signal bits for the command.
- `copy_d0_to_d5`: Copies the longword in d0 into d5, then falls through into the following routine.
- `jump_to_absolute_1c0`: Jumps to the absolute address stored at location 448, transferring control without saving a return address.
- `call_library_offset_neg348`: Saves d2-d3 and a2-a3/a6, loads a6 from the stack, copies six stack arguments into a0-a3 and d0-d3, calls the library vector at offset minus 348 from a6, restores registers and returns.
- `release_objects_above_index`: Saves d5-d7, reads a start index from the stack, then walks object slots downward in the a4 table, calling the release routine for each live non-flagged entry, and finally calls the cleanup routine with the saved argument.
- `jump_through_global_vector`: Performs a single jump to the code pointer stored at offset 332 in the a4-relative global area, transferring control without saving registers or returning.
- `clamp_byte_to_79_and_setup_buffer`: Builds an a5 stack frame, saves d2-d3/d6-d7/a6, reads a byte before the a4 buffer pointer into d7, clamps it to at most 79, and loads a1 with a local buffer address.
- `copy_break_message_and_write_to_output`: Copies d0 bytes from a0 to a1, clears a byte in the a5 frame, finds the current task, and writes a break message plus newline to the task's output stream, returning -1.
- `open_intuition_library_store_base`: Loads the exec library base, opens intuition.library with version zero, stores the base in the a5 frame, and returns -1 if opening failed.
- `run_simulation_with_break_arguments`: Stores the break buffer pointer in the a4 structure and pushes nine arguments onto the stack before calling the simulation routine, returning zero on success or -1 on failure.
- `return_zero_and_restore_registers`: Sets d0 to zero as the return value, pops d2-d3/d6-d7/a6 from the stack, releases the a5 frame and returns.
- `poll_signal_and_run_pending_callback`: Calls exec SetSignal for signal mask 12288, tests the returned bits and a callback pointer at offset 240 of a4, invokes it, clears it on success and calls a cleanup routine with argument 20.
- `blitter_fill_rect_from_stack_args`: Reads five rectangle arguments from the stack, computes the destination offset in the bitmap, derives blit direction and size, then programs the blitter registers and starts the fill.
- `draw_vertical_line_span`: Saves four arguments and the current clip rectangle, sets the clip to the full screen, then repeatedly calls the line drawer with fixed x values while stepping y from the start to the end, finally restoring the clip rectangle.
- `draw_hollow_rectangle_outline`: Saves arguments and clip rectangle, shrinks the clip to full screen, draws the left, top and right edges of a rectangle by calling the line drawer, then draws the bottom edge and restores the clip.
- `blit_tilemap_to_three_planes`: For each of four tile rows and sixty tile columns, reads a tile index byte, looks up three color bytes in a table and writes them into three bitplanes at offsets zero, forty and eighty bytes.
- `toggle_flag_and_branch_if_clear`: Exclusive-ORs the byte at memory location 5492 with 255, then branches to the routine at 5544 if the result is zero.
- `set_flag_and_copy_pointer_pair`: Writes 255 to the byte at 5492, loads address 8860 into a1, copies the longword at 8876 to 9122 and the longword at 8860 to 9118, then branches to 5576.
- `clear_flag_and_copy_four_pairs`: Clears the byte at 5492, copies two longwords between fixed memory locations, then copies four word pairs from 8876 into the buffer pointed to by 10492, advancing eight bytes each time.
- `blit_ship_sprite_rows_to_bitplanes`: Computes source and destination offsets from row and column arguments, sets up four Blitter passes with modulo and control registers, and blits each row of the sprite into the bitplanes.
- `copy_longwords_count_bytes`: Copies a block of memory from the source pointer to the destination pointer, moving longwords in a loop for the given byte count minus one, then returns.
- `subtract_speed_delta_from_sprite_planes`: Reads a sprite structure pointer and two speed words, computes their difference, and subtracts that byte value from eight plane pointers at fixed offsets.
- `render_radar_sweep_and_clear_bitplanes`: Saves all registers, computes sweep position from the frame counter, draws radar blips via the blitter helper, then fills three bitplane rows with ones and zeros to clear the display.
- `blit_four_rows_with_modulo_and_size`: Waits for the blitter, then programs four successive blit passes using a0 and a1 pointers, d1 offset, d4 modulo, d0 and d2 blitcon values and d5 bltsize, advancing both pointers by 8000 bytes each pass.
- `clear_four_bitplane_rows_via_blitter`: Saves d0 and a0, waits for the blitter, then clears four rows starting at a6 using bltcon0 value 256, bltcon1 zero, a0 as destination pointer and bltsize 3220, advancing a0 by 8000 bytes per pass.
- `draw_radar_sweep_line_and_clear_columns`: Advances the radar sweep angle, draws the sweep line via the blitter helper, then fills two screen column bands with all-ones and all-zeros words.
- `blit_clear_radar_column_bands`: Loops four times setting blitter control, modulo and size registers and clearing a screen column band through the blitter, advancing the destination pointer each pass.
- `scroll_radar_sweep_and_clear_rows`: Advances a scroll offset, computes sweep geometry, draws radar sweep lines via the blitter, then fills two screen row blocks with all-ones and zero words.
- `blit_three_plane_rows_with_d6_offset`: Saves d0/a0, calls a setup routine, then programs BLTCON0, BLTCON1, BLTDMOD, BLTDPTH and BLTSIZE for four blits, advancing a0 by 8000 bytes each time.
- `set_blit_pointers_from_237a_plus_12000`: Loads the longword at 237a, adds 12000, stores it at 1fdc, copies 237a to 1fe0 and 23a2 to 1fe4, then branches onward.
- `set_blit_pointers_from_2372_plus_12000`: Loads the longword at 2372, adds 12000, stores it at 1fdc, copies 2372 to 1fe0 and 23a2 to 1fe4, then branches onward.
- `init_blit_pointers_and_jump`: Loads the base pointer at address 2376, adds 24000 to it, stores the result plus the base and a third pointer into three globals, then branches to another routine.
- `blit_three_banded_rectangles`: Sets up three blitter passes using the stored source and destination pointers, writing blitter control, modulo, pointer and size registers, advancing pointers each pass and calling the blit wait routine.
- `blit_four_line_band_column`: Loads the base pointer plus 1600, runs four blitter passes writing BLTCON0, BLTCON1, modulo, pointer and size registers, advancing the pointer by 8000 each pass, then restores registers.
- `blit_three_band_sprite_rows`: Reads source and destination offsets from the stack frame, computes modulo and blit size, then programs BLTCON0, BLTAPTH, BLTDPTH, BLTAFWM, BLTAMOD, BLTDMOD and BLTSIZE to blit four rows of 40-pixel bands.
- `open_libs_alloc_bitmap_memory`: Opens dos.library and graphics.library via OldOpenLibrary, allocates a large chip memory block with AllocMem, stores the base pointer and derived plane pointers, then calls the setup routine.
- `init_plane_pointer_table`: Copies the allocated bitmap base into a table of plane pointers, adding successive offsets for each bitplane, and stores the bitmap and screen base pointers into two more globals.
- `close_libs_free_mem_remintserver`: Loads exec base into a6, calls CloseLibrary on two library bases with values 3 and 5, frees two memory blocks of sizes 350 and an address, then jumps to RemIntServer for the interrupt server at the last pointer.
- `stop_copper_dma_and_close_library`: Saves registers, writes the copper list pointer to COP1LCH, strobes COPJMP1, disables DMA via DMACON with bit 15 set, closes a library through exec, restores registers and returns zero in d0.
- `build_copper_list_and_enable_display`: Copies and offsets copper list entries, fills sprite and bitplane pointers, copies 500 bytes to the copper buffer, programs COP1LCH, DIWSTRT, DIWSTOP, DDFSTRT, DDFSTOP, BPLCON0 and BPL1MOD/BPL2MOD, then enables DMA and returns.
- `alloc_two_msgports_and_start_tasks`: Opens exec.library, allocates 350 bytes of chip memory, stores two message-port pointers, fills port fields and starts two tasks with entry pointers.
- `clear_d0_after_timer_tick`: Calls the countdown routine at the next address, then sets d0 to zero and returns to its caller.
- `decrement_three_display_timers`: For each of three countdown words, decrements it and when it reaches zero clears its flag and writes a value and word 10 into the pointed structure.
- `open_read_close_file_with_retry`: Opens a file by name with mode 1005, reads up to an address bytes into a buffer, closes it, and retries via a helper if the open fails.
- `read_joystick_port_and_fire`: Reads JOY1DAT and CIAA_PRA to build a four-bit direction and fire value in d0, and sets a flag word when the fire button is released.
- `disable_dma_clear_copper_and_audio`: Disables DMA, writes to the copper list pointer, re-enables DMA, calls a sound routine, then zeroes sixteen audio registers at the custom chip base.
- `set_color08_color09_and_call`: Calls a helper routine, then writes the values 3001 and 3820 to the COLOR08 and COLOR09 hardware registers before returning.
- `decode_bod_y_planar_bitmap`: Saves all registers, scans a1 for the BODY marker, then expands run-length encoded bytes into 40-byte planar rows across d7 rows, writing to the bitmap pointer table.
- `load_palette_from_a1_into_color00`: Saves registers, reads a count from a1 offset 44, divides by three, then converts three-byte color entries into words and writes them to COLOR00 onward.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `adjust_palette_nibbles_toward_target`: Loops over 134 bytes, comparing each byte's low and high nibbles with a target table and incrementing or decrementing them one step toward the target, writing results back to the first table.
- `build_copper_list_from_shape_table`: Reads a shape entry selected by the argument, disables DMA via DMACON, writes a copper list of move instructions into the buffer at the global pointer, terminates it, then re-enables DMA.
- `reset_copper_list_and_restart_dma`: Saves all registers, disables DMA through DMACON, writes a terminator to the copper list pointer, re-enables DMA, calls the sound routine, then restores registers and returns.
- `dispatch_audio_handler_via_stack_arg`: Loads the longword at offset four from the stack into d0, branches to a shared tail, then calls two audio setup routines before returning.
- `init_four_audio_channels_and_dma`: Enables DMA, loops over four channel pointers writing volume, period and a computed divisor into each, then sets ADKCON and returns zero in d1.
- `lookup_period_from_index_table`: Doubles the word in d0, reads a period value from the table at a0 indexed by that offset, stores it to the shared period variable and returns.
- `update_four_channel_thresholds`: Reads flags and a value from the caller's stack frame, updates four global minimum thresholds and calls the channel setup routine for each enabled channel, returning the last channel result.
- `setup_audio_channel_from_struct`: Enables a DMA audio channel, stores a sample pointer and length into the channel structure, computes the period from the sample length, and writes the volume and DMA control registers.

### echo

- `allocate_and_init_audio_sample`: Calls library routines to find the highest free memory, allocate a block, fill it with sample header words, copy a structure, and return the sample pointer or minus one.
- `copy_d1_to_d0_and_branch`: Moves the longword in d1 into d0, then branches unconditionally to another routine at a fixed offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_to_runtime_entry_point`: Unconditionally jumps to the runtime segment entry point, transferring execution without saving registers or returning to the caller.
- `undecodable_bytes_8`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_9`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_10`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_11`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_12`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_13`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_14`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_15`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_16`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_to_shared_entry_one`: Performs an unconditional jump to a shared code entry point, transferring execution without touching any registers or memory.
- `jump_to_shared_entry_two`: Performs an unconditional jump to a second shared code entry point, passing control onward without modifying registers or memory.
- `jump_to_shared_entry_three`: Performs an unconditional jump to a third shared code entry point, handing off execution with no register or memory changes.
- `undecodable_bytes_17`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_18`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_19`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_to_shared_runtime_entry`: Performs an unconditional jump to another routine in the runtime segment, transferring execution without touching registers or memory.
- `undecodable_bytes_20`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_to_unreached_runtime_target`: Performs an unconditional jump to a different routine in the runtime segment, transferring control without modifying registers or memory.
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
- `jump_to_third_runtime_entry`: Performs an unconditional jump to yet another routine in the runtime segment, passing control onward without altering registers or memory.
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
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
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
- `load_ed_into_d0_then_branch`: Loads the immediate value 237 into data register d0, then branches unconditionally to the code at address 4cee.
- `set_d0_to_2_and_branch`: Loads the immediate value 2 into data register d0, then branches unconditionally to another routine at a fixed offset.
- `set_d0_to_66_and_branch`: Loads the immediate value 66 into data register d0, then branches unconditionally to another routine at a fixed offset.
- `set_d0_to_65_and_branch`: Loads the immediate value 65 into data register d0, then branches unconditionally to another routine at a fixed offset.
- `load_minus_six_into_d0_and_branch`: Loads the immediate value minus six into data register d0 and branches unconditionally to a shared routine elsewhere in the segment.
- `load_minus_three_into_d0_and_branch`: Loads the immediate value minus three into data register d0 and branches unconditionally to a shared routine elsewhere in the segment.
- `load_ninety_three_into_d0_and_branch`: Loads the immediate value ninety-three into data register d0 and branches unconditionally to a shared routine elsewhere in the segment.
- `set_d0_ff_and_branch_to_4cee`: Loads the immediate value 255 into data register d0 and then branches unconditionally to the code at 4cee.
- `nop_then_branch_to_4cea`: Executes a no-operation instruction and then branches unconditionally to the code at 4cea.
- `increment_a6_offset_20_and_return_a6`: Skips a no-op, branches to 4cd6, adds 1 to the word at offset 32 of a6, moves a6 into d0 and returns.
- `decrement_counter_and_clear_d0`: Reads the word counter at offset 32 of a6, subtracts one, stores it back, then sets d0 to zero and returns to the caller.
- `jump_vector_entry_32`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_33`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_34`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_35`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `push_five_string_args_and_jump`: Jumps through the absolute pointer at the given address; the following entries each subtract eight from a7, push a string address and branch to a shared continuation.
- `push_string_arg_and_branch`: Subtracts eight from the stack pointer a7, pushes the long address of a string, then branches to the shared continuation that follows the other argument-pushing entries.
- `push_nu_string_and_setup_frame`: Decrements the stack pointer by eight, pushes a pointer to the Nu text, branches ahead, then stores a5 and two pc-relative frame pointers into the new stack frame before returning.
- `pop_a5_from_stack_and_return`: Pops a longword from the stack into address register a5 and returns immediately to the caller, restoring the saved frame pointer.
- `jump_vector_entry_36`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `forbid_exec_then_restore_registers`: Calls two library routines, loads the exec base from absolute address four into a6, calls Forbid through its offset, then restores all registers and returns.
- `clear_sound_flag_and_call_init`: Clears the word at the sound flag address, zeroes d0, calls the sound init routine, then tests the value at the sound state pointer and sets bit 4 of the flag word if that value is nonzero.
- `jump_to_two_library_vectors`: Jumps to the first external library vector, then the second jump instruction transfers control to another external vector; both are tail jumps with no return to this code.
- `wait_for_vposr_match_window`: Reads VPOSR, masks to the vertical position field, and returns only when it equals the passed value minus one hundred, after range-checking that value.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `blitter_dual_pointer_setup`: Programs blitter pointer, modulo and control registers from a0 fields and globals, calls a wait routine, then reprograms a second blit and returns.
- `blitter_loop_advance_source`: Sets blitter control and pointers from globals and registers, calls a wait routine, then advances a1 by 8000 and repeats via dbra d4.
- `jump_vector_entry_37`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `ori_immediate_data_byte`: Single instruction ORs immediate value 30 into d5; it is data reached by an absolute move, not a callable routine.
- `ori_byte_a4_into_d2`: Performs a bitwise OR of the immediate byte value 164 into the low byte of data register d2, then falls through to the following code.
- `ori_byte_24_into_d2_first`: Performs a bitwise OR of the immediate byte value 36 into the low byte of data register d2, then falls through to the next instruction.
- `set_d2_low_byte_bits`: ORs the immediate value into the low byte of d2, setting bits 2 and 5 of that byte; it is a single instruction with no branch or call.
- `set_d2_low_bits_a4`: ORs the immediate byte value 164 into the low byte of data register d2, setting those bits and leaving the rest of d2 unchanged.
- `set_d2_low_bits_24`: ORs the immediate byte value 36 into the low byte of data register d2, setting those bits and leaving the rest of d2 unchanged.
- `or_immediate_a4_into_d2_low_byte`: Performs a bitwise OR of the immediate value 164 into the low byte of data register d2, leaving the upper bytes of d2 unchanged.
- `ori_b_3c_into_d3`: Performs a bitwise OR of the immediate value 60 into the low byte of data register d3, leaving the upper bits unchanged.
- `ori_b_24_into_d3`: Performs a bitwise OR of the immediate value 36 into the low byte of data register d3, leaving the upper bits unchanged.
- `ori_b_2c_into_d4`: Performs a bitwise OR of the immediate value 44 into the low byte of data register d4, leaving the upper bits unchanged.
- `ori_byte_7e_into_d4`: Performs a bitwise OR of the immediate byte value 126 into the low byte of data register d4, leaving the upper bits unchanged.
- `ori_byte_7e_into_d5`: Performs a bitwise OR of the immediate byte value 126 into the low byte of data register d5, leaving the upper bits unchanged.
- `ori_byte_7e_into_d5_again`: Performs a bitwise OR of the immediate byte value 126 into the low byte of data register d5, identical to the earlier entry but reached from a different caller.
- `ori_b_immediate_into_d1`: ORs the immediate byte value 176 into the low byte of data register d1, then falls through to the following code.
- `or_immediate_b0_into_d1_low_byte`: Performs a bitwise OR of the immediate value 176 into the low byte of data register d1, leaving the upper bytes of d1 unchanged.
- `init_audio_channel_pointers_and_dma`: Saves all registers, computes and stores a series of audio buffer pointers, copies two pointer values into the fall-through slots, then programs four audio channel structures and enables DMA.
- `allocate_sound_channel_block`: Calls the system allocator, indexes a pointer table by d0 shifted left twice, stores the resulting pointer to (a2), copies a0's longword fields into it, computes their difference into the block's word field, and sets bit 15 and bit 9 in d1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_11`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_12`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_13`: Empty routine: returns to the caller immediately without doing anything.
