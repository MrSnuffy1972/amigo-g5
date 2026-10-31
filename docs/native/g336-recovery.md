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

## Functions


### Disk-Validator

- `validate_disk_image`: This routine runs the disk validation check for the Jail Break Amiga release.

### Jail Break (1990)(Odyssey).adf:JailBreak

- `initialize_game`: This is the program entry point that sets up the game state and starts the main loop.
- `wait_vblank_and_refresh`: This routine synchronizes to the vertical blank interval and then triggers a full screen refresh cycle for the game.
- `wait_for_vblank_and_update_game`: This routine synchronizes the game loop to the Amiga vertical blank by waiting for the appropriate interrupt flag, then runs the per-frame update steps for input, logic and rendering before looping again.
- `run_game_loop_3`: This routine drives the main game loop for Jail Break.
- `run_game_loop`: This routine drives the main gameplay loop for the jail break levels.
- `update_game_state_loop`: This routine runs the main per-frame update loop for the game, repeatedly processing input, game logic and rendering until the level ends.
- `initialize_game_state`: Resets global state and runs the startup sequence that loads the title screen graphics and prepares the initial game objects and UI for Jail Break.
- `initialize_level_state`: This routine sets up the initial state for a new jail level, loading the tile and gadget graphics and configuring player and enemy parameters.
- `init_jail_custom_level`: Initializes the custom jail level layout and its graphics.
- `run_jail_over_sequence`: Plays the jail over sequence after the player is caught, showing the jail gadgets and over screens and waiting for input.
- `process_prisoner_cells_2`: Iterates over the prisoner cells and updates any cell that is in the active state.
- `process_prisoner_cells`: Iterates over the prisoner's cells and triggers the cell interaction routine for any cell that is active.
- `decrement_prisoner_counters`: Iterates over the four prisoner timers and decrements each active counter.
- `reset_player_state`: This routine resets the player’s state when a counter expires.
- `decrement_timer_and_trigger_event`: The routine decrements a countdown timer and, when it reaches zero, clears two game flags and triggers an event.
- `decrement_timer_and_trigger_event_2`: The routine decrements a countdown timer when a specific flag is set and, once the timer reaches zero, resets it and triggers a game event.
- `update_player_position`: This routine updates the player’s position based on joystick input and handles screen wrapping and collision limits.
- `update_prisoner_positions`: This routine updates the on-screen positions and state timers for two prisoners during the jail break sequence.
- `handle_level_two_events`: This routine runs the per-frame logic for level two.
- `reset_player_timers`: This routine checks the player state and clears the three player timers when they are negative or have passed their limit.
- `initialize_game_state_2`: This routine sets up the initial game state and variables for a new play session.
- `run_game_loop_2`: This routine initializes the game systems and then runs the main gameplay loop until the player quits, after which it performs cleanup and exits to the title screen.
- `decrement_counter_and_check`: The routine decrements a counter and then checks a set of game state flags.
- `update_prisoner_flags`: The routine updates the status flags for the prisoners in the current room.
- `init_enemy_sprite_data`: Initializes the data for an enemy sprite before it is drawn.
- `choose_game_mode`: The routine selects which gameplay mode to run based on a global flag.
- `handle_input_command`: Processes a pending player input command flagged by the CIA port and dispatches the corresponding game action, such as changing maps, toggling views, or triggering events.
- `handle_custody_state`: This routine processes the current custody state when the game is active.
- `draw_boss_health_bar`: When the boss encounter is active, the routine draws the boss health bar in the HUD using a series of blitter copies.
- `update_player_animation_timer`: This routine decrements the player animation timer and resets it when it expires.
- `process_player_movement`: This routine updates the player character's movement and collision handling each frame.
- `handle_player_escape`: This routine checks the player’s escape progress and triggers the jailbreak sequence when the required conditions are met.
- `update_prisoner_state`: This routine updates the state of a prisoner being moved or processed.
- `clear_game_state`: Resets a block of game state variables to zero at the start of a new level or when restarting play.
- `update_enemy_ai`: This routine updates the behavior of enemies when the game is in a specific mode.
- `update_prisoner_timer`: The routine advances the prisoner’s countdown timer and triggers a periodic update.
- `wait_for_vblank_twice`: The routine pauses the game by waiting for a number of vertical blank intervals, effectively creating a short delay.
- `write_score_digit`: The routine writes a four-character score digit into the game's score buffer when a score entry is present.
- `sync_player_state`: This routine keeps the player’s current state in sync with its target state.
- `set_blitter_copy_params`: The routine programs the Amiga blitter to copy a small block of graphics from the game’s source buffer into the screen buffer, using the parameters passed in A0 to choose the source and destination.
- `draw_player_sprite`: This routine sets up the Amiga blitter to copy the player sprite from chip memory to the screen.
- `draw_background_tile`: The routine sets up a series of blitter operations to copy background tiles into the screen buffer.
- `reset_level_tiles`: This routine resets the tile map when a level is restarted.
- `handle_player_input`: Processes the player controller state each frame, updating movement and actions based on joystick input and game flags.
- `check_player_input`: This routine checks the player’s joystick and button state and decides whether to start the player movement or the player idle handling.
- `wait_for_player_input`: This routine computes a timing value from game state, updates the display and sound, then blocks until the player presses a button on the joystick.
- `wait_for_player_input_2`: This routine prepares a display-related value and then pauses execution until the player presses a specific button on the joystick.
- `update_map_tile`: Updates the map tile at the player’s current position depending on the current interaction mode.
- `wait_for_vblank`: This routine is called during the game loop to pause execution until the display refresh is complete.
- `update_map_tile_2`: The routine updates a single map tile in the level layout to reflect the current player state.
- `clear_tile_and_update`: The routine clears a specific map tile by writing a zero into the level data and then triggers a screen refresh.
- `clear_map_cell`: The routine clears a single map cell that is currently occupied, marking it empty and then triggering the game’s map update and redraw steps.
- `update_game_state`: This routine advances the game logic by invoking the core update steps for the current level.
- `trigger_level_events`: This routine is called during the level update sequence and fires the per-frame game logic for the current screen.
- `fill_level_tilemap`: The routine fills the level tilemap RAM with a repeating pattern for each screen row and column, preparing the background layout before the level is drawn.
- `draw_background_tiles`: The routine fills the background tilemap by iterating over rows and columns and invoking the tile drawing helper for each cell.
- `initialize_player_object`: This routine sets up the player object for a new game session.
- `update_game_state_2`: This routine is called during the main game loop to advance the current level state.
- `init_player_state`: Initializes the player character's runtime state for a new level or restart.
- `play_prisoner_escape_jingle`: Plays the short musical cue that signals a successful prison break.
- `check_game_over_2`: Checks whether the game is over and, if so, triggers the game-over sequence twice in succession.
- `check_game_over`: The routine checks a global game-over flag and, when the game is not over, triggers the game-over handling twice.
- `wait_for_player_input_3`: Pauses the game flow until the player provides input, repeatedly polling the input state.
- `init_player_state_2`: This routine sets up the player’s initial state and game parameters at the start of a level or after a reset.
- `clear_pill`: Clears the pill object from the playfield.
- `trigger_game_over`: This routine signals the end of the current play session, causing the game to stop the action and present the game-over screen.
- `trigger_game_over_2`: This routine is invoked during gameplay to end the current session and start the game-over sequence.
- `reset_game_state`: This routine clears and initializes the game state variables and sets up the initial player parameters before starting a new game session.
- `reset_player_state_2`: This routine clears and initializes the player state variables used during a jail break attempt.
- `trigger_game_over_3`: This routine initiates the game over sequence when the player is caught.
- `trigger_level_event`: This routine fires a level event in Jail Break.
- `init_game_state`: Initializes the game state and sets up the first level.
- `handle_jail_customer`: This routine processes the jail customer entity, updating its state and movement using the absolute move helper.
- `set_interrupt_mask`: This routine configures the Amiga interrupt mask and installs absolute move handlers used by the game engine.
- `clear_game_buffers`: This routine clears the game’s temporary graphics and state buffers before a new level or restart.
- `increment_timer_and_update`: This routine advances a short-lived timer used during a timed game sequence and refreshes the related on-screen elements.
- `decrement_timer_and_update`: This routine decrements a countdown timer used by a timed game event.
- `increment_level_counter`: This routine advances the level progression counter for the game.
- `decrement_timer_and_trigger_event_3`: This routine decrements a countdown timer used for a timed game event.
- `wait_one_second`: Pauses the game for a short period by repeatedly invoking the engine's frame delay routine.
- `clear_player_flags`: The routine clears a per-player flag for each active player and then updates the game state.
- `clear_player_flags_2`: The routine clears a status flag for each player entry in the game’s player table and then advances to the next entry until the table is exhausted.
- `process_player_input`: Handles the player’s controller actions each frame, updating movement, actions and timers for the current input mode.
- `clear_tile_buffer`: The routine clears a block of tile data in memory and then waits until the game’s input or update routine reports that processing is finished.
- `init_player_state_3`: Initializes the player state when a new game starts.
- `process_tile_interaction`: Handles player interaction with a map tile when stepping on it, applying tile effects such as collecting items, triggering events, or changing the player state.
- `reflect_player_bounce`: The routine reflects the player’s velocity when hitting a wall or floor in Jail Break.
- `check_collision_with_objects`: The routine checks the player’s bounding box against a list of world objects and triggers a collision response when an overlap is found.
- `update_player_inventory_count`: Adjusts the player’s inventory counters for items and keys in response to a pickup or use event.
- `set_guard_patrol_direction`: This routine updates the guard patrol parameters for the current guard.
- `set_tile_collision_flags`: This routine configures collision and interaction flags for a map tile based on the current tile type and the requested interaction mode.
- `update_screen_scroll`: This routine advances the horizontal and vertical screen scroll counters each frame while the game is running, keeping the playfield moving.
- `animate_prisoner_cells`: This routine drives the scrolling animation of the prison cell background while the game is waiting for player input.
- `initialize_game_state_3`: This routine sets up the initial game state and variables for a new play session.
- `wait_frames`: The routine produces a fixed pause by repeatedly invoking a frame delay routine, effectively waiting a number of game frames before returning.
- `wait_for_input`: This routine pauses the game by repeatedly calling a delay routine until a short counter expires, effectively creating a timed wait.
- `wait_frames_2`: This routine is used as a short pause between game actions.
- `copy_tile_row`: Copies a row of tile data from a source buffer into the destination tile buffer used for the playfield.
- `clear_tile_buffer_2`: This routine clears the tile buffer used for the game screen and scans the level map data to count active tiles.
- `decode_level_map`: Reads the packed level map data and expands it into a decoded tile grid used for building the playfield.
- `update_game_state_3`: This routine refreshes the game state by updating internal timers and counters before returning control to the caller.
- `wait_three_frames`: Pauses execution for three frames by repeatedly calling the frame delay routine.
- `parse_script_command`: This routine reads a scripted command stream for the game and processes each command by dispatching to the command handler.
- `write_byte_to_buffer`: The routine writes a single byte from a register into a byte pointed to by a pointer.
- `init_player_state_4`: This routine initializes the player character's state and position for a new level or after a reset.
- `draw_bob_sprite`: This routine sets up a blitter operation to draw a sprite with a bounding box offset and waits for the blitter to finish, repeating for multiple lines.
- `wait_for_vblank_2`: The routine pauses the game by repeatedly waiting for the vertical blank interrupt until a fixed count expires.
- `process_input_string`: Iterates over a null-terminated input string, invoking the per-character handler for each character.
- `copy_tilemap_row`: Copies or clears a set of 8x4 tile rows from a source pattern table into the screen tilemap buffer based on a mask.
- `animate_prisoner_sprite`: This routine steps a prisoner sprite through a short animation sequence, updating its screen position and blitter parameters each frame.
- `draw_bob_sprite_2`: This routine sets up a blitter operation to draw a bob sprite with scaling and offset handling.
- `copy_sprite_frames`: Copies a set of sprite frame words from a source buffer into two destination sprite tables used by the game.
- `wait_for_game_over_flag`: The routine scans a memory table for a game-over sentinel and then triggers the end-of-level/game handling.
- `decode_map_tile_row`: The routine decodes a compressed map row from the game data and expands it into a series of tile codes used for building the level layout.
- `wait_for_input_frames`: The routine blocks until the player provides input, polling the controller for a fixed number of frames.
- `init_player_state_5`: The routine sets up the initial player state for a new game session.
- `accumulate_tile_checksums`: This routine builds a table of accumulated checksums for the level tiles.
- `transpose_tilemap_rows`: The routine reorganises the in-memory tile map by transposing rows and columns for the level layout.
- `clear_input_buffer`: This routine empties any pending keyboard input by repeatedly reading characters until no more are available.
- `wait_for_input_2`: This routine blocks until the player provides input, repeatedly polling the input handler and decrementing a timer each loop.
- `decrement_timer_counters`: This routine repeatedly decrements a set of in-game timer counters used for jail break mechanics.
- `decode_map_data`: This routine decodes the compressed level map data used by Jail Break.
- `wait_frames_3`: Pauses the game by burning a fixed number of frames.
- `clear_entity_slots`: Resets the active entity slot table used by the game engine.
- `get_digit_tile`: The routine extracts a single decimal digit from a numeric value and looks up the corresponding tile index for rendering.
- `set_player_sprite`: Sets the sprite data for a player character in the game.
- `clear_sprite_buffers`: Resets the sprite DMA control and clears the sprite buffer tables used for the game’s hardware sprites.
- `set_sprite_dma`: This routine configures a sprite DMA entry for a game object when the object is active and its sprite slot is free.
- `clear_player_collision_flags`: This routine clears the collision flags for all player objects in the game.
- `update_prisoner_state_2`: Updates the internal prisoner state variable used by the game logic.
- `scramble_level_data`: This routine scrambles the level layout data in memory by applying a bitwise transformation to each byte.
- `scramble_level_data_2`: This routine scrambles the level layout data in memory using a simple bit-mixing operation.
- `draw_prisoner_sprite`: This routine prepares the prisoner sprite data and draws the prisoner on screen.
- `compare_strings`: This routine checks whether two null-terminated strings are identical by comparing their characters one by one.
- `handle_input_state`: Processes player input while the game is in a specific mode, updating internal state variables and triggering UI or gameplay actions such as opening menus or resetting the game.
- `poll_cia_timer`: The routine checks the CIA timer latch and updates a flag to indicate a timer tick.
- `initialize_game_state_4`: This routine runs at game start and sets up the initial state for Jail Break.
- `clear_enemy_targets`: The routine clears the target pointers for a set of enemy slots.
- `process_sfx_jail_general`: This routine initializes and runs the jail general sound effects system.
- `process_entity_callbacks`: Iterates over a set of active game entities and invokes each entity’s update callback when the entity is present and enabled.
- `open_dos_library`: This routine opens the DOS library for the game.
- `close_graphics_library`: This routine shuts down the graphics subsystem used by the game.
- `wait_for_timer_tick`: The routine blocks until the CIA timer reaches a non-zero value, then records the timer count and enables the CIA interrupt.
- `fill_bullet_positions`: This routine initializes the bullet positions for the player’s weapon.
- `initialize_prisoner_positions`: Sets up the initial positions for a group of prisoners in the game.
- `load_jail_tile_graphics`: This routine loads the jail tile graphics resources used for the prison level.
- `copy_character_graphics`: The routine copies the character graphics from the loaded IFF data into the game's tile buffer.
- `show_jail_intro_sequence`: Displays the jail introduction cutscene using two intro graphics and waits for player input before returning to gameplay.
- `handle_jail_title_screen`: This routine displays the jail title screen and waits for player input to start the game.
- `close_game_libraries`: This routine shuts down the game by closing the Amiga OS libraries it opened and disabling the custom hardware interrupts and DMA. It is invoked from the main loop when the player quits or the game ends.
- `run_level_script`: Executes the scripted actions for the current level by iterating through a table of function pointers and invoking each until a zero result is returned.
- `process_game_objects`: Iterates over the active game objects and dispatches each one to its update routine.
- `update_score_display`: The routine refreshes the on-screen score read-out and updates the game state counters after a scoring event.
- `handle_vblank_interrupt`: The routine checks for a vertical blank condition and, when detected, sets a flag and invokes the game’s VBlank handler to perform per-frame updates such as screen refresh and game logic.

### Jail Break (1990)(Odyssey).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
