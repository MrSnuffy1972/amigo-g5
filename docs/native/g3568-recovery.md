# Game 3568 (excalibur (1988)(j. meggers)[m oks]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03568). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 16 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 170 of 170 named (0 author symbols), 170 of 170 described (59 by rule, 111 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1295 of 1295 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4212 of 4212 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 261148 |
| code | 22168 |
| data | 42272 |
| padding | 3116 |
| unused | 572416 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Excalibur (1988)(J. Meggers)[m OKS].adf:EXCALIBUR | 2 | 4202 | 169 |
| Excalibur (1988)(J. Meggers)[m OKS].adf:bootblock | 1 | 10 | 1 |

## Cases handled

- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 2 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 jump target

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Excalibur (1988)(J. Meggers)[m OKS].adf:EXCALIBUR

- `initialize_game_resources`: Allocates memory for game resources, initializes pointers to various data structures within the allocated block, and sets up hardware registers for graphics rendering.
- `execute_rendering_loop`: Executes a series of rendering-related operations by calling setup routines with specific parameters, storing results in a global variable, and repeating this process multiple times.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d0, then returns.
- `initialize_sequence`: Writes a series of bytes to a memory buffer starting at a specific address, incrementing a counter and looping until reaching a termination condition.
- `frag_movea_move_cmpa_bne_rts`: Short code fragment that copies 23104 to a0, then copies an address to the memory at a0 (advancing a0), then compares a0 with 23360, then branches if the last result was not zero, then returns.
- `frag_jsr_cmpi_beq_bne_bra_rts`: Short code fragment that calls another routine, then compares a fixed memory address with 0, then branches if the last result was zero, then compares a fixed memory address with 1, then branches if the last result was not zero, then calls another routine, then jumps to another point in the code, then returns.
- `frag_movea_jsr_addi_cmpi_bne_move_clr_r`: Short code fragment that copies a fixed memory address to a6, then calls another routine, then adds 240 to a fixed memory address, then compares a fixed memory address with 25365, then branches if the last result was not zero, then copies 23925 to a fixed memory address, then clears a fixed memory address, then returns.
- `frag_addi_cmpi_bne_clr_rts`: Short code fragment that adds 1 to a fixed memory address, then compares a fixed memory address with 38, then branches if the last result was not zero, then clears a fixed memory address, then returns.
- `frag_movem_move_neg_ror_jmp`: Short code fragment that saves registers on the stack, then copies a fixed memory address to a fixed memory address, then negates a fixed memory address, then rotates  right by a fixed memory address, then restores saved registers from the stack, then jumps to another point in the code.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `frag_cmpi_bne_move_rts`: Short code fragment that compares a fixed memory address with 61, then branches if the last result was not zero, then copies 5 to a fixed memory address, then copies 4095 to a fixed memory address, then returns.
- `update_game_timers`: Updates multiple game state counters and handles conditional logic based on their values, including decrementing byte counters and setting specific flags.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `frag_andi_bne_move_clr_rts`: Short code fragment that masks a fixed memory address with 64, then branches if the last result was not zero, then copies 1 to a fixed memory address, then masks a fixed memory address with 128, then branches if the last result was not zero, then clears a fixed memory address, then returns.
- `update_animation_state`: Updates animation parameters based on conditional checks and modifies values in a table through iterative operations with decrementing counters.
- `update_game_state`: Updates multiple global variables based on the value of a control byte, setting different values depending on the condition branches taken.
- `update_frame_counters`: Updates frame-related counters based on comparison results, then performs conditional operations including incrementing and decrementing values in memory.
- `copy_blitter_parameters`: Copies word values between two memory locations using a loop with a fixed counter, then returns immediately after completing the copy operation.
- `copy_sprite_data`: Calculates an address offset using multiplication and copies sprite data from one location to another with specific byte offsets.
- `frag_andi_bne_cmpi_beq_move_rts`: Short code fragment that masks a fixed memory address with 128, then branches if the last result was not zero, then compares a fixed memory address with 1, then branches if the last result was zero, then copies 1 to a fixed memory address, then returns.
- `initialize_video_memory`: Initializes video memory by clearing specific memory locations with zero values, sets up various game state variables, and calls other routines to process the data.
- `configure_audio_output`: Adjusts audio period and volume registers based on comparison results, setting different values depending on input conditions.
- `frag_move_rts`: Short code fragment that copies a fixed memory address to a fixed memory address, then copies 6287 to a fixed memory address, then copies 0 to a fixed memory address, then copies 32769 to a fixed memory address, then returns.
- `frag_move_rts_2`: Short code fragment that copies 4 to a fixed memory address, then copies a fixed memory address to a fixed memory address, then copies 5000 to a fixed memory address, then copies 250 to a fixed memory address, then copies 64 to a fixed memory address, then copies 26 to a fixed memory address, then copies 32772 to a fixed memory address, then returns.
- `frag_move_rts_3`: Short code fragment that copies 8 to a fixed memory address, then copies a fixed memory address to a fixed memory address, then copies 5000 to a fixed memory address, then copies 250 to a fixed memory address, then copies 64 to a fixed memory address, then copies 26 to a fixed memory address, then copies 32776 to a fixed memory address, then returns.
- `frag_move_rts_4`: Short code fragment that copies 4 to a fixed memory address, then returns.
- `frag_move_rts_5`: Short code fragment that copies 8 to a fixed memory address, then returns.
- `frag_move_addi_cmp_bne_rts`: Short code fragment that copies a fixed memory address to d0, then copies d0 to d1, then adds 1 to d1, then copies a fixed memory address to d0, then compares d1 with d0, then branches if the last result was not zero, then returns.
- `frag_cmpi_beq_move_rts`: Short code fragment that compares a fixed memory address with 1, then branches if the last result was zero, then copies a fixed memory address to a fixed memory address, then copies 1107 to a fixed memory address, then copies 363 to a fixed memory address, then copies 64 to a fixed memory address, then copies 32770 to a fixed memory address, then returns.
- `frag_move_rts_6`: Short code fragment that copies 55 to a fixed memory address, then copies 2 to a fixed memory address, then copies a fixed memory address to a fixed memory address, then copies 6640 to a fixed memory address, then copies 363 to a fixed memory address, then copies 64 to a fixed memory address, then copies 32770 to a fixed memory address, then returns.
- `frag_move_rts_7`: Short code fragment that copies 40 to a fixed memory address, then copies 2 to a fixed memory address, then copies a fixed memory address to a fixed memory address, then copies 40835 to a fixed memory address, then copies 358 to a fixed memory address, then copies 64 to a fixed memory address, then copies 32770 to a fixed memory address, then returns.
- `frag_move_rts_8`: Short code fragment that copies 2 to a fixed memory address, then returns.
- `frag_move_rts_9`: Short code fragment that copies 2 to a fixed memory address, then returns.
- `frag_move_rts_10`: Short code fragment that copies 2 to a fixed memory address, then returns.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `frag_cmpi_bne_jmp_jsr_btst_beq_rts`: Short code fragment that compares a fixed memory address with 1, then branches if the last result was not zero, then jumps to another point in the code, then calls another routine (2 times in a row), then tests bit 0 of a fixed memory address, then branches if the last result was zero, then calls another routine, then returns.
- `frag_cmpi_beq_addi_bne_jsr_rts`: Short code fragment that compares a fixed memory address with 79, then branches if the last result was zero, then adds 1 to a fixed memory address, then compares a fixed memory address with 79, then branches if the last result was not zero, then calls another routine, then returns.
- `frag_jsr_cmpi_bne_rts`: Short code fragment that calls another routine (5 times in a row), then compares a fixed memory address with 1, then branches if the last result was not zero, then calls another routine, then returns.
- `map_input_to_character`: Maps input character values to specific byte values and stores them at an offset in a memory structure, then calls another routine to process the data.
- `update_display_buffer`: Updates display buffer contents by copying word values between memory locations and managing a pointer with boundary checks.
- `update_audio_timer`: Decrements a global counter and updates audio registers with a calculated value when the counter reaches zero, otherwise returns immediately.
- `frag_cmpi_beq_subi_rts_addi`: Short code fragment that compares a fixed memory address with 1, then branches if the last result was zero, then compares a fixed memory address with 0, then branches if the last result was zero, then subtracts 5 from a fixed memory address, then returns, then compares a fixed memory address with 320, then branches if the last result was zero, then adds 5 to a fixed memory address, then returns.
- `manage_game_timer`: Manages game timer state by checking conditions and decrementing a counter, updating flags when specific thresholds are reached.
- `calculate_and_store_coordinates_2`: Processes a word value by masking its lower nibble, combining it with a left-shifted version, and storing the result, then performs arithmetic operations on another value and stores that result.
- `frag_movea_move_clr_dbra_rts`: Short code fragment that copies a fixed memory address to a0, then copies 18799 to d0, then clears the memory at a0 (advancing a0), then loops back while counting down d0, then returns.
- `frag_cmpi_beq_jsr_subi_rts`: Short code fragment that compares a fixed memory address with 10, then branches if the last result was zero, then calls another routine, then subtracts 1 from a fixed memory address, then returns.
- `frag_cmpi_bne_move_jsr_rts`: Short code fragment that compares a fixed memory address with 45, then branches if the last result was not zero, then copies 1 to a fixed memory address, then calls another routine, then returns.
- `frag_move_jsr_jmp`: Short code fragment that copies 12417 to a fixed memory address, then copies 8385 to a fixed memory address, then copies 39 to a fixed memory address, then calls another routine, then copies 23925 to a fixed memory address, then jumps to another point in the code.
- `frag_jsr_move_rts`: Short code fragment that calls another routine (2 times in a row), then copies a fixed memory address to a fixed memory address, then calls another routine, then copies 320 to a fixed memory address, then calls another routine (2 times in a row), then returns.
- `frag_movea_move_dbra_rts`: Short code fragment that copies 22580 to a0, then copies 22976 to a1, then copies 15 to d0, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then returns.
- `transfer_word_data`: Transfers 32-bit data values to a memory table by extracting high and low words and performing arithmetic operations on the source value.
- `transfer_word_data_2`: Transfers 32-bit data values to a memory table by extracting high and low words and performing arithmetic operations on the source value with different offsets.
- `copy_memory_block`: Copies memory blocks between two locations using nested loops, transferring word values with specific offset calculations and counter decrements.
- `process_counter_digits`: Compares two word values and updates a counter when the condition is met, then processes a value by converting it to digits and storing them in memory.
- `setup_blitter_operation_1`: Calculates blitter parameters by dividing a word value by 10, performing bit manipulation to derive an address offset, then calls a routine to configure and execute a blitter operation.
- `configure_blitter_dma_2`: Sets up blitter parameters for a simple DMA operation and performs three repeated blitter operations with fixed size and increment.
- `frag_cmpi_bne_beq_clr_rts`: Short code fragment that compares a fixed memory address with 1, then branches if the last result was not zero, then compares a fixed memory address with 1, then branches if the last result was zero, then compares a fixed memory address with 1, then branches if the last result was zero, then clears a fixed memory address (2 times in a row), then returns.
- `initialize_pointers`: Loads a base address from a global variable, performs multiple arithmetic operations with constants, and stores the results to various memory locations.
- `initialize_game_state`: Initializes various game state variables with specific values, clears multiple flags, and updates counters before calling other routines to process the data.
- `setup_blitter_operation_3`: Initializes a counter and sets a global byte value when a condition is met, then processes data by reading bytes from memory and calling another routine for each iteration.
- `process_input_loop`: Processes input based on a character value, executes nested loops with specific counters, and updates global variables in conditional branches.
- `execute_nested_loops`: Executes two separate loops with identical counter values, calling another routine for each iteration in both loops.
- `write_pattern_data`: Writes a series of predefined 16-bit values to sequential memory addresses starting from a fixed location.
- `frag_andi_bne_move_bra_clr_rts`: Short code fragment that masks a fixed memory address with 128, then branches if the last result was not zero, then copies 1 to a fixed memory address, then jumps to another point in the code, then clears a fixed memory address, then returns.
- `update_game_state_2`: Checks multiple global variables for specific values and performs conditional updates to various memory locations including clearing bytes and updating structure fields.
- `copy_structure_data`: Copies data from a source structure to multiple destination structures, clearing specific bytes in the process.
- `frag_move_rts_11`: Short code fragment that copies the memory at an offset from a5 to a fixed memory address (6 times in a row), then returns.
- `frag_movea_jsr_cmpi_bne_move_rts`: Short code fragment that copies a fixed memory address to a6, then copies the memory at an offset from a6 to a1, then calls the routine whose address is in a1, then calls another routine, then compares a fixed memory address with 1, then branches if the last result was not zero, then copies 65135 to a fixed memory address, then calls another routine, then returns.
- `update_structure_counters`: Updates counters and performs conditional operations on structure data based on comparison results with global variables.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `validate_structure_state`: Validates the state of structures by checking specific byte values and performs a decrement operation if conditions are met.
- `adjust_position_and_update`: Adjusts position values based on input data, updates global flags, and modifies multiple memory locations with calculated values.
- `populate_structure_data`: Populates structure data with values from a source, sets fixed width and height values, and clears specific flag bytes.
- `copy_and_update_graphics`: Copies graphics data from a buffer to multiple memory locations, updating pointers and handling boundary conditions.
- `update_graphics_coordinates`: Updates coordinate values in multiple structures by dividing existing values by two and adding fixed offsets.
- `frag_move_rts_12`: Short code fragment that copies 105 to a fixed memory address, then copies 65 to a fixed memory address, then copies 105 to a fixed memory address, then copies 135 to a fixed memory address, then copies 20 to a fixed memory address, then copies 40 to a fixed memory address, then copies 60 to a fixed memory address, then copies 40 to a fixed memory address, then returns.
- `write_graphics_pattern`: Writes predefined 16-bit graphics pattern values to specific memory locations representing display data.
- `write_display_data`: Writes a series of 16-bit values to specific global variables, likely initializing or updating display-related registers or memory locations used by the graphics subsystem.
- `update_sprite_data`: Updates sprite position and attribute data by reading from a table, incrementing a pointer, and writing modified values to multiple global variables.
- `adjust_graphics_coordinates`: Updates graphics coordinate values by dividing them by three, adding thirty-four, and then increments several memory locations by eight.
- `calculate_and_set_coordinates`: Calculates new coordinate values using division and addition operations, computes differences from a fixed value, and updates multiple memory locations with both calculated and fixed values.
- `frag_jsr_rts`: Short code fragment that calls another routine, then returns.
- `write_sprite_data`: Writes multiple 16-bit values to specific offsets in a global data structure, likely initializing sprite attributes or display parameters.
- `update_sprite_data_2`: Updates sprite position data by reading from a table and writing to multiple memory locations, with boundary checks and table pointer cycling.
- `update_positions`: Processes coordinate data from a memory structure, divides each coordinate by two, adjusts the values with fixed offsets, and stores the results into multiple global variables representing positions and dimensions.
- `scale_coordinate_values`: Scales coordinate values by dividing them by two, adds specific offsets, and updates multiple memory locations with the results while incrementing several other locations.
- `update_position_data`: Updates multiple position coordinates by processing data from a buffer, dividing values by two, adding constants, and storing results to specific memory locations while managing a buffer pointer.
- `write_graphics_registers`: Writes multiple 16-bit values to a series of global variables located at fixed offsets, likely configuring graphics hardware registers for display settings.
- `initialize_graphics_data`: Writes fixed 16-bit values to multiple global variables and then adds a constant value to a subset of those variables.
- `frag_jsr_subi_rts`: Short code fragment that calls another routine, then subtracts 60 from a fixed memory address (5 times in a row), then returns.
- `update_sprite_data_3`: Updates sprite position data by reading values from a table and storing them at specific offsets, then increments a pointer to the table and resets it when reaching the end.
- `adjust_coordinates`: Adjusts two coordinate values by subtracting them from forty-seven and storing the results back to memory.
- `adjust_coordinate_values`: Adjusts four coordinate values by subtracting them from seventy-five and storing the results back to their respective locations.
- `limit_and_update_coordinates`: Limits coordinate values to a maximum of seventy-five, sets fixed values for graphics data, calculates adjusted differences, and updates multiple memory locations with these values.
- `frag_jsr_move_rts_2`: Short code fragment that calls another routine, then copies 65 to a fixed memory address (5 times in a row), then returns.
- `iterate_structure_fields`: Calls two routines sequentially, loads a value from memory, decrements it, and executes a loop that calls another routine for each iteration until the counter reaches zero.
- `update_collision_flags`: Processes collision detection flags for multiple objects, updating their status based on position and boundary checks before calling a handler routine.
- `frag_cmpi_bne_beq_subi_move_rts`: Short code fragment that compares a fixed memory address with 1, then branches if the last result was not zero, then compares a fixed memory address with 0, then branches if the last result was zero, then subtracts 1 from a fixed memory address, then copies 35 to a fixed memory address, then returns.
- `frag_cmpi_bne_jsr_rts`: Short code fragment that compares a fixed memory address with 0, then branches if the last result was not zero, then calls another routine, then returns, then compares a fixed memory address with 0, then branches if the last result was not zero, then calls another routine, then returns.
- `decrement_counters`: Decrements two byte counters at fixed memory addresses and calls another routine when each reaches zero, then returns.
- `check_collision_pairs`: Processes pairs of collision data structures, checking if certain conditions are met for position and size, then clears flags and calls another routine.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `frag_cmpi_bne_rts`: Short code fragment that compares a fixed memory address with 106, then branches if the last result was not zero, then compares a fixed memory address with 106, then branches if the last result was not zero, then returns.
- `bitwise_pattern_check`: Performs bitwise OR and AND operations on memory values with global variables, storing the result in a global flag based on whether the final value is non-zero.
- `update_animation_state_2`: Updates animation state by incrementing counters and modifying pointers in a loop, changing state when certain thresholds are reached.
- `update_collision_data`: Processes collision detection data by checking multiple conditions and updating coordinates, then calls several subroutines to handle the results.
- `blitter_copy_sequence_1`: Configures blitter registers for multiple copy operations with fixed parameters, waits for each operation to complete, then updates source pointers and repeats.
- `blitter_copy_sequence_2`: Configures blitter registers for three copy operations with fixed parameters, waits for each operation to complete, then updates source pointers and repeats.
- `frag_movea_jsr_rts`: Short code fragment that copies 28236 to a6, then calls another routine, then copies 28262 to a6, then calls another routine, then returns.
- `decrement_and_loop`: Reads a byte from a global variable, decrements it, and loops that many times calling another routine before returning.
- `decrement_and_loop_2`: Reads a byte value from a global structure at offset 23, decrements it, and if the result is not zero, calls two other routines in a loop while incrementing a pointer, then calls another routine before returning.
- `reset_collision_flags`: Clears multiple collision-related flags and counters when a global condition is not met, otherwise processes collision data and updates positions.
- `handle_coordinate_updates`: Updates coordinate values based on input parameters, clears certain memory locations, and manages conditional blitter operations for graphics rendering.
- `process_input_conditions`: Checks input conditions from joystick data, updates state variables, and performs blitter operations when specific criteria are met.
- `frag_cmpi_beq_rts_clr_move_sub_bhi`: Short code fragment that compares a fixed memory address with 1, then branches if the last result was zero, then returns, then clears d3, then copies a fixed memory address to d3, then subtracts d3 from the memory at an offset from a6, then compares the memory at an offset from a6 with 32, then branches if the comparison was higher (unsigned), then clears a fixed memory address, then returns.
- `frag_cmpi_beq_rts`: Short code fragment that compares a fixed memory address with 0, then branches if the last result was zero, then returns.
- `initialize_data_structure`: Initializes a data structure by copying values from a source buffer to a target buffer, updating pointers and performing multiplication operations.
- `manage_blitter_parameters`: Configures multiple blitter operations with varying parameters based on state conditions, updates memory pointers, and performs sequential DMA transfers.
- `configure_blitter_dma`: Sets up blitter registers for a DMA operation with specific parameters including memory addresses, sizes, and control words, then waits for completion.
- `configure_multi_blitter_dma`: Configures multiple blitter operations with varying parameters based on state conditions, updates memory pointers, and performs three sequential DMA operations.
- `setup_blitter_dma`: Configures blitter registers for DMA operations using data from multiple global pointers and performs three sequential blitter operations with incremented addresses.
- `setup_blitter_dma_v2`: Sets up blitter parameters for memory transfers using a different source pointer and performs three blitter operations with modified address calculations.
- `setup_blitter_conditional`: Initializes blitter registers for DMA transfer when a condition check passes, then executes three blitter operations with specific size and address parameters.
- `setup_blitter_simple`: Configures basic blitter parameters for a simple DMA operation and performs three repeated blitter operations with fixed size and incremented addresses.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `frag_tst_beq_movea_adda_jsr_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then copies a fixed memory address to a0, then adds 68 to a0, then copies a fixed memory address to a1, then calls another routine (2 times in a row), then returns.
- `calculate_position_offsets`: Calculates position-related values from input parameters and stores intermediate results in a local data structure for later use in graphics operations.
- `setup_blitter_dynamic`: Calculates dynamic blitter parameters based on global data and executes three blitter operations with varying sizes and address increments.
- `setup_blitter_offset`: Initializes blitter registers using pre-calculated offsets from global pointers and performs three blitter operations with fixed size parameters.
- `setup_blitter_modular`: Configures blitter parameters for modular DMA transfers and executes three blitter operations with specific address increments and fixed size values.
- `setup_blitter_large`: Sets up blitter registers for large memory transfers using specific module sizes and performs three blitter operations with substantial address increments.
- `setup_blitter_loop`: Initializes blitter parameters for a looped DMA operation, executes three blitter operations, and updates a pointer in the routine's local storage.
- `setup_blitter_conditional_loop`: Configures blitter parameters with conditional size calculation based on global data, executes three blitter operations, and updates a pointer in the routine's local storage.
- `configure_blitter`: Sets up blitter registers for a graphics operation using values from the routine's parameter structure, then calls another routine three times with incrementing addresses.
- `compute_position`: Calculates position-related values from input parameters and stores intermediate results in a local data structure.
- `calculate_coordinates_2`: Performs similar coordinate calculations to the previous routine but with different input offsets, storing computed values in a local data structure.
- `calculate_coordinates_3`: Computes coordinate values and offsets based on input parameters, storing results in a local data structure for later use in graphics operations.
- `update_global_counter`: Updates a global counter variable based on conditional checks of the same variable's current value, with calls to other routines when specific conditions are met.
- `update_position_and_status`: Updates position coordinates and status flags based on conditional checks, then clears specific memory locations before returning.
- `process_coordinate_adjustments`: Processes coordinate adjustments by comparing values, updating memory locations, and managing conditional blitter operations for graphics rendering.
- `compute_offset_value`: Calculates an offset value by multiplying a word from memory by 56, then adds a division result and stores the final value in memory.
- `compute_offset_value_2`: Performs similar calculations to the previous routine but with different memory addresses, computing an offset value and storing it in memory.
- `rotate_and_store_bits`: Rotates a bit pattern based on a global variable value, then stores the rotated bits into memory locations pointed to by another global variable.
- `shift_and_store`: Reads a 16-bit value from a global table, performs a bitwise rotation on it, and stores the rotated bytes back into a buffer.
- `clear_memory_blocks`: Clears memory blocks by writing zeros to specific addresses in three separate memory regions, then updates a pointer to point to a new memory location.
- `clear_structure_fields`: Clears specific byte fields in three memory locations pointed to by a global variable and updates the global variable with a value from another global location.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `update_pointer_and_assign`: Compares two values from global variables, updates a pointer based on the comparison result, then assigns new values to three global locations using the updated pointer.
- `split_and_store_words`: Extracts high and low words from a long value, stores them in a table, and returns the original value plus a constant.
- `process_joystick_input`: Reads joystick port data to determine input state, updates global variables based on comparison results, and modifies coordinate values accordingly.
- `frag_cmpi_beq_rts_bhi_addi`: Short code fragment that compares a fixed memory address with 1, then branches if the last result was zero, then returns, then compares a fixed memory address with 165, then branches if the comparison was higher (unsigned), then adds 1 to a fixed memory address, then returns.
- `compare_and_write_values`: Compares values in a table with a global variable, writes bytes to global locations when a condition is met, and returns immediately upon finding a match.
- `update_direction_counter`: Increments a counter and checks conditions to update direction values, then calls another routine to process the data.
- `frag_clr_move_add_addi_jsr_rts`: Short code fragment that clears d0, then copies a fixed memory address to d0, then adds d0 to a fixed memory address, then adds 1 to d0, then adds d0 to a fixed memory address, then calls another routine (2 times in a row), then returns.
- `calculate_and_store_coordinates`: Calculates coordinate values based on a global variable's lower and upper nibbles, stores results in multiple global variables, and adjusts the original variable if it exceeds a threshold.
- `compute_rotated_coordinates`: Computes rotated coordinate values by manipulating nibbles from a global variable, combines them with another global value, and stores results in multiple global locations.
- `decrement_byte_values`: Decrements specific byte values in a memory structure across multiple locations using a loop counter.
- `initialize_frame_pointers`: Sets up multiple frame pointers in memory by adding fixed offsets to an initial address, then calls other routines to process the data.
- `copy_and_clear_word_data`: Copies word data between multiple memory locations and clears the final destination locations after copying.
- `copy_and_clear_large_data`: Copies larger blocks of word data between memory locations and clears the final destination locations after copying.
- `initialize_and_modify_byte_fields`: Initializes byte fields in two memory structures with calculated values, modifies specific bits based on conditions, and updates corresponding data in other memory locations.
- `initialize_frame_data`: Allocates memory for frame data and initializes it with video position information from hardware registers, then stores sequential values and flags in the allocated structure.
- `copy_word_pairs`: Copies 10 word pairs from one memory location to another by extracting high and low words from 32-bit values and storing them as separate words.
- `copy_word_pairs_2`: Copies 4 word pairs from one memory location to another by extracting high and low words from 32-bit values and storing them as separate words.
- `copy_word_pairs_3`: Copies word pairs from one table to another, advancing both tables by four bytes after each copy operation.
- `copy_word_pairs_4`: Copies 14 word pairs from one memory location to another by extracting high and low words from 32-bit values and storing them as separate words.
- `frag_movea_move_dbra_rts_2`: Short code fragment that copies a fixed memory address to a0, then copies 113 to d0, then copies 22488 to a1, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then loops back while counting down d0, then returns.
- `frag_movea_move_dbra_rts_3`: Short code fragment that copies a fixed memory address to a0, then copies 119 to d0, then copies 22940 to a1, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then loops back while counting down d0, then returns.

### Excalibur (1988)(J. Meggers)[m OKS].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
