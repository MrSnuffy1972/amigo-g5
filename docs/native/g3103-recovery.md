# Game 3103 (bones brigade (1989)(genesis software)[t +2 atz]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03103). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (931824 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 18 raw-word lines |
| Functions named and described | pass, 134 of 134 named (0 author symbols), 134 of 134 described (26 by rule, 106 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2852 of 2852 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 1536 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5025 of 5025 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 12540 |
| code | 48788 |
| data | 316520 |
| padding | 504 |
| unused | 553472 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Bones Brigade (1989)(Genesis Software)[t +2 ATZ].adf:bootblock | 1 | 77 | 3 |
| logged disk load from Bones Brigade (1989)(Genesis Software)[t +2 ATZ].adf boot-block load | 1 | 92 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4833 | 127 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| Bones Brigade (1989)(Genesis Software)[t +2 ATZ].adf bytes 73728-74752 (code never loaded by a reference run) | 1 | 0 | 0 |
| Bones Brigade (1989)(Genesis Software)[t +2 ATZ].adf bytes 112640-113152 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- bones brigade (1989)(genesis software)[t +2 atz]/Bones Brigade (1989)(Genesis Software)[t +2 ATZ].adf: custom-format (non-DOS) disk, 675 non-empty blocks: matched against track-loaded segments
- bones brigade (1989)(genesis software)[t +2 atz]/Bones Brigade (1989)(Genesis Software)[t +2 ATZ].adf: the boot block reads 5632 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 2839 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-mixed): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-mixed): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- bones brigade (1989)(genesis software)[t +2 atz]/Bones Brigade (1989)(Genesis Software)[t +2 ATZ].adf: bytes 73728-74752 look like code no run loaded: recovered position-independently (load address unknown)
- bones brigade (1989)(genesis software)[t +2 atz]/Bones Brigade (1989)(Genesis Software)[t +2 ATZ].adf: bytes 112640-113152 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying code at an address (12 bytes change between runtime segment an address and runtime segment an address (play-rs-mixed)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (9 bytes change between runtime segment an address and runtime segment an address (play-rs-mixed)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 call target
- bones brigade (1989)(genesis software)[t +2 atz]/Bones Brigade (1989)(Genesis Software)[t +2 ATZ].adf: 3 logged loads cover 7168 bytes of the image
- bones brigade (1989)(genesis software)[t +2 atz]/Bones Brigade (1989)(Genesis Software)[t +2 ATZ].adf: 55 track-loaded blocks found in recovered runtime segments and classified through them
- bones brigade (1989)(genesis software)[t +2 atz]/Bones Brigade (1989)(Genesis Software)[t +2 ATZ].adf: blocks 150-152 (bytes 76800-78336) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.36)
- bones brigade (1989)(genesis software)[t +2 atz]/Bones Brigade (1989)(Genesis Software)[t +2 ATZ].adf: blocks 276-281 (bytes 141312-144384) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.13)
- bones brigade (1989)(genesis software)[t +2 atz]/Bones Brigade (1989)(Genesis Software)[t +2 ATZ].adf: blocks 322-327 (bytes 164864-167936) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.11)
- bones brigade (1989)(genesis software)[t +2 atz]/Bones Brigade (1989)(Genesis Software)[t +2 ATZ].adf: blocks 335-343 (bytes 171520-176128) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.69)
- bones brigade (1989)(genesis software)[t +2 atz]/Bones Brigade (1989)(Genesis Software)[t +2 ATZ].adf: blocks 366-371 (bytes 187392-190464) outside the filesystem hold data, not code (opcode density 0.004, entropy 2.51)
- bones brigade (1989)(genesis software)[t +2 atz]/Bones Brigade (1989)(Genesis Software)[t +2 ATZ].adf: blocks 375-420 (bytes 192000-215552) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.68)
- bones brigade (1989)(genesis software)[t +2 atz]/Bones Brigade (1989)(Genesis Software)[t +2 ATZ].adf: blocks 477-508 (bytes 244224-260608) outside the filesystem hold data, not code (opcode density 0.004, entropy 2.95)
- bones brigade (1989)(genesis software)[t +2 atz]/Bones Brigade (1989)(Genesis Software)[t +2 ATZ].adf: blocks 668-680 (bytes 342016-348672) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.92)
- bones brigade (1989)(genesis software)[t +2 atz]/Bones Brigade (1989)(Genesis Software)[t +2 ATZ].adf: blocks 682-787 (bytes 349184-403456) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.39)
- bones brigade (1989)(genesis software)[t +2 atz]/Bones Brigade (1989)(Genesis Software)[t +2 ATZ].adf: blocks 789-916 (bytes 403968-469504) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.16)
- bones brigade (1989)(genesis software)[t +2 atz]/Bones Brigade (1989)(Genesis Software)[t +2 ATZ].adf: blocks 924-938 (bytes 473088-480768) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.78)
- bones brigade (1989)(genesis software)[t +2 atz]/Bones Brigade (1989)(Genesis Software)[t +2 ATZ].adf: blocks 940-970 (bytes 481280-497152) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.34)
- bones brigade (1989)(genesis software)[t +2 atz]/Bones Brigade (1989)(Genesis Software)[t +2 ATZ].adf: blocks 1100-1187 (bytes 563200-608256) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.97)
- bones brigade (1989)(genesis software)[t +2 atz]/Bones Brigade (1989)(Genesis Software)[t +2 ATZ].adf: blocks 1398-1398 (bytes 715776-716288) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.69)
- bones brigade (1989)(genesis software)[t +2 atz]/Bones Brigade (1989)(Genesis Software)[t +2 ATZ].adf: blocks 1429-1429 (bytes 731648-732160) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.93)
- bones brigade (1989)(genesis software)[t +2 atz]/Bones Brigade (1989)(Genesis Software)[t +2 ATZ].adf: blocks 1436-1436 (bytes 735232-735744) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.23)
- bones brigade (1989)(genesis software)[t +2 atz]/Bones Brigade (1989)(Genesis Software)[t +2 ATZ].adf: blocks 1447-1447 (bytes 740864-741376) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.95)
- 2 never-loaded region(s), 1536 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 58880 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Bones Brigade (1989)(Genesis Software)[t +2 ATZ].adf:bootblock

- `frag_bra_lea_move_dbra_jmp`: Short code fragment that jumps to another point in the code, then loads the address of a location in the code into a2, then copies 358 to d0, then copies the memory at a2 (advancing a2) to the memory at a3 (advancing a3), then loops back while counting down d0, then jumps to another point in the code.
- `initialize_video_registers_2`: Writes zero values to multiple video hardware registers including display control and color settings, then calls a library routine to configure video modes.
- `initialize_video_registers`: Writes zero values to multiple video hardware registers including display control and color settings, then calls a library routine to configure display modes and sets up various register values for different display states.

### logged disk load from Bones Brigade (1989)(Genesis Software)[t +2 ATZ].adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_game_state_4`: Updates global game state variables based on input conditions and manages counters for different game phases.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `wait_for_raster_position`: Waits for the display beam to reach a specific raster line by continuously reading hardware registers until the condition is met, then returns with preserved register values.
- `setup_blitter_dma`: Configures blitter DMA registers for graphics operations by setting up address pointers, control words, and size parameters, while managing frame counters and performing division operations to calculate display data offsets.
- `check_conditions_and_branch`: Checks two word values at specific memory locations for zero, branches to different offsets if either is zero, then checks a byte value and branches if it equals 255.
- `process_audio_dma`: Configures audio DMA channels by setting up register values, managing counters, and updating audio volume registers based on game state conditions.
- `update_dma_parameters`: Updates DMA control registers and memory locations related to graphics rendering, performs coordinate calculations, and manages a counter that controls when certain operations are executed.
- `setup_bitblt_parameters`: Configures bit block transfer parameters by calculating memory addresses based on input coordinates, setting up source and destination pointers for graphics operations, and configuring transfer sizes and control registers.
- `clear_game_data_blocks`: Clears a large block of memory locations starting at an offset from a pointer, writing zero values to 140 consecutive 32-bit words, and then writes a final zero to the preceding location before updating the pointer to point to the next block.
- `update_game_state_2`: Compares and updates word values in two memory locations by incrementing them based on bit comparisons with corresponding values from another location, storing a flag indicating changes.
- `initialize_data_blocks`: Initializes data blocks by copying values from global tables to memory locations with specific offsets, updating counters and performing arithmetic operations on addresses.
- `set_memory_values`: Sets integer values at specific memory offsets in a table, incrementing the value by seven each iteration while processing a fixed number of entries.
- `copy_and_update_data`: Copies data from a source location to memory table entries, updating integer values at specific offsets with an increment of fourteen per iteration.
- `populate_table_with_data`: Populates memory table entries by copying data from global variables and setting integer values with an increment of eight per iteration.
- `fill_table_with_values`: Fills memory table entries with data from global variables and sets integer values with an increment of five per iteration.
- `initialize_table_entries`: Initializes memory table entries by copying data from global variables, setting a specific offset value to a global pointer, and updating integer values with an increment of ten per iteration.
- `copy_memory_blocks`: Copies data blocks from a source location to a destination table, handling multiple iterations based on input count, with each iteration copying 51 words of data and ending with a termination marker.
- `copy_data_blocks`: Copies sequential data blocks from an input pointer to a fixed table location, processing multiple iterations based on input count where each iteration copies 51 words of data and terminates with a sentinel value.
- `process_data_with_or_operations`: Performs bitwise OR operations on a byte value with a series of constants, copying data blocks from an input pointer to a table location while processing multiple iterations based on input count.
- `copy_data_with_looping`: Copies memory blocks from source to destination table using nested loops, handling multiple iterations where each iteration copies 51 words of data and terminates with a termination marker.
- `frag_movem_move_bsr_rts`: Short code fragment that saves registers on the stack, then copies a fixed memory address to d0, then copies 0 to d1, then calls another routine, then restores saved registers from the stack, then returns.
- `compute_pattern_offsets`: Computes offsets into a pattern table based on division operations with constants, calls another routine with computed parameters for each division step.
- `process_coordinate_bits`: Processes bit patterns at memory locations relative to input coordinates, sets flags based on bit test results and returns specific values depending on coordinate conditions.
- `check_bit_positions`: Examines bit patterns at various memory locations relative to input coordinates, setting flags based on bit test results and returning specific values indicating which bits were set or cleared.
- `update_game_entity`: Updates game entity state based on conditional checks and modifies global variables, then returns immediately.
- `update_structure_field`: Stores a 32-bit value from a register into a field at offset 16 of a structure pointed to by another register, then returns immediately without additional processing.
- `update_game_object`: Updates game object state by modifying global variables and calling another routine multiple times based on conditional checks.
- `update_pattern_data`: Increments a counter in a global variable and processes pattern data through a loop that calls another routine with different parameters.
- `frag_lea_move_adda_bsr_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies a fixed memory address to d0, then adds d0 to a0, then copies 13 to d0, then copies 1 to d1, then calls another routine, then returns.
- `manage_game_state`: Updates a global counter and manages game state by checking conditions and calling routines to handle different game events.
- `check_input_and_reset`: Checks keyboard input and resets game state variables when specific conditions are met, then clears multiple memory blocks and initializes display registers.
- `decrement_and_branch`: Decrements a value in a global variable and branches if the result is zero, otherwise calls another routine.
- `initialize_game_state`: Reads a value from a global variable, clears a word-sized memory location, and based on the read value, sets another global variable to specific byte values while calling other routines, then returns.
- `process_object_data`: Increments a global counter and processes object data through a loop that checks conditions and updates object properties.
- `decrement_and_check_bits`: Processes a series of memory locations by decrementing bit values in groups, checking for zero results and updating flags accordingly, repeating this process until all bits are processed or a timeout occurs.
- `compare_and_increment_bits`: Compares bit values between two memory locations and increments source values when differences are detected, processing multiple iterations while updating flags and repeating until all comparisons are complete.
- `update_sprite_animation`: Updates sprite animation data by modifying position coordinates and texture mapping values based on frame counters, then copies updated sprite properties to a rendering buffer.
- `update_game_state_3`: Updates game state variables based on joystick input and performs conditional operations including decrementing counters and setting flags.
- `increment_and_process`: Increments a global counter and when it reaches ten, clears the counter and processes a table of structures, modifying values based on bit tests and shifts before returning.
- `update_animation_state`: Updates animation state for a sequence of objects, modifying counters and flags while copying object data between memory locations.
- `update_animation_frame`: Updates animation frame data by copying structure fields and modifying counters, then performs collision checks and state restoration before returning.
- `frag_move_sub_rts`: Short code fragment that copies d0 to the stack, then copies a fixed memory address to d0, then subtracts d0 from a fixed memory address, then copies the top of the stack to d0, then returns.
- `update_bone_animation_2`: Updates bone animation state by modifying counters and flags while copying object data between memory locations, then performs collision checks and state restoration before returning.
- `initialize_pattern_data`: Initializes pattern data by copying values from a table, updating global variables, and performing arithmetic operations on pointers before branching to a label.
- `restore_state`: Restores three 32-bit values from global memory to a structure and calls another routine before returning.
- `check_collision`: Compares coordinates and dimensions of two structures with bitwise operations and sets a flag if collision conditions are met.
- `validate_bounds`: Checks if certain coordinate values fall within specified ranges using arithmetic comparisons and sets a flag if all conditions pass.
- `update_game_object_state`: Updates game object state variables including position, timer, and animation frame counters, then performs conditional arithmetic operations on movement values before returning.
- `initialize_render_data_3`: Sets up initial values for rendering data by copying word-sized elements from one memory region to another in a loop, then initializes specific memory locations with the value four.
- `update_table_entries`: Writes fixed 16-bit values to specific offsets in a table and updates a pointer through multiple iterations.
- `frag_lea_clr_rts`: Short code fragment that loads the address of a fixed memory address into a0, then clears the memory at an offset from a0 (8 times in a row), then returns.
- `clear_memory_block`: Calls another routine and then clears ten 16-bit words at fixed offsets from a memory address with zero values.
- `initialize_render_data`: Initializes rendering data based on a global state value, setting coordinates, flags, and texture pointers for multiple graphical elements.
- `update_render_positions`: Updates rendering coordinates and flags for multiple graphical elements, decrementing position values and managing a global counter, while clearing data when thresholds are exceeded.
- `increment_cycle_counter`: Increments a cycle counter in a global variable and resets it to zero when it reaches two, then updates multiple memory locations with calculated values based on table data.
- `initialize_vertex_data`: Initializes vertex data by loading a fixed address into a register, clearing memory at specific offsets, and storing sequential 32-bit values to multiple memory locations with incrementing offsets.
- `copy_vertex_data`: Copies vertex data from a source table to four destination buffers, processing 64 vertices in a loop with incrementing pointers.
- `calculate_render_offsets`: Calculates and stores four memory offsets based on a global variable's bit flags and other values, writing results to four memory locations.
- `initialize_vertex_data_2`: Initializes vertex data structures by storing specific 32-bit values at multiple memory locations pointed to by adjusted pointers, and performs arithmetic operations on global variables before storing the results.
- `compute_vertex_addresses`: Computes vertex addresses by multiplying a table index with 1024, then stores the resulting low and high words to multiple memory locations.
- `store_vertex_coordinates`: Stores low and high word values from four registers to specific memory offsets in a data structure using swap operations.
- `copy_texture_data`: Copies texture data from different source tables based on a global variable's value, processing 47 vertices with incrementing pointers.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `check_conditions_and_update`: Evaluates multiple conditional values from memory locations and updates global variables based on comparison results, then calls another routine with specific parameters.
- `update_animation_frame_2`: Increments a counter in a global variable and updates multiple memory locations with data from a table, resetting the counter to zero when it reaches -1.
- `frag_lea_move_rts`: Short code fragment that loads the address of a fixed memory address into a1, then copies 15 to the memory at an offset from a1, then copies 9 to the memory at an offset from a1, then copies 5 to the memory at an offset from a1, then copies 15 to the memory at an offset from a1, then copies 9 to the memory at an offset from a1, then copies 5 to the memory at an offset from a1, then returns.
- `handle_input_state`: Reads a byte from the keyboard serial register, checks if bit 0 is set, and updates global variables based on the byte's value.
- `update_bone_animation`: Updates bone animation state by checking joystick input and global flags, then modifies animation counters and clears input flags.
- `update_animation_values`: Updates animation state values by shifting and subtracting from memory locations, then performs conditional updates to counters and memory addresses.
- `handle_joystick_input`: Processes joystick input by checking button states and updating counter values based on movement direction and threshold comparisons.
- `process_joystick_input`: Reads joystick data from a hardware register and compares it against fixed values to determine which input actions to process, clearing certain game state variables when specific conditions are met.
- `frag_move_cmp_beq_addi_bset_rts`: Short code fragment that copies a fixed memory address to d0, then compares d0 with a fixed memory address, then branches if the last result was zero, then adds 1 to a fixed memory address, then sets bit 0 of a fixed memory address, then returns.
- `frag_move_cmp_beq_subi_bset_rts`: Short code fragment that copies a fixed memory address to d0, then compares d0 with a fixed memory address, then branches if the last result was zero, then subtracts 1 from a fixed memory address, then sets bit 1 of a fixed memory address, then returns.
- `frag_move_cmp_beq_addi_bset_rts_2`: Short code fragment that copies a fixed memory address to d0, then compares d0 with a fixed memory address, then branches if the last result was zero, then adds 1 to a fixed memory address, then sets bit 2 of a fixed memory address, then returns.
- `frag_move_cmp_beq_subi_bset_rts_2`: Short code fragment that copies a fixed memory address to d0, then compares d0 with a fixed memory address, then branches if the last result was zero, then subtracts 1 from a fixed memory address, then sets bit 3 of a fixed memory address, then returns.
- `update_game_state`: Updates game state variables by incrementing counters, performing conditional arithmetic operations, and modifying memory locations based on comparison results.
- `frag_lea_move_rts_2`: Short code fragment that loads the address of a fixed memory address into a0, then copies a fixed memory address to the memory at an offset from a0 (2 times in a row), then returns.
- `frag_cmpi_beq_bne_bsr_rts`: Short code fragment that compares a fixed memory address with 2, then branches if the last result was zero, then compares a fixed memory address with 0, then branches if the last result was not zero, then calls another routine, then returns, then calls another routine, then returns, then calls another routine, then returns.
- `initialize_game_state_2`: Sets up initial game parameters by writing specific longword values to several memory locations, including a global variable at offset 727C, and initializes rendering data through multiple subroutine calls.
- `initialize_game_state_3`: Sets up initial game parameters by writing predefined values to multiple memory locations, calls several initialization routines, and configures hardware registers for display operations.
- `handle_game_state`: Processes game state based on a word value read from a global variable, performing different actions depending on the value and calling other routines for specific operations.
- `update_pattern_sequence`: Increments a counter and performs conditional operations based on the counter's value, including resetting counters and updating memory pointers in a pattern sequence loop.
- `initialize_game_objects`: Initializes game object states and positions based on conditional checks, then updates global counters and clears memory blocks before returning.
- `frag_cmpi_beq_subi_bne_lea_move_rts`: Short code fragment that compares a fixed memory address with 0, then branches if the last result was zero, then subtracts 1 from a fixed memory address, then compares a fixed memory address with 1, then branches if the last result was not zero, then loads the address of a fixed memory address into a0, then copies an address to the memory at an offset from a0, then returns.
- `frag_movea_cmpi_beq_bsr_rts`: Short code fragment that copies a fixed memory address to a0, then compares the memory at an offset from a0 with 0, then branches if the last result was zero, then calls another routine, then returns.
- `process_data_loop`: Iterates through a data structure starting at a fixed memory address, checking for zero values and calling another routine when conditions are met, continuing until a termination marker is found.
- `process_data_loop_2`: Processes a data structure in a similar manner to the previous routine, iterating through memory locations with fixed offsets and calling another routine when non-zero values are encountered.
- `update_matrix_values`: Copies data from a global memory location to multiple destination addresses, performs three subroutine calls, and updates several global variables with computed values.
- `calculate_math_values`: Performs mathematical calculations involving bit shifts, subtractions, additions, and rotations to update two global variables with computed values.
- `process_matrix_data`: Processes matrix data by performing bit operations, arithmetic calculations, and memory updates to store computed values in global variables.
- `configure_blitter`: Sets up multiple blitter registers with values from global memory locations and calculates a combined value for BLTCON0 register before returning.
- `initialize_coordinates`: Writes fixed coordinate values to specific memory locations in a table, updating position data for objects.
- `manage_sprite_counter`: Updates a sprite counter and associated memory values based on comparison checks, performs arithmetic operations, and writes computed values to specific memory locations.
- `check_input_conditions`: Reads keyboard data from a hardware register and updates global variables based on specific key presses, then waits for certain conditions before returning.
- `copy_pattern_data`: Copies 19 words from one memory location to another using indexed addressing and loop counter to populate pattern data.
- `process_vertex_data`: Processes vertex data by calculating table addresses using shift and multiply operations, reads byte values from a source pointer, and copies word values between tables based on conditional logic.
- `frag_lea_move_clr_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies 549 to d0, then clears the memory at a0 (advancing a0), then loops back while counting down d0, then returns.
- `initialize_game_variables`: Initializes various game variables by writing predefined values to multiple memory locations, including clearing memory blocks and setting up data structures.
- `clear_structure_data`: Clears several longword values in a structure and then iterates through a memory region to clear all longwords until reaching a specific end marker.
- `open_graphics_library`: Opens the graphics library, performs DMA setup, and initializes various hardware registers with specific values for display operations.
- `initialize_rendering_data`: Initializes various rendering parameters by writing fixed values to multiple memory locations, clears memory blocks, and calls several other routines to set up graphics data structures.
- `copy_vertex_data_1`: Copies 28 words of vertex data from multiple source tables to a destination table using indexed addressing, performs bit manipulation operations on values, and manages loop counters for processing.
- `copy_vertex_data_2`: Copies 28 words from one memory location to multiple destination tables using indexed addressing and loop counter.
- `update_vertex_state`: Manages vertex state by comparing and updating word values between source and destination tables, performs bit operations on values, and handles loop counters for processing multiple data elements.
- `copy_vertex_data_2_2`: Copies 4 long words from a source pointer to a destination table, performs bit manipulation operations on word values in a loop, and manages loop counters for processing multiple data elements.
- `decrement_counters`: Decrements specific bit fields in a table based on their current values, then calls another routine with updated data and performs a delay loop before repeating the process for sixteen iterations.
- `wait_for_blitter_idle`: Waits until the blitter is idle by polling a bit in the DMA control register, then configures multiple blitter registers with fixed values.
- `frag_lea_clr_rts_2`: Short code fragment that loads the address of a fixed memory address into a1, then clears the memory at an offset from a1 (8 times in a row), then returns.
- `initialize_vertex_data_3`: Initializes vertex data by clearing four long words in a destination table, compares word values between source and destination tables, performs bit operations on values, and manages loop counters for processing multiple data elements.
- `copy_vertex_data_b`: Copies 8 words from source memory to a specific table location using indexed addressing and incrementing pointers.
- `copy_vertex_data_a`: Copies 8 words from source memory to another specific table location using indexed addressing and incrementing pointers.
- `frag_move_sub_rts_2`: Short code fragment that copies a fixed memory address to d7, then copies the memory at an offset from a5 to a fixed memory address (2 times in a row), then subtracts d7 from a fixed memory address, then copies a fixed memory address to the memory at an offset from a5 (2 times in a row), then returns.
- `initialize_display_buffer`: Initializes a display buffer with a fixed longword value, then copies word values from the buffer to multiple locations in a coordinate table.
- `update_sprite_data_3`: Updates sprite position and rendering data based on frame counters and global state, then copies texture coordinates from a lookup table to sprite display buffers.
- `initialize_render_data_4`: Initializes rendering data by clearing counters and setting up graphics parameters when a condition is met, then updates multiple memory locations with calculated values based on other variables.
- `initialize_render_data_2`: Initializes a rendering data structure by setting up memory pointers, writing fixed values to specific offsets, and updating counters in a loop that processes 8 iterations.
- `frag_move_movea_adda_jsr_rts`: Short code fragment that copies 128 to a fixed memory address, then copies a fixed memory address to a0, then adds 50 to a0, then copies a fixed memory address to the memory at a0, then copies 32896 to a fixed memory address, then copies a fixed memory address to a6, then calls another routine, then returns.
- `frag_cmpi_bne_beq_bsr_rts`: Short code fragment that compares a fixed memory address with 0, then branches if the last result was not zero, then compares a fixed memory address with 0, then branches if the last result was zero, then calls another routine (2 times in a row), then returns.
- `update_collision_values`: Updates collision-related values based on comparison checks between computed and stored data, modifying global variables and performing arithmetic operations to adjust positioning or state information.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `copy_pixel_data`: Copies pixel data from a source buffer to multiple destination locations using indexed addressing and bit manipulation calculations, processing 4 iterations of data transfer operations.
- `update_sprite_data`: Updates sprite data by reading from a lookup table based on a counter value and writing to multiple memory locations.
- `clear_memory_blocks`: Clears memory blocks at specific offsets within a data structure, then calls another routine multiple times while advancing a pointer through a loop.
- `frag_move_dbra_rts`: Short code fragment that copies 24576 to d1, then loops back while counting down d1, then returns.
- `update_sprite_data_2`: Updates sprite position and attribute data by copying values from referenced structures, then performs a series of word assignments in a loop with decrementing counter.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_registers`: Initializes hardware registers based on bit test conditions, writes constant values to memory locations, and jumps to another routine after setting up system state.
