# Game 4337 (marietto (1997)(l. carminati)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04337). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 88 of 88 named (0 author symbols), 88 of 88 described (16 by rule, 65 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 351 of 351 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2666 of 2666 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 149013 |
| code | 11356 |
| data | 47951 |
| padding | 1600 |
| unused | 691200 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Marietto (1997)(L. Carminati).adf:1 | 9 | 2648 | 87 |
| Marietto (1997)(L. Carminati).adf:bootblock | 1 | 18 | 1 |

## Cases handled

- 7 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Marietto (1997)(L. Carminati).adf:1

- `initialize_system`: Opens essential system libraries, configures graphics and input handling, and manages hardware state transitions including blitter ownership and coprocessor registers.
- `initialize_game_state_2`: Initializes multiple global variables with specific values, sets up timer-related data, and configures various game object parameters including position, velocity, and state flags.
- `update_game_state`: Checks a global variable's value and updates multiple global variables based on comparison results, then enters an infinite loop that repeatedly calls another routine and modifies various hardware registers and global variables.
- `update_player_position`: Updates player position based on joystick input and game state flags, handling movement acceleration and boundary checks.
- `update_sprite_positions`: Updates sprite position and state variables based on movement calculations and collision detection.
- `update_enemy_state`: Updates the state and position of an enemy object based on conditional checks and arithmetic operations, managing multiple global variables that control enemy behavior and rendering.
- `update_position_values`: Updates position and velocity values based on comparison conditions, modifies global state variables, and handles boundary checks for movement calculations.
- `frag_lea_moveq_bsr_addq_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads 3 into d1, then calls another routine, then adds 2 to a0, then loops back while counting down d1, then returns.
- `manage_object_timers`: Manages object timers by decrementing countdown values, calling routines when timers reach zero, and updating object state variables based on timer conditions in a loop of 4 iterations.
- `update_object_position`: Updates object position coordinates based on velocity values and handles boundary collisions for horizontal and vertical movement, modifying position and velocity components accordingly.
- `initialize_object`: Sets initial position and state values for an object, including horizontal coordinate, status flags, and timer counters.
- `check_collision_conditions`: Tests collision boundaries between objects using multiple conditional comparisons, clears flags when conditions are met, and sets up audio channel configurations.
- `update_audio_parameters`: Updates audio channel parameters by modifying global variables and writing computed values to specific memory locations, but only if certain conditions on input data are met.
- `copy_sprite_data`: Copies sprite data from one memory location to another in a loop, processing 8 iterations and updating a global counter variable.
- `conditional_copy_sprite`: Conditionally copies sprite data between memory locations based on a global flag, then decrements that flag.
- `update_animation_state`: Updates an animation state variable through a series of conditional checks and increments, managing a counter and transitioning between states based on comparison results.
- `handle_collision_response`: Manages collision response logic by checking various state conditions, performing arithmetic operations on coordinates, and triggering audio events based on collision detection.
- `update_animation_state_2`: Updates an animation state variable through a series of conditional checks and increments, managing transitions between different animation phases based on timer values and external conditions.
- `validate_input_range`: Validates input coordinates against predefined ranges and triggers another routine when conditions are met.
- `update_timer_state`: Manages a timer state machine that updates counters and transitions between states based on comparison results.
- `validate_position_bounds`: Checks position coordinates against boundary conditions and calls another routine when validation passes.
- `manage_timing_counters`: Manages timing counters for state transitions, performs multiplication operations on random values, and resets counters when specific conditions are met.
- `initialize_state_machine`: Initializes a state machine by setting global variables to default values and returns immediately.
- `update_game_state_2`: Updates a global variable's value based on coordinate differences and triggers audio setup routines when certain conditions are met.
- `process_coordinate_sequence`: Processes a sequence of coordinates using multiplication, bit shifting, and conditional negation to update position values.
- `frag_move_movea_moveq_bsr_rts`: Short code fragment that copies 1 to a fixed memory address, then copies 4864 to a fixed memory address, then copies 9252 to a6, then copies 5146 to d7, then copies 428 to d6, then loads 64 into d5, then calls another routine, then returns.
- `frag_move_add_cmp_bcs_addq_addi_rts`: Short code fragment that copies d0 to the stack, then adds d7 to a fixed memory address, then copies a fixed memory address to d0, then compares d0 with a fixed memory address, then branches if the carry flag is set, then adds 1 to a fixed memory address, then adds 10000 to a fixed memory address, then copies the top of the stack to d0, then returns.
- `update_circular_buffer`: Updates values in a circular buffer by adding previous values, applying bit rotation, and managing an index counter.
- `perform_multiple_blitter_operations_2`: Executes multiple blitter operations in a loop, updating source and destination addresses while configuring various blitter control register.
- `frag_cmpi_bcs_move_moveq_movea_bsr_rts`: Short code fragment that compares a fixed memory address with 2, then branches if the carry flag is set, then copies a location in the code to d6, then copies a location in the code to d7, then loads 1 into d4, then loads 16 into d5, then copies a fixed memory address to a6, then calls another routine, then returns.
- `validate_and_update_position_2`: Checks if a byte value from a global variable is less than or equal to 2, and if so, updates position coordinates and calls another routine with adjusted address calculations.
- `frag_cmpi_bne_move_moveq_lea_bsr_rts`: Short code fragment that compares a fixed memory address with 2, then branches if the last result was not zero, then copies a location in the code to d6, then copies a location in the code to d7, then loads 1 into d4, then loads 5 into d5, then loads the address of a fixed memory address into a6, then calls another routine, then returns.
- `setup_audio_channel_2_2`: Configures audio channel 2 by setting up DMA control registers, writing audio parameters to hardware registers, and managing bit flags in a.
- `frag_tst_beq_move_moveq_lea_bsr_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then copies a location in the code to d6, then copies a location in the code to d7, then loads 5 into d4, then loads 7 into d5, then loads the address of a fixed memory address into a6, then calls another routine, then returns.
- `frag_tst_beq_move_moveq_lea_bsr_rts_2`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then copies 34304 to d6, then copies 37248 to d7, then loads 3 into d4, then loads 7 into d5, then loads the address of a fixed memory address into a6, then calls another routine, then returns.
- `initialize_game_state`: Initializes game state variables by parsing command data and setting up memory locations with specific values, then returns after processing a loop of 8 iterations.
- `frag_movea_move_bsr_rts`: Short code fragment that copies 42786 to a6, then copies 14431 to d7, then copies 428 to d6, then copies 178 to d5, then calls another routine, then returns.
- `frag_lea_move_addi_moveq_bsr_rts`: Short code fragment that loads the address of a fixed memory address into a6, then copies a fixed memory address to d7, then adds 1600 to d7, then loads 40 into d6, then loads 5 into d5, then calls another routine, then returns.
- `copy_display_data`: Copies display list data from multiple source locations to destination buffers using indexed addressing, performs memory clearing operations on large blocks, and calls another routine for final processing.
- `iterate_command_data`: Iterates through command data using a loop that advances pointers and calls routines to process values, terminating when a specific byte value is reached.
- `format_score_entry`: Compares a value with data at a memory location, copies data between memory regions with loop control, and writes formatted text including spaces and a null terminator.
- `clear_and_initialize_memory`: Clears large blocks of memory with zero values, initializes various global variables, performs arithmetic operations on memory addresses, and calls multiple routines for further processing.
- `load_score_file`: Opens a score file, reads data into memory, and closes the file handle using DOS library calls.
- `write_score_file`: Opens a file using a specific name and mode, writes a fixed number of bytes from a global variable to the file, then closes the file.
- `frag_bsr_lea_move_rts`: Short code fragment that calls another routine, then loads the address of a fixed memory address into a6, then loads the address of a fixed memory address into a5, then calls another routine, then copies 10240 to a fixed memory address, then loads the address of a fixed memory address into a6, then loads the address of a location in the code into a5, then calls another routine (2 times in a row), then returns.
- `reset_game_variables`: Clears memory blocks with zero values, sets specific global variables to predefined values, calls initialization routines, and manages interrupt handling based on comparison results.
- `initialize_memory_blocks`: Clears large memory blocks with zero values, sets a specific global variable to a predefined value, calls initialization routines, and performs basic setup operations for game objects.
- `frag_lea_move_moveq_bsr_rts`: Short code fragment that loads the address of a fixed memory address into a6, then copies 0 to d7, then copies 10240 to d6, then loads 3 into d5, then calls another routine, then loads the address of a fixed memory address into a6, then copies an address to d7, then loads 1 into d5, then calls another routine, then returns.
- `fill_memory_pattern`: Fills memory with a repeating pattern of values by loading from specific addresses and performing arithmetic operations.
- `frag_ori_move`: Short code fragment that merges 0 into d0, then copies the memory at an offset from a6 to d7 (4 times in a row).
- `frag_subq_move_swap_addq_add_dbra_rts`: Short code fragment that subtracts 1 from d5, then copies d7 to the memory at an offset from a6, then swaps the halves of d7, then copies d7 to the memory at an offset from a6, then swaps the halves of d7, then adds 8 to a6, then adds d6 to d7, then loops back while counting down d5, then returns.
- `initialize_coprocessor`: Sets up coprocessor registers using data from a table, stores allocation results in global variables, and manages a data processing loop before marking initialization as complete.
- `process_pixel_coordinates`: Iterates through pixel coordinate data, performs calculations on values, stores results in memory locations, and manages data processing loops with conditional branches.
- `process_pixel_data`: Processes pixel coordinate data through a loop that shifts and combines values, then returns.
- `validate_coordinates`: Checks if a value at a specific memory location equals -2, or validates bit flags and range conditions before returning.
- `update_display_list`: Updates display list pointers and clears a global flag after processing coordinate data, then frees allocated memory.
- `frag_movem_move_beq_bsr_addq_bra_rts`: Short code fragment that saves registers on the stack, then copies the memory at a5 (advancing a5) to d0, then branches if the last result was zero, then calls another routine, then adds 1 to a6, then jumps to another point in the code, then restores saved registers from the stack, then returns.
- `copy_pixel_data`: Copies pixel data from a source table to multiple destination locations with byte-level addressing and conditional copying.
- `parse_command_data`: Parses command data by iterating through a table, performing calculations on values, and storing results in memory locations using indexed addressing with specific offsets.
- `decode_movement_commands`: Decodes movement command sequences from input data, updates state variables based on command types, and manages memory addressing for coordinate calculations.
- `parse_chunk_data`: Reads a chunk header identifier and processes width and height values from a structure, then copies data rows using a byte sequence copy routine.
- `parse_chunk_header`: Parses a chunk header by comparing against a specific identifier, then advances the pointer and continues parsing.
- `copy_byte_sequence`: Copies byte sequences from input to output locations, handling both positive and negative count values with different copy strategies.
- `frag_move_rts`: Short code fragment that copies an address to a fixed memory address (2 times in a row), then returns.
- `validate_and_update_position`: Checks boundary conditions for position coordinates and updates a global linked list pointer when conditions are met, then stores a termination value in the list.
- `update_position_bounds`: Updates a position value in memory based on boundary checks, adjusting it when a calculated offset exceeds certain limits.
- `calculate_and_store_coordinates`: Calculates coordinate values and stores them in memory locations, with boundary checking that may update a stored value if conditions are met.
- `perform_multiple_blitter_operations`: Executes multiple blitter operations in a loop, updating source and destination addresses while configuring various blitter control registers for each iteration.
- `execute_blitter_dma_sequence`: Processes a sequence of blitter DMA operations by iterating through memory addresses, calculating values, and setting up blitter registers for each operation.
- `frag_bchg_bne_move_rts`: Short code fragment that flips bit 0 of a fixed memory address, then branches if the last result was not zero, then copies 0 to a fixed memory address, then copies 13868 to a fixed memory address, then returns, then copies 51200 to a fixed memory address, then copies 14384 to a fixed memory address, then returns.
- `wait_for_vpos_bit_low_then_high`: Polls bit zero of the custom chip register at offset five until it reads low, then polls again until it reads high, then returns.
- `frag_subq_bsr_dbra_rts`: Short code fragment that subtracts 1 from d7, then calls another routine, then loops back while counting down d7, then returns.
- `compare_and_update_value`: Compares a value with another repeatedly using a helper routine, updating control flow based on comparison results until certain conditions are met.
- `frag_move_lsr_andi_rts`: Short code fragment that copies a fixed memory address to d7, then shifts d7 right by 8, then masks d7 with 511, then returns.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `process_text_data`: Processes character data from a source table, copying bytes to a destination buffer while handling specific control characters and updating pointers.
- `frag_btst_bne_moveq_move_bset_lea_rts`: Short code fragment that tests bit 7 of a location in the code, then branches if the last result was not zero, then loads 0 into d0, then copies a location in the code to d0, then sets bit 7 of a fixed memory address, then loads the address of a location in the code into a0, then copies an indexed entry from a0 to d0, then returns, then loads 0 into d0, then returns.
- `setup_audio_channel_0`: Configures audio channel 0 by setting up DMA control registers, writing audio parameters to hardware registers, and managing bit flags in a global variable.
- `setup_audio_channel_1`: Configures audio channel 1 by setting up DMA control registers, writing audio parameters to hardware registers, and managing bit flags in a global variable.
- `setup_audio_channel_2`: Configures audio channel 2 by setting up DMA control registers, writing audio parameters to hardware registers, and managing bit flags in a global variable.
- `setup_audio_channel_3`: Configures audio channel 3 by setting up DMA control registers, writing audio parameters to hardware registers, and managing bit flags in a global variable.
- `update_audio_dma_lengths`: Updates DMA lengths for four audio channels based on a global variable and hardware status flags, then returns.
- `mute_all_audio_channels`: The routine sets the volume registers for all four audio channels to zero, effectively silencing the sound hardware.
- `setup_graphics_registers`: Sets up graphics hardware registers including sprite positions, coprocessor addresses, interrupt configurations, and DMA control registers, then calls routines to manage task execution and blitter operations.
- `handle_input_interrupt`: Handles input interrupts by reading hardware registers, checking bit conditions, updating global variables based on input data, and managing interrupt enable/disable states with specific register writes.
- `setup_system_interrupts`: Initializes interrupt and DMA control registers, calls several system routines for task management, and returns.
- `configure_system_resources`: Configures system resources by managing task priorities, disabling interrupts, calling various system routines for resource management, and setting up graphics hardware registers with specific values.

### Marietto (1997)(L. Carminati).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
