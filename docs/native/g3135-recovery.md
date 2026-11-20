# Game 3135 (circuit war (1989)(r. reed)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03135). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 9 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 161 of 161 named (52 author symbols), 161 of 161 described (57 by rule, 85 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 888 of 888 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9689 of 9689 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 9035 |
| code | 29298 |
| data | 46393 |
| padding | 2314 |
| unused | 814080 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Circuit War (1989)(R. Reed).adf:CircuitWar | 2 | 9669 | 159 |
| 8 | 1 | 2 | 1 |
| Circuit War (1989)(R. Reed).adf:bootblock | 1 | 18 | 1 |

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Circuit War (1989)(R. Reed).adf:CircuitWar

- `initialize_game`: Initializes game structures and system resources by setting up task-related data, parsing command line arguments, and preparing memory for game operations.
- `cleanup_and_exit`: Cleans up system resources by closing libraries, exiting interrupts, forbidding operations, and replying messages when conditions are met.
- `frag_lea_jsr_rts`: Short code fragment that loads the address of the memory at an offset from a3 into a0, then calls another routine, then loads the address of the memory at an offset from a3 into a0, then calls another routine, then returns.
- `frag_lea_moveq_jsr_move_beq_rts`: Short code fragment that loads the address of the memory at an offset from a4 into a1, then loads 0 into d0, then calls another routine, then copies d0 to the memory at an offset from a4, then branches if the last result was zero, then returns.
- `close_io_resources`: Closes multiple I/O resources including fonts, windows, screens, and libraries using global variables as parameters, then performs a final cleanup operation.
- `initialize_io_resources`: Opens system libraries, disk font, screen, and window resources while handling initialization errors by clearing flags and calling error routines.
- `move_bitmap_region`: Calculates coordinates for bitmap movement, calls a move function with specific parameters, and handles stack management for the operation.
- `calculate_text_coordinates`: Processes bitmap movement coordinates by reading width and height values from a structure, performs fixed-point division calculations, and calls a text rendering function with calculated parameters.
- `validate_stack_pointer_2`: Compares the current stack pointer against a global variable and branches if the stack pointer is below the comparison value.
- `draw_filled_rectangle`: Sets up drawing pen and mode, draws a filled rectangle using coordinates from the stack, then restores previous pen and mode settings.
- `frag_cmpa_bcs_pea_clr_bsr_lea_rts`: Short code fragment that compares a7 with the memory at an offset from a4, then branches if the carry flag is set, then pushes the address of a fixed memory address onto the stack (3 times in a row), then clears the stack, then calls another routine, then loads the address of the memory at an offset from a7 into a7, then returns.
- `manage_menu_strip`: Clears and sets menu strips, handles message processing, and manages window state transitions based on comparison results.
- `handle_message_queue`: Manages message queue operations by waiting for messages, retrieving messages, copying data between buffers, and handling reply operations.
- `compare_and_jump`: Compares a value with entries in a table and jumps to different locations based on the comparison results.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `frag_btst_bra`: Short code fragment that tests bit d0 of d0, then jumps to another point in the code.
- `update_position_data`: Copies data from a structure at an offset from a5 to d0, shifts d0 right by 1, adds 300 to d0, then jumps to another routine.
- `calculate_movement_value`: Reads a word from a structure, stores it in memory, performs bit operations on the value, and returns different results based on comparison conditions.
- `frag_move_bra`: Short code fragment that copies 312 to d0, then jumps to another point in the code.
- `frag_move_lsr_addi_bra_moveq_nop_unlk_r`: Short code fragment that copies the memory at an offset from a5 to d0, then shifts d0 right by 1, then adds 300 to d0, then jumps to another point in the code, then copies 999 to d0, then jumps to another point in the code, then loads 255 into d0, then pauses for one no-operation step, then removes the stack frame held in a5, then returns.
- `handle_window_operations`: Manages window creation, drawing, movement, text rendering, and resource cleanup using Amiga graphics and windowing system calls.
- `update_window_counter`: Updates a window counter value by decrementing it, extending the result to long format, and performing conditional branching based on comparison with a constant.
- `iterate_random_values`: Iterates through random values using a loop that calls a range random function, increments a counter, and performs comparisons to control the iteration.
- `frag_link_cmpa_bcs_move_jsr_addq_unlk_r`: Short code fragment that creates a stack frame in a5, then compares a7 with the memory at an offset from a4, then branches if the carry flag is set, then copies the memory at an offset from a5 to the stack, then calls another routine, then adds 4 to a7, then adds 1 to d0, then removes the stack frame held in a5, then returns.
- `process_table_lookup_2`: Performs indexed table lookups using input parameters to calculate memory addresses, reads byte values from the calculated locations, and calls multiple routines with the retrieved data and additional parameters.
- `calculate_table_index`: Calculates an index into a table using multiplication and addition operations, then performs division on the result before returning.
- `calculate_scaled_value`: Reads a word from a table using indexed addressing, performs fixed-point division by 100, subtracts one from the quotient, and returns the result in a specific format.
- `compute_normalized_value`: Calculates a normalized value from table data using indexed addressing with multiple scaling factors, performs fixed-point division by 100, and returns the quotient and remainder in a packed format.
- `extract_quotient_remainder`: Reads a word from a table using indexed addressing with scaling factors, performs fixed-point division by 100, and returns the quotient and remainder in a swapped register format.
- `update_grid_position`: Calculates grid-based position values using multiplication and addition operations on input coordinates, updates a counter in the structure, and stores the result of a fixed-point division calculation back to memory.
- `process_grid_element`: Handles grid element processing by checking boundary conditions, performing arithmetic operations on coordinate values, and dispatching to different code paths based on input parameters using indexed jumps.
- `draw_image_segment`: Calculates image coordinates by performing multiplication and addition operations on input values, executes fixed-point division calculations, and calls a drawing function with calculated parameters.
- `draw_circuit_segment_2`: Calculates circuit segment coordinates, performs boundary checks, and draws pixel segments based on conditional logic and table lookups.
- `draw_circuit_segment_3`: Calculates and draws circuit segment graphics based on input coordinates and table lookups, performing boundary checks and conditional rendering operations.
- `draw_pixel_line`: Calculates pixel positions by multiplying input coordinates with scaling factors and adding offsets, then draws line segments and individual pixels using graphics commands with specific pen colors.
- `draw_vertical_segments`: Draws vertical graphical segments on screen using rectangle fill operations, handling different drawing modes based on input parameters and performing multiple graphics operations with color settings.
- `draw_vertical_pattern`: Iterates through a counter up to four times, performs conditional arithmetic operations based on input values, and calls a drawing function with scaled parameters to create vertical graphical patterns.
- `render_horizontal_elements`: Renders horizontal graphical elements by iterating through a table of data, drawing rectangles with specific coordinates and colors, and clearing remaining positions with default values.
- `render_text_with_pen`: Sets a graphics pen color, converts a numeric value to ASCII, moves the cursor to a specified position, and renders text using Amiga's text drawing functions.
- `render_text_background`: Sets a graphics pen color, fills a rectangular area with the pen color, converts a numeric value to ASCII, and renders text using Amiga's text rendering system.
- `validate_stack_pointer`: Checks if a global variable's value is less than or equal to the stack pointer, and if so, clears a memory location and performs a graphics operation using predefined coordinates and color settings.
- `draw_line_segment`: Moves to a starting position and draws a line segment to an ending position using graphics commands, setting pen color and performing two separate graphics operations.
- `draw_circuit_segment`: Processes circuit segment coordinates, performs boundary checks, and draws pixel segments based on conditional logic and table lookups.
- `copy_string_buffer`: Copies characters from one buffer to another until reaching a null terminator, then calls another routine with the copied string and returns the address of the destination buffer.
- `calculate_grid_position`: Performs fixed-point arithmetic on coordinates to determine grid position and stores resulting values in global variables.
- `evaluate_conditions`: Tests multiple pairs of short integer values from memory against specific constants and returns zero if any condition matches, otherwise returns one.
- `process_movement_data`: Processes movement parameters from a structure and updates position and status values in global memory based on conditional logic.
- `compare_coordinates`: Compares two coordinate pairs and returns an integer result based on their relative positions, or exits early if a stack pointer check fails.
- `update_sprite_animation`: Updates sprite position and drawing parameters based on random values, manages animation counters, and performs I/O operations for display updates.
- `update_circuit_data`: Processes circuit segment data by performing arithmetic operations on input coordinates, updating table entries with calculated values, and calling drawing routines to render graphics based on the updated data.
- `update_circuit_segment`: Updates circuit segment data by processing coordinates and table entries, performing arithmetic operations and conditional updates based on input parameters and global state.
- `render_pixel_path`: Draws a pixel path on screen by setting pen color, calculating coordinates, and writing pixels at multiple positions with timing delays between each operation.
- `update_circuit_element`: Processes circuit element data by calculating grid positions, updating counters in a table, and modifying segment values based on boundary checks and conditional logic.
- `draw_image_with_pen`: Sets a graphics pen color, converts a numeric value to ASCII, performs image drawing operations, and updates the pen color for subsequent rendering.
- `update_game_state`: Updates game state variables based on input conditions and manages gadget visibility.
- `draw_pixel_grid`: Iterates through a grid of pixel coordinates using nested loops, calling a graphics routine to draw pixels at each position while updating counters and performing arithmetic operations on coordinates.
- `initialize_random_table`: Initializes a table of random values by setting all entries to -1, then fills the table with unique random numbers between 0 and 3 using a loop that checks for duplicates and increments a counter.
- `update_circuit_segments`: Processes circuit segment data in a three-dimensional grid, updating segment states based on calculations and conditions, then performs cleanup operations including resource management and conditional drawing updates.
- `render_game_element`: Handles game element rendering by setting pen colors, selecting images from tables based on state, drawing images, and performing text operations with fixed-point calculations.
- `update_menu_state`: Manages menu item states by checking bit flags, performing conditional operations, and calling menu on/off functions to update visual display based on internal counters.
- `update_menu_strip`: Manages menu strip state by clearing and setting menu structures, handling message processing, and updating grid position values based on input parameters.
- `update_menu_strip_2`: Manages menu strip visibility and pen color settings based on input parameters and global state flags, then performs drawing operations using calculated coordinates and table lookups.
- `manage_gadget_list`: Adds gadgets to a list, manages gadget states through conditional logic, and toggles gadget visibility using Amiga's GUI system calls.
- `update_circuit_display`: Updates circuit segment graphics display by calculating positions, setting drawing parameters, and rendering visual elements based on input coordinates and table lookups.
- `process_movement_data_2`: Processes movement parameters from a structure and updates position and status values in global memory based on conditional logic.
- `update_circuit_element_2`: Processes circuit element data by calculating grid positions, updating counters in a table, and modifying segment values based on boundary conditions and status flags.
- `initialize_circuit_data`: Clears circuit segment data in a three-dimensional table and initializes global variables related to circuit state, then performs a series of operations involving delay and I/O updates while iterating through circuit elements.
- `frag_link_cmpa_bcs_move_pea_jsr_addq_un`: Short code fragment that creates a stack frame in a5, then compares a7 with the memory at an offset from a4, then branches if the carry flag is set, then copies 65538 to the stack, then pushes the address of a fixed memory address onto the stack, then calls another routine, then adds 8 to a7, then removes the stack frame held in a5, then returns.
- `frag_link_cmpa_bcs_pea_move_jsr_addq_un`: Short code fragment that creates a stack frame in a5, then compares a7 with the memory at an offset from a4, then branches if the carry flag is set, then pushes the address of a fixed memory address onto the stack, then copies the memory at an offset from a5 to the stack, then calls another routine, then adds 8 to a7, then removes the stack frame held in a5, then returns.
- `initialize_io_request`: Sets up an I/O request structure with specific values and initiates an asynchronous I/O operation, then waits for its completion before returning a status code.
- `initialize_io_subsystem`: Sets up I/O resources including device ports and memory blocks, initializes global variables for device handles and port references, and configures data structures for communication with hardware devices.
- `update_io_status`: Updates I/O status flags and performs begin/wait I/O operations on a structured data element, then writes multiple values to related structure fields before returning.
- `cleanup_io_resources`: Cleans up I/O resources by closing devices, deleting ports, and freeing memory blocks based on counters stored in global variables, then returns.
- `update_sprite_data`: Updates sprite position and control data in a table based on an index from a global variable, then performs I/O operations using pointers from the table.
- `update_io_status_2`: Reads a byte from a table using an index from a global variable, updates a status field in a structure, and performs I/O operations with multiple table lookups and indexed addressing.
- `call_indirect_table_entry`: Calls one function through a pointer read from a global variable at offset -348 and returns, with no other operations.
- `validate_and_link_node`: Checks if a value is valid and within bounds, then links a node to a list when conditions are met, otherwise sets a global variable to 9.
- `handle_object_collision`: Processes object collision detection by comparing coordinates, calculates differences, and calls another routine with computed values when conditions are met.
- `check_and_forward_call`: Calls a routine with the first argument, and if the result is non-zero calls a second routine using a field of that result with the other two arguments.
- `allocate_memory_block`: Allocates a memory block of specified size plus twelve bytes, stores the block pointer in a global variable, and calls another routine with specific parameters.
- `update_linked_list_entry`: Updates a linked list entry by setting its data pointer, calls another routine with specific parameters, and updates a global pointer if needed.
- `negate_zero_return`: Calls another routine with an argument and returns that value, or returns negative one if the result was zero.
- `free_memory_chain`: Frees memory blocks in a chain by calling the system library function, clears global pointers after processing all elements.
- `update_linked_list_node`: Copies a value from a source pointer into a destination structure, clears the destination's head word, links the destination after the source's tail, and points the source's tail at the destination.
- `convert_integer_to_ascii_2`: Converts a signed 32-bit integer into a null-terminated ASCII string representation, handling negative numbers by setting a minus sign and negating the value.
- `convert_integer_to_ascii`: Converts a numeric value into a null-terminated string of ASCII characters and stores it in a provided buffer.
- `append_string_with_limit`: Appends up to a caller-supplied limit of characters from the source string to the end of the destination string, terminates the result with a null byte, and returns the destination pointer.
- `dispatch_action_by_object_flag`: Looks up an object by identifier, and if its flag byte has bit three set calls a routine with fixed arguments.
- `parse_command_line`: Parses command line arguments from a string, storing up to 32 argument pointers in a global array while skipping whitespace, and initializes game state variables based on parsing results.
- `normalize_fixed_point`: Processes two signed 32-bit values to normalize a fixed-point division result, handling sign changes and bit manipulation for precise arithmetic operations before returning the adjusted value.
- `perform_fixed_point_division`: Executes a complex fixed-point division algorithm that handles different bit patterns and performs multiple arithmetic operations to compute the result.
- `multiply_32bit_words`: Multiplies two 32-bit values taken from the data registers using 16-bit partial products, adding the two cross terms to the low-word product.
- `initialize_device_io`: Initializes device I/O operations by creating port and extended I/O structures, opening a device, performing I/O operations, and cleaning up resources.
- `normalize_fixed_point_division`: Processes two signed 32-bit values to normalize a fixed-point division result, adjusting for bit shifts and handling overflow conditions.
- `process_table_lookup`: Performs iterative table lookups using indexed addressing, subtracting values from an input until a threshold is reached, then updates the input with the remaining value.
- `frag_link_move_moveq_jsr_unlk_rts`: Short code fragment that creates a stack frame in a5, then copies the memory at an offset from a5 to d0, then loads 7 into d1, then calls another routine, then copies d1 to d0, then removes the stack frame held in a5, then returns.
- `copy_memory_backward`: Copies bytes from one memory location to another in reverse order when source is less than destination, otherwise copies forward.
- `append_string_to_buffer`: Scans the destination string to its terminating zero and then copies the source string, including its terminator, starting at that point.
- `process_linked_list_entries`: Iterates through a linked list structure, processes entries based on specific bit conditions, and calls another routine for each valid entry.
- `check_signal_bits_and_handle`: Reads the signal state from the system library, masks out two flag bits, and returns zero when both are clear.
- `frag_bsr_rts`: Short code fragment that calls another routine, then returns.
- `handle_system_signal`: Checks for system signals and handles them by calling library functions, sets a global variable to 5 when conditions are met.
- `process_signal_and_coordinates`: Processes system signals and coordinates based on input parameters, performs different calculations depending on the value of an input parameter.
- `frag_link_tst_beq_jsr_move_movea_moveq`: Short code fragment that creates a stack frame in a5, then tests the memory at an offset from a4, then branches if the last result was zero, then calls another routine, then copies the memory at an offset from a5 to d1, then copies the memory at an offset from a4 to a6, then calls another routine, then loads 0 into d0, then removes the stack frame held in a5, then returns.
- `initialize_game_state`: Initializes a game state structure by opening a library, setting up a buffer, and configuring global variables based on input parameters.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, removes the stack frame with unlk, then returns to the caller.
- `setup_intuition_resources`: Initializes intuition library resources, performs memory operations, sets up global pointers, and calls multiple routines with various parameters.
- `_RangeRan`: Advances the 32-bit RangeSeed linear feedback shift register once per bit of the requested range, stores the new seed, then returns a value scaled to that range in d0.
- `_FastRand`: Reads a 32-bit parameter from the stack and doubles it, and if the doubling produced a carry or a zero result it XORs the value with the constant a fixed address.
- `_BeginIO`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `_NewList`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `_CreatePort`: Obtains a slot index from an unnamed routine and, when valid, allocates a 34-byte block through a library call with flags 65537.
- `_DeletePort`: Releases resources associated with a port by removing it from a list, marking its type byte as 255, setting its long field to minus one, and freeing memory.
- `_CreateExtIO`: Calls a library-backed allocation routine with a second argument, and if it returns a non-null block, stores a byte value, a word from the second argument, and the first argument pointer into fixed offsets of that block.
- `_DeleteExtIO`: If the given pointer is non-null, it sets a byte field to 255 and clears two longword fields of the structure, then passes the pointer and a word field to another routine and returns its result.
- `_OpenDiskFont`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `_Delay`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `_AllocMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `_FreeMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `_FindTask`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `_AllocSignal`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `_FreeSignal`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `_AddPort`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `_RemPort`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `_GetMsg`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `_ReplyMsg`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `_WaitPort`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `_CloseLibrary`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `_OpenDevice`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `_CloseDevice`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `_WaitIO`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `_OpenLibrary`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `_Text`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `_SetFont`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `_CloseFont`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `_LoadRGB4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `_SetRast`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `_Move`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `_Draw`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `_SetRGB4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `_RectFill`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `_WritePixel`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `_SetAPen`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `_SetBPen`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `_SetDrMd`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `_ClearMenuStrip`: Library stub: loads its arguments and the intuition library base, calls ClearMenuStrip and returns its result to the caller.
- `_CloseScreen`: Library stub: loads its arguments and the intuition library base, calls CloseScreen and returns its result to the caller.
- `_CloseWindow`: Library stub: loads its arguments and the intuition library base, calls CloseWindow and returns its result to the caller.
- `_DrawImage`: Library stub: loads its arguments and the intuition library base, calls DrawImage and returns its result to the caller.
- `_OffGadget`: Library stub: loads its arguments and the intuition library base, calls OffGadget and returns its result to the caller.
- `_OffMenu`: Library stub: loads its arguments and the intuition library base, calls OffMenu and returns its result to the caller.
- `_OnGadget`: Library stub: loads its arguments and the intuition library base, calls OnGadget and returns its result to the caller.
- `_OnMenu`: Library stub: loads its arguments and the intuition library base, calls OnMenu and returns its result to the caller.
- `_OpenScreen`: Library stub: loads its arguments and the intuition library base, calls OpenScreen and returns its result to the caller.
- `_OpenWindow`: Library stub: loads its arguments and the intuition library base, calls OpenWindow and returns its result to the caller.
- `_RefreshGadgets`: Library stub: loads its arguments and the intuition library base, calls RefreshGadgets and returns its result to the caller.
- `_SetMenuStrip`: Library stub: loads its arguments and the intuition library base, calls SetMenuStrip and returns its result to the caller.
- `_AddGList`: Library stub: loads a library base from a global variable and calls the library function at offset minus 438 of that base, returning its result to the caller.
- `_RemoveGList`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.

### 8

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.

### Circuit War (1989)(R. Reed).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
