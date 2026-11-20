# Game 3677 (flotte (1993)(h. mueller)[h titan]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03677). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 32 of 32 named (0 author symbols), 32 of 32 described (7 by rule, 24 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1125 of 1125 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1803 of 1803 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 185902 |
| code | 7542 |
| data | 27700 |
| padding | 1064 |
| unused | 678912 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Flotte (1993)(H. Mueller)[h Titan].adf:Flotte | 2 | 1793 | 31 |
| Flotte (1993)(H. Mueller)[h Titan].adf:bootblock | 1 | 10 | 1 |

## Cases handled

- 9 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- flotte (1993)(h. mueller)[h titan]/Flotte (1993)(H. Mueller)[h Titan].adf: blocks 212-212 (bytes 108544-109056) outside the filesystem hold data, not code (opcode density 0.004, entropy 0.81)
- flotte (1993)(h. mueller)[h titan]/Flotte (1993)(H. Mueller)[h Titan].adf: blocks 215-215 (bytes 110080-110592) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.73)
- flotte (1993)(h. mueller)[h titan]/Flotte (1993)(H. Mueller)[h Titan].adf: blocks 217-217 (bytes 111104-111616) outside the filesystem hold data, not code (opcode density 0.004, entropy 0.54)
- flotte (1993)(h. mueller)[h titan]/Flotte (1993)(H. Mueller)[h Titan].adf: blocks 883-884 (bytes 452096-453120) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.64)
- flotte (1993)(h. mueller)[h titan]/Flotte (1993)(H. Mueller)[h Titan].adf: blocks 1154-1154 (bytes 590848-591360) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.73)
- flotte (1993)(h. mueller)[h titan]/Flotte (1993)(H. Mueller)[h Titan].adf: blocks 1157-1157 (bytes 592384-592896) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.77)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Flotte (1993)(H. Mueller)[h Titan].adf:Flotte

- `initialize_dma_and_interrupts_2`: Initializes DMA and interrupt settings, configures custom hardware registers including coprocessor and blitter control, and sets up system interrupts for video synchronization and input handling.
- `frag_btst_bne_rts`: Short code fragment that tests bit 14 of the memory at an offset from a6, then branches if the last result was not zero, then returns.
- `process_input_byte`: Reads a byte from a global variable, processes it through another routine three times, then writes the result to a table.
- `frag_move_cmp_beq_rts`: Short code fragment that copies the memory at an offset from a6 to d1, then compares d1 with the memory at an offset from a6, then branches if the last result was zero, then returns.
- `handle_input_and_animation`: Processes joystick input by reading hardware registers and updates internal state flags accordingly.
- `update_joystick_position`: Reads joystick movement data from hardware registers and updates global variables representing horizontal and vertical positions based on button states.
- `update_object_properties`: Writes register values to memory locations in a structure, updates queue pointer with bounds checking, and manages object state transitions.
- `process_audio_channels`: Decrements audio channel counters, sets up DMA registers when counters reach zero, and manages audio playback state transitions.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `setup_display_parameters`: Initializes display-related parameters including memory addresses and values for video registers, then copies data from a buffer to global variables and sets up control words.
- `copy_vertex_data`: Copies vertex data from a source table to destination buffers, processes 16 iterations of data transfer with incrementing pointers.
- `initialize_dma_and_interrupts`: Configures DMA and interrupt settings for hardware operations by writing to control registers.
- `frag_addq_move_lsl_rts`: Short code fragment that adds 2 to d0, then copies d0 to a fixed memory address, then shifts d1 left by 3, then copies d1 to a fixed memory address, then returns.
- `setup_display_parameters_2`: Sets up display parameters for video rendering by copying coordinates and configuration data, then calls bit testing routines.
- `decode_numeric_sequence`: Processes a sequence of characters to extract numeric values, performs iterative arithmetic, and manipulates string pointers until reaching a specific marker.
- `convert_number_to_string`: Converts a number to a string representation, handles negative numbers by setting a sign flag, and builds the digit string in reverse order.
- `frag_move_beq_bsr_bra_rts`: Short code fragment that copies a location in the code to d4, then copies a location in the code to d5, then copies the memory at a4 (advancing a4) to d2, then branches if the last result was zero, then calls another routine, then jumps to another point in the code, then copies d4 to a fixed memory address, then returns.
- `copy_pixel_bytes`: Copies eight bytes from a source table to an offset in a global buffer, using indexed addressing with a counter and a shift operation.
- `initialize_game_state`: Clears a global variable array of 800 bytes, initializes DMA and interrupt settings, then performs a series of operations including checking game conditions, updating counters, and managing projectile states before returning.
- `process_shot_coordinates`: Processes player shot coordinates by validating and normalizing input values, then stores normalized results to global variables.
- `process_shot_hit`: Handles shot hit logic by checking target coordinates, updating hit status, and managing game state transitions based on target type.
- `check_and_loop`: Checks conditions on two global variables and performs a loop of up to 19 iterations, calling another routine in each iteration if the conditions are met.
- `frag_moveq_bsr_movea_rts`: Short code fragment that loads 0 into d0, then loads 28 into d1, then calls another routine, then copies 9424 to a4, then calls another routine, then returns.
- `initialize_projectile`: Initializes projectile movement parameters based on input coordinates and validates collision data before setting up trajectory vectors.
- `update_position_and_draw`: Processes coordinate values to calculate position offsets, updates global variables with these calculations, and draws pixels at calculated screen positions using a table lookup mechanism.
- `validate_collision_data`: Validates collision data by testing multiple byte conditions against specific offsets, returns 0 if all tests pass or 1 otherwise.
- `frag_moveq_move_lsr_eor_and_rts`: Short code fragment that loads 0 into d0, then copies a fixed memory address to d0, then shifts d0 right by 8, then copies a fixed memory address to d1, then exclusive-ors d1 into d0, then masks d0 with 255, then returns.
- `calculate_graphics_offset`: Processes a input value by shifting it right, using the result to index into a table, multiplying and adding constants to compute a memory address, then calls another routine with specific parameters.
- `update_graphics_data`: Updates graphics-related parameters including position coordinates and memory pointers based on input values.
- `initialize_rendering_state`: Saves all registers, performs bit testing on hardware status, loads data from tables into memory locations, updates graphics parameters, and calls other routines to set up rendering state for multiple objects.
- `update_object_rendering`: Repeatedly waits for vertical blank interrupts, loads object data from tables, performs calculations based on loop counter, updates graphics parameters, and calls routines to handle object rendering with varying parameters.

### Flotte (1993)(H. Mueller)[h Titan].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
