# Game 3585 (Eye v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03585). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1094506 top-level bytes in 33 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 97 of 97 named (0 author symbols), 97 of 97 described (22 by rule, 75 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4180 of 4180 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 412451 |
| code | 19738 |
| data | 662281 |
| padding | 36 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| eye | 1 | 4178 | 96 |

## Cases handled

- Eye_v1.0/Eye_v1.0.hdf: hard-disk image (AmigaDOS OFS, 1248 blocks): its filesystem parses and all 31 files in it are byte-identical to the extracted tree Eye_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### eye

- `initialize_system`: Initializes system resources by opening and closing the exec library, setting up global variables, configuring hardware registers, and performing a delay loop before returning.
- `update_object_parameters`: Initializes object state by setting default values and clearing tile data, performs conditional object setup based on bit comparisons, and copies object parameters between different memory locations.
- `initialize_objects_2`: Initializes object parameters by setting a default value, calling setup routines, and iterating through a counter to process multiple objects.
- `check_game_state`: Compares memory values against specific constants and updates a global variable with different values based on the comparison results.
- `process_tile_graphics`: Processes tile graphic data by reading bytes from a source table, applying bitwise operations, and writing results to two destination buffers with specific offsets.
- `frag_movea_move_dbra_rts`: Short code fragment that copies 30938 to a0, then copies an address to a1, then copies 31 to d0, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then returns.
- `save_processor_state`: Saves processor status register, sets interrupt mask, reads from memory, calls a handler routine, then restores the saved processor status.
- `restore_processor_state`: Saves processor status register, sets interrupt mask, reads from memory, calls a handler routine, then restores the saved processor status with different interrupt settings.
- `update_game_state`: Initializes multiple game object properties including position, velocity, and flags, then performs conditional updates based on comparison results with fixed values.
- `calculate_distance`: Computes a normalized distance value from two input bytes by performing subtraction, absolute value calculation, and clamping operations to a maximum of 16.
- `initialize_object`: Checks a global flag and if clear, sets up object parameters including position, size, and status flags before entering a loop that calls another routine 16 times.
- `setup_render_parameters`: Configures rendering parameters for an object by setting memory addresses, width, height, and status flags, then executes a loop calling another routine 12 times.
- `save_restore_interrupts`: Saves the current processor status register, sets interrupt mask to 9984, updates a global variable with a value from memory, then restores the original status register after two loops.
- `update_statistics`: Clears and initializes a statistics table, processes data from another table by incrementing counters based on byte values, then finds and stores the maximum counter value.
- `initialize_statistics`: Clears a multi-dimensional statistics table, processes data from a memory region by incrementing counters based on byte values, and updates the table through multiple nested loops.
- `process_tile_interactions`: Processes tile graphic data by reading bytes from a source table, applying bitwise operations, and writing results to two destination buffers.
- `update_object_state`: Updates object state by processing a series of data entries, performing bit manipulation on memory locations, and modifying object flags based on conditions.
- `frag_bsr_rts`: Short code fragment that calls another routine, then returns.
- `update_input_state`: Reads input state from a global variable, updates counters in multiple global variables, and processes input data through table lookups when counters reach zero.
- `process_input_sequence_2`: Processes input sequence by reading from tables, updating global variables with computed values, and performing conditional arithmetic operations.
- `process_input_sequence`: Processes input data by reading from tables, updates global variables with computed values, and performs conditional arithmetic operations based on comparison results.
- `process_input_data`: Processes input data by reading from memory tables, updating global variables with computed values, and performing conditional arithmetic operations based on comparison results.
- `update_keyboard_state`: Reads a byte from the keyboard serial register, rotates it right by one bit, inverts all bits, and stores the result to a global variable.
- `initialize_input_tables`: Initializes two data tables by copying values from memory locations pointed to by a1 and a2 into a global variable, then processes input bytes.
- `initialize_data_tables`: Initializes two data tables by copying values from memory locations pointed to by a1 and a2 into a global variable, then processes input bytes from a0 using a lookup table to update another global variable based on conditional logic.
- `frag_movea_tst_bpl_btst_beq_bra`: Short code fragment that copies an indexed entry from a2 to a4, then tests an indexed entry from a3, then branches if the last result was not negative, then copies 31382 to a5, then tests bit 0 of an indexed entry from a3, then branches if the last result was zero, then copies 31450 to a5, then jumps to another point in the code.
- `frag_beq_movea_btst_bra`: Short code fragment that branches if the last result was zero, then copies 31262 to a5, then tests bit 0 of an indexed entry from a3, then branches if the last result was zero, then copies 31322 to a5, then jumps to another point in the code.
- `process_object_data`: Reads object configuration data and writes processed values to output buffers, managing two separate buffer pools based on bit flags, and updates index counters for both pools.
- `frag_move_addi_andi_dbra_movem_rte`: Short code fragment that copies a4 to an indexed entry from a2, then adds 4 to d6, then masks d6 with 12, then loops back while counting down d7, then restores saved registers from the stack, then copies 16 to a fixed memory address, then copies the top of the stack to d0, then returns from the exception.
- `check_condition_loop`: Iterates through a 3x5 grid of byte structures, checking for specific conditions in each structure and setting a global flag when conditions are met.
- `check_zero_condition`: Iterates through a 3x5 grid of byte structures, checking for specific conditions where the third byte equals zero and setting a global flag when conditions are met.
- `update_positions_forward`: Processes a data structure array by checking for end markers, updating position values with bitwise operations, and calling another routine before advancing to the next element.
- `update_position_fields`: Updates two fields in a data structure based on values from adjacent fields, with bitwise operations applied to the results, then calls another routine before proceeding to the next structure entry.
- `update_structure_fields`: Updates specific fields in a data structure based on values from adjacent elements, with bitwise operations applied to the results before storage.
- `update_tile_data_2`: Processes tile data by reading specific byte values, performing bitwise calculations, and storing results at fixed offsets within each tile structure.
- `calculate_and_store_values`: Calculates values based on array indices and stores results in structure fields while modifying global data arrays with bitwise operations.
- `copy_structure_fields`: Copies specific byte fields from a 3x5 grid of structures to corresponding positions in another structure, then copies the first two bytes to a target address.
- `update_and_compare_values`: Updates byte values in a 3x5 grid of structures based on comparison results with other fields, then writes updated values to a target address.
- `update_global_flag`: Sets a global variable to -1, then conditionally updates it based on a check of another global variable, and performs a series of operations involving table lookups and counter decrements.
- `process_object_data_3`: Updates object parameters and processes a data structure array, checking for end markers and performing bitwise operations on position values before calling another routine.
- `update_position_fields_2`: Updates two fields in a data structure based on values from adjacent fields, with bitwise operations applied to the results, then calls another routine to continue processing.
- `initialize_object_loop`: Initializes an object by setting a global variable to -1, then conditionally executing a loop that updates counters and processes data based on comparisons.
- `initialize_level_data`: Copies graphic data blocks from multiple source addresses to destination addresses, updates global variables with specific values, and performs conditional updates to memory locations based on comparison results.
- `initialize_graphics_data`: Calls another routine to set up graphics data and stores various register values into global memory locations with fixed offsets.
- `frag_move_andi_beq_rts`: Short code fragment that copies d0 to the stack, then copies a fixed memory address to d0, then masks d0 with 4, then branches if the last result was zero, then copies 4 to a fixed memory address, then copies the top of the stack to d0, then returns.
- `frag_move_andi_bne_rts`: Short code fragment that copies d0 to the stack, then copies a fixed memory address to d0, then masks d0 with 16384, then branches if the last result was not zero, then copies the top of the stack to d0, then returns.
- `update_position_bounds`: Updates position values with boundary checks and calls another routine to process differences between values.
- `calculate_difference_and_limit`: Calculates the absolute difference between two byte values, limits it to a maximum of four, and handles negative results by negating them.
- `setup_object_parameters`: Sets up object parameters including coordinates, dimensions, and flags in global memory locations based on input structure values.
- `frag_bsr_neg_move_rts`: Short code fragment that calls another routine, then negates a fixed memory address, then copies an address to a fixed memory address, then returns.
- `update_position`: Updates position coordinates based on target values and stores resulting byte values at specific memory offsets, then decrements a counter and performs conditional adjustments to the coordinates.
- `initialize_multiple_objects`: Initializes multiple objects by calling a setup routine with different parameter combinations and setting various global memory locations with fixed values.
- `initialize_objects`: Initializes multiple objects by setting various memory locations with specific values and calling a setup routine.
- `conditional_object_setup`: Sets up object parameters conditionally based on bit tests of a lookup table value, calling a setup routine multiple times with different configurations.
- `copy_and_process_loop`: Copies data from one memory location to another in a loop, updating pointers and counters while making library calls for each iteration.
- `initialize_video_and_input`: Initializes video hardware registers with fixed values, sets up input handling by opening and closing system libraries, and configures display parameters including screen dimensions and color settings.
- `initialize_and_update`: Initializes hardware registers with fixed values, then repeatedly calls another routine while incrementing pointers and data values.
- `frag_move_bsr_rts`: Short code fragment that copies a0 to a fixed memory address, then copies d0 to a fixed memory address, then copies d7 to a fixed memory address, then calls another routine, then returns.
- `bitwise_compare_and_modify`: Copies word values between memory locations, decrements related counters, and performs bitwise operations on data with conditional modifications.
- `word_comparison_and_increment`: Compares words from two memory locations, performs bitwise masking comparisons, and increments values based on comparison results in a loop.
- `string_search_and_assign`: Searches for specific character sequences in memory, extracts data from found positions, and assigns the extracted values to global variables.
- `update_sprite_data`: Processes sprite-related data by updating counters, checking conditions, and modifying memory locations based on comparisons.
- `initialize_sprite_arrays`: Initializes a global array with zero values and sets up sprite-related data structures by processing input coordinates and updating counters in a table.
- `data_accumulation_and_update`: Processes data by accumulating values, updating counters based on comparisons, and modifying memory locations with calculated results.
- `process_sprite_data`: Reads sprite configuration from a global variable, computes movement offsets based on input coordinates, and writes updated sprite data to a memory structure.
- `check_conditions`: Processes a table of byte values and checks if the sum of certain flags equals one, storing a result in a global variable when conditions are met.
- `frag_move_ext_lsr_addi_rts`: Short code fragment that copies a fixed memory address to d0, then sign-extends d0, then shifts d0 right by 1, then adds 143 to d0, then copies the memory at an offset from a5 to d7, then copies d0 to an indexed entry from a5, then copies a fixed memory address to d0, then adds 151 to d0, then copies d0 to an indexed entry from a5, then returns.
- `process_data_block`: Iterates through a data block, compares specific bit patterns with values from another location, and updates multiple global variables based on matching conditions.
- `process_tile_updates`: Updates tile graphic data by modifying bit patterns in a source table, processes conditional tile state changes based on comparison results, and handles tile interaction logic for multiple tile groups.
- `update_bit_fields`: Processes bit fields in three data structures, modifying bits based on a global variable's value and a fixed pattern, then updates the structures with modified values.
- `handle_game_objects`: Handles game objects by setting up memory locations, calling routines to process data, and executing a jump table based on conditions.
- `initialize_game_objects`: Sets up initial positions and properties for three game objects, then copies their data to global variables at specific memory locations.
- `process_input_byte`: Reads a byte from a global variable, compares it against specific values, and conditionally updates a counter and memory locations based on the byte's value.
- `frag_cmp_bne_move_subi_bpl_movea_bra`: Short code fragment that compares d0 with 65, then branches if the last result was not zero, then copies a fixed memory address to d6, then subtracts 1 from d6, then branches if the last result was not negative, then copies 0 to d6, then copies 22382 to a0, then copies 32 to an indexed entry from a0, then copies d6 to a fixed memory address, then jumps to another point in the code.
- `frag_cmp_bne_btst_beq_bra`: Short code fragment that compares d0 with 68, then branches if the last result was not zero, then tests bit 7 of a fixed memory address, then branches if the last result was zero, then jumps to another point in the code.
- `frag_movea_move_bsr_tst_bpl_bra`: Short code fragment that copies 22382 to a0, then copies an address to d0, then copies 7 to d7, then calls another routine, then tests a fixed memory address, then branches if the last result was not negative, then jumps to another point in the code.
- `initialize_video_system`: Initializes video system registers by loading values from global variables, setting up display modes, and configuring hardware state.
- `frag_movea_move_bsr_bra`: Short code fragment that copies 27376 to a0, then copies an address to d0, then copies 24 to d7, then calls another routine, then copies 27401 to a0, then copies an address to d0, then copies 24 to d7, then calls another routine, then jumps to another point in the code.
- `frag_movea_move_bsr_bra_2`: Short code fragment that copies 27326 to a0, then copies an address to d0, then copies 24 to d7, then calls another routine, then copies 27351 to a0, then copies an address to d0, then copies 24 to d7, then calls another routine, then jumps to another point in the code.
- `initialize_game_objects_2`: Sets up two memory regions with specific values by calling a routine to process data, then iterates 100 times performing a memory operation and a conditional call based on a global variable's state.
- `initialize_text_buffer`: Initializes multiple memory locations with space characters, updates a global variable based on input conditions, and sets up text input handling for a buffer structure.
- `frag_cmp_bne_move_subi_bpl_movea_bra_2`: Short code fragment that compares d0 with 65, then branches if the last result was not zero, then copies a fixed memory address to d6, then subtracts 1 from d6, then branches if the last result was not negative, then copies 0 to d6, then copies a fixed memory address to a0, then copies 32 to an indexed entry from a0, then copies d6 to a fixed memory address, then jumps to another point in the code.
- `frag_cmp_bne_btst_beq_bra_2`: Short code fragment that compares d0 with 68, then branches if the last result was not zero, then tests bit 7 of a fixed memory address, then branches if the last result was zero, then jumps to another point in the code.
- `frag_movea_move_bsr_tst_bpl_bra_2`: Short code fragment that copies a fixed memory address to a0, then copies an address to d0, then copies 4 to d7, then calls another routine, then tests a fixed memory address, then branches if the last result was not negative, then jumps to another point in the code.
- `validate_header_and_process_input`: Checks for specific magic strings in memory and updates a flag based on the comparison results.
- `update_input_buffer`: Updates a buffer with input data by reading from a lookup table and writing bytes to memory locations with conditional logic based on input values.
- `frag_cmp_bne_move_subi_bpl_movea_bra_3`: Short code fragment that compares d0 with 65, then branches if the last result was not zero, then copies a fixed memory address to d6, then subtracts 1 from d6, then branches if the last result was not negative, then copies 0 to d6, then copies 24914 to a0, then copies 32 to an indexed entry from a0, then copies d6 to a fixed memory address, then jumps to another point in the code.
- `frag_cmp_bne_btst_beq_bra_3`: Short code fragment that compares d0 with 68, then branches if the last result was not zero, then tests bit 7 of a fixed memory address, then branches if the last result was zero, then jumps to another point in the code.
- `frag_movea_move_bsr_tst_bpl_bra_3`: Short code fragment that copies 24914 to a0, then copies an address to d0, then copies 2 to d7, then calls another routine, then tests a fixed memory address, then branches if the last result was not negative, then jumps to another point in the code.
- `parse_numeric_string`: Converts a three-character numeric string from memory into a word value, with bounds checking and special handling for zero values.
- `process_input_byte_2`: Processes an input byte value by checking against specific constants and setting or clearing bits in multiple memory locations based on comparison results.
- `update_object_positions`: Updates object positions by adding offsets, performs bit manipulation on memory locations, and calls a handler routine multiple times while incrementing counters.
- `process_object_data_2`: Calculates new object coordinates with offset additions, stores data to global variables, and calls a handler routine repeatedly while decrementing a counter.
- `update_tile_data`: Processes input byte data by shifting bits, reads from a lookup table, and writes inverted or normal values to three memory locations with offset calculations.
- `initialize_game_state`: Sets up initial game object data by copying object properties to global variables, configures system flags, and performs multiple initialization calls for different game components.
- `process_input_buffer`: Processes input bytes from a global variable, updates a text buffer at offset 576E based on key mappings, and handles cursor movement and backspace operations.
