# Game 357 (AdventuresOfQuik&Silva v1.1): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00357). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1325907 top-level bytes in 20 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 81 raw-word lines |
| Functions named and described | pass, 286 of 286 named (0 author symbols), 286 of 286 described (9 by rule, 178 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3656 of 3657 (99.97%) |
| Recovered without load address (counted in proof 1, flagged) | 550964 bytes, 16 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5987 of 6005 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 19073 |
| code | 25172 |
| data | 1281573 |
| padding | 89 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 226 | 16 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 473 | 16 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 974 | 74 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 199 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1412 | 52 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1203 | 30 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 943 | 71 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 6 |
| Quick (raw code file, load address unknown) | 1 | 546 | 16 |
| Silva (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- AdventuresOfQuik&Silva_v1.1/AdventuresOfQuik&Silva_v1.1.hdf: hard-disk image (AmigaDOS OFS, 1408 blocks): its filesystem parses and all 9 files in it are byte-identical to the extracted tree AdventuresOfQuik&Silva_v1.1_hdf, which is recovered file by file; no block outside the filesystem holds data
- 4 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 210 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 710 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 152 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 951 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 738 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 703 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- AdventuresOfQuik&Silva_v1.1/AdventuresOfQuik&Silva_v1.1_hdf/AdventuresOfQuik&Silva/Quick: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- AdventuresOfQuik&Silva_v1.1/AdventuresOfQuik&Silva_v1.1_hdf/AdventuresOfQuik&Silva/Silva: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying code at an address (2 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfQuik&Silva_v1.1/AdventuresOfQuik&Silva_v1.1_hdf/AdventuresOfQuik&Silva/game.slave +3B6, AdventuresOfQuik&Silva_v1.1/AdventuresOfQuik&Silva_v1.1_hdf/AdventuresOfQuik&Silva/game.slave +3C0, AdventuresOfQuik&Silva_v1.1/AdventuresOfQuik&Silva_v1.1_hdf/AdventuresOfQuik&Silva/game.slave +3CA
- self-modifying code at an address (2 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-long-joy)): both forms recovered; writer: AdventuresOfQuik&Silva_v1.1/AdventuresOfQuik&Silva_v1.1_hdf/AdventuresOfQuik&Silva/game.slave +3B6, AdventuresOfQuik&Silva_v1.1/AdventuresOfQuik&Silva_v1.1_hdf/AdventuresOfQuik&Silva/game.slave +3C0, AdventuresOfQuik&Silva_v1.1/AdventuresOfQuik&Silva_v1.1_hdf/AdventuresOfQuik&Silva/game.slave +3CA
- self-modifying code at an address (60 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- 2 never-loaded region(s), 550964 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `update_channel_table_entry` (runtime segment an address): Copies a table word selected by a record field into a second table slot and updates two state words. Kind: disk hardware/track configuration check. Pass path: the routine continues to update internal state/buffers based on the values read from the hardware/table, specifically the branch where it updates uRam0007005c/5e if the bitmask check at (unaff_A6 + 6) passes..

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `dispatch_loop_copper_dma_writes`: Runs an endless loop that calls function-pointer entries from a structure, writing the copper pointer and the interrupt and DMA control registers on each pass.
- `stub_self_assignment_return`: Writes a global variable back to itself, which has no effect, and returns.
- `flash_colour_around_handler_call`: Writes a fixed colour value to a global variable, calls a function pointer stored in a structure, and then clears the value back to zero.
- `stub_infinite_loop_halt`: Spins in an endless loop with no work in the body.
- `clear_global_flag_pair`: Writes zero to a global variable twice, with no other logic.
- `set_flag_after_call`: Calls one routine, then sets a flag variable to one and returns.
- `toggle_flag_and_mask_word`: Writes a fixed byte to a global variable and toggles a flag variable between zero and one.
- `process_cia_serial_byte`: Checks a status bit and, when set, sets a bit in a global and computes a rotated byte from the inverted value.
- `stub_vhposr_check_loop`: The routine reads the vertical beam position register according to its listed facts, but the decompiled body is only an endless loop.
- `load_highscores_if_ready`: Calls a function pointer from a structure and, when it returns non-zero, passes a fixed address to another function pointer and then calls a routine.
- `call_highscore_handler_when_clear`: When a global flag is zero, calls one routine, then a function pointer from a structure with a value, and then another routine.
- `xor_scramble_120_bytes`: XORs a block of 120 bytes in place against a descending counter from 120 down to one, and returns 120.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `stub_return_low_byte`: Returns the low byte of its data register input without changing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_title_setup_wait_choice`: Clears a display memory block, sets interrupt and copper registers, and copies setup tables into memory.
- `scan_byte_table_until_negative`: Walks a byte table, skipping three bytes after each zero entry and calling a routine for each other non-negative entry.
- `copy_character_glyph_rows`: Maps a character to a table index, with space mapping to zero, and copies eight bytes from a font table into a buffer.
- `store_interrupt_code_112`: Calls a routine, then stores the constant 112 in a global variable.
- `clear_longword_block_fixed_address`: Writes zero to 1421 consecutive longwords starting at a fixed memory address.
- `insert_entry_shifting_table_records`: Walks a table of 12-byte records and, when a stored field is below the value supplied in a data register, shifts the records from that position down by one entry to make room.
- `pass_through_call_wrapper`: Calls one routine and returns, with no other logic.
- `run_name_entry_input_loop`: Loops on a vertical beam position timing window and processes input codes: some move a cursor in a five-slot text buffer, one copies a template into it, and any other code is stored as a character.
- `clear_indexed_byte_and_global`: Writes zero to a byte selected by a global index value, then clears that global to zero.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `run_six_pass_call_sequence`: Calls one routine, then calls another routine six times in a loop, and finishes with three more calls.
- `call_two_routines_in_order`: Calls two routines in sequence and returns.
- `write_cia_timer_config_values`: Writes a fixed sequence of six values to a global variable.
- `compute_rotated_inverted_byte`: Reads a global byte, writes all ones, then stores a rotated inverted copy of the original byte and writes two fixed values.
- `loop_ten_bcd_steps_calls`: Runs ten passes, each adjusting a BCD counter from one to ten and calling routines five times in an inner loop before three more calls.
- `stub_self_assignment_four_writes`: Writes a global variable to itself four times, which has no effect, and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_dma_control_register`: Checks a global variable and sets a global variable to a constant if certain bits are set, then overwrites a global variable with a series of values from other global variables.
- `reset_global_variables`: Sets several global variables to zero or a fixed constant value.
- `store_register_to_ram`: Stores the current value of a data register into a global variable.
- `update_sprite_properties_2`: Updates an object or sprite structure in a table by decoding a command or value from a register.
- `initialize_memory_tables`: Initializes several memory locations and tables with specific addresses or constant values, and clears a range of memory using a loop.
- `clear_global_variables`: Sets several global variables to zero or specific constant values.
- `update_entity_bounds`: Checks a condition based on a pointer-indexed table and, if met, updates a table entry with a new bound, a new end-range, and a new offset-based value.
- `set_dma_control_register`: Writes a value derived from an input register and a constant to a global variable, which also affects a DMA control register.
- `decrement_counter_and_recursively_execute`: Decrements a value in a structure and recursively calls itself or other routines based on whether the value reaches zero or a certain condition.
- `process_instruction_stream`: Recursively parses and executes a sequence of instructions or commands loaded from a table.
- `execute_bytecode_instruction_2`: Recursively decodes and executes a custom instruction stream by updating a pointer, a counter, and sprite or object properties.
- `update_pointer_value`: Writes a constant to a memory offset and updates a value in a structure based on a comparison or an increment.
- `execute_opcode_stream`: Recursively parses a sequence of instructions or data-driven commands, updating a pointer or index and modifying a state-tracking or or counter.
- `execute_bytecode_instruction`: Decodes and executes a custom instruction stream by updating a virtual program counter, a state-dependent value, and a secondary pointer.
- `copy_and_increment_counter`: Copies a byte from a memory location to another and increments a short-integer counter at a different offset.
- `set_attribute_to_max_2`: stores a maximum byte value into a memory structure offset via a pointer held in a register.
- `update_attribute_and_index`: Updates a pointer or index or a memory structure and conditionally modifies a secondary attribute or a table-based lookup value.
- `update_attribute_and_index_2`: Updates a counter and a pointer-based table or attribute value based on a masked or checked byte.
- `update_attribute_index_and_pointer`: Increments an indexer and updates a pointer or attribute value based on a lookup table and a masked or checked byte value.
- `parse_opcode_stream`: Iterates through a sequence of instructions or data-driven commands, updating a pointer or index and modifying a state-tracking or or cou.
- `parse_instruction_stream`: Decodes a custom instruction stream or bytecode, updating a pointer, a counter, and a state-dependent value.
- `set_attribute_to_max`: stores a maximum byte value into a specific offset of a memory structure pointed to by a register.
- `process_opcode_stream`: Iterates through a sequence of instructions or or data-driven commands, updating a pointer or index and modifying a state-tracking or or counter-based variable.
- `update_entity_attributes_2`: Reads a sequence of values from a table and stores calculated addresses or values into a memory-based structure.
- `clear_memory_byte`: Sets a byte at a specific offset relative to a pointer to zero.
- `update_sprite_attribute_list`: Decrements a counter or resets a value, then iterates through a sequence of or-indexed or pointer-based values to update a list of memory offsets and associated control bytes.
- `update_entity_attributes`: Iterates through a sequence of attribute or pointer values from a table, calculating a new pointer or address for each and storing it in a list of entity properties.
- `clear_byte_at_offset_2`: clears a single byte in a memory structure or array using an offset from a pointer held in a register.
- `update_entity_coordinates`: Updates a list of object or sprite coordinates by reading or calculating pointer offsets from a table and storing the resulting memory addresses and certain status flags into a structure.
- `update_entity_kinematics_and_state_2`: Decrements or resets internal timers and updates position or velocity-based movement or direction-switching logic.
- `update_entity_kinematics_and_state`: Updates an entity's internal timers, position, and velocity-based movement or direction-switching logic.
- `update_entity_state_and_position_2`: Updates an entity's internal counters and state-machine flags, including a conditional indirect call to a routine via a table lookup.
- `update_entity_state_and_position_3`: Updates a global variable and a counter, modifies a byte in a structure, and performs an indirect call to another routine based on a table lookup.
- `update_entity_position_and_counter`: Resets a byte in a structure, calculates a new value from two pointers, stores it in a global variable and a structure, increments a counter, and performs an indirect call to another routine.
- `update_table_index_and_dispatch`: Copies a word to two memory locations, increments a counter, and updates a pointer to a table using the incremented counter as an index.
- `update_entity_physics_and_state`: Updates a pointer-based table of entity properties, applying a fixed-point or scaled-integer calculation to a position or velocity-related value.
- `update_timer_and_branch_to_instruction`: Updates a timer or counter and loads a new instruction pointer from a table before performing an indirect call.
- `update_pointer_and_state_2`: Calculates a new pointer using a table offset, updates a counter, and performs an indirect call to a new routine.
- `update_game_timers_and_interpolation`: Decrements or resets several internal counters and timers while performing fixed-point or scaled-integer interpolation on a pointer-based value.
- `update_entity_attribute_2`: Updates an attribute or property of an entity by calculating a new value using a bitwise-shifted or scaled multiplication and storing it back to a memory location.
- `update_entity_attribute_3`: Updates an entity's attribute or position using a scaled or interpolated value, while managing a counter or state-based transition or lookup.
- `reset_and_dispatch_entity_action`: Clears specific fields in an entity structure, increments an index, and performs an indirect call to a routine or jump table entry based on a value loaded from a pointer-indexed table.
- `update_state_and_pointer_2`: Copies a byte and a short, updates a counter, and conditionally copies a short value before performing an indirect call.
- `update_state_and_dispatch_instruction_2`: Calculates a new value for a byte in a structure, increments a counter, and dispatches a new routine via a table lookup.
- `update_state_and_dispatch_instruction_3`: Assigns a byte from a source to a destination, increments a counter, and dispatches a new routine via a table lookup.
- `update_state_and_pointer_3`: Copies several bytes, increments a counter, and performs an indirect call to a routine located via a pointer-based table lookup.
- `update_instruction_timer_and_dispatch`: Updates a timer or counter based on conditional checks of certain flags, then loads a new instruction or routine pointer from a table and dispatches it via an indirect call.
- `update_entity_state_and_counter`: Copies a byte to two different locations in a structure, updates a pointer-based value, increments a counter, and performs an indirect call to another routine using a table-based offset.
- `update_position_and_timer_2`: Adds two short values, stores the result in a global variable, increments a counter, and performs an indirect call.
- `update_entity_state_and_position_4`: Updates an entity's internal counters, timers, and position-related or or velocity-related values using fixed-point or scaled arithmetic.
- `update_pointer_and_state_3`: Stores a pointer and a counter, calculates a new pointer using a table offset, and performs an indirect call.
- `update_pointer_and_branch_2`: Adds a value from a pointer-referenced memory location to a global variable, updates a pointer using a table lookup, and performs an indirect call.
- `reset_pointer_and_branch`: Resets certain bytes in a global structure to zero or one, updates a pointer using a table lookup, and performs an indirect call.
- `update_entity_state_and_kinematics`: Updates an entity's internal state counters and applies fixed-point velocity or direction-based position adjustments.
- `update_entity_state_and_position`: Updates a state machine and position-related counters or pointers, including a conditional indirect call to another routine and a fixed-point or scaled-integer calculation for a coordinate or or offset.
- `update_position_and_timer`: Adds two short values and stores the result in a global variable and another memory location, increments a counter, and updates a pointer to a new table entry before performing an indirect call.
- `update_entity_kinematics`: Updates an entity's position, velocity, and direction using fixed-point arithmetic and a state-based counter.
- `decrement_and_check_counter`: Decrements a value stored in a table and checks if it has reached zero, potentially resetting a byte or updating a global variable.
- `update_timer_and_pointer_2`: Decrements or resets a byte, updates a counter, and performs an indirect call using a pointer-based table lookup.
- `reset_timer_and_update_pointer`: Sets a byte in a structure to a maximum value, increments a counter, and updates a pointer to a new table entry before performing an indirect call.
- `update_timer_and_pointer`: Decrements a byte from a source, copies a short value to a counter, and updates a pointer to a new table entry before performing an indirect call.
- `update_state_and_dispatch_instruction`: Modifies a timer or counter based on certain flags and updates a pointer or counter before dispatching a new routine via a table lookup.
- `update_entity_attribute`: Copies a byte from one memory location to another and modifies a byte in a table using a bitwise shift.
- `update_state_and_pointer`: Copies a byte, sets a byte to a constant, conditionally copies a short value, updates a short value, increments a counter, and updates a pointer to a new table entry before performing an indirect call.
- `reset_entity_fields`: Clears several bytes and a word in a memory structure to zero.
- `update_pointer_and_state`: Copies a pointer and a counter, calculates a new pointer using an offset from a table, updates a counter and a byte, and updates a pointer to a new table entry before performing an indirect call.
- `update_pointer_and_branch`: Updates a pointer and a counter, then performs an indirect call using a value fetched from a table.
- `update_sprite_properties`: Updates a sprite or object structure in a table based on a decoded command or value in a register.
- `compare_and_set_flag`: Compares a new byte value against a stored byte and updates a global flag based on the comparison result.
- `copy_and_clear_byte`: Copies a byte from one memory location to another and clears a byte at a different offset relative to a pointer.
- `clear_memory_values`: Sets a byte and a longword to zero at specific offsets relative to a pointer.
- `reset_hardware_and_globals`: Clears several global variables and modifies audio volume and DMA or interrupt control registers.
- `set_global_value`: Sets a global variable to a value derived from an input register and a constant, then calls another routine.
- `reset_entity_state_and_sync_attributes`: Resets a set of global or table-based status flags and clears a range of memory or a table or a buffer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_display_color`: Sets a global variable to zero and then copies a value from a register into that global variable before calling another routine.
- `configure_display_and_interrupts`: Modifies certain hardware registers including a color register, a DMA controller, and an interrupt enable register, while also updating a global variable via an interrupt request setting routine.
- `set_interrupt_request`: Sets a global variable to a specific value and modifies the interrupt request register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_bit_and_check_value`: Sets a specific bit in a byte at a memory offset, then calculates a bit-reversed or bit-shifted value from another memory location to compare against a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_bitmask_and_test_value`: Updates a bitmask in a memory location via a pointer and performs a conditional check against a bit-shifted value.
- `toggle_input_bit_and_verify`: Updates a bitmask in a memory location via a pointer and performs a conditional check against a bit-shifted value.
- `modify_bitmask_and_branch`: Updates a bitmask in a memory location via a pointer and performs a conditional check against a bit-shifted value.
- `check_global_range`: Repeatedly checks if a certain bitmask of a global variable falls outside a specific range.
- `set_global_values`: Overwrites a global variable with a sequence of constant values.
- `modify_global_bit_pattern`: Reads a global variable, then overwrites it with a bit-rotated version of its inverse and a new constant.
- `initialize_level_graphics_and_data`: Initializes a block of memory with a constant pattern, copies a source table into a target buffer, and resets a range of global variables.
- `update_global_value`: Sets a global variable to a specific constant value.
- `initialize_nested_loop_data`: Sets several memory locations or a pointer-based structure to specific constant values and then executes a nested loop that increments a pointer-based value.
- `double_global_value`: Doubles a global variable and then executes a loop that decrements a local variable until it reaches minus one.
- `traverse_and_update_list`: Iterates through a list, checking a bit in a pointer, and updates a pointer or offset and a constant value into a memory-based list.
- `populate_list_with_offsets`: Iterates a fixed number of times to store an incrementing pointer or offset and a constant value into a memory-based list.
- `set_list_parameters`: Writes several constant values and register-based values into a memory-based structure or table.
- `update_global_pointer`: Checks a global variable and, regardless of its value, updates it with a new address and performs a series of additive operations.
- `check_bit_flag`: Checks if a specific bit in a memory location is set and returns.
- `modify_global_variable`: Decrements a global variable by two if it is not zero, otherwise sets it to a constant value.
- `traverse_and_clear_list`: Iterates through a list, checking a bit in a pointer, and clears certain memory values or writes constant values to a memory-based structure.
- `modify_global_variable_2`: Updates a global variable by adding or subtracting a constant value.
- `iterate_table_and_branch`: Iterates through a table of unsigned shorts, performing conditional branches or calls based on a specific bit flag in each entry.
- `search_and_move_elements`: Iterates through a list of structures, checking for a specific value match, and if found, copies certain fields from the source structure to a new location in a list.
- `move_list_elements`: Searches a list for a matching value and, if found, copies fields from a source structure to a new location in a list while clearing the source entry.
- `update_table_entries`: Iterates through a table to find non-positive or zero values, then overwrites entries and updates a global variable using BCD adjustment.
- `update_structure_fields`: Decrements a byte in a structure and potentially flips a bit or resets it to a constant, then loads a value from a table into a structure offset.
- `update_list_counters`: Checks a bit in a structure, increments or resets a byte in that structure, and adds a value from a table-based lookup to a pointer.
- `decrement_global_pointer`: Decrements a global pointer or value, or if the pointer or its target meets certain conditions, updates the pointer or sets it to a constant.
- `process_overlapping_structures`: Iterates through a list of structures to find any that overlap with a certain area, and if a match is found, modifies a field and potentially calls another routine.
- `find_overlapping_structure`: Iterates through a list of structures to check if any structure's bounds overlap with a calculated range, and if so, updates a global variable and calls another routine.
- `insert_and_adjust_global`: Searches a list for a non-positive or zero value, inserts a structure containing register values, and increments a global variable with BCD adjustment.
- `write_structure_and_adjust_bcd`: Writes several values, including a constant, into a memory-based structure and then performs BCD adjustment or increments a global variable.
- `find_and_update_table`: Iterates through a table looking for a non-positive or zero value, then overwrites that entry and subsequent memory locations with a constant and register values.
- `insert_table_entry`: Searches a list for a non-positive or zero value and writes a constant and register values to a memory location.
- `write_table_entry`: Writes a constant and certain register values to a memory location pointed to by a register.
- `process_table_events`: Checks if a pointer or value within a table falls within a certain range, then modifies a global variable and potentially calls another routine or performs bcd-adjusted arithmetic.
- `update_state_table_entry`: Decrements a global counter and iterates through a table to update a state-related structure with new parameters.
- `update_global_structure`: Iterates through a linked list or table, updating a global pointer and replacing certain list elements with new values or data copied from a table-based source.
- `check_joystick_bit`: Checks a bit in a register and, depending on a global variable, executes a sequence of recursive calls to another routine.
- `decrement_list_values`: Iterates through a list and decrements certain values, resetting them to zero if they fall below a certain threshold or exceed a certain range.
- `increment_list_elements`: Iterates through a list and increments certain halfword values, resetting them to zero if they exceed a certain range.
- `filter_list_elements`: Iterates through a list and copies elements that meet a certain size or value criteria to a new location, while inserting a new value or a boundary marker if a certain condition is met.
- `filter_list_elements_2`: Iterates through a list, copying certain elements to a new location if they meet a size criteria, and inserts a new entry if a certain value is reached.
- `initialize_memory_structure`: Writes several constant values to a memory structure and sets certain halfwords to a value derived from a global variable.
- `find_list_node`: Searches a list for a non-zero entry and then calls a routine using an offset found in that entry.
- `calculate_and_call_offset`: Decrements a value in a structure, calculates a new offset, writes several values to a structure, and calls a routine via a pointer.
- `calculate_list_offset`: Calculates a new offset based on a global variable and a pointer, writes several values to a structure, and calls a routine via a pointer.
- `increment_list_index`: Increments a value in a list, calculates a new offset, writes several values to a structure, and calls a routine via a pointer.
- `update_offset_and_call`: Increments a value in a structure, calculates a new offset, writes several values to a structure, and calls a routine via a pointer.
- `set_list_offset`: Calculates a new offset and writes several values to a structure if a list index is valid, then calls a routine via a pointer.
- `decrement_list_index`: Decrements a value in a list, and if the result is above a certain threshold, calculates a new offset and writes values to a structure before calling a routine.
- `update_list_and_call`: Updates a memory-based structure with a calculated offset and several values, then calls a routine via a pointer or sets a list terminator.
- `process_list_and_call`: Calculates an offset, writes several values to a memory-based structure, and then calls a routine via a pointer or sets a list terminator.
- `insert_list_node_2`: Increments a global variable, searches for a list terminator, and inserts a new node containing register values and a constant into a list.
- `insert_list_node`: Searches for a specific position in a list and inserts a new entry containing data from registers, while also updating a separate entry in the list with a modified version of the input data.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `save_context_and_sync_timer`: Saves all current processor registers to a stack-based or memory-based context block, then checks a flag to potentially execute a timer-related routine and a decrementing timer wait.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `search_and_update_list`: Increments a global variable and searches through a list of signed shorts until a non-positive value is found, then writes two or three values or a pointer-offset value to that location.
- `update_list_entry`: Searches through a list of signed shorts until a non-positive or zero value is found, then writes two values to the identified location.
- `update_list_with_value`: Searches a table for a non-zero or negative value and replaces it with a calculated offset and a new value.
- `decrement_struct_member`: Iterates through a structure, decrementing a member and clearing a pointer or a global variable if a certain value is reached.
- `initialize_struct_members`: Waits for a specific bit in a structure member to clear, then initializes several offsets within that structure with specific constants or conditional values.
- `update_display_pointers`: Checks a global variable against certain bounds and updates a list of bytes at a specific offset if a match is found.
- `update_display_and_list`: Increments a global variable, performs BCD adjustment, and updates a list of pointers or values based on a calculated screen-space offset and a conditional check on a global variable.
- `process_joystick_input`: Reads certain bits from a global variable, potentially masking or setting bits based on the state of certain flags, and finally sets a global variable to a constant value.
- `update_global_state_and_branch`: Modifies a global variable using arithmetic or bitwise masking and performs conditional branching or recursive calls based on the resulting value.
- `set_global_constant`: Sets a global variable to a constant value.
- `check_global_variable`: Checks if a global variable is zero and returns if so.
- `check_global_zero`: Checks if a global variable is non-zero and returns if so.
- `evaluate_global_zero_status`: Checks if a global variable is non-zero and returns or exits based on the result.
- `calculate_masked_value`: Calculates a new value for a global variable using an input byte and a subtraction, then masks the result.
- `calculate_screen_offset`: Calculates a memory offset based on the difference between a global variable and a constant, multiplied by a fixed stride.
- `set_multiple_address_targets`: Waits for a bit flag to clear, then writes a value and a constant to a structure, repeating this for three different address offsets.
- `execute_multiple_subroutines`: Executes a sequence of other routines.
- `copy_small_table_segment`: Uses a masked input value to calculate an offset into a source table and copies several bytes to a destination pointer.
- `copy_large_table_segment`: Uses a masked input value to calculate an offset into a source table and copies a larger number of bytes to a destination pointer.
- `perform_self_assignment`: Checks if a global variable is equal to itself and performs a self-assignment if a condition is met.
- `decrement_global_with_reset`: Decrements a global variable and resets it to a specific value if it becomes negative, potentially calling another routine.
- `copy_table_offset_data`: Calculates a source table offset using a masked input value and copies a block of bytes to a destination pointer via a secondary routine.
- `copy_table_offset_segment`: Calculates a source table offset using a masked input value and copies a block of bytes to a destination pointer.
- `copy_table_to_destination`: Calculates a source offset using a masked input value and copies a block of data from a table to a destination pointer.
- `update_global_state_with_bounds_check`: Modifies a global variable by adding or subtracting two, or calls other routines based on its current value.
- `render_blitter_sprite_list`: Iterates through a table of sprite or block descriptors to configure the blitter or copper, calculating source and destination coordinates and setting blit size or attribute registers.
- `update_global_pointer_2`: Checks a global variable and updates it with a new or incremented value based on certain bit patterns or specific constant comparisons.
- `modify_global_array`: Modifies a sequence of values in a global array, including performing arithmetic on existing elements and copying or calculating new values into subsequent array indices.
- `populate_data_table`: Iterates through a table of 4-byte or 2-byte values, checking a status bit in a control structure, and writes the extracted values and associated metadata into a destination buffer.
- `shift_memory_block`: Performs a memory move or block copy, shifting bytes or or updating a range of memory based on the value of a byte read from a source pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `flush_pending_bit_values`: Checks a pending value word and, when it is non-zero, writes it to a global and stores nine for each set low-order bit.
- `reset_state_and_default_values`: Clears two state words, calls four routines, then writes zeroes and two fixed values, 15 and 1920, into the global variable.
- `store_input_byte_and_call`: Stores the low byte of its data register input in a state byte and then calls one routine.
- `decode_event_byte_into_channel`: Decodes an incoming event byte and updates a per-channel record.
- `initialise_channel_state_tables`: Stores several pointers and offsets derived from its input, then clears a 32-entry table of words and sets default values in it.
- `reset_audio_state_and_defaults`: Clears two state words, calls four routines, then writes zeroes and two fixed values, 15 and 1920, into the global variable.
- `jump_to_update_audio_channel_parameter`: Jump stub: passes control straight on to update_audio_channel_parameters.
- `reset_hardware_registers`: Clears several memory-mapped hardware registers, including audio period registers and DMA control, and sets certain global or hardware values to a fixed state.
- `decrement_timer_and_wait_flag`: Decrements a 16-bit timer in a record and, when it goes negative, reloads it from a global value and calls routines in a nested loop until a flag byte is zero.
- `tiny_addq`: Small routine of 1 instruction that adds 4 to a0.
- `interpret_sequence_command_byte`: Reads a command byte from a sequence and dispatches on its low nibble.
- `mark_channel_ended_and_advance`: Marks a channel record as ended, advances a counter in a paired word with wraparound back to a start value, calls one routine and sets the flag to all ones.
- `interpret_sequence_command_entry`: An alternate entry into the sequence command interpreter that first handles a countdown before dispatching commands.
- `interpret_sequence_command_from_table`: Loads a channel position from a table indexed by a data byte, then dispatches the next sequence command by its low nibble.
- `set_field_and_advance_step`: Stores a byte from the frame into a channel field and increments the channel's step counter, then returns.
- `set_channel_status_all_ones`: Sets a status byte in a channel record to all ones and returns.
- `jump_to_dispatch_sequence_command_entr`: Jump stub: passes control straight on to dispatch_sequence_command_entry.
- `process_audio_command_stream`: Iterates through a stream of command bytes to update audio channel parameters, such as updating pointers, counters, or or volume/pitch data, based on a decoded command type or opcode.
- `interpret_sequence_commands_loop`: Loops over sequence command bytes for a channel, advancing the step counter and dispatching on the low nibble of each byte.
- `process_sequence_command_stream`: Reads command bytes from a sequence and updates channel state, including position, step, and playback parameters.
- `dispatch_sequence_command_entry`: Sets the channel's position from a table indexed by a masked value, then dispatches the next sequence command by its low nibble.
- `save_position_then_dispatch_command`: Copies the channel's current position and step into backup fields, then loads the next position from a table and dispatches the command byte there.
- `decode_channel_word_group`: Reads a group of up to six words from a sequence and stores each into channel fields.
- `clear_channel_field_byte`: Clears a byte at a field offset in a channel record and returns.
- `clear_byte_at_offset`: Stores a zero byte at a specific offset relative to a pointer held in a register.
- `tick_channel_timer_and_flags`: Decrements a timer in a channel record, clears two flag bytes when the timer was already negative, and clears a pending flag.
- `clear_channel_flag_bytes_step`: Decrements a countdown in a channel record and clears its flag bytes when they are set.
- `advance_channel_step_and_dispatch`: Clears a few channel fields and advances the step counter.
- `dispatch_channel_table_command`: Copies two words into the channel state, increments a step counter, then reads the next sequence value and dispatches it through a jump table.
- `store_channel_offset_and_dispatch`: Stores the sum of two frame values in the channel record and a pointer slot, increments the step counter, and dispatches the next sequence value through a jump table.
- `store_channel_word_and_dispatch`: Copies a frame word into two channel fields, increments the step counter, and dispatches the next sequence value through a jump table.
- `hold_channel_and_dispatch_next`: When a hold flag is clear, sets it and advances the step counter.
- `countdown_channel_hold_and_dispatch`: Counts down a held value in a channel record, or sets the hold flag and advances the step counter when it is clear.
- `start_channel_from_record_pointer`: Builds a channel base pointer from a record and a table, sets step fields and a flag, then loads the first sequence value and dispatches it.
- `advance_channel_repeat_and_step`: Updates a channel on each tick by adding a step value to its position and counting down a repeat count.
- `set_scaled_table_value_dispatch`: Looks up a value in a 64-entry table using a frame byte plus a channel offset, scales it by an adjustment and stores it in the channel.
- `look_up_table_value_dispatch`: Looks up a value in a 64-entry table using a frame byte plus a channel offset, stores it in the channel, and then dispatches the next sequence command through a jump table.
- `reset_channel_fields_and_dispatch`: Clears several channel fields, advances the step counter, and dispatches the next sequence command through a jump table.
- `store_scaled_byte_and_dispatch`: Stores a computed byte made from a frame byte and a channel value into the channel, advances the step counter, and dispatches the next sequence command through a jump table.
- `store_frame_byte_and_dispatch`: Copies a frame byte into a channel field, advances the step counter, and dispatches the next sequence command through a jump table.
- `store_four_bytes_and_dispatch`: Copies four frame bytes into channel fields, advances the step counter, and dispatches the next sequence command through a jump table.
- `count_down_hold_and_dispatch`: Counts down a hold value in a channel record, or sets the hold flag when it is clear.
- `add_word_position_dispatch`: Sets two channel fields from a frame byte, adds a frame word to a running position, stores the result, and dispatches the next sequence command.
- `accumulate_word_and_dispatch`: Adds a frame word to a channel accumulator, stores the result in two places, advances the step counter and dispatches the next sequence command through a jump table.
- `update_channel_hold_and_step`: Counts down a hold value in a channel record, or handles the hold flag when it is set, and then sets the position from a table.
- `save_position_load_table_entry`: Saves the channel's current position and step into backup fields, then loads a new position from a table and dispatches the command at that point.
- `advance_position_by_frame_long`: Adds a frame longword to a channel position, stores the new position and a half-step value, advances the step counter and dispatches the next sequence command.
- `set_position_from_frame_long`: Sets a channel position from a frame longword, sets two fields to one, advances the step counter and dispatches the next sequence command through a jump table.
- `set_channel_hold_and_dispatch`: Sets a channel field from a frame byte, applies hold logic to a second field, and dispatches the next sequence command through a jump table.
- `hold_channel_or_toggle_interrupts`: Holds or counts down a channel's timer in the same way as the other command handlers, then dispatches the next sequence command.
- `update_sequence_and_dispatch`: Adds two values from different tables, updates a counter and a pointer, and performs an indirect call using a value from a table.
- `toggle_channel_hold_and_dispatch`: Clears a channel timer field and sets a hold flag, then dispatches the next sequence command through a jump table.
- `select_channel_status_tick`: Chooses one of three channel records according to bits in a status word, decrements that record's timer, and marks it inactive when the timer goes negative.
- `compare_and_branch_sequence_command`: Compares a frame byte with a channel byte, then either stores a frame word as the new position or advances the step counter.
- `process_flag_and_dispatch`: Modifies a flag byte based on its current value, updates a counter and a position, and dispatches the next command via an indirect call.
- `set_channel_flag_and_dispatch`: Sets a flag byte in a channel record to all ones, advances the step counter, and dispatches the next sequence command through a jump table.
- `set_channel_counter_and_dispatch`: Sets a counter field in a channel record from a frame byte minus one, stores a frame word as the new position, and dispatches the next sequence command through a jump table.
- `dispatch_audio_command`: Updates various fields in a data structure based on conditional checks and executes an indirect call using a value retrieved from a table.
- `look_up_scaled_table_entry`: Looks up a value in a 64-entry table using a frame byte plus a different channel offset, scales it by an adjustment and stores it in the channel.
- `store_frame_word_and_dispatch`: Stores a frame word into two channel fields, advances the step counter, and dispatches the next sequence command through a jump table.
- `reset_fields_and_dispatch`: Clears several fields in a structure, increments a counter, and executes an indirect call using a value retrieved from a table.
- `load_table_entry_and_dispatch`: Copies data to a structure, calculates a new pointer using an offset and a base address, and dispatches a command through an indirect call.
- `restore_position_and_dispatch`: Restores the channel's saved position and step from backup fields, advances the step counter and dispatches the next sequence command through a jump table.
- `update_audio_channel_parameters`: Parses a multi-byte command sequence to update a specific audio channel's control structure.
- `update_table_entry_2`: Calculates a source address using a pointer and an offset, then updates a table entry with a masked value, a new value, and a short.
- `update_table_entry`: Checks a condition based on a pointer and an offset, then updates a table entry with a masked value, a new value, and a short.
- `clear_record_byte_and_longword`: Clears a byte and a longword in a channel record and returns.
- `reset_audio_hardware_state`: Clears specific memory locations, updates channel table entries, and resets audio volume, DMA, and interrupt registers.
- `store_flagged_word_and_call`: Stores its data register input, with bit eight set, into a global word, then calls one routine.
- `update_channel_table_entry`: Copies a table word selected by a record field into a second table slot and updates two state words.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 232 into d6, then merges 68 into d6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `return_from_subroutine`: the routine is empty and performs no operations before returning.
- `return_void`: The routine is empty and performs no operations before returning.
- `return_from_subroutine_2`: The routine is empty and performs no operations before returning to the caller.
- `return_from_subroutine_3`: This routine is empty and performs no operations before returning.
- `return_no_operation`: The routine consists of only two instructions, with no register, memory, library, or hardware accesses, and simply returns to its caller.

### Quick (raw code file, load address unknown)

- `open_library_and_program_display`: The decompiler timed out, so no pseudo-code is available.
- `call_one_unnamed_routine`: The decompiler timed out, so no pseudo-code is available.
- `access_interrupt_request_register`: The decompiler timed out, so no pseudo-code is available.
- `access_second_colour_register`: The decompiler timed out, so no pseudo-code is available.
- `read_fire_port_colour_register`: The decompiler timed out, so no pseudo-code is available.
- `call_one_routine_only`: The decompiler timed out, so no pseudo-code is available.
- `read_vertical_position_and_call`: The decompiler timed out, so no pseudo-code is available.
- `stub_unresolved_routine`: The decompiler timed out, so no pseudo-code is available.
- `open_library_and_program_copper`: The decompiler timed out, so no pseudo-code is available.
- `call_two_routines_unresolved`: The decompiler timed out, so no pseudo-code is available.
- `forward_one_call_unresolved`: The decompiler timed out, so no pseudo-code is available.
- `access_cia_timer_serial_registers`: The decompiler timed out, so no pseudo-code is available.
- `access_display_timing_dma_registers`: The decompiler timed out, so no pseudo-code is available.
- `read_dma_interrupt_enable_state`: The decompiler timed out, so no pseudo-code is available.
- `access_cia_b_interrupt_dma`: The decompiler timed out, so no pseudo-code is available.
- `stub_unresolved_five_instructions`: The decompiler timed out, so no pseudo-code is available.
