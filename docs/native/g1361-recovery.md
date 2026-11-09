# Game 1361 (blues brothers, the (1991)(titus)[cr company]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01361). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 1 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 294 of 294 named (0 author symbols), 294 of 294 described (56 by rule, 236 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1809 of 1809 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 12612 of 16062 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 71229 |
| code | 61178 |
| data | 705729 |
| padding | 8 |
| unused | 62976 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Blues Brothers, The (1991)(Titus)[cr Company].adf:bootblock | 1 | 18 | 1 |
| logged disk load from Blues Brothers, The (1991)(Titus)[cr Company].adf boot-block load | 1 | 447 | 14 |
| logged disk load from Blues Brothers, The (1991)(Titus)[cr Company].adf boot-block load | 1 | 15597 | 279 |

## Cases handled

- blues brothers, the (1991)(titus)[cr company]/Blues Brothers, The (1991)(Titus)[cr Company].adf: custom-format (non-DOS) disk, 1570 non-empty blocks: matched against track-loaded segments
- blues brothers, the (1991)(titus)[cr company]/Blues Brothers, The (1991)(Titus)[cr Company].adf: the boot block reads 47616 bytes from disk offset 805888 to an address (found statically); recovered as a unit
- blues brothers, the (1991)(titus)[cr company]/Blues Brothers, The (1991)(Titus)[cr Company].adf: the boot block reads 98304 bytes from disk offset 707584 to an address (found statically); recovered as a unit
- blues brothers, the (1991)(titus)[cr company]/Blues Brothers, The (1991)(Titus)[cr Company].adf: 2 logged loads cover 145920 bytes of the image
- blues brothers, the (1991)(titus)[cr company]/Blues Brothers, The (1991)(Titus)[cr Company].adf: blocks 2-4 (bytes 1024-2560) outside the filesystem hold data, not code (opcode density 0.018, entropy 6.73)
- blues brothers, the (1991)(titus)[cr company]/Blues Brothers, The (1991)(Titus)[cr Company].adf: blocks 6-7 (bytes 3072-4096) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.77)
- blues brothers, the (1991)(titus)[cr company]/Blues Brothers, The (1991)(Titus)[cr Company].adf: blocks 9-12 (bytes 4608-6656) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.77)
- blues brothers, the (1991)(titus)[cr company]/Blues Brothers, The (1991)(Titus)[cr Company].adf: blocks 16-531 (bytes 8192-272384) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.41)
- blues brothers, the (1991)(titus)[cr company]/Blues Brothers, The (1991)(Titus)[cr Company].adf: blocks 538-786 (bytes 275456-402944) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.88)
- blues brothers, the (1991)(titus)[cr company]/Blues Brothers, The (1991)(Titus)[cr Company].adf: blocks 790-870 (bytes 404480-445952) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.80)
- blues brothers, the (1991)(titus)[cr company]/Blues Brothers, The (1991)(Titus)[cr Company].adf: blocks 874-902 (bytes 447488-462336) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.01)
- blues brothers, the (1991)(titus)[cr company]/Blues Brothers, The (1991)(Titus)[cr Company].adf: blocks 904-904 (bytes 462848-463360) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.59)
- blues brothers, the (1991)(titus)[cr company]/Blues Brothers, The (1991)(Titus)[cr Company].adf: blocks 906-966 (bytes 463872-495104) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.95)
- blues brothers, the (1991)(titus)[cr company]/Blues Brothers, The (1991)(Titus)[cr Company].adf: blocks 968-968 (bytes 495616-496128) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.18)
- blues brothers, the (1991)(titus)[cr company]/Blues Brothers, The (1991)(Titus)[cr Company].adf: blocks 970-970 (bytes 496640-497152) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.38)
- blues brothers, the (1991)(titus)[cr company]/Blues Brothers, The (1991)(Titus)[cr Company].adf: blocks 972-1158 (bytes 497664-593408) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.67)
- blues brothers, the (1991)(titus)[cr company]/Blues Brothers, The (1991)(Titus)[cr Company].adf: blocks 1160-1162 (bytes 593920-595456) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.94)
- blues brothers, the (1991)(titus)[cr company]/Blues Brothers, The (1991)(Titus)[cr Company].adf: blocks 1164-1164 (bytes 595968-596480) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.30)
- blues brothers, the (1991)(titus)[cr company]/Blues Brothers, The (1991)(Titus)[cr Company].adf: blocks 1166-1232 (bytes 596992-631296) outside the filesystem hold data, not code (opcode density 0.002, entropy 1.86)
- blues brothers, the (1991)(titus)[cr company]/Blues Brothers, The (1991)(Titus)[cr Company].adf: blocks 1234-1238 (bytes 631808-634368) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.77)
- blues brothers, the (1991)(titus)[cr company]/Blues Brothers, The (1991)(Titus)[cr Company].adf: blocks 1240-1268 (bytes 634880-649728) outside the filesystem hold data, not code (opcode density 0.005, entropy 2.18)
- blues brothers, the (1991)(titus)[cr company]/Blues Brothers, The (1991)(Titus)[cr Company].adf: blocks 1270-1270 (bytes 650240-650752) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.55)
- blues brothers, the (1991)(titus)[cr company]/Blues Brothers, The (1991)(Titus)[cr Company].adf: blocks 1272-1272 (bytes 651264-651776) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.57)
- blues brothers, the (1991)(titus)[cr company]/Blues Brothers, The (1991)(Titus)[cr Company].adf: blocks 1274-1381 (bytes 652288-707584) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.43)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `store_byte_into_global` (Blues Brothers, The (1991)(Titus)[cr Company].adf boot-block load +an address to an address (98304 bytes)): Stores two fixed byte values into the CIA register at offset a fixed address. Kind: checksum trap / integrity check. Pass path: the code continues execution normally if the byte at the target address matches the expected value.
- `write_double_byte_to_global` (Blues Brothers, The (1991)(Titus)[cr Company].adf boot-block load +an address to an address (98304 bytes)): Writes the value 247 to a global variable after first writing the value 255 to the same location. Kind: disk hardware/checksum trap. Pass path: sub_E164.
- `read_disk_track_sectors` (Blues Brothers, The (1991)(Titus)[cr Company].adf boot-block load +an address to an address (98304 bytes)): Computes a track position from registers, sets disk DMA parameters in the custom chip, and enters a loop that copies 512-byte blocks of data from a source to a destination buffer, incrementing a global byte value until a block is complete. Kind: disk/track check. Pass path: the loop continues with adda.w #an address, a2 and addq.w #an address, d0.
- `adjust_disk_head_track` (Blues Brothers, The (1991)(Titus)[cr Company].adf boot-block load +an address to an address (98304 bytes)): Modifies the CIAA Port A register and calls modify_and_delay to synchronize a global byte value with a target value derived from a register, then sets or clears bit 2 of the CIAA Port A register based on the low bit of the original input. Kind: track/head position validation. Pass path: the loop terminates when uVar1 == u2__d0_E3CC, proceeding to the bitwise manipulation of pbVar2.

## Functions


### Blues Brothers, The (1991)(Titus)[cr Company].adf:bootblock

- `initialize_boot_sequence`: Sets several values in a structure pointed to by a1, calls a library routine, calls a routine at a memory address, and then stores a pointer into three global variables.

### logged disk load from Blues Brothers, The (1991)(Titus)[cr Company].adf boot-block load

- `configure_system_trap`: Sets specific values in a structure, modifies hardware registers, and executes a trap before performing a loop that checks a value in a register and updates two memory locations.
- `initialize_graphics_and_check_input`: Clears a large block of memory, populates several tables with specific patterns and color data, and configures hardware registers for DMA and video output.
- `clear_block_and_wait_frames`: Clears a block of 456 words, sets two stored addresses and a count, then loops waiting for the beam to reach a line, calling helpers on each pass.
- `update_list_entry_count`: Increments a 16-bit value at an offset of 4732 from a pointer in a5 and uses it to calculate addresses for a loop that calls another routine 14 times.
- `set_flag_and_branch`: Sets a byte to 255 at an offset of 4731 from a pointer in a5 and then branches to another location.
- `update_list_entry_and_loop`: Writes a value to an offset of 4732 within a structure pointed to by a5, then enters a loop that calls another routine 14 times while decrementing a counter and adjusting a pointer in a0.
- `reset_state_and_branch`: Clears a byte at a specific offset within a structure pointed to by a5, calls another routine, and then branches to a previous instruction.
- `advance_colour_step_frame`: Counts down a frame counter and, when it reaches zero, moves a step index forward by two, wrapping after 456 entries.
- `update_pointer_and_branch`: Increments a pointer, compares the value at that pointer to 65535, and if not equal, branches to another location; otherwise it loads the pointer into a global variable and branches.
- `wait_for_beam_and_call`: Checks the vertical position register for a specific value and, if not met, branches; otherwise it calls another routine.
- `remap_row_colours_without_wait`: Advances the source row pointer and, for each of 160 bytes, searches a small palette list for the matching value, writing the match into a copper list entry with a fixed value of 4 beside it.
- `write_indexed_list_block_entries`: When the input value is not the end marker 640, writes two groups of words into a list record, each group taken from pointer tables at that index, together with fixed control words and an offset computed from a parameter.
- `fade_palette_towards_targets`: Counts down a frame counter, then moves each of a list of colour entries one step toward its target in each of the three 4-bit components.
- `copy_system_view_pointers`: Forbids task switching while it reads view pointers from the graphics library, copies eight pointer and word entries into the copper block, sets the wait count and frame counter, then waits for frames until the counter reaches zero.

### logged disk load from Blues Brothers, The (1991)(Titus)[cr Company].adf boot-block load

- `copy_interleaved_records`: Reads a record count and a multiplier from a structure, calculates a total number of records, and then copies four-byte records from two separate source locations into a destination by interleaving their words in a loop.
- `pack_and_interleave_table`: Reads a byte length from a global variable and packs its bytes into two separate sequences of 16-bit words by shifting each byte into the high half of a word, then interleaves these two sequences into a single output buffer by copying groups of four source bytes into six consecutive output words.
- `initialize_table_and_pointers`: Writes the value sixteen into two consecutive words of a table six times in a loop, then stores the table address and another address into two consecutive global variables.
- `load_entry_table`: Loads a block of two-word records from a source table into a destination table, copying 41 entries.
- `copy_and_interleave_words`: Copies a variable-length block of words from a source table into a global buffer, then interleaves those words into a second buffer by copying pairs of values with a two-word gap.
- `store_linked_list_pointers`: Calls a routine, then reads a count from a buffer and walks a list of linked entries, storing each pointer into a table at offset 0 of a table in a0 while advancing the pointer by a stride calculated from the entry contents.
- `store_linked_list_entries`: Calls a routine, then reads a count from a buffer and walks a list of linked entries, storing each pointer into a table at offset 0 of a table in a0 while advancing the pointer by a stride calculated from the entry contents, and finally increments a global long by 6.
- `fill_table_with_offset`: Fills a table of 256 words by writing the sum of an index times 26 and a fixed address into each slot.
- `scatter_bytes_nibbles`: Reads a source block described by a header and scatters its bytes into a destination table, splitting each nibble into separate slots.
- `pack_bytes_with_zeros`: Reads a source block described by a header and packs its bytes into a destination table, storing a zero after each inner row.
- `prepare_buffer_for_loop`: Calls a routine, loads a word from a buffer at offset -6 of a0, and branches to another routine.
- `increment_and_store_loop`: Loops over a count, storing a value into a table slot and incrementing both the value and a counter until the counter reaches a limit.
- `call_with_fixed_address`: Invokes another routine passing a fixed address as its single argument, then returns.
- `call_with_fixed_address_2`: Calls another routine passing a fixed memory address as its single argument, then returns.
- `combine_table_and_index`: Returns a word loaded from a table combined with a shifted multiple of an index.
- `run_conditioned_loop`: Selects initial values for a loop counter and step based on whether a flag is zero, then runs a loop that repeatedly calls two other routines and a third routine.
- `copy_buffer_fields`: Copies a fixed number of spaced fields from one buffer to another in a loop.
- `call_wrapper_twice`: Invokes another routine twice with the same global variable as argument and then returns.
- `process_string_list`: Calls a routine, and if the result is not zero, it enters a loop that compares strings using a routine, advances a pointer by 2 or 8, and eventually calls another routine with a pointer and a constant value.
- `expand_and_copy_strings`: Copies characters from a source to a destination, expanding negative bytes by repeating the following character.
- `expand_and_copy_strings_2`: Repeatedly invokes another routine over a range of source offsets and then advances the source pointer by a fixed amount, expanding and copying characters into a destination.
- `pack_nibbles_from_bytes`: Reads three bytes at a time and packs their nibbles into consecutive output words.
- `compare_strings`: Compares two null-terminated strings and returns one if they are equal and the second string is empty, otherwise it returns zero.
- `clear_and_or_global`: Clears a global variable ten times in a row, then calls another routine and performs a series of bitwise OR operations on the variable.
- `extract_and_pack_nibbles`: Scans nibbles of each word in a source sequence, accumulates the highest nibble into a destination word, and repeats this pass over a fixed number of words while refreshing the source.
- `copy_and_unpack_nibbles`: Copies a sequence of words into a global buffer and then reduces each nibble of every word by one when it is nonzero, packing the result back into the same words.
- `init_entity_state_tables`: Loads two table entries, calls two setup routines, and fills a 256-byte table with values from 0 to 255.
- `setup_and_copy_block`: Runs a chain of setup calls that initialize a global variable and a coprocessor, then copies an adjusted block of words from a source array into a destination.
- `run_dual_counter_loops`: Optionally calls a routine when a flag is set, then runs two fixed-count loops that increment a counter and pass it along with a global value to another routine.
- `fill_memory_with_alternating_patterns`: Fills two adjacent blocks of 16-bit words with alternating patterns of 65535 and 0, walking through each block in fixed-size chunks until both are completely written.
- `dispatch_spaced_buffer_fields`: Calculates a loop count based on a value from a stack parameter and iterates to fetch pointers from a table using bit-shifted indices.
- `copy_buffer_fields_spaced`: Copies a set of spaced fields from one buffer to another using a fixed offset pattern.
- `scatter_copy_global_source`: Copies a fixed block of 16-bit words from a global source table into a destination structure, using the same scattered offsets as the copy routine and additionally selecting between two source tables based on a flag.
- `scatter_copy_structures`: Copies a fixed block of 16-bit words from one structure to another using a set of scattered offsets, moving the first word and then the surrounding fields in a fixed pattern.
- `scatter_copy_indexed_source`: Computes a table offset from part of an argument, selects a source pointer from either a global table or a fixed address based on a flag, and copies a block of 16-bit words into a destination structure using scattered offsets.
- `scatter_copy_fixed_table`: Computes a table offset from part of an argument, then copies a block of 16-bit words from a fixed RAM table into a destination structure using scattered offsets.
- `initialize_global_pair`: Initializes a global variable by storing one value followed by another, performing only two sequential stores and returning.
- `set_globals_zero_and_neg_one`: It writes the value 0 into one global variable and then writes the value -1 into a different global variable.
- `clear_and_load_names`: Invokes a routine at a fixed address, then clears a global variable ten times while applying a series of bitwise OR operations to it.
- `initialize_and_loop_table_setup`: Initializes several global variables and stack counters, then enters a loop that repeatedly calls another routine, adjusts two index counters based on its result, writes a byte and a value of one hundred into two adjacent memory slots, and clears several blocks of a table.
- `combine_and_shift_registers`: Combines high halves of the input register and a global variable with shifted copies of the input fields, adding the results together and returning the sum.
- `spread_words_from_shifted_value`: Shifts the two halves of a passed value apart and feeds each half into another routine through a pointer, copying a block of 64-bit data and spreading its 16-bit words across a destination table in a nested loop, then writes sixteen into two consecutive words of a global table.
- `copy_table_block`: Computes a source address by adding a scaled index from a byte to a base address, then copies 64 words from that source to a destination buffer at specific, non-contiguous offsets.
- `adjust_table_pointers`: Adjusts three sets of three pointer slots in a global table by adding or subtracting a fixed increment, depending on the current values.
- `dispatch_copy_by_global_value`: Calls another routine and then dispatches to split_and_copy_array multiple times with argument pairs derived from a global variable and constant offsets, depending on the value of that global variable.
- `compare_global_word`: Compares a word at a global address to 510 and returns if they are not equal, otherwise it returns.
- `test_and_set_status_bit`: Tests a word at a specific memory location and, if it is not zero, sets a byte at offset -1 of a6 to one.
- `update_status_bits`: Clears a byte at offset -1 of a6, then checks several word values at specific offsets to set bits 2, 3, 4, 5, or 6 in that byte, and finally increments a byte at a global address if a comparison between a byte at a structure offset and a byte at a global address succeeds.
- `update_global_pointer_and_byte`: Increments a long value at a global address, moves that address to a0, clears a byte at that address, then writes a byte from a stack offset to an offset of 1 from the address in a0, increments a word at another global address, and branches to another routine.
- `load_and_mask_global_byte`: Loads a byte from a global variable, advances the pointer by two, then stores a second byte, and finally masks the value with several single-bit constants.
- `increment_and_wrap_global_byte`: Compares a global variable against a value in a fixed table, increments it and wraps it to one when it reaches five, otherwise clears it.
- `update_entity_states`: Iterates over a table of entity structures, updating their coordinate fields and status flags, then dispatches each entity to a state handler that advances its position and selects its next behavior.
- `update_indexed_structure`: Calls another routine, uses the result to locate a table, and if a comparison between a byte at offset 102 of a structure and a byte from the table succeeds, it either writes values to offsets 54, 22, and 23 or resets several fields including offsets 66, 36, 100, 2c, 23, and 22.
- `copy_indexed_bytes`: Reads a byte from offset 102 of a structure, increments it, and uses it as an index into a table pointed to by a4 to store a byte at offset 34 of the structure; then repeats this by incrementing the original byte by 2 to store a byte at offset 35.
- `clear_structure_fields`: Clears several byte and word fields at fixed offsets within a structure pointed to by a register.
- `initialize_structure_fields`: Initializes several fields of a structure, conditionally clears one based on a comparison, and toggles a bit in an existing field.
- `decode_and_build_coordinates`: Decodes a packed value from a global variable by repeatedly extracting nibbles and subtracting table offsets, then builds a small data structure with two coordinates derived from lookup tables.
- `invoke_pointer_from_indexed_table`: Combines two structure fields into a single value and passes it to another routine, then reads a pointer from a global table using a computed index and invokes the routine stored there with the structure.
- `load_byte_from_indexed_table`: Computes a table index from a parameter, reads a byte from a global variable, multiplies it by a fixed stride, and loads a word from a computed address in a data table, returning the low byte of that word.
- `load_byte_from_struct_indexed_table`: Reads two fields from a structure passed as a parameter, combines them into a table index, and loads a word from a computed address in a data table, returning the low byte of that word.
- `update_text_struct`: Updates a text structure by decoding a string field and applying per-character effects, recursing into sibling entries when needed.
- `process_structure_fields`: Reads a byte from a structure and, if it is below ten, loops a number of times proportional to that byte while calling another routine.
- `copy_tile_block`: Writes a byte into a global table and, if certain coordinate bounds checks pass, copies a 64-word block of data from a source location to a destination location using a fixed offset pattern.
- `read_table_byte_add_two`: Indexes into a table using a value taken from a structure and a masked field, then reads the byte stored there and adds two to it.
- `read_table_byte_at_offset`: Indexes into a table using a shifted byte and a high half of the argument, then reads the byte stored at a fixed offset from that entry.
- `copy_transformed_table_block`: Walks a linked list of node structures, following a chain of child pointers and recording a computed value into a field.
- `copy_and_merge_data_blocks`: Iterates over a table of source data and destination offsets, copying and bitwise-merging word-sized blocks into scattered locations in a large output buffer.
- `clip_and_fill_table`: Clamps two coordinate pairs into a fixed range and writes the resulting rectangle corners into a table, producing a set of clipped entries.
- `clear_byte_at_offset`: Clears a byte at offset 46 of a structure in a6, loads a byte from offset 46 of the same structure into d0, sign-extends d0 to a long, and branches to another location.
- `push_and_call_routine_2`: Pushes the value in a3 onto the stack, calls another routine, adjusts the stack pointer, and branches to another location.
- `push_pointer_and_call_string_handler`: Pushes the value in a3 onto the stack, calls a routine to select and copy a string, adjusts the stack pointer by four bytes, and then branches to another location.
- `validate_and_copy_block`: Calls a routine, and based on the value at offset 0 of a3, either jumps to a specific location or performs a series of checks and calls a routine to copy data from tables at offsets 0 and 22 of a3, or calls a routine to copy data using values from offsets 51, 52, and 104 of a3.
- `clamp_and_store_indexed_entry`: Clamps a structure byte using a table maximum and stores a table entry through it, then adjusts several coordinate fields and, when a flag is set, either increments a counter or resets a block of fields and writes a marker byte.
- `test_structure_fields`: Checks three specific bytes at offsets 49, 35, and 34 of a structure pointed to by an offset of a6, and if all conditions are met, loads two pointers into a0 and a4 before branching.
- `push_and_call_routine`: Pushes two words from a structure onto the stack, calls another routine, and then compares the result to four.
- `negate_conditional_value`: Tests a word at an offset of a structure and, if it is not greater than or equal to zero, negates it and branches.
- `check_structure_values`: Compares a word at offset 6 of a structure to 10 and a word at offset 12 to 0, and if both conditions are met, loads two pointers from a structure into a0 and a4.
- `load_pointer_and_test_byte`: Loads a pointer from an offset of a structure into a0, loads a second pointer from an offset of that pointer into a4, tests a byte at offset 51 of the structure, and either branches or extends a byte from offset 52 into a long integer.
- `load_pointers_and_branch`: Loads a pointer from offset 4 of a structure into a0, loads a pointer from offset 34 of the address in a0 into a4, and branches to another routine.
- `set_offset_and_branch`: Loads a pointer from offset 4 of a structure into a0, loads a pointer from offset 20 of the address in a0 into a4, writes the word 48 to offset 14 of a structure, and branches to another routine.
- `decrement_and_compare_byte`: Decrements d0 if it is 3 or 7, then compares a byte at offset 2 of a structure with a byte at a location pointed to by a4, and increments the byte at offset 2 of the structure if the comparison fails.
- `store_and_index_byte_copy`: Stores a one at a byte offset in a structure, then reads that value as an index to copy a byte from another indexed structure.
- `select_and_copy_string`: Selects a string from a table of pointers based on various flags and index bytes within a structure, then copies the next character from that string into a buffer at offset 1 of the structure while incrementing a length counter at offset 2.
- `update_structure_state_and_index`: Updates a structure by performing multiple calls to a subroutine using values derived from offsets 36 and 38, then selects a pointer from a table based on bytes at offsets 34 and 35.
- `set_byte_and_branch`: Sets a byte at offset 65 of a structure in a3 to two and branches to another location.
- `clear_byte_conditionally`: Compares a byte at offset 35 of a structure in a3 to two; if it is not equal, it clears a byte at offset 65 of the same structure and branches.
- `set_byte_and_load_pointer`: Sets a byte at offset 65 of a structure, loads a pointer from the stack into a register, and then copies a long value from offset 92 of that pointer to the stack before branching to another routine.
- `update_byte_conditionally`: Compares a byte at offset 35 of a structure in a3 to one; if it matches and a word in d4 is not zero, it sets a byte at offset 65 of the structure to one and branches.
- `set_byte_and_load_pointer_alt`: Sets a byte at offset 65 of a structure to two, loads a pointer from the stack into a register, and then copies a long value from offset 100 of that pointer to the stack before branching to another routine.
- `compare_and_update_pointer`: Compares a byte at offset 35 of a structure to two.
- `update_structure_field`: Sets a byte at offset 65 of a structure, calls another routine with two values from the stack, and if the result is 13, updates a pointer from a table and potentially increments a byte at offset 2 of the structure.
- `update_and_copy_fields`: Sets a byte at offset 2 of a structure to 1, copies a byte from a table into offset 1 of the structure, and if certain conditions are met, updates a global variable, modifies several bytes in a structure, and adjusts two 16-bit values by subtracting 8.
- `update_structure_and_add_offset`: Adjusts four 16-bit values in a structure by adding or subtracting 8, copies two 16-bit values, sets a byte at offset 2 to 1, calls another routine, and adds a constant to the result before storing it at offset 4 of the structure.
- `reset_and_read_table`: Clears a field in a structure and reads a value from a table using an index derived from another field.
- `select_string_and_clear_fields_2`: Checks two bytes for non-zero values, and if a byte at offset 52 of a pointer in a5 is not decimal 100, it stores decimal 100 at offset 64 of a pointer in a4.
- `load_pointer_and_branch`: Loads a pointer from offset -4 of a6 into a0, loads a long value from the address in a0 into the stack, and branches to another routine.
- `load_stack_from_pointer`: Checks a byte at offset 34 of a structure in a3 and compares a word in d4 to seven; if both conditions are met, it loads a long value from an offset of a pointer found at offset -4 of a6 into the stack and branches.
- `load_offset_pointer_and_branch`: Loads a pointer from offset -4 of a6 into a0, loads a long value from offset 4 of the address in a0 into the stack, and branches to another routine.
- `compare_and_load_pointer`: Compares a register to seven.
- `extend_byte_to_long`: Tests a byte at offset 51 of a structure and, if it is not zero, extends the byte at offset 52 into a word and then into a long register before branching.
- `load_structure_field_and_branch`: Loads a long value from an offset of a pointer into a0, stores a long value from an offset of a0 into a location on the stack, and branches to another routine.
- `update_indexed_value`: Loads a pointer from a structure offset into a register, then updates a value at an offset of two within a structure by either incrementing it based on a bit from a global variable or setting it to one.
- `load_structure_field_and_branch_2`: Loads a long value from a pointer into a0, stores a long value from a0 into a location on the stack, and branches to another routine.
- `test_register_value`: Tests a long value in d0 and branches to different locations if the value is 0, 2, or 6, otherwise it branches to another location.
- `branch_to_next_routine`: Performs a short branch to the instruction immediately following this routine.
- `update_byte_from_table`: Loads a pointer from offset -4 of a6, loads a word from that pointer into the stack, moves that word to a0, compares a byte at offset 2 of a3 with a byte at a0, and if they are equal, increments a byte at offset 2 of a3 by the result of an AND operation between a word from a global variable and 1.
- `store_indexed_byte`: Sets a byte at offset 2 of a structure to one, then reads a byte from a table indexed by that value through a pointer found at offset -8 of another structure and stores it at offset 1 of the first structure.
- `advance_and_select_table_index`: Loads a pointer from a table using an index, then advances or resets that index depending on the current value and a flag.
- `dispatch_table_entry`: Checks a byte at offset 98 of a structure; if it is not zero and is less than or equal to 3, it extracts a byte from offset 50 of the structure, extends it to a long integer, and branches.
- `lookup_and_jump_table`: Calls another routine, then checks if d0 is at least 21; if so, it uses d0 to index into a table and jumps to the resulting address, otherwise it performs a different jump or branches back.
- `validate_and_reset_structure`: Checks several conditions based on values at offsets 18 and 20 of a structure and a global value, then either sets bit 0 of offset 34 and writes a word to offset 10, or writes 100 to offset 0 and clears several other fields.
- `init_flag_and_clear_words`: Sets bit 1 of a byte at offset 34 of a structure, sets a word at offset 10 to a fixed value, clears words at offsets 28 and 12, and sets a word at offset 14 to a fixed value.
- `clear_structure_fields_2`: Writes the byte 100 to offset 0 of a structure, clears the byte at offset 63, clears the word at offset 10, clears the byte at offset 51, clears the byte at offset 52, clears the byte at offset 103, and branches to another routine.
- `set_buffer_and_clear_offsets_2`: Clears a byte at offset 48 of a structure, calls another routine, and if the returned word is 11 or 12, writes -3 to offsets 28 and 12 of the structure and shifts two words from offsets 24 and 20 right by 4 bits; otherwise, it writes 100 to the start of the structure and writes zero to offsets 63 and 52.
- `set_buffer_and_clear_offsets`: Stores the decimal value 100 at the start of a buffer and writes zero at offsets 63 and 52 within that buffer.
- `update_structure_state_and_coordinates`: Updates a structure's state and coordinate fields based on a control byte, using a table to determine new values and calling another routine to update global positioning data.
- `calculate_and_update_structure`: Performs several arithmetic operations involving structure fields and a byte at offset 8, calls another routine, and updates various fields in a second structure including a pointer and several bytes based on conditional checks and arithmetic.
- `select_string_and_clear_fields`: Checks two bytes for non-zero values, then if a byte at a specific offset in a selected pointer is not decimal 100, it stores a byte at offset 81 of that pointer and clears bytes at offsets 81 and 101 of a structure.
- `dispatch_state_transition`: Reads an index from a structure at offset 51 of a pointer passed in a6, then uses this index to select a destination structure from a table.
- `check_flag_and_update_struct`: Reads a byte from a fixed offset in a structure and looks up a value in a global table.
- `update_state_structure_fields`: Updates a structure by clearing a byte at offset 66, checking a byte at offset 94, and setting various fields at offsets 14, 28, 34, 35, 44, 45, 51, 52, and 65 based on comparisons of values at offsets 12, 14, 22, 23, 28, 33, 34, 35, 38, 44, 46, 51, 52, 54, 65, 66, 92, 93, and 94.
- `update_structure_state_and_effects`: Updates various fields in a structure based on its current state, including setting flags at offsets 34 and 35, and calling other routines to handle state transitions or visual effects.
- `update_field_with_conditional_increment`: Checks a global word; if it is not zero, it compares a 16-bit value at offset 96 of a structure against a global word, potentially decrements a byte at offset 90 based on a global byte, calls another routine, and updates a 16-bit value at offset 96.
- `clear_field_and_update_structure`: Checks a global word; if it is not zero, it compares a 16-bit value at offset 96 of a structure against a global word, clears that value if it exceeds 99, potentially increments a byte at offset 90, calls another routine, and updates the value at offset 96.
- `decrement_field_and_set_byte`: Checks a byte at offset 104 of a structure; if it is greater than 0, it decrements a byte at offset 90, calls another routine, and if a 16-bit value at offset 54 is not 16, sets a byte at offset 104 to 75.
- `init_or_update_struct`: Initializes or updates a structure depending on the state of a global variable, calling a per-field setup routine for a run of fields and then a second routine with different arguments.
- `adjust_table_values_and_dispatch`: Calculates a difference between a value in a structure and a global variable, then updates a global variable and three elements in a global table by adding or subtracting values retrieved from a lookup table or fixed increments.
- `clamp_and_accumulate_values`: Clamps a signed byte value into a range defined by a 16-bit field, adds this clamped value and another byte to a 16-bit accumulator, and then clamps the accumulator within the bounds of a 16-bit range.
- `adjust_tile_position`: Adjusts a tile-based position by adding a scaled movement value, clamping the result and storing the new coordinates.
- `update_flag_from_table`: Checks a bit in a structure and, depending on other flags, calls a routine to update a flag byte at offset 48 or performs a table lookup using a word from offset 36 or 38 to clear that flag byte.
- `adjust_packed_field`: Performs a table lookup and, based on the result, adjusts a packed 16-bit field at offset 24 using a value from another routine, updates a value at offset 38, and returns a computed quantity by subtracting a global variable from a value derived from the packed field.
- `adjust_table_value_by_mode`: Reads a value from a table using an offset and adjusts it based on a mode byte.
- `adjust_word_against_global`: Adjusts a word field based on comparisons with a global value and stores a difference, clearing a nearby field in some branches.
- `process_collision_state_and_update_status`: Checks a byte at offset 98 of a structure; if non-zero, it evaluates the object type and coordinate values to call another routine with either a pointer to the structure or a specific 16-bit value.
- `check_and_update_collision_regions`: Iterates through a table to check for overlapping spatial regions between two structures, performing coordinate arithmetic and updating status bytes or calling other routines if specific overlap conditions are met.
- `handle_overlapping_region`: Checks whether a region overlaps a table indexed by a byte, and if so copies several fields while computing shifted differences and a flag.
- `store_min_field_differences`: Computes the difference between a global value and two short fields, then stores either those differences or their adjusted absolute values into two short fields depending on which set has the smaller magnitude.
- `apply_clamped_position_offsets`: Iterates through a table of paired values, comparing them against two fields in a structure; if a match is found, it updates several numeric fields in the structure using clamped, incremental offsets and calls a series of helper routines to update the data.
- `decrement_word_when_underflow`: Decrements a word field when a smaller field is below another and a byte is empty, sets a flag, and clears a trailing byte.
- `adjust_word_by_comparison`: Adjusts a word field down or up depending on field comparisons, sets a flag, and clears a trailing byte before returning.
- `update_structure_fields_2`: If a 16-bit value at offset 42 of a structure in a3 is less than the value at offset 38, it clears offset 12, updates offset 24 by masking and subtracting 1, calculates offset 22 and offset 38 using a global variable, and sets bytes at offsets 46 and 44; otherwise, if offset 12 is negative, it performs a different update to offset 24 and offset 22.
- `update_structure_fields`: If a 16-bit value at offset 12 of a structure in a6 is negative, it clears that value, updates offset 24 by masking and adding 15, calculates a new value for offset 22 using a global variable, sets offset 38 by shifting the result of that calculation right by 4, and clears bytes at offsets 44 and 49.
- `store_one_if_fields_set`: Compares two fields of a global value and, when both are non-empty, stores one at a fixed offset before returning.
- `resolve_table_entry`: Adds a value from a structure field to a local variable, then repeatedly calls another routine until the returned value changes, and finally calls a third routine with the final result.
- `apply_offset_and_copy`: Adds a 16-bit value from offset 38 of a structure in a6 to a value from offset 12 of the same structure, then calls another routine with the sum, the constant 228, and a value from offset 76 of the structure.
- `copy_value_into_sequence`: Copies a value from a pointer into a sequence of consecutive word locations a fixed number of times, selecting the destination from a global value.
- `store_word_in_indexed_table`: Writes a 16-bit value from a register into a 32-bit slot of a table, using the low half of the input shifted left by two bits as an index.
- `store_low_word_in_global`: Copies the low 16 bits of an input value into a single global variable.
- `copy_pair_from_indexed_table`: Copies a pair of 16-bit values from an indexed source table into a fixed layout inside a destination buffer, repeating the process in a loop until a counter reaches sixteen.
- `setup_coprocessor_constants`: Clears a global variable, then writes a 16-bit constant and a value of one hundred into two adjacent 32-bit words, and finally stores a 16-bit constant into a global variable.
- `configure_coprocessor_registers`: Clears a 16-bit global variable, writes a 16-bit constant and the value one hundred into two adjacent 32-bit words, and then stores that 16-bit constant into the coprocessor control and jump registers.
- `setup_coprocessor_constants_set_one`: Sets a global variable to one, then writes a 16-bit constant and a value of one hundred into two adjacent 32-bit words, and finally stores a 16-bit constant into a global variable.
- `store_two_24bit_constants`: Stores two different 24-bit constants into two separate global variables, overwriting the first with the second constant.
- `initialize_blitter_and_interrupts`: Sets five short-value slots to their maximum value, writes two values to a hardware register, and checks a bit in a status register.
- `process_linked_list`: Clears a global byte, loads a pointer from a global variable, and if the pointer is not zero, pushes four values from the pointed-to structure onto the stack and calls another routine.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a6, then returns.
- `copy_adjusted_block`: Computes adjusted source and destination offsets and lengths from a pair of coordinates, then copies a block of words from a source array into a destination array while updating the destination pointer.
- `copy_array_and_adjust_pointer`: Copies ten 32-bit words and one 16-bit word from an input array into a global array, then adjusts a pointer derived from that array and assigns a fallback value when the pointer is null.
- `toggle_and_reload_audio_tables`: Toggles a global flag and, depending on the state, reloads several sets of four short-value slots in a table by computing values from a base pointer and fixed increments.
- `unlock_address_register_six_and_return`: Unlocks address register six and returns immediately.
- `initialize_global_if_empty`: It writes a value through a global variable only when that variable currently holds zero.
- `wait_for_flag_to_clear`: It repeatedly reads a global variable in a loop until that variable becomes zero.
- `copy_block_in_place`: Invokes another routine with two identical structure pointers and a fixed length value.
- `init_indexed_structure`: Computes a pointer by scaling a parameter by four and indexing into a global structure, then passes it along with the structure base to another routine.
- `init_global_structure`: Initializes a global structure by storing fixed default values at several fixed offsets, including a small 32-bit value from a parameter and several constant words.
- `copy_and_interleave_table`: Copies a block of 64 words from a source table to a destination table, spreading the individual 16-bit words across the destination at a set of fixed offsets.
- `split_and_copy_array`: Splits a 32-bit value into its high and low halves, adds each to a global base, and shifts a second parameter left by four.
- `copy_array_and_adjust_pointer_2`: Copies ten 32-bit words and one 16-bit word from an input array into a global array, then adjusts a pointer derived from that array.
- `init_global_structure_again`: Initializes a global structure by storing fixed default values at several fixed offsets, including a 32-bit value taken from the structure itself.
- `clear_and_copy_blocks`: Clears a block of 16-bit words to zero and then copies that block into several other blocks in a loop.
- `set_global_fixed_value`: It always stores the same fixed 16-bit value into a global variable regardless of the branch taken.
- `store_default_value`: It writes the constant 32 to a global variable, taking one branch or the other depending on whether that variable currently holds zero, and then returns.
- `clear_global_table_blocks`: It clears two blocks of 16-bit words in a global table and then zeroes a 32-bit value at one of two addresses selected by the argument.
- `update_size_lookup_table`: Validates input ranges and then updates a lookup table by calculating offsets and sizes based on two input lengths and a flag.
- `build_size_lookup_table`: Computes several size and offset values from two input lengths and a flag table, then packs them into a byte lookup table along with a control byte.
- `dispatch_table_by_flags`: Reads a series of small flag fields from a table and, based on which flags are set, looks up and calls a corresponding routine from another table.
- `apply_bitwise_mask_pairs`: Combines two input 16-bit values with bitwise NOT and ANDs them into matching words at two different memory locations.
- `apply_bitwise_mask_pairs_2`: Performs a bitwise OR on d0 with a constant, branches to another routine, and then restores registers and branches elsewhere.
- `run_infinite_loop`: If a value at a specific memory location is not equal to itself, it enters an infinite loop; otherwise it returns.
- `save_mask_disable_interrupts`: Increments a global counter if its value is not equal to the value at a preceding memory location, then updates a byte at an offset from that location to zero and writes a value to the interrupt enable register.
- `handle_keyboard_interrupt`: The routine manipulates global variables based on flags, writes a constant to a hardware register, and touches CIAA and custom registers.
- `store_value_toggle_flags`: Stores an input value into a global variable, conditionally writes a byte to a global variable, and toggles bits in a global flag table.
- `process_joystick_input_2`: Checks bits in a global byte, and if certain conditions are met, writes values to two different global memory locations.
- `process_joystick_input`: Clears several global variables and, if specific bits are set in the joystick register, calculates the difference between current and previous input values and updates two global counters.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `set_clear_bits_in_bytes_2`: Clears a bit in a global variable at an offset derived from a hardware register, then conditionally sets a bit in another global variable and calls another routine.
- `set_clear_bits_in_bytes`: Manipulates individual bits in a set of adjacent memory bytes using a shift amount and an input value.
- `clear_memory_and_setup_interrupts`: Clears a block of 128 bytes starting at a fixed address, updates several global variables with values from hardware registers and constants, sets a new interrupt vector, and modifies CIAA and DMA control registers.
- `calculate_and_fill_pattern_structure`: Calculates a set of coordinates and dimensions based on input ranges and a global variable, then populates a list of structures with calculated values, including offsets, lengths, and specific constants, using a loop.
- `fill_memory_block_with_pattern`: Fills a block of memory with a repeating pattern, writing several fixed values to specific offsets of each entry.
- `initialize_and_rotate_data`: Clears a global word, calls three routines, and if a global word is zero, it performs a series of bitwise rotations and swaps on values loaded from offsets 26, 30, and 28 of a4 before calling another routine.
- `initialize_global_counters`: Checks a global variable and, if it is zero, sets it to one and performs two loops.
- `filter_and_store_bytes`: Fills a memory range with the value 32 using a loop, then reads bytes from a source, clears bit 6 if set, and stores the result into a destination buffer.
- `scan_buffer_against_table`: Walks through a buffer in fixed-size steps, comparing each entry against a table and returning early when a matching or empty entry is found.
- `process_and_copy_data`: If a long value in a5 is between zero and 1023, it calls another routine, copies a block of bytes from a buffer to a destination, and performs bitwise manipulations on a value derived from d0 to update d0 before returning.
- `modify_and_call_routine`: Subtracts two from a register, shifts it left, adds a value from a global variable, and calls another routine.
- `calculate_division_remainder`: Performs a division and multiplication sequence using values from a global variable and a register, then updates a register with the result and branches back to the start of the sequence.
- `store_byte_into_global`: Stores two fixed byte values into the CIA register at offset a fixed address.
- `write_double_byte_to_global`: Writes the value 247 to a global variable after first writing the value 255 to the same location.
- `read_disk_track_sectors`: Computes a track position from registers, sets disk DMA parameters in the custom chip, and enters a loop that copies 512-byte blocks of data from a source to a destination buffer, incrementing a global byte value until a block is complete.
- `process_masked_data_blocks`: Iterates through memory to find specific markers, then processes blocks of data by masking, XORing, and bit-shifting values before storing them in a table.
- `combine_masked_values`: Loads two 32-bit words from a pointer in a0, masks both with d7, shifts the first result left by one, and returns the bitwise OR of the two results.
- `execute_zero_or_instructions`: Performs eleven consecutive bitwise OR operations with zero on d0.
- `restore_registers_and_return`: Restores several registers from the stack and returns.
- `adjust_disk_head_track`: Modifies the CIAA Port A register and calls modify_and_delay to synchronize a global byte value with a target value derived from a register, then sets or clears bit 2 of the CIAA Port A register based on the low bit of the original input.
- `set_bit_and_branch`: Sets bit 0 of a byte at the address pointed to by a1 and branches to another routine.
- `modify_and_delay`: Clears bits 0 and 1 of a byte at the address in a1, sets bit 0, and then executes a loop that decrements d0 from 3584 until it reaches zero before returning.
- `clear_global_byte_and_call`: Writes a zero byte to a global variable and calls another routine.
- `load_name_table_entries`: Calculates a destination buffer address based on a maximum length found in a table, then copies a series of name entries into that buffer.
- `update_audio_parameters`: Increments a global counter and, upon reaching a threshold, performs several loops to update audio channel registers by fetching values from a table and calling another routine for each of the four channels.
- `update_audio_channel_parameters_2`: Processes audio channel commands by fetching parameters from a table and updating channel structures, including volume, frequency, and period values.
- `compare_channel_target_value`: Copies a 12-bit value from a channel record into a field and sets a flag when it is below a stored target.
- `update_audio_channel_parameters`: Processes a command byte to modify a 12-bit audio value or volume within a channel structure, including logic to clamp the value to specific minimum thresholds or adjust it based on a secondary parameter.
- `configure_audio_hardware`: Checks two global variables and, if both conditions are met, calls two other routines, updates several global variables, and calculates new values from a table to write to audio hardware registers including period, volume, and control registers.
- `update_audio_control`: Checks bit 7 of a value at an offset from the custom chip base address; if the bit is set, it increments a 16-bit global variable, and if that variable reaches one, it clears it and calls another routine before returning.
- `write_audio_control`: Writes the value 128 to an offset of 156 from a pointer in a6, calls another routine, and returns from an interrupt.
- `execute_looping_subroutine`: Loads the base address of the custom hardware into a0, sets d2 to 5, calls another routine, and loops 6 times before branching to another location.
- `infinite_compare_loop`: Contains an infinite loop that compares a byte at offset six from a pointer with itself, so the loop never exits and performs no other operation.
- `set_audio_channel_period`: Clears a global variable, writes a fixed period value to the first audio channel period register and sets a control byte.
- `perform_signed_division`: Reads two signed 32-bit values in d0 and d1, determines the sign of the result, calls another routine, and returns a 64-bit value combining the result and the second input.
- `swap_and_rotate_words_2`: Swaps the high and low words of a value, then repeatedly doubles a pair of words while propagating carry and performing conditional subtract.
- `swap_and_rotate_words`: Swaps the high and low words of a value, then repeatedly doubles a pair of words while propagating carry and performing conditional subtractions fifteen times.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `perform_zero_operations`: Performs six identical logical OR operations on register d0 with zero.
- `set_register_and_loop`: Performs a logical OR operation on a longword in a register with a constant value and then enters an infinite loop.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_8`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_9`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_10`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_11`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_12`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_13`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_14`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_15`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_16`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_17`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_18`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_19`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_20`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_21`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_22`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_23`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_24`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_25`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_26`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_27`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_28`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_29`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_30`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_31`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_32`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_33`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_34`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_35`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_36`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_37`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_38`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_39`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_40`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_41`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_42`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_43`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_44`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_45`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_46`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_47`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_48`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_49`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_50`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_51`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_52`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_53`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_54`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
