# Game 2654 (alien hunter (1992)(schatztruhe)(de)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02654). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 68 of 68 named (0 author symbols), 68 of 68 described (15 by rule, 51 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1788 of 1788 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3550 of 3550 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 499595 |
| code | 14072 |
| data | 48440 |
| padding | 1157 |
| unused | 337920 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Alien Hunter (1992)(Schatztruhe)(De).adf:Alien-Hunter | 2 | 3531 | 66 |
| Alien Hunter (1992)(Schatztruhe)(De).adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- 5 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Alien Hunter (1992)(Schatztruhe)(De).adf:Alien-Hunter

- `initialize_graphics_system`: Initializes graphics hardware by configuring DMA and interrupt settings, setting up coprocessor pointers, and installing interrupt servers.
- `frag_btst_bne_rts`: Short code fragment that tests bit 14 of the memory at an offset from a6, then branches if the last result was not zero, then returns.
- `frag_movem_moveq_rts`: Short code fragment that saves registers on the stack, then restores saved registers from the stack, then loads 0 into d0, then returns.
- `handle_input_event`: Reads a byte from a global variable, processes it through another routine three times, then writes it back to a table.
- `check_input_event`: Checks input events by comparing a byte value with itself and returning immediately if they match, otherwise continuing with further processing.
- `fill_lookup_table`: Fills a lookup table with calculated values using division and multiplication operations, storing results at memory locations pointed to by a0.
- `process_coordinate_range`: Processes coordinate values by calculating memory addresses using bit shifting and multiplication, updates a counter in a global variable, and calls another routine to handle pixel data processing.
- `update_player_position`: Updates player position coordinates based on joystick input and boundary checks, modifying global variables for horizontal and vertical positioning while managing movement speed and limits.
- `process_input_event`: Reads input coordinates from a buffer, performs boundary checks, and updates flags based on position comparisons.
- `update_pointers_and_values`: Updates pointer values in global variables and copies word-sized data to memory locations pointed to by a0, iterating four times with incrementing addresses.
- `initialize_entity_data`: Initializes entity data by setting specific memory locations to fixed values, updating a global counter, and performing iterative operations that modify pointers and call other routines based on loop counters.
- `process_input_events`: Processes input events by reading coordinates from a buffer and checking against multiple tables of position data, updating flags and animation entries based on matches.
- `setup_render_parameters`: Sets up rendering parameters for graphical objects by calculating memory addresses and storing various values including coordinates, flags, and pointers in global variables.
- `process_object_data`: Processes object data from a linked list, calculates memory offsets, and calls another routine to set up rendering parameters for each object.
- `update_entity_states`: Updates state variables of entities based on their type and timing, including decrementing counters and modifying position or animation parameters.
- `update_entity_state`: Processes entity state updates based on type and coordinate values, modifying counters and performing boundary checks.
- `validate_and_update_grid`: Validates grid coordinates and updates associated data when conditions are met, otherwise performs coordinate adjustments and calls another routine.
- `initialize_render_context`: Initializes rendering context parameters by calculating memory addresses based on input coordinates and calling a graphics routine with specific setup values.
- `frag_moveq_move_nop`: Short code fragment that loads 0 into d0, then copies the memory at a2 to d0, then loads 0 into d1, then copies the memory at an offset from a2 to d1, then pauses for one no-operation step.
- `calculate_and_store_coordinates`: Calculates memory coordinates using bit testing and table lookups, then stores calculated values including pointers and flags in global variables for rendering.
- `update_grid_cell_data`: Updates grid cell data by storing a value at a calculated memory location, then calculates and stores rendering parameters based on bit testing conditions.
- `setup_multi_render_parameters`: Sets up multiple render parameter configurations by calculating memory addresses, storing various values including coordinates, flags, and pointers in global variables.
- `setup_render_parameters_2`: Sets up render parameters by calculating memory addresses and storing coordinate values, flags, and other configuration data for multiple render operations.
- `update_position_data`: Processes movement coordinates for a structure by updating position values based on velocity components, performing boundary checks, and handling collision responses with negative velocity adjustments.
- `validate_and_render`: Validates coordinate boundaries for a structure and performs rendering setup operations including memory address calculations and parameter configuration before updating position data.
- `frag_move_bra`: Short code fragment that copies an address to the memory at a0, then jumps to another point in the code.
- `frag_movem_move_bsr_rts`: Short code fragment that saves registers on the stack, then copies an address to d0, then copies 877 to d1, then copies 228 to d2, then calls another routine, then restores saved registers from the stack, then returns.
- `validate_position`: Checks if given coordinates are within specific bounds and performs a lookup in a table, returning -1 if out of bounds or if a match is found, otherwise returns 0.
- `validate_grid_positions`: Validates grid positions against boundaries and collision data, returning different values based on whether position checks pass or fail.
- `initialize_animation_slot`: Initializes a slot in a global animation table by writing four values: a value from d0, a value from d1, zero, and the constant three, then calls another routine before returning.
- `update_animation_frames`: Updates animation frame data by decrementing timers, incrementing frame counters, and managing entry states in an animation buffer.
- `initialize_explosion_effect`: Initializes explosion effect data by setting position, velocity, and timing values for multiple animation entries, then creates a new animation entry.
- `update_position_and_collision`: Updates position coordinates in a global structure, checks bounds, handles collision detection with a table, and manages animation state changes based on position and collision results.
- `initialize_animation_slot_2`: Initializes a slot in a global animation table by writing four values: a value from d0, a value from d1, zero, and the constant three.
- `check_collision_state`: Checks collision state by examining global variables and updating a flag based on specific conditions related to object positions and states.
- `copy_and_update_tile_data`: Copies tile data from a source buffer to a destination table and updates global variables with specific memory addresses from the copied data.
- `update_sprite_data`: Updates sprite data by calculating memory offsets based on division and multiplication operations, then copies byte values from one memory location to another using a loop that runs 3000 times.
- `frag_movem_move_and_lea_bsr_rts`: Short code fragment that saves registers on the stack, then copies a location in the code to d0, then masks d0 with 15, then loads the address of a fixed memory address into a0, then calls another routine, then restores saved registers from the stack, then returns.
- `loop_copy_pixels`: Loops 10 times, calling another routine each iteration while updating a pointer offset by 1 after each call.
- `handle_pixel_copy_1`: Handles pixel copying by dividing input values, extracting low bits, and calling a copy routine with calculated memory addresses for two different locations.
- `process_pixel_data`: Processes pixel data by calculating row indices using division and masking, then calls another routine with table pointers derived from those indices.
- `copy_pixel_rows`: Copies pixel data from a source buffer to a destination buffer at multiple fixed offsets, handling 13 different row positions with byte-sized transfers.
- `frag_btst_beq_rts_move`: Short code fragment that tests bit 10 of a fixed memory address, then branches if the last result was zero, then returns, then copies 1 to a fixed memory address, then returns.
- `read_joystick_input`: Reads joystick input from hardware registers and sets bits in a global variable based on button states and movement directions, also checking for fire button status.
- `frag_movem_move_add_cmp_bne_rts`: Short code fragment that saves registers on the stack, then copies a fixed memory address to d7, then adds 10 to d7, then copies a fixed memory address to d6, then compares d7 with d6, then branches if the last result was not zero, then restores saved registers from the stack, then returns.
- `frag_move_moveq_bsr_tst_bne_rts`: Short code fragment that copies 400 to a fixed memory address, then copies 1 to a fixed memory address, then copies 31 to a fixed memory address, then copies 3 to a fixed memory address, then loads 0 into d7, then calls another routine, then tests a fixed memory address, then branches if the last result was not zero, then returns.
- `frag_move_moveq_bsr_tst_bne_rts_2`: Short code fragment that copies 272 to a fixed memory address, then copies 1 to a fixed memory address, then copies 31 to a fixed memory address, then copies 3 to a fixed memory address, then loads 0 into d7, then calls another routine, then tests a fixed memory address, then branches if the last result was not zero, then returns.
- `setup_audio_channels_2`: Sets up audio channels by configuring hardware registers for sound playback, including volume, period, and length values for both audio channels.
- `configure_audio_volumes`: Configures audio volumes by copying data to global variables, then iteratively updates audio channel volumes while decrementing a counter.
- `initialize_animation_frame`: Initializes animation frame data by setting up memory pointers and counters, updating timing values, and toggling a flag when a counter reaches zero.
- `wait_for_vsync`: Waits for vertical sync by polling hardware registers and calling multiple routines until specific timing conditions are met, then resets a flag and returns.
- `frag_clr_cmpi_bne_bsr_tst_beq_rts`: Short code fragment that clears a fixed memory address (2 times in a row), then compares a fixed memory address with 255, then branches if the last result was not zero, then compares a fixed memory address with 47, then branches if the last result was not zero, then calls another routine, then tests a fixed memory address, then branches if the last result was zero, then returns.
- `process_input_and_update_animation`: Reads input bytes from a global pointer, processes character case conversion, updates animation counters and state variables based on input values, and manages animation frame data pointers and timing values.
- `setup_audio_channels`: Configures audio channel registers for sound playback by writing specific values to DMA control and audio registers, then calls another routine to complete the setup.
- `reset_animation_state`: Resets animation state variables by clearing several memory locations and setting up initial values for counters and pointers in a data structure.
- `wait_for_vertical_blank`: Waits for vertical blank by polling a hardware register until it equals 255, then polls again until it equals 47, and finally calls another routine.
- `frag_cmpi_bne_bsr_subq_bpl_rts`: Short code fragment that compares a fixed memory address with 255, then branches if the last result was not zero, then compares a fixed memory address with 47, then branches if the last result was not zero, then calls another routine, then subtracts 1 from d7, then branches if the last result was not negative, then returns.
- `process_audio_and_copy_data`: Processes audio channel data by copying waveform data between buffers, configuring audio registers with sound parameters, and updating animation state variables.
- `update_sprite_positions`: Updates sprite position values by comparing current and target values, incrementing or decrementing based on comparison results, and managing a counter for each position component.
- `update_enemy_position`: Updates the position and state of an enemy object based on its current mode, handling movement in X and Y directions with boundary checks and state transitions.
- `frag_moveq_move_and_cmp_beq_rts`: Short code fragment that loads 0 into d0, then copies a fixed memory address to d0, then masks d0 with 6, then compares d0 with the memory at an offset from a0, then branches if the last result was zero, then copies d0 to the memory at an offset from a0, then returns.
- `check_collision`: Tests if a rectangle defined by coordinates in d0-d3 overlaps with entries in a global table, and if not, marks the entry as inactive and decrements a counter.
- `decrement_and_check_counter`: Decrements a global counter and checks if it has reached zero, resetting it to 200 if so.
- `process_audio_channels`: Handles audio channel processing by reading from multiple buffers, updating hardware registers with sound data, and managing DMA control flags for different audio channels.
- `frag_move_lea_moveq_clr_dbra_rts`: Short code fragment that copies an address to a fixed memory address, then loads the address of a location in the code into a0, then loads 47 into d7, then clears the memory at a0 (advancing a0), then loops back while counting down d7, then copies 15 to a fixed memory address, then returns.
- `create_animation_entry`: Creates animation entries in a buffer by writing position, size, and timing data, updating a pointer to the next available slot and resetting it when reaching the end of the buffer.

### Alien Hunter (1992)(Schatztruhe)(De).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
