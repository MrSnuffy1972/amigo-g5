# Game 1299 (pang (1990)(ocean)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01299). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (970176 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 127 raw-word lines |
| Functions named and described | pass, 159 of 159 named (0 author symbols), 159 of 159 described (6 by rule, 149 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4708 of 4708 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9339 of 10572 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 179068 |
| code | 39376 |
| data | 732358 |
| padding | 942 |
| unused | 18432 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Pang (1990)(Ocean).adf:bootblock | 1 | 31 | 2 |
| logged disk load from Pang (1990)(Ocean).adf boot-block load | 1 | 153 | 9 |
| logged disk load from Pang (1990)(Ocean).adf boot-block load | 1 | 90 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7596 | 100 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2250 | 30 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 286 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 166 | 3 |

## Cases handled

- pang (1990)(ocean)/Pang (1990)(Ocean).adf: custom-format (non-DOS) disk, 1711 non-empty blocks: matched against track-loaded segments
- pang (1990)(ocean)/Pang (1990)(Ocean).adf: the boot block reads 23552 bytes from disk offset 859136 to an address (found statically); recovered as a unit
- pang (1990)(ocean)/Pang (1990)(Ocean).adf: the boot block reads 132096 bytes from disk offset 727040 to an address (found statically); recovered as a unit
- runtime segment an address: 3549 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 747 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 250 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 162 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- pang (1990)(ocean)/Pang (1990)(Ocean).adf: 2 logged loads cover 155648 bytes of the image
- 1 packed track region(s), 726016 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `reset_hardware_and_game_state` (runtime segment an address): Resets the Amiga hardware registers, including color, DMA, and interrupt settings, while clearing global variables and entity tables. Kind: loader/initialization with hardware reset. Pass path: the routine continues to the subsequent initialization calls (u4__sub_3904, etc.) after clearing the hardware/state registers.
- `iterate_pointer_list_and_update_globals` (runtime segment an address): Iterates through a list of data structures until a negative value is encountered. Kind: pointer list iteration for data/code validation. Pass path: the loop continues to the next pointer in the list.

## Functions


### Pang (1990)(Ocean).adf:bootblock

- `initialize_boot_block`: Sets up DMA-based memory copying, clears a certain memory area, and copies a block of data from a source address to a destination address.
- `copy_memory_block`: Copies a block of words from a source address to a destination address, setting a global variable to a certain value and enabling a DMA channel.

### logged disk load from Pang (1990)(Ocean).adf boot-block load

- `copy_code_block`: Copies a block of code from one memory location to another and modifies specific hardware control registers and global variables.
- `assign_routine_address`: Stores the address of a routine into a global variable.
- `copy_boot_data`: Copies a sequence of words from a location in the code or a nearby address to a destination in a higher memory area.
- `write_register_value`: Writes a constant value to an offset relative to a register.
- `initialize_data_structures`: Sets several values in a table, clears a large memory area, and executes a loop that calls another routine.
- `copy_seven_bytes_at_stride`: Copies seven bytes from a source area into destination positions spaced 40 bytes apart.
- `update_entity_state_flags`: Checks a bitmask in a data structure and, if a specific bit isset, modifies certain status flags and counters or increments/decrements a short integer based on a character value.
- `process_table_data`: Iterates through a table using another routine, then performs arithmetic on a value and calls another routine twice.
- `copy_seven_longwords_at_stride`: Copies seven longwords from a source area into a destination area at spacing of 64 bytes.

### logged disk load from Pang (1990)(Ocean).adf boot-block load

- `decode_packed_bit_stream`: Decodes a bitstream and writes the resulting bytes or a sequence of copied bytes to a destination buffer.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_hardware_and_game_state`: Resets the Amiga hardware registers, including color, DMA, and interrupt settings, while clearing global variables and entity tables.
- `copy_data_table`: Copies multiple blocks of data from a source table to two different destination memory areas based on values stored in a global variable.
- `reset_game_state_and_counters`: Checks specific global flags and state variables before performing a massive reset of numerous global counters, pointers, and memory locations.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `set_global_constant_2`: Stores a constant value into a global variable.
- `enable_dma_and_set_global`: Enables DMA channels by writing to the DMA controller and updates a global variable with a specific constant value.
- `update_register_state`: Modifies a global variable and writes a bit-inverted, shifted value to a memory location, followed by a delay loop and a final register or memory update.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `populate_entity_attribute_table_2`: Iterates through a data table to extract entity properties and writes them into a structured memory buffer.
- `reset_palette_and_decrement_counter`: Sets a color register to white, clears several global variables, and executes a sequence of subroutine calls.
- `update_memory_block`: Checks a bit in a memory location and, if not set, increments a global variable and writes a specific multi-byte value to a memory offset.
- `update_digit_counters`: Compares digits extracted from a large value against stored values and decrements a counter to update a table when the counter reaches zero.
- `write_attribute_list`: Calculates offsets based on an input value and writes a series of multi-byte values and integers into a structured list or buffer.
- `append_data_sequence`: Writes a specific sequence of bytes to a memory location determined by an offset and increments that offset by eight.
- `append_different_data_sequence`: Writes a different sequence of bytes to a memory location determined by an offset and increments that offset by eight.
- `execute_indirect_call`: Calculates an address from a table using an index and performs an indirect function call.
- `decrement_and_update_list`: Decrements a global counter and, if it reaches zero, updates a pointer and a list index.
- `execute_conditional_subroutines`: Checks a global byte and calls several other routines multiple times based on that value.
- `decrement_and_update_timer_state`: Decrements a counter and, if it reaches zero, resets it using a value from a table while advancing a secondary pointer.
- `test_projectile_entity_collision`: Walks a linked list of entity records, skipping inactive ones, and tests each entity's bounding box against a fixed rectangle derived from a base position, with different hit-box offsets chosen by an entity type code.
- `check_entity_collision_and_trigger_event`: Iterates through a table of entity pointers to perform bounding box intersection tests against a target area defined by a global variable.
- `check_collision_with_entities`: Iterates through a table of entity pointers to perform bounding box intersection tests against a target area.
- `check_collision_state`: Iterates through a table of data structures to check for rectangular overlaps and updates specific values in the table or global variables based on the results.
- `check_collision_bounds`: Iterates through a table of data structures to check if a specific rectangular area overlaps with the bounds defined in the table entries.
- `update_conditional_value`: Checks if a global variable is zero or if another specific value is not two, and if so, assigns a value to a memory location.
- `execute_jump_table_entry`: Compares a value in a register to a global variable, and if they match, increments a memory location by ten and calls a routine from a jump table.
- `update_object_state_and_counters`: Updates object status and increments table indices while performing BCD adjustments on specific memory locations.
- `manage_object_lifetimes_and_events`: Decrements a counter and manages a list of active objects, triggering event initialization or cleanup when counters reach zero or specific conditions are met.
- `initialize_data_structure_2`: Sets several global variables and pointers using values provided through a pointer in a register.
- `initialize_data_structure`: Stores the address of a global variable and several constant values into a block of memory starting at an offset from a pointer.
- `update_object_state_and_collision`: Updates an object's internal state and velocity based on collision detection against a linked list of bounding boxes.
- `update_object_position_and_collision`: Updates an object's coordinate and velocity based on specific state flags and performs bounding box intersection tests against a linked list of other objects.
- `check_collision_with_object_list`: Iterates through a linked list of objects to perform bounding box intersection tests against a target area.
- `push_data_to_table_2`: Checks two global conditions and then increments a counter and stores a multi-part data entry into a table.
- `push_data_to_table`: Increments a counter in a global variable and stores a four-byte value, a second integer, and a byte into a table at a calculated offset.
- `clear_global_values`: Sets three specific global variables to zero and clears a byte at a memory location relative to a register.
- `reset_state_variables`: Clears three global variables to zero and writes a zero byte to a memory location calculated from a register offset.
- `reset_state_variables_2`: Clears three specific global variables to zero and writes a zero byte to a location determined by an offset from a register.
- `clear_multiple_globals`: Resets several global variables or pointers to zero.
- `set_global_constants`: Sets a global variable to three and clears a byte at a memory location relative to a register.
- `invert_global_bit`: Performs a bitwise NOT operation on a global variable.
- `process_event_command_queue`: Checks a flag and then executes various state changes based on the value of a global variable.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `initialize_state`: Sets a global variable to one hundred, sets another global variable to one, and calls another routine.
- `update_state_and_copy`: Sets a global variable to one hundred and another to two, calls another routine, and then copies a value to a memory location based on a conditional check.
- `initialize_object_state`: Sets a global variable to a constant value, modifies a local register, and then calls another routine via a null pointer.
- `process_object_list`: Iterates through a linked list of pointers, checking specific conditions for each entry and updating a table with calculated values and constants.
- `update_entity_collision_and_state`: Iterates through a list of entities to decrement a counter and update their position or state based on collision checks against a table of objects.
- `write_memory_values`: Writes zero to a location pointed to by a register and stores a constant value at an offset from another register.
- `reset_memory_values`: Sets a word at a pointer to zero and stores the value two at a specific offset from a register.
- `clear_word_and_set_byte`: Clears a word through a pointer and sets a byte in another structure.
- `clear_word_and_set_two_values`: Clears a word through a pointer and sets two word-sized fields in another structure.
- `set_byte_and_clear_word`: Sets a byte in a structure and clears a word through a pointer.
- `clear_word_and_set_word`: Clears a word through a pointer and sets a word in another structure.
- `clear_word_and_set_word_offset`: Clears a word through a pointer and sets a word at a specific offset in another structure.
- `search_table_and_set_flag`: Iterates through a table to find an entry matching specific coordinate criteria and sets a value in a structure based on a comparison.
- `update_object_state_and_manage_tables_2`: Iterates through an object data structure to decrement counters, update status flags, and perform BCD arithmetic on global variables.
- `update_object_state_and_manage_tables`: Iterates through an object data structure to decrement counters, update status flags, and perform BCD arithmetic on global variables.
- `reset_object_and_update_score`: Searches an object table for an entity within specific spatial bounds to reset its status and then updates a score or counter using BCD arithmetic.
- `reset_object_state_and_increment_counter`: Searches a table for an object within specific spatial bounds to reset its status and then increments a counter while updating global variables and BCD-encoded values based on the object's type.
- `tiny_ori_2`: Small routine of 1 instruction that merges 46 into d0.
- `execute_recursive_call`: The routine performs a direct call to itself, resulting in infinite recursion.
- `update_entity_state_and_position`: Updates an entity's position and properties based on a type identifier retrieved from a linked list.
- `fill_global_loop`: Sets a global variable to a value and decrements it through a loop until it reaches negative one.
- `update_object_state_and_bcd`: Updates object status, increments counters, and performs BCD (Binary Coded Decimal) adjustments on specific memory locations.
- `check_range_and_set_values`: Checks if a value falls within a specific range and, if so, sets a byte and a word in another structure.
- `update_buffer_and_manage_pointer`: Manages a data buffer by writing values, incrementing a pointer, and handling wrap-around or termination conditions based on a global flag.
- `process_collision_events`: Iterates through a list of objects to check for collisions based on specific type flags.
- `calculate_signed_difference`: Performs signed subtraction between values from a table and registers while checking for specific bit flags and overflow conditions.
- `update_object_state_and_flags`: Updates various status fields and bitwise flags within an object data structure.
- `reset_object_state_and_reallocate`: Resets a large block of status and counter fields within a data structure to zero or a default value.
- `copy_linked_data_blocks`: Iterates through a list of pointers and performs nested loops to copy blocks of data from one memory location to another.
- `manage_object_lifecycle`: Updates object state and manages memory allocation for dynamic entities.
- `manage_object_lifecycle_and_allocation`: Updates internal state counters and performs memory allocation for objects within a global pool.
- `manage_object_lifecycle_and_allocation_2`: Monitors a counter and resets state variables when it reaches zero.
- `manage_object_lifecycle_and_spawn`: Monitors a counter within a data structure and, upon reaching zero, resets specific status fields and appends a new entry to a global object table.
- `reset_object_and_update_counters`: Iterates through a table of data structures to find an entry matching specific criteria, then resets that entry's status and updates several global counters using BCD arithmetic.
- `append_object_data`: Clears a structure and then appends a new data block to a table using an incrementing offset.
- `append_object_data_variant`: Clears a structure and then appends a new data block to a table with different constant values than another routine.
- `initialize_object_state_and_allocate`: Clears a large block of memory within a structure and then performs two sequential allocations from a global pool.
- `set_global_constant`: Sets a global variable to a specific constant value.
- `pack_bitfield_data`: Combines multiple arithmetic results and bitwise shifts into two consecutive memory locations.
- `dispatch_entry_by_type`: The routine is a single-instruction stub reached only through address-taken moves from several object-setup and allocation routines, so its identity is inferred from its callers rather than from its body.
- `parse_data_packet_stream`: Processes a stream of data packets by reading a type identifier and executing specific actions such as updating memory values, setting table entries, or recursively calling another routine.
- `process_data_packet_stream`: Iterates through a structured data stream, performing different operations based on a type byte found at a specific offset within each record.
- `update_entity_coordinates`: Iterates through a table of data, performing a busy-wait loop on a status flag before calculating new coordinate positions by adding values from the table to existing values.
- `process_sprite_attribute_table`: Iterates through a table of data structures to perform coordinate transformations and bitwise manipulations.
- `iterate_and_update_object_list`: Iterates through a linked list of structures, performing arithmetic calculations on member values and updating pointers.
- `execute_pointer_table_loop`: Iterates through a list of pointers and executes the routine or function located at each address until a certain value is encountered.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 164 into d1, then merges 244 into d1, then merges 12 into d1.
- `process_pointer_list`: Iterates through a list of data structures to extract values and call another routine with calculated parameters.
- `tiny_ori_ori_ori_2`: Small routine of 3 instructions that merges 252 into d0, then merges 232 into d1, then merges 82 into d0.
- `iterate_pointer_list_and_update_globals`: Iterates through a list of data structures until a negative value is encountered.
- `load_global_values`: Loads two values from a pointer into a global variable and then calls another routine with the subsequent address.
- `toggle_and_update_tables`: Toggles a value in a global variable and updates several consecutive 32-bit memory locations with calculated offsets.
- `execute_sequential_subroutines`: This routine performs four consecutive calls to the same subroutine.
- `populate_coordinate_table`: Iterates through a table to calculate and store multiple coordinate sets into a destination buffer based on a source list.
- `update_linked_list`: Iterates through a list, either copying data from another source or decrementing a counter within the existing elements.
- `process_object_attribute_table`: Iterates through a table of data structures to update various global state variables based on coordinate and size calculations.
- `populate_entity_attribute_table`: Iterates through a data table to extract entity properties and writes them into a structured memory buffer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_and_fill_buffer`: Initializes a memory range with a default value and then writes a counter or index into a buffer based on values read from a table.
- `reorganize_object_tables`: Filters and reorders an array of object data structures based on specific flags, followed by calculating relative positional offsets for a secondary table.
- `merge_duplicate_elements`: Iterates through a linked list or array-based structure to find and merge adjacent duplicate entries.
- `resolve_object_collisions`: Iterates through a list of data structures to check for overlapping positional values.
- `update_object_coordinates`: Iterates through a data structure to extract positional values and calculates relative offsets.
- `update_object_positions`: Calls a routine to iterate through a list of object data to calculate new coordinate positions and velocities.
- `initialize_data_tables_2`: Performs multiple loops to write a sequence of incrementing or fixed values and constants into a memory table, followed by resetting a global variable and calling another routine.
- `update_object_position_table`: Iterates through a list of object data to calculate new coordinate positions and velocities.
- `copy_and_update_list`: Copies a block of data from a source pointer to a destination pointer and updates a global variable based on the value found in the source data.
- `fill_memory_with_source`: Copies a block of data from one memory location to two other destination locations for a fixed number of iterations.
- `update_state_counters_2`: Checks several global variables against specific constants and increments a counter if a condition is met.
- `load_attribute_data`: Loads several bytes from a table into a destination buffer based on an index derived from a bit-rotated value.
- `copy_structured_data_blocks`: Iterates through a table to copy sequences of data into a destination buffer.
- `copy_large_data_block`: Copies a large sequence of data from a source pointer into a destination buffer through a loop.
- `initialize_string_and_table_data`: Initializes a sequence of global variables with a constant value, then populates a string buffer by masking and offsetting bits from a source value.
- `check_conditional_states`: Compares several global variables against specific constants and calls another routine if certain conditions are met.
- `write_split_digits`: Writes two sets of data into a buffer based on the tens and units digits of a value.
- `copy_nested_blocks`: Performs a multi-level nested loop to copy blocks of data from one memory area to another.
- `clear_structured_memory_blocks`: Iterates through a nested loop to zero out specific offsets within a series of memory blocks.
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
- `parse_audio_command_stream`: Iterates through a data stream to decode and load audio parameters into memory.
- `process_command_data`: Uses a byte from a source to select one of several operations, such as updating a value in a table or calling another routine.
- `update_clamped_value`: Adds a value to a variable and stores the result in a table, ensuring the value does not exceed a specific limit.
- `transfer_single_byte`: Stores a byte from one memory location into another memory location.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `return_from_subroutine_2`: the routine contains no instructions and performs no operations.
- `parse_command_stream_2`: Decodes a stream of data from a buffer and updates a pointer.
- `reset_audio_flags`: Checks several global flags and resets them to zero if they are set to one.
- `parse_command_stream`: Reads a stream of data from a buffer, handling multi-byte instruction decoding and pointer increments.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `rotate_table_selection_bytes`: Sets a packed word of selection bytes from a table indexed by a global value and rotates its bytes before calling another routine.
- `parse_tile_data_stream`: Iterates through a data stream to extract tile properties and metadata.
- `parse_data_table`: Traverses a table using an index and performs arithmetic calculations on values retrieved from the table to update several global variables.
