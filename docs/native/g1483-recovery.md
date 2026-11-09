# Game 1483 (defender (1994)(ratsoft)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01483). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 20 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 465 of 465 named (51 author symbols), 465 of 465 described (70 by rule, 394 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1508 of 1508 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 11812 of 11812 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 271552 |
| code | 43786 |
| data | 105026 |
| padding | 5108 |
| unused | 475648 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Defender (1994)(Ratsoft)[a].adf:Defender | 6 | 5016 | 52 |
| More | 19 | 2290 | 76 |
| Assign | 2 | 117 | 2 |
| EndCLI | 2 | 117 | 2 |
| Echo | 2 | 117 | 2 |
| LoadWB | 12 | 537 | 30 |
| Ram-Handler | 2 | 10 | 1 |
| ramdrive.device | 3 | 345 | 17 |
| mathtrans.library | 18 | 830 | 97 |
| icon.library | 10 | 1375 | 109 |
| diskfont.library | 15 | 1040 | 76 |
| Defender (1994)(Ratsoft)[a].adf:bootblock | 1 | 18 | 1 |

## Cracktros (kept, not protection)

- none

## Copy protection found

- `execute_complex_logic` (More): A large routine that performs a variety of tasks including parsing signed decimal strings, calculating 32-bit products, and recursively processing structures or message-like data. Kind: sabotage path/unresolved pointer. Pass path: the pointer unaff_A3 must be valid/correctly initialized to avoid a crash or jump to the correct code path.

## Functions


### Defender (1994)(Ratsoft)[a].adf:Defender

- `initialize_game_loop`: Clears a global flag, calls a routine via a pointer, and checks a field in the returned pointer before conditionally calling two other table routines.
- `returnMsg`: Small routine of 1 instruction that merges 0 into d0.
- `start`: Opens a file, decrypts a data table using XOR, and enters a main execution loop that handles game rendering, audio, and input.
- `titles_etc`: Initializes or resets several global game variables, including resetting certain lists to zero, adjusting wave timers, and resetting ship or level-related parameters.
- `p7_j4`: The routine iterates over a table of bytes, extracts nibbles and accumulates them into two global variables, calling a delay routine when the accumulator is non-zero, and updates audio channel pointers.
- `textype`: Reads a 16-bit index table from a global text table until a sentinel value, copying 8-byte font patterns to bitplane buffers.
- `audio`: This routine manages a software sound queue for the Amiga audio hardware.
- `audio_j7`: Iterates backward through a sound-data table to decrement or terminate active sound-effect counters.
- `next_wave`: Decrements a global timer and, if it expires, reads a byte from a level table using a position index to initialize an enemy list entry with a type and active flag, setting or clearing certain enemy fields and updating the level position.
- `drawbobs`: The routine walks a linked list of bob descriptors stored in a global table.
- `drawscan`: The routine waits for a hardware flag, then writes several control words and pointers into a blitter control block to configure a scanline copy.
- `drawicons`: Iterates over a global enemy list and, for each active entry, computes a screen offset from map coordinates, selects a byte from a scanbit table and from four icon tables, shifts it and ORs it into bitplane buffers and sets corresponding collision bytes.
- `drawmap`: Calculates a scroll mask from a global horizontal position and stores it in a global vertical scroll variable.
- `your_shots`: Iterates through a table of laser objects to either spawn new lasers if a certain controller button is pressed or update the animation, position, and display properties of existing lasers.
- `you`: Updates a global ship position, velocity, and thrust variables based on control flags, clamps values, computes a map offset and ship image index, writes thrust to a hardware audio register, stores the ship y coordinate, and writes three words through a pointer derived from an output pointer, using a table lookup for the image.
- `control1`: Reads a 16-bit value from a pointer-offset, updates a facing direction, and sets control flags based on the new direction, direction changes, and any change in the firing button bit detected from the CIAA_PRA register or a global variable.
- `control2`: Tests a keypress table to update a control byte, toggle a facing flag, and set firing or smart flags.
- `rez`: Updates a per-entry counter and flags in a table or object list, conditionally triggering a drawing routine and updating a global enemy slot or related fields, while manipulating a hardware register or sound priority.
- `drawbits`: The routine iterates over a small set of rows and, for each row, computes a screen offset and writes masked bits from a source table into three bit-plane buffers, OR-ing the bits into two consecutive words per row.
- `stars`: Iterates through a table of star data to update or reset star coordinates based on a scrolling offset.
- `addlife`: Waits for a hardware flag to clear, then if a lives counter is below a limit, it programs several blitter registers and sets source and destination pointers using a global variable and a table indexed by a certain value and the lives counter.
- `addsmart`: Checks if a certain bit in a global variable is set, and if a certain value is below a threshold, updates several hardware-related or memory-based registers or pointers with a new calculated range or offset.
- `takelife`: If a lives counter is between one and five, it waits for a hardware flag to clear, programs certain blitter registers, and sets a destination pointer using a table indexed by a certain value and the lives or lives-related counter.
- `takesmart`: Checks if a certain value is between certain bounds, and if so, updates several hardware-related or memory-based registers or pointers with a new calculated range or offset.
- `score_draw`: Compares a global variable against itself and checks a score variable against several thresholds, calling another routine if certain conditions or certain value comparisons are met.
- `dwarem`: Repeats a copy operation eight times, advancing the source index by two bytes each iteration and writing to a destination offset.
- `new_vblank`: Reads the interrupt enable and request registers, masks the result, and conditionally calls two different interrupt service routines if certain interrupt bits are set.
- `vblint`: Writes a constant value to a global variable.
- `copint`: Updates animation counters, scroll state, and game logic or timers, and calls subroutines to draw icons, handle enemies, and manage game state or screen changes.
- `keyboardint`: Checks a bit in a global variable, and if set, clears it and updates a global variable and a table entry based on a bit-shifted value from that variable.
- `key_j1`: Sets a certain global variable to zero, clears a memory location, and then waits for a hardware flag to set a certain global variable to a specific value.
- `enemy`: Iterates through a list of enemy objects, updating their positions and checking for certain conditions to trigger level or life updates.
- `scores`: Performs BCD-adjusted additions to a global variable and updates a spare variable if a certain bitmask does not match a certain value.
- `set_bomb`: Checks several conditions involving a global variable and a counter, then if met, writes a value to a table, modifies a short field, updates a second table entry, and sets a value in a third table entry before resetting a global variable.
- `bombs`: Iterates through a list of active bombs to decrement their lifespans, checks for collisions with the ship's position, and draws or removes bomb sprites by or-ing bitmask patterns into plane buffers.
- `set_shot`: This routine conditionally writes to a shot list entry and associated fields using several global variables and inputs.
- `shots`: Iterates through a list of active projectiles to update their horizontal or vertical positions and check for screen boundary or ship collisions.
- `change_screens`: Toggles a screen flag, updates several plane base addresses and plane pointers, saves current plane pointers into backup variables, and calls a routine to clear the screen.
- `clear_screen`: The routine waits until a bit in a global variable clears, then programs the blitter registers to copy a full-screen area from a source plane to the destination, using a 32-bit depth and a specific blitter control word.
- `initialise`: Saves the current processor state and configures the system interrupts, video blanking, copper, and screen settings.
- `setscreen`: The routine writes constant values to a set of global variables that hold palette and tile palette entries.
- `newcopper`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `change_vblank`: Stores a new routine address into a global variable and writes to the interrupt disable register to enable interrupts.
- `setints`: Stores a pointer to another routine in a global variable, writes values to the CIAA control and timer registers, and masks a byte in a global variable.
- `close_down`: Calls one routine to shut down a system and then calls another routine to revive a system.
- `save_registers`: Opens the graphics library, retrieves a view pointer, and saves the current view pointer, copper list pointer, and several hardware register or interrupt values into global variables.
- `kill_system`: The routine stores a constant value into a global variable and into another memory location, then returns.
- `revive_system`: The routine stores several saved values into global variables, writes a saved DMA control value and a saved interrupt enable value to hardware registers, then waits until a bit in a global variable is set before calling a routine via a pointer and calling another routine.
- `closegraphics`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `opengraphics`: The routine repeatedly calls a function pointer stored in a global variable until the returned value is non-zero, storing the result in a global variable.
- `opendos`: Repeatedly calls a function pointer located at a fixed offset from a global variable until a non-zero value is returned, storing the resulting library base in a global variable.
- `closedos`: The routine calls a function pointer stored in a global variable and then returns.

### More

- `initialize_program_environment`: Initializes the program's execution environment by opening libraries, allocating memory, and parsing a file header or string-based configuration.
- `restore_patch_state`: The routine restores a saved patch state by writing the incoming pointer into a saved slot and then invoking a series of cleanup callbacks stored in a global table.
- `parse_file_header`: Parses a file header to populate a structure with file metadata, including filename, size, and flags, while handling error conditions such as end-of-file or invalid file types.
- `iterate_table_with_function`: Iterates through a table of four values, passing each or a pointer to a global array and a value from a structure member to another routine.
- `validate_input_buffer`: Calls another routine twice with arguments derived from an input pointer and then scans a range of bytes to check if they fall within certain allowed ranges.
- `process_string_token_data`: This routine parses a character or byte-based token or string, checking for specific delimiter or control characters.
- `execute_complex_logic`: A large routine that performs a variety of tasks including parsing signed decimal strings, calculating 32-bit products, and recursively processing structures or message-like data.
- `initialize_entity_state`: Initializes fields in a structure and invokes another routine with values or pointers from that structure, conditionally calling further routines or looping based on certain structure fields.
- `update_entity_and_increment_counter`: The routine calls another routine twice with arguments derived from the input pointer, stores the second result into a field of the pointed-to structure, and if a flag field is zero it increments a short located at an offset from a pointer stored in the structure.
- `parse_buffer_content`: Uses another routine to check a buffer, updates a size field, and scans for a newline or form-feed character while potentially modifying a flag or string content.
- `process_entity_list`: Determines a loop limit based on a field in a passed structure and certain flags, then iterates a limited number of times to call another routine with a pointer to a table.
- `handle_search_loop`: Uses a flag in an argument block to select a global variable and repeatedly calls another routine, updating a counter and looping while a search result is zero.
- `initialize_handler`: Runs a sequence of calls to another routine using arguments from a parameter block and a global variable, returning the result of one of those calls.
- `check_flag_and_call`: Calls another routine with two arguments, then checks a short value at an offset from a pointer and conditionally calls another routine with one argument.
- `draw_scroll_window_bounds`: The routine calls a drawing routine using a pointer and a certain value, then calls another routine using a value or offset from a pointer-based structure.
- `draw_text_window_range`: The routine calls a drawing routine with a fixed or zeroed argument, then calls another routine with a decremented value derived from a pointer-based structure.
- `draw_text_line`: Uses a handler pointer to call drawing primitives while iterating over a text buffer, tracking line count and stopping if a certain character or flag is encountered.
- `dispatch_recursive_pointer`: The routine calls another routine with a single argument, then calls another routine using a pointer from a structure and a global variable.
- `set_flag_and_forward`: Checks a field in a passed structure, optionally sets a flag in a remote structure, and calls another routine using a global variable.
- `dispatch_by_flag`: Selects two pointers based on a flag in a parameter block and calls another routine using either a global variable or a table entry.
- `dispatch_handler_by_flag`: Reads a flag from a parameter block and calls another routine with either three references to a global variable or a value from a table entry.
- `clamp_negative_result`: The routine calls another routine twice with the same first argument and different third arguments, then clamps a negative return value to zero before returning it.
- `consume_from_buffer`: Checks a size field in a structure and, if a certain condition or partial way is met, updates several offset or remaining size fields and calls another routine with a pointer derived from the structure.
- `reset_state_and_move_offset`: The routine resets a set of fields in a structure and then calls another routine with a pointer into the structure and a size derived from a field, adding the returned value to a field when positive.
- `update_scroll_window`: Updates a scrolling window structure by calculating a target position using provided offsets and checking it against current window bounds.
- `read_line_from_buffer`: Repeatedly calls another routine to read bytes from a source buffer into a destination buffer, stopping at a newline or a fixed limit, and appends a null terminator.
- `copy_null_terminated_string`: Copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `convert_string_to_lowercase`: Iterates through a null-terminated string and converts any lowercase ASCII letters to uppercase by subtracting a constant.
- `parse_signed_decimal`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `call_library_function_with_mask`: Checks if a pointer in a structure or table is non-null, then calls a library function using a base address from a global variable and a provided mask.
- `process_message`: Recursively processes a structure by invoking another routine on certain fields and string arguments, then conditionally updates structure fields and invokes further routines.
- `build_message_buffer`: The routine obtains a pointer from another routine and then calls another routine with that pointer, a constant, a stack-allocated buffer containing the input parameter, and a size of one.
- `copy_entity_data`: The routine obtains a handle from another routine and, if valid, reads a value from that handle, passes it to another routine together with a small stack buffer, and copies two fields from thehandle into the caller-provided structure.
- `initialize_object_from_template`: Obtains a template identifier, allocates an object, links its fields to a template and source block, and copies words from a source address into the new object.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_alloc_abs`: Library stub: loads its arguments and the exec library base, calls AllocAbs and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
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
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `recurse_with_stack_slot`: Calls another routine while passing two parameters and a pointer to a specific location on the stack.
- `extract_byte_from_call`: Calls another routine with a pointer to a small stack buffer and a constant, returning a byte from that buffer if the call returns certain a value, otherwise returning negative one.
- `check_string_checksum`: Computes a rolling 32-bit checksum over a stack buffer until a null byte is found and calls another routine with the checksum or a bitwise-not version if non-zero.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `modular_inverse_32bit`: Computes a 32-bit modular inverse or performs a 16-bit modulo reduction depending on the size of a register value.
- `multiply_32_bit_integers`: Computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `recursive_call`: The routine calls itself and then returns.
- `negate_on_flag`: calls another routine and returns its first return value, or the negation of that value if the second return value is non-zero.
- `invoke_table_entry`: Invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_to_address`: Stores a single byte from a register into the memory location pointed to by another register.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `allocate_message_entry`: Attempts to allocate a message buffer, and if successful, populates several fields of that buffer with parameters and a value from another routine before dispatching it.
- `clear_object_state`: If a field at a certain offset is non-zero, the routine calls itself recursively.
- `compare_strings_with_wrap`: Compares two null-terminated strings byte by byte, skipping the check if the first string contains a newline character.
- `setup_self_referential_pointer`: The routine writes the address of its pointer argument into the memory location pointed to by the argument, then adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.

### Assign

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_table_data`: If a pointer is non-null, it copies a fixed number of dwords from a source table to a destination buffer before calling another routine via a pointer.

### EndCLI

- `initialize_global_table_and_data`: Calculates a maximum value from a linked list, populates a table with an incrementing sequence, and copies a fixed block of data from a source table to a destination buffer before updating a function pointer.
- `copy_and_call_routine`: copies a block of sixteen orf-word-sized or double-word-sized values from a memory offset into a stack-based or global buffer, then executes a routine via a pointer.

### Echo

- `initialize_table_and_copy_data`: Calculates the maximum value in a linked list of integers, populates a table with an incrementing sequence, and copies a fixed-size block of data from a source table to a destination buffer while updating a function pointer.
- `copy_table_and_execute_routine`: Copies a fixed-size block of 16 words from a table or structure into the stack and then calls a routine via a pointer.

### LoadWB

- `initialize_loader_environment`: Parses a command-line-style string or file header to initialize internal pointers, allocating memory for string or metadata storage and setting up task or library references.
- `apply_patch_and_cleanup_callbacks`: Saves a new pointer into a slot within a structure and executes a sequence of cleanup callback functions or conditional handlers stored in a global table or linked structure.
- `load_workbench_icon`: This routine processes a global variable by calling several other routines, performing a double XOR decode on a zero-terminated byte string via a pointer, then stores a result from another routine into a buffer obtained earlier.
- `compare_strings`: The routine compares two null-terminated strings byte by byte.
- `build_linked_list_from_table`: Iterates through a table of pointers, allocating a new node for each entry and linking them into a new list.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `invoke_indirect_handler_2`: The routine calls a handler obtained by indexing a global table with a fixed negative offset.
- `call_indirect_table_entry_2`: The routine calls a function pointer taken from a global table.
- `call_indirect_table_entry_3`: The routine calls a function pointer taken from a global table using a fixed offset.
- `call_indirect_from_table`: The routine calls another routine through a pointer obtained by indexing a global variable.
- `call_indirect_table_entry_4`: The routine calls a function pointer taken from a global variable table and then returns.
- `call_indirect_table_entry_5`: The routine calls another routine through an indirect pointer obtained by adding an offset to a global variable.
- `call_indirect_table_entry_6`: The routine calls another routine through a pointer obtained by indexing a global variable table with a fixed negative offset.
- `call_indirect_table_entry_7`: The routine calls a function pointer retrieved from a global table via a negative offset.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_indirect_table_entry_8`: The routine calls a function pointer obtained by indexing a global variable with a large negative offset.
- `invoke_handler_via_pointer_table`: It calls a routine obtained by adding a constant offset to a global variable and indexing into a table of function pointers.
- `compare_string_prefixes`: Library code compares a string against a global string and writes a fixed prefix into a memory location if a null terminator is encountered.
- `write_string_prefix`: This routine writes a fixed sequence of bytes, starting with a dot, into a memory location relative to a register.
- `compare_string_bytes`: This routine compares a null-terminated string against another source, stopping if a mismatch or a null terminator is encountered.
- `empty_routine_5`: The routine contains four instructions and performs no observable operation; it ends with a return.
- `invoke_reschedule_handler`: This routine calls a handler obtained by indexing a global table with a fixed negative offset.
- `invoke_dispatch_handler`: This routine calls a handler obtained by indexing a global table with a fixed negative offset.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `invoke_indirect_alloc_handler`: This routine calls a handler obtained by indexing a global table with a fixed negative offset.
- `call_other_routine_with_stack`: The routine calls another routine, passing the two incoming parameters and a pointer to a local stack slot.
- `check_string_checksum_2`: The routine computes a rolling 32-bit checksum over a 140-byte stack buffer by iterating until a null byte is found, then calls another routine with the original parameter and the buffer and the bitwise-not of thechecksum if the checksum is non-zero.
- `store_byte_to_address_2`: This routine stores a byte from a register into a memory location pointed to by another register.

### Ram-Handler

- `execute_indirect_function_2`: loads a function pointer from a global variable offset and executes it, followed by an indirect jump to an unresolved destination.

### ramdrive.device

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `initialize_ramdrive_buffer_pool_2`: Opens a library, calculates required buffer sizes, and copies a stub routine into a new memory block.
- `initialize_ramdrive_buffer_pool`: Calculates buffer sizes, copies a stub routine into a new buffer, and populates a linked list of available memory blocks.
- `initialize_ramdrive_device_buffers`: Initializes a device structure by calculating buffer sizes, copying an empty stub routine into a new buffer, and populating a linked list of device buffers.
- `initialize_ramdrive_device`: Initializes a device structure by storing pointers to internal buffers and control fields, and links it into a global list.
- `execute_two_indirect_calls`: The routine calls a function pointer obtained by using an offset from a register or a global variable twice.
- `traverse_linked_list`: Library code traverses a linked list of structures, checking a certain bit in a member, and updates a global variable and an interrupt disable/enable register.
- `decrement_global_counter`: Decrements a byte in a global variable and updates a global pointer if the value wraps or reaches a certain threshold.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decrement_global_counter_2`: This library routine decrements a byte in a global structure and updates a global pointer to a specific address if the value wraps or reaches zero.
- `store_register_to_pointer`: Stores a value from a register into a memory location pointed to by a register plus an offset.
- `process_device_message`: Executes a function pointer from a global structure, then looks up a value in a table based on a field in a message structure, stores the result back into the message, and conditionally calls a cleanup routine.
- `return_constant_value_2`: Loads a constant value into a register and returns.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 253 into d0, then returns.
- `update_string_with_colon`: This library routine increments a byte in a structure, manages a linked list, and replaces a null terminator in a string with a colon before calling a library function.
- `terminate_string_with_colon_2`: Traverses a linked list or pointer chain, replaces a null terminator in a string with a colon, and executes a function pointer while storing a return value into a global location.
- `terminate_string_with_colon`: Iterates through a string until a null terminator is found, replaces it with a colon, executes a function pointer, and stores a return value in a global structure.

### mathtrans.library

- `unimplemented_library_instruction`: This library routine contains an unimplemented instruction and stops execution.
- `store_register_to_global`: Stores a register value into a global variable.
- `update_linked_list_node`: If a certain flag in a structure is zero, this routine replaces a node in a linked list with a new value and updates a pointer, then calls another routine via a function pointer.
- `set_bit_in_memory`: Sets the third bit of a byte located at a memory offset.
- `increment_reference_count`: As library code, it increments a value at an offset in a structure and, if the value reaches one, updates a global pointer and calls another routine.
- `return_all_ones`: Returns a value where all bits are set.
- `decrement_counter_and_call`: The library code decrements a short counter stored in a structure and, if the counter reaches zero and a specific flag bit is set, calls another routine.
- `apply_bitmask_to_bitset`: Applies a bitwise mask to a single bit or a group of bits within a bitset or bitfield.
- `update_bitfield_loop`: As library code, it executes a loop that modifies a bitset or bitfield based on a calculated index and bitmasking operations.
- `adjust_bitfield_value`: Takes two input values and a global variable and repeatedly modifies the first value by setting a bit selected by the second value, then adjusts a running difference using a doubling accumulator and a shifting value, and returns a combined result of the modified first value and the high word of the second value when a termination condition on the difference is met.
- `mask_library_result`: As library code, it calls a function via a global pointer and returns the result with certain lower bits masked and a constant bitset.
- `call_jump_table_entries`: Calls jump table entries in an alternating pattern, then calls a third entry once, and calls the second entry once more, using an address derived from a register.
- `execute_complex_callbacks_2`: Library code that executes a sequence of function pointers or conditional callbacks, potentially calling another routine recursively.
- `execute_complex_callbacks`: Library code executes a sequence of function pointers or conditional callbacks, potentially calling another routine recursively and returning a masked value.
- `execute_conditional_callbacks_2`: Library code calls a sequence of function pointers via a structure, with certain calls executed conditionally based on a register or a certain value.
- `execute_conditional_callbacks`: Library code executes a sequence of functions via pointers stored in a structure, with conditional execution based on a register value, and returns a masked result.
- `mask_return_value_2`: The library code calls a routine via a global pointer, potentially calls another routine or a second instance of the first, and returns a value with certain lower bits masked.
- `mask_function_return`: As library code, it calls a function via a global pointer and returns the result with certain lower bits masked.
- `execute_twice_and_mask`: As library code, it calls a function via a global pointer twice and returns a masked version of a third routine's return value.
- `mask_return_value`: Calls a function through a pointer stored in a global variable twice, then calls another routine and returns its result with certain bits cleared.
- `call_indirect_via_global`: The library code calls a routine via a pointer retrieved from a global variable.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `call_indirect_via_global_offset`: The library code calls a routine via a pointer retrieved from a global variable plus a negative offset.
- `reconstruct_value_via_bit_shifting`: This routine reconstructs a large integer by iteratively checking bit-shifted values against a target value and accumulating them into a global temporary variable.
- `decompose_value_with_shifts`: Library code performs a loop that shifts values and decrements a counter, potentially calling other routines recursively.
- `execute_loop_with_branching`: The library code executes a loop that performs bit-shifting and decrementing, potentially calling other routines if a counter reaches certain a value.
- `apply_math_transformation`: As library code, this routine performs bit-shifting and arithmetic on a value, then iterates through a table to add and subtract values from a global variable.
- `apply_math_transformation_alt`: As library code, this routine performs bit-shifting and arithmetic on a value, then iterates through atable to add and subtract values from a global variable.
- `update_math_table`: As library code, this routine calculates a value based on a register, then iterates through a table to subtract or add values to a running total until a condition or a certain number of iterations is met.
- `process_math_table`: A library routine that performs complex conditional bit-shifting and arithmetic or iterates through a pointer to add and subtract values from a register.
- `process_math_table_2`: This library code performs complex conditional bit-shifting and arithmetic or iterates through a pointer to add and subtract values from a global variable.
- `update_registers_from_table`: Library code that iterates through a table, adding or subtracting values from a register while updating a pointer.
- `process_pointer_table`: Library code that iterates through a table of integers, performing arithmetic and bit-shifts on registers while updating a pointer.
- `execute_conditional_loop_2`: Library code that executes a loop performing arithmetic and bit-shifts, calling other routines and potentially calling another routine recursively.
- `execute_conditional_loop`: Library code executes a loop that performs arithmetic and bit-shifts, calling other routines and potentially calling another routine recursively.
- `return_void`: The routine is empty and performs no operations before returning.
- `recursive_call_wrapper`: The routine contains no instructions and immediately returns.
- `execute_recursive_calls`: The library routine calls another routine twice and then returns.
- `execute_conditional_arithmetic`: The library code performs conditional checks on a value and executes a loop involving bit-rotation and arithmetic operations.
- `execute_handler_conditional`: A library routine that checks a bitmask and conditionally executes a handler or performs bit-rotation and arithmetic.
- `return_without_effect`: Library code calls a routine and then calls it again if a certain condition is met.
- `call_other_routine_conditionally`: Library code invokes another routine and then, based on a boolean flag that remains false, invokes three additional routines in sequence.
- `execute_conditional_branching`: Library code checks a byte and, if non-zero, executes a sequence of conditional calls to other routines.
- `return_from_subroutine`: this library routine is empty and performs no operations before returning.
- `return_immediately`: the routine is empty and performs no operations before returning.
- `empty_stub_3`: This library routine is empty and performs no observable work.
- `empty_stub_2`: The library code contains five instructions and performs no observable work; it ends immediately.
- `empty_routine_2`: The library code consists of four 68000 instructions and performs no observable work, immediately returning to its caller.
- `execute_sequence_of_calls_2`: Library code executes a fixed sequence of calls to another routine or a global pointer, sometimes involving bitwise or string-related logic.
- `execute_sequence_of_calls`: Library code executes a fixed sequence of four calls to another routine.
- `update_accumulator_with_table`: The library code iterates through a table, adding or subtracting table entries to or from a register until a counter reaches zero.
- `subtract_table_from_accumulator`: The routine walks through a table pointed to by a pointer and repeatedly adds the current table entry to an accumulator, then subtracts subsequent entries while a counter is decremented.
- `calculate_complex_shifts_2`: Library code executes a loop involving bit-shifting and conditional branches, followed by a loop that doubles a value and checks for a certain condition.
- `return_empty_routine_2`: This library routine is empty and performs no operations.
- `calculate_complex_shifts`: Library code executes a complex loop involving bit-shifting and conditional branches, followed by a loop that doubles a value and checks for an overflow or sign change.
- `complex_loop_with_branching`: The library code executes a loop involving bit-shifting and conditional jumps, potentially calling other routines and performing arithmetic on a value.
- `call_other_routine_thrice`: The library code calls another routine three times in succession and then returns.
- `extract_byte_and_branch_2`: Library code that calls routines and performs arithmetic or bitwise operations on a value to determine which further routines to call.
- `extract_byte_and_branch`: Library code calls routines and performs arithmetic or bitwise operations on a value to determine which further routines to call.
- `empty_library_stub`: The library code contains no instructions and performs no operation.
- `empty_routine_6`: The library routine is empty.
- `return_from_routine_2`: this routine is empty and performs no operations before returning.
- `perform_math_trans_call`: The routine executes six instructions to invoke an empty library routine via a data pointer.
- `jump_to_stub`: The routine executes a sequence of instructions to transfer control to another routine, acting as an indirect branch or wrapper.
- `return_immediately_2`: The routine executes a sequence of instructions that effectively results in an immediate return without performing any data manipulation or calling other functions.
- `return_immediately_3`: The routine contains no instructions and returns immediately.
- `invoke_callback_and_return`: The library code receives a pointer in A0 and a value in D0, calls the routine pointed to by A0, and returns the value originally in D0 unchanged.
- `call_indirect_return_d0`: The library code receives a function pointer in A0 and a value in D0, calls the routine pointed to or a, and then returns the value that was in D0 on entry.
- `return_original_value`: The library code returns a value originally held in a register.
- `execute_indirect_callback`: Library code that calls a routine pointed to by a register and returns the original value held in a data register.
- `invoke_callback_and_return_5`: Library code that receives a pointer in a register and a value in another, calls the routine pointed to by the pointer, and returns the original value.
- `invoke_callback_and_return_7`: This library code calls another routine and returns the original value held in a register.
- `return_passed_value`: As library code, this routine calls another routine and returns the value originally held in a register.
- `invoke_callback_and_return_8`: Library code calls a routine pointed to by a register and returns the original value contained in that register.
- `invoke_callback_to_self`: The routine calls another routine via a pointer provided in a register and returns the original value.
- `store_register_via_pointer`: The routine stores a 32-bit value from a register into memory addressed by a pointer passed in a register, then returns.
- `invoke_callback_and_return_6`: Library code that calls a routine via a pointer and then returns the original value passed in a register.
- `invoke_callback_and_return_2`: As library code, it executes a routine pointed to by a register and returns the original value that was in that register.
- `execute_conditional_branching_2`: Library code that calls a routine via a pointer and, if a byte is non-zero, executes a sequence of conditional calls to other routines.
- `invoke_callback_and_return_4`: Library code calls a routine via a pointer in a register and returns the original value contained in a register.
- `invoke_callback_and_return_3`: Library code calls a routine via a pointer in A0 and returns the original value in D0.
- `invoke_subroutine_and_return`: Calls another routine and returns the original value held in a register.
- `call_indirected_function`: Library code that executes a routine via a pointer held in a register before returning the previously held value in a data register.
- `pass_register_to_routine`: Loads a value from a register and passes it as an argument to another routine before returning it unchanged.
- `invoke_empty_routine`: Library code calls an empty routine and returns a value in a register.
- `call_indirect_with_offset_2`: Library code calls a routine at an address calculated by adding an offset in A0 to a pointer stored in a global variable.
- `call_indirect_with_offset_alternative`: Library code calls a routine at an address calculated using an offset and a global pointer.
- `invoke_callback_and_return_concat_2`: Library code calls a routine via a global pointer and returns the concatenation of two input values.
- `invoke_callback_and_return_concat_alt`: Library code executes a routine via a global pointer and returns a concatenated result.
- `call_indirect_with_offset_alt`: Library code calls a routine using a pointer from a global variable plus an offset in A0.
- `call_indirect_with_offset_alt_2`: Library code performs an indirect call to a routine using an offset and a global pointer.
- `invoke_callback_and_return_concat_alt_2`: Library code calls a routine via a global pointer and returns a concatenated 64-bit value.
- `invoke_callback_and_return_concat_alt_2_2`: Library code invokes a routine through a global pointer and returns the concatenation of two registers.
- `invoke_callback_and_return_concat_alt_3`: Library code calls a routine via a global pointer and returns a concatenated result.
- `invoke_callback_and_return_concat`: The library code invokes another routine via a pointer stored in a global variable and then returns the concatenation of the two input values.
- `call_indirect_with_offset`: The library code calls a routine at an address calculated by adding an offset in A0 to a pointer stored in a global variable, then returns the value in D0.
- `normalize_fixed_point_value`: The library code normalizes a signed 32-bit fixed-point value in D6 by swapping its high and low words, adjusting a shift count based on sign and magnitude, then left-shifting the value while decrementing the count until the count reaches zero or the value overflows, returning early on overflow.

### icon.library

- `return_all_ones_2`: Library code that returns a value consisting of all bits set.
- `increment_counter_and_clear_flag`: The routine increments a 16-bit counter stored at an offset from a pointer in A6 and clears a specific bit in a byte at another offset from the same pointer.
- `decrement_counter_and_call_2`: The routine decrements a short counter at an offset from a pointer in A6 and stores the result back.
- `modify_library_list_node`: This library routine checks a flag in a structure, and if it is zero, it moves a node or a value to a new location in a linked list before calling a handler.
- `set_bit_in_byte`: Library code that sets a specific bit in a byte at an offset from a pointer in A6 and returns zero.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `update_global_pointers`: The library routine copies two pointers into a structure and then calls another routine, storing its return value into a third pointer location or updating a global pointer if the return value is zero.
- `copy_two_pointers_from_table`: This library routine copies two 32-bit values from an address derived from a register into two global variables.
- `return_from_routine`: This routine is empty and performs no operations before returning.
- `return_nothing`: Library code that performs no operations and returns.
- `call_transform_routine`: This library routine calls a routine that performs bit-shifting and arithmetic via a table lookup and then returns.
- `call_icon_copy_routine`: This library routine unconditionally calls a routine that copies icon data to the stack and then returns.
- `call_other_routine_7`: The library routine calls another routine once and then returns.
- `recursive_call_stub`: The routine unconditionally calls another routine and then returns.
- `call_other_routine_2`: The routine calls another routine and then returns.
- `call_other_routine_3`: The library routine calls another routine and then returns.
- `call_other_routine_6`: The routine calls another routine and then returns.
- `call_other_routine_4`: The library routine calls another routine once and then returns.
- `call_other_routine_8`: As library code, the routine calls another routine and then returns.
- `recurse_with_incremented_argument`: This library routine performs an incremental recursive call, where it calls another routine using the same first argument and an incremented second argument before returning.
- `call_other_routine_5`: The routine calls another routine and then returns.
- `call_other_routine_four`: The library routine calls another routine and then returns, and the routine contains four instructions.
- `call_parse_numeric_string`: This library routine calls a routine that parses a numeric string from a buffer and then returns.
- `update_data_field`: Library code that calls another routine and stores a 32-bit value into a location at an offset from the returned pointer.
- `execute_conditional_logic`: Library code that calls another routine and performs conditional calls to a second routine or a third routine based on the returned value.
- `update_icon_node_links_2`: This library code calls another routine, and if the return value is non-zero, it copies a block of data from a stack buffer to a structure and updates a pointer-based field in that structure.
- `copy_and_link_structure_data`: This library routine copies a block of data from a source pointer to a destination pointer and then updates a pointer in a structure to point back to a previously known address.
- `return_zero_stub_3`: The routine is an empty stub that immediately returns the constant zero.
- `copy_icon_data_to_stack`: The routine copies ten longwords from an offset within a structure pointed to by the third argument into a local array on the stack, then copies several additional fields from the same structure into other stack slots and finally calls another routine with the first two arguments and a pointer to a stack-resident header.
- `update_icon_node_links`: Recursively updates the pointer-based fields or linked list structures within an icon node or a structure representing an icon.
- `recursively_update_icon_node_links`: This library routine recursively traverses and updates pointer-based fields or linked list structures, or populates a destination buffer by calling a provided function pointer.
- `update_linked_list_nodes_2`: Library code traverses a linked list and updates pointer-based fields in a structure, or executes a sequence of calls if certain pointers are non-null.
- `update_list_with_callback`: The library routine executes a loop that calls a routine and stores its result, decrementing a counter, and eventually updates a global value or returns a constant based on a conditional check.
- `call_and_return_zero`: This library routine calls another routine and then returns zero.
- `empty_routine_4`: The routine contains four instructions and performs no observable work; it ends immediately with a return.
- `traverse_icon_resource_list_2`: Recursively traverses a list of icon resource pointers, checking flags and updating a counter or a value.
- `traverse_icon_resource_list`: This library routine recursively traverses a list of icon-related pointers or resource tables, checking flags and updating a counter or a value on the stack for each valid entry.
- `iterate_table_entries_2`: Library code walks a null-terminated table of pointers and calls another routine for each entry, or iterates through a list or table while checking certain conditions.
- `iterate_table_entries`: The library routine walks a null-terminated table of pointers and calls another routine for each entry or an extra pointer at a certain offset.
- `iterate_icon_table_entries`: The routine walks a null-terminated table of pointers via an index register, calling another routine for each entry and updating a counter at a fixed offset from a base pointer, then invoking a callback twice and restoring a saved value to a global variable.
- `traverse_linked_list_nodes`: This library routine traverses a linked list and calls a handler on each node, or calls a handler on the final node if it is non-null.
- `iterate_linked_list_nodes`: Traverses a linked list structure by following pointers stored at an offset from the current node.
- `increment_param_and_recurse`: The routine calls another routine with the same first argument and the second argument increased by a constant, then returns.
- `invoke_indirect_callback_2`: Library code that checks a return value and calls a function pointer located at a fixed offset from another routine or a global pointer.
- `return_null_pointer_2`: This library routine returns a null pointer.
- `return_null_pointer`: This library routine returns a null pointer.
- `update_list_element_data`: This library routine decrements a counter through a pointer and, if the counter or its previous value reaches zero, recursively calls itself.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, then uses the counter value as an index into a table pointed to by a global variable.
- `execute_conditional_handler`: Library code that executes a handler if a returned value is non-zero, performing conditional recursive or secondary calls with modified arguments.
- `call_and_return_zero_2`: The library routine calls another routine and then returns zero.
- `call_library_routine_and_return_zero`: The routine calls another routine from the library and then returns zero.
- `incremental_recursive_call`: The routine calls another routine with the same first argument and the second argument increased by a constant, then returns.
- `search_string_delimiters`: Library code traverses a list of pointers, checking for null or a specific character, and returns a pointer to a calculated address.
- `find_string_delimiter`: The library routine compares bytes in a buffer against a delimiter and returns a pointer to the position after the delimiter or a null pointer if the delimiter is not found.
- `find_string_delimiter_2`: Library code traverses a list of pointers, checking if a value is zero or if a specific character exists in a buffer, and returns a pointer to a position or null.
- `verify_string_checksum`: Library code that recursively searches for a pattern in a buffer and validates a checksum or length-related value.
- `search_string_recursively_2`: Library code that recursively or iteratively checks a string or buffer, returning a boolean or a calculated value based on a comparison.
- `search_string_recursively`: Library code that recursively or iteratively checks a string or buffer, returning a boolean or a calculated value based on a comparison or checksum check.
- `recurse_and_compare_values`: This library routine performs a recursive or iterative check involving a counter and a secondary routine, returning a boolean or a calculated difference.
- `search_string_recursively_3`: Library code that iterates through a buffer to check for a specific pattern or value.
- `parse_numeric_string_3`: Library code that parses a numeric string from a buffer and copies the resulting string or a modified version of it to a destination buffer.
- `parse_numeric_string_2`: Library code that iterates through a buffer to calculate a numeric value, using comparisons or a global variable, and calls other routines based on the result.
- `parse_numeric_string`: Library code iterates through a buffer to calculate a numeric value and calls other routines based on the result or a global variable.
- `copy_string_bytes_2`: Library code iterates through a buffer, updating a value based on a calculation, and potentially calls another routine to check a range or copy memory.
- `check_range_and_offset`: This library routine checks if a value is within a certain range and returns an offset or a negative value if it is outside that range.
- `return_all_ones_4`: This library routine returns a 32-bit value consisting entirely of set bits.
- `invoke_indirect_callback`: This library routine calls a function pointer located at a fixed offset from another library routine.
- `invoke_indirect_callback_offset`: This library routine calls a function pointer located at a different fixed offset from another library routine.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: The library routine forwards a set of four parameters to another routine, checks the returned value, and if non-zero calls the same routine again with a modified argument list.
- `set_list_element_size`: This library routine calls another routine and, if a non-zero value is returned, sets a certain offset in a structure to a fixed value and calls a handler recursively.
- `return_zero`: Library code that returns zero.
- `check_returned_value`: Library code that calls another routine and sets a return value to zero if the returned value does not match a provided parameter.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `validate_library_return_value`: This library routine calls another routine and returns zero if the returned value does not match a provided parameter, otherwise it returns the result of the called routine.
- `empty_routine_3`: Library code that is empty.
- `rotate_and_mod_value`: This library routine performs complex bitwise rotations and modulo-like arithmetic or conditional adjustments on a 32-bit or 64-bit value.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves, and subtracts a threshold if the rotated value meets or exceeds it.
- `multiply_32_bit_integers_2`: The library code computes a 32-bit product from two 32-bit register values using 16-bit halfword multiplication.
- `invoke_table_entry_2`: The routine calls a function pointer taken from an offset within a global variable, effectively invoking an entry from a table of handlers.
- `store_byte_to_pointer`: This library routine stores a single byte from a register into a memory location pointed to by a register.
- `compute_string_checksum`: The routine processes a null-terminated byte string by iterating through each character, updating a 32-bit accumulator with a decrementing 16-bit pattern, and returns the bitwise complement of the final accumulator value.
- `find_char_pointer`: Library code that iterates through a null-terminated string until a specific character or a null terminator is encountered, returning a pointer to that location.
- `return_zero_2`: Library code that returns a constant zero value.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `find_and_copy_string`: This library routine finds the end of a string using a source pointer and then copies bytes from a second source pointer to a destination pointer until a null terminator is reached.
- `compare_string_bytes_2`: This library routine compares two memory buffers byte by byte and returns zero if a mismatch or a null terminator is encountered, or if a counter reaches zero.
- `return_constant_one`: This library routine returns a constant value of one.
- `copy_string_bytes`: Library code that iterates through a source buffer and copies bytes to a destination, stopping if a null terminator or a certain count is reached.
- `copy_string_to_buffer`: Copies a string from a source pointer to a destination buffer and null-terminates the remaining bytes.
- `copy_and_pad_string`: This library routine copies a certain number of bytes from a source to a destination and then null-terminates any remaining bytes in the destination buffer.
- `return_empty_routine`: This library routine is empty and performs no operations before returning.
- `self_referential_pointer_setup`: The library code writes a modified pointer value into a destination, clears a subsequent word, and stores the original pointer value into a following word.
- `process_icon_request`: Validates a parameter and delegates processing to another routine if the condition ismet.
- `traverse_list_on_condition`: The library routine repeatedly calls another routine while traversing a linked list and checks a condition or a null pointer.
- `traverse_list_until_null`: The library routine repeatedly calls another routine and checks if the result is non-zero while traversing a linked list, returning one if a null pointer is encountered or zero otherwise.
- `return_zero_stub_2`: The routine returns a constant zero.
- `allocate_list_node_and_verify`: This library routine attempts to allocate a structure or node using a sequence of library calls, populates a field with a calculated value, and returns a status or a pointer.
- `return_minus_one_stub`: The routine returns a value where all bits are set.
- `update_linked_list_nodes`: The library routine traverses a linked list and updates pointer-based fields in a structure or returns an error code if a null pointer is encountered.
- `traverse_and_update_list`: This library routine traverses a linked list, updating certain pointer-based fields in a structure, and returns a status or a pointer if a certain condition or null pointer is met.
- `traverse_and_update_list_2`: The library routine traverses a linked list, updating certain pointer-based fields in a structure, and returns a status or a pointer if a certain condition is met.
- `return_all_ones_3`: This library routine returns a 32-bit value where all bits are set to one.

### diskfont.library

- `obtain_font_handle`: Library code that calls a routine to obtain a font handle and, if successful, stores a register value into a field of the handle structure before calling another routine.
- `check_and_set_font_handle`: As library code, this routine calls a routine to obtain a font handle and, if successful, stores a value from a register into an offset within that handle before invoking a cleanup or notification routine.
- `cleanup_library_resource`: As library code, it calls a function pointer via a library base, checks a return value, and if successful, initializes a structure with a pointer to its own next element or calls a routine to free a resource.
- `library_cleanup_routine`: As library code, it calls a function pointer relative to a global address and then calls another routine to free or close a resource.
- `untrack_font_resources`: This library code iterates through a linked list of resource structures and removes any entry where a size field is less than one.
- `release_font_resource`: Library code that checks a status or size field, potentially calls library functions to close or free memory, and updates a linked list or structure or sets a bit in a status byte.
- `set_byte_high_bit`: As part_of library code, this routine sets the most significant bit of a byte at a fixed offset from a pointer.
- `empty_routine`: This library routine is empty.
- `clear_flag_increment_counter`: As part of library code, this routine clears the most significant bit of a byte at a fixed offset from a base pointer and increments a short integer at another fixed offset.
- `decrement_and_check_refcount`: Decrements a counter or reference count in a data structure and, if the count reaches zero and a status flag is set, calls another routine.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `extract_font_resource_blocks_2`: Iterates through a linked list of font resource buffers to copy string-based font or glyph data into a target buffer.
- `append_font_string_list`: This library routine iterates through a linked list of string buffers, calculating the length of each string and copying the characters into a destination buffer.
- `copy_font_string_segment`: This routine copies a sequence of bytes from a source string or a linked list of string buffers into a destination buffer.
- `parse_font_resource_blocks`: Iterates through a linked list of font resource buffers to identify and extract valid font or glyph data blocks.
- `parse_font_resource_metadata`: This library routine parses a font resource buffer to identify valid headers and extract or register glyph or font data blocks.
- `extract_font_resource_data`: This routine scans a data buffer for a specific magic signature and, if a valid header is found, copies a block of font or glyph data into a target memory structure.
- `parse_font_resource_header`: Parses a disk-based font resource header by reading a block of data and validating a magic signature.
- `extract_font_resource_blocks`: Iterates through a font resource buffer to identify and extract valid font or glyph data blocks.
- `return_zero_stub`: This library routine returns zero and performs no operations on registers, memory, or hardware.
- `calculate_scaled_difference`: This library code calculates a scaled difference between two short integers and further adjusts this value by subtracting elements from a table based on bitwise comparisons of certain bytes.
- `subtract_table_values_iteratively_2`: As library code, this routine subtracts values from a register using a table-based lookup or bitmask-driven offset.
- `subtract_table_values_iteratively`: Subtracts values from a register using a table-based lookup or bitmask-driven offset.
- `iterative_table_subtraction`: Iteratively subtracts values from a register or a global variable using a table-based lookup or bitmask-driven offset.
- `return_constant_value`: This library routine returns a constant value without using any arguments or registers.
- `update_font_attribute_table`: Iterates through a list of font structures to find a font with a specific attribute or size, then updates a tracking table or recalculates certain font properties.
- `initialize_font_attribute_tables`: Iterates through a list of font structures to find a font with a specific attribute or size, then updates a tracking table or recalculates font metadata or counters.
- `scan_font_attribute_map`: Iterates through a list of font structures to find a specific font attribute or size, updating a tracking table with the index of the appropriate font structure.
- `update_font_size_tracking`: Updates a tracking table with the index of the font structure possessing the largest font size.
- `search_font_index_by_size`: Iterates through a list of font structures to find the entry with the largest font size, updating a tracking table with the discovered index or zeroing out specific font attributes if certain conditions are met.
- `find_largest_font_index`: Iterates through a list of font structures to identify the entry with the maximum font size, while updating a tracking table with the index and pointer of the identified font.
- `find_max_font_size`: Iterates through a list of font structures to find the one with the largest font size or specific attribute, updating a table with the discovered pointer and incrementing a counter or status flag.
- `populate_font_table`: This library routine scans a memory buffer to locate a font definition and builds a lookup table containing pointers to font glyphs.
- `load_disk_font_table`: As library code, this routine builds a table for a disk font.
- `dispatch_font_request`: As part of library code, this routine reads a pointer and an argument from the stack and passes them to another routine.
- `invoke_indirect_handler`: This library routine calls a handler obtained by indexing a global table with a fixed negative offset and invoking the resulting function pointer.
- `call_indirect_table_entry`: This library routine calls a function pointer taken from a global table.
- `execute_indirect_function`: loads a function pointer from a global table and calls it.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `call_parent_dir`: Library stub: loads its arguments and the dos library base, calls ParentDir and returns its result to the caller.
- `copy_null_terminated_string_2`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `skip_string_terminator`: This library routine increments a pointer until it reaches a null terminator, then calls another routine with the resulting pointer.
- `skip_to_string_terminator_and_call`: The routine increments a pointer until it reaches the terminating zero byte of a zero-terminated string and then calls another routine with the pointer to that terminator.
- `count_null_terminated_bytes_2`: The routine increments a counter while advancing a pointer and reading bytes until a zero byte is encountered, returning the final count.
- `count_null_terminated_bytes`: As part of library code, this routine increments a counter while advancing a pointer and reading bytes until a zero byte is encountered, returning the final count.
- `compare_string_contents_2`: As library code, this routine compares the contents of two strings, returning a constant if a mismatch or a specific byte order is found, or zero if the strings are identical.
- `compare_string_bytes_and_return_status`: The routine compares bytes in a string until a mismatch or a null terminator is found, returning a constant or a status flag.
- `compare_string_contents`: This library routine compares the contents of two strings, returning a constant if a mismatch or a specific byte order is found, or zero if a null terminator is reached.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `find_byte_in_string_2`: This library routine searches for a specific byte within a null-terminated string, returning a pointer to the match or a null pointer if the terminator is reached.
- `search_string_for_byte`: This library routine iterates through a string to find a matching byte, returning the pointer to the match or a null pointer if the terminator is reached.
- `find_byte_in_string`: The routine increments a pointer while checking if the byte at the pointer matches a specific byte, returning the pointer or a null pointer.
- `search_string_backwards_2`: Library code searches backwards from a pointer for a specific byte, returning a null pointer if the start of the memory range is reached.
- `search_backwards_for_byte`: The routine decrements a pointer while checking if the byte at the pointer matches a certain value, stopping if a certain address limit is reached.
- `search_string_backwards`: This library routine decrements a pointer while checking for a matching byte, stopping and returning a null pointer if a certain address limit is reached.
- `find_string_boundary`: This library code searches backwards from a pointer for a specific character, and if found, returns a pointer or a value incremented by one.
- `increment_returned_value`: This library routine returns the value passed in D0 incremented by one.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_unknown_handler`: This library code routine invokes another routine and returns.
- `call_external_handler`: The routine calls another unnamed routine.
- `call_other_routine`: As library code, the routine calls another routine and then returns.
- `call_another_routine`: As library code, this routine acts as a thin wrapper that immediately invokes another routine and then returns.

### Defender (1994)(Ratsoft)[a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
