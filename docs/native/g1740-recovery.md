# Game 1740 (xybots (1989)(domark)[a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01740). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (951120 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 88 raw-word lines |
| Functions named and described | pass, 269 of 269 named (0 author symbols), 269 of 269 described (27 by rule, 241 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 5037 of 5037 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 12288 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8015 of 8015 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 19735 |
| code | 30664 |
| data | 863306 |
| padding | 39 |
| unused | 37376 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Xybots (1989)(Domark)[a2].adf:bootblock | 1 | 8 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3385 | 84 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 819 | 32 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 687 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2960 | 132 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 156 | 6 |
| Xybots (1989)(Domark)[a2].adf bytes 56320-68608 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- xybots (1989)(domark)[a2]/Xybots (1989)(Domark)[a2].adf: custom-format (non-DOS) disk, 1685 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 2095 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 649 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 603 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1601 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 89 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- xybots (1989)(domark)[a2]/Xybots (1989)(Domark)[a2].adf: bytes 56320-68608 look like code no run loaded: recovered position-independently (load address unknown)
- xybots (1989)(domark)[a2]/Xybots (1989)(Domark)[a2].adf: 1 logged loads cover 12288 bytes of the image
- xybots (1989)(domark)[a2]/Xybots (1989)(Domark)[a2].adf: 13 track-loaded blocks found in recovered runtime segments and classified through them
- xybots (1989)(domark)[a2]/Xybots (1989)(Domark)[a2].adf: blocks 2-26 (bytes 1024-13824) outside the filesystem hold data, not code (opcode density 0.006, entropy 5.80)
- xybots (1989)(domark)[a2]/Xybots (1989)(Domark)[a2].adf: blocks 28-101 (bytes 14336-52224) outside the filesystem hold data, not code (opcode density 0.024, entropy 6.48)
- xybots (1989)(domark)[a2]/Xybots (1989)(Domark)[a2].adf: blocks 141-141 (bytes 72192-72704) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.30)
- xybots (1989)(domark)[a2]/Xybots (1989)(Domark)[a2].adf: blocks 146-219 (bytes 74752-112640) outside the filesystem hold data, not code (opcode density 0.013, entropy 5.20)
- xybots (1989)(domark)[a2]/Xybots (1989)(Domark)[a2].adf: blocks 222-222 (bytes 113664-114176) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.85)
- xybots (1989)(domark)[a2]/Xybots (1989)(Domark)[a2].adf: blocks 224-278 (bytes 114688-142848) outside the filesystem hold data, not code (opcode density 0.044, entropy 5.22)
- xybots (1989)(domark)[a2]/Xybots (1989)(Domark)[a2].adf: blocks 282-287 (bytes 144384-147456) outside the filesystem hold data, not code (opcode density 0.043, entropy 5.79)
- xybots (1989)(domark)[a2]/Xybots (1989)(Domark)[a2].adf: blocks 289-304 (bytes 147968-156160) outside the filesystem hold data, not code (opcode density 0.042, entropy 5.14)
- xybots (1989)(domark)[a2]/Xybots (1989)(Domark)[a2].adf: blocks 306-343 (bytes 156672-176128) outside the filesystem hold data, not code (opcode density 0.015, entropy 3.27)
- xybots (1989)(domark)[a2]/Xybots (1989)(Domark)[a2].adf: blocks 348-348 (bytes 178176-178688) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- xybots (1989)(domark)[a2]/Xybots (1989)(Domark)[a2].adf: blocks 355-415 (bytes 181760-212992) outside the filesystem hold data, not code (opcode density 0.005, entropy 3.68)
- xybots (1989)(domark)[a2]/Xybots (1989)(Domark)[a2].adf: blocks 419-610 (bytes 214528-312832) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.01)
- xybots (1989)(domark)[a2]/Xybots (1989)(Domark)[a2].adf: blocks 615-676 (bytes 314880-346624) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.77)
- xybots (1989)(domark)[a2]/Xybots (1989)(Domark)[a2].adf: blocks 683-742 (bytes 349696-380416) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.00)
- xybots (1989)(domark)[a2]/Xybots (1989)(Domark)[a2].adf: blocks 747-781 (bytes 382464-400384) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.18)
- xybots (1989)(domark)[a2]/Xybots (1989)(Domark)[a2].adf: blocks 813-1759 (bytes 416256-901120) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.78)
- 1 never-loaded region(s), 12288 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Xybots (1989)(Domark)[a2].adf:bootblock

- `store_byte_into_global`: A single-instruction routine that loads the address of a global variable into a register and stores a byte from a register through that pointer.
- `store_byte_global`: Loads the address of a global variable into a register and stores a byte from a register through that pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_sprite_mask_to_bitmaps`: Iterates through a list of sprite data and applies a bitwise mask to corresponding bitmapped graphics memory.
- `render_bitmapped_sprite_layer`: Calculates a memory offset based on coordinates and performs a bitwise logical combination of source pixel data with a destination buffer.
- `update_sprite_graphics`: Calculates and writes bitmask data to a graphics buffer by performing bitwise operations on source pixel data.
- `update_sprite_bitmaps`: Calculates memory offsets to clear or modify specific bitplanes within a sprite or object buffer.
- `update_sprite_mask_table`: Calculates and applies bitwise mask updates to a sprite or object attribute table.
- `update_sprite_mask_data`: Calculates bitwise mask values from a source data structure and applies them to a destination table using bitwise AND operations.
- `update_sprite_bitplanes`: Calculates and writes bitplane data for a set of sprites by performing bitwise operations on source pixel data.
- `update_bitmapped_graphics`: Performs bitwise logical operations to blend or mask pixel data from a source buffer into a destination buffer using bit-shifting and rotation.
- `increment_structure_field`: Retrieves pointers to a global variable and increments a value at a specific offset within that structure.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `call_subroutines_with_offset`: Calls two different routines, the first one using an argument shifted left by three bits.
- `blit_bitplane_data`: Copies bitplane data from a source table to a destination buffer using bitwise operations to handle non-word-aligned shifts.
- `apply_bitmask_to_memory_blocks`: Calculates a bitmask by performing bitwise operations on a sequence of words from a table.
- `process_table_bits`: Iterates through two tables, clearing a specific bit in the first table and calling other routines based on the state of bits in both tables.
- `apply_bitmask_to_buffers_2`: Checks a global variable against a range and then iterates through a loop to perform bitwise OR operations on two separate memory regions using a bitmask.
- `tiny_move_andi`: Small routine of 2 instructions that copies (a4) to d5, then masks d5 with 15.
- `apply_bitmask_to_buffers`: Reads a sequence of bytes from a source buffer and performs a bitwise OR operation to apply them to two separate destination memory regions.
- `modify_bitmask_table`: Iterates through a table and performs bitwise OR or bitwise AND operations on specific bytes based on the value of a global variable.
- `clear_memory_block_2`: Fills a block of memory starting at a fixed address with zeros using a loop.
- `clear_memory_blocks`: Iterates through two memory regions in fixed increments and calls a routine to write values to specific offsets within those blocks.
- `write_multiple_values`: Writes a 32-bit value from a register to several specific offsets within two different memory blocks.
- `overwrite_memory_with_bits`: Clears the high bit of specific bytes in a table and then stores bytes from a source pointer into those same locations.
- `apply_bitwise_or_to_table`: Performs a bitwise OR operation between bytes from a source pointer and specific entries in a destination table.
- `clear_table_entries`: Sets a series of specific byte locations within a table to zero.
- `apply_amiga_hunk_relocations`: Iterates through a sequence of data structures resembling AmigaDOS hunk headers to apply relocation offsets.
- `apply_relocation_hunks`: Iterates through a sequence of bytes and applies them to specific memory locations by masking the target byte with a bitmask and performing a bitwise OR operation.
- `process_entity_event_loop`: Iterates through a table of data structures based on a global count.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `apply_bitmask_to_memory`: Performs a bitwise OR operation between a byte and several specific offsets within a memory block pointed to by a register.
- `apply_bitmask_subset`: Performs a bitwise OR operation between a byte and four specific offsets within a memory block.
- `apply_bitmask_subset_alt`: Performs a bitwise OR operation between a byte and four different offsets within a memory block.
- `test_hunk_types`: Tests two consecutive longwords in memory and branches to another routine if either test is non-zero.
- `apply_bitmask_extended`: Performs a bitwise OR operation between a byte and six specific offsets within a memory block.
- `apply_bitmask_partial`: Performs a bitwise OR operation between a byte and six specific offsets within a memory block, including the base address.
- `calculate_distance_on_grid`: Calculates the Manhattan distance between two coordinates stored in a structure and checks for collisions or path availability within a bitmask table.
- `return_stack_value_2`: Retrieves a value from the stack and returns it to the caller.
- `return_stack_parameter`: Retrieves a value previously placed on the stack and returns it to the caller.
- `return_stacked_argument`: Takes no parameters of its own and returns the 32-bit value read from the top of the stack at entry, which the decompiler treats as an undefined input.
- `return_stack_value`: The routine retrieves a value from the stack and returns it to the caller.
- `execute_indirect_jump`: Calculates an address using a value from a pointer and an offset, then performs an indirect call to that address.
- `execute_table_jump`: Performs an indirect call to an address calculated from a global table using a signed word as an index.
- `check_bit_status_2`: Calculates an offset using a register, masks it, and checks the status of a specific bit in a memory location pointed to by a base register.
- `check_bit_status_3`: Increments a register, masks and shifts it to calculate an offset, then checks a specific bit in a memory location.
- `check_bit_status_4`: Increments a register, masks and shifts another register to calculate an offset, then checks a specific bit in a memory location.
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
- `patch_relocation_hunks`: Parses a data structure to identify specific hunk types and applies word-sized values to a series of fixed offsets relative to a base pointer.
- `apply_relocation_patch`: Iterates through a memory range to apply specific word-sized values to a series of fixed offsets relative to a base pointer.
- `apply_relocation_patches`: Performs complex conditional checks and updates various global variables and memory locations, including calls to patch relocation hunks.
- `decrement_and_trap_on_zero`: Decrements a global variable and enters an infinite loop if the resulting value reaches zero.
- `update_global_counters`: Increments four global variables and decrements one before overwriting a global variable with two different constant values.
- `poll_routine_loop`: Calls another routine and enters a loop that repeatedly calls that same routine until it returns a true value.
- `update_offset_table`: Iterates through a table of 64 values, adding a constant to each and wrapping the value if it reaches a specific threshold.
- `populate_offset_tables`: Stores a 16-bit value and several derived values incremented by fixed offsets into multiple global locations.
- `store_mouse_data`: Stores a byte from a register into a global variable.
- `update_mouse_position`: Compares the value of a hardware register against a global word and performs arithmetic to update that word.
- `modify_global_word`: Adds a byte to a global word, performs bitwise rotations on two registers, and compares them before potentially subtracting them and branching.
- `increment_global_byte`: Adds a byte from a register to a global variable.
- `compare_byte_values`: Compares a byte in a register against four different memory locations and branches if any match is found.
- `combine_input_results`: Calls two other routines and returns the bitwise OR of their results.
- `calculate_input_state`: Performs bitwise operations on a global variable and returns a combined 32-bit value.
- `combine_input_flags`: Takes a value from another routine and performs bitwise OR operations on it based on specific bits set in a global variable.
- `poll_joystick_status`: Reads a byte from a hardware register and branches to another routine based on whether the value is negative.
- `execute_conditional_polls`: Calls another routine multiple times and returns a byte that is conditionally modified by a bitwise OR operation.
- `scan_sprite_attribute_table`: Iterates through a table of data structures to check a specific field within each entry.
- `empty_stub_12`: Empty routine: returns to the caller immediately without doing anything.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `tiny_not_not_rts`: Small routine of 3 instructions that inverts d0, then inverts d1, then returns.
- `invert_byte`: Returns the bitwise NOT of a byte from a register.
- `calculate_interpolated_entity_state`: Performs fixed-point coordinate interpolation and scaling based on a timer value and a lookup table.
- `call_indirect_via_table`: Calculates an address using a global variable and a table, then calls the routine at that address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_indirect_function_call`: Calculates an address by indexing into a table using a global variable, applies an offset, and performs an indirect jump to the resulting location.
- `process_byte_offset`: Subtracts three from a byte located at an offset from a pointer and masks the result.
- `process_byte_offset_2`: Adds four to a byte located at an offset from a pointer and masks the result.
- `subtract_and_mask_byte`: Subtracts two from a byte located at an offset from a pointer and returns the result masked with fifteen.
- `add_and_mask_byte`: Adds two to a byte located at an offset from a pointer and returns the result masked with fifteen.
- `clear_memory_block_3`: Clears a block of memory by writing zero to sixty-four consecutive double-word locations starting from a fixed address.
- `process_nested_table_data`: Iterates through nested loops to perform bitwise checks on memory based on values retrieved from a table and a pointer.
- `copy_structured_data`: Performs a multi-level block copy by reading from a source table and writing to a destination calculated from a pointer and an offset.
- `copy_table_data`: Copies sixteen words from a fixed source table to a destination address calculated from a global variable and another offset.
- `clear_memory_block_4`: Iterates through a specific memory range, writing zero to each four-byte word.
- `update_global_tables`: Or-accumulates values from a source table into two different destination tables and conditionally copies data from a third source.
- `apply_offset_to_table`: Calculates a destination address using a global variable and a table value, then adds values from a source to a destination table using a bitwise mask.
- `update_pixel_data`: Combines bytes from two sources, calculates a value, and performs bitwise OR operations on two specific memory locations.
- `compare_and_update_data`: Iterates through a table of data to compare byte pairs against values derived from a global variable and a pointer.
- `search_and_assign_index`: Compares two bytes from a source against a target pair in a loop and stores a descending counter value into a destination byte if a match is found.
- `calculate_memory_offsets`: Performs arithmetic on register values to derive two different offsets from a base address.
- `copy_data_block`: Copies several bytes from one memory location to another at specific offsets.
- `recursively_reset_global_flag`: Checks a global variable and, if non-zero, resets it to zero and calls itself recursively.
- `fill_memory_loop`: Fills a series of memory locations with the value one hundred twenty-seven using a loop.
- `decode_and_execute_bytecode`: Parses a nested stream of encoded bytes to perform bitwise transformations and indirect function calls.
- `update_pointer_value`: Checks a global variable and, if non-zero, performs several assignments before storing a byte and an incremented pointer into a global variable.
- `set_global_constants`: Stores four specific constant values into a global variable.
- `set_global_constants_alt`: Stores four specific constant values into a global variable in a different order than another routine.
- `empty_stub_13`: Empty routine: returns to the caller immediately without doing anything.
- `populate_data_structures`: Iterates through a memory buffer to write a series of structured byte sequences and control characters.
- `initialize_object_structure`: Clears various bits in a byte, sets several fields in a structure pointed to by a global variable, and increments the pointer.
- `initialize_object_structure_alt`: Clears multiple bits in a byte and populates a structure with specific values before incrementing the pointer.
- `initialize_object_structure_v3`: Clears bits in a byte and sets specific fields in a structure pointed to by a global variable.
- `initialize_object_structure_v4`: Clears bits in a byte and sets specific fields in a structure pointed to by a global variable with different field values.
- `initialize_object_structure_v5`: Clears bits in a byte and sets specific fields in a structure pointed to by a global variable with a different offset value.
- `initialize_data_structure`: Clears several fields in a data structure and sets specific byte and word values at fixed offsets.
- `initialize_data_structure_alt`: Resets a data structure by writing specific constants to various offsets and updating a global pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `perform_logical_or`: Performs a bitwise logical OR operation on a data register with zero.
- `update_and_jump_indirect`: Increments a byte at an offset from a pointer, clears several global variables, and performs an indirect jump to an address calculated from a global variable and a register.
- `validate_and_sort_attribute_table`: Checks bitmask flags within a table of data structures to determine a valid index.
- `update_sprite_table_and_execute`: Calculates a memory offset based on input parameters to check status flags in a table, then performs a bubble sort on a list of entries before executing a function pointer from the sorted table.
- `validate_and_execute_table_entry`: Validates a set of coordinate or index values against a bitmask table and performs a bubble sort on a list of entries before executing a function pointer from the sorted table.
- `sort_and_execute_table_entries`: Performs a bubble sort on a table of values and subsequently executes a function pointer retrieved from a calculated offset within that table.
- `sort_table_elements`: Performs a bubble sort on a table of values until the list is ordered.
- `process_and_sort_entity_table`: Updates specific fields in a data structure based on arithmetic comparisons and then performs a bubble sort on a global table of values.
- `sort_and_update_entity_table`: Performs a bubble sort on a table of values and updates specific fields within the table entries based on calculated offsets and comparisons.
- `update_entity_state_and_sort`: Updates specific data fields within a structure based on calculated offsets and then performs a bubble sort on a global table of values.
- `process_and_sort_entity_data`: Iterates through a list of data structures to perform conditional updates and field assignments based on coordinate or attribute comparisons.
- `copy_relocation_data_table`: Iterates through a list of data structures and performs bulk byte-wise transfers from a source table to a destination buffer.
- `copy_memory_block`: Copies a block of data from one memory location to two different destination locations using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_call_indexed`: Increments a global byte and masks the result to one, then runs a loop that walks a table in sixteen-byte steps and calls a routine selected by an indexed value from each entry.
- `do_nothing_return`: This routine is empty and simply returns without performing any operation.
- `gate_timer_and_reset_counters`: Implements a gated timer that runs a state-machine routine only while a global counter is below a threshold, then checks that counter against a limit to decide whether to reset the counters and clear related fields.
- `run_state_machine_once`: Initializes a global variable to a sentinel value and then runs a shared state-machine routine that branches on that variable to update a structure and a global counter.
- `apply_bonus_conditions`: Increments a referenced byte, then branches on a global variable to add a fixed amount to a pointed-to value and set other structure fields before clearing a field and calling another routine.
- `check_state_and_update_struct`: Inspects a global variable and bytes at fixed offsets in a referenced structure, updating the structure and a global counter when specific conditions match, then clears a field and calls another routine.
- `reset_counters_when_threshold_exceeded`: Clears a set of global counters and a structure field when a counter exceeds a threshold, decrements a global variable, and then calls another routine.
- `call_until_zero`: Sets a global byte to a value, then repeatedly calls another routine until that routine stores zero into the global byte.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `toggle_bit_in_struct`: It toggles a single bit within a byte at a fixed offset in a global structure using an index to select the bit position, then returns the original byte value.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `store_byte_via_a1`: It copies the byte currently in register D0 to the memory location pointed to by A1 and then returns.
- `return_d0_byte`: Passes the byte currently in data register D0 straight through as its return value without modifying it.
- `tiny_ori_rts`: Small routine of 2 instructions that merges 0 into d0, then returns.
- `call_single_routine`: Invokes a single routine located at a fixed address and then returns.
- `store_bytes_with_offset`: It computes a target address from a base register plus an offset and stores three consecutive bytes from a source buffer into a short array there, adding a constant to each value.
- `store_const_short`: It writes a single constant short value into a short field at a fixed offset within a structure addressed by a base register, then returns.
- `store_const_short_other`: It writes a different constant short value into a short field at a fixed offset within a structure addressed by a base register, then returns.
- `dispatch_and_update_timer`: Loads a routine address from a jump table and dispatches to it based on an index, then runs a gated timer that executes a state machine only while a counter is below a threshold.
- `update_counters_and_reset`: Increments two counters and, upon reaching specific thresholds, resets a global variable and a timer function pointer.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `flip_and_increment_flags`: It flips a flag bit in a byte at a fixed offset, then increments a following byte and masks it, clearing a byte further along when the incremented value wraps to zero.
- `increment_and_set_values`: Increments a byte at an offset and, if a specific bitwise condition is met, writes a word and a long value to the structure.
- `advance_state_selector`: Decrements a counter stored at a fixed offset and, when it reaches zero, wraps it back into a small range.
- `decrement_with_table_lookup`: Subtracts a byte from a value at an offset and, if a borrow occurs, decrements a counter and performs a jump to another routine.
- `increment_with_table_lookup_2`: Adds a byte to a value at an offset and, if a carry occurs, increments a counter and performs a jump to another routine.
- `decrement_with_offset_lookup`: Subtracts a byte from a value at a specific offset and, if a borrow occurs, decrements a counter and performs a jump to another routine.
- `increment_with_table_lookup`: Adds a byte to a value and, if a carry occurs, increments a counter and searches a table to set two other bytes.
- `update_entity_timer_and_dispatch`: Updates a per-entity timer and state flags, wrapping a two-bit counter and adjusting related counters, then branches to a handler selected from a jump table based on a global index.
- `decrement_and_call_routine_2`: Subtracts a byte from a value, handles a borrow by decrementing a counter and calling another routine, and then clears a bit in a structure.
- `increment_and_call_routine`: Adds a byte to a value, handles a carry by incrementing a counter and calling another routine, and then clears a bit in a structure.
- `decrement_offset_and_call_routine`: Subtracts a byte from a value at a specific offset, handles a borrow by decrementing a counter and calling another routine, and then clears a bit in a structure.
- `update_entity_state`: Increments a value and, if a carry occurs, increments a counter and calls another routine; it also decrements a timer and copies data to a global table.
- `increment_and_mask_byte`: Increments a byte stored at a fixed offset from a register, masks it to a two-bit value, and branches back to itself while that value equals zero.
- `store_constant_word`: Stores the word value eight into a memory location at a fixed offset from a pointer.
- `update_indexed_data_structure`: Decrements values within a structure and performs recursive updates or bitwise manipulations on a global variable based on table lookups.
- `decrement_with_carry_check`: Subtracts a byte from another and, if a borrow occurs, decrements a counter and calls another routine.
- `increment_byte_with_carry`: Adds a byte to a value at a pointer and increments a nibble-masked byte at an offset if a carry occurs.
- `increment_byte_offset_one`: Adds a byte to a value at a pointer plus one and increments a nibble-masked byte at an offset of seven if a carry occurs.
- `decrement_byte_with_borrow`: Subtracts a byte from a value at a pointer and decrements a nibble-masked byte at an offset if a borrow occurs.
- `call_then_increment_flags`: It calls another routine, then flips a flag bit in a byte at a fixed offset and increments a following byte, clearing that byte when it reaches a specific value.
- `test_and_store_bytes`: Tests a byte at an offset from a pointer and branches if it is zero.
- `update_state_machine_node`: Updates a state machine by toggling a byte at an offset from a pointer and performing conditional branches based on its value.
- `compare_and_decrement`: Compares a byte at an offset from a pointer against several constants and branches accordingly, or decrements a word through a pointer.
- `tiny_addq_rts`: Small routine of 2 instructions that adds 1 to (a5), then returns.
- `update_nibbles_and_counter`: Increments a counter and a byte in a structure, then splits a global byte into two nibbles and stores them at specific offsets from a pointer.
- `split_byte_into_nibbles`: Splits a global byte into two nibbles and stores them at specific offsets from a pointer, then sets the global variable to a fixed value.
- `empty_stub_14`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_structure_data`: Decrements values in a structure, clears specific bytes, and if a condition is met, populates a structure with constants and clears a global word.
- `initialize_data_structure_2`: Stores a constant, a zero word, and a constant byte into a structure at specific offsets from a pointer, then clears a global variable.
- `empty_stub_15`: Empty routine: returns to the caller immediately without doing anything.
- `modify_global_bitfield`: Checks a value in a structure and either modifies a bit in a global variable using a calculated offset or increments a value in that structure.
- `update_entity_collision_state_2`: Evaluates entity status flags and collision bitmasks to update internal counters and modify global bitfields.
- `process_entity_collision_events`: Evaluates collision states and bitmasks to trigger specific entity behaviors, such as updating internal counters, modifying global bitfields, or executing indirect function calls from a table.
- `update_entity_collision_state`: Checks a bitmask against a table of indices to detect collisions or state changes.
- `initialize_structure_fields`: Stores a longword, a word, and a byte into a data structure pointed to by a register.
- `update_conditional_data`: Checks if a pointer matches its own address and then updates several global variables with specific values based on a global flag.
- `initialize_data_structure_4`: Sets specific word and longword values in a structure based on a global flag and then copies the routine's own address into that structure.
- `check_address_offset`: Calculates the difference between the current routine address and a value from a pointer, then updates a structure with new values if the absolute difference is two.
- `clear_data_d0`: Performs a logical OR operation on a data register with zero.
- `clear_d0_register`: Performs a logical OR operation on a data register with zero to clear its contents.
- `clear_data_byte`: performs a logical OR operation on a byte in a data register with zero.
- `clear_data_byte_2`: Performs a logical OR operation on a byte in a data register with zero.
- `clear_data_byte_3`: Performs a logical OR operation on a data register with zero.
- `update_offset_value`: Checks a value at an offset from a pointer and, if certain conditions are met, increments a byte at a specific offset and performs a calculation involving another memory location.
- `clear_data_byte_4`: Performs a logical OR operation on a data register with zero.
- `execute_indirect_call`: Checks a bitmask in a global variable and a byte at an offset before calling a function pointer from a table.
- `iterate_function_table`: Iterates through a list of values, checking a bitmask in a global variable and calling functions from a table based on those values.
- `return_from_subroutine_2`: This routine performs a single return instruction to exit the current function.
- `return_immediately`: This routine performs no operations and returns to its caller immediately.
- `nop_sequence`: This routine is empty and performs no operations.
- `empty_stub_16`: Empty routine: returns to the caller immediately without doing anything.
- `increment_and_call`: Increments a byte in a structure and stores a value into another structure using an offset derived from a register.
- `return_nothing`: The routine is empty and performs no operations before returning.
- `process_indexed_table`: Performs complex logic involving a global variable and a table to determine a value, then calls a routine via an indexed function pointer.
- `update_indexed_data`: Modifies a data structure by performing bitwise XOR operations and updating various fields with values retrieved from other memory locations.
- `empty_stub_17`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_18`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_19`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_20`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_21`: Empty routine: returns to the caller immediately without doing anything.
- `update_entity_state_and_dispatch`: Decrements values within a data structure and performs conditional resets of structure members based on threshold checks.
- `clear_global_word_array`: It sets and clears a global short variable, zeroes a four-byte value, then loops to clear a run of consecutive four-byte words starting at the global variable.
- `find_and_call_indexed`: Walks a table of words until it finds a zero entry, records the index, increments a counter, marks the found entry, and then calls a routine selected by an incoming value.
- `search_and_call_indexed`: Iterates through a global word array to find a zero entry, updates a structure with a count and a pointer, and then calls a function from a table using an index.
- `update_structure_count`: Calculates a value by subtracting a difference between two table offsets from a byte in a source structure, then stores the result and updates a counter in a destination structure.
- `increment_structure_count`: Calculates a value by adding the difference between two table offsets to a byte in a source structure, then stores the result and updates a counter in a destination structure.
- `update_structure_offset`: Calculates a value by subtracting a difference between two table offsets from a byte in a source structure, then stores the result into a different offset within a destination structure.
- `copy_and_format_data`: Copies bytes between two structures, performs arithmetic on a byte, and populates a structure with several constant byte values.
- `initialize_data_structure_3`: Stores a longword, a word, and a byte into a structure pointed to by a register before calling another routine.
- `decrement_indexed_and_store`: Clears a word located via an offset from a register and decrements a value indexed by a byte from that same register.
- `modify_structure_fields`: Clears a word in a structure using an offset from a pointer, decrements a global variable, and stores a byte into the structure.
- `scatter_table_words`: Copies a run of words from a source table into a destination table, scattering the values across a set of fixed offsets and advancing the source by a larger stride than the destination each iteration.
- `clear_memory_block`: Clears a contiguous block of global variables and RAM locations by writing zero to each of them in sequence.
- `decrement_global_twice`: Decrements a global variable by two within each of two conditional blocks, provided the variable is not already zero.
- `decrement_global_thrice`: Decrements a global variable by three within each of two conditional blocks, provided the variable is not already zero.
- `call_table_function`: Calls a function pointer retrieved from a table using an offset stored in a structure.
- `call_indexed_function`: Checks a byte at an offset from a pointer and, if the value is less than two, calls a function from a table using an index derived from another pointer.
- `update_global_from_byte`: Shifts a byte from a structure and adds a constant to it, then stores the result in a global variable.
- `store_processed_values`: Performs bitwise operations on a byte from a pointer and stores the result along with several other values into consecutive global variables.
- `update_global_values`: Stores a word, two bytes, and a longword into consecutive global variables, then adds the value of a register to another global variable.
- `copy_table_words_2`: Copies a sequence of words from a source table into fixed offsets of a destination structure, offset by a shifted register value.
- `copy_table_words`: Copies a sequence of words from a source table, offset by a shifted register value, into fixed offsets of a destination structure.
- `mask_object_flags`: Clears a fixed set of flags in a per-object table, then combines that table with several source tables using bitwise AND to mask or update the stored values.
- `reduce_value_by_powers_of_ten_3`: Subtracts successive powers of ten from a global variable until the value is less than the divisor.
- `reduce_value_by_powers_of_ten_4`: Subtracts powers of ten from a value stored in a register until the value is less than the specified threshold.
- `clear_register_d0`: Performs a logical OR operation on register D0 with zero.
- `reduce_value_by_powers_of_ten`: Repeatedly subtracts one million from a value until it fits, then subtracts one hundred thousand from the result until it fits, calling a helper routine between each reduction.
- `reduce_value_by_powers_of_ten_2`: Adds an incoming value to a stored total and clamps the result to a fixed maximum, then repeatedly subtracts decreasing powers of ten from a second total to reduce it below a threshold.
- `reduce_and_copy_table`: Reduces a large incoming value by repeatedly subtracting powers of ten, then copies a run of words from a source table into two destination tables.
- `copy_table_and_return`: Copies a run of words from a source table into two destination tables using an incoming word count to index that table, then returns two register values.
- `adjust_global_and_copy_table`: Adds a constant to a global variable while it exceeds a threshold, then calls a helper routine twice to copy table entries.
- `conditionally_increment_byte`: If the pointer is null it recurses into another routine; otherwise it adds a constant to the byte the pointer references when that byte exceeds a threshold, then recurses into another routine.
- `adjust_pointed_byte_and_copy`: Adds a constant to a byte pointed to by a register while it exceeds a threshold, then calls a helper routine to copy table entries.
- `repeated_conditional_update`: Performs a self-referencing comparison and assignment on a global variable, then conditionally adds a constant to it before invoking another routine.
- `update_and_copy_table`: Increments a global variable by a constant if it exceeds a threshold and then calls another routine to copy table entries twice.
- `copy_table_entries`: Copies a run of words from a source table into two destination tables at fixed offsets, using an incoming word count to index that table.
- `copy_hunk_table_entries`: Copies a run of words from a source table into two destination tables at a set of fixed offsets, using the incoming word count to index that table.
- `copy_word_blocks`: Copies a block of sixteen words from a source pointer into a destination table, adjusting the block size when the incoming value equals forty-two, then stepping through twelve such blocks.
- `copy_single_word_blocks`: Copies a single word from a source pointer into a destination table, then steps through twelve such blocks.
- `parse_string_to_table_2`: Translates each character of a string into a numeric value based on its code and stores it into a table of shorts, starting a new block when it encounters a carriage return.
- `parse_string_into_table`: Translates each character of a string into a numeric value, storing it into a table of shorts, and stores a fixed value into a global variable when it reaches the end.
- `parse_string_to_table`: Loops through a character string, translating each character into a numeric value and storing it into a table, stopping when it reaches a null terminator.
- `copy_string_to_words`: Copies a null-terminated string into a table of words, translating characters into lookup-table values and stopping at a carriage return.
- `set_global_and_return`: Writes a fixed value into a global variable twice and then returns to the caller.
- `copy_conditional_word_blocks`: Selects one of two source tables depending on whether a global variable is nonzero, then copies a block of sixteen words from that table into a destination table and steps through twelve such blocks.
- `fill_memory_block`: Fills a series of memory locations starting from an offset in a global variable with a specific constant value using a loop.
- `fill_memory_block_2`: Repeatedly writes a specific word value into a memory range using a loop.
- `convert_time_to_clock`: Walks a table of words, converting each value that is a time-of-day count stored as hundredths of a day into hours and minutes.
- `convert_table_values_to_clock`: Loops over a table of values, reducing each by subtracting powers of ten and calling a helper routine to convert them into clock values.
- `apply_byte_masks_2`: Loops over eight source bytes and applies each inverted mask to four destination locations through masked set and clear operations.
- `apply_byte_masks`: Loops over eight bytes, inverting each and applying it to four destination locations through masked set/clear operations controlled by the bits of a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_complex_loop`: Executes a large sequence of nested loops and conditional calls that manipulate various global variables and tables through multiple subroutines.
- `clear_data_byte_5`: performs a logical OR operation on a byte in a data register with zero, effectively leaving the value unchanged.
- `evaluate_global_state`: Tests a global variable against zero and specific constants to set a global variable to either zero or a specific bit pattern.
- `modify_global_bits`: Calls another routine twice and conditionally performs bitwise AND or OR operations on a global variable based on the result.
- `reset_global_state`: Checks a global variable against a constant and, if it does not match, performs a sequence of assignments and calls to other routines.
- `copy_memory_words`: Copies fifteen words from a source pointer to a destination address using a loop.
