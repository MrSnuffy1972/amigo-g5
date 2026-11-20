# Game 3609 (FatalMission v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03609). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (647153 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 127 of 127 named (0 author symbols), 127 of 127 described (45 by rule, 79 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4146 of 4146 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2744 |
| code | 15274 |
| data | 629131 |
| padding | 4 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| FM1 | 1 | 4078 | 122 |
| game.slave | 1 | 68 | 5 |

## Cases handled

- FatalMission_v1.0/FatalMission_v1.0.hdf: hard-disk image (AmigaDOS OFS, 704 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree FatalMission_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `format_score_display` (FM1): Formats score display data by converting numbers to strings and rendering them at specific screen coordinates with proper spacing. Kind: disk track check. Pass path: The routine checks for specific values at an address(a5) and an address(a5), and if they are non-zero, it calls sub_1FA4. A successful check (i.e., the values are set correctly) would proceed to execute those final calls, likely rendering the protectio.

## Functions


### FM1

- `initialize_system_state`: Clears a global variable, calls a routine to initialize hardware, and if a specific field at offset 172 of the returned structure is non-zero, calls two additional routines and stores the resulting pointer in the global variable.
- `initialize_game_memory`: Allocates memory for game data structures and initializes pointers to allocated blocks, then sets up interrupt vectors and graphics parameters based on predefined data tables.
- `frag_movea_move_rts`: Short code fragment that copies an address to a6, then copies the memory at an offset from a6 to the memory at an offset from a5 (2 times in a row), then copies 32767 to the memory at an offset from a6 (2 times in a row), then copies 127 to a fixed memory address, then copies a fixed memory address to the memory at an offset from a5, then returns.
- `setup_audio_registers`: Writes values to audio volume, interrupt enable, DMA control, and CIA interrupt control registers based on data from a structure, then returns.
- `frag_bsr_addq_move_rte`: Short code fragment that calls another routine, then adds 1 to the memory at an offset from a5, then copies 32 to the memory at an offset from a6, then returns from the exception.
- `initialize_character_data`: Initializes character position and graphics parameters in a data structure, updates a global counter, and conditionally processes input from two control sources.
- `calculate_and_store_coordinates`: Performs bit shifts and multiplications on input values to calculate an address offset, stores the result in a global structure, and sets several other values in that structure.
- `update_character_position`: Updates character position based on joystick input, clamps values to valid ranges, and handles boundary conditions for movement controls.
- `frag_clr_rts`: Short code fragment that clears the memory at an offset from a5, then returns.
- `frag_lea_moveq_move_subq_mulu_adda_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads 0 into d0, then copies the memory at an offset from a5 to d0, then subtracts 1 from d0, then multiplies d0 by 8 (unsigned), then adds d0 to a0, then copies the memory at an offset from a5 to the memory at a0, then returns.
- `decrement_and_clamp_character`: Decrements a counter in a global variable, then adjusts a character value at an offset from a pointer based on input from two hardware registers, clamping the result between ASCII 32 and ASCII 90.
- `frag_clr_rts_2`: Short code fragment that clears the memory at an offset from a5, then returns.
- `copy_data_to_table_entry`: Calculates an index into a table using a byte value from a structure, multiplies that index by eight, and copies a long word from another structure to the calculated table location.
- `initialize_game_data`: Initializes multiple structure fields with hardcoded values based on conditional flags, clears several global variables, and sets up game state variables.
- `update_position_and_velocity`: Updates position and velocity values in a structure based on input data and conditional logic, then stores several calculated values in the same structure.
- `format_score_display`: Formats score display data by converting numbers to strings and rendering them at specific screen coordinates with proper spacing.
- `format_number_to_string`: Converts a number into a string representation with decimal point formatting, writing individual digits to memory locations and null terminating the result.
- `frag_subq_cmpi_bne_move_bra`: Short code fragment that subtracts 6 from a0, then compares the memory at a0 (advancing a0) with 48, then branches if the last result was not zero, then copies 32 to the memory at an offset from a0, then jumps to another point in the code.
- `frag_divu_move_addi_clr_swap_rts`: Short code fragment that divides d0 by d4 (unsigned), then copies d0 to the memory at a0, then adds 48 to the memory at a0 (advancing a0), then clears d0, then swaps the halves of d0, then returns.
- `update_score_display`: Updates score display graphics for player lives using character data from lookup tables, setting up window coordinates and graphic parameters based on game state.
- `copy_character_data`: Copies character data from a lookup table to output buffer, processing multiple bytes per character with bit shifting operations.
- `decode_text_pattern`: Reads a sequence of bytes from a buffer, uses each byte as an index to access a table, and copies data from the table to a destination buffer while advancing both pointers.
- `validate_and_clear_structure_entries`: Iterates through a structure array, validates coordinate ranges for each entry, and clears entries that exceed specified bounds.
- `initialize_object_data_3`: Initializes object data by checking conditions and setting up object properties including position, velocity, and state flags.
- `frag_bsr_rts`: Short code fragment that calls another routine (7 times in a row), then returns.
- `update_object_collision`: Updates object collision detection by comparing coordinates, handling hit states, and managing score updates based on object interactions.
- `process_collision_detection_2`: Performs collision detection between object coordinates and a grid of cells, clearing flags and updating state when collisions are detected.
- `check_collision_and_update_2`: Checks for collisions between object boundaries and updates state flags accordingly, clearing bits and modifying global data structures during the process.
- `check_object_collision`: Checks for object collisions by comparing coordinate ranges and handling various state flags to determine collision outcomes.
- `process_collision_detection`: Processes collision detection for structures in an array by calculating positions and checking if values exceed threshold limits, clearing invalid entries when they do.
- `check_collision_and_update`: Checks collision conditions using coordinate calculations and updates state flags based on the results, then performs data copying operations.
- `validate_position_and_modify`: Validates position coordinates against boundary conditions and modifies object state variables when validation passes, including updating bit flags and memory locations.
- `set_bit_flag_and_process`: Sets a bit flag in a data structure and processes related data by copying values between memory locations, handling multiple iterations until a flag is found.
- `initialize_object_state`: Initializes various state fields of a structure based on conditional checks of its current value and related data, setting flags and updating multiple fields accordingly.
- `update_object_movement`: Updates movement parameters for structures in an array by decrementing counters, performing lookups in tables, and modifying position values based on calculated results.
- `process_character_rendering`: Processes character rendering data by calculating positions, updating graphics registers, and handling character state transitions based on input data and conditional logic.
- `update_position_data`: Processes movement coordinates and boundary checks for two data structures based on input flags, updating position values and status bytes while handling overflow conditions.
- `check_joystick_input`: Reads joystick status from CIAA_PRA and performs conditional operations based on bit tests of multiple global structure offsets, updating flags in the structure and calling another routine when specific bits are set.
- `initialize_structure_with_values`: Initializes a data structure with specific values from another location when a condition is met, then updates the structure with calculated values and calls a processing routine.
- `update_animation_data`: Updates animation frame data based on a counter value and stores the result in a global variable.
- `initialize_structure_fields`: Writes predefined byte and word values to specific offsets within a structure, effectively initializing multiple fields of that structure.
- `copy_and_transform_data`: Copies data from one memory location to another while transforming values through table lookups, then continues processing until a condition is met.
- `frag_lea_moveq_tst_beq_adda_dbra_rts`: Short code fragment that loads the address of the memory at an offset from a5 into a0, then loads 47 into d0, then tests the memory at a0, then branches if the last result was zero, then adds 24 to a0, then loops back while counting down d0, then returns.
- `setup_graphics_parameters`: Sets up graphics parameters by writing specific values to memory locations and calling rendering routines with calculated coordinates and values.
- `convert_number_to_digits`: Converts a number into its three-digit decimal representation and stores each digit as an ASCII character in consecutive memory locations.
- `wait_and_setup_graphics`: Waits for a specific bit to clear in a global structure, then sets up various graphics-related parameters including window coordinates and color values.
- `process_data_with_loop`: Processes data in a loop by reading from memory, performing calculations, and writing results to another location, handling multiple iterations with conditional checks.
- `compute_sprite_coordinates`: Calculates sprite position coordinates based on input parameters and stores the result in memory.
- `compute_sprite_position`: Calculates sprite position coordinates based on input parameters and stores the result in memory.
- `update_collision_data`: Processes collision-related coordinates and dimensions from a structure, performs arithmetic operations including shifts, multiplies, and additions on these values, and updates global variables with computed results.
- `decrement_counters`: Decrements several byte counters in a data structure and clears specific bits in status bytes when counters reach zero.
- `clear_collision_entries`: Clears entries from a collision detection table based on comparison with stored values, iterating through 15 entries.
- `update_position_data_2`: Updates position coordinates in a data structure based on bit test conditions and stores results in memory locations.
- `process_graphics_loop`: Processes graphics data by loading word values from memory, performing bit shifts, and calling another routine in a loop.
- `frag_bsr_lea_btst_bne_bra`: Short code fragment that calls another routine, then loads the address of the memory at an offset from a5 into a0, then calls another routine, then loads the address of the memory at an offset from a5 into a0, then tests bit 4 of the memory at an offset from a5, then branches if the last result was not zero, then calls another routine, then jumps to another point in the code.
- `process_multiple_flags`: Processes multiple bit flag conditions by calling a subroutine with different parameters based on which flags are set, then stores a byte from a register through a pointer.
- `handle_bit_flags`: Processes multiple bit flag conditions and calls a subroutine with different parameters based on which flags are set.
- `frag_tst_bne_addq_bra`: Short code fragment that tests the memory at a0, then branches if the last result was not zero, then adds 4 to a0, then jumps to another point in the code.
- `frag_add_sub_tst_bne_addq_bra`: Short code fragment that adds d7 to the memory at a0 (advancing a0), then subtracts d6 from the memory at a0 (advancing a0), then tests the memory at a0, then branches if the last result was not zero, then adds 4 to a0, then jumps to another point in the code.
- `frag_add_addq_tst_bne_bra`: Short code fragment that adds d7 to the memory at a0 (advancing a0), then adds 2 to a0, then tests the memory at a0, then branches if the last result was not zero, then adds 4 to a0, then jumps to another point in the code.
- `tiny_add_add_rts`: Small routine of 3 instructions that adds d7 to (a0)+, then adds d6 to (a0)+, then returns.
- `frag_tst_bne_addq_bra_2`: Short code fragment that tests the memory at a0, then branches if the last result was not zero, then adds 4 to a0, then jumps to another point in the code.
- `frag_sub_tst_bne_addq_bra`: Short code fragment that subtracts d7 from the memory at a0 (advancing a0), then subtracts d6 from the memory at a0 (advancing a0), then tests the memory at a0, then branches if the last result was not zero, then adds 4 to a0, then jumps to another point in the code.
- `frag_sub_addq_tst_bne_bra`: Short code fragment that subtracts d7 from the memory at a0 (advancing a0), then adds 2 to a0, then tests the memory at a0, then branches if the last result was not zero, then adds 4 to a0, then jumps to another point in the code.
- `tiny_sub_add_rts`: Small routine of 3 instructions that subtracts d7 from (a0)+, then adds d6 to (a0)+, then returns.
- `initialize_memory_blocks`: Initializes memory blocks by copying values from a source location, performing arithmetic operations, and storing results in various data locations.
- `copy_structure_data`: Copies data from a source structure to a target structure using multiple memory moves and calculates offsets for specific entries.
- `frag_btst_beq_bsr_rte`: Short code fragment that tests bit 5 of the memory at an offset from a6, then branches if the last result was zero, then calls another routine (2 times in a row), then returns from the exception.
- `update_game_state_2`: Updates game state variables by comparing values, setting flags, and copying data between memory locations based on comparison results, then returns.
- `update_game_state`: Updates game state variables by comparing values, setting flags, and copying data between memory locations based on comparison results.
- `frag_move_bra`: Short code fragment that copies 850 to d0, then jumps to another point in the code.
- `process_render_data`: Processes rendering data by copying values from a source location to a target structure and incrementing a counter.
- `frag_bsr_move_rts`: Short code fragment that calls another routine (3 times in a row), then copies 32 to the memory at an offset from a6, then returns.
- `swap_and_update_data`: Swaps data between two memory locations, updates various counters and flags, and performs bit operations on specific data entries.
- `frag_movea_moveq_move_bpl_lea_bra`: Short code fragment that copies the memory at an offset from a5 to a0, then loads 0 into d0, then copies the memory at a0 (advancing a0) to d0, then branches if the last result was not negative, then loads the address of a location in the code into a0, then copies a0 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_lsl_lea_btst_beq_bra`: Short code fragment that shifts d0 left by 3, then loads the address of the memory at an offset from a5 into a0, then tests bit 3 of the memory at an offset from a5, then branches if the last result was zero, then loads the address of the memory at an offset from a5 into a0, then jumps to another point in the code.
- `clear_and_copy_data`: Clears two word-sized values at specific offsets in a data structure, then copies longword values from a buffer to two locations in the same structure while incrementing a counter.
- `frag_tst_bne_clr_rts`: Short code fragment that tests d0, then branches if the last result was not zero, then tests d1, then branches if the last result was not zero, then clears the memory at an offset from a0, then returns.
- `process_and_store_coordinates`: Processes coordinate values by applying bit shifts, masks, and arithmetic operations before storing the results as word values in memory.
- `calculate_and_store_coordinates_2`: Calculates coordinate values by applying bit shifts, masks, and arithmetic operations before storing the results as word values in memory, then returns.
- `fill_and_initialize_buffer`: Fills a buffer with a series of predefined values including longwords and words, copying data from a table until reaching a termination marker.
- `initialize_object_data`: Reads object configuration flags from a table and writes structured data to memory locations pointed to by another table, handling different data sizes based on flag bits.
- `initialize_object_data_2`: Initializes object data by copying structured information from multiple source locations into target buffers, setting up pointers and clearing specific memory locations during the process.
- `initialize_globals`: Sets specific word values in a global structure at fixed offsets and performs a series of operations involving memory addresses and another routine call.
- `decode_and_copy_data`: Decodes data by comparing magic numbers, performing bit operations on byte values, and copying decoded data to a target location with loop control, then returns.
- `validate_and_copy_data`: Validates data by checking for specific byte sequences, calculates an address offset, copies data to a target location with loop control, then returns.
- `initialize_and_process_entries`: Initializes structure fields with values from another structure when a specific condition is met, clears certain flag bits, then returns.
- `frag_btst_bne_move_rts`: Short code fragment that tests bit 6 of the memory at an offset from a6 (2 times in a row), then branches if the last result was not zero, then copies 2544 to the memory at an offset from a6, then copies 0 to the memory at an offset from a6, then copies 198 to the memory at an offset from a6, then copies 82 to the memory at an offset from a6, then copies an address to the memory at an offset from a6, then returns.
- `calculate_and_store_addresses`: Computes memory addresses based on division and multiplication operations, then stores these addresses along with a fixed value into specific offsets of a structure.
- `process_and_store_coordinates_2`: Processes coordinate values by applying bit shifts, masks, and arithmetic operations before storing the results as word values in memory, then returns.
- `update_counters`: Compares a long value with a constant and updates byte and word counters based on conditional branches, potentially modifying global state through subroutine calls.
- `initialize_structure_fields_2_2`: Initializes various fields of a structure with data from another structure when a specific condition is met, including clearing certain flag bits, then returns.
- `update_and_process_entries`: Processes entries in a table by updating their values based on differences between current and stored values, then performs conditional updates and address adjustments before continuing to the next entry.
- `frag_clr_move_rts`: Short code fragment that clears the memory at an offset from a6 (4 times in a row), then copies 15 to the memory at an offset from a6, then returns.
- `frag_movem_movea_bsr_rts`: Short code fragment that saves registers on the stack, then copies an address to a6, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_bsr_addq_move_cmp_ble_rts`: Short code fragment that calls another routine, then adds 1 to the memory at an offset from a5, then copies the memory at an offset from a5 to d0, then compares d0 with the memory at an offset from a5, then branches if the comparison was less or equal, then returns.
- `update_counter_and_address`: Clears a word value at offset 42 of a5, increments a word counter at offset 46 of a5, compares it to 65, and if equal, updates several counters and calculates a new address from a table before branching to another routine.
- `frag_clr_move_addi_bra`: Short code fragment that clears the memory at an offset from a5, then copies the memory at an offset from a5 to the memory at an offset from a5, then adds 242 to the memory at an offset from a5, then jumps to another point in the code.
- `process_object_data`: Processes four data fields from a structure by loading each field into d0, setting a counter in d2, calling two subroutines, and storing a final counter value at offset 150 of a6.
- `frag_move_andi_tst_bne_rts`: Short code fragment that copies d0 to d5, then masks d0 with 61440, then tests d0, then branches if the last result was not zero, then returns.
- `transform_and_store_data`: Transforms a word value by swapping bytes, masking and shifting bits, calculates an address from a table, and stores multiple values from that address to various global offsets based on a counter value.
- `copy_and_clear`: Checks if a register equals two, clears a word value at an offset, and copies data from a pointer to multiple memory locations with specific offsets.
- `copy_and_update_values`: Copies values between memory locations based on comparison results, clears a word value at an offset, then returns.
- `initialize_structure_fields_2`: Initializes various fields of a structure with data from another structure when a specific condition is met, including clearing certain flags and copying word-sized values from source to destination offsets.
- `route_by_masked_value`: Processes a word value by masking its lower and upper bytes, compares the masked value against several constants, and stores or modifies global variables based on the comparison results.
- `frag_cmp_bne_move_rts`: Short code fragment that compares d6 with 2048, then branches if the last result was not zero, then copies d5 to the memory at an offset from a5, then returns.
- `frag_cmp_bne_bclr_rts`: Short code fragment that compares d6 with 1536, then branches if the last result was not zero, then clears bit 1 of a fixed memory address, then returns.
- `set_flag_if_register_matches`: If a register equals a specific value, sets bit 1 of a global variable.
- `calculate_and_store_addresses_2`: Calculates memory addresses based on division and multiplication operations, stores these addresses along with a fixed value into specific locations, then returns.
- `frag_neg_cmp_bne_move_rts`: Short code fragment that negates d5, then compares d2 with 1, then branches if the last result was not zero, then copies d5 to the memory at an offset from a5, then returns.
- `frag_cmp_bne_move_rts_2`: Short code fragment that compares d2 with 2, then branches if the last result was not zero, then copies d5 to the memory at an offset from a5, then returns.
- `frag_cmp_bne_move_rts_3`: Short code fragment that compares d2 with 3, then branches if the last result was not zero, then copies d5 to the memory at an offset from a5, then returns.
- `frag_tst_bne_move_rts`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was not zero, then copies d5 to the memory at an offset from a5, then returns.
- `frag_cmp_bne_move_rts_4`: Short code fragment that compares d2 with 1, then branches if the last result was not zero, then copies d5 to the memory at an offset from a5, then returns.
- `frag_cmp_bne_move_rts_5`: Short code fragment that compares d2 with 2, then branches if the last result was not zero, then copies d5 to the memory at an offset from a5, then returns.
- `frag_cmp_bne_move_rts_6`: Short code fragment that compares d2 with 3, then branches if the last result was not zero, then copies d5 to the memory at an offset from a5, then returns.
- `frag_tst_bne_move_rts_2`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was not zero, then copies d5 to the memory at an offset from a5, then returns.
- `update_position_and_flags`: Updates position values in a structure and sets flags based on conditions, then stores the combined flags word to another structure.
- `frag_movem_bsr_rts`: Short code fragment that saves registers on the stack, then calls another routine, then restores saved registers from the stack, then returns.
- `process_data_with_tables`: Processes input data using lookup tables and bit manipulation operations, performing conditional copying and arithmetic calculations based on input values.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (moveq) and then falls through into the code that follows, with no return of its own.
- `step_byte_with_space_wrap`: Decrements a byte in the data register a counted number of times, replacing a wrapped zero value with a space character.

### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `initialize_system_components`: Initializes system components by setting up interrupt vectors and configuring hardware registers, then enables all interrupts.
- `frag_move_add_btst_beq`: Short code fragment that copies a location in the code to d0, then adds an address to d0, then tests bit 1 of d1, then branches if the last result was zero.
- `return_constant`: Returns the constant 1024 in d0 to the caller without any other effect.
- `frag_move_movem_lea_movea_adda_moveq_js`: Short code fragment that copies 32767 to the memory at an offset from a6, then saves registers on the stack, then loads the address of a location in the code into a0, then copies a location in the code to a1, then adds an address to a1, then copies a location in the code to a2, then loads 112 into d0, then calls another routine, then restores saved registers from the stack, then returns.
