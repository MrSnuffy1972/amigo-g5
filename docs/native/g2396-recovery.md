# Game 2396 (SuperObliteration v1.1): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02396). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1098427 top-level bytes in 13 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 175 of 175 named (0 author symbols), 175 of 175 described (7 by rule, 165 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4398 of 4398 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 21248 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5787 of 6889 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 270119 |
| code | 26400 |
| data | 801866 |
| padding | 42 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| SuperObliteration | 4 | 6764 | 158 |
| game.slave | 1 | 91 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 5 |
| OSEmu.400 (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- SuperObliteration_v1.1/SuperObliteration_v1.1.hdf: hard-disk image (AmigaDOS OFS, 1184 blocks): its filesystem parses and all 6 files in it are byte-identical to the extracted tree SuperObliteration_v1.1_hdf, which is recovered file by file; no block outside the filesystem holds data
- 4 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- SuperObliteration_v1.1/SuperObliteration_v1.1_hdf/SuperObliteration/OSEmu.400: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 21248 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `execute_event_dispatch` (SuperObliteration): Retrieves function pointers from a table and executes them after updating several global variables and memory locations. Kind: loader/sabotage path. Pass path: The routine performs a series of indirect calls to system/hardware pointers (likely Amiga Exec/Custom Chip vectors) and then proceeds to overwrite a series of function pointers (pcRam00000064 through pcRam00000078) with a specific routine (.
- `fill_memory_block` (runtime segment an address): Writes a 32-bit value to four consecutive memory locations and modifies a global variable using bitwise operations. Kind: memory integrity/checksum trap. Pass path: execution continues to the next instruction after the memory fill/check.

## Functions


### SuperObliteration

- `execute_event_dispatch_2`: This routine is identical to the first routine and performs the same sequence of pointer dereferencing, global variable updates, and hardware register modifications.
- `execute_event_dispatch`: Retrieves function pointers from a table and executes them after updating several global variables and memory locations.
- `mask_interrupt_register`: Reads a value from a hardware register and stores the result with the highest bit cleared into a global variable.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `dispatch_input_event`: Polls the custom hardware status register for changes and processes an input event code by looking up a corresponding function pointer in a jump table.
- `dispatch_entity_event_logic`: Processes input or state events by looking up function pointers in a table and executing them.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `initialize_memory_blocks`: Sets several bytes and words at specific offsets in a memory block and clears a series of consecutive memory locations using a loop.
- `initialize_entity_state_structure`: Initializes a large data structure by setting numerous fields to zero or specific constant values, including several calls to other routines and a loop that clears a global variable array.
- `initialize_data_tables`: Clears a series of memory locations and initializes a table with specific values using a loop.
- `update_palette_and_data`: Swaps data within a table based on a comparison and copies bytes from a source to a destination while potentially updating color values.
- `fill_pattern_table`: Populates a table with repeating sequences of bytes in a loop.
- `update_control_registers`: Writes specific values to a series of memory locations based on the range of a provided unsigned short value.
- `update_entity_state_and_timers`: Updates entity status flags, decrements various countdown timers, and manages state transitions by resetting specific structure fields and calling other routines.
- `update_masked_value_2`: Fetches a byte from a table using a masked value from a memory offset and updates a secondary memory location.
- `load_masked_value`: Loads a byte from a table into a memory location combined with another value, provided a specific condition is met.
- `modify_masked_value`: Increments or decrements a word in a memory location based on bits in a byte, then masks the result to four bits.
- `decrement_masked_value`: Decrements a value in a memory location and applies a bitmask if a specific bit is set in a byte.
- `update_status_flags_2`: Checks bits in a byte to update a specific memory location and a word, then returns a masked value.
- `clear_byte`: Sets a single byte at a specific offset in a memory block to zero.
- `accumulate_masked_nibble_2_2`: Retrieves a byte from a table based on a masked nibble and stores it in a memory location.
- `accumulate_masked_nibble_2`: Writes a byte to a memory location and modifies a word at an offset using addition or conditional increments and decrements, then applies a four-bit mask to the result.
- `accumulate_masked_nibble`: Adds a byte from a pointer to a word in a memory location or increments it under specific conditions, then masks the result to four bits.
- `accumulate_and_mask_nibble`: Adds a byte from a memory location to a word stored at a pointer offset and then masks the result to keep only the lower four bits.
- `copy_structured_data`: Calculates an offset from two bytes and copies a block of data from that location into a destination structure.
- `process_joystick_input_logic`: Updates internal state flags and counters based on joystick button press patterns.
- `initialize_data_table`: Fills a table with three specific word values using a loop and sets a global byte to a constant value.
- `process_data_blocks`: Iterates through a table of structures, updating a counter in each and performing operations involving a global variable and other routines.
- `update_clamped_values`: Calculates a value based on the sum of two words, clamps it between two other stored values, and updates several offsets in a structure including an incremented value and a decremented version of the clamped result.
- `increment_byte_with_limit`: Increments a byte in a structure and resets it to zero if it exceeds a specified threshold.
- `update_structure_counters`: Increments a byte in a structure and updates two other word-sized fields based on the result and a threshold.
- `set_structure_value`: Sets a word in a structure to a specific value based on the state of a signed word and a flag.
- `accumulate_absolute_value`: Adds the absolute value of a word to a byte and wraps that byte by subtracting four if it exceeds three.
- `calculate_complex_offsets`: Computes two new values by adding elements from a global table and other memory locations to existing values, while also updating a byte and potentially setting a flag based on a character wrap-around.
- `dispatch_with_header`: Writes two words to a structure and calls another routine if two specific word fields are not negative one.
- `copy_structure_fields`: Writes two words to a structure and copies a pair of words to a different location within the structure based on a flag.
- `validate_and_update_state`: Checks if two short integer values at specific offsets within a structure are not negative one.
- `update_object_offset_coordinates`: Writes two constant values to a structure and then copies a pair of values from one memory location to another based on a conditional byte check.
- `initialize_structure_data`: Writes five words to a structure and stores a pointer to a global variable in it.
- `initialize_structure_data_alt`: Writes five words to a structure and stores a pointer to a global variable in it using different values.
- `increment_global_list_elements`: Iterates through a table of structures, checking flags and pointers, and increments a byte in each valid entry.
- `update_list_from_table`: Iterates through a table and copies word values into a structure based on a flag and the contents of the table entries.
- `update_sprite_list`: Iterates through a nested list of data structures and updates sprite-like attributes based on a flag.
- `process_object_list`: Iterates through a multi-level table of objects and calls another routine if specific coordinate and status conditions are met.
- `update_object_physics`: Iterates through a table to update sprite attributes and performs arithmetic on signed integer values within the data structures.
- `update_limited_objects`: Iterates through a list of five objects and calls another routine if their coordinate and status fields are valid.
- `update_entity_bounds`: Iterates through a list of five objects and updates their coordinate and bounding box data if they meet specific bitwise and value criteria.
- `update_parameters_alternate`: Checks if two short values are valid before writing specific constants to a structure and calling another routine.
- `update_object_parameters_2`: Writes constants to a structure and then updates one of two possible coordinate pairs based on a flag.
- `tiny_moveq_move`: Small routine of 2 instructions that loads 0 into d2, then copies 200 to d0.
- `update_entity_with_input`: Writes several constants to a structure and updates a coordinate pair using a value passed in a register.
- `validate_and_update_structure_fields`: Checks if two specific short integer values at an offset from a pointer are not equal to negative one.
- `update_parameter_with_offset`: Calculates a value by adding an offset to a global variable and writes it to one of two possible coordinate pairs along with a constant.
- `initialize_entity_data`: Stores a series of constant values into a data structure and sets a pointer to a global variable.
- `copy_memory_block`: Copies nine hundred bytes from a source pointer stored in a structure to a global destination buffer.
- `transform_and_copy_sprite_data`: Iterates through a source buffer to perform a mathematical transformation on each byte, then uses the resulting values as indices to copy multi-word data blocks from a lookup table into a destination buffer.
- `decrement_and_check_counter`: Decrements a byte at an offset from a pointer and performs conditional updates to several other offsets based on the result and global variables.
- `decrement_multi_byte_counter`: Decrements a three-tier counter consisting of a global variable and two local bytes.
- `update_data_from_table`: Iterates through a global variable and, if a byte differs from its successor, copies a block of data from a table based on a shifted index.
- `copy_nested_blocks`: Performs a nested loop to copy blocks of data from a source to a destination with a fixed stride.
- `expand_data_blocks`: Uses a nested loop to copy data from a table into a global variable, using an index derived from the values within the global variable.
- `process_and_update_attributes`: Calculates offsets based on an input, modifies a sequence of bytes, and then copies attribute data from a table for non-zero values.
- `load_attributes_from_index`: Loads a byte from an offset, uses it as an index into a table, and copies five consecutive bytes from that table into global memory.
- `load_attributes_by_digits`: Splits a byte into two digits and uses them to index into a table to copy multiple attribute blocks into a global variable.
- `batch_load_attributes`: Iterates through a null-terminated string and uses each byte to index into a table to copy attribute blocks into a destination buffer.
- `modify_status_byte`: Sets a byte to a specific value based on a comparison of a word, and may increment or decrement another byte before masking it to a single bit.
- `update_offset_tables_core`: Calculates a delta based on byte-level arithmetic and applies it to two separate tables of 16-bit words.
- `conditionally_update_global_byte`: Checks a byte at a specific offset within a structure; if it is zero, it updates a global variable by combining two bytes from the structure into a single value and then calls itself twice recursively.
- `calculate_pointer_offsets`: Calculates a value based on a table entry and several offsets, then stores a pointer and several bytes from a derived memory location into a structure.
- `compare_memory_blocks`: Compares three consecutive memory blocks of a specific size and, if they do not match and certain flags are zero, sets a flag and two integer values.
- `update_entity_state_and_attributes`: Checks if an entity's position is within a specific bounding box and then updates its internal state, attributes, or properties based on its current type.
- `adjust_position_values`: Adjusts two 16-bit values based on a comparison between a signed value and another 16-bit value, then clears several bytes and sets a specific byte to one.
- `check_table_conditions`: Checks a byte in a table and a secondary condition to set a byte to one or zero based on the results.
- `scan_collision_area`: Iterates through a 2D grid or bounding box defined by coordinate offsets to check for non-zero values in a data table.
- `modify_coordinate_values`: Adjusts a 16-bit value and a global variable based on the sign of an input, then clears two 16-bit values in a structure.
- `update_status_flags`: Modifies a 16-bit value in a structure and, if a specific flag is zero, resets several bytes and sets a flag to one.
- `check_character_match`: Checks if a character at a specific memory location is an exclamation mark or a double quote, and if so, sets a flag and two integer values.
- `copy_data_blocks`: Iterates through a list of pointers to copy blocks of data from one table to another, incrementing an index and clearing a pointer when a limit is reached.
- `validate_collision_mask_and_update_state`: Iterates through a list of object structures to verify their spatial offsets against a collision bitmask or table.
- `check_entity_collisions_2`: Iterates through nested lists of data structures to perform bounding box intersection tests and updates status flags accordingly.
- `update_entity_animation_state`: Calculates a new animation frame index based on a timer and spatial bounds, then updates the entity's state byte and animation metadata.
- `update_entity_attribute_table`: Iterates through a list of data structures to update spatial coordinates and status flags.
- `adjust_table_values`: Increments two short values based on a specific byte value and then updates a table of values using a loop.
- `process_data_loop`: Iterates through a loop to update a table of values using a random number generator and toggles a state flag.
- `update_entity_counters`: Iterates through a table of structures to update a byte field based on comparisons between a value in the structure and a global threshold.
- `iterate_entity_attributes`: Loops through a table of data structures and calls another routine if a specific byte in the structure is non-zero.
- `copy_memory_block_2`: Copies a block of data from a source address to a destination address using a loop.
- `clear_memory_block`: Fills a block of memory with zeros.
- `initialize_entity_table`: Populates a table with specific constant values and sets two word fields to all ones.
- `populate_entity_attribute_table`: Iterates through a source data structure to populate a table of entity attributes, copying specific fields such as coordinates, status, and type.
- `sync_entity_data`: Copies several fields from one set of structures to another, occasionally resetting pointers and a word value if a specific byte is non-zero.
- `reset_entity_fields`: Clears multiple specific byte offsets within a series of repeating data structures.
- `process_entity_logic`: Performs arithmetic on structure fields and updates a pointer table, potentially resetting specific bytes in a structure if a condition is met.
- `adjust_entity_positions`: Updates two different coordinate fields in a table of structures by adding a signed value and incrementing or decrementing a counter based on the result.
- `update_entity_position_and_collision`: Iterates through a list of entities to update their spatial coordinates using relative offsets and checks for collisions against a data structure.
- `decrement_coordinate_x`: Subtracts a value from a word at a specific offset in a structure, handling wrap-around for negative values and setting a flag.
- `decrement_coordinate_y`: Subtracts a value from a word at a different offset in a structure, handling wrap-around for negative values and setting a flag.
- `update_counters_in_table`: Iterates through a table of structures, decrementing a counter and incrementing two other values if certain conditions are met.
- `process_table_entries`: Processes a list of entries by performing calculations based on table values and updating a global structure or memory location.
- `clear_memory_block_2`: Fills a specified block of memory with zeros using a loop.
- `read_joystick_state`: Reads the CIAA_PRA register and combines it with a global variable to return a byte representing button states.
- `update_random_seed`: Updates a global variable by multiplying it by two or performing an XOR operation with a constant.
- `update_game_state_and_parse_data`: Updates several state variables within a structure based on specific conditional checks and decrements a counter.
- `set_hardware_pointers`: Writes the address of a global variable and a constant into the hardware registers at a fixed address.
- `copy_data_from_string`: Uses a string as an index to copy bytes from a table into a destination buffer.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `populate_split_word_table`: Splits a 32-bit value into two 16-bit components and stores them in a table.
- `store_long_values_wide`: Splits a 32-bit value into two 16-bit parts and stores them in a table, advancing the pointer by eight words per iteration.
- `store_long_values_narrow`: Splits a 32-bit value into two 16-bit parts and stores them in a table, advancing the pointer by four words per iteration.
- `update_object_position_and_offset`: Calculates a new coordinate value using a combination of register data and table lookups, then writes this value along with a secondary offset into a structure.
- `update_entity_position_and_bounds`: Calculates new coordinate and bounding box values for an object using scaling factors and offsets from a data table.
- `update_sprite_attribute_list`: Calculates new coordinate and attribute values for a set of sprite-like objects and writes them to a data structure.
- `update_object_parameters`: Calculates a value based on a table and several inputs, then writes multiple fields to a structure pointed to by a register, including three sequential updates to a specific integer field.
- `update_object_parameters_alternate`: Calculates a value based on a table and several inputs, then writes multiple fields to a structure pointed to by a register, with different initial values for some fields compared to another routine.
- `decrement_value_loop`: Calls another routine and decrements a value through a pointer based on several byte checks.
- `increment_value_loop`: Calls another routine twice and increments a value through a pointer based on comparisons with global variables and table entries.
- `decrement_byte_at_offset`: Decrements a byte located at a specific offset from a pointer stored in a register.
- `extract_nibbles_from_pointer`: Extracts a low nibble from a byte at a pointer and stores it in a global variable, then splits the second byte of that pointer into two nibbles stored in separate global variables.
- `extract_nibbles_from_pointer_alt`: Extracts a low nibble from a byte at a pointer and stores it in a global variable, then splits the second byte of that pointer into two nibbles stored in separate global variables using a different pointer.
- `initialize_entity_data_entry`: Iterates through a list of entities to initialize their data structures, calculating specific attribute offsets and copying large blocks of state data from a source table into a destination table.
- `update_entity_collision_and_velocity`: Checks an entity's position against boundary constraints and sprite dimensions to set collision flags.
- `find_minimum_area`: Iterates through a table of structures to find the one with the smallest calculated area based on width and height fields, then sets a flag in that structure and returns a value from it.
- `encode_sprite_data`: Iterates through a table to calculate memory offsets and encode bitwise data into a series of destination buffers.
- `initialize_game_assets`: Initializes system components by calling initialization routines, clearing memory blocks, and copying data from a global table into specific memory locations.
- `call_initialize_twice`: Calls another routine two times in succession.
- `call_data_initialization`: This routine calls another routine that populates a series of eight-byte blocks with a repeating sequence of specific byte values.
- `execute_loop_and_add_constant`: Executes a routine within a loop three times and returns the result of the last call incremented by a constant.
- `initialize_data_blocks`: Fills a series of eight-byte blocks with a repeating sequence of specific byte values.
- `process_joystick_and_menu_input`: Reads joystick movement and button states from hardware registers to update input structures and manage menu navigation.
- `check_and_update_status`: Checks if a 16-bit value at a specific offset equals four, and if so, changes it to seven.
- `reset_status_value`: Checks if a value at a specific offset in a structure is 4, 5, or 6, and if so, sets it to 0.
- `set_global_on_interrupt`: Checks a bit in a hardware register and sets a global variable to a specific value if the bit is set.
- `store_split_longword`: Splits a longword into two separate words and stores them into a table at increments.
- `copy_indexed_data`: Iterates through a string to calculate an index and copies blocks of data from a global table to a destination.
- `update_input_state`: Reads bits from a hardware register and a structure to construct a byte value, then updates a structure with new word values.
- `load_sprite_data`: Calculates an offset based on a character value and copies multiple words of data from a source table to a destination table.
- `copy_block_data`: Performs a nested loop to copy data from a source memory location to a destination memory location.
- `expand_directional_data`: Calls another routine to process data related to directional control.
- `expand_joystick_data`: Calls another routine to process data related to a joystick button.
- `expand_start_game_text`: Calls another routine to process text related to starting the game.
- `expand_generic_text`: Calls another routine to process text data.
- `expand_game_end_text`: Calls another routine three times to process text related to game completion or continuation.
- `populate_character_graphics`: Iterates through a string to index into a global table and copies sixteen words of data for each character encountered.
- `expand_character_data`: Reads a string and uses each character to index into a global table, copying 16 words of data for each character.
- `decrement_value_based_on_data`: Iterates through a data structure to decrement a word value based on specific byte patterns found at offsets from a pointer.
- `increment_byte_with_wrap`: Increments a byte at a specific memory location and wraps it to 0 if it reaches 3.
- `increment_value_based_on_data`: Compares bytes from a data structure and global variables to increment a word value based on specific conditions.
- `decrement_byte_with_wrap`: Decrements a byte at a specific memory location and wraps it to 2 if it falls below 0, while also decrementing an adjacent byte.
- `extract_nibbles_from_pointer_2`: Extracts a nibble from the first byte of a memory location into a global variable and splits the second byte into two nibbles stored in separate global variables.
- `extract_nibbles_from_pointer_3`: Reads a byte from a pointer to store its lower nibble in a global variable and stores the upper and lower nibbles of the following byte into two other global variables.
- `initialize_music_system`: Walks a byte table to build a pointer table for music samples, clears several global variables, and resets various counters and values.
- `clear_audio_dma`: The routine writes to the four audio volume registers and to the DMA control register, setting the audio channels off and clearing DMA enables.
- `process_audio_parameters`: Updates audio hardware registers and memory locations by iterating through a loop and calling routines to set channel parameters.
- `update_audio_channel_parameters`: Updates audio hardware control structures by fetching sample data, pitch, and volume parameters from a lookup table based on a command byte.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `update_control_structure_state`: Updates a control structure by performing conditional arithmetic, bitwise masking, and pointer manipulation based on a command byte.
- `execute_callback_sequence`: Calls several functions via pointers stored in a global structure and performs a series of register and memory updates involving a nested function call.
- `perform_complex_dispatch`: Retrieves a pointer from a structure, executes a sequence of indirect function calls, and updates multiple memory locations with saved state before calling another routine.

### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `initialize_hardware_registers`: Retrieves a pointer from a table and writes specific constant values to various offsets within a structure.
- `update_color_and_xor`: Sets a global variable to a specific value and performs a bitwise XOR operation on a memory location.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `store_address_and_constants`: Stores the address of a global variable into itself and writes several constant values to global variables.
- `set_interrupt_handler`: The routine writes a pointer to another routine into a global variable.
- `update_interrupt_state`: Modifies a global variable and performs bitwise operations on a byte from memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `empty_routine`: This routine contains no executable instructions.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_memory_block`: Writes a 32-bit value to four consecutive memory locations and modifies a global variable using bitwise operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_memory_values`: Decrements a byte at a specific offset within a structure and updates several fields in a table using values from pointers and other memory locations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_word_to_memory`: Copies a word from one memory location to another.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `return_immediately`: This routine is empty and performs no operations before returning to the caller.
- `nop_placeholder`: This routine is empty and performs no operations.
- `return_no_operation`: The routine consists of a single instruction that immediately returns to its caller, with no other reads, writes, library calls or hardware register accesses.
- `do_nothing`: the routine is empty and performs no operations before returning.
