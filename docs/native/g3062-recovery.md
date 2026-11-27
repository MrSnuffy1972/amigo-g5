# Game 3062 (Challenge of the Matrix, The): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03062). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901520 top-level bytes in 20 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 46 raw-word lines |
| Functions named and described | pass, 403 of 403 named (62 author symbols), 403 of 403 described (172 by rule, 175 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1561 of 1561 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6920 of 6920 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 143002 |
| code | 10898 |
| data | 198109 |
| padding | 4743 |
| unused | 544768 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| picend | 1 | 380 | 16 |
| Mouseclick | 1 | 3 | 1 |
| picshow | 1 | 491 | 15 |
| NOISEPLAYER | 2 | 233 | 8 |
| Challenge of the Matrix, The.adf:Matrix | 1 | 1283 | 62 |
| Challenge of the Matrix, The.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 64 | 1 |
| NOISEPLAYER#unpacked | 7 | 4605 | 297 |

## Cases handled

- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 48 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Challenge of the Matrix, The/Challenge of the Matrix, The.adf:c/NOISEPLAYER: crunched executable decrunched by XFDMaster (PowerPacker 3.0 (xfd), 17752 -> 31820 bytes), 10 of 13 windows confirmed in reference RAM: recovered as a hunk executable
- Challenge of the Matrix, The/Challenge of the Matrix, The.adf:c/NOISEPLAYER: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_audio_player` (NOISEPLAYER#unpacked): Initializes audio playback by setting up interrupt handlers and managing memory blocks, then processes music file formats based on detected headers. Kind: disk/track check. Pass path: The check passes if `u8__d1_14BC` is non-zero after the call, and the execution continues to the branch where `u8__sub_1_0B6C()` is called, indicating a valid disk/track access or sync pattern..
- `initialize_audio_state` (NOISEPLAYER#unpacked): Clears and initializes audio-related variables and registers, copies data from a table indexed by a value in the structure, updates global state based on comparison results, and sets bits in the CIAA_PRA register. Kind: disk hardware check. Pass path: The routine initializes audio state and sets up ADKCON register, indicating a disk drive or hardware verification check. The call site at sub_1_1534 with bra.b an address followed by tst.w an address(a6) and beq.b an address suggests a conditional branch bas.

## Functions


### picend

- `frag_bra`: Short code fragment that jumps to another point in the code.
- `write_nonempty_string_to_output`: When its nonzero argument is set, measures a null-terminated string and writes it through the DOS Write routine to the saved output handle, skipping empty strings.
- `process_file_operations`: Processes file operations including locking, examining, and reading with error handling for file access and memory allocation.
- `frag_move_movea_jsr_bsr_suba_bra`: Short code fragment that copies d6 to d1, then copies a6 to the stack, then copies a fixed memory address to a6, then calls another routine, then copies the top of the stack to a6, then copies d4 to a0, then calls another routine, then subtracts a0 from a0, then jumps to another point in the code.
- `allocate_block_with_size_header`: Allocates a memory block through the system allocator and, if the allocation succeeds, writes the first argument plus four into the block's first longword.
- `free_block_if_nonnull`: Frees the memory block at the pointer argument through the system FreeMem routine, doing nothing when the pointer is null.
- `terminate_string_at_length`: When a task field is set, saves the buffer pointer in a global slot and writes a null terminator at the position given by the length argument.
- `parse_next_argument_token`: Advances a global argument pointer past spaces, then extracts the next token, accepting quoted strings and stopping at a space or comma.
- `frag_movem_clr_move_movea_jsr_btst_rts`: Short code fragment that saves registers on the stack, then clears d0, then clears d1, then copies a6 to the stack, then copies a fixed memory address to a6, then calls another routine, then copies the top of the stack to a6, then tests bit 12 of d0, then restores saved registers from the stack, then returns.
- `measure_c_string_length`: Counts the characters in a null-terminated string and returns the count, not including the terminator.
- `execute_graphics_sequence`: Calculates two 16-bit values by adding d0 and d1 and subtracting one, then calls another routine twice using these values, calls a third routine, and finally calls the third routine again using a value derived from the first calculation.
- `frag_movem_move_bsr_addq_rts`: Short code fragment that saves registers on the stack, then copies d0 to d2, then calls another routine, then adds 1 to d0, then adds 1 to d2, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_movem_move_bsr_rts`: Short code fragment that saves registers on the stack, then copies d1 to d3, then calls another routine, then restores saved registers from the stack, then returns.
- `execute_graphics_calls`: Calls a routine via a6 using coordinates from d0 and d1, then calls another routine via a6 using coordinates from d2 and d3.
- `initialize_system_resources`: Initializes system resources by opening libraries, allocating memory blocks, and setting up global pointers for various system components including graphics and input/output operations.
- `handle_screen_close`: Handles screen closing operations by finding and removing ports, then calling cleanup routines based on port existence.

### Mouseclick

- `program`: Short code fragment that tests bit 6 of a fixed memory address, then branches if the last result was not zero, then returns.

### picshow

- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `write_nonempty_string_to_output_2`: When its nonzero argument is set, measures a null-terminated string and writes it through the DOS Write routine to the saved output handle, skipping empty strings.
- `validate_and_read_file`: Validates file access permissions and reads file contents into allocated memory blocks with proper error checking.
- `allocate_block_with_size_header_2`: Allocates a memory block through the system allocator and, if the allocation succeeds, writes the first argument plus four into the block's first longword.
- `free_block_if_nonnull_2`: Frees the memory block at the pointer argument through the system FreeMem routine, doing nothing when the pointer is null.
- `terminate_string_at_length_2`: When a task field is set, saves the buffer pointer in a global slot and writes a null terminator at the position given by the length argument.
- `parse_next_argument_token_2`: Advances a global argument pointer past spaces, then extracts the next token, accepting quoted strings and stopping at a space or comma.
- `frag_movem_clr_move_movea_jsr_btst_rts_2`: Short code fragment that saves registers on the stack, then clears d0, then clears d1, then copies a6 to the stack, then copies a fixed memory address to a6, then calls another routine, then copies the top of the stack to a6, then tests bit 12 of d0, then restores saved registers from the stack, then returns.
- `measure_c_string_length_2`: Counts the characters in a null-terminated string and returns the count, not including the terminator.
- `execute_graphics_sequence_2`: Calculates two 16-bit values by adding d0 and d1 and subtracting one, then calls another routine twice using these values, calls a third routine, and finally calls the third routine again using a value derived from the first calculation.
- `frag_movem_move_bsr_addq_rts_2`: Short code fragment that saves registers on the stack, then copies d0 to d2, then calls another routine, then adds 1 to d0, then adds 1 to d2, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_movem_move_bsr_rts_2`: Short code fragment that saves registers on the stack, then copies d1 to d3, then calls another routine, then restores saved registers from the stack, then returns.
- `execute_graphics_calls_2`: Calls a routine via a6 using coordinates from d0 and d1, then calls another routine via a6 using coordinates from d2 and d3.
- `render_rectangle_fill`: Calls the graphics library to fill a rectangular area on a raster port using coordinates and dimensions passed in registers, then returns the original coordinate values.
- `parse_screen_header`: Parses the header of a screen file, extracting dimensions and color palette information, then allocates memory for the image data and copies pixel data into it.

### NOISEPLAYER

- `process_hunk_data`: Processes Amiga hunk-type data structures, allocating memory for each entry and copying data into the allocated blocks while handling relocation and loading operations.
- `decrement_counter`: Decrements a counter in register d1 until it reaches zero, then returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `process_data_shifts`: Processes data through bitwise right shifts and rotates, updating registers and memory locations based on loop counters and conditional branches.
- `frag_lsr_beq_rts_move_roxr`: Short code fragment that shifts d5 right by d4, then branches if the last result was zero, then returns, then copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by d4, then returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (sub.w) and then falls through into the code that follows, with no return of its own.
- `shift_and_rotate`: Performs bitwise right shifts and rotates on data in registers d4 and d5, updating d1 with rotated values through a loop that decrements d0.
- `execute_interrupt_handler`: Executes an interrupt handler that moves a word from memory to a register, triggers a trap, and performs a bitwise OR operation.

### Challenge of the Matrix, The.adf:Matrix

- `PROGRAM`: Short code fragment that calls another routine (3 times in a row).
- `GAMELOOP`: Executes a sequence of game initialization and update routines in a continuous loop until a condition is met, then jumps to a death handling routine.
- `frag_move_jsr_jmp`: Short code fragment that copies 15 to a fixed memory address, then calls another routine (2 times in a row), then jumps to another point in the code.
- `DELAY`: Short code fragment that subtracts 1 from a fixed memory address, then branches if the last result was not zero, then returns.
- `MINETIME`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then subtracts 1 from a fixed memory address, then branches if the last result was not zero, then calls another routine, then returns.
- `EXPLODE`: Executes a mine explosion routine by decrementing values at specific memory locations, copying tile data, and managing counters in a loop structure.
- `SUBCHECK`: Compares a byte from memory with several constants and calls another routine based on the comparison results, then returns.
- `CALCSPEED`: Short code fragment that copies 1600 to a fixed memory address, then tests a fixed memory address, then branches if the last result was zero, then subtracts 600 from a fixed memory address, then returns.
- `JEWELPTS`: Updates score and clears a tile, then calls multiple routines including one that handles century events and another that displays scores.
- `MINEIT`: Short code fragment that copies a fixed memory address to a1, then calls another routine (2 times in a row), then copies a fixed memory address to a0, then copies 0 to the memory at a0, then copies 1 to a fixed memory address, then calls another routine (2 times in a row), then copies 0 to a fixed memory address, then returns.
- `FIRECHECK`: Checks various conditions related to weapon type, mine status, player position, and movement flags before calling another routine to lay a mine.
- `LAYMINE`: Sets up DMA channel enable, initializes weapon type, checks a condition, and places a mine at a specific location with associated graphics and sound effects.
- `STDPTS`: Clears a tile, increments score by one point, and calls routines to handle score addition, display, and matrix updates.
- `REPTS`: Sets up graphics for a standard tile, updates player position, increments score, and calls routines to handle sound, score display, and matrix operations.
- `SUBMATRIX`: Decrements a tile counter and handles level completion logic including screen wiping, message display, and updating game state variables.
- `HOLE`: Short code fragment that copies 0 to a fixed memory address, then subtracts 1 from a fixed memory address, then branches if the last result was not zero, then copies 1 to a fixed memory address, then calls another routine, then returns.
- `ESCAPE`: Short code fragment that compares a fixed memory address with 117, then branches if the last result was not zero, then copies 1 to a fixed memory address, then returns.
- `DEAD`: Displays game over messages, manages input polling, and resets game state by jumping to the main game loop.
- `CENTURY`: Calls the ADDSCORE routine exactly ten times in sequence.
- `ADDSCORE`: Updates multi-digit score counters by incrementing values and resetting them when reaching threshold limits.
- `MOVEYOU`: Processes player movement commands by decrementing a counter, updating coordinates based on direction, and calling specific movement routines.
- `CHECKMOVE`: Validates player movement by checking the current state of a global flag, then compares the value at a specific address with multiple constants to determine the type of action to perform.
- `NULLOBJECT`: Short code fragment that copies 2292 to a fixed memory address, then copies 2192 to a fixed memory address, then copies 2232 to a fixed memory address, then copies 2212 to a fixed memory address, then copies 2252 to a fixed memory address, then returns.
- `MOVEUP`: Short code fragment that loads the address of a fixed memory address into a5, then copies the memory at an offset from a5 to d6, then branches if the last result was not zero, then subtracts 1 from a fixed memory address, then calls another routine, then returns.
- `MOVEDOWN`: Short code fragment that loads the address of a fixed memory address into a5, then copies the memory at an offset from a5 to d6, then branches if the last result was not zero, then adds 1 to a fixed memory address, then calls another routine, then returns.
- `MOVELEFT`: Short code fragment that loads the address of a fixed memory address into a5, then copies the memory at an offset from a5 to d6, then branches if the last result was not zero, then copies a fixed memory address to a0, then adds 1 to a0, then copies a fixed memory address to d0, then subtracts 1 from a fixed memory address, then calls another routine, then returns.
- `PRINTYOU`: Short code fragment that copies a fixed memory address to a0, then adds 1 to a0, then copies a fixed memory address to d0, then calls another routine (2 times in a row), then returns.
- `HORZSHOW`: Renders horizontal pixel data by shifting a word value, setting or clearing bits in memory locations, and restoring original values.
- `CHY`: Short code fragment that copies a fixed memory address to d0, then copies a fixed memory address to a0, then copies d0 to the memory at a0, then adds 16 to d0, then adds 2 to a0, then copies d0 to the memory at a0, then returns.
- `MOVERIGHT`: Short code fragment that loads the address of a fixed memory address into a5, then copies the memory at an offset from a5 to d6, then branches if the last result was not zero, then copies a fixed memory address to a0, then adds 1 to a0, then copies a fixed memory address to d0, then adds 1 to a fixed memory address, then calls another routine, then returns.
- `LETTER`: Converts character input to bitmap representation by indexing into an alphabet table and copying pixel data to output locations.
- `SENTC`: Short code fragment that calls another routine, then subtracts 8319 from a1, then adds 1 to a0, then tests the memory at a0, then branches if the last result was not zero, then returns.
- `DIGIT`: Renders digit bitmaps by copying pixel data from source locations to destination addresses with specific offset calculations.
- `SHOWSCORE`: Displays individual score digits by calling the DIGIT routine for each of six digit positions.
- `ENERBAR`: Displays an energy bar graphic based on the current energy level, clearing the display when energy reaches zero.
- `ENERSHOW`: Draws an energy bar by copying bitmap data from multiple graphic sources to different memory locations with calculated offsets.
- `CHAN1SAMP`: Short code fragment that loads the address of a fixed memory address into a5, then copies a fixed memory address to the memory at an offset from a5, then copies 2078 to the memory at an offset from a5, then copies a fixed memory address to the memory at an offset from a5, then copies 64 to the memory at an offset from a5, then copies 33282 to the memory at an offset from a5, then returns.
- `EATSAMP`: Configures audio registers for a sound sample using data from a global address and adds a fixed offset to determine the sample location.
- `FIRESAMP`: Configures audio registers for a fire sound sample using data from a global address and adds a fixed offset to determine the sample location.
- `CHINGSAMP`: Configures audio registers for a chime sound sample using data from a global address and adds a fixed offset to determine the sample location.
- `GONG`: Plays a gong sound by setting up audio registers for two channels with initial volume, then gradually decreases the volume in a loop while calling a delay routine between each step until volume reaches zero.
- `SOUNDOFF`: Short code fragment that loads the address of a fixed memory address into a5, then subtracts 1 from a fixed memory address, then branches if the last result was zero, then returns, then copies 1 to the memory at an offset from a5, then returns.
- `CHAN2SAMP`: Short code fragment that loads the address of a fixed memory address into a5, then copies a fixed memory address to the memory at an offset from a5, then copies 2078 to the memory at an offset from a5, then copies a fixed memory address to the memory at an offset from a5, then copies 64 to the memory at an offset from a5, then copies 33284 to the memory at an offset from a5, then returns.
- `CHAN3SAMP`: Short code fragment that loads the address of a fixed memory address into a5, then copies a fixed memory address to the memory at an offset from a5, then copies 2078 to the memory at an offset from a5, then copies a fixed memory address to the memory at an offset from a5, then copies 64 to the memory at an offset from a5, then copies 33288 to the memory at an offset from a5, then returns.
- `MUSIC`: Manages playback of three musical channels by decrementing delay counters and triggering sample setup routines when delays reach zero, advancing through musical sequences stored in voice tables.
- `TITLESCREEN`: Manages title screen display logic including tile cycling, timer handling, and input checking for game start.
- `TILECYCLE`: Cycles through different title screen tiles, updating a tile counter and handling special cases for specific tile values.
- `GOTITLE`: Displays various title screen messages and graphics by calling COPYTILE and SENTC routines multiple times.
- `FINDOBJECT`: Loads graphic pointers for a tile type into five global variables based on the value in the D0 register, with some tile types incrementing a tile counter.
- `ASSIGN`: Loads level data from a global table into a buffer, updates a pointer to the loaded data, and sets up bit map locations for the game.
- `COPYMATRIX`: Copies game matrix data using FINDOBJECT and COPYTILE routines, with delays between operations, then sets player position and DMA settings.
- `WIPE`: Clears the screen by copying objects through multiple iterations of COPYTILE calls while waiting for vertical blanking.
- `INITBITMAP`: Sets up bitmap and copper list memory, configures display registers including color table and bitplane control, and initializes hardware for graphics rendering.
- `COPYTILE`: Copies graphic data from five different source addresses to a destination buffer, processing multiple tile layers with specific offsets and byte counts.
- `INITVARS`: Sets initial values for various game state variables including energy, score, level, color, place, tiles, flag, timer, delays, collision flags, maximum value, points, mine address, and weapon type.
- `PAUSE`: Handles pause functionality by checking for a specific key press, displaying a pause message, and cycling through game states until the key is pressed again.
- `SAMPMEMORY`: Allocates memory for audio samples using a library function, then copies sample data from a predefined table to the allocated memory block.
- `FREESAMPMEM`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `SPRITEMEM`: Allocates memory for sprite data using a library function, then copies sprite data from a predefined table to the allocated memory block.
- `FREESPRITE`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `FREEBITMEM`: Frees previously allocated bit map and copper list memory blocks, updates hardware registers related to DMA control and coprocessor settings, and closes a graphics library handle.
- `CYCLE`: Increments or decrements a color value based on flags and boundary conditions, updating both a global variable and a hardware color register.

### Challenge of the Matrix, The.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `sets_global_pointer_returns_minus_one`: Stores a constant into a global variable, removes 4 bytes from the stack, restores registers from memory, and returns the value -1 in d0.
- `frag_move_ori_movea`: Short code fragment that copies d0 to d2, then merges 0 into d0, then copies the memory at an offset from a5 to d0, then copies the memory at an offset from a5 to a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `manage_library_resource`: Manages library resources by performing several system calls including finding names, waiting for messages, opening libraries, and executing code via memory pointers.

### NOISEPLAYER#unpacked

- `initialize_noise_player_3`: Initializes a noise player component by setting up data structures, opening libraries, and managing resource allocation through system calls.
- `frag_bra_3`: Short code fragment that jumps to another point in the code.
- `process_message`: Loads a pointer from a table, calls a routine via a pointer in a6, and then conditionally calls several other library routines including one that uses a pointer in a4 and another that uses a pointer in a0.
- `process_input_data`: Analyzes character input data from a structure, performs comparisons against specific values, and updates global variables based on the results of those comparisons.
- `character_selection`: Compares a byte value from offset 2 of the structure in a0 with several constants, and sets a global variable based on which comparison succeeds.
- `frag_lea_bsr_rts`: Short code fragment that loads the address of a fixed memory address into a0, then calls another routine, then returns.
- `frag_move_dbra_rts`: Short code fragment that copies d0 to the stack, then copies 65534 to d0, then copies d0 to a fixed memory address, then loops back while counting down d0, then copies the top of the stack to d0, then returns.
- `frag_move_dbra_rts_2`: Short code fragment that copies d0 to the stack, then copies 65534 to d0, then copies d0 to a fixed memory address, then loops back while counting down d0, then copies the top of the stack to d0, then returns.
- `initialize_audio_player`: Initializes audio playback by setting up interrupt handlers and managing memory blocks, then processes music file formats based on detected headers.
- `manage_audio_resources`: Checks a global variable and manages audio-related memory blocks, updating flags and calling resource management routines based on conditional checks.
- `handle_input_dispatch`: Processes a global variable value through a series of conditional branches and calls different functions based on the variable's byte value, returning after each function call.
- `process_input_switch`: Checks a global variable against multiple byte values and branches to different code paths based on matches, with some cases sharing the same function call.
- `identify_file_format`: Compares the content of a global variable against various magic numbers and sets another global variable to different byte values based on matching conditions.
- `write_string_sequence`: Writes specific character sequences to memory locations pointed to by a global variable, calling another function twice and updating the global variable's content during execution.
- `initialize_globals`: Clears multiple global variables and sets a flag to zero, then enters a loop that processes a string buffer, checking for specific character sequences and updating pointers accordingly.
- `frag_move_addi_movea_jsr_tst_beq_rts`: Short code fragment that copies 65538 to d1, then copies a fixed memory address to d0, then adds 5100 to d0, then copies a fixed memory address to a6, then calls another routine, then tests d0, then branches if the last result was zero, then copies d0 to a fixed memory address, then returns.
- `free_memory_block`: Frees a memory block using a library function when a global variable is not zero, clears the variable after freeing, and adjusts it by adding or subtracting a fixed value.
- `frag_movem_bsr_rts`: Short code fragment that saves registers on the stack, then calls another routine, then restores saved registers from the stack, then returns.
- `update_color_register`: Updates a color register in hardware with values from memory locations, performing calculations on addresses and byte values while iterating through data structures.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (moveq) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs one instruction (subq.w) and then falls through into the code that follows, with no return of its own.
- `frag_moveq_lsr_roxl_subq_bne_move_dbra`: Short code fragment that loads 0 into d1, then shifts d5 right by 1, then rotates d1 left through the extend flag by 1, then subtracts 1 from d7, then branches if the last result was not zero, then copies d6 to d7, then copies the memory just below a0 (moving a0 down) to d5, then loops back while counting down d0, then returns.
- `count_string_length`: Counts the length of a null-terminated string by incrementing a counter until it reaches the null terminator, then calls another routine with the string address.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `handle_file_operations`: Sets a flag in a structure, calls a routine to open a file, and if successful, performs a series of checks and calls to close, seek, and read files before storing a result in a global variable.
- `read_file_and_update`: Calls a routine to open a file, and if successful, calls another routine to read data and a third routine to close the file, finally storing a value in a global variable.
- `allocate_and_initialize_port`: Saves registers to the stack, calls a routine to allocate memory, and if successful, populates a structure in a5 with various values including a pointer, a byte, and a word, then performs several library calls and branches.
- `manage_device_resources`: Checks a value at offset 28 of a structure in a1, modifies it, and performs several conditional calls to routines using a pointer from a5 and a pointer from a6.
- `frag_movem_movea_moveq_move_bset_jsr_rt`: Short code fragment that saves registers on the stack, then copies a location in the code to a5, then copies a fixed memory address to a6, then loads 0 into d0, then copies the memory at an offset from a5 to d1, then sets bit d1 of d0, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_movem_movea_moveq_cmpi_beq_bra`: Short code fragment that saves registers on the stack, then copies a1 to a5, then copies a0 to a1, then loads 0 into d1, then compares the memory at an offset from a0 with 1, then branches if the last result was zero, then compares the memory at an offset from a0 with 2, then branches if the last result was zero, then jumps to another point in the code.
- `frag_bsr_cmpi_beq_bne_move_andi_cmp_bra`: Short code fragment that calls another routine, then compares a fixed memory address with 6, then branches if the last result was zero, then compares a fixed memory address with 4, then branches if the last result was not zero, then copies the memory at an offset from a0 to d0, then masks d0 with 72, then compares d0 with 72, then branches if the last result was zero, then jumps to another point in the code.
- `frag_cmpi_bne_beq_bra`: Short code fragment that compares a fixed memory address with 5, then branches if the last result was not zero, then compares the memory at an offset from a0 with 69, then branches if the last result was zero, then jumps to another point in the code.
- `frag_cmpi_bne_beq_bra_2`: Short code fragment that compares a fixed memory address with 1, then branches if the last result was not zero, then compares the memory at an offset from a0 with 104, then branches if the last result was zero, then jumps to another point in the code.
- `frag_cmpi_bne_beq_bra_3`: Short code fragment that compares a fixed memory address with 2, then branches if the last result was not zero, then compares the memory at an offset from a0 with 105, then branches if the last result was zero, then jumps to another point in the code.
- `validate_joystick_input`: Checks a global variable value against a specific byte constant, and if they don't match, copies a word value from memory to a register, performs bitwise operations on it, compares the result with another constant, and branches accordingly.
- `frag_move_bra`: Short code fragment that copies a1 to d0, then jumps to another point in the code.
- `copy_and_signal`: Copies a longword from the address in a0 to the address in d1 if d1 is not zero, then calls a routine using a pointer from a6 and values from a5, finally returning the original value from a0 in d0.
- `allocate_port_and_signal`: Allocates a signal and memory for a message port structure, initializes its fields with values from registers and parameters, adds it to the system port list if needed, and returns a pointer to the allocated structure.
- `release_port_and_signal`: If a0 is not null and a value at offset 10 of the structure in a0 is not zero, it calls a routine using a0, clears offset 20 of the structure, and calls two other routines to free a signal and a port.
- `allocate_memory_structure`: Allocates memory for a structure of fixed size, initializes its type byte to 5, sets its word field to 48, and stores a pointer value in its longword field before returning the allocated address.
- `free_memory_and_signal`: If a0 is not null, it sets offsets 20 and 24 of the structure in a0 to negative one, then calls a routine using a value from offset 18 of the structure in a0.
- `frag_cmpi_bne_clr_bra`: Short code fragment that compares the memory at an offset from a0 with 206, then branches if the last result was not zero, then clears a fixed memory address, then jumps to another point in the code.
- `frag_cmpi_bne_clr_bra_2`: Short code fragment that compares the memory at an offset from a0 with 207, then branches if the last result was not zero, then clears a fixed memory address, then jumps to another point in the code.
- `check_and_set_flag`: Reads a word from offset 8 of the structure in a0, masks it with 512 and branches if non-zero, then reads the same word, masks it with 192 and branches if zero, then compares a word at offset 6 with 78 and sets a global flag if equal.
- `frag_cmpi_bne_st_rts`: Short code fragment that compares the memory at an offset from a0 with 79, then branches if the last result was not zero, then sets a fixed memory address to all ones, then returns.
- `process_input_events_2`: Reads a byte from a global variable and branches to different handlers based on its value, with some paths calling another routine before continuing.
- `frag_bsr_bra`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_jsr_bra`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_jsr_bra_2`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_2`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_3`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_4`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_jsr_bra_3`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_clr_bsr_bra`: Short code fragment that clears d0, then calls another routine, then jumps to another point in the code.
- `frag_jsr_bra_4`: Short code fragment that calls another routine, then jumps to another point in the code.
- `update_input_state`: Processes input events and updates a counter in a global variable based on input conditions, then calls another routine to handle the input processing.
- `process_input_events`: Tests a global variable and clears it if non-zero, decrements another global variable, compares it with zero, and calls different routines based on the value of the global variable or hardware register bits.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `fall_through_entry_4`: Entry point part-way through straight-line code: runs two instructions (subi.l, ori.b) and then falls through into the code that follows, with no return of its own.
- `frag_bra_4`: Short code fragment that jumps to another point in the code.
- `frag_bra_5`: Short code fragment that jumps to another point in the code.
- `frag_bra_6`: Short code fragment that jumps to another point in the code.
- `frag_bra_7`: Short code fragment that jumps to another point in the code.
- `frag_bra_8`: Short code fragment that jumps to another point in the code.
- `frag_bra_9`: Short code fragment that jumps to another point in the code.
- `frag_bra_10`: Short code fragment that jumps to another point in the code.
- `frag_bra_11`: Short code fragment that jumps to another point in the code.
- `frag_bra_12`: Short code fragment that jumps to another point in the code.
- `frag_bra_13`: Short code fragment that jumps to another point in the code.
- `frag_bra_14`: Short code fragment that jumps to another point in the code.
- `frag_bra_15`: Short code fragment that jumps to another point in the code.
- `frag_bra_16`: Short code fragment that jumps to another point in the code.
- `frag_movem_lea_tst_bmi_bsr_bra`: Short code fragment that saves registers on the stack, then loads the address of a location in the code into a6, then tests the memory at an offset from a6, then branches if the last result was negative, then calls another routine, then jumps to another point in the code.
- `frag_tst_beq_bne_lea_clr_bra`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was zero, then tests the memory at an offset from a6, then branches if the last result was not zero, then loads the address of a location in the code into a0, then clears the memory at an offset from a0, then jumps to another point in the code.
- `frag_bsr_movem`: Short code fragment that calls another routine (2 times in a row), then restores saved registers from the stack.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `frag_lea_movea_tst_beq_subi_rts`: Short code fragment that loads the address of a location in the code into a5, then copies the memory at an offset from a6 to a4, then tests the memory at an offset from a6, then branches if the last result was zero, then subtracts 1 from the memory at an offset from a6, then returns.
- `frag_move_tst_beq_clr_rts`: Short code fragment that copies the memory at an offset from a5 to the memory at an offset from a6, then tests the memory at an offset from a6, then branches if the last result was zero, then tests the memory at an offset from a6, then branches if the last result was zero, then clears the memory at an offset from a6 (2 times in a row), then copies 1 to the memory at an offset from a6, then returns.
- `process_noise_entries`: Clears a byte in memory at an offset from a6, calls another routine, tests the same byte, and if it's non-zero, increments a pointer and repeats the process until a condition is met.
- `validate_noise_entry`: Checks if a global variable at offset 48 of the structure in a0 has a value between 144 and 254, and if so, sets that variable to 255.
- `frag_tst_beq_subi_rts`: Short code fragment that tests the memory at an offset from a0, then branches if the last result was zero, then subtracts 1 from the memory at an offset from a0, then returns.
- `update_sound_data`: Reads a value from a structure, shifts it left by two bits, uses the result as an offset to load data into another structure.
- `frag_move_andi_asl_lea_adda_jmp`: Short code fragment that copies the memory at an offset from a6 to d0, then masks d0 with 15, then shifts d0 left by 2, then loads the address of a location in the code into a1, then copies an indexed entry from a1 to d1, then loads the address of a location in the code into a1, then adds d1 to a1, then jumps to the address in a1.
- `frag_addi_bra`: Short code fragment that adds 1 to the memory at an offset from a0, then jumps to another point in the code.
- `frag_addi_rts`: Short code fragment that adds 1 to the memory at an offset from a0, then returns.
- `frag_cmpi_beq_move_cmp_bne_bra`: Short code fragment that compares the memory at an offset from a0 with 129, then branches if the last result was zero, then copies the memory at an offset from a5 to d0, then compares d0 with the memory at an offset from a5, then branches if the last result was not zero, then copies the memory at an offset from a5 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_addi_bsr_move_rts`: Short code fragment that adds 1 to the memory at an offset from a5, then calls another routine, then copies 65535 to the memory at an offset from a6, then returns.
- `frag_tst_beq_cmpi_subi_bra`: Short code fragment that tests the memory at an offset from a0, then branches if the last result was zero, then compares the memory at an offset from a0 with 255, then branches if the last result was zero, then subtracts 1 from the memory at an offset from a0, then jumps to another point in the code.
- `frag_move_bra_2`: Short code fragment that copies 255 to the memory at an offset from a0, then jumps to another point in the code.
- `frag_move_subi_bra`: Short code fragment that copies the memory at an offset from a6 to d0, then subtracts 1 from d0, then copies d0 to the memory at an offset from a0, then copies the memory at an offset from a6 to the memory at an offset from a0, then jumps to another point in the code.
- `load_sound_data`: Reads a byte from a structure, masks it with 127, stores it in another structure, uses the masked value as an offset to load data from a table, and stores the result in the second structure.
- `frag_move_bra_3`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a0, then jumps to another point in the code.
- `frag_move_rts`: Short code fragment that copies 255 to the memory at an offset from a0, then returns.
- `frag_move_tst_bne_bsr_bra`: Short code fragment that copies the memory at an offset from a6 to d0, then tests the memory at an offset from a0, then branches if the last result was not zero, then calls another routine, then jumps to another point in the code.
- `load_sound_data_2`: Loads sound data using a lookup table indexed by a byte value, then updates a structure with the loaded value and increments a counter.
- `frag_move_bra_4`: Short code fragment that copies the memory at an offset from a5 to the memory at an offset from a5 (2 times in a row), then jumps to another point in the code.
- `initialize_noise_player`: Initializes noise player state by setting a global flag, copying byte values from offsets 41 and 43 of a6 to offsets 25-27, and conditionally setting offset 28 based on comparisons between bytes at offsets 24 and 25 of a6.
- `frag_move_clr_bra`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a6, then copies 0 to the memory at an offset from a6, then clears the memory at an offset from a1, then jumps to another point in the code.
- `load_and_shift_byte`: Loads an unsigned byte from a global variable, performs a bitwise AND with 255, shifts the result left by one bit, and stores it in another global variable.
- `process_noise_data`: Reads word values from a data structure, compares them with specific constants, and updates global variables based on the comparison results.
- `frag_bsr_clr_lea_move_movem_rts`: Short code fragment that calls another routine, then clears the memory at an offset from a6, then loads the address of a location in the code into a1, then copies 2 to the memory at an offset from a1, then restores saved registers from the stack, then returns.
- `frag_move_bra_5`: Short code fragment that copies the memory at a0 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_addi_bra`: Short code fragment that copies the memory at a0 to the memory at an offset from a5, then copies the memory at a0 to the memory at an offset from a6, then adds 1 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_addi_bra_2`: Short code fragment that copies the memory at a0 to the memory at an offset from a6, then adds 1 to the memory at an offset from a5, then jumps to another point in the code.
- `update_noise_state`: Updates noise-related state variables based on input data and comparison results, then processes noise data and moves fragments.
- `frag_move_clr_bra_2`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a6, then copies 0 to the memory at an offset from a6, then clears the memory at an offset from a1, then jumps to another point in the code.
- `frag_subi_ori`: Short code fragment that subtracts 1194 from the memory at a6, then merges 178 into d0, then merges 196 into d0, then merges 210 into d0.
- `initialize_audio`: Sets up audio registers by loading addresses into a4, a5, and a6, then calls a routine three times to configure audio channels, updating pointers in a5 and a4 each time.
- `frag_tst_beq_bpl_bmi_subi_bra`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was zero, then branches if the last result was not negative, then tests the memory at an offset from a5, then branches if the last result was negative, then subtracts 1 from the memory at an offset from a5, then jumps to another point in the code.
- `process_audio_data_2`: Reads audio command bytes from a structured data buffer, uses them as indices to look up function pointers in a table, and jumps to the corresponding routines.
- `frag_clr_move_bra`: Short code fragment that clears the memory at an offset from a5 (3 times in a row), then copies the memory at an offset from a5 to a fixed memory address, then jumps to another point in the code.
- `frag_move_bra_6`: Short code fragment that copies the memory at an offset from a5 to a fixed memory address, then jumps to another point in the code.
- `frag_move_andi_add_bra`: Short code fragment that copies the memory at an offset from a6 to d0, then masks d0 with an address, then adds the memory at an offset from a6 to d0, then copies d0 to the memory at an offset from a5, then copies d0 to the memory at a4, then jumps to another point in the code.
- `frag_move_bra_7`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a5, then copies the memory at an offset from a6 to the memory at an offset from a4, then jumps to another point in the code.
- `frag_move_bra_8`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_tst_beq_cmpi_subi_bra_2`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was zero, then compares the memory at an offset from a5 with 255, then branches if the last result was zero, then subtracts 1 from the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_9`: Short code fragment that copies 255 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_subi_bra_2`: Short code fragment that copies the memory at an offset from a6 to d0, then subtracts 1 from d0, then copies d0 to the memory at an offset from a5, then copies the memory at an offset from a6 to the memory at an offset from a5, then jumps to another point in the code.
- `load_audio_function`: Loads an audio function pointer from a table using a byte value as index, calculates an address by shifting and adding, and stores the result in a global structure.
- `frag_clr_rts`: Short code fragment that clears the memory at an offset from a5, then returns.
- `update_noise_value`: Calculates a noise value using data from registers and a table, stores the result in a global variable, and updates a counter.
- `update_audio_parameters_4`: Reads a byte value from a global structure, applies a bitwise AND operation with 254, then stores the result in a specific offset of another structure while also updating other values in that structure.
- `frag_move_tst_bne_bra`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a5, then tests the memory at an offset from a5, then branches if the last result was not zero, then copies the memory at an offset from a6 to the memory at an offset from a4, then jumps to another point in the code.
- `initialize_audio_data_2`: Copies data from global variables to specific offsets within a structure, performs bit shifts and comparisons on values, and updates counters in the structure based on conditional checks.
- `frag_move_asl_add_bra`: Short code fragment that copies the memory at an offset from a5 to d0, then shifts d0 left by 1, then adds the memory at an offset from a5 to d0, then adds the memory at an offset from a6 to d0, then copies d0 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_11`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_12`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a5 (4 times in a row), then jumps to another point in the code.
- `frag_tst_bne_bra`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was not zero, then jumps to another point in the code.
- `frag_move_ext_add_bra`: Short code fragment that copies the memory at an offset from a6 to d0, then sign-extends d0, then copies the memory at an offset from a5 to d1, then adds d0 to d1, then copies d1 to the memory at an offset from a5, then copies d1 to the memory at a4, then jumps to another point in the code.
- `frag_move_add_bra`: Short code fragment that copies the memory at an offset from a6 to d0, then copies the memory at an offset from a5 to d1, then adds d0 to d1, then copies d1 to the memory at an offset from a5, then copies d1 to the memory at an offset from a4, then jumps to another point in the code.
- `frag_clr_bra`: Short code fragment that clears the memory at an offset from a5 (3 times in a row), then jumps to another point in the code.
- `configure_dma_and_timing`: Writes a word value from a structure to a hardware register controlling DMA channels, reads byte values from specific offsets of a structure, and modifies those values based on comparison results.
- `calculate_and_store_value`: Loads a long value from a structure, performs arithmetic operations with data from another structure, stores the result back to the first structure, and updates word values in that structure.
- `process_audio_buffer`: Copies long and word values between structures, reads a value from a global variable, performs bitwise operations on it, adds the result to another value in the structure, and stores updated values back to memory.
- `frag_move_bra_13`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a5, then copies the memory at an offset from a6 to the memory at a4, then copies 1 to the memory at an offset from a5, then copies 1 to the memory at an offset from a4, then jumps to another point in the code.
- `update_audio_state`: Loads data from global variables, performs arithmetic operations with values from a structure, stores results in specific offsets of that structure, and updates counters based on conditional checks.
- `decrement_timer_and_accumulate`: When a flag byte in the record at A5 is set, adds a byte value to a short accumulator and, if a companion field is zero, mirrors the sum into a second record; it then decrements a counter byte and, at zero, reloads it from a companion byte and inverts and increments a byte field.
- `update_timer_value`: Updates a timer value based on comparisons with another value, clearing the timer and setting a new value in a global variable when conditions are met.
- `frag_add_cmp_beq_bcc_bra`: Short code fragment that adds the memory at an offset from a5 to d0, then compares d0 with d1, then branches if the last result was zero, then branches if the carry flag is clear, then jumps to another point in the code.
- `frag_tst_beq_subi_rts_2`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was zero, then tests the memory at an offset from a5, then branches if the last result was zero, then subtracts 1 from the memory at an offset from a5, then returns.
- `copy_and_compare_bytes`: Copies a byte from offset 112 of a structure in a5 to offset 113, then reads a byte from offset 114 of the same structure into d0 and compares it with a byte at offset 96.
- `frag_move_clr_rts`: Short code fragment that copies the memory at an offset from a5 to the memory at an offset from a5, then clears the memory at an offset from a5, then returns.
- `frag_move_add_cmp_ble_rts`: Short code fragment that copies the memory at an offset from a5 to d1, then adds d1 to the memory at an offset from a5, then compares d0 with the memory at an offset from a5, then branches if the comparison was less or equal, then returns.
- `process_sound_command`: Reads a byte value from a structure, performs bit manipulation to calculate an offset, checks a condition at that offset, and if the condition is false, updates multiple fields in another structure based on the initial byte value.
- `initialize_sound_channel_2`: Compares a byte value with a constant, and if equal, copies several bytes from one structure to another while performing bit operations on a register before storing the results.
- `frag_cmpi_bne_clr_bra_3`: Short code fragment that compares the memory at an offset from a6 with 245, then branches if the last result was not zero, then clears the memory at an offset from a5, then jumps to another point in the code.
- `initialize_sound_parameters`: Initializes sound channel parameters by copying data from source to destination structures, setting up various channel settings including frequency and volume based on input values.
- `initialize_sound_channel`: Initializes a sound channel by copying data from a source structure to a destination structure, setting up channel parameters including a counter and a lookup table index.
- `frag_movem_lea_andi_asl_adda_move_clr_r`: Short code fragment that saves registers on the stack, then loads the address of a location in the code into a5, then masks d0 with 3, then shifts d0 left by 2, then adds d0 to a5, then copies the memory at an offset from a5 to a fixed memory address, then clears the memory at an offset from a5, then restores saved registers from the stack, then returns.
- `initialize_audio_state`: Clears and initializes audio-related variables and registers, copies data from a table indexed by a value in the structure, updates global state based on comparison results, and sets bits in the CIAA_PRA register.
- `update_noise_flag`: Updates a noise flag based on comparison between two byte values, setting it to 1, -1, or 0 depending on the result of the comparison.
- `frag_move_clr_movem_rts`: Short code fragment that copies 0 to the memory at an offset from a6, then clears the memory at an offset from a5, then restores saved registers from the stack, then returns.
- `load_address_table`: Stores four addresses from a table into offsets 4, 8, 12, and 16 of a structure in a0, then restores a1 from the stack and returns.
- `frag_move_lea_clr_movea_rts`: Short code fragment that copies a0 to the stack, then loads the address of a location in the code into a0, then copies 1 to the memory at an offset from a0, then clears the memory at an offset from a0, then clears the memory at an offset from a6, then copies the top of the stack to a0, then returns.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `load_table_entry`: Sets a word at offset 64 of a structure in a6 to one, copies a value from d0 to offset 100 of a structure in a5, fetches a 32-bit longword from a table using the value in d0 as an index, stores that longword at offset 68 of the structure in a5, clears offset 132 of the structure in a5, and sets offset 102 to all ones.
- `populate_structure_fields`: Sets a word at offset 64 of a structure in a6 to one, copies d0 to offset 100 of a structure in a5, copies a1 to offset 68 of the same structure, clears offset 132 of that structure, and sets offset 102 to all ones.
- `frag_bset_move_lea_movea_rts`: Short code fragment that sets bit 5 of d0, then copies a6 to the stack, then loads the address of a location in the code into a6, then copies d0 to the memory at an offset from a6, then copies the top of the stack to a6, then returns.
- `set_flagged_global_from_register`: Takes a word from a data register, sets bit six of it, and stores the result in a shared variable.
- `reset_audio_hardware`: Clears four audio volume registers, sets a value of fifteen in a hardware register, reads a word from offset 12 of a structure in a6 into d0, uses that value to index into another table, writes the result to an offset in a structure in a6 after performing bitwise operations, clears four words at offsets 2, 6, 10 and 14 of a structure in a5.
- `copy_table_data`: Copies data from two source locations to a destination table using a loop that runs thirty-one times.
- `store_self_reference_in_ram`: The routine writes a global variable into a RAM location, then stores the address of that global variable back into the global variable itself, and finally stores the address of another routine into a RAM location.
- `frag_move_lea_movea_rts`: Short code fragment that copies a6 to the stack, then loads the address of a location in the code into a6, then copies the memory at an offset from a6 to a fixed memory address, then copies the top of the stack to a6, then returns.
- `frag_movem_move_andi_beq_bsr`: Short code fragment that saves registers on the stack, then copies a fixed memory address to d0, then masks d0 with 32, then branches if the last result was zero, then calls another routine.
- `frag_jsr_movem_move_rts`: Short code fragment that calls another routine, then restores saved registers from the stack, then copies a location in the code to the stack, then returns.
- `frag_move_dbra_rts_3`: Short code fragment that copies d0 to the stack, then copies 4095 to d0, then copies d0 to a fixed memory address, then loops back while counting down d0, then copies the top of the stack to d0, then returns.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `generate_audio_parameters`: Performs multiple calculations involving multiplication and addition to populate several tables, then enters a loop to initialize a structure with various values and pointers, including setting up audio channel parameters and configuring DMA registers.
- `frag_clr_move_rts`: Short code fragment that clears a fixed memory address (4 times in a row), then copies 15 to a fixed memory address, then returns.
- `reset_audio_counters`: Decrements a byte counter in a global variable and branches if it's not zero, otherwise calls another routine to reset audio counters and updates multiple audio structures with specific addresses.
- `frag_movea_move_bne_bsr_bra`: Short code fragment that copies the memory at an offset from a1 to a0, then copies the memory at an offset from a1 to d0, then branches if the last result was not zero, then calls another routine, then jumps to another point in the code.
- `frag_add_tst_beq_bpl_cmp_bge_bra`: Short code fragment that adds the memory at an offset from a1 to d0, then tests the memory at an offset from a1, then branches if the last result was zero, then branches if the last result was not negative, then compares d0 with the memory at an offset from a1, then branches if the comparison was greater or equal, then jumps to another point in the code.
- `frag_cmp_ble_move_add_cmpi_bge_bra`: Short code fragment that compares d0 with the memory at an offset from a1, then branches if the comparison was less or equal, then copies the memory at an offset from a1 to d0, then adds the memory at an offset from a1 to d0, then compares d0 with 135, then branches if the comparison was greater or equal, then copies 135 to d0, then jumps to another point in the code.
- `clamp_audio_output`: Checks if a word value exceeds 1023 and clamps it to that maximum, then adds a value from another structure and checks if the result exceeds -1024, clamping it to that minimum if necessary.
- `clamp_audio_value`: Checks if a word at offset 24 of a structure is greater than 1023, and if so, sets that word to 1023.
- `blend_audio_samples`: If a word at offset 34 of a structure in a1 is greater than 64, sets that word to 64, checks if bit 1 of a byte at offset 46 is set, and if so, performs a loop to add bytes from two memory locations, shifts the result right by one bit, and stores it back into a table.
- `frag_move_rts_2`: Short code fragment that copies the memory at an offset from a1 to d0, then copies the memory at an offset from a1 to the memory at an offset from a1 (2 times in a row), then copies d0 to the memory at an offset from a1, then returns.
- `configure_audio_hardware`: Updates pointers and counters in a structure by decrementing a byte counter and updating a pointer when it reaches zero, then performs several subroutine calls with different addresses, updates hardware registers with values from the structure, and uses two loops to process data.
- `update_structure_fields`: Reads a word from offset 32 of a structure in a1, performs bitwise AND with a word at offset 2 of a structure in a1, and if the result is zero, returns immediately.
- `frag_move_beq_andi_lsl_addi_movea_btst`: Short code fragment that copies the memory at an offset from a0 to d1, then branches if the last result was zero, then masks d1 with 255, then shifts d1 left by 1, then adds 2866 to d1, then copies d1 to a2, then tests bit 6 of the memory at an offset from a0, then branches if the last result was zero, then copies the memory at a2 to the memory at an offset from a1, then jumps to another point in the code.
- `initialize_audio_player_2`: Initializes audio player parameters by copying data from a memory location to another, setting up various audio configuration fields, and performing bit operations on audio control flags.
- `frag_movea_btst_bne_move_lsr_bra`: Short code fragment that copies the memory at an offset from a2 to a3, then tests bit 1 of the memory at an offset from a2, then branches if the last result was not zero, then copies the memory at an offset from a2 to d0, then shifts d0 right by 1, then copies d0 to the memory at an offset from a1, then jumps to another point in the code.
- `initialize_audio_parameters_4`: Reads audio configuration data from a structure and writes processed values to another structure, including clearing certain fields based on conditions.
- `initialize_audio_parameters_2`: Reads audio configuration data from a structure and writes processed values to another structure, including clearing certain fields based on conditional checks.
- `initialize_audio_parameters`: Clears a 16-bit word at offset 30 of a structure, masks a byte from memory with 15, sign-extends the result to 16 bits, and stores it at offset 32 of the structure.
- `frag_andi_ext_move_clr_bne_btst_beq_bra`: Short code fragment that masks d0 with 15, then sign-extends d0, then copies d0 to the memory at an offset from a1, then clears the memory at an offset from a1, then copies the memory at an offset from a0 to d0, then branches if the last result was not zero, then tests bit 7 of the memory at an offset from a0, then branches if the last result was zero, then jumps to another point in the code.
- `frag_cmpi_bne_clr_bra_4`: Short code fragment that compares d0 with 255, then branches if the last result was not zero, then clears the memory at an offset from a1, then jumps to another point in the code.
- `frag_btst_beq_move_clr_bra`: Short code fragment that tests bit 7 of the memory at an offset from a0, then branches if the last result was zero, then copies d0 to the memory at an offset from a1 (2 times in a row), then clears the memory at an offset from a1, then jumps to another point in the code.
- `process_audio_data`: Clears bit 7 of a byte in d0, branches if the result is zero, negates the byte in d0, sign-extends it to 16 bits, and stores the result at offset 36 of a structure.
- `frag_clr_btst_beq_move_cmp_bgt_neg_bra`: Short code fragment that clears the memory at an offset from a1, then tests bit 6 of the memory at an offset from a0, then branches if the last result was zero, then copies the memory at an offset from a1 to d1, then compares d1 with the memory at an offset from a1, then branches if the comparison was greater, then negates d0, then jumps to another point in the code.
- `frag_bclr_bne_neg_move_bra`: Short code fragment that clears bit 7 of d0, then branches if the last result was not zero, then negates d0, then copies 135 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_move_ext_rts`: Short code fragment that copies 1019 to the memory at an offset from a1, then sign-extends d0, then copies d0 to the memory at an offset from a1, then returns.
- `frag_movea_jsr_bset_rts`: Short code fragment that copies a location in the code to a0, then calls the routine whose address is in a0, then sets bit 1 of a fixed memory address, then returns.
- `frag_movea_move_jsr_bset_rts`: Short code fragment that copies a location in the code to a0, then copies 20085 to the memory at an offset from a0, then calls another routine, then sets bit 1 of a fixed memory address, then returns.
- `frag_movea_move_jsr_bset_rts_2`: Short code fragment that copies a location in the code to a0, then copies 20085 to the memory at an offset from a0, then calls another routine, then sets bit 1 of a fixed memory address, then returns.
- `frag_movea_jsr_rts`: Short code fragment that copies a location in the code to a0, then calls another routine, then returns.
- `frag_movea_jsr_rts_2`: Short code fragment that copies a location in the code to a0, then calls another routine, then returns.
- `frag_movea_jsr_rts_3`: Short code fragment that copies a location in the code to a0, then calls another routine, then returns.
- `frag_jmp`: Short code fragment that jumps to another point in the code.
- `frag_moveq_movea_jsr_bset_rts`: Short code fragment that loads 1 into d0, then copies a location in the code to a0, then calls the routine whose address is in a0, then sets bit 1 of a fixed memory address, then returns.
- `frag_moveq_movea_jsr_rts`: Short code fragment that loads 0 into d0, then copies a location in the code to a0, then calls the routine whose address is in a0, then returns.
- `frag_jmp_2`: Short code fragment that jumps to another point in the code.
- `frag_clr_move_rts_2`: Short code fragment that clears a fixed memory address (4 times in a row), then copies 15 to a fixed memory address, then returns.
- `update_audio_buffers`: Copies a long value from memory to a table, fills a block of memory with bytes from another source using a loop, and calculates a value based on a table lookup and arithmetic to store in a global variable before performing a loop that writes several long words to memory based on values in a structure.
- `frag_movem_bsr_moveq_rts`: Short code fragment that saves registers on the stack, then calls another routine, then restores saved registers from the stack, then loads 0 into d0, then returns.
- `update_audio_registers`: Updates audio register values based on data from a structure, increments a counter, and conditionally calls another routine depending on the counter's value.
- `frag_cmpi_bne_move_andi_add_bsr_bra`: Short code fragment that compares a fixed memory address with 1, then branches if the last result was not zero, then copies the memory at an offset from a0 to d3, then copies the memory at an offset from a0 to d4, then masks d3 with 15, then masks d4 with 15, then adds d3 to d4, then adds the memory at an offset from a0 to d4, then calls another routine, then jumps to another point in the code.
- `decrement_noise_counter`: Decrements a byte value at offset 157 in a global variable and branches to a specific address if the result is zero, otherwise it returns.
- `copy_and_update_dma`: Copies a byte from one memory location to another, calls another routine, writes a word value to a hardware register, and then performs a loop of four iterations that processes data based on a conditional branch.
- `update_noise_player_state`: Processes noise player data to update entity attributes including position, flags, and audio settings, while handling conditional operations based on input values.
- `update_noise_player`: Updates noise player state based on input data, modifies entity attributes including position and flags, and handles conditional operations based on input values.
- `update_entity_attribute`: Clears a bit in the CIAA port and modifies various fields of a structure based on a value, including performing arithmetic on a 16-bit counter in a global variable and updating a byte in a table when a specific counter reaches thirty.
- `initialize_audio_sequence`: Initializes audio sequence parameters including counters, pointers, and volume adjustments based on data from a table, then updates global audio configuration.
- `update_dma_configuration`: Copies a 32-bit word and a 16-bit word from a source structure to a destination and updates the DMA control register by setting bit 15 and adding a value from d1.
- `initialize_audio_parameters_3`: Sets up audio configuration parameters including counters, pointers, and volume adjustments based on data from a table, then updates global audio state variables.
- `configure_audio_dma`: Copies a byte from memory at offset 2 of a structure to offset 9 of another structure, and if the byte is 255, copies a byte from memory relative to another structure to offset 9 of the second structure.
- `set_player_sprite_offset`: The routine writes a 16-bit value from a table indexed by a byte in D4 to the word at offset six from the pointer in A1.
- `process_audio_registers`: Sets up several pointers including the AUD0LCH register, and if a byte at offset 26 of a structure in a2 is non-zero, it calls another routine and executes a loop four times.
- `update_noise_parameters`: Updates noise generation parameters by decrementing counters, loading new values from tables, and performing arithmetic operations on audio data.
- `decrement_and_check_counter`: Decrements a value in a loop, increments a word at offset 14 of a structure, and if a comparison with a value in another structure matches, clears a word at offset 14 and a byte at offset 29 of the first structure.
- `initialize_audio_hardware`: Clears a byte in a global variable, runs a loop to copy bytes from a table, performs arithmetic to calculate a destination address, runs a second loop to write long words to memory, sets a bit in a custom register, and clears several audio volume and CIAA registers.
- `frag_clr_move_rts_3`: Short code fragment that clears a fixed memory address (4 times in a row), then copies 15 to a fixed memory address, then returns.
- `frag_movem_movea_addq_move_cmp_blt_clr`: Short code fragment that saves registers on the stack, then copies a location in the code to a0, then adds 1 to a fixed memory address, then copies a fixed memory address to d0, then compares d0 with a fixed memory address, then branches if the comparison was less, then clears a fixed memory address, then jumps to another point in the code.
- `configure_audio_channels_2`: Sets up audio channel registers by loading data from multiple tables and calling another routine four times with different register addresses, then branches to a label.
- `normalize_audio_value`: Loads a byte from a global memory location into a register, divides it by three using signed division, swaps the word result, then compares the quotient against zero and two with conditional branches, and finally loads another byte from a structure offset into a register after masking its upper nibble.
- `frag_moveq_move_andi_bra`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_move_bra_10`: Short code fragment that copies the memory at an offset from a6 to d2, then jumps to another point in the code.
- `calculate_audio_value`: Shifts a word value in d0 left by one bit, loads zero into d1, copies a word from memory at offset 16 of a structure to d1, calculates an address using the shifted value and a table, and then performs a loop that compares values and advances a counter.
- `frag_move_rts_3`: Short code fragment that copies d2 to the memory at an offset from a5, then returns.
- `update_audio_parameters_3`: Loads configuration values from memory locations, performs arithmetic operations including bit shifts and additions, clears a word value at a specific address, and calls another routine four times with different register addresses.
- `update_audio_parameters_2`: Loads audio configuration values from memory locations, performs arithmetic operations including multiplication and bit shifts, and conditionally updates multiple audio parameters.
- `update_audio_parameters`: Updates audio parameters by loading values from memory locations, performing arithmetic operations, and conditionally calling another routine based on bit comparisons.
- `process_audio_dma`: Copies data from a structure to another location, writes a word value to a hardware register controlling DMA channels, clears a byte in the structure, stores memory values to a destination pointer, performs bitwise operations on a word value and stores the result to another hardware register, then branches to another routine.
- `configure_audio_channels`: Runs two loops to set values in the DMA control register, copies several long and word values from tables into the audio channel registers, increments a counter in a global variable, and if the counter reaches a limit, resets it and updates a byte from a global variable.
- `frag_move_andi_clr_cmp_beq_bge_rts`: Short code fragment that copies the memory at a6 to d2, then masks d2 with 4095, then copies d2 to the memory at an offset from a6, then copies the memory at an offset from a6 to d0, then clears the memory at an offset from a6, then compares d2 with d0, then branches if the last result was zero, then branches if the comparison was greater or equal, then copies 1 to the memory at an offset from a6, then returns.
- `frag_clr_rts_2`: Short code fragment that clears the memory at an offset from a6, then returns.
- `manage_audio_timer`: Manages an audio timer by checking and updating byte and word values in a structure, performing conditional arithmetic operations, and writing results to hardware registers.
- `update_audio_timer`: Subtracts a word value from memory at an offset in a structure, compares the result with another word value from the same structure, branches backward if the first value is less than the second, updates a timer value in the structure, clears another timer field, and stores the updated timer value to a destination pointer.
- `update_audio_phase`: Reads a byte from memory at offset 3 of a structure, branches if zero, stores the byte at offset 26 of the same structure.
- `subtract_and_advance_phase`: Subtracts d2 from d0 and stores the result in offset 6 of a structure in a5, then advances a phase byte in the structure by an amount taken from the high nibble of a neighbouring byte.
- `frag_move_rts_4`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a5, then returns.
- `process_audio_command_2`: Reads a command byte from a structure, processes it based on its value, and updates audio-related fields in the structure.
- `adjust_audio_frequency`: Loads zero into d0, copies a byte from memory at offset 3 of a structure to d0, shifts d0 right by four bits, tests the result, and branches if zero.
- `frag_moveq_move_andi_sub_bpl_clr_rts`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then subtracts d0 from the memory at an offset from a6, then branches if the last result was not negative, then clears the memory at an offset from a6, then copies the memory at an offset from a6 to the memory at an offset from a5, then returns.
- `decrement_position_clamp_minimum`: Subtracts a byte parameter from a 16-bit position.
- `increment_position_clamp_maximum`: Adds a byte parameter to a 16-bit position.
- `process_noise_command`: Reads a byte from offset 2 of a structure, masks its lower four bits, compares it against fixed values, and performs different operations based on the result, including modifying a global variable and updating hardware registers.
- `update_global_from_ciaa_pra`: Masks bit zero of the byte at offset 3 of the structure in a6, shifts it left by one, and ORs the result into a global variable after clearing bit one of that variable.
- `frag_not_rts`: Short code fragment that inverts a fixed memory address, then returns.
- `frag_move_subq_not_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then subtracts 1 from d0, then copies d0 to a fixed memory address, then inverts a fixed memory address, then returns.
- `frag_cmpi_ble_move_rts`: Short code fragment that compares the memory at an offset from a6 with 64, then branches if the comparison was less or equal, then copies 64 to the memory at an offset from a6, then copies the memory at an offset from a6 to the memory at an offset from a5, then returns.
- `clamp_byte_and_copy`: Clamps a byte at an offset from a pointer to a maximum value and, if the byte is non-zero, copies it to a global variable.
- `frag_movem_movea_moveq_jsr_rts`: Short code fragment that saves registers on the stack, then copies a fixed memory address to a0, then loads 255 into d0, then calls the routine whose address is in a0, then restores saved registers from the stack, then returns.
- `frag_movem_movea_moveq_jsr_rts_2`: Short code fragment that saves registers on the stack, then copies a fixed memory address to a0, then loads 0 into d0, then calls the routine whose address is in a0, then restores saved registers from the stack, then returns.
- `frag_jmp_3`: Short code fragment that jumps to another point in the code.
- `frag_lea_move_jsr_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies a fixed memory address to d0, then copies a fixed memory address to d1, then calls another routine, then returns.
- `frag_lea_move_jsr_rts_2`: Short code fragment that loads the address of a fixed memory address into a0, then copies a fixed memory address to d0, then calls another routine, then returns.
- `frag_lea_jsr_rts`: Short code fragment that loads the address of a fixed memory address into a0, then calls another routine, then returns.
- `frag_lea_jsr_rts_2`: Short code fragment that loads the address of a fixed memory address into a0, then calls another routine, then returns.
- `disable_audio_playback`: Clears the audio playing flag and zeroes four audio channel period registers.
- `initialize_audio_structure`: Sets bit 1 of a hardware register, populates pointers in a structure using offsets from a global variable, copies data from a table to multiple fields in the structure, and initializes audio registers and DMA settings for sound playback.
- `frag_lea_tst_bne_rts`: Short code fragment that loads the address of a location in the code into a5, then tests the memory at an offset from a5, then branches if the last result was not zero, then returns.
- `configure_audio_hardware_2`: Initializes audio hardware by calling routines to set up data, writing to custom chip registers, and entering loops that decrement values in a structure and write results to specific hardware register offsets.
- `update_noise_data`: Updates noise player data by copying values from a structure, advancing pointers, and modifying hardware registers based on conditional checks.
- `update_structure_from_data`: Copies values from a structure into another, performs conditional checks on d1, updates a hardware register, and uses a loop to copy multiple bytes from a table into a structure based on calculated offsets.
- `frag_moveq_tst_beq_subq_bra`: Short code fragment that loads 0 into d7, then tests the memory at an offset from a0, then branches if the last result was zero, then subtracts 1 from the memory at an offset from a0, then jumps to another point in the code.
- `update_noise_player_6`: Processes a noise player data stream by reading commands and updating player state, including hardware registers and pointer positions.
- `update_noise_player_5`: Loads a byte from a pointer, uses it as an index to access a table, copies data from the table to a structure, updates pointers and hardware registers based on the loaded value.
- `frag_cmpi_bne_moveq_move_lsl_movea_adda`: Short code fragment that compares d0 with 231, then branches if the last result was not zero, then loads 0 into d0, then copies the memory at an offset from a1 to d0, then shifts d0 left by d6, then copies a location in the code to a1, then adds d0 to a1, then copies a1 to the memory at an offset from a0, then copies d7 to the memory at an offset from a0, then jumps to another point in the code.
- `frag_cmpi_bne_move_addq_bra`: Short code fragment that compares d0 with 232, then branches if the last result was not zero, then copies the memory at an offset from a1 to the memory at an offset from a0, then adds 2 to the memory at an offset from a0, then jumps to another point in the code.
- `update_noise_player_3`: Updates noise player data by copying values from a structure, advancing pointers, and modifying hardware registers based on conditional checks.
- `decrement_and_branch`: Decrements a byte value at offset 23 of the structure in a0, then branches if the result is not zero.
- `frag_cmpi_beq_bne_move_andi_subq_bra`: Short code fragment that compares d0 with 225, then branches if the last result was zero, then compares d0 with 224, then branches if the last result was not zero, then copies the memory at an offset from a1 to d0, then masks d0 with 63, then subtracts 5 from d0, then copies d0 to the memory at an offset from a0, then jumps to another point in the code.
- `update_noise_data_2`: Updates noise player data by copying values from a structure, advancing pointers, and modifying hardware registers based on conditional checks and arithmetic operations.
- `update_noise_state_2`: Updates a noise-related state byte based on conditions involving values from offsets 27, 28, and 29 of a structure, then modifies the state byte at offset 28 and returns a calculated value.
- `calculate_value_with_bounds`: Performs arithmetic on d1 and d4, modifies a byte in a structure, and executes a series of conditional branches to perform calculations and bitwise operations before returning a byte from a structure.
- `disable_audio_playback_2`: Clears the audio playing flag and zeroes four audio channel period registers.
- `initialize_audio_data`: Sets bit 1 of a hardware register, updates multiple structure pointers, copies and manipulates data into tables, and configures audio hardware registers before returning.
- `frag_lea_tst_bne_rts_2`: Short code fragment that loads the address of a location in the code into a5, then tests the memory at an offset from a5, then branches if the last result was not zero, then returns.
- `setup_audio_channels`: Configures audio channels by calling initialization routines, writing to custom chip registers, and managing loops that decrement values in a structure and write results to specific hardware register offsets.
- `update_audio_data`: Reads audio data from a buffer, updates hardware registers based on calculated offsets, and performs conditional operations on audio structure fields.
- `update_audio_structure`: Copies audio data from a source buffer into a structure, updates hardware registers based on calculated offsets, and performs conditional checks on input data before writing to multiple structure fields.
- `frag_moveq_tst_beq_subq_bra_2`: Short code fragment that loads 0 into d7, then tests the memory at an offset from a0, then branches if the last result was zero, then subtracts 1 from the memory at an offset from a0, then jumps to another point in the code.
- `process_noise_command_2`: Reads a noise command from a buffer, updates command pointers and counters, and handles different command types including DMA configuration and data loading from tables.
- `load_noise_command`: Loads a noise command from a buffer into the player structure, updates command pointers and counters, then branches to a sequence update routine.
- `update_noise_player_7`: Updates noise player state by processing command bytes from a buffer, modifying player structure fields including counters, pointers, and DMA control registers, then advances the buffer position.
- `initialize_noise_player_2`: Initializes a noise player by loading a value from a table, shifting it, and storing the result along with another value into a structure.
- `frag_cmpi_bne_move_addq_bra_2`: Short code fragment that compares d0 with 234, then branches if the last result was not zero, then copies the memory at an offset from a1 to the memory at an offset from a0 (2 times in a row), then adds 3 to the memory at an offset from a0, then jumps to another point in the code.
- `frag_cmpi_bne_move_addq_bra_3`: Short code fragment that compares d0 with 232, then branches if the last result was not zero, then copies the memory at an offset from a1 to the memory at an offset from a0, then adds 2 to the memory at an offset from a0, then jumps to another point in the code.
- `update_noise_sequence`: Updates noise player sequence state by advancing a counter and loading data from a sequence table, then performs conditional arithmetic and bit manipulation on structure fields.
- `update_noise_player_4`: Updates noise player state by decrementing a counter and loading data from a sequence, then performs conditional arithmetic and bit manipulation on structure fields before returning.
- `frag_cmpi_bne_addq_move_bra`: Short code fragment that compares d0 with 232, then branches if the last result was not zero, then adds 2 to the memory at an offset from a0, then copies the memory at an offset from a1 to the memory at an offset from a0, then jumps to another point in the code.
- `frag_cmpi_bne_move_andi_subq_bra`: Short code fragment that compares d0 with 224, then branches if the last result was not zero, then copies the memory at an offset from a1 to d0, then masks d0 with 63, then subtracts 5 from d0, then copies d0 to the memory at an offset from a0, then jumps to another point in the code.
- `toggle_and_count`: Toggles the value of a byte at offset 38 of the structure in a0, then branches if the result is zero.
- `update_noise_player_2`: Updates internal noise player state based on conditional checks and arithmetic operations involving structure offsets, then returns a modified value from a table.
- `update_noise_state_3`: Updates internal noise player state based on conditional checks and arithmetic operations involving structure offsets, then returns a modified value.
- `calculate_offset_value`: Performs arithmetic on values from a structure, conditionally modifies a byte, and uses several conditional branches to eventually return a value derived from a swap operation.
