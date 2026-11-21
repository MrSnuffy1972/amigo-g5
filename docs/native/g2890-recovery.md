# Game 2890 (BiplaneDuel v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02890). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (194227 top-level bytes in 9 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 45 of 45 named (0 author symbols), 45 of 45 described (11 by rule, 34 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 31928 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1738 of 1738 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 38027 |
| code | 5240 |
| data | 150950 |
| padding | 10 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| bip | 1 | 1731 | 42 |
| game.slave | 1 | 7 | 3 |
| OSEmu.400 (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- BiplaneDuel_v1.0/BiplaneDuel_v1.0.hdf: hard-disk image (AmigaDOS OFS, 224 blocks): its filesystem parses and all 7 files in it are byte-identical to the extracted tree BiplaneDuel_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- BiplaneDuel_v1.0/BiplaneDuel_v1.0_hdf/BiplaneDuel/OSEmu.400: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 31928 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### bip

- `initialize_graphics`: Initializes graphics memory structures and sets up blitter parameters for DMA operations, configuring memory copying registers and waiting for channel availability before initiating transfers.
- `initialize_game_data`: Initializes game object data structures by copying values from tables, setting up memory layouts, and configuring hardware registers for display and audio.
- `update_audio_parameters`: Updates audio channel parameters based on input conditions and calculates values for sound output.
- `update_acceleration`: Processes velocity components to calculate and apply acceleration changes to position values, updating movement direction and speed based on input parameters and conditional flags.
- `update_velocity`: Processes velocity components and updates position values based on calculated acceleration.
- `check_flag_loop`: Loops through a global table checking for negative values at specific offsets, and returns early if any are found.
- `setup_audio_parameters`: Calculates and configures audio sample parameters including volume, period, and length values.
- `update_position`: Updates the position coordinates of an object based on velocity values and boundary checks, then stores the resulting coordinates back to the object structure.
- `update_orientation_state`: Updates orientation state values based on input conditions and bit operations, then calls two other routines to complete the processing.
- `apply_acceleration`: Applies acceleration calculations to velocity components using multiplication and bit shifting operations.
- `scan_bit_field`: Scans a 32-bit value for the first set bit starting from the most significant bit and performs arithmetic operations based on the result.
- `perform_division`: Executes a sequence of arithmetic operations including multiplication, subtraction, and division.
- `update_biplane_state`: Updates biplane state variables including position and animation frame, then configures audio DMA settings based on the biplane's current configuration.
- `decrement_and_update`: Decrements a counter value at a specific memory location and performs bit clearing operations when the counter reaches zero.
- `update_position_bounds`: Updates object coordinates with boundary checks and performs arithmetic operations on position values before storing the results back to memory.
- `update_biplane_position`: Updates biplane object coordinates with boundary checks and performs arithmetic operations on position values before storing the results back to memory.
- `frag_btst_bne`: Short code fragment that tests bit 5 of d0, then branches if the last result was not zero.
- `frag_addq_ori_st_rts`: Short code fragment that adds 1 to the memory at an offset from a5, then merges 36 into d0, then sets the memory at an offset from a5 to all ones, then returns.
- `frag_btst_bne_2`: Short code fragment that tests bit 13 of d0, then branches if the last result was not zero.
- `frag_addq_ori_bra`: Short code fragment that adds 1 to the memory at an offset from a5, then merges 9216 into d0, then jumps to another point in the code.
- `process_biplane_collision`: Tests collision conditions between two biplanes using bit flags and distance calculations, updates counters based on collision outcomes, and stores updated state information back to both biplane structures.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (lsr.l, lsr.l) and then falls through into the code that follows, with no return of its own.
- `calculate_distance`: Calculates the distance between two points using bit shifting and subtraction, then compares the result against a threshold value to determine if they are close enough.
- `setup_dma_and_audio`: Configures DMA channels and audio parameters by setting up various hardware registers, updating global counters, and writing values to specific memory locations.
- `update_biplane_states`: Updates biplane state data including counters, positions, and color registers while managing game object lifecycle, then returns.
- `update_biplane_data`: Updates biplane position and state data from a table, then configures blitter registers for DMA transfer operations based on game timing conditions.
- `setup_blitter_dma`: Configures blitter registers for memory copying operations and waits for DMA channels to become available before setting up the transfer parameters.
- `frag_moveq_lea_move_swap_dbra_movea_rts`: Short code fragment that loads 15 into d0, then loads the address of the memory at an offset from a1 into a1, then copies a1 to d1, then copies the memory at a0 (advancing a0) to d2, then swaps the halves of d2, then copies d2 to the memory just below a1 (moving a1 down), then loops back while counting down d0, then copies d1 to a1, then returns.
- `clear_and_set_bits`: Clears a block of memory and then processes bit patterns from source data to set specific bits in destination memory locations.
- `copy_color_data`: Copies word values from a source table to two destinations, updating color register values and a global variable.
- `process_data_block`: Reads a byte from a buffer and processes it based on its value, either copying data directly or handling negative values with specific arithmetic operations, while updating a counter and checking against a threshold.
- `check_and_store_data`: Reads a bit from a structure, branches if set, compares a value with nine, and stores data at specific offsets in a structure.
- `copy_block_data`: Copies 32 blocks of 5 bytes each from one memory region to another, advancing pointers after each block copy operation.
- `copy_offset_data`: Calculates an offset based on input parameters and copies 2 blocks of 5 bytes each from a source to two destination locations with offset advancement.
- `process_dma_blocks`: Configures and executes multiple DMA transfer operations by calling another routine twice in sequence, then updates memory pointers and loops for six iterations.
- `setup_dma_transfer`: Configures DMA transfer parameters for blitter operations, sets up register values for multiple blitter channels, and waits for channel availability before initiating transfers.
- `frag_movem_move_eor_rts`: Short code fragment that saves registers on the stack, then copies a location in the code to d0, then copies a fixed memory address to d1, then exclusive-ors d1 into d0, then copies a fixed memory address to d1, then exclusive-ors d1 into d0, then copies d0 to the memory at an offset from a5, then restores saved registers from the stack, then returns.
- `update_dma_control`: Decrements a global counter and branches to different code paths based on its value, ultimately configuring DMA control registers and performing memory management operations before returning.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs three instructions (btst.l, ori.b, ori.b) and then falls through into the code that follows, with no return of its own.
- `initialize_render_loop`: Sets up loop parameters for rendering operations and calls another routine 12 times while incrementing a pointer address.
- `read_joystick_input`: Reads joystick input from a hardware port, processes the input to detect specific button states, and returns a combined state value after polling.
- `process_input_and_dma`: Processes joystick input, updates game state based on input conditions, and configures blitter DMA registers for memory transfers.

### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_pea_bra`: Short code fragment that pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_move_addq_rts`: Short code fragment that copies a location in the code to the stack, then adds 4 to the top of the stack, then returns.
