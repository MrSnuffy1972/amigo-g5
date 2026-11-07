# Game 1671 (starglider 2 - the egron's strike back (1988)(rainbird)[cr unit a - wow][t +3 defjam]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01671). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (921584 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 14 raw-word lines |
| Functions named and described | pass, 127 of 127 named (0 author symbols), 127 of 127 described (6 by rule, 103 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 898 of 898 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2503 of 2503 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 225466 |
| code | 9724 |
| data | 25765 |
| padding | 1173 |
| unused | 659456 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Starglider 2 - The Egron's Strike Back (1988)(Rainbird)[cr Unit A - WOW][t +3 Defjam].adf:load | 1 | 272 | 19 |
| Starglider 2 - The Egron's Strike Back (1988)(Rainbird)[cr Unit A - WOW][t +3 Defjam].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 221 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1700 | 83 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 70 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 127 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 66 | 1 |

## Cases handled

- runtime segment an address: 208 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-keys): 21 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-keys): 433 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-keys): 43 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-keys): 16 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-keys): 127 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-keys): 50 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- starglider 2 - the egron's strike back (1988)(rainbird)[cr unit a - wow][t +3 defjam]/Starglider 2 - The Egron's Strike Back (1988)(Rainbird)[cr Unit A - WOW][t +3 Defjam].adf:load: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_instruction_table` (runtime segment an address (play-long-keys)): Initializes a block of memory with a sequence of incrementing or fixed values and then iterates through this table to execute or check certain instructions. Kind: instruction table/jump table reconstruction (sabotage path). Pass path: the loop that populates _DAT_000121c0 with valid pointers from the runtime segment, avoiding the zero-check (bVar6 = _DAT_000121c0 == 0) or the invalid pointer trap (u4__sub_40C0/u4__sub_3A26/func_0x000109a0/u4__sub_38D2) or the infinite lo.
- `relocate_pointer_tables` (runtime segment an address (play-long-keys)): Subtracts a base address from a series of discrete memory locations and iterates through several arrays of pointers to convert absolute addresses into relative offsets. Kind: relocation-based loader/protection. Pass path: The routine adjusts pointers relative to a base address (unaff_A6), which is a characteristic of a loader or a relocation-based protection mechanism that allows code to run at any memory offset. Since the routine modifies pointer tables (re.

## Functions


### Starglider 2 - The Egron's Strike Back (1988)(Rainbird)[cr Unit A - WOW][t +3 Defjam].adf:load

- `enter_supervisor_and_configure_dma`: Switches the CPU to supervisor state with exec's SuperState and then writes the DMA control and interrupt enable registers.
- `reconstruct_bitstream_from_packed_data`: This routine processes a stream of packed data by extracting bits through repeated right-shifting and bitwise operations.
- `shift_pointer_value`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `shift_right_bits_with_early_exit`: The routine shifts a 32-bit value right by one bit repeatedly, loading a new 32-bit word from a table pointed to by a pointer each time the shifted value becomes zero, and stops early if a counter reaches -1.
- `open_graphics_and_poll_status`: Opens the graphics library, swaps a saved field in its base for a new value, and loops calling three unnamed routines while a status bit stays set.
- `copy_longwords_until_sentinel`: Copies 32-bit words one by one from a source table into a destination buffer.
- `clear_longword_buffer`: Writes zero to every 32-bit word in a fixed buffer, then clears a global variable.
- `scan_text_rows_until_terminator`: Walks a text table in rows of 40 bytes, calling a per-character unnamed routine for each nonzero byte.
- `copy_character_column_to_buffer`: Maps a character code to a 32-entry column offset in a lookup table, adding a 256-byte bank when bit six is set.
- `decode_keyboard_serial_byte`: Reads the keyboard serial data register and, when the handshake bit is set, converts the byte into a bit index.
- `chain_three_routine_calls`: Calls three unnamed routines one after another and contains no logic of its own.
- `write_bit_on_off_text`: Tests one bit of a global flag word, chosen by the low three bits of the input, and writes the text OFF or ON into the destination.
- `invoke_two_routines`: Calls two unnamed unnamed routines in sequence, and the decompiler produced no pseudo-code for the routine itself.
- `expand_fixed_marker_runs`: Expands a marker-encoded byte stream from a fixed start: literal bytes are copied, and a marker followed by a count and a value writes that value count times.
- `expand_marker_runs_from_pointer`: Expands the same marker-encoded stream format as its neighbour, but reads from a pointer register instead of a fixed start.
- `expand_marker_runs_checked_last`: Expands a marker-encoded byte stream and checks for the end byte after each literal and run pair.
- `expand_leading_marker_run_stream`: Decodes a marker-encoded stream that begins with a counted run, copying literal bytes between runs until the end byte.
- `load_constant_via_routine`: Calls a two-instruction unnamed routine that loads a constant into a data register and returns it.
- `tiny_move_rts`: Small routine of 2 instructions that copies 145 to d0, then returns.

### Starglider 2 - The Egron's Strike Back (1988)(Rainbird)[cr Unit A - WOW][t +3 Defjam].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reconfigure_graphics_loop`: Configures a hardware register, calls a routine, and copies a block of memory until a specific word value is encountered.
- `repack_bitstream_data`: Iterates through a memory buffer to extract and shift bits from a sequence of words, reassembling them into a new bitstream and updating a color register.
- `shift_pointer_value_2`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `find_highest_bit`: Decrements a pointer and checks the most significant bit of the data at that location until a non-zero value is found or a counter reaches zero.
- `reconfigure_graphics_and_poll_status`: Opens the graphics library, updates a field in its base with a new value, and loops calling three routines while a bit in a register or global variable remains set.
- `copy_longwords_until_sentinel_2`: Copies 32-bit words from a source table into a destination buffer, terminating when a word equal to minus two is encountered.
- `clear_long_buffer_and_reset_flag`: Iterates through a fixed-size buffer, setting every longword to zero, and then resets a global variable to zero.
- `iterate_and_load_table_entries`: Iterates through a sequence of null-terminated strings, calling another routine for each string found.
- `load_table_entry`: Calculates an offset based on an input value to select a source table, then copies eight blocks of data to a destination pointed to by a global variable.
- `update_keyboard_bitmask`: Checks if a keyboard serial data register contains a pending byte, then extracts a bit index to update a global bitmask representing key states.
- `chain_three_routine_calls_2`: Calls three unnamed routines one after another and contains no logic of its own.
- `update_bit_status_text`: Checks a specific bit within a global flag word, using a bitmask derived from the lower three bits of an input register, and writes the string 'OFF ' or 'ON  ' to a destination pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `extract_maximum_values`: Iterates through a list of pointers to copy multiple fields to a destination structure and finds the maximum value within a sub-array of shorts.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `parse_string_with_formatting`: Parses a null-terminated string to modify a global counter and a memory-based pointer or value.
- `parse_control_stream_and_toggle_flags`: Parses a stream of encoded control characters to modify a pointer and a counter, while conditionally performing bitwise negation on a table.
- `parse_command_string`: Iterates through a string-based command stream to update a global variable and modify a bit-toggled lookup table.
- `search_global_value`: Iterates through a global variable or table, checking for specific bit patterns or values, and writes a constant to a global variable if a certain condition is met.
- `toggle_global_bit_2`: Toggles a bit in a global variable using a bitwise XOR operation with a constant value.
- `toggle_global_bit`: toggles a bit in a global variable using a bitwise XOR operation with a constant value.
- `process_key_input_string`: Iterates through a string of characters to check for specific key presses and updates a global flag or memory location.
- `parse_bitstream_to_buffer`: Iterates through a source buffer to extract or shift bits, packing or unpacking them into a destination memory location.
- `execute_recursive_function`: Recursively calls another routine and then executes a function pointer stored in a global variable.
- `copy_memory_until_marker`: copies a block of memory from a source address to a destination address until a specific two-byte signature is encountered.
- `set_color_register`: stores a constant value into a color register.
- `update_accumulator_and_counters`: Increments and decrements several internal counters and performs arithmetic updates on a set of three accumulator values.
- `mask_hardware_register`: performs a bitwise AND on a hardware register value and stores the result in a global variable.
- `mask_interrupt_request`: performs a bitwise AND on the interrupt request register and stores the result in a global variable.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `update_bit_shifted_value_2`: Checks a bit in a custom hardware register and updates a global variable with a masked value.
- `update_bit_shifted_value`: Calculates a bit-shifted value and writes it to a memory location relative to a register, then updates a global variable.
- `update_table_and_mask_port`: Updates a memory-mapped table or structure with a value from a register, conditionally setting a secondary value if a bit is clear, and masks a bit in a custom I/O port register.
- `write_constant_to_offset_address`: Writes a constant value to a memory location calculated by adding an offset to a register-held address and updates a global variable with a routine address.
- `execute_sequential_routines_2`: This routine is unreached and contains a sequence of conditional checks on a global variable that trigger various execution paths, including calling other routines and writing bytes to memory or updating a global pointer.
- `execute_sequential_routines`: Executes a sequence of routines and writes a constant to a memory location relative to a register.
- `store_routine_results`: Iterates through a memory range, storing the return value of a routine into a global buffer.
- `populate_memory_from_routine`: Executes several routines and then copies a byte from a routine's return value into a memory buffer until a certain value is reached.
- `loop_until_condition`: Executes a series of routines and then loops until a certain value is returned by another routine.
- `set_memory_offset`: Writes a constant value to a memory location relative to a register and updates a global variable.
- `store_register_to_offset`: stores a constant value into a memory location calculated by adding an offset to a register, and then updates a global pointer with the routine's own address.
- `update_global_state_2`: Repeatedly calls another routine until a certain value is zero, then writes a constant to a memory offset and updates a global variable.
- `write_memory_offset`: Executes another routine multiple times before writing a constant to a memory offset and updating a global variable.
- `clear_global_variable`: sets a global variable to zero.
- `set_global_to_max`: sets a global variable to a maximum byte value.
- `wait_for_bit_set`: loops until a specific bit in a value is set, then stores that value in a global variable.
- `poll_global_bit`: loops until a specific bit in a global variable is set.
- `mask_interrupt_request_bit`: performs a bitwise AND on a hardware interrupt request register and stores the result in a global variable.
- `set_multiple_globals`: sets a global variable to two different constant values sequentially.
- `fetch_table_value`: uses a byte from a memory location as an index into a table and stores the retrieved value in a register before clearing the index byte.
- `initialize_graphics_and_reset_state`: Clears a range of memory, initializes several hardware registers including bitplane and color settings, and resets a sequence of global variables and a function pointer table.
- `set_global_value`: Sets a global variable to a fixed value.
- `do_nothing`: the routine is empty and performs no operations before returning.
- `process_input_data`: Reads values from CIA and custom hardware registers to update several internal state variables and calculate differences between current and previous input values.
- `update_sprite_coordinates`: Calculates and updates sprite position values based on input or movement data.
- `update_bcd_values`: Updates three consecutive memory locations using BCD arithmetic and performs a series of data transfers from a table to multiple destination buffers based on a bit-rotated index.
- `update_bcd_values_2`: Updates two memory locations using BCD-adjusted arithmetic and copies a series of bytes from a table to multiple destination addresses.
- `populate_lookup_tables`: Performs bitwise rotations on a value to generate indices for a series of memory copies.
- `clear_memory_block`: Fills a block of memory starting at a specific address with zeros using a loop.
- `get_input_index`: Checks specific memory offsets for non-zero values and returns a value from a table based on a byte found at an offset.
- `get_input_index_2`: Checks if a byte at a specific offset is non-zero, clears that byte, and returns a value from a table based on that byte.
- `parse_control_stream`: Parses a stream of characters to modify a pointer and a counter, while conditionally performing bitwise negation on a table and checking for specific control characters.
- `calculate_bit_shifts`: Performs complex arithmetic to calculate shift amounts and bitwise operations to update several global memory locations using values from a table.
- `initialize_and_map_command_data`: Clears a block of memory using nested loops and then iterates through a string to map bytes from a lookup table into a secondary, distributed table structure based on the character's bit-shifted index.
- `populate_key_lookup_table`: Copies a block of data from a source address and clears a secondary memory buffer before iterating through a string-based lookup table to map character-based offsets to a destination table containing or-ed or mapped attribute/pointer data.
- `parse_command_stream_and_toggle_flags_2`: Parses a stream of encoded control characters to modify a pointer and a counter, while conditionally performing bitwise negation on a table.
- `parse_command_stream_and_toggle_flags`: Parses a stream of encoded control characters to modify a pointer and a counter, while conditionally performing bitwise negation on a table of flags.
- `parse_string_command_and_toggle_flags`: Parses a string-based command or instruction stream to perform conditional bitwise negation on a table of flags and update a pointer or counter.
- `parse_encoded_command_stream`: Iterates through a stream of encoded bytes to modify a global variable and a pointer, performing bitwise negation on a table or memory location if certain conditions are met.
- `parse_string_command_and_toggle_flags_2`: Parses a string-based command or instruction stream, updating a global counter and toggling bit-flags in a memory table based on certain character triggers.
- `parse_string_command_bytecode`: Parses a custom bytecode-encoded string or instruction stream to perform conditional jumps, bitwise negation of memory or table entries, and pointer updates.
- `return_stack_value`: Returns the value currently stored on the stack.
- `parse_command_stream_and_toggle_flags_3`: Parses a stream of encoded control characters to modify a pointer and a counter, while conditionally performing bitwise negation on a table or updating a global pointer.
- `parse_string_command_stream`: Parses a stream of encoded control characters and data, updating a pointer or a counter and potentially toggling bits in a lookup table.
- `copy_data_blocks`: Copies multiple bytes from four different source locations into a single destination buffer at various offsets.
- `modify_register_value`: This routine executes two instructions and is called by two unnamed routines.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `execute_recursive_copy_loop`: Recursively calls another routine and copies a block of memory from a global variable to a fixed destination until a specific word-marker is encountered.
- `execute_recursive_copy_loop_2`: Updates a register or memory location, calls another routine, and then copies data from a global variable to a fixed memory location until a specific word-marker is encountered, before calling another routine and resetting a register or memory location.
- `execute_recursive_copy_loop_3`: Recursively calls another routine and copies data from a global variable to a fixed memory location until a specific word-marker is encountered.
- `execute_recursive_loop`: Executes a routine recursively while updating a register and a global variable, then copies data from a global variable to a fixed memory location until a specific word-marker is encountered.
- `mask_global_byte`: Performs a bitwise AND operation on a global byte with a constant value.
- `update_global_byte`: Performs a bitwise OR and bitwise AND operation on a global byte with constant values.
- `empty_routine`: the routine contains no executable instructions.
- `empty_routine_2`: This routine is empty and contains no executable instructions.
- `empty_routine_3`: the routine contains no executable instructions or valid instructions before an invalid or missing instruction error occurs.
- `empty_routine_4`: the routine contains no executable instructions or valid instruction data.
- `check_triple_null_condition`: Checks if three consecutive bytes at a certain offset from a pointer are all zero, returning zero if true or a full bitmask if false.
- `initialize_instruction_table`: Initializes a block of memory with a sequence of incrementing or fixed values and then iterates through this table to execute or check certain instructions.
- `update_global_state`: Checks a bit in a global variable and either decrements another global or resets a counter and updates a global with a value from a table.
- `decrement_global_counter`: Decrements a global variable and updates a global variable with a value from a table if a certain condition is met.
- `update_global_pointers`: stores a constant value into a global variable and clears another global variable via an offset from a pointer.
- `reset_struct_fields`: Sets certain fields in a structure to one or zero and assigns specific constant values to other fields using a pointer held in a register.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `copy_memory_block`: copies a fixed number of words from a source address to a destination address using a decrementing counter.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `copy_memory_block_2`: Copies a large block of data from a source address to a destination address using a loop.
- `relocate_pointer_tables`: Subtracts a base address from a series of discrete memory locations and iterates through several arrays of pointers to convert absolute addresses into relative offsets.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `compare_string_and_decrement_counter`: Compares two strings byte-by-byte and updates a global counter or pointer.
- `execute_recursive_string_copy`: Calls another routine, then copies a sequence of words from a global variable address to a fixed buffer until a specific terminator word is encountered, before calling another routine and resetting a register.
- `execute_recursive_string_copy_2`: Updates a register, calls another routine, and then copies a sequence of words from a global variable to a fixed buffer until a specific terminator word is encountered; the routine then calls another routine and resets a register before looping indefinitely.
- `process_input_characters`: Reads characters and updates a global variable, handling backspaces, carriage returns, and escape sequences while storing characters into a buffer.
- `execute_recursive_string_copy_3`: Executes another routine, then copies a sequence of words from a global variable to a fixed buffer until a specific terminator word is encountered, before executing a third routine and resetting a register.
- `execute_recursive_string_copy_4`: Executes another routine, then copies a sequence of words from a global variable to a fixed memory buffer until a specific terminator word is encountered, before calling another routine and resetting a register.
- `process_bit_stream_to_memory`: Iterates through a bitstream, performing bit-shifting and masking operations to extract or reconstruct bit sequences, and writes the resulting bytes or bits into a memory buffer.
- `get_key_value`: Checks a flag in a memory location and, if a certain byte is non-zero, clears that byte and returns a value from a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_data_table`: Populates a memory buffer with a repeating pattern of calculated short integer values using nested loops.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_game_state_structures`: Clears multiple memory blocks and initializes various data structures with default values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_long_key_buffer`: The routine performs a sequence of 66 instructions that modifies a value or a byte within a memory range.
