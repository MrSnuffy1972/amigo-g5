# Game 190 (KarateKid2 v1.1): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00190). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1703200 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 180 of 180 named (0 author symbols), 180 of 180 described (5 by rule, 174 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2159 of 2159 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5793 of 5793 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 299999 |
| code | 20710 |
| data | 1210915 |
| padding | 56 |
| unused | 171520 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 52 | 5 |
| logged disk load from Disk.1 | 1 | 5614 | 159 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 101 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 4 |

## Cases handled

- KarateKid2_v1.1/KarateKid2_v1.1_hdf/KarateKid2/Disk.1: raw disk image (348160 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- KarateKid2_v1.1/KarateKid2_v1.1_hdf/KarateKid2/Disk.2: raw disk image (404480 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- KarateKid2_v1.1/KarateKid2_v1.1.hdf: hard-disk image (AmigaDOS OFS, 1824 blocks): its filesystem parses and all 6 files in it are byte-identical to the extracted tree KarateKid2_v1.1_hdf, which is recovered file by file; no block outside the filesystem holds data
- 8 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- KarateKid2_v1.1/KarateKid2_v1.1_hdf/KarateKid2/Disk.1: 6 logged loads cover 281600 bytes of the image
- KarateKid2_v1.1/KarateKid2_v1.1_hdf/KarateKid2/Disk.2: 2 logged loads cover 30720 bytes of the image
- KarateKid2_v1.1/KarateKid2_v1.1_hdf/KarateKid2/Disk.2: chunks 0-104 (bytes 0-26880) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.15)
- KarateKid2_v1.1/KarateKid2_v1.1_hdf/KarateKid2/Disk.2: chunks 240-344 (bytes 61440-88320) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.86)
- KarateKid2_v1.1/KarateKid2_v1.1_hdf/KarateKid2/Disk.2: chunks 360-464 (bytes 92160-119040) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.42)
- KarateKid2_v1.1/KarateKid2_v1.1_hdf/KarateKid2/Disk.2: chunks 480-584 (bytes 122880-149760) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.41)
- KarateKid2_v1.1/KarateKid2_v1.1_hdf/KarateKid2/Disk.2: chunks 860-964 (bytes 220160-247040) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.37)
- KarateKid2_v1.1/KarateKid2_v1.1_hdf/KarateKid2/Disk.2: chunks 980-1084 (bytes 250880-277760) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.60)
- KarateKid2_v1.1/KarateKid2_v1.1_hdf/KarateKid2/Disk.2: chunks 1100-1204 (bytes 281600-308480) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.54)
- KarateKid2_v1.1/KarateKid2_v1.1_hdf/KarateKid2/Disk.2: chunks 1220-1324 (bytes 312320-339200) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.18)
- KarateKid2_v1.1/KarateKid2_v1.1_hdf/KarateKid2/Disk.2: chunks 1340-1444 (bytes 343040-369920) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.08)
- KarateKid2_v1.1/KarateKid2_v1.1_hdf/KarateKid2/Disk.2: chunks 1460-1564 (bytes 373760-400640) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.84)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `configure_dma_and_copy_data` (Disk.1 +an address loaded at an address (30720 bytes)): Configures disk DMA registers and performs a bit-manipulation loop to transform data in memory. Kind: disk track/sync check and data decryption. Pass path: the loop continues to load subsequent tracks/data via the u1__l0_0B30 loop.
- `perform_timed_loop` (Disk.1 +an address loaded at an address (30720 bytes)): Modifies a global bitmask and executes a loop that runs a specific number of times while waiting for a CIA hardware flag cycle. Kind: disk hardware/timing check. Pass path: the loop completes normally (sVar1 reaches 0).
- `clear_flag_and_zero_word` (Disk.1 +an address loaded at an address (30720 bytes)): Repeatedly modifies a global variable based on a bitmask check and then stores zero at an offset from a pointer. Kind: checksum or state trap. Pass path: bra.w an address.
- `compare_and_increment_values` (Disk.1 +an address loaded at an address (30720 bytes)): Compares two word values at specific offsets and modifies a global variable bitmask while incrementing one of the values. Kind: disk hardware/timing check. Pass path: the loop continues (the break condition is not met).

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `execute_indirect_calls`: uses a pointer passed in a register to call several functions via offsets and performs conditional branching based on a return value.
- `call_external_function`: Calls a function through a pointer stored in a global table and returns the result.
- `wait_for_bit_change`: Polls a global variable until specific bits change state, checking against a condition involving a hardware register.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.

### logged disk load from Disk.1

- `initialize_game_state_and_loop`: Sets various global variables and hardware color registers to prepare the game state, then iterates through a loop to process data and display it.
- `render_intro_sequence`: Sets various global variables and hardware color registers to prepare the game state, then iterates through a loop to process data and display text.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `clear_string_byte`: Loops through a memory location until a null byte is found, then writes a zero to that position.
- `shift_color_data`: Iterates through a source list of values, shifts each value left by one bit, and stores the result in a destination table.
- `increment_global_value`: Adds six to a word stored at an offset from a pointer.
- `decrement_counter`: Decrements a value by one until it reaches zero.
- `iterate_null_terminated_string`: Iterates through a null-terminated string and calls another routine for each character.
- `render_tile_data`: Processes character data to populate four separate memory buffers based on bitmask flags and a lookup table.
- `convert_integer_to_ascii`: Converts an integer into a decimal string by incrementing ASCII characters in a memory buffer and replaces leading zeros with spaces.
- `toggle_memory_value`: Saves a word from a memory location, overwrites it with a constant, calls another routine, and then restores the original value.
- `format_score_string`: Converts a numerical value into a decimal ASCII string by iteratively subtracting powers of ten and incrementing character offsets.
- `convert_integer_to_string`: Decomposes an integer into its decimal digits and stores them as ASCII characters in a series of memory offsets.
- `update_color_hardware`: Updates color registers and DMA/interrupt settings while performing bitwise rotations on data and storing results in memory.
- `rotate_and_update_colors`: Performs bitwise rotations on values and updates two word-sized memory locations while modifying color hardware registers.
- `rotate_and_store_colors`: Performs bitwise rotations on values and updates two word-sized memory locations while writing to color hardware registers.
- `rotate_and_store_colors_2`: Performs bitwise rotations on values and updates two word-sized memory locations.
- `call_update_color_registers`: This routine consists of two instructions that call another routine.
- `rotate_and_update_colors_2`: Performs bitwise rotations on data and updates two word-sized memory locations while modifying color hardware registers.
- `rotate_and_store_colors_3`: Performs bitwise rotations on values and updates two word-sized memory locations.
- `rotate_and_store_colors_4`: Performs bitwise rotations on values and updates two word-sized memory locations.
- `rotate_and_store_colors_5`: Performs bitwise rotations on values and updates two word-sized memory locations.
- `fetch_color_entry_pointer`: A two-instruction routine whose address is taken via a PC-relative load-effective-address and which is called only by update_color_hardware, so it appears to supply a pointer or reference used by that caller.
- `update_color_registers`: Performs bitwise rotations on values and updates two word-sized memory locations while touching color hardware registers.
- `process_loop_and_update_values`: Executes a loop eight times performing a bitwise rotation on a value, then updates two word values at specific offsets from a pointer.
- `process_input_and_update_state`: Checks a bitmask in a global variable and, if a specific value is detected, performs a series of state resets and updates multiple memory locations.
- `calculate_pseudo_random`: Clears a bit in a global variable and performs a series of repeated multiplications to generate a value.
- `update_sprite_attributes`: Updates a block of memory with bit-manipulated values derived from a global variable and increments a counter.
- `set_global_value_small`: Stores a constant value into a global variable.
- `set_global_constant`: stores a constant value into a global variable.
- `set_global_value`: assigns a constant word value to a global variable.
- `setup_dma_and_blitter`: Configures several memory locations, sets up DMA and Blitter registers, and waits for a specific bit in a global variable to be set.
- `store_word_to_ram`: Stores a word into two consecutive memory locations.
- `execute_loop_until_zero`: Repeatedly calls another routine until a specific return value becomes zero.
- `iterate_table_backwards`: Iterates through a table of eight elements in reverse order, calling another routine based on whether the current element is zero.
- `search_non_zero_string_elements`: Iterates through a sequence of bytes, calling another routine until a non-zero byte is encountered or a counter reaches zero.
- `modify_table_and_memory`: Sets a table entry to zero and writes two word-sized values to memory locations calculated from a pointer and an index.
- `write_word_to_table`: Writes a word to a table using a base pointer and an index multiplied by eight.
- `write_byte_to_table`: Stores a byte into a table at an index determined by a value in a register.
- `configure_dma_and_copy_data`: Configures disk DMA registers and performs a bit-manipulation loop to transform data in memory.
- `perform_timed_loop`: Modifies a global bitmask and executes a loop that runs a specific number of times while waiting for a CIA hardware flag cycle.
- `clear_flag_and_zero_word`: Repeatedly modifies a global variable based on a bitmask check and then stores zero at an offset from a pointer.
- `compare_and_increment_values`: Compares two word values at specific offsets and modifies a global variable bitmask while incrementing one of the values.
- `wait_status_bit_then_mark`: Busy-waits until a status bit is set, then writes the value two into the same status word.
- `wait_for_cia_flag_cycle`: Busy-waits until a CIA interrupt status bit clears and then until it sets again, returning after one full cycle of that flag.
- `initialize_name_input`: Checks a value in a structure and, if below a threshold, populates several fields within that structure with specific constants.
- `clear_memory_block`: Clears a series of nine double-words to zero starting from a specific offset in a structure.
- `initialize_display_parameters`: Sets multiple word and long values in memory, performs string processing, and executes loops to increment values in a buffer.
- `copy_and_update_table`: Copies a block of memory, updates a value at a calculated destination, and then copies a range of data from a global table to another location.
- `copy_memory_block`: Calculates a memory offset and copies nineteen bytes from a global variable to a destination pointer.
- `determine_index_threshold`: Compares a value against a series of thresholds in a structure and stores the resulting index into a global variable.
- `process_input_buffer_loop`: Repeatedly checks a character at a memory offset until a null terminator is found.
- `decrement_and_store_space`: Decrements a value in memory, stores a space character into a global array and another memory location, and copies a word between offsets.
- `store_string_character`: Stores a byte into a global array at an index tracked by a variable, increments that index, and copies a word from one memory location to another.
- `clear_memory_buffers`: Resets two memory offsets to zero and fills several consecutive memory locations with space characters.
- `lookup_masked_value`: Masks a bit from an input, then uses the result to fetch a value from a table if the result is within a specific range.
- `execute_empty_loop`: Performs a simple loop that decrements a counter from fifteen to zero.
- `initialize_game_state_and_display_text`: Sets various global variables and hardware color registers to prepare the game state, then iterates through a loop to process data and display specific text strings.
- `update_game_state_and_loop`: Updates various state variables and counters based on conditional checks of memory offsets, modifies specific values in a data structure, and enters a loop that increments counters and performs repeated calls to another routine.
- `initialize_memory_buffers_2`: Sets a pointer to a global variable, assigns a memory address, and clears several large blocks of memory using a loop.
- `increment_conditional_value`: Increments a 16-bit value stored at an offset from a pointer if a specific byte at another offset is zero.
- `setup_dma_transfer_start`: Populates a data structure with specific parameters and a pointer to a starting address, then modifies a global bitmask.
- `setup_dma_transfer_fixed`: Sets several fields in a data structure using constant values and a fixed memory address before updating a global bitmask.
- `setup_dma_transfer_variable`: Writes specific values into a data structure, including a pointer to a starting address, and modifies a global bitmask.
- `setup_dma_transfer_offset`: Fills a data structure with specific parameters and a memory address, then updates a global bitmask.
- `prepare_dma_transfer`: Sets several values in a data structure and modifies a global bitmask.
- `apply_bitmask_to_global`: Calls another routine and then performs a bitwise operation to update a global variable.
- `execute_masked_update`: Calls two other routines and then applies a bitwise mask to a global variable.
- `calculate_dma_parameters`: Performs conditional logic on values within a structure to calculate a new value, then populates a data structure and modifies a global bitmask.
- `calculate_dma_offset`: Calculates a value based on a field in a structure and uses it to populate a data structure before modifying a global bitmask.
- `initialize_text_buffer`: Sets several word values at specific offsets, iterates through a null-terminated string, and assigns a value to a global variable based on a comparison.
- `update_bonus_display`: Updates coordinate offsets and increments counters based on character values and bitmask checks, while managing string-related data.
- `update_game_state_and_counters`: Checks multiple conditions to update various word and byte values at specific offsets, including modifying a memory address and adjusting counters.
- `decrement_and_check_counter`: Decrements a global variable and, if it reaches zero alongside another condition, initializes several memory offsets via another routine.
- `process_state_machine_transitions`: Evaluates a state variable to execute specific data distribution loops, toggle bitmasks, or update memory offsets.
- `distribute_interleaved_data_2`: Copies data from a source buffer into four separate destination buffers at large offsets using a nested loop structure.
- `initialize_game_state_and_buffers`: Resets a large block of global state variables to zero or specific constant values and clears multiple memory buffers using loops.
- `clear_data_fields`: Clears several word-sized memory locations and sets specific offsets to zero or one within a data structure.
- `update_entity_position_and_state`: Calculates and clamps entity coordinate offsets within defined boundaries and updates internal timers or state flags.
- `update_entity_animation_and_velocity`: Updates an entity's animation frame index and retrieves corresponding sprite data from a table.
- `process_data_sequence`: Decrements a counter and updates memory values from a pointer-based table, or calculates a value based on several conditional bitmask checks.
- `update_animation_frame_and_velocity`: Updates an entity's animation frame index and retrieves corresponding sprite data from a table.
- `mask_sign_bit`: Clears the most significant bit of a word and returns the result.
- `update_entity_state_and_logic`: Manages an entity state machine by decrementing timers, processing sequential data from a table, and calculating movement or status offsets based on various flag bits.
- `test_data_condition`: Calls a routine to check if a word at a specific offset in a data structure is zero.
- `update_sprite_buffer_and_mask`: Decrements internal timers and updates a memory buffer by copying data from a source table using calculated offsets.
- `update_entity_state_and_buffer`: Updates an entity's internal timer and state based on conditional checks and data comparisons.
- `update_conditional_state`: Updates a field in a data structure based on comparisons between two word values and a signed short value, then calls another routine multiple times.
- `update_state_table`: Evaluates several conditions based on values in a data structure and updates a specific field and a table entry accordingly.
- `shift_table_values`: Iterates through sixteen word-sized elements in a table and shifts each value left by one bit.
- `update_game_state_parameters`: Stores several word-sized values into specific offsets of a data structure and performs multiple repeated calls to another routine.
- `initialize_interleaved_graphics_buffers`: Populates multiple memory areas by copying data from a source table using an interleaved pattern.
- `copy_interleaved_data`: Copies data from a source table to two separate destination memory areas using an interleaved pattern.
- `initialize_memory_buffers`: Clears two memory buffers and then fills specific segments of them with constant byte values based on counts stored in a global structure.
- `configure_game_over_state`: Sets various hardware and memory parameters, including audio channel settings and buffer values, before checking a condition to display text strings.
- `initialize_memory_buffers_3`: Copies data from a source to multiple destination buffers using interleaved and block transfer loops.
- `bulk_copy_longwords`: Copies a large block of data from one memory location to another using 32-bit word transfers.
- `bulk_copy_memory_blocks`: Copies a large block of data from one memory location to another using a loop of longword transfers.
- `copy_large_data_block`: Performs a massive block transfer of data from a source to a destination using nested loops and 32-bit increments.
- `distribute_interleaved_data`: Takes data from a source and distributes its components across four different memory offsets in a destination buffer.
- `manage_state_and_delay_loops`: Resets several fields in a data structure and enters a loop that performs conditional checks and decrements a counter to create timing delays.
- `clear_status_flag`: Checks two consecutive bytes in a data structure and clears a word at an offset from a pointer.
- `reset_value_on_char_match`: Sets a 16-bit value to zero if a specific character at a given offset matches a certain symbol.
- `set_active_memory_pointer`: Selects between two memory addresses based on a value and updates a pointer and a global variable accordingly.
- `wait_for_bit_and_store`: Loops until a specific bit in a memory location is set, then stores a constant word value into a memory location.
- `update_state_logic`: Evaluates a data structure to select and store various state indices and values based on multiple conditional checks.
- `update_state_index_and_value`: Evaluates multiple conditions based on values within a data structure to select an index.
- `update_state_data`: Validates two values within a data structure before performing a series of conditional checks on a word.
- `process_input_and_clear_memory`: Polls a status flag in a data structure and clears large blocks of memory at specific locations using nested loops.
- `update_color_and_clear_buffers`: Updates a color value and a status byte based on internal state flags and decrements a counter.
- `update_game_state_and_render_tiles`: Updates internal counters and timers based on specific state flags, then iterates through a data table to calculate memory offsets for rendering tile data into buffers.
- `copy_data_block`: Copies a block of data from a source table to a destination calculated from a global pointer and an offset.
- `copy_conditional_data`: Copies data from a source table to a destination based on a value in a global variable.
- `render_tile_data_from_table`: Reads data from a table to calculate memory offsets and performs a nested loop to copy blocks of data into a destination buffer.
- `apply_bitmask_to_buffer_2`: Iterates through a nested loop structure to perform bitwise AND and OR operations on a target memory buffer using values from a source table.
- `update_multiple_buffers`: Performs two consecutive loops that copy data from a source table to two different destination addresses.
- `update_sprite_graphics`: Calculates a memory offset based on sprite dimensions and copies sprite data from a table into a destination buffer.
- `shift_bitstream_left_block`: Performs a multi-pass left bitwise shift on a block of memory by propagating the most significant bit of each word into the least significant bit of the preceding word.
- `rotate_bitstream_right`: Performs a circular right shift on a series of words within a memory buffer.
- `shift_bitstream_left`: Performs a multi-pass left bitwise shift on a block of memory.
- `copy_data_and_bitwise_mask`: Copies a block of data from a calculated table offset into a destination buffer.
- `copy_sprite_data_blocks`: Copies multiple blocks of structured data from a source table to a destination buffer using two sequential loops.
- `blit_quad_sprite_tiles`: Performs a bitwise composition of four source data blocks into a destination buffer using a series of nested loops.
- `copy_data_blocks`: Copies six blocks of data from a source table to a destination buffer using a loop.
- `apply_bitmask_to_buffer`: Performs bitwise operations on a destination buffer by combining its existing contents with values from a source table.
- `deinterleave_planar_graphics_data`: Reads data from a source buffer and distributes its components into four separate destination memory offsets.
- `apply_bitmask_to_memory`: This routine performs bitwise logical operations to update specific memory locations using values from a source table.
- `initialize_and_update_bitmapped_buffer`: Clears a memory buffer and then populates it by copying data from a source table using specific offsets.
- `update_bitmapped_data`: Performs bitwise operations to merge data from a source table into a destination buffer at specific offsets.
- `configure_audio_channels`: Sets the period, volume, and channel parameters for multiple audio channels and enables DMA.
- `configure_audio_channels_2`: Updates several hardware audio registers by calculating period values from a global variable and loading word-sized values into memory.
- `set_table_bit_one`: Fetches a value from a table using a four-bit index and stores the result with the first bit set into a table location.
- `clear_table_bit_zero`: Fetches a value from a table using a four-bit index and stores the result with the first bit cleared into a table location.
- `set_table_bit_two`: Fetches a value from a table using a four-bit index and stores the result with the second bit set into a table location.
- `clear_table_bit_one`: Fetches a value from a table using a four-bit index and stores the result with the second bit cleared into a table location.
- `set_table_bit_three`: Fetches a value from a table using a four-bit index and stores the result with the third bit set into a table location.
- `store_byte_indirect`: Stores a byte from a register into a memory location indexed by a byte from another register.
- `fetch_table_entry`: Uses the lower four bits of a word in a register as an index to return a value from a table.
- `shift_and_store_byte`: Retrieves a value and stores the result of a four-bit right shift into a memory location indexed by a byte from another register.
- `clear_table_bit_three`: Fetches a value from a table using a four-bit index and stores the result with the third bit cleared into a table location.
- `clear_table_bit_four`: Fetches a value from a table using a four-bit index and stores the result with the fifth bit cleared into a table location.
- `clear_memory_block_2`: Clears four consecutive word-sized memory locations using a pointer stored in a register.
- `clear_and_loop`: Clears four consecutive word-sized memory locations using a pointer and enters a loop that continues until a specific value is zero.
- `configure_audio_hardware`: Sets a global value and modifies audio channel length and level registers.
- `update_audio_and_data`: Checks a bitmask in a data structure and a byte value to determine whether to clear several word-sized memory locations or perform a series of calls.
- `check_data_condition`: Checks if a word at a specific offset in a data structure is zero.
- `modify_table_entry`: Checks two values in a data structure and, if both are zero, stores a modified byte into a table using a four-bit index.
- `store_conditional_data`: Checks a value in a data structure and, if zero, stores a long word and a word into memory offsets.
- `update_global_data`: Updates a long word and a word at specific offsets from a pointer based on conditional checks.
- `update_conditional_data`: Checks a value at an offset from a pointer and either updates several global variables or stores a long word and a word into memory locations.
- `setup_audio_dma`: Calculates a value based on a table lookup and configures audio channel period, volume, and DMA settings.
- `initialize_and_clear_data`: Clears several memory locations in a loop, performs a series of writes to a data structure, and modifies a table entry.
- `process_linked_list_element`: Decrements a counter in a structure and performs recursive calls or updates a table entry based on specific data conditions.
- `process_recursive_linked_list`: Decrements a counter in a structure and recursively traverses a linked list of data elements.
- `store_indexed_table_value`: Checks a value at an offset from a pointer and stores a four-bit value from a table into one of three possible table indices.
- `initialize_table_entries`: Stores a byte and a word into specific offsets relative to a pointer obtained from another routine.
- `store_conditional_code`: Checks a value at an offset from a pointer against several constants and stores a code value into a table at a specific index if conditions are met.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `increment_and_fetch_table_value`: Increments a counter and retrieves a value from a table based on a conditional selection of a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_interrupts`: Sets several global variables to constant values, processes tile data, and configures CIA and interrupt enable registers.
- `calculate_scaled_value`: clears a bit in a global variable and performs a series of repeated multiplications by a constant on the result.
- `update_structured_data`: checks a bit in a global variable and, if set, performs multiple writes to a data structure using bitwise manipulations of that global variable.
- `set_global_value_2`: Assigns a constant value to a global variable.
- `set_interrupt_request_mask`: stores a constant value into a global variable and modifies the interrupt request register.
- `set_global_value_3`: Sets a global variable to a specific constant value.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `nop_operation`: This routine is empty and performs no operations.
- `return_immediately_4`: the routine is empty and performs no operations before returning.
- `initialize_data_block`: calls another routine and then writes several constant values to various offsets within a data structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_audio_parameters`: Extracts values from a data structure to set the period and volume for multiple audio channels.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately`: This routine is empty and performs no operations before returning to the caller.
- `return_immediately_2`: This routine is empty and returns control to the caller without performing any operations.
- `return_immediately_3`: This routine is empty and performs no operations before returning to its caller.
- `return_void`: The routine is empty and performs no operations before returning.
