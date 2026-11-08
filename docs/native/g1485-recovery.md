# Game 1485 (defender (19xx)(g. mcardell)(pd)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01485). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 20 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 465 of 465 named (51 author symbols), 465 of 465 described (7 by rule, 456 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 693 of 693 (100.00%) |
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
| Defender (19xx)(G. McArdell)(PD).adf:Defender | 6 | 5016 | 52 |
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
| Defender (19xx)(G. McArdell)(PD).adf:bootblock | 1 | 18 | 1 |

## Cracktros (kept, not protection)

- none

## Copy protection found

- `unknown_routine` (More): The routine contains 175 instructions and is reached only via an unreached code gap. Kind: dispatch/callback-based integrity check. Pass path: u1__draw_text_and_update_window.

## Functions


### Defender (19xx)(G. McArdell)(PD).adf:Defender

- `initialize_game_loop`: The routine clears a global return message flag, calls another routine via a table, checks a field in the returned pointer and conditionally calls two other routines via the table, then calls start.
- `returnMsg`: The routine opens the DOS library, attempts to open a file twice and, on success, XOR-decrypts two 96-byte blocks in memory before closing the library.
- `start`: The routine opens DOS, reads a high-score file, XOR-decodes a table referenced by the text s:defender_scores, writes it back, closes DOS, then initializes hardware registers and enters a loop that updates audio, map, shots, bobs, scan and score, handles pause and game stop, and on exit writes the table back to the file and closes DOS.
- `titles_etc`: The routine clears and resets a set of global variables and tables used by the game, updates counters and timers, and prepares display planes and blitter registers for the next wave.
- `p7_j4`: The routine walks through a table of bytes, accumulating nibble values into a global variable and writing a combined byte to another global variable.
- `textype`: The routine iterates over a null-terminated word table and for each entry writes eight bytes from a font table into three bit-plane buffers, using a running offset that is advanced by a fixed stride each row.
- `audio`: The routine manages a software sound queue for the Amiga audio hardware.
- `audio_j7`: The routine clears a sound flag and walks a per-channel table, decrementing a counter for each active entry and clearing the entry when the counter reaches zero.
- `next_wave`: The routine updates a wave timer and, when it reaches zero, reads a byte from a level table using a position index and writes derived values into an enemy list entry and related fields.
- `drawbobs`: The routine iterates over a linked list of bob descriptors stored in a global table.
- `drawscan`: The routine waits for a hardware flag in a global variable, then writes several 16-bit control words and pointers into a blitter control block pointed to by a0, configuring the blitter for a scanline copy.
- `drawicons`: Iterates over a list of enemy entries; for each active entry it computes a screen position, selects icon pattern bytes from tables, shifts them and ORs them into several playfield bitplanes and associated collision bytes.
- `drawmap`: The routine computes a scroll mask from a global map offset and, when a flag is set, programs the Amiga blitter registers to copy map tiles into a plane buffer.
- `your_shots`: Iterates over a fixed set of laser slots and creates a new laser when a fire flag is set, storing position and direction in per-slot fields.
- `you`: The routine updates a global ship position and velocity using control flags and thrust, clamps values, computes a map offset and ship image index, writes thrust to a hardware audio register, stores the ship coordinates into a structure via a pointer, and clears thrust when a control bit is set.
- `control1`: The routine reads a word from an address pointed to by a0 plus an offset and uses bits from it together with a global variable to compute a control byte.
- `control2`: The routine reads several keypress bytes and sets bits in a control variable according to which keys are pressed.
- `rez`: The routine updates a per-entry timer and state in a table pointed to by a1 with an index in d1.
- `drawbits`: The routine iterates over a set of bit rows taken from a table and from a global variable, computes screen offsets from two input values and a running y coordinate, and ORs the extracted bits into three bit-plane buffers at the computed positions, writing each word and the word 80 bytes below it.
- `stars`: The routine iterates over a table of star entries and updates each entry’s horizontal position relative to a map offset.
- `addlife`: The routine waits for the blitter to be idle, then if a lives counter is less than six it programs the blitter registers for a copy operation and sets up source and destination pointers derived from a global variable, a playoff value and the lives counter, with a fixed blitter control word.
- `addsmart`: The routine waits for a hardware flag to clear, checks a smart count limit, programs a blitter operation by writing to custom chip registers, waits again for the flag, sets up source and destination pointers from a global variable and a table offset based on a playoff value and a smart index, and writes a blitter control word.
- `takelife`: The routine checks a lives counter and, if in range, waits for the blitter to be idle twice, programs the blitter control and mask registers, sets the blitter source and destination pointers using a table indexed by a playoff flag and the lives counter, and starts a blit.
- `takesmart`: The routine checks a counter and, if in range, waits for a blitter busy flag to clear, programs blitter control and source/destination registers for a copy operation, waits again for the blitter to be idle, then sets the blitter source pointer to an entry in a table indexed by a smart counter and a playoff offset, and starts the blitter.
- `score_draw`: The routine checks a global variable against itself and calls another routine if they differ.
- `dwarem`: The routine copies bytes from a table pointed to by a1 using indices derived from d0 and d1 into two destination buffers pointed to by a2 and a3.
- `new_vblank`: The routine reads the Amiga interrupt enable and request registers, masks the result and conditionally calls another routine for the copper interrupt and another routine for the vertical blank interrupt.
- `vblint`: The routine writes a constant to a global variable and returns.
- `copint`: The routine runs when a flag is set, clears the flag and a global variable, updates animation counters and scroll state, then calls several subroutines for drawing and game logic.
- `keyboardint`: The routine reads a global variable and, if a flag bit is set, clears the flag, extracts a key index from the variable, and updates a last-key variable and a keypress table entry for that index.
- `key_j1`: The routine clears a global variable and a byte through a pointer, then spins until a global variable has a specific bit set, clears that global variable and sets the same bit again, and returns the value on the stack.
- `enemy`: The routine walks a table of enemy entries and updates per-entry fields, handling termination when a sentinel is found, resetting global state, and invoking another routine for life and smart counters.
- `scores`: The routine adds an unsigned byte from a register to a global BCD score variable using binary-coded decimal addition with carry propagation, then adds a second byte from a register with carry, adjusts the result to BCD, and stores the intermediate byte into a global variable.
- `set_bomb`: The routine checks a set of conditions involving a global variable, a counter and a map coordinate.
- `bombs`: Iterates a bomb list, decrements a timer for each entry and removes expired entries.
- `set_shot`: The routine checks a global shot index and a distance derived from a map save value and a coordinate.
- `shots`: The routine iterates over a fixed-size shot list and updates each active entry.
- `change_screens`: The routine toggles a screen flag and updates several global variables that hold plane base addresses and control words.
- `clear_screen`: The routine waits until a global variable indicates the blitter is idle, then programs the Amiga blitter registers to copy a solid fill pattern into the first bitplane memory.
- `initialise`: The routine saves registers, kills the system, changes the vertical blank handler, sets interrupts, sets the screen, and creates a new copper list.
- `setscreen`: The routine stores constant values into a set of global variables that appear to hold palette and tile palette entries.
- `newcopper`: The routine stores the address of a copper list in a global variable and writes a value to another global variable.
- `change_vblank`: The routine stores a new address in a global variable and then writes a value to the INTENA register.
- `setints`: The routine stores a pointer to another routine in a global variable, writes a byte to a global variable, then writes a byte and a word to hardware registers for the CIAA chip, clears the CIAA interrupt control register, and masks a bit in the CIAA control register A. It is called by initialise and calls another routine.
- `close_down`: The routine calls another routine to kill the system and then calls another routine to revive the system.
- `save_registers`: The routine opens the graphics library and saves the current view pointer, a copper list pointer, and several hardware register values into global variables.
- `kill_system`: The routine writes a constant value to a global variable twice and then returns.
- `revive_system`: The routine stores several saved values into a global variable, then waits until a bit in the CIAA_PRA register is set, calls a function through a pointer derived from the graphics library base, and then calls closegraphics.
- `closegraphics`: The routine calls a function pointer stored in a global variable and then returns.
- `opengraphics`: The routine repeatedly calls a function pointer stored in a global variable until the returned pointer is non-zero, then returns.
- `opendos`: The routine repeatedly calls a function pointer stored in a global variable until the returned value is non-zero.
- `closedos`: The routine calls a function pointer stored in a global variable to close the OS library.

### More

- `parse_command_line`: The routine obtains a pointer to a command-line argument block, checks for its presence and stores the argument count and pointer in a global structure.
- `restore_task_state`: The routine saves a pointer from a task control block, temporarily replaces a field with the argument, and conditionally calls other routines through a table in a global variable, then restores the saved pointer and returns it.
- `parse_file_header`: The routine parses a file header and populates a table with file metadata.
- `iterate_table_and_recurse`: The routine loops four times, each time calling another routine with a fixed first argument taken from the parameter block and a second argument taken from a global variable table indexed by the loop counter.
- `validate_input_buffer`: The routine calls another routine twice with arguments derived from the input pointer and then scans up to a limited number of bytes at an offset from the input pointer.
- `handle_input_key`: The routine reads a key code from another routine, applies several flag-based filters, and then dispatches to different handling paths depending on the key value, updating fields in a structure passed by the first parameter.
- `unknown_routine`: The routine contains 175 instructions and is reached only via an unreached code gap.
- `initialize_entity_state`: The routine initializes fields in a structure pointed to by param_1 and invokes another routine with values taken from that structure.
- `update_entity_and_increment_counter`: The routine calls another routine twice with arguments derived from the input pointer, stores the second result into a field of the pointed-to structure, and if a flag field is zero it increments a short located at an offset from a pointer stored in the structure.
- `consume_buffer_and_update_window`: The routine checks a size field in a structure and, if the requested amount fits or can be partially satisfied after calling another routine, it updates a scrolling window structure using two offset parameters.
- `process_entity_list`: The routine iterates a bounded number of times, calling another routine with the entity pointer and a table entry, and checks a short range stored through a pointer in the entity.
- `handle_search_loop`: The routine uses a flag at an offset in the argument block to choose a global variable and repeatedly calls another routine with parts of the block, updating a counter at an offset and looping while a search result is zero, then finalises the search with another call and returns a status.
- `initialize_handler`: The routine runs a sequence of calls to another routine using arguments taken from a parameter block and a global variable.
- `check_flag_and_call`: The routine calls another routine with two arguments, then checks a short value at an offset from a pointer and conditionally calls another routine with one argument.
- `draw_text_and_update_window`: The routine uses a handler pointer stored in the object to call drawing primitives and iterates over a text buffer at a fixed offset, drawing each line.
- `draw_text_and_update_window_2`: The routine uses a handler pointer stored in the object to call drawing primitives and iterates over a text buffer at a fixed offset, drawing each line.
- `draw_text_line`: The routine uses a handler pointer stored in the object to call drawing primitives and iterates over a text buffer at a fixed offset, drawing characters one by one while tracking line count and stopping on form feed or a flag.
- `dispatch_by_flag_and_recurse`: The routine selects two pointers based on a flag in the parameter block and then calls another routine with those pointers and a value from the block.
- `set_flag_and_forward`: The routine checks a field in the passed structure.
- `dispatch_by_flag`: The routine selects two pointers based on a flag in the parameter block and then calls another routine with those pointers and a value from the block.
- `dispatch_handler_by_flag`: The routine reads a flag from the parameter block and, depending on whether the flag is zero, calls another routine with either three references to a global variable or with a value taken from a table entry pointed to by the block.
- `clamp_negative_result`: The routine calls another routine twice with the same first argument and different third arguments, then clamps a negative return value to zero before returning it.
- `consume_from_buffer`: The routine checks a size field in a structure and, if the requested amount fits or can be partially satisfied after calling another routine, it updates three offset fields and a remaining size field in the structure and then calls another routine with a pointer derived from the structure.
- `reset_state_and_move_offset`: The routine resets a set of fields in a structure and then calls another routine with a pointer into the structure and a size derived from a field, adding the returned value to a field when positive.
- `update_scroll_window`: The routine updates a scrolling window structure pointed to by param_4 using param_2 and param_3 as offsets.
- `read_line_from_buffer`: The routine repeatedly calls another routine with a pointer taken from an offset in the second argument, stores each returned byte into a buffer pointed to by the first argument, and stops when a newline byte is seen or after a fixed number of iterations.
- `copy_cstring_to_buffer`: The routine copies bytes from a source pointer to a destination pointer one at a time until a zero byte is encountered, writing the terminating zero byte as well.
- `uppercase_string_in_place`: The routine walks a null-terminated string and converts each lowercase letter to its uppercase equivalent in place, leaving other characters unchanged.
- `parse_signed_decimal`: The routine parses a signed decimal integer from a null-terminated character string.
- `conditional_indirect_call`: The routine checks two conditions involving the parameters and, if both are non-zero, calls another routine by indexing into a table stored in a global variable and invoking the retrieved function pointer.
- `process_message`: The routine recursively processes a structure pointed to by a pointer, invoking another routine on several of its fields and on a string argument, then conditionally updates fields of the structure based on other fields and invokes another routine with the structure and a constant.
- `build_message_buffer`: The routine obtains a pointer from another routine and then calls another routine with that pointer, a constant, a stack-allocated buffer containing the input parameter, and a size of one.
- `copy_entity_data`: The routine obtains a handle from another routine and, if valid, reads a value from that handle, passes it to another routine together with a small stack buffer, and copies two fields from the handle into the caller-provided structure.
- `initialize_object_from_template`: The routine obtains a template identifier, allocates an object via another routine and links the object’s fields to the template and to a source block.
- `call_indirect_table_entry`: The routine calls another routine through a pointer obtained by indexing a global variable table.
- `call_indirect_table_entry_2`: The routine fetches a function pointer from a global variable table at a fixed offset and calls the pointed-to routine.
- `invoke_callback_from_table`: The routine loads a pointer from a global variable, subtracts a fixed offset, dereferences it and calls the routine found there.
- `call_indirect_from_table`: The routine calls another routine through an indirect pointer obtained by indexing a global variable table with a negative offset.
- `call_indirect_table_entry_3`: The routine calls another routine through a pointer taken from a global variable table offset.
- `execute_dispatch_indirect`: The routine calls a function pointer obtained by indexing a global variable with a negative offset and then returns.
- `execute_indirect_table_call`: The routine performs an indirect call through a table stored in a global variable, invoking another routine via a pointer offset from that table.
- `invoke_deferred_callback`: The routine calls a function pointer taken from a global variable table offset.
- `call_indirect_table_entry_4`: The routine loads a global variable, applies an offset, dereferences it as a function pointer and calls the pointed-to routine.
- `execute_call_via_table`: The routine loads a pointer from a global table at an offset and calls the routine it points to.
- `call_indirect_handler`: The routine invokes a function pointer taken from a global variable table by applying an offset and then calling through the resulting pointer.
- `call_indirect_table_entry_5`: The routine calls a function pointer taken from a global variable table by an offset.
- `invoke_handler_from_table`: The routine calls a handler obtained by indexing a global table with a negative offset.
- `call_indirect_table_entry_6`: The routine calls another routine through a pointer taken from a global variable table.
- `call_indirect_table_entry_7`: The routine calls another routine through a pointer taken from a global variable table.
- `call_indirect_table_entry_8`: The routine calls another routine through a pointer obtained by indexing a global table.
- `call_indirect_via_table`: The routine calls another routine through a pointer obtained by indexing a global table with a constant offset.
- `invoke_handler_from_table_2`: The routine calls another routine obtained by indexing a global variable table with a negative offset.
- `call_indirect_table_entry_9`: The routine calls another routine through a pointer obtained by indexing a global table with a fixed offset.
- `call_indirect_handler_2`: The routine calls another routine through a pointer stored in a global variable.
- `call_indirect_table_entry_10`: The routine calls another routine through an indirect pointer obtained from a global variable.
- `call_indirect_table_entry_11`: The routine calls a function pointer taken from a global variable table.
- `call_indirect_table_entry_12`: The routine calls another routine through an indirect pointer obtained by adding an offset to a global variable.
- `call_indirect_table_entry_13`: The routine calls another routine through a pointer obtained by indexing a global variable table with a fixed negative offset.
- `call_indirect_table_entry_14`: The routine calls another routine through a pointer obtained by indexing a global table with a negative offset.
- `call_indirect_table_entry_15`: The routine calls another routine through an indirect pointer obtained by adding a constant offset to a global variable.
- `invoke_handler_via_pointer_table`: It calls a routine obtained by adding a constant offset to a global variable and indexing into a table of function pointers.
- `recurse_with_stack_slot`: The routine calls another routine with the same two parameters and a pointer to a 12-byte stack slot.
- `extract_byte_from_call`: The routine calls another routine with a pointer to a four-byte stack buffer and a constant argument.
- `compute_checksum_and_call`: The routine builds a 32-bit checksum over a 140-byte stack buffer by iterating until a zero byte is found, then inverts the result and passes it together with the buffer to another routine if the inverted value is non-zero.
- `store_byte_via_pointer`: The routine stores a byte from a register into the memory location addressed by a pointer register and returns immediately.
- `modular_inverse_32bit`: The routine computes a 32-bit modular inverse using the extended Euclidean algorithm when the divisor fits in 16 bits, otherwise it falls back to a 16-bit modulo reduction.
- `multiply_32_bit_integers`: The routine takes two 32-bit values from registers and computes their 32-bit product using 16-bit halfwords.
- `recursive_call`: The routine calls another routine and then returns.
- `negate_on_flag`: The routine calls another routine and captures its return values.
- `invoke_table_entry`: The routine calls a function pointer taken from an offset within a global variable, effectively invoking an entry from a table of handlers.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `call_indirect_through_table`: The routine calls another routine by indexing into a table stored in a global variable and invoking the retrieved function pointer.
- `allocate_message_entry`: The routine obtains a message buffer from another routine and, if successful, fills several fields of the buffer with the supplied parameters and a value retrieved from another routine, then dispatches the buffer to another routine.
- `clear_object_state`: The routine clears a byte field in a structure to a constant value and sets a 32-bit field to all ones.
- `compare_strings_with_wrap`: The routine compares two null-terminated strings byte by byte, advancing both pointers in lockstep.
- `self_referential_pointer_setup`: The routine writes the address of its pointer argument back into the pointed-to memory, adds four to that stored value, clears the next word, and stores the original pointer value into the following word.

### Assign

- `initialize_memory_table`: The routine walks a linked list of entries stored in a global variable to find the largest size field, then builds a small table in memory with a header and a sequence of increasing values.
- `copy_table_to_stack`: The routine copies sixteen 32-bit values from a table located at an offset from a pointer in A2 into a caller-supplied buffer on the stack, then calls another routine.

### EndCLI

- `initialize_command_line_interface`: The routine walks a linked list of entries to find the maximum value stored in a field of each entry, then builds a small command-line interface structure in memory using that maximum.
- `copy_16_words_from_table`: The routine copies sixteen 32-bit words from a fixed table located at an offset from a pointer in a2 into a caller-supplied buffer, then calls another routine.

### Echo

- `initialize_echo_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field of each entry, then builds a small table in memory using that maximum and a set of constants.
- `copy_table_to_stack_2`: The routine copies sixteen 32-bit values from a table located at an offset from a pointer into a caller-provided buffer on the stack, then calls another routine.

### LoadWB

- `parse_workbench_command_line`: The routine obtains a Workbench message via exec library calls, extracts the command line string from the message, and parses it into separate arguments handling quoted strings and simple escape sequences.
- `restore_workbench_message`: The routine restores the Workbench message port state by saving the current message pointer, installing a new message pointer, and then invoking a series of cleanup steps through function pointers stored in a global table.
- `load_workbench_icon`: The routine processes a global variable by calling several other routines, performing a double XOR decode on a zero-terminated byte string via a pointer, then stores a result from another routine into a buffer obtained earlier.
- `compare_strings`: The routine compares two null-terminated strings byte by byte.
- `build_linked_list`: The routine walks a chain of pointers stored in a table, allocating a node for each entry via another routine and linking them into a list.
- `call_indirect_table_entry_16`: The routine calls a function pointer taken from a global table by adding a negative offset to the table base and dereferencing it.
- `call_indirect_table_entry_17`: The routine calls another routine through a pointer obtained by adding a constant offset to a global variable.
- `call_indirect_table_entry_18`: The routine calls another routine through a pointer obtained by indexing a global variable table with a negative offset.
- `call_indirect_table_entry_19`: The routine performs an indirect call through a pointer stored in a global variable table, invoking another routine with no arguments and then returns.
- `call_indirect_via_table_2`: The routine calls another routine through a pointer obtained by indexing a global variable table.
- `call_indirect_table_entry_20`: The routine calls another routine through a pointer taken from a global variable table.
- `call_indirect_handler_3`: The routine calls a handler obtained by indexing a global table with a fixed negative offset and invoking the resulting function pointer.
- `call_handler_from_table`: The routine fetches a handler address from a global table by adding a constant offset to a base pointer and then calls the handler.
- `call_indirect_via_table_3`: The routine calls another routine through a function pointer obtained by indexing a global variable table.
- `call_indirect_table_entry_21`: The routine loads a pointer from a global variable table and calls the routine it points to.
- `invoke_callback_from_table_2`: The routine calls a function pointer taken from a global table by applying a fixed negative offset to a base address.
- `call_indirect_table_entry_22`: The routine calls another routine through a pointer obtained by indexing a global variable table with a fixed offset.
- `compare_workbench_version`: The routine compares a Workbench version string stored in memory with a global variable and returns early if the version starts with '2'. It walks both strings byte by byte, stopping when a mismatch or a null terminator is found.
- `fill_buffer_spaces`: The routine stores the dot character followed by four space characters into consecutive bytes of a buffer referenced via a pointer.
- `compare_byte_stream`: The routine compares a byte stream from an uninitialized pointer with a byte stream from a global variable, advancing both pointers until a mismatch is found or the global stream ends, then returns.
- `empty_routine_6`: The routine contains four instructions and performs no observable operation; it ends with a return.
- `call_indirect_table_entry_23`: The routine calls another routine through a pointer obtained by indexing a global variable table with a fixed offset.
- `dispatch_handler_via_table`: The routine calls another routine obtained by indexing a global variable table with a fixed offset.
- `call_indirect_table_entry_24`: The routine calls another routine through an indirect pointer obtained by adding a constant offset to a global variable.
- `call_indirect_handler_4`: The routine calls a handler obtained by indexing a global table with a fixed offset and then returns.
- `call_indirect_table_entry_25`: The routine calls another routine through a pointer obtained by indexing a global variable table with a fixed negative offset.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_other_routine_with_stack`: The routine calls another routine, passing the two incoming parameters and a pointer to a local stack slot.
- `compute_checksum_and_call_2`: The routine computes a checksum over a 140-byte stack buffer by iterating until a null byte is found, updating a 32-bit accumulator with a decrementing short value.
- `store_byte_from_register_via_pointer`: The routine writes a byte from register D0 to the memory location pointed to by A3.

### Ram-Handler

- `dispatch_via_table`: The routine calls another routine through a pointer stored in a global variable and then calls a routine selected from a table.

### ramdrive.device

- `return_negative_one`: The routine returns a constant value with all bits set.
- `open_ramdrive_device`: The routine opens the ramdrive.device library, walks a list of device nodes and compares a name stored in a global variable with each node’s name.
- `copy_and_setup_ramdrive`: The routine copies a byte array from a pointer into a global buffer, then loads a small table of word values into another global area, and if a computed size is non-zero it allocates a block, fills it with code from another routine, copies a global block into a second area, patches pointers inside that area, and links the new block into a global list.
- `copy_ramdrive_device`: The routine copies a block of data from a source pointer to a destination pointer, then computes a size value from several global variables and, if non-zero, initializes a global variable with a derived offset and another with a derived size.
- `initialize_ramdrive_device`: The routine initializes a device structure by storing pointers to its internal buffers and control fields, links the structure into a global list, and sets up a 128-entry table with a checksum.
- `double_call_through_table`: The routine calls another routine twice via a pointer stored in a table at a fixed offset from A6, then returns zero.
- `decrement_interrupt_lock_and_call`: The routine increments a lock byte in the caller's structure, walks a linked list of objects filtered by a mask in D1 and a threshold in D0 to find the highest matching value, then calls another routine through a function pointer in the structure.
- `walk_linked_list_and_update_lock`: The routine walks a linked list filtered by a mask in D1 and a threshold in D0, updates a lock byte in a global variable and conditionally writes to INTENA, then returns a value from another routine.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decrement_byte_and_test`: The routine reads a byte from an address offset from a register, decrements the stored byte, and if the original value was less than one, writes a constant to a global variable.
- `store_argument_to_offset`: The routine stores the value in D1 to an address computed from A1 plus a fixed offset and returns zero.
- `handle_ramdrive_message`: The routine processes a message by invoking a handler from a global variable table, reads a short field from the message to index a jump table of handlers, calls the selected handler, writes its byte result back into the message, optionally calls another routine based on a flag in the message, then replies to the message via the Exec library.
- `return_constant_value`: The routine returns a constant value.
- `return_constant_value_2`: The routine returns a constant value.
- `decrement_refcount_and_cleanup`: The routine decrements a short counter in a structure and, when the counter reaches zero and a flag is set, it frees the structure by unlinking it from a linked list, clears the flag, and calls another routine.
- `update_list_and_terminate_string`: The routine copies a value from a table into a pointer, updates a linked list of pointers, calls a function from a table and stores its result into a global variable, then terminates a string at the first colon and calls another routine.
- `set_device_name_and_call_permit`: The routine copies a value from a table into a pointer, calls a function from a table and stores its result into a global variable, then finds the end of a string in memory, overwrites the terminator with a colon, and calls another routine from a table.

### mathtrans.library

- `halt_on_unimplemented_instruction`: The routine contains no implemented instructions.
- `store_input_to_global`: The routine copies the value passed in a0 into a global variable and then returns.
- `set_up_library_context`: The routine checks a flag in a structure pointed to by a6.
- `set_flag_in_struct`: The routine sets a flag bit in a byte field of a structure pointed to by a register, then returns zero.
- `increment_and_init_flag`: The routine checks a flag byte in a structure via an uninitialized pointer.
- `return_all_ones`: The routine returns the constant value all bits set, which on a 32-bit signed integer is -1.
- `decrement_counter_and_call`: The routine decrements a short counter stored in the structure pointed to by A6 and, if the counter reaches zero and a flag bit is set in the structure, it calls another routine.
- `mask_and_call_routine`: As part of library code, this routine masks bit 7 of a byte in d0, calls another routine, and sets the extended bit in the status register.
- `decode_signed_magnitude`: The routine decodes a signed-magnitude value passed in D0 and builds a bit mask in D1 by iterating over a 24-bit field.
- `adjust_bitfield_value`: The routine takes two input values and a global variable and repeatedly modifies the first value by setting a bit selected by the second value, then adjusts a running difference using a doubling accumulator and a shifting value, and returns a combined result of the modified first value and the high word of the second value when a termination condition on the difference is met.
- `mask_and_set_flag`: The routine calls another routine, takes its 32-bit result, clears bit 3 and sets bit 1, then returns the modified value.
- `call_jump_table_entries`: The routine calls two different jump-table entries six times each in an alternating pattern, then calls a third entry once, and calls the second entry once more, using an address derived from an uninitialized register.
- `process_library_state_and_mask`: The routine is library code that reads a global variable, calls several library routines via offsets from it, conditionally calls another routine and returns a masked value.
- `process_library_state`: The routine reads a global variable and calls several library routines via offsets from it, then conditionally calls another routine and returns a masked result from that call.
- `conditional_library_dispatch`: Library code that conditionally dispatches to library routines based on flags and then calls another routine and returns its result masked to a fixed bit pattern.
- `mask_return_value_conditional_2`: Library code that conditionally calls library routines based on flags, then calls another routine and returns its result masked to a fixed bit pattern.
- `mask_return_value_conditional`: The routine is library code that conditionally calls library routines based on flags, then calls another routine and returns its result masked.
- `mask_return_value`: The routine calls another routine through a pointer stored in a global variable and then calls another routine.
- `mask_return_value_2`: The routine calls a function pointer twice via an offset from a register, calls another routine, and returns the result of that call with the low three bits masked to zero by ANDing with a constant.
- `mask_return_value_3`: The routine calls a function through a pointer stored in a global variable twice, then calls another routine and returns its result with the third bit cleared.
- `call_indirect_table_entry_26`: The routine calls another routine through an indirect pointer obtained by indexing a global table with a fixed offset.
- `call_indirect_table_entry_27`: The routine calls another routine through an indirect pointer obtained by indexing a global table.
- `call_indirect_table_entry_28`: The routine calls another routine through a pointer stored in a global variable table.
- `normalize_and_scale_value`: The routine normalizes an input value by masking its high bit and clamping it to a minimum, then extracts a shift count from the low byte and uses it to scale the remaining bits.
- `shift_loop_with_early_exit_2`: Library code that performs a loop with shifts and decrements, calls another routine when a counter underflows, and returns early.
- `shift_loop_with_early_exit`: The routine is library code that performs a loop with shifts and decrements, calls another routine when a counter underflows, and returns early.
- `perform_obfuscated_loop`: The routine processes an input byte from an uninitialized register, applies a series of arithmetic and shift operations with constants, then iterates over a global table of 24 entries using a loop that manipulates three accumulators and a counter.
- `compute_modular_exponentiation`: The routine computes a modular exponentiation using a global table of constants and a loop that repeatedly shifts and adds values derived from an input byte in D7.
- `normalize_table_index`: The routine normalizes an index derived from an input value using a series of shifts and arithmetic loops, then walks a global table of signed offsets to adjust the index and produce a final signed result.
- `normalize_and_reduce_value`: The routine is library code that checks a byte derived from D7, performs arithmetic normalization using shifts and a global table, and conditionally calls another routine.
- `normalize_and_reduce`: The routine normalizes an input value and then reduces it using a global table of offsets.
- `normalize_table_index_2`: This library routine normalizes a table index using a global variable table.
- `process_table_entries`: The routine walks a table of signed entries stored in a global variable, using a small fixed-size loop to accumulate a signed sum with a running offset.
- `process_table_with_shifts_and_loop`: Library code that processes a table using shifting arithmetic in a loop with early exit, then calls helper routines based on a stack flag before returning.
- `process_table_with_shifts`: Library code that iterates over a table with shifting arithmetic on several registers and calls helper routines under specific conditions before returning.
- `empty_library_stub`: The routine contains no instructions and therefore performs no operation.
- `recursive_call_wrapper`: Library code that returns immediately when a register is zero, otherwise calls another routine and restores the stack before returning.
- `call_other_routine_twice`: The routine calls another routine from the library twice in succession and then returns.
- `normalize_word_value`: The routine normalizes a signed 16-bit word stored in a register by taking its absolute value, swapping its high and low bytes, and then left-shifting it repeatedly until a sign bit appears or a counter expires; it also sets a byte variable based on the sign and byte-swap steps.
- `normalize_signed_fixed_point`: The routine checks a flag in a register and returns if set.
- `return_without_effect`: Library code calls a routine and then calls it again if a certain condition is met.
- `call_other_routine_conditionally`: The routine invokes another routine and then, based on a boolean flag that remains false, invokes three additional routines in sequence.
- `check_and_branch_on_flag`: The routine reads a byte from a global variable and tests it against a constant.
- `rotate_constant_value`: As part of library code, this routine loads a constant into d7, performs a right rotate through the carry bit on a byte, and branches.
- `empty_stub_routine`: The routine contains no instructions and returns immediately.
- `test_and_branch`: Loads a word from the stack into d6, tests the byte in d6, and if it is not positive, sets d7 to zero and branches.
- `set_status_and_restore_registers`: Sets the negative flag in the condition code register and restores registers d1 through d6 and a0 from the stack before returning.
- `load_constant_and_return`: Loads a constant value into a data register, adjusts the stack pointer by an offset of 30, and returns to the caller.
- `process_transformed_value`: The routine checks a register value and, if non-zero, calls another routine and then manipulates the register value through byte arithmetic and bit masking, storing intermediate results on the stack.
- `self_recursive_loop`: The routine calls itself four times in a row and then returns.
- `subtract_array_from_accumulator`: The routine walks through a table of signed words pointed to by a pointer, repeatedly adding the table entry to an accumulator while the accumulator is negative and then subtracting the next entry.
- `subtract_table_from_accumulator`: The routine walks through a table pointed to by A0 and repeatedly adds the current table entry to an accumulator in D7, then subtracts subsequent entries while a counter in D1 is decremented.
- `normalize_angle_and_branch`: The routine is library code that checks a byte from D7, normalizes an angle value by repeated subtraction and shifting, and calls other routines conditionally.
- `empty_routine_2`: The routine contains no instructions and returns immediately.
- `compute_normalized_angle`: The routine normalizes an angle value derived from an input byte by repeatedly subtracting scaled powers of two until the remainder fits within a range, using a loop that adjusts a shift count and a counter.
- `compute_normalized_angle_2`: The routine repeatedly shifts and subtracts values derived from registers and a global variable, adjusting a counter and a byte until a condition is met, then loops over a short counter multiplying an integer and branching on its sign.
- `call_other_routine_thrice`: The routine calls another routine three times in succession and then returns.
- `process_byte_table_entry`: The routine reads a byte from a table indexed by the low byte of an input value and dispatches to another routine.
- `process_undefined_word`: The routine checks a word in D7 and returns early if it is zero.
- `transform_and_return_value`: As part of library code, this routine performs a series of arithmetic and bitwise operations on d7, including a swap and a rotate, before returning.
- `null`: The routine is not empty.
- `modifies_byte_in_d7`: Compares a byte in d7 against two specific values, and if it matches the second value, adds eighty-five and performs a left bitwise rotation before testing the result to decide whether to branch to another routine or a different location.
- `test_and_clear_register`: As part of library code, this routine masks bits in d7, tests the result, and either branches or clears d7 before branching.
- `shift_and_test_byte`: Compares d7 to 254, and if equal, performs two logical shifts on d7 and branches if the result is not zero.
- `shift_and_rotate_register`: As part of library code, this routine shifts d7 left, loads a constant into d7, performs a right rotate through the carry bit on a byte, sets the extended bit, and branches.
- `empty_library_stub_3`: The routine contains no instructions and returns immediately.
- `invoke_callback_and_return`: The routine receives a pointer in A0 and a value in D0.
- `call_indirect_return_d0`: The routine receives a function pointer in a0 and a value in d0.
- `library_unreachable_routine`: The routine is unreachable in the library and consists of two instructions: it calls another routine and then returns the value that was in register D0 on entry.
- `call_other_routine_return_d0`: The routine calls another routine and returns the value that was in D0 on entry.
- `call_library_routine_and_return`: The routine calls another routine and then returns the value that was in D0 on entry.
- `return_saved_value`: The routine calls another routine and then returns the value left in register d0.
- `forward_unmodified_value`: The routine calls another routine and returns the value that was in D0 on entry, leaving the result of the call unused.
- `return_register_value`: The routine calls another routine and then returns the value that was in D0 on entry.
- `return_input_value`: The routine calls another routine and then returns the value that was in D0 on entry, leaving the call result unused.
- `store_register_via_pointer`: The routine stores a 32-bit value from a register into memory addressed by a pointer passed in a register, then returns.
- `return_register_value_2`: The routine calls another routine and returns the value that was in D0 on entry.
- `return_register_value_3`: The routine calls another routine and then returns the value that was in register D0 on entry.
- `return_input_value_2`: The routine calls another routine and then returns the value that was in D0 on entry.
- `return_unmodified_value`: This library routine calls another routine and returns the original D0 register value unchanged.
- `call_and_return_input`: The routine calls another routine and returns the value that was in D0 on entry.
- `call_other_routine_return_d0_2`: The routine calls another routine and then returns the value that was in D0 on entry.
- `call_other_routine_return_d0_3`: This routine calls another routine and then returns the value that was in D0 on entry.
- `return_input_value_3`: The routine calls another routine and returns the value from register D0 without modification.
- `return_register_value_4`: The routine calls another routine and then returns the value that was in register D0 on entry.
- `invoke_indirect_through_global`: The routine calls another routine through an address stored in a global variable and returns the value that was in D0 on entry.
- `call_indirect_via_table_4`: The routine calls another routine through an indirect pointer taken from a global variable and returns the value that was in D0 on entry.
- `call_indirect_and_return_concat`: The routine calls another routine through a pointer stored in a global variable and returns the concatenation of the two input values.
- `call_global_and_return_concat`: The routine calls another routine via a pointer stored in a global variable and then returns the concatenation of the two input values.
- `call_indirect_via_table_5`: The routine calls another routine through a pointer taken from a global variable and returns the value that was in D0 on entry.
- `call_indirect_from_table_2`: The routine calls another routine through a pointer stored in a global variable and returns the value from D0.
- `call_indirect_and_return_concat_2`: The routine calls another routine through a pointer stored in a global variable and then returns a 64-bit value formed by concatenating the two input registers.
- `invoke_library_routine`: The routine calls another routine via a pointer stored in a global variable and returns the concatenation of the two input values.
- `call_indirect_and_return_concat_3`: The routine calls another routine through an address derived from a global variable and then returns the concatenation of the two input values in D0 and D1.
- `invoke_callback_and_return_concat`: The routine invokes another routine via a pointer stored in a global variable and then returns the concatenation of the two input values.
- `call_indirect_and_return_input`: The routine is library code that calls a function pointer indexed by A0 from a global base and returns the input value in D0.
- `normalize_fixed_point_value`: The routine normalizes a signed 32-bit fixed-point value in D6 by swapping its high and low words, adjusting a shift count based on sign and magnitude, then left-shifting the value while decrementing the count until the count reaches zero or the value overflows, returning early on overflow.

### icon.library

- `return_constant_value_3`: The routine is the program entry point and immediately returns a constant value.
- `increment_counter_and_clear_flag`: The routine increments a 16-bit counter stored at an offset from a pointer in A6 and clears a specific bit in a byte at another offset from the same pointer.
- `decrement_counter_and_call_2`: The routine decrements a short counter at an offset from a pointer in A6 and stores the result back.
- `process_icon_message`: The routine checks a flag in the structure pointed to by a6.
- `set_flag_in_structure`: The routine sets a flag bit in a structure pointed to by a register, then returns zero.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `set_icon_library_pointers`: The routine stores a pointer from a register into a structure field and saves a library base address into another field.
- `copy_two_pointers_from_table`: The routine reads two 32-bit values from an address derived from a register and stores them into two global variables.
- `call_other_routine`: The routine is empty except for an unconditional call to another routine and an immediate return.
- `call_other_routine_2`: The routine consists of a single call to another routine and then returns.
- `call_other_routine_3`: The routine calls another routine and then returns.
- `recursive_call_stub`: The routine unconditionally calls another routine and then returns.
- `call_other_routine_4`: The routine calls another routine once and then returns.
- `invoke_single_helper`: Library code that calls a single helper routine and then returns.
- `call_other_routine_5`: The routine calls another routine and then returns.
- `self_recursive_call`: The routine immediately calls another routine and then returns.
- `call_other_routine_10`: The routine calls another routine and then returns.
- `recursive_call_2`: The routine calls another routine and returns.
- `call_other_routine_6`: The routine consists of five instructions that unconditionally call another routine and then return.
- `manage_icon_chain`: This library routine calls another routine to manage an icon chain.
- `call_other_routine_11`: The routine calls another routine once and then returns.
- `call_other_routine_7`: The routine calls another routine and then returns.
- `call_other_routine_12`: The routine calls another routine and then returns.
- `set_icon_data_field`: The routine calls another routine via a function pointer taken from a table referenced by the first argument and stores the second argument into a field of the returned structure.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `copy_icon_data_to_structure`: The routine obtains a structure via another routine, fills fields of that structure from a local buffer and from stack slots, and, if a pointer field is non-zero, stores the structure address into a field of the pointed-to object.
- `copy_icon_data_fields`: The routine copies a block of long words from a source area in the caller’s structure to a destination structure and then copies several additional long words and a byte from the source to fixed offsets in the destination.
- `return_zero_stub_2`: Sets d0 to zero, restores d2, d3, and d4 from a location at offset -96 from a6, unloads a6, and returns.
- `copy_icon_data_to_stack`: The routine copies ten longwords from an offset within a structure pointed to by the third argument into a local array on the stack, then copies several additional fields from the same structure into other stack slots and finally calls another routine with the first two arguments and a pointer to a stack-resident header.
- `process_icon_descriptor`: The routine processes an icon descriptor pointed to by a parameter, using a temporary stack buffer and several helper routines to resolve and copy fields.
- `check_icon_structure`: The routine checks a structure pointed to by a2 for a specific signature and flag, then updates fields in a structure pointed to by a4 by calling another routine and storing results.
- `initialize_icon_data`: The routine initializes fields in a structure pointed to by a register, storing results from another routine into two offsets and then conditionally updates further fields based on flags in another structure and a function pointer, using a loop to fill a table when a pointer is set.
- `process_icon_data_loop`: The routine repeatedly calls a callback and another routine, storing the callback result through a pointer and decrementing a counter in a register until the counter is no longer greater than four.
- `call_and_return_zero`: The routine calls another routine and then returns zero.
- `empty_routine`: Moves d4 to d0, restores d2, d3, d4, d5, a2, a3, a4, and a5 from a location at offset -300 from a6, unloads a6, and returns.
- `validate_icon_data_2`: The routine validates an icon structure by obtaining a handle, checking fields, and invoking library helpers to verify and copy data fields.
- `handle_icon_draw`: The routine is part of libs/icon.library and is referenced from .info.
- `process_icon_data`: The routine checks a structure pointed to by a register and, if certain fields are present, calls another routine for each field and updates a length field in a buffer before invoking a callback.
- `traverse_icon_list`: The routine walks a null-terminated list of pointers, updating a counter in a structure and invoking a callback for each entry.
- `iterate_icon_table_entries`: The routine walks a null-terminated table of pointers via an index register, calling another routine for each entry and updating a counter at a fixed offset from a base pointer, then invoking a callback twice and restoring a saved value to a global variable.
- `recursive_param_traversal`: Library code that recursively processes a structure by incrementing a parameter and calling another routine until a termination condition is met.
- `iterate_linked_list_nodes`: Library code that walks a linked list by following pointers stored at an offset from the current node.
- `increment_param_and_recurse`: The routine calls another routine with the same first argument and the second argument increased by a constant, then returns.
- `remove_icon_from_list`: The routine attempts to remove an icon entry from a list.
- `call_decrement_and_store`: Pushes the value at offset 168 from a2 onto the stack, pushes d2 onto the stack, calls another routine, and if d0 is zero, moves a2 to d0 before branching.
- `return_zero_via_stack_restore`: Sets the d0 register to zero and restores several registers from the stack before returning.
- `decrement_and_store_pair`: The routine decrements a short counter pointed to by param_2 and, if the counter reaches zero or was already invalid, it calls another routine with the same arguments and, on success, decrements the counter again.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, then uses the counter value as an index into a table pointed to by a global variable.
- `allocate_icon_buffer_and_process`: The routine obtains a temporary memory block from another routine and, if successful, writes a size value into the block and into a caller-supplied word.
- `empty_routine_3`: The routine contains no instructions and returns immediately.
- `call_other_routine_and_return_zero`: The routine calls another routine from the library and then returns zero.
- `incremental_recursive_call`: The routine calls another routine with the same first argument and the second argument increased by a constant, then returns.
- `parse_icon_name_list`: The routine walks a null-terminated list of integers passed by the caller and calls another routine for each entry, using the result to locate a character in a buffer.
- `parse_equals_delimited_value`: The routine walks a table of pointers using a pointer to a pointer and a base address, repeatedly calling another routine with the current table entry until the call returns zero and the byte at the computed address is not '='. It then returns a pointer just past the '=' character found at the computed offset, or null if a zero table entry is encountered.
- `find_next_delimited_entry`: The routine walks a table of integer entries pointed to by a pointer in A2, calling another routine for each entry until the call returns non-zero or the byte at an offset from D3 equal to the entry is not '='. It then returns a pointer into a buffer at D3+entry+1, or null if the table entry is zero.
- `search_icon_list`: The routine walks a linked list of icon structures by repeatedly calling another routine to fetch the next entry and comparing an identifier field.
- `search_icon_table_entry`: The routine walks through a table using an index in a register and values returned by another routine.
- `find_matching_icon_entry`: The routine loops calling another routine with an index and a constant, using the returned values to update the index and a delta.
- `validate_icon_data`: The routine repeatedly calls another routine with a counter derived from an input value and checks the returned values against a stored reference, looping until the results match or a termination condition is met, then returns a success flag.
- `search_icon_table_entry_2`: Library code that walks a table using an index in a register and values returned by another routine.
- `parse_icon_string`: The routine parses a null-terminated string pointed to by the second argument, extracts a numeric value and a following pointer, and writes the results into the structure pointed to by the first argument using another routine from the library.
- `parse_icon_data`: The routine parses a byte stream starting at an offset from a pointer in A4.
- `parse_numeric_string`: The routine reads a null-terminated ASCII decimal string pointed to by a register, converts it to an integer by accumulating digits, then attempts to parse a further value from the remaining bytes using another routine with a global variable and a size of four.
- `parse_numeric_string_2`: The routine reads a byte stream through a pointer, converts each byte to a numeric digit via another routine and accumulates a decimal value in a register.
- `convert_ascii_digit_to_index`: The routine checks if its input is an ASCII digit character and, if so, converts it to a zero-based index by subtracting the ASCII code for '0'. Otherwise it returns a negative value indicating no conversion.
- `return_constant_all_ones`: The routine returns the constant all-ones value.
- `call_indirect_table_entry_29`: The routine performs an indirect call through a function pointer taken from a global table.
- `call_indirect_through_table_2`: The routine calls another routine indirectly via a pointer obtained from a global variable table.
- `call_indirect_through_table_3`: The routine calls another routine indirectly via a pointer stored in a global table.
- `call_through_table`: The routine calls another routine via a pointer stored in a global table.
- `call_indirect_table_entry_30`: The routine calls another routine through an indirect pointer obtained from a global variable.
- `call_indirect_through_table_4`: The routine calls another routine indirectly via a pointer obtained by adding an offset to a global variable that holds a table base.
- `call_indirect_through_table_5`: The routine calls another routine indirectly via a pointer taken from a global variable table.
- `call_indirect_routine`: The routine calls another routine through an indirect pointer obtained from a global variable.
- `invoke_callback_from_table_3`: This routine calls another routine via a pointer stored in a table.
- `propagate_icon_change`: The routine forwards a set of four parameters to another routine, checks the returned value, and if non-zero calls the same routine again with a modified argument list.
- `set_icon_dimensions`: The routine asks another routine for a block of memory and, if successful, writes a size value into the block and into a caller-supplied word, then calls another routine to update the block.
- `return_zero_stub`: Sets d0 to zero, restores d2 and a2 from the stack, and returns.
- `verify_return_value`: The routine calls another routine with three of its arguments and returns the result.
- `empty_library_stub_4`: The routine contains no instructions and returns immediately.
- `validate_return_value`: The routine calls another routine with the second, third and fourth arguments and checks the result against the fourth argument.
- `empty_routine_4`: The routine contains no instructions and returns immediately.
- `perform_modular_arithmetic`: The routine implements a modular reduction of a 32-bit value in D1 by a 16-bit divisor in D2, returning a 32-bit result in D0 with the remainder in the low word and the quotient in the high word.
- `rotate_32bit_value`: The routine rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_fixed_point`: The routine treats the two input values as 16.16 fixed-point numbers and returns their product as a 32.32 fixed-point value in the D0 register.
- `invoke_callback_from_table_4`: The routine calls another routine through a pointer taken from a global variable offset.
- `store_byte_to_pointer`: Library code that stores a byte from a register through a pointer.
- `compute_string_checksum`: The routine processes a null-terminated byte string by iterating through each character, updating a 32-bit accumulator with a decrementing 16-bit pattern, and returns the bitwise complement of the final accumulator value.
- `find_char_in_string`: The routine scans a null-terminated byte string starting at the first argument for the byte value in the second argument.
- `empty_stub_routine_5`: The routine contains no instructions and immediately returns zero.
- `copy_cstring_to_buffer_2`: The routine copies bytes from a source pointer to a destination pointer one at a time until a zero byte is encountered, advancing both pointers each iteration.
- `copy_string_to_buffer`: The routine copies a null-terminated string from a source pointer to a destination buffer.
- `compare_memory_chunks`: The routine compares up to a given count of bytes from two memory pointers and returns zero if the compared bytes are equal or the count is zero, otherwise it returns one.
- `return_constant_one`: Sets d0 to one, and if the result of a comparison is not greater than zero, sets d0 to 255 before returning.
- `scan_string_until_null`: The routine walks a byte pointer forward one byte at a time while a counter is positive, stopping when the byte is zero or the counter reaches zero.
- `call_copy_string`: Decrements a0 by one and calls another routine.
- `copy_string_with_padding`: The routine copies bytes from a source pointer to a destination pointer until a zero byte is encountered, using a counter in D0 to limit the copy.
- `return_immediately`: The routine contains no instructions that operate on registers, memory, or library calls and returns immediately.
- `initialize_icon_table`: The routine writes to a pointer passed in a register, storing the pointer value, then the pointer value plus four, a zero word, and the pointer value again.
- `process_icon_request`: The routine validates a parameter and delegates processing to another routine if the condition is met.
- `check_icon_chain_2`: The routine walks a linked structure by repeatedly calling another routine with fields loaded from the current node, updating the node pointer from a field, and returns a status based on the results.
- `check_icon_chain`: The routine repeatedly calls another routine until it returns zero or a failure condition is met.
- `return_zero`: Sets d0 to zero, restores d2, d3, and a2 from the stack, and returns.
- `conditional_icon_attribute_update`: The routine conditionally invokes another routine to obtain an identifier and then uses that identifier to fetch and store an attribute value through a pointer, returning the identifier on success or a failure indicator otherwise.
- `return_constant_negative_one`: The routine consists of a single instruction that returns a constant value.
- `iterate_linked_list_2`: The routine walks a linked list by repeatedly loading the next pointer from the current node and calling another routine to process each node, updating fields in the nodes and returning an error code on failure.
- `looped_icon_data_copy`: The routine repeatedly calls another routine to obtain pointers and values, writes a value into a structure field and checks for zero.
- `iterate_linked_list`: The routine walks a linked list by repeatedly loading the next pointer from the current node and calling another routine to process each node.
- `return_minus_one`: The routine returns a constant minus one value.

### diskfont.library

- `check_and_set_library_state`: The routine calls another routine via a table entry and, if the call returns non-zero, stores a value from a register into a field of the returned structure and then calls another routine via a table entry.
- `set_font_data_and_flush`: The routine calls another routine via a table indexed from a6 and, if the call returns non-zero, stores a value from a0 into a field of the returned structure and then calls a second routine via the same table.
- `initialize_font_library`: The routine initializes a font library structure by calling library initialization routines via a jump table and, on success, sets up internal pointers and fields in a structure passed in a register, then returns the structure pointer.
- `run_cleanup_and_return`: The routine calls another routine via a global table and then calls a second routine via the same table, passing an uninitialized register, and returns zero.
- `free_shared_library_segment`: Iterates a linked list of segment descriptors stored in a library context structure, unlinks and frees each descriptor that meets a flag condition, and invokes an unlink helper for each.
- `check_and_release_font`: The routine checks a global variable and a flag in a structure pointed to by a register.
- `set_flag_byte`: The routine sets a flag in a byte pointed to by a register by OR-ing in a bit, then returns zero.
- `empty_routine_5`: The routine contains no instructions and returns immediately.
- `clear_flag_and_increment_counter`: The routine clears a bit in a byte at an offset from an address in A6 and increments a 16-bit value at another offset from the same address.
- `decrement_reference_and_dispatch`: The routine decrements a short reference counter at an offset from a structure pointer and, if the counter reaches zero and a flag byte has its high bit set, it calls a handler routine referenced via a pointer in the same structure.
- `empty_library_stub_5`: The routine contains no instructions and performs no operation.
- `copy_font_entries_to_buffer`: The routine copies font descriptor entries from a global table and from a disk font structure into a caller-supplied buffer, writing a short count and a series of five-word records for each entry that fits.
- `copy_font_characters_to_buffer`: The routine walks a linked list of font character descriptors and for each entry copies the null-terminated character name from the descriptor into a destination buffer, updates a running offset and writes a small record into an output table with fields taken from the descriptor.
- `process_char_table_for_diskfont`: Routine iterates over a null-terminated string in memory, copying blocks of bytes from a source table to a destination buffer while maintaining a running offset.
- `copy_font_to_memory`: A routine from diskfont.library that copies bytes from a source buffer into a destination buffer while building an entry table; it loops over linked structures and writes fields into a table, updating counters and flags.
- `process_font_table`: The routine walks a font table and copies each entry into a destination buffer.
- `decode_font_string`: The routine iterates over a null-terminated byte stream, using a length counter to copy bytes from a source area into a destination buffer, invoking another routine to parse control values and conditionally write a packed record to a table via a pointer.
- `copy_font_to_buffer`: This routine copies a byte stream from a source pointer through a pointer to a destination buffer, advancing the destination address each iteration.
- `copy_font_entries_to_buffer_2`: The routine walks a font table and copies each entry into a destination buffer.
- `return_zero_and_restore_registers`: Sets register d0 to zero, restores registers d2 through d6 and a2 through a4 from a stack location relative to a6, unlocks a6, and returns.
- `compute_font_character_width`: The routine computes a signed width value from two structures, using a short field from the second structure and a short field from the first structure to form a scaled base offset, then adjusts the result by subtracting entries from a global table indexed by bits derived from two bytes of each structure.
- `compute_checksum_delta`: The routine computes a signed integer delta from two input pointers by iterating over seven bit positions.
- `compute_weighted_sum`: The routine walks a small fixed-size table indexed by a counter, subtracting each entry from an accumulator.
- `decode_font_bits`: The routine reads a byte from the input buffer and uses two bit masks to test individual bits.
- `return_constant_value_4`: The routine returns a constant value without using any arguments, registers, or library calls.
- `select_best_font_entry`: The routine prepares a font handle from a parameter block, queries a font structure for size and flags, and then iterates over a table of font entries to find the entry with the largest metric value.
- `select_maximum_font_glyph`: The routine opens a disk font file, reads its header and glyph table, then repeatedly finds the glyph with the largest metric value, processes that glyph entry by updating fields and counters in a global table, and removes it from further consideration until no glyphs remain.
- `process_font_directory_entries`: The routine opens and validates a disk font resource, reads its header and directory, then iterates over the font entries to find the entry with the largest value.
- `select_best_font_entry_2`: The routine scans a table of font entries, repeatedly selecting the entry with the highest score returned by another routine and updating a set of fields in the chosen entry, including a pointer and a flag byte, then looping until no entry remains.
- `select_best_font_entry_3`: The routine scans a table of font entries, keeps the entry with the largest metric value, and for each candidate it calls library routines to test and update font structures.
- `select_best_font_entry_4`: The routine scans a table of font entries and repeatedly picks the entry with the highest score returned by another routine, then clears a flag in that entry and processes it with a series of library calls.
- `select_best_font_entry_5`: The routine scans a table of font entries, repeatedly selects the entry with the highest score returned by another routine, and updates a set of global variables and linked-list pointers for the chosen entry.
- `fetch_font_data`: The routine asks a library service for a font descriptor and a font name and, if a matching font file can be opened, it reads the file header and walks the character table.
- `load_disk_font_table`: This library routine builds a table for a disk font.
- `dispatch_font_request`: This routine reads a pointer and an argument from the stack frame and passes them to another internal routine.
- `call_indirect_through_table_6`: The routine performs a single indirect call through a pointer derived from a global variable and then returns.
- `call_indirect_via_table_6`: The routine calls another routine through a pointer obtained by indexing a global table.
- `call_indirect_table_entry_31`: The routine performs an indirect call through a function pointer taken from a global table.
- `invoke_callback_from_table_5`: The routine fetches a function pointer from a global table via an offset and calls the referenced routine.
- `call_indirect_through_table_7`: The routine calls another routine indirectly through a global variable table.
- `call_indirect_through_table_8`: The routine fetches a function pointer from a global table via an offset and calls the pointed-to routine indirectly.
- `call_indirect_via_table_7`: The routine calls another routine through an indirect pointer obtained by adding a constant offset to a global variable, then returns.
- `indirect_call_through_table`: The routine performs an indirect call through a pointer stored in a global variable.
- `call_indirect_through_table_9`: The routine loads an address from a global variable and calls the routine found there.
- `call_indirect_table_entry_32`: The routine calls another routine through an indirect pointer obtained by indexing a global table with a fixed negative offset.
- `call_indirect_table_entry_33`: The routine is a tiny wrapper that fetches a function pointer from a global table and immediately calls the referenced routine.
- `invoke_callback_from_table_6`: The routine loads a global variable, adds a constant offset to obtain a pointer to a table of code pointers, and calls the routine found at that table entry.
- `call_indirect_table_entry_34`: The routine is a one-instruction wrapper that fetches a function pointer from a global table via an offset and immediately calls the pointed-to routine, then returns.
- `call_indirect_through_table_10`: The routine loads a function pointer from a global table and calls it indirectly.
- `invoke_callback_from_table_7`: The routine calls another routine through a pointer obtained by indexing a global table.
- `invoke_callback_from_table_8`: The routine calls another routine obtained by indexing a global table with a fixed offset and invoking the resulting function pointer.
- `call_indirect_table_entry_35`: The routine fetches a function pointer from a global table and calls the pointed-to routine indirectly, then returns.
- `copy_string_until_null`: The routine copies bytes from a source pointer to a destination pointer one at a time, advancing both pointers, and stops when it copies a zero byte.
- `scan_string_and_recurse`: The routine advances a pointer through a null-terminated byte string until the terminator is found, then calls another routine with the pointer to the terminator and the unchanged second argument.
- `skip_to_string_terminator_and_call`: The routine increments a pointer until it reaches the terminating zero byte of a zero-terminated string and then calls another routine with the pointer to that terminator.
- `count_string_length`: The routine receives a pointer to a zero-terminated byte string and iterates through it, incrementing a counter for each byte until the terminating zero byte is found.
- `count_null_terminated_bytes`: The routine increments a counter in D0 while advancing a pointer in A0 and reading each byte until a zero byte is encountered, then returns the counter.
- `compare_two_strings`: It compares two null-terminated strings byte by byte, advancing both pointers until a difference or a terminator is found.
- `compare_two_strings_2`: The routine receives two pointers in registers A0 and A1 and walks them in lockstep, advancing both pointers while the bytes they point to are equal.
- `compare_two_strings_3`: The routine compares two null-terminated strings byte by byte using two pointers.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `find_char_in_string_2`: The routine scans a null-terminated string for a given character and returns a pointer to the first matching byte or a null pointer if the terminator is reached first.
- `compare_byte_until_match`: The routine receives a pointer in a register and two byte values in registers.
- `find_next_byte`: A routine in libs/diskfont.library that scans a byte string at an address in register A0 forward by one byte each iteration, compares each byte to a byte in register D0, and returns the address of the first matching byte or NULL if a zero terminator is encountered.
- `find_char_backward_in_buffer`: The routine asks another routine for a length, then sets a pointer to the last byte of a buffer and walks backwards byte by byte until it finds a match for a supplied character or the pointer passes the buffer start, returning the matching address or a null pointer.
- `reverse_string_search`: The routine walks backwards through a memory region using a pointer in A0, comparing each byte with a value in D2.
- `reverse_the_string_in_place`: The routine receives a pointer in a0 and walks it backwards until it finds a byte equal to a value in d2, stopping if the pointer passes a limit in d3.
- `adjust_string_length`: The routine calls another routine twice with the same first argument and different second arguments.
- `increment_returned_value`: The routine returns the value passed in D0 incremented by one.
- `invoke_table_entry_2`: The routine calls another routine through a pointer stored in a global variable.
- `invoke_table_entry_3`: The routine calls another routine through a pointer taken from a global variable.
- `invoke_callback_from_table_9`: The routine calls another routine taken from a global table.
- `self_looping_routine`: The routine contains no code that accesses registers, memory or library services; it consists of a single call to another routine and then returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.
- `recursive_stub_call`: This routine immediately calls another routine and returns.
- `call_another_routine_2`: The routine is a thin wrapper that immediately invokes another routine and then returns.

### Defender (19xx)(G. McArdell)(PD).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
