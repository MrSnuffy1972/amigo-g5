# Game 1695 (blues brothers, the (1991)(titus)[cr company][t +3 defjam]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01695). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 1 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 285 of 285 named (0 author symbols), 285 of 285 described (4 by rule, 226 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1827 of 1827 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 12494 of 15944 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 95242 |
| code | 60188 |
| data | 705746 |
| padding | 8 |
| unused | 39936 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf:bootblock | 1 | 35 | 2 |
| logged disk load from Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf boot-block load | 1 | 156 | 2 |
| logged disk load from Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf boot-block load | 1 | 15597 | 279 |
| logged disk load from Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf boot-block load | 1 | 156 | 2 |

## Cases handled

- blues brothers, the (1991)(titus)[cr company][t +3 defjam]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf: custom-format (non-DOS) disk, 1651 non-empty blocks: matched against track-loaded segments
- blues brothers, the (1991)(titus)[cr company][t +3 defjam]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf: the boot block reads 17408 bytes from disk offset 805888 to an address (found statically); recovered as a unit
- blues brothers, the (1991)(titus)[cr company][t +3 defjam]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf: the boot block reads 98304 bytes from disk offset 707584 to an address (found statically); recovered as a unit
- blues brothers, the (1991)(titus)[cr company][t +3 defjam]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf: the boot block reads 53248 bytes from disk offset 823296 to an address (found statically); recovered as a unit
- blues brothers, the (1991)(titus)[cr company][t +3 defjam]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf: 3 logged loads cover 168960 bytes of the image
- blues brothers, the (1991)(titus)[cr company][t +3 defjam]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf: blocks 2-4 (bytes 1024-2560) outside the filesystem hold data, not code (opcode density 0.018, entropy 6.73)
- blues brothers, the (1991)(titus)[cr company][t +3 defjam]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf: blocks 6-7 (bytes 3072-4096) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.77)
- blues brothers, the (1991)(titus)[cr company][t +3 defjam]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf: blocks 9-12 (bytes 4608-6656) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.77)
- blues brothers, the (1991)(titus)[cr company][t +3 defjam]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf: blocks 16-531 (bytes 8192-272384) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.41)
- blues brothers, the (1991)(titus)[cr company][t +3 defjam]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf: blocks 538-786 (bytes 275456-402944) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.88)
- blues brothers, the (1991)(titus)[cr company][t +3 defjam]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf: blocks 790-870 (bytes 404480-445952) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.80)
- blues brothers, the (1991)(titus)[cr company][t +3 defjam]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf: blocks 874-902 (bytes 447488-462336) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.01)
- blues brothers, the (1991)(titus)[cr company][t +3 defjam]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf: blocks 904-904 (bytes 462848-463360) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.59)
- blues brothers, the (1991)(titus)[cr company][t +3 defjam]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf: blocks 906-966 (bytes 463872-495104) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.95)
- blues brothers, the (1991)(titus)[cr company][t +3 defjam]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf: blocks 968-968 (bytes 495616-496128) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.18)
- blues brothers, the (1991)(titus)[cr company][t +3 defjam]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf: blocks 970-970 (bytes 496640-497152) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.38)
- blues brothers, the (1991)(titus)[cr company][t +3 defjam]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf: blocks 972-1158 (bytes 497664-593408) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.67)
- blues brothers, the (1991)(titus)[cr company][t +3 defjam]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf: blocks 1160-1162 (bytes 593920-595456) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.94)
- blues brothers, the (1991)(titus)[cr company][t +3 defjam]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf: blocks 1164-1164 (bytes 595968-596480) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.30)
- blues brothers, the (1991)(titus)[cr company][t +3 defjam]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf: blocks 1166-1232 (bytes 596992-631296) outside the filesystem hold data, not code (opcode density 0.002, entropy 1.86)
- blues brothers, the (1991)(titus)[cr company][t +3 defjam]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf: blocks 1234-1238 (bytes 631808-634368) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.77)
- blues brothers, the (1991)(titus)[cr company][t +3 defjam]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf: blocks 1240-1268 (bytes 634880-649728) outside the filesystem hold data, not code (opcode density 0.005, entropy 2.18)
- blues brothers, the (1991)(titus)[cr company][t +3 defjam]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf: blocks 1270-1270 (bytes 650240-650752) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.55)
- blues brothers, the (1991)(titus)[cr company][t +3 defjam]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf: blocks 1272-1272 (bytes 651264-651776) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.57)
- blues brothers, the (1991)(titus)[cr company][t +3 defjam]/Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf: blocks 1274-1381 (bytes 652288-707584) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.43)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf:bootblock

- `load_disk_blocks`: It invokes a startup routine, then writes control and address fields into a hardware structure to load a large data block from the disk into memory.
- `init_system_and_clear_memory`: Initializes a hardware structure by writing control and address fields, invokes a startup routine, then clears a large block of memory from a start marker up to a fixed limit, and finally stores a byte value into a global variable.

### logged disk load from Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf boot-block load

- `shift_and_store_word`: Reads a word located four bytes before a pointer and writes a word to a global location whose high byte comes from an unchanged register.
- `copy_word_block`: It repeatedly shifts a value right until it becomes nonzero, then copies that many words from one location to another.

### logged disk load from Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf boot-block load

- `copy_interleaved_records`: It reads a length from a global variable and copies that many four-byte records from one location to another, interleaving the source words into consecutive destination words.
- `pack_and_interleave_table`: It reads a byte length from a global table and packs its bytes into a sequence of 16-bit words by shifting each byte into the high half, then repeats the packing over the next block.
- `initialize_table_and_pointer`: It writes the value sixteen into two consecutive words of a global table in a loop that runs six times, then stores the address of that table into two consecutive global variables.
- `load_entry_table`: Loads a block of two-word records from a source table into a destination table, copying 41 entries.
- `copy_and_interleave_words`: Copies a variable-length block of words from a source table into a global buffer, then interleaves those words into a second buffer by copying pairs of values with a two-word gap.
- `store_linked_list_pointers`: Reads a count from a buffer, then walks a list of linked entries storing each pointer into a table while advancing by a stride derived from the entry contents.
- `store_linked_list_entries`: Reads a count from a buffer, then walks a list of linked entries storing each pointer into a table and advancing by a stride computed from the entry contents.
- `fill_table_with_offset`: Fills a table of 256 words by writing the sum of an index times 26 and a fixed address into each slot.
- `scatter_bytes_nibbles`: Reads a source block described by a header and scatters its bytes into a destination table, splitting each nibble into separate slots.
- `pack_bytes_with_zeros`: Reads a source block described by a header and packs its bytes into a destination table, storing a zero after each inner row.
- `store_pointer_table_entries`: Reads a count from a buffer, then stores a running pointer into a table advancing by a fixed stride for each entry.
- `increment_and_store_loop`: Loops over a count, storing a value into a table slot and incrementing both the value and a counter until the counter reaches a limit.
- `call_with_fixed_address`: Invokes another routine passing a fixed address as its single argument, then returns.
- `forward_single_pointer`: Passes a single pointer argument to another routine and returns without modifying anything itself.
- `combine_table_and_index`: Returns a word loaded from a table combined with a shifted multiple of an index.
- `run_conditioned_loop`: Selects initial values for a loop counter and step based on whether a flag is zero, then runs a loop that repeatedly calls two other routines and a third routine.
- `copy_buffer_fields`: Copies a fixed number of spaced fields from one buffer to another in a loop.
- `call_wrapper_twice`: Invokes another routine twice with the same global variable as argument and then returns.
- `scan_and_expand_strings`: Checks a value from a routine, and when it is zero scans a buffer advancing by two, then performs two more operations before returning a flag.
- `expand_and_copy_strings`: Copies characters from a source to a destination, expanding negative bytes by repeating the following character.
- `expand_and_copy_strings_2`: Repeatedly invokes another routine over a range of source offsets and then advances the source pointer by a fixed amount, expanding and copying characters into a destination.
- `pack_nibbles_from_bytes`: Reads three bytes at a time and packs their nibbles into consecutive output words.
- `compare_strings`: Compares two strings byte by byte and returns one when they match or zero when they differ.
- `clear_and_or_global`: Clears a global variable ten times in a row, then calls another routine and performs a series of bitwise OR operations on the variable.
- `extract_and_pack_nibbles`: Scans nibbles of each word in a source sequence, accumulates the highest nibble into a destination word, and repeats this pass over a fixed number of words while refreshing the source.
- `copy_and_unpack_nibbles`: Copies a sequence of words into a global buffer and then reduces each nibble of every word by one when it is nonzero, packing the result back into the same words.
- `init_entity_state_tables`: Loads a couple of table entries, runs two setup routines, then fills a 256-byte table with the values 0 to 255.
- `setup_and_copy_block`: Runs a chain of setup calls that initialize a global variable and a coprocessor, then copies an adjusted block of words from a source array into a destination.
- `run_dual_counter_loops`: Optionally calls a routine when a flag is set, then runs two fixed-count loops that increment a counter and pass it along with a global value to another routine.
- `fill_memory_with_alternating_patterns`: Fills two adjacent blocks of 16-bit words with alternating patterns of 65535 and 0, walking through each block in fixed-size chunks until both are completely written.
- `decrement_and_call_twice`: Loops a counter down by ten until it is no longer greater than ten, then calls another routine twice and returns the original register values.
- `copy_buffer_fields_spaced`: Copies a set of spaced fields from one buffer to another using a fixed offset pattern.
- `scatter_copy_global_source`: Copies a fixed block of 16-bit words from a global source table into a destination structure, using the same scattered offsets as the copy routine and additionally selecting between two source tables based on a flag.
- `scatter_copy_structures`: Copies a fixed block of 16-bit words from one structure to another using a set of scattered offsets, moving the first word and then the surrounding fields in a fixed pattern.
- `scatter_copy_indexed_source`: Computes a table offset from part of an argument, selects a source pointer from either a global table or a fixed address based on a flag, and copies a block of 16-bit words into a destination structure using scattered offsets.
- `scatter_copy_fixed_table`: Computes a table offset from part of an argument, then copies a block of 16-bit words from a fixed RAM table into a destination structure using scattered offsets.
- `initialize_global_pair`: Initializes a global variable by storing one value followed by another, performing only two sequential stores and returning.
- `set_globals_zero_and_neg_one`: It writes the value 0 into one global variable and then writes the value -1 into a different global variable.
- `clear_and_load_names`: Invokes a routine at a fixed address, then clears a global variable ten times while applying a series of bitwise OR operations to it.
- `initialize_and_loop_table_setup`: Initializes a global variable and a couple of stack counters, then enters a loop that repeatedly calls another routine, adjusts two index counters based on its result, writes a byte and a value of one hundred into two adjacent memory slots, and clears several blocks of a table.
- `combine_and_shift_registers`: Combines high halves of the input register and a global variable with shifted copies of the input fields, adding the results together and returning the sum.
- `spread_words_from_shifted_value`: Shifts the two halves of a passed value apart and feeds each half into another routine through a pointer, copying a block of 64-bit data and spreading its 16-bit words across a destination table in a nested loop, then writes sixteen into two consecutive words of a global table.
- `copy_table_block`: Copies a block of 16-bit words from a source table into a destination table.
- `adjust_table_pointers`: Adjusts three pointer slots in a global table by adding or subtracting a fixed increment, depending on the current values.
- `dispatch_copy_by_global_value`: Calls another routine once, then dispatches to a copy routine with different argument pairs depending on the value held in a global variable.
- `update_flag_and_advance`: Reads a byte from a global variable and builds a flag byte based on whether the value is nonzero, then sets additional flag bits accordingly.
- `update_flags_and_advance_counter`: Writes a byte based on a global flag, sets several flag bits when the flag is nonzero, then updates and advances a global counter depending on a comparison.
- `set_flags_and_advance_global_store`: Clears a byte addressed relative to a register, then sets individual flag bits in it depending on the state of a global variable, and finally either increments a global counter or advances a global store pointer and records the byte at the new location.
- `advance_global_store_byte`: Increments a global variable by two bytes and stores a zero byte at the new location, then copies one byte from just before the address in a register into the following location and advances the global variable.
- `load_and_mask_global_byte`: Loads a byte from a global variable, advances the pointer by two, then stores a second byte, and finally masks the value with several single-bit constants.
- `increment_and_wrap_global_byte`: Compares a global variable against a value in a fixed table, increments it and wraps it to one when it reaches five, otherwise clears it.
- `update_entity_states`: Iterates over a table of entity structures, updating their coordinate fields and status flags, then dispatches each entity to a state handler that advances its position and selects its next behavior.
- `store_indexed_bytes_from_table`: Reads a value from a table using an index derived from a structure field and stores it through a pointer, then either copies two adjacent bytes from that table into fixed offsets of the structure or resets several of its fields to zero when a shifted byte exceeds the table limit.
- `copy_indexed_bytes_between_structures`: Reads a byte from a fixed offset in one structure and uses it as an index to copy two consecutive bytes from another structure into fixed offsets in the first structure.
- `clear_structure_fields`: Clears several byte and word fields at fixed offsets within a structure pointed to by a register.
- `initialize_structure_fields`: Initializes several fields of a structure, conditionally clears one based on a comparison, and toggles a bit in an existing field.
- `decode_and_build_coordinates`: Decodes a packed value from a global variable by repeatedly extracting nibbles and subtracting table offsets, then builds a small data structure with two coordinates derived from lookup tables.
- `invoke_pointer_from_indexed_table`: Combines two structure fields into a single value and passes it to another routine, then reads a pointer from a global table using a computed index and invokes the routine stored there with the structure.
- `load_byte_from_indexed_table`: Computes a table index from a parameter, reads a byte from a global variable, multiplies it by a fixed stride, and loads a word from a computed address in a data table, returning the low byte of that word.
- `load_byte_from_struct_indexed_table`: Reads two fields from a structure passed as a parameter, combines them into a table index, and loads a word from a computed address in a data table, returning the low byte of that word.
- `update_text_struct`: Updates a text structure by decoding a string field and applying per-character effects, recursing into sibling entries when needed.
- `process_structure_fields`: Reads a byte from a structure and, if it is below ten, loops a number of times proportional to that byte while calling another routine.
- `copy_tile_block`: Copies a 64-byte block of word-sized data from one location to another, using a fixed offset pattern to place each of the 64 words.
- `read_table_byte_add_two`: Indexes into a table using a value taken from a structure and a masked field, then reads the byte stored there and adds two to it.
- `read_table_byte_at_offset`: Indexes into a table using a shifted byte and a high half of the argument, then reads the byte stored at a fixed offset from that entry.
- `copy_transformed_table_block`: Walks a linked list of node structures, following a chain of child pointers and recording a computed value into a field.
- `copy_and_merge_data_blocks`: Iterates over a table of source data and destination offsets, copying and bitwise-merging word-sized blocks into scattered locations in a large output buffer.
- `clip_and_fill_table`: Clamps two coordinate pairs into a fixed range and writes the resulting rectangle corners into a table, producing a set of clipped entries.
- `validate_and_adjust_block`: Clears a byte at a fixed offset within a block pointed to by its argument, then branches on another byte at a different offset to call a validation-and-copy routine and, in one case, a routine that adjusts block offsets and copies a block.
- `validate_and_adjust_block_2`: Invokes a validation-and-copy routine and then inspects the leading byte and a flag byte of a block to call a routine that adjusts block offsets and copies a block with arguments derived from the block's contents.
- `select_and_copy_string_2`: Invokes a routine that selects a string from a table of pointers and copies it into a buffer, then inspects the leading byte and a flag byte of a block to call a routine that adjusts block offsets and copies a block with arguments derived from the block's contents.
- `validate_and_copy_block`: Validates a byte against fixed ranges and, based on a flag and a computed index, calls a routine that adjusts block offsets and copies a block of words from a source array.
- `clamp_and_store_indexed_entry`: Clamps a structure byte using a table maximum and stores a table entry through it, then adjusts several coordinate fields and, when a flag is set, either increments a counter or resets a block of fields and writes a marker byte.
- `select_pointer_increment_index`: Selects a pointer from a structure based on several flag bytes and a counter, then increments that counter up to a limit and stores an indexed byte from the chosen pointer.
- `select_pointer_and_increment_index`: Selects a pointer from a structure based on a flag byte and a counter value, then increments that counter up to a limit and stores an indexed byte from the selected pointer.
- `select_and_increment_index_2`: Computes an absolute value, selects a limit pointer from a structure based on flags, increments a counter byte, and stores an indexed byte from that pointer.
- `select_and_increment_index`: Selects a limit pointer from a structure based on two field values, then refines it depending on a flag byte and its contents.
- `increment_clamped_index`: Selects a limit pointer from a table based on a structure field, increments a counter byte up to that limit, and stores the indexed byte from the table.
- `clamp_and_store_indexed_byte`: Reads a pointer from a table field, then increments a counter byte up to a table maximum before storing a table byte through a second pointer.
- `set_value_and_store_indexed_byte`: Reads a pointer from a table field, stores a fixed value at a struct offset, then increments a counter byte up to a table maximum before storing a table byte through a second pointer.
- `store_clamped_indexed_byte`: Selects a pointer from a table based on a passed value, clamps a structure byte against that pointer's limit, and stores the referenced table byte through the clamped index, setting a fixed word when a specific value is matched.
- `store_and_index_byte_copy`: Stores a one at a byte offset in a structure, then reads that value as an index to copy a byte from another indexed structure.
- `select_and_copy_string`: Selects one string from a table of pointers based on a code byte and a flag byte, then copies that string into a buffer while tracking its length and clamping it to a maximum size.
- `reinitialize_structure_fields`: Selects one of two string blocks based on a condition, then clears and reinitializes a structure field.
- `set_flag_increment_index_2`: Sets a flag byte, loads a pointer from a structure, increments a counter up to a limit, and stores an indexed byte from that pointer.
- `set_flag_from_field_index`: Sets a flag byte based on a structure field, loads a pointer from a structure, increments a counter up to a limit, and stores an indexed byte from that pointer.
- `adjust_structure_increment_index`: Sets a flag byte, loads a pointer from a structure based on that flag, increments a counter up to a limit, and stores an indexed byte from the pointer.
- `set_flag_increment_index_3`: Sets a flag byte based on a status value, loads a pointer from a structure, and increments a counter up to a limit, then stores an indexed byte from a table through that pointer.
- `set_flag_increment_index`: Sets a flag byte to a fixed value, loads a pointer from a structure, increments a counter up to a limit, and stores an indexed byte from that pointer.
- `select_refined_increment_index`: Chooses a pointer from a structure based on a flag byte, optionally refining it through a lookup routine, then increments a counter up to a limit and stores an indexed byte.
- `adjust_structure_and_increment_index`: Sets a flag byte, loads a pointer from a structure based on a flag byte, then increments a counter up to a limit and stores an indexed byte, or fills several fields of another structure when a different condition holds.
- `copy_and_adjust_structure_2`: Sets a flag byte, stores an indexed byte from a structure, then either calls another routine or copies several fields from one structure to another with adjusted values.
- `copy_and_adjust_structure`: Copies several word-sized fields from one structure to another with fixed offsets added to each value, sets a byte field to one, and stores a value returned by another routine plus a constant into a field of the destination structure.
- `reset_and_read_table`: Clears a field in a structure and reads a value from a table using an index derived from another field.
- `select_text_and_clamp_field`: Selects one of two global text blocks based on a flag byte, then walks a chain of linked records by following pointers until a terminator is found.
- `copy_pointer_and_store_indexed_byte_2`: Copies a double pointer through a register, then clamps a structure byte against the pointed table and stores the referenced byte through the clamped index, using a global flag to adjust the increment.
- `load_pointer_clamp_index`: Selects a pointer from a global table based on a flag byte and a counter, clamps a structure byte against the pointed table, and stores an indexed byte from it.
- `load_pointer_and_store_indexed_byte`: Reads a pointer from an offset within a global table, then clamps a structure byte against the pointed table and stores the referenced byte through the clamped index, using a global flag to adjust the increment.
- `select_pointer_and_increment_index_2`: Selects a pointer from a structure based on a flag byte, then increments a counter up to a limit and stores an indexed byte from that pointer.
- `load_and_increment_index_2`: Selects a pointer from a structure based on a flag byte, then increments a counter byte and stores an indexed byte from that pointer.
- `increment_indexed_byte`: Copies a four-byte value from a table, increments a counter byte up to a stored limit, and stores the indexed byte from that table.
- `update_indexed_byte_2`: Loads a value from a double pointer into a slot, then compares a byte at a pointer offset against that value to decide whether to increment the byte or reset it to one.
- `store_indexed_byte_2`: Loads a byte from a table indexed by a value stored at one pointer, using a byte from another pointer as the index, and writes the result back to that same location.
- `load_and_increment_index`: Loads a four-byte value from a table or pointer depending on a register value, then increments a counter byte up to that limit or resets it.
- `update_indexed_byte`: Compares a byte stored at an offset from one pointer against the value pointed to by a double pointer, then either increments that byte using a global flag or resets it to one.
- `copy_pointer_and_store_indexed_byte`: Copies a double pointer through a register, then increments a counter byte using a global flag before storing a table byte through a computed address.
- `store_indexed_byte`: Sets a byte at a fixed offset to one, then reads a byte from a table indexed by that value through another pointer and stores it there.
- `advance_and_select_table_index`: Loads a pointer from a table using an index, then advances or resets that index depending on the current value and a flag.
- `dispatch_entity_state`: Reads a state byte from a per-entity structure and dispatches to a handler routine based on its value.
- `check_flag_update_struct`: Reads a byte from a fixed offset in a structure and looks up a value in a global table.
- `clamp_word_and_configure_fields`: Clamps a structure word against fixed bounds and sets a flag byte when it falls outside those bounds, then either configures several word fields and a flag according to another structure field or resets a block of fields and returns a constant.
- `init_flag_and_clear_words`: Sets a flag byte with a bit, then clears several word fields and sets one of them to a fixed value before returning zero.
- `set_flag_and_clear_words`: Stores a fixed value at a struct offset, clears several word fields, and returns a constant.
- `adjust_structure_by_table_value`: Clears a field in a structure and reads a value from a table using an index derived from another field.
- `set_buffer_and_clear_offsets`: Stores a constant at the start of a buffer and writes zero at two fixed offsets within it.
- `adjust_and_update_entity_state`: Reads a state byte from a data structure and, based on its value, adjusts and stores coordinate fields along with several flags.
- `adjust_and_store_struct_fields`: Computes adjusted coordinate values from a source struct, calls a routine for a value, stores results and flags across two structs, and returns.
- `select_string_and_clear_fields`: Selects a string pointer based on two conditions, then stores a fixed byte and clears two fields through the chosen pointer.
- `dispatch_state_transition`: Reads a value from a table using an index and stores it through a pointer, then branches on that value to set flags and copy fields between two data blocks, calling other routines to compute coordinates and perform follow-up actions.
- `check_flag_and_update_struct`: Reads a byte from a fixed offset in a structure and looks up a value in a global table.
- `initialize_and_update_struct`: Initializes fields of a structure pointed to by its parameter, then branches on a decoded value to set flags and modify other fields, calling helper routines to update text and process sub-structures.
- `initialize_and_dispatch_text_struct`: Initializes a text structure by clearing two flag bytes, then branches on the contents of its fields to set state values, apply per-character effects, and invoke sibling routines, including a special path triggered when a preceding entry has a specific value.
- `wrap_and_update_global`: Subtracts 100 from a value when it exceeds 99, bumps an adjacent counter, and then updates a global variable from that value.
- `reset_and_update_global`: Resets a value above 99 to zero, increments an adjacent counter, and then updates a global variable from that value.
- `decrement_and_set_flag`: Decrements a counter while it is positive, then sets a flag byte to a fixed value unless it already equals a specific constant.
- `init_or_update_struct`: Initializes or updates a structure depending on the state of a global variable, calling a per-field setup routine for a run of fields and then a second routine with different arguments.
- `apply_clamped_position_offsets`: Reads offset values from a parameter block and applies clamped, incremental adjustments to a set of paired numeric fields in two tables, scaling the change based on the current stored values.
- `clamp_and_accumulate_values`: Clamps a value from a byte field into a range derived from another field, then adds several fields together and clamps the result.
- `adjust_tile_position`: Adjusts a tile-based position by adding a scaled movement value, clamping the result and storing the new coordinates.
- `update_flag_from_table`: Clears or sets a flag byte in a structure based on other flag bytes and a table lookup.
- `adjust_packed_field`: Reads a value from a table using a field of the structure and branches on it.
- `adjust_table_value_by_mode`: Reads a value from a table using an offset and adjusts it based on a mode byte.
- `adjust_word_against_global`: Adjusts a word field based on comparisons with a global value and stores a difference, clearing a nearby field in some branches.
- `process_and_maybe_recurse`: Checks a flag byte, derives a value from a parameter, then branches on several fields before optionally recursing with the derived value.
- `process_overlapping_entities`: Walks a table of entity records, comparing their positions and flags against a source record, and when a match is found it copies several fields, sets state bytes, and invokes another routine to process the linked record.
- `handle_overlapping_region`: Checks whether a region overlaps a table indexed by a byte, and if so copies several fields while computing shifted differences and a flag.
- `store_min_field_differences`: Computes the difference between a global value and two short fields, then stores either those differences or their adjusted absolute values into two short fields depending on which set has the smaller magnitude.
- `apply_clamped_position_offsets_2`: Walks through a table of paired offset values, comparing two fields in each entry against a pair of stored values.
- `decrement_word_when_underflow`: Decrements a word field when a smaller field is below another and a byte is empty, sets a flag, and clears a trailing byte.
- `adjust_word_by_comparison`: Adjusts a word field down or up depending on field comparisons, sets a flag, and clears a trailing byte before returning.
- `adjust_field_and_increment`: Adjusts two short fields in a structure when they are out of order, rounding a word-sized value up or down to a multiple of sixteen and storing the difference.
- `adjust_negative_field_and_increment`: When a short field is negative it zeroes it, rounds another short field up to a multiple of sixteen, and records the difference and shifted value into nearby fields.
- `store_one_if_fields_set`: Compares two fields of a global value and, when both are non-empty, stores one at a fixed offset before returning.
- `resolve_table_entry`: Adds a value to a field, then repeatedly calls a lookup routine until its result changes, and finally invokes another routine on the combined value.
- `apply_offset_and_copy`: Adds a value to a structure field and passes the sum to another routine that copies a block of word-sized data.
- `copy_value_into_sequence`: Copies a value from a pointer into a sequence of consecutive word locations a fixed number of times, selecting the destination from a global value.
- `store_word_in_indexed_table`: Writes a 16-bit value from a register into a 32-bit slot of a table, using the low half of the input shifted left by two bits as an index.
- `store_low_word_in_global`: Copies the low 16 bits of an input value into a single global variable.
- `copy_pair_from_indexed_table`: Copies a pair of 16-bit values from an indexed source table into a fixed layout inside a destination buffer, repeating the process in a loop until a counter reaches sixteen.
- `setup_coprocessor_constants`: Clears a global variable, then writes a 16-bit constant and a value of one hundred into two adjacent 32-bit words, and finally stores a 16-bit constant into a global variable.
- `setup_coprocessor_constants_alt`: Clears a global variable, then writes a 16-bit constant and a value of one hundred into two adjacent 32-bit words, and finally stores a different 16-bit constant into a global variable.
- `setup_coprocessor_constants_set_one`: Sets a global variable to one, then writes a 16-bit constant and a value of one hundred into two adjacent 32-bit words, and finally stores a 16-bit constant into a global variable.
- `store_two_24bit_constants`: Stores two different 24-bit constants into two separate global variables, overwriting the first with the second constant.
- `init_blitter_slots_and_wait`: Initializes four short-value slots to their maximum value and sets two other global values, then waits until a hardware status register bit clears.
- `walk_linked_list`: Clears a global variable, then walks a linked list by following a pointer stored at a fixed offset in each node.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a6, then returns.
- `copy_adjusted_block`: Computes adjusted source and destination offsets and lengths from a pair of coordinates, then copies a block of words from a source array into a destination array while updating the destination pointer.
- `copy_array_and_adjust_pointer`: Copies ten 32-bit words and one 16-bit word from an input array into a global array, then adjusts a pointer derived from that array and assigns a fallback value when the pointer is null.
- `toggle_and_reload_table`: Toggles a global flag between two states and, depending on which state is active, initializes or reloads a set of four short-value slots in a table.
- `do_nothing_routine`: This routine contains no instructions and does nothing to any data.
- `initialize_global_if_empty`: It writes a value through a global variable only when that variable currently holds zero.
- `wait_for_flag_to_clear`: It repeatedly reads a global variable in a loop until that variable becomes zero.
- `copy_block_in_place`: Invokes another routine with two identical structure pointers and a fixed length value.
- `init_indexed_structure`: Computes a pointer by scaling a parameter by four and indexing into a global structure, then passes it along with the structure base to another routine.
- `init_global_structure`: Initializes a global structure by storing fixed default values at several fixed offsets, including a small 32-bit value from a parameter and several constant words.
- `copy_and_interleave_table`: Copies a block of 64-bit data from one table to another, then spreads the individual 16-bit words of that block across a destination table at a set of fixed offsets.
- `split_and_copy_array`: Splits a 32-bit value into its high and low halves, adds each to a global base, and shifts a second parameter left by four.
- `copy_array_and_adjust_pointer_2`: Copies ten 32-bit words and one 16-bit word from an input array into a global array, then adjusts a pointer derived from that array.
- `init_global_structure_again`: Initializes a global structure by storing fixed default values at several fixed offsets, including a 32-bit value taken from the structure itself.
- `clear_and_copy_blocks`: Clears a block of 16-bit words to zero and then copies that block into several other blocks in a loop.
- `set_global_fixed_value`: It always stores the same fixed 16-bit value into a global variable regardless of the branch taken.
- `store_default_value`: It writes the constant 32 to a global variable, taking one branch or the other depending on whether that variable currently holds zero, and then returns.
- `clear_global_table_blocks`: It clears two blocks of 16-bit words in a global table and then zeroes a 32-bit value at one of two addresses selected by the argument.
- `validate_and_clear_table_blocks`: Validates two input values against fixed ranges, then clears four word-sized slots in one of two table blocks depending on a flag and a selector.
- `build_size_lookup_table`: Computes several size and offset values from two input lengths and a flag table, then packs them into a byte lookup table along with a control byte.
- `dispatch_table_by_flags`: Reads a series of small flag fields from a table and, based on which flags are set, looks up and calls a corresponding routine from another table.
- `apply_bitwise_mask_pairs`: It combines two input 16-bit values with bitwise NOT and ANDs them into matching words at two different memory locations.
- `apply_bitwise_masks_to_memory`: Computes two bitwise masks from two input words and applies each mask to the same four word-sized locations in two different memory areas.
- `run_infinite_loop`: It is an empty infinite loop that never returns.
- `save_mask_disable_interrupts`: It increments a global counter, saves the current interrupt mask, and disables interrupts via the interrupt enable register.
- `toggle_flags_and_write_cia_register`: Tests and updates a global flag variable, conditionally invoking another routine, then writes a constant value to a global variable.
- `store_value_toggle_flags`: It stores an input value into a global variable, conditionally writes a byte to a hardware register, and toggles bits in a global flag table.
- `read_ports_update_global`: Reads the joystick and mouse port registers and updates a global variable based on their values.
- `read_ports_and_update_global_variable`: Reads the mouse and joystick port registers and updates a global variable based on their values.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_and_set_bits_in_global_variable`: Clears a bit in a global variable and sets another bit when a corresponding flag is absent, invoking another routine between the two operations.
- `set_clear_bits_in_bytes`: It manipulates individual bits in a set of adjacent memory bytes using a shift amount and an input value.
- `clear_memory_write_globals`: Clears a block of memory starting at a fixed address by repeatedly storing zero bytes until a counter reaches its limit.
- `fill_clipped_table_entries`: Clamps a rectangle defined by two corner coordinates into a fixed range, then writes a series of fixed values and computed offsets into a table of repeated entries.
- `fill_memory_block_with_pattern`: Fills a block of memory with a repeating pattern, writing several fixed values to specific offsets of each entry.
- `init_and_call_routine`: Initializes a global variable to zero, calls another routine three times, then branches on that variable to either call the routine once more or store a fixed value before a final call.
- `init_globals_and_call_loops`: Initializes a global variable to a fixed sequence of values, then repeatedly calls another routine a fixed number of times in two separate loops.
- `clear_memory_and_copy_string`: Clears a region of memory by writing a space byte to each location, then copies bytes from a source area into a destination area.
- `scan_buffer_against_table`: Walks through a buffer in fixed-size steps, comparing each entry against a table and returning early when a matching or empty entry is found.
- `copy_buffer_in_chunks`: Copies a block of 300 bytes from a source table at address a fixed address into a destination buffer, repeating until the destination pointer reaches a limit.
- `copy_table_data_recursive_2`: It indexes a table using a computed offset, then copies a variable number of bytes from the table into a destination buffer in a loop.
- `copy_table_data_recursive`: It divides an input value to compute offsets, copies data from a table into a buffer in a loop, and then calls itself recursively.
- `store_byte_into_global`: It stores a single fixed byte value into a global variable and returns.
- `write_constant_to_global`: Stores a fixed byte value into a global variable and returns.
- `read_disk_track_sectors`: Sets the disk sync word and starts disk DMA transfers into a buffer in a loop, computing the track position from the arguments.
- `find_and_decode_sectors`: Scans a raw disk buffer for sync words and decodes each sector that follows by merging its odd and even bit halves and checking an accumulated XOR. It records the pointer of each valid sector in a table indexed by sector number.
- `merge_odd_even_bits`: Combines two longwords by masking one with a fixed bit pattern and shifting the other left by one bit.
- `stub_return_stack_value`: Returns the value found at the top of the stack without changing it.
- `return_stack_value`: Returns the 64-bit value found at the top of the stack without modifying it, effectively passing the caller's argument straight back out.
- `seek_disk_head_track`: Steps the drive head one track at a time through the CIA port B register until the current track matches the requested one.
- `pulse_drive_control_line`: Sets one control bit, clears the bit below it and sets that bit again in the byte at the target address.
- `clear_and_latch_drive_bit`: Clears the second control bit and the first, then sets the first bit again in the byte at the target address.
- `reset_global_then_call`: Clears a global variable to zero and then calls another routine.
- `load_module_name_table`: Copies a list of name entries for the requested index into a buffer, cutting each entry at the text BODY and clearing four bytes after it.
- `start_audio_channel_dma`: Advances a frame counter and, on the first pass, writes the four audio channel pointer registers from a preset table with short delays between writes.
- `update_audio_channel_registers`: Reads one audio channel command block and takes from it a sample pointer, a length and a period.
- `compare_channel_target_value`: Copies a 12-bit value from a channel record into a field and sets a flag when it is below a stored target.
- `process_channel_effect_command`: Reads the effect nibble of a channel command and applies it to that channel's period or volume.
- `initialise_audio_channel_registers`: Initialises the first audio channel once by writing its length from a table and a fixed period, along with the volume state.
- `service_audio_irq_flag`: Checks bit seven of a flag and, when it is set, either marks the flag as pending or clears it and calls a subroutine.
- `set_record_flag_and_call`: Stores a fixed byte value into a field of the structure addressed by the A0 register and then calls another routine.
- `repeat_call_six_times`: Calls another routine six times in succession using a countdown loop.
- `infinite_compare_loop`: Spins forever, reading a byte at offset six from the A0 register and comparing it with itself, so the loop never exits.
- `set_audio_channel_period`: Clears a global variable, writes a fixed period value to the first audio channel period register and sets a control byte.
- `perform_signed_division`: Works out the sign of the result from the two operands and runs another routine on them.
- `divide_32_bit_unsigned`: Divides a 32-bit unsigned dividend by a divisor with a 16-step shift-and-subtract loop when the divisor is wide, or with two 16-bit divides otherwise.
- `divide_with_swapped_divisor`: Swaps the two halves of the second operand and runs a 16-step shift-and-subtract division of the first operand by it.
- `multiply_32_bit_values`: Multiplies two 32-bit values using their 16-bit halves and returns the low 32 bits of the product in the result.
- `infinite_empty_loop`: Loops forever with an empty body, so control never returns to the caller.
- `infinite_empty_loop_2`: Loops forever with no body in the decompiled code, so control never returns.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `stub_bad_instruction_data`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `stub_bad_instruction_data_2`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `stub_bad_instruction_data_3`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `copy_table_to_global`: It loads a table address and then copies 256 consecutive bytes from that table into a global variable through a pointer.
- `stub_bad_instruction_data_4`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `stub_bad_instruction_data_5`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `empty_routine`: An empty routine that performs no operations and returns without executing any instructions.
- `empty_routine_block`: The routine contains no executable instructions; it is an empty block of data with no code to describe.
- `mark_noncode_data`: This routine contains no executable instructions, only data that is not valid 68000 code.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `halt_on_baddata`: A routine that begins with a data word that is not a valid 68000 instruction, terminating control flow without executing any valid code.
- `stub_bad_instruction_data_6`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `mark_empty_routine`: The routine contains no executable instructions, only data that Ghidra flags as invalid code.
- `noop_routine`: This routine is empty and performs no operations on any data.
- `stub_bad_instruction_data_7`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `stub_bad_instruction_data_8`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `stub_bad_instruction_data_9`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `halt_baddata`: This routine contains no executable instructions, only data that the disassembler could not interpret as valid 68000 code.
- `stub_bad_instruction_data_10`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `stub_bad_instruction_data_11`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `mark_empty_routine_2`: This routine contains no instructions; it is an empty routine that does nothing.
- `stub_bad_instruction_data_12`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `stub_bad_instruction_data_13`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `stub_bad_instruction_data_14`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `stub_bad_instruction_data_15`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `load_table_value`: The routine begins by reading a value from a table using an index, then stores that value through a pointer.
- `stub_bad_instruction_data_16`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `stub_bad_instruction_data_17`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `stub_bad_instruction_data_18`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `stub_bad_instruction_data_19`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `empty_data_routine`: The routine contains no executable instructions, only data that the decompiler cannot interpret as valid 68000 code.
- `stub_bad_instruction_data_20`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `empty_routine_2`: The routine contains no executable instructions; it is an empty routine that performs no operations on any data.
- `store_invalid_data`: This routine contains no executable instructions, only data that the decompiler could not interpret as valid 68000 code.
- `empty_routine_3`: The routine contains no executable instructions, only data marked as invalid by the decompiler, so it performs no operations.
- `stub_bad_instruction_data_21`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `stub_bad_instruction_data_22`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `stub_bad_instruction_data_23`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `load_and_process_data`: The routine begins by loading a value from a memory location into a register using an absolute-move instruction, then continues executing a short sequence of instructions before the decompiler stops at a location containing data that is not valid machine code.
- `empty_data_block`: The routine contains no executable instructions, only data that the decompiler cannot interpret as code.
- `stub_bad_instruction_data_24`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `stub_bad_instruction_data_25`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `stub_bad_instruction_data_26`: The bytes here do not decode to a valid instruction, so there is no real routine.
- `mark_empty_region`: The routine contains no executable instructions, only data that the decompiler could not interpret as code.
- `is_non_code_data`: The routine contains only non-executable data rather than code, so it performs no operations on any data or registers.
- `mark_empty_data`: The routine contains no executable instructions, only data that the decompiler cannot interpret as valid 68000 code.
- `identify_bad_instruction`: The routine contains only a single instruction that is not valid 68000 code, so decompilation fails and no further instructions follow.
- `mark_empty_routine_3`: The routine contains no executable instructions; it is a completely empty routine.
- `identify_bad_instruction_2`: The routine contains only a single instruction that is not valid 68000 code, so decompilation fails and no further instructions follow, though another routine takes its address.
- `mark_empty_region_2`: The routine contains no executable instructions; the loader only reserves this region of memory.
- `identify_bad_instruction_3`: The routine contains only a single instruction that is not valid 68000 code, so decompilation fails and no further instructions follow, even though several audio and loading routines take its address.
- `identify_bad_instruction_4`: The routine contains only a single instruction that is not valid 68000 code, so decompilation fails and no further instructions follow, though an audio register routine takes its address.
- `stub_bad_instruction_data_27`: The bytes here do not decode to a valid instruction, so there is no real routine.

### logged disk load from Blues Brothers, The (1991)(Titus)[cr Company][t +3 Defjam].adf boot-block load

- `read_longword_and_shift`: Reads a longword from just before the A0 pointer and writes its low byte, paired with the high byte of the D7 word, to the destination word.
- `shift_bits_with_refill`: Shifts a register right by one bit per pass for a count taken from the low word of its argument.
