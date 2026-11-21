# Game 4479 (multi zocker (1993)(h.mueller)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04479). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 52 of 52 named (0 author symbols), 52 of 52 described (16 by rule, 32 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1842 of 1842 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3384 of 3384 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 348574 |
| code | 15682 |
| data | 46518 |
| padding | 874 |
| unused | 489472 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Multi Zocker (1993)(H.Mueller).adf:Multi-Zocker | 2 | 3374 | 51 |
| Multi Zocker (1993)(H.Mueller).adf:bootblock | 1 | 10 | 1 |

## Cases handled

- 6 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 immediate+address
- multi zocker (1993)(h.mueller)/Multi Zocker (1993)(H.Mueller).adf: blocks 212-212 (bytes 108544-109056) outside the filesystem hold data, not code (opcode density 0.004, entropy 0.81)
- multi zocker (1993)(h.mueller)/Multi Zocker (1993)(H.Mueller).adf: blocks 215-215 (bytes 110080-110592) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.73)
- multi zocker (1993)(h.mueller)/Multi Zocker (1993)(H.Mueller).adf: blocks 217-217 (bytes 111104-111616) outside the filesystem hold data, not code (opcode density 0.004, entropy 0.54)
- multi zocker (1993)(h.mueller)/Multi Zocker (1993)(H.Mueller).adf: blocks 883-884 (bytes 452096-453120) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.64)
- multi zocker (1993)(h.mueller)/Multi Zocker (1993)(H.Mueller).adf: blocks 1154-1154 (bytes 590848-591360) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.73)
- multi zocker (1993)(h.mueller)/Multi Zocker (1993)(H.Mueller).adf: blocks 1157-1157 (bytes 592384-592896) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.77)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Multi Zocker (1993)(H.Mueller).adf:Multi-Zocker

- `initialize_system`: Initializes system resources by opening libraries, setting up hardware registers, and clearing global memory areas before entering the main game loop.
- `frag_btst_bne_rts`: Short code fragment that tests bit 14 of the memory at an offset from a6, then branches if the last result was not zero, then returns.
- `frag_movem_moveq_rts`: Short code fragment that saves registers on the stack, then restores saved registers from the stack, then loads 255 into d0, then returns.
- `handle_input_event`: Reads a byte from a global variable, processes it through another routine three times, then writes it back to a table.
- `frag_move_cmp_beq_rts`: Short code fragment that copies the memory at an offset from a6 to d1, then compares d1 with the memory at an offset from a6, then branches if the last result was zero, then returns.
- `frag_move_bsr_moveq_dbra_rts`: Short code fragment that copies a location in the code to a fixed memory address, then calls another routine, then loads 12 into d6, then calls another routine, then loops back while counting down d6, then returns.
- `frag_move_bsr_rts`: Short code fragment that copies 1 to a fixed memory address, then calls another routine, then copies an address to a fixed memory address, then copies 1280 to a fixed memory address, then copies 254 to a fixed memory address, then copies 64 to a fixed memory address, then copies 32769 to a fixed memory address, then calls another routine, then copies 2 to a fixed memory address, then returns.
- `initialize_digit_buffer`: Sets up a digit buffer with a specific byte value and performs arithmetic operations on data from tables, then compares the results with a fixed value.
- `setup_blitter_registers`: Initializes blitter control registers with fixed values and calls another routine to configure blitter parameters for graphics operations.
- `setup_blitter_parameters`: Configures blitter registers for graphics operations by calculating memory addresses and setting up control words based on input parameters.
- `initialize_dma_channels`: Configures specific DMA channel settings by writing fixed values to several hardware registers, then calls another routine three times with different parameters before returning.
- `configure_blitter_dma_2`: Sets up blitter DMA parameters including source and destination pointers, control registers, and a counter in a global structure, then returns the input values unchanged.
- `set_display_parameters`: Reads a status byte at offset 2 of a structure in a6, waits for a specific bit to clear, then writes multiple predefined values to offsets 40, 44, 50, 54, 58, and 64 of the same structure.
- `initialize_graphics`: Initializes graphics-related data in a structure pointed to by a6, setting multiple register values and flags.
- `configure_blitter_for_rendering`: Configures blitter parameters for graphic rendering by calculating memory addresses from input data and setting up control words for hardware operations.
- `configure_rendering_parameters`: Sets up rendering parameters by copying data from tables, calling configuration routines, and updating control words in a structure.
- `frag_movem_move_bsr_lea_moveq_rts`: Short code fragment that saves registers on the stack, then copies a location in the code to d2, then calls another routine, then loads the address of the memory at an offset from a0 into a2, then loads the address of a fixed memory address into a0, then loads 19 into d0, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_movem_move_bsr_lea_rts`: Short code fragment that saves registers on the stack, then copies a location in the code to d2, then calls another routine, then loads the address of the memory at an offset from a0 into a2, then loads the address of a fixed memory address into a0, then copies 132 to d0, then calls another routine, then restores saved registers from the stack, then returns.
- `update_rendering_state`: Compares a global value with another global value and updates the second if they are equal, then calls initialization routines.
- `setup_graphics_rendering`: Initializes graphics rendering parameters by setting up blitter registers with calculated addresses and control words based on input values.
- `initialize_rendering_parameters`: Initializes multiple rendering parameter structures by clearing specific offsets and copying data from tables, then configures display and DMA settings based on calculated values.
- `configure_blitter_dma`: Sets up blitter DMA parameters by writing control words and memory addresses to hardware registers, then configures four sequential DMA operations with incremented pointers and fixed offsets.
- `manage_game_state_counters`: Updates game state counters based on conditional checks, modifies global variables, and calls configuration routines for rendering parameters.
- `handle_secondary_counters`: Manages secondary counter values by checking bounds, updating state variables, and calling configuration routines for graphics operations.
- `manage_audio_dma_control`: Controls audio playback by managing DMA channels, updating sound parameters, and configuring hardware registers for audio output based on current game state conditions.
- `configure_audio_channels`: Sets up audio parameters for two channels by writing period, length, and volume values to DMA and audio registers, then enables DMA for audio output.
- `update_audio_parameters`: Configures audio channel settings by writing values to DMA control and audio registers, including period, length, and volume parameters for multiple channels.
- `update_counter_and_check`: Decrements a counter value and performs conditional checks against fixed memory addresses, returning early if conditions are met.
- `validate_and_update_value`: Compares a value with a fixed threshold and updates it if necessary, then performs validation checks before returning.
- `frag_bsr_subq_bpl_move_cmp_beq_blt_bgt`: Short code fragment that calls another routine, then subtracts 1 from a fixed memory address, then branches if the last result was not negative, then copies 7 to a fixed memory address, then copies a location in the code to d0, then compares d0 with a location in the code, then branches if the last result was zero, then branches if the comparison was less, then branches if the comparison was greater, then returns.
- `manage_audio_and_dma_control`: Controls audio playback by managing DMA channels, updating sound parameters, and configuring hardware registers for audio output based on conditional logic.
- `frag_movem_move_add_cmp_bne_rts`: Short code fragment that saves registers on the stack, then copies a fixed memory address to d0, then adds 20 to d0, then copies a fixed memory address to d1, then compares d1 with d0, then branches if the last result was not zero, then restores saved registers from the stack, then returns.
- `frag_movem_move_add_cmp_bne_rts_2`: Short code fragment that saves registers on the stack, then copies a fixed memory address to d0, then adds 10 to d0, then copies a fixed memory address to d1, then compares d1 with d0, then branches if the last result was not zero, then restores saved registers from the stack, then returns.
- `frag_bsr_rts`: Short code fragment that calls another routine, then returns.
- `frag_bsr_dbra_rts`: Short code fragment that calls another routine, then loops back while counting down d0, then returns.
- `frag_moveq_move_mulu_addq_eor_and_rts`: Short code fragment that loads 0 into d0, then copies a location in the code to d0, then multiplies d0 by 221 (unsigned), then adds 1 to d0, then copies a fixed memory address to d1, then exclusive-ors d1 into d0, then copies d0 to a fixed memory address, then masks d0 with 31, then returns.
- `generate_pseudo_random_number`: Generates a pseudo-random number using an XOR operation with previous values and a multiplication constant, then stores the result in memory.
- `frag_move_bsr_rts_2`: Short code fragment that copies 1 to a fixed memory address, then calls another routine, then copies an address to a fixed memory address, then copies 617 to a fixed memory address, then copies 214 to a fixed memory address, then copies 40 to a fixed memory address, then copies 32769 to a fixed memory address, then calls another routine, then copies 2 to a fixed memory address, then returns.
- `frag_movem_move_bra`: Short code fragment that saves registers on the stack, then copies a location in the code to d0, then jumps to another point in the code.
- `setup_render_parameters`: Initializes rendering parameters by writing fixed values to specific offsets in a structure pointed to by a6, then calculates and stores memory addresses for blitter operations based on a table index.
- `frag_cmpi_bne_bsr_rts`: Short code fragment that compares a fixed memory address with 254, then branches if the last result was not zero, then compares a fixed memory address with 255, then branches if the last result was not zero, then calls another routine, then returns.
- `frag_bsr_bra`: Short code fragment that calls another routine, then jumps to another point in the code.
- `update_game_state`: Updates various game state variables including counters, input handling, and audio parameters based on joystick and mouse input.
- `process_rendering_sequence`: Calls another routine to update bitmask data, decrements a counter, and resets it when it reaches zero while updating global pointers.
- `update_bitmask_data`: Processes byte data from a table to update bitmask values in memory, performing bit rotation and logical operations on 32-bit words.
- `configure_audio_and_dma`: Configures audio hardware registers and clears DMA channel settings while updating sound parameter tables based on input data.
- `frag_tst_beq_clr_move_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then clears a fixed memory address (5 times in a row), then copies 15 to a fixed memory address, then returns.
- `update_audio_parameters_2`: Updates audio hardware registers based on global state and input data, handling volume and pitch adjustments for multiple sound channels.
- `update_object_position`: Updates object position coordinates based on input parameters, clamps values within defined ranges, and writes results to global structure offsets.
- `update_game_state_2`: Updates game state variables based on input data, configures audio and DMA settings, and modifies global flags and timers according to control values read from a data structure.
- `decrement_timers_and_clear_flags`: Decrements three 32-bit timer values and clears corresponding flags in global variables when timers reach zero.

### Multi Zocker (1993)(H.Mueller).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
