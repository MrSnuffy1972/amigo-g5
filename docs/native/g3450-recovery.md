# Game 3450 (DragonTiles v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03450). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (704462 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 126 of 126 named (0 author symbols), 126 of 126 described (39 by rule, 69 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2403 of 2403 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 278521 |
| code | 10216 |
| data | 415713 |
| padding | 12 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| DragonTiles | 4 | 2401 | 125 |
| game.slave | 1 | 2 | 1 |

## Cases handled

- DragonTiles_v1.0/DragonTiles_v1.0.hdf: hard-disk image (AmigaDOS OFS, 768 blocks): its filesystem parses and all 9 files in it are byte-identical to the extracted tree DragonTiles_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### DragonTiles

- `check_startup_message`: Initializes a global variable to zero, retrieves the current task pointer, and checks if the task has a CLI. If not, it waits for and gets a message from the task's port, then calls another routine before optionally replying to the received message.
- `wait_for_fire_button`: Waits for the fire button to be pressed and released, then checks a global variable and clears it before returning.
- `setup_blitter_parameters_2`: Configures blitter parameters by setting up multiple hardware registers with specific values, then calls another routine to process these settings for graphics operations.
- `wait_for_input`: Waits for a specific hardware input bit to clear, then waits for it to set, performs a delay loop, and checks two global variables before returning.
- `process_coordinates`: Processes coordinate values by dividing them with specific constants, performs arithmetic operations, and stores the result in global variables.
- `setup_and_process_data`: Sets up loop counters and calls multiple routines to process data, performs iterative operations with incrementing values, and returns after completing all iterations.
- `process_tile_data`: Processes tile data by loading bytes from a buffer, performing bit shifts and calculations, and configuring blitter parameters for each tile in a grid layout.
- `unpack_nibbles`: Extracts high and low nibbles from a sequence of bytes and stores them as separate bytes in another location.
- `initialize_game_state`: Initializes multiple global variables with specific values, sets up blitter parameters, and configures coordinate data for rendering.
- `configure_blitter_loop`: Sets up blitter parameters in a loop, processes data from a buffer by shifting bits and storing values, then calls another routine to execute the blitter configuration.
- `initialize_video_system`: Initializes video registers by writing specific values to multiple hardware registers.
- `read_joystick_input`: Checks joystick button states and updates a global variable with a value based on the input state, then waits for specific button conditions to be met.
- `read_mouse_input`: Checks mouse button states and updates a global variable with a value based on the input state, then waits for specific button conditions to be met.
- `initialize_data_blocks`: Clears several memory locations to zero, copies data from one area to another, and sets specific values in multiple memory locations.
- `configure_video_registers`: Writes various values to video-related registers and performs a delay loop before writing additional values to different registers.
- `clear_structure_data`: Clears multiple memory locations in a structure and resets a flag when a condition is met, then increments a counter in the structure.
- `update_tile_coordinates`: Updates tile position coordinates in global variables based on comparison results from input values, and performs conditional blitter configuration setup.
- `setup_sprite_data`: Calls a polling routine, then sets up sprite-related parameters in a structure with specific values for multiple fields.
- `setup_sprite_data_v2`: Calls a polling routine, then sets up sprite-related parameters in a structure with specific values for multiple fields, using different values than the previous routine.
- `copy_game_data`: Copies game data from multiple source locations to a destination buffer and clears the original source locations.
- `calculate_coordinates`: Calculates coordinate values based on input parameters and performs boundary checks before storing results.
- `calculate_byte_offset`: Performs arithmetic operations on input values to calculate an address offset and returns the byte value at that location.
- `clear_byte_at_offset`: Calculates an address based on input parameters and clears the byte at that location to zero.
- `calculate_and_store_byte`: Calculates an address using three input parameters and stores a byte value at that calculated address.
- `initialize_game_tables`: Clears multiple global memory regions and populates data from a lookup table, then performs iterative updates on game state variables based on comparisons with stored values.
- `update_random_table`: Updates a random table entry and stores computed values to multiple memory locations.
- `setup_blitter_config`: Initializes blitter configuration parameters with specific values for hardware registers.
- `configure_blitter_operations`: Sets up multiple blitter operations by configuring register values and calling the blitter setup routine.
- `clear_and_update_counters`: Clears and updates multiple counters in a nested loop structure, writing zero to specific global variables before incrementing them.
- `set_tile_flag`: Reads coordinate values from global variables, performs boundary checks on the coordinates, and writes the value 1 to a memory location when conditions are met.
- `initialize_blitter_parameters`: Calculates and sets up blitter parameters based on input coordinates and performs conditional configuration.
- `frag_movem_movea_move_bsr_tst_bne_jsr_r`: Short code fragment that saves registers on the stack, then copies an address to a6, then copies 16 to the memory at an offset from a6, then copies 0 to a fixed memory address, then calls another routine, then tests a fixed memory address, then branches if the last result was not zero, then calls another routine, then restores saved registers from the stack, then returns from the exception.
- `process_input_buffer`: Copies bytes from an input buffer to a global variable while decrementing a counter, and continues until the counter reaches zero or a specific condition is met.
- `process_input_and_render`: Processes input data, configures rendering parameters, and updates game state based on input conditions.
- `frag_movem_moveq_mulu_add_move_rts`: Short code fragment that saves registers on the stack, then loads 14 into d2, then multiplies d2 by d1 (unsigned), then adds d0 to d2, then loads 0 into d0, then copies an indexed entry from a0 to d0, then copies d0 to a fixed memory address, then restores saved registers from the stack, then returns.
- `frag_movem_moveq_mulu_add_move_rts_2`: Short code fragment that saves registers on the stack, then loads 14 into d3, then multiplies d3 by d1 (unsigned), then adds d0 to d3, then copies d2 to an indexed entry from a0, then restores saved registers from the stack, then returns.
- `frag_bsr_dbra_rts`: Short code fragment that calls another routine, then loops back while counting down d0, then returns.
- `frag_cmpi_bne_rts`: Short code fragment that compares the memory at an offset from a6 with 241, then branches if the last result was not zero, then returns.
- `frag_btst_bne_move_rts`: Short code fragment that tests bit 14 of the memory at an offset from a6 (2 times in a row), then branches if the last result was not zero, then copies a0 to the memory at an offset from a6, then copies 256 to the memory at an offset from a6, then copies 0 to the memory at an offset from a6 (2 times in a row), then copies 41000 to the memory at an offset from a6, then returns.
- `update_random_data`: Reads hardware timer values from CIA and custom registers, performs bitwise operations on them, and stores the results in two memory locations repeatedly.
- `generate_pseudo_random`: Shifts bits through multiple memory locations using logical and arithmetic operations, then multiplies the result by a constant and scales it to a byte value.
- `frag_movem_lea_divu_bsr_rts`: Short code fragment that saves registers on the stack, then loads the address of a fixed memory address into a0, then divides d1 by 100 (unsigned), then calls another routine, then divides d1 by 10 (unsigned), then calls another routine (2 times in a row), then restores saved registers from the stack, then returns.
- `frag_addi_move_clr_swap_rts`: Short code fragment that adds 48 to d1, then copies d1 to the memory at a0 (advancing a0), then clears d1, then swaps the halves of d1, then returns.
- `check_coordinate_range`: Compares two coordinate values against boundary limits and returns a boolean result indicating whether both coordinates fall within the specified range.
- `adjust_mouse_coordinates`: Reads mouse movement data from a hardware register, calculates differences between current and previous coordinates, clamps the adjusted values within specific ranges, and updates global variables with the new coordinates.
- `adjust_coordinates_2`: Adjusts coordinate values by adding offsets and clamping results to specific ranges.
- `adjust_coordinates`: Performs arithmetic operations on register values, updates memory locations with computed results, and branches based on comparison outcomes.
- `frag_cmpi_ble_move_bge_bra`: Short code fragment that compares a fixed memory address with 318, then branches if the comparison was less or equal, then copies 318 to a fixed memory address, then compares a fixed memory address with 0, then branches if the comparison was greater or equal, then copies 0 to a fixed memory address, then jumps to another point in the code.
- `update_boundary_values`: Updates boundary values based on comparison results and performs memory operations with computed values.
- `setup_blitter_parameters`: Calculates memory addresses for blitter operations, sets up various hardware registers with specific values, and configures blitter parameters in a loop.
- `wait_while_flag_bit_set`: Polls a byte in a structure until bit 6 is clear, then returns.
- `compute_shifted_values`: Shifts and rotates register values through multiple operations, combines results into byte values, and returns the final computed value.
- `frag_movem_lea_adda_moveq_bsr_rts`: Short code fragment that saves registers on the stack, then loads the address of a fixed memory address into a1, then adds 95000 to a1, then loads the address of a fixed memory address into a0, then loads 31 into d0, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_move_addq_dbra_rts`: Short code fragment that copies the memory at a1 (advancing a1) to the memory at a0, then adds 4 to a0, then loops back while counting down d0, then returns.
- `initialize_graphics_data`: Writes predefined constant values to various memory locations, performs swap operations on data, and sets up graphics-related parameters.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `frag_movea_move_rts`: Short code fragment that copies an address to a6, then copies 49168 to the memory at an offset from a6, then copies a0 to a fixed memory address, then returns.
- `wait_for_input_release`: Waits for a specific hardware bit to clear, then waits for it to set, reads input values, and performs a delay loop before returning.
- `setup_system_libraries`: Opens system libraries, reads data from library structures, configures interrupt and DMA registers, and stores various hardware register values.
- `restore_system_state`: Restores saved system register values, updates coprocessor addresses, configures interrupt and DMA settings, and releases system permits.
- `initialize_game_data`: Initializes game data structures by copying a buffer pointer to a global variable, then processes a sequence of bytes from that buffer to build a lookup table and set up hardware registers.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `frag_movem_addq_move_cmp_bcs_clr_tst_be`: Short code fragment that saves registers on the stack, then adds 1 to a fixed memory address, then copies a location in the code to d0, then compares d0 with a location in the code, then branches if the carry flag is set, then clears a fixed memory address, then tests a fixed memory address, then branches if the last result was zero, then calls another routine, then jumps to another point in the code.
- `frag_bsr_bra`: Short code fragment that calls another routine, then jumps to another point in the code.
- `update_audio_registers`: Updates audio period registers based on command data and global state, with conditional logic affecting different audio channels.
- `configure_audio_channels`: Clears a global variable, loads indexed data from lookup tables to configure audio DMA settings, writes control values to hardware registers, and updates audio channel pointers and lengths while performing delay loops between operations.
- `update_sprite_data_2`: Loads sprite configuration data from a lookup table based on indexed values, updates position and status fields, and configures DMA settings accordingly.
- `update_sprite_position`: Updates sprite position and status fields based on indexed table lookup, then performs conditional DMA configuration and bit flag clearing.
- `frag_bsr_bra_2`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_3`: Short code fragment that calls another routine, then jumps to another point in the code.
- `update_sprite_data`: Updates sprite position and status fields based on indexed table lookup, then performs conditional DMA configuration and bit flag clearing before returning.
- `loads_audio_channels_and_counters`: Waits in delay loops, writes a DMA control value with the enable bit set, loads pointer and length values into the location and length registers of the four audio channels, adjusts a global counter and a timing word, and loops over a table entry.
- `process_movement_command`: Reads a command byte from a structure, masks it to extract a lower nibble, and branches to different handling routines based on the value.
- `frag_move_rts`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a5, then returns.
- `load_and_compare_division`: Loads a byte from a global variable, divides it by three, swaps the quotient, compares it with zero and two, and branches accordingly.
- `frag_moveq_move_andi_bra`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_move_bra`: Short code fragment that copies the memory at an offset from a6 to d2, then jumps to another point in the code.
- `lookup_and_store_value`: Performs a lookup operation using indexed values from a table, compares a value with the table entry, and stores the result in memory when the condition is met.
- `frag_move_rts_2`: Short code fragment that copies d2 to the memory at an offset from a5, then returns.
- `adjust_and_limit_value`: Reads a byte from a structure, masks it, subtracts it from a word value in another structure, clamps the result to a minimum threshold, and stores the final word value in a table.
- `update_position_range`: Updates a position value in memory by adding a masked byte from a structure, then clamps the result to a maximum threshold and stores the final value.
- `update_movement_data`: Reads a command byte from a structure, masks it to extract a lower nibble, and branches to different handling routines based on the value.
- `frag_clr_rts`: Short code fragment that clears the memory at an offset from a6, then returns.
- `frag_move_beq_clr`: Short code fragment that copies the memory at an offset from a6 to d0, then branches if the last result was zero, then copies d0 to the memory at an offset from a6, then clears the memory at an offset from a6.
- `adjust_movement_speed`: Checks if a movement value is zero and returns early if so.
- `decrement_channel_period_and_lookup`: Subtracts the input value from the channel period and clamps it to the target when reached.
- `combine_and_store_bits`: Combines nibbles from two bytes in a structure using bitwise operations, stores the result back to one of the bytes, and performs additional calculations based on other values.
- `calculate_and_store_offset`: Calculates an offset based on input parameters, performs bit manipulation operations, and stores the final calculated value in a global structure.
- `frag_tst_bpl_move_sub_bra`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was not negative, then copies 255 to d2, then subtracts d0 from d2, then jumps to another point in the code.
- `frag_move_bra_2`: Short code fragment that copies d0 to d2, then jumps to another point in the code.
- `frag_move_andi_mulu_lsr_tst_bmi_add_bra`: Short code fragment that copies an indexed entry from a4 to d2, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then multiplies d2 by d0 (unsigned), then shifts d2 right by 7, then copies the memory at an offset from a6 to d0, then tests the memory at an offset from a6, then branches if the last result was negative, then adds d2 to d0, then jumps to another point in the code.
- `subtract_and_advance_phase`: Subtracts d2 from d0 and stores the result in offset 6 of a structure in a5, then advances a phase byte in the structure by an amount taken from the high nibble of a neighbouring byte.
- `frag_bsr_bra_4`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_5`: Short code fragment that calls another routine, then jumps to another point in the code.
- `process_and_clamp_value`: Processes bit combinations from bytes in a structure, performs arithmetic operations with lookup table values, clamps the result to a range, and stores the final value.
- `frag_tst_bpl_move_sub_bra_2`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was not negative, then copies 255 to d2, then subtracts d0 from d2, then jumps to another point in the code.
- `frag_move_bra_3`: Short code fragment that copies d0 to d2, then jumps to another point in the code.
- `clamp_and_add_value`: Reads a byte from memory, applies bit masking and multiplication operations, clamps the result to a range, and adds it to another value before storing the sum.
- `clamp_and_store_value`: Clamps the signed difference between d2 and d0 to a range between zero and sixty-four, stores the result as a word at offset 8 of a structure in a5, and updates a byte at offset 29 of a structure in a6 by adding the high nibble of the byte at offset 28 shifted right by two.
- `update_and_clamp_counter`: Loads a byte value, performs conditional operations based on its value, updates a counter, and clamps the result to a maximum threshold before storing it.
- `frag_move_rts_3`: Short code fragment that copies 1 to the memory at an offset from a6, then returns.
- `adjust_and_store_position`: Reads a byte value, shifts it right by four bits, adds it to another byte in memory, compares the sum with a threshold, and stores the result in a global variable.
- `decrement_and_clamp_byte`: Subtracts the low nibble of the byte at offset 3 of the structure in a6 from the byte at offset 19, clears that byte to zero when the result is negative, and stores the resulting byte as a word at offset 8 of the structure in a5.
- `decrement_and_store_byte`: Reads a byte from offset 3 of the structure in a6, subtracts one from it, and stores the result in a global byte, then clears two subsequent global bytes.
- `clamp_byte_and_store`: Reads a byte from offset 3 of the structure in a6, clamps it to a maximum of sixty-four, stores the clamped value as a byte at offset 19 of that structure, and stores it as a word at offset 8 of a structure in a5.
- `validate_and_store_index`: Loads a byte value, performs bit manipulation operations, validates the result against a maximum threshold, and stores the validated value in global memory.
- `frag_move_beq_clr_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then branches if the last result was zero, then clears a fixed memory address, then copies d0 to a fixed memory address, then returns.
- `process_instruction`: Reads a byte from a structure, extracts its lower nibble, and uses it as an index to dispatch to different handling routines based on the value.
- `decode_instruction`: Reads a byte from offset 3 of a structure, extracts its upper nibble, and uses it as an index to jump to different handling routines.
- `update_global_from_ciaa_pra`: Masks bit zero of the byte at offset 3 of the structure in a6, shifts it left by one, and ORs the result into a global variable after clearing bit one of that variable.
- `mask_and_merge_byte`: Masks the low nibble of a byte at offset 31 from a pointer in a6 and then merges the low nibble of a byte at offset 3 from the same pointer into it.
- `clear_low_nibbles_of_byte`: Clears the low nibble of a byte at offset 30 from an address in a6 and then ORs in the low nibble of another byte from the same base.
- `store_masked_byte`: The routine reads a byte from an offset in a structure pointed to by a register, masks it with a constant, and stores the result into a different offset in the same structure.
- `check_and_update_flags`: Tests a global flag and branches if it's not zero, then reads from a structure, updates flags in memory, and returns immediately without further processing.
- `frag_move_bra_4`: Short code fragment that copies d0 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_move_lsr_rts`: Short code fragment that copies a location in the code to d0, then shifts d0 right by 4, then copies d0 to the memory at an offset from a6, then returns.
- `mask_and_merge_nibble`: Masks the low nibble of a byte at offset 30 from a pointer in a6 and merges the high nibble from a byte at offset 3 of the same pointer, storing the result back at offset 30.
- `decode_and_copy_title_data`: Reads a nibble from a table entry and, if a global variable and the entry satisfy a modulo condition, copies two 32-bit words and one 16-bit word from the table through a pointer, performs two delay loops, writes a modified value to a global variable, and then copies two more 32-bit words from the table.
- `frag_tst_bne_moveq_move_andi_bra`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_tst_bne_moveq_move_andi_bra_2`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_moveq_move_andi_cmp_bne_clr_rts`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then compares d0 with a location in the code, then branches if the last result was not zero, then clears the memory at an offset from a6, then copies 0 to the memory at an offset from a5, then returns.
- `frag_moveq_move_andi_cmp_bne_beq_bra`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then compares d0 with a fixed memory address, then branches if the last result was not zero, then copies the memory at a6 to d0, then branches if the last result was zero, then copies d1 to the stack, then jumps to another point in the code.
- `frag_tst_bne_moveq_move_andi_addq_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then tests a fixed memory address, then branches if the last result was not zero, then adds 1 to d0, then copies d0 to a fixed memory address, then returns.
- `frag_tst_bne_move_andi_lsl_or_beq`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then shifts d0 left by 4, then masks the memory at an offset from a6 with 15, then merges d0 into the memory at an offset from a6, then tests d0, then branches if the last result was zero.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.

### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
