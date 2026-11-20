# Game 2798 (bagitman (1992)(bignonia)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02798). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (922080 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 25 raw-word lines |
| Functions named and described | pass, 135 of 135 named (0 author symbols), 135 of 135 described (31 by rule, 101 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3259 of 3259 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4773 of 4775 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 660 |
| code | 19440 |
| data | 110655 |
| padding | 797 |
| unused | 790528 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Bagitman (1992)(Bignonia).adf:1 | 1 | 201 | 6 |
| Bagitman (1992)(Bignonia).adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 778 | 15 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2826 | 93 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 952 | 20 |

## Cases handled

- runtime segment an address (play-lgpwide): 249 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2063 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 752 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 5 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 3 immediate+address, 2 immediate

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_hardware` (runtime segment an address): Initializes hardware registers including DMA control, interrupt enable, and coprocessor settings, then waits for vertical blanking interval before returning. Kind: disk hardware check. Pass path: The routine initializes hardware and performs a loop that checks for a specific condition (_DAT_00072cec == 0) before proceeding. If the check passes (i.e., the condition is met), it continues execution and restores interrupt and DMA states.

## Functions


### Bagitman (1992)(Bignonia).adf:1

- `frag_bra_bsr`: Short code fragment that jumps to another point in the code, then calls another routine, then jumps to another point in the code.
- `copy_and_process_data`: Copies data from a source table to a destination buffer while performing bit manipulation operations on values read from another table, then writes processed values to a hardware color register.
- `extract_byte_and_rotate_long`: Reads a longword from a table in a0, extracts the lowest byte into d7, stores this byte at the destination pointed to by a5, and performs a logical rotate right through the carry flag on the original longword in d0 before returning.
- `rotate_bits_right_2`: Performs bit rotation on a value stored in d0, using a counter in d1 to control the number of iterations.
- `unpack_data`: Reads a long word from memory, extracts a byte from it, stores that byte in memory, and rotates the original value right by one bit before returning.
- `rotate_bits_right`: Performs bit rotation on a value stored in d0, using a counter in d1 to control the number of iterations.

### Bagitman (1992)(Bignonia).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_bra`: Short code fragment that jumps to another point in the code.
- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `frag_bra_3`: Short code fragment that jumps to another point in the code.
- `frag_bra_lea`: Short code fragment that jumps to another point in the code, then loads the address of a location in the code into a0, then jumps to another point in the code.
- `initialize_audio_system`: Initializes audio system registers by setting up DMA channels, configuring audio volume, and preparing audio period updates through table lookups and conditional logic.
- `update_audio_period`: Updates the audio period register based on table lookups and conditional logic, modifying global state and returning a modified value.
- `update_position_data`: Processes input direction data by reading from a table, performing arithmetic operations on word values, and updating position coordinates.
- `handle_input_direction`: Processes input direction data by reading from a table, performing arithmetic operations on word values, and updating position coordinates based on comparison results.
- `calculate_movement_delta_2_2`: Calculates movement delta values by reading from a lookup table, applying bit manipulation operations, and updating position coordinates with conditional comparisons.
- `calculate_movement_delta_2`: Calculates movement delta values by reading from a lookup table, applying bit manipulation operations, and updating position coordinates with arithmetic comparisons.
- `calculate_movement_delta`: Calculates movement delta values by reading from a lookup table, applying bit manipulation operations, and updating position coordinates with addition or subtraction.
- `process_input_direction`: Reads input direction data from a structure, processes it through conditional branches based on specific values, updates position coordinates and flags in the structure, and returns immediately.
- `store_direction_mask`: Extracts the lower nibble of a byte at offset 3 in the structure, stores it at offset 18, and returns immediately.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (bchg.b) and then falls through into the code that follows, with no return of its own.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_game_state_2`: Initializes hardware registers including DMA control, interrupt enable, and copper list pointers.
- `frag_lea_move_addq_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies 8315 to d7, then copies the memory at an offset from a0 to the memory at a0, then copies the memory at an offset from a0 to the memory at an offset from a0 (2 times in a row), then adds 6 to a0, then loops back while counting down d7, then returns.
- `frag_lea_move_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then copies 719 to d7, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then loops back while counting down d7, then returns.
- `frag_lea_move_clr_addq_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies 8315 to d7, then clears the memory at an offset from a0 (2 times in a row), then adds 6 to a0, then loops back while counting down d7, then returns.
- `copy_4_arrays`: Copies data from four source arrays to a destination array with specific offsets, repeating this process 199 times with 9 iterations per cycle.
- `copy_and_update`: Copies data between arrays and updates memory locations with fixed values based on conditional checks of a global variable.
- `decrement_timer`: Decrements a timer value stored in a global variable and updates a pointer to a table, then returns.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (or.b) and then falls through into the code that follows, with no return of its own.
- `process_input_and_update_globals`: Reads joystick/mouse input from CIAA_PRA, updates multiple global variables based on input conditions, and manages state transitions.
- `update_pointer_and_counters`: Updates multiple global variables with values from a table, adjusts pointer positions, and decrements counter values based on conditional checks.
- `update_counter`: Updates a counter value in memory based on comparisons with predefined constants, setting it to either 2 or -2 before adding it to the current counter.
- `process_queue_item`: Processes an item from a queue by updating pointers and values, then decrements a counter after each operation.
- `copy_sprite_data`: Copies sprite data from one memory location to another in multiple blocks, then clears a global variable and increments a counter until it reaches 100.
- `load_sprite_data`: Loads sprite data from a source array to a destination array with specific memory offsets, updating the source pointer after each load.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs one instruction (sbcd) and then falls through into the code that follows, with no return of its own.
- `process_interrupts`: Saves all registers to the stack, reads interrupt status from hardware, writes a value to a DMA register, tests a bit in the interrupt status, and calls multiple routines if the bit is set.
- `frag_andi_lsl_move_cmp_bne_rts`: Short code fragment that masks d0 with 65535, then shifts d0 left by 8, then copies the memory at an offset from a6 to d1, then masks d1 with an address, then compares d0 with d1, then branches if the last result was not zero, then returns.
- `frag_move_ori_not_ror_andi_rts`: Short code fragment that copies a fixed memory address to d0, then merges 64 into a fixed memory address, then inverts d0, then rotates d0 right by 1, then masks a fixed memory address with 191, then copies d0 to a fixed memory address, then returns.
- `manage_sequence_data`: Checks sequence data for completion, clears counters, reads from memory locations, compares values, and updates state variables based on comparison results.
- `process_coordinates`: Reads coordinate values from a table, updates counters based on comparison results, and manages state transitions in a loop until a termination condition is met.
- `compare_coordinates`: Compares coordinate values between two sets and sets flags in a word register based on the comparison results.
- `frag_move_bsr_rts`: Short code fragment that copies a fixed memory address to d0, then copies 7 to d2, then copies a fixed memory address to d3, then calls another routine, then copies d3 to a fixed memory address, then copies d4 to a fixed memory address, then returns.
- `read_joystick_input`: Tests a counter value, reads joystick data from hardware, calls a routine to update flags, stores results in memory locations, and returns.
- `update_joystick_flags`: Reads the joystick/mouse fire button status from CIAA_PRA, updates a global variable based on bit tests, and sets specific bits in a local word variable depending on comparison results.
- `check_input_and_set_flags`: Checks joystick or mouse input from a hardware register and sets flags based on bit tests and conditional logic, then returns.
- `frag_move_bra`: Short code fragment that copies 11 to d7, then jumps to another point in the code.
- `copy_and_check_condition`: Copies data from one array to another with a loop of 33 iterations, then checks a global variable condition and updates another memory location.
- `initialize_globals`: Initializes multiple global variables with specific values including constants, pointers, and clears memory locations in a structured pattern.
- `frag_lea_move_rts`: Short code fragment that loads the address of a fixed memory address into a1, then copies the memory at a0 (advancing a0) to the memory at an offset from a1 (7 times in a row), then returns.
- `initialize_graphics_data`: Writes specific 16-bit values to multiple memory locations at fixed offsets from a base address, then returns.
- `frag_move_lea_moveq_swap_addq_dbra_rts`: Short code fragment that copies an address to d0, then loads the address of a fixed memory address into a0, then loads 7 into d7, then copies d0 to the memory at an offset from a0, then swaps the halves of d0, then copies d0 to the memory at an offset from a0, then swaps the halves of d0, then adds 8 to a0, then loops back while counting down d7, then returns.
- `copy_and_transform_tile_data`: Copies byte data from one memory region to multiple target regions with specific offset calculations, then performs a second copy operation with a different source and destination.
- `frag_lea_move_clr_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies 53999 to d7, then clears the memory at a0 (advancing a0), then loops back while counting down d7, then returns.
- `update_game_state`: Compares a counter value with two, updates multiple memory locations with fixed values, performs arithmetic operations on word values, and calls another routine to update memory values.
- `frag_lea_bra`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then loads the address of a fixed memory address into a2, then jumps to another point in the code.
- `update_game_state_3`: Updates game state variables based on condition codes and global flags, modifying position and status indicators before returning.
- `copy_and_update_pointer`: Copies bytes from a source address to three different destinations with fixed offsets, updates the source pointer, and conditionally writes specific values to memory when a byte equals one.
- `handle_countdown_and_render`: Manages countdown timers and performs rendering operations by copying pixel data from a lookup table based on division results.
- `convert_and_copy_digits`: Reads a word from memory, divides it by 1000, stores the quotient in memory, performs division and modulo operations to extract digit components, and copies pixel data from lookup tables.
- `convert_number_to_digits`: Converts a 16-bit value into four digit components stored in memory, then copies pixel data from a lookup table based on those digits.
- `copy_block_of_words`: Copies nine 32-bit values from one memory region to another, followed by three more values from a second source region.
- `copy_and_distribute_words`: Copies nine 32-bit values from one memory region to two different destinations, with the last three values going to a third destination.
- `initialize_sequence_data`: Sets a global variable to two, configures loop counters, and invokes multiple routines to manage sequence-related data and state transitions.
- `update_game_counter`: Increments a game counter when a condition is met, updates multiple global variables based on comparison results, and performs conditional operations including memory copies and status flag modifications before returning.
- `check_sprite_condition`: Evaluates sprite state conditions based on global variables and modifies sprite properties accordingly before returning.
- `handle_collision_detection`: Processes collision detection logic based on position coordinates and global state, updating internal structures and triggering further rendering operations.
- `process_collision_checks_2`: Validates collision data by reading from multiple memory locations and comparing values against specific constants, then updates a global flag table based on the comparison results.
- `process_collision_checks`: Loads different addresses into a4 and a5 registers, then calls a validation routine three times with those address pairs as parameters.
- `validate_collision_state`: Checks if a state value equals one or two, then validates coordinate differences against specific thresholds before resetting the state.
- `validate_and_process`: Validates a word value from a global variable against zero and three, clears a register if valid, then processes data through two separate routine calls based on table lookups before conditionally branching.
- `validate_position_and_invert`: Validates coordinate differences against specific thresholds and inverts a word register when conditions are met.
- `update_position`: Updates position coordinates based on input parameters and global state, then stores the results in memory.
- `update_sprite_state_2`: Reads a state value from memory, compares it against several constants, and branches to different handlers based on the result.
- `update_sprite_state`: Updates sprite state based on condition codes and global variables, modifying position and status flags before returning.
- `copy_sprite_data_2`: Copies sprite data from one memory location to another in multiple blocks, then clears a global variable and increments a counter until it reaches zero.
- `update_game_state_2`: Processes game state based on a counter value and bit flags in a register, calling different routines depending on the conditions and updating collision detection values.
- `process_game_state`: Processes game state based on a counter value and bit flags in a register, calling different routines depending on the conditions and updating memory locations with calculated values.
- `update_counter_and_position_2`: Updates counter and position values based on conditional checks, then returns.
- `initialize_game_state`: Sets specific values in global variables and loads data from a table into multiple locations, then branches to another routine.
- `update_counter_and_position`: Updates a counter and position values based on conditional checks, then returns.
- `decrement_and_update_state`: Decrements a counter and updates state values based on conditional checks, then returns.
- `frag_cmpi_bne_subq_bra_rts`: Short code fragment that compares a fixed memory address with 2, then branches if the last result was not zero, then subtracts 3 from a fixed memory address, then jumps to another point in the code, then returns.
- `increment_and_modify_coordinates`: Increments a coordinate value and modifies memory locations with calculated values, then returns.
- `update_position_and_state_2`: Updates position and state variables based on conditional checks involving a global variable and arithmetic operations, then adjusts memory locations and returns.
- `update_counter_and_position_2_2`: Updates counter and position values based on conditional checks involving byte comparisons and arithmetic operations, then returns after modifying global variables.
- `update_position_and_state`: Updates position and state variables based on conditional checks involving global flags and arithmetic operations, then adjusts memory locations and branches to another routine.
- `process_movement_data_2`: Processes movement data by checking conditions, updating counters, and managing memory pointers with arithmetic operations, then returns after modifying global variables.
- `process_movement_data`: Processes movement data by checking conditions, updating counters, and managing memory pointers with arithmetic operations.
- `update_collision_check`: Updates collision detection values by checking coordinate ranges and comparing them with global variables, then sets specific values in memory locations based on the comparison results.
- `adjust_coordinate_based_on_flags`: Adjusts a coordinate value based on multiple flag comparisons and returns.
- `load_and_copy_bytes`: Loads bytes from multiple table offsets using calculated indices and copies them to different memory locations, updating pointers during the process.
- `update_memory_with_bit_operations_2`: Updates memory locations with bit operations, manages counter decrements, and handles pointer updates for data structures.
- `update_memory_with_bit_operations`: Performs bit operations on memory locations at calculated addresses and returns.
- `modify_counters_and_flags`: Modifies counters based on bit tests and flag comparisons, then updates a global variable and returns.
- `frag_btst_bne_rts`: Short code fragment that tests bit 14 of the memory at an offset from a6, then branches if the last result was not zero, then returns.
- `frag_lea_move_bsr_dbra`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then copies 21 to d7, then calls another routine, then loops back while counting down d7.
- `loop_and_branch`: Loops 21 times decrementing a counter while testing a global variable, branching to another routine if the variable is non-zero, and otherwise continuing the loop.
- `set_render_mode`: Sets a render mode parameter to 33792, calls another routine twelve times, then sets the same parameter to 1024 and returns.
- `setup_render_parameters`: Sets up render parameters in memory locations using calculated values and calls another routine, then returns.
- `calculate_and_store_render_data`: Calculates render data from table values, stores it in memory locations, calls another routine, and returns.
- `setup_double_render_parameters`: Sets up double render parameters in memory locations using calculated values and calls another routine twice, then returns.
- `compute_and_store_coordinates`: Calculates coordinate values using table lookups and arithmetic operations, then stores the results to specific offsets in a structure pointed by a6.
- `frag_move_clr`: Short code fragment that copies the memory at a0 to d0, then clears a fixed memory address, then copies 56 to a fixed memory address, then copies 320 to a fixed memory address (2 times in a row).
- `update_memory_values`: Updates multiple memory locations with word-sized values derived from a register through bit manipulation and addition operations.
- `frag_clr_jmp`: Short code fragment that clears a fixed memory address, then jumps to another point in the code.
- `frag_clr_jmp_2`: Short code fragment that clears a fixed memory address, then jumps to another point in the code.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `frag_move_jmp`: Short code fragment that copies 65535 to a fixed memory address, then jumps to another point in the code.
- `frag_tst_bne_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then tests a fixed memory address, then branches if the last result was not zero, then tests a fixed memory address, then branches if the last result was not zero, then tests a fixed memory address, then branches if the last result was not zero, then returns.
- `clear_global_variable`: Sets a global variable to zero using multiple write operations.
- `process_audio_dma`: Processes audio data by reading from tables, writing to DMA registers, and managing counters while enabling and disabling DMA channels.
- `update_animation_state_3`: Updates animation state based on frame counter and global variable, modifying data at memory locations and advancing pointers through conditional operations.
- `update_bit_flags`: Reads a word from memory at offset 120 of a register, tests that bit in a global word, toggles a byte at offset 180 of the register, clears a bit in the register and then clears another bit in the register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware`: Initializes hardware registers including DMA control, interrupt enable, and coprocessor settings, then waits for vertical blanking interval before returning.
- `copy_coordinates`: Copies 16-bit values from one memory location to another in a loop, updating multiple fields of a structure through pointer arithmetic.
- `frag_move_swap_add_adda_dbra_rts`: Short code fragment that copies a1 to d0, then copies d0 to the memory at an offset from a0, then swaps the halves of d0, then copies d0 to the memory at a0, then swaps the halves of d0, then adds d1 to d0, then adds d3 to a0, then loops back while counting down d2, then returns.
- `clamp_values`: Clamps 16-bit values between zero and maximum thresholds using conditional comparisons and assignments.
- `scale_pixels`: Performs fixed-point arithmetic operations on pixel data, combining multiplication and bit manipulation to produce scaled output values.
- `setup_graphics`: Calculates memory addresses for graphics rendering by performing arithmetic operations on coordinates and stores the results in a structure.
- `update_sprite_data`: Processes sprite animation data by reading byte values from three source pointers, using them as indices into a lookup table to update corresponding entries in a destination structure.
- `encode_coordinates_2`: Encodes coordinate values into a byte stream using bit manipulation and conditional increments based on carry flags, then stores the results in memory locations.
- `encode_coordinates`: Encodes coordinate values into a byte stream using bit manipulation and conditional increments based on carry flags, then stores the results in memory.
- `update_position_delta`: Calculates and updates position deltas by subtracting input values from stored coordinates, applying two's complement when negative, and updating global variables with the computed results.
- `check_game_conditions`: Evaluates multiple game state conditions including input from hardware registers, global variables, and timer values to determine the next course of action and update related state variables accordingly.
- `update_animation_state_2`: Updates animation state variables based on global conditions and performs conditional operations on counters, then returns.
- `frag_clr_move_rts`: Short code fragment that clears the memory at an offset from a6 (4 times in a row), then copies 15 to the memory at an offset from a6, then returns.
- `process_data_structure`: Processes a data structure by validating header fields, computing memory offsets through multiplication and addition operations, and populating an output array with calculated addresses based on input parameters.
- `update_audio_channels`: Updates audio channel parameters based on object data and global state, modifying hardware registers and processing audio envelopes.
- `process_game_objects`: Iterates through game object data, updates object properties based on stored instructions, and modifies global state variables accordingly.
- `initialize_object`: Initializes object properties based on a lookup table index, clears certain flags, and sets up position and state data.
- `store_channel_period_from_table`: Looks up a 16-bit value in a period table using a byte index and stores it in a field of the channel block.
- `process_audio_envelope`: Loops through audio channel data, checks a flag, and calls another routine when the flag is set, updating pointers and decrementing a counter.
- `update_animation_state`: Updates animation state variables based on frame counters and table lookups, then clears flags when counters reach zero.
