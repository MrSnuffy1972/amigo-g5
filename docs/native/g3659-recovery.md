# Game 3659 (fishy fishy - enhanced (1996)(infinite gain)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03659). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 54 of 54 named (0 author symbols), 54 of 54 described (17 by rule, 37 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1356 of 1356 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2603 of 2603 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 408343 |
| code | 13350 |
| data | 42841 |
| padding | 874 |
| unused | 435712 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Fishy Fishy - Enhanced (1996)(Infinite Gain).adf:FishyFishy_ENH | 2 | 2549 | 53 |
| Fishy Fishy - Enhanced (1996)(Infinite Gain).adf:bootblock | 1 | 54 | 1 |

## Cases handled

- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Fishy Fishy - Enhanced (1996)(Infinite Gain).adf:FishyFishy_ENH

- `initialize_graphics_system`: Initializes graphics hardware registers with values from global variables, configures display parameters, and sets up interrupt and DMA control registers before enabling interrupts.
- `initialize_graphics_library`: Opens a graphics library, reads data from it into global variables, closes the library, and configures hardware registers with values from those global variables.
- `wait_vposr_change`: Waits for vertical retrace position register to reach specific values and decrements a counter until it reaches zero.
- `update_display_colors`: Updates color pattern data for display by reading from a table and writing to hardware registers, then waits for vertical retrace position before proceeding.
- `frag_adda_suba_movea_jsr_tst_bne_bra`: Short code fragment that adds 320 to a1, then subtracts 23 from a1, then copies an address to a0, then calls another routine, then tests a fixed memory address, then branches if the last result was not zero, then copies an address to a0, then subtracts 3 from a1, then calls another routine, then jumps to another point in the code.
- `handle_input_events`: Processes keyboard input from a serial register, updates flags based on key presses, modifies game state variables, and calls other routines to handle different input scenarios.
- `setup_render_parameters`: Initializes various data fields with predefined values, performs calculations using another routine, and configures hardware registers including color and DMA settings for rendering.
- `fill_memory_loop`: Performs a loop that decrements a large counter and then copies bytes from one memory location to two others using a table lookup.
- `initialize_graphics_data`: Initializes various graphics-related registers and data structures with specific values, including clearing arrays and setting up configuration parameters.
- `fill_pattern_rows`: Fills rows of a pattern table with either zero values or data from a global table depending on a comparison result, using a loop counter.
- `fill_pattern_rows_2`: Fills rows of another pattern table with either zero values or data from a global table depending on a comparison result, using a loop counter.
- `set_bits_in_word_array`: Sets or clears specific bits in a word array based on comparison results and writes the modified values to memory locations.
- `set_bits_in_word_array_2`: Sets or clears specific bits in another word array based on comparison results and writes the modified values to memory locations.
- `load_and_process_patterns`: Loads pattern data addresses and offsets from global variables, calls routines to process two sets of pattern data, and returns with saved register values.
- `frag_clr_move_lea_jsr_adda_rts`: Short code fragment that clears d1, then copies a fixed memory address to d1, then loads the address of a fixed memory address into a0, then calls another routine, then loads the address of a fixed memory address into a0, then adds a fixed memory address to a1, then calls another routine, then returns.
- `frag_clr_move_lea_jsr_adda_rts_2`: Short code fragment that clears d1, then copies a fixed memory address to d1, then loads the address of a fixed memory address into a0, then calls another routine, then loads the address of a fixed memory address into a0, then adds a fixed memory address to a1, then calls another routine, then returns.
- `copy_pattern_data`: Copies pattern data from a table to a destination using byte offsets and handles multiple rows with a loop counter.
- `copy_pattern_data_2`: Copies pattern data from another table to a destination using byte offsets and handles multiple rows with a loop counter.
- `frag_divu_bsr`: Short code fragment that divides d1 by 1000 (unsigned), then calls another routine, then divides d1 by 100 (unsigned), then calls another routine, then divides d1 by 10 (unsigned), then calls another routine.
- `frag_addi_move_clr_swap_rts`: Short code fragment that adds 48 to d1, then copies d1 to the memory at a0 (advancing a0), then clears d1, then swaps the halves of d1, then returns.
- `frag_swap_cmp_bpl_addi_bra`: Short code fragment that swaps the halves of d1, then compares d1 with 10, then branches if the last result was not negative, then adds 48 to d1, then jumps to another point in the code.
- `frag_addi_move_swap_rts`: Short code fragment that adds 55 to d1, then copies d1 to the memory at a0 (advancing a0), then copies 0 to d1, then swaps the halves of d1, then returns.
- `configure_audio_dma`: Checks a flag value, configures audio hardware registers with values from memory, sets up DMA control registers, and updates interrupt handling for audio playback.
- `handle_interrupt_and_dma`: Checks interrupt status, decrements a counter, and if it reaches zero, enables DMA and interrupts while setting up a memory address.
- `process_object_data`: Reads object properties from a data structure and updates position, velocity, and state variables based on bit flags.
- `read_joystick_input_2`: Reads joystick input from hardware registers, updates internal state variables based on input bits, and calls other routines to handle different button press combinations.
- `handle_movement_input`: Configures color registers, reads joystick input, tests specific bits in the input value, and calls other routines to adjust movement parameters based on input conditions.
- `process_render_queue`: Initializes a counter, checks data validity in a structure, performs calculations with memory addresses, and calls another routine to process rendering data while incrementing the counter.
- `update_fish_animation`: Updates fish animation states based on global counters and conditions, modifying sprite data pointers and handling frame timing.
- `process_fish_data`: Calculates position coordinates based on input parameters and stores the result in a global structure, then increments a frame counter and updates state variables before returning.
- `process_fish_collision`: Calculates distances between fish entities and updates collision flags, counters, and position data based on comparison results.
- `update_game_state`: Updates game state variables based on bit flags and performs pattern table operations, including clearing and setting bits in word arrays, filling pattern rows, and configuring audio DMA settings.
- `setup_blitter`: Configures multiple blitter registers with specific values and waits for a hardware status bit before setting a size value.
- `blit_pattern_loop`: Processes a pattern loop by updating register values, performing arithmetic operations on data pointers, and looping until a counter reaches zero.
- `blit_pattern_render`: Sets up blitter parameters for rendering a pattern, updates multiple data pointers in a loop, and decrements a counter until it reaches zero.
- `read_joystick_input`: Reads joystick and mouse input states from hardware registers, combines them with CIA port bits, and returns a combined input value.
- `update_counter_a`: Checks a global counter and updates it based on comparison results with predefined values, decrementing it when conditions are met.
- `update_counter_b`: Toggles a global counter state and performs arithmetic operations on another counter based on comparison results with a threshold value.
- `update_counter_c`: Checks a global flag and updates a counter based on comparison results with predefined values, decrementing it when conditions are met.
- `frag_move_add_cmp_bcs_rts`: Short code fragment that copies a fixed memory address to d1, then adds a fixed memory address to d1, then compares d1 with 300, then branches if the carry flag is set, then returns, then copies a fixed memory address to d1, then adds a fixed memory address to d1, then copies d1 to a fixed memory address, then returns.
- `toggle_flag_and_subtract`: Toggles a global flag variable between zero and negative one, then subtracts a value from another global variable if the latter is greater than 128.
- `toggle_flag_and_update_value`: Toggles a global flag and updates a value in a table based on conditions, otherwise returns immediately.
- `update_counter_f`: Checks a global flag and updates a counter based on comparison results with predefined values, decrementing it when conditions are met.
- `frag_move_add_cmp_bcs_rts_2`: Short code fragment that copies a fixed memory address to d1, then adds a fixed memory address to d1, then compares d1 with 300, then branches if the carry flag is set, then returns, then copies a fixed memory address to d1, then adds a fixed memory address to d1, then copies d1 to a fixed memory address, then returns.
- `frag_move_addi_jsr_bra`: Short code fragment that copies d0 to the stack, then adds 1 to a fixed memory address, then calls another routine, then copies 5 to d0, then calls another routine, then copies the top of the stack to d0, then jumps to another point in the code.
- `frag_move_subi_jsr_bra`: Short code fragment that copies d0 to the stack, then subtracts 1 from a fixed memory address, then calls another routine, then copies 5 to d0, then calls another routine, then copies the top of the stack to d0, then jumps to another point in the code.
- `frag_move_cmpi_beq_addi_jsr_bra`: Short code fragment that copies d0 to the stack, then compares a fixed memory address with 40, then branches if the last result was zero, then adds 2 to a fixed memory address, then calls another routine, then copies 5 to d0, then calls another routine, then copies the top of the stack to d0, then jumps to another point in the code.
- `frag_move_cmpi_beq_subi_jsr_bra`: Short code fragment that copies d0 to the stack, then compares a fixed memory address with 0, then branches if the last result was zero, then subtracts 2 from a fixed memory address, then calls another routine, then copies 5 to d0, then calls another routine, then copies the top of the stack to d0, then jumps to another point in the code.
- `calculate_coordinates`: Performs multiplication and addition operations on word-sized values from memory locations, storing the results as words in different memory locations while swapping the byte order of intermediate results.
- `frag_move_subi_bne_rts`: Short code fragment that copies d1 to the stack, then subtracts 1 from d1, then branches if the last result was not zero, then copies the top of the stack to d1, then returns.
- `update_sprite_data`: Initializes sprite attributes, tests bits in a register to set flags, updates byte values based on bit tests, and conditionally stores word values to memory locations based on comparison results.
- `frag_move_btst_bne_clr_rts`: Short code fragment that copies a fixed memory address to d0, then tests bit 1 of d0, then branches if the last result was not zero, then clears a fixed memory address (4 times in a row), then returns.
- `frag_move_btst_bne_clr_rts_2`: Short code fragment that copies a fixed memory address to d0, then tests bit 1 of d0, then branches if the last result was not zero, then clears a fixed memory address (4 times in a row), then returns.

### Fishy Fishy - Enhanced (1996)(Infinite Gain).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
