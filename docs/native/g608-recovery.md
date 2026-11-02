# Game 608 (E-Motion): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00608). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904848 top-level bytes in 10 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 9 raw-word lines |
| Functions named and described | pass, 232 of 232 named (0 author symbols), 232 of 232 described (4 by rule, 128 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3825 of 3825 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5630 of 5630 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 328422 |
| code | 18970 |
| data | 45786 |
| padding | 1718 |
| unused | 509952 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| E-Motion.adf:e_motion | 3 | 363 | 16 |
| E-Motion.adf:bootblock | 1 | 10 | 1 |
| logged disk load from E-Motion.adf:mainbit loaded at an address (314356 bytes) | 1 | 4355 | 186 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 382 | 17 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 504 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |

## Cases handled

- E-Motion/E-Motion.adf:mainbit: loaded verbatim at an address and run (reference run on the nv608 harness: the crack loader reads mainbit into an allocated buffer, copies the bytes read to address 400 hex with interrupts off and jumps there through the trainer stub; executed addresses from 400 hex on match the file bytes); recovered as a unit from the file bytes, 2929 executed PCs map to it
- runtime segment an address (play-l10): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 295 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 451 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- E-Motion/E-Motion.adf:mainbit loaded at an address (314356 bytes) +382C..3850: data by evidence (layout list that the screen builder reads after loading its address; never executed); 7 instructions decoded there were removed
- E-Motion/E-Motion.adf:mainbit loaded at an address (314356 bytes) +3D04..3D1C: data by evidence (option values and their maxima, indexed and changed by the options screen); 6 instructions decoded there were removed
- E-Motion/E-Motion.adf:mainbit loaded at an address (314356 bytes) +4666..468A: data by evidence (layout list that the screen builder reads for the bonus screens); 7 instructions decoded there were removed
- E-Motion/E-Motion.adf:mainbit loaded at an address (314356 bytes) +48DC..48E4: data by evidence (random-number state: two longwords loaded and stored by the random routine, seeded from the beam position at start-up); 3 instructions decoded there were removed
- E-Motion/E-Motion.adf:mainbit loaded at an address (314356 bytes) +4B840..4CBF4: data by evidence (music sequence tables: per-channel lists of sample-block addresses that the music interrupt reads one longword at a time); 1261 instructions decoded there were removed
- E-Motion/E-Motion.adf:e_motion: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- E-Motion.adf:e_motion
- E-Motion.adf:bootblock
- runtime segment an address
- runtime segment an address
- runtime segment an address

## Copy protection found

- `protection_track79_check` (mainbit loaded at an address (314356 bytes)): Copy protection (never called in this crack): reads track 79 twice with the sync word 4124 and fails unless the second sync lies at least 200 words after the first. Kind: copy protection: track-79 check - reads the raw track twice (side 0, side 1) with the non-standard sync an address and fails unless the second sync position is at least 200 words after the first. Pass path: returns D0 = 0 (pass); its only call, in the start-up sequence at +an address, was replaced by four NOPs in this crack, so it never runs.
- `protection_read_raw_track` (mainbit loaded at an address (314356 bytes)): Copy protection: reads a raw track by disk DMA with the sync word 4124 and waits for the transfer to end. Kind: part of the track-79 copy-protection check at +an address (only that check calls it). Pass path: see +an address (pass = D0 0); never runs in this crack.
- `protection_step_tracks` (mainbit loaded at an address (314356 bytes)): Copy protection: steps the drive head inwards by the given number of tracks. Kind: part of the track-79 copy-protection check at +an address (only that check calls it). Pass path: see +an address (pass = D0 0); never runs in this crack.
- `protection_seek_track0` (mainbit loaded at an address (314356 bytes)): Copy protection: steps the head outwards until the track-zero signal. Kind: part of the track-79 copy-protection check at +an address (only that check calls it). Pass path: see +an address (pass = D0 0); never runs in this crack.
- `protection_step_pulse` (mainbit loaded at an address (314356 bytes)): Copy protection: gives the drive one step pulse and waits. Kind: part of the track-79 copy-protection check at +an address (only that check calls it). Pass path: see +an address (pass = D0 0); never runs in this crack.
- `protection_select_drive` (mainbit loaded at an address (314356 bytes)): Copy protection: selects drive 0 with the motor on and waits for it to spin up. Kind: part of the track-79 copy-protection check at +an address (only that check calls it). Pass path: see +an address (pass = D0 0); never runs in this crack.

## Functions


### E-Motion.adf:e_motion

- `run_game_entry`: The routine calls decode_animation_frames and then initialize_game_state, and returns zero.
- `decode_animation_frames`: The routine copies a block of bytes from a static table into a RAM buffer and then repeatedly reads input bits, updates a counter, and writes bytes from another static table into the buffer backwards while calling set_player_direction and wait_for_input_bits.
- `set_player_direction`: The routine reads a word from the address in A0 minus four, stores a byte derived from that word together with the high byte of D7 into the word pointed to by A5, and returns the read word shifted right by one with the sign bit set.
- `wait_for_input_bits`: The routine repeatedly shifts the word in d0 right by one bit and, when the value becomes zero, calls set_player_direction and copies the short result from d1 into a local counter.
- `initialize_game_state`: The routine clears custom registers, programs display window and bitplane control registers, initializes sprite cop lists, installs interrupt handlers, runs game loops and waits for input, then copies palette data to chip memory and executes it.
- `install_interrupt_handlers`: The routine copies three long words from the address of copy_palette_data to the interrupt vector area and then calls the installed handler at that area.
- `copy_palette_words`: The routine copies four 32-bit words from the source pointer in A0 to the destination pointer in A1, decrementing a loop counter in D7.
- `check_and_close_window`: The routine calls a function through a pointer stored at an offset from a6, tests the returned integer, and if non-zero calls two further functions through pointers at other offsets from a6, passing the first result to the second and its result to the third.
- `run_game_loop`: The routine closes the window, runs the game tick loop, decodes and applies the palette, then performs a fade by waiting for scanlines and setting the palette fade fifteen times before returning.
- `run_game_loop_2`: The routine calls decode_and_apply_palette, then performs a sixteen-iteration loop that waits for a scanline and sets the palette fade each iteration, decrementing a counter until it wraps.
- `wait_for_scanline_loop`: The routine sets a short counter and repeatedly calls wait_for_scanline, decrementing the counter each iteration, until the counter reaches -1.
- `decode_palette_and_write_copperlist`: The routine walks a chunked resource area, decodes a palette chunk into a copper list buffer, then repeatedly decodes run-length blocks from a data chunk into a destination buffer, using the bytes as literal copies and negative repeat counts.
- `wait_scanlines_and_fade_palette`: The routine waits for scanlines and updates the palette fade repeatedly.
- `fade_palette_to_black`: The routine reads 32 palette entries from chip memory and writes them to the Amiga color registers.
- `wait_for_scanline`: The routine reads the vertical position register and spins until the value is greater than a lower bound and then spins until the value is less than an upper bound, effectively waiting for a specific scanline range.
- `init_sprite_cop_list`: The routine fills a short array with a repeating pattern of control words and pointer values derived from an initial register value, then writes a terminator pair and stores the base address and final pointer value in two global locations.

### E-Motion.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### logged disk load from E-Motion.adf:mainbit loaded at an address (314356 bytes)

- `program_entry`: First instruction of the main program at an address: a branch to the start-up sequence.
- `init_hardware`: Takes the machine over: clears the work RAM, installs the keyboard and VBlank interrupts, sets the display window, bitplanes, CIAs and audio, and seeds the random state from the beam position.
- `set_copper_list`: Points the copper at the list in A0 and remembers it.
- `init_audio_and_sign_samples`: Sets the four audio channels to a silent sample and converts the sample bank to signed form by flipping the top bit of every byte.
- `toggle_sound_mute`: Flips the mute flag and sets all four channel volumes to zero or full.
- `set_volumes_full`: Sets all four audio channel volumes to full (64).
- `start_sound_effects`: Stops all audio (the music included), installs the sound-effect interrupt handler and marks all four channels free.
- `play_sound_effect`: Starts the sample block in A0 on a free audio channel (or the next channel in turn) and marks it busy for its playing time.
- `audio_interrupt_effects`: Level-4 audio interrupt for sound effects: for each channel that finished, steps its state so the channel falls silent and becomes free.
- `debug_colour_loop_entry`: Unreached branch into an endless loop that writes a colour register (left-over debug code).
- `debug_colour_loop`: Unreached endless loop that counts and writes a colour register (left-over debug code).
- `wait_effects_idle`: Waits until no sound effect is playing on any channel.
- `start_music`: Starts the title music: points each channel at the first block of its sequence table and installs the music audio interrupt.
- `audio_interrupt_music`: Level-4 audio interrupt for the music: queues the next sample block of each finished channel from its sequence table, looping at the end marker.
- `decode_ilbm_picture`: Decodes an IFF ILBM picture: copies its CMAP into a colour table and unpacks the run-length BODY into interleaved bitplanes.
- `keyboard_interrupt`: CIA keyboard interrupt: reads the key code, lets the keypad move the screen up or down, stores the key and handshakes.
- `vblank_interrupt`: Vertical-blank interrupt: applies screen moves and fades, counts frames, reads the mouse, both joysticks and fire buttons, and counts the level timer down.
- `fade_step_palettes`: Advances a fade by one step and writes the faded colours into the gradient copper list and colour registers.
- `fade_colour_list`: Darkens a list of 12-bit colours by the current fade level, clamping each component at zero.
- `apply_bitmask_to_table`: The routine reads five consecutive words from a source pointer and uses the first word as a mask to AND with five entries in a table and then ORs the remaining four words into those entries.
- `wait_key`: Waits for a key press and returns its character.
- `read_key_ascii`: Returns the last key as a character (shifted table when shift is held), or zero when no key is waiting.
- `draw_text`: Draws a zero-terminated string with the 16-pixel font, one character after another.
- `build_gradient_colours`: Builds the fifteen background gradient colours from one 12-bit base colour.
- `set_bitplane_pointers`: Writes the five bitplane addresses of a screen buffer into the copper list.
- `mask_screen_edges`: Clears the leftmost and rightmost words of the playfield rows in both halves of the screen.
- `draw_digit`: Draws one decimal digit with the 16-pixel font.
- `draw_char`: Draws one character of the 16-pixel font into the bitplanes, with its shadow.
- `draw_two_digits`: Draws two decimal digits side by side.
- `draw_two_chars`: Draws two characters side by side with the 16-pixel font, one word at a time.
- `draw_ship_slice`: Draws one 16-line slice of the rotated ship image at the current row, with the wrap at the screen edge.
- `draw_ship_slice_planes`: Shifts one slice of the ship image to its pixel position and masks it into the five bitplanes.
- `draw_ball_clipped`: Draws a ball image at a screen position, splitting it in two when it wraps past the bottom or right edge.
- `blit_ball`: Blits a ball image with its mask into the five bitplanes (cookie-cut).
- `draw_sprite_frame`: Draws one frame of an object image at a screen position, choosing the blit width by image size and handling the right-edge wrap.
- `draw_frame_width_a`: Draw-width table entry for the narrowest object images.
- `draw_frame_width_b`: Draw-width table entry for medium object images.
- `draw_frame_width_c`: Draw-width table entry for the widest object images.
- `draw_frame_width_c_body`: Blits a wide object image, with special cases where it crosses the right edge of the screen.
- `draw_frame_wrap_col34`: Blits a wide object image split at the right edge (starting at word 34).
- `draw_frame_wrap_col36`: Blits a wide object image split at the right edge (starting at word 36).
- `draw_frame_wrap_col38`: Blits a wide object image split at the right edge (starting at word 38).
- `blit_frame_masked`: Blits an object image with its mask into the bitplanes, plane group by plane group.
- `draw_line_dual`: Draws a one-pixel line in colour 1 into two buffers at once, used for the tethers between linked balls.
- `extract_image_from_table`: The routine takes two inputs, computes a table address from them and a global variable, then uses a mask derived from the first input to clear and set 16-bit words in the target table in a repeating 120-byte pattern.
- `write_planes_from_source`: The routine copies 16-bit words from a global source table into a destination table using an index derived from the inputs.
- `render_sprite_to_screen`: Copies sprite data from a global table to the screen buffer, applying bitwise masking and shifting to handle sub-pixel positioning and transparency.
- `wait_for_vblank_and_copy_planes`: The routine waits for a hardware flag to clear, then configures copper registers and blitter registers for a series of block copies.
- `copy_struct_array`: The routine copies a fixed-size array of structures from a source table to a destination table, preserving each field in order while skipping a single word in each structure.
- `copy_struct_block`: The routine copies a fixed-size block of 32-bit words from a source table located at a constant address to a destination global variable, advancing both pointers each iteration.
- `copy_sprite_attribute_blocks`: The routine copies a fixed number of 60-word blocks from a global table to a destination pointed to by A1, skipping one word in each block.
- `clear_object_flags`: The routine copies a block of words from a source table to a destination table, clearing four specific word fields in each destination entry to a constant value.
- `generate_bit_permutation_table`: The routine fills a table in memory with a sequence of 16-bit values derived from repeated addition and carry propagation, then copies the resulting 32-bit words from a source table into the same destination area.
- `hiscore_disk_set_buffer`: High-score disk I/O: sets the track buffer address.
- `hiscore_disk_seek`: High-score disk I/O: turns the motor on and steps the head to the requested track.
- `hiscore_disk_read`: High-score disk I/O: reads and MFM-decodes the high-score track, returning non-zero on error.
- `hiscore_disk_write`: High-score disk I/O: MFM-encodes the high-score block and writes the track.
- `hiscore_disk_motor_off`: High-score disk I/O: deselects the drive and turns the motor off.
- `delay_loop`: The routine writes a constant to a global variable and then executes a nested loop that decrements two short counters to -1, effectively performing a delay.
- `set_global_byte`: The routine writes a constant byte value to a global variable and then returns.
- `wait_for_button_release`: The routine polls a global variable and repeatedly calls another routine while decrementing a counter from 255 down to zero.
- `update_global_state_from_input`: The routine reads a word from D0, masks and conditionally sets bits in a global variable and then computes a signed difference with the stored value.
- `poll_ciab_prb_loop`: The routine reads a global variable, ORs it with 3, writes the result back to the same global variable, then repeatedly calls another routine until that call returns zero.
- `count_down_loop`: The routine loads a short immediate value into a register and repeatedly decrements it until it reaches -1, performing no other operations.
- `copy_tile_row_block`: The routine computes a small offset from an input value and uses it to select a source region in a table.
- `reset_input_buffer_and_poll_ciab`: The routine clears a 10-byte input buffer at a fixed address and programs a CIA timer and control registers via an A6-based structure.
- `decode_data_block`: The routine scans a buffer for a marker value and then searches a global table for a matching 32-bit key.
- `clear_loop_counter`: The routine receives an undefined word value, shifts it right by two, subtracts two, then decrements the result in a loop until it reaches -1.
- `deinterleave_even_words`: The routine copies a block of words from a source pointer, using a second pointer as an offset source, and writes each result to a destination pointer.
- `extract_and_combine_words`: The routine reads two 32-bit words from a table pointed to by a pointer, with an offset derived from a short index.
- `copy_table_segment`: The routine computes a small offset from an input value and stores several inputs into a global variable.
- `wait_for_input`: The routine initializes several fields in a structure pointed to by A6 and clears a global variable.
- `fill_memory_and_loop_calls`: The routine fills a block of memory with a constant pattern by writing to consecutive words in a global variable table, then it calls another routine repeatedly in a small loop controlled by a stack byte.
- `initialize_memory_pattern`: The routine writes a constant 32-bit pattern to a location derived from an uninitialized pointer, then repeatedly calls another routine in a loop that runs four times, followed by a series of additional calls to another routine, one of which is invoked with a constant argument.
- `configure_and_wait_for_flag`: The routine writes a set of control words and two input values into a structure pointed to by a register, then waits for a flag bit in the structure to be set, repeating the write-wait sequence with different control values and input offsets.
- `recursive_routine_calls`: The routine performs three calls to another routine and then returns.
- `bitwise_mix_store_word`: The routine mixes the input word with a transformed mask derived from the low bits of the input, optionally clears the sign bit based on a flag byte before the output pointer, and stores the resulting word through the pointer.
- `xor_reduce_masked`: The routine reads a word from a pointer, then repeatedly XORs it with successive words from memory for a fixed number of iterations determined by an input word, decrementing a counter each time.
- `set_flag_byte`: The routine reads a byte from a pointer, tests a bit in that byte and a bit in the preceding byte, and then sets or clears a bit in the byte pointed to before writing it back.
- `trap_return`: Exception handler that returns at once (installed for divide by zero).
- `decode_input_byte`: The routine decodes a byte from D0 and updates a global variable with masked and combined results from another routine.
- `mask_global_with_complemented_value`: The routine checks a byte in D0 and clears a global variable when the masked value matches one of two constants.
- `build_input_code`: The routine maps a character in D0 to a pair of small codes and returns them combined with D1.
- `read_key_check_cheat`: Reads a key: the % key saves the high scores, and typing MOONUNIT sets the cheat flag.
- `build_title_screen`: Sets the title palette ramps and draws the title screen layout.
- `draw_title_layout`: Clears the screen, draws the title layout and selects the title colours.
- `enter_high_score`: Turns the score into text, finds its place in the table, shifts lower entries down and lets the player type a name.
- `draw_high_score_table`: Draws the eight high-score lines (score, level and name) on the high-score screen.
- `load_high_scores`: Reads the high-score block from track 78 and uses it when its HIGH tag is present.
- `save_high_scores`: Writes the high-score block back to track 78 if the disk had one.
- `offer_continue_from_level_25`: After a game over on levels 25 to 49, shows PRESS FIRE TO CONTINUE FROM LEVEL 25 with a ten-step countdown; if fire is pressed the level is set back to 25 and play restarts, otherwise the title sequence resumes.
- `options_screen`: The options screen: draws the six options and lets the joystick pick a line and change its value until fire is pressed.
- `protection_track79_check`: Copy protection (never called in this crack): reads track 79 twice with the sync word 4124 and fails unless the second sync lies at least 200 words after the first.
- `protection_fail`: Copy protection: returns the failure result (one).
- `protection_find_sync`: Copy protection: searches the raw track buffer for the sync word and returns its word position, or minus one.
- `protection_sync_missing`: Copy protection: returns minus one when the sync word was not found.
- `protection_read_raw_track`: Copy protection: reads a raw track by disk DMA with the sync word 4124 and waits for the transfer to end.
- `protection_step_tracks`: Copy protection: steps the drive head inwards by the given number of tracks.
- `protection_seek_track0`: Copy protection: steps the head outwards until the track-zero signal.
- `protection_step_pulse`: Copy protection: gives the drive one step pulse and waits.
- `protection_step_wait`: Copy protection: waits after a step until the drive is ready.
- `protection_select_drive`: Copy protection: selects drive 0 with the motor on and waits for it to spin up.
- `update_energy_bar`: Grows or shrinks the energy bar by one step towards its target and loses the ship when it runs out.
- `set_game_display`: Selects the game copper list and the game palette ramps.
- `fade_out_if_needed`: Fades the screen out unless it is already dark.
- `fade_out`: Starts a fade to black and waits until it has finished.
- `fade_in`: Starts a fade back up from black if the screen is dark.
- `build_palette_ramps`: Builds three eight-step colour ramps from three step values.
- `build_screen_from_layout`: Draws a screen from a layout list: walls and pieces first, then the joins between them.
- `layout_join_kind`: Finds which piece end a wall joint touches and returns the kind of join to draw.
- `find_free_object`: Returns the first free entry of the object table.
- `swap_screens`: Waits for the next frame when needed, shows the screen just drawn and selects the other buffer for drawing.
- `select_draw_buffer`: Selects which screen buffer and dirty list the next frame draws into.
- `wait_frame`: Waits until the frame counter changes (the next vertical blank).
- `clear_score_digits`: Clears six score digits.
- `add_bonus_and_score`: Adds points to the bonus counter, then to the player's score.
- `add_score`: Adds points to the score (not in practice mode) and awards an extra ship when the second score digit changes from an odd value.
- `add_digits`: Adds a packed number to a six-digit decimal counter, carrying digit by digit.
- `bonus_points_screen`: Shows BONUS POINTS AWARDED with the bonus total and waits for fire.
- `bonus_level_screen`: Shows BONUS LEVEL with its two rule lines and waits for fire.
- `secret_bonus_screen`: Shows SECRET BONUS and the reason for it, then moves on to the next level when the delay ends.
- `draw_char_pairs`: Draws a string two characters at a time.
- `draw_level_banner`: Draws LEVEL and the level number in the status area, or CONGRATULATIONS after the last level.
- `draw_game_over`: Draws GAME OVER letter by letter as its timer runs down.
- `draw_time`: Draws the level timer digits.
- `draw_lives`: Draws the number of ships left.
- `draw_score`: Draws the six score digits in the status bar.
- `draw_six_digits`: Draws a six-digit decimal counter.
- `random_below`: Returns a random number below the given limit.
- `random_next`: Advances the 64-bit random state and returns the next random number.
- `explode_ships`: Plays the explosion sound and turns the ship (both ships with two players) into its exploding form.
- `cycle_frame_four`: Steps an object's animation frame through four frames.
- `set_flag_by_global_range`: The routine checks a global variable against a range of values and, if the value falls within that range, writes a non-zero byte to a memory location; otherwise it clears that byte.
- `decrement_global_and_branch`: The routine receives a word in D0, subtracts a value derived from the high nibble of that word plus eight from a global variable, then calls another routine.
- `increment_global_and_scan_table`: The routine increments a global variable, resets it when it exceeds a threshold, calls another routine twice, then walks a table of short entries until it finds a sentinel value and writes a marker into the table.
- `handle_input_0x20`: The routine returns the input value unless it equals a specific code, in which case it sets a global variable and calls another routine twice.
- `process_collision_and_update_state`: Plays a sound effect and calculates a value from a global variable to potentially add bonus points.
- `check_table_entry_2`: The routine increments a global counter and resets it after a threshold.
- `increment_global_and_update_table`: The routine increments a global variable, wraps it when it exceeds a threshold, calls another routine twice, then walks a table by a fixed stride until it finds a non-negative entry equal to a specific value, and writes that value into a field of the same entry.
- `check_and_increment_flag`: The routine returns the input when the input is not a specific value, otherwise it calls another routine twice, reads a global variable and conditionally calls another routine, increments the global variable and adjusts it with an unsigned short, then returns the result.
- `check_and_update_flag_2`: The routine calls another routine twice, reads a byte from a global variable and, if the variable is not one, calls another routine to obtain a value.
- `check_score_add_bonus`: Plays a sound effect and conditionally adds bonus points based on a global variable range.
- `check_table_and_set_byte`: The routine checks a signed offset derived from a register and conditionally calls another routine.
- `update_object_motion`: The routine updates a motion object stored in the A6 argument block by iterating a table of control points and selecting the closest one based on distance calculations.
- `update_sprite_motion_state`: The routine updates a sprite motion state stored in a structure pointed to by a register.
- `clear_object_state`: The routine calls another routine and, if a zero flag condition is true, writes a constant to a word pointed to by an output pointer and then stores return values from two further calls into two words of a structure referenced by that pointer.
- `spawn_object_from_data`: Plays a sound effect and initializes a global flag.
- `spawn_explosion_particles`: Plays a sound effect and clears the source object's data.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `increment_counter_with_branching`: The routine increments a counter stored in a structure and checks it against a limit.
- `initialize_extra_output_block`: The routine calls another routine and, if a zero flag condition is true, writes a constant word to a memory block pointed to by an output pointer, fills several words in the block by calling another routine twice and writing constants, then loops calling another routine until a masked result is not both bits set, and finally stores a masked result word into the block.
- `process_collision_object`: Updates a collision object’s counters and positions, normalises angles, checks the object against a table of objects and returns early if a match is found.
- `update_animation_state`: The routine updates a set of counters in a global structure, then iterates over a table of objects to find the closest one based on a wrapped distance metric.
- `update_entity_state_and_find_match`: The routine updates several fields of a structure pointed to by a register, reads and writes a global variable and calls another routine conditionally based on flags derived from that variable and from the structure.
- `jump_to_reset_global_state_on_entry`: Jump stub: passes control straight on to reset_global_state_on_entry.
- `increment_and_mask_word`: The routine increments a 16-bit word located at an offset from an address in a register and then masks the result with the value 3.
- `apply_impulse_between_two_entities`: The routine reads two entity pointers from a table, checks their type fields for a specific set of values, and if they match it computes a scaled difference of two short offsets using a divisor in a register, then adds and subtracts the scaled result from velocity fields in a structure pointed to by an output pointer, otherwise it clears a flag in the table.
- `update_animation_state_and_find_nearest`: The routine updates a set of counters and values stored in a table pointed to by a register, calls another routine twice, then scans a table of entries starting at a fixed address to locate the entry whose position is closest to a reference point using wrap-around distance calculations, and returns the index of the best match.
- `accumulate_and_clamp_values`: The routine reads two pairs of 32-bit values from a structure pointed to by A6, adds each pair, writes the sums back into the structure, and then checks the high 16 bits of the two sums against limits.
- `decrement_counter_and_clear`: The routine receives a pointer in A6 and treats it as a two-byte table.
- `decode_object_positions`: Decrements a counter in a structure and updates a global flag when the counter reaches zero, then adjusts position-related fields in the structure by adding and damping values and calls another routine twice.
- `jump_to_empty_stub`: Jump stub: passes control straight on to empty_stub.
- `update_collision_state`: The routine updates a collision state structure in a register, normalises position fields with wrap-around, and then searches a table of objects for the closest matching entry using distance comparisons.
- `update_object_list`: The routine updates a set of counters in an object list, calls another routine twice, then searches a table of objects using distance checks with wrap-around adjustments.
- `increment_counter_and_process`: The routine increments a counter stored in a structure, checks it against a limit, and calls another routine twice.
- `update_animation_state_2`: The routine updates a state block in A6, decays two 32-bit accumulators, calls another routine twice, then walks a table pointed to by a global variable to compare and wrap short fields, adjusting values and possibly returning early.
- `update_and_draw_objects`: Runs the update routine of every object by type, then the draw routine of every object by type.
- `set_sprite_pattern_and_wait`: The routine computes indices from input values and a table pointer, writes a set of short parameters through an output pointer, then calls another routine and waits on a global variable flag.
- `set_sprite_frame_and_wait`: The routine computes a sprite frame index from a value returned by another routine, writes the index and related control words into a structure pointed to by a register, adjusts the control words for two specific frame values, then repeatedly calls another routine while waiting for a timer value to reach a threshold and then optionally waits for a second timer condition.
- `empty_routine_3`: The routine contains no instructions and returns immediately.
- `draw_line_with_mask`: The routine reads two pairs of coordinates and a mask from tables indexed by values in a global structure, then computes a line between the two points.
- `increment_global_and_dispatch`: The routine increments a global counter, loads a value from a table pointed to by a register, and uses it to compute an offset into another table.
- `increment_and_dispatch`: The routine increments a global counter, reads a short value from an address on the stack, and if the value is non-negative it computes an index and copies two words from a global area to fixed RAM locations.
- `increment_global_and_dispatch_2`: The routine increments a global counter and then calls another routine, using the result to index a table of function pointers.
- `increment_global_and_dispatch_3`: The routine increments a global variable under a condition, reads a word from the passed structure, and if the word is non-negative it computes offsets into a table and calls another routine with derived parameters.
- `empty_routine_4`: The routine is empty.
- `increment_global_and_dispatch_4`: The routine increments a global counter, loads a word from a pointer in A6, and if the value is non-negative it computes several offsets into a global table and calls another routine with derived parameters.
- `increment_global_and_dispatch_5`: The routine increments a global counter and then calls another routine, using the result to index a table of jump targets and invoke one of them based on a value read from memory.
- `increment_global_and_dispatch_table`: The routine increments a global counter, loads a word from a structure pointed to by a register, and if the value is non-negative it computes several offsets into a global table and calls another routine with derived parameters.
- `draw_dispatch_last_entry`: Last entry of the object draw dispatch table, indexed by object type: a single branch to the shared drawing path that most object types use.
- `swap_two_table_entries`: The routine reads a 16-bit index from a global variable, uses it to fetch a 16-bit value from a table, then reads another 16-bit index from the same global variable, uses it to fetch a 16-bit value from a second table and stores it back into the global variable, and finally writes the first fetched value into the global variable, effectively swapping the two table-derived entries.
- `empty_routine_placeholder`: This routine is empty and contains no executable instructions other than a return.
- `empty_routine_5`: The routine contains no instructions and returns immediately.
- `compute_bitwise_iteration`: Performs a bitwise arithmetic loop on an input value, shifting and adding bits based on carry flags and comparisons, iterating up to 15 times before returning.
- `wrap_palette_table`: The routine fills a table of short values with a wrapped index sequence.
- `fill_modulo_table`: The routine fills a short table pointed to by a pointer with values derived from a width parameter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_state_for_play_l10`: The routine calls two other routines, writes a constant to a global variable, then conditionally overwrites it with another constant, performs a self-assignment, conditionally overwrites it again, calls another routine, and then writes zero to a RAM location before the disassembly ends with a bad instruction.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_game_state_handler`: The routine stores a pointer to initialize_game_state_2 in a global variable and then triggers a trap.
- `initialize_game_state_two`: The routine calls initialize_game_state_3, copy_code_bytes, update_game_state, func_0x00070022 and initiate_game_loop in that order and then returns.
- `delay_frame_count`: The routine loads a short with a negative constant and decrements it in a loop until it reaches -1, then returns zero.
- `copy_code_bytes`: The routine copies a block of bytes from a fixed source location to the address of initialize_input_interrupt, decrementing a counter until it reaches -1.
- `initialize_game_state_3`: The routine sets up several work areas in RAM, fills tables with constant values, runs a small loop that builds a byte table from a string and derives offsets from it, calls copy_word_pairs_to_buffer twice and load_animation_frames, copies a block of code into the low memory area, and writes a few bytes to custom hardware registers.
- `load_animation_frames`: The routine copies a small block of bytes from a fixed source area to a fixed destination area and then copies a fixed 144-byte block from a fixed source area to a fixed destination area.
- `copy_word_pairs_to_buffer`: The routine copies word pairs from D0 into the buffer pointed to by A0.
- `wait_vblank_update_state_copy`: The routine repeatedly waits for vertical blank, updates HUD digits, bob animation and animation frames while reset_game_state is non-zero, then copies a small block of bytes from a fixed source to a fixed destination and writes four bytes from a fixed source to fixed locations.
- `reset_game_state`: The routine conditionally writes a constant word pattern to several global locations based on the input registers and two memory flags, then stores a jump instruction word and a function pointer, and ends with a trap.
- `set_game_state`: The routine stores a byte value into a memory location and then returns.
- `wait_for_vblank`: The routine polls the vertical position register until the value in the range of the register equals the value that indicates the vertical blank interval, then returns.
- `update_hud_digit_display`: The routine updates a digit counter and a set of screen positions based on the counter, then selects a digit bitmap from a table and copies it to a buffer using the current digit value and a flag from CIAA_PRA and POTGOR. It finally copies a small block of data to a fixed location and returns.
- `update_animation_frames`: The routine increments a palette index with wrap-around and copies eight 16-bit palette entries from a table to a fixed destination.
- `update_emotion_palette`: The routine updates a palette-related counter and, when a condition is met, copies palette data and writes eight entries from a source table into a destination table with a fixed stride, then sets several global variables and waits for a hardware flag to clear before returning.
- `update_bob_animation`: The routine updates a bob animation counter and, when the counter wraps, calls copy_palette_data_2 and copies eight palette entries from a table to a fixed destination using a stride.
- `copy_palette_data_2`: The routine checks a flag in D1.
- `initiate_game_loop`: The routine calls an external function pointer, stores a constant byte in a global, copies a word from the returned pointer into a global, calls initiate_emotion_sprites, and then calls another external function pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_input_interrupt`: The routine calls initiate_emotion_sprites and then calls a function pointer stored in the data segment.
- `disable_interrupt_server`: The routine calls a function pointer stored in the data segment and writes a constant to a data location.
- `initiate_emotion_sprites`: The routine initializes a counter and optionally sets it from a RAM byte when a RAM word and byte match specific values.
- `trigger_game_over`: The routine calls update_sprite_positions and returns the value in d1.
- `update_sprite_positions`: The routine updates sprite position data by iterating over three sprite entries, writes the computed values to custom chip registers via DMACON, and conditionally changes the sprite offset.
- `process_input_events`: The routine iterates over a set of input slots, reads per-slot bytes from RAM and updates a per-slot control block in RAM. It copies and combines values into the block, writes to CIAA_PRA via the custom chip registers, and sets a dirty flag when a field changes.
- `init_sprite_animation`: The routine initializes a sprite animation structure from a table entry selected by a type byte, copies palette and frame data into the sprite buffer, sets DMA control bits, and updates the sprite position fields.
- `set_player_sprite_offset`: The routine writes a 16-bit value from a table indexed by a byte in D4 to the word at offset six from the pointer in A1.
- `update_audio_channels`: The routine loops over a set of channel structures, checks a field in each structure and, when the field is non-zero, calls update_animation_frames_2.
- `update_animation_frames_2`: Updates three independent animation counters stored in the structure pointed to by A2 using tables pointed to by A3.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_two_functions`: The routine calls two functions and then returns zero.
