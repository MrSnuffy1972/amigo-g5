# Game 1722 (volfied (1991)(empire)[cr flt][a3]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01722). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (939664 top-level bytes in 54 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 9 raw-word lines |
| Functions named and described | pass, 372 of 372 named (0 author symbols), 372 of 372 described (24 by rule, 179 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 6669 of 6670 (99.99%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9262 of 9408 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 436100 |
| code | 32052 |
| data | 421439 |
| padding | 10137 |
| unused | 39936 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Volfied (1991)(Empire)[cr FLT][a3].adf:Exit | 2 | 117 | 2 |
| Volfied (1991)(Empire)[cr FLT][a3].adf:ClickMe | 1 | 47 | 1 |
| Volfied (1991)(Empire)[cr FLT][a3].adf:Volfied | 2 | 233 | 8 |
| Volfied (1991)(Empire)[cr FLT][a3].adf:Boot1 | 1 | 45 | 2 |
| Volfied (1991)(Empire)[cr FLT][a3].adf:Boot2 | 1 | 31 | 3 |
| Volfied (1991)(Empire)[cr FLT][a3].adf:Fairlight | 1 | 155 | 3 |
| Volfied (1991)(Empire)[cr FLT][a3].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 126 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 264 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6840 | 234 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 651 | 32 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 876 | 61 |

## Cases handled

- volfied (1991)(empire)[cr flt][a3]/Volfied (1991)(Empire)[cr FLT][a3].adf: file Data/ScreenH.cmp (header 27192 bytes, blocks hold 9272): its blocks are judged as raw blocks outside the filesystem
- volfied (1991)(empire)[cr flt][a3]/Volfied (1991)(Empire)[cr FLT][a3].adf:Data/SpriteA.dat: high-entropy data file (7.91 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a3]/Volfied (1991)(Empire)[cr FLT][a3].adf:Data/SpriteL.dat: high-entropy data file (7.92 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a3]/Volfied (1991)(Empire)[cr FLT][a3].adf:Data/SpriteJ.dat: high-entropy data file (7.96 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a3]/Volfied (1991)(Empire)[cr FLT][a3].adf:Data/SpriteH.dat: high-entropy data file (7.96 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a3]/Volfied (1991)(Empire)[cr FLT][a3].adf:Data/SpriteD.dat: high-entropy data file (7.95 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a3]/Volfied (1991)(Empire)[cr FLT][a3].adf:Data/SpriteO.dat: high-entropy data file (7.95 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a3]/Volfied (1991)(Empire)[cr FLT][a3].adf:Data/SpriteB.dat: high-entropy data file (7.92 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a3]/Volfied (1991)(Empire)[cr FLT][a3].adf:Data/SpriteM.dat: high-entropy data file (7.90 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a3]/Volfied (1991)(Empire)[cr FLT][a3].adf:Data/Sprite@.dat: high-entropy data file (7.91 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a3]/Volfied (1991)(Empire)[cr FLT][a3].adf:Data/SpriteK.dat: high-entropy data file (7.93 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a3]/Volfied (1991)(Empire)[cr FLT][a3].adf:Data/SpriteI.dat: high-entropy data file (7.97 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a3]/Volfied (1991)(Empire)[cr FLT][a3].adf:Data/SpriteG.dat: high-entropy data file (7.92 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a3]/Volfied (1991)(Empire)[cr FLT][a3].adf:Data/SpriteE.dat: high-entropy data file (7.94 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a3]/Volfied (1991)(Empire)[cr FLT][a3].adf:Data/SpriteP.dat: high-entropy data file (7.94 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a3]/Volfied (1991)(Empire)[cr FLT][a3].adf:Data/SpriteC.dat: high-entropy data file (7.96 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a3]/Volfied (1991)(Empire)[cr FLT][a3].adf:Data/SpriteN.dat: high-entropy data file (7.93 bits/byte), classified as packed data
- runtime segment an address: 118 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 255 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-keys): 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-keys): 4520 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-keys): 589 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-keys): 768 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- volfied (1991)(empire)[cr flt][a3]/Volfied (1991)(Empire)[cr FLT][a3].adf: blocks 1657-1712 (bytes 848384-877056) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.35)
- volfied (1991)(empire)[cr flt][a3]/Volfied (1991)(Empire)[cr FLT][a3].adf:Volfied: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- volfied (1991)(empire)[cr flt][a3]/Volfied (1991)(Empire)[cr FLT][a3].adf:Fairlight: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `configure_hardware_and_copy_data` (runtime segment an address): Sets specific values in hardware registers and global variables before entering a loop that writes incrementing values to a memory range. Kind: disk hardware/track check. Pass path: the branch at an address (bne.b an address is skipped if zero, implying success is a zero flag/equality condition).

## Functions


### Volfied (1991)(Empire)[cr FLT][a3].adf:Exit

- `run_program_entry_point`: No pseudo-code is available.
- `call_single_unnamed_routine`: No pseudo-code is available.

### Volfied (1991)(Empire)[cr FLT][a3].adf:ClickMe

- `check_messages_and_open_dos`: No pseudo-code is available for the body.

### Volfied (1991)(Empire)[cr FLT][a3].adf:Volfied

- `allocate_and_free_program_memory`: No pseudo-code is available for the body.
- `call_library_offset_routine`: No pseudo-code is available.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `call_four_unnamed_routines`: No pseudo-code is available.
- `stub_unavailable_pseudocode`: No pseudo-code is available.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `call_subtract_routine_then_routines`: No pseudo-code is available for the body.
- `stub_unavailable_pseudocode_2`: No pseudo-code is available.

### Volfied (1991)(Empire)[cr FLT][a3].adf:Boot1

- `check_task_and_open_dos`: No pseudo-code is available for the body.
- `return_from_subroutine`: Performs a return from a subroutine using two instructions.

### Volfied (1991)(Empire)[cr FLT][a3].adf:Boot2

- `open_intuition_and_dos_libraries`: No pseudo-code is available for the body.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `empty_routine`: This routine contains a single instruction and is empty.

### Volfied (1991)(Empire)[cr FLT][a3].adf:Fairlight

- `set_palette_color_then_call`: No pseudo-code is available for the body.
- `call_two_unnamed_routines`: No pseudo-code is available.
- `call_three_unnamed_routines`: No pseudo-code is available.

### Volfied (1991)(Empire)[cr FLT][a3].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_then_trap_zero`: Calls another routine, stores a fixed constant into a global variable, then executes trap zero and returns zero.
- `wait_for_hardware_flag`: Polls a bit in a global variable until it becomes set, then executes a sequence of other routines and loops.
- `configure_hardware_and_copy_data`: Sets specific values in hardware registers and global variables before entering a loop that writes incrementing values to a memory range.
- `update_memory_values`: Writes several specific values to a memory block pointed to by a register.
- `check_and_increment_byte`: Checks a byte at an offset; if it is negative one, it increments a global variable and loops until the byte is zero.
- `check_byte_and_decrement_global`: Checks a byte at an offset from a pointer; if the byte is negative one, it decrements a global variable and loops until the byte is zero.
- `clear_memory_range`: Fills a range of memory starting at a specific address with zeros using a loop.
- `copy_data_from_table`: Iterates through a list of bytes to copy blocks of data from a source table to a destination memory area.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_global_and_trap`: Stores the address of its own routine in a global variable and then issues a trap-zero instruction.
- `wait_for_flag_handshakes`: Initialises two flag bytes and a counter word in a structure addressed through the A5 register, then runs nested loops that wait for the flag bytes to change state, calling two unnamed routines during each wait.
- `toggle_flag_and_update_table`: Toggles a flag word in a structure addressed through the A5 register; when it reaches zero it advances a word counter there by two and copies a global value into four table entries spaced eight words apart.
- `toggle_flag_and_fill_table`: Toggles a flag word and, when it becomes zero and a counter decremented by two stays nonnegative, copies a global value into four table entries spaced eight words apart.
- `expand_nibble_table_negated`: Calls one unnamed routine 32 times, then expands a table of 32 words into destination entries by taking each word's 4-bit pieces and storing them negated.
- `find_largest_nibble`: Scans the four 4-bit nibbles of the word in the data register, from low to high, and keeps the largest value in the second register.
- `countdown_timer_table_update`: Decrements a countdown at a fixed offset from the data base register and, when it reaches zero, reloads it and walks 31 six-word table entries.
- `decrement_nibble_counter_table`: Decrements a countdown at a fixed offset from the data base register and, on reaching zero, reloads it and visits 31 table words.
- `setup_audio_and_dma_hardware`: Sets the DMA and interrupt enable registers, configures audio and CIA port values, then waits on the video beam position before writing a sample pointer, length, period and volume into each of the four audio channel blocks.
- `build_bitplane_copper_list`: Fills a memory table with register write pairs for four bitplane pointers, each advanced by a fixed row stride, followed by 80 sprite pointer entries cleared to zero.
- `restore_custom_chip_state`: Disables all DMA and interrupt sources, then reloads saved interrupt and DMA values from globals and points the copper location register at a pointer read from a nested structure.
- `store_interrupt_request_constants`: Stores the constants 255 and 32 into two global variables.
- `store_interrupt_control_constants`: Stores the constant 255 into one global variable and 8192 into another, then returns the unchanged data register.
- `report_undecoded_routine_metadata`: The decompiler timed out, so no pseudo-code exists for this routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_trap_routine`: Calls a routine twice, sets a global variable, and executes a system trap.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `verify_and_cleanup_system_libraries`: Checks for the presence of required system libraries and validates their availability.
- `dispatch_indirect_calls`: Copies a string to a global variable and then executes various functions via pointers stored in global variables or tables.
- `swap_word_bytes_in_table`: The routine rotates the bytes of a pointer to a global variable 16 times, writing the high word of the pointer into successive 16-bit entries of a table that starts at the global variable.
- `execute_function_table`: Iterates through a table of pointers and executes functions at those addresses if certain conditions are met.
- `populate_pointer_table`: Iterates through a source table to populate a destination table with values returned by a function call.
- `swap_display_buffers`: Swaps the two display buffer pointers at the end of a frame so the frame just drawn is shown and the next one is drawn into the other buffer.
- `fill_data_table`: Stores a sequence of 16-bit values from a global variable into a table using a loop.
- `copy_word_pairs`: Copies 32 word-sized values from a source pointer into two separate destination locations in a global variable.
- `copy_word_pairs_from_pointer`: The routine copies 16 word pairs from a pointer supplied in A0 into two overlapping locations in a global variable.
- `store_packed_data`: Calculates a packed 32-bit value from two inputs and stores it into two consecutive memory locations.
- `manage_device_io`: Opens and closes hardware devices while managing message nodes and function pointers stored in a global table.
- `cleanup_resource_structures`: Checks several global pointers and, if non-zero, executes associated cleanup routines and modifies structure fields.
- `process_input_event_data`: Evaluates a data structure pointed to by a register to check for specific type flags and character values.
- `tiny_move_rts`: Small routine of 2 instructions that copies a0 to d0, then returns.
- `process_input_character_2`: Checks a character value and a bitmask from a memory location to update global variables or execute a function.
- `process_input_character`: Compares a value in a register against two specific character constants.
- `set_view_and_return`: The routine writes a constant to a global variable, then calls another routine via a table of function pointers, and returns the value received in register A0.
- `clear_table_entry`: The routine clears a global word to zero and stores a pointer to a global variable into two entries of a table pointed to by a global variable.
- `create_message_node`: The routine builds a message node in memory.
- `cleanup_message_struct`: The routine clears two bytes in a structure pointed to by a pointer argument and then invokes two routines via a global variable table, and if a field in the structure is non-zero it also invokes another routine from the same table.
- `set_handler_data`: The routine checks a pointer argument and, if non-zero, calls a function obtained from a global variable table.
- `reset_object_flags`: The routine clears a byte at an offset in a structure pointed to by a parameter and sets two 32-bit fields to all ones, then calls another routine via an indirect pointer from a global variable and returns its result, or returns a constant if the parameter is zero.
- `read_and_close_file`: The routine opens a file, reads a byte from it, seeks within it, and closes it, returning the read byte and a flag.
- `dispatch_through_jump_table`: The routine is documented as having two instructions and is entered only by address-taken reference, called from initialize_message_node_list.
- `initialize_message_node_list`: This routine allocates memory to construct a linked list of message nodes.
- `check_and_call_handlers`: The routine iterates over a small table of flags located relative to its own address, and for each entry it conditionally calls several handler pointers taken from a global structure and calls another routine.
- `update_object_status`: Fetches a pointer from a table using an index, writes two bytes to that pointer, and then calls a function through a pointer stored at an offset within that same structure.
- `unknown_routine_0_0_c18`: It calls another routine to initialize enemy structures, then calls another routine to initialize an object chain.
- `set_object_fields_and_trigger_callback`: The routine indexes into a table using an offset from the first argument, loads a pointer from the table, writes a constant word and a zero byte to fixed offsets in the pointed-to structure, copies a word from the first argument and a word from the second argument to other fixed offsets in the same structure, then calls a function pointer stored in the structure with a fixed negative offset applied.
- `call_table_function`: Loads a value from a table using an index and calls the function located at that address.
- `dispatch_via_table`: The routine takes a short index from a register, uses it to index into a global table of routine pointers and calls the selected routine with the pointer as argument.
- `initialize_object_slot_callback`: Loads a value from a table indexed by a register, initializes an object slot with it, and invokes a callback if a condition is met.
- `initialize_object_slot`: The routine scans a small fixed set of slots in a global structure, finds the first free slot, writes fields from the input pointer and a parameter into that slot, and sets up a handler pointer by indexing a table with a byte from the input or by using a global variable table.
- `clear_flagged_table_entries`: The routine iterates over a small table of short entries stored in a global variable.
- `clear_flags_in_table`: The routine iterates three times over a table of entries spaced apart in memory.
- `parse_object_property_stream`: Iterates through a data stream to initialize object properties by reading type tags and associated values.
- `parse_object_property_stream_3`: Iterates through a data stream to initialize object properties by reading type tags and associated values.
- `parse_object_property_stream_2`: Iterates through a data stream to initialize object properties by reading type tags and associated values.
- `parse_object_definition_stream`: Iterates through a data stream to initialize object properties by reading type tags and associated values.
- `update_animation_state`: The routine updates a pair of 16-bit and 8-bit animation counters stored in a structure.
- `map_value_into_range`: The routine obtains a short value from another routine and maps it into the interval defined by two short fields of its parameter.
- `perform_32bit_addition`: Performs a 32-bit addition using two 16-bit values and updates two global variables with the result.
- `set_flag_on_lowercase`: The routine checks a byte at an offset from a pointer in A1.
- `process_joystick_input_state`: Reads the joystick and fire button status from hardware registers to derive a state value through bitwise operations.
- `empty_routine_2`: The routine contains no executable instructions and does not perform any operation.
- `clear_global_word`: Clears a word in a global variable.
- `wait_for_flag`: Enters a loop that continues until a specific global variable is non-zero.
- `initialize_handler_entry`: The routine writes a word from a register into an object structure, sets a small constant field, clears a 32-bit field, stores a pointer to another routine as a handler, and writes a constant value into another field of the same structure.
- `update_structure_parameters`: Clamps a value in a global structure to a minimum threshold, writes a byte to an offset, calculates a new address based on bit-shifted and scaled values, and stores a bit-shifted constant into a table entry.
- `modify_global_value`: Increments or decrements a global variable by a fixed amount based on a comparison with a constant value.
- `compare_memory_blocks`: Iterates through two memory locations in steps of 24 bytes, checking if a bitwise XOR of the values at those locations matches a specific bitmask.
- `process_object_collision_and_state`: Iterates through a list of objects to check for bitwise collision patterns and updates their state or pointers accordingly.
- `return_empty_subroutine`: This routine is empty and performs no operations before returning to its caller.
- `return_immediately`: This routine performs no operations and returns.
- `return_on_carry`: The routine returns if the carry flag is set.
- `return_from_subroutine_2`: The routine is empty and performs no operations before returning.
- `mask_rotated_word`: Performs a bitwise AND between a word retrieved from a memory offset and a version of a register that has been rotated.
- `mask_memory_word`: Performs a bitwise AND between a rotated 16-bit value and a word retrieved from a memory offset.
- `mask_memory_value`: Performs a bitwise AND between a 16-bit register and a 16-bit value loaded from memory.
- `mask_word_value`: Performs a bitwise AND between a 16-bit register and a word located at a negative offset from a pointer.
- `mask_rotated_pointer_word`: Performs a bitwise AND between a rotated 16-bit value and a word from a table based on a condition.
- `mask_pointer_offset_word`: Performs a bitwise AND between a 16-bit register and a word located at a positive offset from a pointer.
- `mask_conditional_word`: Performs a bitwise AND between a word from a pointer and either a doubled register value or a single bit based on the carry flag.
- `mask_pointer_value`: Performs a bitwise AND between a 16-bit register and a 16-bit value loaded from an offset of a pointer.
- `increment_and_set_bits`: Increments a global variable and uses its shifted value to index a memory location, then performs a bitwise OR loop to set bits in a table.
- `store_calculated_offset`: Calculates a value based on the difference between a register and a global variable, then stores this value into two different memory locations using incremented pointers.
- `decrement_and_clear_bits`: Decrements a global variable and uses its shifted value to index a memory location, then performs a bitwise loop to clear specific bits in a table.
- `update_table_entries`: Reads a short value from a pointer to update a table and computes several short offsets.
- `return_combined_registers`: The routine calls another routine twice and then returns a 64-bit value formed by concatenating the contents of two registers.
- `call_other_routine_while_flag`: The routine loads a short value from a global variable and repeatedly calls another routine while the value is non-zero, updating the loop variable from the callee’s D1 output each iteration.
- `search_bit_pattern_table`: Iterates through a structured table to locate a specific bit pattern by performing bitwise shifts and comparisons.
- `process_coordinate_differences`: Calculates differences between values in a table and executes a series of routines based on whether those differences are zero, positive, or negative.
- `apply_bitmask_loop`: Performs a bitwise OR and a conditional bitwise XOR on a series of words in a table using a shifting mask.
- `traverse_bit_pattern_table`: Iterates through a table using bitwise shifts and comparisons to navigate a structured bit pattern.
- `apply_masked_bits`: Applies a word to specific offsets in a table using bitwise OR and conditional bitwise XOR operations based on a bitmask.
- `compute_bit_from_byte_pair`: The routine takes a pointer and a 16-bit value, reads a 16-bit value from the pointer, adds the 16-bit input, masks the low four bits of the bitwise NOT of the sum, and returns one bit shifted into position determined by that nibble.
- `process_collision_bounds`: Calculates intersection ranges between two sets of coordinate data and updates a structure with resulting bounds, offsets, and status flags.
- `call_function_table_loop`: The routine repeatedly calls a function pointer taken from a table in A1, decrementing a counter in D3 until it reaches -1, then returns the value originally on the stack.
- `call_function_table_loop_2`: The routine repeatedly calls a function taken from a table via A1, decrementing a counter in D3 until it reaches -1, then returns the value originally passed on the stack.
- `call_function_table_loop_3`: The routine repeatedly calls a function pointer taken from a table and decrements a counter until the counter reaches -1.
- `update_short_table_entries`: The routine reads a short value from the input pointer and passes it to another routine, then computes and stores several short offsets derived from the input table into an output area, copying a few input shorts to the output and invoking another routine with computed indices.
- `invoke_callback_and_return_pair`: The routine invokes another routine selected from a table using an index derived from a short field in a structure pointed to by a global variable and an input value, then returns a 32-bit value formed by concatenating two input registers.
- `calculate_and_store_table_data`: Calculates several values based on pointer arithmetic and table lookups, then stores them into a memory block before entering a loop that updates a structure field based on a decrementing counter.
- `mask_memory_block`: Iterates through a block of memory, performing bitwise OR on the first byte and bitwise AND on subsequent bytes at fixed intervals.
- `wait_for_blitter_2`: Requests access to the blitter and waits for a specific bit in a status register to clear.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `call_disown_blitter`: Library stub: loads its arguments and the graphics library base, calls DisownBlitter and returns its result to the caller.
- `setup_blitter_parameters`: Writes several constant and register-derived values into a memory block and requests blitter ownership.
- `initialize_blitter_structure`: Calculates a value derived from a table and pointer offsets, then writes this and other values into a memory block before requesting blitter ownership.
- `set_object_state_from_input`: The routine reads two shorts from a structure pointed to by its argument, calls another routine and uses the result together with a value from a register to conditionally load words from a table into fields of a structure pointed to by a register, then writes several fields of that structure with computed positions and flags derived from the call result, and finally calls another routine again.
- `calculate_object_properties`: Computes several values including a position and a coordinate, then stores them into a table structure.
- `set_sprite_position_table`: The routine reads a pointer from the argument block, uses values from that pointer and from the argument block to compute a position, writes several fields into a global structure, iterates over a bitmask returned by another routine to set a flag and update the position, and waits for a flag in a global variable before repeating.
- `combine_table_entries_into_24_bit_value`: The routine reads a short from a pointer, adds an input short, shifts the result right by three, and combines it with the product of a second summed short and a short fetched via a pointer obtained from a table entry.
- `decode_cnp_picture`: Decodes a .cnp picture: reads width, height and plane count and the palette, then unpacks each plane selected by the caller mask from bytes with run-length escapes (repeat the previous byte or copy from the line above).
- `dispatch_string_characters`: Iterates through a null-terminated string and executes another routine for every character found in the sequence.
- `iterate_string_and_dispatch`: Iterates through a null-terminated string and calls another routine for each character.
- `shift_and_dispatch_loop`: The routine repeatedly shifts an unsigned value right by four bits, then calls another routine through an indirect table.
- `execute_repeated_routine`: Executes another routine a specific number of times based on a value in a register.
- `dispatch_via_table_2`: Executes a routine selected from a table using an index provided in a register.
- `copy_and_set_bytes`: Copies a byte from a source to a destination and clears specific word offsets within each copied unit.
- `copy_and_mask_block`: Copies a block of memory from a source to a destination while clearing specific word offsets within each copied unit.
- `calculate_clipping_rectangles`: Computes the intersection between a source bounding box and a destination area to determine the valid sub-region for a bitplane transfer.
- `calculate_clipping_offsets`: Computes the intersection between a source rectangle and a destination area to determine the valid sub-region for a bitplane transfer.
- `blit_clipping_rectangle`: Calculates the intersection between a source rectangle and a destination area to determine the valid sub-region for a bitplane transfer.
- `initialize_sprite_structure`: The routine prepares a sprite structure in a global variable by storing pointers and computed offsets derived from a table entry, initializing several fields to constant values and a combined value, then overwriting some of those fields with return values from another routine before returning.
- `update_global_pointers`: Increments two global pointers and stores a 16-bit value into the memory locations they previously pointed to.
- `perform_blitter_operations`: Initializes a blitter structure, waits for the hardware to become available, and performs memory clearing and data transfers using a loop.
- `tiny_ori_cmp`: Small routine of 2 instructions that merges 195 into d0, then compares d4 with -(a2).
- `clear_memory_block`: Clears a block of memory by writing zero to a series of longwords.
- `load_round_bob_set`: Loads the bob set of the round in d0 (Sprite A plus the round): clears the bob table, then fetches and unpacks the set so the round enemies can be drawn.
- `decrement_register_and_update_memory`: Decrements a register, updates a byte in memory using a pointer and a register, and pushes a word from a global variable onto the stack before clearing a register.
- `check_bit_flags`: Iterates through a table to check if specific bits are set within a global data structure, provided certain conditions in a global variable are met.
- `test_bit_patterns`: Performs bitwise operations and shifts on a value to check for specific bit patterns across multiple memory locations.
- `process_handler_table`: The routine checks a global counter and, if a handler pointer is present, calls another routine and updates an entry in a table pointed to by a global variable, then iterates over the table calling the handler for each non-empty entry.
- `zero_table_and_set_flag`: Clears eight consecutive longwords in a memory table and sets a global variable to a specific value.
- `clear_jump_table`: The routine clears a small jump table by writing zeros to eight consecutive 32-bit slots starting at the address of another routine, then sets a global variable to -1.
- `clear_active_zone_flag`: The routine clears a flag byte in a table of zone entries and records the flag value in a global variable.
- `update_moving_objects`: Iterates through a list of objects to update their positions based on movement type, including straight-line increments and path-following logic.
- `search_and_insert_coordinate_pair`: Searches a sorted table for a specific coordinate pair and, if not found and within bounds, inserts the new pair into the table while shifting subsequent entries.
- `clear_global_table`: The routine clears a block of memory by writing the same undefined value repeatedly.
- `tiny_ext`: Small routine of 1 instruction that sign-extends d0.
- `update_bcd_counter`: Adds a 32-bit value to a multi-byte BCD counter stored in a global variable and handles overflow logic.
- `process_recursive_calls`: Calls itself recursively based on the value of a global variable before clearing that variable.
- `manage_linked_list_node`: Decrements a value at an offset from a pointer and performs a search through a table to update a global pointer.
- `iterate_data_table`: Iterates through a table of data, performing a series of operations and updating a global variable for each entry.
- `set_dispatch_parameters`: Checks a global variable and a memory value to determine a byte value, which is then stored in a memory location.
- `copy_data_blocks`: Iterates through a loop to either copy blocks of data from one memory location to another or zero out memory locations based on a comparison between a counter and a value in a table.
- `calculate_scaled_value`: Calculates a value based on a global variable, applies a limit, and stores the result as ASCII characters in a table.
- `dispatch_via_table_3`: Performs a series of operations including bit manipulation, scaled value calculation, and data block copying before returning a concatenated result.
- `copy_data_block`: Copies 16-bit words from a source table to multiple offsets within a destination structure in a loop.
- `update_text_buffer_and_handle_input`: Manages a text string buffer by shifting characters and handling input increments or decrements based on specific status flags.
- `execute_structured_loop`: Repeatedly calls another routine multiple times within a loop that processes a table of values.
- `clear_data_register`: Clears the low byte of a data register.
- `clear_data_register_alt`: Sets the low byte of a data register to zero.
- `execute_indirect_calls_extended`: Calls two functions via pointers stored in a global variable and enters a loop that modifies a global variable until a specific bit is cleared.
- `infinite_loop_routine`: The routine checks a global variable for a zero byte.
- `update_split_value`: Performs a 32-bit addition using a global variable and a constant, then splits the result into two separate 16-bit storage locations.
- `decrement_bcd_two_digits`: The routine subtracts a byte value taken from a register, with borrow handling, from a two-digit packed BCD number stored in two consecutive bytes.
- `copy_data_block_2`: Calculates a source address based on a global variable and an offset, then copies a block of data to a destination address through a loop.
- `process_state_machine`: Manages a complex state machine using a counter and an indirect jump table to execute different routines based on a value.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d2, then merges 0 into d0.
- `increment_global_variable`: Increments a global variable and wraps its value if it exceeds a specific threshold.
- `process_state_logic`: Resets a global variable and enters a loop that updates a counter and performs indirect calls based on a table index.
- `update_state_machine`: Modifies a global counter within specific bounds and uses a value to dispatch calls through an indirect jump table.
- `execute_state_machine_loop`: Iterates through a sequence of indices to perform indirect calls from jump tables.
- `modify_address_via_bitmask_byte`: Performs a bitwise AND between a global variable and a byte from a register, then adds the result to that global variable.
- `modify_address_via_bitmask_word`: Performs a bitwise AND between a global variable and a word from a register, then adds the result to that global variable.
- `update_state_machine_counter`: Increments or decrements a global counter within a fixed range and uses the resulting value to index into a jump table for indirect function calls.
- `modify_address_via_bitmask`: Performs a bitwise AND between a global variable and a value from a register, then adds the result to that global variable.
- `update_state_machine_via_jump_table`: Updates a state machine by iterating through a sequence of indices and executing indirect calls from jump tables.
- `update_state_machine_logic`: Updates a state machine by cycling through a sequence of indices and executing indirect calls from jump tables based on a counter.
- `run_repeated_sequence`: Executes a specific routine four times in succession.
- `parse_text_stream`: Iterates through a character stream to identify line endings and specific control characters while managing a loop counter.
- `copy_null_terminated_string`: Copies a sequence of bytes from a source address to a destination address until a null byte is encountered.
- `load_compressed_asset_data`: Decodes an LZHUF compressed stream into a destination buffer and manages the resulting linked object data.
- `reset_game_state_flags`: The routine checks a global flag and, if set, calls another routine and then clears two global variables.
- `initialize_global_structure`: Writes several constant values to a global structure and updates a character-based global variable using values from that structure.
- `life_lost`: Runs when the frame loop ends with a life lost: clears the cut, plays the loss sequence and takes a life; the caller goes to game over when none are left, otherwise restarts the round loop.
- `execute_recursive_calls`: This routine performs a series of six consecutive calls to itself before returning.
- `call_three_other_routines`: The routine calls another routine three times in sequence and then returns.
- `dispatch_indirect_call`: Sets a global byte value and a global integer to zero before performing an indirect call using an offset from a register.
- `modify_and_dispatch`: Performs bitwise operations on global variables and executes a routine via an indirect call using a table offset.
- `execute_indirect_call_3`: Performs an indirect call using a value retrieved from a table and an offset.
- `round_clear`: Runs when the frame loop ends with the round cleared: shows the full picture and the round clear text and adds the round bonus before the next round is set up.
- `reset_global_counters`: Sets a global byte to zero and enters a loop that increments a global variable until it reaches one hundred before setting another global to two.
- `reset_game_state`: The routine clears a global variable and repeatedly calls another routine until the variable reaches a threshold, then sets a different global variable to a constant and returns.
- `tiny_move`: Small routine of 1 instruction that copies 99 to d0.
- `process_key_input`: Checks a global variable against specific values to return a result, then updates that variable to a specific constant and clears another.
- `process_input_loop`: Executes a loop that checks for specific key codes and handles escape or pause logic based on the returned value.
- `check_input_condition`: Iterates through a loop checking a condition derived from a routine call, then processes specific key codes to return a value.
- `check_escape_and_pause_keys`: Reads the last key code once a frame and acts on Escape (back to the title) and P (pause); returns non-zero to keep the frame loop going.
- `decrement_timer_tables`: The routine decrements a set of 32-bit timer counters stored in three consecutive tables in chip RAM, each table containing 32 entries.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `clear_d0_register`: Clears the d0 register by performing a logical or with zero.
- `clear_data_register_triple`: Performs a sequence of clearing the low byte of a data register.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `iterate_six_slot_table`: Walks a fixed table of six 6-byte entries starting at a constant base address, testing the first 16-bit word of each entry and recursively invoking the routine when that word is nonzero.
- `clear_data_register_byte`: clears the lower byte of a data register to zero.
- `tiny_ori_ori_ori_2`: Small routine of 3 instructions that merges 0 into d1, then merges 0 into d0, then merges 0 into d0.
- `mask_global_word_table`: Iterates through a global table of words, skipping null entries, and applies a masking operation from a lookup table to each valid entry.
- `decrement_and_call_handler`: The routine checks a global variable and, if non-zero, calls another routine and stores its result.
- `decrement_and_call`: The routine reads a 16-bit value from a pointer, decrements it, writes it back, and if the result is zero it clears a global variable when the pointer equals the routine’s own address and then calls another routine.
- `handle_object_state`: Manages complex object state by updating BCD counters, searching flag tables, and performing bitplane transfers based on calculated offsets.
- `update_powerup_boxes`: Frame-loop logic step for the power-up boxes hidden in the field: updates the box table and its timers and hands out the power-up when the player claims a box.
- `search_flag_table`: Iterates through a table to check if a specific bit is set within a value, and if found, clears a byte at a specific offset.
- `iterate_flag_table`: The routine walks a table of entries and for each entry whose first flag byte is set it calls another routine, possibly twice, and updates the entry’s flag byte and a counter.
- `set_flag_bits`: Iterates through a table and performs a bitwise OR operation to set a specific bit in each entry using a calculated mask.
- `self_recursive_call`: The routine calls another routine twice and then returns.
- `scan_table_and_call_routine`: The routine walks a table of entries spaced at fixed intervals, checking a flag byte in each entry and a condition involving a global variable.
- `clear_flag_bits`: Iterates through a table and performs a bitwise AND operation to clear a specific bit in each entry using a calculated mask.
- `apply_bitmask_to_table`: Iterates through a table of 32-bit values, applying a bitwise mask to each entry based on a value from a pointer.
- `store_calculated_values`: Calculates a value based on a global variable and a constant, then stores it into two different memory locations while incrementing pointers.
- `self_link_table_entries`: The routine writes the address of a global variable into the word located four bytes after each element of a table that starts at the global variable.
- `clear_object_flags`: The routine sets a flag in a global variable and then iterates over a table of objects, calling another routine twice for each entry that matches certain byte values, otherwise calling it twice and clearing a byte in the entry.
- `update_flag_table`: Iterates through a table and writes a bitmask derived from a specific bit position into a secondary table at a corresponding offset.
- `link_object_data`: Iterates through a table to copy data and link pointers into a destination structure based on values found in a global table.
- `build_bob_set`: Iterates through a list of metrics to expand and copy data blocks, likely constructing a set of animation frames.
- `expand_and_copy_data_block`: Expands a data block through recursive scaling and performs a block memory copy based on bitmask flags.
- `build_bob_frame`: Builds one bob frame from the set: copies its four interleaved bitplane rows, applies the mirror and flip flags and makes the mask from colour 15 when asked.
- `compute_bob_mask_plane`: Reads a header of width and height from the input block, stores a pointer to the output data region, then for each row combines four bitplane words with a bitwise AND and writes the inverted result as a single-plane mask word.
- `compute_bitmask_convolution`: Performs a bitwise logical operation across a 2D grid of data by checking four adjacent memory locations for each target pixel.
- `invert_bitmap_rows`: The routine reads a short from a pointer and uses it to compute a destination pointer and a row stride.
- `compute_tile_metrics`: The routine walks a short array from a start pointer up to a limit pointer, reading a pair of dimensions and a flags word for each entry.
- `rotate_bitmap_block`: The routine copies a rectangular block of short words from a source buffer to a destination buffer and then rotates each 16-bit word left by one bit while applying a mask.
- `unpack_bitplane_words_to_block`: Reads 16-bit words from a source buffer, shifts their bits out MSB-first and reassembles them into 16-bit words written backwards into a destination buffer, looping over a rectangular block of rows and columns and advancing both pointers by a computed row stride.
- `rotate_bitmap_block_bits`: Takes a source bitplane word stream and writes it into a destination block one bit column at a time: for each of four passes it shifts 16 source bits per word into a 16-bit accumulator and stores each completed accumulator to the destination, stepping through the rows of a rectangle of the given width and height.
- `transform_bitstream_to_planar_buffer`: Performs a bitwise transformation on a source data stream by shifting and rotating bits into a new format.
- `convert_bitstream_to_word_array`: Reads a stream of bits from a source memory area and performs a bit-reversal operation while packing the bits into 16-bit words.
- `rotate_bitmap_block_rows`: Performs a bit-serial rotation of a rectangular block of 16-bit words: for each row it shifts source bits out of the high end one at a time and packs them into destination words, writing completed 16-bit results backward through the output pointer.
- `unpack_16bit_tilemap`: The routine unpacks a packed 16-bit tile map stored in a global table into a destination buffer using a global pointer.
- `copy_and_offset_blocks`: Copies a block of data to a new location and then performs a second pass to copy data from the source to a calculated offset within the destination.
- `copy_tile_rows`: The routine copies a rectangular block of 32-bit words from a source pointer to a destination pointer using a row-by-row loop.
- `set_linked_object_data`: The routine writes fields from a source structure into a newly allocated or linked object and links it into a chain.
- `execute_linked_chain`: Traverses a linked list of routines and manages memory availability while executing indirect pointers.
- `search_string_table`: Traverses a linked list of strings to find a match for a specific character sequence and stores the resulting pointer.
- `call_indirect_pointer`: Loads a pointer from a global variable and executes the routine located at that address.
- `call_avail_mem`: Wrapper that calls the exec library function AvailMem and returns its result to the caller.
- `store_value_if_large`: The routine calls another routine via a function pointer taken from a global variable and checks the returned size.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `close_file_and_seek`: The routine obtains a file handle from a global variable, calls a helper to test it, stores the result, and if the handle is valid it invokes another routine, closes the file via the DOS library, seeks the file via the DOS library, and returns the result of the close operation; otherwise it returns an error code.
- `close_file_if_open`: The routine calls a function through a table stored in a global variable and stores the result in another global variable.
- `swap_table_entries`: The routine reads a short value from another routine, uses it as an index into a table to fetch two bytes from a global table and writes them to two RAM locations, then swaps a byte from RAM with a byte from the table and clears a RAM byte.
- `reset_audio_and_dma`: The routine writes to a global variable six times and then returns.
- `update_audio_parameters`: Updates several global memory locations with values from a table and manages a counter through various conditional checks.
- `update_audio_playback_parameters`: Processes an input structure to calculate audio sample offsets and volume levels from a lookup table.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to the low twelve bits, stores the result into an offset of the same structure and clears a byte at another offset.
- `update_input_state_structure`: Processes input data from a structure by checking a status nibble to perform various arithmetic or lookup operations.
- `gamepad_button_handler`: The routine reads the joystick fire buttons and disk status from the CIA port, then loops over a table of bytes, comparing each byte to the previous one.
- `execute_arithmetic_loop`: Performs a series of calculations involving bitwise masking, modulo operations, and coordinate updates before entering an infinite loop.
- `update_global_indices`: Calculates two new values based on a short integer using modulo and division operations, then stores them in two global variables.
- `perform_arithmetic_loop`: Executes a loop that modifies a global variable through repeated addition and bitwise operations.
- `process_recursive_object_updates`: A recursive routine that iterates through a nested structure of objects, updating their parameters and managing state changes.
- `lzhuf_decode`: Decodes an LZHUF stream (Okumura coder with a 4096-byte ring buffer and adaptive Huffman codes) into the output buffer; used for the bob sets.
- `initialize_lookup_tables`: Populates two tables by performing a series of incremental writes and cross-referencing indices.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `lzhuf_init_tree`: Initialises the adaptive Huffman tree of the LZHUF decoder: the leaf frequencies, the parent and child links and the root.
- `lzhuf_update_tree`: Updates the adaptive Huffman tree of the LZHUF decoder after a symbol: increments its frequency, re-sorts the nodes and rebuilds the tree when the root frequency reaches its limit.
- `calculate_sprite_tile_offsets`: Iterates through a data structure to calculate screen coordinates and tile offsets for sprite rendering.
- `initialize_data_structure`: Clears a range of memory and writes several constant word values to various offsets within a structure.
- `clear_object_table_entries`: The routine clears a table of object entries by copying two words from a source pointer into each entry and zeroing two other words.
- `decrement_pointer_and_copy_word`: Decrements a value pointed to by a register and copies a word from one memory location to another.
- `copy_word_from_pointer_to_global`: The routine copies a 16-bit word from an address pointed to by a pointer in a register to a global variable.
- `clear_three_globals`: The routine calls another routine twice, clears three word-sized global variables, then calls another routine twice.
- `update_object_timer_and_state`: Decrements a counter in a structure and updates various state flags and counters based on the remaining value.
- `decrement_timer_and_update_counter`: Decrements a word value at a specific offset and conditionally increments or decrements another value based on several checks.
- `update_packed_data_and_flags`: Updates a series of memory locations using values from a packed data structure and performs conditional bitwise inversions and arithmetic adjustments on global state variables.
- `decrement_timer_and_advance_index`: The routine decrements a 16-bit counter in the object, resets it to a constant when it reaches zero, loads a value from a global table using an index from the object, writes that value into the object and advances the index.
- `set_status_from_table`: The routine reads a short from an input structure, indexes into a global table and a second table, compares the input short with the two table entries and writes constant values to two words in an output structure.
- `process_object_table`: The routine iterates over a table of objects and updates their fields.
- `update_object_attributes`: Updates a structure by adding values from a data table to specific offsets and increments a 16-bit index with wrap-around logic.
- `calculate_velocity_difference`: Computes the difference between source and target coordinates, stores the result in a destination buffer, and increments a counter if the difference is non-zero.
- `increment_ushort_with_wrap`: The routine reads a 16-bit value from an offset in a structure pointed to by a register, adds one, wraps it to zero when it exceeds the maximum signed 16-bit positive range, and writes the result back to the same location.
- `decrement_and_update_counters`: The routine decrements a counter in a global structure and, when it reaches zero, copies a value from another field, updates a masked position field in a second structure, possibly reverses a direction field, and then decrements a second counter, resetting it and adjusting related fields when it reaches zero.
- `round1_enemy_logic`: Round 1 enemy logic (entry 0 of the handler table at +E91A): runs the boss state machine (continuation pointer at object +44 with its delay at +48) unless frozen, then the three guard objects through their state table at +9A0E, then the respawn of guards when the count says so.
- `initialize_object_table`: The routine initializes a table of objects by writing constant values into a series of entries, then marks a global flag.
- `execute_indirect_call`: Performs an indirect function call using an address retrieved from a table based on an offset stored in a structure.
- `decrement_counter_and_reset_state`: The routine decrements a short counter at offset ten from a pointer in a4 and, if the counter reaches zero, it resets the counter to one, calls another routine, copies a word from a table pointed to by a5 into the structure at a4, and writes constant words to several fields of the structure at a4.
- `update_entity_state_a`: Checks a global flag to conditionally decrement counters and update positions, clears collision flags, checks bounds, and dispatches to a handler table if conditions are met.
- `decrement_counter_and_toggle_flag`: The routine decrements a short counter stored at an offset from a pointer in A4 and writes the new value back.
- `initialize_object_fields`: The routine writes constant values into several word fields of a structure pointed to by a register.
- `update_object_state`: Checks a flag and a global variable, then performs conditional updates to structure members including sign inversion and value copying.
- `process_complex_logic`: A long routine that manipulates various global variables and performs conditional loops and arithmetic on structure members.
- `set_two_word_values`: The routine writes the value four to two consecutive word locations in a global variable and then calls another routine.
- `check_two_word_flags`: The routine calls another routine and then returns a flag.
- `decrement_counter_reset_animation`: The routine decrements a 16-bit counter stored at an offset within a structure pointed to by a register.
- `set_status_from_table_2`: The routine reads a two-bit field from an object structure, uses it as an index into a global variable table and a second table, compares a short field of the object with the looked-up values and writes a short status word in the object.
- `decrement_timer_and_update_state`: The routine decrements a short timer in a structure and, when it reaches zero, resets the timer and calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_and_call`: Fills a block of memory with zeros using a loop and then executes an indirect function call via a table.
- `execute_indirect_call_2`: Performs an indirect function call using an address calculated from a global table and a structure offset.
- `execute_callback_and_format_string`: Calls a function pointer from a table and, if certain conditions are met, converts a value to a character and writes two words to a memory location.
- `initialize_memory_block`: The routine calls another routine via a table and then writes two 16-bit values to memory locations offset from an address in A4, setting one word to a constant and another word to a constant.
- `update_collision_bounding_box`: Calculates intersection or overlap between a moving entity and a target object by performing coordinate shifts and bitwise masking.
- `accumulate_flag_masks`: The routine iterates over seven entries in a table, and for each entry whose flag is set it copies two words from the entry into two global variables, calls another routine, and ORs the returned mask into an accumulator global variable.
- `read_two_word_table`: The routine calls another routine and then checks a global variable.
- `update_entity_collision_bounds`: Adjusts coordinate offsets for an entity and toggles bitwise flags within a state structure.
- `update_game_state_flags`: The routine clears and sets fields in a game state structure, copies values from a table to the structure, and conditionally updates four short fields in a substructure and manipulates flag words.
- `reset_zone_state_2`: Clears several fields in a structure, copies two words from another structure into it, and calls another routine.
- `reset_zone_state_variant`: Initializes a structure by clearing fields and copying data from another structure, with conditional logic based on a global variable.
- `reset_zone_state`: The routine clears several fields in a structure pointed to by a register, copies two words from another structure into it, and invokes another routine twice.
- `test_collision_segments`: Iterates over two sequences of segment pointers obtained from another routine, calling a collision test routine for each segment and checking a mask in a register against the segment value.
- `set_collision_thresholds`: The routine writes a constant to a global variable, then reads two pairs of values from a short table pointed to by a register, computes the absolute difference of each pair, and if the difference exceeds a small threshold it writes a different constant to two separate global variables.
- `merge_flag_bits_into_word`: The routine reads a flag field from a structure pointed to by a register, extracts two bits, optionally adds an offset if a short field is non-zero, uses the result as an index into a global table of words, and ORs the fetched word with another word in the structure, storing the result back into that structure field.
- `combine_two_short_values`: The routine reads a 3-bit field from a global variable, uses it as an index into a table of offsets stored in a global variable, adds the offsets to two base pointers and loads two 16-bit values from the resulting addresses.
- `check_flag_and_exit`: The routine reads a short from a global variable and returns if it is non-zero.
- `check_flag_in_table`: The routine iterates over a table of words using a pointer in A0 and a function pointer in A6.
- `check_collision_and_store`: The routine iterates over a table of words using a function pointer, testing each word against a mask in a register.
- `mask_bits_low`: Returns the bitwise NOT of a word masked to the lower four bits.
- `update_bounding_box`: Calculates and stores updated boundary coordinates in a structure based on input values and existing offsets.
- `shift_short_from_table`: The routine reads a short value from a global variable via an address derived from an uninitialized A5 register and returns the value shifted left by three bits.
- `return_early`: The routine consists of ten instructions and performs no observable work; it reaches a return instruction and exits immediately.
- `extract_and_combine_table_entry`: The routine reads a 32-bit value from a table using the low three bits of the input word as an index, optionally byte-swaps it if a sign flag from another table entry is negative, then combines that value with a shifted signed value from the same second table entry to form a 64-bit result.
- `empty_routine_3`: The routine contains twelve instructions and performs no observable work; it ends with a return.
- `accumulate_short_field`: The routine reads a short from an object field, calls another routine twice, then adds the read value to another short field of the same object and stores the sum back.
- `rotate_left_and_shift`: The routine rotates a 16-bit value left by a variable amount masked to the low six bits and then left-shifts the result repeatedly until a signed counter reaches -1.
- `decrement_counter_and_update_table_entry`: The routine receives a pointer to a small table in memory and decrements a counter stored at offset five.
- `update_entity_state_flags`: The routine updates a 16-bit field at offset six of a structure pointed to by an address register using a 16-bit value in D0.
- `clamp_value_high`: Compares two words in a structure and sets the first to the value of the second if the first is smaller.
- `set_flag_based_on_offset`: The routine writes a constant to a word in a global variable and then reads a short from a global variable and subtracts a short from another global variable.
- `test_collision_table`: Iterates through a table of values to check against specific thresholds and updates a structure if conditions are met.
- `increment_counter_and_adjust_value`: The routine increments a counter stored in a table pointed to by a register and, if a flag bit in a global variable is set, subtracts a constant from another value in the same table entry.
- `copy_word_from_table`: The routine copies a 16-bit word from a table indexed by a short value in a structure to a field in another structure.
- `set_constant_byte`: Stores the constant value four into a specific offset of a structure.
- `select_quadrant_index`: Calculates the absolute difference between two coordinate pairs to determine an index into a global table.
- `copy_state_flags_and_mask`: The routine reads a word from a global variable via an address in A4, tests bits in that word and conditionally copies a word from an offset in the same structure to the base and another word to an offset.
- `clear_collision_flag`: The routine clears a two-byte field in a structure pointed to by a pointer, then if a flag byte in another structure is zero it computes the absolute difference of two short values from two structures, and if both differences are less than a small threshold it writes a sentinel value to the same two-byte field and sets a global variable to a constant.
- `copy_word_block`: The routine copies a block of 16-bit words from a source pointer to a destination pointer.
- `copy_stack_data`: Copies a sequence of words from the stack to a destination pointer using a loop.
- `check_empty_routine`: The routine consists of five instructions and performs no operation.
- `process_collision_table`: The routine iterates over a table of words pointed to by a global pointer, testing each entry against a mask in a register.
- `clear_sprite_attribute_table`: The routine clears a block of sixteen 16-bit entries in a structure pointed to by a4, then writes constant values into four of those entries.
- `clear_sprite_attributes`: Clears a block of sixteen 16-bit entries in a structure and then copies specific values from a source into various memory locations.
- `reset_entity_table_entries`: The routine iterates over a table pointed to by a global variable and, for each entry, calls another routine and then copies several bytes from the routine’s outputs into fixed offsets of the current entry, clearing one word in the process.
- `check_range_bounds`: Compares two values from a table against specific limits and returns a status code.
- `decrement_timer_and_update_position`: The routine decrements a counter at offset 5 in a structure pointed to by A4 and, when it reaches zero, saves the value at offset 4, calls another routine, recombines a value from D2 with the saved value into a 32-bit word at offset 5, masks a byte at offset 3, calls another routine and adds its 16-bit result to the words at offsets 1 and 0 with 9-bit and 10-bit wrapping.
- `clear_flags_and_test`: The routine clears a word in a global variable and a word in another global variable, then calls another routine.
- `set_flag_based_on_range`: Writes zero to a memory location and then updates it to all ones if two values in a table fall outside a specific range.
- `tiny_adda_rts`: Small routine of 2 instructions that adds 36 to a0, then returns.
- `tiny_adda_rts_2`: Small routine of 2 instructions that adds 72 to a0, then returns.
- `tiny_adda_rts_3`: Small routine of 2 instructions that adds 108 to a0, then returns.
- `tiny_suba_rts`: Small routine of 2 instructions that subtracts 36 from a0, then returns.
- `tiny_suba_rts_2`: Small routine of 2 instructions that subtracts 72 from a0, then returns.
- `tiny_suba_rts_3`: Small routine of 2 instructions that subtracts 108 from a0, then returns.
- `empty_routine_4`: The routine contains five instructions and performs no observable work; it ends with a return.
- `cursor_step_two_bits_left`: Line cursor helper: steps the two-bit pixel mask left by one pixel, moving the screen pointer back one word when the mask is already in the top two bits.
- `cursor_step_three_bits_left`: Line cursor helper for 120-byte rows: steps the three-bit pixel mask left by one pixel, moving the screen pointer back one word when the mask is already near the top of the word.
- `empty_routine_5`: The routine is empty.
- `empty_routine_6`: The routine is empty and performs no operation.
- `cursor_step_three_bits_right`: Line cursor helper for 120-byte rows: moves the screen pointer forward one word when the pixel mask is in the lowest three bits, then rotates the mask by three bits.
