# Game 4132 (kastle kumquat (1992)(cybernetix)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04132). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (906352 top-level bytes in 10 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 143 of 143 named (0 author symbols), 143 of 143 described (67 by rule, 75 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1320 of 1320 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2715 of 2715 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 348363 |
| code | 11174 |
| data | 58924 |
| padding | 1491 |
| unused | 486400 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Kastle Kumquat (1992)(Cybernetix).adf:Game | 8 | 1841 | 66 |
| Kastle Kumquat (1992)(Cybernetix).adf:bootblock | 1 | 54 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 820 | 76 |

## Cases handled

- 21 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 4 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 immediate, 1 jump target, 1 immediate+address

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Kastle Kumquat (1992)(Cybernetix).adf:Game

- `initialize_game`: Initializes game libraries and rendering data by opening required system libraries, setting up graphics registers, and configuring global variables for display and input handling.
- `initialize_render_data_3`: Initializes rendering data by setting specific global variables to predefined values, clearing memory locations, and invoking multiple subroutines to prepare for rendering operations.
- `swap_and_store_coordinates`: Exchanges two longword values in global memory, then stores parts of the first value as words at various offsets in global memory, followed by swapping and storing the second value's parts at different offsets.
- `process_rendering_loop`: Processes a rendering loop by copying data from global tables to memory locations and updating pointers through a series of conditional branches and iterations.
- `update_rendering_loop`: Updates rendering loop parameters by copying data from global tables to memory locations, performing arithmetic operations, and incrementing counters.
- `frag_divu_bsr`: Short code fragment that divides d0 by 1000 (unsigned), then calls another routine, then divides d0 by 100 (unsigned), then calls another routine, then divides d0 by 10 (unsigned), then calls another routine.
- `frag_addi_move_clr_swap_rts`: Short code fragment that adds 48 to d0, then copies d0 to the memory at a1 (advancing a1), then clears d0, then swaps the halves of d0, then returns.
- `execute_render_pattern`: Executes a static pattern rendering loop by calling another routine with specific register values and pointer addresses, then updating a counter in d1 until it reaches 40.
- `increment_and_reset_counter`: Checks a word value in global memory, clears a byte if zero, otherwise increments a counter in a structure and updates another counter after reaching a threshold.
- `update_animation_state`: Updates animation state by decrementing a global counter, copying data from memory locations, and modifying coordinate values based on comparison results with predefined thresholds.
- `copy_and_transform_data`: Calculates an address based on coordinates and a lookup table, then copies bytes from one memory location to another using a loop with incrementing offsets.
- `copy_structure_elements`: Copies a series of longwords and words from one memory area to another in a loop, advancing both source and destination pointers while copying data elements.
- `copy_and_skip_elements`: Copies a series of longwords and words from one memory area to another, skipping certain elements by advancing the source pointer, and continues until all elements are copied.
- `move_object_coordinates`: Moves object coordinates by comparing current values with lookup table entries, adjusting movement direction based on comparison results, and updating structure fields with new position data.
- `frag_moveq_move_mulu_addi_movea_lea_dbr`: Short code fragment that loads 0 into d0, then copies a fixed memory address to d0, then multiplies d0 by 2596 (unsigned), then adds 0 to d0, then copies d0 to a1, then loads the address of a fixed memory address into a2, then copies 649 to d0, then copies the memory at a1 (advancing a1) to the memory at a2 (advancing a2), then loops back while counting down d0, then returns.
- `frag_moveq_move_mulu_addi_movea_lea_dbr_2`: Short code fragment that loads 0 into d0, then copies a fixed memory address to d0, then multiplies d0 by 2596 (unsigned), then adds 0 to d0, then copies d0 to a1, then loads the address of a fixed memory address into a2, then copies 649 to d0, then copies the memory at a2 (advancing a2) to the memory at a1 (advancing a1), then loops back while counting down d0, then returns.
- `update_game_tile`: Processes a game tile by checking its state, modifying bit flags based on conditions, updating global variables, and calling rendering routines.
- `update_render_targets`: Updates three memory locations with specific values based on bitwise conditions from a global variable, then executes a loop routine a fixed number of times before returning.
- `process_pattern_elements`: Processes pattern elements by loading data from memory, performing arithmetic operations, and calling other routines with different parameters in a loop.
- `calculate_and_store_value`: Performs arithmetic operations on a value from memory, divides it by a constant, and stores the result as a word at an offset in another memory area.
- `render_pattern_elements`: Processes three pattern elements by loading data from memory, performing arithmetic operations, and calling another routine with different parameters for each element.
- `update_position_and_render`: Updates position coordinates and rendering parameters based on bit tests and conditional logic, then calls other routines to handle rendering and state management before returning.
- `copy_structure_data_forward`: Copies data elements from a structure starting at a fixed address to another location, advancing both source and destination pointers while copying longwords, words, and bytes.
- `load_data_from_pointer`: Reads multiple data values in reverse order from a memory location pointed to by a1, storing them into various global variables and table entries, then returns.
- `update_entity_position`: Processes input flags to update entity coordinates and rendering parameters, handling movement in four directions with associated animation frame updates.
- `initialize_global_variables`: Sets multiple global variables to specific values including zero, a word value, and several longword values, effectively initializing a data structure.
- `update_state_machine_values`: Updates state machine values in global memory based on comparisons with thresholds, storing different byte values and updating word values at specific offsets depending on the conditions met.
- `update_input_flags`: Processes joystick input bits to set global flags based on bit comparisons and stores the result in a global variable.
- `frag_move_movea_rts`: Short code fragment that copies a2 to the stack, then copies a fixed memory address to a2, then copies a1 to the memory at a2 (advancing a2), then copies a2 to a fixed memory address, then copies the top of the stack to a2, then returns.
- `setup_render_params`: Calculates rendering parameters from input coordinates and stores them in multiple memory locations including position and color data.
- `update_object_list`: Updates an object list by adding new entries and setting up rendering parameters for each object in the list.
- `process_object_cleanup`: Cleans up objects from a list by moving pointers and updating memory locations with specific rendering values.
- `setup_render_data`: Calculates rendering data based on table indices and stores position, color and size parameters in memory locations.
- `process_pattern_loop`: Processes a pattern rendering loop by iterating through specific memory locations and calling other routines to update values.
- `frag_lea_moveq_move_adda_mulu_rts`: Short code fragment that loads the address of a fixed memory address into a2, then loads 0 into d0, then copies the memory at a1 to d0, then adds 8 to a1, then multiplies d0 by 12 (unsigned), then adds d0 to a2, then returns.
- `tiny_move_add_rts`: Small routine of 3 instructions that copies d2 to d3, then adds d3 to d3, then returns.
- `render_pattern_loop`: Loops through a pattern rendering sequence, updating memory locations with specific values and incrementing pointers.
- `render_static_pattern`: Processes a static pattern rendering loop by updating memory locations with position data and incrementing pointers.
- `copy_and_process_data`: Copies word values from a source structure to a destination pointer, processes coordinates, and updates multiple memory locations including a global pointer and a table entry.
- `render_diagonal_pattern`: Processes a diagonal pattern rendering loop, updating memory locations with specific values and incrementing pointers.
- `render_offset_pattern`: Processes an offset pattern rendering loop by updating memory locations with position data and incrementing pointers.
- `initialize_render_data_2`: Initializes render data by copying object coordinates, performing arithmetic operations, and updating pointer addresses in a loop until reaching a termination condition.
- `initialize_render_data`: Initializes render data by setting specific memory values and performing coordinate adjustments.
- `update_render_state`: Updates rendering state by copying byte data from a source location to a global variable, then calling routines to process the data.
- `frag_movem_bsr_move_sub_cmp_ble_rts`: Short code fragment that saves registers on the stack, then calls another routine, then copies d7 to d0, then calls another routine, then subtracts d0 from d7, then compares d7 with d2, then branches if the comparison was less or equal, then restores saved registers from the stack, then returns.
- `frag_clr_move_lsl_rts`: Short code fragment that clears d7, then copies a fixed memory address to d7, then shifts d7 left by 4 (2 times in a row), then copies a fixed memory address to d7, then shifts d7 left by 4 (2 times in a row), then copies a fixed memory address to d7, then returns.
- `allocate_memory`: Allocates a block of memory with a specific size and stores the result in a global variable, then calls another routine if the allocation succeeds.
- `frag_move_beq_movea_lea`: Short code fragment that copies a fixed memory address to d0, then branches if the last result was zero, then copies d0 to a1, then loads the address of a fixed memory address into a2, then copies 32450 to d0.
- `frag_move_subq_bne_rts`: Short code fragment that copies the memory at a1 (advancing a1) to the memory at a2 (advancing a2), then subtracts 1 from d0, then branches if the last result was not zero, then returns.
- `load_level_data`: Loads level data from files by opening them, reading specific amounts of data into memory locations, and closing the file handles.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `close_file_handle`: Reads a file handle from a global variable, closes it using the operating system function, and then returns.
- `frag_moveq_move_mulu_dbra_movea_jsr_rts`: Short code fragment that loads 79 into d1, then copies 3583 to d0, then multiplies d3 by d3 (unsigned) (2 times in a row), then loops back while counting down d0, then loops back while counting down d1, then copies a fixed memory address to a6, then calls another routine, then returns.
- `call_permit`: Wrapper that calls the exec library function Permit and returns its result to the caller.
- `frag_move_rts`: Short code fragment that copies 6944 to a fixed memory address, then copies 32784 to a fixed memory address, then returns.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `frag_move_lea_clr_subq_bne_rts`: Short code fragment that copies an address to d0, then loads the address of a fixed memory address into a1, then clears the memory at a1 (advancing a1), then subtracts 1 from d0, then branches if the last result was not zero, then returns.
- `frag_movem_andi_beq_move_jsr_jmp`: Short code fragment that saves registers on the stack, then masks a fixed memory address with 16, then branches if the last result was zero, then copies 16 to a fixed memory address, then calls another routine, then restores saved registers from the stack, then jumps to another point in the code.
- `find_min_and_fill_tables`: Searches a byte table for the smallest entry, then fills an integer table with computed values and clears a following block before returning.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `update_audio_channels`: Updates audio channel parameters including playback position, volume, and table lookup values based on command bytes, and writes updated values to hardware registers AUD0LCH through AUD3LCH.
- `update_audio_parameter`: Reads a command byte from a structure and adjusts audio parameters including playback position, volume, and table lookup values based on the command.
- `update_object_attributes`: Fetches object properties from a lookup table using an index derived from two bytes in the input structure, updates position and sprite data, and modifies global state variables based on conditional checks.
- `initialize_game_assets`: Initializes various game asset files by opening them, reading specific data chunks, and processing the loaded data through multiple routines to set up rendering and object states.
- `update_color_palette`: Updates color palette values in hardware registers while copying pixel data from source to destination buffers.

### Kastle Kumquat (1992)(Cybernetix).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `copy_data_block`: Copies 14 longwords from a source table to a global structure, incrementing both pointers after each copy operation.
- `reverse_copy_data`: Copies 14 longwords from a global structure to a destination table, using pointer arithmetic to navigate through the data.
- `update_position_values`: Processes position coordinates and updates movement parameters based on comparison results with stored values.
- `frag_add_moveq_bra`: Short code fragment that adds d5 to d6, then loads 1 into d7, then jumps to another point in the code.
- `frag_sub_moveq_cmp_blt_bgt_bra`: Short code fragment that subtracts d5 from d6, then loads 1 into d7, then compares d1 with d3, then branches if the comparison was less, then branches if the comparison was greater, then jumps to another point in the code.
- `frag_add_moveq_bra_2`: Short code fragment that adds d5 to d1, then loads 1 into d7, then jumps to another point in the code.
- `update_object_position_3`: Updates object coordinates based on input values and stores computed position data.
- `frag_tst_bne_add_move_cmp_blt_moveq_bra`: Short code fragment that tests d7, then branches if the last result was not zero, then adds the memory at an offset from a1 to d4, then copies 28372 to the memory at an offset from a1, then copies a fixed memory address to the memory at an offset from a1, then compares d4 with the memory at an offset from a1, then branches if the comparison was less, then loads 0 into d4, then jumps to another point in the code.
- `update_object_timer`: Updates a timer value at offset 8 in the structure pointed to by a1, increments it, and clears it when reaching 24.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `frag_moveq_move_mulu_addi_movea_lea_dbr_3`: Short code fragment that loads 0 into d0, then copies a fixed memory address to d0, then multiplies d0 by 2596 (unsigned), then adds an address to d0, then copies d0 to a1, then loads the address of a fixed memory address into a2, then copies 649 to d0, then copies the memory at a1 (advancing a1) to the memory at a2 (advancing a2), then loops back while counting down d0, then returns.
- `frag_moveq_move_mulu_addi_movea_lea_dbr_4`: Short code fragment that loads 0 into d0, then copies a fixed memory address to d0, then multiplies d0 by 2596 (unsigned), then adds an address to d0, then copies d0 to a1, then loads the address of a fixed memory address into a2, then copies 649 to d0, then copies the memory at a2 (advancing a2) to the memory at a1 (advancing a1), then loops back while counting down d0, then returns.
- `frag_movem_move_andi_beq_bra`: Short code fragment that saves registers on the stack, then copies a fixed memory address to d0, then masks d0 with 16, then branches if the last result was zero, then copies 81920 to a fixed memory address, then jumps to another point in the code.
- `frag_move_andi_beq_bra`: Short code fragment that copies 81760 to a fixed memory address, then copies a fixed memory address to d0, then masks d0 with 8, then branches if the last result was zero, then copies 82080 to a fixed memory address, then jumps to another point in the code.
- `frag_move_andi_beq_bra_2`: Short code fragment that copies 81760 to a fixed memory address, then copies a fixed memory address to d0, then masks d0 with 4, then branches if the last result was zero, then copies 82240 to a fixed memory address, then jumps to another point in the code.
- `initialize_rendering_loop`: Sets a global variable to a fixed value and repeatedly calls another routine until a condition is met, then restores registers and returns.
- `frag_lea_adda_moveq_btst_beq_addq_bra`: Short code fragment that loads the address of a fixed memory address into a1, then adds 196 to a1, then loads 0 into d7, then tests bit 0 of an indexed entry from a1, then branches if the last result was zero, then tests bit 1 of an indexed entry from a1, then branches if the last result was zero, then adds 1 to an indexed entry from a1, then jumps to another point in the code.
- `update_object_state_2`: Updates object state by decrementing a counter, checking conditions on bit flags, clearing bits when certain conditions are met, and performing memory lookups based on calculated indices.
- `validate_and_clear_bits`: Checks if a value at a specific memory location equals -2, and if so, clears and tests a bit in another location before making conditional branches based on the test result.
- `calculate_and_store_value_2`: Performs arithmetic operations on input values to calculate a result, then stores that result at an indexed memory location.
- `render_object_parts`: Sets up rendering parameters for three object parts by loading addresses from a structure, adding offsets to a global value, and calling a rendering setup routine with different parameters for each part.
- `frag_btst_bne_beq_bset_clr_move_bra`: Short code fragment that tests bit 0 of a fixed memory address, then branches if the last result was not zero, then tests bit 0 of a fixed memory address, then branches if the last result was zero, then sets bit 0 of a fixed memory address, then clears a fixed memory address, then copies a fixed memory address to a fixed memory address, then jumps to another point in the code.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `update_movement_parameters`: Modifies movement values based on bit tests and updates corresponding memory locations with calculated results.
- `frag_btst_beq_add_move_sub_bra`: Short code fragment that tests bit 2 of d3, then branches if the last result was zero, then adds d6 to the memory at an offset from a1, then copies 28224 to the memory at an offset from a1, then tests bit 3 of d3, then branches if the last result was zero, then subtracts d6 from the memory at a1, then copies 28252 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_btst_beq_add_move_andi_cmp_bne_bra`: Short code fragment that tests bit 4 of d3, then branches if the last result was zero, then adds d6 to the memory at a1, then copies 28280 to the memory at an offset from a1, then copies d3 to d4, then masks d4 with 10, then compares d4 with 10, then branches if the last result was not zero, then copies 28112 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_move_andi_cmp_bne_bra`: Short code fragment that copies d3 to d4, then masks d4 with 18, then compares d4 with 18, then branches if the last result was not zero, then copies 28140 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_move_andi_cmp_bne_bra_2`: Short code fragment that copies d3 to d4, then masks d4 with 12, then compares d4 with 12, then branches if the last result was not zero, then copies 28196 to the memory at an offset from a1, then jumps to another point in the code.
- `update_object_state`: Processes object state changes based on bitwise operations, updates memory locations with computed values, and calls other routines to handle rendering.
- `frag_addq_bne_btst_bra`: Short code fragment that adds 8 to d2, then branches if the last result was not zero, then tests bit 0 of d2, then branches if the last result was not zero, then jumps to another point in the code.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `frag_tst_bne_clr_bra`: Short code fragment that tests d2, then branches if the last result was not zero, then clears the memory at an offset from a1, then jumps to another point in the code.
- `frag_move_moveq_btst_beq_sub_bra`: Short code fragment that copies d2 to a fixed memory address, then copies the memory at an offset from a1 to d1, then loads 1 into d7, then tests bit 1 of d2, then branches if the last result was zero, then subtracts d7 from the memory at an offset from a1, then copies 27932 to the memory at an offset from a1, then jumps to another point in the code.
- `modify_object_position`: Modifies object coordinates by adding or subtracting values, setting specific longword values, updating position data through multiple memory operations, and performing bitwise operations to control movement behavior.
- `update_object_coordinates_2`: Updates object position data by adding a value to a memory location, setting specific longword values, and performing bitwise operations to determine which memory address to update next.
- `frag_move_andi_cmp_bne_bra_3`: Short code fragment that copies d2 to d4, then masks d4 with 18, then compares d4 with 18, then branches if the last result was not zero, then copies 28004 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_move_andi_cmp_bne_bra_4`: Short code fragment that copies d2 to d4, then masks d4 with 12, then compares d4 with 12, then branches if the last result was not zero, then copies 28076 to the memory at an offset from a1, then jumps to another point in the code.
- `calculate_object_position`: Calculates object position coordinates using division and multiplication operations, updates memory locations with computed values, and handles boundary conditions.
- `set_global_variables`: Writes specific byte and word values to multiple global variables located at fixed offsets, then copies longword values from one memory region to another.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `frag_ori_blt`: Short code fragment that merges an address into the memory at a0 (advancing a0), then branches if the comparison was less.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `frag_ori_cmpi_bgt_move_bra`: Short code fragment that merges 249 into d5, then merges 116 into d0, then compares the memory at a1 with 1, then branches if the comparison was greater, then copies a fixed memory address to a fixed memory address, then copies 303 to the memory at a1, then jumps to another point in the code.
- `frag_cmpi_blt_move_bra`: Short code fragment that compares the memory at a1 with 304, then branches if the comparison was less, then copies a fixed memory address to a fixed memory address, then copies 2 to the memory at a1, then jumps to another point in the code.
- `frag_cmpi_bgt_move_bra`: Short code fragment that compares the memory at an offset from a1 with 1, then branches if the comparison was greater, then copies a fixed memory address to a fixed memory address, then copies 223 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_cmpi_blt_move_rts`: Short code fragment that compares the memory at an offset from a1 with 224, then branches if the comparison was less, then copies a fixed memory address to a fixed memory address, then copies 2 to the memory at an offset from a1, then copies the memory at a1 to a fixed memory address, then returns.
- `process_input_flags`: Tests various bit conditions of input data and sets flags in a global variable based on the results of those tests.
- `frag_bset_bra`: Short code fragment that sets bit 2 of a fixed memory address, then jumps to another point in the code.
- `frag_bset_bra_2`: Short code fragment that sets bit 4 of a fixed memory address, then jumps to another point in the code.
- `frag_bset_bra_3`: Short code fragment that sets bit 3 of a fixed memory address, then jumps to another point in the code.
- `frag_bset_btst_bne_movem_rts`: Short code fragment that sets bit 1 of a fixed memory address, then tests bit 7 of a fixed memory address, then branches if the last result was not zero, then sets bit 0 of a fixed memory address, then restores saved registers from the stack, then returns.
- `frag_move_movea_rts_2`: Short code fragment that copies a2 to the stack, then copies a fixed memory address to a2, then copies a1 to the memory at a2 (advancing a2), then copies a2 to a fixed memory address, then copies the top of the stack to a2, then returns.
- `update_object_position`: Calculates object coordinates based on input values and updates memory locations with computed position data.
- `update_object_position_2`: Calculates object position values and updates multiple memory locations with rendering parameters for a game object.
- `update_object_data`: Reads a value from a memory location, checks a bit in another location, and writes multiple values to fixed offsets in a structure pointed to by a register.
- `frag_move_rts_2`: Short code fragment that copies a2 to a fixed memory address, then returns.
- `initialize_object_data`: Loads data from global tables based on index values, performs arithmetic operations on coordinates, and stores the results in structure fields at offsets 64, 66, 76, 80, 88, 98, 100 of the structure pointed to by a0.
- `tiny_move_tst_rts`: Small routine of 3 instructions that copies 0 to d0, then tests d0, then returns.
- `process_rendering_data`: Processes rendering data by iterating through a loop, calling other routines to set up parameters, and updating counters until reaching specific limits.
- `frag_lea_moveq_move_adda_mulu_rts_2`: Short code fragment that loads the address of a fixed memory address into a2, then loads 0 into d0, then copies the memory at a1 to d0, then adds 8 to a1, then multiplies d0 by 12 (unsigned), then adds d0 to a2, then returns.
- `tiny_move_add_rts_2`: Small routine of 3 instructions that copies d2 to d3, then adds d3 to d3, then returns.
- `setup_render_parameters`: Initializes rendering parameters for an object by setting specific memory locations with predefined values and updating pointer addresses in a loop that iterates based on a counter in d0.
- `configure_object_position`: Configures object position data by copying register values to structure fields at offsets 64, 66, 80, 84, 100, 102, 88 and incrementing pointer addresses in a loop that iterates four times.
- `copy_object_coordinates`: Copies object coordinate data from one memory location to another, performs arithmetic operations on the values, and updates pointer addresses in a loop.
- `frag_movea_move_rts`: Short code fragment that copies a fixed memory address to a5, then copies a6 to the memory at a5, then copies a1 to a fixed memory address, then returns.
- `update_object_coordinates`: Updates object coordinates by copying register values to structure fields at offsets 64, 66, 80, 84, 100, 102, 88 and incrementing pointer addresses in a loop that iterates based on a counter in d0.
- `set_object_dimensions`: Sets object dimension values in structure fields at offsets 64, 66, 100, 98, 96, 102 and updates pointer addresses in a loop that iterates four times.
- `adjust_object_position_2`: Adjusts object position by decrementing pointer values and storing updated coordinates in structure fields at specific offsets through a loop.
- `frag_move_rts_3`: Short code fragment that copies a1 to the memory at an offset from a2, then returns.
- `adjust_object_position`: Adjusts object position by decrementing pointer values and storing updated coordinates in structure fields at offsets 64, 66, 80, 84, 100, 102, 88 in a loop that iterates four times.
- `frag_moveq_move_mulu_dbra_movea_jsr_rts_2`: Short code fragment that loads 79 into d1, then copies 3583 to d0, then multiplies d3 by d3 (unsigned) (2 times in a row), then loops back while counting down d0, then loops back while counting down d1, then copies a fixed memory address to a6, then calls another routine, then returns.
- `call_permit_2`: Wrapper that calls the exec library function Permit and returns its result to the caller.
- `frag_move_rts_4`: Short code fragment that copies 23336 to a fixed memory address, then copies 32784 to a fixed memory address, then returns.
- `start_copper_list_2`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
