# Game 2179 (GreatGianaSisters v1.3 2500): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02179). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2066516 top-level bytes in 18 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 48 of 48 units identical (bytes and relocations), 13 raw-word lines |
| Functions named and described | pass, 302 of 302 named (0 author symbols), 302 of 302 described (0 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4225 of 4225 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6709 of 6710 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1520907 |
| code | 50642 |
| data | 341942 |
| padding | 3009 |
| unused | 150016 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 136 | 12 |
| logged disk load from Disk.1 | 1 | 713 | 38 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 5087 | 195 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 149 | 15 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 582 | 31 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 5 |

## Cases handled

- GreatGianaSisters_v1.3_2500/GreatGianaSisters_v1.3_2500_hdf/GreatGianaSisters/Disk.1: raw disk image (894976 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Disk.1 +an address loaded at an address (5888 bytes) (static): load proved statically: track 83 is the high-score track: the routine at +an address (address an address) calls the track loader with track an address, one track, into the name table at an address (under WHDLoad the slave serves its first 96 bytes from the gianahigh file or this offset)
- 41 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address (play-at2): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-at2): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-at2): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-at2): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Disk.1 +an address loaded at an address (11776 bytes) +4BA..78C: data by evidence (the boot program's variables and tables (allocation table, load table, picture and joystick state, palette buffer, logo colour table), read and written as data by the boot code; the same bytes in the run-time copy at an address are typed as data); 181 instructions decoded there were removed
- Disk.1 +an address loaded at an address (11776 bytes) +0..20: data by evidence (the hunk file header of the boot program (magic, hunk table, code hunk size); the slave and the boot code jump past it to the code); 0 instructions decoded there were removed
- Disk.1 +an address loaded at an address (52992 bytes) +E4..18E: data by evidence (sound-effect player state and tables: the active flag word at +an address and sample-base longword at +an address written by the effect trigger at +an address, then four ten-entry longword tables (sample offset, length, period, volume) it indexes with the effect number, and the sample bank pointer at +an address set at start-up); 43 instructions decoded there were removed
- Disk.1 +an address loaded at an address (52992 bytes) +27E..320: data by evidence (music player state: current tune word at +an address, four channel sequence pointers, then five tunes' channel start tables (four longwords each, two copies) loaded with movem by the tune starters at +an address..+an address); 41 instructions decoded there were removed
- Disk.1 +an address loaded at an address (52992 bytes) +414..41C: data by evidence (the four volume words the unreferenced routine at +an address reads with (a2)+); 0 instructions decoded there were removed
- Disk.1 +an address loaded at an address (52992 bytes) +566..56A: data by evidence (music tempo word written by the tune starter at +an address and multiplied into every note length (mulu)); 1 instructions decoded there were removed
- Disk.1 +an address loaded at an address (52992 bytes) +0..20: data by evidence (the hunk file header of the game program (magic, hunk table, code hunk size); the boot program jumps past it to the code); 0 instructions decoded there were removed
- GreatGianaSisters_v1.3_2500/GreatGianaSisters_v1.3_2500_hdf/GreatGianaSisters/Disk.1: 41 logged loads cover 894976 bytes of the image
- GreatGianaSisters_v1.3_2500/GreatGianaSisters_v1.3_2500.hdf:ReadMe: identical to the extracted copy GreatGianaSisters_v1.3_2500/GreatGianaSisters_v1.3_2500_hdf/GreatGianaSisters/ReadMe, classified through it
- GreatGianaSisters_v1.3_2500/GreatGianaSisters_v1.3_2500.hdf:ReadMe.info: identical to the extracted copy GreatGianaSisters_v1.3_2500/GreatGianaSisters_v1.3_2500_hdf/GreatGianaSisters/ReadMe.info, classified through it
- GreatGianaSisters_v1.3_2500/GreatGianaSisters_v1.3_2500.hdf:Disk.1: identical to the extracted copy GreatGianaSisters_v1.3_2500/GreatGianaSisters_v1.3_2500_hdf/GreatGianaSisters/Disk.1, classified through it
- GreatGianaSisters_v1.3_2500/GreatGianaSisters_v1.3_2500.hdf:game.slave: identical to the extracted copy GreatGianaSisters_v1.3_2500/GreatGianaSisters_v1.3_2500_hdf/GreatGianaSisters/game.slave, classified through it
- GreatGianaSisters_v1.3_2500/GreatGianaSisters_v1.3_2500.hdf:GreatGianaSisters.info: identical to the extracted copy GreatGianaSisters_v1.3_2500/GreatGianaSisters_v1.3_2500_hdf/GreatGianaSisters/GreatGianaSisters.info, classified through it

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `slave_cli_guard`: Slave header entry: returns minus one so the slave does nothing if started from the shell.
- `slave_start`: WHDLoad entry: keeps the resload base, moves the stack, loads the first two tracks, checks the boot program checksum (quits on a wrong version), installs the keyboard interrupt, patches the boot program and starts it.
- `slave_colour_register_fix`: Patched into the boot palette routine: replaces the instructions the patch overwrote (counter adjust and colour register base).
- `slave_game_hook`: Called instead of the jump to the game: patches the game program with the second patch list and jumps to the game start.
- `slave_read_joystick_two_buttons`: Replacement joystick read: maps the second fire button to up, so it jumps.
- `slave_wait_raster`: Replacement raster wait that waits on the beam counter without the original busy loop problems.
- `slave_dbf_delay`: Replacement for an empty counting delay in the music player: waits a fixed number of beam lines instead, so it does not depend on processor speed.
- `slave_dma_enable`: Replacement for a DMA enable write in the game loop.
- `slave_track_loader`: Replacement track loader: converts track number and count into a disk image offset and length and calls the WHDLoad disk load; the high-score track comes from the saved file when it exists.
- `slave_save_highscores`: Replacement track writer: saves the high-score table to a file unless the trainer was used.
- `slave_keyboard_interrupt`: Level-2 interrupt: reads a key; the trainer key toggles life loss by flipping one game instruction, the quit key ends WHDLoad.
- `slave_read_key`: Reads and acknowledges a key from the keyboard serial port and checks it against the quit and debug keys.

### logged disk load from Disk.1

- `boot_main`: Body of the boot program after its branch: blanks the colour, allocates, loads, shows logo and title, loads the game and jumps to it through the slave hook.
- `boot_allocate_buffers`: Hands out the boot buffers downward from the top of chip memory and derives the logo, title and music addresses.
- `boot_load_tracks`: Calls the track loader for each entry of the boot load table.
- `show_logo`: Unpacks the Time Warp logo picture, sets its palette and shows it until fire is pressed.
- `show_title_picture`: Unpacks the title picture, sets its palette and shows it with the title music until fire is pressed.
- `logo_loop`: Displays the logo and cycles its colours every frame until fire is pressed, then blanks the display.
- `build_logo_copper_list`: Builds the logo copper list and sets up the display for the picture size and depth.
- `cycle_logo_colours`: Every second frame rotates the logo colour table by one and writes it to the colour registers.
- `boot_display_off`: Switches bitplane and copper DMA off.
- `title_picture_loop`: Shows the title picture with the music playing until fire is pressed, then stops the music and blanks the display.
- `build_title_copper_lists`: Builds two alternating copper lists for the title picture and starts the display.
- `boot_wait_raster_line`: Waits until the video beam reaches the line given in d6.
- `boot_delay_loop`: Unreferenced busy-wait loop that counts d0 down to zero.
- `boot_add_copper_pointer_pair`: Appends two copper moves that load a chip address into a pointer register pair.
- `unpack_iff_picture`: Reads an IFF ILBM picture: header, plane pointers and body.
- `read_ilbm_header`: Reads the bitmap header (size, depth, compression) and finds the colour map and body chunks.
- `set_picture_plane_pointers`: Computes the start of each bitplane of the picture buffer.
- `unpack_ilbm_body`: Unpacks the picture body row by row into the bitplanes.
- `unpack_byterun1_row`: Unpacks one bitplane row, copying literal runs or repeating a word (ByteRun1 style, in words).
- `load_picture_palette`: Converts the picture colour map to Amiga colour words and writes them to the colour registers and the cycling table.
- `boot_read_joystick`: Reads the joystick in port two and the fire button into direction and fire flags.
- `boot_relocate_music`: Adds the music bank address to every pointer in the boot music relocation list.
- `boot_start_music`: Starts the title music and switches audio DMA on.
- `boot_stop_music`: Silences the music and switches audio DMA off.
- `boot_start_sequencer_channels`: Initialises the four sequencer channel records for the title music.
- `boot_reset_sequencer_channel`: Resets one sequencer channel record of the boot copy of the music player.
- `boot_silence_channels`: Silences all four audio channels from the boot volume table.
- `boot_silence_one_channel`: Silences one audio channel and writes its volume from the table.
- `boot_music_tick`: Advances the four channels of the boot copy of the music sequencer once.
- `boot_sequencer_channel_tick`: One tick of a channel in the boot copy of the Beathoven sequencer: note length, pattern commands, portamento and DMA restart.
- `boot_track_loader`: Track loader of the boot program; in RAM the slave has replaced it with a jump to its own loader.
- `boot_drive_seek`: Boot loader drive set-up: selects drive zero, motor on, steps to the requested cylinder and side.
- `boot_wait_drive_ready`: Boot loader: waits until the drive reports ready.
- `boot_read_raw_track`: Boot loader: reads one raw track with disk DMA after the sync word, with a timeout.
- `boot_step_to_next_track`: Boot loader: switches side or steps one cylinder in.
- `boot_drive_off`: Boot loader: stops the motor and deselects the drive.
- `boot_drive_settle_delay`: Boot loader: short delay between drive control changes.
- `boot_drive_step_delay`: Boot loader: delay after a head step.

### logged disk load from Disk.1

- `program_start`: Program entry after the hunk header: points the trap 0 vector at the supervisor start and executes trap 0, so the game continues in supervisor mode.
- `supervisor_start`: Supervisor-mode start: sets the stack below the program, runs the one-time machine set-up and the first copper list, then enters the endless title-and-game loop.
- `sound_effect_dma_step`: Called from the vertical blank: second and third phase of a sound effect on audio channel four, first re-enabling its DMA after the trigger, then pointing the channel at the silent loop so the sample plays once.
- `play_sound_effect`: Starts sound effect number d0 on audio channel four: looks up the sample offset, length, period and volume in the effect tables, writes them to the channel and arms the vertical-blank step that enables DMA.
- `relocate_music_bank`: Adds the address of the loaded music bank to every pointer in the music relocation list and stores the silent-sample address, so the sequencer data can be used where the allocator placed it.
- `start_title_tune`: Starts tune 1 (the title music) on all four audio channels with its four channel sequence tables.
- `start_stage_tune`: Selects tune 2, the normal stage music, then falls into the three-channel start so channel four stays free for sound effects.
- `start_stage_tune_channels`: Second entry of the stage-tune start: marks three-channel mode, loads the tune-2 channel tables and starts the sequencer with tempo 2.
- `start_castle_tune`: Starts tune 3, the music of every fourth stage, in three-channel mode with its own channel tables.
- `start_highscore_tune`: Starts tune 4, the high-score screen music, on all four channels with tempo 3.
- `start_get_ready_tune`: Starts tune 5, the short jingle of the get-ready and message screens, in three-channel mode.
- `stop_music`: Clears the current-tune word so the sequencer stops, and switches audio DMA off for all four channels.
- `start_sequencer_channels`: Stores the tempo and initialises the four channel records with their hardware audio registers, sequence list pointers, loop pointers and DMA bits, then resets each channel.
- `reset_sequencer_channel`: Resets one channel record: clears transpose and flags, sets the note counter to one, mutes the hardware volume and loads the first pattern pointer and its transpose from the sequence list.
- `set_channel_volumes`: Unreferenced routine: walks the four channel records and writes each channel volume from the volume table.
- `set_one_channel_volume`: Unreferenced helper of the volume reset: clears the hardware volume of one channel and writes the next value from the volume table.
- `music_tick`: Called from the vertical blank: if a tune is playing, advances channels one to three, and channel four too unless the tune runs in three-channel mode.
- `sequencer_channel_tick`: One tick of a channel of the Beathoven sequencer: keeps the sample looping, counts down the note length and interprets pattern commands (instrument, portamento, vibrato, rest, arpeggio, transpose, note), then applies portamento and restarts DMA for a new note.
- `title_screen`: Title and menu: builds the title copper list, prints the credits, starts the title tune and scrolls the attract-mode stage until the joystick picks one player (left) or two players (right); then resets scores, lives, stages and player names for a new game.
- `build_title_copper_list`: Builds the title copper list: bitplane modulos and pointers for the text area, eight sprite pointers, a forty-line colour bar gradient and the playfield section that shows the scrolling attract stage.
- `start_title_display`: Points the copper at the title list and sets the display window, data fetch and four-bitplane mode.
- `print_title_credits`: Clears the four planes of the title text area and prints the player-choice line and the credits.
- `attract_scroll_step`: Moves the attract-mode scroll position by its direction and reverses at either end of the stage, so the title stage scrolls back and forth.
- `machine_setup`: One-time set-up: allocates the buffers, loads the game data and music bank from disk, relocates the music data and installs the vertical-blank interrupt.
- `install_vblank_interrupt`: Installs the level-3 interrupt handler and enables the vertical-blank interrupt.
- `vblank_interrupt`: Level-3 interrupt: on a vertical blank runs the sound-effect step and the music tick, then acknowledges the interrupt.
- `allocate_buffers`: Hands out the game buffers downward from the top of chip memory using the size and pointer table, then derives the addresses of the sprite sets, graphics banks, font and object graphics inside the loaded data blocks.
- `load_startup_tracks`: Reads the start-up load table and calls the track loader for each entry: the graphics and stage-data block and the music bank.
- `build_game_copper_list`: Builds the in-game copper list: status-bar bitplanes, sprite pointers for the cloud sprites, a wait for the playfield start, the scrolling playfield bitplane pointers and the scroll register entry.
- `build_message_copper_list`: Builds the copper list of the message screens (get ready, warp, time-out): four bitplane pointers, empty sprite pointers and the end-of-screen interrupt request.
- `add_copper_pointer_pair`: Appends two copper moves that load a 32-bit chip address into a pair of pointer registers, advancing the register number.
- `start_game_display`: Points the copper at the in-game list and sets the display window, data fetch and four-bitplane mode.
- `clear_message_screen`: Clears the four planes of the message screen, switches the display to the message copper list and sets the text colours.
- `show_get_ready`: Shows the get-ready screen before a life (player name and stage number) with its jingle and waits; skipped in bonus stages and after the last stage.
- `show_out_of_time`: Shows the out-of-time message screen when the stage timer has run out.
- `show_game_over`: Unreferenced routine: clears the message screen and prints the game-over text.
- `show_wake_up_message`: At the end of a game (all lives lost or the dream finished) shows the closing message that Giana wakes up because the sun has chased away the night.
- `stage_clear_tally`: After a cleared stage: counts the remaining time down second by second, adding ten points per second to the score with the tally screen and sound, unless the stage was left by warp.
- `show_warp_message`: After a warp block: waits, clears the message screen and shows the warp-to-level message with the new stage number.
- `play_one_life`: Plays one life: sets up the turn, loads and builds the stage, shows get ready, then runs the per-frame game loop (scores, player, enemies, blocks, bobs, timer, scroll, sprites) until the life ends or the mouse button quits.
- `check_quit_button`: Sets the quit flag when the left mouse button is pressed, which ends the game and returns to the title.
- `start_turn`: Between lives: handles the end message, stage advance after a cleared stage, the high-score screen on game over, lives, and in two-player games saves the current player state and swaps to the other player.
- `set_player_name`: Copies the current player name (Giana or Maria) into the get-ready, status-bar and tally texts.
- `set_start_position`: Chooses where the player starts: the stage start or the halfway checkpoint, or the entry and return points of a bonus stage, and resets the player, scroll and object state for the new life.
- `reset_life_state`: Resets per-life state: death animation, power flags, player sprite set, and clears every object and animation list of the stage.
- `clear_object_list`: Clears the words of an object list up to its end marker.
- `use_normal_player_frames`: Selects the normal player animation frames (eight pointers) for drawing.
- `use_powered_player_frames`: Selects the animation frames of the powered-up player for drawing.
- `draw_status_bar`: Clears the status bar and prints its labels (name, bonus, lives, stage, time) and current values.
- `start_stage_music`: Starts the stage music for the current stage: the castle tune on every fourth stage, otherwise the normal stage tune, unless it is already playing.
- `wait_raster_line`: Waits until the video beam reaches the line given in d6.
- `count_down_time`: Every fifty frames takes one second off the stage timer; when it reaches zero the player loses the life with the out-of-time message.
- `update_status_numbers`: Reprints the score, bonus diamonds, lives and time on the status bar when they have changed since the last frame.
- `move_cloud_sprites`: Moves the background cloud sprites with the scroll (half speed, wrapping at the screen edges) and writes their sprite positions; sprite DMA is off in stages without clouds.
- `set_sprite_position`: Points one sprite at its image and writes its start and stop position words for the given screen position.
- `set_playfield_scroll`: Converts the scroll position into the fine-scroll register value and the four bitplane pointers of the playfield copper list.
- `update_player`: Per-frame player update: reads the joystick, reads the map around the player, handles shots, power timers, block hits, movement and the lift.
- `read_tiles_around_player`: Reads the map tiles in a three-by-four block around the player and stores their collision classes; touching the exit tile marks the stage as cleared.
- `classify_tile_row`: Converts three map tiles into collision classes through the tile class table; the exit tile also sets the stage-cleared flags.
- `player_movement`: Player state machine for one frame: death fall, power-up change, collision summary, jumping and falling, walking left and right and choosing the animation frame.
- `player_jump_state`: Jump and fall state: starts a jump when up is pressed on the ground (with the jump sound), climbs while up is held up to the jump table limit, then falls with the fall table, landing when the feet hit ground.
- `player_air_control`: While in the air, lets the joystick change the horizontal speed within its limits and slows the player when no direction is held.
- `player_vertical_move`: Moves the player vertically by the current jump or fall table entry and picks the jump frame for the facing direction.
- `player_death_check`: Runs the hazard checks and, when the player has just died, plays the death sound and starts the death animation.
- `walk_right`: Walking right: accelerates up to the maximum speed and advances the right-facing walk animation at a speed-dependent rate.
- `walk_left`: Walking left: accelerates up to the maximum speed and advances the left-facing walk animation.
- `slow_down_right`: With no direction held while moving right, reduces the speed by one and keeps animating.
- `slow_down_left`: With no direction held while moving left, reduces the speed by one and keeps animating.
- `player_animation_frames`: Chooses and draws the player frame for the current state (standing, walking, skidding, jumping, lift ride) and runs the death animation that bounces the player off the bottom of the screen.
- `draw_player_frame`: Draws the chosen player frame as a bob at the player position on the playfield.
- `apply_horizontal_speed`: Moves the player horizontally: scrolls the playfield while the player is right of the screen middle and the stage has more to show, otherwise moves the player on screen within the edges.
- `power_up_transformation`: After collecting a power-up, plays the change animation for thirty-two frames with the player frozen, then switches to the powered player frames.
- `summarise_collision`: Counts the solid tiles under the feet and over the head and sets the left-blocked and right-blocked flags, then decides whether the head is blocked and whether there is ground.
- `check_head_blocked`: Decides from the tiles above the player whether the head is blocked.
- `check_ground`: Decides from the tiles under the player whether there is ground under the feet.
- `check_hazards`: Kills the player on falling below the screen, deadly tiles (water, spikes unless protected, flames) and starts a bonus stage or the ending when the player enters their special tiles.
- `set_facing`: Sets the facing direction from the joystick: right faces right, left faces left.
- `stop_at_walls`: Stops horizontal movement into a wall on the blocked side.
- `update_player_shot`: Fires and moves the dream bubble shot of the powered-up player: one shot at a time, bouncing, wall-reflecting or homing depending on the power level, removed off screen.
- `shot_map_collision`: Checks the shot against the map: bounces it off the ground or walls, or ends it with a small explosion and sound at lower power levels.
- `shot_catch_check`: Ends the shot when it comes back into the player box (second power level).
- `shot_bounce_arc`: Moves the shot along its bounce arc from the arc table, reversing the step at the top.
- `find_homing_target`: Finds the first live enemy ahead of the player in the facing direction for the homing shot.
- `homing_shot_step`: Moves the homing shot to the next point of its path, or the fallback point if the tile there is solid.
- `draw_shot`: Draws the shot bob with its animation frame and removes it when it leaves the screen vertically.
- `plot_homing_path`: Computes up to ten line-step points from the shot towards its target (a Bresenham line) for the homing shot.
- `draw_shot_explosion`: Draws the eight-frame explosion where a shot ended.
- `update_power_items`: Handles collected items: an extra life, the freeze-enemies item (fire with down-left) and the kill-all-enemies item (fire with down-right), and counts the freeze timer down.
- `lift_collision`: Checks whether the player stands on the moving lift and, if so, rides it (up, or down in bonus stages).
- `update_crumbling_bridges`: Detects bridge tiles under the player and animates the bridges that are falling away.
- `detect_crumbling_bridge`: If the player stands on a crumbling-bridge tile, registers that bridge piece for falling.
- `animate_crumbling_bridges`: Animates each registered bridge piece for twenty-four frames and then removes its tiles from the map.
- `add_crumbling_bridge`: Adds a bridge piece to the falling list unless it is already there.
- `move_lift`: Moves the lift up and down between its limits; when ridden it climbs and leaves the stage at the top (or carries the player out of a bonus stage).
- `update_block_hits`: Checks every kind of block the player can hit or collect this frame and updates the block animations.
- `collect_diamond_tile`: Collects a free-standing diamond the player touches: removes its tiles, counts a diamond and twenty-five points and plays the collect sound.
- `hit_diamond_block`: Head hits a diamond block: the block becomes empty, bumps, releases a diamond and gives a diamond and points.
- `start_block_bump`: Starts a block bump animation in one of the two bump slots.
- `hit_power_block`: Head hits a power block: it becomes empty, bumps and releases the power item that rises out of it.
- `set_power_item_start`: Stores where the released power item starts.
- `hit_warp_blocks`: Landing on the hidden warp blocks replaces them with empty blocks and starts the warp that skips stages.
- `break_brick`: With a power-up, head hits a breakable brick: it is destroyed with the breaking animation and sound.
- `break_hidden_brick`: With a power-up, head hits the second brick type: it is removed with its own debris animation.
- `hit_multi_diamond_block`: With a power-up, head hits a multi-diamond block: each hit gives a diamond until its count runs out and it turns into a plain block.
- `mark_block_used`: Marks a block position in an animated-tile list as used so it stops animating.
- `update_block_animations`: Runs all block-related animations: rising power item, block bumps, multi-diamond hits, collected diamonds, breaking bricks, debris and dropped blocks.
- `raise_power_item`: Animates the power item rising out of its block, then turns it into a moving item object in the enemy list.
- `bump_animation_a`: Plays block bump animation slot one; at its end after a warp hit, advances two stages and ends the stage.
- `bump_animation_b`: Plays block bump animation slot two.
- `multi_block_animation_a`: Plays multi-diamond block animation slot one.
- `multi_block_animation_b`: Plays multi-diamond block animation slot two.
- `start_multi_block_animation`: Starts a multi-diamond block animation in a free slot.
- `animate_collected_diamonds`: Moves every collected diamond up its flight path and draws it until it disappears.
- `add_collected_diamond`: Adds a diamond flying out of a block to the collected-diamond list.
- `animate_breaking_bricks`: Draws the breaking-brick animation and clears the brick tiles from the map partway through.
- `add_breaking_brick`: Adds a brick to the breaking list.
- `animate_brick_debris`: Plays the debris animation of the second brick type in its two slots.
- `update_dropped_blocks`: Moves every dropped block down until it lands on solid ground, then writes block tiles into the map with a landing sound.
- `add_dropped_block`: Adds a block to the dropped-block list.
- `add_diamond`: Counts one bonus diamond; at one hundred the counter wraps and the player gets an extra life.
- `add_life`: Adds one life and plays the extra-life sound.
- `lose_life`: Takes one life away.
- `next_stage`: Advances to the next stage number and its displayed copy.
- `previous_stage`: Unreferenced routine: steps back one stage number and its displayed copy.
- `add_points`: Adds the pending bonus points to the six-digit score.
- `update_enemies`: Spawns the enemies that scroll into view and updates every live enemy (outside bonus stages).
- `spawn_enemies_in_view`: Walks the stage enemy list and spawns each unused enemy whose column has just scrolled into view.
- `spawn_enemy`: Fills a free enemy record from the enemy type template: speeds, animations, hit points, points and offsets, at the given map position.
- `update_enemy_list`: For every live enemy record: collisions, movement against the map, animation and drawing.
- `enemy_collisions`: Checks one enemy against the player and the shot.
- `enemy_player_collision`: Player against enemy: landing on it kills it with points and a bounce, touching it otherwise kills the player; collecting an item enemy raises the power level.
- `enemy_shot_collision`: Shot against enemy: the shot explodes and the enemy loses a hit point, dying with points at zero.
- `enemy_map_movement`: Moves an enemy against the map by its movement type: turning at walls and edges, falling under gravity and landing on ground.
- `animate_enemy`: Moves, animates and draws one enemy, freezes it while the freeze item runs, and removes it when it leaves the screen.
- `animate_map_tiles`: Draws the four kinds of animated map tiles in view.
- `animate_diamond_blocks`: Animates the shimmer of the diamond blocks in view.
- `animate_power_blocks`: Animates the shimmer of the power blocks in view.
- `animate_flames`: Animates the flame tiles in view over forty frames.
- `animate_water`: Animates the water tiles in view over twenty-four frames.
- `draw_animated_tile_list`: Draws one frame for every position of an animated-tile list that lies in the visible part of the stage.
- `restore_bob_backgrounds`: Restores the saved backgrounds under last frame bobs with the blitter, newest first.
- `load_stage`: Loads the stage map from disk (or copies the bonus stage map), parses it and records the crumbling-bridge markers.
- `build_stage`: Builds a stage: header, sanity check, object names, lift, enemy list, palette, background, block drawing and bridges.
- `parse_stage_header`: Reads the stage width and height and finds the tagged sections (palette, objects, blocks, tiles) in the stage data.
- `find_tag`: Searches forward for a tag word in the stage data.
- `check_stage_data`: Sanity check of the stage graphics: masks the first long of the graphics block and of the tile table and, if they disagree, turns the screen yellow and waits for fire (a load error signal, not a protection check).
- `resolve_object_names`: Looks up each named object (bridge, thorns, bonus, flame, hint) in the stage object table.
- `find_object_name`: Searches the stage object table for a name.
- `find_lift_start`: Finds the lift start tile in the map and sets the lift position and direction.
- `select_enemy_list`: Selects the enemy spawn list of the current stage and marks all its entries unused.
- `set_stage_palette`: Writes the stage palette and the sprite colours into the colour registers.
- `fill_stage_background`: Fills the playfield with the repeating background pattern of the stage.
- `draw_stage_scenery`: Draws the large scenery blocks of the stage into the playfield.
- `draw_stage_tiles`: Draws every tile object of the stage into the playfield with the blitter.
- `draw_crumbling_bridges`: Draws the crumbling-bridge pieces at the recorded marker positions.
- `draw_bob`: Draws a bob on the playfield and, when it reaches into the status-bar area, also into the status bar.
- `queue_bob`: Records the parameters of a bob for drawing later in the frame.
- `draw_queued_bobs`: Draws every queued bob and empties the queue.
- `blit_bob`: Clips a bob against the playfield and blits it with its mask in four planes, saving the background first when asked.
- `save_bob_background`: Copies the playfield area under a bob into the background save list.
- `blit_masked_bob`: Cookie-cut blit of a bob into four bitplanes.
- `wait_blitter`: Waits until the blitter is idle.
- `print_text`: Prints a text string with the eight-by-eight font into a bitplane at the current text cursor, with line breaks.
- `read_joystick`: Reads the joystick in port two and the fire button into direction and fire flags; a forced jump counter overrides up.
- `wait_fire`: Waits for the fire button to be released and pressed again.
- `word_to_hex_text`: Unreferenced debug routine: writes a word as four hexadecimal digits.
- `hex_letter_digit`: Second entry of the hex text routine for digits above nine.
- `bcd_to_text`: Converts packed decimal bytes into digit characters.
- `display_on`: Waits for the beam and switches bitplane, copper and sprite DMA on.
- `display_off`: Waits for the beam and switches bitplane, copper and sprite DMA off.
- `delay_loop`: Busy-wait loop that counts d0 down to zero.
- `build_highscore_copper_list`: Builds the copper list of the high-score screen with its colour bars.
- `clear_highscore_screen`: Clears the high-score screen planes and starts its display.
- `highscore_screen`: High-score sequence after a game: copper list, music, clear, load and format the tables, scroll them in, name entry and table update.
- `load_highscore_track`: Loads the all-time high-score table from disk once per session.
- `format_highscore_tables`: Formats the all-time and today tables (names, scores, stages) into the screen text.
- `scroll_in_highscores`: Scrolls the high-score text into view and fades the colour bars.
- `enter_highscore_name`: If the score qualifies, lets the player enter a name with the joystick, a blinking cursor showing the letter being chosen.
- `insert_highscore`: Inserts the score, name and stage into the all-time and today tables and, on the mouse button, writes the all-time table back to disk.
- `track_loader`: Reads d1 tracks starting at track d0 into memory: decodes each custom MFM track (sync, signature, track number, data, checksum), retrying until it is read correctly.
- `drive_seek`: Selects drive zero, starts the motor, steps to track zero once if needed and then to the requested cylinder and side.
- `wait_drive_ready`: Waits until the drive reports ready.
- `read_raw_track`: Reads one raw track with disk DMA after the sync word, with a timeout.
- `step_to_next_track`: Moves to the next track: switches side, or steps one cylinder in.
- `drive_off`: Stops the motor and deselects the drive.
- `drive_settle_delay`: Short four-instruction delay between drive control changes.
- `drive_step_delay`: Waits after a head step.
- `track_writer`: Writes d1 tracks from memory starting at track d0: builds each track in the custom MFM format with sync, signature, track number and checksum, and writes it (used for the high-score table).
- `drive_seek_for_write`: Selects drive zero, starts the motor and steps to the cylinder and side to write.
- `wait_drive_ready_for_write`: Waits until the drive reports ready before writing.
- `write_raw_track`: Writes one encoded track with disk DMA and waits for the transfer to finish.
- `step_to_next_track_for_write`: Moves to the next track while writing.
- `drive_off_after_write`: Stops the motor and deselects the drive after writing.
- `write_settle_delay`: Short delay between drive control changes while writing.
- `write_step_delay`: Waits after a head step while writing.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `supervisor_start_patched`: Form of the routine supervisor_start as it is in memory under WHDLoad (the slave patched it): supervisor-mode start: sets the stack below the program, runs the one-time machine set-up and the first copper list, then enters the endless title-and-game loop.
- `sequencer_channel_tick_patch_site_754`: WHDLoad patch site inside the routine sequencer_channel_tick: the slave replaced instructions here with a call into its own glue code; the rest is the routine as described there.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `play_one_life_patch_site_120`: WHDLoad patch site inside the routine play_one_life: the slave replaced instructions here with a call into its own glue code; the rest is the routine as described there.
- `check_quit_button_patched`: Form of the routine check_quit_button as it is in memory under WHDLoad (the slave patched it): sets the quit flag when the left mouse button is pressed, which ends the game and returns to the title.
- `wait_raster_line_patched`: Form of the routine wait_raster_line as it is in memory under WHDLoad (the slave patched it): waits until the video beam reaches the line given in d6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `read_joystick_patch_site_18`: WHDLoad patch site inside the routine read_joystick: the slave replaced instructions here with a call into its own glue code; the rest is the routine as described there.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `insert_highscore_patch_site_432`: WHDLoad patch site inside the routine insert_highscore: the slave replaced instructions here with a call into its own glue code; the rest is the routine as described there.
- `track_loader_patched`: Form of the routine track_loader as it is in memory under WHDLoad (the slave patched it): reads d1 tracks starting at track d0 into memory: decodes each custom MFM track (sync, signature, track number, data, checksum), retrying until it is read correctly.
- `track_loader_patch_site_6`: WHDLoad patch site inside the routine track_loader: the slave replaced instructions here with a call into its own glue code; the rest is the routine as described there.
- `drive_seek_patched`: Form of the routine drive_seek as it is in memory under WHDLoad (the slave patched it): selects drive zero, starts the motor, steps to track zero once if needed and then to the requested cylinder and side.
- `drive_seek_patch_site_94`: WHDLoad patch site inside the routine drive_seek: the slave replaced instructions here with a call into its own glue code; the rest is the routine as described there.
- `drive_seek_patch_site_150`: WHDLoad patch site inside the routine drive_seek: the slave replaced instructions here with a call into its own glue code; the rest is the routine as described there.
- `drive_seek_patch_site_190`: WHDLoad patch site inside the routine drive_seek: the slave replaced instructions here with a call into its own glue code; the rest is the routine as described there.
- `wait_drive_ready_patched`: Form of the routine wait_drive_ready as it is in memory under WHDLoad (the slave patched it): waits until the drive reports ready.
- `read_raw_track_patched`: Form of the routine read_raw_track as it is in memory under WHDLoad (the slave patched it): reads one raw track with disk DMA after the sync word, with a timeout.
- `read_raw_track_patch_site_88`: WHDLoad patch site inside the routine read_raw_track: the slave replaced instructions here with a call into its own glue code; the rest is the routine as described there.
- `step_to_next_track_patched`: Form of the routine step_to_next_track as it is in memory under WHDLoad (the slave patched it): moves to the next track: switches side, or steps one cylinder in.
- `drive_off_patched`: Form of the routine drive_off as it is in memory under WHDLoad (the slave patched it): stops the motor and deselects the drive.
- `drive_settle_delay_patched`: Form of the routine drive_settle_delay as it is in memory under WHDLoad (the slave patched it): short four-instruction delay between drive control changes.
- `drive_step_delay_patched`: Form of the routine drive_step_delay as it is in memory under WHDLoad (the slave patched it): waits after a head step.
- `track_writer_patched`: Form of the routine track_writer as it is in memory under WHDLoad (the slave patched it): writes d1 tracks from memory starting at track d0: builds each track in the custom MFM format with sync, signature, track number and checksum, and writes it (used for the high-score table).

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `boot_start`: Boot program: allocates buffers, loads the logo and title data, shows the Time Warp logo and the title picture, loads the game program and jumps to it.
- `boot_allocate_buffers_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): hands out the boot buffers downward from the top of chip memory and derives the logo, title and music addresses.
- `boot_load_tracks_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): calls the track loader for each entry of the boot load table.
- `show_logo_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): unpacks the Time Warp logo picture, sets its palette and shows it until fire is pressed.
- `show_title_picture_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): unpacks the title picture, sets its palette and shows it with the title music until fire is pressed.
- `logo_loop_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): displays the logo and cycles its colours every frame until fire is pressed, then blanks the display.
- `build_logo_copper_list_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): builds the logo copper list and sets up the display for the picture size and depth.
- `cycle_logo_colours_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): every second frame rotates the logo colour table by one and writes it to the colour registers.
- `boot_display_off_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): switches bitplane and copper DMA off.
- `title_picture_loop_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): shows the title picture with the music playing until fire is pressed, then stops the music and blanks the display.
- `build_title_copper_lists_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): builds two alternating copper lists for the title picture and starts the display.
- `boot_wait_raster_line_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): waits until the video beam reaches the line given in d6.
- `boot_delay_loop_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): unreferenced busy-wait loop that counts d0 down to zero.
- `boot_add_copper_pointer_pair_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): appends two copper moves that load a chip address into a pointer register pair.
- `unpack_iff_picture_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): reads an IFF ILBM picture: header, plane pointers and body.
- `read_ilbm_header_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): reads the bitmap header (size, depth, compression) and finds the colour map and body chunks.
- `set_picture_plane_pointers_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): computes the start of each bitplane of the picture buffer.
- `unpack_ilbm_body_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): unpacks the picture body row by row into the bitplanes.
- `unpack_byterun1_row_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): unpacks one bitplane row, copying literal runs or repeating a word (ByteRun1 style, in words).
- `load_picture_palette_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): converts the picture colour map to Amiga colour words and writes them to the colour registers and the cycling table.
- `boot_read_joystick_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): reads the joystick in port two and the fire button into direction and fire flags.
- `boot_relocate_music_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): adds the music bank address to every pointer in the boot music relocation list.
- `boot_start_music_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): starts the title music and switches audio DMA on.
- `boot_stop_music_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): silences the music and switches audio DMA off.
- `boot_start_sequencer_channels_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): initialises the four sequencer channel records for the title music.
- `boot_reset_sequencer_channel_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): resets one sequencer channel record of the boot copy of the music player.
- `boot_silence_channels_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): silences all four audio channels from the boot volume table.
- `boot_silence_one_channel_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): silences one audio channel and writes its volume from the table.
- `boot_music_tick_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): advances the four channels of the boot copy of the music sequencer once.
- `boot_sequencer_channel_tick_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): one tick of a channel in the boot copy of the Beathoven sequencer: note length, pattern commands, portamento and DMA restart.
- `boot_track_loader_raw_copy`: Copy in the raw-loaded boot hunk (the same bytes as the run-time copy): track loader of the boot program; in RAM the slave has replaced it with a jump to its own loader.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `harness_resload_save_file`: Reference-harness stub for the WHDLoad save-file call (not part of the game).
- `harness_resload_get_file_size`: Reference-harness stub for the WHDLoad file-size call (not part of the game).
- `harness_resload_disk_load`: Reference-harness stub for the WHDLoad disk-load call (not part of the game).
- `harness_resload_crc16`: Reference-harness stub for the WHDLoad checksum call (not part of the game).
- `harness_resload_patch`: Reference-harness stub for the WHDLoad patch call (not part of the game).
