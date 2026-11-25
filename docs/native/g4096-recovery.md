# Game 4096 (journey to the center of the earth (1988)(u. s. gold)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04096). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802304 top-level bytes in 101 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 23 of 23 units identical (bytes and relocations), 10 raw-word lines |
| Functions named and described | pass, 335 of 335 named (0 author symbols), 335 of 335 described (111 by rule, 152 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2032 of 2032 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 97089 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 18466 of 18466 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1085800 |
| code | 64786 |
| data | 285468 |
| padding | 25770 |
| unused | 340480 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 1 of 2).adf:journey | 4 | 18463 | 333 |
| Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 1 of 2).adf:EH. (raw code file, load address unknown) | 1 | 0 | 0 |
| Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 1 of 2).adf:E8. (raw code file, load address unknown) | 1 | 0 | 0 |
| Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 1 of 2).adf:EI. (raw code file, load address unknown) | 1 | 0 | 0 |
| Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 1 of 2).adf:EO. (raw code file, load address unknown) | 1 | 0 | 0 |
| Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 1 of 2).adf:EF. (raw code file, load address unknown) | 1 | 0 | 0 |
| Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 1 of 2).adf:EG. (raw code file, load address unknown) | 1 | 0 | 0 |
| Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 1 of 2).adf:E1. (raw code file, load address unknown) | 1 | 0 | 0 |
| Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf:EH. (raw code file, load address unknown) | 1 | 0 | 0 |
| Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf:E8. (raw code file, load address unknown) | 1 | 0 | 0 |
| Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf:EI. (raw code file, load address unknown) | 1 | 0 | 0 |
| Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf:EO. (raw code file, load address unknown) | 1 | 0 | 0 |
| Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf:ed. (raw code file, load address unknown) | 1 | 0 | 0 |
| Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf:e9. (raw code file, load address unknown) | 1 | 0 | 0 |
| Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf:J5. (raw code file, load address unknown) | 1 | 0 | 0 |
| Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf:e4. (raw code file, load address unknown) | 1 | 0 | 0 |
| Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf:J1. (raw code file, load address unknown) | 1 | 0 | 0 |
| Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf:EF. (raw code file, load address unknown) | 1 | 0 | 0 |
| Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf:ea. (raw code file, load address unknown) | 1 | 0 | 0 |
| Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf:EG. (raw code file, load address unknown) | 1 | 0 | 0 |
| Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf:E1. (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- journey to the center of the earth (1988)(u. s. gold)(disk 2 of 2)/Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf:J3.: high-entropy data file (7.73 bits/byte), classified as packed data
- journey to the center of the earth (1988)(u. s. gold)(disk 2 of 2)/Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf:J4.: high-entropy data file (7.65 bits/byte), classified as packed data
- journey to the center of the earth (1988)(u. s. gold)(disk 2 of 2)/Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf:ee.: high-entropy data file (7.62 bits/byte), classified as packed data
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- journey to the center of the earth (1988)(u. s. gold)(disk 1 of 2)/Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 1 of 2).adf:EH.: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- journey to the center of the earth (1988)(u. s. gold)(disk 1 of 2)/Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 1 of 2).adf:E8.: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- journey to the center of the earth (1988)(u. s. gold)(disk 1 of 2)/Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 1 of 2).adf:EI.: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- journey to the center of the earth (1988)(u. s. gold)(disk 1 of 2)/Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 1 of 2).adf:EO.: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- journey to the center of the earth (1988)(u. s. gold)(disk 1 of 2)/Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 1 of 2).adf:EF.: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- journey to the center of the earth (1988)(u. s. gold)(disk 1 of 2)/Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 1 of 2).adf:EG.: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- journey to the center of the earth (1988)(u. s. gold)(disk 1 of 2)/Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 1 of 2).adf:E1.: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- journey to the center of the earth (1988)(u. s. gold)(disk 2 of 2)/Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf:EH.: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- journey to the center of the earth (1988)(u. s. gold)(disk 2 of 2)/Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf:E8.: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- journey to the center of the earth (1988)(u. s. gold)(disk 2 of 2)/Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf:EI.: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- journey to the center of the earth (1988)(u. s. gold)(disk 2 of 2)/Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf:EO.: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- journey to the center of the earth (1988)(u. s. gold)(disk 2 of 2)/Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf:ed.: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- journey to the center of the earth (1988)(u. s. gold)(disk 2 of 2)/Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf:e9.: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- journey to the center of the earth (1988)(u. s. gold)(disk 2 of 2)/Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf:J5.: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- journey to the center of the earth (1988)(u. s. gold)(disk 2 of 2)/Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf:e4.: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- journey to the center of the earth (1988)(u. s. gold)(disk 2 of 2)/Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf:J1.: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- journey to the center of the earth (1988)(u. s. gold)(disk 2 of 2)/Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf:EF.: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- journey to the center of the earth (1988)(u. s. gold)(disk 2 of 2)/Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf:ea.: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- journey to the center of the earth (1988)(u. s. gold)(disk 2 of 2)/Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf:EG.: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- journey to the center of the earth (1988)(u. s. gold)(disk 2 of 2)/Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf:E1.: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- journey to the center of the earth (1988)(u. s. gold)(disk 2 of 2)/Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 2 of 2).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- 20 never-loaded region(s), 97089 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 1 of 2).adf:journey

- `frag_jmp`: Short code fragment that jumps to another point in the code.
- `process_directory_entry`: Processes a directory entry by constructing a file path from two string segments and storing the result in a global buffer.
- `process_message_chain`: Iterates through a linked list of messages, calling a routine for each element and then clearing the list head.
- `frag_lea_jsr_rts`: Short code fragment that loads the address of the memory at an offset from a4 into a0, then calls another routine, then loads the address of the memory at an offset from a4 into a0, then calls another routine, then returns.
- `frag_lea_moveq_jsr_move_beq_rts`: Short code fragment that loads the address of a fixed memory address into a1, then loads 0 into d0, then calls another routine, then copies d0 to a fixed memory address, then branches if the last result was zero, then returns.
- `initialize_graphics_data`: Sets up graphics-related data by writing specific values to memory locations pointed to by a global variable, including a word value, a long value, and another long value, then calls a library function with the base address.
- `initialize_graphics_data_2`: Sets up graphics-related data by writing specific values to memory locations pointed to by a global variable, including a word value, a long.
- `update_object_property`: Loads data from a table using an index, increments a counter at the table entry, reads a word value from the loaded address, and calls another routine if the counter is not zero and less than 2.
- `update_game_state`: Loads a value from a global variable, stores it in a local register, and conditionally calls another routine with an immediate value if the loaded value is greater than one.
- `open_graphics_and_intuition`: Opens the graphics and intuition libraries by name through a library call and stores their base pointers in global variables.
- `initialize_game_data`: Initializes global game variables and structures by setting up pointers, clearing counters, and calling initialization routines with specific parameters.
- `initialize_entity_coordinates`: Sets initial x and y coordinates based on entity type, then performs collision detection and rendering setup using calculated position parameters.
- `clear_and_call`: Clears a global variable and makes two consecutive calls to another routine with specific parameters, then returns.
- `frag_clr_pea_jsr_lea_rts`: Short code fragment that clears the stack, then pushes the address of a fixed memory address onto the stack (2 times in a row), then calls another routine, then loads the address of the memory at an offset from a7 into a7, then returns.
- `frag_clr_pea_jsr_lea_rts_2`: Short code fragment that clears the stack, then pushes the address of a fixed memory address onto the stack (2 times in a row), then calls another routine, then loads the address of the memory at an offset from a7 into a7, then returns.
- `convert_string_to_number`: Processes characters from a string, converts them to numeric values using fixed-point arithmetic, and accumulates the results into a short integer.
- `update_movement_coordinates`: Updates position coordinates in a structure based on movement parameters and calls a library function with calculated values, while handling conditional state changes for movement direction.
- `update_object_position`: Calculates and updates object coordinates based on input parameters, then stores the results in the structure at a5.
- `update_coordinates`: Processes object coordinate data by calculating differences and updating position values, then returns a calculated word value.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, removes the stack frame with unlk, then returns to the caller.
- `parse_and_update_coordinates`: Processes a string of space-separated values, converts them to numeric coordinates, updates position data in a structure, and modifies related parameters based on comparison checks.
- `process_coordinates_2`: Calculates an address from a byte value, loads data from that address, and calls another routine with the loaded data and additional parameters.
- `load_and_update_coordinates`: Reads position data from a table using a byte index, updates four coordinate fields in a structure, and performs arithmetic operations on the coordinates before storing the results back.
- `handle_coordinates`: Calculates an address from a word value, loads data from that address, and calls another routine with the loaded data and additional parameters.
- `setup_entity_coordinates`: Initializes entity coordinates by calling a routine to set up x and y positions, then stores the result byte at an offset from a5 register and performs additional coordinate calculations using values from the structure.
- `convert_string_entry`: Processes a string entry by converting characters to numeric values using fixed-point arithmetic and accumulating results into a short integer.
- `update_position_coordinates`: Calculates and updates position coordinates in a structure based on movement parameters, then calls a library function with the calculated values.
- `process_entity_entries`: Processes entity entries by iterating through a table, checking for null terminators, and calling a routine with data from the table entries.
- `process_entity_operations`: Processes entity operations by handling file I/O, managing a loop through entity data, and calling routines to process each entry with specific parameters.
- `write_to_array`: Writes two 32-bit values to an array at calculated offsets, incrementing an index pointer after each write operation.
- `initialize_entity_data`: Initializes entity-related data by writing multiple 32-bit values to a global array at calculated offsets, then performs conditional operations based on a parameter value before returning.
- `frag_link_movea_move_unlk_rts`: Short code fragment that creates a stack frame in a5, then copies the memory at an offset from a5 to a0, then copies the memory at an offset from a0 to the memory at an offset from a5, then copies the memory at an offset from a5 to a1, then copies the memory at an offset from a1 to a0, then copies a0 to d0, then removes the stack frame held in a5, then returns.
- `copy_word_array`: Copies word values from one memory location to another while incrementing counters and updating control bytes during a loop iteration.
- `process_structure_data`: Processes structure data by calling multiple library functions with parameters derived from the input structure, including offset calculations and value passing.
- `frag_movea_move_rts`: Short code fragment that copies a fixed memory address to a0, then copies the memory at an offset from a0 to a fixed memory address, then returns.
- `frag_move_jsr_addq_rts`: Short code fragment that copies a fixed memory address to the stack, then calls another routine, then adds 4 to a7, then returns.
- `initialize_structure`: Initializes fields of a data structure starting at the address in a5, sets up pointers and clears counters, then performs a loop to populate array elements with results from subsequent calculations.
- `process_table_entries`: Iterates through table entries using an index counter, calling a library function with parameters from each entry.
- `update_tile_attributes`: Processes tile data by reading word values from a memory location, performing bitwise operations on them, and writing modified values back to the same location while updating counters and control bytes during a loop iteration.
- `process_tile_data`: Processes tile data by iterating through a table of word values, modifying their bits, and updating the table entries based on bitwise operations.
- `call_library_function`: Calls a library function using a base address stored in a global variable.
- `handle_collision_data`: Processes collision-related data by loading coordinates and parameters from a structure, then calling two library functions with different parameter combinations to handle collision detection and response.
- `call_library_function_2`: Calls a library function using a base address stored in a global variable.
- `call_library_function_3`: Calls a library function using a base address stored in a global variable.
- `conditional_library_call`: Performs a conditional check on a byte value in the structure, then calls a library function with either a value of 1 or 0 depending on whether the condition is met.
- `handle_structure_coordinates`: Processes structure coordinates by loading values from memory locations and calling library functions with calculated parameters.
- `load_and_call_function`: Loads a library base from a global variable and calls a function at a specific offset, passing parameters from the stack and returning its result.
- `call_two_functions`: Calls two different library functions with parameters derived from structure offsets, using a byte value as an index for the first call.
- `copy_memory_blocks`: Copies memory blocks between two arrays in a loop that iterates five times, moving data between pointers with a fixed increment of 78 bytes.
- `frag_link_move_bsr_addq_unlk_rts_2`: Short code fragment that creates a stack frame in a5, then copies the memory at an offset from a5 to the stack, then calls another routine, then adds 4 to a7, then removes the stack frame held in a5, then returns.
- `update_tile_pixels`: Updates pixel data for tiles by copying values from source to destination addresses while incrementing counters, and performs fixed-point arithmetic operations on tile coordinates.
- `copy_word_array_2`: Copies word values from one memory location to another while incrementing counters and updating control bytes during a loop iteration.
- `update_tile_animation`: Processes tile animation states based on a control byte, performing different actions for each state including drawing operations and updating counters, then returns.
- `update_position_bounds`: Processes position coordinates with boundary checks, updates movement flags, and prepares parameters for subsequent rendering or collision routines.
- `shift_array_elements`: Shifts array elements by copying values from one location to another and updating a counter based on conditions related to array bounds.
- `perform_collision_check`: Executes a complex collision detection algorithm by loading data from a table, performing multiplication and division operations on coordinate values, then comparing results to determine if a collision occurred.
- `process_object_data`: Processes object data by calculating differences between coordinates, preparing parameters for another routine, and returning a word value from the processed data.
- `frag_link_movea_move_unlk_rts_2`: Short code fragment that creates a stack frame in a5, then copies a fixed memory address to a0, then copies the memory at an offset from a0 to d0, then removes the stack frame held in a5, then returns.
- `frag_link_movea_move_unlk_rts_3`: Short code fragment that creates a stack frame in a5, then copies a fixed memory address to a0, then copies the memory at an offset from a0 to d0, then removes the stack frame held in a5, then returns.
- `read_joystick_status`: Reads the joystick status byte from CIAA_PRA, performs bit manipulation to extract fire button states, and returns an ORed result of inverted bits.
- `check_global_change`: Calls a library function and returns true if a global variable's value changed during the call.
- `read_joystick_port`: Reads the joystick port movement value, sets a specific word value based on bit tests of that value, then returns the final value.
- `process_joystick_input`: Processes joystick input by performing bitwise operations on the joystick port movement value, comparing bits from different positions, and setting a specific word value based on the comparison results.
- `frag_link_move_andi_unlk_rts`: Short code fragment that creates a stack frame in a5, then copies a fixed memory address to d0, then masks d0 with 255, then removes the stack frame held in a5, then returns.
- `frag_link_move_lsr_unlk_rts`: Short code fragment that creates a stack frame in a5, then copies a fixed memory address to d0, then copies d0 to the memory at an offset from a5, then shifts d0 right by 8, then removes the stack frame held in a5, then returns.
- `compare_and_assign_value`: Compares signed values with thresholds and assigns specific word values based on conditional logic involving arithmetic operations and comparisons.
- `frag_bsr_tst_bne_beq_rts`: Short code fragment that calls another routine, then tests d0, then branches if the last result was not zero, then calls another routine, then tests d0, then branches if the last result was zero, then calls another routine, then tests d0, then branches if the last result was not zero, then returns.
- `frag_jsr_move_rts`: Short code fragment that calls another routine (3 times in a row), then copies 32 to a fixed memory address, then calls another routine, then returns.
- `frag_jsr_move_rts_2`: Short code fragment that calls another routine (3 times in a row), then copies 32800 to a fixed memory address, then calls another routine, then returns.
- `decrement_counter_loop`: Loads a byte value from offset 11 of a local structure, initializes a counter with this value, and enters a loop that decrements the counter while calling another routine.
- `update_collision_flag`: Updates a collision status flag based on comparison results from multiple input values, then returns the updated flag value.
- `frag_link_move_ext_jsr_addq_unlk_rts`: Short code fragment that creates a stack frame in a5, then copies the memory at an offset from a5 to d0, then sign-extends d0, then copies d0 to the stack, then calls another routine, then adds 4 to a7, then removes the stack frame held in a5, then returns.
- `format_digit_string`: Converts a fixed-point division result into ASCII characters and stores them in a buffer, managing a flag to track whether any non-zero digits have been written.
- `update_text_cursor`: Updates a text cursor position and stores ASCII digit characters, while managing a counter and status flag in local variables.
- `clear_table_entry`: The routine stores a zero byte into a memory location computed as the sum of an uninitialized pointer and an offset, effectively clearing a single table entry, then returns without further operations.
- `manage_collision_objects`: Checks a value in a structure and either calls library functions with specific parameters or recursively calls itself with values 0 and 1 to handle different object states.
- `process_collision_data`: Loads data from specific memory locations based on parameter values, calls library functions with that data, and iterates through a counter while processing entries in tables.
- `frag_move_bsr_addq_unlk_rts`: Short code fragment that copies the memory at an offset from a5 to the stack, then calls another routine, then adds 4 to a7, then removes the stack frame held in a5, then returns.
- `check_condition_and_load`: Loads a byte from a table based on a parameter and tests it, returning immediately if the byte is zero or if the parameter is greater than or equal to two.
- `update_collision_properties`: Copies data from source structures to collision object properties, updates coordinates with arithmetic operations, and sets flags in the collision object structure.
- `frag_link_movea_adda_clr_move_bsr_addq`: Short code fragment that creates a stack frame in a5, then copies the memory at an offset from a5 to a0, then adds 953 to a0, then clears the memory at a0, then copies the memory at an offset from a5 to the stack, then calls another routine, then adds 4 to a7, then removes the stack frame held in a5, then returns.
- `load_object_data`: Loads object data from a table using an index, increments a counter at the table entry, reads a word value from the loaded address, stores it in multiple locations within the routine's stack frame, and then calls another routine with several parameters including the read values and the index.
- `initialize_object_data`: Copies data from a structure to a table entry using an index, reads word values from the source structure, stores them in multiple locations within the routine's stack frame, sets specific bytes in the structure to fixed values, and then calls another routine with several parameters including the read values and the index.
- `process_and_clear_entry`: Processes an entry in a table by calling another routine with its contents, then clears the entry's value before returning.
- `iterate_and_process_entries`: Iterates through entries in a table, processes each non-zero entry by calling another routine, clears the entry's value, and increments a counter for each processed entry.
- `copy_and_swap_data`: Copies data between memory locations while swapping byte order, using a global table for source and destination addresses.
- `update_object_properties`: Loads object data from a table using a multiplier, updates multiple structure fields including a counter, status byte, and coordinates, then calls another routine with the object pointer.
- `check_condition`: Reads a value from a global variable, compares it to 32, and stores either 1 or 0 in another global variable depending on whether the comparison is equal.
- `calculate_position_data`: Performs fixed-point arithmetic operations including division and addition, stores intermediate results in local variables, and calls multiple routines with calculated parameters.
- `frag_jsr_rts`: Short code fragment that calls another routine, then returns.
- `initialize_file_handle`: Initializes a file handle by calling a routine to open or create a device file, stores the result in a global variable, and sets a status byte based on whether the operation succeeded.
- `fetch_byte_from_buffer`: Checks if a buffer is empty and reads data from a file handle if needed.
- `compare_string_segments`: Compares characters from two string pointers, incrementing both pointers until a null terminator is found, returning 1 if any characters differ.
- `process_input_loop`: Reads joystick status in a loop until a non-zero value is returned, then decrements a global variable and executes a fixed number of iterations of another routine before returning.
- `open_device_file_handle`: Initializes a file handle with a default value, attempts to open a device file, and retries with different parameters if the first attempt fails.
- `initialize_level_data`: Reads level data from a file handle into a table, processes bit flags for each entry, and updates file position counters.
- `process_game_data`: Iterates through a table of game objects, calculates memory offsets for each entry, copies byte sequences with bit manipulation, and updates pointers and counters in the table entries while managing memory allocation for game objects.
- `iterate_and_read`: Reads data from a file handle four times in a loop, incrementing a counter each time, and stores the result of each read operation in a global variable.
- `decrement_and_branch`: Decrements a counter in a global variable and branches back to the beginning of a loop if the counter is not zero, otherwise it returns.
- `process_data_blocks`: Processes data blocks based on input parameters, modifying memory locations through iterative operations and conditional logic, then returns.
- `execute_search_if_not_null`: Calls another routine with three parameters if a specific global variable is not zero, then returns immediately.
- `parse_bitmap_data`: Reads bitmap header and color map information from a file handle, storing parsed color values in a table and updating global variables during iteration.
- `decode_and_encrypt_data`: Extracts high and low nibbles from input bytes and stores them as separate values.
- `frag_link_movea_adda_move_unlk_rts`: Short code fragment that creates a stack frame in a5, then copies the memory at an offset from a5 to a0, then adds 16956 to a0, then copies the memory at a0 to d0, then removes the stack frame held in a5, then returns.
- `reinitialize_tile_attributes`: Clears and reorganizes tile attribute data by copying word values between memory locations, zeroing specific array elements, and performing multiple library calls to update tile states.
- `process_entity_data`: Initializes entity coordinates and handles collision detection, then processes position boundaries and updates rendering parameters based on calculated coordinates.
- `update_earth_journey`: Processes movement and state changes for an object based on input values, updating counters and flags while managing transition states between different operational modes.
- `initialize_entity`: Sets up entity coordinates and performs collision detection and rendering setup using calculated position parameters, then clears a global variable and makes multiple calls to other routines with specific parameters.
- `setup_entity_render`: Initializes entity rendering by calling several setup routines, performs coordinate calculations, and updates entity state variables before returning.
- `frag_move_add_rts`: Short code fragment that copies a fixed memory address to d0, then copies a fixed memory address to d1, then adds d1 to d0, then copies d0 to a fixed memory address, then returns.
- `update_position_components`: Updates position coordinates and their components with incremental values, performs division operations on the results, and clamps the final values to a maximum of 1000.
- `update_movement_values`: Updates movement-related values based on a global variable and conditional checks against specific constants, then calls another routine before returning.
- `update_entity_coordinates_2`: Updates entity coordinates based on a counter value loaded from a global variable, performs multiple conditional operations and subroutine calls, then stores the result back to the global variable.
- `process_entity_animation`: Processes entity animation states by calculating addresses from byte values, loading data from those addresses, and calling routines with loaded data and additional parameters.
- `update_entity_graphics`: Updates entity graphics by performing coordinate calculations, calling rendering routines with specific parameters, and managing entity state variables through multiple function calls.
- `configure_entity_display`: Configures entity display settings by calling initialization routines, setting up rendering parameters, and updating entity state variables through a series of function calls.
- `process_collision_data_2`: Loads coordinates and parameters from a structure, then calls two library functions with different parameter combinations to handle collision-related data processing.
- `update_player_position`: Updates player position and animation state based on input controls and collision detection results, managing movement flags and boundary checks.
- `update_animation_frame`: Updates an animation frame counter in a local variable, clamps it between 1 and 20, then calls two routines with calculated parameters based on the counter value.
- `normalize_position_coordinates`: Divides a coordinate value by 100, clamps it between 0 and 9, and calls another routine with adjusted parameters for position boundary handling.
- `validate_and_process_coordinates`: Validates coordinate values against thresholds, sets boundary flags, and calls several routines with coordinate data and calculated parameters.
- `update_entity_coordinates`: Calculates and updates entity position coordinates based on type-specific offsets, performs boundary checks, and manages collision detection flags during gameplay.
- `process_entity_coordinates`: Calculates entity coordinates using table lookups and boundary checks, then updates position parameters and collision flags based on the computed values.
- `initialize_level_entities`: Sets up level entity coordinates and performs collision detection, then calls multiple initialization routines with various parameters including tile animation processing and coordinate updates.
- `setup_level_properties`: Initializes level properties by calling several setup routines with specific parameters, clears global variables, and sets initial values for two word-sized memory locations to zero.
- `configure_entity_parameters`: Configures entity parameters by calling initialization routines with specific coordinate and animation values, then performs collision detection setup and calls multiple processing routines.
- `manage_game_objects`: Manages game object states by performing conditional checks on global variables, calling initialization and update routines with various parameters, and handling different game states based on boundary conditions.
- `process_level_rendering`: Processes level rendering by managing entity positions and boundaries, updating coordinates through arithmetic operations, and calling drawing routines with calculated parameters while maintaining state tracking.
- `initialize_level_objects`: Initializes level object data including position coordinates, animation states, and collision properties.
- `update_animation_counters`: Updates animation-related counters and performs conditional operations based on their values, including incrementing and resetting counters with specific thresholds.
- `format_and_display_values`: Formats numerical values into string representations and displays them using a series of library calls that handle string concatenation and output operations.
- `update_tile_animation_state`: Processes tile animation states based on a control byte, performing different actions for each state including drawing operations and updates.
- `update_entity_position`: Updates entity position coordinates and manages associated counters, performing boundary checks and calling rendering routines when conditions are met.
- `update_entity_state`: Updates entity state by performing boundary checks on a value, setting state flags based on comparisons, and calling routines with calculated parameters.
- `handle_entity_animation`: Processes entity animation states based on a control byte, performing different actions for each state including drawing operations and updating display parameters.
- `update_earthquake_data`: Processes earthquake-related data by checking global flags and updating tile animation control bytes, then performs collision detection and updates game state variables based on collision results.
- `update_position_components_2`: Updates position coordinates by adding movement parameters, performs division operations on the results, and calls a library function to handle boundary checks and rendering.
- `initialize_level_data_2`: Reads level data from a file handle into a table, processes bit flags for each entry, and updates file position counters.
- `update_display_coordinates`: Calculates and updates display coordinates by adding values from specific memory locations, then calls a display routine with the computed values.
- `process_entity_actions`: Processes entity actions based on type and state parameters, performing conditional calls to another routine with calculated coordinates and action codes.
- `update_position_and_state`: Updates position coordinates based on a global state variable and performs conditional operations including coordinate adjustments, state resets, and table initialization before returning.
- `update_entity_position_2`: Updates entity coordinates based on direction flags and performs boundary checks, modifying position values and calling routines with calculated parameters.
- `process_entity_collision`: Processes entity collision detection by performing multiple coordinate calculations and calling collision routines, then returns a modified counter value.
- `initialize_entity_data_2`: Initializes entity-related data by writing multiple 32-bit values to a global array at calculated offsets, then performs conditional operations based on entity state and animation counters.
- `initialize_game_state`: Sets up initial game state values including various counters and flags, then initializes an array of structures with zero values.
- `update_game_state_2`: Updates game state variables including counters and flags, initializes structures with zero values, and manages entity rendering setup.
- `update_player_animation`: Manages player animation states and coordinates, updating position values and handling boundary conditions based on movement flags and control bytes.
- `initialize_level_objects_2`: Sets up initial state values for level objects including position coordinates, movement flags, and animation counters.
- `initialize_combat_data`: Initializes combat-related data structures including position coordinates, animation states, and movement flags.
- `format_integer_string`: Converts a signed integer into a string representation, handling negative numbers by adding a minus sign, and returns the length of the resulting string.
- `parse_command_string`: Parses a space, tab, or newline-delimited string into a table of pointers, supporting quoted segments by replacing the closing quote with a null terminator.
- `frag_jmp_2`: Short code fragment that jumps to another point in the code.
- `execute_callback_function`: Stores a function pointer in a global variable, then conditionally executes the stored function if it is not null and not equal to a specific value.
- `open_device_file`: Finds an available file handle, opens or creates a device file based on flags, updates the handle table with file descriptor and position, then returns the handle.
- `prepare_file_flags`: Constructs file access flags by manipulating bits in a parameter value, then calls another routine with the modified flags and other parameters for device file operations.
- `convert_fixed_point_to_string`: Converts a fixed-point number into a string representation using division operations and stores the result in a buffer.
- `copy_string_segment`: Copies a sequence of characters from a source to a destination, ensuring the result is null-terminated.
- `traverse_callback_list`: Iterates through a linked list of structures and calls another routine if specific bitmask conditions are met within the structure, then calls another routine with a parameter.
- `dopen`: Opens an existing directory file using provided parameters, returning an error code if the operation fails.
- `create_directory_file`: Creates a new directory file with extended attributes, returning an error if the file cannot be properly created or initialized.
- `dcreat`: Creates a new file with specific permissions, handling abort conditions and error states during the file creation process.
- `close`: Validates a file handle, clears its status bit 2 if set, and closes the underlying device handle through a system call.
- `append_string_to_buffer`: Scans the destination string to its terminating zero and then copies the source string, including its terminator, starting at that point.
- `initialize_game_state_2`: Initializes a global variable to zero if it's not already set, then copies another global variable's value to a third global variable before returning.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_callback_chain`: Iterates through a list of pointers and executes a routine for each non-zero entry that meets a specific bitmask condition, then calls another routine with two parameters.
- `frag_jmp_3`: Short code fragment that jumps to another point in the code.
- `read`: Validates a file handle, reads data from the underlying device using three parameters, returning the result or an error code.
- `frag_link_tst_beq_jsr_move_addq_moveq_u`: Short code fragment that creates a stack frame in a5, then tests a fixed memory address, then branches if the last result was zero, then calls another routine, then copies the memory at an offset from a5 to the stack, then calls another routine, then adds 4 to a7, then loads 0 into d0, then removes the stack frame held in a5, then returns.
- `frag_movea_cmpi_bne_subq_suba_move_rts`: Short code fragment that copies the memory at an offset from a7 to a0, then compares the memory at a0 (advancing a0) with 0, then branches if the last result was not zero, then subtracts 1 from a0, then subtracts the memory at an offset from a7 from a0, then copies a0 to d0, then returns.
- `process_buffer_entry`: Looks up a value in a table and if found, checks a bit in the returned structure; if set, it calls another routine with specific parameters and returns a result based on a global variable's state.
- `handle_io_error`: Checks a bitmask status and attempts to process a string buffer, potentially calling an error routine and updating a global variable if an error occurs.
- `handle_io_error_2`: Checks a bitmask status and attempts to process a string buffer, potentially calling an error routine and updating a global variable if an error occurs.
- `execute_search_sequence`: Calls another routine to find a value, then calls a second routine with three parameters, returning a specific value if either call returns zero or if a global variable is non-zero.
- `process_pointer_offset`: Processes a pointer offset, adds a constant to another value at an offset, and stores the result back into a memory location.
- `update_offset_pointer`: Retrieves a value from a pointer offset, adds a constant to another value at an offset, and stores the result back into a memory location.
- `frag_link_move_bsr_addq_tst_beq_unlk_rt`: Short code fragment that creates a stack frame in a5, then copies the memory at an offset from a5 to the stack, then calls another routine, then adds 4 to a7, then copies d0 to the memory at an offset from a5, then tests d0, then branches if the last result was zero, then removes the stack frame held in a5, then returns.
- `frag_movea_move_unlk_rts`: Short code fragment that copies an address to a0, then copies a0 to d0, then removes the stack frame held in a5, then returns.
- `clear_and_initialize`: Clears multiple global variables to zero and initializes several data structures.
- `frag_move_jsr_addq_moveq_movem_unlk_rts`: Short code fragment that copies d7 to the stack, then copies a4 to the stack, then calls another routine, then adds 8 to a7, then loads 0 into d0, then restores saved registers from the stack, then removes the stack frame held in a5, then returns.
- `traverse_and_clear_list`: Iterates through a linked list, calling another routine for each element, and then clears the list head.
- `append_node_to_list`: Appends a node to the tail of a doubly linked list whose header holds the head and tail pointers, setting the node's forward link to empty and its back link to the old tail.
- `handle_file_operation`: Handles file operations including seeking and error checking, updating global variables based on operation results.
- `search_table_for_value`: Iterates through a table of 32-bit values to find an entry matching the value in a parameter, returning a pointer to the match or a pointer to the table with a specific value set in a global variable if no match is found.
- `multiply_32bit_words`: Multiplies two 32-bit values taken from the data registers using 16-bit partial products, adding the two cross terms to the low-word product.
- `perform_fixed_point_division`: Performs fixed-point division with handling of negative numbers and returns the result of the division operation.
- `normalize_fixed_point_divide`: The routine divides a 32-bit dividend in d0 by a 32-bit divisor in d1 using a normalized fixed-point method.
- `frag_link_move_bsr_addq_unlk_rts`: Short code fragment that creates a stack frame in a5, then copies the memory at an offset from a5 to d0, then copies d0 to the stack, then copies the memory at an offset from a5 to the stack, then copies d0 to the memory at an offset from a5, then calls another routine, then adds 8 to a7, then removes the stack frame held in a5, then returns.
- `frag_link_movem_move_tst_bgt_moveq_unlk`: Short code fragment that creates a stack frame in a5, then saves registers on the stack, then copies the memory at an offset from a5 to d7, then tests d7, then branches if the comparison was greater, then loads 255 into d0, then restores saved registers from the stack, then removes the stack frame held in a5, then returns.
- `align_and_link`: Processes a value in register d7 by aligning it to a 4-byte boundary, then performs linked list operations using pointers from a global structure.
- `copy_structure_data`: Copies data between structures, adds values from one structure to another, and updates pointers in a global variable before returning.
- `frag_cmpa_bcc_sub_moveq_movem_unlk_rts`: Short code fragment that compares a2 with the memory at an offset from a5, then branches if the carry flag is clear, then subtracts d7 from a fixed memory address, then loads 255 into d0, then restores saved registers from the stack, then removes the stack frame held in a5, then returns.
- `update_linked_list_node_2`: Adds the value from d7 to the word at offset 4 of the structure pointed to by a4, and if the pointer at a4 matches d6, updates the pointer.
- `update_linked_list_node`: Adds the value from d7 to the word at offset 4 of the structure pointed to by a4, and if the pointer at a4 matches d6, updates the pointer at a4 with the value from offset 4 of the structure pointed to by d6.
- `frag_movea_move_bra`: Short code fragment that copies a4 to a3, then copies the memory at an offset from a5 to the memory at an offset from a5, then copies the memory at a4 to a4, then jumps to another point in the code.
- `frag_move_suba_movem_unlk_rts`: Short code fragment that copies a2 to the memory at a3, then subtracts a0 from a0, then copies a0 to the memory at a2, then copies d7 to the memory at an offset from a2, then copies a0 to d0, then restores saved registers from the stack, then removes the stack frame held in a5, then returns.
- `check_bitmask_status`: Calls a library function with a value of 3072 and returns 0 if the result does not have bits 13 or 14 set.
- `frag_bsr_rts`: Short code fragment that calls another routine, then returns.
- `multiply_high_bits_2`: Multiplies the low 16 bits of d0 by the low 16 bits of d5, clears the low word of d0, swaps the halves so the upper 16 bits of the product are returned.
- `multiply_high_bits`: Multiplies the low 16 bits of d0 by the low 16 bits of d5, clears the low word of d0, swaps the halves so the upper 16 bits of the product are returned in the low word of d0, restores d4 and d5 from the stack, and returns.
- `frag_move_add_bhi_eori_rts`: Short code fragment that copies the memory at an offset from a7 to d0, then adds d0 to d0, then branches if the comparison was higher (unsigned), then exclusive-ors an address into d0, then returns.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `frag_movea_move_addq_clr_rts`: Short code fragment that copies the memory at an offset from a7 to a0, then copies a0 to the memory at a0, then adds 4 to the memory at a0, then clears the memory at an offset from a0, then copies a0 to the memory at an offset from a0, then returns.
- `allocate_and_init_message_node`: Allocates a node via the memory routines, fills its type, size, flags and data pointer fields from the arguments, links it into a list or queue, and returns the node pointer.
- `release_message_port`: Removes the port from a list when it is linked, marks it as deleted and frees its signal bit, then releases the 34-byte memory block.
- `frag_move_pea_jsr_addq_rts`: Short code fragment that copies d2 to the stack, then copies the memory at an offset from a7 to d2, then pushes the address of a fixed memory address onto the stack, then copies d2 to the stack, then calls another routine, then adds 8 to a7, then copies the top of the stack to d2, then returns.
- `frag_move_jsr_addq_rts_2`: Short code fragment that copies d2 to the stack, then copies the memory at an offset from a7 to d2, then copies d2 to the stack, then calls another routine, then adds 4 to a7, then copies the top of the stack to d2, then returns.
- `create_ext_io`: Allocates a cleared memory block of a given size for an I/O request, marks its type, stores its size and attaches it to a port.
- `delete_io_request_block`: If the pointer is non-null, marks the request's type and two status fields as cleared, then frees the block using its stored size.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_seek`: Library stub: loads its arguments and the dos library base, calls Seek and returns its result to the caller.
- `call_delete_file`: Library stub: loads its arguments and the dos library base, calls DeleteFile and returns its result to the caller.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_delay`: Library stub: loads its arguments and the dos library base, calls Delay and returns its result to the caller.
- `call_execute`: Library stub: loads its arguments and the dos library base, calls Execute and returns its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `frag_movem_movea_jsr_rts`: Short code fragment that saves registers on the stack, then copies a fixed memory address to a6, then copies the memory at an offset from a7 to a0, then copies several registers to or from memory, then copies the memory at an offset from a7 to a1, then copies several registers to or from memory, then calls another routine, then restores saved registers from the stack, then returns.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_library_vector_306_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_324`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_library_vector_354_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_492`: Library stub: loads a library base from a global variable and calls the library function at offset minus 492 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_540`: Library stub: loads a library base from a global variable and calls the library function at offset minus 540 of that base, returning its result to the caller.
- `call_library_vector_564`: Library stub: loads a library base from a global variable and calls the library function at offset minus 564 of that base, returning its result to the caller.
- `call_library_vector_570`: Library stub: loads a library base from a global variable and calls the library function at offset minus 570 of that base, returning its result to the caller.
- `call_library_vector_576`: Library stub: loads a library base from a global variable and calls the library function at offset minus 576 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_204_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_282`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_indirect_table_entry`: Calls one function through a pointer read from a global variable at offset -348 and returns, with no other operations.
- `prepare_and_combine`: Sets up multiple stack frames and calls several routines with different parameters, then performs bit manipulation operations on word-sized values from the stack to produce a final 32-bit result.
- `process_coordinates`: Processes coordinate data by calling a sub-routine with various parameters, storing results in memory locations, and performing arithmetic operations on floating-point values.
- `prepare_render_data`: Prepares rendering data by shifting and masking values from memory, storing them in stack locations, and calling a sub-routine with specific parameters.
- `initialize_render_params`: Initializes rendering parameters by setting up stack values, calling a sub-routine with different parameter combinations, and performing arithmetic operations on input data.
- `frag_link_moveq_move_cmpi_bcc_asl_jmp`: Short code fragment that creates a stack frame in a5, then loads 0 into d0, then copies the memory at an offset from a5 to d0, then compares d0 with 6, then branches if the carry flag is clear, then shifts d0 left by 2, then jumps to another point in the code.
- `frag_move_nop_unlk_rts`: Short code fragment that copies 5 to a fixed memory address, then copies 57068 to a fixed memory address, then copies 58989 to a fixed memory address, then copies 11 to a fixed memory address, then pauses for one no-operation step, then removes the stack frame held in a5, then returns.
- `multiply_32bit_words_2`: Multiplies two 32-bit values using 16-bit partial products, adds cross terms to the low-word product, and stores results in a structure pointed to by a0.
- `frag_movem_bsr_rts`: Short code fragment that saves registers on the stack, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_movem_bsr_rts_2`: Short code fragment that saves registers on the stack, then calls another routine, then restores saved registers from the stack, then returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (bchg.b) and then falls through into the code that follows, with no return of its own.
- `normalize_coordinates_4`: Processes coordinate values in registers d0 and d2 through bitwise operations and comparisons, adjusting their signs and magnitudes based on specific bit patterns, then performs conditional checks and value swaps before jumping to another routine.
- `frag_cmp_beq_jmp`: Short code fragment that compares d6 with d4, then branches if the last result was zero, then jumps to another point in the code.
- `conditional_swap_operations`: Performs conditional swapping of register values based on test results, with branches that may lead to different execution paths depending on the state of various registers.
- `frag_move_swap_eor_rts`: Short code fragment that copies d2 to d0, then copies d3 to d1, then swaps the halves of d0, then exclusive-ors d6 into d0, then exclusive-ors d7 into d0, then swaps the halves of d0, then returns.
- `frag_swap_bne_tst_eor_rts`: Short code fragment that swaps the halves of d2, then branches if the last result was not zero, then tests d3, then branches if the last result was not zero, then swaps the halves of d0, then exclusive-ors d4 into d0, then exclusive-ors d5 into d0, then swaps the halves of d0, then returns.
- `normalize_coordinates_3`: Adjusts coordinate values by shifting and rotating bits, performing arithmetic operations with carry flags, and calling another routine to process the results.
- `frag_subx_bpl_neg_negx_move_jmp`: Short code fragment that subtracts d2 and the extend flag from d0, then branches if the last result was not negative, then negates d1, then negates d0 with the extend flag, then copies d6 to d4, then jumps to another point in the code.
- `frag_jmp_4`: Short code fragment that jumps to another point in the code.
- `normalize_coordinates`: Normalizes coordinate values by shifting and rotating bits, performing arithmetic operations with carry flags, and calling another routine based on comparison results.
- `frag_swap_add_or_rts`: Short code fragment that swaps the halves of d0, then adds d5 to d0, then merges d4 into d0, then swaps the halves of d0, then returns.
- `swap_registers_and_process`: Exchanges values between multiple data registers, performs conditional checks on register contents, and applies bitwise operations to adjust position-related data before returning.
- `frag_add_addx_tst_bne_swap_bra`: Short code fragment that adds d3 to d3, then adds d2 and the extend flag to d2, then tests d5, then branches if the last result was not zero, then swaps the halves of d0, then adds d1 to d1, then adds d0 and the extend flag to d0, then jumps to another point in the code.
- `frag_tst_bne_jmp`: Short code fragment that tests d0, then branches if the last result was not zero, then tests d1, then branches if the last result was not zero, then jumps to another point in the code.
- `frag_jmp_5`: Short code fragment that jumps to another point in the code.
- `adjust_position_values_3`: Processes position-related data by performing bit manipulation and arithmetic operations on multiple registers, then conditionally adjusts values based on comparison results before returning a modified register combination.
- `frag_addi_swap_add_or_rts`: Short code fragment that adds 16 to d5, then swaps the halves of d0, then adds d5 to d0, then merges d4 into d0, then swaps the halves of d0, then returns.
- `adjust_position_values_2`: Adjusts position values by performing bit operations on data from multiple registers, then calls another routine when certain conditions are met.
- `frag_jmp_6`: Short code fragment that jumps to another point in the code.
- `adjust_position_values`: Adjusts position values by subtracting with extend flags, performing bit operations, and calling another routine when certain conditions are met.
- `frag_cmpi_blt_lsr_roxr_tst_bge_bra`: Short code fragment that compares d0 with an address, then branches if the comparison was less, then shifts d0 right by 1, then rotates d1 right through the extend flag by 1, then tests d5, then branches if the comparison was greater or equal, then jumps to another point in the code.
- `frag_subi_bge_neg_lsr_addi_cmpi_ble_jmp`: Short code fragment that subtracts 16 from d5, then branches if the comparison was greater or equal, then negates d5, then shifts d5 right by 4, then adds 4 to d5, then compares d5 with 57, then branches if the comparison was less or equal, then jumps to another point in the code.
- `manipulate_bit_stream`: Performs bitwise shifts and logical operations on register contents while decrementing a counter.
- `process_bit_operations`: Performs bit shifting and rotation operations on data registers, executes exclusive-or operations, and calls another routine based on comparison results.
- `tiny_eor_swap_rts`: Small routine of 3 instructions that exclusive-ors d4 into d0, then swaps the halves of d0, then returns.
- `frag_movem_bsr_rts_3`: Short code fragment that saves registers on the stack, then calls another routine, then restores saved registers from the stack, then returns.
- `normalize_coordinates_2`: Processes coordinate data in registers d0 and d2, performing bitwise operations and comparisons to normalize values, then branches based on comparison results to handle special cases or call another routine for further processing.
- `frag_cmp_blt_cmpi_bne_tst_beq_move_jmp`: Short code fragment that compares d2 with d7, then branches if the comparison was less, then compares d2 with 32752, then branches if the last result was not zero, then tests d3, then branches if the last result was zero, then copies d2 to d0, then copies d3 to d1, then jumps to another point in the code.
- `frag_jmp_7`: Short code fragment that jumps to another point in the code.
- `check_and_process`: Tests two long registers for zero, conditionally executes a routine with an argument of 3 if they are both zero, then performs a bitwise operation on a word register and returns the result.
- `frag_jmp_8`: Short code fragment that jumps to another point in the code.
- `frag_cmp_blt_cmpi_bne_tst_beq_move_jmp_2`: Short code fragment that compares d2 with d7, then branches if the comparison was less, then compares d2 with 32752, then branches if the last result was not zero, then tests d3, then branches if the last result was zero, then copies d2 to d0, then copies d3 to d1, then jumps to another point in the code.
- `frag_jmp_9`: Short code fragment that jumps to another point in the code.
- `copy_and_branch`: Copies data from multiple source registers to a destination register, performs conditional branches based on the values, and jumps to another location.
- `frag_jmp_10`: Short code fragment that jumps to another point in the code.
- `frag_jsr_bra`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_eor_eori_and_bne_tst_bra`: Short code fragment that exclusive-ors d5 into d0, then exclusive-ors 16 into d0, then masks d7 with d2, then branches if the last result was not zero, then tests d2, then branches if the last result was not zero, then tests d3, then branches if the last result was not zero, then jumps to another point in the code.
- `frag_exg_jsr_bra`: Short code fragment that exchanges d0 and d2, then exchanges d1 and d3, then exchanges d5 and d7, then calls another routine, then exchanges d0 and d2, then exchanges d1 and d3, then exchanges d5 and d7, then jumps to another point in the code.
- `frag_eor_eori_subi_sub_bvc_jmp`: Short code fragment that exclusive-ors d7 into d2, then exclusive-ors 16 into d2, then subtracts 16352 from d7, then subtracts d7 from d5, then branches if the overflow flag is clear, then jumps to another point in the code.
- `divide_and_accumulate`: Performs multiple 16-bit division operations with accumulated results, adjusting values based on comparison outcomes and carrying over overflow.
- `divide_and_accumulate_2`: Performs 16-bit unsigned division on accumulated values and adjusts results based on carry flags, then normalizes coordinates.
- `normalize_coordinate`: Processes coordinate data using multiplication, subtraction, and division operations before passing values to another routine for further handling.
- `frag_cmpi_bge_swap_move_clr_subi_bra`: Short code fragment that compares d0 with 32, then branches if the comparison was greater or equal, then swaps the halves of d0, then swaps the halves of d1, then copies d1 to d0, then clears d1, then subtracts 256 from d5, then branches if the comparison was greater or equal, then jumps to another point in the code.
- `cxren5`: Performs bit manipulation on d0 and d1 to calculate a value, then uses a byte from a table at a calculated offset to perform further bitwise operations and returns.
- `frag_moveq_move_eor_swap_rts`: Short code fragment that loads 0 into d0, then copies d0 to d1, then exclusive-ors d4 into d0, then swaps the halves of d0, then returns.
- `frag_movem_pea_jsr_addq_moveq_eor_swap`: Short code fragment that saves registers on the stack, then pushes the address of a fixed memory address onto the stack, then calls another routine, then adds 4 to a7, then restores saved registers from the stack, then loads 0 into d0, then loads 0 into d1, then exclusive-ors d4 into d0, then swaps the halves of d0, then returns.
- `frag_movem_pea_jsr_addq`: Short code fragment that saves registers on the stack, then pushes the address of a fixed memory address onto the stack, then calls another routine, then adds 4 to a7, then restores saved registers from the stack.
- `frag_move_eor_moveq_swap_rts`: Short code fragment that copies 32752 to d0, then exclusive-ors d4 into d0, then loads 0 into d1, then swaps the halves of d0, then returns.
- `frag_movem_pea_jsr_addq_move_moveq_rts`: Short code fragment that saves registers on the stack, then pushes the address of a fixed memory address onto the stack, then calls another routine, then adds 4 to a7, then restores saved registers from the stack, then copies an address to d0, then loads 0 into d1, then returns.
- `cxnan5`: If bit three of d0 is set, it calls another routine with the value four and clears bit three and sets bit one of d5; otherwise it clears bit three, sets bit one, performs an or with a constant, and returns the swapped result.
- `frag_movem_moveq_bra`: Short code fragment that saves registers on the stack, then loads 0 into d4, then loads 0 into d1, then jumps to another point in the code.

### Journey to the Center of the Earth (1988)(U. S. Gold)(Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
