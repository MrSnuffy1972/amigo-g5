# Game 619 (Lotus Esprit Turbo Challenge): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00619). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (921264 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 94 raw-word lines |
| Functions named and described | pass, 150 of 150 named (0 author symbols), 150 of 150 described (6 by rule, 142 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1508 of 1508 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4186 of 4186 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 36101 |
| code | 16456 |
| data | 780435 |
| padding | 208 |
| unused | 88064 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Lotus Esprit Turbo Challenge.adf:bootblock | 1 | 18 | 2 |
| logged disk load from Lotus Esprit Turbo Challenge.adf boot-block load | 1 | 127 | 2 |
| logged disk load from Lotus Esprit Turbo Challenge.adf boot-block load | 1 | 134 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3762 | 139 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 145 | 5 |

## Cases handled

- Lotus Esprit Turbo Challenge/Lotus Esprit Turbo Challenge.adf: custom-format (non-DOS) disk, 1582 non-empty blocks: matched against track-loaded segments
- Lotus Esprit Turbo Challenge/Lotus Esprit Turbo Challenge.adf: the boot block reads 30208 bytes from disk offset 867328 to an address (found statically); recovered as a unit
- Lotus Esprit Turbo Challenge/Lotus Esprit Turbo Challenge.adf: the boot block reads 5632 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- runtime segment an address: 1380 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 128 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Lotus Esprit Turbo Challenge/Lotus Esprit Turbo Challenge.adf: 2 logged loads cover 35840 bytes of the image
- Lotus Esprit Turbo Challenge/Lotus Esprit Turbo Challenge.adf: blocks 22-203 (bytes 11264-104448) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.62)
- 1 packed track region(s), 683008 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `calculate_buffer_offset_with_sync` (runtime segment an address): Waits for a bit in a certain memory location to change, then iterates through a memory range to calculate an offset. Kind: track/sector length check. Pass path: sVar2 (the calculated offset/length-based value).

## Functions


### Lotus Esprit Turbo Challenge.adf:bootblock

- `read_disk_block_and_patch`: Issues one disk read request through the operating system I/O call, using a fixed length, disk offset and memory destination.
- `read_disk_block_then_call`: Issues a disk read through the operating system I/O call using a fixed length, disk offset and memory destination, then calls a routine at a fixed address and returns.

### logged disk load from Lotus Esprit Turbo Challenge.adf boot-block load

- `tiny_clr`: Small routine of 1 instruction that clears d0.
- `xor_accumulate_color00`: Loops decrementing a counter while toggling a flag, XORing a global variable through a pointer and storing the result.

### logged disk load from Lotus Esprit Turbo Challenge.adf boot-block load

- `shift_table_bytes_backward`: Walks backward through a byte table, overwriting each entry with the byte that precedes it.
- `xor_accumulate_color_byte`: A loop that decrements two counters until one reaches -1, conditionally storing a byte from a table through a decrementing pointer into a global variable and decrementing a flag each iteration, while touching the COLOR00 hardware register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `invoke_audio_channel_initialization`: Calls a routine that initializes an audio channel block and returns a concatenated 64-bit value composed of the contents of two 32-bit registers.
- `pass_register_values`: Copies the contents of two 32-bit registers or a single 64-bit value through the routine without modification.
- `reset_audio_channel_volumes_2`: This routine calls a routine that writes a certain value to one status global, zeroes four globals, and writes a certain value to a fifth.
- `call_and_return_registers`: Calls another routine and returns the contents of two registers.
- `update_audio_and_set_flag`: calls a routine to update audio parameters and sets a global flag to one.
- `split_words_to_globals_2`: This routine calls a routine that checks a status byte and, if set, splits two input words into four separate globals.
- `call_and_return_registers_2`: Calls another routine and returns thecontents of two registers.
- `check_audio_callbacks`: tests a status byte and conditionally executes two different routines if certain flags or non-zero values are present.
- `clear_audio_channel_state`: Zeroes five state words in the audio channel block and four further globals.
- `initialize_audio_channel_block`: Sets a status byte to a certain value, clears a set of counters and flags, sets a bit in a global variable, and stores a pointer computed from a table index.
- `reset_audio_channel_volumes`: Writes 255 to one status global, zeroes four globals and writes fifteen to a fifth.
- `process_audio_command_stream_2`: Parses an audio command stream to update audio registers and certain globals, while also managing a DMA-enabled buffer and updating a pointer-based address.
- `parse_audio_command_stream`: Parses a command stream to update audio parameters or modify a global buffer.
- `process_audio_command_stream`: Parses a command stream to modify audio parameters or copy values into a global buffer.
- `split_words_to_globals`: Checks a status byte and, when set, splits the incoming D0 and D1 words into four separate globals.
- `call_audio_update_and_set_flag`: Calls a routine to update audio parameters and then sets a global flag to one.
- `call_then_set_second_flag`: Calls another routine and then sets a second global variable to one.
- `run_pending_audio_callbacks`: Checks a status byte and calls two different routines if specific flags or non-zero values are present.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `update_audio_parameters`: Updates audio channel length and frequency or modifies a table-driven sound parameterset.
- `enable_dma_channels_with_delay`: Sets a DMA enable bit pattern in a channel state word, writes it to the DMA control register, waits a short count, then copies two saved state words into globals.
- `start_audio_dma_with_delay`: Sets the DMA enable bits in a channel state word, writes it to the DMA control register, waits a short count and copies two saved state words into globals.
- `set_audio_parameters`: Sets a global variable to a specific value, executes two other routines, and performs a loop that decrements a register, before updating a global variable with a bitwise OR or a value from a memory location.
- `build_channel_pointer_table`: Fills a table of 32 entries with pointers into a buffer.
- `search_table_for_value`: searches a memory table for a specific word and calculates the offset from a fixed starting point.
- `calculate_buffer_offset_with_sync`: Waits for a bit in a certain memory location to change, then iterates through a memory range to calculate an offset.
- `update_sprite_attributes_recursive`: Performs a series of complex operations including clearing and verifying memory, updating sprite or tile attribute buffers, and recursively calling itself based on a global variable's value.
- `stub_no_visible_effect`: The routine only assigns global variables to themselves, so it has no visible effect.
- `clear_and_verify_memory`: Clears a memory area, copies data from one location to another, and compares the copied data against a reference to set a global flag.
- `mask_word_and_store`: It masks a word from an offset in a pointer with the value 3 and stores the result at another offset in that pointer.
- `handle_vblank_state_update`: Runs on the vertical blank interrupt, acknowledges interrupt request bits, and updates a state value through a sequence of comparisons and table-driven copies.
- `reset_audio_and_set_state`: Resets audio channel volumes, zeroes several global variables, and sets state words to minus one before calling further helper routines.
- `set_state_and_call_helpers`: Sets two state words to minus one, stores a helper result, and calls three further routines.
- `reset_audio_volume`: Calls a routine that resets audio volumes and then sets a global variable to a maximum value.
- `run_conditional_helpers_reset_flags`: Runs a sequence of conditional helper calls based on state globals, then clears two flag words and resets two state globals to minus one.
- `calculate_scaled_value`: Calculates a value based on the absolute value of a register and returns a concatenated 32-bit result.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `calculate_scaled_value_2`: Performs absolute value and scaling logic on a register and returns a concatenated 32-bit result.
- `calculate_scaled_offset`: calculates a 32-bit value consisting of a bit-shifted input and a clamped, offset-adjusted version of that input.
- `empty_routine`: the routine is empty and performs no operations or data transfers.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `update_global_index`: Checks a global variable against a constant and, if certain conditions are met, updates that global variable to a new value.
- `check_and_update_global_state`: Checks if a global variable equals a specific value and calls another routine if so.
- `set_global_flag`: Checks if a global variable equals zero and certain range, then sets it to one.
- `decrement_global_variable`: Decrements a global variable if a specific bit is set and the variable is not zero.
- `invoke_sprite_update`: calls a routine that updates or copies sprite or tile attribute data in a global buffer.
- `clear_global_variable`: Sets a global variable to zero and calls several other routines.
- `update_sprite_attributes`: This routine calls a routine that parses a command stream to update or copy sprite or tile attribute data in a global buffer.
- `copy_and_update_data`: Copies a block of bytes from a source to a destination and performs several conditional updates to global variables and table data.
- `update_data_buffer`: Copies bytes from two source locations into a global table and performs a series of conditional swaps on segments of that table.
- `copy_conditional_data`: Searches a table for a value and copies a fixed number of bytes from a source to a destination if a condition is met.
- `recursively_call_routine`: Checks if a global variable equals a specific value and then calls itself either once or twice depending on that condition.
- `update_sprite_attributes_2`: Calls a routine to update or copy sprite or tile attribute data in a global buffer.
- `invoke_sprite_attribute_update`: Calls a routine that processes a command stream to update or copy sprite or tile attribute data in a global buffer.
- `copy_telemetry_buffer`: Selects a destination pointer based on a global variable and copies bytes from a calculated offset in a table to that destination.
- `convert_value_to_ascii`: Converts a numerical value into a sequence of ASCII characters by repeatedly calculating the remainder and quotient of the value divided by ten.
- `convert_sum_to_ascii`: Calculates the sum of adjacent bytes from a source list and stores the result as an ASCII character in a destination list.
- `empty_routine_2`: The routine is empty and performs no operations.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `invoke_sprite_attribute_update_2`: calls a routine that processes a command stream to update or copy sprite or tile attribute data in a global buffer.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `invoke_sprite_update_2`: This routine calls a routine that updates or copies sprite or tile attribute data in a global buffer using a command stream.
- `invoke_sprite_attribute_update_3`: Calls a routine that processes a command stream to modify or duplicate sprite or tile attribute data in a global buffer.
- `invoke_sprite_attribute_update_4`: Calls a routine that updates or copies sprite or tile attribute data in a global buffer.
- `invoke_sprite_attribute_update_5`: Calls a routine that parses a command stream to update or copy sprite or tile attribute data in a global buffer.
- `perform_bitmask_decompression`: Performs a nested loop to decompress data by XORing and bitwise ANDing bytes from two source buffers into a destination buffer.
- `update_sprite_attribute_table`: Parses a command stream to update or copy sprite or tile attribute data in a global buffer.
- `apply_bitmask_to_buffer`: Iterates through a source table to perform bitwise OR operations on specific bit positions within a destination buffer.
- `update_state_and_copy_data`: Updates state-related globals and function pointers, performs a nested loop to copy a block of data, and updates a table-driven pointer or offset.
- `set_bit_and_swap_pointers`: Sets a bit in a byte pointed to by a register and swaps the contents of a global pointer with a value returned from another routine.
- `set_bit_in_byte`: Sets a specific bit in a byte at a memory location pointed to by a register, where the bit index is determined by a register value masked or otherwise masked with a bitmask.
- `clear_function_pointer`: Replaces a global function pointer with a previously saved address.
- `swap_function_pointers`: Saves two function pointers from a table, calls another routine, and then restores the original pointers and updates a global variable.
- `check_value_range`: Compares a value against certain thresholds and conditionally calls another routine.
- `clear_d0_register`: Performs a logical OR operation on the data register zero with zero to clear its contents.
- `copy_memory_block`: Performs a nested loop to copy data from one memory location to another in large chunks.
- `populate_data_table`: Repeatedly calls a routine to copy a block of data from a source table to a destination table using a nested loop structure.
- `copy_structured_data`: Copies data from a source table to a destination table using a nested loop with specific offsets.
- `call_indexed_copy_and_subroutine`: Calls a routine to copy data from a source table to a destination table using an index and offsets, then calls another routine.
- `copy_indexed_data`: Copies data from a source table to a destination table using an index and specific offsets.
- `update_sprite_attribute_buffer`: Checks a global index against a table and, if certain value conditions are met, copies a byte from the table into a destination buffer while incrementing or decrementing the global index.
- `update_counter_and_call`: Increments or decrements a value at a pointer address based on a bitmask in a global variable, then calls a function pointer.
- `traverse_and_dereference`: Iterates through a sequence of bytes and returns a value loaded from a pointer-based lookup table.
- `update_global_variable`: Updates a global variable with a byte read from a memory location offset by a global variable.
- `modify_global_variable`: Modifies a global variable by checking a bit and adding a constant before reading a new byte from a memory location offset by a global variable.
- `copy_data_from_table`: Copies nineteen bytes from a location calculated from a table offset into a global variable.
- `copy_data_to_self`: Copies nineteen bytes from a fixed memory location back into a global variable.
- `copy_sprite_attribute_buffer`: copies nineteen bytes from one global buffer to another global buffer.
- `copy_sprite_attribute_data`: Copies nineteen bytes from a location calculated from a table offset into a global buffer.
- `copy_data_from_table_2`: Copies nineteen bytes from a location calculated from a table offset into a global or fixed memory location.
- `copy_sprite_attribute_data_2`: Copies nineteen bytes from a location calculated from a table offset into a global buffer.
- `copy_sprite_attribute_data_3`: Copies nineteen bytes from a location calculated from a table offset into a global buffer.
- `copy_sprite_attribute_data_4`: Copies nineteen bytes from a location calculated from a table offset into a global buffer.
- `copy_sprite_attribute_data_5`: Copies nineteen bytes from a location calculated from a table offset into a global buffer.
- `initialize_and_shuffle_table`: Fills a table with sequential values, performs a series of swaps based on scaled values from another routine, and then populates a second table with values or specific constants based on comparisons.
- `update_high_score_table`: Iterates through a list of entries to update a running total and format numeric values into ASCII characters.
- `calculate_max_values`: Iterates through a table to find and store maximum values and their indices into two separate tables.
- `parse_data_table_to_string`: Iterates through a data table to identify specific marker values and copies corresponding byte sequences into a destination buffer.
- `convert_number_to_string`: Calculates the hundreds and tens digits of a value and converts them into a character sequence.
- `check_global_conditions`: Compares multiple global variables against specific constant values and calls another routine if conditions are met.
- `update_global_byte`: Stores a byte from a calculated memory address into a global variable.
- `format_telemetry_strings`: Converts numerical values from global variables into ASCII character strings for display.
- `copy_word_array`: Copies a series of words from a source table to a destination table using a loop.
- `fill_table_with_data`: Stores a sequence of incrementing 32-bit values into a destination table.
- `fill_table_with_increments`: Populates a table with 16-bit values by incrementing a counter by a fixed amount in each iteration.
- `perform_bit_manipulation_math`: Iterates through a table to perform bitwise operations and arithmetic on each word, storing the results back into a table.
- `call_subroutine_and_return`: This routine calls another routine and then returns.
- `invoke_subroutine`: calls another routine and then returns.
- `expand_nibbles_to_arrays`: Calls a routine that unpacks nibbles from a source buffer into byte arrays using a lookup table.
- `execute_triple_subroutine_sequence`: Executes three consecutive subroutines and then performs a multiplication of two word values to update global variables before executing an indirect jump.
- `run_secondary_sequence`: Performs two subroutine calls in succession.
- `invoke_third_sequence`: Calls two other routines sequentially.
- `process_fourth_sequence`: Executes a sequence of two subroutine calls.
- `trigger_fifth_sequence`: Runs two subroutins one after another.
- `perform_sixth_sequence`: Calls two subroutines in a row.
- `execute_three_subroutines`: Calls three consecutive subroutines and then performs a product calculation and an indirect jump.
- `execute_three_subroutines_v2`: Triggers three subroutines in sequence before calculating a product and jumping to a new address.
- `copy_memory_blocks`: Copies a block of data from a source to a destination, followed by a second block copy from a different source to a different destination.
- `execute_sequential_subroutines`: Calls three consecutive subroutines in sequence and then performs a data expansion task by unpacking nibbles from a buffer to index into a table.
- `execute_three_subroutines_v3`: Executes three subroutines and then calculates a product to perform an indirect jump.
- `execute_two_subroutines`: Calls two subroutines located at specific addresses.
- `nop_sequence_execution`: The routine performs no operations and immediately returns to the caller.
- `execute_two_subroutines_2`: Performs two consecutive subroutine calls.
- `copy_and_modify_data`: Copies large blocks of data from one location to another and then performs a secondary pass to modify specific bytes within that data.
- `update_memory_table`: Iterates through a table to store calculated values into a memory range, using a loop counter and a termination condition based on a global variable.
- `execute_subroutine_sequence`: Performs a sequence of no-operation instructions and calls another routine.
- `shift_and_clear_data`: Copies a block of data to a new location, clears a range of memory, and performs a series of data transfers involving multiple subroutine calls.
- `insert_value_into_list`: Checks if a value falls within a specific range and, if so, attempts to insert it into a list of up to sixteen entries.
- `call_subroutine`: Calls another routine and then returns.
- `execute_recursive_call`: The routine calls itself and then returns.
- `multiply_and_branch`: Multiplies several values and uses a result to calculate a jump target for a function call.
- `compute_bit_shifted_values_2`: Calculates bit-shifted or bit-rotated values and stores them in a destination, using a loop that checks a global variable and potentially calls another routine via a pointer.
- `execute_indirect_jump`: Compares two values and, if they are not equal, performs an indirect jump to a function selected from a table via a scaled index.
- `compute_bit_shifted_values`: Calculates a series of bit-shifted or bit-rotated values using a sequence of left-shifts and carry-bit propagation, then stores the resulting bytes into a destination buffer offset by a global variable.
- `loop_shift_with_decrement`: Repeatedly shifts a register left and, if it becomes zero, updates a pointer and loads a new value from a table.
- `looped_shift_with_table_lookup`: The routine repeatedly shifts an input value, using a table pointer when the value becomes zero, decrementing a short counter derived from an unable to be determined input register until it reaches minus one.
- `traverse_hierarchical_tree_structure`: Iteratively descends through a tree or linked structure by checking for zero values and moving to a parent or child node using a series of conditional checks.
- `unpack_nibbles_and_expand_table`: Unpacks nibbles from a source buffer into a series of byte arrays and uses those values to index into a table for generating an expanded data set.
- `clear_memory_range`: Clears a range of memory by setting several global variables to zero and then zeroing out a block of memory using a pointer and a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_global_values`: Performs a series of additions using several global variables and constant values.
- `decode_joystick_direction_bits`: Reads a value from a memory location, decodes certain bits using an XOR-shift technique, and stores the result in a global variable, while also performing certain arithmetic on a pointer-based value.
- `execute_single_instruction`: This routine consists of a single instruction and is called by one unnamed routine.
- `move_register_to_memory`: stores a single register value into a memory location using an absolute move instruction.
- `modify_memory_via_move`: This routine contains thirteen instructions and is called by two unnamed routines using move and clear instructions.
