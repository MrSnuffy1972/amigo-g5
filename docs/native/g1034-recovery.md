# Game 1034 (stardust (demo) (1993)(bloodhouse)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01034). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 9 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 107 raw-word lines |
| Functions named and described | pass, 119 of 119 named (0 author symbols), 119 of 119 described (9 by rule, 109 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 42 of 42 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 418456 bytes, 74 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8251 of 8309 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 441907 |
| code | 2944 |
| data | 442532 |
| padding | 1449 |
| unused | 12288 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Stardust (demo) (1993)(Bloodhouse).adf:Echo | 2 | 241 | 10 |
| Stardust (demo) (1993)(Bloodhouse).adf:PAL | 1 | 45 | 1 |
| Stardust (demo) (1993)(Bloodhouse).adf:SDPREV | 3 | 328 | 16 |
| Stardust (demo) (1993)(Bloodhouse).adf:TUNNELPREV | 4 | 328 | 17 |
| Stardust (demo) (1993)(Bloodhouse).adf:bootblock | 1 | 18 | 1 |
| Stardust (demo) (1993)(Bloodhouse).adf:SDPREV#unpacked | 1 | 7349 | 74 |

## Cases handled

- stardust (demo) (1993)(bloodhouse)/Stardust (demo) (1993)(Bloodhouse).adf:SDPREV: crunched executable unpacked with ancient scan (CrM2: Crunch-Mania LZH-mode, 255230 -> 418456 bytes), 68 of 184 windows confirmed in reference RAM, loaded at an address: recovered as u5
- stardust (demo) (1993)(bloodhouse)/Stardust (demo) (1993)(Bloodhouse).adf:TUNNELPREV: CrM2: Crunch-Mania LZH-mode stream found by ancient scan (250588 -> 476136 bytes) but only 1 of 184 sampled windows are in reference RAM: not used
- stardust (demo) (1993)(bloodhouse)/Stardust (demo) (1993)(Bloodhouse).adf:SDPREV: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- stardust (demo) (1993)(bloodhouse)/Stardust (demo) (1993)(Bloodhouse).adf:TUNNELPREV: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_system_environment` (SDPREV): Calls several library routines to check a structure, sets a global pointer to a specific routine, clears four hardware registers with the value 32767, and either jumps to a routine or calls two library functions before returning. Kind: disk hardware/integrity check. Pass path: The branch following the call that continues normal system initialization/loading..
- `setup_and_jump_to_handler` (TUNNELPREV): Performs library calls to inspect a structure, stores a specific routine address into a global variable, writes 32767 to four hardware registers, and jumps to a routine or calls two library functions. Kind: disk hardware/track validation routine. Pass path: The branch taken when the hardware check (ADKCON/disk controller interaction) succeeds, typically leading to the main game/application data loading..

## Functions


### Stardust (demo) (1993)(Bloodhouse).adf:Echo

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of d1 to d0 and then branches to another location.
- `update_animation_data_2`: Updates animation-related data including frame counts and timing values based on memory comparisons and conditional checks, then performs additional processing.
- `update_animation_data`: Updates animation-related data including frame counts and timing values based on memory comparisons and conditional checks, then performs additional processing based on the computed results.
- `copy_memory_block_3`: Copies a 32-bit value from one memory location to another and then branches to a specific address without returning.
- `memory_comparison_loop`: Sets up a memory value, performs multiple conditional checks on memory locations, and updates another memory location based on comparison results.
- `copy_memory_and_branch`: Copies a longword value from one memory location to another and then unconditionally branches to a fixed address.
- `update_memory_and_call`: Copies data between memory locations, performs comparisons, loads values from memory, and calls another routine based on conditional checks.
- `memory_access_check`: Loads a value from memory, performs bit shifting, checks a condition based on that value, and calls another routine if the condition is met.
- `initialize_dma_channels`: Initializes DMA channels by setting up multiple register values and calling a routine based on conditional checks, then jumps to another routine.

### Stardust (demo) (1993)(Bloodhouse).adf:PAL

- `check_ecs_hardware`: Opens the dos and graphics libraries and checks the system chip configuration.

### Stardust (demo) (1993)(Bloodhouse).adf:SDPREV

- `initialize_system_environment`: Calls several library routines to check a structure, sets a global pointer to a specific routine, clears four hardware registers with the value 32767, and either jumps to a routine or calls two library functions before returning.
- `copy_memory_block`: Compares two addresses and, if the first is not greater than the second, copies a byte from the first to the second and branches based on a comparison with a constant.
- `copy_and_trap`: Copies a byte from a memory location to a destination address and calls a trap if the source address is not zero.
- `initialize_system_tables`: Sets the processor status register, calculates several memory addresses, calls routines to populate tables, and finally writes a value to a hardware register before performing an indirect jump.
- `apply_amiga_hunk_relocations_2`: Iterates through a data structure to copy memory blocks and perform relocations by adding a base address to specific offsets if the hunk type matches AmigaDOS relocation codes.
- `process_amiga_hunk_data`: Iterates through a data structure to copy memory blocks and perform relocations by adding values to specific offsets if the hunk type matches certain AmigaDOS constants.
- `process_hunk_data`: Iterates through a data structure, copying blocks of memory and performing additions to specific offsets if a value matches a specific constant, then continues based on a loop counter in d2.
- `initialize_and_populate_data_tables`: Clears a sixteen-word table at an offset from a pointer in a6, then calls several routines to populate various tables.
- `decrement_counter_and_shift_register`: Decrements a 16-bit value in a register, and if the result is not zero, performs a logical shift right on another register before branching to another routine; otherwise, it branches to a different location.
- `fetch_interpolated_value`: Performs bitwise manipulations on d6 and d0, then enters a loop that compares a value in d1 against a value from a table to return a word from a table at an offset determined by a calculation.
- `process_bit_rotation`: Performs a loop that rotates d6 and d7 based on d1, pulls a word from a table pointed to by a0, and returns a bitwise AND of that word with d0.
- `fill_tables_with_processed_data`: Calls another routine to process data, then uses two loops to store the resulting words into two different destination tables, updating a pointer and a counter in each loop.
- `fill_accumulated_code_tables`: Sets the first table entry to zero, then runs fifteen steps that accumulate counts from a source array into running sums.
- `initialize_dma_tables`: Populates a series of memory locations with incrementing values derived from d0, then writes a pointer to a table into a hardware register and sets specific values in other hardware registers.
- `trigger_system_trap`: Loads a constant into a register and calls a trap.
- `transfer_handler_data`: Checks a bit in a global variable, clears the status register, copies a longword from a routine address to a destination, and performs an indirect jump if a comparison succeeds.

### Stardust (demo) (1993)(Bloodhouse).adf:TUNNELPREV

- `setup_and_jump_to_handler`: Performs library calls to inspect a structure, stores a specific routine address into a global variable, writes 32767 to four hardware registers, and jumps to a routine or calls two library functions.
- `copy_memory_block_2`: Copies bytes from a source address in a global variable to a destination address in another global variable until the source address reaches a specific limit, then returns.
- `copy_byte_and_trap`: Copies a byte from a memory location to a destination address calculated by adding an offset to a value from a PC-relative table, then calls a system trap after writing a value to a global variable.
- `setup_memory_tables`: Sets the processor status register, calculates memory addresses, calls routines to clear and populate several tables, and performs an indirect jump.
- `apply_amiga_hunk_relocations_3`: Iterates through a list of memory hunks to copy data blocks and apply additive relocations to specific memory locations based on hunk type codes.
- `process_amiga_hunk_relocations`: Iterates through a list of memory hunks, copying data blocks and applying additive relocations to specific memory locations based on hunk type codes.
- `apply_amiga_hunk_relocations`: Iterates through a list of data structures, copying blocks of memory to a destination and performing additive relocations if a specific hunk type is encountered.
- `populate_data_structures`: Calculates several memory addresses from a source structure, clears a 16-word table, and performs multiple calls to populate various tables and decode symbol offsets using loops.
- `decrement_and_shift_loop_counter`: Clears register d1, decrements a 16-bit counter in d7, and if the counter is not zero, shifts register d6 left by one bit and branches back to the start of the routine; otherwise it returns.
- `search_bitstream_table`: Performs a bit-by-bit search through a stream of data pointed to by a register, using a loop that runs up to sixteen times.
- `fetch_table_entry_with_bitmask`: Calculates an index using bitwise shifts and arithmetic on registers, then performs a bitwise AND between a value from a memory table and a word from a pointer in a0 before returning.
- `populate_dual_tables_from_lookup`: Calls a lookup routine to fetch values into two separate destination tables, using a calculated sum of the first set of fetched values to determine the iteration count for the second set.
- `accumulate_symbol_decoder_offsets`: Clears a 16-bit value at the start of a destination table and then executes a loop fifteen times that reads values from a source array to calculate and store running sums into two separate tables at the current destination pointer.
- `configure_dma_registers`: Stores the high and low words of d0 into two consecutive memory locations, then writes a pointer to a table into a hardware register, clears another hardware register, and sets a third hardware register to a constant value.
- `clear_memory_block`: Clears sixteen consecutive 16-bit words in a memory block starting at a location derived from the program counter using a loop of clear and increment instructions.
- `trigger_system_trap_2`: Loads the address of a local constant into a register, moves it to a specific memory location, and executes a system trap.
- `copy_single_word_and_jump`: Checks a bit in a global structure, disables interrupts, and copies a single longword from a source address to a destination address before jumping to a location stored at that destination address.

### Stardust (demo) (1993)(Bloodhouse).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Stardust (demo) (1993)(Bloodhouse).adf:SDPREV#unpacked

- `setup_and_loop`: Initializes memory patterns, fills data structures with calculated values, and enters an infinite loop that calls multiple routines and handles interrupt processing.
- `fill_memory_pattern`: Fills memory with a repeating pattern of values, sets up DMA control registers, and returns after initializing various data structures.
- `initialize_graphics_and_audio_2`: Saves registers, reads hardware status from custom registers, copies data between memory locations, sets up DMA and interrupt control registers, then restores registers and returns.
- `initialize_graphics_and_audio`: Initializes graphics bitplane operations by setting up hardware registers for multiple blitter commands, clears a memory table, and configures audio registers for sound output.
- `clear_and_setup_registers`: Clears several memory locations, sets up register values for graphics or sound hardware, and performs a loop to initialize arrays of data.
- `initialize_audio_and_graphics`: Sets up audio registers for sound playback, configures DMA channels, and calls multiple routines to handle graphics rendering and animation state updates.
- `set_copper_instruction`: Checks a bit in a memory location and sets a copper instruction address accordingly, returning immediately if the bit is not set.
- `configure_blitter_operations`: Configures multiple blitter registers for graphics operations by calculating addresses and setting up data transfer parameters based on input values.
- `update_animation_state`: Updates animation state based on a global counter, modifies object properties in a data structure, and calls another routine with updated values.
- `update_animation_state_2`: Updates animation state based on a global counter, modifies position values in a data structure, and calls another routine with updated coordinates.
- `update_starfield_bounds`: Processes starfield boundary checks using bit testing operations and updates multiple global variables based on conditional logic.
- `starfield_processing`: Processes starfield data with boundary checks, performs bit testing operations, and calls multiple routines for rendering and data updates.
- `lookup_table_access`: Performs bit manipulation and conditional shifts on two input values to calculate an index into a lookup table, then returns a byte from that table.
- `initialize_effect_data`: Initializes multiple effect data structures with specific parameters including position, color, and timing values, then returns.
- `update_starfield`: Updates starfield coordinates and renders star sprites by modifying position data in memory tables, with different behavior based on a global variable's state, and increments a counter stored at a specific memory location.
- `initialize_render_data`: Initializes rendering parameters and processes a data structure, copying specific fields from a source table to global variables while checking a status flag.
- `render_data_initialization`: Initializes rendering data by checking flags, calling subroutines, and processing multiple data structures with conditional logic.
- `compute_vertex_coordinates`: Calculates vertex positions and related values based on input parameters, storing computed coordinates and flags in global memory locations.
- `process_starfield_data`: Checks multiple bit flags in global variables and processes coordinate data for starfield rendering when certain conditions are met, calling another routine to update graphical attributes.
- `update_starfield_2`: Processes starfield rendering parameters based on input coordinates and updates graphical attributes for display.
- `manage_game_state`: Initializes multiple effect data structures with specific parameters including position, color, and timing values, then returns after performing various conditional checks on global variables.
- `process_object_data_2`: Reads object parameters from a data structure and writes transformed values to a global buffer, including coordinates, dimensions, and flags.
- `toggle_rendering_state`: Toggles a bit in a global variable and conditionally updates multiple global values based on the state, setting flags and performing arithmetic operations.
- `update_entity_list`: Updates a list of entities by decrementing a counter, advancing pointers through a table, and storing entity data from registers into allocated slots.
- `update_starfield_coordinates`: Performs BCD arithmetic operations on coordinate values and updates starfield data structures based on calculated differences, then jumps to a rendering routine.
- `set_display_mode`: Configures the display mode by setting a hardware register to a specific value, calls a screen update routine, and increments a global counter.
- `process_object_data`: Reads object parameters from a data structure and writes transformed values to a global buffer, including coordinates, dimensions, and flags.
- `load_sprite_data`: Processes sprite configuration data from a buffer, updating global variables with position, size, and animation parameters while handling data compression.
- `process_sprite_data`: Reads sprite configuration data from a buffer, updates global variables with sprite properties such as position and state, and modifies sprite behavior based on conditional checks.
- `blit_pattern_lines`: Performs multiple bit-block transfers using pattern data from a table, with conditional logic based on a global variable to determine transfer parameters and offsets.
- `process_vector_data`: Processes signed 32-bit values from memory locations, applies bit shifting and multiplication with a fixed value, handles negative numbers by negating the result, then stores the computed values to specific memory locations.
- `initialize_data_structures`: Clears two word-sized memory locations, loads pointers to data tables, and performs a loop with conditional branching based on byte values at specific offsets.
- `update_object_position`: Reads position coordinates from a structure and adjusts them based on boundary checks, updating the structure with the new values.
- `compare_and_copy_data`: Clears a word-sized counter at a fixed memory location, increments another counter, loads a pointer from memory, compares the value at that pointer with a specific 32-bit constant, and if they match, performs a series of 8 memory copy operations from one buffer to another.
- `process_coordinate_data`: Loads pointers to data tables, performs a loop with conditional branching based on byte values at specific offsets, and uses a jump table for indirect execution.
- `update_player_coordinates`: Adjusts player position coordinates based on boundary checks and updates related data structures, including joystick input handling and object-related operations.
- `filter_and_store_pointers`: Initializes a pointer and loop counters, reads data from a fixed memory location, checks conditions on byte values at specific offsets, and if conditions are met, stores the pointer to an array.
- `update_transformations_and_counters`: Performs bitwise OR operations on a register with values from program data, sets a bit in memory, increments counters at specific memory locations, and applies bitmask operations to a counter.
- `update_counter_and_mask`: Increments a byte value at a specific memory location, applies a bitmask to that byte, and performs conditional operations based on the result.
- `increment_and_mask`: Clears a word register, increments a byte value at a specific memory location, applies a bitmask to that byte, and then branches to a fixed address.
- `process_object_collision`: Calculates object positions and performs collision detection by comparing coordinates, updates counters and masks based on comparison results, and branches to different routines depending on conditions.
- `update_counter_and_mask_2`: Increments a byte value at memory location 32(a3) by 8 and branches to 32702 if the result is greater than or equal to 100.
- `update_object_transformations`: Calculates object transformations by multiplying values from a lookup table with a byte value, performs arithmetic operations on the results, and updates global variables with transformed coordinates.
- `calculate_object_coordinates`: Calculates object coordinates by adding values from multiple memory locations, performs arithmetic operations on word-sized values, and updates global variables with calculated results.
- `bcd_addition_loop`: Performs four BCD addition operations on word values stored in memory locations, updating them with corresponding values from another memory location.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `update_position_bounds`: Adjusts position coordinates based on boundary checks and updates related data structures.
- `update_player_position`: Adjusts player position coordinates based on boundary checks and updates related data structures, including joystick input handling and object rendering parameters.
- `update_starfield_objects`: Manages starfield object creation and updates by checking game state conditions, calculating object positions using multiplication and bit shifts, and storing the results in memory.
- `manage_starfield_objects`: Manages starfield object creation and updates by checking game state conditions, calculating object positions using multiplication and bit shifts, and storing transformed data in memory.
- `bcd_adjust_loop`: Performs four binary-coded decimal adjustments on data pointed to by two address registers, then increments a global variable by one.
- `bcd_adjust_multiple`: Executes multiple binary-coded decimal operations with different memory source addresses and increments a global counter by three.
- `bcd_starfield_update`: Performs four BCD addition operations on word values stored in memory locations, updating them with corresponding values from another memory location.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_4`: Entry point part-way through straight-line code: runs three instructions (ori.b, ori.b, ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_5`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_6`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_7`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_8`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `initialize_starfield`: Initializes starfield coordinate values and clears multiple global variables based on conditional flags.
- `coordinate_calculation`: Calculates coordinate values based on input parameters, stores results in memory locations, and returns a combined value.
- `memory_addressing_operations`: Performs memory addressing operations with conditional checks, calculates values using shift and add operations, and stores results in multiple memory locations.
- `calculate_starfield_coordinates`: Performs coordinate calculations based on input parameters, stores results in memory locations, clears certain global variables, and sets up values for starfield rendering.
- `coordinate_calculation_v2`: Calculates coordinate values based on input parameters, stores results in memory locations, and returns a combined value with different calculation logic.
- `memory_addressing_operations_v2`: Performs memory addressing operations with conditional checks, calculates values using shift and add operations, and stores results in multiple memory locations with additional offset calculations.
- `memory_addressing_operations_v3`: Performs memory addressing operations with conditional checks, calculates values using shift and add operations, and stores results in multiple memory locations with additional offset calculations and data initialization.
- `update_starfield_data`: Processes starfield coordinate data by reading from two memory locations, performing arithmetic operations on word values, and updating memory with computed results.
- `decrement_d3_and_branch`: Decrements a word register and then branches to a fixed address.
- `increment_d3_and_branch`: Increments a word register and then branches to a fixed address.
- `decrement_and_branch`: Decrements a 16-bit value in data register d4 and unconditionally branches to another routine, likely part of a loop structure.
- `increment_and_branch`: Increments a 16-bit value in a data register and unconditionally branches to another routine.
- `decrement_and_branch_2`: Decrements a 16-bit value in register d5 and unconditionally branches to an earlier address in the routine.
- `increment_d5`: Increments a 16-bit value in data register d5 by one and unconditionally branches to another routine.
