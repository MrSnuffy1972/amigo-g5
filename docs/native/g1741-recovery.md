# Game 1741 (xybots (1989)(domark)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01741). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (951120 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 94 raw-word lines |
| Functions named and described | pass, 275 of 275 named (0 author symbols), 275 of 275 described (27 by rule, 248 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 5071 of 5071 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 12288 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8067 of 8067 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 19613 |
| code | 30980 |
| data | 863112 |
| padding | 39 |
| unused | 37376 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Xybots (1989)(Domark)[a].adf:bootblock | 1 | 8 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3363 | 85 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 819 | 32 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 687 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3034 | 137 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 156 | 6 |
| Xybots (1989)(Domark)[a].adf bytes 56320-68608 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- xybots (1989)(domark)[a]/Xybots (1989)(Domark)[a].adf: custom-format (non-DOS) disk, 1685 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 2088 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 649 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 597 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1648 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 89 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- xybots (1989)(domark)[a]/Xybots (1989)(Domark)[a].adf: bytes 56320-68608 look like code no run loaded: recovered position-independently (load address unknown)
- xybots (1989)(domark)[a]/Xybots (1989)(Domark)[a].adf: 1 logged loads cover 12288 bytes of the image
- xybots (1989)(domark)[a]/Xybots (1989)(Domark)[a].adf: 13 track-loaded blocks found in recovered runtime segments and classified through them
- xybots (1989)(domark)[a]/Xybots (1989)(Domark)[a].adf: blocks 2-26 (bytes 1024-13824) outside the filesystem hold data, not code (opcode density 0.006, entropy 5.80)
- xybots (1989)(domark)[a]/Xybots (1989)(Domark)[a].adf: blocks 28-101 (bytes 14336-52224) outside the filesystem hold data, not code (opcode density 0.024, entropy 6.48)
- xybots (1989)(domark)[a]/Xybots (1989)(Domark)[a].adf: blocks 141-141 (bytes 72192-72704) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.30)
- xybots (1989)(domark)[a]/Xybots (1989)(Domark)[a].adf: blocks 146-219 (bytes 74752-112640) outside the filesystem hold data, not code (opcode density 0.013, entropy 5.20)
- xybots (1989)(domark)[a]/Xybots (1989)(Domark)[a].adf: blocks 222-222 (bytes 113664-114176) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.85)
- xybots (1989)(domark)[a]/Xybots (1989)(Domark)[a].adf: blocks 224-278 (bytes 114688-142848) outside the filesystem hold data, not code (opcode density 0.044, entropy 5.22)
- xybots (1989)(domark)[a]/Xybots (1989)(Domark)[a].adf: blocks 282-287 (bytes 144384-147456) outside the filesystem hold data, not code (opcode density 0.043, entropy 5.79)
- xybots (1989)(domark)[a]/Xybots (1989)(Domark)[a].adf: blocks 289-304 (bytes 147968-156160) outside the filesystem hold data, not code (opcode density 0.042, entropy 5.14)
- xybots (1989)(domark)[a]/Xybots (1989)(Domark)[a].adf: blocks 306-343 (bytes 156672-176128) outside the filesystem hold data, not code (opcode density 0.015, entropy 3.27)
- xybots (1989)(domark)[a]/Xybots (1989)(Domark)[a].adf: blocks 348-348 (bytes 178176-178688) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- xybots (1989)(domark)[a]/Xybots (1989)(Domark)[a].adf: blocks 355-415 (bytes 181760-212992) outside the filesystem hold data, not code (opcode density 0.005, entropy 3.68)
- xybots (1989)(domark)[a]/Xybots (1989)(Domark)[a].adf: blocks 419-610 (bytes 214528-312832) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.01)
- xybots (1989)(domark)[a]/Xybots (1989)(Domark)[a].adf: blocks 615-676 (bytes 314880-346624) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.77)
- xybots (1989)(domark)[a]/Xybots (1989)(Domark)[a].adf: blocks 683-742 (bytes 349696-380416) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.00)
- xybots (1989)(domark)[a]/Xybots (1989)(Domark)[a].adf: blocks 747-781 (bytes 382464-400384) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.18)
- xybots (1989)(domark)[a]/Xybots (1989)(Domark)[a].adf: blocks 813-1759 (bytes 416256-901120) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.78)
- 1 never-loaded region(s), 12288 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Xybots (1989)(Domark)[a].adf:bootblock

- `store_byte_into_table`: A single-instruction routine that loads the address of a table into a register and then stores a byte from a register through that pointer.
- `store_byte_via_pointer`: Loads the address of a table into a register and then stores a byte from a register through a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_sprite_mask_to_bitmaps`: Iterates through a list of sprite data and applies a bitwise mask to corresponding bitmapped graphics memory.
- `update_sprite_mask_buffer`: Calculates a bitmask or sprite-related bitset using bitwise rotations and logical operations, then writes the result into a destination buffer.
- `update_bitmask_graphics`: Calculates a bitmask or sprite-like pattern using bitwise shifts and logical operations, then applies it to a destination buffer using bitwise AND/OR operations.
- `update_sprite_mask_and_position`: Calculates a new bitmask or sprite attribute or a pointer-based buffer and applies bitwise logical operations to update a pattern or mask in memory.
- `update_sprite_mask_and_position_2`: Calculates a new memory offset based on a coordinate or index and clears or updates specific bits in a bitmask-based sprite or object table.
- `update_sprite_mask_and_position_3`: Calculates a new memory offset based on a coordinate or index and clears a range of memory or a bitmask.
- `update_sprite_bitmask`: Calculates and applies bitmasking or masking operations to a buffer or table of sprite/object data.
- `update_sprite_bitmask_2`: Calculates and writes bitmask or sprite-related pattern data to a memory buffer using bitwise shifts and logical operations.
- `increment_structure_field`: Retrieves pointers to a global variable and increments a value at a specific offset within that structure.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `call_subroutines_with_offset`: Calls two different routines, the first one using an argument or register value shifted left by three bits.
- `blit_bitplane_data`: Copies bitplane data from a source table to a destination buffer using bitwise operations to handle non-word-aligned shifts.
- `apply_bitmask_to_memory_blocks`: Calculates a bitmask by performing bitwise operations on a sequence of words from a table.
- `process_table_bits`: Iterates through two tables, clearing a specific bit in the first table and calling other routines based on the state of bits in both tables.
- `apply_bitmask_to_buffers_2`: Checks a global variable against a range and then iterates through a loop to perform bitwise OR operations on two separate memory regions using a bitmask.
- `tiny_move_andi`: Small routine of 2 instructions that copies (a4) to d5, then masks d5 with 15.
- `apply_bitmask_to_buffers`: Reads a sequence of bytes from a source buffer and performs a bitwise OR operation to apply them to two separate destination memory regions.
- `modify_bitmask_table`: Iterates through a table and performs bitwise OR or bitwise AND operations on specific bytes based on the value of a global variable.
- `clear_memory_block`: Fills a block of memory starting at a fixed address with zeros using a loop.
- `clear_memory_blocks`: Iterates through two memory regions in fixed increments and calls a routine to write values to specific offsets within those blocks.
- `write_multiple_values`: Writes a 32-bit value from a register to several specific offsets within two different memory blocks.
- `overwrite_memory_with_bits`: Clears the high bit of specific bytes in a table and then stores bytes from a source pointer into those same locations.
- `apply_bitwise_or_to_table`: Performs a bitwise OR operation between bytes from a source pointer and specific entries in a destination table.
- `clear_table_entries`: Sets a series of specific byte locations within a table to zero.
- `apply_relocation_hunks_2`: Iterates through a sequence of relocation or executable hunk type codes, using a bitmask to update specific memory locations with new byte values.
- `apply_relocation_hunks`: Iterates through a sequence of bytes and applies them to specific memory locations by masking the target byte with a bitmask and performing a bitwise OR operation.
- `process_entity_event_loop`: Iterates through a table of data structures based on a global count.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `apply_bitmask_to_memory`: Performs a bitwise OR operation between a byte and several specific offsets within a memory block pointed to by a register.
- `apply_bitmask_subset`: Performs a bitwise OR operation between a byte and four specific offsets within a memory block.
- `apply_bitmask_to_offsets`: Performs a bitwise OR operation between a byte and four different offsets within a memory block.
- `test_memory_bytes`: Tests if certain bytes at specific memory addresses are non-zero and branches if they are.
- `apply_bitmask_extended`: Performs a bitwise OR operation between a byte and six specific offsets within a memory block.
- `apply_bitmask_partial`: Performs a bitwise OR operation between a byte and six specific offsets within a memory block, including the base address.
- `update_global_variable_state`: Calculates the absolute difference between a global variable and a value stored in a memory structure, then updates the global variable based on certain bitwise conditions and a secondary lookup in a table.
- `return_stack_value_3`: Retrieves a value from the stack and returns it to the caller.
- `subtract_and_branch`: Loads a byte from a pointer, subtracts another byte from it, and branches or loads new values into registers based on the result.
- `load_constants_and_branch`: Loads two immediate values into registers and performs an unconditional branch to another location.
- `return_stack_value_2`: Returns a value that was previously placed on the stack.
- `return_stack_value`: The routine retrieves a value from the stack and returns it to the caller.
- `call_indirect_from_table`: Calculates a target address using a value from a register and an offset from a global table, then calls that address.
- `execute_indexed_function`: calculates an offset from a global pointer using a value from a register and calls the resulting function pointer.
- `check_bit_status_2`: Shifts and masks a register, or-s it with another register, and then tests a specific bit at a memory location offset by a register value.
- `test_bit_at_offset`: Modifies a register to create a bitmask and checks if a specific bit is set at a calculated offset in a table.
- `test_bit_at_offset_2`: Modifies a register to create a bitmask and checks if a specific bit is unable to be cleared at a calculated offset in a table.
- `check_bit_status`: Checks if a specific bit is set at a calculated memory location and returns if the bit is zero.
- `execute_callback_list`: Checks a global variable and a table entry, then iterates through a list of pointers to execute several subroutines based on specific conditions.
- `mask_byte_bits`: Performs a bitwise OR between a byte and a constant value.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `check_byte_threshold`: Compares a byte at a specific offset in a structure against a value and calls another routine if the condition is met.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_11`: Empty routine: returns to the caller immediately without doing anything.
- `update_and_process_value`: Subtracts a byte from another and updates a structure; if the result is negative, it modifies a global variable, updates several structure fields, and calls another routine.
- `decrement_and_call_routine`: Performs a subtraction on a byte within a structure and, if the result is negative, decrements a global variable and calls another routine.
- `clear_entity_attribute_tables`: Decrements two global counters and, if non-zero, iterates through two separate memory blocks to reset or mask specific word-aligned values.
- `apply_relocation_patch_2`: Iterates through a range of memory offsets to apply a specific word value and subsequently clears a secondary range of memory offsets to zero.
- `apply_relocation_patch`: Iterates through a memory range to apply specific word-sized values to a series of fixed offsets relative to a base pointer, then zeros out corresponding offset locations based on counts derived from a register.
- `apply_relocation_patches`: Performs complex conditional checks and updates various global variables and memory locations, including calls to patch relocation hunks.
- `decrement_and_check_zero`: Decrements a global variable and enters an infinite loop if the value reaches zero.
- `update_global_counters`: Updates several global variables by incrementing or decrementing them and overwrites a global variable with a constant value.
- `poll_routine_loop`: Calls another routine and enters a loop that repeatedly calls that same routine until it returns a true value.
- `update_offset_table`: Iterates through a table of 64 values, adding a constant to each and wrapping the value if it reaches a specific threshold.
- `populate_offset_tables`: Stores a 16-bit value and several derived values incremented by fixed offsets into multiple global locations.
- `store_mouse_data`: Stores a byte from a register into a global variable.
- `update_mouse_position`: Reads a word from a hardware port, compares it against a global variable, and updates that variable or performs arithmetic if certain conditions are met.
- `modify_global_pointer`: Adds a byte from a register to a global pointer and performs conditional checks and bit rotations.
- `increment_global_byte`: Adds a byte from a register to a global variable.
- `compare_byte_against_list`: Compares a register against a series of bytes stored in a table and returns if a match is found.
- `combine_input_results`: Calls another routine twice and returns the bitwise OR of the returned value and a value from a register.
- `calculate_input_state`: Performs bitwise operations on a global variable and returns a combined 32-bit value.
- `combine_input_flags`: Takes a value from another routine and performs bitwise OR operations on it based on specific bits set in a global variable.
- `test_input_and_branch`: Loads a byte from a hardware register and branches or calls another routine depending on whether the sign bit is set.
- `execute_conditional_polls`: Calls another routine multiple times and returns a byte that is conditionally modified by a bitwise OR operation.
- `scan_sprite_attribute_table`: Iterates through a table of data structures and checks if a specific field within each entry is zero, calling another routine if a zero value is found.
- `empty_stub_12`: Empty routine: returns to the caller immediately without doing anything.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `tiny_not_not_rts`: Small routine of 3 instructions that inverts d0, then inverts d1, then returns.
- `invert_byte`: Returns the bitwise NOT of a byte from a register.
- `calculate_interpolated_entity_state`: Performs fixed-point coordinate interpolation and scaling based on a timer value and a lookup table.
- `execute_indirect_call`: Calculates an address by indexing into a table using a global variable, applies an offset, and performs an indirect jump to the resulting location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_indexed_indirect_call`: Calculates an address by indexing into a table using a global variable, applies a constant offset, and performs an indirect jump to the resulting location.
- `process_byte_offset_subtract`: Subtracts three from a byte located at an offset from a pointer and masks the result.
- `process_byte_offset_add`: Adds four to a byte located at an offset from a pointer and masks the result.
- `subtract_and_mask_byte`: Subtracts two from a byte located at an offset from a pointer and returns the result masked with fifteen.
- `add_and_mask_byte`: Adds two to a byte located at an offset from a pointer and returns the result masked with fifteen.
- `clear_memory_block_2`: Clears a block of memory by writing zero to sixty-four consecutive double-word locations starting from a fixed address.
- `process_nested_table_data`: Iterates through nested loops to perform bitwise checks on memory based on values retrieved from a table and a pointer.
- `copy_structured_data`: Performs a multi-level block copy by reading from a source table and writing to a destination calculated from a pointer and anoffset.
- `copy_table_data`: Copies sixteen words from a fixed source table to a destination address calculated from a global variable and another offset.
- `clear_memory_block_3`: Iterates through a specific memory range, writing zero to each four-byte word.
- `update_global_tables`: Or-accumulates values from a source table into two different destination tables and conditionally copies data from a third source.
- `apply_offset_to_table`: Calculates a destination address using a global variable and a table value, then adds values from a source to a destination table using a bitwise mask.
- `update_pixel_data`: Combines bytes from two sources, calculates a value, and performs bitwise OR operations on two specific memory locations.
- `compare_and_update_data`: Iterates through a table of data to compare byte pairs against values derived from a global variable and a pointer.
- `search_and_assign_index`: Compares two bytes from a source against a target pair in a loop and stores a descending counter value into a destination byte if a match is found.
- `calculate_address_offset`: Calculates a new address in a table using a value from a register and a base pointer, then calls another routine.
- `copy_data_block`: Copies several bytes from one memory location to another at specific offsets.
- `recursively_reset_global_flag`: Checks if a global variable is non-zero, resets it to zero and calls itself recursively, then checks again to reset the variable to one and calls itself recursively.
- `fill_memory_loop`: Fills a series of memory locations with the value one hundred twenty-seven using a loop.
- `process_encoded_instruction_stream`: Iterates through a nested loop of encoded bytes to decode or expand instructions, updating a destination buffer and conditionally calling other subroutines based on certain bit patterns or prefix-like values.
- `update_pointer_value`: Checks a global variable and, if non-zero, performs several assignments before storing a byte and an incremented pointer into a global variable.
- `set_global_constants`: Stores four specific constant values into a global variable.
- `set_global_constants_alt`: Stores four specific constant values into a global variable in a different order than another routine.
- `empty_stub_13`: Empty routine: returns to the caller immediately without doing anything.
- `populate_data_structures`: Iterates through a memory buffer to write a series of structured byte sequences and control characters.
- `initialize_object_structure`: Clears certain bits in a byte, sets several fields in a structure pointed to by a global variable, and increments the pointer.
- `initialize_object_structure_alt`: Clears multiple bits in a byte and populates a structure with specific values before incrementing the pointer.
- `initialize_object_structure_v3`: Clears bits in a byte and sets specific fields in a structure pointed to by a global variable.
- `initialize_object_structure_v4`: Clears bits in a byte and sets specific fields in a structure pointed to by a global variable with different field values.
- `initialize_object_structure_v5`: Clears bits in a byte and sets specific fields in a structure pointed to by a global variable with a different offset value.
- `initialize_data_structure`: Clears several fields in a data structure and sets specific byte and word values at fixed offsets.
- `initialize_data_structure_alt`: Resets a data structure by writing specific constants to various offsets and updating a global pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_d0_register`: clears the least significant byte of a data register.
- `update_pointer_and_branch`: Increments a byte at an offset from a pointer, clears a global variable, and performs an indirect call to a destination calculated from a global variable and a register-based offset.
- `update_entity_table_and_sort`: Validates a range of entity or sprite attribute data and performs a bubble sort on the table if certain flags or bounds checks pass.
- `update_entity_table_and_sort_2`: Validates a range of entity or sprite attribute data and performs a bubble sort on the table if certain flags or bounds-checks fail.
- `validate_and_sort_entity_table_2`: Checks a range of memory for valid attribute flags and, if valid, performs a bubble sort on a table of values.
- `validate_and_sort_entity_table`: Checks a range of memory for valid attribute flags and, if valid, performs a bubble sort on a table of values.
- `sort_table_elements`: Performs a bubble sort on a table of values until the list is ordered.
- `sort_entity_list`: Performs a bubble sort on a list of table elements, swapping values if a certain condition is met.
- `sort_entity_attributes`: Performs a bubble sort on a table of values, swapping elements if a certain condition is met, and updates a secondary table with a modified value and a calculated offset or identifier.
- `sort_entity_table`: Performs a bubble sort on a table of values until the list is ordered.
- `sort_entity_table_2`: Performs a bubble sort on a table of values, swapping elements if a certain condition is met, and updates a pointer or value in the table during the process.
- `expand_entity_attributes`: Iterates through a list of entity indices and expands or copies a set of attribute bytes from a source table into a destination buffer.
- `copy_memory_block`: Copies a block of data from one memory location to two different destination locations using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `toggle_flag_and_call_table`: It increments a global variable, masks the result to one bit, then repeatedly decrements it while clearing a flag bit in a structure, and finally runs a small loop that reads a pointer table and calls each entry.
- `do_nothing`: This routine is empty and performs no operations.
- `run_bonus_state_once`: Runs a one-shot routine that, on first entry, performs the bonus-state update and captures a return value, then on subsequent entries checks a counter threshold and resets the machine's fields when exceeded.
- `advance_bonus_state`: Initializes a state machine by storing a sentinel value and pointing a global at a structure, then branches on the structure's counter to either advance it or apply a bonus amount and a final state value.
- `handle_bonus_progress`: Increments a byte in a structure when a flag is in a certain range, then inspects other structure fields to add a value and set flags before running a shared cleanup sequence.
- `handle_warp_bonus_state`: Checks several byte fields in a structure and, on a match, updates a global flag, adds to a pointed-to value, and stores a pointer, then runs a longer sequence that resets flags and calls another routine.
- `reset_and_toggle_flags`: Tests a counter against a threshold and, when exceeded, resets several fields, clears a flag, calls another routine, and sets a final flag before returning.
- `set_value_and_call_twice`: It stores a pointer to a global variable, sets that variable to a fixed byte value, then calls another routine twice while clearing a second pointer slot between the calls.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `toggle_bit_in_byte`: It toggles a single bit within a byte at a fixed offset in a structure, using a shift amount derived from a register.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `return_d0_byte`: Passes the byte already in data register D0 straight back to the caller without changing it.
- `return_byte_from_d0_pointer`: Reads a byte from the memory location pointed to by register D0 and returns that byte unchanged.
- `tiny_ori_rts`: Small routine of 2 instructions that merges 0 into d0, then returns.
- `reset_globals_then_call`: It resets a global variable twice when nonzero, then calls a single other routine and returns.
- `store_bytes_as_words`: It adds a constant to three consecutive bytes from an input pointer and stores each resulting value as a word into three consecutive word slots in a structure.
- `store_fixed_word_value`: It writes a fixed word value into a word slot at a fixed offset within a structure.
- `store_another_word_value`: It writes a different fixed word value into a word slot at a fixed offset within a structure.
- `handle_bonus_state_update`: Detects a bonus-state trigger and, on first entry, records the reward digits into a display table while capturing the result.
- `update_counters_and_reset`: Increments two counters and, upon reaching specific thresholds, resets a global variable and a timer function pointer.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `increment_byte_with_flags`: It flips a flag bit in a byte at a fixed offset, then increments a following byte and masks it, resetting a nearby byte when the incremented value wraps around.
- `increment_and_set_values`: Increments a byte at an offset and, if a specific bitwise condition is met, writes a word and a long value to the structure.
- `advance_object_state_machine`: Adjusts a per-object counter and a two-bit state field when a flag is set and a countdown reaches zero, then dispatches a scripted action from an indexed table based on the resulting state.
- `decrement_and_branch`: Decrements a byte through a pointer and branches if no carry occurs, otherwise decrements a byte at a fixed offset from a pointer and jumps to a new address.
- `increment_and_branch`: Adds a byte to a value through a pointer and branches if no carry occurs, otherwise increments a byte at a fixed offset from a pointer and jumps to a new address.
- `decrement_and_mask_byte`: Subtracts a byte from a memory location and, if no carry occurs, decrements and masks a byte in a secondary memory location before jumping to another routine.
- `increment_with_table_lookup`: Adds a byte to a value and, if a carry occurs, increments a counter and searches a table to set two other bytes.
- `advance_entity_state_machine`: Updates a per-entity state machine by incrementing and masking a two-bit field, adjusting a neighboring counter on wrap, and recording a flag or status byte.
- `update_entity_state_with_branch`: Decrements a byte through a pointer and branches if no carry occurs, otherwise decrements a byte at a fixed offset, calls another routine, clears a bit in a pointer-relative byte, and branches to another routine.
- `update_entity_state_with_branch_add`: Adds a byte to a value through a pointer and branches if no carry occurs, otherwise increments a byte at a fixed offset, calls another routine, clears a bit in a pointer-relative byte, and branches to another routine.
- `update_entity_state_with_branch_sub`: Decrements a byte through a pointer and branches if no carry occurs, otherwise decrements a byte at a different fixed offset, calls another or another routine, clears a bit in a pointer-relative byte, and branches to another routine.
- `update_entity_state`: Increments a byte and, if a carry occurs, increments a counter and copies data from a source pointer to a table offset.
- `increment_byte_modulo_four`: Increments a byte at a fixed offset within a structure, masks it to a two-bit value, and loops back into itself while that masked value equals zero.
- `set_word_constant`: Stores a constant word value into a memory location at a fixed offset.
- `process_entity_lifecycle_and_dispatch`: Decrements a counter or a structure member and, if a certain condition is met, updates a global bitmask and executes an indirect call or a loop of indirect calls.
- `decrement_with_carry_check`: Subtracts a byte from another and, if a borrow occurs, decrements a counter and calls another routine.
- `increment_byte_with_carry`: Adds a byte to a value at a pointer and increments a nibble-masked byte at an offset if a carry occurs.
- `increment_byte_offset_one`: Adds a byte to a value at a pointer plus one and increments a nibble-masked byte at an offset of seven if a carry occurs.
- `decrement_byte_with_borrow`: Subtracts a byte from a value at a pointer and decrements a nibble-masked byte at an offset if a borrow occurs.
- `increment_byte_after_call`: It calls another routine, then flips a flag bit in a byte and increments a following byte, clearing that byte when it reaches a specific value.
- `check_and_update_counters`: Tests a byte in a structure and, if non-zero, loads a pointer and compares a global word against a constant.
- `toggle_and_update_state_counters`: Toggles a bit in a structure, increments or decrements a global variable, and conditionally updates or resets secondary counters and a pointer-based value based on certain threshold checks.
- `check_byte_and_decrement`: Compares a byte at an offset in a structure against certain constants and, if a match is found, decrements a word through a pointer.
- `tiny_addq_rts`: Small routine of 2 instructions that adds 1 to (a5), then returns.
- `split_and_set_nibbles`: Increments a value through a pointer, increments a byte through a pointer, and if a certain value is reached, sets a byte to a fixed value and splits a global byte into two nibbles stored at pointer-relative offsets before setting the global variable to a fixed value.
- `split_byte_into_nibbles`: Splits a global byte into two nibbles and stores them at specific offsets from a pointer, then sets the global variable to a fixed value.
- `empty_stub_14`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_structure_and_clear_global`: Decrements a word through a pointer, decrements a byte through a pointer, and if a certain condition is met, clears bytes through a pointer, checks a byte through a pointer, and if non-zero, moves a constant, a zero word, and a constant byte into a structure before clearing a global variable.
- `initialize_data_structure_2`: Stores a constant, a zero word, and a constant byte into a structure at specific offsets from a pointer, then clears a global variable.
- `empty_stub_15`: Empty routine: returns to the caller immediately without doing anything.
- `modify_global_bitfield`: Sets a global variable to a fixed value, and if a value in a structure exceeds a certain threshold, calculates an offset to clear a bit in a global variable, sets a value in a structure, and calls another routine, otherwise increments a value in a structure.
- `update_entity_state_and_pointer_2_2`: Updates a global bitfield and entity-related counters or pointers based on conditional checks or threshold-based increments.
- `update_entity_state_and_pointer_2`: Updates a global bitfield and entity-related counters or pointers based on conditional checks or threshold-based increments.
- `update_entity_state_and_pointer`: Updates a counter or pointer-based structure and conditionally modifies a global variable or a table entry based on a bitmask check.
- `initialize_structure_fields`: Stores a longword, a word, and a byte into a structure pointed to by a register, and copies its own address into that structure.
- `update_conditional_data`: Checks if a pointer matches its own address and then updates several global variables with specific values based on a global flag.
- `initialize_data_structure_4`: Sets specific word and longword values in a structure based on a global flag and then copies the routine's own address into that structure.
- `check_address_offset`: Calculates the difference between the current routine address and a value from a pointer, then updates a structure with new values if the absolute difference is two.
- `clear_d0_register_2`: Clears the lower byte of a data register.
- `clear_d0_register_3`: clears the low byte of a data register.
- `clear_d0_register_4`: Clears the least significant byte of a data register to zero.
- `clear_d0_register_5`: clears the lowest byte of the d0 register to zero.
- `clear_d0_register_6`: clears the least significant byte of the d0 register to zero.
- `update_offset_value`: Checks a value at an offset from a pointer and, if certain conditions are met, increments a byte at a specific offset and performs a calculation involving another memory location.
- `clear_register_d0`: Clears the least significant byte of register d0.
- `execute_indirect_call_2`: Checks a bitmask in a global variable and a byte at an offset before calling a function pointer from a table.
- `iterate_function_table`: Iterates through a list of values, checking a bitmask in a global variable and calling functions from a table based on those values.
- `return_from_subroutine_2`: Performs a single return instruction to exit the current function.
- `return_from_subroutine_3`: performs a single return instruction to exit the current function.
- `exit_subroutine`: This routine performs a single return instruction to exit the current function.
- `empty_stub_16`: Empty routine: returns to the caller immediately without doing anything.
- `increment_and_call`: Increments a byte in a structure and stores a byte into another structure using an offset derived from a register before calling a function from a table.
- `update_global_and_dispatch_indirect`: Updates a global variable based on a conditional check of a structure member, or otherwise modifies a structure's fields and executes an indirect call using a calculated offset from a table.
- `update_entity_lifecycle`: Decrements a global variable and performs several conditional updates to a structure or a global bitmask before executing an indirect call via an indexed table.
- `empty_stub_17`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_18`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_19`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_20`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_21`: Empty routine: returns to the caller immediately without doing anything.
- `update_entity_state_and_dispatch`: Decrements a value in a global array and, if a certain condition is met, resets the entry and updates a global counter or bitmask.
- `clear_global_variables`: It sets and clears a global variable, zeroes a four-byte region, and then clears a run of four-word slots starting at that global variable.
- `find_free_slot_and_call_indexed`: It scans a table of words to find the first zero slot, records the count of filled slots, increments a counter, marks the found slot, then stores incoming values and calls an indexed routine.
- `search_and_execute_function`: Searches a list of pointers for a null terminator, updates a structure with a count and a pointer, and then calls a function pointer retrieved from a table using an offset.
- `decrement_count_and_pointer`: Calculates a difference between a value in a source pointer and a value in a destination pointer, then updates a count and a pointer in a structure.
- `update_count_and_offset`: Calculates a difference between a byte in a source pointer and a byte in a destination pointer, then adds this difference to a byte in a structure.
- `update_count_and_pointer_offset`: Calculates a difference between a value in a source pointer and a value in a destination pointer, then updates a count and a pointer offset in a structure.
- `update_structure_data`: Copies data from one structure to another and updates certain fields or flags based on a comparison or a global variable.
- `initialize_data_structure_3`: Stores a longword, a word, and a byte into a structure pointed to by a register before calling another routine.
- `decrement_table_entry`: Clears a word in a table indexed by a value from a structure and decrements a counter entry within that same table, then sets a flag byte in the structure before returning.
- `modify_structure_fields`: Clears a word in a structure using an offset from a pointer, decrements a global variable, and stores a byte into the structure.
- `interleave_copy_words`: Copies a block of 16-bit words from a source table into four interleaved regions of a destination table, advancing the source faster than the destination, and only when a global flag is set.
- `clear_global_memory`: Writes zero into a long run of consecutive global memory locations, spanning several address ranges, without reading any of them first.
- `decrement_global_counter`: Subtracts one from a global counter twice when it is nonzero, repeating the same conditional decrement a second time before returning.
- `decrement_counter_six`: Decrements a global counter six times in two groups of three, each group guarded by a nonzero check, before returning.
- `call_table_function`: Calls a function pointer retrieved from a table using an offset stored in a structure.
- `modify_global_and_accumulate_2`: Checks if a global variable is non-zero, then calculates a new value for that global variable using a bitwise NOT and a right shift on a value, and adds a register to a global variable.
- `update_global_values`: Stores a word, two bytes, and a longword into consecutive global variables, then adds the value of a register to another global variable.
- `modify_global_and_accumulate`: Checks if a global variable is non-zero, then calculates a new value for that global variable using a bitwise NOT and a right shift on a value from a pointer, followed by adding a register value to a global variable.
- `update_global_state_constants_2`: Calculates a value from a bit-shifted, inverted byte and stores it or a modified version into a global variable, then overwrites a sequence of a word, a byte, and a longword in consecutive global variables before adding a register value to a final global variable.
- `update_global_state_constants`: Stores a word, a byte, and a longword into consecutive global variables, then adds a register value to another global variable.
- `call_indexed_function`: Checks a byte at an offset from a pointer and, if the value is less than two, calls a function from a table using an index derived from another pointer.
- `update_global_from_byte`: Shifts a byte from a structure and adds a constant to it, then stores the result in a global variable, or otherwise stores a shifted byte and a longword into consecutive global variables and adds a register to a global variable.
- `update_global_values_2_2`: Stores a word, a byte, and a longword into consecutive global variables, then adds the value of a register to another global variable, based on a byte check.
- `update_global_values_2`: Stores a word, a byte, and a longword into consecutive global variables, then adds the value of a register to another global variable.
- `update_conditional_value`: checks a byte at a pointer offset and either adds a bit-shifted value or the original bit-shifted value to a global variable, then calls a routine to reduce a large value or reset an accumulator-related variable.
- `copy_table_to_struct_repeated`: It copies a sequence of values from a source table, offset by a shift-adjusted index, into fixed offsets of a destination structure, then repeats the same copy several more times.
- `copy_table_to_struct`: Copies a sequence of values from a source table, offset by a shift-adjusted index, into fixed offsets of a destination structure, performing the transfers one word at a time.
- `merge_entity_flags`: Clears a block of per-entity tables to zero, then combines the current entity's flag and data words into shared tables using bitwise AND. It processes a fixed set of bitfield slots per entity and repeats this for a limited number of entities.
- `clear_register_d0_2`: Performs a logical OR operation on a register with zero.
- `clear_global_memory_2`: This routine clears a specific block of memory by writing zero values to a global address.
- `reset_accumulator_variable`: Sets a global variable to zero.
- `reduce_large_value_and_call`: Reads a large unsigned value from a global variable, reduces it by repeatedly subtracting one million and then one hundred thousand, then invokes another routine several times in sequence.
- `accumulate_and_count_values`: Adds pending amounts into two separate accumulators, clamping each to a ten-million ceiling, then counts down large and small thresholds in each branch before returning.
- `reduce_value_and_call_routine`: It reduces a large unsigned value by repeatedly subtracting one million and then one hundred thousand, then calls a routine several times.
- `copy_block_and_return_registers`: It copies a block of words from a source table into two destination tables, then returns two incoming register values unchanged.
- `adjust_counter_and_call_twice`: Adds a constant to a global counter when it exceeds a threshold, then calls another routine twice before returning the incoming register values.
- `clamp_and_increment_byte`: Adds a constant to a byte value, clamping the result when it exceeds a threshold, and handles both a byte referenced through a register and a global byte.
- `adjust_pointed_byte_and_call`: Adds a constant to a byte pointed to by a register when it exceeds a threshold, then calls another routine before returning.
- `guarded_counter_adjust`: Runs two guarded blocks that each add a constant to a global counter when it exceeds a threshold and call another routine, with the guards never allowing the body to execute.
- `update_and_copy_table`: Increments a global variable by a constant if it exceeds a threshold and then calls another routine to copy table entries twice.
- `copy_block_twice`: It copies a block of words from a source table into two destination tables, then calls the same copy routine once more before returning.
- `copy_table_block_to_two_destinations`: Copies a block of words from a source table into two destination tables at a set of fixed offsets, using a register computed from a shift and add.
- `copy_word_block_adjusting_base`: Copies a block of sixteen words from a source pointer into a destination table, adjusting the destination base when the input register equals forty-two.
- `copy_word_block_from_source_pointer`: Copies a block of words from a source pointer into a destination table whose base depends on the input register, decrementing the source and destination pointers each iteration.
- `map_string_to_codes`: Walks a character string and writes mapped 16-bit codes into a destination buffer, converting spaces and letters into different ranges and starting a new line when it hits a carriage return.
- `transform_string_in_place_2`: Transforms a string in place by rewriting each character based on its value, mapping spaces to a fixed byte and stopping when it reaches a null terminator.
- `transform_string_in_place`: It walks a string, writing transformed values into a destination buffer, handling spaces and digit ranges, and returns when it reaches the end of the input.
- `convert_string_to_codes`: Copies a NUL-terminated string into a table of 16-bit values, mapping spaces and letters to different numeric codes and stopping at a carriage return.
- `set_global_and_return`: Writes a constant to a global variable twice and then returns, performing no other work.
- `copy_word_block_conditional_source`: Selects one of two source table bases depending on whether a global variable is nonzero, then copies a block of sixteen words from that source into a destination table.
- `fill_memory_block`: Fills a series of memory locations starting from an offset in a global variable with a specific constant value using a loop, and then updates a global variable with a constant value.
- `fill_memory_block_2`: Repeatedly writes a specific word value into a memory range using a loop.
- `normalize_counter_table`: Walks a table of 24-word records while clamping a large counter to its valid range and a smaller counter to its own range, then advances through successive entries until the table is fully traversed.
- `process_records_with_counter`: Loops over a table of records, reducing a large counter by one million and one hundred thousand before each iteration, then calls a series of routines for every entry.
- `apply_bitwise_complement_loop_2`: Walks a source byte table and combines the bitwise complement and original value of each byte into four separate destination buffers based on the bits of a control value.
- `apply_bitwise_complement_loop`: Loops over a source byte table, computing the bitwise complement of each byte and applying it to a destination table using conditional AND and OR operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_complex_initialization`: Performs a sequence of operations including clearing registers, updating global states, and copying words from a source pointer to a destination.
- `clear_d0_register_7`: clears the lower byte of a data register to zero.
- `evaluate_global_state`: Tests a global variable against zero and specific constants to set a global variable to either zero or a specific bit pattern.
- `modify_global_bits`: Calls another routine twice and conditionally performs bitwise AND or OR operations on a global variable based on the result.
- `reset_global_state`: Checks a global variable against a constant and, if it does not match, performs a sequence of assignments and calls to other routines.
- `copy_memory_words`: Copies fifteen words from a source pointer to a destination address using a loop.
