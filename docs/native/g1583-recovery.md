# Game 1583 (pang (1990)(ocean)[cr hz][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01583). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (970176 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 127 raw-word lines |
| Functions named and described | pass, 152 of 152 named (0 author symbols), 152 of 152 described (6 by rule, 137 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4579 of 4579 (100.00%) |
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
| Pang (1990)(Ocean)[cr HZ][a].adf:bootblock | 1 | 29 | 2 |
| logged disk load from Pang (1990)(Ocean)[cr HZ][a].adf boot-block load | 1 | 86 | 2 |
| logged disk load from Pang (1990)(Ocean)[cr HZ][a].adf boot-block load | 1 | 90 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7596 | 100 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2250 | 30 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 286 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 166 | 3 |

## Cases handled

- pang (1990)(ocean)[cr hz][a]/Pang (1990)(Ocean)[cr HZ][a].adf: custom-format (non-DOS) disk, 1722 non-empty blocks: matched against track-loaded segments
- pang (1990)(ocean)[cr hz][a]/Pang (1990)(Ocean)[cr HZ][a].adf: the boot block reads 23552 bytes from disk offset 859136 to an address (found statically); recovered as a unit
- pang (1990)(ocean)[cr hz][a]/Pang (1990)(Ocean)[cr HZ][a].adf: the boot block reads 132096 bytes from disk offset 727040 to an address (found statically); recovered as a unit
- runtime segment an address: 3420 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 747 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 250 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 162 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- pang (1990)(ocean)[cr hz][a]/Pang (1990)(Ocean)[cr HZ][a].adf: 2 logged loads cover 155648 bytes of the image
- 1 packed track region(s), 726016 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `reset_hardware_and_game_state` (runtime segment an address): Resets the Amiga hardware registers, including color, DMA, and interrupt settings, while clearing global variables and entity tables. Kind: loader/initialization with hardware reset. Pass path: the routine continues to the subsequent initialization calls (u4__sub_3904, etc.) after clearing the hardware/state registers.
- `dispatch_table_with_port_setup` (runtime segment an address): Passes the CIA B port address to a handler and executes a table-dispatch loop that iterates through three-word entries until a negative terminator is encountered. Kind: dispatch table setup for hardware/disk verification. Pass path: the loop continues to the next entry in the table (psVar3 + 3).

## Functions


### Pang (1990)(Ocean)[cr HZ][a].adf:bootblock

- `clear_sprite_registers_after_call`: A 21-instruction boot block entry, entered from the Kickstart strap, that calls an Amiga library routine at offset minus 456 and clears sixteen longwords from the sprite-zero position register.
- `copy_longwords_to_fixed_address`: An eight-instruction routine, reached by a pc-relative address reference and called from one unnamed routine.

### logged disk load from Pang (1990)(Ocean)[cr HZ][a].adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from Pang (1990)(Ocean)[cr HZ][a].adf boot-block load

- `decode_packed_bit_stream`: A 69-instruction depacker routine that touches the background color register.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_hardware_and_game_state`: Resets the Amiga hardware registers, including color, DMA, and interrupt settings, while clearing global variables and entity tables.
- `copy_paired_data_blocks`: Copies data from a source table into two separate destination memory areas using a nested loop structure and a stride.
- `manage_game_state_and_reset`: Checks various global state flags and conditional counters to determine if a state transition or reset is required.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `set_handler_pointer_and_value`: Stores a routine address into a global, writes a constant value to a second global, and modifies the interrupt request register.
- `store_handler_with_conditional_flag`: If a global variable is non-zero, it sets that variable to a specific value.
- `update_register_state`: Modifies a global variable and writes a bit-inverted, shifted value to a memory location, followed by a delay loop and a final register or memory update.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `append_entity_records_to_list`: Appends eight-byte records containing two longwords and a byte of 255 to a list, using different source values based on a state variable.
- `clear_state_and_set_background`: Clears several globals and flags, makes an indirect call through a table, and runs a countdown loop when a flag is unable to be cleared.
- `update_memory_with_offset`: Increments a global variable by eight and writes three specific values to a memory location calculated from that variable.
- `update_digit_counters`: Extracts decimal digits from a value and updates a series of global counters and storage locations based on those digits.
- `write_paired_blocks_at_stride`: Writes a series of blocks, where each pass writes two blocks to a register block with a stride, using values derived from a global variable and a register.
- `append_first_pair_entry`: Appends an eight-byte record to a list using two longword values from globals and a one-byte value of 255, then advances the list index by eight.
- `append_list_entry`: Appends an eight-byte record to a list by storing two longword values from global memory and a byte of 255, then increments the list pointer by eight.
- `call_indexed_routine`: Calls a routine selected from a table using an index stored in a global variable.
- `decrement_and_update_list`: Decrements a global counter and, if it reaches zero, updates a pointer and a list index.
- `execute_conditional_calls`: Checks a global byte and calls another routine twice if the byte is zero, otherwise calls it twice regardless of the check.
- `decrement_and_update_timer_state`: Decrements a counter and, if it reaches zero, resets it using a value from a table while advancing a secondary pointer.
- `check_entity_collision_hits`: Walks a null-terminated list of entity record pointers, and for each active record compares its position fields against a bounding box derived from a global object's position; on overlap it marks the record as consumed, sets a flag and a score-like value, and returns a pointer to the hit record.
- `check_player_object_collisions`: Iterates through a linked list of objects to perform bounding box collision detection against a target area defined by a global structure.
- `check_collision_with_objects`: Iterates through a linked list of object structures to perform bounding box collision detection against a target area.
- `check_collision_state`: Iterates through a table of data structures to check for rectangular overlaps and updates status values in the table and global variables based on the results.
- `check_collision_bounds`: Iterates through a table of data structures to check if a specific rectangular area overlaps with any entries.
- `update_conditional_value`: Checks if a global variable is zero or if another specific value is not two, and if so, assigns a constant to a memory location.
- `execute_jump_table_entry`: Compares a value to a global variable and, if they match, increments a memory location by ten and calls a function from a jump table.
- `update_object_state_and_score`: Updates object coordinates and properties within a table based on specific conditional flags.
- `manage_object_lifetimes`: Decrements a counter and manages a list of active objects by checking their status and updating pointers.
- `initialize_record_pointers`: Sets two pointer fields to a global base, assigns a value to a word field, and stores a specific longword value.
- `initialize_record_pointer_fields`: Sets two pointer fields to a global base, stores the value 3 in a word field, and stores a longword with its top byte set.
- `resolve_entity_collision_and_velocity`: Checks an entity's bounding box against a table of other entities to detect intersections.
- `update_entity_physics_and_collisions`: Updates an entity's position and velocity based on movement logic and checks for bounding box intersections against a list of other entities.
- `check_collision_with_entities`: Iterates through a linked list of entity structures to perform bounding box intersection tests against a target area.
- `append_data_to_list`: Checks two global conditions and, if met, appends a longword and a byte to a list while updating a counter and a pointer.
- `append_adjusted_longword_entry`: Appends a longword or a pointer to a list while updating a counter and a pointer.
- `clear_multiple_globals`: Sets three global variables to zero and clears a byte at a location relative to a register.
- `clear_multiple_globals_abs`: Sets three global variables to zero and clears a byte at a location relative to a register using an absolute address.
- `clear_multiple_globals_pcrel`: Sets three global variables to zero and clears a byte at a location relative to a register using a program counter relative address.
- `clear_multiple_globals_2`: Resets several global variables or pointers to zero.
- `set_global_and_clear_byte`: Sets a global variable to three and clears a byte at a location relative to a register.
- `invert_global_bit`: Performs a bitwise NOT operation on a global variable.
- `process_state_command_dispatch`: Checks a flag and dispatches various state changes based on the value of a global variable.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `set_mode_one_clear_pointer`: Sets a global variable to one hundred, sets a mode byte to one, calls another routine, and clears a routine-pointer global to zero.
- `set_mode_two_copy_value`: Sets a mode byte to two, calls another routine, and copies a longword from a pointer into a field in a list record.
- `set_mode_three_clear_pointer`: Sets a global variable to one hundred, sets a mode byte to three, and clears a routine-pointer global to zero.
- `claim_idle_records_for_slots`: Walks a linked list of records to find idle entries and initializes them with new data from a table.
- `update_entity_list_and_check_collisions`: Iterates through a linked list of structures, decrementing a counter and updating position data.
- `clear_word_and_set_three`: Clears a word through a pointer, sets a field in another structure to three, and calls another routine.
- `set_word_and_short`: Sets a word to zero and a short to two at specific offsets from a register.
- `clear_word_and_set_byte`: Clears a word through a pointer and sets a byte in a structure at a specific offset.
- `clear_word_and_set_two_values`: Clears a word through a pointer and sets two word-sized fields in another structure.
- `set_enemy_flag_and_clear_ptr`: Writes a specific byte value to a global variable and clears a value through a pointer.
- `clear_word_and_set_short`: Clears a word through a pointer and sets a short value in a structure.
- `clear_word_and_set_short_offset`: Clears a word through a pointer and sets a short value at a different offset in a structure.
- `find_record_overlapping_box`: Searches an array of records for a box overlap and updates a flag in the matching record.
- `update_sprite_physics_and_collision`: Updates the position of sprite objects and handles boundary collisions by reversing direction.
- `update_bouncing_sprite_object`: Updates a sprite object's position based on its velocity and handles screen boundary collisions by reversing direction.
- `iterate_object_list_and_update_state`: Iterates through a linked list of objects to check for spatial overlaps within specific coordinate bounds.
- `handle_object_collision_and_update_state`: Checks for collisions between an object and a list of other objects by comparing coordinates, then updates the object's state, increments a counter, and modifies global variables or score-related BCD values based on the collision type.
- `tiny_ori_2`: Small routine of 1 instruction that merges 46 into d0.
- `perform_tiny_ori`: Performs a single instruction that merges a constant into a register.
- `update_entity_state_and_timer`: Updates an entity's properties based on a state value retrieved from a pointer, modifying coordinates or counters accordingly.
- `fill_global_with_countdown`: Repeatedly writes a decreasing sequence of values into a global variable starting from ten thousand.
- `update_object_state_and_score_2`: Updates object properties and state flags based on specific conditional checks.
- `check_range_and_set_values`: Checks if a value falls within a specific range and, if so, sets a byte and a word in a structure.
- `emit_pointer_list_entry`: Writes one entry from the current pointer list into the display list on each call.
- `process_object_table_frames`: Walks a table of ten-word object records until a negative terminator.
- `test_box_overlap_offset`: Tests whether a point lies inside a rectangle described by four words in a record, with an optional flag check.
- `process_entity_collision_and_state`: Checks for overlaps between an entity and a rectangular area, then updates the entity's internal state flags and resets its properties if a collision is detected.
- `reset_entity_state_and_queue`: Clears a large block of state variables and flags within an entity record and then appends new entries to a global entity table.
- `copy_multiple_data_blocks`: Iterates through a list of pointers to copy several blocks of data from one memory location to another.
- `update_entity_state_and_queue`: Updates an entity's internal state, modifies its position or timing parameters, and appends a new record to a global entity table.
- `process_entity_lifecycle_and_queue`: Manages the state and lifecycle of an entity by updating its internal counters, checking for expiration or completion conditions, and appending new records to a global entity table.
- `manage_entity_lifecycle`: Updates a counter within a structure and, upon reaching zero, resets several state variables and appends a new record to a global table.
- `manage_object_lifecycle_and_queue`: Monitors a counter within an object record and performs cleanup or state resets when the counter reaches zero.
- `process_object_collision_and_update_state`: Iterates through a table of object records to check for collision conditions based on coordinate comparisons.
- `clear_object_and_queue_record`: Clears several state words of an object record and sets its flag byte.
- `reset_object_queue_pair`: Clears the state words of an object record and sets its flag byte.
- `initialize_entity_record`: Clears a large block of memory within an entity structure and then appends new record entries to a global table.
- `set_global_color_value`: Sets a global variable to a specific value and modifies the first color register.
- `encode_sprite_control_words`: Packs a sprite's horizontal and vertical start and stop positions, with fixed display-window offsets, into the two control words the Amiga sprite DMA reads.
- `dispatch_command_queue_entries`: Walks a queue of fixed-size records, using each record's type field to select a branch that either writes hardware-register pairs for a custom chip or copies the record into an indexed slot of a secondary table with a size increment.
- `clear_d0_and_nop`: Performs a bitwise OR operation on the d0 register with zero and executes three no-operation instructions.
- `dispatch_command_records`: Walks through a sequence of nine-word command records, executing specific actions based on a type field in each record.
- `blit_sprite_records`: Waits for the blitter to be idle, then for each three-word record programs the blitter control, mask and address registers.
- `blit_visible_records_with_clipping`: Iterates through a list of eleven-word records, filtering for those whose vertical extent is within a visible window.
- `iterate_and_update_entity_list`: Iterates through a linked list of pointers to update entity properties and manage memory allocation.
- `execute_pointer_table_loop`: Iterates through a list of pointers and executes the routine or function located at each address until a certain value is encountered.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 164 into d1, then merges 244 into d1, then merges 12 into d1.
- `process_pointer_list_with_parameters`: Iterates through a list of records to extract data and call a routine with various calculated parameters and global values.
- `tiny_ori_ori_ori_2`: Small routine of 3 instructions that merges 252 into d0, then merges 232 into d1, then merges 82 into d0.
- `dispatch_table_with_port_setup`: Passes the CIA B port address to a handler and executes a table-dispatch loop that iterates through three-word entries until a negative terminator is encountered.
- `copy_pair_and_call_handler`: Copies two 32-bit values from the record at the input pointer into shared globals, then calls a handler with the pointer advanced past them.
- `toggle_buffer_set_planes`: Swaps a double-buffer selector between two screen buffer addresses and sets a flag word to match.
- `execute_nested_calls`: Performs four consecutive calls to another routine.
- `blit_record_to_five_planes`: Waits for the blitter to be idle, then for each record programs the blitter destination address five times, one bitplane apart.
- `refill_slots_from_pool`: Refills an active-slot table from a source pool whenever a slot's countdown reaches zero.
- `draw_clipped_blit_objects`: Walks a list of object entries and clips each one against the 320 by 200 screen area, adjusting the blitter source pointer, modulo and start address for the clipped part.
- `blit_visible_objects_to_planes`: Waits for the blitter, then iterates through a list of object records, skipping those outside the screen boundaries.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_and_fill_buffer`: Initializes a memory range with a default value and then writes a counter or index into a buffer based on values read from a table.
- `reorganize_object_tables`: Filters and reorders an array of object data structures based on a specific flag, then performs a secondary pass to calculate relative coordinate offsets.
- `merge_duplicate_elements`: Iterates through a linked list or array-based structure to find and merge adjacent duplicate entries.
- `resolve_object_collisions`: Iterates through a list of data structures to detect overlaps between objects.
- `update_color_palette`: Updates a color register using a value derived from a data structure.
- `update_object_positions`: Calls a routine to iterate through a list of object data to calculate new coordinate positions and velocities.
- `initialize_data_tables_2`: Performs multiple loops to write a sequence of incrementing or fixed values and constants into a memory table, followed by resetting a global variable and calling another routine.
- `update_object_position_table`: Iterates through a list of object data to calculate new coordinate positions and velocities.
- `copy_and_update_list`: Copies a block of data from a source pointer to a destination pointer and updates a global variable based on the value found in the source data.
- `fill_memory_with_source`: Copies a block of data from one memory location to two other destination locations for a fixed number of iterations.
- `step_two_timed_sequences`: Manages two timed sequences by comparing a global timer against thresholds.
- `render_nibble_glyphs_to_rows`: Splits a 32-bit value into eight nibbles, skips leading zero nibbles, and writes each remaining nibble as a glyph from a seven-row font table into a buffer with rows 40 bytes apart.
- `copy_stack_selected_tile_block_2`: Copies a block of data from a source pointer into a destination buffer using a fixed number of iterations.
- `copy_stack_selected_tile_block`: Performs the same fill-then-copy as its neighbour, but takes the block index from the caller's stack and applies no range clamp.
- `initialize_string_and_table_data`: Initializes a sequence of global variables with a constant value, then populates a string buffer by masking and offsetting bits from a source value.
- `queue_channel_updates_by_threshold`: Calls the update routine for each of two channels when that channel's counter is above zero or above four, and the global mode value does not select the other channel.
- `render_two_digit_number`: Clamps a value to 99, splits it into tens and units, and writes the two glyphs from a seven-row font table into a buffer 40 bytes per row.
- `copy_clamped_tile_block_count`: Copies up to four blocks of tile rows from a source buffer into a strided destination, each block being four rows of sixteen words with fixed row and block spacing.
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
- `advance_sound_step_command`: Advances a step counter while the active flag is set, queuing command records with specific addresses and values at different stages.
- `clear_bit_and_busy_wait`: Clears one bit in a byte-sized flag at the given address, then spins in a busy-wait loop of a fixed count before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_blt_parameters`: Configures hardware registers and certain memory locations with calculated offsets and bitmask-derived values, including a busy-wait check on a global variable.
- `copy_block_of_memory`: Performs a nested block copy of bytes from a source address to a destination address.
- `initialize_audio_parameters`: Resets several global variables to zero or a fixed value and copies certain values from a source pointer into a set of global variables.
- `update_audio_channel_data`: Iterates through a data table to load audio parameters into global variables and updates hardware audio channel registers.
- `process_command_table`: Uses a byte from a table to select one of several operations, including updating a value with a clamped sum or calling a function via a pointer.
- `add_nibble_clamped_to_word`: Adds the low nibble of an input byte to a 16-bit value, clamps the sum at a fixed upper limit, and stores it in the value and in a shadow copy.
- `transfer_byte_between_pointers`: Copies a single byte from one memory location to another using two pointers.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `return_from_subroutine_2`: This routine performs a single return instruction to exit the current execution flow.
- `dispatch_channel_handler_by_tick`: Calls a channel handler chosen from a jump table indexed by the driver's tick counter.
- `update_audio_dma_restart_flags`: Combines a shared DMA mask with a shared word, then clears the restart flag of each of four audio channels when that flag is set.
- `process_command_table_entry`: Parses a command structure from a data stream and executes specific operations based on a command byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `rotate_table_selection_bytes`: Sets a packed word of selection bytes from a table indexed by a global value and rotates its bytes before calling another routine.
- `parse_compressed_data_stream`: Iterates through a data stream to decode compressed or encoded values by reading specific marker bytes.
- `parse_data_table`: Traverses a table using values stored in a global variable to perform arithmetic calculations and update several other global variables.
