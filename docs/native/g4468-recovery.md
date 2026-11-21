# Game 4468 (moving (19xx)(h. mueller)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04468). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 40 of 40 named (0 author symbols), 40 of 40 described (8 by rule, 27 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1313 of 1313 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2342 of 2342 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 346648 |
| code | 9036 |
| data | 49484 |
| padding | 912 |
| unused | 495104 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Moving (19xx)(H. Mueller).adf:Moving | 2 | 2331 | 38 |
| Moving (19xx)(H. Mueller).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- 10 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- moving (19xx)(h. mueller)/Moving (19xx)(H. Mueller).adf: blocks 212-212 (bytes 108544-109056) outside the filesystem hold data, not code (opcode density 0.004, entropy 0.81)
- moving (19xx)(h. mueller)/Moving (19xx)(H. Mueller).adf: blocks 215-215 (bytes 110080-110592) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.73)
- moving (19xx)(h. mueller)/Moving (19xx)(H. Mueller).adf: blocks 217-217 (bytes 111104-111616) outside the filesystem hold data, not code (opcode density 0.004, entropy 0.54)
- moving (19xx)(h. mueller)/Moving (19xx)(H. Mueller).adf: blocks 883-884 (bytes 452096-453120) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.64)
- moving (19xx)(h. mueller)/Moving (19xx)(H. Mueller).adf: blocks 1154-1154 (bytes 590848-591360) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.73)
- moving (19xx)(h. mueller)/Moving (19xx)(H. Mueller).adf: blocks 1157-1157 (bytes 592384-592896) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.77)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Moving (19xx)(H. Mueller).adf:Moving

- `initialize_game`: Initializes game state by setting up hardware registers, configuring interrupt servers, and preparing global variables for gameplay.
- `frag_btst_bne_rts`: Short code fragment that tests bit 14 of the memory at an offset from a6, then branches if the last result was not zero, then returns.
- `handle_input_event`: Reads a byte from a global variable, processes it through another routine three times, then writes it back to a table.
- `frag_move_cmp_beq_rts`: Short code fragment that copies the memory at an offset from a6 to d1, then compares d1 with the memory at an offset from a6, then branches if the last result was zero, then returns.
- `initialize_counters`: Clears two global longword counters and performs a loop to compare bytes in a table with specific values, decrementing a lookup table entry when matches are found.
- `initialize_render_buffer`: Calls several routines to set up rendering operations with different parameters, then clears a byte at the end of a calculated buffer address.
- `update_game_state`: Updates game state variables including counters and position data, performs boundary checks on coordinates, and manages game object states through bit manipulation and conditional branching based on memory values.
- `process_joystick_input`: Reads joystick fire button status from a hardware register, updates a global state variable based on the input, and calls another routine to handle sprite data.
- `update_joystick_position`: Reads joystick movement data from hardware registers and updates position coordinates in global variables based on the input.
- `copy_sprite_data`: Copies 16 blocks of sprite data from a source table to two destination buffers, advancing pointers through each copy operation.
- `setup_blitter_dma`: Configures blitter DMA registers with fixed values and waits for a bit to clear before returning.
- `frag_btst_bne_move_bra`: Short code fragment that tests bit 14 of the memory at an offset from a6, then branches if the last result was not zero, then copies an address to a fixed memory address, then copies an address to a fixed memory address, then copies an address to a fixed memory address, then copies an address to a fixed memory address, then copies 6145 to d2, then jumps to another point in the code.
- `initialize_background_dma`: Sets up DMA transfer parameters for background graphics using hardware registers BLTAFWM, BLTAMOD, BLTBPTH, BLTCPTH, BLTDPTH, BLTCON0, BLTCMOD, BLTAPTH, and BLTAPTH. Initializes multiple data pointers and performs looped operations to configure display list entries based on character codes from memory.
- `calculate_movement_flags`: Evaluates character codes from memory locations and sets flags based on comparison results, then maps the flags to a movement value.
- `configure_blitter_dma`: Sets up blitter registers for DMA operations using calculated memory addresses and stores movement-related data in a structure.
- `setup_blitter_render`: Calculates table offsets and configures blitter registers for rendering operations using multiple data sources and fixed values.
- `frag_bsr_rts`: Short code fragment that calls another routine (2 times in a row), then returns.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `convert_number_to_string`: Converts a number into a string representation in base-10, storing the result in a buffer with leading zeros.
- `frag_movem_move_bsr_lea_moveq_rts`: Short code fragment that saves registers on the stack, then copies a location in the code to d2, then calls another routine, then loads the address of a location in the code into a0, then copies 154 to d0, then copies 190 to d1, then loads 4 into d2, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_movem_move_addq_bsr_lea_moveq`: Short code fragment that saves registers on the stack, then copies a location in the code to d2, then adds 1 to d2, then calls another routine, then loads the address of a location in the code into a0, then copies 159 to d0, then copies 208 to d1, then loads 4 into d2, then calls another routine, then restores saved registers from the stack.
- `compute_time_value`: Calculates a time value from multiple components using multiplication and addition, then calls another routine to configure blitter registers for rendering characters.
- `render_multiple_strings`: Processes multiple numeric values through string conversion and calls a routine to configure blitter registers for character rendering with varying parameters.
- `render_character_sequence`: Processes a character sequence to configure blitter registers for rendering each character and continues processing until end marker.
- `update_collision_data`: Updates collision detection data by iterating through a table, clearing memory regions, and setting hardware registers before returning.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `update_frame_counter`: Increments a global byte counter and compares it with another global byte value, branching based on the comparison result.
- `update_audio_channels`: Processes audio channel updates for four sound channels, modifying period values based on global parameters and table lookups, then writes updated period values to audio registers.
- `update_movement_data`: Processes input bytes to update movement parameters including position, velocity, and direction, then stores the results in a structure.
- `process_input_byte_2`: Reads a nibble from a byte in a structure, uses it as an index into a global table, and updates a position value based on movement parameters while clamping it within bounds.
- `copy_position_word`: Copies a 16-bit value from an offset in a structure to another offset in a different structure.
- `calculate_movement_delta_2`: Calculates a movement delta value by indexing into a table based on input parameters, updates a global variable with the result, and sets a control byte based on comparison.
- `update_position_delta`: Updates a position value based on movement parameters and clamps it within bounds, then reads from a lookup table to determine a final value.
- `calculate_movement_delta`: Computes a movement delta value based on input parameters, updates position data in a structure, and modifies related control bytes.
- `process_input_byte`: Processes a nibble value from a byte in a structure, updates global variables based on the value, and modifies hardware registers.
- `store_movement_byte`: Stores a nibble value from a register into a global variable at offset 18, then returns immediately without further processing.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
- `frag_movem_lea_move_addq_cmp_bne_rts`: Short code fragment that saves registers on the stack, then loads the address of a fixed memory address into a0, then copies the memory at a0 to d0, then adds 4 to d0, then copies the memory at a0 to d1, then compares d1 with d0, then branches if the last result was not zero, then restores saved registers from the stack, then returns.

### Moving (19xx)(H. Mueller).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
