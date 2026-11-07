# Game 1721 (volfied (1991)(empire)[cr flt][a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01721). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (939664 top-level bytes in 55 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 9 raw-word lines |
| Functions named and described | pass, 371 of 371 named (0 author symbols), 371 of 371 described (23 by rule, 210 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 6650 of 6651 (99.98%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9259 of 9405 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 463292 |
| code | 32036 |
| data | 394125 |
| padding | 10275 |
| unused | 39936 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Volfied (1991)(Empire)[cr FLT][a2].adf:Exit | 2 | 117 | 2 |
| Volfied (1991)(Empire)[cr FLT][a2].adf:ClickMe | 1 | 47 | 1 |
| Volfied (1991)(Empire)[cr FLT][a2].adf:Volfied | 2 | 233 | 8 |
| Volfied (1991)(Empire)[cr FLT][a2].adf:Boot1 | 1 | 45 | 2 |
| Volfied (1991)(Empire)[cr FLT][a2].adf:Boot2 | 1 | 31 | 3 |
| Volfied (1991)(Empire)[cr FLT][a2].adf:Fairlight | 1 | 155 | 3 |
| Volfied (1991)(Empire)[cr FLT][a2].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 126 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 264 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6837 | 233 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 651 | 32 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 876 | 61 |

## Cases handled

- volfied (1991)(empire)[cr flt][a2]/Volfied (1991)(Empire)[cr FLT][a2].adf:Data/SpriteA.dat: high-entropy data file (7.91 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a2]/Volfied (1991)(Empire)[cr FLT][a2].adf:Data/SpriteL.dat: high-entropy data file (7.92 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a2]/Volfied (1991)(Empire)[cr FLT][a2].adf:Data/SpriteJ.dat: high-entropy data file (7.96 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a2]/Volfied (1991)(Empire)[cr FLT][a2].adf:Data/SpriteH.dat: high-entropy data file (7.96 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a2]/Volfied (1991)(Empire)[cr FLT][a2].adf:Data/SpriteD.dat: high-entropy data file (7.95 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a2]/Volfied (1991)(Empire)[cr FLT][a2].adf:Data/SpriteO.dat: high-entropy data file (7.95 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a2]/Volfied (1991)(Empire)[cr FLT][a2].adf:Data/SpriteB.dat: high-entropy data file (7.92 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a2]/Volfied (1991)(Empire)[cr FLT][a2].adf:Data/SpriteM.dat: high-entropy data file (7.90 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a2]/Volfied (1991)(Empire)[cr FLT][a2].adf:Data/Sprite@.dat: high-entropy data file (7.91 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a2]/Volfied (1991)(Empire)[cr FLT][a2].adf:Data/SpriteK.dat: high-entropy data file (7.93 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a2]/Volfied (1991)(Empire)[cr FLT][a2].adf:Data/SpriteI.dat: high-entropy data file (7.97 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a2]/Volfied (1991)(Empire)[cr FLT][a2].adf:Data/SpriteG.dat: high-entropy data file (7.92 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a2]/Volfied (1991)(Empire)[cr FLT][a2].adf:Data/SpriteE.dat: high-entropy data file (7.94 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a2]/Volfied (1991)(Empire)[cr FLT][a2].adf:Data/SpriteP.dat: high-entropy data file (7.94 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a2]/Volfied (1991)(Empire)[cr FLT][a2].adf:Data/SpriteC.dat: high-entropy data file (7.96 bits/byte), classified as packed data
- volfied (1991)(empire)[cr flt][a2]/Volfied (1991)(Empire)[cr FLT][a2].adf:Data/SpriteN.dat: high-entropy data file (7.93 bits/byte), classified as packed data
- runtime segment an address: 118 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 255 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-keys): 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-keys): 4514 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-keys): 575 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-keys): 769 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- volfied (1991)(empire)[cr flt][a2]/Volfied (1991)(Empire)[cr FLT][a2].adf:Volfied: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- volfied (1991)(empire)[cr flt][a2]/Volfied (1991)(Empire)[cr FLT][a2].adf:Fairlight: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `configure_hardware_and_copy_data` (runtime segment an address): Sets specific values in hardware registers and global variables before entering a loop that writes incrementing values to a memory range. Kind: disk hardware/track check. Pass path: the branch at an address (bne.b an address is not taken, implying z flag set).

## Functions


### Volfied (1991)(Empire)[cr FLT][a2].adf:Exit

- `allocate_exit_buffer`: A routine that walks a linked list in a global variable to find the maximum of a field, then prepares a fixed-size exit buffer on the stack with a header and an index table, and copies a small block from a global table into the buffer before invoking another routine and returning a status code.
- `copy_buffer_and_call`: Copies sixteen words from a global table to a stack pointer if the pointer is valid, then calls another routine.

### Volfied (1991)(Empire)[cr FLT][a2].adf:ClickMe

- `initialize_system_libraries`: Opens required Amiga OS libraries such as dos.library and initializes the system environment by calling internal setup routines and handling message ports.

### Volfied (1991)(Empire)[cr FLT][a2].adf:Volfied

- `load_and_relocate_hunks`: Iterates through Amiga executable hunk structures, allocating memory for data sections and copying their contents.
- `decrement_word_loop`: Decrements a 16-bit value in register D1 until it reaches zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `copy_bytes_with_filled_prefix`: The routine copies bytes from a table pointed to by a register through a pointer.
- `check_bitmask_nonzero`: The routine tests whether a 64-bit mask derived from a global variable and an index is non-zero and returns early if it is.
- `decode_bitstream_data`: Extracts a sequence of bits from a pointer to a word array into an accumulator, handling bit shifting and buffer reloading when the current word is exhausted.
- `decode_bitstream_data_2`: The routine reads a variable-length bitstream from a byte buffer using a bit offset and extracts bits one at a time, assembling them into a 32-bit accumulator.
- `store_float_and_clear_byte`: Executes a system trap and stores a floating-point value and a combined byte into memory locations pointed to by registers.

### Volfied (1991)(Empire)[cr FLT][a2].adf:Boot1

- `initialize_system_libraries_2`: Forbids multitasking and opens the dos.library to prepare the system environment.
- `execute_init_sequence`: Calls initialization routines from a global structure and loops until a status check returns non-zero.

### Volfied (1991)(Empire)[cr FLT][a2].adf:Boot2

- `initialize_system_libraries_3`: Opens the dos and intuition libraries required for the game to run.
- `other_routine`: The routine is empty.
- `check_cpu_condition_flags`: Evaluates the state of the processor condition flags to determine execution flow, likely serving as a branch target for control transfer logic.

### Volfied (1991)(Empire)[cr FLT][a2].adf:Fairlight

- `decode_and_unpack_palette`: The routine copies a block of bytes from a global variable into a RAM buffer and then repeatedly decodes a stream of bits produced by another routine.
- `pack_and_shift_value`: Reads a word from a memory offset relative to a pointer, combines it with a global value to form a new word stored via another pointer, and returns the original word shifted right by one bit with the high bit set.
- `iterate_bitwise_loop`: The routine decrements a counter in register D1 and shifts a value in register D0 right by one bit in each iteration.

### Volfied (1991)(Empire)[cr FLT][a2].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_then_trap_zero`: Calls another routine, stores a fixed constant into a global variable, then executes trap zero and returns zero.
- `wait_for_hardware_flag`: Polls a bit in a global variable until it becomes set, then executes a sequence of other routines and loops.
- `configure_hardware_and_copy_data`: Sets specific values in hardware registers and global variables before entering a loop that writes incrementing values to a memory range.
- `update_memory_values`: Writes several specific values to a memory block pointed to by a register.
- `check_and_increment_byte`: Checks a byte at an offset; if it is negative one, it increments a global variable and loops until the byte is zero.
- `check_byte_and_decrement_global`: Checks a byte at a specific offset from a pointer.
- `clear_memory_range`: Fills a range of memory starting at a specific address with zeros using a loop.
- `copy_data_from_table`: Iterates through a data stream to copy blocks of bytes from a table into a destination memory area based on specific marker bytes.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_global_and_trap`: Stores a pointer to the current routine into a global variable and executes a trap instruction.
- `process_a5_state_loops`: The routine initializes byte and word values at fixed offsets in a global structure pointed to by register A5.
- `toggle_flag_and_update_table`: Toggles a bit in a global variable and, if the result is zero, increments a counter and copies its value into a table at a fixed offset.
- `toggle_flag_and_fill_buffer`: Toggles a bit in a global variable and, if the result is zero and a counter remains positive, decrements the counter and copies a value into a buffer array four times.
- `unpack_compressed_table`: Iterates through a source table of word-sized values, extracting nibbles to populate a destination array.
- `find_max_nibble_value`: Iterates through the four nibbles of a 16-bit word in register D0, updating a global variable in D2 if a nibble is larger than the current maximum.
- `update_sprite_timers`: Decrements a global counter and, when it reaches zero, resets it and processes a table of sprite timers.
- `decrement_and_reset_table`: Decrements a counter stored at an offset from a base pointer.
- `initialize_dma_and_copper`: Configures DMA and interrupt control registers, sets up audio and CIA hardware states, and waits for vertical blank synchronization.
- `initialize_display_palette_table`: Populates a memory table with incrementing color values and offsets, likely for a display list or palette structure.
- `update_ciab_state`: Writes values to memory offsets relative to a base pointer and updates a global variable.
- `set_intreq_values`: Writes two constant byte values to a global variable.
- `set_cia_intreq_bits`: Writes specific values to a global variable, likely configuring interrupt request settings for the CIA hardware.
- `load_code_pointer`: This routine loads a program counter relative address into a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_system_trap`: Calls a function twice and then triggers a software interrupt trap.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `cleanup_system_resources`: Releases allocated memory, closes libraries, and terminates system-level tasks by traversing a series of pointers and calling associated cleanup routines.
- `process_string_and_call`: Copies a null-terminated string to a global buffer and then executes a function pointer retrieved from a table.
- `swap_word_bytes_in_table`: The routine rotates the bytes of a pointer to a global variable 16 times, writing the high word of the pointer into successive 16-bit entries of a table that starts at the global variable.
- `execute_function_table`: Iterates through a table of pointers and executes functions if certain conditions are met, finally calling another routine via an offset from a global variable.
- `populate_pointer_rotation_table`: Iterates through a data table to populate a secondary table by rotating the high and low words of a global variable.
- `swap_display_buffers`: Swaps the two display buffer pointers at the end of a frame so the frame just drawn is shown and the next one is drawn into the other buffer.
- `fill_data_table`: Stores a sequence of 16-bit values from a global variable into a table using a loop.
- `copy_word_pairs`: Copies a sequence of 32 words from a source pointer into two separate destination locations in a global variable.
- `copy_word_pairs_from_pointer`: The routine copies 16 word pairs from a pointer supplied in A0 into two overlapping locations in a global variable.
- `store_packed_data`: Calculates a packed 32-bit value from two inputs and stores it into two consecutive memory locations.
- `manage_device_nodes`: Opens input and console devices to build and manipulate message nodes in memory.
- `cleanup_resource_structures`: Checks multiple global pointers to perform cleanup operations, including calling functions via tables and setting bytes to a constant value.
- `process_input_event_data`: Checks a byte value at an offset from a pointer to determine an action.
- `tiny_move_rts`: Small routine of 2 instructions that copies a0 to d0, then returns.
- `process_input_data`: Checks a byte value and extracts data from a pointer to update global variables.
- `process_input_key_codes`: Compares a value in a register against specific character constants to determine control flow.
- `set_view_and_return`: The routine writes a constant to a global variable, then calls another routine via a table of function pointers, and returns the value received in register A0.
- `clear_table_entry`: The routine clears a global word to zero and stores a pointer to a global variable into two entries of a table pointed to by a global variable.
- `create_message_node`: The routine builds a message node in memory.
- `cleanup_message_struct`: The routine clears two bytes in a structure pointed to by a pointer argument and then invokes two routines via a global variable table, and if a field in the structure is non-zero it also invokes another routine from the same table.
- `set_handler_data`: The routine checks a pointer argument and, if non-zero, calls a function obtained from a global variable table.
- `reset_object_flags`: The routine clears a byte at an offset in a structure pointed to by a parameter and sets two 32-bit fields to all ones, then calls another routine via an indirect pointer from a global variable and returns its result, or returns a constant if the parameter is zero.
- `read_and_close_file`: The routine opens a file, reads a byte from it, seeks within it, and closes it, returning the read byte and a flag.
- `initialize_linked_list_structure`: Populates a series of data structures by calling functions via offsets from a global pointer and linking the resulting objects.
- `initialize_message_linked_list`: Allocates memory for a series of message nodes and links them together into a list structure.
- `check_and_call_handlers`: The routine iterates over a small table of flags located relative to its own address, and for each entry it conditionally calls several handler pointers taken from a global structure and calls another routine.
- `initialize_object_data`: Fetches a pointer from a table using an index, writes two bytes to that pointer, and then calls a subroutine through a pointer stored at an offset within that same memory block.
- `unknown_routine_0_0_c18`: It calls another routine to initialize enemy structures, then calls another routine to initialize an object chain.
- `set_object_fields_and_trigger_callback`: The routine indexes into a table using an offset from the first argument, loads a pointer from the table, writes a constant word and a zero byte to fixed offsets in the pointed-to structure, copies a word from the first argument and a word from the second argument to other fixed offsets in the same structure, then calls a function pointer stored in the structure with a fixed negative offset applied.
- `call_table_function`: Loads a value from a table using an index and calls the subroutine located at that address.
- `dispatch_via_table`: The routine takes a short index from a register, uses it to index into a global table of routine pointers and calls the selected routine with the pointer as argument.
- `initialize_object_slot_callback`: Loads a value from a table indexed by a register, initializes an object slot with it, and invokes a callback if a condition is met.
- `initialize_object_slot`: The routine scans a small fixed set of slots in a global structure, finds the first free slot, writes fields from the input pointer and a parameter into that slot, and sets up a handler pointer by indexing a table with a byte from the input or by using a global variable table.
- `clear_flagged_table_entries`: The routine iterates over a small table of short entries stored in a global variable.
- `clear_flags_in_table`: The routine iterates three times over a table of entries spaced apart in memory.
- `parse_object_data_stream`: Iterates through a data stream to decode type-specific fields and update an object's properties.
- `decode_object_property_stream`: Iterates through a byte stream using a command-based switch to update specific fields within an object structure.
- `parse_object_data_stream_2`: Iterates through a byte stream to decode type-specific fields and update an object's properties.
- `parse_object_data_chain`: Iterates through a linked list of data structures, decoding type-specific fields and updating object properties.
- `update_animation_state`: The routine updates a pair of 16-bit and 8-bit animation counters stored in a structure.
- `map_value_into_range`: The routine obtains a short value from another routine and maps it into the interval defined by two short fields of its parameter.
- `perform_32bit_addition`: Performs a 32-bit addition using two 16-bit values and updates two global variables with the result.
- `set_flag_on_lowercase`: The routine checks a byte at an offset from a pointer in A1.
- `update_status_flags`: Performs bitwise operations on a global variable to determine a value, updates a global variable with that value, and increments two other global variables.
- `process_joystick_input_state`: Reads the joystick and fire button status from hardware registers to derive a state value through bitwise operations.
- `empty_routine`: The routine contains no executable instructions and does not perform any operation.
- `clear_global_word`: Clears a longword in a global memory location.
- `wait_for_non_zero`: Enters a loop that continues as long as a specific global variable remains zero.
- `initialize_handler_entry`: The routine writes a word from a register into an object structure, sets a small constant field, clears a 32-bit field, stores a pointer to another routine as a handler, and writes a constant value into another field of the same structure.
- `update_structure_parameters`: Clamps a value in a global structure to a minimum threshold, writes a byte to an offset, calculates a new address based on bit-shifted and scaled values, and stores a bit-shifted constant into a table entry.
- `modify_global_value`: Increments or decrements a global variable by a fixed amount based on a comparison with a constant value.
- `compare_memory_blocks`: Iterates through two memory locations in steps of 18 bytes, checking if a bitwise XOR of the values at those locations matches a specific bitmask.
- `process_object_collision_and_state`: Iterates through a list of objects to check for bitwise collision patterns and updates their state or pointers accordingly.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `return_immediately_2`: This routine is empty and performs no operations before returning to the caller.
- `return_on_carry`: returns from the routine if the carry flag is set.
- `return_from_subroutine`: The routine is empty and performs no operations before returning.
- `mask_rotated_value_2`: Performs a bitwise AND between a bit-rotated 16-bit value and a 16-bit value retrieved from a memory offset.
- `mask_rotated_value`: performs a bitwise AND between a bit-rotated 16-bit value and a 16-bit value retrieved from a memory offset.
- `mask_memory_value`: Performs a bitwise AND between a 16-bit value and a 16-bit value retrieved from a memory location.
- `mask_register_value`: performs a bitwise AND between a 16-bit register and a 16-bit value retrieved from a memory offset.
- `mask_rotated_pointer_value`: performs a bitwise AND between a bit-rotated 16-bit register and a 16-bit value retrieved from a pointer.
- `mask_pointer_offset_value`: Performs a bitwise AND operation between a 16-bit register and a 16-bit value located at a specific offset from a pointer.
- `mask_conditional_pointer_value`: performs a bitwise AND between a 16-bit register and a value retrieved from a pointer, using different operands based on the carry flag.
- `mask_pointer_value`: Performs a bitwise AND between a 16-bit register and a 16-bit value loaded from an offset of a pointer.
- `increment_and_set_bits`: Increments a global variable and uses its shifted value to index a memory location, then performs a bitwise OR loop to set bits in a table.
- `store_calculated_value`: Calculates a value based on the difference between a register and a global variable, then stores this value into two memory locations pointed to by incrementing global pointers.
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
- `process_range_table_data`: Calculates a range of values between two pointers and populates a structure with derived indices, offsets, and metadata.
- `call_function_table_loop`: The routine repeatedly calls a function pointer taken from a table in A1, decrementing a counter in D3 until it reaches -1, then returns the value originally on the stack.
- `call_function_table_loop_2`: The routine repeatedly calls a function taken from a table via A1, decrementing a counter in D3 until it reaches -1, then returns the value originally passed on the stack.
- `call_function_table_loop_3`: The routine repeatedly calls a function pointer taken from a table and decrements a counter until the counter reaches -1.
- `update_short_table_entries`: The routine reads a short value from the input pointer and passes it to another routine, then computes and stores several short offsets derived from the input table into an output area, copying a few input shorts to the output and invoking another routine with computed indices.
- `invoke_callback_and_return_pair`: The routine invokes another routine selected from a table using an index derived from a short field in a structure pointed to by a global variable and an input value, then returns a 32-bit value formed by concatenating two input registers.
- `calculate_and_store_table_data`: Calculates several values based on pointer arithmetic and table lookups, then stores them into a memory block before entering a loop that updates a structure field based on a decrementing counter.
- `mask_memory_block`: Iterates through a block of memory, performing bitwise OR on the first byte and bitwise AND on subsequent bytes at fixed intervals.
- `wait_for_blitter_2`: Calls a routine to acquire the blitter and waits until a specific bit in a global variable is cleared.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `call_disown_blitter`: Library stub: loads its arguments and the graphics library base, calls DisownBlitter and returns its result to the caller.
- `setup_blitter_parameters`: Writes several constant and register-derived values into a memory block and requests blitter ownership.
- `initialize_blitter_structure`: Calculates a value derived from a table and pointer offsets, then writes this and other values into a memory block before requesting blitter ownership.
- `set_object_state_from_input`: The routine reads two shorts from a structure pointed to by its argument, calls another routine and uses the result together with a value from a register to conditionally load words from a table into fields of a structure pointed to by a register, then writes several fields of that structure with computed positions and flags derived from the call result, and finally calls another routine again.
- `update_object_properties`: Calculates several values based on a pointer and a table, then stores them into a structure at various offsets.
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
- `clip_source_rectangle`: Calculates the intersection between a source rectangle and a destination area to determine the valid sub-region for a bitplane transfer.
- `calculate_clipping_offsets`: Calculates the intersection between a source rectangle and a destination area to determine the valid sub-region for a bitplane transfer.
- `blit_clipping_rectangle`: Calculates the intersection between a source rectangle and a destination area to determine the valid sub-region for a bitplane transfer.
- `initialize_sprite_structure`: The routine prepares a sprite structure in a global variable by storing pointers and computed offsets derived from a table entry, initializing several fields to constant values and a combined value, then overwriting some of those fields with return values from another routine before returning.
- `update_global_pointers`: Increments two global pointers and stores a 16-bit value into the memory locations they previously pointed to.
- `perform_blitter_operations`: Initializes a blitter structure, waits for the hardware to become available, and performs memory clearing and data transfers using a loop.
- `tiny_ori_cmp`: Small routine of 2 instructions that merges 195 into d0, then compares d4 with -(a2).
- `clear_memory_block`: Clears a block of memory by writing zero to a series of words.
- `load_round_bob_set`: Loads the bob set of the round in d0 (Sprite A plus the round): clears the bob table, then fetches and unpacks the set so the round enemies can be drawn.
- `modify_register_and_stack`: Decrements a register using an offset, loads a constant into a register, and pushes a word from a global variable onto the stack.
- `check_bit_flags`: Iterates through a table to check if specific bits are set within a global data structure, provided certain conditions in a global variable are met.
- `test_bit_patterns`: Performs bitwise operations and shifts on a value to check for specific bit patterns across multiple memory locations.
- `process_handler_table`: The routine checks a global counter and, if a handler pointer is present, calls another routine and updates an entry in a table pointed to by a global variable, then iterates over the table calling the handler for each non-empty entry.
- `zero_jump_table`: Writes zero to eight consecutive words starting at a specific address and sets a global variable to a specific value.
- `clear_jump_table`: The routine clears a small jump table by writing zeros to eight consecutive 32-bit slots starting at the address of another routine, then sets a global variable to -1.
- `clear_active_zone_flag`: The routine clears a flag byte in a table of zone entries and records the flag value in a global variable.
- `update_moving_objects`: Iterates through a list of objects to update their positions based on movement type, including straight-line increments and path-following logic.
- `search_and_insert_coordinate_pair`: Searches a sorted table for a specific coordinate pair derived from a pointer offset.
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
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `clear_data_register`: Clears the low byte of a data register.
- `execute_indirect_calls_extended`: Calls two functions via pointers stored in a global variable and enters a loop that modifies a global variable until a specific bit is cleared.
- `infinite_loop_routine`: The routine checks a global variable for a zero byte.
- `update_split_value`: Performs a 32-bit addition using a global variable and a constant, then splits the result into two separate 16-bit storage locations.
- `decrement_bcd_two_digits`: The routine subtracts a byte value taken from a register, with borrow handling, from a two-digit packed BCD number stored in two consecutive bytes.
- `copy_data_block_2`: Calculates a source address based on a global variable and an offset, then copies a block of data to a destination address through a loop.
- `process_state_machine`: Manages a complex state machine using a counter and a jump table to execute different routines based on a global value.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d2.
- `update_state_index_and_dispatch`: Increments or decrements a state index based on a comparison value and handles wrap-around logic.
- `update_state_and_dispatch`: Updates a state index variable and uses a value to index into a table for an indirect call.
- `update_state_index_and_dispatch_3`: Updates a state index by incrementing or decrementing it based on a comparison value and handles wrap-around logic.
- `update_state_index_and_dispatch_4`: Manages a state index by incrementing or decrementing it based on a comparison value and handles wrap-around logic.
- `dispatch_state_index`: Manages a state index by incrementing or decrementing it and performs an indirect call through a table based on a value.
- `update_state_index_and_dispatch_5`: Updates a state index variable by incrementing or decrementing it based on a comparison value, applying wrap-around logic to keep it within a specific range.
- `update_state_index_and_dispatch_6`: Manages a state index by incrementing or decrementing it based on a comparison value and handles wrap-around logic.
- `update_state_index_and_dispatch_2`: Manages a state index by incrementing or decrementing it based on a comparison value and handles wrap-around logic.
- `run_repeated_sequence`: Executes a specific routine four times in succession.
- `parse_text_stream`: Iterates through a character stream to identify line endings and specific control characters while managing a loop counter.
- `clear_multiple_registers`: Clears the low byte of a data register multiple times.
- `process_compressed_data_stream`: Decodes an LZHUF compressed stream into a destination buffer and manages the associated linked object data.
- `reset_game_state_flags`: The routine checks a global flag and, if set, calls another routine and then clears two global variables.
- `initialize_global_structure`: Writes several constant values to a global structure and updates a character-based global variable using values from that structure.
- `life_lost`: Runs when the frame loop ends with a life lost: clears the cut, plays the loss sequence and takes a life; the caller goes to game over when none are left, otherwise restarts the round loop.
- `execute_recursive_calls`: This routine performs a series of six consecutive calls to itself before returning.
- `call_three_other_routines`: The routine calls another routine three times in sequence and then returns.
- `execute_indirect_call_3`: Sets a global byte value and then performs an indirect call through a table using a value from a global variable.
- `execute_indirect_call_2`: Modifies bits in a global variable and a memory location before performing an indirect call via a table.
- `execute_indirect_call`: Performs an indirect call using a value retrieved from a table and an offset.
- `round_clear`: Runs when the frame loop ends with the round cleared: shows the full picture and the round clear text and adds the round bonus before the next round is set up.
- `reset_global_counters`: Sets a global variable to zero and enters a loop that increments another global variable until it reaches one hundred, before setting a third global variable to two.
- `reset_game_state`: The routine clears a global variable and repeatedly calls another routine until the variable reaches a threshold, then sets a different global variable to a constant and returns.
- `tiny_move`: Small routine of 1 instruction that copies 99 to d0.
- `process_key_input`: Checks a global variable against specific values to return a result, then sets that variable to a constant and clears another.
- `process_input_loop`: Executes a loop that checks for specific key codes and updates a global variable until a termination condition is met.
- `check_input_status`: Checks for specific key codes within a loop and returns a value based on the result of the check.
- `check_escape_and_pause_keys`: Reads the last key code once a frame and acts on Escape (back to the title) and P (pause); returns non-zero to keep the frame loop going.
- `decrement_timer_tables`: The routine decrements a set of 32-bit timer counters stored in three consecutive tables in chip RAM, each table containing 32 entries.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `scan_five_timer_slots_recursively`: Walks a fixed table of five three-word records starting at a constant base address, and for each record whose first word is nonzero it calls the routine itself before advancing to the next record.
- `clear_data_registers_repeatedly`: Clears the low byte of a data register through a sequence of instructions.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `clear_data_registers_twice`: Clears the low byte of a data register twice.
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
- `store_calculated_values`: Calculates a value using a global variable and a 16-bit input, then stores the result into two different memory locations using incrementing pointers.
- `self_link_table_entries`: The routine writes the address of a global variable into the word located four bytes after each element of a table that starts at the global variable.
- `clear_object_flags`: The routine sets a flag in a global variable and then iterates over a table of objects, calling another routine twice for each entry that matches certain byte values, otherwise calling it twice and clearing a byte in the entry.
- `update_flag_table`: Iterates through a table and writes a bitmask derived from a specific bit position into a secondary table at a corresponding offset.
- `link_object_data`: Iterates through a table to copy data and link pointers into a destination structure based on values found in a global table.
- `expand_bob_set`: Iterates through a table of data to perform recursive expansions and updates several global pointers and counters.
- `expand_and_copy_data_block`: This routine performs a recursive expansion of a data block based on bitmask flags, involving memory copying and scaling of values.
- `build_bob_frame`: Builds one bob frame from the set: copies its four interleaved bitplane rows, applies the mirror and flip flags and makes the mask from colour 15 when asked.
- `compute_bitwise_and_not`: Performs a bitwise AND operation on four consecutive words and stores the bitwise NOT of the result into a destination buffer.
- `clear_register_d0_byte`: Clears the lower byte of a data register to zero.
- `invert_bitmap_rows`: The routine reads a short from a pointer and uses it to compute a destination pointer and a row stride.
- `compute_tile_metrics`: The routine walks a short array from a start pointer up to a limit pointer, reading a pair of dimensions and a flags word for each entry.
- `rotate_bitmap_block`: The routine copies a rectangular block of short words from a source buffer to a destination buffer and then rotates each 16-bit word left by one bit while applying a mask.
- `clear_d0_register`: clears the least significant byte of a data register using a logical or instruction with zero.
- `rotate_bitmap_bit_columns`: Takes a source word-array and a width/height pair, and walks it in 16-pixel-wide column strips, shifting one bit at a time out of each source word and assembling the bits into destination words written in reverse order.
- `transform_bitstream_to_tiles`: Performs a bitwise transformation on a source data stream by shifting and rotating bits into a new bit pattern.
- `clear_d0_register_2`: clears the least significant byte of a data register to zero.
- `rotate_bitmap_rows_bitwise`: Rotates a rectangular block of 1-bit-per-pixel bitmap data by one bit position, shifting each source word left and feeding the carried-out bit into a destination word assembled from the opposite end, with the source and destination pointers stepped by a row stride.
- `unpack_16bit_tilemap`: The routine unpacks a packed 16-bit tile map stored in a global table into a destination buffer using a global pointer.
- `copy_and_offset_blocks`: Copies a block of longwords from a source to a destination and then copies it again to a different offset within the destination.
- `copy_tile_rows`: The routine copies a rectangular block of 32-bit words from a source pointer to a destination pointer using a row-by-row loop.
- `set_linked_object_data`: The routine writes fields from a source structure into a newly allocated or linked object and links it into a chain.
- `execute_linked_callbacks`: Traverses a list of pointers and executes routines stored at those locations while checking for memory availability.
- `search_string_table`: Traverses a linked list of strings to find a match for a specific character sequence and stores the resulting pointer.
- `call_indirect_pointer`: Loads a pointer from a global variable and executes the routine located at that address.
- `call_avail_mem`: Wrapper that calls the exec library function AvailMem and returns its result to the caller.
- `store_value_if_large`: The routine calls another routine via a function pointer taken from a global variable and checks the returned size.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `close_file_and_seek`: The routine obtains a file handle from a global variable, calls a helper to test it, stores the result, and if the handle is valid it invokes another routine, closes the file via the DOS library, seeks the file via the DOS library, and returns the result of the close operation; otherwise it returns an error code.
- `close_file_if_open`: The routine calls a function through a table stored in a global variable and stores the result in another global variable.
- `swap_table_entries`: The routine reads a short value from another routine, uses it as an index into a table to fetch two bytes from a global table and writes them to two RAM locations, then swaps a byte from RAM with a byte from the table and clears a RAM byte.
- `reset_audio_and_dma`: The routine writes to a global variable six times and then returns.
- `process_audio_sequence`: Manages a sequence of data by incrementing counters, copying multiple 16-bit values from a source to a destination, and updating audio hardware registers.
- `update_audio_sample_parameters`: Calculates and updates audio playback parameters including sample length, frequency, and DMA settings based on an input command mask.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to the low twelve bits, stores the result into an offset of the same structure and clears a byte at another offset.
- `update_input_state`: Processes input data by reading a control word and branching based on a four-bit command mask.
- `gamepad_button_handler`: The routine reads the joystick fire buttons and disk status from the CIA port, then loops over a table of bytes, comparing each byte to the previous one.
- `execute_arithmetic_loop`: Performs a series of calculations involving bitwise masking, modulo operations, and coordinate updates before entering an infinite loop.
- `update_global_indices`: Calculates two new values based on a short integer using modulo and division operations, then stores them in two global variables.
- `perform_arithmetic_loop`: Executes a loop that modifies a global variable through repeated addition and bitwise operations.
- `process_object_state_and_timers`: Iterates through a list of objects to update their internal counters, handle state transitions, and manage linked-list structures.
- `lzhuf_decode`: Decodes an LZHUF stream (Okumura coder with a 4096-byte ring buffer and adaptive Huffman codes) into the output buffer; used for the bob sets.
- `initialize_prefix_sum_table`: Initializes a table with incremental values and then performs a prefix sum calculation on a sequence of short integers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `lzhuf_init_tree`: Initialises the adaptive Huffman tree of the LZHUF decoder: the leaf frequencies, the parent and child links and the root.
- `lzhuf_update_tree`: Updates the adaptive Huffman tree of the LZHUF decoder after a symbol: increments its frequency, re-sorts the nodes and rebuilds the tree when the root frequency reaches its limit.
- `calculate_sprite_tile_offsets`: Iterates through a data structure to retrieve sprite dimensions and properties from a table.
- `initialize_memory_block_2`: Writes several constant word values to specific offsets within a memory block pointed to by registers.
- `clear_object_table_entries`: The routine clears a table of object entries by copying two words from a source pointer into each entry and zeroing two other words.
- `decrement_pointer_and_copy_word`: Decrements a value pointed to by a register and copies a word from one memory location to another.
- `copy_word_from_pointer_to_global`: The routine copies a 16-bit word from an address pointed to by a pointer in a register to a global variable.
- `clear_three_globals`: The routine calls another routine twice, clears three word-sized global variables, then calls another routine twice.
- `update_object_state_2`: Decrements a value in a structure and performs conditional updates to various fields in a global structure based on the result and other memory contents.
- `decrement_and_update_counter`: Decrements a word value at a specific offset and conditionally increments or decrements another value based on the result and other memory checks.
- `update_packed_data_state`: Updates a set of global data structures by copying values from a packed data source and performing conditional bitwise inversions and arithmetic adjustments.
- `decrement_timer_and_advance_index`: The routine decrements a 16-bit counter in the object, resets it to a constant when it reaches zero, loads a value from a global table using an index from the object, writes that value into the object and advances the index.
- `set_status_from_table`: The routine reads a short from an input structure, indexes into a global table and a second table, compares the input short with the two table entries and writes constant values to two words in an output structure.
- `process_object_table`: The routine iterates over a table of objects and updates their fields.
- `update_object_position`: Adds values from a data source to a structure and increments a 16-bit index with wrap-around logic.
- `calculate_velocity_difference`: Computes the difference between source and target coordinates, stores the result in a destination buffer, and increments a counter if the difference is non-zero.
- `increment_ushort_with_wrap`: The routine reads a 16-bit value from an offset in a structure pointed to by a register, adds one, wraps it to zero when it exceeds the maximum signed 16-bit positive range, and writes the result back to the same location.
- `decrement_and_update_counters`: The routine decrements a counter in a global structure and, when it reaches zero, copies a value from another field, updates a masked position field in a second structure, possibly reverses a direction field, and then decrements a second counter, resetting it and adjusting related fields when it reaches zero.
- `round1_enemy_logic`: Round 1 enemy logic (entry 0 of the handler table at +E91A): runs the boss state machine (continuation pointer at object +44 with its delay at +48) unless frozen, then the three guard objects through their state table at +9A0E, then the respawn of guards when the count says so.
- `initialize_object_table`: The routine initializes a table of objects by writing constant values into a series of entries, then marks a global flag.
- `call_table_function_2`: Executes a function pointer retrieved from a table using an index stored in a structure.
- `decrement_counter_and_reset_state`: The routine decrements a short counter at offset ten from a pointer in a4 and, if the counter reaches zero, it resets the counter to one, calls another routine, copies a word from a table pointed to by a5 into the structure at a4, and writes constant words to several fields of the structure at a4.
- `update_entity_state_a`: Checks a global flag to conditionally decrement counters and update positions, clears collision flags, checks bounds, and dispatches to a handler table if conditions are met.
- `decrement_counter_and_toggle_flag`: The routine decrements a short counter stored at an offset from a pointer in A4 and writes the new value back.
- `initialize_object_fields`: The routine writes constant values into several word fields of a structure pointed to by a register.
- `update_object_state`: Checks a flag and a global variable, then performs conditional updates to structure members including sign inversion and value copying.
- `process_complex_logic`: Performs a series of conditional checks, memory updates, and loops involving multiple global variables and function calls.
- `set_two_word_values`: The routine writes the value four to two consecutive word locations in a global variable and then calls another routine.
- `check_two_word_flags`: The routine calls another routine and then returns a flag.
- `decrement_counter_reset_animation`: The routine decrements a 16-bit counter stored at an offset within a structure pointed to by a register.
- `set_status_from_table_2`: The routine reads a two-bit field from an object structure, uses it as an index into a global variable table and a second table, compares a short field of the object with the looked-up values and writes a short status word in the object.
- `decrement_timer_and_update_state`: The routine decrements a short timer in a structure and, when it reaches zero, resets the timer and calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_and_call`: Fills a block of memory with zeros using a loop and then executes an indirect function call via a table.
- `dispatch_indirect_function`: Calculates an offset from a global variable using a value stored at a fixed displacement, then executes the function pointer found at that calculated address.
- `execute_callback_and_format_string`: Calls a function pointer from a table and, if certain conditions are met, converts a value to a character and writes two words to a memory location.
- `initialize_memory_block`: The routine calls another routine via a table and then writes two 16-bit values to memory locations offset from an address in A4, setting one word to a constant and another word to a constant.
- `update_sprite_position`: Calculates new coordinate offsets for a sprite by applying velocity and handling boundary wrapping or clamping.
- `accumulate_flag_masks`: The routine iterates over seven entries in a table, and for each entry whose flag is set it copies two words from the entry into two global variables, calls another routine, and ORs the returned mask into an accumulator global variable.
- `read_two_word_table`: The routine calls another routine and then checks a global variable.
- `update_entity_bounding_box`: Adjusts coordinate offsets within a structure and modifies specific state flags based on conditional checks.
- `update_game_state_flags`: The routine clears and sets fields in a game state structure, copies values from a table to the structure, and conditionally updates four short fields in a substructure and manipulates flag words.
- `reset_zone_state_2`: Clears several fields in a structure pointed to by a register, copies two words from another structure into it, and invokes another routine.
- `reset_zone_state_variant`: Clears several fields in a structure, conditionally sets a byte based on a global variable, and copies data from another structure.
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
- `empty_routine_2`: The routine contains twelve instructions and performs no observable work; it ends with a return.
- `accumulate_short_field`: The routine reads a short from an object field, calls another routine twice, then adds the read value to another short field of the same object and stores the sum back.
- `rotate_left_and_shift`: The routine rotates a 16-bit value left by a variable amount masked to the low six bits and then left-shifts the result repeatedly until a signed counter reaches -1.
- `decrement_counter_and_update_table_entry`: The routine receives a pointer to a small table in memory and decrements a counter stored at offset five.
- `update_entity_state_flags`: The routine updates a 16-bit field at offset six of a structure pointed to by an address register using a 16-bit value in D0.
- `clamp_value_high`: Compares two words in a structure and sets the first to the value of the second if the first is smaller.
- `set_flag_based_on_offset`: The routine writes a constant to a word in a global variable and then reads a short from a global variable and subtracts a short from another global variable.
- `test_collision_table`: Iterates through a table of values to check against specific thresholds and updates a structure if conditions are met.
- `increment_counter_and_adjust_value`: The routine increments a counter stored in a table pointed to by a register and, if a flag bit in a global variable is set, subtracts a constant from another value in the same table entry.
- `copy_word_from_table`: The routine copies a 16-bit word from a table indexed by a short value in a structure to a field in another structure.
- `set_constant_value`: Stores the constant value four into a word at an offset within a structure.
- `select_quadrant_index`: Calculates the absolute difference between two coordinate pairs to determine an index into a global table.
- `copy_state_flags_and_mask`: The routine reads a word from a global variable via an address in A4, tests bits in that word and conditionally copies a word from an offset in the same structure to the base and another word to an offset.
- `clear_collision_flag`: The routine clears a two-byte field in a structure pointed to by a pointer, then if a flag byte in another structure is zero it computes the absolute difference of two short values from two structures, and if both differences are less than a small threshold it writes a sentinel value to the same two-byte field and sets a global variable to a constant.
- `copy_word_block`: The routine copies a block of 16-bit words from a source pointer to a destination pointer.
- `copy_stack_data`: Copies a sequence of words from the stack to a memory location and then performs an indirect call.
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
- `empty_routine_3`: The routine contains five instructions and performs no observable work; it ends with a return.
- `cursor_step_two_bits_left`: Line cursor helper: steps the two-bit pixel mask left by one pixel, moving the screen pointer back one word when the mask is already in the top two bits.
- `cursor_step_three_bits_left`: Line cursor helper for 120-byte rows: steps the three-bit pixel mask left by one pixel, moving the screen pointer back one word when the mask is already near the top of the word.
- `empty_routine_4`: The routine is empty.
- `empty_routine_5`: The routine is empty and performs no operation.
- `cursor_step_three_bits_right`: Line cursor helper for 120-byte rows: moves the screen pointer forward one word when the pixel mask is in the lowest three bits, then rotates the mask by three bits.
