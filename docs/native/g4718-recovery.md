# Game 4718 (pipeline 3 (1993)(exterminators)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04718). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 18 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 62 of 62 named (0 author symbols), 62 of 62 described (17 by rule, 40 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 602 of 602 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 5524 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2999 of 2999 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 219953 |
| code | 13532 |
| data | 45163 |
| padding | 5000 |
| unused | 617472 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Pipeline 3 (1993)(Exterminators).adf:type | 1 | 426 | 2 |
| Pipeline 3 (1993)(Exterminators).adf:pipe | 2 | 2555 | 59 |
| Pipeline 3 (1993)(Exterminators).adf:bootblock | 1 | 18 | 1 |
| Intro.o (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- pipeline 3 (1993)(exterminators)/Pipeline 3 (1993)(Exterminators).adf:Data/Intro.o: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 jump target
- 1 never-loaded region(s), 5524 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- Intro.o (raw code file, load address unknown)

## Copy protection found

- `initialize_graphics_buffers` (pipe): Allocates multiple memory buffers for graphics data and reads corresponding files into them, then sets up DMA control registers and initializes display hardware. Kind: disk/track check. Pass path: The routine loads data from several files (Pipeline.GFX, Pipeline.SCO, etc.) and performs checks on them. If all reads succeed and the final dos.Open succeeds, it calls u1__sub_0_2A7E() and continues execution. The presence of multiple file.
- `close_library_handles` (pipe): Closes three library handles stored in global variables by calling the exec.CloseLibrary function for each handle. Kind: CloseLibrary handles check. Pass path: The routine closes library handles, suggesting a check that validates access to certain libraries or resources. If this check passes (i.e., all handles are valid and can be closed), the program continues normally. The fact that it's in a pi.

## Functions


### Pipeline 3 (1993)(Exterminators).adf:type

- `parse_type_command_arguments`: Parses command-line arguments to identify hex or number flags, validates that they are not used simultaneously, and handles error reporting for invalid options or file access failures.
- `format_output_string`: Formats strings and numbers into a character buffer using specific text patterns and hexadecimal characters.

### Pipeline 3 (1993)(Exterminators).adf:pipe

- `process_message`: Checks for a message in a system port and processes it by retrieving and replying to the message, while managing task synchronization through a global variable.
- `initialize_graphics_buffers`: Allocates multiple memory buffers for graphics data and reads corresponding files into them, then sets up DMA control registers and initializes display hardware.
- `wait_for_dma_ready`: Checks a hardware register bit repeatedly until it becomes clear, then returns immediately.
- `clear_color_table`: Clears a table of 32 color values starting at a fixed hardware address by writing zeros to each word location.
- `animate_color_fade`: Updates color values in a table by incrementing components based on comparisons with another table, then writes the updated values to the hardware color registers.
- `decrease_color_components`: Decrements color components in a table and copies the updated values to hardware color registers, performing multiple iterations of this process.
- `initialize_game_state`: Allocates memory for display buffers and initializes hardware registers for graphics rendering.
- `open_system_libraries`: Opens three system libraries using their names as strings and stores the returned library pointers in global variables.
- `close_library_handles`: Closes three library handles stored in global variables by calling the exec.CloseLibrary function for each handle.
- `toggle_memory_bits`: Toggles specific bits in a memory block by XORing with a constant value, iterating through a fixed number of locations.
- `fill_pattern_data`: Writes a sequence of word values to a memory location, including incrementing and decrementing values while maintaining a fixed pattern structure.
- `render_bonus_display`: Displays bonus score information on screen by copying character data to display buffers and updating score values in memory.
- `update_difficulty_selection`: Updates a global variable based on joystick input, cycles through difficulty levels, and writes text strings to display buffers.
- `setup_graphics_dma`: Allocates memory blocks for graphics data, initializes a display list with specific register values, and sets up DMA channels for graphics rendering.
- `cleanup_graphics`: Disables DMA channels, frees allocated memory blocks, and permits system operations after graphics cleanup.
- `process_grid_element`: Processes a grid element at specified coordinates, updating display buffer bits and handling bonus scoring when a cell is activated.
- `frag_addi_bra_move_addq_rts`: Short code fragment that adds 256 to a fixed memory address, then jumps to another point in the code, then copies a fixed memory address to a fixed memory address, then adds 1 to a fixed memory address, then returns.
- `frag_move_movea_adda_lea_bsr_rts`: Short code fragment that copies 1 to a fixed memory address, then copies 3 to a fixed memory address, then copies a fixed memory address to a1, then adds 10240 to a1, then adds 8321 to a1, then loads the address of a fixed memory address into a0, then calls another routine, then copies 1 to a fixed memory address, then copies 60 to a fixed memory address, then returns.
- `frag_move_movea_adda_lea_bsr_addi_rts`: Short code fragment that copies 1 to a fixed memory address, then copies 3 to a fixed memory address, then copies a fixed memory address to a1, then adds 10240 to a1, then adds 8321 to a1, then loads the address of a fixed memory address into a0, then calls another routine, then adds 500 to a fixed memory address, then returns.
- `frag_bsr_andi_cmp_bhi_lea_mulu_movea_js`: Short code fragment that calls another routine, then masks d0 with 15, then compares d0 with 12, then branches if the comparison was higher (unsigned), then loads the address of a fixed memory address into a0, then multiplies d0 by 4 (unsigned), then copies an indexed entry from a0 to a0, then calls the routine whose address is in a0, then returns.
- `set_time_stop_flags`: Sets global variables to enable time stop functionality with specific delay values.
- `update_game_state`: Updates game state variables including timers and counters, performs calculations based on game parameters, and triggers specific actions when conditions are met.
- `frag_movea_adda_move_mulu_clr_rts`: Short code fragment that copies a fixed memory address to a0, then adds 40960 to a0, then adds 4560 to a0, then copies a fixed memory address to d0, then multiplies d0 by 40 (unsigned), then adds d0 to a0, then clears the memory at a0, then returns.
- `process_input_and_collision`: Processes joystick input and keyboard events, handles collision detection, updates game objects, and manages game state transitions based on input and collision data.
- `mark_collision_points`: Writes specific word values to memory locations at calculated offsets, marking collision points with a fixed pattern structure.
- `clear_collision_data`: Clears word values in a memory block by writing zeros to sequential locations, iterating through a fixed number of entries.
- `process_joystick_input`: Reads joystick port 1 data and updates position counters based on button states, with boundary checks to prevent values from going out of range.
- `calculate_delay_value`: Determines a delay value based on game state variables and performs a loop that decrements this value until it reaches zero.
- `copy_tile_data`: Copies tile data from one memory location to another using multiplication factors for addressing, with nested loops to handle multiple rows and columns.
- `copy_pattern_data`: Copies pattern data from a source buffer to a destination using byte-based operations and address calculations with loop counters.
- `generate_random_seed`: Generates a pseudo-random seed value by combining several hardware register readings through bit shifts and exclusive OR operations.
- `frag_move_bsr_movea_adda_lea_rts`: Short code fragment that copies a fixed memory address to d2, then calls another routine, then copies a fixed memory address to a1, then adds 40960 to a1, then adds 7807 to a1, then loads the address of a fixed memory address into a0, then calls another routine, then returns.
- `format_number_string`: Converts numeric values to character representations in a string buffer, handling carry operations and padding with spaces when needed.
- `copy_glyph_data`: Copies character glyph data from a font table to a display buffer, processing multiple rows for each character.
- `frag_bsr_tst_beq_bra`: Short code fragment that calls another routine, then tests d0, then branches if the last result was zero, then jumps to another point in the code.
- `place_wall_object`: Places a wall object at a randomly selected position in the game grid, ensuring the position is empty before placing the object.
- `check_memory_pattern`: Counts occurrences of a specific byte value in a memory region and returns a counter if the count meets or exceeds a threshold.
- `return_constant`: Returns the constant 1 in d0 to the caller without any other effect.
- `frag_move_movea_adda_lea_bsr_addi_rts_2`: Short code fragment that copies 1 to a fixed memory address, then copies 3 to a fixed memory address, then copies a fixed memory address to a1, then adds 10240 to a1, then adds 8321 to a1, then loads the address of a fixed memory address into a0, then calls another routine, then adds 600 to a fixed memory address, then returns.
- `frag_bsr_tst_beq_bra_2`: Short code fragment that calls another routine, then tests d0, then branches if the last result was zero, then jumps to another point in the code.
- `create_black_hole`: Creates a black hole object at a randomly selected position in the game grid, clears surrounding area, and initializes the black hole's visual representation.
- `frag_move_movea_adda_lea_bsr_rts_2`: Short code fragment that copies 1 to a fixed memory address, then copies 3 to a fixed memory address, then copies a fixed memory address to a1, then adds 10240 to a1, then adds 8321 to a1, then loads the address of a fixed memory address into a0, then calls another routine, then returns.
- `set_clock_stop_flags`: Sets global variables to enable clock stop functionality with specific delay values.
- `set_oil_stop_flags`: Sets global variables to enable oil flow stop functionality with specific delay values.
- `reduce_score_if_needed`: Checks if the current score is sufficient to reduce it by 200 points, and performs the reduction if the condition is met.
- `frag_clr_rts`: Short code fragment that clears a fixed memory address, then returns.
- `frag_move_movea_adda_lea_bsr_rts_3`: Short code fragment that copies 1 to a fixed memory address, then copies 3 to a fixed memory address, then copies a fixed memory address to a1, then adds 10240 to a1, then adds 8321 to a1, then loads the address of a fixed memory address into a0, then calls another routine, then returns.
- `initialize_joystick_data`: Sets specific values in global variables related to joystick port status and timing, then calls another routine to process character glyph data.
- `frag_move_movea_adda_lea_bsr_rts_4`: Short code fragment that copies 1 to a fixed memory address, then copies 3 to a fixed memory address, then copies a fixed memory address to a1, then adds 10240 to a1, then adds 8321 to a1, then loads the address of a fixed memory address into a0, then calls another routine, then copies 1 to a fixed memory address, then returns.
- `clear_wall_objects`: Clears all wall objects from the game grid by setting their positions to empty values and updating visual representation.
- `update_tile_data`: Updates tile data based on conditions checked against a table, modifying values in two memory regions while iterating through a grid of 10 rows and 16 columns.
- `frag_move_movea_adda_lea_bsr_rts_5`: Short code fragment that copies 1 to a fixed memory address, then copies 3 to a fixed memory address, then copies a fixed memory address to a1, then adds 10240 to a1, then adds 8321 to a1, then loads the address of a fixed memory address into a0, then calls another routine, then copies 1792 to a fixed memory address, then returns.
- `frag_bsr_jmp`: Short code fragment that calls another routine, then jumps to another point in the code.
- `init_music_sample_table`: Searches a byte table for the smallest value, then builds a pointer table by linking zeroed entries to sample start addresses indexed from a second table, sets bit 1 of a global variable, clears several global variables, and returns.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `update_audio_dma_channels`: Updates audio DMA channel registers with values from memory locations and performs loop iterations to synchronize audio playback.
- `update_pipeline_data`: Processes pipeline configuration data from a structure, updates position values, and handles audio DMA control registers based on input flags.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `update_position`: Processes movement commands based on input data and updates position values in a structure.

### Pipeline 3 (1993)(Exterminators).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
