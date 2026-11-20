# Game 3608 (fatal mission 3 (1994)(r. burrows)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03608). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 18 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 81 of 81 named (0 author symbols), 81 of 81 described (23 by rule, 57 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2416 of 2416 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3251 of 3251 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 18867 |
| code | 10742 |
| data | 100460 |
| padding | 4075 |
| unused | 766976 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Fatal Mission 3 (1994)(R. Burrows).adf:1 | 3 | 3233 | 80 |
| Fatal Mission 3 (1994)(R. Burrows).adf:bootblock | 1 | 18 | 1 |

## Cases handled

- fatal mission 3 (1994)(r. burrows)/Fatal Mission 3 (1994)(R. Burrows).adf:Graphics/Fighter1.RAW: PowerPacker Data (xfd)-packed, unpacked with ancient (45900 -> 94320 bytes)
- fatal mission 3 (1994)(r. burrows)/Fatal Mission 3 (1994)(R. Burrows).adf:Graphics/ShipRot.RAW: PowerPacker Data (xfd)-packed, unpacked with ancient (23052 -> 48992 bytes)
- fatal mission 3 (1994)(r. burrows)/Fatal Mission 3 (1994)(R. Burrows).adf:Graphics/Explosion.RAW: PowerPacker Data (xfd)-packed, unpacked with ancient (3556 -> 17552 bytes)
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Fatal Mission 3 (1994)(R. Burrows).adf:1

- `process_system_messages`: Reads system messages from a message port into a global variable, extracts a value from the message, and handles message reply logic by calling forbidden routines before returning.
- `initialize_graphics_subsystem_3`: Initializes graphics subsystem by opening required libraries, allocating memory for graphics resources, setting up screen and window parameters, and managing resource cleanup.
- `initialize_memory_blocks`: Clears multiple memory blocks starting from a base address and sets up pointers to various offsets within the same memory region, then stores a zero byte at a specific global offset.
- `initialize_graphics_subsystem_2`: Allocates memory blocks for graphics resources, opens a screen and window with specified parameters, sets up raster operations and pointer display, then performs additional graphics initialization including color mapping and text rendering before returning.
- `setup_audio_device`: Allocates a signal, opens an audio device, and stores the result status in a global variable before returning.
- `cleanup_audio_device`: Checks if an audio device was initialized, disables DMA channels, closes the device, and frees the allocated signal if it was successfully set up.
- `initialize_game_state`: Initializes various game state variables including flags, counters, and graphics parameters, then loads color palettes and clears memory regions.
- `update_sprite_positions`: Calculates sprite coordinates based on memory values, updates sprite positions using graphics functions, and moves sprites to calculated screen locations.
- `handle_input_and_render`: Processes input from joystick hardware, updates rendering states, and draws line segments using graphics functions in a loop until completion.
- `load_and_display_text`: Loads color palette entries into a graphics structure, then displays text strings using a raster port and a table of text parameters.
- `wait_for_joystick_input`: Waits for specific joystick input states by polling a hardware register, updates a global counter based on button presses, and sets a flag when the counter reaches a specific value.
- `initialize_graphics_system`: Sets up graphics interrupt handlers, manages blitter ownership, and initializes various game state variables including timers and memory blocks before entering the main game loop.
- `frag_move_andi_movea_moveq_add_dbra_cmp`: Short code fragment that copies 7998 to d0, then masks d0 with an address, then copies d0 to a0, then loads 0 into d0, then loads 79 into d1, then adds the memory at a0 (advancing a0) to d0, then loops back while counting down d1, then compares d0 with an address, then branches if the last result was not zero, then returns.
- `render_text_frame`: Loads color palette data, moves sprites to specific screen coordinates, sets drawing pen color, draws text, and waits for vertical retrace interrupts.
- `initialize_text_buffer`: Initializes text buffer variables and sets up initial character values in global memory.
- `render_text_and_graphics`: Renders text and graphics elements on screen by calculating positions, setting pen colors, and drawing rectangles and text based on game state variables.
- `process_character_data`: Performs mathematical operations on input data to calculate coordinates, then calls graphics functions to draw lines using those calculated values.
- `draw_line_segments`: Reads coordinate pairs from memory and draws line segments between them using graphics functions, repeating this process a fixed number of times.
- `render_score_entries`: Processes and displays numeric score data entries in a structured format, handling digit conversion and text rendering for multiple entries.
- `frag_movem_movea_bsr_addq_rts`: Short code fragment that saves registers on the stack, then copies a fixed memory address to a5, then calls another routine, then adds 1 to the memory at an offset from a5, then restores saved registers from the stack, then returns.
- `process_animation_frame`: Updates animation frame counters and manages frame data retrieval for game objects by processing a counter and accessing frame tables based on calculated indices.
- `draw_polygon_outline`: Checks for end-of-data marker, sets drawing pen color, then draws connected lines between coordinate pairs read from memory.
- `play_audio_samples`: Processes audio sample data by reading from memory locations, setting up audio registers with calculated values, and updating pointers for the next iteration.
- `animate_color_transitions`: Handles color transition animations by decrementing counters, calculating RGB values, and calling graphics functions to update screen colors in sequence.
- `update_collision_detection`: Processes collision detection for game elements by iterating through a table of object data and checking position boundaries against calculated values, clearing flags when conditions are met.
- `check_collision`: Checks for collisions between two sets of game objects and updates their states when a collision is detected.
- `update_animation`: Manages animation state updates for game elements by decrementing a counter and modifying object properties based on animation frame data.
- `decrement_timer`: Decrements a timer value stored in a global variable and returns immediately if the value is zero.
- `update_projectile_effects`: Updates projectile effect positions and renders them, clearing inactive effects and handling drawing operations for multiple projectile instances.
- `draw_lines`: Processes a series of line drawing commands from a table, moving and drawing lines using graphics routines until a counter reaches zero.
- `update_objects`: Updates object states in a table by decrementing counters and incrementing animation frames, clearing objects when conditions are met.
- `update_object_position`: Updates object positions in a data structure by modifying velocity values based on acceleration calculations, handling boundary conditions for position values and managing countdown timers.
- `update_position_data`: Searches for an empty slot in a data structure, stores position and velocity values, then initializes additional fields with calculated values.
- `initialize_objects`: Manages object initialization by decrementing timers, calculating values using division and rotation operations, and setting up object properties in a table.
- `frag_addq_move_moveq_mulu_rts`: Short code fragment that adds 1 to the memory at an offset from a5, then copies 100 to the memory at an offset from a5 (2 times in a row), then loads 0 into d0, then copies the memory at an offset from a5 to d0, then multiplies d0 by 3 (unsigned), then copies d0 to the memory at an offset from a5, then returns.
- `copy_data`: Copies data from a lookup table to a destination buffer based on an index, then loads RGB color data into graphics viewports.
- `frag_moveq_move_rol_eori_addi_rts`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a5 to d0, then rotates d0 left by 3, then exclusive-ors 44053 into d0, then rotates d0 left by 2, then adds 991 to d0, then copies d0 to the memory at an offset from a5, then returns.
- `render_score_display`: Manages score display rendering by decrementing a counter and updating character values in a buffer when the counter reaches zero, otherwise converting numeric values to ASCII digits for display.
- `format_and_draw_number`: Converts a byte value to ASCII representation and draws it on screen using graphics functions, then formats a 32-bit number into a string of six characters with leading zeros.
- `format_number`: Converts a 32-bit number into a string of six characters representing decimal digits with leading zeros.
- `blit_data`: Configures graphics hardware registers for bit block transfer operations, then performs multiple blits using data from tables until a negative value is encountered.
- `process_object_collision`: Processes object collision detection by calculating coordinates and adjusting position values based on object properties and boundaries, then stores the resulting position data into a global structure.
- `adjust_vertical_position`: Modifies a vertical position value based on bit flags in a global variable, clamps the value within boundary limits and calls another routine to update sprite positions.
- `cleanup_graphics_resources`: Frees allocated graphics resources including windows, screens, and memory blocks by calling system functions, clearing pointers and handles after releasing each resource.
- `load_graphics_data`: Reads graphic data from files into memory buffers and updates table values based on loaded data.
- `initialize_sprites`: Initializes sprite data by setting up position control pointers, height values, and clearing x/y coordinates before calling a graphics routine to load sprite data.
- `free_sprites`: Frees sprite resources by calling a graphics function and clearing flags in a table for each of four sprites.
- `frag_lea_moveq_movea_jsr_rts`: Short code fragment that loads the address of the memory at an offset from a5 into a1, then loads 0 into d0, then copies a fixed memory address to a6, then calls another routine, then loads the address of the memory at an offset from a5 into a1, then loads 0 into d0, then calls another routine, then returns.
- `fill_tables`: Populates two memory tables with data from a global variable, shifting values left by four bits for the first table and copying a long word value repeatedly for the second.
- `process_graphics_data`: Processes graphics data by performing bit shifts, arithmetic operations, and conditional updates to memory locations while managing sprite positioning and bit manipulation.
- `process_pixel_data`: Processes 90 words of pixel data by copying, negating, and reordering values between multiple memory locations.
- `swap_viewport_data`: Swaps viewport-related data pointers in a structure and waits for a vertical blank interrupt before returning.
- `initialize_graphics_subsystem`: Initializes two graphics viewports with color maps, bitmaps, and raster planes, setting up display structures for rendering.
- `free_graphics_resources`: Frees multiple graphics resources including color maps, cop lists, rasters, and fonts, handling three sets of resources with loop counters.
- `frag_clr_cmpi_beq_addq_bra`: Short code fragment that clears the stack, then compares the memory at a0 with an address, then branches if the last result was zero, then adds 2 to a0, then jumps to another point in the code.
- `decode_compressed_data_2`: Processes compressed data by reading bytes from a source buffer, performing bit manipulation to extract 16-bit values, and copying them to a destination buffer.
- `decode_compressed_data`: Processes compressed data by reading bytes from a source buffer, performing bit manipulation to extract 16-bit values, and copying them to a destination buffer.
- `frag_addq_movea_adda_bra`: Short code fragment that adds 4 to a0, then copies the memory at a0 (advancing a0) to a2, then adds a0 to a2, then jumps to another point in the code.
- `frag_dbra_addq_rts`: Short code fragment that loops back while counting down d4, then adds 2 to a7, then returns.
- `copy_memory_with_offset`: Copies memory blocks with offset calculations, handling both direct and compressed copy operations based on signed byte values.
- `frag_neg_move_dbra_addq_bra`: Short code fragment that negates d1, then copies the memory at a0 to the memory at a1 (advancing a1), then loops back while counting down d1, then adds 1 to a0, then jumps to another point in the code.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `frag_move_subq_dbra_bra`: Short code fragment that copies d3 to d0, then subtracts 1 from d0, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then jumps to another point in the code.
- `frag_cmpa_bgt_moveq_rts`: Short code fragment that compares a1 with a0, then branches if the comparison was greater, then loads 255 into d0, then returns.
- `frag_cmp_bne_addq_moveq_rts`: Short code fragment that compares d0 with the memory at a0 (advancing a0), then branches if the last result was not zero, then adds 4 to a0, then loads 0 into d0, then returns.
- `frag_adda_bra`: Short code fragment that adds the memory at a0 (advancing a0) to a0, then jumps to another point in the code.
- `frag_moveq_move_lsl_andi_lsr_or_rts`: Short code fragment that loads 0 into d0, then copies the memory at a0 (advancing a0) to d0, then shifts d0 left by 4, then copies the memory at a0 (advancing a0) to d0, then masks d0 with 240, then copies the memory at a0 (advancing a0) to d1, then shifts d1 right by 4, then merges d1 into d0, then returns.
- `frag_ror_lsl_move_andi_rts`: Short code fragment that rotates d0 right by 8, then shifts d0 left by 4, then copies d0 to the memory at a0 (advancing a0), then rotates d0 right by 8, then copies d0 to d1, then masks d0 with 240, then copies d0 to the memory at a0 (advancing a0), then shifts d1 left by 4, then copies d1 to the memory at a0 (advancing a0), then returns.
- `frag_movem_move_mulu_bsr_adda_subq_bne`: Short code fragment that saves registers on the stack, then copies d0 to d5, then multiplies d5 by d3 (unsigned), then calls another routine, then adds d5 to a0, then subtracts 1 from d4, then branches if the last result was not zero, then restores saved registers from the stack, then returns.
- `shift_and_compare`: Shifts a word value left by three bits, stores it temporarily, then repeatedly compares the original value with results from another routine until the counter reaches zero.
- `frag_move_addq_subq_bne_movem_rts`: Short code fragment that copies the top of the stack to d0, then adds 1 to d0, then subtracts 1 from d5, then branches if the last result was not zero, then restores saved registers from the stack, then returns.
- `set_bits_in_array`: Sets or clears specific bits in an array of bytes based on bit positions calculated from input parameters, using a loop to process multiple entries.
- `frag_bclr_lea_addq_dbra_movem_rts`: Short code fragment that clears bit d3 of the memory at a0, then loads the address of an indexed entry from a0 into a0, then adds 1 to d2, then loops back while counting down d0, then restores saved registers from the stack, then returns.
- `extract_bits_from_array`: Extracts bits from an array of bytes and returns a bitmask representing the extracted values, processing a fixed number of entries with bit testing.
- `read_joystick_input`: Reads joystick input from hardware registers, processes button states, and stores the combined input state to a global variable.
- `frag_movem_bsr_rts`: Short code fragment that saves registers on the stack, then calls another routine, then restores saved registers from the stack, then returns.
- `process_data_with_tables`: Processes data using lookup tables and bit manipulation operations, performing conditional copying and arithmetic calculations based on input values.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (moveq) and then falls through into the code that follows, with no return of its own.
- `step_byte_with_space_wrap`: Decrements a byte in the data register a counted number of times, replacing a wrapped zero value with a space character.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (movep.w) and then falls through into the code that follows, with no return of its own.

### Fatal Mission 3 (1994)(R. Burrows).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
