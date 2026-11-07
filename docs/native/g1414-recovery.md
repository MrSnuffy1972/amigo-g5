# Game 1414 (soccer kid (1994)(krisalis software)(disk 1 of 4)[cr pdx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01414). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (926016 top-level bytes in 16 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 17 of 17 units identical (bytes and relocations), 59 raw-word lines |
| Functions named and described | pass, 187 of 187 named (0 author symbols), 187 of 187 described (2 by rule, 185 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1277 of 1278 (99.92%) |
| Recovered without load address (counted in proof 1, flagged) | 8704 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3447 of 3463 instructions identical in start and length, 2 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 12534 |
| data | 912837 |
| padding | 133 |
| unused | 512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Soccer Kid (1994)(Krisalis Software)(Disk 1 of 4)[cr PDX].adf:bootblock | 1 | 192 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 894 | 34 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 26 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 29 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 52 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 172 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1591 | 97 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 32 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 381 | 11 |
| Soccer Kid (1994)(Krisalis Software)(Disk 1 of 4)[cr PDX].adf bytes 511488-520192 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- soccer kid (1994)(krisalis software)(disk 1 of 4)[cr pdx]/Soccer Kid (1994)(Krisalis Software)(Disk 1 of 4)[cr PDX].adf: custom-format (non-DOS) disk, 1757 non-empty blocks: matched against track-loaded segments
- runtime segment an address (play-long-joy): 647 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 12 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 16 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 21 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 80 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 308 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 13 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 173 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- soccer kid (1994)(krisalis software)(disk 1 of 4)[cr pdx]/Soccer Kid (1994)(Krisalis Software)(Disk 1 of 4)[cr PDX].adf: bytes 511488-520192 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying code at an address (35 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (10 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (49 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (50 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (34 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (34 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (51 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (52 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (52 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- soccer kid (1994)(krisalis software)(disk 1 of 4)[cr pdx]/Soccer Kid (1994)(Krisalis Software)(Disk 1 of 4)[cr PDX].adf: 1 logged loads cover 8704 bytes of the image
- soccer kid (1994)(krisalis software)(disk 1 of 4)[cr pdx]/Soccer Kid (1994)(Krisalis Software)(Disk 1 of 4)[cr PDX].adf: 2 track-loaded blocks found in recovered runtime segments and classified through them
- soccer kid (1994)(krisalis software)(disk 1 of 4)[cr pdx]/Soccer Kid (1994)(Krisalis Software)(Disk 1 of 4)[cr PDX].adf: blocks 1017-1759 (bytes 520704-901120) outside the filesystem hold data, not code (opcode density 0.009, entropy 7.14)
- 1 never-loaded region(s), 8704 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 509440 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Soccer Kid (1994)(Krisalis Software)(Disk 1 of 4)[cr PDX].adf:bootblock

- `initialize_boot_sequence`: Performs memory copies, initializes a header or structure, configures hardware registers, and copies an instruction sequence to a destination address.
- `copy_instruction_sequence`: Sets a global variable to a specific address and then copies a sequence of instructions from a routine to a new memory location.
- `set_high_bit_after_calls`: Executes two subroutines and then repeatedly sets the high bit of a register until it becomes zero.
- `copy_memory_block`: Copies a block of memory from one location to another using a counter-based loop.
- `write_text_marker_by_flag`: Writes one of two four-character text markers, FAT0 or FAT1, chosen by one bit of a global value, and updates a memory location.
- `detect_refresh_rate_hz`: Resets the CIA port and timer registers, then reads a counter from two CIA bytes and returns fifty or sixty depending on whether it is below a threshold, while writing a text marker to a structure.
- `wait_until_status_byte_clear`: Repeatedly checks a status byte at a fixed offset from a base register and returns minus one once its top bit is clear.
- `initialize_boot_header`: Clears a block of memory, writes certain identification strings, and populates a structure with data or pointers based on a size check.
- `traverse_linked_list`: Traverses a linked list of pointers, checking a bitmask against a value in each node, until a certain condition or a null pointer is met.
- `traverse_linked_list_with_mask`: Iterates through a linked list while performing a bitwise check against a value stored in the current node.
- `pack_and_store_word`: Checks a global variable and, if zero, packs a byte with a constant and stores it into a memory location relative to a pointer.
- `store_packed_word`: Copies a word from one memory location and stores a packed value consisting of that word and a constant into another memory location.
- `initialize_function_table`: Stores a series of routine addresses into a sequence of global or memory-based pointers.
- `update_jump_target`: Checks a value at a memory offset and, if a certain pattern is matched, updates that memory location before performing an indirect jump.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_blt_table_and_setup_blitter`: Updates a table with calculated or obfuscated values, clears a memory range, and populates a secondary table with a sequence of incrementing or fixed values.
- `update_blt_parameters_from_table`: Iterates through a table, calculating a new address and updating several bitplane and blitter registers or a global variable based on the table contents.
- `rearrange_nibbles_in_buffer`: Iterates through a buffer and redistributes or reassembles nibbles from source words into a new destination buffer.
- `copy_memory_blocks`: Waits for a global variable to reach zero, then copies a series of memory blocks from a source to a destination.
- `copy_memory_block_5`: Copies a block of data from a source address to a destination address while clearing a global variable and waiting for a zero-check.
- `set_blt_parameters_fixed`: Sets several hardware registers and a global variable to fixed or calculated values.
- `setup_blt_registers`: Writes several constant values to specific memory-mapped or global registers and waits for a bit in a global variable to clear.
- `wait_for_function_result`: Repeatedly calls another routine until its return value is not less than one.
- `wait_for_condition`: Repeatedly calls another routine and pauses execution as long as the returned value is less than one.
- `wait_for_condition_2`: Repeatedly calls a routine and loops until the returned value is not less than one.
- `execute_conditional_logic`: Checks a global variable and calls other routines, potentially executing a third routine if a structure member or a function return value meets certain criteria.
- `check_range_and_call`: Checks if a value is within a specific range and calls another routine if it is.
- `process_bitmask_loop_2`: Checks a value in a structure and enters a loop that compares a value from a function against a structure member.
- `do_nothing_routine`: the routine is empty and performs no operations.
- `store_split_word`: Stores a 32-bit value or parts of it into a memory block using a pointer.
- `process_buffer_stream_2`: Iterates through a memory buffer looking for a specific sentinel value and then copies or transforms data into a destination buffer.
- `validate_index_range`: Checks if a signed integer or a value derived from a register falls within a specific range and verifies if a corresponding byte in a table is non-zero.
- `initialize_table_values`: Copies a value from one memory location to another and writes several constant values to a table or memory-mapped registers.
- `set_table_values`: Writes a constant value to a table offset and waits for a global flag or a loop counter to expire.
- `modify_device_register`: Modifies a byte at a specific offset using bitwise operations and a value from another function.
- `copy_variable_block`: Copies a block of memory using either a byte-by-byte or a word-by-word method depending on a size check, then updates a counter.
- `copy_memory_block_3`: Copies a block of bytes from a source pointer to a destination pointer using a counter, then clears a dword in a global structure.
- `process_buffer_stream`: Iterates through a memory buffer looking for a specific sentinel value and then copies or transforms data into a destination buffer.
- `calculate_masked_bitwise_result`: Returns a value calculated by masking and bit-shifting a pointer-referenced value.
- `execute_empty_sequence`: This routine is empty and performs no operations.
- `clear_register_bit`: Clears a bit in a memory location and then sets a different bit in the same location.
- `write_word_wait_for_flag`: Clears a certain flag, writes the low and high bytes of a word to two device registers, then spins until another routine sets the flag.
- `update_control_byte_bits`: Keeps the top three bits of one byte at a fixed offset from a base pointer while setting bit three, and writes the constant 130 into another byte at a fixed offset.
- `write_device_word`: Clears a flag, writes a word to two device registers, and waits for a flag change.
- `store_split_byte`: Stores a byte from a register into a table offset, splitting the byte into a low byte and a high byte stored at different offsets.
- `process_bitmask_loop`: Checks a value in a structure and enters a loop that compares a value from a function against a structure member.
- `modify_byte_with_mask`: Clears a bit at a certain position and sets certain bits in a byte located at an offset from a pointer.
- `modify_byte_with_mask_alt`: Clears a bit at a certain position and sets different bits in a byte located at an offset from a pointer.
- `update_bitmask_and_recurse`: Checks a condition in a table, modifies a bitmask in a global structure, and recursively calls itself while decrementing a counter if a specific bit in a pointer-referenced byte is not set.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_bit_in_memory`: Calculates a bitmask based on a register and uses it to clear a specific bit in a byte at a memory location offset from a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_bit_in_memory_2`: Clears a specific bit in a byte at a memory offset using a bitmask derived from a register.
- `update_memory_with_loop`: Checks a condition in a table, updates a byte in memory, and executes a loop that calls another routine while checking a bit in a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `toggle_bit_in_memory`: Performs a series of bitwise operations to clear and then set a specific bit in a byte at a memory offset, followed by a call to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_bit_and_loop`: Clears a bit in a memory offset, calls another routine, and then executes a loop that calls that same routine while checking a bit in a pointer.
- `reset_and_toggle_bit`: Resets a byte in a table to zero, then performs a sequence of bitwise operations to clear and then set a specific bit in a memory offset, followed by a call to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_bit_in_memory`: Performs a sequence of bitwise operations to clear and then set a specific bit in a byte at a memory offset, followed by a call to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_and_set_bit`: Modifies a byte at a memory offset using bitwise masks and then clears and sets a specific bit, followed by a call to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_bit_in_byte`: calculates a bitmask using a register and a bit-shift, then clears and sets a specific bit within a byte at a memory location pointed to by a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `toggle_bit_in_memory_2`: toggles a specific bit within a byte at a memory location pointed to by a register.
- `rotate_and_compare_bytes`: Rotates the upper bytes of two registers and compares them against a character, decrementing a counter until a certain condition or a specific character is met.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `increment_loop_counter`: Increments a local register in a loop until a certain value is reached.
- `update_joystick_bitmask`: Modifies a byte in a global variable by masking and setting specific bits, then updates a secondary memory location using a bit-shifted value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_input_update`: Enters an infinite loop that waits for a value from a function call to be at least certain value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `toggle_bit_and_update_memory`: Clears and then sets a bit in a byte at an offset from a pointer, while also updating other memory locations and calling a function.
- `reset_table_structure`: Performs conditional checks on a byte in a table and, if certain bits match, resets a word in a table and initializes a structure with a pointer and a constant.
- `clear_table_entry`: Writes a zero to a word in a table at an offset calculated from a register.
- `get_table_entry`: Returns a word from a table at an offset calculated from a register.
- `set_table_entry`: Writes a word from a register into a table at an offset calculated from a register.
- `increment_table_entry`: Increments a word in a table at an offset calculated from a register.
- `do_nothing`: the routine is empty and performs no operations before returning.
- `verify_paired_results`: Calls two routines and compares their results, repeating the comparison once more if they match, and returns a specific error value if any comparison fails.
- `xor_fold_longwords_masked`: XORs together a run of longwords read through a pointer, with the run length taken from a data register divided by four, then returns that fold ANDed with a data register mask.
- `initialize_table_structure`: Checks a byte in a table and, if certain bits match, resets a word in a table, initializes a structure with a pointer, and loops to clear or set certain bytes in a table or reset a pointer.
- `update_byte_with_mask`: Constructs a new byte by masking a value from a register and a byte from a table, then stores it back to a table offset.
- `set_control_bits_return_input`: Sets a fixed set of bits in a control byte at a fixed offset from a base pointer and returns its word input unchanged.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_routine`: The routine contains no executable instructions and performs no operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_byte_to_table`: Stores a byte from a register into a table at a fixed offset.
- `set_global_to_max`: Sets a global variable to a fixed maximum value.
- `set_global_to_zero`: Sets a global variable to zero.
- `write_longword_to_table`: Waits for a global variable to become zero, then writes a longword consisting of a fixed value and a register value into a table.
- `clamp_global_value`: Clamps a global variable or a copy of it within a specific range of allowed values.
- `update_button_state`: Reads several bits from a table and updates a global variable using bitwise OR operations based on those bits.
- `update_global_pointer_2`: Modifies a global pointer and associated counters or values based on bit-masking and conditional pointer dereferencing.
- `update_global_pointer`: Checks a value against a global pointer and either increments a counter or updates the pointer and resets a counter, then performs conditional pointer dereferencing.
- `update_pointer_and_get_value`: Updates a global pointer with a new address and performs a series of conditional pointer dereferences and arithmetic before returning a value.
- `clear_hardware_bits`: Sets a global variable to a specific value and clears certain bits in a hardware register and memory locations.
- `increment_and_mask_global_variable`: Increments a global variable and applies bitwise masking, conditional resets, and BCD-style adjustments.
- `update_blt_parameter_state_recursively`: Recursively modifies a global variable using bitwise masking, increments, and conditional resets, while performing BCD-style adjustments and triggering a trap or further routine calls based on the resulting state.
- `set_memory_constants`: Clears a bit in a global variable and writes a sequence of constant values to a series of memory locations.
- `clear_memory_range`: Sets a series of memory locations and a global variable to zero.
- `decrement_and_set_registers`: Decrements a global variable with a lower bound check and sets a bit in a memory location and several other memory addresses to specific values.
- `update_register_and_table`: Clears certain bits in a hardware register, clears several memory locations, and updates a global variable or loads data from a table based on a pointer offset.
- `return_immediately`: This routine is empty and returns to the caller immediately without performing any operations.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_audio_buffers`: Clears memory buffers and certain registers, then populates a table with incrementing values and calculates a maximum value from a memory range.
- `infinite_loop`: This routine executes an infinite loop.
- `clear_memory_buffers`: Clears several memory locations and certain registers to zero or a fixed value.
- `increment_global_variable`: Increments a global variable and, if it equals zero, executes a sequence that modifies a global variable using bitwise shifts and table-based conditional checks.
- `modify_global_state`: Performs bitwise shifts and masking on a global variable, updates audio registers based on a table search, and executes a loop that shifts a value while checking it against a table.
- `modify_global_variable`: Performs bitwise shifts and masking on a global variable and checks a value in a table.
- `update_audio_registers`: Updates certain audio or hardware registers based on a table search and a conditional check of a global variable.
- `update_entity_attribute`: Updates a pointer-based structure or table entry with new attribute or position data, calculating an offset or clamped index based on a lookup.
- `update_entity_attribute_table`: Updates a pointer-based structure or table entry with new attribute or position data, calculating an offset or clamped index based on a lookup table.
- `update_clamped_index_and_pointer_2`: Updates a pointer or index-based value by iterating through a table and applying a clamped or offset-based calculation.
- `update_clamped_index_and_pointer`: Iterates through a table to find a pointer or index, then updates a global variable or pointer-based value using a clamped or offset-based value.
- `update_clamped_index_value`: Updates a global variable or a table entry with a clamped or masked version of a value from a pointer, while optionally updating a secondary pointer or index with a clamped or masked value.
- `update_pointer_and_clamp_value_2`: Iterates through a table to find a pointer or index, then updates a global variable or pointer-based value using a clamped or offset-based calculation.
- `process_pointer_loop`: Performs conditional checks on a global variable and enters a loop that shifts a value and checks it against a table.
- `update_clamped_offset_and_table`: Updates a structure member or a global variable using a masked or clamped value, and then performs a reverse table search or pointer-based lookup.
- `copy_word_from_offset`: Copies a word from one memory location offset to another memory location offset.
- `update_table_entry_with_threshold_search`: Checks a global variable modulo or otherwise extracts a nibble from a byte, then searches a table backward for a value less than or equal to a threshold.
- `find_threshold_index`: Iterates backward through a table until a value is less than or equal to a threshold, then copies a word from a calculated offset.
- `copy_word_to_offset`: Copies a word from a memory location offset from a pointer to another memory location offset from a different pointer.
- `search_and_copy_word`: Iterates backward through a table until a value is found that is less than or equal to a certain threshold, then copies a word from a calculated index to a memory location.
- `copy_word_from_index`: Copies a word from a memory location calculated using a pointer and an index to another memory location.
- `store_register_to_memory`: Stores a word from a register into a memory location offset from a pointer.
- `decrement_value_with_limit`: Subtracts a masked byte from a value in a structure and clamps the lower bits of that value to a specific minimum if it falls below a threshold.
- `increment_value_with_limit`: Adds a masked byte to a value in a structure and clamps the lower bits of that value to a specific minimum if it exceeds a certain threshold.
- `search_and_update_table_value`: Iterates backward through a table or array or a certain offset to find a value that is less than or equal to a masked value from a pointer.
- `clear_struct_member`: clears a word-sized value at a specific offset within a structure or a global memory area.
- `update_and_search_table`: Updates a structure member, potentially resetting another, and then searches a table to find a value or updates a memory location.
- `update_structure_and_search`: Updates a value in a structure, potentially resetting a secondary member, and then searches a table to find a value or updates a memory location with a found value.
- `search_and_update_value`: Subtracts a value from a structure member, potentially resetting a secondary member to zero, and then searches through a table of values to find a new value that satisfies a condition.
- `update_clamped_offset_2`: Modifies a byte in a structure using bitwise operations and then calculates a scaled, sign-aware offset to store in a pointer-based destination.
- `calculate_scaled_offset_2`: Calculates a scaled value using a bitmask and a byte from a structure, then updates a memory location and a byte in a structure using a shift.
- `update_scaled_offset_with_sign`: Calculates a scaled offset using a bitmask and a sign-dependent bit-shift, then adds this value to a destination memory location.
- `scale_and_offset_value`: Calculates a scaled value using a bitmask and a structure byte, then updates a memory location and a structure member.
- `calculate_scaled_offset`: Calculates a scaled value using a bitmask and a byte from a structure, then updates a memory location and a byte in a structure using a shifted value.
- `update_score_and_flags`: Calculates a difference between two values and stores it in a global variable, while also updating a byte in a structure by adding a shifted nibble from a neighboring byte.
- `update_clamped_offset`: Updates a structure member by adding or subtracting a masked byte, clamping the result and updating a secondary offset.
- `update_clamped_velocity`: Updates a byte in a structure by adding or subtracting a scaled or masked value, clamps the result to a maximum, and stores the clamped difference relative to a global variable into a short at an offset.
- `update_scaled_offset`: Updates a byte in a structure by masking and oring bits, then calculates a scaled, clamped, and sign-aware offset to store in a pointer-based destination.
- `update_clamped_offset_with_scaled_delta`: Calculates a scaled and clamped delta based on a masked byte and a sign-aware value, then updates a global variable and a pointer-based value.
- `apply_scaled_clamped_delta`: Calculates a scaled, clamped, and sign-aware delta, then updates a structure member and a secondary value.
- `calculate_clamped_difference`: Calculates a value based on a masked byte and a pointer-based value, applies a sign, clamps it to a range, and stores the difference between this value and a global variable into a structure member.
- `clamp_and_update_difference`: Calculates a clamped difference between two values, then calculates a second difference relative to a global variable and stores it in a global variable.
- `update_clamped_value`: Modifies a structure member by adding or subtracting a masked byte, clamps the result to a maximum value, and stores the difference between this value and a global variable into a structure member.
- `update_clamped_difference`: Subtracts a masked byte from a structure member, clamps that member to zero if it becomes negative, and then calculates and stores a clamped difference relative to a global variable into a global variable.
- `update_pointer_and_clamp_value`: Updates a pointer or a global variable using a table lookup or an offset, while clamping a byte-sized value or a pointer-based index within a certain range.
- `update_joystick_state_and_timer`: Updates a global state or a pointer-based structure with values or bitmasks derived from a joystick input or a timer-related byte.
- `mask_low_nibble_to_offset`: The routine stores the low nibble of a byte from an address derived from an uninitialized pointer into a byte at a fixed offset from that pointer.
- `update_pointer_based_on_flag`: The routine checks a high bit in a structure field; if set, it clears the field, adjusts a pointer based on a table lookup and offset, and negates the byte at the new pointer location.
- `decrement_and_check_global`: Decrements a global variable and clears it if it drops below zero, then checks a bitmask in a global variable to clear a byte in memory or set a value.
- `test_bit_and_clear_memory`: checks a bitmask in a global variable against a bitset and, if a specific constant value is present in a global, clears a byte in a memory-mapped or external buffer.
- `initialize_linked_list_node_recursively`: A recursive routine that initializes a linked list node with a pointer and a decremented counter if certain memory or return value conditions are met.
- `initialize_linked_list_node`: Initializes a memory location with a pointer and sets the following word to a decremented value.
- `search_table_for_value`: Iterates through a table starting at a fixed address to find a matching value and returns a concatenated 64-bit result.
- `search_table_and_return_data`: Iterates through a table until a matching value is found, then returns a concatenated result from the table entry.
- `traverse_linked_list_2`: Traverses a linked list of pointers until a null pointer is encountered.
- `empty_routine_2`: The routine is empty and performs no operations.
- `validate_and_set_pointer`: Checks if a pointer or address range is valid and, if so, updates a memory location with a relative offset and a pointer.
- `infinite_loop_with_color_set`: The routine enters an infinite loop after setting a hardware color register.
- `insert_into_sorted_list`: Traverses a linked list to find an insertion point based on a value, then updates pointers to insert a new node.
- `insert_and_update_list_size`: Traverses a linked list to insert a node and updates a size counter if certain pointer conditions or offsets match.
- `sum_two_linked_lists`: Traverses two separate linked lists, accumulating the second word of each node into two separate counters, and returns the results.
- `sum_linked_list_values`: Traverses a linked list and accumulates the values found in the second word of each node into a running total.
- `empty_routine_3`: the routine contains no executable instructions and performs no operations.
- `shift_bits_recursively_2`: Iteratively shifts bits from a register and recursively calls another routine if a bit is cleared.
- `shift_bits_recursively`: Iteratively shifts bits from a register and recursively calls another routine if a bit is cleared.
- `rotate_right_extend_long`: Returns a 32-bit word read from memory, shifted right one bit, with the extend-flag bit placed at the top and the top bit forced set.
- `shift_bits_into_register`: Iteratively shifts bits from a register into a new value, prepending a bit from a previous register state.
- `decrement_global_counter`: Sets a global variable to a fixed value and then executes a loop that decrements that variable until it reaches a certain value.
- `copy_memory_block_2`: The routine copies a sequence of words from a source pointer to two different destination pointers.
- `set_blt_registers_and_wait`: The routine writes several values to certain memory-mapped registers and waits for a bit in a global variable to clear.
- `update_bcd_value`: The routine performs a seriesed series of additions and BCD adjustments on a global variable and potentially resets it to a fixed value.
- `calculate_blt_parameters`: Calculates and stores several hardware-related or memory-mapped values into a series of global memory locations and waits for a bit in a global variable to clear.
- `update_blt_parameters`: Calculates a new address and certain bitmask or attribute values, then stores them into a series of global memory locations and certain hardware registers.
- `modify_global_value`: Multiplies a global variable by two and conditionally applies a bitwise XOR if a certain flag is not set.
- `copy_relocation_table`: Copies a sequence of words from a source table to a destination address and then populates a secondary table with incrementing pointer values.
- `setup_blt_parameters`: Sets several global memory locations or hardware registers to specific constants and calculates new pointer or attribute values based on input registers.
- `execute_sequence_and_loop`: Executes a sequence of other routines and writes a fixed value to a memory location before entering an infinite loop.
- `execute_recursive_calls`: calls another routine twice and returns the concatenated result of the registers used in the calls.
- `recurse_without_side_effects`: This routine performs an infinite or unbounded recursive call to itself without modifying any registers or memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_buffer_with_offset`: Checks several bitflags in global memory, then writes a modified version of a value and an offset-based value into a target buffer.
- `split_and_store_integer`: Reads a 32-bit integer from a pointer and splits it into two 16-bit halves, storing them into a target buffer along with a modified version of the original value.
- `extract_high_word`: Extracts the upper 16 bits of a 32-bit value located at an offset from a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `infinite_loop_empty`: The routine is empty, consisting only of an infinite loop.
- `update_global_with_table_lookup`: Increments a global variable, resets it if it reaches a certain value, and then replaces it with a value loaded from a table using a calculated index.
- `copy_memory_block_4`: Copies a block of words from a source pointer to a destination pointer using a nested loop structure.
- `initialize_relocation_data`: Performs a sequence of routine calls and copies a block of words from a source table to a destination address before populating a secondary table with incrementing pointer values.
- `execute_infinite_loop`: This routine is empty and performs no actions other than an infinite loop.
- `apply_bitmask_to_table`: Iterates through a list of pointers, applying bitwise AND and OR operations to specific offsets within a table.
- `return_empty_sequence`: The routine is empty and performs no operations before returning.
- `update_address_table`: Updates a series of memory offsets with new address values while waiting for a global flag to clear.
- `set_address_offsets`: Writes a fixed value to a memory offset and then updates a pointer-based table with incremental address values.
- `set_address_offsets_alt`: Writes a fixed value to a memory offset and then updates a pointer-based table with incremental address values using a different source register.
- `mask_and_shift_value`: Applies a bitmask to an input value and shifts the result right by three bits.
