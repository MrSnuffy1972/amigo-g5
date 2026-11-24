# Game 4466 (mover (1992)(h. mueller)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04466). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903440 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 81 of 81 named (0 author symbols), 81 of 81 described (35 by rule, 42 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1553 of 1553 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2671 of 2671 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 359210 |
| code | 11550 |
| data | 41487 |
| padding | 697 |
| unused | 490496 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Mover (1992)(H. Mueller).adf:Mover | 2 | 2638 | 76 |
| Mover (1992)(H. Mueller).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 4 |

## Cases handled

- 11 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 immediate+address
- mover (1992)(h. mueller)/Mover (1992)(H. Mueller).adf: blocks 212-212 (bytes 108544-109056) outside the filesystem hold data, not code (opcode density 0.004, entropy 0.81)
- mover (1992)(h. mueller)/Mover (1992)(H. Mueller).adf: blocks 215-215 (bytes 110080-110592) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.73)
- mover (1992)(h. mueller)/Mover (1992)(H. Mueller).adf: blocks 217-217 (bytes 111104-111616) outside the filesystem hold data, not code (opcode density 0.004, entropy 0.54)
- mover (1992)(h. mueller)/Mover (1992)(H. Mueller).adf: blocks 883-884 (bytes 452096-453120) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.64)
- mover (1992)(h. mueller)/Mover (1992)(H. Mueller).adf: blocks 1154-1154 (bytes 590848-591360) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.73)
- mover (1992)(h. mueller)/Mover (1992)(H. Mueller).adf: blocks 1157-1157 (bytes 592384-592896) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.77)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Mover (1992)(H. Mueller).adf:Mover

- `initialize_game_state_2`: Initializes global variables related to game state and hardware registers, sets up interrupt servers, and configures display and audio systems.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `handle_input_event`: Reads a byte from a global variable, processes it through another routine three times, then writes it back to a table.
- `frag_move_cmp_beq_rts`: Short code fragment that copies the memory at an offset from a6 to d1, then compares d1 with the memory at an offset from a6, then branches if the last result was zero, then returns.
- `update_game_state_2`: Increments a global counter and conditionally updates game state variables based on comparisons with other values, then performs a series of operations on a data table before returning.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `wait_for_raster_line_2`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `wait_for_raster_line_3`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `frag_btst_beq_bne_rts`: Short code fragment that tests bit 7 of a fixed memory address, then branches if the last result was zero, then tests bit 6 of a fixed memory address, then branches if the last result was not zero, then returns.
- `initialize_counters`: Initializes two counter variables with specific values and stores them in global memory locations, then performs nested loops that decrement these counters and call another routine.
- `frag_movem_move_lsl_lea_adda_mulu_bra`: Short code fragment that saves registers on the stack, then copies a location in the code to d1, then shifts d1 left by 2, then loads the address of a location in the code into a0, then adds d1 to a0, then copies a location in the code to d1, then multiplies d1 by 80 (unsigned), then adds d1 to a0, then copies the memory at a0 to d3, then jumps to another point in the code.
- `setup_blitter_dma`: Configures blitter registers for memory-to-memory transfers using a loop of four iterations, setting up source and destination pointers with specific offsets and transfer sizes.
- `setup_blitter_transfer`: Configures blitter registers for memory transfers, updates counter values, and sets specific register values based on conditional checks.
- `format_number_string`: Converts a signed integer value into an 8-character ASCII string representation with leading zeros, handling negative numbers by negating the input and setting a sign flag.
- `update_joystick_input`: Processes mouse and joystick input data to update two global variables representing horizontal and vertical movement values, clamping them within specific ranges and applying directional adjustments.
- `frag_bsr_rts`: Short code fragment that calls another routine (6 times in a row), then returns.
- `frag_move_cmp_bne_rts_bsr_movea`: Short code fragment that copies a location in the code to d2, then compares d2 with a location in the code, then branches if the last result was not zero, then returns, then copies d2 to a fixed memory address, then calls another routine, then copies an address to a1, then copies an address to a2, then calls another routine, then returns.
- `frag_move_cmp_bne_rts_bsr_movea_2`: Short code fragment that copies a location in the code to d2, then compares d2 with a location in the code, then branches if the last result was not zero, then returns, then copies d2 to a fixed memory address, then calls another routine, then copies an address to a1, then copies an address to a2, then calls another routine, then returns.
- `check_and_update_value`: Compares a value from memory with another value and updates the second if they differ, then calls another routine twice before returning.
- `compare_and_store_value`: Checks if two memory values are equal and updates a target location with the source value if they differ, followed by two calls to another routine before returning.
- `update_and_render_value`: Updates a memory location with an incremented value and then calls another routine twice, handling sprite rendering operations with specific parameters.
- `frag_cmpi_beq_subq_bpl`: Short code fragment that compares a fixed memory address with 1, then branches if the last result was zero, then subtracts 1 from a fixed memory address, then branches if the last result was not negative.
- `manage_sprite_counters`: Manages sprite counter values by decrementing them and setting flags, updating memory locations with new values when counters change, and calling rendering routines.
- `render_character_sprite`: Draws a character sprite using the blitter by setting up transfer parameters based on a bit flag, then executing four blitter operations with specific register values and incrementing pointers.
- `process_input_sequence`: Processes a sequence of input bytes, storing values at specific memory locations when encountering a delimiter byte, and continues processing until end marker is found.
- `handle_text_input`: Processes text input events including backspace and carriage return keys, updating cursor position and line tracking variables, and renders character sprites using the blitter for display.
- `initialize_audio_channels`: Initializes audio channel registers by clearing volume settings, setting up frequency values, and configuring multiple audio parameters including sample data pointers and volume controls.
- `reset_audio_dma`: Clears a global byte, clears the four audio volume registers, and sets a value in a register that controls DMA or audio settings before returning.
- `update_audio_channels`: Updates audio channel registers based on state variables and performs volume or period adjustments for sound effects, while managing a frame counter that resets after reaching a specific value.
- `update_object_position`: Updates object position and velocity based on input parameters and state, clamping values within defined ranges before storing results.
- `update_fireball`: Processes fireball object data including position updates, sprite rendering parameters, and state transitions based on object type.
- `check_and_update_flags`: Checks byte sequences in a global area for zero values and updates flags based on the results.
- `compare_and_advance`: Compares bytes from two memory locations, advances pointers when bytes match, and loops until a mismatch or counter expires.
- `frag_cmp_bne_rts`: Short code fragment that compares d0 with a location in the code, then branches if the last result was not zero, then returns.
- `update_game_state`: Updates game state based on input from joystick or mouse, modifies position coordinates and global flags, and configures audio registers for sound effects.
- `initialize_blitter`: Configures blitter registers based on input parameters and initializes memory patterns, then performs multiple blitter operations with varying source and destination pointers before returning.
- `update_position_bits`: Updates position bits in a structure by comparing and adjusting values in multiple bit planes, then clears a control flag when no changes occur.
- `frag_move_bsr`: Short code fragment that copies 1 to a fixed memory address, then calls another routine, then copies an address to a fixed memory address, then copies 3552 to a fixed memory address, then copies 198 to a fixed memory address, then copies 64 to a fixed memory address, then copies 32769 to a fixed memory address, then calls another routine, then copies 2 to a fixed memory address.
- `frag_movem_lea_move_addq_cmp_bne_rts`: Short code fragment that saves registers on the stack, then loads the address of a fixed memory address into a0, then copies the memory at a0 to d0, then adds 5 to d0, then copies the memory at a0 to d1, then compares d1 with d0, then branches if the last result was not zero, then restores saved registers from the stack, then returns.
- `update_video_timings`: Updates video timing registers with specific values, adjusts a counter, and modifies register settings based on the counter's value before returning.
- `initialize_display_mode`: Sets up display parameters by storing specific values to global variables, increments a counter through a loop, and updates the display mode based on the counter's value.
- `frag_move_swap_rts`: Short code fragment that copies 420 to d0, then copies d0 to a fixed memory address, then swaps the halves of d0, then copies d0 to a fixed memory address, then returns.
- `initialize_graphics`: Sets up graphics parameters by storing a word value at two specific memory locations and returns immediately.
- `frag_move_cmpi_beq_rts`: Short code fragment that copies 1 to a fixed memory address, then compares a fixed memory address with 1, then branches if the last result was zero, then returns.
- `frag_move_bra`: Short code fragment that copies an address to a fixed memory address (7 times in a row), then jumps to another point in the code.
- `frag_clr_move_bsr_rts`: Short code fragment that clears a fixed memory address, then copies 152 to a fixed memory address, then copies 11552 to a fixed memory address, then copies 32 to a fixed memory address, then calls another routine, then returns.
- `frag_lea_move_mulu_adda_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies a location in the code to d0, then multiplies d0 by 986 (unsigned), then adds d0 to a0, then loads the address of a location in the code into a1, then copies 985 to d0, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then returns.
- `initialize_game_state`: Initializes game state variables by copying data from a table, clears counters, configures audio and blitter registers, and sets up sprite management.
- `swap_memory_pointers`: Swaps the contents of two memory locations by loading addresses from a table, copying data between them, and updating the table entries.
- `frag_move_bra_2`: Short code fragment that copies an address to d0, then jumps to another point in the code.
- `fill_pattern_table`: Fills a pattern table with sequential values by writing word pairs to memory locations at fixed offsets, repeating four times.
- `blit_pattern_data`: Configures and executes multiple blitter operations to copy pattern data between memory locations using specific size and address settings.
- `blit_large_pattern`: Configures and executes multiple blitter operations to copy large pattern data with specific size settings and address increments.
- `process_character`: Reads characters from a source pointer, compares them with a table, and performs blitter operations with varying configurations when a match is found, updating a global counter after each operation.
- `configure_blitter`: Configures multiple blitter registers with specific values based on input parameters, then performs a series of blitter operations using those settings.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `read_input_buttons_2`: Reads joystick and mouse button states from hardware registers, checks for specific button combinations, and updates a global variable based on the input states.
- `frag_move_tst_beq_rts`: Short code fragment that copies 2 to a fixed memory address, then tests d0, then branches if the last result was zero, then returns.
- `read_input_buttons`: Reads joystick and mouse button states from hardware registers, checks for specific button combinations, and updates a global variable based on the input detected.
- `frag_bsr_rts_2`: Short code fragment that calls another routine (4 times in a row), then returns.
- `frag_move_cmp_bne_rts`: Short code fragment that copies a location in the code to d2, then compares d2 with a location in the code, then branches if the last result was not zero, then returns.
- `frag_move_addq_bsr_movea_rts`: Short code fragment that copies d2 to a fixed memory address, then adds 1 to d2, then calls another routine, then adds 6 to a0, then copies an address to a1, then copies an address to a2, then calls another routine, then returns.
- `update_display_vars_2`: Updates two global variables with byte values from memory locations, and calls routines to render character sprites when the values change.
- `update_score_variables`: Compares byte values with stored longword variables, updates the longwords when values differ, and calls routines to handle score display updates.
- `update_global_vars`: Compares byte values with stored longword values, updates the longwords when values differ, and calls another routine twice for each update.
- `update_display_vars`: Updates several display-related global variables based on a counter value, with conditional adjustments and decrements.
- `setup_video_timing_reverse`: Initializes video timing registers for display synchronization in reverse direction, then adjusts them based on a counter until reaching a specific threshold.
- `update_coordinates`: Updates two global variables representing coordinates, increments them in a nested loop structure, and resets the outer coordinate when a limit is reached.
- `frag_bsr_subq_bne_rts`: Short code fragment that calls another routine (2 times in a row), then subtracts 1 from d0, then branches if the last result was not zero, then returns.
- `frag_lea_bra`: Short code fragment that loads the address of a location in the code into a1, then loads the address of a location in the code into a0, then jumps to another point in the code.
- `frag_lea_move_dbra_rts`: Short code fragment that loads the address of a location in the code into a0, then loads the address of a location in the code into a1, then copies 985 to d0, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then returns.
- `update_animation_frame`: Increments animation counters and copies data from a table to a structure, cycling through different frame indices and updating pointers accordingly.
- `frag_lea_move_moveq_bsr_cmpi_bne_rts`: Short code fragment that loads the address of a location in the code into a1, then copies the memory at a1 (advancing a1) to a fixed memory address (2 times in a row), then loads 2 into d3, then calls another routine, then compares the memory at a1 with 21845, then branches if the last result was not zero, then returns.
- `frag_cmpi_beq_move_rts`: Short code fragment that compares a fixed memory address with 10, then branches if the last result was zero, then copies 10 to a fixed memory address, then returns.
- `frag_clr_rts`: Short code fragment that clears a fixed memory address, then returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (subq.w, ori.b) and then falls through into the code that follows, with no return of its own.

### Mover (1992)(H. Mueller).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move_movem_moveq_rts`: Short code fragment that copies the top of the stack to d0, then restores saved registers from the stack, then loads 1 into d7, then returns.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `frag_cmp_bne_rts_2`: Short code fragment that compares d0 with a location in the code, then branches if the last result was not zero, then returns.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
