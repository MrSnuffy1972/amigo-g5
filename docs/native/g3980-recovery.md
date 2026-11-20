# Game 3980 (Hyperion v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03980). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1112820 top-level bytes in 12 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 50 raw-word lines |
| Functions named and described | pass, 157 of 157 named (0 author symbols), 157 of 157 described (55 by rule, 101 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 160372 bytes, 111 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6124 of 6163 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 73806 |
| code | 24436 |
| data | 1014570 |
| padding | 8 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 19 | 4 |
| MAIN | 1 | 188 | 36 |
| TITLE | 1 | 155 | 3 |
| WRITER | 1 | 155 | 3 |
| CODE (raw code file, load address unknown) | 1 | 5491 | 108 |
| DATA (raw code file, load address unknown) | 1 | 155 | 3 |

## Cases handled

- Hyperion_v1.0/Hyperion_v1.0.hdf: hard-disk image (AmigaDOS OFS, 1216 blocks): its filesystem parses and all 10 files in it are byte-identical to the extracted tree Hyperion_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- Hyperion_v1.0/Hyperion_v1.0_hdf/Hyperion/data/MAIN: Double Action 1.0 (xfd) stream found by ancient scan (2084 -> 2816 bytes) but only 0 of 2 sampled windows are in reference RAM: not used
- Hyperion_v1.0/Hyperion_v1.0_hdf/Hyperion/data/TITLE: DefJam Cruncher 3.2 (xfd) stream found by ancient scan (247472 -> 339968 bytes) but only 0 of 150 sampled windows are in reference RAM: not used
- Hyperion_v1.0/Hyperion_v1.0_hdf/Hyperion/data/WRITER: DefJam Cruncher 3.2 (xfd) stream found by ancient scan (3760 -> 16384 bytes) but only 0 of 3 sampled windows are in reference RAM: not used
- Hyperion_v1.0/Hyperion_v1.0_hdf/Hyperion/data/CODE: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Hyperion_v1.0/Hyperion_v1.0_hdf/Hyperion/data/DATA: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Hyperion_v1.0/Hyperion_v1.0_hdf/Hyperion/data/TITLE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 94836 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_hardware_and_copy_code` (MAIN): Initializes hardware registers including interrupt and DMA control, writes specific values to a hardware port, sets a bit in a memory location, and copies 517 bytes of code from one location to another using a loop with decrementing counter. Kind: disk hardware check. Pass path: The routine initializes hardware and performs a copy of code via a jump table, suggesting it may be checking for specific hardware states or disk integrity before proceeding. If the check passes, execution continues normally after the jumpt.

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_movem_movea_jsr_rts`: Short code fragment that saves registers on the stack, then copies a location in the code to a2, then calls another routine, then restores saved registers from the stack, then returns.
- `call_library_offset_eight_via_a2`: Saves d1 and a0-a2 on the stack, loads a library base from a pc-relative long into a2, calls the routine at offset eight of that base, then restores the registers and returns.
- `frag_lea_movem_movea_jsr_rts`: Short code fragment that loads the address of a location in the code into a0, then loads the address of a fixed memory address into a1, then saves registers on the stack, then copies a location in the code to a2, then calls another routine, then restores saved registers from the stack, then returns.

### MAIN

- `initialize_hardware_and_copy_code`: Initializes hardware registers including interrupt and DMA control, writes specific values to a hardware port, sets a bit in a memory location, and copies 517 bytes of code from one location to another using a loop with decrementing counter.
- `frag_movea_move_sub_cmpa_bcs_subq_bne_b`: Short code fragment that copies a location in the code to a1, then copies a location in the code to d0, then subtracts a1 from d0, then compares a0 with a1, then branches if the carry flag is set, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then subtracts 4 from d0, then branches if the last result was not zero, then jumps to another point in the code.
- `frag_adda_move_subq_bne_bra`: Short code fragment that adds d0 to a0, then adds d0 to a1, then copies the memory just below a0 (moving a0 down) to the memory just below a1 (moving a1 down), then subtracts 4 from d0, then branches if the last result was not zero, then jumps to another point in the code.
- `frag_subq_move_lea_moveq_jmp`: Short code fragment that subtracts 1 from d6, then copies d6 to d3, then loads the address of a location in the code into a6, then loads 8 into d4, then jumps to the address in a4.
- `frag_move_dbra_lea_bra`: Short code fragment that copies d6 to the memory just below a2 (moving a2 down), then loops back while counting down d3, then loads the address of a location in the code into a6, then jumps to another point in the code.
- `frag_tst_beq_bra`: Short code fragment that tests d6, then branches if the last result was zero, then jumps to another point in the code.
- `frag_moveq_lea_jmp`: Short code fragment that loads 3 into d4, then loads the address of a location in the code into a6, then jumps to the address in a4.
- `frag_addi_bra`: Short code fragment that adds 6 to d6, then jumps to another point in the code.
- `frag_moveq_lea_jmp_2`: Short code fragment that loads 10 into d4, then loads the address of a location in the code into a6, then jumps to the address in a4.
- `frag_moveq_bra`: Short code fragment that loads 16 into d4, then jumps to another point in the code.
- `initialize_and_process`: Initializes multiple pointers from program counter relative addresses, processes data through a loop using bit manipulation and conditional jumps, and performs memory operations based on comparison results.
- `frag_cmp_bcs_beq_lea_jmp`: Short code fragment that compares d6 with 6, then branches if the carry flag is set, then branches if the last result was zero, then loads the address of a location in the code into a6, then jumps to the address in a5.
- `frag_lea_jmp`: Short code fragment that loads the address of a location in the code into a6, then jumps to the address in a5.
- `frag_cmp_bcs_beq_lea_moveq_jmp`: Short code fragment that compares d6 with 2, then branches if the carry flag is set, then branches if the last result was zero, then loads the address of a location in the code into a6, then loads 1 into d4, then jumps to the address in a4.
- `frag_moveq_jmp`: Short code fragment that loads 2 into d4, then jumps to the address in a4.
- `frag_moveq_bra_2`: Short code fragment that loads 3 into d3, then jumps to another point in the code.
- `frag_moveq_tst_beq_bra`: Short code fragment that loads 1 into d3, then tests d6, then branches if the last result was zero, then jumps to another point in the code.
- `frag_tst_beq_lea_bra`: Short code fragment that tests d6, then branches if the last result was zero, then loads the address of a location in the code into a6, then jumps to another point in the code.
- `frag_tst_beq_bra_2`: Short code fragment that tests d6, then branches if the last result was zero, then jumps to another point in the code.
- `frag_cmp_bcs_lea_jmp`: Short code fragment that compares d6 with 3, then branches if the carry flag is set, then loads the address of a location in the code into a6, then jumps to the address in a5.
- `frag_cmp_bcs_lea_jmp_2`: Short code fragment that compares d6 with 3, then branches if the carry flag is set, then loads the address of a location in the code into a6, then jumps to the address in a5.
- `frag_cmp_bcs_moveq_lea_jmp`: Short code fragment that compares d6 with 3, then branches if the carry flag is set, then loads 8 into d4, then loads 10 into d3, then loads the address of a location in the code into a6, then jumps to the address in a4.
- `frag_add_lea_bra`: Short code fragment that adds d6 to d3, then loads the address of a location in the code into a6, then jumps to another point in the code.
- `frag_tst_bne_moveq_bra`: Short code fragment that tests d6, then branches if the last result was not zero, then loads 7 into d4, then loads 2 into d2, then jumps to another point in the code.
- `frag_moveq_move_lea_jmp`: Short code fragment that loads 8 into d4, then copies 130 to d2, then loads the address of a location in the code into a6, then jumps to the address in a4.
- `frag_add_movea_adda_move_dbra_bra`: Short code fragment that adds d2 to d6, then copies a2 to a6, then adds d6 to a6, then copies the memory just below a6 (moving a6 down) to the memory just below a2 (moving a2 down), then loops back while counting down d3, then jumps to another point in the code.
- `frag_moveq_bra_3`: Short code fragment that loads 7 into d3, then jumps to another point in the code.
- `frag_moveq_bra_4`: Short code fragment that loads 4 into d3, then jumps to another point in the code.
- `frag_moveq_cmp_bcs_beq_bra`: Short code fragment that loads 2 into d3, then compares d6 with 1, then branches if the carry flag is set, then branches if the last result was zero, then loads 3 into d3, then jumps to another point in the code.
- `frag_moveq_bra_5`: Short code fragment that loads 0 into d6, then jumps to another point in the code.
- `frag_lsl_beq_roxl_dbra_jmp`: Short code fragment that shifts d5 left by 1, then branches if the last result was zero, then rotates d6 left through the extend flag by 1, then loops back while counting down d4, then jumps to the address in a6.
- `frag_cmpa_beq_move_roxl_bra`: Short code fragment that compares a0 with a1, then branches if the last result was zero, then copies the memory just below a0 (moving a0 down) to d5, then copies d5 to the memory at an offset from a3, then rotates d5 left through the extend flag by 1, then jumps to another point in the code.
- `copy_and_compare`: Copies bytes from one memory area to another while comparing addresses, then processes input bytes based on comparison results and stores modified values.
- `frag_move_movea_jmp`: Short code fragment that copies d0 to the memory just below a2 (moving a2 down), then copies a location in the code to a2, then copies 33280 to the memory at an offset from a4, then copies 49152 to the memory at an offset from a4, then jumps to the address in a2.
- `frag_move_subq_bne_bra`: Short code fragment that copies the memory just below a0 (moving a0 down) to d2, then copies the memory just below a0 (moving a0 down) to d1, then copies d1 to the memory just below a2 (moving a2 down), then subtracts 1 from d2, then branches if the last result was not zero, then copies d1 to the memory at an offset from a4, then jumps to another point in the code.
- `frag_moveq_move_bra`: Short code fragment that loads 0 into d1, then copies the memory just below a0 (moving a0 down) to d2, then jumps to another point in the code.

### TITLE

- `initialize_title_screen`: Initializes title screen data by copying a sequence of bytes from a source table to a destination buffer, then performs bit-shifting operations on a 32-bit value while accumulating results in a counter, storing byte values to memory locations based on the accumulated counter.
- `shift_and_store_byte`: Shifts a 32-bit value right by one bit, accumulates results in a counter, and stores byte data to memory locations.
- `shift_and_accumulate`: Shifts a 32-bit value right by one bit in a loop, accumulates results in a counter, and calls another routine when the shift operation completes.

### WRITER

- `initialize_data_copy`: Initializes data copying by setting up source and destination pointers, then copies bytes from a source buffer to a destination based on a loop condition.
- `shift_and_store_byte_2`: Shifts a 32-bit value right by one bit, accumulates results in a counter, and stores byte data to memory locations.
- `shift_and_store_byte_3`: Loads a byte from memory, performs bitwise operations on it, and stores the result to a destination location while updating counters.

### CODE (raw code file, load address unknown)

- `initialize_video_and_audio`: Initializes video and audio hardware registers, clears a memory region, and sets up interrupt and DMA control registers.
- `update_counter`: Decrements a global counter and handles different conditional branches based on the counter value and other memory states, updating various game parameters.
- `frag_lea_moveq_move_addq_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a1, then loads 15 into d7, then copies the memory at a0 (advancing a0) to the memory at a1, then adds 4 to a1, then loops back while counting down d7, then returns.
- `update_memory_values`: Updates word values at specific memory locations by comparing them with a fixed value and incrementing when they don't match, then calls two other routines.
- `decrement_memory_values`: Decrements word values at specific memory locations conditionally based on their current values, then calls multiple routines with different loop counts.
- `bitwise_addition`: Processes word values from two memory locations by masking and comparing components, then adds specific values to components when they don't match before combining them with OR operations.
- `bitwise_subtraction`: Processes word values from a memory location by masking and comparing components, then subtracts specific values from components when they are non-zero before combining them with OR operations.
- `copy_and_accumulate_data_2`: Copies data from an input buffer to a destination table while accumulating values and updating global counters based on a conditional switch statement.
- `copy_and_accumulate_data`: Copies data from input buffer to a destination table while accumulating values and updating global counters.
- `validate_and_initialize_structure`: Validates a structure value, calls initialization routines, and sets multiple global variables with specific constants.
- `table_lookup`: Uses a byte value as an index to look up and copy word values from two tables into corresponding positions in two other tables.
- `frag_subq_bmi_rts_move_not_beq`: Short code fragment that subtracts 1 from a fixed memory address, then branches if the last result was negative, then returns, then copies 2 to a fixed memory address, then inverts a fixed memory address, then branches if the last result was zero, then copies 47 to a fixed memory address, then returns, then copies 39 to a fixed memory address, then returns.
- `process_graphics_data`: Processes graphics data by decrementing a counter, checking for null values, and performing bit operations on memory locations when conditions are met.
- `bit_shift_and_modify`: Shifts a word value right by one bit, conditionally adds 64 to it, stores the result in memory, and modifies a bit in another memory location based on the least significant bit of the original value.
- `initialize_and_update`: Initializes a byte value to one, clears a word value at a fixed address, and updates memory locations with values from a buffer.
- `frag_lea_move_clr_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a2, then copies 7 to d1, then copies 9 to d0, then clears the memory at a2 (advancing a2), then loops back while counting down d0, then loads the address of the memory at an offset from a2 into a2, then loops back while counting down d1, then returns.
- `frag_move_addq_cmp_bne_rts`: Short code fragment that copies a fixed memory address to d0, then adds 3 to d0, then compares d0 with a fixed memory address, then branches if the last result was not zero, then returns.
- `initialize_and_process_2`: Initializes several memory locations with specific values, clears a buffer, then processes data from multiple sources to set bits in two different memory areas based on comparison results.
- `process_object_collision`: Processes object collision detection by comparing properties of pairs of objects and updating global variables when conditions are met.
- `process_collision_data`: Processes collision detection data by iterating through object pairs, comparing their properties, and updating global variables when conditions are met.
- `find_matching_value`: Searches through a sequence of memory locations for a matching value and calls another routine when found.
- `update_game_objects`: Processes game object data by updating coordinates and handling collision detection with specific value ranges, then increments a global counter and sets a flag when reaching a threshold.
- `update_entity_positions`: Updates positions of entities by adding calculated values to their coordinates, performs boundary checks on entity data, and manages a global counter that controls audio playback triggers.
- `process_object_structures`: Checks multiple object structure flags and calls processing routines for each non-zero structure in sequence.
- `process_object_data`: Processes object data by iterating through structures, updating position and velocity values, and performing comparisons between fields.
- `process_structured_data`: Processes structured data by checking for zero values, calling routines when conditions are met, and updating memory locations with calculated values.
- `check_and_update_entities`: Checks multiple global variables for zero value and calls another routine for each non-zero case, updating entity states accordingly.
- `update_entity_position`: Updates the position coordinates of an entity by adding velocity components and performs boundary checks on the resulting values.
- `frag_lea_tst_beq_bsr_rts`: Short code fragment that loads the address of a fixed memory address into a0, then tests the memory at a0, then branches if the last result was zero, then calls another routine, then returns.
- `calculate_and_accumulate`: Calculates values using multiplication and addition operations on word values from memory locations, then accumulates results in a sequence of memory locations.
- `frag_lea_move_subq_rts`: Short code fragment that loads the address of a fixed memory address into a1, then loads the address of a fixed memory address into a5, then copies the memory at a0 (advancing a0) to d7, then subtracts 1 from d7, then copies a fixed memory address to d5, then copies a fixed memory address to d6, then returns.
- `update_tile_graphics`: Updates tile graphics data based on position and type, performing bit operations on memory locations determined by coordinates and a lookup table.
- `update_position_data`: Updates multiple position and velocity values in a structure based on register contents and performs boundary checks on certain fields.
- `copy_tile_data`: Copies 8x8 pixel tile data from two source locations to two destination locations, processing 8 rows of data with specific byte offsets.
- `copy_tile_row`: Copies 8 rows of pixel data from a source location to a destination, using fixed offsets and processing 4 bytes per row.
- `frag_move_lea_addq_cmpi_bne_subi_rts`: Short code fragment that copies 1 to a fixed memory address, then loads the address of a fixed memory address into a0, then adds 1 to the memory at an offset from a0, then compares the memory at an offset from a0 with 10, then branches if the last result was not zero, then subtracts 10 from the memory at an offset from a0, then adds 1 to the memory at a0, then returns.
- `frag_move_lea_subq_tst_bge_addi_rts`: Short code fragment that copies 1 to a fixed memory address, then loads the address of a fixed memory address into a0, then subtracts 1 from the memory at an offset from a0, then tests the memory at an offset from a0, then branches if the comparison was greater or equal, then adds 10 to the memory at an offset from a0, then subtracts 1 from the memory at a0, then returns.
- `update_counter_2`: Updates a byte value at an address, increments it if it's less than 10, otherwise resets it to zero and increments another byte.
- `decrement_counter`: Decrements a counter value at a specific memory location and handles borrow when the counter goes below zero.
- `process_hex_digits`: Processes a 32-bit value by extracting individual hex digits, rotating them, and storing them in a sequence of memory locations with carry handling.
- `copy_tile_rows`: Copies 8 rows of pixel data from multiple source locations to a destination, processing 4 different tile sets with specific byte offsets.
- `fill_memory_blocks`: Fills memory blocks at specific addresses with predefined values by calling another routine multiple times with different parameters.
- `frag_move_swap_addq_rts`: Short code fragment that copies d0 to the memory at an offset from a0, then swaps the halves of d0, then copies d0 to the memory at an offset from a0, then adds 8 to a0, then returns.
- `copy_structure_data`: Copies data from a global structure to memory locations and then copies data from a source buffer to a destination buffer in chunks of 34 bytes.
- `transfer_buffer_data`: Transfers data from a source buffer to a destination buffer in chunks of 34 bytes, processing until reaching the end of the destination area.
- `copy_matrix_data`: Copies 8x8 matrix data from one location to another, processing 4 shorts per row and 8 rows total.
- `frag_lea_moveq_move_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads 99 into d7, then loads 39 into d6, then copies an address to the memory at a0 (advancing a0), then loops back while counting down d6, then loads 39 into d6, then copies an address to the memory at a0 (advancing a0), then loops back while counting down d6, then loops back while counting down d7, then returns.
- `frag_lea_moveq_move_addi_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads 23 into d7, then copies an address to d0, then copies d0 to the memory at a0 (advancing a0), then copies an address to the memory at a0 (advancing a0), then copies an address to the memory at a0 (advancing a0), then adds an address to d0, then loops back while counting down d7, then returns.
- `frag_lea_move_moveq_addi_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies 199 to d7, then loads 0 into d0, then copies d0 to the memory at a0 (advancing a0), then adds 160 to d0, then loops back while counting down d7, then returns.
- `calculate_divisions`: Calculates division results for a sequence of values and stores them in a table.
- `sort_structure_entries`: Sorts structure entries based on comparison criteria, swapping elements when necessary.
- `fill_memory_blocks_2`: Fills three distinct memory blocks with specific values using loop counters.
- `set_graphics_flags`: Sets graphics-related flags and performs conditional updates to various game state variables.
- `frag_lea_adda_move_addq_cmpi_bne_clr_rt`: Short code fragment that loads the address of a fixed memory address into a0, then adds a fixed memory address to a0, then copies the memory at a0 to a fixed memory address, then adds 2 to a fixed memory address, then compares a fixed memory address with 16, then branches if the last result was not zero, then clears a fixed memory address, then returns.
- `update_counter_backward`: Updates a counter value by subtracting half the difference between two values, with boundary checks.
- `update_counter_forward`: Updates a counter value by adding half the difference between two values, with boundary checks.
- `update_matrix_elements`: Processes word values from a global table, applies transformations including addition, subtraction, and bit shifting, then stores results back into the same table.
- `matrix_operations`: Performs matrix-like calculations using multiplication, subtraction, and bit shifting operations on word values from memory locations.
- `update_matrix_values`: Processes three word values from memory locations, applies transformations including addition, subtraction, and bit shifting, then stores results to specific memory addresses.
- `frag_lea_move_mulu_cmpi_blt_bsr_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies a fixed memory address to d0, then multiplies d0 by 12 (unsigned), then copies 0 to an indexed entry from a0, then compares a fixed memory address with 19, then branches if the comparison was less, then loads the address of a fixed memory address into a1, then calls another routine, then returns.
- `frag_lea_move_mulu_cmpi_bgt_bsr_rts`: Short code fragment that loads the address of a fixed memory address into a1, then copies a fixed memory address to d0, then multiplies d0 by 12 (unsigned), then copies 3072 to an indexed entry from a1, then compares a fixed memory address with 6, then branches if the comparison was greater, then loads the address of a fixed memory address into a1, then calls another routine, then returns.
- `clear_conditional_counters`: Clears specific counters when certain conditions are met based on comparison results.
- `calculate_and_initialize`: Loads four word values from a memory location, performs arithmetic operations including subtraction and bit shifting, compares the result with a negative value, and initializes multiple memory locations with specific values when the comparison succeeds.
- `frag_tst_bne_beq_bsr_move_clr_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then tests a fixed memory address, then branches if the last result was zero, then calls another routine (2 times in a row), then copies 1 to a fixed memory address, then calls another routine, then clears a fixed memory address, then returns.
- `decrement_timer`: Decrements a global timer variable and returns if it's still positive.
- `update_animation_state`: Updates animation state variables including position, velocity, and frame counters based on calculated values and conditions.
- `process_render_elements`: Processes graphical rendering elements by calculating coordinates, validating ranges, and configuring hardware registers for display.
- `update_projectile_positions`: Updates projectile coordinates by adding velocity components and performs boundary checks on position values.
- `initialize_effect`: Decrements a counter at offset 14 of a structure pointed to by a0.
- `initialize_render_state`: Sets up rendering parameters for a graphical operation based on input coordinates and global configuration, then configures multiple hardware registers with calculated values before returning.
- `calculate_vector_components`: Computes vector components using multiplication and shift operations, storing results in global variables at specific memory offsets.
- `initialize_graphics_parameters`: Initializes graphics parameters by setting values in a table based on a conditional check of a global variable.
- `update_render_params`: Reads word values from fixed memory locations, passes them to another routine for processing, and stores resulting values into memory locations.
- `frag_lea_adda_movem_add_asr_move_rts`: Short code fragment that loads the address of a fixed memory address into a0, then adds a fixed memory address to a0 (2 times in a row), then copies several registers to or from memory, then adds d0 to d1, then adds d1 to d2, then adds d2 to d3, then shifts d3 right by 2, then copies d3 to a fixed memory address, then returns.
- `update_counter_with_bounds_check`: Updates a counter value with bounds checking, performing arithmetic operations and loop control based on comparison results.
- `process_object_entries`: Processes object entries from a source table, filtering them based on coordinate thresholds and copying valid entries to a destination table.
- `frag_tst_beq_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then returns.
- `conditional_object_update`: Checks a global variable for zero value and branches to a section of code when the condition is met.
- `copy_structure_data_2`: Copies structure data to an array element after verifying the element is empty, then advances to the next array element.
- `initialize_and_process_data`: Initializes and processes data by copying values from global variables to a structure, then performs arithmetic operations on structure elements.
- `process_object_data_2`: Processes object state data including coordinates, angles, and audio settings, updating values based on calculations and conditional checks before returning.
- `update_distance_value`: Calculates a distance approximation using coordinate differences and square root computation, then adjusts a value based on comparison with the result.
- `compute_square_root_approximation`: Computes an approximation of the square root of a value using bit manipulation and iterative refinement techniques.
- `update_angle_value`: Updates an angle value based on position and velocity calculations, with boundary checks and clamping logic that modifies the angle when it exceeds certain thresholds.
- `update_coordinates`: Updates three coordinate values in a structure pointed to by a0, using multiplication and bit shifting operations on values from the stack, and also modifies two global variables with computed results.
- `update_position`: Updates position coordinates based on velocity and boundary checks, modifying values in a data structure.
- `update_position_and_check_collision`: Updates position coordinates based on velocity values and performs boundary checks with wrapping logic.
- `check_conditions_and_initialize`: Checks multiple global variables for zero values and returns early if any are non-zero.
- `frag_lea_moveq_bsr_rts`: Short code fragment that loads the address of a fixed memory address into a1, then loads the address of a fixed memory address into a5, then loads 34 into d7, then calls another routine, then returns.
- `frag_lea_moveq_bsr_rts_2`: Short code fragment that loads the address of a fixed memory address into a1, then loads the address of a fixed memory address into a5, then loads 20 into d7, then calls another routine, then returns.
- `initialize_render_parameters`: Sets up rendering parameters including viewport dimensions and coordinate limits in global variables, then performs conditional updates based on input values and calls other routines for further processing.
- `initialize_rendering_params`: Sets up rendering parameters including viewport dimensions and coordinate limits in global variables, then performs conditional updates based on bit flags and table lookups.
- `initialize_render_parameters_2`: Sets up rendering parameters including viewport dimensions and coordinate limits in global variables, then performs conditional updates based on a status flag.
- `calculate_and_store_values`: Performs arithmetic operations on input parameters and stores the results into various global structure offsets.
- `validate_and_process_coordinates`: Checks if a signed 16-bit value in register d3 is negative or exceeds 199, and calls another routine to handle invalid values.
- `validate_and_store_coordinates`: Validates input coordinates against a maximum threshold, stores processed values in global variables, and performs arithmetic operations on them.
- `validate_and_return`: Checks if a signed 16-bit value is negative, and if so, returns zero; otherwise, it returns the original value.
- `calculate_scaled_value`: Computes a scaled value based on input coordinates using multiplication and division operations, with bounds checking.
- `process_coordinate_difference`: Processes coordinate differences through arithmetic operations and stores results, handling negative values by returning zero.
- `process_and_clamp_values`: Processes pairs of values from memory tables, compares them, and updates global variables with maximum or minimum values.
- `perform_matrix_multiplication`: Executes matrix multiplication operations using data from registers and memory locations, storing intermediate results in global variables.
- `execute_advanced_matrix_operations`: Carries out advanced matrix calculations involving multiple data sources and registers, storing results in memory locations.
- `lookup_table_access`: Accesses a lookup table using a word value from a register as index, performs bounds checking, and returns a value from the table.
- `evaluate_collision_conditions`: Processes geometric data from memory locations to evaluate collision conditions between objects.
- `transform_coordinates_with_lookup`: Transforms coordinate values using lookup table data and stores the results in memory locations.
- `scale_and_divide_coordinates`: Scales coordinate values using a multiplier and divides them by a denominator value.
- `compute_scaled_values_with_global_data`: Computes scaled coordinate values using global data and lookup table entries, performing division operations with a denominator.
- `transform_vertices`: Processes vertex data by calculating interpolated values and storing transformed coordinates.

### DATA (raw code file, load address unknown)

- `decode_bitstream`: Processes a bitstream by shifting values right and accumulating results, storing decoded bytes to memory locations based on computed offsets.
- `load_and_shift_word`: Loads a 32-bit value from memory location pointed to by a0, extracts its lower byte into d7, stores the word value in d7 to memory location pointed to by a5, sets the condition code register to 16, and performs a right rotate through carry operation on the loaded value.
- `shift_and_accumulate_2`: Shifts a 32-bit value right by one bit in a loop, accumulates results in a counter, and calls another routine when the shift operation completes.
