# Game 3728 (FrostByte v0501.1 1529): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03728). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (697972 top-level bytes in 21 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 163 of 163 named (0 author symbols), 163 of 163 described (79 by rule, 82 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 2704 bytes, 12 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2763 of 2763 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 269306 |
| code | 12924 |
| data | 414864 |
| padding | 878 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 8 | 3 |
| .prg | 1 | 2611 | 148 |
| sp (raw code file, load address unknown) | 1 | 144 | 12 |

## Cases handled

- FrostByte_v0501.1_1529/FrostByte_v0501.1_1529.hdf: hard-disk image (AmigaDOS OFS, 768 blocks): its filesystem parses and all 19 files in it are byte-identical to the extracted tree FrostByte_v0501.1_1529_hdf, which is recovered file by file; no block outside the filesystem holds data
- FrostByte_v0501.1_1529/FrostByte_v0501.1_1529_hdf/FrostByte/data/sp: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 3 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 jump target, 1 immediate

## Cracktros (kept, not protection)

- none

## Copy protection found

- `setup_audio_dma_channels` (.prg): Configures audio DMA channels by copying data from a table to specific hardware registers, sets up multiple channel parameters, and enables DMA with specific bit patterns. Kind: disk hardware check. Pass path: The routine sets up DMA channels and ADKCON register, suggesting a check on disk hardware or track integrity. The call from sub_4D2C implies this is part of a validation routine that must pass for the program to continue execution..

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_pea_bra`: Short code fragment that pushes the address of a fixed memory address onto the stack (2 times in a row), then jumps to another point in the code.
- `frag_move_addq_rts`: Short code fragment that copies a location in the code to the stack, then adds 4 to the top of the stack, then returns.

### .prg

- `frag_move_bsr`: Short code fragment that copies a7 to a fixed memory address, then calls another routine (2 times in a row).
- `copy_and_expand_data`: Copies 4000 words from a source to four different destination tables, then copies 8000 words from the first destination table to a new location pointed to by the stack.
- `process_data_table_entries`: Iterates through a data table, processes entries by calling another routine with calculated parameters, and then calls multiple routines with fixed addresses.
- `copy_word_pairs`: Copies word pairs from a source area to two destination areas, advancing pointers until reaching a specific address limit.
- `clear_memory_blocks`: Clears word values at multiple offsets within two memory blocks, iterating through a loop of 160 iterations.
- `initialize_color_table`: Initializes a color table by clearing memory locations and copying word values from source tables, then sets multiple global variables to specific values before returning.
- `initialize_game_state_2`: Initializes multiple global variables to zero or specific values, performs a delay loop, and calls other routines to set up game state.
- `process_game_state`: Checks global variables for specific values and calls other routines based on conditional branches, performing operations like incrementing counters and updating memory locations.
- `initialize_game_state`: Initializes multiple global variables to zero or specific values, performs a delay loop, and calls other routines to set up game state.
- `update_counter_state`: Toggles a global flag and updates another global counter based on conditions involving other global variables.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `frag_move_rts`: Short code fragment that copies a5 to a fixed memory address (2 times in a row), then returns.
- `frag_move_cmpi_bne_rts`: Short code fragment that copies 0 to a fixed memory address, then copies d1 to a fixed memory address, then compares a fixed memory address with 65535, then branches if the last result was not zero, then copies 0 to a fixed memory address, then returns.
- `frag_move_rts_2`: Short code fragment that copies an address to a fixed memory address (2 times in a row), then returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `frag_movea_lsl_andi_move_eori_bsr_rts`: Short code fragment that copies 2902 to a0, then shifts d0 left by 2, then masks d0 with 7, then copies an indexed entry from a0 to d1, then exclusive-ors 4 into d0, then copies an indexed entry from a0 to a fixed memory address, then calls another routine, then returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (or.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `frag_move_subq_rts`: Short code fragment that copies 65535 to a fixed memory address, then subtracts 1 from a fixed memory address, then returns.
- `frag_cmpi_ori`: Short code fragment that compares a fixed memory address with 16, then merges 176 into d6, then merges 27 into the memory at a0, then merges 56 into d6, then merges 28 into the memory at a0.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs two instructions (ori.b, ori.b) and then falls through into the code that follows, with no return of its own.
- `frag_ori`: Short code fragment that merges 124 into d4, then merges 15 into the memory at a4, then merges 76 into d4, then merges 15 into the memory at a4.
- `validate_and_assign`: Validates data from a table and assigns a word value to a global variable if conditions are met.
- `iterate_and_execute`: Initializes a counter to zero, executes a loop that processes data and calls another routine until the counter reaches sixteen iterations.
- `process_object_data`: Processes object data by reading from tables, performing calculations, and writing structured data to memory locations.
- `copy_word_block`: Copies word values from a source to multiple destinations in a structured pattern, advancing pointers through nested loops.
- `handle_data_structure_iteration`: Processes entries in a data structure by comparing values, advancing pointers through nested loops, and calling another routine with specific parameters based on the iteration count.
- `frag_tst_bpl_move_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was not negative, then copies 1 to a fixed memory address, then returns.
- `validate_and_process_data`: Clears a global variable, iterates through memory locations with specific offsets, and performs bit operations on data values before calling another routine.
- `frag_bsr`: Short code fragment that calls another routine.
- `update_and_jump_conditionally`: Increments a counter, updates a global variable with bit operations, checks conditions in a table, and either jumps to itself or calls another routine based on the comparison results.
- `initialize_object_fields`: Initializes object fields by copying data from tables and performing arithmetic operations on global variables.
- `frag_move_movea_bsr_addi_cmp_blt_rts`: Short code fragment that copies 0 to d6, then copies 2932 to a0, then copies 7804 to a1, then calls another routine, then adds 1 to d6, then compares d6 with 16, then branches if the comparison was less, then returns.
- `update_object_state`: Updates the state of an object based on its type and position, modifying counters, flags, and coordinates in global structures.
- `iterate_and_copy`: Initializes counters, iterates through a nested loop structure, performs calculations using memory values, and copies data between memory blocks when certain conditions are met.
- `frag_movea_move_cmpa_blt_rts`: Short code fragment that copies an address to a0, then copies an address to a1, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then compares a0 with an address, then branches if the comparison was less, then returns.
- `copy_sprite_data`: Copies sprite data from a source address to four destination addresses, advancing the source pointer by 40 bytes after each copy operation.
- `frag_move_clr_rts`: Short code fragment that copies 5 to a fixed memory address, then clears a fixed memory address, then copies 65535 to a fixed memory address, then clears a fixed memory address (2 times in a row), then copies a fixed memory address to a fixed memory address (2 times in a row), then returns.
- `frag_clr_move_rts`: Short code fragment that clears a fixed memory address (2 times in a row), then copies 2 to a fixed memory address, then clears a fixed memory address, then returns.
- `initialize_variables`: Initializes several global variables with values from other global variables and performs bit operations based on conditional checks.
- `update_position_data`: Processes movement coordinates and updates global variables based on conditional checks and arithmetic operations.
- `update_movement_state`: Updates movement state variables based on conditional checks and modifies related counters and flags accordingly.
- `calculate_tile_index`: Calculates a tile index using bit shifts, arithmetic operations, and table lookups to determine a byte value from memory.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d0, then returns.
- `process_table_lookup`: Performs a table lookup based on input values and stores the result in a global variable before jumping to another routine.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `frag_bsr_bra`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_2`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_3`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_2`: Short code fragment that calls another routine (2 times in a row).
- `frag_move_andi_cmp_bgt_bge_clr_rts`: Short code fragment that copies a fixed memory address to d1, then masks d1 with 31, then compares d1 with 18, then branches if the comparison was greater, then compares d1 with 14, then branches if the comparison was greater or equal, then clears d0, then returns.
- `frag_move_andi_cmp_ble_clr_rts`: Short code fragment that copies a fixed memory address to d1, then masks d1 with 31, then compares d1 with 3, then branches if the comparison was less or equal, then clears d0, then returns.
- `frag_move_andi_cmp_bge_rts`: Short code fragment that copies a fixed memory address to d1, then masks d1 with 31, then compares d1 with 16, then branches if the comparison was greater or equal, then returns.
- `frag_move_andi_cmp_ble_rts`: Short code fragment that copies a fixed memory address to d1, then masks d1 with 31, then compares d1 with 15, then branches if the comparison was less or equal, then returns.
- `check_horizontal_bounds`: Checks if horizontal position values are within specified bounds and branches accordingly, otherwise returns immediately.
- `check_vertical_bounds`: Checks if vertical position values are within specified bounds and branches accordingly, otherwise returns immediately.
- `validate_position_range`: Validates position values against multiple range checks and branches based on comparison results, otherwise returns immediately.
- `verify_boundary_conditions`: Verifies boundary conditions for both horizontal and vertical positions and branches based on comparisons, otherwise returns immediately.
- `execute_table_operation`: Performs a table lookup operation and stores the result in a global variable before jumping to another routine.
- `frag_bsr_bra_4`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_5`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_6`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_7`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_cmp_beq_clr_rts`: Short code fragment that compares d0 with 0, then branches if the last result was zero, then clears d0, then returns.
- `return_constant`: Returns the constant 5 in d0 to the caller without any other effect.
- `frag_move_andi_cmp_bge_bra`: Short code fragment that copies a fixed memory address to d1, then masks d1 with 31, then compares d1 with 16, then branches if the comparison was greater or equal, then copies a fixed memory address to d1, then masks d1 with 31, then compares d1 with 18, then branches if the comparison was greater or equal, then jumps to another point in the code.
- `frag_move_andi_cmp_ble_bge_bra`: Short code fragment that copies a fixed memory address to d1, then masks d1 with 31, then compares d1 with 15, then branches if the comparison was less or equal, then copies a fixed memory address to d1, then masks d1 with 31, then compares d1 with 18, then branches if the comparison was greater or equal, then jumps to another point in the code.
- `frag_move_andi_cmp_bge_clr_rts`: Short code fragment that copies a fixed memory address to d1, then masks d1 with 31, then compares d1 with 16, then branches if the comparison was greater or equal, then clears d0, then returns.
- `frag_move_andi_cmp_blt_clr_rts`: Short code fragment that copies a fixed memory address to d1, then masks d1 with 31, then compares d1 with 16, then branches if the comparison was less, then clears d0, then returns.
- `frag_move_andi_cmp_bge_bra_2`: Short code fragment that copies a fixed memory address to d1, then masks d1 with 31, then compares d1 with 16, then branches if the comparison was greater or equal, then jumps to another point in the code.
- `frag_move_andi_cmp_blt_bra`: Short code fragment that copies a fixed memory address to d1, then masks d1 with 31, then compares d1 with 16, then branches if the comparison was less, then jumps to another point in the code.
- `frag_move_andi_cmp_blt_bge_bra`: Short code fragment that copies a fixed memory address to d1, then masks d1 with 31, then compares d1 with 16, then branches if the comparison was less, then copies a fixed memory address to d1, then masks d1 with 31, then compares d1 with 16, then branches if the comparison was greater or equal, then jumps to another point in the code.
- `frag_move_andi_cmp_bge_bra_3`: Short code fragment that copies a fixed memory address to d1, then masks d1 with 31, then compares d1 with 16, then branches if the comparison was greater or equal, then copies a fixed memory address to d1, then masks d1 with 31, then compares d1 with 16, then branches if the comparison was greater or equal, then jumps to another point in the code.
- `decrement_and_process`: Decrements a global counter and performs bitwise operations on table entries when the counter reaches zero, then updates multiple global variables based on conditional checks.
- `update_game_state`: Updates game state variables based on conditional checks and arithmetic operations, then performs table lookups and bit manipulations before returning.
- `initialize_game_variables`: Copies coordinate values from global variables to memory locations, calls other routines for processing, and updates game state variables based on comparison results.
- `process_input_data`: Processes input data based on multiple conditions and stores results in global variables.
- `frag_movem_rts`: Short code fragment that saves registers on the stack, then returns.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `process_data_structure`: Iterates through a data structure, compares values with a global variable, performs calculations using table lookups, and updates memory locations with computed results.
- `initialize_flags`: Initializes flags for a grid of elements by clearing existing flags, checking multiple bit conditions, and setting a specific flag when all conditions are met.
- `update_positions`: Updates position values for grid elements by clearing a flag, adding a velocity value to a position, and resetting the position when it exceeds a threshold.
- `frag_movea_clr_move_addi_cmp_blt_rts`: Short code fragment that copies 16342 to a0, then copies 16438 to a1, then clears d0, then copies an indexed entry from a0 to an indexed entry from a1, then adds 8 to d0, then compares d0 with 96, then branches if the comparison was less, then returns.
- `process_input_data_2`: Checks multiple global variables for zero values and processes input data based on conditions, updating several global variables and tables.
- `check_and_copy_values`: Checks two global variables for zero, copies a value from one to another if the first is not zero, and returns zero or negative one based on the conditions.
- `decrement_and_clear`: Decrements a counter in memory and clears multiple memory locations with zero values in a loop.
- `load_and_shift`: Reads a word value from a global table, performs bit shifting and arithmetic operations on it, then stores the result to another global variable.
- `frag_bsr_move_cmp_blt_rts`: Short code fragment that calls another routine, then copies a fixed memory address to d0, then compares d0 with 10, then branches if the comparison was less, then copies 9 to d0, then calls another routine, then returns.
- `frag_move_bsr_subi_bne_rts`: Short code fragment that copies 999 to a fixed memory address, then calls another routine (2 times in a row), then copies an address to d0, then subtracts 1 from d0, then branches if the last result was not zero, then returns.
- `initialize_and_loop`: Initializes multiple global variables and enters a loop that updates counters, processes flags, and handles conditional branches based on counter values and status bits, repeating until specific conditions are met.
- `copy_memory_blocks`: Copies memory blocks between three different memory regions by copying word values from one location to another.
- `update_data_structure`: Updates a data structure by comparing a value with a threshold, storing results in global variables, and copying large blocks of memory between fixed addresses.
- `frag_clr_rts`: Short code fragment that clears a fixed memory address (2 times in a row), then returns.
- `update_digit_fields`: Updates digit values in a table based on input from a register, with carry propagation and modulo operation.
- `update_pixel_fields`: Updates pixel bit fields in memory locations based on conditions and bitwise operations with global variables.
- `update_bit_fields`: Updates bit fields in memory locations based on a condition check and bitwise operations.
- `frag_adda_subq_bne_addi_eori_cmp_blt_rt`: Short code fragment that adds 40 to a2, then adds 40 to a3, then adds 40 to a4, then subtracts 1 from d6, then branches if the last result was not zero, then adds 1 to d0, then exclusive-ors 1 into a fixed memory address, then compares d0 with 5, then branches if the comparison was less, then returns.
- `clear_memory_blocks_2`: Clears memory blocks by setting multiple word values to zero.
- `copy_word_values`: Copies word values between memory locations in a loop.
- `process_loop_iteration`: Processes a loop iteration by calling another routine, checking a condition, and continuing until a counter reaches zero.
- `copy_long_values`: Copies long values between memory locations in a loop.
- `clear_bit_fields`: Clears specific bit fields in memory locations by performing bitwise AND operations with a constant value.
- `update_memory_regions`: Updates two memory regions by setting specific bits in a loop that iterates fifteen times, advancing pointers by thirty-two bytes each iteration.
- `frag_move_rts_3`: Short code fragment that copies 12 to a fixed memory address, then copies 50 to a fixed memory address, then copies 65535 to a fixed memory address, then returns.
- `update_graphics_data`: Decrements a counter in a global variable and branches if it's not zero.
- `reset_install_counters`: Sets three state variables to zero and returns nothing.
- `validate_and_reset`: Validates conditions against memory values and resets specific fields to zero when conditions are met.
- `process_input_condition`: Checks if a specific memory location contains a particular value, clears that location, and performs multiple memory copy operations between several address ranges.
- `check_input_conditions`: Checks if a global variable equals 32, then verifies multiple input conditions and returns different values based on matches.
- `open_libraries_sequentially`: Attempts to open two system libraries sequentially, storing the result in global variables if the operation succeeds.
- `frag_bsr_beq_move_rts`: Short code fragment that calls another routine, then branches if the last result was zero, then calls another routine (3 times in a row), then copies a fixed memory address to a fixed memory address, then calls another routine (2 times in a row), then returns.
- `frag_move_lea_clr_dbra_rts`: Short code fragment that copies 7999 to d0, then loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then clears the memory at a0 (advancing a0), then clears the memory at a1 (advancing a1), then loops back while counting down d0, then returns.
- `frag_lea_move_clr_rts`: Short code fragment that loads the address of a fixed memory address into a6, then copies a fixed memory address to a fixed memory address, then copies 19924 to a fixed memory address, then copies a fixed memory address to a fixed memory address, then copies 19970 to a fixed memory address, then clears the memory at an offset from a6, then copies 160 to the memory at an offset from a6, then returns.
- `frag_clr_move_beq_rts`: Short code fragment that clears d0, then copies a fixed memory address to d0, then branches if the last result was zero, then clears a fixed memory address, then returns.
- `frag_bsr_tst_beq_rts`: Short code fragment that calls another routine, then tests d0, then branches if the last result was zero, then returns.
- `frag_movem_move_bsr_bchg_rts`: Short code fragment that saves registers on the stack, then copies a fixed memory address to d0, then calls another routine, then copies a fixed memory address to a fixed memory address, then flips bit 0 of a fixed memory address, then restores saved registers from the stack, then copies a fixed memory address to the stack, then returns.
- `handle_interrupt_update`: Reads Amiga custom chip registers to check for a specific interrupt condition, increments a global counter if the condition is met, updates bitplane pointers in hardware registers, and calls three routines.
- `frag_or_clr_tst_beq_rts`: Short code fragment that merges d0 into d0, then clears a fixed memory address, then tests a fixed memory address, then branches if the last result was zero, then returns.
- `frag_lea_move_rts`: Short code fragment that loads the address of a fixed memory address into a6, then copies 16896 to the memory at an offset from a6, then copies 11393 to the memory at an offset from a6, then copies 62657 to the memory at an offset from a6, then copies 56 to the memory at an offset from a6, then copies 208 to the memory at an offset from a6, then copies an address to a fixed memory address, then returns.
- `store_register_values`: Stores a word value into a table at an offset determined by the lower nibble of another value, and also stores the shifted value into a hardware register at an offset determined by that same value.
- `frag_btst_bne_rts`: Short code fragment that tests bit 14 of the memory at an offset from a6, then branches if the last result was not zero, then returns.
- `process_joystick_input`: Reads input from hardware registers to determine joystick state and sets corresponding bits in a global variable based on the input values.
- `update_table_entry`: Processes a byte value through bit rotation and exclusive OR operations, then uses the result to update a table entry and conditionally modify another table location.
- `clear_table_entry`: Clears a long global variable, masks d0 with bit 7, and clears a byte in a table at an offset determined by the masked d0 value, then returns.
- `save_and_call_subroutines`: Saves register values into global variables, calls a subroutine, and if the result is non-zero, calls two additional subroutines before returning.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `initialize_and_execute`: Initializes memory locations with specific values and executes a function pointer from a table based on a condition.
- `frag_bsr_adda_move_rts`: Short code fragment that calls another routine, then adds 8000 to a0, then adds 8000 to a1, then copies a1 to the memory at an offset from a6, then copies an address to the memory at an offset from a6, then copies a0 to the memory at an offset from a6 (2 times in a row), then copies d3 to the memory at an offset from a6, then returns.
- `initialize_structure_fields`: Calls another routine to set up structure fields, then updates several memory locations with calculated values based on register contents and a constant.
- `update_structure_fields`: Stores multiple values from registers and global variables into a structure pointed to by a6, then calculates a return value using bit shifts and logical OR operations.
- `set_structure_constants`: Sets specific constant values and register contents into a structure pointed to by a6, then calculates a value in d3 using a left shift and logical OR with d2 before returning.
- `initialize_structure_fields_3`: Saves all registers to the stack, calls multiple routines, restores registers, and stores specific values from registers and global variables into fields of a structure pointed to by a6, then performs bit operations and additional routine calls.
- `initialize_structure_values`: Initializes several fields in a structure pointed to by a6 with specific values, calls another routine, and then updates three fields in the structure before returning.
- `setup_structure_fields`: Sets up multiple fields in a data structure with calculated values and updates a global variable based on a bit test result.
- `populate_structure_data`: Populates a data structure with values from global variables, registers, and memory locations, then executes multiple routine calls.
- `frag_move_lea`: Short code fragment that copies 3580 to d7, then copies 4094 to d6, then loads the address of a fixed memory address into a5, then copies 65535 to the memory at an offset from a6.
- `initialize_structure_fields_2`: Sets multiple fields in a data structure starting at address in a6 with values from registers and other addresses, then calls another routine and updates some fields based on its result.
- `calculate_address_offset`: Computes an address by performing arithmetic operations on d1 and d0 values, adds a global variable value to that address, and returns the result.
- `fall_through_entry_4`: Entry point part-way through straight-line code: runs one instruction (or.b) and then falls through into the code that follows, with no return of its own.
- `setup_audio_dma_channels`: Configures audio DMA channels by copying data from a table to specific hardware registers, sets up multiple channel parameters, and enables DMA with specific bit patterns.
- `frag_lea_move_bsr_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies 22590 to d1, then copies 720 to d0, then calls another routine, then returns.
- `decrement_channel_timers`: Decrements timers for audio channels and clears a global variable when timers reach zero, then calls another routine.
- `frag_lea_moveq_ror_addi_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads 47 into d0, then rotates  right by the memory at a0, then adds 5248 to the memory at a0 (advancing a0), then loops back while counting down d0, then returns.
- `process_audio_channel`: Updates audio channel parameters including volume and period settings by reading from tables and applying conditional modifications based on flags, then decrements a counter and resets the channel when it reaches zero.
- `update_audio_parameters_2`: Modifies audio volume and period settings for a sound channel by reading values from global variables, performing bit shifts and masking operations, and writing results to memory locations.
- `update_audio_parameters`: Modifies audio volume and period settings for a sound channel by reading values from global variables, performing bit shifts and masking operations, then writes results to hardware registers.

### sp (raw code file, load address unknown)

- `initialize_and_process`: Saves registers, sets up a data structure with values from memory locations, calls multiple routines to process the data, and then restores registers before returning.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `frag_move_cmpa_bgt_jmp_lea_bsr_sf_rte`: Short code fragment that copies the memory at a6 (advancing a6) to d7, then compares a6 with d6, then branches if the comparison was greater, then jumps to the address in a5, then copies 9984 to the status register, then loads the address of a location in the code into a0, then calls another routine, then clears the memory at an offset from a2, then copies 9472 to the status register, then returns from the exception.
- `copy_and_expand_data_2`: Reads 32-bit and 16-bit values from an array at a calculated offset, expands them into memory locations pointed to by a4, and returns with the expanded data.
- `frag_andi_lsl_move_rte`: Short code fragment that masks d7 with 255, then shifts d7 left by 1, then copies an indexed entry from a4 to d7, then returns from the exception.
- `frag_movea_move_rts`: Short code fragment that copies an address to a0, then copies the memory at an offset from a0 to the memory at an offset from a2 (7 times in a row), then returns.
- `frag_movea_move_rts_2`: Short code fragment that copies an address to a0, then copies the memory at an offset from a2 to the memory at an offset from a0 (7 times in a row), then returns.
- `frag_move_rts_4`: Short code fragment that copies the status register to d0, then copies 9984 to the status register, then copies a0 to a fixed memory address, then copies d0 to the status register, then returns.
- `frag_movea_move_andi_lea_lsl_lsr_rts`: Short code fragment that copies an address to a1, then copies 0 to the memory at an offset from a1, then masks d0 with 7, then loads the address of a location in the code into a0, then shifts d0 left by 1, then copies an indexed entry from a0 to d0, then copies d0 to the memory at an offset from a1, then shifts d0 right by 8, then copies d0 to the memory at an offset from a1, then returns.
- `frag_movea_move_bclr_rts`: Short code fragment that copies an address to a0, then copies 32 to the memory at an offset from a0, then copies 0 to the memory at an offset from a0, then copies 32 to the memory at an offset from a0, then copies 0 to the memory at an offset from a0, then clears bit 3 of the memory at an offset from a0, then returns.
- `write_sequence_to_port`: Writes a sequence of byte values to specific memory locations at address a fixed address, setting alternating bytes to zero and incrementing values in a pattern.
- `set_interrupt_mask`: Saves the current processor status register, sets the interrupt mask to disable all interrupts, clears a memory location, and restores the original processor status register.
