# Game 3646 (fireball (1994)(h. mueller)[h titan]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03646). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 39 of 39 named (0 author symbols), 39 of 39 described (9 by rule, 27 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2337 of 2337 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3018 of 3018 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 328529 |
| code | 12738 |
| data | 124706 |
| padding | 1035 |
| unused | 434176 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Fireball (1994)(H. Mueller)[h Titan].adf:Fireball | 2 | 3007 | 37 |
| Fireball (1994)(H. Mueller)[h Titan].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 immediate+address
- fireball (1994)(h. mueller)[h titan]/Fireball (1994)(H. Mueller)[h Titan].adf: blocks 212-212 (bytes 108544-109056) outside the filesystem hold data, not code (opcode density 0.004, entropy 0.81)
- fireball (1994)(h. mueller)[h titan]/Fireball (1994)(H. Mueller)[h Titan].adf: blocks 215-215 (bytes 110080-110592) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.73)
- fireball (1994)(h. mueller)[h titan]/Fireball (1994)(H. Mueller)[h Titan].adf: blocks 217-217 (bytes 111104-111616) outside the filesystem hold data, not code (opcode density 0.004, entropy 0.54)
- fireball (1994)(h. mueller)[h titan]/Fireball (1994)(H. Mueller)[h Titan].adf: blocks 883-884 (bytes 452096-453120) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.64)
- fireball (1994)(h. mueller)[h titan]/Fireball (1994)(H. Mueller)[h Titan].adf: blocks 1154-1154 (bytes 590848-591360) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.73)
- fireball (1994)(h. mueller)[h titan]/Fireball (1994)(H. Mueller)[h Titan].adf: blocks 1157-1157 (bytes 592384-592896) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.77)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Fireball (1994)(H. Mueller)[h Titan].adf:Fireball

- `setup_display_and_interrupts`: Initializes display hardware registers including DMA control, interrupt control, and copper lists, then sets up an interrupt server for vertical blanking.
- `initialize_game_state`: Initializes global variables related to game state and input handling, setting up hardware registers for mouse and joystick input detection.
- `read_mouse_input`: Reads mouse movement data from a hardware port and stores the low and high bytes of the reading into global variables.
- `frag_btst_bne_rts`: Short code fragment that tests bit 14 of the memory at an offset from a6, then branches if the last result was not zero, then returns.
- `process_audio_timing`: Manages audio timing by decrementing a counter and setting up audio channel states, calling routines to update audio frame counters and process audio channel data.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `initialize_audio_registers`: Clears multiple audio volume registers and initializes a table of values, then sets specific volume register values to predefined constants.
- `frag_tst_beq_clr_move_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then clears a fixed memory address (5 times in a row), then copies 15 to a fixed memory address, then returns.
- `update_audio_counter`: Increments a global audio frame counter and resets it to zero when reaching six, then updates audio channel registers based on global state and performs audio parameter adjustments.
- `update_position`: Updates object position based on input parameters and clamps values within defined ranges, writing results to global structure offsets.
- `update_fireball`: Processes fireball object data including position updates, sprite rendering parameters, and state transitions based on object type.
- `frag_movem_lea_move_addq_cmp_bne_rts`: Short code fragment that saves registers on the stack, then loads the address of a fixed memory address into a0, then copies the memory at a0 to d0, then adds 4 to d0, then copies the memory at a0 to d1, then compares d1 with d0, then branches if the last result was not zero, then restores saved registers from the stack, then returns.
- `frag_cmpi_bne_rts`: Short code fragment that compares a fixed memory address with 65535, then branches if the last result was not zero, then returns.
- `update_object_data`: Updates object data fields including position and type, then manages a global counter and performs bounds checking.
- `process_audio_channels`: Processes audio channel data by decrementing counters, setting up DMA registers when counters reach zero, and managing audio playback state transitions.
- `calculate_lookup_values`: Calculates lookup table values using division and multiplication operations, storing results in two separate tables.
- `frag_moveq_move_swap_add_lea_dbra_rts`: Short code fragment that loads 3 into d7, then copies d0 to the memory at an offset from a0, then swaps the halves of d0, then copies d0 to the memory at an offset from a0, then swaps the halves of d0, then adds 40 to d0, then loads the address of the memory at an offset from a0 into a0, then loops back while counting down d7, then returns.
- `copy_pattern_data`: Copies pattern data between multiple memory locations with specific offsets, then updates a sequence of words in a final table.
- `update_sprite_positions`: Calculates and stores sprite coordinates based on global position variables, then updates sprite data pointers and related registers before returning.
- `update_sprite_data`: Calculates sprite coordinates and updates multiple global variables with position and rendering parameters, then waits for a specific bit to clear before returning.
- `update_counters`: Decrements multiple global counters and updates corresponding memory locations with new values based on conditional logic.
- `handle_joystick_input`: Processes joystick input from two ports, updating velocity components based on movement and button states, while managing bounds checking and acceleration adjustments.
- `reset_counters`: Clears two 32-bit values in a structure and sets a third to 100, then decrements a counter and resets it to zero when it goes below zero, while setting another field to 2.
- `fill_screen_pattern`: Writes a 16-bit pattern value to multiple memory locations in two separate data structures, with specific offsets of 160, 320 and 480 bytes from the base address.
- `update_position_and_collision`: Updates object position coordinates and handles collision detection with surrounding tiles, modifying velocity components based on collision results.
- `check_tile_collision`: Examines a tile at a specified position for collision detection and updates its state based on the tile type.
- `update_character_values`: Updates character-related data values in global variables and calls another routine to process character glyph data when certain conditions are met.
- `copy_character_data`: Copies character glyph data from a lookup table to a destination buffer, processing six rows of data for each character and advancing both source and destination pointers.
- `convert_number_to_string`: Converts a numeric value into a string representation by repeatedly subtracting values from a lookup table and storing the resulting digits in memory, handling cases where the input is zero or positive.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `manage_frame_counter`: Decrements a frame counter and resets it to three when it goes below zero, then calls another routine multiple times with different parameters based on predefined values.
- `update_frame_counter`: Decrements a frame counter and resets it to 3 when it goes below zero, then performs multiple calls to another routine with different parameters to update various data fields.
- `frag_subq_bpl_move`: Short code fragment that subtracts 1 from a fixed memory address, then branches if the last result was not negative, then copies 3 to a fixed memory address.
- `adjust_word_values`: Compares and adjusts word values in two memory regions based on bitwise AND operations with masks, incrementing or decrementing the value in one region depending on comparison results and setting a flag when changes occur.
- `decode_bitstream`: Reads compressed data from a bit buffer and writes decompressed bytes to a destination buffer, handling multiple decoding modes based on read values.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Fireball (1994)(H. Mueller)[h Titan].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
