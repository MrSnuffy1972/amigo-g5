# Game 1078 (ruff 'n' tumble (1994)(renegade)[cr pdx](disk 1 of 2)[t +17 atx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01078). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (906000 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 85 of 85 named (0 author symbols), 85 of 85 described (2 by rule, 71 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 701 of 701 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 1024 bytes, 7 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1606 of 1644 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 14620 |
| code | 8374 |
| data | 882321 |
| padding | 173 |
| unused | 512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Ruff 'N' Tumble (1994)(Renegade)[cr PDX](Disk 1 of 2)[t +17 ATX].adf:bootblock | 1 | 40 | 2 |
| logged disk load from Ruff 'N' Tumble (1994)(Renegade)[cr PDX](Disk 1 of 2)[t +17 ATX].adf boot-block load | 1 | 205 | 10 |
| logged disk load from Ruff 'N' Tumble (1994)(Renegade)[cr PDX](Disk 1 of 2)[t +17 ATX].adf boot-block load | 1 | 107 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 618 | 40 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 431 | 24 |
| Ruff 'N' Tumble (1994)(Renegade)[cr PDX](Disk 1 of 2)[t +17 ATX].adf bytes 35328-36352 (code never loaded by a reference run) | 1 | 243 | 7 |

## Cases handled

- ruff 'n' tumble (1994)(renegade)[cr pdx](disk 1 of 2)[t +17 atx]/Ruff 'N' Tumble (1994)(Renegade)[cr PDX](Disk 1 of 2)[t +17 ATX].adf: custom-format (non-DOS) disk, 1754 non-empty blocks: matched against track-loaded segments
- ruff 'n' tumble (1994)(renegade)[cr pdx](disk 1 of 2)[t +17 atx]/Ruff 'N' Tumble (1994)(Renegade)[cr PDX](Disk 1 of 2)[t +17 ATX].adf: the boot block reads 5632 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- ruff 'n' tumble (1994)(renegade)[cr pdx](disk 1 of 2)[t +17 atx]/Ruff 'N' Tumble (1994)(Renegade)[cr PDX](Disk 1 of 2)[t +17 ATX].adf: the boot block reads 10240 bytes from disk offset 890880 to an address (found statically); recovered as a unit
- runtime segment an address (play-long-joy): 306 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 395 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ruff 'n' tumble (1994)(renegade)[cr pdx](disk 1 of 2)[t +17 atx]/Ruff 'N' Tumble (1994)(Renegade)[cr PDX](Disk 1 of 2)[t +17 ATX].adf: bytes 35328-36352 look like code no run loaded: recovered position-independently (load address unknown)
- ruff 'n' tumble (1994)(renegade)[cr pdx](disk 1 of 2)[t +17 atx]/Ruff 'N' Tumble (1994)(Renegade)[cr PDX](Disk 1 of 2)[t +17 ATX].adf: 3 logged loads cover 16896 bytes of the image
- ruff 'n' tumble (1994)(renegade)[cr pdx](disk 1 of 2)[t +17 atx]/Ruff 'N' Tumble (1994)(Renegade)[cr PDX](Disk 1 of 2)[t +17 ATX].adf: 7 track-loaded blocks found in recovered runtime segments and classified through them
- ruff 'n' tumble (1994)(renegade)[cr pdx](disk 1 of 2)[t +17 atx]/Ruff 'N' Tumble (1994)(Renegade)[cr PDX](Disk 1 of 2)[t +17 ATX].adf:blocks 173-1739 (bytes 88576-890880)+92672#unpacked: high-entropy data file (7.83 bits/byte), classified as packed data
- ruff 'n' tumble (1994)(renegade)[cr pdx](disk 1 of 2)[t +17 atx]/Ruff 'N' Tumble (1994)(Renegade)[cr PDX](Disk 1 of 2)[t +17 ATX].adf:blocks 173-1739 (bytes 88576-890880)+414208#unpacked: high-entropy data file (7.82 bits/byte), classified as packed data
- ruff 'n' tumble (1994)(renegade)[cr pdx](disk 1 of 2)[t +17 atx]/Ruff 'N' Tumble (1994)(Renegade)[cr PDX](Disk 1 of 2)[t +17 ATX].adf: blocks 173-1739 (bytes 88576-890880) hold 3 packed streams (RNC1: Rob Northen RNC1 Compressor) found by scanning, 93% of the bytes
- 3 packed track region(s), 76800 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Ruff 'N' Tumble (1994)(Renegade)[cr PDX](Disk 1 of 2)[t +17 ATX].adf:bootblock

- `alloc_load_and_jump`: Boot-block entry that allocates a buffer, reads blocks into it with exec I/O requests until the read flag clears, stores a fixed value in the loaded code and jumps there; returns minus one if the allocation fails.
- `load_two_blocks_and_call`: Issues two exec disk reads into fixed memory areas, calls one unnamed routine between them, and then calls the code at the second destination.

### logged disk load from Ruff 'N' Tumble (1994)(Renegade)[cr PDX](Disk 1 of 2)[t +17 ATX].adf boot-block load

- `copy_longwords_and_relocate_code`: Copies a run of longwords from a source area to a destination derived from a global value, then copies a small block of its own code into place.
- `unpack_literal_and_repeat_runs`: Unpacks a compressed stream by alternating literal byte runs with repeated copies of earlier output, using counts from the bit reader, then copies ten words onto the stack and jumps through the address left there.
- `unpack_literal_and_repeat_runs_2`: Variant of the neighbouring unpacker: copies a literal run, then repeats copies of earlier output for a counted number of blocks, and when the block counter ends copies ten words onto the stack and jumps.
- `decode_table_entry_bits`: Scans word pairs until one matches the masked value, then combines a shift count from the matching entry with a bit taken from a mask to build the returned value, calling another routine on a borrow condition.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 255 into d0, then loads 16 into d1.
- `mask_and_conditional_call`: Masks one word with another, and when one byte is less than another it calls another routine and returns that routine's word instead of the masked value.
- `return_empty_routine`: The routine is empty; it contains no instructions that modify memory or registers and immediately returns.
- `accumulate_32bit_from_bytes`: Reads four consecutive bytes from a pointer and concatenates them into a single 32-bit value returned in a data register.
- `decode_and_fill_table`: Reads a count and that many bytes, then for each code length writes a bit mask and a derived value into table entries whose bytes match, building a lookup table for the bit reader.
- `copy_words_stack_and_jump`: Copies ten words from a global area onto the stack and jumps through the address left there.

### logged disk load from Ruff 'N' Tumble (1994)(Renegade)[cr PDX](Disk 1 of 2)[t +17 ATX].adf boot-block load

- `refill_bit_reader_word`: Takes the longword just before the source pointer, stores its low byte together with a saved high byte as an output word, and returns the longword shifted right by one with its top bit set.
- `skip_bits_with_refill`: Shifts a bit buffer right by a requested count, calling the refill routine whenever the buffer runs out and reloading its counter from the result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_global_and_pointer`: Stores a word from a pointer into a memory location and clears a global variable.
- `increment_and_reset_variable`: Increments a global variable if it is non-zero and then resets that variable to zero.
- `reset_global_variable`: Checks if a global variable is non-zero and performs arithmetic operations on it.
- `update_state_sequence`: Checks a global byte and, if zero, performs a series of updates to several global variables and calls another routine multiple times with varying values stored in a table.
- `build_dma_descriptor_list`: Constructs a linked list of data structures by traversing an existing list and then configures the Amiga DMA controller registers to enable specific DMA channels.
- `update_circular_buffer_2`: Stores a byte from a register into a memory table at an offset determined by a masked value in another register.
- `traverse_linked_list_nodes_2`: Increments a global counter, recursively traverses a linked list of structures by checking null pointers, and then decrements the counter.
- `update_circular_buffer`: Shifts four consecutive bytes in a memory table and inserts a new byte derived from a masked input value.
- `traverse_null_terminated_string`: Inverts a boolean flag and then iterates through a sequence of memory locations, calling another routine if a non-zero byte is encountered.
- `initialize_dma_channels`: Initializes a data structure by traversing a linked list and then configures several DMA channels by writing specific values to the DMA control registers.
- `fill_memory_with_zero`: Fills a block of memory with zeros using a loop that runs eighty-seven times.
- `update_status_flags`: Sets several global bytes based on the lower bits of a provided integer.
- `traverse_linked_list_nodes`: Increments a global counter, then recursively traverses a linked list of structures by checking null pointers at specific offsets.
- `update_object_properties`: Updates a table of data by reading from a pointer and writing multiple values to a calculated offset.
- `update_bitfield_and_structure`: Checks a bit in a global variable to trigger a bitwise update to another global variable and performs a series of memory writes to a calculated offset.
- `process_conditional_logic`: Checks specific bytes within structures and executes a series of routines based on the state of those bytes and pointers.
- `process_flag_and_pointer`: Reads a byte at offset 57 from a structure, clears its lowest bit, and if the bit was set, updates a pointer and copies data from offsets 22 and 12 within the structure.
- `check_flag_and_set_status`: Reads a byte at offset 56 from a structure, clears its lowest bit, and if the bit was set, sets a byte at offset 57 to 255 and updates a word at offset 150 in another structure with a value from offset 46 ORed with 33280.
- `execute_bytecode_opcode`: Interprets a single instruction from an encoded data stream by reading an opcode and performing corresponding actions.
- `interpret_bytecode_stream`: Iterates through an encoded data stream to execute virtual instructions by updating a state structure.
- `execute_bytecode_stream`: Iterates through a stream of encoded commands, performing conditional jumps and updating a state structure.
- `parse_bytecode_stream_2`: Iterates through a data stream to interpret encoded commands, updating a state structure and performing conditional jumps or pointer arithmetic based on the opcode values.
- `process_bytecode_stream`: Iterates through a stream of encoded commands, performing conditional jumps and updating internal state structures.
- `interpret_bytecode_stream_2`: Iterates through a stream of encoded bytes to perform table lookups and pointer arithmetic.
- `interpret_bytecode_stream_3`: Reads and executes a sequence of encoded instructions from a data stream.
- `interpret_encoded_bytecode`: Iterates through a data stream to execute a series of commands by reading opcodes and updating a structure.
- `parse_bytecode_stream`: Iterates through a data stream to interpret encoded instructions, performing pointer arithmetic and table lookups based on the operand values.
- `update_scaled_values`: Performs scaling, clamping, and arithmetic operations to update values within a structure or table.
- `scale_and_clamp_value`: Calculates a scaled value using multiplication and bit-shifting, clamps the result between certain limits, and stores it in a global or table entry.
- `increment_index_and_scale`: Updates a value in a structure using a multiplier from a pointer and increments an index or resets it using a value from a table.
- `update_structure_value`: Calculates a new byte value by subtracting a product of two bytes divided by a third byte from a fourth byte, then stores it in a structure.
- `reset_byte_flag`: Stores a zero byte into a specific offset within a structure.
- `initialize_structure_data`: Copies values from a source structure into a destination structure, including a byte, a subtraction result, and two other bytes, then sets a flag to two.
- `set_word_constant`: Stores the constant value sixty-four into a word at an offset within a structure.
- `reset_structure_bytes`: If a pointer in a structure is non-zero, it updates several bytes at specific offsets within that structure using values from the pointed-to memory.
- `update_position_and_counter`: Updates a pointer or index using a scaling calculation, decrements a counter, and updates a third value using an addition or pointer-based lookup.
- `setup_sprite_data`: Updates several fields in a structure, including a pointer, a byte, and several short values, based on data from a table.
- `load_table_values`: If a byte in a structure differs from a provided value, it updates that byte and copies multiple fields from a table into the structure.
- `clear_flag_and_update_ptr`: Clears the least significant bit of a byte flag.
- `clear_byte_flag`: Clears the least significant bit of a byte and sets another byte to a specific value if the bit was previously set.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_data_tables`: Sets a global variable, calls two other routines, and populates a series of memory locations with a repeating constant value.
- `copy_memory_block`: Copies thirty-one words from a source pointer to a destination pointer and then calls two other routines.
- `parse_and_copy_data`: Iterates through a null-terminated list of pointers to parse encoded strings and copy structured word data into a global structure.
- `modify_global_and_call`: Calls another routine and then modifies a global variable by performing a bitwise AND operation.
- `parse_encoded_string`: Iterates through a string to interpret encoded characters, performing different calculations or calls based on the character value and storing the results in global variables.
- `find_null_terminator`: Iterates through a null-terminated string until it reaches a zero byte.
- `configure_dma_transfer`: Sets up multiple DMA control parameters and waits for a status bit in a global variable to change across several iterations.
- `populate_table_with_calls`: Repeatedly calls another routine to fill a table with returned words and modifies a global variable.
- `populate_table_with_calls_2`: Executes a loop that calls another routine multiple times to store results in a table and sets a global variable.
- `initialize_table_and_dma`: Sets a global variable, enables DMA channels, and executes a loop that calls another routine to populate a table.
- `copy_words_to_table`: Copies thirty-one words from another routine into a destination table.
- `fill_table_with_words`: Calls another routine one hundred and ninety-nine times and stores each returned word into a table at a specific offset.
- `copy_words_from_routine`: Calls another routine thirty-one times and stores the returned word into a destination table.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `nop_operation`: The routine is empty and performs no operations before returning.
- `initialize_data_table`: Copies a block of data from a global variable and then populates a memory area with a repeating pattern of structured multi-byte values.
- `copy_structured_data`: Copies a series of words from a source pointer to a destination offset within a global structure, adjusting the destination index based on an input value.
- `update_offset_table`: Stores a value and several increments of that value plus a constant into various locations within a global structure.
- `decompress_bitstream_data`: Reads a compressed bitstream using a bit-reader and expands it into a destination buffer.
- `initialize_data_structures`: Calls several routines to initialize data tables, enable DMA channels, and copy a block of words.
- `copy_word_block`: Copies a block of thirty-one words from a source pointer to a destination pointer and calls two other routines.
- `clear_memory_blocks`: Copies a value from a global variable and then performs a nested loop to zero out multiple blocks of memory.
- `wait_for_bit_toggle`: Enters a loop that waits for a specific bit in a global variable to clear, followed by a loop that waits for it to set.

### Ruff 'N' Tumble (1994)(Renegade)[cr PDX](Disk 1 of 2)[t +17 ATX].adf bytes 35328-36352 (code never loaded by a reference run)

- `bitwise_or_store`: Performs a bitwise OR between a word and a byte, then stores the result into a structure.
- `calculate_scaled_value`: Performs a series of shifts and multiplications on values from a structure to calculate a result, which is then clamped between zero and a maximum value.
- `update_index_and_value`: Updates a value in a structure using a multiplication and shift based on a byte from another memory location, then increments an index or wraps it around.
- `decrement_and_call_routine`: Decrements a value in a structure using a calculation involving three other bytes, and calls a function via a pointer if a counter reaches zero.
- `reset_structure_parameters`: Clears several bytes in a structure and initializes them using values from a pointer stored within that structure.
- `load_character_data`: Loads character data from a table into a structure and updates several related fields, including a pointer and specific byte values.
- `update_pointer_and_attributes`: If a value has changed, it calculates a new pointer based on an index and copies multiple attributes from that pointer's location into a structure.
