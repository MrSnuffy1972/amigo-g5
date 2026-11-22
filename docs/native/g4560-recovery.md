# Game 4560 (og! (1993)(d. parsons)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04560). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901696 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 219 of 219 named (0 author symbols), 219 of 219 described (97 by rule, 119 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2397 of 2397 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4951 of 4951 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 215932 |
| code | 23514 |
| data | 145395 |
| padding | 759 |
| unused | 516096 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Og! (1993)(D. Parsons).adf:og! | 2 | 4910 | 215 |
| Og! (1993)(D. Parsons).adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 114 | 3 |

## Cases handled

- 5 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 jump target

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Og! (1993)(D. Parsons).adf:og!

- `initialize_game`: Initializes graphics system, configures hardware registers including display and DMA control, sets up global variables for game state, and performs raster line synchronization waits.
- `initialize_graphics_system`: Allocates memory blocks for graphics structures, populates pattern tables with control codes and data pointers, configures coprocessor pointers and DMA channels, and sets up display timing registers.
- `initialize_audio_system`: Sets up audio hardware registers including interrupt enable and request control, clears a disk status byte, and writes specific values to CIA interrupt control registers before returning.
- `frag_addi_move_andi_bne_rte`: Short code fragment that adds 1 to a fixed memory address, then copies d0 to a fixed memory address, then copies a fixed memory address to d0, then masks d0 with 32, then branches if the last result was not zero, then copies a fixed memory address to d0, then returns from the exception.
- `frag_move_movem_cmpi_beq_jsr_rte`: Short code fragment that copies d0 to a fixed memory address, then copies a fixed memory address to d0, then saves registers on the stack, then compares a fixed memory address with 0, then branches if the last result was zero, then calls another routine, then restores saved registers from the stack, then returns from the exception.
- `frag_addi_cmpi_beq_jsr_jmp`: Short code fragment that adds 1 to a fixed memory address, then compares a fixed memory address with 0, then branches if the last result was zero, then calls another routine, then jumps to another point in the code.
- `initialize_graphics`: Initializes graphics library components by setting up coprocessor pointers and enabling DMA channels, then frees multiple memory blocks using the exec library.
- `clear_blitter_registers`: Clears all blitter control and data registers to zero, effectively resetting the graphics hardware's bit-blitting capabilities.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `frag_movea_cmpa_bne_movem_rts`: Short code fragment that copies 1636 to a0, then copies a fixed memory address to a1, then compares a1 with a fixed memory address, then branches if the last result was not zero, then copies 1764 to a0, then copies several registers to or from memory, then copies a fixed memory address to a1, then copies several registers to or from memory, then returns.
- `copy_three_longs_to_globals`: Copies three 32-bit values from one block of global variables into a second block of three 32-bit global variables, then returns.
- `populate_pattern_table`: Writes a series of word values including control codes and data from memory locations into two separate tables, organizing data for pattern generation.
- `update_blitter_registers`: Processes byte values through bit rotation and manipulation operations, updates multiple global variables with computed results, and performs blitter register configuration before returning.
- `process_coordinate_offset`: Processes coordinate values by comparing them against thresholds, performing arithmetic operations and table lookups to calculate offset adjustments.
- `setup_blitter_operations_3`: Configures multiple blitter registers with values derived from input parameters and a table, then performs four blitter operations with varying parameters.
- `setup_blitter_operations`: Configures multiple blitter registers with values derived from input parameters and a table, then performs four blitter operations with varying offsets and waits for each to complete before proceeding.
- `setup_keyboard_handler`: Configures the keyboard serial register and timer registers for input handling, then waits for a specific interrupt condition before returning.
- `frag_jsr_andi_asl_addi_move_jmp`: Short code fragment that calls another routine, then masks d0 with 31, then shifts d0 left by 1, then adds 190 to d0, then copies d0 to d3, then copies 64 to d2, then copies 4 to d0, then jumps to another point in the code.
- `frag_cmpi_beq_movem_eori_move_jsr_rts`: Short code fragment that compares a fixed memory address with 0, then branches if the last result was zero, then saves registers on the stack, then exclusive-ors 1 into a fixed memory address, then copies a fixed memory address to d4, then calls another routine, then restores saved registers from the stack, then returns.
- `configure_audio_dma`: Sets up audio DMA parameters including channel length, period, and volume, then enables DMA for audio output.
- `frag_cmpi_beq_bmi_subi_bne_move_rts`: Short code fragment that compares a fixed memory address with 0, then branches if the last result was zero, then branches if the last result was negative, then subtracts 1 from a fixed memory address, then branches if the last result was not zero, then copies 0 to a fixed memory address, then copies 4 to a fixed memory address, then copies 8 to a fixed memory address, then returns.
- `frag_move_jsr_jmp`: Short code fragment that copies 15 to a fixed memory address, then calls another routine, then copies a fixed memory address to a fixed memory address, then copies 1024 to a fixed memory address, then jumps to another point in the code.
- `wait_for_disk_status`: Clears two word-sized global variables, sets a byte to 8, inverts another byte, then repeatedly calls routines while incrementing a counter until a disk status bit becomes clear.
- `initialize_game_state_2`: Initializes multiple global variables to zero, sets up blitter configuration parameters, configures several hardware registers with fixed values, and performs a series of routine calls to prepare the game state for execution.
- `initialize_game_state`: Initializes multiple global variables to zero, sets up blitter configuration parameters, and configures several hardware registers with fixed values before entering a main loop.
- `update_counter_value`: Decrements a counter value, checks if it reaches zero, and updates another counter based on comparison results.
- `frag_move_addi_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then adds 2 to d0, then copies d0 to the memory at an offset from a6, then returns.
- `frag_jsr_andi_asl_addi_move_rts`: Short code fragment that calls another routine, then masks d0 with 31, then shifts d0 left by 2, then adds 60 to d0, then copies d0 to the memory at an offset from a6, then copies the memory at an offset from a6 to the memory at an offset from a6, then returns.
- `frag_lea_move_clr_adda_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a6, then copies 34 to d7, then clears the memory at an offset from a6, then adds 24 to a6, then loops back while counting down d7, then returns.
- `update_joystick_state`: Updates joystick input state by reading hardware registers and modifying counters based on button presses and conditional checks.
- `update_position_and_collision`: Updates position coordinates based on input values and performs collision detection by calling routines to validate tile data, then adjusts movement parameters accordingly.
- `update_counter_and_coordinates`: Decrements a global counter and calculates coordinate values based on table lookups and arithmetic operations, storing the results in a structure pointed to by a5.
- `check_entity_collision`: Tests if an entity at a specific offset matches certain criteria and updates related configuration values, otherwise returns immediately.
- `update_blitter_config`: Configures blitter-related parameters by calculating and storing word values derived from bitwise operations and memory updates, repeating a fixed number of times.
- `update_blitter_config_2`: Configures multiple blitter registers using values from global variables and performs blitter operations based on conditional logic.
- `decode_input_value`: Reads input values from memory locations, applies bitwise operations, and returns different results based on conditional branches.
- `configure_blitter_operations`: Sets up multiple blitter registers with values derived from input parameters and a table, then performs four blitter operations with varying parameters.
- `process_animation_frames`: Processes animation frame data by updating counters, checking flags, and performing blitter operations based on frame type.
- `frag_jsr_btst_beq_subi_bpl_rts`: Short code fragment that calls another routine, then tests bit 7 of a fixed memory address, then branches if the last result was zero, then subtracts 1 from d7, then branches if the last result was not negative, then returns.
- `setup_blitter_operations_2`: Configures blitter registers for graphics operations including source and destination pointers, size parameters, and waits for DMA completion.
- `initialize_blitter_dma`: Sets up initial blitter parameters for graphics rendering, configures DMA registers, and waits for completion before continuing.
- `configure_blitter_dma_2`: Configures blitter DMA registers by clearing and setting specific values, performs multiple DMA operations through a loop, and updates state variables based on iteration counts.
- `initialize_blitter_dma_2`: Initializes blitter DMA operations by setting up register values, calculating memory offsets, and configuring blitter control registers.
- `configure_blitter_dma`: Sets up blitter registers for multiple DMA operations with specific source and destination addresses, then waits for each operation to complete.
- `handle_input_events`: Processes input events by reading bytes from a buffer, comparing them against a threshold, and calling another routine when conditions are met while incrementing a counter.
- `process_input_event`: Handles input events by updating sprite coordinates and attributes based on event codes, with different behaviors for specific event values.
- `initialize_sprite_data`: Sets a global word variable to -1, writes the value 5 to another global byte variable, and inverts the bits of a third global byte variable before returning.
- `update_sprite_data`: Updates sprite coordinate and attribute values based on input parameters and stores the results in memory locations, with different behavior depending on a condition.
- `update_coordinates`: Updates coordinate values based on input parameters and stores the results in memory locations, with different behavior depending on a condition.
- `calculate_coordinates`: Computes coordinate values by scaling and offsetting input registers, then stores the results in memory locations.
- `frag_lea_jmp`: Short code fragment that loads the address of a fixed memory address into a5, then jumps to another point in the code.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (lea.l, adda.l) and then falls through into the code that follows, with no return of its own.
- `validate_and_increment`: Validates a bit condition in memory, increments a counter if valid, updates pointer addresses, and returns based on comparison results.
- `validate_and_advance`: Validates a bit condition in memory, increments a counter if valid, advances pointer addresses, and returns based on comparison results.
- `reverse_validate_and_decrement`: Validates a bit condition in memory, decrements a counter, updates pointer addresses, and returns based on comparison results.
- `setup_blitter_dma`: Configures blitter registers for multiple DMA operations with specific source and destination addresses, then waits for each operation to complete by polling the DMA status bit.
- `clear_memory_blocks`: Initializes multiple memory blocks by writing zero values to specific offsets within a data structure, using a loop that processes 174 blocks of data.
- `frag_movea_move_dbra_rts`: Short code fragment that copies a fixed memory address to a0, then copies 21119 to d7, then copies 0 to the memory at a0 (advancing a0), then loops back while counting down d7, then returns.
- `update_blitter_parameters`: Updates blitter configuration parameters based on computed values and triggers blitter operations when screen coordinates change, otherwise it returns.
- `frag_cmpi_bne_rts`: Short code fragment that compares a fixed memory address with 0, then branches if the last result was not zero, then returns.
- `lookup_tile_data`: Performs coordinate validation and table lookups to retrieve tile data from multiple tables based on input coordinates, returning the combined result or zero if no valid data is found.
- `validate_and_process_collision`: Validates input coordinates against boundary conditions, performs tile lookups similar to routine R3, and checks for collision data before returning appropriate status values.
- `handle_input_and_setup`: Checks input conditions using a global variable and CIAA_PRA register, clears a flag when a specific bit is set, and sets up data for a structure pointed to by a global variable.
- `execute_object_functions`: Iterates through a list of object entries, calling functions stored at specific offsets when certain conditions are met, processing up to 34 entries in sequence.
- `update_counter_condition`: Checks if a counter value is below a threshold, increments the counter, and compares it with another value before potentially branching based on the comparison result.
- `frag_neg_clr_move_andi_lsr_addi_rts`: Short code fragment that negates the memory at an offset from a6, then clears the memory at an offset from a6, then copies a fixed memory address to d0, then masks d0 with 2, then shifts d0 right by 1, then adds 160 to d0, then copies d0 to the memory at an offset from a6, then returns.
- `frag_move_add_cmpi_bhi_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then adds d0 to the memory at an offset from a6, then copies the memory at an offset from a6 to d0, then adds d0 to the memory at an offset from a6, then compares the memory at an offset from a6 with 384, then branches if the comparison was higher (unsigned), then returns.
- `frag_cmpi_bcs_rts`: Short code fragment that compares the memory at an offset from a6 with 64, then branches if the carry flag is set, then returns.
- `frag_cmpi_bcs_rts_2`: Short code fragment that compares the memory at an offset from a6 with 80, then branches if the carry flag is set, then returns.
- `frag_cmpi_bcs_move_add_rts`: Short code fragment that compares the memory at an offset from a6 with 64, then branches if the carry flag is set, then copies the memory at an offset from a6 to d0, then adds d0 to the memory at an offset from a6, then returns.
- `update_graphics_params`: Updates graphics parameters for display operations by manipulating register values and storing results to memory locations pointed to by a5, with conditional execution based on counter values.
- `frag_jsr_andi_bne_move_rts`: Short code fragment that calls another routine, then masks d0 with 63, then branches if the last result was not zero, then copies 60 to a fixed memory address, then copies 60 to the memory at an offset from a6, then returns.
- `update_movement_state`: Validates position coordinates against boundaries, updates counters and movement parameters, and modifies object state based on conditional checks and arithmetic operations.
- `validate_and_update_position`: Validates coordinate boundaries and updates position values, with conditional logic that modifies a counter and performs tile lookups.
- `validate_and_modify_position`: Checks if position coordinates are within valid ranges, compares values with stored data, and modifies memory values when specific conditions are met.
- `frag_clr_rts`: Short code fragment that clears the memory at an offset from a6, then returns.
- `frag_move_addi_rts_2`: Short code fragment that copies 1 to the memory at an offset from a6, then adds 1 to the memory at an offset from a6, then returns.
- `modify_and_validate_position`: Validates coordinate boundaries, performs comparisons between stored values, and updates memory locations based on conditional logic and arithmetic operations.
- `frag_clr_rts_2`: Short code fragment that clears the memory at an offset from a6, then returns.
- `frag_move_subi_cmp_bhi_addi_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then subtracts 2 from d0, then compares d0 with the memory at an offset from a6, then branches if the comparison was higher (unsigned), then copies 1 to the memory at an offset from a6, then adds 1 to the memory at an offset from a6, then returns.
- `update_position`: Updates a position value at offset 2 of the structure and checks if it matches a specific threshold, branching to a location if it does.
- `frag_move_beq_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then branches if the last result was zero, then copies 65535 to the memory at an offset from a6, then returns.
- `check_boundary_conditions`: Checks boundary conditions for coordinates and updates a counter, with branching based on comparison results and bit testing.
- `frag_subi_move_rts`: Short code fragment that subtracts 1 from the memory at an offset from a6, then copies 34831 to the memory at an offset from a6, then returns.
- `update_counter_and_flags`: Increments a counter in the structure, clears a bit in a status byte, and updates several memory locations with fixed values before calling other routines.
- `increment_and_compare`: Increments two counters in the structure, compares one of them to a specific value, and clears the second counter if they match.
- `validate_coordinates`: Checks if x and y coordinates are within specified bounds, and updates state variables based on coordinate validation results.
- `frag_move_addi_rts_3`: Short code fragment that copies 1 to the memory at an offset from a6, then adds 1 to the memory at an offset from a6, then returns.
- `frag_move_andi_bne_addi_rts`: Short code fragment that copies a fixed memory address to d0, then masks d0 with 3, then branches if the last result was not zero, then copies 1 to the memory at an offset from a6, then adds 1 to the memory at an offset from a6, then returns.
- `frag_clr_rts_3`: Short code fragment that clears the memory at an offset from a6, then returns.
- `validate_and_initialize`: Validates several conditions based on memory values and fixed addresses, then initializes multiple fields in the structure with specific values when all conditions are met.
- `handle_position_adjustment`: Adjusts position coordinates based on input values, performs distance calculations, and updates state variables with conditional branching.
- `frag_subi_move_rts_2`: Short code fragment that subtracts 8 from the memory at an offset from a6, then copies 69 to the memory at an offset from a6, then copies 33794 to the memory at an offset from a6, then copies 100 to the memory at an offset from a6, then returns.
- `frag_cmpi_beq_addi_rts`: Short code fragment that compares the memory at an offset from a6 with 9, then branches if the last result was zero, then adds 1 to the memory at an offset from a6, then returns.
- `frag_cmpi_bcs_move_andi_addi_rts`: Short code fragment that compares the memory at an offset from a6 with 32, then branches if the carry flag is set, then copies a fixed memory address to d0, then masks d0 with 7, then adds 103 to d0, then copies d0 to the memory at an offset from a6, then returns.
- `update_counter_and_flags_2`: Updates a counter in a global structure and manipulates flags based on conditional checks, then returns a modified value from the structure.
- `frag_subi_rts`: Short code fragment that subtracts 1 from the memory at an offset from a6, then returns.
- `frag_cmpi_bne`: Short code fragment that compares the memory at an offset from a6 with 357, then branches if the last result was not zero.
- `update_blitter_config_3`: Configures blitter-related parameters by calculating and storing word values derived from bitwise operations and memory updates, repeating a set number of times.
- `frag_cmpi_bcs_move_andi_beq_jsr_bne_rts`: Short code fragment that compares the memory at an offset from a6 with 80, then branches if the carry flag is set, then copies a fixed memory address to d0, then masks d0 with 1, then branches if the last result was zero, then calls another routine, then masks d0 with 31, then branches if the last result was not zero, then copies 137 to the memory at an offset from a6, then returns.
- `frag_move_cmp_bcs_sub_asr_beq_bmi_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then compares d0 with 80, then branches if the carry flag is set, then subtracts a fixed memory address from d0, then shifts d0 right by 3, then branches if the last result was zero, then branches if the last result was negative, then copies 134 to the memory at an offset from a6, then returns.
- `frag_move_rts`: Short code fragment that copies 135 to the memory at an offset from a6, then returns.
- `frag_move_rts_2`: Short code fragment that copies 136 to the memory at an offset from a6, then returns.
- `frag_cmpi_bhi_beq_addi_move_add_rts`: Short code fragment that compares the memory at an offset from a6 with 384, then branches if the comparison was higher (unsigned), then compares the memory at an offset from a6 with 13, then branches if the last result was zero, then adds 1 to the memory at an offset from a6, then copies the memory at an offset from a6 to d0, then adds d0 to the memory at an offset from a6, then returns.
- `update_and_accumulate`: Updates a counter in the structure, uses it to index into a table, reads a value from that table, and accumulates it with another value in the structure.
- `update_and_accumulate_2`: Similar to the previous routine but with different offset calculations and accumulation logic, updating a counter and using it to index into another table.
- `frag_subi_beq_rts`: Short code fragment that subtracts 1 from the memory at an offset from a6 (2 times in a row), then branches if the last result was zero, then returns.
- `frag_addi_subi_bmi_rts`: Short code fragment that adds 1 to the memory at an offset from a6, then subtracts 1 from the memory at an offset from a6, then branches if the last result was negative, then returns.
- `update_timer_and_flags`: Updates a timer value and manages status flags based on coordinate comparisons and bit operations, clearing specific bits when the timer reaches zero.
- `update_position_4`: Updates position coordinates based on boundary checks and performs decrement operations on counters while managing state variables through conditional branches.
- `frag_cmpi_bcs_move_add_andi_rts`: Short code fragment that compares the memory at an offset from a6 with 32, then branches if the carry flag is set, then copies the memory at an offset from a6 to d0, then adds d0 to the memory at an offset from a6, then copies a fixed memory address to d0, then masks d0 with 1, then adds the memory at an offset from a6 to d0, then copies d0 to the memory at an offset from a6, then returns.
- `adjust_coordinates`: Adjusts coordinate values by adding components from other memory locations, performs boundary comparisons, and updates state variables when conditions are met.
- `frag_move_addi_jsr_beq_bra`: Short code fragment that copies the memory at an offset from a6 to d0, then copies the memory at an offset from a6 to d1, then adds 8 to d0, then adds 7 to d1, then calls another routine, then branches if the last result was zero, then copies 65535 to a fixed memory address, then jumps to another point in the code.
- `update_cycle_counter`: Updates a cycle counter by adding two values together, applies a bitwise AND operation, and updates other counters based on the result.
- `frag_move_rts_3`: Short code fragment that copies 2 to the memory at an offset from a6, then copies 34818 to the memory at an offset from a6, then copies 37 to the memory at an offset from a6, then returns.
- `frag_move_bra`: Short code fragment that copies 65535 to a fixed memory address, then jumps to another point in the code.
- `modify_movement`: Modifies movement parameters by incrementing a counter, updating position values, and performing collision detection with boundary checks before adjusting coordinates.
- `frag_cmpi_beq_addi_rts_2`: Short code fragment that compares the memory at an offset from a6 with 9, then branches if the last result was zero, then adds 1 to the memory at an offset from a6, then returns.
- `frag_move_rts_4`: Short code fragment that copies 0 to the memory at an offset from a6, then returns.
- `control_velocity`: Controls velocity by checking boundary conditions, updating position values through addition operations, and managing state variables based on conditional results.
- `frag_cmpi_beq_addi_rts_3`: Short code fragment that compares the memory at an offset from a6 with 9, then branches if the last result was zero, then adds 1 to the memory at an offset from a6, then returns.
- `frag_subi_bne_move_rts`: Short code fragment that subtracts 1 from the memory at an offset from a6, then branches if the last result was not zero, then copies 65530 to the memory at an offset from a6, then returns.
- `advance_and_accumulate`: Advances a position value by adding another value to it, increments a counter, uses that counter to index into a table, and accumulates values from the table.
- `conditional_increment`: Updates a position value by adding another value to it, performs bitwise operations on a global variable, and conditionally increments or decrements a counter based on the result.
- `frag_subi_rts_2`: Short code fragment that subtracts 1 from the memory at an offset from a6, then returns.
- `distance_comparison`: Calculates distances between two points, uses those distances to index into lookup tables, accumulates values from the tables, and compares the result with another value before returning.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `frag_move_jsr`: Short code fragment that copies an address to d2, then calls another routine.
- `delay_loop`: Executes a series of timed delays using a fixed sequence of values loaded into register d2, calling another routine with each value before proceeding to the next delay.
- `frag_moveq_addi_sub_bpl_add_jmp`: Short code fragment that loads 15 into d3, then adds 1 to d3, then subtracts d2 from d1, then branches if the last result was not negative, then adds d2 to d1, then jumps to another point in the code.
- `frag_move_jsr_2`: Short code fragment that copies 10000 to d2, then calls another routine, then copies 1000 to d2, then calls another routine.
- `frag_moveq_jsr`: Short code fragment that loads 100 into d2, then calls another routine.
- `frag_moveq_jsr_2`: Short code fragment that loads 10 into d2, then calls another routine, then loads 1 into d2.
- `frag_moveq_addi_sub_bpl_add`: Short code fragment that loads 15 into d3, then adds 1 to d3, then subtracts d2 from d1, then branches if the last result was not negative, then adds d2 to d1.
- `copy_pattern_data`: Copies 7 bytes from a pattern table to two destination buffers, advancing pointers by 42 bytes after each copy operation, and updates a global variable pointer.
- `transfer_byte_sequence`: Transfers 8 bytes from a source table to a destination buffer with specific offset calculations, updating the destination pointer by 42 bytes after each transfer.
- `process_joystick_input_2`: Updates joystick input state by checking hardware registers and modifying global variables based on input conditions.
- `update_joystick_state_2`: Updates joystick input state by checking hardware registers and modifying global variables based on input conditions.
- `process_joystick_input`: Processes joystick input by reading hardware registers, checking bit patterns, and updating global variables based on input conditions.
- `frag_move_beq_jsr_add_subi_jmp`: Short code fragment that copies a fixed memory address to d3, then branches if the last result was zero, then calls another routine, then copies 20 to d2, then adds d3 to d2, then copies a fixed memory address to d3, then branches if the last result was zero, then copies 24 to d2, then subtracts 8 from d1, then jumps to another point in the code.
- `setup_blitter_fragment_2`: Configures blitter registers with fragment-specific parameters, performs boundary checks on coordinates, and initializes blitter operations.
- `process_dma_setup`: Sets up DMA operations by clearing registers, configuring blitter parameters, and performing bit tests on hardware status flags before executing blitter commands.
- `frag_move_clr_jsr_beq_rts`: Short code fragment that copies a fixed memory address to d0, then copies a fixed memory address to d1, then clears a fixed memory address, then copies 496 to d7, then calls another routine, then copies a fixed memory address to d0, then branches if the last result was zero, then clears a fixed memory address, then copies 7 to a fixed memory address, then returns.
- `update_game_state_3`: Processes input key values to update various game state variables and perform actions based on specific key codes, including incrementing counters, setting flags, and calling display update routines.
- `update_display_registers`: Updates blitter-related display registers by processing a word value through bit manipulation, then performs a register update operation with predefined constants before returning.
- `update_game_state`: Processes game state updates based on input conditions, modifies global variables and display data, then calls other routines to handle rendering and timing.
- `check_and_update_status`: Checks multiple conditions related to a global variable's status bits and updates another global variable based on those checks, then performs a loop with decrementing counter and conditional exits.
- `clear_blitter_registers_2`: Clears the blitter data register and recursively calls another routine if a global variable's high bit is set.
- `validate_range`: Checks if a value in register d1 is within a specific range and returns early if it's below a threshold, otherwise continues execution without modifying any data.
- `setup_blitter_fragment`: Configures blitter registers with fragment-specific parameters, performs boundary checks on coordinates, and initializes blitter operations for rendering graphics fragments.
- `setup_blitter_parameters`: Configures multiple blitter registers with calculated values based on input parameters, waits for a hardware status bit to clear, and sets a global flag to indicate completion.
- `update_timer_values`: Decrements a timer value and updates two memory locations with computed values from a lookup table, or sets fixed values if the timer reaches zero.
- `initialize_data_structure`: Initializes an array of 32 words with incrementing values starting from 384, then copies data from two lookup tables to a memory region with specific formatting and padding.
- `frag_move_rts_5`: Short code fragment that copies a0 to a fixed memory address, then copies 142 to the memory at a0 (advancing a0), then copies 11393 to the memory at a0 (advancing a0), then copies 144 to the memory at a0 (advancing a0), then copies 62657 to the memory at a0 (advancing a0), then returns.
- `frag_clr_move_not_rts`: Short code fragment that clears a fixed memory address, then copies 255 to a fixed memory address, then inverts a fixed memory address, then returns.
- `update_game_state_2`: Updates position and status data for game objects by performing arithmetic operations on coordinates, setting flags, and managing counters.
- `update_position_3`: Updates object coordinates based on velocity values and manages a countdown timer.
- `process_table_entry`: Loads data from a table based on an index, updates several registers with values from the table, and calls another routine if a condition is met, otherwise performs coordinate adjustments.
- `frag_move_addi_rts_4`: Short code fragment that copies 116 to the memory at an offset from a6, then adds 6 to the memory at an offset from a6, then returns.
- `frag_move_bra_2`: Short code fragment that copies 6 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_move_jsr_dbra_rts`: Short code fragment that copies 15 to d6, then calls another routine (3 times in a row), then loops back while counting down d6, then returns.
- `frag_clr_jsr_addi_cmp_bne_rts`: Short code fragment that clears d6, then calls another routine (3 times in a row), then adds 1 to d6, then compares d6 with 15, then branches if the last result was not zero, then returns.
- `frag_move_jsr_dbra_rts_2`: Short code fragment that copies 15 to d6, then calls another routine (3 times in a row), then loops back while counting down d6, then returns.
- `frag_cmpi_beq_move_jsr_addi_cmp_bne_rts`: Short code fragment that compares a fixed memory address with 15, then branches if the last result was zero, then copies a fixed memory address to d6, then calls another routine (3 times in a row), then adds 1 to d6, then compares d6 with 15, then branches if the last result was not zero, then returns.
- `frag_lea_movea_adda_move_jmp`: Short code fragment that loads the address of a fixed memory address into a0, then copies a fixed memory address to a1, then adds 2 to a1, then copies 4 to d4, then copies 30 to d7, then jumps to another point in the code.
- `frag_lea_movea_adda_move_jmp_2`: Short code fragment that loads the address of a fixed memory address into a0, then copies a fixed memory address to a1, then adds 2 to a1, then copies 4 to d4, then copies 30 to d7, then jumps to another point in the code.
- `frag_lea_movea_adda_move_jmp_3`: Short code fragment that loads the address of a fixed memory address into a0, then copies a fixed memory address to a1, then adds 6 to a1, then copies 51 to d7, then copies 8 to d4, then jumps to another point in the code.
- `decode_and_transform_words`: Processes a sequence of words by applying bitwise operations to extract and modify specific nibble values, storing the results back to memory with pointer arithmetic.
- `read_and_test_bytes`: Reads 20 longwords from memory in reverse order, then performs a bit test operation on register d0 using a value loaded from the stack, without modifying any other data.
- `frag_clr_move_cmp_beq_subi_jsr_jmp`: Short code fragment that clears d3, then copies the memory at a0 (advancing a0) to d3, then compares d3 with 13, then branches if the last result was zero, then subtracts 32 from d3, then calls another routine, then jumps to another point in the code.
- `copy_bytes_loop`: Copies bytes from a source location to two destination locations in a loop, advancing pointers and decrementing a counter until it reaches zero.
- `frag_clr_move_cmp_beq_subi_jsr_jmp_2`: Short code fragment that clears d3, then copies the memory at a0 (advancing a0) to d3, then compares d3 with 13, then branches if the last result was zero, then subtracts 32 from d3, then calls another routine, then jumps to another point in the code.
- `copy_and_modify_bytes`: Copies bytes from a source location to two destination locations while applying bitwise operations based on a control flag, then advances pointers and decrements a counter.
- `frag_clr_move_cmp_beq_subi_jsr_jmp_3`: Short code fragment that clears d3, then copies the memory at a0 (advancing a0) to d3, then compares d3 with 13, then branches if the last result was zero, then subtracts 32 from d3, then calls another routine, then jumps to another point in the code.
- `bitwise_merge_loop`: Processes bytes in a loop, performing bitwise AND or OR operations with memory locations based on bits in a control register, advancing pointers and decrementing a counter.
- `frag_and_btst_beq_or_bra`: Short code fragment that masks the memory at a3 with d1, then tests bit 1 of d4, then branches if the last result was zero, then merges d0 into the memory at an offset from a3, then jumps to another point in the code.
- `frag_and_btst_beq_or_bra_2`: Short code fragment that masks the memory at an offset from a3 with d1, then tests bit 2 of d4, then branches if the last result was zero, then merges d0 into the memory at an offset from a3, then jumps to another point in the code.
- `frag_and_btst_beq_or_bra_3`: Short code fragment that masks the memory at an offset from a3 with d1, then tests bit 3 of d4, then branches if the last result was zero, then merges d0 into the memory at an offset from a3, then jumps to another point in the code.
- `frag_and_btst_beq_or_bra_4`: Short code fragment that masks the memory at an offset from a3 with d1, then tests bit 4 of d4, then branches if the last result was zero, then merges d0 into the memory at a2, then jumps to another point in the code.
- `frag_and_adda_dbra_rts`: Short code fragment that masks the memory at a2 with d1, then adds 42 to a3, then adds 42 to a2, then loops back while counting down d2, then adds 1 to a4, then returns.
- `frag_movea_move_jmp`: Short code fragment that copies 8142 to a4, then copies a fixed memory address to d1, then jumps to another point in the code.
- `frag_movea_move_jmp_2`: Short code fragment that copies 8136 to a4, then copies a fixed memory address to d1, then jumps to another point in the code.
- `update_display_data`: Updates display-related data by copying values from a source table to multiple destinations based on indexed offsets, then performs a loop of byte transfers before returning.
- `frag_move_andi_bne_subi_jsr_jmp_rts`: Short code fragment that copies a fixed memory address to d0, then masks d0 with 7, then branches if the last result was not zero, then subtracts 1 from a fixed memory address, then calls another routine, then jumps to another point in the code, then returns.
- `frag_cmpi_bmi_add_rts_move`: Short code fragment that compares a fixed memory address with 0, then branches if the last result was negative, then adds d7 to a fixed memory address, then returns, then copies d7 to a fixed memory address, then returns.
- `conditional_counter_update`: Updates a counter based on conditions, calls other routines to process data, and handles different execution paths depending on the counter's value.
- `update_status_bytes`: Updates two status bytes in global memory locations, clears a word-sized counter, and performs several subroutine calls to handle game state transitions.
- `frag_jsr_movea_lea_jmp`: Short code fragment that calls another routine, then copies 8106 to a4, then loads the address of a fixed memory address into a0, then jumps to another point in the code.
- `render_display_elements`: Processes display-related data by copying values from source tables to destination locations using indexed offsets, then performs a loop operation that advances pointers and copies bytes between memory regions with fixed stride patterns.
- `copy_and_advance_loop`: Copies bytes between memory locations in nested loops, advancing pointers by fixed offsets and decrementing counters until both reach zero.
- `process_input_and_render`: Processes input from a hardware register and calls multiple display-related routines based on comparison results with a fixed value.
- `frag_cmpi_bne_beq_not_jmp`: Short code fragment that compares a fixed memory address with 33, then branches if the last result was not zero, then compares a fixed memory address with 33, then branches if the last result was zero, then inverts a fixed memory address, then jumps to another point in the code.
- `frag_cmpi_bne_beq_not_move_jmp_rts`: Short code fragment that compares a fixed memory address with 55, then branches if the last result was not zero, then compares a fixed memory address with 55, then branches if the last result was zero, then inverts a fixed memory address, then copies a fixed memory address to a fixed memory address, then jumps to another point in the code, then returns.
- `conditional_string_processing`: Processes a string conditionally based on memory values, calling other routines to handle different branches of execution.
- `frag_jsr_movea_jmp`: Short code fragment that calls another routine, then copies 8106 to a4, then copies an address to a0, then jumps to another point in the code.
- `iterate_pattern_blocks`: Repeatedly calls a rendering routine with different memory addresses and offsets, processing a fixed sequence of pattern data blocks in a loop.
- `frag_move_cmp_beq_rts`: Short code fragment that copies a fixed memory address to d0, then compares d0 with a fixed memory address, then branches if the last result was zero, then returns.
- `update_and_lookup_values`: Updates counter values and performs lookups in tables based on the counter's value, then stores results back to memory locations.
- `frag_jmp`: Short code fragment that jumps to another point in the code.
- `copy_data_blocks`: Copies structured data blocks from a source table to multiple destinations in memory, using loop counters and calculated offsets.
- `update_position_2`: Updates position coordinates based on data from a table and increments a counter, resetting it when reaching a specific value.
- `frag_move_rts_6`: Short code fragment that copies 65530 to the memory at an offset from a6, then copies 32776 to the memory at an offset from a6, then returns.
- `adjust_velocity`: Adjusts velocity components by reading from a lookup table and modifying the main velocity based on comparison with a threshold value, potentially increasing or decreasing it.
- `frag_cmpi_beq_addi_rts_4`: Short code fragment that compares the memory at an offset from a6 with 3, then branches if the last result was zero, then adds 1 to the memory at an offset from a6, then returns.
- `initialize_object_data`: Initializes object properties by reading input values, performing bit manipulation operations, and setting various fields in a data structure.
- `configure_rendering_parameters`: Sets up rendering parameters by reading from memory locations, performing arithmetic operations, and calling another routine to configure blitter registers.
- `frag_move_jsr_andi_add_addi_cmpi_bne_cl`: Short code fragment that copies 90 to the memory at an offset from a6, then calls another routine, then masks d0 with 31, then adds d0 to the memory at an offset from a6, then adds 1 to the memory at an offset from a6, then compares the memory at an offset from a6 with 7, then branches if the last result was not zero, then clears the memory at an offset from a6, then returns.
- `configure_render_parameters`: Sets up rendering parameters by configuring specific memory locations with predefined values, performing bitwise operations on input data, and conditionally updating fields based on comparison results.
- `update_render_state`: Writes byte value 8 to a global variable, inverts another global variable, sets a word-sized global variable to 1, then calls a routine 18 times in a loop, followed by two more routine calls, and finally sets the word-sized global variable to 0.
- `initialize_audio`: Initializes audio system by setting up data structures, clearing memory regions, configuring hardware registers for sound output, and updating status flags related to audio channels and input devices.
- `frag_clr_move_rts`: Short code fragment that clears a fixed memory address (2 times in a row), then copies 15 to a fixed memory address, then returns.
- `update_audio_registers`: Updates audio hardware registers by reading from memory tables and copying values to specific register locations.
- `process_object_data`: Reads object properties from a structure, processes movement parameters based on a lookup table, and updates position and velocity values.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `process_input_command`: Reads a command byte from a structure, masks its lower four bits, and processes different operations based on the command value.

### Og! (1993)(D. Parsons).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_raster_line_2`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `copy_registers`: Copies register contents between memory locations, performing a series of moves from one address to another while maintaining the original values in the source location.
- `frag_move_rts_7`: Short code fragment that copies a fixed memory address to a fixed memory address (3 times in a row), then returns.
