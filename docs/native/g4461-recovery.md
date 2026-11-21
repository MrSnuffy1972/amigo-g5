# Game 4461 (mostares 2 (19xx)(vector games)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04461). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 16 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 157 of 157 named (0 author symbols), 157 of 157 described (48 by rule, 92 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 231 of 231 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4478 of 4478 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 564911 |
| code | 19596 |
| data | 308038 |
| padding | 3967 |
| unused | 4608 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| run | 2 | 759 | 12 |
| cd | 2 | 494 | 12 |
| startup | 1 | 1064 | 76 |
| part1 | 3 | 634 | 17 |
| part2 | 3 | 1377 | 18 |
| part3 | 3 | 414 | 12 |
| part4 | 3 | 231 | 9 |
| Mostares 2 (19xx)(Vector Games).adf:bootblock | 1 | 18 | 1 |

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `frag_move_bra`: Short code fragment that copies d1 to d0, then jumps to another point in the code.
- `initialize_object_state`: Initializes various fields of a structure including position, velocity, and state flags by reading values from tables and performing arithmetic operations, then calls another routine to update object state.
- `update_object_state_2`: Reads a byte from a table using a value from a structure as index, stores it in the structure, then updates several fields including position, velocity, and flags.
- `update_object_state`: Updates object state data including position, velocity, and flags stored at offsets within a data structure referenced by a1.
- `update_object_data`: Updates object state data including position, velocity, and flags stored at offsets within a data structure referenced by a1.
- `frag_moveq_add_bra`: Short code fragment that loads 1 into d1, then adds the memory at an offset from a1 to d1, then jumps to another point in the code.
- `frag_tst_bne_bra`: Short code fragment that tests the memory at an offset from a1, then branches if the last result was not zero, then jumps to another point in the code.
- `frag_move_and_tst_beq_ori_moveq_cmp_blt`: Short code fragment that copies 128 to d1, then masks d1 with the memory at an offset from a1, then tests d1, then branches if the last result was zero, then merges an address into the memory at an offset from a1, then loads 133 into d1, then compares d1 with the memory at an offset from a1, then branches if the comparison was less, then jumps to another point in the code.
- `update_and_branch`: Updates a counter field in a structure and conditionally branches based on its value, otherwise continues execution.
- `initialize_and_process_3`: Initializes several data fields in a structure pointed to by a1, performs conditional operations based on field values, and updates counters.
- `initialize_and_process`: Initializes several data fields in a structure pointed to by a1, performs conditional operations based on field values, and updates counters in a memory table.

### cd

- `find_max_and_prepare_buffer_2`: Walks a linked list to find the maximum value, writes a header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `frag_move_bra_2`: Short code fragment that copies d1 to d0, then jumps to another point in the code.
- `update_collision_flags`: Updates collision detection flags and performs several initialization operations on global variables.
- `initialize_and_process_2_2`: Initializes multiple global variables with values from memory locations, performs several subroutine calls with fixed parameters, and updates a counter variable.
- `initialize_and_process_2`: Initializes multiple global variables with values from memory locations, performs several subroutine calls with fixed parameters, and updates a counter in a data structure before jumping to another routine.
- `decrement_and_compare`: Decrements a value at offset 8 of a1, compares it with zero, and branches to another routine if the comparison is not equal.
- `load_and_store_coordinates`: Loads data from memory locations relative to a1 and a0 registers, performs bit shifts and arithmetic operations on the loaded values, and stores results back to specific offsets in a1.
- `copy_and_process_data`: Copies data from memory locations relative to a1 and a2 registers, performs several arithmetic operations on the copied values, and stores results back to specific offsets in a1.
- `frag_move_moveq_movea_jsr_tst_bne_jmp`: Short code fragment that copies the memory at an offset from a1 to d1, then loads 48 into d0, then copies the memory at an offset from a2 to a4, then calls the routine whose address is in a5, then tests the memory at an offset from a1, then branches if the last result was not zero, then loads 0 into d1, then jumps to the address in a6.
- `calculate_and_branch_conditionally`: Performs calculations involving data from a1 and a4 registers, compares the results with values read from a buffer, and branches based on the comparison.
- `update_and_compare_values`: Updates a counter at offset 32 of a1, reads a byte from a buffer using an index, compares it with a constant value, and branches based on the comparison result.
- `frag_move_lsl_moveq_add_jmp`: Short code fragment that copies the memory at an offset from a1 to d1, then shifts d1 left by 2, then loads 0 into d2, then copies an indexed entry from a0 to d2, then adds the memory at an offset from a1 to d2, then copies the memory at an offset from a1 to d3, then shifts d3 left by 2, then copies d2 to an indexed entry from a0, then loads 255 into d1, then jumps to the address in a6.

### startup

- `open_game_libraries`: Opens three system libraries using global variables as parameters, stores their handles in global memory locations, and proceeds with game resource initialization if successful.
- `initialize_game_resources`: Allocates memory for game resources, opens and reads a file into the allocated buffer, then sets up pointers to a routine and a data location.
- `frag_movea_move_moveq_jsr_bra_jmp`: Short code fragment that copies a fixed memory address to a6, then copies 906 to d1, then loads 0 into d2, then loads 0 into d3, then calls another routine, then jumps to another point in the code, then copies 888 to a fixed memory address, then copies 338 to a fixed memory address, then jumps to another point in the code.
- `setup_and_execute`: Loads a memory address into a6 register, copies a fixed value to memory at an offset, then calls another routine twice before copying additional addresses to registers.
- `frag_lea_move_movea_clr_rts`: Short code fragment that loads the address of a fixed memory address into a5, then copies 416 to the memory at an offset from a5, then copies a fixed memory address to a1, then copies the memory at an offset from a1 to the memory at an offset from a5, then clears the memory at an offset from a5, then copies 33776 to the memory at an offset from a5, then returns.
- `initialize_game_system`: Initializes game resources by locking a directory, allocating memory, opening and reading a file into the allocated buffer, then setting up hardware registers and calling initialization routines before jumping to the main execution loop.
- `frag_jsr_movea_move_rts`: Short code fragment that calls another routine (2 times in a row), then copies a fixed memory address to a6, then copies a fixed memory address to a1, then copies a fixed memory address to d0, then calls another routine, then returns.
- `initialize_input_system`: Initializes input handling by opening a resource, configuring hardware registers based on library version checks, and setting up global variables for input state management.
- `clear_input_flag`: Clears a global variable and modifies bits in structures based on conditional tests, then calls a library routine.
- `process_division_output`: Performs division operations and stores byte values at specific offsets in memory, with conditional logic determining the storage locations.
- `frag_move_lsr_rts`: Short code fragment that copies d2 to the memory at an offset from a4, then shifts d2 right by 8, then copies d2 to the memory at an offset from a4, then returns.
- `setup_and_initialize`: Sets up memory pointers, copies data from one location to another, initializes an array of structures, and configures hardware registers including joystick status and audio channels.
- `frag_sf_lea_clr_move_rts`: Short code fragment that clears a fixed memory address, then loads the address of a fixed memory address into a0, then clears the memory at an offset from a0 (4 times in a row), then copies 15 to a fixed memory address, then returns.
- `check_audio_state`: Reads a global variable, increments it, compares it with another global variable's value, and branches based on the comparison result.
- `frag_bsr_bra`: Short code fragment that calls another routine, then jumps to another point in the code.
- `configure_audio_channels`: Configures audio channel registers based on input values and performs conditional operations on global variables, then returns a modified value.
- `update_audio_registers`: Updates audio hardware registers with values from lookup tables, clears a global counter, and performs conditional updates to a frequency value based on byte comparisons.
- `initialize_object`: Initializes object data using lookup table values based on input bytes, updates position and velocity fields, and sets up related game state variables.
- `update_game_state_2`: Updates internal game state based on input data, modifies DMA and audio registers, and handles specific input byte values to control game behavior.
- `frag_bsr_bra_2`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_3`: Short code fragment that calls another routine, then jumps to another point in the code.
- `update_input_state`: Reads input data from a structure, updates internal state based on input values, and configures DMA and audio registers accordingly.
- `update_audio_parameters`: Updates audio hardware registers with values from a table, modifies a global counter, and performs conditional logic based on byte values.
- `process_input_value`: Reads a byte value from a structure, masks its upper four bits, and performs conditional operations based on the masked value.
- `frag_move_rts`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a5, then returns.
- `calculate_and_compare`: Loads a byte value from memory, divides it by three using word division, swaps the result bytes, and compares the quotient against specific values to determine branch targets.
- `frag_moveq_move_and_bra`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_move_bra_3`: Short code fragment that copies the memory at an offset from a6 to d2, then jumps to another point in the code.
- `lookup_and_compare`: Calculates an address offset based on input parameters, performs a lookup in a table, compares values, and stores the result if the comparison succeeds.
- `frag_move_rts_2`: Short code fragment that copies d2 to the memory at an offset from a5, then returns.
- `adjust_and_limit_value`: Tests a memory byte for zero, sets another byte to fifteen if zero, masks a register's lower four bits with the same byte, and adjusts a word value based on comparison limits.
- `increment_and_limit_value`: Tests a memory byte for zero, sets another byte to fifteen if zero, masks a register's lower four bits with the same byte, and adjusts a word value based on comparison limits in the opposite direction.
- `update_input_value`: Processes a input value from a structure, performs boundary checks and conditional operations based on the value, then stores the result in a global variable at offset 24 of the structure.
- `frag_clr_rts`: Short code fragment that clears the memory at an offset from a6, then returns.
- `frag_move_beq_clr`: Short code fragment that copies the memory at an offset from a6 to d0, then branches if the last result was zero, then copies d0 to the memory at an offset from a6, then clears the memory at an offset from a6.
- `adjust_and_clamp`: Adjusts a value in a structure by adding or subtracting another value based on a condition, performs comparison and clamping operations, and updates the structure's value at offset 16.
- `subtract_and_clamp_offset`: Subtracts the low word of d0 from the word at offset 16 of the structure in a6, and if that value is at or below the word at offset 24 it copies offset 24 into offset 16 and clears offset 24.
- `combine_and_store_bits`: Processes byte values from memory locations, applies bit shifts and masks, performs conditional operations based on comparison results, and stores combined bit values back to memory.
- `calculate_and_shift`: Processes byte values from memory locations, applies bit shifts and masks, performs conditional operations based on comparison results, and accumulates calculated values into another memory location.
- `frag_tst_bpl_move_sub_bra`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was not negative, then copies 255 to d2, then subtracts d0 from d2, then jumps to another point in the code.
- `frag_move_bra_4`: Short code fragment that copies d0 to d2, then jumps to another point in the code.
- `frag_move_and_mulu_lsr_tst_bmi_add_bra`: Short code fragment that copies an indexed entry from a4 to d2, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then multiplies d2 by d0 (unsigned), then shifts d2 right by 7, then copies the memory at an offset from a6 to d0, then tests the memory at an offset from a6, then branches if the last result was negative, then adds d2 to d0, then jumps to another point in the code.
- `frag_sub_move_lsr_and_add_rts`: Short code fragment that subtracts d2 from d0, then copies d0 to the memory at an offset from a5, then copies the memory at an offset from a6 to d0, then shifts d0 right by 2, then masks d0 with 60, then adds d0 to the memory at an offset from a6, then returns.
- `frag_bsr_bra_4`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_5`: Short code fragment that calls another routine, then jumps to another point in the code.
- `update_position_and_velocity`: Updates position and velocity values based on input data and table lookups, then stores the result in a global variable.
- `frag_tst_bpl_move_sub_bra_2`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was not negative, then copies 255 to d2, then subtracts d0 from d2, then jumps to another point in the code.
- `frag_move_bra_5`: Short code fragment that copies d0 to d2, then jumps to another point in the code.
- `accumulate_and_clamp`: Loads a byte value from memory, multiplies it by a masked register value, shifts the result right by six bits, adds the result to another memory value, and branches based on sign comparison.
- `clamp_and_accumulate`: Calculates the difference between two values, clamps it to a maximum limit, stores the result, and accumulates a shifted byte value into another memory location.
- `conditional_update_and_accumulate`: Loads a byte value from memory, performs conditional operations based on its value, updates another memory location with the loaded value, and accumulates calculated results into a global variable.
- `frag_move_rts_3`: Short code fragment that copies 1 to the memory at an offset from a6, then returns.
- `accumulate_and_limit`: Loads a byte value from memory, applies bit shifts and masks, performs conditional accumulation, and limits the accumulated value to a maximum threshold before storing it.
- `frag_moveq_move_and_sub_bpl_clr_rts`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then subtracts d0 from the memory at an offset from a6, then branches if the last result was not negative, then clears the memory at an offset from a6, then copies the memory at an offset from a6 to d0, then copies d0 to the memory at an offset from a5, then returns.
- `decrement_and_store_byte`: Reads a byte from offset 3 of the structure in a6, subtracts one from it, and stores the result in a global byte, then clears two subsequent global bytes.
- `clamp_byte_and_store`: Reads a byte from offset 3 of the structure in a6, clamps it to a maximum of sixty-four, stores the clamped value as a byte at offset 19 of that structure, and stores it as a word at offset 8 of a structure in a5.
- `calculate_and_store`: Processes byte values through multiplication and bit manipulation operations, compares the result against a threshold, and stores the final calculated value in global memory locations.
- `frag_moveq_move_beq_cmp_bcc_clr_rts`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then branches if the last result was zero, then compares d0 with 32, then branches if the carry flag is clear, then clears a fixed memory address, then copies d0 to a fixed memory address, then returns.
- `dispatch_input_byte`: Reads a byte from offset 2 of the structure in a6, extracts its lower nibble, and branches to different handling routines based on the nibble value.
- `process_input_byte`: Reads a byte from offset 3 of a structure, extracts its upper nibble, and uses that value as an index to determine which operation to perform on the structure's data.
- `frag_move_and_asl_andi_or_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then masks d0 with 1, then shifts d0 left by 1, then masks a fixed memory address with 253, then merges d0 into a fixed memory address, then returns.
- `replace_low_nibble_at_offset`: Clears the low nibble of the byte at offset 31 of a6 and replaces it with the low nibble of the byte at offset 3 of a6.
- `replace_low_nibble_at_offset_2`: Clears the low nibble of the byte at offset 30 of a6 and replaces it with the low nibble of the byte at offset 3 of a6.
- `store_low_nibble_at_offset`: Stores the low nibble of the byte at offset 3 of a6 into the byte at offset 18 of the same structure, then returns.
- `decrement_and_update`: Tests a memory byte for zero, sets another byte to fifteen if zero, masks a register's lower four bits with the same byte, decrements a counter, and updates memory locations based on conditional checks.
- `frag_move_bra_6`: Short code fragment that copies d0 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_move_lsr_rts_2`: Short code fragment that copies a location in the code to d0, then shifts d0 right by 4, then copies d0 to the memory at an offset from a6, then returns.
- `frag_move_and_lsl_andi_or_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then masks d0 with 15, then shifts d0 left by 4, then masks the memory at an offset from a6 with 15, then merges d0 into the memory at an offset from a6, then returns.
- `conditionally_write_dma_control`: Tests the low nibble of the byte at offset 3 of the structure in a6 and a global variable modulo that nibble; when the checks pass it writes the DMA control register from offset 20, delays with two countdown loops of 300 iterations, sets bit 15 of the value and writes it again, then copies longwords from offsets 4 and 14 into the structure pointed to by a5.
- `frag_tst_bne_moveq_move_and_bra`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_tst_bne_moveq_move_and_bra_2`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_moveq_move_and_cmp_bne_clr_rts`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then compares d0 with a location in the code, then branches if the last result was not zero, then clears the memory at an offset from a6, then copies 0 to the memory at an offset from a5, then returns.
- `frag_moveq_move_and_cmp_bne_beq_bra`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then compares d0 with a fixed memory address, then branches if the last result was not zero, then copies the memory at a6 to d0, then branches if the last result was zero, then copies d1 to the stack, then jumps to another point in the code.
- `frag_tst_bne_moveq_move_and_addq_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then tests a fixed memory address, then branches if the last result was not zero, then adds 1 to d0, then copies d0 to a fixed memory address, then returns.
- `frag_tst_bne_move_and_lsl_andi_or_beq`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then shifts d0 left by 4, then masks the memory at an offset from a6 with 15, then merges d0 into the memory at an offset from a6, then tests d0, then branches if the last result was zero.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.

### part1

- `initialize_video_system`: Initializes video hardware registers including color values and display control settings, then performs a series of loops to update these registers with incremental values while checking joystick input status.
- `initialize_and_copy_coordinates`: Initializes various hardware registers with predefined values, copies coordinate data from memory locations to an output buffer, and sets up multiple global variables for rendering operations.
- `process_and_combine_bytes`: Processes three consecutive bytes from a source buffer, multiplies them by a scaling factor, applies bit masking and shifting operations, combines the results into words, and stores them in an output array.
- `normalize_vector`: Processes vector coordinates by clamping values and performing linear interpolation, then calculates and stores normalized vector components and related parameters.
- `update_vector_coordinates_2`: Processes vector coordinate data by clamping values, performing linear interpolation, and storing normalized components.
- `frag_btst_bne_move_rts`: Short code fragment that tests bit 14 of the memory at an offset from a5, then branches if the last result was not zero, then copies 0 to the memory at an offset from a5, then copies an address to the memory at an offset from a5, then copies a fixed memory address to the memory at an offset from a5, then copies 25 to the memory at an offset from a5, then returns.
- `frag_btst_bne_move`: Short code fragment that tests bit 14 of the memory at an offset from a5, then branches if the last result was not zero, then copies 0 to the memory at an offset from a5, then copies an address to the memory at an offset from a5, then copies a fixed memory address to the memory at an offset from a5, then copies 49172 to the memory at an offset from a5.
- `frag_btst_bne_rts`: Short code fragment that tests bit 14 of the memory at an offset from a5, then branches if the last result was not zero, then returns.
- `swap_and_store_vectors`: Swaps three longword values between memory locations and stores word values at specific offsets in a structure.
- `copy_encoded_data_block`: Copies encoded byte data from one buffer to another, handling both positive and negative count values with different processing paths for each case.
- `copy_short_encoded_data`: Copies short encoded byte data from one buffer to another, processing both positive and negative count values with distinct code paths for each scenario.
- `initialize_and_process_vectors`: Initializes vector processing parameters and iteratively updates coordinate values while decrementing a counter until it becomes negative, then clears a global flag.
- `setup_and_iterate_coordinates`: Sets up initial coordinate values and repeatedly calls other routines to process vector data while adjusting coordinate values in opposite directions until a condition is met, then clears a global flag.
- `update_and_loop_counter`: Updates a word value in memory through a loop, performs conditional checks on the counter, and sets specific status flags based on the counter's value.
- `frag_move_dbra_cmp_bne_sub_bpl_rts`: Short code fragment that copies 4095 to d0, then copies d0 to the memory at an offset from a5, then copies 40960 to d1, then loops back while counting down d1, then compares d0 with 1638, then branches if the last result was not zero, then copies 0 to the memory at an offset from a5, then subtracts 273 from d0, then branches if the last result was not negative, then returns.
- `frag_move_tst_bpl_dbra_rts`: Short code fragment that copies 8 to d2, then copies 20479 to d1, then tests a fixed memory address, then branches if the last result was not negative, then loops back while counting down d1, then loops back while counting down d2, then returns.
- `process_vector_data`: Processes vector coordinates by reading from a table, applying offsets, clamping values, performing fixed-point multiplication, and storing updated coordinates back to the table.

### part2

- `initialize_game_memory`: Allocates memory for game graphics and initializes multiple global variables with calculated offsets.
- `initialize_graphics_parameters`: Initializes various graphics configuration parameters including window settings, color values, and coordinate data, then calls another routine and sets a final status flag.
- `update_vector_coordinates`: Processes vector coordinate data by calculating new positions based on input values and storing results in a global variable, while updating counters and handling boundary conditions.
- `process_line_segment_2`: Calculates line segment coordinates and stores computed values including slope-related data, position offsets, and bit flags in global variables.
- `process_line_segments_2`: Processes line segment coordinates by calculating slope-related data, position offsets, and bit flags, then stores the results in global variables.
- `process_line_segment`: Calculates line segment coordinates and stores computed values including slope-related data, position offsets, and bit flags in global variables.
- `initialize_game_state`: Clears a counter in a global variable, sets a specific value at offset 64 of the structure in a5, copies a constant value to offset 84 of the structure, and stores a word value at offset 88 of the structure.
- `swap_and_copy_coordinates`: Swaps three longword values in a global table, copies the first value to a global variable, and updates several fields in a structure with specific values including a counter, a longword, and a word.
- `frag_btst_bne_rts_2`: Short code fragment that tests bit 14 of the memory at an offset from a5, then branches if the last result was not zero, then returns.
- `frag_btst_beq_rts`: Short code fragment that tests bit 4 of the memory at an offset from a5, then branches if the last result was zero, then returns.
- `frag_jsr`: Short code fragment that calls another routine.
- `initialize_graphics_state`: Initializes multiple graphics-related global variables with specific values, sets up data buffers, and configures display parameters including color settings, screen coordinates, and timing constants.
- `read_joystick_input`: Reads joystick movement and fire button data from hardware registers, performs bit manipulation on the input to generate a byte value representing the joystick state, and returns this value.
- `copy_and_decompress_data`: Copies bytes from one memory location to another, handles both positive and negative count values for copying, and manages buffer boundaries during the copy operation.
- `process_line_segments`: Processes line segment coordinates by calculating slope-related data, position offsets, and bit flags, then stores the results in global variables.
- `frag_lea_moveq_move_add_cmp_bcs_rts`: Short code fragment that loads the address of a fixed memory address into a1, then loads 0 into d1, then copies an indexed entry from a1 to an indexed entry from a0, then adds 40 to d0, then adds 80 to d1, then compares d1 with 640, then branches if the carry flag is set, then loads the address of the memory at an offset from a0 into a0, then returns.
- `initialize_and_process_data`: Initializes several word-sized values at specific offsets in a local data structure, then processes a sequence of bytes from a buffer, copying data from a table to another location based on the byte values, and includes a delay loop.
- `update_game_state`: Processes joystick input to update game object coordinates and orientation, then stores computed values including slope-related data, position offsets, and bit flags in global variables.

### part3

- `initialize_video_memory`: Allocates memory for video rendering, sets up display parameters including color registers and bitplane control, and initializes vector drawing state variables.
- `initialize_rendering_parameters`: Calls another routine, sets multiple word values at specific offsets of a structure, copies longword values to a global variable, and stores a final longword value in the structure.
- `update_vector_counters`: Increments and modifies several global variables used for tracking vector rendering state, then returns.
- `frag_move_rts_4`: Short code fragment that copies 65466 to a fixed memory address, then copies 582 to a fixed memory address, then copies 2048 to a fixed memory address, then copies 0 to a fixed memory address, then returns.
- `scale_and_pack_rgb_values`: Reads three bytes from a table, multiplies them by a scale factor, applies bit masking and shifting operations to each component, combines the results into a single word value, and stores this value in a global array.
- `normalize_vector_2`: Processes vector coordinates by clamping values, performing linear interpolation, and calculating normalized direction components.
- `process_vector_coordinates`: Processes vector coordinates by reading from a table, applying transformations including multiplication and bit shifting, clamping results to a range, and storing updated values back to the table.
- `initialize_globals`: Waits for a specific bit in memory to clear, then initializes several global variables with hardcoded values and sets a flag bit.
- `swap_and_copy_values`: Swaps three 32-bit values in memory, copies the first value to a buffer, and updates multiple global variables with specific constants.
- `frag_btst_bne_rts_3`: Short code fragment that tests bit 14 of the memory at an offset from a5, then branches if the last result was not zero, then returns.
- `copy_compressed_data`: Copies compressed data from one buffer to another using a run-length encoding scheme with negative length markers.
- `process_memory_sequence`: Processes a sequence of memory operations including comparisons, value lookups, and bit manipulation while updating various registers and memory locations.

### part4

- `initialize_graphics_system`: Allocates memory for graphics buffers, initializes display parameters including color registers and bitplane control, sets up rendering coordinates, and configures hardware registers for video output.
- `setup_render_parameters`: Initializes rendering parameters by copying data from external tables, setting up coordinate values, and configuring various global variables with specific constants.
- `update_graphics_buffer`: Calculates screen coordinates using scaled values from global variables, performs bit manipulation to determine a pixel position, and sets a bit in a graphics buffer.
- `calculate_coordinates`: Calculates two 16-bit coordinates by extracting and modifying bits from a global value, then stores the results in memory locations.
- `scale_and_combine_bytes`: Scales three bytes from a source buffer using a multiplier, combines the scaled values into a single 16-bit word, and stores the result in memory.
- `initialize_globals_copy`: Waits for a specific bit in memory to clear, then initializes several global variables with hardcoded values and sets a flag bit, identical to routine R1.
- `swap_and_copy_values_copy`: Swaps three 32-bit values in memory, copies the first value to a buffer, and updates multiple global variables with specific constants, identical to routine R2.
- `frag_btst_bne_rts_4`: Short code fragment that tests bit 14 of the memory at an offset from a5, then branches if the last result was not zero, then returns.
- `copy_compressed_data_copy`: Copies compressed data from one buffer to another using a run-length encoding scheme with negative length markers, identical to routine R3.

### Mostares 2 (19xx)(Vector Games).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
