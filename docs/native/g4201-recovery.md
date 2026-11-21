# Game 4201 (ladder v1.0 (1991)(t. van der meijden)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04201). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 157 of 157 named (0 author symbols), 157 of 157 described (28 by rule, 129 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 757 of 757 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5513 of 5513 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 20348 |
| data | 23427 |
| padding | 1281 |
| unused | 856064 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Ladder v1.0 (1991)(T. van der Meijden).adf:Ladder | 1 | 2536 | 90 |
| Ladder v1.0 (1991)(T. van der Meijden).adf:LevelEditor | 1 | 2959 | 66 |
| Ladder v1.0 (1991)(T. van der Meijden).adf:bootblock | 1 | 18 | 1 |

## Cracktros (kept, not protection)

- none

## Copy protection found

- `clear_screen_and_process_input` (Ladder): Clears the screen and processes input events in a loop until a specific condition is met, updating a counter at offset 634 of the structure in a4. Kind: manual/code-wheel lookup. Pass path: The routine checks for specific keypresses (e.g., '\x12' or '\x19') and may trigger a lookup or validation based on user input, likely involving a code wheel or manual entry check. The successful path involves handling these inputs to proce.

## Functions


### Ladder v1.0 (1991)(T. van der Meijden).adf:Ladder

- `initialize_game_structure`: Allocates memory for a game data structure, initializes its header fields including a pointer to the exec library and several internal counters, then sets up graphics and file handling components by calling related routines and storing their results in the structure.
- `frag_movem_lea_movea_addq_moveq_move_db`: Short code fragment that saves registers on the stack, then loads the address of the memory at an offset from a4 into a0, then copies the memory at an offset from a4 to a1, then adds 4 to a1, then loads 29 into d0, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then restores saved registers from the stack, then returns.
- `manage_gadget_display`: Adds and manages a gadget display, processes input events until a specific condition is met, then removes the gadget.
- `sort_linked_list`: Compares and swaps elements in a linked list structure, updating pointers to maintain sorted order based on numeric values.
- `handle_input_events`: Processes input events from a buffer, calls library routines for each event, and updates internal counters and data structures.
- `frag_movem_movea_move_lsl_jsr_rts`: Short code fragment that saves registers on the stack, then copies the memory at an offset from a4 to a0, then copies the memory at an offset from a4 to a1, then copies the memory at an offset from a4 to d0, then shifts d0 left by 1, then copies the memory at a4 to a6, then calls another routine, then restores saved registers from the stack, then returns.
- `handle_pause_menu`: Displays pause menu text, handles user input for continue, abort, or screen to back actions, and returns appropriate values.
- `clear_screen_and_process_input`: Clears the screen and processes input events in a loop until a specific condition is met, updating a counter at offset 634 of the structure in a4.
- `frag_move_bsr_rts`: Short code fragment that copies 3 to the memory at an offset from a4, then copies 3320 to the memory at an offset from a4, then calls another routine, then returns.
- `process_text_characters`: Processes each character from a string buffer through a lookup table, performs bit testing on the retrieved data, and writes either the original character or a space to a global buffer.
- `check_input_conditions`: Evaluates input state values against predefined constants, updates memory locations based on matching conditions, and calls another routine when specific criteria are met.
- `copy_tile_data`: Copies 8 bytes of tile data from a source location to a destination in video memory, advancing by 80 bytes each iteration.
- `initialize_collision_data_2`: Initializes various collision-related fields in a global data structure to zero or specific values, then calls two library routines to process linked list elements and update collision data based on object flags.
- `initialize_level_data_4`: Reads level configuration data from a buffer, processes tile information, and initializes global variables with coordinates and tile properties.
- `call_move`: Library stub: loads its arguments and the graphics library base, calls Move and returns its result to the caller.
- `call_text`: Library stub: loads its arguments and the graphics library base, calls Text and returns its result to the caller.
- `setup_graphics_coordinates`: Loads a value from memory, processes it through a subroutine, sets up graphics coordinates for text rendering, and updates memory locations with specific values based on comparison results.
- `initialize_graphics`: Saves all general-purpose and address registers on the stack, calls a graphics library routine to set up display parameters, then restores the registers before returning.
- `display_number_with_padding`: Loads a 32-bit value from a global variable, converts it to a string representation with padding, sets up graphics coordinates for text rendering, and calls the text drawing routine to display the formatted number.
- `load_and_draw_text`: Loads two 16-bit values into registers and calls a graphics routine to draw text at specified coordinates, then increments a pointer and calls another routine with a constant value.
- `setup_text_display`: Configures text display parameters by setting specific coordinate values in a structure, then calls graphics library functions to render text at those coordinates multiple times.
- `update_game_state_3`: Updates various game state variables including counters and coordinates based on conditional logic, then calls multiple subroutines to handle graphics and object updates.
- `update_animation_frame`: Increments animation counters and updates frame data in a sequence, modifying values in a table based on current frame indices.
- `store_coordinates`: Stores coordinate pairs in a memory buffer at specific offsets, incrementing an index counter after each write operation.
- `render_number_sequence`: Processes a loop of 13 iterations, sets up graphics coordinates for text rendering, calls a subroutine to display numbers, and updates memory locations with specific values during each iteration.
- `load_file_data`: Opens and reads file data using DOS library calls, processes file information, and updates internal file handles and buffers.
- `load_save_highscore`: Opens a file named highscore.lad for reading and writing, copies data from a table to a buffer, writes the buffer to the file, then closes the file.
- `load_high_score_file`: Opens a high score file, reads data into memory, closes the file, and processes the read bytes by subtracting values from a routine.
- `open_libraries`: Attempts to open graphics, intuition, and dos libraries by calling a stub routine with specific parameters for each library.
- `initialize_display`: Initializes screen and font resources, loads color palette data, and sets up display structures using graphics and intuition calls.
- `close_display`: Closes open screen and font resources using intuition and graphics library calls, releasing allocated memory.
- `open_window`: Creates a new window by allocating memory, copying window parameters, and calling the intuition library to open the window.
- `frag_movem_move_tst_beq_movea_jsr_rts`: Short code fragment that saves registers on the stack, then copies the memory at an offset from a4 to d0, then tests d0, then branches if the last result was zero, then copies d0 to a0, then copies the memory at an offset from a4 to a6, then calls another routine, then restores saved registers from the stack, then returns.
- `initialize_collision_data`: Initializes collision-related variables in a data structure to zero, then processes a list of objects through two library calls, updating the structure with values from each object and incrementing a counter until no more objects are returned.
- `call_alloc_remember`: Library stub: loads its arguments and the intuition library base, calls AllocRemember and returns its result to the caller.
- `call_free_remember`: Library stub: loads its arguments and the intuition library base, calls FreeRemember and returns its result to the caller.
- `calculate_string_length`: Calculates the length of a null-terminated string by counting characters until it encounters a zero byte, returning the count minus one as a negative value.
- `frag_movem_move_subq_movea_jsr_dbra_rts`: Short code fragment that saves registers on the stack, then copies d1 to d2, then subtracts 1 from d2, then copies the memory at an offset from a4 to a6, then calls another routine, then loops back while counting down d2, then restores saved registers from the stack, then returns.
- `convert_number_to_string_2`: Copies data from one memory location to another, performs unsigned word division operations, handles negative number cases, and converts numerical values to string representations.
- `unsigned_division_loop`: Performs unsigned word division with bit-by-bit refinement, handling both cases where the divisor is zero and when it's non-zero, returning the result in a register.
- `divide_unsigned_words`: Performs unsigned word division with bit-by-bit refinement, handling both cases where the divisor is zero and when it's non-zero, returning quotient and remainder values.
- `convert_number_to_string`: Converts a number into a string representation by repeatedly dividing by ten and storing remainder digits in reverse order, padding with spaces to fill a fixed buffer size.
- `update_random_seed`: Updates a random seed value by doubling it and XORing with a constant if the result overflows, then either swaps the word bytes or multiplies the original value by a parameter depending on whether the parameter is zero.
- `update_tile_data`: Checks conditions related to tile data and updates memory locations with specific values, including setting flags and copying tile information from one location to another.
- `frag_movem_move_subq_bsr_rts`: Short code fragment that saves registers on the stack, then copies 0 to the memory at an offset from a0, then subtracts 1 from the memory at an offset from a4, then copies the memory at an offset from a0 to d0, then copies the memory at an offset from a0 to d1, then copies the memory at an offset from a0 to d2, then calls another routine, then restores saved registers from the stack, then returns.
- `process_linked_list_elements`: Traverses a linked list structure, checking each element's flag and calling another routine if the flag is set, continuing until reaching a null pointer in the list.
- `initialize_rendering`: Sets up initial rendering parameters by storing specific values to memory locations, calls a graphics routine, and updates game state variables with incremented values.
- `lookup_table_value`: Uses two parameters as coordinates to index into a lookup table, retrieves a byte value from that position, and extends it to a word value for return.
- `process_game_objects`: Processes game object data by checking conditions, calling movement routines, updating position flags, and modifying velocity values based on specific input conditions.
- `handle_movement_conditions`: Processes movement state based on input parameters, updates position flags, modifies velocity values according to specific conditions, and calls different subroutines based on comparison results.
- `frag_movem_move_addq_rts`: Short code fragment that saves registers on the stack, then copies the memory at an offset from a3 to the memory at an offset from a3 (2 times in a row), then adds 1 to the memory at an offset from a3, then copies 0 to the memory at an offset from a3, then copies 1 to the memory at an offset from a3, then restores saved registers from the stack, then returns.
- `handle_movement_logic`: Processes movement state based on input parameters, updates position flags, and modifies velocity values according to specific conditions and comparisons.
- `frag_movea_bsr_bra_neg_move_add_movem_r`: Short code fragment that copies a3 to a0, then calls another routine, then jumps to another point in the code, then negates the memory at an offset from a3, then copies the memory at an offset from a3 to the memory at an offset from a3 (2 times in a row), then copies the memory at an offset from a3 to d0, then adds d0 to the memory at an offset from a3, then restores saved registers from the stack, then returns.
- `process_input_state`: Evaluates input state values against predefined constants and calls different routines based on matching conditions, updating memory locations accordingly.
- `frag_move_rts`: Short code fragment that copies 65535 to the memory at an offset from a4, then returns.
- `frag_move_rts_2`: Short code fragment that copies 1 to the memory at an offset from a4, then returns.
- `frag_move_rts_3`: Short code fragment that copies 1 to the memory at an offset from a4, then returns.
- `frag_move_clr_rts`: Short code fragment that copies 65535 to the memory at an offset from a4, then clears the memory at an offset from a4, then copies 11 to the memory at an offset from a4, then returns.
- `frag_move_clr_rts_2`: Short code fragment that copies 1 to the memory at an offset from a4, then clears the memory at an offset from a4, then copies 1 to the memory at an offset from a4, then returns.
- `update_state_machine`: Clears specific state variables in a data structure, then updates a main state variable based on its current value by setting it to predefined constants when matching certain conditions.
- `check_collision`: Tests if a game object intersects with another object by comparing coordinates and updates game state accordingly, otherwise it returns.
- `read_surrounding_tiles`: Calculates an address from two coordinates and a base pointer, then reads multiple bytes from nearby memory locations relative to that address, storing each value in different fields of the data structure.
- `read_neighboring_tiles`: Calculates an address from two coordinates and a base pointer, then reads two specific bytes from memory locations relative to that address, storing each value in different fields of the data structure.
- `process_collision_event`: Processes different collision event types based on a byte value from another routine, updates memory locations accordingly, and handles object placement and state changes.
- `clear_tile_value`: Calculates a video memory address using coordinate values, writes a zero byte to that location, then calls another routine.
- `frag_movem_move_bsr_rts`: Short code fragment that saves registers on the stack, then copies the memory at an offset from a4 to d0, then copies the memory at an offset from a4 to d1, then copies the memory at an offset from a4 to d2, then calls another routine, then restores saved registers from the stack, then returns.
- `evaluate_movement_conditions`: Checks various conditions based on values in a data structure and returns a boolean result indicating whether movement should be allowed, with multiple conditional branches determining the outcome.
- `update_game_state`: Evaluates random values and updates state variables based on comparison results, potentially calling other routines or setting specific memory locations.
- `update_ladder_state_2`: Checks ladder-related game conditions and updates state variables based on tile data and position coordinates, then increments a ladder state counter.
- `update_position_forward`: Checks multiple conditions related to game state and updates position coordinates, then calls another routine for tile data processing before returning a status value.
- `update_position_backward`: Verifies several conditions before updating position coordinates in reverse direction, calls a tile data routine, and sets specific memory values upon completion.
- `update_position_continue`: Evaluates game state conditions and updates position coordinates while setting a zero value in memory, then calls another routine for tile data processing before returning.
- `update_coordinates_and_render`: Compares memory values against specific constants, performs conditional operations based on these comparisons, calls a subroutine to render graphics, and updates coordinate values in memory.
- `update_game_state_2`: Updates game state variables based on conditional checks and calls other routines to handle specific operations, then stores a zero value in a global variable before returning.
- `update_ladder_state`: Updates a ladder state variable based on conditional checks of neighboring tile data and game coordinates, then increments the state variable if it's less than six, otherwise clears both the state and a related coordinate variable.
- `update_position_decrement`: Updates position coordinates by decrementing the vertical coordinate and adjusting the horizontal coordinate based on a tile data copy operation, but only when certain conditions are met.
- `update_position_increment`: Updates position coordinates by incrementing the vertical coordinate and adjusting the horizontal coordinate based on a tile data copy operation, but only when certain conditions are met.
- `update_position_decrement_both`: Updates both position coordinates by decrementing them based on a tile data copy operation, but only when certain conditions are met.
- `update_position_increment_horizontal`: Updates position coordinates by incrementing the horizontal coordinate and decrementing the vertical coordinate based on a tile data copy operation, but only when certain conditions are met.
- `update_position_increment_both`: Updates both position coordinates by incrementing them based on a tile data copy operation, but only when certain conditions are met.
- `update_position_decrement_horizontal`: Updates position coordinates by decrementing the horizontal coordinate and incrementing the vertical coordinate based on a tile data copy operation, but only when certain conditions are met.
- `setup_audio_device`: Allocates memory for an audio device structure, initializes its fields, and opens the audio.device using a library call.
- `initialize_structure`: Initializes a data structure at an offset from a4 register by setting multiple fields including a pointer, a 32-bit value, and several byte values, then calls a function through a function pointer at offset 20 of the structure.
- `conditional_update`: Checks two conditions in a data structure and updates fields accordingly, calling a function twice with different parameters based on these conditions.
- `setup_object_structure`: Sets up an object structure with various fields including pointers, values, and flags, then calls library functions to finalize the setup process.
- `process_structure_fields`: Checks specific fields of a structure and calls library functions based on their values, then performs cleanup operations on other fields.
- `allocate_and_setup`: Allocates memory using a library call, checks if allocation succeeded, and if so, sets up several fields in the allocated structure including a byte, a word, and a long value.
- `clear_structure_fields`: Sets specific fields of a structure to zero or -1 values, then calls a library function with the structure as parameter.
- `initialize_object`: Initializes a data structure with values from registers and global variables, then performs conditional allocation and linking based on a test result.
- `cleanup_and_finalize`: Checks if a field is non-zero, calls a cleanup function if so, sets specific fields to -1 or 255, and then calls other library functions for finalization.

### Ladder v1.0 (1991)(T. van der Meijden).adf:LevelEditor

- `initialize_editor_2`: Initializes the level editor by allocating memory for a structure, setting up pointers and offsets within it, then performing validation checks on level data and initializing display elements.
- `compress_data_stream`: Processes input data to create a compressed output stream by identifying repeated sequences and encoding them with control bytes.
- `store_level_data`: Writes a value from a register to a global variable, then performs a loop of 80 iterations calling another routine and incrementing a counter.
- `translate_bytes`: Reads bytes from a buffer and uses them as indices to look up values in a table, storing the looked-up values back into the same buffer.
- `process_input_bytes`: Reads bytes from a buffer and uses each byte as an index to look up a value in a table, storing the looked-up value back into the same buffer.
- `count_special_bytes`: Counts occurrences of specific byte values in a data stream, incrementing different counters based on which byte value is encountered.
- `count_level_elements`: Iterates through a level data buffer to count occurrences of specific element types, storing the final counts in global variables.
- `show_player_required`: Displays an error message requesting that exactly one player be placed in the level layout.
- `initialize_level_data`: Initializes several fields of a level data structure starting at offset 482 in a4, including setting specific byte and word values, copying a pointer, and calling a library function with parameters derived from the initialized data.
- `initialize_level_data_2`: Initializes several fields of a data structure at offset 482 in a4, including bytes and words with specific values, and sets a pointer to a string constant.
- `initialize_level_data_3`: Initializes several fields of a data structure at offset 482 in a4, including setting bytes and words to specific values, copying a pointer from another location, and calling a library function with parameters derived from the initialized data.
- `initialize_level_buffer`: Initializes a memory buffer by writing zero values to a series of locations, then performs a loop of calls to another routine with incrementing counters until reaching a maximum value.
- `setup_display_elements`: Sets up display elements by calling several graphics routines with specific parameters, then calls an intuition library function to set a pointer, and performs conditional checks based on global values before returning.
- `process_tile_data`: Processes tile data from a buffer by reading bytes and performing conditional operations, storing values in global variables, and calling graphics routines to handle display updates while iterating through a counter.
- `update_level_editor`: Updates level editor parameters based on input events, modifying tile coordinates and element counts, then refreshes the display.
- `draw_level_tile`: Reads a word value from a specific memory location, processes it through a subroutine, then writes fixed coordinate values to graphics library functions before copying data to a buffer.
- `draw_level_element`: Loads a byte value from memory, extends it to a word, processes it through a subroutine, then writes fixed coordinate values to graphics library functions before copying data to a buffer.
- `initialize_editor_window`: Initializes the editor window by clearing the screen, setting up gadgets, and refreshing their display.
- `enable_gadgets`: Enables multiple gadgets by setting their flags and refreshing them on the window interface.
- `disable_gadgets`: Disables multiple gadgets by clearing their flags and refreshing them on the window interface.
- `frag_movem_lea_tst_beq_clr_bra_move_rts`: Short code fragment that saves registers on the stack, then loads the address of the memory at an offset from a4 into a0, then tests an indexed entry from a0, then branches if the last result was zero, then clears an indexed entry from a0, then jumps to another point in the code, then copies 1 to an indexed entry from a0, then restores saved registers from the stack, then returns.
- `initialize_editor_gadgets`: Initializes and displays editor gadgets by adding them to a window, then enters a loop to handle user input and update gadget states based on global variables.
- `initialize_editor`: Initializes the level editor by clearing a global flag, setting up graphics and input handling, then performing validation checks on level data and gadget selections before returning.
- `update_gadget_selection`: Updates the selection state of a gadget in the user interface, clearing previous selections and setting new ones.
- `handle_tile_events`: Reads tile data from memory, performs conditional operations based on the tile type, and calls different routines depending on whether the tile is a door, wall, or other object.
- `validate_coordinates_3`: Checks coordinate values against specific constants and updates position data accordingly, calling another routine to validate coordinates when certain conditions are met.
- `frag_movem_bsr_tst_beq_rts`: Short code fragment that saves registers on the stack, then calls another routine, then tests the memory at an offset from a4, then branches if the last result was zero, then calls another routine, then restores saved registers from the stack, then returns.
- `validate_coordinates`: Reads word values from memory locations, performs bit shifting and comparison operations, and calls subroutines with fixed coordinate values when conditions are met.
- `validate_coordinates_2`: Reads coordinate values from memory locations, performs bit shifting and comparison operations on them, and branches based on the results to call other routines or skip them.
- `handle_input_events_2`: Clears position coordinates in a global structure, reads input state and position values from memory, updates position values based on input, and calls library functions to handle window gadget lists and input processing.
- `update_level_position`: Reads input state and position coordinates from global memory, updates position values based on input, and writes the modified coordinates back to global memory.
- `reset_position_coordinates`: Reads word values from memory locations, performs bit shifting and comparison operations, then copies processed coordinate values to position tracking variables and calls graphics library functions.
- `copy_tile_data_2`: Processes tile data by reading bytes from a source buffer, performing arithmetic operations on coordinates, and copying byte values to destination buffers with specific offsets.
- `call_move_2`: Library stub: loads its arguments and the graphics library base, calls Move and returns its result to the caller.
- `write_tile_to_level`: Calculates memory addresses based on coordinate values, reads data from a lookup table, and writes byte values to two separate memory locations with fixed offsets.
- `write_tile_to_level_v2`: Calculates memory addresses based on coordinate values, reads data from a lookup table, and writes byte values to two separate memory locations with fixed offsets in a different order.
- `copy_level_tile_data`: Copies tile data from a source buffer to two destination buffers at specific offsets, processing 8 bytes per iteration.
- `write_level_byte`: Writes a byte value from a register to a memory address calculated using coordinates and a base pointer, then returns the original coordinates.
- `invert_bytes`: Inverts the bits of bytes in a memory region and advances through it in steps of 80 bytes, repeating 8 times.
- `call_set_a_pen`: Library stub: loads its arguments and the graphics library base, calls SetAPen and returns its result to the caller.
- `call_text_2`: Library stub: loads its arguments and the graphics library base, calls Text and returns its result to the caller.
- `open_libraries_2`: Attempts to open three different Amiga OS libraries by calling a common library function with different parameters and stores the results in global variables.
- `setup_screen`: Initializes screen and font resources by allocating memory, copying data structures, opening a screen, and setting up color palette information.
- `cleanup_resources`: Closes opened screen and font resources if they exist, releasing system resources used for graphics and display.
- `open_window_2`: Allocates memory for a window structure, copies predefined data into it, opens the window using the screen reference, and stores window-related pointers.
- `close_window_and_cleanup`: Checks if a window pointer is valid, calls initialization routines, and closes the window using the intuition library when the initialization succeeds.
- `initialize_data_fields`: Initializes several global variables to zero, then calls a library function to retrieve data, and if successful, copies specific fields from the retrieved data to global variables.
- `call_alloc_remember_2`: Library stub: loads its arguments and the intuition library base, calls AllocRemember and returns its result to the caller.
- `call_free_remember_2`: Library stub: loads its arguments and the intuition library base, calls FreeRemember and returns its result to the caller.
- `count_nonzero_bytes`: Counts non-zero bytes in a string up to a maximum of 255 characters and returns the count as a negative value.
- `frag_movem_move_subq_movea_jsr_dbra_rts_2`: Short code fragment that saves registers on the stack, then copies d1 to d2, then subtracts 1 from d2, then copies the memory at an offset from a4 to a6, then calls another routine, then loops back while counting down d2, then restores saved registers from the stack, then returns.
- `format_number`: Converts a number into a decimal string representation, storing it in memory with leading spaces and returning the original number.
- `initialize_level_objects`: Initializes a series of level object structures with fixed parameters and links them together, setting up their properties and positioning data.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `count_elements`: Counts elements in a linked list starting at a memory location and stores the count in a global variable.
- `save_level_data`: Opens a file for writing, processes level elements through multiple subroutines, writes structured data to a buffer, and closes the file handle after writing.
- `process_level_data`: Reads bytes from a buffer using indices from another buffer, stores looked-up values back into the same buffer, and updates multiple data structures based on loop iterations.
- `copy_and_expand_data`: Copies data from input buffer to output buffer, expanding repeated bytes using a special marker value.
- `load_level_file`: Opens and reads level data from a file, performing various file system operations and memory management.
- `update_position_data`: Updates position coordinates in a data structure based on a flag value, then calls a graphics library function.
- `setup_console_device`: Allocates memory for an object, initializes its fields with specific values, opens a console device using a library call, and sets up a data structure field with the device handle.
- `check_and_call`: Checks a condition in a data structure and calls a library function if the condition is false, then processes two memory locations through conditional checks and subroutine calls.
- `initialize_structure_2`: Initializes various fields of a data structure to specific values and calls a library function.
- `initialize_and_notify`: Initializes fields in a data structure and performs library function calls based on conditional checks.
- `allocate_and_setup_object`: Allocates memory for an object using a library call, then initializes its fields with specific values.
- `create_game_object`: Creates a game object by allocating memory, setting up its properties, and linking it into a data structure.

### Ladder v1.0 (1991)(T. van der Meijden).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
