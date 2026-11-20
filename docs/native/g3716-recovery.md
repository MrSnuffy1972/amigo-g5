# Game 3716 (Frantic Freddie): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03716). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (930288 top-level bytes in 12 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 32 raw-word lines |
| Functions named and described | pass, 177 of 177 named (0 author symbols), 177 of 177 described (55 by rule, 116 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3731 of 3731 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5593 of 5593 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 155961 |
| code | 27610 |
| data | 18939 |
| padding | 1250 |
| unused | 726528 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Frantic Freddie.adf:FranticFreddie | 2 | 316 | 13 |
| Frantic Freddie.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3825 | 120 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 272 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 385 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 514 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 268 | 12 |

## Cases handled

- 18 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2383 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 237 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 237 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 428 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 140 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 16 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 10 immediate, 3 jump target, 3 address

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Frantic Freddie.adf:FranticFreddie

- `setup_video_registers_2`: Initializes video display registers by setting up jump table entries with addresses of hardware control routines, then executes these routines in sequence to configure display parameters.
- `copy_and_shift_bytes`: Copies bytes from a source table to a destination based on bit operations, shifting data and managing control flow through conditional branches and subroutine calls.
- `rotate_right_and_store`: Performs a right rotate operation on a 32-bit value from memory, stores the lower byte of the original value to a table, and returns the rotated value with bit 31 set.
- `recursive_bit_shift_operation`: Performs recursive bit shifting operations with conditional subroutine calls, handling bit rotation and loop control based on input parameters.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `shift_and_store_2`: Shifts a 32-bit value right by one bit, stores the lower byte of the original value to a table, and returns the shifted value with bit 31 set if the original value was odd.
- `shift_and_store`: Shifts a 32-bit value right by one bit, stores the lower byte of the original value to a table, and returns the shifted value with bit 31 set.
- `shift_and_loop`: Shifts a value in a register right by one bit and loops until a counter reaches zero, calling another routine when the shifted value becomes zero.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `process_pixel_data`: Processes pixel data by shifting bits right, copying bytes between buffers, and performing conditional operations based on comparisons.
- `loop_shift_and_copy`: Processes data in a loop by shifting values right, copying bytes between buffers, and performing conditional operations based on comparison results.
- `shift_and_loop_2`: Shifts a value right by one bit in a loop controlled by a counter, calling another routine when the shifted value becomes zero, and decrements the counter until it reaches zero.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### Frantic Freddie.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_gameplay`: Initializes hardware registers including interrupts, DMA, and custom chip settings, then sets up multiple global variables and memory tables with specific values based on hardcoded constants.
- `read_keyboard_input`: Reads keyboard serial data, modifies CIA registers to enable/disable interrupts, checks for specific bit patterns, and conditionally calls a video initialization routine.
- `handle_keyboard_serial`: Processes keyboard serial input by reading CIA register values, manipulating interrupt flags, and calling multiple routines based on specific conditions while managing a counter for input timing.
- `update_animation_state`: Updates a global variable based on the current value of a memory location and modifies a word-sized value at a fixed address.
- `initialize_game_state`: Initializes multiple global variables and hardware registers with specific values, including clearing memory regions and setting display control parameters.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (btst.l) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (andi.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_4`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_5`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_6`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_7`: Entry point part-way through straight-line code: runs one instruction (andi.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_8`: Entry point part-way through straight-line code: runs one instruction (btst.l) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_9`: Entry point part-way through straight-line code: runs one instruction (andi.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_10`: Entry point part-way through straight-line code: runs one instruction (subi.b) and then falls through into the code that follows, with no return of its own.
- `render_background_layer`: Copies data from multiple source locations to a target buffer using indexed addressing and updates hardware registers for display control.
- `clear_video_memory`: Writes zero values to video memory regions using a loop counter and increments a global offset pointer.
- `copy_sprite_data`: Transfers data from a source buffer to video memory locations with specific offsets and updates multiple pointers during the operation.
- `increment_and_check_counter`: Increments a 16-bit counter and checks if it reaches 768, resetting it to zero when exceeded.
- `update_movement_parameters`: Updates movement parameters based on specific conditions, modifying video memory offsets and calling other routines to handle animation states.
- `frag_move_btst_bne_rts`: Short code fragment that copies 33792 to a fixed memory address, then tests bit 14 of a fixed memory address, then branches if the last result was not zero, then copies 1024 to a fixed memory address, then returns.
- `process_character_tile`: Reads character data from a table, calculates an offset, and copies bytes to a destination buffer while updating an index.
- `configure_blitter_registers`: Configures multiple blitter hardware registers with specific values and calls routines to manage video memory operations while updating a global counter.
- `copy_tile_row`: Copies character data from a source table to video memory with row-by-row processing and updates both source and destination pointers.
- `update_animation_states`: Updates animation state variables based on conditional checks and increments counters, then copies byte data from multiple table offsets into memory locations starting at a fixed address.
- `update_animation_frames_2`: Increments two counters and copies data from memory locations to video buffer addresses, processing 8 bytes per iteration for two separate data blocks.
- `update_video_memory_pointer`: Updates a video memory pointer by decrementing a long integer value and checking if it reaches a specific threshold, then branches based on the comparison result.
- `initialize_video_registers`: Waits for vertical blank interrupt, sets up coprocessor registers, initializes multiple memory regions with specific values, and copies screen buffer data between locations.
- `check_input_flags`: Reads a 16-bit value from a custom hardware register, checks specific bits, and conditionally updates a global variable and another memory location before returning.
- `copy_sprite_data_2`: Copies sprite-related data from one memory location to another using indexed addressing, processing 18 iterations of data transfers.
- `clear_three_bytes`: Sets three specific bytes in global memory to zero and then returns.
- `frag_movea_adda_move_rts`: Short code fragment that copies an address to a0, then copies an address to a1, then adds a fixed memory address to a0, then copies the memory at a0 to the memory at an offset from a1, then copies the memory at an offset from a0 to the memory at an offset from a1 (2 times in a row), then returns.
- `frag_movea_move_rts`: Short code fragment that copies an address to a1, then copies 4095 to the memory at an offset from a1, then returns.
- `update_game_state_9`: Updates multiple global variables and performs screen rendering operations including clearing memory, copying data between buffers, and managing game state counters.
- `update_game_state_8`: Updates game state variables and performs screen rendering operations including clearing memory, copying data between buffers, and managing display registers.
- `frag_moveq_move_movea_subi_adda_bne_jsr`: Short code fragment that loads 0 into d0, then copies 24 to d0, then copies an address to a0, then subtracts 1 from the memory at an offset from a0, then adds 160 to a0, then subtracts 1 from d0, then branches if the last result was not zero, then calls another routine, then returns.
- `update_sprite_positions`: Updates sprite position data by incrementing byte values at multiple memory offsets and performs conditional operations based on hardware register checks.
- `frag_movea_move_rts_2`: Short code fragment that copies an address to a1, then copies 2608 to the memory at an offset from a1, then copies 4080 to the memory at an offset from a1, then copies 2608 to the memory at an offset from a1, then returns.
- `frag_movea_move_rts_3`: Short code fragment that copies an address to a1, then copies 128 to the memory at an offset from a1, then copies 4095 to the memory at an offset from a1, then copies 128 to the memory at an offset from a1, then returns.
- `update_collision_parameters`: Updates collision detection parameters by copying word values from a table to various memory addresses, then increments an index pointer.
- `initialize_game_objects`: Initializes game object properties by writing byte values to memory locations at specific offsets from a base address, updates counters and performs conditional operations based on comparison results, then configures multiple hardware registers and memory areas with predefined values.
- `reset_input_flags`: Clears and resets specific input state flags based on conditions derived from a word-sized value, then returns.
- `update_game_state`: Updates multiple game state flags based on conditions checked against a word-sized value and a global byte variable, then returns.
- `handle_input_state_3`: Checks input conditions and updates state flags based on comparison results, transitioning between different input states depending on value ranges.
- `check_and_call`: Checks if a global variable equals 1 and calls another routine if true, otherwise continues to the next condition.
- `check_and_execute_state_change`: Checks global variables for specific values and executes different routines based on the comparison results, updating internal game state counters.
- `execute_conditional_state_transitions`: Executes conditional state transitions by checking global variables and calling different routines depending on their values, managing game behavior through multiple flags.
- `manage_game_state_machine_2`: Manages a finite state machine by incrementing counters and setting various flags based on comparison results, cycling through different game states with conditional logic.
- `frag_move_mulu_cmp_bhi_rts`: Short code fragment that copies a fixed memory address to d0, then copies a fixed memory address to d1, then multiplies d1 by 2 (unsigned), then compares d1 with d0, then branches if the comparison was higher (unsigned), then copies 0 to a fixed memory address, then copies 1 to a fixed memory address, then copies 0 to a fixed memory address (2 times in a row), then returns.
- `manage_game_state_machine`: Manages a finite state machine by incrementing a counter and setting various flags based on the counter's value, cycling through four distinct states.
- `update_animation_counters`: Updates animation-related counters and manages state transitions based on comparison results, setting specific flags in global variables according to conditional logic.
- `compare_and_set_flags`: Compares two values from memory locations and sets specific flags based on the comparison result, using multiplication and conditional branching to control program flow.
- `initialize_graphics_flags`: Initializes four graphics-related flags to specific values and returns immediately.
- `process_game_logic`: Processes game logic by checking multiple conditions including counters, input states, and comparison operations, then sets flags based on the results of these checks.
- `update_game_state_2`: Updates internal game state counters and performs conditional logic based on global variables, setting specific flags in a structure depending on comparison results.
- `test_and_set_bits`: Tests bits in two data registers and sets specific bits in those registers based on the test results.
- `update_direction_flags`: Updates direction flags based on counter values and table lookups, then sets specific global variables to indicate movement direction before returning.
- `update_game_state_3`: Updates internal game state counters and performs conditional logic based on table lookups, setting specific global variables to control game behavior.
- `update_game_state_4`: Updates internal game state counters and checks conditions based on table lookups, setting flags in global variables depending on comparison results.
- `frag_btst_ori`: Short code fragment that tests bit d0 of d1, then merges 1 into d1, then merges 0 into d1, then tests bit d0 of d0.
- `update_game_state_5`: Updates internal counters and checks conditions based on table lookups, then sets multiple global variables to control game behavior before returning.
- `update_game_state_6`: Updates internal game state counters and performs conditional logic based on their values, setting flags in global variables before returning.
- `process_joystick_input`: Reads joystick data from a hardware register, processes bit patterns to determine input state, and calls different routines based on specific bit conditions and memory values.
- `frag_btst_beq_jsr_rts`: Short code fragment that tests bit 8 of d0, then branches if the last result was zero, then calls another routine, then returns, then tests bit 0 of d0, then branches if the last result was zero, then calls another routine, then returns.
- `bit_test_and_execute`: Tests specific bits in a register and calls different routines based on which bits are set.
- `frag_btst_beq_jsr_rts_2`: Short code fragment that tests bit 1 of d0, then branches if the last result was zero, then calls another routine, then returns, then tests bit 8 of d0, then branches if the last result was zero, then calls another routine, then returns.
- `frag_btst_beq_jsr_rts_3`: Short code fragment that tests bit 8 of d0, then branches if the last result was zero, then calls another routine, then returns, then tests bit 9 of d0, then branches if the last result was zero, then calls another routine, then returns.
- `conditional_routine_calls`: Tests three specific bits in a register and calls routines accordingly.
- `bit_test_execution`: Performs bit testing on register contents and executes corresponding routine calls based on which bits are set.
- `update_game_state_7`: Reads a global variable, increments it, and writes its value to multiple memory locations in a loop structure.
- `decrement_and_copy`: Decrements a global variable and performs multiple byte copies from the global variable to memory locations offset by 1 and 105 relative to a pointer, then calls another routine.
- `update_position_and_render_2`: Clears three registers and loads a constant value into d4, then processes data in a loop that iterates twice.
- `update_sprite_data`: Updates sprite coordinate values in memory by reading a global variable, adding 24 to it, and storing the result at offsets 2 and 106 from a pointer.
- `decrement_and_update`: Decrements a global variable and updates a table of bytes with the variable's value and its incremented version, iterating a fixed number of times.
- `update_animation_frame`: Increments a global counter and updates byte values at specific offsets in a memory area, then advances the memory pointer by 208 bytes for eight iterations.
- `toggle_render_state`: Checks a global condition, increments a counter, and writes alternating zero or one values to specific memory locations based on the counter's parity, then updates multiple byte values in a memory area.
- `frag_cmpi_bne_clr_subi_move_andi_beq_br`: Short code fragment that compares a fixed memory address with 1, then branches if the last result was not zero, then clears d0, then subtracts 1 from a fixed memory address, then copies a fixed memory address to d0, then masks d0 with 1, then compares d0 with 1, then branches if the last result was zero, then jumps to another point in the code.
- `update_render_state`: Checks a global condition and increments a counter when the condition is met, then processes data in a loop that iterates four times.
- `frag_cmpi_bne_clr_subi_bra`: Short code fragment that compares a fixed memory address with 1, then branches if the last result was not zero, then clears d2, then clears d3, then subtracts 1 from a fixed memory address, then jumps to another point in the code.
- `frag_clr_bra`: Short code fragment that clears d2, then clears d3, then jumps to another point in the code.
- `frag_movea_adda_move_ori_subi_bne_rts`: Short code fragment that copies an address to a0, then adds a fixed memory address to a0, then copies 4 to d3, then merges 2 into the memory at an offset from a0 (2 times in a row), then adds 208 to a0, then subtracts 1 from d3, then branches if the last result was not zero, then returns.
- `frag_movea_adda_move_andi_subi_bne_rts`: Short code fragment that copies an address to a0, then adds a fixed memory address to a0, then copies 4 to d3, then masks the memory at an offset from a0 with 253 (2 times in a row), then adds 208 to a0, then subtracts 1 from d3, then branches if the last result was not zero, then returns.
- `update_freddie_pattern`: Updates a pattern buffer based on a global counter and writes bytes to specific offsets relative to a table address, alternating between zero and one values.
- `check_input_condition`: Compares a global value with one and branches if not equal, otherwise clears a register and performs arithmetic operations on another global value before branching based on comparison results.
- `update_freddie_pattern_3`: Increments a global counter and updates pattern buffer bytes at specific offsets relative to a table address, alternating between zero and incremented values.
- `frag_cmpi_bne_clr_subi_bra_2`: Short code fragment that compares a fixed memory address with 1, then branches if the last result was not zero, then clears d2, then clears d3, then subtracts 1 from a fixed memory address, then jumps to another point in the code.
- `frag_clr_bra_2`: Short code fragment that clears d2, then clears d3, then jumps to another point in the code.
- `frag_clr_movea_adda_move_ori_subi_bne_r`: Short code fragment that clears d3, then copies an address to a0, then adds a fixed memory address to a0, then copies 4 to d3, then merges 2 into the memory at an offset from a0 (2 times in a row), then adds 208 to a0, then subtracts 1 from d3, then branches if the last result was not zero, then returns.
- `frag_clr_movea_adda_move_andi_subi_bne`: Short code fragment that clears d3, then copies an address to a0, then adds a fixed memory address to a0, then copies 4 to d3, then masks the memory at an offset from a0 with 253 (2 times in a row), then adds 208 to a0, then subtracts 1 from d3, then branches if the last result was not zero, then returns.
- `update_freddie_pattern_2`: Updates Freddie's visual pattern by modifying multiple memory locations based on a counter value, and sets specific bytes in a table with calculated offsets.
- `check_input_condition_2`: Compares a global value with one and branches if not equal, otherwise clears a register and performs arithmetic operations on another global value before branching based on comparison results.
- `update_freddie_pattern_4`: Increments a counter in a global variable and updates visual pattern data for Freddie by writing byte values to specific offsets in a table, with an inner loop processing four iterations.
- `frag_cmpi_bne_clr_subi_bra_3`: Short code fragment that compares a fixed memory address with 1, then branches if the last result was not zero, then clears d2, then clears d3, then subtracts 1 from a fixed memory address, then jumps to another point in the code.
- `frag_clr_bra_3`: Short code fragment that clears d2, then clears d3, then jumps to another point in the code.
- `frag_clr_movea_adda_move_ori_subi_bne_r_2`: Short code fragment that clears d3, then copies an address to a0, then adds a fixed memory address to a0, then copies 4 to d3, then merges 2 into the memory at an offset from a0 (2 times in a row), then adds 208 to a0, then subtracts 1 from d3, then branches if the last result was not zero, then returns.
- `frag_clr_movea_adda_move_andi_subi_bne_2`: Short code fragment that clears d3, then copies an address to a0, then adds a fixed memory address to a0, then copies 4 to d3, then masks the memory at an offset from a0 with 253 (2 times in a row), then adds 208 to a0, then subtracts 1 from d3, then branches if the last result was not zero, then returns.
- `frag_subi_bne_move_clr_addi_cmpi_beq`: Short code fragment that subtracts 1 from a fixed memory address, then branches if the last result was not zero, then copies 3 to a fixed memory address, then copies 1 to a fixed memory address (2 times in a row), then clears d1, then adds 208 to a fixed memory address, then compares a fixed memory address with 832, then branches if the last result was zero.
- `copy_coordinates`: Calculates word values from a base address plus an offset, stores them in two memory locations, swaps the bytes of the calculated value, and stores the swapped result in two additional memory locations.
- `frag_subi_bne_move_clr_addi_cmpi_beq_2`: Short code fragment that subtracts 1 from a fixed memory address, then branches if the last result was not zero, then copies 3 to a fixed memory address, then copies 1 to a fixed memory address (2 times in a row), then clears d1, then adds 208 to a fixed memory address, then compares a fixed memory address with 1664, then branches if the last result was zero.
- `update_position_data`: Updates position-related data in two structures with values derived from a global variable, then increments that global variable by 832 before looping back to itself.
- `update_position_and_render`: Decrements a counter and updates memory values based on conditional checks, then calculates and stores coordinate values in memory locations using a base address plus an offset.
- `render_frame_sequence`: Increments a counter and updates memory values when the counter reaches eight, then calculates and stores coordinate values in multiple memory locations using a base address plus two offsets.
- `update_render_position`: Increments a counter and updates memory values when the counter reaches two, then calculates and stores coordinate values in memory locations using a base address plus an offset.
- `update_render_position_2`: Increments a counter and updates memory values when the counter reaches five, then calculates and stores coordinate values in memory locations using a base address plus an offset.
- `update_animation_state_2`: Increments a counter and resets it to zero when reaching four, then updates a long integer value at a specific memory location by adding fixed offsets and stores the result in multiple word locations.
- `process_input_coordinates`: Calculates memory addresses based on global values and calls three different routines with computed offsets, performing arithmetic operations to determine the parameters for each call.
- `check_input_sequence_i_2`: Processes input values through arithmetic calculations and calls four different routines with computed parameters, performing multiple operations on global variables before each routine call.
- `calculate_positions`: Calculates and prepares coordinate values for multiple graphical elements by performing arithmetic operations on global variables, then passes these calculated values to routines responsible for rendering or updating their display positions.
- `calculate_coordinates`: Reads byte values from specific memory locations, performs arithmetic operations including addition, subtraction, multiplication and division on these values, and stores the results in registers before calling multiple routines with calculated offsets.
- `check_input_sequence_i`: Reads a byte from a calculated memory address and sets flags based on whether it equals 'i' or 'n', clearing other flags accordingly.
- `check_input_sequence_j`: Checks if a specific flag is set, reads a byte from a calculated memory address, and updates flags based on whether the byte equals 'j' or 'o'.
- `check_input_sequence_k`: Reads a byte from a calculated memory address and sets flags based on whether it equals 'k' or 'n', clearing other flags accordingly.
- `check_input_sequence_l`: Checks if a specific flag is set, reads a byte from a calculated memory address, and updates flags based on whether the byte equals 'l' or 'o'.
- `store_input_value`: Stores the input value to a global variable, reads a byte from a calculated memory address, and sets two flags based on whether the byte equals 'b' or 'c'.
- `process_character_input`: Reads a byte from a calculated memory address and sets multiple flags based on whether it equals 'i', 'f', 'j', 'h', 'n', or 'o'.
- `handle_special_inputs`: Reads a byte from a calculated memory address and sets multiple flags based on whether it equals 'k', 'l', 'a', 'g', 'f', 'h', 'n', or 'o'.
- `render_pattern_data`: Checks flags at specific memory locations and based on their values, copies data from one memory region to another using a nested loop structure that processes rows and columns of bytes.
- `copy_animation_frames`: When a flag is set, reads byte values from a source address and writes them to a destination address in a grid pattern, updating counters and flags after completing the copy operation.
- `copy_pattern_data`: Increments a counter and when it reaches sixteen, copies data from one memory region to another using a loop with specific offsets and then resets the counters.
- `frag_subi_bne_move_movea_rts`: Short code fragment that subtracts 1 from a fixed memory address, then branches if the last result was not zero, then copies 4 to a fixed memory address, then copies an address to a0, then copies the memory at an offset from a0 to d0, then copies the memory at an offset from a0 to the memory at an offset from a0, then copies d0 to the memory at an offset from a0, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware`: Sets up video and DMA hardware registers by writing specific values to control interrupt enable flags, DMA channels, and coprocessor jump addresses before calling initialization routines.
- `vertical_blank_loop`: Loops waiting for vertical blanking by checking a video position register, then calls multiple routines before checking a joystick button status in a CIA register and returning when the button is released.
- `frag_lea_clr_cmpa_bne_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then clears the memory at a0 (advancing a0), then compares a1 with a0, then branches if the last result was not zero, then returns.
- `frag_move_btst_bne_rts_2`: Short code fragment that copies 33792 to a fixed memory address, then tests bit 14 of a fixed memory address, then branches if the last result was not zero, then copies 1024 to a fixed memory address, then returns.
- `render_and_process_input`: Checks if a timer is active and decrements it, waits for vertical blanking, processes input from a buffer, and copies graphics data to video memory based on input values.
- `setup_blitter_dma`: Configures blitter registers for memory transfer operations and processes a data stream to set up display buffer addresses.
- `swap_and_store_values`: Swaps byte order of a 32-bit value and stores the result in two separate memory locations, then updates another memory location with incremented value.
- `frag_bsr_move_clr_rts`: Short code fragment that calls another routine, then copies an address to a fixed memory address, then copies a fixed memory address to a fixed memory address, then clears a fixed memory address, then copies 5724 to a fixed memory address, then returns.
- `copy_pattern_data_2`: Copies pattern data from one memory region to another using indexed addressing, handles end-of-data markers, and manages multiple data streams.
- `process_dma_data`: Loads addresses from memory locations into register a0, reads bytes from those addresses, performs conditional checks on the byte values, calls other routines to process data, and updates pointers in memory locations.
- `fill_dma_buffer`: Fills a DMA buffer with word values from source data, advancing both source and destination pointers while decrementing a counter.
- `frag_movea_move_rts_4`: Short code fragment that copies a fixed memory address to a0, then copies a fixed memory address to a fixed memory address, then copies a0 to a fixed memory address, then copies a fixed memory address to a0, then copies a fixed memory address to a fixed memory address, then copies a0 to a fixed memory address, then returns.
- `fill_table_entries`: Writes word values to a table in a loop, advancing the table pointer and decrementing a counter until it reaches zero.
- `update_global_flag`: Writes a 32-bit value from a register to a memory location pointed to by another register, then performs a bitwise OR operation on a global variable with the decimal value 148.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_music_sample_table`: Searches a byte table for the smallest value, then builds a pointer table by linking zeroed entries to sample start addresses indexed from a second table, sets bit 1 of a global variable, clears several global variables, and returns.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `update_audio_registers`: Updates audio channel registers with values from memory tables, processes sprite data through multiple subroutine calls, and manages a counter that controls when audio updates occur.
- `update_sprite_data_2`: Processes sprite animation and position data, updating structure fields based on input values and handling audio register writes.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `update_position`: Processes input data to update position values in a structure, handling boundary checks and bit manipulation for movement control.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_audio_system`: Initializes audio system by clearing volume registers, setting DMA control, processing data stream, and calculating buffer addresses for audio playback.
- `frag_lea_move_clr_subq_bne_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies 294 to d0, then clears the memory at a0 (advancing a0), then subtracts 1 from d0, then branches if the last result was not zero, then returns.
- `update_object_positions`: Updates object positions and DMA parameters for audio channels, managing animation states and handling audio command processing based on global flags.
- `process_audio_commands`: Reads audio command data from a table and updates audio channel parameters including volume, frequency, and status flags.
- `initialize_object`: Initializes object properties based on a lookup table, updates object state variables, and configures movement parameters.
- `set_player_sprite_offset`: The routine writes a 16-bit value from a table indexed by a byte in D4 to the word at offset six from the pointer in A1.
- `update_animation_frames_3`: Processes three animation counters in a structure using lookup tables, conditionally calling another routine and updating pointers in a loop.
- `update_animation_frames`: Updates three independent animation counters stored in the structure pointed to by A2 using tables pointed to by A3.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_video_and_input`: Initializes video registers and waits for specific input conditions before proceeding, using timer and interrupt controls.
- `handle_timer_countdown`: Manages a countdown timer that decrements a value at a fixed memory address, with conditional logic to reset or modify values based on comparison results.
- `initialize_memory_block`: Allocates memory using the system allocator, copies data from a fixed source to the allocated block, and calculates a checksum from system variables.
- `frag_lea_moveq_clr_move_addi_cmpa_bne_r`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then loads 0 into d0, then clears the memory at a0 (advancing a0), then copies d0 to a fixed memory address, then adds 1 to d0, then compares a1 with a0, then branches if the last result was not zero, then returns.
- `update_animation_frame_2`: Updates an animation frame by decrementing a counter and managing frame pointers, then performs byte-by-byte copying of sprite data from a source table to a destination buffer with calculated offsets.
- `frag_bsr_move_clr_rts_2`: Short code fragment that calls another routine, then copies an address to a fixed memory address, then copies an address to a fixed memory address, then copies an address to a fixed memory address, then clears a fixed memory address, then copies 5144 to a fixed memory address, then returns.
- `frag_move_btst_bne_rts_3`: Short code fragment that copies 33792 to a fixed memory address, then tests bit 14 of a fixed memory address, then branches if the last result was not zero, then copies 1024 to a fixed memory address, then returns.
- `frag_lea_sf_cmpa_bne_st_rts`: Short code fragment that loads the address of a fixed memory address into a0, then clears the memory at a0 (advancing a0), then compares a0 with an address, then branches if the last result was not zero, then sets the memory at a0 (advancing a0) to all ones, then compares a0 with an address, then branches if the last result was not zero, then returns.
- `frag_move_lea_swap_rts`: Short code fragment that copies an address to d0, then loads the address of a fixed memory address into a0, then copies d0 to the memory at an offset from a0, then swaps the halves of d0, then copies d0 to the memory at an offset from a0, then returns.
- `setup_video_registers`: Initializes video register values by writing data to fixed memory locations in a loop, then writes specific long values to multiple memory addresses.
- `update_animation_state_3`: Manages animation frame timing and state transitions by decrementing a counter, updating sprite positions based on frame states, and performing byte-wise data shuffling of sprite attributes.
- `frag_lea_move_swap_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies an address to d0, then copies d0 to the memory at an offset from a0, then swaps the halves of d0, then copies d0 to the memory at an offset from a0, then returns.
