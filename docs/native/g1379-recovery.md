# Game 1379 (super cars ii (1991)(gremlin)[cr hyperion]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01379). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1849072 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 91 raw-word lines |
| Functions named and described | pass, 378 of 378 named (0 author symbols), 378 of 378 described (198 by rule, 167 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 5549 of 5549 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9027 of 9027 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 8282 |
| code | 34802 |
| data | 1805948 |
| padding | 40 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Super Cars II (1991)(Gremlin)[cr Hyperion](Disk 1 of 2).adf:bootblock | 1 | 155 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8872 | 370 |

## Cases handled

- super cars ii (1991)(gremlin)[cr hyperion](disk 1 of 2)/Super Cars II (1991)(Gremlin)[cr Hyperion](Disk 1 of 2).adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- super cars ii (1991)(gremlin)[cr hyperion](disk 2 of 2)/Super Cars II (1991)(Gremlin)[cr Hyperion](Disk 2 of 2).adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 5549 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 branch displacement
- 2 packed track region(s), 1800192 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `empty_stub` (bootblock): Empty routine: returns to the caller immediately without doing anything. Kind: disk hardware check. Pass path: DSKPTH instruction succeeds and continues execution.
- `hardware_initialize` (runtime segment an address): Initializes hardware registers by setting specific DMA control and interrupt enable bits, then copies data from a source table to various memory locations. Kind: disk hardware check. Pass path: DSKLEN and ADKCON registers are properly initialized and sync pattern matches expected values.

## Functions


### Super Cars II (1991)(Gremlin)[cr Hyperion](Disk 1 of 2).adf:bootblock

- `trap_then_spin_forever`: Executes a trap instruction and then loops forever, so the boot block never returns control.
- `load_nu_text_and_call_stub`: It loads the address of a text string containing the repeated characters 'NuNuNuNu' and then calls an empty stub that returns to the caller immediately without doing anything.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `modify_byte_flags`: It reads a byte from a memory location, sets its high bits, clears a subset of them, and sets another group of bits in sequence.
- `frag_and_add_or_rts`: Short code fragment that masks d4 with d7, then masks d5 with d7, then adds d4 to d4, then merges d5 into d4, then returns.
- `update_speed_counter`: Updates a speed counter based on input values and performs bit operations on a global variable, with conditional branching and arithmetic operations.
- `fall_through_entry`: Entry point part-way through straight-line code: runs three instructions (moveq, bclr.b, bset.b) and then falls through into the code that follows, with no return of its own.
- `frag_move_btst_bne_rts`: Short code fragment that copies 136 to the memory at an offset from a5, then copies 19 to the memory at an offset from a5, then copies 25 to the memory at an offset from a5, then tests bit 0 of the memory at an offset from a5, then branches if the last result was not zero, then tests bit 5 of the memory at an offset from a5, then branches if the last result was not zero, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_tst_beq_jsr_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then calls another routine, then branches if the last result was zero, then returns.
- `validate_and_process_header`: Compares a register value with a constant and branches if they are equal, otherwise stores data to a global variable, performs bit shifting on another register, reads from a table using the shifted value, and calls a routine to process the result.
- `frag_move_asl_lea_movea_jsr_rts`: Short code fragment that copies d1 to a fixed memory address, then copies d0 to the stack, then shifts d0 left by 3, then loads the address of a fixed memory address into a0, then copies an indexed entry from a0 to a fixed memory address, then copies the top of the stack to d0, then copies a fixed memory address to a0, then calls another routine, then loads the address of a fixed memory address into a6, then returns.
- `setup_hardware_and_loop`: Clears multiple global variables, initializes hardware registers including interrupt and DMA control, sets up interrupt vectors, then enters an infinite loop.
- `toggle_flag_bits`: Toggles specific bits in a global variable based on comparison results and performs bit testing operations on other global variables.
- `increment_and_mask_values`: Increments values at multiple memory locations with a constant and applies bitwise AND operations with a fixed mask value.
- `calculate_vertex_coordinates`: Calculates vertex coordinates for rendering by processing data from tables, performing arithmetic operations, and storing results in memory.
- `validate_game_state`: Validates game state by comparing values against thresholds and checking bit conditions in global variables, returning specific values based on these checks.
- `frag_addq_move_rte`: Short code fragment that adds 1 to a fixed memory address, then copies 1 to a fixed memory address, then copies 4 to the memory at an offset from a6, then returns from the exception.
- `frag_move_lea_clr_dbra_rts`: Short code fragment that copies 13202 to d0, then loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then clears the memory at a0 (advancing a0), then clears the memory at a1 (advancing a1), then loops back while counting down d0, then returns.
- `process_input_bits_2`: Reads keyboard input from hardware registers, processes bit patterns, updates a global variable with input state, and modifies hardware interrupt control bits.
- `process_input_bits`: Processes input characters to set or clear specific bits in a global variable, updates a counter, and modifies hardware register bits.
- `frag_bset_bra`: Short code fragment that sets bit 0 of d1, then jumps to another point in the code.
- `frag_bset_bra_2`: Short code fragment that sets bit 1 of d1, then jumps to another point in the code.
- `frag_bset_bra_3`: Short code fragment that sets bit 3 of d1, then jumps to another point in the code.
- `frag_bset_bra_4`: Short code fragment that sets bit 2 of d1, then jumps to another point in the code.
- `frag_bset_bra_5`: Short code fragment that sets bit 4 of d1, then jumps to another point in the code.
- `frag_bclr_bra`: Short code fragment that clears bit 0 of d1, then jumps to another point in the code.
- `frag_bclr_bra_2`: Short code fragment that clears bit 1 of d1, then jumps to another point in the code.
- `frag_bclr_bra_3`: Short code fragment that clears bit 3 of d1, then jumps to another point in the code.
- `frag_bclr_bra_4`: Short code fragment that clears bit 2 of d1, then jumps to another point in the code.
- `frag_bclr_bra_5`: Short code fragment that clears bit 4 of d1, then jumps to another point in the code.
- `frag_bclr_move_ext_rts`: Short code fragment that clears bit 6 of a fixed memory address, then copies a fixed memory address to d0, then copies 255 to a fixed memory address, then sign-extends d0, then copies d0 to a fixed memory address, then returns.
- `normalize_coordinate_value`: Normalizes a coordinate value by shifting, masking, and adjusting it based on table lookups, then performs division operations to calculate a final result.
- `initialize_and_update_tables`: Initializes memory locations with specific values, updates counters, and calculates vertex positions by performing arithmetic operations and storing results in tables.
- `frag_move_clr`: Short code fragment that copies 2544 to the memory at an offset from a6, then clears the memory at an offset from a6 (2 times in a row), then copies a fixed memory address to the memory at an offset from a6, then copies an address to the memory at an offset from a6.
- `setup_graphics_loop`: Runs a loop that tests a memory value, branches if zero, copies data from one memory location to another, and writes fixed values to specific offsets in a structure.
- `frag_move_rte`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6 (2 times in a row), then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies 1027 to the memory at an offset from a6, then returns from the exception.
- `frag_move_rte_2`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6 (2 times in a row), then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies 1027 to the memory at an offset from a6, then returns from the exception.
- `frag_move_rte_3`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6 (2 times in a row), then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies 1027 to the memory at an offset from a6, then returns from the exception.
- `frag_move_rte_4`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6 (2 times in a row), then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies 1027 to the memory at an offset from a6, then returns from the exception.
- `setup_graphics_parameters_2`: Copies data from a pointer to specific offsets in a structure, writes a function address to a fixed offset, sets specific word values in the structure, and returns via rte instruction.
- `frag_move_rte_5`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6 (2 times in a row), then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies the memory at a5 (advancing a5) to the memory at an offset from a6, then returns from the exception.
- `frag_move_rte_6`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6 (2 times in a row), then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies the memory at a5 (advancing a5) to the memory at an offset from a6, then returns from the exception.
- `frag_move_rte_7`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6 (2 times in a row), then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies the memory at a5 (advancing a5) to the memory at an offset from a6, then returns from the exception.
- `frag_move_rte_8`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6 (2 times in a row), then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies the memory at a5 (advancing a5) to the memory at an offset from a6, then returns from the exception.
- `frag_move_rte_9`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6 (2 times in a row), then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies the memory at a5 (advancing a5) to the memory at an offset from a6, then returns from the exception.
- `frag_move_rte_10`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6 (3 times in a row), then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies the memory at a5 (advancing a5) to the memory at an offset from a6, then returns from the exception.
- `frag_move_rte_11`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6 (2 times in a row), then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies the memory at a5 (advancing a5) to the memory at an offset from a6, then returns from the exception.
- `frag_move_rte_12`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6 (2 times in a row), then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies the memory at a5 (advancing a5) to the memory at an offset from a6, then returns from the exception.
- `frag_move_rte_13`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6 (2 times in a row), then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies the memory at a5 (advancing a5) to the memory at an offset from a6, then returns from the exception.
- `frag_move_rte_14`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6 (2 times in a row), then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies the memory at a5 (advancing a5) to the memory at an offset from a6, then returns from the exception.
- `setup_graphics_parameters`: Copies data from a pointer to specific offsets in a structure, writes a function address to a fixed offset, sets specific word values in the structure, and returns using an RTE instruction.
- `frag_move_rte_15`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6 (2 times in a row), then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies the memory at a5 (advancing a5) to the memory at an offset from a6, then returns from the exception.
- `frag_move_rte_16`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6 (2 times in a row), then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies the memory at a5 (advancing a5) to the memory at an offset from a6, then returns from the exception.
- `frag_move_rte_17`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6 (2 times in a row), then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies the memory at a5 (advancing a5) to the memory at an offset from a6, then returns from the exception.
- `frag_move_rte_18`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6 (2 times in a row), then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies the memory at a5 (advancing a5) to the memory at an offset from a6, then returns from the exception.
- `frag_move_rte_19`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6 (2 times in a row), then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies the memory at a5 (advancing a5) to the memory at an offset from a6, then returns from the exception.
- `initialize_graphics_data`: Reads control words from a source pointer and initializes graphics-related fields in a target structure based on the control word values, then sets a global variable to point to another routine and returns.
- `copy_struct_fields_to_context`: The routine copies four longs from a source pointer into fixed offsets of a target structure pointed to by a register, stores the address of another routine into a global variable, writes a constant word and copies a word from the source into the target, then returns.
- `frag_move_rte_20`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6 (5 times in a row), then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies the memory at a5 (advancing a5) to the memory at an offset from a6, then returns from the exception.
- `frag_move_rte_21`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6 (5 times in a row), then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies the memory at a5 (advancing a5) to the memory at an offset from a6, then returns from the exception.
- `frag_move_rte_22`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6 (4 times in a row), then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies the memory at a5 (advancing a5) to the memory at an offset from a6, then returns from the exception.
- `store_struct_fields`: Moves data from a source pointer to specific offsets within a target structure and sets a global variable to point to another routine.
- `copy_struct_fields_to_context_2`: The routine copies four longs from a source pointer into fixed offsets of a target structure pointed to by a register, stores the address of another routine into a global variable, writes a constant word and copies a word from the source into the target, then returns.
- `frag_move_rte_23`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6 (4 times in a row), then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies the memory at a5 (advancing a5) to the memory at an offset from a6, then returns from the exception.
- `frag_move_rte_24`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6 (4 times in a row), then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies the memory at a5 (advancing a5) to the memory at an offset from a6, then returns from the exception.
- `frag_move_rte_25`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6 (4 times in a row), then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies the memory at a5 (advancing a5) to the memory at an offset from a6, then returns from the exception.
- `frag_move_rte_26`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6 (4 times in a row), then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies the memory at a5 (advancing a5) to the memory at an offset from a6, then returns from the exception.
- `copy_and_setup_graphics_data`: Copies data from a source location to a destination with offset calculations, clears specific memory locations, and sets up graphics parameters for rendering.
- `frag_move_rte_27`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6, then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies 12801 to the memory at an offset from a6, then returns from the exception.
- `frag_move_rte_28`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6, then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies 12801 to the memory at an offset from a6, then returns from the exception.
- `frag_move_rte_29`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6, then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies 12801 to the memory at an offset from a6, then returns from the exception.
- `frag_move_rte_30`: Short code fragment that copies the memory at a5 (advancing a5) to the memory at an offset from a6, then copies an address to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies 12801 to the memory at an offset from a6, then returns from the exception.
- `frag_move_rte_31`: Short code fragment that copies 64 to the memory at an offset from a6, then copies 1 to a fixed memory address, then returns from the exception.
- `update_sprite_data`: Processes sprite positioning and rendering parameters based on input coordinates and updates global sprite structure fields.
- `compute_render_coordinates`: Calculates and stores coordinate adjustments for rendering operations based on input parameters and global variables, writing results to a memory buffer.
- `check_collision_bounds`: Tests collision boundaries against input values and updates global variables with calculated coordinates based on comparison results.
- `calculate_movement_delta`: Calculates movement deltas for position adjustments based on various conditional checks and updates global variables accordingly.
- `frag_clr_move_tst_beq_bra`: Short code fragment that clears a fixed memory address, then copies a fixed memory address to a fixed memory address, then copies d5 to a fixed memory address, then tests a fixed memory address, then branches if the last result was zero, then jumps to another point in the code (2 times in a row).
- `update_car_positions`: Processes car position data by rotating and adding coordinates, updates global variables with computed values, and performs collision checks against boundaries.
- `update_sprite_positions`: Loads sprite data from a table using an index from a global variable, then updates position coordinates and calls rendering routines with different parameters based on conditional checks and global state values.
- `calculate_sprite_offset`: Computes a sprite offset value by processing a coordinate from the input structure through a series of arithmetic operations, stores the result in a global variable, then updates position coordinates and calls rendering routines with specific parameters.
- `update_sprite_positions_2`: Loads sprite coordinates from global memory locations, calculates averaged values, and calls a rendering routine with the computed data.
- `process_sprite_rendering`: Copies a global value to a local register, then updates sprite position coordinates and calls rendering routines multiple times with different parameter combinations based on conditional checks and global state values.
- `initialize_render_data`: Initializes rendering data by writing a word value to a memory location and then populating a sequence of long values with calculated offsets.
- `compute_coordinate_adjustment`: Computes coordinate adjustments using division and multiplication operations based on input parameters and global variables.
- `update_lookup_tables`: Updates multiple lookup tables by performing arithmetic operations and copying data from various memory locations to global variables.
- `initialize_car_data`: Initializes car-related data structures by clearing and setting specific fields based on loop iterations and global variable values, then performs additional setup operations including memory copying and arithmetic calculations.
- `perform_addition_with_carry`: Performs addition operations with carry propagation across multiple 32-bit values stored in a sequence of memory locations.
- `process_vehicle_logic`: Processes vehicle state updates including position calculations, collision detection, and animation frame management based on input parameters and global game data.
- `process_sprite_data_2`: Iterates through sprite data structures, calculates various position and movement values based on conditional logic, and calls a rendering routine with computed parameters.
- `update_sprite_coordinates`: Updates sprite position coordinates in a global structure and calls rendering routines to process sprite data based on calculated movement deltas.
- `manage_movement_counters`: Manages decrementing counters for movement tracking and performs addition operations with carry propagation on 32-bit values stored in memory locations.
- `frag_bsr_addq_andi_move_asl_add_lea_mov`: Short code fragment that calls another routine, then adds 1 to a fixed memory address, then masks a fixed memory address with 3, then copies a fixed memory address to d0, then shifts d0 left by 2, then adds a fixed memory address to d0, then shifts d0 left by 2, then loads the address of a fixed memory address into a0, then copies an indexed entry from a0 to a0, then jumps to the address in a0.
- `compare_and_update_coordinates`: Compares current coordinate values with stored values and updates global variables when differences are detected.
- `frag_move_subq_tst_bne_beq_lea_bsr_rts`: Short code fragment that copies a fixed memory address to d7, then subtracts 1 from d7, then tests a fixed memory address, then branches if the last result was not zero, then tests a fixed memory address, then branches if the last result was zero, then loads the address of a fixed memory address into a0, then calls another routine, then returns.
- `transfer_pixel_data_2`: Checks conditional flags and calls a pixel data transfer routine when specific conditions are met, using a fixed address as the source pointer.
- `frag_move_sub_bpl_clr_addi_lea_bsr_rts`: Short code fragment that copies a fixed memory address to d7, then subtracts a fixed memory address from d7, then branches if the last result was not negative, then clears d7, then adds 10 to d7, then loads the address of a fixed memory address into a0, then calls another routine, then returns.
- `frag_move_sub_bpl_clr_addi_lea_bsr_rts_2`: Short code fragment that copies a fixed memory address to d7, then subtracts a fixed memory address from d7, then branches if the last result was not negative, then clears d7, then adds 10 to d7, then loads the address of a fixed memory address into a0, then calls another routine, then returns.
- `frag_move_bpl_clr_lsr_lea_bsr_rts`: Short code fragment that copies a fixed memory address to d7, then branches if the last result was not negative, then clears d7, then shifts d7 right by 6, then loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then calls another routine, then returns.
- `frag_move_bpl_clr_lsr_lea_bsr_rts_2`: Short code fragment that copies a fixed memory address to d7, then branches if the last result was not negative, then clears d7, then shifts d7 right by 6, then loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then calls another routine, then returns.
- `frag_move_bpl_clr_lsr_lea_bsr_rts_3`: Short code fragment that copies a fixed memory address to d7, then branches if the last result was not negative, then clears d7, then shifts d7 right by 6, then loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then calls another routine, then returns.
- `frag_move_bpl_clr_lsr_lea_bsr_rts_4`: Short code fragment that copies a fixed memory address to d7, then branches if the last result was not negative, then clears d7, then shifts d7 right by 6, then loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then calls another routine, then returns.
- `transfer_pixel_data_2_2`: Checks conditional flags and calls a pixel data transfer routine when specific conditions are met, using another fixed address as the source pointer.
- `frag_move_sub_bpl_clr_addi_lea_bsr_rts_3`: Short code fragment that copies a fixed memory address to d7, then subtracts a fixed memory address from d7, then branches if the last result was not negative, then clears d7, then adds 10 to d7, then loads the address of a fixed memory address into a0, then calls another routine, then returns.
- `frag_move_bpl_clr_lsr_lea_bsr_rts_5`: Short code fragment that copies a fixed memory address to d7, then branches if the last result was not negative, then clears d7, then shifts d7 right by 6, then loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then calls another routine, then returns.
- `frag_move_bpl_clr_lsr_lea_bsr_rts_6`: Short code fragment that copies a fixed memory address to d7, then branches if the last result was not negative, then clears d7, then shifts d7 right by 6, then loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then calls another routine, then returns.
- `copy_and_combine_bytes`: Copies and combines byte values from source to destination locations using bitwise AND operations and loop counters.
- `transfer_pixel_data_blocks`: Transfers pixel data blocks between memory locations by copying byte values from a source table to a destination buffer with offset calculations.
- `process_memory_block_operations`: Processes memory block operations by performing arithmetic calculations, copying data between addresses, and updating global variables during nested loop iterations.
- `rotate_and_extract_bits`: Rotates bits in a word value and extracts specific bit fields to form a composite result with conditional logic based on input values.
- `frag_lea_andi_mulu_lsr_add_move_rol_rts`: Short code fragment that loads the address of a fixed memory address into a0, then masks d0 with 65532, then masks d1 with 65532, then multiplies d1 by a fixed memory address (unsigned), then shifts d0 right by 1, then adds d0 to d1, then copies an indexed entry from a0 to d0, then rotates d0 left by 4, then masks d0 with 15, then returns.
- `calculate_texture_offset`: Computes a texture coordinate offset by multiplying a value from a global table with a word register, then shifts the result right by one bit before adding it to another table index.
- `compare_vertex_data`: Compares vertex data between two structures in memory, performing multiple bit shifts and additions on word values before checking for equality or ordering.
- `update_game_state`: Processes game state variables by performing arithmetic operations on counter values and updating memory locations based on conditional checks.
- `update_car_physics`: Updates car position and velocity values based on input parameters and performs collision detection with other objects.
- `process_object_state`: Updates object state variables including position, velocity, and flags based on input parameters and global data tables.
- `initialize_object`: Initializes an object at a calculated offset based on input parameter, setting various fields including type, position, and velocity components, then returns.
- `accumulate_scaled_values`: Scales values from a global table using a register value, shifts the result right by eight bits, and accumulates these into two word offsets in another structure.
- `transform_coordinates`: Performs matrix-like coordinate transformations using multiplication and addition operations on word-sized values from memory, storing the results as bytes in two separate output locations.
- `initialize_tables`: Populates multiple memory regions with predefined word values through a series of loops, copying data from one table to another and generating sequential values with incrementing offsets.
- `copy_vertex_data`: Copies vertex data from one memory location to another, processing sixteen word values through a loop with fixed offsets and incrementing pointers.
- `read_joystick_input`: Reads joystick and mouse input registers, processes bit flags for movement and button states, then stores the combined result in a global variable.
- `initialize_audio_buffers`: Initializes audio buffer structures by clearing memory locations, setting up pointers, and configuring various audio parameters for multiple channels.
- `frag_moveq_bsr_tst_beq_rts`: Short code fragment that loads 4 into d0, then loads 0 into d1, then calls another routine, then tests a fixed memory address, then branches if the last result was zero, then loads 4 into d0, then loads 1 into d1, then calls another routine, then returns.
- `update_audio_channel`: Updates audio channel settings by reading from a global table, performing bit manipulation on a register value, and writing to hardware registers.
- `configure_audio_parameters`: Configures audio parameters for a sound channel by calculating offsets, reading values from tables, and writing to hardware registers with bit shifts.
- `update_velocity_values`: Processes velocity data by reading from global memory locations, performing conditional comparisons, and calling subroutines to modify values based on thresholds.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (bset.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs one instruction (bset.b) and then falls through into the code that follows, with no return of its own.
- `normalize_velocity`: Processes a velocity value and adjusts it based on thresholds, potentially clamping or modifying the value before returning.
- `frag_tst_bpl_clr_swap_divu_mulu_addi`: Short code fragment that tests d1, then branches if the last result was not negative, then clears d1, then swaps the halves of d1, then clears d1, then divides d1 by 3360 (unsigned), then multiplies d1 by 5500 (unsigned), then swaps the halves of d1, then adds 2500 to d1.
- `frag_move_divu_lea_asl_adda_rts`: Short code fragment that copies an address to d3, then divides d3 by d1 (unsigned), then loads the address of a fixed memory address into a0, then shifts d2 left by 4, then adds d2 to a0, then copies d0 to the memory at an offset from a0, then copies d3 to the memory at an offset from a0, then returns.
- `frag_move_moveq_lsr_addi_bra`: Short code fragment that copies a3 to d1, then loads 0 into d0, then copies the memory at an offset from a3 to d2, then shifts d2 right by 6, then adds 32 to d2, then jumps to another point in the code.
- `frag_move_moveq_bra`: Short code fragment that copies a3 to d1, then loads 2 into d0, then loads 64 into d2, then jumps to another point in the code.
- `frag_move_moveq_bra_2`: Short code fragment that copies a2 to d1, then loads 2 into d0, then loads 64 into d2, then jumps to another point in the code.
- `frag_move_moveq_lsr_addi_bra_2`: Short code fragment that copies a3 to d1, then loads 1 into d0, then copies the memory at an offset from a3 to d2, then shifts d2 right by 6, then adds 32 to d2, then jumps to another point in the code.
- `frag_move_moveq_lsr_addi_bra_3`: Short code fragment that copies a2 to d1, then loads 1 into d0, then copies the memory at an offset from a2 to d2, then shifts d2 right by 6, then adds 32 to d2, then jumps to another point in the code.
- `frag_move_moveq_lsr_addi_bra_4`: Short code fragment that copies a3 to d1, then loads 6 into d0, then copies the memory at an offset from a3 to d2, then shifts d2 right by 6, then adds 32 to d2, then jumps to another point in the code.
- `frag_move_moveq_lsr_addi_bra_5`: Short code fragment that copies a3 to d1, then loads 7 into d0, then copies the memory at an offset from a3 to d2, then shifts d2 right by 6, then adds 32 to d2, then jumps to another point in the code.
- `frag_move_moveq_lsr_addi_bra_6`: Short code fragment that copies a3 to d1, then loads 5 into d0, then copies the memory at an offset from a3 to d2, then shifts d2 right by 6, then adds 32 to d2, then jumps to another point in the code.
- `configure_audio_channels`: Configures two audio channels by calling an audio setup routine twice with specific parameter values for channel, register, and data settings.
- `configure_audio_parameters_2`: Sets up audio parameters for sound channels by calculating offsets, reading from tables, and writing to hardware registers using specific values.
- `frag_moveq_move_bsr_rts`: Short code fragment that loads 80 into d0, then copies an address to d1, then calls another routine, then loads 81 into d0, then copies an address to d1, then calls another routine, then returns.
- `execute_sound_commands`: Executes a series of sound command operations by calling another routine multiple times, checking results after each call and branching based on test conditions.
- `clear_and_copy_memory`: Clears large memory blocks and copies data between different memory regions using fixed offsets, performing multiple memory operations with loop counters.
- `copy_structure_data`: Copies data from a source structure to a destination structure, performing 679 iterations of copying 13 words and setting a counter to zero.
- `frag_lea_movea_move_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies a fixed memory address to a1, then copies 1999 to d7, then copies the memory at an offset from a0 to the memory at an offset from a1 (4 times in a row), then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d7, then returns.
- `store_vertex_attributes`: Reads vertex data from memory locations and stores them into specific offsets in another structure, then clears several global variables.
- `update_vertex_attributes`: Writes vertex data from global variables to memory locations at calculated offsets, updating a structure with new values.
- `frag_move_mulu_lea_rts`: Short code fragment that copies a fixed memory address to d0, then multiplies d0 by 6 (unsigned), then loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then copies an indexed entry from a1 to the memory at an offset from a0 (3 times in a row), then returns.
- `modify_bit_flags`: Modifies bit flags in a memory structure by performing bitwise AND operations with 8191 and OR operations with data from another structure.
- `transfer_pixel_data`: Copies pixel data from one memory location to another, transferring 104 rows of 5 words each using a loop counter.
- `copy_pixel_data`: Copies pixel data from a source address to multiple destination addresses based on loop counters and conditional checks.
- `frag_move_addi_bsr_rts`: Short code fragment that copies a fixed memory address to d0, then adds 224 to d0, then copies an address to d1, then calls another routine, then returns.
- `frag_move_addi_bsr_rts_2`: Short code fragment that copies a fixed memory address to d0, then adds 192 to d0, then copies an address to d1, then calls another routine, then returns.
- `frag_move_bsr_rts`: Short code fragment that copies 252 to d0, then copies an address to d1, then calls another routine, then returns.
- `manage_object_list_2`: Updates an object list by storing pointers in a table based on a value, then calls another routine with the result when a condition is met.
- `manage_object_list`: Manages an object list by storing pointers in a table based on a value, updating global variables and calling another routine when conditions are met.
- `initialize_rendering_data`: Initializes rendering data by calling another routine with different parameter combinations, setting up various memory locations with specific values.
- `frag_moveq_move_bsr_rts_2`: Short code fragment that loads 64 into d0, then copies an address to d1, then calls another routine (2 times in a row), then returns.
- `frag_move_tst_beq_bsr_rts`: Short code fragment that copies 176 to d0, then tests a fixed memory address, then branches if the last result was zero, then copies 177 to d0, then copies an address to d1, then calls another routine, then returns.
- `control_rendering_state`: Controls rendering state by setting flags, calling a routine, incrementing a counter, and clearing a flag when the counter reaches four.
- `frag_moveq_move_bsr_tst_bne_rts`: Short code fragment that loads 48 into d0, then copies an address to d1, then calls another routine, then tests a fixed memory address, then branches if the last result was not zero, then loads 50 into d0, then copies an address to d1, then calls another routine, then returns.
- `frag_moveq_move_bsr_rts_3`: Short code fragment that loads 32 into d0, then copies 61760 to d1, then calls another routine, then returns.
- `frag_moveq_move_bsr_rts_4`: Short code fragment that loads 33 into d0, then copies 61760 to d1, then calls another routine, then returns.
- `frag_moveq_move_bsr_rts_5`: Short code fragment that loads 23 into d0, then copies a fixed memory address to d1, then calls another routine, then returns.
- `frag_moveq_move_bsr_rts_6`: Short code fragment that loads 28 into d0, then copies a fixed memory address to d1, then calls another routine, then returns.
- `frag_moveq_move_bsr_rts_7`: Short code fragment that loads 24 into d0, then copies a fixed memory address to d1, then calls another routine, then returns.
- `frag_moveq_move_bsr_rts_8`: Short code fragment that loads 25 into d0, then copies a fixed memory address to d1, then calls another routine, then returns.
- `frag_moveq_move_bsr_rts_9`: Short code fragment that loads 26 into d0, then copies a fixed memory address to d1, then calls another routine, then returns.
- `frag_moveq_move_bsr_rts_10`: Short code fragment that loads 27 into d0, then copies an address to d1, then calls another routine, then returns.
- `frag_moveq_move_bsr_rts_11`: Short code fragment that loads 40 into d0, then copies an address to d1, then calls another routine (2 times in a row), then returns.
- `frag_moveq_move_bsr_rts_12`: Short code fragment that loads 41 into d0, then copies an address to d1, then calls another routine (2 times in a row), then returns.
- `frag_moveq_move_bsr_rts_13`: Short code fragment that loads 42 into d0, then copies an address to d1, then calls another routine (2 times in a row), then returns.
- `frag_moveq_move_bsr_rts_14`: Short code fragment that loads 29 into d0, then copies an address to d1, then calls another routine, then returns.
- `frag_movea_adda_move_dbra_rts`: Short code fragment that copies a fixed memory address to a1, then adds 7200 to a1, then copies an address to a0, then copies 5 to d6, then copies 199 to d7, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d7, then adds 7200 to a1, then loops back while counting down d6, then returns.
- `frag_moveq_move_bsr_rts_15`: Short code fragment that loads 16 into d0, then copies 21760 to d1, then calls another routine, then returns.
- `frag_moveq_move_bsr_rts_16`: Short code fragment that loads 17 into d0, then copies an address to d1, then calls another routine, then returns.
- `frag_moveq_move_bsr_rts_17`: Short code fragment that loads 18 into d0, then copies an address to d1, then calls another routine, then returns.
- `frag_moveq_move_bsr_rts_18`: Short code fragment that loads 19 into d0, then copies an address to d1, then calls another routine, then returns.
- `frag_moveq_move_bsr_rts_19`: Short code fragment that loads 20 into d0, then copies an address to d1, then calls another routine, then returns.
- `frag_moveq_move_bsr_rts_20`: Short code fragment that loads 21 into d0, then copies 21760 to d1, then calls another routine, then returns.
- `frag_moveq_move_bsr_rts_21`: Short code fragment that loads 22 into d0, then copies an address to d1, then calls another routine, then returns.
- `frag_moveq_move_bsr_rts_22`: Short code fragment that loads 43 into d0, then copies 21760 to d1, then calls another routine, then returns.
- `call_stub`: Call stub: calls a routine at a fixed address outside this code unit, then returns to its own caller.
- `call_stub_2`: Call stub: calls a routine at a fixed address outside this code unit, then returns to its own caller.
- `initialize_game_interrupts`: Sets up interrupt vectors and enables specific DMA and interrupt channels while clearing several global variables, then performs hardware register updates and data copying operations.
- `initialize_game_state`: Initializes multiple global variables and hardware registers including interrupt and DMA control, sets up interrupt vectors, then enters a main loop.
- `duplicate_pointer_values`: Duplicates a long value from a global variable to multiple stack locations, then copies data from a table to a structure using indexed addressing.
- `copy_string_segments_2`: Calculates memory addresses using multiplication and table lookups, copies byte sequences from one location to another, and performs a loop to copy multiple bytes with specific offset calculations.
- `process_game_state`: Loads a value from global memory, performs arithmetic operations including multiplication and addition, stores the result in a memory location, and updates a counter based on conditional checks.
- `update_and_store_values`: Loads a value from global memory, performs arithmetic operations including multiplication and addition, stores the result in a memory location, and clears the memory if the result is negative.
- `frag_subq_lea_move_add_suba_bpl_clr_rts`: Short code fragment that subtracts 5 from d1, then loads the address of a fixed memory address into a0, then copies a fixed memory address to d0, then adds d0 to d0, then subtracts d0 from a0, then adds d1 to the memory at a0, then branches if the last result was not negative, then clears the memory at a0, then returns.
- `swap_and_store_values`: Swaps three values using exg instructions, performs memory block exchanges between different addresses, and stores the final results in a structure at a specific location.
- `swap_memory_blocks`: Exchanges 32-bit words between two memory locations in a loop that runs three times, advancing both pointers after each iteration.
- `find_and_mark_slot`: Calculates an index using multiplication and table lookups, tests a memory location for zero, marks it as occupied when found, stores the index in global memory, and calls other routines to process the data.
- `copy_string_segments`: Copies string data segments to different memory locations by calculating offsets from a table and calling another routine to perform the actual copy operation for five distinct string buffers.
- `copy_string_data`: Copies string data from one memory location to another, filling a buffer with forward slashes and then copying characters until reaching a null terminator.
- `frag_lea_bra`: Short code fragment that loads the address of a fixed memory address into a0, then jumps to another point in the code.
- `frag_lea_move_swap_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies a0 to d0, then copies d0 to a fixed memory address, then swaps the halves of d0, then copies d0 to a fixed memory address, then returns.
- `update_player_data`: Copies player position and status data from input buffers to a global structure, updates counters and flags, and initializes game object properties based on player state.
- `compare_and_set_flags`: Compares 32-bit values from two memory regions and sets flags to indicate equality, performing the comparison twice with different memory addresses.
- `frag_move_rts`: Short code fragment that copies a fixed memory address to the stack, then copies a fixed memory address to a fixed memory address, then copies the top of the stack to a fixed memory address, then copies a fixed memory address to a fixed memory address, then returns.
- `frag_move_rts_2`: Short code fragment that copies 15 to the memory at an offset from a6, then returns.
- `interrupt_handler`: Reads the interrupt request register and clears bit four when set, then reads counter values and updates a state word before calling a handler from a function table.
- `copy_memory_block`: Copies 765 32-bit words from one memory location to another using a loop that advances both pointers after each iteration.
- `xor_global_byte`: Performs a bitwise XOR operation between a byte in the d0 register and a byte at a specific memory address, storing the result back at that address.
- `copy_pattern_data`: Copies structured data from a source table to multiple destination locations based on index calculations, handling different data segments through conditional branching and fixed offset addressing.
- `set_global_variables`: Initializes several global variables with specific word values and updates a status flag based on a conditional test of another global variable's value.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `frag_bsr_rts`: Short code fragment that calls another routine, then returns.
- `frag_bsr_rts_2`: Short code fragment that calls another routine, then returns.
- `update_animation_state`: Updates an animation state counter based on bit flags in a global variable, resets the counter to specific values when reaching boundaries, and modifies other global variables depending on conditions.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `hardware_initialize`: Initializes hardware registers by setting specific DMA control and interrupt enable bits, then copies data from a source table to various memory locations.
- `initialize_hardware_registers`: Initializes hardware registers by setting specific bits in DMA control and interrupt enable registers, then copies data from a source table to a destination table using a loop.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `copy_sprite_data`: Copies sprite data from a source table to a destination table based on a calculated index, performing the copy operation 30 times in a loop.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_11`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_12`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_13`: Empty routine: returns to the caller immediately without doing anything.
- `frag_cmpi_bne_bsr_bcs_tst_move_rts`: Short code fragment that compares a fixed memory address with 8, then branches if the last result was not zero, then calls another routine, then compares a fixed memory address with 20, then branches if the carry flag is set, then tests a fixed memory address, then branches if the last result was not zero, then copies 1 to a fixed memory address, then returns.
- `frag_bsr_rts_3`: Short code fragment that calls another routine, then returns.
- `empty_stub_14`: Empty routine: returns to the caller immediately without doing anything.
- `frag_bsr_rts_4`: Short code fragment that calls another routine (2 times in a row), then returns.
- `frag_bsr_rts_5`: Short code fragment that calls another routine (2 times in a row), then returns.
- `frag_bsr_rts_6`: Short code fragment that calls another routine, then returns.
- `frag_bsr_rts_7`: Short code fragment that calls another routine (2 times in a row), then returns.
- `frag_bsr_rts_8`: Short code fragment that calls another routine, then returns.
- `frag_bsr_rts_9`: Short code fragment that calls another routine, then returns.
- `initialize_rendering`: Copies byte values from specific memory locations to a global status byte, checks a word value against a constant, and sets various flags and counters in global memory based on conditional tests.
- `update_game_state_2`: Updates a counter variable based on bit tests of a global flag word, then writes the updated value back to memory and performs additional operations based on comparison results with predefined constants.
- `frag_move_bsr_rts_2`: Short code fragment that copies 2 to a fixed memory address, then calls another routine, then copies 1 to a fixed memory address, then returns.
- `update_counter_and_flags`: Increments a counter in global memory, compares values from global memory with a loaded value, sets flags based on comparison results, and updates multiple global variables depending on the outcome of these comparisons.
- `frag_bsr_move_rts`: Short code fragment that calls another routine, then copies 1 to a fixed memory address, then returns.
- `frag_bsr_rts_10`: Short code fragment that calls another routine (2 times in a row), then returns.
- `empty_stub_15`: Empty routine: returns to the caller immediately without doing anything.
- `copy_tile_data`: Copies tile data from a source table to a destination table using an index calculated from a global variable, performing the copy operation 50 times in a loop.
- `update_car_state`: Updates car position boundaries based on input flags and global state, then performs conditional actions based on a state variable.
- `update_position_bounds`: Updates position coordinates based on input flags, clamping values within specific bounds and modifying global variables accordingly.
- `frag_addq_bra`: Short code fragment that adds 1 to d4, then jumps to another point in the code.
- `empty_stub_16`: Empty routine: returns to the caller immediately without doing anything.
- `process_input_event`: Processes input events based on a value in a register, updating global variables and calling other routines conditionally.
- `frag_bsr_rts_11`: Short code fragment that calls another routine (2 times in a row), then returns.
- `frag_move_bsr_rts_3`: Short code fragment that copies 1 to a fixed memory address, then calls another routine (6 times in a row), then returns.
- `frag_cmpi_beq_bra`: Short code fragment that compares a fixed memory address with 0, then branches if the last result was zero, then compares a fixed memory address with 1, then branches if the last result was zero, then compares a fixed memory address with 2, then branches if the last result was zero, then jumps to another point in the code.
- `decrement_and_accumulate`: Decrements a value in a global array and accumulates the result into another global variable, calling subroutines to process the accumulated data.
- `compare_and_modify_values`: Compares values from two memory regions, performs conditional modifications based on comparison results, and updates global variables accordingly.
- `frag_cmp_bcc_bne_bra`: Short code fragment that compares d4 with 6, then branches if the carry flag is clear, then compares d4 with a fixed memory address, then branches if the last result was not zero, then jumps to another point in the code.
- `bit_flag_modify`: Saves a word value from a global variable onto the stack, modifies bit flags in a memory structure using a fixed value of 8, then restores the original value.
- `frag_cmp_bcc_bne_bra_2`: Short code fragment that compares d4 with 6, then branches if the carry flag is clear, then compares d4 with a fixed memory address, then branches if the last result was not zero, then jumps to another point in the code.
- `bit_flag_modify_2`: Saves a word value from another global variable onto the stack, modifies bit flags in a memory structure using a fixed value of 4, then restores the original value.
- `word_swap_and_modify`: Swaps two word values between global variables, then modifies bit flags in a memory structure using a fixed value of 4, followed by another flag modification with value 6.
- `initialize_and_process`: Initializes a global variable to zero and calls multiple subroutines to process data, without performing any direct operations on the initialized value.
- `frag_cmpi_bcs_move_clr_bsr_moveq_rts`: Short code fragment that compares a fixed memory address with 2, then branches if the carry flag is set, then copies a fixed memory address to a fixed memory address, then clears a fixed memory address, then calls another routine, then loads 0 into d1, then loads 4 into d2, then loads 8 into d3, then calls another routine, then returns.
- `frag_cmpi_bcs_move_bsr_moveq_rts`: Short code fragment that compares a fixed memory address with 2, then branches if the carry flag is set, then copies a fixed memory address to a fixed memory address, then copies 1 to a fixed memory address, then calls another routine, then loads 0 into d1, then loads 8 into d2, then loads 11 into d3, then calls another routine, then returns.
- `frag_move_bsr_moveq_rts`: Short code fragment that copies 2 to a fixed memory address, then calls another routine, then loads 0 into d1, then loads 11 into d2, then loads 14 into d3, then calls another routine (2 times in a row), then returns.
- `frag_move_bsr_moveq_rts_2`: Short code fragment that copies 3 to a fixed memory address, then calls another routine, then loads 0 into d1, then loads 14 into d2, then loads 17 into d3, then calls another routine (2 times in a row), then returns.
- `validate_and_update_counters`: Validates values against global thresholds, updates counters in memory locations, and accumulates results into another global variable through subroutine calls.
- `frag_cmpi_beq_moveq_bra`: Short code fragment that compares a fixed memory address with 2, then branches if the last result was zero, then compares a fixed memory address with 0, then branches if the last result was zero, then compares a fixed memory address with 1, then branches if the last result was zero, then loads 14 into d2, then loads 17 into d3, then jumps to another point in the code.
- `frag_moveq_bra`: Short code fragment that loads 4 into d2, then loads 8 into d3, then jumps to another point in the code.
- `frag_moveq_bra_2`: Short code fragment that loads 8 into d2, then loads 11 into d3, then jumps to another point in the code.
- `frag_moveq_bsr_rts`: Short code fragment that loads 11 into d2, then loads 14 into d3, then loads 0 into d1, then calls another routine, then returns.
- `frag_moveq_bsr_rts_2`: Short code fragment that loads 1 into d1, then loads 0 into d2, then loads 20 into d3, then calls another routine, then returns.
- `loop_with_flags`: Processes data from three memory regions in a loop that iterates seven times, performing bitwise operations and calling another routine for each iteration.
- `conditional_loop_with_flags`: Processes data from three memory regions in a loop that iterates eight times when a global value is not equal to 2, performing bitwise operations and calling another routine for each iteration.
- `loop_with_conditional_modify`: Processes data from three memory regions in a loop that iterates eight times, performing conditional modifications based on counter values and calling another routine for each iteration.
- `initialize_and_loop`: Initializes two global variables to zero, then enters a loop that compares a value with memory contents and adjusts a counter until the comparison fails.
- `frag_move_tst_bne_cmpa_beq_bra`: Short code fragment that copies d0 to the stack, then tests a fixed memory address, then branches if the last result was not zero, then compares a2 with an address, then branches if the last result was zero, then tests d4, then branches if the last result was zero, then copies 1 to a fixed memory address, then jumps to another point in the code.
- `frag_addq_bra_2`: Short code fragment that adds 1 to a fixed memory address, then jumps to another point in the code.
- `process_loop_with_conditions`: Processes data through a loop that compares values with memory contents, adjusts counters, and modifies global variables based on comparison outcomes.
- `frag_move_addq_moveq_bsr_rts`: Short code fragment that copies a fixed memory address to d0, then adds 1 to d0, then loads 37 into d1, then loads 7 into d2, then loads 1 into d3, then calls another routine, then returns.
- `frag_move_moveq_bsr_rts`: Short code fragment that copies a fixed memory address to d0, then loads 42 into d1, then loads 13 into d2, then loads 6 into d3, then calls another routine, then returns.
- `pattern_buffer_fill`: Fills a pattern buffer with zero bytes at specific offsets, then performs bitwise operations to set specific bits in the buffer based on multiplication results.
- `fill_pattern_buffer`: Fills a pattern buffer with data from a table using bitwise operations, performing the operation for multiple rows in a loop while updating a counter.
- `modify_pixel_data`: Processes pixel data by applying bitwise operations with shifted values from a source table, modifying destination data through multiple memory locations.
- `bitwise_modify_pixels`: Modifies pixel data in a destination table by applying bitwise AND and OR operations with shifted values from a source table, performing the operation for multiple rows in a loop.
- `calculate_scaled_values`: Calculates scaled values using multiplication, division, and carry propagation operations on data stored in sequential memory locations.
- `process_arithmetic_operations`: Performs arithmetic calculations including multiplication, division, and conditional updates on data values stored in a global variable table.
- `initialize_structure_flags`: Clears a memory location and sets multiple flags based on input values, then stores byte values at specific offsets within the structure.
- `check_conditions_and_copy`: Tests word values against specific conditions and performs conditional jumps, checking bit states in global memory locations before branching to different code paths.
- `frag_move_rts_3`: Short code fragment that copies 100 to a fixed memory address, then copies 110 to a fixed memory address, then copies 99 to a fixed memory address, then returns.
- `clear_and_copy`: Clears a global word variable and then copies nineteen bytes from multiple table locations into a global buffer through a series of routine calls.
- `frag_lea_bsr_rts`: Short code fragment that loads the address of a fixed memory address into a1, then calls another routine, then returns.
- `update_score_data`: Copies byte data between multiple tables and updates score-related variables based on conditional comparisons and arithmetic operations.
- `copy_and_compare_data`: Selects a data table based on a condition, copies data from multiple sources into the selected table, and performs byte-by-byte comparisons with specific offsets.
- `find_and_copy_entry`: Searches through a table for a matching value, then copies a fixed number of bytes from the matching entry to a destination buffer.
- `frag_bsr_lea_rts`: Short code fragment that calls another routine, then loads the address of a fixed memory address into a1, then calls another routine, then returns.
- `frag_lea_bsr_rts_2`: Short code fragment that loads the address of a fixed memory address into a1, then calls another routine, then returns.
- `frag_lea_bsr_rts_3`: Short code fragment that loads the address of a fixed memory address into a1, then calls another routine, then returns.
- `load_and_store_table_data`: Selects a data table based on input conditions, loads specific entries from that table, and stores them at designated offsets within a structure.
- `frag_movea_bsr_rts`: Short code fragment that copies a fixed memory address to a1, then calls another routine, then returns.
- `frag_lea_bsr_rts_4`: Short code fragment that loads the address of a fixed memory address into a1, then calls another routine, then returns.
- `frag_lea_bsr_rts_5`: Short code fragment that loads the address of a fixed memory address into a1, then calls another routine, then returns.
- `frag_lea_bsr_rts_6`: Short code fragment that loads the address of a fixed memory address into a1, then calls another routine, then returns.
- `frag_lea_bsr_rts_7`: Short code fragment that loads the address of a fixed memory address into a1, then calls another routine, then returns.
- `frag_tst_beq_moveq_move_movea_bra`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then loads 0 into d6, then loads 0 into d7, then copies a fixed memory address to d6, then copies a fixed memory address to d7, then copies a fixed memory address to a1, then jumps to another point in the code.
- `empty_stub_17`: Empty routine: returns to the caller immediately without doing anything.
- `process_sprite_data`: Reads sprite configuration data from a buffer and performs bit manipulation on memory locations using shift and mask operations.
- `process_bitwise_operations`: Performs bitwise operations on memory locations using shift and mask operations, then applies logical AND and OR operations to modify data in multiple memory regions.
- `empty_stub_18`: Empty routine: returns to the caller immediately without doing anything.
- `update_player_input`: Updates player input state based on control bits and manages input buffer pointers, handling character input processing and buffer management for player data.
- `update_counter_and_call`: Checks bits in a global variable to determine increment or decrement behavior for a counter, then calls a function pointer based on the result.
- `calculate_sum_from_table`: Processes a sequence of bytes by indexing into lookup tables and accumulating values, returning the final accumulated sum as a word value.
- `frag_lea_move_moveq_bsr_rts`: Short code fragment that loads the address of a fixed memory address into a1, then copies a fixed memory address to d0, then copies an indexed entry from a1 to d0, then loads 0 into d1, then calls another routine, then returns.
- `frag_lea_move_andi_bsr_rts`: Short code fragment that loads the address of a fixed memory address into a1, then copies a fixed memory address to d0, then copies a fixed memory address to d1, then masks d1 with 2, then copies an indexed entry from a1 to d0, then calls another routine, then returns.
- `copy_data_from_table`: Calculates an address offset using a table value, copies nineteen bytes from the calculated location into a global variable, and calls another routine to process the data.
- `frag_lea_moveq_move_dbra_bsr_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then loads 19 into d7, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d7, then loads the address of a fixed memory address into a1, then calls another routine, then returns.
- `copy_data_to_self`: Copies nineteen bytes from a global variable into itself, performing a self-copy operation through a loop that iterates through memory locations.
- `copy_data_from_offset_table`: Calculates an address offset using another table value, copies nineteen bytes from the calculated location into a global variable, and calls another routine to process the data.
- `copy_data_from_table_2`: Copies nineteen bytes from a location calculated from a table offset into a global variable.
- `copy_data_from_table_3`: Copies nineteen bytes from a location calculated from a table offset into a global variable.
- `copy_table_data`: Copies nineteen bytes from a table location calculated using a word value into a global buffer.
- `copy_table_data_2`: Copies nineteen bytes from a location calculated using a table index into a global variable.
- `sort_and_update`: Sorts data in a table using bubble sort logic with carry propagation, then updates values based on comparisons with global variables.
- `update_score_display`: Updates a score display buffer with formatted numeric digits representing the score, then processes game object positions and updates global score variables based on object states.
- `find_min_max_values`: Finds minimum and maximum values from a set of data, stores the results in two separate buffers, and updates tracking variables during iteration.
- `initialize_table_data`: Initializes table entries by copying data from a source buffer to a destination buffer, with specific handling for values 8 and 9, and sets flags in global variables based on these values.
- `convert_number_to_digits`: Converts a word value into hundreds, tens, and units digits, then converts these digits into ASCII character representations with leading zero handling.
- `check_global_conditions`: Compares global variables against specific constant values and calls another routine when conditions are met, performing conditional execution based on multiple variable states.
- `frag_move_lea_bsr_tst_beq_rts`: Short code fragment that copies a fixed memory address to d0, then loads the address of a fixed memory address into a1, then calls another routine, then tests a fixed memory address, then branches if the last result was zero, then copies a fixed memory address to d0, then loads the address of a fixed memory address into a1, then calls another routine, then returns.
- `load_and_process_data`: Loads a word value from a fixed memory location into register d0, then loads another address into a1, calls another routine, tests a fixed memory location, and branches to the end if the test result is zero.
- `frag_move_andi_bsr_rts`: Short code fragment that copies an indexed entry from a1 to d0, then copies a fixed memory address to d1, then masks d1 with 2, then calls another routine, then returns.
- `bitwise_modify_memory`: Calculates an address based on input parameters, then performs bitwise operations on a memory region using either AND or OR operations depending on a condition.
- `format_and_copy_data`: Processes numerical data by dividing it into digits and converting them to ASCII characters, then copies character data from multiple source locations to a destination buffer using string copy operations.
- `copy_word_data`: Copies word values from one memory region to another with specific offset calculations, performing two separate copying operations with different loop counts and address increments.
- `update_hardware_registers`: Updates hardware register values with calculated bit patterns, copies data to a custom chip register, and performs a series of memory writes with specific offsets and incrementing addresses.
- `interleave_and_scale_data`: Processes word data by applying bitwise operations and scaling factors to combine multiple values, then stores the results in a memory buffer with specific addressing patterns.
- `frag_move_addi_lea_bsr_rts`: Short code fragment that copies a fixed memory address to d0, then adds 49 to d0, then copies d0 to a fixed memory address, then loads the address of a fixed memory address into a1, then calls another routine, then returns.
- `validate_header`: Checks the header signature from a data structure and validates its type, returning early if it's not a recognized format.
- `frag_move_moveq_add_bne_addx_roxr_dbra`: Short code fragment that copies a location in the code to d1, then loads 0 into d7, then adds d0 to d0, then branches if the last result was not zero, then copies the memory just below a0 (moving a0 down) to d0, then copies 255 to the condition codes, then adds d0 and the extend flag to d0, then rotates d7 right through the extend flag by 1, then loops back while counting down d1, then returns.
- `process_data_structure`: Processes a data structure by reading header information, copying data blocks, and executing function pointers from a table while managing counters and offsets.
- `looped_shift_with_table_lookup`: The routine repeatedly shifts an input value, using a table pointer when the value becomes zero, decrementing a short counter derived from an input register until it reaches minus one.
- `frag_moveq_bra_3`: Short code fragment that loads 0 into d6, then jumps to another point in the code.
- `frag_moveq_bra_4`: Short code fragment that loads 1 into d6, then jumps to another point in the code.
- `frag_moveq_bra_5`: Short code fragment that loads 2 into d6, then jumps to another point in the code.
- `frag_moveq_bra_6`: Short code fragment that loads 3 into d6, then jumps to another point in the code.
- `frag_moveq_bra_7`: Short code fragment that loads 4 into d6, then jumps to another point in the code.
- `frag_moveq_bra_8`: Short code fragment that loads 5 into d6, then jumps to another point in the code.
- `frag_moveq_bsr_add_bra`: Short code fragment that loads 1 into d1, then calls another routine, then loads 6 into d6, then adds d7 to d6, then jumps to another point in the code.
- `frag_moveq_bsr_add_bra_2`: Short code fragment that loads 1 into d1, then calls another routine, then loads 8 into d6, then adds d7 to d6, then jumps to another point in the code.
- `frag_moveq_bsr_add_bra_3`: Short code fragment that loads 2 into d1, then calls another routine, then loads 10 into d6, then adds d7 to d6, then jumps to another point in the code.
- `frag_moveq_bsr_add_bra_4`: Short code fragment that loads 3 into d1, then calls another routine, then loads 14 into d6, then adds d7 to d6, then jumps to another point in the code.
- `frag_moveq_bsr_add_bra_5`: Short code fragment that loads 5 into d1, then calls another routine, then loads 22 into d6, then adds d7 to d6, then jumps to another point in the code.
- `shift_and_copy_with_table`: Performs iterative shifting operations using lookup tables, copies data between memory locations with calculated offsets, and updates counters while maintaining circular buffer behavior.
- `frag_moveq_bsr_move_bra`: Short code fragment that loads 4 into d1, then calls another routine, then copies d7 to d6, then jumps to another point in the code.
- `shift_and_store_data`: Executes shifting operations on byte values, stores intermediate results in a memory buffer, and performs iterative copying operations with specific address calculations and counter decrements.
- `frag_moveq_bra_9`: Short code fragment that loads 1 into d6, then jumps to another point in the code.
- `frag_moveq_bsr_add_subq_andi_move`: Short code fragment that loads 3 into d1, then calls another routine, then loads 2 into d6, then adds d7 to d6, then subtracts 1 from d6, then calls another routine, then subtracts 1 from d4, then masks d4 with 16383, then copies d7 to an indexed entry from a2.
- `frag_addq_dbra_rts`: Short code fragment that adds 1 to d2, then loops back while counting down d6, then returns.
- `accumulate_and_store_shifted_values`: Accumulates shifted bit values from input data, stores the results in a memory buffer with calculated offsets, and performs multiple memory writes with specific addressing patterns.
- `shift_and_store_values`: Processes byte input data by performing iterative shifting operations, accumulates results in registers, and stores the final values in a memory buffer with calculated offsets.
- `process_bit_data`: Processes 16 bytes from a source pointer by shifting and accumulating values into four 16-bit registers, then stores the results at specific offsets in a destination buffer.
- `process_bit_data_v2`: Processes 16 bytes from a source pointer by shifting and accumulating values into three 16-bit registers, then stores the results at specific offsets in a destination buffer.
- `frag_lea_moveq_move_swap_addq_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads 7 into d7, then copies an address to d1, then copies d1 to the memory at an offset from a0, then swaps the halves of d1, then copies d1 to the memory at an offset from a0, then adds 8 to a0, then loops back while counting down d7, then returns.
- `set_color_registers`: Writes predefined color values to hardware color registers in two separate loops, with each loop performing a fixed number of iterations and writing different register values.
- `call_subroutines_if_flagged`: Checks if a global flag is non-zero, then calls a routine to reset audio channel state and another to reset global counters.
- `frag_move_clr_rts`: Short code fragment that copies 15 to the memory at an offset from a6, then clears a fixed memory address, then returns.
- `frag_movem_bsr_move_rts`: Short code fragment that saves registers on the stack, then calls another routine, then restores saved registers from the stack, then copies 1 to a fixed memory address, then returns.
- `initialize_audio`: Initializes audio system by clearing volume levels, setting specific hardware control values, and updating internal data structures related to audio channels and timing.
- `update_music_sequence`: Increments a counter and handles music sequence progression, including resetting counters and updating sequence data based on comparison results with threshold values.
- `update_audio_channels`: Processes audio channel updates by calling a helper routine four times, once for each of the four audio channels.
- `start_audio_dma`: Clears a global variable, performs delay loops, enables audio DMA channels, and loads four audio location pointers from a table.
- `parse_music_entry`: Extracts instrument number and period from a four-byte music data entry, looks up sample pointer and applies effects, then updates channel pointers and length.
- `update_position_value`: Processes movement commands and updates a position value with bounds checking and clamping, returning the updated position.
- `process_header_entries`: Processes a sequence of header entries by loading data from memory locations, checking for zero values, and storing processed data to another location while updating a counter.
- `validate_and_copy_data`: Tests a global variable for zero, then processes data from three memory locations by comparing values and copying bytes based on conditions, storing results in a fixed memory location.
- `initialize_and_process_2`: Initializes a counter with a fixed value, performs address calculations using a table, copies data between memory locations, and calls another routine to process the data.
- `initialize_sprite_data`: Writes fixed byte values to specific memory offsets relative to a calculated address, then performs bitwise NOT and AND operations on bytes from two source pointers before storing the results in a third location.
- `frag_move_add_lea_adda_rts`: Short code fragment that copies a fixed memory address to d0, then adds d0 to d0, then loads the address of a fixed memory address into a0, then adds d0 to a0, then loads the address of a fixed memory address into a1, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1) (2 times in a row), then returns.
- `frag_lea_bsr_rts_8`: Short code fragment that loads the address of a fixed memory address into a1, then calls another routine, then returns.
- `copy_struct_fields`: Copies data between two memory regions using looped longword transfers and sets specific memory locations to zero during the process.
