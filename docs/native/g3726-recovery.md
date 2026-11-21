# Game 3726 (frogger (19xx)(-)[m ale]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03726). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (907600 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 67 of 67 named (0 author symbols), 67 of 67 described (11 by rule, 54 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 868 of 868 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2386 of 2386 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1387 |
| code | 5506 |
| data | 11973 |
| padding | 414 |
| unused | 888320 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Frogger (19xx)(-)[m ALE].adf:frogger | 6 | 232 | 6 |
| Frogger (19xx)(-)[m ALE].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1068 | 30 |
| Frogger (19xx)(-)[m ALE].adf:frogger#unpacked | 2 | 1068 | 30 |

## Cases handled

- runtime segment an address: 639 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- frogger (19xx)(-)[m ale]/Frogger (19xx)(-)[m ALE].adf:frogger: crunched executable decrunched by XFDMaster (Imploder (xfd), 6784 -> 13432 bytes), 4 of 7 windows confirmed in reference RAM: recovered as a hunk executable
- frogger (19xx)(-)[m ale]/Frogger (19xx)(-)[m ALE].adf:frogger: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Frogger (19xx)(-)[m ALE].adf:frogger

- `free_memory_blocks_loop`: Calls one routine, then calls the exec library's memory-free routine in a loop of three passes.
- `frag_move_lea_movea_adda_dbra_addq_rts`: Short code fragment that copies d6 to d7, then loads the address of a location in the code into a1, then copies the memory at a1 to a1, then adds a1 to a1 (2 times in a row), then loops back while counting down d7, then adds 4 to a1, then returns.
- `initialize_and_process`: Initializes a counter and processes data through iterative subroutine calls, updating values in memory based on conditional checks and loop counters.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_byte_pattern`: Copies bytes from a source buffer to a destination, decrementing a counter and handling carry flags during bit operations.
- `frag_clr_ori_move_movem_bra`: Short code fragment that clears d0, then merges 182 into d0, then copies d0 to the memory just below a4 (moving a4 down), then copies d1 to the memory just below a4 (moving a4 down), then copies d2 to the memory just below a4 (moving a4 down), then restores saved registers from the stack, then jumps to another point in the code.

### Frogger (19xx)(-)[m ALE].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_graphics_system`: Opens a library, configures hardware registers including DMA control and interrupt enable, sets up blitter parameters, and performs several system initialization calls before returning.
- `frag_lea_move_addi_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then copies 14 to d0, then copies 386 to d1, then copies d1 to the memory at a1 (advancing a1), then adds 2 to d1, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then returns.
- `frag_move_cmp_blt_rts_addi`: Short code fragment that copies a fixed memory address to d1, then compares d1 with a fixed memory address, then branches if the comparison was less, then returns, then copies d1 to a fixed memory address, then adds 1 to a fixed memory address, then returns.
- `validate_and_update_coordinates_2`: Validates coordinate values in a data structure, updates them based on conditions, and calls a setup routine for graphics operations when validation passes, otherwise returns immediately.
- `validate_and_update_coordinates`: Validates coordinate values against boundaries, calculates differences, and updates a global variable with a clamped result when conditions are met.
- `initialize_game_state`: Checks if a global variable equals -1, and if so, increments another global variable by 750, then calls several routines to configure blitter operations and sets multiple global variables to specific values.
- `check_and_update_flags`: Checks if a global variable's value falls within specific ranges, updates flags based on bitwise operations, and calls other routines when conditions are met.
- `setup_blitter_dma_5`: Configures blitter DMA parameters for graphics operations by setting up various control registers and calling a secondary setup routine based on bit flags.
- `initialize_dma_channels`: Initializes multiple DMA channels for graphics rendering by configuring blitter parameters, enabling interrupts, and setting up hardware registers.
- `read_joystick_input`: Reads joystick input from a hardware register, processes the value through a series of conditional checks, and stores the result in a global variable.
- `update_frame_counter`: Increments a global counter and conditionally resets it based on a value read from another global variable, then performs blitter register setup for graphics rendering.
- `configure_blitter_registers`: Sets up blitter control registers with specific values and performs multiple DMA transfers to render graphics using calculated address offsets.
- `setup_blitter_dma`: Configures blitter registers for DMA operations by setting up various control and address values based on calculated offsets from global variables.
- `setup_blitter_dma_2`: Initializes blitter parameters for DMA transfers including size, addresses, and control registers using offset calculations derived from input coordinates.
- `configure_blitter`: Sets up blitter hardware registers with fixed values for control, address, and size parameters to prepare for graphics operations.
- `process_object_data`: Reads coordinate values from a data structure, updates them by adding an offset, clamps the result to a range between 0 and 384, and sets the updated value back to the structure.
- `frag_lea_move_cmp_beq_bsr_bra_rts`: Short code fragment that loads the address of a fixed memory address into a3, then copies the memory at a3 (advancing a3) to d0, then compares d0 with 65535, then branches if the last result was zero, then calls another routine, then loads the address of the memory at an offset from a3 into a3, then jumps to another point in the code, then returns.
- `validate_and_update`: Validates coordinate values in a data structure, updates them based on conditions, and calls a setup routine for graphics operations when values are within range.
- `iterate_coordinate_list`: Iterates through a list of coordinates, processes each entry with a setup routine, and continues until reaching an end marker value.
- `setup_blitter_dma_3`: Configures blitter DMA parameters including control registers, addresses, and transfer sizes using input coordinates and global data structures.
- `setup_blitter_dma_4`: Initializes blitter registers for graphics rendering by setting up address pointers, control values, and transfer parameters based on input data.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `configure_blitter_operations`: Sets up blitter parameters based on a counter value, calling another routine for each condition and updating memory addresses accordingly.
- `blit_with_dma_wait`: Performs multiple blitter operations with DMA synchronization by waiting for previous transfers to complete before starting new ones.
- `blit_with_dma_wait_2`: Executes a sequence of blitter operations with DMA synchronization, updating destination addresses and transfer parameters in a loop.
- `blit_with_dma_wait_3`: Processes blitter commands with DMA waiting, calculating source and destination addresses while iterating through data structures.
- `parse_time_components`: Decomposes a word value into decimal components by repeatedly subtracting powers of ten and stores the results in four global variables.
- `frag_move_rts`: Short code fragment that copies 0 to a fixed memory address, then copies 65535 to a fixed memory address, then copies 0 to a fixed memory address, then copies 32832 to a fixed memory address, then copies 42 to a fixed memory address, then copies 65534 to a fixed memory address (2 times in a row), then copies 42 to a fixed memory address, then returns.
- `blit_with_dma_wait_4`: Handles multiple blitter operations in a loop, updating addresses and waiting for DMA completion between each transfer iteration.
- `initialize_graphics_data`: Writes a series of long values to memory locations starting at 28640 and updates word values at offsets 23004, 23000, 23012, 23008, 23020, 23016, 23028, and 23024 based on the loaded long values.

### Frogger (19xx)(-)[m ALE].adf:frogger#unpacked

- `setup_display_controller`: Initializes display buffer pointers, configures blitter registers with specific values, sets up DMA channels and interrupt handling, and waits for vertical blanking before returning.
- `copy_pattern_data`: Copies word values from one memory location to another in a loop, incrementing the source pointer and destination pointer while updating intermediate values in the destination area.
- `frag_move_cmp_blt_rts_addi_2`: Short code fragment that copies a fixed memory address to d1, then compares d1 with a fixed memory address, then branches if the comparison was less, then returns, then copies d1 to a fixed memory address, then adds 1 to a fixed memory address, then returns.
- `validate_coordinate_range`: Checks if a coordinate value falls within a specific range and performs validation on associated data, updating global variables based on the comparison results.
- `process_coordinate_data_loop`: Reads coordinate values from a data structure, updates them by adding an offset, clamps the result to a range between 0 and 384, and sets the updated value in a global variable, then continues processing entries in a loop.
- `initialize_game_state_loop`: Checks if a global variable equals -1, and if so, increments another global variable by 750, then calls several routines to configure blitter operations and sets multiple global variables to specific values, continuing in a loop.
- `update_frog_position`: Checks if frog coordinates are within range of specific values, updates flags in a global variable, and modifies frog position data when conditions are met.
- `setup_blitter_modifiers`: Configures blitter modifier values based on bit flags in a global variable, calling another routine to set up blitter operations for each enabled bit.
- `execute_blitter_sequence`: Executes a series of blitter operations with different modifier values, calling another routine to configure the blitter for each operation.
- `process_joystick_input`: Reads joystick data from hardware register JOY1DAT, processes the input to determine a movement direction, and stores this direction in a global variable while checking against a previous value.
- `update_frogger_state`: Updates a global counter and processes conditional logic based on memory comparisons, then configures multiple blitter registers with specific values and offsets.
- `update_blitter_parameters`: Updates blitter parameters based on comparison results between memory values and thresholds, then calls another routine to set up blitter operations with calculated offsets.
- `setup_blitter_operations`: Configures multiple blitter registers including BLTCMOD, BLTBMOD, BLTAMOD, BLTDMOD, BLTCON1, BLTALWM, BLTCON0, and sets up memory addresses for blitter operations based on calculated offsets.
- `configure_blitter_for_rendering`: Initializes blitter registers with specific values for rendering operations, calculates memory offsets, and configures multiple blitter parameters including BLTAPTH, BLTDPTH, and BLTSIZE for different drawing operations.
- `set_blitter_render_parameters`: Sets up blitter registers with fixed values for rendering operations, configures memory addresses for source and destination data, and sets the size of the operation to be performed.
- `process_blitter_data`: Processes a sequence of data entries, updating values and calling another routine for valid entries within specific range limits.
- `iterate_blitter_entries`: Iterates through a list of blitter configuration entries until reaching a termination marker, calling another routine for each entry.
- `process_blitter_data_loop`: Processes a sequence of data entries in a loop, updating values and calling another routine for valid entries within specific range limits.
- `iterate_blitter_entries_loop`: Iterates through a list of blitter configuration entries until reaching a termination marker, calling another routine for each entry in a loop.
- `configure_blitter_with_shifted_data`: Calculates memory offsets based on input parameters, configures blitter registers with shifted values, and sets up multiple blitter operations for rendering with specific source and destination addresses.
- `perform_multiple_blitter_operations`: Sets up blitter parameters for rendering operations, calculates memory offsets from a stack location, and performs four sequential blitter operations with identical configurations but different memory addresses.
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `process_blitter_conditions`: Evaluates a counter value against multiple thresholds and calls another routine based on comparison results, updating memory addresses for blitter operations.
- `execute_blitter_sequence_with_wait`: Configures blitter registers for a sequence of operations, writes memory addresses to BLTDPTH, waits for DMA completion, and repeats this process three times with incremented addresses.
- `perform_blitter_operations_with_dma_wait`: Initializes blitter registers for rendering, sets up source and destination memory addresses, performs a sequence of operations with DMA wait checks, and repeats the process three times with incremented addresses.
- `configure_blitter_dma`: Sets up blitter DMA parameters for graphics operations by configuring address pointers, control registers, and transfer sizes using calculated offsets.
- `parse_timer_value`: Decomposes a timer value into individual decimal digits and stores them in consecutive memory locations, then configures blitter parameters and updates a global state pointer based on input from a hardware register.
- `frag_move_rts_2`: Short code fragment that copies 0 to a fixed memory address, then copies 65535 to a fixed memory address, then copies 0 to a fixed memory address, then copies 32832 to a fixed memory address, then copies 42 to a fixed memory address, then copies 65534 to a fixed memory address (2 times in a row), then copies 42 to a fixed memory address, then returns.
- `render_pattern_dma`: Configures and executes multiple blitter DMA transfers to render graphical patterns by copying data from source to destination addresses with specific offsets.
- `initialize_display_buffers`: Initializes display buffer pointers for graphics rendering by setting up four consecutive memory locations with calculated address values.
