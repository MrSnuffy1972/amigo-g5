# Game 608 (E-Motion): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00608). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904784 top-level bytes in 9 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 45 of 45 named (0 author symbols), 45 of 45 described (1 by rule, 44 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 895 of 895 (100.00%) |
| Independent decoder cross-check (Ghidra against capstone) | 1263 of 1263 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 345862 |
| code | 4666 |
| data | 42597 |
| padding | 1707 |
| unused | 509952 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| E-Motion.adf:e_motion | 3 | 363 | 16 |
| E-Motion.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 382 | 17 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 504 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |

## Cases handled

- runtime segment an address: 295 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 451 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- E-Motion/E-Motion.adf:e_motion: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Functions


### E-Motion.adf:e_motion

- `run_game_entry`: This is the program entry point that starts the game.
- `decode_animation_frames`: This routine decodes a compressed animation frame stream into the game's frame buffer and updates the colour register used for palette effects.
- `set_player_direction`: The routine reads the current player direction from the object structure and stores a packed direction byte for the sprite.
- `wait_for_input_bits`: The routine waits for a series of input bits to be consumed, repeatedly shifting the input value and refilling it from the input source when it runs out, until the required count is exhausted.
- `initialize_game_state`: This routine sets up the Amiga custom hardware registers and memory for the game, initializes global state and loads resources, then starts the main game loop.
- `install_interrupt_handlers`: The routine copies a small set of interrupt handler entry points into the system interrupt vector table and then immediately invokes the first installed handler.
- `copy_palette_data`: Copies a small block of palette data from a source to a destination in memory.
- `check_and_close_window`: This routine checks whether a window is open and, if so, closes it.
- `run_game_loop`: This routine drives the main gameplay loop for E-Motion.
- `run_game_loop_2`: This routine drives one frame of E-Motion gameplay.
- `wait_for_input`: This routine pauses the game by repeatedly calling a small delay routine until a short counter expires.
- `decode_and_apply_palette`: The routine decodes a packed palette block marked with a magic tag and expands it into the game's colour table, then applies a series of byte-wise transformations to the screen buffer using the decoded data.
- `run_game_tick_loop`: This routine drives a fixed number of game update steps, repeatedly invoking the per-frame logic and the per-frame rendering.
- `set_palette_fade`: This routine fades the Amiga screen palette by adjusting the intensity of the first 32 color registers.
- `wait_for_scanline`: This routine blocks execution until the Amiga video beam reaches a specific scanline range.
- `init_sprite_cop_list`: Initializes a set of sprite cop entries for the game’s display list.

### E-Motion.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_game_state_handler`: This routine installs the game’s main state handler and then halts execution.
- `initialize_game_state_2`: This routine runs at startup to set up the game environment before the first frame is drawn.
- `delay_frame_count`: This routine burns a fixed number of CPU cycles to create a short pause in the game.
- `copy_code_bytes`: The routine copies a small block of code into a writable area and then continues execution by jumping to a game routine and a callback stored in the data segment.
- `initialize_game_state_3`: This routine sets up the initial game state and memory tables for E-Motion, preparing the title screen and input handling before the first frame is drawn.
- `load_animation_frames`: Loads animation frame data from a packed table into the active sprite buffer and copies a small code stub into RAM. It is used to prepare the next set of animation frames for the player or objects.
- `copy_word_pairs_to_buffer`: The routine copies a series of 16-bit word pairs from a source value into a destination buffer, advancing the buffer each iteration.
- `update_game_state`: This routine refreshes the game state by running the core update steps and then copies a small block of data from ROM into RAM. It is invoked from the main loop and from the pause and game-over handlers.
- `reset_game_state`: This routine clears a set of global state variables and installs a default jump vector, then halts execution.
- `set_game_state`: The routine writes a constant value into a global variable that the game uses to track its current state.
- `wait_for_vblank`: The routine pauses execution until the display reaches the vertical blank interval.
- `update_hud_digit_display`: The routine updates the on-screen HUD digit for the selected player slot, adjusting the scoreboard offsets and the digit counter, then copies the chosen digit graphics into the blitter buffer for display.
- `update_animation_frames`: This routine advances two animation frame indices and copies the corresponding frame data into the sprite tables used for rendering the player character.
- `update_emotion_palette`: This routine updates the emotion palette and related display data for the current character expression.
- `update_bob_animation`: This routine advances the bob animation for the player character and refreshes the sprite data in chip memory.
- `copy_palette_data_2`: This routine copies a small block of palette data into the hardware palette registers and updates the palette pointer.
- `initiate_game_loop`: This routine performs the initial setup for starting a new game session.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_input_interrupt`: Sets up the game’s input interrupt handler so player controls are processed each frame.
- `disable_interrupt_server`: The routine disables the Amiga interrupt server and sets a flag to indicate the interrupt system is off.
- `initiate_emotion_sprites`: The routine initializes the sprite setup for the E-Motion title screen.
- `trigger_game_over`: This routine is executed during the end-of-level sequence and initiates the game-over handling.
- `update_sprite_positions`: The routine updates the positions of the player and enemy sprites each frame and refreshes the hardware sprite pointers.
- `process_input_events`: The routine processes the current input event table for the game, updating per-player state such as sprite selection, movement, and control flags.
- `self`: This routine initializes and updates a music or sound channel entry for the E-Motion title screen or gameplay, loading instrument and note data from the game’s tables and configuring playback parameters.
- `set_player_sprite_offset`: The routine copies a sprite offset value into the player object structure.
- `update_audio_channels`: Iterates over the set of audio channel descriptors and refreshes each active channel by invoking the channel update routine.
- `update_animation_frames_2`: Advances per-object animation timers and loads the next frame data for palette, colour and sprite animations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_game_state`: This routine is called at startup to prepare the game.
