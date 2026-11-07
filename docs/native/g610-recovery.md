# Game 610 (SuperOffRoad v1.3a 0387): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00610). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2128268 top-level bytes in 16 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 18 raw-word lines |
| Functions named and described | pass, 389 of 389 named (0 author symbols), 389 of 389 described (2 by rule, 386 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3617 of 3670 (98.56%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7227 of 7239 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 369215 |
| code | 23712 |
| data | 1389215 |
| padding | 14 |
| unused | 346112 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 250 | 10 |
| Disk.1:bootblock | 1 | 19 | 2 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 54 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1377 | 92 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3000 | 146 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 884 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1654 | 128 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- SuperOffRoad_v1.3a_0387/SuperOffRoad_v1.3a_0387_hdf/SuperOffRoad/Disk.1: custom-format (non-DOS) disk, 1077 non-empty blocks: matched against track-loaded segments
- SuperOffRoad_v1.3a_0387/SuperOffRoad_v1.3a_0387.hdf: hard-disk image (AmigaDOS OFS, 2240 blocks): its filesystem parses and all 9 files in it are byte-identical to the extracted tree SuperOffRoad_v1.3a_0387_hdf, which is recovered file by file; no block outside the filesystem holds data
- 3 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 1011 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1143 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 320 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 945 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- SuperOffRoad_v1.3a_0387/SuperOffRoad_v1.3a_0387_hdf/SuperOffRoad/Disk.1: 3 logged loads cover 383488 bytes of the image
- SuperOffRoad_v1.3a_0387/SuperOffRoad_v1.3a_0387_hdf/SuperOffRoad/Disk.1: blocks 769-769 (bytes 393728-394240) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.45)
- SuperOffRoad_v1.3a_0387/SuperOffRoad_v1.3a_0387_hdf/SuperOffRoad/Disk.1: blocks 771-771 (bytes 394752-395264) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.51)
- SuperOffRoad_v1.3a_0387/SuperOffRoad_v1.3a_0387_hdf/SuperOffRoad/Disk.1: blocks 773-773 (bytes 395776-396288) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.08)
- SuperOffRoad_v1.3a_0387/SuperOffRoad_v1.3a_0387_hdf/SuperOffRoad/Disk.1: blocks 775-805 (bytes 396800-412672) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.33)
- SuperOffRoad_v1.3a_0387/SuperOffRoad_v1.3a_0387_hdf/SuperOffRoad/Disk.1: blocks 807-843 (bytes 413184-432128) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.59)
- SuperOffRoad_v1.3a_0387/SuperOffRoad_v1.3a_0387_hdf/SuperOffRoad/Disk.1: blocks 845-879 (bytes 432640-450560) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.67)
- SuperOffRoad_v1.3a_0387/SuperOffRoad_v1.3a_0387_hdf/SuperOffRoad/Disk.1: blocks 881-913 (bytes 451072-467968) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.80)
- SuperOffRoad_v1.3a_0387/SuperOffRoad_v1.3a_0387_hdf/SuperOffRoad/Disk.1: blocks 915-955 (bytes 468480-489472) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.17)
- SuperOffRoad_v1.3a_0387/SuperOffRoad_v1.3a_0387_hdf/SuperOffRoad/Disk.1: blocks 957-1077 (bytes 489984-551936) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.47)
- SuperOffRoad_v1.3a_0387/SuperOffRoad_v1.3a_0387_hdf/SuperOffRoad/Disk.1: blocks 1079-1112 (bytes 552448-569856) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.49)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `return_negative_one`: The routine returns a constant negative one value.
- `wait_for_flag_then_call`: The routine saves an incoming value into a global location, invokes another routine, then sets a byte flag to one.
- `dispatch_hardware_initialization`: Checks a global variable against specific magic values to determine execution flow.
- `initialize_copper_and_dma`: The routine configures hardware registers for display and DMA control, clears a memory region, and copies data to a table.
- `empty_routine`: The routine contains 64 instructions but performs no observable work; it does not read or write memory, does not manipulate registers, and does not call other routines.
- `set_interrupt_and_dma_state`: The routine writes a constant to a global variable, stores a pointer to another routine in a global variable, repeats this pattern with different constants, and returns a value that sets the interrupt enable bit and the DMA enable bit in the hardware control registers.
- `write_global_table_entries`: The routine writes a series of constant values and a pointer to another routine into a global variable repeatedly, and stores a constant into a RAM location.
- `read_keyboard_serial_byte`: The routine accesses the CIAA serial data register, which carries keyboard input on the Amiga, and then ends by branching into the routine self.
- `call_indirect_table_entry`: The routine calls a function taken from a table stored in a global variable and returns the combined value of the two input registers.
- `call_indirect_table_entry_2`: The routine calls another routine through an indirect pointer taken from a global variable plus a fixed offset, then returns the value that was in D1 on entry.

### Disk.1:bootblock

- `clear_memory_and_copy`: The routine clears a block of memory, writes a function pointer into a global variable, then traps, and later copies a block of memory from a global area to a start area before calling another routine via an indirect pointer.
- `copy_table_then_call`: The routine copies a block of 256 words from a global variable to a destination pointed to by A5, decrementing a counter from 255 to -1, then performs an indirect call through a pointer derived from A5.

### logged disk load from Disk.1

- `initialize_display_and_dma`: Clears a global memory block and configures display lists by copying code or adjusting pointers based on a control flag.
- `copy_block_with_decrementing_counter`: The routine copies a block of 32-bit words from a source pointer to a destination pointer, decrementing a counter each iteration until the counter becomes negative.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_function_table_entries`: The routine calls five routines via a table pointed to by a register, then checks a word flag and two bytes from a structure pointed to by another register.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_high_score_entry`: The routine reads a byte from a table indexed by a masked input byte and compares it with a stored byte in a global variable.
- `store_value_and_call`: The routine writes the value from a register to a field of a structure pointed to by a register, calls another routine, then writes a constant to another field of the same structure and returns.
- `parse_script_byte`: A routine reads a byte from a script table using an index from a structure and advances the pointer, then interprets the byte as an opcode with several ranges.
- `parse_script_byte_stream`: The routine reads a byte stream pointed to by a pointer in its structure and processes control bytes.
- `initialize_object_state`: The routine writes a word and a byte from registers into a structure, computes a pointer from a short value and a global offset, stores the result and a constant, and installs the same routine pointer into three function slots.
- `copy_word_from_table_to_table`: The routine copies a 16-bit value from an offset in one table to an offset in another table and then writes a constant 16-bit value into a 32-bit field in the second table.
- `init_track_entry`: A routine that initializes a track entry structure from three input structures.
- `decrement_counter_and_copy_flags`: The routine checks a flag in a structure and, if clear, writes a byte and a word to locations indexed by offsets in the same structure.
- `decrement_counter_and_copy_bytes`: The routine calls another routine, then decrements a 16-bit counter at an offset from a pointer in a register.
- `decrement_counter_and_copy_fields`: The routine calls another routine, decrements a 16-bit counter at an offset from a pointer, and returns if the counter is non-zero.
- `decrement_and_copy_byte`: The routine decrements a short counter stored at an offset from a pointer and, if it reaches zero, copies a byte from another pointer and writes a constant to a word at an offset, then returns.
- `decrement_counter_and_reset_state`: The routine decrements a byte field in a structure pointed to by a register and, if the result is zero, clears several fields in that structure and copies a word from another structure into a table indexed by a short offset.
- `clamp_and_store_scaled_value`: The routine reads a signed 16-bit value from an offset in a structure pointed to by a parameter, adds a signed 8-bit value from a nearby offset, clamps the result to zero and to a limit stored in a global variable, writes the clamped value back to the structure, and stores a scaled version of that value into a table pointed to by another parameter using an index taken from the structure.
- `copy_tile_data_to_buffer`: The routine copies a small header from a source structure into a destination structure and then copies a variable number of 16-bit words from the source to the destination, using a loop count taken from a byte in the source.
- `copy_bytes_and_decrement_counter`: The routine copies three bytes from two source locations into three fixed offsets of a structure pointed to by a pointer, writes a constant word to another offset of the same structure, calls another routine, decrements a counter byte in the structure and if the counter reaches zero writes a different constant word to the same word offset.
- `decrement_counter_and_set_flag`: The routine calls another routine, then decrements a byte stored at an offset from a pointer in a register and writes the result back.
- `process_animation_frame`: The routine updates a small animation state stored in a structure pointed to by a register.
- `update_object_position`: Adjusts a short value in a data structure based on conditional checks of other bytes.
- `swap_and_set_flag`: The routine swaps a word stored at a global variable with a word passed in a register, writes the same word to two other locations in the same structure, and stores a constant word at a fixed offset.
- `update_short_value`: The routine reads a short from a global variable, subtracts the value at the start of the same structure, shifts the result by a mask of the following byte, and if the result is non-zero adds it back to the start value.
- `copy_word_from_table_and_set_value`: The routine writes a word from a table entry into a global variable and then stores a constant word into another global variable.
- `set_tile_and_offset`: The routine writes a constant to a word location derived from a global variable and a short offset, writes a constant to a dword field of a global structure, calls another routine to obtain a word value, masks that value, adds a constant from a global table, and stores the result to a dword location derived from another global variable and a short offset.
- `store_offset_value`: The routine reads a value from another routine, masks it, and adds an offset from a global variable to form an index.
- `process_input_devices`: Iterates through a list of input device structures to call their update routines.
- `execute_trap_handler`: Issues a trap instruction, calls a routine pointer derived from a data structure, and then issues another trap instruction before returning.
- `call_indirect_via_table`: The routine triggers a trap, then calls a function obtained by indexing a table with an offset derived from a global variable, and triggers another trap.
- `set_input_device_bits`: The routine reads the joystick/gamepad state, computes a mask from the input, and updates a byte of input flags by setting or clearing a single bit based on a table lookup.
- `clear_flag_and_wait_input`: The routine clears a single bit in a byte located at an offset from a base pointer, then repeatedly calls another routine until it returns a non-zero character, and finally clears a bit in a global variable before returning the value originally passed on the stack.
- `process_linked_list_and_wait_key`: The routine walks a singly linked list via a pointer stored in a global variable, calling a function pointer from each node until the link is null, then blocks until a non-zero byte is returned by another routine, and clears a bit in a global variable before returning.
- `poll_input_until_nonzero`: The routine walks a linked list of structures by following a pointer stored six bytes from each entry, invoking a callback stored at offset sixteen of each entry until a zero terminator is reached.
- `decode_character_from_table`: The routine decodes a character from a table using a packed index derived from the input word and a global variable.
- `empty_routine_2`: The routine contains ten instructions and performs no observable work; it falls through to a return without accessing memory, registers or calling another routine.
- `store_byte_and_return_byte`: The routine stores the low byte of the input register into a byte field of a structure pointed to by a pointer in a register, then returns the byte stored at that field.
- `read_joystick_and_mouse_state`: The routine writes a constant to a custom hardware register and copies a value from a hardware register into a table entry.
- `update_two_short_fields_and_dispatch`: The routine stores a signed difference of two input words into a global variable and stores a scaled sum of the same inputs into another global variable, then copies an input word into a third global variable.
- `store_input_state`: Stores input data into a global buffer based on an index calculation.
- `set_flag_and_call_indirect`: The routine stores the value 2 into a word at an offset from a global variable, triggers a trap, then calls a function through a pointer obtained from a table referenced by another global variable, triggers a second trap, and returns.
- `set_flag_and_call_table`: The routine writes a constant to a global variable and then triggers a trap.
- `initialize_interrupt_timers_and_dispatch`: A global variable is set to zero multiple times and then filled with values that form a table; a loop writes entries from that table to memory locations referenced by the table.
- `set_global_variable_twice`: The routine writes a constant to a global variable, then overwrites it with a different constant, and returns.
- `empty_routine_3`: The routine contains no instructions and returns immediately.
- `read_cia_a_control_register`: The routine reads the CIA A control register B, stores the value in a memory location, and returns.
- `execute_secondary_trap`: Issues a trap instruction, calls a routine pointer derived from a different data structure than R5, and then issues another trap instruction before returning.
- `call_indirect_through_table`: The routine triggers a trap, then calls another routine indirectly via a pointer obtained from a global variable table, and triggers a second trap before returning.
- `increment_global_counter`: The routine increments a global variable by one and returns.
- `decrement_global_variable`: The routine decrements a global variable by one.
- `wait_for_flag_clear`: The routine spins while a global variable has a particular bit set, then conditionally writes a constant to the same global variable.
- `wait_for_ciaa_crb_and_decrement_counter`: The routine polls a hardware register until a condition on a global variable clears, then decrements that global variable and resets it to a constant if it reaches zero.
- `indirect_jump_through_table`: The routine loads a function pointer from a global table and performs an indirect call to another routine via that pointer.
- `check_timer_condition`: Issues a trap instruction, calls a routine pointer, sets a bit in a byte, and compares two values.
- `call_indirect_via_pointer`: The routine triggers a trap, then calls a routine through a pointer in a register, then triggers two more traps.
- `save_context_and_resume`: The routine saves the current CPU registers and stack into a structure pointed to by a register, clears a flag byte, decrements a short counter and conditionally traps, then manipulates a linked list via a table pointer to pick the next entry, restores two words from that entry into a global variable, and returns a value from the saved context.
- `save_exception_context_and_dispatch`: The routine saves the current CPU registers and stack into a structure pointed to by a register, then walks a linked list of exception handlers stored in a global table to find a matching handler, invokes the selected handler, updates the handler list links and bookkeeping bytes, copies a few bytes from the handler entry into global variables, and finally returns a pointer from the handler entry.
- `handle_unreached_trap`: This routine is unreached.
- `call_indirect_via_table_then_trap`: The routine loads a pointer from a global variable, indexes a table through that pointer to obtain a function address, calls the indirect function, and then executes a trap instruction.
- `call_indirect_via_table_2`: The routine loads a pointer from a global variable, reads a short offset from a fixed offset in that structure, adds the offset to the base pointer and calls the resulting address as a function.
- `initialize_custom_chipset`: The routine writes constants to a global variable and to memory locations referenced through an uninitialized pointer, and clears two custom chipset registers.
- `set_global_variable_twice_2`: The routine stores the same constant value into a global variable twice in succession and then returns.
- `empty_routine_4`: The routine contains no instructions and returns immediately.
- `call_input_update`: Calls a routine pointer from a data structure, stores the result, sets bits in a global variable, and writes a constant value to a memory location before enabling interrupts.
- `increment_counter_and_call_indirect`: The routine increments a global counter and then calls another routine through an indirect pointer derived from a global table.
- `invoke_indirect_table_entry`: The routine triggers a trap, then calls a function obtained by indexing a table through a global variable, and triggers another trap.
- `invoke_indirect_table_entry_2`: The routine triggers a trap, then calls a function obtained by indexing a table of function pointers stored in a global variable, and triggers another trap.
- `call_random_table_init`: Invokes a routine to initialize a pseudo-random number table.
- `fill_pseudo_random_table`: The routine writes a sequence of pseudo-random 16-bit values into a table in memory and stores the final seed value into a global variable.
- `decrement_three_bytes_and_update_short`: The routine decrements three bytes in a global structure and then reads two shorts from a table indexed by two offsets in that structure.
- `increment_counter_and_fetch_value`: The routine increments a byte stored in a global variable and then returns a 16-bit value read from a table indexed by a short stored in a global variable.
- `copy_bytes_and_update_pointers`: Copies a block of bytes into a buffer pointed to by a register argument, updates a structure pointer for that buffer and sets up an offset table for it, then calls another routine and walks a table of structures calling that routine when a flag is clear.
- `copy_and_update_buffer`: The routine copies 71 bytes from a source buffer pointed to by a field in a structure to a temporary buffer, adds an offset to each copied value, then calls another routine and updates a pointer field in the structure to the temporary buffer and clears a flag.
- `process_linked_list_data`: Iterates through a linked list of structures, copying data from a source buffer into destination fields based on signed offsets, until a null terminator is reached.
- `allocate_and_link_nodes`: Allocates two memory blocks, initializes their fields with global values and flags, links them into a global structure, and then traverses a separate linked list to call a routine if a specific condition is met.
- `store_result_via_pointer`: The routine checks a flag stored through a pointer and calls another routine if the flag is zero.
- `decrement_counter_and_call_handler`: The routine selects one of six input pointers based on a two-bit selector from the stack, then if the selected pointer is non-zero it reads a short offset from a fixed displacement within the pointed-to structure, calls the routine found at that offset, and decrements a short counter at a fixed displacement in the same structure.
- `decrement_counter_and_call_handler_2`: The routine checks a flag in a structure pointed to by a0.
- `decrement_counter_and_call_indirect`: The routine checks a flag in a global variable and, if clear, calls an indirect routine via a table entry and decrements a short counter in the same structure.
- `copy_interrupt_vector_table`: The routine writes a constant to a global variable, then copies six 32-bit words from another routine into a table at a fixed address, clears the global variable three times and writes two more constants to it.
- `disable_interrupts_and_clear_flag`: The routine reads a byte from a table entry, combines it with a global variable, writes the result back to the same table entry, then writes a value to a global variable and returns.
- `xor_mask_byte_in_table`: The routine reads a byte from a table entry pointed to by an address register, ORs it with a global variable, writes the result back after XOR masking with a byte from D0, then stores a constant to the global variable.
- `empty_routine_5`: The routine contains no instructions and returns immediately.
- `check_interrupt_flags_and_call_handlers`: The routine reads a global variable, masks it with the INTENAR and INTREQ registers, and uses the result to index a table of function pointers.
- `handle_cia_interrupts`: Checks for pending interrupts on the CIA hardware, masks them, and dispatches calls to function pointers in a table based on the specific interrupt flags detected.
- `dispatch_interrupt_handlers`: The routine reads a global variable, masks it with the interrupt enable and request registers, and uses the result as an index into a table of function pointers.
- `call_table_functions`: The routine fetches a byte from a table of function pointers via an address derived from a register, calls the first entry, doubles the returned byte and, if the result is negative, calls the next entry.
- `invoke_callback_pair`: The routine loads a function pointer from a table via an index register, calls it, multiplies the returned byte by two and tests the sign.
- `call_indirect_through_table_2`: The routine calls another routine via an indirect pointer derived from a global variable and returns a value taken from the stack.
- `increment_global_counter_2`: The routine reads a global variable, adds one to it, and writes the result back to the same global variable.
- `decrement_global_counter`: The routine decrements a global variable by one and, if the result is zero, sets its high bit.
- `empty_routine_6`: The routine contains no instructions and returns immediately without performing any operation.
- `traverse_signed_table`: The routine walks a table of signed integers stored in memory, following a linked list where each entry points to the next via an offset stored in the following word.
- `iterate_table_and_call`: The routine walks a short table pointed to by a global variable, skipping entries until it finds a negative sentinel.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_returned_value_and_flag`: The routine calls another routine and stores the returned value into a field of a structure pointed to by a register.
- `initialize_game_state`: The routine calls several other routines and writes constants to fields of a structure pointed to by a register, clears and sets bits in several bytes of that structure, and then enters a loop that repeatedly calls other routines and manipulates flags in the structure, with a branch that traps and loops forever if a flag byte is zero.
- `decode_tile_index`: The routine reads a byte from a global variable using an index stored in a structure, forms a 16-bit value from that byte and the index, and writes the value and its multiples into the structure.
- `wait_for_flag_then_signal`: The routine calls another routine, then loops until a short value at an offset from an uninitialized pointer is non-zero, then calls the same routine again and returns.
- `invoke_indirect_via_table`: The routine loads a pointer from a global variable, uses it to fetch a function pointer from an offset in a table and calls the resulting routine indirectly.
- `do_initial_setup`: The routine executes a library routine, then calls itself three times before returning.
- `recursive_self_calls`: The routine calls another routine three times in a row and then returns.
- `copy_state_flag`: The routine copies a 16-bit value from one field of a structure pointed to by A5 to another field of the same structure.
- `process_animation_callbacks`: The routine invokes a series of callbacks stored in tables and copies several values from a pointer into fields of a global structure, then decrements a counter at that pointer and invokes more callbacks from the same tables.
- `execute_callback_table_entries`: The routine calls several routines via indirect table lookups using a global variable as a base, then sets a byte in memory pointed to by a pointer and invokes two more routines via indirect table lookups from a pointer, followed by additional indirect calls from the global variable base and from two other pointers.
- `invoke_callback_twice_store_results`: The routine loads a function pointer from a table pointed to by a global variable, calls the function, stores the returned value from register A0 into a global variable, then calls the same function again and stores the second returned value from register A0 into another global variable.
- `initialize_extra_outputs`: The routine calls another routine via a table indexed by a global variable, stores the returned pointer in a global variable, then calls a routine through a table indexed by that pointer and writes a constant to a global variable.
- `invoke_indirect_via_table_2`: The routine loads a pointer from a global variable, reads an index from an offset in that structure, and calls the function located at the computed address.
- `set_flag_in_structure`: The routine calls several other routines and then sets a bit in a byte located at a fixed offset from an address in a register.
- `set_flag_and_call_helpers`: The routine calls several other routines and then checks a flag byte in a structure pointed to by A5.
- `call_other_routines`: The routine calls another routine three times and calls a second routine once, then returns.
- `set_status_flag_on_conditions`: The routine checks three short fields in a structure pointed to by A5 for zero.
- `set_status_flags_on_object`: The routine checks three short fields in a structure pointed to by a register.
- `clear_status_flag_and_wait`: The routine clears a status bit in a byte at an offset from a pointer in A5, calls another routine, then waits in a loop calling another routine until a different bit in the same byte is set, and clears the status bit again.
- `self_recursive_call`: The routine is empty except for a single call to itself, then returns.
- `decrement_counter_and_dispatch`: The routine checks a short counter at an object offset.
- `update_state_flags`: The routine reads and writes a byte at an offset from a pointer in A5, using it as a set of flags.
- `update_call_result_and_flags`: The routine calls a function pointer stored at the start of a structure pointed to by A0, saves the returned value into the structure, and copies a word from the structure to a field and back.
- `negate_indirect_table_call_result`: The routine loads a function pointer from a table pointed to by a global variable, calls that function, and returns the negated short result.
- `negate_indirect_table_call`: The routine loads a pointer from a global variable, indexes a table of function pointers using a short value from that structure, calls the retrieved routine, and returns the negated short result.
- `decode_input_flags`: The routine calls another routine through a table indexed by a short value from a global variable and builds a 32-bit result from the returned value and a second routine call.
- `loop_with_two_calls`: The routine calls another routine once, then repeatedly calls two other routines in a loop until a global variable is zero.
- `loop_until_flag_cleared`: The routine calls another routine, then enters a loop where it repeatedly calls two other routines and then calls itself twice.
- `load_track_tilemap_data`: Decompresses run-length encoded tile data for a specific track into a destination buffer and initializes associated palette and state pointers.
- `copy_tilemap_with_palette`: The routine copies a block of tile entries from a source pointer to a destination buffer obtained from a global variable, writing each tile index and a palette value.
- `copy_and_transform_word_pairs`: The routine copies a block of word pairs from an input buffer to an output buffer via a pointer held in a global variable, writing each source word and a running offset.
- `copy_tile_data`: The routine copies a 16-bit value from a global table indexed by a short stored in a structure, stores it to two 16-bit fields in a buffer, and then clears a 16-bit flag in the structure and a 32-bit field in the structure.
- `initialize_vehicle_state`: The routine populates a vehicle state structure with data from global tables and configuration arrays, handling conditional logic for different vehicle types or states.
- `set_short_values_in_table`: The routine calls another routine and then iterates over a table of pointers stored in a global variable.
- `reset_player_state_flags`: The routine writes a computed value derived from a global variable into a word at an offset from an uninitialized pointer, then clears two words at other offsets from the same pointer.
- `self_recursive_calls`: The routine calls another routine three times in a row and then returns.
- `wait_for_input_flag`: The routine sets up a set of flags and counters in a global structure, then repeatedly copies a counter value and polls two bits in a status byte, calling other routines while waiting.
- `copy_word_from_offset`: The routine copies a 16-bit value from an offset to another offset within the same structure pointed to by an uninitialized A5.
- `increment_counter_and_update_flags`: The routine increments a short counter stored in a global variable and wraps it to a fixed value, then calls another routine three times.
- `update_flag_and_copy_state`: The routine updates a flag word in a global variable based on a comparison between an input value and a table entry, then copies two words from a structure pointed to by a pointer into the global variable and clears a word in the structure.
- `bubble_sort_swap_fields`: The routine performs a bubble sort pass over a table of pointers held in a global variable.
- `copy_table_entries_to_structs`: The routine iterates over a table of 16-bit words pointed to by a global variable and, for each entry, calls another routine and stores the returned pointer in an array.
- `clear_two_word_fields`: The routine clears two 16-bit fields in a structure pointed to by a register, then reads a 32-bit value from a global table using an index stored in the structure and writes that value to two 32-bit fields in the same structure.
- `reset_input_state`: The routine clears several memory locations, loads two values from an address derived from a register, then repeatedly writes a constant to a memory location and waits until a global variable no longer has a specific bit set, repeating this wait a fixed number of times.
- `clear_custom_chip_regs`: The routine clears a set of custom chip registers and copies two words from a structure in A5 to custom registers.
- `fill_patterned_tile_buffer`: The routine reads a byte stream from a pointer supplied in A0 and a repeat count from the stream, and writes the byte into a 19 by 19 tile area of a global buffer, repeating the byte for the indicated count.
- `decode_run_length_tile`: Iterates through a source buffer to decompress run-length encoded tile data into a destination buffer.
- `decode_and_store_palette`: The routine reads a table of 16-bit words from a global variable and expands each entry into a 32-bit value using a rotate and byte-swap operation.
- `calculate_display_offsets`: Computes memory addresses for display blocks by iterating through a linked list of short integers and storing calculated offsets into global variables.
- `check_and_update_flags`: The routine reads a global variable and two groups of short values from a structure pointed to by A5, calls another routine several times, and conditionally calls it again based on the high bits of the variable and on each short being below a threshold.
- `update_collision_mask_bits`: Reads word-sized values from a source table and applies bitwise AND and OR operations to specific offsets within a destination structure.
- `copy_palette_entries`: The routine initializes a hardware register block and then iterates over a short table pointed to by a global pointer, updating two global integers with base values plus the table entry and computing two other globals from the next two shorts.
- `compute_display_block_offsets`: The routine reads short entries from a table pointed to by an address in A5 until a negative sentinel, and for each entry it writes computed offsets and coordinates into a set of global variables.
- `process_palette_entry_list`: Iterates through a linked list of short integers to calculate memory addresses and store values into global variables.
- `copy_sprites_to_buffer`: The routine reads a table of short entries until a negative sentinel, computes a couple of offsets and a packed byte from the entry, writes them to a set of chip registers, then busy-waits on a global variable bit before returning.
- `fill_tile_block`: The routine reads four 16-bit values from a global table indexed by the low word of D0 and copies them into a block of memory pointed to by a pointer stored in A5.
- `compute_angle_index_from_vector`: The routine computes a magnitude from two 16-bit inputs, refines a divisor by iterative averaging, derives a scaled ratio and a sign test, then performs a binary search in a global short table using a step size that halves each iteration to locate the entry matching the ratio.
- `shifted_index_search`: The routine takes a short from D0 and a pointer in A0 and repeatedly halves an offset while comparing a short from D4 with table entries addressed by A0 plus the running index.
- `swap_registered_pointers`: The routine swaps two 32-bit values stored at fixed offsets from a base pointer in memory.
- `fill_two_short_tables`: The routine fills two short tables pointed to by offsets from a5 with two arithmetic progressions, incrementing the first by a constant and the second by a constant each iteration, stopping after a fixed number of iterations.
- `store_pointer_and_mark`: The routine copies a 32-bit value from an offset in a structure to another offset, then calls another routine.
- `swap_state_fields`: The routine swaps several 32-bit fields within a structure pointed to by a5 and then calls a function through a pointer table.
- `store_value_and_call_indirect`: The routine stores the value from a register into a field of a structure pointed to by a register and then calls an indirect routine obtained from a table pointed to by another field of the same structure.
- `reset_input_state_2`: The routine clears two words in a structure pointed to by a register, writes a constant to a third word, then calls a function through a pointer table and calls another routine.
- `reset_state_and_dispatch_callback`: The routine writes a constant to a word at an offset from a pointer in A5, clears another word at a different offset from the same pointer, then calls a function obtained by indexing a table pointed to by a global variable.
- `initialize_global_state`: The routine writes two 16-bit values to offsets within a global variable and then calls a function obtained by indexing a table stored in another global variable.
- `reset_timer_and_loop`: The routine reads a signed short from an offset in a structure pointed to by A5.
- `reset_game_state`: The routine clears a byte in a structure, zeroes a short field, sets two word pointers to -1, zeroes 240 then 32 longwords in a global array, then repeatedly dereferences and stores four pointer fields in a structure while calling another routine and decrementing a short counter, and finally clears a global variable.
- `process_event_queue`: The routine clears a small fixed-size array in the stack frame and then walks a linked list of event structures stored in a global variable.
- `swap_linked_list_nodes_2`: Manipulates pointers in a linked list structure by swapping the head pointer with the next node.
- `empty_routine_placeholder`: The routine performs no operations and returns immediately.
- `decode_message_bytes`: The routine reads a short counter from a structure and, if it is zero, repeatedly calls another routine via a jump table until a non-negative word is obtained, then stores the high and low bytes of that word into two fields of the structure and updates an offset field.
- `empty_routine_7`: The routine contains no instructions; it is empty.
- `iterate_linked_lists_with_callback`: The routine walks a set of sixteen linked lists stored in a global table.
- `mask_global_words`: The routine masks a range of words in a global variable using a bit mask derived from a flag in a structure.
- `set_tile_flags`: The routine clears a flag in a structure and, if a bit index is present, computes a mask from it and ORs that mask into a range of 32-bit words in a global variable.
- `clear_collision_flags`: The routine clears a flag field in a structure when a condition is false and otherwise computes a combined bitmask from two tables referenced through pointers in the structure, masks it with a field and a bit mask, and then scans a global table backwards to accumulate a secondary mask from entries whose flag matches the computed mask, finally storing the OR of that mask with a constant into the structure.
- `increment_counter_and_link_node`: The routine increments a short counter in a structure pointed to by A5, stores a pointer from A5 into a caller-provided pointer, links the caller’s pointer back into the structure, then calls another routine twice and conditionally writes a value into a field of a structure pointed to by A1.
- `copy_table_with_early_exit`: The routine copies 16-bit entries from a table pointed to by a pointer, using each entry as an index into a destination buffer, and stops when it finds a sentinel value.
- `copy_word_list_with_early_exit`: Iterates over a word list terminated by -1; for each entry it copies a word from the list into a destination area indexed by the entry, aborting if the entry matches in_D0w.
- `other_routine`: The routine is empty and returns immediately without performing any operations.
- `check_flag_and_copy_field`: The routine checks a flag in a 16-bit field at an offset from a pointer in a2.
- `update_input_pointer`: The routine updates a short field from a mask and copies a dword from one offset to another, or advances a pointer through a table of shorts until a sentinel is found and stores the next entry back.
- `check_flag_and_copy_value`: The routine checks a flag in a 16-bit field at an offset from a pointer in A2.
- `store_sum_to_offset`: The routine adds a 16-bit value from a register to a value in another register and stores the 32-bit result through a pointer derived from a register plus a fixed offset.
- `clear_entity_table_entries`: The routine clears a block of four consecutive longwords in a structure pointed to by a pointer when a byte mask is zero; otherwise it writes a short value or a long value into an offset within the same structure based on a flag.
- `apply_vehicle_flags_and_offsets`: The routine reads a pointer from an object structure and, if present, uses a flags word from that structure to conditionally copy or accumulate position offsets, adjust a short angle value up or down, copy a two-byte field, optionally copy a pointer under combined flag conditions, and set or clear a sign bit in a word field.
- `copy_flags_and_fields`: The routine copies selected fields from a source structure to a destination structure based on flag bits in a control word.
- `set_flag_from_source_word`: The routine copies the sign bit from a source word into the high bit of a destination word.
- `set_flag_in_table_entry`: The routine sets a flag in a table entry by OR-ing a bit into a 16-bit field at a fixed offset from an address in a register.
- `decrement_and_unlink_list_entry`: The routine decrements a reference count in a structure pointed to by a parameter, and if the count reaches zero and a flag is clear, it unlinks the structure from a doubly linked list, calls another routine, and decrements a counter in a global structure.
- `call_other_routine`: The routine is empty except for a single call to another routine.
- `set_flag_in_word_at_offset`: The routine ORs a single bit into a 16-bit word located ten bytes after a pointer in a register, using a bit index taken from another register masked to five bits.
- `clear_flag_in_word`: The routine clears a single bit in a 16-bit word pointed to by a pointer in a register.
- `set_bit_in_byte_table`: The routine sets a single bit in a byte taken from a table.
- `clear_bit_in_byte_array`: The routine clears a single bit in a byte taken from a byte array.
- `call_a_helper_routine`: The routine is a small wrapper that calls another routine and then returns.
- `invoke_other_routine`: The routine calls another routine and then returns.
- `dispatch_reset_callback`: Acts as a wrapper that immediately calls another routine to reset state and dispatch a callback.
- `check_flag_and_call`: The routine tests a global variable and, if it is non-zero, calls another routine.
- `initialize_game_structures`: Calls a sequence of four other routines in succession to initialize game state structures.
- `store_table_entry_to_struct`: The routine writes the incoming pointer to a location derived from a caller-provided pointer, calls another routine, then stores a short from a register into two different structures and copies a word from a register and two words from a table indexed by the short into fields of the output structure.
- `update_vehicle_state_from_table`: The routine reads a pointer from an offset in a2, uses entries from that table together with values from a5 to compute two short values stored at offsets in a2.
- `update_collision_flags`: Iterates over a set of collision flags derived from a mask in a global variable and, for each active flag, updates a byte in a structure pointed to by an entry in a table and writes a clamped byte to another field, calling another routine for the update.
- `compute_collision_angles`: Calculates angle indices from vector differences between two structures and stores the results.
- `reset_input_state_3`: The routine writes several word values into a structure pointed to by an address register, clears a few fields, masks a byte in the first word, decrements a word field, sets another word field to all ones, and then calls another routine twice.
- `check_flag_and_call_2`: The routine tests a bit in a byte from a global variable and, if set, calls another routine.
- `add_bcd_value_to_bytes`: Adds a value to two specific byte locations in memory, applying BCD adjustment after each addition.
- `add_bcd_to_table_bytes`: The routine adds the low byte of an input register to two consecutive bytes in a table pointed to by an address register plus offsets, stores the result back, and performs BCD adjustment on each result.
- `clear_short_flags`: The routine clears a series of short fields in a table of structures.
- `bubble_sort_linked_list`: The routine repeatedly performs a single pass over a small list of four entries, swapping each adjacent pair when the second entry’s field is smaller than the first’s, and repeats the passes until a pass makes no swaps.
- `bubble_sort_two_field_table`: The routine performs a bubble sort pass over a table of pointers stored in a global variable, swapping adjacent entries while the second entry's field is smaller than the first's, repeating until no swaps occur.
- `copy_table_to_buffer`: The routine copies a 23-word block from a source table pointed to by a global variable into a buffer located at an offset from the return value of another routine, and then copies a 31-word block from a table pointed to by a register into a destination table pointed to by a global variable.
- `find_matching_entry`: The routine walks a table of short pointers stored in a global variable and compares the first five shorts of each entry with a short pattern supplied via a pointer in A4.
- `check_flag_and_call_3`: The routine tests a bit in a global variable and, if set, calls another routine.
- `swap_a0_with_global`: The routine swaps the value pointed to by a register with a global variable and then stores the register value into two locations in the same global structure.
- `swap_a0_with_global_2`: The routine swaps the value pointed to by a pointer in register A0 with the contents of a global variable.
- `interpret_byte_stream_loop`: The routine reads a byte stream from a pointer in A0 until a zero terminator is found.
- `store_word_via_pointer`: The routine stores a 16-bit word from a pointer into a global variable.
- `store_byte_to_word_table`: The routine checks a byte pointed to by a pointer and, if the value is less than 200, stores that byte as a 16-bit value into a word field located at an offset from a base pointer.
- `call_two_routines`: The routine calls another routine twice and then returns.
- `add_short_to_global_offset`: The routine adds the value pointed to by a pointer in a register to a short stored at a fixed offset from a base pointer in another register, writing the sum back to that offset.
- `add_short_to_global_offset_2`: The routine adds the byte pointed to by a pointer to a short stored at an offset from a base pointer and writes the sum back to that short.
- `recursive_self_call`: The routine calls another routine twice and then returns.
- `self_calling_stub`: The routine consists of a single call to another routine and then returns.
- `copy_word_and_accumulate_short`: The routine copies a 16-bit value from an offset in a structure pointed to by A5 to another offset in the same structure.
- `clear_two_byte_table`: The routine clears a two-byte table by writing zeros to consecutive entries.
- `fill_short_table_from_index`: The routine reads a short index from a global variable, computes a base value from it, then fills a short table in memory with a decreasing arithmetic progression until the index reaches -1.
- `clear_table_and_call`: The routine reads a length from a global variable, clears a block of words in a table pointed to by a global variable, and then calls another routine.
- `fill_decreasing_word_table`: The routine reads a 16-bit value from a global variable, subtracts one, and then fills a table of 16-bit words in memory with a decreasing sequence starting from half of that value.
- `add_short_arrays`: The routine reads a count from a global variable, then adds two short arrays element-wise and stores the results into a third short array.
- `store_byte_from_pointer_to_offset`: The routine stores a byte from a pointer into a short at a fixed offset from a register.
- `store_byte_from_pointer_to_table`: The routine copies a byte pointed to by a pointer into a short field of a table indexed by a base pointer.
- `search_linked_list_entry`: Traverses a linked list of integers to find a match for a target value.
- `store_byte_to_table`: The routine stores a byte from a pointer into a table entry addressed by a global variable.
- `decrement_counter_loop`: The routine reads a byte from an unspecified pointer, subtracts one to form a short counter, then repeatedly calls another routine while decrementing the counter until it reaches -1.
- `copy_table_entries`: The routine searches a table of short pointers for an entry matching a value pointed to by a2, then copies a fixed number of 32-bit words from the found entry into a global buffer at an offset from a5.
- `copy_word_to_offset`: The routine copies a 32-bit word from an offset within a block pointed to by an address register to another offset within the same block.
- `clear_a5_offset_word`: The routine stores a zero word into a memory location computed as the contents of register A5 plus a fixed offset.
- `set_flag_in_memory`: The routine stores the value 1 into a two-byte location at a fixed offset from an address held in a register.
- `call_indirect_via_table_entry`: The routine loads a function pointer from an offset in a global variable and calls it if the pointer is non-zero.
- `process_track_segment`: The routine reads a segment descriptor from a table pointed to by a global variable and uses a global offset to index into it.
- `xor_decrypt_screen_buffer`: The routine expands a 4-word key with a 16-round feedback derived from a byte table, rotates the key words and a mask by a variable amount, and XORs the mask into four 32-bit words of a destination buffer while OR-ing the rotated key words into the same locations.
- `decrement_loop_with_callback`: The routine repeatedly calls another routine through a function pointer in A6 while decrementing a counter in D4 until it reaches -1.
- `decrement_table_counters`: The routine reads a global variable and iterates over a table of entries.
- `do_nothing_routine`: The routine contains no executable instructions and returns immediately.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_offset_multiple_times`: Calls another routine four times while adding a value to a word at an offset and incrementing a counter.
- `increment_offset_three_times`: Calls another routine three times while adding a value to a word at an offset and incrementing a counter.
- `increment_offset_once`: Calls another routine once while adding a value to a word at an offset and incrementing a counter.
- `update_bitmask_blending_table`: Calculates a bitwise mask based on a coordinate-derived index and performs selective word-level merging of two data sets into a destination table.
- `update_bitmask_table`: Performs bitwise AND and OR operations to update several word-sized elements in a table using values from registers.
- `update_table_from_high_words`: Extracts the high words from two registers and uses them to perform bitwise updates on a series of table entries.
- `blend_table_values`: Uses bitwise logic to selectively merge two sets of word values into a table based on a mask derived from a register.
- `write_constant_values`: Stores four specific constant values into a memory block at offsets from a pointer and calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_sprite_attributes`: Increments a counter in a data structure, adjusts offset values in a global state block, and copies a table of bytes into a buffer.
- `store_returned_value`: The routine calls another routine and stores the value returned in a register into a global variable via a pointer offset from a register.
- `increment_byte_field`: The routine increments a byte stored at a fixed offset from an address in a register.
- `decrement_byte_field`: The routine decrements a byte stored at a fixed offset from an address in a register.
- `indirect_table_call`: The routine loads a pointer from a global variable, reads a short offset from that structure, adds the offset to the pointer and calls the resulting address.
- `invoke_external_routine`: The routine calls another routine and then returns.
- `set_short_and_long_from_pointer`: The routine reads a pointer from an argument structure, selects a 32-bit value from that structure or from a global variable, splits the value into low and high 16-bit halves and stores them into two short fields of the pointed-to structure.
- `decrement_counter_and_call`: The routine reads a short value from a table pointed to by a global variable, decrements it by one, and traps on underflow.
- `setup_sprite_with_bplcon2`: Similar to R1, this routine increments a counter, copies a larger table of bytes into a buffer, and invokes a dispatch function.
- `add_byte_to_two_table_entries`: The routine reads an integer from a global variable, uses it as a base to locate two bytes in a table, adds the byte in D0 to each of those bytes, and then calls another routine.
- `invoke_callback_with_optional_extra`: The routine reads a pointer from a structure, optionally calls a callback through a table and captures its extra output, then calls another callback through a table passing the structure pointer and the captured value.
- `decrement_and_dispatch_callback`: The routine decrements a counter in a structure pointed to by a global variable and then invokes a callback stored in a table via an index.
- `decrement_counter_with_borrow_check`: The routine calls another routine, loads a short from memory pointed to by a global variable plus an offset, decrements the short by one and checks for a borrow.
- `fill_sprite_attribute_table`: The routine writes a fixed pattern of bytes into a buffer pointed to by a register, repeating the pattern for a number of entries determined by a short value in memory.
- `conditional_callback_dispatch`: The routine compares a short value from a register with a short stored at an offset from a pointer and, if they differ, calls a routine obtained by indexing a table through a pointer.
- `process_message_queue`: The routine walks a table of 32-bit message entries pointed to by a pointer in A0, using a count from A6, and processes each non-zero entry.
- `apply_delta_to_entity_fields`: The routine adds a signed short delta to three 16-bit fields at fixed offsets from a base pointer and adds the same delta to a series of bytes spaced by a fixed stride, the count being taken from a 16-bit field in the structure.
- `store_callback_and_dispatch`: The routine stores an argument pointer and a word into fields of a structure pointed to by a register, then checks a flag byte in that structure.
- `set_global_pointer`: The routine calls another routine and then stores a constant value into a global variable accessed via an uninitialized pointer offset.
- `clear_table_entry`: The routine calls another routine and then stores a constant value into a global variable accessed via an offset from a register.
- `read_offset_from_pointer`: Reads a 32-bit value from ten bytes after the address held in an uninitialized pointer register and returns that value minus one.
- `call_indirect_table_entries`: The routine loads a pointer from a global variable and calls an indirect routine selected by a short offset from a table entry, then loads a second pointer from the same global variable and calls another indirect routine selected by a short offset.
- `copy_descriptor_fields_to_state`: The routine copies several 16-bit fields from a descriptor pointed to by a0 into a state structure pointed to by a6, clears some state words, and then either initializes the state from the descriptor or updates it via a callback in a1.
- `update_pseudo_random_state`: The routine updates several short fields in a structure pointed to by A6 by adding input values and mixing a short field with a mask to produce a pseudo-random index.
- `call_indirect_via_table_3`: The routine reads a short value from an offset within a pointer in A4, adds it to the pointer, and calls the resulting address as code.
- `read_word_from_pointer_offset`: The routine reads a 16-bit word from an address formed by adding a constant offset to a pointer in a register and returns that value.
- `return_state_block_pointer`: Returns a 64-bit value constructed from four 16-bit words located at specific offsets within a global state block.
- `invoke_indirect_through_table`: The routine loads a pointer from a global variable, reads a short offset from that pointer, adds it to the pointer and calls the resulting address.
- `initialize_collision_table`: The routine stores a sequence of increasing 32-bit values into a table pointed to by a pointer stored in a global variable, then writes a final sentinel value, and calls another routine.
- `call_single_routine`: The routine performs a single call to another routine and then returns.
- `call_two_routines_2`: The routine calls another routine and then calls another routine.
- `call_copy_and_mask_routines`: Calls two other routines sequentially: one that copies a word table and another that masks tile flags.
- `copy_word_table`: The routine copies a sequence of 16-bit words from a source pointer to a destination buffer.
- `copy_word_table_2`: The routine copies a block of word-sized values from a source pointer to a destination pointer.
- `minimize_and_iterate_table`: The routine reads a limit from a global variable and two values from two tables via function pointers stored in each table, keeps the smaller of the two values and the limit, then repeatedly skips negative entries in a short table, calls another routine, adds its short result to a value in D1 and stores the sum through a pointer in A1.
- `accumulate_table_values`: The routine walks a short table pointed to by a pointer in A1, stopping when it finds a sentinel value of -1.
- `decrement_loop_with_table_scan`: The routine repeatedly decrements a counter and scans a table of signed 16-bit values using a pointer.
- `apply_table_mask_xor`: The routine reads two 16-bit values from a table indexed by the first input word and, using a pointer and a count taken from the caller’s stack frame, applies a mask-and-xor operation to a block of 16-bit words, or to a single word when the second input is non-negative.
- `mask_tile_flags`: The routine receives a table pointer and a count and iterates over the table, masking each entry with a constant to clear flag bits while preserving the low bits.
- `pack_color_components`: The routine combines a 16-bit input value with a 16-bit multiplier and a 32-bit input value to produce a 32-bit result.
- `check_and_call_indirect`: The routine calls a helper to obtain an index and, if the index is non-negative, it reads a short offset from a table pointed to by an uninitialized pointer and calls the routine found at that offset via an indirect call.
- `set_status_flag`: The routine calls another routine and then stores the value four into a byte at an offset from an uninitialized pointer.
- `empty_routine_9`: The routine contains no instructions and returns immediately.
- `set_flag_and_clear_counter`: The routine checks a flag stored in a structure pointed to by a register.
- `reset_state_flags`: The routine calls another routine and then clears a 32-bit word at an offset from an address in a register and writes the byte value 2 at the next offset.
- `check_and_update_state`: The routine checks a field in a structure pointed to by a register.
- `process_path_string_dispatch`: Checks a string for a backslash or colon character.
- `dispatch_object_method`: Checks input registers to determine whether to decrement a counter and trigger a trap or to invoke a method pointer from a table based on object state.
- `call_via_table`: The routine tests a register value and then calls a function obtained from a table entry, using an offset stored in the table entry.
- `select_and_call_handler`: The routine selects a function pointer from a table based on a flag in a structure and calls it, then calls a second handler from another table using the selected pointer, and finally calls a third handler from the same selected table passing the structure pointer.
- `invoke_dispatch_functions_twice`: Calls a function via a dispatch table using an offset from a pointer in D1.
- `store_word_triple`: The routine copies three consecutive 16-bit values from a source pointer into three consecutive 16-bit fields of a structure pointed to by an index register, then returns a constant.
- `reset_state_fields`: The routine clears three consecutive word fields in a structure pointed to by the address register and then calls another routine.
- `read_pointer_offset`: The routine returns the 32-bit value stored at an offset from a pointer passed in a register.
- `set_bit_flag`: The routine reads a signed byte from a memory location pointed to by a register, uses the low seven bits to index a byte table and a bit mask, and either sets or clears a bit in a byte table depending on the sign of the value.
- `clear_bit_from_byte_table`: The routine clears a single bit in a byte taken from a table.
- `clear_state_fields`: The routine clears four consecutive long words in a structure pointed to by an address register and then calls another routine.
- `empty_stub_routine`: The routine contains four instructions and performs no observable work; it ends immediately.
- `compute_bitmask_from_table`: Iterates through a table of signed 16-bit values.
- `check_signed_byte`: The routine loads a byte from a memory location pointed to by a register, tests whether the value is greater than -1, and returns that byte if the test succeeds, otherwise returns zero.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `set_initial_state_from_table`: The routine stores a byte from a register into a structure field and copies a signed short from a table pointer into four consecutive integer fields of the structure.
- `configure_animation_pointers`: The routine selects a pair of fixed constants depending on whether a flag in D1 is zero, stores the selected values into two consecutive words of a structure pointed to by A6, and then calls another routine.
- `decrement_counter_and_dispatch_2`: The routine checks a flag in a structure pointed to by a register.
- `decrement_counter_and_call_2`: The routine reads a short from a table pointed to by a global variable, decrements it, and traps on borrow.
- `increment_counter_and_store_pointer`: The routine stores a byte from a register into a structure field and copies a pointer into another field, then increments a short counter at an offset from that pointer and increments a short counter at an offset from the stored pointer.
- `compare_byte_to_global`: The routine loads a byte from D0 and compares it with a byte stored at an offset from a pointer in A6.
- `store_word_then_call`: The routine stores a 16-bit value from a register into a global variable via a pointer and then calls another routine.
- `copy_palette_from_buffer`: The routine copies a set of bytes from a buffer pointed to by a register into a structure pointed to by another register, then computes a short value from fields of that structure and stores the result.
- `initialize_game_state_2`: The routine calls two other routines and returns the value it received in D0.
- `clear_two_bytes_from_pointer`: The routine stores zero into a two-byte field and a one-byte field at fixed offsets from an address in a register.
- `set_flag_and_increment_counter`: The routine writes a constant byte to an offset in a structure pointed to by a register, then increments a 16-bit value at another offset in the same structure.
- `store_two_values_into_struct`: The routine stores a 32-bit value from a register into a field of a structure pointed to by a pointer, stores a 16-bit value from a register into the next field, and clears the following 16-bit field.
- `decrement_loop_with_callbacks`: The routine checks a flag in a structure and, if clear, invokes a callback three times, storing each call’s A0 result into three fields of the structure.
- `decrement_counter_loop_2`: The routine repeatedly calls two indirect routines obtained from tables pointed to by a global variable and another pointer, then decrements a short counter at an address pointed to by a pointer and a separate short counter until the latter reaches -1, and finally increments a short value at an offset from another pointer.
- `call_indirect_through_table_3`: The routine calls another routine and then calls an indirect routine obtained by adding an offset from a global variable to the global variable and indexing a table of code pointers.
- `call_indirect_and_store_result`: The routine calls another routine through an indirect pointer formed from a global variable and a short offset, then stores the returned value from D1 into a location addressed by another global variable plus a constant offset.
- `set_status_from_flag`: The routine reads a flag byte from a global variable via an indirect pointer, checks a length field and a colon character in a string, and stores a small status code into another field of the same structure.
- `other_routine_2`: The routine is empty except for a single call to another routine, then it returns.
- `initialize_game_state_3`: The routine writes several fields of a structure pointed to by a register, clears a small table of 15 words, sets a few flags and size values, calls another routine, and then triggers a trap if a pointer argument is non-zero, followed by an unconditional trap.
- `compare_byte_stream`: The routine compares a byte stream pointed to by a0 with a byte stream pointed to by a global variable.
- `read_and_update_flags`: The routine reads a pointer from a global variable, calls another routine via a table entry, stores the result into a flag field, and if a flag mask is set it clears a field, builds a return value from a masked flag and a byte, and may trap.
- `execute_traps_and_add_value`: Executes a Trap #5, then adds a value from a global variable to an input value and stores the result in a local structure.
- `reset_state_and_check_flags`: The routine clears a global variable, sets a flag bit in a byte at an offset from an address in a register, and then checks two conditions involving a global variable and a flag bit, trapping if either is set.
- `clear_pending_input_flag`: The routine reads a value from a register and a global variable, and if the masked result is zero it stores the register value into another global variable, clears a flag in a global variable, and then performs a trap.
- `call_indirect_through_table_4`: The routine calls another routine and then calls an indirect routine obtained by indexing a table with an offset stored in a global variable.
- `decrement_counter_and_call_3`: The routine reads a short value from an offset in a structure pointed to by a register, decrements it, writes it back, and if the result is zero it calls a routine whose address is taken from an offset in another structure pointed to by a register.
- `clear_flag_in_buffer`: The routine calls another routine and then stores a zero word into a global variable referenced via an uninitialized pointer.
- `increment_flag_and_call`: The routine checks a short flag at an offset from a pointer in register A6.
- `execute_callback_chain`: The routine walks a singly linked list whose head pointer is read from a global variable.
- `call_indirect_via_table_4`: The routine loads an address from a global variable and calls the routine stored at an offset from that base.
- `store_byte_and_call_indirect`: The routine stores a byte from a register and a pointer from a register into a structure pointed to by a register, then calls another routine indirectly via a table indexed by a short value.
- `call_indirect_through_table_5`: The routine loads a function pointer from a global variable and calls it indirectly.
- `link_list_nodes`: The routine increments a counter at an offset in a passed structure, then walks a linked list of structures using fields at fixed offsets until a condition on a byte field is met.
- `link_list_nodes_2`: The routine increments a short field at offset 4 in a structure pointed to by a0, then links two structures together by writing pointers into fields at offsets 6 and 10 of both structures, using a value taken from a global variable.
- `link_two_nodes`: The routine increments a short field at offset four in the structure pointed to by a0, then links two structures together by writing pointers to each other in fields at offsets six and ten, and updates the back-pointer of the previously linked node.
- `increment_and_link_node`: The routine increments a short field at offset four of a pointer in A0, then links a structure pointed to by A0 with a structure referenced by a global variable.
- `swap_linked_list_nodes`: The routine swaps the next and previous pointers of two linked list nodes referenced through a pointer in A0 and a global variable.
- `iterate_callback_table`: The routine walks a linked list of structures obtained from a global variable.
- `iterate_object_list`: Traverses a linked list of objects, invoking a method on each node until a non-zero return value is encountered or the list ends.
- `loop_dispatch_until_nonzero`: Repeatedly calls a function via a dispatch table using an offset from a pointer.
- `reset_linked_list_head`: The routine walks a singly linked list whose head pointer is stored in a global variable, decrementing a short counter at each node and following the next pointer until a null terminator is reached.
- `decrement_linked_list_counters`: The routine walks a singly linked list via a global pointer and decrements a 16-bit counter stored in each node.
- `decrement_linked_counter_chain`: The routine walks a singly linked list via a pointer in D0 and a node pointer in A0.
- `increment_counter_if_nonzero`: The routine reads an integer from a global variable via a pointer stored in a register.
- `initialize_stack_frame`: The routine stores a pointer to its own stack frame in a global variable and clears a word on the stack.
- `update_linked_list_pointers`: Increments a counter in a structure and calls a dispatch function.
- `unlink_and_decrement_counter`: The routine takes a structure pointed to by a0, reads two pointers from offsets 6 and 10, links them to each other by writing each into the other's structure at the opposite offset, clears the two pointer fields in the original structure, decrements a short counter at offset 4 and traps on borrow.
- `swap_next_pointer`: The routine reads a pointer from an address on the stack, follows it to a second pointer, and if the second pointer is non-zero it copies a value from the first structure to the second, updates the stack pointer to the second structure, and clears two fields in the first structure.
- `empty_routine_10`: The routine contains no instructions and returns immediately.
- `store_byte_via_pointer`: The routine stores a byte from a register into an offset of a pointer passed on the stack.
- `unlink_double_linked_node`: The routine reads two pointers from a structure pointed to by an address in a register, writes them into each other's link fields to maintain a bidirectional link, then clears the two fields in the original structure.
- `copy_string_with_length_limit`: The routine copies a null-terminated source string to a destination buffer using a signed length in d0, handling negative lengths by counting characters first, then stores the resulting length and the remaining character count in fields of a structure pointed to by a6.
- `read_global_variable_offset`: The routine returns the 32-bit value stored at a fixed offset from an uninitialized address register.
- `clear_table_from_index`: The routine takes a pointer to a table and a 32-bit index from a global variable, then iterates through the table using the high and low words of the index as counters.
- `fill_memory_region_loop`: Iterates through a memory region pointed to by a register, writing a constant value to each word.
- `store_value_and_call_2`: The routine stores a value from a register into a global variable via a pointer, calls another routine, then stores a value returned in a register into another global variable via a pointer.
- `store_value_and_call_helper`: The routine stores a value from a register into a field of a structure pointed to by a register, calls another routine, then stores the result from the helper into another field of the same structure.
- `store_value_and_call_3`: The routine stores a value from a register into a global variable, calls another routine, then stores a value returned in a register into a different global variable.
- `call_single_helper`: This routine calls another routine and then returns.
- `read_table_entry`: The routine reads a 32-bit value from an address computed as the contents of register A6 plus a fixed offset.
- `store_struct_fields`: A routine stores four incoming values into a structure pointed to by a register, clears a flag byte and optionally sets its low bit if a short input is non-zero.
- `dispatch_script_or_stack`: The routine selects one of two helper routines based on a flag in a global variable, stores the current tick count and a counter into a context block, then either walks a linked list of extra blocks to build a stack and dispatches through an indirect jump table, or dispatches via a function pointer stored in the context.
- `store_table_entry_and_call_handler`: The routine stores a 32-bit value from a table into a destination computed from an input word and a byte from a pointer, then calls another routine selected by an index derived from the same byte and a byte from the pointer.
- `copy_word_and_dispatch`: The routine copies a 16-bit word from a table pointed to by a pointer argument to a destination computed from an index in D0 and a byte from the pointer argument, then calls another routine selected by an index formed from the same D0 high byte and a byte from the pointer argument.
- `dispatch_by_table_or_pointer`: The routine checks a stack pointer; if non-zero it performs an indirect jump through a table indexed by a byte from the pointer, otherwise it calls a routine via a pointer stored in A0 plus an offset.
- `empty_stub_return`: This routine is empty and immediately returns without performing any operations or modifying state.
- `empty_routine_11`: The routine contains six 68000 instructions and performs no observable work.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `do_nothing_stub`: The routine is empty and returns immediately without performing any operations or modifying data.
