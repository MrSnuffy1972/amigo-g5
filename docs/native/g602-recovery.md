# Game 602 (Damocles v1.3 0643): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00602). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1123096 top-level bytes in 13 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 96 raw-word lines |
| Functions named and described | pass, 463 of 463 named (0 author symbols), 463 of 463 described (15 by rule, 448 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4676 of 4676 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 22501 of 22504 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 292589 |
| code | 73174 |
| data | 756126 |
| padding | 55 |
| unused | 1152 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Mercenary2save.islave | 1 | 73 | 6 |
| game.slave | 1 | 218 | 11 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 22201 | 440 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 3 |

## Cases handled

- Damocles_v1.3_0643/Damocles_v1.3_0643_hdf/Damocles/Disk.1: raw disk image (485376 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Damocles_v1.3_0643/Damocles_v1.3_0643.hdf: hard-disk image (AmigaDOS OFS, 1216 blocks): its filesystem parses and all 8 files in it are byte-identical to the extracted tree Damocles_v1.3_0643_hdf, which is recovered file by file; no block outside the filesystem holds data
- 2 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Damocles_v1.3_0643/Damocles_v1.3_0643_hdf/Damocles/Disk.1: 2 logged loads cover 477504 bytes of the image
- Damocles_v1.3_0643/Damocles_v1.3_0643_hdf/Damocles/Disk.1: chunks 126-128 (bytes 32256-33024) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.41)
- Damocles_v1.3_0643/Damocles_v1.3_0643_hdf/Damocles/Disk.1: chunks 133-143 (bytes 34048-36864) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.72)
- Damocles_v1.3_0643/Damocles_v1.3_0643_hdf/Damocles/Disk.1: chunks 1884-1895 (bytes 482304-485376) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.92)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `poll_joystick_input` (Disk.1 +an address loaded at an address (445376 bytes)): The routine repeatedly clears a palette buffer, triggers a disk load routine twice with a short delay between them, toggles a bit in a global variable, polls the joystick button, and waits for game frames until a condition on two output registers is met, then returns the global variable. Kind: disk hardware check. Pass path: not determined.
- `poll_joystick_input_2` (Disk.1 +an address loaded at an address (445376 bytes)): The routine repeatedly attempts a disk load and a copy-protection decode, polling a joystick button between attempts while copying a small block of memory byte-by-byte and exiting early if a pointer reaches a limit. Kind: disk hardware check. Pass path: bmi.w an address.
- `initiate_disk_load_2` (Disk.1 +an address loaded at an address (445376 bytes)): The routine writes several values to a global variable, calls another routine to wait for vertical blank, and then writes two more values to the same global variable before returning. Kind: disk hardware setup / trackloader. Pass path: not determined.

## Functions


### Mercenary2save.islave

- `start_game_with_default_save`: The routine is empty and returns a constant value.
- `verify_save_checksum`: The routine calls another routine to obtain a seed value repeatedly until a specific value is reached or a counter expires, then it repeatedly calls the seed routine, updates a 32-bit accumulator with a byte from the seed mixed with the accumulator, writes the seed byte through a pointer, and finally compares a derived 16-bit value with a value built from two seed bytes, returning an error code if they differ.
- `return_error_code`: The routine returns a constant 32-bit value with all bits set.
- `generate_mercenary_seed`: The routine builds a 32-bit value from the high word of an input and a word pointed to by a pointer, then repeatedly doubles the value while mixing high and low parts for seven iterations.
- `decrement_mercenary_counter`: The routine calls another routine via a pointer stored in memory and then, if a condition is true, calls verify_save_checksum.
- `check_save_slot_name`: The routine scans a global variable byte by byte from index 9 down to 0, checking for a zero byte.

### game.slave

- `return_error_code_2`: The routine is the program entry point and returns a constant value to the caller.
- `initialize_hardware_registers`: The routine writes to custom chip registers including color, copper lists, DMA control and interrupt enable, and to CIAA registers for I/O. It clears and sets interrupt requests and calls several other routines to reset audio, execute a game script and invoke a dynamic callback.
- `call_dynamic_callback`: The routine loads a pointer from a global variable, adds a fixed offset to it, dereferences it twice to obtain a code pointer, and calls the pointed-to routine.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_interrupt_request`: The routine waits for vertical blank synchronization and then stores a constant value into a global variable.
- `set_interrupt_request`: The routine waits for vertical blank synchronization and then writes a value to a global variable.
- `set_interrupt_request_2`: The routine waits for vertical blank synchronization and then stores the value 8 into a global variable.
- `wait_for_vblank_sync`: The routine sets a global variable and checks a flag; if the flag is clear it returns the contents of two input registers.
- `reset_audio_channels`: The routine clears a memory location via a pointer, calls another routine to set game speed, then conditionally calls another routine to set game speed again and invokes a function from a table based on an index.
- `execute_game_script`: The routine calls set_game_speed and then calls a function pointer located at an offset from an uninitialized pointer.
- `set_game_speed`: The routine reads a global variable, shifts it right by one, subtracts one, ORs the result with a constant, and stores the byte into a global variable.

### logged disk load from Disk.1

- `process_script_commands`: The routine repeatedly dereferences a global variable through two jump tables to follow a chain of script pointers, then builds a 32-bit pointer from successive bytes and advances the pointer.
- `copy_string_with_jump_table`: The routine copies bytes from a source pointer to a destination pointer, following a jump table for negative byte values.
- `reset_audio_channels_2`: The routine writes zero to a series of global variables and then writes a constant to one of them.
- `jump_to_update_enemy_positions_2`: Jump stub: passes control straight on to update_enemy_positions_2.
- `jump_to_draw_tile_mask_3`: Jump stub: passes control straight on to draw_tile_mask_3.
- `jump_to_apply_palette_masks`: Jump stub: passes control straight on to apply_palette_masks.
- `update_enemy_positions`: The routine reads several values from a global variable and from tables indexed through a global variable, combines them with values returned by another routine, and writes the results back to a global variable.
- `update_enemy_targeting`: The routine reads several global variables and performs repeated 16-bit arithmetic with table lookups using a masked index derived from the low word of intermediate sums, then writes the resulting combined values back to a global variable and calls another routine twice.
- `update_enemy_projectile`: The routine reads several values from a global variable and from memory tables, performs a series of 16-bit additions and index calculations using a masked shift pattern to look up entries in a table, writes intermediate results back to the global variable, and calls another routine twice.
- `update_enemy_positions_2`: The routine repeatedly reads a global variable and a table at a fixed offset, combines parts of the values with arithmetic to form an index, looks up a word in a table and writes the result back to the global variable.
- `process_input_queue`: The routine repeatedly calls two other routines and decrements a counter until it reaches a sentinel value.
- `wait_for_vblank`: The routine repeatedly calls a function and decrements a counter until the counter reaches -1, then returns.
- `clear_tile_range_bits`: The routine clears bits in four parallel tile maps for a range of tile indices.
- `draw_hud_bar`: The routine draws a horizontal bar into a planar bitmap using a mask table.
- `mask_tile_plane`: The routine masks a range of words in a plane buffer using a mask table.
- `draw_tile_mask`: Sets a mask over a range of bytes in four 320-byte planes.
- `clear_screen_rect`: The routine clears a rectangular region of a planar bitmap by masking the planes with values taken from a table.
- `draw_hud_bar_segment`: The routine clamps two short inputs to a valid range and, if the start is not past the end, computes the length of a segment.
- `apply_tile_mask`: The routine checks two short arguments for range, computes a length and a start offset, then uses a table indexed from a global variable to fetch a mask word.
- `draw_tile_pattern`: The routine clips two word inputs to a range and computes a span length.
- `clear_tile_range`: The routine checks a range of tile indices passed in registers and, if the range is valid, computes a pointer into a tile buffer using a global base and the start index.
- `clear_enemy_bullets`: The routine reads a 16-bit mask from a table using two parameters and clears bits in four screen buffers at fixed offsets, then zeroes a repeating pattern of buffer entries with a mask and a fill value, and finally applies a second mask from the table to the remaining buffers.
- `clear_screen_buffers`: The routine clears three screen buffers by writing the same word value to each buffer at the same offset, repeating for a fixed number of iterations determined by a counter.
- `draw_hud_bar_segment_2`: Clamps two short inputs and computes a length, then uses a table indexed by the start offset to OR a mask into a word at a base pointer and applies the inverse mask to three other planes.
- `apply_tile_mask_2`: The routine clamps two short inputs and, when the range is valid, computes an offset into a mask table using a global pointer and applies the mask to four 16-bit planes at a base address derived from the first input.
- `draw_hud_bar_segment_3`: The routine clips two 16-bit values to a range, computes a length, and uses a table to fetch 16-bit patterns.
- `clear_screen_region`: The routine clears a rectangular region of a bitplane screen by masking words in four bitplane planes using a mask table.
- `mask_tile_rows`: The routine masks a vertical range of tile rows in four bitplanes using a mask table.
- `draw_player_sprite`: The routine reads a short from a table indexed by a register and ORs it into a destination word and into three other words at fixed offsets, then clears a region of the destination by writing a constant to successive words and two fixed offsets, and finally reads another short from the table with a different index and ORs it into the destination and the same three offsets.
- `fill_palette_ram`: The routine writes the same 16-bit values from registers to four equally spaced locations in a word array pointed to by a pointer, repeating until a counter reaches -1.
- `apply_tile_mask_3`: The routine clamps two short inputs and, if they define a non-zero range, computes a start pointer from one input and a base address, then applies a mask from a table to the first word and ORs the mask into three other words spaced apart in the same block.
- `draw_tile_mask_3`: The routine clamps two short inputs to a valid range and computes a length.
- `jump_to_mask_sprite_plane`: Jump stub: passes control straight on to mask_sprite_plane.
- `mask_sprite_plane`: The routine computes a bounding box from the supplied coordinates, selects a mask word from a table using the start column and a global offset, and then ANDs that mask into four vertically spaced planes of a global bitplane buffer for each row in the box.
- `mask_tilemap_rows`: The routine masks a tilemap area using two mask words loaded from a table via an index derived from an input word and a register value, applying the first mask to four planes and zeroing a number of words per row, then applying the second mask to the same planes.
- `apply_palette_masks`: The routine writes a 16-bit mask value to four equally spaced locations in a destination buffer and then repeatedly applies two different 16-bit masks with AND to a set of source words at fixed offsets, advancing a pointer through a table.
- `set_game_state`: The routine contains only a self-referencing assignment through an undefined pointer and two calls to other routines, then returns.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d3.
- `update_game_state_2`: The routine contains no observable operations; it is empty.
- `initialize_game_state`: The routine calls two other routines and then returns.
- `initialize_game_state_2`: The routine is empty; it contains no instructions.
- `normalize_angle_delta`: The routine reorders two 32-bit inputs in D1 and D2, builds a swapped word value, computes a 16-bit delta between the high words, and then conditionally shifts the delta left until a particular bit becomes set or cleared.
- `skip_on_zero_input`: The routine receives a 16-bit value in D1 and converts it to a non-negative magnitude.
- `skip_if_negative_input`: The routine receives a value in D1, sign-extends it to a 32-bit signed integer and takes its absolute value.
- `get_absolute_value`: Checks if a register contains a negative value and converts it to a positive value.
- `update_player_input_state`: The routine reads the joystick port movement register and the CIA port A register, combines the values with a bitwise expression involving a global variable, and writes the result back to the global variable.
- `skip_game_logic_if_disabled`: The routine checks a global variable and returns the value in D0 unchanged in both branches.
- `init_audio_channel`: The routine writes a series of constants to a global variable and then sets up audio hardware registers for channel 2, enabling DMA for that channel.
- `update_audio_period`: The routine updates a global variable used for audio timing and writes a computed period value to the AUD2PER hardware register.
- `init_audio_hardware`: The routine writes a series of values to global variables and then checks a global variable.
- `update_audio_channel_period`: The routine updates a global variable using arithmetic and masking, clamps it to a minimum, then increments it and checks for zero.
- `reset_audio_channels_3`: The routine writes zero to a global variable, writes six to a global variable, writes zero to a global variable, and stores the address of a global variable into a global variable.
- `init_audio_channel_2`: The routine writes a series of values to a global variable and then returns.
- `decrement_music_counter`: The routine writes a constant to a global variable, decrements a byte in a global variable and tests it for zero.
- `init_game_state`: The routine writes constant values to a global variable, stores the address of that variable into another global variable, and returns the low word of the entry register masked with a constant bit pattern.
- `reset_audio_channel`: The routine writes to a global variable, clears it, stores the address of the variable in another global, and returns.
- `set_audio_channel_3`: The routine writes to the audio channel three hardware registers and then calls another routine to stop the channel.
- `stop_audio_channel`: The routine stops audio channel 3 by writing to the audio channel 3 registers for period, volume and length, and clears the audio DMA enable bit in DMACON. It also writes a value to a custom chip register.
- `init_audio_channel_3`: The routine writes constants to a set of global variables and then updates them with values returned by another routine.
- `update_audio_channel_3`: The routine decrements a global counter and, if it reaches zero, loads a new value from another routine and clamps it; it also decrements a second byte and, if zero, calls another routine to advance dialogue text and then resets several globals before returning.
- `init_audio_channel_4`: The routine writes constants to a global variable and a table entry and then returns.
- `initialize_audio_channel`: The routine writes a series of values to a set of global variables and then returns.
- `update_audio_channel_3_2`: The routine updates a global variable used for audio channel 3 by decrementing a counter and, when it reaches zero, advances a dialogue pointer and sets a period value from a table, clamping it if out of range.
- `set_audio_channel_2`: The routine writes to the Amiga audio channel 2 hardware registers and DMACON to enable DMA, and it initializes several global variables and a pointer.
- `update_animation_frame`: The routine checks a global variable and, if it holds a sentinel value, points the variable at a table and returns.
- `update_audio_period_2`: The routine computes a value from a 12-bit input in D0 and a pointer in A0, clamps the computed value to a minimum, writes it to the AUD2PER hardware register, and then decrements a counter pointed to by A0.
- `init_audio_channel_2_2`: The routine writes a series of values to a global variable and then returns.
- `init_audio_channel_2_3`: The routine writes a series of values to a set of global variables and then returns.
- `decrement_sound_timer`: The routine decrements a global counter and, when it reaches zero, reloads it from a table entry and adjusts the reload value.
- `set_audio_volume`: The routine writes to a global variable and then clears it, performing no observable audio or hardware work.
- `init_game_state_2`: The routine writes a series of constant values to a set of global variables, setting several of them to the same value and initializing the rest with distinct constants.
- `reset_game_state_flags`: The routine writes a series of constant values to a set of global variables, initializing them to fixed defaults.
- `initialize_audio_channel_five`: The routine writes a series of constant values to a global variable and then returns.
- `init_game_state_4`: The routine writes a series of constant values to a set of global variables and then returns.
- `advance_dialogue_pointer`: The routine reads a short from a table pointed to by a pointer stored in memory and writes that short to a location in memory.
- `read_word_and_advance_pointer`: The routine reads a 16-bit word from a pointer and advances the pointer.
- `advance_dialogue_text`: The routine copies a byte from a pointer stored in a global variable into a byte pointed to by a register, then advances the stored pointer by two bytes.
- `read_next_byte`: The routine reads a byte from a pointer and advances the pointer.
- `trigger_damocles_cutscene`: The routine checks a global variable and, when a condition holds, calls another routine several times, reads a word from a table indexed by a register value, computes a small value from it, and writes several words to global variables; otherwise it writes a few values to global variables and returns.
- `process_player_input`: The routine reads a short value from a structure pointed to by A0 and dispatches on its value, calling another routine and updating a global variable and several fields in the pointed-to structure.
- `update_player_state`: The routine checks a global flag and a counter and returns early if conditions are met, otherwise it clears a range of entries and then updates a global counter and may call another routine.
- `clear_expired_alarms`: The routine walks a table of entries backwards, comparing a global variable to a byte field in each entry.
- `clear_enemy_state`: The routine clears a byte flag in a global table entry and zeroes two other fields associated with the entry.
- `remove_enemy_from_list`: The routine clears a flag in a global table entry indexed from a register value and zeroes two associated fields in memory.
- `reset_game_state`: The routine calls another routine, then writes four values to a global variable in sequence, ending with a zero, and returns.
- `process_player_death`: The routine clears a set of flags in memory, calls clear_expired_alarms, then uses a global variable as an index into a table to load configuration words and run a loop that clears flags for a range of entries.
- `update_enemy_ai_state`: The routine checks a flag in a global variable; if set it runs a cleanup sequence that clears enemy state, masks a byte, zeroes fields, calls two other routines and processes player death, then resets globals.
- `update_enemy_ai_state_2`: The routine processes a global variable and a table at a fixed address, repeatedly computing indices from 16-bit values, looking up short entries in a table, and invoking another routine multiple times.
- `increment_game_tick`: The routine increments a global variable, calls another routine, inverts the global variable, then clears it and writes a byte pattern to a small RAM area by comparing a shifted value with entries in a table and storing a decrementing byte when a match occurs.
- `map_input_keycode`: The routine receives a 16-bit value in D0, checks whether its low byte lies between the ASCII codes for 'f' and 'q' (exclusive of 'f' and inclusive of the upper bound).
- `init_game_state_5`: The routine reads a global variable, derives an index, stores two bytes from a table and a global array, calls another routine, then iterates over a byte array using a mask from a table to build five 32-bit accumulators, rotates them by a signed word value, splits the results into low and high words and writes them into two tables, with additional conditional word merges.
- `update_player_stats`: Entry is reached from map input handling and a player state update, and it calls another state routine and an error display routine.
- `set_game_state_2`: The routine calls another routine and stores the result in a global variable, then returns.
- `apply_palette_shift`: The routine reads a global variable, updates it by indexing a table via another routine, then copies 32 words from one table to another shifting each word right by the original variable masked to six bits.
- `set_palette_flag`: The routine ORs a byte at the address in A0 with a constant and returns a value built from bits of D0 combined with a constant.
- `compute_sprite_offset`: The routine takes a 16-bit input value, XORs it with a global variable, masks the result, adds it back to the XORed value, then masks and adds again to produce a 16-bit result.
- `jump_to_compute_sprite_offset`: Jump stub: passes control straight on to compute_sprite_offset.
- `run_game_loop`: This routine starts a cutscene, then branches on a global variable.
- `copy_tilemap_data`: The routine copies a small block of tilemap words from a source table selected by a bit in a global variable into two destination tables.
- `show_trade_error_message`: The routine calls decode_map_tiles twice and then returns.
- `update_screen_tiles`: The routine updates a set of tile map words by rotating five 32-bit accumulators with a global variable and writing the resulting words to two tables, then ORs and ANDs the map words with the accumulator bytes.
- `update_cipher_state`: The routine updates on-screen digit display from a global variable and runs a five-parallel 32-bit stream cipher loop over a table driven by a code pointer.
- `update_timer_display`: The routine clears and initializes several global variables, computes seconds and sub-second digits from a global timer value and writes ASCII digits to screen RAM, then runs a cipher-like mixing loop over a table driven by a code pointer and writes the resulting 16-bit words to two tables and updates a global variable table.
- `decrypt_game_data`: The routine updates five 32-bit rolling masks from bytes in the program image indexed by a global variable, rotates the masks by a signed 16-bit value taken from a global table, and writes the resulting high and low 16-bit halves to two tables.
- `run_game_timer`: The routine calls another routine to decode map tiles and then returns.
- `update_game_state_flag`: The routine reads a global variable and, if it is not equal to two and not equal to zero, sets a local copy to a constant.
- `shift_palette_and_update`: The routine shifts a global byte array left by one and updates a global variable with a table lookup derived from an input byte, then calls another routine to set a score digit and resets audio volume.
- `init_game_state_6`: The routine copies a block of bytes from a fixed source into a global variable array by iterating backwards, then updates a global variable using an indexed table lookup and arithmetic, calls a routine to set a score digit, resets audio volume, and conditionally calls two other routines before returning a value derived from the audio reset result.
- `set_score_digit`: The routine takes a word from d0 and a pointer in a0.
- `shift_cipher_decrypt`: The routine updates a small rotating buffer of global variables depending on a flag, then builds four 32-bit accumulators from bytes indexed by a global variable and rotates them by a value derived from a word table.
- `render_tilemap_layer`: The routine builds five 32-bit masks from a table of bytes indexed by a global variable, rotates the masks by an amount taken from a table of signed words, extracts low and high 16-bit halves, and writes those halves to two tables of words at fixed offsets.
- `compute_fixed_point_sqrt`: The routine reads a signed fixed-point value from a global variable, normalises it to a 32-bit representation and computes an integer square root using a shift-based approximation.
- `compute_fixed_point_sqrt_2`: The routine reads a signed fixed-point value from a global variable, adds a constant offset, and computes an approximate square root using a shift-based method with sign handling and overflow checks.
- `update_displayed_score`: The routine reads a global variable, converts it to a fixed-point value, compares it with another global and, if changed, indexes a table with the high word of the value and a scaled low part to build two 16-bit values written to memory and updates the global variable before calling another routine.
- `update_display_timer`: The routine checks a global variable against itself and, when the condition is taken, writes a constant to a global variable, possibly changing it based on the sign of another global variable.
- `update_seed_display`: The routine checks a global variable against itself, writes a constant to a global byte and conditionally changes it, then iterates eight times writing a byte derived from shifting a copy of the global variable right by 28 bits into consecutive bytes of a table, updating the copy each iteration by masking and multiplying by ten, and finally calls another routine.
- `convert_score_to_bcd_and_display`: The routine checks a global variable and returns if it is not set.
- `format_score_bcd`: The routine clears a few global variables, adds a 32-bit value to a BCD accumulator using repeated digit-wise addition with carry and BCD adjustment, then converts the BCD result to ASCII digits stored backwards at a fixed buffer, strips leading zeros, and copies the resulting characters into a fixed display area with separators.
- `decrypt_game_data_2`: The routine processes a small fixed-size block of bytes stored in a table and a global variable.
- `verify_password_entry`: The routine compares five bytes from a table with five bytes from a global variable, subtracting each pair and applying a BCD adjustment, and sets a borrow flag based on the comparison.
- `update_hex_display`: The routine reads a global variable and a value from the stack, combines them, and if a self-comparison test on the global variable is true it writes a series of nibbles derived from the global variable into a table and then writes three bytes derived from the combined value into the same table, after which it calls another routine.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `set_global_variable_2`: Writes a constant value to a global variable.
- `sync_game_state`: The routine compares a global variable against itself twice, each time assigning the variable to itself and then calling load_level_data, followed by either apply_palette_mask or clear_player_state.
- `clear_game_state_buffers`: The routine clears multiple 16-kilobyte blocks in two memory tables and writes two word values from RAM into each table, then toggles a global variable.
- `clear_double_buffer_tables`: The routine checks a global variable and, if it differs from a stored value, clears large blocks of a double-buffered table pair to zero and sets specific words to sentinel values, then manipulates the global variable through bitwise inversion, finally returning.
- `update_time_display`: The routine resets a game state and, when a condition on a global variable holds, copies a global variable to itself, derives two scaled values from a global variable using a fixed-point multiply and shift, splits each into hundreds and tens/units digits and stores the four digit bytes into RAM, then calls another routine to decode a sprite mask.
- `do_nothing`: This routine contains a single instruction and performs no operations.
- `synchronize_input_state`: The routine resets game state and then performs three identical checks on a global variable compared with itself.
- `load_level_data`: The routine contains instructions but ends with an unconditional return, so it does not perform any observable work in the recovered version.
- `clear_player_state`: The routine clears or restores two player state blocks pointed to by a register and a global pointer.
- `set_palette_for_both_players`: The routine masks and combines five 32-bit values in D3-D7 with two tables pointed to by A1 and A5, clearing low nibbles and setting high nibbles in several entries of each table and then AND-ing palette entries with derived masks from D3-D7 when D0 is non-negative.
- `apply_palette_mask`: The routine masks and then combines five 16-bit palette entries in two tables pointed to by a1 and a5.
- `apply_bitmask_and_rotate`: Iterates through a table to apply bitwise AND and bitwise rotation or shifting operations to a set of buffer values.
- `decode_sprite_mask`: The routine builds four 32-bit masks from a byte table indexed by the input bytes in A0 and writes the rotated masks to two word buffers in A1 and A5.
- `decode_map_tiles`: Routine iterates over map tile entries using A0, A1 and an offset in A4 to build five 32-bit rolling buffers from bytes taken eight at a time, rotating them by an amount taken from a table in A2 and writing derived words into the destination table in A1 and a second table in A5.
- `update_player_position`: The routine reads a global variable, combines it with values from memory and a table lookup to compute an angle delta, then sets a global variable based on the sign of the result.
- `set_difficulty_flag`: The routine calls another routine twice, reads a 16-bit value from another routine, XORs it with a constant and stores the result into a global variable.
- `compute_fixed_point_sine`: The routine takes two 32-bit fixed-point values, makes them positive by adjusting a quadrant flag, forms their difference, and uses the difference to index a sine table and a cosine table via a global variable and another table.
- `wait_for_input_event`: The routine loops while a global variable is non-negative, checking a short from a table indexed by the variable.
- `decrement_counter_and_spawn`: The routine repeatedly checks a short value stored in a global variable and decrements it while it remains greater than -1, then calls another routine and returns its result.
- `decompress_level_table`: Routine clears a table of 32-word blocks to zero then expands a packed byte stream into a larger table using nibble arithmetic and offset calculations.
- `check_collision_response`: The routine reads a byte from a table pointed to by a global variable and, depending on the byte and the sign of an input value, computes a signed offset from a base value and compares it with a scaled magnitude.
- `write_palette_entry`: The routine takes a value from a register and uses it as an index into a table pointed to by a pointer register.
- `draw_tile_glyph`: The routine reads a byte from an input structure and, depending on its value, updates a small word table via a pointer, performs fixed-point arithmetic, collision checks, damage application and palette writes, and manipulates a global counter.
- `apply_damage_effect`: The routine reads a byte from a table indexed by a global variable and a register value, then conditionally writes four-byte entries from two tables through a pointer, updates a global variable and increments a pointer, and sets a global variable based on a masked value.
- `apply_tile_effects`: The routine reads a byte from a table indexed by a short offset and a short index, and if the byte is non-zero it extracts a 3-bit field and conditionally copies 32-bit entries from two tables into a destination pointer, updates a global variable and increments the pointer.
- `increment_score_counter`: The routine conditionally calls another routine if a global variable is negative.
- `compare_fixed_point_values`: The routine reads two fixed-point values from a global variable and from a pointer, normalises their order, performs a fixed-point scaling and rounding operation on each pair, and then compares two table lookups indexed by the resulting values.
- `compute_16bit_fixed_point_delta`: The routine reads 16.16 fixed-point values from two structures via a table, forms a delta, looks up a signed offset in a table, adds a base value from a table, clamps the result to a range taken from another structure and performs a small fixed-point rounding/normalisation loop.
- `apply_player_damage`: The routine reads a byte from a structure pointed to by a register and checks a flag.
- `decode_player_movement`: The routine decodes a movement value passed in registers using a table in memory referenced by a pointer, performs arithmetic and bit manipulations on the resulting indices, and stores a short result into a global variable.
- `process_input_command`: The routine reads a null-terminated string from a pointer, draws each character as a tile glyph and updates a score counter.
- `process_input_command_2`: A routine that reads characters from an input buffer and renders tile glyphs for the non-empty characters, then updates a score counter and runs a short sorting pass over a small high-score table; it also walks a list of 16-bit values using a fixed-point interpolation loop and updates a global variable with computed results.
- `parse_input_file`: The routine reads a byte stream from a pointer, draws tile glyphs for each non-null byte and updates a score counter, then performs a series of table lookups and arithmetic on several global variables and memory tables, including a loop that copies and compares 16-bit entries and manipulates a fixed-point interpolation value before returning.
- `process_input_command_2_2`: The routine reads a command string from a pointer, draws glyphs for each character and updates a score counter.
- `draw_scanline_sprite_2`: The routine processes a scanline sprite by iterating over a table of word entries and maintaining fixed-point accumulators for horizontal and vertical positions.
- `initialize_game_state_3`: The routine calls another routine and then returns.
- `handle_player_death`: The routine checks a signed offset derived from an input word and a signed input value and a pointer, conditionally calls a collision routine and branches to a common tail.
- `set_palette_entry`: The routine checks a short value in D0 and, if negative, calls another routine and stores a return address and a saved register on the stack before continuing.
- `decode_map_tile`: The routine decodes a map tile by performing a series of arithmetic and bit manipulations on values loaded from a global table and from registers, using intermediate stack storage and a helper routine.
- `compute_sprite_collision`: The routine reads several values via a helper function and uses them as indices into a global variable and a table to build 32-bit values, then performs arithmetic and bit manipulation on those values, including shifts, masking and conditional adjustments.
- `dispatch_game_state`: The routine loads an offset from a register and uses it to index a table of code pointers, then performs an indirect call through that table using clear_object_flag as a base.
- `clear_object_flag`: The routine computes a base pointer from a global variable and two tables indexed by the inputs, then clears a single bit in four bytes spaced 8KB apart starting at that base.
- `clear_tile_collision_flag`: The routine computes a byte pointer from a global variable and two input values using a table lookup, then clears a bit in the byte at that address and sets the same bit in a byte 8000 bytes later, clears the bit in a byte 16000 bytes later, and sets the bit in a byte 24000 bytes later.
- `set_tile_collision_flag`: The routine computes a byte pointer from a global variable and two input registers using a table lookup, then sets or clears a single bit in four bytes spaced 8 KiB apart in the pointed-to block.
- `clear_object_flag_2`: The routine computes a byte pointer from a global variable and two tables indexed by the inputs, then clears a bit in the byte at that pointer and in the byte 8000 bytes later, and sets the same bit in the bytes 16000 and 24000 bytes later.
- `clear_bit_set_flags`: The routine computes a byte pointer from a global variable and two tables indexed by the inputs in D0 and D1, then clears a single bit in the byte pointed to and sets the same bit position in three other bytes located 8000, 16000 and 24000 bytes after it.
- `set_collision_bitmask`: The routine computes a pointer from a global variable and two tables indexed by the inputs, then sets a bit in four bytes spaced 8000 bytes apart starting at that pointer.
- `update_sprite_collision`: The routine computes a fixed-point scaling factor from a global variable and then iterates over a table of short values, writing a derived index into the table and clearing a screen buffer pointer.
- `update_enemy_ai_state_3`: The routine returns early if a global variable is non-zero and performs a fixed-point scaling operation on that variable using shift and sign checks.
- `reset_game_state_5`: The routine computes a flag from a global variable using a masked addition and a bitwise AND, then conditionally clears or sets several global variables and returns the flag.
- `clear_collision_table_and_check`: The routine clears a collision table in memory and walks it to find matching entries.
- `compare_tile_heights`: The routine walks a short table pointed to by a pointer in A1, adding the difference between a global short and each table entry to a global short and testing the result against zero.
- `process_level_objects`: The routine clears a global flag, iterates over a linked list of short values, walks a 16-element short table to find a matching entry, updates a counter in a buffer routine, and then scans a table of short pairs using arithmetic on the current index to decide when to call another routine, returning a computed short.
- `clear_screen_buffers_2`: The routine writes a repeated short pattern to a table of words, then clears a block of bytes in two passes while incrementing a byte counter, and finally fills a global variable and a caller-provided output area with values returned by another routine until a termination condition is met.
- `clear_screen_buffers_3`: The routine clears a short table by writing a series of increasing values and then clears two overlapping byte regions in memory by writing a byte counter that increments each iteration.
- `zero_audio_channel_buffers`: The routine writes constants to a series of global variables, then clears a word buffer by repeatedly storing zero words starting at its own address until reaching another routine, and returns a constant.
- `clear_word_buffer`: The routine clears a word buffer.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `set_game_over_pointer`: The routine writes the address of a global variable into the same global variable twice, effectively storing a self-referential pointer.
- `copy_game_data_to_ram`: The routine sets a game-over pointer, initializes audio channels, copies a block of longwords from ROM to a RAM table, then copies a block of words from a RAM table to another RAM table using a strided pattern, clears screen buffers, copies a block of longwords from one RAM table to another with interleaved offsets, and finally calls another routine to copy household items.
- `clear_screen_buffers_4`: The routine clears two screen buffers by writing zeros to three groups of words per buffer and writing a fill pattern to a fourth group, repeating for a fixed count.
- `set_enemy_sprite_attributes`: The routine writes to a global variable twice and then iterates six times, OR-ing a constant into two 16-bit words in two tables spaced by a fixed stride.
- `toggle_wish_miracle_text`: The routine toggles the colour of the text "ANT YOU A WISH - NOT A MIRACLE" by masking or setting the high byte of six 16-bit words in two tables.
- `clear_enemy_flags`: The routine checks a global variable and, if it is non-zero, clears it and then iterates six times, OR-ing a constant mask into two pairs of 16-bit words in two tables spaced by a fixed stride, advancing both pointers each iteration.
- `toggle_planet_text`: The routine toggles a six-entry pair of 16-bit table words by clearing or setting a bit mask, using a global variable as a toggle flag.
- `toggle_player_status_icons`: The routine checks a global variable against itself and, if the condition is met, iterates six times over two tables of words.
- `decrement_timers_and_clear_states`: The routine walks a set of timer entries in a global structure.
- `reset_game_state_6`: The routine clears a number of global variables, then uses a value derived from a global variable to compare against entries in a table.
- `calculate_weapon_stats`: The routine computes a fixed-point value from a global variable and a table, stores the result through an output pointer, and repeats a similar calculation for a second output location.
- `process_legislation_summary`: The routine checks a global variable and, depending on its value, calls game-over handling, state verification, player and entity state copying, palette rotation and several game-state initialisers.
- `store_player_state`: The routine writes the value in a register to a global variable offset by a register, then writes two other register values to two other locations offset from the same base register.
- `copy_entity_state`: The routine copies two 32-bit values from a source location to a destination location.
- `empty_routine`: The routine contains instructions but performs no observable work; it ends without reading or writing any memory and without calling another routine.
- `uninitialized_memory_clear`: The routine contains sixteen instructions but performs no observable work and immediately returns.
- `rotate_palette_entries`: The routine iterates over a table of 16-bit palette entries stored in a global variable.
- `reset_game_state_and_wait_for_input`: The routine resets a global variable and runs a short visual effect loop that waits for vertical blank and updates a table of words with a masked decrement pattern, then starts the main game loop and waits for a flag to clear.
- `burn_cpu_cycles`: The routine contains a loop that decrements a local counter from a large initial value down to zero, performing no other operations.
- `execute_initialization_sequence`: A large routine that calls several subroutines to clear buffers, copy or decompress data, and update hardware or game state parameters; it is called during game or state resets.
- `set_enemy_spawn_limits`: The routine calls another routine and then writes to a global variable twice.
- `set_boss_phase_timers`: The routine calls another routine and then writes a constant to a global variable twice.
- `set_enemy_spawn_bounds`: The routine calls another routine and then writes to a global variable twice.
- `clear_screen_buffers_5`: The routine iterates over a table of pointers stored in a global variable, using a counter that starts at a positive value and decrements to -1.
- `init_audio_channels_2`: The routine writes a sequence of constant values to a global variable and then returns.
- `update_audio_periods`: The routine reads a global variable and a RAM location, combines them into an index, fetches a value from a table, advances the pointer, masks and scales the value, clamps it to a minimum, repeats the scaling and clamping, then writes constants to the global variable.
- `set_game_state_pointer`: The routine calls another routine, then stores a constant value into a global variable and returns.
- `update_enemy_ai_state_4`: The routine updates a table of enemy positions and flags by iterating over a global list, applying a series of arithmetic and bit manipulations to each entry, and then writes the results back to memory.
- `clear_sprite_collision_flags`: Clears a specific bit in a sequence of three adjacent bytes using a bitmask derived from a register.
- `set_tile_collision_bitflags`: The routine sets a bit in three byte arrays at offsets 0, 8000 and 16000 using a bit index derived from an input value, clears a bit in the third array, and if a flag is non-zero it reads a short from a table, builds a 32-bit value from several inputs and a table lookup, stores it through a pointer, then calls another routine via an indirect jump through a table indexed by an input value and a short.
- `set_collision_flags`: The routine sets a bit in a byte pointed to by a pointer, clears the same bit in a byte 8000 bytes later, and sets the same bit in a byte 16000 bytes later.
- `clear_bit_and_set_flags`: The routine clears a bit in a byte pointed to by a pointer and sets the same bit in two far words of the same block, then returns if a word is zero.
- `set_collision_bit`: The routine sets a bit in three byte arrays at offsets zero, eight thousand and sixteen thousand using a value from a register masked to the low three bits, then returns if a word is zero.
- `reset_player_state`: The routine checks a global variable and returns early if the condition is met.
- `process_event_dispatch`: Checks a global event flag and dispatches or executes logic based on its value, specifically setting a player spawn flag or a man report flag and updating a global state or pointer.
- `set_string_terminator`: The routine receives a pointer in A0, walks forward byte by byte until it finds a zero byte, then writes a constant 1 to a global variable and writes the low bit of the pointer after the terminator to the same global variable.
- `clear_game_state_flag`: The routine writes zero to a global variable and then writes a 16-bit value taken from address register A0 to the same global variable, overwriting the zero.
- `pop_stack_pointer`: The routine reads a byte from a global variable, then writes a pointer formed by concatenating that byte with the next two bytes from the same global variable into the global variable.
- `decode_input_byte`: Checks a pointer and a global variable, then either loads a short from an offset or updates a global or a pointer-based value based on bitflags.
- `set_target_sprite`: The routine reads a byte from a table pointed to by a pointer and a global variable, conditionally toggles a bit in the byte, and then updates a global variable with values derived from the pointer and the table.
- `store_today_marker`: The routine writes a short from a pointer-based source into a location computed from the same pointer plus a fixed offset, then stores the pointer advanced by four bytes into a global variable.
- `wait_for_vblank_2`: The routine contains an infinite loop with no operations.
- `update_game_timer`: The routine XORs a value with a global variable and then, if a condition on a byte is met, stores a constant in a global variable and returns.
- `update_audio_interrupt_state`: The routine reads a global variable, masks and rewrites it, and conditionally clears and sets the global variable to zero and a non-zero value.
- `reset_audio_channel_2`: The routine writes zero to a global variable, then writes a constant to the same global variable, and returns the value it received in a register.
- `set_interrupt_request_3`: The routine writes a constant value to a global variable and returns.
- `decrypt_and_load_game_state`: The routine copies a block of words from a global variable into a set of RAM locations, then branches through a table of addresses.
- `update_game_state_4`: The routine initializes a set of hardware registers and a global variable, then calls a timer reset and several subroutines including audio and player updates.
- `copy_household_items`: The routine copies fourteen 16-bit values from a source table to a destination table by repeatedly reading a word, writing it to the destination, and advancing both pointers.
- `check_disk_error_indicator`: The routine contains no instructions and returns immediately.
- `update_enemy_position`: The routine updates a table of enemy entries using a fixed-point step and a loop over the entries.
- `check_game_over_conditions`: The routine tests a global variable with two comparisons and, when the condition holds, calls another routine twice, then sets a flag via another routine.
- `advance_music_step`: The routine updates a global variable that tracks a music step counter.
- `play_voice_sample`: The routine contains a loop that copies a block of bytes from one area of memory to another.
- `update_player_status_flags`: The routine sets a byte at an offset from a pointer in A0 using D0 and then updates a global variable by incrementing or decrementing it while toggling bits in four bytes derived from a table entry, toggling bits in two other bytes derived from a table entry, and toggling bits in two other bytes derived from a table entry, returning early in one branch.
- `update_laboratory_at_06_05_bare_island`: The routine adjusts a global variable toward three thresholds and, for each threshold, updates two pairs of bytes in memory by setting or clearing specific bits depending on whether the variable is below or above the threshold.
- `initialize_game_data_arrays`: When a global variable is non-zero the routine copies blocks from a table to two arrays and then writes several words from the same global variable into fixed offsets of those arrays.
- `copy_game_state_tables`: The routine copies blocks from a source table to two destination tables when a global variable is non-zero, then conditionally writes parts of that global variable into specific offsets of the tables and updates the global variable.
- `update_player_palette`: The routine copies a set of palette entries from a table indexed by a masked global variable into two groups of memory locations.
- `update_ship_graphics`: The routine reads a masked value from a global variable and, if it differs from a stored value, updates the global variable and copies a block of word entries from a table into two sets of memory locations.
- `set_enemy_ai_state`: The routine clears a global variable and then scans a table of short values backwards, comparing each entry to a value derived from the global variable.
- `copy_map_tiles`: Iterates through a table to find a matching tile pattern and copies a sequence of bytes or words to a new destination, while adjusting a pointer.
- `initialize_enemy_table`: The routine checks a global variable and, if non-zero, iterates over a number of entries equal to its value.
- `update_screen_palette`: The routine clears and sets global state, repeatedly invokes a helper routine three times and then calls another routine that computes weapon damage.
- `process_message_queue`: The routine initializes a global variable based on its sign, then processes a byte stream from it through a loop that calls another routine repeatedly under a condition and then calls calculate_weapon_damage.
- `copy_string_to_buffer`: The routine copies a null-terminated byte string from a pointer in memory to a global buffer, byte by byte, stopping when a byte with the high bit set is encountered.
- `copy_name_terminated_string`: The routine copies bytes from a source pointer in A1, using the value in D0 as the first byte, into a global variable at a fixed address, continuing until a byte with the high bit set is encountered.
- `update_screen_palette_2`: The routine reads a byte from a table and uses it to index into a table, then performs a series of arithmetic and bit operations on values obtained from another routine and from memory, producing a 32-bit result that is written to a location pointed to by a pointer.
- `calculate_screen_scroll`: The routine reads a byte from a table pointed to by a global variable and repeatedly processes pairs of values obtained via another routine, combining them with table lookups and arithmetic to compute a 32-bit scroll value.
- `draw_text_line`: The routine processes a null-terminated byte string pointed to by a register, using a global variable and two tables accessed via A5 and A6 to compute 32-bit values that are combined with shifts, masks and conditional arithmetic, then writes the resulting byte to memory via a pointer.
- `process_input_state`: The routine loops over a table pointed to by a register, skipping entries whose first word is zero and whose second word is zero, calling another routine to draw a player sprite while scanning.
- `check_player_collision`: The routine reads a byte from a global variable and compares it with a constant, then loads a short from a global table and performs arithmetic on it.
- `draw_hud_element`: The routine loads a byte from a global variable and stores it through a pointer, then calls another routine.
- `draw_player_sprite_2`: The routine reads a byte from a global variable and a short from a global variable, combines them with arithmetic on a global table, and stores the result through a pointer.
- `draw_player_sprite_3`: The routine draws a player sprite by accessing a table and copying bytes to a global variable, updating the screen palette through a call to another routine, and then returns.
- `jump_to_draw_player_sprite_2`: Jump stub: passes control straight on to draw_player_sprite_2.
- `jump_to_update_enemy_ai_state_2`: Jump stub: passes control straight on to update_enemy_ai_state_2.
- `draw_boss_sprites`: The routine is a large, mostly unreachable block of 68000 code that contains many conditional branches, table lookups and calls to other routines for collision, input, palette and script processing.
- `tiny_ori_3`: Small routine of 1 instruction that merges 1 into d1.
- `find_active_room`: Iterates a table of rooms to find one containing a point, then clears a 16-word table and builds a zero-terminated list of room objects from a script table, marking object slots as free.
- `find_active_object`: This routine scans a table of objects and selects one whose stored position range contains the given coordinates.
- `update_room_state`: Iterates through a list of objects to check if certain bounds or conditions are met, then clears or updates a table of values.
- `process_level_script`: The routine initializes a global variable and processes a table of sprite entries, transforming vertices and computing bounds, then builds a list of script commands by copying bytes from a script buffer into a command buffer and a pointer table.
- `build_text_line_buffer`: This routine processes a text buffer by iterating over a table of pointers and copying or moving characters into a destination buffer while updating a linked list of pointers.
- `build_text_display_buffer`: The routine clears a global flag and walks a table of short entries, copying selected entries into a buffer and invoking a sprite transform routine in a loop.
- `transform_sprite_vertices`: The routine writes two scaled 16-bit values into a short buffer using a multiplier from D1, then computes two pairs of 16-bit sums from A1 offsets scaled by D0 and D2 with base values in D5 and D6.
- `compute_sprite_bounds`: Calculates a rectangular area by multiplying or adding offsets and writes the resulting coordinates or dimensions to a memory location.
- `write_sprite_bounds`: The routine processes a table of sprite records pointed to by a pointer in a register and writes derived bounds to an output buffer pointed to by another register.
- `process_game_state`: Iterates over a text buffer to build lines using a buffer routine, then walks a zero-terminated pointer table, copying each name-terminated string.
- `increment_timer_table`: The routine walks a global table of pointers.
- `execute_script_line`: Parses a command buffer or scriptlet, calculating an offset or pointer, and iterates through a sequence of encoded instructions or data, potentially calling a decoding routine or a score-comparison routine based on a conditional check or a flag in the instruction byte.
- `execute_script_command_sequence`: Parses and executes a sequence of commands from a script pointer, updating a global offset or index.
- `process_script_command_2`: The routine reads a script command block from a pointer, copies several fields into global variables, resolves a table index using a loop over a table, and then either decodes a script command or iterates over a byte stream, dispatching each byte through a lookup table to call another routine repeatedly and conditionally updating two accumulator values.
- `process_input_command_3`: Routine reads a packed command block from memory, updates several global variables from the block, computes an index from a flag field and walks a table to resolve an input mapping, then dispatches a script command or iterates over a byte stream, looking up handlers via a table and invoking a comparison routine or a self loop per entry.
- `decode_script_command`: The routine reads a byte stream from a pointer, looks up a table entry for each non-zero byte and walks a variable-length entry, calling another routine repeatedly while a signed byte condition holds.
- `process_input_table`: Iterates over a table of input entries, dispatching each entry to a handler based on a flag bit and a byte value, and updates a global variable and a pointer into a table for the next entry.
- `handle_game_over`: The routine is empty.
- `draw_damage_flash`: The routine contains 16 instructions but performs no observable work; it ends with a return without modifying registers, memory, or calling other routines.
- `handle_game_over_2`: The routine contains a sequence of 19 instructions but performs no observable work with memory or registers; it ends with a return without side effects.
- `draw_player_sprite_4`: The routine loads a global variable, updates several global variables, then calls another routine to check a level flag and another to initialize game state.
- `shift_global_variable`: The routine reads a global variable, and if it is not equal to a constant, it shifts the variable left by one bit and writes it back.
- `check_level_flag`: The routine scans a table of short values starting at a global variable until it finds an entry whose low byte matches a value derived from a register.
- `reset_palette_and_state`: The routine updates the screen palette via another routine and then manipulates a global variable with a series of range checks, assigning fixed values in different branches and forcing the variable to zero on the fallback path.
- `update_game_state_5`: The routine clears collision flags, resets game state and initializes several global variables, then processes input and updates a set of memory locations derived from a self routine result.
- `reset_game_state_7`: The routine writes a constant value to a global variable twice, then writes zero to three global variables.
- `set_player_spawn_flag`: The routine checks a global variable and a byte at an offset from a pointer in D1.
- `set_man_report_flag`: The routine checks a byte at an offset from a pointer in D1 and, if non-zero, manipulates a global variable and a table indexed by D1.
- `guard_today_marker`: A routine that checks a global variable and, if the variable is zero, calls another routine to set a today marker and then sets the variable to one.
- `init_game_state_8`: The routine checks a global variable and, if it is zero, computes a pointer from a register value, reads a word from a table, combines it with the pointer’s high word to form a new value for the global variable, then sets the global variable to one.
- `set_today_marker`: The routine computes a pointer from a short value in D1 and a fixed base, then reads a word from that table entry and combines it with the high word of the pointer to form a 32-bit value.
- `reset_audio_volume`: The routine checks a global variable and, if it is zero, writes a constant to it and then tests it again; if the test succeeds it clears the variable to zero.
- `loop_write_to_memory`: The routine runs an infinite loop.
- `apply_palette_effect`: The routine computes two bounds from a global variable and a register value, then iterates over a range calling another routine twice per inner iteration.
- `set_player_score`: The routine receives a short value in D1 and a pointer in D7 and a pointer in A4.
- `update_game_state_table`: The routine scans a table pointed to by a global variable and calls another routine for each entry that is non-zero, then walks a second table calling a player/enemy compare routine for each entry and optionally once more.
- `reset_input_state_and_apply_palette`: The routine clears a global variable, then conditionally recombines two nibbles from it into a new value or sets it to zero, calls another routine, optionally processes input when the value equals one, loads a byte from a table indexed by the high bits of the variable and updates the variable and triggers a palette effect if they differ, and ends with three self-assignments to the global variable.
- `update_game_state_8`: The routine updates a global variable from a constant or a value returned by another routine, then normalises a byte derived from that variable and calls a player update routine, optionally processing input.
- `update_player_state_3`: The routine processes a player index from a register and updates several global variables from a per-player record, then computes a screen position using a table lookup and a palette update.
- `format_score_display_3`: The routine converts a 32-bit value taken from a table indexed by a global variable into packed BCD, then unpacks the BCD into ASCII digits stored backwards at a fixed buffer, strips leading zeros, and writes the resulting characters into a fixed display area with separators.
- `format_score_display_4`: The routine extracts a 16-bit value from the incoming register, combines it with a global variable to form a 32-bit value, then writes eight decimal digits into a fixed memory area, alternating the write position each iteration.
- `wait_for_flag_then_continue`: The routine spins until a global variable is zero, then calls another routine to set a today marker and writes one to the same global variable before returning.
- `wait_for_daily_flag`: The routine calls another routine to set a daily flag and then spins until a global variable becomes zero, after which it returns.
- `set_daily_flag`: The routine spins until a global variable is cleared, then computes a pointer from a short in D1 into a table and writes a combined value into a global variable, followed by storing two constant values into the same global variable.
- `copy_legislation_text`: The routine copies a block of 8000 words from a table pointed to by a global variable to another area obtained by XORing the global pointer with a constant, using a loop that decrements a counter from 7999 down to -1.
- `check_input_and_confirm`: The routine resets audio volume, waits for a flag to clear, reads a keypress and stores an adjusted value in a global variable.
- `handle_legislation_key_input`: The routine resets audio, waits for a flag to clear and reads a key press.
- `poll_joystick_input`: The routine repeatedly clears a palette buffer, triggers a disk load routine twice with a short delay between them, toggles a bit in a global variable, polls the joystick button, and waits for game frames until a condition on two output registers is met, then returns the global variable.
- `wait_for_vblank_and_copy`: The routine loops, calling a disk load starter and a copy-protection decoder, then polls input and copies bytes from a source pointer to a destination pointer until a limit counter expires or a sentinel address is reached, updating a global variable and waiting for input events between iterations.
- `poll_joystick_input_2`: The routine repeatedly attempts a disk load and a copy-protection decode, polling a joystick button between attempts while copying a small block of memory byte-by-byte and exiting early if a pointer reaches a limit.
- `clear_palette_buffer`: The routine writes a constant 16-bit pattern to successive entries of a global table using a signed counter, then stores four 32-bit values from global variables into the table, calls another routine five times, loops calling that routine until it returns zero, and calls it two more times.
- `decode_input_bits`: The routine reads a byte from a register, expands its eight bits into a 16-bit mask by testing each bit and shifting the results into the high byte positions, then writes a word to a pointer using an XOR with a word from a register and a combination of the mask and another word from a register.
- `decode_copy_protection_key`: The routine decodes a copy protection key stored in a global buffer and verifies it.
- `initiate_disk_load`: The routine writes a series of values to a global variable, calls another routine to wait for vertical blank, then writes two more values to the same global variable.
- `initiate_disk_load_2`: The routine writes several values to a global variable, calls another routine to wait for vertical blank, and then writes two more values to the same global variable before returning.
- `wait_for_vblank_loop`: The routine loads a large constant into a counter and repeatedly decrements it while checking a global variable.
- `set_ciab_portb_and_update`: The routine writes a constant byte to a global variable and then calls another routine.
- `set_ciab_portb_mask`: The routine modifies a global variable by clearing one bit and setting two bits, then returns.
- `set_ciab_prb_and_init`: The routine writes a constant byte to a global variable and then calls another routine that waits for a button release.
- `wait_for_vblank_and_poll_ciab`: The routine polls a global variable until its bit is cleared, calling another routine to set CIAB port B bits on each iteration.
- `wait_for_input_event_2`: The routine reads a global variable and, if it is clear, spins while a bit in the variable is set, repeatedly setting CIAB port B bits and then clears the variable; otherwise it waits for a vertical blank and polls a joystick button until the returned value matches the global variable.
- `set_ciab_portb_bits`: The routine modifies a global variable by clearing one bit and setting another, waits for game frames, then sets another bit, waits again, and finally decrements the variable.
- `poll_joystick_button`: The routine masks a global variable, waits for game frames, sets a bit in the variable, waits again, then increments the variable.
- `wait_game_frames`: The routine contains a decrementing loop that counts down from a constant value to zero and then returns the value originally in D0 without modifying it.
- `wait_for_button_release`: The routine polls a global variable until its bit 5 is clear, then returns.
- `reset_cia_timer`: The routine checks a global variable and, if a flag is set, performs a delay loop and clears the flag.
- `wait_for_keypress`: The routine spins until a global variable is non-zero, then clears the variable and returns its previous value.
- `reset_game_state_context`: Resets or reinitializes certain game state parameters and hardware timers.
- `reset_game_state_8`: The routine writes a series of constants to a global variable, then calls another routine, a weapon targeting routine, a game state reset routine, and an unnamed routine, and returns.
- `parse_legislation_text`: The routine reads characters from a keypress source and uses them to select a screen of legislation text, resetting audio and waiting for flags before each read.
- `reset_audio_channels_4`: The routine conditionally writes a series of constant values to a set of global variables and then returns.
- `initialize_game_state_4`: The routine writes several constants to a global variable, loads screen text, invokes another routine to set enemy AI state, invokes another routine to reset game state, and conditionally clears a global variable based on a comparison involving a global variable and a constant.
- `process_input_digit`: The routine reads a byte from a global variable offset by a saved index and, if the byte is zero and the index is zero, it performs a series of writes to global variables, calls another routine, and returns its result.
- `reset_game_state_and_handle_input`: The routine resets a set of global variables and calls several other routines when a condition is met, then returns.
- `reset_game_state_2`: The routine resets a set of global variables to fixed values and then calls another routine, an audio reset routine and a game state reset routine when a condition on a global variable holds.
- `reset_game_state_10`: The routine writes several constants to a set of global variables, then conditionally calls another routine and two more routines, and finally clears a global variable, calls a routine to set enemy AI state, calls another reset routine, and calls a routine to reset audio channels.
- `load_screen_text`: The routine is empty; it returns immediately without performing any operations.
- `jump_to_update_screen_scroll_offsets`: Jump stub: passes control straight on to update_screen_scroll_offsets.
- `update_weapon_targeting`: The routine repeatedly reads a global variable, indexes into two tables using masked and shifted forms of the variable, performs subtractions on 32-bit values formed from high words and table entries, and writes the results back to the global variable.
- `search_enemy_bounds_and_update`: Iterates through a list of enemy-related data structures to check if a target's coordinates fall within specific bounding boxes.
- `find_best_enemy_target`: The routine iterates over a table of short entries terminated by a sentinel and, using two helper calls, computes an index for each entry and keeps the entry with the smallest index.
- `process_input_and_update_state`: The routine iterates over a short table until a sentinel value is found, invoking a helper routine twice per entry and then calling another routine.
- `process_level_events`: The routine returns early if a global variable is zero, otherwise it repeatedly calls two unnamed functions and another routine until a short result equals four.
- `compute_3d_projection`: The routine computes a 3D projection using fixed-point arithmetic and table lookups.
- `normalize_fixed_point_range`: The routine loads several words from registers and a global variable, uses two tables via a pointer pair to look up 16-bit values, combines them with arithmetic and bit operations to compute a fixed-point interval, clamps and normalizes the interval with shifts and masks, and then tail-calls another routine to calculate weapon damage.
- `calculate_weapon_damage`: The routine performs fixed-point arithmetic on three pairs of values taken from a global table and a table of 16-bit entries, selects the larger of each pair, normalises the values and applies a small conditional shift and mask to produce a final 32-bit result.
- `normalize_enemy_position`: The routine reads two source values, resolves their indices through a global variable and a table, and selects a range.
- `compare_player_scores`: The routine reads two 16-bit indices from a0, loads bytes from the structures at those indices, and if the first byte is zero it repeatedly checks a byte from the second structure until it is zero.
- `set_collision_response_flags`: The routine updates entries in a short table derived from a global index and then processes two 32-bit values from structures pointed to by two arguments.
- `update_enemy_ai_state_5`: The routine updates a table of short entries using a loop that walks a linked list of indices, then performs fixed-point arithmetic on values read from two structures pointed to by registers, using shifts and sign handling to compute a result and store a byte flag in a global variable.
- `compare_player_enemy_state`: The routine compares two 32-bit fixed-point values taken from the structures pointed to by A1 and A2, using a custom division routine that works on 16.16 fixed-point numbers, and sets a flag based on the sign of the results.
- `process_game_state_two`: The routine checks a global variable and handles several flag-based branches; it may accumulate score values, adjust a global counter, skip inactive enemies and set a state, or dispatch via a jump table.
- `tiny_rts`: Small routine of 1 instruction that returns.
- `init_game_state_9`: The routine calls another routine to test a collision box and, if a condition is not met, clears and sets a global variable, calls another routine to update the player state, resets the game state, and then either sets a global variable or updates the game state before looping a fixed number of times.
- `set_game_timer`: The routine writes a constant value to a global variable and then executes a decrementing loop that counts down from a large constant to zero.
- `update_enemy_state`: The routine checks a global variable for a non-zero negative value and, if so, reads from it using pointer arithmetic and stores the result back into the global variable.
- `adjust_player_position`: The routine reads a global variable and a set of values from a structure pointed to by a register, uses two tables indexed by the global variable and by a computed index to accumulate 16-bit values, then updates the global variable with a wrapped offset.
- `decrement_global_variable`: The routine subtracts a constant from a global variable.
- `update_player_position_3`: The routine reads and updates a global variable using a table indexed by a masked value, applies conditional adjustments based on flags returned by another routine, and performs several table lookups with bit manipulation to compute an index.
- `calculate_weapon_damage_2`: The routine adjusts a global angle variable using values from a structure, then performs fixed-point table lookups and arithmetic to compute a signed magnitude, normalizes it with a shift-based scaling, and stores the result in a global variable.
- `calculate_fixed_point_sine`: The routine converts a fixed-point angle in D1 to a sine value using a fixed-point lookup table via A5 and an index table via A6, performs the fixed-point arithmetic shown, and returns the result in D1.
- `compute_fixed_point_division`: The routine performs fixed-point division with overflow and rounding handling using a global variable and two tables referenced through registers.
- `compute_fixed_point_sine_2`: The routine uses a global variable as a temporary accumulator and performs fixed-point arithmetic with two tables accessed via A5 and A6.
- `accumulate_fixed_point`: The routine uses a global variable as a table index and a second table to look up 16-bit values, combines them with a value from D1, and performs a fixed-point division-like scaling with sign handling and rounding.
- `compute_fixed_point_divide`: The routine performs a fixed-point division using a lookup table and a global variable, applying rounding and handling overflow and sign cases, then repeats the table lookup and arithmetic on the result.
- `perform_fixed_point_division`: The routine performs a fixed-point division using a lookup table and a global variable.
- `update_game_state_9`: The routine performs fixed-point arithmetic on values taken from registers and from a global variable, using a table indexed by a scrambled short and a second table indexed by a scrambled short.
- `compute_fixed_point_score`: The routine updates a global variable using a signed fixed-point multiplication and division sequence that reads 16-bit values from a table via an index derived from an input value and from the global variable, then applies a shift-based scaling with overflow handling.
- `accumulate_fixed_point_score`: The routine adjusts a global variable by a fixed-point offset using a table indexed by the high word of a 32-bit value, performs a signed fixed-point division with rounding and overflow handling, then adds the result back to the global variable and repeats a similar table lookup and division to update a value derived from an input pointer.
- `update_game_state_10`: The routine performs fixed-point arithmetic on values loaded from a table using indices derived from the inputs, accumulates the results into a global variable, and then updates a game state structure via another routine.
- `update_game_timer_2`: The routine updates a global timer value using a table lookup and arithmetic on values derived from registers and a global variable, then branches to collision and state handling when the timer reaches a threshold or underflows.
- `update_game_timer_value`: Calculates a new value for a global timer or counter using bitwise shifts and conditional checks, then checks this value against a threshold to potentially trigger a collision routine or reset the game state.
- `handle_game_over_3`: The routine updates a global variable using a table lookup and arithmetic derived from inputs, then branches to different paths based on the resulting value, calling another routine and performing a delay loop before returning.
- `update_game_timer_3`: The routine computes a 32-bit value from two 16-bit indices using a table lookup, applies a signed shift or multiplication depending on the high word, optionally negates the result based on a flag, accumulates it into a global variable and then branches to collision checks, state resets and other routines when the accumulator crosses thresholds.
- `update_game_timer_4`: The routine performs fixed-point arithmetic on a global variable and several registers, using a table lookup indexed by a computed value, and then updates the global variable with a signed result.
- `update_game_timer_5`: The routine updates a global variable by adding a signed offset derived from a flag in memory and then performs a series of conditional branches that may reset the variable, call other routines, and spin for a short period.
- `update_weapon_display`: The routine computes a scaled value from a 32-bit input using a shift amount derived from a global variable, applies sign handling based on a flag in memory, and accumulates the result into a global variable.
- `handle_game_over_4`: The routine reads a short flag from a structure pointed to by a0, updates a global signed accumulator with a sign based on that flag and a signed word from d3, and then performs a series of conditional resets and writes to global variables.
- `adjust_game_timer_state`: The routine updates a global variable by conditionally skipping inactive enemies and then performing arithmetic and shifts using values from a table and an input word.
- `update_game_timer_6`: The routine updates a global timer by adding a scaled value derived from a register pair and a shift amount, then checks the timer against a threshold and branches to collision handling, game state updates, or a busy-wait loop.
- `reset_game_state_11`: The routine clears a global variable and then either computes an index through a series of table lookups and bit operations on several uninitialized registers and stores the result back to the global variable, or it calls another routine, resets flags, invokes collision checks and player state updates, and then loops a fixed number of times before returning.
- `update_score_display`: The routine clears three global variables, then uses a table lookup chain to transform an input value and writes the result back to a global variable.
- `update_enemy_ai_state_8`: The routine updates a global variable using an input value and a 32-bit input.
- `update_score_display_2`: The routine reads a word from a register, shifts it right by a variable amount derived from a byte in another register, adds one if a bit test is true, and accumulates the result into a global variable.
- `set_game_flag`: The routine reads an unspecified 16-bit value from a register, ORs it with one, and writes the result to a global variable.
- `adjust_screen_scroll`: The routine reads a value from a register, derives a shift count from its high word and a mask, shifts a value from another register right by that count, stores the result in a global variable, and conditionally increments the stored value based on the shift count and a bit test.
- `skip_inactive_enemy`: The routine iterates over a table of short entries starting at a fixed address, advancing seven entries per iteration.
- `update_screen_scroll_offsets`: The routine calls another routine three times, reads a short value from another routine twice, computes two masked sums with a constant and stores the results in a global variable, then calls another routine once and returns.
- `calculate_weapon_damage_3`: The routine uses a table and a global variable to combine indices and look up values, then performs arithmetic and shifts on the results and writes a value to a global variable.
- `accumulate_score_values`: The routine reads a value from a global variable and from a table indexed by the incoming value, combines them with a second table lookup, and then computes a scaled result using shifts and rounding.
- `accumulate_score_values_2`: The routine computes two values using a table lookup based on a combined index derived from two global variables and an input value, applies a shift and rounding operation to each, combines them with repeated doubling and decrement, and stores the result in a global variable.
- `init_level_palette`: The routine reads a short from a table indexed by a global variable, updates the global variable, and if the value matches two specific cases it copies 20 entries from one table to another by writing each source value and its bitwise complement into the destination.
- `update_palette_and_check_day`: The routine updates a global variable using a multiplication and shift, then checks its value.
- `update_player_state_2`: The routine masks a global variable twice, then branches on its sign.
- `copy_enemy_state`: The routine copies a set of values from a global variable into several RAM locations and updates a few other globals.
- `reset_game_state_12`: The routine copies values from a global variable to several global variables and then sets another global variable to zero, conditionally overwriting it with the global variable when the global variable equals one and its bit six is clear, and finally stores the global variable to one more global variable.
- `reset_global_state_sequence`: The routine checks a global variable and, if non-zero, repeatedly calls another routine and writes the variable to a table via an index, then performs a series of comparisons and assignments on the global variable and a small table.
- `set_collision_flags_2`: The routine writes zero to a global variable and then, if the first byte of that variable is non-negative, compares the variable against the second and third bytes of the variable and updates it to 1 or 2 accordingly.
- `update_input_state_3`: The routine clears a global variable, then conditionally updates it using two input words and four table entries, applying a series of comparisons and additions to the variable.
- `check_collision_box`: The routine reads a global variable and, if it is zero, iterates over a table of short entries at a fixed address.
- `select_next_level`: The routine examines a global variable and a table entry at an offset from a level index, then steps the index by a fixed amount while skipping entries whose marker byte is zero or 'x' or whose second marker byte is non-zero, and returns the chosen index after initializing level state and resetting game state.
- `name`: The routine does not appear to have a clear purpose from the provided code.
- `init_level_state_2`: The routine copies several values from a level structure into global variables, initializes a few global flags, calls another routine to set up game state, updates player and game state through helper routines, and returns the result of a skip routine unless a condition forces an early return.
- `update_starfield_offset`: The routine updates a global variable by shifting it right, adding a constant and masking it, then reads a short from a table indexed by that variable, adds the variable to the value and masks the result again, storing it back to the global variable.
- `return_immediately`: The routine contains no instructions and returns immediately.
- `copy_tilemap_block`: The routine copies a block of long words from a source area in memory to a destination area in memory by decrementing two pointers and copying each long word until the source pointer reaches the start of the source block.
- `initialize_game_state_5`: The routine sets a global variable, copies a tilemap block, calls another routine, writes several constants to a RAM location and calls a few other routines, then clears a range of words in its own code area up to a reset routine, updates a global variable, calls an update routine and conditionally writes a value to RAM.
- `reset_game_state_13`: The routine clears a block of word memory pointed to by two registers until the pointers meet, then writes constants to a global variable and a RAM location, calls another routine, and conditionally writes further constants to RAM and a global variable, invokes several other routines, and manipulates a global variable based on a comparison flag before truncating.
- `xor_global_state_words`: The routine xors a block of words inside a global structure with a value derived from the base pointer and conditionally masks the base pointer.
- `handle_game_event_2`: The routine computes a signed fixed-point delta from a global variable using a lookup table at a fixed offset, normalises the angle with a helper, then indexes a jump table built from RAM to dispatch an event handler.
- `clear_level_buffers`: The routine checks a short counter and a word derived from two integers; if the counter is zero it returns, and if the word is below a threshold it dispatches through a table and returns.
- `clear_screen_buffer`: The routine clears a screen buffer by writing a constant value from a register to a series of words at fixed offsets within each row of the buffer, then advances the buffer pointer by a fixed amount and repeats until a counter expires.
- `save_game_state`: The routine calls fill_memory_block and save_game_state_2 several times, then stores the result of fill_memory_block into a table and copies a set of register values into multiple repeated blocks of a global variable, saving the same register snapshot in several locations.
- `fill_memory_with_d0`: The routine writes the 32-bit value from D0 repeatedly to consecutive 32-bit locations starting at the address in A6, filling a block of memory with that value.
- `save_game_state_2`: The routine copies the values currently in registers D0, D1, A0 and A1 and the values in D2-D7 and A2-A5 into a block of memory pointed to by A6, repeating the register set multiple times in sequence.
- `fill_text_display_buffers`: The routine builds two sets of per-character display entries for a text buffer.
- `build_collision_tables`: The routine reads a table pointed to by a global variable and builds several collision-related tables in memory, scaling values with two global variables and swapping pairs depending on their sign.
- `process_collision_response`: The routine validates a global variable and then processes a table entry pointed to by a fixed address.
- `map_input_to_tile_properties`: Iterates through a table of boundary-defined tile or object data to find a matching ID, then updates a secondary lookup table and clears a block of memory or attribute-related flags.
- `check_object_collision`: The routine tests whether a signed offset stored in a global variable lies inside a set of ranges defined by a global table and a small table at a fixed address.
- `test_tile_bounds`: The routine tests a coordinate against several short bounds stored in a global variable and, if inside, iterates a table looking for matching entries and may return early or adjust the coordinate via arithmetic and comparisons.
- `update_enemy_entities`: Iterates through a list of game objects to update enemy movement, AI states, and player states based on certain flags and proximity checks.
- `process_enemy_ai`: The routine processes a list of enemy entries stored in a global table and updates their movement and AI state.
- `compute_enemy_collision_bounds_and_flag`: The routine sets a global flag based on a byte in D0 and values in a global table, then iterates a fixed-size enemy list using a global base pointer.
- `set_global_variable`: The routine writes a constant byte value to a global variable and returns the value that was in D0 on entry.
- `update_enemy_animation_frames`: The routine iterates over a list of enemy structures and updates animation frame values in the structures using a set of conditional adjustments to frame counters and related fields.
- `update_enemy_state_3`: The routine reads bytes and words from structures pointed to by a register and a global variable, performs flag tests and arithmetic on those values, and writes results back to a global variable and to stack temporaries.
- `check_player_collision_2`: The routine reads fields from a structure pointed to by a0 and a flag in d3, then uses conditional loads to copy values from several offsets of that structure into a global variable and into registers.
- `check_player_collision_3`: The routine reads a set of fields from a structure pointed to by a0, masks and compares byte flags, loads and compares 32-bit values, stores several words and longs from the structure into a global variable, selects between two update routines based on a flag, and returns a 32-bit value from the structure under certain flag conditions; otherwise it falls through to further arithmetic on the structure fields.
- `update_player_state_5`: The routine reads a set of flags from a word in D3 and from bytes at offsets from a pointer in A0.
- `update_player_state_6`: The routine checks flags in a word argument and early-exits on certain byte conditions read from a structure pointed to by A0.
- `update_enemy_ai_state_9`: Evaluates flags and status-related or position-related data from two different entities or memory pointers to determine if a state update or conditional branch should occur.
- `initialize_game_state_6`: The routine saves two values from memory, writes two values to memory, sets a global variable to zero, stores a pointer to another routine in memory, triggers a trap, calls a routine twice, reads a value from memory, calls a routine again, combines the saved values with the result using shifts and XOR, and stores the result through a pointer.
- `scramble_seed_with_input`: Calculates a new value using bitwise rotation, addition, and XOR involving registers and memory-based values, then stores it via a pointer.
- `apply_joystick_rotation`: The routine calls a routine to check input state and stores the result in a local variable.
- `scramble_level_seed_2`: The routine calls a routine that checks input state, then combines values from registers D2 and D4 with a rotation and XORs the result with the return value of the input check, storing the result through a pointer in A0.
- `check_input_state`: The routine contains five instructions and performs no observable operation on registers, memory, or hardware, effectively doing nothing before returning.
- `rotate_and_xor_input`: Performs a bitwise rotation and XOR on a value derived from a register and then stores the result into a memory location using a pointer.
- `xor_memory_with_register`: XORs a register value with another register and stores the result into a memory location using a pointer.
- `tiny_sub_add_sub`: Small routine of 3 instructions that subtracts d1 from d3, then adds d3 to d5, then subtracts d6 from d2.
- `store_register_to_memory`: Stores the contents of a register into a memory location using a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_game_state_7`: The routine calls two other routines and then returns.
- `initialize_game_state_8`: The routine calls two other routines and then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_routine_2`: This routine is empty and returns immediately.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `gamepad_read_input`: The routine is empty; it contains no instructions and performs no operations.
- `dummy_routine`: The routine contains no instructions and returns immediately.
- `empty_routine_stub`: The routine is empty and performs no operations.
