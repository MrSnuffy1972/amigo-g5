# Game 1406 (defender (1994)(ratsoft)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01406). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 20 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 465 of 465 named (51 author symbols), 465 of 465 described (71 by rule, 103 by the local model) |
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
| Defender (1994)(Ratsoft).adf:Defender | 6 | 5016 | 52 |
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
| Defender (1994)(Ratsoft).adf:bootblock | 1 | 18 | 1 |

## Cases handled

- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Defender (1994)(Ratsoft).adf:Defender

- `run_program_entry_point`: Finds the current task, clears a return-message flag, calls the main routine and then runs a short cleanup, returning the main routine's result.
- `returnMsg`: Small routine of 1 instruction that merges 0 into d0.
- `start`: Opens the DOS library, opens and reads the score file named in the text, decodes the loaded data with an XOR key, then runs the per-frame routines for titles, shots, map, bobs, scan, audio and score display.
- `titles_etc`: Draws the title screen by programming the blitter for graphic blits and calling text and control routines.
- `p7_j4`: Walks a table of packed nibbles, accumulating them into two counters and calling text and life or smart-item routines as the values change.
- `textype`: Walks a list of 16-bit indices until a sentinel, and for each index copies eight font bytes into three bitplane buffers, advancing a running offset.
- `audio`: Updates the audio DMA control and channel registers, decrements per-channel counters and loads the next sound sample into a channel when its current one has finished or a higher-priority request exists.
- `audio_j7`: Clears the current sound request, records the last sound from a channel table, and then passes control to the sound-channel service routine.
- `next_wave`: Counts down a wave timer and, when it expires, reads the next level table byte, writes a new enemy list entry, sets its flag and initialises its fields from a counter.
- `drawbobs`: Walks a linked list of bob descriptors, programming the blitter's source, destination and mask registers for each entry, waiting for the blitter to be idle, starting the blit and marking the entry as done.
- `drawscan`: Waits for the blitter to be free and writes control words and pointers into a blitter control block to set up a scanline copy, with a second blit configured from a table entry when a flag applies.
- `drawicons`: Walks the enemy list and, for each active entry, computes its screen offset, picks shifted icon bytes from tables and ORs them into bitplane buffers while setting collision bytes.
- `drawmap`: Computes a vertical scroll offset from the map's horizontal position and, when the space flag is clear, programs the blitter and copies fifteen map entries, calculating source and destination addresses from map tables for each.
- `your_shots`: Scans the laser list for free slots and, when a fire control bit is set, starts a new shot at the ship's position and sets the shot sound.
- `you`: Updates the ship's vertical position, velocity and thrust from the control flags, clamps the values, computes a map offset and image index, writes thrust to an audio register and stores the ship's coordinates through a pointer.
- `control1`: Reads a 16-bit input word through a pointer and decodes it into a control value and facing direction.
- `control2`: Tests a table of key press bytes and sets a control bit for each pressed key, and calls a routine to update the smart counter when a smart key is pressed.
- `rez`: Increments a per-entry counter and, when a specific mode value and a free enemy slot line up, fills that slot's position and sprite fields and claims it.
- `drawbits`: Loops over a few rows and, for each one, ORs masked bit patterns read from a table into three bit-plane buffers at a computed screen offset.
- `stars`: Walks forty star slots, moving each one by a scroll delta and respawning it from a running counter when it expires or leaves the visible range.
- `addlife`: Waits for the blitter to go idle, then if the lives count is below six it programs a blitter copy with masks and pointers, taking the source from a table indexed by the play-off value and the lives count.
- `addsmart`: Waits for the blitter to be idle and returns if the smart-bomb count is above five.
- `takelife`: When the lives count is between one and five, waits for the blitter twice, programs a blitter operation with masks and pointers, and sets the size register to start the transfer.
- `takesmart`: When the smart-bomb count is between one and five, waits for the blitter to be idle, programs its control, mask and pointer registers, then starts a blit whose source comes from a table indexed by the play-off value and the smart count.
- `score_draw`: Calls the same redraw routine once for each score threshold the score has passed, and again when the score differs from its saved backup copy.
- `dwarem`: Copies eight rows of glyph bytes from a table into two bit-plane buffers for a pair of digit values, writing the second digit only when a flag byte is zero.
- `new_vblank`: Reads the interrupt request and enable registers, ANDs them together, and calls the copper interrupt handler when its bit is set.
- `vblint`: Sets the global flag word to 32 to mark that a vertical blank has happened.
- `copint`: Runs once per frame when the frame-done flag is set: it switches screens, applies the scroll value and cycles an animation counter, then draws icons.
- `keyboardint`: Reads a keyboard byte from the serial data register when a flag is set, decodes the key number and its press or release bit, and records it in the last-key variable and the key state table.
- `key_j1`: Clears the last-key variable and one key-state byte, then waits for the keyboard handshake flag and sets the handshake value to acknowledge it.
- `enemy`: Iterates through a list of enemy structures to update their positions and handle state changes.
- `scores`: Adds an amount to the score held as packed decimal digits, applying decimal correction to each byte.
- `set_bomb`: When a bomb slot is free, the frame parity matches and the source is within a horizontal window of the map position, it writes the bomb's horizontal position and a start height into that slot.
- `bombs`: Walks the bomb slots each frame, counting down each bomb's timer and freeing expired ones.
- `set_shot`: If a shot slot is free and the target lies within the visible window, it plays a shot sound, writes the shot's position and then a velocity derived from the ship's speed and facing.
- `shots`: Walks the shot slots, advancing each shot by its velocity and freeing it when it leaves the visible range or hits the top or bottom edge.
- `change_screens`: Toggles between two screen buffers: it saves the current plane pointers, loads the other buffer's pointer values, and points the active planes at it.
- `clear_screen`: Waits for the blitter to go idle, then programs it with full-screen size and masks and no source channel, writing to the buffer named by the first plane pointer.
- `initialise`: Runs the start-up sequence in order: saves the system registers, shuts down the operating system, installs the vertical blank handler, sets up interrupts, sets the initial screen pointers and builds the copper list.
- `setscreen`: Stores fixed low and high word values into three plane pointer variables and three alternate pointer variables, setting the starting screen buffer addresses.
- `newcopper`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `change_vblank`: Stores the address of the new vertical blank handler in a global variable and writes an interrupt-enable value that turns on the master interrupt bit along with the vertical blank, copper and ports bits.
- `setints`: Stores the address of the keyboard handler, then writes interrupt and timer register values for the first CIA chip.
- `close_down`: Disables interrupts and DMA by calling the system kill routine, then calls the revive routine to put the saved system state back.
- `save_registers`: Opens the graphics library and saves the current view and copper list pointers.
- `kill_system`: Writes a value with every bit set except the enable flag to the DMA control and interrupt enable registers, turning off all DMA channels and interrupts.
- `revive_system`: Disables everything, then restores the saved copper pointer, vertical blank and level-two interrupt vectors, and the saved DMA and interrupt enable values with the enable flag set.
- `closegraphics`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `opengraphics`: Repeatedly calls the library-open routine through a stored pointer until it returns a non-zero base address, then keeps that address in a global variable.
- `opendos`: Repeatedly calls the library-open routine through a stored pointer until it returns a non-zero base address, then keeps that address for the DOS library in a global variable.
- `closedos`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.

### More

- `start_program_parse_arguments`: Program entry point: it opens the DOS library, allocates a state record, and splits the command-line text into words.
- `release_program_resources`: Stores its argument in the program state record, then conditionally closes a file handle, replies to a message port, frees a memory block and closes a library, each guarded by a stored pointer.
- `run_file_pager_loop`: Main pager routine: it sets up the display window and opens the named file, showing a percentage progress prompt and a quit hint.
- `call_four_table_entries`: Calls a routine four times, once per entry of a four-slot global table, passing each entry together with a value taken from the structure it receives.
- `check_buffer_bytes_allowed`: Makes two library calls on a handle with an 80-byte buffer, then scans that buffer and returns one when every byte falls in the accepted character ranges, and zero otherwise.
- `dispatch_pager_key_command`: Waits for a key with a timeout, skips escape sequences, and dispatches the key code through a large switch.
- `run_pager_navigation_action`: Calls a few library routines with the pager state and then runs a stored routine through a pointer.
- `initialise_page_and_loop`: Sets the line counter to one and the stop flag to zero, calls a file routine and stores its result.
- `read_chunk_and_count_lines`: Reads a block into the line buffer through two library calls and stores the byte count.
- `read_next_text_line`: Reads up to 128 bytes into the buffer and scans forward to the next newline or form-feed, returning how many characters that line holds.
- `display_page_until_full`: Works out how many lines fit on a page from a few flags, then prints lines from the line table until the page is full or the file ends.
- `search_text_across_pages`: Shows a searching message, then repeatedly reads a block and scans it for the search string until a match is found or the file ends.
- `run_pager_call_with_setup`: Runs a fixed sequence around one main call: it prepares the state record, optionally writes a buffer, calls the supplied routine with the parameter, then releases the state.
- `call_and_conditionally_flush`: Calls a routine with two arguments, then calls a second routine with the first argument only when a flag in the state record is zero.
- `update_page_position_display`: Calls the output routine with the file handle and a stored value, then calls it again with the stored line limit from the state record.
- `redraw_bottom_line_row`: Calls the output routine with a zero offset and a count of one, then calls it again with the stored line limit minus one.
- `draw_text_rows_wrapped`: Draws up to the requested number of rows from the text buffer, writing at most 128 characters per chunk and counting line breaks.
- `call_chain_then_handler`: Calls a routine with its argument, then calls a second routine through a function pointer kept in the state record, passing the global table as its data.
- `set_page_flag_and_forward`: When a field in the state record is zero, sets a short field in a linked record to one.
- `call_output_with_flag_args`: Calls an output routine with one of two argument sets chosen by a flag in the state record: zeros, or a count made from the stored line limit plus one.
- `print_prompt_by_flag`: Calls an output routine with the file handle and either three copies of the global table pointer or the stored line limit, depending on a flag.
- `seek_and_return_clamped_position`: Makes two seek calls with a zero offset, then returns the second result, clamped to zero when it is negative.
- `copy_chunk_from_read_buffer`: Takes up to the requested number of bytes from the read buffer in the state record, refilling it first when it runs short.
- `refill_read_buffer_block`: Flushes a pending full buffer block if one exists, then reads up to 4096 bytes into the read buffer through a library call.
- `update_scroll_window_position`: Works out a target line from the base, start or end position chosen by a mode argument, then checks it against the current window bounds.
- `read_line_from_buffer`: Reads characters one at a time through a source call into the destination until a newline or 127 characters are read.
- `copy_null_terminated_string`: Copies bytes from a source string to a destination one at a time, including the terminating zero byte.
- `convert_string_to_uppercase`: Walks a null-terminated string and changes each letter from a to z into its capital by subtracting the case offset.
- `parse_signed_decimal`: Reads an optional plus or minus sign, then accumulates decimal digits from the string into an integer.
- `call_library_function_with_mask`: Checks if a pointer in a structure or table is non-null, then calls a library function using a base address from a global variable and a provided mask.
- `finish_and_free_request`: Writes its text argument to the output when it is not empty, waits for a newline if a flag is set, then frees and restores the linked buffers and fields of the request record.
- `send_single_byte_message`: Stores its argument in a one-element stack buffer and passes it to a library call as a one-byte message.
- `copy_record_fields_then_free`: Allocates a 36-byte record through a library call and, if that succeeds, reads a value from the stored port into a small stack buffer.
- `allocate_linked_record_copy`: Allocates a 68-byte record and links it into a list through library calls, then copies a given number of words from the source array into it.
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
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
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
- `forward_with_stack_pointer`: Forwards its two arguments to a second routine together with a pointer to a fixed stack slot.
- `read_single_byte_or_fail`: Calls a routine to fill a one-byte stack buffer.
- `format_message_and_emit`: Formats text into a 140-byte stack buffer through a library formatting call, measures its length, and passes the text and length to a second routine when it is not empty.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `divide_32bit_with_remainder`: Divides a 32-bit value by a 16-bit divisor.
- `multiply_32bit_partial_products`: Multiplies two 32-bit values using 16-bit partial products and returns the 32-bit product.
- `stub_self_call_recursion`: Only calls itself and returns, so it does no real work and is best treated as a stub or decoded padding.
- `negate_result_if_flagged`: Calls a routine and returns its first result, negated when the second return register is non-zero.
- `call_library_entry_from_table`: Calls a library routine through a function pointer stored at a fixed offset in the global table, then returns.
- `store_byte_at_pointer`: Stores the low byte of the data register at the address held in the pointer register, then returns.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `allocate_and_send_message`: Allocates a 34-byte message block through a library call and fills in its fields from the arguments and a value from another library call.
- `release_message_port_block`: When a field in the block is non-zero it first calls a removal routine.
- `search_substring_before_newline`: Searches the first string for the second one, stopping at a newline.
- `initialise_list_header_node`: Sets the first word of a block to point four bytes past its own start, clears the second word, and points the third word back at the block.

### Assign

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_table_block_and_call`: When the destination pointer on the stack is non-null, copies sixteen 32-bit words from a fixed table in the data area into it.

### EndCLI

- `launch_command_in_new_process`: Program entry: scans a list for the largest size value, then builds a 140-byte buffer and a new process block.
- `copy_startup_block_and_call`: When the destination pointer on the stack is non-null, copies sixteen 32-bit words from a fixed table into it.

### Echo

- `initialize_task_environment`: Traverses a linked list to find a maximum value and then populates a data structure with a sequence of incrementing values.
- `copy_stack_data`: Copies a block of sixteen bytes from a memory location to a destination on the stack before calling another routine.

### LoadWB

- `start_program_parse_arguments_2`: Program entry point: it opens the DOS library, allocates a state record, and splits the command-line text into words.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `start_workbench_tool_task`: Looks up a resident tag for the named tool and starts it as a task inside a forbid and permit pair.
- `compare_strings_ordered`: Compares two null-terminated strings and returns minus one when the first is less than the second, one when it is greater, and zero when they are equal.
- `build_linked_list_from_table`: Walks a table of entries, allocating an 8-byte node for each one and copying in a value obtained through a library call.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_132`: Calls a library function at a fixed negative offset from the library base held in a global variable, then returns.
- `call_library_function_offset_8a`: Calls a function located at a specific offset within a library structure.
- `call_indirect_function`: Fetches a function pointer from a location relative to a global variable and executes it.
- `call_indirect_function_2`: Fetches a function pointer from an offset within a global structure and executes it.
- `execute_indirect_function`: Fetches a function pointer from an offset within a global structure and executes it.
- `call_indirect_function_3`: Fetches a function pointer from a location within a global data structure and executes it.
- `call_offset_handler_1`: Calls a routine through a pointer stored at a specific offset from a global variable.
- `call_offset_handler_2`: Calls a routine through a pointer stored at a different offset from a global variable.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_offset_handler_3`: Calls a routine through a pointer stored at a third offset from a global variable.
- `call_offset_handler_4`: Calls a routine through a pointer stored at a fourth offset from a global variable.
- `check_release_name_suffix`: Compares the start of a name with a reference string, and when the version character equals '2' it rewrites the short suffix after it with a dot and spaces.
- `write_dot_padding_suffix`: Writes a dot followed by four spaces into five consecutive bytes of the buffer at a fixed offset.
- `match_reference_prefix`: Advances through the buffer while its bytes match the reference string, stopping at the end of the reference or at the first mismatch.
- `stub_return_only`: Contains only a return instruction, with no calls, memory access or hardware use, so it does no work.
- `reschedule_current_task`: Calls the operating system's reschedule function through the exec library base and returns.
- `dispatch_task_scheduler`: Calls the operating system's dispatch function through the exec library base and returns.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `allocate_memory_block`: Calls the operating system's memory allocation function through the exec library base and returns.
- `forward_with_stack_pointer_2`: Forwards its two arguments to a second routine together with a pointer to a fixed stack slot, then returns.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_to_pointer`: Stores a byte from a register through a pointer.

### Ram-Handler

- `copy_block_then_jump_target`: Program entry: copies a block of memory through the exec library's copy routine, then makes an indirect call to a jump target that the decompiler could not resolve.

### ramdrive.device

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `create_ramdrive_unit`: Opens the exec and DOS libraries, locates the device structure and sets up a new unit.
- `create_ramdrive_unit_from_params`: Copies a parameter block from the request into the global buffer, then sets up a unit the same way as the main routine.
- `copy_params_and_init_unit`: Copies a block of bytes from the source to the destination, then sets up a unit in the same way as the other setup routines.
- `initialize_ramdrive_device`: Stores pointers to the device's internal buffers and control fields and links the device into a global list.
- `call_library_vector_210`: Calls a library function at a fixed negative offset twice in a row, then returns zero.
- `find_fitting_memory_node`: Disables interrupts with a nesting count, then walks a linked list of memory nodes for one whose attribute flags include the requested mask and whose size is large enough.
- `find_fitting_node_and_allocate`: Walks a linked list of memory nodes whose attribute flags include the requested mask, recording a size that fits the request.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decrement_interrupt_nesting`: Decrements the interrupt-disable nesting count and re-enables interrupts when the count was already at its lowest level.
- `store_field_and_return_zero`: Stores the data-register value into a fixed field of the structure in the address register, then returns zero.
- `dispatch_device_request`: Forbids task switching, then dispatches the request to a handler picked from a 16-entry table by its command number and stores the result back in the request.
- `return_minus_three_error`: Returns the constant minus three as an error code without doing any other work.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 253 into d0, then returns.
- `remove_unit_and_free_memory`: Decrements the unit's use count and, when it drops below zero and the unit is still linked, unlinks it from the device list and the global list.
- `unlink_unit_and_append_colon`: Copies a stored value into the output location and removes a unit from the global list, then calls a library routine for it.
- `copy_value_and_append_colon`: Stores a value into the output location and calls a library routine.

### mathtrans.library

- `halt_execution`: This library routine is empty due to an unimplemented instruction.
- `store_argument_in_global`: Copies the address-register argument into a global variable slot and returns.
- `expunge_library_when_unused`: If the library's open count is zero, unlinks it from the library list and frees its memory through a library call, then returns a saved value.
- `set_expunge_pending_flag`: Sets the expunge-pending flag bit in the library structure and returns zero.
- `open_and_count_library_use`: Unless the library is marked for expunge, increments its open count and, on the first open, loads the companion mathffp.library through a library routine.
- `return_failure_minus_one`: Returns minus one, the library's failure result, and does nothing else.
- `close_and_expunge_idle_library`: Decrements the library's open count.
- `forward_call_no_arguments`: Calls another routine with no arguments and returns, as a thin wrapper.
- `compute_float_mantissa_bits`: Builds the mantissa bits of a floating-point value held in the data register through a loop of up to 23 shift-and-subtract steps.
- `divide_bits_with_rounding`: Sets one bit in the value from an index, then runs a shift-and-subtract loop over the given number of bit steps.
- `call_and_set_status_bits`: Calls a library routine, then clears bit 3 and sets bit 1 in its result before returning it.
- `run_fixed_float_op_sequence`: Calls a fixed sequence of entries through the library base at three fixed offsets, alternating between two of them and ending with a third.
- `run_float_op_sign_branch`: Runs a series of floating-point primitives through library offsets, with the path chosen by comparison results and an extra step when the input is negative.
- `run_float_op_three_branch`: Runs floating-point primitives through library offsets along one of three paths, chosen by comparison results and the sign of the input.
- `dispatch_float_sign_zero_path`: Checks a zero-test value and the sign of the input, then runs the matching sequence of floating-point primitives through library offsets.
- `dispatch_float_primitive_by_sign`: Runs one of a few floating-point primitive sequences chosen by a zero test and the sign of the input, then returns the status with bits masked off.
- `dispatch_float_sign_only`: Runs one floating-point primitive chosen by the sign of the input and a zero-test value, then returns the status with bits masked off.
- `run_float_primitive_masked_status`: Calls one floating-point primitive through the library base, then returns the status with bits masked off.
- `run_float_primitive_pair_masked`: Calls one floating-point primitive twice through the library base, then returns the status with bits masked off.
- `mask_return_value`: The library code calls a function through a pointer stored in a global variable twice, then calls another routine and returns its result with certain bits cleared.
- `invoke_indirect_pointer`: This library code calls a routine through a double pointer stored in a global variable.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `execute_indirect_call`: This library routine fetches a function pointer from a location determined by an offset from a global variable and executes it.
- `compute_float_quotient_bits`: Works on the exponent and mantissa bits of a floating-point value, checking for an out-of-range case first.
- `compute_shift_subtract_quotient`: Runs a shift-and-subtract loop of up to 24 steps over the input value to build its quotient bits.
- `run_shift_subtract_until_exhausted`: Runs a shift-and-subtract loop that stops when its step counter argument runs out.
- `compute_trig_angle_iteration`: Reduces an angle held in the data register by repeatedly adding or subtracting a fixed constant, then runs a 23-step shift-and-add iteration over a table of constants.
- `compute_trig_angle_range_checked`: Returns early for angles outside a set range, otherwise reduces the angle and runs the same 23-step iteration as its neighbours.
- `reduce_angle_and_accumulate`: Reduces the angle the same way as its neighbouring routines, then accumulates table constants over 23 steps.
- `compute_trig_angle_sign_check`: Returns early for small angles, reduces the angle and runs the 23-step iteration, then takes one of several output paths based on the sign and value of a stack argument.
- `compute_trig_angle_single_output`: Reduces the angle the same way as its neighbours, runs the 23-step iteration, then makes one output library call.
- `compute_trig_angle_signed_output`: Reduces the angle, runs the 23-step iteration, and outputs through library calls.
- `iterate_trig_constants_output`: Runs a 23-step shift-and-add iteration whose constants come from a register, then outputs the result through library calls.
- `run_trig_iteration_output`: Runs the 23-step shift-and-add iteration on the input angle and outputs the result through library calls.
- `run_table_shift_loop_output`: Runs a shift-and-add loop over a table for a count argument, then outputs the result through library calls.
- `stub_empty_return`: Does nothing and returns, so it is an empty routine.
- `recursive_call_wrapper`: The routine contains no instructions and immediately returns.
- `call_two_routines_in_sequence`: Calls two routines one after the other with no arguments, then returns.
- `normalise_float_value_bits`: Normalises a 32-bit value by shifting it left until its top bit is set, adjusting a 16-bit exponent from a fixed base.
- `normalise_shifted_value_bits`: Normalises a value held in the data register by shifting it left until its top bit is set, adjusting a 16-bit exponent from a fixed base.
- `call_then_conditional_pair`: Calls one routine, then calls two more in sequence when a flag stays false.
- `call_one_then_three_routines`: Calls one routine, then calls three more in sequence because a fixed flag stays false.
- `call_routines_with_incremented_byte`: When the input byte is non-zero and incrementing it does not overflow, calls one routine, then calls two routines with the incremented byte and one more routine.
- `empty_stub_2`: This library code is empty.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `execute_empty_stub`: This library routine is empty.
- `empty_stub_3`: The library code contains five instructions and performs no observable work; it ends immediately.
- `empty_routine`: The library code consists of four 68000 instructions and performs no observable work, immediately returning to its caller.
- `pack_float_exponent_and_sign`: Returns at once for a zero input.
- `call_four_routines_in_sequence`: Calls four routines one after another with no arguments, then returns.
- `accumulate_signed_table_sum`: Walks a table of 32-bit values for a count, adding each value when the accumulator is negative and subtracting it otherwise.
- `accumulate_table_until_negative`: Adds the first table entry to an accumulator, then subtracts successive entries while the accumulator stays non-negative.
- `encode_float_mantissa_exponent`: Returns early through a routine for a zero input.
- `return_void_2`: This is an empty routine that performs no operations and immediately returns.
- `encode_float_mantissa_scaled`: Calls two routines, then computes the mantissa with a shift-and-subtract loop of up to 22 steps.
- `run_mantissa_loop_with_scaling`: Runs a shift-and-subtract loop limited by a count argument, then scales the result by a biased exponent byte and calls output routines.
- `call_three_routines_in_sequence`: Calls three routines in succession, then returns.
- `round_and_pack_float_value`: For a non-negative input it calls two routines, works out a rounding decision from the exponent bits, and packs the result with its sign through further routine calls.
- `pack_float_after_calls`: Calls two routines, returns early for a zero input, then computes a shifted mantissa from the exponent bits and packs the value through routine calls, using the sign to fix up the result.
- `stub_empty_return_2`: Performs no operation and returns, so it is an empty library stub.
- `frag_swap_ror_eori_add_bvs_addq_rts`: Short code fragment that swaps the halves of d7, then rotates d7 right by 7, then exclusive-ors 128 into d7, then adds d7 to d7, then branches if the overflow flag is set, then adds 5 to d7, then branches if the overflow flag is set, then exclusive-ors 128 into d7, then rotates d7 right by 1, then returns.
- `return_immediately_4`: This library routine is empty and performs no operations before returning to the caller.
- `return_from_subroutine`: The routine executes a single return instruction to exit the current procedure.
- `empty_library_stub`: The library routine is empty.
- `return_from_subroutine_2`: This library routine is empty and performs no operations before returning.
- `stub_empty_return_3`: Does nothing and returns; the decompiled body is empty, so it is an empty library stub.
- `invoke_callback_and_return`: Calls the routine whose address is in the address register, then returns the data-register value it was given, unchanged.
- `call_indirect_return_d0`: The library code receives a function pointer in A0 and a value in D0, calls the routine pointed to, and then returns the value that was in D0 on entry.
- `forward_call_return_input`: Calls another routine with no arguments and then returns the data-register value it was given, unchanged.
- `invoke_callback_and_return_2`: This library routine calls the routine whose address is in the address register, then returns the data-register value it was given, unchanged.
- `execute_callback_and_return`: Invokes a routine via a pointer stored in an address register and returns the original value held in a data register.
- `execute_callback_and_return_2`: Calls a routine via a pointer stored in an address register and returns the original value of a data register unchanged.
- `invoke_callback_and_return_3`: This library routine calls a subroutine through a pointer in an address register and returns the original data register value unchanged.
- `execute_callback_and_return_3`: Executes a routine via a pointer stored in an address register and returns the original value of a data register unchanged.
- `execute_callback_and_return_4`: Invokes a callback routine via a pointer and returns the original data register value unchanged.
- `call_then_store_long`: Calls another routine, then stores the 32-bit data-register value into the memory address held in the data register pointer argument.
- `execute_callback_and_return_5`: Calls a routine via a pointer and returns the original value held in a data register.
- `forward_library_call_d0`: Entry stub that calls one other routine in this shared library and returns the D0 value that call produced unchanged.
- `execute_indirect_callback`: Calls a routine through a pointer provided in an address register and returns the original value of a data register unchanged.
- `execute_callback_and_return_6`: Executes a routine via a pointer provided in an address register and returns the original value of a data register unchanged.
- `execute_callback_and_return_7`: Executes a routine via a pointer and returns the original value held in a data register.
- `execute_callback_and_return_8`: Invokes a routine via a pointer and returns the original value held in a data register.
- `call_indirect_handler`: This library routine calls a function pointer provided in a register and returns the value held in that register.
- `execute_indirect_callback_2`: Calls a routine via a pointer retrieved from a data structure and returns the original value of a data register.
- `invoke_callback`: This library routine calls a subroutine via an address in an address register and returns the original value of a data register.
- `call_library_slot_return_d0`: Loads a global base pointer, adds a fixed negative offset to it to pick an entry in a table of library routines, calls that entry and returns D0.
- `call_indirect_with_offset_2`: This library routine calls a subroutine using an address calculated by adding an offset in an address register to a global pointer.
- `call_library_slot_return_d0d1`: Loads a global base pointer, adds a fixed negative offset to it to pick an entry in a table of library routines, calls that entry and returns the D0 and D1 pair as one 64-bit value.
- `invoke_callback_and_concat`: This library routine invokes a routine via a pointer stored in a global variable and returns the concatenation of two registers.
- `invoke_indirect_offset_routine`: This library code calls a routine via a pointer stored in a global variable plus an offset held in register A0.
- `execute_indirect_offset_call`: Calls a routine located at an offset from a global pointer and returns the result in a register.
- `invoke_indirect_callback`: Fetches a function pointer from a global variable with a fixed offset and executes it.
- `invoke_callback_and_concatenate_results`: Executes a routine via a pointer retrieved from a global variable and returns the concatenation of the resulting two 32-bit registers.
- `invoke_callback_and_concatenate`: Executes a routine via a pointer stored in a global variable and returns the concatenation of the resulting two 32-bit registers.
- `invoke_callback_and_return_concat`: The library code invokes another routine via a pointer stored in a global variable and then returns the concatenation of the two input values.
- `call_indirect_with_offset`: Calls a routine whose address is a global pointer plus the offset held in register A0, then returns the D0 value.
- `normalize_signed_long_value`: Normalises the signed 32-bit value in D6: a negative value is negated and its sign recorded, small magnitudes have their two 16-bit halves swapped, and the value is then doubled until its top bit is set.

### icon.library

- `return_minus_one_status`: Entry point of the library file that loads minus one into D0 and returns, doing no other work.
- `increment_count_clear_flag`: Adds one to a 16-bit counter at a fixed offset from the structure pointer in A6, then clears bit 3 of a flags byte at another offset in the same structure.
- `decrement_count_release_if_flagged`: Subtracts one from a 16-bit count in the structure.
- `unlink_node_or_mark_pending`: Tests a 16-bit field in the structure.
- `set_flag_bit_return_zero`: Sets bit 3 of a flags byte in the structure addressed by A6 and returns zero in D0.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `store_result_or_alert`: Stores two input values in the structure and calls a library routine whose result it keeps in the structure.
- `copy_two_longs_to_globals`: Copies two 32-bit values from fixed offsets of a structure addressed by A2 into two global variables.
- `tail_jump_to_library_routine`: Jumps straight to one other library routine, which does all the work; it adds nothing of its own.
- `execute_self_loop`: This library routine performs an unconditional jump to itself.
- `call_library_handler`: This library routine simply calls another routine and then returns.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `call_icon_init_routine`: This library routine invokes a single initialization function to set up icon data structures and then returns.
- `forward_library_call_void`: Calls one other routine in this library and then returns.
- `call_handler_pointer`: This library routine fetches a function pointer from a data structure and executes it.
- `call_incrementing_handler`: This library routine calls another routine with an incremented second parameter and then returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_icon_initialization`: This library routine calls a single initialization function to set up icon data structures and then returns.
- `dispatch_library_handler`: This library routine retrieves a handler pointer from a data structure and calls it.
- `invoke_handler_via_recursion`: This library routine calls another routine that performs a recursive call with an incremented parameter until a termination condition is met.
- `call_other_routine_2`: This library code calls another routine and then returns.
- `call_other_routine_3`: The routine calls another routine and then returns.
- `call_parse_copy_prefix_name`: This library routine calls another routine and then returns.
- `store_value_in_returned_record`: Calls a routine through a table in the structure given as its first argument, passing that argument, then stores its second argument as a 32-bit value at a fixed offset in the pointer that call returns.
- `chain_calls_release_on_failure`: Calls one routine to obtain a value, then passes that value to a second routine.
- `load_header_into_record`: Obtains a record from one routine and reads a second routine's output into a stack buffer.
- `copy_buffer_fields_into_record`: Copies ten longwords from a buffer addressed through A6 into the record in A2, then copies several more fields from the same buffer into fixed positions in the record.
- `return_zero_stub`: Does no work and returns zero in D0.
- `build_stack_header_and_forward`: Builds a header on the stack from a fixed 16-bit tag and the value one, copies ten longwords and several fields from the record into it, then passes the header with two arguments to another routine.
- `read_icon_header_and_sections`: Reads the structure of an icon file: it builds a path using the .info suffix, checks a 16-bit magic word and a version of one, then loads the optional sections and per-entry data into newly allocated memory.
- `read_icon_sections_from_registers`: Performs the same magic and version check and section loading as the full icon reader, but takes its record, callback and output pointers from address registers instead of arguments.
- `load_remaining_icon_sections`: Continues the icon loading sequence: it stores two values from a read into the record, then loads optional pointer tables or further blocks depending on which fields are set.
- `load_pointer_table_entries`: Loops over a count kept in the caller's frame, reading one 32-bit entry per pass through a routine and storing each result in a pointer array.
- `forward_call_return_zero`: Calls one other library routine, discards its result and returns zero.
- `empty_routine_2`: The routine contains four instructions and performs no observable work; it ends immediately with a return.
- `emit_icon_sections_with_lengths`: Allocates a 260-byte work buffer and writes an icon's optional sections in order, computing a length word and writing it just before each block.
- `emit_icon_sections_from_registers`: Writes the same section sequence as the buffer-based writer, but takes its inputs from address registers and allocates no work buffer.
- `emit_optional_sections_with_lengths`: Writes the optional sections of an icon, each block preceded by a computed length, including a null-terminated table of pointers.
- `emit_table_entries_then_block`: Walks a pointer table in the structure, writing the length and value of each entry through two routines, then writes the optional secondary entry.
- `emit_null_terminated_table_entries`: Walks a null-terminated table of pointers, writing a length word and each entry through two callbacks, then writes an optional secondary entry.
- `process_linked_list_nodes`: Walks a linked list from the node after the header until it returns to the header sentinel, passing each node to another routine.
- `walk_node_list_until_end`: Walks a chain of nodes starting from the value in D0, passing each node to one routine and fetching the next node from another.
- `increment_param_and_recurse`: Library code that calls another routine with the same first argument and the second argument increased by a constant, then returns.
- `allocate_and_init_record`: Allocates a 168-byte memory block, calls one routine on one of its sub-fields and then another routine to set up the block.
- `initialize_table_slot`: The library routine calls another routine and then returns a constant value of zero.
- `return_zero_constant_2`: This library routine returns a constant value of zero to its caller.
- `store_pair_in_table_slot`: Takes one slot from a counter, growing the table through a routine when no slots remain, then writes two 32-bit values into the next eight-byte entry of the table.
- `decrement_counter_store_pair`: Decrements a 16-bit counter through a pointer, then writes two 32-bit values from registers into the table entry selected by the new counter value.
- `allocate_and_init_block`: Allocates a 94-byte block and calls one routine on a sub-field, then another routine to set the block up.
- `return_without_work`: Contains no work and returns immediately without any result.
- `call_library_routine_and_return_zero`: Library code that calls another routine and then returns zero.
- `perform_incremental_recursion`: Library code that performs a recursive call with an incremented parameter until the callee stops the recursion.
- `find_text_after_equals_sign`: Scans a null-terminated list of entries, testing each one with a routine, and returns a pointer to the text after an equals sign in the entry that passes.
- `find_text_after_equals_sign_2`: Scans a null-terminated list of entries, testing each one with a routine, and returns a pointer to the text after an equals sign in the entry that passes.
- `find_equals_sign_offset`: Library code that iterates through a list of pointers to find an entry containing an equals sign and returns a pointer to the character immediately following it.
- `search_field_by_length`: The library routine iterates through a sequence of fields to find one that matches a specific length and satisfies a secondary condition via another routine.
- `find_matching_separated_field`: Walks the fields of a string separated by a fixed separator character, comparing each field's length with a reference length and then testing the field with a routine.
- `compare_separated_fields`: The library routine compares fields within a string by iterating through them and checking for specific length and content matches.
- `match_field_length_loop`: The library routine performs a loop to check field lengths against a target value using a series of library calls.
- `match_separated_field_length`: Library code that iterates through fields in a string to check if any field matches a specific length and is followed by a null terminator.
- `parse_copy_prefix_name`: Reads a name that may begin with a copy prefix and takes the copy number from the digits after it.
- `parse_decimal_copy_number`: Checks a three-character prefix, then reads a decimal copy number from the text after it.
- `parse_decimal_until_marker`: Reads decimal digits from the text until a non-digit is reached, then checks the four characters that follow for a marker.
- `parse_decimal_copy_count`: Accumulates decimal digits from the text into a count, stopping at the first non-digit, then checks the marker text that follows.
- `convert_decimal_digit_char`: Returns the numeric value of a character when it is an ASCII decimal digit, and minus one for any other character.
- `return_minus_one_status_2`: Entry point that loads minus one into D0 and returns, doing no other work.
- `call_library_slot_void`: Calls an entry in a library table through a fixed offset from a global base pointer and returns no value.
- `jump_to_library_function`: Library code that retrieves a function pointer from a table and jumps to it.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: The library routine forwards a set of four parameters to another routine, checks the returned value, and if non-zero calls the same routine again with a modified argument list.
- `allocate_block_with_count_ten`: Allocates a 96-byte block, sets a 16-bit field inside it, stores ten in a 16-bit value through the caller's pointer, and then calls a routine to initialise the block.
- `return_zero`: This library code returns zero.
- `verify_call_result_equals_count`: Calls one library routine with three arguments and returns its result when that result equals the last argument; otherwise it returns zero.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `validate_parameter_match`: The library routine calls another routine with three parameters and checks if the returned value matches a fourth parameter, returning zero if they differ.
- `return_immediately_2`: This library code returns immediately without performing any operations.
- `divide_long_by_word`: Divides the 32-bit dividend in D1 by the divisor in D2 and returns the remainder and quotient as a 64-bit pair.
- `shift_subtract_divide_loop`: Runs sixteen shift-and-subtract steps that divide the 32-bit value in D1 by the 16-bit divisor in D2, keeping the working quotient and remainder in local values.
- `multiply_low_32_bits`: Multiplies the values in D0 and D1 using 16-bit partial products and returns the low 32 bits of the product as the high half of a 64-bit result.
- `call_format_library_routine`: Calls a routine through a fixed offset from a global base pointer and returns no value.
- `store_byte_at_pointer_2`: Writes the low byte of D0 to the memory location addressed by A3 and returns.
- `count_c_string_length`: Counts the characters before the zero terminator of a string and returns the count.
- `find_char_in_c_string`: Scans a zero-terminated string for a given character and returns a pointer to its first occurrence, or zero when the terminator comes first.
- `return_zero_2`: Library code that returns zero.
- `copy_c_string`: Copies bytes from the source string to the destination, including the zero terminator, and returns no value.
- `append_c_string`: Moves to the end of the destination string, then copies the source string onto it including its zero terminator.
- `strings_differ_within_limit`: Compares two strings up to a length limit and returns one at the first differing character.
- `return_one_stub`: Does no work and returns one in D0.
- `walk_string_within_limit`: Advances through a string, counting down a limit, until the zero terminator or the limit is reached.
- `perform_empty_call`: Library code that calls another routine and returns.
- `copy_c_string_padded`: Copies up to a given number of characters from the source string to the destination, stopping after the zero terminator, then pads the rest of the limit with zero bytes.
- `return_immediately_3`: Library code that returns immediately.
- `init_empty_list_header`: Sets up a list header inside the block it is given.
- `validate_and_process_request`: Returns one when given no request.
- `process_linked_entries_checked`: Walks a linked list of entries, reading each entry's header through one routine and then processing its data block through another, with the size taken from a byte field.
- `walk_entries_until_failure`: Repeatedly runs two routines on each entry of a linked list, moving on through the link field.
- `return_zero_constant`: This library routine returns a constant zero value to the caller.
- `build_entry_from_stream`: Allocates a 20-byte entry and fills it through two routines: one reads the entry's header, and the other reads a value and stores it in the entry's field at offset 10 before checking it.
- `return_negative_one`: This library code returns a value of all ones.
- `build_entry_list_from_count`: Builds a chain of entries, one per pass, until the requested count runs out.
- `build_entry_chain_to_end`: Builds a chain of entries using the same steps as the counted builder, stopping when an entry's link field is zero.
- `build_entry_chain_loop`: Builds a chain of entries from a sequence of reads, linking each new entry to the previous one, and stops when an entry's link field is zero.
- `return_all_ones`: This routine returns a value consisting of all bits set to one.

### diskfont.library

- `allocate_store_and_finish`: Calls a library routine to allocate a block and, when one is obtained, stores the argument at offset 34 of the block and calls a second library routine on it.
- `check_and_set_font_handle`: The routine is library code that calls a routine via a table entry using the library base.
- `open_two_libraries_init_list`: Opens two system libraries whose names the text references.
- `close_library_free_structure`: Closes a system library through a library routine, then frees the structure through the memory-free routine, and returns zero.
- `release_idle_font_entries`: Walks a linked list of entries, unlinking each entry whose use count has dropped to zero, unloading its segment through a DOS routine, and reducing the structure's count for each one.
- `close_libraries_free_when_idle`: When no entries remain in use, closes two libraries, frees the structure and returns a saved value.
- `set_flag_bit_return_zero_2`: Sets the top bit of a flags byte in the structure and returns zero.
- `return_void`: This routine is empty and performs no operations before returning.
- `clear_flag_increment_count`: Clears the top bit of a flags byte in the structure and increments a 16-bit count at another offset in it.
- `decrement_count_finish_if_flagged`: Decrements the 16-bit count in the structure.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `collect_font_names_into_buffer`: Fills a buffer with font entries: first from an in-memory list of names and attributes, then, when the file flag is set, from entries read off disk that carry a fixed header marker.
- `copy_font_directory_entries`: Iterates through a linked list of directory entries to calculate string lengths and copy file names and associated metadata into a destination buffer.
- `parse_font_directory_entries`: Iterates through a linked list of directory entries to extract file names and associated attributes.
- `scan_fonts_directory_into_buffer`: Locks and examines the FONTS: directory on disk, copying each entry's name character by character into the output buffer along with its attributes.
- `collect_font_files_from_directory`: Walks a directory listing, selecting names that contain a dot and end with the font suffix and whose header marker matches.
- `store_font_directory_entries`: Copies the names of matching font entries from a directory listing into the output buffer and records their attributes.
- `read_font_file_entries`: Reads a font file through a read routine into a block, copying name characters and the attributes of matching entries into the output buffer.
- `scan_font_directory_with_cleanup`: Walks a directory with examine-next and read calls, copying names and attributes of matching font entries into the output buffer.
- `return_zero_stub_2`: Library code that returns zero and performs no operations on registers, memory, or hardware.
- `score_font_match_distance`: Compares two font descriptors by a size difference and by seven flag bits in each attribute byte, subtracting a weight from a running score for every mismatched flag.
- `score_font_flag_mismatch`: Starts from a size-based value and subtracts a weight for each mismatched bit across seven flag positions in two attribute bytes.
- `score_font_flag_mismatch_2`: Starts from a value taken from the flags of the descriptor and subtracts a weight for each mismatched bit across the flag positions in two attribute bytes.
- `calculate_mismatched_bit_weight`: Library code that subtracts values from a starting total based on bitwise comparisons between two attribute bytes across seven positions.
- `return_fixed_sentinel_value`: Returns the constant minus 32767 in D0 and reads no inputs.
- `select_best_scoring_font_entry`: Looks for an in-memory font matching the requested descriptor.
- `select_best_scoring_font_entry_2`: Locks the FONTS: directory and reads its font file entries, repeatedly choosing the candidate that scores best against the request and adding it to the font list.
- `select_best_scoring_font_entry_3`: Reads candidate font entries from disk and repeatedly chooses the one that scores best against the request, adding each chosen entry to the font list.
- `add_best_scoring_font_entries`: Repeatedly picks the highest-scoring candidate, adds it to the font list and moves on until no candidates remain.
- `load_and_register_font_entries`: Walks candidate font entries, choosing the best-scoring one on each pass, loading its segment with a DOS routine and registering it through the list-add and font-add routines.
- `unload_and_add_best_entries`: Repeatedly picks the highest-scoring candidate, unloads the segment of the previously chosen entry through a DOS routine, and adds the new entry to the font list.
- `add_best_entries_release_locks`: Repeatedly picks the highest-scoring candidate and adds it to the font list.
- `build_font_file_table`: Locks a directory named by the request and counts its entries that carry the .font suffix.
- `build_disk_font_table`: Scans a font directory, allocating a table with a header and a 16-bit count, then copies each matching font's name and metric bytes into the table.
- `free_stack_slot_block`: Passes the value stored four bytes below its argument, together with that slot's address, to a memory-free routine.
- `forbid_task_switching`: Calls the system routine that disables task switching and returns no value.
- `permit_task_switching`: Calls the system routine that re-enables task switching and returns no value.
- `allocate_memory_via_library`: Calls the system memory-allocation routine through a fixed offset from a global base pointer.
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
- `copy_null_terminated_string_2`: Library code that copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `seek_terminator_then_call`: Advances to the zero terminator of the string and passes that position, with the second argument, to another routine.
- `seek_terminator_then_call_2`: Advances to the zero terminator of the string in A0 and passes that pointer to another routine.
- `count_c_string_length_2`: Counts the characters before the zero terminator of a string and returns the count.
- `accumulate_length_until_zero`: Adds one to the value in D0 for each byte read from the string, including the zero terminator, and returns that total.
- `compare_c_strings`: Compares two zero-terminated strings byte by byte.
- `compare_c_strings_reversed`: Compares two zero-terminated strings byte by byte.
- `compare_two_strings`: Library code that compares two zero-terminated strings byte by byte and returns a result based on their equality or lexicographical order.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `find_character_in_string`: Library code that scans a null-terminated string for a specific character and returns a pointer to its first occurrence or zero if not found.
- `find_char_in_c_string_2`: Scans forward from the pointer in A0 for the character in D0 and returns a pointer to the first match.
- `find_char_after_first_byte`: Starts one byte past the pointer it is given and scans forward for the character in D0.
- `find_last_char_in_string`: Finds the last occurrence of a character in a string by measuring its length and scanning backward from the end.
- `find_last_char_in_string_2`: Scans backward from the pointer in A0 toward a lower bound for the character in D2.
- `find_last_char_in_string_3`: Steps backward from the pointer in A0, checking each position against the character in D2.
- `skip_past_path_separator`: Returns the position just after the first slash or colon in the string, or the original pointer when neither occurs.
- `increment_value_in_d0`: Adds one to the value in D0 and returns it.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_unknown_handler`: The routine is library code that invokes another unnamed function and returns.
- `call_external_handler`: The routine is library code that calls another unnamed routine and returns.
- `call_other_routine`: The routine is library code that calls another routine and then returns.
- `call_another_routine`: The routine is library code that acts as a thin wrapper which immediately invokes another routine and then returns without performing any additional operations.

### Defender (1994)(Ratsoft).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
