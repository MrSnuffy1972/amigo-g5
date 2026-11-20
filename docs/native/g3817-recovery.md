# Game 3817 (gnome (1991)(real arcade)[cr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03817). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (925456 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 10 raw-word lines |
| Functions named and described | pass, 81 of 81 named (0 author symbols), 81 of 81 described (10 by rule, 70 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2857 of 2857 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 18432 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5354 of 5354 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 22754 |
| data | 895531 |
| padding | 3 |
| unused | 7168 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Gnome (1991)(Real Arcade)[cr].adf:bootblock | 1 | 116 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4870 | 70 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 324 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 44 | 4 |
| Gnome (1991)(Real Arcade)[cr].adf bytes 1024-19456 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- gnome (1991)(real arcade)[cr]/Gnome (1991)(Real Arcade)[cr].adf: custom-format (non-DOS) disk, 1744 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 2604 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 209 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 44 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- gnome (1991)(real arcade)[cr]/Gnome (1991)(Real Arcade)[cr].adf: bytes 1024-19456 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 address
- gnome (1991)(real arcade)[cr]/Gnome (1991)(Real Arcade)[cr].adf: 1 logged loads cover 18432 bytes of the image
- gnome (1991)(real arcade)[cr]/Gnome (1991)(Real Arcade)[cr].adf: 1 track-loaded blocks found in recovered runtime segments and classified through them
- gnome (1991)(real arcade)[cr]/Gnome (1991)(Real Arcade)[cr].adf: blocks 39-372 (bytes 19968-190976) outside the filesystem hold data, not code (opcode density 0.006, entropy 4.57)
- gnome (1991)(real arcade)[cr]/Gnome (1991)(Real Arcade)[cr].adf: blocks 386-569 (bytes 197632-291840) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.99)
- gnome (1991)(real arcade)[cr]/Gnome (1991)(Real Arcade)[cr].adf: blocks 571-1759 (bytes 292352-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.78)
- 1 never-loaded region(s), 18432 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Gnome (1991)(Real Arcade)[cr].adf:bootblock

- `process_hunk_headers`: Reads and processes AmigaDOS hunk type codes from a data stream, performing different actions based on the code values, including memory allocation and data structure initialization.
- `store_shifted_value`: Reads a longword from the input pointer, shifts it right by two bits, and stores the result at the address in a2.
- `initialize_and_clear`: Sets up a structure with values from a table indexed by a register, then performs two library calls to initialize and clear specific fields in the structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_game_state_3`: Reads joystick input status and updates audio volumes, then processes character data and numeric input for display.
- `update_gnome_animation_2`: Updates gnome animation state by incrementing counters and processing conditional logic based on global variables and table lookups, then writes updated values to memory locations.
- `update_gnome_state`: Updates a global state variable based on conditions involving a table lookup and counter values, then writes the updated counter to memory and returns.
- `update_gnome_position`: Updates the position of a gnome character based on grid coordinates and collision detection, modifying global variables for position and state.
- `update_gnome_position_3`: Updates position coordinates of a gnome object using global variables and table lookups, then checks collision with obstacles and updates sprite entry parameters accordingly.
- `update_gnome_states`: Updates state information for gnome objects in a table, including position counters and flags, then processes conditional logic based on object states and global variables before incrementing the object index.
- `update_gnome_data`: Processes gnome object data in a table, updating positions and states based on calculated values and global coordinates, with conditional logic for different gnome states and boundary checks.
- `process_character_data`: Processes character data from a buffer, converting ASCII digits to numerical values and calling a graphics routine with calculated parameters.
- `handle_numeric_input`: Handles numeric input processing by reading characters from a buffer, skipping spaces, converting digits, and invoking a graphics configuration routine.
- `configure_graphics_loop`: Configures graphics parameters in a loop, processing character data from a buffer and calling a blitter setup routine for each valid character.
- `update_gnome_state_3`: Updates internal game state based on input coordinates and global parameters, then configures graphics rendering parameters and handles collision detection with game objects.
- `adjust_input_values`: Updates numeric input buffer values by adding a constant and performing multiplication operations on stored data.
- `update_input_buffer`: Processes a character buffer by reading bytes and skipping spaces, then converts valid digits to numerical values and stores them in a table.
- `calculate_graphics_parameters`: Calculates scaled coordinates using values from global variables and hardware registers, then stores intermediate results in memory locations.
- `frag_cmp_bgt_move_bra`: Short code fragment that compares d2 with 65533, then branches if the comparison was greater, then copies 0 to a fixed memory address (3 times in a row), then jumps to another point in the code.
- `validate_coordinates`: Compares scaled values from two memory locations and performs arithmetic operations on register values.
- `frag_cmp_bgt_move_bra_2`: Short code fragment that compares d2 with 65534, then branches if the comparison was greater, then copies 0 to a fixed memory address (3 times in a row), then jumps to another point in the code.
- `update_display_coordinates`: Updates display coordinates by performing arithmetic operations on various hardware register values and calculating memory addresses for graphics rendering.
- `update_graphics_parameters`: Calculates and stores blitter parameters for graphics rendering based on input coordinates and global state, then configures multiple custom registers including COLOR00.
- `process_sprite_animation`: Manages sprite animation state by incrementing counters and traversing a data table, updating position and size parameters for sprite rendering through a subroutine call.
- `initialize_object_entry`: Initializes an object entry in a table by calculating position coordinates from hardware registers and a global variable, then stores the calculated values and a byte from a lookup table into the entry.
- `process_gnome_object`: Processes a gnome object by checking its state and updating position data based on collision detection results.
- `increment_and_check`: Increments a counter at offset 14 of a structure and checks if it reaches a limit at offset 18; if so, it resets the counter and performs additional checks on data pointed to by offset 20, updating another counter at offset 16 before returning.
- `manage_gnome_state`: Manages gnome state transitions by decrementing a counter, updating position data, and setting specific values based on the current state identifier.
- `update_gnome_position_2`: Updates the horizontal position of a character based on movement calculations and collision detection with a grid, then adjusts vertical positioning and state flags accordingly.
- `handle_gnome_state`: Updates gnome state based on position and collision detection, modifying velocity and state flags according to boundary conditions.
- `update_position_and_collision`: Updates position coordinates and handles collision detection with boundaries, modifying velocity and state flags based on boundary conditions.
- `update_ball_physics`: Updates the position and velocity of a game ball object, handling collisions with boundaries and modifying object state based on physical interactions.
- `update_game_state`: Processes game object coordinates and updates their state based on collision detection results, modifying values in multiple tables and setting flags.
- `calculate_sprite_position`: Calculates sprite position values using table lookups and arithmetic operations, then calls another routine to update sprite entry parameters.
- `update_game_state_2`: Updates game state variables based on input and global conditions, including decrementing counters and modifying structure fields.
- `update_tile_bits`: Updates bit patterns in a tile grid based on coordinates from a data structure, modifying pixel data through bitwise operations.
- `update_gnome_sprite`: Updates sprite position and renders pixel bits for a gnome character, handling animation frames and boundary checks before writing to video memory.
- `update_gnome_positions`: Updates position and velocity data for multiple gnome objects in a table, calculating values based on global coordinates and hardware register readings before storing them back into the table.
- `update_tile_flags`: Updates tile flags in a table based on conditions, decrementing a global counter when certain flags are changed.
- `loop_and_call`: Processes a loop that decrements a counter in a register, then calls a routine with a specific value before returning.
- `update_and_process`: Updates a counter in a global variable and processes data from a table until a null byte is encountered, then returns.
- `update_audio_volumes`: Reads audio volume values from a table and writes them to the volume registers of four audio channels, then updates a global counter and returns.
- `process_game_state`: Reads a global state value and branches to different handlers based on its value, updating various game timers and counters before returning.
- `update_game_timers`: Updates multiple game timer variables based on a global counter, performs arithmetic operations on various global values, and sets flags in memory depending on comparison results.
- `read_joystick_input`: Reads joystick input from a hardware register and performs bitwise operations to determine movement directions, storing results in two global variables.
- `copy_data_blocks`: Copies data blocks from two different source locations to corresponding destination addresses with a loop that processes 32 elements.
- `frag_movea_move_dbra_rts`: Short code fragment that copies an address to a0, then copies 95 to d0, then copies 0 to the memory at a0 (advancing a0), then loops back while counting down d0, then returns.
- `update_frame_buffer_2`: Copies data from a table to multiple memory locations, performs bit manipulation on a global variable, and writes 4-word values to fixed addresses.
- `update_graphics_pointers`: Updates graphics-related memory pointers by calculating offsets from a table and storing values at specific memory locations, performing multiple pointer arithmetic operations.
- `update_graphics_pointers_2`: Updates graphics-related memory pointers by calculating offsets from a different table and storing values at specific memory locations, performing multiple pointer arithmetic operations.
- `update_graphics_pointers_3`: Updates graphics-related memory pointers by calculating offsets from another table and storing values at specific memory locations, performing multiple pointer arithmetic operations.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `frag_move_addi_tst_beq_jsr_rte`: Short code fragment that copies 1 to a fixed memory address, then adds 1 to a fixed memory address, then tests a fixed memory address, then branches if the last result was zero, then calls another routine, then tests a fixed memory address, then branches if the last result was zero, then calls another routine, then copies 16 to a fixed memory address, then returns from the exception.
- `frag_lea_move_cmpa_ble_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then copies 0 to the memory at a0 (advancing a0), then copies 0 to the memory at a1 (advancing a1), then compares a1 with an address, then branches if the comparison was less or equal, then returns.
- `configure_blitter_parameters_3`: Initializes blitter parameters by calculating memory addresses and setting hardware registers with values derived from table lookups.
- `increment_and_compare_2`: Compares a register value against zero and branches to a specific location when not equal.
- `increment_and_compare`: Increments a counter value and compares it against another value, updating memory locations when conditions are met, then sets a specific byte to a constant value.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `configure_blitter_parameters`: Configures blitter parameters by calculating memory addresses and setting hardware registers with values derived from table lookups and arithmetic operations.
- `configure_blitter_parameters_2`: Configures blitter parameters by calculating memory addresses and setting hardware registers with values derived from table lookups and arithmetic operations, similar to another routine but using different source data.
- `configure_blitter_with_save`: Saves all registers, configures blitter parameters by calculating memory addresses and setting hardware registers with values derived from table lookups and arithmetic operations, then restores all registers before returning.
- `setup_blitter_parameters`: Configures multiple blitter registers with values derived from table lookups and calculations, then sets up blitter operation parameters including source and destination pointers, modulo values, and transfer size.
- `update_sprite_entry`: Updates sprite entry parameters including position, size, and pointers using values from a table and global state, then configures blitter registers for sprite rendering.
- `configure_blitter`: Sets up blitter registers for a graphics operation using parameters from a table, then updates a counter in a global structure.
- `update_sprite_entry_2`: Updates sprite entry parameters including position, size, and pointers in a double-buffered structure, then configures blitter registers for graphics operations.
- `blit_sprite`: Processes sprite data for blitter operations, updating coordinates and initiating hardware rendering when a sprite's active flag is set.
- `update_blitter_parameters`: Calculates and stores blitter parameters for graphics rendering based on input coordinates and global state, then configures multiple custom hardware registers to prepare the blitter for a drawing operation.
- `initialize_blitter`: Reads coordinates from a data structure, initializes blitter registers including source and destination pointers, and sets up blitter control words for a graphics operation.
- `update_sprite_data`: Calculates and stores sprite rendering parameters including position, size, and blitter configuration based on input coordinates and global state, then configures multiple custom hardware registers for blitter operations.
- `initialize_blitter_2`: Reads data from a table using index values, initializes blitter registers with specific values including source and destination pointers, and sets up blitter control and size parameters before returning.
- `setup_blitter_parameters_2`: Configures multiple blitter registers with values derived from table lookups and arithmetic operations, then returns.
- `copy_tile_data`: Copies tile data from a source structure to a destination buffer based on calculated offsets and dimensions, then updates a counter variable.
- `read_joystick_input_2`: Reads joystick movement and fire button states from hardware registers, processes directional inputs, and returns combined button state values.
- `update_frame_buffer`: Updates a frame buffer with data from a table, performs bit manipulation on a global variable, and writes 4 words to a fixed memory address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_gnome_animation`: Processes input event data from a structure, updates position and animation parameters based on event type, and writes transformed values to audio registers.
- `update_gnome_state_2`: Processes input event data from a structure, updates position and animation parameters based on event type, and writes transformed values to another structure.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `process_input_event`: Processes input events based on the event type stored at offset 2 of the structure in a6, updating values at offsets 6 and 16 of another structure in a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_cmp_bcs_move_rts`: Short code fragment that compares d0 with a fixed memory address, then branches if the carry flag is set, then copies d0 to a fixed memory address, then copies d1 to a fixed memory address, then copies d2 to a fixed memory address, then copies a0 to a fixed memory address, then returns.
- `configure_audio_channels`: Configures audio channels by setting up registers for sound playback, copying values to DMA control registers, and managing interrupt flags.
- `frag_subi_bne_bsr_move_rte`: Short code fragment that subtracts 1 from a fixed memory address, then branches if the last result was not zero, then calls another routine, then copies 1 to a fixed memory address, then copies 512 to a fixed memory address, then returns from the exception.
- `frag_clr_move_rts`: Short code fragment that clears a fixed memory address, then copies 512 to a fixed memory address, then copies 15 to a fixed memory address, then copies 512 to a fixed memory address, then copies 0 to a fixed memory address (2 times in a row), then returns.
