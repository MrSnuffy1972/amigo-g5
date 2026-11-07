# Game 1300 (pang (1990)(ocean)[cr hz]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01300). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (970176 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 127 raw-word lines |
| Functions named and described | pass, 152 of 152 named (0 author symbols), 152 of 152 described (6 by rule, 139 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4771 of 4771 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9270 of 10503 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 180633 |
| code | 39062 |
| data | 731109 |
| padding | 940 |
| unused | 18432 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Pang (1990)(Ocean)[cr HZ].adf:bootblock | 1 | 29 | 2 |
| logged disk load from Pang (1990)(Ocean)[cr HZ].adf boot-block load | 1 | 86 | 2 |
| logged disk load from Pang (1990)(Ocean)[cr HZ].adf boot-block load | 1 | 90 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7596 | 100 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2250 | 30 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 286 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 166 | 3 |

## Cases handled

- pang (1990)(ocean)[cr hz]/Pang (1990)(Ocean)[cr HZ].adf: custom-format (non-DOS) disk, 1722 non-empty blocks: matched against track-loaded segments
- pang (1990)(ocean)[cr hz]/Pang (1990)(Ocean)[cr HZ].adf: the boot block reads 23552 bytes from disk offset 859136 to an address (found statically); recovered as a unit
- pang (1990)(ocean)[cr hz]/Pang (1990)(Ocean)[cr HZ].adf: the boot block reads 132096 bytes from disk offset 727040 to an address (found statically); recovered as a unit
- runtime segment an address: 3612 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 747 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 250 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 162 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- pang (1990)(ocean)[cr hz]/Pang (1990)(Ocean)[cr HZ].adf: 2 logged loads cover 155648 bytes of the image
- 1 packed track region(s), 726016 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `reset_hardware_and_game_state` (runtime segment an address): Resets the Amiga hardware registers, including color, DMA, and interrupt settings, while clearing global variables and entity tables. Kind: loader/initialization with hardware reset. Pass path: the routine continues to the subsequent initialization calls (u4__sub_3904, etc.) after clearing the hardware/state registers.
- `iterate_pointer_list_and_update_globals` (runtime segment an address): Iterates through a list of data structures until a negative value is encountered. Kind: pointer list iteration for data/code validation. Pass path: the loop continues to the next pointer in the list.

## Functions


### Pang (1990)(Ocean)[cr HZ].adf:bootblock

- `initialize_hardware_and_copy_data`: Clears a range of hardware registers and performs two memory block copies using specific source and destination addresses.
- `copy_memory_block`: Copies a block of data from a source table to a destination memory address using a loop.

### logged disk load from Pang (1990)(Ocean)[cr HZ].adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from Pang (1990)(Ocean)[cr HZ].adf boot-block load

- `decompress_bitstream_data`: Decompresses a bit-packed stream into a destination buffer using a variable-length coding scheme.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_hardware_and_game_state`: Resets the Amiga hardware registers, including color, DMA, and interrupt settings, while clearing global variables and entity tables.
- `copy_data_blocks`: Copies multiple blocks of data from a source table to two different destination buffers based on a global index.
- `reset_game_state_and_hardware`: Resets multiple global variables, hardware registers, and memory structures to zero or default values.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `set_global_constant_2`: Stores a constant value into a global variable and modifies a hardware color register.
- `enable_dma_and_set_global`: Enables DMA channels by writing to the DMA controller and updates a global variable with a specific constant value.
- `update_register_state`: Modifies a global variable and writes a bit-inverted, shifted value to a memory location, followed by a delay loop and a final register or memory update.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `manage_entity_attribute_tables`: Updates and populates structured memory buffers by iterating through data tables and writing entity properties, coordinates, and state flags.
- `reset_palette_and_decrement_counter`: Sets a color register to white, clears several global variables, and executes a sequence of subroutine calls.
- `update_memory_values`: Checks a bit in a memory location and, if not set, increments a global variable and writes a specific sequence of bytes to a new memory location.
- `update_digit_counters`: Extracts decimal digits from a global value and updates a series of counters and stored digits when changes are detected.
- `write_attribute_list`: Calculates offsets based on an input value and writes a series of multi-byte values and integers into a structured list or buffer.
- `append_data_sequence`: Writes a specific sequence of bytes to a memory location determined by an offset and increments that offset by eight.
- `append_data_sequence_2`: Writes a different specific sequence of bytes to a memory location determined by an offset and increments that offset by eight.
- `call_indirect_via_table`: Calculates an offset into a table of pointers and calls the resulting subroutine.
- `decrement_and_update_list`: Decrements a global counter and, if it reaches zero, updates a pointer and a list index.
- `execute_conditional_updates`: Checks a global byte and calls four different routines, some of which may be called multiple times depending on the condition.
- `decrement_and_update_timer_state`: Decrements a counter and, if it reaches zero, resets it using a value from a table while advancing a secondary pointer.
- `scan_entity_list_for_collision`: Walks a linked list of entity records, skipping inactive ones, and tests each entity's stored bounding box (position plus a size chosen by an entity type code of 8, 16, or 32) against a rectangle built from a global base position.
- `check_entity_collision_bounds`: Iterates through a table of entity pointers to perform bounding box intersection tests against a target area defined by a global structure.
- `check_collision_with_entities`: Iterates through a table of entity pointers to perform bounding box intersection tests against a target area.
- `check_collision_and_update_state`: Iterates through a table of structures to check for rectangular overlaps and updates specific values in a table and global variables based on the results.
- `check_collision_bounds`: Iterates through a table of structures to check if a specific rectangular area overlaps with the bounds defined in the table entries.
- `update_conditional_value`: Checks if a global variable is zero or if another global variable does not equal two, then stores a constant value into a memory location.
- `execute_jump_table_entry`: Compares a value in a table with a global variable, increments a global counter, and then calls a function pointer retrieved from a jump table.
- `spawn_and_update_entity`: Manages the creation and lifecycle of entities by allocating entries from tables and updating their properties.
- `manage_object_list_and_timers`: Decrements a counter and iterates through a linked list of objects to update their timing and state.
- `initialize_data_structure_2`: Stores values from memory into several global variables and sets a specific global variable to a constant value.
- `initialize_data_structure`: Stores the address of a global variable and several constant values into a block of memory starting at an offset from a pointer.
- `resolve_entity_collision_and_velocity`: Checks an entity against a table of other entities for bounding box intersections and updates its position and velocity based on the collision results.
- `update_entity_physics_and_collisions`: Updates an entity's position and velocity based on movement logic and checks for bounding box intersections against a list of other entities.
- `check_collision_with_entities_2`: Iterates through a linked list of entity structures to perform bounding box intersection tests against a target area.
- `push_object_data`: Checks two global conditions and then appends a multi-byte data block to a table using an incrementing offset.
- `push_data_to_table`: Increments a counter in a global variable and stores a four-byte value, a second integer, and a byte into a table at a calculated offset.
- `clear_multiple_globals`: Sets three global variables to zero and clears a byte at a memory location pointed to by a register.
- `clear_multiple_globals_abs`: Sets three global variables to zero using an absolute address and clears a byte at a memory location pointed to by a register.
- `clear_multiple_globals_pcrel`: Sets three global variables to zero using a relative address and clears a byte at a memory location pointed to by a register.
- `clear_multiple_globals_2`: Resets three global variables to zero and clears a byte at a memory location pointed to by a register.
- `set_partial_state`: Sets a global variable to three and clears a byte at a memory location pointed to by a register.
- `invert_global_bit`: Performs a bitwise NOT operation on a global variable.
- `process_event_command_queue`: Checks a flag and then executes various state changes or resets global variables based on the value of a specific control variable.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `initialize_state`: Sets a global variable to one hundred, sets another global variable to one, and calls another routine.
- `update_state_and_copy`: Sets a global variable to one hundred and another to two, calls another routine, and then copies a value to a memory location based on a conditional check.
- `initialize_object_state`: Sets a global variable to a constant value, modifies a local register, and then calls another routine via a null pointer.
- `process_object_list`: Iterates through a linked list of pointers, checking specific conditions for each entry and updating a table with calculated values and constants.
- `update_entity_list`: Iterates through a linked list of structures, decrementing a counter in each element and updating its position or properties.
- `write_memory_values`: Writes zero to a location pointed to by a register and stores a constant value at an offset from another register.
- `reset_memory_values`: Sets a word at a memory location to zero and sets a word at another location to two.
- `clear_word_and_set_byte`: Clears a word through a pointer and sets a byte in another structure.
- `clear_word_and_set_two_values`: Clears a word through a pointer and sets two word-sized fields in another structure.
- `set_byte_and_clear_word`: Sets a byte in a structure and clears a word through a pointer.
- `clear_word_and_set_word`: Clears a word through a pointer and sets a word in another structure.
- `clear_word_and_set_word_offset`: Clears a word through a pointer and sets a word at a specific offset in another structure.
- `search_table_and_set_flag`: Iterates through a table to find an entry matching specific coordinate criteria and sets a value in a structure based on a comparison.
- `update_object_counters_and_adjust_bcd`: Decrements internal counters and updates status flags within an object data structure.
- `update_object_state_and_manage_tables`: Iterates through an object data structure to decrement counters, update status flags, and perform BCD arithmetic on global variables.
- `reset_object_by_bounds`: Iterates through a table to find an object whose spatial coordinates fall within a specific bounding box.
- `reset_object_state_and_increment_counter`: Searches a table for an object within specific spatial bounds to reset its status and then increments a counter while updating global variables and BCD-encoded values based on the object's type.
- `tiny_ori_2`: Small routine of 1 instruction that merges 46 into d0.
- `load_constant_to_d0`: Loads a constant value into a register.
- `update_entity_state`: Processes an entity's state by checking its type and updating its position, velocity, or attributes based on a lookup table.
- `fill_global_with_countdown`: Sets a global variable to a value and then decrements it through a loop until it reaches negative one.
- `update_entity_state_and_bcd`: Updates entity status flags and coordinates based on specific state conditions, including BCD-adjusted arithmetic on memory values.
- `check_range_and_set_values`: Checks if a value falls within a specific range and, if so, sets a byte and a word in another structure.
- `update_buffer_and_manage_pointer`: Manages a data buffer by writing values, incrementing a pointer, and handling wrap-around or termination conditions based on a global flag.
- `process_collision_events`: Iterates through a list of objects to check for collisions based on specific type flags.
- `calculate_signed_difference`: Performs signed subtraction between values from a table and registers while checking for specific bit flags and overflow conditions.
- `update_entity_state_and_flags`: Updates an entity's internal state by calculating differences between table values and registers, then setting specific bit flags.
- `reset_entity_state_and_requeue`: Resets a large block of state variables and counters within an entity structure to zero or a default value.
- `copy_linked_list_data`: Iterates through a linked list of pointers to copy blocks of memory from one location to another.
- `update_entity_animation_state`: Calculates the current frame of an animation based on a timer and updates the entity's internal state.
- `manage_entity_lifecycle`: Updates internal counters and state flags for an entity, potentially resetting its properties if specific conditions are met.
- `manage_entity_lifecycle_state`: Monitors a counter within a structure and resets several state variables when the counter reaches zero.
- `manage_entity_lifecycle_and_spawn`: Monitors an entity's internal counter and state, resetting its properties when the counter reaches zero.
- `reset_and_append_entity_data`: Clears several global variables and a specific memory byte before iterating through a table to find an inactive entry.
- `append_object_data`: Clears a structure and then appends a new data block to a table using an incrementing offset.
- `append_object_data_variant`: Clears a structure and then appends a new data block to a table with different constant values than another routine.
- `initialize_entity_structure`: Clears a large block of memory within an entity structure and then appends two new entries to a global linked list or table.
- `set_global_constant`: Sets a global variable to a specific constant value and modifies the hardware color register.
- `pack_bitfield_data`: Combines multiple arithmetic results and bitwise shifts into two consecutive memory locations.
- `dispatch_script_commands`: Walks a byte-coded command stream, using a small type field in each 9-word record to select an action: writing hardware sound-channel registers, invoking a recursive handler, storing a longword to a target, or clearing a pointed-to longword.
- `dispatch_packet_records`: Walks a stream of 9-word records, switching on each record's type field: type 1 writes a write-enable value to the custom chip register block for channels below four, type 2 recurses into a sibling routine, type 3 stores a long value into the same register block, and type 4 clears a long at a pointer taken from the record.
- `process_data_packet_stream`: Iterates through a structured data stream, performing different operations based on a type byte found at a specific offset within each record.
- `update_entity_coordinates`: Iterates through a table of data, performing a busy-wait loop on a status flag before calculating new coordinate positions by adding values from the table to existing values.
- `process_sprite_attribute_table`: Iterates through a table of data structures to perform coordinate transformations and bitwise manipulations.
- `update_entity_position_and_bounds`: Iterates through a linked list of entities to update their spatial coordinates and bounding box dimensions.
- `execute_pointer_table_loop`: Iterates through a list of pointers and executes the routine or function located at each address until a certain value is encountered.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 164 into d1, then merges 244 into d1, then merges 12 into d1.
- `process_pointer_list`: Iterates through a list of pointers to extract data and call another routine with calculated offsets and values.
- `tiny_ori_ori_ori_2`: Small routine of 3 instructions that merges 252 into d0, then merges 232 into d1, then merges 82 into d0.
- `iterate_pointer_list_and_update_globals`: Iterates through a list of data structures until a negative value is encountered.
- `load_global_values`: Loads two values from a pointer into a global variable and then calls another routine with the subsequent address.
- `toggle_and_update_tables`: Toggles a value in a global variable and updates several consecutive 32-bit memory locations with calculated offsets.
- `execute_subroutine_sequence`: Executes a sequence of four subroutine calls.
- `populate_coordinate_table`: Iterates through a table to calculate and store multiple coordinate sets into a destination buffer based on a source list.
- `update_linked_list`: Iterates through a list, either copying data from another source or decrementing a counter within the existing elements.
- `process_object_attribute_table`: Iterates through a table of data structures to update various global state variables based on coordinate and size calculations.
- `populate_entity_attribute_table`: Iterates through a data table to extract entity properties and writes them into a structured memory buffer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_and_fill_buffer`: Initializes a memory range with a default value and then writes a counter or index into a buffer based on values read from a table.
- `reorganize_object_tables`: Filters and reorders entries within an object list based on specific flags, followed by a loop that calculates relative coordinate offsets and updates a secondary data structure.
- `merge_duplicate_elements`: Iterates through a linked list or array-based structure to find and merge adjacent duplicate entries.
- `resolve_object_collisions`: Iterates through a list of data structures to detect overlapping properties.
- `update_color_palette`: Updates a color register using values derived from a data structure.
- `update_object_positions`: Calls a routine to iterate through a list of object data to calculate new coordinate positions and velocities.
- `initialize_data_tables_2`: Performs multiple loops to write a sequence of incrementing or fixed values and constants into a memory table, followed by resetting a global variable and calling another routine.
- `update_object_position_table`: Iterates through a list of object data to calculate new coordinate positions and velocities.
- `copy_and_update_list`: Copies a block of data from a source pointer to a destination pointer and updates a global variable based on the value found in the source data.
- `fill_memory_with_source`: Copies a block of data from one memory location to two other destination locations for a fixed number of iterations.
- `update_state_counters_2`: Checks several global variables against specific constants and increments a counter if a condition is met.
- `load_attribute_data`: Loads several bytes from a table into a destination buffer based on an index derived from a bit-rotated value.
- `copy_structured_data_block`: Iterates through a table to copy sequences of data into a destination buffer.
- `copy_large_data_block`: Copies a large sequence of data from a source pointer into a destination buffer through a loop.
- `initialize_string_and_table_data`: Initializes a sequence of global variables with a constant value, then populates a string buffer by masking and offsetting bits from a source value.
- `check_conditional_states`: Compares several global variables against specific constants and calls another routine if certain conditions are met.
- `write_split_digits`: Writes two sets of data into a buffer based on the tens and units digits of a value.
- `copy_nested_blocks`: Performs a multi-level nested loop to copy blocks of data from one memory area to another.
- `clear_structured_data_buffers`: Iterates through a nested loop to zero out specific offsets within a series of memory blocks.
- `clear_memory_buffers`: Fills several memory buffers with zero values using nested loops.
- `map_string_to_table`: Iterates through a source string, checking for a match against a global string, and writes modified or offset character values into a destination table.
- `copy_font_to_buffers`: Copies certain bytes from a font table into two different destination buffers using a loop controlled by a global variable.
- `initialize_data_tables`: Writes a sequence of incrementing or offset values into a data table and another sequence into a different memory area.
- `expand_and_populate_table`: Copies a block of data from a source, then populates a destination table with a sequence of incrementing values and data from another source.
- `call_subroutine_only`: This routine calls another routine and then returns.
- `combine_buffer_pixels`: Performs a bitwise OR-based combination of bytes from a source buffer into a destination buffer using a nested loop structure.
- `populate_lookup_tables`: Calculates and stores squared values and a scaled sequence of integers into two separate memory tables.
- `initialize_memory_tables`: Fills a memory area with a constant value, populates a table using values from another source, and performs a conditional update on a third table.
- `update_state_counters`: Increments a global variable and performs conditional memory writes to a table based on the new value.
- `clear_bit_in_byte`: Clears a specific bit within a byte at a memory location determined by a pointer and a bit index.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_blt_parameters`: Configures hardware registers and certain memory locations with calculated offsets and bitmask-derived values, including a busy-wait check on a global variable.
- `copy_block_of_memory`: Performs a nested block copy of bytes from a source address to a destination address.
- `initialize_audio_parameters`: Resets several global variables to zero or a fixed value and copies certain values from a source pointer into a set of global variables.
- `update_audio_command_buffer`: Iterates through a command stream to update audio hardware registers and internal state.
- `process_command_data`: Uses a value from a source to select a branch from a table of operations, which may include updating a variable or calling another routine.
- `update_clamped_value`: Adds a value to a variable and stores the result in a table, ensuring the value does not exceed a specific limit.
- `store_byte_from_pointer`: Stores a byte from one memory location into another memory location.
- `return_immediately`: The routine is empty and performs no operations before returning.
- `return_from_subroutine`: This routine performs a single return instruction to exit the current execution flow.
- `parse_command_stream_2`: Decodes a stream of data from a buffer and updates a pointer.
- `reset_audio_flags`: Checks several global flags and resets them to zero if they are set to one.
- `parse_command_stream`: Reads and decodes a stream of command data from a buffer, updating internal state and pointers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `rotate_table_selection_bytes`: Sets a packed word of selection bytes from a table indexed by a global value and rotates its bytes before calling another routine.
- `parse_compressed_data_stream`: Iterates through a data stream to decode compressed or encoded sequences.
- `traverse_data_table`: Iterates through a table using values read from the table itself to update several global variables and counters.
