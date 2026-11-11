# Game 1484 (defender (1994)(ratsoft)[t +10 apocalypse]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01484). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 22 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 467 of 467 named (0 author symbols), 467 of 467 described (71 by rule, 246 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 335 of 335 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 11913 of 11913 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 271552 |
| code | 44032 |
| data | 91867 |
| padding | 5733 |
| unused | 487936 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Defender (1994)(Ratsoft)[t +10 Apocalypse].adf:Defender | 6 | 5018 | 51 |
| More | 19 | 2290 | 76 |
| Assign | 2 | 117 | 2 |
| EndCLI | 2 | 117 | 2 |
| Echo | 2 | 117 | 2 |
| LoadWB | 12 | 537 | 30 |
| Ram-Handler | 2 | 10 | 1 |
| Defender (1994)(Ratsoft)[t +10 Apocalypse].adf:Apc-Trainer | 1 | 99 | 3 |
| ramdrive.device | 3 | 345 | 17 |
| mathtrans.library | 18 | 830 | 97 |
| icon.library | 10 | 1375 | 109 |
| diskfont.library | 15 | 1040 | 76 |
| Defender (1994)(Ratsoft)[t +10 Apocalypse].adf:bootblock | 1 | 18 | 1 |

## Cases handled

- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4EB8) A7=an address (A7)=an address, slid to exec.library -an address): 490 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4

## Cracktros (kept, not protection)

- Defender (1994)(Ratsoft)[t +10 Apocalypse].adf:Apc-Trainer

## Copy protection found

- `finish_and_exit_program` (More): Calls a cleanup routine and another routine, then invokes an exit routine supplied through a register. Kind: sabotage path / exit routine. Pass path: not determined.

## Functions


### Defender (1994)(Ratsoft)[t +10 Apocalypse].adf:Defender

- `initialize_game_loop`: Clears a global flag, calls a routine via a table, checks a field in the returned pointer, and conditionally calls two more table routines to set the flag.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `decrypt_and_loop_game_logic`: Performs a XOR-based decryption on a block of memory and enters a main loop that manages game state through various conditional branches and function pointer calls.
- `program_blitter_operation_2`: Configures the blitter hardware registers to perform block transfers and updates various global state variables and counters.
- `process_nibble_packed_data`: Iterates through a table of bytes, extracting high and low nibbles to accumulate values into two global variables.
- `render_animated_text_glyphs`: Reads a 16-bit index table from a global table until a sentinel value is reached.
- `update_audio_queue`: Manages a software sound queue by decrementing per-channel counters in a table and updating the audio DMA control and channel registers.
- `manage_audio_sample_buffer`: Iterates through a table of audio data to decrement sample counters and manage a software sound queue.
- `next_wave`: Updates a global timer and, upon expiration, reads an enemy type from a level table using a position index.
- `drawbobs`: The routine walks a linked list of bob descriptors stored in a global table.
- `drawscan`: The routine waits for a hardware flag, then writes several control words and pointers into a blitter control block to configure a scanline copy.
- `draw_icons`: Iterates over a global enemy list and, for each active entry, computes a screen offset from map coordinates, selects a byte from a scanbit table and from four icon tables, shifts it and ORs it into six bitplane buffers and sets corresponding collision bytes.
- `draw_map_scroll`: When a global space flag is zero, this routine programs the blitter registers for a copy operation and loops fifteen times to calculate source and destination addresses from a map table and a map y table, triggering the blitter for each entry.
- `update_sprite_animation_table`: Iterates through a table of entries to update sprite frame indices and calculate animation offsets.
- `update_ship_physics_and_graphics`: Updates a ship's position, velocity, and thrust based on control flags, clamps the values, and computes a map offset and image index.
- `update_input_control_state`: Reads a 16-bit value from a memory location offset from a pointer to determine direction and control flags.
- `update_input_and_state`: Tests bytes in a keypress table to update a control byte and several global variables.
- `update_object_animation_and_slots`: Updates a per-entry counter and flags in a table using an index, conditionally invoking another routine to draw bits.
- `draw_bits`: Iterates over a set of rows to compute screen offsets and writes masked bits from a source table into three bit-plane buffers by OR-ing them into two consecutive words per row.
- `update_particle_effects`: Iterates through a table of active entities to update their positions and handle lifecycle management.
- `add_life`: Waits for a hardware flag to clear, then if a lives counter is below a limit it programs a blitter operation by writing to several custom chip registers and sets up source and destination pointers from a global variable and a table indexed by a play-off value and the lives counter.
- `program_blitter_operation`: Waits for a hardware flag to clear and, if a global variable is below a certain limit, programs several blitter registers to perform a block transfer.
- `take_life`: Checks a lives counter and, if it is between one and five, waits for the blitter to be idle twice, programs the blitter control and mask registers and the blitter source and destination pointers, then waits for the blitter again and copies a byte from a table indexed by a playoff value and the lives counter to a destination pointer, and finally sets a blitter size register.
- `program_blitter_mask`: If a global variable is between one and five, waits for the blitter to be idle and programs the blitter control and mask registers.
- `score_draw`: Checks a global variable against itself and calls another routine if they differ.
- `render_nibble_pair_rows`: Splits a byte into high and low nibbles, mapping a zero nibble to ten, and writes eight rows of table-indexed values into two buffers forty bytes apart, duplicating each value into the adjacent byte.
- `dispatch_copper_and_vblank_interrupts`: Ands the interrupt enable and request registers and calls one handler when the copper bit is set and another when the vertical blank bit is set.
- `acknowledge_vblank_interrupt`: Writes the vertical blank bit to the interrupt request register to clear that pending interrupt.
- `run_frame_tick_on_flag`: When a pending flag is set, clears it and advances an animation counter and a scroll value, calls drawing and game logic routines, and counts down a timer that spawns a new table entry when it expires.
- `handle_keyboard_serial_byte`: When a pending bit is set, reads one keyboard serial byte from the CIA-A data register, stores its state as zero or all-ones in a table, and waits for the handshake bit before resetting.
- `update_object_table_by_type`: Walks a table of object entries until an end marker, dispatching on each entry's type code with special cases for a few types, advances horizontal positions with masks, and reloads spawn timers when an entry is consumed.
- `add_packed_decimal_score`: Adds packed decimal digits into a multi-byte score one byte at a time, applying decimal adjustment after each step and carrying between bytes.
- `claim_free_slot_for_entry`: When a slot index is valid, a fixed sprite-style check passes and a position falls inside a window, it stores a position word in a table entry, adjusts it by fixed offsets, writes a second value and a parallel field, and clears the slot index to mark it used.
- `update_and_draw_entries`: Walks a fixed set of entries, decrements each timer and frees expired ones, then for live entries in view ORs shifted masks into two plane buffers and raises a hit flag when a position falls in a range.
- `spawn_projectile_entry`: When a slot index is set and a horizontal offset is inside a window, fills a projectile entry with a position plus a fixed offset, zeroes two fields, sets a vertical velocity from a scaled difference, and sets two timing values, with different timing for one type.
- `advance_and_draw_projectiles`: Walks the projectile list, advances each position, frees entries that leave the view, raises a hit flag on a range match, and ORs pixel masks into one of two plane buffers chosen by a global.
- `toggle_double_buffer_pointers`: Flips a one-bit buffer flag and reloads the active and back bitplane pointers, storing the outgoing pointers into backup slots, then calls a routine.
- `start_full_screen_blit`: Waits for the blitter to be idle, programs the first and last word masks, modulo, source and destination pointers, and a fixed block size, then starts the blit.
- `run_six_setup_steps`: Calls six routine routines in sequence, one of which points the copper at its list, with no logic of its own.
- `store_default_palette_entries`: Writes fixed constant word pairs into two groups of globals that look like palette and tile palette entries, then returns.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `install_interrupt_handler`: Stores a handler address in an interrupt vector global, then writes an enable mask to the interrupt enable register to turn on the copper, vertical blank and ports interrupts.
- `configure_cia_serial_interrupt`: Stores a handler address, enables the CIA-A serial interrupt, sets a timer count, clears the serial output mode bit, then masks a byte in a global.
- `call_subroutines`: Calls two other routines in succession.
- `save_system_display_state`: Calls a routine, then stores a graphics structure pointer, two copies of a global state word, and the interrupt enable register value into globals for later restoration.
- `disable_all_dma_and_interrupts`: Writes the all-clear mask to the DMA control and interrupt enable registers, turning off every DMA channel and interrupt source.
- `restore_system_state_and_close`: Restores the saved DMA, interrupt and copper pointer values, waits until a flag bit in the CIA port register reads set, then closes a library.
- `call_close_library`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `poll_until_nonzero_result`: Calls a routine through a function pointer held in a global until it returns a nonzero value, then stores that value in a global.
- `poll_alternate_result_until_nonzero`: Calls a routine through a function pointer held in a global until it returns a nonzero value, storing that result in a second global.
- `call_stored_routine_pointer`: Calls a routine through a function pointer held in a global, once, and returns.

### More

- `allocate_linked_command_record`: Allocates a record through a library call, stores two arguments into it and links it into a list, then reads two library values.
- `replace_task_record_and_release`: Stores a new record pointer in a per-task slot and releases the old record, calling a cleanup routine for each of the old record's fields that is set, then returns the new pointer.
- `setup_command_context_from_args`: Allocates a context block, checks the first argument for a question-mark usage request, fills the context with pointers from library calls, then either runs the main action or reports usage.
- `apply_field_to_four_slots`: Loops over four global slots and calls one routine for each, passing a field from the input structure and that slot's value.
- `scan_buffer_for_control_bytes`: Calls a routine twice to prepare an eighty-byte buffer, then scans it and returns one if a byte falls in a range that looks like control codes, otherwise zero.
- `dispatch_pager_key_command`: Waits for an input character while watching for a break signal, maps some key codes to zero, and dispatches the remaining codes to pager command handlers, returning a status value.
- `finish_and_exit_program`: Calls a cleanup routine and another routine, then invokes an exit routine supplied through a register.
- `run_viewer_page_loop`: Sets up a viewer state structure with routine calls, then repeatedly calls a page routine while the current position is before the end of the buffer and no stop flag is set.
- `advance_viewer_line_count`: Reads a buffer through two routine calls, stores the second result in the viewer state, and increments the current line counter when a link field is clear.
- `read_next_text_line`: Reads up to 128 bytes into a buffer, scans for a newline or form-feed character to find the line length, seeks past the terminator, and sets a flag when a form feed was found.
- `display_page_lines_until_quit`: Chooses how many lines to show from a flag and a screen-size field, then loops displaying lines and reading keys until the key read is the q character.
- `search_text_forward_loop`: Repeatedly advances through the text and tests each step with a routine until a match is reported, returning one when found and zero when the count runs out.
- `call_with_setup_and_teardown`: Calls setup and teardown routines around a main call whose arguments come from a parameter block and a global, optionally calls an extra routine when a flag is set, and returns the main call's result.
- `call_routine_with_optional_followup`: Calls a routine with two arguments, then calls a second routine with the first argument only when a state flag is clear.
- `refresh_status_line_at_position`: Calls a drawing routine with a state field and a fixed negative value, then calls a second routine with the current line index from the viewer state.
- `redraw_last_visible_line`: Calls a drawing routine with a fixed value, then calls a second routine with the viewer's last line index minus one.
- `print_text_lines_to_limit`: Walks a text buffer up to a limit of 128 characters, counting lines and calling output routines for each segment, and stops at a terminating character or when the line count is reached.
- `call_display_routine_with_buffer`: Calls a routine with one argument, then calls a second routine with a buffer pointer from the state structure and a global.
- `reset_line_index_if_clear`: When a mode field in the parameter block is zero, resets the viewer's current line index to one, then calls a routine with a buffer pointer.
- `call_routine_with_line_pointer`: Calls a routine with a buffer pointer and a second argument that is the viewer's current line index plus one when a flag is set, otherwise a global address.
- `forward_line_or_global_args`: Calls a routine with a global address repeated three times when a flag is clear, or with the viewer's current line index when the flag is set.
- `clamp_second_result_to_zero`: Calls a routine twice with the same first argument and different seek-style third arguments, then returns the second result clamped to zero when negative.
- `copy_from_read_buffer`: Copies a requested number of bytes from a read buffer into the destination, refilling the buffer through a routine when it runs short, then advances the buffer offset and position fields and reduces the remaining count.
- `refill_read_buffer_block`: Moves any unread bytes to the front of the read buffer and reads a fixed block of 4096 bytes through a library call, updating the buffer size and position fields.
- `scroll_window_to_target_line`: Computes a target position from a mode argument, either the window start or the last line, and adjusts the window position while checking it against the buffer bounds.
- `read_line_into_buffer`: Reads bytes one at a time with a routine into a buffer until a newline or a limit of about 127 bytes, terminates the string with a zero byte and returns the count read.
- `copy_null_terminated_string`: Copies bytes from a source string into a destination, including the terminating zero byte.
- `uppercase_string_in_place`: Converts each lowercase ASCII letter in a null-terminated string to uppercase, in place.
- `parse_signed_decimal_string`: Parses an optional plus or minus sign followed by decimal digits from a null-terminated string and returns the signed integer value.
- `signal_bound_task_if_present`: Calls a library function with a bound task pointer and a full signal mask when the structure holds a non-null task pointer and the argument is non-null.
- `output_text_entry_with_newline`: Outputs the linked parts of a structure recursively, then the text argument or a stored string, and when a flag is set reads from the output until a newline character is returned.
- `send_one_word_request`: Stores one argument in a stack buffer and passes it with a fixed request code and a length of one to a library call, returning that call's result.
- `request_block_and_copy_fields`: Allocates a 36-byte block, sends its size in longwords with a fixed request code, copies two fields from the block into the output structure and frees the block.
- `allocate_linked_record_from_source`: Allocates a 68-byte record, links its self pointers and its template and source fields, then copies a count of words from the source address into the record.
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
- `forward_call_with_stack_slot`: Calls a routine with its two parameters plus the address of a stack argument slot.
- `read_single_character`: Calls a routine to read into a four-byte stack buffer, returning the character when exactly one byte was read and minus one otherwise.
- `print_stack_buffer_string`: Calls a library function, then measures the null-terminated text in a 140-byte stack buffer and passes the text and its length to a routine when it is not empty.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `divide_unsigned_longword`: Performs unsigned 32-bit division by shift and subtract, returning the quotient and remainder, with a separate path when the divisor fits in 16 bits.
- `multiply_longwords_partial_products`: Multiplies two 32-bit values using 16-bit partial products and returns the low 32 bits of the product.
- `stub_bad_instruction_data`: Decoded from data rather than real code: the routine calls itself without any condition and is not a working routine.
- `negate_result_when_flag_set`: Calls a routine and returns its first result, negated when the routine's second result is nonzero.
- `invoke_stored_library_vector`: Calls a library function whose address is read from a fixed offset in a global base pointer, with no arguments.
- `store_byte_through_pointer`: Writes the low byte of the input register to the memory address held in the address register.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `allocate_and_fill_message_block`: Allocates a 34-byte message block, fills in a pointer argument, a parameter byte and a type code, sends it through a library call and returns the result, freeing the block on failure.
- `release_message_port`: Removes the entry from its list when it is linked, marks its status and identifier fields as deleted, releases its signal bit and frees the 34-byte block.
- `search_text_for_pattern`: Searches the first text for the second string, giving up at a newline; returns one when the pattern is found and zero otherwise.
- `initialize_list_header`: Sets up an empty list header in a structure by pointing its first word to itself, clearing the next word and setting the tail pointer to the header itself.

### Assign

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longwords_and_call`: If a destination pointer is given, copies sixteen longwords from a fixed offset of a base into it, then calls a stored handler.

### EndCLI

- `launch_command_process`: Builds a launch record and tag list on the stack, copies sixteen longwords of setup data into it, stores a handler pointer, and calls library routines to create and start the process.
- `copy_data_and_call_function`: Copies sixteen bytes from a memory location to a stack-based destination and then calls a function pointer.

### Echo

- `initialize_dynamic_object_table`: Traverses a linked list to find the maximum value of a field, then allocates and initializes a new table entry.
- `copy_stack_data`: Copies a sequence of longwords from a source pointer to a destination pointer on the stack and then calls another routine.

### LoadWB

- `run_command_line_as_process`: Allocates a context record, then either takes the current shell buffer or the command line, splits it into arguments at spaces and tabs, and calls library routines to run the command.
- `store_argument_in_task_slot`: Retrieves a pointer from a global structure, stores the argument in a slot of the structure it points to, and calls cleanup routines for previously stored values that are set before returning the new slot value.
- `run_loadwb_setup_with_buffer`: Checks a library lookup, allocates a 162-byte buffer, passes a global string through a pair of XOR loops, calls several library functions and handles the result before returning.
- `compare_strings_ordered`: Compares two null-terminated strings byte by byte and returns minus one, one or zero for less, greater or equal.
- `build_node_list_from_table`: For each entry of a pointer table, allocates a small node, copies a value from the table into it and links the nodes into a new list, returning the head node.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_function_1`: Calls a function pointer retrieved from a global table to execute a library routine.
- `call_library_function_via_offset`: Fetches a function pointer from a global table using a specific offset and executes it.
- `execute_indirect_function_pointer_2`: Fetches a function pointer from a location relative to a global variable and executes it.
- `call_library_function_via_offset_2`: Fetches a function pointer from a global table using a specific offset and executes it.
- `call_offset_function_pointer`: Fetches a function pointer from a global table using a specific offset and executes it.
- `call_library_function`: Fetches a function pointer from an offset within a global structure and executes it.
- `call_library_function_2`: Fetches a function pointer from an offset within a library base address and executes it.
- `call_indirect_pointer`: Calls a routine through a pointer stored at an offset from a global variable.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_indirect_pointer_2`: Calls a routine through a pointer stored at a different offset from a global variable.
- `invoke_library_vector_624`: Calls a library function at a fixed offset from the base held in a global variable, with no arguments, and returns.
- `pad_matching_name_buffer`: Compares a global string with a fixed-position buffer and, if the global string matches its start, writes a dot and four spaces into the buffer at a fixed offset.
- `write_dot_and_spaces_suffix`: Writes a dot followed by four spaces into a buffer at a fixed offset.
- `match_global_prefix`: Compares a global string with the start of a buffer, stopping at the end of the global string or the first mismatch.
- `empty_routine_4`: The routine contains four instructions and performs no observable operation; it ends with a return.
- `call_reschedule_2`: Calls a function pointer retrieved from a global table to trigger a system reschedule.
- `call_dispatch_2`: Calls a function pointer retrieved from a global table to perform a dispatch operation.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem_2`: Calls a function pointer retrieved from a global table to allocate memory.
- `call_other_routine_with_stack`: The routine calls another routine, passing the two incoming parameters and a pointer to a local stack slot.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_at_address`: This routine stores a byte from a register into the memory location pointed to by another register.

### Ram-Handler

- `execute_jump_table`: Calls a routine through a pointer found in a jump table and then executes an indirect jump.

### Defender (1994)(Ratsoft)[t +10 Apocalypse].adf:Apc-Trainer

- `set_background_color_entry`: Writes to the background colour register and calls four routines.
- `store_low_byte_and_halve`: Stores the low byte of a value read from memory into a destination word, then returns the value shifted right by one with its top bit set.
- `shift_bits_from_stream`: Shifts a bit buffer right one bit per iteration for a count, refilling the buffer through a routine whenever it becomes zero.

### ramdrive.device

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `find_device_entry_by_name`: Looks up a device entry in a linked list by comparing a name string, then copies the matched entry's name and up to sixteen longword fields into shared storage.
- `copy_device_entry_fields`: Copies a name string from a record into a shared buffer, then copies up to sixteen longwords from the record's list into a second buffer.
- `allocate_ramdisk_buffers`: Copies a block of longwords, computes the disk size from block bounds, allocates a memory buffer for it and stores pointers to it, clearing them when allocation fails.
- `initialize_device_unit_and_link`: Fills in the unit's internal pointers, links the unit at the head of a device list while interrupts are disabled, calls a library function and re-enables interrupts when the nesting counter drops.
- `repeat_library_vector_call_twice`: Calls a library function at a fixed offset from a base register twice, then returns zero.
- `disable_interrupts_and_find_entry`: Writes the interrupt-enable clear mask, increments a nesting counter and walks a linked list until it reaches an entry that meets its condition.
- `release_interrupt_nesting`: Decrements a nesting counter and re-enables interrupts with the enable mask when it reaches zero; a routine is called first when a flag is set.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decrement_interrupt_nesting`: Decrements a nesting counter and, when it drops below one, writes the enable mask to the interrupt register to re-enable interrupts.
- `store_value_in_device_field`: Stores its argument into a fixed field of the structure in the address register and returns zero.
- `dispatch_request_through_table`: Disables multitasking, reads a request code from the message, calls the handler selected from a sixteen-entry table, stores its result in the message and then calls a library routine to finish.
- `return_error_code_minus_three`: Returns the constant minus three as an error result.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 253 into d0, then returns.
- `process_request_queue_entry`: Increments an activity counter, decrements a countdown field in the request and, when it goes negative with a link present, processes the request through routine calls.
- `unlink_device_unit`: Copies a field to the output pointer, removes the given unit from a singly linked list of units, then calls a library function.
- `append_colon_to_name_string`: Calls a library function, finds the end of a null-terminated name, writes a colon over its terminator and then calls another library function.

### mathtrans.library

- `return_empty_routine`: This routine is empty and performs no operations.
- `store_argument_in_global`: Stores its register input into a global variable and returns.
- `remove_node_and_release_block`: If the node's pending flag is clear, unlinks it from its list and calls a library release function, returning that result; otherwise marks the node pending and returns zero.
- `mark_node_pending_removal`: Sets a pending-removal bit in a node's flag byte and returns zero.
- `acquire_node_use`: Increments a node's use count unless it is pending removal, initialises its values on first use, and returns the node; returns minus one when it is pending removal.
- `return_minus_one_error`: Returns minus one as a fixed failure result.
- `release_node_use`: Decrements a node's use count, removing it through a routine when the count reaches zero and the pending-removal flag is set, and returns that result or zero.
- `forward_to_math_routine`: Calls one math routine through a library vector and returns.
- `process_single_float_mantissa_loop`: Returns early for zero and forwards negative inputs to a routine; otherwise it runs a loop of twenty-three steps over a mantissa-sized value, the width of a single-precision fraction.
- `divide_by_restoring_bit_loop`: Repeatedly sets one result bit selected by a count and subtracts it from a shifted remainder, a restoring division loop that runs for the count's width.
- `adjust_status_word_flags`: Calls a library routine and adjusts its status word by clearing one flag bit and setting another, then returns the result.
- `run_math_routine_sequence`: Calls a fixed sequence of nine library routines from one base, alternating between two table entries and finishing with a third.
- `check_operand_flags_and_dispatch`: Calls two library routines that set comparison flags and, when both flags match the expected pattern, calls a further routine and returns its result.
- `check_second_flags_and_dispatch`: Calls two library routines that set comparison flags and, when both flags match the expected pattern, calls a further routine and returns its result, using the second flag variable.
- `finish_math_operation_with_flags`: Calls a library routine to test flags, conditionally calls two more routines, then checks an operand value before returning a result.
- `dispatch_by_sign_and_zero`: Chooses between library routine sequences based on whether an operand is zero and on its sign, then returns the routine result.
- `dispatch_by_sign_then_finish`: Chooses a library routine sequence from the sign and zero state of an operand, calls the chosen sequence, then calls a finishing routine and returns its result.
- `finish_math_result_clear_flags`: Calls a library routine, then returns its result with two status bits cleared.
- `finish_math_result_twice`: Calls the same library routine twice, then returns its result with two status bits cleared.
- `mask_return_value`: The library code calls a function through a pointer stored in a global variable twice, then calls another routine and returns its result with certain bits cleared.
- `execute_indirect_function_2`: Fetches a function pointer from a global table and executes it.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `execute_indirect_function_3`: Fetches a function pointer from a location calculated by adding an offset to a global variable and then calls that function.
- `clamp_operand_exponent_range`: Clears the sign bit of an operand and, when its exponent-like byte is above a bound, sets a flag and calls a routine.
- `shift_subtract_quotient_loop`: Shifts an operand by a masked count and subtracts shifted values in a loop limited to twenty-four steps, building a quotient.
- `accumulate_shifted_partial_sums`: Repeatedly adds a shifted value to an accumulator and subtracts shifted partial values, calling routines when a counter reaches its limit.
- `classify_byte_operand_range`: Tests a byte operand against doubled-value bounds, returns early outside the top range, otherwise calls three routines and masks the low byte of the operand.
- `classify_signed_byte_band`: Returns early for a signed byte outside a band, otherwise tests its doubled value against bounds and calls routines.
- `classify_negative_byte_bound`: Tests a signed byte against doubled-value bounds, returns early outside the range and otherwise calls routines for the band.
- `classify_unsigned_byte_bound`: Returns early for an unsigned byte below a bound, otherwise tests its doubled value against an upper limit and calls routines.
- `classify_signed_byte_with_calls`: Returns early for a byte outside the sign-dependent band, otherwise tests its doubled value and calls routines for the matching range.
- `normalize_byte_operand_flags`: Calls three routines, then masks the operand's low byte and sets its sign flag from the result of a routine call.
- `compute_scaled_remainder_loop`: Adjusts a sum by a shifted divisor in a loop controlled by a signed count, then returns the adjusted value.
- `select_fixed_constant_by_sign`: Selects one of two large fixed-point constants according to the sign of its operand, adds a scaled operand and runs a shift loop over the result.
- `accumulate_fixed_point_terms`: Loops over a table of terms, accumulating scaled sums into three accumulators and calling routines for each step.
- `stub_empty_return`: Does nothing and returns; it is an empty stub with no observable operation.
- `stub_bad_instruction_data_2`: Decoded from data rather than real code: it makes a self-call to its own entry point with no condition, so it is not a working routine.
- `chain_two_routine_calls`: Calls two routines in sequence without any logic of its own.
- `convert_integer_to_float_format`: Converts a signed integer to the packed float form by normalising its magnitude through shifts and selecting an exponent code from its sign.
- `normalize_mantissa_by_doubling`: Returns early when its exponent-like byte is at its limit, otherwise normalises the mantissa by repeated doubling and records the result.
- `call_routine_then_repeat`: Calls a routine, then calls it again twice when a flag remains clear.
- `call_routine_then_three_followups`: Calls a routine, then calls three further routines in sequence when a flag remains clear.
- `call_routines_with_incremented_count`: When its input is nonzero and no overflow occurs, calls two routines with the count incremented by one and then a third routine.
- `empty_routine_5`: This library code routine is empty.
- `return_from_subroutine_2`: This is an empty routine that performs no operations before returning.
- `return_from_subroutine_3`: This library routine performs a sequence of instructions that results in an immediate return to the caller.
- `empty_stub_2`: The library code contains five instructions and performs no observable work; it ends immediately.
- `empty_routine_6`: The library code consists of four 68000 instructions and performs no observable work, immediately returning to its caller.
- `pack_float_with_shift`: Returns for zero input, otherwise computes a shift from the low exponent byte, shifts the mantissa and stores a packed word in the result, calling routines along the way.
- `run_four_routine_sequence`: Calls four routines in sequence without any logic of its own.
- `sum_alternating_table_entries`: Walks a table, adding each entry while the accumulator is negative and subtracting it otherwise, until the count runs out.
- `accumulate_table_with_subtraction`: Walks a table, adding and subtracting entries according to the sign of a running accumulator, and returns when the table is exhausted.
- `dispatch_operand_sign_routines`: Tests a byte operand for zero and sign, calls routines accordingly, then returns.
- `return_from_subroutine_5`: This routine performs a single instruction to return control to the caller.
- `pack_operand_after_routines`: Calls four routines, then when a flag is set shifts a mantissa-sized value over a loop to pack the result.
- `divide_remainder_by_shift_loop`: Computes a remainder and quotient bits by repeatedly subtracting a shifted divisor in a loop controlled by a count.
- `call_routine_three_times`: Calls the same routine three times in succession, then returns.
- `classify_operand_sign_and_range`: Calls routines depending on the sign of a byte operand and, for a nonzero value in range, calls further routines and sets a flag.
- `normalize_operand_after_calls`: Calls two routines, returns if the operand is zero, otherwise calls a routine and computes a shift from the exponent byte to normalise the operand.
- `empty_library_stub`: The library code contains no instructions and performs no operation.
- `return_immediately_2`: Library code that performs no operations and returns to the caller.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `execute_short_routine`: Library code executes a short sequence of instructions and calls another routine.
- `return_from_subroutine_4`: The routine is empty and performs no operations before returning.
- `execute_intermediate_routine`: Library code executes a sequence of five instructions and calls another routine.
- `execute_minimal_routine`: Library code executes a two-instruction sequence and calls another routine.
- `call_pointer_and_return_input`: Calls the routine whose address is in the address register and returns the value that was in the data register on entry.
- `call_indirect_return_d0`: The library code receives a function pointer in A0 and a value in D0, calls the routine pointed to, and then returns the value that was in D0 on entry.
- `call_routine_and_return_input`: Calls one routine, then returns the value that was in the data register on entry.
- `call_pointer_and_return_input_2`: This library code calls the routine whose address is in the address register and returns the value that was in the data register on entry.
- `execute_indirect_call`: Calls a routine via a pointer stored in a data structure and returns the resulting value in a data register.
- `pass_through_register_value`: This routine calls another routine and returns the original value held in a data register.
- `pass_through_register_value_2`: This routine acts as a transparent wrapper that immediately calls another routine and returns the original input value held in a data register.
- `classify_negative_byte`: Library code calls a routine via a pointer and then tests a signed byte against doubled-value bounds.
- `classify_signed_band`: Library code calls a routine via a pointer and then performs a signed byte band classification.
- `call_routine_then_store_value`: Calls a routine, then stores the data register's longword into the memory address held in the second data register.
- `call_routine_sequence`: Library code calls a routine via a pointer and then executes three subsequent routines if a flag is clear.
- `search_pattern_in_text`: Library code calls a routine via a pointer and then searches for a pattern within a text string.
- `call_routines_incremented`: Library code calls a routine via a pointer and then calls two routines with an incremented count followed by a third routine.
- `pack_float_value`: Library code calls a routine via a pointer and then computes a packed word from a floating point value using a shift.
- `read_single_character_2`: Library code calls a routine via a pointer and then attempts to read a single character into a stack buffer.
- `call_routine_thrice`: Library code calls a routine via a pointer and then calls that same routine three times in succession.
- `call_indirect_and_return_d0`: The library routine calls a function via a pointer in a register and returns the value that was in a data register.
- `forward_call_with_stack`: Library code calls a routine via a pointer and then performs a call using two parameters and a stack argument slot.
- `call_and_return_register`: This library code calls a routine through a pointer and returns the value that was in the data register upon entry.
- `call_indirect_with_offset_and_return_d0`: The library routine calls a routine at an address calculated by adding an offset to a pointer stored in a global variable and returns the value in a data register.
- `execute_indirect_offset_call`: Calls another routine by adding a negative offset to a pointer retrieved from a global variable.
- `execute_callback_and_concatenate`: Invokes a routine via a pointer retrieved from a global variable and returns the concatenation of the resulting data registers.
- `invoke_callback_and_concatenate`: Library code invokes another routine via a pointer stored in a global variable and returns the concatenation of two 32-bit values.
- `call_indirect_and_return`: Library code calls a routine at an address calculated by adding an offset to a pointer stored in a global variable and returns a 32-bit value.
- `call_indirect_with_offset_2`: Library code executes a routine at an address derived from a global pointer and an offset, returning a 32-bit value.
- `invoke_callback_and_concatenate_alt`: Library code performs an indirect call using a global pointer and returns the combined result of two 32-bit registers.
- `invoke_callback_and_concatenate_v2`: Library code calls a subroutine through a pointer in a global variable and returns the concatenation of two 32-bit values.
- `invoke_callback_and_concatenate_v3`: Library code executes a routine via a pointer stored in a global variable and returns the concatenation of two 32-bit values.
- `invoke_callback_and_return_concat`: Library code invokes another routine via a pointer stored in a global variable and then returns the concatenation of the two input values.
- `call_indirect_with_offset`: The library code calls a routine at an address calculated by adding an offset in A0 to a pointer stored in a global variable, then returns the value in D0.
- `normalize_fixed_point_value`: The library code normalizes a signed 32-bit fixed-point value in D6 by swapping its high and low words, adjusting a shift count based on sign and magnitude, then left-shifting the value while decrementing the count until the count reaches zero or the value overflows, returning early on overflow.

### icon.library

- `return_error_code`: Library code that returns a constant value.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `update_icon_state`: This library code checks a value in a structure; if zero, it manipulates a linked list of pointers, otherwise it sets a specific bit in a byte.
- `set_bit_in_byte`: Library code that sets a specific bit in a byte at an offset from a base pointer and returns zero.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `update_library_state`: This library code copies two 32-bit values from a table into global variables, or calls a routine and updates globals if the return value is non-zero.
- `copy_two_pointers_from_table`: This library routine copies two 32-bit values from an address derived from a register into two global variables.
- `empty_routine`: Library code that is empty.
- `return_from_library_call`: This routine performs a simple return to the caller.
- `call_library_handler`: Library code that simply calls another routine and then returns.
- `frag_movem_move_jsr_lea_rts`: Short code fragment that saves registers on the stack, then copies a6 to the stack, then calls copy_icon_data_to_stack, then loads the address of the memory at an offset from a7 into a7, then returns.
- `trigger_icon_resource_update`: This library routine serves as a wrapper that invokes a single function to update icon resource pointers and then returns.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `call_library_handler_2`: This library routine simply calls another routine and then returns.
- `increment_and_call_handler`: This library routine increments a parameter and calls another routine with the updated value before returning.
- `call_conditional_callbacks`: This library routine calls another routine and then executes a series of conditional callbacks based on the results of that call.
- `call_icon_init_routine`: This library routine invokes a single initialization function to set up icon data structures and then returns.
- `call_library_handler_3`: This library routine calls another routine and then returns.
- `invoke_handler_pointer`: This library routine retrieves a function pointer from a data structure and executes it.
- `call_other_routine_2`: This routine calls another routine and then returns.
- `call_search_string_offset`: This routine calls another routine and then returns.
- `call_numeric_parser`: This library routine calls another routine and then returns.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `execute_nested_calls`: Library code that performs a series of conditional calls to other routines based on returned values and modifies a local variable before returning.
- `update_icon_structure`: This library routine calls several subroutines to process data, copies ten longwords from a structure to the stack, and updates various fields within a structure based on the results.
- `copy_data_to_structure`: This library code copies a block of eleven words from one location to another and then performs several individual word transfers into a structure.
- `return_zero_stub_2`: The routine is an empty stub that immediately returns the constant zero.
- `copy_icon_data_to_stack`: This library routine copies ten longwords from an offset within a structure pointed to by the third argument into a local array on the stack, then copies several additional fields from the same structure into other stack slots and finally calls another routine with the first two arguments and a pointer to a stack-resident header.
- `update_icon_resource_pointers`: Updates an icon structure by allocating and assigning new memory buffers for its image data, color palettes, or attribute lists.
- `update_icon_attribute_list`: Iterates through a linked list of icon attributes to update specific structure offsets based on returned values from a callback routine.
- `update_linked_list_nodes`: Library code that iterates through a linked list, calling a function pointer to update specific offsets within a structure based on the returned values.
- `execute_library_commands`: The library code executes a series of function calls from a list, updating a pointer and a value in a structure based on the results.
- `call_and_return_zero`: This library code calls another routine and then returns the constant zero.
- `empty_routine_2`: The routine contains four instructions and performs no observable work; it ends immediately with a return.
- `process_icon_metadata_structures_2`: Iterates through nested icon metadata structures, including single pointers and null-terminated tables of pointers, to update or validate their contents.
- `process_icon_metadata_structures`: Iterates through a series of nested icon metadata structures, including single pointers and null-terminated tables of pointers.
- `traverse_icon_structures`: Library code that recursively processes a linked list and a null-terminated table of pointers, invoking a callback for each entry.
- `process_icon_table`: This library routine walks a null-terminated table of pointers, calling a routine for each entry, updating a counter, and invoking a callback.
- `iterate_icon_table_entries`: Library code that walks a null-terminated table of pointers, calling a routine for each entry, updating a counter, and invoking a callback while potentially processing a secondary entry.
- `traverse_linked_list_2`: The library code traverses a linked list structure by following pointers stored at an offset from the current node.
- `iterate_linked_list_nodes`: The library code traverses a linked list structure by following pointers stored at an offset from the current node.
- `increment_param_and_recurse`: Library code that calls another routine with the same first argument and the second argument increased by a constant, then returns.
- `execute_conditional_callbacks`: Library code that performs a check and executes a series of function calls with varying parameters based on the results of previous calls.
- `clear_index_and_update_table`: Returns zero and then decrements a counter through a pointer before using that counter as an index to store two 32-bit values into a table.
- `return_zero_2`: The library code returns zero.
- `decrement_counter_and_store_pair_2`: The library code decrements a short counter through a pointer, then uses the counter value as an index into a table to store two 32-bit values.
- `decrement_counter_and_store_pair`: The library code decrements a short counter through a pointer, then uses the counter value as an index into a table pointed to by a global variable.
- `execute_library_logic`: This library routine performs a series of conditional calls and data processing steps involving multiple subroutines and returns a value derived from one of the calls.
- `call_library_routine_and_return_zero_2`: This library routine calls another routine and then returns zero.
- `call_library_routine_and_return_zero`: Library code that calls another routine and then returns zero.
- `incremental_recursive_call`: Library code that performs a recursive call with an incremented parameter until the callee stops the recursion.
- `find_string_assignment`: This library code iterates through a list of pointers to find a specific character within a string and returns a pointer to the character following an equals sign.
- `find_character_match`: This library routine compares sequences of bytes to find the position of a specific character.
- `find_equality_delimiter`: Library code that iterates through a list of pointers to find a specific character within a string.
- `search_string_offset`: This library code searches for a specific offset within a string and performs a comparison.
- `search_string_for_offset`: Library code that searches through a string to determine if a specific offset exists within the sequence.
- `search_linked_list`: As library code, this routine performs a search through a linked structure using arithmetic and conditional checks to find a specific element or return a status.
- `execute_loop_logic`: The library code performs a loop involving arithmetic operations and conditional calls to other routines.
- `verify_string_offset`: This library code searches through a string to determine if a specific offset exists within the sequence and returns a boolean result.
- `parse_numeric_string_3`: This library routine parses a string to extract a numeric value, performing memory comparisons and string manipulations before storing the result in a structure.
- `parse_numeric_string_2`: This library code converts a sequence of characters into a numeric value.
- `convert_numeric_string`: This library code converts a string of characters into a numeric value and performs a memory comparison.
- `parse_numeric_string`: Library code that converts a string of characters into a numeric value and performs a memory comparison.
- `offset_input_value`: The library code subtracts a constant from an input value if it falls within a specific range, otherwise returning negative one.
- `return_negative_one`: The library code returns a value representing negative one.
- `call_indirect_pointer_3`: The library code calls a function pointer retrieved from an offset within a global variable.
- `invoke_indirect_function`: This library code retrieves a function pointer from a memory location and executes it.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: The library routine forwards a set of four parameters to another routine, checks the returned value, and if non-zero calls the same routine again with a modified argument list.
- `reschedule_library_call`: This library code calls a function and, if the result is non-zero, writes a constant to a memory location and triggers a system reschedule.
- `return_zero_stub`: Library code that returns the constant zero.
- `validate_parameter_match`: Library code that calls another routine and returns zero if the result does not match a passed parameter, otherwise returning the result.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `validate_library_return`: This library code calls another routine and returns zero if the returned value does not match a provided parameter.
- `return_void_stub`: Library code that performs no work and returns.
- `perform_32bit_math`: The library code performs complex 32-bit arithmetic involving bit rotation and division based on the input values.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_at_pointer`: Library code that stores a byte from a register into a memory location pointed to by a register.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `find_character_in_string`: The library code searches for a specific character within a null-terminated string and returns a pointer to its location.
- `return_zero_3`: This library routine is empty and simply returns zero to the caller.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `copy_string_data`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered.
- `compare_memory_blocks`: Library code that compares two sequences of bytes and returns one if they match or zero if they do not.
- `return_one`: The library code returns one.
- `decrement_and_skip_string`: As library code, this routine iterates through a null-terminated string while decrementing a counter until the string ends or the counter reaches zero.
- `call_and_return`: Library code that calls another routine and then returns.
- `copy_and_terminate_string`: The library code copies a sequence of bytes from one location to another and then appends a null terminator.
- `empty_routine_3`: This library routine is empty.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `process_icon_request`: Library code that validates a parameter, calls another routine to check a return value, and if successful, performs further calls using a value from a pointer.
- `traverse_icon_list_recursively`: Recursively traverses a linked list structure by following pointers.
- `iterate_list_until_zero`: This library routine repeatedly calls another routine in a loop, traversing a structure through a pointer until a zero value is encountered.
- `return_zero`: This library routine performs no operations and returns zero to the caller.
- `calculate_complex_product`: This library code performs a series of nested function calls and multiplications to compute a value and store it in a structure.
- `return_negative_one_stub`: Library code that returns a value of all bits set.
- `validate_linked_list_nodes`: This library routine iterates through a linked list by repeatedly calling a function pointer and validating the returned values against specific offsets in a data structure.
- `traverse_linked_list`: This library code iterates through a list of pointers, performing checks and updating values in a loop until a condition is met or an error occurs.
- `validate_and_update_linked_list_node`: Iterates through a linked list by repeatedly calling a function pointer and validating the returned values against specific offsets in a data structure.
- `return_all_ones`: This routine returns a value consisting of all bits set to one.

### diskfont.library

- `dispatch_font_handle`: Library code that calls a function via a table offset and stores a value into a structure if the call succeeds.
- `check_and_set_font_handle`: As library code, this routine calls a function via a table entry using the library base, stores an argument into an offset within the returned handle if successful, and then calls another routine via a table entry.
- `release_library_resources_2`: Library code that calls two different routines to close a library and free memory.
- `release_library_resources`: This library code calls two different routines to close a library and free memory.
- `reclaim_unused_font_memory`: Iterates through a linked list of font structures to identify and free those with a reference count of zero.
- `manage_font_memory`: This library routine checks specific conditions in a structure to either update a linked list and free memory or set a bit in a byte and return zero.
- `set_high_bit`: Library code that sets the most significant bit of a byte at a specific offset from a pointer.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `decrement_and_dispatch`: Library code that decrements a short integer at an offset and, if the result is zero and a specific bit is set, calls a function pointer located at another offset.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `populate_font_metadata_table`: Iterates through a linked list of font data structures to extract character names and bitmap information.
- `populate_font_buffer_from_list`: Iterates through a linked list of data structures to calculate string lengths and copy character data into a destination buffer.
- `process_font_string_data`: Iterates through a linked list of data structures to parse strings and copy character data into a destination buffer.
- `parse_font_metadata_stream_2`: Iterates through a data stream to extract character metadata and bitmap information.
- `parse_font_data_stream_2`: Iterates through a font data stream to extract character metadata and bitmap data.
- `populate_font_glyph_table`: Iterates through a font data stream to extract character metadata and bitmap data.
- `parse_font_data_stream`: Iterates through a data stream to identify specific font character markers and metadata.
- `parse_font_metadata_stream`: Iterates through a data stream to extract character metadata and bitmap information.
- `return_zero_stub_3`: Library code that returns zero and performs no operations on registers, memory, or hardware.
- `calculate_font_similarity_score`: Calculates a weighted score by comparing bit patterns between two data structures.
- `calculate_weighted_font_index_2`: As library code, this routine performs a weighted summation by iterating through a table of values using a bitwise loop to compare bytes.
- `compute_weighted_font_index`: Performs a weighted summation by iterating through a table of values using a bitwise loop.
- `calculate_weighted_font_index`: Iterates through a table of values using a bitwise loop to perform a weighted summation or reduction.
- `return_constant_value`: As part of library code, this routine returns a constant value without using any arguments or registers.
- `select_optimal_font_glyph`: Iterates through a collection of font glyphs to identify the one with the highest priority value.
- `update_font_glyph_metadata`: Iterates through a font resource buffer to find the glyph with the highest priority and updates its internal metadata, including size and status flags.
- `reallocate_font_resource_buffer`: Iterates through a list of font entries to find the one with the highest priority and reallocates its memory buffer to a new size.
- `select_optimal_font_resource_2`: Iterates through a list of font resource entries to find the entry with the highest priority or size value.
- `search_font_resource_list`: Iterates through a list of font resource entries to identify the optimal resource based on priority or size.
- `select_optimal_font_resource`: Iterates through a list of font resource entries to find the one with the highest priority or size value.
- `cleanup_font_resources`: Releases allocated memory, closes open files, and closes font handles while clearing specific entries in a data structure.
- `parse_disk_font_data`: Iterates through a font data structure to extract character metrics and control bytes.
- `load_disk_font_table`: As library code, this routine builds a table for a disk font by allocating memory, filling a header, and copying font metric and control bytes from an input source into the table.
- `dispatch_font_request`: As part of library code, this routine reads a pointer and an argument from the stack and passes them to another routine.
- `call_exec_forbid`: As part of library code, this routine calls a function through a pointer stored in a global variable.
- `execute_indirect_function`: Fetches a function pointer from a location calculated by an offset from a global variable and executes it.
- `execute_indirect_function_pointer`: Fetches a function pointer from a location offset from a global variable and executes it.
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
- `skip_string_terminator`: This library routine increments a pointer until it reaches a zero byte and then calls another routine.
- `skip_to_string_terminator_and_call`: The library routine increments a pointer until it reaches the terminating zero byte of a zero-terminated string and then calls another routine with the pointer to that terminator.
- `count_null_terminated_bytes_2`: The library routine increments a counter while advancing a pointer and reading bytes until a zero byte is encountered.
- `count_null_terminated_bytes`: As part of library code, this routine increments a counter while advancing a pointer and reading bytes until a zero byte is encountered, returning the final count.
- `compare_string_bytes_2`: This library routine compares two strings byte by byte to determine their lexicographical order.
- `compare_string_consecutive_bytes`: The library routine compares consecutive bytes in a string and returns a constant value based on whether the first byte is less than the second byte.
- `compare_string_bytes`: This library routine compares two strings byte by byte and returns a value based on their lexicographical order.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `find_byte_in_string_2`: This library routine searches for a specific byte within a zero-terminated string and returns a pointer to its location or zero if not found.
- `search_string_for_byte`: This library routine iterates through a string to find a matching byte and returns the pointer to that byte or zero.
- `find_byte_in_string`: The library routine searches for a specific byte within a string and returns a pointer to its location or zero if not found.
- `search_backward_for_byte_3`: This library routine searches backwards through memory starting from a specified offset to find the first occurrence of a specific byte.
- `search_backward_for_byte`: The library routine decrements a pointer to search backwards through memory for a specific byte and returns the pointer to that byte if found.
- `search_backward_for_byte_2`: This library routine decrements a pointer to search backwards through memory for a specific byte until a limit is reached.
- `search_specific_bytes`: Library code that searches backwards through memory for two specific byte values.
- `increment_returned_value`: This library routine returns the value passed in D0 incremented by one.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.
- `call_other_routine`: The routine calls another routine and then returns.
- `call_another_routine`: This library code acts as a thin wrapper which immediately invokes another routine and then returns without performing any additional operations.

### Defender (1994)(Ratsoft)[t +10 Apocalypse].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
