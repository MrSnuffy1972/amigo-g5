# Game 4423 (missile (1993)(h. mueller)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04423). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 11 raw-word lines |
| Functions named and described | pass, 47 of 47 named (0 author symbols), 47 of 47 described (12 by rule, 32 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1338 of 1338 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5442 of 5442 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 105484 |
| code | 14154 |
| data | 22554 |
| padding | 720 |
| unused | 758272 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Missile (1993)(H. Mueller).adf:Missile | 2 | 5431 | 45 |
| Missile (1993)(H. Mueller).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- 7 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- missile (1993)(h. mueller)/Missile (1993)(H. Mueller).adf: blocks 212-212 (bytes 108544-109056) outside the filesystem hold data, not code (opcode density 0.004, entropy 0.81)
- missile (1993)(h. mueller)/Missile (1993)(H. Mueller).adf: blocks 215-215 (bytes 110080-110592) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.73)
- missile (1993)(h. mueller)/Missile (1993)(H. Mueller).adf: blocks 217-217 (bytes 111104-111616) outside the filesystem hold data, not code (opcode density 0.004, entropy 0.54)
- missile (1993)(h. mueller)/Missile (1993)(H. Mueller).adf: blocks 883-884 (bytes 452096-453120) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.64)
- missile (1993)(h. mueller)/Missile (1993)(H. Mueller).adf: blocks 1154-1154 (bytes 590848-591360) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.73)
- missile (1993)(h. mueller)/Missile (1993)(H. Mueller).adf: blocks 1157-1157 (bytes 592384-592896) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.77)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Missile (1993)(H. Mueller).adf:Missile

- `initialize_game_system`: Initializes system resources including graphics and interrupt handlers, sets up hardware registers for display and input, and prepares global data structures for game operation.
- `frag_btst_bne_rts`: Short code fragment that tests bit 14 of the memory at an offset from a6, then branches if the last result was not zero, then returns.
- `handle_input_event`: Reads a byte from a global variable, processes it through another routine three times, then writes it back to a table.
- `frag_move_cmp_beq_rts`: Short code fragment that copies the memory at an offset from a6 to d1, then compares d1 with the memory at an offset from a6, then branches if the last result was zero, then returns.
- `reset_missile_pointers`: Resets missile-related pointers and clears specific memory locations when a condition is met, then updates related global variables and handles input state changes based on hardware register values.
- `calculate_line_parameters`: Processes coordinate differences to determine step directions and initialize counters for Bresenham's line algorithm, storing results in a structure at a3.
- `update_position_coordinates`: Updates position coordinates in a structure by adding step values from the structure to current coordinates, and modifies direction flags.
- `update_joystick_input`: Reads joystick and mouse port data to update movement values, clamping them within specific ranges and applying directional adjustments based on input bits.
- `store_object_data`: Writes four data values from registers to a structure at a0, updates a pointer in a global variable, and resets the pointer when it reaches a limit.
- `process_audio_channels`: Processes audio channel data by decrementing counters, setting up DMA registers when counters reach zero, and managing audio playback state transitions.
- `convert_number_to_string`: Converts a number to its string representation in a buffer, handling negative numbers by setting a flag and processing digits through a loop.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `initialize_render_data`: Calculates memory addresses for rendering data based on input coordinates, sets up graphics parameters, and stores pointers to various data structures.
- `setup_render_parameters`: Initializes rendering parameters by setting specific values in a structure at a6, including color data, position information, and graphic pointers.
- `initialize_missile_data`: Initializes missile-related data by setting specific values in a structure referenced through a6, including a global variable pointer, a 16-bit word, and several long words with predefined constants.
- `frag_btst_bne_move_clr_rts`: Short code fragment that tests bit 14 of the memory at an offset from a6, then branches if the last result was not zero, then copies an address to the memory at an offset from a6, then copies an address to the memory at an offset from a6, then clears the memory at an offset from a6 (2 times in a row), then copies 520 to the memory at an offset from a6, then copies 16404 to the memory at an offset from a6, then returns.
- `update_object_animation`: Updates object animation states by checking conditions and calling routines to handle position updates, rendering data setup, and animation counter management based on object properties.
- `update_missile_data`: Updates missile position and state data based on counter values from global variables, with conditional logic determining which counters to decrement and what values to load into registers.
- `process_render_queue`: Processes a render queue by iterating through pointers, updating rendering parameters for each entry, and handling special cases when reaching the end of the queue.
- `frag_lea_movea_move_cmpa_beq_clr_bra_rt`: Short code fragment that loads the address of a location in the code into a5, then copies the memory at a5 to a0, then copies an address to the memory at a5 (advancing a5), then compares a0 with an address, then branches if the last result was zero, then clears the memory at a0, then jumps to another point in the code, then returns.
- `decrement_counter`: Decrements a 16-bit value pointed to by a5 by two, repeated 142 times.
- `set_pixel_bit`: Sets a specific bit in a global buffer based on coordinates, using division and multiplication to calculate the buffer offset and bit position.
- `set_pixel_bit_2`: Sets a bit in a global buffer based on two input coordinates, calculating the buffer offset using division and multiplication operations.
- `clear_pixel_bit`: Clears a specific bit in a table based on two input coordinates, after validating their ranges and calculating an offset using division and multiplication.
- `test_pixel_bit`: Tests if a specific bit is set in a global buffer at given coordinates and returns a flag indicating the result.
- `frag_movem_move_lea_bra`: Short code fragment that saves registers on the stack, then copies a location in the code to d0, then loads the address of a fixed memory address into a0, then jumps to another point in the code.
- `frag_movem_move_lea_bra_2`: Short code fragment that saves registers on the stack, then copies a location in the code to d0, then loads the address of a fixed memory address into a0, then jumps to another point in the code.
- `frag_movem_move_lea_bra_3`: Short code fragment that saves registers on the stack, then copies a location in the code to d0, then loads the address of a fixed memory address into a0, then jumps to another point in the code.
- `copy_pattern_data`: Copies 12 bytes of pattern data from one location to another in a global buffer, using a loop with decrementing counter and pointer arithmetic.
- `initialize_audio_channels`: Clears audio volume registers for both channels, sets up audio parameters including length, period, and channel pointers, then waits for raster lines before enabling DMA and updating volume values in a loop.
- `frag_moveq_bsr_tst_bmi_move_subq_bgt_rt`: Short code fragment that loads 64 into d0, then calls another routine, then tests d0, then branches if the last result was negative, then copies d0 to a fixed memory address (2 times in a row), then subtracts 1 from d0, then tests d0, then branches if the comparison was greater, then returns.
- `wait_vertical_sync`: Waits for vertical blank period by polling a hardware register until the value changes, then decrements a counter and repeats if needed.
- `update_line_parameters`: Processes coordinate differences to determine step directions and initialize counters for Bresenham's line algorithm, storing results in a structure while decrementing a counter until it reaches zero.
- `update_object_counters`: Updates object counters and handles conditional logic based on counter values, including decrementing and resetting counters, and managing object state transitions.
- `update_score_display`: Compares a value with a threshold, updates a global variable if the value is greater, then converts numbers to string representations and copies character pattern data to display buffers.
- `frag_move_bsr_lea_bra`: Short code fragment that copies a location in the code to d2, then calls another routine, then loads the address of a location in the code into a0, then loads the address of a fixed memory address into a1, then jumps to another point in the code.
- `frag_move_bsr_lea`: Short code fragment that copies a location in the code to d2, then calls another routine, then loads the address of a location in the code into a0, then loads the address of a fixed memory address into a1.
- `copy_character_data`: Copies character pattern data from a lookup table to a destination buffer, using a loop that processes 8 bytes per iteration.
- `copy_pattern_bytes`: Copies 9 rows of pattern data from one location to another in a global buffer, with fixed offsets for each row and a decrementing counter.
- `update_animation_state`: Initializes a counter, processes data structures by comparing values and updating counters, then calls another routine to handle animation state changes based on comparison results.
- `initialize_graphics_data`: Initializes graphics parameters and calls another routine twice to set up data structures in a global buffer, using pointer arithmetic and loop control.
- `manage_object_data`: Handles object data management by checking conditions and calling routines to copy pattern data, clearing pointers, and resetting counters based on input values and global state checks.
- `decrement_counter_2`: Decrements a 16-bit value pointed to by a5 multiple times, each decrement being by two, effectively reducing the value by 32 in total.
- `render_pixel_data`: Waits for specific raster line positions, then copies pixel data from a lookup table to a buffer using bit shifting calculations, repeating this process for all 8 rows of pixel data.
- `process_movement_data`: Reads and processes coordinate values from a structure, performs arithmetic operations on them, and updates the structure with new values.

### Missile (1993)(H. Mueller).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
