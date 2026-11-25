# Game 3254 (croins (1997)(l. carminati)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03254). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901200 top-level bytes in 13 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 202 of 202 named (0 author symbols), 202 of 202 described (55 by rule, 127 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 393 of 393 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4336 of 4336 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 733159 |
| code | 17422 |
| data | 79094 |
| padding | 2917 |
| unused | 68608 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Croins (1997)(L. Carminati).adf:1 | 8 | 4309 | 200 |
| Croins (1997)(L. Carminati).adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |

## Cases handled

- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Croins (1997)(L. Carminati).adf:1

- `initialize_system`: Initializes system libraries and hardware registers, sets up task priority and blitter ownership, and configures display timing loops.
- `initialize_screen`: Sets up screen resources by calling intuition library functions and graphics library functions, then performs several operations on a global variable before returning.
- `setup_display`: Configures display parameters by setting values in a global structure, then calls graphics library functions to perform screen operations and finally frees allocated resources.
- `initialize_device`: Initializes device parameters, opens a device using the exec library, performs I/O operations, and handles error conditions during device setup.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `cleanup_device`: Checks if a cleanup flag is set, and if not, initializes device parameters and performs I/O operations before marking the cleanup as complete.
- `allocate_signal`: Allocates a signal using the exec library, stores the signal number in a structure, and sets up task-related fields in the same structure before returning.
- `call_free_signal`: Library stub: loads its arguments and the exec library base, calls FreeSignal and returns its result to the caller.
- `initialize_hardware`: Initializes hardware registers including DMA control, interrupt enable, and audio registers with specific values.
- `save_dma_registers`: Saves DMA and interrupt controller register values to global variables, then restores them before returning.
- `frag_movem_movea_move_cmp_bcs_jsr_rts`: Short code fragment that saves registers on the stack, then copies a fixed memory address to a6, then copies the memory at an offset from a6 to d0, then compares d0 with 37, then branches if the carry flag is set, then calls another routine, then restores saved registers from the stack, then returns.
- `update_game_state_4`: Updates game state variables based on conditional checks and arithmetic operations, including handling counters, flags, and memory addresses.
- `check_and_set_flag`: Checks a global variable for a specific value and sets another global variable to one if the condition is met.
- `update_game_state_3`: Updates multiple global variables based on conditional checks and arithmetic operations, then performs audio and DMA configuration before branching to different execution paths.
- `initialize_structure_fields`: Clears several memory locations, sets specific values in memory, calls multiple routines for initialization, and copies data between memory regions using a loop.
- `configure_audio_data`: Sets specific memory values, calls initialization routines, clears memory locations, and copies data between memory regions using a loop.
- `frag_clr_move_rts`: Short code fragment that clears a fixed memory address, then copies an address to a fixed memory address, then copies an address to a fixed memory address, then clears a fixed memory address (5 times in a row), then returns.
- `update_position_and_sequence`: Updates position coordinates and sequence values based on bit flag checks and arithmetic operations, then clears or modifies global state variables accordingly.
- `update_sequence_value_2`: Updates a sequence value in memory by reading from a pointer, performing arithmetic and bit manipulation operations, storing the result back.
- `validate_range_1`: Validates that a global byte value equals one, then checks if two word values fall within specific ranges before calling another routine.
- `validate_range_2`: Validates that a global byte value equals two, then checks if two word values fall within specific ranges before calling another routine.
- `update_game_state`: Updates game state variables based on conditional logic and arithmetic operations, including handling counters, flags, and memory addresses.
- `validate_range_3`: Checks two global byte flags, reads word values from memory locations, performs subtractions, and validates the results against range limits before calling another routine.
- `clear_structure_fields`: Clears specific byte and word fields in a structure multiple times while decrementing a counter, then sets a field based on the counter value.
- `frag_cmpi_beq_movea_moveq_bsr_addq_dbra`: Short code fragment that compares a fixed memory address with 2, then branches if the last result was zero, then copies 13832 to a0, then loads 9 into d0, then calls another routine, then adds 4 to a0, then loops back while counting down d0, then returns.
- `initialize_enemy_data`: Initializes enemy position and audio parameters based on global state and table lookups, then configures audio channels and updates sequence values.
- `validate_range_4`: Checks two global byte flags, reads word values from memory locations, performs subtractions, validates the results against range limits, and clears specific structure fields before calling another routine.
- `validate_range_5`: Validates that a global byte value equals one, then checks if two word values fall within specific ranges, clears specific structure fields, and calls two routines with a fixed value.
- `validate_conditions`: Checks multiple boundary conditions on byte and word values from registers against fixed constants, clears specific memory locations when all conditions are met, then calls two other routines with a predefined value.
- `update_audio_state`: Checks multiple conditions involving global variables and coordinate differences, clears specific memory locations when conditions are met, then calls another routine and updates global state variables before returning.
- `process_audio_sequence`: Processes audio sequence data by initializing memory locations, performing arithmetic operations on values read from global tables, storing results in memory, and updating various global flags based on comparison results.
- `initialize_audio_data`: Initializes audio data structures by clearing memory locations, performing multiplication and addition operations on global values, storing results in memory locations, and updating global state variables with specific values.
- `configure_audio_parameters`: Configures audio parameters by checking global conditions, performing arithmetic operations on global values, storing results in memory locations, updating global flags, and setting specific values in memory based on comparison results.
- `frag_movea_moveq_bsr_addq_dbra_rts`: Short code fragment that copies 14112 to a0, then loads 31 into d0, then calls another routine, then adds 4 to a0, then loops back while counting down d0, then returns.
- `update_position_and_state`: Updates position coordinates based on velocity values and clamps them within specific ranges.
- `check_state_and_execute`: Validates that a byte value at a fixed offset equals one, and that word values from registers fall within specific ranges, then calls another routine if all conditions are satisfied.
- `manage_audio_channel`: Manages audio channel states by checking multiple conditions, updating memory locations with specific values, calling audio setup routines with different parameters, and modifying global variables based on conditional branches.
- `verify_state_and_proceed`: Tests whether a byte value at a fixed offset equals three, and verifies that word values from registers remain within specified bounds, then invokes another routine when all checks pass.
- `confirm_parameters_and_call`: Ensures a byte value at a fixed offset is one, and confirms that word values from registers are within defined limits, then executes another routine if the conditions are fulfilled.
- `validate_position_and_invoke`: Verifies that a byte value at a fixed offset equals two, and checks that word values from registers stay within certain thresholds, then calls another routine when all criteria are met.
- `test_conditions_and_execute`: Checks if a byte value at a fixed offset is three, and ensures word values from registers fall within specific ranges, then executes another routine upon successful validation.
- `assess_state_and_continue`: Determines whether a byte value at a fixed offset equals one, and validates that word values from registers are within given bounds, then calls another routine if all tests pass.
- `evaluate_parameters_and_call`: Confirms that a byte value at a fixed offset is two, and verifies that word values from registers remain within specified limits, then invokes another routine when conditions are satisfied.
- `verify_values_and_proceed`: Tests if a byte value at a fixed offset equals three, and ensures word values from registers stay within defined thresholds, then calls another routine upon successful validation.
- `update_game_state_2`: Checks multiple conditions based on memory values and coordinates, updates state variables, and performs arithmetic operations on data pointers before calling other routines.
- `process_collision_data`: Validates collision parameters using coordinate comparisons, manages countdown timers, and updates memory locations with calculated values while calling helper routines.
- `transfer_audio_data`: Transfers audio data between memory locations by copying values from one structure to another, performing arithmetic operations on memory values, calling audio setup routines, and updating global state variables based on conditional checks.
- `calculate_audio_value`: Calculates an audio value by calling a routine, performing multiplication and bit shifting operations, adding a global table value, storing the result in a register, and calling another routine with conditional execution based on sign.
- `update_counters_and_flag`: Increments a counter at one memory location, checks if it reaches a threshold, resets the counter when reached, decrements another counter, and sets a flag to one when that counter reaches zero.
- `initialize_graphics_data`: Initializes graphics data by setting up memory locations with calculated values through multiplication and addition operations, calling a routine multiple times, and updating memory locations in a loop until a counter reaches zero.
- `update_positions`: Processes 20 elements in a data structure, adjusting values at offsets 0 and 80 based on shift operations and boundary checks, then advances to the next element.
- `frag_move_clr_movea_moveq_bsr_rts`: Short code fragment that copies 5 to a fixed memory address, then clears a fixed memory address, then copies 4578 to a6, then copies 4614 to d7, then copies 640 to d6, then loads 64 into d5, then calls another routine, then returns.
- `frag_move_clr_movea_moveq_bsr_rts_2`: Short code fragment that copies 3 to a fixed memory address, then clears a fixed memory address, then copies 20136 to a6, then copies 3182 to d7, then copies 678 to d6, then loads 64 into d5, then calls another routine, then returns.
- `frag_move_movea_moveq_bsr_rts`: Short code fragment that copies 2 to the memory at an offset from a1, then copies 18 to the memory at an offset from a1, then copies 128 to the memory at an offset from a1, then copies 15 to the memory at an offset from a1, then copies 20136 to a6, then copies 3182 to d7, then copies 570 to d6, then loads 40 into d5, then calls another routine, then returns.
- `frag_add_move_cmp_bcs_addq_addi_rts`: Short code fragment that adds d7 to a fixed memory address, then copies a fixed memory address to d7, then compares d7 with a fixed memory address, then branches if the carry flag is set, then adds 1 to a fixed memory address, then adds 10000 to a fixed memory address, then returns.
- `calculate_parameters`: Computes multiple values from a global variable using arithmetic operations and stores results in various memory locations, including word and long values.
- `initialize_graphics`: Reads global variables to determine initialization path, performs division operations, and calls multiple graphics routines to set up display parameters.
- `calculate_coordinates`: Calculates coordinate values by reading from global tables, performing arithmetic operations on byte values, comparing results with thresholds, and storing modified values back to registers while handling conditional branches based on comparison outcomes.
- `process_coordinate_data`: Processes coordinate data by reading from global tables, performing arithmetic operations on byte values with right shift operations, comparing results with thresholds, and storing modified values back to registers while handling conditional branches based on comparison outcomes.
- `initialize_graphics_data_2`: Initializes graphics data by loading word values from memory locations into registers, performing conditional comparisons, and setting a byte value based on the comparison results.
- `frag_bchg_bne_move_rts`: Short code fragment that flips bit 0 of a fixed memory address, then branches if the last result was not zero, then copies 0 to a fixed memory address, then copies 16310 to a fixed memory address, then copies 13476 to a fixed memory address, then returns, then copies 39200 to a fixed memory address, then copies 16826 to a fixed memory address, then copies 13556 to a fixed memory address, then returns.
- `frag_movea_move_moveq_bsr_rts`: Short code fragment that copies 440 to a6, then copies a fixed memory address to d7, then loads 40 into d6, then loads 4 into d5, then calls another routine, then returns.
- `validate_input_state`: Checks a specific global flag value and if true, reads coordinate data from memory locations and calls another routine with the values.
- `iterate_coordinate_processing`: Processes a fixed number of coordinate entries by reading from memory locations, calling a processing routine, and incrementing pointers in a loop.
- `process_pattern_data`: Iterates through pattern data entries, reads values from memory locations, calls processing routines with calculated parameters, and updates pointer addresses.
- `setup_display_elements`: Checks global state flags and performs conditional operations based on those flags, calling multiple routines to initialize display parameters and update memory locations.
- `frag_cmpi_bne_move_moveq_movea_bsr_rts`: Short code fragment that compares a fixed memory address with 7, then branches if the last result was not zero, then copies 232 to d6, then copies 159 to d7, then loads 7 into d4, then loads 14 into d5, then copies 2576 to a6, then calls another routine, then returns.
- `frag_tst_beq_move_moveq_movea_bsr_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then copies 256 to d6, then copies 267 to d7, then loads 4 into d4, then loads 14 into d5, then copies 12828 to a6, then calls another routine, then returns.
- `set_bit_flags`: Sets specific bit flags in memory locations based on calculated values from a data structure, with conditional flag setting for certain elements.
- `clear_bit_flags`: Clears specific bit flags in memory locations for 20 elements in a data structure, with conditional clearing for certain elements based on a comparison.
- `copy_pattern_data_3`: Copies pattern data from a source table using loop iterations, performs arithmetic operations on values, and calls another routine with calculated parameters.
- `clear_and_copy_memory`: Clears memory locations in a loop, copies data between memory regions, and updates a global variable to indicate completion of the operation.
- `frag_movea_moveq_move_bsr_rts`: Short code fragment that copies 78400 to a6, then loads 1 into d7, then copies 320 to d6, then copies 11226 to a5, then calls another routine (2 times in a row), then returns.
- `frag_movea_move_moveq_bsr_rts_2`: Short code fragment that copies 78406 to a6, then copies 320 to d6, then copies 2 to d7, then loads 0 into d5, then copies a fixed memory address to d5, then loads 2 into d4, then calls another routine, then returns.
- `frag_bsr_bclr_beq_rts`: Short code fragment that calls another routine (3 times in a row), then clears bit 3 of a fixed memory address, then branches if the last result was zero, then returns.
- `initialize_memory_blocks`: Clears a block of memory by setting 3200 long words to zero, then initializes several data structures through a series of subroutine calls with specific parameters.
- `copy_and_process_data`: Copies data between memory locations based on comparison results, then writes a fixed byte value and calls several routines before returning.
- `load_score_file`: Opens a file named 'Croins-TopScores' for reading, reads data into memory, and closes the file handle.
- `manage_file_operations`: Opens a file using a specific name and mode, writes data to it, then closes the file.
- `configure_video_system`: Sets up video system registers and performs multiple subroutine calls to configure display parameters, including blitter setup and screen configuration operations.
- `configure_blitter`: Configures blitter registers with specific values and sets up source and destination addresses for multiple operations in a loop.
- `initialize_display_buffers`: Clears multiple memory regions with specific patterns, processes display buffer data through loops, and manages global flags during initialization sequence.
- `clear_memory_block`: Clears a 512-word block of memory starting at a specific address by writing zero values to each word location, then performs several operations including register saving and memory copying.
- `update_coordinates_2`: Updates coordinate values by calling another routine multiple times with different parameters, including address pointers, memory values, and counter values.
- `frag_lea_moveq_bsr_tst_beq_clr_rts`: Short code fragment that loads the address of a location in the code into a0, then loads the address of a location in the code into a1, then loads 2 into d0, then calls another routine, then tests d0, then branches if the last result was zero, then clears a fixed memory address, then returns.
- `frag_lea_tst_beq_bsr_rts`: Short code fragment that loads the address of a location in the code into a0, then tests the memory at a0, then branches if the last result was zero, then calls another routine, then returns.
- `setup_interrupt_handler`: Configures interrupt handling by checking a pointer value, calling another routine if the pointer is valid, updating memory locations with function addresses, and modifying hardware register values.
- `frag_cmpi_beq_move_movea_bsr_rts`: Short code fragment that compares a fixed memory address with 0, then branches if the last result was zero, then copies 8192 to a fixed memory address, then copies 159 to a fixed memory address, then copies a location in the code to a0, then copies a location in the code to the memory at an offset from a0, then calls another routine, then returns.
- `handle_interrupt`: Checks interrupt status flags from hardware registers, calls another routine if conditions are met, and sets an interrupt flag before returning.
- `initialize_data`: Writes specific byte values to memory locations in a loop, setting repeated patterns of data for initialization purposes.
- `frag_ori_move`: Short code fragment that merges 0 into d0, then copies the memory at an offset from a6 to d7 (5 times in a row).
- `frag_moveq_move_swap_addq_dbra_rts`: Short code fragment that loads 7 into d0, then copies 14688 to d1, then copies d1 to the memory at an offset from a6, then swaps the halves of d1, then copies d1 to the memory at an offset from a6, then swaps the halves of d1, then adds 8 to a6, then loops back while counting down d0, then returns.
- `setup_graphics_data`: Calculates memory addresses based on input parameters, writes byte values to specific offsets in the calculated structure, and stores word values in global variables.
- `frag_move_rts`: Short code fragment that copies an address to a fixed memory address (2 times in a row), then returns.
- `validate_and_process_memory`: Validates coordinate ranges using comparison operations, calculates offsets based on input values, calls multiple routines to process data, and updates a memory pointer with a specific value.
- `adjust_value_limit`: Adjusts a value based on comparison with calculated limits, updating a memory location with either the adjusted or original value depending on conditions.
- `initialize_blitter_params`: Sets up blitter parameters based on input values and conditionally configures bitmasks for blitter operations.
- `configure_blitter_dma`: Configures DMA registers for blitter operations including memory addresses, sizes, and control words.
- `setup_blitter_operations`: Initializes blitter operations by setting up control registers and processing a data structure with conditional execution.
- `frag_movem_move_beq_bsr_addq_bra_rts`: Short code fragment that saves registers on the stack, then copies the memory at a5 (advancing a5) to d0, then branches if the last result was zero, then calls another routine, then adds 1 to a6, then jumps to another point in the code, then restores saved registers from the stack, then returns.
- `copy_pattern_data`: Copies pattern data from a source table to multiple destination offsets based on bit flags and loop counters.
- `copy_pattern_data_2`: Copies pattern data from a source table by iterating through elements, performing arithmetic operations on values, calling another routine for processing, and updating loop counters.
- `process_input_buffer`: Processes input buffer data by testing bit flags, comparing values with constants, conditionally storing bytes to memory locations, and updating counters through loop iterations.
- `frag_btst_bne_moveq_move_bset_movea_rts`: Short code fragment that tests bit 7 of a fixed memory address, then branches if the last result was not zero, then loads 0 into d0, then copies a fixed memory address to d0, then sets bit 7 of a fixed memory address, then copies 18152 to a0, then copies an indexed entry from a0 to d0, then returns, then loads 0 into d0, then returns.
- `copy_structure_data`: Copies structured data elements from one memory location to another, processing multiple data blocks with specific byte counts and updating pointers during the copy operation.
- `parse_structure_data`: Parses structured data by comparing against a fixed identifier and handling conditional data movement based on bit flags.
- `copy_encoded_data`: Copies encoded data by reading byte values from a source pointer, extending them to word values, handling negative counts with repetition logic, and storing results to a destination pointer.
- `setup_audio_channel_0`: Configures audio channel 0 settings including DMA control, volume, and period values while managing global state flags.
- `setup_audio_channel_1`: Configures audio channel 1 settings including DMA control, volume, and period values while managing global state flags.
- `setup_audio_channel_2`: Configures audio channel 2 settings including DMA control, volume, and period values while managing global state flags.
- `setup_audio_channel_3`: Configures audio channel 3 settings including DMA control, volume, and period values while managing global state flags.
- `start_audio_channels`: Activates audio channels by setting DMA control registers and length values based on global state flags.
- `frag_clr_rts`: Short code fragment that clears a fixed memory address (4 times in a row), then returns.
- `frag_btst_beq_bne_rts`: Short code fragment that tests bit 0 of a fixed memory address, then branches if the last result was zero, then tests bit 0 of a fixed memory address, then branches if the last result was not zero, then returns.
- `frag_subq_bsr_dbra_rts`: Short code fragment that subtracts 1 from d7, then calls another routine, then loops back while counting down d7, then returns.
- `compare_and_update_value`: Repeatedly calls another routine to retrieve a value, compares it with a register value, and updates control flow based on the comparison results until certain conditions are met.
- `frag_move_lsr_andi_rts`: Short code fragment that copies a fixed memory address to d7, then shifts d7 right by 8, then masks d7 with 511, then returns.
- `update_coordinates`: Updates coordinate values in a structure by decrementing a counter, storing word values at specific offsets, performing arithmetic operations, and looping until the counter reaches zero.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `process_joystick_input`: Processes joystick input data from hardware registers, performs bitwise operations on the data, updates a global variable with modified values based on conditional checks, and returns the result.
- `frag_movea_move_moveq_rol_dbra_rts`: Short code fragment that copies 19204 to a0, then copies a0 to a fixed memory address, then copies a fixed memory address to the memory at a0, then copies a fixed memory address to the memory at an offset from a0, then copies the memory at a0 (advancing a0) to d0, then loads 6 into d1, then rotates d0 left by 7, then copies d0 to the memory at a0 (advancing a0), then loops back while counting down d1, then returns.
- `update_sequence_value`: Updates a sequence value in memory by reading from a pointer, performing arithmetic and bit manipulation operations, storing the result back to memory, and updating the pointer to point to the next location.
- `allocate_and_process_memory`: Allocates memory using a system call, processes pixel coordinates by iterating through data and storing results, then frees allocated memory and updates global state variables.
- `process_pixel_coordinates`: Processes pixel coordinates by iterating through data, performing calculations on values, and storing results in memory locations.
- `process_pixel_data`: Processes pixel coordinate data through a loop that shifts and combines values, performs multiplication and division operations, and returns the final combined result.
- `validate_coordinates`: Checks if a value at a specific memory location equals -2, or validates bit flags and range conditions before returning with appropriate status codes.
- `free_and_cleanup_resources`: Frees allocated memory using a system call, processes pixel coordinates by copying values from one location to another, and updates global state variables accordingly.
- `initialize_graphics_2`: Sets up graphics registers including DMA control, sprite positions, and bitplane configurations.
- `handle_input_interrupt`: Processes keyboard input interrupt by reading hardware registers, performing bit manipulation on input data, and updating control bits while synchronizing with vertical blanking periods.
- `update_control_bits`: Updates control bits in a global variable based on comparison results with specific byte values, setting or clearing individual bit positions according to matching conditions.
- `setup_system_interrupts`: Initializes interrupt and DMA control registers, calls several system routines for task management, and returns after setting up the system state.
- `setup_interrupts`: Disables system tasks and sets up interrupt handling by configuring interrupt enable registers, clearing interrupt requests, and setting up DMA control registers before returning.
- `load_file_data`: Loads file data into memory by locking a directory, allocating memory for file information, examining the file, allocating memory for the file content, reading the file content, and cleaning up resources.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `update_visual_elements`: Updates visual elements by processing data from an array, performing bit shifting operations, storing values in a table, and setting various hardware registers to control audio output.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `frag_movem_addq_move_cmp_bcs_clr_tst_be`: Short code fragment that saves registers on the stack, then adds 1 to a fixed memory address, then copies a location in the code to d0, then compares d0 with a location in the code, then branches if the carry flag is set, then clears a fixed memory address, then tests a fixed memory address, then branches if the last result was zero, then calls another routine, then jumps to another point in the code.
- `frag_bsr_bra`: Short code fragment that calls another routine, then jumps to another point in the code.
- `process_audio_channels`: Processes audio channel configuration based on a global state value by loading different addresses into register a6 and calling another routine with varying parameters.
- `update_audio_channels`: Updates audio channel registers based on a global variable's value, clears a timing counter, and manages audio parameter loading for multiple channels.
- `initialize_object_state`: Initializes object state variables based on input data, updates position coordinates, and sets up movement parameters before proceeding with object processing.
- `update_position_and_state_2`: Updates position coordinates and state flags based on input values, then processes state transitions and updates global variables accordingly.
- `frag_bsr_bra_2`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_3`: Short code fragment that calls another routine, then jumps to another point in the code.
- `update_animation_state`: Updates animation state based on a lookup table indexed by a value from the structure, then performs conditional operations on DMA and bit flags before returning.
- `frag_move_moveq_cmp_beq_dbra_rts`: Short code fragment that copies d1 to the stack, then loads 6 into d0, then copies a fixed memory address to d1, then compares d1 with a fixed memory address, then branches if the last result was zero, then loops back while counting down d0, then copies the top of the stack to d1, then returns.
- `manage_audio_parameters`: Manages audio parameters by updating DMA control registers, copying audio channel data to custom hardware registers, and modifying timing values based on conditional checks.
- `process_input_state`: Reads a byte value from a specific offset in a structure, performs bitwise operations on it, and branches based on the result to execute different code paths.
- `frag_move_rts_2`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a5, then returns.
- `calculate_lookup_index`: Calculates a lookup index by loading a byte value, dividing it by 3, swapping the result, testing the quotient, and branching based on comparison results with specific values.
- `frag_moveq_move_andi_bra`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_move_bra`: Short code fragment that copies the memory at an offset from a6 to d2, then jumps to another point in the code.
- `lookup_and_compare`: Looks up a value in a table using a byte from the structure, compares it with another value, and stores the table value if the condition is met.
- `frag_move_rts_3`: Short code fragment that copies d2 to the memory at an offset from a5, then returns.
- `update_position_limit`: Updates a position value with a masked input, checks against a threshold, and clamps the result to a maximum value before storing it.
- `update_position_range`: Increments a position value with a masked input, checks against a minimum threshold, and clamps the result to a minimum value before storing it.
- `update_animation_state_2`: Processes animation frame data based on input state and updates a global variable with the current frame value.
- `frag_clr_rts_2`: Short code fragment that clears the memory at an offset from a6, then returns.
- `frag_move_beq_clr`: Short code fragment that copies the memory at an offset from a6 to d0, then branches if the last result was zero, then copies d0 to the memory at an offset from a6, then clears the memory at an offset from a6.
- `adjust_movement_counter`: Manages a movement counter by adding or subtracting a byte value from it, then compares against a target value.
- `decrement_channel_period_and_lookup`: Subtracts the input value from the channel period and clamps it to the target when reached.
- `combine_input_bits`: Combines nibble values from two memory locations using bitwise operations, then calculates a scaled result based on arithmetic and bit manipulation before storing it.
- `calculate_scaled_value`: Processes input values to calculate a scaled result based on bit manipulation and arithmetic operations, then stores the result in a global variable.
- `frag_tst_bpl_move_sub_bra`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was not negative, then copies 255 to d2, then subtracts d0 from d2, then jumps to another point in the code.
- `frag_move_bra_2`: Short code fragment that copies d0 to d2, then jumps to another point in the code.
- `frag_move_andi_mulu_lsr_tst_bmi_add_bra`: Short code fragment that copies an indexed entry from a4 to d2, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then multiplies d2 by d0 (unsigned), then shifts d2 right by 7, then copies the memory at an offset from a6 to d0, then tests the memory at an offset from a6, then branches if the last result was negative, then adds d2 to d0, then jumps to another point in the code.
- `subtract_and_advance_phase`: Subtracts d2 from d0 and stores the result in offset 6 of a structure in a5, then advances a phase byte in the structure by an amount taken from the high nibble of a neighbouring byte.
- `frag_bsr_bra_4`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_5`: Short code fragment that calls another routine, then jumps to another point in the code.
- `process_input_scaling`: Processes input values through bit shifting and masking, performs conditional calculations based on input state, and clamps the final result within a range before storing it.
- `frag_tst_bpl_move_sub_bra_2`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was not negative, then copies 255 to d2, then subtracts d0 from d2, then jumps to another point in the code.
- `frag_move_bra_3`: Short code fragment that copies d0 to d2, then jumps to another point in the code.
- `multiply_and_shift`: Reads a byte from memory, multiplies it with another value, shifts the result right, and adds it to a third value before returning.
- `clamp_and_store_value`: Clamps the signed difference between d2 and d0 to a range between zero and sixty-four, stores the result as a word at offset 8 of a structure in a5, and updates a byte at offset 29 of a structure in a6 by adding the high nibble of the byte at offset 28 shifted right by two.
- `update_accumulator`: Performs conditional arithmetic operations on a value, updates a global accumulator, and modifies memory based on comparison results.
- `frag_move_rts_4`: Short code fragment that copies 1 to the memory at an offset from a6, then returns.
- `clamp_and_store`: Reads a byte from memory, processes it through bit shifting and addition, clamps the result to a maximum value, and stores it in another location.
- `decrement_and_clamp_byte`: Subtracts the low nibble of the byte at offset 3 of the structure in a6 from the byte at offset 19, clears that byte to zero when the result is negative, and stores the resulting byte as a word at offset 8 of the structure in a5.
- `decrement_and_store_byte`: Reads a byte from offset 3 of the structure in a6, subtracts one from it, and stores the result in a global byte, then clears two subsequent global bytes.
- `clamp_byte_and_store`: Reads a byte from offset 3 of the structure in a6, clamps it to a maximum of sixty-four, stores the clamped value as a byte at offset 19 of that structure, and stores it as a word at offset 8 of a structure in a5.
- `compute_and_validate`: Processes input values through multiplication, masking, and addition operations, validates the result against a threshold, and stores the validated value.
- `frag_move_beq_clr_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then branches if the last result was zero, then clears a fixed memory address, then copies d0 to a fixed memory address, then returns.
- `process_state_byte`: Reads a byte from offset 2 of the structure in a6, extracts its lower nibble, and uses it as an index to jump to different handling routines.
- `decode_state_byte`: Reads a byte from offset 3 of a structure, extracts its upper nibble, uses it as a switch index to jump to different routines, and returns immediately after the jump.
- `merge_bit_to_port_a`: Extracts bit 0 of a byte at offset 3 of a structure in a6, doubles it, and merges it into bit 1 of the CIAA port A register while clearing bit 1 of that register.
- `mask_and_merge_byte`: Masks the low nibble of a byte at offset 31 from a pointer in a6 and then merges the low nibble of a byte at offset 3 from the same pointer into it.
- `clear_low_nibbles_of_byte`: Clears the low nibble of a byte at offset 30 from an address in a6 and then ORs in the low nibble of another byte from the same base.
- `store_masked_byte`: The routine reads a byte from an offset in a structure pointed to by a register, masks it with a constant, and stores the result into a different offset in the same structure.
- `check_and_update_counter`: Tests a memory value and branches if zero, then decrements a counter and updates memory locations based on conditional checks before returning.
- `frag_move_bra_4`: Short code fragment that copies d0 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_move_lsr_rts`: Short code fragment that copies a location in the code to d0, then shifts d0 right by 4, then copies d0 to the memory at an offset from a6, then returns.
- `mask_and_merge_nibble`: Masks the low nibble of a byte at offset 30 from a pointer in a6 and merges the high nibble from a byte at offset 3 of the same pointer, storing the result back at offset 30.
- `update_dma_and_transfer`: Configures DMA control registers with values from a structure, performs memory transfers, and updates multiple memory locations based on conditional checks involving a counter and a divisor.
- `frag_tst_bne_moveq_move_andi_bra`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_tst_bne_moveq_move_andi_bra_2`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_moveq_move_andi_cmp_bne_clr_rts`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then compares d0 with a location in the code, then branches if the last result was not zero, then clears the memory at an offset from a6, then copies 0 to the memory at an offset from a5, then returns.
- `frag_moveq_move_andi_cmp_bne_beq_bra`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then compares d0 with a location in the code, then branches if the last result was not zero, then copies the memory at a6 to d0, then branches if the last result was zero, then copies d1 to the stack, then jumps to another point in the code.
- `frag_tst_bne_moveq_move_andi_addq_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then tests a fixed memory address, then branches if the last result was not zero, then adds 1 to d0, then copies d0 to a fixed memory address, then returns.
- `frag_tst_bne_move_andi_lsl_or_beq`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then shifts d0 left by 4, then masks the memory at an offset from a6 with 15, then merges d0 into the memory at an offset from a6, then tests d0, then branches if the last result was zero.
- `update_pointer_table`: Reads a byte from memory, uses it to index into a table, adds the table value to another location, clears the byte if bit 7 is set, and updates a pointer.

### Croins (1997)(L. Carminati).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_movem_movea_jsr_moveq_rts`: Short code fragment that saves registers on the stack, then copies a1 to a2, then copies an address to a0, then copies the memory at an offset from a2 to a1, then copies the memory at an offset from a2 to a2, then calls the routine whose address is in a2, then restores saved registers from the stack, then loads 0 into d0, then returns.
