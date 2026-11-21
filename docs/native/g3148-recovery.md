# Game 3148 (classic racer (2000)(relay soft)(de)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03148). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 59 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 158 of 158 named (0 author symbols), 158 of 158 described (41 by rule, 112 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 569 of 569 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9136 of 9136 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 195751 |
| code | 48708 |
| data | 635848 |
| padding | 16205 |
| unused | 4608 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Classic Racer (2000)(Relay Soft)(De).adf:ClassicRacer | 5 | 9118 | 157 |
| Classic Racer (2000)(Relay Soft)(De).adf:bootblock | 1 | 18 | 1 |

## Cases handled

- classic racer (2000)(relay soft)(de)/Classic Racer (2000)(Relay Soft)(De).adf:Docs/cr_kurs2.png: high-entropy data file (7.88 bits/byte), classified as packed data
- classic racer (2000)(relay soft)(de)/Classic Racer (2000)(Relay Soft)(De).adf:Docs/marble.png: high-entropy data file (7.96 bits/byte), classified as packed data
- classic racer (2000)(relay soft)(de)/Classic Racer (2000)(Relay Soft)(De).adf:Docs/cr_kurs7.png: high-entropy data file (7.79 bits/byte), classified as packed data
- classic racer (2000)(relay soft)(de)/Classic Racer (2000)(Relay Soft)(De).adf:Docs/cr_shot1.png: high-entropy data file (7.87 bits/byte), classified as packed data
- classic racer (2000)(relay soft)(de)/Classic Racer (2000)(Relay Soft)(De).adf:Docs/cr_kurs5.png: high-entropy data file (7.80 bits/byte), classified as packed data
- classic racer (2000)(relay soft)(de)/Classic Racer (2000)(Relay Soft)(De).adf:Docs/cr_kurs3.png: high-entropy data file (7.88 bits/byte), classified as packed data
- classic racer (2000)(relay soft)(de)/Classic Racer (2000)(Relay Soft)(De).adf:Docs/cr_icon.png: high-entropy data file (7.95 bits/byte), classified as packed data
- classic racer (2000)(relay soft)(de)/Classic Racer (2000)(Relay Soft)(De).adf:Docs/cr_kurs1.png: high-entropy data file (7.83 bits/byte), classified as packed data
- classic racer (2000)(relay soft)(de)/Classic Racer (2000)(Relay Soft)(De).adf:Docs/cr_shot8.png: high-entropy data file (7.97 bits/byte), classified as packed data
- classic racer (2000)(relay soft)(de)/Classic Racer (2000)(Relay Soft)(De).adf:Docs/reso.png: high-entropy data file (7.89 bits/byte), classified as packed data
- classic racer (2000)(relay soft)(de)/Classic Racer (2000)(Relay Soft)(De).adf:Docs/cr_shot2.png: high-entropy data file (7.83 bits/byte), classified as packed data
- classic racer (2000)(relay soft)(de)/Classic Racer (2000)(Relay Soft)(De).adf:Docs/cr_kurs6.png: high-entropy data file (7.85 bits/byte), classified as packed data
- classic racer (2000)(relay soft)(de)/Classic Racer (2000)(Relay Soft)(De).adf:Docs/cr_kurs4.png: high-entropy data file (7.81 bits/byte), classified as packed data
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 5 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 3 immediate, 2 address

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Classic Racer (2000)(Relay Soft)(De).adf:ClassicRacer

- `initialize_game_system`: Initializes game system resources by allocating memory, setting up task management, and handling message queues, then cleans up allocated resources and returns a value from a global variable.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `initialize_game_library`: Initializes a game library by opening and closing libraries, copying data from a table to memory, and setting up a status byte bit.
- `setup_graphics`: Sets up graphics resources by opening a library, storing the result to a global variable, and calling another routine when the result is non-zero.
- `cleanup_library_handles`: Cleans up library handles in a table by closing each library and then freeing the memory allocated for the table.
- `frag_st_lea_bsr_rts`: Short code fragment that sets a fixed memory address to all ones, then loads the address of a location in the code into a0, then loads the address of a location in the code into a1, then calls another routine, then returns.
- `parse_text_buffer`: Parses text from an input buffer, skipping whitespace and special characters, and writes the result to an output buffer.
- `frag_cmpi_bne_move_lea_bra`: Short code fragment that compares the memory at a0 with 34, then branches if the last result was not zero, then copies 34 to the memory at a1 (advancing a1), then loads the address of the memory at an offset from a0 into a0, then jumps to another point in the code.
- `frag_cmpi_bne_move_lea_bra_2`: Short code fragment that compares the memory at a0 with 110, then branches if the last result was not zero, then copies 10 to the memory at a1 (advancing a1), then loads the address of the memory at an offset from a0 into a0, then jumps to another point in the code.
- `frag_tst_beq_sf_bra`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then clears a fixed memory address, then jumps to another point in the code.
- `frag_st_bra`: Short code fragment that sets a fixed memory address to all ones, then jumps to another point in the code.
- `frag_tst_bne_clr_move_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then tests a fixed memory address, then branches if the last result was not zero, then clears the memory at a1, then copies a location in the code to d0, then returns.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `open_system_libraries`: Opens several system libraries including graphics, intuition, and dos, sets up task priorities, and handles errors by calling another routine when a library fails to open.
- `initialize_audio_and_input`: Initializes audio channel registers with specific volume settings, configures input handling by setting up joystick and keyboard interfaces, and manages system task synchronization through OS library calls.
- `execute_library_functions`: Executes several library functions in sequence with different parameters, performing operations like setting up graphics and handling input events.
- `frag_tst_beq_movea_move_jsr_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then copies a fixed memory address to a1, then copies a fixed memory address to d0, then copies a fixed memory address to a6, then calls another routine, then copies 0 to a fixed memory address, then returns.
- `initialize_and_process`: Initializes multiple variables from global data, performs arithmetic operations on counters, and calls another routine with specific parameters while iterating through a table.
- `increment_and_loop`: Increments a counter in a global variable and loops until it reaches a specific value, then calls another routine and returns.
- `frag_move_movem_jsr_rts`: Short code fragment that copies d0 to the memory just below a1 (moving a1 down), then restores saved registers from the stack, then copies 5 to d1, then calls another routine, then returns.
- `frag_move_cmpi_rts`: Short code fragment that copies 0 to a fixed memory address, then compares a fixed memory address with 256, then returns.
- `frag_move_cmpi_rts_2`: Short code fragment that copies 0 to a fixed memory address, then compares a fixed memory address with 1, then returns.
- `frag_move_cmpi_rts_3`: Short code fragment that copies 0 to a fixed memory address, then compares a fixed memory address with 3, then returns.
- `read_joystick_input`: Reads joystick data from a hardware register and clears a test register, then returns immediately.
- `initialize_input_handlers`: Initializes input handling by setting global variables, configuring hardware registers for keyboard input, and establishing a loop that monitors specific keyboard scan codes.
- `initialize_game_state_3`: Initializes global variables related to game rendering and input handling, sets up hardware registers for keyboard and joystick input, and configures audio DMA channels.
- `copy_memory_blocks`: Copies data between two memory blocks in chunks of 6 and 3 words respectively, then advances the pointers by 8 bytes each.
- `update_object_data`: Updates object data in a table by storing coordinate values from another table, using a counter to determine the target index, and increments the counter after each update.
- `handle_state_changes`: Checks a value against specific constants and sets a word at a fixed memory address to zero when the value matches 6 or 11.
- `process_game_data`: Processes game data by performing arithmetic operations on coordinates, calling library functions for graphics rendering, and updating counters.
- `render_game_objects`: Renders game objects by calling a graphics library function multiple times with different parameters, processing coordinates from memory locations.
- `render_game_object_1`: Processes and renders specific game object data by calling a graphics library function with coordinate values and parameters from memory.
- `render_game_object_2`: Processes and renders another type of game object by calling a graphics library function with different coordinate values and parameters from memory.
- `render_game_object_3`: Processes and renders a third type of game object by calling a graphics library function with specific coordinate values and parameters from memory.
- `update_state_counter`: Updates a state counter in a global variable based on comparisons with fixed values, and performs conditional resets of another global variable.
- `setup_graphics_params`: Calculates scaled coordinate values from global variables and sets up parameters for graphics rendering by loading display coordinates and calling graphics library functions.
- `setup_race_graphics`: Initializes graphics rendering parameters by calculating scaled values from memory locations, setting up display coordinates, and calling graphics library functions with specific parameters.
- `format_race_strings`: Formats and processes string data for race-related display elements using library routines, handling multiple text segments with varying parameters.
- `initialize_game_state_2`: Initializes game state variables and handles input processing by reading hardware registers and updating global flags.
- `update_game_state_2`: Updates game state variables based on input conditions and handles rendering updates.
- `initialize_race_session`: Initializes race session variables and handles input processing based on joystick and keyboard events, including checking for specific button states and updating global game state flags.
- `update_race_state`: Updates race-related game state variables and handles input processing based on joystick and keyboard events, including checking for specific button presses and managing hardware registers.
- `initialize_race_sequence_2`: Sets up race-related game state variables and initializes hardware registers before starting the main game loop.
- `initialize_race_sequence`: Sets up race-related game state variables and initializes hardware registers before starting the main game loop.
- `initialize_game_state`: Sets up initial game state variables and performs hardware register initialization before entering the main game loop.
- `handle_input_and_render`: Processes input from joystick and keyboard, updates rendering state, and handles game loop control flow based on input conditions.
- `format_and_process_input`: Formats a string by copying characters from one location to another, appends specific file extension bytes, and processes input data through multiple library function calls based on conditional checks.
- `process_input_data_2`: Processes input data by iterating through a table of words, checking conditions on each word, and calling another routine with specific parameters.
- `frag_negx_movem_bra_move_movea_jsr`: Short code fragment that negates d0 with the extend flag, then restores saved registers from the stack, then jumps to another point in the code, then saves registers on the stack, then copies 132 to d2, then copies d6 to d3, then copies 31 to d4, then copies 27658 to a3, then calls another routine, then jumps to another point in the code.
- `save_and_process_data`: Saves registers to stack, writes a word value from d7 to a global variable at offset 30028, restores registers from stack, then saves registers again before making a library call.
- `frag_negx_movem_move_movea_jsr_bra`: Short code fragment that negates d0 with the extend flag, then restores saved registers from the stack, then saves registers on the stack, then copies 85 to d2, then copies d6 to d3, then copies 31 to d4, then copies 27750 to a3, then calls another routine, then jumps to another point in the code.
- `copy_and_format_string`: Copies character data between memory locations, processes input values through conditional checks, and formats string output by writing specific byte sequences to a buffer.
- `frag_move_movem_adda_addi_dbra_rts`: Short code fragment that copies d0 to the memory just below a1 (moving a1 down), then restores saved registers from the stack, then adds 31 to a4, then adds 10 to d6, then loops back while counting down d5, then restores saved registers from the stack, then returns.
- `initialize_input_buffer`: Initializes a global input buffer by setting all entries to zero, then processes input data by comparing values against a global variable and updating a counter accordingly.
- `update_and_invoke_operation`: Negates a byte in d0, restores saved registers from stack, updates a global word variable at offset 30028 by incrementing it, saves registers again, then makes a library call.
- `frag_negx_movem_move_movea_jsr_bra_2`: Short code fragment that negates d0 with the extend flag, then restores saved registers from the stack, then saves registers on the stack, then copies 85 to d2, then copies d6 to d3, then copies 29 to d4, then copies 28364 to a3, then calls another routine, then jumps to another point in the code.
- `process_input_buffer`: Reads characters from a hardware keyboard register, processes them based on global state, and updates a buffer while managing counters and status flags.
- `process_input_character`: Reads a character from a hardware register, processes it based on global state, and stores the result in a buffer while updating counters and pointers.
- `frag_move_movem_addi_cmp_beq_bra`: Short code fragment that copies d0 to the memory just below a1 (moving a1 down), then restores saved registers from the stack, then adds 8 to d5, then compares d7 with a fixed memory address, then branches if the last result was zero, then jumps to another point in the code.
- `process_input_selection`: Handles input selection by checking a global variable's value and calling different routines based on whether it matches specific numeric characters.
- `store_and_execute_2`: Stores a data register value onto the stack, restores all registers from the stack, returns, then calls a routine with fixed parameters and.
- `store_and_execute`: Stores a data register value onto the stack, restores all registers from the stack, returns, then calls a routine with fixed parameters and branches to a specific address.
- `store_and_call`: Stores a long word from data register 0 onto the stack, restores all data and address registers from the stack, then returns.
- `frag_move_movem_rts`: Short code fragment that copies d0 to the memory just below a1 (moving a1 down), then restores saved registers from the stack, then returns.
- `handle_input_selection_2`: Processes input selection by checking a global variable's value and calling different routines based on whether it matches specific numeric characters, with a fallback to another routine.
- `execute_with_different_parameters`: Saves all registers to the stack, restores them, then calls a routine with different fixed parameters including another specific address value.
- `execute_with_parameters`: Stores a data register value onto the stack, restores all registers from the stack, returns, then calls a routine with fixed parameters and branches to a specific address.
- `store_and_call_2`: Stores a value from a register onto the stack, then calls a routine using a pointer from another routine's data, and finally branches to a specific address.
- `frag_move_movem_rts_2`: Short code fragment that copies d0 to the memory just below a1 (moving a1 down), then restores saved registers from the stack, then returns.
- `initialize_render_context_2`: Sets up rendering context parameters by copying data from various structures and calling graphics library functions with specific configurations, then performs conditional operations based on a global variable's value.
- `update_render_state_2`: Updates rendering state variables including position data and global counters, performs graphics library calls with specific parameters, and manages display attribute configurations.
- `check_input_state`: Checks keyboard input state by reading from hardware registers, comparing values against specific constants, and returning early when matches are found.
- `update_object_position`: Updates object position data by calculating scaled values from table entries and storing them in a buffer, then increments a global counter.
- `update_velocity`: Calculates and updates velocity components based on input values, then performs boundary checks and updates global position variables accordingly.
- `update_game_state`: Updates game state variables based on input conditions and performs conditional logic to set values in global memory, including handling specific state transitions and updating counters.
- `select_render_parameters`: Selects rendering parameters based on a state value and calls graphics functions with specific coordinates and color values.
- `initialize_rendering_2`: Saves all registers, initializes rendering parameters, and calls graphics library routines with specific coordinates and dimensions.
- `render_frame`: Performs multiple graphics operations including drawing with different parameters, calling a rendering function, and making library calls for display updates.
- `validate_and_update_coordinates`: Validates coordinate parameters against boundary conditions and updates global variables with clamped coordinate values.
- `initialize_graphics`: Sets specific values in memory locations and calls functions to process game data and input strings.
- `process_input_string`: Processes character data from a string by reading bytes and calling different routines based on character values, handling digits 0-9 through conditional branches.
- `initialize_and_call`: Initializes two pointers from global data and calls a library routine twice, then makes several calls to another routine with different parameters.
- `load_and_execute`: Loads values from global memory locations and executes two library calls with different parameters, repeating the sequence twice with different pointer sources.
- `frag_jsr_rts_5`: Short code fragment that calls another routine (2 times in a row), then returns.
- `frag_jsr_rts`: Short code fragment that calls another routine (5 times in a row), then returns.
- `frag_jsr_rts_2`: Short code fragment that calls another routine (5 times in a row), then returns.
- `frag_jsr_rts_3`: Short code fragment that calls another routine (4 times in a row), then returns.
- `frag_jsr_rts_4`: Short code fragment that calls another routine (5 times in a row), then returns.
- `frag_jsr_rts_6`: Short code fragment that calls another routine (6 times in a row), then returns.
- `program`: Calls an initialization routine, a main routine, and an exit routine before returning.
- `frag_jsr_rts_7`: Short code fragment that calls another routine (7 times in a row), then returns.
- `frag_jsr_rts_8`: Short code fragment that calls another routine (6 times in a row), then returns.
- `frag_jsr_rts_9`: Short code fragment that calls another routine (6 times in a row), then returns.
- `update_global_values`: Updates four global variables with values from registers and performs arithmetic operations on them, then calls another routine using a pointer in global memory.
- `update_coordinates`: Updates two global memory locations with values from registers and performs arithmetic operations on them before calling another routine.
- `adjust_position`: Modifies global variables with register values using addition operations, then calls a routine that executes library functions with specific parameters.
- `compute_offset`: Calculates new values for global memory locations by adding constants to register contents and performs arithmetic operations before calling another routine.
- `set_boundary_values`: Stores register values into global memory locations with specific offsets, performs addition operations on them, and calls a library function with parameters.
- `store_and_accumulate`: Writes values from two data registers to global variables at specific offsets, performs arithmetic operations on the stored values, and executes a library routine using the computed results as parameters.
- `initialize_context`: Copies register values to global memory locations, performs addition operations, and calls another routine that executes library functions with parameters.
- `update_positions`: Updates position coordinates based on comparisons with boundary values, performs arithmetic operations, and calls rendering functions when conditions are met.
- `frag_tst_ble_sub_move_rts`: Short code fragment that tests a fixed memory address, then branches if the comparison was less or equal, then subtracts d5 from a fixed memory address, then copies 1 to d7, then returns.
- `frag_move_subi_cmp_ble_add_rts`: Short code fragment that copies a fixed memory address to d0, then subtracts 200 from d0, then compares d0 with a fixed memory address, then branches if the comparison was less or equal, then adds d5 to a fixed memory address, then copies 1 to d7, then returns.
- `frag_tst_ble_sub_move_rts_2`: Short code fragment that tests a fixed memory address, then branches if the comparison was less or equal, then subtracts d6 from a fixed memory address, then copies 1 to d7, then returns.
- `frag_move_subi_cmp_ble_add_rts_2`: Short code fragment that copies a fixed memory address to d0, then subtracts 251 from d0, then compares d0 with a fixed memory address, then branches if the comparison was less or equal, then adds d6 to a fixed memory address, then copies 1 to d7, then returns.
- `setup_rendering`: Saves all registers, checks a condition in global memory, and calls graphics routines with different parameters depending on the condition's state before restoring registers.
- `frag_btst_ori_andi`: Short code fragment that tests bit d0 of a fixed memory address, then merges 0 into d0, then tests bit d0 of a fixed memory address, then merges 0 into d0, then masks d0 with 0.
- `validate_parameters`: Compares four input values against constants and branches to an exit label if any condition is not met, otherwise returns immediately.
- `frag_btst_beq_move_rts`: Short code fragment that tests bit 5 of a fixed memory address, then branches if the last result was zero, then copies 32 to a fixed memory address, then returns.
- `frag_move_rts`: Short code fragment that copies 32800 to a fixed memory address, then returns.
- `process_input_data`: Processes input data by extracting and transforming nibble values from a source table into a destination table, then performs conditional operations based on a global state variable before calling another routine and returning.
- `update_game_counters`: Increments two global counters and checks a keyboard input value, branching based on the comparison result.
- `handle_keyboard_input`: Reads a byte from the keyboard serial register and stores specific numeric values in a global variable based on the input character.
- `configure_audio`: Configures audio settings by calling a library routine twice with specific parameters, and manages a global flag to control the configuration flow.
- `setup_audio_parameters`: Sets a flag in a global variable and calls a library routine twice with specific parameters to configure audio settings.
- `initialize_audio_system`: Initializes audio settings by configuring multiple audio parameters through library calls, using values loaded into registers and pointers to global structures.
- `configure_audio_settings`: Sets a flag in a global variable and makes multiple calls to a library routine with varying parameters to set up different audio configurations.
- `handle_input_state`: Checks keyboard input state and updates audio DMA configuration based on game conditions, then calls another routine to process input.
- `frag_move_subi_dbra_rts`: Short code fragment that copies d0 to d7, then subtracts 1 from d7, then copies the memory at a0 (advancing a0) to the memory at a6 (advancing a6), then loops back while counting down d7, then returns.
- `setup_audio_channel`: Configures audio channel 0 DMA settings including length, period, and volume values, then initializes audio playback parameters.
- `setup_audio_channel_3`: Configures audio channel 3 DMA settings including length, period, and volume values, then initializes audio playback parameters.
- `setup_audio_channel_1`: Sets up audio channel 1 DMA registers with specific length, period, and volume settings while updating related data structures.
- `initialize_audio_dma`: Configures DMA channel 2 for audio playback by setting registers and calling another routine, then updates audio volume and length values based on calculations.
- `update_audio_dma_state`: Updates various audio DMA state flags and volume settings by decrementing counters and writing to hardware registers based on conditional checks.
- `update_race_data`: Updates race-related data by copying blocks of memory and adjusting pointers, then resets counters and recalculates memory addresses based on a global variable's value.
- `render_object_coordinates`: Processes coordinate values through a series of arithmetic operations and calls rendering functions with specific context pointers.
- `clear_input_counter`: Initializes a global counter to zero and reads from CIAA_PRA register, clearing bit 7 if it's not set.
- `grid_render_loop`: Executes a nested loop that calls multiple graphics routines, processing grid coordinates and updating rendering parameters until reaching specific boundary conditions.
- `update_counter_and_divide`: Increments a global counter value and performs division operations on input parameters to calculate results.
- `multiply_accumulate`: Performs 32-bit multiplication of two 16-bit values with carry propagation, accumulating results into a 32-bit output value through multiple arithmetic operations.
- `process_compressed_data`: Decodes compressed data sequences by reading from source pointers, copying values to destination locations, and updating control flags.
- `decode_and_copy_data`: Processes compressed data by reading bytes from source and destination pointers, handling both direct copying and negative-length sequences with bit manipulation.
- `frag_adda_movea_move_mulu_dbra_rts`: Short code fragment that adds 196 to a0, then copies 6562 to a1, then copies a fixed memory address to d0, then copies d0 to d1, then multiplies d1 by 4 (unsigned), then copies an indexed entry from a0 to an indexed entry from a1, then loops back while counting down d0, then returns.
- `unpack_and_store_data`: Iterates 32 times to extract 4-bit values from a source buffer, combines them into 16-bit words, and stores the results in a destination table.
- `frag_adda_movea_move_jsr_rts`: Short code fragment that adds 36 to a0, then copies 6602 to a1, then copies 32 to d0, then copies a fixed memory address to a6, then calls another routine, then returns.
- `frag_adda_movea_move_jsr_rts_2`: Short code fragment that adds 36 to a0, then copies 6730 to a1, then copies 32 to d0, then copies a fixed memory address to a6, then calls another routine, then returns.
- `compare_and_update_pointers`: Compares memory values between two structures, updates pointer chains when matches are found, and copies specific field values from one structure to global variables.
- `initialize_render_context`: Saves registers, sets up rendering context pointers, calls multiple graphics routines with different parameters, and performs conditional rendering based on comparison results.
- `setup_render_context`: Saves all registers, initializes rendering context pointers, calls several graphics routines with specific parameters, and returns without additional processing.
- `configure_display_attributes`: Calls graphics library routines to configure display attributes, setting up color and positioning parameters for screen rendering operations.
- `allocate_and_initialize_memory`: Allocates memory blocks for audio processing, initializes various data structures including length, volume, and pointer values, then sets up multiple memory regions.
- `free_allocated_memory`: Frees previously allocated memory blocks and performs cleanup operations on audio data structures when memory is released.
- `frag_move_cmp_bgt_rts`: Short code fragment that copies 0 to a fixed memory address, then compares d1 with a fixed memory address, then branches if the comparison was greater, then returns.
- `frag_moveq_move_rts`: Short code fragment that loads 0 into d0, then copies a fixed memory address to d0, then returns.
- `frag_jsr_cmp_blt_rts`: Short code fragment that calls another routine, then compares d0 with 250, then branches if the comparison was less, then returns.
- `validate_and_render_coordinates`: Validates coordinate parameters against boundary conditions, then calls rendering routines with specific context pointers based on validation results.
- `update_render_state`: Updates rendering state by calling graphics routines with parameters loaded from global tables, and iterates through a byte buffer until encountering a zero terminator.
- `initialize_rendering`: Initializes rendering parameters by setting up coordinates and calling graphics library functions with specific values, then processes a buffer until encountering a null byte before returning.
- `update_render_context`: Updates rendering context pointers and performs conditional operations based on a global variable's value, then writes a status byte to a hardware register.
- `frag_movem_movea_jsr_move_rts`: Short code fragment that saves registers on the stack, then copies 49552 to a0, then calls another routine, then copies 0 to a fixed memory address, then copies 135 to a fixed memory address, then copies a fixed memory address to a0, then calls another routine, then restores saved registers from the stack, then returns.
- `format_integer_string`: Formats integer values into string representations using multiple library calls, then processes the formatted output for display.
- `process_string_formatting`: Handles string formatting operations by checking special character markers and processing input data through various library functions.
- `configure_rendering_context`: Sets up rendering context parameters by copying data from various structures and calling graphics library functions with specific configuration values.
- `format_integer`: Converts a signed integer into a formatted string representation, handling negative numbers and removing leading zeros, storing the result in a global buffer.
- `initialize_course_data`: Initializes course data by copying strings from a global buffer into a table, checking for specific file extensions, and updating a counter.
- `load_and_store`: Loads a value from a global variable, calls library functions using that value, and stores another value from a global variable to a specific location in memory.
- `update_game_loop`: Processes game loop iterations by performing division operations on counters, calling library functions for input handling, and updating position values.

### Classic Racer (2000)(Relay Soft)(De).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
