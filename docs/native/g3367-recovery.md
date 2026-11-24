# Game 3367 (diablo v1.0 (1986)(extended software)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03367). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901160 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 432 of 432 named (72 author symbols), 432 of 432 described (225 by rule, 140 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1274 of 1274 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9124 of 9124 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 691 |
| code | 33058 |
| data | 36050 |
| padding | 1409 |
| unused | 829952 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Diablo v1.0 (1986)(Extended Software).adf:Diablo | 122 | 9109 | 430 |
| Diablo v1.0 (1986)(Extended Software).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |

## Cases handled

- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Diablo v1.0 (1986)(Extended Software).adf:Diablo

- `process_message`: Processes a message by checking global variables, calling routines, and freeing memory blocks, then replies to the message and returns a value.
- `process_message_and_free`: Processes a message by checking global variables, calling routines, and freeing memory blocks, then replies to the message and returns a value from a global pointer.
- `frag_lea_jsr_rts`: Short code fragment that loads the address of the memory at an offset from a4 into a0, then calls another routine, then loads the address of the memory at an offset from a4 into a0, then calls another routine, then returns.
- `frag_lea_moveq_jsr_move_beq_rts`: Short code fragment that loads the address of a fixed memory address into a1, then loads 0 into d0, then calls another routine, then copies d0 to a fixed memory address, then branches if the last result was zero, then returns.
- `initialize_timer_device`: Initializes a timer device by creating an extended I/O request, opening the device, and performing a read operation.
- `initialize_display`: Initializes screen and window structures using system libraries, sets up color mapping and raster data for graphics rendering, then returns.
- `cleanup_and_initialize`: Initializes and cleans up global resources including memory blocks, device handles, and screen/window objects.
- `parse_string_table_2`: Processes a string table by iterating through entries, parsing quoted and unquoted strings, storing pointers to parsed strings in a global table, and managing input event handling.
- `render_tile_pattern`: Renders a tile pattern at specified coordinates using graphics hardware, updates tile state information, and handles pattern blitting with color settings.
- `render_sprite`: Renders a sprite by calculating its position and calling a clipping blit function with specific dimensions and coordinates, then updates the sprite's position in memory.
- `draw_sprite_pattern`: Draws a sprite pattern using blitting operations with specific pen settings and clipping, then performs additional pattern drawing based on a conditional check.
- `render_sprite_2`: Renders a sprite by calling a blitting function with calculated coordinates and dimensions, then checks a condition and renders a second sprite if the condition is true.
- `load_tile_data_2`: Loads tile-related data from a table using coordinates from global variables, stores the data in other global variables, and sets a flag indicating whether the tile is walkable.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `frag_bra_3`: Short code fragment that jumps to another point in the code.
- `frag_bra_4`: Short code fragment that jumps to another point in the code.
- `frag_bra_5`: Short code fragment that jumps to another point in the code.
- `frag_bra_6`: Short code fragment that jumps to another point in the code.
- `frag_bra_7`: Short code fragment that jumps to another point in the code.
- `frag_bra_8`: Short code fragment that jumps to another point in the code.
- `frag_move_muls_movea_adda_bra`: Short code fragment that copies a fixed memory address to d0, then multiplies d0 by 60 (signed), then copies d0 to a0, then adds 0 to a0, then copies a fixed memory address to d0, then multiplies d0 by 6 (signed), then adds d0 to a0, then copies the memory at a0 to a1, then copies the memory at an offset from a1 to a fixed memory address, then jumps to another point in the code.
- `load_tile_coordinates`: Calculates a memory address using values from global variables, accesses data at that address, and stores a longword from the accessed structure into another global variable.
- `load_tile_data_pointer`: Computes a memory address based on coordinates from global variables, retrieves a pointer from that location, and copies a longword from the structure pointed to into a global variable.
- `load_tile_data`: Loads tile-related data from a table using coordinates from global variables, stores the data in other global variables, and sets a flag indicating whether the operation was successful.
- `initialize_explosion_data`: Sets fixed values into multiple global variables, performs several subroutine calls with specific parameters, and manages stack operations before returning.
- `check_tile_collision`: Tests collision flags at a calculated tile position using coordinates from global variables, and returns specific bitmasked values based on the result.
- `rotate_pattern_bits`: Rotates bit patterns in a table based on input parameters, updating global variables and performing boundary checks.
- `process_tile_coordinates`: Reads tile coordinate data from global variables, applies bit-wise operations to determine offset values, stores adjusted coordinates in another global variable structure, and then calls a routine with extended data.
- `parse_input_buffer`: Reads input from a buffer, parses quoted strings and unquoted tokens, stores parsed data into a table, and manages file handles based on parsing results.
- `update_animation_state_2`: Updates animation frame counters and state variables based on collision detection results, then refreshes display gadgets and processes input buffer data.
- `format_number_string`: Converts a numeric value into a decimal string representation in a local buffer, padding with leading zeros if necessary.
- `format_and_print_text`: Formats multiple numeric values into decimal strings using a shared buffer and prints them via a text output routine.
- `process_input_data_2`: Parses input data from a buffer, extracts quoted strings and unquoted tokens, stores pointers to parsed data in global variables, and manages a counter for processed items.
- `parse_text_buffer`: Processes a text buffer to extract quoted strings and numeric values, storing them in a global table while managing stack pointers and calling output routines.
- `process_tile_pattern`: Reads tile pattern data from a table using coordinates, processes the data through a lookup table, and writes pixel values to graphics hardware based on the processed data.
- `frag_bra_9`: Short code fragment that jumps to another point in the code.
- `frag_bra_10`: Short code fragment that jumps to another point in the code.
- `frag_bra_11`: Short code fragment that jumps to another point in the code.
- `frag_bra_12`: Short code fragment that jumps to another point in the code.
- `frag_bra_13`: Short code fragment that jumps to another point in the code.
- `frag_bra_14`: Short code fragment that jumps to another point in the code.
- `frag_move_muls_movea_adda_bra_2`: Short code fragment that copies the memory at an offset from a6 to d0, then multiplies d0 by 60 (signed), then copies d0 to a0, then adds 0 to a0, then copies the memory at an offset from a6 to d0, then multiplies d0 by 6 (signed), then adds d0 to a0, then copies 1676 to the memory at a0, then jumps to another point in the code.
- `frag_move_muls_movea_adda_bra_3`: Short code fragment that copies the memory at an offset from a6 to d0, then multiplies d0 by 60 (signed), then copies d0 to a0, then adds 0 to a0, then copies the memory at an offset from a6 to d0, then multiplies d0 by 6 (signed), then adds d0 to a0, then copies 1710 to the memory at a0, then jumps to another point in the code.
- `frag_move_muls_movea_adda_bra_4`: Short code fragment that copies the memory at an offset from a6 to d0, then multiplies d0 by 60 (signed), then copies d0 to a0, then adds 0 to a0, then copies the memory at an offset from a6 to d0, then multiplies d0 by 6 (signed), then adds d0 to a0, then copies 1744 to the memory at a0, then jumps to another point in the code.
- `frag_move_muls_movea_adda_bra_5`: Short code fragment that copies the memory at an offset from a6 to d0, then multiplies d0 by 60 (signed), then copies d0 to a0, then adds 0 to a0, then copies the memory at an offset from a6 to d0, then multiplies d0 by 6 (signed), then adds d0 to a0, then copies 1778 to the memory at a0, then jumps to another point in the code.
- `frag_move_muls_movea_adda_bra_6`: Short code fragment that copies the memory at an offset from a6 to d0, then multiplies d0 by 60 (signed), then copies d0 to a0, then adds 0 to a0, then copies the memory at an offset from a6 to d0, then multiplies d0 by 6 (signed), then adds d0 to a0, then copies 1608 to the memory at a0, then jumps to another point in the code.
- `render_tile_pattern_2`: Renders a tile pattern at specified coordinates using graphics hardware, updates tile state information, and handles pattern blitting with color data.
- `process_gadget_list`: Processes a list of gadget-related data by calling a handler routine with specific parameters, then performs cleanup and returns.
- `process_gadget_events`: Handles gadget events by calling a handler routine with specific parameters, processes input data through a loop, and manages global variables based on conditional checks.
- `update_ui_elements`: Updates UI element positions and states based on counter values, performs gadget management operations, and handles conditional rendering logic.
- `update_entity_position`: Updates entity coordinates and collision state based on movement vectors, clamps coordinates within bounds, and manages animation frame flags.
- `draw_filled_rectangle`: Draws a filled rectangle using graphics primitives with specified coordinates and pen settings, then performs cleanup operations on stack and global variables.
- `render_tile_pattern_3`: Processes tile rendering coordinates and performs blitting operations with clipping based on boundary checks, then updates global variables and calls additional drawing routines.
- `process_input_message`: Handles input messages from a port, processes mouse and keyboard events, and updates global variables based on message content.
- `frag_link_cmpa_bcc_jmp`: Short code fragment that creates a stack frame in a6, then compares a7 with a fixed memory address, then branches if the carry flag is clear, then jumps to another point in the code.
- `frag_move_bsr_cmpi_bne_unlk_rts`: Short code fragment that copies 2 to the memory at an offset from a6, then calls another routine, then copies d0 to the memory at an offset from a6, then compares the memory at an offset from a6 with 2, then branches if the last result was not zero, then removes the stack frame held in a6, then returns.
- `frag_link_cmpa_bcc_jmp_2`: Short code fragment that creates a stack frame in a6, then compares a7 with a fixed memory address, then branches if the carry flag is clear, then jumps to another point in the code.
- `process_input_event_2`: Reads a word from a global variable, compares it against specific values, and updates multiple global variables based on the comparison results.
- `frag_move_andi_bra`: Short code fragment that copies 1 to a fixed memory address, then copies 64 to the memory at an offset from a6, then copies a fixed memory address to d0, then masks d0 with 65407, then copies d0 to a fixed memory address, then jumps to another point in the code.
- `frag_move_andi_bra_2`: Short code fragment that copies a fixed memory address to d0, then masks d0 with 65407, then copies d0 to a fixed memory address, then jumps to another point in the code.
- `initialize_game_state`: Clears a word-sized counter at a specific memory location, reads another word from the same location into a register, tests if that value is zero and branches to a label if so, then calls a library routine with a pointer from another memory location.
- `frag_bsr_tst_beq_move_addq_unlk_rts`: Short code fragment that calls another routine, then tests a fixed memory address, then branches if the last result was zero, then copies a fixed memory address to the stack, then calls another routine, then adds 4 to a7, then removes the stack frame held in a6, then returns.
- `validate_index_range`: Checks if a value in a register is within a specific range and branches to different locations based on the comparison results.
- `frag_bra_15`: Short code fragment that jumps to another point in the code.
- `frag_bra_16`: Short code fragment that jumps to another point in the code.
- `frag_bra_17`: Short code fragment that jumps to another point in the code.
- `frag_bra_18`: Short code fragment that jumps to another point in the code.
- `frag_bra_19`: Short code fragment that jumps to another point in the code.
- `frag_bra_20`: Short code fragment that jumps to another point in the code.
- `frag_bra_21`: Short code fragment that jumps to another point in the code.
- `frag_bsr_move_andi_bra`: Short code fragment that calls another routine, then copies a fixed memory address to d0, then masks d0 with 65407, then copies d0 to a fixed memory address, then jumps to another point in the code.
- `frag_clr_jsr_addq_move_andi_bra`: Short code fragment that clears the stack, then calls another routine, then adds 4 to a7, then copies a fixed memory address to d0, then masks d0 with 65407, then copies d0 to a fixed memory address, then jumps to another point in the code.
- `frag_tst_beq_moveq_move_jsr_addq_andi_b`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then loads 2 into d0, then copies d0 to the stack, then calls another routine, then adds 4 to a7, then copies a fixed memory address to d0, then masks d0 with 65407, then copies d0 to a fixed memory address, then jumps to another point in the code.
- `update_gadget_status`: Updates gadget-related status flags based on input conditions, clears a global counter, and processes UI rendering updates.
- `frag_move_andi_bra_3`: Short code fragment that copies a fixed memory address to d0, then masks d0 with 65407, then copies d0 to a fixed memory address, then jumps to another point in the code.
- `process_input_event`: Reads a word value from a global variable, compares it against specific constants, and branches based on the comparison results to handle input events.
- `frag_move_pea_jsr_addq_cmpi_bne_clr_bra`: Short code fragment that copies a fixed memory address to the stack, then pushes the address of a fixed memory address onto the stack, then calls another routine, then adds 8 to a7, then copies a fixed memory address to d0, then compares d0 with 51, then branches if the last result was not zero, then clears a fixed memory address, then copies 1 to a fixed memory address, then jumps to another point in the code.
- `clear_bit_in_status`: Clears bit 7 of a 16-bit value in a global variable, then calls another routine, tests a word value, and branches based on the result.
- `frag_move_andi_bra_4`: Short code fragment that copies a fixed memory address to d0, then masks d0 with 128, then copies d0 to a fixed memory address, then jumps to another point in the code.
- `frag_move_btst_beq_bra`: Short code fragment that copies a fixed memory address to d0, then tests bit 7 of d0, then branches if the last result was zero, then copies 16666 to a fixed memory address, then jumps to another point in the code.
- `frag_move_clr_movea_jsr_lea_rts`: Short code fragment that copies a fixed memory address to a fixed memory address, then clears the stack, then copies a fixed memory address to the stack, then copies a fixed memory address to a0, then copies the memory at an offset from a0 to the stack, then calls another routine, then loads the address of the memory at an offset from a7 into a7, then returns.
- `process_input_event_3`: Handles input events by checking for specific key codes and calling related routines, updating global variables based on event types.
- `_CloseScreen`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `_CloseWindow`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `_DrawImage`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.
- `_EndRequest`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `_GetPrefs`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `_InitRequester`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `_ModifyIDCMP`: Library stub: loads a library base from a global variable and calls the library function at offset minus 150 of that base, returning its result to the caller.
- `_ModifyProp`: Library stub: loads a library base from a global variable and calls the library function at offset minus 156 of that base, returning its result to the caller.
- `_OffGadget`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `_OnGadget`: Library stub: loads a library base from a global variable and calls the library function at offset minus 186 of that base, returning its result to the caller.
- `_OpenScreen`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `_OpenWindow`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `_PrintIText`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `_RefreshGadgets`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `_Request`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `_ShowTitle`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `_ViewPortAddress`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `_SetPrefs`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `_AutoRequest`: Calls one function through a pointer read from a global variable at offset -348 and returns, with no other operations.
- `visit_global_table_indices`: Loops twenty times calling one routine to get an index and passing the global table address offset by that index to a second routine.
- `_AddIntServer`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `_RemIntServer`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `_AllocMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `_FreeMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `_FindTask`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `_SetSignal`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
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
- `_DoIO`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `_SendIO`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `_OpenLibrary`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `_CreateExtIO`: Allocates a cleared memory block of a given size for an I/O request, marks its type, stores its size and attaches it to a port.
- `_DeleteExtIO`: If the pointer is non-null, marks the request's type and two status fields as cleared, then frees the block using its stored size.
- `_LoadRGB4`: Library stub: loads its arguments and the exec library base, calls Deallocate and returns its result to the caller.
- `_InitRastPort`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `_SetRast`: Library stub: loads its arguments and the exec library base, calls Insert and returns its result to the caller.
- `_RectFill`: Library stub: loads its arguments and the exec library base, calls SetSignal and returns its result to the caller.
- `_BltPattern`: Library stub: loads its arguments and the exec library base, calls SetExcept and returns its result to the caller.
- `_WritePixel`: Library stub: loads its arguments and the exec library base, calls Signal and returns its result to the caller.
- `_SetAPen`: Library stub: loads its arguments and the exec library base, calls AllocTrap and returns its result to the caller.
- `_SetBPen`: Library stub: loads its arguments and the exec library base, calls FreeTrap and returns its result to the caller.
- `_SetDrMd`: Library stub: loads its arguments and the exec library base, calls AddPort and returns its result to the caller.
- `_InitBitMap`: Library stub: loads its arguments and the exec library base, calls FindPort and returns its result to the caller.
- `_AllocRaster`: Library stub: loads its arguments and the exec library base, calls RemResource and returns its result to the caller.
- `_FreeRaster`: Library stub: loads its arguments and the exec library base, calls OpenResource and returns its result to the caller.
- `_ClipBlit`: Short code fragment that saves registers on the stack, then copies the memory at an offset from a7 to a0, then copies several registers to or from memory, then copies the memory at an offset from a7 to a1, then copies several registers to or from memory, then copies a fixed memory address to a6, then calls another routine, then restores saved registers from the stack, then returns.
- `_GetColorMap`: Library stub: loads its arguments and the exec library base, calls ReleaseSemaphore and returns its result to the caller.
- `_FreeColorMap`: Library stub: loads its arguments and the exec library base, calls AttemptSemaphore and returns its result to the caller.
- `update_animation_state`: Updates animation-related counters and state variables based on conditional checks, then performs a jump table lookup using a calculated index.
- `frag_bra_22`: Short code fragment that jumps to another point in the code.
- `frag_bra_23`: Short code fragment that jumps to another point in the code.
- `frag_bra_24`: Short code fragment that jumps to another point in the code.
- `frag_bra_25`: Short code fragment that jumps to another point in the code.
- `load_and_store_word`: Loads a word value from a memory location calculated using data from global variables, stores it in another global variable, and then writes a byte value to a specific memory address.
- `calculate_and_copy_word`: Calculates an address based on values from global variables, copies a word from that calculated address to another global variable, and then stores a byte value at a specific memory location.
- `update_graphics_parameters`: Reads values from global variables, performs arithmetic operations on pointers, and writes a byte to a global variable using calculated offsets.
- `update_io_parameters`: Updates various global variables and performs IO operations based on conditional checks, including setting values in memory and calling an IO routine.
- `process_input_data`: Reads input data from a buffer, parses quoted strings and unquoted tokens, stores pointers to parsed data in global variables, and manages a counter for parsed items.
- `frag_bra_26`: Short code fragment that jumps to another point in the code.
- `frag_bra_27`: Short code fragment that jumps to another point in the code.
- `frag_bra_28`: Short code fragment that jumps to another point in the code.
- `frag_bra_29`: Short code fragment that jumps to another point in the code.
- `frag_moveq_move_bra`: Short code fragment that loads 48 into d0, then loads 3 into d1, then copies d0 to a fixed memory address (2 times in a row), then copies d1 to a fixed memory address (2 times in a row), then jumps to another point in the code.
- `frag_moveq_move_bra_2`: Short code fragment that loads 8 into d0, then copies d0 to a fixed memory address (4 times in a row), then jumps to another point in the code.
- `initialize_io_settings`: Sets specific byte values in multiple global variables to predefined constants and stores a word value from the stack to another global variable.
- `initialize_audio_system`: Allocates memory for audio structures and initializes device parameters including port creation, device opening, and I/O submission.
- `cleanup_and_parse`: Frees memory blocks referenced by a global table, closes device and port handles, and performs parsing of structured data from a buffer.
- `compare_strings_counted`: Compares up to a counted number of bytes of two strings.
- `compute_signed_magnitude_division`: Divides two register inputs by a restoring division loop over 32 bit iterations on their magnitudes, giving the quotient the sign of the xor of the inputs.
- `frag_link_cmpa_bcc_jmp_3`: Short code fragment that creates a stack frame in a6, then compares a7 with a fixed memory address, then branches if the carry flag is clear, then jumps to another point in the code.
- `initialize_and_scan_directory`: Initializes a counter and a memory area, then performs directory scanning operations using file handles and updates a global table with directory entry data.
- `scan_directory_entries`: Iterates through directory entries using a lock and examine handle, updates a counter for each entry, and modifies a global table based on the entry data.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: puts the result in d0, removes the stack frame with unlk, then returns to the caller.
- `frag_tst_bne_jsr_move_bra`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was not zero, then calls another routine, then copies d0 to the memory at an offset from a6, then jumps to another point in the code.
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: puts the result in d0, removes the stack frame with unlk, then returns to the caller.
- `frag_cmpi_bne_move_unlk_rts`: Short code fragment that compares the memory at an offset from a6 with 232, then branches if the last result was not zero, then copies a fixed memory address to d0, then removes the stack frame held in a6, then returns.
- `function_epilogue_3`: Function exit sequence shared or jumped to by a routine: puts the result in d0, removes the stack frame with unlk, then returns to the caller.
- `frag_link_cmpa_bcc_jmp_4`: Short code fragment that creates a stack frame in a6, then compares a7 with a fixed memory address, then branches if the carry flag is clear, then jumps to another point in the code.
- `update_tile_pointers`: Calculates and stores memory addresses for multiple tile locations based on a counter value, then calls a gadget refresh routine with specific parameters.
- `frag_link_cmpa_bcc_jmp_5`: Short code fragment that creates a stack frame in a6, then compares a7 with a fixed memory address, then branches if the carry flag is clear, then jumps to another point in the code.
- `handle_property_modification`: Updates a global variable with a computed value, performs a property modification operation with several parameters, and returns the result of a subsequent calculation.
- `function_epilogue_4`: Function exit sequence shared or jumped to by a routine: puts the result in d0, removes the stack frame with unlk, then returns to the caller.
- `frag_link_cmpa_bcc_jmp_6`: Short code fragment that creates a stack frame in a6, then compares a7 with a fixed memory address, then branches if the carry flag is clear, then jumps to another point in the code.
- `initialize_requester`: Initializes a requester structure with specific dimensions and color values, then clears a memory block based on a counter before calling another routine.
- `initialize_display_loop`: Initializes a display loop counter to 50 and clears memory bytes at offsets calculated from the counter value, then calls a routine to request display resources and processes input events until the counter reaches zero.
- `request_display_resources`: Requests display resources, initializes palette entries, processes input events until a condition is met, and updates global variables based on event types.
- `frag_bra_30`: Short code fragment that jumps to another point in the code.
- `frag_bra_31`: Short code fragment that jumps to another point in the code.
- `frag_bra_32`: Short code fragment that jumps to another point in the code.
- `frag_bra_33`: Short code fragment that jumps to another point in the code.
- `frag_bra_34`: Short code fragment that jumps to another point in the code.
- `frag_bra_35`: Short code fragment that jumps to another point in the code.
- `frag_bra_36`: Short code fragment that jumps to another point in the code.
- `frag_pea_bsr_addq_move_bra`: Short code fragment that pushes the address of the memory at an offset from a6 onto the stack, then calls another routine, then adds 4 to a7, then copies d0 to the memory at an offset from a6, then jumps to another point in the code.
- `process_input_and_update`: Processes input events and updates internal state variables based on input conditions, including handling specific key presses and managing display refresh operations.
- `load_and_process_input`: Initializes a display palette and requests a resource, then processes input events until a specific condition is met, updating internal state variables based on event data.
- `frag_clr_move_andi_bra`: Short code fragment that clears the memory at an offset from a6, then copies a fixed memory address to d0, then masks d0 with 65407, then copies d0 to a fixed memory address, then jumps to another point in the code.
- `frag_moveq_move_andi_bra`: Short code fragment that loads 0 into d0, then copies d0 to a fixed memory address, then copies a fixed memory address to d1, then masks d1 with 65407, then copies d1 to a fixed memory address, then copies d0 to the memory at an offset from a6, then jumps to another point in the code.
- `load_palette_and_request`: Initializes a display palette and requests a resource, then processes input events until a specific condition is met, updating internal state flags based on event types.
- `function_epilogue_5`: Function exit sequence shared or jumped to by a routine: puts the result in d0, removes the stack frame with unlk, then returns to the caller.
- `render_explosion`: Renders an explosion animation by drawing patterns with clipped blitting operations, updates frame counters, and manages rendering states through global variables.
- `update_sprite_position`: Updates sprite position coordinates based on boundary checks and performs blitting operations with clipped regions.
- `frag_link_cmpa_bcc_jmp_7`: Short code fragment that creates a stack frame in a6, then compares a7 with a fixed memory address, then branches if the carry flag is clear, then jumps to another point in the code.
- `dispatch_tile_action`: Uses a lookup table to determine which action to perform based on tile data, then jumps to specific code locations for different tile types.
- `frag_bra_37`: Short code fragment that jumps to another point in the code.
- `frag_bra_38`: Short code fragment that jumps to another point in the code.
- `frag_bra_39`: Short code fragment that jumps to another point in the code.
- `frag_bra_40`: Short code fragment that jumps to another point in the code.
- `frag_bra_41`: Short code fragment that jumps to another point in the code.
- `frag_bra_42`: Short code fragment that jumps to another point in the code.
- `frag_bra_43`: Short code fragment that jumps to another point in the code.
- `frag_bra_44`: Short code fragment that jumps to another point in the code.
- `frag_bra_45`: Short code fragment that jumps to another point in the code.
- `frag_bra_46`: Short code fragment that jumps to another point in the code.
- `frag_bra_47`: Short code fragment that jumps to another point in the code.
- `frag_bra_48`: Short code fragment that jumps to another point in the code.
- `frag_bra_49`: Short code fragment that jumps to another point in the code.
- `frag_move_muls_movea_adda_bra_7`: Short code fragment that copies the memory at an offset from a6 to d0, then multiplies d0 by 60 (signed), then copies d0 to a0, then adds 0 to a0, then copies the memory at an offset from a6 to d0, then multiplies d0 by 6 (signed), then adds d0 to a0, then copies 1608 to the memory at a0, then jumps to another point in the code.
- `frag_move_muls_movea_adda_bra_8`: Short code fragment that copies the memory at an offset from a6 to d0, then multiplies d0 by 60 (signed), then copies d0 to a0, then adds 0 to a0, then copies the memory at an offset from a6 to d0, then multiplies d0 by 6 (signed), then adds d0 to a0, then copies 1642 to the memory at a0, then jumps to another point in the code.
- `store_tile_value_68c`: Calculates an address using coordinates and stores the value 1676 at that location before branching to a fixed location.
- `frag_move_muls_movea_adda_bra_9`: Short code fragment that copies the memory at an offset from a6 to d0, then multiplies d0 by 60 (signed), then copies d0 to a0, then adds 0 to a0, then copies the memory at an offset from a6 to d0, then multiplies d0 by 6 (signed), then adds d0 to a0, then copies 1710 to the memory at a0, then jumps to another point in the code.
- `store_tile_value_6d0`: Calculates an address using coordinates and stores the value 1744 at that location before branching to a fixed location.
- `store_tile_value_6f2`: Calculates an address using coordinates and stores the value 1778 at that location before branching to a fixed location.
- `frag_move_muls_movea_adda_unlk_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then multiplies d0 by 60 (signed), then copies d0 to a0, then adds 0 to a0, then copies the memory at an offset from a6 to d0, then multiplies d0 by 6 (signed), then adds d0 to a0, then copies 1812 to the memory at a0, then removes the stack frame held in a6, then returns.
- `frag_link_cmpa_bcc_jmp_8`: Short code fragment that creates a stack frame in a6, then compares a7 with a fixed memory address, then branches if the carry flag is clear, then jumps to another point in the code.
- `fill_tile_data`: Writes word values from a table into memory locations calculated from offsets, then writes a series of fixed words to a memory area pointed to by a6 minus 8.
- `frag_addq_bra`: Short code fragment that adds 1 to the memory at an offset from a6, then jumps to another point in the code.
- `write_tile_data`: Writes a series of word values to memory locations starting from a base pointer, then performs a calculation and writes the result followed by additional word values.
- `frag_link_cmpa_bcc_jmp_9`: Short code fragment that creates a stack frame in a6, then compares a7 with a fixed memory address, then branches if the carry flag is clear, then jumps to another point in the code.
- `initialize_tile_loop`: Initializes loop counters for tile rendering, updates global variables with values from a data structure, and manages a stack frame during execution.
- `frag_addq_bra_2`: Short code fragment that adds 1 to the memory at an offset from a6, then jumps to another point in the code.
- `update_rendering_parameters`: Reads multiple word-sized values from a memory location pointed to by a register, storing them into various global variables.
- `setup_rendering_params`: Initializes rendering parameters by setting up buffer pointers, performing blitting operations with specific offsets, and configuring display settings.
- `initialize_rendering_sequence`: Initializes rendering parameters by setting up buffer pointers, performing blitting operations with specific offsets, and configuring display settings before starting graphical updates.
- `frag_link_cmpa_bcc_jmp_10`: Short code fragment that creates a stack frame in a6, then compares a7 with a fixed memory address, then branches if the carry flag is clear, then jumps to another point in the code.
- `write_file_data`: Opens two files with different parameters, seeks to the end of the second file, writes data to it, and closes both files while returning the write operation result.
- `frag_link_cmpa_bcc_jmp_11`: Short code fragment that creates a stack frame in a6, then compares a7 with a fixed memory address, then branches if the carry flag is clear, then jumps to another point in the code.
- `frag_move_jsr_addq_cmpi_bge_moveq_unlk`: Short code fragment that copies 1005 to the stack, then copies the memory at an offset from a6 to the stack, then calls another routine, then adds 8 to a7, then copies d0 to the memory at an offset from a6, then compares d0 with 1, then branches if the comparison was greater or equal, then loads 0 into d0, then removes the stack frame held in a6, then returns.
- `read_file_data`: Seeks to the beginning of a file, reads 300 bytes into memory, closes the file, and returns the result of the read operation.
- `render_pixel_pattern`: Processes pixel coordinates from a table to draw graphical patterns on screen, updating counters and handling conditional rendering logic based on data from memory locations.
- `render_game_ui`: Processes UI elements for the game screen, drawing images at specific coordinates and handling conditional rendering based on input parameters.
- `frag_bra_50`: Short code fragment that jumps to another point in the code.
- `frag_bra_51`: Short code fragment that jumps to another point in the code.
- `frag_bra_52`: Short code fragment that jumps to another point in the code.
- `frag_bra_53`: Short code fragment that jumps to another point in the code.
- `frag_bra_moveq_move_pea_jsr_lea`: Short code fragment that jumps to another point in the code, then loads 92 into d0, then copies d0 to the stack, then copies 128 to the stack, then pushes the address of a fixed memory address onto the stack, then copies a fixed memory address to the stack, then calls another routine, then loads the address of the memory at an offset from a7 into a7, then jumps to another point in the code.
- `frag_moveq_move_pea_jsr_lea_bra`: Short code fragment that loads 92 into d0, then copies d0 to the stack, then loads 80 into d0, then copies d0 to the stack, then pushes the address of a fixed memory address onto the stack, then copies a fixed memory address to the stack, then calls another routine, then loads the address of the memory at an offset from a7 into a7, then jumps to another point in the code.
- `frag_moveq_move_pea_jsr_lea_bra_2`: Short code fragment that loads 88 into d0, then copies d0 to the stack, then copies 140 to the stack, then pushes the address of a fixed memory address onto the stack, then copies a fixed memory address to the stack, then calls another routine, then loads the address of the memory at an offset from a7 into a7, then jumps to another point in the code.
- `render_text_patterns`: Processes pixel coordinates from a table to draw graphical patterns on screen, updating counters and handling conditional rendering logic.
- `render_text_strings`: Processes pixel coordinates from a table to draw graphical patterns on screen, updating counters and handling conditional rendering logic.
- `frag_link_cmpa_bcc_jmp_12`: Short code fragment that creates a stack frame in a6, then compares a7 with a fixed memory address, then branches if the carry flag is clear, then jumps to another point in the code.
- `process_game_ui_elements`: Processes UI elements for the game screen by drawing images at specific coordinates and handling conditional rendering based on input parameters.
- `parse_input_and_draw`: Creates a stack frame, parses input from a buffer, and manages sprite drawing operations based on parsed data and input conditions.
- `round_trip_value_through_helper`: Calls a helper in mode zero with the address of a local slot, makes two further helper calls, stores a register value into the slot, calls the helper again in mode one, and returns the slot's value.
- `frag_link_pea_moveq_move_bsr_addq_unlk`: Short code fragment that creates a stack frame in a6, then pushes the address of the memory at an offset from a6 onto the stack, then loads 1 into d0, then copies d0 to the stack, then calls another routine, then adds 8 to a7, then removes the stack frame held in a6, then returns.
- `transfer_global_word_by_direction`: Either stores a global 32-bit word into the caller's pointer when the direction flag is zero, or loads the caller's 32-bit value into the global word otherwise, then returns.
- `_BeginIO`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `_CreatePort`: The routine allocates a signal bit and a memory block, initializes fields in the block with parameters and the current task pointer, and either initializes a message list or adds the port to a global list.
- `_DeletePort`: Removes the port from a list when it is linked, marks it as deleted and frees its signal bit, then releases the 34-byte memory block.
- `stub_no_observable_effect`: The decompiled body is only a return; the listed instructions show no effect on memory, so the routine is treated as a stub.
- `copy_and_compare`: Copies bytes from one memory location to another while comparing them with a counter value, increments an internal counter when bytes match, and continues until a null byte is encountered.
- `increment_and_clear`: Decrements a counter, compares it with another value, and if they match, clears a byte at a calculated memory address before returning an incremented counter value.
- `count_string_bytes`: Scans forward from a pointer byte by byte until it reaches a zero byte, counting the bytes examined, and returns that count as the length of the string.
- `_Open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `_Close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `_Read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `_Write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `_Input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `_Output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `_Seek`: Library stub: loads its arguments and the dos library base, calls Seek and returns its result to the caller.
- `_DeleteFile`: Library stub: loads its arguments and the dos library base, calls DeleteFile and returns its result to the caller.
- `_Rename`: Library stub: loads its arguments and the dos library base, calls Rename and returns its result to the caller.
- `_Lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `_Examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `_ExNext`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `_IoErr`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `compute_binary_weighted_sum`: Takes two integer inputs and returns an integer result.
- `handle_game_event`: The routine is 21 instructions long but ends with an immediate return, performing no observable work on data or registers.
- `copy_string_until_terminator`: Copies bytes from the source string into the destination, including the terminating zero byte, and returns the destination pointer.
- `copy_string_with_termination`: Copies characters from one buffer to another until a null terminator is encountered or a specified count is reached, then writes a null terminator at the end of the destination buffer.
- `frag_movea_clr_move_unlk_rts`: Short code fragment that copies the memory at an offset from a6 to a0, then clears the memory at a0, then copies the memory at an offset from a6 to d0, then removes the stack frame held in a6, then returns.
- `copy_string_to_offset`: Calls a helper with the first argument to find an offset, then copies a NUL-terminated string from the second argument to that offset in memory, and returns the first argument.
- `copy_string_to_buffer`: Copies bytes from a source string into a buffer at an offset found through a helper, stopping at the NUL byte or when the count runs out, writes a terminating zero and returns the first argument.
- `expand_format_string`: Copies a format string into a bounded output buffer through a count-checked store, handling doubled percent signs and escape sequences.
- `allocate_and_init_block`: Allocates a block sized to the input plus a constant, stores that size inside the block, passes the global block and the block address to another routine, and returns the block address plus a constant offset.
- `frag_link_move_bsr_addq_tst_beq_unlk_rt`: Short code fragment that creates a stack frame in a6, then copies the memory at an offset from a6 to the stack, then calls another routine, then adds 4 to a7, then copies d0 to the memory at an offset from a6, then tests d0, then branches if the last result was zero, then removes the stack frame held in a6, then returns.
- `frag_movea_move_unlk_rts`: Short code fragment that copies an address to a0, then copies a0 to d0, then removes the stack frame held in a6, then returns.
- `call_then_clear_globals`: Calls another routine, then clears the long values stored at offsets 16, 8, 12, 8, 0, 12, and 4 of two global structures before returning.
- `free_linked_memory_blocks`: Walks a singly linked list whose head is held in a global, freeing each node with the size stored at its offset and following the next pointer until the list ends.
- `append_node_to_list`: Appends a node to a list whose head and tail pointers are held in the first structure: the node's next pointer is cleared, the previous tail is linked to it, and it becomes the tail, and also the head when the list was empty.
- `frag_link_movea_tst_bne_move_bra`: Short code fragment that creates a stack frame in a6, then copies the memory at an offset from a6 to a0, then tests the memory at a0, then branches if the last result was not zero, then copies the memory at an offset from a6 to a1, then copies the memory at an offset from a0 to the memory at an offset from a1, then jumps to another point in the code.
- `copy_structure_data`: Copies data from one memory location to another using intermediate pointers, then performs a conditional check on the copied data and branches based on the result.
- `frag_movea_move_unlk_rts_2`: Short code fragment that copies the memory at an offset from a6 to a1, then copies the memory at an offset from a1 to a0, then copies the memory at a1 to the memory at a0, then removes the stack frame held in a6, then returns.
- `parse_string_table`: Splits a command string into tokens separated by spaces, keeping double-quoted text as one token and storing a pointer to each token in a table.
- `check_flag_and_return_pointer`: Optionally calls a helper, then, if the index is in range and the table entry has its flag bit set, returns a pointer into the table.
- `process_flag_and_update_status`: Initializes a counter, compares it with a global limit, calls another routine to get an index, updates a table entry based on input parameters, opens a file handle, and stores the handle in the structure.
- `frag_move_cmp_bne_moveq_unlk_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then compares d0 with a fixed memory address, then branches if the last result was not zero, then loads 24 into d0, then copies d0 to a fixed memory address, then loads 255 into d0, then removes the stack frame held in a6, then returns.
- `update_status_and_open_file`: Updates a status byte based on bitwise operations between memory values, modifies file handling flags, and opens a file for reading or writing based on those conditions.
- `update_status_byte`: Updates a status byte at offset -12 from a6 based on a value from offset 12 of a6, then performs conditional bit operations and modifies the status byte accordingly.
- `frag_btst_beq_moveq_bra`: Short code fragment that tests bit 3 of the memory at an offset from a6, then branches if the last result was zero, then loads 8 into d0, then jumps to another point in the code.
- `frag_moveq_ori_movea_move_or_bra`: Short code fragment that loads 0 into d0, then merges 32 into d0, then loads 0 into d1, then copies the memory at an offset from a6 to a0, then copies the memory at a0 to d1, then merges d0 into d1, then copies d1 to the memory at a0, then jumps to another point in the code.
- `frag_btst_beq_moveq_bra_2`: Short code fragment that tests bit 3 of the memory at an offset from a6, then branches if the last result was zero, then loads 8 into d0, then jumps to another point in the code.
- `frag_moveq_ori_movea_move_or_bra_2`: Short code fragment that loads 0 into d0, then merges 96 into d0, then loads 0 into d1, then copies the memory at an offset from a6 to a0, then copies the memory at a0 to d1, then merges d0 into d1, then copies d1 to the memory at a0, then jumps to another point in the code.
- `frag_movea_clr_moveq_move_unlk_rts`: Short code fragment that copies the memory at an offset from a6 to a0, then clears the memory at a0, then loads 22 into d0, then copies d0 to a fixed memory address, then loads 255 into d0, then removes the stack frame held in a6, then returns.
- `check_and_modify`: Tests a bit flag in memory, branches if the flag is clear, loads values from memory, passes them to a system routine, and modifies a global variable based on the result.
- `open_file_and_store_handle`: Prepares arguments for a file open operation, calls the system routine to open a file, stores the resulting handle in a structure, and returns an error code if the operation failed.
- `function_epilogue_6`: Function exit sequence shared or jumped to by a routine: puts the result in d0, removes the stack frame with unlk, then returns to the caller.
- `clear_status_flag_and_recurse`: Clears a status bit in the second argument and calls the routine again with the modified arguments, passing the first argument unchanged.
- `read_into_buffer_with_error_handling`: Checks if an index is valid, calls another routine to get a buffer pointer, reads data into that buffer using a system call, and returns the result of the read operation or an error code.
- `read_into_buffer_checked`: Calls a read library routine on the stored handle with the caller's buffer and length, and keeps the byte count.
- `write_to_handle_checked`: Obtains a file handle and, when its flag bit is set, seeks to the end before writing the buffer.
- `seek_file_with_offset_table`: Obtains a file handle and, for an index below three, seeks to an offset taken from a global table, records any I/O error, then seeks back to the start and returns the result.
- `close_handle_if_flag_clear`: Obtains a structure from its argument and returns success without closing when its flag bit is set.
- `delete_file_and_handle_error`: Calls a system routine to delete a file, checks if the operation succeeded, and returns an error code if it failed.
- `function_epilogue_7`: Function exit sequence shared or jumped to by a routine: puts the result in d0, removes the stack frame with unlk, then returns to the caller.
- `forward_delete_file`: Pushes the long argument at offset 8 of the stack frame and calls the file-deletion routine with it, then removes the argument and returns.
- `rename_file_and_handle_error`: Calls a system routine to rename a file, checks if the operation succeeded, and returns an error code if it failed.
- `function_epilogue_8`: Function exit sequence shared or jumped to by a routine: puts the result in d0, removes the stack frame with unlk, then returns to the caller.
- `frag_move_pea_jsr_addq_rts`: Short code fragment that copies 1005 to the stack, then pushes the address of a fixed memory address onto the stack, then calls another routine, then adds 8 to a7, then copies d0 to a fixed memory address, then returns.
- `check_signal_and_read_byte`: Checks a global flag, calls another routine if set, reads a byte from a handle, and returns that byte value.
- `frag_moveq_move_pea_jsr_lea_unlk_rts`: Short code fragment that loads 1 into d0, then copies d0 to the stack, then pushes the address of the memory at an offset from a6 onto the stack, then copies a fixed memory address to the stack, then calls another routine, then loads the address of the memory at an offset from a7 into a7, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then removes the stack frame held in a6, then returns.
- `frag_link_tst_beq_move_clr_andi_unlk_rt`: Short code fragment that creates a stack frame in a6, then tests a fixed memory address, then branches if the last result was zero, then copies a fixed memory address to d0, then clears a fixed memory address, then copies d0 to the memory at an offset from a6, then masks d0 with 255, then removes the stack frame held in a6, then returns.
- `frag_bsr_unlk_rts`: Short code fragment that calls another routine, then removes the stack frame held in a6, then returns.
- `write_byte_to_handle`: Checks global flags, calls setup routines if needed, then calls the Write library function to write one byte from the stack argument to the global file handle and returns it.
- `frag_link_tst_beq_moveq_unlk_rts`: Short code fragment that creates a stack frame in a6, then tests a fixed memory address, then branches if the last result was zero, then loads 255 into d0, then removes the stack frame held in a6, then returns.
- `frag_move_unlk_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then copies d0 to a fixed memory address, then removes the stack frame held in a6, then returns.
- `read_file_via_dispatch`: Runs the signal check and dispatch routine when a global flag is nonzero, opens the file if its handle is zero, then reads up to 255 bytes into the buffer and writes a zero terminator after the bytes read, returning the buffer pointer.
- `write_string_to_file`: Runs the signal check and dispatch routine when a global flag is nonzero, opens the file if its handle is zero, then computes the byte count of the string argument and writes that many bytes to the file.
- `check_signal_and_dispatch`: Reads the signal state for two bits, and when neither is set returns zero.
- `_NewList`: Short code fragment that copies the memory at an offset from a7 to a0, then copies a0 to the memory at a0, then adds 4 to the memory at a0, then clears the memory at an offset from a0, then copies a0 to the memory at an offset from a0, then returns.
- `format_unsigned_decimal_text`: Converts an unsigned value into up to eleven decimal digit characters in a stack buffer, pads the result to a minimum width when it is shorter, copies the characters into the destination and zero-terminates it, returning the length.
- `initialize_counter_and_check`: Initializes a 32-bit counter in a global variable to zero, then compares the address stored at a specific offset with a fixed value of 480, and branches to the end if the comparison result is not greater or equal.
- `frag_movea_cmpa_bne_moveq_unlk_rts`: Short code fragment that copies the memory at an offset from a6 to a0, then compares a0 with 480, then branches if the last result was not zero, then loads 0 into d0, then removes the stack frame held in a6, then returns.
- `frag_move_bsr_lea_unlk_rts`: Short code fragment that copies the memory at an offset from a6 to the stack (3 times in a row), then calls another routine, then loads the address of the memory at an offset from a7 into a7, then removes the stack frame held in a6, then returns.
- `process_command_input`: Reads command characters from memory, tests for specific conditions, and calls other routines to handle different command types with various flags and parameters.
- `process_input_command`: Reads a command character from a memory location, tests if it's a plus sign, and based on the command and test result, calls another routine with specific parameters and stores the returned value.
- `initialize_and_process`: Loads a constant value into register d0, combines it with data from the stack, passes the result to another routine, and stores the returned value in a global variable at offset -8 of the frame.
- `frag_moveq_move_clr_jsr_lea_tst_beq_bra`: Short code fragment that loads 2 into d0, then copies d0 to the stack, then clears the stack, then copies the memory at an offset from a6 to the stack, then calls another routine, then loads the address of the memory at an offset from a7 into a7, then tests the memory at an offset from a6, then branches if the last result was zero, then copies 128 to d0, then jumps to another point in the code.
- `frag_moveq_move_bra_3`: Short code fragment that loads 2 into d0, then copies d0 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_tst_beq_moveq_bra`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was zero, then loads 2 into d0, then jumps to another point in the code.
- `process_and_update`: Combines data from memory with a zero value, passes the combined result to another routine, and stores the returned value in a global variable at offset -8 of the frame.
- `frag_tst_beq_move_bra`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was zero, then copies 128 to d0, then jumps to another point in the code.
- `frag_moveq_move_bra_4`: Short code fragment that loads 1 into d0, then copies d0 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_tst_beq_moveq_bra_2`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was zero, then loads 2 into d0, then jumps to another point in the code.
- `set_flags_and_execute`: Sets multiple bit flags in a register, combines the register value with additional constants, passes the result to another routine, and stores the returned value in a global variable at offset -8 of the frame.
- `frag_tst_beq_move_bra_2`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was zero, then copies 128 to d0, then jumps to another point in the code.
- `frag_moveq_move_bra_5`: Short code fragment that loads 2 into d0, then copies d0 to the memory at an offset from a6, then jumps to another point in the code.
- `function_epilogue_9`: Function exit sequence shared or jumped to by a routine: puts the result in d0, removes the stack frame with unlk, then returns to the caller.
- `validate_and_update`: Retrieves data from memory, calls a system routine with that data, updates memory locations based on the result, and returns a pointer to a structure.
- `clear_and_set_record_fields`: Clears a 32-bit field in a structure, sets a byte value at an offset from that structure, copies a value from another location to the structure's buffer pointer, clears two 32-bit fields, sets a byte value at another offset, and returns a pointer to the structure.
- `clear_object_links`: Calls one routine with the object when flag bit two is set, then clears its two pointer fields and flag byte, and finally passes a byte from the object to another routine.
- `frag_link_move_asl_unlk_rts`: Short code fragment that creates a stack frame in a6, then copies a fixed memory address to d0, then shifts d0 left by 3, then removes the stack frame held in a6, then returns.
- `frag_link_move_bsr_addq_unlk_rts`: Short code fragment that creates a stack frame in a6, then copies the memory at an offset from a6 to d0, then copies d0 to the stack, then copies d0 to the memory at an offset from a6, then calls another routine, then adds 4 to a7, then removes the stack frame held in a6, then returns.
- `find_node_by_distance`: Walks a linked list of nodes from a global head, moving by the requested distance and updating a global counter.
- `frag_link_move_bsr_addq_unlk_rts_2`: Short code fragment that creates a stack frame in a6, then copies the memory at an offset from a6 to d0, then copies d0 to the stack, then copies the memory at an offset from a6 to the stack, then copies d0 to the memory at an offset from a6, then calls another routine, then adds 8 to a7, then removes the stack frame held in a6, then returns.
- `insert_or_update_list_node`: Inserts a node into a list anchored by a global variable at a position derived from a global counter, or updates an existing node.
- `frag_link_movea_move_andi_tst_beq_moveq`: Short code fragment that creates a stack frame in a6, then copies the memory at an offset from a6 to a0, then copies the memory at an offset from a0 to d0, then masks d0 with 48, then tests d0, then branches if the last result was zero, then loads 255 into d0, then removes the stack frame held in a6, then returns.
- `allocate_buffer_and_update_flags`: Checks if a stream is already allocated or flagged, allocates a buffer if needed, updates flags in the stream structure, and returns a byte from the buffer.
- `frag_movea_move_btst_beq_moveq_bra`: Short code fragment that copies the memory at an offset from a6 to a0, then copies the memory at an offset from a0 to d0, then tests bit 2 of d0, then branches if the last result was zero, then loads 1 into d0, then copies d0 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_movea_move_btst_beq_moveq_unlk_rts`: Short code fragment that copies the memory at an offset from a6 to a0, then copies the memory at an offset from a0 to d0, then tests bit 1 of d0, then branches if the last result was zero, then loads 255 into d0, then removes the stack frame held in a6, then returns.
- `read_and_update_flags`: Updates a byte value at a specific offset in memory, reads data using a system call, stores the result in memory, and modifies flags based on the read result.
- `decrement_and_read_byte`: Loads a counter value from a structure, decrements it, stores the updated value back, checks if the counter is negative, increments another pointer, and returns a byte from that pointer.
- `frag_move_bsr_addq_unlk_rts`: Short code fragment that copies the memory at an offset from a6 to the stack, then calls another routine, then adds 4 to a7, then removes the stack frame held in a6, then returns.
- `write_byte_with_retry`: Writes a byte from its first argument into a buffer tracked by a state byte and a count field, updating the buffer position and counter.
- `allocate_stream_buffer`: Returns minus one when the stream is already flagged.
- `close_active_handles`: Loops over a table of six-byte entries up to a global count, closing each entry that is non-zero and does not have flag bit four set, then calls another routine with its original parameters.
- `init_intuition_request`: Opens the intuition library, stores the handle in a global variable, and performs an auto request with fixed dimensions.
- `parse_format_specifier`: Parses format string characters to extract numeric values and flags, performs calculations on these values, and handles different format specifiers like '-' and '.', then jumps to specific handlers based on character matches.
- `frag_ori_andi_bra`: Short code fragment that merges 24576 into the memory just below a6 (moving a6 down), then masks the memory at an offset from a6 with 0, then jumps to another point in the code.
- `frag_ori_andi_bra_2`: Short code fragment that merges 24576 into the stack, then masks the memory at a0 with 0, then merges 24576 into the memory just below a3 (moving a3 down), then masks the memory at an offset from a2 with 0, then jumps to another point in the code.
- `frag_ori_bra`: Short code fragment that merges 24576 into the memory at an offset from a7, then merges 120 into d0, then jumps to another point in the code.
- `frag_ori_bra_2`: Short code fragment that merges 24576 into an indexed entry from a5, then merges 100 into d0, then jumps to another point in the code.
- `frag_tst_beq_movea_addq_move_bra`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was zero, then copies the memory at an offset from a6 to a1, then copies the memory at a1 to a0, then adds 4 to the memory at a1, then copies the memory at a0 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_movea_addq_move_tst_bpl_moveq_bra`: Short code fragment that copies the memory at an offset from a6 to a1, then copies the memory at a1 to a0, then adds 4 to the memory at a1, then copies the memory at a0 to the memory at an offset from a6, then tests the memory at an offset from a6, then branches if the last result was not negative, then loads 1 into d0, then copies d0 to the memory at an offset from a6, then jumps to another point in the code.
- `check_and_advance_pointer`: Tests a global variable for zero and branches if zero, otherwise loads two pointers from a structure, advances one pointer by four bytes, and copies a long word from the other pointer to a global variable.
- `convert_integer_to_string`: Converts a signed integer into its decimal string representation in a local buffer, handling negative numbers by prepending a minus sign, and manages digit placement and buffer boundaries.
- `validate_and_copy_data`: Tests a global variable for zero and branches if zero, otherwise loads two pointers from a structure, advances one pointer by four bytes, and copies a long word from the other pointer to a global variable.
- `decode_nibble_sequence`: Processes a sequence of nibbles from a source buffer, mapping them through a table and storing the results in a destination buffer.
- `load_data_segment`: Loads a data segment by reading a value from a pointer, incrementing the pointer, and storing the read value into another location.
- `decode_data_segment`: Processes a data segment by extracting bits and storing them as characters in a buffer, then calls another routine to handle the decoded data.
- `initialize_data_buffer`: Initializes a data buffer by setting up pointers and counters based on conditional checks, then copies data from a source location to a destination buffer while managing offsets and lengths.
- `load_byte_to_buffer`: Loads a byte from a source pointer into a buffer at offset -72 relative to a6, then updates the source pointer and sets a counter in a global variable.
- `frag_moveq_move_bra_6`: Short code fragment that loads 2 into d0, then copies d0 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_clr_bra`: Short code fragment that clears the memory at an offset from a6, then jumps to another point in the code.
- `frag_moveq_move_bra_movem_unlk_rts`: Short code fragment that loads 1 into d0, then copies d0 to the memory at an offset from a6, then jumps to another point in the code, then loads 0 into d0, then restores saved registers from the stack, then removes the stack frame held in a6, then returns.
- `copy_data_segments`: Copies data segments from a source pointer to a destination buffer, managing counters and offsets based on conditional logic.
- `decrement_and_write`: Decrements a counter value, checks if it's negative, and if not, calculates an address by adding an offset to a pointer, increments the offset, writes a byte from a register to that address, then loops back to continue processing.
- `decrement_and_write_2`: Decrements a counter value in memory, checks if it's negative, calculates a memory address by adding an offset to a pointer, and writes data from another location to that calculated address.
- `decrement_and_copy`: Decrements a counter value, checks if it's negative, and if not, calculates an address by adding an offset to a pointer, increments the offset, copies a byte from another memory location to that address, increments the source offset, then loops back to continue processing.
- `frag_movea_move_addq_movem_unlk_rts`: Short code fragment that copies the memory at an offset from a6 to a0, then copies the memory at an offset from a6 to the memory at a0, then copies the memory at an offset from a6 to a0, then adds 1 to a0, then copies a0 to d0, then restores saved registers from the stack, then removes the stack frame held in a6, then returns.
- `frag_cmpi_bne_moveq_move_blt_bra`: Short code fragment that compares the memory at an offset from a6 with an address, then branches if the last result was not zero, then loads 6 into d0, then copies d0 to the memory at an offset from a6, then compares the memory at an offset from a6 with 20, then branches if the comparison was less, then loads 19 into d0, then jumps to another point in the code.
- `update_counter_2`: Updates a counter value based on conditions involving multiple global variables, then stores the result in a memory location pointed to by a pointer.
- `update_counter`: Updates a counter value based on conditions involving multiple global variables, then stores the result in a memory location pointed to by a6+12.
- `frag_move_addq_add_tst_bpl_neg_bra`: Short code fragment that copies the memory at an offset from a6 to d0, then adds 6 to d0, then adds d0 to the memory at an offset from a6, then tests the memory at an offset from a6, then branches if the last result was not negative, then copies the memory at an offset from a6 to d0, then negates d0, then jumps to another point in the code.
- `convert_integer_to_string_2`: Converts an integer value into its string representation, storing digits at a memory location pointed to by a6+12.
- `write_numeric_string`: Writes a numeric string representation to a memory location pointed to by a6+12, handling digit placement and decimal point insertion based on counter values.
- `copy_and_increment`: Decrements a counter value in memory, checks if it's negative, calculates a memory address by adding an offset to a pointer, copies data from one location to another, and increments both offsets.
- `frag_movea_move_addq_bra`: Short code fragment that copies the memory at an offset from a6 to a0, then copies 48 to the memory at a0, then adds 1 to the memory at an offset from a6, then jumps to another point in the code.
- `decrement_and_copy_2`: Decrements two counters stored at offsets -40 and -12 from a6, copies bytes from one memory location to another based on the counter values, and updates both source and destination pointers.
- `frag_movea_move_addq_bra_2`: Short code fragment that copies the memory at an offset from a6 to a0, then copies 48 to the memory at a0, then adds 1 to the memory at an offset from a6, then jumps to another point in the code.
- `loop_and_write_data`: Processes data in a loop by copying bytes from one memory location to another, updates counters, writes specific values to memory, and modifies global variables based on the processing results.
- `frag_movea_move_addq_bra_3`: Short code fragment that copies the memory at an offset from a6 to a0, then copies 48 to the memory at a0, then adds 1 to the memory at an offset from a6, then jumps to another point in the code.
- `decrement_and_copy_character`: Loads a long word from a global variable, decrements it, stores it back, tests if the result is negative and branches if so, otherwise loads two pointers from global variables, copies a byte from the first pointer to the second pointer, and advances both pointers.
- `write_decimal_digit`: Writes a decimal digit '0' to memory location pointed to by a6 offset 12, increments the pointer, then writes a period character '.' to the next location and increments the pointer again.
- `frag_movea_move_addq_bra_4`: Short code fragment that copies the memory at an offset from a6 to a0, then copies 48 to the memory at a0, then adds 1 to the memory at an offset from a6, then jumps to another point in the code.
- `format_number_string_3`: Writes a character 'E' to memory location pointed to by a6 offset 12, increments the pointer, and handles negative numbers by writing a '-' character and negating the value at offset -40 from a6.
- `format_number_string_2`: Converts a number into a string representation by repeatedly dividing by ten and storing remainder digits in reverse order, then copies the result to an output buffer.
- `process_and_write_data`: Compares two counter values and performs subtraction if conditions are met, decrements a counter, calculates an address by adding an offset to a pointer, increments the offset, writes a byte from a register to that address, increments another counter, then loops back to continue processing.
- `frag_movea_move_addq_movem_unlk_rts_2`: Short code fragment that copies the memory at an offset from a6 to a0, then copies the memory at an offset from a6 to the memory at a0, then copies the memory at an offset from a6 to a0, then adds 1 to a0, then copies a0 to d0, then restores saved registers from the stack, then removes the stack frame held in a6, then returns.
- `update_game_state`: Updates internal game state variables based on conditional checks and counter values, then performs branching logic depending on those conditions.
- `update_decimal_counter`: Updates a decimal counter stored in memory, adjusts its value based on conditions, and performs bit manipulation operations on the counter's components.
- `frag_bsr_addq_bra`: Short code fragment that calls another routine, then adds 1 to the memory at an offset from a1, then jumps to another point in the code.
- `process_64bit_value`: Processes a 64-bit value through multiple iterations of multiplication by ten, updates counters, and performs bit shifting operations on the value while maintaining carry flags.
- `validate_and_shift_value`: Validates a 64-bit value for zero condition, performs multiplication by ten if non-zero, checks exponent limits, and updates a counter based on comparison results.
- `normalize_decimal_string`: Processes a decimal string representation, increments digits when overflow occurs, and shifts the decimal point position while updating counters.
- `frag_lsr_roxr_sub_bne_rts`: Short code fragment that shifts d0 right by 1, then rotates d1 right through the extend flag by 1, then subtracts 1 from d2, then branches if the last result was not zero, then returns.
- `multiply_64bit_by_ten`: Multiplies a 64-bit value held in d0 and d1 by ten using shifts and adds with carry, producing the high word in d0 and the low word in d1.
- `shift_bits_into_decimal_digits`: Runs a 64-iteration loop that shifts a 64-bit value in d0 and d1 left one bit at a time, moving each bit into a 16-bit accumulator in d6 and subtracting ten with a carry into d1 whenever the accumulator exceeds nine.
- `initialize_numeric_value`: Initializes a structured numeric value by clearing its components and setting up default state, then processes input digits and signs to build a numeric representation.
- `parse_numeric_value`: Processes a numeric character input to update a structured value, handling digit parsing, sign adjustment, and exponent management while modifying flags and counters in the structure pointed to by a1.
- `frag_bsr_move_andi_add_clr_addx_bra`: Short code fragment that calls another routine, then copies the memory at an offset from a1 to d6, then masks d6 with 15, then adds d6 to d1, then clears d6, then adds d6 and the extend flag to d0, then jumps to another point in the code.
- `frag_cmpi_bne_tst_addq_bra`: Short code fragment that compares d6 with 46, then branches if the last result was not zero, then tests the memory at an offset from a1, then branches if the last result was not zero, then adds 1 to the memory at an offset from a1, then jumps to another point in the code.
- `parse_number`: Parses a numeric value from input characters, handling signed numbers and scaling operations, then stores the result in a memory location.
- `frag_move_andi_bne_movea_clr_bra`: Short code fragment that copies the memory at an offset from a1 to the stack, then masks the top of the stack with 8192, then branches if the last result was not zero, then copies the memory at an offset from a0 to a2, then clears the memory at a2, then jumps to another point in the code.
- `adjust_and_shift`: Reads a 16-bit value from memory at offset 0 of a pointer in a1, applies a bitwise AND with 4096 to determine if the sign should be negated, then adds the result to a 16-bit value at offset 2 of the same pointer.
- `frag_tst_beq_bmi_moveq_add_bsr_subq_bra`: Short code fragment that tests the memory at an offset from a1, then branches if the last result was zero, then branches if the last result was negative, then loads 4 into d2, then adds d2 to the memory at an offset from a1, then calls another routine (2 times in a row), then subtracts 1 from the memory at an offset from a1, then jumps to another point in the code.
- `frag_bsr_addq_bra_2`: Short code fragment that calls another routine, then adds 1 to the memory at an offset from a1, then jumps to another point in the code.
- `store_value_set_done`: Stores a 64-bit value through a result pointer held in the frame and sets the done flag in the result record to one.
- `frag_move_beq_rts`: Short code fragment that copies the memory at an offset from a0 to d6, then branches if the last result was zero, then copies 0 to the memory at an offset from a0, then returns.
- `save_registers_and_execute`: Saves six data registers and three address registers onto the stack, loads an address from offset 16 of a pointer in a0, calls that address, stores the result in d6, restores all saved registers, writes the value in d6 to offset 8 of a pointer in a1, decrements a 32-bit value at offset 12 of the pointer in a0, and branches back to this routine if the decrement result is non-zero.
- `frag_move_andi_beq_moveq_rts`: Short code fragment that copies the memory at an offset from a1 to the stack, then masks the top of the stack with 2048, then branches if the last result was zero, then loads 255 into d6, then returns.
- `frag_link_move_unlk_rts`: Short code fragment that creates a stack frame in a6, then copies the memory at an offset from a6 to a fixed memory address, then removes the stack frame held in a6, then returns.
- `frag_move_and_bne_moveq_bra`: Short code fragment that copies d0 to d2, then masks d0 with an address, then branches if the last result was not zero, then loads 0 into d0, then loads 0 into d1, then loads 0 into d2, then jumps to another point in the code.
- `shift_sign_bits_to_result`: Shifts the value in D1 left ten times, moving the bit shifted out of D1 into D0 at each step, then returns D0 with its top bit set.
- `normalize_fixed_point_2`: Normalizes a fixed-point value by shifting bits left or right to position the decimal point, handles overflow conditions, and updates exponent and mantissa values accordingly.
- `normalize_fixed_point`: Shifts bits in registers to normalize a fixed-point value, performs conditional operations based on bit tests and comparisons, handles overflow conditions, and returns the normalized result with proper bit manipulation.
- `frag_move_jsr_moveq_bra`: Short code fragment that copies 1 to the stack, then calls another routine, then loads 0 into d0, then jumps to another point in the code.
- `frag_move_jsr_swap_or_clr_lea_moveq_bra`: Short code fragment that copies 2 to the stack, then calls another routine, then copies 32752 to d0, then swaps the halves of d2, then merges d2 into d0, then swaps the halves of d0, then clears d0, then loads the address of the memory at an offset from a7 into a7, then loads 0 into d1, then jumps to another point in the code.
- `frag_movea_jsr_move`: Short code fragment that copies a fixed memory address to a7, then calls another routine, then copies 20 to the stack, then calls another routine.

### Diablo v1.0 (1986)(Extended Software).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_movea_jsr_move_ori`: Short code fragment that copies a fixed memory address to a7, then calls another routine, then copies 20 to the stack, then calls another routine, then merges 116 into d0.
