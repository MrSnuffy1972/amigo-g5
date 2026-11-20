# Game 3774 (gardener (1995)(intersoft)(de)[h smurf]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03774). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (907184 top-level bytes in 14 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 151 of 151 named (0 author symbols), 151 of 151 described (35 by rule, 84 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1417 of 1417 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6530 of 6530 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 361274 |
| code | 19810 |
| data | 145292 |
| padding | 2952 |
| unused | 377856 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Type | 1 | 426 | 2 |
| Gardener (1995)(Intersoft)(De)[h Smurf].adf:Gardener | 2 | 2999 | 45 |
| Gardener (1995)(Intersoft)(De)[h Smurf].adf:Type | 1 | 426 | 2 |
| Gardener (1995)(Intersoft)(De)[h Smurf].adf:SMURF | 2 | 198 | 7 |
| Gardener (1995)(Intersoft)(De)[h Smurf].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 418 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 816 | 13 |
| Gardener (1995)(Intersoft)(De)[h Smurf].adf:SMURF#unpacked | 1 | 1228 | 72 |

## Cases handled

- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 389 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 333 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- gardener (1995)(intersoft)(de)[h smurf]/Gardener (1995)(Intersoft)(De)[h Smurf].adf:SMURF: crunched executable decrunched by XFDMaster (StoneCracker 4.04 (xfd), 17784 -> 139024 bytes), 14 of 15 windows confirmed in reference RAM: recovered as a hunk executable
- gardener (1995)(intersoft)(de)[h smurf]/Gardener (1995)(Intersoft)(De)[h Smurf].adf:SMURF: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_graphics_and_interrupts` (runtime segment an address): Sets up graphics registers including DMA control, interrupt masks, and copper list pointers. Kind: disk hardware check (ADKCON). Pass path: The routine initializes hardware registers including ADKCON and waits for specific VPOS values and CIA input. If the check passes (i.e., the expected hardware state or signal is detected), it continues execution and eventually reaches the `.
- `initialize_video_system` (SMURF#unpacked): Sets up video hardware registers including DMA control, interrupt masks, and copper list pointers. Kind: disk hardware check using ADKCON and custom video initialization. Pass path: The routine initializes video system and waits for specific VPOSR values, suggesting a sync or timing-based check. The loop involving VPOSR and CIAA_PRA indicates a potential manual/code-wheel or hardware-based protection check that must pa.

## Functions


### Type

- `parse_type_command_arguments`: Parses command-line arguments to identify hex or number flags, validates that they are not used simultaneously, and handles error reporting for invalid options or file access failures.
- `format_output_string`: Formats strings and numbers into a character buffer using specific text patterns and hexadecimal characters.

### Gardener (1995)(Intersoft)(De)[h Smurf].adf:Gardener

- `initialize_gardener_system`: Sets up system resources including graphics and interrupt handlers, initializes global variables related to video display and input devices, and configures hardware registers for DMA and blitter operations.
- `initialize_gardener_data`: Initializes multiple global variables and memory locations with specific values, sets up data structures for gardener object rendering, and manages state transitions based on counter conditions.
- `frag_lea_moveq_move_addq_dbra_rts`: Short code fragment that loads the address of a location in the code into a0, then loads the address of a fixed memory address into a1, then loads 31 into d7, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then adds 2 to a1, then loops back while counting down d7, then returns.
- `frag_movem_tst_bne_bsr_beq_moveq_rts`: Short code fragment that saves registers on the stack, then tests a fixed memory address, then branches if the last result was not zero, then calls another routine, then tests a fixed memory address, then branches if the last result was zero, then calls another routine, then restores saved registers from the stack, then loads 255 into d0, then returns.
- `handle_keyboard_input`: Reads a byte from the keyboard serial register, sets a bit in a custom register, clears the keyboard register, calls another routine three times, clears a bit in the custom register, and stores the processed byte value at a specific memory location.
- `frag_move_cmp_beq_rts`: Short code fragment that copies the memory at an offset from a6 to d1, then compares d1 with the memory at an offset from a6, then branches if the last result was zero, then returns.
- `update_object_fields`: Checks if a global variable is zero, and if so, copies data from registers to a structure pointed to by another variable, updates multiple fields including position and type, increments a counter, and performs bounds checking.
- `process_audio_channels`: Processes audio channel data by decrementing counters, setting up DMA registers when counters reach zero, and managing audio playback state transitions.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `synchronize_video_dma`: Waits for specific vertical blanking conditions in custom hardware registers, copies memory locations between two data structures, and then writes word values to a series of memory locations with calculated offsets.
- `update_gardener_state`: Processes input from joystick port 1 to update gardener object state, including movement direction and animation timing.
- `process_object_fields`: Processes object fields based on a global variable value, calls different routines for different cases, updates multiple structure fields, and performs arithmetic operations on data pointers.
- `update_positions`: Processes a sequence of data structures, updating position values and managing counters.
- `manage_timers`: Handles timer-based operations for data structures, decrementing countdown values and managing state transitions.
- `handle_counters`: Manages counter values for data structures, decrementing timers and incrementing counters.
- `setup_render_data`: Calculates memory addresses based on input parameters, checks if a memory location contains a specific value, and if not, stores calculated addresses and values in two different data structures.
- `initialize_render_state`: Clears a specific byte in a data structure, calculates memory addresses based on input parameters, checks a bit in another structure, and if the bit is not set, stores several predefined values in the structure.
- `load_tile_value`: Reads a 32-bit value from a table using two parameters as indices, then stores the value in a global variable while masking it to 8 bits.
- `initialize_graphics_data`: Sets up graphics parameters including memory addresses and word values in specific structure offsets based on input data, and configures display attributes for multiple graphic elements.
- `setup_display_attributes`: Checks a bit in a structure, and if not set, stores several predefined values including memory addresses and word values in specific offsets of that structure.
- `calculate_sprite_positions`: Performs arithmetic operations on input parameters to calculate sprite positions, updates memory locations with calculated values, and stores several predefined values in a structure based on the calculations.
- `clear_render_queue`: Reads memory addresses from a queue structure, clears specific memory locations, and if a bit is not set in another structure, stores several predefined values including memory addresses and word values in that structure.
- `initialize_character_data`: Copies character data between memory regions using indices from a source buffer, with multiple iterations and address calculations for different data structures.
- `copy_character_data`: Reads bytes from a source buffer, uses them as indices to copy character data between two memory regions, and then increments the destination pointer and continues processing.
- `convert_number_to_string`: Converts a signed long integer into a string representation using base-10 digits, storing the result in a global buffer and handling negative numbers by taking their absolute value before conversion.
- `initialize_graphics_mode`: Sets up graphics mode registers with specific values for display and blitter configuration, including setting up various hardware registers related to video timing and memory layout.
- `setup_video_parameters`: Configures video parameters for rendering operations by initializing multiple hardware registers with predefined values for display and graphics processing.
- `update_gardener_state_2`: Updates the state of a gardener object by checking collisions with other objects, modifying object coordinates, and managing object lifecycle flags.
- `update_tile_coordinates`: Updates tile coordinate values based on comparisons between parameters, modifying coordinates when differences are detected and handling boundary conditions.
- `frag_subq_bpl`: Short code fragment that subtracts 1 from a fixed memory address, then branches if the last result was not negative.
- `update_coordinates`: Updates coordinate values in a structure based on comparisons between two sets of data, modifying the coordinates when differences are detected and incrementing counters accordingly.
- `setup_audio_and_dma`: Initializes audio volume registers and DMA configuration, clears multiple hardware registers, and sets up joystick input handling for game controls.
- `frag_tst_beq_clr_move_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then clears a fixed memory address (5 times in a row), then copies 15 to a fixed memory address, then returns.
- `increment_and_compare`: Increments a global counter and compares it with another global value, clearing the counter and branching based on the comparison result while updating game state variables.
- `update_audio_registers_2`: Updates audio period registers based on game object state and table lookups, with conditional logic affecting sound timing and pitch.
- `update_game_object`: Processes game object coordinates and state changes based on input data, updating position and behavior attributes.
- `process_coordinate_update`: Reads a nibble from a byte in a structure, uses it as an index into a global table, and updates coordinate values based on table lookups and arithmetic operations.
- `store_coordinate_value`: Copies a word value from a structure to another location in memory and returns immediately without performing any other operations.
- `update_coordinate_value_2`: Calculates and updates coordinate values by performing table lookups with multiplication and comparison operations, handling both addition and subtraction of movement values based on conditional checks.
- `update_coordinate_value`: Updates a coordinate value based on movement calculations and table lookups, handling both addition and subtraction of values with conditional checks and resets.
- `calculate_movement_values`: Processes movement calculations using bit operations and table lookups, updating coordinate values and handling conditional arithmetic based on input parameters.
- `process_input_byte_2`: Processes a byte from the input buffer based on its lower nibble, performing different operations including coordinate updates, bit manipulations, and table lookups before returning.
- `store_low_nibble`: Extracts the lower nibble from a byte value and stores it in a global variable, then returns immediately without further processing.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
- `frag_movem_lea_move_addq_cmp_bne_rts`: Short code fragment that saves registers on the stack, then loads the address of a fixed memory address into a0, then copies the memory at a0 to d0, then adds 4 to d0, then copies the memory at a0 to d1, then compares d1 with d0, then branches if the last result was not zero, then restores saved registers from the stack, then returns.

### Gardener (1995)(Intersoft)(De)[h Smurf].adf:Type

- `parse_type_command_arguments_2`: Parses command-line arguments to identify hex or number flags, validates that they are not used simultaneously, and handles error reporting for invalid options or file access failures.
- `format_output_string_2`: Formats strings and numbers into a character buffer using specific text patterns and hexadecimal characters.

### Gardener (1995)(Intersoft)(De)[h Smurf].adf:SMURF

- `initialize_system`: Initializes system resources by setting up memory management structures, calling initialization routines, and configuring system pointers for further operations.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `process_tile_data`: Processes tile data by calculating memory offsets and updating values, with bitwise operations on word-sized data.
- `perform_bit_operations`: Executes bitwise logical operations and shifts on word-sized data, with conditional branches that modify register values and perform loop iterations.
- `copy_and_process_data`: Copies bytes from a source buffer to a destination in reverse order, then processes tile data by calculating memory offsets and updating values at those locations.
- `update_tile_data`: Processes tile data by reading bytes from a buffer, performing bit shifts and additions to calculate offsets, then updating values at calculated memory addresses.
- `frag_movea_addq_adda_dbra_rts`: Short code fragment that copies a4 to a1, then adds 1 to d0, then copies the memory at a1 to a1, then adds a1 to a1 (2 times in a row), then loops back while counting down d0, then adds 4 to a1, then returns.

### Gardener (1995)(Intersoft)(De)[h Smurf].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `sets_global_pointer_returns_minus_one`: Stores a constant into a global variable, removes 4 bytes from the stack, restores registers from memory, and returns the value -1 in d0.
- `initialize_graphics_and_interrupts`: Sets up graphics registers including DMA control, interrupt masks, and copper list pointers.
- `process_frame_data`: Processes frame data by updating counters, reading from memory locations, performing arithmetic operations, and managing state transitions based on comparison results.
- `handle_input_processing`: Processes input data by comparing values against a lookup table, calculating offsets, and writing character data to specific memory locations using blitter parameters.
- `configure_blitter_and_draw_lines`: Configures blitter registers with modulo values and mask words, then draws lines using the blitter by computing step counts and octant information, and finally waits for the blitter to become idle before setting up a second blitter operation.
- `reset_blitter_masks_and_modulo`: Waits for the blitter to become idle, then writes full first and last mask words, a control word, and two modulo values to configure the blitter for subsequent operations.
- `build_row_offset_table`: Fills 256 consecutive words with offsets that increase by forty for each row, forming a lookup table of row start positions.
- `draw_line_with_blitter`: Orders line endpoints, computes step counts and octant information, toggles a pixel bit at the starting position, and programs the blitter with modulo, control, and size values to draw the line.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_audio_and_input_state`: Sets up audio volume registers and initializes a table of pointers and values.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `manage_audio_processing`: Manages audio processing by incrementing a counter, comparing it with a threshold, clearing the counter when exceeded, and calling audio update routines based on conditional checks.
- `update_audio_channels`: Updates audio channel registers based on input data and global state, handling different audio processing modes through conditional operations.
- `process_input_byte_5`: Processes an input byte by reading a value from a table, updating a global variable with that value, and conditionally copying another value.
- `process_input_byte_3_2`: Processes a byte from the input buffer based on its lower nibble, performing different operations such as storing values, modifying global variables, and updating a counter.
- `process_input_byte_4`: Processes an input byte by reading a value from a table, updating a global variable with that value, and conditionally copying another value to a structure field.
- `update_value_range`: Processes a value from a global structure based on conditional logic involving bit operations and table lookups, then stores the result back.
- `adjust_value_based_on_conditions`: Processes a value from a global structure based on conditional logic involving bit operations and table lookups, then stores the result back into the same structure.
- `process_input_byte_3`: Processes a byte from the input buffer based on its lower nibble, performing different operations such as storing values, modifying global variables, or updating structure fields depending on the nibble value.
- `store_nibble_value`: Extracts the lower nibble from a byte value and stores it at a specific offset in a global structure.
- `advance_sample_pointer_on_wrap`: Adds a table value to a counter when the high nibble of a field is non-zero.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (bchg.b) and then falls through into the code that follows, with no return of its own.

### Gardener (1995)(Intersoft)(De)[h Smurf].adf:SMURF#unpacked

- `initialize_video_system`: Sets up video hardware registers including DMA control, interrupt masks, and copper list pointers.
- `process_frame_data_v2`: Processes frame data by updating counters, reading from memory locations, performing arithmetic operations, and managing state transitions based on comparison results, similar to routine R6 but with different addresses.
- `copy_character_data_2`: Copies character data from a source buffer to a target location in memory, updating counters and handling boundary conditions based on comparison results.
- `initialize_blitter_pattern`: Configures blitter registers for pattern drawing using modulo values and mask words, then performs multiple blitter operations with line drawing routines.
- `wait_for_blitter_idle`: Waits until the blitter is idle, then writes full first and last mask words, a control word and two modulo values for the blitter.
- `build_row_offset_table_2`: Fills 256 consecutive words with offsets that increase by forty for each row, forming a lookup table of row start positions.
- `draw_line_with_blitter_2`: Orders two endpoints, computes step counts and octant of a line, toggles one pixel bit at the start position, and programs the blitter with modulo, control and size values to draw the line.
- `initialize_audio_and_input_state_2`: Sets up audio volume registers and initializes a table of pointers and values.
- `mt_end_2`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `frag_addq_move_cmp_bcs_clr_tst_beq_bsr`: Short code fragment that adds 1 to a fixed memory address, then copies a location in the code to d0, then compares d0 with a location in the code, then branches if the carry flag is set, then clears a fixed memory address, then tests a fixed memory address, then branches if the last result was zero, then calls another routine, then jumps to another point in the code.
- `frag_bsr_bra`: Short code fragment that calls another routine, then jumps to another point in the code.
- `update_audio_registers`: Updates audio period registers based on data from a structure, with conditional logic affecting different audio channels and handling counter decrements.
- `update_audio_channels_2`: Initializes audio DMA channels by setting up channel pointers and parameters, then updates audio hardware registers with channel data from memory locations.
- `update_sprite_state`: Updates sprite position and state data based on input nibbles, then processes input handling routines and updates global variables according.
- `update_sprite_data`: Updates sprite position and state data based on input nibbles, then processes input handling routines and updates global variables accordingly.
- `frag_bsr_bra_2`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_3`: Short code fragment that calls another routine, then jumps to another point in the code.
- `update_sprite_position`: Reads a nibble from a structure, uses it to index a table, and updates sprite position data.
- `update_audio_hardware_and_channels`: Sets up DMA channels and audio registers, then updates channel parameters and manages a counter based on a global table.
- `process_input_value`: Reads a byte value from a specific offset in a structure, masks it with a nibble, and performs conditional operations based on the result.
- `frag_move_rts`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a5, then returns.
- `normalize_position`: Loads a byte from a global location into the data register, divides it by three, swaps the word result, and compares it with zero and two.
- `frag_moveq_move_andi_bra`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_move_bra`: Short code fragment that copies the memory at an offset from a6 to d2, then jumps to another point in the code.
- `lookup_and_compare_values`: Performs indexed lookup in a table, compares a value with the result, and stores the indexed value if the comparison succeeds, otherwise returns immediately.
- `frag_move_rts_2`: Short code fragment that copies d2 to the memory at an offset from a5, then returns.
- `adjust_value_down`: Tests a global variable and branches if it's not zero.
- `update_position_and_status`: Updates a position value in a structure, checks if it exceeds a threshold, and adjusts the value accordingly.
- `update_coordinate_value_3`: Reads a word value from a table at a calculated offset, compares it with a masked value from a structure, and updates a coordinate value based on the comparison result.
- `frag_clr_rts`: Short code fragment that clears the memory at an offset from a6, then returns.
- `frag_move_beq_clr`: Short code fragment that copies the memory at an offset from a6 to d0, then branches if the last result was zero, then copies d0 to the memory at an offset from a6, then clears the memory at an offset from a6.
- `adjust_movement_speed`: Adjusts a movement speed value by adding or subtracting a byte value from a structure, clamps it to bounds, and stores the result back into the structure.
- `calculate_index_and_store`: Subtracts a value from a word, performs a loop to find a value in a table based on a calculated index, and stores the result into a structure.
- `combine_bits_and_scale`: Processes byte values from specific offsets in a6 to combine bits using bitwise operations.
- `calculate_scaled_value`: Calculates a scaled value based on input parameters, applies conditional logic for different modes, and updates multiple fields in a structure with the computed result.
- `frag_tst_bpl_move_sub_bra`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was not negative, then copies 255 to d2, then subtracts d0 from d2, then jumps to another point in the code.
- `frag_move_bra_2`: Short code fragment that copies d0 to d2, then jumps to another point in the code.
- `frag_move_andi_mulu_lsr_tst_bmi_add_bra`: Short code fragment that copies an indexed entry from a4 to d2, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then multiplies d2 by d0 (unsigned), then shifts d2 right by 7, then copies the memory at an offset from a6 to d0, then tests the memory at an offset from a6, then branches if the last result was negative, then adds d2 to d0, then jumps to another point in the code.
- `subtract_and_advance_phase`: Subtracts d2 from d0 and stores the result in offset 6 of a structure in a5, then advances a phase byte in the structure by an amount taken from the high nibble of a neighbouring byte.
- `frag_bsr_bra_4`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_5`: Short code fragment that calls another routine, then jumps to another point in the code.
- `process_and_limit_value`: Processes byte values from specific offsets in a6 to combine bits using bitwise operations.
- `frag_tst_bpl_move_sub_bra_2`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was not negative, then copies 255 to d2, then subtracts d0 from d2, then jumps to another point in the code.
- `frag_move_bra_3`: Short code fragment that copies d0 to d2, then jumps to another point in the code.
- `add_scaled_difference`: Reads a byte from memory, scales it by another value, adds the scaled result to a third value, and stores the final result in memory.
- `clamp_and_update_value`: Calculates the difference between two values, clamps the result between zero and sixty-four, stores it in a structure, and updates a byte by merging a shifted nibble from a neighboring byte.
- `update_counter_and_position`: Updates a counter based on input values, performs conditional operations, and modifies position data using bit shifting and arithmetic operations.
- `frag_move_rts_3`: Short code fragment that copies 1 to the memory at an offset from a6, then returns.
- `clamp_and_update_byte`: Reads a byte from memory, shifts it right by four bits, adds the result to another byte, clamps the sum to a maximum value, and stores the final result.
- `decrement_and_clamp_byte`: Subtracts the low nibble of the byte at offset 3 of the structure in a6 from the byte at offset 19, clears that byte to zero when the result is negative, and stores the resulting byte as a word at offset 8 of the structure in a5.
- `decrement_and_store_byte`: Reads a byte from offset 3 of the structure in a6, subtracts one from it, and stores the result in a global byte, then clears two subsequent global bytes.
- `clamp_and_store_byte`: Reads a byte from offset 3 of a structure in a6, clamps its value to a maximum of sixty-four, and stores the result in both a byte at offset 19 and a word at offset 8 of a structure in a5.
- `update_and_store_value`: Reads a byte from memory, performs bit manipulation and arithmetic operations, compares the result with a threshold, and stores the result in global memory if it meets the condition.
- `frag_move_beq_clr_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then branches if the last result was zero, then clears a fixed memory address, then copies d0 to a fixed memory address, then returns.
- `process_input_nibble`: Reads a nibble from a byte at offset 2 of a structure, compares it against several constants, and branches to different handling routines based on the value.
- `process_input_byte`: Reads a byte from offset 3 of a structure, extracts its upper nibble, and uses it as an index to jump to different handling routines.
- `update_global_from_ciaa_pra`: Masks bit zero of the byte at offset 3 of the structure in a6, shifts it left by one, and ORs the result into a global variable after clearing bit one of that variable.
- `mask_and_merge_byte`: Masks the low nibble of a byte at offset 31 from a pointer in a6 and then merges the low nibble of a byte at offset 3 from the same pointer into it.
- `clear_low_nibbles_of_byte`: Clears the low nibble of a byte at offset 30 from an address in a6 and then ORs in the low nibble of another byte from the same base.
- `store_masked_byte`: The routine reads a byte from an offset in a structure pointed to by a register, masks it with a constant, and stores the result into a different offset in the same structure.
- `decrement_and_store`: Tests a global variable and branches if it's not zero.
- `frag_move_bra_4`: Short code fragment that copies d0 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_move_lsr_rts`: Short code fragment that copies a location in the code to d0, then shifts d0 right by 4, then copies d0 to the memory at an offset from a6, then returns.
- `mask_and_merge_nibble`: Masks the low nibble of a byte at offset 30 from a pointer in a6 and merges the high nibble from a byte at offset 3 of the same pointer, storing the result back at offset 30.
- `configure_dma_and_copy_data`: Checks if a value at offset 3 of the structure in a6 masked with 15 is non-zero and if a 16-bit value at offset 0 is divisible by that value; if so, it performs two loops of 280 iterations, writes a value from offset 20 of the structure to the DMA controller, and copies data between pointers provided in the structure.
- `frag_tst_bne_moveq_move_andi_bra`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_tst_bne_moveq_move_andi_bra_2`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_moveq_move_andi_cmp_bne_clr_rts`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then compares d0 with a location in the code, then branches if the last result was not zero, then clears the memory at an offset from a6, then copies 0 to the memory at an offset from a5, then returns.
- `frag_moveq_move_andi_cmp_bne_beq_bra`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then compares d0 with a fixed memory address, then branches if the last result was not zero, then copies the memory at a6 to d0, then branches if the last result was zero, then copies d1 to the stack, then jumps to another point in the code.
- `frag_tst_bne_moveq_move_andi_addq_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then tests a fixed memory address, then branches if the last result was not zero, then adds 1 to d0, then copies d0 to a fixed memory address, then returns.
- `frag_tst_bne_move_andi_lsl_or_beq`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then shifts d0 left by 4, then masks the memory at an offset from a6 with 15, then merges d0 into the memory at an offset from a6, then tests d0, then branches if the last result was zero.
- `advance_sample_pointer_on_wrap_2`: Adds a table value to a counter when the high nibble of a field is non-zero.
