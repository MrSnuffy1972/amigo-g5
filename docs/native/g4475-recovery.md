# Game 4475 (mr. tengus adventure (1989)(pegasus)[h powerslaves]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04475). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (911088 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 111 of 111 named (0 author symbols), 111 of 111 described (31 by rule, 77 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2438 of 2438 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2746 of 2746 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 9826 |
| data | 1248 |
| padding | 942 |
| unused | 899072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Mr. Tengus Adventure (1989)(Pegasus)[h Powerslaves].adf:bootblock | 1 | 14 | 1 |
| logged disk load from Mr. Tengus Adventure (1989)(Pegasus)[h Powerslaves].adf boot-block load | 1 | 70 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 232 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2430 | 102 |

## Cases handled

- mr. tengus adventure (1989)(pegasus)[h powerslaves]/Mr. Tengus Adventure (1989)(Pegasus)[h Powerslaves].adf: the boot block reads 512 bytes from disk offset 788480 to an address (found statically); recovered as a unit
- runtime segment an address: 159 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2279 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- mr. tengus adventure (1989)(pegasus)[h powerslaves]/Mr. Tengus Adventure (1989)(Pegasus)[h Powerslaves].adf: 1 logged loads cover 0 bytes of the image

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_boot_block` (bootblock): Initializes hardware registers and sets up system parameters for boot process, including clearing color registers, configuring DMA settings, and calling library functions to manage system resources. Kind: disk hardware check using CIAB_PRB. Pass path: The routine reads from CIAB_PRB and masks it with 7 (bVar1 & 7), then continues execution. A successful check would proceed without altering control flow, implying the mask result is valid or expected..
- `setup_disk_io` (Mr. Tengus Adventure (1989)(Pegasus)[h Powerslaves].adf boot-block load +an address to an address (512 bytes)): Sets up disk I/O operations by finding the current task, adding a port, opening a device, performing I/O operations, and cleaning up by removing the port and closing the device. Kind: trackdisk.device I/O check. Pass path: The routine successfully opens and performs DoIO calls on the trackdisk device, indicating a valid disk check or load operation. If the disk is valid, it proceeds through the normal execution path without triggering any sabotage or alternat.
- `configure_disk_io` (Mr. Tengus Adventure (1989)(Pegasus)[h Powerslaves].adf boot-block load +an address to an address (512 bytes)): Configures disk I/O parameters for trackdisk device operations, performs read/write I/O requests, and handles device cleanup including port removal and device closure. Kind: track check. Pass path: The routine successfully opens and performs I/O operations on the trackdisk.device, and if it completes without error, the program continues execution (e.g., calls func_0x00025020()). The check likely verifies the presence of a valid disk o.

## Functions


### Mr. Tengus Adventure (1989)(Pegasus)[h Powerslaves].adf:bootblock

- `initialize_boot_block`: Initializes hardware registers and sets up system parameters for boot process, including clearing color registers, configuring DMA settings, and calling library functions to manage system resources.

### logged disk load from Mr. Tengus Adventure (1989)(Pegasus)[h Powerslaves].adf boot-block load

- `setup_disk_io`: Sets up disk I/O operations by finding the current task, adding a port, opening a device, performing I/O operations, and cleaning up by removing the port and closing the device.
- `configure_disk_io`: Configures disk I/O parameters for trackdisk device operations, performs read/write I/O requests, and handles device cleanup including port removal and device closure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `populate_pointer_table`: Populates a table of pointers with longword values, then iterates through the table to clear each memory location pointed to before storing a single byte value into a final global variable.
- `reset_audio_volumes`: The routine writes zero to four global variables and then writes zero to the four audio channel volume registers AUD0VOL through AUD3VOL, effectively silencing all audio channels.
- `disable_dma_and_audio`: Clears a global word, sets four audio volume registers to zero, and writes a value to the DMA control register to enable specific channels.
- `update_audio_parameters`: Updates audio and DMA parameters by reading from tables and structures, performing calculations, and calling another routine multiple times based on conditional checks.
- `update_audio_dma_parameters`: Updates audio and DMA parameters by reading from tables and structures, performing calculations, and calling another routine multiple times to update hardware registers based on input conditions.
- `update_dma_parameters`: Reads a structure from a table, extracts an index from a byte, and uses it to fetch data from another table to update various fields in the structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_bsr_tst_beq_bra`: Short code fragment that calls another routine, then tests a fixed memory address, then branches if the last result was zero, then calls another routine (2 times in a row), then jumps to another point in the code.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `process_dma_data`: Processes DMA data by iterating through a series of memory locations, updating hardware registers with values from lookup tables, and managing DMA channel enable/disable operations.
- `frag_bsr_andi_move_rts`: Short code fragment that calls another routine, then masks d0 with 3, then copies d0 to the memory at an offset from a6, then returns.
- `frag_move_bsr_blt_lea_add_addi_rts`: Short code fragment that copies 65535 to the memory at an offset from a6, then calls another routine, then branches if the comparison was less, then loads the address of a fixed memory address into a1, then adds d0 to d0 (2 times in a row), then adds 500 to an indexed entry from a1, then calls another routine, then returns.
- `handle_input_events`: Processes input events by checking various conditions and calling different routines based on the state of global variables, ultimately setting a specific value in a memory location.
- `update_collision_entry`: Reads a byte value from a lookup table using index D7, multiplies it by 50, and stores the result at offset 44 of a structure.
- `update_memory_value_1`: Updates a memory value at a calculated offset in a table when specific conditions are met, adding a fixed constant to the target memory location based on input parameters.
- `check_and_update_table`: Checks if a register value matches a specific condition, then updates a table entry with an offset value.
- `update_memory_value_3`: Updates a memory value at a calculated offset in a table when specific conditions are met, decrementing a counter and setting a new value when the counter reaches zero.
- `frag_move_rts`: Short code fragment that copies 500 to a fixed memory address, then returns.
- `frag_lea_add_adda_bsr_andi_mulu_rts`: Short code fragment that loads the address of a fixed memory address into a0, then adds d0 to d0 (2 times in a row), then adds d0 to a0, then calls another routine, then masks d0 with 15, then multiplies d0 by 10 (unsigned), then adds d0 to the memory at a0, then returns.
- `frag_clr_rts`: Short code fragment that clears a fixed memory address, then returns.
- `frag_moveq_tst_beq_lea_lsl_move_rts`: Short code fragment that loads 2 into d1, then tests d0, then branches if the last result was zero, then loads 3 into d1, then loads the address of a fixed memory address into a0, then shifts d1 left by 6, then copies 100 to an indexed entry from a0, then returns.
- `process_animation_frame`: Handles animation frame processing by decrementing a counter, updating coordinate values from a table, and calling another routine when certain conditions are met.
- `frag_lea_move_bsr_bset_rts`: Short code fragment that loads the address of a location in the code into a0, then copies an indexed entry from a0 to d1, then calls another routine, then sets bit d1 of the memory at a0, then returns.
- `frag_lea_move_bsr_bclr_rts`: Short code fragment that loads the address of a location in the code into a0, then copies an indexed entry from a0 to d1, then calls another routine, then clears bit d1 of the memory at a0, then returns.
- `frag_move_clr_bsr_rts`: Short code fragment that copies 256 to a fixed memory address, then copies 56192 to a fixed memory address, then clears a fixed memory address, then copies 51560 to a fixed memory address, then calls another routine, then returns.
- `render_score_display`: Processes score display data by copying word-sized values from lookup tables to a destination buffer, handling string termination and rendering character glyphs.
- `frag_move_lea_dbra_rts`: Short code fragment that copies 1599 to d0, then loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then returns.
- `handle_input_and_render`: Processes input data from joystick or mouse by examining bit patterns, then renders text glyphs by copying pixel data from a font table into a destination buffer.
- `frag_move_rts_2`: Short code fragment that copies an address to a fixed memory address, then copies d0 to a fixed memory address, then copies 32767 to a fixed memory address, then returns.
- `read_keyboard_input`: Reads keyboard serial data when an interrupt is pending, performs bit manipulation on the data, updates hardware control registers, and applies a timing delay before clearing a bit in another control register.
- `manage_tile_data`: Manages tile data by checking bit flags, performing calculations on memory values, and updating tile parameters in tables based on conditional comparisons.
- `copy_tile_bytes`: Copies multiple bytes from a source table to a destination buffer based on a calculated index, processing data in a loop with decrementing counter.
- `build_display_list`: Constructs a display list entry by combining various data values and storing them in memory locations, with a loop that processes 80 iterations.
- `frag_addq_moveq_divu_swap_addi_move_clr`: Short code fragment that adds 6 to a0, then loads 5 into d1, then divides d0 by 10 (unsigned), then swaps the halves of d0, then adds 48 to d0, then copies d0 to the memory just below a0 (moving a0 down), then clears d0, then swaps the halves of d0, then loops back while counting down d1, then returns.
- `render_numeric_value`: Processes numeric values for display by dividing by ten, converting to ASCII characters, and copying pixel data from a font table into a destination buffer.
- `update_counters`: Updates two global variables by decrementing them and resetting them to specific values based on bitwise conditions checked after calling another routine.
- `copy_structured_data`: Copies structured data from one memory location to another using calculated offsets, processing 22 iterations with specific memory access patterns.
- `update_sorted_value`: Updates a value in a sorted array by comparing with another value and inserting a new value if needed, maintaining order.
- `insert_sorted_value`: Inserts a value into a sorted array at the correct position while maintaining order, shifting elements and returning an index.
- `frag_move_rts_3`: Short code fragment that copies d0 to the memory at an offset from a0 (8 times in a row), then returns.
- `configure_sprite_dma`: Sets up sprite DMA configurations by initializing blitter registers with specific values, copying data between memory locations, and configuring coprocessor parameters.
- `initialize_rendering_data`: Initializes rendering data by loading a pointer from a global variable, checking if it's null, and setting up blitter registers for pattern data transfer operations.
- `blit_pattern_data`: Configures blitter registers for pattern data transfer operations, performs multiple blitter operations with calculated parameters, and updates a global counter.
- `setup_rendering_parameters`: Configures rendering hardware registers with specific values, performs busy-wait operations, and handles conditional updates to global variables based on input data.
- `frag_movea_moveq_addq_tst_bne_sub_rts`: Short code fragment that copies a0 to a1, then loads 255 into d2, then adds 1 to d2, then tests the memory at a1 (advancing a1), then branches if the last result was not zero, then subtracts d2 from d0, then returns.
- `process_input_and_update`: Reads keyboard input when an interrupt is pending, processes the data, and updates counters while managing a lookup table for character rendering.
- `update_animation_state`: Decrements a counter and toggles a bit flag when the counter reaches zero, then performs coordinate adjustments based on memory values and bit tests.
- `load_character_data`: Loads character data pointers from a global variable, clears a counter, and configures rendering parameters based on a condition.
- `render_character_glyphs`: Processes character data to render glyphs by copying word-sized values from a lookup table to a destination buffer, handling string termination.
- `render_text_glyphs`: Processes character codes from input data to render text glyphs by copying pixel data from a font table into a destination buffer, handling special control characters and mapping ASCII values to font indices.
- `initialize_grid_edges`: Initializes the edges of a grid structure by setting boundary bytes to -1 and marking specific positions with bit flags.
- `initialize_rendering_structure`: Clears multiple global variables, sets up rendering parameters by calling various routines, and performs a loop operation with specific register values.
- `update_sprite_data`: Calculates sprite position and size parameters based on input, then stores these values in a table.
- `configure_display_elements`: Configures various display elements by setting up blitter operations, initializing grid structures, and performing multiple rendering operations with calculated parameters.
- `frag_moveq_bsr_add_dbra_rts`: Short code fragment that loads 14 into d0, then calls another routine (2 times in a row), then adds d1 to the memory at a6, then loops back while counting down d0, then returns.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `scale_coordinates`: Scales coordinate values by multiplying them with a register value, performing division with a fixed divisor, and storing the results in a table.
- `frag_lea_moveq_move_dbra_bsr_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads 7 into d1, then copies d0 to the memory at a0 (advancing a0), then loops back while counting down d1, then calls another routine, then returns.
- `frag_bsr_move_lsl_addi_rts`: Short code fragment that calls another routine, then copies d0 to d1, then shifts d1 left by 8, then adds 11265 to d1, then copies d1 to a fixed memory address, then returns.
- `frag_move_lea_clr_moveq_bsr_dbra_rts`: Short code fragment that copies 256 to a fixed memory address, then loads the address of a fixed memory address into a0, then clears a fixed memory address, then loads 4 into d1, then copies a0 to a fixed memory address, then copies 1300 to a fixed memory address, then calls another routine, then loads the address of the memory at an offset from a0 into a0, then loops back while counting down d1, then returns.
- `configure_blitter_dma`: Sets up multiple blitter operations with specific DMA configurations, including source and destination addresses, sizes, and control registers.
- `setup_blitter_operations`: Sets up multiple blitter operations by configuring control registers, initializing data structures, and performing several blitter DMA operations with specific parameters.
- `initialize_display_system`: Initializes display system variables by clearing multiple memory locations, setting up interrupt handlers, and configuring blitter registers with fixed values.
- `decrement_and_render`: Decrements a global variable and performs conditional rendering operations based on its value, including handling different input states and updating display data.
- `setup_rendering_context`: Sets up rendering parameters by loading specific addresses into memory locations, clearing several variables, and calling multiple initialization routines for graphics operations.
- `frag_tst_bne_jsr_move_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then calls another routine, then copies 255 to a fixed memory address, then returns.
- `validate_coordinate_range`: Performs a series of comparisons and arithmetic operations to validate coordinate values against specific thresholds and returns adjusted results.
- `initialize_graphics_system`: Initializes graphics system by clearing multiple memory locations, setting up interrupt handlers, configuring blitter registers, and calling initialization routines for display operations.
- `build_display_lists`: Constructs display lists by processing input data, copying values between memory locations, and calling other routines based on comparison results with specific thresholds.
- `determine_movement_direction`: Compares coordinates from two data structures to determine a movement direction value based on relative position comparisons.
- `compare_coordinates`: Compares coordinate pairs between two structures and returns specific values indicating match conditions or differences.
- `frag_move_add_roxl_rts`: Short code fragment that copies a fixed memory address to d0, then adds a fixed memory address to d0, then adds the top of the stack to d0, then adds a fixed memory address to d0, then rotates d0 left through the extend flag by d0, then copies d0 to a fixed memory address, then returns.
- `configure_blitter_registers`: Configures blitter registers for pattern data transfer operations, performs multiple blitter operations with calculated parameters, and updates address pointers.
- `frag_move_lea_lsl_adda_rts`: Short code fragment that copies d0 to the stack, then loads the address of a fixed memory address into a6, then copies d7 to d0, then shifts d0 left by 6, then adds d0 to a6, then copies the top of the stack to d0, then returns.
- `setup_blitter_graphics`: Configures blitter registers for graphics rendering operations including size, address, and control parameters while clearing memory areas.
- `process_table_entry`: Reads a word value from a table, compares it with a register value, and updates the table entry if they match.
- `initialize_collision_data`: Initializes collision detection data by clearing a table, setting up coordinate calculations, and performing conditional operations based on input values.
- `manage_game_state`: Manages game state by updating counters, checking conditions against global variables, and calling other routines based on input values from a memory location.
- `frag_eori_move_addq_cmpi_blt_moveq_andi`: Short code fragment that exclusive-ors 1 into a fixed memory address, then copies a fixed memory address to d0, then adds 1 to d0, then compares d0 with 3, then branches if the comparison was less, then loads 0 into d0, then copies d0 to a fixed memory address, then adds 1 to a fixed memory address, then masks a fixed memory address with 3, then returns.
- `frag_move_lea_clr_dbra_rts`: Short code fragment that copies 419 to d0, then loads the address of a fixed memory address into a0, then clears the memory at a0 (advancing a0), then loops back while counting down d0, then returns.
- `process_pattern_data`: Processes pattern data by clearing flags, calculating offsets, and performing blitter operations for multiple iterations, then clears additional flags based on conditional checks.
- `update_animation_counters`: Processes animation data by reading values from a table, decrementing counters, and updating memory locations based on comparison results with specific thresholds.
- `process_animation_frame_2`: Updates animation frame data based on input parameters and handles coordinate adjustments, then returns a modified frame counter.
- `load_animation_data`: Loads animation frame data from lookup tables based on input parameters and stores the results in structure fields.
- `handle_collision_response`: Processes collision responses by updating coordinates, checking bit flags, and modifying table entries based on comparison results and conditional branches.
- `calculate_collision_flags`: Calculates collision detection flags by comparing multiple byte values against a reference and setting bits based on comparison results.
- `setup_blitter_pattern_data`: Calculates blitter pattern data transfer parameters, configures hardware registers with specific values, and performs multiple blitter operations using calculated addresses.
- `setup_blitter_pattern`: Configures blitter pattern data transfer operations by calculating address offsets, setting up control registers, and performing blitter operations with specific parameters.
- `decode_joystick_input`: Decodes joystick or mouse input data by examining bit patterns and returning specific direction values based on bitwise operations.
- `handle_joystick_input`: Processes joystick input by calculating a value based on register contents, storing it in a memory location, and performing conditional operations based on bit tests.
- `frag_lea_move_lsl_add_adda_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies the memory at an offset from a6 to d0, then shifts d0 left by 6, then adds the memory at an offset from a6 to d0, then adds d0 to a0, then returns.
- `update_counter_and_compare`: Updates a global variable and performs conditional comparisons against shifted and offset values from another global variable, modifying the first variable based on comparison results.
- `copy_and_sort_pairs`: Copies word pairs from one memory location to another while performing bit shifts on the first word, then sorts the resulting data in a specific order.
- `process_collision_data`: Reads collision data from a fixed memory location and processes it based on comparison results with predefined values, updating counters and calling other routines conditionally.
- `clamp_coordinate`: Clamps a coordinate value between two bounds based on comparison with a global variable, returning either the clamped value or a default result.
- `render_sprite`: Configures blitter registers for graphics rendering operations including size, address, and control parameters while clearing memory areas, then performs multiple blitter operations to render a sprite using data from global tables.
- `configure_blitter_and_wait`: Sets up blitter parameters for two separate operations by configuring control and address registers, then waits for each operation to complete using a routine that polls the blitter status.
- `blitter_multiple_operations`: Performs multiple sequential blitter operations with varying parameters, including address calculations and repeated wait operations.
- `initialize_tables`: Initializes several memory tables by copying data from fixed locations and clearing specific memory regions.
- `calculate_and_store_coordinates`: Calculates coordinate values based on input parameters, stores them in a structure, then performs additional bit manipulation and conditional operations.
- `frag_movem_btst_beq_move_addq_bsr_rte`: Short code fragment that saves registers on the stack, then tests bit 4 of a fixed memory address, then branches if the last result was zero, then copies 16 to a fixed memory address, then adds 1 to a fixed memory address, then calls another routine, then restores saved registers from the stack, then returns from the exception.
- `update_pointer_references`: Updates pointer references in global variables based on a condition check, then returns immediately if the condition is not met.
- `fill_memory_with_calculated_values`: Calculates values using multiplication and addition operations, then stores these values in memory locations with specific offsets.
- `perform_multiple_blitter_operations`: Executes multiple blitter operations with incrementing addresses, waiting for each operation to complete before proceeding to the next one.
- `process_and_store_data`: Processes byte data from a source location, stores results in a destination table, and updates counters based on specific conditions.
- `configure_blitter_operations`: Sets up blitter registers with specific values for pattern data transfer operations, then performs multiple blitter operations with calculated parameters and updates.
- `frag_move_clr_bsr_rts_2`: Short code fragment that copies 496 to a fixed memory address, then clears a fixed memory address (2 times in a row), then copies a1 to a fixed memory address, then copies 4128 to a fixed memory address, then calls another routine, then returns.
- `frag_lea_moveq_move_dbra_rts`: Short code fragment that loads the address of a location in the code into a0, then loads the address of a fixed memory address into a1, then loads 15 into d0, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then returns.
- `frag_lea_moveq_move_addq_dbra_rts`: Short code fragment that loads the address of a location in the code into a0, then loads the address of a fixed memory address into a1, then loads 31 into d0, then copies 384 to d1, then copies d1 to the memory at a1 (advancing a1), then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then adds 2 to d1, then loops back while counting down d0, then returns.
