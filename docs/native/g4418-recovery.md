# Game 4418 (miniblast v1.0 (1990)(a. bjerin)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04418). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 15 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 15 raw-word lines |
| Functions named and described | pass, 413 of 413 named (37 author symbols), 413 of 413 described (263 by rule, 135 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2467 of 2467 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 82644 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9029 of 9029 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 105170 |
| code | 29586 |
| data | 99133 |
| padding | 5215 |
| unused | 662016 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Miniblast v1.0 (1990)(A. Bjerin).adf:Miniblast | 20 | 9011 | 412 |
| Miniblast v1.0 (1990)(A. Bjerin).adf:bootblock | 1 | 18 | 1 |
| Miniblast v1.0 (1990)(A. Bjerin).adf:Level4.dat (raw code file, load address unknown) | 1 | 0 | 0 |
| Miniblast v1.0 (1990)(A. Bjerin).adf:Level2.dat (raw code file, load address unknown) | 1 | 0 | 0 |
| Miniblast v1.0 (1990)(A. Bjerin).adf:Level3.dat (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- miniblast v1.0 (1990)(a. bjerin)/Miniblast v1.0 (1990)(A. Bjerin).adf:Level4.dat: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- miniblast v1.0 (1990)(a. bjerin)/Miniblast v1.0 (1990)(A. Bjerin).adf:Level2.dat: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- miniblast v1.0 (1990)(a. bjerin)/Miniblast v1.0 (1990)(A. Bjerin).adf:Level3.dat: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 3 never-loaded region(s), 82644 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Miniblast v1.0 (1990)(A. Bjerin).adf:Miniblast

- `initialize_system`: Initializes system structures by clearing memory, setting up task management variables, and preparing for message handling.
- `shutdown_system`: Performs system shutdown by calling library functions to close libraries, exit interrupts, forbid operations, and reply messages, then returns the input parameter.
- `frag_lea_moveq_jsr_move_beq_rts`: Short code fragment that loads the address of a location in the code into a1, then loads 0 into d0, then calls another routine, then copies d0 to the memory at an offset from a4, then branches if the last result was zero, then returns.
- `frag_jmp`: Short code fragment that jumps to another point in the code.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `validate_and_process_pointer`: Saves several registers to the stack, retrieves parameters from the stack, calls another routine with those parameters, and then performs conditional operations based on a bit test of a structure field before returning a value.
- `initialize_structure`: Initializes a data structure by setting multiple fields to zero, then performs conditional operations based on flags and pointer values before returning.
- `append_byte_to_buffer`: Appends a byte value to a managed buffer structure, updating internal pointers and handling buffer capacity constraints based on flags and size limits.
- `parse_data_stream_structure`: Iterates through a data buffer by advancing pointers and evaluating type tags to manage state flags.
- `frag_movea_jsr_pea_ori_moveq`: Short code fragment that copies the memory at an offset from a4 to a7, then calls another routine, then pushes the address of a fixed memory address onto the stack, then calls another routine, then merges 0 into d0 (2 times in a row), then loads 97 into d0.
- `compare_c_strings`: Compares two null-terminated strings byte by byte.
- `frag_movea_move_tst_bne_subq_rts`: Short code fragment that copies the memory at an offset from a7 to a1, then copies the memory at an offset from a7 to a0, then copies a0 to d0, then tests the memory at a0 (advancing a0), then branches if the last result was not zero, then subtracts 1 from a0, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then branches if the last result was not zero, then returns.
- `format_signed_int_as_decimal_string`: Converts the signed long in the second argument to decimal digits on the stack, prepends a minus sign for negatives, copies the digits into the caller's buffer, null-terminates it and returns the length.
- `insert_into_list`: Inserts a memory block into a sorted linked list, manages block sizes and global counters, handles boundary conditions and overlaps, and updates pointers to maintain list integrity.
- `frag_movem_movea_move_jsr_addq_rts`: Short code fragment that saves registers on the stack, then copies the memory at an offset from a7 to a3, then copies the memory at an offset from a7 to d7, then copies d7 to the stack, then copies a3 to the stack, then calls another routine, then adds 8 to a7, then restores saved registers from the stack, then returns.
- `validate_and_read_data`: Saves registers on the stack, retrieves parameters from the stack, calls another routine to read data, checks a global error flag, and returns either a valid pointer or -1 based on the operation result.
- `manage_file_operations`: Initializes local variables and global error status, performs loop iterations with conditional checks, manipulates file handle values through bitwise operations, and calls multiple file system routines to open, lock, and manage files.
- `validate_and_seek_file`: Saves registers on the stack, retrieves parameters from the stack, calls another routine to read data, performs a file seek operation, checks for errors in global status, and returns either a valid file position or -1.
- `allocate_memory_block_2`: Allocates a memory block of specified size by first checking available space, then calling another routine to perform the actual allocation and initialization.
- `frag_move_jsr_addq_rts`: Short code fragment that copies d7 to the stack, then copies the memory at an offset from a7 to d7, then copies d7 to the stack, then calls another routine, then adds 4 to a7, then copies the top of the stack to d7, then returns.
- `read_bytes_until_eof_into_buffer`: Copies up to a requested count of bytes from a stream buffer into the destination pointer, refilling via the stream read routine and stopping early on end-of-file, returning the number of bytes stored.
- `initialize_structure_2`: Initializes a data structure by allocating memory, clearing it, and linking it into a global list.
- `process_file_access_operations`: Analyzes command strings to determine file access modes, calls a file handling routine with specific parameters, and updates various structure fields including pointers and bitmask values based on the analysis.
- `check_node_flags_and_run_children`: Saves d6-d7/a2-a3, reads a node pointer from the stack, tests bit 1 of its flags at offset 27, optionally calls a routine with the node and minus one, then calls two routines on its fields and returns zero or minus one.
- `process_active_nodes`: Processes active nodes in a collection by checking specific bit flags, calculating a difference value, and calling another routine with calculated parameters when conditions are met.
- `write_data_to_file`: Writes data to a file using the Amiga DOS library, updates error status if the operation fails, and sets a timeout value in a global variable.
- `seek_file_position`: Seeks to a specific position in a file using the Amiga DOS library, handles error conditions by updating error status, and calculates new file positions based on input parameters.
- `read_data_from_file`: Reads data from a file using the Amiga DOS library, updates error status if the operation fails, and sets a timeout value in a global variable.
- `open_file_handle`: Opens a file using the Amiga DOS library, updates error status if the operation fails, and sets a timeout value in a global variable when opening fails.
- `delete_std_io`: Sets specific fields in a structure to fixed values and then frees the memory allocated for that structure.
- `free_msgport_signal_and_memory`: Removes the message port from the exec list if its name is set, marks it invalid, frees its signal bit and releases its thirty-four byte memory block.
- `lock_directory`: Locks a directory using the Amiga DOS library, opens a file if locking fails, handles error conditions by updating error status, and returns appropriate file handles.
- `delete_and_open_file`: Deletes a file if it can be locked, opens a file for reading using the Amiga DOS library, handles error conditions by updating error status, and returns appropriate file handles.
- `close_file_handle`: Checks if a global file handle is valid, calls another routine to close it using the Amiga DOS library, and returns zero after cleaning up the stack.
- `initialize_intuition`: Initializes the intuition library by opening it if not already open, copies a string from one memory location to another, stores the destination address to a global variable, and calls dialog display routines with multiple parameters.
- `compute_fixed_point_product`: The routine takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.
- `divide_with_sign`: Performs division of two 32-bit integers with sign handling, calling another routine for the actual division calculation before returning the result.
- `normalize_fixed_point_divide`: The routine divides a 32-bit dividend in d0 by a 32-bit divisor in d1 using a normalized fixed-point method.
- `allocate_and_set_node`: Allocates a memory block through the system routine and, when it succeeds, stores a type value of five, clears one byte and records the argument in the node.
- `allocate_and_initialize_structure`: The routine allocates memory for a structure, populates its fields with provided arguments and system values, and inserts it into a linked list.
- `process_node_flag`: Checks a flag bit in a data structure and conditionally clears a value at a memory location, then returns a status code based on the operations performed.
- `validate_and_lookup`: Validates an index against bounds and checks if a memory location is allocated, then returns a pointer to that location or sets a global error code.
- `check_signal_status`: Sets a signal mask using an operating system call, checks if specific bits are set in the returned value, and conditionally executes another routine based on whether a function pointer is valid and returns a result.
- `frag_bsr_rts`: Short code fragment that calls another routine, then returns.
- `request_dialog`: Calls the intuition library's AutoRequest function with multiple parameters to display a dialog box and returns its result.
- `iterate_active_nodes`: Iterates through a collection of nodes, processes each node by calling another routine with a parameter, and decrements a counter until all nodes are handled.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `free_memory_blocks`: Frees all memory blocks in a linked list by calling the exec library's FreeMem function for each block, then clears the head and tail pointers.
- `allocate_memory_block`: Allocates a new memory block using the exec library's AllocMem function, initializes its header fields, and inserts it into a linked list structure.
- `initialize_game_state_2`: Initializes various game state variables and data structures, setting multiple global values to -1 or zero, and performs a series of operations on a data structure pointed to by a global variable.
- `draw_text_elements`: Prepares and executes multiple graphics drawing operations by calling various jump stubs that interface with Amiga's graphics library functions.
- `draw_text_elements_v2`: Prepares and executes multiple graphics drawing operations by calling various jump stubs that interface with Amiga's graphics library functions, similar to the previous routine but with different parameters.
- `render_text_elements`: Processes text rendering commands by calling a routine with various parameters derived from global data, handling different text display modes based on a condition, and managing stack operations for each command sequence.
- `render_text_elements_2`: Processes text rendering commands by calling a routine with varying parameters, including string pointers and numeric values, then adjusts the stack pointer before returning.
- `render_text_elements_3`: Processes text rendering commands by calling a routine with various parameters derived from a global structure, then adjusts the stack pointer before returning.
- `render_text_elements_4`: Processes text rendering commands by calling a routine with various parameters including coordinates, character data, and control values, then adjusts the stack pointer before returning.
- `render_game_elements`: Processes game element rendering based on a counter value, calling related routines with specific parameters for each element type, and returns a final result from one of the calls.
- `process_input_events`: Processes input events from a queue, updates game state based on event types and values, and handles specific key press conditions to modify a global variable.
- `frag_ori_bra`: Short code fragment that merges 0 into the memory at a0, then merges 23 into the memory just below a0 (moving a0 down), then jumps to another point in the code.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `frag_ori_bra_moveq`: Short code fragment that merges 0 into the memory at a1 (advancing a1), then merges 2 into d2, then jumps to another point in the code, then loads 3 into d4, then jumps to another point in the code.
- `frag_moveq_bra`: Short code fragment that loads 4 into d4, then jumps to another point in the code.
- `select_action`: Evaluates input conditions and selects an action based on comparison results, then jumps to a handler table based on the selected value.
- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `frag_bra_3`: Short code fragment that jumps to another point in the code.
- `frag_bra_4`: Short code fragment that jumps to another point in the code.
- `frag_bra_5`: Short code fragment that jumps to another point in the code.
- `process_input_events_3`: Processes input events by checking event types and calling corresponding handlers, with specific logic for different event values including key press conditions that modify game state.
- `frag_moveq_bsr_bra_movem_unlk_rts`: Short code fragment that loads 0 into d4, then calls another routine (3 times in a row), then jumps to another point in the code, then restores saved registers from the stack, then removes the stack frame held in a5, then returns.
- `validate_object_type`: Checks the type of an object and performs different actions based on its value, including validating ranges and calling specific handlers for types 512, 8, and 1024.
- `frag_ori_bra_2`: Short code fragment that merges 0 into an indexed entry from a7, then merges 24576 into d5, then merges 54 into the memory at a6, then jumps to another point in the code.
- `frag_moveq_bra_2`: Short code fragment that loads 0 into d5, then jumps to another point in the code.
- `validate_input_range`: Checks if a value from memory falls within specific ranges and sets a global variable to zero when certain conditions are met.
- `frag_jsr_bra`: Short code fragment that calls another routine (2 times in a row), then jumps to another point in the code.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, removes the stack frame with unlk, then returns to the caller.
- `process_input_event_2`: Processes input events by checking event types and calling corresponding handlers, with specific logic for different event values including bounds checking and conditional jumps.
- `frag_bra_6`: Short code fragment that jumps to another point in the code.
- `frag_ori_bra_3`: Short code fragment that merges 0 into the memory just below a0 (moving a0 down), then merges 39 into the memory at a2 (advancing a2), then jumps to another point in the code.
- `frag_bsr_bra`: Short code fragment that calls another routine, then jumps to another point in the code.
- `process_key_input`: Processes keyboard input values by comparing them against multiple thresholds and calling initialization routines based on the results.
- `initialize_game_objects`: Initializes game object data by iterating through a loop of ten iterations, setting up parameters for each object and calling initialization routines with specific offsets and values from tables.
- `initialize_render_loop`: Initializes rendering parameters and executes a loop of ten iterations to set up drawing operations, then performs finalization steps including clearing a global variable and calling a routine with specific parameters.
- `initialize_game_objects_2`: Initializes game object data by setting up pointers and calling initialization routines for multiple object types, then performs a string comparison between two memory regions before returning.
- `process_text_rendering`: Processes text rendering commands by calling a routine with various parameters including coordinates, character data, and control values, then performs conditional jumps based on comparison results.
- `frag_ori_bra_4`: Short code fragment that merges 0 into the memory at a2, then merges 55 into the memory at a6 (advancing a6), then jumps to another point in the code.
- `frag_ori_bra_5`: Short code fragment that merges 24576 into d5, then merges 54 into the memory at a2, then jumps to another point in the code.
- `frag_ori_bra_6`: Short code fragment that merges 0 into the memory at a5, then merges 1 into d2, then jumps to another point in the code.
- `set_flag_based_on_range`: Sets a global flag to either zero or one based on whether a value from memory falls within specific numeric ranges.
- `process_game_element`: Processes a game element based on its type and position, performing different actions depending on the element's properties and calling related routines with specific parameters.
- `update_input_state`: Updates input state variables by comparing values against multiple thresholds and setting flags based on comparison results.
- `render_text_elements_5`: Processes text rendering commands by calling a routine with various parameters derived from a global structure, then performs string comparison operations on data pointed to by stack values.
- `initialize_and_process`: Initializes several global variables to zero, calls a graphics routine, and processes text rendering commands until a specific condition is met, then returns a value from another routine.
- `frag_moveq_cmp_bne_clr_bra`: Short code fragment that loads 104 into d0, then compares d0 with the memory at an offset from a5, then branches if the last result was not zero, then clears the memory at an offset from a5, then jumps to another point in the code.
- `frag_jsr_bra_2`: Short code fragment that calls another routine (2 times in a row), then jumps to another point in the code.
- `prepare_graphics_call`: Prepares parameters for a graphics drawing operation by pushing several values onto the stack and calling a graphics routine.
- `frag_jmp_2`: Short code fragment that jumps to another point in the code (2 times in a row).
- `jump_to__draw`: Jump stub: passes control straight on to _Draw.
- `jump_to__move`: Jump stub: passes control straight on to _Move.
- `frag_jmp_3`: Short code fragment that jumps to another point in the code.
- `jump_to__setapen`: Jump stub: passes control straight on to _SetAPen.
- `jump_to__openwindow`: Jump stub: passes control straight on to _OpenWindow.
- `frag_jmp_4`: Short code fragment that jumps to another point in the code.
- `frag_jmp_5`: Short code fragment that jumps to another point in the code.
- `frag_jmp_6`: Short code fragment that jumps to another point in the code.
- `jump_to__getmsg`: Jump stub: passes control straight on to _GetMsg.
- `frag_jmp_7`: Short code fragment that jumps to another point in the code.
- `frag_jmp_8`: Short code fragment that jumps to another point in the code.
- `frag_jmp_9`: Short code fragment that jumps to another point in the code.
- `frag_jmp_10`: Short code fragment that jumps to another point in the code.
- `jump_to__openlibrary`: Jump stub: passes control straight on to _OpenLibrary.
- `jump_to__drawimage`: Jump stub: passes control straight on to _DrawImage.
- `frag_jmp_11`: Short code fragment that jumps to another point in the code.
- `frag_jmp_12`: Short code fragment that jumps to another point in the code.
- `jump_to__setpointer`: Jump stub: passes control straight on to _SetPointer.
- `frag_jmp_13`: Short code fragment that jumps to another point in the code.
- `frag_jmp_14`: Short code fragment that jumps to another point in the code.
- `frag_jmp_15`: Short code fragment that jumps to another point in the code.
- `frag_jmp_16`: Short code fragment that jumps to another point in the code.
- `frag_jmp_17`: Short code fragment that jumps to another point in the code.
- `frag_jmp_18`: Short code fragment that jumps to another point in the code.
- `frag_jmp_19`: Short code fragment that jumps to another point in the code.
- `jump_to__wait`: Jump stub: passes control straight on to _Wait.
- `jump_to__setdrmd`: Jump stub: passes control straight on to _SetDrMd.
- `jump_to__setrgb4`: Jump stub: passes control straight on to _SetRGB4.
- `jump_to__text`: Jump stub: passes control straight on to _Text.
- `jump_to__setbpen`: Jump stub: passes control straight on to _SetBPen.
- `frag_jmp_20`: Short code fragment that jumps to another point in the code.
- `initialize_memory_buffers`: Initializes memory buffers by calculating offsets, calling allocation routines, and setting up data structures for graphics operations.
- `validate_file_checksum`: Validates a file checksum by calculating a sum of specific data segments and comparing it to a fixed value, then performs a string comparison on a global structure if the checksum matches.
- `initialize_game_state_3`: Sets initial values for multiple game state variables including counters, flags, and status indicators.
- `frag_ori_bra_7`: Short code fragment that merges 0 into an indexed entry from a3, then merges 0 into an indexed entry from a5, then merges 0 into the memory at a1, then merges 69 into the memory at a2, then jumps to another point in the code.
- `frag_ori_bra_8`: Short code fragment that merges 0 into an indexed entry from a7, then merges an address into the top of the stack, then merges 0 into the top of the stack, then merges 152 into the memory just below a0 (moving a0 down), then jumps to another point in the code.
- `frag_ori_move_bra`: Short code fragment that merges 0 into the memory at a0 (advancing a0), then merges 1 into d2, then copies d0 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_clr_bra`: Short code fragment that clears the memory at an offset from a5, then jumps to another point in the code.
- `frag_moveq_move_bra`: Short code fragment that loads 255 into d0, then copies d0 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_clr_bra_2`: Short code fragment that clears the memory at an offset from a5, then jumps to another point in the code.
- `frag_clr_bra_3`: Short code fragment that clears the memory at an offset from a5, then jumps to another point in the code.
- `frag_moveq_move_ext_jsr_addq_bra`: Short code fragment that loads 1 into d0, then copies d0 to the memory at an offset from a5, then sign-extends d0, then copies d0 to the stack, then calls another routine, then adds 4 to a7, then jumps to another point in the code.
- `frag_moveq_move_ext_jsr_addq_bra_2`: Short code fragment that loads 0 into d0, then copies d0 to the memory at an offset from a5, then sign-extends d0, then copies d0 to the stack, then calls another routine, then adds 4 to a7, then jumps to another point in the code.
- `process_input_events_2`: Processes input events from a global data structure, updating internal state variables based on event codes and ranges.
- `process_pattern_data`: Iterates through a pattern table, calling a routine with varying parameters derived from the table entries, and updates a counter in a global structure.
- `process_input_data`: Reads input data from a table using a byte index from the stack, processes it through multiple subroutine calls, and stores results back to the stack before returning.
- `insert_projectile`: Inserts a new projectile into a linked list structure based on position checks, updating pointers and initializing projectile data.
- `update_linked_list_2`: Updates a linked list structure by modifying pointers and clearing flags when certain conditions are met, then calls another routine with a pointer to the updated node.
- `update_linked_list`: Updates a linked list structure by modifying pointers and clearing flags when certain conditions are met, then calls another routine with a fixed parameter.
- `initialize_buffers`: Initializes memory buffers by setting specific byte values at calculated offsets, using a loop that iterates 6400 times to configure the data structures.
- `open_and_read_file`: Opens a file for reading and performs read operations on it, checking for specific conditions and calling another routine with file handle data.
- `open_and_write_file`: Opens a file for writing and performs write operations on it, checking for specific conditions and calling another routine with file handle data.
- `frag_jmp_21`: Short code fragment that jumps to another point in the code (2 times in a row).
- `jump_to__open`: Jump stub: passes control straight on to _Open.
- `jump_to__draw_2`: Jump stub: passes control straight on to _Draw.
- `jump_to__move_2`: Jump stub: passes control straight on to _Move.
- `frag_jmp_22`: Short code fragment that jumps to another point in the code.
- `frag_jmp_23`: Short code fragment that jumps to another point in the code.
- `jump_to__setapen_2`: Jump stub: passes control straight on to _SetAPen.
- `jump_to__freemem`: Jump stub: passes control straight on to _FreeMem.
- `jump_to__getmsg_2`: Jump stub: passes control straight on to _GetMsg.
- `frag_jmp_24`: Short code fragment that jumps to another point in the code.
- `jump_to__close`: Jump stub: passes control straight on to _Close.
- `frag_jmp_25`: Short code fragment that jumps to another point in the code.
- `frag_jmp_26`: Short code fragment that jumps to another point in the code.
- `jump_to__write`: Jump stub: passes control straight on to _Write.
- `jump_to__displaybeep`: Jump stub: passes control straight on to _DisplayBeep.
- `jump_to__setpointer_2`: Jump stub: passes control straight on to _SetPointer.
- `frag_jmp_27`: Short code fragment that jumps to another point in the code.
- `jump_to__setbpen_2`: Jump stub: passes control straight on to _SetBPen.
- `jump_to__read`: Jump stub: passes control straight on to _Read.
- `jump_to__allocmem`: Jump stub: passes control straight on to _AllocMem.
- `process_rendering_loop_2`: Processes a rendering loop that continues until a specific bit is set, calling various routines to handle rendering operations and manage sprite data with conditional execution paths based on return values.
- `handle_rendering_operations`: Handles rendering operations by processing sprite data, calling graphics routines with different parameters, managing memory operations, and returning computed values based on conditional logic and loop iterations.
- `process_rendering_loop`: Processes a rendering loop that continues until a specific bit is set, calling various routines to handle rendering operations and manage state.
- `frag_jmp_28`: Short code fragment that jumps to another point in the code.
- `frag_jmp_29`: Short code fragment that jumps to another point in the code.
- `frag_jmp_30`: Short code fragment that jumps to another point in the code.
- `frag_jmp_31`: Short code fragment that jumps to another point in the code.
- `frag_jmp_32`: Short code fragment that jumps to another point in the code.
- `process_graphics_queue`: Processes a graphics queue by iterating through linked list nodes, calling graphics routines with parameters, and clearing queue pointers when the list is empty.
- `process_game_events`: Updates multiple counters in a data structure and calls graphics routines with parameters based on those counter values, then processes various queue and window management operations.
- `update_game_counters`: Updates multiple game counters in a data structure and calls graphics routines with parameters based on those counter values, then processes vari.
- `initialize_graphics_params`: Initializes various graphics parameters for game objects by setting specific memory locations to predefined values and calling graphics routines with different parameters.
- `setup_rendering_values`: Sets up rendering values for game elements by copying data from registers to memory locations and calling graphics routines with specific parameters.
- `update_graphics_state`: Updates graphics state by setting flags on linked list nodes and traversing the list to ensure all nodes have their flags properly set before returning.
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: removes the stack frame with unlk, then returns to the caller.
- `draw_sprite_pattern`: Draws sprite patterns by copying data from a global memory location, calling graphics routines with calculated parameters, and iterating through a loop to draw multiple pattern elements.
- `frag_pea_movea_move_jsr_movem_unlk_rts`: Short code fragment that pushes the address of a fixed memory address onto the stack, then copies the memory at an offset from a4 to a0, then copies the memory at an offset from a0 to the stack, then calls another routine, then copies several registers to or from memory, then removes the stack frame held in a5, then returns.
- `copy_byte_block`: Copies 18 bytes from a fixed memory location to a local buffer, then performs several calculations involving array indices and calls multiple graphics routines with computed parameters.
- `frag_pea_movea_move_jsr_movem_unlk_rts_2`: Short code fragment that pushes the address of a fixed memory address onto the stack, then copies the memory at an offset from a4 to a0, then copies the memory at an offset from a0 to the stack, then calls another routine, then copies several registers to or from memory, then removes the stack frame held in a5, then returns.
- `setup_sprite_data`: Configures sprite-related data by calling a graphics routine multiple times with different parameters, setting specific word values in memory locations, and managing sprite operation setup.
- `configure_sprite_operations`: Sets up sprite operations by configuring various parameters, calling routines to initialize different sprite components, and managing a loop that processes sprite data with conditional logic based on iteration count.
- `setup_sprite_operations`: Sets up sprite operations by configuring various parameters, calling routines to initialize different sprite components, and managing a loop that iterates four times.
- `frag_bsr_move_rts`: Short code fragment that calls another routine, then copies the top of the stack to d7, then returns.
- `format_and_display_text`: Formats numerical data into a text buffer, displays text using graphics routines, and manages character positioning and display operations for text rendering.
- `process_input_event`: Checks if a counter exceeds a limit and sets it to the limit if so, then calls several graphics routines with different parameters.
- `jump_to__getsprite`: Jump stub: passes control straight on to _GetSprite.
- `jump_to__draw_3`: Jump stub: passes control straight on to _Draw.
- `jump_to__move_3`: Jump stub: passes control straight on to _Move.
- `jump_to__setapen_3`: Jump stub: passes control straight on to _SetAPen.
- `jump_to__waitport`: Jump stub: passes control straight on to _WaitPort.
- `frag_jmp_33`: Short code fragment that jumps to another point in the code.
- `frag_jmp_34`: Short code fragment that jumps to another point in the code.
- `jump_to__freemem_2`: Jump stub: passes control straight on to _FreeMem.
- `jump_to__getmsg_3`: Jump stub: passes control straight on to _GetMsg.
- `jump_to__movesprite`: Jump stub: passes control straight on to _MoveSprite.
- `jump_to__delay`: Jump stub: passes control straight on to _Delay.
- `jump_to__changesprite`: Jump stub: passes control straight on to _ChangeSprite.
- `frag_jmp_35`: Short code fragment that jumps to another point in the code.
- `frag_jmp_36`: Short code fragment that jumps to another point in the code.
- `frag_jmp_37`: Short code fragment that jumps to another point in the code.
- `frag_jmp_38`: Short code fragment that jumps to another point in the code.
- `frag_jmp_39`: Short code fragment that jumps to another point in the code.
- `jump_to__freesprite`: Jump stub: passes control straight on to _FreeSprite.
- `jump_to__closewindow`: Jump stub: passes control straight on to _CloseWindow.
- `jump_to__text_2`: Jump stub: passes control straight on to _Text.
- `jump_to__setbpen_3`: Jump stub: passes control straight on to _SetBPen.
- `frag_jmp_40`: Short code fragment that jumps to another point in the code.
- `jump_to__closelibrary`: Jump stub: passes control straight on to _CloseLibrary.
- `check_stack_limit`: Compares the stack pointer with a global limit and branches if the stack is too deep.
- `setup_io_request`: Configures an I/O request structure by setting various fields including command codes, buffer pointers, and data values, then sends the request to a device.
- `configure_io_request`: Sets up an I/O request structure with specific command codes, buffer configurations, and parameter values before sending it to a device for processing.
- `initialize_io_operations`: Sets up I/O request structures by configuring command codes, buffer pointers, and data values.
- `process_io_request`: Processes an I/O request by reading data from memory locations, calling another routine with specific parameters, and returning a boolean result based on comparison operations.
- `process_io_operations`: Processes I/O operations by checking conditions on data structures, updating values, and calling another routine when certain criteria are met.
- `frag_moveq_bra_3`: Short code fragment that loads 0 into d0, then jumps to another point in the code.
- `function_epilogue_3`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, removes the stack frame with unlk, then returns to the caller.
- `handle_io_message`: Handles I/O messages by checking conditions on data structures, calling another routine with specific parameters, and looping until a condition is satisfied or a timeout occurs.
- `handle_io_messages`: Handles I/O messages by processing received data, calling another routine with specific parameters, and looping until a condition is satisfied.
- `function_epilogue_4`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, removes the stack frame with unlk, then returns to the caller.
- `frag_jmp_41`: Short code fragment that jumps to another point in the code.
- `frag_jmp_42`: Short code fragment that jumps to another point in the code.
- `frag_jmp_43`: Short code fragment that jumps to another point in the code (2 times in a row).
- `jump_to__doio`: Jump stub: passes control straight on to _DoIO.
- `jump_to__waitport_2`: Jump stub: passes control straight on to _WaitPort.
- `jump_to__getmsg_4`: Jump stub: passes control straight on to _GetMsg.
- `jump_to__closedevice`: Jump stub: passes control straight on to _CloseDevice.
- `frag_jmp_44`: Short code fragment that jumps to another point in the code.
- `jump_to__opendevice`: Jump stub: passes control straight on to _OpenDevice.
- `jump_to__sendio`: Jump stub: passes control straight on to _SendIO.
- `frag_jmp_45`: Short code fragment that jumps to another point in the code.
- `update_sprite_data`: Iterates through sprite-related data tables, calls processing routines with specific values from the table, and updates various memory locations based on conditional checks and calculations.
- `manage_sprite_animation`: Manages sprite animation by checking if sprites are active, calling processing routines with parameters, and updating animation state flags in memory locations.
- `frag_move_add_lea_clr_movem_unlk_rts`: Short code fragment that copies the memory at an offset from a5 to d0, then adds d0 to d0, then loads the address of the memory at an offset from a4 into a0, then clears an indexed entry from a0, then restores saved registers from the stack, then removes the stack frame held in a5, then returns.
- `clear_sprite_flags`: Clears sprite flags by checking if a counter exceeds a limit, calling processing routines, and setting specific memory locations to zero based on conditional checks.
- `process_sprite_data`: Processes sprite-related data by iterating through a table, calling another routine with specific values from the table when certain conditions are met.
- `frag_jmp_46`: Short code fragment that jumps to another point in the code.
- `jump_to__freemem_3`: Jump stub: passes control straight on to _FreeMem.
- `frag_jmp_47`: Short code fragment that jumps to another point in the code.
- `frag_jmp_48`: Short code fragment that jumps to another point in the code.
- `frag_jmp_49`: Short code fragment that jumps to another point in the code.
- `frag_jmp_50`: Short code fragment that jumps to another point in the code.
- `jump_to__allocmem_2`: Jump stub: passes control straight on to _AllocMem.
- `handle_resource_operations`: Processes resource operations by calling various routines to initialize and manage resources, with conditional branching based on return values from those calls.
- `frag_jmp_51`: Short code fragment that jumps to another point in the code.
- `frag_jmp_52`: Short code fragment that jumps to another point in the code.
- `frag_jmp_53`: Short code fragment that jumps to another point in the code.
- `frag_jmp_54`: Short code fragment that jumps to another point in the code.
- `frag_jmp_55`: Short code fragment that jumps to another point in the code.
- `frag_jmp_56`: Short code fragment that jumps to another point in the code.
- `manage_graphics_resources`: Manages graphics resources by calling initialization routines and handling conditional logic based on return values from resource operations.
- `frag_jmp_57`: Short code fragment that jumps to another point in the code.
- `frag_jmp_58`: Short code fragment that jumps to another point in the code.
- `frag_jmp_59`: Short code fragment that jumps to another point in the code.
- `frag_jmp_60`: Short code fragment that jumps to another point in the code.
- `frag_jmp_61`: Short code fragment that jumps to another point in the code.
- `initialize_object`: Sets up object properties based on a parameter value, then performs conditional allocations and assignments before returning a status code.
- `frag_bra_7`: Short code fragment that jumps to another point in the code.
- `frag_bra_8`: Short code fragment that jumps to another point in the code.
- `frag_bra_9`: Short code fragment that jumps to another point in the code.
- `frag_bra_10`: Short code fragment that jumps to another point in the code.
- `frag_move_bra`: Short code fragment that copies 1 to the memory at an offset from a4, then jumps to another point in the code.
- `frag_move_bra_2`: Short code fragment that copies 2 to the memory at an offset from a4, then jumps to another point in the code.
- `frag_move_bra_3`: Short code fragment that copies 4 to the memory at an offset from a4, then jumps to another point in the code.
- `frag_move_pea_jsr_addq_bne_clr_bra`: Short code fragment that copies 8 to the memory at an offset from a4, then copies 65537 to the stack, then pushes the address of a fixed memory address onto the stack, then calls another routine, then adds 8 to a7, then copies d0 to the memory at a3, then branches if the last result was not zero, then clears the memory at a2, then jumps to another point in the code.
- `update_object_attributes_3`: Sets a specific byte value at a fixed offset in memory, clears a stack location, calls a routine with parameters from registers, stores the result to memory, and conditionally executes different code paths based on that result.
- `update_object_attributes_2`: Updates object attributes by setting specific bytes and words in memory locations pointed to by a3 register, with conditional logic based on counter values and calling memory management routines.
- `update_object_attributes`: Updates object attributes by setting specific bytes and words in memory locations pointed to by a3 register, with conditional logic based on a value comparison.
- `frag_movea_move_movem_unlk_rts`: Short code fragment that copies the memory at a3 to a0, then copies 1 to the memory at an offset from a0, then restores saved registers from the stack, then removes the stack frame held in a5, then returns.
- `initialize_and_check`: Initializes several global variables with specific values including a negative one, performs a subroutine call, and then checks a word value.
- `setup_and_check_condition`: Initializes several global variables with specific values including a negative one, performs a subroutine call, and then checks a word value at a specific offset in memory before branching based on the result.
- `load_and_sync_counters`: Loads a longword from a global variable at offset -4 relative to a5, synchronizes two counters using another routine, and then checks a condition based on a value at offset -12 relative to a5.
- `process_io_operations_2`: Processes input/output operations by checking memory locations, calling system routines, and updating memory with new values based on conditional logic.
- `synchronize_counters_3`: Checks if a counter value is zero and sets another memory location to one if so, otherwise copies data from registers to memory locations and calls routines with parameters, decrementing the counter in a loop until it reaches zero.
- `frag_moveq_move_bra_2`: Short code fragment that loads 1 into d0, then copies d0 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bsr_movem_unlk_rts`: Short code fragment that copies the memory at an offset from a5 to the stack, then calls another routine, then copies several registers to or from memory, then removes the stack frame held in a5, then returns.
- `validate_and_process`: Validates and processes data structures by checking specific offsets, calling routines based on conditions, and performing memory operations before returning.
- `synchronize_counters_2`: Synchronizes two counters by comparing a value against a threshold, updating memory locations with counter values, and adjusting them based on comparison results before returning.
- `synchronize_counters`: Synchronizes two counters by setting a global value, updating memory locations with counter values, and adjusting them based on comparison results.
- `frag_clr_movem_unlk_rts`: Short code fragment that clears the memory at a3, then restores saved registers from the stack, then removes the stack frame held in a5, then returns.
- `adjust_memory_allocation`: Adjusts memory allocation by comparing a value against a threshold, updating memory locations with new values, and modifying counters accordingly.
- `frag_movea_move_clr_movem_unlk_rts`: Short code fragment that copies the memory at an offset from a5 to a0, then copies d0 to the memory at an offset from a0, then copies the memory at a2 to the memory at an offset from a0, then clears the memory at a3, then restores saved registers from the stack, then removes the stack frame held in a5, then returns.
- `frag_jmp_62`: Short code fragment that jumps to another point in the code (2 times in a row).
- `jump_to__freemem_4`: Jump stub: passes control straight on to _FreeMem.
- `jump_to__closedevice_2`: Jump stub: passes control straight on to _CloseDevice.
- `frag_jmp_63`: Short code fragment that jumps to another point in the code.
- `jump_to__opendevice_2`: Jump stub: passes control straight on to _OpenDevice.
- `jump_to__checkio`: Jump stub: passes control straight on to _CheckIO.
- `jump_to__beginio`: Jump stub: passes control straight on to _BeginIO.
- `jump_to__abortio`: Jump stub: passes control straight on to _AbortIO.
- `jump_to__allocmem_3`: Jump stub: passes control straight on to _AllocMem.
- `initialize_game_state_4`: Sets up initial game state variables including counters, flags, and position values in a local data structure.
- `process_object_collision`: Processes object collision states based on type values, updates global variables, and handles specific conditional jumps for different object types.
- `check_input_conditions`: Tests multiple conditions based on values from a structure referenced via a4, and updates a global variable based on these checks.
- `frag_jsr_bra_3`: Short code fragment that calls another routine (2 times in a row), then jumps to another point in the code.
- `reset_explosion_vars`: Resets explosion-related counters and state variables, initializes object properties, and handles collision detection updates.
- `update_explosion_state_3`: Updates explosion-related counters and state variables based on object properties, then handles collision detection and updates explosion state.
- `frag_cmpi_bcc_asl_jmp`: Short code fragment that compares d0 with 6, then branches if the carry flag is clear, then shifts d0 left by 1, then jumps to another point in the code.
- `frag_bra_11`: Short code fragment that jumps to another point in the code.
- `frag_bra_12`: Short code fragment that jumps to another point in the code.
- `frag_bra_13`: Short code fragment that jumps to another point in the code.
- `frag_bra_14`: Short code fragment that jumps to another point in the code.
- `frag_bra_15`: Short code fragment that jumps to another point in the code.
- `frag_bra_16`: Short code fragment that jumps to another point in the code.
- `frag_addq_bra`: Short code fragment that adds 1 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_addq_bra_2`: Short code fragment that adds 2 to the memory at an offset from a5, then adds 1 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_addq_bra_3`: Short code fragment that adds 2 to the memory at an offset from a5, then jumps to another point in the code.
- `update_explosion_2`: Updates explosion-related counters and state variables based on object properties, then handles collision detection and updates explosion state.
- `update_explosion_counter_2`: Increments a counter stored at offset -94 relative to a5 and updates related state variables.
- `update_explosion_state_2`: Updates explosion-related counters and state variables based on object properties, then handles collision detection and updates explosion state.
- `frag_cmpi_bcc_asl_jmp_2`: Short code fragment that compares d0 with 6, then branches if the carry flag is clear, then shifts d0 left by 1, then jumps to another point in the code.
- `frag_bra_17`: Short code fragment that jumps to another point in the code.
- `frag_bra_18`: Short code fragment that jumps to another point in the code.
- `frag_bra_19`: Short code fragment that jumps to another point in the code.
- `frag_bra_20`: Short code fragment that jumps to another point in the code.
- `frag_bra_21`: Short code fragment that jumps to another point in the code.
- `frag_bra_22`: Short code fragment that jumps to another point in the code.
- `frag_addq_bra_4`: Short code fragment that adds 1 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_addq_bra_5`: Short code fragment that adds 2 to the memory at an offset from a5, then adds 1 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_addq_bra_6`: Short code fragment that adds 2 to the memory at an offset from a5, then jumps to another point in the code.
- `update_explosion_data`: Updates explosion-related position counters and state variables based on object properties, then updates explosion state and handles collision detection with other game objects.
- `update_explosion_counter`: Increments a counter at offset -90 in the frame structure and decrements a counter at offset -74.
- `update_explosion`: Processes explosion-related data including position counters and state variables based on object properties, then updates explosion state and handles collision detection with player objects.
- `update_explosion_pointer`: Loads a global variable, checks if it's zero, and if not, compares a value from a table with another global value.
- `update_explosion_state`: Updates explosion-related data including position counters and state variables based on object properties, then enters a loop to update explosion effects.
- `initialize_explosion`: Initializes explosion-related data including position counters and state variables based on object properties, then enters a loop to update explosion animation states and coordinates.
- `update_game_timer`: Decrements two counters at specific offsets from the frame pointer, processes game state based on their values and a global counter, then increments a timer and resets it after reaching a threshold.
- `load_sprite_data`: Loads sprite-related data from a global variable into a local pointer, then calls a routine to process that data before jumping to another location.
- `update_game_logic_2`: Decrements two counters at offsets -102 and -110 from the frame pointer, then processes game state based on their values and a global counter.
- `update_game_state_5`: Decrements two counters at offsets -98 and -106 from the stack frame, then processes game logic based on their values and a global counter at offset -114.
- `update_game_counter`: Decrements a counter at offset -98 in the stack frame, then increments a global counter at offset -114.
- `calculate_and_invoke_function`: Loads a longword constant into a register, adds a value from a global variable to it, uses the result as an offset to calculate a memory address, and then calls two routines with calculated parameters before branching to another location.
- `update_game_state_4`: Updates game logic variables including counters and flags based on calculated values from sprite positions and other game data.
- `update_game_logic`: Processes game state variables including counters and flags based on calculated values from sprite positions and other game data, then performs conditional updates to game objects and their states.
- `update_game_state_3`: Processes game state variables including counters and flags based on calculated values from sprite positions and other game data, then performs updates.
- `update_game_state_2`: Updates game state variables including counters and flags based on calculated values from sprite positions and other game data, then performs conditional resets or increments of internal game counters.
- `initialize_game_state`: Initializes multiple game variables and resets counters when a condition is met.
- `update_game_state`: Increments a counter in a global variable and performs conditional updates based on the counter's value, including resetting flags and calling other routines.
- `frag_jsr_movem_unlk_rts`: Short code fragment that calls another routine, then restores saved registers from the stack, then removes the stack frame held in a5, then returns.
- `jump_to__replymsg`: Jump stub: passes control straight on to _ReplyMsg.
- `frag_jmp_64`: Short code fragment that jumps to another point in the code.
- `frag_jmp_65`: Short code fragment that jumps to another point in the code.
- `jump_to__setapen_4`: Jump stub: passes control straight on to _SetAPen.
- `frag_jmp_66`: Short code fragment that jumps to another point in the code.
- `jump_to__getmsg_5`: Jump stub: passes control straight on to _GetMsg.
- `frag_jmp_67`: Short code fragment that jumps to another point in the code.
- `jump_to__movesprite_2`: Jump stub: passes control straight on to _MoveSprite.
- `frag_jmp_68`: Short code fragment that jumps to another point in the code.
- `frag_jmp_69`: Short code fragment that jumps to another point in the code.
- `jump_to__delay_2`: Jump stub: passes control straight on to _Delay.
- `jump_to__displaybeep_2`: Jump stub: passes control straight on to _DisplayBeep.
- `jump_to__setpointer_3`: Jump stub: passes control straight on to _SetPointer.
- `jump_to__changesprite_2`: Jump stub: passes control straight on to _ChangeSprite.
- `frag_jmp_70`: Short code fragment that jumps to another point in the code.
- `frag_jmp_71`: Short code fragment that jumps to another point in the code.
- `frag_jmp_72`: Short code fragment that jumps to another point in the code.
- `frag_jmp_73`: Short code fragment that jumps to another point in the code.
- `frag_jmp_74`: Short code fragment that jumps to another point in the code.
- `frag_jmp_75`: Short code fragment that jumps to another point in the code.
- `frag_jmp_76`: Short code fragment that jumps to another point in the code.
- `frag_jmp_77`: Short code fragment that jumps to another point in the code.
- `frag_jmp_78`: Short code fragment that jumps to another point in the code.
- `frag_jmp_79`: Short code fragment that jumps to another point in the code.
- `jump_to__setbpen_4`: Jump stub: passes control straight on to _SetBPen.
- `frag_jmp_80`: Short code fragment that jumps to another point in the code.
- `jump_to__readpixel`: Jump stub: passes control straight on to _ReadPixel.
- `_AllocMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `_FreeMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `_Wait`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `_GetMsg`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `_ReplyMsg`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `_WaitPort`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `_CloseLibrary`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `_OpenDevice`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `_CloseDevice`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `_DoIO`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `_SendIO`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `_CheckIO`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `_AbortIO`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `_OpenLibrary`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `_Text`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `_Move`: Library stub: loads its arguments and the exec library base, calls AddHead and returns its result to the caller.
- `_Draw`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `_SetRGB4`: Library stub: loads its arguments and the exec library base, calls RemTask and returns its result to the caller.
- `_ReadPixel`: Library stub: loads its arguments and the exec library base, calls Wait and returns its result to the caller.
- `_SetAPen`: Library stub: loads its arguments and the exec library base, calls AllocTrap and returns its result to the caller.
- `_SetBPen`: Library stub: loads its arguments and the exec library base, calls FreeTrap and returns its result to the caller.
- `_SetDrMd`: Library stub: loads its arguments and the exec library base, calls AddPort and returns its result to the caller.
- `_GetSprite`: Library stub: loads its arguments and the exec library base, calls OldOpenLibrary and returns its result to the caller.
- `_FreeSprite`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `_ChangeSprite`: Library stub: loads its arguments and the exec library base, calls SetFunction and returns its result to the caller.
- `_MoveSprite`: Library stub: loads its arguments and the exec library base, calls SumLibrary and returns its result to the caller.
- `_CloseWindow`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `_DisplayBeep`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `_DrawImage`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.
- `_OpenWindow`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `_SetPointer`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `_Open`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `_Close`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `_Read`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `_Write`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `_Delay`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `_BeginIO`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `frag_link_cmpa_bcs_moveq_unlk_rts_jmp`: Short code fragment that creates a stack frame in a5, then compares a7 with the memory at an offset from a4, then branches if the carry flag is set, then loads 0 into d0, then removes the stack frame held in a5, then returns, then jumps to another point in the code.

### Miniblast v1.0 (1990)(A. Bjerin).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
