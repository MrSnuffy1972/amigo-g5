# Game 2905 (BlackShadow v1.0 0997): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02905). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2029520 top-level bytes in 57 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 72 of 72 named (0 author symbols), 72 of 72 described (10 by rule, 62 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3768 of 3768 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 741990 |
| code | 13440 |
| data | 1212780 |
| padding | 12158 |
| unused | 49152 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Disk.1:BLACK.O | 1 | 3729 | 66 |
| game.slave | 1 | 29 | 5 |
| Disk.1:bootblock | 1 | 10 | 1 |

## Cases handled

- BlackShadow_v1.0_0997/BlackShadow_v1.0_0997.hdf: hard-disk image (AmigaDOS OFS, 2176 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree BlackShadow_v1.0_0997_hdf, which is recovered file by file; no block outside the filesystem holds data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Disk.1:BLACK.O

- `initialize_graphics_system`: Allocates memory for graphics resources and sets up pointers to various data structures within those allocations, including sprite and background data tables.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_game_data`: Sets up multiple global variables with initial values including timer settings, sprite coordinates, and table entries.
- `open_resource_file`: Opens a file resource using the DOS library and performs read operations on it, returning an error code if the operation fails or zero on success.
- `open_animation_file`: Opens an animation file resource using the DOS library and performs read operations on it, returning an error code if the operation fails or zero on success.
- `open_girl_image_file`: Opens a girl image file resource using the DOS library and performs read operations on it, returning an error code if the operation fails or zero on success.
- `open_ship_files`: Opens two ship-related files using the DOS library and performs read operations on them, returning an error code if any operation fails or zero on success.
- `open_boy_image_file`: Opens a boy image file resource using the DOS library and performs read operations on it, returning an error code if the operation fails or zero on success.
- `open_crl_file`: Opens a CRL file resource using the DOS library and performs read operations on it, returning an error code if the operation fails or zero on success.
- `open_jj_file`: Opens a JJ file resource using the DOS library and performs read operations on it, returning an error code if the operation fails or zero on success.
- `open_sound_effects`: Opens multiple sound effect files using the DOS library and performs read operations on them, returning an error code if any operation fails or zero on success.
- `open_weapon_files`: Opens three weapon-related files using the DOS library and performs read operations on them, returning an error code if any operation fails or zero on success.
- `open_resource_file_2`: Opens a resource file using a filename from the program's data section and stores the file handle in a global variable.
- `close_and_return`: Closes a file handle stored in a global variable and returns zero if the handle was valid, otherwise returns immediately.
- `open_alien_files`: Opens and processes eight alien graphic files sequentially, performing file operations for each using a global variable as a function pointer table offset.
- `load_explosion_effects`: Loads left and right explosion effect graphics from resource files into memory using specific buffer sizes and offsets.
- `open_and_read_files`: Opens and reads data from two specific files using operating system calls, storing the results in global variables.
- `load_game_assets`: Opens and reads multiple graphic asset files from disk, loading them into memory buffers with specific sizes and offsets.
- `frag_movea_lea_moveq_move_dbra_rts`: Short code fragment that copies a location in the code to a0, then loads the address of the memory at an offset from a0 into a0, then loads 31 into d7, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then loads the address of the memory at an offset from a0 into a0, then loops back while counting down d7, then returns, then loads 255 into d0, then returns.
- `blit_background_pattern`: Performs multiple bit-block transfers using hardware registers to render a background pattern, then updates counters in global variables.
- `setup_graphics_rendering`: Sets up graphics rendering parameters by configuring blitter DMA registers with specific source and destination addresses, then calls another routine to process the data.
- `rotate_and_combine`: Performs bit rotation and bitwise operations on data from two sources, combining results into a target location.
- `swap_cop_registers`: Exchanges pairs of register values and waits for vertical blanking before writing to coprocessor registers.
- `update_counter_value`: Updates a counter value based on conditional logic and writes the result to memory, with branching for different counter states.
- `process_bitmap_data`: Processes bitmap data by reading from memory locations and performing bit operations on multiple destinations based on calculated offsets.
- `update_joystick_input`: Reads joystick input from hardware registers and updates position variables for two control axes, with boundary checks and velocity adjustments based on input direction.
- `configure_blitter`: Configures blitter registers for graphics operations by setting up control words, dimensions, and waiting for blitter readiness.
- `call_blt_clear`: Library stub: loads its arguments and the graphics library base, calls BltClear and returns its result to the caller.
- `calculate_pixel_positions`: Calculates pixel position values by performing division operations on a counter and calling another routine with different parameters, then returns.
- `bitwise_combine_pixels`: Performs bitwise operations on pixel data from a source location and combines the results with destination data using AND, OR, and NOT operations.
- `bitwise_merge_pixels`: Merges pixel data from a source location with destination data using OR and AND operations, applying NOT to some destinations while preserving others.
- `process_pixel_data`: Processes pixel data by checking a condition and calling another routine with specific parameters, then returns.
- `update_collision_state`: Updates collision detection state variables based on position and condition checks, then performs bit manipulation on a table of data before returning a calculated value.
- `initialize_collision_vars`: Initializes two 16-bit variables to zero and returns immediately.
- `update_shadow_data`: Updates shadow-related global variables based on position and state checks, then modifies game data structures accordingly.
- `initialize_shadow_data`: Sets two global 16-bit variables to zero and returns.
- `update_sprite_data`: Processes sprite rendering data by iterating through a table, performing bitwise operations on coordinates, and updating state variables based on conditional checks.
- `check_collision`: Evaluates collision conditions between game objects and updates global variables based on comparison results.
- `load_background_file`: Loads a background file based on a counter value and updates the counter, returning the new counter value or -1 if an error occurs.
- `load_welcome_file`: Loads a welcome file and returns the result of the file operation or -1 if it fails.
- `setup_audio_channels`: Configures three audio channels with specific parameters and enables DMA for each channel.
- `frag_move_nop_dbra_rts`: Short code fragment that copies 3 to d2, then copies 127 to d0, then copies 127 to d1, then pauses for one no-operation step (3 times in a row), then loops back while counting down d1, then loops back while counting down d0, then loops back while counting down d2, then returns.
- `process_background_layer`: Processes a background layer by applying bit manipulation operations to pixel data and updates internal counters.
- `process_background_layer_2`: Processes a second background layer using similar bit manipulation techniques as the first routine, updating internal state variables.
- `apply_bitwise_operations`: Applies bitwise AND and XOR operations between data from two memory locations and stores the result in a third location.
- `update_collision_data`: Updates collision detection data by comparing coordinates against predefined boundaries and modifying state variables.
- `handle_collision_update`: Updates collision detection coordinates by subtracting eight from x and y values, sets flags, and calls another routine with calculated parameters.
- `update_collision_coordinates`: Updates collision coordinates by subtracting eight from x and y values, sets flags, and calls another routine with calculated parameters.
- `reset_audio_channels`: Resets audio channel pointers to zero and configures their length registers with a fixed value of 2.
- `manage_position_state`: Manages position coordinates and state flags based on conditional checks, then calls another routine with calculated parameters.
- `update_position_and_state`: Updates position coordinates and state flags based on conditional checks, then calls another routine with calculated parameters.
- `handle_object_collision`: Checks for object collisions by comparing coordinate ranges and updates object states and global counters when collisions occur.
- `process_sprite_data`: Processes sprite data by selecting different memory locations based on a condition, performing calculations, and calling another routine with computed values.
- `process_pixel_data_blocks`: Processes pixel data blocks using bitwise operations on word-sized values and updates memory locations with combined results.
- `handle_sprite_rendering`: Handles sprite rendering by iterating through data, performing bitwise operations, and calling another routine with calculated parameters.
- `read_joystick_input`: Reads joystick or mouse input from hardware registers and stores the low and high bytes of the value into global variables, then performs conditional bit operations based on comparison results with threshold values.
- `configure_blitter_dma`: Configures blitter DMA registers for graphics operations by setting up source and destination pointers, transfer sizes, and control words, then waits for blitter readiness.
- `render_background_elements`: Renders background elements by calling a routine with specific parameters and returns.
- `setup_blitter_dma`: Configures blitter DMA registers for graphics operations by setting up source and destination pointers, transfer sizes, and control words, then executes multiple blitter operations while waiting for blitter completion.
- `process_sprite_list`: Processes a list of sprites by iterating through data, performing calculations, and calling another routine with computed values.
- `update_position_flags_2`: Updates position-related flags in a data structure based on comparisons between current coordinates and threshold values, then stores incrementing values in a specific memory location.
- `update_position_flags`: Updates position-related flags in a data structure based on comparisons between current coordinates and threshold values, then stores incremented coordinates in the same structure.
- `process_movement_data`: Processes movement data by updating coordinates based on direction flags, then performs boundary checks and calls another routine for out-of-bounds conditions.
- `frag_movea_move_beq_lea_bra_rts`: Short code fragment that copies a0 to a1, then copies the memory at an offset from a1 to the memory at a1, then copies the memory at an offset from a1 to the memory at an offset from a1 (2 times in a row), then copies the memory at a1 to d0, then branches if the last result was zero, then loads the address of the memory at an offset from a1 into a1, then jumps to another point in the code, then returns.
- `copy_graphics_tile`: Copies graphics tile data from one memory location to another by calculating source and destination addresses based on input parameters, then performs multiple memory copy operations with loop counters.
- `configure_graphics_operations`: Configures graphics operations by setting up blitter registers for graphics operations, processing pixel data blocks using bitwise operations, and performing multiple graphics-related function calls with varying parameters.

### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `update_game_state`: Updates game state variables by modifying flags in memory, adjusting a counter value, setting global pointers, and calling multiple routines through function pointers.
- `frag_cmpa_beq_jmp`: Short code fragment that compares a4 with a location in the code, then branches if the last result was zero, then jumps to another point in the code.
- `frag_lea_bsr_jmp`: Short code fragment that loads the address of the memory at an offset from a5 into a6, then calls another routine, then jumps to the address in a6.
- `frag_move_addi_rts`: Short code fragment that copies a location in the code to the stack, then adds 32 to the top of the stack, then returns.

### Disk.1:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
