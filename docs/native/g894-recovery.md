# Game 894 (micro machines (1992)(codemasters)[cr csl]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00894). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 33 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 16 of 16 units identical (bytes and relocations), 170 raw-word lines |
| Functions named and described | pass, 188 of 188 named (0 author symbols), 188 of 188 described (12 by rule, 140 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 11644 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3345 of 3347 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 4426 |
| code | 9822 |
| data | 825288 |
| padding | 4816 |
| unused | 56832 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Micro Machines (1992)(Codemasters)[cr CSL].adf:GAM | 1 | 1939 | 98 |
| Micro Machines (1992)(Codemasters)[cr CSL].adf:LV1#unpacked | 1 | 18 | 1 |
| Micro Machines (1992)(Codemasters)[cr CSL].adf:LV2#unpacked | 1 | 18 | 1 |
| Micro Machines (1992)(Codemasters)[cr CSL].adf:LV3#unpacked | 1 | 21 | 1 |
| Micro Machines (1992)(Codemasters)[cr CSL].adf:LV4#unpacked | 1 | 22 | 1 |
| Micro Machines (1992)(Codemasters)[cr CSL].adf:LV5#unpacked | 1 | 21 | 1 |
| Micro Machines (1992)(Codemasters)[cr CSL].adf:LV6#unpacked | 1 | 19 | 1 |
| Micro Machines (1992)(Codemasters)[cr CSL].adf:LV7#unpacked | 1 | 20 | 1 |
| Micro Machines (1992)(Codemasters)[cr CSL].adf:LV8#unpacked | 1 | 22 | 1 |
| Micro Machines (1992)(Codemasters)[cr CSL].adf:LV9#unpacked | 1 | 19 | 1 |
| Micro Machines (1992)(Codemasters)[cr CSL].adf:FRT | 1 | 679 | 45 |
| Micro Machines (1992)(Codemasters)[cr CSL].adf:MID | 1 | 28 | 3 |
| Micro Machines (1992)(Codemasters)[cr CSL].adf:bootblock | 1 | 294 | 21 |
| logged disk load from Micro Machines (1992)(Codemasters)[cr CSL].adf boot-block load | 1 | 227 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 0 | 0 |
| Micro Machines (1992)(Codemasters)[cr CSL].adf:MD2#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- micro machines (1992)(codemasters)[cr csl]/Micro Machines (1992)(Codemasters)[cr CSL].adf:ICN: Imploder Data (xfd)-packed, unpacked with ancient (36620 -> 61440 bytes)
- micro machines (1992)(codemasters)[cr csl]/Micro Machines (1992)(Codemasters)[cr CSL].adf:LV1: Imploder Data (xfd)-packed, unpacked with ancient (86532 -> 103120 bytes)
- micro machines (1992)(codemasters)[cr csl]/Micro Machines (1992)(Codemasters)[cr CSL].adf:LV2: Imploder Data (xfd)-packed, unpacked with ancient (60366 -> 77124 bytes)
- micro machines (1992)(codemasters)[cr csl]/Micro Machines (1992)(Codemasters)[cr CSL].adf:LV3: Imploder Data (xfd)-packed, unpacked with ancient (53222 -> 65448 bytes)
- micro machines (1992)(codemasters)[cr csl]/Micro Machines (1992)(Codemasters)[cr CSL].adf:LV4: Imploder Data (xfd)-packed, unpacked with ancient (76090 -> 88296 bytes)
- micro machines (1992)(codemasters)[cr csl]/Micro Machines (1992)(Codemasters)[cr CSL].adf:LV5: Imploder Data (xfd)-packed, unpacked with ancient (39336 -> 50772 bytes)
- micro machines (1992)(codemasters)[cr csl]/Micro Machines (1992)(Codemasters)[cr CSL].adf:LV6: Imploder Data (xfd)-packed, unpacked with ancient (65154 -> 75932 bytes)
- micro machines (1992)(codemasters)[cr csl]/Micro Machines (1992)(Codemasters)[cr CSL].adf:LV7: Imploder Data (xfd)-packed, unpacked with ancient (43110 -> 54964 bytes)
- micro machines (1992)(codemasters)[cr csl]/Micro Machines (1992)(Codemasters)[cr CSL].adf:MD0: Imploder Data (xfd)-packed, unpacked with ancient (15388 -> 20596 bytes)
- micro machines (1992)(codemasters)[cr csl]/Micro Machines (1992)(Codemasters)[cr CSL].adf:LV8: Imploder Data (xfd)-packed, unpacked with ancient (49834 -> 62796 bytes)
- micro machines (1992)(codemasters)[cr csl]/Micro Machines (1992)(Codemasters)[cr CSL].adf:MD1: Imploder Data (xfd)-packed, unpacked with ancient (18190 -> 22606 bytes)
- micro machines (1992)(codemasters)[cr csl]/Micro Machines (1992)(Codemasters)[cr CSL].adf:LV9: Imploder Data (xfd)-packed, unpacked with ancient (67428 -> 80716 bytes)
- micro machines (1992)(codemasters)[cr csl]/Micro Machines (1992)(Codemasters)[cr CSL].adf:MD2: Imploder Data (xfd)-packed, unpacked with ancient (8160 -> 11644 bytes)
- micro machines (1992)(codemasters)[cr csl]/Micro Machines (1992)(Codemasters)[cr CSL].adf:MD3: Imploder Data (xfd)-packed, unpacked with ancient (15856 -> 19400 bytes)
- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 444E) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- micro machines (1992)(codemasters)[cr csl]/Micro Machines (1992)(Codemasters)[cr CSL].adf: the boot block reads 5632 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- micro machines (1992)(codemasters)[cr csl]/Micro Machines (1992)(Codemasters)[cr CSL].adf:MD2#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- micro machines (1992)(codemasters)[cr csl]/Micro Machines (1992)(Codemasters)[cr CSL].adf: 1 logged loads cover 5120 bytes of the image
- 1 never-loaded region(s), 11644 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `undecoded_ciab_port_routine` (bootblock): The decompiler timed out, so only the instruction count, the single call, and the CIA-B port B register access are known. Kind: disk hardware/drive identification check. Pass path: the branch not taken immediately following the call (the 'else' or error path is typically the failure/sabotage path).

## Functions


### Micro Machines (1992)(Codemasters)[cr CSL].adf:GAM

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `perform_bit_tests`: Performs several bit tests on d0 and d1, performs logical OR operations on d0, and returns if a word at offset 3a40 is zero.
- `calculate_clamped_values`: Performs multiple conditional checks on a global variable and a structure in a0, then calculates two values using arithmetic and bit shifts and stores them at offsets 120 and 122 of the structure in a0.
- `branch_on_word_value`: Moves a word from offset 76 of a structure into d0 and branches to another routine.
- `copy_structure_members`: Depending on various byte flags within a structure, it copies values from offsets 32 and 36 to offsets 0 and 4, and finally copies a byte from offset 229 to offset 228.
- `update_status_byte`: Sets a byte at offset 211 of a0 based on the sign of the difference between two calculated values derived from global variables and offsets 52 and 54 of a0.
- `compare_and_return`: Compares d2 to 240 and returns if d2 is greater than or equal to that value; otherwise it returns.
- `clamp_word_value_2`: Checks a word at offset 148 of a0; if non-zero, it loads a word from offset 74 of a0, subtracts it from d0, and if the result is non-negative, stores the result into offset 74 of a0.
- `clamp_word_value`: Loads a word from offset 74 of a0 into d0, and if bit 4 of a byte at offset 230 of a0 is clear, adds a word from offset 114 of a0 to d0 and clamps the result to a minimum value at offset 120 of a0.
- `clamp_calculated_value`: Checks bit 6 of a byte at offset 230 of a structure; if set, it subtracts a word at offset 116 from d0 and compares it against a negated, arithmetic-shifted version of a word at offset 118, updating d0 accordingly.
- `clamp_value_to_minimum`: Subtracts a 16-bit value at offset 130 from a register and, if the result is less than a provided value, stores that value into offset 74 of the structure in a0.
- `calculate_scaled_values_2`: Performs a signed multiplication of a word from offset 126 and a value derived from subtracting offset 74 from offset 118 and shifting right by 6, then conditionally stores results in offsets 232, 46, or 10 and 14 of a structure based on various bit tests and comparisons.
- `calculate_scaled_values`: Performs arithmetic on a value in d0 and d1, potentially iterating through a list of words from a global variable, and stores two 32-bit results at offsets 16 and 20 of the structure in a0 using values from a table.
- `update_entity_state_and_counters`: Checks several global flags and structure members to determine if an entity's state should be updated, potentially calling another routine and updating a global variable to a specific constant.
- `update_structure_values_2`: Decrements a 16-bit value at offset 150 of a structure in a0, calls another routine, and if a global variable is zero and specific byte conditions at offsets 247 and 211 are met, it performs a calculation involving offset 130 and offset 74 of the structure.
- `clear_on_comparison`: Compares a longword at offset 40 of a structure with d1 and, if d1 is greater than or equal, clears a word at offset 102 of the structure before returning.
- `update_accumulated_values`: If a 16-bit value at offset 140 of the structure in a0 is non-zero, it calculates two values using a byte from offset 227 and a table at offset 7cfde, then adds these scaled results to the 32-bit integers at offsets 16 and 20 of the structure.
- `check_and_double_value`: Loads a word from offset 128 of a structure in a0 into d2, shifts it right by 4, and if bytes at offsets 128 and 129 are non-zero and the word at offset 66 is non-negative, it doubles the value in d2 and branches to the end.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `scale_and_clamp_values`: Checks words at offsets 224 and 104 of a0, then performs arithmetic and bit shifts on d1 and d2, calls another routine, and conditionally clamps and stores values into offsets 8 and 12 of the structure in a0.
- `decrement_and_scale_value`: Decrements a word at offset 160 of a0 if it is non-zero, performs bit shifts on d2 based on a global variable, and conditionally moves d1 to d0 if a calculated difference is within a specific range.
- `compare_and_call_negated_add`: Compares d2 to d3 and, if d2 is less than d3, moves d1 to d0 before returning; otherwise it returns.
- `add_negated_value`: If a 16-bit value in d4 is zero, it negates d2 and adds the result to d0, then returns.
- `negate_and_halve_values`: Checks bytes at offsets 214 and 212 of the structure in a0; if they are not zero, it divides the 32-bit values at offsets 8 and 12 by two, negates them, and stores them back, while also updating a status byte based on the sign flag.
- `reduce_and_clamp_values`: Divides two 32-bit values at offsets 8 and 12 by two and negates them, copies a byte from offset 229 to offset 228, and if a 16-bit value at offset 74 is greater than 250, it reduces that value by one quarter and stores it back.
- `copy_and_clamp_longs`: Copies two longwords from offsets 4 and 8 of a0 to offsets 32 and 36 of a0, then adds a longword from offset 8 of a0 to d0 and clears offset 8 of a0 if the result is negative.
- `clamp_long_values`: Compares d0 to a constant, and if it is greater, clears a 32-bit value at offset 8 of the structure in a0 and sets d0 to that constant; it also performs a conditional clear of a 32-bit value at offset 12 of the structure in a0 based on d1.
- `clamp_and_store_longs`: If a 32-bit value in d1 is greater than a constant, it clears offset 12 of the structure in a0 and sets d1 to that constant, then stores d0 and d1 into offsets 0 and 4 of the structure.
- `process_summed_values`: Sets a byte at offset 217 of a structure in a0, compares a word at offset 98 to 16, and if it matches, performs arithmetic on values at offsets 66 and 68 before potentially calling another routine and clearing offset 96.
- `update_global_and_call`: Checks a word at offset 98 of a0 and a word at offset 14880, then pushes d0 to the stack, calls another routine, restores d0, and writes the value 10 to offset 14896.
- `update_offset_after_call`: Pushes d0 to the stack, calls another routine, restores d0, stores a byte at offset 217 of a0, negates d1, and performs several conditional checks on global variables and a byte at offset 208 of a0 before shifting d1 right.
- `compare_halved_value`: Shifts d1 right by two bits and compares it to d6; if d1 is less than d6, it clears d1 and d0, otherwise it continues.
- `store_summed_values`: Stores a 16-bit value from d0 into offset 66 of the structure in a0 and stores the sum of d1 and d7 into offset 68.
- `initialize_memory_and_branch`: Sets bit 7 of d7, clears d0, writes a word of all ones to a memory location pointed to by a0, calculates a value based on a word from memory and d0, and branches to another routine if the result is not zero.
- `fetch_table_entry`: Loads a 16-bit value from a memory location, uses it to calculate an offset into a table pointed to by d0, and loads a longword from that table into d0 if the value is not zero.
- `process_list_of_structures`: Iterates up to six times, copying data from a source list into a table at offset 24888, and calls a function pointer located at offset 6 of each source element.
- `decompose_and_lookup_values`: Extracts high words from two 32-bit values at offsets 0 and 4, performs division and modulo operations to derive new values, and uses the results to perform a table lookup that updates bytes at offsets 222 and 223 of the structure.
- `process_conditional_update`: Sets a byte at offset 384 of a0 using the sign flag, then checks words at offsets 154 and 239 of a0; if both are zero, it calls another routine, potentially calls a second routine with d0 set to 16, and clears offset 152 and sets offset 154 of a0.
- `update_structure_byte`: Checks a global word; if non-zero, it compares a byte from a table indexed by a masked byte at offset 222 of the structure in a0 against a word at offset 138 of the structure in a0.
- `compare_table_values`: Loads a byte from a table at an offset in d0 using a pointer in a1, and if that byte is not zero, compares it to the value 8; if the value is less than 8, it calculates a new offset using d1 and a bit shift to fetch a second byte from the table, then sets the sign flag based on that byte before branching.
- `compare_table_byte`: Loads a byte from a table at an offset determined by a0 and d0, and if it is not negative, compares it to a byte at offset 76 of the structure in a0; if they are not equal, it loads another byte from the table into d2.
- `check_byte_and_return_2`: Tests a byte at offset 211 of a0 and returns if the byte is zero; otherwise it returns.
- `update_structure_from_table_2`: Checks if two specific fields in a structure are zero, and if so, sets a byte at offset 235 to 4, calculates an offset into a table using a value from offset 70 of the structure, and then performs several additions and calls another routine to update values in a table using data from the structure and the table.
- `loop_bit_test`: Tests a bit in a word from a global variable using d4 as a counter, and loops until d4 reaches zero.
- `set_bit_and_table_values`: Sets a bit in d4 using d3, updates a global word with the result, calculates an offset by shifting d4 left by 3, and writes d0, d1, and d2 to the calculated address in a1, finally writing 7 to the start of that address.
- `decrement_loop_value`: Loads a word from a fixed address into d6, checks a bit in d6, and if set, enters a loop that decrements a 16-bit value at a fixed address and clears a bit in d6 upon completion.
- `loop_and_update_global`: Calculates a value in d2 using offsets 6 and 2 of a0, calls another routine within a loop controlled by d7, and finally stores d6 into a global variable.
- `clear_structure_fields`: Clears a word at offset 140 of a structure, and if a byte at offset 228 is not negative, clears a word at offset 136 of the structure.
- `fetch_and_store_table_word`: Calculates an index from d4, fetches a word from a table, and if d4 is not 135 or 136, calls another routine and stores the table word at offset 136 of the structure in a0 if a specific byte in d2 is zero.
- `clear_word_at_offset`: Clears the 16-bit word at offset 136 of the structure in a0 and returns.
- `update_status_bits`: Extracts bits 3 and 4 from a byte at offset 226 of the structure in a0, shifts the result right by 3, stores it as a 16-bit word at offset 140 of the structure in a0, and clears the 16-bit word at offset 136 of the structure in a0.
- `update_offset_conditionally`: Clears a 16-bit word at offset 140 of the structure in a0 and, if several conditions regarding offsets 136, 228, and 66 are met, calls another routine and stores the value 4 at offset 136 of the structure in a0.
- `set_offset`: Extracts bit 4 from a byte at offset 226 of the structure in a0, shifts it right by 2, and stores the result as a 16-bit word at offset 136 of the structure in a0.
- `clear_offsets_conditionally`: Clears a 16-bit word at offset 136 of the structure in a0, and if a byte at offset 228 is not negative, clears a 16-bit word at offset 140 of the structure in a0.
- `mask_and_store_word`: Masks d0 with 127 and stores the result as a 16-bit word at offset 140 of the structure in a0.
- `fetch_table_value`: If d2 is non-zero, it uses a global word and a value from d2 to calculate an index and an address to fetch a 16-bit word from a table and return it in d0; otherwise it returns 0.
- `test_and_set_status_bytes`: Performs three separate checks by calling another routine with different address offsets, and if the returned status in d2 is non-zero, it sets specific bytes at offsets 212, 213, or 214 of the structure.
- `conditional_call_with_stack`: Tests a byte at offset 17782 and a word at offset 14736, then if a byte at offset 228 of a0 is 11, it pushes d0 to the stack, calls another routine, restores d0, and branches to another routine.
- `call_subroutine_with_stack_data`: Pushes the word in d0 onto the stack, sets d0 to 15, calls another routine, restores d0 from the stack, and branches to another routine.
- `decrement_global_byte`: Pushes d0 to the stack, calls another routine, restores d0, writes the byte 9 to a global variable, and then decrements that global variable if it is not zero.
- `check_byte_and_return`: Tests a byte at offset 15 of the structure in a0; if the byte is zero, it returns, otherwise it clears d2 and returns.
- `execute_jump_table_entry`: Uses a global word to calculate an offset into a jump table and performs an indirect jump to the resulting address.
- `calculate_bit_status`: Performs arithmetic on d0 and d1 involving division, swapping, and multiplication to derive an index, then checks a specific bit in a table at an offset derived from a global pointer and returns the result of the bit test.
- `calculate_and_test_index`: Performs arithmetic on d0 and d1 to fetch values from tables using a pointer from a global variable, calls another routine, and returns zero if a bit is set or if a byte from a calculated address is within specific ranges.
- `calculate_scaled_hypotenuse`: Reads two signed 32-bit values from a structure, scales them down by dividing by 4096, calculates the sum of their squares, calls another routine, and stores the result shifted left by 3 into offset 112 of the structure.
- `iterate_and_copy_data`: Initializes a loop using a value from a fixed address, calls three different routines, copies words from offsets 76 and 78 to offsets 92 and 94, and copies a byte from offset 222 to 224 within the loop.
- `initialize_entity_structure`: Updates a structure in a0 by copying values from tables based on an index in d0, calculates and stores 32-bit coordinate values at offsets 0, 4, 32, and 36, clears numerous fields within the structure, and branches to another routine.
- `update_structure_from_table`: Copies five 16-bit words from a table into offsets 108, 110, 114, 116, and 130 of the structure in a0 using d0 as an index, then branches to another routine.
- `update_offset_and_pointer`: Calculates a value by adding a 32-bit word from a table to itself shifted right by 2, stores this in offset 40 of the structure in a0, and stores the original 32-bit word from the table into offset 44 of the structure in a0.
- `apply_conditional_physics_decay`: Calculates a multiplier by adding a global value to a 16-bit value at offset 124 of the structure in a0, then uses this multiplier to decay several 16-bit values at various offsets within the structure via fixed-point multiplication and addition, provided certain global conditions and a flag at offset 210 are met.
- `apply_physics_decay_2`: Performs fixed-point multiplication and bit-shifting to decay several 16-bit values at offsets 106 and 118 of a structure in a0, then subtracts 20 from the value at offset 118 if a global variable is non-zero.
- `apply_physics_decay`: Performs multiple iterations of fixed-point multiplication and bit-shifting to decay several 16-bit values at offsets 106, 114, 116, and 128 of a structure in a0.
- `update_physics_parameters`: Performs multiple iterations of a fixed-point multiplication and bit-shift calculation on a 16-bit value at offset 128 of a structure in a0, using values from d0, d2, and d3.
- `apply_physics_decay_3`: Checks a global word, and if non-zero, performs a series of multiplications and divisions to subtract a calculated value from offset 124 of a structure in a0, repeating this for two different structure base addresses.
- `fetch_scaled_word`: Saves registers to the stack, compares d0 to 16384, and if d0 is greater than or equal to that value, shifts it right by 7, masks bit 1, and fetches a word from a table at a fixed address.
- `scale_and_fetch_word`: Compares a 16-bit value in d0 to 4096; if it is less than 4096, it shifts d0 right by 5, clears bit 1, fetches a 16-bit word from a table pointed to by a1 using d0 as an index, shifts the result right by 1, and branches to another routine.
- `scale_and_fetch_table_value`: Compares a 16-bit value in d0 to 1024; if it is 1024 or greater, it shifts d0 right by 3 and clears bit 0.
- `fetch_table_value_2`: Compares a 16-bit value in d0 to 256, and if it is 256 or greater, shifts d0 right by 1 and clears bit 0; it then uses the resulting value as an index to fetch a 16-bit word from a table pointed to by a1 into d0, shifts d0 right by 3, and branches to another routine.
- `get_table_value`: Adds a 16-bit value in d0 to itself, uses the result as an index to fetch a 16-bit word from a table pointed to by a1, shifts the result right by 4 bits, and returns it in d0.
- `update_global_value`: Saves d4 and d5 to the stack, performs a loop that doubles a 32-bit value from a global variable and conditionally applies an XOR mask based on the carry flag, then either stores the result back to the global variable or calls another routine with the swapped upper 16 bits of the result.
- `multiply_high_bits`: Multiplies two 16-bit values passed in d0 and d5 and returns the upper 16 bits of the 32-bit product in d0.
- `copy_if_matches_header`: Checks if a 32-bit value at a0 matches a specific constant, then copies a block of data from a0 to a1 for a length determined by a value at offset 4 of the source structure, and finally calls another routine.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `update_entity_collision_and_position`: Calculates coordinate adjustments and spatial offsets based on entity dimensions and position, then updates a global variable and writes calculated values to a structure via a pointer in a5.
- `update_entity_spatial_data`: Calculates offset values based on a structure in a5 and a table in a3, then writes several calculated coordinates and bitwise-combined values into a structure in a6.
- `update_structure_fields`: Calculates several values based on registers and a structure in a5, writes them to a structure in a6, and conditionally copies data from two source pointers into a destination buffer pointed to by a4.
- `update_object_physics_and_position`: Calculates new position and velocity values for an object by performing arithmetic on input parameters and table data, then updates several fields in a structure pointed to by a register, including a 32-bit value, a 16-bit value, and two 16-bit values composed of combined bytes.
- `process_offset_data`: Performs arithmetic on d1, d3, and d6 to determine an offset, fetches a pointer from a table in a2, and updates various fields in a structure in a6 based on the fetched data.
- `copy_list_to_structure`: Iterates through a list of data pointed to by a structure at a fixed address, copying 14 elements of varying sizes into a structure in a6, and then updates the list pointer in the original structure.
- `swap_pointers`: Swaps two pointers stored at a fixed memory location and copies a 32-bit value from offset 4 of the old pointer to offset 80 of a structure in a6.
- `process_byte_from_pointer`: Saves registers a3 and a4 to the stack, loads a constant into a2, clears d1, fetches a byte from the address in a0 and increments a0, and if the byte is not zero, compares it to 93; if it is greater than 93, it clears bit 5 of the byte and calls another routine before branching to another routine.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `update_structure_elements`: Subtracts 33 from a 16-bit value in d1 and, if the result is not negative, copies a byte from a source table to a destination structure and clears bytes at offsets 40, 80, and 120 of that structure, repeating this process eight times in a loop; otherwise, it clears a byte at the destination structure and repeats the loop eight times.
- `update_pointer_registers`: Sets a4 to the value in a1, sets d6 to 7, updates the sign flag based on a byte at the address in a4, calculates new values for a4 and a3 using offsets of 0 and 50 respectively, and loops using a dbra instruction.
- `store_byte_via_table`: Uses a 16-bit value in d0 to calculate an index into a table at a fixed address, fetches a 16-bit word from that table, and stores the original byte from d0 into a destination at a fixed address using the fetched word as an offset.
- `update_audio_hardware_2`: Iterates through a loop four times, checking a byte from a memory location; if the byte minus one is not negative, it writes a longword to the AUD0LCH register, sets a bit in the DMACON register, and writes a value to a memory location, otherwise it writes a value to a memory location if a specific byte is non-zero.
- `update_audio_hardware`: Tests a byte from a memory location pointed to by a1 and branches if it is zero; otherwise, it writes a longword to a location pointed to by a3, writes a word to an offset of 4 from that same location, updates the sign flag based on a byte at an offset of minus 1 from a1, increments a1, modifies d4, increments a4, and loops using a dbra instruction until d7 reaches zero.
- `setup_audio_dma`: Reads a byte from a source, uses it to index into a table to fetch audio parameters, writes these parameters to audio hardware registers, and enables DMA via the DMACON register while looping four times.

### Micro Machines (1992)(Codemasters)[cr CSL].adf:LV1#unpacked

- `clear_toggle_bits_mask_longword`: Clears one bit in a byte addressed by an address register, toggles bits in two other bytes selected by data registers, and masks a longword at a computed offset.

### Micro Machines (1992)(Codemasters)[cr CSL].adf:LV2#unpacked

- `store_longword_set_bits`: Writes a 32-bit value into four consecutive bytes of a record addressed by an address register, then sets and toggles single bits in several byte locations.

### Micro Machines (1992)(Codemasters)[cr CSL].adf:LV3#unpacked

- `set_low_bits_xor_longword`: Sets the low twelve bits of a word through one address register, then exclusive-ORs a fixed bit pattern into a longword in a record addressed by another register.

### Micro Machines (1992)(Codemasters)[cr CSL].adf:LV4#unpacked

- `clear_toggle_bit_patch_longword`: Clears a bit in a byte selected by a data register, toggles another bit in a byte at a second address, stores a data-register value into a longword, then adds a constant to the longword at that second address.

### Micro Machines (1992)(Codemasters)[cr CSL].adf:LV5#unpacked

- `toggle_clear_bits_mask_word`: Toggles one bit and clears another in bytes selected by data registers, masks a word through an address register, and adds a constant to a byte before another address register.

### Micro Machines (1992)(Codemasters)[cr CSL].adf:LV6#unpacked

- `perform_self_assignment`: Copies a word from a memory location back to the same location.

### Micro Machines (1992)(Codemasters)[cr CSL].adf:LV7#unpacked

- `xor_word_toggle_clear_bit`: XORs a constant into a word addressed by one register, toggles a bit in a byte at an offset computed from a data register and another register, and clears a bit in a byte addressed by a third register.

### Micro Machines (1992)(Codemasters)[cr CSL].adf:LV8#unpacked

- `xor_longword_with_constant`: XORs a fixed bit pattern into a longword at an offset from an address register.

### Micro Machines (1992)(Codemasters)[cr CSL].adf:LV9#unpacked

- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Micro Machines (1992)(Codemasters)[cr CSL].adf:FRT

- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `decompress_bitstream_data_2`: Checks a magic constant in a global variable and, if matched, performs a bitstream decompression by reading bits from a source buffer and writing expanded values to a destination buffer.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `expand_bitstream_data`: Decrements a counter in d2 and copies bytes from a source pointer in a3 to a destination pointer in a4 until the counter reaches zero.
- `expand_bitstream_control_byte_2`: Reads a byte from a pointer in a3 and performs a series of conditional shifts and additions using the carry flag to update d3 and d4.
- `expand_bitstream_control_byte`: Reads a byte from a pointer in a3 and performs a series of conditional shifts and additions using the carry flag to update d3 and d4.
- `load_initial_constants`: Loads the values 4 and 3 into registers d4 and d0, then branches to another routine.
- `load_initial_constants_2`: Loads the values 3 and 2 into registers d4 and d0, then branches to another routine.
- `load_initial_constants_3`: Loads the values 2 and 1 into registers d4 and d0, then branches to another routine.
- `expand_compressed_bitstream_2`: Reads a control byte from a table to determine a bit count, then performs bitwise expansion of a bitstream using a carry-based shift loop to reconstruct data.
- `expand_compressed_bitstream`: Reads a control byte from a table to determine a bit count, then performs bitwise expansion of a bitstream using a carry-based shift loop to populate a destination buffer.
- `decompress_bitstream_data`: Reads a control byte from a table to determine a bit count, then performs bitwise expansion of a bitstream using a carry-based shift loop to copy data into a destination buffer.
- `clear_audio_volumes`: Clears a 16-bit value in a structure, sets a value at a fixed address, and clears the four audio volume registers and the DMA controller.
- `xor_word_from_table`: Loads a word from a table using a pointer in a6, performs an eor with 15 on it, and returns.
- `update_dma_and_table`: Writes a masked version of d0 to the DMACON register, then stores the eor of that value with 15 into a table offset 44 from a6.
- `load_word_from_table`: Loads a word from a table at offset 52 from a6 and returns.
- `copy_multiple_tables_to_pointer`: Copies four blocks of four longwords each from various tables into a destination pointed to by a0, and then copies six words from a fifth table into the same destination.
- `set_table_values`: Masks d0 with 63 and stores the result in two locations at offsets 34 and 36 from a6, sets a 16-bit value at offset 30 to 65535, and clears a word at offset 32 from a6.
- `process_audio_request_and_update_table`: Tests a 16-bit word at offset 30 of a structure in a6 and calls another routine if non-zero, then checks a bitmask at offset 44 to call another routine with various data pointers based on bits 0 through 3.
- `branch_on_structure_word`: Tests a 16-bit word at offset 28 of the structure in a6 and branches to another routine if the value is zero, otherwise it performs an unconditional branch to another routine.
- `initialize_and_update_data_structure`: Calls another routine four times with different addresses, then updates several fields in a structure pointed to by a6.
- `update_index_and_copy_data`: Increments a 16-bit counter at offset 36 from a6 and, if it reaches 64, updates a 16-bit value at offset 22 from a6 using a byte from a table, then copies several longwords from a table into four different locations and clears a word at offset 112 from a6.
- `parse_command_data_structure`: Initializes several fields in a structure pointed to by a0 to zero and sets a field at offset 66 to negative one.
- `branch_to_offset`: Performs a relative branch to a specific address.
- `branch_to_offset_2`: Performs a relative branch to a specific address.
- `update_structure_fields_2`: Stores a 16-bit value from d1 into offset 24 of the structure in a0, updates offset 10 with a masked byte from a5, and if the masked byte is not 63, calls another routine before performing a pointer addition and storing the result at the start of the structure in a0.
- `update_structure_values_3`: Masks a byte from a pointer in a5, stores it at offsets 4 and 5 of a structure in a0, and fetches a 16-bit value from a table using that byte as an index to store at offset 15.
- `split_and_store_word`: If a 16-bit value in d1 is not zero, it masks the lower 4 bits into offset 28 of a structure in a0, shifts the original value right by 4 and stores it at offset 2a of the structure, sets offset 26 to 65535, clears offset 2c, and branches to another routine; otherwise it branches to another routine.
- `negate_and_store_word`: Negates the 16-bit value in d1, stores it at offset 24 of a structure in a0, and branches to another routine.
- `store_word_to_offset`: Stores the 16-bit value from d1 into offset 24 of a structure in a0 and branches to another routine.
- `update_structure_values`: Masks a byte from a pointer in a5, stores the result at offsets 4 and 5 of a structure in a0, fetches a 16-bit value from a table using the masked value as an index to store at offset 15 of the structure, and if d1 is non-zero, updates offset 58 of the structure with either d1 or its negation based on a comparison between the table value and the value at offset 7 of the structure.
- `update_signed_offset`: Checks the 16-bit value at offset 3a of a structure in a0; if it is negative, it negates d1, stores it at offset 3a of the structure, sets offset 38 to 65535, and branches to another routine; otherwise it branches to another routine.
- `update_object_pointer`: Masks a byte from a global variable, stores it as a word at offset 20 of a structure in a0, and otherwise updates a pointer in a0 using a value from a table indexed by a word at offset 50 of a6.
- `clear_offset_fields`: Compares a word at offset 72 of a0 to 1; if it is 1, it clears the words at offsets 70 and 72 of a0.
- `decrement_word_at_offset`: Decrements a longword in d0 and stores the result as a word at offset 72 from a0.
- `update_pointer_and_table`: If d0 masked with 127 is not 1, it decrements d0, stores it at offset 70 from a0, and sets a word at offset 70 from a0 to 65535; it then increments a longword in a5 and stores it at the address in a0.
- `copy_record_fields_to_table`: Stores a word from the d0 register into a field of the object pointed to by the a0 register, then loads a word from a table indexed by d0 into another field.
- `copy_audio_channel_registers`: Saves a word field from a structure and clears it, stores it in a global variable, and loops calling another routine until a value returned in d1 is zero.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `check_and_clear_status`: Loads a long value from offset 8 of a structure in a0 into a4, tests a 16-bit value at offset 66 of the structure, and if that value is zero, branches to another routine; otherwise, it clears the 16-bit value at offset 66 and branches to another routine.
- `update_object_timers_and_state`: Updates several 16-bit values within a structure in a0, including incrementing or decrementing counters, applying bounds to values, and performing conditional updates to a 16-bit value at offset 28 based on values at offsets 36, 58, and 60.
- `update_object_state_and_timer_2`: Updates a 16-bit value at offset 28 of the structure in a0 if it is less than d0, clears offset 56, and modifies a 16-bit value at offset 30 by adding or subtracting a 16-bit value from offset 34, clamping the result between 0 and 64.
- `update_object_timer_and_state`: Subtracts a 16-bit value at offset 34 from a 16-bit value at offset 30 in the structure in a0, clamping the result to zero if negative.
- `update_object_state_and_timer`: Swaps the high and low words of a 32-bit value at offset 40 of the structure in a0, fetches a 16-bit value from a table using an index derived from offset 16 and offset 28, and performs a division using a byte from a table and a 16-bit value at offset 52 to update offset 54.
- `advance_counter_and_reset`: Decrements a word counter at offset 36 of a structure; if it is zero, it reloads it from offset 34, increments a second counter at offset 32, and if that counter reaches 65, it clears four audio volume registers, sets the DMA controller, and resets two words in the structure.

### Micro Machines (1992)(Codemasters)[cr CSL].adf:MID

- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `extract_structure_fields`: Loads a pointer from a global variable into a6, then checks a byte at offset 530 of that pointer and a longword at offset 62.
- `set_conditional_registers`: Checks if a0 is non-zero, then checks d4, and finally compares d6 to an address; if d6 is not equal to that value, it sets d4 to an address and moves d6 to d5.

### Micro Machines (1992)(Codemasters)[cr CSL].adf:bootblock

- `boot_block_read_reset_chips`: Fills an I/O request structure with a fixed length and offset, runs the library call that performs the I/O, then writes the custom chip registers to disable all DMA and interrupts and to set the disk-control bits.
- `undecoded_ciab_port_routine`: The decompiler timed out, so only the instruction count, the single call, and the CIA-B port B register access are known.
- `initialise_ciab_and_copy_block`: Sets two shared values and writes all bits to a CIA-B port register, then calls three unnamed routines.
- `set_control_port_bits`: Sets bits 4 to 6 of a byte at a fixed offset from an address register, then clears the top bit and bit 3, leaving the other bits unchanged.
- `set_two_control_port_bits`: Sets the top bit and bit 3 of a byte at a fixed offset from an address register and leaves the other bits unchanged.
- `find_named_entry_and_copy`: Hashes a zero-terminated name by multiplying a running value by thirteen and adding each character, then picks a bucket of entries by taking the hash modulo seventy-two.
- `copy_matched_entry_blocks`: Compares a stored name against the requested name case-insensitively and, on a full match, copies the entry's data in eight-byte blocks and single bytes until a count runs out.
- `push_return_address_and_call`: Writes a fixed code address into the slot four bytes below a stack pointer read from a structure, then makes one call.
- `divide_index_and_compare`: Divides a data-register value by eleven and compares the quotient's low word with a word stored in a structure.
- `step_stored_position_to_target`: Steps a stored position one unit at a time toward a target, signalling each step by toggling bits in a control byte and calling a routine for the next value.
- `step_until_position_matches`: Repeats a control-bit toggle and a routine call until the value returned matches an expected value, then stores the target into a structure.
- `store_and_flag_mismatch`: Stores a value into a structure, then, if it differs from an expected value, sets a control bit and writes two hardware registers.
- `start_transfer_and_wait_status`: Sets a control bit, writes three register values, then waits until a status bit is set before clearing the control bit.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `run_transfer_with_timing_values`: Sets a control bit, writes a count and a second value to two registers and waits for a status bit before clearing the control bit.
- `wait_while_flag_set`: Sets a control bit and, while a shared flag's fourth bit stays set, pulses another control bit and calls a routine.
- `call_then_clear_count`: Calls one unnamed routine and then clears a 16-bit count in a structure.
- `start_sync_word_dma_read`: Writes a fixed sync word and DMA control values to the custom chip registers, loads a buffer pointer from a structure and starts a transfer in two steps.
- `decode_mfm_sector_with_retry`: Searches a raw buffer for the sync word and matches the sector identity, then decodes the data by separating alternate bits and checks it with an XOR checksum.
- `decode_mfm_sector_payload`: Decodes sector data in place by separating alternate bits and checks the result with an XOR checksum, returning on a match.
- `decode_mfm_sector_search_loop`: Fetches the next word and searches the raw buffer for a sector that matches, then splits each long into alternate bits while copying data and computing an XOR checksum.

### logged disk load from Micro Machines (1992)(Codemasters)[cr CSL].adf boot-block load

- `relocate_block_backwards_and_copy`: Computes a destination from a stored offset, moves a block backwards in 32-byte groups, then copies fifty-seven longword pairs from a source to that destination with a fixed stride.
- `unpack_literal_and_repeat_runs`: Reads length counts from a stream and copies literal byte runs from one buffer to another, repeating earlier output bytes as back-reference runs until the output end is reached.
- `unpack_repeat_runs_and_launch`: Copies a literal byte and then a run of bytes from a stream, alternating with repeats of earlier output until the output limit is reached.
- `match_table_entry_extract_bits`: Scans a table of word pairs for the first entry whose masked value equals its stored companion, then reads a count and a shift from that entry.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 255 into d0, then loads 16 into d1.
- `mask_and_conditional_call`: ANDs two 16-bit values, then if one byte is less than another byte it calls an unnamed routine and returns that routine's result.
- `frag_add_lsr_swap_addq_move_rol_sub_mov`: Short code fragment that adds d1 to d7, then shifts d6 right by d7, then swaps the halves of d6, then adds 4 to a3, then copies the memory just below a3 (moving a3 down) to d6, then rotates d6 left by 8, then copies the memory just below a3 (moving a3 down) to d6, then swaps the halves of d6, then subtracts d7 from d1, then loads 16 into d7, then subtracts d1 from d7, then returns.
- `read_four_bytes_into_long`: Reads four consecutive bytes from a pointer and builds them into one 32-bit value with the first byte in the highest position.
- `build_canonical_code_table`: Reads a count and fills a stack buffer of code lengths, then assigns bit-reversed canonical codes for each length and stores them with their masks in a table.
- `launch_from_saved_words`: Copies ten words from a data area onto the stack in reverse order and then jumps to the address now on top of the stack.
- `restore_registers_from_stack`: Moves the value in d1 to the stack, restores d0 through d7 and a0 through a7 from the stack, and returns.
- `clear_bytes_dbra_restore_regs_return`: Clears a byte at offset 0 from a6 using a loop, stores a5 at offset 50 of the stack, restores all registers from the stack, and returns.
