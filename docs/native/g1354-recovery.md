# Game 1354 (xybots (1989)(domark)[cr qtx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01354). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (932624 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 71 raw-word lines |
| Functions named and described | pass, 105 of 105 named (0 author symbols), 105 of 105 described (5 by rule, 100 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 767 of 767 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3351 of 3351 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 11872 |
| code | 13370 |
| data | 869485 |
| padding | 9 |
| unused | 37888 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Xybots (1989)(Domark)[cr QTX].adf:bootblock | 1 | 8 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1352 | 35 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 104 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1790 | 62 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 88 | 4 |

## Cases handled

- xybots (1989)(domark)[cr qtx]/Xybots (1989)(Domark)[cr QTX].adf: custom-format (non-DOS) disk, 1684 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 332 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 12 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 371 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 47 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- xybots (1989)(domark)[cr qtx]/Xybots (1989)(Domark)[cr QTX].adf: 4 track-loaded blocks found in recovered runtime segments and classified through them
- xybots (1989)(domark)[cr qtx]/Xybots (1989)(Domark)[cr QTX].adf: blocks 2-26 (bytes 1024-13824) outside the filesystem hold data, not code (opcode density 0.006, entropy 5.80)
- xybots (1989)(domark)[cr qtx]/Xybots (1989)(Domark)[cr QTX].adf: blocks 28-133 (bytes 14336-68608) outside the filesystem hold data, not code (opcode density 0.032, entropy 6.62)
- xybots (1989)(domark)[cr qtx]/Xybots (1989)(Domark)[cr QTX].adf: blocks 141-141 (bytes 72192-72704) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.30)
- xybots (1989)(domark)[cr qtx]/Xybots (1989)(Domark)[cr QTX].adf: blocks 146-220 (bytes 74752-113152) outside the filesystem hold data, not code (opcode density 0.013, entropy 5.17)
- xybots (1989)(domark)[cr qtx]/Xybots (1989)(Domark)[cr QTX].adf: blocks 222-222 (bytes 113664-114176) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.85)
- xybots (1989)(domark)[cr qtx]/Xybots (1989)(Domark)[cr QTX].adf: blocks 224-278 (bytes 114688-142848) outside the filesystem hold data, not code (opcode density 0.044, entropy 5.22)
- xybots (1989)(domark)[cr qtx]/Xybots (1989)(Domark)[cr QTX].adf: blocks 282-287 (bytes 144384-147456) outside the filesystem hold data, not code (opcode density 0.043, entropy 5.79)
- xybots (1989)(domark)[cr qtx]/Xybots (1989)(Domark)[cr QTX].adf: blocks 289-304 (bytes 147968-156160) outside the filesystem hold data, not code (opcode density 0.042, entropy 5.14)
- xybots (1989)(domark)[cr qtx]/Xybots (1989)(Domark)[cr QTX].adf: blocks 306-343 (bytes 156672-176128) outside the filesystem hold data, not code (opcode density 0.015, entropy 3.27)
- xybots (1989)(domark)[cr qtx]/Xybots (1989)(Domark)[cr QTX].adf: blocks 348-348 (bytes 178176-178688) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- xybots (1989)(domark)[cr qtx]/Xybots (1989)(Domark)[cr QTX].adf: blocks 355-415 (bytes 181760-212992) outside the filesystem hold data, not code (opcode density 0.005, entropy 3.68)
- xybots (1989)(domark)[cr qtx]/Xybots (1989)(Domark)[cr QTX].adf: blocks 419-610 (bytes 214528-312832) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.01)
- xybots (1989)(domark)[cr qtx]/Xybots (1989)(Domark)[cr QTX].adf: blocks 615-676 (bytes 314880-346624) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.77)
- xybots (1989)(domark)[cr qtx]/Xybots (1989)(Domark)[cr QTX].adf: blocks 683-742 (bytes 349696-380416) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.00)
- xybots (1989)(domark)[cr qtx]/Xybots (1989)(Domark)[cr QTX].adf: blocks 747-781 (bytes 382464-400384) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.18)
- xybots (1989)(domark)[cr qtx]/Xybots (1989)(Domark)[cr QTX].adf: blocks 813-1757 (bytes 416256-900096) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.77)
- xybots (1989)(domark)[cr qtx]/Xybots (1989)(Domark)[cr QTX].adf: blocks 1759-1759 (bytes 900608-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.75)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Xybots (1989)(Domark)[cr QTX].adf:bootblock

- `branch_to_offset`: branches to an offset of 52.
- `copy_memory_block`: Copies 255 words from a source table to a destination address using a loop, then calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_table_bits`: Iterates through two tables, clearing a specific bit in the first table and calling other routines based on the state of bits in both tables.
- `set_bits_in_regions`: Returns early when a global variable is zero or greater than twelve, otherwise loops combining two counters into an address, sets bit seven of two byte regions, calls a routine, and repeats until the counters reach their limits.
- `tiny_move_andi`: Small routine of 2 instructions that copies (a4) to d5, then masks d5 with 15.
- `apply_bitmask_to_buffers`: Reads a byte from a source buffer and applies it via a bitwise OR to two destination regions, then advances the source pointer and repeats for the next byte until the loop ends.
- `clear_bits_in_loop`: Clears bit 2 of a byte at offset 31 from a pointer in a0, then increments the pointer by 32 and repeats this loop for a count derived from a global variable.
- `set_bits_in_loop`: Sets bit 2 of a byte at offset 31 from a pointer in a0, then increments the pointer by the value in d7 and repeats this loop for the number of iterations specified in d0.
- `merge_bits_into_table`: Computes a source address from a base and a low nibble shift, then for the first four entries clears the high bit of each destination byte before OR-ing in the corresponding source byte, and copies the remaining bytes verbatim into the scattered table locations.
- `apply_bitwise_or_to_table`: Computes a source address from a base plus a scaled index and ORs each of the twenty-four source bytes into the matching scattered entries of a destination table, leaving the existing bits set.
- `clear_table_entries`: Calculates a table address by adding a value from a register to a base address stored in a global variable, then writes zero into twenty-eight specific byte locations at fixed offsets within that table before returning.
- `apply_hunk_relocations`: Iterates through a series of hunk types, marking bytes in a relocation table by setting bits 6 and 7, then calls another routine to apply these relocations to a target memory structure using a series of byte-wise updates from a table.
- `apply_hunk_relocation_bytes`: Reads 24 bytes from a table indexed by a value in d0 and, for each byte, sets bit 7 of the corresponding byte in a second table pointed to by a1, then returns.
- `process_entity_event_loop`: Iterates through a table of data structures based on a global count.
- `execute_callback_list`: Checks a global variable and a table entry, then iterates through a list of pointers to execute several subroutines based on specific conditions.
- `apply_sequential_bit_masks`: Performs a series of bitwise OR operations on the byte in register d0 using a sequence of specific constant values and then returns.
- `clear_entity_attribute_tables`: Decrements two global counters and, if non-zero, iterates through two separate memory blocks to reset or mask specific word-aligned values.
- `update_hunk_type_and_patch_memory`: Compares a value from a structure against specific AmigaDOS hunk type constants to determine a status code, then calls a routine to perform system tasks.
- `apply_relocation_patch`: Iterates through a memory range to apply specific word-sized values to a series of fixed offsets relative to a base pointer in a0, then performs a second loop to zero out a different set of word-sized offsets at the same base pointer.
- `update_global_counter_modulo`: Increments a global variable modulo 16, then, when the value is zero and bit 1 is clear, branches into a loop that repeatedly reads a byte from a table, clamps it, subtracts it from the global variable, and stores zero back until the loop condition fails.
- `decrement_and_copy_data`: Decrements a global 16-bit word and returns if it reaches zero, otherwise it copies a series of 16-bit words from one memory location to another using two different stride patterns and calls another routine.
- `update_global_counters`: Sets a byte to 255, increments three global counters, decrements another, writes the constant 32 to a hardware register, sets another byte, and returns via a return from exception.
- `poll_until_flag_set`: Polls a routine that sets a global flag until that flag becomes true, then returns.
- `update_offset_table`: Iterates through a table of 64 values, adding 272 to each and wrapping the value to 1968 if it reaches 504.
- `populate_offset_tables`: Stores a 16-bit input value from a register into two global variables and writes that value plus the offsets 8000, 16000 and 24000, along with their high bytes, into several other global locations.
- `store_mouse_data`: Stores a byte from a register into a global variable.
- `calculate_mouse_delta`: Reads a 16-bit value from the mouse port, compares it to a global variable, and if they differ, calculates the difference and stores the new value in the global variable.
- `process_byte_difference`: Adds a byte from d0 to a 16-bit global variable, performs bitwise rotations on d0 and d1, and then compares the results to branch based on the sign of the difference.
- `increment_global_byte`: Adds a byte from a register to a global variable, masking the register value to a byte before the addition.
- `compare_byte_values`: Compares d0 against four consecutive bytes in a table and branches to another routine if any match is found.
- `combine_input_flags_2`: Calls two different routines and performs a bitwise OR on their returned byte values.
- `calculate_input_state`: Combines bits from a global variable with the high byte of a register value using AND, OR and XOR operations, then returns the resulting 32-bit value.
- `combine_input_flags`: Takes a value from another routine and performs bitwise OR operations on it based on specific bits set in a global variable.
- `process_joystick_bytes`: Reads two bytes from a specific location, compares them to zero, and calls another routine based on whether the bytes are negative or positive.
- `call_conditional_polls`: Invokes another routine four times in sequence, skipping the second call when a flag returned by the first call is clear, then returns a byte that is conditionally modified by a bitwise OR with 16.
- `scan_sprite_attribute_table`: Iterates through two separate tables of data structures, checking a 16-bit field at offset 18 of each entry; if the field is zero, it calls another routine.
- `clear_registers`: Loads a constant into a register using a PC-relative address and clears registers d0, d1, and d7.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `recursively_reset_global_flag`: Checks a global word and, when it is nonzero, resets it to zero and calls another routine, then repeats the check and reset with the value one before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_relocation_hunks`: Iterates through a table of hunk descriptors and, for each non-zero entry, calculates an offset by shifting the value left by five bits and uses it to write specific bytes from a source table into various offsets of a destination structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `toggle_byte_and_loop_calls`: Toggles a global byte between zero and one, then loops over a table in sixteen-byte steps, clearing bit 1 of a field at offset 31 and calling a routine selected by a shifted word at offset 16 when a field at offset 18 is zero.
- `do_nothing_return`: Performs a series of bitwise OR operations on registers d1 and d2 and then returns.
- `check_status_conditions`: Clears the low byte of register d0, tests a word at a global variable, and if zero, branches to another routine; otherwise, it clears a word at a global variable, compares a word at another global variable to 23, 25, and 27, and if any match, branches to another routine, otherwise it returns.
- `update_bonus_state`: Updates a pointer in a1 by adding 9992, sets a byte to 10 at offset 0 of a3, sets a byte to 100 at offset 29 of a2, and clears a global variable.
- `process_warp_bonus`: Checks a global variable and a structure in a2 to determine if it should increment a byte in a3, update a global variable, and modify a pointer in a1 and a structure in a2.
- `handle_bonus_logic`: Checks several conditions involving a global variable and a structure in a2, then updates a global variable, increments a pointer in a1, and modifies bytes in a2 and a3 before calling other routines.
- `clear_and_decrement_counters`: If a global variable exceeds 80, it clears three global variables, writes a value to a structure in a0, and decrements two other global variables before calling another routine.
- `prepare_data_and_branch`: Stores a long value into a global variable, stores a byte into a global variable, and then branches to another routine.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `change_byte_at_offset`: Changes a byte at offset 1406 of a structure addressed by a6 using the value in d2.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `zero_register`: Performs a bitwise OR operation on register d0 with zero.
- `clear_data_register_d0`: Performs a logical OR operation on data register d0 with zero.
- `tiny_ori_rts`: Small routine of 2 instructions that merges 0 into d0, then returns.
- `process_and_copy_data`: Calls a fixed routine, clears a global word, and then enters a complex loop that modifies a global word and a global byte based on register values, eventually copying data from a structure and calling two other routines.
- `store_bytes_with_offset`: Computes a target address from a base register plus an offset, then stores three consecutive bytes from a source buffer into a short array there, adding 756 to each value.
- `store_const_short`: Writes the constant short value 850 into a short field at an offset within a structure addressed by a base register, then returns.
- `store_const_short_other`: Writes the constant short value 847 into a short field at an offset within a structure addressed by a base register, then returns.
- `update_game_state_and_tables`: Checks status conditions and updates several global tables by calculating values based on a division by ten and writing them to a destination table.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `flip_and_increment_flags`: Flips a flag bit in a byte at offset 22, then increments a following byte and masks it by fifteen, setting a byte at offset 18 to all ones when the incremented value wraps to zero.
- `update_entity_state_and_dispatch`: Checks a global variable and a bit in a structure to set a flag, then decrements a counter at offset 4 and updates a circular index at offset 20.
- `update_structure_and_dispatch_entity`: Decrements a byte at offset 4 of the structure in a0 and, if it reaches zero, increments a 16-bit counter at offset 20 masked with 7, then calls another routine and performs a jump table dispatch based on a value retrieved from a global variable.
- `update_structure_from_buffer`: Subtracts a byte from a buffer, potentially calls another routine and clears a bit, then decrements a counter and copies several words and a byte from the buffer into a structure at an offset determined by a byte in the buffer.
- `increment_and_check_structure_counter`: Increments a byte at offset 23 of the structure in a0 and masks it with 3; if the result is 0, it calls another routine and returns, otherwise it continues to perform further conditional checks and updates to offsets 22, 23, 25, and 30 of the structure.
- `call_then_increment_flags`: Calls another routine, then flips a flag bit in a byte at offset 22 and increments a following byte, clearing that byte when it reaches the value three.
- `clear_global_word_array`: Sets two global byte variables at offsets f49a and f4ba, clears the bytes at offsets f497 and f4b7, zeroes the word at offset 204fa, then loops nine times to clear a run of consecutive four-byte words starting at the global variable and returns.
- `update_structure_and_call_functions_2`: Checks a global word, loops through a list of pointers to clear entries, increments a global counter, and then copies data from a structure into another structure before calling a routine from a jump table.
- `update_structure_and_call_functions`: Calculates a value from a pointer in a0 and stores it at offset 25 of a structure in a5, sets several other fields in the structure, and calls two different routines before returning a value in d0.
- `decrement_indexed_and_store`: Reads a word at offset 4 of the structure in a0 and clears the matching word in a global table, then decrements a byte at offset 25 indexed by a shifted value and decrements a global counter before storing 255 in a byte at offset 18.
- `scatter_table_words`: When a global variable is nonzero it copies a run of words from a source table into a destination table, scattering the values across fixed offsets and advancing the source by 80 while advancing the destination by 20 each iteration, then calls another routine.
- `clear_memory_block`: Writes zero to a series of global variables and RAM locations at fixed offsets in sequence, performing no other operation before returning.
- `decrement_global_twice`: Decrements a global variable by two if it is not zero, then decrements another global variable by two if it is not zero, before returning.
- `decrement_global_thrice`: Decrements a global variable by three if it is not zero, then decrements another global variable by three if it is not zero, before returning zero.
- `copy_table_words_2`: Copies a byte and a series of words from a source table offset by a shifted register value into fixed offsets of a destination structure, then returns.
- `copy_table_words`: Copies a byte and a series of words from a source table offset by a shifted register value into fixed offsets of the destination structure in a0, transferring the data without altering the source values before returning.
- `mask_object_flags`: Iterates through a list of objects to clear a fixed set of flag bytes in a per-object table, then updates those flags by performing bitwise AND operations between the object's data and several source tables.
- `clear_d0_register`: Clears the byte in register d0.
- `clear_d0_register_2`: Sets the byte in register d0 to zero.
- `clear_d0_register_3`: Performs a logical OR on register d0 with zero.
- `setup_table_reduction_calls`: Loads a value from a global variable into d0, sets up three pairs of pointers in a1 and a2, and calls another routine twice with different values in d0.
- `reduce_and_copy_table_2`: Adds a value from a global variable to another, subtracts an address if the result exceeds an address, clears a third global variable, and then calls another routine to reduce and copy values.
- `reduce_and_copy_table`: Reduces a large incoming value by repeatedly subtracting one million and then one hundred thousand, then copies a run of words from a source table into two destination tables at fixed offsets.
- `copy_table_and_return`: Copies a run of words from a source table into two destination tables at fixed offsets using an incoming word count to index that table, then returns two register values.
- `adjust_global_copy_table`: Subtracts 99 from a byte from a global variable if it exceeds 99, then calls another routine to copy table entries, and repeats this process for a second global variable.
- `adjust_byte_and_copy_2`: If a global variable is not equal to 48782, it checks if the byte at the address in a3 is greater than 99, subtracts 99 from it, and calls another routine.
- `adjust_byte_and_copy`: Reads a byte from a global variable through a register, subtracts 99 from it while the value exceeds 99, stores the result back, and then calls another routine to copy table entries.
- `repeated_conditional_update`: Performs a self-referencing comparison and assignment on a global variable, then conditionally subtracts 99 from it before invoking another routine, repeating this sequence twice.
- `copy_table_entries`: Copies a run of words from a source table into two destination tables at fixed offsets, using an incoming word count to index that table, then returns.
- `copy_hunk_table_entries`: Copies a run of words from a source table into two destination tables at a set of fixed offsets, using the incoming word count to index that table.
- `copy_word_blocks`: Copies a block of sixteen words from a source pointer into a destination table, adjusting the block size when the incoming value equals forty-two, then stepping through twelve such blocks.
- `copy_single_word_blocks`: Copies a single word from a source pointer into a destination table, then steps through twelve such blocks.
- `parse_and_store_values`: Sets the stack pointer based on a value in d0, then reads bytes from a4 to convert them into specific word values which are stored in a destination table pointed to by a1.
- `modify_register_and_branch`: Compares d0 to 57, and if d0 is greater than 57, adds 812 to d0 before branching to another location.
- `increment_and_store_value`: Adds a constant to a register, stores the result at the address in a6 and increments a6, then branches to another routine.
- `update_stack_pointer`: Copies the stack pointer to a1, adds 80 to a1, and branches to another routine.
- `set_global_and_return`: Writes a fixed value into a global variable twice and then returns to the caller.
- `copy_conditional_word_blocks`: Selects one of two source tables depending on whether a global variable is nonzero, then copies a block of sixteen words from that table into a destination table and steps through twelve such blocks.
- `test_bit_in_register`: Moves a word from an offset of a5 into d7 twice and tests bit 0 of d7 against d0.
- `convert_table_values_to_clock`: Loops seven times over a table of words, reducing each value by subtracting successive powers of ten and calling a helper routine to convert them into clock values, then advances to the next entry and repeats until the counter reaches its limit.
- `scale_register_value`: Copies d0 to d2, subtracts 32 from d2, shifts d2 left by 3, and branches to another routine.
- `apply_byte_masks`: Loops over eight bytes, inverting each and applying it to four destination locations through masked set/clear operations controlled by the bits of a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `evaluate_and_modify_state`: Enters a loop that calls various routines and checks a global variable against zero, conditionally setting the global variable to zero or incrementing it.
- `clears_data_register_zero`: Performs a logical OR operation on data register zero with zero.
- `evaluate_global_state`: Tests a global variable against zero and specific constants to set a global variable to either zero or a specific bit pattern, otherwise returning an incoming register value.
- `modify_global_bits`: Calls another routine twice and conditionally performs bitwise AND or OR operations on a global variable based on the result of each call.
