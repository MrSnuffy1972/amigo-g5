# Game 2277 (star wars (1987)(domark)[cr bs1 - tkt]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02277). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901684 top-level bytes in 39 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 9 raw-word lines |
| Functions named and described | pass, 538 of 538 named (0 author symbols), 538 of 538 described (81 by rule, 32 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 882 of 882 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 19117 of 19119 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 236054 |
| code | 63902 |
| data | 73717 |
| padding | 7307 |
| unused | 520704 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Star Wars (1987)(Domark)[cr BS1 - TKT].adf:binddrivers | 4 | 542 | 19 |
| Run | 2 | 435 | 12 |
| Stack | 2 | 117 | 2 |
| List | 2 | 286 | 18 |
| WB2 | 3 | 22 | 1 |
| Port-Handler | 1 | 306 | 1 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 493 | 33 |
| Star Wars (1987)(Domark)[cr BS1 - TKT].adf:KENT | 1 | 221 | 10 |
| Star Wars (1987)(Domark)[cr BS1 - TKT].adf:starwars | 3 | 16268 | 410 |
| Star Wars (1987)(Domark)[cr BS1 - TKT].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 27 | 27 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |

## Cases handled

- 1 first-execution byte records contradict the logged opcode word (first an address): bad captures, the opcode word is used
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 3 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 0 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 4 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 jump target, 1 address, 1 displacement

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Star Wars (1987)(Domark)[cr BS1 - TKT].adf:binddrivers

- `set_up_display_and_dma`: Entry routine that opens the graphics library through the exec vectors, copies eight colour words into the custom colour registers, and builds a copper list and bitplane pointer table.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `advance_bitplane_offsets_and_store`: Adds a signed step to a stored 32-bit value and reverses the step.
- `start_blitter_and_wait`: Sets the blitter control, modulo and destination pointer registers from the current buffer pointer, starts a fixed-size blit, and busy-waits until the blitter reports it is finished.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `plot_and_advance_points`: Loops over 64 point records, setting one bit in the bitplane at each point's position.
- `blit_rows_from_offset_table`: Waits for the blitter, then queues a series of blits for a grid of rows, taking each row's offset from a lookup table.
- `blit_tile_grid_from_table`: Draws a grid of 32 rows by 2 cells, issuing one blitter operation per cell after waiting for the blitter.
- `advance_bouncing_positions`: Updates two coordinates by their velocities and reverses each direction at fixed limits.
- `fill_lookup_table_entries`: Fills up to 32 eight-byte table entries, each holding an incrementing byte and a word taken from a second table.
- `wait_blitter_then_write_control`: Calls a subroutine, saves the status value it returns, and busy-waits until the blitter busy flag clears.
- `init_audio_channel_table`: Sets up four audio channel records that point at the audio volume registers and their handler routines.
- `update_audio_sample_buffer`: Increments a counter and, upon reaching a threshold, calculates a new memory address for an audio buffer based on a table lookup and a multiplier.
- `update_audio_channel_pitch`: Updates one audio channel record's pitch value by a step taken from a parameter byte.
- `start_audio_channel_sample`: Starts playback on an audio channel: it copies the sample pointer from a table, reads the sample length and volume from parameter tables, and writes them to the channel's pointer and length registers.
- `dispatch_interrupt_handlers`: Checks hardware status bits and calls a routine from a table based on which bit is set.
- `dispatch_interrupt_bit_handlers`: Tests two request bits in the supplied value.
- `call_interrupt_handler_vector`: Tests bit 16 of the supplied flags and, when it is set, calls the handler through library vector 136.
- `return_saved_value_stub`: Returns the value it was passed on the stack without doing any other work.

### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_fifteen_longs_then_call`: Copies fifteen longwords from a fixed table into the buffer passed in, if that buffer pointer is non-null, then calls the routine whose address is held in a register.
- `initialize_entity_structure`: Sets various fields in a structure, including bitwise operations on a byte, clearing specific offsets, and copying data from a table into the structure.
- `update_entity_state_and_buffer`: Fetches a byte from a table using an index stored in a structure, stores it at offset 32 and increments the value at offset 36 of that structure, then calls another routine to update a value at offset 12 and performs a loop to copy bytes from a source buffer into a destination buffer based on a calculated offset.
- `copy_byte_to_indexed_buffer`: Calculates a destination offset by adding a value from a structure to a scaled index, then copies a single byte from a source buffer into that destination.
- `initialize_object_properties_and_call`: Sets specific byte and longword values in a structure pointed to by a0 using offsets derived from a1, calls a subroutine, and then updates a longword in the structure in a1 based on the return value.
- `load_offset_to_register`: Loads a 32-bit value from an offset of 80 in a structure pointed to by a1 into d1 and then branches to another routine.
- `test_longword_and_branch`: Tests a 32-bit value at offset 44 of a structure in a1 and branches to another routine if the value is non-zero, otherwise it branches to a third routine.
- `modify_status_byte`: Tests a longword at offset 52 of a structure in a1 against a constant, and if the bitwise AND result is zero, sets bits 24 through 31 of that same memory location before branching to another routine.
- `update_entity_pointer_and_call_handler`: Copies a longword from offset 60 of a structure to offset 104 of the same structure, then calls a routine using parameters loaded from offset 132 of a second structure and offset 32 of the first structure.
- `reset_entity_state`: Updates an entry in a table using values from a structure in a1, clears several fields in that table entry, and updates multiple fields in the structure in a1 including a pointer and a status value.
- `update_game_state`: Calls several routines using parameters from a structure in a2, updates a 32-bit value in a1 from a register, and clears a byte in a memory buffer at an offset calculated from a1 and a4, before jumping to a location in a6.

### Stack

- `find_max_and_prepare_buffer_2`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_stack_data`: Copies a block of sixteen bytes from a source address to a destination address on the stack before calling another routine.

### List

- `find_max_and_prepare_buffer_3`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_stack_data_2`: Copies sixteen words from a source pointer to a destination pointer located on the stack.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `search_table_for_match`: Writes a value from a structure in a4 to a structure in a1, then enters a loop that compares a value in a1 with values in a table at a0 using an index calculated from a structure in a4.
- `copy_and_jump`: Increments a value at an offset of negative 40 from a4, calculates an index by adding the value at offset negative 44 from a4 to the incremented value and shifting it left by two, copies a longword from a1 to a destination at a0 plus that index, and performs an indirect jump using a6.
- `compare_and_set_constant`: Changes bit 3 of d3, moves a longword from a pointer in a0 to d0, and compares d0 with a value from a structure in a4, otherwise it sets d2 to negative 1 and stores it in a structure in a4.
- `store_byte_and_jump`: Increments a value at an offset of negative 108 from a4, calculates an index using the value at offset negative 120 from a4 and the incremented value shifted left by two, loads a byte from a0 plus that index into d2, stores d2 at an offset of negative 112 from a4, and performs an indirect jump using a6.
- `modify_bit_and_call`: Performs a bitwise change on a value at an offset of 25976 from a6, compares the value at offset negative 168 from a4 with 39, and if they are equal, calls a routine at a5 twice with d0 set to 12 and a4 pointing to an offset of negative 56 from its original value before performing an indirect jump using a6.
- `update_structure_fields`: Modifies a byte at an offset determined by adding a register to a0 and d7, then moves two longwords from offsets of negative 200 and negative 204 relative to a4 into offsets 0 and 4 of a structure in a1 before calling a routine at the address in a5.
- `write_constant_and_jump`: Sets a 32-bit value to all ones at an offset of negative 192 from a4, loads a 32-bit value from a pointer in a1 into d1, and performs an indirect jump using the address in a6.
- `call_twice_and_jump`: Calls a routine at a5 with a4 at its current address and d0 set to 32, then calls the same routine with a4 at an offset of 276 from its original address and d0 set to 20, and finally performs an indirect jump using a6.
- `call_and_update_pointer`: Calls a routine at a5 with d0 set to 20 and a4 at an offset of 140, stores the returned value from d1 back into the location pointed to by a1, compares the value at offset negative 204 from a4 with 41, and if they are not equal, stores -1 at offset negative 192 from a4 before calling a5 again and performing an indirect jump using a6.
- `dispatch_by_id`: Loads a longword from a structure in a1 and compares it against several constant values, branching to different locations if the value matches -1, 35, 40, 41, or 124.
- `process_parameter_list`: Clears a longword at offset 4 of a structure in a1, calls a routine using a pointer from a0 and a modified address in a4, stores a longword in offset 8 of the structure in a1, and then performs a second call using values from the structure and a different offset in a4 before jumping to an address in a6.
- `write_byte_to_offset`: Calculates an offset by shifting a longword from a4 and adding a longword from a1, then writes a byte from a longword at offset negative 340 of a4 to the address in a0 plus that offset, stores that same longword into a1, and calls a routine at the address in a5.
- `call_subroutine_with_parameters`: Loads values from structures in a4 and a1 into d2 and d1, sets d0 to 24, updates a4, and calls a routine at the address in a5.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `permute_linked_list_bytes`: Iterates through a linked list, swapping a byte from a table into a memory location and updating the list pointer until a null pointer is encountered, then performs an indirect jump.

### WB2

- `open_graphics_intuition_libraries`: Opens the graphics and intuition libraries through library calls, using the library names referenced in the file.

### Port-Handler

- `initialise_handler_message_port`: Initialises a handler's control block for a message port.

### Disk-Validator

- `validate_disk_block_table`: Program entry for a disk validation utility.

### Ram-Handler

- `copy_memory_then_jump`: Calls the exec copy-memory vector to move a block of memory, then jumps to the address it has loaded.
- `process_object_event_data`: Retrieves an index from a structure in a1 to look up a value from a table in a0, then performs a series of conditional calls to anot based on the retrieved value.
- `update_object_event_data`: Retrieves an index from a structure in a1, uses it to look up a value from a table in a0, and performs a series of conditional calls to anot.
- `register_object_event_data`: Retrieves an index from a structure in a1, uses it to look up a value from a table in a0, and performs a series of conditional calls to anot.
- `execute_dual_io_requests`: Performs two sequential calls to a function using parameters loaded from structures in a1 and a2, passing specific command constants of 232 and 216 in d0.
- `update_ram_handler_state`: Fetches a value from an offset in a structure pointed to by a1, calls a routine with a parameter in d0, then fetches another value from a structure in a2 to call the same routine again with a different parameter before branching.
- `execute_parameterized_function_call`: Loads three values from a structure pointed to by a1 into registers d1, d2, and d0, where d0 is set to 216, then calls a function using a pointer from a5 and a target address from a4, before branching back to a previous location.
- `update_entity_state_via_table`: Retrieves an index from a structure in a1, uses it to look up a value in a table pointed to by a0, and then performs two calls to a function in a5 using different parameters and offsets derived from the retrieved value and a structure in a2.
- `dispatch_event_data`: Fetches an index from a structure in a1, uses it to retrieve a value from a table in a0, and performs two calls with different parameters based on that value.
- `update_object_state_via_table`: Fetches an index from a structure in a1, uses it to retrieve four related pointers from a table in a0, and performs two calls to a function in a5 using specific status codes.
- `process_ram_handler_request`: Fetches an index from a structure in a1, uses it to look up a value in a table pointed to by a0, and stores the result at offset 204 of the structure in a1.
- `execute_ram_handler_request`: Loads a value from an offset of 364 in a structure pointed to by a2 into d3, sets d2 to zero, loads a value from an offset of 200 in a structure pointed to by a1 into d1, sets d0 to 220, loads a pointer from an offset of 196 in the same structure into a4, and calls a function via a pointer in a5.
- `execute_dual_system_calls`: Loads a value from an offset in a1 into d1, sets d0 to 232, and calls a function via a pointer in a4.
- `dispatch_event_type_actions`: Fetches an index from a structure in a1, uses it to retrieve three values from tables in a0, and performs a series of conditional calls to a function in a5 based on the value retrieved from a table at offset 20 of the structure in a0.
- `dispatch_event_type_handler`: Fetches an index from a structure in a1, uses it to retrieve three values from tables in a0, and calls a function twice with different parameters.
- `update_resource_state`: Calculates an index from a value at offset 200 of a structure in a1, uses this index to fetch three pointers from a table in a0, and performs two calls to a function using specific constants in d0 and modified pointers in a4.
- `update_resource_table_entry`: Fetches an index from a structure in a1, uses it to retrieve three values from a table in a0, and calls a function twice with different parameters and offsets.
- `dispatch_event_type_handler_2`: Fetches an index from a structure in a1, uses it to retrieve three values from a table in a0, and performs a series of conditional calls to a function in a5 based on the value retrieved from the table.
- `dispatch_resource_request`: Calculates an index from a value at offset 200 of a structure in a1, uses it to fetch a pointer from a table in a0, and performs a series of conditional calls to a subroutine based on the value retrieved from that pointer.
- `update_structure_field_via_table`: Calculates an offset into a table using a value from a structure, retrieves a longword from that table, and calls a function pointer in a5.
- `update_ram_handler_state_2`: Performs two sequential calls to a function pointer in a5, using data from a1 and a2 to update specific fields in a structure.
- `update_resource_table_entry_2`: Fetches an index from a structure in a1, uses it to retrieve a value from a table in a0, and then calls two subroutines to update various fields in structures pointed to by a1 and a2 based on that value.
- `execute_conditional_resource_requests`: Checks a flag in a structure via a2; if zero, it performs three sequential calls to a function using parameters from a1 and a2, specifically setting d3 to zero or 202 and d2 to negative one or zero, before branching to another routine.
- `update_ram_handler_state_3`: Performs two consecutive calls to a function pointer using different parameters and structures.
- `dispatch_command_by_index`: Fetches an index from a structure in a1, uses it to look up a command type from a table in a0, and calls a function via a5.
- `execute_event_action`: Loads a value from an offset of 200 in a structure pointed to by a1 into d1, sets d0 to 216, updates a pointer in a4 by adding 1534, and calls a subroutine via a pointer in a5.
- `dispatch_resource_request_2`: Loads a value from a table using an index retrieved from a structure, then executes a large switch-case block that calls another routine multiple times based on the retrieved value and updates specific fields in a structure.
- `dispatch_event_handler`: Loads a value from an offset in a structure into a register, retrieves a function pointer from a table using that value, and calls the function via a pointer in a register.
- `dispatch_event_type_actions_4`: Retrieves an index from a structure in a1, uses it to look up a value from a table in a0, and performs a series of conditional calls to another routine based on that value.
- `dispatch_ram_handler_request_type`: Compares a value in register d1 against several constants and branches to specific subroutines based on the match.
- `dispatch_event_type_actions_3`: Compares a value in d1 against several constants and branches to specific routines based on the match; otherwise, it returns.
- `dispatch_event_type_actions_2`: Compares a value in d1 against several constants and branches to specific routines based on the match; otherwise, it performs a series of conditional calls to another routine.
- `branch_to_offset`: Performs a short branch to an offset.

### Star Wars (1987)(Domark)[cr BS1 - TKT].adf:KENT

- `dispatch_command_line_arguments`: Program entry that checks the argument count.
- `open_dos_library`: Opens a library through the exec vector and, on success, stores its base in a global.
- `write_string_if_nonempty`: Measures the length of the null-terminated string at the address in a register.
- `copy_magic_cookie_to_buffer`: The routine obtains a file lock, opens a file, reads a block into a buffer, scans the buffer for the ASCII sequence 'magic' and, if found, copies 300 bytes from a global variable into the buffer starting at that position, writes the modified block back to the file and repeats the operation.
- `free_memory_or_return`: Returns the stored value if a global flag is zero.
- `allocate_and_copy_magic`: The routine stores two values from a table into global variables, allocates memory via Exec AllocMem, copies 96 bytes from a fixed address to the allocated block, and stores the allocated pointer and an offset pointer into global variables.
- `call_two_subroutines`: Four-instruction routine whose pseudo-code could not be produced, so only its listed facts are known.
- `execute_conditional_calls`: Calls one routine, then if the zero flag is not set, calls another routine before jumping to an absolute address.
- `return_without_action`: Performs no work and returns straight to its caller.
- `store_dos_name_bytes`: Writes the four ASCII characters of the text dos.

### Star Wars (1987)(Domark)[cr BS1 - TKT].adf:starwars

- `start_program_with_memory_check`: Program entry that requests memory from the system.
- `clear_return_register`: Clears a register to zero and returns.
- `copy_masked_flag_field`: When a byte field of the structure equals 1 and the high bit of another byte is set, writes the structure's word field, masked to its low seven bits, through the output pointer.
- `swap_buffers_and_update_planes`: Swaps two stored buffer pointers when a pending flag is set and counts a frame counter down.
- `decode_joystick_movement_deltas`: Decodes the direction bits of a joystick data word into horizontal and vertical movement values.
- `read_joystick_fire_and_movement`: Reads the fire line from the CIA port register and the joystick data word.
- `decode_joystick_and_reset_fire`: Sets the fire-state byte to all ones, decodes the joystick data word into movement deltas, and stores them in the joystick state.
- `accumulate_joystick_position_deltas`: Decodes direction bits from a joystick data word into horizontal and vertical steps and adds them to stored position values.
- `accumulate_joystick_x_clamped`: Uses the joystick data word already in a register to derive horizontal and vertical steps, adds them to the stored position, and clamps the horizontal value to the range 0 to 319.
- `accumulate_joystick_steps_clamped`: Adds the joystick steps to the stored position, clamps the horizontal value to the range 0 to 319, and clamps the vertical value to the range 0 to 199.
- `clamp_position_and_countdown`: Clamps the vertical position to the range 0 to 199 and holds the horizontal position at zero or above.
- `countdown_timer_and_reload_record`: Clamps a stored value to be nonnegative and, when a flag is zero, counts a timer down.
- `update_slot_timers_only`: Counts down four timers without changing any position values.
- `read_and_clear_pending_word`: Returns a stored 16-bit value and resets the stored value to zero when it was nonzero.
- `request_handle_into_stack_buffer`: Requests a handle through a system vector.
- `read_library_buffer_field`: Points a stack buffer at a fixed location, stores a constant in it, passes it with a handle to a library routine and then the handle to a second library routine.
- `request_resource_1005_and_release`: Requests resource number 1005 through a system vector.
- `call_indirect_pointer`: Calls a routine through a pointer stored at an offset from a register.
- `request_resource_1005_alt_release`: Requests resource number 1005 and, when it is obtained, passes it and two parameters to a different system vector before releasing it.
- `execute_indirect_pointer`: Fetches a pointer from a memory location calculated by adding an offset to a value in a register and then calls the routine at that address.
- `request_resource_1005_or_1006`: Requests resource number 1005 and, if that fails, requests resource number 1006 instead.
- `call_handle_for_code`: Requests a handle using code 1006 from the first argument, passes it with two stored arguments to a library function and then to a second library routine that takes only the handle.
- `draw_stepped_offset_pairs`: Calls a setup vector, then loops over a count, stepping through a table index.
- `draw_mirrored_offset_pairs`: Repeatedly calls two vectors with a coordinate and its mirror image around a stored centre value, advancing a stepped counter on each pass.
- `draw_mirrored_offset_pairs_loop`: Repeatedly calls two vectors with a coordinate and its mirror image around a stored centre value, updating the step values on each pass until the loop ends.
- `blit_rectangle_rows`: Works out the screen address and row width of a rectangle from stored offsets and two inputs, enables the blitter DMA bit, and blits each row, waiting for the blitter between passes.
- `fill_object_record_fields`: Fills an object record.
- `fill_object_records_per_plane`: Repeatedly fills object records and adds 8000 to a bitplane address on each pass, resetting a count value for each row.
- `advance_seeded_counter`: On first use, seeds a longword from a 16-bit global counter in its upper and lower halves.
- `update_dma_control_bits_2`: Writes to the DMA control register to change channel enable bits.
- `access_dma_control_register`: What it reads or writes cannot be established from the evidence.
- `wait_until_flag_clear`: Busy-waits until a stored word is zero, then returns.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `run_pending_callbacks_then_flag`: Calls each of two subroutines when its stored pointer or count is nonzero, then sets a stored flag word to all ones.
- `call_two_routines_in_sequence`: Calls two subroutines one after the other and returns.
- `blit_rectangle_from_coordinates`: Computes the source and destination addresses and row width of a rectangle from two coordinate pairs, sets the blitter DMA bit, and blits the rectangle in several passes, waiting for the blitter on each.
- `load_blitter_registers`: Waits until the blitter is idle, then loads the blitter pointer, modulo and control registers from stored values.
- `load_blitter_registers_loop`: Repeatedly loads the blitter pointer, modulo and control registers from stored values, one set per pass.
- `fill_fixed_length_region`: Calls a routine with a bitplane address offset from a stored base, a stored value, zero and a fixed length of 45.
- `configure_blitter`: Calls a routine with a pointer retrieved from a table and a value from a table plus an offset of 3720.
- `fill_row_span_from_coordinates`: Calls a routine with the bitplane address of a row, a stored value, zero, and the width between two stored coordinates.
- `fill_row_span_alt_order`: Calls a routine with a stored value, the bitplane address of a row, zero, and the width between two stored coordinates.
- `request_with_flags_or_fail`: Calls a system vector with the given value and a fixed flags value.
- `return_stored_longword`: Returns a longword stored at a fixed offset from its base register.
- `return_stored_longword_2`: This routine returns a longword value stored at a fixed offset from a base register.
- `call_vector_with_two_params`: Calls one system vector, passing on the two parameters it was given.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `setup_interrupt_and_dma_state`: Calls two unnamed routines, a library function, and then two further unnamed routines with one library call between them.
- `store_vector_result_slot`: Calls a system vector with the value two and stores the returned value in a stored slot.
- `setup_input_device_request`: Requests two values from a library routine using fixed codes and calls a fallback when either is zero.
- `pass_stored_blocks_with_sizes`: Walks a six-entry table, multiplying each entry by ten, and passes every non-null stored pointer with its size to one library function.
- `pass_stored_handles_to_library`: Passes a stored value to a library function, calls an unnamed routine, then passes two stored handles to a second library function.
- `copy_bytes_with_overlap`: Copies a block of bytes from source to destination.
- `copy_byte_run`: Copies a run of bytes forward from source to destination, one byte at a time, for the count plus one bytes.
- `copy_words_with_overlap`: Copies a block of 16-bit words from source to destination.
- `copy_word_run`: Copies a run of 16-bit words forward from source to destination, one word at a time, for the count plus one.
- `return_absolute_value_word`: Returns the absolute value of a signed 16-bit input, negating it when it is negative.
- `return_absolute_value_long`: Returns the absolute value of a signed longword, negating it when it is negative.
- `multiply_and_divide_words`: Multiplies two signed 16-bit values, divides the 32-bit product by a signed 16-bit divisor, and returns the remainder in the high word and the quotient in the low word.
- `return_sum_of_squares`: Returns the sum of the squares of two signed 16-bit values packed into its input.
- `return_product_of_words`: Returns the product of two signed 16-bit values packed into its input.
- `sample_three_slot_pairs`: Calls one unnamed routine twice for each of three global slots and stores each returned word in a neighbouring global word, six results in total.
- `shift_word_history_after_calls`: Calls an unnamed routine three times, then saves a word from one global slot, calls another routine and stores its returned word in a second global slot while moving the saved word into the first.
- `sample_into_pair_buffer`: Calls an unnamed routine twice with the same global slot, then calls a third routine, and stores the returned word and the first argument into the two words of the buffer passed in.
- `return_empty_routine`: This routine is empty and performs no operations before returning.
- `normalise_angle_with_offset`: Adds a fixed offset of 360 to an angle-like value and wraps it into a circular range using a divisor of 1440, mirroring values past 720 or 360.
- `lookup_folded_table_value`: Reduces an angle argument modulo 1440 and folds it into a quarter-wave using symmetry.
- `scale_four_offsets_by_divisor`: Adds 600 to a divisor argument, then stores four values, each a sum or difference of global offsets and arguments shifted left by nine and divided by that divisor.
- `scale_two_offsets_by_divisor`: Stores two values, each a global offset plus an argument shifted into a high word and divided by a divisor formed from another argument plus 600.
- `add_packed_decimal_value`: Adds a four-byte packed decimal value into a stored packed decimal number, adjusting each byte for decimal carry.
- `add_bcd_counter_track_max`: Stores its argument in a global longword, adds four bytes of that value into a packed BCD counter with decimal adjustment, and raises a stored maximum to the counter when the counter is larger.
- `increment_packed_decimal_byte`: Adds one to a stored packed decimal byte with decimal adjustment, then calls a subroutine.
- `project_points_perspective`: Projects fifty three-dimensional points from a table onto the screen with a perspective divide by depth plus 600, adding a centre offset passed in as a parameter, and writes the resulting 2D coordinates into an output array.
- `copy_pair_after_call`: Calls a subroutine, then copies two stored word fields into the two words at the address held in a register.
- `apply_linear_transformations`: Performs three separate loops to update elements in a table using different linear coefficients.
- `emit_stepped_coordinate_pairs`: Walks a pair of 16-bit tables, writing coordinate pairs offset by plus or minus 30 for each step, and continues while a called routine returns a nonzero value.
- `project_sixty_records_perspective`: Sets loop counts from a mode argument, optionally copies three-word entries through called routines, then either clears three flag bytes when a global value is below minus 469 or projects a table of sixty records to 2D coordinates by dividing by depth plus 600.
- `call_one_subroutine`: Pseudo-code could not be produced, so only the listed facts are known.
- `transform_and_project_vertex_groups`: Transforms groups of 3D vertices by adding and subtracting a scaled global value from their coordinates, then projects each vertex to 2D by dividing by depth plus 600, repeating over the list of groups until its counter expires.
- `copy_records_and_duplicate_entries`: Copies 16-bit coordinate records from a table into an output buffer, pairing each with a value from a called routine, then duplicates six earlier entries and, when the argument is nonzero, fills further entries with a global value offset by 300.
- `store_word_parameter`: Stores the low word of its argument into a stored word field.
- `select_four_flag_offsets`: Tests four bits of its argument and, for each set bit, replaces a default constant with an alternative value; it writes the resulting values into several global locations and sets four flag bytes to match.
- `store_and_scale_coordinate_pair`: Stores two coordinate arguments in stored fields and computes a third field from one of them, scaled by two and offset by a stored value.
- `compute_boundary_difference_value`: Computes a value from the differences between two inputs and four boundary globals, each doubled, and stores it in a global variable.
- `project_coordinate_with_divisor`: Computes a coordinate from the difference between a stored value and an argument, multiplied by a second argument and divided by a third.
- `project_coordinate_for_mode_two`: Computes a coordinate from the difference between a stored value and an argument, multiplied by a second argument and divided by a third, when bit 2 of the mode value is set.
- `project_coordinate_for_mode_eight`: Computes a coordinate from the difference between a stored value and an argument, multiplied by a second argument and divided by a third, when bit 8 of the mode value is set.
- `return_empty_routine_2`: This routine performs no operations and immediately returns to its caller.
- `trace_line_bits_in_bitplane`: Walks coordinate steps along a line, computing byte addresses and bit positions in the bitplane and setting bits as it goes.
- `trace_line_segment_bits`: Walks coordinate steps along a segment, computing byte addresses and bit positions in the bitplane, and calls three subroutines along the way.
- `plot_bitplane_row_span`: Computes a bitplane address from a start point and a row count, then calls four subroutines for pixel handling along the row span.
- `set_pixel_bit_run`: Computes the bitplane byte address and bit position for a coordinate pair, then sets the bit and steps along the run.
- `set_bit_in_four_planes`: Sets one pixel bit in each of four bitplanes spaced 8000 bytes apart, for a run of rows advancing by a stored stride.
- `set_pixel_bits_in_row`: Computes the byte address and bit position for a coordinate, then sets bits along a horizontal run within the bitplane.
- `draw_slanted_line_at_pointer`: Draws a sloped line into four bitplanes spaced 8000 bytes apart, starting from a buffer address passed in an address register and advancing rows with an error term that tracks the minor axis.
- `compute_bitplane_pixel_bits`: Computes a bitplane byte address and bit position from a coordinate pair, then combines the bit values into the bitplane bytes.
- `set_bits_in_memory`: Sets specific bits in four different memory locations at regular intervals using a loop.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `set_pixel_bits_for_point`: Computes the bitplane byte address and bit position for a coordinate pair and sets the bit at that point.
- `draw_sloped_span_at_pointer`: Draws a sloped line into four bitplanes spaced 8000 bytes apart by plotting short runs of pixels row by row, starting from a buffer address passed in an address register and using an error term to step across columns.
- `store_pair_then_call_routine`: Stores the two 16-bit halves of its argument into two global variables and then calls another routine.
- `store_coordinate_pair_return_value`: Stores the two words of its argument into stored coordinate fields, calls one subroutine, and returns a value.
- `plot_point_if_in_bounds`: Checks whether a point lies inside a rectangle of stored bounds.
- `set_four_plane_bit_single`: Sets one pixel bit in each of four bitplanes spaced 8000 bytes apart, at the address passed in a register.
- `clear_display_longwords_backward`: Zeroes 32-bit words in display memory by writing backward from a row address computed from two globals, using an unrolled sequence inside a nested loop; the decompiled listing is truncated, so only the start of the clearing sequence is evident.
- `draw_fixed_offset_sequence`: Makes a long sequence of calls to a drawing vector with fixed coordinate offsets derived from a parameter and a stored value, and calls one subroutine.
- `project_strip_quads_and_plot`: Projects up to twenty-four 3D points by perspective division with depth plus 600, expands each into a strip of four coordinate pairs in a buffer, then calls another routine for each buffered point that lies within the horizontal bounds.
- `print_menu_labels_fixed_positions`: Calls a sequence of routines with fixed coordinate pairs, then indirect routines; the referenced text includes select a death star, easy, medium, hard, wave 1, wave 3, wave 5, bonus and no bonus, so it lays out labels at fixed positions.
- `draw_countdown_digit`: Converts a stored timer value into a digit by dividing it by 50 and subtracting from nine, then draws the countdown text and that digit with several drawing calls.
- `run_prompt_state_loop`: Draws two positioned text items, then runs a three-state loop until the state reaches its end value, drawing the prompt text that tells the player to pull the trigger to start; the loop is preceded and followed by display-pointer swaps.
- `wait_for_input_or_timeout`: Loops calling system and drawing routines until a flag is set or a frame counter reaches 500, then returns the final state.
- `build_point_table_from_lookup`: Builds a table of 128 points by passing each source value through two indirect lookup calls, shifting the results right, adding offsets of 100 and 2000, and storing the pairs in an output buffer, then calls another routine.
- `draw_credit_text_lines`: Draws the publisher credit, the Amiga version credit, the title text and one further element at fixed positions, using the referenced text lines as the displayed strings.
- `draw_intro_crawl_lines`: Draws eight lines of the opening text at successive vertical offsets from a base row, using the referenced lines about the Empire, the rebel planet and Obi-Wan Kenobi, then draws one further element at a computed position.
- `run_unrecovered_routine_block`: A 47-instruction routine called from two places.
- `offset_point_table_lookup`: Derives two offsets from a fixed-point lookup, adds them to each of 128 entries in a point table, and writes a pair of global words into an output buffer for each entry; it also increments a global when its argument exceeds ten and has its low two bits set.
- `run_unrecovered_routine_block_2`: A 30-instruction routine called from one place.
- `draw_shape_vertex_pairs`: Passes many coordinate pairs from a shape table to indirect drawing routines, offsetting the second value of each pair by a global, so that the table is drawn as an outline; it is preceded by calls that set up the drawing state.
- `poll_until_result_or_limit`: Calls setup routines, then loops up to 100 times calling a status routine and returns its value as soon as it is nonzero.
- `select_mode_from_stored_values`: Calls one routine, then another if its value is nonzero, and returns a small mode code chosen by comparing stored values with its parameter.
- `update_particle_records_respawn`: Updates seventy three-word records by adding a velocity to each position, respawning a record with random values when it crosses a limit, and calls a drawing routine for each record; the argument selects whether the vertical or horizontal limits are tested.
- `randomize_seventy_particle_records`: Fills seventy three-word records with random values, storing the high byte of two random numbers as positions and a value from one to eight as a speed.
- `draw_briefing_text_by_mode`: Selects a display position from a mode number between zero and sixteen, then draws the matching briefing text line through a text routine; the referenced text covers the X-Wing shields, deflector strength, Trench Turrets, the Death Star and the flight instructions.
- `draw_scoring_table_labels`: Draws a scoring table of target names, including Tie Fighters, Laser Towers, Laser Bunkers, Trench Turrets, Exhaust Port and Darth Vader Ship, with their point values such as 1,000, 2,000, 25,000 and 50,000, at fixed row offsets, then draws one final element.
- `run_display_setup_sequence`: Initialises a display state, reads a table value, and computes a result through a short sequence of system calls.
- `run_scene_frames_stepping_copy`: Runs up to 768 frames of a scene with the same stepping loop as its sibling routine, stepping a position parameter by 256 every five frames and returning early with a nonzero code when an input routine reports one.
- `run_display_setup_sequence_2`: Initialises a display state, reads a table value, and computes a result through a short sequence of system calls, using a different starting value.
- `store_colour_table_entry`: Stores the second word of its argument into a 16-entry word table, at the index given by the low four bits of the first word.
- `wait_until_counter_reaches`: Resets a stored counter to zero, then loops until it reaches the value passed in.
- `compute_integer_square_root`: Computes the integer square root of its argument by iterating an estimate until the estimate stops changing.
- `nop_padding`: This routine is empty and performs no operations.
- `store_default_bounds_values`: Stores fixed word values into three stored fields, setting default bounds and a size of 15.
- `store_default_viewport_bounds`: Stores six fixed word values into stored position and bound fields, establishing default bounds.
- `decrement_entries_with_wrap`: Calls a system vector, then loops over 31 stored words, subtracting a step from each and wrapping values that pass a limit.
- `scroll_entries_with_wrap`: Loops over stored entries, subtracting a step from each and wrapping values below minus 500 by adding 14500.
- `scroll_entries_with_wrap_2`: Loops over 31 stored entries, subtracting a step from each and wrapping values that pass a limit.
- `advance_depth_records_counted_wrap`: Moves a counted run of three-word records along depth by subtracting a speed value, wrapping any record above 14000 by subtracting 14500, and draws each record through called routines when a depth test passes.
- `scroll_entries_by_parameter`: Calls a setup vector, then loops over 31 stored entries, subtracting the parameter from each and wrapping values that pass the limit.
- `update_entries_wrap_and_mask`: Loops over entries, subtracting a step and wrapping values below minus 500 by adding 14500 while negating two companion fields.
- `scroll_mirrored_entries`: Loops over stored entries, subtracting a step with wrap and negating companion fields when it wraps, and calls a drawing routine for each entry.
- `advance_counted_records_mirrored`: Moves a counted run of three-word records along depth by subtracting a speed value, wrapping any record above 14000 by subtracting 14500 and flipping the sign of its two velocity words, then draws each record through a called routine.
- `wait_for_counter_then_finish`: Resets a stored counter, calls several routines with the parameter, and waits until the counter is positive before calling a final routine.
- `call_indirect_pointer_2`: The routine loads a pointer from a memory location and calls the address contained within it.
- `remove_expired_list_entries`: Walks backward through a list of six-byte entries, calling a routine for each entry whose key is at least the argument and reducing a count by six, then stores the new list end.
- `scan_entries_within_range`: Scans stored entries from the current position, calling a routine for each entry that passes a range test.
- `insert_sorted_list_entry`: Finds the insertion point for a key in a sorted list of six-byte entries, increments the list count, moves the later entries with a block-move routine, and returns the address of the new slot.
- `build_entry_projection_buffer`: Converts each entry of a depth-sorted list into a three-word output record holding two global values and a perspective scale of 25600 divided by depth plus 600, and draws each record when the argument is nonzero.
- `update_particle_list_expire`: Moves each entry of a particle list by subtracting a speed, applying two velocity words to its coordinates with wrapping within a 4096 range, then removes entries that have passed the depth or vertical limits by calling a removal routine.
- `draw_six_point_ring_shape`: Derives three scaled offsets from a rotation-style table and draws a six-point ring of segments around a centre point, drawing each segment through called plotting routines; a negative argument selects a variant that sets a colour for each segment.
- `update_difficulty_parameters`: Derives three parameters from a level counter, capping them at fixed values once the counter reaches seven, then calls one routine and increments the counter up to 99.
- `run_unrecovered_routine_block_3`: A 10-instruction routine called from one place that calls one other routine.
- `send_word_sequence_to_routine`: Passes each 16-bit word of a table to a called routine in turn, advancing the table pointer and stopping after the count supplied in a data register, then returns the next table address.
- `call_vector_with_two_params_2`: Calls two system vectors, passing on the two parameters it was given.
- `call_eleven_routines_in_sequence`: Calls one routine eleven times in succession, with no other work, and returns.
- `initialise_stored_state_words`: Clears four stored state words, sets a starting value of 16000, and calls several routines to set up the state.
- `run_timed_frame_loop`: Repeats a block of library calls and a sub-routine call while a counter stays under a limit and a flag is clear, adding one to a tally each pass and reducing a stored value by a fixed step when it exceeds a threshold.
- `wait_for_pending_items_clear`: Clears a flag, repeatedly calls a routine until it returns zero and the flag is zero, then makes one final call with a global argument.
- `draw_zigzag_marks_scaled`: Draws a zigzag pattern of short segments whose length scales with its argument, moving between fixed offsets from a centre point through called plotting routines.
- `draw_shield_status_display`: Draws the shield status by choosing one of three bar settings from the shield value, drawing either the shield text or the shield gone text, decrementing a blink counter, and then drawing the remaining status element.
- `apply_shield_value_change`: When a mode flag equals one, adds the signed parameter to the stored shield value and refreshes its display.
- `add_offset_to_shield_value`: Saves the current shield value into one stored slot, sets a second slot to ten, adds a value from the caller's argument block to the shield value, and calls a routine to refresh it.
- `dispatch_pause_and_flag_commands`: Handles four command codes: code 64 waits until a poll returns zero, calls an unnamed routine with the 'Game Paused' text pointer while swapping a stored long value, then waits for the poll to return nonzero before resetting a counter.
- `draw_animated_indicator_frame`: Draws a small marker in one of two states, switching its shape each time state three is entered, and decrements two countdown values while redrawing each countdown at an offset.
- `update_marker_and_countdowns`: Switches the marker shape when state three is active, decrements two countdown values and redraws each countdown at an offset from the marker position, then draws the fixed outline.
- `draw_marker_at_stored_position`: Draws the marker outline and its two countdown displays at a position held in a register, with the same paired moves and draws as the other marker routines.
- `draw_marker_outline_and_countdown`: Draws the first countdown display at a fixed offset from the marker, decrements the second countdown and draws it when it is nonzero, then draws the marker outline.
- `draw_outline_store_result`: Draws a fixed symbol and a fixed outline at positions relative to a base row, then stores a called routine's result in a record and calls another routine when it is nonzero.
- `update_velocity_with_sign_flag`: Computes a scaled horizontal offset from stored positions, reads three values from a library lookup and blends two velocity values toward new targets, clamping them to fixed ranges.
- `update_velocity_from_offsets`: Computes a scaled horizontal offset from stored positions, reads three values from a library lookup and blends two velocity values toward new targets, clamping them to fixed ranges.
- `clamp_and_blend_velocity_values`: Clamps the first velocity to a positive limit and the second to a fixed range, then blends both toward new targets.
- `blend_velocities_and_flag`: Caps the second velocity value at a positive limit, blends both velocity values toward new targets, and when a pending flag is set clears it, counts down a value and passes control to a routine.
- `set_countdown_to_seven`: Sets one stored countdown value to seven and calls a routine with zero, which redraws the countdown display.
- `draw_packed_digit_string`: Draws up to eight digits taken from successive four-bit groups of a 32-bit value, calling a digit routine for each one and placing a separator before every fourth digit.
- `tick_timer_flash_message`: Decrements a countdown timer and, when it reaches zero, draws a message at a fixed position twice with a display buffer swap in between, then calls another routine.
- `draw_timed_message_block`: Draws two fixed text items on a fixed row, starts a countdown of ten, then draws a further fixed-size block at a fixed position through a buffer copy routine.
- `draw_count_symbols_block`: Draws one or two copies of a symbol depending on a count byte, then draws a fixed-size block at a position in the display area.
- `draw_two_digit_count_symbols`: Draws one or two copies of a symbol depending on whether its argument is at least ten, then draws a fixed-size block at a fixed position.
- `draw_five_offset_markers`: Derives a scale and a centre offset from two position values, then draws five markers at offset positions taken from tables, placing each through a routine that draws it at the given coordinates.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `build_angle_lookup_tables`: Advances a base angle by 120 and fills several 16-bit tables from two indirect lookup calls: six entries at successive offsets of 120 plus a few fixed entries, with the second table taking the other lookup's results.
- `plot_points_decreasing_steps`: Plots points at a fixed height of 250 across the horizontal range, with the horizontal step shrinking through the sequence 46, 36, 25, 18, 11, 7 and 4, calling a plot routine for each point.
- `draw_shape_by_selector`: Selects one of ten drawing patterns from a numeric code and, for each, stores a start point and issues a short series of paired move and draw calls at small offsets.
- `execute_glyph_sequence_variant_a`: Stores two 16-bit values into a table at offsets -25346 and -25344 relative to a4, clears a word at offset -25338, then performs four calls to another routine with specific stack-based arguments, updates the table values, and performs a final call before branching.
- `execute_glyph_sequence_variant_b`: Stores two incremented 16-bit values into a table at offsets -25346 and -25344 relative to a4, clears a word at offset -25338, performs two calls to another routine with stack-based arguments, updates the table values, and performs a final call before branching.
- `execute_glyph_sequence_variant_c`: Stores two 16-bit values into a table at offsets -25346 and -25344 relative to a4, clears a word at offset -25338, performs five calls to another routine using various combinations of d4 and d5 as stack arguments, and then branches.
- `execute_glyph_sequence_variant_d`: Stores two 16-bit values into a table at offsets -25346 and -25344 relative to a4, clears a word at offset -25338, performs three calls to another routine, updates the table with incremented values, and performs a final call before branching.
- `execute_glyph_sequence_variant_e`: Stores two 16-bit values into a table at offsets -25346 and -25344 relative to a4, clears a word at offset -25338, and performs four calls to another routine using specific offsets of d5 and d4 as stack arguments before branching.
- `execute_glyph_sequence_variant_f`: Stores an incremented value into a table at offset -25346 relative to a4, stores d5 into offset -25344, clears a word at offset -25338, and performs five calls to another routine with various stack-based arguments before branching.
- `execute_glyph_sequence_2`: Stores two 16-bit values into a table at offsets -25346 and -25344 relative to a4, clears a word at offset -25338, and performs four calls to another routine using modified versions of the second stored value as arguments.
- `execute_glyph_sequence_variant_h`: Stores two 16-bit values into a table at offsets -25346 and -25344 relative to a4, clears a word at offset -25338, and performs two calls to another routine with stack-based arguments before branching.
- `execute_glyph_sequence_variant_i`: Stores two 16-bit values into a table at offsets -25346 and -25344 relative to a4, clears a word at offset -25338, performs four calls to another routine, updates the table with an incremented d5 value, and performs a final call before branching.
- `execute_glyph_sequence`: Stores two values into a table at offsets -25346 and -25344 relative to a4, then performs a sequence of four calls to another routine using values from d4 and d5, followed by a conditional jump based on a comparison.
- `draw_text_string_glyphs`: Draws a null-terminated string one character at a time.
- `render_glyph_control_sequence_2`: Updates a coordinate structure in a memory block by writing values from registers and then calls a drawing routine multiple times with varying offsets.
- `render_glyph_control_sequence_3`: Updates a coordinate structure in a memory block and calls a drawing routine multiple times based on specific offsets and values derived from a string of control bytes.
- `render_glyph_control_sequence_4`: Updates a coordinate structure in a memory block and calls a drawing routine for each glyph control byte in a string.
- `render_complex_glyph_sequence`: Stores two 16-bit values and a zero into a structure at a negative offset from a4, then performs a series of calls to render_glyph_control_sequence using various coordinate offsets derived from d4 and d5.
- `render_glyph_control_sequence_5`: Updates a coordinate structure in a memory block and calls a drawing routine for each glyph control byte in a string.
- `update_glyph_coordinates_and_draw`: Updates a coordinate structure in a memory block using values from registers and calls a drawing routine multiple times with varying offsets.
- `render_glyph_control_sequence_preamble`: Updates a coordinate structure in a memory block by adding offsets to values in d4 and d5, then calls render_glyph_control_sequence multiple times with varying coordinate increments.
- `render_glyph_control_sequence_6`: Updates a coordinate structure in a memory block by adding offsets to values in d4 and d5, then calls a drawing routine multiple times with varying coordinate parameters.
- `render_glyph_control_sequence_7`: Updates a coordinate structure in a memory block and calls a drawing routine for each glyph control byte in a string.
- `render_glyph_control_sequence_8`: Updates a coordinate structure in a memory block by adding offsets to values in d4 and d5, then calls a drawing routine multiple times with varying parameters.
- `render_glyph_control_sequence_9`: Updates a coordinate structure in a memory block by adding various offsets to values stored in registers d4 and d5, then calls a drawing routine multiple times with these modified coordinates.
- `render_glyph_control_sequence`: Iterates through a string of glyph control bytes, updating a coordinate structure in a memory block and calling a drawing routine for each character or special stroke pattern.
- `select_random_entry_and_print`: Clears a flag and, when a counter exceeds two, prints a fixed message first.
- `handle_text_entry_key`: Handles one keypress for a text entry field: a printable character is stored at the current position, a space is stored for code zero, and code one ends the entry while codes two and three add a space or delete the last character.
- `draw_counter_text_item`: Derives a horizontal position from a counter, draws a text item through a lookup call, then clears a flag and calls one routine when the state flag was set.
- `draw_high_score_table`: Draws the high score screen with ten rows of ranks, initials and scores taken from stored records, highlighting the row matched by the parameter.
- `insert_random_score_into_table`: Sums nine values drawn from a library random source and places the total into a sorted ten-entry score table, shifting lower entries down.
- `rank_score_record_into_table`: Accumulates nine random values into the caller's score record, then inserts the total into the sorted ten-entry score table at its rank and shifts lower entries down.
- `insert_score_at_rank`: Walks the sorted ten-entry score table from the top and, at the first entry the stored score is at least as high as, shifts the lower entries down and writes the score into that position.
- `open_system_libraries_setup`: Opens system libraries and creates device and timer request structures through library vector calls, setting their fields and issuing requests; it calls an error routine whenever an open or allocation returns zero.
- `clear_flag_and_call_library`: Clears a global variable to zero and calls one routine with a fixed 32-bit constant, which is a library vector call at a fixed offset from a loaded library base.
- `initialize_data_structure`: Modifies bits in global variables and populates a table with several constant values and a pointer.
- `reset_level_state_draw_hints`: Resets scoring, timer and motion globals for a new round, sets a speed-dependent value from the level counter, and when a flag is set draws the Shoot Fireballs and Shoot Tie-Fighters hint text at fixed positions.
- `advance_records_by_rotation_offsets`: Moves thirty-one three-word records along depth by subtracting a fixed step, flipping velocity signs when a record wraps, rotating its two position words by offsets masked to a 4096 range, and draws each record through a called routine.
- `randomize_object_position_fields`: Fills three position fields of an object record with values from two lookup calls, each shifted right by two bits, after applying a fixed offset to a size field.
- `spawn_random_object_slot`: When the object count is below its limit and a timer threshold is met, claims the next record, fills its position and velocity from random values, sets its lifetime and flags, and draws a lookup entry for that lifetime.
- `update_active_object_list`: Updates each active object in a list: counts down its timers, moves it by its velocity, applies random speed and shot adjustments when timers expire, and removes it from the list when it passes the lower depth limit, then updates the list-level state.
- `launch_object_projectiles`: For each active object inside the target bounds, when a random roll passes, claims a free projectile record and sets its position, a direction flag and velocity components scaled by the object's distance.
- `call_alternating_pair_sequence`: Makes a fixed sequence of thirteen calls that alternate between two routines, with no other work, and returns the incoming data registers unchanged.
- `draw_record_outline_shape`: Draws a fixed outline shape by stepping through points taken from an object record's coordinate fields, plotting them with two routines, and then draws a short second stroke.
- `draw_object_sprite_strokes`: Draws an object's sprite from flags in its record: one or two strokes offset from a base position, then, when a further flag is set, four blocks of five values and five more entries through called routines.
- `check_object_hits_and_draw`: Checks each active object against target bounds and a random chance; on a hit it reduces the object count, plays a hit effect through a call and adjusts its timers, then updates the list pointers and draws each object.
- `remove_random_object_in_range`: Scans the object list for the first record within the depth and screen bounds that passes two random tests, removes it by moving the rest of the list down, decrements the count and calls a routine.
- `update_hud_counters_and_flags`: Derives display values from two status globals and sets two lookup values; once the level counter reaches 100 it subtracts scaled rates from two running counters, and when a flag is set it toggles a byte, resets a countdown and calls a routine.
- `draw_timed_marker_blink`: While a flag is set and a frame counter is below 320, draws a small marker whose vertical position depends on a status bit, then advances the counter.
- `draw_score_readout_values`: Draws a score or status readout: below level 111 it draws a value from a lookup with a decreasing offset, and otherwise it converts two running totals into lookup values and draws the resulting figure at a fixed position.
- `draw_active_object_markers`: Draws one marker for each active object at a fixed level of 300, using each object's stored value for position and stepping through the object records.
- `run_round_until_end`: Runs frames of a round while a state flag is zero and a timer stays non-negative, waiting for each frame to pass; when the state flag is set it performs an end-of-round action, clears a pending flag, and returns the state flag value.
- `scale_lookup_by_shift`: Passes its argument to one lookup call, takes a second lookup of the constant 10000, shifts that result right by 13 bits, and returns the value.
- `forward_call_without_arguments`: Calls one routine with no arguments and returns, passing nothing on to its caller.
- `append_word_pair_to_buffer`: Stores two 16-bit values at the current position of a shared output buffer and advances the buffer pointer by eight bytes.
- `build_geometry_table_entries`: Fills a table of longword pairs from lookup calls: thirty-one pairs stepped by 48, ten further entries, then sixteen pairs stepped by 96 that receive offsets of 7500 and minus 10000, followed by twenty-one routine calls with fixed constants.
- `initialise_random_record_table_spread`: Initialises thirty-one three-word records with a horizontal position starting at minus 259 and stepping by 483 per record, and random vertical and depth values each shifted right by 12 bits.
- `delay_after_ten_calls`: Calls one routine ten times with the same global argument, then spins in a countdown loop of two million iterations before returning.
- `clear_and_load_shape_records`: Clears the first longword of six records, then calls a loader six times with the addresses of those records; the referenced file names are laser, nose and star shape files.
- `load_resource_into_record`: Looks up a named resource through a library call, stores the returned handle and a word value into the caller's record, then uses the returned value and the handle in a second library call to finish the load.
- `load_sound_assets_pack_digits`: Loads the sound and music resources by name, including the Death Star, exhaust, hit, laser and missile sounds and the music file, clears an eleven-entry table, and converts an eight-character number into four packed-decimal bytes.
- `allocate_buffers_init_tables`: Clears a set of global pointers, allocates buffers of 32000, 4000, 1000 and 800 bytes through a memory routine, and fills a sixteen-word table with fixed values that include pairs of repeated nibbles.
- `show_return_key_prompt`: Shows a prompt asking the user to press the Return key, with a list heading.
- `reset_game_state_globals`: Sets a large set of game globals to defaults, including speed and position values, a wave value of 17 and a timer of eight, clears a 31-entry table, then calls routines to reset the display area.
- `draw_score_wave_headers`: Sets game flags and resets two counters, then draws the score and wave headers through called routines, clearing a status area and drawing two further labels at a fixed position.
- `wait_for_input_event_loop`: Loops reading input events through called routines until one of them returns zero, then calls a finishing routine and returns, polling a second set of inputs while it waits.
- `load_image_record_from_file`: Opens a resource by identifier, reads a ten-byte header to derive width, height and data size, allocates a buffer, reads the image data into it, closes the resource, and fills a record with the size, offsets and buffer pointers.
- `load_sprite_planes_with_masks`: Loads a sprite resource and rearranges its rows into interleaved bitplane buffers, writing an all-ones mask word before each row and filling the mask and plane buffers for every sprite row.
- `blit_clipped_masked_sprite`: Clips a sprite against the screen bounds, then programs the blitter control and size registers for a masked copy of its bitplanes with the computed modulo values, waiting for the blitter to be free before each start.
- `load_sound_sample_tables`: Loads the trumpet sound resource, builds five sample pointer and length tables, fills a table with twelve note period values and twelve random values, and fills a 120-entry table with scaled lookup results.
- `locate_sample_chunks_in_file`: Reads a chunked resource and walks its chunk headers, skipping odd-length padding, to record the data pointers and sizes of two sample chunks.
- `advance_music_playback_step`: Advances music playback on each tick: when its countdown reaches zero it reads two bytes from the sequence and stops at the end marker.
- `play_channel0_note_from_table`: Looks up a pitch period and a sample pointer for a note index from tables, then writes the channel address, length, period and volume registers.
- `advance_channel0_sequence_step`: Advances the channel sequencer on each tick: when its countdown ends it reads two bytes, stops at the end marker, and for note bytes looks up a value from a table and passes it to the channel-trigger routine.
- `play_channel1_note_and_timing`: Looks up a period for a note index, writes the second channel's sample address, period and volume and enables its DMA. It then sets the channel countdown from a table, doubling or dividing it according to flag bits in the next byte.
- `set_channel1_short_sample`: When the countdown is below three, points the second channel at a sample base offset by 120 and sets its length to a fixed short value.
- `initialise_music_playback`: Resets both music sequence pointers to their starts, clears the timer and duration globals, enables the audio DMA channels and interrupts, and marks playback as active.
- `stop_music_and_disable_channels`: Disables the first two audio channels in the DMA control register and clears the music playing flag.
- `set_music_playing_flag`: Sets the music playing flag word to one.
- `get_music_playing_flag`: Returns the music playing flag word, which callers use to check whether music is still active.
- `map_code_to_routine_result`: Takes a code and calls one routine for the codes 0, 2, 4, 6, 8, 12 and 19, returning that routine's result; for any other code it returns the code minus 19 without calling anything.
- `initialise_round_state_and_layout`: Resets round state, derives scoring values from a level counter, fills 26 records with random positions, loads a level layout from a table, counts marker entries, and for later levels draws the tower-clearing bonus text and status shapes at fixed positions.
- `update_and_compact_entry_list`: Moves each entry in a list by a speed and an acceleration, adds a random jitter to some entries, then removes entries that fall below a threshold by compacting the list.
- `scroll_entry_ring_respawn`: Scrolls a ring of records by a random step, appends a fresh record when the leading record passes a threshold, rotates a five-entry group between two sets, and periodically refreshes a pattern table from a stored list.
- `generate_row_entries_both_sides`: Generates entries for five rows by scanning from each side of the screen and placing an entry wherever a random test passes, storing its position, type and a value derived from its depth.
- `draw_tower_entity_by_type`: Draws one tower record according to its type field: type two draws a fixed set of point and line calls placed from the record's coordinate fields, and other types draw their own outline or move the record first.
- `spawn_marked_entries_from_list`: For each entry of a type below three that is not already marked, when a random test passes, marks it and creates a new entry at an offset position within the list's range.
- `check_entry_hits_and_draw`: Checks the entries of a list against a target position and type, and on a hit marks the entry, plays a sound effect through a call and sets a flag; it then draws the entries at their positions.
- `claim_nearby_entry_when_clear`: Walks the list of records and removes the first one ahead of a threshold that passes two random tests, starting a timer; when a flag is clear it also scans for a secondary condition on the remaining records.
- `update_scroll_offset_and_lookup`: Adjusts a scroll offset that eases toward a target or accelerates while a flag is active, then looks up two values at that offset and stores them as scaled offsets.
- `update_scroll_velocity_damped`: Sets motion globals from a level counter, damps a velocity value toward a target, and when a flag is set toggles a byte, resets a countdown and calls a routine.
- `draw_blinking_indicator`: Draws a blinking indicator at a fixed position: when a timer is not running it draws a symbol on alternate frames, otherwise it chooses one of two symbols from a status bit, and it advances the frame counter.
- `draw_frame_with_optional_overlay`: Draws the background and foreground layers through called routines, then draws two overlay items when the level is above two, a counter is below 500 and a flag is clear, and finishes with a final call.
- `run_tower_height_loop`: While a stored height is above 5000, adjusts a stored value and steps through fifty entries, updating a speed value and redrawing each entry.
- `project_two_depth_offsets`: Divides two stored 32-bit values by two depth-dependent denominators offset by 470 and 730 units, and stores the four 16-bit quotients in globals.
- `project_offset_pair_by_depth`: Divides the argument offset by 30720 above and below, using two depth-dependent denominators offset by 470 and 730 units, and stores the four quotients in globals.
- `project_triple_values_by_depth`: Divides three stored 32-bit values by two depth-dependent denominators offset by 450 and 750 units, storing the six quotients in globals.
- `project_offset_triple_wide`: Divides the argument by denominators derived from a depth value to compute two quotients offset by plus and minus 46080 and a third quotient, storing the three results in globals.
- `draw_tactical_hint_text`: Draws hint text lines chosen by the level, including Use the Force, Exhaust Port Ahead, Exhaust Port Missed and Avoid Catwalks, then draws a block of cells that cycles through three marker values.
- `initialise_column_positions`: Resets scroll and count state, points two list pointers at their buffers, and fills eleven column slots with increasing positions stepped by 1900, each paired with a second value 950 further on.
- `scroll_column_list_tick`: Scrolls eleven column positions and a set of records, recycling the first column off the left edge by shifting the list and appending a new column 1900 further along, updates two projection values from a rotation global, and compacts records that have left the screen.
- `draw_column_tile_pattern`: Draws a vertical column of tiles between two rows derived from a mode value, plotting sixteen-entry tile patterns through lookup and plot calls for each row.
- `show_bonus_score_screen`: Draws the end-of-wave bonus screen with the referenced lines about the Death Star being destroyed, starting bonus points, shield bonus and shield strength, builds a bonus line from the shield count, flashes the display over several timed frames, then clears it.
- `test_shot_hit_region`: Tests whether a shot lies within a target box for the flagged regions, and on a hit clears the active flag, toggles a status byte, calls a hit routine and starts a sound effect.
- `draw_tower_object_by_code`: Draws a shape selected by a type code: code twelve draws a fixed pattern of line calls with a register-driven value, code eight draws a line from stored coordinate fields, and other codes draw nothing.
- `draw_masked_nibble_markers`: Walks the four-bit groups of a mask and draws a marker pair for each group with its low bits set, moving down the column by a fixed step per group and returning the last drawn value.
- `save_positions_and_set_limit`: Copies four position words into backup globals and calls an unnamed routine with a value taken from a table indexed by a counter, or the fixed value 18400 once the counter passes eight.
- `check_trigger_target_hits`: Checks the target flags of the current entry against a range test and a random chance; when a flag bit matches and the position is inside the shot window, it sets a state value, plays a sound through a call and marks the entry as hit, then updates the active flag.
- `draw_moving_target_sprites`: Draws moving target sprites and their trailing strokes at computed positions, advancing their timers, playing a sound effect and spawning a new marker when a random test passes.
- `process_entries_below_threshold`: Repeatedly calls one routine for each list entry whose key is below the argument, advancing the list pointers past each one.
- `spawn_lane_entries_check_hits`: Spawns new list entries in left and right lanes when random tests pass and the state allows, rebuilds the table of entry positions, then checks each entry for a hit within a window and, on a hit, clears the flag and plays a sound effect.
- `draw_periodic_marker_sprites`: Draws marker sprites at fixed positions chosen by a status flag and level counters, using a timing counter so the markers appear on a rhythm, with a third marker drawn when a flag is set.
- `draw_trench_frame_entries`: Draws the trench frame: plots two rows of marker strokes, draws lane markers from a counter, then walks the list of six entries drawing hits and each entry in range through called routines, and rebuilds the list pointers.
- `draw_projected_wireframe_frame`: Projects a wireframe of fifty-seven vertex pairs from stored rotation values into screen coordinates, scaling each vertex by depth, then plots the wireframe segments through called routines.
- `animate_sweep_steps`: Steps an animation parameter from minus 720 up to zero, advancing by forty, twenty or twelve as it passes thresholds, and in each step sets a value from a random routine, draws a frame and waits for a frame to pass.
- `run_frames_until_flag`: Runs frames, each drawing through called routines and waiting for a frame, until a hit flag or a status flag is set; it then clears the sequence state and sets the end flag when no further hit is pending.
- `run_display_loop_until_flag`: Repeats a display and wait cycle, alternating between two display routines according to a level counter, until an input or status flag is set; it then clears a pending flag and returns the status value.
- `return_state_block_address`: Returns the address of a fixed block of storage held relative to the program base.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `clear_table_and_open_libraries`: Zeroes a long block of longwords, records the exec library base in a global, and checks a flag bit in a library structure before calling a library vector.
- `stub_empty_return`: Contains only a return instruction, so calling it does nothing.
- `allocate_and_fill_file_record`: Allocates a block sized from a stored index multiplied by six, sets its header fields and writes a four-character tag into it.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `write_counted_text_line`: Builds a text line from a counted string held in a stored structure and a fixed separator, writes it through the output wrapper, and then writes the caller's value after it.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a given count or until a null byte is copied, then fills the rest of the count with zeros.
- `append_string_to_destination`: Moves to the end of the destination string and appends the source text one byte at a time, stopping at the terminating null or after 32766 bytes, and then terminates the result.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_to_buffer`: Copies bytes from a source string to a destination buffer, including the terminating zero byte, and returns the original destination pointer.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `open_icon_library_resource`: Opens the icon library through the library lookup chain, reads a resource with a fixed ID from it, and stores that resource's pointer and a second value in the caller's record.
- `format_string_to_buffer`: Stores a destination pointer, runs a formatting routine with a character output callback and the caller's arguments, then writes a null terminator at the end of the output.
- `format_into_buffer_short`: Stores a destination pointer in a global, runs a formatting routine that emits characters through the byte-writing callback, then writes a terminating zero at the destination.
- `write_char_to_format_buffer`: Writes one byte to the current position of the formatting destination, advances that position by one, and returns the byte written.
- `forward_format_to_formatter`: Passes its arguments and the character output callback to the formatting routine without any other change.
- `invoke_two_subroutines`: A routine consisting of 48 instructions that calls two other routines and returns a 64-bit value.
- `format_text_string_with_output`: Walks a format string, sending plain characters to an output routine and expanding each percent specification by reading its argument from a list, with space padding as the default.
- `perform_signed_division`: Reads two signed inputs and sets a sign flag from their signs, calls another routine, and negates that result when the flag is set.
- `negate_result_if_negative`: Calls one routine and negates its result when the first input is negative, then returns the result together with the second input.
- `call_and_return_concatenated`: Calls one routine and returns a 64-bit value built from the register value left by that call together with the original second register value.
- `recursive_call`: The routine calls itself and then returns.
- `perform_32bit_division`: Calculates the quotient and remainder of a 32-bit integer division using inputs in registers d0 and d1, returning the result in a concatenated 64-bit value.
- `compute_string_length`: Walks a zero-terminated byte string from its start to the terminator and returns the number of bytes before it.
- `call_subroutine`: The routine calls another routine and then returns.
- `call_three_helper_routines`: The decompiled body is unavailable, so this description rests on the listing alone: the 21-instruction routine calls three other routines and is reached both by a direct call and by a taken address.
- `call_two_unnamed_routines`: The decompiled body is unavailable, so this description rests on the listing alone: the 15-instruction routine calls two other routines and is reached by a direct call and by a taken address.
- `process_fixed_range_in_steps`: Calls one routine for each slot in a fixed address range, stepping forward by twenty-two bytes each pass until the end of the range is reached.
- `clear_record_resources_by_flags`: Returns minus one for a null record.
- `flush_or_refill_stream_buffer`: Operates on a buffered stream record: it returns minus one when an error bit is set, and a request value of minus one clears the buffer and returns zero.
- `allocate_free_record_slot`: Scans a table of twenty-two-byte records from its start and returns the first record whose flag byte is clear, after zeroing its three longword fields.
- `allocate_record_from_pointer`: Scans forward from the pointer held in a register through twenty-two-byte records for the first free one, clears its three longword fields and returns it.
- `clear_three_word_record`: Zeroes the three longword fields of the record addressed by a register and returns nothing.
- `init_stream_buffer_record`: Initialises a stream record: when an allocation check returns zero it sets a size of one and points the buffer at an area inside the record.
- `free_linked_record_list`: Walks a singly linked list of records from a global head pointer, releasing each record through a routine with its stored size plus eight.
- `allocate_and_link_record`: Allocates a block sized to the parameter plus eight bytes through a library call.
- `forward_low_word_argument`: Passes the low 16-bit word of its argument to another routine and then returns.
- `unlink_and_free_record`: Searches the global linked list for the record whose header matches the given pointer, returning minus one when none matches.
- `check_indexed_entry_present`: Checks that an index is within the table count and that its entry is present, setting an error code of two and returning minus one when it is out of range or empty.
- `delete_file_with_error_report`: Calls a file deletion wrapper with its argument.
- `call_two_routines_by_pointer`: The decompiled body is unavailable, so this description rests on the listing alone: the 19-instruction routine calls two other routines, is called by five, and has its address taken.
- `check_break_signal_flag`: Checks the break signal bit through a library call with a mask for that bit.
- `execute_output_and_callback`: Performs an output operation and then calls a function through a pointer stored in a table.
- `call_optional_hook_routine`: Calls a hook routine through a stored pointer when that pointer is set, then calls another routine and returns.
- `shutdown_and_release_resources`: Releases the table of records by calling a routine for each entry and then once for the whole block.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_indexed_file_entry`: Validates a file-table index and returns minus one, setting error code two, when the index is out of range or the entry is empty.
- `call_close_handler_from_table`: Calls a routine selected from a jump table through a stored pointer at a fixed offset.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_examine_handler_from_table`: Calls a routine selected from a jump table through a stored pointer at a fixed offset.
- `call_examine`: Wrapper that calls the dos library function Examine and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_lock_handler_from_table`: Calls a routine selected from a jump table through a stored pointer at a fixed offset.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `call_open_handler_from_table`: Calls a routine selected from a jump table through a stored pointer at a fixed offset.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read_handler_from_table`: Calls a routine selected from a jump table through a stored pointer at a fixed offset.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_unlock_handler_from_table`: Calls a routine selected from a jump table through a stored pointer at a fixed offset.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_write_handler_from_table`: Calls a routine selected from a jump table through a stored pointer at a fixed offset.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_entry_from_table`: Calls a library routine selected by a fixed offset from a stored library base.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `allocate_cleared_handle_record`: Reads a value through a library call and allocates a cleared 34-byte record through the system memory allocator.
- `allocate_cleared_record_from_frame`: Allocates a cleared 34-byte record through the system memory allocator, stores values from the caller's frame into it and sets a type code of four.
- `fill_existing_record_fields`: Fills an existing record addressed by a register with values from the caller's frame and sets its type code to four, then calls a routine to record the handle.
- `call_routine_with_record_end`: Passes the address of a record's end field at a fixed offset to a routine and then returns.
- `release_record_block`: Clears the record's marker byte and two longword sentinels, calls another routine on the record's byte field when its pointer field is set, and then frees the record block through a library call.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `forward_argument_with_constant`: Takes one argument and calls another routine with that argument together with the fixed constant 48, then returns nothing.
- `forward_pointer_argument`: Passes its single argument unchanged to another routine and returns nothing.
- `allocate_entry_with_fields`: Returns zero when the first argument is zero.
- `reset_entry_and_call_library`: If the argument is non-zero, sets its status byte to 255 and two longword fields to minus one, then calls a library routine with the block and a word field from it.
- `call_library_function_stub`: Loads a library base from a global variable and calls the library function at offset minus 198 from it, passing no arguments of its own.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_function_stub_2`: Loads a library base from a global variable and calls the library function at offset minus 294 from it, passing no arguments of its own.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_function_stub_3`: Loads a library base from a global variable and calls the library function at offset minus 132 from it, passing no arguments of its own.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_function_stub_4`: Loads a library base from a global variable and calls the library function at offset minus 210 from it, passing no arguments of its own.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `initialise_self_referencing_header`: Stores its own address plus four in the first longword, clears the second longword and stores its own address in the third longword.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_function_stub_5`: Loads a library base from a global variable and calls the library function at offset minus 552 from it, passing no arguments of its own.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `setup_disk_track_dma`: Sets up the floppy disk controller for a track transfer: it disables interrupts around a timing loop, writes a sync word and the buffer address and length to the disk registers, and enables interrupts again.

### Star Wars (1987)(Domark)[cr BS1 - TKT].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_minus_one_stub`: Returns the value minus one and does nothing else.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_9`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_10`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_8`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_11`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_9`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_12`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_10`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_11`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_13`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_12`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_13`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_14`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_15`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_14`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_return_stub`: Does nothing and returns immediately.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_global_pointer`: Stores a 32-bit constant into a global variable and restores several registers from a stack location before returning.
