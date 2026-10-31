# Game 336 (jail break (1990)(odyssey)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00336). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 35 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 156 of 156 named (0 author symbols), 156 of 156 described (1 by rule, 155 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 432 of 432 (100.00%) |
| Independent decoder cross-check (Ghidra against capstone) | 4842 of 4842 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 789956 |
| code | 20844 |
| data | 76921 |
| padding | 8791 |
| unused | 4608 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Disk-Validator | 1 | 386 | 1 |
| Jail Break (1990)(Odyssey).adf:JailBreak | 4 | 4446 | 154 |
| Jail Break (1990)(Odyssey).adf:bootblock | 1 | 10 | 1 |

## Copy protection found

- none found

## Functions


### Disk-Validator

- `validate_disk_image`: The routine writes a character into a buffer indexed by a1, updates several fields in a2, clears and initializes memory ranges via a5 callbacks, and then runs a loop that sets up per-iteration values in a1 and a2 before invoking a5 repeatedly.

### Jail Break (1990)(Odyssey).adf:JailBreak

- `initialize_game_state_and_loop`: The routine calls initialization helpers, runs a level script, conditionally starts game state setup and the main loop, then processes game objects and closes libraries if needed.
- `wait_vblank_and_refresh`: The routine waits for the vertical blank interrupt request bit in the INTREQ register to be set, then clears the custom register at offset 1F and calls several subroutines to process prisoner cells, reset level tiles, update enemy AI, handle level two events, update player position, and handle input state.
- `wait_for_vblank_and_update_game`: The routine waits for the vertical blank interrupt request bit in the INTREQ register to be set, then clears the corresponding bit in the custom chip register, calls process_prisoner_cells_2, process_prisoner_cells, reset_level_tiles and poll_cia_timer, and repeats the sequence while the unused word in d7 remains zero.
- `run_game_loop_3`: Repeatedly waits for vblank and refreshes, decrements a timer and triggers an event, updates prisoner flags, and checks a state byte.
- `run_game_loop`: The routine repeatedly waits for vertical blank and refreshes the display, checks player input, handles player input, and updates prisoner flags, looping until the global variable _u1__d1_0120 is no longer -1.
- `wait_vblank_and_refresh_loop`: The routine repeatedly waits for vertical blank and refreshes, then handles player input, resets player state, updates the player animation timer, decrements a timer and triggers an event, updates the prisoner timer, and resets player timers, looping while a memory flag is non-zero.
- `initialize_game_state`: The routine clears entity slots and the input buffer, initializes game state, decodes a map tile row, waits for the game over flag, accumulates tile checksums, resets and re-initializes state, waits again, accumulates checksums, sets sprite DMA twice, waits for input, clears player flags, waits for player input, checks game over twice and waits one second twice.
- `initialize_level_state`: The routine clears input and tile buffers, initializes and resets game state, waits for flags and frames, accumulates tile checksums, runs a game loop, sets many byte and word globals to fixed constants, triggers game over, copies tile rows, decodes map data and map tile rows, plays a jingle, resets player state, gets a digit tile, sets interrupt mask and sprite DMA, and returns.
- `init_jail_custom_level`: Initializes jail custom level state and waits for input.
- `run_jail_over_sequence`: The routine resets game state and input buffers, waits for a game over flag and input, accumulates tile checksums, sets sprite DMA, and waits for vertical blanking while polling CIAA_PRA for the fire button to allow early exit.
- `process_prisoner_cells_2`: The routine calls draw_boss_health_bar and then iterates a fixed number of times, checking a short value at an offset derived from the loop counter and calling set_blitter_copy_params when the value equals two.
- `process_prisoner_cells`: The routine iterates a fixed count of times, checks a memory location for the value one, and if so calls write_score_digit and sync_player_state, then continues the loop.
- `decrement_prisoner_counters`: The routine loops over three entries in a short array, decrementing each non-zero counter by one and setting a bit in a global word when a counter reaches zero.
- `reset_player_state`: The routine checks a counter stored in _u1__d1_00EA. If the counter is between zero and zero inclusive, it clears _u1__d1_00C4 to zero, sets _u1__d1_00C2 to the maximum unsigned value, and resets the counter to -1.
- `decrement_timer_and_trigger_event`: The routine checks a byte timer at _u1__d1_00EE. If the timer is non-zero it decrements it and tests whether the result is zero or the original value was less than one.
- `decrement_timer_and_trigger_event_2`: The routine checks a flag in D6 and decrements a counter at a global location.
- `update_player_position`: The routine reads the mouse movement from the hardware register JOY0DAT, computes signed delta values and updates a set of position variables.
- `update_prisoner_positions`: The routine checks two flags and, when set, calls set_sprite_dma and updates a group of position variables by adding offsets to base values, with a conditional adjustment for a Y coordinate threshold, and writes a constant to a timer variable.
- `check_level_two_flags_and_update`: The routine checks if a level flag equals two and, when true, calls process_player_movement for each of three flag bytes if they are non-zero, then calls handle_player_escape and update_prisoner_state.
- `reset_player_timers`: The routine checks a flag and a condition on an input register, then clears three timer value pairs when their counters are negative or exceed a threshold, and calls initialize_game_state_2 when all three cleared flags are zero.
- `initialize_game_state_2`: Routine resets sprite and tile buffers, waits for video refresh, decodes map data and configures sprite DMA and collision flags.
- `reset_and_run_game_loop`: The routine clears sprite buffers, initializes game state, decodes a map tile row and waits for a game over flag, then initializes prisoner positions, waits for input, sets sprite DMA and loops waiting for vertical blank and refresh until a non-zero return.
- `decrement_counter_and_check`: The routine decrements a short at an offset from an uninitialized pointer, waits three frames, then clears a byte if a set of comparisons on memory locations and the pointer value are true.
- `update_prisoner_flags`: The routine iterates over a table of prisoner records when CIAA_PRA bit 6 is clear, clearing or setting flag bits in each record and calling init_enemy_sprite_data, wait_for_player_input_3, check_game_over and check_game_over_2, and updates a counter; when CIAA_PRA bit 6 is set and the counter is positive it clears a flag in one record and calls init_enemy_sprite_data, set_sprite_dma and choose_game_mode.
- `init_enemy_sprite_data`: The routine copies values from registers and from the structure pointed to by a1 into a set of RAM locations, conditionally selecting which fields of the structure to copy based on a flag byte, then writes constant values to several RAM words and stores a small constant, and finally calls draw_bob_sprite_2.
- `choose_game_mode`: The routine checks the value of _u1__d1_00D0.
- `handle_input_command`: The routine polls the CIAA_PRA input register and, when the fire button bit is set, dispatches on the command selector _u1__d1_0126.
- `handle_custody_state`: The routine checks a bit in a global flag using CIAA_PRA and, when set, dispatches on a state byte.
- `draw_boss_health_bar`: The routine checks a mode flag and, when set, programs Amiga blitter registers to copy a small block of memory four times with a fixed offset, then repeats the same blitter setup with a different source offset if a condition on D6 is true.
- `decrement_animation_timer`: The routine decrements a short timer and reloads it when it reaches zero or underflows.
- `process_player_movement`: The routine updates player movement state using the values in the global variables and the input pointers.
- `handle_player_escape`: The routine checks a flag and a counter, updates the counter, and if the counter is within a range and a position difference is within a signed range, it sets the counter, waits for vertical blank and refreshes, clears the flag, increments a short at an offset from an unused register, and processes player input.
- `update_prisoner_state`: The routine checks a flag and decrements a counter, calling process_tile_interaction and wait_vblank_and_refresh when conditions are met.
- `clear_game_state`: The routine clears a block of game state data by zeroing words in a fixed memory area.
- `update_enemy_ai`: The routine processes a list of enemy entities and updates their vertical motion.
- `update_prisoner_timer`: The routine increments a counter at offset 6 of a structure in A5 when the value at offset 10 is less than or equal to the value at offset 8, then calls wait_three_frames and adds 1000 to the value at offset 10.
- `wait_for_vblank_twice`: The routine calls init_player_state_2 eight times in total, arranged as two groups of four calls followed by two additional calls.
- `write_score_digit`: The routine checks a flag in a0 and, if set, computes an address from fields in a0, writes fixed bytes to that location, adds a value from a0 to two bytes, combines a bit from a0[1] with a byte, shifts a0[1] and adds it to another byte, then copies four bytes from the computed address to an offset location.
- `sync_player_state`: The routine compares two short values in the structure pointed to by a0.
- `set_blitter_copy_params`: The routine configures blitter registers for a copy operation and programs a loop that repeatedly waits for the blitter to become idle before updating a blitter source pointer.
- `draw_player_sprite`: The routine programs Amiga blitter registers for a copy operation and then waits for the blitter to finish, repeating the setup four times with updated source and destination pointers.
- `draw_background_tile`: The routine programs Amiga blitter registers to copy a 16x16 tile from chip memory to the screen using a 4x4 sub-tile loop, then returns the combined d0 and d1 values.
- `reset_level_tiles`: The routine checks a flag and a counter and, when the condition is met, resets the counter and iterates over a 25 by 13 byte tile buffer.
- `handle_player_input`: The routine reads the CIAA_PRA register and, if a bit is clear, branches on a mode byte to either update map tiles or wait for player input, calling update_prisoner_positions under certain flag conditions; otherwise it sets a flag to -1.
- `check_player_input`: The routine checks the POTGOR register and two global bytes for a range condition and a non-zero flag.
- `wait_for_player_input`: The routine computes a value from two memory locations, calls clear_pill and trigger_game_over, then loops calling wait_vblank_and_refresh until bit 6 of CIAA_PRA is set, indicating a joystick or mouse fire button press or disk status change.
- `wait_for_input_and_update_state`: The routine computes a value from several memory locations, calls update_game_state_2, then repeatedly calls wait_vblank_and_refresh until the bit in memory at an address is set.
- `update_map_tile`: The routine computes a map byte address from several globals and updates the byte according to the value in a global flag.
- `wait_for_vblank`: The routine checks a byte at a fixed address and, if it equals a specific value, executes a short delay loop that decrements a counter from a start value down to zero.
- `update_map_tile_2`: The routine computes a byte pointer from several global values, loads a byte from a global, subtracts four, and compares it with the byte at the computed address.
- `clear_tile_and_update`: The routine computes a byte pointer from four globals, tests the byte at that address, and if it is non-zero writes a zero byte there and then calls trigger_level_events.
- `clear_map_cell`: The routine computes an address from several global values, checks the byte at that address, and if it is non-zero writes zero to it and then calls init_player_state_4 and set_sprite_dma.
- `update_game_state`: The routine calls draw_bob_sprite and then set_sprite_dma and returns.
- `trigger_level_events`: The routine calls draw_bob_sprite and set_sprite_dma and then returns.
- `fill_level_tilemap`: The routine writes a fixed set of values to a block of RAM locations and then repeatedly calls animate_prisoner_sprite, updating two short loop counters with the call's returned values.
- `draw_background_tiles`: The routine loops over a grid of tile positions, writes fixed tile parameters to RAM locations and a base address to a register, then calls animate_prisoner_sprite to update the next tile coordinates and continues the nested loops.
- `initialize_player_object`: The routine copies a constant to a RAM variable and computes two other RAM values from data constants, then sets a series of RAM fields to fixed constants.
- `trigger_game_over_and_reset`: The routine calls trigger_game_over_2, then wait_one_second, then init_player_state.
- `init_player_state`: The routine copies values from data symbols into RAM locations, sets several RAM words to constant values, and then calls draw_bob_sprite_2.
- `play_prisoner_escape_jingle`: The routine checks the value stored in the unaffiliated A5 pointer.
- `check_game_over_2`: The routine checks a global flag _u1__d1_00D0.
- `check_game_over`: The routine checks a global flag and, if it is zero, calls init_player_state_2 twice.
- `wait_for_player_input_3`: The routine checks a global flag and, if it is clear, calls init_player_state_2 eight times in a row, then returns the concatenation of the two input registers.
- `init_player_state_2`: The routine copies a value from a global to another global, writes several constants to a block of RAM, stores the incoming register values into a block of RAM, then calls draw_bob_sprite_2.
- `clear_pill`: The routine calls trigger_game_over_2, waits one second, then initializes the player object and processes an input string.
- `trigger_game_over`: The routine calls set_sprite_dma and then returns.
- `reset_player_state_2`: The routine calls init_player_state_2 and then returns.
- `reset_game_state`: The routine writes a fixed set of constants to a block of RAM locations, copies the values in registers A0 and A1 into two RAM locations, and then calls draw_bob_sprite_2.
- `reset_player_state_2_2`: The routine clears several RAM locations to zero and writes constant values to others, stores the incoming A0 and A1 arguments into RAM, then calls animate_prisoner_sprite.
- `call_init_player_state_2`: The routine calls init_player_state_2 and returns.
- `trigger_level_event`: The routine calls init_player_state_2 and then returns.
- `init_game_state`: The routine stores a constant value into a global word, stores a pointer to update_score_display into a global word, calls an indirect function via a pointer derived from a global, and if the result is non-zero it calls two more indirect functions derived from the same global.
- `handle_jail_customer`: The routine stores a constant in a global and sets another global to point to update_score_display, then calls a function pointer from a base address offset, and if the result is non-zero it calls two other function pointers from the same base, repeats the call and conditionally repeats the two calls.
- `set_interrupt_mask`: The routine writes to the INTENA hardware register to enable and then disable interrupts, stores addresses for update_score_display and handle_vblank_interrupt in memory, and calls an absolute routine.
- `clear_game_buffers`: The routine clears two memory regions byte by byte by writing zero to each byte.
- `increment_timer_and_update`: The routine increments a short counter at _u1__d1_0116 and, while the value is below 100, calls wait_one_second, decode_map_data, wait_frames_2 and decode_level_map in sequence.
- `decrement_timer_and_update`: The routine decrements a byte variable and, if it becomes negative, sets it to zero and calls clear_sprite_buffers.
- `increment_level_counter`: The routine checks a byte at _u1__d1_00DA plus one against nine.
- `decrement_timer_and_trigger_event_3`: The routine decrements a byte timer and clamps it to zero.
- `wait_one_second`: The routine sets a short counter to 1 and then repeatedly calls draw_bob_sprite while decrementing the counter until it reaches -1.
- `clear_player_flags`: The routine clears a bit in a byte field of a structure pointed to by a global variable and then calls another routine.
- `clear_player_flags_2`: The routine clears a bit in a byte at an offset from a global variable and then calls another routine.
- `process_player_input`: The routine branches on a state byte and updates a set of global variables, calls sprite DMA setup, tile collision flag setup, game state initializers, prisoner state update, and wait helpers.
- `clear_tile_buffer`: The routine clears a tile buffer pointed to by a pointer stored at an offset from a5.
- `store_player_state`: The routine checks a flag and a mask on the input value and, when the condition holds, copies a word from a pointer, computes a masked word from the next pointer element, and stores two small values derived from the input.
- `process_tile_interaction`: Checks a tile at the position in A1 against a map table and updates player state, inventory and sprite DMA based on the tile type.
- `reflect_player_bounce`: The routine takes two short pointers in A0 and A1 and a pair of undefined values in D0/D1.
- `check_object_collision`: Iterates over a table of object entries and tests axis-aligned overlap with the player’s bounding box.
- `update_player_inventory_count`: The routine calls update_prisoner_state_2 and then uses the value in D7 to modify two short fields at offsets 4 and 6 from the address in A0.
- `set_guard_patrol_direction`: The routine checks a global flag and a bit in D6 and, if set, writes constants to globals, calls set_sprite_dma and clears a flag.
- `set_tile_collision_flags`: The routine selects a set of tile-collision flag values based on the current tile type and a secondary tile value, with an additional branch when a bit in an input word is set.
- `update_screen_scroll`: The routine increments a byte at 00E0 by a fixed amount and then repeatedly decrements a byte at 0368, increments a byte at 0378, decrements a byte at 0398 and increments a byte at 03A8, calls update_player_position, and calls wait_vblank_and_refresh, looping until wait_vblank_and_refresh returns zero.
- `animate_prisoner_cells`: The routine increments and decrements four one-byte globals in a loop, calls update_player_position, then calls wait_vblank_and_refresh and repeats while the call returns non-zero.
- `init_game_state_3`: Clears sprite buffers and input state, waits for vblank and input, then resets game memory, loads map data and player state, configures interrupts and sprite DMA, and sets up initial tile and sprite values before returning.
- `wait_frames`: The routine calls draw_bob_sprite repeatedly in nested loops, decrementing counters until they underflow, then returns the concatenation of the incoming d0 and d1 registers.
- `wait_for_input`: The routine repeatedly calls draw_bob_sprite a fixed number of times in a countdown loop, then returns the concatenation of the two input registers.
- `wait_frames_2`: The routine loops twelve times, and for each iteration it calls draw_bob_sprite twelve times.
- `copy_tile_row`: The routine copies a sequence of 346 bytes from a source address computed from a field in memory and a global offset to a destination address taken from a field in memory.
- `clear_tile_buffer_2`: The routine clears a tile buffer area to zero and scans a second buffer of bytes.
- `decode_level_map`: The routine iterates over a fixed grid of cells using two nested loops, reads a byte from a data buffer, checks its low and high nibbles, and conditionally calls init_player_state_4 to obtain a 64-bit value that is split into parts to update loop counters and the buffer pointer.
- `set_sprite_dma_and_wait_vblank`: The routine calls set_sprite_dma and then wait_vblank_and_refresh, and returns the value originally in D0.
- `wait_three_frames`: The routine sets a counter to 3 and loops, calling draw_bob_sprite each iteration until the counter reaches -1, which results in four calls to draw_bob_sprite.
- `parse_script_command`: The routine reads bytes from a pointer in A0, passing each byte to write_byte_to_buffer and updating A0 with the returned pointer.
- `write_byte_to_buffer`: The routine stores the byte from d0 into the memory location pointed to by a1.
- `init_player_state_4`: The routine stores the incoming parameters into RAM locations and computes a pointer value using several global variables and arithmetic shifts, then initializes a set of fixed state fields.
- `draw_bob_sprite`: The routine computes a source address from the inputs and writes a set of blitter control words to memory, then loops four times waiting for the blitter to become idle before updating the source pointer.
- `wait_for_vblank_by_copying_tilemap`: The routine repeatedly calls copy_tilemap_row until a short counter reaches -1, then returns the concatenation of the two input registers.
- `process_input_string`: The routine receives a pointer in A0 and two values in D0 and D1.
- `copy_tilemap_row`: The routine copies or clears eight bytes per column for four columns into a tilemap buffer.
- `animate_prisoner_sprite`: The routine initializes global animation registers and updates the prisoner sprite position fields in the structure pointed to by A0 using offsets from the structure, then loops to compute a blitter destination address from sprite data and updates the sprite position fields each iteration.
- `configure_and_run_bob_blitter`: The routine programs Amiga blitter registers for a bitplane copy and then repeatedly updates source and destination pointers and blitter control values in a nested loop, using the object data in a0 to compute offsets.
- `copy_sprite_frames`: The routine copies sixteen sixteen-bit words from the source area pointed to by a1 to two destination blocks referenced by a2, writing one block starting at offset four and the other starting at offset six, each entry spaced four bytes apart.
- `wait_for_game_over_flag`: The routine repeatedly reads a word from memory starting at a0 and advances the pointer by two bytes each iteration until the word equals the constant.
- `decode_map_tile_row`: The routine scans a byte buffer for the ASCII marker 'CMAP' and then decodes thirty-two three-byte entries into sixteen-bit tile values stored in a static word array.
- `wait_for_input_frames`: The routine repeatedly calls init_player_state_5 in a nested loop, decrementing counters to perform a fixed number of iterations, and returns the last value from init_player_state_5 plus a constant offset.
- `init_player_state_5`: The routine writes a set of fixed values and the incoming D0, D1 and A1 arguments into a block of RAM starting at the address computed from D2 and A2, clears several words, stores the incoming D0 and D1 as words, and then calls animate_prisoner_sprite before returning the concatenation of D0 and D1.
- `accumulate_tile_checksums`: The routine iterates over a fixed number of source words and for each source word it ORs together four words spaced by a constant stride, then stores the resulting 32-bit value into a destination array.
- `transpose_tilemap_rows`: The routine reads 16-bit words from a source area using a column-major stride and writes the accumulated OR of four words per column into a destination buffer row by row, repeating the operation for multiple blocks.
- `clear_input_buffer`: The routine sets a memory byte to a constant and then loops, calling wait_vblank_and_refresh and incrementing a byte at an address in a0 by backspace until the call returns zero.
- `wait_for_input_2`: The routine writes a constant byte to a RAM location and then loops, calling wait_vblank_and_refresh and decrementing a byte at an address offset from the return address until wait_vblank_and_refresh returns zero.
- `decrement_timer_counters`: The routine repeatedly waits for vertical blank and refresh, then iterates over a block of 32-bit timer words and decrements each nibble that is non-zero, writing the updated word back.
- `decode_map_data`: The routine iterates over a fixed 12 by 19 grid, reading bytes from a data pointer and for each non-zero byte calling draw_bob_sprite to obtain a 64-bit value.
- `wait_frames_3`: The routine sets a short counter to twelve and repeatedly calls draw_bob_sprite while decrementing the counter until it reaches minus one.
- `clear_entity_slots`: The routine clears a block of entity slots by writing zero to each slot in a fixed-size loop and then waits for vertical blank and refreshes the display.
- `get_digit_tile`: The routine computes an index from the input value in d0 and a value from a5, extracts a digit, scales it to a tile offset, reads a word from a data table and stores it into a global variable.
- `set_player_sprite`: The routine writes a short value from the first argument to a hardware register whose address is computed from the second argument via a table lookup, then returns the concatenation of the two arguments.
- `clear_sprite_buffers`: The routine clears a small array of four 16-bit sprite buffer entries to zero, sets a global flag to a constant value, then calls wait_vblank_and_refresh.
- `set_sprite_dma`: The routine attempts to copy a sprite definition from a per-type table into the Amiga sprite DMA registers.
- `clear_player_collision_flags`: The routine calls init_game_state and then iterates over a block of structures, clearing a specific bit in a field of each structure.
- `update_prisoner_state_2`: The routine updates a global word using a shift and xor operation with a constant and returns the concatenation of the two input registers.
- `scramble_level_data`: The routine iterates over a block of short values starting at index 899 down to zero, applying a bitwise transformation to each element.
- `scramble_level_data_2`: The routine iterates over a 900-element short array starting at index 899 down to zero and replaces each element with a scrambled value derived from a left rotate by five bits combined with an XOR with four.
- `draw_prisoner_sprite`: The routine calls update_prisoner_state_2, computes an address using an uninitialized word and a constant offset, then calls wait_for_vblank_2 twice before returning.
- `compare_strings`: The routine receives two pointers in A0 and A1 and compares the bytes they point to one by one.
- `handle_input_state`: The routine checks a state variable and, when the state is active, polls a timer and processes input from an uninitialized register.
- `poll_cia_timer`: The routine reads the CIA timer value, sets a bit in the CIA control register, and if the timer value changed and the timer flag is set, it calls set_sprite_dma.
- `initialize_jail_state`: The routine opens the DOS library, waits for a timer tick, fills bullet positions and clears enemy targets, stores a pointer and two values into RAM, then runs the jail intro sequence, loads jail tile graphics, copies character graphics, handles the jail title screen, and returns.
- `clear_enemy_targets`: The routine sets a flag in a global word and then iterates over a series of structures.
- `process_sfx_jail_general`: The routine calls an initialization function, then iterates over a fixed set of entries in a data table.
- `iterate_entity_callbacks`: The routine walks a fixed list of entity descriptors and, for each entry whose first field is non-zero and second field is zero, calls a callback routine located at a fixed offset from a global pointer.
- `open_dos_library`: The routine calls the function pointer stored at the address computed from an address minus a constant and returns.
- `close_graphics_library`: The routine obtains a pointer from a global variable, calls a function through an indirect pointer derived from that variable, stores a value from the returned pointer into a global variable, then calls another function through an indirect pointer derived from the first variable.
- `wait_for_timer_tick`: The routine polls the combined CIA timer value until it becomes non-zero, then copies that value into a memory location and writes a constant to the CIA interrupt enable register, which enables interrupts.
- `fill_bullet_positions`: The routine writes a series of 32-bit values from a running integer into a table of 16-bit words, advancing the destination pointer by eight bytes each iteration and incrementing the source integer by a constant amount, repeating for eight entries.
- `initialize_prisoner_positions`: The routine writes a series of values to a block of memory starting at a fixed data location.
- `load_jail_tile_graphics`: The routine calls init_game_state twice with wait_for_game_over_flag and wait_for_input_frames between calls, then calls transpose_tilemap_rows.
- `copy_character_graphics`: The routine calls init_game_state and wait_for_game_over_flag, then copies bytes from a source area referenced by _u1__d1_001C to a destination area referenced by _u1__d1_0064 using nested loops with offsets.
- `show_jail_intro_sequence`: Clears input, initializes prisoner positions and game state, decodes a map tile row, waits for the game over flag, sets a delay value, waits for input, sets sprite DMA twice and plays a jail sound effect.
- `handle_jail_title_screen`: This routine initializes the game state, waits for flags, accumulates tile checksums, decodes a map tile row and triggers a level event, then enters a loop that draws a prisoner sprite, polls CIA timer and input, builds an input string in memory, compares strings and scrambles level data before returning.
- `close_game_libraries`: The routine runs entity callbacks, calls an indirect routine, writes to memory locations, then closes the graphics library.
- `run_level_script`: The routine repeatedly calls a function taken from a table pointed to by a global variable, stores each returned value into successive entries of a global variable array, and stops when the returned value is zero or a negative sentinel is found in the array, returning that sentinel or zero.
- `process_game_objects`: The routine walks a table of entries stored in a global variable, stepping through the table by a fixed stride.
- `update_score_display`: The routine updates the score display by writing to a custom hardware register.
- `handle_vblank_interrupt`: The routine checks a bit in a custom hardware register and, if set, writes a value to a custom hardware register and calls decrement_prisoner_counters.

### Jail Break (1990)(Odyssey).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
