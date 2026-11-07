# Game 247 (BlackCrypt v2.0 0281): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00247). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2403573 top-level bytes in 43 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 22 raw-word lines |
| Functions named and described | pass, 673 of 673 named (0 author symbols), 673 of 673 described (14 by rule, 654 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2019 of 2019 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 144169 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 11893 of 11954 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2036270 |
| code | 38438 |
| data | 328853 |
| padding | 12 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| BlackCrypt.Slave | 1 | 1175 | 77 |
| BlackCrypt | 3 | 1745 | 48 |
| bcdfp | 3 | 6150 | 184 |
| bcdfq | 2 | 1065 | 44 |
| bcdft | 7 | 29 | 2 |
| bcdfu | 2 | 1490 | 84 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 285 | 225 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 6 |
| bcdfx (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- 6 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 203 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- BlackCrypt_v2.0_0281/BlackCrypt/data/bcdfx: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- BlackCrypt_v2.0_0281/BlackCrypt/data/bcdft: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 144169 bytes, recorded as UNVERIFIED DATA (0 bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### BlackCrypt.Slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `dispatch_indirect_calls`: The routine stores a pointer and a register value into global variables, then executes two indirect function calls via offsets from a pointer, followed by another indirect call through a global variable.
- `update_jump_table_entries`: The routine calculates an address by adding a constant to a global variable, stores this address into two entries of a jump table, calls another routine, and finally performs an indirect jump through the jump table.
- `jump_to_indirect_table`: Performs an indirect jump to an address retrieved from an unrecoverable jump table.
- `set_register_and_return_offset`: If a specific register is zero, it copies another register's value into it.
- `mask_global_value`: Returns the value of a global variable with specific bits cleared by applying a bitwise AND mask.
- `return_stack_value`: Returns a value located on the stack.
- `store_stack_to_global`: Copies a value from the stack into a global variable.
- `return_zero_constant`: The routine immediately returns zero without performing any operations or modifying data.
- `set_bit_in_struct`: The routine receives a pointer in a register and sets bit 1 in the byte at offset 14 of the pointed-to structure.
- `call_empty_stub`: The routine is empty and performs no operations.
- `store_register_to_global`: Copies the value from register D0 into a global variable.
- `update_global_and_jump`: Updates a global variable with a value from another global location, then performs an indirect jump via an unrecoverable jump table.
- `copy_byte_to_offset`: Copies a single byte from an address derived from one register to an address derived from another register plus a fixed offset.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `check_display_mode_flag`: Loads a constant value and checks a global variable against two specific values.
- `initialize_pointer_chain`: Writes a sentinel value into a pointer and stores the address of the next entry into three fields in two different structures.
- `copy_nested_pointers`: Copies 32-bit values from pointers referenced at offsets four and eight of a source structure into fields of a destination structure.
- `negate_nonzero_flag`: The routine receives a value in a register and returns zero if the value is non-zero, and returns a negative one if the value is zero.
- `check_vhposr_value`: The routine reads the VHPOSR hardware register, performs bitwise operations on the value, and compares it against a global variable.
- `return_immediately_15`: The routine executes a return instruction without performing any data manipulation or calling other functions.
- `conditional_data_swap`: The routine checks a register value; if non-zero, it stores a stack value to a global variable and returns stack data.
- `calculate_table_offset`: Reads values from a global table and performs arithmetic operations including division, multiplication, and addition.
- `process_disk_flag_logic`: The routine references the text 'Disk.' and manipulates bytes at specific offsets in structures based on bit checks.
- `toggle_flag_and_negate`: Negates a 16-bit value at a specific offset in a structure and toggles a high bit in a byte field of another structure.
- `set_flag_from_bit`: Reads a bit from a global variable using an index derived from a byte in another memory location.
- `empty_stub_2`: The routine is empty and performs no operations, immediately returning to the caller.
- `return_immediately`: The routine performs no data manipulation or hardware interaction.
- `update_bitmask_counter`: The routine checks a bit in a global byte variable derived from a structure offset.
- `return_immediately_14`: This routine performs no operations and returns control to the caller immediately.
- `do_nothing_stub`: The routine executes five instructions but performs no observable work, ending without modifying memory, registers, or calling another routine.
- `call_external_stub`: The routine is empty and performs no operations.
- `clear_offset_and_save_register`: The routine stores zero to a word-sized location at an offset from a pointer in a register, then saves the value of another register into a global variable before returning.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `dispatch_slave_code_pointers`: Compares a register value against a global variable and performs an indirect jump through a pointer table.
- `set_library_entry_point`: The routine stores its own address into a global variable and calls a setup routine that allocates memory and initializes structures using exec and dos libraries.
- `allocate_and_setup_buffer`: The routine references 'dos.library' and uses AllocMem to allocate memory.
- `initialize_expansion_structures`: The routine allocates memory, opens the expansion library, and sets flags and pointers in structures obtained via table calls.
- `call_dos_library_entry`: Invokes a function pointer retrieved from a library base structure associated with dos.library.
- `handle_message_port`: The routine references 'dos.library' and uses exec library calls such as FindTask, GetMsg, PutMsg, and WaitPort.
- `verify_boot_files`: The routine calls functions to handle message ports and verify boot file names by comparing strings against 'WHDBoot.exe' and 'WHDBoot.ss'. It also invokes stub routines that return immediately.
- `stub_message_handler`: An unreached code gap that calls a message port handler, a do-nothing stub, and a callback verification routine.
- `verify_boot_file_names`: The routine checks boot file names against 'WHDBoot.exe' and 'WHDBoot.ss' and handles message ports.
- `handle_message_port_call`: The routine calls a message handler function that utilizes dos.library and exec library functions for port operations, along with one unnamed routine.
- `call_message_handler`: Calls another routine that handles message ports using dos.library and exec library functions.
- `verify_boot_stub`: A two-instruction routine that calls a stub function for verifying boot files and handling message ports.
- `verify_boot_files_stub`: The routine calls functions to handle message ports and verify boot file names, similar to another routine, while also invoking stub routines that return immediately.
- `verify_boot_files_via_stub`: The routine invokes a stub to verify boot files and handles message ports using exec library calls.
- `handle_message_callback`: Calls a message port handler and a routine that verifies and executes a callback based on register values and pointer fields.
- `call_port_handler`: An unreached code gap that calls a function to handle message ports using exec library functions.
- `call_message_port_handler`: The routine calls a function to handle message ports using exec library functions and invokes another routine for message port operations.
- `call_message_handler_2`: Calls another routine that handles message port operations.
- `do_nothing_stub_2`: The routine executes three instructions before returning, despite the decompiled output showing no logical operations.
- `return_immediately_2`: The routine executes a return instruction to exit immediately.
- `return_immediately_3`: The routine executes a standard return sequence to exit the function.
- `rotate_string_and_store_length`: The routine rotates a null-terminated byte string one position to the right by shifting each byte forward and writing the original first byte into the last position, then stores the length of the string minus one as a byte just before the string in memory.
- `verify_boot_file_names_2`: Constructs file paths and compares string pointers against 'WHDBoot.exe' and 'WHDBoot.ss' to verify file identities, returning status codes.
- `verify_and_execute_callback`: The routine checks a register value and compares a pointer field in a structure.
- `construct_file_path`: Concatenates a directory path and a filename into a buffer, handling slash separators and path normalization.
- `check_boot_file_names`: Compares two string pointers against the text 'WHDBoot.exe' and 'WHDBoot.ss' to verify file identities.
- `compare_two_strings`: The routine compares the bytes pointed to by two pointers until a zero byte is found, returning early if the bytes differ, otherwise it falls through after the terminating zero.
- `empty_routine`: The routine is empty, containing no valid instructions.
- `return_early_stub`: The routine is a short stub of eight 68000 instructions whose Ghidra pseudo-code collapses to a bare return, so the decompiler's output does not reflect its real behavior.
- `setup_library_entry_point_2`: Allocates memory, copies code blocks, and initializes structures using exec.OldOpenLibrary and dos.library, referencing disk names like GAMEDISK1.
- `setup_library_entry_point`: Copies a block of code into a global memory region and initializes a structure with specific constants and pointer values.
- `tiny_eori`: Small routine of 1 instruction that exclusive-ors 113 into d0.
- `parse_path_and_match_entry`: The routine strips trailing slashes and colons from a byte string, calculates an offset from a linked list, and compares the processed string against entries in a table while converting lowercase letters to uppercase, calling other routines if a match is found.
- `parse_path_and_match_entry_2`: Traverses a linked list to calculate an offset, then iterates through a global table to find a matching string entry by comparing characters while normalizing case.
- `parse_path_and_lookup_entry`: Traverses a linked list to calculate an offset, then scans a global table for a matching entry.
- `parse_and_match_path`: Extracts a substring from a path-like string by skipping forward slashes and colons, then iterates through a global table to find a matching entry.
- `patch_code_pointers_2`: Iterates through a list of pointers, checking for specific magic values and comparing bytes against a global variable.
- `patch_code_pointers`: The routine iterates through a list of pointers, checking for specific magic values.
- `iterate_pointer_list_2`: Traverses a linked list of pointers by dereferencing and shifting values until a zero is encountered, modifying COLOR00 hardware register.
- `iterate_pointer_list`: Traverses a linked list of pointers by dereferencing and shifting values until a zero is encountered.
- `return_stack_value_2`: Returns the value located at the top of the stack frame without performing any other operations.
- `clear_color_register`: Iterates through a memory region to find a specific hardware register value and clears its lower bits, then sets a global variable.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### BlackCrypt

- `initialize_program_entry`: This routine initializes memory by zeroing a block of data and setting up pointers.
- `handle_workbench_disk_access`: Opens and closes the Workbench interface, locks the game disk volume, and executes a loop processing input events and state changes before unlocking the disk.
- `initialize_graphics_and_config`: Opens the graphics and intuition libraries, loads the configuration file, and initializes bitmaps and screen structures for the main game window.
- `execute_shutdown_callbacks_3`: The routine checks multiple global flags and executes corresponding function pointers from library bases.
- `delay_and_unload_segment`: Sets a value in a global variable and calls several indirect function pointers.
- `poll_input_event_loop`: Loops calling an input dispatch routine until a specific condition is met, handling cases for event types 1 through 4 by updating global state variables or invoking other handlers.
- `convert_string_case`: Copies a byte array and converts lowercase characters to uppercase by subtracting a constant.
- `convert_string_case_2`: Copies a byte array and converts lowercase characters to uppercase by subtracting a constant, iterating through a global variable array.
- `handle_disk_insertion_prompt`: Checks a global flag and, if not set, displays a 'PLEASE INSERT DISK' message using string conversion and tile copying routines.
- `wait_for_library_call`: Loops calling a function pointer from a library base until a second function pointer returns a non-zero value.
- `initialize_game_segments`: Loads executable segments and creates a process using DOS library calls, then initializes global pointers and flags by invoking shutdown callback routines and checking disk insertion status.
- `execute_library_shutdown_callbacks`: Invokes function pointers from library bases and global structures to perform shutdown sequences, checking flags and iterating through callback lists.
- `handle_segment_load_unload`: Calls routines to load or unload memory segments, checks the return value, and if successful, invokes a function pointer with calculated parameters before cleaning up.
- `process_string_with_tiles`: Iterates through a null-terminated string, calling a routine to copy tile data for each character.
- `return_immediately_4`: The routine executes a return instruction to exit the current function.
- `calculate_length_and_copy_tiles`: Calculates the length of a null-terminated string and iterates through it, calling a routine to copy tile data for each character.
- `scan_global_variable_array_2`: Iterates through a global variable array in two passes, stepping by two bytes, and increments nibble values in lists of words.
- `decrement_nibble_loop_2`: Executes a loop sixteen times, calling indirect function pointers and decrementing nibble values in a list of words, using graphics.BltClear.
- `process_string_with_tiles_2`: Iterates through null-terminated strings like 'CONFIGURATION MENU' and 'MAIN MENU', copying tile data for each character.
- `check_input_event`: Polls an input event queue, checking message types and data fields to determine a return value based on specific conditions.
- `dispatch_input_event_handler`: Polls an input event queue and dispatches execution to different sub-routines based on the event type code.
- `store_register_values`: Stores the contents of data registers into specific global memory locations.
- `copy_tile_data`: Copies a block of data from a source pointer to multiple destination addresses with fixed offsets, iterating five times.
- `scan_global_variable_array`: Iterates through a global variable array in two passes, stepping by two bytes.
- `decrement_nibble_loop`: Executes a loop sixteen times, calling indirect function pointers and a routine to decrement nibble values in a list of words.
- `decrement_nibble_values`: Iterates through a list of words, decrementing each of the three nibble components if they are non-zero.
- `increment_nibble_loop`: Executes a loop sixteen times, calling indirect function pointers and a routine to increment nibble values in lists of words.
- `increment_nibble_values`: Iterates through two lists of words, incrementing the nibble components of the first list if they differ from the corresponding components in the second list.
- `copy_byte_array`: Copies a specified number of bytes from a source pointer to a destination pointer.
- `compare_string_bytes`: The routine compares two byte strings character by character, returning a status code based on the first difference or equality.
- `return_immediately_5`: The routine is empty and returns immediately without performing any operations.
- `handle_workbench_disk_access_2`: The routine interacts with the Workbench interface, locks the game disk volume, parses strings, opens icon libraries, and processes input events and state changes.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `parse_string_into_tokens`: The routine allocates a buffer and parses a source string into multiple null-terminated tokens, handling quoted strings and whitespace delimiters.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_with_limit`: The routine finds the end of the first string, then copies characters from the second string into the first, decrementing a counter for each character copied.
- `open_icon_library`: Opens the icon.library and retrieves a pointer to an icon structure.
- `append_string_to_buffer`: The routine advances a pointer to the end of the first string, then copies characters from the second string into that location until the null terminator is copied.
- `copy_string_or_pad`: The routine copies characters from a source string to a destination buffer up to a specified count or until the source ends.
- `create_file_entry`: Allocates or retrieves a file entry structure and initializes its fields with provided parameters, including a pointer and a byte value.
- `initialize_library_context`: The routine checks a field in a structure; if non-zero, it calls a function from a library base.
- `calculate_string_length`: The routine receives a pointer to a zero-terminated byte string and advances a pointer until it finds the terminating zero byte.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `copy_string_simple`: The routine copies characters from a source string to a destination buffer until the null terminator is copied.
- `execute_shutdown_callbacks_2`: Iterates through a list of indices to invoke helper routines and executes conditional function pointers stored in global memory.
- `execute_shutdown_callbacks`: Iterates through a list of indices to invoke a helper routine, then executes a series of conditional function pointers stored in global memory.
- `return_immediately_6`: The routine contains no instructions and returns immediately.
- `close_file_entry`: The routine validates an index against a global limit and checks if a file entry pointer is null.

### bcdfp

- `initialize_game_state`: Clears a block of memory and checks a flag in a global structure to determine whether to execute a specific callback or another routine based on the return value.
- `open_graphics_intuition_libraries`: Opens graphics.library and intuition.library, checks for failure using referenced text strings, and calls routines to wait for library calls and execute cleanup callbacks.
- `initialize_data_block`: Calls a routine to initialize a data structure, then retrieves a pointer from a global variable.
- `process_linked_list`: Traverses a linked list of structures using an index.
- `initialize_entity_structures`: Iterates four times to initialize distinct data structures, setting specific byte flags, clearing arrays, and copying configuration values from a global table.
- `initialize_entity_arrays`: Iterates four times to populate multiple global arrays with calculated values and increments specific byte fields.
- `serialize_character_data`: Copies character configuration blocks, including linked list entries and attribute tables, from memory buffers into a file handle.
- `wait_for_library_call_2`: Calls a function pointer from a global structure repeatedly until a second function pointer from the same structure returns a non-zero value.
- `initialize_library_call_state_zero`: Sets a global state variable to zero and invokes a function pointer from a global structure.
- `initialize_library_call_state_two`: Sets a global state variable to two and invokes a function pointer from a global structure.
- `initialize_library_call_state_eight`: Sets a global state variable to eight and invokes a function pointer from a global structure.
- `initialize_library_call_state_three`: Sets a global state variable to three and stores a parameter in a global variable.
- `initialize_library_call_state_seven`: Sets a global state variable to seven and stores a parameter in a global variable.
- `initialize_library_call_state_ten`: Sets a global state variable to ten and stores two parameters in global variables.
- `manage_dungeon_save_files`: Opens, reads, and writes files on disk using DOS library calls, initializing entity structures and serializing character data between memory buffers and disk files.
- `manage_disk_access_and_saves`: Uses DOS library calls to lock and unlock the game disk, checks disk status with referenced text, and calls routines to manage dungeon save files and handle workbench disk access.
- `dispatch_sprite_rendering_2`: The routine calculates sprite offsets, checks range overlaps in a global table, and dispatches to specific rendering routines based on switch values, including text rendering and queue management.
- `setup_sprite_rendering_loop`: Iterates through loops to calculate sprite offsets, configure rendering parameters, and invoke sprite setup and rendering routines for multiple entities.
- `loop_calculate_sprite_offsets`: Iterates nine times, calling a routine that calculates a memory offset by multiplying a register value by a constant and adding a base, storing the result in a global variable.
- `call_sprite_offset_calculator`: Calls a routine that calculates a memory offset by multiplying a register value by a constant and adding a base, storing the result in a global variable.
- `calculate_sprite_offset_value_2`: Computes a memory offset based on a register value and a constant, stores the result in a global structure, and calls helper routines for sprite processing.
- `configure_sprite_rendering_params`: Sets specific word-sized values in registers, calls routines to calculate sprite offsets and setup rendering, and invokes text rendering and sprite configuration helpers.
- `dispatch_sprite_rendering`: Calculates sprite offsets and checks range overlaps in a global table, then dispatches to specific rendering routines based on a switch value derived from register calculations.
- `check_range_overlap`: Iterates through ten entries in a global table, checking if a passed parameter falls within specific ranges defined by the table entries.
- `check_and_update_sprite_condition`: Checks values in a global table indexed by parameters, increments or decrements specific fields if conditions are met, and calls helper routines for further processing.
- `check_sprite_condition`: Reads a value from a global table indexed by a parameter, divides it by ten, and branches to different helper routines based on whether the result is zero.
- `render_text_string_2`: Calculates string length and renders text by copying character data from global tables into a buffer, clearing the buffer beforehand.
- `clear_buffer_entry_data`: Checks bit flags in a global variable and clears a byte in a global structure at an index derived from a register, potentially calculating string lengths before clearing the data.
- `clear_buffer_entry_if_condition`: Checks bit flags from a global variable, and if specific bits are clear, sets a byte in a global structure to zero using an index derived from registers.
- `clear_buffer_entry`: Sets a byte in a global structure to zero using an index derived from registers, then calls several other routines to process data and render text.
- `update_text_field`: Retrieves a value from a global table, checks it against a constant, and if valid, stores a character and a null terminator into a specific offset in a global structure before calling helper routines.
- `search_short_table`: Iterates through a global array of 26 short integers, comparing each against a passed parameter.
- `call_indirect_routines`: Calls another routine and then invokes a function pointer located at a specific offset from a global pointer.
- `allocate_memory_block`: Allocates a block of memory using the Exec library and returns the pointer.
- `initialize_graphics_library`: Allocates memory, opens the graphics library, and populates a data structure with pointers and values from global variables and library calls, returning a status code.
- `process_integer_list`: Iterates through a list of integers until a terminator value is found, invoking a library function for each entry.
- `process_list_with_lib`: Calls a library function, then iterates through a list of integers until a value of -1 is found, calling another library function for each entry.
- `empty_stub_3`: This routine is empty and performs no operations.
- `cleanup_allocated_memory`: Iterates through tables of function pointers and global variables to invoke cleanup routines, then frees allocated memory blocks and closes libraries.
- `initialize_game_state_structures`: Writes values to structure fields, touches the CIAA_ICR hardware register, and calls routines to build copper lists, copy data blocks, and initialize input and player states.
- `configure_cia_registers`: Writes specific values to CIAA control and interrupt registers and updates global status flags and memory offsets.
- `wait_and_set_flag`: Polls a bit in a global status word until it is set, then sets a corresponding bit in another global word and returns the status value.
- `repeat_wait_flag`: Repeats a polling and flag-setting operation a number of times determined by an input parameter, waiting for a specific bit in a global status word before setting a bit in another global word.
- `clear_memory_loop`: Calculates a count based on a global variable and an input parameter, then repeatedly clears a byte at a specific offset in a global structure, waiting for it to become non-zero before decrementing the count.
- `build_copper_list`: Constructs a sequence of graphics hardware commands in memory by writing control words and color data from global variables.
- `initialize_copper_list`: Constructs a sequence of graphics hardware commands in memory by writing control words and color data from global variables.
- `copy_block_data`: Copies a block of 17 words from a source address derived from a global pointer to a destination buffer.
- `add_offset_to_pointer`: Returns the value of a global pointer variable plus a constant offset of 96.
- `store_word_at_offset`: Stores a 16-bit value passed in a register into a memory location calculated by adding an offset to a global pointer.
- `initialize_buffer_values`: Checks a condition and either clears a buffer with a specific constant or fills a series of memory locations with another constant value.
- `copy_global_to_buffer`: Stores a value to a local offset and copies a sequence of words from a global variable area into a buffer structure.
- `update_player_coordinates`: The routine toggles a status flag and increments a counter within a game state structure.
- `dispatch_indirect_jump`: Performs an indirect jump to an address contained in a jump table, acting as a dispatcher for multiple code paths.
- `update_status_flags`: Masks and stores a value into a structure field at one offset, and sets a byte flag to 1 at another offset within a different structure.
- `set_offset_value`: Writes the value 8 to a specific offset in a data structure and returns a combined register value.
- `dispatch_jump_table_call`: Performs an indirect jump using a pointer to a jump table, dispatching execution to one of several target routines based on the table entry.
- `update_state_and_dispatch`: Decrement a counter, toggle a flag, and conditionally fill a buffer with constants or global values before dispatching to a jump table.
- `reset_state_and_dispatch`: Sets a counter to a specific value, copies global words into a buffer, and then dispatches to a jump table.
- `initialize_buffer_and_dispatch`: Checks a status flag and, if set, clears it and copies data from a source pointer into a destination buffer.
- `initialize_input_state`: Resets input counters and queue entries, reads hardware status bytes, and initializes local structure fields with specific constant values.
- `fetch_table_value_and_dispatch`: Checks a counter and a modulo condition; if met, it retrieves a value from a global table pointer, updates the pointer, and stores the value into a structure field.
- `dispatch_input_handler`: Reads keyboard input and checks button states, storing results in local variables before performing an indirect jump to a dispatcher routine.
- `process_input_queue_bounds`: Checks input state flags and coordinate bounds against a table to update a queue, handling specific bit masks for different conditions.
- `process_input_queue_loop`: This routine iterates through an array of input state structures, checking button flags and coordinate bounds to update a queue.
- `process_input_queue_state_2`: The routine iterates through a structure array to check input states and button flags, appending to a queue if conditions are met, and dispatches to an indirect jump table.
- `process_input_queue_state`: Iterates through a structure array to check input states and button flags, appending to a queue if conditions are met, and dispatches to an indirect jump table based on the result.
- `process_input_state_queue`: Iterates through a structure array, checking conditions on short values and byte flags, then processes input states and dispatches to an indirect jump table.
- `process_input_state`: Calls helper routines to process input, stores results into a local structure, decrements a counter if it is positive, and performs an indirect jump based on a table.
- `process_input_queue_3`: Checks button states and processes keyboard input, decrementing a counter if valid, then dispatches to an indirect jump table.
- `process_keyboard_input`: Reads the CIAA keyboard serial register to detect key presses.
- `set_bit_in_byte`: Sets a specific bit within a byte located at a fixed offset in a structure, where the bit position is determined by a register value.
- `read_byte_from_global`: Returns the byte value located at a fixed offset from a global pointer variable.
- `initialize_structure_fields`: Sets several byte and word fields within a structure to zero or specific constants, and then clears a block of 335 words pointed to by a pointer within that structure.
- `update_palette_and_dispatch_2`: Checks a status flag and updates a palette buffer by copying data from registers into a structure array, then increments an index with wraparound before dispatching to a jump table.
- `update_palette_and_dispatch`: Checks a status flag and updates a palette buffer by copying data from registers into a structure array, then increments an index with wrapping.
- `update_counter_and_copy_data`: Increments a byte counter in a structure if it does not match a register value.
- `read_circular_buffer_byte_2`: Reads a byte from a circular buffer using a head index, decrements the index with wrapping, and returns the shifted byte value.
- `read_circular_buffer_byte`: Reads a byte from a circular buffer using a head index, decrements the index (wrapping around if necessary), and returns the byte value shifted and adjusted.
- `clear_status_byte`: Sets a specific byte field within a structure to zero.
- `write_circular_buffer_byte_2`: Writes a byte into a circular buffer at the current tail index, decrements the index with wrapping, and processes a string until a null terminator is found.
- `write_circular_buffer_byte`: Writes a byte into a circular buffer at the current tail index, then decrements the index (wrapping around if necessary).
- `store_word_in_structure`: Stores a 16-bit value passed in a register into a specific word field within a structure.
- `decrement_timer_array`: Iterates through an array of nine word-sized values in a structure, decrementing each non-zero value by one.
- `update_input_state_2`: Initializes a structure with specific constant values for coordinates and flags, rotates a byte counter, and stores word values into indexed fields.
- `update_and_clamp_coordinates`: Calculates new coordinate values by adding offsets or computing signed distances, clamps the results within defined limits, and stores them back into the context structure before dispatching to a jump table.
- `clamp_and_dispatch_input`: Clamps two short values from a structure against limits and stores the results into specific fields.
- `dispatch_secondary_input`: Compares short values, updates input state fields, and processes secondary input before dispatching to an indirect jump table.
- `clamp_coordinates_and_call`: Calculates new coordinates by adding offsets to base values.
- `clear_structure_fields`: Retrieves pointers to two different structures from the main context.
- `calculate_signed_distance_2`: Computes the signed distance between two pairs of byte values by subtracting them, handling overflow and sign corrections, and returning the combined result.
- `calculate_signed_distance`: Computes a signed distance or difference between two integer values passed in registers.
- `return_immediately_7`: The routine contains instructions but performs no data manipulation or hardware access before returning control to the caller.
- `update_input_state`: Rotates a byte counter and uses it to index into a structure, storing two word values from global offsets into that index.
- `process_secondary_input`: Decrement a byte counter with a lower bound check, then call another routine to compute values.
- `compute_input_flags`: Calculates a composite value by adding input registers to global offsets, checking specific bit conditions, and combining the results into a structured return value.
- `check_button_state`: Reads a global variable representing hardware status and returns a boolean indicating whether a specific bit is cleared.
- `store_mouse_data`: Writes byte values from registers and memory locations into specific offsets of a global structure, while managing interrupt enable flags based on a condition.
- `store_word_data`: Stores two word-sized values from registers into specific offsets of a global structure and updates a global flag based on a memory condition.
- `store_unreached_words`: Stores two word-sized values from registers into specific offsets of a global structure, though this code path is not reached during normal execution.
- `manage_queue_operations`: Checks a state byte and a counter; if active, it waits for a flag to clear while decrementing a value.
- `clear_queue_entries`: Resets three short counters to zero and iterates through a table of word pairs, setting each pair to zero.
- `set_flag_to_one`: Sets a specific word-sized field within a global structure to the value one.
- `clear_flag_to_zero`: Sets a specific word-sized field within a global structure to the value zero, though this code path is not reached during normal execution.
- `mask_and_check_bit`: Masks an input word with a constant, stores it in a global structure, conditionally sets a byte to zero if the lowest bit is clear, and returns the lowest bit.
- `return_immediately_8`: The routine is empty and returns immediately without performing any operations, though this code path is not reached during normal execution.
- `append_to_queue`: Checks if a bit is set in a mask and if a counter is within bounds.
- `append_to_buffer_list`: Stores a pointer into a table indexed by a scaled global counter, increments the counter with a wrap-around check, and writes four words of data to the pointed-to buffer.
- `calculate_scaled_value`: Multiplies a register value by a global word, divides the result by a constant, ensures the outcome is at least one, and stores the byte result to a global variable.
- `transform_linked_list`: Iterates through a linked list of short values, copying and transforming them into a contiguous buffer.
- `render_text_string`: Clears a buffer and copies character data from two global tables based on input string characters.
- `empty_stub_4`: The routine is empty and performs no operations.
- `initialize_struct_fields`: Sets multiple fields within two global structures to specific constant values, including pointers to offsets within the structure and a reference to a global variable.
- `process_hunk_header`: Reads six words from a pointer stored in a global structure, copies them to other fields in the structure, toggles a status byte, and swaps two pointer fields based on that status.
- `call_sprite_setup_routine`: The routine calls a helper to configure sprite rendering parameters and then invokes an empty stub.
- `calculate_sprite_offset_value`: The routine calculates a memory offset by multiplying a register value by a constant and adding a base, storing the result in a global variable.
- `scan_sprite_stripes`: Iterates through a table of words using a pointer, calling a rendering routine for each entry until a negative value is encountered.
- `render_sprite_stripes`: The routine configures a graphics structure with source and destination coordinates derived from input parameters.
- `setup_sprite_rendering`: Stores three input parameters into global memory locations and then invokes two rendering routines.
- `call_sprite_stripe_renderer`: Calls an empty stub routine and then invokes the sprite stripe rendering function.
- `render_sprite_stripes_2`: Calculates memory offsets and color indices for a sprite based on global state, then iterates through a table to write stripe data to a buffer while polling a hardware status register.
- `prepare_sprite_render_state`: Configures global rendering parameters and calculates memory offsets for sprite drawing based on position flags and clipping bounds.
- `calculate_sprite_dimensions`: Computes clipping bounds and memory offsets for a graphical object based on its position and size parameters.
- `wait_and_update_nibbles`: Waits for specific byte flags at offsets 1112 and 1113 from a base pointer to become non-zero, then calls helper routines to decrement or increment nibble values in tables.
- `decrement_nibble_values_2`: Iterates through a table of 16-bit words, decrementing each non-zero nibble by one.
- `increment_nibble_if_different`: Compares nibbles of words in a destination table against corresponding words in a source table.
- `clear_pointer_arrays`: Retrieves a pointer to an array of pointers from a global variable.
- `process_jump_table_entries`: Iterates through a loop to process entries in a global table, updating pointers and flags based on character codes.
- `dispatch_jump_table_logic_2`: The routine iterates through a loop, decrementing a counter and checking values in a global table.
- `dispatch_jump_table_logic`: Iterates through a loop decrementing a counter and checking values in a global table.
- `write_jump_table_entry`: Writes a specific code pointer and a 16-bit value into a global jump table at an index determined by the second parameter.
- `write_jump_table_entry_2`: Stores a constant value, a 16-bit parameter, and a calculated code pointer into a global table at an index determined by the second argument.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `write_jump_table_entry_3`: Writes a constant value of 2, a 16-bit parameter, and a calculated code pointer into a global table at an index determined by the second argument.
- `format_string_output`: Calls a routine to parse a format string and convert arguments into a formatted output buffer.
- `copy_string_buffer`: Copies bytes from a source pointer to a destination pointer in a loop based on a count parameter, using exec library calls.
- `empty_stub_5`: This routine is empty and performs no operations other than returning immediately to the caller.
- `parse_icon_library_data`: Opens the icon library, parses string tokens, and processes data structures using DOS library calls for input and output.
- `parse_string_tokens`: The routine extracts a string from a data structure, skips leading whitespace, and tokenizes the content by splitting on spaces, tabs, or newlines while handling quoted strings.
- `parse_and_tokenize_string`: The routine processes a source string by skipping leading whitespace and extracting tokens delimited by spaces or quotes.
- `copy_string_with_limit_2`: Copies characters from a source string to a destination buffer, repeating the source content if a specified count is exceeded, and terminates the result with a null byte.
- `copy_string_with_limit_3`: Copies characters from a source string to a destination buffer, repeating the source content multiple times based on a 16-bit counter in the lower word of the third parameter.
- `open_icon_library_2`: The routine opens the icon.library using the DOS library and retrieves function pointers for icon handling.
- `append_string_to_buffer_2`: Appends a null-terminated source string to the end of a destination string by finding the terminator and copying characters until the source ends.
- `copy_string_to_null`: Locates the null terminator of the first string parameter by incrementing a pointer until a zero byte is found.
- `copy_string_with_padding`: Copies characters from a source string to a destination buffer, padding with null bytes if the source ends before a specified count is reached, or stopping early if the count is zero.
- `create_icon_node`: The routine allocates or retrieves a node structure from a global table, initializes its fields with passed parameters and computed values, and links it into a chain.
- `initialize_data_structure`: Sets specific bytes and words within a data structure pointed to by an argument, conditionally calls a function from a global table, and invokes two other functions from that table with a register value.
- `format_string_to_buffer`: Parses a format string containing percent signs and hex digits, converting arguments into a formatted output buffer while handling width, precision, and type specifiers.
- `calculate_string_length_2`: Calculates the length of a null-terminated byte string by advancing a pointer until the terminator is found and returning the difference between the final and initial addresses.
- `init_pointer_chain_node`: Stores the address of the pointer argument into the memory location it points to, adds four to that stored value, clears the next word in the block, and stores the original pointer value into the following word.
- `iterate_node_list`: The routine loops through a range of memory addresses, calling a node removal and formatting routine at each step.
- `process_node_data_recursive`: Recursively processes a linked list node structure, validating flags and indices before manipulating data buffers and file handles.
- `remove_and_format_node_2`: The routine removes a node from a linked list, formats a string by converting characters to digits, clears structure fields, and returns a status code.
- `remove_and_format_node`: Removes a node from a linked list, formats a string by converting characters to digits, and clears structure fields, returning a status code based on flag checks.
- `return_zero_stub`: The routine is empty and immediately returns zero without performing any operations or modifying state.
- `tiny_addq_addq`: Small routine of 2 instructions that adds 2 to a5, then adds 8 to d0.
- `check_icon_flags_recursive`: The routine iterates through a table of structures, checking specific flag bits in a short field.
- `check_icon_flags`: The routine iterates through a table of structures, checking specific flag bits in a short field.
- `recursive_call_stub`: The routine calls itself recursively without performing any other operations or passing arguments, leading to infinite recursion.
- `empty_stub_routine`: The routine consists of six instructions that perform no observable work and end with a return.
- `compute_signed_division`: The routine calls a division helper and adjusts the result based on the sign of the input.
- `call_and_return_concatenated`: The routine calls a division helper and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `call_division_routine`: Calls a routine that performs 32-bit integer division and then returns.
- `perform_32bit_division`: Calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1, employing a bit-shifting algorithm for large divisors or standard modulo arithmetic for smaller ones, and returns the result in a concatenated 64-bit value.
- `allocate_and_validate_node`: Allocates memory for a linked list node and validates an index against bounds.
- `remove_node_from_list`: Traverses a linked list stored in a global variable to find a node matching a specific pointer offset.
- `traverse_and_store_pointer`: Traverses a linked list of pointers until a condition is met or the list ends.
- `store_pointer_and_call_vector`: The routine stores a pointer from a register into a global variable or another pointer location depending on a condition.
- `store_pointer_and_call`: The routine stores a pointer from a register into a global variable and then calls a specific function vector from a library base table.
- `iterate_and_call_function_2`: The routine traverses a linked list of pointers stored in a global variable, invoking a specific function pointer from a global table for each node until a null pointer is reached, then clears the global variable.
- `iterate_and_call_function`: Traverses a linked list of pointers and invokes a specific function pointer from a global table for each node until a null pointer is reached.
- `allocate_linked_list_node`: The routine allocates memory for a linked list node by calling a library function.
- `copy_string_data`: Copies characters from a source string to a destination buffer byte by byte until a null terminator is encountered.
- `validate_and_check_interactive`: Validates an index against bounds and a table entry, setting an error code if invalid.
- `seek_file_position`: Validates an index against a global limit and checks a handle table, then invokes a function pointer to seek a file position, storing error codes in a global variable if the operation fails.
- `delete_file_and_check_error`: The routine calls a library function to delete a file and checks the return value.
- `return_zero_constant_2`: The routine returns a constant zero value immediately.
- `write_file_data`: Validates an index and checks file handle flags, then invokes a function pointer to write data to a file, storing error codes in a global variable if the operation fails.
- `check_and_dispatch_event`: Invokes a function pointer from a global table to retrieve a status value.
- `output_and_cleanup_callbacks`: Writes data to the standard output using library calls.
- `execute_cleanup_callbacks_2`: The routine iterates through a list of function pointers stored in a global structure, calling each one.
- `execute_cleanup_callbacks`: The routine iterates through a count stored in a global structure, calling a validation routine for each index.
- `empty_stub_6`: This routine is empty and contains no executable instructions other than a return.
- `close_handle_entry`: Validates an index and checks if a handle is open.

### bcdfq

- `main_program_entry`: Opens system libraries, initializes global variables, and enters a main loop that processes graphics, input, and hardware DMA channels, waiting for conditions before executing further routines.
- `check_cia_port_bit`: Reads the CIAA parallel port register and checks if a specific bit associated with joystick or mouse fire buttons is clear.
- `check_memory_flag`: Checks if a specific bit in a global memory variable is clear.
- `wait_for_condition`: Polls two adjacent short values in a structure, returning immediately if the first is zero, otherwise waiting until the second becomes non-zero.
- `process_graphics_loop`: Executes a loop that disables interrupts, waits for the top of frame, processes a table of graphics objects by updating their properties, and re-enables interrupts.
- `open_file_resource`: Calls a routine to open a file resource referenced by a global pointer and stores the resulting handle in a global variable.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `copy_block_words`: Copies a block of 3000 words from a source pointer to a destination pointer.
- `clear_and_copy_block`: Calls a function pointer from a global structure, then copies 8000 words from a source buffer to a destination buffer.
- `loop_call_until_flag`: Iterates through a table of short values, calling a function pointer from a global structure for each non-zero entry.
- `process_input_queue`: Iterates through a global table of short integers, calling a function pointer from the execution context for each non-zero entry until a flag in the context indicates completion.
- `process_input_queue_2`: Iterates through a global table of short integers, invoking a function pointer from the current task's execution context for each non-zero entry.
- `call_function_pointer`: Calls a single function pointer located at a negative offset from a base address stored in a global structure.
- `loop_decrement_nibbles`: The routine executes a countdown loop from fifteen to zero.
- `decrement_nibble_values_3`: Iterates through an array of words, decrementing each of the three 4-bit nibble fields by one if they are non-zero.
- `loop_increment_nibbles`: The routine executes a countdown loop from fifteen to zero.
- `increment_nibble_on_mismatch`: Iterates through two arrays of words in parallel.
- `blit_multi_source_blocks`: Configures and executes multiple blitter operations using hardware registers.
- `set_dma_channel_mask`: Masks the lower six bits of an input byte.
- `loop_clear_global_flag`: The routine executes a countdown loop from fifteen to zero, calling another routine in each iteration.
- `configure_dma_channels`: The routine calculates a bitmask based on an input register and updates a global variable if the input is small.
- `enter_infinite_loop`: The routine contains no executable instructions and is effectively empty, immediately entering an infinite loop.
- `conditional_pointer_copy`: The routine checks a bit in the input register and a field in a structure; if both conditions are met, it copies a pointer and a 16-bit value from the structure to a destination pointed to by another field in the structure.
- `process_dma_pointer_copy`: The routine checks a global flag; if set, it calls several helper routines and processes a pointer structure.
- `process_copper_list_entries`: Iterates through a copper list structure, decoding instruction bytes to configure DMA channels and update hardware registers.
- `update_dma_pointer_flags`: Updates memory locations by setting and clearing specific bits in byte and word fields.
- `process_character_loop`: Loops through characters, updating state variables and checking bounds.
- `update_character_state`: Adjusts character state variables based on register values, iterates through a table of character structures to update bounds and flags, and sets global flags if specific conditions are met.
- `process_character_state_loop`: Iterates through a table of character structures, updating state variables and checking bounds against a global counter.
- `dispatch_character_handler`: Similar to R7, it loops through characters and updates state variables.
- `process_dma_pointer_copy_2`: The routine checks a global flag and processes a pointer structure by updating memory locations and calling helper routines.
- `decode_table_entry`: The routine reads a 16-bit value from a table in a global variable using an index derived from the input, or computes a division if the index is out of range, then stores the low and high bytes of the result into two global byte locations.
- `fetch_table_or_divide`: Checks if the input word is less than eleven; if so, it retrieves a value from a global table using the input as an index.
- `lookup_or_divide_table_entry`: Checks if the input index is within bounds to read a 16-bit value from a global table; otherwise, it divides a global constant by the input.
- `tiny_ori_2`: Small routine of 1 instruction that merges 240 into d7.
- `store_global_and_return_offset`: The routine stores the address of a global variable into a slot indexed by an uninitialized register and returns the sum of the value pointed to by the input pointer and the input pointer plus six.
- `add_to_pointer_if_nonzero`: The routine receives a pointer in a0 and a value in d1.
- `adjust_pointer_offsets`: The routine calls several helper routines in a loop until a condition is met.
- `scale_palette_from_source`: The routine reads a scaling byte from a structure pointed to by the argument and multiplies a 16-byte source table by that byte, storing the scaled 16-bit results into a global variable table.
- `handle_ciab_resource_flags`: Checks a global variable and manipulates flag bits in memory locations associated with the CIA hardware.
- `check_ciab_and_call_handler`: Checks a global variable and calls a handler if non-zero.
- `clear_flag_and_call_handler`: The routine clears the least-significant bit of a byte stored through a pointer and then clears a global variable that holds a handler pointer.
- `open_ciab_resource`: The routine checks a version character in a global structure, then attempts to open the 'ciab.resource' file using the exec library.
- `close_ciab_resource`: The routine checks if a global resource base pointer is non-zero.

### bcdft

- `initialize_and_free_memory`: The routine serves as the program entry point.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.

### bcdfu

- `initialize_game_environment`: The routine opens the dos, graphics, and intuition libraries, brings the screen to the front, and disables DMA channels.
- `iterate_null_terminated_string`: The routine loops through a null-terminated string, calling another routine for each character until the null terminator is reached.
- `loop_copy_tile_data`: The routine iterates through a null-terminated string.
- `copy_tile_data_block`: Copies eight bytes from a source buffer to five destination locations spaced by 8000 bytes, using offsets derived from input registers.
- `initialize_graphics_blit`: Calls multiple routines to initialize graphics state, including loading RGB4 data and configuring Blitter registers.
- `load_palette_colors`: Multiplies an input value by a global variable, waits for the accumulator to clear, and calls the graphics library to load RGB4 color data.
- `check_ciaa_bit_six`: Reads the CIAA parallel port register and returns true if bit six is clear.
- `check_potgor_bit_two`: Reads the POTGOR register and returns true if bit two is clear.
- `clear_main_buffer`: Sets four consecutive longwords to zero in a loop, repeating the operation 6000 times to clear a large memory block.
- `clear_offset_buffer`: Sets four consecutive longwords to zero in a loop, repeating the operation 500 times on a memory block offset by 32000 bytes.
- `multiply_accumulator_values`: Multiplies an input register value by a global variable and stores the result in another global variable.
- `wait_for_zero_accumulator`: Loops continuously until a specific global variable becomes zero.
- `wait_for_flag_or_zero`: Loops until a primary global variable becomes zero or a secondary global variable becomes non-zero.
- `process_animation_frames`: Disables interrupts, waits for the top of frame, updates animation counters and pixel values for three objects over 105 iterations, then re-enables interrupts.
- `wait_for_ram_flag`: Polls a byte at a fixed offset in Chip RAM until it equals the value 15.
- `open_data_file`: Calls the DOS library to open a file named 'bcdfv' and stores the resulting file handle in a global variable.
- `close_data_file`: Calls the DOS library to close the file handle stored in a global variable.
- `decode_rle_chunk_one`: Invokes a function pointer from a global structure and decodes run-length encoded data from a source to a destination buffer.
- `decode_rle_data_2`: Decodes run-length encoded data from a source pointer to a destination pointer.
- `read_file_data`: Calls the DOS library to read data from the currently open file handle into a buffer.
- `read_and_decode_rle_data`: Invokes the DOS Read library function to fetch data, then passes the buffer to a routine that decodes run-length encoded information.
- `read_file_data_again`: Calls the DOS library to read data from the currently open file handle into a buffer.
- `shift_buffer_data`: Copies a block of 32-bit words from a higher memory address to a lower address, overlapping the source and destination.
- `decode_and_call_global`: Calls a function pointer stored in a global variable, then invokes a routine to decode run-length encoded data.
- `invoke_decode_rle`: Calls a function pointer stored in a global structure to trigger data decompression.
- `clear_buffer_region`: Sets a large block of 32-bit words to zero in a specific memory region.
- `decode_rle_data`: Decodes run-length encoded data from a source pointer to a destination pointer.
- `decrement_tile_timers`: Loops sixteen times calling function pointers and a tile decrement routine, then sets a global status word to zero.
- `decrement_tile_values`: Iterates through an array of 16-bit values and decrements specific nibble fields if they are non-zero.
- `call_load_r_g_b4`: Library stub: loads its arguments and the graphics library base, calls LoadRGB4 and returns its result to the caller.
- `wait_for_library_completion`: Loops sixteen times calling function pointers and a wait routine, then sets a global status word to one.
- `process_nibble_differences`: Loops sixteen times calling function pointers and a nibble comparison routine, then sets a global status word to one.
- `compare_and_increment_tiles`: Compares corresponding 16-bit values from two arrays and increments specific nibble fields in the first array if they differ from the second.
- `decode_nibble_differences`: Iterates through two arrays of words, comparing corresponding 4-bit nibbles.
- `initialize_graphics_buffers`: The routine initializes a memory block by setting words to a specific value, then calls routines to clear buffers, decode data, load graphics, and process differences.
- `check_input_ports`: Checks bit six of the CIAA parallel port register and bit two of the POTGOR register to determine input state.
- `blit_copper_list_segments_2`: Iterates through a table of offsets to configure Blitter registers for multiple copy operations, waiting for the Blitter to finish each step before proceeding to the next segment.
- `blit_copper_list_segments`: Owns the Blitter and iterates through a table of offsets to configure Blitter registers for multiple copy operations.
- `blit_vertical_stripes_2`: Configures the Blitter to copy data from a source buffer to a destination buffer with specific stride and size parameters, looping until a termination condition in a global variable is met.
- `blit_vertical_stripes`: Configures the Blitter to copy data from a source buffer to a destination buffer with specific stride and size parameters.
- `copy_tile_data_blocks`: Copies blocks of 32-bit words from a source pointer to a destination pointer in a nested loop structure.
- `blit_conditional_pattern`: Calculates Blitter parameters based on an input value, setting different source and destination pointers and sizes depending on whether the input is negative or positive.
- `blit_conditional_pattern_2`: Calculates Blitter parameters based on an input value, setting different source and destination pointers and sizes depending on whether a global flag is set, repeating the operation in a countdown loop.
- `set_dma_channel_mask_2`: Masks the low bits of an input byte and, if the result is less than four, stores a single-bit mask derived from those bits into a global variable.
- `self_recursive_loop_with_flag_clear`: Sets a short local variable to a constant and repeatedly calls another routine in a countdown loop until the variable reaches -1, then clears a global variable and returns.
- `update_cia_timer_values_2`: Stores register values into specific offsets of a structure and a global variable, likely updating CIA timer registers or related control state, while managing DMA channel masks.
- `update_timer_structure_fields`: The routine updates fields in a structure and a global variable based on input values, handling conditional logic to store timer-related data.
- `store_timer_register_values`: Writes specific register values into offsets of a structure and a global variable, handling conditional logic based on an input value to determine which values are stored.
- `update_cia_timer_values`: Stores register values into specific offsets of a structure and a global variable, likely updating CIA timer registers or related control structures for hardware timing or interrupt handling.
- `empty_routine_2`: The routine contains no executable instructions and is effectively empty.
- `shift_and_update_pointer`: Checks a condition on an input byte and a global pointer.
- `shift_and_update_pointer_2`: Checks a condition on an input byte and a global pointer, shifting the byte and updating a pointer structure if the condition is met, while managing DMA channel states.
- `enable_dma_and_process_stream`: Writes to the DMACON hardware register to enable DMA channels and calls a routine to process a byte stream in a loop.
- `clear_flags_and_enable_dma`: Calls routines to clear flags, enable DMA timers, and update DMA and CIA timer structures by writing to the DMACON register.
- `clear_flag_and_update_dma`: This routine invokes a helper to clear a recursive flag via a countdown loop and then calls a function to write to the DMACON register and update CIA timer structures.
- `recursive_flag_clear_loop`: This routine sets a local variable and enters a recursive loop that calls another routine until a countdown condition is met.
- `enable_dma_channels_3`: The routine enables DMA channels by writing to the DMACON hardware register and updates CIA timer structures.
- `enable_dma_channels_2`: Masks input bits to generate a DMA channel mask and writes to the DMACON hardware register to enable DMA channels, updating CIA timer structures.
- `enable_dma_timers`: The routine calls a helper to write to the DMACON hardware register and update CIA timer structures.
- `trigger_dma_cia_update`: Calls a routine to write to the DMACON hardware register to enable DMA channels and store values into CIA timer-related structures.
- `update_dma_cia_timers`: Calls routines to update CIA timer structures and enable DMA channels by writing to the DMACON register.
- `update_cia_and_dma_timers`: This routine calls a function to update CIA timer structures by writing to the DMACON register, followed by another call to enable DMA channels and store values into CIA timer-related structures.
- `update_cia_timers`: The routine updates CIA timer-related structures by writing to the DMACON hardware register.
- `update_dma_timers_lookup`: This routine updates DMA and CIA timer settings by writing to the DMACON register and storing values in timer structures.
- `enable_dma_and_cia_timers`: Calls a routine that writes to the DMACON hardware register to enable DMA channels and stores values into CIA timer-related structures.
- `update_dma_and_cia_timers`: Writes to the DMACON hardware register to enable DMA channels and stores values into CIA timer-related structures.
- `enable_dma_channels`: Writes to the DMACON hardware register with bit 15 set to enable DMA channels.
- `process_byte_stream_loop`: The routine processes a byte stream in a loop, shifting bytes and updating pointers, then performs conditional checks and indirect jumps based on global variables.
- `lookup_or_divide_table_entry_2`: Checks if an input word is less than eleven.
- `lookup_or_divide_value`: The routine checks if an input word is less than eleven; if so, it retrieves a value from a global table using the input as an index, otherwise it divides a global constant by the input.
- `decode_table_entry_2`: Reads a 16-bit value from a table in a global variable using an index derived from the input, or computes a division if the index is out of range, then stores the low and high bytes of the result into two global byte locations.
- `store_global_and_return_offset_2`: The routine stores the address of a global variable into a memory location offset from a base register.
- `store_global_ptr_and_add_offset`: Stores the address of a global variable into a slot in a table indexed by a register.
- `add_to_pointer_if_nonzero_2`: The routine receives a pointer and a value.
- `add_to_pointer_if_nonzero_2_2`: Receives a pointer and a value, iterating through a structure to add offsets to pointers if a condition is met, and returns a value based on the state of a global variable.
- `scale_palette_from_source_2`: Reads a scaling byte from a structure pointed to by the argument and multiplies a 16-byte source table by that byte, storing the scaled 16-bit results into a global variable table; if a secondary structure pointer is null or its count field is zero it clears a 64-byte region of the global variable, otherwise it copies a variable-length block from the secondary structure into the global variable.
- `read_joystick_and_ciab_state`: Reads the CIAA_PRA hardware register to detect joystick or mouse button inputs.
- `process_mmd0_input`: The routine reads joystick states via CIAA_PRA, checks a global variable, modifies memory at specific offsets in a structure, and opens the ciab.resource file if conditions are met.
- `process_mmd0_input_alt`: The routine reads joystick states via CIAA_PRA, checks a global variable, modifies memory at specific offsets in a different structure, and opens the ciab.resource file if conditions are met.
- `read_joystick_input`: Reads the CIAA_PRA hardware register to access joystick or mouse fire button states.
- `clear_flag_and_call_handler_2`: The routine clears the least-significant bit of a byte stored through a pointer, clears a global handler pointer, and calls the handler if it is non-zero.
- `open_ciab_resource_2`: Checks a global byte for a specific character value and opens the ciab.resource file using the exec library.
- `call_function_if_global_set`: The routine checks if a global variable is non-zero.
- `empty_routine_3`: The routine is empty, containing only bad instruction data that truncates control flow.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `halt_on_bad_data`: The routine contains no valid instructions and immediately terminates execution due to corrupted or invalid code data.
- `halt_on_bad_data_2`: The routine contains no valid instructions and immediately terminates execution due to corrupted or invalid data.
- `halt_on_bad_data_3`: The routine contains no valid instructions and immediately terminates execution due to corrupted or invalid data.
- `halts_on_bad_data`: The routine contains no valid executable instructions and immediately terminates control flow due to invalid data.
- `halt_on_bad_data_4`: The routine is empty and contains no valid executable instructions, terminating immediately due to bad instruction data.
- `empty_return_routine_4`: The routine is empty and returns immediately to the caller.
- `decode_invalid_data`: The routine contains only invalid or unreached data and performs no executable operations.
- `abort_invalid_code`: The routine contains no valid instructions and immediately terminates execution due to corrupted or unmapped data in the instruction stream.
- `halt_execution`: The routine is empty or contains invalid data and performs no operations.
- `halt_on_bad_data_5`: The routine contains no valid instructions and immediately terminates execution due to invalid data.
- `invalid_code_gap`: The routine consists of a single instruction that is invalid or corrupt data.
- `abort_on_invalid_code`: The routine encounters malformed or non-instruction data at its entry point and immediately terminates execution flow.
- `decode_invalid_data_2`: The routine contains no valid instructions and terminates immediately due to corrupted or non-code data in the segment.
- `abort_bad_instruction`: The routine contains no valid 68000 instructions.
- `halt_on_bad_data_6`: The routine contains no valid instructions and immediately terminates execution due to corrupted or invalid data at the entry point.
- `return_immediately_16`: The routine contains no instructions other than the return operation.
- `return_immediately_17`: The routine is empty and returns immediately without performing any operations.
- `empty_return_routine_2`: The routine is empty and performs no operations other than returning immediately to the caller.
- `halt_on_bad_data_7`: The routine is empty and contains no valid instructions, as indicated by the bad instruction data warning.
- `halt_execution_2`: The routine is empty and contains no valid instructions, likely due to corrupted data or an incomplete binary segment.
- `halt_on_invalid_data`: The routine contains no valid instructions and immediately terminates execution due to corrupted or unmapped code data.
- `return_immediately_18`: The routine contains no instructions and returns immediately.
- `empty_routine_13`: The routine is empty and contains no executable instructions.
- `return_immediately_19`: The routine contains no instructions other than the return opcode.
- `unknown_entry_routine`: The routine contains a single instruction that the disassembler cannot decode, so control flow is truncated.
- `decode_unreachable_gap`: This routine is an artifact of the disassembler interpreting non-executable data as code.
- `abort_bad_instruction_2`: The routine contains no valid executable instructions and terminates immediately due to invalid data.
- `unknown_routine`: The routine contains no valid instructions and is truncated by the disassembler, so it performs no observable operations.
- `halt_on_bad_data_8`: The routine is empty and contains no valid instructions, causing execution to stop due to invalid data encountered at this memory location.
- `handle_bad_instruction_3`: The routine contains no recoverable instructions.
- `decode_truncated_block`: The routine contains no valid instructions and ends immediately.
- `handle_invalid_instruction`: The routine contains no valid executable instructions and immediately terminates due to corrupt or non-code data.
- `unreached_gap_decode`: The routine is empty and contains no executable instructions.
- `end_of_segment`: This routine contains no valid instructions, marking the end of the executable code segment.
- `halt_on_bad_data_9`: The routine contains no valid instructions and immediately terminates execution due to corrupted or invalid data.
- `handle_invalid_instruction_2`: The routine contains no valid executable code, causing the analyzer to truncate control flow due to bad instruction data.
- `halt_due_to_corrupt_data`: The routine terminates execution immediately.
- `halt_on_bad_data_10`: The routine is empty because the entry point lies within a gap of unreached data that does not decode into valid instructions.
- `halt_on_bad_data_11`: The routine is empty and contains no valid instructions.
- `empty_unreached_code`: The routine is empty and contains no valid instructions.
- `halt_on_bad_data_12`: The routine is empty or contains invalid data, causing the decompiler to truncate control flow.
- `halt_on_bad_data_13`: The routine contains no valid executable instructions and terminates immediately due to invalid data in the code segment.
- `abort_on_invalid_code_2`: The routine is empty or contains invalid data, causing the decompiler to truncate control flow.
- `handle_invalid_data`: The routine contains no valid instructions and immediately terminates execution due to corrupted or non-code data at the entry point.
- `terminate_execution`: The routine contains no valid executable instructions, causing immediate termination of the execution flow.
- `unreachable_code_gap`: The routine contains no valid instructions and is unreachable, likely representing a decoding gap in the binary.
- `empty_routine_14`: This routine is empty and contains no executable instructions.
- `abort_execution`: The routine contains no valid instructions.
- `invalid_data_detected`: The routine contains invalid or corrupted instruction data, causing the execution flow to terminate immediately.
- `abort_on_bad_data`: The routine contains no valid instructions and immediately terminates execution due to invalid data at the entry point.
- `terminate_on_bad_data`: The routine is empty and contains no valid instructions, causing immediate termination of control flow due to corrupted or unreadable data.
- `abort_on_invalid_code_3`: The routine contains no valid instructions and immediately terminates execution due to invalid data.
- `handle_invalid_data_2`: The routine contains no valid instructions and terminates immediately due to malformed data.
- `decode_invalid_data_3`: The routine contains no valid instructions and decodes as garbage data.
- `terminate_execution_2`: The routine is empty or contains invalid data, causing immediate termination of the control flow.
- `invalid_code_halt`: This routine contains no valid 68000 instructions.
- `call_unnamed_routine`: The routine executes a single call to an unnamed function and returns immediately.
- `decode_invalid_data_4`: The routine contains no valid instructions and terminates immediately due to invalid data.
- `handle_invalid_instruction_3`: The routine terminates immediately due to corrupted or missing instruction data at its entry point.
- `handle_bad_data`: The routine contains no valid instructions and the disassembler stops control flow because of bad instruction data.
- `halt_on_bad_data_14`: The routine contains no valid instructions and immediately terminates due to invalid data.
- `halt_on_bad_data_15`: The routine contains no valid instructions and immediately terminates execution due to invalid data.
- `halt_on_bad_instruction_4`: The routine contains no valid instructions and the decompiler truncates control flow, so it performs no observable operations.
- `empty_routine_15`: This routine contains no valid instructions and performs no operations.
- `abort_invalid_code_2`: The routine is empty and contains no valid instructions, likely indicating a corrupted or uninitialized code segment.
- `decode_invalid_instruction`: The routine contains no valid executable instructions.
- `halt_on_bad_data_16`: The routine contains no valid instructions and immediately terminates execution due to invalid data.
- `handle_bad_instruction_data`: The routine contains no usable instructions and the decompiler stops at a bad instruction.
- `halt_on_bad_data_17`: The routine is empty and contains no valid instructions.
- `empty_routine_4`: The routine contains no usable instructions; the disassembler reports bad instruction data and truncates control flow, so the routine does nothing.
- `halt_on_bad_data_18`: The routine contains no valid instructions and immediately terminates execution due to corrupted or invalid data at the entry point.
- `decode_invalid_data_5`: The routine contains no valid instructions and terminates immediately due to malformed data.
- `halt_on_bad_data_19`: The routine is empty and contains no valid instructions.
- `halt_on_bad_data_20`: The routine contains no valid instructions and immediately terminates execution due to invalid data.
- `empty_routine_5`: The routine contains no valid instructions and is truncated by the disassembler.
- `invalid_code_stub`: The routine consists of a single instruction that the decompiler failed to parse, indicating corrupted code or data interpreted as executable instructions.
- `abort_on_bad_data_2`: The routine is empty and contains no valid instructions.
- `halt_on_bad_data_21`: The routine contains no valid instructions and immediately terminates execution due to invalid data.
- `halt_on_invalid_code`: Terminates execution immediately due to encountering non-executable data or bad instruction bytes in the runtime segment.
- `handle_bad_instruction`: The routine contains invalid or corrupted instruction data, causing the control flow to terminate immediately.
- `halt_on_bad_instruction`: The routine consists solely of a bad instruction detected by the decompiler, causing control flow to truncate.
- `halt_on_bad_data_51`: The routine contains no valid instructions and the decompiler inserts a stop for bad instruction data.
- `halt_on_bad_data_22`: The routine is empty or contains invalid data, causing the decompiler to truncate control flow.
- `empty_routine_6`: The routine contains no usable instructions and ends immediately due to bad instruction data.
- `halt_on_bad_instruction_2`: The routine terminates execution due to encountering invalid or corrupted instruction data in the binary stream.
- `halt_on_bad_data_23`: The routine is empty and contains no valid instructions.
- `abort_on_bad_instruction`: The routine is empty or invalid due to corrupted data or a decompilation error.
- `halt_baddata`: The routine contains no valid instructions, consisting solely of data or corrupted code.
- `invalid_code_gap_2`: The routine consists of a single instruction that decodes as invalid data, indicating an unreached gap in the code segment.
- `abort_on_bad_data_3`: The routine is empty and contains no valid instructions.
- `halt_on_bad_data_24`: The routine is empty and contains no valid instructions, likely representing an unreached code gap or corrupted data rather than executable logic.
- `halt_on_bad_data_25`: The routine is empty and terminates immediately.
- `end_of_code_segment`: The routine contains no valid instructions and marks the termination of the executable flow in this segment.
- `halt_on_bad_data_26`: The routine contains no valid executable instructions and immediately terminates control flow due to invalid data.
- `decode_invalid_data_6`: The routine contains no valid executable instructions and is identified as unreached code within a data gap.
- `empty_routine_7`: The routine contains no usable instructions; Ghidra reports bad instruction data and truncates control flow, so it performs no observable operations.
- `empty_routine_8`: The routine contains no reachable instructions and ends immediately.
- `decode_invalid_data_7`: The routine contains no valid instructions and terminates immediately due to invalid data.
- `abort_invalid_instruction`: The routine contains invalid machine code, causing the decompiler to truncate control flow.
- `abort_on_bad_data_4`: The routine is empty and contains no valid instructions.
- `decode_invalid_data_8`: The routine contains no valid instructions and is marked as unreached code.
- `handle_invalid_data_3`: Encounters malformed instruction data at entry and terminates execution flow.
- `decode_invalid_data_9`: The routine contains no valid instructions and is unreachable.
- `abort_invalid_code_3`: The routine contains no valid instructions and immediately terminates execution due to corrupted or unmapped data.
- `halt_on_bad_data_27`: The routine is empty and contains no valid instructions.
- `halt_on_bad_data_28`: The routine contains no valid instructions and terminates immediately due to corrupted or non-executable data at its entry point.
- `decode_unreached_gap`: The routine is empty and contains no valid instructions.
- `decode_invalid_data_10`: The routine is empty due to corrupted instruction data at the entry point.
- `stop_execution_flow`: The routine contains no valid instructions and immediately terminates control flow.
- `decode_invalid_data_11`: The routine contains no valid instructions and consists of data or garbage bytes.
- `handle_bad_instruction_4`: The routine contains an instruction that the disassembler cannot decode, so control flow is truncated.
- `halt_on_bad_data_52`: The decompiler reports only a single instruction at this entry point and flags it as invalid data, so the routine's body never decoded into real 68000 operations.
- `halt_on_bad_data_29`: The routine is empty and contains no valid instructions, likely due to corrupted data or a decompilation error.
- `halt_on_bad_data_30`: The routine contains no valid instructions and terminates immediately due to invalid data at the entry point.
- `halt_on_bad_data_31`: The routine contains no valid 68000 instructions and immediately terminates execution due to decoded garbage data.
- `halt_on_bad_data_32`: The routine contains no valid instructions and immediately terminates execution due to malformed data.
- `decode_invalid_data_12`: The routine contains no valid instructions and is unreachable.
- `handle_bad_instruction_5`: The routine contains no valid instructions and the disassembler stops the control flow because the bytes at the entry point are not a recognizable 68000 opcode.
- `decode_invalid_data_13`: The routine contains no valid instructions and is unreachable.
- `halt_on_bad_data_33`: The routine is empty and contains no valid instructions.
- `abort_on_bad_data_5`: The routine contains no valid instructions and immediately terminates execution due to invalid data encountered at its entry point.
- `bad_instruction_trap`: The routine contains no valid instructions and immediately terminates due to invalid data.
- `halt_execution_3`: The routine is empty or contains invalid data, causing immediate termination of control flow.
- `unreached_gap_routine`: The routine is empty and contains no executable instructions.
- `null_instruction_padding`: This routine contains no valid instructions and acts as a data padding or an empty execution sink.
- `abort_execution_2`: The routine contains no valid instructions and terminates immediately.
- `stop_on_invalid_data`: The routine is empty and contains no executable instructions.
- `halt_on_bad_data_34`: The routine contains no valid instructions and terminates immediately due to invalid data in the code stream.
- `halt_on_bad_data_35`: The routine contains no valid 68000 instructions and immediately terminates execution due to corrupted or non-code data at the entry point.
- `unreachable_gap_stub`: The routine is empty and contains no valid instructions.
- `abort_invalid_data`: Terminates execution due to encountering invalid or malformed instruction data in the code stream.
- `unreached_gap_decode_2`: The routine is empty and contains no executable instructions.
- `abort_execution_flow`: The routine contains no valid instructions and immediately terminates execution due to corrupt or invalid data.
- `abort_invalid_instruction_2`: Terminates execution flow immediately due to encountering invalid or corrupted instruction data.
- `halt_on_bad_data_36`: The routine contains no valid instructions and terminates execution immediately due to corrupted or invalid data at the entry point.
- `empty_routine_16`: The routine contains no executable instructions.
- `empty_routine_9`: The routine contains no usable instructions; the disassembler reports bad instruction data and the control flow is truncated immediately.
- `empty_placeholder`: The routine contains no valid instructions and is effectively empty.
- `empty_routine_17`: This routine is empty and contains no valid instructions.
- `halt_on_bad_data_37`: The routine contains no valid instructions and immediately terminates execution due to corrupted or invalid data.
- `trap_bad_instruction_data`: The routine consists of a single instruction that the decompiler could not decode, so its control flow is truncated at that point.
- `halt_execution_4`: The routine is empty and contains no valid instructions, likely indicating a truncated or corrupted code segment.
- `abort_bad_data`: The routine is empty and contains no valid instructions.
- `abort_on_invalid_data`: The routine contains no valid executable instructions and immediately terminates execution due to malformed code data.
- `abort_on_bad_data_6`: The routine contains no valid executable instructions.
- `halt_on_bad_data_38`: The routine contains no valid instructions and immediately terminates execution due to invalid data.
- `halt_on_bad_data_39`: The routine contains no valid instructions and immediately terminates execution due to invalid data.
- `mark_bad_instruction`: The routine is empty and contains no executable instructions.
- `empty_routine_10`: The routine contains no usable instructions.
- `halt_on_bad_data_40`: The routine is empty and contains no executable instructions.
- `halt_on_bad_data_41`: The routine contains no valid instructions and immediately terminates execution due to invalid code data.
- `empty_routine_18`: This routine contains no executable instructions.
- `empty_routine_11`: The routine contains no usable instructions; the disassembler truncates control flow because the bytes at the entry are not valid 68000 opcodes.
- `decode_invalid_data_14`: The routine contains no valid executable instructions.
- `halt_execution_5`: The routine is empty or contains invalid data, terminating control flow immediately without performing any operations.
- `handle_invalid_data_4`: The routine contains no valid 68000 instructions due to corrupted or non-code data at this address.
- `mark_gap_unreachable`: This routine is empty and contains no executable instructions.
- `truncated_control_flow`: The routine contains bad instruction data and the decompiler truncates control flow, so no further instructions are recovered.
- `halt_on_bad_data_42`: The routine contains no valid instructions and immediately terminates execution due to invalid data.
- `unreached_code_fragment`: The routine contains no valid instructions and is therefore empty.
- `halt_on_bad_data_43`: The routine contains no valid instructions and immediately terminates execution due to corrupted or invalid data at the entry point.
- `handle_bad_instruction_6`: The routine contains no valid instructions and the decompiler truncates control flow at a bad instruction.
- `decode_unreachable_gap_2`: The routine is empty because the entry point lies in an unreached gap where data decodes as invalid code.
- `halt_invalid_code`: The routine consists of invalid machine code, causing execution to terminate immediately.
- `handle_invalid_data_5`: The routine is empty or contains invalid data due to a decoding error in the executable stream.
- `terminate_flow_unexpectedly`: The routine contains invalid data and immediately terminates execution due to bad instructions.
- `unreachable_code_stub`: This routine is empty and contains no valid instructions.
- `execute_invalid_instruction`: This routine consists of a single byte of data that does not form a valid instruction.
- `halt_on_bad_data_44`: The routine contains no valid instructions and terminates immediately due to corrupted or invalid code data.
- `decode_unreachable_gap_3`: The routine is empty because it is located in an unreached code gap.
- `halt_on_bad_data_45`: This routine is empty or contains invalid data, causing the decompiler to fail.
- `decode_unreachable_gap_4`: This routine is empty and contains no valid instructions.
- `abort_bad_instruction_3`: The routine is empty due to corrupted or non-code data at the entry point.
- `halt_invalid_code_2`: The routine contains no valid executable instructions.
- `trap_on_bad_instruction`: The routine contains a single instruction that the disassembler could not decode as valid 68000 code, so decompilation stops at it.
- `decode_invalid_data_15`: The routine is empty and contains no valid instructions.
- `handle_invalid_data_6`: The routine contains no valid instructions and immediately terminates execution due to corrupted or missing code data.
- `halt_execution_6`: The routine contains no valid instructions and immediately terminates control flow.
- `empty_unreached_routine`: The routine is empty and contains no executable instructions.
- `halt_on_bad_data_46`: The routine contains no valid executable instructions.
- `abort_execution_3`: The routine is empty and contains no valid instructions.
- `halt_on_bad_data_47`: The routine contains no valid 68000 instructions.
- `halt_execution_7`: The routine is empty due to bad instruction data at the entry point.
- `halt_execution_8`: The routine contains no valid instructions and terminates immediately due to invalid data.
- `halt_baddata_2`: The routine is empty and contains no valid instructions.
- `abort_on_invalid_data_2`: The routine contains no valid executable instructions and terminates immediately due to corrupted or non-code data at its entry point.
- `halt_on_bad_data_48`: The routine is empty or contains invalid data, causing immediate termination.
- `decode_invalid_data_16`: The routine contains no valid instructions and is marked as unreached code within a data gap.
- `handle_bad_instruction_2`: The routine is empty or consists solely of invalid data, causing the disassembler to truncate control flow immediately upon entry.
- `halt_on_invalid_data_2`: The routine consists of a single instruction that terminates execution upon encountering malformed data.
- `decode_invalid_data_17`: The routine consists of a single instruction that is invalid or corrupted, causing the control flow to terminate immediately.
- `halt_on_bad_data_49`: The routine contains no valid executable instructions, causing immediate execution failure.
- `bad_instruction_handler`: The routine contains no valid instructions and the disassembler stops at a bad instruction, so it performs no observable operations.
- `decode_invalid_data_18`: The routine contains no valid instructions and represents a gap in the code space where data was incorrectly decoded as code.
- `halt_execution_9`: The routine contains no valid instructions and immediately terminates control flow.
- `decode_invalid_data_19`: The routine contains no valid instructions and is identified as unreached code within a data gap.
- `abort_invalid_code_4`: The routine contains no valid instructions and immediately terminates execution due to corrupted or unmapped data.
- `halt_on_bad_instruction_3`: The routine contains no valid instructions.
- `decode_invalid_instruction_2`: The routine consists of a single byte that does not form a valid Motorola 68000 instruction.
- `halt_bad_data`: The routine contains no valid instructions and acts as a dead end for control flow.
- `call_decrement_word`: Calls another routine that decrements a 16-bit value at a fixed offset from a base pointer register and returns the resulting zero value.
- `call_indirect_pointer`: Executes a function pointer stored at a specific memory location.
- `call_indirect_function_pointer`: Executes a function located at a specific memory address stored in a global variable.
- `call_indirect_function_pointer_2`: Invokes a routine via an indirect jump through a global pointer variable.
- `call_indirect_function_pointer_3`: Invokes a routine via an indirect jump through a memory pointer and returns the result in the data register.
- `call_indirect_function_pointer_4`: Executes a function located at a specific memory address stored in a global variable.
- `call_indirect_pointer_2`: Executes a routine located at an address stored in a global pointer variable.
- `call_indirect_function_pointer_5`: Executes a function located at a specific memory address stored in a global variable.
- `call_indirect_pointer_3`: The routine executes a function pointer located at a fixed memory address and returns the result.
- `call_indirect_function_pointer_6`: Invokes a routine via an indirect jump through a memory pointer.
- `call_indirect_function`: Executes a function pointer stored at a specific global address.
- `call_indirect_function_pointer_7`: Executes a function located at a specific memory address stored in a global variable.
- `return_zero_value`: The routine immediately returns a zero value without executing any other instructions or modifying memory.
- `increment_word_at_offset`: The routine increments a 16-bit value located at a fixed positive offset from a base pointer register in place and then returns.
- `decrement_word_at_offset`: The routine decrements a 16-bit value located at a fixed positive offset from a base pointer register and then returns a zero value.
- `handle_bad_instruction_data_2`: The routine contains no valid instructions; the disassembler stops because the bytes at the entry cannot be decoded as 68000 opcodes.
- `halt_on_bad_data_50`: The routine contains no valid instructions and terminates immediately due to unreached or corrupted code data.
- `abort_invalid_code_5`: The routine contains no valid instructions and immediately terminates execution due to malformed data.
- `decode_and_halt`: The routine contains no valid instructions and the disassembler truncates control flow.
- `empty_routine_12`: The routine contains 16 instructions but Ghidra cannot decode them and reports bad instruction data, so the control flow is truncated.
- `empty_return_routine_3`: The routine performs no operations other than returning to the caller.
- `empty_return_routine`: The routine performs no operations other than returning to the caller.
- `tiny_movea_rts`: Small routine of 2 instructions that copies (a7)+ to a5, then returns.
- `single_instruction_routine`: The routine consists of a single instruction and is entered from outside the static flow, but its specific operation is not detailed in the available data.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `immediate_return_routine`: The routine performs no operations and returns immediately to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_single_instruction`: The routine consists of one instruction but the decompiler timed out.
- `nop_operation`: The routine is empty and contains a single instruction that performs no operation.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately_9`: This routine is empty and performs no operations before returning to the caller.
- `return_immediately_10`: This routine is empty and performs no operations before returning.
- `return_from_subroutine`: The routine is empty and only contains a return instruction.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning to its caller.
- `return_no_operation`: The routine consists of a single instruction that performs an immediate return and does nothing else: it reads no registers or hardware, makes no library calls, and modifies no state.
- `return_from_subroutine_3`: This routine is an empty subroutine that returns immediately.
