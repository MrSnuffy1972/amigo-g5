# Game 4049 (j.m. aspar master grand prix (1989)(dinamic)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04049). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (921760 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 36 raw-word lines |
| Functions named and described | pass, 158 of 158 named (0 author symbols), 158 of 158 described (52 by rule, 106 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3396 of 3396 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4423 of 4423 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 598 |
| code | 19538 |
| data | 2538 |
| padding | 14 |
| unused | 899072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| J.M. Aspar Master Grand Prix (1989)(Dinamic).adf:bootblock | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4402 | 157 |

## Cases handled

- runtime segment an address: 3396 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 12 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 5 call target, 3 immediate+address, 2 immediate, 1 address, 1 immediate+displacement

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### J.M. Aspar Master Grand Prix (1989)(Dinamic).adf:bootblock

- `initialize_dma_block`: Allocates memory for a DMA block, sets up its parameters including size and command, performs an I/O operation to initialize it, then performs another I/O operation to clear it before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_blitter_config`: Reads a word value from a global variable, updates blitter configuration registers with specific values, and calls routines to process blitter operations.
- `handle_interrupt_event`: Checks for specific interrupt conditions from hardware registers, updates counters in memory, sets a flag when a counter reaches a threshold, and manages CIA timer control bits.
- `read_joystick_input`: Clears a joystick input flag, reads the current joystick state from hardware registers, applies bitwise operations to detect movement and button presses, and sets corresponding bits in the input flag based on the detected conditions.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `frag_andi_beq_rts`: Short code fragment that masks a fixed memory address with 64, then branches if the last result was zero, then returns.
- `set_interrupt_enable`: Writes a specific value to a global variable controlling interrupt enable bits and then executes a trap instruction.
- `frag_mulu_lea_subq_clr_dbra_rts`: Short code fragment that multiplies d0 by 8000 (unsigned), then loads the address of a fixed memory address into a0, then subtracts 1 from d0, then clears the memory at a0 (advancing a0), then loops back while counting down d0, then returns.
- `frag_move_swap_addi_addq_dbra_rts`: Short code fragment that copies an address to d0, then copies d0 to the memory at an offset from a0, then swaps the halves of d0, then copies d0 to the memory at an offset from a0, then swaps the halves of d0, then adds 8000 to d0, then adds 8 to a0, then loops back while counting down d1, then returns.
- `frag_lea_move_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a1, then copies 31 to d0, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then returns.
- `copy_data_block`: Copies data from one memory location to another in a structured pattern, using loop counters and address calculations to move multiple longwords at a time across a large data block.
- `frag_tst_beq_move_andi_bsr_mulu_addq_ad`: Short code fragment that tests the memory at a1, then branches if the last result was zero, then copies a1 to the memory at a0 (advancing a0), then copies the memory at a1 to a fixed memory address, then masks a fixed memory address with 127, then calls another routine, then multiplies d0 by 4 (unsigned), then adds 2 to d0, then adds d0 to a1, then jumps to another point in the code.
- `calculate_position_values`: Performs arithmetic operations on several byte and word values from memory locations, combining them through shifts, additions, multiplications, and masking to produce a final calculated value stored in memory.
- `setup_blitter_parameters`: Processes a word value through bit manipulation and table lookup, then configures multiple blitter hardware registers with various values including memory addresses, sizes, and control flags for graphics operations.
- `frag_move_add_ext_clr_mulu_rts`: Short code fragment that copies a fixed memory address to d0, then adds d0 to d0, then sign-extends d0, then clears d1, then copies a fixed memory address to d1, then multiplies d0 by d1 (unsigned), then copies d0 to a fixed memory address, then returns.
- `update_racing_data`: Updates racing position and timing data based on input values, then configures blitter registers for graphics rendering.
- `setup_blitter_operations`: Configures blitter registers with values from memory locations, performs conditional updates to global variables based on bit tests, and loops through a fixed count of iterations to set up graphics operations.
- `process_keyboard_input`: Checks interrupt conditions and keyboard status registers, reads a byte from a serial input, processes it through bit rotation and masking operations, stores the result in memory, and handles special cases based on the processed value.
- `frag_movem_rte`: Short code fragment that restores saved registers from the stack, then returns from the exception.
- `frag_movea_move_clr_bra`: Short code fragment that copies 76316 to a0, then copies 1 to an indexed entry from a0, then clears a fixed memory address, then jumps to another point in the code.
- `frag_lea_tst_rts`: Short code fragment that loads the address of a fixed memory address into a6, then tests an indexed entry from a6, then returns.
- `handle_interrupt_signals`: Checks interrupt status flags, processes different interrupt conditions based on bit tests, and manages audio channel updates or interrupt enable/disable operations accordingly.
- `frag_move_rol_and_bne_or_bra`: Short code fragment that copies d1 to d0, then rotates d0 left by 7, then copies d0 to a fixed memory address, then copies d1 to d3, then masks d1 with a fixed memory address, then branches if the last result was not zero, then copies d0 to a fixed memory address, then merges d3 into a fixed memory address, then copies d3 to a fixed memory address, then jumps to another point in the code.
- `frag_not_and_movem_rte`: Short code fragment that inverts d1, then masks a fixed memory address with d1, then restores saved registers from the stack, then returns from the exception.
- `update_audio_parameters`: Loads audio parameters from indexed memory locations, calculates audio channel values using multiplication and bit shifting, writes these values to hardware registers controlling sound output, and updates a counter.
- `frag_move_bra`: Short code fragment that copies 52162 to a fixed memory address, then jumps to another point in the code.
- `frag_subq_bne_move_addq_cmp_clr_bra`: Short code fragment that subtracts 1 from a fixed memory address, then branches if the last result was not zero, then copies 5 to a fixed memory address, then adds 1 to a fixed memory address, then copies a fixed memory address to d1, then compares d1 with a fixed memory address, then branches if the last result was not zero, then clears a fixed memory address, then jumps to another point in the code.
- `frag_subq_bne_move_clr_bra`: Short code fragment that subtracts 1 from a fixed memory address, then branches if the last result was not zero, then copies 5 to a fixed memory address, then subtracts 1 from a fixed memory address, then branches if the last result was not zero, then clears a fixed memory address, then jumps to another point in the code.
- `configure_dma_and_interrupts`: Sets up DMA and interrupt control registers with calculated values based on input parameters, performs a loop of repeated operations, writes data to specific hardware addresses using indexed addressing, and updates multiple control registers.
- `process_data_loop`: Copies data from one memory location to another with specific offset calculations, performs bitwise operations on a global variable, and executes a loop that continues until certain conditions are met.
- `process_sprite_data`: Calculates sprite-related memory addresses using multiplication and bit operations, copies byte data from source to destination locations, and performs conditional processing based on byte values.
- `copy_sprite_data`: Copies sprite data from a source buffer to multiple destinations in memory, handling both forward and backward data copying based on a bit test condition.
- `process_input_sequence`: Reads input bytes from a buffer and processes them based on command values, updating global variables with calculated results.
- `process_racing_data`: Reads racing event data from a buffer, updates global variables based on the data, and configures blitter registers for graphics rendering.
- `update_racing_position`: Updates racing position values using input data from registers, processes the data through multiple subroutine calls, and manages state transitions for racing operations.
- `write_tile_data`: Writes tile data to a specific memory location calculated from coordinates, with bounds checking for row and column values.
- `lookup_and_store`: Looks up a value in a table using the input register as an index, stores the result in a global variable, and sets another global variable to either the looked-up value shifted left by four bits or the constant eight depending on a condition.
- `handle_input_sequence_2`: Processes input bytes from a buffer through a lookup table, updates global variables with calculated values based on the input sequence, and performs conditional operations.
- `handle_input_sequence`: Reads input bytes from a buffer, processes them through a lookup table, and updates global variables with calculated values based on the input sequence.
- `handle_extended_input`: Processes extended input data by reading bytes from a buffer, applying bounds checking, and updating global variables with calculated values based on input sequence.
- `update_flags_loop`: Processes a block of data through multiple iterations, decrementing flag values in a structured manner while performing memory operations.
- `compare_and_update_flags`: Compares and updates flag values between two data structures, incrementally modifying each flag based on bit comparisons.
- `frag_cmpi_bne_move_rts`: Short code fragment that compares the memory at a1 (advancing a1) with an address, then branches if the last result was not zero, then copies a0 to the memory at an offset from a1, then copies an address to the memory at a1, then copies 1 to a fixed memory address, then returns.
- `process_character_data`: Processes character data from a buffer, updating a counter and calling another routine for character lookup when conditions are met.
- `frag_lea_andi_lsl_adda_movea_addq`: Short code fragment that loads the address of a fixed memory address into a1, then masks d0 with 255, then shifts d0 left by 2, then adds d0 to a1, then copies the memory at a1 to a1, then adds 2 to a1.
- `write_tile_data_v2`: Writes tile data to memory locations calculated from coordinates with bounds checking, performing similar operations to routine R2 but with different addressing.
- `process_character_data_v2`: Processes character data from a buffer, updating a counter and calling another routine for character lookup when conditions are met, similar to routine R5.
- `initialize_racing_data`: Initializes racing-related data by clearing counters, setting global variables to fixed values, and performing a delay loop before updating position and timing information based on input state.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `update_joystick_state`: Reads joystick input flags from a global variable, updates a counter based on direction bits, and performs boundary checks to ensure the counter stays within valid range.
- `update_input_counter`: Maintains a counter that increments or decrements based on joystick input flags, resets to zero when reaching certain bounds, and updates related graphics data structures.
- `calculate_checksum`: Calculates a checksum by summing bytes from a source buffer and storing the result in a destination buffer with rotation operations.
- `copy_input_data`: Copies twelve bytes from a fixed memory location to another location with a stride of sixteen bytes, then stores two bytes from the source into global variables.
- `update_input_state_2`: Processes joystick input flags to update a counter, clamps the counter between zero and one, and performs table lookups to configure graphics rendering parameters based on the counter's value.
- `update_input_state`: Updates an input state counter based on joystick flags, clamps it within a range, and performs table lookups to update global variables with coordinate values.
- `load_and_calculate_values`: Loads values from memory tables, performs arithmetic operations including division and bit shifts, then stores calculated results to global variables.
- `initialize_racing_data_2`: Initializes multiple global variables with specific values and performs arithmetic operations on a counter before returning.
- `initialize_race_state`: Sets specific values in global variables, clears a 16-bit variable, copies coordinate data from one location to another, and calls multiple routines to process input and update game state.
- `process_joystick_movement`: Checks joystick input flags, updates movement coordinates, and writes specific byte values to memory locations based on input state.
- `update_global_value`: Reads values from two registers, performs arithmetic operations including division and multiplication, combines the results, and stores the final value into a global variable.
- `clear_race_variables`: Clears multiple global variables and sets specific values in a data structure, then calls another routine to process the cleared data.
- `initialize_race_data`: Initializes race-related global variables and sets up display text for player input handling.
- `frag_clr_rts`: Short code fragment that clears a fixed memory address (4 times in a row), then returns.
- `decrement_and_copy`: Decrements a counter and sets a flag when the counter reaches zero, then copies data from different memory locations based on the counter's value.
- `copy_byte_sequence`: Copies bytes from one memory location to another until encountering a termination marker, then stores the termination marker at the end of the copied sequence.
- `frag_cmpi_bne_rts_bsr`: Short code fragment that compares a fixed memory address with 3, then branches if the last result was not zero, then returns, then compares a fixed memory address with 1, then branches if the last result was not zero, then returns, then calls another routine, then returns.
- `frag_move_cmp_bcs_clr_rts`: Short code fragment that copies a fixed memory address to d0, then compares d0 with a fixed memory address, then branches if the carry flag is set, then copies a fixed memory address to a fixed memory address (2 times in a row), then clears a fixed memory address, then returns.
- `frag_move_rts`: Short code fragment that copies the memory at an offset from a0 to a fixed memory address (9 times in a row), then returns.
- `frag_clr_bra`: Short code fragment that clears a fixed memory address, then jumps to another point in the code.
- `update_vehicle_physics`: Processes joystick input flags and updates vehicle position coordinates based on input values, then performs iterative physics calculations to adjust movement parameters before storing final position values.
- `process_graphics_data`: Reads word values from a data stream, updates graphics register values, and processes a loop of fixed-length data entries until encountering an end marker.
- `frag_move_bra_2`: Short code fragment that copies 61148 to a fixed memory address, then jumps to another point in the code.
- `update_race_parameters`: Updates various race-related parameters including coordinate values, speed limits, and velocity components based on input conditions and bit masks.
- `update_coordinates`: Updates coordinate values based on input conditions and bit masks, performing addition or subtraction operations on global variables.
- `update_speed_limit`: Updates a speed limit value based on comparison with another value, setting it to zero if the condition is met or calculating a derived value otherwise.
- `calculate_absolute_value`: Calculates the absolute value of two signed 16-bit values and stores them in corresponding global variables.
- `compute_magnitude`: Computes the absolute values of two signed 16-bit quantities and stores the results in designated global memory locations.
- `adjust_velocity_component`: Performs division on a value, applies conditional adjustments to a velocity component based on its sign, and updates the component accordingly.
- `update_velocity_components`: Performs division operations on coordinate values, adjusts velocity components based on comparison results, and manages state transitions for movement calculations.
- `update_velocity_components_2`: Copies velocity component values from a structure to global memory locations, performs arithmetic operations on the values, and stores the results in a global variable.
- `reset_velocity_components`: Checks if a counter matches a target value, clears velocity component flags when they match, resets counter values, and calls other routines to update velocity data.
- `manage_state_counter`: Increments a counter and manages state transitions based on comparison with threshold values, updating memory locations accordingly.
- `update_vehicle_position`: Updates vehicle position coordinates based on input values and performs iterative physics calculations, then stores the results in global variables.
- `frag_lea_move_andi_cmp_bls_clr_rts`: Short code fragment that loads the address of a fixed memory address into a1, then copies a fixed memory address to d0, then copies an indexed entry from a1 to d0, then copies d0 to d1, then masks d1 with 15, then compares d1 with 10, then branches if the comparison was lower or same (unsigned), then clears d0, then copies d0 to a fixed memory address, then returns.
- `compute_address_offset`: Calculates an address offset by performing arithmetic operations on values from global memory and stores the result in a register.
- `load_byte_to_memory`: Loads a byte value from a memory location, processes it through bitwise operations, and stores the result in a global variable.
- `validate_and_update_input`: Validates input conditions based on global variables and updates counters and flags accordingly, performing boundary checks and conditional operations on input data.
- `update_memory_counters`: Copies word values from two memory locations to other memory locations, then calls another routine and performs conditional checks on memory values before returning.
- `frag_lea_bsr_move_addi_rts`: Short code fragment that loads the address of a fixed memory address into a0, then calls another routine, then copies a fixed memory address to d0, then copies 36 to a fixed memory address, then copies 2 to a fixed memory address, then adds 48 to d0, then calls another routine, then returns.
- `frag_tst_beq_rts_move_bra`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then returns, then copies 5 to a fixed memory address, then copies 20 to a fixed memory address, then copies 1 to a fixed memory address, then jumps to another point in the code.
- `validate_and_set_flag`: Validates conditions based on comparisons between memory values and sets a flag to a specific value when conditions are met.
- `update_frame_index`: Increments a frame index stored in a global variable, resets it to zero if it exceeds a maximum value, and calculates memory addresses based on the index to read byte values for further processing.
- `convert_ascii_to_decimal`: Converts ASCII character digits to decimal values, performs arithmetic operations to calculate a combined decimal number from multiple digit inputs, and stores the result in memory.
- `convert_number_to_digits`: Converts a 16-bit number into its decimal digit representation and stores each digit as an ASCII character in consecutive memory locations.
- `frag_movea_move_subq_clr_adda_dbra_rts`: Short code fragment that copies 74880 to a0, then copies 16 to d0, then copies 12 to d1, then subtracts 1 from d1, then clears the memory at a0, then adds d0 to a0, then loops back while counting down d1, then returns.
- `load_and_scale_values`: Loads word values from memory locations, applies scaling and offset operations, then stores the results to specific memory addresses.
- `write_input_data`: Writes two 16-bit values from a table to global variables based on a command index, after setting specific bytes in a global variable and calling routines.
- `update_racing_position_3`: Updates racing position and timing data based on input values, configures blitter registers for graphics rendering, and processes character data from memory.
- `set_bit_mask_positions`: Sets bits in memory at multiple offsets using a bit mask calculated from input coordinates, with each position being 8000, 16000, and 24000 bytes apart.
- `frag_mulu_nop_subq_bne_rts`: Short code fragment that multiplies d0 by 65535 (unsigned), then pauses for one no-operation step (2 times in a row), then subtracts 1 from d0, then branches if the last result was not zero, then returns.
- `process_joystick_data`: Reads joystick input values and processes them through multiple subroutine calls, updating coordinate values based on the input data.
- `process_joystick_input`: Reads joystick state from hardware registers, applies bitwise operations to detect movement and button presses, and updates coordinate values based on input conditions.
- `frag_cmp_bcs_eor_rts_bcc_move`: Short code fragment that compares d1 with 32768, then branches if the carry flag is set, then exclusive-ors d1 into d1, then returns, then compares d1 with d0, then branches if the carry flag is clear, then returns, then copies d0 to d1, then returns.
- `frag_bsr_rts`: Short code fragment that calls another routine (3 times in a row), then returns.
- `process_input_state`: Reads a value from a global memory location and compares it against several constants.
- `frag_adda_move_cmp_rts`: Short code fragment that adds 26 to a0, then copies the memory at a0 to d0, then compares d0 with 255, then returns.
- `update_racing_position_2`: Processes coordinate values and timing data to update racing position information, then calls another routine to handle the updated data.
- `calculate_coordinate_values`: Calculates coordinate values using a lookup table and performs arithmetic operations on the results before calling another routine to process the final values.
- `frag_clr_bsr_move_rts`: Short code fragment that clears d0, then clears a fixed memory address, then calls another routine, then copies 24 to d0, then copies 1 to a fixed memory address, then calls another routine (2 times in a row), then returns.
- `process_data_loop_2`: Reads a value from memory, performs arithmetic operations on it, then executes a loop that processes data multiple times before returning.
- `frag_move_add_andi_bsr_rts`: Short code fragment that copies a fixed memory address to d1, then adds d0 to d1, then copies a fixed memory address to d0, then masks d0 with 1, then adds d0 to d1, then masks d1 with 65534, then copies d1 to a fixed memory address, then copies a fixed memory address to a fixed memory address, then calls another routine, then returns.
- `calculate_racing_coordinates`: Performs arithmetic operations on coordinate values and timing data, then calls another routine to update racing position information.
- `initialize_and_update_flags`: Initializes several global variables to zero or specific values, performs arithmetic calculations using lookup tables, and updates flags based on comparison results.
- `compare_and_store_flag`: Compares input values with specific constants, performs conditional operations based on comparison outcomes, and stores flags in memory locations based on the results.
- `compute_and_store_differences`: Computes differences between values from memory locations, stores results with conditional adjustments based on comparison outcomes, and handles overflow conditions.
- `compute_arithmetic_result`: Processes byte values through multiple arithmetic operations including bit manipulation and conditional adjustments, then stores the result in a memory location.
- `compute_signed_arithmetic`: Performs signed arithmetic operations including bit manipulation, conditional adjustments, and remainder calculations to produce a final byte result based on input values.
- `frag_and_bpl_subi_neg_rts`: Short code fragment that masks d0 with d0, then branches if the last result was not negative, then subtracts 144 from d0, then negates d0, then returns.
- `calculate_division_remainder`: Performs division of byte values with remainder calculation, applies conditional logic to determine return value based on comparison results.
- `manage_memory_blocks`: Updates memory block counters, processes data from multiple locations, and manages state transitions for memory management operations including conditional updates and loop iterations.
- `find_minimum_value`: Iterates through memory locations to find minimum value, updates a counter when conditions are met, and stores a fixed value at calculated offset.
- `search_and_update_index`: Searches backwards through memory for a matching byte value, updates an index when found, and returns either the updated index or a default value.
- `frag_movem_clr_move_cmp_blt_bra`: Short code fragment that saves registers on the stack, then clears a fixed memory address, then copies d0 to d6, then compares d6 with 100, then branches if the comparison was less, then copies 99 to d6, then jumps to another point in the code.
- `frag_movem_clr_move_cmp_blt_bra_2`: Short code fragment that saves registers on the stack, then clears a fixed memory address, then copies d0 to d6, then compares d6 with 1000, then branches if the comparison was less, then copies 999 to d6, then jumps to another point in the code.
- `frag_movem_clr_move_cmp_blt_bra_3`: Short code fragment that saves registers on the stack, then clears a fixed memory address, then copies d0 to d6, then compares d6 with 10000, then branches if the comparison was less, then copies 9999 to d6, then jumps to another point in the code.
- `execute_multiple_operations`: Executes a series of operations with different parameters, calling another routine multiple times with varying input values before returning.
- `update_tile_state`: Updates tile coordinate values based on comparison results, sets a flag in memory when certain conditions are met, and calls another routine to process tile data.
- `update_tile_coordinates`: Calculates and updates tile data coordinates using a lookup table, then stores the result in a global variable and adds it to another global variable.
- `handle_input_lookup`: Processes input values through a lookup table, performs arithmetic operations on the results, and updates global variables with calculated values.
- `frag_move_bsr_rts`: Short code fragment that copies 0 to a fixed memory address (2 times in a row), then copies a fixed memory address to d0, then copies a fixed memory address to a fixed memory address, then copies 1 to a fixed memory address, then copies 32 to a fixed memory address, then copies 6 to a fixed memory address, then calls another routine, then copies a fixed memory address to a fixed memory address, then returns.
- `update_weighted_average`: Calculates weighted average of values with conditional bounds checking, performs multiplication and division operations, then stores result to memory.
- `process_character_counter`: Increments a value in a table and updates a counter, then calls another routine to process character data when conditions are met.
- `frag_lea_bsr_bcs_subq`: Short code fragment that loads the address of a fixed memory address into a0, then calls another routine, then branches if the carry flag is set, then subtracts 2 from a0.
- `frag_addq_cmpi_blt_move_subq_rts`: Short code fragment that adds 1 to the memory at a0, then compares the memory at a0 with 58, then branches if the comparison was less, then copies 48 to the memory at a0, then subtracts 1 from a0, then adds 1 to the memory at a0, then compares the memory at a0 with 54, then branches if the comparison was less, then copies 48 to the memory at a0, then returns.
- `find_maximum_and_store`: Finds maximum byte value in a sequence while tracking position, updates memory with termination marker at calculated offset.
- `process_coordinate_data`: Iterates through coordinate data, applies arithmetic operations to each value, and stores the results in memory locations based on a lookup table.
- `frag_move_bsr_rts_2`: Short code fragment that copies d2 to a fixed memory address, then copies d3 to a fixed memory address, then calls another routine, then returns.
- `process_byte_with_lookup`: Processes a byte value through a lookup table using multiplication and bit shifting operations, then stores the result back to a register.
- `frag_move_mulu_addi_rts`: Short code fragment that copies a fixed memory address to d4, then multiplies d4 by 41 (unsigned), then adds 17 to d4, then copies d4 to a fixed memory address, then returns.
- `copy_and_sort_data`: Copies sequential byte values from one memory location to another, then processes the copied data through another routine to find maximum values.
- `frag_move_clr_bsr_addq_rts`: Short code fragment that copies 42 to a fixed memory address, then clears a fixed memory address, then calls another routine, then adds 1 to a fixed memory address, then returns.
- `handle_joystick_input`: Reads joystick state from hardware registers, applies bitwise operations to detect movement, and updates input flags and counters accordingly.
- `frag_move_and_bne_beq_rts`: Short code fragment that copies a fixed memory address to d0, then masks d0 with d0, then branches if the last result was not zero, then copies a fixed memory address to d0, then masks d0 with d0, then branches if the last result was zero, then returns.
- `frag_move_bsr_addq_rts`: Short code fragment that copies a fixed memory address to d0, then copies 8 to d1, then calls another routine, then copies the memory at a0 to d0, then adds 1 to d0, then copies d0 to a fixed memory address, then returns.
- `handle_input_sequence_3`: Saves input bytes from a buffer to global variables, modifies those bytes in the buffer, and then restores the original values.
- `initialize_and_process_data`: Initializes data structures with values from memory, performs conditional processing based on bit tests, and writes structured data to memory locations.
- `initialize_rendering_loop`: Initializes rendering parameters by setting memory addresses to specific routine pointers, clears a global counter, and performs a loop that updates tile coordinates and calls other routines multiple times before returning.
- `handle_input_processing`: Processes input values through arithmetic operations and lookup tables, updates global variables with calculated results, then calls multiple routines to handle the processed data.
- `indirect_jump_stub`: Jump stub: passes control to an address held in or computed from register a1, so the target is only known at run time.
- `frag_bsr_move_andi_cmp_bcc_rts`: Short code fragment that calls another routine, then copies d4 to d0, then masks d0 with 127, then compares d0 with 50, then branches if the carry flag is clear, then returns.
- `validate_input_range`: Checks if a word value from a global table equals 59, and if not, calls another routine and performs byte comparisons with a global variable.
- `frag_lea_lsl_movea_adda_rts`: Short code fragment that loads the address of a fixed memory address into a0, then shifts d0 left by 2, then copies an indexed entry from a0 to a0, then adds d1 to a0, then returns.
- `frag_andi_lsl_lea_adda_rts`: Short code fragment that masks d0 with 255, then shifts d0 left by 4, then loads the address of a fixed memory address into a0, then adds d0 to a0, then returns.
- `frag_bsr_addq_bra`: Short code fragment that calls another routine, then adds 2 to a0, then jumps to another point in the code.
- `frag_clr_move_lsl_lea_adda_movea_bsr_rt`: Short code fragment that clears d0, then copies a fixed memory address to d0, then shifts d0 left by 2, then loads the address of a fixed memory address into a0, then adds d0 to a0, then copies the memory at a0 to a0, then calls another routine, then returns.
- `update_sequence_counter`: Updates a sequence counter with bounds checking, stores a value in a global variable, and calls another routine to process data.
- `sort_and_compare_data`: Initializes a table with sequential byte values, copies data between memory locations, then compares bytes at specific offsets and swaps them when necessary.
- `update_coordinate_data`: Updates coordinate values in memory by adding specific constants, performs conditional checks on a counter value, and calls another routine to process the updated coordinates.
